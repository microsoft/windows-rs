#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_DirectoryServices`, `Win32_Foundation`, `Win32_Security_Authorization_UI`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
    pub fn DSCreateISecurityInfoObject();
    #[doc = "*Required features: `Win32_Security_DirectoryServices`, `Win32_Foundation`, `Win32_Security_Authorization_UI`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
    pub fn DSCreateISecurityInfoObjectEx();
    #[doc = "*Required features: `Win32_Security_DirectoryServices`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn DSCreateSecurityPage();
    #[doc = "*Required features: `Win32_Security_DirectoryServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSEditSecurity();
}
