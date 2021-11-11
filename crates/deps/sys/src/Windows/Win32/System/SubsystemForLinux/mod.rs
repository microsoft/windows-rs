#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_SubsystemForLinux`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslConfigureDistribution();
    #[doc = "*Required features: `Win32_System_SubsystemForLinux`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslGetDistributionConfiguration();
    #[doc = "*Required features: `Win32_System_SubsystemForLinux`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslIsDistributionRegistered();
    #[doc = "*Required features: `Win32_System_SubsystemForLinux`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslLaunch();
    #[doc = "*Required features: `Win32_System_SubsystemForLinux`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslLaunchInteractive();
    #[doc = "*Required features: `Win32_System_SubsystemForLinux`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslRegisterDistribution();
    #[doc = "*Required features: `Win32_System_SubsystemForLinux`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslUnregisterDistribution();
}
