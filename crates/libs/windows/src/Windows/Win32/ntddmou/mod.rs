pub const BALLPOINT_I8042_HARDWARE: i32 = 8;
pub const BALLPOINT_SERIAL_HARDWARE: i32 = 16;
pub const DD_MOUSE_DEVICE_NAME: windows_core::PCSTR = windows_core::s!("\\Device\\PointerClass");
pub const DD_MOUSE_DEVICE_NAME_U: windows_core::PCWSTR = windows_core::w!("\\Device\\PointerClass");
pub const GUID_DEVINTERFACE_MOUSE: windows_core::GUID = windows_core::GUID::from_u128(0x378de44c_56ef_11d1_bc8c_00a0c91405dd);
pub const HORIZONTAL_WHEEL_PRESENT: i32 = 32768;
pub const IOCTL_MOUSE_INSERT_DATA: i32 = 983044;
pub const IOCTL_MOUSE_QUERY_ATTRIBUTES: i32 = 983040;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOUSE_ATTRIBUTES {
    pub MouseIdentifier: u16,
    pub NumberOfButtons: u16,
    pub SampleRate: u16,
    pub InputDataQueueLength: u32,
}
pub const MOUSE_BUTTON_1_DOWN: i32 = 1;
pub const MOUSE_BUTTON_1_UP: i32 = 2;
pub const MOUSE_BUTTON_2_DOWN: i32 = 4;
pub const MOUSE_BUTTON_2_UP: i32 = 8;
pub const MOUSE_BUTTON_3_DOWN: i32 = 16;
pub const MOUSE_BUTTON_3_UP: i32 = 32;
pub const MOUSE_BUTTON_4_DOWN: i32 = 64;
pub const MOUSE_BUTTON_4_UP: i32 = 128;
pub const MOUSE_BUTTON_5_DOWN: i32 = 256;
pub const MOUSE_BUTTON_5_UP: i32 = 512;
pub const MOUSE_ERROR_VALUE_BASE: i32 = 20000;
pub const MOUSE_HID_HARDWARE: i32 = 128;
pub const MOUSE_HWHEEL: i32 = 2048;
pub const MOUSE_I8042_HARDWARE: i32 = 2;
pub const MOUSE_INPORT_HARDWARE: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MOUSE_INPUT_DATA {
    pub UnitId: u16,
    pub Flags: u16,
    pub Anonymous: MOUSE_INPUT_DATA_0,
    pub RawButtons: u32,
    pub LastX: i32,
    pub LastY: i32,
    pub ExtraInformation: u32,
}
impl Default for MOUSE_INPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MOUSE_INPUT_DATA_0 {
    pub Buttons: u32,
    pub Anonymous: MOUSE_INPUT_DATA_0_0,
}
impl Default for MOUSE_INPUT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOUSE_INPUT_DATA_0_0 {
    pub ButtonFlags: u16,
    pub ButtonData: u16,
}
pub const MOUSE_LEFT_BUTTON_DOWN: i32 = 1;
pub const MOUSE_LEFT_BUTTON_UP: i32 = 2;
pub const MOUSE_MIDDLE_BUTTON_DOWN: i32 = 16;
pub const MOUSE_MIDDLE_BUTTON_UP: i32 = 32;
pub const MOUSE_RIGHT_BUTTON_DOWN: i32 = 4;
pub const MOUSE_RIGHT_BUTTON_UP: i32 = 8;
pub const MOUSE_SERIAL_HARDWARE: i32 = 4;
pub const MOUSE_TERMSRV_SRC_SHADOW: i32 = 256;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOUSE_UNIT_ID_PARAMETER {
    pub UnitId: u16,
}
pub const MOUSE_WHEEL: i32 = 1024;
pub type PMOUSE_ATTRIBUTES = *mut MOUSE_ATTRIBUTES;
pub type PMOUSE_INPUT_DATA = *mut MOUSE_INPUT_DATA;
pub type PMOUSE_UNIT_ID_PARAMETER = *mut MOUSE_UNIT_ID_PARAMETER;
pub const WHEELMOUSE_HID_HARDWARE: i32 = 256;
pub const WHEELMOUSE_I8042_HARDWARE: i32 = 32;
pub const WHEELMOUSE_SERIAL_HARDWARE: i32 = 64;
