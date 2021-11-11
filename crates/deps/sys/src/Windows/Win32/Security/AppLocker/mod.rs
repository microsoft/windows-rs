#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferCloseLevel();
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferComputeTokenFromLevel();
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferCreateLevel();
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferGetLevelInformation();
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferGetPolicyInformation();
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferIdentifyLevel();
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferRecordEventLogEntry();
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferSetLevelInformation();
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferSetPolicyInformation();
    #[doc = "*Required features: `Win32_Security_AppLocker`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SaferiIsExecutableFileType();
}
