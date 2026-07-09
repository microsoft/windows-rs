windows_link::link!("kernel32.dll" "system" fn AddDllDirectory(newdirectory : windows_sys::core::PCWSTR) -> DLL_DIRECTORY_COOKIE);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn DisableThreadLibraryCalls(hlibmodule : super::minwindef::HMODULE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceLanguagesExA(hmodule : super::minwindef::HMODULE, lptype : windows_sys::core::PCSTR, lpname : windows_sys::core::PCSTR, lpenumfunc : ENUMRESLANGPROCA, lparam : isize, dwflags : u32, langid : super::winnt::LANGID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceLanguagesExW(hmodule : super::minwindef::HMODULE, lptype : windows_sys::core::PCWSTR, lpname : windows_sys::core::PCWSTR, lpenumfunc : ENUMRESLANGPROCW, lparam : isize, dwflags : u32, langid : super::winnt::LANGID) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn EnumResourceNamesA(hmodule : super::minwindef::HMODULE, lptype : windows_sys::core::PCSTR, lpenumfunc : ENUMRESNAMEPROCA, lparam : isize) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceNamesExA(hmodule : super::minwindef::HMODULE, lptype : windows_sys::core::PCSTR, lpenumfunc : ENUMRESNAMEPROCA, lparam : isize, dwflags : u32, langid : super::winnt::LANGID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceNamesExW(hmodule : super::minwindef::HMODULE, lptype : windows_sys::core::PCWSTR, lpenumfunc : ENUMRESNAMEPROCW, lparam : isize, dwflags : u32, langid : super::winnt::LANGID) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn EnumResourceNamesW(hmodule : super::minwindef::HMODULE, lptype : windows_sys::core::PCWSTR, lpenumfunc : ENUMRESNAMEPROCW, lparam : isize) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceTypesExA(hmodule : super::minwindef::HMODULE, lpenumfunc : ENUMRESTYPEPROCA, lparam : isize, dwflags : u32, langid : super::winnt::LANGID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceTypesExW(hmodule : super::minwindef::HMODULE, lpenumfunc : ENUMRESTYPEPROCW, lparam : isize, dwflags : u32, langid : super::winnt::LANGID) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn FindResourceExW(hmodule : super::minwindef::HMODULE, lptype : windows_sys::core::PCWSTR, lpname : windows_sys::core::PCWSTR, wlanguage : u16) -> super::minwindef::HRSRC);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn FindResourceW(hmodule : super::minwindef::HMODULE, lpname : windows_sys::core::PCWSTR, lptype : windows_sys::core::PCWSTR) -> super::minwindef::HRSRC);
windows_link::link!("kernel32.dll" "system" fn FindStringOrdinal(dwfindstringordinalflags : u32, lpstringsource : windows_sys::core::PCWSTR, cchsource : i32, lpstringvalue : windows_sys::core::PCWSTR, cchvalue : i32, bignorecase : windows_sys::core::BOOL) -> i32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : super::minwindef::HMODULE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn FreeLibraryAndExitThread(hlibmodule : super::minwindef::HMODULE, dwexitcode : u32) -> !);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn FreeResource(hresdata : super::minwindef::HGLOBAL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetModuleFileNameA(hmodule : super::minwindef::HMODULE, lpfilename : windows_sys::core::PSTR, nsize : u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetModuleFileNameW(hmodule : super::minwindef::HMODULE, lpfilename : windows_sys::core::PWSTR, nsize : u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetModuleHandleA(lpmodulename : windows_sys::core::PCSTR) -> super::minwindef::HMODULE);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetModuleHandleExA(dwflags : u32, lpmodulename : windows_sys::core::PCSTR, phmodule : *mut super::minwindef::HMODULE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetModuleHandleExW(dwflags : u32, lpmodulename : windows_sys::core::PCWSTR, phmodule : *mut super::minwindef::HMODULE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetModuleHandleW(lpmodulename : windows_sys::core::PCWSTR) -> super::minwindef::HMODULE);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : super::minwindef::HMODULE, lpprocname : windows_sys::core::PCSTR) -> super::minwindef::FARPROC);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn LoadLibraryA(lplibfilename : windows_sys::core::PCSTR) -> super::minwindef::HMODULE);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : windows_sys::core::PCSTR, hfile : super::winnt::HANDLE, dwflags : u32) -> super::minwindef::HMODULE);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn LoadLibraryExW(lplibfilename : windows_sys::core::PCWSTR, hfile : super::winnt::HANDLE, dwflags : u32) -> super::minwindef::HMODULE);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn LoadLibraryW(lplibfilename : windows_sys::core::PCWSTR) -> super::minwindef::HMODULE);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn LoadResource(hmodule : super::minwindef::HMODULE, hresinfo : super::minwindef::HRSRC) -> super::minwindef::HGLOBAL);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("user32.dll" "system" fn LoadStringA(hinstance : super::minwindef::HINSTANCE, uid : u32, lpbuffer : windows_sys::core::PSTR, cchbuffermax : i32) -> i32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("user32.dll" "system" fn LoadStringW(hinstance : super::minwindef::HINSTANCE, uid : u32, lpbuffer : windows_sys::core::PWSTR, cchbuffermax : i32) -> i32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn LockResource(hresdata : super::minwindef::HGLOBAL) -> *mut core::ffi::c_void);
windows_link::link!("kernel32.dll" "system" fn RemoveDllDirectory(cookie : DLL_DIRECTORY_COOKIE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetDefaultDllDirectories(directoryflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn SizeofResource(hmodule : super::minwindef::HMODULE, hresinfo : super::minwindef::HRSRC) -> u32);
pub const CURRENT_IMPORT_REDIRECTION_VERSION: u32 = 1;
pub type DLL_DIRECTORY_COOKIE = *mut core::ffi::c_void;
pub const DONT_RESOLVE_DLL_REFERENCES: u32 = 1;
#[cfg(feature = "Win32_minwindef")]
pub type ENUMRESLANGPROCA = Option<unsafe extern "system" fn(hmodule: super::minwindef::HMODULE, lptype: windows_sys::core::PCSTR, lpname: windows_sys::core::PCSTR, wlanguage: u16, lparam: isize) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_minwindef")]
pub type ENUMRESLANGPROCW = Option<unsafe extern "system" fn(hmodule: super::minwindef::HMODULE, lptype: windows_sys::core::PCWSTR, lpname: windows_sys::core::PCWSTR, wlanguage: u16, lparam: isize) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_minwindef")]
pub type ENUMRESNAMEPROCA = Option<unsafe extern "system" fn(hmodule: super::minwindef::HMODULE, lptype: windows_sys::core::PCSTR, lpname: windows_sys::core::PCSTR, lparam: isize) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_minwindef")]
pub type ENUMRESNAMEPROCW = Option<unsafe extern "system" fn(hmodule: super::minwindef::HMODULE, lptype: windows_sys::core::PCWSTR, lpname: windows_sys::core::PCWSTR, lparam: isize) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_minwindef")]
pub type ENUMRESTYPEPROCA = Option<unsafe extern "system" fn(hmodule: super::minwindef::HMODULE, lptype: windows_sys::core::PCSTR, lparam: isize) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_minwindef")]
pub type ENUMRESTYPEPROCW = Option<unsafe extern "system" fn(hmodule: super::minwindef::HMODULE, lptype: windows_sys::core::PCWSTR, lparam: isize) -> windows_sys::core::BOOL>;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct ENUMUILANG {
    pub NumOfEnumUILang: u32,
    pub SizeOfEnumUIBuffer: u32,
    pub pEnumUIBuffer: *mut super::winnt::LANGID,
}
#[cfg(feature = "Win32_winnt")]
impl Default for ENUMUILANG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: u32 = 1024;
pub const FIND_RESOURCE_DIRECTORY_NAMES: u32 = 512;
pub const FIND_RESOURCE_DIRECTORY_TYPES: u32 = 256;
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 4;
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 1;
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 2;
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: u32 = 16;
pub const LOAD_LIBRARY_AS_DATAFILE: u32 = 2;
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: u32 = 64;
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: u32 = 32;
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: u32 = 32768;
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: u32 = 128;
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: u32 = 8192;
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: u32 = 512;
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: u32 = 4096;
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: u32 = 256;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 2048;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER: u32 = 16384;
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: u32 = 1024;
pub const LOAD_WITH_ALTERED_SEARCH_PATH: u32 = 8;
pub type PCREDIRECTION_DESCRIPTOR = *const REDIRECTION_DESCRIPTOR;
pub type PCREDIRECTION_FUNCTION_DESCRIPTOR = *const REDIRECTION_FUNCTION_DESCRIPTOR;
pub type PDLL_DIRECTORY_COOKIE = *mut *mut core::ffi::c_void;
#[cfg(feature = "Win32_winnt")]
pub type PENUMUILANG = *mut ENUMUILANG;
#[cfg(feature = "Win32_minwindef")]
pub type PGET_MODULE_HANDLE_EXA = Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: windows_sys::core::PCSTR, phmodule: *mut super::minwindef::HMODULE) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_minwindef")]
pub type PGET_MODULE_HANDLE_EXW = Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: windows_sys::core::PCWSTR, phmodule: *mut super::minwindef::HMODULE) -> windows_sys::core::BOOL>;
pub type PREDIRECTION_DESCRIPTOR = *mut REDIRECTION_DESCRIPTOR;
pub type PREDIRECTION_FUNCTION_DESCRIPTOR = *mut REDIRECTION_FUNCTION_DESCRIPTOR;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REDIRECTION_DESCRIPTOR {
    pub Version: u32,
    pub FunctionCount: u32,
    pub Redirections: PCREDIRECTION_FUNCTION_DESCRIPTOR,
}
impl Default for REDIRECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REDIRECTION_FUNCTION_DESCRIPTOR {
    pub DllName: windows_sys::core::PCSTR,
    pub FunctionName: windows_sys::core::PCSTR,
    pub RedirectionTarget: *mut core::ffi::c_void,
}
impl Default for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RESOURCE_ENUM_LN: u32 = 1;
pub const RESOURCE_ENUM_MODULE_EXACT: u32 = 16;
pub const RESOURCE_ENUM_MUI: u32 = 2;
pub const RESOURCE_ENUM_MUI_SYSTEM: u32 = 4;
pub const RESOURCE_ENUM_VALIDATE: u32 = 8;
pub const SUPPORT_LANG_NUMBER: u32 = 32;
