#[cfg(all(feature = "minwindef", feature = "winnls"))]
windows_link::link!("kernel32.dll" "system" fn CompareStringEx(lplocalename : windows_sys::core::PCWSTR, dwcmpflags : u32, lpstring1 : *const u16, cchcount1 : i32, lpstring2 : *const u16, cchcount2 : i32, lpversioninformation : *const super::NLSVERSIONINFO, lpreserved : *const core::ffi::c_void, lparam : super::LPARAM) -> i32);
windows_link::link!("kernel32.dll" "system" fn CompareStringOrdinal(lpstring1 : *const u16, cchcount1 : i32, lpstring2 : *const u16, cchcount2 : i32, bignorecase : windows_sys::core::BOOL) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CompareStringW(locale : super::LCID, dwcmpflags : u32, lpstring1 : *const u16, cchcount1 : i32, lpstring2 : *const u16, cchcount2 : i32) -> i32);
windows_link::link!("kernel32.dll" "system" fn FoldStringW(dwmapflags : u32, lpsrcstr : *const u16, cchsrc : i32, lpdeststr : windows_sys::core::PWSTR, cchdest : i32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetStringTypeExW(locale : super::LCID, dwinfotype : u32, lpsrcstr : *const u16, cchsrc : i32, lpchartype : *mut u16) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetStringTypeW(dwinfotype : u32, lpsrcstr : *const u16, cchsrc : i32, lpchartype : *mut u16) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn MultiByteToWideChar(codepage : u32, dwflags : u32, lpmultibytestr : *const i8, cbmultibyte : i32, lpwidecharstr : windows_sys::core::PWSTR, cchwidechar : i32) -> i32);
windows_link::link!("kernel32.dll" "system" fn WideCharToMultiByte(codepage : u32, dwflags : u32, lpwidecharstr : *const u16, cchwidechar : i32, lpmultibytestr : windows_sys::core::PSTR, cbmultibyte : i32, lpdefaultchar : *const i8, lpuseddefaultchar : *mut windows_sys::core::BOOL) -> i32);
