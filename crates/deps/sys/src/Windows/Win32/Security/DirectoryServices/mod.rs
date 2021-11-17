#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
    pub fn DSCreateISecurityInfoObject(pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, ppsi: *mut super::Authorization::UI::ISecurityInformation, pfnreadsd: ::core::option::Option<PFNREADOBJECTSECURITY>, pfnwritesd: ::core::option::Option<PFNWRITEOBJECTSECURITY>, lpcontext: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
    pub fn DSCreateISecurityInfoObjectEx(
        pwszobjectpath: super::super::Foundation::PWSTR,
        pwszobjectclass: super::super::Foundation::PWSTR,
        pwszserver: super::super::Foundation::PWSTR,
        pwszusername: super::super::Foundation::PWSTR,
        pwszpassword: super::super::Foundation::PWSTR,
        dwflags: u32,
        ppsi: *mut super::Authorization::UI::ISecurityInformation,
        pfnreadsd: ::core::option::Option<PFNREADOBJECTSECURITY>,
        pfnwritesd: ::core::option::Option<PFNWRITEOBJECTSECURITY>,
        lpcontext: super::super::Foundation::LPARAM,
    ) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn DSCreateSecurityPage(pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, phpage: *mut super::super::UI::Controls::HPROPSHEETPAGE, pfnreadsd: ::core::option::Option<PFNREADOBJECTSECURITY>, pfnwritesd: ::core::option::Option<PFNWRITEOBJECTSECURITY>, lpcontext: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSEditSecurity(hwndowner: super::super::Foundation::HWND, pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, pwszcaption: super::super::Foundation::PWSTR, pfnreadsd: ::core::option::Option<PFNREADOBJECTSECURITY>, pfnwritesd: ::core::option::Option<PFNWRITEOBJECTSECURITY>, lpcontext: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
}
pub const DSSI_IS_ROOT: u32 = 16u32;
pub const DSSI_NO_ACCESS_CHECK: u32 = 2u32;
pub const DSSI_NO_EDIT_OWNER: u32 = 8u32;
pub const DSSI_NO_EDIT_SACL: u32 = 4u32;
pub const DSSI_NO_FILTER: u32 = 32u32;
pub const DSSI_NO_READONLY_MESSAGE: u32 = 64u32;
pub const DSSI_READ_ONLY: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
pub type PFNDSCREATEISECINFO = unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: u32, param3: *mut super::Authorization::UI::ISecurityInformation, param4: ::core::option::Option<PFNREADOBJECTSECURITY>, param5: ::core::option::Option<PFNWRITEOBJECTSECURITY>, param6: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
pub type PFNDSCREATEISECINFOEX =
    unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: super::super::Foundation::PWSTR, param3: super::super::Foundation::PWSTR, param4: super::super::Foundation::PWSTR, param5: u32, param6: *mut super::Authorization::UI::ISecurityInformation, param7: ::core::option::Option<PFNREADOBJECTSECURITY>, param8: ::core::option::Option<PFNWRITEOBJECTSECURITY>, param9: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type PFNDSCREATESECPAGE = unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: u32, param3: *mut super::super::UI::Controls::HPROPSHEETPAGE, param4: ::core::option::Option<PFNREADOBJECTSECURITY>, param5: ::core::option::Option<PFNWRITEOBJECTSECURITY>, param6: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PFNDSEDITSECURITY = unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: super::super::Foundation::PWSTR, param2: super::super::Foundation::PWSTR, param3: u32, param4: super::super::Foundation::PWSTR, param5: ::core::option::Option<PFNREADOBJECTSECURITY>, param6: ::core::option::Option<PFNWRITEOBJECTSECURITY>, param7: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PFNREADOBJECTSECURITY = unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut *mut super::SECURITY_DESCRIPTOR, param3: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PFNWRITEOBJECTSECURITY = unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut super::SECURITY_DESCRIPTOR, param3: super::super::Foundation::LPARAM) -> ::windows_sys::core::HRESULT;
