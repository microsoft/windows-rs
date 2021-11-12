#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_Security_DirectoryServices`*"]
pub const DSSI_IS_ROOT: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_DirectoryServices`*"]
pub const DSSI_NO_ACCESS_CHECK: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_DirectoryServices`*"]
pub const DSSI_NO_EDIT_OWNER: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_DirectoryServices`*"]
pub const DSSI_NO_EDIT_SACL: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_DirectoryServices`*"]
pub const DSSI_NO_FILTER: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_DirectoryServices`*"]
pub const DSSI_NO_READONLY_MESSAGE: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_DirectoryServices`*"]
pub const DSSI_READ_ONLY: u32 = 1u32;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_DirectoryServices`, `Win32_Foundation`, `Win32_Security_Authorization_UI`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
    pub fn DSCreateISecurityInfoObject(pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, ppsi: *mut super::Authorization::UI::ISecurityInformation, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DirectoryServices`, `Win32_Foundation`, `Win32_Security_Authorization_UI`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
    pub fn DSCreateISecurityInfoObjectEx(pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, pwszserver: super::super::Foundation::PWSTR, pwszusername: super::super::Foundation::PWSTR, pwszpassword: super::super::Foundation::PWSTR, dwflags: u32, ppsi: *mut super::Authorization::UI::ISecurityInformation, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DirectoryServices`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn DSCreateSecurityPage(pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, phpage: *mut super::super::UI::Controls::HPROPSHEETPAGE, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DirectoryServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSEditSecurity(hwndowner: super::super::Foundation::HWND, pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, pwszcaption: super::super::Foundation::PWSTR, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
}
