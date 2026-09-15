#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_GetButtonArray(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : u16, usage : super::USAGE, buttondata : PHIDP_BUTTON_ARRAY_DATA, buttondatalength : super::PUSHORT, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : u32) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_GetButtonCaps(reporttype : HIDP_REPORT_TYPE, buttoncaps : PHIDP_BUTTON_CAPS, buttoncapslength : super::PUSHORT, preparseddata : PHIDP_PREPARSED_DATA) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "hidusage")]
windows_link::link!("hid.dll" "system" fn HidP_GetCaps(preparseddata : PHIDP_PREPARSED_DATA, capabilities : PHIDP_CAPS) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_GetData(reporttype : HIDP_REPORT_TYPE, datalist : PHIDP_DATA, datalength : super::PULONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : u32) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "minwindef")]
windows_link::link!("hid.dll" "system" fn HidP_GetExtendedAttributes(reporttype : HIDP_REPORT_TYPE, dataindex : u16, preparseddata : PHIDP_PREPARSED_DATA, attributes : PHIDP_EXTENDED_ATTRIBUTES, lengthattributes : super::PULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "hidusage", feature = "minwindef"))]
windows_link::link!("hid.dll" "system" fn HidP_GetLinkCollectionNodes(linkcollectionnodes : PHIDP_LINK_COLLECTION_NODE, linkcollectionnodeslength : super::PULONG, preparseddata : PHIDP_PREPARSED_DATA) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "hidusage", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_GetScaledUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : u16, usage : super::USAGE, usagevalue : super::PLONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : u32) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_GetSpecificButtonCaps(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : u16, usage : super::USAGE, buttoncaps : PHIDP_BUTTON_CAPS, buttoncapslength : super::PUSHORT, preparseddata : PHIDP_PREPARSED_DATA) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_GetSpecificValueCaps(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : u16, usage : super::USAGE, valuecaps : PHIDP_VALUE_CAPS, valuecapslength : super::PUSHORT, preparseddata : PHIDP_PREPARSED_DATA) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_GetUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : u16, usage : super::USAGE, usagevalue : super::PULONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : u32) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "hidusage", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_GetUsageValueArray(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : u16, usage : super::USAGE, usagevalue : super::PCHAR, usagevaluebytelength : u16, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : u32) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_GetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : u16, usagelist : super::PUSAGE, usagelength : super::PULONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : u32) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "hidusage", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_GetUsagesEx(reporttype : HIDP_REPORT_TYPE, linkcollection : u16, buttonlist : PUSAGE_AND_PAGE, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : u32) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_GetValueCaps(reporttype : HIDP_REPORT_TYPE, valuecaps : PHIDP_VALUE_CAPS, valuecapslength : super::PUSHORT, preparseddata : PHIDP_PREPARSED_DATA) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidP_InitializeReportForID(reporttype : HIDP_REPORT_TYPE, reportid : u8, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : u32) -> windows_sys::core::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_MaxDataListLength(reporttype : HIDP_REPORT_TYPE, preparseddata : PHIDP_PREPARSED_DATA) -> u32);
#[cfg(feature = "hidusage")]
windows_link::link!("hid.dll" "system" fn HidP_MaxUsageListLength(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, preparseddata : PHIDP_PREPARSED_DATA) -> u32);
#[cfg(all(feature = "hidusage", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_SetButtonArray(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : u16, usage : super::USAGE, buttondata : PHIDP_BUTTON_ARRAY_DATA, buttondatalength : u16, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : u32) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_SetData(reporttype : HIDP_REPORT_TYPE, datalist : PHIDP_DATA, datalength : super::PULONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : u32) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "hidusage", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_SetScaledUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : u16, usage : super::USAGE, usagevalue : i32, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : u32) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "hidusage", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_SetUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : u16, usage : super::USAGE, usagevalue : u32, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : u32) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "hidusage", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_SetUsageValueArray(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : u16, usage : super::USAGE, usagevalue : super::PCHAR, usagevaluebytelength : u16, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : u32) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_SetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : u16, usagelist : super::PUSAGE, usagelength : super::PULONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : u32) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "hidusage", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_TranslateUsagesToI8042ScanCodes(changedusagelist : super::PUSAGE, usagelistlength : u32, keyaction : HIDP_KEYBOARD_DIRECTION, modifierstate : PHIDP_KEYBOARD_MODIFIER_STATE, insertcodesprocedure : PHIDP_INSERT_SCANCODES, insertcodescontext : *const core::ffi::c_void) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidP_UnsetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : u16, usagelist : super::PUSAGE, usagelength : super::PULONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : u32) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "hidusage")]
windows_link::link!("hid.dll" "system" fn HidP_UsageListDifference(previoususagelist : super::PUSAGE, currentusagelist : super::PUSAGE, breakusagelist : super::PUSAGE, makeusagelist : super::PUSAGE, usagelistlength : u32) -> windows_sys::core::NTSTATUS);
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct HIDP_BUTTON_ARRAY_DATA {
    pub ArrayIndex: u16,
    pub On: super::BOOLEAN,
}
#[repr(C)]
#[cfg(all(feature = "hidusage", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct HIDP_BUTTON_CAPS {
    pub UsagePage: super::USAGE,
    pub ReportID: u8,
    pub IsAlias: super::BOOLEAN,
    pub BitField: u16,
    pub LinkCollection: u16,
    pub LinkUsage: super::USAGE,
    pub LinkUsagePage: super::USAGE,
    pub IsRange: super::BOOLEAN,
    pub IsStringRange: super::BOOLEAN,
    pub IsDesignatorRange: super::BOOLEAN,
    pub IsAbsolute: super::BOOLEAN,
    pub ReportCount: u16,
    pub Reserved2: u16,
    pub Reserved: [u32; 9],
    pub Anonymous: HIDP_BUTTON_CAPS_0,
}
#[cfg(all(feature = "hidusage", feature = "winnt"))]
impl Default for HIDP_BUTTON_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "hidusage", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union HIDP_BUTTON_CAPS_0 {
    pub Range: HIDP_BUTTON_CAPS_0_0,
    pub NotRange: HIDP_BUTTON_CAPS_0_1,
}
#[cfg(all(feature = "hidusage", feature = "winnt"))]
impl Default for HIDP_BUTTON_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "hidusage", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct HIDP_BUTTON_CAPS_0_0 {
    pub UsageMin: super::USAGE,
    pub UsageMax: super::USAGE,
    pub StringMin: u16,
    pub StringMax: u16,
    pub DesignatorMin: u16,
    pub DesignatorMax: u16,
    pub DataIndexMin: u16,
    pub DataIndexMax: u16,
}
#[repr(C)]
#[cfg(all(feature = "hidusage", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct HIDP_BUTTON_CAPS_0_1 {
    pub Usage: super::USAGE,
    pub Reserved1: super::USAGE,
    pub StringIndex: u16,
    pub Reserved2: u16,
    pub DesignatorIndex: u16,
    pub Reserved3: u16,
    pub DataIndex: u16,
    pub Reserved4: u16,
}
#[repr(C)]
#[cfg(feature = "hidusage")]
#[derive(Clone, Copy)]
pub struct HIDP_CAPS {
    pub Usage: super::USAGE,
    pub UsagePage: super::USAGE,
    pub InputReportByteLength: u16,
    pub OutputReportByteLength: u16,
    pub FeatureReportByteLength: u16,
    pub Reserved: [u16; 17],
    pub NumberLinkCollectionNodes: u16,
    pub NumberInputButtonCaps: u16,
    pub NumberInputValueCaps: u16,
    pub NumberInputDataIndices: u16,
    pub NumberOutputButtonCaps: u16,
    pub NumberOutputValueCaps: u16,
    pub NumberOutputDataIndices: u16,
    pub NumberFeatureButtonCaps: u16,
    pub NumberFeatureValueCaps: u16,
    pub NumberFeatureDataIndices: u16,
}
#[cfg(feature = "hidusage")]
impl Default for HIDP_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct HIDP_DATA {
    pub DataIndex: u16,
    pub Reserved: u16,
    pub Anonymous: HIDP_DATA_0,
}
#[cfg(feature = "winnt")]
impl Default for HIDP_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union HIDP_DATA_0 {
    pub RawValue: u32,
    pub On: super::BOOLEAN,
}
#[cfg(feature = "winnt")]
impl Default for HIDP_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct HIDP_EXTENDED_ATTRIBUTES {
    pub NumGlobalUnknowns: u8,
    pub Reserved: [u8; 3],
    pub GlobalUnknowns: PHIDP_UNKNOWN_TOKEN,
    pub Data: [u32; 1],
}
#[cfg(target_arch = "x86")]
impl Default for HIDP_EXTENDED_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct HIDP_EXTENDED_ATTRIBUTES {
    pub NumGlobalUnknowns: u8,
    pub Reserved: [u8; 3],
    pub GlobalUnknowns: PHIDP_UNKNOWN_TOKEN,
    pub Data: [u32; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for HIDP_EXTENDED_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HIDP_KEYBOARD_DIRECTION = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HIDP_KEYBOARD_MODIFIER_STATE {
    pub Anonymous: HIDP_KEYBOARD_MODIFIER_STATE_0,
}
impl Default for HIDP_KEYBOARD_MODIFIER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HIDP_KEYBOARD_MODIFIER_STATE_0 {
    pub Anonymous: HIDP_KEYBOARD_MODIFIER_STATE_0_0,
    pub ul: u32,
}
impl Default for HIDP_KEYBOARD_MODIFIER_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HIDP_KEYBOARD_MODIFIER_STATE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "hidusage")]
#[derive(Clone, Copy, Default)]
pub struct HIDP_LINK_COLLECTION_NODE {
    pub LinkUsage: super::USAGE,
    pub LinkUsagePage: super::USAGE,
    pub Parent: u16,
    pub NumberOfChildren: u16,
    pub NextSibling: u16,
    pub FirstChild: u16,
    pub _bitfield: u32,
    pub UserContext: *mut core::ffi::c_void,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "hidusage")]
