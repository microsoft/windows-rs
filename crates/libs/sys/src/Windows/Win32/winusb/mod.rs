windows_link::link!("winusb.dll" "system" fn WinUsb_AbortPipe(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : u8) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_ControlTransfer(interfacehandle : WINUSB_INTERFACE_HANDLE, setuppacket : WINUSB_SETUP_PACKET, buffer : *mut u8, bufferlength : u32, lengthtransferred : *mut u32, overlapped : *const super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_FlushPipe(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : u8) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_Free(interfacehandle : WINUSB_INTERFACE_HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_GetAdjustedFrameNumber(currentframenumber : *mut u32, timestamp : i64) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_GetAssociatedInterface(interfacehandle : WINUSB_INTERFACE_HANDLE, associatedinterfaceindex : u8, associatedinterfacehandle : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_GetCurrentAlternateSetting(interfacehandle : WINUSB_INTERFACE_HANDLE, settingnumber : *mut u8) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_GetCurrentFrameNumber(interfacehandle : WINUSB_INTERFACE_HANDLE, currentframenumber : *mut u32, timestamp : *mut i64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_usbioctl", feature = "Win32_winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_GetCurrentFrameNumberAndQpc(interfacehandle : WINUSB_INTERFACE_HANDLE, frameqpcinfo : *const super::usbioctl::USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_GetDescriptor(interfacehandle : WINUSB_INTERFACE_HANDLE, descriptortype : u8, index : u8, languageid : u16, buffer : *mut u8, bufferlength : u32, lengthtransferred : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_GetOverlappedResult(interfacehandle : WINUSB_INTERFACE_HANDLE, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpnumberofbytestransferred : *mut u32, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_GetPipePolicy(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : u8, policytype : u32, valuelength : *mut u32, value : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_GetPowerPolicy(interfacehandle : WINUSB_INTERFACE_HANDLE, policytype : u32, valuelength : *mut u32, value : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("winusb.dll" "system" fn WinUsb_Initialize(devicehandle : super::winnt::HANDLE, interfacehandle : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_usbspec")]
windows_link::link!("winusb.dll" "system" fn WinUsb_ParseConfigurationDescriptor(configurationdescriptor : *const super::usbspec::USB_CONFIGURATION_DESCRIPTOR, startposition : *const core::ffi::c_void, interfacenumber : i32, alternatesetting : i32, interfaceclass : i32, interfacesubclass : i32, interfaceprotocol : i32) -> super::usbspec::PUSB_INTERFACE_DESCRIPTOR);
#[cfg(feature = "Win32_usbspec")]
windows_link::link!("winusb.dll" "system" fn WinUsb_ParseDescriptors(descriptorbuffer : *const core::ffi::c_void, totallength : u32, startposition : *const core::ffi::c_void, descriptortype : i32) -> super::usbspec::PUSB_COMMON_DESCRIPTOR);
windows_link::link!("winusb.dll" "system" fn WinUsb_QueryDeviceInformation(interfacehandle : WINUSB_INTERFACE_HANDLE, informationtype : u32, bufferlength : *mut u32, buffer : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_usbspec")]
windows_link::link!("winusb.dll" "system" fn WinUsb_QueryInterfaceSettings(interfacehandle : WINUSB_INTERFACE_HANDLE, alternateinterfacenumber : u8, usbaltinterfacedescriptor : *mut super::usbspec::USB_INTERFACE_DESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_usb", feature = "Win32_winusbio"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_QueryPipe(interfacehandle : WINUSB_INTERFACE_HANDLE, alternateinterfacenumber : u8, pipeindex : u8, pipeinformation : *mut super::winusbio::WINUSB_PIPE_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_usb", feature = "Win32_winusbio"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_QueryPipeEx(interfacehandle : WINUSB_INTERFACE_HANDLE, alternatesettingnumber : u8, pipeindex : u8, pipeinformationex : *mut super::winusbio::WINUSB_PIPE_INFORMATION_EX) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_usb", feature = "Win32_winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_ReadIsochPipe(bufferhandle : WINUSB_ISOCH_BUFFER_HANDLE, offset : u32, length : u32, framenumber : *mut u32, numberofpackets : u32, isopacketdescriptors : *mut super::usb::USBD_ISO_PACKET_DESCRIPTOR, overlapped : *const super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_usb", feature = "Win32_winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_ReadIsochPipeAsap(bufferhandle : WINUSB_ISOCH_BUFFER_HANDLE, offset : u32, length : u32, continuestream : windows_sys::core::BOOL, numberofpackets : u32, isopacketdescriptors : *mut super::usb::USBD_ISO_PACKET_DESCRIPTOR, overlapped : *const super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_ReadPipe(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : u8, buffer : *mut u8, bufferlength : u32, lengthtransferred : *mut u32, overlapped : *const super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_RegisterIsochBuffer(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : u8, buffer : *mut u8, bufferlength : u32, isochbufferhandle : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_ResetPipe(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : u8) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_SetCurrentAlternateSetting(interfacehandle : WINUSB_INTERFACE_HANDLE, settingnumber : u8) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_SetPipePolicy(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : u8, policytype : u32, valuelength : u32, value : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_SetPowerPolicy(interfacehandle : WINUSB_INTERFACE_HANDLE, policytype : u32, valuelength : u32, value : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_usbioctl", feature = "Win32_winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_StartTrackingForTimeSync(interfacehandle : WINUSB_INTERFACE_HANDLE, starttrackinginfo : *const super::usbioctl::USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_usbioctl", feature = "Win32_winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_StopTrackingForTimeSync(interfacehandle : WINUSB_INTERFACE_HANDLE, stoptrackinginfo : *const super::usbioctl::USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_UnregisterIsochBuffer(isochbufferhandle : WINUSB_ISOCH_BUFFER_HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_WriteIsochPipe(bufferhandle : WINUSB_ISOCH_BUFFER_HANDLE, offset : u32, length : u32, framenumber : *mut u32, overlapped : *const super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_WriteIsochPipeAsap(bufferhandle : WINUSB_ISOCH_BUFFER_HANDLE, offset : u32, length : u32, continuestream : windows_sys::core::BOOL, overlapped : *const super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_WritePipe(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : u8, buffer : *const u8, bufferlength : u32, lengthtransferred : *mut u32, overlapped : *const super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
pub type PWINUSB_INTERFACE_HANDLE = *mut *mut core::ffi::c_void;
pub type PWINUSB_ISOCH_BUFFER_HANDLE = *mut *mut core::ffi::c_void;
pub type PWINUSB_SETUP_PACKET = *mut WINUSB_SETUP_PACKET;
pub type WINUSB_INTERFACE_HANDLE = *mut core::ffi::c_void;
pub type WINUSB_ISOCH_BUFFER_HANDLE = *mut core::ffi::c_void;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WINUSB_SETUP_PACKET {
    pub RequestType: u8,
    pub Request: u8,
    pub Value: u16,
    pub Index: u16,
    pub Length: u16,
}
