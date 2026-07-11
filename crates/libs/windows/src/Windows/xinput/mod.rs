#[inline]
pub unsafe fn XInputEnable(enable: bool) {
    windows_core::link!("xinput1_4.dll" "system" fn XInputEnable(enable : windows_core::BOOL));
    unsafe { XInputEnable(enable.into()) }
}
#[inline]
pub unsafe fn XInputGetAudioDeviceIds(dwuserindex: u32, prenderdeviceid: Option<windows_core::PWSTR>, prendercount: Option<*mut u32>, pcapturedeviceid: Option<windows_core::PWSTR>, pcapturecount: Option<*mut u32>) -> u32 {
    windows_core::link!("xinput1_4.dll" "system" fn XInputGetAudioDeviceIds(dwuserindex : u32, prenderdeviceid : windows_core::PWSTR, prendercount : *mut u32, pcapturedeviceid : windows_core::PWSTR, pcapturecount : *mut u32) -> u32);
    unsafe { XInputGetAudioDeviceIds(dwuserindex, prenderdeviceid.unwrap_or(core::mem::zeroed()) as _, prendercount.unwrap_or(core::mem::zeroed()) as _, pcapturedeviceid.unwrap_or(core::mem::zeroed()) as _, pcapturecount.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn XInputGetBatteryInformation(dwuserindex: u32, devtype: u8, pbatteryinformation: *mut XINPUT_BATTERY_INFORMATION) -> u32 {
    windows_core::link!("xinput1_4.dll" "system" fn XInputGetBatteryInformation(dwuserindex : u32, devtype : u8, pbatteryinformation : *mut XINPUT_BATTERY_INFORMATION) -> u32);
    unsafe { XInputGetBatteryInformation(dwuserindex, devtype, pbatteryinformation as _) }
}
#[inline]
pub unsafe fn XInputGetCapabilities(dwuserindex: u32, dwflags: u32, pcapabilities: *mut XINPUT_CAPABILITIES) -> u32 {
    windows_core::link!("xinput1_4.dll" "system" fn XInputGetCapabilities(dwuserindex : u32, dwflags : u32, pcapabilities : *mut XINPUT_CAPABILITIES) -> u32);
    unsafe { XInputGetCapabilities(dwuserindex, dwflags, pcapabilities as _) }
}
#[inline]
pub unsafe fn XInputGetKeystroke(dwuserindex: u32, dwreserved: Option<u32>, pkeystroke: *mut XINPUT_KEYSTROKE) -> u32 {
    windows_core::link!("xinput1_4.dll" "system" fn XInputGetKeystroke(dwuserindex : u32, dwreserved : u32, pkeystroke : *mut XINPUT_KEYSTROKE) -> u32);
    unsafe { XInputGetKeystroke(dwuserindex, dwreserved.unwrap_or(core::mem::zeroed()) as _, pkeystroke as _) }
}
#[inline]
pub unsafe fn XInputGetState(dwuserindex: u32, pstate: *mut XINPUT_STATE) -> u32 {
    windows_core::link!("xinput1_4.dll" "system" fn XInputGetState(dwuserindex : u32, pstate : *mut XINPUT_STATE) -> u32);
    unsafe { XInputGetState(dwuserindex, pstate as _) }
}
#[inline]
pub unsafe fn XInputSetState(dwuserindex: u32, pvibration: *const XINPUT_VIBRATION) -> u32 {
    windows_core::link!("xinput1_4.dll" "system" fn XInputSetState(dwuserindex : u32, pvibration : *const XINPUT_VIBRATION) -> u32);
    unsafe { XInputSetState(dwuserindex, pvibration) }
}
pub const BATTERY_DEVTYPE_GAMEPAD: u32 = 0;
pub const BATTERY_DEVTYPE_HEADSET: u32 = 1;
pub const BATTERY_LEVEL_EMPTY: u32 = 0;
pub const BATTERY_LEVEL_FULL: u32 = 3;
pub const BATTERY_LEVEL_LOW: u32 = 1;
pub const BATTERY_LEVEL_MEDIUM: u32 = 2;
pub const BATTERY_TYPE_ALKALINE: u32 = 2;
pub const BATTERY_TYPE_DISCONNECTED: u32 = 0;
pub const BATTERY_TYPE_NIMH: u32 = 3;
pub const BATTERY_TYPE_UNKNOWN: u32 = 255;
pub const BATTERY_TYPE_WIRED: u32 = 1;
pub type PXINPUT_BATTERY_INFORMATION = *mut XINPUT_BATTERY_INFORMATION;
pub type PXINPUT_CAPABILITIES = *mut XINPUT_CAPABILITIES;
pub type PXINPUT_GAMEPAD = *mut XINPUT_GAMEPAD;
pub type PXINPUT_KEYSTROKE = *mut XINPUT_KEYSTROKE;
pub type PXINPUT_STATE = *mut XINPUT_STATE;
pub type PXINPUT_VIBRATION = *mut XINPUT_VIBRATION;
pub const VK_PAD_A: u32 = 22528;
pub const VK_PAD_B: u32 = 22529;
pub const VK_PAD_BACK: u32 = 22549;
pub const VK_PAD_DPAD_DOWN: u32 = 22545;
pub const VK_PAD_DPAD_LEFT: u32 = 22546;
pub const VK_PAD_DPAD_RIGHT: u32 = 22547;
pub const VK_PAD_DPAD_UP: u32 = 22544;
pub const VK_PAD_LSHOULDER: u32 = 22533;
pub const VK_PAD_LTHUMB_DOWN: u32 = 22561;
pub const VK_PAD_LTHUMB_DOWNLEFT: u32 = 22567;
pub const VK_PAD_LTHUMB_DOWNRIGHT: u32 = 22566;
pub const VK_PAD_LTHUMB_LEFT: u32 = 22563;
pub const VK_PAD_LTHUMB_PRESS: u32 = 22550;
pub const VK_PAD_LTHUMB_RIGHT: u32 = 22562;
pub const VK_PAD_LTHUMB_UP: u32 = 22560;
pub const VK_PAD_LTHUMB_UPLEFT: u32 = 22564;
pub const VK_PAD_LTHUMB_UPRIGHT: u32 = 22565;
pub const VK_PAD_LTRIGGER: u32 = 22534;
pub const VK_PAD_RSHOULDER: u32 = 22532;
pub const VK_PAD_RTHUMB_DOWN: u32 = 22577;
pub const VK_PAD_RTHUMB_DOWNLEFT: u32 = 22583;
pub const VK_PAD_RTHUMB_DOWNRIGHT: u32 = 22582;
pub const VK_PAD_RTHUMB_LEFT: u32 = 22579;
pub const VK_PAD_RTHUMB_PRESS: u32 = 22551;
pub const VK_PAD_RTHUMB_RIGHT: u32 = 22578;
pub const VK_PAD_RTHUMB_UP: u32 = 22576;
pub const VK_PAD_RTHUMB_UPLEFT: u32 = 22580;
pub const VK_PAD_RTHUMB_UPRIGHT: u32 = 22581;
pub const VK_PAD_RTRIGGER: u32 = 22535;
pub const VK_PAD_START: u32 = 22548;
pub const VK_PAD_X: u32 = 22530;
pub const VK_PAD_Y: u32 = 22531;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XINPUT_BATTERY_INFORMATION {
    pub BatteryType: u8,
    pub BatteryLevel: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XINPUT_CAPABILITIES {
    pub Type: u8,
    pub SubType: u8,
    pub Flags: u16,
    pub Gamepad: XINPUT_GAMEPAD,
    pub Vibration: XINPUT_VIBRATION,
}
pub const XINPUT_CAPS_FFB_SUPPORTED: u32 = 1;
pub const XINPUT_CAPS_NO_NAVIGATION: u32 = 16;
pub const XINPUT_CAPS_PMD_SUPPORTED: u32 = 8;
pub const XINPUT_CAPS_VOICE_SUPPORTED: u32 = 4;
pub const XINPUT_CAPS_WIRELESS: u32 = 2;
pub const XINPUT_DEVSUBTYPE_ARCADE_PAD: u32 = 19;
pub const XINPUT_DEVSUBTYPE_ARCADE_STICK: u32 = 3;
pub const XINPUT_DEVSUBTYPE_DANCE_PAD: u32 = 5;
pub const XINPUT_DEVSUBTYPE_DRUM_KIT: u32 = 8;
pub const XINPUT_DEVSUBTYPE_FLIGHT_STICK: u32 = 4;
pub const XINPUT_DEVSUBTYPE_GAMEPAD: u32 = 1;
pub const XINPUT_DEVSUBTYPE_GUITAR: u32 = 6;
pub const XINPUT_DEVSUBTYPE_GUITAR_ALTERNATE: u32 = 7;
pub const XINPUT_DEVSUBTYPE_GUITAR_BASS: u32 = 11;
pub const XINPUT_DEVSUBTYPE_UNKNOWN: u32 = 0;
pub const XINPUT_DEVSUBTYPE_WHEEL: u32 = 2;
pub const XINPUT_DEVTYPE_GAMEPAD: u32 = 1;
pub const XINPUT_DLL_A: windows_core::PCSTR = windows_core::s!("xinput1_4.dll");
pub const XINPUT_DLL_W: windows_core::PCWSTR = windows_core::w!("xinput1_4.dll");
pub const XINPUT_FLAG_GAMEPAD: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XINPUT_GAMEPAD {
    pub wButtons: u16,
    pub bLeftTrigger: u8,
    pub bRightTrigger: u8,
    pub sThumbLX: i16,
    pub sThumbLY: i16,
    pub sThumbRX: i16,
    pub sThumbRY: i16,
}
pub const XINPUT_GAMEPAD_A: u32 = 4096;
pub const XINPUT_GAMEPAD_B: u32 = 8192;
pub const XINPUT_GAMEPAD_BACK: u32 = 32;
pub const XINPUT_GAMEPAD_DPAD_DOWN: u32 = 2;
pub const XINPUT_GAMEPAD_DPAD_LEFT: u32 = 4;
pub const XINPUT_GAMEPAD_DPAD_RIGHT: u32 = 8;
pub const XINPUT_GAMEPAD_DPAD_UP: u32 = 1;
pub const XINPUT_GAMEPAD_LEFT_SHOULDER: u32 = 256;
pub const XINPUT_GAMEPAD_LEFT_THUMB: u32 = 64;
pub const XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE: u32 = 7849;
pub const XINPUT_GAMEPAD_RIGHT_SHOULDER: u32 = 512;
pub const XINPUT_GAMEPAD_RIGHT_THUMB: u32 = 128;
pub const XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE: u32 = 8689;
pub const XINPUT_GAMEPAD_START: u32 = 16;
pub const XINPUT_GAMEPAD_TRIGGER_THRESHOLD: u32 = 30;
pub const XINPUT_GAMEPAD_X: u32 = 16384;
pub const XINPUT_GAMEPAD_Y: u32 = 32768;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XINPUT_KEYSTROKE {
    pub VirtualKey: u16,
    pub Unicode: u16,
    pub Flags: u16,
    pub UserIndex: u8,
    pub HidCode: u8,
}
pub const XINPUT_KEYSTROKE_KEYDOWN: u32 = 1;
pub const XINPUT_KEYSTROKE_KEYUP: u32 = 2;
pub const XINPUT_KEYSTROKE_REPEAT: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XINPUT_STATE {
    pub dwPacketNumber: u32,
    pub Gamepad: XINPUT_GAMEPAD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XINPUT_VIBRATION {
    pub wLeftMotorSpeed: u16,
    pub wRightMotorSpeed: u16,
}
pub const XUSER_INDEX_ANY: u32 = 255;
pub const XUSER_MAX_COUNT: u32 = 4;
