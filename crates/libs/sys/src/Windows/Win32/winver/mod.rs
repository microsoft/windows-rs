#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("version.dll" "system" fn GetFileVersionInfoA(lptstrfilename : windows_sys::core::PCSTR, dwhandle : super::DWORD, dwlen : super::DWORD, lpdata : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("version.dll" "system" fn GetFileVersionInfoExA(dwflags : super::DWORD, lpwstrfilename : windows_sys::core::PCSTR, dwhandle : super::DWORD, dwlen : super::DWORD, lpdata : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("version.dll" "system" fn GetFileVersionInfoExW(dwflags : super::DWORD, lpwstrfilename : windows_sys::core::PCWSTR, dwhandle : super::DWORD, dwlen : super::DWORD, lpdata : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("version.dll" "system" fn GetFileVersionInfoSizeA(lptstrfilename : windows_sys::core::PCSTR, lpdwhandle : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("version.dll" "system" fn GetFileVersionInfoSizeExA(dwflags : super::DWORD, lpwstrfilename : windows_sys::core::PCSTR, lpdwhandle : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("version.dll" "system" fn GetFileVersionInfoSizeExW(dwflags : super::DWORD, lpwstrfilename : windows_sys::core::PCWSTR, lpdwhandle : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("version.dll" "system" fn GetFileVersionInfoSizeW(lptstrfilename : windows_sys::core::PCWSTR, lpdwhandle : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("version.dll" "system" fn GetFileVersionInfoW(lptstrfilename : windows_sys::core::PCWSTR, dwhandle : super::DWORD, dwlen : super::DWORD, lpdata : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("version.dll" "system" fn VerFindFileA(uflags : super::DWORD, szfilename : windows_sys::core::PCSTR, szwindir : windows_sys::core::PCSTR, szappdir : windows_sys::core::PCSTR, szcurdir : windows_sys::core::PSTR, pucurdirlen : super::PUINT, szdestdir : windows_sys::core::PSTR, pudestdirlen : super::PUINT) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("version.dll" "system" fn VerFindFileW(uflags : super::DWORD, szfilename : windows_sys::core::PCWSTR, szwindir : windows_sys::core::PCWSTR, szappdir : windows_sys::core::PCWSTR, szcurdir : windows_sys::core::PWSTR, pucurdirlen : super::PUINT, szdestdir : windows_sys::core::PWSTR, pudestdirlen : super::PUINT) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("version.dll" "system" fn VerInstallFileA(uflags : super::DWORD, szsrcfilename : windows_sys::core::PCSTR, szdestfilename : windows_sys::core::PCSTR, szsrcdir : windows_sys::core::PCSTR, szdestdir : windows_sys::core::PCSTR, szcurdir : windows_sys::core::PCSTR, sztmpfile : windows_sys::core::PSTR, putmpfilelen : super::PUINT) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("version.dll" "system" fn VerInstallFileW(uflags : super::DWORD, szsrcfilename : windows_sys::core::PCWSTR, szdestfilename : windows_sys::core::PCWSTR, szsrcdir : windows_sys::core::PCWSTR, szdestdir : windows_sys::core::PCWSTR, szcurdir : windows_sys::core::PCWSTR, sztmpfile : windows_sys::core::PWSTR, putmpfilelen : super::PUINT) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn VerLanguageNameA(wlang : super::DWORD, szlang : windows_sys::core::PSTR, cchlang : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn VerLanguageNameW(wlang : super::DWORD, szlang : windows_sys::core::PWSTR, cchlang : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("version.dll" "system" fn VerQueryValueA(pblock : super::LPCVOID, lpsubblock : windows_sys::core::PCSTR, lplpbuffer : *mut super::LPVOID, pulen : super::PUINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("version.dll" "system" fn VerQueryValueW(pblock : super::LPCVOID, lpsubblock : windows_sys::core::PCWSTR, lplpbuffer : *mut super::LPVOID, pulen : super::PUINT) -> windows_sys::core::BOOL);
