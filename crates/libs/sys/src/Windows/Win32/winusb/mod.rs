#[cfg(feature = "shared")]
windows_link::link!("winusb.dll" "system" fn WinUsb_AbortPipe(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : super::UCHAR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_ControlTransfer(interfacehandle : WINUSB_INTERFACE_HANDLE, setuppacket : WINUSB_SETUP_PACKET, buffer : super::PUCHAR, bufferlength : super::ULONG, lengthtransferred : super::PULONG, overlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winusb.dll" "system" fn WinUsb_FlushPipe(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : super::UCHAR) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_Free(interfacehandle : WINUSB_INTERFACE_HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_GetAdjustedFrameNumber(currentframenumber : super::PULONG, timestamp : super::LARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winusb.dll" "system" fn WinUsb_GetAssociatedInterface(interfacehandle : WINUSB_INTERFACE_HANDLE, associatedinterfaceindex : super::UCHAR, associatedinterfacehandle : PWINUSB_INTERFACE_HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("winusb.dll" "system" fn WinUsb_GetCurrentAlternateSetting(interfacehandle : WINUSB_INTERFACE_HANDLE, settingnumber : super::PUCHAR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_GetCurrentFrameNumber(interfacehandle : WINUSB_INTERFACE_HANDLE, currentframenumber : super::PULONG, timestamp : *mut super::LARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "usbioctl", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_GetCurrentFrameNumberAndQpc(interfacehandle : WINUSB_INTERFACE_HANDLE, frameqpcinfo : super::PUSB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_GetDescriptor(interfacehandle : WINUSB_INTERFACE_HANDLE, descriptortype : super::UCHAR, index : super::UCHAR, languageid : super::USHORT, buffer : super::PUCHAR, bufferlength : super::ULONG, lengthtransferred : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_GetOverlappedResult(interfacehandle : WINUSB_INTERFACE_HANDLE, lpoverlapped : super::LPOVERLAPPED, lpnumberofbytestransferred : super::LPDWORD, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_GetPipePolicy(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : super::UCHAR, policytype : super::ULONG, valuelength : super::PULONG, value : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_GetPowerPolicy(interfacehandle : WINUSB_INTERFACE_HANDLE, policytype : super::ULONG, valuelength : super::PULONG, value : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("winusb.dll" "system" fn WinUsb_Initialize(devicehandle : super::HANDLE, interfacehandle : PWINUSB_INTERFACE_HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "usbspec", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_ParseConfigurationDescriptor(configurationdescriptor : super::PUSB_CONFIGURATION_DESCRIPTOR, startposition : super::PVOID, interfacenumber : super::LONG, alternatesetting : super::LONG, interfaceclass : super::LONG, interfacesubclass : super::LONG, interfaceprotocol : super::LONG) -> super::PUSB_INTERFACE_DESCRIPTOR);
#[cfg(all(feature = "shared", feature = "usbspec", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_ParseDescriptors(descriptorbuffer : super::PVOID, totallength : super::ULONG, startposition : super::PVOID, descriptortype : super::LONG) -> super::PUSB_COMMON_DESCRIPTOR);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_QueryDeviceInformation(interfacehandle : WINUSB_INTERFACE_HANDLE, informationtype : super::ULONG, bufferlength : super::PULONG, buffer : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "usbspec"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_QueryInterfaceSettings(interfacehandle : WINUSB_INTERFACE_HANDLE, alternateinterfacenumber : super::UCHAR, usbaltinterfacedescriptor : super::PUSB_INTERFACE_DESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "usb"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_QueryPipe(interfacehandle : WINUSB_INTERFACE_HANDLE, alternateinterfacenumber : super::UCHAR, pipeindex : super::UCHAR, pipeinformation : PWINUSB_PIPE_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "usb"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_QueryPipeEx(interfacehandle : WINUSB_INTERFACE_HANDLE, alternatesettingnumber : super::UCHAR, pipeindex : super::UCHAR, pipeinformationex : PWINUSB_PIPE_INFORMATION_EX) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "usb", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_ReadIsochPipe(bufferhandle : WINUSB_ISOCH_BUFFER_HANDLE, offset : super::ULONG, length : super::ULONG, framenumber : super::PULONG, numberofpackets : super::ULONG, isopacketdescriptors : super::PUSBD_ISO_PACKET_DESCRIPTOR, overlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "usb", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_ReadIsochPipeAsap(bufferhandle : WINUSB_ISOCH_BUFFER_HANDLE, offset : super::ULONG, length : super::ULONG, continuestream : windows_sys::core::BOOL, numberofpackets : super::ULONG, isopacketdescriptors : super::PUSBD_ISO_PACKET_DESCRIPTOR, overlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_ReadPipe(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : super::UCHAR, buffer : super::PUCHAR, bufferlength : super::ULONG, lengthtransferred : super::PULONG, overlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_RegisterIsochBuffer(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : super::UCHAR, buffer : super::PUCHAR, bufferlength : super::ULONG, isochbufferhandle : PWINUSB_ISOCH_BUFFER_HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winusb.dll" "system" fn WinUsb_ResetPipe(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : super::UCHAR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winusb.dll" "system" fn WinUsb_SetCurrentAlternateSetting(interfacehandle : WINUSB_INTERFACE_HANDLE, settingnumber : super::UCHAR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_SetPipePolicy(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : super::UCHAR, policytype : super::ULONG, valuelength : super::ULONG, value : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_SetPowerPolicy(interfacehandle : WINUSB_INTERFACE_HANDLE, policytype : super::ULONG, valuelength : super::ULONG, value : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "usbioctl", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_StartTrackingForTimeSync(interfacehandle : WINUSB_INTERFACE_HANDLE, starttrackinginfo : super::PUSB_START_TRACKING_FOR_TIME_SYNC_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "usbioctl", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_StopTrackingForTimeSync(interfacehandle : WINUSB_INTERFACE_HANDLE, stoptrackinginfo : super::PUSB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION) -> windows_sys::core::BOOL);
windows_link::link!("winusb.dll" "system" fn WinUsb_UnregisterIsochBuffer(isochbufferhandle : WINUSB_ISOCH_BUFFER_HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_WriteIsochPipe(bufferhandle : WINUSB_ISOCH_BUFFER_HANDLE, offset : super::ULONG, length : super::ULONG, framenumber : super::PULONG, overlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_WriteIsochPipeAsap(bufferhandle : WINUSB_ISOCH_BUFFER_HANDLE, offset : super::ULONG, length : super::ULONG, continuestream : windows_sys::core::BOOL, overlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("winusb.dll" "system" fn WinUsb_WritePipe(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : super::UCHAR, buffer : super::PUCHAR, bufferlength : super::ULONG, lengthtransferred : super::PULONG, overlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
pub const ALLOW_PARTIAL_READS: i32 = 5;
pub const AUTO_CLEAR_STALL: i32 = 2;
pub const AUTO_FLUSH: i32 = 6;
pub const AUTO_SUSPEND: i32 = 129;
pub const DEVICE_SPEED: i32 = 1;
pub const FullSpeed: i32 = 2;
pub const HighSpeed: i32 = 3;
pub const IGNORE_SHORT_PACKETS: i32 = 4;
pub const LowSpeed: i32 = 1;
pub const MAXIMUM_TRANSFER_SIZE: i32 = 8;
pub const PIPE_TRANSFER_TIMEOUT: i32 = 3;
pub type PWINUSB_INTERFACE_HANDLE = *mut *mut core::ffi::c_void;
pub type PWINUSB_ISOCH_BUFFER_HANDLE = *mut *mut core::ffi::c_void;
#[cfg(all(feature = "shared", feature = "usb"))]
pub type PWINUSB_PIPE_INFORMATION = *mut WINUSB_PIPE_INFORMATION;
#[cfg(all(feature = "shared", feature = "usb"))]
pub type PWINUSB_PIPE_INFORMATION_EX = *mut WINUSB_PIPE_INFORMATION_EX;
#[cfg(feature = "shared")]
pub type PWINUSB_SETUP_PACKET = *mut WINUSB_SETUP_PACKET;
pub const RAW_IO: i32 = 7;
pub const RESET_PIPE_ON_RESUME: i32 = 9;
pub const SHORT_PACKET_TERMINATE: i32 = 1;
pub const SUSPEND_DELAY: i32 = 131;
pub type WINUSB_INTERFACE_HANDLE = *mut core::ffi::c_void;
pub type WINUSB_ISOCH_BUFFER_HANDLE = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct WINUSB_PIPE_INFORMATION {
    pub PipeType: super::USBD_PIPE_TYPE,
    pub PipeId: super::UCHAR,
    pub MaximumPacketSize: super::USHORT,
    pub Interval: super::UCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct WINUSB_PIPE_INFORMATION_EX {
    pub PipeType: super::USBD_PIPE_TYPE,
    pub PipeId: super::UCHAR,
    pub MaximumPacketSize: super::USHORT,
    pub Interval: super::UCHAR,
    pub MaximumBytesPerInterval: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINUSB_SETUP_PACKET {
    pub RequestType: super::UCHAR,
    pub Request: super::UCHAR,
    pub Value: super::USHORT,
    pub Index: super::USHORT,
    pub Length: super::USHORT,
}
pub const WinUSB_TestGuid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xda812bff_12c3_46a2_8e2b_dbd3b7834c43);
