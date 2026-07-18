#[cfg(feature = "windef")]
windows_link::link!("oleacc.dll" "system" fn AccNotifyTouchInteraction(hwndapp : super::HWND, hwndtarget : super::HWND, pttarget : super::POINT) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("oleacc.dll" "system" fn AccSetRunningUtilityState(hwndapp : super::HWND, dwutilitystatemask : u32, dwutilitystate : u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleacc.dll" "system" fn AccessibleChildren(pacccontainer : *mut core::ffi::c_void, ichildstart : i32, cchildren : i32, rgvarchildren : *mut super::VARIANT, pcobtained : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleacc.dll" "system" fn AccessibleObjectFromEvent(hwnd : super::HWND, dwid : u32, dwchildid : u32, ppacc : *mut *mut core::ffi::c_void, pvarchild : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleacc.dll" "system" fn AccessibleObjectFromPoint(ptscreen : super::POINT, ppacc : *mut *mut core::ffi::c_void, pvarchild : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("oleacc.dll" "system" fn AccessibleObjectFromWindow(hwnd : super::HWND, dwid : u32, riid : *const windows_sys::core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("oleacc.dll" "system" fn CreateStdAccessibleObject(hwnd : super::HWND, idobject : i32, riid : *const windows_sys::core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("oleacc.dll" "system" fn CreateStdAccessibleProxyA(hwnd : super::HWND, pclassname : windows_sys::core::PCSTR, idobject : i32, riid : *const windows_sys::core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("oleacc.dll" "system" fn CreateStdAccessibleProxyW(hwnd : super::HWND, pclassname : windows_sys::core::PCWSTR, idobject : i32, riid : *const windows_sys::core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("oleacc.dll" "system" fn GetOleaccVersionInfo(pver : *mut u32, pbuild : *mut u32));
windows_link::link!("oleacc.dll" "system" fn GetRoleTextA(lrole : u32, lpszrole : windows_sys::core::PSTR, cchrolemax : u32) -> u32);
windows_link::link!("oleacc.dll" "system" fn GetRoleTextW(lrole : u32, lpszrole : windows_sys::core::PWSTR, cchrolemax : u32) -> u32);
windows_link::link!("oleacc.dll" "system" fn GetStateTextA(lstatebit : u32, lpszstate : windows_sys::core::PSTR, cchstate : u32) -> u32);
windows_link::link!("oleacc.dll" "system" fn GetStateTextW(lstatebit : u32, lpszstate : windows_sys::core::PWSTR, cchstate : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("oleacc.dll" "system" fn LresultFromObject(riid : *const windows_sys::core::GUID, wparam : super::WPARAM, punk : *mut core::ffi::c_void) -> super::LRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("oleacc.dll" "system" fn ObjectFromLresult(lresult : super::LRESULT, riid : *const windows_sys::core::GUID, wparam : super::WPARAM, ppvobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "windef"))]
windows_link::link!("oleacc.dll" "system" fn WindowFromAccessibleObject(param0 : *mut core::ffi::c_void, phwnd : *mut super::HWND) -> windows_sys::core::HRESULT);
pub const ANNO_CONTAINER: AnnoScope = 1;
pub const ANNO_THIS: AnnoScope = 0;
pub const ANRUS_ON_SCREEN_KEYBOARD_ACTIVE: u32 = 1;
pub const ANRUS_PRIORITY_AUDIO_ACTIVE: u32 = 4;
pub const ANRUS_PRIORITY_AUDIO_ACTIVE_NODUCK: u32 = 8;
pub const ANRUS_PRIORITY_AUDIO_DYNAMIC_DUCK: u32 = 16;
pub const ANRUS_TOUCH_MODIFICATION_ACTIVE: u32 = 2;
pub type AnnoScope = i32;
pub const CAccPropServices: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb5f8350b_0548_48b1_a6ee_88bd00b4a5e7);
pub const CLSID_AccPropServices: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb5f8350b_0548_48b1_a6ee_88bd00b4a5e7);
pub const DISPID_ACC_CHILD: i32 = -5002;
pub const DISPID_ACC_CHILDCOUNT: i32 = -5001;
pub const DISPID_ACC_DEFAULTACTION: i32 = -5013;
pub const DISPID_ACC_DESCRIPTION: i32 = -5005;
pub const DISPID_ACC_DODEFAULTACTION: i32 = -5018;
pub const DISPID_ACC_FOCUS: i32 = -5011;
pub const DISPID_ACC_HELP: i32 = -5008;
pub const DISPID_ACC_HELPTOPIC: i32 = -5009;
pub const DISPID_ACC_HITTEST: i32 = -5017;
pub const DISPID_ACC_KEYBOARDSHORTCUT: i32 = -5010;
pub const DISPID_ACC_LOCATION: i32 = -5015;
pub const DISPID_ACC_NAME: i32 = -5003;
pub const DISPID_ACC_NAVIGATE: i32 = -5016;
pub const DISPID_ACC_PARENT: i32 = -5000;
pub const DISPID_ACC_ROLE: i32 = -5006;
pub const DISPID_ACC_SELECT: i32 = -5014;
pub const DISPID_ACC_SELECTION: i32 = -5012;
pub const DISPID_ACC_STATE: i32 = -5007;
pub const DISPID_ACC_VALUE: i32 = -5004;
pub const IID_IAccPropMgrInternal: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2bd370a9_3e7f_4edd_8a85_f8fed1f8e51f);
pub const IIS_ControlAccessible: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x38c682a6_9731_43f2_9fae_e901e641b101);
pub const IIS_IsOleaccProxy: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x902697fa_80e4_4560_802a_a13f22a64709);
pub const LIBID_Accessibility: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1ea4dbf0_3c3b_11cf_810c_00aa00389b71);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub type LPFNACCESSIBLECHILDREN = Option<unsafe extern "system" fn(pacccontainer: *mut core::ffi::c_void, ichildstart: i32, cchildren: i32, rgvarchildren: *mut super::VARIANT, pcobtained: *mut i32) -> windows_sys::core::HRESULT>;
#[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub type LPFNACCESSIBLEOBJECTFROMPOINT = Option<unsafe extern "system" fn(ptscreen: super::POINT, ppacc: *mut *mut core::ffi::c_void, pvarchild: *mut super::VARIANT) -> windows_sys::core::HRESULT>;
#[cfg(feature = "windef")]
pub type LPFNACCESSIBLEOBJECTFROMWINDOW = Option<unsafe extern "system" fn(hwnd: super::HWND, dwid: u32, riid: *const windows_sys::core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[cfg(feature = "windef")]
pub type LPFNCREATESTDACCESSIBLEOBJECT = Option<unsafe extern "system" fn(hwnd: super::HWND, idobject: i32, riid: *const windows_sys::core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
#[cfg(feature = "minwindef")]
pub type LPFNLRESULTFROMOBJECT = Option<unsafe extern "system" fn(riid: *const windows_sys::core::GUID, wparam: super::WPARAM, punk: *mut core::ffi::c_void) -> super::LRESULT>;
#[cfg(feature = "minwindef")]
pub type LPFNOBJECTFROMLRESULT = Option<unsafe extern "system" fn(lresult: super::LRESULT, riid: *const windows_sys::core::GUID, wparam: super::WPARAM, ppvobject: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub type LPMSAAMENUINFO = *mut MSAAMENUINFO;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSAAMENUINFO {
    pub dwMSAASignature: u32,
    pub cchWText: u32,
    pub pszWText: windows_sys::core::PWSTR,
}
impl Default for MSAAMENUINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MSAAPROPID = windows_sys::core::GUID;
pub const MSAA_MENU_SIG: u32 = 2853040141;
pub const NAVDIR_DOWN: u32 = 2;
pub const NAVDIR_FIRSTCHILD: u32 = 7;
pub const NAVDIR_LASTCHILD: u32 = 8;
pub const NAVDIR_LEFT: u32 = 3;
pub const NAVDIR_MAX: u32 = 9;
pub const NAVDIR_MIN: u32 = 0;
pub const NAVDIR_NEXT: u32 = 5;
pub const NAVDIR_PREVIOUS: u32 = 6;
pub const NAVDIR_RIGHT: u32 = 4;
pub const NAVDIR_UP: u32 = 1;
pub const PROPID_ACC_DEFAULTACTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x180c072b_c27f_43c7_9922_f63562a4632b);
pub const PROPID_ACC_DESCRIPTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4d48dfe4_bd3f_491f_a648_492d6f20c588);
pub const PROPID_ACC_DESCRIPTIONMAP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1ff1435f_8a14_477b_b226_a0abe279975d);
pub const PROPID_ACC_DODEFAULTACTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1ba09523_2e3b_49a6_a059_59682a3c48fd);
pub const PROPID_ACC_FOCUS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6eb335df_1c29_4127_b12c_dee9fd157f2b);
pub const PROPID_ACC_HELP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc831e11f_44db_4a99_9768_cb8f978b7231);
pub const PROPID_ACC_HELPTOPIC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x787d1379_8ede_440b_8aec_11f7bf9030b3);
pub const PROPID_ACC_KEYBOARDSHORTCUT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7d9bceee_7d1e_4979_9382_5180f4172c34);
pub const PROPID_ACC_NAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x608d3df8_8128_4aa7_a428_f55e49267291);
pub const PROPID_ACC_NAV_DOWN: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x031670ed_3cdf_48d2_9613_138f2dd8a668);
pub const PROPID_ACC_NAV_FIRSTCHILD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcfd02558_557b_4c67_84f9_2a09fce40749);
pub const PROPID_ACC_NAV_LASTCHILD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x302ecaa5_48d5_4f8d_b671_1a8d20a77832);
pub const PROPID_ACC_NAV_LEFT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x228086cb_82f1_4a39_8705_dcdc0fff92f5);
pub const PROPID_ACC_NAV_NEXT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1cdc5455_8cd9_4c92_a371_3939a2fe3eee);
pub const PROPID_ACC_NAV_PREV: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x776d3891_c73b_4480_b3f6_076a16a15af6);
pub const PROPID_ACC_NAV_RIGHT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcd211d9f_e1cb_4fe5_a77c_920b884d095b);
pub const PROPID_ACC_NAV_UP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x016e1a2b_1a4e_4767_8612_3386f66935ec);
pub const PROPID_ACC_PARENT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x474c22b6_ffc2_467a_b1b5_e958b4657330);
pub const PROPID_ACC_ROLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcb905ff2_7bd1_4c05_b3c8_e6c241364d70);
pub const PROPID_ACC_ROLEMAP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf79acda2_140d_4fe6_8914_208476328269);
pub const PROPID_ACC_SELECTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb99d073c_d731_405b_9061_d95e8f842984);
pub const PROPID_ACC_STATE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa8d4d5b0_0a21_42d0_a5c0_514e984f457b);
pub const PROPID_ACC_STATEMAP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x43946c5e_0ac0_4042_b525_07bbdbe17fa7);
pub const PROPID_ACC_VALUE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x123fe443_211a_4615_9527_c45a7e93717a);
pub const PROPID_ACC_VALUEMAP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xda1c3d79_fc5c_420e_b399_9d1533549e75);
pub const ROLE_SYSTEM_ALERT: u32 = 8;
pub const ROLE_SYSTEM_ANIMATION: u32 = 54;
pub const ROLE_SYSTEM_APPLICATION: u32 = 14;
pub const ROLE_SYSTEM_BORDER: u32 = 19;
pub const ROLE_SYSTEM_BUTTONDROPDOWN: u32 = 56;
pub const ROLE_SYSTEM_BUTTONDROPDOWNGRID: u32 = 58;
pub const ROLE_SYSTEM_BUTTONMENU: u32 = 57;
pub const ROLE_SYSTEM_CARET: u32 = 7;
pub const ROLE_SYSTEM_CELL: u32 = 29;
pub const ROLE_SYSTEM_CHARACTER: u32 = 32;
pub const ROLE_SYSTEM_CHART: u32 = 17;
pub const ROLE_SYSTEM_CHECKBUTTON: u32 = 44;
pub const ROLE_SYSTEM_CLIENT: u32 = 10;
pub const ROLE_SYSTEM_CLOCK: u32 = 61;
pub const ROLE_SYSTEM_COLUMN: u32 = 27;
pub const ROLE_SYSTEM_COLUMNHEADER: u32 = 25;
pub const ROLE_SYSTEM_COMBOBOX: u32 = 46;
pub const ROLE_SYSTEM_CURSOR: u32 = 6;
pub const ROLE_SYSTEM_DIAGRAM: u32 = 53;
pub const ROLE_SYSTEM_DIAL: u32 = 49;
pub const ROLE_SYSTEM_DIALOG: u32 = 18;
pub const ROLE_SYSTEM_DOCUMENT: u32 = 15;
pub const ROLE_SYSTEM_DROPLIST: u32 = 47;
pub const ROLE_SYSTEM_EQUATION: u32 = 55;
pub const ROLE_SYSTEM_GRAPHIC: u32 = 40;
pub const ROLE_SYSTEM_GRIP: u32 = 4;
pub const ROLE_SYSTEM_GROUPING: u32 = 20;
pub const ROLE_SYSTEM_HELPBALLOON: u32 = 31;
pub const ROLE_SYSTEM_HOTKEYFIELD: u32 = 50;
pub const ROLE_SYSTEM_INDICATOR: u32 = 39;
pub const ROLE_SYSTEM_IPADDRESS: u32 = 63;
pub const ROLE_SYSTEM_LINK: u32 = 30;
pub const ROLE_SYSTEM_LIST: u32 = 33;
pub const ROLE_SYSTEM_LISTITEM: u32 = 34;
pub const ROLE_SYSTEM_MENUBAR: u32 = 2;
pub const ROLE_SYSTEM_MENUITEM: u32 = 12;
pub const ROLE_SYSTEM_MENUPOPUP: u32 = 11;
pub const ROLE_SYSTEM_OUTLINE: u32 = 35;
pub const ROLE_SYSTEM_OUTLINEBUTTON: u32 = 64;
pub const ROLE_SYSTEM_OUTLINEITEM: u32 = 36;
pub const ROLE_SYSTEM_PAGETAB: u32 = 37;
pub const ROLE_SYSTEM_PAGETABLIST: u32 = 60;
pub const ROLE_SYSTEM_PANE: u32 = 16;
pub const ROLE_SYSTEM_PROGRESSBAR: u32 = 48;
pub const ROLE_SYSTEM_PROPERTYPAGE: u32 = 38;
pub const ROLE_SYSTEM_PUSHBUTTON: u32 = 43;
pub const ROLE_SYSTEM_RADIOBUTTON: u32 = 45;
pub const ROLE_SYSTEM_ROW: u32 = 28;
pub const ROLE_SYSTEM_ROWHEADER: u32 = 26;
pub const ROLE_SYSTEM_SCROLLBAR: u32 = 3;
pub const ROLE_SYSTEM_SEPARATOR: u32 = 21;
pub const ROLE_SYSTEM_SLIDER: u32 = 51;
pub const ROLE_SYSTEM_SOUND: u32 = 5;
pub const ROLE_SYSTEM_SPINBUTTON: u32 = 52;
pub const ROLE_SYSTEM_SPLITBUTTON: u32 = 62;
pub const ROLE_SYSTEM_STATICTEXT: u32 = 41;
pub const ROLE_SYSTEM_STATUSBAR: u32 = 23;
pub const ROLE_SYSTEM_TABLE: u32 = 24;
pub const ROLE_SYSTEM_TEXT: u32 = 42;
pub const ROLE_SYSTEM_TITLEBAR: u32 = 1;
pub const ROLE_SYSTEM_TOOLBAR: u32 = 22;
pub const ROLE_SYSTEM_TOOLTIP: u32 = 13;
pub const ROLE_SYSTEM_WHITESPACE: u32 = 59;
pub const ROLE_SYSTEM_WINDOW: u32 = 9;
pub const SELFLAG_ADDSELECTION: u32 = 8;
pub const SELFLAG_EXTENDSELECTION: u32 = 4;
pub const SELFLAG_NONE: u32 = 0;
pub const SELFLAG_REMOVESELECTION: u32 = 16;
pub const SELFLAG_TAKEFOCUS: u32 = 1;
pub const SELFLAG_TAKESELECTION: u32 = 2;
pub const SELFLAG_VALID: u32 = 31;
pub const STATE_SYSTEM_HASPOPUP: u32 = 1073741824;
