pub const DD_KEYBOARD_DEVICE_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("\\Device\\KeyboardClass");
pub const DD_KEYBOARD_DEVICE_NAME_U: windows_sys::core::PCWSTR = windows_sys::core::w!("\\Device\\KeyboardClass");
pub const GUID_DEVINTERFACE_KEYBOARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x884b96c3_56ef_11d1_bc8c_00a0c91405dd);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct INDICATOR_LIST {
    pub MakeCode: super::USHORT,
    pub IndicatorFlags: super::USHORT,
}
pub const IOCTL_KEYBOARD_INSERT_DATA: i32 = 721152;
pub const IOCTL_KEYBOARD_QUERY_ATTRIBUTES: i32 = 720896;
pub const IOCTL_KEYBOARD_QUERY_EXTENDED_ATTRIBUTES: i32 = 721408;
pub const IOCTL_KEYBOARD_QUERY_IME_STATUS: i32 = 724992;
pub const IOCTL_KEYBOARD_QUERY_INDICATORS: i32 = 720960;
pub const IOCTL_KEYBOARD_QUERY_INDICATOR_TRANSLATION: i32 = 721024;
pub const IOCTL_KEYBOARD_QUERY_TYPEMATIC: i32 = 720928;
pub const IOCTL_KEYBOARD_SET_IME_STATUS: i32 = 724996;
pub const IOCTL_KEYBOARD_SET_INDICATORS: i32 = 720904;
pub const IOCTL_KEYBOARD_SET_TYPEMATIC: i32 = 720900;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct KEYBOARD_ATTRIBUTES {
    pub KeyboardIdentifier: KEYBOARD_ID,
    pub KeyboardMode: super::USHORT,
    pub NumberOfFunctionKeys: super::USHORT,
    pub NumberOfIndicators: super::USHORT,
    pub NumberOfKeysTotal: super::USHORT,
    pub InputDataQueueLength: super::ULONG,
    pub KeyRepeatMinimum: KEYBOARD_TYPEMATIC_PARAMETERS,
    pub KeyRepeatMaximum: KEYBOARD_TYPEMATIC_PARAMETERS,
}
pub const KEYBOARD_CAPS_LOCK_ON: i32 = 4;
pub const KEYBOARD_ERROR_VALUE_BASE: i32 = 10000;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct KEYBOARD_EXTENDED_ATTRIBUTES {
    pub Version: super::UCHAR,
    pub FormFactor: super::UCHAR,
    pub KeyType: super::UCHAR,
    pub PhysicalLayout: super::UCHAR,
    pub VendorSpecificPhysicalLayout: super::UCHAR,
    pub IETFLanguageTagIndex: super::UCHAR,
    pub ImplementedInputAssistControls: super::UCHAR,
}
pub const KEYBOARD_EXTENDED_ATTRIBUTES_STRUCT_VERSION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct KEYBOARD_ID {
    pub Type: super::UCHAR,
    pub Subtype: super::UCHAR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct KEYBOARD_IME_STATUS {
    pub UnitId: super::USHORT,
    pub ImeOpen: super::ULONG,
    pub ImeConvMode: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct KEYBOARD_INDICATOR_PARAMETERS {
    pub UnitId: super::USHORT,
    pub LedFlags: super::USHORT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct KEYBOARD_INDICATOR_TRANSLATION {
    pub NumberOfIndicatorKeys: super::USHORT,
    pub IndicatorList: [INDICATOR_LIST; 1],
}
#[cfg(feature = "shared")]
impl Default for KEYBOARD_INDICATOR_TRANSLATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct KEYBOARD_INPUT_DATA {
    pub UnitId: super::USHORT,
    pub MakeCode: super::USHORT,
    pub Flags: super::USHORT,
    pub Reserved: super::USHORT,
    pub ExtraInformation: super::ULONG,
}
pub const KEYBOARD_KANA_LOCK_ON: i32 = 8;
pub const KEYBOARD_LED_INJECTED: i32 = 32768;
pub const KEYBOARD_NUM_LOCK_ON: i32 = 2;
pub const KEYBOARD_SCROLL_LOCK_ON: i32 = 1;
pub const KEYBOARD_SHADOW: i32 = 16384;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct KEYBOARD_TYPEMATIC_PARAMETERS {
    pub UnitId: super::USHORT,
    pub Rate: super::USHORT,
    pub Delay: super::USHORT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct KEYBOARD_UNIT_ID_PARAMETER {
    pub UnitId: super::USHORT,
}
pub const KEY_BREAK: i32 = 1;
pub const KEY_E0: i32 = 2;
pub const KEY_E1: i32 = 4;
pub const KEY_FROM_KEYBOARD_OVERRIDER: i32 = 128;
pub const KEY_MAKE: i32 = 0;
pub const KEY_RIM_VKEY: i32 = 64;
pub const KEY_TERMSRV_SET_LED: i32 = 8;
pub const KEY_TERMSRV_SHADOW: i32 = 16;
pub const KEY_TERMSRV_VKPACKET: i32 = 32;
pub const KEY_UNICODE_SEQUENCE_END: i32 = 512;
pub const KEY_UNICODE_SEQUENCE_ITEM: i32 = 256;
#[cfg(feature = "shared")]
pub type PINDICATOR_LIST = *mut INDICATOR_LIST;
#[cfg(feature = "shared")]
pub type PKEYBOARD_ATTRIBUTES = *mut KEYBOARD_ATTRIBUTES;
#[cfg(feature = "shared")]
pub type PKEYBOARD_EXTENDED_ATTRIBUTES = *mut KEYBOARD_EXTENDED_ATTRIBUTES;
#[cfg(feature = "shared")]
pub type PKEYBOARD_ID = *mut KEYBOARD_ID;
#[cfg(feature = "shared")]
pub type PKEYBOARD_IME_STATUS = *mut KEYBOARD_IME_STATUS;
#[cfg(feature = "shared")]
pub type PKEYBOARD_INDICATOR_PARAMETERS = *mut KEYBOARD_INDICATOR_PARAMETERS;
#[cfg(feature = "shared")]
pub type PKEYBOARD_INDICATOR_TRANSLATION = *mut KEYBOARD_INDICATOR_TRANSLATION;
#[cfg(feature = "shared")]
pub type PKEYBOARD_INPUT_DATA = *mut KEYBOARD_INPUT_DATA;
#[cfg(feature = "shared")]
pub type PKEYBOARD_TYPEMATIC_PARAMETERS = *mut KEYBOARD_TYPEMATIC_PARAMETERS;
#[cfg(feature = "shared")]
pub type PKEYBOARD_UNIT_ID_PARAMETER = *mut KEYBOARD_UNIT_ID_PARAMETER;
