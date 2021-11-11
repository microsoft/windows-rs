#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn QOSAddSocketToFlow(qoshandle: super::super::Foundation::HANDLE, socket: super::super::Networking::WinSock::SOCKET, destaddr: *const super::super::Networking::WinSock::SOCKADDR, traffictype: QOS_TRAFFIC_TYPE, flags: u32, flowid: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn QOSCancel(qoshandle: super::super::Foundation::HANDLE, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QOSCloseHandle(qoshandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QOSCreateHandle(version: *const QOS_VERSION, qoshandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QOSEnumerateFlows(qoshandle: super::super::Foundation::HANDLE, size: *mut u32, buffer: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn QOSNotifyFlow(qoshandle: super::super::Foundation::HANDLE, flowid: u32, operation: QOS_NOTIFY_FLOW, size: *mut u32, buffer: *mut ::core::ffi::c_void, flags: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn QOSQueryFlow(qoshandle: super::super::Foundation::HANDLE, flowid: u32, operation: QOS_QUERY_FLOW, size: *mut u32, buffer: *mut ::core::ffi::c_void, flags: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn QOSRemoveSocketFromFlow(qoshandle: super::super::Foundation::HANDLE, socket: super::super::Networking::WinSock::SOCKET, flowid: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn QOSSetFlow(qoshandle: super::super::Foundation::HANDLE, flowid: u32, operation: QOS_SET_FLOW, size: u32, buffer: *const ::core::ffi::c_void, flags: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn QOSStartTrackingClient(qoshandle: super::super::Foundation::HANDLE, destaddr: *const super::super::Networking::WinSock::SOCKADDR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
    pub fn QOSStopTrackingClient(qoshandle: super::super::Foundation::HANDLE, destaddr: *const super::super::Networking::WinSock::SOCKADDR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcAddFilter(flowhandle: super::super::Foundation::HANDLE, pgenericfilter: *const TC_GEN_FILTER, pfilterhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcAddFlow(ifchandle: super::super::Foundation::HANDLE, clflowctx: super::super::Foundation::HANDLE, flags: u32, pgenericflow: *const TC_GEN_FLOW, pflowhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcCloseInterface(ifchandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcDeleteFilter(filterhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcDeleteFlow(flowhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcDeregisterClient(clienthandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcEnumerateFlows(ifchandle: super::super::Foundation::HANDLE, penumhandle: *mut super::super::Foundation::HANDLE, pflowcount: *mut u32, pbufsize: *mut u32, buffer: *mut ENUMERATION_BUFFER) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`, `Win32_NetworkManagement_Ndis`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn TcEnumerateInterfaces(clienthandle: super::super::Foundation::HANDLE, pbuffersize: *mut u32, interfacebuffer: *mut TC_IFC_DESCRIPTOR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcGetFlowNameA(flowhandle: super::super::Foundation::HANDLE, strsize: u32, pflowname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcGetFlowNameW(flowhandle: super::super::Foundation::HANDLE, strsize: u32, pflowname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcModifyFlow(flowhandle: super::super::Foundation::HANDLE, pgenericflow: *const TC_GEN_FLOW) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcOpenInterfaceA(pinterfacename: super::super::Foundation::PSTR, clienthandle: super::super::Foundation::HANDLE, clifcctx: super::super::Foundation::HANDLE, pifchandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcOpenInterfaceW(pinterfacename: super::super::Foundation::PWSTR, clienthandle: super::super::Foundation::HANDLE, clifcctx: super::super::Foundation::HANDLE, pifchandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcQueryFlowA(pflowname: super::super::Foundation::PSTR, pguidparam: *const ::windows::runtime::GUID, pbuffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcQueryFlowW(pflowname: super::super::Foundation::PWSTR, pguidparam: *const ::windows::runtime::GUID, pbuffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcQueryInterface(ifchandle: super::super::Foundation::HANDLE, pguidparam: *const ::windows::runtime::GUID, notifychange: super::super::Foundation::BOOLEAN, pbuffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcRegisterClient(tciversion: u32, clregctx: super::super::Foundation::HANDLE, clienthandlerlist: *const ::core::mem::ManuallyDrop<TCI_CLIENT_FUNC_LIST>, pclienthandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcSetFlowA(pflowname: super::super::Foundation::PSTR, pguidparam: *const ::windows::runtime::GUID, buffersize: u32, buffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcSetFlowW(pflowname: super::super::Foundation::PWSTR, pguidparam: *const ::windows::runtime::GUID, buffersize: u32, buffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcSetInterface(ifchandle: super::super::Foundation::HANDLE, pguidparam: *const ::windows::runtime::GUID, buffersize: u32, buffer: *const ::core::ffi::c_void) -> u32;
}
