#[cfg(all(feature = "minwindef", feature = "winnls"))]
#[inline]
pub unsafe fn CompareStringEx<P0>(lplocalename: P0, dwcmpflags: u32, lpstring1: *const u16, cchcount1: i32, lpstring2: *const u16, cchcount2: i32, lpversioninformation: Option<*const super::winnls::NLSVERSIONINFO>, lpreserved: Option<*const core::ffi::c_void>, lparam: Option<super::minwindef::LPARAM>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CompareStringEx(lplocalename : windows_core::PCWSTR, dwcmpflags : u32, lpstring1 : *const u16, cchcount1 : i32, lpstring2 : *const u16, cchcount2 : i32, lpversioninformation : *const super::winnls::NLSVERSIONINFO, lpreserved : *const core::ffi::c_void, lparam : super::minwindef::LPARAM) -> i32);
    unsafe { CompareStringEx(lplocalename.param().abi(), dwcmpflags, lpstring1, cchcount1, lpstring2, cchcount2, lpversioninformation.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _, lparam.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CompareStringOrdinal(lpstring1: *const u16, cchcount1: i32, lpstring2: *const u16, cchcount2: i32, bignorecase: bool) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn CompareStringOrdinal(lpstring1 : *const u16, cchcount1 : i32, lpstring2 : *const u16, cchcount2 : i32, bignorecase : windows_core::BOOL) -> i32);
    unsafe { CompareStringOrdinal(lpstring1, cchcount1, lpstring2, cchcount2, bignorecase.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CompareStringW(locale: super::winnt::LCID, dwcmpflags: u32, lpstring1: *const u16, cchcount1: i32, lpstring2: *const u16, cchcount2: i32) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn CompareStringW(locale : super::winnt::LCID, dwcmpflags : u32, lpstring1 : *const u16, cchcount1 : i32, lpstring2 : *const u16, cchcount2 : i32) -> i32);
    unsafe { CompareStringW(locale, dwcmpflags, lpstring1, cchcount1, lpstring2, cchcount2) }
}
#[inline]
pub unsafe fn FoldStringW(dwmapflags: u32, lpsrcstr: *const u16, cchsrc: i32, lpdeststr: Option<&mut [u16]>) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn FoldStringW(dwmapflags : u32, lpsrcstr : *const u16, cchsrc : i32, lpdeststr : windows_core::PWSTR, cchdest : i32) -> i32);
    unsafe { FoldStringW(dwmapflags, lpsrcstr, cchsrc, core::mem::transmute(lpdeststr.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpdeststr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetStringTypeExW(locale: super::winnt::LCID, dwinfotype: u32, lpsrcstr: *const u16, lpchartype: &mut [u16]) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetStringTypeExW(locale : super::winnt::LCID, dwinfotype : u32, lpsrcstr : *const u16, cchsrc : i32, lpchartype : *mut u16) -> windows_core::BOOL);
    unsafe { GetStringTypeExW(locale, dwinfotype, lpsrcstr, lpchartype.len().try_into().unwrap(), lpchartype.as_mut_ptr()) }
}
#[inline]
pub unsafe fn GetStringTypeW(dwinfotype: u32, lpsrcstr: *const u16, cchsrc: i32, lpchartype: *mut u16) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetStringTypeW(dwinfotype : u32, lpsrcstr : *const u16, cchsrc : i32, lpchartype : *mut u16) -> windows_core::BOOL);
    unsafe { GetStringTypeW(dwinfotype, lpsrcstr, cchsrc, lpchartype as _) }
}
#[inline]
pub unsafe fn MultiByteToWideChar(codepage: u32, dwflags: u32, lpmultibytestr: *const i8, cbmultibyte: i32, lpwidecharstr: Option<&mut [u16]>) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn MultiByteToWideChar(codepage : u32, dwflags : u32, lpmultibytestr : *const i8, cbmultibyte : i32, lpwidecharstr : windows_core::PWSTR, cchwidechar : i32) -> i32);
    unsafe { MultiByteToWideChar(codepage, dwflags, lpmultibytestr, cbmultibyte, core::mem::transmute(lpwidecharstr.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpwidecharstr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn WideCharToMultiByte(codepage: u32, dwflags: u32, lpwidecharstr: *const u16, cchwidechar: i32, lpmultibytestr: Option<&mut [u8]>, lpdefaultchar: Option<*const i8>, lpuseddefaultchar: Option<*mut windows_core::BOOL>) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn WideCharToMultiByte(codepage : u32, dwflags : u32, lpwidecharstr : *const u16, cchwidechar : i32, lpmultibytestr : windows_core::PSTR, cbmultibyte : i32, lpdefaultchar : *const i8, lpuseddefaultchar : *mut windows_core::BOOL) -> i32);
    unsafe { WideCharToMultiByte(codepage, dwflags, lpwidecharstr, cchwidechar, core::mem::transmute(lpmultibytestr.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpmultibytestr.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpdefaultchar.unwrap_or(core::mem::zeroed()) as _, lpuseddefaultchar.unwrap_or(core::mem::zeroed()) as _) }
}
