pub const CURRENT_IMPORT_REDIRECTION_VERSION: u32 = 1u32;
pub const DONT_RESOLVE_DLL_REFERENCES: LOAD_LIBRARY_FLAGS = 1u32;
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: u32 = 1024u32;
pub const FIND_RESOURCE_DIRECTORY_NAMES: u32 = 512u32;
pub const FIND_RESOURCE_DIRECTORY_TYPES: u32 = 256u32;
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 4u32;
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 1u32;
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 2u32;
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: LOAD_LIBRARY_FLAGS = 16u32;
pub const LOAD_LIBRARY_AS_DATAFILE: LOAD_LIBRARY_FLAGS = 2u32;
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: LOAD_LIBRARY_FLAGS = 64u32;
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: LOAD_LIBRARY_FLAGS = 32u32;
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: u32 = 32768u32;
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: LOAD_LIBRARY_FLAGS = 128u32;
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: LOAD_LIBRARY_FLAGS = 8192u32;
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: LOAD_LIBRARY_FLAGS = 512u32;
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = 4096u32;
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: LOAD_LIBRARY_FLAGS = 256u32;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: LOAD_LIBRARY_FLAGS = 2048u32;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER: LOAD_LIBRARY_FLAGS = 16384u32;
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: LOAD_LIBRARY_FLAGS = 1024u32;
pub const LOAD_WITH_ALTERED_SEARCH_PATH: LOAD_LIBRARY_FLAGS = 8u32;
pub const RESOURCE_ENUM_LN: u32 = 1u32;
pub const RESOURCE_ENUM_MODULE_EXACT: u32 = 16u32;
pub const RESOURCE_ENUM_MUI: u32 = 2u32;
pub const RESOURCE_ENUM_MUI_SYSTEM: u32 = 4u32;
pub const RESOURCE_ENUM_VALIDATE: u32 = 8u32;
pub const SUPPORT_LANG_NUMBER: u32 = 32u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LOAD_LIBRARY_FLAGS(pub u32);
impl windows_core::TypeKind for LOAD_LIBRARY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENUMUILANG {
    pub NumOfEnumUILang: u32,
    pub SizeOfEnumUIBuffer: u32,
    pub pEnumUIBuffer: *mut u16,
}
impl Default for ENUMUILANG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ENUMUILANG {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REDIRECTION_DESCRIPTOR {
    pub Version: u32,
    pub FunctionCount: u32,
    pub Redirections: *mut REDIRECTION_FUNCTION_DESCRIPTOR,
}
impl Default for REDIRECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REDIRECTION_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REDIRECTION_FUNCTION_DESCRIPTOR {
    pub DllName: windows_core::PCSTR,
    pub FunctionName: windows_core::PCSTR,
    pub RedirectionTarget: *mut core::ffi::c_void,
}
impl Default for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REDIRECTION_FUNCTION_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
pub type ENUMRESLANGPROCA = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCSTR, lpname: windows_core::PCSTR, wlanguage: u16, lparam: isize) -> super::super::Foundation::BOOL>;
pub type ENUMRESLANGPROCW = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCWSTR, lpname: windows_core::PCWSTR, wlanguage: u16, lparam: isize) -> super::super::Foundation::BOOL>;
pub type ENUMRESNAMEPROCA = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCSTR, lpname: windows_core::PCSTR, lparam: isize) -> super::super::Foundation::BOOL>;
pub type ENUMRESNAMEPROCW = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCWSTR, lpname: windows_core::PCWSTR, lparam: isize) -> super::super::Foundation::BOOL>;
pub type ENUMRESTYPEPROCA = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCSTR, lparam: isize) -> super::super::Foundation::BOOL>;
pub type ENUMRESTYPEPROCW = Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HMODULE, lptype: windows_core::PCWSTR, lparam: isize) -> super::super::Foundation::BOOL>;
pub type PGET_MODULE_HANDLE_EXA = Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: windows_core::PCSTR, phmodule: *mut super::super::Foundation::HMODULE) -> super::super::Foundation::BOOL>;
pub type PGET_MODULE_HANDLE_EXW = Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: windows_core::PCWSTR, phmodule: *mut super::super::Foundation::HMODULE) -> super::super::Foundation::BOOL>;
