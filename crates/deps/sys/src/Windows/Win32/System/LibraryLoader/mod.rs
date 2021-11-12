#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddDllDirectory(newdirectory: super::super::Foundation::PWSTR) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginUpdateResourceA(pfilename: super::super::Foundation::PSTR, bdeleteexistingresources: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginUpdateResourceW(pfilename: super::super::Foundation::PWSTR, bdeleteexistingresources: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisableThreadLibraryCalls(hlibmodule: super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndUpdateResourceA(hupdate: super::super::Foundation::HANDLE, fdiscard: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndUpdateResourceW(hupdate: super::super::Foundation::HANDLE, fdiscard: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceLanguagesA(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpname: super::super::Foundation::PSTR, lpenumfunc: ENUMRESLANGPROCA, lparam: isize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceLanguagesExA(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpname: super::super::Foundation::PSTR, lpenumfunc: ENUMRESLANGPROCA, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceLanguagesExW(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, lpenumfunc: ENUMRESLANGPROCW, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceLanguagesW(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, lpenumfunc: ENUMRESLANGPROCW, lparam: isize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceNamesA(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceNamesExA(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceNamesExW(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceNamesW(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceTypesA(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceTypesExA(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceTypesExW(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceTypesW(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindResourceA(hmodule: super::super::Foundation::HINSTANCE, lpname: super::super::Foundation::PSTR, lptype: super::super::Foundation::PSTR) -> super::super::Foundation::HRSRC;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindResourceExA(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpname: super::super::Foundation::PSTR, wlanguage: u16) -> super::super::Foundation::HRSRC;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindResourceExW(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, wlanguage: u16) -> super::super::Foundation::HRSRC;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindResourceW(hmodule: super::super::Foundation::HINSTANCE, lpname: super::super::Foundation::PWSTR, lptype: super::super::Foundation::PWSTR) -> super::super::Foundation::HRSRC;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeLibrary(hlibmodule: super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeLibraryAndExitThread(hlibmodule: super::super::Foundation::HINSTANCE, dwexitcode: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeResource(hresdata: isize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDllDirectoryA(nbufferlength: u32, lpbuffer: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDllDirectoryW(nbufferlength: u32, lpbuffer: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleFileNameA(hmodule: super::super::Foundation::HINSTANCE, lpfilename: super::super::Foundation::PSTR, nsize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleFileNameW(hmodule: super::super::Foundation::HINSTANCE, lpfilename: super::super::Foundation::PWSTR, nsize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleHandleA(lpmodulename: super::super::Foundation::PSTR) -> super::super::Foundation::HINSTANCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleHandleExA(dwflags: u32, lpmodulename: super::super::Foundation::PSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleHandleExW(dwflags: u32, lpmodulename: super::super::Foundation::PWSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleHandleW(lpmodulename: super::super::Foundation::PWSTR) -> super::super::Foundation::HINSTANCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcAddress(hmodule: super::super::Foundation::HINSTANCE, lpprocname: super::super::Foundation::PSTR) -> ::core::option::Option<super::super::Foundation::FARPROC>;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadLibraryA(lplibfilename: super::super::Foundation::PSTR) -> super::super::Foundation::HINSTANCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadLibraryExA(lplibfilename: super::super::Foundation::PSTR, hfile: super::super::Foundation::HANDLE, dwflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::HINSTANCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadLibraryExW(lplibfilename: super::super::Foundation::PWSTR, hfile: super::super::Foundation::HANDLE, dwflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::HINSTANCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadLibraryW(lplibfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::HINSTANCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadModule(lpmodulename: super::super::Foundation::PSTR, lpparameterblock: *const ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadPackagedLibrary(lpwlibfilename: super::super::Foundation::PWSTR, reserved: u32) -> super::super::Foundation::HINSTANCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadResource(hmodule: super::super::Foundation::HINSTANCE, hresinfo: super::super::Foundation::HRSRC) -> isize;
    pub fn LockResource(hresdata: isize) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDllDirectory(cookie: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDefaultDllDirectories(directoryflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDllDirectoryA(lppathname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDllDirectoryW(lppathname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SizeofResource(hmodule: super::super::Foundation::HINSTANCE, hresinfo: super::super::Foundation::HRSRC) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateResourceA(hupdate: super::super::Foundation::HANDLE, lptype: super::super::Foundation::PSTR, lpname: super::super::Foundation::PSTR, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateResourceW(hupdate: super::super::Foundation::HANDLE, lptype: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL;
}
pub const CURRENT_IMPORT_REDIRECTION_VERSION: u32 = 1u32;
pub struct ENUMRESLANGPROCA(i32);
pub struct ENUMRESLANGPROCW(i32);
pub struct ENUMRESNAMEPROCA(i32);
pub struct ENUMRESNAMEPROCW(i32);
pub struct ENUMRESTYPEPROCA(i32);
pub struct ENUMRESTYPEPROCW(i32);
pub struct ENUMUILANG(i32);
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: u32 = 1024u32;
pub const FIND_RESOURCE_DIRECTORY_NAMES: u32 = 512u32;
pub const FIND_RESOURCE_DIRECTORY_TYPES: u32 = 256u32;
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 4u32;
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 1u32;
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 2u32;
pub struct LOAD_LIBRARY_FLAGS(i32);
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: u32 = 32768u32;
pub struct PGET_MODULE_HANDLE_EXA(i32);
pub struct PGET_MODULE_HANDLE_EXW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct REDIRECTION_DESCRIPTOR(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct REDIRECTION_FUNCTION_DESCRIPTOR(i32);
pub const RESOURCE_ENUM_LN: u32 = 1u32;
pub const RESOURCE_ENUM_MODULE_EXACT: u32 = 16u32;
pub const RESOURCE_ENUM_MUI: u32 = 2u32;
pub const RESOURCE_ENUM_MUI_SYSTEM: u32 = 4u32;
pub const RESOURCE_ENUM_VALIDATE: u32 = 8u32;
pub const SUPPORT_LANG_NUMBER: u32 = 32u32;
