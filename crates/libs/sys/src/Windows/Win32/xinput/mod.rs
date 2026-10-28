windows_link::link!("xinput1_4.dll" "system" fn XInputEnable(enable : windows_sys::core::BOOL));
#[cfg(feature = "shared")]
windows_link::link!("xinput1_4.dll" "system" fn XInputGetAudioDeviceIds(dwuserindex : super::DWORD, prenderdeviceid : windows_sys::core::PWSTR, prendercount : *mut super::UINT, pcapturedeviceid : windows_sys::core::PWSTR, pcapturecount : *mut super::UINT) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("xinput1_4.dll" "system" fn XInputGetBatteryInformation(dwuserindex : super::DWORD, devtype : super::BYTE, pbatteryinformation : *mut XINPUT_BATTERY_INFORMATION) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("xinput1_4.dll" "system" fn XInputGetCapabilities(dwuserindex : super::DWORD, dwflags : super::DWORD, pcapabilities : *mut XINPUT_CAPABILITIES) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("xinput1_4.dll" "system" fn XInputGetKeystroke(dwuserindex : super::DWORD, dwreserved : super::DWORD, pkeystroke : PXINPUT_KEYSTROKE) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("xinput1_4.dll" "system" fn XInputGetState(dwuserindex : super::DWORD, pstate : *mut XINPUT_STATE) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("xinput1_4.dll" "system" fn XInputSetState(dwuserindex : super::DWORD, pvibration : *const XINPUT_VIBRATION) -> super::DWORD);
pub const BATTERY_DEVTYPE_GAMEPAD: i32 = 0;
pub const BATTERY_DEVTYPE_HEADSET: i32 = 1;
pub const BATTERY_LEVEL_EMPTY: i32 = 0;
pub const BATTERY_LEVEL_FULL: i32 = 3;
pub const BATTERY_LEVEL_LOW: i32 = 1;
pub const BATTERY_LEVEL_MEDIUM: i32 = 2;
pub const BATTERY_TYPE_ALKALINE: i32 = 2;
pub const BATTERY_TYPE_DISCONNECTED: i32 = 0;
pub const BATTERY_TYPE_NIMH: i32 = 3;
pub const BATTERY_TYPE_UNKNOWN: i32 = 255;
pub const BATTERY_TYPE_WIRED: i32 = 1;
#[cfg(feature = "shared")]
pub type PXINPUT_BATTERY_INFORMATION = *mut XINPUT_BATTERY_INFORMATION;
#[cfg(feature = "shared")]
pub type PXINPUT_CAPABILITIES = *mut XINPUT_CAPABILITIES;
#[cfg(feature = "shared")]
pub type PXINPUT_GAMEPAD = *mut XINPUT_GAMEPAD;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PXINPUT_KEYSTROKE = *mut XINPUT_KEYSTROKE;
#[cfg(feature = "shared")]
pub type PXINPUT_STATE = *mut XINPUT_STATE;
#[cfg(feature = "shared")]
pub type PXINPUT_VIBRATION = *mut XINPUT_VIBRATION;
pub const VK_PAD_A: i32 = 22528;
pub const VK_PAD_B: i32 = 22529;
pub const VK_PAD_BACK: i32 = 22549;
pub const VK_PAD_DPAD_DOWN: i32 = 22545;
pub const VK_PAD_DPAD_LEFT: i32 = 22546;
pub const VK_PAD_DPAD_RIGHT: i32 = 22547;
pub const VK_PAD_DPAD_UP: i32 = 22544;
pub const VK_PAD_LSHOULDER: i32 = 22533;
pub const VK_PAD_LTHUMB_DOWN: i32 = 22561;
pub const VK_PAD_LTHUMB_DOWNLEFT: i32 = 22567;
pub const VK_PAD_LTHUMB_DOWNRIGHT: i32 = 22566;
pub const VK_PAD_LTHUMB_LEFT: i32 = 22563;
pub const VK_PAD_LTHUMB_PRESS: i32 = 22550;
pub const VK_PAD_LTHUMB_RIGHT: i32 = 22562;
pub const VK_PAD_LTHUMB_UP: i32 = 22560;
pub const VK_PAD_LTHUMB_UPLEFT: i32 = 22564;
pub const VK_PAD_LTHUMB_UPRIGHT: i32 = 22565;
pub const VK_PAD_LTRIGGER: i32 = 22534;
pub const VK_PAD_RSHOULDER: i32 = 22532;
pub const VK_PAD_RTHUMB_DOWN: i32 = 22577;
pub const VK_PAD_RTHUMB_DOWNLEFT: i32 = 22583;
pub const VK_PAD_RTHUMB_DOWNRIGHT: i32 = 22582;
pub const VK_PAD_RTHUMB_LEFT: i32 = 22579;
pub const VK_PAD_RTHUMB_PRESS: i32 = 22551;
pub const VK_PAD_RTHUMB_RIGHT: i32 = 22578;
pub const VK_PAD_RTHUMB_UP: i32 = 22576;
pub const VK_PAD_RTHUMB_UPLEFT: i32 = 22580;
pub const VK_PAD_RTHUMB_UPRIGHT: i32 = 22581;
pub const VK_PAD_RTRIGGER: i32 = 22535;
pub const VK_PAD_START: i32 = 22548;
pub const VK_PAD_X: i32 = 22530;
pub const VK_PAD_Y: i32 = 22531;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XINPUT_BATTERY_INFORMATION {
    pub BatteryType: super::BYTE,
    pub BatteryLevel: super::BYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XINPUT_CAPABILITIES {
    pub Type: super::BYTE,
    pub SubType: super::BYTE,
    pub Flags: super::WORD,
    pub Gamepad: XINPUT_GAMEPAD,
    pub Vibration: XINPUT_VIBRATION,
}
pub const XINPUT_CAPS_FFB_SUPPORTED: i32 = 1;
pub const XINPUT_CAPS_NO_NAVIGATION: i32 = 16;
pub const XINPUT_CAPS_PMD_SUPPORTED: i32 = 8;
pub const XINPUT_CAPS_VOICE_SUPPORTED: i32 = 4;
pub const XINPUT_CAPS_WIRELESS: i32 = 2;
pub const XINPUT_DEVSUBTYPE_ARCADE_PAD: i32 = 19;
pub const XINPUT_DEVSUBTYPE_ARCADE_STICK: i32 = 3;
pub const XINPUT_DEVSUBTYPE_DANCE_PAD: i32 = 5;
pub const XINPUT_DEVSUBTYPE_DRUM_KIT: i32 = 8;
pub const XINPUT_DEVSUBTYPE_FLIGHT_STICK: i32 = 4;
pub const XINPUT_DEVSUBTYPE_GAMEPAD: i32 = 1;
pub const XINPUT_DEVSUBTYPE_GUITAR: i32 = 6;
pub const XINPUT_DEVSUBTYPE_GUITAR_ALTERNATE: i32 = 7;
pub const XINPUT_DEVSUBTYPE_GUITAR_BASS: i32 = 11;
pub const XINPUT_DEVSUBTYPE_UNKNOWN: i32 = 0;
pub const XINPUT_DEVSUBTYPE_WHEEL: i32 = 2;
pub const XINPUT_DEVTYPE_GAMEPAD: i32 = 1;
pub const XINPUT_DLL: windows_sys::core::PCSTR = windows_sys::core::s!("xinput1_4.dll");
pub const XINPUT_DLL_A: windows_sys::core::PCSTR = windows_sys::core::s!("xinput1_4.dll");
pub const XINPUT_DLL_W: windows_sys::core::PCWSTR = windows_sys::core::w!("xinput1_4.dll");
pub const XINPUT_FLAG_GAMEPAD: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XINPUT_GAMEPAD {
    pub wButtons: super::WORD,
    pub bLeftTrigger: super::BYTE,
    pub bRightTrigger: super::BYTE,
    pub sThumbLX: super::SHORT,
    pub sThumbLY: super::SHORT,
    pub sThumbRX: super::SHORT,
    pub sThumbRY: super::SHORT,
}
pub const XINPUT_GAMEPAD_A: i32 = 4096;
pub const XINPUT_GAMEPAD_B: i32 = 8192;
pub const XINPUT_GAMEPAD_BACK: i32 = 32;
pub const XINPUT_GAMEPAD_DPAD_DOWN: i32 = 2;
pub const XINPUT_GAMEPAD_DPAD_LEFT: i32 = 4;
pub const XINPUT_GAMEPAD_DPAD_RIGHT: i32 = 8;
pub const XINPUT_GAMEPAD_DPAD_UP: i32 = 1;
pub const XINPUT_GAMEPAD_LEFT_SHOULDER: i32 = 256;
pub const XINPUT_GAMEPAD_LEFT_THUMB: i32 = 64;
pub const XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE: i32 = 7849;
pub const XINPUT_GAMEPAD_RIGHT_SHOULDER: i32 = 512;
pub const XINPUT_GAMEPAD_RIGHT_THUMB: i32 = 128;
pub const XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE: i32 = 8689;
pub const XINPUT_GAMEPAD_START: i32 = 16;
pub const XINPUT_GAMEPAD_TRIGGER_THRESHOLD: i32 = 30;
pub const XINPUT_GAMEPAD_X: i32 = 16384;
pub const XINPUT_GAMEPAD_Y: i32 = 32768;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct XINPUT_KEYSTROKE {
    pub VirtualKey: super::WORD,
    pub Unicode: super::WCHAR,
    pub Flags: super::WORD,
    pub UserIndex: super::BYTE,
    pub HidCode: super::BYTE,
}
pub const XINPUT_KEYSTROKE_KEYDOWN: i32 = 1;
pub const XINPUT_KEYSTROKE_KEYUP: i32 = 2;
pub const XINPUT_KEYSTROKE_REPEAT: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XINPUT_STATE {
    pub dwPacketNumber: super::DWORD,
    pub Gamepad: XINPUT_GAMEPAD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XINPUT_VIBRATION {
    pub wLeftMotorSpeed: super::WORD,
    pub wRightMotorSpeed: super::WORD,
}
pub const XUSER_INDEX_ANY: i32 = 255;
pub const XUSER_MAX_COUNT: i32 = 4;
