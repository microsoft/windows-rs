#[cfg(all(feature = "minwindef", feature = "winnls", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CompareStringEx(lplocalename : windows_sys::core::PCWSTR, dwcmpflags : u32, lpstring1 : super::LPCWCH, cchcount1 : i32, lpstring2 : super::LPCWCH, cchcount2 : i32, lpversioninformation : super::LPNLSVERSIONINFO, lpreserved : *mut core::ffi::c_void, lparam : super::LPARAM) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CompareStringOrdinal(lpstring1 : super::LPCWCH, cchcount1 : i32, lpstring2 : super::LPCWCH, cchcount2 : i32, bignorecase : windows_sys::core::BOOL) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CompareStringW(locale : super::LCID, dwcmpflags : u32, lpstring1 : super::PCNZWCH, cchcount1 : i32, lpstring2 : super::PCNZWCH, cchcount2 : i32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FoldStringW(dwmapflags : u32, lpsrcstr : super::LPCWCH, cchsrc : i32, lpdeststr : windows_sys::core::PWSTR, cchdest : i32) -> i32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetStringTypeExW(locale : super::LCID, dwinfotype : u32, lpsrcstr : super::LPCWCH, cchsrc : i32, lpchartype : super::LPWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetStringTypeW(dwinfotype : u32, lpsrcstr : super::LPCWCH, cchsrc : i32, lpchartype : super::LPWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn MultiByteToWideChar(codepage : u32, dwflags : u32, lpmultibytestr : super::LPCCH, cbmultibyte : i32, lpwidecharstr : windows_sys::core::PWSTR, cchwidechar : i32) -> i32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WideCharToMultiByte(codepage : u32, dwflags : u32, lpwidecharstr : super::LPCWCH, cchwidechar : i32, lpmultibytestr : windows_sys::core::PSTR, cbmultibyte : i32, lpdefaultchar : super::LPCCH, lpuseddefaultchar : super::LPBOOL) -> i32);
