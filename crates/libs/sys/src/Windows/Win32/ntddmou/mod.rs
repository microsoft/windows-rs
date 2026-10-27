pub const BALLPOINT_I8042_HARDWARE: i32 = 8;
pub const BALLPOINT_SERIAL_HARDWARE: i32 = 16;
pub const DD_MOUSE_DEVICE_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("\\Device\\PointerClass");
pub const DD_MOUSE_DEVICE_NAME_U: windows_sys::core::PCWSTR = windows_sys::core::w!("\\Device\\PointerClass");
pub const GUID_DEVINTERFACE_MOUSE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x378de44c_56ef_11d1_bc8c_00a0c91405dd);
pub const HORIZONTAL_WHEEL_PRESENT: i32 = 32768;
pub const IOCTL_MOUSE_INSERT_DATA: i32 = 983044;
pub const IOCTL_MOUSE_QUERY_ATTRIBUTES: i32 = 983040;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MOUSE_ATTRIBUTES {
    pub MouseIdentifier: super::USHORT,
    pub NumberOfButtons: super::USHORT,
    pub SampleRate: super::USHORT,
    pub InputDataQueueLength: super::ULONG,
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MOUSE_INPUT_DATA {
    pub UnitId: super::USHORT,
    pub Flags: super::USHORT,
    pub Anonymous: MOUSE_INPUT_DATA_0,
    pub RawButtons: super::ULONG,
    pub LastX: super::LONG,
    pub LastY: super::LONG,
    pub ExtraInformation: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for MOUSE_INPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MOUSE_INPUT_DATA_0 {
    pub Buttons: super::ULONG,
    pub Anonymous: MOUSE_INPUT_DATA_0_0,
}
#[cfg(feature = "shared")]
impl Default for MOUSE_INPUT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MOUSE_INPUT_DATA_0_0 {
    pub ButtonFlags: super::USHORT,
    pub ButtonData: super::USHORT,
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MOUSE_UNIT_ID_PARAMETER {
    pub UnitId: super::USHORT,
}
pub const MOUSE_WHEEL: i32 = 1024;
#[cfg(feature = "shared")]
pub type PMOUSE_ATTRIBUTES = *mut MOUSE_ATTRIBUTES;
#[cfg(feature = "shared")]
pub type PMOUSE_INPUT_DATA = *mut MOUSE_INPUT_DATA;
#[cfg(feature = "shared")]
pub type PMOUSE_UNIT_ID_PARAMETER = *mut MOUSE_UNIT_ID_PARAMETER;
pub const WHEELMOUSE_HID_HARDWARE: i32 = 256;
pub const WHEELMOUSE_I8042_HARDWARE: i32 = 32;
pub const WHEELMOUSE_SERIAL_HARDWARE: i32 = 64;
