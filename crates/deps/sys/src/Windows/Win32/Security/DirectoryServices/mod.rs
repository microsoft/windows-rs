#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_DirectoryServices`, `Win32_Foundation`, `Win32_Security_Authorization_UI`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
    pub fn DSCreateISecurityInfoObject(pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, ppsi: *mut ::windows::runtime::RawPtr, pfnreadsd: ::windows::runtime::RawPtr, pfnwritesd: ::windows::runtime::RawPtr, lpcontext: super::super::Foundation::LPARAM) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_DirectoryServices`, `Win32_Foundation`, `Win32_Security_Authorization_UI`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
    pub fn DSCreateISecurityInfoObjectEx(pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, pwszserver: super::super::Foundation::PWSTR, pwszusername: super::super::Foundation::PWSTR, pwszpassword: super::super::Foundation::PWSTR, dwflags: u32, ppsi: *mut ::windows::runtime::RawPtr, pfnreadsd: ::windows::runtime::RawPtr, pfnwritesd: ::windows::runtime::RawPtr, lpcontext: super::super::Foundation::LPARAM) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_DirectoryServices`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn DSCreateSecurityPage(pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, phpage: *mut super::super::UI::Controls::HPROPSHEETPAGE, pfnreadsd: ::windows::runtime::RawPtr, pfnwritesd: ::windows::runtime::RawPtr, lpcontext: super::super::Foundation::LPARAM) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Security_DirectoryServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSEditSecurity(hwndowner: super::super::Foundation::HWND, pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, pwszcaption: super::super::Foundation::PWSTR, pfnreadsd: ::windows::runtime::RawPtr, pfnwritesd: ::windows::runtime::RawPtr, lpcontext: super::super::Foundation::LPARAM) -> ::windows::runtime::HRESULT;
}
