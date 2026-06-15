windows_link::link!("dinput8.dll" "system" fn DirectInput8Create(hinst : super::super::Foundation::HINSTANCE, dwversion : u32, riidltf : *const windows_sys::core::GUID, ppvout : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("hid.dll" "system" fn HidD_FlushQueue(hiddeviceobject : super::super::Foundation::HANDLE) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_FreePreparsedData(preparseddata : PHIDP_PREPARSED_DATA) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_GetAttributes(hiddeviceobject : super::super::Foundation::HANDLE, attributes : *mut HIDD_ATTRIBUTES) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_GetConfiguration(hiddeviceobject : super::super::Foundation::HANDLE, configuration : *mut HIDD_CONFIGURATION, configurationlength : u32) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_GetFeature(hiddeviceobject : super::super::Foundation::HANDLE, reportbuffer : *mut core::ffi::c_void, reportbufferlength : u32) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_GetHidGuid(hidguid : *mut windows_sys::core::GUID));
windows_link::link!("hid.dll" "system" fn HidD_GetIndexedString(hiddeviceobject : super::super::Foundation::HANDLE, stringindex : u32, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_GetInputReport(hiddeviceobject : super::super::Foundation::HANDLE, reportbuffer : *mut core::ffi::c_void, reportbufferlength : u32) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_GetManufacturerString(hiddeviceobject : super::super::Foundation::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_GetMsGenreDescriptor(hiddeviceobject : super::super::Foundation::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_GetNumInputBuffers(hiddeviceobject : super::super::Foundation::HANDLE, numberbuffers : *mut u32) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_GetPhysicalDescriptor(hiddeviceobject : super::super::Foundation::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_GetPreparsedData(hiddeviceobject : super::super::Foundation::HANDLE, preparseddata : *mut PHIDP_PREPARSED_DATA) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_GetProductString(hiddeviceobject : super::super::Foundation::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_GetSerialNumberString(hiddeviceobject : super::super::Foundation::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_SetConfiguration(hiddeviceobject : super::super::Foundation::HANDLE, configuration : *const HIDD_CONFIGURATION, configurationlength : u32) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_SetFeature(hiddeviceobject : super::super::Foundation::HANDLE, reportbuffer : *const core::ffi::c_void, reportbufferlength : u32) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_SetNumInputBuffers(hiddeviceobject : super::super::Foundation::HANDLE, numberbuffers : u32) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_SetOutputReport(hiddeviceobject : super::super::Foundation::HANDLE, reportbuffer : *const core::ffi::c_void, reportbufferlength : u32) -> bool);
windows_link::link!("hid.dll" "system" fn HidP_GetButtonArray(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, buttondata : *mut HIDP_BUTTON_ARRAY_DATA, buttondatalength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA, report : windows_sys::core::PCSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_GetButtonCaps(reporttype : HIDP_REPORT_TYPE, buttoncaps : *mut HIDP_BUTTON_CAPS, buttoncapslength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_GetCaps(preparseddata : PHIDP_PREPARSED_DATA, capabilities : *mut HIDP_CAPS) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_GetData(reporttype : HIDP_REPORT_TYPE, datalist : *mut HIDP_DATA, datalength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_sys::core::PSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_GetExtendedAttributes(reporttype : HIDP_REPORT_TYPE, dataindex : u16, preparseddata : PHIDP_PREPARSED_DATA, attributes : *mut HIDP_EXTENDED_ATTRIBUTES, lengthattributes : *mut u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_GetLinkCollectionNodes(linkcollectionnodes : *mut HIDP_LINK_COLLECTION_NODE, linkcollectionnodeslength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_GetScaledUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : *mut i32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_sys::core::PCSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_GetSpecificButtonCaps(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, buttoncaps : *mut HIDP_BUTTON_CAPS, buttoncapslength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_GetSpecificValueCaps(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, valuecaps : *mut HIDP_VALUE_CAPS, valuecapslength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_GetUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_sys::core::PCSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_GetUsageValueArray(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : windows_sys::core::PSTR, usagevaluebytelength : u16, preparseddata : PHIDP_PREPARSED_DATA, report : windows_sys::core::PCSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_GetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usagelist : *mut u16, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_sys::core::PSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_GetUsagesEx(reporttype : HIDP_REPORT_TYPE, linkcollection : u16, buttonlist : *mut USAGE_AND_PAGE, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_sys::core::PCSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_GetValueCaps(reporttype : HIDP_REPORT_TYPE, valuecaps : *mut HIDP_VALUE_CAPS, valuecapslength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_InitializeReportForID(reporttype : HIDP_REPORT_TYPE, reportid : u8, preparseddata : PHIDP_PREPARSED_DATA, report : windows_sys::core::PSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_MaxDataListLength(reporttype : HIDP_REPORT_TYPE, preparseddata : PHIDP_PREPARSED_DATA) -> u32);
windows_link::link!("hid.dll" "system" fn HidP_MaxUsageListLength(reporttype : HIDP_REPORT_TYPE, usagepage : u16, preparseddata : PHIDP_PREPARSED_DATA) -> u32);
windows_link::link!("hid.dll" "system" fn HidP_SetButtonArray(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, buttondata : *const HIDP_BUTTON_ARRAY_DATA, buttondatalength : u16, preparseddata : PHIDP_PREPARSED_DATA, report : windows_sys::core::PSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_SetData(reporttype : HIDP_REPORT_TYPE, datalist : *mut HIDP_DATA, datalength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_sys::core::PCSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_SetScaledUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : i32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_sys::core::PSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_SetUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_sys::core::PSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_SetUsageValueArray(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : windows_sys::core::PCSTR, usagevaluebytelength : u16, preparseddata : PHIDP_PREPARSED_DATA, report : windows_sys::core::PSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_SetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usagelist : *mut u16, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_sys::core::PCSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_TranslateUsagesToI8042ScanCodes(changedusagelist : *const u16, usagelistlength : u32, keyaction : HIDP_KEYBOARD_DIRECTION, modifierstate : *mut HIDP_KEYBOARD_MODIFIER_STATE, insertcodesprocedure : PHIDP_INSERT_SCANCODES, insertcodescontext : *const core::ffi::c_void) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_UnsetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usagelist : *mut u16, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_sys::core::PCSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("hid.dll" "system" fn HidP_UsageListDifference(previoususagelist : *const u16, currentusagelist : *const u16, breakusagelist : *mut u16, makeusagelist : *mut u16, usagelistlength : u32) -> super::super::Foundation::NTSTATUS);
windows_link::link!("winmm.dll" "system" fn joyConfigChanged(dwflags : u32) -> u32);
pub const BALLPOINT_I8042_HARDWARE: u32 = 8;
pub const BALLPOINT_SERIAL_HARDWARE: u32 = 16;
pub const BUTTON_BIT_ALLBUTTONSMASK: u32 = 16383;
pub const BUTTON_BIT_BACK: u32 = 32;
pub const BUTTON_BIT_CAMERAFOCUS: u32 = 128;
pub const BUTTON_BIT_CAMERALENS: u32 = 4096;
pub const BUTTON_BIT_CAMERASHUTTER: u32 = 256;
pub const BUTTON_BIT_HEADSET: u32 = 1024;
pub const BUTTON_BIT_HWKBDEPLOY: u32 = 2048;
pub const BUTTON_BIT_OEMCUSTOM: u32 = 8192;
pub const BUTTON_BIT_OEMCUSTOM2: u32 = 16384;
pub const BUTTON_BIT_OEMCUSTOM3: u32 = 32768;
pub const BUTTON_BIT_POWER: u32 = 1;
pub const BUTTON_BIT_RINGERTOGGLE: u32 = 512;
pub const BUTTON_BIT_ROTATION_LOCK: u32 = 16;
pub const BUTTON_BIT_SEARCH: u32 = 64;
pub const BUTTON_BIT_VOLUMEDOWN: u32 = 8;
pub const BUTTON_BIT_VOLUMEUP: u32 = 4;
pub const BUTTON_BIT_WINDOWS: u32 = 2;
pub const CLSID_DirectInput: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x25e609e0_b259_11cf_bfc7_444553540000);
pub const CLSID_DirectInput8: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x25e609e4_b259_11cf_bfc7_444553540000);
pub const CLSID_DirectInputDevice: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x25e609e1_b259_11cf_bfc7_444553540000);
pub const CLSID_DirectInputDevice8: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x25e609e5_b259_11cf_bfc7_444553540000);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CPOINT {
    pub lP: i32,
    pub dwLog: u32,
}
pub const DD_KEYBOARD_DEVICE_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("\\Device\\KeyboardClass");
pub const DD_KEYBOARD_DEVICE_NAME_U: windows_sys::core::PCWSTR = windows_sys::core::w!("\\Device\\KeyboardClass");
pub const DD_MOUSE_DEVICE_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("\\Device\\PointerClass");
pub const DD_MOUSE_DEVICE_NAME_U: windows_sys::core::PCWSTR = windows_sys::core::w!("\\Device\\PointerClass");
pub const DEVPKEY_DeviceInterface_HID_BackgroundAccess: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 8 };
pub const DEVPKEY_DeviceInterface_HID_IsReadOnly: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 4 };
pub const DEVPKEY_DeviceInterface_HID_ProductId: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 6 };
pub const DEVPKEY_DeviceInterface_HID_UsageId: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 3 };
pub const DEVPKEY_DeviceInterface_HID_UsagePage: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 2 };
pub const DEVPKEY_DeviceInterface_HID_VendorId: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 5 };
pub const DEVPKEY_DeviceInterface_HID_VersionNumber: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 7 };
pub const DEVPKEY_DeviceInterface_HID_WakeScreenOnInputCapable: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 9 };
pub const DI8DEVCLASS_ALL: u32 = 0;
pub const DI8DEVCLASS_DEVICE: u32 = 1;
pub const DI8DEVCLASS_GAMECTRL: u32 = 4;
pub const DI8DEVCLASS_KEYBOARD: u32 = 3;
pub const DI8DEVCLASS_POINTER: u32 = 2;
pub const DI8DEVTYPE1STPERSON_LIMITED: u32 = 1;
pub const DI8DEVTYPE1STPERSON_SHOOTER: u32 = 4;
pub const DI8DEVTYPE1STPERSON_SIXDOF: u32 = 3;
pub const DI8DEVTYPE1STPERSON_UNKNOWN: u32 = 2;
pub const DI8DEVTYPEDEVICECTRL_COMMSSELECTION: u32 = 3;
pub const DI8DEVTYPEDEVICECTRL_COMMSSELECTION_HARDWIRED: u32 = 4;
pub const DI8DEVTYPEDEVICECTRL_UNKNOWN: u32 = 2;
pub const DI8DEVTYPEDRIVING_COMBINEDPEDALS: u32 = 2;
pub const DI8DEVTYPEDRIVING_DUALPEDALS: u32 = 3;
pub const DI8DEVTYPEDRIVING_HANDHELD: u32 = 5;
pub const DI8DEVTYPEDRIVING_LIMITED: u32 = 1;
pub const DI8DEVTYPEDRIVING_THREEPEDALS: u32 = 4;
pub const DI8DEVTYPEFLIGHT_LIMITED: u32 = 1;
pub const DI8DEVTYPEFLIGHT_RC: u32 = 4;
pub const DI8DEVTYPEFLIGHT_STICK: u32 = 2;
pub const DI8DEVTYPEFLIGHT_YOKE: u32 = 3;
pub const DI8DEVTYPEGAMEPAD_LIMITED: u32 = 1;
pub const DI8DEVTYPEGAMEPAD_STANDARD: u32 = 2;
pub const DI8DEVTYPEGAMEPAD_TILT: u32 = 3;
pub const DI8DEVTYPEJOYSTICK_LIMITED: u32 = 1;
pub const DI8DEVTYPEJOYSTICK_STANDARD: u32 = 2;
pub const DI8DEVTYPEKEYBOARD_J3100: u32 = 12;
pub const DI8DEVTYPEKEYBOARD_JAPAN106: u32 = 10;
pub const DI8DEVTYPEKEYBOARD_JAPANAX: u32 = 11;
pub const DI8DEVTYPEKEYBOARD_NEC98: u32 = 7;
pub const DI8DEVTYPEKEYBOARD_NEC98106: u32 = 9;
pub const DI8DEVTYPEKEYBOARD_NEC98LAPTOP: u32 = 8;
pub const DI8DEVTYPEKEYBOARD_NOKIA1050: u32 = 5;
pub const DI8DEVTYPEKEYBOARD_NOKIA9140: u32 = 6;
pub const DI8DEVTYPEKEYBOARD_OLIVETTI: u32 = 2;
pub const DI8DEVTYPEKEYBOARD_PCAT: u32 = 3;
pub const DI8DEVTYPEKEYBOARD_PCENH: u32 = 4;
pub const DI8DEVTYPEKEYBOARD_PCXT: u32 = 1;
pub const DI8DEVTYPEKEYBOARD_UNKNOWN: u32 = 0;
pub const DI8DEVTYPEMOUSE_ABSOLUTE: u32 = 6;
pub const DI8DEVTYPEMOUSE_FINGERSTICK: u32 = 3;
pub const DI8DEVTYPEMOUSE_TOUCHPAD: u32 = 4;
pub const DI8DEVTYPEMOUSE_TRACKBALL: u32 = 5;
pub const DI8DEVTYPEMOUSE_TRADITIONAL: u32 = 2;
pub const DI8DEVTYPEMOUSE_UNKNOWN: u32 = 1;
pub const DI8DEVTYPEREMOTE_UNKNOWN: u32 = 2;
pub const DI8DEVTYPESCREENPTR_LIGHTGUN: u32 = 3;
pub const DI8DEVTYPESCREENPTR_LIGHTPEN: u32 = 4;
pub const DI8DEVTYPESCREENPTR_TOUCH: u32 = 5;
pub const DI8DEVTYPESCREENPTR_UNKNOWN: u32 = 2;
pub const DI8DEVTYPESUPPLEMENTAL_2NDHANDCONTROLLER: u32 = 3;
pub const DI8DEVTYPESUPPLEMENTAL_COMBINEDPEDALS: u32 = 10;
pub const DI8DEVTYPESUPPLEMENTAL_DUALPEDALS: u32 = 11;
pub const DI8DEVTYPESUPPLEMENTAL_HANDTRACKER: u32 = 5;
pub const DI8DEVTYPESUPPLEMENTAL_HEADTRACKER: u32 = 4;
pub const DI8DEVTYPESUPPLEMENTAL_RUDDERPEDALS: u32 = 13;
pub const DI8DEVTYPESUPPLEMENTAL_SHIFTER: u32 = 7;
pub const DI8DEVTYPESUPPLEMENTAL_SHIFTSTICKGATE: u32 = 6;
pub const DI8DEVTYPESUPPLEMENTAL_SPLITTHROTTLE: u32 = 9;
pub const DI8DEVTYPESUPPLEMENTAL_THREEPEDALS: u32 = 12;
pub const DI8DEVTYPESUPPLEMENTAL_THROTTLE: u32 = 8;
pub const DI8DEVTYPESUPPLEMENTAL_UNKNOWN: u32 = 2;
pub const DI8DEVTYPE_1STPERSON: u32 = 24;
pub const DI8DEVTYPE_DEVICE: u32 = 17;
pub const DI8DEVTYPE_DEVICECTRL: u32 = 25;
pub const DI8DEVTYPE_DRIVING: u32 = 22;
pub const DI8DEVTYPE_FLIGHT: u32 = 23;
pub const DI8DEVTYPE_GAMEPAD: u32 = 21;
pub const DI8DEVTYPE_JOYSTICK: u32 = 20;
pub const DI8DEVTYPE_KEYBOARD: u32 = 19;
pub const DI8DEVTYPE_LIMITEDGAMESUBTYPE: u32 = 1;
pub const DI8DEVTYPE_MOUSE: u32 = 18;
pub const DI8DEVTYPE_REMOTE: u32 = 27;
pub const DI8DEVTYPE_SCREENPOINTER: u32 = 26;
pub const DI8DEVTYPE_SUPPLEMENTAL: u32 = 28;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DIACTIONA {
    pub uAppData: usize,
    pub dwSemantic: u32,
    pub dwFlags: u32,
    pub Anonymous: DIACTIONA_0,
    pub guidInstance: windows_sys::core::GUID,
    pub dwObjID: u32,
    pub dwHow: u32,
}
impl Default for DIACTIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DIACTIONA_0 {
    pub lptszActionName: windows_sys::core::PCSTR,
    pub uResIdString: u32,
}
impl Default for DIACTIONA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIACTIONFORMATA {
    pub dwSize: u32,
    pub dwActionSize: u32,
    pub dwDataSize: u32,
    pub dwNumActions: u32,
    pub rgoAction: *mut DIACTIONA,
    pub guidActionMap: windows_sys::core::GUID,
    pub dwGenre: u32,
    pub dwBufferSize: u32,
    pub lAxisMin: i32,
    pub lAxisMax: i32,
    pub hInstString: super::super::Foundation::HINSTANCE,
    pub ftTimeStamp: super::super::Foundation::FILETIME,
    pub dwCRC: u32,
    pub tszActionMap: [i8; 260],
}
impl Default for DIACTIONFORMATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIACTIONFORMATW {
    pub dwSize: u32,
    pub dwActionSize: u32,
    pub dwDataSize: u32,
    pub dwNumActions: u32,
    pub rgoAction: *mut DIACTIONW,
    pub guidActionMap: windows_sys::core::GUID,
    pub dwGenre: u32,
    pub dwBufferSize: u32,
    pub lAxisMin: i32,
    pub lAxisMax: i32,
    pub hInstString: super::super::Foundation::HINSTANCE,
    pub ftTimeStamp: super::super::Foundation::FILETIME,
    pub dwCRC: u32,
    pub tszActionMap: [u16; 260],
}
impl Default for DIACTIONFORMATW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DIACTIONW {
    pub uAppData: usize,
    pub dwSemantic: u32,
    pub dwFlags: u32,
    pub Anonymous: DIACTIONW_0,
    pub guidInstance: windows_sys::core::GUID,
    pub dwObjID: u32,
    pub dwHow: u32,
}
impl Default for DIACTIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DIACTIONW_0 {
    pub lptszActionName: windows_sys::core::PCWSTR,
    pub uResIdString: u32,
}
impl Default for DIACTIONW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIAFTS_NEWDEVICEHIGH: u32 = 4294967295;
pub const DIAFTS_NEWDEVICELOW: u32 = 4294967295;
pub const DIAFTS_UNUSEDDEVICEHIGH: u32 = 0;
pub const DIAFTS_UNUSEDDEVICELOW: u32 = 0;
pub const DIAH_APPREQUESTED: u32 = 2;
pub const DIAH_DEFAULT: u32 = 32;
pub const DIAH_ERROR: u32 = 2147483648;
pub const DIAH_HWAPP: u32 = 4;
pub const DIAH_HWDEFAULT: u32 = 8;
pub const DIAH_UNMAPPED: u32 = 0;
pub const DIAH_USERCONFIG: u32 = 1;
pub const DIAPPIDFLAG_NOSIZE: u32 = 2;
pub const DIAPPIDFLAG_NOTIME: u32 = 1;
pub const DIAXIS_2DCONTROL_INOUT: u32 = 587301379;
pub const DIAXIS_2DCONTROL_LATERAL: u32 = 587235841;
pub const DIAXIS_2DCONTROL_MOVE: u32 = 587268610;
pub const DIAXIS_2DCONTROL_ROTATEZ: u32 = 587350532;
pub const DIAXIS_3DCONTROL_INOUT: u32 = 604078595;
pub const DIAXIS_3DCONTROL_LATERAL: u32 = 604013057;
pub const DIAXIS_3DCONTROL_MOVE: u32 = 604045826;
pub const DIAXIS_3DCONTROL_ROTATEX: u32 = 604193284;
pub const DIAXIS_3DCONTROL_ROTATEY: u32 = 604160517;
pub const DIAXIS_3DCONTROL_ROTATEZ: u32 = 604127750;
pub const DIAXIS_ANY_1: u32 = 4278206977;
pub const DIAXIS_ANY_2: u32 = 4278206978;
pub const DIAXIS_ANY_3: u32 = 4278206979;
pub const DIAXIS_ANY_4: u32 = 4278206980;
pub const DIAXIS_ANY_A_1: u32 = 4278436353;
pub const DIAXIS_ANY_A_2: u32 = 4278436354;
pub const DIAXIS_ANY_B_1: u32 = 4278469121;
pub const DIAXIS_ANY_B_2: u32 = 4278469122;
pub const DIAXIS_ANY_C_1: u32 = 4278501889;
pub const DIAXIS_ANY_C_2: u32 = 4278501890;
pub const DIAXIS_ANY_R_1: u32 = 4278338049;
pub const DIAXIS_ANY_R_2: u32 = 4278338050;
pub const DIAXIS_ANY_S_1: u32 = 4278534657;
pub const DIAXIS_ANY_S_2: u32 = 4278534658;
pub const DIAXIS_ANY_U_1: u32 = 4278370817;
pub const DIAXIS_ANY_U_2: u32 = 4278370818;
pub const DIAXIS_ANY_V_1: u32 = 4278403585;
pub const DIAXIS_ANY_V_2: u32 = 4278403586;
pub const DIAXIS_ANY_X_1: u32 = 4278239745;
pub const DIAXIS_ANY_X_2: u32 = 4278239746;
pub const DIAXIS_ANY_Y_1: u32 = 4278272513;
pub const DIAXIS_ANY_Y_2: u32 = 4278272514;
pub const DIAXIS_ANY_Z_1: u32 = 4278305281;
pub const DIAXIS_ANY_Z_2: u32 = 4278305282;
pub const DIAXIS_ARCADEP_LATERAL: u32 = 570458625;
pub const DIAXIS_ARCADEP_MOVE: u32 = 570491394;
pub const DIAXIS_ARCADES_LATERAL: u32 = 553681409;
pub const DIAXIS_ARCADES_MOVE: u32 = 553714178;
pub const DIAXIS_BASEBALLB_LATERAL: u32 = 251691521;
pub const DIAXIS_BASEBALLB_MOVE: u32 = 251724290;
pub const DIAXIS_BASEBALLF_LATERAL: u32 = 285245953;
pub const DIAXIS_BASEBALLF_MOVE: u32 = 285278722;
pub const DIAXIS_BASEBALLP_LATERAL: u32 = 268468737;
pub const DIAXIS_BASEBALLP_MOVE: u32 = 268501506;
pub const DIAXIS_BBALLD_LATERAL: u32 = 318800385;
pub const DIAXIS_BBALLD_MOVE: u32 = 318833154;
pub const DIAXIS_BBALLO_LATERAL: u32 = 302023169;
pub const DIAXIS_BBALLO_MOVE: u32 = 302055938;
pub const DIAXIS_BIKINGM_BRAKE: u32 = 470041091;
pub const DIAXIS_BIKINGM_PEDAL: u32 = 469828098;
pub const DIAXIS_BIKINGM_TURN: u32 = 469795329;
pub const DIAXIS_BROWSER_LATERAL: u32 = 671121921;
pub const DIAXIS_BROWSER_MOVE: u32 = 671154690;
pub const DIAXIS_BROWSER_VIEW: u32 = 671187459;
pub const DIAXIS_CADF_INOUT: u32 = 620855811;
pub const DIAXIS_CADF_LATERAL: u32 = 620790273;
pub const DIAXIS_CADF_MOVE: u32 = 620823042;
pub const DIAXIS_CADF_ROTATEX: u32 = 620970500;
pub const DIAXIS_CADF_ROTATEY: u32 = 620937733;
pub const DIAXIS_CADF_ROTATEZ: u32 = 620904966;
pub const DIAXIS_CADM_INOUT: u32 = 637633027;
pub const DIAXIS_CADM_LATERAL: u32 = 637567489;
pub const DIAXIS_CADM_MOVE: u32 = 637600258;
pub const DIAXIS_CADM_ROTATEX: u32 = 637747716;
pub const DIAXIS_CADM_ROTATEY: u32 = 637714949;
pub const DIAXIS_CADM_ROTATEZ: u32 = 637682182;
pub const DIAXIS_DRIVINGC_ACCELERATE: u32 = 33788418;
pub const DIAXIS_DRIVINGC_ACCEL_AND_BRAKE: u32 = 33638916;
pub const DIAXIS_DRIVINGC_BRAKE: u32 = 33821187;
pub const DIAXIS_DRIVINGC_STEER: u32 = 33589761;
pub const DIAXIS_DRIVINGR_ACCELERATE: u32 = 17011202;
pub const DIAXIS_DRIVINGR_ACCEL_AND_BRAKE: u32 = 16861700;
pub const DIAXIS_DRIVINGR_BRAKE: u32 = 17043971;
pub const DIAXIS_DRIVINGR_STEER: u32 = 16812545;
pub const DIAXIS_DRIVINGT_ACCELERATE: u32 = 50565635;
pub const DIAXIS_DRIVINGT_ACCEL_AND_BRAKE: u32 = 50416134;
pub const DIAXIS_DRIVINGT_BARREL: u32 = 50397698;
pub const DIAXIS_DRIVINGT_BRAKE: u32 = 50614789;
pub const DIAXIS_DRIVINGT_ROTATE: u32 = 50463236;
pub const DIAXIS_DRIVINGT_STEER: u32 = 50366977;
pub const DIAXIS_FIGHTINGH_LATERAL: u32 = 134251009;
pub const DIAXIS_FIGHTINGH_MOVE: u32 = 134283778;
pub const DIAXIS_FIGHTINGH_ROTATE: u32 = 134365699;
pub const DIAXIS_FISHING_LATERAL: u32 = 234914305;
pub const DIAXIS_FISHING_MOVE: u32 = 234947074;
pub const DIAXIS_FISHING_ROTATE: u32 = 235028995;
pub const DIAXIS_FLYINGC_BANK: u32 = 67144193;
pub const DIAXIS_FLYINGC_BRAKE: u32 = 67398148;
pub const DIAXIS_FLYINGC_FLAPS: u32 = 67459590;
pub const DIAXIS_FLYINGC_PITCH: u32 = 67176962;
pub const DIAXIS_FLYINGC_RUDDER: u32 = 67260933;
pub const DIAXIS_FLYINGC_THROTTLE: u32 = 67342851;
pub const DIAXIS_FLYINGH_BANK: u32 = 100698625;
pub const DIAXIS_FLYINGH_COLLECTIVE: u32 = 100764163;
pub const DIAXIS_FLYINGH_PITCH: u32 = 100731394;
pub const DIAXIS_FLYINGH_THROTTLE: u32 = 100915717;
pub const DIAXIS_FLYINGH_TORQUE: u32 = 100817412;
pub const DIAXIS_FLYINGM_BANK: u32 = 83921409;
pub const DIAXIS_FLYINGM_BRAKE: u32 = 84173317;
pub const DIAXIS_FLYINGM_FLAPS: u32 = 84234758;
pub const DIAXIS_FLYINGM_PITCH: u32 = 83954178;
pub const DIAXIS_FLYINGM_RUDDER: u32 = 84036100;
pub const DIAXIS_FLYINGM_THROTTLE: u32 = 84120067;
pub const DIAXIS_FOOTBALLD_LATERAL: u32 = 385909249;
pub const DIAXIS_FOOTBALLD_MOVE: u32 = 385942018;
pub const DIAXIS_FOOTBALLO_LATERAL: u32 = 369132033;
pub const DIAXIS_FOOTBALLO_MOVE: u32 = 369164802;
pub const DIAXIS_FOOTBALLQ_LATERAL: u32 = 352354817;
pub const DIAXIS_FOOTBALLQ_MOVE: u32 = 352387586;
pub const DIAXIS_FPS_LOOKUPDOWN: u32 = 151093763;
pub const DIAXIS_FPS_MOVE: u32 = 151060994;
pub const DIAXIS_FPS_ROTATE: u32 = 151028225;
pub const DIAXIS_FPS_SIDESTEP: u32 = 151142916;
pub const DIAXIS_GOLF_LATERAL: u32 = 402686465;
pub const DIAXIS_GOLF_MOVE: u32 = 402719234;
pub const DIAXIS_HOCKEYD_LATERAL: u32 = 436240897;
pub const DIAXIS_HOCKEYD_MOVE: u32 = 436273666;
pub const DIAXIS_HOCKEYG_LATERAL: u32 = 453018113;
pub const DIAXIS_HOCKEYG_MOVE: u32 = 453050882;
pub const DIAXIS_HOCKEYO_LATERAL: u32 = 419463681;
pub const DIAXIS_HOCKEYO_MOVE: u32 = 419496450;
pub const DIAXIS_HUNTING_LATERAL: u32 = 218137089;
pub const DIAXIS_HUNTING_MOVE: u32 = 218169858;
pub const DIAXIS_HUNTING_ROTATE: u32 = 218251779;
pub const DIAXIS_MECHA_ROTATE: u32 = 687997443;
pub const DIAXIS_MECHA_STEER: u32 = 687899137;
pub const DIAXIS_MECHA_THROTTLE: u32 = 688095748;
pub const DIAXIS_MECHA_TORSO: u32 = 687931906;
pub const DIAXIS_RACQUET_LATERAL: u32 = 536904193;
pub const DIAXIS_RACQUET_MOVE: u32 = 536936962;
pub const DIAXIS_REMOTE_SLIDER: u32 = 654639617;
pub const DIAXIS_REMOTE_SLIDER2: u32 = 654656002;
pub const DIAXIS_SKIING_SPEED: u32 = 486605314;
pub const DIAXIS_SKIING_TURN: u32 = 486572545;
pub const DIAXIS_SOCCERD_LATERAL: u32 = 520126977;
pub const DIAXIS_SOCCERD_MOVE: u32 = 520159746;
pub const DIAXIS_SOCCERO_BEND: u32 = 503415299;
pub const DIAXIS_SOCCERO_LATERAL: u32 = 503349761;
pub const DIAXIS_SOCCERO_MOVE: u32 = 503382530;
pub const DIAXIS_SPACESIM_CLIMB: u32 = 117555716;
pub const DIAXIS_SPACESIM_LATERAL: u32 = 117473793;
pub const DIAXIS_SPACESIM_MOVE: u32 = 117506562;
pub const DIAXIS_SPACESIM_ROTATE: u32 = 117588485;
pub const DIAXIS_SPACESIM_THROTTLE: u32 = 117670403;
pub const DIAXIS_STRATEGYR_LATERAL: u32 = 184582657;
pub const DIAXIS_STRATEGYR_MOVE: u32 = 184615426;
pub const DIAXIS_STRATEGYR_ROTATE: u32 = 184697347;
pub const DIAXIS_STRATEGYT_LATERAL: u32 = 201359873;
pub const DIAXIS_STRATEGYT_MOVE: u32 = 201392642;
pub const DIAXIS_TPS_MOVE: u32 = 167838210;
pub const DIAXIS_TPS_STEP: u32 = 167821827;
pub const DIAXIS_TPS_TURN: u32 = 167903745;
pub const DIA_APPFIXED: u32 = 16;
pub const DIA_APPMAPPED: u32 = 2;
pub const DIA_APPNOMAP: u32 = 4;
pub const DIA_FORCEFEEDBACK: u32 = 1;
pub const DIA_NORANGE: u32 = 8;
pub const DIBUTTON_2DCONTROL_DEVICE: u32 = 587220222;
pub const DIBUTTON_2DCONTROL_DISPLAY: u32 = 587219973;
pub const DIBUTTON_2DCONTROL_MENU: u32 = 587203837;
pub const DIBUTTON_2DCONTROL_PAUSE: u32 = 587220220;
pub const DIBUTTON_2DCONTROL_SELECT: u32 = 587203585;
pub const DIBUTTON_2DCONTROL_SPECIAL: u32 = 587203587;
pub const DIBUTTON_2DCONTROL_SPECIAL1: u32 = 587203586;
pub const DIBUTTON_2DCONTROL_SPECIAL2: u32 = 587203588;
pub const DIBUTTON_3DCONTROL_DEVICE: u32 = 603997438;
pub const DIBUTTON_3DCONTROL_DISPLAY: u32 = 603997189;
pub const DIBUTTON_3DCONTROL_MENU: u32 = 603981053;
pub const DIBUTTON_3DCONTROL_PAUSE: u32 = 603997436;
pub const DIBUTTON_3DCONTROL_SELECT: u32 = 603980801;
pub const DIBUTTON_3DCONTROL_SPECIAL: u32 = 603980803;
pub const DIBUTTON_3DCONTROL_SPECIAL1: u32 = 603980802;
pub const DIBUTTON_3DCONTROL_SPECIAL2: u32 = 603980804;
pub const DIBUTTON_ARCADEP_BACK_LINK: u32 = 570508520;
pub const DIBUTTON_ARCADEP_CROUCH: u32 = 570426371;
pub const DIBUTTON_ARCADEP_DEVICE: u32 = 570443006;
pub const DIBUTTON_ARCADEP_FIRE: u32 = 570426370;
pub const DIBUTTON_ARCADEP_FIRESECONDARY: u32 = 570442758;
pub const DIBUTTON_ARCADEP_FORWARD_LINK: u32 = 570508512;
pub const DIBUTTON_ARCADEP_JUMP: u32 = 570426369;
pub const DIBUTTON_ARCADEP_LEFT_LINK: u32 = 570475748;
pub const DIBUTTON_ARCADEP_MENU: u32 = 570426621;
pub const DIBUTTON_ARCADEP_PAUSE: u32 = 570443004;
pub const DIBUTTON_ARCADEP_RIGHT_LINK: u32 = 570475756;
pub const DIBUTTON_ARCADEP_SELECT: u32 = 570426373;
pub const DIBUTTON_ARCADEP_SPECIAL: u32 = 570426372;
pub const DIBUTTON_ARCADEP_VIEW_DOWN_LINK: u32 = 570934504;
pub const DIBUTTON_ARCADEP_VIEW_LEFT_LINK: u32 = 570934500;
pub const DIBUTTON_ARCADEP_VIEW_RIGHT_LINK: u32 = 570934508;
pub const DIBUTTON_ARCADEP_VIEW_UP_LINK: u32 = 570934496;
pub const DIBUTTON_ARCADES_ATTACK: u32 = 553649155;
pub const DIBUTTON_ARCADES_BACK_LINK: u32 = 553731304;
pub const DIBUTTON_ARCADES_CARRY: u32 = 553649154;
pub const DIBUTTON_ARCADES_DEVICE: u32 = 553665790;
pub const DIBUTTON_ARCADES_FORWARD_LINK: u32 = 553731296;
pub const DIBUTTON_ARCADES_LEFT_LINK: u32 = 553698532;
pub const DIBUTTON_ARCADES_MENU: u32 = 553649405;
pub const DIBUTTON_ARCADES_PAUSE: u32 = 553665788;
pub const DIBUTTON_ARCADES_RIGHT_LINK: u32 = 553698540;
pub const DIBUTTON_ARCADES_SELECT: u32 = 553649157;
pub const DIBUTTON_ARCADES_SPECIAL: u32 = 553649156;
pub const DIBUTTON_ARCADES_THROW: u32 = 553649153;
pub const DIBUTTON_ARCADES_VIEW_DOWN_LINK: u32 = 554157288;
pub const DIBUTTON_ARCADES_VIEW_LEFT_LINK: u32 = 554157284;
pub const DIBUTTON_ARCADES_VIEW_RIGHT_LINK: u32 = 554157292;
pub const DIBUTTON_ARCADES_VIEW_UP_LINK: u32 = 554157280;
pub const DIBUTTON_BASEBALLB_BACK_LINK: u32 = 251741416;
pub const DIBUTTON_BASEBALLB_BOX: u32 = 251675658;
pub const DIBUTTON_BASEBALLB_BUNT: u32 = 251659268;
pub const DIBUTTON_BASEBALLB_BURST: u32 = 251659270;
pub const DIBUTTON_BASEBALLB_CONTACT: u32 = 251659272;
pub const DIBUTTON_BASEBALLB_DEVICE: u32 = 251675902;
pub const DIBUTTON_BASEBALLB_FORWARD_LINK: u32 = 251741408;
pub const DIBUTTON_BASEBALLB_LEFT_LINK: u32 = 251708644;
pub const DIBUTTON_BASEBALLB_MENU: u32 = 251659517;
pub const DIBUTTON_BASEBALLB_NORMAL: u32 = 251659266;
pub const DIBUTTON_BASEBALLB_NOSTEAL: u32 = 251675657;
pub const DIBUTTON_BASEBALLB_PAUSE: u32 = 251675900;
pub const DIBUTTON_BASEBALLB_POWER: u32 = 251659267;
pub const DIBUTTON_BASEBALLB_RIGHT_LINK: u32 = 251708652;
pub const DIBUTTON_BASEBALLB_SELECT: u32 = 251659265;
pub const DIBUTTON_BASEBALLB_SLIDE: u32 = 251659271;
pub const DIBUTTON_BASEBALLB_STEAL: u32 = 251659269;
pub const DIBUTTON_BASEBALLF_AIM_LEFT_LINK: u32 = 285263076;
pub const DIBUTTON_BASEBALLF_AIM_RIGHT_LINK: u32 = 285263084;
pub const DIBUTTON_BASEBALLF_BACK_LINK: u32 = 285295848;
pub const DIBUTTON_BASEBALLF_BURST: u32 = 285213700;
pub const DIBUTTON_BASEBALLF_DEVICE: u32 = 285230334;
pub const DIBUTTON_BASEBALLF_DIVE: u32 = 285213702;
pub const DIBUTTON_BASEBALLF_FORWARD_LINK: u32 = 285295840;
pub const DIBUTTON_BASEBALLF_JUMP: u32 = 285213701;
pub const DIBUTTON_BASEBALLF_MENU: u32 = 285213949;
pub const DIBUTTON_BASEBALLF_NEAREST: u32 = 285213697;
pub const DIBUTTON_BASEBALLF_PAUSE: u32 = 285230332;
pub const DIBUTTON_BASEBALLF_SHIFTIN: u32 = 285230087;
pub const DIBUTTON_BASEBALLF_SHIFTOUT: u32 = 285230088;
pub const DIBUTTON_BASEBALLF_THROW1: u32 = 285213698;
pub const DIBUTTON_BASEBALLF_THROW2: u32 = 285213699;
pub const DIBUTTON_BASEBALLP_BACK_LINK: u32 = 268518632;
pub const DIBUTTON_BASEBALLP_BASE: u32 = 268436483;
pub const DIBUTTON_BASEBALLP_DEVICE: u32 = 268453118;
pub const DIBUTTON_BASEBALLP_FAKE: u32 = 268436485;
pub const DIBUTTON_BASEBALLP_FORWARD_LINK: u32 = 268518624;
pub const DIBUTTON_BASEBALLP_LEFT_LINK: u32 = 268485860;
pub const DIBUTTON_BASEBALLP_LOOK: u32 = 268452871;
pub const DIBUTTON_BASEBALLP_MENU: u32 = 268436733;
pub const DIBUTTON_BASEBALLP_PAUSE: u32 = 268453116;
pub const DIBUTTON_BASEBALLP_PITCH: u32 = 268436482;
pub const DIBUTTON_BASEBALLP_RIGHT_LINK: u32 = 268485868;
pub const DIBUTTON_BASEBALLP_SELECT: u32 = 268436481;
pub const DIBUTTON_BASEBALLP_THROW: u32 = 268436484;
pub const DIBUTTON_BASEBALLP_WALK: u32 = 268452870;
pub const DIBUTTON_BBALLD_BACK_LINK: u32 = 318850280;
pub const DIBUTTON_BBALLD_BURST: u32 = 318768134;
pub const DIBUTTON_BBALLD_DEVICE: u32 = 318784766;
pub const DIBUTTON_BBALLD_FAKE: u32 = 318768131;
pub const DIBUTTON_BBALLD_FORWARD_LINK: u32 = 318850272;
pub const DIBUTTON_BBALLD_JUMP: u32 = 318768129;
pub const DIBUTTON_BBALLD_LEFT_LINK: u32 = 318817508;
pub const DIBUTTON_BBALLD_MENU: u32 = 318768381;
pub const DIBUTTON_BBALLD_PAUSE: u32 = 318784764;
pub const DIBUTTON_BBALLD_PLAY: u32 = 318768135;
pub const DIBUTTON_BBALLD_PLAYER: u32 = 318768133;
pub const DIBUTTON_BBALLD_RIGHT_LINK: u32 = 318817516;
pub const DIBUTTON_BBALLD_SPECIAL: u32 = 318768132;
pub const DIBUTTON_BBALLD_STEAL: u32 = 318768130;
pub const DIBUTTON_BBALLD_SUBSTITUTE: u32 = 318784521;
pub const DIBUTTON_BBALLD_TIMEOUT: u32 = 318784520;
pub const DIBUTTON_BBALLO_BACK_LINK: u32 = 302073064;
pub const DIBUTTON_BBALLO_BURST: u32 = 301990919;
pub const DIBUTTON_BBALLO_CALL: u32 = 301990920;
pub const DIBUTTON_BBALLO_DEVICE: u32 = 302007550;
pub const DIBUTTON_BBALLO_DUNK: u32 = 301990914;
pub const DIBUTTON_BBALLO_FAKE: u32 = 301990916;
pub const DIBUTTON_BBALLO_FORWARD_LINK: u32 = 302073056;
pub const DIBUTTON_BBALLO_JAB: u32 = 302007307;
pub const DIBUTTON_BBALLO_LEFT_LINK: u32 = 302040292;
pub const DIBUTTON_BBALLO_MENU: u32 = 301991165;
pub const DIBUTTON_BBALLO_PASS: u32 = 301990915;
pub const DIBUTTON_BBALLO_PAUSE: u32 = 302007548;
pub const DIBUTTON_BBALLO_PLAY: u32 = 302007306;
pub const DIBUTTON_BBALLO_PLAYER: u32 = 301990918;
pub const DIBUTTON_BBALLO_POST: u32 = 302007308;
pub const DIBUTTON_BBALLO_RIGHT_LINK: u32 = 302040300;
pub const DIBUTTON_BBALLO_SCREEN: u32 = 302007305;
pub const DIBUTTON_BBALLO_SHOOT: u32 = 301990913;
pub const DIBUTTON_BBALLO_SPECIAL: u32 = 301990917;
pub const DIBUTTON_BBALLO_SUBSTITUTE: u32 = 302007310;
pub const DIBUTTON_BBALLO_TIMEOUT: u32 = 302007309;
pub const DIBUTTON_BIKINGM_BRAKE_BUTTON_LINK: u32 = 470041832;
pub const DIBUTTON_BIKINGM_CAMERA: u32 = 469763074;
pub const DIBUTTON_BIKINGM_DEVICE: u32 = 469779710;
pub const DIBUTTON_BIKINGM_FASTER_LINK: u32 = 469845216;
pub const DIBUTTON_BIKINGM_JUMP: u32 = 469763073;
pub const DIBUTTON_BIKINGM_LEFT_LINK: u32 = 469812452;
pub const DIBUTTON_BIKINGM_MENU: u32 = 469763325;
pub const DIBUTTON_BIKINGM_PAUSE: u32 = 469779708;
pub const DIBUTTON_BIKINGM_RIGHT_LINK: u32 = 469812460;
pub const DIBUTTON_BIKINGM_SELECT: u32 = 469763076;
pub const DIBUTTON_BIKINGM_SLOWER_LINK: u32 = 469845224;
pub const DIBUTTON_BIKINGM_SPECIAL1: u32 = 469763075;
pub const DIBUTTON_BIKINGM_SPECIAL2: u32 = 469763077;
pub const DIBUTTON_BIKINGM_ZOOM: u32 = 469779462;
pub const DIBUTTON_BROWSER_DEVICE: u32 = 671106302;
pub const DIBUTTON_BROWSER_FAVORITES: u32 = 671106054;
pub const DIBUTTON_BROWSER_HISTORY: u32 = 671106057;
pub const DIBUTTON_BROWSER_HOME: u32 = 671106053;
pub const DIBUTTON_BROWSER_MENU: u32 = 671089917;
pub const DIBUTTON_BROWSER_NEXT: u32 = 671106055;
pub const DIBUTTON_BROWSER_PAUSE: u32 = 671106300;
pub const DIBUTTON_BROWSER_PREVIOUS: u32 = 671106056;
pub const DIBUTTON_BROWSER_PRINT: u32 = 671106058;
pub const DIBUTTON_BROWSER_REFRESH: u32 = 671089666;
pub const DIBUTTON_BROWSER_SEARCH: u32 = 671106051;
pub const DIBUTTON_BROWSER_SELECT: u32 = 671089665;
pub const DIBUTTON_BROWSER_STOP: u32 = 671106052;
pub const DIBUTTON_CADF_DEVICE: u32 = 620774654;
pub const DIBUTTON_CADF_DISPLAY: u32 = 620774405;
pub const DIBUTTON_CADF_MENU: u32 = 620758269;
pub const DIBUTTON_CADF_PAUSE: u32 = 620774652;
pub const DIBUTTON_CADF_SELECT: u32 = 620758017;
pub const DIBUTTON_CADF_SPECIAL: u32 = 620758019;
pub const DIBUTTON_CADF_SPECIAL1: u32 = 620758018;
pub const DIBUTTON_CADF_SPECIAL2: u32 = 620758020;
pub const DIBUTTON_CADM_DEVICE: u32 = 637551870;
pub const DIBUTTON_CADM_DISPLAY: u32 = 637551621;
pub const DIBUTTON_CADM_MENU: u32 = 637535485;
pub const DIBUTTON_CADM_PAUSE: u32 = 637551868;
pub const DIBUTTON_CADM_SELECT: u32 = 637535233;
pub const DIBUTTON_CADM_SPECIAL: u32 = 637535235;
pub const DIBUTTON_CADM_SPECIAL1: u32 = 637535234;
pub const DIBUTTON_CADM_SPECIAL2: u32 = 637535236;
pub const DIBUTTON_DRIVINGC_ACCELERATE_LINK: u32 = 33805536;
pub const DIBUTTON_DRIVINGC_AIDS: u32 = 33571847;
pub const DIBUTTON_DRIVINGC_BRAKE: u32 = 33573896;
pub const DIBUTTON_DRIVINGC_DASHBOARD: u32 = 33571846;
pub const DIBUTTON_DRIVINGC_DEVICE: u32 = 33572094;
pub const DIBUTTON_DRIVINGC_FIRE: u32 = 33557505;
pub const DIBUTTON_DRIVINGC_FIRESECONDARY: u32 = 33573897;
pub const DIBUTTON_DRIVINGC_GLANCE_LEFT_LINK: u32 = 34063588;
pub const DIBUTTON_DRIVINGC_GLANCE_RIGHT_LINK: u32 = 34063596;
pub const DIBUTTON_DRIVINGC_MENU: u32 = 33555709;
pub const DIBUTTON_DRIVINGC_PAUSE: u32 = 33572092;
pub const DIBUTTON_DRIVINGC_SHIFTDOWN: u32 = 33573893;
pub const DIBUTTON_DRIVINGC_SHIFTUP: u32 = 33573892;
pub const DIBUTTON_DRIVINGC_STEER_LEFT_LINK: u32 = 33606884;
pub const DIBUTTON_DRIVINGC_STEER_RIGHT_LINK: u32 = 33606892;
pub const DIBUTTON_DRIVINGC_TARGET: u32 = 33557507;
pub const DIBUTTON_DRIVINGC_WEAPONS: u32 = 33557506;
pub const DIBUTTON_DRIVINGR_ACCELERATE_LINK: u32 = 17028320;
pub const DIBUTTON_DRIVINGR_AIDS: u32 = 16794630;
pub const DIBUTTON_DRIVINGR_BOOST: u32 = 16794632;
pub const DIBUTTON_DRIVINGR_BRAKE: u32 = 16796676;
pub const DIBUTTON_DRIVINGR_DASHBOARD: u32 = 16794629;
pub const DIBUTTON_DRIVINGR_DEVICE: u32 = 16794878;
pub const DIBUTTON_DRIVINGR_GLANCE_LEFT_LINK: u32 = 17286372;
pub const DIBUTTON_DRIVINGR_GLANCE_RIGHT_LINK: u32 = 17286380;
pub const DIBUTTON_DRIVINGR_MAP: u32 = 16794631;
pub const DIBUTTON_DRIVINGR_MENU: u32 = 16778493;
pub const DIBUTTON_DRIVINGR_PAUSE: u32 = 16794876;
pub const DIBUTTON_DRIVINGR_PIT: u32 = 16794633;
pub const DIBUTTON_DRIVINGR_SHIFTDOWN: u32 = 16780290;
pub const DIBUTTON_DRIVINGR_SHIFTUP: u32 = 16780289;
pub const DIBUTTON_DRIVINGR_STEER_LEFT_LINK: u32 = 16829668;
pub const DIBUTTON_DRIVINGR_STEER_RIGHT_LINK: u32 = 16829676;
pub const DIBUTTON_DRIVINGR_VIEW: u32 = 16784387;
pub const DIBUTTON_DRIVINGT_ACCELERATE_LINK: u32 = 50582752;
pub const DIBUTTON_DRIVINGT_BARREL_DOWN_LINK: u32 = 50414824;
pub const DIBUTTON_DRIVINGT_BARREL_UP_LINK: u32 = 50414816;
pub const DIBUTTON_DRIVINGT_BRAKE: u32 = 50351110;
pub const DIBUTTON_DRIVINGT_DASHBOARD: u32 = 50355205;
pub const DIBUTTON_DRIVINGT_DEVICE: u32 = 50349310;
pub const DIBUTTON_DRIVINGT_FIRE: u32 = 50334721;
pub const DIBUTTON_DRIVINGT_FIRESECONDARY: u32 = 50351111;
pub const DIBUTTON_DRIVINGT_GLANCE_LEFT_LINK: u32 = 50840804;
pub const DIBUTTON_DRIVINGT_GLANCE_RIGHT_LINK: u32 = 50840812;
pub const DIBUTTON_DRIVINGT_MENU: u32 = 50332925;
pub const DIBUTTON_DRIVINGT_PAUSE: u32 = 50349308;
pub const DIBUTTON_DRIVINGT_ROTATE_LEFT_LINK: u32 = 50480356;
pub const DIBUTTON_DRIVINGT_ROTATE_RIGHT_LINK: u32 = 50480364;
pub const DIBUTTON_DRIVINGT_STEER_LEFT_LINK: u32 = 50384100;
pub const DIBUTTON_DRIVINGT_STEER_RIGHT_LINK: u32 = 50384108;
pub const DIBUTTON_DRIVINGT_TARGET: u32 = 50334723;
pub const DIBUTTON_DRIVINGT_VIEW: u32 = 50355204;
pub const DIBUTTON_DRIVINGT_WEAPONS: u32 = 50334722;
pub const DIBUTTON_FIGHTINGH_BACKWARD_LINK: u32 = 134300904;
pub const DIBUTTON_FIGHTINGH_BLOCK: u32 = 134218755;
pub const DIBUTTON_FIGHTINGH_CROUCH: u32 = 134218756;
pub const DIBUTTON_FIGHTINGH_DEVICE: u32 = 134235390;
pub const DIBUTTON_FIGHTINGH_DISPLAY: u32 = 134235145;
pub const DIBUTTON_FIGHTINGH_DODGE: u32 = 134235146;
pub const DIBUTTON_FIGHTINGH_FORWARD_LINK: u32 = 134300896;
pub const DIBUTTON_FIGHTINGH_JUMP: u32 = 134218757;
pub const DIBUTTON_FIGHTINGH_KICK: u32 = 134218754;
pub const DIBUTTON_FIGHTINGH_LEFT_LINK: u32 = 134268132;
pub const DIBUTTON_FIGHTINGH_MENU: u32 = 134219005;
pub const DIBUTTON_FIGHTINGH_PAUSE: u32 = 134235388;
pub const DIBUTTON_FIGHTINGH_PUNCH: u32 = 134218753;
pub const DIBUTTON_FIGHTINGH_RIGHT_LINK: u32 = 134268140;
pub const DIBUTTON_FIGHTINGH_SELECT: u32 = 134235144;
pub const DIBUTTON_FIGHTINGH_SPECIAL1: u32 = 134218758;
pub const DIBUTTON_FIGHTINGH_SPECIAL2: u32 = 134218759;
pub const DIBUTTON_FISHING_BACK_LINK: u32 = 234964200;
pub const DIBUTTON_FISHING_BAIT: u32 = 234882052;
pub const DIBUTTON_FISHING_BINOCULAR: u32 = 234882051;
pub const DIBUTTON_FISHING_CAST: u32 = 234882049;
pub const DIBUTTON_FISHING_CROUCH: u32 = 234898439;
pub const DIBUTTON_FISHING_DEVICE: u32 = 234898686;
pub const DIBUTTON_FISHING_DISPLAY: u32 = 234898438;
pub const DIBUTTON_FISHING_FORWARD_LINK: u32 = 234964192;
pub const DIBUTTON_FISHING_JUMP: u32 = 234898440;
pub const DIBUTTON_FISHING_LEFT_LINK: u32 = 234931428;
pub const DIBUTTON_FISHING_MAP: u32 = 234882053;
pub const DIBUTTON_FISHING_MENU: u32 = 234882301;
pub const DIBUTTON_FISHING_PAUSE: u32 = 234898684;
pub const DIBUTTON_FISHING_RIGHT_LINK: u32 = 234931436;
pub const DIBUTTON_FISHING_ROTATE_LEFT_LINK: u32 = 235029732;
pub const DIBUTTON_FISHING_ROTATE_RIGHT_LINK: u32 = 235029740;
pub const DIBUTTON_FISHING_TYPE: u32 = 234882050;
pub const DIBUTTON_FLYINGC_BRAKE_LINK: u32 = 67398880;
pub const DIBUTTON_FLYINGC_DEVICE: u32 = 67126526;
pub const DIBUTTON_FLYINGC_DISPLAY: u32 = 67118082;
pub const DIBUTTON_FLYINGC_FASTER_LINK: u32 = 67359968;
pub const DIBUTTON_FLYINGC_FLAPSDOWN: u32 = 67134469;
pub const DIBUTTON_FLYINGC_FLAPSUP: u32 = 67134468;
pub const DIBUTTON_FLYINGC_GEAR: u32 = 67120131;
pub const DIBUTTON_FLYINGC_GLANCE_DOWN_LINK: u32 = 67618024;
pub const DIBUTTON_FLYINGC_GLANCE_LEFT_LINK: u32 = 67618020;
pub const DIBUTTON_FLYINGC_GLANCE_RIGHT_LINK: u32 = 67618028;
pub const DIBUTTON_FLYINGC_GLANCE_UP_LINK: u32 = 67618016;
pub const DIBUTTON_FLYINGC_MENU: u32 = 67110141;
pub const DIBUTTON_FLYINGC_PAUSE: u32 = 67126524;
pub const DIBUTTON_FLYINGC_SLOWER_LINK: u32 = 67359976;
pub const DIBUTTON_FLYINGC_VIEW: u32 = 67118081;
pub const DIBUTTON_FLYINGH_COUNTER: u32 = 100684804;
pub const DIBUTTON_FLYINGH_DEVICE: u32 = 100680958;
pub const DIBUTTON_FLYINGH_FASTER_LINK: u32 = 100916448;
pub const DIBUTTON_FLYINGH_FIRE: u32 = 100668417;
pub const DIBUTTON_FLYINGH_FIRESECONDARY: u32 = 100682759;
pub const DIBUTTON_FLYINGH_GEAR: u32 = 100688902;
pub const DIBUTTON_FLYINGH_GLANCE_DOWN_LINK: u32 = 101172456;
pub const DIBUTTON_FLYINGH_GLANCE_LEFT_LINK: u32 = 101172452;
pub const DIBUTTON_FLYINGH_GLANCE_RIGHT_LINK: u32 = 101172460;
pub const DIBUTTON_FLYINGH_GLANCE_UP_LINK: u32 = 101172448;
pub const DIBUTTON_FLYINGH_MENU: u32 = 100664573;
pub const DIBUTTON_FLYINGH_PAUSE: u32 = 100680956;
pub const DIBUTTON_FLYINGH_SLOWER_LINK: u32 = 100916456;
pub const DIBUTTON_FLYINGH_TARGET: u32 = 100668419;
pub const DIBUTTON_FLYINGH_VIEW: u32 = 100688901;
pub const DIBUTTON_FLYINGH_WEAPONS: u32 = 100668418;
pub const DIBUTTON_FLYINGM_BRAKE_LINK: u32 = 84174048;
pub const DIBUTTON_FLYINGM_COUNTER: u32 = 83909636;
pub const DIBUTTON_FLYINGM_DEVICE: u32 = 83903742;
pub const DIBUTTON_FLYINGM_DISPLAY: u32 = 83911686;
pub const DIBUTTON_FLYINGM_FASTER_LINK: u32 = 84137184;
pub const DIBUTTON_FLYINGM_FIRE: u32 = 83889153;
pub const DIBUTTON_FLYINGM_FIRESECONDARY: u32 = 83905545;
pub const DIBUTTON_FLYINGM_FLAPSDOWN: u32 = 83907592;
pub const DIBUTTON_FLYINGM_FLAPSUP: u32 = 83907591;
pub const DIBUTTON_FLYINGM_GEAR: u32 = 83911690;
pub const DIBUTTON_FLYINGM_GLANCE_DOWN_LINK: u32 = 84395240;
pub const DIBUTTON_FLYINGM_GLANCE_LEFT_LINK: u32 = 84395236;
pub const DIBUTTON_FLYINGM_GLANCE_RIGHT_LINK: u32 = 84395244;
pub const DIBUTTON_FLYINGM_GLANCE_UP_LINK: u32 = 84395232;
pub const DIBUTTON_FLYINGM_MENU: u32 = 83887357;
pub const DIBUTTON_FLYINGM_PAUSE: u32 = 83903740;
pub const DIBUTTON_FLYINGM_SLOWER_LINK: u32 = 84137192;
pub const DIBUTTON_FLYINGM_TARGET: u32 = 83889155;
pub const DIBUTTON_FLYINGM_VIEW: u32 = 83911685;
pub const DIBUTTON_FLYINGM_WEAPONS: u32 = 83889154;
pub const DIBUTTON_FOOTBALLD_AUDIBLE: u32 = 385893387;
pub const DIBUTTON_FOOTBALLD_BACK_LINK: u32 = 385959144;
pub const DIBUTTON_FOOTBALLD_BULLRUSH: u32 = 385893385;
pub const DIBUTTON_FOOTBALLD_DEVICE: u32 = 385893630;
pub const DIBUTTON_FOOTBALLD_FAKE: u32 = 385876997;
pub const DIBUTTON_FOOTBALLD_FORWARD_LINK: u32 = 385959136;
pub const DIBUTTON_FOOTBALLD_JUMP: u32 = 385876995;
pub const DIBUTTON_FOOTBALLD_LEFT_LINK: u32 = 385926372;
pub const DIBUTTON_FOOTBALLD_MENU: u32 = 385877245;
pub const DIBUTTON_FOOTBALLD_PAUSE: u32 = 385893628;
pub const DIBUTTON_FOOTBALLD_PLAY: u32 = 385876993;
pub const DIBUTTON_FOOTBALLD_RIGHT_LINK: u32 = 385926380;
pub const DIBUTTON_FOOTBALLD_RIP: u32 = 385893386;
pub const DIBUTTON_FOOTBALLD_SELECT: u32 = 385876994;
pub const DIBUTTON_FOOTBALLD_SPIN: u32 = 385893383;
pub const DIBUTTON_FOOTBALLD_SUBSTITUTE: u32 = 385893389;
pub const DIBUTTON_FOOTBALLD_SUPERTACKLE: u32 = 385876998;
pub const DIBUTTON_FOOTBALLD_SWIM: u32 = 385893384;
pub const DIBUTTON_FOOTBALLD_TACKLE: u32 = 385876996;
pub const DIBUTTON_FOOTBALLD_ZOOM: u32 = 385893388;
pub const DIBUTTON_FOOTBALLO_BACK_LINK: u32 = 369181928;
pub const DIBUTTON_FOOTBALLO_DEVICE: u32 = 369116414;
pub const DIBUTTON_FOOTBALLO_DIVE: u32 = 369116169;
pub const DIBUTTON_FOOTBALLO_FORWARD_LINK: u32 = 369181920;
pub const DIBUTTON_FOOTBALLO_JUKE: u32 = 369116166;
pub const DIBUTTON_FOOTBALLO_JUMP: u32 = 369099777;
pub const DIBUTTON_FOOTBALLO_LEFTARM: u32 = 369099778;
pub const DIBUTTON_FOOTBALLO_LEFT_LINK: u32 = 369149156;
pub const DIBUTTON_FOOTBALLO_MENU: u32 = 369100029;
pub const DIBUTTON_FOOTBALLO_PAUSE: u32 = 369116412;
pub const DIBUTTON_FOOTBALLO_RIGHTARM: u32 = 369099779;
pub const DIBUTTON_FOOTBALLO_RIGHT_LINK: u32 = 369149164;
pub const DIBUTTON_FOOTBALLO_SHOULDER: u32 = 369116167;
pub const DIBUTTON_FOOTBALLO_SPIN: u32 = 369099781;
pub const DIBUTTON_FOOTBALLO_SUBSTITUTE: u32 = 369116171;
pub const DIBUTTON_FOOTBALLO_THROW: u32 = 369099780;
pub const DIBUTTON_FOOTBALLO_TURBO: u32 = 369116168;
pub const DIBUTTON_FOOTBALLO_ZOOM: u32 = 369116170;
pub const DIBUTTON_FOOTBALLP_DEVICE: u32 = 335561982;
pub const DIBUTTON_FOOTBALLP_HELP: u32 = 335545347;
pub const DIBUTTON_FOOTBALLP_MENU: u32 = 335545597;
pub const DIBUTTON_FOOTBALLP_PAUSE: u32 = 335561980;
pub const DIBUTTON_FOOTBALLP_PLAY: u32 = 335545345;
pub const DIBUTTON_FOOTBALLP_SELECT: u32 = 335545346;
pub const DIBUTTON_FOOTBALLQ_AUDIBLE: u32 = 352338953;
pub const DIBUTTON_FOOTBALLQ_BACK_LINK: u32 = 352404712;
pub const DIBUTTON_FOOTBALLQ_DEVICE: u32 = 352339198;
pub const DIBUTTON_FOOTBALLQ_FAKE: u32 = 352322566;
pub const DIBUTTON_FOOTBALLQ_FAKESNAP: u32 = 352338951;
pub const DIBUTTON_FOOTBALLQ_FORWARD_LINK: u32 = 352404704;
pub const DIBUTTON_FOOTBALLQ_JUMP: u32 = 352322563;
pub const DIBUTTON_FOOTBALLQ_LEFT_LINK: u32 = 352371940;
pub const DIBUTTON_FOOTBALLQ_MENU: u32 = 352322813;
pub const DIBUTTON_FOOTBALLQ_MOTION: u32 = 352338952;
pub const DIBUTTON_FOOTBALLQ_PASS: u32 = 352322565;
pub const DIBUTTON_FOOTBALLQ_PAUSE: u32 = 352339196;
pub const DIBUTTON_FOOTBALLQ_RIGHT_LINK: u32 = 352371948;
pub const DIBUTTON_FOOTBALLQ_SELECT: u32 = 352322561;
pub const DIBUTTON_FOOTBALLQ_SLIDE: u32 = 352322564;
pub const DIBUTTON_FOOTBALLQ_SNAP: u32 = 352322562;
pub const DIBUTTON_FPS_APPLY: u32 = 150995971;
pub const DIBUTTON_FPS_BACKWARD_LINK: u32 = 151078120;
pub const DIBUTTON_FPS_CROUCH: u32 = 150995973;
pub const DIBUTTON_FPS_DEVICE: u32 = 151012606;
pub const DIBUTTON_FPS_DISPLAY: u32 = 151012360;
pub const DIBUTTON_FPS_DODGE: u32 = 151012361;
pub const DIBUTTON_FPS_FIRE: u32 = 150995969;
pub const DIBUTTON_FPS_FIRESECONDARY: u32 = 151012364;
pub const DIBUTTON_FPS_FORWARD_LINK: u32 = 151078112;
pub const DIBUTTON_FPS_GLANCEL: u32 = 151012362;
pub const DIBUTTON_FPS_GLANCER: u32 = 151012363;
pub const DIBUTTON_FPS_GLANCE_DOWN_LINK: u32 = 151110888;
pub const DIBUTTON_FPS_GLANCE_UP_LINK: u32 = 151110880;
pub const DIBUTTON_FPS_JUMP: u32 = 150995974;
pub const DIBUTTON_FPS_MENU: u32 = 150996221;
pub const DIBUTTON_FPS_PAUSE: u32 = 151012604;
pub const DIBUTTON_FPS_ROTATE_LEFT_LINK: u32 = 151045348;
pub const DIBUTTON_FPS_ROTATE_RIGHT_LINK: u32 = 151045356;
pub const DIBUTTON_FPS_SELECT: u32 = 150995972;
pub const DIBUTTON_FPS_STEP_LEFT_LINK: u32 = 151143652;
pub const DIBUTTON_FPS_STEP_RIGHT_LINK: u32 = 151143660;
pub const DIBUTTON_FPS_STRAFE: u32 = 150995975;
pub const DIBUTTON_FPS_WEAPONS: u32 = 150995970;
pub const DIBUTTON_GOLF_BACK_LINK: u32 = 402736360;
pub const DIBUTTON_GOLF_DEVICE: u32 = 402670846;
pub const DIBUTTON_GOLF_DOWN: u32 = 402654212;
pub const DIBUTTON_GOLF_FLYBY: u32 = 402654214;
pub const DIBUTTON_GOLF_FORWARD_LINK: u32 = 402736352;
pub const DIBUTTON_GOLF_LEFT_LINK: u32 = 402703588;
pub const DIBUTTON_GOLF_MENU: u32 = 402654461;
pub const DIBUTTON_GOLF_PAUSE: u32 = 402670844;
pub const DIBUTTON_GOLF_RIGHT_LINK: u32 = 402703596;
pub const DIBUTTON_GOLF_SELECT: u32 = 402654210;
pub const DIBUTTON_GOLF_SUBSTITUTE: u32 = 402670601;
pub const DIBUTTON_GOLF_SWING: u32 = 402654209;
pub const DIBUTTON_GOLF_TERRAIN: u32 = 402654213;
pub const DIBUTTON_GOLF_TIMEOUT: u32 = 402670600;
pub const DIBUTTON_GOLF_UP: u32 = 402654211;
pub const DIBUTTON_GOLF_ZOOM: u32 = 402670599;
pub const DIBUTTON_HOCKEYD_BACK_LINK: u32 = 436290792;
pub const DIBUTTON_HOCKEYD_BLOCK: u32 = 436208644;
pub const DIBUTTON_HOCKEYD_BURST: u32 = 436208643;
pub const DIBUTTON_HOCKEYD_DEVICE: u32 = 436225278;
pub const DIBUTTON_HOCKEYD_FAKE: u32 = 436208645;
pub const DIBUTTON_HOCKEYD_FORWARD_LINK: u32 = 436290784;
pub const DIBUTTON_HOCKEYD_LEFT_LINK: u32 = 436258020;
pub const DIBUTTON_HOCKEYD_MENU: u32 = 436208893;
pub const DIBUTTON_HOCKEYD_PAUSE: u32 = 436225276;
pub const DIBUTTON_HOCKEYD_PLAYER: u32 = 436208641;
pub const DIBUTTON_HOCKEYD_RIGHT_LINK: u32 = 436258028;
pub const DIBUTTON_HOCKEYD_STEAL: u32 = 436208642;
pub const DIBUTTON_HOCKEYD_STRATEGY: u32 = 436225031;
pub const DIBUTTON_HOCKEYD_SUBSTITUTE: u32 = 436225033;
pub const DIBUTTON_HOCKEYD_TIMEOUT: u32 = 436225032;
pub const DIBUTTON_HOCKEYD_ZOOM: u32 = 436225030;
pub const DIBUTTON_HOCKEYG_BACK_LINK: u32 = 453068008;
pub const DIBUTTON_HOCKEYG_BLOCK: u32 = 452985860;
pub const DIBUTTON_HOCKEYG_DEVICE: u32 = 453002494;
pub const DIBUTTON_HOCKEYG_FORWARD_LINK: u32 = 453068000;
pub const DIBUTTON_HOCKEYG_LEFT_LINK: u32 = 453035236;
pub const DIBUTTON_HOCKEYG_MENU: u32 = 452986109;
pub const DIBUTTON_HOCKEYG_PASS: u32 = 452985857;
pub const DIBUTTON_HOCKEYG_PAUSE: u32 = 453002492;
pub const DIBUTTON_HOCKEYG_POKE: u32 = 452985858;
pub const DIBUTTON_HOCKEYG_RIGHT_LINK: u32 = 453035244;
pub const DIBUTTON_HOCKEYG_STEAL: u32 = 452985859;
pub const DIBUTTON_HOCKEYG_STRATEGY: u32 = 453002246;
pub const DIBUTTON_HOCKEYG_SUBSTITUTE: u32 = 453002248;
pub const DIBUTTON_HOCKEYG_TIMEOUT: u32 = 453002247;
pub const DIBUTTON_HOCKEYG_ZOOM: u32 = 453002245;
pub const DIBUTTON_HOCKEYO_BACK_LINK: u32 = 419513576;
pub const DIBUTTON_HOCKEYO_BURST: u32 = 419431427;
pub const DIBUTTON_HOCKEYO_DEVICE: u32 = 419448062;
pub const DIBUTTON_HOCKEYO_FAKE: u32 = 419431429;
pub const DIBUTTON_HOCKEYO_FORWARD_LINK: u32 = 419513568;
pub const DIBUTTON_HOCKEYO_LEFT_LINK: u32 = 419480804;
pub const DIBUTTON_HOCKEYO_MENU: u32 = 419431677;
pub const DIBUTTON_HOCKEYO_PASS: u32 = 419431426;
pub const DIBUTTON_HOCKEYO_PAUSE: u32 = 419448060;
pub const DIBUTTON_HOCKEYO_RIGHT_LINK: u32 = 419480812;
pub const DIBUTTON_HOCKEYO_SHOOT: u32 = 419431425;
pub const DIBUTTON_HOCKEYO_SPECIAL: u32 = 419431428;
pub const DIBUTTON_HOCKEYO_STRATEGY: u32 = 419447815;
pub const DIBUTTON_HOCKEYO_SUBSTITUTE: u32 = 419447817;
pub const DIBUTTON_HOCKEYO_TIMEOUT: u32 = 419447816;
pub const DIBUTTON_HOCKEYO_ZOOM: u32 = 419447814;
pub const DIBUTTON_HUNTING_AIM: u32 = 218104834;
pub const DIBUTTON_HUNTING_BACK_LINK: u32 = 218186984;
pub const DIBUTTON_HUNTING_BINOCULAR: u32 = 218104836;
pub const DIBUTTON_HUNTING_CALL: u32 = 218104837;
pub const DIBUTTON_HUNTING_CROUCH: u32 = 218121225;
pub const DIBUTTON_HUNTING_DEVICE: u32 = 218121470;
pub const DIBUTTON_HUNTING_DISPLAY: u32 = 218121224;
pub const DIBUTTON_HUNTING_FIRE: u32 = 218104833;
pub const DIBUTTON_HUNTING_FIRESECONDARY: u32 = 218121227;
pub const DIBUTTON_HUNTING_FORWARD_LINK: u32 = 218186976;
pub const DIBUTTON_HUNTING_JUMP: u32 = 218121226;
pub const DIBUTTON_HUNTING_LEFT_LINK: u32 = 218154212;
pub const DIBUTTON_HUNTING_MAP: u32 = 218104838;
pub const DIBUTTON_HUNTING_MENU: u32 = 218105085;
pub const DIBUTTON_HUNTING_PAUSE: u32 = 218121468;
pub const DIBUTTON_HUNTING_RIGHT_LINK: u32 = 218154220;
pub const DIBUTTON_HUNTING_ROTATE_LEFT_LINK: u32 = 218252516;
pub const DIBUTTON_HUNTING_ROTATE_RIGHT_LINK: u32 = 218252524;
pub const DIBUTTON_HUNTING_SPECIAL: u32 = 218104839;
pub const DIBUTTON_HUNTING_WEAPON: u32 = 218104835;
pub const DIBUTTON_MECHA_BACK_LINK: u32 = 687949032;
pub const DIBUTTON_MECHA_CENTER: u32 = 687883271;
pub const DIBUTTON_MECHA_DEVICE: u32 = 687883518;
pub const DIBUTTON_MECHA_FASTER_LINK: u32 = 688112864;
pub const DIBUTTON_MECHA_FIRE: u32 = 687866881;
pub const DIBUTTON_MECHA_FIRESECONDARY: u32 = 687883273;
pub const DIBUTTON_MECHA_FORWARD_LINK: u32 = 687949024;
pub const DIBUTTON_MECHA_JUMP: u32 = 687866886;
pub const DIBUTTON_MECHA_LEFT_LINK: u32 = 687916260;
pub const DIBUTTON_MECHA_MENU: u32 = 687867133;
pub const DIBUTTON_MECHA_PAUSE: u32 = 687883516;
pub const DIBUTTON_MECHA_REVERSE: u32 = 687866884;
pub const DIBUTTON_MECHA_RIGHT_LINK: u32 = 687916268;
pub const DIBUTTON_MECHA_ROTATE_LEFT_LINK: u32 = 688014564;
pub const DIBUTTON_MECHA_ROTATE_RIGHT_LINK: u32 = 688014572;
pub const DIBUTTON_MECHA_SLOWER_LINK: u32 = 688112872;
pub const DIBUTTON_MECHA_TARGET: u32 = 687866883;
pub const DIBUTTON_MECHA_VIEW: u32 = 687883272;
pub const DIBUTTON_MECHA_WEAPONS: u32 = 687866882;
pub const DIBUTTON_MECHA_ZOOM: u32 = 687866885;
pub const DIBUTTON_RACQUET_BACKSWING: u32 = 536871938;
pub const DIBUTTON_RACQUET_BACK_LINK: u32 = 536954088;
pub const DIBUTTON_RACQUET_DEVICE: u32 = 536888574;
pub const DIBUTTON_RACQUET_FORWARD_LINK: u32 = 536954080;
pub const DIBUTTON_RACQUET_LEFT_LINK: u32 = 536921316;
pub const DIBUTTON_RACQUET_MENU: u32 = 536872189;
pub const DIBUTTON_RACQUET_PAUSE: u32 = 536888572;
pub const DIBUTTON_RACQUET_RIGHT_LINK: u32 = 536921324;
pub const DIBUTTON_RACQUET_SELECT: u32 = 536871941;
pub const DIBUTTON_RACQUET_SMASH: u32 = 536871939;
pub const DIBUTTON_RACQUET_SPECIAL: u32 = 536871940;
pub const DIBUTTON_RACQUET_SUBSTITUTE: u32 = 536888327;
pub const DIBUTTON_RACQUET_SWING: u32 = 536871937;
pub const DIBUTTON_RACQUET_TIMEOUT: u32 = 536888326;
pub const DIBUTTON_REMOTE_ADJUST: u32 = 654334990;
pub const DIBUTTON_REMOTE_CABLE: u32 = 654334985;
pub const DIBUTTON_REMOTE_CD: u32 = 654334986;
pub const DIBUTTON_REMOTE_CHANGE: u32 = 654320646;
pub const DIBUTTON_REMOTE_CUE: u32 = 654320644;
pub const DIBUTTON_REMOTE_DEVICE: u32 = 654329086;
pub const DIBUTTON_REMOTE_DIGIT0: u32 = 654332943;
pub const DIBUTTON_REMOTE_DIGIT1: u32 = 654332944;
pub const DIBUTTON_REMOTE_DIGIT2: u32 = 654332945;
pub const DIBUTTON_REMOTE_DIGIT3: u32 = 654332946;
pub const DIBUTTON_REMOTE_DIGIT4: u32 = 654332947;
pub const DIBUTTON_REMOTE_DIGIT5: u32 = 654332948;
pub const DIBUTTON_REMOTE_DIGIT6: u32 = 654332949;
pub const DIBUTTON_REMOTE_DIGIT7: u32 = 654332950;
pub const DIBUTTON_REMOTE_DIGIT8: u32 = 654332951;
pub const DIBUTTON_REMOTE_DIGIT9: u32 = 654332952;
pub const DIBUTTON_REMOTE_DVD: u32 = 654334989;
pub const DIBUTTON_REMOTE_MENU: u32 = 654312701;
pub const DIBUTTON_REMOTE_MUTE: u32 = 654312449;
pub const DIBUTTON_REMOTE_PAUSE: u32 = 654329084;
pub const DIBUTTON_REMOTE_PLAY: u32 = 654320643;
pub const DIBUTTON_REMOTE_RECORD: u32 = 654320647;
pub const DIBUTTON_REMOTE_REVIEW: u32 = 654320645;
pub const DIBUTTON_REMOTE_SELECT: u32 = 654312450;
pub const DIBUTTON_REMOTE_TUNER: u32 = 654334988;
pub const DIBUTTON_REMOTE_TV: u32 = 654334984;
pub const DIBUTTON_REMOTE_VCR: u32 = 654334987;
pub const DIBUTTON_SKIING_CAMERA: u32 = 486540291;
pub const DIBUTTON_SKIING_CROUCH: u32 = 486540290;
pub const DIBUTTON_SKIING_DEVICE: u32 = 486556926;
pub const DIBUTTON_SKIING_FASTER_LINK: u32 = 486622432;
pub const DIBUTTON_SKIING_JUMP: u32 = 486540289;
pub const DIBUTTON_SKIING_LEFT_LINK: u32 = 486589668;
pub const DIBUTTON_SKIING_MENU: u32 = 486540541;
pub const DIBUTTON_SKIING_PAUSE: u32 = 486556924;
pub const DIBUTTON_SKIING_RIGHT_LINK: u32 = 486589676;
pub const DIBUTTON_SKIING_SELECT: u32 = 486540293;
pub const DIBUTTON_SKIING_SLOWER_LINK: u32 = 486622440;
pub const DIBUTTON_SKIING_SPECIAL1: u32 = 486540292;
pub const DIBUTTON_SKIING_SPECIAL2: u32 = 486540294;
pub const DIBUTTON_SKIING_ZOOM: u32 = 486556679;
pub const DIBUTTON_SOCCERD_BACK_LINK: u32 = 520176872;
pub const DIBUTTON_SOCCERD_BLOCK: u32 = 520094721;
pub const DIBUTTON_SOCCERD_CLEAR: u32 = 520111114;
pub const DIBUTTON_SOCCERD_DEVICE: u32 = 520111358;
pub const DIBUTTON_SOCCERD_FAKE: u32 = 520094723;
pub const DIBUTTON_SOCCERD_FORWARD_LINK: u32 = 520176864;
pub const DIBUTTON_SOCCERD_FOUL: u32 = 520111112;
pub const DIBUTTON_SOCCERD_GOALIECHARGE: u32 = 520111115;
pub const DIBUTTON_SOCCERD_HEAD: u32 = 520111113;
pub const DIBUTTON_SOCCERD_LEFT_LINK: u32 = 520144100;
pub const DIBUTTON_SOCCERD_MENU: u32 = 520094973;
pub const DIBUTTON_SOCCERD_PAUSE: u32 = 520111356;
pub const DIBUTTON_SOCCERD_PLAYER: u32 = 520094724;
pub const DIBUTTON_SOCCERD_RIGHT_LINK: u32 = 520144108;
pub const DIBUTTON_SOCCERD_SELECT: u32 = 520094726;
pub const DIBUTTON_SOCCERD_SLIDE: u32 = 520094727;
pub const DIBUTTON_SOCCERD_SPECIAL: u32 = 520094725;
pub const DIBUTTON_SOCCERD_STEAL: u32 = 520094722;
pub const DIBUTTON_SOCCERD_SUBSTITUTE: u32 = 520111116;
pub const DIBUTTON_SOCCERO_BACK_LINK: u32 = 503399656;
pub const DIBUTTON_SOCCERO_CONTROL: u32 = 503333900;
pub const DIBUTTON_SOCCERO_DEVICE: u32 = 503334142;
pub const DIBUTTON_SOCCERO_FAKE: u32 = 503317507;
pub const DIBUTTON_SOCCERO_FORWARD_LINK: u32 = 503399648;
pub const DIBUTTON_SOCCERO_HEAD: u32 = 503333901;
pub const DIBUTTON_SOCCERO_LEFT_LINK: u32 = 503366884;
pub const DIBUTTON_SOCCERO_MENU: u32 = 503317757;
pub const DIBUTTON_SOCCERO_PASS: u32 = 503317506;
pub const DIBUTTON_SOCCERO_PASSTHRU: u32 = 503333898;
pub const DIBUTTON_SOCCERO_PAUSE: u32 = 503334140;
pub const DIBUTTON_SOCCERO_PLAYER: u32 = 503317508;
pub const DIBUTTON_SOCCERO_RIGHT_LINK: u32 = 503366892;
pub const DIBUTTON_SOCCERO_SELECT: u32 = 503317510;
pub const DIBUTTON_SOCCERO_SHOOT: u32 = 503317505;
pub const DIBUTTON_SOCCERO_SHOOTHIGH: u32 = 503333897;
pub const DIBUTTON_SOCCERO_SHOOTLOW: u32 = 503333896;
pub const DIBUTTON_SOCCERO_SPECIAL1: u32 = 503317509;
pub const DIBUTTON_SOCCERO_SPRINT: u32 = 503333899;
pub const DIBUTTON_SOCCERO_SUBSTITUTE: u32 = 503333895;
pub const DIBUTTON_SPACESIM_BACKWARD_LINK: u32 = 117523688;
pub const DIBUTTON_SPACESIM_DEVICE: u32 = 117458174;
pub const DIBUTTON_SPACESIM_DISPLAY: u32 = 117457925;
pub const DIBUTTON_SPACESIM_FASTER_LINK: u32 = 117687520;
pub const DIBUTTON_SPACESIM_FIRE: u32 = 117441537;
pub const DIBUTTON_SPACESIM_FIRESECONDARY: u32 = 117457929;
pub const DIBUTTON_SPACESIM_FORWARD_LINK: u32 = 117523680;
pub const DIBUTTON_SPACESIM_GEAR: u32 = 117457928;
pub const DIBUTTON_SPACESIM_GLANCE_DOWN_LINK: u32 = 117949672;
pub const DIBUTTON_SPACESIM_GLANCE_LEFT_LINK: u32 = 117949668;
pub const DIBUTTON_SPACESIM_GLANCE_RIGHT_LINK: u32 = 117949676;
pub const DIBUTTON_SPACESIM_GLANCE_UP_LINK: u32 = 117949664;
pub const DIBUTTON_SPACESIM_LEFT_LINK: u32 = 117490916;
pub const DIBUTTON_SPACESIM_LOWER: u32 = 117457927;
pub const DIBUTTON_SPACESIM_MENU: u32 = 117441789;
pub const DIBUTTON_SPACESIM_PAUSE: u32 = 117458172;
pub const DIBUTTON_SPACESIM_RAISE: u32 = 117457926;
pub const DIBUTTON_SPACESIM_RIGHT_LINK: u32 = 117490924;
pub const DIBUTTON_SPACESIM_SLOWER_LINK: u32 = 117687528;
pub const DIBUTTON_SPACESIM_TARGET: u32 = 117441539;
pub const DIBUTTON_SPACESIM_TURN_LEFT_LINK: u32 = 117589220;
pub const DIBUTTON_SPACESIM_TURN_RIGHT_LINK: u32 = 117589228;
pub const DIBUTTON_SPACESIM_VIEW: u32 = 117457924;
pub const DIBUTTON_SPACESIM_WEAPONS: u32 = 117441538;
pub const DIBUTTON_STRATEGYR_APPLY: u32 = 184550402;
pub const DIBUTTON_STRATEGYR_ATTACK: u32 = 184550404;
pub const DIBUTTON_STRATEGYR_BACK_LINK: u32 = 184632552;
pub const DIBUTTON_STRATEGYR_CAST: u32 = 184550405;
pub const DIBUTTON_STRATEGYR_CROUCH: u32 = 184550406;
pub const DIBUTTON_STRATEGYR_DEVICE: u32 = 184567038;
pub const DIBUTTON_STRATEGYR_DISPLAY: u32 = 184566793;
pub const DIBUTTON_STRATEGYR_FORWARD_LINK: u32 = 184632544;
pub const DIBUTTON_STRATEGYR_GET: u32 = 184550401;
pub const DIBUTTON_STRATEGYR_JUMP: u32 = 184550407;
pub const DIBUTTON_STRATEGYR_LEFT_LINK: u32 = 184599780;
pub const DIBUTTON_STRATEGYR_MAP: u32 = 184566792;
pub const DIBUTTON_STRATEGYR_MENU: u32 = 184550653;
pub const DIBUTTON_STRATEGYR_PAUSE: u32 = 184567036;
pub const DIBUTTON_STRATEGYR_RIGHT_LINK: u32 = 184599788;
pub const DIBUTTON_STRATEGYR_ROTATE_LEFT_LINK: u32 = 184698084;
pub const DIBUTTON_STRATEGYR_ROTATE_RIGHT_LINK: u32 = 184698092;
pub const DIBUTTON_STRATEGYR_SELECT: u32 = 184550403;
pub const DIBUTTON_STRATEGYT_APPLY: u32 = 201327619;
pub const DIBUTTON_STRATEGYT_BACK_LINK: u32 = 201409768;
pub const DIBUTTON_STRATEGYT_DEVICE: u32 = 201344254;
pub const DIBUTTON_STRATEGYT_DISPLAY: u32 = 201344008;
pub const DIBUTTON_STRATEGYT_FORWARD_LINK: u32 = 201409760;
pub const DIBUTTON_STRATEGYT_INSTRUCT: u32 = 201327618;
pub const DIBUTTON_STRATEGYT_LEFT_LINK: u32 = 201376996;
pub const DIBUTTON_STRATEGYT_MAP: u32 = 201344007;
pub const DIBUTTON_STRATEGYT_MENU: u32 = 201327869;
pub const DIBUTTON_STRATEGYT_PAUSE: u32 = 201344252;
pub const DIBUTTON_STRATEGYT_RIGHT_LINK: u32 = 201377004;
pub const DIBUTTON_STRATEGYT_SELECT: u32 = 201327617;
pub const DIBUTTON_STRATEGYT_TEAM: u32 = 201327620;
pub const DIBUTTON_STRATEGYT_TURN: u32 = 201327621;
pub const DIBUTTON_STRATEGYT_ZOOM: u32 = 201344006;
pub const DIBUTTON_TPS_ACTION: u32 = 167773186;
pub const DIBUTTON_TPS_BACKWARD_LINK: u32 = 167855336;
pub const DIBUTTON_TPS_DEVICE: u32 = 167789822;
pub const DIBUTTON_TPS_DODGE: u32 = 167789577;
pub const DIBUTTON_TPS_FORWARD_LINK: u32 = 167855328;
pub const DIBUTTON_TPS_GLANCE_DOWN_LINK: u32 = 168281320;
pub const DIBUTTON_TPS_GLANCE_LEFT_LINK: u32 = 168281316;
pub const DIBUTTON_TPS_GLANCE_RIGHT_LINK: u32 = 168281324;
pub const DIBUTTON_TPS_GLANCE_UP_LINK: u32 = 168281312;
pub const DIBUTTON_TPS_INVENTORY: u32 = 167789578;
pub const DIBUTTON_TPS_JUMP: u32 = 167773189;
pub const DIBUTTON_TPS_MENU: u32 = 167773437;
pub const DIBUTTON_TPS_PAUSE: u32 = 167789820;
pub const DIBUTTON_TPS_RUN: u32 = 167773185;
pub const DIBUTTON_TPS_SELECT: u32 = 167773187;
pub const DIBUTTON_TPS_STEPLEFT: u32 = 167789575;
pub const DIBUTTON_TPS_STEPRIGHT: u32 = 167789576;
pub const DIBUTTON_TPS_TURN_LEFT_LINK: u32 = 167920868;
pub const DIBUTTON_TPS_TURN_RIGHT_LINK: u32 = 167920876;
pub const DIBUTTON_TPS_USE: u32 = 167773188;
pub const DIBUTTON_TPS_VIEW: u32 = 167789574;
pub const DICD_DEFAULT: u32 = 0;
pub const DICD_EDIT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DICOLORSET {
    pub dwSize: u32,
    pub cTextFore: u32,
    pub cTextHighlight: u32,
    pub cCalloutLine: u32,
    pub cCalloutHighlight: u32,
    pub cBorder: u32,
    pub cControlFill: u32,
    pub cHighlightFill: u32,
    pub cAreaFill: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DICONDITION {
    pub lOffset: i32,
    pub lPositiveCoefficient: i32,
    pub lNegativeCoefficient: i32,
    pub dwPositiveSaturation: u32,
    pub dwNegativeSaturation: u32,
    pub lDeadBand: i32,
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct DICONFIGUREDEVICESPARAMSA {
    pub dwSize: u32,
    pub dwcUsers: u32,
    pub lptszUserNames: windows_sys::core::PSTR,
    pub dwcFormats: u32,
    pub lprgFormats: *mut DIACTIONFORMATA,
    pub hwnd: super::super::Foundation::HWND,
    pub dics: DICOLORSET,
    pub lpUnkDDSTarget: core::mem::ManuallyDrop<*mut core::ffi::c_void>,
}
impl Default for DICONFIGUREDEVICESPARAMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct DICONFIGUREDEVICESPARAMSW {
    pub dwSize: u32,
    pub dwcUsers: u32,
    pub lptszUserNames: windows_sys::core::PWSTR,
    pub dwcFormats: u32,
    pub lprgFormats: *mut DIACTIONFORMATW,
    pub hwnd: super::super::Foundation::HWND,
    pub dics: DICOLORSET,
    pub lpUnkDDSTarget: core::mem::ManuallyDrop<*mut core::ffi::c_void>,
}
impl Default for DICONFIGUREDEVICESPARAMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DICONSTANTFORCE {
    pub lMagnitude: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DICUSTOMFORCE {
    pub cChannels: u32,
    pub dwSamplePeriod: u32,
    pub cSamples: u32,
    pub rglForceData: *mut i32,
}
impl Default for DICUSTOMFORCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIDAL_BOTTOMALIGNED: u32 = 8;
pub const DIDAL_CENTERED: u32 = 0;
pub const DIDAL_LEFTALIGNED: u32 = 1;
pub const DIDAL_MIDDLE: u32 = 0;
pub const DIDAL_RIGHTALIGNED: u32 = 2;
pub const DIDAL_TOPALIGNED: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDATAFORMAT {
    pub dwSize: u32,
    pub dwObjSize: u32,
    pub dwFlags: u32,
    pub dwDataSize: u32,
    pub dwNumObjs: u32,
    pub rgodf: *mut DIOBJECTDATAFORMAT,
}
impl Default for DIDATAFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIDBAM_DEFAULT: u32 = 0;
pub const DIDBAM_HWDEFAULTS: u32 = 4;
pub const DIDBAM_INITIALIZE: u32 = 2;
pub const DIDBAM_PRESERVE: u32 = 1;
pub const DIDC_ALIAS: u32 = 65536;
pub const DIDC_ATTACHED: u32 = 1;
pub const DIDC_DEADBAND: u32 = 16384;
pub const DIDC_EMULATED: u32 = 4;
pub const DIDC_FFATTACK: u32 = 512;
pub const DIDC_FFFADE: u32 = 1024;
pub const DIDC_FORCEFEEDBACK: u32 = 256;
pub const DIDC_HIDDEN: u32 = 262144;
pub const DIDC_PHANTOM: u32 = 131072;
pub const DIDC_POLLEDDATAFORMAT: u32 = 8;
pub const DIDC_POLLEDDEVICE: u32 = 2;
pub const DIDC_POSNEGCOEFFICIENTS: u32 = 4096;
pub const DIDC_POSNEGSATURATION: u32 = 8192;
pub const DIDC_SATURATION: u32 = 2048;
pub const DIDC_STARTDELAY: u32 = 32768;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIDEVCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDevType: u32,
    pub dwAxes: u32,
    pub dwButtons: u32,
    pub dwPOVs: u32,
    pub dwFFSamplePeriod: u32,
    pub dwFFMinTimeResolution: u32,
    pub dwFirmwareRevision: u32,
    pub dwHardwareRevision: u32,
    pub dwFFDriverVersion: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIDEVCAPS_DX3 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDevType: u32,
    pub dwAxes: u32,
    pub dwButtons: u32,
    pub dwPOVs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEIMAGEINFOA {
    pub tszImagePath: [i8; 260],
    pub dwFlags: u32,
    pub dwViewID: u32,
    pub rcOverlay: super::super::Foundation::RECT,
    pub dwObjID: u32,
    pub dwcValidPts: u32,
    pub rgptCalloutLine: [super::super::Foundation::POINT; 5],
    pub rcCalloutRect: super::super::Foundation::RECT,
    pub dwTextAlign: u32,
}
impl Default for DIDEVICEIMAGEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEIMAGEINFOHEADERA {
    pub dwSize: u32,
    pub dwSizeImageInfo: u32,
    pub dwcViews: u32,
    pub dwcButtons: u32,
    pub dwcAxes: u32,
    pub dwcPOVs: u32,
    pub dwBufferSize: u32,
    pub dwBufferUsed: u32,
    pub lprgImageInfoArray: *mut DIDEVICEIMAGEINFOA,
}
impl Default for DIDEVICEIMAGEINFOHEADERA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEIMAGEINFOHEADERW {
    pub dwSize: u32,
    pub dwSizeImageInfo: u32,
    pub dwcViews: u32,
    pub dwcButtons: u32,
    pub dwcAxes: u32,
    pub dwcPOVs: u32,
    pub dwBufferSize: u32,
    pub dwBufferUsed: u32,
    pub lprgImageInfoArray: *mut DIDEVICEIMAGEINFOW,
}
impl Default for DIDEVICEIMAGEINFOHEADERW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEIMAGEINFOW {
    pub tszImagePath: [u16; 260],
    pub dwFlags: u32,
    pub dwViewID: u32,
    pub rcOverlay: super::super::Foundation::RECT,
    pub dwObjID: u32,
    pub dwcValidPts: u32,
    pub rgptCalloutLine: [super::super::Foundation::POINT; 5],
    pub rcCalloutRect: super::super::Foundation::RECT,
    pub dwTextAlign: u32,
}
impl Default for DIDEVICEIMAGEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEINSTANCEA {
    pub dwSize: u32,
    pub guidInstance: windows_sys::core::GUID,
    pub guidProduct: windows_sys::core::GUID,
    pub dwDevType: u32,
    pub tszInstanceName: [i8; 260],
    pub tszProductName: [i8; 260],
    pub guidFFDriver: windows_sys::core::GUID,
    pub wUsagePage: u16,
    pub wUsage: u16,
}
impl Default for DIDEVICEINSTANCEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEINSTANCEW {
    pub dwSize: u32,
    pub guidInstance: windows_sys::core::GUID,
    pub guidProduct: windows_sys::core::GUID,
    pub dwDevType: u32,
    pub tszInstanceName: [u16; 260],
    pub tszProductName: [u16; 260],
    pub guidFFDriver: windows_sys::core::GUID,
    pub wUsagePage: u16,
    pub wUsage: u16,
}
impl Default for DIDEVICEINSTANCEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEINSTANCE_DX3A {
    pub dwSize: u32,
    pub guidInstance: windows_sys::core::GUID,
    pub guidProduct: windows_sys::core::GUID,
    pub dwDevType: u32,
    pub tszInstanceName: [i8; 260],
    pub tszProductName: [i8; 260],
}
impl Default for DIDEVICEINSTANCE_DX3A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEINSTANCE_DX3W {
    pub dwSize: u32,
    pub guidInstance: windows_sys::core::GUID,
    pub guidProduct: windows_sys::core::GUID,
    pub dwDevType: u32,
    pub tszInstanceName: [u16; 260],
    pub tszProductName: [u16; 260],
}
impl Default for DIDEVICEINSTANCE_DX3W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIDEVICEOBJECTDATA {
    pub dwOfs: u32,
    pub dwData: u32,
    pub dwTimeStamp: u32,
    pub dwSequence: u32,
    pub uAppData: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIDEVICEOBJECTDATA_DX3 {
    pub dwOfs: u32,
    pub dwData: u32,
    pub dwTimeStamp: u32,
    pub dwSequence: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEOBJECTINSTANCEA {
    pub dwSize: u32,
    pub guidType: windows_sys::core::GUID,
    pub dwOfs: u32,
    pub dwType: u32,
    pub dwFlags: u32,
    pub tszName: [i8; 260],
    pub dwFFMaxForce: u32,
    pub dwFFForceResolution: u32,
    pub wCollectionNumber: u16,
    pub wDesignatorIndex: u16,
    pub wUsagePage: u16,
    pub wUsage: u16,
    pub dwDimension: u32,
    pub wExponent: u16,
    pub wReportId: u16,
}
impl Default for DIDEVICEOBJECTINSTANCEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEOBJECTINSTANCEW {
    pub dwSize: u32,
    pub guidType: windows_sys::core::GUID,
    pub dwOfs: u32,
    pub dwType: u32,
    pub dwFlags: u32,
    pub tszName: [u16; 260],
    pub dwFFMaxForce: u32,
    pub dwFFForceResolution: u32,
    pub wCollectionNumber: u16,
    pub wDesignatorIndex: u16,
    pub wUsagePage: u16,
    pub wUsage: u16,
    pub dwDimension: u32,
    pub wExponent: u16,
    pub wReportId: u16,
}
impl Default for DIDEVICEOBJECTINSTANCEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEOBJECTINSTANCE_DX3A {
    pub dwSize: u32,
    pub guidType: windows_sys::core::GUID,
    pub dwOfs: u32,
    pub dwType: u32,
    pub dwFlags: u32,
    pub tszName: [i8; 260],
}
impl Default for DIDEVICEOBJECTINSTANCE_DX3A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEOBJECTINSTANCE_DX3W {
    pub dwSize: u32,
    pub guidType: windows_sys::core::GUID,
    pub dwOfs: u32,
    pub dwType: u32,
    pub dwFlags: u32,
    pub tszName: [u16; 260],
}
impl Default for DIDEVICEOBJECTINSTANCE_DX3W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIDEVICESTATE {
    pub dwSize: u32,
    pub dwState: u32,
    pub dwLoad: u32,
}
pub const DIDEVTYPEJOYSTICK_FLIGHTSTICK: u32 = 3;
pub const DIDEVTYPEJOYSTICK_GAMEPAD: u32 = 4;
pub const DIDEVTYPEJOYSTICK_HEADTRACKER: u32 = 7;
pub const DIDEVTYPEJOYSTICK_RUDDER: u32 = 5;
pub const DIDEVTYPEJOYSTICK_TRADITIONAL: u32 = 2;
pub const DIDEVTYPEJOYSTICK_UNKNOWN: u32 = 1;
pub const DIDEVTYPEJOYSTICK_WHEEL: u32 = 6;
pub const DIDEVTYPEKEYBOARD_J3100: u32 = 12;
pub const DIDEVTYPEKEYBOARD_JAPAN106: u32 = 10;
pub const DIDEVTYPEKEYBOARD_JAPANAX: u32 = 11;
pub const DIDEVTYPEKEYBOARD_NEC98: u32 = 7;
pub const DIDEVTYPEKEYBOARD_NEC98106: u32 = 9;
pub const DIDEVTYPEKEYBOARD_NEC98LAPTOP: u32 = 8;
pub const DIDEVTYPEKEYBOARD_NOKIA1050: u32 = 5;
pub const DIDEVTYPEKEYBOARD_NOKIA9140: u32 = 6;
pub const DIDEVTYPEKEYBOARD_OLIVETTI: u32 = 2;
pub const DIDEVTYPEKEYBOARD_PCAT: u32 = 3;
pub const DIDEVTYPEKEYBOARD_PCENH: u32 = 4;
pub const DIDEVTYPEKEYBOARD_PCXT: u32 = 1;
pub const DIDEVTYPEKEYBOARD_UNKNOWN: u32 = 0;
pub const DIDEVTYPEMOUSE_FINGERSTICK: u32 = 3;
pub const DIDEVTYPEMOUSE_TOUCHPAD: u32 = 4;
pub const DIDEVTYPEMOUSE_TRACKBALL: u32 = 5;
pub const DIDEVTYPEMOUSE_TRADITIONAL: u32 = 2;
pub const DIDEVTYPEMOUSE_UNKNOWN: u32 = 1;
pub const DIDEVTYPE_DEVICE: u32 = 1;
pub const DIDEVTYPE_HID: u32 = 65536;
pub const DIDEVTYPE_JOYSTICK: u32 = 4;
pub const DIDEVTYPE_KEYBOARD: u32 = 3;
pub const DIDEVTYPE_MOUSE: u32 = 2;
pub const DIDFT_ABSAXIS: u32 = 2;
pub const DIDFT_ALIAS: u32 = 134217728;
pub const DIDFT_ALL: u32 = 0;
pub const DIDFT_ANYINSTANCE: u32 = 16776960;
pub const DIDFT_AXIS: u32 = 3;
pub const DIDFT_BUTTON: u32 = 12;
pub const DIDFT_COLLECTION: u32 = 64;
pub const DIDFT_FFACTUATOR: u32 = 16777216;
pub const DIDFT_FFEFFECTTRIGGER: u32 = 33554432;
pub const DIDFT_INSTANCEMASK: u32 = 16776960;
pub const DIDFT_NOCOLLECTION: u32 = 16776960;
pub const DIDFT_NODATA: u32 = 128;
pub const DIDFT_OUTPUT: u32 = 268435456;
pub const DIDFT_POV: u32 = 16;
pub const DIDFT_PSHBUTTON: u32 = 4;
pub const DIDFT_RELAXIS: u32 = 1;
pub const DIDFT_TGLBUTTON: u32 = 8;
pub const DIDFT_VENDORDEFINED: u32 = 67108864;
pub const DIDF_ABSAXIS: u32 = 1;
pub const DIDF_RELAXIS: u32 = 2;
pub const DIDIFT_CONFIGURATION: u32 = 1;
pub const DIDIFT_DELETE: u32 = 16777216;
pub const DIDIFT_OVERLAY: u32 = 2;
pub const DIDOI_ASPECTACCEL: u32 = 768;
pub const DIDOI_ASPECTFORCE: u32 = 1024;
pub const DIDOI_ASPECTMASK: u32 = 3840;
pub const DIDOI_ASPECTPOSITION: u32 = 256;
pub const DIDOI_ASPECTVELOCITY: u32 = 512;
pub const DIDOI_FFACTUATOR: u32 = 1;
pub const DIDOI_FFEFFECTTRIGGER: u32 = 2;
pub const DIDOI_GUIDISUSAGE: u32 = 65536;
pub const DIDOI_POLLED: u32 = 32768;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIDRIVERVERSIONS {
    pub dwSize: u32,
    pub dwFirmwareRevision: u32,
    pub dwHardwareRevision: u32,
    pub dwFFDriverVersion: u32,
}
pub const DIDSAM_DEFAULT: u32 = 0;
pub const DIDSAM_FORCESAVE: u32 = 2;
pub const DIDSAM_NOUSER: u32 = 1;
pub const DIEB_NOTRIGGER: u32 = 4294967295;
pub const DIEDBSFL_ATTACHEDONLY: u32 = 0;
pub const DIEDBSFL_AVAILABLEDEVICES: u32 = 4096;
pub const DIEDBSFL_FORCEFEEDBACK: u32 = 256;
pub const DIEDBSFL_MULTIMICEKEYBOARDS: u32 = 8192;
pub const DIEDBSFL_NONGAMINGDEVICES: u32 = 16384;
pub const DIEDBSFL_THISUSER: u32 = 16;
pub const DIEDBSFL_VALID: u32 = 28944;
pub const DIEDBS_MAPPEDPRI1: u32 = 1;
pub const DIEDBS_MAPPEDPRI2: u32 = 2;
pub const DIEDBS_NEWDEVICE: u32 = 32;
pub const DIEDBS_RECENTDEVICE: u32 = 16;
pub const DIEDFL_ALLDEVICES: u32 = 0;
pub const DIEDFL_ATTACHEDONLY: u32 = 1;
pub const DIEDFL_FORCEFEEDBACK: u32 = 256;
pub const DIEDFL_INCLUDEALIASES: u32 = 65536;
pub const DIEDFL_INCLUDEHIDDEN: u32 = 262144;
pub const DIEDFL_INCLUDEPHANTOMS: u32 = 131072;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIEFFECT {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDuration: u32,
    pub dwSamplePeriod: u32,
    pub dwGain: u32,
    pub dwTriggerButton: u32,
    pub dwTriggerRepeatInterval: u32,
    pub cAxes: u32,
    pub rgdwAxes: *mut u32,
    pub rglDirection: *mut i32,
    pub lpEnvelope: *mut DIENVELOPE,
    pub cbTypeSpecificParams: u32,
    pub lpvTypeSpecificParams: *mut core::ffi::c_void,
    pub dwStartDelay: u32,
}
impl Default for DIEFFECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIEFFECTATTRIBUTES {
    pub dwEffectId: u32,
    pub dwEffType: u32,
    pub dwStaticParams: u32,
    pub dwDynamicParams: u32,
    pub dwCoords: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIEFFECTINFOA {
    pub dwSize: u32,
    pub guid: windows_sys::core::GUID,
    pub dwEffType: u32,
    pub dwStaticParams: u32,
    pub dwDynamicParams: u32,
    pub tszName: [i8; 260],
}
impl Default for DIEFFECTINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIEFFECTINFOW {
    pub dwSize: u32,
    pub guid: windows_sys::core::GUID,
    pub dwEffType: u32,
    pub dwStaticParams: u32,
    pub dwDynamicParams: u32,
    pub tszName: [u16; 260],
}
impl Default for DIEFFECTINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIEFFECT_DX5 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDuration: u32,
    pub dwSamplePeriod: u32,
    pub dwGain: u32,
    pub dwTriggerButton: u32,
    pub dwTriggerRepeatInterval: u32,
    pub cAxes: u32,
    pub rgdwAxes: *mut u32,
    pub rglDirection: *mut i32,
    pub lpEnvelope: *mut DIENVELOPE,
    pub cbTypeSpecificParams: u32,
    pub lpvTypeSpecificParams: *mut core::ffi::c_void,
}
impl Default for DIEFFECT_DX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIEFFESCAPE {
    pub dwSize: u32,
    pub dwCommand: u32,
    pub lpvInBuffer: *mut core::ffi::c_void,
    pub cbInBuffer: u32,
    pub lpvOutBuffer: *mut core::ffi::c_void,
    pub cbOutBuffer: u32,
}
impl Default for DIEFFESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIEFF_CARTESIAN: u32 = 16;
pub const DIEFF_OBJECTIDS: u32 = 1;
pub const DIEFF_OBJECTOFFSETS: u32 = 2;
pub const DIEFF_POLAR: u32 = 32;
pub const DIEFF_SPHERICAL: u32 = 64;
pub const DIEFT_ALL: u32 = 0;
pub const DIEFT_CONDITION: u32 = 4;
pub const DIEFT_CONSTANTFORCE: u32 = 1;
pub const DIEFT_CUSTOMFORCE: u32 = 5;
pub const DIEFT_DEADBAND: u32 = 16384;
pub const DIEFT_FFATTACK: u32 = 512;
pub const DIEFT_FFFADE: u32 = 1024;
pub const DIEFT_HARDWARE: u32 = 255;
pub const DIEFT_PERIODIC: u32 = 3;
pub const DIEFT_POSNEGCOEFFICIENTS: u32 = 4096;
pub const DIEFT_POSNEGSATURATION: u32 = 8192;
pub const DIEFT_RAMPFORCE: u32 = 2;
pub const DIEFT_SATURATION: u32 = 2048;
pub const DIEFT_STARTDELAY: u32 = 32768;
pub const DIEGES_EMULATED: u32 = 2;
pub const DIEGES_PLAYING: u32 = 1;
pub const DIENUM_CONTINUE: u32 = 1;
pub const DIENUM_STOP: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIENVELOPE {
    pub dwSize: u32,
    pub dwAttackLevel: u32,
    pub dwAttackTime: u32,
    pub dwFadeLevel: u32,
    pub dwFadeTime: u32,
}
pub const DIEP_ALLPARAMS: u32 = 1023;
pub const DIEP_ALLPARAMS_DX5: u32 = 511;
pub const DIEP_AXES: u32 = 32;
pub const DIEP_DIRECTION: u32 = 64;
pub const DIEP_DURATION: u32 = 1;
pub const DIEP_ENVELOPE: u32 = 128;
pub const DIEP_GAIN: u32 = 4;
pub const DIEP_NODOWNLOAD: u32 = 2147483648;
pub const DIEP_NORESTART: u32 = 1073741824;
pub const DIEP_SAMPLEPERIOD: u32 = 2;
pub const DIEP_START: u32 = 536870912;
pub const DIEP_STARTDELAY: u32 = 512;
pub const DIEP_TRIGGERBUTTON: u32 = 8;
pub const DIEP_TRIGGERREPEATINTERVAL: u32 = 16;
pub const DIEP_TYPESPECIFICPARAMS: u32 = 256;
pub const DIERR_ACQUIRED: windows_sys::core::HRESULT = 0x800700AA_u32 as _;
pub const DIERR_ALREADYINITIALIZED: windows_sys::core::HRESULT = 0x800704DF_u32 as _;
pub const DIERR_BADDRIVERVER: windows_sys::core::HRESULT = 0x80070077_u32 as _;
pub const DIERR_BADINF: i32 = -2147220478;
pub const DIERR_BETADIRECTINPUTVERSION: windows_sys::core::HRESULT = 0x80070481_u32 as _;
pub const DIERR_CANCELLED: i32 = -2147220479;
pub const DIERR_DEVICEFULL: i32 = -2147220991;
pub const DIERR_DEVICENOTREG: i32 = -2147221164;
pub const DIERR_DRIVERFIRST: i32 = -2147220736;
pub const DIERR_DRIVERLAST: i32 = -2147220481;
pub const DIERR_EFFECTPLAYING: i32 = -2147220984;
pub const DIERR_GENERIC: i32 = -2147467259;
pub const DIERR_HANDLEEXISTS: i32 = -2147024891;
pub const DIERR_HASEFFECTS: i32 = -2147220988;
pub const DIERR_INCOMPLETEEFFECT: i32 = -2147220986;
pub const DIERR_INPUTLOST: windows_sys::core::HRESULT = 0x8007001E_u32 as _;
pub const DIERR_INSUFFICIENTPRIVS: i32 = -2147220992;
pub const DIERR_INVALIDCLASSINSTALLER: i32 = -2147220480;
pub const DIERR_INVALIDPARAM: i32 = -2147024809;
pub const DIERR_MAPFILEFAIL: i32 = -2147220981;
pub const DIERR_MOREDATA: i32 = -2147220990;
pub const DIERR_NOAGGREGATION: i32 = -2147221232;
pub const DIERR_NOINTERFACE: i32 = -2147467262;
pub const DIERR_NOMOREITEMS: windows_sys::core::HRESULT = 0x80070103_u32 as _;
pub const DIERR_NOTACQUIRED: windows_sys::core::HRESULT = 0x8007000C_u32 as _;
pub const DIERR_NOTBUFFERED: i32 = -2147220985;
pub const DIERR_NOTDOWNLOADED: i32 = -2147220989;
pub const DIERR_NOTEXCLUSIVEACQUIRED: i32 = -2147220987;
pub const DIERR_NOTFOUND: windows_sys::core::HRESULT = 0x80070002_u32 as _;
pub const DIERR_NOTINITIALIZED: windows_sys::core::HRESULT = 0x80070015_u32 as _;
pub const DIERR_OBJECTNOTFOUND: windows_sys::core::HRESULT = 0x80070002_u32 as _;
pub const DIERR_OLDDIRECTINPUTVERSION: windows_sys::core::HRESULT = 0x8007047E_u32 as _;
pub const DIERR_OTHERAPPHASPRIO: i32 = -2147024891;
pub const DIERR_OUTOFMEMORY: i32 = -2147024882;
pub const DIERR_READONLY: i32 = -2147024891;
pub const DIERR_REPORTFULL: i32 = -2147220982;
pub const DIERR_UNPLUGGED: i32 = -2147220983;
pub const DIERR_UNSUPPORTED: i32 = -2147467263;
pub const DIES_NODOWNLOAD: u32 = 2147483648;
pub const DIES_SOLO: u32 = 1;
pub const DIFEF_DEFAULT: u32 = 0;
pub const DIFEF_INCLUDENONSTANDARD: u32 = 1;
pub const DIFEF_MODIFYIFNEEDED: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIFFDEVICEATTRIBUTES {
    pub dwFlags: u32,
    pub dwFFSamplePeriod: u32,
    pub dwFFMinTimeResolution: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIFFOBJECTATTRIBUTES {
    pub dwFFMaxForce: u32,
    pub dwFFForceResolution: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIFILEEFFECT {
    pub dwSize: u32,
    pub GuidEffect: windows_sys::core::GUID,
    pub lpDiEffect: *mut DIEFFECT,
    pub szFriendlyName: [i8; 260],
}
impl Default for DIFILEEFFECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIGDD_PEEK: u32 = 1;
pub const DIGFFS_ACTUATORSOFF: u32 = 32;
pub const DIGFFS_ACTUATORSON: u32 = 16;
pub const DIGFFS_DEVICELOST: u32 = 2147483648;
pub const DIGFFS_EMPTY: u32 = 1;
pub const DIGFFS_PAUSED: u32 = 4;
pub const DIGFFS_POWEROFF: u32 = 128;
pub const DIGFFS_POWERON: u32 = 64;
pub const DIGFFS_SAFETYSWITCHOFF: u32 = 512;
pub const DIGFFS_SAFETYSWITCHON: u32 = 256;
pub const DIGFFS_STOPPED: u32 = 2;
pub const DIGFFS_USERFFSWITCHOFF: u32 = 2048;
pub const DIGFFS_USERFFSWITCHON: u32 = 1024;
pub const DIHATSWITCH_2DCONTROL_HATSWITCH: u32 = 587220481;
pub const DIHATSWITCH_3DCONTROL_HATSWITCH: u32 = 603997697;
pub const DIHATSWITCH_ARCADEP_VIEW: u32 = 570443265;
pub const DIHATSWITCH_ARCADES_VIEW: u32 = 553666049;
pub const DIHATSWITCH_BBALLD_GLANCE: u32 = 318785025;
pub const DIHATSWITCH_BBALLO_GLANCE: u32 = 302007809;
pub const DIHATSWITCH_BIKINGM_SCROLL: u32 = 469779969;
pub const DIHATSWITCH_CADF_HATSWITCH: u32 = 620774913;
pub const DIHATSWITCH_CADM_HATSWITCH: u32 = 637552129;
pub const DIHATSWITCH_DRIVINGC_GLANCE: u32 = 33572353;
pub const DIHATSWITCH_DRIVINGR_GLANCE: u32 = 16795137;
pub const DIHATSWITCH_DRIVINGT_GLANCE: u32 = 50349569;
pub const DIHATSWITCH_FIGHTINGH_SLIDE: u32 = 134235649;
pub const DIHATSWITCH_FISHING_GLANCE: u32 = 234898945;
pub const DIHATSWITCH_FLYINGC_GLANCE: u32 = 67126785;
pub const DIHATSWITCH_FLYINGH_GLANCE: u32 = 100681217;
pub const DIHATSWITCH_FLYINGM_GLANCE: u32 = 83904001;
pub const DIHATSWITCH_FPS_GLANCE: u32 = 151012865;
pub const DIHATSWITCH_GOLF_SCROLL: u32 = 402671105;
pub const DIHATSWITCH_HOCKEYD_SCROLL: u32 = 436225537;
pub const DIHATSWITCH_HOCKEYG_SCROLL: u32 = 453002753;
pub const DIHATSWITCH_HOCKEYO_SCROLL: u32 = 419448321;
pub const DIHATSWITCH_HUNTING_GLANCE: u32 = 218121729;
pub const DIHATSWITCH_MECHA_GLANCE: u32 = 687883777;
pub const DIHATSWITCH_RACQUET_GLANCE: u32 = 536888833;
pub const DIHATSWITCH_SKIING_GLANCE: u32 = 486557185;
pub const DIHATSWITCH_SOCCERD_GLANCE: u32 = 520111617;
pub const DIHATSWITCH_SOCCERO_GLANCE: u32 = 503334401;
pub const DIHATSWITCH_SPACESIM_GLANCE: u32 = 117458433;
pub const DIHATSWITCH_STRATEGYR_GLANCE: u32 = 184567297;
pub const DIHATSWITCH_TPS_GLANCE: u32 = 167790081;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIHIDFFINITINFO {
    pub dwSize: u32,
    pub pwszDeviceInterface: windows_sys::core::PWSTR,
    pub GuidInstance: windows_sys::core::GUID,
}
impl Default for DIHIDFFINITINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIJC_CALLOUT: u32 = 8;
pub const DIJC_GAIN: u32 = 4;
pub const DIJC_GUIDINSTANCE: u32 = 1;
pub const DIJC_REGHWCONFIGTYPE: u32 = 2;
pub const DIJC_WDMGAMEPORT: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIJOYCONFIG {
    pub dwSize: u32,
    pub guidInstance: windows_sys::core::GUID,
    pub hwc: JOYREGHWCONFIG,
    pub dwGain: u32,
    pub wszType: [u16; 256],
    pub wszCallout: [u16; 256],
    pub guidGameport: windows_sys::core::GUID,
}
impl Default for DIJOYCONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIJOYCONFIG_DX5 {
    pub dwSize: u32,
    pub guidInstance: windows_sys::core::GUID,
    pub hwc: JOYREGHWCONFIG,
    pub dwGain: u32,
    pub wszType: [u16; 256],
    pub wszCallout: [u16; 256],
}
impl Default for DIJOYCONFIG_DX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIJOYSTATE {
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub lRx: i32,
    pub lRy: i32,
    pub lRz: i32,
    pub rglSlider: [i32; 2],
    pub rgdwPOV: [u32; 4],
    pub rgbButtons: [u8; 32],
}
impl Default for DIJOYSTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIJOYSTATE2 {
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub lRx: i32,
    pub lRy: i32,
    pub lRz: i32,
    pub rglSlider: [i32; 2],
    pub rgdwPOV: [u32; 4],
    pub rgbButtons: [u8; 128],
    pub lVX: i32,
    pub lVY: i32,
    pub lVZ: i32,
    pub lVRx: i32,
    pub lVRy: i32,
    pub lVRz: i32,
    pub rglVSlider: [i32; 2],
    pub lAX: i32,
    pub lAY: i32,
    pub lAZ: i32,
    pub lARx: i32,
    pub lARy: i32,
    pub lARz: i32,
    pub rglASlider: [i32; 2],
    pub lFX: i32,
    pub lFY: i32,
    pub lFZ: i32,
    pub lFRx: i32,
    pub lFRy: i32,
    pub lFRz: i32,
    pub rglFSlider: [i32; 2],
}
impl Default for DIJOYSTATE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIJOYTYPEINFO {
    pub dwSize: u32,
    pub hws: JOYREGHWSETTINGS,
    pub clsidConfig: windows_sys::core::GUID,
    pub wszDisplayName: [u16; 256],
    pub wszCallout: [u16; 260],
    pub wszHardwareId: [u16; 256],
    pub dwFlags1: u32,
    pub dwFlags2: u32,
    pub wszMapFile: [u16; 256],
}
impl Default for DIJOYTYPEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIJOYTYPEINFO_DX5 {
    pub dwSize: u32,
    pub hws: JOYREGHWSETTINGS,
    pub clsidConfig: windows_sys::core::GUID,
    pub wszDisplayName: [u16; 256],
    pub wszCallout: [u16; 260],
}
impl Default for DIJOYTYPEINFO_DX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIJOYTYPEINFO_DX6 {
    pub dwSize: u32,
    pub hws: JOYREGHWSETTINGS,
    pub clsidConfig: windows_sys::core::GUID,
    pub wszDisplayName: [u16; 256],
    pub wszCallout: [u16; 260],
    pub wszHardwareId: [u16; 256],
    pub dwFlags1: u32,
}
impl Default for DIJOYTYPEINFO_DX6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIJOYUSERVALUES {
    pub dwSize: u32,
    pub ruv: JOYREGUSERVALUES,
    pub wszGlobalDriver: [u16; 256],
    pub wszGameportEmulator: [u16; 256],
}
impl Default for DIJOYUSERVALUES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIJU_GAMEPORTEMULATOR: u32 = 4;
pub const DIJU_GLOBALDRIVER: u32 = 2;
pub const DIJU_USERVALUES: u32 = 1;
pub const DIKEYBOARD_0: u32 = 2164261899;
pub const DIKEYBOARD_1: u32 = 2164261890;
pub const DIKEYBOARD_2: u32 = 2164261891;
pub const DIKEYBOARD_3: u32 = 2164261892;
pub const DIKEYBOARD_4: u32 = 2164261893;
pub const DIKEYBOARD_5: u32 = 2164261894;
pub const DIKEYBOARD_6: u32 = 2164261895;
pub const DIKEYBOARD_7: u32 = 2164261896;
pub const DIKEYBOARD_8: u32 = 2164261897;
pub const DIKEYBOARD_9: u32 = 2164261898;
pub const DIKEYBOARD_A: u32 = 2164261918;
pub const DIKEYBOARD_ABNT_C1: u32 = 2164262003;
pub const DIKEYBOARD_ABNT_C2: u32 = 2164262014;
pub const DIKEYBOARD_ADD: u32 = 2164261966;
pub const DIKEYBOARD_APOSTROPHE: u32 = 2164261928;
pub const DIKEYBOARD_APPS: u32 = 2164262109;
pub const DIKEYBOARD_AT: u32 = 2164262033;
pub const DIKEYBOARD_AX: u32 = 2164262038;
pub const DIKEYBOARD_B: u32 = 2164261936;
pub const DIKEYBOARD_BACK: u32 = 2164261902;
pub const DIKEYBOARD_BACKSLASH: u32 = 2164261931;
pub const DIKEYBOARD_C: u32 = 2164261934;
pub const DIKEYBOARD_CALCULATOR: u32 = 2164262049;
pub const DIKEYBOARD_CAPITAL: u32 = 2164261946;
pub const DIKEYBOARD_COLON: u32 = 2164262034;
pub const DIKEYBOARD_COMMA: u32 = 2164261939;
pub const DIKEYBOARD_CONVERT: u32 = 2164262009;
pub const DIKEYBOARD_D: u32 = 2164261920;
pub const DIKEYBOARD_DECIMAL: u32 = 2164261971;
pub const DIKEYBOARD_DELETE: u32 = 2164262099;
pub const DIKEYBOARD_DIVIDE: u32 = 2164262069;
pub const DIKEYBOARD_DOWN: u32 = 2164262096;
pub const DIKEYBOARD_E: u32 = 2164261906;
pub const DIKEYBOARD_END: u32 = 2164262095;
pub const DIKEYBOARD_EQUALS: u32 = 2164261901;
pub const DIKEYBOARD_ESCAPE: u32 = 2164261889;
pub const DIKEYBOARD_F: u32 = 2164261921;
pub const DIKEYBOARD_F1: u32 = 2164261947;
pub const DIKEYBOARD_F10: u32 = 2164261956;
pub const DIKEYBOARD_F11: u32 = 2164261975;
pub const DIKEYBOARD_F12: u32 = 2164261976;
pub const DIKEYBOARD_F13: u32 = 2164261988;
pub const DIKEYBOARD_F14: u32 = 2164261989;
pub const DIKEYBOARD_F15: u32 = 2164261990;
pub const DIKEYBOARD_F2: u32 = 2164261948;
pub const DIKEYBOARD_F3: u32 = 2164261949;
pub const DIKEYBOARD_F4: u32 = 2164261950;
pub const DIKEYBOARD_F5: u32 = 2164261951;
pub const DIKEYBOARD_F6: u32 = 2164261952;
pub const DIKEYBOARD_F7: u32 = 2164261953;
pub const DIKEYBOARD_F8: u32 = 2164261954;
pub const DIKEYBOARD_F9: u32 = 2164261955;
pub const DIKEYBOARD_G: u32 = 2164261922;
pub const DIKEYBOARD_GRAVE: u32 = 2164261929;
pub const DIKEYBOARD_H: u32 = 2164261923;
pub const DIKEYBOARD_HOME: u32 = 2164262087;
pub const DIKEYBOARD_I: u32 = 2164261911;
pub const DIKEYBOARD_INSERT: u32 = 2164262098;
pub const DIKEYBOARD_J: u32 = 2164261924;
pub const DIKEYBOARD_K: u32 = 2164261925;
pub const DIKEYBOARD_KANA: u32 = 2164262000;
pub const DIKEYBOARD_KANJI: u32 = 2164262036;
pub const DIKEYBOARD_L: u32 = 2164261926;
pub const DIKEYBOARD_LBRACKET: u32 = 2164261914;
pub const DIKEYBOARD_LCONTROL: u32 = 2164261917;
pub const DIKEYBOARD_LEFT: u32 = 2164262091;
pub const DIKEYBOARD_LMENU: u32 = 2164261944;
pub const DIKEYBOARD_LSHIFT: u32 = 2164261930;
pub const DIKEYBOARD_LWIN: u32 = 2164262107;
pub const DIKEYBOARD_M: u32 = 2164261938;
pub const DIKEYBOARD_MAIL: u32 = 2164262124;
pub const DIKEYBOARD_MEDIASELECT: u32 = 2164262125;
pub const DIKEYBOARD_MEDIASTOP: u32 = 2164262052;
pub const DIKEYBOARD_MINUS: u32 = 2164261900;
pub const DIKEYBOARD_MULTIPLY: u32 = 2164261943;
pub const DIKEYBOARD_MUTE: u32 = 2164262048;
pub const DIKEYBOARD_MYCOMPUTER: u32 = 2164262123;
pub const DIKEYBOARD_N: u32 = 2164261937;
pub const DIKEYBOARD_NEXT: u32 = 2164262097;
pub const DIKEYBOARD_NEXTTRACK: u32 = 2164262041;
pub const DIKEYBOARD_NOCONVERT: u32 = 2164262011;
pub const DIKEYBOARD_NUMLOCK: u32 = 2164261957;
pub const DIKEYBOARD_NUMPAD0: u32 = 2164261970;
pub const DIKEYBOARD_NUMPAD1: u32 = 2164261967;
pub const DIKEYBOARD_NUMPAD2: u32 = 2164261968;
pub const DIKEYBOARD_NUMPAD3: u32 = 2164261969;
pub const DIKEYBOARD_NUMPAD4: u32 = 2164261963;
pub const DIKEYBOARD_NUMPAD5: u32 = 2164261964;
pub const DIKEYBOARD_NUMPAD6: u32 = 2164261965;
pub const DIKEYBOARD_NUMPAD7: u32 = 2164261959;
pub const DIKEYBOARD_NUMPAD8: u32 = 2164261960;
pub const DIKEYBOARD_NUMPAD9: u32 = 2164261961;
pub const DIKEYBOARD_NUMPADCOMMA: u32 = 2164262067;
pub const DIKEYBOARD_NUMPADENTER: u32 = 2164262044;
pub const DIKEYBOARD_NUMPADEQUALS: u32 = 2164262029;
pub const DIKEYBOARD_O: u32 = 2164261912;
pub const DIKEYBOARD_OEM_102: u32 = 2164261974;
pub const DIKEYBOARD_P: u32 = 2164261913;
pub const DIKEYBOARD_PAUSE: u32 = 2164262085;
pub const DIKEYBOARD_PERIOD: u32 = 2164261940;
pub const DIKEYBOARD_PLAYPAUSE: u32 = 2164262050;
pub const DIKEYBOARD_POWER: u32 = 2164262110;
pub const DIKEYBOARD_PREVTRACK: u32 = 2164262032;
pub const DIKEYBOARD_PRIOR: u32 = 2164262089;
pub const DIKEYBOARD_Q: u32 = 2164261904;
pub const DIKEYBOARD_R: u32 = 2164261907;
pub const DIKEYBOARD_RBRACKET: u32 = 2164261915;
pub const DIKEYBOARD_RCONTROL: u32 = 2164262045;
pub const DIKEYBOARD_RETURN: u32 = 2164261916;
pub const DIKEYBOARD_RIGHT: u32 = 2164262093;
pub const DIKEYBOARD_RMENU: u32 = 2164262072;
pub const DIKEYBOARD_RSHIFT: u32 = 2164261942;
pub const DIKEYBOARD_RWIN: u32 = 2164262108;
pub const DIKEYBOARD_S: u32 = 2164261919;
pub const DIKEYBOARD_SCROLL: u32 = 2164261958;
pub const DIKEYBOARD_SEMICOLON: u32 = 2164261927;
pub const DIKEYBOARD_SLASH: u32 = 2164261941;
pub const DIKEYBOARD_SLEEP: u32 = 2164262111;
pub const DIKEYBOARD_SPACE: u32 = 2164261945;
pub const DIKEYBOARD_STOP: u32 = 2164262037;
pub const DIKEYBOARD_SUBTRACT: u32 = 2164261962;
pub const DIKEYBOARD_SYSRQ: u32 = 2164262071;
pub const DIKEYBOARD_T: u32 = 2164261908;
pub const DIKEYBOARD_TAB: u32 = 2164261903;
pub const DIKEYBOARD_U: u32 = 2164261910;
pub const DIKEYBOARD_UNDERLINE: u32 = 2164262035;
pub const DIKEYBOARD_UNLABELED: u32 = 2164262039;
pub const DIKEYBOARD_UP: u32 = 2164262088;
pub const DIKEYBOARD_V: u32 = 2164261935;
pub const DIKEYBOARD_VOLUMEDOWN: u32 = 2164262062;
pub const DIKEYBOARD_VOLUMEUP: u32 = 2164262064;
pub const DIKEYBOARD_W: u32 = 2164261905;
pub const DIKEYBOARD_WAKE: u32 = 2164262115;
pub const DIKEYBOARD_WEBBACK: u32 = 2164262122;
pub const DIKEYBOARD_WEBFAVORITES: u32 = 2164262118;
pub const DIKEYBOARD_WEBFORWARD: u32 = 2164262121;
pub const DIKEYBOARD_WEBHOME: u32 = 2164262066;
pub const DIKEYBOARD_WEBREFRESH: u32 = 2164262119;
pub const DIKEYBOARD_WEBSEARCH: u32 = 2164262117;
pub const DIKEYBOARD_WEBSTOP: u32 = 2164262120;
pub const DIKEYBOARD_X: u32 = 2164261933;
pub const DIKEYBOARD_Y: u32 = 2164261909;
pub const DIKEYBOARD_YEN: u32 = 2164262013;
pub const DIKEYBOARD_Z: u32 = 2164261932;
pub const DIK_0: u32 = 11;
pub const DIK_1: u32 = 2;
pub const DIK_2: u32 = 3;
pub const DIK_3: u32 = 4;
pub const DIK_4: u32 = 5;
pub const DIK_5: u32 = 6;
pub const DIK_6: u32 = 7;
pub const DIK_7: u32 = 8;
pub const DIK_8: u32 = 9;
pub const DIK_9: u32 = 10;
pub const DIK_A: u32 = 30;
pub const DIK_ABNT_C1: u32 = 115;
pub const DIK_ABNT_C2: u32 = 126;
pub const DIK_ADD: u32 = 78;
pub const DIK_APOSTROPHE: u32 = 40;
pub const DIK_APPS: u32 = 221;
pub const DIK_AT: u32 = 145;
pub const DIK_AX: u32 = 150;
pub const DIK_B: u32 = 48;
pub const DIK_BACK: u32 = 14;
pub const DIK_BACKSLASH: u32 = 43;
pub const DIK_BACKSPACE: u32 = 14;
pub const DIK_C: u32 = 46;
pub const DIK_CALCULATOR: u32 = 161;
pub const DIK_CAPITAL: u32 = 58;
pub const DIK_CAPSLOCK: u32 = 58;
pub const DIK_CIRCUMFLEX: u32 = 144;
pub const DIK_COLON: u32 = 146;
pub const DIK_COMMA: u32 = 51;
pub const DIK_CONVERT: u32 = 121;
pub const DIK_D: u32 = 32;
pub const DIK_DECIMAL: u32 = 83;
pub const DIK_DELETE: u32 = 211;
pub const DIK_DIVIDE: u32 = 181;
pub const DIK_DOWN: u32 = 208;
pub const DIK_DOWNARROW: u32 = 208;
pub const DIK_E: u32 = 18;
pub const DIK_END: u32 = 207;
pub const DIK_EQUALS: u32 = 13;
pub const DIK_ESCAPE: u32 = 1;
pub const DIK_F: u32 = 33;
pub const DIK_F1: u32 = 59;
pub const DIK_F10: u32 = 68;
pub const DIK_F11: u32 = 87;
pub const DIK_F12: u32 = 88;
pub const DIK_F13: u32 = 100;
pub const DIK_F14: u32 = 101;
pub const DIK_F15: u32 = 102;
pub const DIK_F2: u32 = 60;
pub const DIK_F3: u32 = 61;
pub const DIK_F4: u32 = 62;
pub const DIK_F5: u32 = 63;
pub const DIK_F6: u32 = 64;
pub const DIK_F7: u32 = 65;
pub const DIK_F8: u32 = 66;
pub const DIK_F9: u32 = 67;
pub const DIK_G: u32 = 34;
pub const DIK_GRAVE: u32 = 41;
pub const DIK_H: u32 = 35;
pub const DIK_HOME: u32 = 199;
pub const DIK_I: u32 = 23;
pub const DIK_INSERT: u32 = 210;
pub const DIK_J: u32 = 36;
pub const DIK_K: u32 = 37;
pub const DIK_KANA: u32 = 112;
pub const DIK_KANJI: u32 = 148;
pub const DIK_L: u32 = 38;
pub const DIK_LALT: u32 = 56;
pub const DIK_LBRACKET: u32 = 26;
pub const DIK_LCONTROL: u32 = 29;
pub const DIK_LEFT: u32 = 203;
pub const DIK_LEFTARROW: u32 = 203;
pub const DIK_LMENU: u32 = 56;
pub const DIK_LSHIFT: u32 = 42;
pub const DIK_LWIN: u32 = 219;
pub const DIK_M: u32 = 50;
pub const DIK_MAIL: u32 = 236;
pub const DIK_MEDIASELECT: u32 = 237;
pub const DIK_MEDIASTOP: u32 = 164;
pub const DIK_MINUS: u32 = 12;
pub const DIK_MULTIPLY: u32 = 55;
pub const DIK_MUTE: u32 = 160;
pub const DIK_MYCOMPUTER: u32 = 235;
pub const DIK_N: u32 = 49;
pub const DIK_NEXT: u32 = 209;
pub const DIK_NEXTTRACK: u32 = 153;
pub const DIK_NOCONVERT: u32 = 123;
pub const DIK_NUMLOCK: u32 = 69;
pub const DIK_NUMPAD0: u32 = 82;
pub const DIK_NUMPAD1: u32 = 79;
pub const DIK_NUMPAD2: u32 = 80;
pub const DIK_NUMPAD3: u32 = 81;
pub const DIK_NUMPAD4: u32 = 75;
pub const DIK_NUMPAD5: u32 = 76;
pub const DIK_NUMPAD6: u32 = 77;
pub const DIK_NUMPAD7: u32 = 71;
pub const DIK_NUMPAD8: u32 = 72;
pub const DIK_NUMPAD9: u32 = 73;
pub const DIK_NUMPADCOMMA: u32 = 179;
pub const DIK_NUMPADENTER: u32 = 156;
pub const DIK_NUMPADEQUALS: u32 = 141;
pub const DIK_NUMPADMINUS: u32 = 74;
pub const DIK_NUMPADPERIOD: u32 = 83;
pub const DIK_NUMPADPLUS: u32 = 78;
pub const DIK_NUMPADSLASH: u32 = 181;
pub const DIK_NUMPADSTAR: u32 = 55;
pub const DIK_O: u32 = 24;
pub const DIK_OEM_102: u32 = 86;
pub const DIK_P: u32 = 25;
pub const DIK_PAUSE: u32 = 197;
pub const DIK_PERIOD: u32 = 52;
pub const DIK_PGDN: u32 = 209;
pub const DIK_PGUP: u32 = 201;
pub const DIK_PLAYPAUSE: u32 = 162;
pub const DIK_POWER: u32 = 222;
pub const DIK_PREVTRACK: u32 = 144;
pub const DIK_PRIOR: u32 = 201;
pub const DIK_Q: u32 = 16;
pub const DIK_R: u32 = 19;
pub const DIK_RALT: u32 = 184;
pub const DIK_RBRACKET: u32 = 27;
pub const DIK_RCONTROL: u32 = 157;
pub const DIK_RETURN: u32 = 28;
pub const DIK_RIGHT: u32 = 205;
pub const DIK_RIGHTARROW: u32 = 205;
pub const DIK_RMENU: u32 = 184;
pub const DIK_RSHIFT: u32 = 54;
pub const DIK_RWIN: u32 = 220;
pub const DIK_S: u32 = 31;
pub const DIK_SCROLL: u32 = 70;
pub const DIK_SEMICOLON: u32 = 39;
pub const DIK_SLASH: u32 = 53;
pub const DIK_SLEEP: u32 = 223;
pub const DIK_SPACE: u32 = 57;
pub const DIK_STOP: u32 = 149;
pub const DIK_SUBTRACT: u32 = 74;
pub const DIK_SYSRQ: u32 = 183;
pub const DIK_T: u32 = 20;
pub const DIK_TAB: u32 = 15;
pub const DIK_U: u32 = 22;
pub const DIK_UNDERLINE: u32 = 147;
pub const DIK_UNLABELED: u32 = 151;
pub const DIK_UP: u32 = 200;
pub const DIK_UPARROW: u32 = 200;
pub const DIK_V: u32 = 47;
pub const DIK_VOLUMEDOWN: u32 = 174;
pub const DIK_VOLUMEUP: u32 = 176;
pub const DIK_W: u32 = 17;
pub const DIK_WAKE: u32 = 227;
pub const DIK_WEBBACK: u32 = 234;
pub const DIK_WEBFAVORITES: u32 = 230;
pub const DIK_WEBFORWARD: u32 = 233;
pub const DIK_WEBHOME: u32 = 178;
pub const DIK_WEBREFRESH: u32 = 231;
pub const DIK_WEBSEARCH: u32 = 229;
pub const DIK_WEBSTOP: u32 = 232;
pub const DIK_X: u32 = 45;
pub const DIK_Y: u32 = 21;
pub const DIK_YEN: u32 = 125;
pub const DIK_Z: u32 = 44;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIMOUSESTATE {
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub rgbButtons: [u8; 4],
}
impl Default for DIMOUSESTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIMOUSESTATE2 {
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub rgbButtons: [u8; 8],
}
impl Default for DIMOUSESTATE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIMSGWP_DX8APPSTART: u32 = 2;
pub const DIMSGWP_DX8MAPPERAPPSTART: u32 = 3;
pub const DIMSGWP_NEWAPPSTART: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIOBJECTATTRIBUTES {
    pub dwFlags: u32,
    pub wUsagePage: u16,
    pub wUsage: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIOBJECTCALIBRATION {
    pub lMin: i32,
    pub lCenter: i32,
    pub lMax: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIOBJECTDATAFORMAT {
    pub pguid: *const windows_sys::core::GUID,
    pub dwOfs: u32,
    pub dwType: u32,
    pub dwFlags: u32,
}
impl Default for DIOBJECTDATAFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIPERIODIC {
    pub dwMagnitude: u32,
    pub lOffset: i32,
    pub dwPhase: u32,
    pub dwPeriod: u32,
}
pub const DIPH_BYID: u32 = 2;
pub const DIPH_BYOFFSET: u32 = 1;
pub const DIPH_BYUSAGE: u32 = 3;
pub const DIPH_DEVICE: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIPOVCALIBRATION {
    pub lMin: [i32; 5],
    pub lMax: [i32; 5],
}
impl Default for DIPOVCALIBRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIPOV_ANY_1: u32 = 4278208001;
pub const DIPOV_ANY_2: u32 = 4278208002;
pub const DIPOV_ANY_3: u32 = 4278208003;
pub const DIPOV_ANY_4: u32 = 4278208004;
pub const DIPROPAUTOCENTER_OFF: u32 = 0;
pub const DIPROPAUTOCENTER_ON: u32 = 1;
pub const DIPROPAXISMODE_ABS: u32 = 0;
pub const DIPROPAXISMODE_REL: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIPROPCAL {
    pub diph: DIPROPHEADER,
    pub lMin: i32,
    pub lCenter: i32,
    pub lMax: i32,
}
pub const DIPROPCALIBRATIONMODE_COOKED: u32 = 0;
pub const DIPROPCALIBRATIONMODE_RAW: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIPROPCALPOV {
    pub diph: DIPROPHEADER,
    pub lMin: [i32; 5],
    pub lMax: [i32; 5],
}
impl Default for DIPROPCALPOV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIPROPCPOINTS {
    pub diph: DIPROPHEADER,
    pub dwCPointsNum: u32,
    pub cp: [CPOINT; 8],
}
impl Default for DIPROPCPOINTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIPROPDWORD {
    pub diph: DIPROPHEADER,
    pub dwData: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIPROPGUIDANDPATH {
    pub diph: DIPROPHEADER,
    pub guidClass: windows_sys::core::GUID,
    pub wszPath: [u16; 260],
}
impl Default for DIPROPGUIDANDPATH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIPROPHEADER {
    pub dwSize: u32,
    pub dwHeaderSize: u32,
    pub dwObj: u32,
    pub dwHow: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIPROPPOINTER {
    pub diph: DIPROPHEADER,
    pub uData: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIPROPRANGE {
    pub diph: DIPROPHEADER,
    pub lMin: i32,
    pub lMax: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIPROPSTRING {
    pub diph: DIPROPHEADER,
    pub wsz: [u16; 260],
}
impl Default for DIPROPSTRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIPROP_APPDATA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000016);
pub const DIPROP_AUTOCENTER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000009);
pub const DIPROP_AXISMODE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000002);
pub const DIPROP_BUFFERSIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000001);
pub const DIPROP_CALIBRATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000b);
pub const DIPROP_CALIBRATIONMODE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000a);
pub const DIPROP_CPOINTS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000015);
pub const DIPROP_DEADZONE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000005);
pub const DIPROP_FFGAIN: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000007);
pub const DIPROP_FFLOAD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000008);
pub const DIPROP_GETPORTDISPLAYNAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000010);
pub const DIPROP_GRANULARITY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000003);
pub const DIPROP_GUIDANDPATH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000c);
pub const DIPROP_INSTANCENAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000d);
pub const DIPROP_JOYSTICKID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000f);
pub const DIPROP_KEYNAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000014);
pub const DIPROP_LOGICALRANGE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000013);
pub const DIPROP_PHYSICALRANGE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000012);
pub const DIPROP_PRODUCTNAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000e);
pub const DIPROP_RANGE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000004);
pub const DIPROP_SATURATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000006);
pub const DIPROP_SCANCODE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000017);
pub const DIPROP_TYPENAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_00000000001a);
pub const DIPROP_USERNAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000019);
pub const DIPROP_VIDPID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000018);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIRAMPFORCE {
    pub lStart: i32,
    pub lEnd: i32,
}
pub const DIRECTINPUT_HEADER_VERSION: u32 = 2048;
pub const DIRECTINPUT_NOTIFICATION_MSGSTRING: windows_sys::core::PCWSTR = windows_sys::core::w!("DIRECTINPUT_NOTIFICATION_MSGSTRING");
pub const DIRECTINPUT_NOTIFICATION_MSGSTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("DIRECTINPUT_NOTIFICATION_MSGSTRING");
pub const DIRECTINPUT_NOTIFICATION_MSGSTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("DIRECTINPUT_NOTIFICATION_MSGSTRING");
pub const DIRECTINPUT_REGSTR_KEY_LASTAPP: windows_sys::core::PCWSTR = windows_sys::core::w!("MostRecentApplication");
pub const DIRECTINPUT_REGSTR_KEY_LASTAPPA: windows_sys::core::PCSTR = windows_sys::core::s!("MostRecentApplication");
pub const DIRECTINPUT_REGSTR_KEY_LASTAPPW: windows_sys::core::PCWSTR = windows_sys::core::w!("MostRecentApplication");
pub const DIRECTINPUT_REGSTR_KEY_LASTMAPAPP: windows_sys::core::PCWSTR = windows_sys::core::w!("MostRecentMapperApplication");
pub const DIRECTINPUT_REGSTR_KEY_LASTMAPAPPA: windows_sys::core::PCSTR = windows_sys::core::s!("MostRecentMapperApplication");
pub const DIRECTINPUT_REGSTR_KEY_LASTMAPAPPW: windows_sys::core::PCWSTR = windows_sys::core::w!("MostRecentMapperApplication");
pub const DIRECTINPUT_REGSTR_VAL_APPIDFLAG: windows_sys::core::PCWSTR = windows_sys::core::w!("AppIdFlag");
pub const DIRECTINPUT_REGSTR_VAL_APPIDFLAGA: windows_sys::core::PCSTR = windows_sys::core::s!("AppIdFlag");
pub const DIRECTINPUT_REGSTR_VAL_APPIDFLAGW: windows_sys::core::PCWSTR = windows_sys::core::w!("AppIdFlag");
pub const DIRECTINPUT_REGSTR_VAL_ID: windows_sys::core::PCWSTR = windows_sys::core::w!("Id");
pub const DIRECTINPUT_REGSTR_VAL_IDA: windows_sys::core::PCSTR = windows_sys::core::s!("Id");
pub const DIRECTINPUT_REGSTR_VAL_IDW: windows_sys::core::PCWSTR = windows_sys::core::w!("Id");
pub const DIRECTINPUT_REGSTR_VAL_LASTSTART: windows_sys::core::PCWSTR = windows_sys::core::w!("MostRecentStart");
pub const DIRECTINPUT_REGSTR_VAL_LASTSTARTA: windows_sys::core::PCSTR = windows_sys::core::s!("MostRecentStart");
pub const DIRECTINPUT_REGSTR_VAL_LASTSTARTW: windows_sys::core::PCWSTR = windows_sys::core::w!("MostRecentStart");
pub const DIRECTINPUT_REGSTR_VAL_MAPPER: windows_sys::core::PCWSTR = windows_sys::core::w!("UsesMapper");
pub const DIRECTINPUT_REGSTR_VAL_MAPPERA: windows_sys::core::PCSTR = windows_sys::core::s!("UsesMapper");
pub const DIRECTINPUT_REGSTR_VAL_MAPPERW: windows_sys::core::PCWSTR = windows_sys::core::w!("UsesMapper");
pub const DIRECTINPUT_REGSTR_VAL_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("Name");
pub const DIRECTINPUT_REGSTR_VAL_NAMEA: windows_sys::core::PCSTR = windows_sys::core::s!("Name");
pub const DIRECTINPUT_REGSTR_VAL_NAMEW: windows_sys::core::PCWSTR = windows_sys::core::w!("Name");
pub const DIRECTINPUT_REGSTR_VAL_VERSION: windows_sys::core::PCWSTR = windows_sys::core::w!("Version");
pub const DIRECTINPUT_REGSTR_VAL_VERSIONA: windows_sys::core::PCSTR = windows_sys::core::s!("Version");
pub const DIRECTINPUT_REGSTR_VAL_VERSIONW: windows_sys::core::PCWSTR = windows_sys::core::w!("Version");
pub const DIRECTINPUT_VERSION: u32 = 2048;
pub const DISCL_BACKGROUND: u32 = 8;
pub const DISCL_EXCLUSIVE: u32 = 1;
pub const DISCL_FOREGROUND: u32 = 4;
pub const DISCL_NONEXCLUSIVE: u32 = 2;
pub const DISCL_NOWINKEY: u32 = 16;
pub const DISDD_CONTINUE: u32 = 1;
pub const DISFFC_CONTINUE: u32 = 8;
pub const DISFFC_PAUSE: u32 = 4;
pub const DISFFC_RESET: u32 = 1;
pub const DISFFC_SETACTUATORSOFF: u32 = 32;
pub const DISFFC_SETACTUATORSON: u32 = 16;
pub const DISFFC_STOPALL: u32 = 2;
pub const DITC_CALLOUT: u32 = 8;
pub const DITC_CLSIDCONFIG: u32 = 2;
pub const DITC_DISPLAYNAME: u32 = 4;
pub const DITC_FLAGS1: u32 = 32;
pub const DITC_FLAGS2: u32 = 64;
pub const DITC_HARDWAREID: u32 = 16;
pub const DITC_MAPFILE: u32 = 128;
pub const DITC_REGHWSETTINGS: u32 = 1;
pub const DIVIRTUAL_ARCADE_PLATFORM: u32 = 570425344;
pub const DIVIRTUAL_ARCADE_SIDE2SIDE: u32 = 553648128;
pub const DIVIRTUAL_BROWSER_CONTROL: u32 = 671088640;
pub const DIVIRTUAL_CAD_2DCONTROL: u32 = 587202560;
pub const DIVIRTUAL_CAD_3DCONTROL: u32 = 603979776;
pub const DIVIRTUAL_CAD_FLYBY: u32 = 620756992;
pub const DIVIRTUAL_CAD_MODEL: u32 = 637534208;
pub const DIVIRTUAL_DRIVING_COMBAT: u32 = 33554432;
pub const DIVIRTUAL_DRIVING_MECHA: u32 = 687865856;
pub const DIVIRTUAL_DRIVING_RACE: u32 = 16777216;
pub const DIVIRTUAL_DRIVING_TANK: u32 = 50331648;
pub const DIVIRTUAL_FIGHTING_FPS: u32 = 150994944;
pub const DIVIRTUAL_FIGHTING_HAND2HAND: u32 = 134217728;
pub const DIVIRTUAL_FIGHTING_THIRDPERSON: u32 = 167772160;
pub const DIVIRTUAL_FLYING_CIVILIAN: u32 = 67108864;
pub const DIVIRTUAL_FLYING_HELICOPTER: u32 = 100663296;
pub const DIVIRTUAL_FLYING_MILITARY: u32 = 83886080;
pub const DIVIRTUAL_REMOTE_CONTROL: u32 = 654311424;
pub const DIVIRTUAL_SPACESIM: u32 = 117440512;
pub const DIVIRTUAL_SPORTS_BASEBALL_BAT: u32 = 251658240;
pub const DIVIRTUAL_SPORTS_BASEBALL_FIELD: u32 = 285212672;
pub const DIVIRTUAL_SPORTS_BASEBALL_PITCH: u32 = 268435456;
pub const DIVIRTUAL_SPORTS_BASKETBALL_DEFENSE: u32 = 318767104;
pub const DIVIRTUAL_SPORTS_BASKETBALL_OFFENSE: u32 = 301989888;
pub const DIVIRTUAL_SPORTS_BIKING_MOUNTAIN: u32 = 469762048;
pub const DIVIRTUAL_SPORTS_FISHING: u32 = 234881024;
pub const DIVIRTUAL_SPORTS_FOOTBALL_DEFENSE: u32 = 385875968;
pub const DIVIRTUAL_SPORTS_FOOTBALL_FIELD: u32 = 335544320;
pub const DIVIRTUAL_SPORTS_FOOTBALL_OFFENSE: u32 = 369098752;
pub const DIVIRTUAL_SPORTS_FOOTBALL_QBCK: u32 = 352321536;
pub const DIVIRTUAL_SPORTS_GOLF: u32 = 402653184;
pub const DIVIRTUAL_SPORTS_HOCKEY_DEFENSE: u32 = 436207616;
pub const DIVIRTUAL_SPORTS_HOCKEY_GOALIE: u32 = 452984832;
pub const DIVIRTUAL_SPORTS_HOCKEY_OFFENSE: u32 = 419430400;
pub const DIVIRTUAL_SPORTS_HUNTING: u32 = 218103808;
pub const DIVIRTUAL_SPORTS_RACQUET: u32 = 536870912;
pub const DIVIRTUAL_SPORTS_SKIING: u32 = 486539264;
pub const DIVIRTUAL_SPORTS_SOCCER_DEFENSE: u32 = 520093696;
pub const DIVIRTUAL_SPORTS_SOCCER_OFFENSE: u32 = 503316480;
pub const DIVIRTUAL_STRATEGY_ROLEPLAYING: u32 = 184549376;
pub const DIVIRTUAL_STRATEGY_TURN: u32 = 201326592;
pub const DIVOICE_ALL: u32 = 2197816330;
pub const DIVOICE_CHANNEL1: u32 = 2197816321;
pub const DIVOICE_CHANNEL2: u32 = 2197816322;
pub const DIVOICE_CHANNEL3: u32 = 2197816323;
pub const DIVOICE_CHANNEL4: u32 = 2197816324;
pub const DIVOICE_CHANNEL5: u32 = 2197816325;
pub const DIVOICE_CHANNEL6: u32 = 2197816326;
pub const DIVOICE_CHANNEL7: u32 = 2197816327;
pub const DIVOICE_CHANNEL8: u32 = 2197816328;
pub const DIVOICE_PLAYBACKMUTE: u32 = 2197816332;
pub const DIVOICE_RECORDMUTE: u32 = 2197816331;
pub const DIVOICE_TEAM: u32 = 2197816329;
pub const DIVOICE_TRANSMIT: u32 = 2197816333;
pub const DIVOICE_VOICECOMMAND: u32 = 2197816336;
pub const DI_BUFFEROVERFLOW: i32 = 1;
pub const DI_DEGREES: u32 = 100;
pub const DI_DOWNLOADSKIPPED: windows_sys::core::HRESULT = 0x3_u32 as _;
pub const DI_EFFECTRESTARTED: windows_sys::core::HRESULT = 0x4_u32 as _;
pub const DI_FFNOMINALMAX: u32 = 10000;
pub const DI_NOEFFECT: i32 = 1;
pub const DI_NOTATTACHED: i32 = 1;
pub const DI_OK: i32 = 0;
pub const DI_POLLEDDEVICE: windows_sys::core::HRESULT = 0x2_u32 as _;
pub const DI_PROPNOEFFECT: i32 = 1;
pub const DI_SECONDS: u32 = 1000000;
pub const DI_SETTINGSNOTSAVED: windows_sys::core::HRESULT = 0xB_u32 as _;
pub const DI_TRUNCATED: windows_sys::core::HRESULT = 0x8_u32 as _;
pub const DI_TRUNCATEDANDRESTARTED: windows_sys::core::HRESULT = 0xC_u32 as _;
pub const DI_WRITEPROTECT: windows_sys::core::HRESULT = 0x13_u32 as _;
pub type GPIOBUTTONS_BUTTON_TYPE = i32;
pub const GPIO_BUTTON_BACK: GPIOBUTTONS_BUTTON_TYPE = 5;
pub const GPIO_BUTTON_CAMERA_FOCUS: GPIOBUTTONS_BUTTON_TYPE = 7;
pub const GPIO_BUTTON_CAMERA_LENS: GPIOBUTTONS_BUTTON_TYPE = 12;
pub const GPIO_BUTTON_CAMERA_SHUTTER: GPIOBUTTONS_BUTTON_TYPE = 8;
pub const GPIO_BUTTON_COUNT: GPIOBUTTONS_BUTTON_TYPE = 16;
pub const GPIO_BUTTON_COUNT_MIN: GPIOBUTTONS_BUTTON_TYPE = 5;
pub const GPIO_BUTTON_HEADSET: GPIOBUTTONS_BUTTON_TYPE = 10;
pub const GPIO_BUTTON_HWKB_DEPLOY: GPIOBUTTONS_BUTTON_TYPE = 11;
pub const GPIO_BUTTON_OEM_CUSTOM: GPIOBUTTONS_BUTTON_TYPE = 13;
pub const GPIO_BUTTON_OEM_CUSTOM2: GPIOBUTTONS_BUTTON_TYPE = 14;
pub const GPIO_BUTTON_OEM_CUSTOM3: GPIOBUTTONS_BUTTON_TYPE = 15;
pub const GPIO_BUTTON_POWER: GPIOBUTTONS_BUTTON_TYPE = 0;
pub const GPIO_BUTTON_RINGER_TOGGLE: GPIOBUTTONS_BUTTON_TYPE = 9;
pub const GPIO_BUTTON_ROTATION_LOCK: GPIOBUTTONS_BUTTON_TYPE = 4;
pub const GPIO_BUTTON_SEARCH: GPIOBUTTONS_BUTTON_TYPE = 6;
pub const GPIO_BUTTON_VOLUME_DOWN: GPIOBUTTONS_BUTTON_TYPE = 3;
pub const GPIO_BUTTON_VOLUME_UP: GPIOBUTTONS_BUTTON_TYPE = 2;
pub const GPIO_BUTTON_WINDOWS: GPIOBUTTONS_BUTTON_TYPE = 1;
pub const GUID_Button: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa36d02f0_c9f3_11cf_bfc7_444553540000);
pub const GUID_ConstantForce: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13541c20_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_CustomForce: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13541c2b_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_DEVINTERFACE_HID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4d1e55b2_f16f_11cf_88cb_001111000030);
pub const GUID_DEVINTERFACE_KEYBOARD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x884b96c3_56ef_11d1_bc8c_00a0c91405dd);
pub const GUID_DEVINTERFACE_MOUSE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x378de44c_56ef_11d1_bc8c_00a0c91405dd);
pub const GUID_Damper: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13541c28_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Friction: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13541c2a_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_HIDClass: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x745a17a0_74d3_11d0_b6fe_00a0c90f57da);
pub const GUID_HID_INTERFACE_HIDPARSE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf5c315a5_69ac_4bc2_9279_d0b64576f44b);
pub const GUID_HID_INTERFACE_NOTIFY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2c4e2e88_25e6_4c33_882f_3d82e6073681);
pub const GUID_Inertia: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13541c29_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Joystick: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f1d2b70_d5a0_11cf_bfc7_444553540000);
pub const GUID_Key: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x55728220_d33c_11cf_bfc7_444553540000);
pub const GUID_KeyboardClass: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4d36e96b_e325_11ce_bfc1_08002be10318);
pub const GUID_MediaClass: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4d36e96c_e325_11ce_bfc1_08002be10318);
pub const GUID_MouseClass: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4d36e96f_e325_11ce_bfc1_08002be10318);
pub const GUID_POV: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa36d02f2_c9f3_11cf_bfc7_444553540000);
pub const GUID_RampForce: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13541c21_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_RxAxis: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa36d02f4_c9f3_11cf_bfc7_444553540000);
pub const GUID_RyAxis: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa36d02f5_c9f3_11cf_bfc7_444553540000);
pub const GUID_RzAxis: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa36d02e3_c9f3_11cf_bfc7_444553540000);
pub const GUID_SawtoothDown: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13541c26_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_SawtoothUp: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13541c25_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Sine: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13541c23_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Slider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa36d02e4_c9f3_11cf_bfc7_444553540000);
pub const GUID_Spring: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13541c27_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Square: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13541c22_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_SysKeyboard: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f1d2b61_d5a0_11cf_bfc7_444553540000);
pub const GUID_SysKeyboardEm: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f1d2b82_d5a0_11cf_bfc7_444553540000);
pub const GUID_SysKeyboardEm2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f1d2b83_d5a0_11cf_bfc7_444553540000);
pub const GUID_SysMouse: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f1d2b60_d5a0_11cf_bfc7_444553540000);
pub const GUID_SysMouseEm: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f1d2b80_d5a0_11cf_bfc7_444553540000);
pub const GUID_SysMouseEm2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f1d2b81_d5a0_11cf_bfc7_444553540000);
pub const GUID_Triangle: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13541c24_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Unknown: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa36d02f3_c9f3_11cf_bfc7_444553540000);
pub const GUID_XAxis: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa36d02e0_c9f3_11cf_bfc7_444553540000);
pub const GUID_YAxis: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa36d02e1_c9f3_11cf_bfc7_444553540000);
pub const GUID_ZAxis: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa36d02e2_c9f3_11cf_bfc7_444553540000);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HIDD_ATTRIBUTES {
    pub Size: u32,
    pub VendorID: u16,
    pub ProductID: u16,
    pub VersionNumber: u16,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct HIDD_CONFIGURATION {
    pub cookie: *mut core::ffi::c_void,
    pub size: u32,
    pub RingBufferSize: u32,
}
impl Default for HIDD_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HIDP_BUTTON_ARRAY_DATA {
    pub ArrayIndex: u16,
    pub On: bool,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HIDP_BUTTON_CAPS {
    pub UsagePage: u16,
    pub ReportID: u8,
    pub IsAlias: bool,
    pub BitField: u16,
    pub LinkCollection: u16,
    pub LinkUsage: u16,
    pub LinkUsagePage: u16,
    pub IsRange: bool,
    pub IsStringRange: bool,
    pub IsDesignatorRange: bool,
    pub IsAbsolute: bool,
    pub ReportCount: u16,
    pub Reserved2: u16,
    pub Reserved: [u32; 9],
    pub Anonymous: HIDP_BUTTON_CAPS_0,
}
impl Default for HIDP_BUTTON_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HIDP_BUTTON_CAPS_0 {
    pub Range: HIDP_BUTTON_CAPS_0_0,
    pub NotRange: HIDP_BUTTON_CAPS_0_1,
}
impl Default for HIDP_BUTTON_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HIDP_BUTTON_CAPS_0_0 {
    pub UsageMin: u16,
    pub UsageMax: u16,
    pub StringMin: u16,
    pub StringMax: u16,
    pub DesignatorMin: u16,
    pub DesignatorMax: u16,
    pub DataIndexMin: u16,
    pub DataIndexMax: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HIDP_BUTTON_CAPS_0_1 {
    pub Usage: u16,
    pub Reserved1: u16,
    pub StringIndex: u16,
    pub Reserved2: u16,
    pub DesignatorIndex: u16,
    pub Reserved3: u16,
    pub DataIndex: u16,
    pub Reserved4: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HIDP_CAPS {
    pub Usage: u16,
    pub UsagePage: u16,
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
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct HIDP_EXTENDED_ATTRIBUTES {
    pub NumGlobalUnknowns: u8,
    pub Reserved: [u8; 3],
    pub GlobalUnknowns: *mut HIDP_UNKNOWN_TOKEN,
    pub Data: [u32; 1],
}
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HIDP_KEYBOARD_MODIFIER_STATE_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct HIDP_LINK_COLLECTION_NODE {
    pub LinkUsage: u16,
    pub LinkUsagePage: u16,
    pub Parent: u16,
    pub NumberOfChildren: u16,
    pub NextSibling: u16,
    pub FirstChild: u16,
    pub _bitfield: u32,
    pub UserContext: *mut core::ffi::c_void,
}
impl Default for HIDP_LINK_COLLECTION_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HIDP_REPORT_TYPE = i32;
pub const HIDP_STATUS_BAD_LOG_PHY_VALUES: super::super::Foundation::NTSTATUS = 0xC0110006_u32 as _;
pub const HIDP_STATUS_BUFFER_TOO_SMALL: super::super::Foundation::NTSTATUS = 0xC0110007_u32 as _;
pub const HIDP_STATUS_BUTTON_NOT_PRESSED: super::super::Foundation::NTSTATUS = 0xC011000F_u32 as _;
pub const HIDP_STATUS_DATA_INDEX_NOT_FOUND: super::super::Foundation::NTSTATUS = 0xC011000D_u32 as _;
pub const HIDP_STATUS_DATA_INDEX_OUT_OF_RANGE: super::super::Foundation::NTSTATUS = 0xC011000E_u32 as _;
pub const HIDP_STATUS_I8042_TRANS_UNKNOWN: super::super::Foundation::NTSTATUS = 0xC0110009_u32 as _;
pub const HIDP_STATUS_I8242_TRANS_UNKNOWN: super::super::Foundation::NTSTATUS = 0xC0110009_u32 as _;
pub const HIDP_STATUS_INCOMPATIBLE_REPORT_ID: super::super::Foundation::NTSTATUS = 0xC011000A_u32 as _;
pub const HIDP_STATUS_INTERNAL_ERROR: super::super::Foundation::NTSTATUS = 0xC0110008_u32 as _;
pub const HIDP_STATUS_INVALID_PREPARSED_DATA: super::super::Foundation::NTSTATUS = 0xC0110001_u32 as _;
pub const HIDP_STATUS_INVALID_REPORT_LENGTH: super::super::Foundation::NTSTATUS = 0xC0110003_u32 as _;
pub const HIDP_STATUS_INVALID_REPORT_TYPE: super::super::Foundation::NTSTATUS = 0xC0110002_u32 as _;
pub const HIDP_STATUS_IS_VALUE_ARRAY: super::super::Foundation::NTSTATUS = 0xC011000C_u32 as _;
pub const HIDP_STATUS_NOT_BUTTON_ARRAY: super::super::Foundation::NTSTATUS = 0xC0110021_u32 as _;
pub const HIDP_STATUS_NOT_IMPLEMENTED: super::super::Foundation::NTSTATUS = 0xC0110020_u32 as _;
pub const HIDP_STATUS_NOT_VALUE_ARRAY: super::super::Foundation::NTSTATUS = 0xC011000B_u32 as _;
pub const HIDP_STATUS_NULL: super::super::Foundation::NTSTATUS = 0x80110001_u32 as _;
pub const HIDP_STATUS_REPORT_DOES_NOT_EXIST: super::super::Foundation::NTSTATUS = 0xC0110010_u32 as _;
pub const HIDP_STATUS_SUCCESS: super::super::Foundation::NTSTATUS = 0x110000_u32 as _;
pub const HIDP_STATUS_USAGE_NOT_FOUND: super::super::Foundation::NTSTATUS = 0xC0110004_u32 as _;
pub const HIDP_STATUS_VALUE_OUT_OF_RANGE: super::super::Foundation::NTSTATUS = 0xC0110005_u32 as _;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy)]
pub struct HIDP_VALUE_CAPS {
    pub UsagePage: u16,
    pub ReportID: u8,
    pub IsAlias: bool,
    pub BitField: u16,
    pub LinkCollection: u16,
    pub LinkUsage: u16,
    pub LinkUsagePage: u16,
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
impl Default for HIDP_VALUE_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HIDP_VALUE_CAPS_0 {
    pub Range: HIDP_VALUE_CAPS_0_0,
    pub NotRange: HIDP_VALUE_CAPS_0_1,
}
impl Default for HIDP_VALUE_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HIDP_VALUE_CAPS_0_0 {
    pub UsageMin: u16,
    pub UsageMax: u16,
    pub StringMin: u16,
    pub StringMax: u16,
    pub DesignatorMin: u16,
    pub DesignatorMax: u16,
    pub DataIndexMin: u16,
    pub DataIndexMax: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HIDP_VALUE_CAPS_0_1 {
    pub Usage: u16,
    pub Reserved1: u16,
    pub StringIndex: u16,
    pub Reserved2: u16,
    pub DesignatorIndex: u16,
    pub Reserved3: u16,
    pub DataIndex: u16,
    pub Reserved4: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HID_COLLECTION_INFORMATION {
    pub DescriptorSize: u32,
    pub Polled: bool,
    pub Reserved1: [u8; 1],
    pub VendorID: u16,
    pub ProductID: u16,
    pub VersionNumber: u16,
}
impl Default for HID_COLLECTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HID_DRIVER_CONFIG {
    pub Size: u32,
    pub RingBufferSize: u32,
}
pub const HID_REVISION: u32 = 1;
pub const HID_USAGE_ALPHANUMERIC_14_SEGMENT_DIRECT_MAP: u16 = 69;
pub const HID_USAGE_ALPHANUMERIC_7_SEGMENT_DIRECT_MAP: u16 = 67;
pub const HID_USAGE_ALPHANUMERIC_ALPHANUMERIC_DISPLAY: u16 = 1;
pub const HID_USAGE_ALPHANUMERIC_ASCII_CHARACTER_SET: u16 = 33;
pub const HID_USAGE_ALPHANUMERIC_ATTRIBUTE_DATA: u16 = 74;
pub const HID_USAGE_ALPHANUMERIC_ATTRIBUTE_READBACK: u16 = 73;
pub const HID_USAGE_ALPHANUMERIC_BITMAPPED_DISPLAY: u16 = 2;
pub const HID_USAGE_ALPHANUMERIC_BITMAP_SIZE_X: u16 = 128;
pub const HID_USAGE_ALPHANUMERIC_BITMAP_SIZE_Y: u16 = 129;
pub const HID_USAGE_ALPHANUMERIC_BIT_DEPTH_FORMAT: u16 = 131;
pub const HID_USAGE_ALPHANUMERIC_BLIT_DATA: u16 = 143;
pub const HID_USAGE_ALPHANUMERIC_BLIT_RECTANGLE_X1: u16 = 139;
pub const HID_USAGE_ALPHANUMERIC_BLIT_RECTANGLE_X2: u16 = 141;
pub const HID_USAGE_ALPHANUMERIC_BLIT_RECTANGLE_Y1: u16 = 140;
pub const HID_USAGE_ALPHANUMERIC_BLIT_RECTANGLE_Y2: u16 = 142;
pub const HID_USAGE_ALPHANUMERIC_BLIT_REPORT: u16 = 138;
pub const HID_USAGE_ALPHANUMERIC_CHARACTER_ATTRIBUTE: u16 = 72;
pub const HID_USAGE_ALPHANUMERIC_CHARACTER_MAPPING: u16 = 207;
pub const HID_USAGE_ALPHANUMERIC_CHARACTER_PAGE_MAPPING: u16 = 223;
pub const HID_USAGE_ALPHANUMERIC_CHARACTER_REPORT: u16 = 43;
pub const HID_USAGE_ALPHANUMERIC_CHAR_ATTR_BLINK: u16 = 77;
pub const HID_USAGE_ALPHANUMERIC_CHAR_ATTR_ENHANCE: u16 = 75;
pub const HID_USAGE_ALPHANUMERIC_CHAR_ATTR_UNDERLINE: u16 = 76;
pub const HID_USAGE_ALPHANUMERIC_CHAR_HEIGHT: u16 = 62;
pub const HID_USAGE_ALPHANUMERIC_CHAR_SPACING_HORIZONTAL: u16 = 63;
pub const HID_USAGE_ALPHANUMERIC_CHAR_SPACING_VERTICAL: u16 = 64;
pub const HID_USAGE_ALPHANUMERIC_CHAR_WIDTH: u16 = 61;
pub const HID_USAGE_ALPHANUMERIC_CLEAR_DISPLAY: u16 = 37;
pub const HID_USAGE_ALPHANUMERIC_COLUMN: u16 = 52;
pub const HID_USAGE_ALPHANUMERIC_COLUMNS: u16 = 54;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_BLINK: u16 = 58;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_ENABLE: u16 = 57;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_MODE: u16 = 56;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_PIXEL_POSITIONING: u16 = 55;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_POSITION_REPORT: u16 = 50;
pub const HID_USAGE_ALPHANUMERIC_DATA_READ_BACK: u16 = 34;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_ATTRIBUTES_REPORT: u16 = 32;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_BRIGHTNESS: u16 = 70;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_CONTRAST: u16 = 71;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_CONTROL_REPORT: u16 = 36;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_DATA: u16 = 44;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_DATA_EXTENSIONS: u16 = 204;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_ENABLE: u16 = 38;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_ORIENTATION: u16 = 132;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_STATUS: u16 = 45;
pub const HID_USAGE_ALPHANUMERIC_ERR_FONT_DATA_CANNOT_BE_READ: u16 = 49;
pub const HID_USAGE_ALPHANUMERIC_ERR_NOT_A_LOADABLE_CHARACTER: u16 = 48;
pub const HID_USAGE_ALPHANUMERIC_FONT_14_SEGMENT: u16 = 68;
pub const HID_USAGE_ALPHANUMERIC_FONT_7_SEGMENT: u16 = 66;
pub const HID_USAGE_ALPHANUMERIC_FONT_DATA: u16 = 60;
pub const HID_USAGE_ALPHANUMERIC_FONT_READ_BACK: u16 = 35;
pub const HID_USAGE_ALPHANUMERIC_FONT_REPORT: u16 = 59;
pub const HID_USAGE_ALPHANUMERIC_HORIZONTAL_SCROLL: u16 = 42;
pub const HID_USAGE_ALPHANUMERIC_MAX_BLIT_SIZE: u16 = 130;
pub const HID_USAGE_ALPHANUMERIC_PALETTE_DATA: u16 = 136;
pub const HID_USAGE_ALPHANUMERIC_PALETTE_DATA_OFFSET: u16 = 135;
pub const HID_USAGE_ALPHANUMERIC_PALETTE_DATA_SIZE: u16 = 134;
pub const HID_USAGE_ALPHANUMERIC_PALETTE_REPORT: u16 = 133;
pub const HID_USAGE_ALPHANUMERIC_REQUEST_REPORT: u16 = 255;
pub const HID_USAGE_ALPHANUMERIC_ROW: u16 = 51;
pub const HID_USAGE_ALPHANUMERIC_ROWS: u16 = 53;
pub const HID_USAGE_ALPHANUMERIC_SCREEN_SAVER_DELAY: u16 = 39;
pub const HID_USAGE_ALPHANUMERIC_SCREEN_SAVER_ENABLE: u16 = 40;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON: u16 = 144;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_ID: u16 = 145;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_OFFSET1: u16 = 147;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_OFFSET2: u16 = 148;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_REPORT: u16 = 149;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_SIDE: u16 = 146;
pub const HID_USAGE_ALPHANUMERIC_SOFT_KEYS: u16 = 194;
pub const HID_USAGE_ALPHANUMERIC_STATUS_NOT_READY: u16 = 46;
pub const HID_USAGE_ALPHANUMERIC_STATUS_READY: u16 = 47;
pub const HID_USAGE_ALPHANUMERIC_UNICODE_CHAR_SET: u16 = 65;
pub const HID_USAGE_ALPHANUMERIC_UNICODE_EQUIVALENT: u16 = 221;
pub const HID_USAGE_ALPHANUMERIC_VERTICAL_SCROLL: u16 = 41;
pub const HID_USAGE_ARCADE_ALARM_INPUT: u16 = 69;
pub const HID_USAGE_ARCADE_COIN_DOOR: u16 = 2;
pub const HID_USAGE_ARCADE_COIN_DOOR_COUNTER: u16 = 70;
pub const HID_USAGE_ARCADE_COIN_DOOR_LOCKOUT: u16 = 64;
pub const HID_USAGE_ARCADE_COIN_DOOR_TEST: u16 = 57;
pub const HID_USAGE_ARCADE_COIN_DRAWER_DROP_COUNT: u16 = 53;
pub const HID_USAGE_ARCADE_COIN_DRAWER_SERVICE: u16 = 55;
pub const HID_USAGE_ARCADE_COIN_DRAWER_START: u16 = 54;
pub const HID_USAGE_ARCADE_COIN_DRAWER_TILT: u16 = 56;
pub const HID_USAGE_ARCADE_EXTENDED_OPTICAL_INPUT_STATE: u16 = 73;
pub const HID_USAGE_ARCADE_GENERAL_PURPOSE_ANALOG_INPUT_STATE: u16 = 48;
pub const HID_USAGE_ARCADE_GENERAL_PURPOSE_DIGITAL_INPUT_STATE: u16 = 49;
pub const HID_USAGE_ARCADE_GENERAL_PURPOSE_DIGITAL_OUTPUT_STATE: u16 = 51;
pub const HID_USAGE_ARCADE_GENERAL_PURPOSE_IO_CARD: u16 = 1;
pub const HID_USAGE_ARCADE_GENERAL_PURPOSE_OPTICAL_INPUT_STATE: u16 = 50;
pub const HID_USAGE_ARCADE_IO_DIRECTION_MAPPING: u16 = 71;
pub const HID_USAGE_ARCADE_NUMBER_OF_COIN_DOORS: u16 = 52;
pub const HID_USAGE_ARCADE_PIN_PAD_COMMAND: u16 = 77;
pub const HID_USAGE_ARCADE_PIN_PAD_INPUT_STATE: u16 = 74;
pub const HID_USAGE_ARCADE_PIN_PAD_OUTPUT: u16 = 76;
pub const HID_USAGE_ARCADE_PIN_PAD_STATUS: u16 = 75;
pub const HID_USAGE_ARCADE_SET_IO_DIRECTION_MAPPING: u16 = 72;
pub const HID_USAGE_ARCADE_WATCHDOG_ACTION: u16 = 66;
pub const HID_USAGE_ARCADE_WATCHDOG_REBOOT: u16 = 67;
pub const HID_USAGE_ARCADE_WATCHDOG_RESTART: u16 = 68;
pub const HID_USAGE_ARCADE_WATCHDOG_TIMEOUT: u16 = 65;
pub const HID_USAGE_ARCADE_WATCHDOG_TIMER: u16 = 3;
pub const HID_USAGE_BARCODE_SCANNER_2D_CONTROL_REPORT: u16 = 31;
pub const HID_USAGE_BARCODE_SCANNER_ACTIVE_TIME: u16 = 85;
pub const HID_USAGE_BARCODE_SCANNER_ADD_EAN_23_LABEL_DEFINITION: u16 = 191;
pub const HID_USAGE_BARCODE_SCANNER_AIMINGPOINTER_MODE: u16 = 48;
pub const HID_USAGE_BARCODE_SCANNER_AIMING_LASER_PATTERN: u16 = 86;
pub const HID_USAGE_BARCODE_SCANNER_AIM_DURATION: u16 = 122;
pub const HID_USAGE_BARCODE_SCANNER_ATTRIBUTE_REPORT: u16 = 16;
pub const HID_USAGE_BARCODE_SCANNER_AZTEC_CODE: u16 = 272;
pub const HID_USAGE_BARCODE_SCANNER_BARCODE_BADGE_READER: u16 = 1;
pub const HID_USAGE_BARCODE_SCANNER_BARCODE_SCANNER: u16 = 2;
pub const HID_USAGE_BARCODE_SCANNER_BAR_CODE_PRESENT: u16 = 87;
pub const HID_USAGE_BARCODE_SCANNER_BAR_CODE_PRESENT_SENSOR: u16 = 49;
pub const HID_USAGE_BARCODE_SCANNER_BAR_CODE_SCANNER_CRADLE: u16 = 5;
pub const HID_USAGE_BARCODE_SCANNER_BAR_SPACE_DATA: u16 = 256;
pub const HID_USAGE_BARCODE_SCANNER_BC412: u16 = 273;
pub const HID_USAGE_BARCODE_SCANNER_BEEPER_STATE: u16 = 88;
pub const HID_USAGE_BARCODE_SCANNER_BOOKLAND_EAN: u16 = 145;
pub const HID_USAGE_BARCODE_SCANNER_CHANNEL_CODE: u16 = 274;
pub const HID_USAGE_BARCODE_SCANNER_CHECK: u16 = 176;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT: u16 = 214;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_CODABAR_ENABLE: u16 = 222;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_CODE_39_ENABLE: u16 = 223;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_DISABLE: u16 = 215;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_ENABLE_INTERLEAVED_2_OF_5_OPCC: u16 = 216;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_ENABLE_INTERLEAVED_2_OF_5_USS: u16 = 217;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_ENABLE_ONE_MSI_PLESSEY: u16 = 220;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_ENABLE_STANDARD_2_OF_5_OPCC: u16 = 218;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_ENABLE_STANDARD_2_OF_5_USS: u16 = 219;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_ENABLE_TWO_MSI_PLESSEY: u16 = 221;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DISABLE_PRICE: u16 = 177;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_ENABLE_4_DIGIT_PRICE: u16 = 178;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_ENABLE_5_DIGIT_PRICE: u16 = 179;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_ENABLE_EUROPEAN_4_DIGIT_PRICE: u16 = 180;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_ENABLE_EUROPEAN_5_DIGIT_PRICE: u16 = 181;
pub const HID_USAGE_BARCODE_SCANNER_CLASS_1A_LASER: u16 = 50;
pub const HID_USAGE_BARCODE_SCANNER_CLASS_2_LASER: u16 = 51;
pub const HID_USAGE_BARCODE_SCANNER_CLEAR_ALL_EAN_23_LABEL_DEFINITIONS: u16 = 192;
pub const HID_USAGE_BARCODE_SCANNER_CODABAR: u16 = 195;
pub const HID_USAGE_BARCODE_SCANNER_CODABAR_CONTROL_REPORT: u16 = 28;
pub const HID_USAGE_BARCODE_SCANNER_CODE_128: u16 = 196;
pub const HID_USAGE_BARCODE_SCANNER_CODE_128_CONTROL_REPORT: u16 = 29;
pub const HID_USAGE_BARCODE_SCANNER_CODE_16: u16 = 275;
pub const HID_USAGE_BARCODE_SCANNER_CODE_32: u16 = 276;
pub const HID_USAGE_BARCODE_SCANNER_CODE_39: u16 = 199;
pub const HID_USAGE_BARCODE_SCANNER_CODE_39_CONTROL_REPORT: u16 = 24;
pub const HID_USAGE_BARCODE_SCANNER_CODE_49: u16 = 277;
pub const HID_USAGE_BARCODE_SCANNER_CODE_93: u16 = 200;
pub const HID_USAGE_BARCODE_SCANNER_CODE_ONE: u16 = 278;
pub const HID_USAGE_BARCODE_SCANNER_COLORCODE: u16 = 279;
pub const HID_USAGE_BARCODE_SCANNER_COMMIT_PARAMETERS_TO_NVM: u16 = 109;
pub const HID_USAGE_BARCODE_SCANNER_CONSTANT_ELECTRONIC_ARTICLE_SURVEILLANCE: u16 = 55;
pub const HID_USAGE_BARCODE_SCANNER_CONTACT_SCANNER: u16 = 53;
pub const HID_USAGE_BARCODE_SCANNER_CONVERT_EAN_8_TO_13_TYPE: u16 = 146;
pub const HID_USAGE_BARCODE_SCANNER_CONVERT_UPCE_TO_A: u16 = 148;
pub const HID_USAGE_BARCODE_SCANNER_CONVERT_UPC_A_TO_EAN13: u16 = 147;
pub const HID_USAGE_BARCODE_SCANNER_CORDLESS_SCANNER_BASE: u16 = 4;
pub const HID_USAGE_BARCODE_SCANNER_DATA_LENGTH_METHOD: u16 = 266;
pub const HID_USAGE_BARCODE_SCANNER_DATA_MATRIX: u16 = 280;
pub const HID_USAGE_BARCODE_SCANNER_DATA_PREFIX: u16 = 79;
pub const HID_USAGE_BARCODE_SCANNER_DECODED_DATA: u16 = 254;
pub const HID_USAGE_BARCODE_SCANNER_DECODE_DATA_CONTINUED: u16 = 255;
pub const HID_USAGE_BARCODE_SCANNER_DISABLE_CHECK_DIGIT_TRANSMIT: u16 = 241;
pub const HID_USAGE_BARCODE_SCANNER_DISCRETE_LENGTH_TO_DECODE_1: u16 = 264;
pub const HID_USAGE_BARCODE_SCANNER_DISCRETE_LENGTH_TO_DECODE_2: u16 = 265;
pub const HID_USAGE_BARCODE_SCANNER_DL_METHOD_CHECK_FOR_DISCRETE: u16 = 269;
pub const HID_USAGE_BARCODE_SCANNER_DL_METHOD_CHECK_IN_RANGE: u16 = 268;
pub const HID_USAGE_BARCODE_SCANNER_DL_METHOD_READ_ANY: u16 = 267;
pub const HID_USAGE_BARCODE_SCANNER_DUMB_BAR_CODE_SCANNER: u16 = 3;
pub const HID_USAGE_BARCODE_SCANNER_EAN13: u16 = 149;
pub const HID_USAGE_BARCODE_SCANNER_EAN8: u16 = 150;
pub const HID_USAGE_BARCODE_SCANNER_EAN99_128_MANDATORY: u16 = 151;
pub const HID_USAGE_BARCODE_SCANNER_EAN99_P5128_OPTIONAL: u16 = 152;
pub const HID_USAGE_BARCODE_SCANNER_EAN_13_FLAG_DIGIT_1: u16 = 188;
pub const HID_USAGE_BARCODE_SCANNER_EAN_13_FLAG_DIGIT_2: u16 = 189;
pub const HID_USAGE_BARCODE_SCANNER_EAN_13_FLAG_DIGIT_3: u16 = 190;
pub const HID_USAGE_BARCODE_SCANNER_EAN_23_LABEL_CONTROL_REPORT: u16 = 23;
pub const HID_USAGE_BARCODE_SCANNER_EAN_8_FLAG_DIGIT_1: u16 = 185;
pub const HID_USAGE_BARCODE_SCANNER_EAN_8_FLAG_DIGIT_2: u16 = 186;
pub const HID_USAGE_BARCODE_SCANNER_EAN_8_FLAG_DIGIT_3: u16 = 187;
pub const HID_USAGE_BARCODE_SCANNER_EAN_THREE_LABEL: u16 = 184;
pub const HID_USAGE_BARCODE_SCANNER_EAN_TWO_LABEL: u16 = 183;
pub const HID_USAGE_BARCODE_SCANNER_ELECTRONIC_ARTICLE_SURVEILLANCE_NOTIFICATION: u16 = 54;
pub const HID_USAGE_BARCODE_SCANNER_ENABLE_CHECK_DIGIT_TRANSMIT: u16 = 242;
pub const HID_USAGE_BARCODE_SCANNER_ENABLE_EAN_TWO_LABEL: u16 = 153;
pub const HID_USAGE_BARCODE_SCANNER_ERROR_INDICATION: u16 = 56;
pub const HID_USAGE_BARCODE_SCANNER_FIXED_BEEPER: u16 = 57;
pub const HID_USAGE_BARCODE_SCANNER_FRAGMENT_DECODING: u16 = 77;
pub const HID_USAGE_BARCODE_SCANNER_FULL_ASCII_CONVERSION: u16 = 201;
pub const HID_USAGE_BARCODE_SCANNER_GOOD_DECODE_INDICATION: u16 = 58;
pub const HID_USAGE_BARCODE_SCANNER_GOOD_READ_LAMP_DURATION: u16 = 123;
pub const HID_USAGE_BARCODE_SCANNER_GOOD_READ_LAMP_INTENSITY: u16 = 124;
pub const HID_USAGE_BARCODE_SCANNER_GOOD_READ_LED: u16 = 125;
pub const HID_USAGE_BARCODE_SCANNER_GOOD_READ_TONE_FREQUENCY: u16 = 126;
pub const HID_USAGE_BARCODE_SCANNER_GOOD_READ_TONE_LENGTH: u16 = 127;
pub const HID_USAGE_BARCODE_SCANNER_GOOD_READ_TONE_VOLUME: u16 = 128;
pub const HID_USAGE_BARCODE_SCANNER_GOOD_READ_WHEN_TO_WRITE: u16 = 136;
pub const HID_USAGE_BARCODE_SCANNER_GRWTI_AFTER_DECODE: u16 = 137;
pub const HID_USAGE_BARCODE_SCANNER_GRWTI_BEEPLAMP_AFTER_TRANSMIT: u16 = 138;
pub const HID_USAGE_BARCODE_SCANNER_GRWTI_NO_BEEPLAMP_USE_AT_ALL: u16 = 139;
pub const HID_USAGE_BARCODE_SCANNER_HANDS_FREE_SCANNING: u16 = 59;
pub const HID_USAGE_BARCODE_SCANNER_HEATER_PRESENT: u16 = 52;
pub const HID_USAGE_BARCODE_SCANNER_INITIATE_BARCODE_READ: u16 = 96;
pub const HID_USAGE_BARCODE_SCANNER_INTERLEAVED_2_OF_5: u16 = 202;
pub const HID_USAGE_BARCODE_SCANNER_INTERLEAVED_2_OF_5_CONTROL_REPORT: u16 = 25;
pub const HID_USAGE_BARCODE_SCANNER_INTRINSICALLY_SAFE: u16 = 60;
pub const HID_USAGE_BARCODE_SCANNER_ITALIAN_PHARMACY_CODE: u16 = 203;
pub const HID_USAGE_BARCODE_SCANNER_KLASSE_EINS_LASER: u16 = 61;
pub const HID_USAGE_BARCODE_SCANNER_LASER_ON_TIME: u16 = 89;
pub const HID_USAGE_BARCODE_SCANNER_LASER_STATE: u16 = 90;
pub const HID_USAGE_BARCODE_SCANNER_LOCKOUT_TIME: u16 = 91;
pub const HID_USAGE_BARCODE_SCANNER_LONG_RANGE_SCANNER: u16 = 62;
pub const HID_USAGE_BARCODE_SCANNER_MAXICODE: u16 = 281;
pub const HID_USAGE_BARCODE_SCANNER_MAXIMUM_LENGTH_TO_DECODE: u16 = 263;
pub const HID_USAGE_BARCODE_SCANNER_MICROPDF: u16 = 282;
pub const HID_USAGE_BARCODE_SCANNER_MINIMUM_LENGTH_TO_DECODE: u16 = 262;
pub const HID_USAGE_BARCODE_SCANNER_MIRROR_SPEED_CONTROL: u16 = 63;
pub const HID_USAGE_BARCODE_SCANNER_MISC_1D_CONTROL_REPORT: u16 = 30;
pub const HID_USAGE_BARCODE_SCANNER_MOTOR_STATE: u16 = 92;
pub const HID_USAGE_BARCODE_SCANNER_MOTOR_TIMEOUT: u16 = 93;
pub const HID_USAGE_BARCODE_SCANNER_MSIPLESSEY: u16 = 204;
pub const HID_USAGE_BARCODE_SCANNER_MSI_PLESSEY_CONTROL_REPORT: u16 = 27;
pub const HID_USAGE_BARCODE_SCANNER_MULTIRANGE_SCANNER: u16 = 69;
pub const HID_USAGE_BARCODE_SCANNER_NOT_ON_FILE_INDICATION: u16 = 64;
pub const HID_USAGE_BARCODE_SCANNER_NOT_ON_FILE_VOLUME: u16 = 131;
pub const HID_USAGE_BARCODE_SCANNER_NO_READ_MESSAGE: u16 = 130;
pub const HID_USAGE_BARCODE_SCANNER_PARAMETERS_CHANGED: u16 = 111;
pub const HID_USAGE_BARCODE_SCANNER_PARAMETER_SCANNING: u16 = 110;
pub const HID_USAGE_BARCODE_SCANNER_PDF417: u16 = 283;
pub const HID_USAGE_BARCODE_SCANNER_PERIODICAL: u16 = 169;
pub const HID_USAGE_BARCODE_SCANNER_PERIODICAL_AUTODISCRIMINATE_2: u16 = 170;
pub const HID_USAGE_BARCODE_SCANNER_PERIODICAL_AUTODISCRIMINATE_5: u16 = 173;
pub const HID_USAGE_BARCODE_SCANNER_PERIODICAL_IGNORE_2: u16 = 172;
pub const HID_USAGE_BARCODE_SCANNER_PERIODICAL_IGNORE_5: u16 = 175;
pub const HID_USAGE_BARCODE_SCANNER_PERIODICAL_ONLY_DECODE_WITH_2: u16 = 171;
pub const HID_USAGE_BARCODE_SCANNER_PERIODICAL_ONLY_DECODE_WITH_5: u16 = 174;
pub const HID_USAGE_BARCODE_SCANNER_POLARITY_INVERTED_BAR_CODE: u16 = 259;
pub const HID_USAGE_BARCODE_SCANNER_POLARITY_NORMAL_BAR_CODE: u16 = 260;
pub const HID_USAGE_BARCODE_SCANNER_POSICODE: u16 = 284;
pub const HID_USAGE_BARCODE_SCANNER_POWERUP_BEEP: u16 = 132;
pub const HID_USAGE_BARCODE_SCANNER_POWER_ON_RESET_SCANNER: u16 = 94;
pub const HID_USAGE_BARCODE_SCANNER_PREFIX_AIMI: u16 = 80;
pub const HID_USAGE_BARCODE_SCANNER_PREFIX_NONE: u16 = 81;
pub const HID_USAGE_BARCODE_SCANNER_PREFIX_PROPRIETARY: u16 = 82;
pub const HID_USAGE_BARCODE_SCANNER_PREVENT_READ_OF_BARCODES: u16 = 95;
pub const HID_USAGE_BARCODE_SCANNER_PROGRAMMABLE_BEEPER: u16 = 65;
pub const HID_USAGE_BARCODE_SCANNER_PROXIMITY_SENSOR: u16 = 70;
pub const HID_USAGE_BARCODE_SCANNER_QR_CODE: u16 = 285;
pub const HID_USAGE_BARCODE_SCANNER_RAW_DATA_POLARITY: u16 = 258;
pub const HID_USAGE_BARCODE_SCANNER_RAW_SCANNED_DATA_REPORT: u16 = 19;
pub const HID_USAGE_BARCODE_SCANNER_SCANNED_DATA_REPORT: u16 = 18;
pub const HID_USAGE_BARCODE_SCANNER_SCANNER_DATA_ACCURACY: u16 = 257;
pub const HID_USAGE_BARCODE_SCANNER_SCANNER_IN_CRADLE: u16 = 117;
pub const HID_USAGE_BARCODE_SCANNER_SCANNER_IN_RANGE: u16 = 118;
pub const HID_USAGE_BARCODE_SCANNER_SCANNER_READ_CONFIDENCE: u16 = 78;
pub const HID_USAGE_BARCODE_SCANNER_SETTINGS_REPORT: u16 = 17;
pub const HID_USAGE_BARCODE_SCANNER_SET_PARAMETER_DEFAULT_VALUES: u16 = 112;
pub const HID_USAGE_BARCODE_SCANNER_SOUND_ERROR_BEEP: u16 = 133;
pub const HID_USAGE_BARCODE_SCANNER_SOUND_GOOD_READ_BEEP: u16 = 134;
pub const HID_USAGE_BARCODE_SCANNER_SOUND_NOT_ON_FILE_BEEP: u16 = 135;
pub const HID_USAGE_BARCODE_SCANNER_STANDARD_2_OF_5: u16 = 206;
pub const HID_USAGE_BARCODE_SCANNER_STANDARD_2_OF_5_CONTROL_REPORT: u16 = 26;
pub const HID_USAGE_BARCODE_SCANNER_STANDARD_2_OF_5_IATA: u16 = 205;
pub const HID_USAGE_BARCODE_SCANNER_STATUS_REPORT: u16 = 21;
pub const HID_USAGE_BARCODE_SCANNER_SUPERCODE: u16 = 286;
pub const HID_USAGE_BARCODE_SCANNER_SYMBOLOGY_IDENTIFIER_1: u16 = 251;
pub const HID_USAGE_BARCODE_SCANNER_SYMBOLOGY_IDENTIFIER_2: u16 = 252;
pub const HID_USAGE_BARCODE_SCANNER_SYMBOLOGY_IDENTIFIER_3: u16 = 253;
pub const HID_USAGE_BARCODE_SCANNER_TRANSMIT_CHECK_DIGIT: u16 = 240;
pub const HID_USAGE_BARCODE_SCANNER_TRANSMIT_STARTSTOP: u16 = 211;
pub const HID_USAGE_BARCODE_SCANNER_TRIGGERLESS: u16 = 66;
pub const HID_USAGE_BARCODE_SCANNER_TRIGGER_MODE: u16 = 98;
pub const HID_USAGE_BARCODE_SCANNER_TRIGGER_MODE_BLINKING_LASER_ON: u16 = 99;
pub const HID_USAGE_BARCODE_SCANNER_TRIGGER_MODE_CONTINUOUS_LASER_ON: u16 = 100;
pub const HID_USAGE_BARCODE_SCANNER_TRIGGER_MODE_LASER_ON_WHILE_PULLED: u16 = 101;
pub const HID_USAGE_BARCODE_SCANNER_TRIGGER_MODE_LASER_STAYS_ON_AFTER_RELEASE: u16 = 102;
pub const HID_USAGE_BARCODE_SCANNER_TRIGGER_REPORT: u16 = 20;
pub const HID_USAGE_BARCODE_SCANNER_TRIGGER_STATE: u16 = 97;
pub const HID_USAGE_BARCODE_SCANNER_TRIOPTIC: u16 = 212;
pub const HID_USAGE_BARCODE_SCANNER_UCCEAN128: u16 = 213;
pub const HID_USAGE_BARCODE_SCANNER_ULTRACODE: u16 = 287;
pub const HID_USAGE_BARCODE_SCANNER_UPCA: u16 = 157;
pub const HID_USAGE_BARCODE_SCANNER_UPCA_WITH_128_MANDATORY: u16 = 158;
pub const HID_USAGE_BARCODE_SCANNER_UPCA_WITH_128_OPTIONAL: u16 = 159;
pub const HID_USAGE_BARCODE_SCANNER_UPCA_WITH_P5_OPTIONAL: u16 = 160;
pub const HID_USAGE_BARCODE_SCANNER_UPCE: u16 = 161;
pub const HID_USAGE_BARCODE_SCANNER_UPCE1: u16 = 162;
pub const HID_USAGE_BARCODE_SCANNER_UPCEAN: u16 = 154;
pub const HID_USAGE_BARCODE_SCANNER_UPCEAN_CONTROL_REPORT: u16 = 22;
pub const HID_USAGE_BARCODE_SCANNER_UPCEAN_COUPON_CODE: u16 = 155;
pub const HID_USAGE_BARCODE_SCANNER_UPCEAN_PERIODICALS: u16 = 156;
pub const HID_USAGE_BARCODE_SCANNER_USD5_SLUG_CODE: u16 = 288;
pub const HID_USAGE_BARCODE_SCANNER_VERICODE: u16 = 289;
pub const HID_USAGE_BARCODE_SCANNER_WAND: u16 = 67;
pub const HID_USAGE_BARCODE_SCANNER_WATER_RESISTANT: u16 = 68;
pub const HID_USAGE_BATTERY_SYSTEM_ABSOLUTE_STATE_OF_CHARGE: u16 = 101;
pub const HID_USAGE_BATTERY_SYSTEM_AC_PRESENT: u16 = 208;
pub const HID_USAGE_BATTERY_SYSTEM_ALARM_INHIBITED: u16 = 211;
pub const HID_USAGE_BATTERY_SYSTEM_AT_RATE: u16 = 43;
pub const HID_USAGE_BATTERY_SYSTEM_AT_RATE_OK: u16 = 73;
pub const HID_USAGE_BATTERY_SYSTEM_AT_RATE_TIME_TO_EMPTY: u16 = 97;
pub const HID_USAGE_BATTERY_SYSTEM_AT_RATE_TIME_TO_FULL: u16 = 96;
pub const HID_USAGE_BATTERY_SYSTEM_AVERAGE_CURRENT: u16 = 98;
pub const HID_USAGE_BATTERY_SYSTEM_AVERAGE_TIME_TO_EMPTY: u16 = 105;
pub const HID_USAGE_BATTERY_SYSTEM_AVERAGE_TIME_TO_FULL: u16 = 106;
pub const HID_USAGE_BATTERY_SYSTEM_BATTERY_INSERTION: u16 = 24;
pub const HID_USAGE_BATTERY_SYSTEM_BATTERY_PACK_MODEL_LEVEL: u16 = 128;
pub const HID_USAGE_BATTERY_SYSTEM_BATTERY_PRESENT: u16 = 209;
pub const HID_USAGE_BATTERY_SYSTEM_BATTERY_SUPPORTED: u16 = 27;
pub const HID_USAGE_BATTERY_SYSTEM_BELOW_REMAINING_CAPACITY_LIMIT: u16 = 66;
pub const HID_USAGE_BATTERY_SYSTEM_BROADCAST_TO_CHARGER: u16 = 45;
pub const HID_USAGE_BATTERY_SYSTEM_CAPACITY_GRANULARITY_1: u16 = 141;
pub const HID_USAGE_BATTERY_SYSTEM_CAPACITY_GRANULARITY_2: u16 = 142;
pub const HID_USAGE_BATTERY_SYSTEM_CAPACITY_MODE: u16 = 44;
pub const HID_USAGE_BATTERY_SYSTEM_CHARGER_CONNECTION: u16 = 23;
pub const HID_USAGE_BATTERY_SYSTEM_CHARGER_SELECTOR_SUPPORT: u16 = 240;
pub const HID_USAGE_BATTERY_SYSTEM_CHARGER_SPEC: u16 = 241;
pub const HID_USAGE_BATTERY_SYSTEM_CHARGE_CONTROLLER: u16 = 47;
pub const HID_USAGE_BATTERY_SYSTEM_CHARGING: u16 = 68;
pub const HID_USAGE_BATTERY_SYSTEM_CHARGING_INDICATOR: u16 = 29;
pub const HID_USAGE_BATTERY_SYSTEM_CONDITIONING_FLAG: u16 = 72;
pub const HID_USAGE_BATTERY_SYSTEM_CONNECTION_TO_SM_BUS: u16 = 21;
pub const HID_USAGE_BATTERY_SYSTEM_CURRENT_NOT_REGULATED: u16 = 218;
pub const HID_USAGE_BATTERY_SYSTEM_CURRENT_OUT_OF_RANGE: u16 = 217;
pub const HID_USAGE_BATTERY_SYSTEM_CYCLE_COUNT: u16 = 107;
pub const HID_USAGE_BATTERY_SYSTEM_DESIGN_CAPACITY: u16 = 131;
pub const HID_USAGE_BATTERY_SYSTEM_DISCHARGING: u16 = 69;
pub const HID_USAGE_BATTERY_SYSTEM_ENABLE_POLLING: u16 = 193;
pub const HID_USAGE_BATTERY_SYSTEM_FULLY_CHARGED: u16 = 70;
pub const HID_USAGE_BATTERY_SYSTEM_FULLY_DISCHARGED: u16 = 71;
pub const HID_USAGE_BATTERY_SYSTEM_FULL_CHARGE_CAPACITY: u16 = 103;
pub const HID_USAGE_BATTERY_SYSTEM_IDEVICE_CHEMISTRY: u16 = 137;
pub const HID_USAGE_BATTERY_SYSTEM_IDEVICE_NAME: u16 = 136;
pub const HID_USAGE_BATTERY_SYSTEM_IMANUFACTURER_NAME: u16 = 135;
pub const HID_USAGE_BATTERY_SYSTEM_INHIBIT_CHARGE: u16 = 192;
pub const HID_USAGE_BATTERY_SYSTEM_INTERNAL_CHARGE_CONTROLLER: u16 = 129;
pub const HID_USAGE_BATTERY_SYSTEM_IOEM_INFORMATION: u16 = 143;
pub const HID_USAGE_BATTERY_SYSTEM_LEVEL_2: u16 = 242;
pub const HID_USAGE_BATTERY_SYSTEM_LEVEL_3: u16 = 243;
pub const HID_USAGE_BATTERY_SYSTEM_MANUFACTURER_ACCESS: u16 = 40;
pub const HID_USAGE_BATTERY_SYSTEM_MANUFACTURER_DATA: u16 = 138;
pub const HID_USAGE_BATTERY_SYSTEM_MANUFACTURE_DATE: u16 = 133;
pub const HID_USAGE_BATTERY_SYSTEM_MASTER_MODE: u16 = 220;
pub const HID_USAGE_BATTERY_SYSTEM_MAX_ERROR: u16 = 99;
pub const HID_USAGE_BATTERY_SYSTEM_NEED_REPLACEMENT: u16 = 75;
pub const HID_USAGE_BATTERY_SYSTEM_OK_TO_USE: u16 = 26;
pub const HID_USAGE_BATTERY_SYSTEM_OPTIONAL_MFG_FUNCTION_1: u16 = 16;
pub const HID_USAGE_BATTERY_SYSTEM_OPTIONAL_MFG_FUNCTION_2: u16 = 17;
pub const HID_USAGE_BATTERY_SYSTEM_OPTIONAL_MFG_FUNCTION_3: u16 = 18;
pub const HID_USAGE_BATTERY_SYSTEM_OPTIONAL_MFG_FUNCTION_4: u16 = 19;
pub const HID_USAGE_BATTERY_SYSTEM_OPTIONAL_MFG_FUNCTION_5: u16 = 20;
pub const HID_USAGE_BATTERY_SYSTEM_OUTPUT_CONNECTION: u16 = 22;
pub const HID_USAGE_BATTERY_SYSTEM_POWER_FAIL: u16 = 210;
pub const HID_USAGE_BATTERY_SYSTEM_PRIMARY_BATTERY: u16 = 46;
pub const HID_USAGE_BATTERY_SYSTEM_PRIMARY_BATTERY_SUPPORT: u16 = 130;
pub const HID_USAGE_BATTERY_SYSTEM_RECHARGABLE: u16 = 139;
pub const HID_USAGE_BATTERY_SYSTEM_RELATIVE_STATE_OF_CHARGE: u16 = 100;
pub const HID_USAGE_BATTERY_SYSTEM_REMAINING_CAPACITY: u16 = 102;
pub const HID_USAGE_BATTERY_SYSTEM_REMAINING_CAPACITY_LIMIT: u16 = 41;
pub const HID_USAGE_BATTERY_SYSTEM_REMAINING_TIME_LIMIT: u16 = 42;
pub const HID_USAGE_BATTERY_SYSTEM_REMAINING_TIME_LIMIT_EXPIRED: u16 = 67;
pub const HID_USAGE_BATTERY_SYSTEM_RESET_TO_ZERO: u16 = 194;
pub const HID_USAGE_BATTERY_SYSTEM_RUN_TIME_TO_EMPTY: u16 = 104;
pub const HID_USAGE_BATTERY_SYSTEM_SELECTOR_REVISION: u16 = 28;
pub const HID_USAGE_BATTERY_SYSTEM_SERIAL_NUMBER: u16 = 134;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_ALARM_WARNING: u16 = 3;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_BATTERY_MODE: u16 = 1;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_BATTERY_STATUS: u16 = 2;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_CHARGER_MODE: u16 = 4;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_CHARGER_SPEC_INFO: u16 = 6;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_CHARGER_STATUS: u16 = 5;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_ERROR_CODE: u16 = 74;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_SELECTOR_INFO: u16 = 9;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_SELECTOR_PRESETS: u16 = 8;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_SELECTOR_STATE: u16 = 7;
pub const HID_USAGE_BATTERY_SYSTEM_SPECIFICATION_INFO: u16 = 132;
pub const HID_USAGE_BATTERY_SYSTEM_TERMINATE_CHARGE: u16 = 64;
pub const HID_USAGE_BATTERY_SYSTEM_TERMINATE_DISCHARGE: u16 = 65;
pub const HID_USAGE_BATTERY_SYSTEM_THERMISTOR_COLD: u16 = 214;
pub const HID_USAGE_BATTERY_SYSTEM_THERMISTOR_HOT: u16 = 213;
pub const HID_USAGE_BATTERY_SYSTEM_THERMISTOR_OVER_RANGE: u16 = 215;
pub const HID_USAGE_BATTERY_SYSTEM_THERMISTOR_UNDER_RANGE: u16 = 212;
pub const HID_USAGE_BATTERY_SYSTEM_USE_NEXT: u16 = 25;
pub const HID_USAGE_BATTERY_SYSTEM_VOLTAGE_NOT_REGULATED: u16 = 219;
pub const HID_USAGE_BATTERY_SYSTEM_VOLTAGE_OUT_OF_RANGE: u16 = 216;
pub const HID_USAGE_BATTERY_SYSTEM_WARNING_CAPACITY_LIMIT: u16 = 140;
pub const HID_USAGE_BRAILLE_DISPLAY_6_DOT_BRAILLE_CELL: u16 = 4;
pub const HID_USAGE_BRAILLE_DISPLAY_8_DOT_BRAILLE_CELL: u16 = 3;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_BUTTONS: u16 = 512;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_DISPLAY: u16 = 1;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_DPAD_CENTER: u16 = 533;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_DPAD_DOWN: u16 = 535;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_DPAD_LEFT: u16 = 536;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_DPAD_RIGHT: u16 = 537;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_DPAD_UP: u16 = 534;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_FACE_CONTROLS: u16 = 524;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_JOYSTICK_CENTER: u16 = 528;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_JOYSTICK_DOWN: u16 = 530;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_JOYSTICK_LEFT: u16 = 531;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_JOYSTICK_RIGHT: u16 = 532;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_JOYSTICK_UP: u16 = 529;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_DOT_1: u16 = 513;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_DOT_2: u16 = 514;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_DOT_3: u16 = 515;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_DOT_4: u16 = 516;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_DOT_5: u16 = 517;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_DOT_6: u16 = 518;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_DOT_7: u16 = 519;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_DOT_8: u16 = 520;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_LEFT_SPACE: u16 = 522;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_RIGHT_SPACE: u16 = 523;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_SPACE: u16 = 521;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_LEFT_CONTROLS: u16 = 525;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_PAN_LEFT: u16 = 538;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_PAN_RIGHT: u16 = 539;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_RIGHT_CONTROLS: u16 = 526;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_ROCKER_DOWN: u16 = 541;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_ROCKER_PRESS: u16 = 542;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_ROCKER_UP: u16 = 540;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_ROW: u16 = 2;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_TOP_CONTROLS: u16 = 527;
pub const HID_USAGE_BRAILLE_DISPLAY_NUMBER_OF_BRAILLE_CELLS: u16 = 5;
pub const HID_USAGE_BRAILLE_DISPLAY_ROUTER_KEY: u16 = 256;
pub const HID_USAGE_BRAILLE_DISPLAY_ROUTER_SET_1: u16 = 250;
pub const HID_USAGE_BRAILLE_DISPLAY_ROUTER_SET_2: u16 = 251;
pub const HID_USAGE_BRAILLE_DISPLAY_ROUTER_SET_3: u16 = 252;
pub const HID_USAGE_BRAILLE_DISPLAY_ROW_ROUTER_KEY: u16 = 257;
pub const HID_USAGE_BRAILLE_DISPLAY_SCREEN_READER_CONTROL: u16 = 6;
pub const HID_USAGE_BRAILLE_DISPLAY_SCREEN_READER_IDENTIFIER: u16 = 7;
pub const HID_USAGE_CAMERA_AUTO_FOCUS: u16 = 32;
pub const HID_USAGE_CAMERA_SHUTTER: u16 = 33;
pub const HID_USAGE_CONSUMERCTRL: u16 = 1;
pub const HID_USAGE_CONSUMER_10: u16 = 32;
pub const HID_USAGE_CONSUMER_100: u16 = 33;
pub const HID_USAGE_CONSUMER_3D_MODE_SELECT: u16 = 110;
pub const HID_USAGE_CONSUMER_AC_ADD_TO_CART: u16 = 610;
pub const HID_USAGE_CONSUMER_AC_ALL_CAPS: u16 = 580;
pub const HID_USAGE_CONSUMER_AC_ATTACH_COMMENT: u16 = 623;
pub const HID_USAGE_CONSUMER_AC_ATTACH_FILE: u16 = 653;
pub const HID_USAGE_CONSUMER_AC_BACK: u16 = 548;
pub const HID_USAGE_CONSUMER_AC_BOLD: u16 = 574;
pub const HID_USAGE_CONSUMER_AC_BOOKMARKS: u16 = 554;
pub const HID_USAGE_CONSUMER_AC_BULLETED_LIST: u16 = 602;
pub const HID_USAGE_CONSUMER_AC_BUYCHECKOUT: u16 = 609;
pub const HID_USAGE_CONSUMER_AC_CANCEL: u16 = 607;
pub const HID_USAGE_CONSUMER_AC_CATALOG: u16 = 608;
pub const HID_USAGE_CONSUMER_AC_CLEAR_ALARM: u16 = 643;
pub const HID_USAGE_CONSUMER_AC_CLOSE: u16 = 515;
pub const HID_USAGE_CONSUMER_AC_COLLAPSE: u16 = 613;
pub const HID_USAGE_CONSUMER_AC_COLLAPSE_ALL: u16 = 614;
pub const HID_USAGE_CONSUMER_AC_COPY: u16 = 539;
pub const HID_USAGE_CONSUMER_AC_CUT: u16 = 540;
pub const HID_USAGE_CONSUMER_AC_DELETE: u16 = 618;
pub const HID_USAGE_CONSUMER_AC_DELETE_COMMENT: u16 = 624;
pub const HID_USAGE_CONSUMER_AC_DEMOTE: u16 = 604;
pub const HID_USAGE_CONSUMER_AC_DESKTOP_SHOW_ALL_APPLICATIONS: u16 = 674;
pub const HID_USAGE_CONSUMER_AC_DESKTOP_SHOW_ALL_WINDOWS: u16 = 671;
pub const HID_USAGE_CONSUMER_AC_DISRIBUTE_HORIZONTALLY: u16 = 667;
pub const HID_USAGE_CONSUMER_AC_DISTRIBUTE_VERTICALLY: u16 = 668;
pub const HID_USAGE_CONSUMER_AC_DOWNLOAD_SAVE_TARGET_AS: u16 = 655;
pub const HID_USAGE_CONSUMER_AC_EDIT: u16 = 573;
pub const HID_USAGE_CONSUMER_AC_EDIT_TIME_ZONES: u16 = 641;
pub const HID_USAGE_CONSUMER_AC_EXIT: u16 = 516;
pub const HID_USAGE_CONSUMER_AC_EXPAND: u16 = 611;
pub const HID_USAGE_CONSUMER_AC_EXPAND_ALL: u16 = 612;
pub const HID_USAGE_CONSUMER_AC_FILTER: u16 = 637;
pub const HID_USAGE_CONSUMER_AC_FIND: u16 = 543;
pub const HID_USAGE_CONSUMER_AC_FIND_AND_REPLACE: u16 = 544;
pub const HID_USAGE_CONSUMER_AC_FLIP_HORIZONTAL: u16 = 583;
pub const HID_USAGE_CONSUMER_AC_FLIP_VERTICAL: u16 = 584;
pub const HID_USAGE_CONSUMER_AC_FONT_COLOR: u16 = 588;
pub const HID_USAGE_CONSUMER_AC_FONT_SELECT: u16 = 587;
pub const HID_USAGE_CONSUMER_AC_FONT_SIZE: u16 = 589;
pub const HID_USAGE_CONSUMER_AC_FORMAT: u16 = 572;
pub const HID_USAGE_CONSUMER_AC_FORWARD: u16 = 549;
pub const HID_USAGE_CONSUMER_AC_FORWARD_MSG: u16 = 651;
pub const HID_USAGE_CONSUMER_AC_FULL_SCREEN_VIEW: u16 = 560;
pub const HID_USAGE_CONSUMER_AC_GOTO: u16 = 546;
pub const HID_USAGE_CONSUMER_AC_HISTORY: u16 = 555;
pub const HID_USAGE_CONSUMER_AC_HOME: u16 = 547;
pub const HID_USAGE_CONSUMER_AC_IDLE_KEEP_ALIVE: u16 = 688;
pub const HID_USAGE_CONSUMER_AC_INDENT_DECREASE: u16 = 598;
pub const HID_USAGE_CONSUMER_AC_INDENT_INCREASE: u16 = 599;
pub const HID_USAGE_CONSUMER_AC_INSERT_COLUMN: u16 = 658;
pub const HID_USAGE_CONSUMER_AC_INSERT_FILE: u16 = 659;
pub const HID_USAGE_CONSUMER_AC_INSERT_MODE: u16 = 617;
pub const HID_USAGE_CONSUMER_AC_INSERT_OBJECT: u16 = 661;
pub const HID_USAGE_CONSUMER_AC_INSERT_PICTURE: u16 = 660;
pub const HID_USAGE_CONSUMER_AC_INSERT_ROW: u16 = 657;
pub const HID_USAGE_CONSUMER_AC_INSERT_SYMBOL: u16 = 662;
pub const HID_USAGE_CONSUMER_AC_ITALICS: u16 = 575;
pub const HID_USAGE_CONSUMER_AC_JUSTIFY_BLOCK_H: u16 = 593;
pub const HID_USAGE_CONSUMER_AC_JUSTIFY_BLOCK_V: u16 = 597;
pub const HID_USAGE_CONSUMER_AC_JUSTIFY_BOTTOM: u16 = 596;
pub const HID_USAGE_CONSUMER_AC_JUSTIFY_CENTER_H: u16 = 591;
pub const HID_USAGE_CONSUMER_AC_JUSTIFY_CENTER_V: u16 = 595;
pub const HID_USAGE_CONSUMER_AC_JUSTIFY_LEFT: u16 = 590;
pub const HID_USAGE_CONSUMER_AC_JUSTIFY_RIGHT: u16 = 592;
pub const HID_USAGE_CONSUMER_AC_JUSTIFY_TOP: u16 = 594;
pub const HID_USAGE_CONSUMER_AC_LOCK: u16 = 619;
pub const HID_USAGE_CONSUMER_AC_MAXIMIZE: u16 = 517;
pub const HID_USAGE_CONSUMER_AC_MERGE: u16 = 665;
pub const HID_USAGE_CONSUMER_AC_MINIMIZE: u16 = 518;
pub const HID_USAGE_CONSUMER_AC_MIRROR_HORIZONTAL: u16 = 585;
pub const HID_USAGE_CONSUMER_AC_MIRROR_VERTICAL: u16 = 586;
pub const HID_USAGE_CONSUMER_AC_NAVIGATION_GUIDANCE: u16 = 670;
pub const HID_USAGE_CONSUMER_AC_NEW: u16 = 513;
pub const HID_USAGE_CONSUMER_AC_NEW_WINDOW: u16 = 569;
pub const HID_USAGE_CONSUMER_AC_NEXT: u16 = 553;
pub const HID_USAGE_CONSUMER_AC_NEXT_KEYBOARD_LAYOUT_SELECT: u16 = 669;
pub const HID_USAGE_CONSUMER_AC_NO: u16 = 606;
pub const HID_USAGE_CONSUMER_AC_NORMAL_VIEW: u16 = 561;
pub const HID_USAGE_CONSUMER_AC_NUMBERED_LIST: u16 = 600;
pub const HID_USAGE_CONSUMER_AC_OPEN: u16 = 514;
pub const HID_USAGE_CONSUMER_AC_PAN: u16 = 568;
pub const HID_USAGE_CONSUMER_AC_PAN_LEFT: u16 = 566;
pub const HID_USAGE_CONSUMER_AC_PAN_RIGHT: u16 = 567;
pub const HID_USAGE_CONSUMER_AC_PASTE: u16 = 541;
pub const HID_USAGE_CONSUMER_AC_PASTE_SPECIAL: u16 = 616;
pub const HID_USAGE_CONSUMER_AC_PREVIOUS: u16 = 552;
pub const HID_USAGE_CONSUMER_AC_PRINT: u16 = 520;
pub const HID_USAGE_CONSUMER_AC_PRINT_PREVIEW: u16 = 615;
pub const HID_USAGE_CONSUMER_AC_PROMOTE: u16 = 603;
pub const HID_USAGE_CONSUMER_AC_PROPERTIES: u16 = 521;
pub const HID_USAGE_CONSUMER_AC_PROTECT: u16 = 621;
pub const HID_USAGE_CONSUMER_AC_REDOREPEAT: u16 = 633;
pub const HID_USAGE_CONSUMER_AC_REFRESH: u16 = 551;
pub const HID_USAGE_CONSUMER_AC_RENAME: u16 = 664;
pub const HID_USAGE_CONSUMER_AC_REPLY: u16 = 649;
pub const HID_USAGE_CONSUMER_AC_REPLY_ALL: u16 = 650;
pub const HID_USAGE_CONSUMER_AC_RESET_ALARM: u16 = 645;
pub const HID_USAGE_CONSUMER_AC_RESIZE: u16 = 582;
pub const HID_USAGE_CONSUMER_AC_RESTART_NUMBERING: u16 = 601;
pub const HID_USAGE_CONSUMER_AC_ROTATE: u16 = 581;
pub const HID_USAGE_CONSUMER_AC_SAVE: u16 = 519;
pub const HID_USAGE_CONSUMER_AC_SAVE_AND_CLOSE: u16 = 663;
pub const HID_USAGE_CONSUMER_AC_SCROLL: u16 = 565;
pub const HID_USAGE_CONSUMER_AC_SCROLL_DOWN: u16 = 564;
pub const HID_USAGE_CONSUMER_AC_SCROLL_UP: u16 = 563;
pub const HID_USAGE_CONSUMER_AC_SEARCH: u16 = 545;
pub const HID_USAGE_CONSUMER_AC_SELECT_ALL: u16 = 542;
pub const HID_USAGE_CONSUMER_AC_SELECT_COLUMN: u16 = 629;
pub const HID_USAGE_CONSUMER_AC_SELECT_OBJECT: u16 = 632;
pub const HID_USAGE_CONSUMER_AC_SELECT_PARAGRAPH: u16 = 628;
pub const HID_USAGE_CONSUMER_AC_SELECT_ROW: u16 = 630;
pub const HID_USAGE_CONSUMER_AC_SELECT_SENTENCE: u16 = 627;
pub const HID_USAGE_CONSUMER_AC_SELECT_TABLE: u16 = 631;
pub const HID_USAGE_CONSUMER_AC_SELECT_TIME_ZONE: u16 = 640;
pub const HID_USAGE_CONSUMER_AC_SELECT_WORD: u16 = 626;
pub const HID_USAGE_CONSUMER_AC_SEND: u16 = 652;
pub const HID_USAGE_CONSUMER_AC_SENDRECEIVE: u16 = 647;
pub const HID_USAGE_CONSUMER_AC_SEND_TO: u16 = 648;
pub const HID_USAGE_CONSUMER_AC_SET_ALARM: u16 = 642;
pub const HID_USAGE_CONSUMER_AC_SET_BORDERS: u16 = 656;
pub const HID_USAGE_CONSUMER_AC_SET_CLOCK: u16 = 638;
pub const HID_USAGE_CONSUMER_AC_SNOOZE_ALARM: u16 = 644;
pub const HID_USAGE_CONSUMER_AC_SOFT_KEY_LEFT: u16 = 672;
pub const HID_USAGE_CONSUMER_AC_SOFT_KEY_RIGHT: u16 = 673;
pub const HID_USAGE_CONSUMER_AC_SORT: u16 = 634;
pub const HID_USAGE_CONSUMER_AC_SORT_ASCENDING: u16 = 635;
pub const HID_USAGE_CONSUMER_AC_SORT_DESCENDING: u16 = 636;
pub const HID_USAGE_CONSUMER_AC_SPLIT: u16 = 666;
pub const HID_USAGE_CONSUMER_AC_STOP: u16 = 550;
pub const HID_USAGE_CONSUMER_AC_STRIKETHROUGH: u16 = 577;
pub const HID_USAGE_CONSUMER_AC_SUBSCRIPT: u16 = 578;
pub const HID_USAGE_CONSUMER_AC_SUBSCRIPTIONS: u16 = 556;
pub const HID_USAGE_CONSUMER_AC_SUPERSCRIPT: u16 = 579;
pub const HID_USAGE_CONSUMER_AC_SYNCHRONIZE: u16 = 646;
pub const HID_USAGE_CONSUMER_AC_TILE_HORIZONTALLY: u16 = 570;
pub const HID_USAGE_CONSUMER_AC_TILE_VERTICALLY: u16 = 571;
pub const HID_USAGE_CONSUMER_AC_UNDERLINE: u16 = 576;
pub const HID_USAGE_CONSUMER_AC_UNDO: u16 = 538;
pub const HID_USAGE_CONSUMER_AC_UNLOCK: u16 = 620;
pub const HID_USAGE_CONSUMER_AC_UNPROTECT: u16 = 622;
pub const HID_USAGE_CONSUMER_AC_UPLOAD: u16 = 654;
pub const HID_USAGE_CONSUMER_AC_VIEW_CLOCK: u16 = 639;
pub const HID_USAGE_CONSUMER_AC_VIEW_COMMENT: u16 = 625;
pub const HID_USAGE_CONSUMER_AC_VIEW_TOGGLE: u16 = 562;
pub const HID_USAGE_CONSUMER_AC_YES: u16 = 605;
pub const HID_USAGE_CONSUMER_AC_ZOOM: u16 = 559;
pub const HID_USAGE_CONSUMER_AC_ZOOM_IN: u16 = 557;
pub const HID_USAGE_CONSUMER_AC_ZOOM_OUT: u16 = 558;
pub const HID_USAGE_CONSUMER_ALTERNATE_AUDIO_DECREMENT: u16 = 372;
pub const HID_USAGE_CONSUMER_ALTERNATE_AUDIO_INCREMENT: u16 = 371;
pub const HID_USAGE_CONSUMER_AL_ALARMS: u16 = 434;
pub const HID_USAGE_CONSUMER_AL_AUDIO_BROWSER: u16 = 439;
pub const HID_USAGE_CONSUMER_AL_AUDIO_PLAYER: u16 = 455;
pub const HID_USAGE_CONSUMER_AL_AV_CAPTUREPLAYBACK: u16 = 403;
pub const HID_USAGE_CONSUMER_AL_BROWSER: u16 = 404;
pub const HID_USAGE_CONSUMER_AL_CALCULATOR: u16 = 402;
pub const HID_USAGE_CONSUMER_AL_CALENDARSCHEDULE: u16 = 398;
pub const HID_USAGE_CONSUMER_AL_CHECKBOOKFINANCE: u16 = 401;
pub const HID_USAGE_CONSUMER_AL_CLOCK: u16 = 435;
pub const HID_USAGE_CONSUMER_AL_COMMAND_LINE_PROCESSORRUN: u16 = 416;
pub const HID_USAGE_CONSUMER_AL_CONFIGURATION: u16 = 387;
pub const HID_USAGE_CONSUMER_AL_CONTACTSADDRESS_BOOK: u16 = 397;
pub const HID_USAGE_CONSUMER_AL_CONTACT_SYNC: u16 = 457;
pub const HID_USAGE_CONSUMER_AL_CONTEXTAWARE_DESKTOP_ASSISTANT: u16 = 459;
pub const HID_USAGE_CONSUMER_AL_CONTROL_PANEL: u16 = 415;
pub const HID_USAGE_CONSUMER_AL_CUSTOMIZED_CORPORATE_NEWS_BROWSER: u16 = 452;
pub const HID_USAGE_CONSUMER_AL_DATABASE_APP: u16 = 393;
pub const HID_USAGE_CONSUMER_AL_DESKTOP: u16 = 426;
pub const HID_USAGE_CONSUMER_AL_DICTIONARY: u16 = 425;
pub const HID_USAGE_CONSUMER_AL_DIGITAL_RIGHTS_MANAGER: u16 = 441;
pub const HID_USAGE_CONSUMER_AL_DIGITAL_WALLET: u16 = 442;
pub const HID_USAGE_CONSUMER_AL_DOCUMENTS: u16 = 423;
pub const HID_USAGE_CONSUMER_AL_EMAIL: u16 = 394;
pub const HID_USAGE_CONSUMER_AL_ENCRYPTION: u16 = 432;
pub const HID_USAGE_CONSUMER_AL_ENTERTAINMENT_CONTENT_BROWSER: u16 = 448;
pub const HID_USAGE_CONSUMER_AL_FILE_BROWSER: u16 = 436;
pub const HID_USAGE_CONSUMER_AL_GRAMMAR_CHECK: u16 = 428;
pub const HID_USAGE_CONSUMER_AL_GRAPHICS_EDITOR: u16 = 391;
pub const HID_USAGE_CONSUMER_AL_IMAGE_BROWSER: u16 = 438;
pub const HID_USAGE_CONSUMER_AL_INSTANT_MESSAGING: u16 = 444;
pub const HID_USAGE_CONSUMER_AL_INTEGRATED_HELP_CENTER: u16 = 422;
pub const HID_USAGE_CONSUMER_AL_INTERNET_BROWSER: u16 = 406;
pub const HID_USAGE_CONSUMER_AL_KEYBOARD_LAYOUT: u16 = 430;
pub const HID_USAGE_CONSUMER_AL_LANWAN_BROWSER: u16 = 405;
pub const HID_USAGE_CONSUMER_AL_LAUNCH_BUTTON_CONFIGURATION_TOOL: u16 = 385;
pub const HID_USAGE_CONSUMER_AL_LOGJOURNALTIMECARD: u16 = 400;
pub const HID_USAGE_CONSUMER_AL_LOGOFF: u16 = 412;
pub const HID_USAGE_CONSUMER_AL_LOGON: u16 = 411;
pub const HID_USAGE_CONSUMER_AL_LOGONLOGOFF: u16 = 413;
pub const HID_USAGE_CONSUMER_AL_MARKET_MONITORFINANCE_BROWSER: u16 = 451;
pub const HID_USAGE_CONSUMER_AL_MESSAGE_STATUS: u16 = 456;
pub const HID_USAGE_CONSUMER_AL_MOVIE_BROWSER: u16 = 440;
pub const HID_USAGE_CONSUMER_AL_NAVIGATION: u16 = 458;
pub const HID_USAGE_CONSUMER_AL_NETWORK_CHAT: u16 = 409;
pub const HID_USAGE_CONSUMER_AL_NETWORK_CONFERENCE: u16 = 408;
pub const HID_USAGE_CONSUMER_AL_NEWSREADER: u16 = 395;
pub const HID_USAGE_CONSUMER_AL_NEXT_TASKAPPLICATION: u16 = 419;
pub const HID_USAGE_CONSUMER_AL_OEM_FEATURES_TIPSTUTORIAL_BROWSER: u16 = 445;
pub const HID_USAGE_CONSUMER_AL_OEM_HELP: u16 = 446;
pub const HID_USAGE_CONSUMER_AL_ONLINE_ACTIVITY_BROWSER: u16 = 453;
pub const HID_USAGE_CONSUMER_AL_ONLINE_COMMUNITY: u16 = 447;
pub const HID_USAGE_CONSUMER_AL_ONLINE_SHOPPING_BROWSER: u16 = 449;
pub const HID_USAGE_CONSUMER_AL_POWER_STATUS: u16 = 437;
pub const HID_USAGE_CONSUMER_AL_PREEMPTIVE_HALT_TASKAPPLICATION: u16 = 421;
pub const HID_USAGE_CONSUMER_AL_PRESENTATION_APP: u16 = 392;
pub const HID_USAGE_CONSUMER_AL_PREVIOUS_TASKAPPLICATION: u16 = 420;
pub const HID_USAGE_CONSUMER_AL_PROCESSTASK_MANAGER: u16 = 417;
pub const HID_USAGE_CONSUMER_AL_PROGRAMMABLE_BUTTON_CONFIGURATION: u16 = 386;
pub const HID_USAGE_CONSUMER_AL_REMOTE_NETWORKINGISP_CONNECT: u16 = 407;
pub const HID_USAGE_CONSUMER_AL_SCREEN_SAVER: u16 = 433;
pub const HID_USAGE_CONSUMER_AL_SEARCH: u16 = 454;
pub const HID_USAGE_CONSUMER_AL_SELECT_TASKAPPLICATION: u16 = 418;
pub const HID_USAGE_CONSUMER_AL_SMARTCARD_INFORMATIONHELP: u16 = 450;
pub const HID_USAGE_CONSUMER_AL_SPELL_CHECK: u16 = 427;
pub const HID_USAGE_CONSUMER_AL_SPREADSHEET: u16 = 390;
pub const HID_USAGE_CONSUMER_AL_TASKPROJECT_MANAGER: u16 = 399;
pub const HID_USAGE_CONSUMER_AL_TELEPHONYDIALER: u16 = 410;
pub const HID_USAGE_CONSUMER_AL_TERMINAL_LOCKSCREENSAVER: u16 = 414;
pub const HID_USAGE_CONSUMER_AL_TEXT_EDITOR: u16 = 389;
pub const HID_USAGE_CONSUMER_AL_THESAURUS: u16 = 424;
pub const HID_USAGE_CONSUMER_AL_VIRUS_PROTECTION: u16 = 431;
pub const HID_USAGE_CONSUMER_AL_VOICEMAIL: u16 = 396;
pub const HID_USAGE_CONSUMER_AL_WIRELESS_STATUS: u16 = 429;
pub const HID_USAGE_CONSUMER_AL_WORD_PROCESSOR: u16 = 388;
pub const HID_USAGE_CONSUMER_AMPM: u16 = 34;
pub const HID_USAGE_CONSUMER_APPLICATION_LAUNCH_BUTTONS: u16 = 384;
pub const HID_USAGE_CONSUMER_ASPECT: u16 = 109;
pub const HID_USAGE_CONSUMER_ASSIGN_SELECTION: u16 = 129;
pub const HID_USAGE_CONSUMER_BALANCE: u16 = 225;
pub const HID_USAGE_CONSUMER_BALANCE_LEFT: u16 = 337;
pub const HID_USAGE_CONSUMER_BALANCE_RIGHT: u16 = 336;
pub const HID_USAGE_CONSUMER_BASS: u16 = 227;
pub const HID_USAGE_CONSUMER_BASS_BOOST: u16 = 229;
pub const HID_USAGE_CONSUMER_BASS_DECREMENT: u16 = 339;
pub const HID_USAGE_CONSUMER_BASS_INCREMENT: u16 = 338;
pub const HID_USAGE_CONSUMER_BLUE_MENU_BUTTON: u16 = 107;
pub const HID_USAGE_CONSUMER_BROADCAST_MODE: u16 = 100;
pub const HID_USAGE_CONSUMER_CAMERA_ACCESS_DISABLED: u16 = 119;
pub const HID_USAGE_CONSUMER_CAMERA_ACCESS_ENABLED: u16 = 118;
pub const HID_USAGE_CONSUMER_CAMERA_ACCESS_TOGGLE: u16 = 120;
pub const HID_USAGE_CONSUMER_CHANNEL: u16 = 134;
pub const HID_USAGE_CONSUMER_CHANNEL_CENTER: u16 = 355;
pub const HID_USAGE_CONSUMER_CHANNEL_CENTER_FRONT: u16 = 357;
pub const HID_USAGE_CONSUMER_CHANNEL_DECREMENT: u16 = 157;
pub const HID_USAGE_CONSUMER_CHANNEL_FRONT: u16 = 356;
pub const HID_USAGE_CONSUMER_CHANNEL_INCREMENT: u16 = 156;
pub const HID_USAGE_CONSUMER_CHANNEL_LEFT: u16 = 353;
pub const HID_USAGE_CONSUMER_CHANNEL_LOW_FREQUENCY_ENHANCEMENT: u16 = 360;
pub const HID_USAGE_CONSUMER_CHANNEL_RIGHT: u16 = 354;
pub const HID_USAGE_CONSUMER_CHANNEL_SIDE: u16 = 358;
pub const HID_USAGE_CONSUMER_CHANNEL_SURROUND: u16 = 359;
pub const HID_USAGE_CONSUMER_CHANNEL_TOP: u16 = 361;
pub const HID_USAGE_CONSUMER_CHANNEL_UNKNOWN: u16 = 362;
pub const HID_USAGE_CONSUMER_CLEAR_MARK: u16 = 195;
pub const HID_USAGE_CONSUMER_CLIMATE_CONTROL_ENABLE: u16 = 260;
pub const HID_USAGE_CONSUMER_CLOSED_CAPTION: u16 = 97;
pub const HID_USAGE_CONSUMER_CLOSED_CAPTION_SELECT: u16 = 98;
pub const HID_USAGE_CONSUMER_CONTACT_ADDED: u16 = 1281;
pub const HID_USAGE_CONSUMER_CONTACT_EDITED: u16 = 1280;
pub const HID_USAGE_CONSUMER_CONTACT_EMAIL_BUSINESS: u16 = 1295;
pub const HID_USAGE_CONSUMER_CONTACT_EMAIL_MAIN: u16 = 1297;
pub const HID_USAGE_CONSUMER_CONTACT_EMAIL_OTHER: u16 = 1296;
pub const HID_USAGE_CONSUMER_CONTACT_EMAIL_PERSONAL: u16 = 1294;
pub const HID_USAGE_CONSUMER_CONTACT_FIRST_NAME: u16 = 1285;
pub const HID_USAGE_CONSUMER_CONTACT_FULL_NAME: u16 = 1287;
pub const HID_USAGE_CONSUMER_CONTACT_INDEX: u16 = 1283;
pub const HID_USAGE_CONSUMER_CONTACT_LAST_NAME: u16 = 1286;
pub const HID_USAGE_CONSUMER_CONTACT_MISC: u16 = 1300;
pub const HID_USAGE_CONSUMER_CONTACT_NICKNAME: u16 = 1284;
pub const HID_USAGE_CONSUMER_CONTACT_PHONE_NUMBER_BUSINESS: u16 = 1289;
pub const HID_USAGE_CONSUMER_CONTACT_PHONE_NUMBER_FAX: u16 = 1292;
pub const HID_USAGE_CONSUMER_CONTACT_PHONE_NUMBER_MOBILE: u16 = 1290;
pub const HID_USAGE_CONSUMER_CONTACT_PHONE_NUMBER_OTHER: u16 = 1293;
pub const HID_USAGE_CONSUMER_CONTACT_PHONE_NUMBER_PAGER: u16 = 1291;
pub const HID_USAGE_CONSUMER_CONTACT_PHONE_NUMBER_PERSONAL: u16 = 1288;
pub const HID_USAGE_CONSUMER_CONTACT_RECORD_ACTIVE: u16 = 1282;
pub const HID_USAGE_CONSUMER_CONTACT_SPEED_DIAL_NUMBER: u16 = 1298;
pub const HID_USAGE_CONSUMER_CONTACT_STATUS_FLAG: u16 = 1299;
pub const HID_USAGE_CONSUMER_COUNTER_RESET: u16 = 200;
pub const HID_USAGE_CONSUMER_DAILY: u16 = 162;
pub const HID_USAGE_CONSUMER_DATA_ON_SCREEN: u16 = 96;
pub const HID_USAGE_CONSUMER_DISPLAY_BACKLIGHT_TOGGLE: u16 = 114;
pub const HID_USAGE_CONSUMER_DISPLAY_BRIGHTNESS: u16 = 113;
pub const HID_USAGE_CONSUMER_DISPLAY_BRIGHTNESS_DECREMENT: u16 = 112;
pub const HID_USAGE_CONSUMER_DISPLAY_BRIGHTNESS_INCREMENT: u16 = 111;
pub const HID_USAGE_CONSUMER_DISPLAY_SET_AUTO_BRIGHTNESS: u16 = 117;
pub const HID_USAGE_CONSUMER_DISPLAY_SET_BRIGHTNESS_TO_MAXIMUM: u16 = 116;
pub const HID_USAGE_CONSUMER_DISPLAY_SET_BRIGHTNESS_TO_MINIMUM: u16 = 115;
pub const HID_USAGE_CONSUMER_DURESS_ALARM: u16 = 267;
pub const HID_USAGE_CONSUMER_EJECT: u16 = 184;
pub const HID_USAGE_CONSUMER_ENTER_CHANNEL: u16 = 132;
pub const HID_USAGE_CONSUMER_ENTER_DISC: u16 = 187;
pub const HID_USAGE_CONSUMER_EXTENDED_KEYBOARD_ATTRIBUTES_COLLECTION: u16 = 704;
pub const HID_USAGE_CONSUMER_EXTENDED_PLAY: u16 = 244;
pub const HID_USAGE_CONSUMER_FAN_ENABLE: u16 = 256;
pub const HID_USAGE_CONSUMER_FAN_SPEED: u16 = 257;
pub const HID_USAGE_CONSUMER_FAST_FORWARD: u16 = 179;
pub const HID_USAGE_CONSUMER_FIRE_ALARM: u16 = 263;
pub const HID_USAGE_CONSUMER_FRAME_BACK: u16 = 193;
pub const HID_USAGE_CONSUMER_FRAME_FORWARD: u16 = 192;
pub const HID_USAGE_CONSUMER_FUNCTION_BUTTONS: u16 = 54;
pub const HID_USAGE_CONSUMER_GAMEDVR_OPEN_GAMEBAR: u16 = 208;
pub const HID_USAGE_CONSUMER_GAMEDVR_RECORD_CLIP: u16 = 210;
pub const HID_USAGE_CONSUMER_GAMEDVR_SCREENSHOT: u16 = 211;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_BROADCAST: u16 = 215;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_CAMERA: u16 = 214;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_INDICATOR: u16 = 212;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_MICROPHONE: u16 = 213;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_RECORD: u16 = 209;
pub const HID_USAGE_CONSUMER_GENERIC_GUI_APPLICATION_CONTROLS: u16 = 512;
pub const HID_USAGE_CONSUMER_GRAPHIC_EQUALIZER: u16 = 6;
pub const HID_USAGE_CONSUMER_GREEN_MENU_BUTTON: u16 = 106;
pub const HID_USAGE_CONSUMER_HEADPHONE: u16 = 5;
pub const HID_USAGE_CONSUMER_HELP: u16 = 149;
pub const HID_USAGE_CONSUMER_HOLDUP_ALARM: u16 = 268;
pub const HID_USAGE_CONSUMER_ILLUMINATION: u16 = 53;
pub const HID_USAGE_CONSUMER_IMPLEMENTED_KEYBOARD_INPUT_ASSIST_CONTROLS: u16 = 710;
pub const HID_USAGE_CONSUMER_INVOKEDISMISS_EMOJI_PICKER: u16 = 217;
pub const HID_USAGE_CONSUMER_KEYBOARD_BACKLIGHT: u16 = 7;
pub const HID_USAGE_CONSUMER_KEYBOARD_BACKLIGHT_AUTO: u16 = 127;
pub const HID_USAGE_CONSUMER_KEYBOARD_BACKLIGHT_LEVEL_SUGGESTION: u16 = 1303;
pub const HID_USAGE_CONSUMER_KEYBOARD_BACKLIGHT_OOC: u16 = 124;
pub const HID_USAGE_CONSUMER_KEYBOARD_BACKLIGHT_SET_LEVEL: u16 = 123;
pub const HID_USAGE_CONSUMER_KEYBOARD_BACKLIGHT_SET_MAXIMUM: u16 = 126;
pub const HID_USAGE_CONSUMER_KEYBOARD_BACKLIGHT_SET_MINIMUM: u16 = 125;
pub const HID_USAGE_CONSUMER_KEYBOARD_BRIGHTNESS_DECREMENT: u16 = 122;
pub const HID_USAGE_CONSUMER_KEYBOARD_BRIGHTNESS_INCREMENT: u16 = 121;
pub const HID_USAGE_CONSUMER_KEYBOARD_BRIGHTNESS_NEXT: u16 = 1301;
pub const HID_USAGE_CONSUMER_KEYBOARD_BRIGHTNESS_PREVIOUS: u16 = 1302;
pub const HID_USAGE_CONSUMER_KEYBOARD_FORM_FACTOR: u16 = 705;
pub const HID_USAGE_CONSUMER_KEYBOARD_IETF_LANGUAGE_TAG_INDEX: u16 = 709;
pub const HID_USAGE_CONSUMER_KEYBOARD_INPUT_ASSIST_ACCEPT: u16 = 715;
pub const HID_USAGE_CONSUMER_KEYBOARD_INPUT_ASSIST_CANCEL: u16 = 716;
pub const HID_USAGE_CONSUMER_KEYBOARD_INPUT_ASSIST_NEXT: u16 = 712;
pub const HID_USAGE_CONSUMER_KEYBOARD_INPUT_ASSIST_NEXT_GROUP: u16 = 714;
pub const HID_USAGE_CONSUMER_KEYBOARD_INPUT_ASSIST_PREVIOUS: u16 = 711;
pub const HID_USAGE_CONSUMER_KEYBOARD_INPUT_ASSIST_PREVIOUS_GROUP: u16 = 713;
pub const HID_USAGE_CONSUMER_KEYBOARD_KEY_TYPE: u16 = 706;
pub const HID_USAGE_CONSUMER_KEYBOARD_PHYSICAL_LAYOUT: u16 = 707;
pub const HID_USAGE_CONSUMER_LIGHT_ENABLE: u16 = 258;
pub const HID_USAGE_CONSUMER_LIGHT_ILLUMINATION_LEVEL: u16 = 259;
pub const HID_USAGE_CONSUMER_LONG_PLAY: u16 = 243;
pub const HID_USAGE_CONSUMER_LOUDNESS: u16 = 231;
pub const HID_USAGE_CONSUMER_MARK: u16 = 194;
pub const HID_USAGE_CONSUMER_MEDIA_SELECTION: u16 = 135;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_CABLE: u16 = 151;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_CALL: u16 = 155;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_CD: u16 = 145;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_COMPUTER: u16 = 136;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_DVD: u16 = 139;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_GAMES: u16 = 143;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_HOME: u16 = 154;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_MESSAGES: u16 = 144;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_PROGRAM_GUIDE: u16 = 141;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_SAP: u16 = 158;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_SATELLITE: u16 = 152;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_SECURITY: u16 = 153;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_TAPE: u16 = 150;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_TELEPHONE: u16 = 140;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_TUNER: u16 = 147;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_TV: u16 = 137;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_VCR: u16 = 146;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_VIDEO_PHONE: u16 = 142;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_WWW: u16 = 138;
pub const HID_USAGE_CONSUMER_MEDICAL_ALARM: u16 = 269;
pub const HID_USAGE_CONSUMER_MENU: u16 = 64;
pub const HID_USAGE_CONSUMER_MENU_DOWN: u16 = 67;
pub const HID_USAGE_CONSUMER_MENU_ESCAPE: u16 = 70;
pub const HID_USAGE_CONSUMER_MENU_LEFT: u16 = 68;
pub const HID_USAGE_CONSUMER_MENU_PICK: u16 = 65;
pub const HID_USAGE_CONSUMER_MENU_RIGHT: u16 = 69;
pub const HID_USAGE_CONSUMER_MENU_UP: u16 = 66;
pub const HID_USAGE_CONSUMER_MENU_VALUE_DECREASE: u16 = 72;
pub const HID_USAGE_CONSUMER_MENU_VALUE_INCREASE: u16 = 71;
pub const HID_USAGE_CONSUMER_MICROPHONE: u16 = 4;
pub const HID_USAGE_CONSUMER_MODE_STEP: u16 = 130;
pub const HID_USAGE_CONSUMER_MONTHLY: u16 = 164;
pub const HID_USAGE_CONSUMER_MOTION: u16 = 266;
pub const HID_USAGE_CONSUMER_MPX: u16 = 232;
pub const HID_USAGE_CONSUMER_MUTE: u16 = 226;
pub const HID_USAGE_CONSUMER_NUMERIC_KEY_PAD: u16 = 2;
pub const HID_USAGE_CONSUMER_ONCE: u16 = 161;
pub const HID_USAGE_CONSUMER_ORDER_MOVIE: u16 = 133;
pub const HID_USAGE_CONSUMER_PAUSE: u16 = 177;
pub const HID_USAGE_CONSUMER_PICTUREINPICTURE_SWAP: u16 = 104;
pub const HID_USAGE_CONSUMER_PICTUREINPICTURE_TOGGLE: u16 = 103;
pub const HID_USAGE_CONSUMER_PLAY: u16 = 176;
pub const HID_USAGE_CONSUMER_PLAYBACK_SPEED: u16 = 241;
pub const HID_USAGE_CONSUMER_PLAYSKIP: u16 = 206;
pub const HID_USAGE_CONSUMER_PLAY_PAUSE: u16 = 205;
pub const HID_USAGE_CONSUMER_POLICE_ALARM: u16 = 264;
pub const HID_USAGE_CONSUMER_POWER: u16 = 48;
pub const HID_USAGE_CONSUMER_PRIVACY_SCREEN_LEVEL_DECREMENT: u16 = 721;
pub const HID_USAGE_CONSUMER_PRIVACY_SCREEN_LEVEL_INCREMENT: u16 = 722;
pub const HID_USAGE_CONSUMER_PRIVACY_SCREEN_LEVEL_MAXIMUM: u16 = 724;
pub const HID_USAGE_CONSUMER_PRIVACY_SCREEN_LEVEL_MINIMUM: u16 = 723;
pub const HID_USAGE_CONSUMER_PRIVACY_SCREEN_TOGGLE: u16 = 720;
pub const HID_USAGE_CONSUMER_PROGRAMMABLE_BUTTONS: u16 = 3;
pub const HID_USAGE_CONSUMER_PROXIMITY: u16 = 265;
pub const HID_USAGE_CONSUMER_QUIT: u16 = 148;
pub const HID_USAGE_CONSUMER_RANDOM_PLAY: u16 = 185;
pub const HID_USAGE_CONSUMER_RECALL_LAST: u16 = 131;
pub const HID_USAGE_CONSUMER_RECORD: u16 = 178;
pub const HID_USAGE_CONSUMER_RED_MENU_BUTTON: u16 = 105;
pub const HID_USAGE_CONSUMER_REPEAT: u16 = 188;
pub const HID_USAGE_CONSUMER_REPEAT_FROM_MARK: u16 = 196;
pub const HID_USAGE_CONSUMER_RESET: u16 = 49;
pub const HID_USAGE_CONSUMER_RETURN_TO_MARK: u16 = 197;
pub const HID_USAGE_CONSUMER_REWIND: u16 = 180;
pub const HID_USAGE_CONSUMER_ROOM_TEMPERATURE: u16 = 261;
pub const HID_USAGE_CONSUMER_SCAN_NEXT_TRACK: u16 = 181;
pub const HID_USAGE_CONSUMER_SCAN_PREV_TRACK: u16 = 182;
pub const HID_USAGE_CONSUMER_SEARCH_MARK_BACKWARDS: u16 = 199;
pub const HID_USAGE_CONSUMER_SEARCH_MARK_FORWARD: u16 = 198;
pub const HID_USAGE_CONSUMER_SECURITY_ENABLE: u16 = 262;
pub const HID_USAGE_CONSUMER_SELECTION: u16 = 128;
pub const HID_USAGE_CONSUMER_SELECT_DISC: u16 = 186;
pub const HID_USAGE_CONSUMER_SHOW_COUNTER: u16 = 201;
pub const HID_USAGE_CONSUMER_SLEEP: u16 = 50;
pub const HID_USAGE_CONSUMER_SLEEP_AFTER: u16 = 51;
pub const HID_USAGE_CONSUMER_SLEEP_MODE: u16 = 52;
pub const HID_USAGE_CONSUMER_SLOW: u16 = 245;
pub const HID_USAGE_CONSUMER_SLOW_TRACKING: u16 = 191;
pub const HID_USAGE_CONSUMER_SNAPSHOT: u16 = 101;
pub const HID_USAGE_CONSUMER_SPEAKER_SYSTEM: u16 = 352;
pub const HID_USAGE_CONSUMER_SPEED_SELECT: u16 = 240;
pub const HID_USAGE_CONSUMER_STANDARD_PLAY: u16 = 242;
pub const HID_USAGE_CONSUMER_START_OR_STOP_VOICE_DICTATION_SESSION: u16 = 216;
pub const HID_USAGE_CONSUMER_STILL: u16 = 102;
pub const HID_USAGE_CONSUMER_STOP: u16 = 183;
pub const HID_USAGE_CONSUMER_STOPEJECT: u16 = 204;
pub const HID_USAGE_CONSUMER_SUBCHANNEL: u16 = 368;
pub const HID_USAGE_CONSUMER_SUBCHANNEL_DECREMENT: u16 = 370;
pub const HID_USAGE_CONSUMER_SUBCHANNEL_INCREMENT: u16 = 369;
pub const HID_USAGE_CONSUMER_SURROUND_MODE: u16 = 230;
pub const HID_USAGE_CONSUMER_TRACKING: u16 = 189;
pub const HID_USAGE_CONSUMER_TRACKING_DECREMENT: u16 = 203;
pub const HID_USAGE_CONSUMER_TRACKING_INCREMENT: u16 = 202;
pub const HID_USAGE_CONSUMER_TRACK_NORMAL: u16 = 190;
pub const HID_USAGE_CONSUMER_TREBLE: u16 = 228;
pub const HID_USAGE_CONSUMER_TREBLE_DECREMENT: u16 = 341;
pub const HID_USAGE_CONSUMER_TREBLE_INCREMENT: u16 = 340;
pub const HID_USAGE_CONSUMER_VCRTV: u16 = 99;
pub const HID_USAGE_CONSUMER_VCR_PLUS: u16 = 160;
pub const HID_USAGE_CONSUMER_VENDOR_SPECIFIC_KEYBOARD_PHYSICAL_LAYOUT: u16 = 708;
pub const HID_USAGE_CONSUMER_VOICE_COMMAND: u16 = 207;
pub const HID_USAGE_CONSUMER_VOLUME: u16 = 224;
pub const HID_USAGE_CONSUMER_VOLUME_DECREMENT: u16 = 234;
pub const HID_USAGE_CONSUMER_VOLUME_INCREMENT: u16 = 233;
pub const HID_USAGE_CONSUMER_WEEKLY: u16 = 163;
pub const HID_USAGE_CONSUMER_YELLOW_MENU_BUTTON: u16 = 108;
pub const HID_USAGE_DIGITIZER_3D_DIGITIZER: u16 = 8;
pub const HID_USAGE_DIGITIZER_ALTITUDE: u16 = 64;
pub const HID_USAGE_DIGITIZER_ARMATURE: u16 = 11;
pub const HID_USAGE_DIGITIZER_ARTICULATED_ARM: u16 = 10;
pub const HID_USAGE_DIGITIZER_AZIMUTH: u16 = 63;
pub const HID_USAGE_DIGITIZER_BARREL_PRESSURE: u16 = 49;
pub const HID_USAGE_DIGITIZER_BARREL_SWITCH: u16 = 68;
pub const HID_USAGE_DIGITIZER_BATTERY_STRENGTH: u16 = 59;
pub const HID_USAGE_DIGITIZER_BRUSH: u16 = 118;
pub const HID_USAGE_DIGITIZER_BUTTON_PRESS_THRESHOLD: u16 = 176;
pub const HID_USAGE_DIGITIZER_BUTTON_SWITCH: u16 = 88;
pub const HID_USAGE_DIGITIZER_CHARACTER_GESTURE: u16 = 36;
pub const HID_USAGE_DIGITIZER_CHARACTER_GESTURE_DATA: u16 = 99;
pub const HID_USAGE_DIGITIZER_CHARACTER_GESTURE_DATA_LENGTH: u16 = 98;
pub const HID_USAGE_DIGITIZER_CHISEL_MARKER: u16 = 117;
pub const HID_USAGE_DIGITIZER_CONTACT_COUNT: u16 = 84;
pub const HID_USAGE_DIGITIZER_CONTACT_COUNT_MAXIMUM: u16 = 85;
pub const HID_USAGE_DIGITIZER_CONTACT_IDENTIFIER: u16 = 81;
pub const HID_USAGE_DIGITIZER_COORD_MEASURING: u16 = 7;
pub const HID_USAGE_DIGITIZER_DATA_VALID: u16 = 55;
pub const HID_USAGE_DIGITIZER_DEVICE_CONFIGURATION: u16 = 14;
pub const HID_USAGE_DIGITIZER_DEVICE_IDENTIFIER: u16 = 83;
pub const HID_USAGE_DIGITIZER_DEVICE_MODE: u16 = 82;
pub const HID_USAGE_DIGITIZER_DEVICE_SETTINGS: u16 = 35;
pub const HID_USAGE_DIGITIZER_DEVICE_SUPPORTED_PROTOCOLS: u16 = 147;
pub const HID_USAGE_DIGITIZER_DIGITIZER: u16 = 1;
pub const HID_USAGE_DIGITIZER_DIGITIZER_DIAGNOSTIC: u16 = 128;
pub const HID_USAGE_DIGITIZER_DIGITIZER_ERROR: u16 = 129;
pub const HID_USAGE_DIGITIZER_ERASER: u16 = 69;
pub const HID_USAGE_DIGITIZER_ERR_CHARGE_LOW: u16 = 133;
pub const HID_USAGE_DIGITIZER_ERR_FULL_TRANS_FEATURES_UNAVAILABLE: u16 = 132;
pub const HID_USAGE_DIGITIZER_ERR_NORMAL_STATUS: u16 = 130;
pub const HID_USAGE_DIGITIZER_ERR_TRANSDUCERS_EXCEEDED: u16 = 131;
pub const HID_USAGE_DIGITIZER_FINGER: u16 = 34;
pub const HID_USAGE_DIGITIZER_FREE_SPACE_WAND: u16 = 13;
pub const HID_USAGE_DIGITIZER_GESTURE_CHARACTER_ENABLE: u16 = 109;
pub const HID_USAGE_DIGITIZER_GESTURE_CHARACTER_ENCODING: u16 = 100;
pub const HID_USAGE_DIGITIZER_GESTURE_CHARACTER_QUALITY: u16 = 97;
pub const HID_USAGE_DIGITIZER_HEAT_MAP: u16 = 15;
pub const HID_USAGE_DIGITIZER_HEAT_MAP_FRAME_DATA: u16 = 108;
pub const HID_USAGE_DIGITIZER_HEAT_MAP_PROTOCOL_VENDOR_ID: u16 = 106;
pub const HID_USAGE_DIGITIZER_HEAT_MAP_PROTOCOL_VERSION: u16 = 107;
pub const HID_USAGE_DIGITIZER_HEIGHT: u16 = 73;
pub const HID_USAGE_DIGITIZER_HIGHLIGHTER: u16 = 116;
pub const HID_USAGE_DIGITIZER_INK: u16 = 114;
pub const HID_USAGE_DIGITIZER_INVERT: u16 = 60;
pub const HID_USAGE_DIGITIZER_IN_RANGE: u16 = 50;
pub const HID_USAGE_DIGITIZER_LATENCY_MODE: u16 = 96;
pub const HID_USAGE_DIGITIZER_LIGHT_PEN: u16 = 3;
pub const HID_USAGE_DIGITIZER_MICROSOFT_PEN_PROTOCOL: u16 = 152;
pub const HID_USAGE_DIGITIZER_MULTI_POINT: u16 = 12;
pub const HID_USAGE_DIGITIZER_NO_PREFERENCE: u16 = 119;
pub const HID_USAGE_DIGITIZER_NO_PREFERRED_COLOR: u16 = 111;
pub const HID_USAGE_DIGITIZER_NO_PROTOCOL: u16 = 149;
pub const HID_USAGE_DIGITIZER_PAD_TYPE: u16 = 89;
pub const HID_USAGE_DIGITIZER_PEN: u16 = 2;
pub const HID_USAGE_DIGITIZER_PENCIL: u16 = 115;
pub const HID_USAGE_DIGITIZER_PREFERRED_COLOR: u16 = 92;
pub const HID_USAGE_DIGITIZER_PREFERRED_COLOR_IS_LOCKED: u16 = 93;
pub const HID_USAGE_DIGITIZER_PREFERRED_LINE_STYLE: u16 = 112;
pub const HID_USAGE_DIGITIZER_PREFERRED_LINE_STYLE_IS_LOCKED: u16 = 113;
pub const HID_USAGE_DIGITIZER_PREFERRED_LINE_WIDTH: u16 = 94;
pub const HID_USAGE_DIGITIZER_PREFERRED_LINE_WIDTH_IS_LOCKED: u16 = 95;
pub const HID_USAGE_DIGITIZER_PROG_CHANGE_KEYS: u16 = 58;
pub const HID_USAGE_DIGITIZER_PUCK: u16 = 33;
pub const HID_USAGE_DIGITIZER_QUALITY: u16 = 54;
pub const HID_USAGE_DIGITIZER_REPORT_RATE: u16 = 161;
pub const HID_USAGE_DIGITIZER_SCAN_TIME: u16 = 86;
pub const HID_USAGE_DIGITIZER_SECONDARY_BARREL_SWITCH: u16 = 90;
pub const HID_USAGE_DIGITIZER_SECONDARY_TIP_SWITCH: u16 = 67;
pub const HID_USAGE_DIGITIZER_STEREO_PLOTTER: u16 = 9;
pub const HID_USAGE_DIGITIZER_STYLUS: u16 = 32;
pub const HID_USAGE_DIGITIZER_SUPPORTED_REPORT_RATES: u16 = 160;
pub const HID_USAGE_DIGITIZER_SURFACE_SWITCH: u16 = 87;
pub const HID_USAGE_DIGITIZER_SWITCH_DISABLED: u16 = 163;
pub const HID_USAGE_DIGITIZER_SWITCH_UNIMPLEMENTED: u16 = 164;
pub const HID_USAGE_DIGITIZER_TABLET_FUNC_KEYS: u16 = 57;
pub const HID_USAGE_DIGITIZER_TABLET_PICK: u16 = 70;
pub const HID_USAGE_DIGITIZER_TAP: u16 = 53;
pub const HID_USAGE_DIGITIZER_TIP_PRESSURE: u16 = 48;
pub const HID_USAGE_DIGITIZER_TIP_SWITCH: u16 = 66;
pub const HID_USAGE_DIGITIZER_TOUCH: u16 = 51;
pub const HID_USAGE_DIGITIZER_TOUCH_PAD: u16 = 5;
pub const HID_USAGE_DIGITIZER_TOUCH_SCREEN: u16 = 4;
pub const HID_USAGE_DIGITIZER_TOUCH_VALID: u16 = 71;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_CONNECTED: u16 = 162;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_INDEX: u16 = 56;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_INDEX_SELECTOR: u16 = 166;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_PRODUCT: u16 = 146;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_SERIAL: u16 = 91;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_SERIAL_PART2: u16 = 110;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_SOFTWARE_INFO: u16 = 144;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_SUPPORTED_PROTOCOLS: u16 = 148;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_SWITCHES: u16 = 165;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_VENDOR: u16 = 145;
pub const HID_USAGE_DIGITIZER_TWIST: u16 = 65;
pub const HID_USAGE_DIGITIZER_UNTOUCH: u16 = 52;
pub const HID_USAGE_DIGITIZER_USI_PROTOCOL: u16 = 151;
pub const HID_USAGE_DIGITIZER_UTF16_BIG_ENDIAN_CHARACTER_GESTURE_ENCODING: u16 = 103;
pub const HID_USAGE_DIGITIZER_UTF16_LITTLE_ENDIAN_CHARACTER_GESTURE_ENCODING: u16 = 102;
pub const HID_USAGE_DIGITIZER_UTF32_BIG_ENDIAN_CHARACTER_GESTURE_ENCODING: u16 = 105;
pub const HID_USAGE_DIGITIZER_UTF32_LITTLE_ENDIAN_CHARACTER_GESTURE_ENCODING: u16 = 104;
pub const HID_USAGE_DIGITIZER_UTF8_CHARACTER_GESTURE_ENCODING: u16 = 101;
pub const HID_USAGE_DIGITIZER_WACOM_AES_PROTOCOL: u16 = 150;
pub const HID_USAGE_DIGITIZER_WHITE_BOARD: u16 = 6;
pub const HID_USAGE_DIGITIZER_WIDTH: u16 = 72;
pub const HID_USAGE_DIGITIZER_X_TILT: u16 = 61;
pub const HID_USAGE_DIGITIZER_Y_TILT: u16 = 62;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_CALIBRATED_SCREEN_HEIGHT: u16 = 517;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_CALIBRATED_SCREEN_WIDTH: u16 = 516;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_CAPABILITIES: u16 = 17;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_CONFIGURATION: u16 = 18;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_CONFIGURATION_STATUS: u16 = 769;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_CONTROL: u16 = 20;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_DEVICE_MODE_REQUEST: u16 = 1024;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_DISPLAY_MANUFACTURER_DATE: u16 = 515;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_DISPLAY_MANUFACTURER_ID: u16 = 512;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_DISPLAY_PRODUCT_ID: u16 = 513;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_DISPLAY_SERIAL_NUMBER: u16 = 514;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_EYE_TRACKER: u16 = 1;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_GAZE_POINT: u16 = 36;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_HEAD_DIRECTION_POINT: u16 = 40;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_HEAD_POSITION: u16 = 39;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_HEAD_TRACKER: u16 = 2;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_LEFT_EYE_POSITION: u16 = 37;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_MAXIMUM_SCREEN_PLANE_HEIGHT: u16 = 261;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_MAXIMUM_SCREEN_PLANE_WIDTH: u16 = 260;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_MAXIMUM_TRACKING_DISTANCE: u16 = 259;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_MINIMUM_TRACKING_DISTANCE: u16 = 257;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_OPTIMUM_TRACKING_DISTANCE: u16 = 258;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_POSITION_X: u16 = 33;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_POSITION_Y: u16 = 34;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_POSITION_Z: u16 = 35;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_RIGHT_EYE_POSITION: u16 = 38;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_ROTATION_ABOUT_X_AXIS: u16 = 41;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_ROTATION_ABOUT_Y_AXIS: u16 = 42;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_ROTATION_ABOUT_Z_AXIS: u16 = 43;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_SAMPLING_FREQUENCY: u16 = 768;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_SENSOR_TIMESTAMP: u16 = 32;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_STATUS: u16 = 19;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_TRACKER_QUALITY: u16 = 256;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_TRACKING_DATA: u16 = 16;
pub const HID_USAGE_FIDO_ALLIANCE_INPUT_REPORT_DATA: u16 = 32;
pub const HID_USAGE_FIDO_ALLIANCE_OUTPUT_REPORT_DATA: u16 = 33;
pub const HID_USAGE_FIDO_ALLIANCE_U2F_AUTHENTICATOR_DEVICE: u16 = 1;
pub const HID_USAGE_GAME_3D_GAME_CONTROLLER: u16 = 1;
pub const HID_USAGE_GAME_BUMP: u16 = 44;
pub const HID_USAGE_GAME_FLIPPER: u16 = 42;
pub const HID_USAGE_GAME_FORMFITTING_GAMEPAD: u16 = 58;
pub const HID_USAGE_GAME_GAMEPAD_FIRE_JUMP: u16 = 55;
pub const HID_USAGE_GAME_GAMEPAD_TRIGGER: u16 = 57;
pub const HID_USAGE_GAME_GUN_AUTOMATIC: u16 = 53;
pub const HID_USAGE_GAME_GUN_BOLT: u16 = 48;
pub const HID_USAGE_GAME_GUN_BURST: u16 = 52;
pub const HID_USAGE_GAME_GUN_CLIP: u16 = 49;
pub const HID_USAGE_GAME_GUN_DEVICE: u16 = 3;
pub const HID_USAGE_GAME_GUN_SAFETY: u16 = 54;
pub const HID_USAGE_GAME_GUN_SELECTOR: u16 = 50;
pub const HID_USAGE_GAME_GUN_SINGLE_SHOT: u16 = 51;
pub const HID_USAGE_GAME_LEAN_FORWARD_BACK: u16 = 40;
pub const HID_USAGE_GAME_LEAN_RIGHT_LEFT: u16 = 39;
pub const HID_USAGE_GAME_MOVE_FORWARD_BACK: u16 = 37;
pub const HID_USAGE_GAME_MOVE_RIGHT_LEFT: u16 = 36;
pub const HID_USAGE_GAME_MOVE_UP_DOWN: u16 = 38;
pub const HID_USAGE_GAME_NEW_GAME: u16 = 45;
pub const HID_USAGE_GAME_PINBALL_DEVICE: u16 = 2;
pub const HID_USAGE_GAME_PITCH_FORWARD_BACK: u16 = 34;
pub const HID_USAGE_GAME_PLAYER: u16 = 47;
pub const HID_USAGE_GAME_POV: u16 = 32;
pub const HID_USAGE_GAME_POV_HEIGHT: u16 = 41;
pub const HID_USAGE_GAME_ROLL_RIGHT_LEFT: u16 = 35;
pub const HID_USAGE_GAME_SECONDARY_FLIPPER: u16 = 43;
pub const HID_USAGE_GAME_SHOOT_BALL: u16 = 46;
pub const HID_USAGE_GAME_TURN_RIGHT_LEFT: u16 = 33;
pub const HID_USAGE_GENERIC_ASSISTIVE_CONTROL: u16 = 16;
pub const HID_USAGE_GENERIC_BYTE_COUNT: u16 = 59;
pub const HID_USAGE_GENERIC_CALL_ACTIVE_LED: u16 = 224;
pub const HID_USAGE_GENERIC_CALL_MUTE_LED: u16 = 226;
pub const HID_USAGE_GENERIC_CALL_MUTE_TOGGLE: u16 = 225;
pub const HID_USAGE_GENERIC_CALL_STATE_MANAGEMENT_CONTROL: u16 = 19;
pub const HID_USAGE_GENERIC_CHASSIS_ENCLOSURE: u16 = 197;
pub const HID_USAGE_GENERIC_COMPUTER_CHASSIS_DEVICE: u16 = 11;
pub const HID_USAGE_GENERIC_CONTROL_ENABLE: u16 = 203;
pub const HID_USAGE_GENERIC_COOLANT_CRITICAL_LEVEL: u16 = 195;
pub const HID_USAGE_GENERIC_COOLANT_LEVEL: u16 = 194;
pub const HID_USAGE_GENERIC_COOLANT_PUMP: u16 = 196;
pub const HID_USAGE_GENERIC_COUNTED_BUFFER: u16 = 58;
pub const HID_USAGE_GENERIC_DEVICE_BACKGROUNDNONUSER_CONTROLS: u16 = 1;
pub const HID_USAGE_GENERIC_DEVICE_BATTERY_STRENGTH: u16 = 32;
pub const HID_USAGE_GENERIC_DEVICE_BOTH_HANDS: u16 = 52;
pub const HID_USAGE_GENERIC_DEVICE_DISCOVER_WIRELESS_CONTROL: u16 = 35;
pub const HID_USAGE_GENERIC_DEVICE_DOCK: u16 = 17;
pub const HID_USAGE_GENERIC_DEVICE_EITHER_HAND: u16 = 49;
pub const HID_USAGE_GENERIC_DEVICE_GRIP_POSE_OFFSET: u16 = 64;
pub const HID_USAGE_GENERIC_DEVICE_HANDEDNESS: u16 = 48;
pub const HID_USAGE_GENERIC_DEVICE_HARDWARE_VERSION: u16 = 44;
pub const HID_USAGE_GENERIC_DEVICE_LEFT_HAND: u16 = 50;
pub const HID_USAGE_GENERIC_DEVICE_MAJOR: u16 = 45;
pub const HID_USAGE_GENERIC_DEVICE_MINOR: u16 = 46;
pub const HID_USAGE_GENERIC_DEVICE_POINTER_POSE_OFFSET: u16 = 65;
pub const HID_USAGE_GENERIC_DEVICE_PROTOCOL_VERSION: u16 = 43;
pub const HID_USAGE_GENERIC_DEVICE_REVISION: u16 = 47;
pub const HID_USAGE_GENERIC_DEVICE_RF_SIGNAL_STRENGTH: u16 = 41;
pub const HID_USAGE_GENERIC_DEVICE_RIGHT_HAND: u16 = 51;
pub const HID_USAGE_GENERIC_DEVICE_SECURITY_CODE_CHAR_ENTERED: u16 = 36;
pub const HID_USAGE_GENERIC_DEVICE_SECURITY_CODE_CHAR_ERASED: u16 = 37;
pub const HID_USAGE_GENERIC_DEVICE_SECURITY_CODE_CLEARED: u16 = 38;
pub const HID_USAGE_GENERIC_DEVICE_SEQUENCE_ID: u16 = 39;
pub const HID_USAGE_GENERIC_DEVICE_SEQUENCE_ID_RESET: u16 = 40;
pub const HID_USAGE_GENERIC_DEVICE_SOFTWARE_VERSION: u16 = 42;
pub const HID_USAGE_GENERIC_DEVICE_WIRELESS_CHANNEL: u16 = 33;
pub const HID_USAGE_GENERIC_DEVICE_WIRELESS_ID: u16 = 34;
pub const HID_USAGE_GENERIC_DIAL: u16 = 55;
pub const HID_USAGE_GENERIC_DOCKABLE_DEVICE: u16 = 18;
pub const HID_USAGE_GENERIC_DOCKABLE_DEVICE_DISPLAY_OCCLUSION: u16 = 213;
pub const HID_USAGE_GENERIC_DOCKABLE_DEVICE_DOCKING_STATE: u16 = 212;
pub const HID_USAGE_GENERIC_DOCKABLE_DEVICE_OBJECT_TYPE: u16 = 214;
pub const HID_USAGE_GENERIC_DOCKABLE_DEVICE_PRIMARY_USAGE_ID: u16 = 211;
pub const HID_USAGE_GENERIC_DOCKABLE_DEVICE_PRIMARY_USAGE_PAGE: u16 = 210;
pub const HID_USAGE_GENERIC_DOCKABLE_DEVICE_UNIQUE_ID: u16 = 208;
pub const HID_USAGE_GENERIC_DOCKABLE_DEVICE_VENDOR_ID: u16 = 209;
pub const HID_USAGE_GENERIC_DPAD_DOWN: u16 = 145;
pub const HID_USAGE_GENERIC_DPAD_LEFT: u16 = 147;
pub const HID_USAGE_GENERIC_DPAD_RIGHT: u16 = 146;
pub const HID_USAGE_GENERIC_DPAD_UP: u16 = 144;
pub const HID_USAGE_GENERIC_FEATURE_NOTIFICATION: u16 = 71;
pub const HID_USAGE_GENERIC_GAMEPAD: u16 = 5;
pub const HID_USAGE_GENERIC_HATSWITCH: u16 = 57;
pub const HID_USAGE_GENERIC_INDEX_TRIGGER: u16 = 148;
pub const HID_USAGE_GENERIC_INTERACTIVE_CONTROL: u16 = 14;
pub const HID_USAGE_GENERIC_JOYSTICK: u16 = 4;
pub const HID_USAGE_GENERIC_KEYBOARD: u16 = 6;
pub const HID_USAGE_GENERIC_KEYPAD: u16 = 7;
pub const HID_USAGE_GENERIC_MOTION_WAKEUP: u16 = 60;
pub const HID_USAGE_GENERIC_MOUSE: u16 = 2;
pub const HID_USAGE_GENERIC_MULTI_AXIS_CONTROLLER: u16 = 8;
pub const HID_USAGE_GENERIC_PALM_TRIGGER: u16 = 149;
pub const HID_USAGE_GENERIC_POINTER: u16 = 1;
pub const HID_USAGE_GENERIC_PORTABLE_DEVICE_CONTROL: u16 = 13;
pub const HID_USAGE_GENERIC_QW: u16 = 76;
pub const HID_USAGE_GENERIC_QX: u16 = 73;
pub const HID_USAGE_GENERIC_QY: u16 = 74;
pub const HID_USAGE_GENERIC_QZ: u16 = 75;
pub const HID_USAGE_GENERIC_RESOLUTION_MULTIPLIER: u16 = 72;
pub const HID_USAGE_GENERIC_RPM: u16 = 193;
pub const HID_USAGE_GENERIC_RX: u16 = 51;
pub const HID_USAGE_GENERIC_RY: u16 = 52;
pub const HID_USAGE_GENERIC_RZ: u16 = 53;
pub const HID_USAGE_GENERIC_SELECT: u16 = 62;
pub const HID_USAGE_GENERIC_SENSOR_ZONE: u16 = 192;
pub const HID_USAGE_GENERIC_SLIDER: u16 = 54;
pub const HID_USAGE_GENERIC_SPATIAL_CONTROLLER: u16 = 15;
pub const HID_USAGE_GENERIC_START: u16 = 61;
pub const HID_USAGE_GENERIC_SYSCTL_ACCESSIBILITY_BINDING: u16 = 170;
pub const HID_USAGE_GENERIC_SYSCTL_APP_BREAK: u16 = 165;
pub const HID_USAGE_GENERIC_SYSCTL_APP_DBG_BREAK: u16 = 166;
pub const HID_USAGE_GENERIC_SYSCTL_APP_MENU: u16 = 134;
pub const HID_USAGE_GENERIC_SYSCTL_COLD_RESTART: u16 = 142;
pub const HID_USAGE_GENERIC_SYSCTL_CONTEXT_MENU: u16 = 132;
pub const HID_USAGE_GENERIC_SYSCTL_DISMISS_NOTIFICATION: u16 = 154;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_AUTOSCALE: u16 = 183;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_BOTH: u16 = 179;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_DUAL: u16 = 180;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_EXTERNAL: u16 = 178;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_INTERNAL: u16 = 177;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_INVERT: u16 = 176;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_SWAP: u16 = 182;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_TOGGLE: u16 = 181;
pub const HID_USAGE_GENERIC_SYSCTL_DOCK: u16 = 160;
pub const HID_USAGE_GENERIC_SYSCTL_FN: u16 = 151;
pub const HID_USAGE_GENERIC_SYSCTL_FN_LOCK: u16 = 152;
pub const HID_USAGE_GENERIC_SYSCTL_FN_LOCK_INDICATOR: u16 = 153;
pub const HID_USAGE_GENERIC_SYSCTL_HELP_MENU: u16 = 135;
pub const HID_USAGE_GENERIC_SYSCTL_HIBERNATE: u16 = 168;
pub const HID_USAGE_GENERIC_SYSCTL_MAIN_MENU: u16 = 133;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_DOWN: u16 = 141;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_EXIT: u16 = 136;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_LEFT: u16 = 139;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_RIGHT: u16 = 138;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_SELECT: u16 = 137;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_UP: u16 = 140;
pub const HID_USAGE_GENERIC_SYSCTL_MICROPHONE_MUTE: u16 = 169;
pub const HID_USAGE_GENERIC_SYSCTL_MUTE: u16 = 167;
pub const HID_USAGE_GENERIC_SYSCTL_POWER: u16 = 129;
pub const HID_USAGE_GENERIC_SYSCTL_SETUP: u16 = 162;
pub const HID_USAGE_GENERIC_SYSCTL_SLEEP: u16 = 130;
pub const HID_USAGE_GENERIC_SYSCTL_SYS_BREAK: u16 = 163;
pub const HID_USAGE_GENERIC_SYSCTL_SYS_DBG_BREAK: u16 = 164;
pub const HID_USAGE_GENERIC_SYSCTL_UNDOCK: u16 = 161;
pub const HID_USAGE_GENERIC_SYSCTL_WAKE: u16 = 131;
pub const HID_USAGE_GENERIC_SYSCTL_WARM_RESTART: u16 = 143;
pub const HID_USAGE_GENERIC_SYSTEM_CTL: u16 = 128;
pub const HID_USAGE_GENERIC_SYSTEM_DISPLAY_ROTATION_LOCK_BUTTON: u16 = 201;
pub const HID_USAGE_GENERIC_SYSTEM_DISPLAY_ROTATION_LOCK_SLIDER_SWITCH: u16 = 202;
pub const HID_USAGE_GENERIC_SYSTEM_DO_NOT_DISTURB: u16 = 155;
pub const HID_USAGE_GENERIC_TABLET_PC_SYSTEM_CTL: u16 = 9;
pub const HID_USAGE_GENERIC_THUMBSTICK: u16 = 150;
pub const HID_USAGE_GENERIC_VBRX: u16 = 67;
pub const HID_USAGE_GENERIC_VBRY: u16 = 68;
pub const HID_USAGE_GENERIC_VBRZ: u16 = 69;
pub const HID_USAGE_GENERIC_VNO: u16 = 70;
pub const HID_USAGE_GENERIC_VX: u16 = 64;
pub const HID_USAGE_GENERIC_VY: u16 = 65;
pub const HID_USAGE_GENERIC_VZ: u16 = 66;
pub const HID_USAGE_GENERIC_WATER_COOLING_DEVICE: u16 = 10;
pub const HID_USAGE_GENERIC_WHEEL: u16 = 56;
pub const HID_USAGE_GENERIC_WIRELESS_RADIO_BUTTON: u16 = 198;
pub const HID_USAGE_GENERIC_WIRELESS_RADIO_CONTROLS: u16 = 12;
pub const HID_USAGE_GENERIC_WIRELESS_RADIO_LED: u16 = 199;
pub const HID_USAGE_GENERIC_WIRELESS_RADIO_SLIDER_SWITCH: u16 = 200;
pub const HID_USAGE_GENERIC_X: u16 = 48;
pub const HID_USAGE_GENERIC_Y: u16 = 49;
pub const HID_USAGE_GENERIC_Z: u16 = 50;
pub const HID_USAGE_HAPTICS_AUTO_ASSOCIATED_CONTROL: u16 = 34;
pub const HID_USAGE_HAPTICS_AUTO_TRIGGER: u16 = 32;
pub const HID_USAGE_HAPTICS_DURATION_LIST: u16 = 17;
pub const HID_USAGE_HAPTICS_INTENSITY: u16 = 35;
pub const HID_USAGE_HAPTICS_MANUAL_TRIGGER: u16 = 33;
pub const HID_USAGE_HAPTICS_REPEAT_COUNT: u16 = 36;
pub const HID_USAGE_HAPTICS_RETRIGGER_PERIOD: u16 = 37;
pub const HID_USAGE_HAPTICS_SIMPLE_CONTROLLER: u16 = 1;
pub const HID_USAGE_HAPTICS_WAVEFORM_BRUSH_CONTINUOUS: u16 = 4111;
pub const HID_USAGE_HAPTICS_WAVEFORM_BUZZ: u16 = 4100;
pub const HID_USAGE_HAPTICS_WAVEFORM_CHISEL_MARKER_CONTINUOUS: u16 = 4110;
pub const HID_USAGE_HAPTICS_WAVEFORM_CLICK: u16 = 4099;
pub const HID_USAGE_HAPTICS_WAVEFORM_CUTOFF_TIME: u16 = 40;
pub const HID_USAGE_HAPTICS_WAVEFORM_ERASER_CONTINUOUS: u16 = 4112;
pub const HID_USAGE_HAPTICS_WAVEFORM_ERROR: u16 = 4106;
pub const HID_USAGE_HAPTICS_WAVEFORM_HOVER: u16 = 4104;
pub const HID_USAGE_HAPTICS_WAVEFORM_INK_CONTINUOUS: u16 = 4107;
pub const HID_USAGE_HAPTICS_WAVEFORM_LIST: u16 = 16;
pub const HID_USAGE_HAPTICS_WAVEFORM_MARKER_CONTINUOUS: u16 = 4109;
pub const HID_USAGE_HAPTICS_WAVEFORM_NONE: u16 = 4097;
pub const HID_USAGE_HAPTICS_WAVEFORM_PENCIL_CONTINUOUS: u16 = 4108;
pub const HID_USAGE_HAPTICS_WAVEFORM_PRESS: u16 = 4102;
pub const HID_USAGE_HAPTICS_WAVEFORM_RELEASE: u16 = 4103;
pub const HID_USAGE_HAPTICS_WAVEFORM_RUMBLE: u16 = 4101;
pub const HID_USAGE_HAPTICS_WAVEFORM_SPARKLE_CONTINUOUS: u16 = 4113;
pub const HID_USAGE_HAPTICS_WAVEFORM_STOP: u16 = 4098;
pub const HID_USAGE_HAPTICS_WAVEFORM_SUCCESS: u16 = 4105;
pub const HID_USAGE_HAPTICS_WAVEFORM_VENDOR_ID: u16 = 39;
pub const HID_USAGE_HAPTICS_WAVEFORM_VENDOR_PAGE: u16 = 38;
pub const HID_USAGE_KEYBOARD_AGAIN: u16 = 121;
pub const HID_USAGE_KEYBOARD_ALTERNATE_ERASE: u16 = 153;
pub const HID_USAGE_KEYBOARD_APPLICATION: u16 = 101;
pub const HID_USAGE_KEYBOARD_BACKSLASH_AND_PIPE: u16 = 49;
pub const HID_USAGE_KEYBOARD_CANCEL: u16 = 155;
pub const HID_USAGE_KEYBOARD_CAPS_LOCK: u16 = 57;
pub const HID_USAGE_KEYBOARD_CLEAR: u16 = 156;
pub const HID_USAGE_KEYBOARD_CLEAR_AGAIN: u16 = 162;
pub const HID_USAGE_KEYBOARD_COMMA_AND_LESSTHAN: u16 = 54;
pub const HID_USAGE_KEYBOARD_COPY: u16 = 124;
pub const HID_USAGE_KEYBOARD_CRSEL_PROPS: u16 = 163;
pub const HID_USAGE_KEYBOARD_CURRENCY_SUBUNIT: u16 = 181;
pub const HID_USAGE_KEYBOARD_CURRENCY_UNIT: u16 = 180;
pub const HID_USAGE_KEYBOARD_CUT: u16 = 123;
pub const HID_USAGE_KEYBOARD_DASH: u16 = 86;
pub const HID_USAGE_KEYBOARD_DASH_AND_UNDERSCORE: u16 = 45;
pub const HID_USAGE_KEYBOARD_DECIMAL_SEPARATOR: u16 = 179;
pub const HID_USAGE_KEYBOARD_DELETE: u16 = 42;
pub const HID_USAGE_KEYBOARD_DELETE_FORWARD: u16 = 76;
pub const HID_USAGE_KEYBOARD_DOWNARROW: u16 = 81;
pub const HID_USAGE_KEYBOARD_EIGHT_AND_STAR: u16 = 37;
pub const HID_USAGE_KEYBOARD_END: u16 = 77;
pub const HID_USAGE_KEYBOARD_EQUALS_AND_PLUS: u16 = 46;
pub const HID_USAGE_KEYBOARD_ESCAPE: u16 = 41;
pub const HID_USAGE_KEYBOARD_EXECUTE: u16 = 116;
pub const HID_USAGE_KEYBOARD_EXSEL: u16 = 164;
pub const HID_USAGE_KEYBOARD_F1: u16 = 58;
pub const HID_USAGE_KEYBOARD_F10: u16 = 67;
pub const HID_USAGE_KEYBOARD_F11: u16 = 68;
pub const HID_USAGE_KEYBOARD_F12: u16 = 69;
pub const HID_USAGE_KEYBOARD_F13: u16 = 104;
pub const HID_USAGE_KEYBOARD_F14: u16 = 105;
pub const HID_USAGE_KEYBOARD_F15: u16 = 106;
pub const HID_USAGE_KEYBOARD_F16: u16 = 107;
pub const HID_USAGE_KEYBOARD_F17: u16 = 108;
pub const HID_USAGE_KEYBOARD_F18: u16 = 109;
pub const HID_USAGE_KEYBOARD_F19: u16 = 110;
pub const HID_USAGE_KEYBOARD_F2: u16 = 59;
pub const HID_USAGE_KEYBOARD_F20: u16 = 111;
pub const HID_USAGE_KEYBOARD_F21: u16 = 112;
pub const HID_USAGE_KEYBOARD_F22: u16 = 113;
pub const HID_USAGE_KEYBOARD_F23: u16 = 114;
pub const HID_USAGE_KEYBOARD_F24: u16 = 115;
pub const HID_USAGE_KEYBOARD_F3: u16 = 60;
pub const HID_USAGE_KEYBOARD_F4: u16 = 61;
pub const HID_USAGE_KEYBOARD_F5: u16 = 62;
pub const HID_USAGE_KEYBOARD_F6: u16 = 63;
pub const HID_USAGE_KEYBOARD_F7: u16 = 64;
pub const HID_USAGE_KEYBOARD_F8: u16 = 65;
pub const HID_USAGE_KEYBOARD_F9: u16 = 66;
pub const HID_USAGE_KEYBOARD_FIND: u16 = 126;
pub const HID_USAGE_KEYBOARD_FIVE_AND_PERCENT: u16 = 34;
pub const HID_USAGE_KEYBOARD_FORWARDSLASH_AND_QUESTIONMARK: u16 = 56;
pub const HID_USAGE_KEYBOARD_FOUR_AND_DOLLAR: u16 = 33;
pub const HID_USAGE_KEYBOARD_GRAVE_ACCENT_AND_TILDE: u16 = 53;
pub const HID_USAGE_KEYBOARD_HELP: u16 = 117;
pub const HID_USAGE_KEYBOARD_HOME: u16 = 74;
pub const HID_USAGE_KEYBOARD_INSERT: u16 = 73;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL1: u16 = 135;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL2: u16 = 136;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL3: u16 = 137;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL4: u16 = 138;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL5: u16 = 139;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL6: u16 = 140;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL7: u16 = 141;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL8: u16 = 142;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL9: u16 = 143;
pub const HID_USAGE_KEYBOARD_KEYPAD_A: u16 = 188;
pub const HID_USAGE_KEYBOARD_KEYPAD_AMPERSAND: u16 = 199;
pub const HID_USAGE_KEYBOARD_KEYPAD_AT: u16 = 206;
pub const HID_USAGE_KEYBOARD_KEYPAD_B: u16 = 189;
pub const HID_USAGE_KEYBOARD_KEYPAD_BACKSPACE: u16 = 187;
pub const HID_USAGE_KEYBOARD_KEYPAD_BANG: u16 = 207;
pub const HID_USAGE_KEYBOARD_KEYPAD_BAR: u16 = 201;
pub const HID_USAGE_KEYBOARD_KEYPAD_BINARY: u16 = 218;
pub const HID_USAGE_KEYBOARD_KEYPAD_C: u16 = 190;
pub const HID_USAGE_KEYBOARD_KEYPAD_CARET: u16 = 195;
pub const HID_USAGE_KEYBOARD_KEYPAD_CLEAR: u16 = 216;
pub const HID_USAGE_KEYBOARD_KEYPAD_CLEAR_ENTRY: u16 = 217;
pub const HID_USAGE_KEYBOARD_KEYPAD_COLON: u16 = 203;
pub const HID_USAGE_KEYBOARD_KEYPAD_COMMA: u16 = 133;
pub const HID_USAGE_KEYBOARD_KEYPAD_D: u16 = 191;
pub const HID_USAGE_KEYBOARD_KEYPAD_DECIMAL: u16 = 220;
pub const HID_USAGE_KEYBOARD_KEYPAD_DOUBLE_0: u16 = 176;
pub const HID_USAGE_KEYBOARD_KEYPAD_DOUBLE_AMPERSAND: u16 = 200;
pub const HID_USAGE_KEYBOARD_KEYPAD_DOUBLE_BAR: u16 = 202;
pub const HID_USAGE_KEYBOARD_KEYPAD_E: u16 = 192;
pub const HID_USAGE_KEYBOARD_KEYPAD_EIGHT_AND_UP_ARROW: u16 = 96;
pub const HID_USAGE_KEYBOARD_KEYPAD_ENTER: u16 = 88;
pub const HID_USAGE_KEYBOARD_KEYPAD_EQUALS: u16 = 103;
pub const HID_USAGE_KEYBOARD_KEYPAD_EQUAL_SIGN: u16 = 134;
pub const HID_USAGE_KEYBOARD_KEYPAD_F: u16 = 193;
pub const HID_USAGE_KEYBOARD_KEYPAD_FIVE: u16 = 93;
pub const HID_USAGE_KEYBOARD_KEYPAD_FORWARDSLASH: u16 = 84;
pub const HID_USAGE_KEYBOARD_KEYPAD_FOUR_AND_LEFT_ARROW: u16 = 92;
pub const HID_USAGE_KEYBOARD_KEYPAD_GREATER: u16 = 198;
pub const HID_USAGE_KEYBOARD_KEYPAD_HASH: u16 = 204;
pub const HID_USAGE_KEYBOARD_KEYPAD_HEXADECIMAL: u16 = 221;
pub const HID_USAGE_KEYBOARD_KEYPAD_LEFT_BRACE: u16 = 184;
pub const HID_USAGE_KEYBOARD_KEYPAD_LEFT_BRACKET: u16 = 182;
pub const HID_USAGE_KEYBOARD_KEYPAD_LESS: u16 = 197;
pub const HID_USAGE_KEYBOARD_KEYPAD_MEMORY_ADD: u16 = 211;
pub const HID_USAGE_KEYBOARD_KEYPAD_MEMORY_CLEAR: u16 = 210;
pub const HID_USAGE_KEYBOARD_KEYPAD_MEMORY_DIVIDE: u16 = 214;
pub const HID_USAGE_KEYBOARD_KEYPAD_MEMORY_MULTIPLY: u16 = 213;
pub const HID_USAGE_KEYBOARD_KEYPAD_MEMORY_RECALL: u16 = 209;
pub const HID_USAGE_KEYBOARD_KEYPAD_MEMORY_STORE: u16 = 208;
pub const HID_USAGE_KEYBOARD_KEYPAD_MEMORY_SUBTRACT: u16 = 212;
pub const HID_USAGE_KEYBOARD_KEYPAD_NINE_AND_PAGEUP: u16 = 97;
pub const HID_USAGE_KEYBOARD_KEYPAD_OCTAL: u16 = 219;
pub const HID_USAGE_KEYBOARD_KEYPAD_ONE_AND_END: u16 = 89;
pub const HID_USAGE_KEYBOARD_KEYPAD_PERCENTAGE: u16 = 196;
pub const HID_USAGE_KEYBOARD_KEYPAD_PERIOD_AND_DELETE: u16 = 99;
pub const HID_USAGE_KEYBOARD_KEYPAD_PLUS: u16 = 87;
pub const HID_USAGE_KEYBOARD_KEYPAD_PLUS_MINUS: u16 = 215;
pub const HID_USAGE_KEYBOARD_KEYPAD_RIGHT_BRACE: u16 = 185;
pub const HID_USAGE_KEYBOARD_KEYPAD_RIGHT_BRACKET: u16 = 183;
pub const HID_USAGE_KEYBOARD_KEYPAD_SEVEN_AND_HOME: u16 = 95;
pub const HID_USAGE_KEYBOARD_KEYPAD_SIX_AND_RIGHT_ARROW: u16 = 94;
pub const HID_USAGE_KEYBOARD_KEYPAD_SPACE: u16 = 205;
pub const HID_USAGE_KEYBOARD_KEYPAD_STAR: u16 = 85;
pub const HID_USAGE_KEYBOARD_KEYPAD_TAB: u16 = 186;
pub const HID_USAGE_KEYBOARD_KEYPAD_THREE_AND_PAGEDN: u16 = 91;
pub const HID_USAGE_KEYBOARD_KEYPAD_TRIPLE_0: u16 = 177;
pub const HID_USAGE_KEYBOARD_KEYPAD_TWO_AND_DOWN_ARROW: u16 = 90;
pub const HID_USAGE_KEYBOARD_KEYPAD_XOR: u16 = 194;
pub const HID_USAGE_KEYBOARD_KEYPAD_ZERO_AND_INSERT: u16 = 98;
pub const HID_USAGE_KEYBOARD_LALT: u16 = 226;
pub const HID_USAGE_KEYBOARD_LANG1: u16 = 144;
pub const HID_USAGE_KEYBOARD_LANG2: u16 = 145;
pub const HID_USAGE_KEYBOARD_LANG3: u16 = 146;
pub const HID_USAGE_KEYBOARD_LANG4: u16 = 147;
pub const HID_USAGE_KEYBOARD_LANG5: u16 = 148;
pub const HID_USAGE_KEYBOARD_LANG6: u16 = 149;
pub const HID_USAGE_KEYBOARD_LANG7: u16 = 150;
pub const HID_USAGE_KEYBOARD_LANG8: u16 = 151;
pub const HID_USAGE_KEYBOARD_LANG9: u16 = 152;
pub const HID_USAGE_KEYBOARD_LCTRL: u16 = 224;
pub const HID_USAGE_KEYBOARD_LEFTARROW: u16 = 80;
pub const HID_USAGE_KEYBOARD_LEFT_APOS_AND_DOUBLE: u16 = 52;
pub const HID_USAGE_KEYBOARD_LEFT_BRACE: u16 = 47;
pub const HID_USAGE_KEYBOARD_LGUI: u16 = 227;
pub const HID_USAGE_KEYBOARD_LOCKING_CAPS_LOCK: u16 = 130;
pub const HID_USAGE_KEYBOARD_LOCKING_NUM_LOCK: u16 = 131;
pub const HID_USAGE_KEYBOARD_LOCKING_SCROLL_LOCK: u16 = 132;
pub const HID_USAGE_KEYBOARD_LSHFT: u16 = 225;
pub const HID_USAGE_KEYBOARD_MENU: u16 = 118;
pub const HID_USAGE_KEYBOARD_MUTE: u16 = 127;
pub const HID_USAGE_KEYBOARD_NINE_AND_LEFT_BRACKET: u16 = 38;
pub const HID_USAGE_KEYBOARD_NOEVENT: u16 = 0;
pub const HID_USAGE_KEYBOARD_NONUS_BACKSLASH_AND_PIPE: u16 = 100;
pub const HID_USAGE_KEYBOARD_NONUS_HASH_AND_TILDE: u16 = 50;
pub const HID_USAGE_KEYBOARD_NUM_LOCK: u16 = 83;
pub const HID_USAGE_KEYBOARD_ONE: u16 = 30;
pub const HID_USAGE_KEYBOARD_OPER: u16 = 161;
pub const HID_USAGE_KEYBOARD_OUT: u16 = 160;
pub const HID_USAGE_KEYBOARD_PAGEDOWN: u16 = 78;
pub const HID_USAGE_KEYBOARD_PAGEUP: u16 = 75;
pub const HID_USAGE_KEYBOARD_PASTE: u16 = 125;
pub const HID_USAGE_KEYBOARD_PAUSE: u16 = 72;
pub const HID_USAGE_KEYBOARD_PERIOD_AND_GREATERTHAN: u16 = 55;
pub const HID_USAGE_KEYBOARD_POSTFAIL: u16 = 2;
pub const HID_USAGE_KEYBOARD_POWER: u16 = 102;
pub const HID_USAGE_KEYBOARD_PRINT_SCREEN: u16 = 70;
pub const HID_USAGE_KEYBOARD_PRIOR: u16 = 157;
pub const HID_USAGE_KEYBOARD_RALT: u16 = 230;
pub const HID_USAGE_KEYBOARD_RCTRL: u16 = 228;
pub const HID_USAGE_KEYBOARD_RETURN: u16 = 40;
pub const HID_USAGE_KEYBOARD_RETURN_NO_ENTER: u16 = 158;
pub const HID_USAGE_KEYBOARD_RGUI: u16 = 231;
pub const HID_USAGE_KEYBOARD_RIGHTARROW: u16 = 79;
pub const HID_USAGE_KEYBOARD_RIGHT_BRACE: u16 = 48;
pub const HID_USAGE_KEYBOARD_ROLLOVER: u16 = 1;
pub const HID_USAGE_KEYBOARD_RSHFT: u16 = 229;
pub const HID_USAGE_KEYBOARD_SCROLL_LOCK: u16 = 71;
pub const HID_USAGE_KEYBOARD_SELECT: u16 = 119;
pub const HID_USAGE_KEYBOARD_SEMICOLON_AND_COLON: u16 = 51;
pub const HID_USAGE_KEYBOARD_SEPARATOR: u16 = 159;
pub const HID_USAGE_KEYBOARD_SEVEN_AND_AMPERSAND: u16 = 36;
pub const HID_USAGE_KEYBOARD_SIX_AND_CARET: u16 = 35;
pub const HID_USAGE_KEYBOARD_SPACEBAR: u16 = 44;
pub const HID_USAGE_KEYBOARD_STOP: u16 = 120;
pub const HID_USAGE_KEYBOARD_SYSREQ_ATTENTION: u16 = 154;
pub const HID_USAGE_KEYBOARD_TAB: u16 = 43;
pub const HID_USAGE_KEYBOARD_THOUSANDS_SEPARATOR: u16 = 178;
pub const HID_USAGE_KEYBOARD_THREE_AND_HASH: u16 = 32;
pub const HID_USAGE_KEYBOARD_TWO_AND_AT: u16 = 31;
pub const HID_USAGE_KEYBOARD_UNDEFINED: u16 = 3;
pub const HID_USAGE_KEYBOARD_UNDO: u16 = 122;
pub const HID_USAGE_KEYBOARD_UPARROW: u16 = 82;
pub const HID_USAGE_KEYBOARD_VOLUME_DOWN: u16 = 129;
pub const HID_USAGE_KEYBOARD_VOLUME_UP: u16 = 128;
pub const HID_USAGE_KEYBOARD_ZERO: u16 = 39;
pub const HID_USAGE_KEYBOARD_aA: u16 = 4;
pub const HID_USAGE_KEYBOARD_bB: u16 = 5;
pub const HID_USAGE_KEYBOARD_cC: u16 = 6;
pub const HID_USAGE_KEYBOARD_dD: u16 = 7;
pub const HID_USAGE_KEYBOARD_eE: u16 = 8;
pub const HID_USAGE_KEYBOARD_fF: u16 = 9;
pub const HID_USAGE_KEYBOARD_gG: u16 = 10;
pub const HID_USAGE_KEYBOARD_hH: u16 = 11;
pub const HID_USAGE_KEYBOARD_iI: u16 = 12;
pub const HID_USAGE_KEYBOARD_jJ: u16 = 13;
pub const HID_USAGE_KEYBOARD_kK: u16 = 14;
pub const HID_USAGE_KEYBOARD_lL: u16 = 15;
pub const HID_USAGE_KEYBOARD_mM: u16 = 16;
pub const HID_USAGE_KEYBOARD_nN: u16 = 17;
pub const HID_USAGE_KEYBOARD_oO: u16 = 18;
pub const HID_USAGE_KEYBOARD_pP: u16 = 19;
pub const HID_USAGE_KEYBOARD_qQ: u16 = 20;
pub const HID_USAGE_KEYBOARD_rR: u16 = 21;
pub const HID_USAGE_KEYBOARD_sS: u16 = 22;
pub const HID_USAGE_KEYBOARD_tT: u16 = 23;
pub const HID_USAGE_KEYBOARD_uU: u16 = 24;
pub const HID_USAGE_KEYBOARD_vV: u16 = 25;
pub const HID_USAGE_KEYBOARD_wW: u16 = 26;
pub const HID_USAGE_KEYBOARD_xX: u16 = 27;
pub const HID_USAGE_KEYBOARD_yY: u16 = 28;
pub const HID_USAGE_KEYBOARD_zZ: u16 = 29;
pub const HID_USAGE_LAMPARRAY: u16 = 1;
pub const HID_USAGE_LAMPARRAY_ATTRBIUTES_REPORT: u16 = 2;
pub const HID_USAGE_LAMPARRAY_AUTONOMOUS_MODE: u16 = 113;
pub const HID_USAGE_LAMPARRAY_BLUE_LEVEL_COUNT: u16 = 42;
pub const HID_USAGE_LAMPARRAY_BOUNDING_BOX_DEPTH_IN_MICROMETERS: u16 = 6;
pub const HID_USAGE_LAMPARRAY_BOUNDING_BOX_HEIGHT_IN_MICROMETERS: u16 = 5;
pub const HID_USAGE_LAMPARRAY_BOUNDING_BOX_WIDTH_IN_MICROMETERS: u16 = 4;
pub const HID_USAGE_LAMPARRAY_CONTROL_REPORT: u16 = 112;
pub const HID_USAGE_LAMPARRAY_GREEN_LEVEL_COUNT: u16 = 41;
pub const HID_USAGE_LAMPARRAY_INPUT_BINDING: u16 = 45;
pub const HID_USAGE_LAMPARRAY_INTENSITY_LEVEL_COUNT: u16 = 43;
pub const HID_USAGE_LAMPARRAY_IS_PROGRAMMABLE: u16 = 44;
pub const HID_USAGE_LAMPARRAY_KIND: u16 = 7;
pub const HID_USAGE_LAMPARRAY_LAMP_ATTRIBUTES_REQUEST_REPORT: u16 = 32;
pub const HID_USAGE_LAMPARRAY_LAMP_ATTRIBUTES_RESPONSE_REPORT: u16 = 34;
pub const HID_USAGE_LAMPARRAY_LAMP_BLUE_UPDATE_CHANNEL: u16 = 83;
pub const HID_USAGE_LAMPARRAY_LAMP_COUNT: u16 = 3;
pub const HID_USAGE_LAMPARRAY_LAMP_GREEN_UPDATE_CHANNEL: u16 = 82;
pub const HID_USAGE_LAMPARRAY_LAMP_ID: u16 = 33;
pub const HID_USAGE_LAMPARRAY_LAMP_ID_END: u16 = 98;
pub const HID_USAGE_LAMPARRAY_LAMP_ID_START: u16 = 97;
pub const HID_USAGE_LAMPARRAY_LAMP_INTENSITY_UPDATE_CHANNEL: u16 = 84;
pub const HID_USAGE_LAMPARRAY_LAMP_MULTI_UPDATE_REPORT: u16 = 80;
pub const HID_USAGE_LAMPARRAY_LAMP_PURPOSES: u16 = 38;
pub const HID_USAGE_LAMPARRAY_LAMP_RANGE_UPDATE_REPORT: u16 = 96;
pub const HID_USAGE_LAMPARRAY_LAMP_RED_UPDATE_CHANNEL: u16 = 81;
pub const HID_USAGE_LAMPARRAY_LAMP_UPDATE_FLAGS: u16 = 85;
pub const HID_USAGE_LAMPARRAY_MIN_UPDATE_INTERVAL_IN_MICROSECONDS: u16 = 8;
pub const HID_USAGE_LAMPARRAY_POSITION_X_IN_MICROMETERS: u16 = 35;
pub const HID_USAGE_LAMPARRAY_POSITION_Y_IN_MICROMETERS: u16 = 36;
pub const HID_USAGE_LAMPARRAY_POSITION_Z_IN_MICROMETERS: u16 = 37;
pub const HID_USAGE_LAMPARRAY_RED_LEVEL_COUNT: u16 = 40;
pub const HID_USAGE_LAMPARRAY_UPDATE_LATENCY_IN_MICROSECONDS: u16 = 39;
pub const HID_USAGE_LED_AMBER: u16 = 74;
pub const HID_USAGE_LED_BATTERY_LOW: u16 = 29;
pub const HID_USAGE_LED_BATTERY_OK: u16 = 28;
pub const HID_USAGE_LED_BATTERY_OPERATION: u16 = 27;
pub const HID_USAGE_LED_BLUE_LED_CHANNEL: u16 = 84;
pub const HID_USAGE_LED_BUSY: u16 = 44;
pub const HID_USAGE_LED_CALL_PICKUP: u16 = 37;
pub const HID_USAGE_LED_CAMERA_OFF: u16 = 41;
pub const HID_USAGE_LED_CAMERA_ON: u16 = 40;
pub const HID_USAGE_LED_CAPS_LOCK: u16 = 2;
pub const HID_USAGE_LED_CAV: u16 = 20;
pub const HID_USAGE_LED_CLV: u16 = 21;
pub const HID_USAGE_LED_COMPOSE: u16 = 4;
pub const HID_USAGE_LED_CONFERENCE: u16 = 38;
pub const HID_USAGE_LED_COVERAGE: u16 = 34;
pub const HID_USAGE_LED_DATA_MODE: u16 = 26;
pub const HID_USAGE_LED_DO_NOT_DISTURB: u16 = 8;
pub const HID_USAGE_LED_EQUALIZER_ENABLE: u16 = 13;
pub const HID_USAGE_LED_ERROR: u16 = 57;
pub const HID_USAGE_LED_EXTERNAL_POWER: u16 = 77;
pub const HID_USAGE_LED_FAST_BLINK_OFF_TIME: u16 = 70;
pub const HID_USAGE_LED_FAST_BLINK_ON_TIME: u16 = 69;
pub const HID_USAGE_LED_FAST_FORWARD: u16 = 53;
pub const HID_USAGE_LED_FLASH_ON_TIME: u16 = 66;
pub const HID_USAGE_LED_FORWARD: u16 = 49;
pub const HID_USAGE_LED_GENERIC_INDICATOR: u16 = 75;
pub const HID_USAGE_LED_GOOD_STATUS: u16 = 80;
pub const HID_USAGE_LED_GREEN: u16 = 73;
pub const HID_USAGE_LED_GREEN_LED_CHANNEL: u16 = 85;
pub const HID_USAGE_LED_HEAD_SET: u16 = 31;
pub const HID_USAGE_LED_HIGH_CUT_FILTER: u16 = 11;
pub const HID_USAGE_LED_HOLD: u16 = 32;
pub const HID_USAGE_LED_INDICATOR_BLUE: u16 = 78;
pub const HID_USAGE_LED_INDICATOR_COLOR: u16 = 71;
pub const HID_USAGE_LED_INDICATOR_FAST_BLINK: u16 = 64;
pub const HID_USAGE_LED_INDICATOR_FLASH: u16 = 62;
pub const HID_USAGE_LED_INDICATOR_OFF: u16 = 65;
pub const HID_USAGE_LED_INDICATOR_ON: u16 = 61;
pub const HID_USAGE_LED_INDICATOR_ORANGE: u16 = 79;
pub const HID_USAGE_LED_INDICATOR_SLOW_BLINK: u16 = 63;
pub const HID_USAGE_LED_IN_USE_INDICATOR: u16 = 59;
pub const HID_USAGE_LED_KANA: u16 = 5;
pub const HID_USAGE_LED_LED_INTENSITY: u16 = 86;
pub const HID_USAGE_LED_LOW_CUT_FILTER: u16 = 12;
pub const HID_USAGE_LED_MESSAGE_WAITING: u16 = 25;
pub const HID_USAGE_LED_MICROPHONE: u16 = 33;
pub const HID_USAGE_LED_MULTI_MODE_INDICATOR: u16 = 60;
pub const HID_USAGE_LED_MUTE: u16 = 9;
pub const HID_USAGE_LED_NIGHT_MODE: u16 = 35;
pub const HID_USAGE_LED_NUM_LOCK: u16 = 1;
pub const HID_USAGE_LED_OFF_HOOK: u16 = 23;
pub const HID_USAGE_LED_OFF_LINE: u16 = 43;
pub const HID_USAGE_LED_ON_LINE: u16 = 42;
pub const HID_USAGE_LED_PAPER_JAM: u16 = 47;
pub const HID_USAGE_LED_PAPER_OUT: u16 = 46;
pub const HID_USAGE_LED_PAUSE: u16 = 55;
pub const HID_USAGE_LED_PLAY: u16 = 54;
pub const HID_USAGE_LED_PLAYER_1: u16 = 97;
pub const HID_USAGE_LED_PLAYER_2: u16 = 98;
pub const HID_USAGE_LED_PLAYER_3: u16 = 99;
pub const HID_USAGE_LED_PLAYER_4: u16 = 100;
pub const HID_USAGE_LED_PLAYER_5: u16 = 101;
pub const HID_USAGE_LED_PLAYER_6: u16 = 102;
pub const HID_USAGE_LED_PLAYER_7: u16 = 103;
pub const HID_USAGE_LED_PLAYER_8: u16 = 104;
pub const HID_USAGE_LED_PLAYER_INDICATOR: u16 = 96;
pub const HID_USAGE_LED_POWER: u16 = 6;
pub const HID_USAGE_LED_READY: u16 = 45;
pub const HID_USAGE_LED_RECORD: u16 = 56;
pub const HID_USAGE_LED_RECORDING_FORMAT_DET: u16 = 22;
pub const HID_USAGE_LED_RED: u16 = 72;
pub const HID_USAGE_LED_RED_LED_CHANNEL: u16 = 83;
pub const HID_USAGE_LED_REMOTE: u16 = 48;
pub const HID_USAGE_LED_REPEAT: u16 = 16;
pub const HID_USAGE_LED_REVERSE: u16 = 50;
pub const HID_USAGE_LED_REWIND: u16 = 52;
pub const HID_USAGE_LED_RGB_LED: u16 = 82;
pub const HID_USAGE_LED_RING: u16 = 24;
pub const HID_USAGE_LED_SAMPLING_RATE_DETECT: u16 = 18;
pub const HID_USAGE_LED_SCROLL_LOCK: u16 = 3;
pub const HID_USAGE_LED_SELECTED_INDICATOR: u16 = 58;
pub const HID_USAGE_LED_SEND_CALLS: u16 = 36;
pub const HID_USAGE_LED_SHIFT: u16 = 7;
pub const HID_USAGE_LED_SLOW_BLINK_OFF_TIME: u16 = 68;
pub const HID_USAGE_LED_SLOW_BLINK_ON_TIME: u16 = 67;
pub const HID_USAGE_LED_SOUND_FIELD_ON: u16 = 14;
pub const HID_USAGE_LED_SPEAKER: u16 = 30;
pub const HID_USAGE_LED_SPINNING: u16 = 19;
pub const HID_USAGE_LED_STAND_BY: u16 = 39;
pub const HID_USAGE_LED_STEREO: u16 = 17;
pub const HID_USAGE_LED_STOP: u16 = 51;
pub const HID_USAGE_LED_SURROUND_FIELD_ON: u16 = 15;
pub const HID_USAGE_LED_SYSTEM_MICROPHONE_MUTE: u16 = 87;
pub const HID_USAGE_LED_SYSTEM_SUSPEND: u16 = 76;
pub const HID_USAGE_LED_TONE_ENABLE: u16 = 10;
pub const HID_USAGE_LED_WARNING_STATUS: u16 = 81;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_MSR_DEVICE_READONLY: u16 = 1;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_1_DATA: u16 = 33;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_1_LENGTH: u16 = 17;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_2_DATA: u16 = 34;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_2_LENGTH: u16 = 18;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_3_DATA: u16 = 35;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_3_LENGTH: u16 = 19;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_DATA: u16 = 32;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_JIS_DATA: u16 = 36;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_JIS_LENGTH: u16 = 20;
pub const HID_USAGE_MEDICAL_INSTRUMENT_2D_MODE_ADJUST: u16 = 137;
pub const HID_USAGE_MEDICAL_INSTRUMENT_2D_MODE_SELECT: u16 = 136;
pub const HID_USAGE_MEDICAL_INSTRUMENT_CINE: u16 = 64;
pub const HID_USAGE_MEDICAL_INSTRUMENT_CLIP_STORE: u16 = 34;
pub const HID_USAGE_MEDICAL_INSTRUMENT_COLOR_DOPPLER_ADJUST: u16 = 133;
pub const HID_USAGE_MEDICAL_INSTRUMENT_COLOR_DOPPLER_MODE_SELECT: u16 = 132;
pub const HID_USAGE_MEDICAL_INSTRUMENT_DEPTH: u16 = 68;
pub const HID_USAGE_MEDICAL_INSTRUMENT_DEPTH_GAIN_COMPENSATION: u16 = 112;
pub const HID_USAGE_MEDICAL_INSTRUMENT_FOCUS: u16 = 67;
pub const HID_USAGE_MEDICAL_INSTRUMENT_FREEZETHAW: u16 = 33;
pub const HID_USAGE_MEDICAL_INSTRUMENT_MEDICAL_ULTRASOUND: u16 = 1;
pub const HID_USAGE_MEDICAL_INSTRUMENT_MICROPHONE_ENABLE: u16 = 39;
pub const HID_USAGE_MEDICAL_INSTRUMENT_MOTION_MODE_ADJUST: u16 = 135;
pub const HID_USAGE_MEDICAL_INSTRUMENT_MOTION_MODE_SELECT: u16 = 134;
pub const HID_USAGE_MEDICAL_INSTRUMENT_NEXT: u16 = 36;
pub const HID_USAGE_MEDICAL_INSTRUMENT_PRINT: u16 = 38;
pub const HID_USAGE_MEDICAL_INSTRUMENT_SAVE: u16 = 37;
pub const HID_USAGE_MEDICAL_INSTRUMENT_SOFT_CONTROL_ADJUST: u16 = 161;
pub const HID_USAGE_MEDICAL_INSTRUMENT_SOFT_CONTROL_SELECT: u16 = 160;
pub const HID_USAGE_MEDICAL_INSTRUMENT_SOFT_STEP__PRIMARY: u16 = 96;
pub const HID_USAGE_MEDICAL_INSTRUMENT_SOFT_STEP__SECONDARY: u16 = 97;
pub const HID_USAGE_MEDICAL_INSTRUMENT_SPECTRAL_DOPPLER_ADJUST: u16 = 131;
pub const HID_USAGE_MEDICAL_INSTRUMENT_SPECTRAL_DOPPLER_MODE_SELECT: u16 = 130;
pub const HID_USAGE_MEDICAL_INSTRUMENT_TRANSMIT_POWER: u16 = 65;
pub const HID_USAGE_MEDICAL_INSTRUMENT_UPDATE: u16 = 35;
pub const HID_USAGE_MEDICAL_INSTRUMENT_VCRACQUISITION: u16 = 32;
pub const HID_USAGE_MEDICAL_INSTRUMENT_VOLUME: u16 = 66;
pub const HID_USAGE_MEDICAL_INSTRUMENT_ZOOM_ADJUST: u16 = 129;
pub const HID_USAGE_MEDICAL_INSTRUMENT_ZOOM_SELECT: u16 = 128;
pub const HID_USAGE_MICROSOFT_BLUETOOTH_HANDSFREE_CALL_SETUP: u16 = 36;
pub const HID_USAGE_MICROSOFT_BLUETOOTH_HANDSFREE_DIAL_MEMORY: u16 = 34;
pub const HID_USAGE_MICROSOFT_BLUETOOTH_HANDSFREE_DIAL_NUMBER: u16 = 33;
pub const HID_USAGE_MICROSOFT_BLUETOOTH_HANDSFREE_GENERAL: u16 = 1;
pub const HID_USAGE_MONITOR_EDID_INFORMATION: u16 = 2;
pub const HID_USAGE_MONITOR_MONITOR_CONTROL: u16 = 1;
pub const HID_USAGE_MONITOR_VDIF_INFORMATION: u16 = 3;
pub const HID_USAGE_MONITOR_VESA_VERSION: u16 = 4;
pub const HID_USAGE_PAGE_ALPHANUMERIC: u16 = 20;
pub const HID_USAGE_PAGE_ARCADE: u16 = 145;
pub const HID_USAGE_PAGE_BARCODE_SCANNER: u16 = 140;
pub const HID_USAGE_PAGE_BATTERY_SYSTEM: u16 = 133;
pub const HID_USAGE_PAGE_BRAILLE_DISPLAY: u16 = 65;
pub const HID_USAGE_PAGE_BUTTON: u16 = 9;
pub const HID_USAGE_PAGE_CAMERA_CONTROL: u16 = 144;
pub const HID_USAGE_PAGE_CONSUMER: u16 = 12;
pub const HID_USAGE_PAGE_DIGITIZER: u16 = 13;
pub const HID_USAGE_PAGE_EYE_AND_HEAD_TRACKERS: u16 = 18;
pub const HID_USAGE_PAGE_FIDO_ALLIANCE: u16 = 61904;
pub const HID_USAGE_PAGE_GAME: u16 = 5;
pub const HID_USAGE_PAGE_GAMING_DEVICE: u16 = 146;
pub const HID_USAGE_PAGE_GENERIC: u16 = 1;
pub const HID_USAGE_PAGE_GENERIC_DEVICE: u16 = 6;
pub const HID_USAGE_PAGE_HAPTICS: u16 = 14;
pub const HID_USAGE_PAGE_KEYBOARD: u16 = 7;
pub const HID_USAGE_PAGE_LED: u16 = 8;
pub const HID_USAGE_PAGE_LIGHTING_ILLUMINATION: u16 = 89;
pub const HID_USAGE_PAGE_MAGNETIC_STRIPE_READER: u16 = 142;
pub const HID_USAGE_PAGE_MEDICAL_INSTRUMENT: u16 = 64;
pub const HID_USAGE_PAGE_MICROSOFT_BLUETOOTH_HANDSFREE: u16 = 65523;
pub const HID_USAGE_PAGE_MONITOR: u16 = 128;
pub const HID_USAGE_PAGE_MONITOR_ENUMERATED: u16 = 129;
pub const HID_USAGE_PAGE_ORDINAL: u16 = 10;
pub const HID_USAGE_PAGE_PID: u16 = 15;
pub const HID_USAGE_PAGE_POWER: u16 = 132;
pub const HID_USAGE_PAGE_SENSOR: u16 = 32;
pub const HID_USAGE_PAGE_SIMULATION: u16 = 2;
pub const HID_USAGE_PAGE_SOC: u16 = 17;
pub const HID_USAGE_PAGE_SPORT: u16 = 4;
pub const HID_USAGE_PAGE_TELEPHONY: u16 = 11;
pub const HID_USAGE_PAGE_UNDEFINED: u16 = 0;
pub const HID_USAGE_PAGE_UNICODE: u16 = 16;
pub const HID_USAGE_PAGE_VENDOR_DEFINED_BEGIN: u16 = 65280;
pub const HID_USAGE_PAGE_VENDOR_DEFINED_END: u16 = 65535;
pub const HID_USAGE_PAGE_VESA_VIRTUAL_CONTROLS: u16 = 130;
pub const HID_USAGE_PAGE_VR: u16 = 3;
pub const HID_USAGE_PAGE_WEIGHING_DEVICE: u16 = 141;
pub const HID_USAGE_PID_ACTUATORS_ENABLED: u16 = 160;
pub const HID_USAGE_PID_ACTUATOR_OVERRIDE_SWITCH: u16 = 165;
pub const HID_USAGE_PID_ACTUATOR_POWER: u16 = 166;
pub const HID_USAGE_PID_ATTACK_LEVEL: u16 = 91;
pub const HID_USAGE_PID_ATTACK_TIME: u16 = 92;
pub const HID_USAGE_PID_AXES_ENABLE: u16 = 85;
pub const HID_USAGE_PID_BLOCK_FREE_REPORT: u16 = 144;
pub const HID_USAGE_PID_BLOCK_HANDLE: u16 = 143;
pub const HID_USAGE_PID_BLOCK_LOAD_ERROR: u16 = 142;
pub const HID_USAGE_PID_BLOCK_LOAD_FULL: u16 = 141;
pub const HID_USAGE_PID_BLOCK_LOAD_REPORT: u16 = 137;
pub const HID_USAGE_PID_BLOCK_LOAD_STATUS: u16 = 139;
pub const HID_USAGE_PID_BLOCK_LOAD_SUCCESS: u16 = 140;
pub const HID_USAGE_PID_BLOCK_TYPE: u16 = 89;
pub const HID_USAGE_PID_CP_OFFSET: u16 = 96;
pub const HID_USAGE_PID_CREATE_NEW_EFFECT: u16 = 171;
pub const HID_USAGE_PID_CUSTOM_FORCE_DATA: u16 = 105;
pub const HID_USAGE_PID_CUSTOM_FORCE_DATA_OFFSET: u16 = 108;
pub const HID_USAGE_PID_CUSTOM_FORCE_DATA_REPORT: u16 = 104;
pub const HID_USAGE_PID_CUSTOM_FORCE_VENDOR_DEFINED_DATA: u16 = 106;
pub const HID_USAGE_PID_DC_DEVICE_CONTINUE: u16 = 156;
pub const HID_USAGE_PID_DC_DEVICE_PAUSE: u16 = 155;
pub const HID_USAGE_PID_DC_DEVICE_RESET: u16 = 154;
pub const HID_USAGE_PID_DC_DISABLE_ACTUATORS: u16 = 152;
pub const HID_USAGE_PID_DC_ENABLE_ACTUATORS: u16 = 151;
pub const HID_USAGE_PID_DC_STOP_ALL_EFFECTS: u16 = 153;
pub const HID_USAGE_PID_DEAD_BAND: u16 = 101;
pub const HID_USAGE_PID_DEVICE_CONTROL: u16 = 150;
pub const HID_USAGE_PID_DEVICE_GAIN: u16 = 126;
pub const HID_USAGE_PID_DEVICE_GAIN_REPORT: u16 = 125;
pub const HID_USAGE_PID_DEVICE_MANAGED_POOL: u16 = 169;
pub const HID_USAGE_PID_DEVICE_PAUSED: u16 = 159;
pub const HID_USAGE_PID_DIRECTION: u16 = 87;
pub const HID_USAGE_PID_DIRECTION_ENABLE: u16 = 86;
pub const HID_USAGE_PID_DOWNLOAD_FORCE_SAMPLE: u16 = 102;
pub const HID_USAGE_PID_DURATION: u16 = 80;
pub const HID_USAGE_PID_EFFECT_BLOCK_INDEX: u16 = 34;
pub const HID_USAGE_PID_EFFECT_OPERATION: u16 = 120;
pub const HID_USAGE_PID_EFFECT_OPERATION_REPORT: u16 = 119;
pub const HID_USAGE_PID_EFFECT_PLAYING: u16 = 148;
pub const HID_USAGE_PID_EFFECT_TYPE: u16 = 37;
pub const HID_USAGE_PID_ET_CONSTANT: u16 = 38;
pub const HID_USAGE_PID_ET_CUSTOM: u16 = 40;
pub const HID_USAGE_PID_ET_DAMPER: u16 = 65;
pub const HID_USAGE_PID_ET_FRICTION: u16 = 67;
pub const HID_USAGE_PID_ET_INERTIA: u16 = 66;
pub const HID_USAGE_PID_ET_RAMP: u16 = 39;
pub const HID_USAGE_PID_ET_SAWTOOTH_DOWN: u16 = 52;
pub const HID_USAGE_PID_ET_SAWTOOTH_UP: u16 = 51;
pub const HID_USAGE_PID_ET_SINE: u16 = 49;
pub const HID_USAGE_PID_ET_SPRING: u16 = 64;
pub const HID_USAGE_PID_ET_SQUARE: u16 = 48;
pub const HID_USAGE_PID_ET_TRIANGLE: u16 = 50;
pub const HID_USAGE_PID_FADE_LEVEL: u16 = 93;
pub const HID_USAGE_PID_FADE_TIME: u16 = 94;
pub const HID_USAGE_PID_GAIN: u16 = 82;
pub const HID_USAGE_PID_ISOCH_CUSTOMFORCE_ENABLE: u16 = 103;
pub const HID_USAGE_PID_LOOP_COUNT: u16 = 124;
pub const HID_USAGE_PID_MAGNITUDE: u16 = 112;
pub const HID_USAGE_PID_MOVE_DESTINATION: u16 = 135;
pub const HID_USAGE_PID_MOVE_LENGTH: u16 = 136;
pub const HID_USAGE_PID_MOVE_SOURCE: u16 = 134;
pub const HID_USAGE_PID_NEGATIVE_COEFFICIENT: u16 = 98;
pub const HID_USAGE_PID_NEGATIVE_SATURATION: u16 = 100;
pub const HID_USAGE_PID_NORMAL: u16 = 32;
pub const HID_USAGE_PID_OFFSET: u16 = 111;
pub const HID_USAGE_PID_OP_EFFECT_START: u16 = 121;
pub const HID_USAGE_PID_OP_EFFECT_START_SOLO: u16 = 122;
pub const HID_USAGE_PID_OP_EFFECT_STOP: u16 = 123;
pub const HID_USAGE_PID_PARAMETER_BLOCK_MOVE_REPORT: u16 = 133;
pub const HID_USAGE_PID_PARAMETER_BLOCK_OFFSET: u16 = 35;
pub const HID_USAGE_PID_PARAMETER_BLOCK_SIZE: u16 = 168;
pub const HID_USAGE_PID_PERIOD: u16 = 114;
pub const HID_USAGE_PID_PHASE: u16 = 113;
pub const HID_USAGE_PID_PHYSICAL_INPUT_DEVICE: u16 = 1;
pub const HID_USAGE_PID_PID_DEVICE_CONTROL_REPORT: u16 = 149;
pub const HID_USAGE_PID_POOL_ALIGNMENT: u16 = 132;
pub const HID_USAGE_PID_POOL_REPORT: u16 = 127;
pub const HID_USAGE_PID_POSITIVE_COEFFICIENT: u16 = 97;
pub const HID_USAGE_PID_POSITIVE_SATURATION: u16 = 99;
pub const HID_USAGE_PID_RAMPOOL_AVAILABLE: u16 = 172;
pub const HID_USAGE_PID_RAMP_END: u16 = 118;
pub const HID_USAGE_PID_RAMP_START: u16 = 117;
pub const HID_USAGE_PID_RAM_POOL_SIZE: u16 = 128;
pub const HID_USAGE_PID_ROM_EFFECT_BLOCK_COUNT: u16 = 130;
pub const HID_USAGE_PID_ROM_FLAG: u16 = 36;
pub const HID_USAGE_PID_ROM_POOL_SIZE: u16 = 129;
pub const HID_USAGE_PID_SAFETY_SWITCH: u16 = 164;
pub const HID_USAGE_PID_SAMPLE_COUNT: u16 = 109;
pub const HID_USAGE_PID_SAMPLE_PERIOD: u16 = 81;
pub const HID_USAGE_PID_SET_CONDITION_REPORT: u16 = 95;
pub const HID_USAGE_PID_SET_CONSTANT_FORCE_REPORT: u16 = 115;
pub const HID_USAGE_PID_SET_CUSTOM_FORCE_REPORT: u16 = 107;
pub const HID_USAGE_PID_SET_EFFECT_REPORT: u16 = 33;
pub const HID_USAGE_PID_SET_ENVELOPE_REPORT: u16 = 90;
pub const HID_USAGE_PID_SET_PERIODIC_REPORT: u16 = 110;
pub const HID_USAGE_PID_SET_RAMP_FORCE_REPORT: u16 = 116;
pub const HID_USAGE_PID_SHARED_PARAMETER_BLOCKS: u16 = 170;
pub const HID_USAGE_PID_SIMULTANEOUS_EFFECTS_MAX: u16 = 131;
pub const HID_USAGE_PID_START_DELAY: u16 = 167;
pub const HID_USAGE_PID_STATE_REPORT: u16 = 146;
pub const HID_USAGE_PID_TRIGGER_BUTTON: u16 = 83;
pub const HID_USAGE_PID_TRIGGER_REPEAT_INTERVAL: u16 = 84;
pub const HID_USAGE_PID_TYPE_SPECIFIC_BLOCK_HANDLE: u16 = 145;
pub const HID_USAGE_PID_TYPE_SPECIFIC_BLOCK_OFFSET: u16 = 88;
pub const HID_USAGE_POWER_ACTIVE_POWER: u16 = 52;
pub const HID_USAGE_POWER_APPARENT_POWER: u16 = 51;
pub const HID_USAGE_POWER_AUDIBLE_ALARM_CONTROL: u16 = 90;
pub const HID_USAGE_POWER_AWAITING_POWER: u16 = 114;
pub const HID_USAGE_POWER_BAD_COUNT: u16 = 56;
pub const HID_USAGE_POWER_BATTERY: u16 = 18;
pub const HID_USAGE_POWER_BATTERY_ID: u16 = 19;
pub const HID_USAGE_POWER_BATTERY_SYSTEM: u16 = 16;
pub const HID_USAGE_POWER_BATTERY_SYSTEM_ID: u16 = 17;
pub const HID_USAGE_POWER_BOOST: u16 = 110;
pub const HID_USAGE_POWER_BUCK: u16 = 111;
pub const HID_USAGE_POWER_CHANGED_STATUS: u16 = 3;
pub const HID_USAGE_POWER_CHARGER: u16 = 20;
pub const HID_USAGE_POWER_CHARGER_ID: u16 = 21;
pub const HID_USAGE_POWER_COMMUNICATION_LOST: u16 = 115;
pub const HID_USAGE_POWER_CONFIG_ACTIVE_POWER: u16 = 68;
pub const HID_USAGE_POWER_CONFIG_APPARENT_POWER: u16 = 67;
pub const HID_USAGE_POWER_CONFIG_CURRENT: u16 = 65;
pub const HID_USAGE_POWER_CONFIG_FREQUENCY: u16 = 66;
pub const HID_USAGE_POWER_CONFIG_HUMIDITY: u16 = 71;
pub const HID_USAGE_POWER_CONFIG_PERCENT_LOAD: u16 = 69;
pub const HID_USAGE_POWER_CONFIG_TEMPERATURE: u16 = 70;
pub const HID_USAGE_POWER_CONFIG_VOLTAGE: u16 = 64;
pub const HID_USAGE_POWER_CURRENT: u16 = 49;
pub const HID_USAGE_POWER_DELAY_BEFORE_REBOOT: u16 = 85;
pub const HID_USAGE_POWER_DELAY_BEFORE_SHUTDOWN: u16 = 87;
pub const HID_USAGE_POWER_DELAY_BEFORE_STARTUP: u16 = 86;
pub const HID_USAGE_POWER_FLOW: u16 = 30;
pub const HID_USAGE_POWER_FLOW_ID: u16 = 31;
pub const HID_USAGE_POWER_FREQUENCY: u16 = 50;
pub const HID_USAGE_POWER_FREQUENCY_OUT_OF_RANGE: u16 = 100;
pub const HID_USAGE_POWER_GANG: u16 = 34;
pub const HID_USAGE_POWER_GANG_ID: u16 = 35;
pub const HID_USAGE_POWER_GOOD: u16 = 97;
pub const HID_USAGE_POWER_HIGH_VOLTAGE_TRANSFER: u16 = 84;
pub const HID_USAGE_POWER_HUMIDITY: u16 = 55;
pub const HID_USAGE_POWER_IMANUFACTURER: u16 = 253;
pub const HID_USAGE_POWER_INAME: u16 = 1;
pub const HID_USAGE_POWER_INITIALIZED: u16 = 112;
pub const HID_USAGE_POWER_INPUT: u16 = 26;
pub const HID_USAGE_POWER_INPUT_ID: u16 = 27;
pub const HID_USAGE_POWER_INTERNAL_FAILURE: u16 = 98;
pub const HID_USAGE_POWER_IPRODUCT: u16 = 254;
pub const HID_USAGE_POWER_ISERIALNUMBER: u16 = 255;
pub const HID_USAGE_POWER_LOW_VOLTAGE_TRANSFER: u16 = 83;
pub const HID_USAGE_POWER_MODULE_RESET: u16 = 89;
pub const HID_USAGE_POWER_OUTLET: u16 = 32;
pub const HID_USAGE_POWER_OUTLET_ID: u16 = 33;
pub const HID_USAGE_POWER_OUTLET_SYSTEM: u16 = 24;
pub const HID_USAGE_POWER_OUTLET_SYSTEM_ID: u16 = 25;
pub const HID_USAGE_POWER_OUTPUT: u16 = 28;
pub const HID_USAGE_POWER_OUTPUT_ID: u16 = 29;
pub const HID_USAGE_POWER_OVERLOAD: u16 = 101;
pub const HID_USAGE_POWER_OVER_CHARGED: u16 = 102;
pub const HID_USAGE_POWER_OVER_TEMPERATURE: u16 = 103;
pub const HID_USAGE_POWER_PERCENT_LOAD: u16 = 53;
pub const HID_USAGE_POWER_POWER_CONVERTER: u16 = 22;
pub const HID_USAGE_POWER_POWER_CONVERTER_ID: u16 = 23;
pub const HID_USAGE_POWER_POWER_SUMMARY: u16 = 36;
pub const HID_USAGE_POWER_POWER_SUMMARY_ID: u16 = 37;
pub const HID_USAGE_POWER_POWER_SUPPLY: u16 = 5;
pub const HID_USAGE_POWER_PRESENT: u16 = 96;
pub const HID_USAGE_POWER_PRESENT_STATUS: u16 = 2;
pub const HID_USAGE_POWER_SHUTDOWN_IMMINENT: u16 = 105;
pub const HID_USAGE_POWER_SHUTDOWN_REQUESTED: u16 = 104;
pub const HID_USAGE_POWER_SWITCHABLE: u16 = 108;
pub const HID_USAGE_POWER_SWITCH_OFF_CONTROL: u16 = 81;
pub const HID_USAGE_POWER_SWITCH_ONOFF: u16 = 107;
pub const HID_USAGE_POWER_SWITCH_ON_CONTROL: u16 = 80;
pub const HID_USAGE_POWER_TEMPERATURE: u16 = 54;
pub const HID_USAGE_POWER_TEST: u16 = 88;
pub const HID_USAGE_POWER_TESTED: u16 = 113;
pub const HID_USAGE_POWER_TOGGLE_CONTROL: u16 = 82;
pub const HID_USAGE_POWER_UPS: u16 = 4;
pub const HID_USAGE_POWER_USED: u16 = 109;
pub const HID_USAGE_POWER_VOLTAGE: u16 = 48;
pub const HID_USAGE_POWER_VOLTAG_OUT_OF_RANGE: u16 = 99;
pub const HID_USAGE_SENSORS_ACCURACY_DEFAULT: u16 = 2144;
pub const HID_USAGE_SENSORS_ACCURACY_HIGH: u16 = 2145;
pub const HID_USAGE_SENSORS_ACCURACY_LOW: u16 = 2147;
pub const HID_USAGE_SENSORS_ACCURACY_MEDIUM: u16 = 2146;
pub const HID_USAGE_SENSORS_ACTIVITY_STATE_END_ACTIVITY: u16 = 2402;
pub const HID_USAGE_SENSORS_ACTIVITY_STATE_NO_STATE_CHANGE: u16 = 2400;
pub const HID_USAGE_SENSORS_ACTIVITY_STATE_START_ACTIVITY: u16 = 2401;
pub const HID_USAGE_SENSORS_ACTIVITY_TYPE_BIKING: u16 = 2358;
pub const HID_USAGE_SENSORS_ACTIVITY_TYPE_FIDGETING: u16 = 2354;
pub const HID_USAGE_SENSORS_ACTIVITY_TYPE_IDLE: u16 = 2359;
pub const HID_USAGE_SENSORS_ACTIVITY_TYPE_IN_VEHICLE: u16 = 2357;
pub const HID_USAGE_SENSORS_ACTIVITY_TYPE_RUNNING: u16 = 2356;
pub const HID_USAGE_SENSORS_ACTIVITY_TYPE_STATIONARY: u16 = 2353;
pub const HID_USAGE_SENSORS_ACTIVITY_TYPE_UNKNOWN: u16 = 2352;
pub const HID_USAGE_SENSORS_ACTIVITY_TYPE_WALKING: u16 = 2355;
pub const HID_USAGE_SENSORS_BIOMETRIC: u16 = 16;
pub const HID_USAGE_SENSORS_BIOMETRIC_BLOOD_PRESSURE: u16 = 20;
pub const HID_USAGE_SENSORS_BIOMETRIC_BODY_TEMPERATURE: u16 = 21;
pub const HID_USAGE_SENSORS_BIOMETRIC_HEART_RATE: u16 = 22;
pub const HID_USAGE_SENSORS_BIOMETRIC_HEART_RATE_VARIABILITY: u16 = 23;
pub const HID_USAGE_SENSORS_BIOMETRIC_HUMAN_PRESENCE: u16 = 17;
pub const HID_USAGE_SENSORS_BIOMETRIC_HUMAN_PROXIMITY: u16 = 18;
pub const HID_USAGE_SENSORS_BIOMETRIC_HUMAN_TOUCH: u16 = 19;
pub const HID_USAGE_SENSORS_BIOMETRIC_PERIPHERAL_OXYGEN_SATURATION: u16 = 24;
pub const HID_USAGE_SENSORS_BIOMETRIC_RESPIRATORY_RATE: u16 = 25;
pub const HID_USAGE_SENSORS_CONNECTION_TYPE_PC_ATTACHED: u16 = 2097;
pub const HID_USAGE_SENSORS_CONNECTION_TYPE_PC_EXTERNAL: u16 = 2098;
pub const HID_USAGE_SENSORS_CONNECTION_TYPE_PC_INTEGRATED: u16 = 2096;
pub const HID_USAGE_SENSORS_DATA_FIELD_ABSOLUTE_PRESSURE: u16 = 1173;
pub const HID_USAGE_SENSORS_DATA_FIELD_ACCELERATION: u16 = 1106;
pub const HID_USAGE_SENSORS_DATA_FIELD_ACCELERATION_AXIS_X: u16 = 1107;
pub const HID_USAGE_SENSORS_DATA_FIELD_ACCELERATION_AXIS_Y: u16 = 1108;
pub const HID_USAGE_SENSORS_DATA_FIELD_ACCELERATION_AXIS_Z: u16 = 1109;
pub const HID_USAGE_SENSORS_DATA_FIELD_ACTIVITY_STATE: u16 = 1426;
pub const HID_USAGE_SENSORS_DATA_FIELD_ACTIVITY_TYPE: u16 = 1425;
pub const HID_USAGE_SENSORS_DATA_FIELD_ADDRESS_LINE_1: u16 = 1058;
pub const HID_USAGE_SENSORS_DATA_FIELD_ADDRESS_LINE_2: u16 = 1059;
pub const HID_USAGE_SENSORS_DATA_FIELD_AIR_QUALITY_INDEX: u16 = 1079;
pub const HID_USAGE_SENSORS_DATA_FIELD_ALTITUDE_ANTENNA_SEA_LEVEL: u16 = 1026;
pub const HID_USAGE_SENSORS_DATA_FIELD_ALTITUDE_ELLIPSOID: u16 = 1029;
pub const HID_USAGE_SENSORS_DATA_FIELD_ALTITUDE_ELLIPSOID_ERROR: u16 = 1028;
pub const HID_USAGE_SENSORS_DATA_FIELD_ALTITUDE_SEA_LEVEL: u16 = 1031;
pub const HID_USAGE_SENSORS_DATA_FIELD_ALTITUDE_SEA_LEVEL_ERROR: u16 = 1030;
pub const HID_USAGE_SENSORS_DATA_FIELD_ANGULAR_POSITION: u16 = 1114;
pub const HID_USAGE_SENSORS_DATA_FIELD_ANGULAR_POSITION_ABOUT_X_AXIS: u16 = 1115;
pub const HID_USAGE_SENSORS_DATA_FIELD_ANGULAR_POSITION_ABOUT_Y_AXIS: u16 = 1116;
pub const HID_USAGE_SENSORS_DATA_FIELD_ANGULAR_POSITION_ABOUT_Z_AXIS: u16 = 1117;
pub const HID_USAGE_SENSORS_DATA_FIELD_ANGULAR_VELOCITY: u16 = 1110;
pub const HID_USAGE_SENSORS_DATA_FIELD_ANGULAR_VELOCITY_ABOUT_X_AXIS: u16 = 1111;
pub const HID_USAGE_SENSORS_DATA_FIELD_ANGULAR_VELOCITY_ABOUT_Y_AXIS: u16 = 1112;
pub const HID_USAGE_SENSORS_DATA_FIELD_ANGULAR_VELOCITY_ABOUT_Z_AXIS: u16 = 1113;
pub const HID_USAGE_SENSORS_DATA_FIELD_ATMOSPHERIC_PRESSURE: u16 = 1073;
pub const HID_USAGE_SENSORS_DATA_FIELD_BIOMETRIC: u16 = 1200;
pub const HID_USAGE_SENSORS_DATA_FIELD_BLOOD_PRESSURE: u16 = 1205;
pub const HID_USAGE_SENSORS_DATA_FIELD_BLOOD_PRESSURE_DIASTOLIC: u16 = 1206;
pub const HID_USAGE_SENSORS_DATA_FIELD_BLOOD_PRESSURE_SYSTOLIC: u16 = 1207;
pub const HID_USAGE_SENSORS_DATA_FIELD_BLUE_LIGHT: u16 = 1242;
pub const HID_USAGE_SENSORS_DATA_FIELD_BOOLEAN_SWITCH_ARRAY_STATES: u16 = 1170;
pub const HID_USAGE_SENSORS_DATA_FIELD_BOOLEAN_SWITCH_STATE: u16 = 1169;
pub const HID_USAGE_SENSORS_DATA_FIELD_CAPACITANCE: u16 = 1281;
pub const HID_USAGE_SENSORS_DATA_FIELD_CHROMATICITY: u16 = 1235;
pub const HID_USAGE_SENSORS_DATA_FIELD_CHROMATICITY_X: u16 = 1236;
pub const HID_USAGE_SENSORS_DATA_FIELD_CHROMATICITY_Y: u16 = 1237;
pub const HID_USAGE_SENSORS_DATA_FIELD_CITY: u16 = 1060;
pub const HID_USAGE_SENSORS_DATA_FIELD_COLOR_TEMPERATURE: u16 = 1234;
pub const HID_USAGE_SENSORS_DATA_FIELD_CONSUMER_IR_SENTENCE_RECEIVE: u16 = 1238;
pub const HID_USAGE_SENSORS_DATA_FIELD_COUNTRY_OR_REGION: u16 = 1062;
pub const HID_USAGE_SENSORS_DATA_FIELD_CURRENT: u16 = 1282;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM: u16 = 1344;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_BOOLEAN_ARRAY: u16 = 1346;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_TYPE_ID: u16 = 1456;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_USAGE: u16 = 1345;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE: u16 = 1347;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_1: u16 = 1348;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_10: u16 = 1357;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_11: u16 = 1358;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_12: u16 = 1359;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_13: u16 = 1360;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_14: u16 = 1361;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_15: u16 = 1362;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_16: u16 = 1363;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_17: u16 = 1364;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_18: u16 = 1365;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_19: u16 = 1366;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_2: u16 = 1349;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_20: u16 = 1367;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_21: u16 = 1368;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_22: u16 = 1369;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_23: u16 = 1370;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_24: u16 = 1371;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_25: u16 = 1372;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_26: u16 = 1373;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_27: u16 = 1374;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_28: u16 = 1375;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_3: u16 = 1350;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_4: u16 = 1351;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_5: u16 = 1352;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_6: u16 = 1353;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_7: u16 = 1354;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_8: u16 = 1355;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_9: u16 = 1356;
pub const HID_USAGE_SENSORS_DATA_FIELD_DAY: u16 = 1315;
pub const HID_USAGE_SENSORS_DATA_FIELD_DAY_OF_WEEK: u16 = 1316;
pub const HID_USAGE_SENSORS_DATA_FIELD_DEVICE_POSITION: u16 = 1427;
pub const HID_USAGE_SENSORS_DATA_FIELD_DIFFERENTIAL_GPS_DATA_AGE: u16 = 1032;
pub const HID_USAGE_SENSORS_DATA_FIELD_DIFFERENTIAL_REFERENCE_STATION_ID: u16 = 1027;
pub const HID_USAGE_SENSORS_DATA_FIELD_DISTANCE: u16 = 1145;
pub const HID_USAGE_SENSORS_DATA_FIELD_DISTANCE_OUTOFRANGE: u16 = 1149;
pub const HID_USAGE_SENSORS_DATA_FIELD_DISTANCE_X_AXIS: u16 = 1146;
pub const HID_USAGE_SENSORS_DATA_FIELD_DISTANCE_Y_AXIS: u16 = 1147;
pub const HID_USAGE_SENSORS_DATA_FIELD_DISTANCE_Z_AXIS: u16 = 1148;
pub const HID_USAGE_SENSORS_DATA_FIELD_ELECTRICAL: u16 = 1280;
pub const HID_USAGE_SENSORS_DATA_FIELD_ELECTRICAL_POWER: u16 = 1283;
pub const HID_USAGE_SENSORS_DATA_FIELD_ENUMERATOR_TABLE_ROW_COUNT: u16 = 1387;
pub const HID_USAGE_SENSORS_DATA_FIELD_ENUMERATOR_TABLE_ROW_INDEX: u16 = 1386;
pub const HID_USAGE_SENSORS_DATA_FIELD_ENVIRONMENTAL: u16 = 1072;
pub const HID_USAGE_SENSORS_DATA_FIELD_EQUIVALENT_CO2: u16 = 1080;
pub const HID_USAGE_SENSORS_DATA_FIELD_ERROR_RADIUS: u16 = 1033;
pub const HID_USAGE_SENSORS_DATA_FIELD_FIX_QUALITY: u16 = 1034;
pub const HID_USAGE_SENSORS_DATA_FIELD_FIX_TYPE: u16 = 1035;
pub const HID_USAGE_SENSORS_DATA_FIELD_FORCE: u16 = 1172;
pub const HID_USAGE_SENSORS_DATA_FIELD_FREQUENCY: u16 = 1287;
pub const HID_USAGE_SENSORS_DATA_FIELD_GAUGE_PRESSURE: u16 = 1174;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC: u16 = 1376;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_CATEGORY_GUID: u16 = 1378;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_DATA_FIELD: u16 = 1385;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_DATA_FIELD_PROPERTYKEY: u16 = 1382;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_EVENT: u16 = 1383;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_EVENT_PROPERTYKEY: u16 = 1380;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_FIRMWARE_VARTYPE: u16 = 1394;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_GUID: u16 = 1389;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_GUID_OR_PROPERTYKEY: u16 = 1377;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_GUID_OR_PROPERTYKEY_KIND: u16 = 1388;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_PROPERTY: u16 = 1384;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_PROPERTYKEY: u16 = 1390;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_PROPERTY_PROPERTYKEY: u16 = 1381;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_REPORT_COUNT: u16 = 1398;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_REPORT_ID: u16 = 1392;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_REPORT_ITEM_POSITION_INDEX: u16 = 1393;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_REPORT_SIZE: u16 = 1397;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_TOP_LEVEL_COLLECTION_ID: u16 = 1391;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_TYPE_GUID: u16 = 1379;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_UNIT_EXPONENT: u16 = 1396;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_UNIT_OF_MEASURE: u16 = 1395;
pub const HID_USAGE_SENSORS_DATA_FIELD_GEOIDAL_SEPARATION: u16 = 1036;
pub const HID_USAGE_SENSORS_DATA_FIELD_GESTURE_SENSOR: u16 = 1520;
pub const HID_USAGE_SENSORS_DATA_FIELD_GESTURE_STATE: u16 = 1521;
pub const HID_USAGE_SENSORS_DATA_FIELD_GPS_OPERATION_MODE: u16 = 1037;
pub const HID_USAGE_SENSORS_DATA_FIELD_GPS_SELECTION_MODE: u16 = 1038;
pub const HID_USAGE_SENSORS_DATA_FIELD_GPS_STATUS: u16 = 1039;
pub const HID_USAGE_SENSORS_DATA_FIELD_GREEN_LIGHT: u16 = 1241;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEADING: u16 = 1137;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEADING_COMPENSATED_MAGNETIC_NORTH: u16 = 1141;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEADING_COMPENSATED_TRUE_NORTH: u16 = 1142;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEADING_MAGNETIC_NORTH: u16 = 1143;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEADING_TRUE_NORTH: u16 = 1144;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEADING_X_AXIS: u16 = 1138;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEADING_Y_AXIS: u16 = 1139;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEADING_Z_AXIS: u16 = 1140;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEARTBEAT_INTERVAL: u16 = 1210;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEART_RATE: u16 = 1208;
pub const HID_USAGE_SENSORS_DATA_FIELD_HINGE: u16 = 1504;
pub const HID_USAGE_SENSORS_DATA_FIELD_HINGE_ANGLE: u16 = 1505;
pub const HID_USAGE_SENSORS_DATA_FIELD_HINGE_FOLD_CONTRIBUTING_PANEL: u16 = 1524;
pub const HID_USAGE_SENSORS_DATA_FIELD_HINGE_FOLD_FINAL_ANGLE: u16 = 1523;
pub const HID_USAGE_SENSORS_DATA_FIELD_HINGE_FOLD_INITIAL_ANGLE: u16 = 1522;
pub const HID_USAGE_SENSORS_DATA_FIELD_HINGE_FOLD_TYPE: u16 = 1525;
pub const HID_USAGE_SENSORS_DATA_FIELD_HORIZONTAL_DILUTION_OF_PRECISION: u16 = 1041;
pub const HID_USAGE_SENSORS_DATA_FIELD_HOUR: u16 = 1317;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_ATTENTION_DETECTED: u16 = 1213;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_CORRELATION_ID: u16 = 1219;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_HEAD_ALTITUDE: u16 = 1215;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_HEAD_AZIMUTH: u16 = 1214;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_HEAD_PITCH: u16 = 1217;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_HEAD_ROLL: u16 = 1216;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_HEAD_YAW: u16 = 1218;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_PRESENCE: u16 = 1201;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_PROXIMITY_OUT_OF_RANGE: u16 = 1203;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_PROXIMITY_RANGE: u16 = 1202;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_TOUCH_STATE: u16 = 1204;
pub const HID_USAGE_SENSORS_DATA_FIELD_ILLUMINANCE: u16 = 1233;
pub const HID_USAGE_SENSORS_DATA_FIELD_INDUCTANCE: u16 = 1284;
pub const HID_USAGE_SENSORS_DATA_FIELD_INFRARED_LIGHT: u16 = 1239;
pub const HID_USAGE_SENSORS_DATA_FIELD_JULIAN_DAY_OF_YEAR: u16 = 1322;
pub const HID_USAGE_SENSORS_DATA_FIELD_LATITUDE: u16 = 1043;
pub const HID_USAGE_SENSORS_DATA_FIELD_LIGHT: u16 = 1232;
pub const HID_USAGE_SENSORS_DATA_FIELD_LOCATION: u16 = 1024;
pub const HID_USAGE_SENSORS_DATA_FIELD_LONGITUDE: u16 = 1044;
pub const HID_USAGE_SENSORS_DATA_FIELD_MAGNETIC_FLUX: u16 = 1156;
pub const HID_USAGE_SENSORS_DATA_FIELD_MAGNETIC_FLUX_X_AXIS: u16 = 1157;
pub const HID_USAGE_SENSORS_DATA_FIELD_MAGNETIC_FLUX_Y_AXIS: u16 = 1158;
pub const HID_USAGE_SENSORS_DATA_FIELD_MAGNETIC_FLUX_Z_AXIS: u16 = 1159;
pub const HID_USAGE_SENSORS_DATA_FIELD_MAGNETIC_HEADING: u16 = 1046;
pub const HID_USAGE_SENSORS_DATA_FIELD_MAGNETIC_VARIATION: u16 = 1047;
pub const HID_USAGE_SENSORS_DATA_FIELD_MAGNETOMETER_ACCURACY: u16 = 1160;
pub const HID_USAGE_SENSORS_DATA_FIELD_MECHANICAL: u16 = 1168;
pub const HID_USAGE_SENSORS_DATA_FIELD_MILLISECOND: u16 = 1320;
pub const HID_USAGE_SENSORS_DATA_FIELD_MINUTE: u16 = 1318;
pub const HID_USAGE_SENSORS_DATA_FIELD_MONTH: u16 = 1314;
pub const HID_USAGE_SENSORS_DATA_FIELD_MOTION: u16 = 1104;
pub const HID_USAGE_SENSORS_DATA_FIELD_MOTION_INTENSITY: u16 = 1119;
pub const HID_USAGE_SENSORS_DATA_FIELD_MOTION_SPEED: u16 = 1118;
pub const HID_USAGE_SENSORS_DATA_FIELD_MOTION_STATE: u16 = 1105;
pub const HID_USAGE_SENSORS_DATA_FIELD_MULTIVALUE_SWITCH_VALUE: u16 = 1171;
pub const HID_USAGE_SENSORS_DATA_FIELD_NEAR_INFRARED_LIGHT: u16 = 1246;
pub const HID_USAGE_SENSORS_DATA_FIELD_NFC_SENTENCE_RECEIVE: u16 = 1266;
pub const HID_USAGE_SENSORS_DATA_FIELD_NMEA_SENTENCE: u16 = 1057;
pub const HID_USAGE_SENSORS_DATA_FIELD_OBJECT_PRESENCE: u16 = 1082;
pub const HID_USAGE_SENSORS_DATA_FIELD_OBJECT_PROXIMITY_OUT_OF_RANGE: u16 = 1084;
pub const HID_USAGE_SENSORS_DATA_FIELD_OBJECT_PROXIMITY_RANGE: u16 = 1083;
pub const HID_USAGE_SENSORS_DATA_FIELD_ORIENTATION: u16 = 1136;
pub const HID_USAGE_SENSORS_DATA_FIELD_PERCENT_OF_RANGE: u16 = 1289;
pub const HID_USAGE_SENSORS_DATA_FIELD_PERIOD: u16 = 1288;
pub const HID_USAGE_SENSORS_DATA_FIELD_PERSONAL_ACTIVITY: u16 = 1424;
pub const HID_USAGE_SENSORS_DATA_FIELD_POSITION_DILUTION_OF_PRECISION: u16 = 1040;
pub const HID_USAGE_SENSORS_DATA_FIELD_POSTAL_CODE: u16 = 1063;
pub const HID_USAGE_SENSORS_DATA_FIELD_QUATERNION: u16 = 1155;
pub const HID_USAGE_SENSORS_DATA_FIELD_RED_LIGHT: u16 = 1240;
pub const HID_USAGE_SENSORS_DATA_FIELD_RELATIVE_HUMIDITY: u16 = 1075;
pub const HID_USAGE_SENSORS_DATA_FIELD_RESISTANCE: u16 = 1285;
pub const HID_USAGE_SENSORS_DATA_FIELD_RESPIRATORY_RATE: u16 = 1211;
pub const HID_USAGE_SENSORS_DATA_FIELD_RESTING_HEART_RATE: u16 = 1209;
pub const HID_USAGE_SENSORS_DATA_FIELD_RFID_TAG_40_BIT: u16 = 1265;
pub const HID_USAGE_SENSORS_DATA_FIELD_ROTATION_MATRIX: u16 = 1154;
pub const HID_USAGE_SENSORS_DATA_FIELD_SATELLITES_IN_VIEW: u16 = 1049;
pub const HID_USAGE_SENSORS_DATA_FIELD_SATELLITES_IN_VIEW_AZIMUTH: u16 = 1050;
pub const HID_USAGE_SENSORS_DATA_FIELD_SATELLITES_IN_VIEW_ELEVATION: u16 = 1051;
pub const HID_USAGE_SENSORS_DATA_FIELD_SATELLITES_IN_VIEW_IDS: u16 = 1052;
pub const HID_USAGE_SENSORS_DATA_FIELD_SATELLITES_IN_VIEW_PRNS: u16 = 1053;
pub const HID_USAGE_SENSORS_DATA_FIELD_SATELLITES_IN_VIEW_SN_RATIOS: u16 = 1054;
pub const HID_USAGE_SENSORS_DATA_FIELD_SATELLITES_USED_COUNT: u16 = 1055;
pub const HID_USAGE_SENSORS_DATA_FIELD_SATELLITES_USED_PRNS: u16 = 1056;
pub const HID_USAGE_SENSORS_DATA_FIELD_SCANNER: u16 = 1264;
pub const HID_USAGE_SENSORS_DATA_FIELD_SECOND: u16 = 1319;
pub const HID_USAGE_SENSORS_DATA_FIELD_SIMPLE_ORIENTATION_DIRECTION: u16 = 1161;
pub const HID_USAGE_SENSORS_DATA_FIELD_SPEED: u16 = 1048;
pub const HID_USAGE_SENSORS_DATA_FIELD_SPO2: u16 = 1212;
pub const HID_USAGE_SENSORS_DATA_FIELD_STATE_OR_PROVINCE: u16 = 1061;
pub const HID_USAGE_SENSORS_DATA_FIELD_STEP_COUNT: u16 = 1428;
pub const HID_USAGE_SENSORS_DATA_FIELD_STEP_COUNT_RESET: u16 = 1429;
pub const HID_USAGE_SENSORS_DATA_FIELD_STEP_DURATION: u16 = 1430;
pub const HID_USAGE_SENSORS_DATA_FIELD_STEP_TYPE: u16 = 1431;
pub const HID_USAGE_SENSORS_DATA_FIELD_STRAIN: u16 = 1175;
pub const HID_USAGE_SENSORS_DATA_FIELD_TEMPERATURE: u16 = 1076;
pub const HID_USAGE_SENSORS_DATA_FIELD_TILT: u16 = 1150;
pub const HID_USAGE_SENSORS_DATA_FIELD_TILT_X_AXIS: u16 = 1151;
pub const HID_USAGE_SENSORS_DATA_FIELD_TILT_Y_AXIS: u16 = 1152;
pub const HID_USAGE_SENSORS_DATA_FIELD_TILT_Z_AXIS: u16 = 1153;
pub const HID_USAGE_SENSORS_DATA_FIELD_TIME: u16 = 1312;
pub const HID_USAGE_SENSORS_DATA_FIELD_TIMESTAMP: u16 = 1321;
pub const HID_USAGE_SENSORS_DATA_FIELD_TIME_SINCE_SYSTEM_BOOT: u16 = 1323;
pub const HID_USAGE_SENSORS_DATA_FIELD_TRUE_HEADING: u16 = 1045;
pub const HID_USAGE_SENSORS_DATA_FIELD_ULTRAVIOLET_A_LIGHT: u16 = 1243;
pub const HID_USAGE_SENSORS_DATA_FIELD_ULTRAVIOLET_B_LIGHT: u16 = 1244;
pub const HID_USAGE_SENSORS_DATA_FIELD_ULTRAVIOLET_INDEX: u16 = 1245;
pub const HID_USAGE_SENSORS_DATA_FIELD_VERTICAL_DILUTION_OF_PRECISION: u16 = 1042;
pub const HID_USAGE_SENSORS_DATA_FIELD_VOLATILE_ORGANIC_COMPOUND_CONCENTRATION: u16 = 1081;
pub const HID_USAGE_SENSORS_DATA_FIELD_VOLTAGE: u16 = 1286;
pub const HID_USAGE_SENSORS_DATA_FIELD_WEIGHT: u16 = 1176;
pub const HID_USAGE_SENSORS_DATA_FIELD_WIND_DIRECTION: u16 = 1077;
pub const HID_USAGE_SENSORS_DATA_FIELD_WIND_SPEED: u16 = 1078;
pub const HID_USAGE_SENSORS_DATA_FIELD_YEAR: u16 = 1313;
pub const HID_USAGE_SENSORS_DAY_OF_WEEK_FRIDAY: u16 = 2245;
pub const HID_USAGE_SENSORS_DAY_OF_WEEK_MONDAY: u16 = 2241;
pub const HID_USAGE_SENSORS_DAY_OF_WEEK_SATURDAY: u16 = 2246;
pub const HID_USAGE_SENSORS_DAY_OF_WEEK_SUNDAY: u16 = 2240;
pub const HID_USAGE_SENSORS_DAY_OF_WEEK_THURSDAY: u16 = 2244;
pub const HID_USAGE_SENSORS_DAY_OF_WEEK_TUESDAY: u16 = 2242;
pub const HID_USAGE_SENSORS_DAY_OF_WEEK_WEDNESDAY: u16 = 2243;
pub const HID_USAGE_SENSORS_DEVICE_POSITION_IN_HAND: u16 = 2435;
pub const HID_USAGE_SENSORS_DEVICE_POSITION_MOVING_IN_BAG: u16 = 2436;
pub const HID_USAGE_SENSORS_DEVICE_POSITION_ON_DESK: u16 = 2434;
pub const HID_USAGE_SENSORS_DEVICE_POSITION_STATIONARY_IN_BAG: u16 = 2437;
pub const HID_USAGE_SENSORS_DEVICE_POSITION_UNCHANGED: u16 = 2433;
pub const HID_USAGE_SENSORS_DEVICE_POSITION_UNKNOWN: u16 = 2432;
pub const HID_USAGE_SENSORS_ELECTRICAL: u16 = 32;
pub const HID_USAGE_SENSORS_ELECTRICAL_CAPACITANCE: u16 = 33;
pub const HID_USAGE_SENSORS_ELECTRICAL_CURRENT: u16 = 34;
pub const HID_USAGE_SENSORS_ELECTRICAL_FREQUENCY: u16 = 40;
pub const HID_USAGE_SENSORS_ELECTRICAL_INDUCTANCE: u16 = 36;
pub const HID_USAGE_SENSORS_ELECTRICAL_PERIOD: u16 = 41;
pub const HID_USAGE_SENSORS_ELECTRICAL_POTENTIOMETER: u16 = 39;
pub const HID_USAGE_SENSORS_ELECTRICAL_POWER: u16 = 35;
pub const HID_USAGE_SENSORS_ELECTRICAL_RESISTANCE: u16 = 37;
pub const HID_USAGE_SENSORS_ELECTRICAL_VOLTAGE: u16 = 38;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL: u16 = 48;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_AIR_QUALITY: u16 = 54;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_ATMOSPHERIC_PRESSURE: u16 = 49;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_HEAT_INDEX: u16 = 55;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_HUMIDITY: u16 = 50;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_OBJECT_PRESENCE: u16 = 58;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_OBJECT_PROXIMITY: u16 = 59;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_SURFACE_TEMPERATURE: u16 = 56;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_TEMPERATURE: u16 = 51;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_VOLATILE_ORGANIC_COMPOUNDS: u16 = 57;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_WIND_DIRECTION: u16 = 52;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_WIND_SPEED: u16 = 53;
pub const HID_USAGE_SENSORS_EVENT: u16 = 512;
pub const HID_USAGE_SENSORS_EVENT_SENSOR_EVENT: u16 = 514;
pub const HID_USAGE_SENSORS_EVENT_SENSOR_STATE: u16 = 513;
pub const HID_USAGE_SENSORS_EXPONENT_0: u16 = 2416;
pub const HID_USAGE_SENSORS_EXPONENT_1: u16 = 2417;
pub const HID_USAGE_SENSORS_EXPONENT_2: u16 = 2418;
pub const HID_USAGE_SENSORS_EXPONENT_3: u16 = 2419;
pub const HID_USAGE_SENSORS_EXPONENT_4: u16 = 2420;
pub const HID_USAGE_SENSORS_EXPONENT_5: u16 = 2421;
pub const HID_USAGE_SENSORS_EXPONENT_6: u16 = 2422;
pub const HID_USAGE_SENSORS_EXPONENT_7: u16 = 2423;
pub const HID_USAGE_SENSORS_EXPONENT_8: u16 = 2424;
pub const HID_USAGE_SENSORS_EXPONENT_9: u16 = 2425;
pub const HID_USAGE_SENSORS_EXPONENT_A: u16 = 2426;
pub const HID_USAGE_SENSORS_EXPONENT_B: u16 = 2427;
pub const HID_USAGE_SENSORS_EXPONENT_C: u16 = 2428;
pub const HID_USAGE_SENSORS_EXPONENT_D: u16 = 2429;
pub const HID_USAGE_SENSORS_EXPONENT_E: u16 = 2430;
pub const HID_USAGE_SENSORS_EXPONENT_F: u16 = 2431;
pub const HID_USAGE_SENSORS_FIX_QUALITY_DGPS: u16 = 2162;
pub const HID_USAGE_SENSORS_FIX_QUALITY_GPS: u16 = 2161;
pub const HID_USAGE_SENSORS_FIX_QUALITY_NO_FIX: u16 = 2160;
pub const HID_USAGE_SENSORS_FIX_TYPE_DGPS_SPS_MODE_FIX_VALID: u16 = 2178;
pub const HID_USAGE_SENSORS_FIX_TYPE_ESTIMATED_DEAD_RECKONED: u16 = 2182;
pub const HID_USAGE_SENSORS_FIX_TYPE_FLOAT_RTK: u16 = 2181;
pub const HID_USAGE_SENSORS_FIX_TYPE_GPS_PPS_MODE_FIX_VALID: u16 = 2179;
pub const HID_USAGE_SENSORS_FIX_TYPE_GPS_SPS_MODE_FIX_VALID: u16 = 2177;
pub const HID_USAGE_SENSORS_FIX_TYPE_MANUAL_INPUT_MODE: u16 = 2183;
pub const HID_USAGE_SENSORS_FIX_TYPE_NO_FIX: u16 = 2176;
pub const HID_USAGE_SENSORS_FIX_TYPE_REAL_TIME_KINEMATIC: u16 = 2180;
pub const HID_USAGE_SENSORS_FIX_TYPE_SIMULATOR_MODE: u16 = 2184;
pub const HID_USAGE_SENSORS_GESTURE: u16 = 208;
pub const HID_USAGE_SENSORS_GESTURE_CHASSIS_FLIP_GESTURE: u16 = 209;
pub const HID_USAGE_SENSORS_GESTURE_HINGE_FOLD_GESTURE: u16 = 210;
pub const HID_USAGE_SENSORS_GESTURE_STATE_CANCELLED: u16 = 2467;
pub const HID_USAGE_SENSORS_GESTURE_STATE_COMPLETED: u16 = 2466;
pub const HID_USAGE_SENSORS_GESTURE_STATE_STARTED: u16 = 2465;
pub const HID_USAGE_SENSORS_GESTURE_STATE_UNKNOWN: u16 = 2464;
pub const HID_USAGE_SENSORS_GPS_OPERATION_MODE_AUTOMATIC: u16 = 2193;
pub const HID_USAGE_SENSORS_GPS_OPERATION_MODE_MANUAL: u16 = 2192;
pub const HID_USAGE_SENSORS_GPS_SELECTION_MODE_AUTONOMOUS: u16 = 2208;
pub const HID_USAGE_SENSORS_GPS_SELECTION_MODE_DATA_NOT_VALID: u16 = 2213;
pub const HID_USAGE_SENSORS_GPS_SELECTION_MODE_DGPS: u16 = 2209;
pub const HID_USAGE_SENSORS_GPS_SELECTION_MODE_ESTIMATED_DEAD_RECKONED: u16 = 2210;
pub const HID_USAGE_SENSORS_GPS_SELECTION_MODE_MANUAL_INPUT: u16 = 2211;
pub const HID_USAGE_SENSORS_GPS_SELECTION_MODE_SIMULATOR: u16 = 2212;
pub const HID_USAGE_SENSORS_GPS_STATUS_DATA_NOT_VALID: u16 = 2225;
pub const HID_USAGE_SENSORS_GPS_STATUS_DATA_VALID: u16 = 2224;
pub const HID_USAGE_SENSORS_HINGE_FOLD_CONTRIBUTING_PANEL_BOTH: u16 = 2483;
pub const HID_USAGE_SENSORS_HINGE_FOLD_CONTRIBUTING_PANEL_PANEL_1: u16 = 2481;
pub const HID_USAGE_SENSORS_HINGE_FOLD_CONTRIBUTING_PANEL_PANEL_2: u16 = 2482;
pub const HID_USAGE_SENSORS_HINGE_FOLD_CONTRIBUTING_PANEL_UNKNOWN: u16 = 2480;
pub const HID_USAGE_SENSORS_HINGE_FOLD_TYPE_DECREASING: u16 = 2486;
pub const HID_USAGE_SENSORS_HINGE_FOLD_TYPE_INCREASING: u16 = 2485;
pub const HID_USAGE_SENSORS_HINGE_FOLD_TYPE_UNKNOWN: u16 = 2484;
pub const HID_USAGE_SENSORS_HUMAN_PRESENCE_DETECTION_TYPE_AUDIO_BIOMETRIC: u16 = 2499;
pub const HID_USAGE_SENSORS_HUMAN_PRESENCE_DETECTION_TYPE_FACIAL_BIOMETRIC: u16 = 2498;
pub const HID_USAGE_SENSORS_HUMAN_PRESENCE_DETECTION_TYPE_VENDORDEFINED_BIOMETRIC: u16 = 2497;
pub const HID_USAGE_SENSORS_HUMAN_PRESENCE_DETECTION_TYPE_VENDORDEFINED_NONBIOMETRIC: u16 = 2496;
pub const HID_USAGE_SENSORS_KIND_CATEGORY: u16 = 2256;
pub const HID_USAGE_SENSORS_KIND_DATA_FIELD: u16 = 2260;
pub const HID_USAGE_SENSORS_KIND_EVENT: u16 = 2258;
pub const HID_USAGE_SENSORS_KIND_PROPERTY: u16 = 2259;
pub const HID_USAGE_SENSORS_KIND_TYPE: u16 = 2257;
pub const HID_USAGE_SENSORS_LIGHT: u16 = 64;
pub const HID_USAGE_SENSORS_LIGHT_AMBIENT_LIGHT: u16 = 65;
pub const HID_USAGE_SENSORS_LIGHT_CONSUMER_INFRARED: u16 = 66;
pub const HID_USAGE_SENSORS_LIGHT_INFRARED_LIGHT: u16 = 67;
pub const HID_USAGE_SENSORS_LIGHT_ULTRAVIOLET_LIGHT: u16 = 69;
pub const HID_USAGE_SENSORS_LIGHT_VISIBLE_LIGHT: u16 = 68;
pub const HID_USAGE_SENSORS_LOCATION: u16 = 80;
pub const HID_USAGE_SENSORS_LOCATION_BROADCAST: u16 = 81;
pub const HID_USAGE_SENSORS_LOCATION_DEAD_RECKONING: u16 = 82;
pub const HID_USAGE_SENSORS_LOCATION_GPS_GLOBAL_POSITIONING_SYSTEM: u16 = 83;
pub const HID_USAGE_SENSORS_LOCATION_LOOKUP: u16 = 84;
pub const HID_USAGE_SENSORS_LOCATION_OTHER: u16 = 85;
pub const HID_USAGE_SENSORS_LOCATION_STATIC: u16 = 86;
pub const HID_USAGE_SENSORS_LOCATION_TRIANGULATION: u16 = 87;
pub const HID_USAGE_SENSORS_MAGNETOMETER_ACCURACY_HIGH: u16 = 2274;
pub const HID_USAGE_SENSORS_MAGNETOMETER_ACCURACY_LOW: u16 = 2272;
pub const HID_USAGE_SENSORS_MAGNETOMETER_ACCURACY_MEDIUM: u16 = 2273;
pub const HID_USAGE_SENSORS_MECHANICAL: u16 = 96;
pub const HID_USAGE_SENSORS_MECHANICAL_BOOLEAN_SWITCH: u16 = 97;
pub const HID_USAGE_SENSORS_MECHANICAL_BOOLEAN_SWITCH_ARRAY: u16 = 98;
pub const HID_USAGE_SENSORS_MECHANICAL_FORCE: u16 = 100;
pub const HID_USAGE_SENSORS_MECHANICAL_HALL_EFFECT_SWITCH: u16 = 105;
pub const HID_USAGE_SENSORS_MECHANICAL_HAPTIC_VIBRATOR: u16 = 104;
pub const HID_USAGE_SENSORS_MECHANICAL_MULTIVALUE_SWITCH: u16 = 99;
pub const HID_USAGE_SENSORS_MECHANICAL_PRESSURE: u16 = 101;
pub const HID_USAGE_SENSORS_MECHANICAL_STRAIN: u16 = 102;
pub const HID_USAGE_SENSORS_MECHANICAL_WEIGHT: u16 = 103;
pub const HID_USAGE_SENSORS_MODIFIER_ACCURACY: u16 = 16384;
pub const HID_USAGE_SENSORS_MODIFIER_CALIBRATION_MULTIPLIER: u16 = 36864;
pub const HID_USAGE_SENSORS_MODIFIER_CALIBRATION_OFFSET: u16 = 32768;
pub const HID_USAGE_SENSORS_MODIFIER_CHANGE_SENSITIVITY_ABSOLUTE: u16 = 4096;
pub const HID_USAGE_SENSORS_MODIFIER_CHANGE_SENSITIVITY_PERCENT_OF_RANGE: u16 = 53248;
pub const HID_USAGE_SENSORS_MODIFIER_CHANGE_SENSITIVITY_PERCENT_RELATIVE: u16 = 57344;
pub const HID_USAGE_SENSORS_MODIFIER_FREQUENCY_MAX: u16 = 45056;
pub const HID_USAGE_SENSORS_MODIFIER_MAXIMUM: u16 = 8192;
pub const HID_USAGE_SENSORS_MODIFIER_MINIMUM: u16 = 12288;
pub const HID_USAGE_SENSORS_MODIFIER_PERIOD_MAX: u16 = 49152;
pub const HID_USAGE_SENSORS_MODIFIER_REPORT_INTERVAL: u16 = 40960;
pub const HID_USAGE_SENSORS_MODIFIER_RESOLUTION: u16 = 20480;
pub const HID_USAGE_SENSORS_MODIFIER_THRESHOLD_HIGH: u16 = 24576;
pub const HID_USAGE_SENSORS_MODIFIER_THRESHOLD_LOW: u16 = 28672;
pub const HID_USAGE_SENSORS_MODIFIER_VENDOR_RESERVED: u16 = 61440;
pub const HID_USAGE_SENSORS_MOTION: u16 = 112;
pub const HID_USAGE_SENSORS_MOTION_ACCELEROMETER: u16 = 121;
pub const HID_USAGE_SENSORS_MOTION_ACCELEROMETER_1D: u16 = 113;
pub const HID_USAGE_SENSORS_MOTION_ACCELEROMETER_2D: u16 = 114;
pub const HID_USAGE_SENSORS_MOTION_ACCELEROMETER_3D: u16 = 115;
pub const HID_USAGE_SENSORS_MOTION_GRAVITY_VECTOR: u16 = 123;
pub const HID_USAGE_SENSORS_MOTION_GYROMETER: u16 = 122;
pub const HID_USAGE_SENSORS_MOTION_GYROMETER_1D: u16 = 116;
pub const HID_USAGE_SENSORS_MOTION_GYROMETER_2D: u16 = 117;
pub const HID_USAGE_SENSORS_MOTION_GYROMETER_3D: u16 = 118;
pub const HID_USAGE_SENSORS_MOTION_LINEAR_ACCELEROMETER: u16 = 124;
pub const HID_USAGE_SENSORS_MOTION_MOTION_DETECTOR: u16 = 119;
pub const HID_USAGE_SENSORS_MOTION_SPEEDOMETER: u16 = 120;
pub const HID_USAGE_SENSORS_ORIENTATION: u16 = 128;
pub const HID_USAGE_SENSORS_ORIENTATION_COMPASS: u16 = 139;
pub const HID_USAGE_SENSORS_ORIENTATION_COMPASS_1D: u16 = 129;
pub const HID_USAGE_SENSORS_ORIENTATION_COMPASS_2D: u16 = 130;
pub const HID_USAGE_SENSORS_ORIENTATION_COMPASS_3D: u16 = 131;
pub const HID_USAGE_SENSORS_ORIENTATION_DEVICE_ORIENTATION: u16 = 138;
pub const HID_USAGE_SENSORS_ORIENTATION_DISTANCE: u16 = 141;
pub const HID_USAGE_SENSORS_ORIENTATION_DISTANCE_1D: u16 = 135;
pub const HID_USAGE_SENSORS_ORIENTATION_DISTANCE_2D: u16 = 136;
pub const HID_USAGE_SENSORS_ORIENTATION_DISTANCE_3D: u16 = 137;
pub const HID_USAGE_SENSORS_ORIENTATION_EXTENDED: u16 = 192;
pub const HID_USAGE_SENSORS_ORIENTATION_EXTENDED_GEOMAGNETIC_ORIENTATION: u16 = 193;
pub const HID_USAGE_SENSORS_ORIENTATION_EXTENDED_MAGNETOMETER: u16 = 194;
pub const HID_USAGE_SENSORS_ORIENTATION_INCLINOMETER: u16 = 140;
pub const HID_USAGE_SENSORS_ORIENTATION_INCLINOMETER_1D: u16 = 132;
pub const HID_USAGE_SENSORS_ORIENTATION_INCLINOMETER_2D: u16 = 133;
pub const HID_USAGE_SENSORS_ORIENTATION_INCLINOMETER_3D: u16 = 134;
pub const HID_USAGE_SENSORS_ORIENTATION_RELATIVE_ORIENTATION: u16 = 142;
pub const HID_USAGE_SENSORS_ORIENTATION_SIMPLE_ORIENTATION: u16 = 143;
pub const HID_USAGE_SENSORS_OTHER: u16 = 224;
pub const HID_USAGE_SENSORS_OTHER_CUSTOM: u16 = 225;
pub const HID_USAGE_SENSORS_OTHER_GENERIC: u16 = 226;
pub const HID_USAGE_SENSORS_OTHER_GENERIC_ENUMERATOR: u16 = 227;
pub const HID_USAGE_SENSORS_OTHER_HINGE_ANGLE: u16 = 228;
pub const HID_USAGE_SENSORS_PERSONAL_ACTIVITY: u16 = 176;
pub const HID_USAGE_SENSORS_PERSONAL_ACTIVITY_ACTIVITY_DETECTION: u16 = 177;
pub const HID_USAGE_SENSORS_PERSONAL_ACTIVITY_DEVICE_POSITION: u16 = 178;
pub const HID_USAGE_SENSORS_PERSONAL_ACTIVITY_FLOOR_TRACKER: u16 = 179;
pub const HID_USAGE_SENSORS_PERSONAL_ACTIVITY_PEDOMETER: u16 = 180;
pub const HID_USAGE_SENSORS_PERSONAL_ACTIVITY_STEP_DETECTION: u16 = 181;
pub const HID_USAGE_SENSORS_POWER_STATE_D0_FULL_POWER: u16 = 2129;
pub const HID_USAGE_SENSORS_POWER_STATE_D1_LOW_POWER: u16 = 2130;
pub const HID_USAGE_SENSORS_POWER_STATE_D2_STANDBY_POWER_WITH_WAKEUP: u16 = 2131;
pub const HID_USAGE_SENSORS_POWER_STATE_D3_SLEEP_WITH_WAKEUP: u16 = 2132;
pub const HID_USAGE_SENSORS_POWER_STATE_D4_POWER_OFF: u16 = 2133;
pub const HID_USAGE_SENSORS_POWER_STATE_UNDEFINED: u16 = 2128;
pub const HID_USAGE_SENSORS_PROPERTY: u16 = 768;
pub const HID_USAGE_SENSORS_PROPERTY_ACCURACY: u16 = 786;
pub const HID_USAGE_SENSORS_PROPERTY_ARM_ALARM: u16 = 1333;
pub const HID_USAGE_SENSORS_PROPERTY_AUTO_BRIGHTNESS_PREFERRED: u16 = 1250;
pub const HID_USAGE_SENSORS_PROPERTY_AUTO_COLOR_PREFERRED: u16 = 1251;
pub const HID_USAGE_SENSORS_PROPERTY_BACKWARD_VIBRATION_SPEED: u16 = 1187;
pub const HID_USAGE_SENSORS_PROPERTY_CHANGE_SENSITIVITY_ABSOLUTE: u16 = 783;
pub const HID_USAGE_SENSORS_PROPERTY_CHANGE_SENSITIVITY_PERCENT_OF_RANGE: u16 = 784;
pub const HID_USAGE_SENSORS_PROPERTY_CHANGE_SENSITIVITY_PERCENT_RELATIVE: u16 = 785;
pub const HID_USAGE_SENSORS_PROPERTY_CONSUMER_IR_SENTENCE_SEND: u16 = 1248;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM: u16 = 1472;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_1: u16 = 1473;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_10: u16 = 1482;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_11: u16 = 1483;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_12: u16 = 1484;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_13: u16 = 1485;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_14: u16 = 1486;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_15: u16 = 1487;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_16: u16 = 1488;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_2: u16 = 1474;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_3: u16 = 1475;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_4: u16 = 1476;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_5: u16 = 1477;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_6: u16 = 1478;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_7: u16 = 1479;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_8: u16 = 1480;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_9: u16 = 1481;
pub const HID_USAGE_SENSORS_PROPERTY_DAYLIGHT_SAVINGS_TIME_OBSERVED: u16 = 1331;
pub const HID_USAGE_SENSORS_PROPERTY_ENUMERATOR_TABLE_ROW_COUNT: u16 = 1410;
pub const HID_USAGE_SENSORS_PROPERTY_ENUMERATOR_TABLE_ROW_INDEX: u16 = 1409;
pub const HID_USAGE_SENSORS_PROPERTY_ENVIRONMENTAL: u16 = 1088;
pub const HID_USAGE_SENSORS_PROPERTY_FIRMWARE_VERSION: u16 = 780;
pub const HID_USAGE_SENSORS_PROPERTY_FLOOR_HEIGHT: u16 = 1445;
pub const HID_USAGE_SENSORS_PROPERTY_FLUSH_FIFO_EVENTS: u16 = 796;
pub const HID_USAGE_SENSORS_PROPERTY_FORWARD_VIBRATION_SPEED: u16 = 1186;
pub const HID_USAGE_SENSORS_PROPERTY_FRIENDLY_NAME: u16 = 769;
pub const HID_USAGE_SENSORS_PROPERTY_GENERIC: u16 = 1408;
pub const HID_USAGE_SENSORS_PROPERTY_HARDWARE_REVISION: u16 = 779;
pub const HID_USAGE_SENSORS_PROPERTY_HUMAN_PRESENCE_DETECTION_TYPE: u16 = 799;
pub const HID_USAGE_SENSORS_PROPERTY_IS_PRIMARY: u16 = 798;
pub const HID_USAGE_SENSORS_PROPERTY_LIGHT: u16 = 1247;
pub const HID_USAGE_SENSORS_PROPERTY_LOCATION: u16 = 1066;
pub const HID_USAGE_SENSORS_PROPERTY_LOCATION_DESIRED_ACCURACY: u16 = 1067;
pub const HID_USAGE_SENSORS_PROPERTY_MAXIMUM: u16 = 788;
pub const HID_USAGE_SENSORS_PROPERTY_MAXIMUM_FIFO_EVENTS: u16 = 794;
pub const HID_USAGE_SENSORS_PROPERTY_MAXIMUM_POWER_CONSUMPTION: u16 = 797;
pub const HID_USAGE_SENSORS_PROPERTY_MECHANICAL: u16 = 1184;
pub const HID_USAGE_SENSORS_PROPERTY_MINIMUM: u16 = 789;
pub const HID_USAGE_SENSORS_PROPERTY_MINIMUM_ACTIVITY_DETECTION_INTERVAL: u16 = 1440;
pub const HID_USAGE_SENSORS_PROPERTY_MINIMUM_REPORT_INTERVAL: u16 = 772;
pub const HID_USAGE_SENSORS_PROPERTY_NFC_SENTENCE_SEND: u16 = 1273;
pub const HID_USAGE_SENSORS_PROPERTY_PERSISTENT_UNIQUE_ID: u16 = 770;
pub const HID_USAGE_SENSORS_PROPERTY_POWER_STATE: u16 = 793;
pub const HID_USAGE_SENSORS_PROPERTY_REFERENCE_PRESSURE: u16 = 1089;
pub const HID_USAGE_SENSORS_PROPERTY_RELEASE_DATE: u16 = 781;
pub const HID_USAGE_SENSORS_PROPERTY_REPORTING_STATE: u16 = 790;
pub const HID_USAGE_SENSORS_PROPERTY_REPORT_INTERVAL: u16 = 782;
pub const HID_USAGE_SENSORS_PROPERTY_REPORT_LATENCY: u16 = 795;
pub const HID_USAGE_SENSORS_PROPERTY_RESOLUTION: u16 = 787;
pub const HID_USAGE_SENSORS_PROPERTY_RESPONSE_CURVE: u16 = 792;
pub const HID_USAGE_SENSORS_PROPERTY_SAMPLING_RATE: u16 = 791;
pub const HID_USAGE_SENSORS_PROPERTY_SCANNER: u16 = 1272;
pub const HID_USAGE_SENSORS_PROPERTY_SENSOR_CONNECTION_TYPE: u16 = 777;
pub const HID_USAGE_SENSORS_PROPERTY_SENSOR_DESCRIPTION: u16 = 776;
pub const HID_USAGE_SENSORS_PROPERTY_SENSOR_DEVICE_PATH: u16 = 778;
pub const HID_USAGE_SENSORS_PROPERTY_SENSOR_MANUFACTURER: u16 = 773;
pub const HID_USAGE_SENSORS_PROPERTY_SENSOR_MODEL: u16 = 774;
pub const HID_USAGE_SENSORS_PROPERTY_SENSOR_SERIAL_NUMBER: u16 = 775;
pub const HID_USAGE_SENSORS_PROPERTY_SENSOR_STATUS: u16 = 771;
pub const HID_USAGE_SENSORS_PROPERTY_SUBSCRIBED_ACTIVITY_TYPES: u16 = 1442;
pub const HID_USAGE_SENSORS_PROPERTY_SUBSCRIBED_STEP_TYPES: u16 = 1444;
pub const HID_USAGE_SENSORS_PROPERTY_SUPPORTED_ACTIVITY_TYPES: u16 = 1441;
pub const HID_USAGE_SENSORS_PROPERTY_SUPPORTED_STEP_TYPES: u16 = 1443;
pub const HID_USAGE_SENSORS_PROPERTY_TIME: u16 = 1328;
pub const HID_USAGE_SENSORS_PROPERTY_TIME_TRIM_ADJUSTMENT: u16 = 1332;
pub const HID_USAGE_SENSORS_PROPERTY_TIME_ZONE_NAME: u16 = 1330;
pub const HID_USAGE_SENSORS_PROPERTY_TIME_ZONE_OFFSET_FROM_UTC: u16 = 1329;
pub const HID_USAGE_SENSORS_PROPERTY_VIBRATION_STATE: u16 = 1185;
pub const HID_USAGE_SENSORS_REPORTING_STATE_ANYTIME_SEL: u16 = 2118;
pub const HID_USAGE_SENSORS_REPORTING_STATE_REPORT_ALL_EVENTS: u16 = 2113;
pub const HID_USAGE_SENSORS_REPORTING_STATE_REPORT_NO_EVENTS: u16 = 2112;
pub const HID_USAGE_SENSORS_REPORTING_STATE_REPORT_THRESHOLD_EVENTS: u16 = 2114;
pub const HID_USAGE_SENSORS_REPORTING_STATE_WAKE_ON_ALL_EVENTS: u16 = 2116;
pub const HID_USAGE_SENSORS_REPORTING_STATE_WAKE_ON_NO_EVENTS: u16 = 2115;
pub const HID_USAGE_SENSORS_REPORTING_STATE_WAKE_ON_THRESHOLD_EVENTS: u16 = 2117;
pub const HID_USAGE_SENSORS_SCANNER: u16 = 144;
pub const HID_USAGE_SENSORS_SCANNER_BARCODE: u16 = 145;
pub const HID_USAGE_SENSORS_SCANNER_NFC: u16 = 147;
pub const HID_USAGE_SENSORS_SCANNER_RFID: u16 = 146;
pub const HID_USAGE_SENSORS_SENSOR: u16 = 1;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_CHANGE_SENSITIVITY: u16 = 2069;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_COMPLEX_TRIGGER: u16 = 2080;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_DATA_UPDATED: u16 = 2067;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_FREQUENCY_EXCEEDED: u16 = 2079;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_HIGH_THRESHOLD_CROSS_DOWNWARD: u16 = 2073;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_HIGH_THRESHOLD_CROSS_UPWARD: u16 = 2072;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_LOW_THRESHOLD_CROSS_DOWNWARD: u16 = 2075;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_LOW_THRESHOLD_CROSS_UPWARD: u16 = 2074;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_PERIOD_EXCEEDED: u16 = 2078;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_POLL_RESPONSE: u16 = 2068;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_PROPERTY_CHANGED: u16 = 2066;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_RANGE_MAXIMUM_REACHED: u16 = 2070;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_RANGE_MINIMUM_REACHED: u16 = 2071;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_STATE_CHANGED: u16 = 2065;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_UNKNOWN: u16 = 2064;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_ZERO_THRESHOLD_CROSS_DOWNWARD: u16 = 2077;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_ZERO_THRESHOLD_CROSS_UPWARD: u16 = 2076;
pub const HID_USAGE_SENSORS_SENSOR_STATE_ACCESS_DENIED: u16 = 2053;
pub const HID_USAGE_SENSORS_SENSOR_STATE_ERROR: u16 = 2054;
pub const HID_USAGE_SENSORS_SENSOR_STATE_INITIALIZING: u16 = 2052;
pub const HID_USAGE_SENSORS_SENSOR_STATE_NOT_AVAILABLE: u16 = 2050;
pub const HID_USAGE_SENSORS_SENSOR_STATE_NO_DATA: u16 = 2051;
pub const HID_USAGE_SENSORS_SENSOR_STATE_READY: u16 = 2049;
pub const HID_USAGE_SENSORS_SENSOR_STATE_UNDEFINED: u16 = 2048;
pub const HID_USAGE_SENSORS_SIMPLE_ORIENTATION_DIRECTION_FACE_DOWN: u16 = 2293;
pub const HID_USAGE_SENSORS_SIMPLE_ORIENTATION_DIRECTION_FACE_UP: u16 = 2292;
pub const HID_USAGE_SENSORS_SIMPLE_ORIENTATION_DIRECTION_NOT_ROTATED: u16 = 2288;
pub const HID_USAGE_SENSORS_SIMPLE_ORIENTATION_DIRECTION_ROTATED_180_DEGREES_CCW: u16 = 2290;
pub const HID_USAGE_SENSORS_SIMPLE_ORIENTATION_DIRECTION_ROTATED_270_DEGREES_CCW: u16 = 2291;
pub const HID_USAGE_SENSORS_SIMPLE_ORIENTATION_DIRECTION_ROTATED_90_DEGREES_CCW: u16 = 2289;
pub const HID_USAGE_SENSORS_STEP_TYPE_RUNNING: u16 = 2450;
pub const HID_USAGE_SENSORS_STEP_TYPE_UNKNOWN: u16 = 2448;
pub const HID_USAGE_SENSORS_STEP_TYPE_WALKING: u16 = 2449;
pub const HID_USAGE_SENSORS_TIME: u16 = 160;
pub const HID_USAGE_SENSORS_TIME_ALARM_TIMER: u16 = 161;
pub const HID_USAGE_SENSORS_TIME_REAL_TIME_CLOCK: u16 = 162;
pub const HID_USAGE_SENSORS_UNIT_AMPERE: u16 = 2379;
pub const HID_USAGE_SENSORS_UNIT_BAR: u16 = 2385;
pub const HID_USAGE_SENSORS_UNIT_BITS: u16 = 2398;
pub const HID_USAGE_SENSORS_UNIT_BYTES: u16 = 2396;
pub const HID_USAGE_SENSORS_UNIT_DEGREES: u16 = 2388;
pub const HID_USAGE_SENSORS_UNIT_DEGREESSECOND: u16 = 2389;
pub const HID_USAGE_SENSORS_UNIT_DEGREESSECONDSECOND: u16 = 2390;
pub const HID_USAGE_SENSORS_UNIT_DEGREES_ANTICLOCKWISE: u16 = 2386;
pub const HID_USAGE_SENSORS_UNIT_DEGREES_CELSIUS: u16 = 2371;
pub const HID_USAGE_SENSORS_UNIT_DEGREES_CLOCKWISE: u16 = 2387;
pub const HID_USAGE_SENSORS_UNIT_DEGREES_KELVIN: u16 = 2370;
pub const HID_USAGE_SENSORS_UNIT_FARAD: u16 = 2378;
pub const HID_USAGE_SENSORS_UNIT_G: u16 = 2395;
pub const HID_USAGE_SENSORS_UNIT_HENRY: u16 = 2381;
pub const HID_USAGE_SENSORS_UNIT_HERTZ: u16 = 2384;
pub const HID_USAGE_SENSORS_UNIT_KILOGRAM: u16 = 2375;
pub const HID_USAGE_SENSORS_UNIT_KNOT: u16 = 2391;
pub const HID_USAGE_SENSORS_UNIT_LUX: u16 = 2369;
pub const HID_USAGE_SENSORS_UNIT_METER: u16 = 2376;
pub const HID_USAGE_SENSORS_UNIT_METERSSECOND: u16 = 2374;
pub const HID_USAGE_SENSORS_UNIT_METERSSECONDSECOND: u16 = 2377;
pub const HID_USAGE_SENSORS_UNIT_MILLIGAUSS: u16 = 2397;
pub const HID_USAGE_SENSORS_UNIT_MILLISECOND: u16 = 2394;
pub const HID_USAGE_SENSORS_UNIT_NEWTON: u16 = 2373;
pub const HID_USAGE_SENSORS_UNIT_NOT_SPECIFIED: u16 = 2368;
pub const HID_USAGE_SENSORS_UNIT_OHM: u16 = 2382;
pub const HID_USAGE_SENSORS_UNIT_PASCAL: u16 = 2372;
pub const HID_USAGE_SENSORS_UNIT_PERCENT: u16 = 2392;
pub const HID_USAGE_SENSORS_UNIT_SECOND: u16 = 2393;
pub const HID_USAGE_SENSORS_UNIT_VOLT: u16 = 2383;
pub const HID_USAGE_SENSORS_UNIT_WATT: u16 = 2380;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_1: u16 = 240;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_10: u16 = 249;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_11: u16 = 250;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_12: u16 = 251;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_13: u16 = 252;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_14: u16 = 253;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_15: u16 = 254;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_16: u16 = 255;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_2: u16 = 241;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_3: u16 = 242;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_4: u16 = 243;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_5: u16 = 244;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_6: u16 = 245;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_7: u16 = 246;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_8: u16 = 247;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_9: u16 = 248;
pub const HID_USAGE_SENSORS_VT_BOOL: u16 = 2305;
pub const HID_USAGE_SENSORS_VT_CLSID: u16 = 2318;
pub const HID_USAGE_SENSORS_VT_F16E0: u16 = 2320;
pub const HID_USAGE_SENSORS_VT_F16E1: u16 = 2321;
pub const HID_USAGE_SENSORS_VT_F16E2: u16 = 2322;
pub const HID_USAGE_SENSORS_VT_F16E3: u16 = 2323;
pub const HID_USAGE_SENSORS_VT_F16E4: u16 = 2324;
pub const HID_USAGE_SENSORS_VT_F16E5: u16 = 2325;
pub const HID_USAGE_SENSORS_VT_F16E6: u16 = 2326;
pub const HID_USAGE_SENSORS_VT_F16E7: u16 = 2327;
pub const HID_USAGE_SENSORS_VT_F16E8: u16 = 2328;
pub const HID_USAGE_SENSORS_VT_F16E9: u16 = 2329;
pub const HID_USAGE_SENSORS_VT_F16EA: u16 = 2330;
pub const HID_USAGE_SENSORS_VT_F16EB: u16 = 2331;
pub const HID_USAGE_SENSORS_VT_F16EC: u16 = 2332;
pub const HID_USAGE_SENSORS_VT_F16ED: u16 = 2333;
pub const HID_USAGE_SENSORS_VT_F16EE: u16 = 2334;
pub const HID_USAGE_SENSORS_VT_F16EF: u16 = 2335;
pub const HID_USAGE_SENSORS_VT_F32E0: u16 = 2336;
pub const HID_USAGE_SENSORS_VT_F32E1: u16 = 2337;
pub const HID_USAGE_SENSORS_VT_F32E2: u16 = 2338;
pub const HID_USAGE_SENSORS_VT_F32E3: u16 = 2339;
pub const HID_USAGE_SENSORS_VT_F32E4: u16 = 2340;
pub const HID_USAGE_SENSORS_VT_F32E5: u16 = 2341;
pub const HID_USAGE_SENSORS_VT_F32E6: u16 = 2342;
pub const HID_USAGE_SENSORS_VT_F32E7: u16 = 2343;
pub const HID_USAGE_SENSORS_VT_F32E8: u16 = 2344;
pub const HID_USAGE_SENSORS_VT_F32E9: u16 = 2345;
pub const HID_USAGE_SENSORS_VT_F32EA: u16 = 2346;
pub const HID_USAGE_SENSORS_VT_F32EB: u16 = 2347;
pub const HID_USAGE_SENSORS_VT_F32EC: u16 = 2348;
pub const HID_USAGE_SENSORS_VT_F32ED: u16 = 2349;
pub const HID_USAGE_SENSORS_VT_F32EE: u16 = 2350;
pub const HID_USAGE_SENSORS_VT_F32EF: u16 = 2351;
pub const HID_USAGE_SENSORS_VT_I1: u16 = 2307;
pub const HID_USAGE_SENSORS_VT_I2: u16 = 2309;
pub const HID_USAGE_SENSORS_VT_I4: u16 = 2311;
pub const HID_USAGE_SENSORS_VT_I8: u16 = 2313;
pub const HID_USAGE_SENSORS_VT_NULL: u16 = 2304;
pub const HID_USAGE_SENSORS_VT_R4: u16 = 2314;
pub const HID_USAGE_SENSORS_VT_R8: u16 = 2315;
pub const HID_USAGE_SENSORS_VT_STR: u16 = 2317;
pub const HID_USAGE_SENSORS_VT_UI1: u16 = 2306;
pub const HID_USAGE_SENSORS_VT_UI2: u16 = 2308;
pub const HID_USAGE_SENSORS_VT_UI4: u16 = 2310;
pub const HID_USAGE_SENSORS_VT_UI8: u16 = 2312;
pub const HID_USAGE_SENSORS_VT_VECTOR_VT_UI1: u16 = 2319;
pub const HID_USAGE_SENSORS_VT_WSTR: u16 = 2316;
pub const HID_USAGE_SIMULATION_ACCELERATOR: u16 = 196;
pub const HID_USAGE_SIMULATION_AILERON: u16 = 176;
pub const HID_USAGE_SIMULATION_AILERON_TRIM: u16 = 177;
pub const HID_USAGE_SIMULATION_AIRPLANE_SIMULATION_DEVICE: u16 = 9;
pub const HID_USAGE_SIMULATION_ANTI_TORQUE_CONTROL: u16 = 178;
pub const HID_USAGE_SIMULATION_AUTOMOBILE_SIMULATION_DEVICE: u16 = 2;
pub const HID_USAGE_SIMULATION_AUTOPIOLOT_ENABLE: u16 = 179;
pub const HID_USAGE_SIMULATION_BALLAST: u16 = 204;
pub const HID_USAGE_SIMULATION_BARREL_ELEVATION: u16 = 202;
pub const HID_USAGE_SIMULATION_BICYCLE_CRANK: u16 = 205;
pub const HID_USAGE_SIMULATION_BICYCLE_SIMULATION_DEVICE: u16 = 12;
pub const HID_USAGE_SIMULATION_BRAKE: u16 = 197;
pub const HID_USAGE_SIMULATION_CHAFF_RELEASE: u16 = 180;
pub const HID_USAGE_SIMULATION_CLUTCH: u16 = 198;
pub const HID_USAGE_SIMULATION_COLLECTIVE_CONTROL: u16 = 181;
pub const HID_USAGE_SIMULATION_CYCLIC_CONTROL: u16 = 34;
pub const HID_USAGE_SIMULATION_CYCLIC_TRIM: u16 = 35;
pub const HID_USAGE_SIMULATION_DIVE_BRAKE: u16 = 182;
pub const HID_USAGE_SIMULATION_DIVE_PLANE: u16 = 203;
pub const HID_USAGE_SIMULATION_ELECTRONIC_COUNTERMEASURES: u16 = 183;
pub const HID_USAGE_SIMULATION_ELEVATOR: u16 = 184;
pub const HID_USAGE_SIMULATION_ELEVATOR_TRIM: u16 = 185;
pub const HID_USAGE_SIMULATION_FLARE_RELEASE: u16 = 189;
pub const HID_USAGE_SIMULATION_FLIGHT_COMMUNICATIONS: u16 = 188;
pub const HID_USAGE_SIMULATION_FLIGHT_CONTROL_STICK: u16 = 32;
pub const HID_USAGE_SIMULATION_FLIGHT_SIMULATION_DEVICE: u16 = 1;
pub const HID_USAGE_SIMULATION_FLIGHT_STICK: u16 = 33;
pub const HID_USAGE_SIMULATION_FLIGHT_YOKE: u16 = 36;
pub const HID_USAGE_SIMULATION_FRONT_BRAKE: u16 = 207;
pub const HID_USAGE_SIMULATION_HANDLE_BARS: u16 = 206;
pub const HID_USAGE_SIMULATION_HELICOPTER_SIMULATION_DEVICE: u16 = 10;
pub const HID_USAGE_SIMULATION_LANDING_GEAR: u16 = 190;
pub const HID_USAGE_SIMULATION_MAGIC_CARPET_SIMULATION_DEVICE: u16 = 11;
pub const HID_USAGE_SIMULATION_MOTORCYCLE_SIMULATION_DEVICE: u16 = 7;
pub const HID_USAGE_SIMULATION_REAR_BRAKE: u16 = 208;
pub const HID_USAGE_SIMULATION_RUDDER: u16 = 186;
pub const HID_USAGE_SIMULATION_SAILING_SIMULATION_DEVICE: u16 = 6;
pub const HID_USAGE_SIMULATION_SHIFTER: u16 = 199;
pub const HID_USAGE_SIMULATION_SPACESHIP_SIMULATION_DEVICE: u16 = 4;
pub const HID_USAGE_SIMULATION_SPORTS_SIMULATION_DEVICE: u16 = 8;
pub const HID_USAGE_SIMULATION_STEERING: u16 = 200;
pub const HID_USAGE_SIMULATION_SUBMARINE_SIMULATION_DEVICE: u16 = 5;
pub const HID_USAGE_SIMULATION_TANK_SIMULATION_DEVICE: u16 = 3;
pub const HID_USAGE_SIMULATION_THROTTLE: u16 = 187;
pub const HID_USAGE_SIMULATION_TOE_BRAKE: u16 = 191;
pub const HID_USAGE_SIMULATION_TRACK_CONTROL: u16 = 37;
pub const HID_USAGE_SIMULATION_TRIGGER: u16 = 192;
pub const HID_USAGE_SIMULATION_TURRET_DIRECTION: u16 = 201;
pub const HID_USAGE_SIMULATION_WEAPONS_ARM: u16 = 193;
pub const HID_USAGE_SIMULATION_WEAPONS_SELECT: u16 = 194;
pub const HID_USAGE_SIMULATION_WING_FLAPS: u16 = 195;
pub const HID_USAGE_SOC_FILE_OFFSET_IN_BYTES: u16 = 4;
pub const HID_USAGE_SOC_FILE_PAYLOAD: u16 = 6;
pub const HID_USAGE_SOC_FILE_PAYLOAD_CONTAINS_LAST_BYTES: u16 = 8;
pub const HID_USAGE_SOC_FILE_PAYLOAD_SIZE_IN_BYTES: u16 = 7;
pub const HID_USAGE_SOC_FILE_TRANSFER_SIZE_MAX_IN_BYTES: u16 = 5;
pub const HID_USAGE_SOC_FILE_TRANSFER_STOP: u16 = 9;
pub const HID_USAGE_SOC_FILE_TRANSFER_TILL_END: u16 = 10;
pub const HID_USAGE_SOC_FIRMWARE_FILE_ID: u16 = 3;
pub const HID_USAGE_SOC_FIRMWARE_TRANSFER: u16 = 2;
pub const HID_USAGE_SOC_SOC_CONTROL: u16 = 1;
pub const HID_USAGE_SPORT_10_IRON: u16 = 90;
pub const HID_USAGE_SPORT_11_IRON: u16 = 91;
pub const HID_USAGE_SPORT_1_IRON: u16 = 81;
pub const HID_USAGE_SPORT_1_WOOD: u16 = 95;
pub const HID_USAGE_SPORT_2_IRON: u16 = 82;
pub const HID_USAGE_SPORT_3_IRON: u16 = 83;
pub const HID_USAGE_SPORT_3_WOOD: u16 = 96;
pub const HID_USAGE_SPORT_4_IRON: u16 = 84;
pub const HID_USAGE_SPORT_5_IRON: u16 = 85;
pub const HID_USAGE_SPORT_5_WOOD: u16 = 97;
pub const HID_USAGE_SPORT_6_IRON: u16 = 86;
pub const HID_USAGE_SPORT_7_IRON: u16 = 87;
pub const HID_USAGE_SPORT_7_WOOD: u16 = 98;
pub const HID_USAGE_SPORT_8_IRON: u16 = 88;
pub const HID_USAGE_SPORT_9_IRON: u16 = 89;
pub const HID_USAGE_SPORT_9_WOOD: u16 = 99;
pub const HID_USAGE_SPORT_BASEBALL_BAT: u16 = 1;
pub const HID_USAGE_SPORT_FOLLOW_THROUGH: u16 = 54;
pub const HID_USAGE_SPORT_GOLF_CLUB: u16 = 2;
pub const HID_USAGE_SPORT_HEEL_TOE: u16 = 53;
pub const HID_USAGE_SPORT_HEIGHT: u16 = 57;
pub const HID_USAGE_SPORT_LOFT_WEDGE: u16 = 93;
pub const HID_USAGE_SPORT_OAR: u16 = 48;
pub const HID_USAGE_SPORT_POWER_WEDGE: u16 = 94;
pub const HID_USAGE_SPORT_PUTTER: u16 = 80;
pub const HID_USAGE_SPORT_RATE: u16 = 50;
pub const HID_USAGE_SPORT_ROWING_MACHINE: u16 = 3;
pub const HID_USAGE_SPORT_SAND_WEDGE: u16 = 92;
pub const HID_USAGE_SPORT_SLOPE: u16 = 49;
pub const HID_USAGE_SPORT_STICK_FACE_ANGLE: u16 = 52;
pub const HID_USAGE_SPORT_STICK_SPEED: u16 = 51;
pub const HID_USAGE_SPORT_STICK_TYPE: u16 = 56;
pub const HID_USAGE_SPORT_TEMPO: u16 = 55;
pub const HID_USAGE_SPORT_TREADMILL: u16 = 4;
pub const HID_USAGE_TELEPHONY_ACTIVATE_HANDSET_AUDIO: u16 = 243;
pub const HID_USAGE_TELEPHONY_ADDRESS_BOOK_ID: u16 = 327;
pub const HID_USAGE_TELEPHONY_ALERT_SOUND_CONFIRM: u16 = 252;
pub const HID_USAGE_TELEPHONY_ALERT_SOUND_ERROR: u16 = 251;
pub const HID_USAGE_TELEPHONY_ALERT_SOUND_NOTIFICATION: u16 = 253;
pub const HID_USAGE_TELEPHONY_ALTERNATE_FUNCTION: u16 = 41;
pub const HID_USAGE_TELEPHONY_ANSWERING_MACHINE: u16 = 2;
pub const HID_USAGE_TELEPHONY_ANSWER_ONOFF: u16 = 116;
pub const HID_USAGE_TELEPHONY_CALLER_ID: u16 = 48;
pub const HID_USAGE_TELEPHONY_CALL_DURATION: u16 = 330;
pub const HID_USAGE_TELEPHONY_CALL_WAITING_TONE: u16 = 153;
pub const HID_USAGE_TELEPHONY_CONFERENCE: u16 = 44;
pub const HID_USAGE_TELEPHONY_CONFIRMATION_TONE_1: u16 = 154;
pub const HID_USAGE_TELEPHONY_CONFIRMATION_TONE_2: u16 = 155;
pub const HID_USAGE_TELEPHONY_DO_NOT_DISTURB: u16 = 114;
pub const HID_USAGE_TELEPHONY_DROP: u16 = 38;
pub const HID_USAGE_TELEPHONY_DUAL_MODE_PHONE: u16 = 331;
pub const HID_USAGE_TELEPHONY_EMAIL_MESSAGE_WAITING: u16 = 264;
pub const HID_USAGE_TELEPHONY_FEATURE: u16 = 34;
pub const HID_USAGE_TELEPHONY_FLASH: u16 = 33;
pub const HID_USAGE_TELEPHONY_FORWARD_CALLS: u16 = 40;
pub const HID_USAGE_TELEPHONY_HANDSET: u16 = 4;
pub const HID_USAGE_TELEPHONY_HANDSET_NICKNAME: u16 = 326;
pub const HID_USAGE_TELEPHONY_HEADSET: u16 = 5;
pub const HID_USAGE_TELEPHONY_HOLD: u16 = 35;
pub const HID_USAGE_TELEPHONY_HOOK_SWITCH: u16 = 32;
pub const HID_USAGE_TELEPHONY_HOST_AVAILABLE: u16 = 241;
pub const HID_USAGE_TELEPHONY_HOST_CALL_ACTIVE: u16 = 242;
pub const HID_USAGE_TELEPHONY_HOST_CONTROL: u16 = 240;
pub const HID_USAGE_TELEPHONY_HOST_HOLD: u16 = 266;
pub const HID_USAGE_TELEPHONY_HOST_RING_TONE: u16 = 250;
pub const HID_USAGE_TELEPHONY_INCOMING_CALL_HISTORY: u16 = 274;
pub const HID_USAGE_TELEPHONY_INCOMING_CALL_HISTORY_COUNT: u16 = 272;
pub const HID_USAGE_TELEPHONY_INSIDE_DIAL_TONE: u16 = 144;
pub const HID_USAGE_TELEPHONY_INSIDE_RINGBACK: u16 = 149;
pub const HID_USAGE_TELEPHONY_INSIDE_RING_TONE: u16 = 146;
pub const HID_USAGE_TELEPHONY_KEYPAD: u16 = 6;
pub const HID_USAGE_TELEPHONY_LINE: u16 = 42;
pub const HID_USAGE_TELEPHONY_LINE_BUSY_TONE: u16 = 151;
pub const HID_USAGE_TELEPHONY_MESSAGE: u16 = 115;
pub const HID_USAGE_TELEPHONY_MESSAGE_CONTROLS: u16 = 3;
pub const HID_USAGE_TELEPHONY_OUTGOING_CALL_HISTORY: u16 = 275;
pub const HID_USAGE_TELEPHONY_OUTGOING_CALL_HISTORY_COUNT: u16 = 273;
pub const HID_USAGE_TELEPHONY_OUTSIDE_DIAL_TONE: u16 = 145;
pub const HID_USAGE_TELEPHONY_OUTSIDE_RINGBACK: u16 = 157;
pub const HID_USAGE_TELEPHONY_OUTSIDE_RING_TONE: u16 = 147;
pub const HID_USAGE_TELEPHONY_PARK: u16 = 39;
pub const HID_USAGE_TELEPHONY_PHONE: u16 = 1;
pub const HID_USAGE_TELEPHONY_PHONE_CALLER_ID_KEY: u16 = 193;
pub const HID_USAGE_TELEPHONY_PHONE_CALL_HISTORY_KEY: u16 = 192;
pub const HID_USAGE_TELEPHONY_PHONE_DATE_DAY: u16 = 323;
pub const HID_USAGE_TELEPHONY_PHONE_DATE_MONTH: u16 = 324;
pub const HID_USAGE_TELEPHONY_PHONE_DATE_YEAR: u16 = 325;
pub const HID_USAGE_TELEPHONY_PHONE_DIRECTORY: u16 = 83;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_0: u16 = 176;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_1: u16 = 177;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_2: u16 = 178;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_3: u16 = 179;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_4: u16 = 180;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_5: u16 = 181;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_6: u16 = 182;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_7: u16 = 183;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_8: u16 = 184;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_9: u16 = 185;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_A: u16 = 188;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_B: u16 = 189;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_C: u16 = 190;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_D: u16 = 191;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_POUND: u16 = 187;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_STAR: u16 = 186;
pub const HID_USAGE_TELEPHONY_PHONE_LOCALE: u16 = 276;
pub const HID_USAGE_TELEPHONY_PHONE_MUTE: u16 = 47;
pub const HID_USAGE_TELEPHONY_PHONE_SETTINGS_KEY: u16 = 194;
pub const HID_USAGE_TELEPHONY_PHONE_TIME_HOUR: u16 = 322;
pub const HID_USAGE_TELEPHONY_PHONE_TIME_MINUTE: u16 = 321;
pub const HID_USAGE_TELEPHONY_PHONE_TIME_SECOND: u16 = 320;
pub const HID_USAGE_TELEPHONY_PRIORITY_RINGBACK: u16 = 150;
pub const HID_USAGE_TELEPHONY_PRIORITY_RING_TONE: u16 = 148;
pub const HID_USAGE_TELEPHONY_PROGRAMMABLE_BUTTON: u16 = 7;
pub const HID_USAGE_TELEPHONY_PSTN_RING_TONE: u16 = 249;
pub const HID_USAGE_TELEPHONY_RECALL_NUMBER: u16 = 82;
pub const HID_USAGE_TELEPHONY_REDIAL: u16 = 36;
pub const HID_USAGE_TELEPHONY_REDIALABLE_PHONE_NUMBER: u16 = 245;
pub const HID_USAGE_TELEPHONY_REORDER_TONE: u16 = 152;
pub const HID_USAGE_TELEPHONY_RINGER: u16 = 158;
pub const HID_USAGE_TELEPHONY_RING_ENABLE: u16 = 45;
pub const HID_USAGE_TELEPHONY_RING_SELECT: u16 = 46;
pub const HID_USAGE_TELEPHONY_RING_TYPE: u16 = 244;
pub const HID_USAGE_TELEPHONY_SCREEN_CALLS: u16 = 113;
pub const HID_USAGE_TELEPHONY_SEND: u16 = 49;
pub const HID_USAGE_TELEPHONY_SILENT_RING: u16 = 254;
pub const HID_USAGE_TELEPHONY_SPEAKER_PHONE: u16 = 43;
pub const HID_USAGE_TELEPHONY_SPEED_DIAL: u16 = 80;
pub const HID_USAGE_TELEPHONY_STOP_RING_TONE: u16 = 248;
pub const HID_USAGE_TELEPHONY_STORE_NUMBER: u16 = 81;
pub const HID_USAGE_TELEPHONY_TONES_OFF: u16 = 156;
pub const HID_USAGE_TELEPHONY_TRANSFER: u16 = 37;
pub const HID_USAGE_TELEPHONY_VOICEMAIL_MESSAGE_WAITING: u16 = 265;
pub const HID_USAGE_TELEPHONY_VOICE_MAIL: u16 = 112;
pub const HID_USAGE_UNDEFINED: u16 = 0;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_AUTO_SIZE_CENTER: u16 = 162;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_BLUE_VIDEO_BLACK_LEVEL: u16 = 112;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_BLUE_VIDEO_GAIN: u16 = 26;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_BOTTOM_CORNER_DISTORTION_BALANCE: u16 = 76;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_BOTTOM_CORNER_DISTORTION_CONTROL: u16 = 74;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_BRIGHTNESS: u16 = 16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_CONTRAST: u16 = 18;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_DEGAUSS: u16 = 1;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_FOCUS: u16 = 28;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_GREEN_VIDEO_BLACK_LEVEL: u16 = 110;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_GREEN_VIDEO_GAIN: u16 = 24;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_FREQUENCY: u16 = 172;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_LINEARITY: u16 = 42;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_LINEARITY_BALANCE: u16 = 44;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_MISCONVERGENCE: u16 = 40;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_MOIRE: u16 = 86;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_PINCUSHION: u16 = 36;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_PINCUSHION_BALANCE: u16 = 38;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_POSITION: u16 = 32;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_SIZE: u16 = 34;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_INPUT_LEVEL_SELECT: u16 = 94;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_INPUT_SOURCE_SELECT: u16 = 96;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_ON_SCREEN_DISPLAY: u16 = 202;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_PARALLELOGRAM_DISTORTION_KEY_BALANCE: u16 = 64;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_POLARITY_HORIZONTAL_SYNCHRONIZATION: u16 = 164;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_POLARITY_VERTICAL_SYNCHRONIZATION: u16 = 166;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_RED_VIDEO_BLACK_LEVEL: u16 = 108;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_RED_VIDEO_GAIN: u16 = 22;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_SCREEN_ORIENTATION: u16 = 170;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_SETTINGS: u16 = 176;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_STEREO_MODE: u16 = 212;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_SYNCHRONIZATION_TYPE: u16 = 168;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_TILT_ROTATION: u16 = 68;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_TOP_CORNER_DISTORTION_BALANCE: u16 = 72;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_TOP_CORNER_DISTORTION_CONTROL: u16 = 70;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_TRAPEZOIDAL_DISTORTION_KEY: u16 = 66;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_FREQUENCY: u16 = 174;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_LINEARITY: u16 = 58;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_LINEARITY_BALANCE: u16 = 60;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_MISCONVERGENCE: u16 = 56;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_MOIRE: u16 = 88;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_PINCUSHION: u16 = 52;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_PINCUSHION_BALANCE: u16 = 54;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_POSITION: u16 = 48;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_SIZE: u16 = 50;
pub const HID_USAGE_VR_ANIMATRONIC_DEVICE: u16 = 10;
pub const HID_USAGE_VR_BELT: u16 = 1;
pub const HID_USAGE_VR_BODY_SUIT: u16 = 2;
pub const HID_USAGE_VR_DISPLAY_ENABLE: u16 = 33;
pub const HID_USAGE_VR_FLEXOR: u16 = 3;
pub const HID_USAGE_VR_GLOVE: u16 = 4;
pub const HID_USAGE_VR_HAND_TRACKER: u16 = 7;
pub const HID_USAGE_VR_HEAD_MOUNTED_DISPLAY: u16 = 6;
pub const HID_USAGE_VR_HEAD_TRACKER: u16 = 5;
pub const HID_USAGE_VR_OCULOMETER: u16 = 8;
pub const HID_USAGE_VR_STEREO_ENABLE: u16 = 32;
pub const HID_USAGE_VR_VEST: u16 = 9;
pub const HID_USAGE_WEIGHING_DEVICE_CALIBRATION_COUNT: u16 = 96;
pub const HID_USAGE_WEIGHING_DEVICE_DATA_SCALING: u16 = 65;
pub const HID_USAGE_WEIGHING_DEVICE_DATA_WEIGHT: u16 = 64;
pub const HID_USAGE_WEIGHING_DEVICE_ENFORCED_ZERO_RETURN: u16 = 129;
pub const HID_USAGE_WEIGHING_DEVICE_REZERO_COUNT: u16 = 97;
pub const HID_USAGE_WEIGHING_DEVICE_SCALES: u16 = 1;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_ATTRIBUTE_REPORT: u16 = 48;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS: u16 = 33;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_GENERIC: u16 = 42;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_IIIL_ENGLISH: u16 = 40;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_IIIL_METRIC: u16 = 37;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_III_ENGLISH: u16 = 39;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_III_METRIC: u16 = 36;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_II_METRIC: u16 = 35;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_IV_ENGLISH: u16 = 41;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_IV_METRIC: u16 = 38;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_I_METRIC: u16 = 34;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CONTROL_REPORT: u16 = 49;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_DATA_REPORT: u16 = 50;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_DEVICE: u16 = 32;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATISTICS_REPORT: u16 = 53;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS: u16 = 112;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_FAULT: u16 = 113;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_IN_MOTION: u16 = 115;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_OVER_WEIGHT_LIMIT: u16 = 118;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_REPORT: u16 = 51;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_REQUIRES_CALIBRATION: u16 = 119;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_REQUIRES_REZEROING: u16 = 120;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_STABLE_AT_CENTER_OF_ZERO: u16 = 114;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_UNDER_ZERO: u16 = 117;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_WEIGHT_STABLE: u16 = 116;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_WEIGHT_LIMIT_REPORT: u16 = 52;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT: u16 = 80;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_AVOIR_TON: u16 = 89;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_CARATS: u16 = 84;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_GRAINS: u16 = 86;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_GRAM: u16 = 82;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_KILOGRAM: u16 = 83;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_METRIC_TON: u16 = 88;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_MILLIGRAM: u16 = 81;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_OUNCE: u16 = 91;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_PENNYWEIGHTS: u16 = 87;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_POUND: u16 = 92;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_TAELS: u16 = 85;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_TROY_OUNCE: u16 = 90;
pub const HID_USAGE_WEIGHING_DEVICE_ZERO_SCALE: u16 = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HID_XFER_PACKET {
    pub reportBuffer: *mut u8,
    pub reportBufferLen: u32,
    pub reportId: u8,
}
impl Default for HID_XFER_PACKET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HORIZONTAL_WHEEL_PRESENT: u32 = 32768;
pub const HidP_Feature: HIDP_REPORT_TYPE = 2;
pub const HidP_Input: HIDP_REPORT_TYPE = 0;
pub const HidP_Keyboard_Break: HIDP_KEYBOARD_DIRECTION = 0;
pub const HidP_Keyboard_Make: HIDP_KEYBOARD_DIRECTION = 1;
pub const HidP_Output: HIDP_REPORT_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INDICATOR_LIST {
    pub MakeCode: u16,
    pub IndicatorFlags: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INPUT_BUTTON_ENABLE_INFO {
    pub ButtonType: GPIOBUTTONS_BUTTON_TYPE,
    pub Enabled: bool,
}
pub const IOCTL_BUTTON_GET_ENABLED_ON_IDLE: u32 = 721580;
pub const IOCTL_BUTTON_SET_ENABLED_ON_IDLE: u32 = 721576;
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
pub const IOCTL_MOUSE_INSERT_DATA: u32 = 983044;
pub const IOCTL_MOUSE_QUERY_ATTRIBUTES: u32 = 983040;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOYCALIBRATE {
    pub wXbase: u32,
    pub wXdelta: u32,
    pub wYbase: u32,
    pub wYdelta: u32,
    pub wZbase: u32,
    pub wZdelta: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOYPOS {
    pub dwX: u32,
    pub dwY: u32,
    pub dwZ: u32,
    pub dwR: u32,
    pub dwU: u32,
    pub dwV: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOYRANGE {
    pub jpMin: JOYPOS,
    pub jpMax: JOYPOS,
    pub jpCenter: JOYPOS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOYREGHWCONFIG {
    pub hws: JOYREGHWSETTINGS,
    pub dwUsageSettings: u32,
    pub hwv: JOYREGHWVALUES,
    pub dwType: u32,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOYREGHWSETTINGS {
    pub dwFlags: u32,
    pub dwNumButtons: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JOYREGHWVALUES {
    pub jrvHardware: JOYRANGE,
    pub dwPOVValues: [u32; 4],
    pub dwCalFlags: u32,
}
impl Default for JOYREGHWVALUES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOYREGUSERVALUES {
    pub dwTimeOut: u32,
    pub jrvRanges: JOYRANGE,
    pub jpDeadZone: JOYPOS,
}
pub const JOYTYPE_ANALOGCOMPAT: i32 = 8;
pub const JOYTYPE_DEFAULTPROPSHEET: i32 = -2147483648;
pub const JOYTYPE_DEVICEHIDE: i32 = 65536;
pub const JOYTYPE_ENABLEINPUTREPORT: i32 = 16777216;
pub const JOYTYPE_GAMEHIDE: i32 = 524288;
pub const JOYTYPE_HIDEACTIVE: i32 = 1048576;
pub const JOYTYPE_INFODEFAULT: i32 = 0;
pub const JOYTYPE_INFOMASK: i32 = 14680064;
pub const JOYTYPE_INFOYRPEDALS: i32 = 6291456;
pub const JOYTYPE_INFOYYPEDALS: i32 = 2097152;
pub const JOYTYPE_INFOZISSLIDER: i32 = 2097152;
pub const JOYTYPE_INFOZISZ: i32 = 4194304;
pub const JOYTYPE_INFOZRPEDALS: i32 = 8388608;
pub const JOYTYPE_INFOZYPEDALS: i32 = 4194304;
pub const JOYTYPE_KEYBHIDE: i32 = 262144;
pub const JOYTYPE_MOUSEHIDE: i32 = 131072;
pub const JOYTYPE_NOAUTODETECTGAMEPORT: i32 = 2;
pub const JOYTYPE_NOHIDDIRECT: i32 = 4;
pub const JOYTYPE_ZEROGAMEENUMOEMDATA: i32 = 1;
pub const JOY_HWS_AUTOLOAD: i32 = 268435456;
pub const JOY_HWS_GAMEPORTBUSBUSY: i32 = 1;
pub const JOY_HWS_HASPOV: i32 = 2;
pub const JOY_HWS_HASR: i32 = 524288;
pub const JOY_HWS_HASU: i32 = 8388608;
pub const JOY_HWS_HASV: i32 = 16777216;
pub const JOY_HWS_HASZ: i32 = 1;
pub const JOY_HWS_ISANALOGPORTDRIVER: i32 = 134217728;
pub const JOY_HWS_ISCARCTRL: i32 = 64;
pub const JOY_HWS_ISGAMEPAD: i32 = 32;
pub const JOY_HWS_ISGAMEPORTBUS: i32 = -2147483648;
pub const JOY_HWS_ISGAMEPORTDRIVER: i32 = 67108864;
pub const JOY_HWS_ISHEADTRACKER: i32 = 33554432;
pub const JOY_HWS_ISYOKE: i32 = 16;
pub const JOY_HWS_NODEVNODE: i32 = 536870912;
pub const JOY_HWS_POVISBUTTONCOMBOS: i32 = 4;
pub const JOY_HWS_POVISJ1X: i32 = 65536;
pub const JOY_HWS_POVISJ1Y: i32 = 131072;
pub const JOY_HWS_POVISJ2X: i32 = 262144;
pub const JOY_HWS_POVISPOLL: i32 = 8;
pub const JOY_HWS_RISJ1X: i32 = 1048576;
pub const JOY_HWS_RISJ1Y: i32 = 2097152;
pub const JOY_HWS_RISJ2Y: i32 = 4194304;
pub const JOY_HWS_XISJ1Y: i32 = 128;
pub const JOY_HWS_XISJ2X: i32 = 256;
pub const JOY_HWS_XISJ2Y: i32 = 512;
pub const JOY_HWS_YISJ1X: i32 = 1024;
pub const JOY_HWS_YISJ2X: i32 = 2048;
pub const JOY_HWS_YISJ2Y: i32 = 4096;
pub const JOY_HWS_ZISJ1X: i32 = 8192;
pub const JOY_HWS_ZISJ1Y: i32 = 16384;
pub const JOY_HWS_ZISJ2X: i32 = 32768;
pub const JOY_HW_2A_2B_GENERIC: u32 = 2;
pub const JOY_HW_2A_4B_GENERIC: u32 = 3;
pub const JOY_HW_2B_FLIGHTYOKE: u32 = 5;
pub const JOY_HW_2B_FLIGHTYOKETHROTTLE: u32 = 6;
pub const JOY_HW_2B_GAMEPAD: u32 = 4;
pub const JOY_HW_3A_2B_GENERIC: u32 = 7;
pub const JOY_HW_3A_4B_GENERIC: u32 = 8;
pub const JOY_HW_4B_FLIGHTYOKE: u32 = 10;
pub const JOY_HW_4B_FLIGHTYOKETHROTTLE: u32 = 11;
pub const JOY_HW_4B_GAMEPAD: u32 = 9;
pub const JOY_HW_CUSTOM: u32 = 1;
pub const JOY_HW_LASTENTRY: u32 = 13;
pub const JOY_HW_NONE: u32 = 0;
pub const JOY_HW_TWO_2A_2B_WITH_Y: u32 = 12;
pub const JOY_ISCAL_POV: i32 = 32;
pub const JOY_ISCAL_R: i32 = 4;
pub const JOY_ISCAL_U: i32 = 8;
pub const JOY_ISCAL_V: i32 = 16;
pub const JOY_ISCAL_XY: i32 = 1;
pub const JOY_ISCAL_Z: i32 = 2;
pub const JOY_OEMPOLL_PASSDRIVERDATA: u32 = 7;
pub const JOY_PASSDRIVERDATA: i32 = 268435456;
pub const JOY_POVVAL_BACKWARD: u32 = 1;
pub const JOY_POVVAL_FORWARD: u32 = 0;
pub const JOY_POVVAL_LEFT: u32 = 2;
pub const JOY_POVVAL_RIGHT: u32 = 3;
pub const JOY_POV_NUMDIRS: u32 = 4;
pub const JOY_US_HASRUDDER: i32 = 1;
pub const JOY_US_ISOEM: i32 = 4;
pub const JOY_US_PRESENT: i32 = 2;
pub const JOY_US_RESERVED: i32 = -2147483648;
pub const JOY_US_VOLATILE: i32 = 8;
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
pub const KEYBOARD_OVERRUN_MAKE_CODE: u32 = 255;
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
pub type LPDICONFIGUREDEVICESCALLBACK = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type LPDIENUMCREATEDEFFECTOBJECTSCALLBACK = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type LPDIENUMDEVICEOBJECTSCALLBACKA = Option<unsafe extern "system" fn(param0: *mut DIDEVICEOBJECTINSTANCEA, param1: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type LPDIENUMDEVICEOBJECTSCALLBACKW = Option<unsafe extern "system" fn(param0: *mut DIDEVICEOBJECTINSTANCEW, param1: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type LPDIENUMDEVICESBYSEMANTICSCBA = Option<unsafe extern "system" fn(param0: *mut DIDEVICEINSTANCEA, param1: *mut core::ffi::c_void, param2: u32, param3: u32, param4: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type LPDIENUMDEVICESBYSEMANTICSCBW = Option<unsafe extern "system" fn(param0: *mut DIDEVICEINSTANCEW, param1: *mut core::ffi::c_void, param2: u32, param3: u32, param4: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type LPDIENUMDEVICESCALLBACKA = Option<unsafe extern "system" fn(param0: *mut DIDEVICEINSTANCEA, param1: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type LPDIENUMDEVICESCALLBACKW = Option<unsafe extern "system" fn(param0: *mut DIDEVICEINSTANCEW, param1: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type LPDIENUMEFFECTSCALLBACKA = Option<unsafe extern "system" fn(param0: *mut DIEFFECTINFOA, param1: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type LPDIENUMEFFECTSCALLBACKW = Option<unsafe extern "system" fn(param0: *mut DIEFFECTINFOW, param1: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type LPDIENUMEFFECTSINFILECALLBACK = Option<unsafe extern "system" fn(param0: *mut DIFILEEFFECT, param1: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type LPDIJOYTYPECALLBACK = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type LPFNSHOWJOYCPL = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND)>;
pub const MAXCPOINTSNUM: u32 = 8;
pub const MAX_JOYSTICKOEMVXDNAME: u32 = 260;
pub const MAX_JOYSTRING: u32 = 256;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MOUSE_UNIT_ID_PARAMETER {
    pub UnitId: u16,
}
pub const MOUSE_WHEEL: u32 = 1024;
pub type PFN_HidP_GetVersionInternal = Option<unsafe extern "system" fn(version: *mut u32) -> super::super::Foundation::NTSTATUS>;
pub type PHIDP_INSERT_SCANCODES = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, newscancodes: windows_sys::core::PCSTR, length: u32) -> bool>;
pub type PHIDP_PREPARSED_DATA = isize;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USAGE_AND_PAGE {
    pub Usage: u16,
    pub UsagePage: u16,
}
pub const WHEELMOUSE_HID_HARDWARE: u32 = 256;
pub const WHEELMOUSE_I8042_HARDWARE: u32 = 32;
pub const WHEELMOUSE_SERIAL_HARDWARE: u32 = 64;
