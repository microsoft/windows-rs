#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn ActivateKeyboardLayout(hkl: super::super::TextServices::HKL, flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS) -> super::super::TextServices::HKL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BlockInput(fblockit: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragDetect(hwnd: super::super::super::Foundation::HWND, pt: super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableWindow(hwnd: super::super::super::Foundation::HWND, benable: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetActiveWindow() -> super::super::super::Foundation::HWND;
    pub fn GetAsyncKeyState(vkey: i32) -> i16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCapture() -> super::super::super::Foundation::HWND;
    pub fn GetDoubleClickTime() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFocus() -> super::super::super::Foundation::HWND;
    pub fn GetKBCodePage() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyNameTextA(lparam: i32, lpstring: super::super::super::Foundation::PSTR, cchsize: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyNameTextW(lparam: i32, lpstring: super::super::super::Foundation::PWSTR, cchsize: i32) -> i32;
    pub fn GetKeyState(nvirtkey: i32) -> i16;
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn GetKeyboardLayout(idthread: u32) -> super::super::TextServices::HKL;
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn GetKeyboardLayoutList(nbuff: i32, lplist: *mut super::super::TextServices::HKL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyboardLayoutNameA(pwszklid: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyboardLayoutNameW(pwszklid: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyboardState(lpkeystate: *mut u8) -> super::super::super::Foundation::BOOL;
    pub fn GetKeyboardType(ntypeflag: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLastInputInfo(plii: *mut LASTINPUTINFO) -> super::super::super::Foundation::BOOL;
    pub fn GetMouseMovePointsEx(cbsize: u32, lppt: *const MOUSEMOVEPOINT, lpptbuf: *mut MOUSEMOVEPOINT, nbufpoints: i32, resolution: GET_MOUSE_MOVE_POINTS_EX_RESOLUTION) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWindowEnabled(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn LoadKeyboardLayoutA(pwszklid: super::super::super::Foundation::PSTR, flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS) -> super::super::TextServices::HKL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn LoadKeyboardLayoutW(pwszklid: super::super::super::Foundation::PWSTR, flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS) -> super::super::TextServices::HKL;
    pub fn MapVirtualKeyA(ucode: u32, umaptype: u32) -> u32;
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn MapVirtualKeyExA(ucode: u32, umaptype: u32, dwhkl: super::super::TextServices::HKL) -> u32;
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn MapVirtualKeyExW(ucode: u32, umaptype: u32, dwhkl: super::super::TextServices::HKL) -> u32;
    pub fn MapVirtualKeyW(ucode: u32, umaptype: u32) -> u32;
    pub fn OemKeyScan(woemchar: u16) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterHotKey(hwnd: super::super::super::Foundation::HWND, id: i32, fsmodifiers: HOT_KEY_MODIFIERS, vk: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseCapture() -> super::super::super::Foundation::BOOL;
    pub fn SendInput(cinputs: u32, pinputs: *const INPUT, cbsize: i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetActiveWindow(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCapture(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDoubleClickTime(param0: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFocus(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetKeyboardState(lpkeystate: *const u8) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SwapMouseButton(fswap: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    pub fn ToAscii(uvirtkey: u32, uscancode: u32, lpkeystate: *const u8, lpchar: *mut u16, uflags: u32) -> i32;
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn ToAsciiEx(uvirtkey: u32, uscancode: u32, lpkeystate: *const u8, lpchar: *mut u16, uflags: u32, dwhkl: super::super::TextServices::HKL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ToUnicode(wvirtkey: u32, wscancode: u32, lpkeystate: *const u8, pwszbuff: super::super::super::Foundation::PWSTR, cchbuff: i32, wflags: u32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ToUnicodeEx(wvirtkey: u32, wscancode: u32, lpkeystate: *const u8, pwszbuff: super::super::super::Foundation::PWSTR, cchbuff: i32, wflags: u32, dwhkl: super::super::TextServices::HKL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TrackMouseEvent(lpeventtrack: *mut TRACKMOUSEEVENT) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn UnloadKeyboardLayout(hkl: super::super::TextServices::HKL) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterHotKey(hwnd: super::super::super::Foundation::HWND, id: i32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VkKeyScanA(ch: super::super::super::Foundation::CHAR) -> i16;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn VkKeyScanExA(ch: super::super::super::Foundation::CHAR, dwhkl: super::super::TextServices::HKL) -> i16;
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn VkKeyScanExW(ch: u16, dwhkl: super::super::TextServices::HKL) -> i16;
    pub fn VkKeyScanW(ch: u16) -> i16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn _TrackMouseEvent(lpeventtrack: *mut TRACKMOUSEEVENT) -> super::super::super::Foundation::BOOL;
    pub fn keybd_event(bvk: u8, bscan: u8, dwflags: KEYBD_EVENT_FLAGS, dwextrainfo: usize);
    pub fn mouse_event(dwflags: MOUSE_EVENT_FLAGS, dx: i32, dy: i32, dwdata: u32, dwextrainfo: usize);
}
pub const KLF_REORDER: u32 = 8u32;
pub const KLF_RESET: u32 = 1073741824u32;
pub const KLF_SETFORPROCESS: u32 = 256u32;
pub const KLF_SHIFTLOCK: u32 = 65536u32;
pub const KLF_ACTIVATE: u32 = 1u32;
pub const KLF_NOTELLSHELL: u32 = 128u32;
pub const KLF_REPLACELANG: u32 = 16u32;
pub const KLF_SUBSTITUTE_OK: u32 = 2u32;
pub const ACUTE: u32 = 769u32;
pub const AX_KBD_DESKTOP_TYPE: u32 = 1u32;
pub const BREVE: u32 = 774u32;
pub const CAPLOK: u32 = 1u32;
pub const CAPLOKALTGR: u32 = 4u32;
pub const CEDILLA: u32 = 807u32;
pub const CIRCUMFLEX: u32 = 770u32;
#[repr(C)]
pub struct DEADKEY {
    pub dwBoth: u32,
    pub wchComposed: u16,
    pub uFlags: u16,
}
impl ::core::marker::Copy for DEADKEY {}
impl ::core::clone::Clone for DEADKEY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DEC_KBD_ANSI_LAYOUT_TYPE: u32 = 1u32;
pub const DEC_KBD_JIS_LAYOUT_TYPE: u32 = 2u32;
pub const DIARESIS: u32 = 776u32;
pub const DIARESIS_TONOS: u32 = 901u32;
pub const DKF_DEAD: u32 = 1u32;
pub const DONTCARE_BIT: u32 = 33554432u32;
pub const DOT_ABOVE: u32 = 775u32;
pub const DOUBLE_ACUTE: u32 = 779u32;
pub const EXTENDED_BIT: u32 = 16777216u32;
pub const FAKE_KEYSTROKE: u32 = 33554432u32;
pub const FMR_KBD_JIS_TYPE: u32 = 0u32;
pub const FMR_KBD_OASYS_TYPE: u32 = 1u32;
pub const FMV_KBD_OASYS_TYPE: u32 = 2u32;
pub const GMMP_USE_DISPLAY_POINTS: u32 = 1u32;
pub const GMMP_USE_HIGH_RESOLUTION_POINTS: u32 = 2u32;
pub const GRAVE: u32 = 768u32;
pub const GRPSELTAP: u32 = 128u32;
pub const HACEK: u32 = 780u32;
#[repr(C)]
pub struct HARDWAREINPUT {
    pub uMsg: u32,
    pub wParamL: u16,
    pub wParamH: u16,
}
impl ::core::marker::Copy for HARDWAREINPUT {}
impl ::core::clone::Clone for HARDWAREINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HOOK_ABOVE: u32 = 777u32;
pub const MOD_ALT: u32 = 1u32;
pub const MOD_CONTROL: u32 = 2u32;
pub const MOD_NOREPEAT: u32 = 16384u32;
pub const MOD_SHIFT: u32 = 4u32;
pub const MOD_WIN: u32 = 8u32;
#[repr(C)]
pub struct INPUT {
    pub r#type: INPUT_TYPE,
    pub Anonymous: INPUT_0,
}
impl ::core::marker::Copy for INPUT {}
impl ::core::clone::Clone for INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union INPUT_0 {
    pub mi: MOUSEINPUT,
    pub ki: KEYBDINPUT,
    pub hi: HARDWAREINPUT,
}
impl ::core::marker::Copy for INPUT_0 {}
impl ::core::clone::Clone for INPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const INPUT_MOUSE: u32 = 0u32;
pub const INPUT_KEYBOARD: u32 = 1u32;
pub const INPUT_HARDWARE: u32 = 2u32;
pub const KANALOK: u32 = 8u32;
pub const KBDALT: u32 = 4u32;
pub const KBDBASE: u32 = 0u32;
pub const KBDCTRL: u32 = 2u32;
pub const KBDGRPSELTAP: u32 = 128u32;
pub const KBDKANA: u32 = 8u32;
pub const KBDLOYA: u32 = 32u32;
pub const KBDNLS_ALPHANUM: u32 = 5u32;
pub const KBDNLS_CODEINPUT: u32 = 10u32;
pub const KBDNLS_CONV_OR_NONCONV: u32 = 15u32;
pub const KBDNLS_HELP_OR_END: u32 = 11u32;
pub const KBDNLS_HIRAGANA: u32 = 6u32;
pub const KBDNLS_HOME_OR_CLEAR: u32 = 12u32;
pub const KBDNLS_INDEX_ALT: u32 = 2u32;
pub const KBDNLS_INDEX_NORMAL: u32 = 1u32;
pub const KBDNLS_KANAEVENT: u32 = 14u32;
pub const KBDNLS_KANALOCK: u32 = 4u32;
pub const KBDNLS_KATAKANA: u32 = 7u32;
pub const KBDNLS_NOEVENT: u32 = 1u32;
pub const KBDNLS_NULL: u32 = 0u32;
pub const KBDNLS_NUMPAD: u32 = 13u32;
pub const KBDNLS_ROMAN: u32 = 9u32;
pub const KBDNLS_SBCSDBCS: u32 = 8u32;
pub const KBDNLS_SEND_BASE_VK: u32 = 2u32;
pub const KBDNLS_SEND_PARAM_VK: u32 = 3u32;
pub const KBDNLS_TYPE_NORMAL: u32 = 1u32;
pub const KBDNLS_TYPE_NULL: u32 = 0u32;
pub const KBDNLS_TYPE_TOGGLE: u32 = 2u32;
pub const KBDROYA: u32 = 16u32;
pub const KBDSHIFT: u32 = 1u32;
#[repr(C)]
pub struct KBDTABLE_DESC {
    pub wszDllName: [u16; 32],
    pub dwType: u32,
    pub dwSubType: u32,
}
impl ::core::marker::Copy for KBDTABLE_DESC {}
impl ::core::clone::Clone for KBDTABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct KBDTABLE_MULTI {
    pub nTables: u32,
    pub aKbdTables: [KBDTABLE_DESC; 8],
}
impl ::core::marker::Copy for KBDTABLE_MULTI {}
impl ::core::clone::Clone for KBDTABLE_MULTI {
    fn clone(&self) -> Self {
        *self
    }
}
pub const KBDTABLE_MULTI_MAX: u32 = 8u32;
pub const KBD_TYPE: u32 = 4u32;
#[repr(C)]
pub struct KBD_TYPE_INFO {
    pub dwVersion: u32,
    pub dwType: u32,
    pub dwSubType: u32,
}
impl ::core::marker::Copy for KBD_TYPE_INFO {}
impl ::core::clone::Clone for KBD_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const KBD_VERSION: u32 = 1u32;
#[repr(C)]
pub struct KEYBDINPUT {
    pub wVk: VIRTUAL_KEY,
    pub wScan: u16,
    pub dwFlags: KEYBD_EVENT_FLAGS,
    pub time: u32,
    pub dwExtraInfo: usize,
}
impl ::core::marker::Copy for KEYBDINPUT {}
impl ::core::clone::Clone for KEYBDINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const KEYEVENTF_EXTENDEDKEY: u32 = 1u32;
pub const KEYEVENTF_KEYUP: u32 = 2u32;
pub const KEYEVENTF_SCANCODE: u32 = 8u32;
pub const KEYEVENTF_UNICODE: u32 = 4u32;
pub const KEYBOARD_TYPE_GENERIC_101: u32 = 4u32;
pub const KEYBOARD_TYPE_JAPAN: u32 = 7u32;
pub const KEYBOARD_TYPE_KOREA: u32 = 8u32;
pub const KEYBOARD_TYPE_UNKNOWN: u32 = 81u32;
pub const KLLF_ALTGR: u32 = 1u32;
pub const KLLF_GLOBAL_ATTRS: u32 = 2u32;
pub const KLLF_LRM_RLM: u32 = 4u32;
pub const KLLF_SHIFTLOCK: u32 = 2u32;
#[repr(C)]
pub struct LASTINPUTINFO {
    pub cbSize: u32,
    pub dwTime: u32,
}
impl ::core::marker::Copy for LASTINPUTINFO {}
impl ::core::clone::Clone for LASTINPUTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct LIGATURE1 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 1],
}
impl ::core::marker::Copy for LIGATURE1 {}
impl ::core::clone::Clone for LIGATURE1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct LIGATURE2 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 2],
}
impl ::core::marker::Copy for LIGATURE2 {}
impl ::core::clone::Clone for LIGATURE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct LIGATURE3 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 3],
}
impl ::core::marker::Copy for LIGATURE3 {}
impl ::core::clone::Clone for LIGATURE3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct LIGATURE4 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 4],
}
impl ::core::marker::Copy for LIGATURE4 {}
impl ::core::clone::Clone for LIGATURE4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct LIGATURE5 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 5],
}
impl ::core::marker::Copy for LIGATURE5 {}
impl ::core::clone::Clone for LIGATURE5 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MACRON: u32 = 772u32;
pub const MICROSOFT_KBD_001_TYPE: u32 = 4u32;
pub const MICROSOFT_KBD_002_TYPE: u32 = 3u32;
pub const MICROSOFT_KBD_101A_TYPE: u32 = 0u32;
pub const MICROSOFT_KBD_101B_TYPE: u32 = 4u32;
pub const MICROSOFT_KBD_101C_TYPE: u32 = 5u32;
pub const MICROSOFT_KBD_101_TYPE: u32 = 0u32;
pub const MICROSOFT_KBD_103_TYPE: u32 = 6u32;
pub const MICROSOFT_KBD_106_TYPE: u32 = 2u32;
pub const MICROSOFT_KBD_AX_TYPE: u32 = 1u32;
pub const MICROSOFT_KBD_FUNC: u32 = 12u32;
#[repr(C)]
pub struct MODIFIERS {
    pub pVkToBit: *mut VK_TO_BIT,
    pub wMaxModBits: u16,
    pub ModNumber: [u8; 1],
}
impl ::core::marker::Copy for MODIFIERS {}
impl ::core::clone::Clone for MODIFIERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MOUSEINPUT {
    pub dx: i32,
    pub dy: i32,
    pub mouseData: u32,
    pub dwFlags: MOUSE_EVENT_FLAGS,
    pub time: u32,
    pub dwExtraInfo: usize,
}
impl ::core::marker::Copy for MOUSEINPUT {}
impl ::core::clone::Clone for MOUSEINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MOUSEMOVEPOINT {
    pub x: i32,
    pub y: i32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
impl ::core::marker::Copy for MOUSEMOVEPOINT {}
impl ::core::clone::Clone for MOUSEMOVEPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MOUSEEVENTF_ABSOLUTE: u32 = 32768u32;
pub const MOUSEEVENTF_LEFTDOWN: u32 = 2u32;
pub const MOUSEEVENTF_LEFTUP: u32 = 4u32;
pub const MOUSEEVENTF_MIDDLEDOWN: u32 = 32u32;
pub const MOUSEEVENTF_MIDDLEUP: u32 = 64u32;
pub const MOUSEEVENTF_MOVE: u32 = 1u32;
pub const MOUSEEVENTF_RIGHTDOWN: u32 = 8u32;
pub const MOUSEEVENTF_RIGHTUP: u32 = 16u32;
pub const MOUSEEVENTF_WHEEL: u32 = 2048u32;
pub const MOUSEEVENTF_XDOWN: u32 = 128u32;
pub const MOUSEEVENTF_XUP: u32 = 256u32;
pub const MOUSEEVENTF_HWHEEL: u32 = 4096u32;
pub const MOUSEEVENTF_MOVE_NOCOALESCE: u32 = 8192u32;
pub const MOUSEEVENTF_VIRTUALDESK: u32 = 16384u32;
pub const NEC_KBD_106_TYPE: u32 = 5u32;
pub const NEC_KBD_H_MODE_TYPE: u32 = 3u32;
pub const NEC_KBD_LAPTOP_TYPE: u32 = 4u32;
pub const NEC_KBD_NORMAL_TYPE: u32 = 1u32;
pub const NEC_KBD_N_MODE_TYPE: u32 = 2u32;
pub const NLSKBD_INFO_ACCESSIBILITY_KEYMAP: u32 = 2u32;
pub const NLSKBD_INFO_EMURATE_101_KEYBOARD: u32 = 16u32;
pub const NLSKBD_INFO_EMURATE_106_KEYBOARD: u32 = 32u32;
pub const NLSKBD_INFO_SEND_IME_NOTIFICATION: u32 = 1u32;
pub const NLSKBD_OEM_AX: u32 = 1u32;
pub const NLSKBD_OEM_DEC: u32 = 24u32;
pub const NLSKBD_OEM_EPSON: u32 = 4u32;
pub const NLSKBD_OEM_FUJITSU: u32 = 5u32;
pub const NLSKBD_OEM_IBM: u32 = 7u32;
pub const NLSKBD_OEM_MATSUSHITA: u32 = 10u32;
pub const NLSKBD_OEM_MICROSOFT: u32 = 0u32;
pub const NLSKBD_OEM_NEC: u32 = 13u32;
pub const NLSKBD_OEM_TOSHIBA: u32 = 18u32;
pub const OGONEK: u32 = 808u32;
pub const OVERSCORE: u32 = 773u32;
pub const RING: u32 = 778u32;
pub const SCANCODE_ALT: u32 = 56u32;
pub const SCANCODE_CTRL: u32 = 29u32;
pub const SCANCODE_LSHIFT: u32 = 42u32;
pub const SCANCODE_LWIN: u32 = 91u32;
pub const SCANCODE_NUMPAD_FIRST: u32 = 71u32;
pub const SCANCODE_NUMPAD_LAST: u32 = 82u32;
pub const SCANCODE_RSHIFT: u32 = 54u32;
pub const SCANCODE_RWIN: u32 = 92u32;
pub const SCANCODE_THAI_LAYOUT_TOGGLE: u32 = 41u32;
pub const SGCAPS: u32 = 2u32;
pub const SHFT_INVALID: u32 = 15u32;
pub const TILDE: u32 = 771u32;
pub const TONOS: u32 = 900u32;
pub const TOSHIBA_KBD_DESKTOP_TYPE: u32 = 13u32;
pub const TOSHIBA_KBD_LAPTOP_TYPE: u32 = 15u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TRACKMOUSEEVENT {
    pub cbSize: u32,
    pub dwFlags: TRACKMOUSEEVENT_FLAGS,
    pub hwndTrack: super::super::super::Foundation::HWND,
    pub dwHoverTime: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRACKMOUSEEVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRACKMOUSEEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TME_CANCEL: u32 = 2147483648u32;
pub const TME_HOVER: u32 = 1u32;
pub const TME_LEAVE: u32 = 2u32;
pub const TME_NONCLIENT: u32 = 16u32;
pub const TME_QUERY: u32 = 1073741824u32;
pub const UMLAUT: u32 = 776u32;
pub const VK_0: u16 = 48u16;
pub const VK_1: u16 = 49u16;
pub const VK_2: u16 = 50u16;
pub const VK_3: u16 = 51u16;
pub const VK_4: u16 = 52u16;
pub const VK_5: u16 = 53u16;
pub const VK_6: u16 = 54u16;
pub const VK_7: u16 = 55u16;
pub const VK_8: u16 = 56u16;
pub const VK_9: u16 = 57u16;
pub const VK_A: u16 = 65u16;
pub const VK_B: u16 = 66u16;
pub const VK_C: u16 = 67u16;
pub const VK_D: u16 = 68u16;
pub const VK_E: u16 = 69u16;
pub const VK_F: u16 = 70u16;
pub const VK_G: u16 = 71u16;
pub const VK_H: u16 = 72u16;
pub const VK_I: u16 = 73u16;
pub const VK_J: u16 = 74u16;
pub const VK_K: u16 = 75u16;
pub const VK_L: u16 = 76u16;
pub const VK_M: u16 = 77u16;
pub const VK_N: u16 = 78u16;
pub const VK_O: u16 = 79u16;
pub const VK_P: u16 = 80u16;
pub const VK_Q: u16 = 81u16;
pub const VK_R: u16 = 82u16;
pub const VK_S: u16 = 83u16;
pub const VK_T: u16 = 84u16;
pub const VK_U: u16 = 85u16;
pub const VK_V: u16 = 86u16;
pub const VK_W: u16 = 87u16;
pub const VK_X: u16 = 88u16;
pub const VK_Y: u16 = 89u16;
pub const VK_Z: u16 = 90u16;
pub const VK_LBUTTON: u16 = 1u16;
pub const VK_RBUTTON: u16 = 2u16;
pub const VK_CANCEL: u16 = 3u16;
pub const VK_MBUTTON: u16 = 4u16;
pub const VK_XBUTTON1: u16 = 5u16;
pub const VK_XBUTTON2: u16 = 6u16;
pub const VK_BACK: u16 = 8u16;
pub const VK_TAB: u16 = 9u16;
pub const VK_CLEAR: u16 = 12u16;
pub const VK_RETURN: u16 = 13u16;
pub const VK_SHIFT: u16 = 16u16;
pub const VK_CONTROL: u16 = 17u16;
pub const VK_MENU: u16 = 18u16;
pub const VK_PAUSE: u16 = 19u16;
pub const VK_CAPITAL: u16 = 20u16;
pub const VK_KANA: u16 = 21u16;
pub const VK_HANGEUL: u16 = 21u16;
pub const VK_HANGUL: u16 = 21u16;
pub const VK_IME_ON: u16 = 22u16;
pub const VK_JUNJA: u16 = 23u16;
pub const VK_FINAL: u16 = 24u16;
pub const VK_HANJA: u16 = 25u16;
pub const VK_KANJI: u16 = 25u16;
pub const VK_IME_OFF: u16 = 26u16;
pub const VK_ESCAPE: u16 = 27u16;
pub const VK_CONVERT: u16 = 28u16;
pub const VK_NONCONVERT: u16 = 29u16;
pub const VK_ACCEPT: u16 = 30u16;
pub const VK_MODECHANGE: u16 = 31u16;
pub const VK_SPACE: u16 = 32u16;
pub const VK_PRIOR: u16 = 33u16;
pub const VK_NEXT: u16 = 34u16;
pub const VK_END: u16 = 35u16;
pub const VK_HOME: u16 = 36u16;
pub const VK_LEFT: u16 = 37u16;
pub const VK_UP: u16 = 38u16;
pub const VK_RIGHT: u16 = 39u16;
pub const VK_DOWN: u16 = 40u16;
pub const VK_SELECT: u16 = 41u16;
pub const VK_PRINT: u16 = 42u16;
pub const VK_EXECUTE: u16 = 43u16;
pub const VK_SNAPSHOT: u16 = 44u16;
pub const VK_INSERT: u16 = 45u16;
pub const VK_DELETE: u16 = 46u16;
pub const VK_HELP: u16 = 47u16;
pub const VK_LWIN: u16 = 91u16;
pub const VK_RWIN: u16 = 92u16;
pub const VK_APPS: u16 = 93u16;
pub const VK_SLEEP: u16 = 95u16;
pub const VK_NUMPAD0: u16 = 96u16;
pub const VK_NUMPAD1: u16 = 97u16;
pub const VK_NUMPAD2: u16 = 98u16;
pub const VK_NUMPAD3: u16 = 99u16;
pub const VK_NUMPAD4: u16 = 100u16;
pub const VK_NUMPAD5: u16 = 101u16;
pub const VK_NUMPAD6: u16 = 102u16;
pub const VK_NUMPAD7: u16 = 103u16;
pub const VK_NUMPAD8: u16 = 104u16;
pub const VK_NUMPAD9: u16 = 105u16;
pub const VK_MULTIPLY: u16 = 106u16;
pub const VK_ADD: u16 = 107u16;
pub const VK_SEPARATOR: u16 = 108u16;
pub const VK_SUBTRACT: u16 = 109u16;
pub const VK_DECIMAL: u16 = 110u16;
pub const VK_DIVIDE: u16 = 111u16;
pub const VK_F1: u16 = 112u16;
pub const VK_F2: u16 = 113u16;
pub const VK_F3: u16 = 114u16;
pub const VK_F4: u16 = 115u16;
pub const VK_F5: u16 = 116u16;
pub const VK_F6: u16 = 117u16;
pub const VK_F7: u16 = 118u16;
pub const VK_F8: u16 = 119u16;
pub const VK_F9: u16 = 120u16;
pub const VK_F10: u16 = 121u16;
pub const VK_F11: u16 = 122u16;
pub const VK_F12: u16 = 123u16;
pub const VK_F13: u16 = 124u16;
pub const VK_F14: u16 = 125u16;
pub const VK_F15: u16 = 126u16;
pub const VK_F16: u16 = 127u16;
pub const VK_F17: u16 = 128u16;
pub const VK_F18: u16 = 129u16;
pub const VK_F19: u16 = 130u16;
pub const VK_F20: u16 = 131u16;
pub const VK_F21: u16 = 132u16;
pub const VK_F22: u16 = 133u16;
pub const VK_F23: u16 = 134u16;
pub const VK_F24: u16 = 135u16;
pub const VK_NAVIGATION_VIEW: u16 = 136u16;
pub const VK_NAVIGATION_MENU: u16 = 137u16;
pub const VK_NAVIGATION_UP: u16 = 138u16;
pub const VK_NAVIGATION_DOWN: u16 = 139u16;
pub const VK_NAVIGATION_LEFT: u16 = 140u16;
pub const VK_NAVIGATION_RIGHT: u16 = 141u16;
pub const VK_NAVIGATION_ACCEPT: u16 = 142u16;
pub const VK_NAVIGATION_CANCEL: u16 = 143u16;
pub const VK_NUMLOCK: u16 = 144u16;
pub const VK_SCROLL: u16 = 145u16;
pub const VK_OEM_NEC_EQUAL: u16 = 146u16;
pub const VK_OEM_FJ_JISHO: u16 = 146u16;
pub const VK_OEM_FJ_MASSHOU: u16 = 147u16;
pub const VK_OEM_FJ_TOUROKU: u16 = 148u16;
pub const VK_OEM_FJ_LOYA: u16 = 149u16;
pub const VK_OEM_FJ_ROYA: u16 = 150u16;
pub const VK_LSHIFT: u16 = 160u16;
pub const VK_RSHIFT: u16 = 161u16;
pub const VK_LCONTROL: u16 = 162u16;
pub const VK_RCONTROL: u16 = 163u16;
pub const VK_LMENU: u16 = 164u16;
pub const VK_RMENU: u16 = 165u16;
pub const VK_BROWSER_BACK: u16 = 166u16;
pub const VK_BROWSER_FORWARD: u16 = 167u16;
pub const VK_BROWSER_REFRESH: u16 = 168u16;
pub const VK_BROWSER_STOP: u16 = 169u16;
pub const VK_BROWSER_SEARCH: u16 = 170u16;
pub const VK_BROWSER_FAVORITES: u16 = 171u16;
pub const VK_BROWSER_HOME: u16 = 172u16;
pub const VK_VOLUME_MUTE: u16 = 173u16;
pub const VK_VOLUME_DOWN: u16 = 174u16;
pub const VK_VOLUME_UP: u16 = 175u16;
pub const VK_MEDIA_NEXT_TRACK: u16 = 176u16;
pub const VK_MEDIA_PREV_TRACK: u16 = 177u16;
pub const VK_MEDIA_STOP: u16 = 178u16;
pub const VK_MEDIA_PLAY_PAUSE: u16 = 179u16;
pub const VK_LAUNCH_MAIL: u16 = 180u16;
pub const VK_LAUNCH_MEDIA_SELECT: u16 = 181u16;
pub const VK_LAUNCH_APP1: u16 = 182u16;
pub const VK_LAUNCH_APP2: u16 = 183u16;
pub const VK_OEM_1: u16 = 186u16;
pub const VK_OEM_PLUS: u16 = 187u16;
pub const VK_OEM_COMMA: u16 = 188u16;
pub const VK_OEM_MINUS: u16 = 189u16;
pub const VK_OEM_PERIOD: u16 = 190u16;
pub const VK_OEM_2: u16 = 191u16;
pub const VK_OEM_3: u16 = 192u16;
pub const VK_GAMEPAD_A: u16 = 195u16;
pub const VK_GAMEPAD_B: u16 = 196u16;
pub const VK_GAMEPAD_X: u16 = 197u16;
pub const VK_GAMEPAD_Y: u16 = 198u16;
pub const VK_GAMEPAD_RIGHT_SHOULDER: u16 = 199u16;
pub const VK_GAMEPAD_LEFT_SHOULDER: u16 = 200u16;
pub const VK_GAMEPAD_LEFT_TRIGGER: u16 = 201u16;
pub const VK_GAMEPAD_RIGHT_TRIGGER: u16 = 202u16;
pub const VK_GAMEPAD_DPAD_UP: u16 = 203u16;
pub const VK_GAMEPAD_DPAD_DOWN: u16 = 204u16;
pub const VK_GAMEPAD_DPAD_LEFT: u16 = 205u16;
pub const VK_GAMEPAD_DPAD_RIGHT: u16 = 206u16;
pub const VK_GAMEPAD_MENU: u16 = 207u16;
pub const VK_GAMEPAD_VIEW: u16 = 208u16;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON: u16 = 209u16;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: u16 = 210u16;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_UP: u16 = 211u16;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_DOWN: u16 = 212u16;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT: u16 = 213u16;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_LEFT: u16 = 214u16;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_UP: u16 = 215u16;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN: u16 = 216u16;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: u16 = 217u16;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT: u16 = 218u16;
pub const VK_OEM_4: u16 = 219u16;
pub const VK_OEM_5: u16 = 220u16;
pub const VK_OEM_6: u16 = 221u16;
pub const VK_OEM_7: u16 = 222u16;
pub const VK_OEM_8: u16 = 223u16;
pub const VK_OEM_AX: u16 = 225u16;
pub const VK_OEM_102: u16 = 226u16;
pub const VK_ICO_HELP: u16 = 227u16;
pub const VK_ICO_00: u16 = 228u16;
pub const VK_PROCESSKEY: u16 = 229u16;
pub const VK_ICO_CLEAR: u16 = 230u16;
pub const VK_PACKET: u16 = 231u16;
pub const VK_OEM_RESET: u16 = 233u16;
pub const VK_OEM_JUMP: u16 = 234u16;
pub const VK_OEM_PA1: u16 = 235u16;
pub const VK_OEM_PA2: u16 = 236u16;
pub const VK_OEM_PA3: u16 = 237u16;
pub const VK_OEM_WSCTRL: u16 = 238u16;
pub const VK_OEM_CUSEL: u16 = 239u16;
pub const VK_OEM_ATTN: u16 = 240u16;
pub const VK_OEM_FINISH: u16 = 241u16;
pub const VK_OEM_COPY: u16 = 242u16;
pub const VK_OEM_AUTO: u16 = 243u16;
pub const VK_OEM_ENLW: u16 = 244u16;
pub const VK_OEM_BACKTAB: u16 = 245u16;
pub const VK_ATTN: u16 = 246u16;
pub const VK_CRSEL: u16 = 247u16;
pub const VK_EXSEL: u16 = 248u16;
pub const VK_EREOF: u16 = 249u16;
pub const VK_PLAY: u16 = 250u16;
pub const VK_ZOOM: u16 = 251u16;
pub const VK_NONAME: u16 = 252u16;
pub const VK_PA1: u16 = 253u16;
pub const VK_OEM_CLEAR: u16 = 254u16;
pub const VK_ABNT_C1: u32 = 193u32;
pub const VK_ABNT_C2: u32 = 194u32;
pub const VK_DBE_ALPHANUMERIC: u32 = 240u32;
pub const VK_DBE_CODEINPUT: u32 = 250u32;
pub const VK_DBE_DBCSCHAR: u32 = 244u32;
pub const VK_DBE_DETERMINESTRING: u32 = 252u32;
pub const VK_DBE_ENTERDLGCONVERSIONMODE: u32 = 253u32;
pub const VK_DBE_ENTERIMECONFIGMODE: u32 = 248u32;
pub const VK_DBE_ENTERWORDREGISTERMODE: u32 = 247u32;
pub const VK_DBE_FLUSHSTRING: u32 = 249u32;
pub const VK_DBE_HIRAGANA: u32 = 242u32;
pub const VK_DBE_KATAKANA: u32 = 241u32;
pub const VK_DBE_NOCODEINPUT: u32 = 251u32;
pub const VK_DBE_NOROMAN: u32 = 246u32;
pub const VK_DBE_ROMAN: u32 = 245u32;
pub const VK_DBE_SBCSCHAR: u32 = 243u32;
#[repr(C)]
pub struct VK_TO_BIT {
    pub Vk: u8,
    pub ModBits: u8,
}
impl ::core::marker::Copy for VK_TO_BIT {}
impl ::core::clone::Clone for VK_TO_BIT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VK_TO_WCHARS1 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 1],
}
impl ::core::marker::Copy for VK_TO_WCHARS1 {}
impl ::core::clone::Clone for VK_TO_WCHARS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VK_TO_WCHARS10 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 10],
}
impl ::core::marker::Copy for VK_TO_WCHARS10 {}
impl ::core::clone::Clone for VK_TO_WCHARS10 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VK_TO_WCHARS2 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 2],
}
impl ::core::marker::Copy for VK_TO_WCHARS2 {}
impl ::core::clone::Clone for VK_TO_WCHARS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VK_TO_WCHARS3 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 3],
}
impl ::core::marker::Copy for VK_TO_WCHARS3 {}
impl ::core::clone::Clone for VK_TO_WCHARS3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VK_TO_WCHARS4 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 4],
}
impl ::core::marker::Copy for VK_TO_WCHARS4 {}
impl ::core::clone::Clone for VK_TO_WCHARS4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VK_TO_WCHARS5 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 5],
}
impl ::core::marker::Copy for VK_TO_WCHARS5 {}
impl ::core::clone::Clone for VK_TO_WCHARS5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VK_TO_WCHARS6 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 6],
}
impl ::core::marker::Copy for VK_TO_WCHARS6 {}
impl ::core::clone::Clone for VK_TO_WCHARS6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VK_TO_WCHARS7 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 7],
}
impl ::core::marker::Copy for VK_TO_WCHARS7 {}
impl ::core::clone::Clone for VK_TO_WCHARS7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VK_TO_WCHARS8 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 8],
}
impl ::core::marker::Copy for VK_TO_WCHARS8 {}
impl ::core::clone::Clone for VK_TO_WCHARS8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VK_TO_WCHARS9 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 9],
}
impl ::core::marker::Copy for VK_TO_WCHARS9 {}
impl ::core::clone::Clone for VK_TO_WCHARS9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VK_TO_WCHAR_TABLE {
    pub pVkToWchars: *mut VK_TO_WCHARS1,
    pub nModifications: u8,
    pub cbSize: u8,
}
impl ::core::marker::Copy for VK_TO_WCHAR_TABLE {}
impl ::core::clone::Clone for VK_TO_WCHAR_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VK_VSC {
    pub Vk: u8,
    pub Vsc: u8,
}
impl ::core::marker::Copy for VK_VSC {}
impl ::core::clone::Clone for VK_VSC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VK__none_: u32 = 255u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VSC_LPWSTR {
    pub vsc: u8,
    pub pwsz: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VSC_LPWSTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VSC_LPWSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VSC_VK {
    pub Vsc: u8,
    pub Vk: u16,
}
impl ::core::marker::Copy for VSC_VK {}
impl ::core::clone::Clone for VSC_VK {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WCH_DEAD: u32 = 61441u32;
pub const WCH_LGTR: u32 = 61442u32;
pub const WCH_NONE: u32 = 61440u32;
#[repr(C)]
pub struct _VK_FUNCTION_PARAM {
    pub NLSFEProcIndex: u8,
    pub NLSFEProcParam: u32,
}
impl ::core::marker::Copy for _VK_FUNCTION_PARAM {}
impl ::core::clone::Clone for _VK_FUNCTION_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct _VK_TO_FUNCTION_TABLE {
    pub Vk: u8,
    pub NLSFEProcType: u8,
    pub NLSFEProcCurrent: u8,
    pub NLSFEProcSwitch: u8,
    pub NLSFEProc: [_VK_FUNCTION_PARAM; 8],
    pub NLSFEProcAlt: [_VK_FUNCTION_PARAM; 8],
}
impl ::core::marker::Copy for _VK_TO_FUNCTION_TABLE {}
impl ::core::clone::Clone for _VK_TO_FUNCTION_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct tagKbdLayer {
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for tagKbdLayer {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for tagKbdLayer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct tagKbdNlsLayer {
    pub OEMIdentifier: u16,
    pub LayoutInformation: u16,
    pub NumOfVkToF: u32,
    pub pVkToF: *mut _VK_TO_FUNCTION_TABLE,
    pub NumOfMouseVKey: i32,
    pub pusMouseVKey: *mut u16,
}
impl ::core::marker::Copy for tagKbdNlsLayer {}
impl ::core::clone::Clone for tagKbdNlsLayer {
    fn clone(&self) -> Self {
        *self
    }
}
