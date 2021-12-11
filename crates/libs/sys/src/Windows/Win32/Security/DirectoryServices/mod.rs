#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: 'Win32_Security_DirectoryServices', 'Win32_Foundation', 'Win32_Security_Authorization_UI'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
    pub fn DSCreateISecurityInfoObject(pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, ppsi: *mut super::Authorization::UI::ISecurityInformation, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DirectoryServices', 'Win32_Foundation', 'Win32_Security_Authorization_UI'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
    pub fn DSCreateISecurityInfoObjectEx(pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, pwszserver: super::super::Foundation::PWSTR, pwszusername: super::super::Foundation::PWSTR, pwszpassword: super::super::Foundation::PWSTR, dwflags: u32, ppsi: *mut super::Authorization::UI::ISecurityInformation, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DirectoryServices', 'Win32_Foundation', 'Win32_UI_Controls'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn DSCreateSecurityPage(pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, phpage: *mut super::super::UI::Controls::HPROPSHEETPAGE, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DirectoryServices', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSEditSecurity(hwndowner: super::super::Foundation::HWND, pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, pwszcaption: super::super::Foundation::PWSTR, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: 'Win32_Security_DirectoryServices'*"]
pub const DSSI_IS_ROOT: u32 = 16u32;
#[doc = "*Required features: 'Win32_Security_DirectoryServices'*"]
pub const DSSI_NO_ACCESS_CHECK: u32 = 2u32;
#[doc = "*Required features: 'Win32_Security_DirectoryServices'*"]
pub const DSSI_NO_EDIT_OWNER: u32 = 8u32;
#[doc = "*Required features: 'Win32_Security_DirectoryServices'*"]
pub const DSSI_NO_EDIT_SACL: u32 = 4u32;
#[doc = "*Required features: 'Win32_Security_DirectoryServices'*"]
pub const DSSI_NO_FILTER: u32 = 32u32;
#[doc = "*Required features: 'Win32_Security_DirectoryServices'*"]
pub const DSSI_NO_READONLY_MESSAGE: u32 = 64u32;
#[doc = "*Required features: 'Win32_Security_DirectoryServices'*"]
pub const DSSI_READ_ONLY: u32 = 1u32;
#[doc = "*Required features: 'Win32_Security_DirectoryServices', 'Win32_Foundation', 'Win32_Security_Authorization_UI'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
pub type PFNDSCREATEISECINFO = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: u32, param3: *mut super::Authorization::UI::ISecurityInformation, param4: PFNREADOBJECTSECURITY, param5: PFNWRITEOBJECTSECURITY, param6: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: 'Win32_Security_DirectoryServices', 'Win32_Foundation', 'Win32_Security_Authorization_UI'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
pub type PFNDSCREATEISECINFOEX = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: super::super::Foundation::PWSTR, param3: super::super::Foundation::PWSTR, param4: super::super::Foundation::PWSTR, param5: u32, param6: *mut super::Authorization::UI::ISecurityInformation, param7: PFNREADOBJECTSECURITY, param8: PFNWRITEOBJECTSECURITY, param9: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: 'Win32_Security_DirectoryServices', 'Win32_Foundation', 'Win32_UI_Controls'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type PFNDSCREATESECPAGE = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: u32, param3: *mut super::super::UI::Controls::HPROPSHEETPAGE, param4: PFNREADOBJECTSECURITY, param5: PFNWRITEOBJECTSECURITY, param6: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: 'Win32_Security_DirectoryServices', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNDSEDITSECURITY = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: super::super::Foundation::PWSTR, param2: super::super::Foundation::PWSTR, param3: u32, param4: super::super::Foundation::PWSTR, param5: PFNREADOBJECTSECURITY, param6: PFNWRITEOBJECTSECURITY, param7: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: 'Win32_Security_DirectoryServices', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNREADOBJECTSECURITY = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut *mut super::SECURITY_DESCRIPTOR, param3: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: 'Win32_Security_DirectoryServices', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNWRITEOBJECTSECURITY = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut super::SECURITY_DESCRIPTOR, param3: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT>;
