#[inline]
pub unsafe fn GetFileVersionInfoA<P0>(lptstrfilename: P0, dwhandle: Option<u32>, dwlen: u32, lpdata: *mut core::ffi::c_void) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("version.dll" "system" fn GetFileVersionInfoA(lptstrfilename : windows_core::PCSTR, dwhandle : u32, dwlen : u32, lpdata : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { GetFileVersionInfoA(lptstrfilename.param().abi(), dwhandle.unwrap_or(core::mem::zeroed()) as _, dwlen, lpdata as _) }
}
#[inline]
pub unsafe fn GetFileVersionInfoExA<P1>(dwflags: u32, lpwstrfilename: P1, dwhandle: Option<u32>, dwlen: u32, lpdata: *mut core::ffi::c_void) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("version.dll" "system" fn GetFileVersionInfoExA(dwflags : u32, lpwstrfilename : windows_core::PCSTR, dwhandle : u32, dwlen : u32, lpdata : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { GetFileVersionInfoExA(dwflags, lpwstrfilename.param().abi(), dwhandle.unwrap_or(core::mem::zeroed()) as _, dwlen, lpdata as _) }
}
#[inline]
pub unsafe fn GetFileVersionInfoExW<P1>(dwflags: u32, lpwstrfilename: P1, dwhandle: Option<u32>, dwlen: u32, lpdata: *mut core::ffi::c_void) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("version.dll" "system" fn GetFileVersionInfoExW(dwflags : u32, lpwstrfilename : windows_core::PCWSTR, dwhandle : u32, dwlen : u32, lpdata : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { GetFileVersionInfoExW(dwflags, lpwstrfilename.param().abi(), dwhandle.unwrap_or(core::mem::zeroed()) as _, dwlen, lpdata as _) }
}
#[inline]
pub unsafe fn GetFileVersionInfoSizeA<P0>(lptstrfilename: P0, lpdwhandle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("version.dll" "system" fn GetFileVersionInfoSizeA(lptstrfilename : windows_core::PCSTR, lpdwhandle : *mut u32) -> u32);
    unsafe { GetFileVersionInfoSizeA(lptstrfilename.param().abi(), lpdwhandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetFileVersionInfoSizeExA<P1>(dwflags: u32, lpwstrfilename: P1, lpdwhandle: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("version.dll" "system" fn GetFileVersionInfoSizeExA(dwflags : u32, lpwstrfilename : windows_core::PCSTR, lpdwhandle : *mut u32) -> u32);
    unsafe { GetFileVersionInfoSizeExA(dwflags, lpwstrfilename.param().abi(), lpdwhandle as _) }
}
#[inline]
pub unsafe fn GetFileVersionInfoSizeExW<P1>(dwflags: u32, lpwstrfilename: P1, lpdwhandle: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("version.dll" "system" fn GetFileVersionInfoSizeExW(dwflags : u32, lpwstrfilename : windows_core::PCWSTR, lpdwhandle : *mut u32) -> u32);
    unsafe { GetFileVersionInfoSizeExW(dwflags, lpwstrfilename.param().abi(), lpdwhandle as _) }
}
#[inline]
pub unsafe fn GetFileVersionInfoSizeW<P0>(lptstrfilename: P0, lpdwhandle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("version.dll" "system" fn GetFileVersionInfoSizeW(lptstrfilename : windows_core::PCWSTR, lpdwhandle : *mut u32) -> u32);
    unsafe { GetFileVersionInfoSizeW(lptstrfilename.param().abi(), lpdwhandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetFileVersionInfoW<P0>(lptstrfilename: P0, dwhandle: Option<u32>, dwlen: u32, lpdata: *mut core::ffi::c_void) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("version.dll" "system" fn GetFileVersionInfoW(lptstrfilename : windows_core::PCWSTR, dwhandle : u32, dwlen : u32, lpdata : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { GetFileVersionInfoW(lptstrfilename.param().abi(), dwhandle.unwrap_or(core::mem::zeroed()) as _, dwlen, lpdata as _) }
}
#[inline]
pub unsafe fn VerFindFileA<P1, P2, P3>(uflags: u32, szfilename: P1, szwindir: P2, szappdir: P3, szcurdir: windows_core::PSTR, pucurdirlen: *mut u32, szdestdir: windows_core::PSTR, pudestdirlen: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("version.dll" "system" fn VerFindFileA(uflags : u32, szfilename : windows_core::PCSTR, szwindir : windows_core::PCSTR, szappdir : windows_core::PCSTR, szcurdir : windows_core::PSTR, pucurdirlen : *mut u32, szdestdir : windows_core::PSTR, pudestdirlen : *mut u32) -> u32);
    unsafe { VerFindFileA(uflags, szfilename.param().abi(), szwindir.param().abi(), szappdir.param().abi(), core::mem::transmute(szcurdir), pucurdirlen as _, core::mem::transmute(szdestdir), pudestdirlen as _) }
}
#[inline]
pub unsafe fn VerFindFileW<P1, P2, P3>(uflags: u32, szfilename: P1, szwindir: P2, szappdir: P3, szcurdir: windows_core::PWSTR, pucurdirlen: *mut u32, szdestdir: windows_core::PWSTR, pudestdirlen: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("version.dll" "system" fn VerFindFileW(uflags : u32, szfilename : windows_core::PCWSTR, szwindir : windows_core::PCWSTR, szappdir : windows_core::PCWSTR, szcurdir : windows_core::PWSTR, pucurdirlen : *mut u32, szdestdir : windows_core::PWSTR, pudestdirlen : *mut u32) -> u32);
    unsafe { VerFindFileW(uflags, szfilename.param().abi(), szwindir.param().abi(), szappdir.param().abi(), core::mem::transmute(szcurdir), pucurdirlen as _, core::mem::transmute(szdestdir), pudestdirlen as _) }
}
#[inline]
pub unsafe fn VerInstallFileA<P1, P2, P3, P4, P5>(uflags: u32, szsrcfilename: P1, szdestfilename: P2, szsrcdir: P3, szdestdir: P4, szcurdir: P5, sztmpfile: windows_core::PSTR, putmpfilelen: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("version.dll" "system" fn VerInstallFileA(uflags : u32, szsrcfilename : windows_core::PCSTR, szdestfilename : windows_core::PCSTR, szsrcdir : windows_core::PCSTR, szdestdir : windows_core::PCSTR, szcurdir : windows_core::PCSTR, sztmpfile : windows_core::PSTR, putmpfilelen : *mut u32) -> u32);
    unsafe { VerInstallFileA(uflags, szsrcfilename.param().abi(), szdestfilename.param().abi(), szsrcdir.param().abi(), szdestdir.param().abi(), szcurdir.param().abi(), core::mem::transmute(sztmpfile), putmpfilelen as _) }
}
#[inline]
pub unsafe fn VerInstallFileW<P1, P2, P3, P4, P5>(uflags: u32, szsrcfilename: P1, szdestfilename: P2, szsrcdir: P3, szdestdir: P4, szcurdir: P5, sztmpfile: windows_core::PWSTR, putmpfilelen: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("version.dll" "system" fn VerInstallFileW(uflags : u32, szsrcfilename : windows_core::PCWSTR, szdestfilename : windows_core::PCWSTR, szsrcdir : windows_core::PCWSTR, szdestdir : windows_core::PCWSTR, szcurdir : windows_core::PCWSTR, sztmpfile : windows_core::PWSTR, putmpfilelen : *mut u32) -> u32);
    unsafe { VerInstallFileW(uflags, szsrcfilename.param().abi(), szdestfilename.param().abi(), szsrcdir.param().abi(), szdestdir.param().abi(), szcurdir.param().abi(), core::mem::transmute(sztmpfile), putmpfilelen as _) }
}
#[inline]
pub unsafe fn VerLanguageNameA(wlang: u32, szlang: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn VerLanguageNameA(wlang : u32, szlang : windows_core::PSTR, cchlang : u32) -> u32);
    unsafe { VerLanguageNameA(wlang, core::mem::transmute(szlang.as_ptr()), szlang.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn VerLanguageNameW(wlang: u32, szlang: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn VerLanguageNameW(wlang : u32, szlang : windows_core::PWSTR, cchlang : u32) -> u32);
    unsafe { VerLanguageNameW(wlang, core::mem::transmute(szlang.as_ptr()), szlang.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn VerQueryValueA<P1>(pblock: *const core::ffi::c_void, lpsubblock: P1, lplpbuffer: *mut *mut core::ffi::c_void, pulen: *mut u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("version.dll" "system" fn VerQueryValueA(pblock : *const core::ffi::c_void, lpsubblock : windows_core::PCSTR, lplpbuffer : *mut *mut core::ffi::c_void, pulen : *mut u32) -> windows_core::BOOL);
    unsafe { VerQueryValueA(pblock, lpsubblock.param().abi(), lplpbuffer as _, pulen as _) }
}
#[inline]
pub unsafe fn VerQueryValueW<P1>(pblock: *const core::ffi::c_void, lpsubblock: P1, lplpbuffer: *mut *mut core::ffi::c_void, pulen: *mut u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("version.dll" "system" fn VerQueryValueW(pblock : *const core::ffi::c_void, lpsubblock : windows_core::PCWSTR, lplpbuffer : *mut *mut core::ffi::c_void, pulen : *mut u32) -> windows_core::BOOL);
    unsafe { VerQueryValueW(pblock, lpsubblock.param().abi(), lplpbuffer as _, pulen as _) }
}
