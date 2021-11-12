#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
pub const E_UNKNOWNTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144665560i32 as _);
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeXamlDiagnostic(endpointname: super::super::super::Foundation::PWSTR, pid: u32, wszdllxamldiagnostics: super::super::super::Foundation::PWSTR, wsztapdllname: super::super::super::Foundation::PWSTR, tapclsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeXamlDiagnosticsEx(endpointname: super::super::super::Foundation::PWSTR, pid: u32, wszdllxamldiagnostics: super::super::super::Foundation::PWSTR, wsztapdllname: super::super::super::Foundation::PWSTR, tapclsid: ::windows_sys::core::GUID, wszinitializationdata: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
}