#[derive(Clone, Copy, Default)]
pub struct HIDP_LINK_COLLECTION_NODE {
    pub LinkUsage: super::USAGE,
    pub LinkUsagePage: super::USAGE,
    pub Parent: u16,
    pub NumberOfChildren: u16,
    pub NextSibling: u16,
    pub FirstChild: u16,
    pub _bitfield: u32,
    pub UserContext: *mut core::ffi::c_void,
}
pub const HIDP_LINK_COLLECTION_ROOT: u16 = 65535;
pub const HIDP_LINK_COLLECTION_UNSPECIFIED: u16 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HIDP_PREPARSED_DATA(pub u8);
pub type HIDP_REPORT_TYPE = i32;
pub const HIDP_STATUS_BAD_LOG_PHY_VALUES: windows_sys::core::NTSTATUS = 0xC0110006_u32 as _;
pub const HIDP_STATUS_BUFFER_TOO_SMALL: windows_sys::core::NTSTATUS = 0xC0110007_u32 as _;
pub const HIDP_STATUS_BUTTON_NOT_PRESSED: windows_sys::core::NTSTATUS = 0xC011000F_u32 as _;
pub const HIDP_STATUS_DATA_INDEX_NOT_FOUND: windows_sys::core::NTSTATUS = 0xC011000D_u32 as _;
pub const HIDP_STATUS_DATA_INDEX_OUT_OF_RANGE: windows_sys::core::NTSTATUS = 0xC011000E_u32 as _;
pub const HIDP_STATUS_I8042_TRANS_UNKNOWN: windows_sys::core::NTSTATUS = 0xC0110009_u32 as _;
pub const HIDP_STATUS_I8242_TRANS_UNKNOWN: windows_sys::core::NTSTATUS = 0xC0110009_u32 as _;
pub const HIDP_STATUS_INCOMPATIBLE_REPORT_ID: windows_sys::core::NTSTATUS = 0xC011000A_u32 as _;
pub const HIDP_STATUS_INTERNAL_ERROR: windows_sys::core::NTSTATUS = 0xC0110008_u32 as _;
pub const HIDP_STATUS_INVALID_PREPARSED_DATA: windows_sys::core::NTSTATUS = 0xC0110001_u32 as _;
pub const HIDP_STATUS_INVALID_REPORT_LENGTH: windows_sys::core::NTSTATUS = 0xC0110003_u32 as _;
pub const HIDP_STATUS_INVALID_REPORT_TYPE: windows_sys::core::NTSTATUS = 0xC0110002_u32 as _;
pub const HIDP_STATUS_IS_VALUE_ARRAY: windows_sys::core::NTSTATUS = 0xC011000C_u32 as _;
pub const HIDP_STATUS_NOT_BUTTON_ARRAY: windows_sys::core::NTSTATUS = 0xC0110021_u32 as _;
pub const HIDP_STATUS_NOT_IMPLEMENTED: windows_sys::core::NTSTATUS = 0xC0110020_u32 as _;
pub const HIDP_STATUS_NOT_VALUE_ARRAY: windows_sys::core::NTSTATUS = 0xC011000B_u32 as _;
pub const HIDP_STATUS_NULL: windows_sys::core::NTSTATUS = 0x80110001_u32 as _;
pub const HIDP_STATUS_REPORT_DOES_NOT_EXIST: windows_sys::core::NTSTATUS = 0xC0110010_u32 as _;
pub const HIDP_STATUS_SUCCESS: windows_sys::core::NTSTATUS = 0x110000_u32 as _;
pub const HIDP_STATUS_USAGE_NOT_FOUND: windows_sys::core::NTSTATUS = 0xC0110004_u32 as _;
pub const HIDP_STATUS_VALUE_OUT_OF_RANGE: windows_sys::core::NTSTATUS = 0xC0110005_u32 as _;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HIDP_UNKNOWN_TOKEN {
    pub Token: u8,
    pub Reserved: [u8; 3],
    pub BitField: u32,
}
impl Default for HIDP_UNKNOWN_TOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "hidusage", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct HIDP_VALUE_CAPS {
    pub UsagePage: super::USAGE,
    pub ReportID: u8,
    pub IsAlias: super::BOOLEAN,
    pub BitField: u16,
    pub LinkCollection: u16,
    pub LinkUsage: super::USAGE,
    pub LinkUsagePage: super::USAGE,
    pub IsRange: super::BOOLEAN,
    pub IsStringRange: super::BOOLEAN,
    pub IsDesignatorRange: super::BOOLEAN,
    pub IsAbsolute: super::BOOLEAN,
    pub HasNull: super::BOOLEAN,
    pub Reserved: u8,
    pub BitSize: u16,
    pub ReportCount: u16,
    pub Reserved2: [u16; 5],
    pub UnitsExp: u32,
    pub Units: u32,
    pub LogicalMin: i32,
    pub LogicalMax: i32,
    pub PhysicalMin: i32,
    pub PhysicalMax: i32,
    pub Anonymous: HIDP_VALUE_CAPS_0,
}
#[cfg(all(feature = "hidusage", feature = "winnt"))]
impl Default for HIDP_VALUE_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "hidusage", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union HIDP_VALUE_CAPS_0 {
    pub Range: HIDP_VALUE_CAPS_0_0,
    pub NotRange: HIDP_VALUE_CAPS_0_1,
}
#[cfg(all(feature = "hidusage", feature = "winnt"))]
impl Default for HIDP_VALUE_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "hidusage", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct HIDP_VALUE_CAPS_0_0 {
    pub UsageMin: super::USAGE,
    pub UsageMax: super::USAGE,
    pub StringMin: u16,
    pub StringMax: u16,
    pub DesignatorMin: u16,
    pub DesignatorMax: u16,
    pub DataIndexMin: u16,
    pub DataIndexMax: u16,
}
#[repr(C)]
#[cfg(all(feature = "hidusage", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct HIDP_VALUE_CAPS_0_1 {
    pub Usage: super::USAGE,
    pub Reserved1: super::USAGE,
    pub StringIndex: u16,
    pub Reserved2: u16,
    pub DesignatorIndex: u16,
    pub Reserved3: u16,
    pub DataIndex: u16,
    pub Reserved4: u16,
}
pub const HidP_Feature: HIDP_REPORT_TYPE = 2;
pub const HidP_Input: HIDP_REPORT_TYPE = 0;
pub const HidP_Keyboard_Break: HIDP_KEYBOARD_DIRECTION = 0;
pub const HidP_Keyboard_Make: HIDP_KEYBOARD_DIRECTION = 1;
pub const HidP_Output: HIDP_REPORT_TYPE = 1;
pub type PFN_HidP_GetVersionInternal = Option<unsafe extern "C" fn(version: *mut u32) -> windows_sys::core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PHIDP_BUTTON_ARRAY_DATA = *mut HIDP_BUTTON_ARRAY_DATA;
#[cfg(all(feature = "hidusage", feature = "winnt"))]
pub type PHIDP_BUTTON_CAPS = *mut HIDP_BUTTON_CAPS;
#[cfg(feature = "hidusage")]
pub type PHIDP_CAPS = *mut HIDP_CAPS;
#[cfg(feature = "winnt")]
pub type PHIDP_DATA = *mut HIDP_DATA;
pub type PHIDP_EXTENDED_ATTRIBUTES = *mut HIDP_EXTENDED_ATTRIBUTES;
#[cfg(feature = "winnt")]
pub type PHIDP_INSERT_SCANCODES = Option<unsafe extern "C" fn(context: *const core::ffi::c_void, newscancodes: super::PCHAR, length: u32) -> super::BOOLEAN>;
pub type PHIDP_KEYBOARD_MODIFIER_STATE = *mut HIDP_KEYBOARD_MODIFIER_STATE;
#[cfg(feature = "hidusage")]
pub type PHIDP_LINK_COLLECTION_NODE = *mut HIDP_LINK_COLLECTION_NODE;
pub type PHIDP_PREPARSED_DATA = *mut HIDP_PREPARSED_DATA;
#[cfg(feature = "minwindef")]
pub type PHIDP_REPORT_DESCRIPTOR = super::PUCHAR;
pub type PHIDP_UNKNOWN_TOKEN = *mut HIDP_UNKNOWN_TOKEN;
#[cfg(all(feature = "hidusage", feature = "winnt"))]
pub type PHIDP_VALUE_CAPS = *mut HIDP_VALUE_CAPS;
#[cfg(feature = "hidusage")]
pub type PUSAGE_AND_PAGE = *mut USAGE_AND_PAGE;
#[repr(C)]
#[cfg(feature = "hidusage")]
#[derive(Clone, Copy, Default)]
pub struct USAGE_AND_PAGE {
    pub Usage: super::USAGE,
    pub UsagePage: super::USAGE,
}
