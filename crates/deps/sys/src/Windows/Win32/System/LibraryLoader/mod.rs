#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddDllDirectory(newdirectory: super::super::Foundation::PWSTR) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginUpdateResourceA(pfilename: super::super::Foundation::PSTR, bdeleteexistingresources: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginUpdateResourceW(pfilename: super::super::Foundation::PWSTR, bdeleteexistingresources: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisableThreadLibraryCalls(hlibmodule: super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndUpdateResourceA(hupdate: super::super::Foundation::HANDLE, fdiscard: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndUpdateResourceW(hupdate: super::super::Foundation::HANDLE, fdiscard: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceLanguagesA(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpname: super::super::Foundation::PSTR, lpenumfunc: ::windows::runtime::RawPtr, lparam: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceLanguagesExA(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpname: super::super::Foundation::PSTR, lpenumfunc: ::windows::runtime::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceLanguagesExW(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, lpenumfunc: ::windows::runtime::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceLanguagesW(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, lpenumfunc: ::windows::runtime::RawPtr, lparam: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceNamesA(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpenumfunc: ::windows::runtime::RawPtr, lparam: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceNamesExA(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpenumfunc: ::windows::runtime::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceNamesExW(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpenumfunc: ::windows::runtime::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceNamesW(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpenumfunc: ::windows::runtime::RawPtr, lparam: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceTypesA(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: ::windows::runtime::RawPtr, lparam: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceTypesExA(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: ::windows::runtime::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceTypesExW(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: ::windows::runtime::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceTypesW(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: ::windows::runtime::RawPtr, lparam: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindResourceA(hmodule: super::super::Foundation::HINSTANCE, lpname: super::super::Foundation::PSTR, lptype: super::super::Foundation::PSTR) -> super::super::Foundation::HRSRC;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindResourceExA(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpname: super::super::Foundation::PSTR, wlanguage: u16) -> super::super::Foundation::HRSRC;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindResourceExW(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, wlanguage: u16) -> super::super::Foundation::HRSRC;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindResourceW(hmodule: super::super::Foundation::HINSTANCE, lpname: super::super::Foundation::PWSTR, lptype: super::super::Foundation::PWSTR) -> super::super::Foundation::HRSRC;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeLibrary(hlibmodule: super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeLibraryAndExitThread(hlibmodule: super::super::Foundation::HINSTANCE, dwexitcode: u32);
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeResource(hresdata: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDllDirectoryA(nbufferlength: u32, lpbuffer: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDllDirectoryW(nbufferlength: u32, lpbuffer: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleFileNameA(hmodule: super::super::Foundation::HINSTANCE, lpfilename: super::super::Foundation::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleFileNameW(hmodule: super::super::Foundation::HINSTANCE, lpfilename: super::super::Foundation::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleHandleA(lpmodulename: super::super::Foundation::PSTR) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleHandleExA(dwflags: u32, lpmodulename: super::super::Foundation::PSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleHandleExW(dwflags: u32, lpmodulename: super::super::Foundation::PWSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleHandleW(lpmodulename: super::super::Foundation::PWSTR) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcAddress(hmodule: super::super::Foundation::HINSTANCE, lpprocname: super::super::Foundation::PSTR) -> ::core::option::Option<super::super::Foundation::FARPROC>;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadLibraryA(lplibfilename: super::super::Foundation::PSTR) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadLibraryExA(lplibfilename: super::super::Foundation::PSTR, hfile: super::super::Foundation::HANDLE, dwflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadLibraryExW(lplibfilename: super::super::Foundation::PWSTR, hfile: super::super::Foundation::HANDLE, dwflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadLibraryW(lplibfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadModule(lpmodulename: super::super::Foundation::PSTR, lpparameterblock: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadPackagedLibrary(lpwlibfilename: super::super::Foundation::PWSTR, reserved: u32) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadResource(hmodule: super::super::Foundation::HINSTANCE, hresinfo: super::super::Foundation::HRSRC) -> isize;
    #[doc = "*Required features: `Win32_System_LibraryLoader`*"]
    pub fn LockResource(hresdata: isize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDllDirectory(cookie: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDefaultDllDirectories(directoryflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDllDirectoryA(lppathname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDllDirectoryW(lppathname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SizeofResource(hmodule: super::super::Foundation::HINSTANCE, hresinfo: super::super::Foundation::HRSRC) -> u32;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateResourceA(hupdate: super::super::Foundation::HANDLE, lptype: super::super::Foundation::PSTR, lpname: super::super::Foundation::PSTR, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateResourceW(hupdate: super::super::Foundation::HANDLE, lptype: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL;
}
