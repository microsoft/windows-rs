#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AbortSystemShutdownA();
    #[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AbortSystemShutdownW();
    #[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckForHiberboot();
    #[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExitWindowsEx();
    #[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitiateShutdownA();
    #[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitiateShutdownW();
    #[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitiateSystemShutdownA();
    #[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitiateSystemShutdownExA();
    #[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitiateSystemShutdownExW();
    #[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitiateSystemShutdownW();
    #[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LockWorkStation();
    #[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShutdownBlockReasonCreate();
    #[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShutdownBlockReasonDestroy();
    #[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShutdownBlockReasonQuery();
}
