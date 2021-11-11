#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn QOSAddSocketToFlow();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn QOSCancel();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QOSCloseHandle();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QOSCreateHandle();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QOSEnumerateFlows();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn QOSNotifyFlow();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn QOSQueryFlow();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn QOSRemoveSocketFromFlow();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn QOSSetFlow();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn QOSStartTrackingClient();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn QOSStopTrackingClient();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcAddFilter();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcAddFlow();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcCloseInterface();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcDeleteFilter();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcDeleteFlow();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcDeregisterClient();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcEnumerateFlows();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_NetworkManagement_Ndis`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn TcEnumerateInterfaces();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcGetFlowNameA();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcGetFlowNameW();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcModifyFlow();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcOpenInterfaceA();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcOpenInterfaceW();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcQueryFlowA();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcQueryFlowW();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcQueryInterface();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcRegisterClient();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcSetFlowA();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcSetFlowW();
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcSetInterface();
}
