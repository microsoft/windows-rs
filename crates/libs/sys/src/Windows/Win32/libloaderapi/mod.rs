windows_link::link!("kernel32.dll" "system" fn AddDllDirectory(newdirectory : windows_sys::core::PCWSTR) -> DLL_DIRECTORY_COOKIE);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn DisableThreadLibraryCalls(hlibmodule : super::HMODULE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceLanguagesExA(hmodule : super::HMODULE, lptype : windows_sys::core::PCSTR, lpname : windows_sys::core::PCSTR, lpenumfunc : ENUMRESLANGPROCA, lparam : super::LONG_PTR, dwflags : super::DWORD, langid : super::LANGID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceLanguagesExW(hmodule : super::HMODULE, lptype : windows_sys::core::PCWSTR, lpname : windows_sys::core::PCWSTR, lpenumfunc : ENUMRESLANGPROCW, lparam : super::LONG_PTR, dwflags : super::DWORD, langid : super::LANGID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceNamesA(hmodule : super::HMODULE, lptype : windows_sys::core::PCSTR, lpenumfunc : ENUMRESNAMEPROCA, lparam : super::LONG_PTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceNamesExA(hmodule : super::HMODULE, lptype : windows_sys::core::PCSTR, lpenumfunc : ENUMRESNAMEPROCA, lparam : super::LONG_PTR, dwflags : super::DWORD, langid : super::LANGID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceNamesExW(hmodule : super::HMODULE, lptype : windows_sys::core::PCWSTR, lpenumfunc : ENUMRESNAMEPROCW, lparam : super::LONG_PTR, dwflags : super::DWORD, langid : super::LANGID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceNamesW(hmodule : super::HMODULE, lptype : windows_sys::core::PCWSTR, lpenumfunc : ENUMRESNAMEPROCW, lparam : super::LONG_PTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceTypesExA(hmodule : super::HMODULE, lpenumfunc : ENUMRESTYPEPROCA, lparam : super::LONG_PTR, dwflags : super::DWORD, langid : super::LANGID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceTypesExW(hmodule : super::HMODULE, lpenumfunc : ENUMRESTYPEPROCW, lparam : super::LONG_PTR, dwflags : super::DWORD, langid : super::LANGID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn FindResourceExW(hmodule : super::HMODULE, lptype : windows_sys::core::PCWSTR, lpname : windows_sys::core::PCWSTR, wlanguage : super::WORD) -> super::HRSRC);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn FindResourceW(hmodule : super::HMODULE, lpname : windows_sys::core::PCWSTR, lptype : windows_sys::core::PCWSTR) -> super::HRSRC);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn FindStringOrdinal(dwfindstringordinalflags : super::DWORD, lpstringsource : windows_sys::core::PCWSTR, cchsource : i32, lpstringvalue : windows_sys::core::PCWSTR, cchvalue : i32, bignorecase : windows_sys::core::BOOL) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : super::HMODULE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn FreeLibraryAndExitThread(hlibmodule : super::HMODULE, dwexitcode : super::DWORD) -> !);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn FreeResource(hresdata : super::HGLOBAL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetModuleFileNameA(hmodule : super::HMODULE, lpfilename : windows_sys::core::PSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetModuleFileNameW(hmodule : super::HMODULE, lpfilename : windows_sys::core::PWSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetModuleHandleA(lpmodulename : windows_sys::core::PCSTR) -> super::HMODULE);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetModuleHandleExA(dwflags : super::DWORD, lpmodulename : windows_sys::core::PCSTR, phmodule : *mut super::HMODULE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetModuleHandleExW(dwflags : super::DWORD, lpmodulename : windows_sys::core::PCWSTR, phmodule : *mut super::HMODULE) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetModuleHandleW(lpmodulename : windows_sys::core::PCWSTR) -> super::HMODULE);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : super::HMODULE, lpprocname : windows_sys::core::PCSTR) -> super::FARPROC);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn LoadLibraryA(lplibfilename : windows_sys::core::PCSTR) -> super::HMODULE);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : windows_sys::core::PCSTR, hfile : super::HANDLE, dwflags : super::DWORD) -> super::HMODULE);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LoadLibraryExW(lplibfilename : windows_sys::core::PCWSTR, hfile : super::HANDLE, dwflags : super::DWORD) -> super::HMODULE);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn LoadLibraryW(lplibfilename : windows_sys::core::PCWSTR) -> super::HMODULE);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn LoadResource(hmodule : super::HMODULE, hresinfo : super::HRSRC) -> super::HGLOBAL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("user32.dll" "system" fn LoadStringA(hinstance : super::HINSTANCE, uid : super::UINT, lpbuffer : windows_sys::core::PSTR, cchbuffermax : i32) -> i32);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("user32.dll" "system" fn LoadStringW(hinstance : super::HINSTANCE, uid : super::UINT, lpbuffer : windows_sys::core::PWSTR, cchbuffermax : i32) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn LockResource(hresdata : super::HGLOBAL) -> super::LPVOID);
windows_link::link!("kernel32.dll" "system" fn RemoveDllDirectory(cookie : DLL_DIRECTORY_COOKIE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn SetDefaultDllDirectories(directoryflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn SizeofResource(hmodule : super::HMODULE, hresinfo : super::HRSRC) -> super::DWORD);
pub const CURRENT_IMPORT_REDIRECTION_VERSION: i32 = 1;
pub type DLL_DIRECTORY_COOKIE = *mut core::ffi::c_void;
pub const DONT_RESOLVE_DLL_REFERENCES: i32 = 1;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type ENUMRESLANGPROCA = Option<unsafe extern "system" fn(hmodule: super::HMODULE, lptype: windows_sys::core::PCSTR, lpname: windows_sys::core::PCSTR, wlanguage: super::WORD, lparam: super::LONG_PTR) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type ENUMRESLANGPROCW = Option<unsafe extern "system" fn(hmodule: super::HMODULE, lptype: windows_sys::core::PCWSTR, lpname: windows_sys::core::PCWSTR, wlanguage: super::WORD, lparam: super::LONG_PTR) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type ENUMRESNAMEPROCA = Option<unsafe extern "system" fn(hmodule: super::HMODULE, lptype: windows_sys::core::PCSTR, lpname: windows_sys::core::PCSTR, lparam: super::LONG_PTR) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type ENUMRESNAMEPROCW = Option<unsafe extern "system" fn(hmodule: super::HMODULE, lptype: windows_sys::core::PCWSTR, lpname: windows_sys::core::PCWSTR, lparam: super::LONG_PTR) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type ENUMRESTYPEPROCA = Option<unsafe extern "system" fn(hmodule: super::HMODULE, lptype: windows_sys::core::PCSTR, lparam: super::LONG_PTR) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type ENUMRESTYPEPROCW = Option<unsafe extern "system" fn(hmodule: super::HMODULE, lptype: windows_sys::core::PCWSTR, lparam: super::LONG_PTR) -> windows_sys::core::BOOL>;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ENUMUILANG {
    pub NumOfEnumUILang: super::ULONG,
    pub SizeOfEnumUIBuffer: super::ULONG,
    pub pEnumUIBuffer: *mut super::LANGID,
}
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: i32 = 1024;
pub const FIND_RESOURCE_DIRECTORY_NAMES: i32 = 512;
pub const FIND_RESOURCE_DIRECTORY_TYPES: i32 = 256;
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: i32 = 4;
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: i32 = 1;
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: i32 = 2;
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: i32 = 16;
pub const LOAD_LIBRARY_AS_DATAFILE: i32 = 2;
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: i32 = 64;
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: i32 = 32;
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: i32 = 32768;
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: i32 = 128;
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: i32 = 8192;
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: i32 = 512;
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: i32 = 4096;
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: i32 = 256;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: i32 = 2048;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER: i32 = 16384;
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: i32 = 1024;
pub const LOAD_WITH_ALTERED_SEARCH_PATH: i32 = 8;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCREDIRECTION_DESCRIPTOR = *const REDIRECTION_DESCRIPTOR;
#[cfg(feature = "winnt")]
pub type PCREDIRECTION_FUNCTION_DESCRIPTOR = *const REDIRECTION_FUNCTION_DESCRIPTOR;
pub type PDLL_DIRECTORY_COOKIE = *mut *mut core::ffi::c_void;
#[cfg(feature = "shared")]
pub type PENUMUILANG = *mut ENUMUILANG;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PGET_MODULE_HANDLE_EXA = Option<unsafe extern "system" fn(dwflags: super::DWORD, lpmodulename: windows_sys::core::PCSTR, phmodule: *mut super::HMODULE) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PGET_MODULE_HANDLE_EXW = Option<unsafe extern "system" fn(dwflags: super::DWORD, lpmodulename: windows_sys::core::PCWSTR, phmodule: *mut super::HMODULE) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PREDIRECTION_DESCRIPTOR = *mut REDIRECTION_DESCRIPTOR;
#[cfg(feature = "winnt")]
pub type PREDIRECTION_FUNCTION_DESCRIPTOR = *mut REDIRECTION_FUNCTION_DESCRIPTOR;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct REDIRECTION_DESCRIPTOR {
    pub Version: super::ULONG,
    pub FunctionCount: super::ULONG,
    pub Redirections: PCREDIRECTION_FUNCTION_DESCRIPTOR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct REDIRECTION_FUNCTION_DESCRIPTOR {
    pub DllName: windows_sys::core::PCSTR,
    pub FunctionName: windows_sys::core::PCSTR,
    pub RedirectionTarget: super::PVOID,
}
pub const RESOURCE_ENUM_LN: i32 = 1;
pub const RESOURCE_ENUM_MODULE_EXACT: i32 = 16;
pub const RESOURCE_ENUM_MUI: i32 = 2;
pub const RESOURCE_ENUM_MUI_SYSTEM: i32 = 4;
pub const RESOURCE_ENUM_VALIDATE: i32 = 8;
pub const SUPPORT_LANG_NUMBER: i32 = 32;
