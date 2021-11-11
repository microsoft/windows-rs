#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Recovery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplicationRecoveryFinished();
    #[doc = "*Required features: `Win32_System_Recovery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplicationRecoveryInProgress();
    #[doc = "*Required features: `Win32_System_Recovery`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn GetApplicationRecoveryCallback();
    #[doc = "*Required features: `Win32_System_Recovery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetApplicationRestartSettings();
    #[doc = "*Required features: `Win32_System_Recovery`, `Win32_System_WindowsProgramming`*"]
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn RegisterApplicationRecoveryCallback();
    #[doc = "*Required features: `Win32_System_Recovery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterApplicationRestart();
    #[doc = "*Required features: `Win32_System_Recovery`*"]
    pub fn UnregisterApplicationRecoveryCallback();
    #[doc = "*Required features: `Win32_System_Recovery`*"]
    pub fn UnregisterApplicationRestart();
}
