#[inline]
pub unsafe fn WinUsb_AbortPipe(interfacehandle: WINUSB_INTERFACE_HANDLE, pipeid: u8) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_AbortPipe(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : u8) -> windows_core::BOOL);
    unsafe { WinUsb_AbortPipe(interfacehandle, pipeid) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WinUsb_ControlTransfer(interfacehandle: WINUSB_INTERFACE_HANDLE, setuppacket: WINUSB_SETUP_PACKET, buffer: Option<&mut [u8]>, lengthtransferred: Option<*mut u32>, overlapped: Option<*const super::minwinbase::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_ControlTransfer(interfacehandle : WINUSB_INTERFACE_HANDLE, setuppacket : WINUSB_SETUP_PACKET, buffer : *mut u8, bufferlength : u32, lengthtransferred : *mut u32, overlapped : *const super::minwinbase::OVERLAPPED) -> windows_core::BOOL);
    unsafe { WinUsb_ControlTransfer(interfacehandle, core::mem::transmute(setuppacket), core::mem::transmute(buffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), buffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lengthtransferred.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WinUsb_FlushPipe(interfacehandle: WINUSB_INTERFACE_HANDLE, pipeid: u8) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_FlushPipe(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : u8) -> windows_core::BOOL);
    unsafe { WinUsb_FlushPipe(interfacehandle, pipeid) }
}
#[inline]
pub unsafe fn WinUsb_Free(interfacehandle: WINUSB_INTERFACE_HANDLE) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_Free(interfacehandle : WINUSB_INTERFACE_HANDLE) -> windows_core::BOOL);
    unsafe { WinUsb_Free(interfacehandle) }
}
#[inline]
pub unsafe fn WinUsb_GetAdjustedFrameNumber(currentframenumber: *mut u32, timestamp: i64) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_GetAdjustedFrameNumber(currentframenumber : *mut u32, timestamp : i64) -> windows_core::BOOL);
    unsafe { WinUsb_GetAdjustedFrameNumber(currentframenumber as _, timestamp) }
}
#[inline]
pub unsafe fn WinUsb_GetAssociatedInterface(interfacehandle: WINUSB_INTERFACE_HANDLE, associatedinterfaceindex: u8, associatedinterfacehandle: *mut *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_GetAssociatedInterface(interfacehandle : WINUSB_INTERFACE_HANDLE, associatedinterfaceindex : u8, associatedinterfacehandle : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WinUsb_GetAssociatedInterface(interfacehandle, associatedinterfaceindex, associatedinterfacehandle as _) }
}
#[inline]
pub unsafe fn WinUsb_GetCurrentAlternateSetting(interfacehandle: WINUSB_INTERFACE_HANDLE, settingnumber: *mut u8) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_GetCurrentAlternateSetting(interfacehandle : WINUSB_INTERFACE_HANDLE, settingnumber : *mut u8) -> windows_core::BOOL);
    unsafe { WinUsb_GetCurrentAlternateSetting(interfacehandle, settingnumber as _) }
}
#[inline]
pub unsafe fn WinUsb_GetCurrentFrameNumber(interfacehandle: WINUSB_INTERFACE_HANDLE, currentframenumber: *mut u32, timestamp: *mut i64) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_GetCurrentFrameNumber(interfacehandle : WINUSB_INTERFACE_HANDLE, currentframenumber : *mut u32, timestamp : *mut i64) -> windows_core::BOOL);
    unsafe { WinUsb_GetCurrentFrameNumber(interfacehandle, currentframenumber as _, timestamp as _) }
}
#[cfg(all(feature = "Win32_usbioctl", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WinUsb_GetCurrentFrameNumberAndQpc(interfacehandle: WINUSB_INTERFACE_HANDLE, frameqpcinfo: *const super::usbioctl::USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_GetCurrentFrameNumberAndQpc(interfacehandle : WINUSB_INTERFACE_HANDLE, frameqpcinfo : *const super::usbioctl::USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION) -> windows_core::BOOL);
    unsafe { WinUsb_GetCurrentFrameNumberAndQpc(interfacehandle, frameqpcinfo) }
}
#[inline]
pub unsafe fn WinUsb_GetDescriptor(interfacehandle: WINUSB_INTERFACE_HANDLE, descriptortype: u8, index: u8, languageid: u16, buffer: Option<&mut [u8]>, lengthtransferred: *mut u32) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_GetDescriptor(interfacehandle : WINUSB_INTERFACE_HANDLE, descriptortype : u8, index : u8, languageid : u16, buffer : *mut u8, bufferlength : u32, lengthtransferred : *mut u32) -> windows_core::BOOL);
    unsafe { WinUsb_GetDescriptor(interfacehandle, descriptortype, index, languageid, core::mem::transmute(buffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), buffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lengthtransferred as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WinUsb_GetOverlappedResult(interfacehandle: WINUSB_INTERFACE_HANDLE, lpoverlapped: *const super::minwinbase::OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: bool) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_GetOverlappedResult(interfacehandle : WINUSB_INTERFACE_HANDLE, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpnumberofbytestransferred : *mut u32, bwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WinUsb_GetOverlappedResult(interfacehandle, lpoverlapped, lpnumberofbytestransferred as _, bwait.into()) }
}
#[inline]
pub unsafe fn WinUsb_GetPipePolicy(interfacehandle: WINUSB_INTERFACE_HANDLE, pipeid: u8, policytype: u32, valuelength: *mut u32, value: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_GetPipePolicy(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : u8, policytype : u32, valuelength : *mut u32, value : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WinUsb_GetPipePolicy(interfacehandle, pipeid, policytype, valuelength as _, value as _) }
}
#[inline]
pub unsafe fn WinUsb_GetPowerPolicy(interfacehandle: WINUSB_INTERFACE_HANDLE, policytype: u32, valuelength: *mut u32, value: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_GetPowerPolicy(interfacehandle : WINUSB_INTERFACE_HANDLE, policytype : u32, valuelength : *mut u32, value : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WinUsb_GetPowerPolicy(interfacehandle, policytype, valuelength as _, value as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn WinUsb_Initialize(devicehandle: super::winnt::HANDLE, interfacehandle: *mut *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_Initialize(devicehandle : super::winnt::HANDLE, interfacehandle : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WinUsb_Initialize(devicehandle, interfacehandle as _) }
}
#[cfg(feature = "Win32_usbspec")]
#[inline]
pub unsafe fn WinUsb_ParseConfigurationDescriptor(configurationdescriptor: *const super::usbspec::USB_CONFIGURATION_DESCRIPTOR, startposition: *const core::ffi::c_void, interfacenumber: i32, alternatesetting: i32, interfaceclass: i32, interfacesubclass: i32, interfaceprotocol: i32) -> super::usbspec::PUSB_INTERFACE_DESCRIPTOR {
    windows_core::link!("winusb.dll" "system" fn WinUsb_ParseConfigurationDescriptor(configurationdescriptor : *const super::usbspec::USB_CONFIGURATION_DESCRIPTOR, startposition : *const core::ffi::c_void, interfacenumber : i32, alternatesetting : i32, interfaceclass : i32, interfacesubclass : i32, interfaceprotocol : i32) -> super::usbspec::PUSB_INTERFACE_DESCRIPTOR);
    unsafe { WinUsb_ParseConfigurationDescriptor(configurationdescriptor, startposition, interfacenumber, alternatesetting, interfaceclass, interfacesubclass, interfaceprotocol) }
}
#[cfg(feature = "Win32_usbspec")]
#[inline]
pub unsafe fn WinUsb_ParseDescriptors(descriptorbuffer: *const core::ffi::c_void, totallength: u32, startposition: *const core::ffi::c_void, descriptortype: i32) -> super::usbspec::PUSB_COMMON_DESCRIPTOR {
    windows_core::link!("winusb.dll" "system" fn WinUsb_ParseDescriptors(descriptorbuffer : *const core::ffi::c_void, totallength : u32, startposition : *const core::ffi::c_void, descriptortype : i32) -> super::usbspec::PUSB_COMMON_DESCRIPTOR);
    unsafe { WinUsb_ParseDescriptors(descriptorbuffer, totallength, startposition, descriptortype) }
}
#[inline]
pub unsafe fn WinUsb_QueryDeviceInformation(interfacehandle: WINUSB_INTERFACE_HANDLE, informationtype: u32, bufferlength: *mut u32, buffer: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_QueryDeviceInformation(interfacehandle : WINUSB_INTERFACE_HANDLE, informationtype : u32, bufferlength : *mut u32, buffer : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WinUsb_QueryDeviceInformation(interfacehandle, informationtype, bufferlength as _, buffer as _) }
}
#[cfg(feature = "Win32_usbspec")]
#[inline]
pub unsafe fn WinUsb_QueryInterfaceSettings(interfacehandle: WINUSB_INTERFACE_HANDLE, alternateinterfacenumber: u8, usbaltinterfacedescriptor: *mut super::usbspec::USB_INTERFACE_DESCRIPTOR) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_QueryInterfaceSettings(interfacehandle : WINUSB_INTERFACE_HANDLE, alternateinterfacenumber : u8, usbaltinterfacedescriptor : *mut super::usbspec::USB_INTERFACE_DESCRIPTOR) -> windows_core::BOOL);
    unsafe { WinUsb_QueryInterfaceSettings(interfacehandle, alternateinterfacenumber, usbaltinterfacedescriptor as _) }
}
#[cfg(all(feature = "Win32_usb", feature = "Win32_winusbio"))]
#[inline]
pub unsafe fn WinUsb_QueryPipe(interfacehandle: WINUSB_INTERFACE_HANDLE, alternateinterfacenumber: u8, pipeindex: u8, pipeinformation: *mut super::winusbio::WINUSB_PIPE_INFORMATION) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_QueryPipe(interfacehandle : WINUSB_INTERFACE_HANDLE, alternateinterfacenumber : u8, pipeindex : u8, pipeinformation : *mut super::winusbio::WINUSB_PIPE_INFORMATION) -> windows_core::BOOL);
    unsafe { WinUsb_QueryPipe(interfacehandle, alternateinterfacenumber, pipeindex, pipeinformation as _) }
}
#[cfg(all(feature = "Win32_usb", feature = "Win32_winusbio"))]
#[inline]
pub unsafe fn WinUsb_QueryPipeEx(interfacehandle: WINUSB_INTERFACE_HANDLE, alternatesettingnumber: u8, pipeindex: u8, pipeinformationex: *mut super::winusbio::WINUSB_PIPE_INFORMATION_EX) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_QueryPipeEx(interfacehandle : WINUSB_INTERFACE_HANDLE, alternatesettingnumber : u8, pipeindex : u8, pipeinformationex : *mut super::winusbio::WINUSB_PIPE_INFORMATION_EX) -> windows_core::BOOL);
    unsafe { WinUsb_QueryPipeEx(interfacehandle, alternatesettingnumber, pipeindex, pipeinformationex as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_usb", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WinUsb_ReadIsochPipe(bufferhandle: WINUSB_ISOCH_BUFFER_HANDLE, offset: u32, length: u32, framenumber: *mut u32, isopacketdescriptors: &mut [super::usb::USBD_ISO_PACKET_DESCRIPTOR], overlapped: Option<*const super::minwinbase::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_ReadIsochPipe(bufferhandle : WINUSB_ISOCH_BUFFER_HANDLE, offset : u32, length : u32, framenumber : *mut u32, numberofpackets : u32, isopacketdescriptors : *mut super::usb::USBD_ISO_PACKET_DESCRIPTOR, overlapped : *const super::minwinbase::OVERLAPPED) -> windows_core::BOOL);
    unsafe { WinUsb_ReadIsochPipe(bufferhandle, offset, length, framenumber as _, isopacketdescriptors.len().try_into().unwrap(), core::mem::transmute(isopacketdescriptors.as_ptr()), overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_usb", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WinUsb_ReadIsochPipeAsap(bufferhandle: WINUSB_ISOCH_BUFFER_HANDLE, offset: u32, length: u32, continuestream: bool, isopacketdescriptors: &mut [super::usb::USBD_ISO_PACKET_DESCRIPTOR], overlapped: Option<*const super::minwinbase::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_ReadIsochPipeAsap(bufferhandle : WINUSB_ISOCH_BUFFER_HANDLE, offset : u32, length : u32, continuestream : windows_core::BOOL, numberofpackets : u32, isopacketdescriptors : *mut super::usb::USBD_ISO_PACKET_DESCRIPTOR, overlapped : *const super::minwinbase::OVERLAPPED) -> windows_core::BOOL);
    unsafe { WinUsb_ReadIsochPipeAsap(bufferhandle, offset, length, continuestream.into(), isopacketdescriptors.len().try_into().unwrap(), core::mem::transmute(isopacketdescriptors.as_ptr()), overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WinUsb_ReadPipe(interfacehandle: WINUSB_INTERFACE_HANDLE, pipeid: u8, buffer: Option<&mut [u8]>, lengthtransferred: Option<*mut u32>, overlapped: Option<*const super::minwinbase::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_ReadPipe(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : u8, buffer : *mut u8, bufferlength : u32, lengthtransferred : *mut u32, overlapped : *const super::minwinbase::OVERLAPPED) -> windows_core::BOOL);
    unsafe { WinUsb_ReadPipe(interfacehandle, pipeid, core::mem::transmute(buffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), buffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lengthtransferred.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WinUsb_RegisterIsochBuffer(interfacehandle: WINUSB_INTERFACE_HANDLE, pipeid: u8, buffer: &mut [u8], isochbufferhandle: *mut *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_RegisterIsochBuffer(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : u8, buffer : *mut u8, bufferlength : u32, isochbufferhandle : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WinUsb_RegisterIsochBuffer(interfacehandle, pipeid, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), isochbufferhandle as _) }
}
#[inline]
pub unsafe fn WinUsb_ResetPipe(interfacehandle: WINUSB_INTERFACE_HANDLE, pipeid: u8) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_ResetPipe(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : u8) -> windows_core::BOOL);
    unsafe { WinUsb_ResetPipe(interfacehandle, pipeid) }
}
#[inline]
pub unsafe fn WinUsb_SetCurrentAlternateSetting(interfacehandle: WINUSB_INTERFACE_HANDLE, settingnumber: u8) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_SetCurrentAlternateSetting(interfacehandle : WINUSB_INTERFACE_HANDLE, settingnumber : u8) -> windows_core::BOOL);
    unsafe { WinUsb_SetCurrentAlternateSetting(interfacehandle, settingnumber) }
}
#[inline]
pub unsafe fn WinUsb_SetPipePolicy(interfacehandle: WINUSB_INTERFACE_HANDLE, pipeid: u8, policytype: u32, valuelength: u32, value: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_SetPipePolicy(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : u8, policytype : u32, valuelength : u32, value : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WinUsb_SetPipePolicy(interfacehandle, pipeid, policytype, valuelength, value) }
}
#[inline]
pub unsafe fn WinUsb_SetPowerPolicy(interfacehandle: WINUSB_INTERFACE_HANDLE, policytype: u32, valuelength: u32, value: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_SetPowerPolicy(interfacehandle : WINUSB_INTERFACE_HANDLE, policytype : u32, valuelength : u32, value : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WinUsb_SetPowerPolicy(interfacehandle, policytype, valuelength, value) }
}
#[cfg(all(feature = "Win32_usbioctl", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WinUsb_StartTrackingForTimeSync(interfacehandle: WINUSB_INTERFACE_HANDLE, starttrackinginfo: *const super::usbioctl::USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_StartTrackingForTimeSync(interfacehandle : WINUSB_INTERFACE_HANDLE, starttrackinginfo : *const super::usbioctl::USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION) -> windows_core::BOOL);
    unsafe { WinUsb_StartTrackingForTimeSync(interfacehandle, starttrackinginfo) }
}
#[cfg(all(feature = "Win32_usbioctl", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WinUsb_StopTrackingForTimeSync(interfacehandle: WINUSB_INTERFACE_HANDLE, stoptrackinginfo: *const super::usbioctl::USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_StopTrackingForTimeSync(interfacehandle : WINUSB_INTERFACE_HANDLE, stoptrackinginfo : *const super::usbioctl::USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION) -> windows_core::BOOL);
    unsafe { WinUsb_StopTrackingForTimeSync(interfacehandle, stoptrackinginfo) }
}
#[inline]
pub unsafe fn WinUsb_UnregisterIsochBuffer(isochbufferhandle: WINUSB_ISOCH_BUFFER_HANDLE) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_UnregisterIsochBuffer(isochbufferhandle : WINUSB_ISOCH_BUFFER_HANDLE) -> windows_core::BOOL);
    unsafe { WinUsb_UnregisterIsochBuffer(isochbufferhandle) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WinUsb_WriteIsochPipe(bufferhandle: WINUSB_ISOCH_BUFFER_HANDLE, offset: u32, length: u32, framenumber: *mut u32, overlapped: Option<*const super::minwinbase::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_WriteIsochPipe(bufferhandle : WINUSB_ISOCH_BUFFER_HANDLE, offset : u32, length : u32, framenumber : *mut u32, overlapped : *const super::minwinbase::OVERLAPPED) -> windows_core::BOOL);
    unsafe { WinUsb_WriteIsochPipe(bufferhandle, offset, length, framenumber as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WinUsb_WriteIsochPipeAsap(bufferhandle: WINUSB_ISOCH_BUFFER_HANDLE, offset: u32, length: u32, continuestream: bool, overlapped: Option<*const super::minwinbase::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_WriteIsochPipeAsap(bufferhandle : WINUSB_ISOCH_BUFFER_HANDLE, offset : u32, length : u32, continuestream : windows_core::BOOL, overlapped : *const super::minwinbase::OVERLAPPED) -> windows_core::BOOL);
    unsafe { WinUsb_WriteIsochPipeAsap(bufferhandle, offset, length, continuestream.into(), overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WinUsb_WritePipe(interfacehandle: WINUSB_INTERFACE_HANDLE, pipeid: u8, buffer: &[u8], lengthtransferred: Option<*mut u32>, overlapped: Option<*const super::minwinbase::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("winusb.dll" "system" fn WinUsb_WritePipe(interfacehandle : WINUSB_INTERFACE_HANDLE, pipeid : u8, buffer : *const u8, bufferlength : u32, lengthtransferred : *mut u32, overlapped : *const super::minwinbase::OVERLAPPED) -> windows_core::BOOL);
    unsafe { WinUsb_WritePipe(interfacehandle, pipeid, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), lengthtransferred.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWINUSB_INTERFACE_HANDLE(pub *mut *mut core::ffi::c_void);
impl PWINUSB_INTERFACE_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWINUSB_INTERFACE_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWINUSB_ISOCH_BUFFER_HANDLE(pub *mut *mut core::ffi::c_void);
impl PWINUSB_ISOCH_BUFFER_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWINUSB_ISOCH_BUFFER_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWINUSB_SETUP_PACKET(pub *mut WINUSB_SETUP_PACKET);
impl PWINUSB_SETUP_PACKET {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWINUSB_SETUP_PACKET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WINUSB_INTERFACE_HANDLE(pub *mut core::ffi::c_void);
impl WINUSB_INTERFACE_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for WINUSB_INTERFACE_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WINUSB_ISOCH_BUFFER_HANDLE(pub *mut core::ffi::c_void);
impl WINUSB_ISOCH_BUFFER_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for WINUSB_ISOCH_BUFFER_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WINUSB_SETUP_PACKET {
    pub RequestType: u8,
    pub Request: u8,
    pub Value: u16,
    pub Index: u16,
    pub Length: u16,
}
