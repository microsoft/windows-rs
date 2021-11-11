#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
    pub fn NdfCancelIncident();
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
    pub fn NdfCloseIncident();
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
    pub fn NdfCreateConnectivityIncident();
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreateDNSIncident();
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn NdfCreateGroupingIncident();
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreateIncident();
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
    pub fn NdfCreateNetConnectionIncident();
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreatePnrpIncident();
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreateSharingIncident();
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreateWebIncident();
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfCreateWebIncidentEx();
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`, `Win32_Networking_WinSock`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security"))]
    pub fn NdfCreateWinSockIncident();
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfDiagnoseIncident();
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfExecuteDiagnosis();
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfGetTraceFile();
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NdfRepairIncident();
}
