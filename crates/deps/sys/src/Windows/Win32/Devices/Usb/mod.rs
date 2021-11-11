#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_AbortPipe(interfacehandle: *const ::core::ffi::c_void, pipeid: u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_ControlTransfer(interfacehandle: *const ::core::ffi::c_void, setuppacket: WINUSB_SETUP_PACKET, buffer: *mut u8, bufferlength: u32, lengthtransferred: *mut u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_FlushPipe(interfacehandle: *const ::core::ffi::c_void, pipeid: u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_Free(interfacehandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetAdjustedFrameNumber(currentframenumber: *mut u32, timestamp: i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetAssociatedInterface(interfacehandle: *const ::core::ffi::c_void, associatedinterfaceindex: u8, associatedinterfacehandle: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetCurrentAlternateSetting(interfacehandle: *const ::core::ffi::c_void, settingnumber: *mut u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetCurrentFrameNumber(interfacehandle: *const ::core::ffi::c_void, currentframenumber: *mut u32, timestamp: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetCurrentFrameNumberAndQpc(interfacehandle: *const ::core::ffi::c_void, frameqpcinfo: *const USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetDescriptor(interfacehandle: *const ::core::ffi::c_void, descriptortype: u8, index: u8, languageid: u16, buffer: *mut u8, bufferlength: u32, lengthtransferred: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_GetOverlappedResult(interfacehandle: *const ::core::ffi::c_void, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetPipePolicy(interfacehandle: *const ::core::ffi::c_void, pipeid: u8, policytype: u32, valuelength: *mut u32, value: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_GetPowerPolicy(interfacehandle: *const ::core::ffi::c_void, policytype: u32, valuelength: *mut u32, value: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_Initialize(devicehandle: super::super::Foundation::HANDLE, interfacehandle: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`*"]
    pub fn WinUsb_ParseConfigurationDescriptor(configurationdescriptor: *const USB_CONFIGURATION_DESCRIPTOR, startposition: *const ::core::ffi::c_void, interfacenumber: i32, alternatesetting: i32, interfaceclass: i32, interfacesubclass: i32, interfaceprotocol: i32) -> *mut USB_INTERFACE_DESCRIPTOR;
    #[doc = "*Required features: `Win32_Devices_Usb`*"]
    pub fn WinUsb_ParseDescriptors(descriptorbuffer: *const ::core::ffi::c_void, totallength: u32, startposition: *const ::core::ffi::c_void, descriptortype: i32) -> *mut USB_COMMON_DESCRIPTOR;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_QueryDeviceInformation(interfacehandle: *const ::core::ffi::c_void, informationtype: u32, bufferlength: *mut u32, buffer: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_QueryInterfaceSettings(interfacehandle: *const ::core::ffi::c_void, alternateinterfacenumber: u8, usbaltinterfacedescriptor: *mut USB_INTERFACE_DESCRIPTOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_QueryPipe(interfacehandle: *const ::core::ffi::c_void, alternateinterfacenumber: u8, pipeindex: u8, pipeinformation: *mut WINUSB_PIPE_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_QueryPipeEx(interfacehandle: *const ::core::ffi::c_void, alternatesettingnumber: u8, pipeindex: u8, pipeinformationex: *mut WINUSB_PIPE_INFORMATION_EX) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_ReadIsochPipe(bufferhandle: *const ::core::ffi::c_void, offset: u32, length: u32, framenumber: *mut u32, numberofpackets: u32, isopacketdescriptors: *mut USBD_ISO_PACKET_DESCRIPTOR, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_ReadIsochPipeAsap(bufferhandle: *const ::core::ffi::c_void, offset: u32, length: u32, continuestream: super::super::Foundation::BOOL, numberofpackets: u32, isopacketdescriptors: *mut USBD_ISO_PACKET_DESCRIPTOR, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_ReadPipe(interfacehandle: *const ::core::ffi::c_void, pipeid: u8, buffer: *mut u8, bufferlength: u32, lengthtransferred: *mut u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_RegisterIsochBuffer(interfacehandle: *const ::core::ffi::c_void, pipeid: u8, buffer: *mut u8, bufferlength: u32, isochbufferhandle: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_ResetPipe(interfacehandle: *const ::core::ffi::c_void, pipeid: u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_SetCurrentAlternateSetting(interfacehandle: *const ::core::ffi::c_void, settingnumber: u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_SetPipePolicy(interfacehandle: *const ::core::ffi::c_void, pipeid: u8, policytype: u32, valuelength: u32, value: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_SetPowerPolicy(interfacehandle: *const ::core::ffi::c_void, policytype: u32, valuelength: u32, value: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_StartTrackingForTimeSync(interfacehandle: *const ::core::ffi::c_void, starttrackinginfo: *const USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_StopTrackingForTimeSync(interfacehandle: *const ::core::ffi::c_void, stoptrackinginfo: *const USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinUsb_UnregisterIsochBuffer(isochbufferhandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_WriteIsochPipe(bufferhandle: *const ::core::ffi::c_void, offset: u32, length: u32, framenumber: *mut u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_WriteIsochPipeAsap(bufferhandle: *const ::core::ffi::c_void, offset: u32, length: u32, continuestream: super::super::Foundation::BOOL, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Usb`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WinUsb_WritePipe(interfacehandle: *const ::core::ffi::c_void, pipeid: u8, buffer: *const u8, bufferlength: u32, lengthtransferred: *mut u32, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
}
