#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetworkIsolationDiagnoseConnectFailureAndGetInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationEnumAppContainers();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationFreeAppContainers();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationGetAppContainerConfig();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationRegisterForAppContainerChanges();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationSetAppContainerConfig();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetworkIsolationSetupAppContainerBinaries();
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetworkIsolationUnregisterForAppContainerChanges();
}
