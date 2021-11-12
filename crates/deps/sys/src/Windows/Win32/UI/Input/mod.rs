#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Win32_UI_Input_Ime")]
pub mod Ime;
#[cfg(feature = "Win32_UI_Input_Ink")]
pub mod Ink;
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
pub mod KeyboardAndMouse;
#[cfg(feature = "Win32_UI_Input_Pointer")]
pub mod Pointer;
#[cfg(feature = "Win32_UI_Input_Radial")]
pub mod Radial;
#[cfg(feature = "Win32_UI_Input_Touch")]
pub mod Touch;
#[cfg(feature = "Win32_UI_Input_XboxController")]
pub mod XboxController;
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefRawInputProc(parawinput: *const *const RAWINPUT, ninput: i32, cbsizeheader: u32) -> super::super::Foundation::LRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCIMSSM(inputmessagesource: *mut INPUT_MESSAGE_SOURCE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentInputMessageSource(inputmessagesource: *mut INPUT_MESSAGE_SOURCE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRawInputBuffer(pdata: *mut RAWINPUT, pcbsize: *mut u32, cbsizeheader: u32) -> u32;
    pub fn GetRawInputData(hrawinput: HRAWINPUT, uicommand: RAW_INPUT_DATA_COMMAND_FLAGS, pdata: *mut ::core::ffi::c_void, pcbsize: *mut u32, cbsizeheader: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRawInputDeviceInfoA(hdevice: super::super::Foundation::HANDLE, uicommand: RAW_INPUT_DEVICE_INFO_COMMAND, pdata: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRawInputDeviceInfoW(hdevice: super::super::Foundation::HANDLE, uicommand: RAW_INPUT_DEVICE_INFO_COMMAND, pdata: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRawInputDeviceList(prawinputdevicelist: *mut RAWINPUTDEVICELIST, puinumdevices: *mut u32, cbsize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRegisteredRawInputDevices(prawinputdevices: *mut RAWINPUTDEVICE, puinumdevices: *mut u32, cbsize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterRawInputDevices(prawinputdevices: *const RAWINPUTDEVICE, uinumdevices: u32, cbsize: u32) -> super::super::Foundation::BOOL;
}
#[repr(C)]
pub struct HRAWINPUT(i32);
#[repr(transparent)]
pub struct INPUT_MESSAGE_DEVICE_TYPE(pub i32);
pub const IMDT_UNAVAILABLE: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(0i32);
pub const IMDT_KEYBOARD: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(1i32);
pub const IMDT_MOUSE: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(2i32);
pub const IMDT_TOUCH: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(4i32);
pub const IMDT_PEN: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(8i32);
pub const IMDT_TOUCHPAD: INPUT_MESSAGE_DEVICE_TYPE = INPUT_MESSAGE_DEVICE_TYPE(16i32);
#[repr(transparent)]
pub struct INPUT_MESSAGE_ORIGIN_ID(pub i32);
pub const IMO_UNAVAILABLE: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(0i32);
pub const IMO_HARDWARE: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(1i32);
pub const IMO_INJECTED: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(2i32);
pub const IMO_SYSTEM: INPUT_MESSAGE_ORIGIN_ID = INPUT_MESSAGE_ORIGIN_ID(4i32);
#[repr(C)]
pub struct INPUT_MESSAGE_SOURCE(i32);
#[repr(C)]
pub struct RAWHID(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RAWINPUT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RAWINPUTDEVICE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RAWINPUTDEVICELIST(i32);
#[repr(transparent)]
pub struct RAWINPUTDEVICE_FLAGS(pub u32);
pub const RIDEV_REMOVE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(1u32);
pub const RIDEV_EXCLUDE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(16u32);
pub const RIDEV_PAGEONLY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(32u32);
pub const RIDEV_NOLEGACY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(48u32);
pub const RIDEV_INPUTSINK: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(256u32);
pub const RIDEV_CAPTUREMOUSE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(512u32);
pub const RIDEV_NOHOTKEYS: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(512u32);
pub const RIDEV_APPKEYS: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(1024u32);
pub const RIDEV_EXINPUTSINK: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(4096u32);
pub const RIDEV_DEVNOTIFY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(8192u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RAWINPUTHEADER(i32);
#[repr(C)]
pub struct RAWKEYBOARD(i32);
#[repr(C)]
pub struct RAWMOUSE(i32);
#[repr(transparent)]
pub struct RAW_INPUT_DATA_COMMAND_FLAGS(pub u32);
pub const RID_HEADER: RAW_INPUT_DATA_COMMAND_FLAGS = RAW_INPUT_DATA_COMMAND_FLAGS(268435461u32);
pub const RID_INPUT: RAW_INPUT_DATA_COMMAND_FLAGS = RAW_INPUT_DATA_COMMAND_FLAGS(268435459u32);
#[repr(transparent)]
pub struct RAW_INPUT_DEVICE_INFO_COMMAND(pub u32);
pub const RIDI_PREPARSEDDATA: RAW_INPUT_DEVICE_INFO_COMMAND = RAW_INPUT_DEVICE_INFO_COMMAND(536870917u32);
pub const RIDI_DEVICENAME: RAW_INPUT_DEVICE_INFO_COMMAND = RAW_INPUT_DEVICE_INFO_COMMAND(536870919u32);
pub const RIDI_DEVICEINFO: RAW_INPUT_DEVICE_INFO_COMMAND = RAW_INPUT_DEVICE_INFO_COMMAND(536870923u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RID_DEVICE_INFO(i32);
#[repr(C)]
pub struct RID_DEVICE_INFO_HID(i32);
#[repr(C)]
pub struct RID_DEVICE_INFO_KEYBOARD(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RID_DEVICE_INFO_MOUSE(i32);
#[repr(transparent)]
pub struct RID_DEVICE_INFO_TYPE(pub u32);
pub const RIM_TYPEMOUSE: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(0u32);
pub const RIM_TYPEKEYBOARD: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(1u32);
pub const RIM_TYPEHID: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(2u32);
