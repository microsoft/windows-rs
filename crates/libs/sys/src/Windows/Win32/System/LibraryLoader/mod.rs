#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
    pub fn AddDllDirectory(newdirectory: ::windows_sys::core::PCWSTR) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginUpdateResourceA(pfilename: ::windows_sys::core::PCSTR, bdeleteexistingresources: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginUpdateResourceW(pfilename: ::windows_sys::core::PCWSTR, bdeleteexistingresources: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisableThreadLibraryCalls(hlibmodule: super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndUpdateResourceA(hupdate: super::super::Foundation::HANDLE, fdiscard: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndUpdateResourceW(hupdate: super::super::Foundation::HANDLE, fdiscard: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceLanguagesA(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows_sys::core::PCSTR, lpname: ::windows_sys::core::PCSTR, lpenumfunc: ENUMRESLANGPROCA, lparam: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceLanguagesExA(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows_sys::core::PCSTR, lpname: ::windows_sys::core::PCSTR, lpenumfunc: ENUMRESLANGPROCA, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceLanguagesExW(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows_sys::core::PCWSTR, lpname: ::windows_sys::core::PCWSTR, lpenumfunc: ENUMRESLANGPROCW, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceLanguagesW(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows_sys::core::PCWSTR, lpname: ::windows_sys::core::PCWSTR, lpenumfunc: ENUMRESLANGPROCW, lparam: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceNamesA(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows_sys::core::PCSTR, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceNamesExA(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows_sys::core::PCSTR, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceNamesExW(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows_sys::core::PCWSTR, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceNamesW(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows_sys::core::PCWSTR, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceTypesA(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceTypesExA(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceTypesExW(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceTypesW(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindResourceA(hmodule: super::super::Foundation::HINSTANCE, lpname: ::windows_sys::core::PCSTR, lptype: ::windows_sys::core::PCSTR) -> super::super::Foundation::HRSRC;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindResourceExA(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows_sys::core::PCSTR, lpname: ::windows_sys::core::PCSTR, wlanguage: u16) -> super::super::Foundation::HRSRC;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindResourceExW(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows_sys::core::PCWSTR, lpname: ::windows_sys::core::PCWSTR, wlanguage: u16) -> super::super::Foundation::HRSRC;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindResourceW(hmodule: super::super::Foundation::HINSTANCE, lpname: ::windows_sys::core::PCWSTR, lptype: ::windows_sys::core::PCWSTR) -> super::super::Foundation::HRSRC;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeLibrary(hlibmodule: super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeLibraryAndExitThread(hlibmodule: super::super::Foundation::HINSTANCE, dwexitcode: u32) -> !;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeResource(hresdata: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
    pub fn GetDllDirectoryA(nbufferlength: u32, lpbuffer: ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
    pub fn GetDllDirectoryW(nbufferlength: u32, lpbuffer: ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleFileNameA(hmodule: super::super::Foundation::HINSTANCE, lpfilename: ::windows_sys::core::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleFileNameW(hmodule: super::super::Foundation::HINSTANCE, lpfilename: ::windows_sys::core::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleHandleA(lpmodulename: ::windows_sys::core::PCSTR) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleHandleExA(dwflags: u32, lpmodulename: ::windows_sys::core::PCSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleHandleExW(dwflags: u32, lpmodulename: ::windows_sys::core::PCWSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleHandleW(lpmodulename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcAddress(hmodule: super::super::Foundation::HINSTANCE, lpprocname: ::windows_sys::core::PCSTR) -> super::super::Foundation::FARPROC;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadLibraryA(lplibfilename: ::windows_sys::core::PCSTR) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadLibraryExA(lplibfilename: ::windows_sys::core::PCSTR, hfile: super::super::Foundation::HANDLE, dwflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadLibraryExW(lplibfilename: ::windows_sys::core::PCWSTR, hfile: super::super::Foundation::HANDLE, dwflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadLibraryW(lplibfilename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
    pub fn LoadModule(lpmodulename: ::windows_sys::core::PCSTR, lpparameterblock: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadPackagedLibrary(lpwlibfilename: ::windows_sys::core::PCWSTR, reserved: u32) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadResource(hmodule: super::super::Foundation::HINSTANCE, hresinfo: super::super::Foundation::HRSRC) -> isize;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
    pub fn LockResource(hresdata: isize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDllDirectory(cookie: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDefaultDllDirectories(directoryflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDllDirectoryA(lppathname: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDllDirectoryW(lppathname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SizeofResource(hmodule: super::super::Foundation::HINSTANCE, hresinfo: super::super::Foundation::HRSRC) -> u32;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateResourceA(hupdate: super::super::Foundation::HANDLE, lptype: ::windows_sys::core::PCSTR, lpname: ::windows_sys::core::PCSTR, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateResourceW(hupdate: super::super::Foundation::HANDLE, lptype: ::windows_sys::core::PCWSTR, lpname: ::windows_sys::core::PCWSTR, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL;
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const CURRENT_IMPORT_REDIRECTION_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESLANGPROCA = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows_sys::core::PCSTR, lpname: ::windows_sys::core::PCSTR, wlanguage: u16, lparam: isize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESLANGPROCW = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows_sys::core::PCWSTR, lpname: ::windows_sys::core::PCWSTR, wlanguage: u16, lparam: isize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESNAMEPROCA = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows_sys::core::PCSTR, lpname: ::windows_sys::core::PCSTR, lparam: isize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESNAMEPROCW = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows_sys::core::PCWSTR, lpname: ::windows_sys::core::PCWSTR, lparam: isize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESTYPEPROCA = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows_sys::core::PCSTR, lparam: isize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESTYPEPROCW = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows_sys::core::PCWSTR, lparam: isize) -> super::super::Foundation::BOOL>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub struct ENUMUILANG {
    pub NumOfEnumUILang: u32,
    pub SizeOfEnumUIBuffer: u32,
    pub pEnumUIBuffer: *mut u16,
}
impl ::core::marker::Copy for ENUMUILANG {}
impl ::core::clone::Clone for ENUMUILANG {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const FIND_RESOURCE_DIRECTORY_NAMES: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const FIND_RESOURCE_DIRECTORY_TYPES: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub type LOAD_LIBRARY_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const DONT_RESOLVE_DLL_REFERENCES: LOAD_LIBRARY_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_AS_DATAFILE: LOAD_LIBRARY_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_WITH_ALTERED_SEARCH_PATH: LOAD_LIBRARY_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: LOAD_LIBRARY_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: LOAD_LIBRARY_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: LOAD_LIBRARY_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: LOAD_LIBRARY_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: LOAD_LIBRARY_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: LOAD_LIBRARY_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: LOAD_LIBRARY_FLAGS = 1024u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: LOAD_LIBRARY_FLAGS = 2048u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: LOAD_LIBRARY_FLAGS = 8192u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER: LOAD_LIBRARY_FLAGS = 16384u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PGET_MODULE_HANDLE_EXA = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: ::windows_sys::core::PCSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PGET_MODULE_HANDLE_EXW = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: ::windows_sys::core::PCWSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub struct REDIRECTION_DESCRIPTOR {
    pub Version: u32,
    pub FunctionCount: u32,
    pub Redirections: *mut REDIRECTION_FUNCTION_DESCRIPTOR,
}
impl ::core::marker::Copy for REDIRECTION_DESCRIPTOR {}
impl ::core::clone::Clone for REDIRECTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub struct REDIRECTION_FUNCTION_DESCRIPTOR {
    pub DllName: ::windows_sys::core::PCSTR,
    pub FunctionName: ::windows_sys::core::PCSTR,
    pub RedirectionTarget: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for REDIRECTION_FUNCTION_DESCRIPTOR {}
impl ::core::clone::Clone for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const RESOURCE_ENUM_LN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const RESOURCE_ENUM_MODULE_EXACT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const RESOURCE_ENUM_MUI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const RESOURCE_ENUM_MUI_SYSTEM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const RESOURCE_ENUM_VALIDATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const SUPPORT_LANG_NUMBER: u32 = 32u32;
