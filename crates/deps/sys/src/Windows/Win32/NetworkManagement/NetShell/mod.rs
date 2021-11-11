#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_NetShell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MatchEnumTag();
    #[doc = "*Required features: `Win32_NetworkManagement_NetShell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MatchToken();
    #[doc = "*Required features: `Win32_NetworkManagement_NetShell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PreprocessCommand();
    #[doc = "*Required features: `Win32_NetworkManagement_NetShell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrintError();
    #[doc = "*Required features: `Win32_NetworkManagement_NetShell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrintMessage();
    #[doc = "*Required features: `Win32_NetworkManagement_NetShell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrintMessageFromModule();
    #[doc = "*Required features: `Win32_NetworkManagement_NetShell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterContext();
    #[doc = "*Required features: `Win32_NetworkManagement_NetShell`*"]
    pub fn RegisterHelper();
}
