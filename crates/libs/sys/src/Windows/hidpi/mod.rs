#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_GetButtonArray(reporttype : HIDP_REPORT_TYPE, usagepage : super::hidusage::USAGE, linkcollection : u16, usage : super::hidusage::USAGE, buttondata : *mut HIDP_BUTTON_ARRAY_DATA, buttondatalength : *mut u16, preparseddata : *const _HIDP_PREPARSED_DATA, report : *const i8, reportlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_GetButtonCaps(reporttype : HIDP_REPORT_TYPE, buttoncaps : *mut HIDP_BUTTON_CAPS, buttoncapslength : *mut u16, preparseddata : *const _HIDP_PREPARSED_DATA) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_GetCaps(preparseddata : *const _HIDP_PREPARSED_DATA, capabilities : *mut HIDP_CAPS) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "bcrypt")]
windows_link::link!("hid.dll" "system" fn HidP_GetData(reporttype : HIDP_REPORT_TYPE, datalist : *mut HIDP_DATA, datalength : *mut u32, preparseddata : *const _HIDP_PREPARSED_DATA, report : *mut i8, reportlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "bcrypt")]
windows_link::link!("hid.dll" "system" fn HidP_GetExtendedAttributes(reporttype : HIDP_REPORT_TYPE, dataindex : u16, preparseddata : *const _HIDP_PREPARSED_DATA, attributes : *mut HIDP_EXTENDED_ATTRIBUTES, lengthattributes : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_GetLinkCollectionNodes(linkcollectionnodes : *mut HIDP_LINK_COLLECTION_NODE, linkcollectionnodeslength : *mut u32, preparseddata : *const _HIDP_PREPARSED_DATA) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_GetScaledUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : super::hidusage::USAGE, linkcollection : u16, usage : super::hidusage::USAGE, usagevalue : *mut i32, preparseddata : *const _HIDP_PREPARSED_DATA, report : *const i8, reportlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_GetSpecificButtonCaps(reporttype : HIDP_REPORT_TYPE, usagepage : super::hidusage::USAGE, linkcollection : u16, usage : super::hidusage::USAGE, buttoncaps : *mut HIDP_BUTTON_CAPS, buttoncapslength : *mut u16, preparseddata : *const _HIDP_PREPARSED_DATA) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_GetSpecificValueCaps(reporttype : HIDP_REPORT_TYPE, usagepage : super::hidusage::USAGE, linkcollection : u16, usage : super::hidusage::USAGE, valuecaps : *mut HIDP_VALUE_CAPS, valuecapslength : *mut u16, preparseddata : *const _HIDP_PREPARSED_DATA) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_GetUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : super::hidusage::USAGE, linkcollection : u16, usage : super::hidusage::USAGE, usagevalue : *mut u32, preparseddata : *const _HIDP_PREPARSED_DATA, report : *const i8, reportlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_GetUsageValueArray(reporttype : HIDP_REPORT_TYPE, usagepage : super::hidusage::USAGE, linkcollection : u16, usage : super::hidusage::USAGE, usagevalue : *mut i8, usagevaluebytelength : u16, preparseddata : *const _HIDP_PREPARSED_DATA, report : *const i8, reportlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_GetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : super::hidusage::USAGE, linkcollection : u16, usagelist : *mut u16, usagelength : *mut u32, preparseddata : *const _HIDP_PREPARSED_DATA, report : *mut i8, reportlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_GetUsagesEx(reporttype : HIDP_REPORT_TYPE, linkcollection : u16, buttonlist : *mut USAGE_AND_PAGE, usagelength : *mut u32, preparseddata : *const _HIDP_PREPARSED_DATA, report : *const i8, reportlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_GetValueCaps(reporttype : HIDP_REPORT_TYPE, valuecaps : *mut HIDP_VALUE_CAPS, valuecapslength : *mut u16, preparseddata : *const _HIDP_PREPARSED_DATA) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "bcrypt")]
windows_link::link!("hid.dll" "system" fn HidP_InitializeReportForID(reporttype : HIDP_REPORT_TYPE, reportid : u8, preparseddata : *const _HIDP_PREPARSED_DATA, report : *mut i8, reportlength : u32) -> super::bcrypt::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_MaxDataListLength(reporttype : HIDP_REPORT_TYPE, preparseddata : *const _HIDP_PREPARSED_DATA) -> u32);
#[cfg(feature = "hidusage")]
windows_link::link!("hid.dll" "system" fn HidP_MaxUsageListLength(reporttype : HIDP_REPORT_TYPE, usagepage : super::hidusage::USAGE, preparseddata : *const _HIDP_PREPARSED_DATA) -> u32);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_SetButtonArray(reporttype : HIDP_REPORT_TYPE, usagepage : super::hidusage::USAGE, linkcollection : u16, usage : super::hidusage::USAGE, buttondata : *const HIDP_BUTTON_ARRAY_DATA, buttondatalength : u16, preparseddata : *const _HIDP_PREPARSED_DATA, report : *mut i8, reportlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "bcrypt")]
windows_link::link!("hid.dll" "system" fn HidP_SetData(reporttype : HIDP_REPORT_TYPE, datalist : *mut HIDP_DATA, datalength : *mut u32, preparseddata : *const _HIDP_PREPARSED_DATA, report : *const i8, reportlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_SetScaledUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : super::hidusage::USAGE, linkcollection : u16, usage : super::hidusage::USAGE, usagevalue : i32, preparseddata : *const _HIDP_PREPARSED_DATA, report : *mut i8, reportlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_SetUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : super::hidusage::USAGE, linkcollection : u16, usage : super::hidusage::USAGE, usagevalue : u32, preparseddata : *const _HIDP_PREPARSED_DATA, report : *mut i8, reportlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_SetUsageValueArray(reporttype : HIDP_REPORT_TYPE, usagepage : super::hidusage::USAGE, linkcollection : u16, usage : super::hidusage::USAGE, usagevalue : *const i8, usagevaluebytelength : u16, preparseddata : *const _HIDP_PREPARSED_DATA, report : *mut i8, reportlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_SetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : super::hidusage::USAGE, linkcollection : u16, usagelist : *mut u16, usagelength : *mut u32, preparseddata : *const _HIDP_PREPARSED_DATA, report : *const i8, reportlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "bcrypt")]
windows_link::link!("hid.dll" "system" fn HidP_TranslateUsagesToI8042ScanCodes(changedusagelist : *const u16, usagelistlength : u32, keyaction : HIDP_KEYBOARD_DIRECTION, modifierstate : *mut HIDP_KEYBOARD_MODIFIER_STATE, insertcodesprocedure : PHIDP_INSERT_SCANCODES, insertcodescontext : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "hidusage"))]
windows_link::link!("hid.dll" "system" fn HidP_UnsetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : super::hidusage::USAGE, linkcollection : u16, usagelist : *mut u16, usagelength : *mut u32, preparseddata : *const _HIDP_PREPARSED_DATA, report : *const i8, reportlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "bcrypt")]
windows_link::link!("hid.dll" "system" fn HidP_UsageListDifference(previoususagelist : *const u16, currentusagelist : *const u16, breakusagelist : *mut u16, makeusagelist : *mut u16, usagelistlength : u32) -> super::bcrypt::NTSTATUS);
pub const FACILITY_HID_ERROR_CODE: u32 = 17;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HIDP_BUTTON_ARRAY_DATA {
    pub ArrayIndex: u16,
    pub On: bool,
}
#[repr(C)]
#[cfg(feature = "hidusage")]
#[derive(Clone, Copy)]
pub struct HIDP_BUTTON_CAPS {
    pub UsagePage: super::hidusage::USAGE,
    pub ReportID: u8,
    pub IsAlias: bool,
    pub BitField: u16,
    pub LinkCollection: u16,
    pub LinkUsage: super::hidusage::USAGE,
    pub LinkUsagePage: super::hidusage::USAGE,
    pub IsRange: bool,
    pub IsStringRange: bool,
    pub IsDesignatorRange: bool,
    pub IsAbsolute: bool,
    pub ReportCount: u16,
    pub Reserved2: u16,
    pub Reserved: [u32; 9],
    pub Anonymous: HIDP_BUTTON_CAPS_0,
}
#[cfg(feature = "hidusage")]
impl Default for HIDP_BUTTON_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "hidusage")]
#[derive(Clone, Copy)]
pub union HIDP_BUTTON_CAPS_0 {
    pub Range: HIDP_BUTTON_CAPS_0_0,
    pub NotRange: HIDP_BUTTON_CAPS_0_1,
}
#[cfg(feature = "hidusage")]
impl Default for HIDP_BUTTON_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "hidusage")]
#[derive(Clone, Copy, Default)]
pub struct HIDP_BUTTON_CAPS_0_0 {
    pub UsageMin: super::hidusage::USAGE,
    pub UsageMax: super::hidusage::USAGE,
    pub StringMin: u16,
    pub StringMax: u16,
    pub DesignatorMin: u16,
    pub DesignatorMax: u16,
    pub DataIndexMin: u16,
    pub DataIndexMax: u16,
}
#[repr(C)]
#[cfg(feature = "hidusage")]
#[derive(Clone, Copy, Default)]
pub struct HIDP_BUTTON_CAPS_0_1 {
    pub Usage: super::hidusage::USAGE,
    pub Reserved1: super::hidusage::USAGE,
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
    pub Usage: super::hidusage::USAGE,
    pub UsagePage: super::hidusage::USAGE,
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
#[derive(Clone, Copy)]
pub struct HIDP_DATA {
    pub DataIndex: u16,
    pub Reserved: u16,
    pub Anonymous: HIDP_DATA_0,
}
impl Default for HIDP_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HIDP_DATA_0 {
    pub RawValue: u32,
    pub On: bool,
}
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
#[derive(Clone, Copy)]
pub struct HIDP_LINK_COLLECTION_NODE {
    pub LinkUsage: super::hidusage::USAGE,
    pub LinkUsagePage: super::hidusage::USAGE,
    pub Parent: u16,
    pub NumberOfChildren: u16,
    pub NextSibling: u16,
    pub FirstChild: u16,
    pub _bitfield: u32,
    pub UserContext: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "hidusage")]
