#![allow(non_snake_case, non_camel_case_types)]
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
pub struct HRAWINPUT(i32);
pub struct INPUT_MESSAGE_DEVICE_TYPE(i32);
pub struct INPUT_MESSAGE_ORIGIN_ID(i32);
pub struct INPUT_MESSAGE_SOURCE(i32);
pub struct RAWHID(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RAWINPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RAWINPUTDEVICE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RAWINPUTDEVICELIST(i32);
pub struct RAWINPUTDEVICE_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RAWINPUTHEADER(i32);
pub struct RAWKEYBOARD(i32);
pub struct RAWMOUSE(i32);
pub struct RAW_INPUT_DATA_COMMAND_FLAGS(i32);
pub struct RAW_INPUT_DEVICE_INFO_COMMAND(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RID_DEVICE_INFO(i32);
pub struct RID_DEVICE_INFO_HID(i32);
pub struct RID_DEVICE_INFO_KEYBOARD(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RID_DEVICE_INFO_MOUSE(i32);
pub struct RID_DEVICE_INFO_TYPE(i32);
