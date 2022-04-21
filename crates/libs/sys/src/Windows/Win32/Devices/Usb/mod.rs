#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_AbortPipe(interfacehandle: *const ::core::ffi::c_void, pipeid: u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_ControlTransfer(interfacehandle: *const ::core::ffi::c_void, setuppacket: WINUSB_SETUP_PACKET, buffer: *mut u8, bufferlength: u32, lengthtransferred: *mut u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_FlushPipe(interfacehandle: *const ::core::ffi::c_void, pipeid: u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_Free(interfacehandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetAdjustedFrameNumber(currentframenumber: *mut u32, timestamp: i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetAssociatedInterface(interfacehandle: *const ::core::ffi::c_void, associatedinterfaceindex: u8, associatedinterfacehandle: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetCurrentAlternateSetting(interfacehandle: *const ::core::ffi::c_void, settingnumber: *mut u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetCurrentFrameNumber(interfacehandle: *const ::core::ffi::c_void, currentframenumber: *mut u32, timestamp: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetCurrentFrameNumberAndQpc(interfacehandle: *const ::core::ffi::c_void, frameqpcinfo: *const USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetDescriptor(interfacehandle: *const ::core::ffi::c_void, descriptortype: u8, index: u8, languageid: u16, buffer: *mut u8, bufferlength: u32, lengthtransferred: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_GetOverlappedResult(interfacehandle: *const ::core::ffi::c_void, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetPipePolicy(interfacehandle: *const ::core::ffi::c_void, pipeid: u8, policytype: u32, valuelength: *mut u32, value: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetPowerPolicy(interfacehandle: *const ::core::ffi::c_void, policytype: u32, valuelength: *mut u32, value: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_Initialize(devicehandle: super::super::Foundation::HANDLE, interfacehandle: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
    pub fn WinUsb_ParseConfigurationDescriptor(configurationdescriptor: *const USB_CONFIGURATION_DESCRIPTOR, startposition: *const ::core::ffi::c_void, interfacenumber: i32, alternatesetting: i32, interfaceclass: i32, interfacesubclass: i32, interfaceprotocol: i32) -> *mut USB_INTERFACE_DESCRIPTOR;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
    pub fn WinUsb_ParseDescriptors(descriptorbuffer: *const ::core::ffi::c_void, totallength: u32, startposition: *const ::core::ffi::c_void, descriptortype: i32) -> *mut USB_COMMON_DESCRIPTOR;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_QueryDeviceInformation(interfacehandle: *const ::core::ffi::c_void, informationtype: u32, bufferlength: *mut u32, buffer: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_QueryInterfaceSettings(interfacehandle: *const ::core::ffi::c_void, alternateinterfacenumber: u8, usbaltinterfacedescriptor: *mut USB_INTERFACE_DESCRIPTOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_QueryPipe(interfacehandle: *const ::core::ffi::c_void, alternateinterfacenumber: u8, pipeindex: u8, pipeinformation: *mut WINUSB_PIPE_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_QueryPipeEx(interfacehandle: *const ::core::ffi::c_void, alternatesettingnumber: u8, pipeindex: u8, pipeinformationex: *mut WINUSB_PIPE_INFORMATION_EX) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_ReadIsochPipe(bufferhandle: *const ::core::ffi::c_void, offset: u32, length: u32, framenumber: *mut u32, numberofpackets: u32, isopacketdescriptors: *mut USBD_ISO_PACKET_DESCRIPTOR, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_ReadIsochPipeAsap(bufferhandle: *const ::core::ffi::c_void, offset: u32, length: u32, continuestream: super::super::Foundation::BOOL, numberofpackets: u32, isopacketdescriptors: *mut USBD_ISO_PACKET_DESCRIPTOR, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_ReadPipe(interfacehandle: *const ::core::ffi::c_void, pipeid: u8, buffer: *mut u8, bufferlength: u32, lengthtransferred: *mut u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_RegisterIsochBuffer(interfacehandle: *const ::core::ffi::c_void, pipeid: u8, buffer: *mut u8, bufferlength: u32, isochbufferhandle: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_ResetPipe(interfacehandle: *const ::core::ffi::c_void, pipeid: u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_SetCurrentAlternateSetting(interfacehandle: *const ::core::ffi::c_void, settingnumber: u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_SetPipePolicy(interfacehandle: *const ::core::ffi::c_void, pipeid: u8, policytype: u32, valuelength: u32, value: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_SetPowerPolicy(interfacehandle: *const ::core::ffi::c_void, policytype: u32, valuelength: u32, value: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_StartTrackingForTimeSync(interfacehandle: *const ::core::ffi::c_void, starttrackinginfo: *const USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_StopTrackingForTimeSync(interfacehandle: *const ::core::ffi::c_void, stoptrackinginfo: *const USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_UnregisterIsochBuffer(isochbufferhandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_WriteIsochPipe(bufferhandle: *const ::core::ffi::c_void, offset: u32, length: u32, framenumber: *mut u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_WriteIsochPipeAsap(bufferhandle: *const ::core::ffi::c_void, offset: u32, length: u32, continuestream: super::super::Foundation::BOOL, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_WritePipe(interfacehandle: *const ::core::ffi::c_void, pipeid: u8, buffer: *const u8, bufferlength: u32, lengthtransferred: *mut u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const ALLOW_PARTIAL_READS: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct ALTERNATE_INTERFACE {
    pub InterfaceNumber: u16,
    pub AlternateInterfaceNumber: u16,
}
impl ::core::marker::Copy for ALTERNATE_INTERFACE {}
impl ::core::clone::Clone for ALTERNATE_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const AUTO_CLEAR_STALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const AUTO_FLUSH: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const AUTO_SUSPEND: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const BMREQUEST_CLASS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const BMREQUEST_DEVICE_TO_HOST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const BMREQUEST_HOST_TO_DEVICE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const BMREQUEST_STANDARD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const BMREQUEST_TO_DEVICE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const BMREQUEST_TO_ENDPOINT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const BMREQUEST_TO_INTERFACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const BMREQUEST_TO_OTHER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const BMREQUEST_VENDOR: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union BM_REQUEST_TYPE {
    pub s: BM_REQUEST_TYPE_0,
    pub B: u8,
}
impl ::core::marker::Copy for BM_REQUEST_TYPE {}
impl ::core::clone::Clone for BM_REQUEST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct BM_REQUEST_TYPE_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for BM_REQUEST_TYPE_0 {}
impl ::core::clone::Clone for BM_REQUEST_TYPE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const BULKIN_FLAG: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct CHANNEL_INFO {
    pub EventChannelSize: u32,
    pub uReadDataAlignment: u32,
    pub uWriteDataAlignment: u32,
}
impl ::core::marker::Copy for CHANNEL_INFO {}
impl ::core::clone::Clone for CHANNEL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct DEVICE_DESCRIPTOR {
    pub usVendorId: u16,
    pub usProductId: u16,
    pub usBcdDevice: u16,
    pub usLanguageId: u16,
}
impl ::core::marker::Copy for DEVICE_DESCRIPTOR {}
impl ::core::clone::Clone for DEVICE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const DEVICE_SPEED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct DRV_VERSION {
    pub major: u32,
    pub minor: u32,
    pub internal: u32,
}
impl ::core::marker::Copy for DRV_VERSION {}
impl ::core::clone::Clone for DRV_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const FILE_DEVICE_USB: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const FILE_DEVICE_USB_SCAN: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const FullSpeed: u32 = 2u32;
pub const GUID_DEVINTERFACE_USB_BILLBOARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1587206895, data2: 63609, data3: 18239, data4: [184, 7, 78, 94, 167, 125, 27, 28] };
pub const GUID_DEVINTERFACE_USB_DEVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2782707472, data2: 25904, data3: 4562, data4: [144, 31, 0, 192, 79, 185, 81, 237] };
pub const GUID_DEVINTERFACE_USB_HOST_CONTROLLER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 985624365, data2: 29124, data3: 17962, data4: [138, 146, 30, 104, 97, 230, 175, 39] };
pub const GUID_DEVINTERFACE_USB_HUB: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4052356744, data2: 49932, data3: 4560, data4: [136, 21, 0, 160, 201, 6, 190, 216] };
pub const GUID_USB_MSOS20_PLATFORM_CAPABILITY_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3638386911, data2: 17801, data3: 19655, data4: [156, 210, 101, 157, 158, 100, 138, 159] };
pub const GUID_USB_PERFORMANCE_TRACING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3588126630, data2: 27369, data3: 16988, data4: [177, 226, 245, 97, 95, 211, 72, 169] };
pub const GUID_USB_TRANSFER_TRACING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1746843818, data2: 16445, data3: 17708, data4: [159, 138, 240, 97, 111, 172, 149, 64] };
pub const GUID_USB_WMI_DEVICE_PERF_INFO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1723968060, data2: 18847, data3: 18848, data4: [169, 165, 97, 226, 53, 159, 100, 7] };
pub const GUID_USB_WMI_NODE_INFO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2618790743, data2: 56442, data3: 20289, data4: [182, 107, 50, 59, 157, 220, 181, 177] };
pub const GUID_USB_WMI_STD_DATA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1315060512, data2: 51988, data3: 4561, data4: [179, 49, 0, 160, 201, 89, 187, 210] };
pub const GUID_USB_WMI_STD_NOTIFICATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1315060512, data2: 51988, data3: 4561, data4: [179, 49, 0, 160, 201, 89, 187, 210] };
pub const GUID_USB_WMI_SURPRISE_REMOVAL_NOTIFICATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2612787249, data2: 41714, data3: 17332, data4: [150, 209, 134, 148, 75, 89, 20, 179] };
pub const GUID_USB_WMI_TRACING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 979470363, data2: 46310, data3: 19449, data4: [174, 15, 60, 216, 243, 148, 229, 47] };
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const HCD_DIAGNOSTIC_MODE_OFF: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const HCD_DIAGNOSTIC_MODE_ON: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const HCD_DISABLE_PORT: u32 = 268u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const HCD_ENABLE_PORT: u32 = 269u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const HCD_GET_DRIVERKEY_NAME: u32 = 265u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const HCD_GET_ROOT_HUB_NAME: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const HCD_GET_STATS_1: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const HCD_GET_STATS_2: u32 = 266u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const HCD_TRACE_READ_REQUEST: u32 = 275u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const HCD_USER_REQUEST: u32 = 270u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const HighSpeed: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IGNORE_SHORT_PACKETS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_ABORT_PIPE: u32 = 2147491844u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_CANCEL_IO: u32 = 2147491844u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GENERICUSBFN_ACTIVATE_USB_BUS: u32 = 2277420u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GENERICUSBFN_BUS_EVENT_NOTIFICATION: u32 = 2277430u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GENERICUSBFN_CONTROL_STATUS_HANDSHAKE_IN: u32 = 2277400u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GENERICUSBFN_CONTROL_STATUS_HANDSHAKE_OUT: u32 = 2277404u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GENERICUSBFN_DEACTIVATE_USB_BUS: u32 = 2277424u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GENERICUSBFN_GET_CLASS_INFO: u32 = 2277410u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GENERICUSBFN_GET_CLASS_INFO_EX: u32 = 2277434u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GENERICUSBFN_GET_INTERFACE_DESCRIPTOR_SET: u32 = 2277438u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GENERICUSBFN_GET_PIPE_STATE: u32 = 2277414u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GENERICUSBFN_REGISTER_USB_STRING: u32 = 2277441u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GENERICUSBFN_SET_PIPE_STATE: u32 = 2277417u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GENERICUSBFN_TRANSFER_IN: u32 = 2277389u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GENERICUSBFN_TRANSFER_IN_APPEND_ZERO_PKT: u32 = 2277393u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GENERICUSBFN_TRANSFER_OUT: u32 = 2277398u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GET_CHANNEL_ALIGN_RQST: u32 = 2147491860u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GET_DEVICE_DESCRIPTOR: u32 = 2147491864u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GET_HCD_DRIVERKEY_NAME: u32 = 2229284u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GET_PIPE_CONFIGURATION: u32 = 2147491880u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GET_USB_DESCRIPTOR: u32 = 2147491872u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_GET_VERSION: u32 = 2147491840u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INDEX: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_CYCLE_PORT: u32 = 2228255u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_ENABLE_PORT: u32 = 2228247u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_FAIL_GET_STATUS_FROM_DEVICE: u32 = 2229347u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_GET_BUSGUID_INFO: u32 = 2229288u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_GET_BUS_INFO: u32 = 2229280u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_GET_CONTROLLER_NAME: u32 = 2229284u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_GET_DEVICE_CONFIG_INFO: u32 = 2229327u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE: u32 = 2229299u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE_EX: u32 = 2229303u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_GET_HUB_COUNT: u32 = 2228251u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_GET_HUB_NAME: u32 = 2228256u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_GET_PARENT_HUB_INFO: u32 = 2229292u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_GET_PORT_STATUS: u32 = 2228243u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_GET_ROOTHUB_PDO: u32 = 2228239u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_GET_TOPOLOGY_ADDRESS: u32 = 2229311u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_GET_TT_DEVICE_HANDLE: u32 = 2229307u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_NOTIFY_IDLE_READY: u32 = 2229315u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_RECORD_FAILURE: u32 = 2228267u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_REGISTER_COMPOSITE_DEVICE: u32 = 4784131u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_REQUEST_REMOTE_WAKE_NOTIFICATION: u32 = 4784139u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_REQ_GLOBAL_RESUME: u32 = 2229323u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_REQ_GLOBAL_SUSPEND: u32 = 2229319u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_RESET_PORT: u32 = 2228231u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_SUBMIT_IDLE_NOTIFICATION: u32 = 2228263u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_SUBMIT_URB: u32 = 2228227u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_INTERNAL_USB_UNREGISTER_COMPOSITE_DEVICE: u32 = 4784135u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_READ_REGISTERS: u32 = 2147491852u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_RESET_PIPE: u32 = 2147491868u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_SEND_USB_REQUEST: u32 = 2147491876u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_SET_TIMEOUT: u32 = 2147491884u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_DIAGNOSTIC_MODE_OFF: u32 = 2229252u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_DIAGNOSTIC_MODE_ON: u32 = 2229248u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_DIAG_IGNORE_HUBS_OFF: u32 = 2229276u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_DIAG_IGNORE_HUBS_ON: u32 = 2229272u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_GET_DESCRIPTOR_FROM_NODE_CONNECTION: u32 = 2229264u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_GET_DEVICE_CHARACTERISTICS: u32 = 2229376u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_GET_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC: u32 = 2229368u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_GET_HUB_CAPABILITIES: u32 = 2229308u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_GET_HUB_CAPABILITIES_EX: u32 = 2229328u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_GET_HUB_INFORMATION_EX: u32 = 2229332u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_GET_NODE_CONNECTION_ATTRIBUTES: u32 = 2229312u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_GET_NODE_CONNECTION_DRIVERKEY_NAME: u32 = 2229280u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_GET_NODE_CONNECTION_INFORMATION: u32 = 2229260u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX: u32 = 2229320u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2: u32 = 2229340u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_GET_NODE_CONNECTION_NAME: u32 = 2229268u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_GET_NODE_INFORMATION: u32 = 2229256u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_GET_PORT_CONNECTOR_PROPERTIES: u32 = 2229336u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_GET_ROOT_HUB_NAME: u32 = 2229256u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_GET_TRANSPORT_CHARACTERISTICS: u32 = 2229348u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_HCD_DISABLE_PORT: u32 = 2229296u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_HCD_ENABLE_PORT: u32 = 2229300u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_HCD_GET_STATS_1: u32 = 2229244u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_HCD_GET_STATS_2: u32 = 2229288u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_HUB_CYCLE_PORT: u32 = 2229316u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_NOTIFY_ON_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 2229356u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_REGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 2229352u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_RESET_HUB: u32 = 2229324u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_START_TRACKING_FOR_TIME_SYNC: u32 = 2229364u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_STOP_TRACKING_FOR_TIME_SYNC: u32 = 2229372u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_USB_UNREGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 2229360u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_WAIT_ON_DEVICE_EVENT: u32 = 2147491848u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const IOCTL_WRITE_REGISTERS: u32 = 2147491856u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct IO_BLOCK {
    pub uOffset: u32,
    pub uLength: u32,
    pub pbyData: *mut u8,
    pub uIndex: u32,
}
impl ::core::marker::Copy for IO_BLOCK {}
impl ::core::clone::Clone for IO_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct IO_BLOCK_EX {
    pub uOffset: u32,
    pub uLength: u32,
    pub pbyData: *mut u8,
    pub uIndex: u32,
    pub bRequest: u8,
    pub bmRequestType: u8,
    pub fTransferDirectionIn: u8,
}
impl ::core::marker::Copy for IO_BLOCK_EX {}
impl ::core::clone::Clone for IO_BLOCK_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const KREGMANUSBFNENUMPATH: &str = "\\Registry\\Machine\\SYSTEM\\CurrentControlSet\\Control\\ManufacturingMode\\Current\\USBFN\\";
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const KREGUSBFNENUMPATH: &str = "\\Registry\\Machine\\SYSTEM\\CurrentControlSet\\Control\\USBFN\\";
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const LowSpeed: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const MAXIMUM_TRANSFER_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const MAXIMUM_USB_STRING_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const MAX_ALTERNATE_NAME_LENGTH: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const MAX_ASSOCIATION_NAME_LENGTH: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const MAX_CONFIGURATION_NAME_LENGTH: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const MAX_INTERFACE_NAME_LENGTH: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const MAX_NUM_PIPES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const MAX_NUM_USBFN_ENDPOINTS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const MAX_SUPPORTED_CONFIGURATIONS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const MAX_USB_STRING_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const MS_GENRE_DESCRIPTOR_INDEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const MS_OS_FLAGS_CONTAINERID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const MS_OS_STRING_SIGNATURE: &str = "MSFT100";
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const MS_POWER_DESCRIPTOR_INDEX: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct OS_STRING {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub MicrosoftString: [u16; 7],
    pub bVendorCode: u8,
    pub Anonymous: OS_STRING_0,
}
impl ::core::marker::Copy for OS_STRING {}
impl ::core::clone::Clone for OS_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union OS_STRING_0 {
    pub bPad: u8,
    pub bFlags: u8,
}
impl ::core::marker::Copy for OS_STRING_0 {}
impl ::core::clone::Clone for OS_STRING_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const OS_STRING_DESCRIPTOR_INDEX: u32 = 238u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct PACKET_PARAMETERS {
    pub DeviceAddress: u8,
    pub EndpointAddress: u8,
    pub MaximumPacketSize: u16,
    pub Timeout: u32,
    pub Flags: u32,
    pub DataLength: u32,
    pub HubDeviceAddress: u16,
    pub PortTTNumber: u16,
    pub ErrorCount: u8,
    pub Pad: [u8; 3],
    pub UsbdStatusCode: i32,
    pub Data: [u8; 4],
}
impl ::core::marker::Copy for PACKET_PARAMETERS {}
impl ::core::clone::Clone for PACKET_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const PIPE_TRANSFER_TIMEOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub type PIPE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const EVENT_PIPE: PIPE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const READ_DATA_PIPE: PIPE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WRITE_DATA_PIPE: PIPE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const ALL_PIPE: PIPE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const PORT_LINK_STATE_COMPLIANCE_MODE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const PORT_LINK_STATE_DISABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const PORT_LINK_STATE_HOT_RESET: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const PORT_LINK_STATE_INACTIVE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const PORT_LINK_STATE_LOOPBACK: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const PORT_LINK_STATE_POLLING: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const PORT_LINK_STATE_RECOVERY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const PORT_LINK_STATE_RX_DETECT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const PORT_LINK_STATE_TEST_MODE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const PORT_LINK_STATE_U0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const PORT_LINK_STATE_U1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const PORT_LINK_STATE_U2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const PORT_LINK_STATE_U3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const RAW_IO: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub type RAW_PIPE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBSCAN_PIPE_CONTROL: RAW_PIPE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBSCAN_PIPE_ISOCHRONOUS: RAW_PIPE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBSCAN_PIPE_BULK: RAW_PIPE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBSCAN_PIPE_INTERRUPT: RAW_PIPE_TYPE = 3i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct RAW_RESET_PORT_PARAMETERS {
    pub PortNumber: u16,
    pub PortStatus: u16,
}
impl ::core::marker::Copy for RAW_RESET_PORT_PARAMETERS {}
impl ::core::clone::Clone for RAW_RESET_PORT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct RAW_ROOTPORT_FEATURE {
    pub PortNumber: u16,
    pub PortFeature: u16,
    pub PortStatus: u16,
}
impl ::core::marker::Copy for RAW_ROOTPORT_FEATURE {}
impl ::core::clone::Clone for RAW_ROOTPORT_FEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct RAW_ROOTPORT_PARAMETERS {
    pub PortNumber: u16,
    pub PortStatus: u16,
}
impl ::core::marker::Copy for RAW_ROOTPORT_PARAMETERS {}
impl ::core::clone::Clone for RAW_ROOTPORT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const RESET_PIPE_ON_RESUME: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const SHORT_PACKET_TERMINATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const SUSPEND_DELAY: u32 = 131u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct URB {
    pub Anonymous: URB_0,
}
impl ::core::marker::Copy for URB {}
impl ::core::clone::Clone for URB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union URB_0 {
    pub UrbHeader: _URB_HEADER,
    pub UrbSelectInterface: _URB_SELECT_INTERFACE,
    pub UrbSelectConfiguration: _URB_SELECT_CONFIGURATION,
    pub UrbPipeRequest: _URB_PIPE_REQUEST,
    pub UrbFrameLengthControl: _URB_FRAME_LENGTH_CONTROL,
    pub UrbGetFrameLength: _URB_GET_FRAME_LENGTH,
    pub UrbSetFrameLength: _URB_SET_FRAME_LENGTH,
    pub UrbGetCurrentFrameNumber: _URB_GET_CURRENT_FRAME_NUMBER,
    pub UrbControlTransfer: _URB_CONTROL_TRANSFER,
    pub UrbControlTransferEx: _URB_CONTROL_TRANSFER_EX,
    pub UrbBulkOrInterruptTransfer: _URB_BULK_OR_INTERRUPT_TRANSFER,
    pub UrbIsochronousTransfer: _URB_ISOCH_TRANSFER,
    pub UrbControlDescriptorRequest: _URB_CONTROL_DESCRIPTOR_REQUEST,
    pub UrbControlGetStatusRequest: _URB_CONTROL_GET_STATUS_REQUEST,
    pub UrbControlFeatureRequest: _URB_CONTROL_FEATURE_REQUEST,
    pub UrbControlVendorClassRequest: _URB_CONTROL_VENDOR_OR_CLASS_REQUEST,
    pub UrbControlGetInterfaceRequest: _URB_CONTROL_GET_INTERFACE_REQUEST,
    pub UrbControlGetConfigurationRequest: _URB_CONTROL_GET_CONFIGURATION_REQUEST,
    pub UrbOSFeatureDescriptorRequest: _URB_OS_FEATURE_DESCRIPTOR_REQUEST,
    pub UrbOpenStaticStreams: _URB_OPEN_STATIC_STREAMS,
    pub UrbGetIsochPipeTransferPathDelays: _URB_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS,
}
impl ::core::marker::Copy for URB_0 {}
impl ::core::clone::Clone for URB_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_ABORT_PIPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_BULK_OR_INTERRUPT_TRANSFER: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_BULK_OR_INTERRUPT_TRANSFER_USING_CHAINED_MDL: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_CLASS_DEVICE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_CLASS_ENDPOINT: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_CLASS_INTERFACE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_CLASS_OTHER: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_CLEAR_FEATURE_TO_DEVICE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_CLEAR_FEATURE_TO_ENDPOINT: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_CLEAR_FEATURE_TO_INTERFACE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_CLEAR_FEATURE_TO_OTHER: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_CLOSE_STATIC_STREAMS: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_CONTROL_TRANSFER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_CONTROL_TRANSFER_EX: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_GET_CONFIGURATION: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_GET_CURRENT_FRAME_NUMBER: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_GET_DESCRIPTOR_FROM_DEVICE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_GET_DESCRIPTOR_FROM_ENDPOINT: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_GET_DESCRIPTOR_FROM_INTERFACE: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_GET_FRAME_LENGTH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_GET_INTERFACE: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS: u32 = 61u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_GET_MS_FEATURE_DESCRIPTOR: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_GET_STATUS_FROM_DEVICE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_GET_STATUS_FROM_ENDPOINT: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_GET_STATUS_FROM_INTERFACE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_GET_STATUS_FROM_OTHER: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_ISOCH_TRANSFER: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_ISOCH_TRANSFER_USING_CHAINED_MDL: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_OPEN_STATIC_STREAMS: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_RELEASE_FRAME_LENGTH_CONTROL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_RESERVED_0X0016: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_RESERVE_0X001D: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_RESERVE_0X002B: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_RESERVE_0X002C: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_RESERVE_0X002D: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_RESERVE_0X002E: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_RESERVE_0X002F: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_RESERVE_0X0033: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_RESERVE_0X0034: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_RESET_PIPE: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_SELECT_CONFIGURATION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_SELECT_INTERFACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_SET_DESCRIPTOR_TO_DEVICE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_SET_DESCRIPTOR_TO_ENDPOINT: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_SET_DESCRIPTOR_TO_INTERFACE: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_SET_FEATURE_TO_DEVICE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_SET_FEATURE_TO_ENDPOINT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_SET_FEATURE_TO_INTERFACE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_SET_FEATURE_TO_OTHER: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_SET_FRAME_LENGTH: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_SYNC_CLEAR_STALL: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_SYNC_RESET_PIPE: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_SYNC_RESET_PIPE_AND_CLEAR_STALL: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_TAKE_FRAME_LENGTH_CONTROL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_VENDOR_DEVICE: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_VENDOR_ENDPOINT: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_VENDOR_INTERFACE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_FUNCTION_VENDOR_OTHER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const URB_OPEN_STATIC_STREAMS_VERSION_100: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UREGMANUSBFNENUMPATH: &str = "HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Control\\ManufacturingMode\\Current\\USBFN\\";
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UREGUSBFNENUMPATH: &str = "HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Control\\USBFN\\";
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBDI_VERSION: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_DEFAULT_MAXIMUM_TRANSFER_SIZE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_DEFAULT_PIPE_TRANSFER: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBD_DEVICE_INFORMATION {
    pub OffsetNext: u32,
    pub UsbdDeviceHandle: *mut ::core::ffi::c_void,
    pub DeviceDescriptor: USB_DEVICE_DESCRIPTOR,
}
impl ::core::marker::Copy for USBD_DEVICE_INFORMATION {}
impl ::core::clone::Clone for USBD_DEVICE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBD_ENDPOINT_OFFLOAD_INFORMATION {
    pub Size: u32,
    pub EndpointAddress: u16,
    pub ResourceId: u32,
    pub Mode: USBD_ENDPOINT_OFFLOAD_MODE,
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub TransferSegmentLA: i64,
    pub TransferSegmentVA: *mut ::core::ffi::c_void,
    pub TransferRingSize: usize,
    pub TransferRingInitialCycleBit: u32,
    pub MessageNumber: u32,
    pub EventRingSegmentLA: i64,
    pub EventRingSegmentVA: *mut ::core::ffi::c_void,
    pub EventRingSize: usize,
    pub EventRingInitialCycleBit: u32,
}
impl ::core::marker::Copy for USBD_ENDPOINT_OFFLOAD_INFORMATION {}
impl ::core::clone::Clone for USBD_ENDPOINT_OFFLOAD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub type USBD_ENDPOINT_OFFLOAD_MODE = i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbdEndpointOffloadModeNotSupported: USBD_ENDPOINT_OFFLOAD_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbdEndpointOffloadSoftwareAssisted: USBD_ENDPOINT_OFFLOAD_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbdEndpointOffloadHardwareAssisted: USBD_ENDPOINT_OFFLOAD_MODE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBD_INTERFACE_INFORMATION {
    pub Length: u16,
    pub InterfaceNumber: u8,
    pub AlternateSetting: u8,
    pub Class: u8,
    pub SubClass: u8,
    pub Protocol: u8,
    pub Reserved: u8,
    pub InterfaceHandle: *mut ::core::ffi::c_void,
    pub NumberOfPipes: u32,
    pub Pipes: [USBD_PIPE_INFORMATION; 1],
}
impl ::core::marker::Copy for USBD_INTERFACE_INFORMATION {}
impl ::core::clone::Clone for USBD_INTERFACE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBD_ISO_PACKET_DESCRIPTOR {
    pub Offset: u32,
    pub Length: u32,
    pub Status: i32,
}
impl ::core::marker::Copy for USBD_ISO_PACKET_DESCRIPTOR {}
impl ::core::clone::Clone for USBD_ISO_PACKET_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_ISO_START_FRAME_RANGE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_PF_CHANGE_MAX_PACKET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_PF_ENABLE_RT_THREAD_ACCESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_PF_HANDLES_SSP_HIGH_BANDWIDTH_ISOCH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_PF_INTERACTIVE_PRIORITY: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_PF_MAP_ADD_TRANSFERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_PF_PRIORITY_MASK: u32 = 240u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_PF_SHORT_PACKET_OPT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_PF_SSP_HIGH_BANDWIDTH_ISOCH: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_PF_VIDEO_PRIORITY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_PF_VOICE_PRIORITY: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBD_PIPE_INFORMATION {
    pub MaximumPacketSize: u16,
    pub EndpointAddress: u8,
    pub Interval: u8,
    pub PipeType: USBD_PIPE_TYPE,
    pub PipeHandle: *mut ::core::ffi::c_void,
    pub MaximumTransferSize: u32,
    pub PipeFlags: u32,
}
impl ::core::marker::Copy for USBD_PIPE_INFORMATION {}
impl ::core::clone::Clone for USBD_PIPE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub type USBD_PIPE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbdPipeTypeControl: USBD_PIPE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbdPipeTypeIsochronous: USBD_PIPE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbdPipeTypeBulk: USBD_PIPE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbdPipeTypeInterrupt: USBD_PIPE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_PORT_CONNECTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_PORT_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_SHORT_TRANSFER_OK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_START_ISO_TRANSFER_ASAP: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBD_STREAM_INFORMATION {
    pub PipeHandle: *mut ::core::ffi::c_void,
    pub StreamID: u32,
    pub MaximumTransferSize: u32,
    pub PipeFlags: u32,
}
impl ::core::marker::Copy for USBD_STREAM_INFORMATION {}
impl ::core::clone::Clone for USBD_STREAM_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_TRANSFER_DIRECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_TRANSFER_DIRECTION_IN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBD_TRANSFER_DIRECTION_OUT: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBD_VERSION_INFORMATION {
    pub USBDI_Version: u32,
    pub Supported_USB_Version: u32,
}
impl ::core::marker::Copy for USBD_VERSION_INFORMATION {}
impl ::core::clone::Clone for USBD_VERSION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USBFN_BUS_CONFIGURATION_INFO {
    pub ConfigurationName: [u16; 40],
    pub IsCurrent: super::super::Foundation::BOOLEAN,
    pub IsActive: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USBFN_BUS_CONFIGURATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USBFN_BUS_CONFIGURATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub type USBFN_BUS_SPEED = i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnBusSpeedLow: USBFN_BUS_SPEED = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnBusSpeedFull: USBFN_BUS_SPEED = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnBusSpeedHigh: USBFN_BUS_SPEED = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnBusSpeedSuper: USBFN_BUS_SPEED = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnBusSpeedMaximum: USBFN_BUS_SPEED = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USBFN_CLASS_INFORMATION_PACKET {
    pub FullSpeedClassInterface: USBFN_CLASS_INTERFACE,
    pub HighSpeedClassInterface: USBFN_CLASS_INTERFACE,
    pub InterfaceName: [u16; 40],
    pub InterfaceGuid: [u16; 39],
    pub HasInterfaceGuid: super::super::Foundation::BOOLEAN,
    pub SuperSpeedClassInterface: USBFN_CLASS_INTERFACE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USBFN_CLASS_INFORMATION_PACKET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USBFN_CLASS_INFORMATION_PACKET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USBFN_CLASS_INFORMATION_PACKET_EX {
    pub FullSpeedClassInterfaceEx: USBFN_CLASS_INTERFACE_EX,
    pub HighSpeedClassInterfaceEx: USBFN_CLASS_INTERFACE_EX,
    pub SuperSpeedClassInterfaceEx: USBFN_CLASS_INTERFACE_EX,
    pub InterfaceName: [u16; 40],
    pub InterfaceGuid: [u16; 39],
    pub HasInterfaceGuid: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USBFN_CLASS_INFORMATION_PACKET_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USBFN_CLASS_INFORMATION_PACKET_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBFN_CLASS_INTERFACE {
    pub InterfaceNumber: u8,
    pub PipeCount: u8,
    pub PipeArr: [USBFN_PIPE_INFORMATION; 16],
}
impl ::core::marker::Copy for USBFN_CLASS_INTERFACE {}
impl ::core::clone::Clone for USBFN_CLASS_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBFN_CLASS_INTERFACE_EX {
    pub BaseInterfaceNumber: u8,
    pub InterfaceCount: u8,
    pub PipeCount: u8,
    pub PipeArr: [USBFN_PIPE_INFORMATION; 16],
}
impl ::core::marker::Copy for USBFN_CLASS_INTERFACE_EX {}
impl ::core::clone::Clone for USBFN_CLASS_INTERFACE_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub type USBFN_DEVICE_STATE = i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnDeviceStateMinimum: USBFN_DEVICE_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnDeviceStateAttached: USBFN_DEVICE_STATE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnDeviceStateDefault: USBFN_DEVICE_STATE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnDeviceStateDetached: USBFN_DEVICE_STATE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnDeviceStateAddressed: USBFN_DEVICE_STATE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnDeviceStateConfigured: USBFN_DEVICE_STATE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnDeviceStateSuspended: USBFN_DEVICE_STATE = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnDeviceStateStateMaximum: USBFN_DEVICE_STATE = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub type USBFN_DIRECTION = i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnDirectionMinimum: USBFN_DIRECTION = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnDirectionIn: USBFN_DIRECTION = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnDirectionOut: USBFN_DIRECTION = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnDirectionTx: USBFN_DIRECTION = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnDirectionRx: USBFN_DIRECTION = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnDirectionMaximum: USBFN_DIRECTION = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub type USBFN_EVENT = i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnEventMinimum: USBFN_EVENT = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnEventAttach: USBFN_EVENT = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnEventReset: USBFN_EVENT = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnEventDetach: USBFN_EVENT = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnEventSuspend: USBFN_EVENT = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnEventResume: USBFN_EVENT = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnEventSetupPacket: USBFN_EVENT = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnEventConfigured: USBFN_EVENT = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnEventUnConfigured: USBFN_EVENT = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnEventPortType: USBFN_EVENT = 9i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnEventBusTearDown: USBFN_EVENT = 10i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnEventSetInterface: USBFN_EVENT = 11i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnEventMaximum: USBFN_EVENT = 12i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBFN_INTERFACE_INFO {
    pub InterfaceNumber: u8,
    pub Speed: USBFN_BUS_SPEED,
    pub Size: u16,
    pub InterfaceDescriptorSet: [u8; 1],
}
impl ::core::marker::Copy for USBFN_INTERFACE_INFO {}
impl ::core::clone::Clone for USBFN_INTERFACE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBFN_INTERRUPT_ENDPOINT_SIZE_NOT_UPDATEABLE_MASK: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBFN_NOTIFICATION {
    pub Event: USBFN_EVENT,
    pub u: USBFN_NOTIFICATION_0,
}
impl ::core::marker::Copy for USBFN_NOTIFICATION {}
impl ::core::clone::Clone for USBFN_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USBFN_NOTIFICATION_0 {
    pub BusSpeed: USBFN_BUS_SPEED,
    pub SetupPacket: USB_DEFAULT_PIPE_SETUP_PACKET,
    pub ConfigurationValue: u16,
    pub PortType: USBFN_PORT_TYPE,
    pub AlternateInterface: ALTERNATE_INTERFACE,
}
impl ::core::marker::Copy for USBFN_NOTIFICATION_0 {}
impl ::core::clone::Clone for USBFN_NOTIFICATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBFN_PIPE_INFORMATION {
    pub EpDesc: USB_ENDPOINT_DESCRIPTOR,
    pub PipeId: u32,
}
impl ::core::marker::Copy for USBFN_PIPE_INFORMATION {}
impl ::core::clone::Clone for USBFN_PIPE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub type USBFN_PORT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnUnknownPort: USBFN_PORT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnStandardDownstreamPort: USBFN_PORT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnChargingDownstreamPort: USBFN_PORT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnDedicatedChargingPort: USBFN_PORT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnInvalidDedicatedChargingPort: USBFN_PORT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnProprietaryDedicatedChargingPort: USBFN_PORT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbfnPortTypeMaximum: USBFN_PORT_TYPE = 6i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBFN_USB_STRING {
    pub StringIndex: u8,
    pub UsbString: [u16; 255],
}
impl ::core::marker::Copy for USBFN_USB_STRING {}
impl ::core::clone::Clone for USBFN_USB_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBSCAN_GET_DESCRIPTOR {
    pub DescriptorType: u8,
    pub Index: u8,
    pub LanguageId: u16,
}
impl ::core::marker::Copy for USBSCAN_GET_DESCRIPTOR {}
impl ::core::clone::Clone for USBSCAN_GET_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBSCAN_PIPE_CONFIGURATION {
    pub NumberOfPipes: u32,
    pub PipeInfo: [USBSCAN_PIPE_INFORMATION; 8],
}
impl ::core::marker::Copy for USBSCAN_PIPE_CONFIGURATION {}
impl ::core::clone::Clone for USBSCAN_PIPE_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBSCAN_PIPE_INFORMATION {
    pub MaximumPacketSize: u16,
    pub EndpointAddress: u8,
    pub Interval: u8,
    pub PipeType: RAW_PIPE_TYPE,
}
impl ::core::marker::Copy for USBSCAN_PIPE_INFORMATION {}
impl ::core::clone::Clone for USBSCAN_PIPE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBSCAN_TIMEOUT {
    pub TimeoutRead: u32,
    pub TimeoutWrite: u32,
    pub TimeoutEvent: u32,
}
impl ::core::marker::Copy for USBSCAN_TIMEOUT {}
impl ::core::clone::Clone for USBSCAN_TIMEOUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBUSER_BANDWIDTH_INFO_REQUEST {
    pub Header: USBUSER_REQUEST_HEADER,
    pub BandwidthInformation: USB_BANDWIDTH_INFO,
}
impl ::core::marker::Copy for USBUSER_BANDWIDTH_INFO_REQUEST {}
impl ::core::clone::Clone for USBUSER_BANDWIDTH_INFO_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USBUSER_BUS_STATISTICS_0_REQUEST {
    pub Header: USBUSER_REQUEST_HEADER,
    pub BusStatistics0: USB_BUS_STATISTICS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USBUSER_BUS_STATISTICS_0_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USBUSER_BUS_STATISTICS_0_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_CLEAR_ROOTPORT_FEATURE: u32 = 536870918u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBUSER_CLOSE_RAW_DEVICE {
    pub Header: USBUSER_REQUEST_HEADER,
    pub Parameters: USB_CLOSE_RAW_DEVICE_PARAMETERS,
}
impl ::core::marker::Copy for USBUSER_CLOSE_RAW_DEVICE {}
impl ::core::clone::Clone for USBUSER_CLOSE_RAW_DEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBUSER_CONTROLLER_INFO_0 {
    pub Header: USBUSER_REQUEST_HEADER,
    pub Info0: USB_CONTROLLER_INFO_0,
}
impl ::core::marker::Copy for USBUSER_CONTROLLER_INFO_0 {}
impl ::core::clone::Clone for USBUSER_CONTROLLER_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBUSER_CONTROLLER_UNICODE_NAME {
    pub Header: USBUSER_REQUEST_HEADER,
    pub UnicodeName: USB_UNICODE_NAME,
}
impl ::core::marker::Copy for USBUSER_CONTROLLER_UNICODE_NAME {}
impl ::core::clone::Clone for USBUSER_CONTROLLER_UNICODE_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_GET_BANDWIDTH_INFORMATION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_GET_BUS_STATISTICS_0: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_GET_CONTROLLER_DRIVER_KEY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_GET_CONTROLLER_INFO_0: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USBUSER_GET_DRIVER_VERSION {
    pub Header: USBUSER_REQUEST_HEADER,
    pub Parameters: USB_DRIVER_VERSION_PARAMETERS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USBUSER_GET_DRIVER_VERSION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USBUSER_GET_DRIVER_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_GET_POWER_STATE_MAP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_GET_ROOTHUB_SYMBOLIC_NAME: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_GET_ROOTPORT_STATUS: u32 = 536870919u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBUSER_GET_USB2HW_VERSION {
    pub Header: USBUSER_REQUEST_HEADER,
    pub Parameters: USB_USB2HW_VERSION_PARAMETERS,
}
impl ::core::marker::Copy for USBUSER_GET_USB2HW_VERSION {}
impl ::core::clone::Clone for USBUSER_GET_USB2HW_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_GET_USB2_HW_VERSION: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_GET_USB_DRIVER_VERSION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_INVALID_REQUEST: u32 = 4294967280u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBUSER_OPEN_RAW_DEVICE {
    pub Header: USBUSER_REQUEST_HEADER,
    pub Parameters: USB_OPEN_RAW_DEVICE_PARAMETERS,
}
impl ::core::marker::Copy for USBUSER_OPEN_RAW_DEVICE {}
impl ::core::clone::Clone for USBUSER_OPEN_RAW_DEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_OP_CLOSE_RAW_DEVICE: u32 = 536870915u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_OP_MASK_DEVONLY_API: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_OP_MASK_HCTEST_API: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_OP_OPEN_RAW_DEVICE: u32 = 536870914u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_OP_RAW_RESET_PORT: u32 = 536870913u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_OP_SEND_ONE_PACKET: u32 = 268435457u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_OP_SEND_RAW_COMMAND: u32 = 536870916u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_PASS_THRU: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBUSER_PASS_THRU_REQUEST {
    pub Header: USBUSER_REQUEST_HEADER,
    pub PassThru: USB_PASS_THRU_PARAMETERS,
}
impl ::core::marker::Copy for USBUSER_PASS_THRU_REQUEST {}
impl ::core::clone::Clone for USBUSER_PASS_THRU_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USBUSER_POWER_INFO_REQUEST {
    pub Header: USBUSER_REQUEST_HEADER,
    pub PowerInformation: USB_POWER_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USBUSER_POWER_INFO_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USBUSER_POWER_INFO_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBUSER_RAW_RESET_ROOT_PORT {
    pub Header: USBUSER_REQUEST_HEADER,
    pub Parameters: RAW_RESET_PORT_PARAMETERS,
}
impl ::core::marker::Copy for USBUSER_RAW_RESET_ROOT_PORT {}
impl ::core::clone::Clone for USBUSER_RAW_RESET_ROOT_PORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBUSER_REFRESH_HCT_REG {
    pub Header: USBUSER_REQUEST_HEADER,
    pub Flags: u32,
}
impl ::core::marker::Copy for USBUSER_REFRESH_HCT_REG {}
impl ::core::clone::Clone for USBUSER_REFRESH_HCT_REG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBUSER_REQUEST_HEADER {
    pub UsbUserRequest: u32,
    pub UsbUserStatusCode: USB_USER_ERROR_CODE,
    pub RequestBufferLength: u32,
    pub ActualBufferLength: u32,
}
impl ::core::marker::Copy for USBUSER_REQUEST_HEADER {}
impl ::core::clone::Clone for USBUSER_REQUEST_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBUSER_ROOTPORT_FEATURE_REQUEST {
    pub Header: USBUSER_REQUEST_HEADER,
    pub Parameters: RAW_ROOTPORT_FEATURE,
}
impl ::core::marker::Copy for USBUSER_ROOTPORT_FEATURE_REQUEST {}
impl ::core::clone::Clone for USBUSER_ROOTPORT_FEATURE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBUSER_ROOTPORT_PARAMETERS {
    pub Header: USBUSER_REQUEST_HEADER,
    pub Parameters: RAW_ROOTPORT_PARAMETERS,
}
impl ::core::marker::Copy for USBUSER_ROOTPORT_PARAMETERS {}
impl ::core::clone::Clone for USBUSER_ROOTPORT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBUSER_SEND_ONE_PACKET {
    pub Header: USBUSER_REQUEST_HEADER,
    pub PacketParameters: PACKET_PARAMETERS,
}
impl ::core::marker::Copy for USBUSER_SEND_ONE_PACKET {}
impl ::core::clone::Clone for USBUSER_SEND_ONE_PACKET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USBUSER_SEND_RAW_COMMAND {
    pub Header: USBUSER_REQUEST_HEADER,
    pub Parameters: USB_SEND_RAW_COMMAND_PARAMETERS,
}
impl ::core::marker::Copy for USBUSER_SEND_RAW_COMMAND {}
impl ::core::clone::Clone for USBUSER_SEND_RAW_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_SET_ROOTPORT_FEATURE: u32 = 536870917u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_USB_REFRESH_HCT_REG: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USBUSER_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_20_ENDPOINT_TYPE_INTERRUPT_RESERVED_MASK: u32 = 252u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_20_HUB_DESCRIPTOR_TYPE: u32 = 41u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_20_PORT_CHANGE {
    pub AsUshort16: u16,
    pub Anonymous: USB_20_PORT_CHANGE_0,
}
impl ::core::marker::Copy for USB_20_PORT_CHANGE {}
impl ::core::clone::Clone for USB_20_PORT_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_20_PORT_CHANGE_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for USB_20_PORT_CHANGE_0 {}
impl ::core::clone::Clone for USB_20_PORT_CHANGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_20_PORT_STATUS {
    pub AsUshort16: u16,
    pub Anonymous: USB_20_PORT_STATUS_0,
}
impl ::core::marker::Copy for USB_20_PORT_STATUS {}
impl ::core::clone::Clone for USB_20_PORT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_20_PORT_STATUS_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for USB_20_PORT_STATUS_0 {}
impl ::core::clone::Clone for USB_20_PORT_STATUS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_RESERVED_MASK: u32 = 204u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_MASK: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_NOTIFICATION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_PERIODIC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_RESERVED10: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_30_ENDPOINT_TYPE_INTERRUPT_USAGE_RESERVED11: u32 = 48u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_30_HUB_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bNumberOfPorts: u8,
    pub wHubCharacteristics: u16,
    pub bPowerOnToPowerGood: u8,
    pub bHubControlCurrent: u8,
    pub bHubHdrDecLat: u8,
    pub wHubDelay: u16,
    pub DeviceRemovable: u16,
}
impl ::core::marker::Copy for USB_30_HUB_DESCRIPTOR {}
impl ::core::clone::Clone for USB_30_HUB_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_30_HUB_DESCRIPTOR_TYPE: u32 = 42u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_30_PORT_CHANGE {
    pub AsUshort16: u16,
    pub Anonymous: USB_30_PORT_CHANGE_0,
}
impl ::core::marker::Copy for USB_30_PORT_CHANGE {}
impl ::core::clone::Clone for USB_30_PORT_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_30_PORT_CHANGE_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for USB_30_PORT_CHANGE_0 {}
impl ::core::clone::Clone for USB_30_PORT_CHANGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_30_PORT_STATUS {
    pub AsUshort16: u16,
    pub Anonymous: USB_30_PORT_STATUS_0,
}
impl ::core::marker::Copy for USB_30_PORT_STATUS {}
impl ::core::clone::Clone for USB_30_PORT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_30_PORT_STATUS_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for USB_30_PORT_STATUS_0 {}
impl ::core::clone::Clone for USB_30_PORT_STATUS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ALLOW_FIRMWARE_UPDATE: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_BANDWIDTH_INFO {
    pub DeviceCount: u32,
    pub TotalBusBandwidth: u32,
    pub Total32secBandwidth: u32,
    pub AllocedBulkAndControl: u32,
    pub AllocedIso: u32,
    pub AllocedInterrupt_1ms: u32,
    pub AllocedInterrupt_2ms: u32,
    pub AllocedInterrupt_4ms: u32,
    pub AllocedInterrupt_8ms: u32,
    pub AllocedInterrupt_16ms: u32,
    pub AllocedInterrupt_32ms: u32,
}
impl ::core::marker::Copy for USB_BANDWIDTH_INFO {}
impl ::core::clone::Clone for USB_BANDWIDTH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_BOS_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wTotalLength: u16,
    pub bNumDeviceCaps: u8,
}
impl ::core::marker::Copy for USB_BOS_DESCRIPTOR {}
impl ::core::clone::Clone for USB_BOS_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_BOS_DESCRIPTOR_TYPE: u32 = 15u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USB_BUS_STATISTICS_0 {
    pub DeviceCount: u32,
    pub CurrentSystemTime: i64,
    pub CurrentUsbFrame: u32,
    pub BulkBytes: u32,
    pub IsoBytes: u32,
    pub InterruptBytes: u32,
    pub ControlDataBytes: u32,
    pub PciInterruptCount: u32,
    pub HardResetCount: u32,
    pub WorkerSignalCount: u32,
    pub CommonBufferBytes: u32,
    pub WorkerIdleTimeMs: u32,
    pub RootHubEnabled: super::super::Foundation::BOOLEAN,
    pub RootHubDevicePowerState: u8,
    pub Unused: u8,
    pub NameIndex: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USB_BUS_STATISTICS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USB_BUS_STATISTICS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_CHARGING_POLICY_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_CHARGING_POLICY_ICCHPF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_CHARGING_POLICY_ICCLPF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_CHARGING_POLICY_NO_POWER: u32 = 3u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_CLOSE_RAW_DEVICE_PARAMETERS {
    pub xxx: u32,
}
impl ::core::marker::Copy for USB_CLOSE_RAW_DEVICE_PARAMETERS {}
impl ::core::clone::Clone for USB_CLOSE_RAW_DEVICE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_COMMON_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
}
impl ::core::marker::Copy for USB_COMMON_DESCRIPTOR {}
impl ::core::clone::Clone for USB_COMMON_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_CONFIGURATION_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wTotalLength: u16,
    pub bNumInterfaces: u8,
    pub bConfigurationValue: u8,
    pub iConfiguration: u8,
    pub bmAttributes: u8,
    pub MaxPower: u8,
}
impl ::core::marker::Copy for USB_CONFIGURATION_DESCRIPTOR {}
impl ::core::clone::Clone for USB_CONFIGURATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_CONFIGURATION_DESCRIPTOR_TYPE: u32 = 2u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_CONFIGURATION_POWER_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub SelfPowerConsumedD0: [u8; 3],
    pub bPowerSummaryId: u8,
    pub bBusPowerSavingD1: u8,
    pub bSelfPowerSavingD1: u8,
    pub bBusPowerSavingD2: u8,
    pub bSelfPowerSavingD2: u8,
    pub bBusPowerSavingD3: u8,
    pub bSelfPowerSavingD3: u8,
    pub TransitionTimeFromD1: u16,
    pub TransitionTimeFromD2: u16,
    pub TransitionTimeFromD3: u16,
}
impl ::core::marker::Copy for USB_CONFIGURATION_POWER_DESCRIPTOR {}
impl ::core::clone::Clone for USB_CONFIGURATION_POWER_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_CONFIG_BUS_POWERED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_CONFIG_POWERED_MASK: u32 = 192u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_CONFIG_POWER_DESCRIPTOR_TYPE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_CONFIG_REMOTE_WAKEUP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_CONFIG_RESERVED: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_CONFIG_SELF_POWERED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub type USB_CONTROLLER_FLAVOR = i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_HcGeneric: USB_CONTROLLER_FLAVOR = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const OHCI_Generic: USB_CONTROLLER_FLAVOR = 100i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const OHCI_Hydra: USB_CONTROLLER_FLAVOR = 101i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const OHCI_NEC: USB_CONTROLLER_FLAVOR = 102i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_Generic: USB_CONTROLLER_FLAVOR = 200i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_Piix4: USB_CONTROLLER_FLAVOR = 201i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_Piix3: USB_CONTROLLER_FLAVOR = 202i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_Ich2: USB_CONTROLLER_FLAVOR = 203i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_Reserved204: USB_CONTROLLER_FLAVOR = 204i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_Ich1: USB_CONTROLLER_FLAVOR = 205i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_Ich3m: USB_CONTROLLER_FLAVOR = 206i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_Ich4: USB_CONTROLLER_FLAVOR = 207i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_Ich5: USB_CONTROLLER_FLAVOR = 208i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_Ich6: USB_CONTROLLER_FLAVOR = 209i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_Intel: USB_CONTROLLER_FLAVOR = 249i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_VIA: USB_CONTROLLER_FLAVOR = 250i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_VIA_x01: USB_CONTROLLER_FLAVOR = 251i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_VIA_x02: USB_CONTROLLER_FLAVOR = 252i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_VIA_x03: USB_CONTROLLER_FLAVOR = 253i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_VIA_x04: USB_CONTROLLER_FLAVOR = 254i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UHCI_VIA_x0E_FIFO: USB_CONTROLLER_FLAVOR = 264i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const EHCI_Generic: USB_CONTROLLER_FLAVOR = 1000i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const EHCI_NEC: USB_CONTROLLER_FLAVOR = 2000i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const EHCI_Lucent: USB_CONTROLLER_FLAVOR = 3000i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const EHCI_NVIDIA_Tegra2: USB_CONTROLLER_FLAVOR = 4000i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const EHCI_NVIDIA_Tegra3: USB_CONTROLLER_FLAVOR = 4001i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const EHCI_Intel_Medfield: USB_CONTROLLER_FLAVOR = 5001i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_CONTROLLER_INFO_0 {
    pub PciVendorId: u32,
    pub PciDeviceId: u32,
    pub PciRevision: u32,
    pub NumberOfRootPorts: u32,
    pub ControllerFlavor: USB_CONTROLLER_FLAVOR,
    pub HcFeatureFlags: u32,
}
impl ::core::marker::Copy for USB_CONTROLLER_INFO_0 {}
impl ::core::clone::Clone for USB_CONTROLLER_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_CYCLE_PORT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEBUG_DESCRIPTOR_TYPE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEFAULT_DEVICE_ADDRESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEFAULT_ENDPOINT_ADDRESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEFAULT_MAX_PACKET: u32 = 64u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEFAULT_PIPE_SETUP_PACKET {
    pub bmRequestType: BM_REQUEST_TYPE,
    pub bRequest: u8,
    pub wValue: USB_DEFAULT_PIPE_SETUP_PACKET_1,
    pub wIndex: USB_DEFAULT_PIPE_SETUP_PACKET_0,
    pub wLength: u16,
}
impl ::core::marker::Copy for USB_DEFAULT_PIPE_SETUP_PACKET {}
impl ::core::clone::Clone for USB_DEFAULT_PIPE_SETUP_PACKET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_DEFAULT_PIPE_SETUP_PACKET_0 {
    pub Anonymous: USB_DEFAULT_PIPE_SETUP_PACKET_0_0,
    pub W: u16,
}
impl ::core::marker::Copy for USB_DEFAULT_PIPE_SETUP_PACKET_0 {}
impl ::core::clone::Clone for USB_DEFAULT_PIPE_SETUP_PACKET_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEFAULT_PIPE_SETUP_PACKET_0_0 {
    pub LowByte: u8,
    pub HiByte: u8,
}
impl ::core::marker::Copy for USB_DEFAULT_PIPE_SETUP_PACKET_0_0 {}
impl ::core::clone::Clone for USB_DEFAULT_PIPE_SETUP_PACKET_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_DEFAULT_PIPE_SETUP_PACKET_1 {
    pub Anonymous: USB_DEFAULT_PIPE_SETUP_PACKET_1_0,
    pub W: u16,
}
impl ::core::marker::Copy for USB_DEFAULT_PIPE_SETUP_PACKET_1 {}
impl ::core::clone::Clone for USB_DEFAULT_PIPE_SETUP_PACKET_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEFAULT_PIPE_SETUP_PACKET_1_0 {
    pub LowByte: u8,
    pub HiByte: u8,
}
impl ::core::marker::Copy for USB_DEFAULT_PIPE_SETUP_PACKET_1_0 {}
impl ::core::clone::Clone for USB_DEFAULT_PIPE_SETUP_PACKET_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_BATTERY_INFO: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_BILLBOARD: u32 = 13u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub iAddtionalInfoURL: u8,
    pub bNumberOfAlternateModes: u8,
    pub bPreferredAlternateMode: u8,
    pub VconnPower: USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_1,
    pub bmConfigured: [u8; 32],
    pub bReserved: u32,
    pub AlternateMode: [USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_0; 1],
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_0 {
    pub wSVID: u16,
    pub bAlternateMode: u8,
    pub iAlternateModeSetting: u8,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_0 {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_1 {
    pub AsUshort: u16,
    pub Anonymous: USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_1_0,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_1 {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_1_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_1_0 {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_CONTAINER_ID: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bReserved: u8,
    pub ContainerID: [u8; 16],
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_DESCRIPTOR {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_DESCRIPTOR_TYPE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_FIRMWARE_STATUS: u32 = 17u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bcdDescriptorVersion: u8,
    pub bmAttributes: USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR_0,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR_0 {
    pub AsUlong: u32,
    pub Anonymous: USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR_0_0,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR_0 {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR_0_0 {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_MAX_U1_LATENCY: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_MAX_U2_LATENCY: u32 = 2047u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT: u32 = 8u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bReserved: u8,
    pub bmCapabilities: USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_0,
    pub wMinVoltage: u16,
    pub wMaxVoltage: u16,
    pub wReserved: u16,
    pub dwMaxOperatingPower: u32,
    pub dwMaxPeakPower: u32,
    pub dwMaxPeakPowerTime: u32,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_0 {
    pub AsUshort: u16,
    pub Anonymous: USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_0_0,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_0 {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_0_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_0_0 {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_PD_PROVIDER_PORT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_PLATFORM: u32 = 5u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bReserved: u8,
    pub PlatformCapabilityUuid: ::windows_sys::core::GUID,
    pub CapabililityData: [u8; 1],
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_POWER_DELIVERY: u32 = 6u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bReserved: u8,
    pub bmAttributes: USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_0,
    pub bmProviderPorts: u16,
    pub bmConsumerPorts: u16,
    pub bcdBCVersion: u16,
    pub bcdPDVersion: u16,
    pub bcdUSBTypeCVersion: u16,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_0 {
    pub AsUlong: u32,
    pub Anonymous: USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_0_0,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_0 {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_0_0 {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_PRECISION_TIME_MEASUREMENT: u32 = 11u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED {
    pub AsUlong32: u32,
    pub Anonymous: USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_0,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_0 {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_DIR_RX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_DIR_TX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_BPS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_GBPS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_KBPS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_LSE_MBPS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_MODE_ASYMMETRIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_MODE_SYMMETRIC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_PROTOCOL_SS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED_PROTOCOL_SSP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB: u32 = 10u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bReserved: u8,
    pub bmAttributes: USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_0,
    pub wFunctionalitySupport: USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_1,
    pub wReserved: u16,
    pub bmSublinkSpeedAttr: [USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED; 1],
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_0 {
    pub AsUlong: u32,
    pub Anonymous: USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_0_0,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_0 {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_0_0 {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_1 {
    pub AsUshort: u16,
    pub Anonymous: USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_1_0,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_1 {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_1_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_1_0 {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_BMATTRIBUTES_LTM_CAPABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_BMATTRIBUTES_RESERVED_MASK: u32 = 253u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_FULL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_HIGH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_LOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_RESERVED_MASK: u32 = 65520u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_SPEEDS_SUPPORTED_SUPER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_U1_DEVICE_EXIT_MAX_VALUE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_U2_DEVICE_EXIT_MAX_VALUE: u32 = 2047u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_SUPERSPEED_USB: u32 = 3u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bmAttributes: u8,
    pub wSpeedsSupported: u16,
    pub bFunctionalitySupport: u8,
    pub bU1DevExitLat: u8,
    pub wU2DevExitLat: u16,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_USB20_EXTENSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_USB20_EXTENSION_BMATTRIBUTES_RESERVED_MASK: u32 = 4294901985u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bmAttributes: USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_0,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_0 {
    pub AsUlong: u32,
    pub Anonymous: USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_0_0,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_0 {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_0_0 {}
impl ::core::clone::Clone for USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CAPABILITY_WIRELESS_USB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CHARACTERISTICS_MAXIMUM_PATH_DELAYS_AVAILABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CHARACTERISTICS_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_APPLICATION_SPECIFIC: u32 = 254u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_AUDIO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_AUDIO_VIDEO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_BILLBOARD: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_CDC_DATA: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_COMMUNICATIONS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_CONTENT_SECURITY: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_DIAGNOSTIC_DEVICE: u32 = 220u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_HUB: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_HUMAN_INTERFACE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_IMAGE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_MISCELLANEOUS: u32 = 239u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_MONITOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_PERSONAL_HEALTHCARE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_PHYSICAL_INTERFACE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_POWER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_PRINTER: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_RESERVED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_SMART_CARD: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_STORAGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_VENDOR_SPECIFIC: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_VIDEO: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_CLASS_WIRELESS_CONTROLLER: u32 = 224u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdUSB: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bMaxPacketSize0: u8,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub iManufacturer: u8,
    pub iProduct: u8,
    pub iSerialNumber: u8,
    pub bNumConfigurations: u8,
}
impl ::core::marker::Copy for USB_DEVICE_DESCRIPTOR {}
impl ::core::clone::Clone for USB_DEVICE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_DESCRIPTOR_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_FIRMWARE_HASH_LENGTH: u32 = 32u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_QUALIFIER_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdUSB: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bMaxPacketSize0: u8,
    pub bNumConfigurations: u8,
    pub bReserved: u8,
}
impl ::core::marker::Copy for USB_DEVICE_QUALIFIER_DESCRIPTOR {}
impl ::core::clone::Clone for USB_DEVICE_QUALIFIER_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DEVICE_QUALIFIER_DESCRIPTOR_TYPE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub type USB_DEVICE_SPEED = i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbLowSpeed: USB_DEVICE_SPEED = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbFullSpeed: USB_DEVICE_SPEED = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbHighSpeed: USB_DEVICE_SPEED = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbSuperSpeed: USB_DEVICE_SPEED = 3i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_DEVICE_STATUS {
    pub AsUshort16: u16,
    pub Anonymous: USB_DEVICE_STATUS_0,
}
impl ::core::marker::Copy for USB_DEVICE_STATUS {}
impl ::core::clone::Clone for USB_DEVICE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_DEVICE_STATUS_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for USB_DEVICE_STATUS_0 {}
impl ::core::clone::Clone for USB_DEVICE_STATUS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub type USB_DEVICE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const Usb11Device: USB_DEVICE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const Usb20Device: USB_DEVICE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DIAG_IGNORE_HUBS_OFF: u32 = 263u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DIAG_IGNORE_HUBS_ON: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_DISALLOW_FIRMWARE_UPDATE: u32 = 0u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USB_DRIVER_VERSION_PARAMETERS {
    pub DriverTrackingCode: u32,
    pub USBDI_Version: u32,
    pub USBUSER_Version: u32,
    pub CheckedPortDriver: super::super::Foundation::BOOLEAN,
    pub CheckedMiniportDriver: super::super::Foundation::BOOLEAN,
    pub USB_Version: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USB_DRIVER_VERSION_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USB_DRIVER_VERSION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENABLE_PORT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_ADDRESS_MASK: u32 = 15u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_ENDPOINT_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bEndpointAddress: u8,
    pub bmAttributes: u8,
    pub wMaxPacketSize: u16,
    pub bInterval: u8,
}
impl ::core::marker::Copy for USB_ENDPOINT_DESCRIPTOR {}
impl ::core::clone::Clone for USB_ENDPOINT_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_DESCRIPTOR_TYPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_DIRECTION_MASK: u32 = 128u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_ENDPOINT_STATUS {
    pub AsUshort16: u16,
    pub Anonymous: USB_ENDPOINT_STATUS_0,
}
impl ::core::marker::Copy for USB_ENDPOINT_STATUS {}
impl ::core::clone::Clone for USB_ENDPOINT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_ENDPOINT_STATUS_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for USB_ENDPOINT_STATUS_0 {}
impl ::core::clone::Clone for USB_ENDPOINT_STATUS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_SUPERSPEED_BULK_MAX_PACKET_SIZE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_SUPERSPEED_CONTROL_MAX_PACKET_SIZE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_SUPERSPEED_INTERRUPT_MAX_PACKET_SIZE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_SUPERSPEED_ISO_MAX_PACKET_SIZE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_BULK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_BULK_RESERVED_MASK: u32 = 252u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_CONTROL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_CONTROL_RESERVED_MASK: u32 = 252u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_INTERRUPT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_RESERVED_MASK: u32 = 192u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_ADAPTIVE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_ASYNCHRONOUS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_MASK: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_NO_SYNCHRONIZATION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_SYNCHRONIZATION_SYNCHRONOUS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_DATA_ENDOINT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_FEEDBACK_ENDPOINT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_IMPLICIT_FEEDBACK_DATA_ENDPOINT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_MASK: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS_USAGE_RESERVED: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_ENDPOINT_TYPE_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FAIL_GET_STATUS: u32 = 280u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FEATURE_BATTERY_WAKE_MASK: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FEATURE_CHARGING_POLICY: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FEATURE_ENDPOINT_STALL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FEATURE_FUNCTION_SUSPEND: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FEATURE_INTERFACE_POWER_D0: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FEATURE_INTERFACE_POWER_D1: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FEATURE_INTERFACE_POWER_D2: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FEATURE_INTERFACE_POWER_D3: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FEATURE_LDM_ENABLE: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FEATURE_LTM_ENABLE: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FEATURE_OS_IS_PD_AWARE: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FEATURE_POLICY_MODE: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FEATURE_REMOTE_WAKEUP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FEATURE_TEST_MODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FEATURE_U1_ENABLE: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_FEATURE_U2_ENABLE: u32 = 49u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION {
    pub TimeTrackingHandle: super::super::Foundation::HANDLE,
    pub InputFrameNumber: u32,
    pub InputMicroFrameNumber: u32,
    pub QueryPerformanceCounterAtInputFrameOrMicroFrame: i64,
    pub QueryPerformanceCounterFrequency: i64,
    pub PredictedAccuracyInMicroSeconds: u32,
    pub CurrentGenerationID: u32,
    pub CurrentQueryPerformanceCounter: i64,
    pub CurrentHardwareFrameNumber: u32,
    pub CurrentHardwareMicroFrameNumber: u32,
    pub CurrentUSBFrameNumber: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_FUNCTION_SUSPEND_OPTIONS {
    pub AsUchar: u8,
    pub Anonymous: USB_FUNCTION_SUSPEND_OPTIONS_0,
}
impl ::core::marker::Copy for USB_FUNCTION_SUSPEND_OPTIONS {}
impl ::core::clone::Clone for USB_FUNCTION_SUSPEND_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_FUNCTION_SUSPEND_OPTIONS_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for USB_FUNCTION_SUSPEND_OPTIONS_0 {}
impl ::core::clone::Clone for USB_FUNCTION_SUSPEND_OPTIONS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GETSTATUS_LTM_ENABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GETSTATUS_REMOTE_WAKEUP_ENABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GETSTATUS_SELF_POWERED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GETSTATUS_U1_ENABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GETSTATUS_U2_ENABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_BUSGUID_INFO: u32 = 266u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_BUS_INFO: u32 = 264u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_CONTROLLER_NAME: u32 = 265u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_DESCRIPTOR_FROM_NODE_CONNECTION: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_DEVICE_CHARACTERISTICS: u32 = 288u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_DEVICE_HANDLE: u32 = 268u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_DEVICE_HANDLE_EX: u32 = 269u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_FIRMWARE_ALLOWED_OR_DISALLOWED_STATE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_FIRMWARE_HASH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC: u32 = 286u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_HUB_CAPABILITIES: u32 = 271u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_HUB_CAPABILITIES_EX: u32 = 276u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_HUB_CONFIG_INFO: u32 = 275u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_HUB_COUNT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_HUB_INFORMATION_EX: u32 = 277u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_HUB_NAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_NODE_CONNECTION_ATTRIBUTES: u32 = 272u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_NODE_CONNECTION_DRIVERKEY_NAME: u32 = 264u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_NODE_CONNECTION_INFORMATION: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_NODE_CONNECTION_INFORMATION_EX: u32 = 274u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_NODE_CONNECTION_INFORMATION_EX_V2: u32 = 279u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_NODE_CONNECTION_NAME: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_NODE_INFORMATION: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_PARENT_HUB_INFO: u32 = 267u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_PORT_CONNECTOR_PROPERTIES: u32 = 278u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_PORT_STATUS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_ROOTHUB_PDO: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_TOPOLOGY_ADDRESS: u32 = 271u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_TRANSPORT_CHARACTERISTICS: u32 = 281u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_GET_TT_DEVICE_HANDLE: u32 = 270u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_HC_FEATURE_FLAG_PORT_POWER_SWITCHING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_HC_FEATURE_FLAG_SEL_SUSPEND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_HC_FEATURE_LEGACY_BIOS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_HC_FEATURE_TIME_SYNC_API: u32 = 8u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_HIGH_SPEED_MAXPACKET {
    pub us: u16,
}
impl ::core::marker::Copy for USB_HIGH_SPEED_MAXPACKET {}
impl ::core::clone::Clone for USB_HIGH_SPEED_MAXPACKET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_HIGH_SPEED_MAXPACKET_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for USB_HIGH_SPEED_MAXPACKET_0 {}
impl ::core::clone::Clone for USB_HIGH_SPEED_MAXPACKET_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_HUB_30_PORT_REMOTE_WAKE_MASK {
    pub AsUchar8: u8,
    pub Anonymous: USB_HUB_30_PORT_REMOTE_WAKE_MASK_0,
}
impl ::core::marker::Copy for USB_HUB_30_PORT_REMOTE_WAKE_MASK {}
impl ::core::clone::Clone for USB_HUB_30_PORT_REMOTE_WAKE_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_HUB_30_PORT_REMOTE_WAKE_MASK_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for USB_HUB_30_PORT_REMOTE_WAKE_MASK_0 {}
impl ::core::clone::Clone for USB_HUB_30_PORT_REMOTE_WAKE_MASK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_HUB_CHANGE {
    pub AsUshort16: u16,
    pub Anonymous: USB_HUB_CHANGE_0,
}
impl ::core::marker::Copy for USB_HUB_CHANGE {}
impl ::core::clone::Clone for USB_HUB_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_HUB_CHANGE_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for USB_HUB_CHANGE_0 {}
impl ::core::clone::Clone for USB_HUB_CHANGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_HUB_CYCLE_PORT: u32 = 273u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_HUB_DESCRIPTOR {
    pub bDescriptorLength: u8,
    pub bDescriptorType: u8,
    pub bNumberOfPorts: u8,
    pub wHubCharacteristics: u16,
    pub bPowerOnToPowerGood: u8,
    pub bHubControlCurrent: u8,
    pub bRemoveAndPowerMask: [u8; 64],
}
impl ::core::marker::Copy for USB_HUB_DESCRIPTOR {}
impl ::core::clone::Clone for USB_HUB_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_HUB_STATUS {
    pub AsUshort16: u16,
    pub Anonymous: USB_HUB_STATUS_0,
}
impl ::core::marker::Copy for USB_HUB_STATUS {}
impl ::core::clone::Clone for USB_HUB_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_HUB_STATUS_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for USB_HUB_STATUS_0 {}
impl ::core::clone::Clone for USB_HUB_STATUS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_HUB_STATUS_AND_CHANGE {
    pub AsUlong32: u32,
    pub Anonymous: USB_HUB_STATUS_AND_CHANGE_0,
}
impl ::core::marker::Copy for USB_HUB_STATUS_AND_CHANGE {}
impl ::core::clone::Clone for USB_HUB_STATUS_AND_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_HUB_STATUS_AND_CHANGE_0 {
    pub HubStatus: USB_HUB_STATUS,
    pub HubChange: USB_HUB_CHANGE,
}
impl ::core::marker::Copy for USB_HUB_STATUS_AND_CHANGE_0 {}
impl ::core::clone::Clone for USB_HUB_STATUS_AND_CHANGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub type USB_IDLE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_IDLE_CALLBACK_INFO {
    pub IdleCallback: USB_IDLE_CALLBACK,
    pub IdleContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for USB_IDLE_CALLBACK_INFO {}
impl ::core::clone::Clone for USB_IDLE_CALLBACK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_IDLE_NOTIFICATION: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_IDLE_NOTIFICATION_EX: u32 = 272u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_INTERFACE_ASSOCIATION_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bFirstInterface: u8,
    pub bInterfaceCount: u8,
    pub bFunctionClass: u8,
    pub bFunctionSubClass: u8,
    pub bFunctionProtocol: u8,
    pub iFunction: u8,
}
impl ::core::marker::Copy for USB_INTERFACE_ASSOCIATION_DESCRIPTOR {}
impl ::core::clone::Clone for USB_INTERFACE_ASSOCIATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_INTERFACE_ASSOCIATION_DESCRIPTOR_TYPE: u32 = 11u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_INTERFACE_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bInterfaceNumber: u8,
    pub bAlternateSetting: u8,
    pub bNumEndpoints: u8,
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    pub iInterface: u8,
}
impl ::core::marker::Copy for USB_INTERFACE_DESCRIPTOR {}
impl ::core::clone::Clone for USB_INTERFACE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_INTERFACE_DESCRIPTOR_TYPE: u32 = 4u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_INTERFACE_POWER_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bmCapabilitiesFlags: u8,
    pub bBusPowerSavingD1: u8,
    pub bSelfPowerSavingD1: u8,
    pub bBusPowerSavingD2: u8,
    pub bSelfPowerSavingD2: u8,
    pub bBusPowerSavingD3: u8,
    pub bSelfPowerSavingD3: u8,
    pub TransitionTimeFromD1: u16,
    pub TransitionTimeFromD2: u16,
    pub TransitionTimeFromD3: u16,
}
impl ::core::marker::Copy for USB_INTERFACE_POWER_DESCRIPTOR {}
impl ::core::clone::Clone for USB_INTERFACE_POWER_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_INTERFACE_POWER_DESCRIPTOR_TYPE: u32 = 8u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_INTERFACE_STATUS {
    pub AsUshort16: u16,
    pub Anonymous: USB_INTERFACE_STATUS_0,
}
impl ::core::marker::Copy for USB_INTERFACE_STATUS {}
impl ::core::clone::Clone for USB_INTERFACE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_INTERFACE_STATUS_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for USB_INTERFACE_STATUS_0 {}
impl ::core::clone::Clone for USB_INTERFACE_STATUS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_NOTIFY_ON_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 283u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_OPEN_RAW_DEVICE_PARAMETERS {
    pub PortStatus: u16,
    pub MaxPacketEp0: u16,
}
impl ::core::marker::Copy for USB_OPEN_RAW_DEVICE_PARAMETERS {}
impl ::core::clone::Clone for USB_OPEN_RAW_DEVICE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_OTG_DESCRIPTOR_TYPE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_OTHER_SPEED_CONFIGURATION_DESCRIPTOR_TYPE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PACKETFLAG_ASYNC_IN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PACKETFLAG_ASYNC_OUT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PACKETFLAG_FULL_SPEED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PACKETFLAG_HIGH_SPEED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PACKETFLAG_ISO_IN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PACKETFLAG_ISO_OUT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PACKETFLAG_LOW_SPEED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PACKETFLAG_SETUP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PACKETFLAG_TOGGLE0: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PACKETFLAG_TOGGLE1: u32 = 512u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_PASS_THRU_PARAMETERS {
    pub FunctionGUID: ::windows_sys::core::GUID,
    pub ParameterLength: u32,
    pub Parameters: [u8; 4],
}
impl ::core::marker::Copy for USB_PASS_THRU_PARAMETERS {}
impl ::core::clone::Clone for USB_PASS_THRU_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PORTATTR_MINI_CONNECTOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PORTATTR_NO_CONNECTOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PORTATTR_NO_OVERCURRENT_UI: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PORTATTR_OEM_CONNECTOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PORTATTR_OWNED_BY_CC: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PORTATTR_SHARED_USB2: u32 = 2u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_PORT_CHANGE {
    pub AsUshort16: u16,
    pub Usb20PortChange: USB_20_PORT_CHANGE,
    pub Usb30PortChange: USB_30_PORT_CHANGE,
}
impl ::core::marker::Copy for USB_PORT_CHANGE {}
impl ::core::clone::Clone for USB_PORT_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_PORT_EXT_STATUS {
    pub AsUlong32: u32,
    pub Anonymous: USB_PORT_EXT_STATUS_0,
}
impl ::core::marker::Copy for USB_PORT_EXT_STATUS {}
impl ::core::clone::Clone for USB_PORT_EXT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_PORT_EXT_STATUS_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for USB_PORT_EXT_STATUS_0 {}
impl ::core::clone::Clone for USB_PORT_EXT_STATUS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_PORT_EXT_STATUS_AND_CHANGE {
    pub AsUlong64: u64,
    pub Anonymous: USB_PORT_EXT_STATUS_AND_CHANGE_0,
}
impl ::core::marker::Copy for USB_PORT_EXT_STATUS_AND_CHANGE {}
impl ::core::clone::Clone for USB_PORT_EXT_STATUS_AND_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_PORT_EXT_STATUS_AND_CHANGE_0 {
    pub PortStatusChange: USB_PORT_STATUS_AND_CHANGE,
    pub PortExtStatus: USB_PORT_EXT_STATUS,
}
impl ::core::marker::Copy for USB_PORT_EXT_STATUS_AND_CHANGE_0 {}
impl ::core::clone::Clone for USB_PORT_EXT_STATUS_AND_CHANGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_PORT_STATUS {
    pub AsUshort16: u16,
    pub Usb20PortStatus: USB_20_PORT_STATUS,
    pub Usb30PortStatus: USB_30_PORT_STATUS,
}
impl ::core::marker::Copy for USB_PORT_STATUS {}
impl ::core::clone::Clone for USB_PORT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_PORT_STATUS_AND_CHANGE {
    pub AsUlong32: u32,
    pub Anonymous: USB_PORT_STATUS_AND_CHANGE_0,
}
impl ::core::marker::Copy for USB_PORT_STATUS_AND_CHANGE {}
impl ::core::clone::Clone for USB_PORT_STATUS_AND_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_PORT_STATUS_AND_CHANGE_0 {
    pub PortStatus: USB_PORT_STATUS,
    pub PortChange: USB_PORT_CHANGE,
}
impl ::core::marker::Copy for USB_PORT_STATUS_AND_CHANGE_0 {}
impl ::core::clone::Clone for USB_PORT_STATUS_AND_CHANGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PORT_STATUS_CONNECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PORT_STATUS_ENABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PORT_STATUS_HIGH_SPEED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PORT_STATUS_LOW_SPEED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PORT_STATUS_OVER_CURRENT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PORT_STATUS_POWER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PORT_STATUS_RESET: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_PORT_STATUS_SUSPEND: u32 = 4u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USB_POWER_INFO {
    pub SystemState: WDMUSB_POWER_STATE,
    pub HcDevicePowerState: WDMUSB_POWER_STATE,
    pub HcDeviceWake: WDMUSB_POWER_STATE,
    pub HcSystemWake: WDMUSB_POWER_STATE,
    pub RhDevicePowerState: WDMUSB_POWER_STATE,
    pub RhDeviceWake: WDMUSB_POWER_STATE,
    pub RhSystemWake: WDMUSB_POWER_STATE,
    pub LastSystemSleepState: WDMUSB_POWER_STATE,
    pub CanWakeup: super::super::Foundation::BOOLEAN,
    pub IsPowered: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USB_POWER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USB_POWER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_RECORD_FAILURE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REGISTER_COMPOSITE_DEVICE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REGISTER_FOR_TRANSPORT_BANDWIDTH_CHANGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 282u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REGISTER_FOR_TRANSPORT_LATENCY_CHANGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_CLEAR_FEATURE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_CLEAR_TT_BUFFER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_GET_CONFIGURATION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_GET_DESCRIPTOR: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_GET_FIRMWARE_STATUS: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_GET_INTERFACE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_GET_PORT_ERR_COUNT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_GET_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_GET_STATUS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_GET_TT_STATE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_ISOCH_DELAY: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_REMOTE_WAKE_NOTIFICATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_RESET_TT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_SET_ADDRESS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_SET_CONFIGURATION: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_SET_DESCRIPTOR: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_SET_FEATURE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_SET_FIRMWARE_STATUS: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_SET_HUB_DEPTH: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_SET_INTERFACE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_SET_SEL: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_STOP_TT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQUEST_SYNC_FRAME: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQ_GLOBAL_RESUME: u32 = 274u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_REQ_GLOBAL_SUSPEND: u32 = 273u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_RESERVED_DESCRIPTOR_TYPE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_RESET_HUB: u32 = 275u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_RESET_PORT: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_SEND_RAW_COMMAND_PARAMETERS {
    pub Usb_bmRequest: u8,
    pub Usb_bRequest: u8,
    pub Usb_wVlaue: u16,
    pub Usb_wIndex: u16,
    pub Usb_wLength: u16,
    pub DeviceAddress: u16,
    pub MaximumPacketSize: u16,
    pub Timeout: u32,
    pub DataLength: u32,
    pub UsbdStatusCode: i32,
    pub Data: [u8; 4],
}
impl ::core::marker::Copy for USB_SEND_RAW_COMMAND_PARAMETERS {}
impl ::core::clone::Clone for USB_SEND_RAW_COMMAND_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_START_TRACKING_FOR_TIME_SYNC: u32 = 285u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION {
    pub TimeTrackingHandle: super::super::Foundation::HANDLE,
    pub IsStartupDelayTolerable: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_STATUS_EXT_PORT_STATUS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_STATUS_PD_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_STATUS_PORT_STATUS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_STOP_TRACKING_FOR_TIME_SYNC: u32 = 287u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION {
    pub TimeTrackingHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_STRING_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bString: [u16; 1],
}
impl ::core::marker::Copy for USB_STRING_DESCRIPTOR {}
impl ::core::clone::Clone for USB_STRING_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_STRING_DESCRIPTOR_TYPE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_SUBMIT_URB: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_SUPERSPEEDPLUS_ISOCHRONOUS_MAX_BYTESPERINTERVAL: u32 = 16777215u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_SUPERSPEEDPLUS_ISOCHRONOUS_MIN_BYTESPERINTERVAL: u32 = 49153u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wReserved: u16,
    pub dwBytesPerInterval: u32,
}
impl ::core::marker::Copy for USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR {}
impl ::core::clone::Clone for USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR_TYPE: u32 = 49u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bMaxBurst: u8,
    pub bmAttributes: USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0,
    pub wBytesPerInterval: u16,
}
impl ::core::marker::Copy for USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR {}
impl ::core::clone::Clone for USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub union USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0 {
    pub AsUchar: u8,
    pub Bulk: USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0_0,
    pub Isochronous: USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0_1,
}
impl ::core::marker::Copy for USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0 {}
impl ::core::clone::Clone for USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0_0 {}
impl ::core::clone::Clone for USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0_1 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0_1 {}
impl ::core::clone::Clone for USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR_TYPE: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_SUPERSPEED_ISOCHRONOUS_MAX_MULTIPLIER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_SUPPORT_D0_COMMAND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_SUPPORT_D1_COMMAND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_SUPPORT_D1_WAKEUP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_SUPPORT_D2_COMMAND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_SUPPORT_D2_WAKEUP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_SUPPORT_D3_COMMAND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_TEST_MODE_TEST_FORCE_ENABLE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_TEST_MODE_TEST_J: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_TEST_MODE_TEST_K: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_TEST_MODE_TEST_PACKET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_TEST_MODE_TEST_SE0_NAK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_TRANSPORT_CHARACTERISTICS_BANDWIDTH_AVAILABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_TRANSPORT_CHARACTERISTICS_LATENCY_AVAILABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_TRANSPORT_CHARACTERISTICS_VERSION_1: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_UNICODE_NAME {
    pub Length: u32,
    pub String: [u16; 1],
}
impl ::core::marker::Copy for USB_UNICODE_NAME {}
impl ::core::clone::Clone for USB_UNICODE_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_UNREGISTER_COMPOSITE_DEVICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const USB_UNREGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE: u32 = 284u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct USB_USB2HW_VERSION_PARAMETERS {
    pub Usb2HwRevision: u8,
}
impl ::core::marker::Copy for USB_USB2HW_VERSION_PARAMETERS {}
impl ::core::clone::Clone for USB_USB2HW_VERSION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub type USB_USER_ERROR_CODE = i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbUserSuccess: USB_USER_ERROR_CODE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbUserNotSupported: USB_USER_ERROR_CODE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbUserInvalidRequestCode: USB_USER_ERROR_CODE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbUserFeatureDisabled: USB_USER_ERROR_CODE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbUserInvalidHeaderParameter: USB_USER_ERROR_CODE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbUserInvalidParameter: USB_USER_ERROR_CODE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbUserMiniportError: USB_USER_ERROR_CODE = 6i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbUserBufferTooSmall: USB_USER_ERROR_CODE = 7i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbUserErrorNotMapped: USB_USER_ERROR_CODE = 8i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbUserDeviceNotStarted: USB_USER_ERROR_CODE = 9i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const UsbUserNoDeviceConnected: USB_USER_ERROR_CODE = 10i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub type WDMUSB_POWER_STATE = i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WdmUsbPowerNotMapped: WDMUSB_POWER_STATE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WdmUsbPowerSystemUnspecified: WDMUSB_POWER_STATE = 100i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WdmUsbPowerSystemWorking: WDMUSB_POWER_STATE = 101i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WdmUsbPowerSystemSleeping1: WDMUSB_POWER_STATE = 102i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WdmUsbPowerSystemSleeping2: WDMUSB_POWER_STATE = 103i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WdmUsbPowerSystemSleeping3: WDMUSB_POWER_STATE = 104i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WdmUsbPowerSystemHibernate: WDMUSB_POWER_STATE = 105i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WdmUsbPowerSystemShutdown: WDMUSB_POWER_STATE = 106i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WdmUsbPowerDeviceUnspecified: WDMUSB_POWER_STATE = 200i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WdmUsbPowerDeviceD0: WDMUSB_POWER_STATE = 201i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WdmUsbPowerDeviceD1: WDMUSB_POWER_STATE = 202i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WdmUsbPowerDeviceD2: WDMUSB_POWER_STATE = 203i32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WdmUsbPowerDeviceD3: WDMUSB_POWER_STATE = 204i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct WINUSB_PIPE_INFORMATION {
    pub PipeType: USBD_PIPE_TYPE,
    pub PipeId: u8,
    pub MaximumPacketSize: u16,
    pub Interval: u8,
}
impl ::core::marker::Copy for WINUSB_PIPE_INFORMATION {}
impl ::core::clone::Clone for WINUSB_PIPE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct WINUSB_PIPE_INFORMATION_EX {
    pub PipeType: USBD_PIPE_TYPE,
    pub PipeId: u8,
    pub MaximumPacketSize: u16,
    pub Interval: u8,
    pub MaximumBytesPerInterval: u32,
}
impl ::core::marker::Copy for WINUSB_PIPE_INFORMATION_EX {}
impl ::core::clone::Clone for WINUSB_PIPE_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct WINUSB_SETUP_PACKET {
    pub RequestType: u8,
    pub Request: u8,
    pub Value: u16,
    pub Index: u16,
    pub Length: u16,
}
impl ::core::marker::Copy for WINUSB_SETUP_PACKET {}
impl ::core::clone::Clone for WINUSB_SETUP_PACKET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WMI_USB_DEVICE_NODE_INFORMATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WMI_USB_DRIVER_INFORMATION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WMI_USB_DRIVER_NOTIFICATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WMI_USB_HUB_NODE_INFORMATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WMI_USB_PERFORMANCE_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub const WMI_USB_POWER_DEVICE_ENABLE: u32 = 2u32;
pub const WinUSB_TestGuid: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3665898495, data2: 4803, data3: 18082, data4: [142, 43, 219, 211, 183, 131, 76, 67] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_BULK_OR_INTERRUPT_TRANSFER {
    pub Hdr: _URB_HEADER,
    pub PipeHandle: *mut ::core::ffi::c_void,
    pub TransferFlags: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut ::core::ffi::c_void,
    pub TransferBufferMDL: *mut ::core::ffi::c_void,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
}
impl ::core::marker::Copy for _URB_BULK_OR_INTERRUPT_TRANSFER {}
impl ::core::clone::Clone for _URB_BULK_OR_INTERRUPT_TRANSFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_CONTROL_DESCRIPTOR_REQUEST {
    pub Hdr: _URB_HEADER,
    pub Reserved: *mut ::core::ffi::c_void,
    pub Reserved0: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut ::core::ffi::c_void,
    pub TransferBufferMDL: *mut ::core::ffi::c_void,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub Reserved1: u16,
    pub Index: u8,
    pub DescriptorType: u8,
    pub LanguageId: u16,
    pub Reserved2: u16,
}
impl ::core::marker::Copy for _URB_CONTROL_DESCRIPTOR_REQUEST {}
impl ::core::clone::Clone for _URB_CONTROL_DESCRIPTOR_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_CONTROL_FEATURE_REQUEST {
    pub Hdr: _URB_HEADER,
    pub Reserved: *mut ::core::ffi::c_void,
    pub Reserved2: u32,
    pub Reserved3: u32,
    pub Reserved4: *mut ::core::ffi::c_void,
    pub Reserved5: *mut ::core::ffi::c_void,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub Reserved0: u16,
    pub FeatureSelector: u16,
    pub Index: u16,
    pub Reserved1: u16,
}
impl ::core::marker::Copy for _URB_CONTROL_FEATURE_REQUEST {}
impl ::core::clone::Clone for _URB_CONTROL_FEATURE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_CONTROL_GET_CONFIGURATION_REQUEST {
    pub Hdr: _URB_HEADER,
    pub Reserved: *mut ::core::ffi::c_void,
    pub Reserved0: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut ::core::ffi::c_void,
    pub TransferBufferMDL: *mut ::core::ffi::c_void,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub Reserved1: [u8; 8],
}
impl ::core::marker::Copy for _URB_CONTROL_GET_CONFIGURATION_REQUEST {}
impl ::core::clone::Clone for _URB_CONTROL_GET_CONFIGURATION_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_CONTROL_GET_INTERFACE_REQUEST {
    pub Hdr: _URB_HEADER,
    pub Reserved: *mut ::core::ffi::c_void,
    pub Reserved0: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut ::core::ffi::c_void,
    pub TransferBufferMDL: *mut ::core::ffi::c_void,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub Reserved1: [u8; 4],
    pub Interface: u16,
    pub Reserved2: u16,
}
impl ::core::marker::Copy for _URB_CONTROL_GET_INTERFACE_REQUEST {}
impl ::core::clone::Clone for _URB_CONTROL_GET_INTERFACE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_CONTROL_GET_STATUS_REQUEST {
    pub Hdr: _URB_HEADER,
    pub Reserved: *mut ::core::ffi::c_void,
    pub Reserved0: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut ::core::ffi::c_void,
    pub TransferBufferMDL: *mut ::core::ffi::c_void,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub Reserved1: [u8; 4],
    pub Index: u16,
    pub Reserved2: u16,
}
impl ::core::marker::Copy for _URB_CONTROL_GET_STATUS_REQUEST {}
impl ::core::clone::Clone for _URB_CONTROL_GET_STATUS_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_CONTROL_TRANSFER {
    pub Hdr: _URB_HEADER,
    pub PipeHandle: *mut ::core::ffi::c_void,
    pub TransferFlags: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut ::core::ffi::c_void,
    pub TransferBufferMDL: *mut ::core::ffi::c_void,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub SetupPacket: [u8; 8],
}
impl ::core::marker::Copy for _URB_CONTROL_TRANSFER {}
impl ::core::clone::Clone for _URB_CONTROL_TRANSFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_CONTROL_TRANSFER_EX {
    pub Hdr: _URB_HEADER,
    pub PipeHandle: *mut ::core::ffi::c_void,
    pub TransferFlags: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut ::core::ffi::c_void,
    pub TransferBufferMDL: *mut ::core::ffi::c_void,
    pub Timeout: u32,
    pub hca: _URB_HCD_AREA,
    pub SetupPacket: [u8; 8],
}
impl ::core::marker::Copy for _URB_CONTROL_TRANSFER_EX {}
impl ::core::clone::Clone for _URB_CONTROL_TRANSFER_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_CONTROL_VENDOR_OR_CLASS_REQUEST {
    pub Hdr: _URB_HEADER,
    pub Reserved: *mut ::core::ffi::c_void,
    pub TransferFlags: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut ::core::ffi::c_void,
    pub TransferBufferMDL: *mut ::core::ffi::c_void,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub RequestTypeReservedBits: u8,
    pub Request: u8,
    pub Value: u16,
    pub Index: u16,
    pub Reserved1: u16,
}
impl ::core::marker::Copy for _URB_CONTROL_VENDOR_OR_CLASS_REQUEST {}
impl ::core::clone::Clone for _URB_CONTROL_VENDOR_OR_CLASS_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_FRAME_LENGTH_CONTROL {
    pub Hdr: _URB_HEADER,
}
impl ::core::marker::Copy for _URB_FRAME_LENGTH_CONTROL {}
impl ::core::clone::Clone for _URB_FRAME_LENGTH_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_GET_CURRENT_FRAME_NUMBER {
    pub Hdr: _URB_HEADER,
    pub FrameNumber: u32,
}
impl ::core::marker::Copy for _URB_GET_CURRENT_FRAME_NUMBER {}
impl ::core::clone::Clone for _URB_GET_CURRENT_FRAME_NUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_GET_FRAME_LENGTH {
    pub Hdr: _URB_HEADER,
    pub FrameLength: u32,
    pub FrameNumber: u32,
}
impl ::core::marker::Copy for _URB_GET_FRAME_LENGTH {}
impl ::core::clone::Clone for _URB_GET_FRAME_LENGTH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS {
    pub Hdr: _URB_HEADER,
    pub PipeHandle: *mut ::core::ffi::c_void,
    pub MaximumSendPathDelayInMilliSeconds: u32,
    pub MaximumCompletionPathDelayInMilliSeconds: u32,
}
impl ::core::marker::Copy for _URB_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS {}
impl ::core::clone::Clone for _URB_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_HCD_AREA {
    pub Reserved8: [*mut ::core::ffi::c_void; 8],
}
impl ::core::marker::Copy for _URB_HCD_AREA {}
impl ::core::clone::Clone for _URB_HCD_AREA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_HEADER {
    pub Length: u16,
    pub Function: u16,
    pub Status: i32,
    pub UsbdDeviceHandle: *mut ::core::ffi::c_void,
    pub UsbdFlags: u32,
}
impl ::core::marker::Copy for _URB_HEADER {}
impl ::core::clone::Clone for _URB_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_ISOCH_TRANSFER {
    pub Hdr: _URB_HEADER,
    pub PipeHandle: *mut ::core::ffi::c_void,
    pub TransferFlags: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut ::core::ffi::c_void,
    pub TransferBufferMDL: *mut ::core::ffi::c_void,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub StartFrame: u32,
    pub NumberOfPackets: u32,
    pub ErrorCount: u32,
    pub IsoPacket: [USBD_ISO_PACKET_DESCRIPTOR; 1],
}
impl ::core::marker::Copy for _URB_ISOCH_TRANSFER {}
impl ::core::clone::Clone for _URB_ISOCH_TRANSFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_OPEN_STATIC_STREAMS {
    pub Hdr: _URB_HEADER,
    pub PipeHandle: *mut ::core::ffi::c_void,
    pub NumberOfStreams: u32,
    pub StreamInfoVersion: u16,
    pub StreamInfoSize: u16,
    pub Streams: *mut USBD_STREAM_INFORMATION,
}
impl ::core::marker::Copy for _URB_OPEN_STATIC_STREAMS {}
impl ::core::clone::Clone for _URB_OPEN_STATIC_STREAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_OS_FEATURE_DESCRIPTOR_REQUEST {
    pub Hdr: _URB_HEADER,
    pub Reserved: *mut ::core::ffi::c_void,
    pub Reserved0: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut ::core::ffi::c_void,
    pub TransferBufferMDL: *mut ::core::ffi::c_void,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub _bitfield: u8,
    pub Reserved2: u8,
    pub InterfaceNumber: u8,
    pub MS_PageIndex: u8,
    pub MS_FeatureDescriptorIndex: u16,
    pub Reserved3: u16,
}
impl ::core::marker::Copy for _URB_OS_FEATURE_DESCRIPTOR_REQUEST {}
impl ::core::clone::Clone for _URB_OS_FEATURE_DESCRIPTOR_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_PIPE_REQUEST {
    pub Hdr: _URB_HEADER,
    pub PipeHandle: *mut ::core::ffi::c_void,
    pub Reserved: u32,
}
impl ::core::marker::Copy for _URB_PIPE_REQUEST {}
impl ::core::clone::Clone for _URB_PIPE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_SELECT_CONFIGURATION {
    pub Hdr: _URB_HEADER,
    pub ConfigurationDescriptor: *mut USB_CONFIGURATION_DESCRIPTOR,
    pub ConfigurationHandle: *mut ::core::ffi::c_void,
    pub Interface: USBD_INTERFACE_INFORMATION,
}
impl ::core::marker::Copy for _URB_SELECT_CONFIGURATION {}
impl ::core::clone::Clone for _URB_SELECT_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_SELECT_INTERFACE {
    pub Hdr: _URB_HEADER,
    pub ConfigurationHandle: *mut ::core::ffi::c_void,
    pub Interface: USBD_INTERFACE_INFORMATION,
}
impl ::core::marker::Copy for _URB_SELECT_INTERFACE {}
impl ::core::clone::Clone for _URB_SELECT_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Usb\"`*"]
pub struct _URB_SET_FRAME_LENGTH {
    pub Hdr: _URB_HEADER,
    pub FrameLengthDelta: i32,
}
impl ::core::marker::Copy for _URB_SET_FRAME_LENGTH {}
impl ::core::clone::Clone for _URB_SET_FRAME_LENGTH {
    fn clone(&self) -> Self {
        *self
    }
}
