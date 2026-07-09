pub const DD_KEYBOARD_DEVICE_NAME: windows_core::PCSTR = windows_core::s!("\\Device\\KeyboardClass");
pub const DD_KEYBOARD_DEVICE_NAME_U: windows_core::PCWSTR = windows_core::w!("\\Device\\KeyboardClass");
pub const GUID_DEVINTERFACE_KEYBOARD: windows_core::GUID = windows_core::GUID::from_u128(0x884b96c3_56ef_11d1_bc8c_00a0c91405dd);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INDICATOR_LIST {
    pub MakeCode: u16,
    pub IndicatorFlags: u16,
}
pub const IOCTL_KEYBOARD_INSERT_DATA: u32 = 721152;
pub const IOCTL_KEYBOARD_QUERY_ATTRIBUTES: u32 = 720896;
pub const IOCTL_KEYBOARD_QUERY_EXTENDED_ATTRIBUTES: u32 = 721408;
pub const IOCTL_KEYBOARD_QUERY_IME_STATUS: u32 = 724992;
pub const IOCTL_KEYBOARD_QUERY_INDICATORS: u32 = 720960;
pub const IOCTL_KEYBOARD_QUERY_INDICATOR_TRANSLATION: u32 = 721024;
pub const IOCTL_KEYBOARD_QUERY_TYPEMATIC: u32 = 720928;
pub const IOCTL_KEYBOARD_SET_IME_STATUS: u32 = 724996;
pub const IOCTL_KEYBOARD_SET_INDICATORS: u32 = 720904;
pub const IOCTL_KEYBOARD_SET_TYPEMATIC: u32 = 720900;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEYBOARD_ATTRIBUTES {
    pub KeyboardIdentifier: KEYBOARD_ID,
    pub KeyboardMode: u16,
    pub NumberOfFunctionKeys: u16,
    pub NumberOfIndicators: u16,
    pub NumberOfKeysTotal: u16,
    pub InputDataQueueLength: u32,
    pub KeyRepeatMinimum: KEYBOARD_TYPEMATIC_PARAMETERS,
    pub KeyRepeatMaximum: KEYBOARD_TYPEMATIC_PARAMETERS,
}
pub const KEYBOARD_CAPS_LOCK_ON: u32 = 4;
pub const KEYBOARD_ERROR_VALUE_BASE: u32 = 10000;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEYBOARD_EXTENDED_ATTRIBUTES {
    pub Version: u8,
    pub FormFactor: u8,
    pub KeyType: u8,
    pub PhysicalLayout: u8,
    pub VendorSpecificPhysicalLayout: u8,
    pub IETFLanguageTagIndex: u8,
    pub ImplementedInputAssistControls: u8,
}
pub const KEYBOARD_EXTENDED_ATTRIBUTES_STRUCT_VERSION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEYBOARD_ID {
    pub Type: u8,
    pub Subtype: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEYBOARD_IME_STATUS {
    pub UnitId: u16,
    pub ImeOpen: u32,
    pub ImeConvMode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEYBOARD_INDICATOR_PARAMETERS {
    pub UnitId: u16,
    pub LedFlags: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KEYBOARD_INDICATOR_TRANSLATION {
    pub NumberOfIndicatorKeys: u16,
    pub IndicatorList: [INDICATOR_LIST; 1],
}
impl Default for KEYBOARD_INDICATOR_TRANSLATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEYBOARD_INPUT_DATA {
    pub UnitId: u16,
    pub MakeCode: u16,
    pub Flags: u16,
    pub Reserved: u16,
    pub ExtraInformation: u32,
}
pub const KEYBOARD_KANA_LOCK_ON: u32 = 8;
pub const KEYBOARD_LED_INJECTED: u32 = 32768;
pub const KEYBOARD_NUM_LOCK_ON: u32 = 2;
pub const KEYBOARD_SCROLL_LOCK_ON: u32 = 1;
pub const KEYBOARD_SHADOW: u32 = 16384;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEYBOARD_TYPEMATIC_PARAMETERS {
    pub UnitId: u16,
    pub Rate: u16,
    pub Delay: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEYBOARD_UNIT_ID_PARAMETER {
    pub UnitId: u16,
}
pub const KEY_BREAK: u32 = 1;
pub const KEY_E0: u32 = 2;
pub const KEY_E1: u32 = 4;
pub const KEY_FROM_KEYBOARD_OVERRIDER: u32 = 128;
pub const KEY_MAKE: u32 = 0;
pub const KEY_RIM_VKEY: u32 = 64;
pub const KEY_TERMSRV_SET_LED: u32 = 8;
pub const KEY_TERMSRV_SHADOW: u32 = 16;
pub const KEY_TERMSRV_VKPACKET: u32 = 32;
pub const KEY_UNICODE_SEQUENCE_END: u32 = 512;
pub const KEY_UNICODE_SEQUENCE_ITEM: u32 = 256;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PINDICATOR_LIST(pub *mut INDICATOR_LIST);
impl PINDICATOR_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PINDICATOR_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKEYBOARD_ATTRIBUTES(pub *mut KEYBOARD_ATTRIBUTES);
impl PKEYBOARD_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PKEYBOARD_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKEYBOARD_EXTENDED_ATTRIBUTES(pub *mut KEYBOARD_EXTENDED_ATTRIBUTES);
impl PKEYBOARD_EXTENDED_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PKEYBOARD_EXTENDED_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKEYBOARD_ID(pub *mut KEYBOARD_ID);
impl PKEYBOARD_ID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PKEYBOARD_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKEYBOARD_IME_STATUS(pub *mut KEYBOARD_IME_STATUS);
impl PKEYBOARD_IME_STATUS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PKEYBOARD_IME_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKEYBOARD_INDICATOR_PARAMETERS(pub *mut KEYBOARD_INDICATOR_PARAMETERS);
impl PKEYBOARD_INDICATOR_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PKEYBOARD_INDICATOR_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKEYBOARD_INDICATOR_TRANSLATION(pub *mut KEYBOARD_INDICATOR_TRANSLATION);
impl PKEYBOARD_INDICATOR_TRANSLATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PKEYBOARD_INDICATOR_TRANSLATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKEYBOARD_INPUT_DATA(pub *mut KEYBOARD_INPUT_DATA);
impl PKEYBOARD_INPUT_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PKEYBOARD_INPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKEYBOARD_TYPEMATIC_PARAMETERS(pub *mut KEYBOARD_TYPEMATIC_PARAMETERS);
impl PKEYBOARD_TYPEMATIC_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PKEYBOARD_TYPEMATIC_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKEYBOARD_UNIT_ID_PARAMETER(pub *mut KEYBOARD_UNIT_ID_PARAMETER);
impl PKEYBOARD_UNIT_ID_PARAMETER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PKEYBOARD_UNIT_ID_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
