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
    fn DefRawInputProc();
    fn GetCIMSSM();
    fn GetCurrentInputMessageSource();
    fn GetRawInputBuffer();
    fn GetRawInputData();
    fn GetRawInputDeviceInfoA();
    fn GetRawInputDeviceInfoW();
    fn GetRawInputDeviceList();
    fn GetRegisteredRawInputDevices();
    fn HRAWINPUT();
    fn INPUT_MESSAGE_DEVICE_TYPE();
    fn INPUT_MESSAGE_ORIGIN_ID();
    fn INPUT_MESSAGE_SOURCE();
    fn RAWHID();
    fn RAWINPUT();
    fn RAWINPUTDEVICE();
    fn RAWINPUTDEVICELIST();
    fn RAWINPUTDEVICE_FLAGS();
    fn RAWINPUTHEADER();
    fn RAWKEYBOARD();
    fn RAWMOUSE();
    fn RAW_INPUT_DATA_COMMAND_FLAGS();
    fn RAW_INPUT_DEVICE_INFO_COMMAND();
    fn RID_DEVICE_INFO();
    fn RID_DEVICE_INFO_HID();
    fn RID_DEVICE_INFO_KEYBOARD();
    fn RID_DEVICE_INFO_MOUSE();
    fn RID_DEVICE_INFO_TYPE();
    fn RegisterRawInputDevices();
}
