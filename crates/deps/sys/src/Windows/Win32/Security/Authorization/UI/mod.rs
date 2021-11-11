#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_UI_Controls`*"]
    #[cfg(feature = "Win32_UI_Controls")]
    pub fn CreateSecurityPage();
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EditSecurity();
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EditSecurityAdvanced();
}
