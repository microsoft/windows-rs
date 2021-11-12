#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_AbortPipe(interfacehandle: *const ::core::ffi::c_void, pipeid: u8) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_ControlTransfer(interfacehandle: *const ::core::ffi::c_void, setuppacket: WINUSB_SETUP_PACKET, buffer: *mut u8, bufferlength: u32, lengthtransferred: *mut u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_FlushPipe(interfacehandle: *const ::core::ffi::c_void, pipeid: u8) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_Free(interfacehandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetAdjustedFrameNumber(currentframenumber: *mut u32, timestamp: i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetAssociatedInterface(interfacehandle: *const ::core::ffi::c_void, associatedinterfaceindex: u8, associatedinterfacehandle: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetCurrentAlternateSetting(interfacehandle: *const ::core::ffi::c_void, settingnumber: *mut u8) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetCurrentFrameNumber(interfacehandle: *const ::core::ffi::c_void, currentframenumber: *mut u32, timestamp: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetCurrentFrameNumberAndQpc(interfacehandle: *const ::core::ffi::c_void, frameqpcinfo: *const USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetDescriptor(interfacehandle: *const ::core::ffi::c_void, descriptortype: u8, index: u8, languageid: u16, buffer: *mut u8, bufferlength: u32, lengthtransferred: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_GetOverlappedResult(interfacehandle: *const ::core::ffi::c_void, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetPipePolicy(interfacehandle: *const ::core::ffi::c_void, pipeid: u8, policytype: u32, valuelength: *mut u32, value: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetPowerPolicy(interfacehandle: *const ::core::ffi::c_void, policytype: u32, valuelength: *mut u32, value: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_Initialize(devicehandle: super::super::Foundation::HANDLE, interfacehandle: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    pub fn WinUsb_ParseConfigurationDescriptor(configurationdescriptor: *const USB_CONFIGURATION_DESCRIPTOR, startposition: *const ::core::ffi::c_void, interfacenumber: i32, alternatesetting: i32, interfaceclass: i32, interfacesubclass: i32, interfaceprotocol: i32) -> *mut USB_INTERFACE_DESCRIPTOR;
    pub fn WinUsb_ParseDescriptors(descriptorbuffer: *const ::core::ffi::c_void, totallength: u32, startposition: *const ::core::ffi::c_void, descriptortype: i32) -> *mut USB_COMMON_DESCRIPTOR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_QueryDeviceInformation(interfacehandle: *const ::core::ffi::c_void, informationtype: u32, bufferlength: *mut u32, buffer: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_QueryInterfaceSettings(interfacehandle: *const ::core::ffi::c_void, alternateinterfacenumber: u8, usbaltinterfacedescriptor: *mut USB_INTERFACE_DESCRIPTOR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_QueryPipe(interfacehandle: *const ::core::ffi::c_void, alternateinterfacenumber: u8, pipeindex: u8, pipeinformation: *mut WINUSB_PIPE_INFORMATION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_QueryPipeEx(interfacehandle: *const ::core::ffi::c_void, alternatesettingnumber: u8, pipeindex: u8, pipeinformationex: *mut WINUSB_PIPE_INFORMATION_EX) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_ReadIsochPipe(bufferhandle: *const ::core::ffi::c_void, offset: u32, length: u32, framenumber: *mut u32, numberofpackets: u32, isopacketdescriptors: *mut USBD_ISO_PACKET_DESCRIPTOR, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_ReadIsochPipeAsap(bufferhandle: *const ::core::ffi::c_void, offset: u32, length: u32, continuestream: super::super::Foundation::BOOL, numberofpackets: u32, isopacketdescriptors: *mut USBD_ISO_PACKET_DESCRIPTOR, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_ReadPipe(interfacehandle: *const ::core::ffi::c_void, pipeid: u8, buffer: *mut u8, bufferlength: u32, lengthtransferred: *mut u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_RegisterIsochBuffer(interfacehandle: *const ::core::ffi::c_void, pipeid: u8, buffer: *mut u8, bufferlength: u32, isochbufferhandle: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_ResetPipe(interfacehandle: *const ::core::ffi::c_void, pipeid: u8) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_SetCurrentAlternateSetting(interfacehandle: *const ::core::ffi::c_void, settingnumber: u8) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_SetPipePolicy(interfacehandle: *const ::core::ffi::c_void, pipeid: u8, policytype: u32, valuelength: u32, value: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_SetPowerPolicy(interfacehandle: *const ::core::ffi::c_void, policytype: u32, valuelength: u32, value: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_StartTrackingForTimeSync(interfacehandle: *const ::core::ffi::c_void, starttrackinginfo: *const USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_StopTrackingForTimeSync(interfacehandle: *const ::core::ffi::c_void, stoptrackinginfo: *const USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_UnregisterIsochBuffer(isochbufferhandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_WriteIsochPipe(bufferhandle: *const ::core::ffi::c_void, offset: u32, length: u32, framenumber: *mut u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_WriteIsochPipeAsap(bufferhandle: *const ::core::ffi::c_void, offset: u32, length: u32, continuestream: super::super::Foundation::BOOL, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_WritePipe(interfacehandle: *const ::core::ffi::c_void, pipeid: u8, buffer: *const u8, bufferlength: u32, lengthtransferred: *mut u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
}
pub const ALLOW_PARTIAL_READS: u32 = 5u32;
#[repr(C)]
pub struct ALTERNATE_INTERFACE(i32);
pub const AUTO_CLEAR_STALL: u32 = 2u32;
pub const AUTO_FLUSH: u32 = 6u32;
pub const AUTO_SUSPEND: u32 = 129u32;
pub const BMREQUEST_CLASS: u32 = 1u32;
pub const BMREQUEST_DEVICE_TO_HOST: u32 = 1u32;
pub const BMREQUEST_HOST_TO_DEVICE: u32 = 0u32;
pub const BMREQUEST_STANDARD: u32 = 0u32;
pub const BMREQUEST_TO_DEVICE: u32 = 0u32;
pub const BMREQUEST_TO_ENDPOINT: u32 = 2u32;
pub const BMREQUEST_TO_INTERFACE: u32 = 1u32;
pub const BMREQUEST_TO_OTHER: u32 = 3u32;
pub const BMREQUEST_VENDOR: u32 = 2u32;
#[repr(C)]
pub struct BM_REQUEST_TYPE(i32);
pub const BULKIN_FLAG: u32 = 128u32;
#[repr(C)]
pub struct CHANNEL_INFO(i32);
#[repr(C)]
pub struct DEVICE_DESCRIPTOR(i32);
pub const DEVICE_SPEED: u32 = 1u32;
#[repr(C)]
pub struct DRV_VERSION(i32);
pub const FILE_DEVICE_USB: u32 = 34u32;
pub const FILE_DEVICE_USB_SCAN: u32 = 32768u32;
pub const FullSpeed: u32 = 2u32;
pub const GUID_DEVINTERFACE_USB_BILLBOARD: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1587206895, data2: 63609, data3: 18239, data4: [184, 7, 78, 94, 167, 125, 27, 28] };
pub const GUID_DEVINTERFACE_USB_DEVICE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2782707472, data2: 25904, data3: 4562, data4: [144, 31, 0, 192, 79, 185, 81, 237] };
pub const GUID_DEVINTERFACE_USB_HOST_CONTROLLER: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 985624365, data2: 29124, data3: 17962, data4: [138, 146, 30, 104, 97, 230, 175, 39] };
pub const GUID_DEVINTERFACE_USB_HUB: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4052356744, data2: 49932, data3: 4560, data4: [136, 21, 0, 160, 201, 6, 190, 216] };
pub const GUID_USB_MSOS20_PLATFORM_CAPABILITY_ID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3638386911,
    data2: 17801,
    data3: 19655,
    data4: [156, 210, 101, 157, 158, 100, 138, 159],
};
pub const GUID_USB_PERFORMANCE_TRACING: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3588126630,
    data2: 27369,
    data3: 16988,
    data4: [177, 226, 245, 97, 95, 211, 72, 169],
};
pub const GUID_USB_TRANSFER_TRACING: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1746843818,
    data2: 16445,
    data3: 17708,
    data4: [159, 138, 240, 97, 111, 172, 149, 64],
};
pub const GUID_USB_WMI_DEVICE_PERF_INFO: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1723968060, data2: 18847, data3: 18848, data4: [169, 165, 97, 226, 53, 159, 100, 7] };
pub const GUID_USB_WMI_NODE_INFO: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2618790743,
    data2: 56442,
    data3: 20289,
    data4: [182, 107, 50, 59, 157, 220, 181, 177],
};
pub const GUID_USB_WMI_STD_DATA: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1315060512, data2: 51988, data3: 4561, data4: [179, 49, 0, 160, 201, 89, 187, 210] };
pub const GUID_USB_WMI_STD_NOTIFICATION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1315060512, data2: 51988, data3: 4561, data4: [179, 49, 0, 160, 201, 89, 187, 210] };
pub const GUID_USB_WMI_SURPRISE_REMOVAL_NOTIFICATION: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2612787249,
    data2: 41714,
    data3: 17332,
    data4: [150, 209, 134, 148, 75, 89, 20, 179],
};
pub const GUID_USB_WMI_TRACING: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 979470363, data2: 46310, data3: 19449, data4: [174, 15, 60, 216, 243, 148, 229, 47] };
pub const HCD_DIAGNOSTIC_MODE_OFF: u32 = 257u32;
pub const HCD_DIAGNOSTIC_MODE_ON: u32 = 256u32;
pub const HCD_DISABLE_PORT: u32 = 268u32;
pub const HCD_ENABLE_PORT: u32 = 269u32;
pub const HCD_GET_DRIVERKEY_NAME: u32 = 265u32;
pub const HCD_GET_ROOT_HUB_NAME: u32 = 258u32;
pub const HCD_GET_STATS_1: u32 = 255u32;
pub const HCD_GET_STATS_2: u32 = 266u32;
pub const HCD_TRACE_READ_REQUEST: u32 = 275u32;
pub const HCD_USER_REQUEST: u32 = 270u32;
pub const HighSpeed: u32 = 3u32;
pub const IGNORE_SHORT_PACKETS: u32 = 4u32;
pub const IOCTL_ABORT_PIPE: u32 = 2147491844u32;
pub const IOCTL_CANCEL_IO: u32 = 2147491844u32;
pub const IOCTL_GENERICUSBFN_ACTIVATE_USB_BUS: u32 = 2277420u32;
pub const IOCTL_GENERICUSBFN_BUS_EVENT_NOTIFICATION: u32 = 2277430u32;
pub const IOCTL_GENERICUSBFN_CONTROL_STATUS_HANDSHAKE_IN: u32 = 2277400u32;
pub const IOCTL_GENERICUSBFN_CONTROL_STATUS_HANDSHAKE_OUT: u32 = 2277404u32;
pub const IOCTL_GENERICUSBFN_DEACTIVATE_USB_BUS: u32 = 2277424u32;
pub const IOCTL_GENERICUSBFN_GET_CLASS_INFO: u32 = 2277410u32;
pub const IOCTL_GENERICUSBFN_GET_CLASS_INFO_EX: u32 = 2277434u32;
pub const IOCTL_GENERICUSBFN_GET_INTERFACE_DESCRIPTOR_SET: u32 = 2277438u32;
pub const IOCTL_GENERICUSBFN_GET_PIPE_STATE: u32 = 2277414u32;
pub const IOCTL_GENERICUSBFN_REGISTER_USB_STRING: u32 = 2277441u32;
pub const IOCTL_GENERICUSBFN_SET_PIPE_STATE: u32 = 2277417u32;
pub const IOCTL_GENERICUSBFN_TRANSFER_IN: u32 = 2277389u32;
pub const IOCTL_GENERICUSBFN_TRANSFER_IN_APPEND_ZERO_PKT: u32 = 2277393u32;
pub const IOCTL_GENERICUSBFN_TRANSFER_OUT: u32 = 2277398u32;
pub const IOCTL_GET_CHANNEL_ALIGN_RQST: u32 = 2147491860u32;
pub const IOCTL_GET_DEVICE_DESCRIPTOR: u32 = 2147491864u32;
pub const IOCTL_GET_HCD_DRIVERKEY_NAME: u32 = 2229284u32;
pub const IOCTL_GET_PIPE_CONFIGURATION: u32 = 2147491880u32;
pub const IOCTL_GET_USB_DESCRIPTOR: u32 = 2147491872u32;
pub const IOCTL_GET_VERSION: u32 = 2147491840u32;
pub const IOCTL_INDEX: u32 = 2048u32;
pub const IOCTL_INTERNAL_USB_CYCLE_PORT: u32 = 2228255u32;
pub const IOCTL_INTERNAL_USB_ENABLE_PORT: u32 = 2228247u32;
pub const IOCTL_INTERNAL_USB_FAIL_GET_STATUS_FROM_DEVICE: u32 = 2229347u32;
pub const IOCTL_INTERNAL_USB_GET_BUSGUID_INFO: u32 = 2229288u32;
pub const IOCTL_INTERNAL_USB_GET_BUS_INFO: u32 = 2229280u32;
pub const IOCTL_INTERNAL_USB_GET_CONTROLLER_NAME: u32 = 2229284u32;
pub const IOCTL_INTERNAL_USB_GET_DEVICE_CONFIG_INFO: u32 = 2229327u32;
pub const IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE: u32 = 2229299u32;
pub const IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE_EX: u32 = 2229303u32;
pub const IOCTL_INTERNAL_USB_GET_HUB_COUNT: u32 = 2228251u32;
pub const IOCTL_INTERNAL_USB_GET_HUB_NAME: u32 = 2228256u32;
pub const IOCTL_INTERNAL_USB_GET_PARENT_HUB_INFO: u32 = 2229292u32;
pub const IOCTL_INTERNAL_USB_GET_PORT_STATUS: u32 = 2228243u32;
pub const IOCTL_INTERNAL_USB_GET_ROOTHUB_PDO: u32 = 2228239u32;
pub const IOCTL_INTERNAL_USB_GET_TOPOLOGY_ADDRESS: u32 = 2229311u32;
pub const IOCTL_INTERNAL_USB_GET_TT_DEVICE_HANDLE: u32 = 2229307u32;
pub const IOCTL_INTERNAL_USB_NOTIFY_IDLE_READY: u32 = 2229315u32;
pub const IOCTL_INTERNAL_USB_RECORD_FAILURE: u32 = 2228267u32;
pub const IOCTL_INTERNAL_USB_REGISTER_COMPOSITE_DEVICE: u32 = 4784131u32;
pub const IOCTL_INTERNAL_USB_REQUEST_REMOTE_WAKE_NOTIFICATION: u32 = 4784139u32;
pub const IOCTL_INTERNAL_USB_REQ_GLOBAL_RESUME: u32 = 2229323u32;
pub const IOCTL_INTERNAL_USB_REQ_GLOBAL_SUSPEND: u32 = 2229319u32;
pub const IOCTL_INTERNAL_USB_RESET_PORT: u32 = 2228231u32;
pub const IOCTL_INTERNAL_USB_SUBMIT_IDLE_NOTIFICATION: u32 = 2228263u32;
pub const IOCTL_INTERNAL_USB_SUBMIT_URB: u32 = 2228227u32;
pub const IOCTL_INTERNAL_USB_UNREGISTER_COMPOSITE_DEVICE: u32 = 4784135u32;
pub const IOCTL_READ_REGISTERS: u32 = 2147491852u32;
pub const IOCTL_RESET_PIPE: u32 = 2147491868u32;
pub const IOCTL_SEND_USB_REQUEST: u32 = 2147491876u32;
pub const IOCTL_SET_TIMEOUT: u32 = 2147491884u32;
pub const IOCTL_USB_DIAGNOSTIC_MODE_OFF: u32 = 2229252u32;
pub const IOCTL_USB_DIAGNOSTIC_MODE_ON: u32 = 2229248u32;
pub const IOCTL_USB_DIAG_IGNORE_HUBS_OFF: u32 = 2229276u32;
pub const IOCTL_USB_DIAG_IGNORE_HUBS_ON: u32 = 2229272u32;
pub const IOCTL_USB_GET_DESCRIPTOR_FROM_NODE_CONNECTION: u32 = 2229264u32;
pub const IOCTL_USB_GET_DEVICE_CHARACTERISTICS: u32 = 2229376u32;
pub const IOCTL_USB_GET_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC: u32 = 2229368u32;
pub const IOCTL_USB_GET_HUB_CAPABILITIES: u32 = 2229308u32;
pub const IOCTL_USB_GET_HUB_CAPABILITIES_EX: u32 = 2229328u32;
pub const IOCTL_USB_GET_HUB_INFORMATION_EX: u32 = 2229332u32;
pub const IOCTL_USB_GET_NODE_CONNECTION_ATTRIBUTES: u32 = 2229312u32;
pub const IOCTL_USB_GET_NODE_CONNECTION_DRIVERKEY_NAME: u32 = 2229280u32;
pub const IOCTL_USB_GET_NODE_CONNECTION_INFORMATION: u32 = 2229260u32;
pub const IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX: u32 = 2229320u32;
pub const IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2: u32 = 2229340u32;
pub const IOCTL_USB_GET_NODE_CONNECTION_NAME: u32 = 2229268u32;
pub const IOCTL_USB_GET_NODE_INFORMATION: u32 = 2229256u32;
pub const IOCTL_USB_GET_PORT_CONNECTOR_PROPERTIES: u32 = 2229336u32;
pub const IOCTL_USB_GET_ROOT_HUB_NAME: u32 = 2229256u32;
pub const IOCTL_USB_GET_TRANSPORT_CHARACTERISTICS: u32 = 2229348u32;
pub const IOCTL_USB_HCD_DISABLE_PORT: u32 = 2229296u32;
pub const IOCTL_USB_HCD_ENABLE_PORT: u32 = 2229300u32;
pub const IOCTL_USB_HCD_GET_STATS_1: u32 = 2229244u32;
pub const IOCTL_USB_HCD_GET_STATS_2: u32 = 2229288u32;
pub const IOCTL_USB_HUB_CYCLE_PORT: u32 = 2229316u32;
pub const IOCTL_USB_NOTIFY_ON_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 2229356u32;
pub const IOCTL_USB_REGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 2229352u32;
pub const IOCTL_USB_RESET_HUB: u32 = 2229324u32;
pub const IOCTL_USB_START_TRACKING_FOR_TIME_SYNC: u32 = 2229364u32;
pub const IOCTL_USB_STOP_TRACKING_FOR_TIME_SYNC: u32 = 2229372u32;
pub const IOCTL_USB_UNREGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 2229360u32;
pub const IOCTL_WAIT_ON_DEVICE_EVENT: u32 = 2147491848u32;
pub const IOCTL_WRITE_REGISTERS: u32 = 2147491856u32;
#[repr(C)]
pub struct IO_BLOCK(i32);
#[repr(C)]
pub struct IO_BLOCK_EX(i32);
pub const LowSpeed: u32 = 1u32;
pub const MAXIMUM_TRANSFER_SIZE: u32 = 8u32;
pub const MAXIMUM_USB_STRING_LENGTH: u32 = 255u32;
pub const MAX_ALTERNATE_NAME_LENGTH: u32 = 40u32;
pub const MAX_ASSOCIATION_NAME_LENGTH: u32 = 40u32;
pub const MAX_CONFIGURATION_NAME_LENGTH: u32 = 40u32;
pub const MAX_INTERFACE_NAME_LENGTH: u32 = 40u32;
pub const MAX_NUM_PIPES: u32 = 8u32;
pub const MAX_NUM_USBFN_ENDPOINTS: u32 = 15u32;
pub const MAX_SUPPORTED_CONFIGURATIONS: u32 = 12u32;
pub const MAX_USB_STRING_LENGTH: u32 = 255u32;
pub const MS_GENRE_DESCRIPTOR_INDEX: u32 = 1u32;
pub const MS_OS_FLAGS_CONTAINERID: u32 = 2u32;
pub const MS_POWER_DESCRIPTOR_INDEX: u32 = 2u32;
#[repr(C)]
pub struct OS_STRING(i32);
pub const OS_STRING_DESCRIPTOR_INDEX: u32 = 238u32;
#[repr(C)]
pub struct PACKET_PARAMETERS(i32);
pub const PIPE_TRANSFER_TIMEOUT: u32 = 3u32;
#[repr(transparent)]
pub struct PIPE_TYPE(pub i32);
pub const EVENT_PIPE: PIPE_TYPE = PIPE_TYPE(0i32);
pub const READ_DATA_PIPE: PIPE_TYPE = PIPE_TYPE(1i32);
pub const WRITE_DATA_PIPE: PIPE_TYPE = PIPE_TYPE(2i32);
pub const ALL_PIPE: PIPE_TYPE = PIPE_TYPE(3i32);
pub const PORT_LINK_STATE_COMPLIANCE_MODE: u32 = 10u32;
pub const PORT_LINK_STATE_DISABLED: u32 = 4u32;
pub const PORT_LINK_STATE_HOT_RESET: u32 = 9u32;
pub const PORT_LINK_STATE_INACTIVE: u32 = 6u32;
pub const PORT_LINK_STATE_LOOPBACK: u32 = 11u32;
pub const PORT_LINK_STATE_POLLING: u32 = 7u32;
pub const PORT_LINK_STATE_RECOVERY: u32 = 8u32;
pub const PORT_LINK_STATE_RX_DETECT: u32 = 5u32;
pub const PORT_LINK_STATE_TEST_MODE: u32 = 11u32;
pub const PORT_LINK_STATE_U0: u32 = 0u32;
pub const PORT_LINK_STATE_U1: u32 = 1u32;
pub const PORT_LINK_STATE_U2: u32 = 2u32;
pub const PORT_LINK_STATE_U3: u32 = 3u32;
pub const RAW_IO: u32 = 7u32;
#[repr(transparent)]
pub struct RAW_PIPE_TYPE(pub i32);
pub const USBSCAN_PIPE_CONTROL: RAW_PIPE_TYPE = RAW_PIPE_TYPE(0i32);
pub const USBSCAN_PIPE_ISOCHRONOUS: RAW_PIPE_TYPE = RAW_PIPE_TYPE(1i32);
pub const USBSCAN_PIPE_BULK: RAW_PIPE_TYPE = RAW_PIPE_TYPE(2i32);
pub const USBSCAN_PIPE_INTERRUPT: RAW_PIPE_TYPE = RAW_PIPE_TYPE(3i32);
#[repr(C)]
pub struct RAW_RESET_PORT_PARAMETERS(i32);
#[repr(C)]
pub struct RAW_ROOTPORT_FEATURE(i32);
#[repr(C)]
pub struct RAW_ROOTPORT_PARAMETERS(i32);
pub const RESET_PIPE_ON_RESUME: u32 = 9u32;
pub const SHORT_PACKET_TERMINATE: u32 = 1u32;
pub const SUSPEND_DELAY: u32 = 131u32;
#[repr(C)]
pub struct URB(i32);
pub const URB_FUNCTION_ABORT_PIPE: u32 = 2u32;
pub const URB_FUNCTION_BULK_OR_INTERRUPT_TRANSFER: u32 = 9u32;
pub const URB_FUNCTION_BULK_OR_INTERRUPT_TRANSFER_USING_CHAINED_MDL: u32 = 55u32;
pub const URB_FUNCTION_CLASS_DEVICE: u32 = 26u32;
pub const URB_FUNCTION_CLASS_ENDPOINT: u32 = 28u32;
pub const URB_FUNCTION_CLASS_INTERFACE: u32 = 27u32;
pub const URB_FUNCTION_CLASS_OTHER: u32 = 31u32;
pub const URB_FUNCTION_CLEAR_FEATURE_TO_DEVICE: u32 = 16u32;
pub const URB_FUNCTION_CLEAR_FEATURE_TO_ENDPOINT: u32 = 18u32;
pub const URB_FUNCTION_CLEAR_FEATURE_TO_INTERFACE: u32 = 17u32;
pub const URB_FUNCTION_CLEAR_FEATURE_TO_OTHER: u32 = 34u32;
pub const URB_FUNCTION_CLOSE_STATIC_STREAMS: u32 = 54u32;
pub const URB_FUNCTION_CONTROL_TRANSFER: u32 = 8u32;
pub const URB_FUNCTION_CONTROL_TRANSFER_EX: u32 = 50u32;
pub const URB_FUNCTION_GET_CONFIGURATION: u32 = 38u32;
pub const URB_FUNCTION_GET_CURRENT_FRAME_NUMBER: u32 = 7u32;
pub const URB_FUNCTION_GET_DESCRIPTOR_FROM_DEVICE: u32 = 11u32;
pub const URB_FUNCTION_GET_DESCRIPTOR_FROM_ENDPOINT: u32 = 36u32;
pub const URB_FUNCTION_GET_DESCRIPTOR_FROM_INTERFACE: u32 = 40u32;
pub const URB_FUNCTION_GET_FRAME_LENGTH: u32 = 5u32;
pub const URB_FUNCTION_GET_INTERFACE: u32 = 39u32;
pub const URB_FUNCTION_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS: u32 = 61u32;
pub const URB_FUNCTION_GET_MS_FEATURE_DESCRIPTOR: u32 = 42u32;
pub const URB_FUNCTION_GET_STATUS_FROM_DEVICE: u32 = 19u32;
pub const URB_FUNCTION_GET_STATUS_FROM_ENDPOINT: u32 = 21u32;
pub const URB_FUNCTION_GET_STATUS_FROM_INTERFACE: u32 = 20u32;
pub const URB_FUNCTION_GET_STATUS_FROM_OTHER: u32 = 33u32;
pub const URB_FUNCTION_ISOCH_TRANSFER: u32 = 10u32;
pub const URB_FUNCTION_ISOCH_TRANSFER_USING_CHAINED_MDL: u32 = 56u32;
pub const URB_FUNCTION_OPEN_STATIC_STREAMS: u32 = 53u32;
pub const URB_FUNCTION_RELEASE_FRAME_LENGTH_CONTROL: u32 = 4u32;
pub const URB_FUNCTION_RESERVED_0X0016: u32 = 22u32;
pub const URB_FUNCTION_RESERVE_0X001D: u32 = 29u32;
pub const URB_FUNCTION_RESERVE_0X002B: u32 = 43u32;
pub const URB_FUNCTION_RESERVE_0X002C: u32 = 44u32;
pub const URB_FUNCTION_RESERVE_0X002D: u32 = 45u32;
pub const URB_FUNCTION_RESERVE_0X002E: u32 = 46u32;
pub const URB_FUNCTION_RESERVE_0X002F: u32 = 47u32;
pub const URB_FUNCTION_RESERVE_0X0033: u32 = 51u32;
pub const URB_FUNCTION_RESERVE_0X0034: u32 = 52u32;
pub const URB_FUNCTION_RESET_PIPE: u32 = 30u32;
pub const URB_FUNCTION_SELECT_CONFIGURATION: u32 = 0u32;
pub const URB_FUNCTION_SELECT_INTERFACE: u32 = 1u32;
pub const URB_FUNCTION_SET_DESCRIPTOR_TO_DEVICE: u32 = 12u32;
pub const URB_FUNCTION_SET_DESCRIPTOR_TO_ENDPOINT: u32 = 37u32;
pub const URB_FUNCTION_SET_DESCRIPTOR_TO_INTERFACE: u32 = 41u32;
pub const URB_FUNCTION_SET_FEATURE_TO_DEVICE: u32 = 13u32;
pub const URB_FUNCTION_SET_FEATURE_TO_ENDPOINT: u32 = 15u32;
pub const URB_FUNCTION_SET_FEATURE_TO_INTERFACE: u32 = 14u32;
pub const URB_FUNCTION_SET_FEATURE_TO_OTHER: u32 = 35u32;
pub const URB_FUNCTION_SET_FRAME_LENGTH: u32 = 6u32;
pub const URB_FUNCTION_SYNC_CLEAR_STALL: u32 = 49u32;
pub const URB_FUNCTION_SYNC_RESET_PIPE: u32 = 48u32;
pub const URB_FUNCTION_SYNC_RESET_PIPE_AND_CLEAR_STALL: u32 = 30u32;
pub const URB_FUNCTION_TAKE_FRAME_LENGTH_CONTROL: u32 = 3u32;
pub const URB_FUNCTION_VENDOR_DEVICE: u32 = 23u32;
pub const URB_FUNCTION_VENDOR_ENDPOINT: u32 = 25u32;
pub const URB_FUNCTION_VENDOR_INTERFACE: u32 = 24u32;
pub const URB_FUNCTION_VENDOR_OTHER: u32 = 32u32;
pub const URB_OPEN_STATIC_STREAMS_VERSION_100: u32 = 256u32;
pub const USBDI_VERSION: u32 = 1536u32;
pub const USBD_DEFAULT_MAXIMUM_TRANSFER_SIZE: u32 = 4294967295u32;
pub const USBD_DEFAULT_PIPE_TRANSFER: u32 = 8u32;
#[repr(C)]
pub struct USBD_DEVICE_INFORMATION(i32);
#[repr(C)]
pub struct USBD_ENDPOINT_OFFLOAD_INFORMATION(i32);
#[repr(transparent)]
pub struct USBD_ENDPOINT_OFFLOAD_MODE(pub i32);
pub const UsbdEndpointOffloadModeNotSupported: USBD_ENDPOINT_OFFLOAD_MODE = USBD_ENDPOINT_OFFLOAD_MODE(0i32);
pub const UsbdEndpointOffloadSoftwareAssisted: USBD_ENDPOINT_OFFLOAD_MODE = USBD_ENDPOINT_OFFLOAD_MODE(1i32);
pub const UsbdEndpointOffloadHardwareAssisted: USBD_ENDPOINT_OFFLOAD_MODE = USBD_ENDPOINT_OFFLOAD_MODE(2i32);
#[repr(C)]
pub struct USBD_INTERFACE_INFORMATION(i32);
#[repr(C)]
pub struct USBD_ISO_PACKET_DESCRIPTOR(i32);
pub const USBD_ISO_START_FRAME_RANGE: u32 = 1024u32;
pub const USBD_PF_CHANGE_MAX_PACKET: u32 = 1u32;
pub const USBD_PF_ENABLE_RT_THREAD_ACCESS: u32 = 4u32;
pub const USBD_PF_HANDLES_SSP_HIGH_BANDWIDTH_ISOCH: u32 = 256u32;
pub const USBD_PF_INTERACTIVE_PRIORITY: u32 = 48u32;
pub const USBD_PF_MAP_ADD_TRANSFERS: u32 = 8u32;
pub const USBD_PF_PRIORITY_MASK: u32 = 240u32;
pub const USBD_PF_SHORT_PACKET_OPT: u32 = 2u32;
pub const USBD_PF_SSP_HIGH_BANDWIDTH_ISOCH: u32 = 65536u32;
pub const USBD_PF_VIDEO_PRIORITY: u32 = 16u32;
pub const USBD_PF_VOICE_PRIORITY: u32 = 32u32;
#[repr(C)]
pub struct USBD_PIPE_INFORMATION(i32);
#[repr(transparent)]
pub struct USBD_PIPE_TYPE(pub i32);
pub const UsbdPipeTypeControl: USBD_PIPE_TYPE = USBD_PIPE_TYPE(0i32);
pub const UsbdPipeTypeIsochronous: USBD_PIPE_TYPE = USBD_PIPE_TYPE(1i32);
pub const UsbdPipeTypeBulk: USBD_PIPE_TYPE = USBD_PIPE_TYPE(2i32);
pub const UsbdPipeTypeInterrupt: USBD_PIPE_TYPE = USBD_PIPE_TYPE(3i32);
pub const USBD_PORT_CONNECTED: u32 = 2u32;
pub const USBD_PORT_ENABLED: u32 = 1u32;
pub const USBD_SHORT_TRANSFER_OK: u32 = 2u32;
pub const USBD_START_ISO_TRANSFER_ASAP: u32 = 4u32;
#[repr(C)]
pub struct USBD_STREAM_INFORMATION(i32);
pub const USBD_TRANSFER_DIRECTION: u32 = 1u32;
pub const USBD_TRANSFER_DIRECTION_IN: u32 = 1u32;
pub const USBD_TRANSFER_DIRECTION_OUT: u32 = 0u32;
#[repr(C)]
pub struct USBD_VERSION_INFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USBFN_BUS_CONFIGURATION_INFO(i32);
#[repr(transparent)]
pub struct USBFN_BUS_SPEED(pub i32);
pub const UsbfnBusSpeedLow: USBFN_BUS_SPEED = USBFN_BUS_SPEED(0i32);
pub const UsbfnBusSpeedFull: USBFN_BUS_SPEED = USBFN_BUS_SPEED(1i32);
pub const UsbfnBusSpeedHigh: USBFN_BUS_SPEED = USBFN_BUS_SPEED(2i32);
pub const UsbfnBusSpeedSuper: USBFN_BUS_SPEED = USBFN_BUS_SPEED(3i32);
pub const UsbfnBusSpeedMaximum: USBFN_BUS_SPEED = USBFN_BUS_SPEED(4i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USBFN_CLASS_INFORMATION_PACKET(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USBFN_CLASS_INFORMATION_PACKET_EX(i32);
#[repr(C)]
pub struct USBFN_CLASS_INTERFACE(i32);
#[repr(C)]
pub struct USBFN_CLASS_INTERFACE_EX(i32);
#[repr(transparent)]
pub struct USBFN_DEVICE_STATE(pub i32);
pub const UsbfnDeviceStateMinimum: USBFN_DEVICE_STATE = USBFN_DEVICE_STATE(0i32);
pub const UsbfnDeviceStateAttached: USBFN_DEVICE_STATE = USBFN_DEVICE_STATE(1i32);
pub const UsbfnDeviceStateDefault: USBFN_DEVICE_STATE = USBFN_DEVICE_STATE(2i32);
pub const UsbfnDeviceStateDetached: USBFN_DEVICE_STATE = USBFN_DEVICE_STATE(3i32);
pub const UsbfnDeviceStateAddressed: USBFN_DEVICE_STATE = USBFN_DEVICE_STATE(4i32);
pub const UsbfnDeviceStateConfigured: USBFN_DEVICE_STATE = USBFN_DEVICE_STATE(5i32);
pub const UsbfnDeviceStateSuspended: USBFN_DEVICE_STATE = USBFN_DEVICE_STATE(6i32);
pub const UsbfnDeviceStateStateMaximum: USBFN_DEVICE_STATE = USBFN_DEVICE_STATE(7i32);
#[repr(transparent)]
pub struct USBFN_DIRECTION(pub i32);
pub const UsbfnDirectionMinimum: USBFN_DIRECTION = USBFN_DIRECTION(0i32);
pub const UsbfnDirectionIn: USBFN_DIRECTION = USBFN_DIRECTION(1i32);
pub const UsbfnDirectionOut: USBFN_DIRECTION = USBFN_DIRECTION(2i32);
pub const UsbfnDirectionTx: USBFN_DIRECTION = USBFN_DIRECTION(1i32);
pub const UsbfnDirectionRx: USBFN_DIRECTION = USBFN_DIRECTION(2i32);
pub const UsbfnDirectionMaximum: USBFN_DIRECTION = USBFN_DIRECTION(3i32);
#[repr(transparent)]
pub struct USBFN_EVENT(pub i32);
pub const UsbfnEventMinimum: USBFN_EVENT = USBFN_EVENT(0i32);
pub const UsbfnEventAttach: USBFN_EVENT = USBFN_EVENT(1i32);
pub const UsbfnEventReset: USBFN_EVENT = USBFN_EVENT(2i32);
pub const UsbfnEventDetach: USBFN_EVENT = USBFN_EVENT(3i32);
pub const UsbfnEventSuspend: USBFN_EVENT = USBFN_EVENT(4i32);
pub const UsbfnEventResume: USBFN_EVENT = USBFN_EVENT(5i32);
pub const UsbfnEventSetupPacket: USBFN_EVENT = USBFN_EVENT(6i32);
pub const UsbfnEventConfigured: USBFN_EVENT = USBFN_EVENT(7i32);
pub const UsbfnEventUnConfigured: USBFN_EVENT = USBFN_EVENT(8i32);
pub const UsbfnEventPortType: USBFN_EVENT = USBFN_EVENT(9i32);
pub const UsbfnEventBusTearDown: USBFN_EVENT = USBFN_EVENT(10i32);
pub const UsbfnEventSetInterface: USBFN_EVENT = USBFN_EVENT(11i32);
pub const UsbfnEventMaximum: USBFN_EVENT = USBFN_EVENT(12i32);
#[repr(C)]
pub struct USBFN_INTERFACE_INFO(i32);
pub const USBFN_INTERRUPT_ENDPOINT_SIZE_NOT_UPDATEABLE_MASK: u32 = 128u32;
#[repr(C)]
pub struct USBFN_NOTIFICATION(i32);
#[repr(C)]
pub struct USBFN_PIPE_INFORMATION(i32);
#[repr(transparent)]
pub struct USBFN_PORT_TYPE(pub i32);
pub const UsbfnUnknownPort: USBFN_PORT_TYPE = USBFN_PORT_TYPE(0i32);
pub const UsbfnStandardDownstreamPort: USBFN_PORT_TYPE = USBFN_PORT_TYPE(1i32);
pub const UsbfnChargingDownstreamPort: USBFN_PORT_TYPE = USBFN_PORT_TYPE(2i32);
pub const UsbfnDedicatedChargingPort: USBFN_PORT_TYPE = USBFN_PORT_TYPE(3i32);
pub const UsbfnInvalidDedicatedChargingPort: USBFN_PORT_TYPE = USBFN_PORT_TYPE(4i32);
pub const UsbfnProprietaryDedicatedChargingPort: USBFN_PORT_TYPE = USBFN_PORT_TYPE(5i32);
pub const UsbfnPortTypeMaximum: USBFN_PORT_TYPE = USBFN_PORT_TYPE(6i32);
#[repr(C)]
pub struct USBFN_USB_STRING(i32);
#[repr(C)]
pub struct USBSCAN_GET_DESCRIPTOR(i32);
#[repr(C)]
pub struct USBSCAN_PIPE_CONFIGURATION(i32);
#[repr(C)]
pub struct USBSCAN_PIPE_INFORMATION(i32);
#[repr(C)]
pub struct USBSCAN_TIMEOUT(i32);
#[repr(C)]
pub struct USBUSER_BANDWIDTH_INFO_REQUEST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USBUSER_BUS_STATISTICS_0_REQUEST(i32);
pub const USBUSER_CLEAR_ROOTPORT_FEATURE: u32 = 536870918u32;
#[repr(C)]
pub struct USBUSER_CLOSE_RAW_DEVICE(i32);
#[repr(C)]
pub struct USBUSER_CONTROLLER_INFO_0(i32);
#[repr(C)]
pub struct USBUSER_CONTROLLER_UNICODE_NAME(i32);
pub const USBUSER_GET_BANDWIDTH_INFORMATION: u32 = 5u32;
pub const USBUSER_GET_BUS_STATISTICS_0: u32 = 6u32;
pub const USBUSER_GET_CONTROLLER_DRIVER_KEY: u32 = 2u32;
pub const USBUSER_GET_CONTROLLER_INFO_0: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USBUSER_GET_DRIVER_VERSION(i32);
pub const USBUSER_GET_POWER_STATE_MAP: u32 = 4u32;
pub const USBUSER_GET_ROOTHUB_SYMBOLIC_NAME: u32 = 7u32;
pub const USBUSER_GET_ROOTPORT_STATUS: u32 = 536870919u32;
#[repr(C)]
pub struct USBUSER_GET_USB2HW_VERSION(i32);
pub const USBUSER_GET_USB2_HW_VERSION: u32 = 9u32;
pub const USBUSER_GET_USB_DRIVER_VERSION: u32 = 8u32;
pub const USBUSER_INVALID_REQUEST: u32 = 4294967280u32;
#[repr(C)]
pub struct USBUSER_OPEN_RAW_DEVICE(i32);
pub const USBUSER_OP_CLOSE_RAW_DEVICE: u32 = 536870915u32;
pub const USBUSER_OP_MASK_DEVONLY_API: u32 = 268435456u32;
pub const USBUSER_OP_MASK_HCTEST_API: u32 = 536870912u32;
pub const USBUSER_OP_OPEN_RAW_DEVICE: u32 = 536870914u32;
pub const USBUSER_OP_RAW_RESET_PORT: u32 = 536870913u32;
pub const USBUSER_OP_SEND_ONE_PACKET: u32 = 268435457u32;
pub const USBUSER_OP_SEND_RAW_COMMAND: u32 = 536870916u32;
pub const USBUSER_PASS_THRU: u32 = 3u32;
#[repr(C)]
pub struct USBUSER_PASS_THRU_REQUEST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USBUSER_POWER_INFO_REQUEST(i32);
#[repr(C)]
pub struct USBUSER_RAW_RESET_ROOT_PORT(i32);
#[repr(C)]
pub struct USBUSER_REFRESH_HCT_REG(i32);
#[repr(C)]
pub struct USBUSER_REQUEST_HEADER(i32);
#[repr(C)]
pub struct USBUSER_ROOTPORT_FEATURE_REQUEST(i32);
#[repr(C)]
pub struct USBUSER_ROOTPORT_PARAMETERS(i32);
#[repr(C)]
pub struct USBUSER_SEND_ONE_PACKET(i32);
#[repr(C)]
pub struct USBUSER_SEND_RAW_COMMAND(i32);
pub const USBUSER_SET_ROOTPORT_FEATURE: u32 = 536870917u32;
pub const USBUSER_USB_REFRESH_HCT_REG: u32 = 10u32;
pub const USBUSER_VERSION: u32 = 4u32;
pub const USB_20_ENDPOINT_TYPE_INTERRUPT_RESERVED_MASK: u32 = 252u32;
pub const USB_20_HUB_DESCRIPTOR_TYPE: u32 = 41u32;
#[repr(C)]
pub struct USB_20_PORT_CHANGE(i32);
#[repr(C)]
pub struct USB_20_PORT_STATUS(i32);
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_RESERVED_MASK: u32 = 204u32;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_MASK: u32 = 48u32;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_NOTIFICATION: u32 = 16u32;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_PERIODIC: u32 = 0u32;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_RESERVED10: u32 = 32u32;
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_RESERVED11: u32 = 48u32;
#[repr(C)]
pub struct USB_30_HUB_DESCRIPTOR(i32);
pub const USB_30_HUB_DESCRIPTOR_TYPE: u32 = 42u32;
#[repr(C)]
pub struct USB_30_PORT_CHANGE(i32);
#[repr(C)]
pub struct USB_30_PORT_STATUS(i32);
pub const USB_ALLOW_FIRMWARE_UPDATE: u32 = 1u32;
#[repr(C)]
pub struct USB_BANDWIDTH_INFO(i32);
#[repr(C)]
pub struct USB_BOS_DESCRIPTOR(i32);
pub const USB_BOS_DESCRIPTOR_TYPE: u32 = 15u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USB_BUS_STATISTICS_0(i32);
pub const USB_CHARGING_POLICY_DEFAULT: u32 = 0u32;
pub const USB_CHARGING_POLICY_ICCHPF: u32 = 1u32;
pub const USB_CHARGING_POLICY_ICCLPF: u32 = 2u32;
pub const USB_CHARGING_POLICY_NO_POWER: u32 = 3u32;
#[repr(C)]
pub struct USB_CLOSE_RAW_DEVICE_PARAMETERS(i32);
#[repr(C)]
pub struct USB_COMMON_DESCRIPTOR(i32);
#[repr(C)]
pub struct USB_CONFIGURATION_DESCRIPTOR(i32);
pub const USB_CONFIGURATION_DESCRIPTOR_TYPE: u32 = 2u32;
#[repr(C)]
pub struct USB_CONFIGURATION_POWER_DESCRIPTOR(i32);
pub const USB_CONFIG_BUS_POWERED: u32 = 128u32;
pub const USB_CONFIG_POWERED_MASK: u32 = 192u32;
pub const USB_CONFIG_POWER_DESCRIPTOR_TYPE: u32 = 7u32;
pub const USB_CONFIG_REMOTE_WAKEUP: u32 = 32u32;
pub const USB_CONFIG_RESERVED: u32 = 31u32;
pub const USB_CONFIG_SELF_POWERED: u32 = 64u32;
#[repr(transparent)]
pub struct USB_CONTROLLER_FLAVOR(pub i32);
pub const USB_HcGeneric: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(0i32);
pub const OHCI_Generic: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(100i32);
pub const OHCI_Hydra: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(101i32);
pub const OHCI_NEC: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(102i32);
pub const UHCI_Generic: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(200i32);
pub const UHCI_Piix4: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(201i32);
pub const UHCI_Piix3: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(202i32);
pub const UHCI_Ich2: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(203i32);
pub const UHCI_Reserved204: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(204i32);
pub const UHCI_Ich1: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(205i32);
pub const UHCI_Ich3m: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(206i32);
pub const UHCI_Ich4: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(207i32);
pub const UHCI_Ich5: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(208i32);
pub const UHCI_Ich6: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(209i32);
pub const UHCI_Intel: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(249i32);
pub const UHCI_VIA: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(250i32);
pub const UHCI_VIA_x01: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(251i32);
pub const UHCI_VIA_x02: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(252i32);
pub const UHCI_VIA_x03: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(253i32);
pub const UHCI_VIA_x04: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(254i32);
pub const UHCI_VIA_x0E_FIFO: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(264i32);
pub const EHCI_Generic: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(1000i32);
pub const EHCI_NEC: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(2000i32);
pub const EHCI_Lucent: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(3000i32);
pub const EHCI_NVIDIA_Tegra2: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(4000i32);
pub const EHCI_NVIDIA_Tegra3: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(4001i32);
pub const EHCI_Intel_Medfield: USB_CONTROLLER_FLAVOR = USB_CONTROLLER_FLAVOR(5001i32);
#[repr(C)]
pub struct USB_CONTROLLER_INFO_0(i32);
pub const USB_CYCLE_PORT: u32 = 7u32;
pub const USB_DEBUG_DESCRIPTOR_TYPE: u32 = 10u32;
pub const USB_DEFAULT_DEVICE_ADDRESS: u32 = 0u32;
pub const USB_DEFAULT_ENDPOINT_ADDRESS: u32 = 0u32;
pub const USB_DEFAULT_MAX_PACKET: u32 = 64u32;
#[repr(C)]
pub struct USB_DEFAULT_PIPE_SETUP_PACKET(i32);
pub const USB_DEVICE_CAPABILITY_BATTERY_INFO: u32 = 7u32;
pub const USB_DEVICE_CAPABILITY_BILLBOARD: u32 = 13u32;
#[repr(C)]
pub struct USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR(i32);
pub const USB_DEVICE_CAPABILITY_CONTAINER_ID: u32 = 4u32;
#[repr(C)]
pub struct USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR(i32);
#[repr(C)]
pub struct USB_DEVICE_CAPABILITY_DESCRIPTOR(i32);
pub const USB_DEVICE_CAPABILITY_DESCRIPTOR_TYPE: u32 = 16u32;
pub const USB_DEVICE_CAPABILITY_FIRMWARE_STATUS: u32 = 17u32;
#[repr(C)]
pub struct USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR(i32);
pub const USB_DEVICE_CAPABILITY_MAX_U1_LATENCY: u32 = 10u32;
pub const USB_DEVICE_CAPABILITY_MAX_U2_LATENCY: u32 = 2047u32;
pub const USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT: u32 = 8u32;
#[repr(C)]
pub struct USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR(i32);
pub const USB_DEVICE_CAPABILITY_PD_PROVIDER_PORT: u32 = 9u32;
pub const USB_DEVICE_CAPABILITY_PLATFORM: u32 = 5u32;
#[repr(C)]
pub struct USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR(i32);
pub const USB_DEVICE_CAPABILITY_POWER_DELIVERY: u32 = 6u32;
#[repr(C)]
pub struct USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR(i32);
pub const USB_DEVICE_CAPABILITY_PRECISION_TIME_MEASUREMENT: u32 = 11u32;
#[repr(C)]
pub struct USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED(i32);
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_DIR_RX: u32 = 0u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_DIR_TX: u32 = 1u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_BPS: u32 = 0u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_GBPS: u32 = 3u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_KBPS: u32 = 1u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_MBPS: u32 = 2u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_MODE_ASYMMETRIC: u32 = 1u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_MODE_SYMMETRIC: u32 = 0u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_PROTOCOL_SS: u32 = 0u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_PROTOCOL_SSP: u32 = 1u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB: u32 = 10u32;
#[repr(C)]
pub struct USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR(i32);
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_BMATTRIBUTES_LTM_CAPABLE: u32 = 2u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_BMATTRIBUTES_RESERVED_MASK: u32 = 253u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_FULL: u32 = 2u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_HIGH: u32 = 4u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_LOW: u32 = 1u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_RESERVED_MASK: u32 = 65520u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_SUPER: u32 = 8u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_U1_DEVICE_EXIT_MAX_VALUE: u32 = 10u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_U2_DEVICE_EXIT_MAX_VALUE: u32 = 2047u32;
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_USB: u32 = 3u32;
#[repr(C)]
pub struct USB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR(i32);
pub const USB_DEVICE_CAPABILITY_USB20_EXTENSION: u32 = 2u32;
pub const USB_DEVICE_CAPABILITY_USB20_EXTENSION_BMATTRIBUTES_RESERVED_MASK: u32 = 4294901985u32;
#[repr(C)]
pub struct USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR(i32);
pub const USB_DEVICE_CAPABILITY_WIRELESS_USB: u32 = 1u32;
pub const USB_DEVICE_CHARACTERISTICS_MAXIMUM_PATH_DELAYS_AVAILABLE: u32 = 1u32;
pub const USB_DEVICE_CHARACTERISTICS_VERSION_1: u32 = 1u32;
pub const USB_DEVICE_CLASS_APPLICATION_SPECIFIC: u32 = 254u32;
pub const USB_DEVICE_CLASS_AUDIO: u32 = 1u32;
pub const USB_DEVICE_CLASS_AUDIO_VIDEO: u32 = 16u32;
pub const USB_DEVICE_CLASS_BILLBOARD: u32 = 17u32;
pub const USB_DEVICE_CLASS_CDC_DATA: u32 = 10u32;
pub const USB_DEVICE_CLASS_COMMUNICATIONS: u32 = 2u32;
pub const USB_DEVICE_CLASS_CONTENT_SECURITY: u32 = 13u32;
pub const USB_DEVICE_CLASS_DIAGNOSTIC_DEVICE: u32 = 220u32;
pub const USB_DEVICE_CLASS_HUB: u32 = 9u32;
pub const USB_DEVICE_CLASS_HUMAN_INTERFACE: u32 = 3u32;
pub const USB_DEVICE_CLASS_IMAGE: u32 = 6u32;
pub const USB_DEVICE_CLASS_MISCELLANEOUS: u32 = 239u32;
pub const USB_DEVICE_CLASS_MONITOR: u32 = 4u32;
pub const USB_DEVICE_CLASS_PERSONAL_HEALTHCARE: u32 = 15u32;
pub const USB_DEVICE_CLASS_PHYSICAL_INTERFACE: u32 = 5u32;
pub const USB_DEVICE_CLASS_POWER: u32 = 6u32;
pub const USB_DEVICE_CLASS_PRINTER: u32 = 7u32;
pub const USB_DEVICE_CLASS_RESERVED: u32 = 0u32;
pub const USB_DEVICE_CLASS_SMART_CARD: u32 = 11u32;
pub const USB_DEVICE_CLASS_STORAGE: u32 = 8u32;
pub const USB_DEVICE_CLASS_VENDOR_SPECIFIC: u32 = 255u32;
pub const USB_DEVICE_CLASS_VIDEO: u32 = 14u32;
pub const USB_DEVICE_CLASS_WIRELESS_CONTROLLER: u32 = 224u32;
#[repr(C)]
pub struct USB_DEVICE_DESCRIPTOR(i32);
pub const USB_DEVICE_DESCRIPTOR_TYPE: u32 = 1u32;
pub const USB_DEVICE_FIRMWARE_HASH_LENGTH: u32 = 32u32;
#[repr(C)]
pub struct USB_DEVICE_QUALIFIER_DESCRIPTOR(i32);
pub const USB_DEVICE_QUALIFIER_DESCRIPTOR_TYPE: u32 = 6u32;
#[repr(transparent)]
pub struct USB_DEVICE_SPEED(pub i32);
pub const UsbLowSpeed: USB_DEVICE_SPEED = USB_DEVICE_SPEED(0i32);
pub const UsbFullSpeed: USB_DEVICE_SPEED = USB_DEVICE_SPEED(1i32);
pub const UsbHighSpeed: USB_DEVICE_SPEED = USB_DEVICE_SPEED(2i32);
pub const UsbSuperSpeed: USB_DEVICE_SPEED = USB_DEVICE_SPEED(3i32);
#[repr(C)]
pub struct USB_DEVICE_STATUS(i32);
#[repr(transparent)]
pub struct USB_DEVICE_TYPE(pub i32);
pub const Usb11Device: USB_DEVICE_TYPE = USB_DEVICE_TYPE(0i32);
pub const Usb20Device: USB_DEVICE_TYPE = USB_DEVICE_TYPE(1i32);
pub const USB_DIAG_IGNORE_HUBS_OFF: u32 = 263u32;
pub const USB_DIAG_IGNORE_HUBS_ON: u32 = 262u32;
pub const USB_DISALLOW_FIRMWARE_UPDATE: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USB_DRIVER_VERSION_PARAMETERS(i32);
pub const USB_ENABLE_PORT: u32 = 5u32;
pub const USB_ENDPOINT_ADDRESS_MASK: u32 = 15u32;
#[repr(C)]
pub struct USB_ENDPOINT_DESCRIPTOR(i32);
pub const USB_ENDPOINT_DESCRIPTOR_TYPE: u32 = 5u32;
pub const USB_ENDPOINT_DIRECTION_MASK: u32 = 128u32;
#[repr(C)]
pub struct USB_ENDPOINT_STATUS(i32);
pub const USB_ENDPOINT_SUPERSPEED_BULK_MAX_PACKET_SIZE: u32 = 1024u32;
pub const USB_ENDPOINT_SUPERSPEED_CONTROL_MAX_PACKET_SIZE: u32 = 512u32;
pub const USB_ENDPOINT_SUPERSPEED_INTERRUPT_MAX_PACKET_SIZE: u32 = 1024u32;
pub const USB_ENDPOINT_SUPERSPEED_ISO_MAX_PACKET_SIZE: u32 = 1024u32;
pub const USB_ENDPOINT_TYPE_BULK: u32 = 2u32;
pub const USB_ENDPOINT_TYPE_BULK_RESERVED_MASK: u32 = 252u32;
pub const USB_ENDPOINT_TYPE_CONTROL: u32 = 0u32;
pub const USB_ENDPOINT_TYPE_CONTROL_RESERVED_MASK: u32 = 252u32;
pub const USB_ENDPOINT_TYPE_INTERRUPT: u32 = 3u32;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS: u32 = 1u32;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_RESERVED_MASK: u32 = 192u32;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_ADAPTIVE: u32 = 8u32;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_ASYNCHRONOUS: u32 = 4u32;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_MASK: u32 = 12u32;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_NO_SYNCHRONIZATION: u32 = 0u32;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_SYNCHRONOUS: u32 = 12u32;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_DATA_ENDOINT: u32 = 0u32;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_FEEDBACK_ENDPOINT: u32 = 16u32;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_IMPLICIT_FEEDBACK_DATA_ENDPOINT: u32 = 32u32;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_MASK: u32 = 48u32;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_RESERVED: u32 = 48u32;
pub const USB_ENDPOINT_TYPE_MASK: u32 = 3u32;
pub const USB_FAIL_GET_STATUS: u32 = 280u32;
pub const USB_FEATURE_BATTERY_WAKE_MASK: u32 = 40u32;
pub const USB_FEATURE_CHARGING_POLICY: u32 = 54u32;
pub const USB_FEATURE_ENDPOINT_STALL: u32 = 0u32;
pub const USB_FEATURE_FUNCTION_SUSPEND: u32 = 0u32;
pub const USB_FEATURE_INTERFACE_POWER_D0: u32 = 2u32;
pub const USB_FEATURE_INTERFACE_POWER_D1: u32 = 3u32;
pub const USB_FEATURE_INTERFACE_POWER_D2: u32 = 4u32;
pub const USB_FEATURE_INTERFACE_POWER_D3: u32 = 5u32;
pub const USB_FEATURE_LDM_ENABLE: u32 = 53u32;
pub const USB_FEATURE_LTM_ENABLE: u32 = 50u32;
pub const USB_FEATURE_OS_IS_PD_AWARE: u32 = 41u32;
pub const USB_FEATURE_POLICY_MODE: u32 = 42u32;
pub const USB_FEATURE_REMOTE_WAKEUP: u32 = 1u32;
pub const USB_FEATURE_TEST_MODE: u32 = 2u32;
pub const USB_FEATURE_U1_ENABLE: u32 = 48u32;
pub const USB_FEATURE_U2_ENABLE: u32 = 49u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION(i32);
#[repr(C)]
pub struct USB_FUNCTION_SUSPEND_OPTIONS(i32);
pub const USB_GETSTATUS_LTM_ENABLE: u32 = 16u32;
pub const USB_GETSTATUS_REMOTE_WAKEUP_ENABLED: u32 = 2u32;
pub const USB_GETSTATUS_SELF_POWERED: u32 = 1u32;
pub const USB_GETSTATUS_U1_ENABLE: u32 = 4u32;
pub const USB_GETSTATUS_U2_ENABLE: u32 = 8u32;
pub const USB_GET_BUSGUID_INFO: u32 = 266u32;
pub const USB_GET_BUS_INFO: u32 = 264u32;
pub const USB_GET_CONTROLLER_NAME: u32 = 265u32;
pub const USB_GET_DESCRIPTOR_FROM_NODE_CONNECTION: u32 = 260u32;
pub const USB_GET_DEVICE_CHARACTERISTICS: u32 = 288u32;
pub const USB_GET_DEVICE_HANDLE: u32 = 268u32;
pub const USB_GET_DEVICE_HANDLE_EX: u32 = 269u32;
pub const USB_GET_FIRMWARE_ALLOWED_OR_DISALLOWED_STATE: u32 = 0u32;
pub const USB_GET_FIRMWARE_HASH: u32 = 1u32;
pub const USB_GET_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC: u32 = 286u32;
pub const USB_GET_HUB_CAPABILITIES: u32 = 271u32;
pub const USB_GET_HUB_CAPABILITIES_EX: u32 = 276u32;
pub const USB_GET_HUB_CONFIG_INFO: u32 = 275u32;
pub const USB_GET_HUB_COUNT: u32 = 6u32;
pub const USB_GET_HUB_INFORMATION_EX: u32 = 277u32;
pub const USB_GET_HUB_NAME: u32 = 8u32;
pub const USB_GET_NODE_CONNECTION_ATTRIBUTES: u32 = 272u32;
pub const USB_GET_NODE_CONNECTION_DRIVERKEY_NAME: u32 = 264u32;
pub const USB_GET_NODE_CONNECTION_INFORMATION: u32 = 259u32;
pub const USB_GET_NODE_CONNECTION_INFORMATION_EX: u32 = 274u32;
pub const USB_GET_NODE_CONNECTION_INFORMATION_EX_V2: u32 = 279u32;
pub const USB_GET_NODE_CONNECTION_NAME: u32 = 261u32;
pub const USB_GET_NODE_INFORMATION: u32 = 258u32;
pub const USB_GET_PARENT_HUB_INFO: u32 = 267u32;
pub const USB_GET_PORT_CONNECTOR_PROPERTIES: u32 = 278u32;
pub const USB_GET_PORT_STATUS: u32 = 4u32;
pub const USB_GET_ROOTHUB_PDO: u32 = 3u32;
pub const USB_GET_TOPOLOGY_ADDRESS: u32 = 271u32;
pub const USB_GET_TRANSPORT_CHARACTERISTICS: u32 = 281u32;
pub const USB_GET_TT_DEVICE_HANDLE: u32 = 270u32;
pub const USB_HC_FEATURE_FLAG_PORT_POWER_SWITCHING: u32 = 1u32;
pub const USB_HC_FEATURE_FLAG_SEL_SUSPEND: u32 = 2u32;
pub const USB_HC_FEATURE_LEGACY_BIOS: u32 = 4u32;
pub const USB_HC_FEATURE_TIME_SYNC_API: u32 = 8u32;
#[repr(C)]
pub struct USB_HIGH_SPEED_MAXPACKET(i32);
#[repr(C)]
pub struct USB_HUB_30_PORT_REMOTE_WAKE_MASK(i32);
#[repr(C)]
pub struct USB_HUB_CHANGE(i32);
pub const USB_HUB_CYCLE_PORT: u32 = 273u32;
#[repr(C)]
pub struct USB_HUB_DESCRIPTOR(i32);
#[repr(C)]
pub struct USB_HUB_STATUS(i32);
#[repr(C)]
pub struct USB_HUB_STATUS_AND_CHANGE(i32);
#[repr(C)]
pub struct USB_IDLE_CALLBACK(i32);
#[repr(C)]
pub struct USB_IDLE_CALLBACK_INFO(i32);
pub const USB_IDLE_NOTIFICATION: u32 = 9u32;
pub const USB_IDLE_NOTIFICATION_EX: u32 = 272u32;
#[repr(C)]
pub struct USB_INTERFACE_ASSOCIATION_DESCRIPTOR(i32);
pub const USB_INTERFACE_ASSOCIATION_DESCRIPTOR_TYPE: u32 = 11u32;
#[repr(C)]
pub struct USB_INTERFACE_DESCRIPTOR(i32);
pub const USB_INTERFACE_DESCRIPTOR_TYPE: u32 = 4u32;
#[repr(C)]
pub struct USB_INTERFACE_POWER_DESCRIPTOR(i32);
pub const USB_INTERFACE_POWER_DESCRIPTOR_TYPE: u32 = 8u32;
#[repr(C)]
pub struct USB_INTERFACE_STATUS(i32);
pub const USB_NOTIFY_ON_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 283u32;
#[repr(C)]
pub struct USB_OPEN_RAW_DEVICE_PARAMETERS(i32);
pub const USB_OTG_DESCRIPTOR_TYPE: u32 = 9u32;
pub const USB_OTHER_SPEED_CONFIGURATION_DESCRIPTOR_TYPE: u32 = 7u32;
pub const USB_PACKETFLAG_ASYNC_IN: u32 = 8u32;
pub const USB_PACKETFLAG_ASYNC_OUT: u32 = 16u32;
pub const USB_PACKETFLAG_FULL_SPEED: u32 = 2u32;
pub const USB_PACKETFLAG_HIGH_SPEED: u32 = 4u32;
pub const USB_PACKETFLAG_ISO_IN: u32 = 32u32;
pub const USB_PACKETFLAG_ISO_OUT: u32 = 64u32;
pub const USB_PACKETFLAG_LOW_SPEED: u32 = 1u32;
pub const USB_PACKETFLAG_SETUP: u32 = 128u32;
pub const USB_PACKETFLAG_TOGGLE0: u32 = 256u32;
pub const USB_PACKETFLAG_TOGGLE1: u32 = 512u32;
#[repr(C)]
pub struct USB_PASS_THRU_PARAMETERS(i32);
pub const USB_PORTATTR_MINI_CONNECTOR: u32 = 4u32;
pub const USB_PORTATTR_NO_CONNECTOR: u32 = 1u32;
pub const USB_PORTATTR_NO_OVERCURRENT_UI: u32 = 33554432u32;
pub const USB_PORTATTR_OEM_CONNECTOR: u32 = 8u32;
pub const USB_PORTATTR_OWNED_BY_CC: u32 = 16777216u32;
pub const USB_PORTATTR_SHARED_USB2: u32 = 2u32;
#[repr(C)]
pub struct USB_PORT_CHANGE(i32);
#[repr(C)]
pub struct USB_PORT_EXT_STATUS(i32);
#[repr(C)]
pub struct USB_PORT_EXT_STATUS_AND_CHANGE(i32);
#[repr(C)]
pub struct USB_PORT_STATUS(i32);
#[repr(C)]
pub struct USB_PORT_STATUS_AND_CHANGE(i32);
pub const USB_PORT_STATUS_CONNECT: u32 = 1u32;
pub const USB_PORT_STATUS_ENABLE: u32 = 2u32;
pub const USB_PORT_STATUS_HIGH_SPEED: u32 = 1024u32;
pub const USB_PORT_STATUS_LOW_SPEED: u32 = 512u32;
pub const USB_PORT_STATUS_OVER_CURRENT: u32 = 8u32;
pub const USB_PORT_STATUS_POWER: u32 = 256u32;
pub const USB_PORT_STATUS_RESET: u32 = 16u32;
pub const USB_PORT_STATUS_SUSPEND: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USB_POWER_INFO(i32);
pub const USB_RECORD_FAILURE: u32 = 10u32;
pub const USB_REGISTER_COMPOSITE_DEVICE: u32 = 0u32;
pub const USB_REGISTER_FOR_TRANSPORT_BANDWIDTH_CHANGE: u32 = 2u32;
pub const USB_REGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 282u32;
pub const USB_REGISTER_FOR_TRANSPORT_LATENCY_CHANGE: u32 = 1u32;
pub const USB_REQUEST_CLEAR_FEATURE: u32 = 1u32;
pub const USB_REQUEST_CLEAR_TT_BUFFER: u32 = 8u32;
pub const USB_REQUEST_GET_CONFIGURATION: u32 = 8u32;
pub const USB_REQUEST_GET_DESCRIPTOR: u32 = 6u32;
pub const USB_REQUEST_GET_FIRMWARE_STATUS: u32 = 26u32;
pub const USB_REQUEST_GET_INTERFACE: u32 = 10u32;
pub const USB_REQUEST_GET_PORT_ERR_COUNT: u32 = 13u32;
pub const USB_REQUEST_GET_STATE: u32 = 2u32;
pub const USB_REQUEST_GET_STATUS: u32 = 0u32;
pub const USB_REQUEST_GET_TT_STATE: u32 = 10u32;
pub const USB_REQUEST_ISOCH_DELAY: u32 = 49u32;
pub const USB_REQUEST_REMOTE_WAKE_NOTIFICATION: u32 = 2u32;
pub const USB_REQUEST_RESET_TT: u32 = 9u32;
pub const USB_REQUEST_SET_ADDRESS: u32 = 5u32;
pub const USB_REQUEST_SET_CONFIGURATION: u32 = 9u32;
pub const USB_REQUEST_SET_DESCRIPTOR: u32 = 7u32;
pub const USB_REQUEST_SET_FEATURE: u32 = 3u32;
pub const USB_REQUEST_SET_FIRMWARE_STATUS: u32 = 27u32;
pub const USB_REQUEST_SET_HUB_DEPTH: u32 = 12u32;
pub const USB_REQUEST_SET_INTERFACE: u32 = 11u32;
pub const USB_REQUEST_SET_SEL: u32 = 48u32;
pub const USB_REQUEST_STOP_TT: u32 = 11u32;
pub const USB_REQUEST_SYNC_FRAME: u32 = 12u32;
pub const USB_REQ_GLOBAL_RESUME: u32 = 274u32;
pub const USB_REQ_GLOBAL_SUSPEND: u32 = 273u32;
pub const USB_RESERVED_DESCRIPTOR_TYPE: u32 = 6u32;
pub const USB_RESET_HUB: u32 = 275u32;
pub const USB_RESET_PORT: u32 = 1u32;
#[repr(C)]
pub struct USB_SEND_RAW_COMMAND_PARAMETERS(i32);
pub const USB_START_TRACKING_FOR_TIME_SYNC: u32 = 285u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION(i32);
pub const USB_STATUS_EXT_PORT_STATUS: u32 = 2u32;
pub const USB_STATUS_PD_STATUS: u32 = 1u32;
pub const USB_STATUS_PORT_STATUS: u32 = 0u32;
pub const USB_STOP_TRACKING_FOR_TIME_SYNC: u32 = 287u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION(i32);
#[repr(C)]
pub struct USB_STRING_DESCRIPTOR(i32);
pub const USB_STRING_DESCRIPTOR_TYPE: u32 = 3u32;
pub const USB_SUBMIT_URB: u32 = 0u32;
pub const USB_SUPERSPEEDPLUS_ISOCHRONOUS_MAX_BYTESPERINTERVAL: u32 = 16777215u32;
pub const USB_SUPERSPEEDPLUS_ISOCHRONOUS_MIN_BYTESPERINTERVAL: u32 = 49153u32;
#[repr(C)]
pub struct USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR(i32);
pub const USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR_TYPE: u32 = 49u32;
#[repr(C)]
pub struct USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR(i32);
pub const USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_TYPE: u32 = 48u32;
pub const USB_SUPERSPEED_ISOCHRONOUS_MAX_MULTIPLIER: u32 = 2u32;
pub const USB_SUPPORT_D0_COMMAND: u32 = 1u32;
pub const USB_SUPPORT_D1_COMMAND: u32 = 2u32;
pub const USB_SUPPORT_D1_WAKEUP: u32 = 16u32;
pub const USB_SUPPORT_D2_COMMAND: u32 = 4u32;
pub const USB_SUPPORT_D2_WAKEUP: u32 = 32u32;
pub const USB_SUPPORT_D3_COMMAND: u32 = 8u32;
pub const USB_TEST_MODE_TEST_FORCE_ENABLE: u32 = 5u32;
pub const USB_TEST_MODE_TEST_J: u32 = 1u32;
pub const USB_TEST_MODE_TEST_K: u32 = 2u32;
pub const USB_TEST_MODE_TEST_PACKET: u32 = 4u32;
pub const USB_TEST_MODE_TEST_SE0_NAK: u32 = 3u32;
pub const USB_TRANSPORT_CHARACTERISTICS_BANDWIDTH_AVAILABLE: u32 = 2u32;
pub const USB_TRANSPORT_CHARACTERISTICS_LATENCY_AVAILABLE: u32 = 1u32;
pub const USB_TRANSPORT_CHARACTERISTICS_VERSION_1: u32 = 1u32;
#[repr(C)]
pub struct USB_UNICODE_NAME(i32);
pub const USB_UNREGISTER_COMPOSITE_DEVICE: u32 = 1u32;
pub const USB_UNREGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 284u32;
#[repr(C)]
pub struct USB_USB2HW_VERSION_PARAMETERS(i32);
#[repr(transparent)]
pub struct USB_USER_ERROR_CODE(pub i32);
pub const UsbUserSuccess: USB_USER_ERROR_CODE = USB_USER_ERROR_CODE(0i32);
pub const UsbUserNotSupported: USB_USER_ERROR_CODE = USB_USER_ERROR_CODE(1i32);
pub const UsbUserInvalidRequestCode: USB_USER_ERROR_CODE = USB_USER_ERROR_CODE(2i32);
pub const UsbUserFeatureDisabled: USB_USER_ERROR_CODE = USB_USER_ERROR_CODE(3i32);
pub const UsbUserInvalidHeaderParameter: USB_USER_ERROR_CODE = USB_USER_ERROR_CODE(4i32);
pub const UsbUserInvalidParameter: USB_USER_ERROR_CODE = USB_USER_ERROR_CODE(5i32);
pub const UsbUserMiniportError: USB_USER_ERROR_CODE = USB_USER_ERROR_CODE(6i32);
pub const UsbUserBufferTooSmall: USB_USER_ERROR_CODE = USB_USER_ERROR_CODE(7i32);
pub const UsbUserErrorNotMapped: USB_USER_ERROR_CODE = USB_USER_ERROR_CODE(8i32);
pub const UsbUserDeviceNotStarted: USB_USER_ERROR_CODE = USB_USER_ERROR_CODE(9i32);
pub const UsbUserNoDeviceConnected: USB_USER_ERROR_CODE = USB_USER_ERROR_CODE(10i32);
#[repr(transparent)]
pub struct WDMUSB_POWER_STATE(pub i32);
pub const WdmUsbPowerNotMapped: WDMUSB_POWER_STATE = WDMUSB_POWER_STATE(0i32);
pub const WdmUsbPowerSystemUnspecified: WDMUSB_POWER_STATE = WDMUSB_POWER_STATE(100i32);
pub const WdmUsbPowerSystemWorking: WDMUSB_POWER_STATE = WDMUSB_POWER_STATE(101i32);
pub const WdmUsbPowerSystemSleeping1: WDMUSB_POWER_STATE = WDMUSB_POWER_STATE(102i32);
pub const WdmUsbPowerSystemSleeping2: WDMUSB_POWER_STATE = WDMUSB_POWER_STATE(103i32);
pub const WdmUsbPowerSystemSleeping3: WDMUSB_POWER_STATE = WDMUSB_POWER_STATE(104i32);
pub const WdmUsbPowerSystemHibernate: WDMUSB_POWER_STATE = WDMUSB_POWER_STATE(105i32);
pub const WdmUsbPowerSystemShutdown: WDMUSB_POWER_STATE = WDMUSB_POWER_STATE(106i32);
pub const WdmUsbPowerDeviceUnspecified: WDMUSB_POWER_STATE = WDMUSB_POWER_STATE(200i32);
pub const WdmUsbPowerDeviceD0: WDMUSB_POWER_STATE = WDMUSB_POWER_STATE(201i32);
pub const WdmUsbPowerDeviceD1: WDMUSB_POWER_STATE = WDMUSB_POWER_STATE(202i32);
pub const WdmUsbPowerDeviceD2: WDMUSB_POWER_STATE = WDMUSB_POWER_STATE(203i32);
pub const WdmUsbPowerDeviceD3: WDMUSB_POWER_STATE = WDMUSB_POWER_STATE(204i32);
#[repr(C)]
pub struct WINUSB_PIPE_INFORMATION(i32);
#[repr(C)]
pub struct WINUSB_PIPE_INFORMATION_EX(i32);
#[repr(C)]
pub struct WINUSB_SETUP_PACKET(i32);
pub const WMI_USB_DEVICE_NODE_INFORMATION: u32 = 2u32;
pub const WMI_USB_DRIVER_INFORMATION: u32 = 0u32;
pub const WMI_USB_DRIVER_NOTIFICATION: u32 = 1u32;
pub const WMI_USB_HUB_NODE_INFORMATION: u32 = 4u32;
pub const WMI_USB_PERFORMANCE_INFORMATION: u32 = 1u32;
pub const WMI_USB_POWER_DEVICE_ENABLE: u32 = 2u32;
pub const WinUSB_TestGuid: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3665898495, data2: 4803, data3: 18082, data4: [142, 43, 219, 211, 183, 131, 76, 67] };
#[repr(C)]
pub struct _URB_BULK_OR_INTERRUPT_TRANSFER(i32);
#[repr(C)]
pub struct _URB_CONTROL_DESCRIPTOR_REQUEST(i32);
#[repr(C)]
pub struct _URB_CONTROL_FEATURE_REQUEST(i32);
#[repr(C)]
pub struct _URB_CONTROL_GET_CONFIGURATION_REQUEST(i32);
#[repr(C)]
pub struct _URB_CONTROL_GET_INTERFACE_REQUEST(i32);
#[repr(C)]
pub struct _URB_CONTROL_GET_STATUS_REQUEST(i32);
#[repr(C)]
pub struct _URB_CONTROL_TRANSFER(i32);
#[repr(C)]
pub struct _URB_CONTROL_TRANSFER_EX(i32);
#[repr(C)]
pub struct _URB_CONTROL_VENDOR_OR_CLASS_REQUEST(i32);
#[repr(C)]
pub struct _URB_FRAME_LENGTH_CONTROL(i32);
#[repr(C)]
pub struct _URB_GET_CURRENT_FRAME_NUMBER(i32);
#[repr(C)]
pub struct _URB_GET_FRAME_LENGTH(i32);
#[repr(C)]
pub struct _URB_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS(i32);
#[repr(C)]
pub struct _URB_HCD_AREA(i32);
#[repr(C)]
pub struct _URB_HEADER(i32);
#[repr(C)]
pub struct _URB_ISOCH_TRANSFER(i32);
#[repr(C)]
pub struct _URB_OPEN_STATIC_STREAMS(i32);
#[repr(C)]
pub struct _URB_OS_FEATURE_DESCRIPTOR_REQUEST(i32);
#[repr(C)]
pub struct _URB_PIPE_REQUEST(i32);
#[repr(C)]
pub struct _URB_SELECT_CONFIGURATION(i32);
#[repr(C)]
pub struct _URB_SELECT_INTERFACE(i32);
#[repr(C)]
pub struct _URB_SET_FRAME_LENGTH(i32);
