#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddDllDirectory();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginUpdateResourceA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginUpdateResourceW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisableThreadLibraryCalls();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndUpdateResourceA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndUpdateResourceW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceLanguagesA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceLanguagesExA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceLanguagesExW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceLanguagesW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceNamesA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceNamesExA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceNamesExW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceNamesW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceTypesA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceTypesExA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceTypesExW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumResourceTypesW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindResourceA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindResourceExA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindResourceExW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindResourceW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeLibrary();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeLibraryAndExitThread();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeResource();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDllDirectoryA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDllDirectoryW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleFileNameA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleFileNameW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleHandleA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleHandleExA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleHandleExW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetModuleHandleW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcAddress();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadLibraryA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadLibraryExA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadLibraryExW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadLibraryW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadModule();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadPackagedLibrary();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadResource();
    #[doc = "*Required features: `Win32_System_LibraryLoader`*"]
    pub fn LockResource();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveDllDirectory();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDefaultDllDirectories();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDllDirectoryA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDllDirectoryW();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SizeofResource();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateResourceA();
    #[doc = "*Required features: `Win32_System_LibraryLoader`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateResourceW();
}
