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
    pub fn TcQueryFlowA(pflowname: super::super::Foundation::PSTR, pguidparam: *const ::windows_sys::core::GUID, pbuffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcQueryFlowW(pflowname: super::super::Foundation::PWSTR, pguidparam: *const ::windows_sys::core::GUID, pbuffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcQueryInterface(ifchandle: super::super::Foundation::HANDLE, pguidparam: *const ::windows_sys::core::GUID, notifychange: super::super::Foundation::BOOLEAN, pbuffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcRegisterClient(tciversion: u32, clregctx: super::super::Foundation::HANDLE, clienthandlerlist: *const TCI_CLIENT_FUNC_LIST, pclienthandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcSetFlowA(pflowname: super::super::Foundation::PSTR, pguidparam: *const ::windows_sys::core::GUID, buffersize: u32, buffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcSetFlowW(pflowname: super::super::Foundation::PWSTR, pguidparam: *const ::windows_sys::core::GUID, buffersize: u32, buffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_QoS`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TcSetInterface(ifchandle: super::super::Foundation::HANDLE, pguidparam: *const ::windows_sys::core::GUID, buffersize: u32, buffer: *const ::core::ffi::c_void) -> u32;
}
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ABLE_TO_RECV_RSVP: u32 = 50002u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct ADDRESS_LIST_DESCRIPTOR(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ADM_CTRL_FAILED: u32 = 3u32;
pub struct ADSPEC(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const AD_FLAG_BREAK_BIT: u32 = 1u32;
pub struct AD_GENERAL_PARAMS(i32);
pub struct AD_GUARANTEED(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ALLOWED_TO_SEND_DATA: u32 = 50001u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ANY_DEST_ADDR: u32 = 4294967295u32;
pub struct CBADMITRESULT(i32);
pub struct CBGETRSVPOBJECTS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const CONTROLLED_DELAY_SERV: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const CONTROLLED_LOAD_SERV: u32 = 5u32;
pub struct CONTROL_SERVICE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const CREDENTIAL_SUB_TYPE_ASCII_ID: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const CREDENTIAL_SUB_TYPE_KERBEROS_TKT: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const CREDENTIAL_SUB_TYPE_PGP_CERT: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const CREDENTIAL_SUB_TYPE_UNICODE_ID: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const CREDENTIAL_SUB_TYPE_X509_V3_CERT: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const CURRENT_TCI_VERSION: u32 = 2u32;
pub struct CtrlLoadFlowspec(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const DUP_RESULTS: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const END_TO_END_QOSABILITY: u32 = 50006u32;
pub struct ENUMERATION_BUFFER(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_ADDRESS_TYPE_NOT_SUPPORTED: u32 = 7511u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_DS_MAPPING_EXISTS: u32 = 7518u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_DUPLICATE_FILTER: u32 = 7509u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_FILTER_CONFLICT: u32 = 7510u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_INCOMPATABLE_QOS: u32 = 7513u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_INCOMPATIBLE_TCI_VERSION: u32 = 7501u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_INVALID_ADDRESS_TYPE: u32 = 7508u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_INVALID_DIFFSERV_FLOW: u32 = 7517u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_INVALID_DS_CLASS: u32 = 7520u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_INVALID_FLOW_MODE: u32 = 7516u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_INVALID_PEAK_RATE: u32 = 7504u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_INVALID_QOS_PRIORITY: u32 = 7506u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_INVALID_SD_MODE: u32 = 7505u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_INVALID_SERVICE_TYPE: u32 = 7502u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_INVALID_SHAPE_RATE: u32 = 7519u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_INVALID_TOKEN_RATE: u32 = 7503u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_INVALID_TRAFFIC_CLASS: u32 = 7507u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_NO_MORE_INFO: u32 = 1u32;
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct ERROR_SPEC(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_SPECF_InPlace: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_SPECF_NotGuilty: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_TC_NOT_SUPPORTED: u32 = 7514u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_TC_OBJECT_LENGTH_INVALID: u32 = 7515u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_TC_SUPPORTED_OBJECTS_EXIST: u32 = 7512u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERROR_TOO_MANY_CLIENTS: u32 = 7521u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERR_FORWARD_OK: u32 = 32768u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERR_Usage_globl: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERR_Usage_local: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERR_Usage_serv: u32 = 17u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ERR_global_mask: u32 = 4095u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const EXPIRED_CREDENTIAL: u32 = 4u32;
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct Error_Spec_IPv4(i32);
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct FILTER_SPEC(i32);
pub struct FLOWDESCRIPTOR(i32);
pub struct FLOWSPEC(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const FLOW_DURATION: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const FORCE_IMMEDIATE_REFRESH: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const FSCTL_TCP_BASE: u32 = 18u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const FVEB_UNLOCK_FLAG_AUK_OSFVEINFO: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const FVEB_UNLOCK_FLAG_CACHED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const FVEB_UNLOCK_FLAG_EXTERNAL: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const FVEB_UNLOCK_FLAG_MEDIA: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const FVEB_UNLOCK_FLAG_NBP: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const FVEB_UNLOCK_FLAG_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const FVEB_UNLOCK_FLAG_PASSPHRASE: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const FVEB_UNLOCK_FLAG_PIN: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const FVEB_UNLOCK_FLAG_RECOVERY: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const FVEB_UNLOCK_FLAG_TPM: u32 = 4u32;
pub struct FilterType(i32);
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct Filter_Spec_IPv4(i32);
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct Filter_Spec_IPv4GPI(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GENERAL_INFO: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GQOS_API: u32 = 56400u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GQOS_ERRORCODE_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GQOS_ERRORVALUE_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GQOS_KERNEL_TC: u32 = 56700u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GQOS_KERNEL_TC_SYS: u32 = 56500u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GQOS_NET_ADMISSION: u32 = 56100u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GQOS_NET_POLICY: u32 = 56200u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GQOS_NO_ERRORCODE: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GQOS_NO_ERRORVALUE: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GQOS_RSVP: u32 = 56300u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GQOS_RSVP_SYS: u32 = 56600u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GUARANTEED_SERV: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GUAR_ADSPARM_C: i32 = 131i32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GUAR_ADSPARM_Csum: i32 = 135i32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GUAR_ADSPARM_Ctot: i32 = 133i32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GUAR_ADSPARM_D: i32 = 132i32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GUAR_ADSPARM_Dsum: i32 = 136i32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const GUAR_ADSPARM_Dtot: i32 = 134i32;
pub const GUID_QOS_BESTEFFORT_BANDWIDTH: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3985134224, data2: 16620, data3: 4561, data4: [44, 145, 0, 170, 0, 87, 73, 21] };
pub const GUID_QOS_ENABLE_AVG_STATS: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3137039633,
    data2: 10180,
    data3: 18433,
    data4: [164, 111, 239, 128, 128, 193, 136, 200],
};
pub const GUID_QOS_ENABLE_WINDOW_ADJUSTMENT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2861983525, data2: 54249, data3: 19541, data4: [179, 53, 42, 0, 39, 154, 30, 100] };
pub const GUID_QOS_FLOW_8021P_CONFORMING: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 146923539, data2: 64722, data3: 4562, data4: [190, 30, 0, 160, 201, 158, 230, 59] };
pub const GUID_QOS_FLOW_8021P_NONCONFORMING: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 151142289, data2: 64722, data3: 4562, data4: [190, 30, 0, 160, 201, 158, 230, 59] };
pub const GUID_QOS_FLOW_COUNT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 289929344, data2: 16621, data3: 4561, data4: [44, 145, 0, 170, 0, 87, 73, 21] };
pub const GUID_QOS_FLOW_IP_CONFORMING: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 133798539, data2: 64722, data3: 4562, data4: [190, 30, 0, 160, 201, 158, 230, 59] };
pub const GUID_QOS_FLOW_IP_NONCONFORMING: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 142236039, data2: 64722, data3: 4562, data4: [190, 30, 0, 160, 201, 158, 230, 59] };
pub const GUID_QOS_FLOW_MODE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1552034058, data2: 20826, data3: 4562, data4: [142, 88, 0, 192, 79, 201, 191, 203] };
pub const GUID_QOS_ISSLOW_FLOW: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2884793252, data2: 60935, data3: 4562, data4: [190, 27, 0, 160, 201, 158, 230, 59] };
pub const GUID_QOS_LATENCY: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4232089328, data2: 16620, data3: 4561, data4: [44, 145, 0, 170, 0, 87, 73, 21] };
pub const GUID_QOS_MAX_OUTSTANDING_SENDS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 371194502, data2: 24864, data3: 4561, data4: [44, 145, 0, 170, 0, 87, 73, 21] };
pub const GUID_QOS_NON_BESTEFFORT_LIMIT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 408700128, data2: 16621, data3: 4561, data4: [44, 145, 0, 170, 0, 87, 73, 21] };
pub const GUID_QOS_REMAINING_BANDWIDTH: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301250848, data2: 16620, data3: 4561, data4: [44, 145, 0, 170, 0, 87, 73, 21] };
pub const GUID_QOS_STATISTICS_BUFFER: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3140225408, data2: 59648, data3: 4561, data4: [176, 126, 0, 128, 199, 19, 130, 191] };
pub const GUID_QOS_TIMER_RESOLUTION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3121663112, data2: 61758, data3: 4562, data4: [190, 27, 0, 160, 201, 158, 230, 59] };
pub struct Gads_parms_t(i32);
pub struct GenAdspecParams(i32);
pub struct GenTspec(i32);
pub struct GenTspecParms(i32);
pub struct GuarFlowSpec(i32);
pub struct GuarRspec(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const HIGHLY_DELAY_SENSITIVE: u32 = 4294967294u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const IDENTITY_CHANGED: u32 = 5u32;
pub struct IDPE_ATTR(i32);
pub struct ID_ERROR_OBJECT(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const IF_MIB_STATS_ID: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const INFO_NOT_AVAILABLE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const INSUFFICIENT_PRIVILEGES: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const INTSERV_VERSION0: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const INTSERV_VERS_MASK: u32 = 240u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const INV_LPM_HANDLE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const INV_REQ_HANDLE: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const INV_RESULTS: u32 = 5u32;
pub struct IN_ADDR_IPV4(i32);
pub struct IN_ADDR_IPV6(i32);
pub struct IPX_PATTERN(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const IP_INTFC_INFO_ID: u32 = 259u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const IP_MIB_ADDRTABLE_ENTRY_ID: u32 = 258u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const IP_MIB_STATS_ID: u32 = 1u32;
pub struct IP_PATTERN(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ISPH_FLG_INV: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const ISSH_BREAK_BIT: u32 = 128u32;
pub struct IS_ADSPEC_BODY(i32);
pub struct IS_FLOWSPEC(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const IS_GUAR_RSPEC: i32 = 130i32;
pub struct IntServFlowSpec(i32);
pub struct IntServMainHdr(i32);
pub struct IntServParmHdr(i32);
pub struct IntServServiceHdr(i32);
pub struct IntServTspecBody(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LINE_RATE: u32 = 50003u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LOCAL_QOSABILITY: u32 = 50005u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LOCAL_TRAFFIC_CONTROL: u32 = 50004u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LPM_API_VERSION_1: u32 = 1u32;
pub struct LPM_HANDLE(i32);
pub struct LPM_INIT_INFO(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LPM_OK: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LPM_PE_ALL_TYPES: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LPM_PE_APP_IDENTITY: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LPM_PE_USER_IDENTITY: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LPM_RESULT_DEFER: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LPM_RESULT_READY: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LPM_TIME_OUT: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LPV_DONT_CARE: u32 = 65534u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LPV_DROP_MSG: u32 = 65533u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LPV_MAX_PRIORITY: u32 = 65280u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LPV_MIN_PRIORITY: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LPV_REJECT: u32 = 65535u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const LPV_RESERVED: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const MAX_PHYSADDR_SIZE: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const MAX_STRING_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const MODERATELY_DELAY_SENSITIVE: u32 = 4294967293u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const OSDEVICE_TYPE_BLOCKIO_CDROM: u32 = 65539u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const OSDEVICE_TYPE_BLOCKIO_FILE: u32 = 65541u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const OSDEVICE_TYPE_BLOCKIO_HARDDISK: u32 = 65537u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const OSDEVICE_TYPE_BLOCKIO_PARTITION: u32 = 65540u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const OSDEVICE_TYPE_BLOCKIO_RAMDISK: u32 = 65542u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const OSDEVICE_TYPE_BLOCKIO_REMOVABLEDISK: u32 = 65538u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const OSDEVICE_TYPE_BLOCKIO_VIRTUALHARDDISK: u32 = 65543u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const OSDEVICE_TYPE_COMPOSITE: u32 = 327680u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const OSDEVICE_TYPE_SERIAL: u32 = 131072u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const OSDEVICE_TYPE_UDP: u32 = 196608u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const OSDEVICE_TYPE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const OSDEVICE_TYPE_VMBUS: u32 = 262144u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const Opt_Distinct: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const Opt_Explicit: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const Opt_Share_mask: u32 = 24u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const Opt_Shared: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const Opt_SndSel_mask: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const Opt_Wildcard: u32 = 1u32;
pub struct PALLOCMEM(i32);
pub struct PARAM_BUFFER(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const PCM_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const PE_ATTRIB_TYPE_CREDENTIAL: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const PE_ATTRIB_TYPE_POLICY_LOCATOR: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const PE_TYPE_APPID: u32 = 3u32;
pub struct PFREEMEM(i32);
pub struct POLICY_DATA(i32);
pub struct POLICY_ELEMENT(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_CRAZY_FLOWSPEC: u32 = 57u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_EXPIRED_CREDENTIALS: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_EXPIRED_USER_TOKEN: u32 = 51u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_DEF_FLOW_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_DEF_FLOW_DURATION: u32 = 9u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_DEF_FLOW_RATE: u32 = 17u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_DEF_PEAK_RATE: u32 = 25u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_DEF_SUM_FLOW_RATE: u32 = 33u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_DEF_SUM_PEAK_RATE: u32 = 41u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_GRP_FLOW_COUNT: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_GRP_FLOW_DURATION: u32 = 10u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_GRP_FLOW_RATE: u32 = 18u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_GRP_PEAK_RATE: u32 = 26u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_GRP_SUM_FLOW_RATE: u32 = 34u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_GRP_SUM_PEAK_RATE: u32 = 42u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_UNAUTH_USER_FLOW_COUNT: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_UNAUTH_USER_FLOW_DURATION: u32 = 12u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_UNAUTH_USER_FLOW_RATE: u32 = 20u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_UNAUTH_USER_PEAK_RATE: u32 = 28u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_UNAUTH_USER_SUM_FLOW_RATE: u32 = 36u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_UNAUTH_USER_SUM_PEAK_RATE: u32 = 44u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_USER_FLOW_COUNT: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_USER_FLOW_DURATION: u32 = 11u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_USER_FLOW_RATE: u32 = 19u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_USER_PEAK_RATE: u32 = 27u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_USER_SUM_FLOW_RATE: u32 = 35u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_GLOBAL_USER_SUM_PEAK_RATE: u32 = 43u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_IDENTITY_CHANGED: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_INSUFFICIENT_PRIVILEGES: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_NO_ACCEPTS: u32 = 55u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_NO_MEMORY: u32 = 56u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_NO_MORE_INFO: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_NO_PRIVILEGES: u32 = 50u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_NO_RESOURCES: u32 = 52u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_PRE_EMPTED: u32 = 53u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_DEF_FLOW_COUNT: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_DEF_FLOW_DURATION: u32 = 13u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_DEF_FLOW_RATE: u32 = 21u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_DEF_PEAK_RATE: u32 = 29u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_DEF_SUM_FLOW_RATE: u32 = 37u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_DEF_SUM_PEAK_RATE: u32 = 45u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_GRP_FLOW_COUNT: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_GRP_FLOW_DURATION: u32 = 14u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_GRP_FLOW_RATE: u32 = 22u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_GRP_PEAK_RATE: u32 = 30u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_GRP_SUM_FLOW_RATE: u32 = 38u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_GRP_SUM_PEAK_RATE: u32 = 46u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_UNAUTH_USER_FLOW_COUNT: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_UNAUTH_USER_FLOW_DURATION: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_UNAUTH_USER_FLOW_RATE: u32 = 24u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_UNAUTH_USER_PEAK_RATE: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_UNAUTH_USER_SUM_FLOW_RATE: u32 = 40u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_UNAUTH_USER_SUM_PEAK_RATE: u32 = 48u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_USER_FLOW_COUNT: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_USER_FLOW_DURATION: u32 = 15u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_USER_FLOW_RATE: u32 = 23u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_USER_PEAK_RATE: u32 = 31u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_USER_SUM_FLOW_RATE: u32 = 39u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_SUBNET_USER_SUM_PEAK_RATE: u32 = 47u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_UNKNOWN_USER: u32 = 49u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_UNSUPPORTED_CREDENTIAL_TYPE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_ERRV_USER_CHANGED: u32 = 54u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_LOCATOR_SUB_TYPE_ASCII_DN: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_LOCATOR_SUB_TYPE_ASCII_DN_ENC: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_LOCATOR_SUB_TYPE_UNICODE_DN: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POLICY_LOCATOR_SUB_TYPE_UNICODE_DN_ENC: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const POSITIVE_INFINITY_RATE: u32 = 4294967294u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const PREDICTIVE_SERV: u32 = 3u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QOS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const QOSSPBASE: u32 = 50000u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const QOSSP_ERR_BASE: u32 = 56000u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QOS_DESTADDR(i32);
pub struct QOS_DIFFSERV(i32);
pub struct QOS_DIFFSERV_RULE(i32);
pub struct QOS_DS_CLASS(i32);
pub struct QOS_FLOWRATE_OUTGOING(i32);
pub struct QOS_FLOWRATE_REASON(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct QOS_FLOW_FUNDAMENTALS(i32);
pub struct QOS_FRIENDLY_NAME(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const QOS_GENERAL_ID_BASE: u32 = 2000u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const QOS_MAX_OBJECT_STRING_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const QOS_NON_ADAPTIVE_FLOW: u32 = 2u32;
pub struct QOS_NOTIFY_FLOW(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const QOS_NOT_SPECIFIED: u32 = 4294967295u32;
pub struct QOS_OBJECT_HDR(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const QOS_OUTGOING_DEFAULT_MINIMUM_BANDWIDTH: u32 = 4294967295u32;
pub struct QOS_PACKET_PRIORITY(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const QOS_QUERYFLOW_FRESH: u32 = 1u32;
pub struct QOS_QUERY_FLOW(i32);
pub struct QOS_SD_MODE(i32);
pub struct QOS_SET_FLOW(i32);
pub struct QOS_SHAPING(i32);
pub struct QOS_SHAPING_RATE(i32);
pub struct QOS_TCP_TRAFFIC(i32);
pub struct QOS_TRAFFIC_CLASS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const QOS_TRAFFIC_GENERAL_ID_BASE: u32 = 4000u32;
pub struct QOS_TRAFFIC_TYPE(i32);
pub struct QOS_VERSION(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const QUALITATIVE_SERV: u32 = 6u32;
pub struct QualAppFlowSpec(i32);
pub struct QualTspec(i32);
pub struct QualTspecParms(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RCVD_PATH_TEAR: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RCVD_RESV_TEAR: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RESOURCES_ALLOCATED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RESOURCES_MODIFIED: u32 = 2u32;
pub struct RESV_STYLE(i32);
pub struct RHANDLE(i32);
pub struct RSVP_ADSPEC(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_DEFAULT_STYLE: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_ADMISSION: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_AMBIG_FILTER: u32 = 9u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_API_ERROR: u32 = 20u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_BAD_DSTPORT: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_BAD_SNDPORT: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_BAD_STYLE: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_NO_PATH: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_NO_SENDER: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_POLICY: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_PREEMPTED: u32 = 12u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_RSVP_SYS_ERROR: u32 = 23u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_TC_ERROR: u32 = 21u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_TC_SYS_ERROR: u32 = 22u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_UNKNOWN_CTYPE: u32 = 14u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_UNKNOWN_STYLE: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Err_UNKN_OBJ_CLASS: u32 = 13u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Erv_API: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Erv_Bandwidth: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Erv_Bucket_szie: u32 = 32770u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Erv_Conflict_Serv: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Erv_Crazy_Flowspec: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Erv_Crazy_Tspec: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Erv_DelayBnd: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Erv_Flow_Rate: u32 = 32769u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Erv_MEMORY: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Erv_MTU: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Erv_Min_Policied_size: u32 = 32772u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Erv_No_Serv: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Erv_Nonev: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Erv_Other: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_Erv_Peak_Rate: u32 = 32771u32;
pub struct RSVP_FILTERSPEC(i32);
pub struct RSVP_FILTERSPEC_V4(i32);
pub struct RSVP_FILTERSPEC_V4_GPI(i32);
pub struct RSVP_FILTERSPEC_V6(i32);
pub struct RSVP_FILTERSPEC_V6_FLOW(i32);
pub struct RSVP_FILTERSPEC_V6_GPI(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_FIXED_FILTER_STYLE: u32 = 2u32;
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct RSVP_HOP(i32);
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct RSVP_MSG_OBJS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_OBJECT_ID_BASE: u32 = 1000u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_PATH: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_PATH_ERR: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_PATH_TEAR: u32 = 5u32;
pub struct RSVP_POLICY(i32);
pub struct RSVP_POLICY_INFO(i32);
pub struct RSVP_RESERVE_INFO(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_RESV: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_RESV_ERR: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_RESV_TEAR: u32 = 6u32;
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct RSVP_SCOPE(i32);
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct RSVP_SESSION(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_SHARED_EXPLICIT_STYLE: u32 = 3u32;
pub struct RSVP_STATUS_INFO(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const RSVP_WILDCARD_STYLE: u32 = 1u32;
pub struct RsvpObjHdr(i32);
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct Rsvp_Hop_IPv4(i32);
pub struct SENDER_TSPEC(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SERVICETYPE_BESTEFFORT: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SERVICETYPE_CONTROLLEDLOAD: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SERVICETYPE_GENERAL_INFORMATION: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SERVICETYPE_GUARANTEED: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SERVICETYPE_NETWORK_CONTROL: u32 = 10u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SERVICETYPE_NETWORK_UNAVAILABLE: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SERVICETYPE_NOCHANGE: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SERVICETYPE_NONCONFORMING: u32 = 9u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SERVICETYPE_NOTRAFFIC: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SERVICETYPE_QUALITATIVE: u32 = 13u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SERVICE_BESTEFFORT: u32 = 2147549184u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SERVICE_CONTROLLEDLOAD: u32 = 2147614720u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SERVICE_GUARANTEED: u32 = 2147745792u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SERVICE_NO_QOS_SIGNALING: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SERVICE_NO_TRAFFIC_CONTROL: u32 = 2164260864u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SERVICE_QUALITATIVE: u32 = 2149580800u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SESSFLG_E_Police: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAERROR_FIRMWAREFAILURE: u32 = 196609u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAERROR_INTERNALFAILURE: u32 = 196611u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENTTYPE_AGGREGATION: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENTTYPE_AUTHORITY: u32 = 393216u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENTTYPE_CONTAINER: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENTTYPE_DRTM: u32 = 786432u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENTTYPE_ELAM: u32 = 589824u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENTTYPE_ERROR: u32 = 196608u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENTTYPE_INFORMATION: u32 = 131072u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENTTYPE_KSR: u32 = 720896u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENTTYPE_LOADEDMODULE: u32 = 458752u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENTTYPE_NONMEASURED: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENTTYPE_OSPARAMETER: u32 = 327680u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENTTYPE_PREOSPARAMETER: u32 = 262144u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENTTYPE_TRUSTPOINT: u32 = 524288u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENTTYPE_VBS: u32 = 655360u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_APPLICATION_RETURN: u32 = 131076u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_APPLICATION_SVN: u32 = 131081u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_AUTHENTICODEHASH: u32 = 458756u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_AUTHORITYISSUER: u32 = 458757u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_AUTHORITYPUBKEY: u32 = 393218u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_AUTHORITYPUBLISHER: u32 = 458760u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_AUTHORITYSERIAL: u32 = 458758u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_AUTHORITYSHA1THUMBPRINT: u32 = 458761u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_BITLOCKER_UNLOCK: u32 = 131077u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_BOOTCOUNTER: u32 = 131074u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_BOOTDEBUGGING: u32 = 262145u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_BOOT_REVOCATION_LIST: u32 = 262146u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_CODEINTEGRITY: u32 = 327682u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_COUNTERID: u32 = 131079u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_DATAEXECUTIONPREVENTION: u32 = 327684u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_DRIVER_LOAD_POLICY: u32 = 327694u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_DRTM_AMD_SMM_HASH: u32 = 786435u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_DRTM_AMD_SMM_SIGNER_KEY: u32 = 786436u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_DRTM_SMM_LEVEL: u32 = 786434u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_DRTM_STATE_AUTH: u32 = 786433u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_DUMPS_DISABLED: u32 = 327717u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_DUMP_ENCRYPTION_ENABLED: u32 = 327718u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_DUMP_ENCRYPTION_KEY_DIGEST: u32 = 327719u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_ELAM_CONFIGURATION: u32 = 589826u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_ELAM_KEYNAME: u32 = 589825u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_ELAM_MEASURED: u32 = 589828u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_ELAM_POLICY: u32 = 589827u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_EVENTCOUNTER: u32 = 131078u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_FILEPATH: u32 = 458753u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_FLIGHTSIGNING: u32 = 327713u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_HASHALGORITHMID: u32 = 458755u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_HIBERNATION_DISABLED: u32 = 327716u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_HYPERVISOR_BOOT_DMA_PROTECTION: u32 = 327728u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_HYPERVISOR_DEBUG: u32 = 327693u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_HYPERVISOR_IOMMU_POLICY: u32 = 327692u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_HYPERVISOR_LAUNCH_TYPE: u32 = 327690u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_HYPERVISOR_MMIO_NX_POLICY: u32 = 327696u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_HYPERVISOR_MSR_FILTER_POLICY: u32 = 327697u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_HYPERVISOR_PATH: u32 = 327691u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_IMAGEBASE: u32 = 458759u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_IMAGESIZE: u32 = 458754u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_IMAGEVALIDATED: u32 = 458762u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_INFORMATION: u32 = 131073u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_KSR_SIGNATURE: u32 = 720897u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_LSAISO_CONFIG: u32 = 327720u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_MODULE_SVN: u32 = 458763u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_MORBIT_API_STATUS: u32 = 131083u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_MORBIT_NOT_CANCELABLE: u32 = 131080u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_NOAUTHORITY: u32 = 393217u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_OSDEVICE: u32 = 327688u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_OSKERNELDEBUG: u32 = 327681u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_OS_REVOCATION_LIST: u32 = 327699u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_PAGEFILE_ENCRYPTION_ENABLED: u32 = 327714u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_PHYSICALADDRESSEXTENSION: u32 = 327687u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_SAFEMODE: u32 = 327685u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_SBCP_INFO: u32 = 327721u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_SI_POLICY: u32 = 327695u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_SMT_STATUS: u32 = 327700u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_SVN_CHAIN_STATUS: u32 = 131082u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_SYSTEMROOT: u32 = 327689u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_TESTSIGNING: u32 = 327683u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_TRANSFER_CONTROL: u32 = 131075u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_VBS_DUMP_USES_AMEROOT: u32 = 655369u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_VBS_HVCI_POLICY: u32 = 655367u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_VBS_IOMMU_REQUIRED: u32 = 655363u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_VBS_MANDATORY_ENFORCEMENT: u32 = 655366u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_VBS_MICROSOFT_BOOT_CHAIN_REQUIRED: u32 = 655368u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_VBS_MMIO_NX_REQUIRED: u32 = 655364u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_VBS_MSR_FILTERING_REQUIRED: u32 = 655365u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_VBS_SECUREBOOT_REQUIRED: u32 = 655362u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_VBS_VSM_NOSECRETS_ENFORCED: u32 = 655370u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_VBS_VSM_REQUIRED: u32 = 655361u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_VSM_IDKS_INFO: u32 = 327715u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_VSM_IDK_INFO: u32 = 327712u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_VSM_LAUNCH_TYPE: u32 = 327698u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEVENT_WINPE: u32 = 327686u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_ACTION: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_AMD_SL_EVENT_BASE: u32 = 32768u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_AMD_SL_LOAD: u32 = 32769u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_AMD_SL_LOAD_1: u32 = 32774u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_AMD_SL_PSP_FW_SPLT: u32 = 32770u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_AMD_SL_PUB_KEY: u32 = 32772u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_AMD_SL_SEPARATOR: u32 = 32775u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_AMD_SL_SVN: u32 = 32773u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_AMD_SL_TSME_RB_FUSE: u32 = 32771u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_COMPACT_HASH: u32 = 12u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_CPU_MICROCODE: u32 = 9u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EFI_ACTION: u32 = 2147483655u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EFI_BOOT_SERVICES_APPLICATION: u32 = 2147483651u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EFI_BOOT_SERVICES_DRIVER: u32 = 2147483652u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EFI_EVENT_BASE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EFI_GPT_EVENT: u32 = 2147483654u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EFI_HANDOFF_TABLES: u32 = 2147483657u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EFI_HANDOFF_TABLES2: u32 = 2147483659u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EFI_HCRTM_EVENT: u32 = 2147483664u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EFI_PLATFORM_FIRMWARE_BLOB: u32 = 2147483656u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EFI_PLATFORM_FIRMWARE_BLOB2: u32 = 2147483658u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EFI_RUNTIME_SERVICES_DRIVER: u32 = 2147483653u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EFI_SPDM_FIRMWARE_BLOB: u32 = 2147483873u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EFI_SPDM_FIRMWARE_CONFIG: u32 = 2147483874u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EFI_VARIABLE_AUTHORITY: u32 = 2147483872u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EFI_VARIABLE_BOOT: u32 = 2147483650u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EFI_VARIABLE_DRIVER_CONFIG: u32 = 2147483649u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_EVENT_TAG: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_IPL: u32 = 13u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_IPL_PARTITION_DATA: u32 = 14u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_NONHOST_CODE: u32 = 15u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_NONHOST_CONFIG: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_NONHOST_INFO: u32 = 17u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_NO_ACTION: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_OMIT_BOOT_DEVICE_EVENTS: u32 = 18u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_PLATFORM_CONFIG_FLAGS: u32 = 10u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_POST_CODE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_PREBOOT_CERT: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_SEPARATOR: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_S_CRTM_CONTENTS: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_S_CRTM_VERSION: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TABLE_OF_DEVICES: u32 = 11u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_BIOSAC_REG_DATA: u32 = 1034u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_BOOT_POL_HASH: u32 = 1050u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_BPM_HASH: u32 = 1047u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_BPM_INFO_HASH: u32 = 1049u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_CAP_VALUE: u32 = 1279u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_COLD_BOOT_BIOS_HASH: u32 = 1045u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_COMBINED_HASH: u32 = 1027u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_CPU_SCRTM_STAT: u32 = 1035u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_ELEMENTS_HASH: u32 = 1037u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_EVENT_BASE: u32 = 1024u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_HASH_START: u32 = 1026u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_KM_HASH: u32 = 1046u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_KM_INFO_HASH: u32 = 1048u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_LCP_AUTHORITIES_HASH: u32 = 1043u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_LCP_CONTROL_HASH: u32 = 1036u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_LCP_DETAILS_HASH: u32 = 1042u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_LCP_HASH: u32 = 1041u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_MLE_HASH: u32 = 1028u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_NV_INFO_HASH: u32 = 1044u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_OSSINITDATA_CAP_HASH: u32 = 1039u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_PCR_MAPPING: u32 = 1025u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_RANDOM_VALUE: u32 = 1278u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_SINIT_PUBKEY_HASH: u32 = 1040u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_TXT_STM_HASH: u32 = 1038u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAEV_UNUSED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAHDRSIGNATURE: u32 = 1279476311u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPAKSRHDRSIGNATURE: u32 = 1297240907u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const SIPALOGVERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const STATE_TIMEOUT: u32 = 4u32;
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct Scope_list_ipv4(i32);
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct Session_IPv4(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const TCBASE: u32 = 7500u32;
pub struct TCG_PCClientPCREventStruct(i32);
pub struct TCG_PCClientTaggedEventStruct(i32);
pub struct TCI_ADD_FLOW_COMPLETE_HANDLER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct TCI_CLIENT_FUNC_LIST(i32);
pub struct TCI_DEL_FLOW_COMPLETE_HANDLER(i32);
pub struct TCI_MOD_FLOW_COMPLETE_HANDLER(i32);
pub struct TCI_NOTIFY_HANDLER(i32);
pub struct TC_GEN_FILTER(i32);
pub struct TC_GEN_FLOW(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct TC_IFC_DESCRIPTOR(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const TC_NONCONF_BORROW: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const TC_NONCONF_BORROW_PLUS: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const TC_NONCONF_DISCARD: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const TC_NONCONF_SHAPE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const TC_NOTIFY_FLOW_CLOSE: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const TC_NOTIFY_IFC_CHANGE: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const TC_NOTIFY_IFC_CLOSE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const TC_NOTIFY_IFC_UP: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const TC_NOTIFY_PARAM_CHANGED: u32 = 4u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct TC_SUPPORTED_INFO_BUFFER(i32);
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const UNSUPPORTED_CREDENTIAL_TYPE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_DIGEST_ALG_BITMAP_SHA3_256: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_DIGEST_ALG_BITMAP_SHA3_384: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_DIGEST_ALG_BITMAP_SHA3_512: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_DIGEST_ALG_BITMAP_SHA_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_DIGEST_ALG_BITMAP_SHA_2_256: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_DIGEST_ALG_BITMAP_SHA_2_384: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_DIGEST_ALG_BITMAP_SHA_2_512: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_DIGEST_ALG_BITMAP_SM3_256: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_DIGEST_ALG_ID_SHA3_256: u32 = 39u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_DIGEST_ALG_ID_SHA3_384: u32 = 40u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_DIGEST_ALG_ID_SHA3_512: u32 = 41u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_DIGEST_ALG_ID_SHA_1: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_DIGEST_ALG_ID_SHA_2_256: u32 = 11u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_DIGEST_ALG_ID_SHA_2_384: u32 = 12u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_DIGEST_ALG_ID_SHA_2_512: u32 = 13u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_DIGEST_ALG_ID_SM3_256: u32 = 18u32;
#[doc = "*Required features: `Win32_NetworkManagement_QoS`*"]
pub const WBCL_HASH_LEN_SHA1: u32 = 20u32;
pub struct WBCL_Iterator(i32);
pub struct WBCL_LogHdr(i32);
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct flow_desc(i32);
pub struct int_serv_wkp(i32);
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct lpmiptable(i32);
pub struct policy_decision(i32);
pub struct tag_SIPAEVENT_KSR_SIGNATURE_PAYLOAD(i32);
pub struct tag_SIPAEVENT_REVOCATION_LIST_PAYLOAD(i32);
pub struct tag_SIPAEVENT_SBCP_INFO_PAYLOAD_V1(i32);
pub struct tag_SIPAEVENT_SI_POLICY_PAYLOAD(i32);
pub struct tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD(i32);
pub struct tag_SIPAEVENT_VSM_IDK_RSA_INFO(i32);
