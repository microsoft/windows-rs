#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_UI_Controls`*"]
    #[cfg(feature = "Win32_UI_Controls")]
    pub fn CreateSecurityPage(psi: ::windows::runtime::RawPtr) -> super::super::super::UI::Controls::HPROPSHEETPAGE;
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EditSecurity(hwndowner: super::super::super::Foundation::HWND, psi: ::windows::runtime::RawPtr) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EditSecurityAdvanced(hwndowner: super::super::super::Foundation::HWND, psi: ::windows::runtime::RawPtr, usipage: SI_PAGE_TYPE) -> ::windows::runtime::HRESULT;
}
