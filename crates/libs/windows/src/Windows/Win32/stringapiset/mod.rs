#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnls", feature = "winnt"))]
#[inline]
pub unsafe fn CompareStringEx<P0>(lplocalename: P0, dwcmpflags: super::DWORD, lpstring1: super::LPCWCH, cchcount1: i32, lpstring2: super::LPCWCH, cchcount2: i32, lpversioninformation: Option<super::LPNLSVERSIONINFO>, lpreserved: Option<super::LPVOID>, lparam: Option<super::LPARAM>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CompareStringEx(lplocalename : windows_core::PCWSTR, dwcmpflags : super::DWORD, lpstring1 : super::LPCWCH, cchcount1 : i32, lpstring2 : super::LPCWCH, cchcount2 : i32, lpversioninformation : super::LPNLSVERSIONINFO, lpreserved : super::LPVOID, lparam : super::LPARAM) -> i32);
    unsafe { CompareStringEx(lplocalename.param().abi(), dwcmpflags, lpstring1, cchcount1, lpstring2, cchcount2, lpversioninformation.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _, lparam.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CompareStringOrdinal(lpstring1: super::LPCWCH, cchcount1: i32, lpstring2: super::LPCWCH, cchcount2: i32, bignorecase: bool) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn CompareStringOrdinal(lpstring1 : super::LPCWCH, cchcount1 : i32, lpstring2 : super::LPCWCH, cchcount2 : i32, bignorecase : windows_core::BOOL) -> i32);
    unsafe { CompareStringOrdinal(lpstring1, cchcount1, lpstring2, cchcount2, bignorecase.into()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CompareStringW(locale: super::LCID, dwcmpflags: super::DWORD, lpstring1: super::PCNZWCH, cchcount1: i32, lpstring2: super::PCNZWCH, cchcount2: i32) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn CompareStringW(locale : super::LCID, dwcmpflags : super::DWORD, lpstring1 : super::PCNZWCH, cchcount1 : i32, lpstring2 : super::PCNZWCH, cchcount2 : i32) -> i32);
    unsafe { CompareStringW(locale, dwcmpflags, lpstring1, cchcount1, lpstring2, cchcount2) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FoldStringW(dwmapflags: super::DWORD, lpsrcstr: super::LPCWCH, cchsrc: i32, lpdeststr: Option<windows_core::PWSTR>, cchdest: i32) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn FoldStringW(dwmapflags : super::DWORD, lpsrcstr : super::LPCWCH, cchsrc : i32, lpdeststr : windows_core::PWSTR, cchdest : i32) -> i32);
    unsafe { FoldStringW(dwmapflags, lpsrcstr, cchsrc, lpdeststr.unwrap_or(core::mem::zeroed()) as _, cchdest) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetStringTypeExW(locale: super::LCID, dwinfotype: super::DWORD, lpsrcstr: super::LPCWCH, cchsrc: i32, lpchartype: super::LPWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetStringTypeExW(locale : super::LCID, dwinfotype : super::DWORD, lpsrcstr : super::LPCWCH, cchsrc : i32, lpchartype : super::LPWORD) -> windows_core::BOOL);
    unsafe { GetStringTypeExW(locale, dwinfotype, lpsrcstr, cchsrc, lpchartype as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetStringTypeW(dwinfotype: super::DWORD, lpsrcstr: super::LPCWCH, cchsrc: i32, lpchartype: super::LPWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetStringTypeW(dwinfotype : super::DWORD, lpsrcstr : super::LPCWCH, cchsrc : i32, lpchartype : super::LPWORD) -> windows_core::BOOL);
    unsafe { GetStringTypeW(dwinfotype, lpsrcstr, cchsrc, lpchartype as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MultiByteToWideChar(codepage: super::UINT, dwflags: super::DWORD, lpmultibytestr: super::LPCCH, cbmultibyte: i32, lpwidecharstr: Option<windows_core::PWSTR>, cchwidechar: i32) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn MultiByteToWideChar(codepage : super::UINT, dwflags : super::DWORD, lpmultibytestr : super::LPCCH, cbmultibyte : i32, lpwidecharstr : windows_core::PWSTR, cchwidechar : i32) -> i32);
    unsafe { MultiByteToWideChar(codepage, dwflags, lpmultibytestr, cbmultibyte, lpwidecharstr.unwrap_or(core::mem::zeroed()) as _, cchwidechar) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WideCharToMultiByte(codepage: super::UINT, dwflags: super::DWORD, lpwidecharstr: super::LPCWCH, cchwidechar: i32, lpmultibytestr: Option<windows_core::PSTR>, cbmultibyte: i32, lpdefaultchar: Option<super::LPCCH>, lpuseddefaultchar: Option<super::LPBOOL>) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn WideCharToMultiByte(codepage : super::UINT, dwflags : super::DWORD, lpwidecharstr : super::LPCWCH, cchwidechar : i32, lpmultibytestr : windows_core::PSTR, cbmultibyte : i32, lpdefaultchar : super::LPCCH, lpuseddefaultchar : super::LPBOOL) -> i32);
    unsafe { WideCharToMultiByte(codepage, dwflags, lpwidecharstr, cchwidechar, lpmultibytestr.unwrap_or(core::mem::zeroed()) as _, cbmultibyte, lpdefaultchar.unwrap_or(core::mem::zeroed()) as _, lpuseddefaultchar.unwrap_or(core::mem::zeroed()) as _) }
}