impl Default for HIDP_LINK_COLLECTION_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "hidusage")]
#[derive(Clone, Copy)]
pub struct HIDP_LINK_COLLECTION_NODE {
    pub LinkUsage: super::hidusage::USAGE,
    pub LinkUsagePage: super::hidusage::USAGE,
    pub Parent: u16,
    pub NumberOfChildren: u16,
    pub NextSibling: u16,
    pub FirstChild: u16,
    pub _bitfield: u32,
    pub UserContext: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "hidusage")]
impl Default for HIDP_LINK_COLLECTION_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HIDP_LINK_COLLECTION_ROOT: u16 = 65535;
pub const HIDP_LINK_COLLECTION_UNSPECIFIED: u16 = 0;
pub type HIDP_REPORT_TYPE = i32;
pub const HIDP_STATUS_BAD_LOG_PHY_VALUES: i32 = -1072627706;
pub const HIDP_STATUS_BUFFER_TOO_SMALL: i32 = -1072627705;
pub const HIDP_STATUS_BUTTON_NOT_PRESSED: i32 = -1072627697;
pub const HIDP_STATUS_DATA_INDEX_NOT_FOUND: i32 = -1072627699;
pub const HIDP_STATUS_DATA_INDEX_OUT_OF_RANGE: i32 = -1072627698;
pub const HIDP_STATUS_I8042_TRANS_UNKNOWN: i32 = -1072627703;
pub const HIDP_STATUS_I8242_TRANS_UNKNOWN: i32 = -1072627703;
pub const HIDP_STATUS_INCOMPATIBLE_REPORT_ID: i32 = -1072627702;
pub const HIDP_STATUS_INTERNAL_ERROR: i32 = -1072627704;
pub const HIDP_STATUS_INVALID_PREPARSED_DATA: i32 = -1072627711;
pub const HIDP_STATUS_INVALID_REPORT_LENGTH: i32 = -1072627709;
pub const HIDP_STATUS_INVALID_REPORT_TYPE: i32 = -1072627710;
pub const HIDP_STATUS_IS_VALUE_ARRAY: i32 = -1072627700;
pub const HIDP_STATUS_NOT_BUTTON_ARRAY: i32 = -1072627679;
pub const HIDP_STATUS_NOT_IMPLEMENTED: i32 = -1072627680;
pub const HIDP_STATUS_NOT_VALUE_ARRAY: i32 = -1072627701;
pub const HIDP_STATUS_NULL: i32 = -2146369535;
pub const HIDP_STATUS_REPORT_DOES_NOT_EXIST: i32 = -1072627696;
pub const HIDP_STATUS_SUCCESS: u32 = 1114112;
pub const HIDP_STATUS_USAGE_NOT_FOUND: i32 = -1072627708;
pub const HIDP_STATUS_VALUE_OUT_OF_RANGE: i32 = -1072627707;
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
#[cfg(feature = "hidusage")]
#[derive(Clone, Copy)]
pub struct HIDP_VALUE_CAPS {
    pub UsagePage: super::hidusage::USAGE,
    pub ReportID: u8,
    pub IsAlias: bool,
    pub BitField: u16,
    pub LinkCollection: u16,
    pub LinkUsage: super::hidusage::USAGE,
    pub LinkUsagePage: super::hidusage::USAGE,
    pub IsRange: bool,
    pub IsStringRange: bool,
    pub IsDesignatorRange: bool,
    pub IsAbsolute: bool,
    pub HasNull: bool,
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
#[cfg(feature = "hidusage")]
impl Default for HIDP_VALUE_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "hidusage")]
#[derive(Clone, Copy)]
pub union HIDP_VALUE_CAPS_0 {
    pub Range: HIDP_VALUE_CAPS_0_0,
    pub NotRange: HIDP_VALUE_CAPS_0_1,
}
#[cfg(feature = "hidusage")]
impl Default for HIDP_VALUE_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "hidusage")]
#[derive(Clone, Copy, Default)]
pub struct HIDP_VALUE_CAPS_0_0 {
    pub UsageMin: super::hidusage::USAGE,
    pub UsageMax: super::hidusage::USAGE,
    pub StringMin: u16,
    pub StringMax: u16,
    pub DesignatorMin: u16,
    pub DesignatorMax: u16,
    pub DataIndexMin: u16,
    pub DataIndexMax: u16,
}
#[repr(C)]
#[cfg(feature = "hidusage")]
#[derive(Clone, Copy, Default)]
pub struct HIDP_VALUE_CAPS_0_1 {
    pub Usage: super::hidusage::USAGE,
    pub Reserved1: super::hidusage::USAGE,
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
#[cfg(feature = "bcrypt")]
pub type PFN_HidP_GetVersionInternal = Option<unsafe extern "system" fn(version: *mut u32) -> super::bcrypt::NTSTATUS>;
pub type PHIDP_BUTTON_ARRAY_DATA = *mut HIDP_BUTTON_ARRAY_DATA;
#[cfg(feature = "hidusage")]
pub type PHIDP_BUTTON_CAPS = *mut HIDP_BUTTON_CAPS;
#[cfg(feature = "hidusage")]
pub type PHIDP_CAPS = *mut HIDP_CAPS;
pub type PHIDP_DATA = *mut HIDP_DATA;
pub type PHIDP_EXTENDED_ATTRIBUTES = *mut HIDP_EXTENDED_ATTRIBUTES;
pub type PHIDP_INSERT_SCANCODES = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, newscancodes: *const i8, length: u32) -> bool>;
pub type PHIDP_KEYBOARD_MODIFIER_STATE = *mut HIDP_KEYBOARD_MODIFIER_STATE;
#[cfg(feature = "hidusage")]
pub type PHIDP_LINK_COLLECTION_NODE = *mut HIDP_LINK_COLLECTION_NODE;
pub type PHIDP_PREPARSED_DATA = *mut _HIDP_PREPARSED_DATA;
#[cfg(feature = "minwindef")]
pub type PHIDP_REPORT_DESCRIPTOR = super::minwindef::PUCHAR;
pub type PHIDP_UNKNOWN_TOKEN = *mut HIDP_UNKNOWN_TOKEN;
#[cfg(feature = "hidusage")]
pub type PHIDP_VALUE_CAPS = *mut HIDP_VALUE_CAPS;
#[cfg(feature = "hidusage")]
pub type PUSAGE_AND_PAGE = *mut USAGE_AND_PAGE;
#[repr(C)]
#[cfg(feature = "hidusage")]
#[derive(Clone, Copy, Default)]
pub struct USAGE_AND_PAGE {
    pub Usage: super::hidusage::USAGE,
    pub UsagePage: super::hidusage::USAGE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _HIDP_PREPARSED_DATA(pub u8);
