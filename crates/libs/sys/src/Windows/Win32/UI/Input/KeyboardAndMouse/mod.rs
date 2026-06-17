windows_link::link!("user32.dll" "system" fn ActivateKeyboardLayout(hkl : HKL, flags : ACTIVATE_KEYBOARD_LAYOUT_FLAGS) -> HKL);
windows_link::link!("user32.dll" "system" fn BlockInput(fblockit : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn DragDetect(hwnd : super::super::super::Foundation::HWND, pt : super::super::super::Foundation::POINT) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn EnableWindow(hwnd : super::super::super::Foundation::HWND, benable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetActiveWindow() -> super::super::super::Foundation::HWND);
windows_link::link!("user32.dll" "system" fn GetAsyncKeyState(vkey : i32) -> i16);
windows_link::link!("user32.dll" "system" fn GetCapture() -> super::super::super::Foundation::HWND);
windows_link::link!("user32.dll" "system" fn GetDoubleClickTime() -> u32);
windows_link::link!("user32.dll" "system" fn GetFocus() -> super::super::super::Foundation::HWND);
windows_link::link!("user32.dll" "system" fn GetKBCodePage() -> u32);
windows_link::link!("user32.dll" "system" fn GetKeyNameTextA(lparam : i32, lpstring : windows_sys::core::PSTR, cchsize : i32) -> i32);
windows_link::link!("user32.dll" "system" fn GetKeyNameTextW(lparam : i32, lpstring : windows_sys::core::PWSTR, cchsize : i32) -> i32);
windows_link::link!("user32.dll" "system" fn GetKeyState(nvirtkey : i32) -> i16);
windows_link::link!("user32.dll" "system" fn GetKeyboardLayout(idthread : u32) -> HKL);
windows_link::link!("user32.dll" "system" fn GetKeyboardLayoutList(nbuff : i32, lplist : *mut HKL) -> i32);
windows_link::link!("user32.dll" "system" fn GetKeyboardLayoutNameA(pwszklid : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetKeyboardLayoutNameW(pwszklid : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetKeyboardState(lpkeystate : *mut u8) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetKeyboardType(ntypeflag : i32) -> i32);
windows_link::link!("user32.dll" "system" fn GetLastInputInfo(plii : *mut LASTINPUTINFO) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn GetMouseMovePointsEx(cbsize : u32, lppt : *const MOUSEMOVEPOINT, lpptbuf : *mut MOUSEMOVEPOINT, nbufpoints : i32, resolution : GET_MOUSE_MOVE_POINTS_EX_RESOLUTION) -> i32);
windows_link::link!("user32.dll" "system" fn IsWindowEnabled(hwnd : super::super::super::Foundation::HWND) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn LoadKeyboardLayoutA(pwszklid : windows_sys::core::PCSTR, flags : ACTIVATE_KEYBOARD_LAYOUT_FLAGS) -> HKL);
windows_link::link!("user32.dll" "system" fn LoadKeyboardLayoutW(pwszklid : windows_sys::core::PCWSTR, flags : ACTIVATE_KEYBOARD_LAYOUT_FLAGS) -> HKL);
windows_link::link!("user32.dll" "system" fn MapVirtualKeyA(ucode : u32, umaptype : MAP_VIRTUAL_KEY_TYPE) -> u32);
windows_link::link!("user32.dll" "system" fn MapVirtualKeyExA(ucode : u32, umaptype : MAP_VIRTUAL_KEY_TYPE, dwhkl : HKL) -> u32);
windows_link::link!("user32.dll" "system" fn MapVirtualKeyExW(ucode : u32, umaptype : MAP_VIRTUAL_KEY_TYPE, dwhkl : HKL) -> u32);
windows_link::link!("user32.dll" "system" fn MapVirtualKeyW(ucode : u32, umaptype : MAP_VIRTUAL_KEY_TYPE) -> u32);
windows_link::link!("user32.dll" "system" fn OemKeyScan(woemchar : u16) -> u32);
windows_link::link!("user32.dll" "system" fn RegisterHotKey(hwnd : super::super::super::Foundation::HWND, id : i32, fsmodifiers : HOT_KEY_MODIFIERS, vk : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn ReleaseCapture() -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SendInput(cinputs : u32, pinputs : *const INPUT, cbsize : i32) -> u32);
windows_link::link!("user32.dll" "system" fn SetActiveWindow(hwnd : super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND);
windows_link::link!("user32.dll" "system" fn SetCapture(hwnd : super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND);
windows_link::link!("user32.dll" "system" fn SetDoubleClickTime(param0 : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SetFocus(hwnd : super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND);
windows_link::link!("user32.dll" "system" fn SetKeyboardState(lpkeystate : *const u8) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn SwapMouseButton(fswap : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn ToAscii(uvirtkey : u32, uscancode : u32, lpkeystate : *const u8, lpchar : *mut u16, uflags : u32) -> i32);
windows_link::link!("user32.dll" "system" fn ToAsciiEx(uvirtkey : u32, uscancode : u32, lpkeystate : *const u8, lpchar : *mut u16, uflags : u32, dwhkl : HKL) -> i32);
windows_link::link!("user32.dll" "system" fn ToUnicode(wvirtkey : u32, wscancode : u32, lpkeystate : *const u8, pwszbuff : windows_sys::core::PWSTR, cchbuff : i32, wflags : u32) -> i32);
windows_link::link!("user32.dll" "system" fn ToUnicodeEx(wvirtkey : u32, wscancode : u32, lpkeystate : *const u8, pwszbuff : windows_sys::core::PWSTR, cchbuff : i32, wflags : u32, dwhkl : HKL) -> i32);
windows_link::link!("user32.dll" "system" fn TrackMouseEvent(lpeventtrack : *mut TRACKMOUSEEVENT) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn UnloadKeyboardLayout(hkl : HKL) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn UnregisterHotKey(hwnd : super::super::super::Foundation::HWND, id : i32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn VkKeyScanA(ch : i8) -> i16);
windows_link::link!("user32.dll" "system" fn VkKeyScanExA(ch : i8, dwhkl : HKL) -> i16);
windows_link::link!("user32.dll" "system" fn VkKeyScanExW(ch : u16, dwhkl : HKL) -> i16);
windows_link::link!("user32.dll" "system" fn VkKeyScanW(ch : u16) -> i16);
windows_link::link!("comctl32.dll" "system" fn _TrackMouseEvent(lpeventtrack : *mut TRACKMOUSEEVENT) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn keybd_event(bvk : u8, bscan : u8, dwflags : KEYBD_EVENT_FLAGS, dwextrainfo : usize));
windows_link::link!("user32.dll" "system" fn mouse_event(dwflags : MOUSE_EVENT_FLAGS, dx : i32, dy : i32, dwdata : i32, dwextrainfo : usize));
pub type ACTIVATE_KEYBOARD_LAYOUT_FLAGS = u32;
pub const ACUTE: u32 = 769;
pub const AX_KBD_DESKTOP_TYPE: u32 = 1;
pub const BREVE: u32 = 774;
pub const CAPLOK: u32 = 1;
pub const CAPLOKALTGR: u32 = 4;
pub const CEDILLA: u32 = 807;
pub const CIRCUMFLEX: u32 = 770;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEADKEY {
    pub dwBoth: u32,
    pub wchComposed: u16,
    pub uFlags: u16,
}
pub const DEC_KBD_ANSI_LAYOUT_TYPE: u32 = 1;
pub const DEC_KBD_JIS_LAYOUT_TYPE: u32 = 2;
pub const DIARESIS: u32 = 776;
pub const DIARESIS_TONOS: u32 = 901;
pub const DKF_DEAD: u32 = 1;
pub const DONTCARE_BIT: u32 = 33554432;
pub const DOT_ABOVE: u32 = 775;
pub const DOUBLE_ACUTE: u32 = 779;
pub const EXTENDED_BIT: u32 = 16777216;
pub const FAKE_KEYSTROKE: u32 = 33554432;
pub const FMR_KBD_JIS_TYPE: u32 = 0;
pub const FMR_KBD_OASYS_TYPE: u32 = 1;
pub const FMV_KBD_OASYS_TYPE: u32 = 2;
pub type GET_MOUSE_MOVE_POINTS_EX_RESOLUTION = u32;
pub const GMMP_USE_DISPLAY_POINTS: GET_MOUSE_MOVE_POINTS_EX_RESOLUTION = 1;
pub const GMMP_USE_HIGH_RESOLUTION_POINTS: GET_MOUSE_MOVE_POINTS_EX_RESOLUTION = 2;
pub const GRAVE: u32 = 768;
pub const GRPSELTAP: u32 = 128;
pub const HACEK: u32 = 780;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HARDWAREINPUT {
    pub uMsg: u32,
    pub wParamL: u16,
    pub wParamH: u16,
}
pub type HKL = *mut core::ffi::c_void;
pub const HOOK_ABOVE: u32 = 777;
pub type HOT_KEY_MODIFIERS = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INPUT {
    pub r#type: INPUT_TYPE,
    pub Anonymous: INPUT_0,
}
impl Default for INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INPUT_0 {
    pub mi: MOUSEINPUT,
    pub ki: KEYBDINPUT,
    pub hi: HARDWAREINPUT,
}
impl Default for INPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INPUT_HARDWARE: INPUT_TYPE = 2;
pub const INPUT_KEYBOARD: INPUT_TYPE = 1;
pub const INPUT_MOUSE: INPUT_TYPE = 0;
pub type INPUT_TYPE = u32;
pub const KANALOK: u32 = 8;
pub const KBDALT: u32 = 4;
pub const KBDBASE: u32 = 0;
pub const KBDCTRL: u32 = 2;
pub const KBDGRPSELTAP: u32 = 128;
pub const KBDKANA: u32 = 8;
pub const KBDLOYA: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KBDNLSTABLES {
    pub OEMIdentifier: u16,
    pub LayoutInformation: u16,
    pub NumOfVkToF: u32,
    pub pVkToF: *mut VK_F,
    pub NumOfMouseVKey: i32,
    pub pusMouseVKey: *mut u16,
}
impl Default for KBDNLSTABLES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KBDNLS_ALPHANUM: u32 = 5;
pub const KBDNLS_CODEINPUT: u32 = 10;
pub const KBDNLS_CONV_OR_NONCONV: u32 = 15;
pub const KBDNLS_HELP_OR_END: u32 = 11;
pub const KBDNLS_HIRAGANA: u32 = 6;
pub const KBDNLS_HOME_OR_CLEAR: u32 = 12;
pub const KBDNLS_INDEX_ALT: u32 = 2;
pub const KBDNLS_INDEX_NORMAL: u32 = 1;
pub const KBDNLS_KANAEVENT: u32 = 14;
pub const KBDNLS_KANALOCK: u32 = 4;
pub const KBDNLS_KATAKANA: u32 = 7;
pub const KBDNLS_NOEVENT: u32 = 1;
pub const KBDNLS_NULL: u32 = 0;
pub const KBDNLS_NUMPAD: u32 = 13;
pub const KBDNLS_ROMAN: u32 = 9;
pub const KBDNLS_SBCSDBCS: u32 = 8;
pub const KBDNLS_SEND_BASE_VK: u32 = 2;
pub const KBDNLS_SEND_PARAM_VK: u32 = 3;
pub const KBDNLS_TYPE_NORMAL: u32 = 1;
pub const KBDNLS_TYPE_NULL: u32 = 0;
pub const KBDNLS_TYPE_TOGGLE: u32 = 2;
pub const KBDROYA: u32 = 16;
pub const KBDSHIFT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KBDTABLES {
    pub pCharModifiers: *mut MODIFIERS,
    pub pVkToWcharTable: *mut VK_TO_WCHAR_TABLE,
    pub pDeadKey: *mut DEADKEY,
    pub pKeyNames: *mut VSC_LPWSTR,
    pub pKeyNamesExt: *mut VSC_LPWSTR,
    pub pKeyNamesDead: *mut *mut u16,
    pub pusVSCtoVK: *mut u16,
    pub bMaxVSCtoVK: u8,
    pub pVSCtoVK_E0: *mut VSC_VK,
    pub pVSCtoVK_E1: *mut VSC_VK,
    pub fLocaleFlags: u32,
    pub nLgMax: u8,
    pub cbLgEntry: u8,
    pub pLigature: *mut LIGATURE1,
    pub dwType: u32,
    pub dwSubType: u32,
}
impl Default for KBDTABLES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KBDTABLE_DESC {
    pub wszDllName: [u16; 32],
    pub dwType: u32,
    pub dwSubType: u32,
}
impl Default for KBDTABLE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KBDTABLE_MULTI {
    pub nTables: u32,
    pub aKbdTables: [KBDTABLE_DESC; 8],
}
impl Default for KBDTABLE_MULTI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KBDTABLE_MULTI_MAX: u32 = 8;
pub const KBD_TYPE: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KBD_TYPE_INFO {
    pub dwVersion: u32,
    pub dwType: u32,
    pub dwSubType: u32,
}
pub const KBD_VERSION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KEYBDINPUT {
    pub wVk: VIRTUAL_KEY,
    pub wScan: u16,
    pub dwFlags: KEYBD_EVENT_FLAGS,
    pub time: u32,
    pub dwExtraInfo: usize,
}
pub type KEYBD_EVENT_FLAGS = u32;
pub const KEYBOARD_TYPE_GENERIC_101: u32 = 4;
pub const KEYBOARD_TYPE_JAPAN: u32 = 7;
pub const KEYBOARD_TYPE_KOREA: u32 = 8;
pub const KEYBOARD_TYPE_UNKNOWN: u32 = 81;
pub const KEYEVENTF_EXTENDEDKEY: KEYBD_EVENT_FLAGS = 1;
pub const KEYEVENTF_KEYUP: KEYBD_EVENT_FLAGS = 2;
pub const KEYEVENTF_SCANCODE: KEYBD_EVENT_FLAGS = 8;
pub const KEYEVENTF_UNICODE: KEYBD_EVENT_FLAGS = 4;
pub const KLF_ACTIVATE: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = 1;
pub const KLF_NOTELLSHELL: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = 128;
pub const KLF_REORDER: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = 8;
pub const KLF_REPLACELANG: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = 16;
pub const KLF_RESET: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = 1073741824;
pub const KLF_SETFORPROCESS: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = 256;
pub const KLF_SHIFTLOCK: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = 65536;
pub const KLF_SUBSTITUTE_OK: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = 2;
pub const KLLF_ALTGR: u32 = 1;
pub const KLLF_GLOBAL_ATTRS: u32 = 2;
pub const KLLF_LRM_RLM: u32 = 4;
pub const KLLF_SHIFTLOCK: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LASTINPUTINFO {
    pub cbSize: u32,
    pub dwTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LIGATURE1 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 1],
}
impl Default for LIGATURE1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LIGATURE2 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 2],
}
impl Default for LIGATURE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LIGATURE3 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 3],
}
impl Default for LIGATURE3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LIGATURE4 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 4],
}
impl Default for LIGATURE4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LIGATURE5 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 5],
}
impl Default for LIGATURE5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MACRON: u32 = 772;
pub const MAPVK_VK_TO_CHAR: MAP_VIRTUAL_KEY_TYPE = 2;
pub const MAPVK_VK_TO_VSC: MAP_VIRTUAL_KEY_TYPE = 0;
pub const MAPVK_VK_TO_VSC_EX: MAP_VIRTUAL_KEY_TYPE = 4;
pub const MAPVK_VSC_TO_VK: MAP_VIRTUAL_KEY_TYPE = 1;
pub const MAPVK_VSC_TO_VK_EX: MAP_VIRTUAL_KEY_TYPE = 3;
pub type MAP_VIRTUAL_KEY_TYPE = u32;
pub const MICROSOFT_KBD_001_TYPE: u32 = 4;
pub const MICROSOFT_KBD_002_TYPE: u32 = 3;
pub const MICROSOFT_KBD_101A_TYPE: u32 = 0;
pub const MICROSOFT_KBD_101B_TYPE: u32 = 4;
pub const MICROSOFT_KBD_101C_TYPE: u32 = 5;
pub const MICROSOFT_KBD_101_TYPE: u32 = 0;
pub const MICROSOFT_KBD_103_TYPE: u32 = 6;
pub const MICROSOFT_KBD_106_TYPE: u32 = 2;
pub const MICROSOFT_KBD_AX_TYPE: u32 = 1;
pub const MICROSOFT_KBD_FUNC: u32 = 12;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MODIFIERS {
    pub pVkToBit: *mut VK_TO_BIT,
    pub wMaxModBits: u16,
    pub ModNumber: [u8; 1],
}
impl Default for MODIFIERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MOD_ALT: HOT_KEY_MODIFIERS = 1;
pub const MOD_CONTROL: HOT_KEY_MODIFIERS = 2;
pub const MOD_NOREPEAT: HOT_KEY_MODIFIERS = 16384;
pub const MOD_SHIFT: HOT_KEY_MODIFIERS = 4;
pub const MOD_WIN: HOT_KEY_MODIFIERS = 8;
pub const MOUSEEVENTF_ABSOLUTE: MOUSE_EVENT_FLAGS = 32768;
pub const MOUSEEVENTF_HWHEEL: MOUSE_EVENT_FLAGS = 4096;
pub const MOUSEEVENTF_LEFTDOWN: MOUSE_EVENT_FLAGS = 2;
pub const MOUSEEVENTF_LEFTUP: MOUSE_EVENT_FLAGS = 4;
pub const MOUSEEVENTF_MIDDLEDOWN: MOUSE_EVENT_FLAGS = 32;
pub const MOUSEEVENTF_MIDDLEUP: MOUSE_EVENT_FLAGS = 64;
pub const MOUSEEVENTF_MOVE: MOUSE_EVENT_FLAGS = 1;
pub const MOUSEEVENTF_MOVE_NOCOALESCE: MOUSE_EVENT_FLAGS = 8192;
pub const MOUSEEVENTF_RIGHTDOWN: MOUSE_EVENT_FLAGS = 8;
pub const MOUSEEVENTF_RIGHTUP: MOUSE_EVENT_FLAGS = 16;
pub const MOUSEEVENTF_VIRTUALDESK: MOUSE_EVENT_FLAGS = 16384;
pub const MOUSEEVENTF_WHEEL: MOUSE_EVENT_FLAGS = 2048;
pub const MOUSEEVENTF_XDOWN: MOUSE_EVENT_FLAGS = 128;
pub const MOUSEEVENTF_XUP: MOUSE_EVENT_FLAGS = 256;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MOUSEINPUT {
    pub dx: i32,
    pub dy: i32,
    pub mouseData: u32,
    pub dwFlags: MOUSE_EVENT_FLAGS,
    pub time: u32,
    pub dwExtraInfo: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MOUSEMOVEPOINT {
    pub x: i32,
    pub y: i32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
pub type MOUSE_EVENT_FLAGS = u32;
pub const NEC_KBD_106_TYPE: u32 = 5;
pub const NEC_KBD_H_MODE_TYPE: u32 = 3;
pub const NEC_KBD_LAPTOP_TYPE: u32 = 4;
pub const NEC_KBD_NORMAL_TYPE: u32 = 1;
pub const NEC_KBD_N_MODE_TYPE: u32 = 2;
pub const NLSKBD_INFO_ACCESSIBILITY_KEYMAP: u32 = 2;
pub const NLSKBD_INFO_EMURATE_101_KEYBOARD: u32 = 16;
pub const NLSKBD_INFO_EMURATE_106_KEYBOARD: u32 = 32;
pub const NLSKBD_INFO_SEND_IME_NOTIFICATION: u32 = 1;
pub const NLSKBD_OEM_AX: u32 = 1;
pub const NLSKBD_OEM_DEC: u32 = 24;
pub const NLSKBD_OEM_EPSON: u32 = 4;
pub const NLSKBD_OEM_FUJITSU: u32 = 5;
pub const NLSKBD_OEM_IBM: u32 = 7;
pub const NLSKBD_OEM_MATSUSHITA: u32 = 10;
pub const NLSKBD_OEM_MICROSOFT: u32 = 0;
pub const NLSKBD_OEM_NEC: u32 = 13;
pub const NLSKBD_OEM_TOSHIBA: u32 = 18;
pub const OGONEK: u32 = 808;
pub const OVERSCORE: u32 = 773;
pub const RING: u32 = 778;
pub const SCANCODE_ALT: u32 = 56;
pub const SCANCODE_CTRL: u32 = 29;
pub const SCANCODE_LSHIFT: u32 = 42;
pub const SCANCODE_LWIN: u32 = 91;
pub const SCANCODE_NUMPAD_FIRST: u32 = 71;
pub const SCANCODE_NUMPAD_LAST: u32 = 82;
pub const SCANCODE_RSHIFT: u32 = 54;
pub const SCANCODE_RWIN: u32 = 92;
pub const SCANCODE_THAI_LAYOUT_TOGGLE: u32 = 41;
pub const SGCAPS: u32 = 2;
pub const SHFT_INVALID: u32 = 15;
pub const TILDE: u32 = 771;
pub const TME_CANCEL: TRACKMOUSEEVENT_FLAGS = 2147483648;
pub const TME_HOVER: TRACKMOUSEEVENT_FLAGS = 1;
pub const TME_LEAVE: TRACKMOUSEEVENT_FLAGS = 2;
pub const TME_NONCLIENT: TRACKMOUSEEVENT_FLAGS = 16;
pub const TME_QUERY: TRACKMOUSEEVENT_FLAGS = 1073741824;
pub const TONOS: u32 = 900;
pub const TOSHIBA_KBD_DESKTOP_TYPE: u32 = 13;
pub const TOSHIBA_KBD_LAPTOP_TYPE: u32 = 15;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TRACKMOUSEEVENT {
    pub cbSize: u32,
    pub dwFlags: TRACKMOUSEEVENT_FLAGS,
    pub hwndTrack: super::super::super::Foundation::HWND,
    pub dwHoverTime: u32,
}
impl Default for TRACKMOUSEEVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TRACKMOUSEEVENT_FLAGS = u32;
pub const UMLAUT: u32 = 776;
pub type VIRTUAL_KEY = u16;
pub const VK_0: VIRTUAL_KEY = 48;
pub const VK_1: VIRTUAL_KEY = 49;
pub const VK_2: VIRTUAL_KEY = 50;
pub const VK_3: VIRTUAL_KEY = 51;
pub const VK_4: VIRTUAL_KEY = 52;
pub const VK_5: VIRTUAL_KEY = 53;
pub const VK_6: VIRTUAL_KEY = 54;
pub const VK_7: VIRTUAL_KEY = 55;
pub const VK_8: VIRTUAL_KEY = 56;
pub const VK_9: VIRTUAL_KEY = 57;
pub const VK_A: VIRTUAL_KEY = 65;
pub const VK_ABNT_C1: VIRTUAL_KEY = 193;
pub const VK_ABNT_C2: VIRTUAL_KEY = 194;
pub const VK_ACCEPT: VIRTUAL_KEY = 30;
pub const VK_ADD: VIRTUAL_KEY = 107;
pub const VK_APPS: VIRTUAL_KEY = 93;
pub const VK_ATTN: VIRTUAL_KEY = 246;
pub const VK_B: VIRTUAL_KEY = 66;
pub const VK_BACK: VIRTUAL_KEY = 8;
pub const VK_BROWSER_BACK: VIRTUAL_KEY = 166;
pub const VK_BROWSER_FAVORITES: VIRTUAL_KEY = 171;
pub const VK_BROWSER_FORWARD: VIRTUAL_KEY = 167;
pub const VK_BROWSER_HOME: VIRTUAL_KEY = 172;
pub const VK_BROWSER_REFRESH: VIRTUAL_KEY = 168;
pub const VK_BROWSER_SEARCH: VIRTUAL_KEY = 170;
pub const VK_BROWSER_STOP: VIRTUAL_KEY = 169;
pub const VK_C: VIRTUAL_KEY = 67;
pub const VK_CANCEL: VIRTUAL_KEY = 3;
pub const VK_CAPITAL: VIRTUAL_KEY = 20;
pub const VK_CLEAR: VIRTUAL_KEY = 12;
pub const VK_CONTROL: VIRTUAL_KEY = 17;
pub const VK_CONVERT: VIRTUAL_KEY = 28;
pub const VK_CRSEL: VIRTUAL_KEY = 247;
pub const VK_D: VIRTUAL_KEY = 68;
pub const VK_DBE_ALPHANUMERIC: VIRTUAL_KEY = 240;
pub const VK_DBE_CODEINPUT: VIRTUAL_KEY = 250;
pub const VK_DBE_DBCSCHAR: VIRTUAL_KEY = 244;
pub const VK_DBE_DETERMINESTRING: VIRTUAL_KEY = 252;
pub const VK_DBE_ENTERDLGCONVERSIONMODE: VIRTUAL_KEY = 253;
pub const VK_DBE_ENTERIMECONFIGMODE: VIRTUAL_KEY = 248;
pub const VK_DBE_ENTERWORDREGISTERMODE: VIRTUAL_KEY = 247;
pub const VK_DBE_FLUSHSTRING: VIRTUAL_KEY = 249;
pub const VK_DBE_HIRAGANA: VIRTUAL_KEY = 242;
pub const VK_DBE_KATAKANA: VIRTUAL_KEY = 241;
pub const VK_DBE_NOCODEINPUT: VIRTUAL_KEY = 251;
pub const VK_DBE_NOROMAN: VIRTUAL_KEY = 246;
pub const VK_DBE_ROMAN: VIRTUAL_KEY = 245;
pub const VK_DBE_SBCSCHAR: VIRTUAL_KEY = 243;
pub const VK_DECIMAL: VIRTUAL_KEY = 110;
pub const VK_DELETE: VIRTUAL_KEY = 46;
pub const VK_DIVIDE: VIRTUAL_KEY = 111;
pub const VK_DOWN: VIRTUAL_KEY = 40;
pub const VK_E: VIRTUAL_KEY = 69;
pub const VK_END: VIRTUAL_KEY = 35;
pub const VK_EREOF: VIRTUAL_KEY = 249;
pub const VK_ESCAPE: VIRTUAL_KEY = 27;
pub const VK_EXECUTE: VIRTUAL_KEY = 43;
pub const VK_EXSEL: VIRTUAL_KEY = 248;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VK_F {
    pub Vk: u8,
    pub NLSFEProcType: u8,
    pub NLSFEProcCurrent: u8,
    pub NLSFEProcSwitch: u8,
    pub NLSFEProc: [VK_FPARAM; 8],
    pub NLSFEProcAlt: [VK_FPARAM; 8],
}
impl Default for VK_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VK_F: VIRTUAL_KEY = 70;
pub const VK_F1: VIRTUAL_KEY = 112;
pub const VK_F10: VIRTUAL_KEY = 121;
pub const VK_F11: VIRTUAL_KEY = 122;
pub const VK_F12: VIRTUAL_KEY = 123;
pub const VK_F13: VIRTUAL_KEY = 124;
pub const VK_F14: VIRTUAL_KEY = 125;
pub const VK_F15: VIRTUAL_KEY = 126;
pub const VK_F16: VIRTUAL_KEY = 127;
pub const VK_F17: VIRTUAL_KEY = 128;
pub const VK_F18: VIRTUAL_KEY = 129;
pub const VK_F19: VIRTUAL_KEY = 130;
pub const VK_F2: VIRTUAL_KEY = 113;
pub const VK_F20: VIRTUAL_KEY = 131;
pub const VK_F21: VIRTUAL_KEY = 132;
pub const VK_F22: VIRTUAL_KEY = 133;
pub const VK_F23: VIRTUAL_KEY = 134;
pub const VK_F24: VIRTUAL_KEY = 135;
pub const VK_F3: VIRTUAL_KEY = 114;
pub const VK_F4: VIRTUAL_KEY = 115;
pub const VK_F5: VIRTUAL_KEY = 116;
pub const VK_F6: VIRTUAL_KEY = 117;
pub const VK_F7: VIRTUAL_KEY = 118;
pub const VK_F8: VIRTUAL_KEY = 119;
pub const VK_F9: VIRTUAL_KEY = 120;
pub const VK_FINAL: VIRTUAL_KEY = 24;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VK_FPARAM {
    pub NLSFEProcIndex: u8,
    pub NLSFEProcParam: u32,
}
pub const VK_G: VIRTUAL_KEY = 71;
pub const VK_GAMEPAD_A: VIRTUAL_KEY = 195;
pub const VK_GAMEPAD_B: VIRTUAL_KEY = 196;
pub const VK_GAMEPAD_DPAD_DOWN: VIRTUAL_KEY = 204;
pub const VK_GAMEPAD_DPAD_LEFT: VIRTUAL_KEY = 205;
pub const VK_GAMEPAD_DPAD_RIGHT: VIRTUAL_KEY = 206;
pub const VK_GAMEPAD_DPAD_UP: VIRTUAL_KEY = 203;
pub const VK_GAMEPAD_LEFT_SHOULDER: VIRTUAL_KEY = 200;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON: VIRTUAL_KEY = 209;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_DOWN: VIRTUAL_KEY = 212;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_LEFT: VIRTUAL_KEY = 214;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT: VIRTUAL_KEY = 213;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_UP: VIRTUAL_KEY = 211;
pub const VK_GAMEPAD_LEFT_TRIGGER: VIRTUAL_KEY = 201;
pub const VK_GAMEPAD_MENU: VIRTUAL_KEY = 207;
pub const VK_GAMEPAD_RIGHT_SHOULDER: VIRTUAL_KEY = 199;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: VIRTUAL_KEY = 210;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN: VIRTUAL_KEY = 216;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT: VIRTUAL_KEY = 218;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: VIRTUAL_KEY = 217;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_UP: VIRTUAL_KEY = 215;
pub const VK_GAMEPAD_RIGHT_TRIGGER: VIRTUAL_KEY = 202;
pub const VK_GAMEPAD_VIEW: VIRTUAL_KEY = 208;
pub const VK_GAMEPAD_X: VIRTUAL_KEY = 197;
pub const VK_GAMEPAD_Y: VIRTUAL_KEY = 198;
pub const VK_H: VIRTUAL_KEY = 72;
pub const VK_HANGEUL: VIRTUAL_KEY = 21;
pub const VK_HANGUL: VIRTUAL_KEY = 21;
pub const VK_HANJA: VIRTUAL_KEY = 25;
pub const VK_HELP: VIRTUAL_KEY = 47;
pub const VK_HOME: VIRTUAL_KEY = 36;
pub const VK_I: VIRTUAL_KEY = 73;
pub const VK_ICO_00: VIRTUAL_KEY = 228;
pub const VK_ICO_CLEAR: VIRTUAL_KEY = 230;
pub const VK_ICO_HELP: VIRTUAL_KEY = 227;
pub const VK_IME_OFF: VIRTUAL_KEY = 26;
pub const VK_IME_ON: VIRTUAL_KEY = 22;
pub const VK_INSERT: VIRTUAL_KEY = 45;
pub const VK_J: VIRTUAL_KEY = 74;
pub const VK_JUNJA: VIRTUAL_KEY = 23;
pub const VK_K: VIRTUAL_KEY = 75;
pub const VK_KANA: VIRTUAL_KEY = 21;
pub const VK_KANJI: VIRTUAL_KEY = 25;
pub const VK_L: VIRTUAL_KEY = 76;
pub const VK_LAUNCH_APP1: VIRTUAL_KEY = 182;
pub const VK_LAUNCH_APP2: VIRTUAL_KEY = 183;
pub const VK_LAUNCH_MAIL: VIRTUAL_KEY = 180;
pub const VK_LAUNCH_MEDIA_SELECT: VIRTUAL_KEY = 181;
pub const VK_LBUTTON: VIRTUAL_KEY = 1;
pub const VK_LCONTROL: VIRTUAL_KEY = 162;
pub const VK_LEFT: VIRTUAL_KEY = 37;
pub const VK_LMENU: VIRTUAL_KEY = 164;
pub const VK_LSHIFT: VIRTUAL_KEY = 160;
pub const VK_LWIN: VIRTUAL_KEY = 91;
pub const VK_M: VIRTUAL_KEY = 77;
pub const VK_MBUTTON: VIRTUAL_KEY = 4;
pub const VK_MEDIA_NEXT_TRACK: VIRTUAL_KEY = 176;
pub const VK_MEDIA_PLAY_PAUSE: VIRTUAL_KEY = 179;
pub const VK_MEDIA_PREV_TRACK: VIRTUAL_KEY = 177;
pub const VK_MEDIA_STOP: VIRTUAL_KEY = 178;
pub const VK_MENU: VIRTUAL_KEY = 18;
pub const VK_MODECHANGE: VIRTUAL_KEY = 31;
pub const VK_MULTIPLY: VIRTUAL_KEY = 106;
pub const VK_N: VIRTUAL_KEY = 78;
pub const VK_NAVIGATION_ACCEPT: VIRTUAL_KEY = 142;
pub const VK_NAVIGATION_CANCEL: VIRTUAL_KEY = 143;
pub const VK_NAVIGATION_DOWN: VIRTUAL_KEY = 139;
pub const VK_NAVIGATION_LEFT: VIRTUAL_KEY = 140;
pub const VK_NAVIGATION_MENU: VIRTUAL_KEY = 137;
pub const VK_NAVIGATION_RIGHT: VIRTUAL_KEY = 141;
pub const VK_NAVIGATION_UP: VIRTUAL_KEY = 138;
pub const VK_NAVIGATION_VIEW: VIRTUAL_KEY = 136;
pub const VK_NEXT: VIRTUAL_KEY = 34;
pub const VK_NONAME: VIRTUAL_KEY = 252;
pub const VK_NONCONVERT: VIRTUAL_KEY = 29;
pub const VK_NUMLOCK: VIRTUAL_KEY = 144;
pub const VK_NUMPAD0: VIRTUAL_KEY = 96;
pub const VK_NUMPAD1: VIRTUAL_KEY = 97;
pub const VK_NUMPAD2: VIRTUAL_KEY = 98;
pub const VK_NUMPAD3: VIRTUAL_KEY = 99;
pub const VK_NUMPAD4: VIRTUAL_KEY = 100;
pub const VK_NUMPAD5: VIRTUAL_KEY = 101;
pub const VK_NUMPAD6: VIRTUAL_KEY = 102;
pub const VK_NUMPAD7: VIRTUAL_KEY = 103;
pub const VK_NUMPAD8: VIRTUAL_KEY = 104;
pub const VK_NUMPAD9: VIRTUAL_KEY = 105;
pub const VK_O: VIRTUAL_KEY = 79;
pub const VK_OEM_1: VIRTUAL_KEY = 186;
pub const VK_OEM_102: VIRTUAL_KEY = 226;
pub const VK_OEM_2: VIRTUAL_KEY = 191;
pub const VK_OEM_3: VIRTUAL_KEY = 192;
pub const VK_OEM_4: VIRTUAL_KEY = 219;
pub const VK_OEM_5: VIRTUAL_KEY = 220;
pub const VK_OEM_6: VIRTUAL_KEY = 221;
pub const VK_OEM_7: VIRTUAL_KEY = 222;
pub const VK_OEM_8: VIRTUAL_KEY = 223;
pub const VK_OEM_ATTN: VIRTUAL_KEY = 240;
pub const VK_OEM_AUTO: VIRTUAL_KEY = 243;
pub const VK_OEM_AX: VIRTUAL_KEY = 225;
pub const VK_OEM_BACKTAB: VIRTUAL_KEY = 245;
pub const VK_OEM_CLEAR: VIRTUAL_KEY = 254;
pub const VK_OEM_COMMA: VIRTUAL_KEY = 188;
pub const VK_OEM_COPY: VIRTUAL_KEY = 242;
pub const VK_OEM_CUSEL: VIRTUAL_KEY = 239;
pub const VK_OEM_ENLW: VIRTUAL_KEY = 244;
pub const VK_OEM_FINISH: VIRTUAL_KEY = 241;
pub const VK_OEM_FJ_JISHO: VIRTUAL_KEY = 146;
pub const VK_OEM_FJ_LOYA: VIRTUAL_KEY = 149;
pub const VK_OEM_FJ_MASSHOU: VIRTUAL_KEY = 147;
pub const VK_OEM_FJ_ROYA: VIRTUAL_KEY = 150;
pub const VK_OEM_FJ_TOUROKU: VIRTUAL_KEY = 148;
pub const VK_OEM_JUMP: VIRTUAL_KEY = 234;
pub const VK_OEM_MINUS: VIRTUAL_KEY = 189;
pub const VK_OEM_NEC_EQUAL: VIRTUAL_KEY = 146;
pub const VK_OEM_PA1: VIRTUAL_KEY = 235;
pub const VK_OEM_PA2: VIRTUAL_KEY = 236;
pub const VK_OEM_PA3: VIRTUAL_KEY = 237;
pub const VK_OEM_PERIOD: VIRTUAL_KEY = 190;
pub const VK_OEM_PLUS: VIRTUAL_KEY = 187;
pub const VK_OEM_RESET: VIRTUAL_KEY = 233;
pub const VK_OEM_WSCTRL: VIRTUAL_KEY = 238;
pub const VK_P: VIRTUAL_KEY = 80;
pub const VK_PA1: VIRTUAL_KEY = 253;
pub const VK_PACKET: VIRTUAL_KEY = 231;
pub const VK_PAUSE: VIRTUAL_KEY = 19;
pub const VK_PLAY: VIRTUAL_KEY = 250;
pub const VK_PRINT: VIRTUAL_KEY = 42;
pub const VK_PRIOR: VIRTUAL_KEY = 33;
pub const VK_PROCESSKEY: VIRTUAL_KEY = 229;
pub const VK_Q: VIRTUAL_KEY = 81;
pub const VK_R: VIRTUAL_KEY = 82;
pub const VK_RBUTTON: VIRTUAL_KEY = 2;
pub const VK_RCONTROL: VIRTUAL_KEY = 163;
pub const VK_RETURN: VIRTUAL_KEY = 13;
pub const VK_RIGHT: VIRTUAL_KEY = 39;
pub const VK_RMENU: VIRTUAL_KEY = 165;
pub const VK_RSHIFT: VIRTUAL_KEY = 161;
pub const VK_RWIN: VIRTUAL_KEY = 92;
pub const VK_S: VIRTUAL_KEY = 83;
pub const VK_SCROLL: VIRTUAL_KEY = 145;
pub const VK_SELECT: VIRTUAL_KEY = 41;
pub const VK_SEPARATOR: VIRTUAL_KEY = 108;
pub const VK_SHIFT: VIRTUAL_KEY = 16;
pub const VK_SLEEP: VIRTUAL_KEY = 95;
pub const VK_SNAPSHOT: VIRTUAL_KEY = 44;
pub const VK_SPACE: VIRTUAL_KEY = 32;
pub const VK_SUBTRACT: VIRTUAL_KEY = 109;
pub const VK_T: VIRTUAL_KEY = 84;
pub const VK_TAB: VIRTUAL_KEY = 9;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VK_TO_BIT {
    pub Vk: u8,
    pub ModBits: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VK_TO_WCHARS1 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 1],
}
impl Default for VK_TO_WCHARS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VK_TO_WCHARS10 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 10],
}
impl Default for VK_TO_WCHARS10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VK_TO_WCHARS2 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 2],
}
impl Default for VK_TO_WCHARS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VK_TO_WCHARS3 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 3],
}
impl Default for VK_TO_WCHARS3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VK_TO_WCHARS4 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 4],
}
impl Default for VK_TO_WCHARS4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VK_TO_WCHARS5 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 5],
}
impl Default for VK_TO_WCHARS5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VK_TO_WCHARS6 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 6],
}
impl Default for VK_TO_WCHARS6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VK_TO_WCHARS7 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 7],
}
impl Default for VK_TO_WCHARS7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VK_TO_WCHARS8 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 8],
}
impl Default for VK_TO_WCHARS8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VK_TO_WCHARS9 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 9],
}
impl Default for VK_TO_WCHARS9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VK_TO_WCHAR_TABLE {
    pub pVkToWchars: *mut VK_TO_WCHARS1,
    pub nModifications: u8,
    pub cbSize: u8,
}
impl Default for VK_TO_WCHAR_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VK_U: VIRTUAL_KEY = 85;
pub const VK_UP: VIRTUAL_KEY = 38;
pub const VK_V: VIRTUAL_KEY = 86;
pub const VK_VOLUME_DOWN: VIRTUAL_KEY = 174;
pub const VK_VOLUME_MUTE: VIRTUAL_KEY = 173;
pub const VK_VOLUME_UP: VIRTUAL_KEY = 175;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VK_VSC {
    pub Vk: u8,
    pub Vsc: u8,
}
pub const VK_W: VIRTUAL_KEY = 87;
pub const VK_X: VIRTUAL_KEY = 88;
pub const VK_XBUTTON1: VIRTUAL_KEY = 5;
pub const VK_XBUTTON2: VIRTUAL_KEY = 6;
pub const VK_Y: VIRTUAL_KEY = 89;
pub const VK_Z: VIRTUAL_KEY = 90;
pub const VK_ZOOM: VIRTUAL_KEY = 251;
pub const VK__none_: VIRTUAL_KEY = 255;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VSC_LPWSTR {
    pub vsc: u8,
    pub pwsz: windows_sys::core::PWSTR,
}
impl Default for VSC_LPWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VSC_VK {
    pub Vsc: u8,
    pub Vk: u16,
}
pub const WCH_DEAD: u32 = 61441;
pub const WCH_LGTR: u32 = 61442;
pub const WCH_NONE: u32 = 61440;
pub const wszACUTE: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{301}");
pub const wszBREVE: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{306}");
pub const wszCEDILLA: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{327}");
pub const wszCIRCUMFLEX: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{302}");
pub const wszDIARESIS_TONOS: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{385}");
pub const wszDOT_ABOVE: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{307}");
pub const wszDOUBLE_ACUTE: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{30b}");
pub const wszGRAVE: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{300}");
pub const wszHACEK: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{30c}");
pub const wszHOOK_ABOVE: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{309}");
pub const wszMACRON: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{304}");
pub const wszOGONEK: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{328}");
pub const wszOVERSCORE: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{305}");
pub const wszRING: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{30a}");
pub const wszTILDE: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{303}");
pub const wszTONOS: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{384}");
pub const wszUMLAUT: windows_sys::core::PCWSTR = windows_sys::core::w!("\u{308}");
