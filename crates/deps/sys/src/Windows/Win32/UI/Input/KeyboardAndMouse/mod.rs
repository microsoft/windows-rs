#![allow(non_snake_case, non_camel_case_types)]
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
pub struct ACTIVATE_KEYBOARD_LAYOUT_FLAGS(i32);
pub const ACUTE: u32 = 769u32;
pub const AX_KBD_DESKTOP_TYPE: u32 = 1u32;
pub const BREVE: u32 = 774u32;
pub const CAPLOK: u32 = 1u32;
pub const CAPLOKALTGR: u32 = 4u32;
pub const CEDILLA: u32 = 807u32;
pub const CIRCUMFLEX: u32 = 770u32;
pub struct DEADKEY(i32);
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
pub struct GET_MOUSE_MOVE_POINTS_EX_RESOLUTION(i32);
pub const GRAVE: u32 = 768u32;
pub const GRPSELTAP: u32 = 128u32;
pub const HACEK: u32 = 780u32;
pub struct HARDWAREINPUT(i32);
pub const HOOK_ABOVE: u32 = 777u32;
pub struct HOT_KEY_MODIFIERS(i32);
pub struct INPUT(i32);
pub struct INPUT_TYPE(i32);
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
pub struct KBDTABLE_DESC(i32);
pub struct KBDTABLE_MULTI(i32);
pub const KBDTABLE_MULTI_MAX: u32 = 8u32;
pub const KBD_TYPE: u32 = 4u32;
pub struct KBD_TYPE_INFO(i32);
pub const KBD_VERSION: u32 = 1u32;
pub struct KEYBDINPUT(i32);
pub struct KEYBD_EVENT_FLAGS(i32);
pub const KEYBOARD_TYPE_GENERIC_101: u32 = 4u32;
pub const KEYBOARD_TYPE_JAPAN: u32 = 7u32;
pub const KEYBOARD_TYPE_KOREA: u32 = 8u32;
pub const KEYBOARD_TYPE_UNKNOWN: u32 = 81u32;
pub const KLLF_ALTGR: u32 = 1u32;
pub const KLLF_GLOBAL_ATTRS: u32 = 2u32;
pub const KLLF_LRM_RLM: u32 = 4u32;
pub const KLLF_SHIFTLOCK: u32 = 2u32;
pub struct LASTINPUTINFO(i32);
pub struct LIGATURE1(i32);
pub struct LIGATURE2(i32);
pub struct LIGATURE3(i32);
pub struct LIGATURE4(i32);
pub struct LIGATURE5(i32);
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
pub struct MODIFIERS(i32);
pub struct MOUSEINPUT(i32);
pub struct MOUSEMOVEPOINT(i32);
pub struct MOUSE_EVENT_FLAGS(i32);
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
#[cfg(feature = "Win32_Foundation")]
pub struct TRACKMOUSEEVENT(i32);
pub struct TRACKMOUSEEVENT_FLAGS(i32);
pub const UMLAUT: u32 = 776u32;
pub struct VIRTUAL_KEY(i32);
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
pub struct VK_TO_BIT(i32);
pub struct VK_TO_WCHARS1(i32);
pub struct VK_TO_WCHARS10(i32);
pub struct VK_TO_WCHARS2(i32);
pub struct VK_TO_WCHARS3(i32);
pub struct VK_TO_WCHARS4(i32);
pub struct VK_TO_WCHARS5(i32);
pub struct VK_TO_WCHARS6(i32);
pub struct VK_TO_WCHARS7(i32);
pub struct VK_TO_WCHARS8(i32);
pub struct VK_TO_WCHARS9(i32);
pub struct VK_TO_WCHAR_TABLE(i32);
pub struct VK_VSC(i32);
pub const VK__none_: u32 = 255u32;
#[cfg(feature = "Win32_Foundation")]
pub struct VSC_LPWSTR(i32);
pub struct VSC_VK(i32);
pub const WCH_DEAD: u32 = 61441u32;
pub const WCH_LGTR: u32 = 61442u32;
pub const WCH_NONE: u32 = 61440u32;
pub struct _VK_FUNCTION_PARAM(i32);
pub struct _VK_TO_FUNCTION_TABLE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct tagKbdLayer(i32);
pub struct tagKbdNlsLayer(i32);
