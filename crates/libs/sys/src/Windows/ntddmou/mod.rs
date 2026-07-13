pub const BALLPOINT_I8042_HARDWARE: u32 = 8;
pub const BALLPOINT_SERIAL_HARDWARE: u32 = 16;
pub const DD_MOUSE_DEVICE_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("\\Device\\PointerClass");
pub const DD_MOUSE_DEVICE_NAME_U: windows_sys::core::PCWSTR = windows_sys::core::w!("\\Device\\PointerClass");
pub const GUID_DEVINTERFACE_MOUSE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x378de44c_56ef_11d1_bc8c_00a0c91405dd);
pub const HORIZONTAL_WHEEL_PRESENT: u32 = 32768;
pub const IOCTL_MOUSE_INSERT_DATA: u32 = 983044;
pub const IOCTL_MOUSE_QUERY_ATTRIBUTES: u32 = 983040;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MOUSE_ATTRIBUTES {
    pub MouseIdentifier: u16,
    pub NumberOfButtons: u16,
    pub SampleRate: u16,
    pub InputDataQueueLength: u32,
}
pub const MOUSE_BUTTON_1_DOWN: u32 = 1;
pub const MOUSE_BUTTON_1_UP: u32 = 2;
pub const MOUSE_BUTTON_2_DOWN: u32 = 4;
pub const MOUSE_BUTTON_2_UP: u32 = 8;
pub const MOUSE_BUTTON_3_DOWN: u32 = 16;
pub const MOUSE_BUTTON_3_UP: u32 = 32;
pub const MOUSE_BUTTON_4_DOWN: u32 = 64;
pub const MOUSE_BUTTON_4_UP: u32 = 128;
pub const MOUSE_BUTTON_5_DOWN: u32 = 256;
pub const MOUSE_BUTTON_5_UP: u32 = 512;
pub const MOUSE_ERROR_VALUE_BASE: u32 = 20000;
pub const MOUSE_HID_HARDWARE: u32 = 128;
pub const MOUSE_HWHEEL: u32 = 2048;
pub const MOUSE_I8042_HARDWARE: u32 = 2;
pub const MOUSE_INPORT_HARDWARE: u32 = 1;
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
#[derive(Clone, Copy, Default)]
pub struct MOUSE_INPUT_DATA_0_0 {
    pub ButtonFlags: u16,
    pub ButtonData: u16,
}
pub const MOUSE_LEFT_BUTTON_DOWN: u32 = 1;
pub const MOUSE_LEFT_BUTTON_UP: u32 = 2;
pub const MOUSE_MIDDLE_BUTTON_DOWN: u32 = 16;
pub const MOUSE_MIDDLE_BUTTON_UP: u32 = 32;
pub const MOUSE_RIGHT_BUTTON_DOWN: u32 = 4;
pub const MOUSE_RIGHT_BUTTON_UP: u32 = 8;
pub const MOUSE_SERIAL_HARDWARE: u32 = 4;
pub const MOUSE_TERMSRV_SRC_SHADOW: u32 = 256;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MOUSE_UNIT_ID_PARAMETER {
    pub UnitId: u16,
}
pub const MOUSE_WHEEL: u32 = 1024;
pub type PMOUSE_ATTRIBUTES = *mut MOUSE_ATTRIBUTES;
pub type PMOUSE_INPUT_DATA = *mut MOUSE_INPUT_DATA;
pub type PMOUSE_UNIT_ID_PARAMETER = *mut MOUSE_UNIT_ID_PARAMETER;
pub const WHEELMOUSE_HID_HARDWARE: u32 = 256;
pub const WHEELMOUSE_I8042_HARDWARE: u32 = 32;
pub const WHEELMOUSE_SERIAL_HARDWARE: u32 = 64;
