#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_UI_TextServices\"`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn ActivateKeyboardLayout(hkl: super::super::TextServices::HKL, flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS) -> super::super::TextServices::HKL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BlockInput(fblockit: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragDetect(hwnd: super::super::super::Foundation::HWND, pt: super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableWindow(hwnd: super::super::super::Foundation::HWND, benable: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetActiveWindow() -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn GetAsyncKeyState(vkey: i32) -> i16;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCapture() -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn GetDoubleClickTime() -> u32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFocus() -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn GetKBCodePage() -> u32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn GetKeyNameTextA(lparam: i32, lpstring: ::windows_sys::core::PSTR, cchsize: i32) -> i32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn GetKeyNameTextW(lparam: i32, lpstring: ::windows_sys::core::PWSTR, cchsize: i32) -> i32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn GetKeyState(nvirtkey: i32) -> i16;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_UI_TextServices\"`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn GetKeyboardLayout(idthread: u32) -> super::super::TextServices::HKL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_UI_TextServices\"`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn GetKeyboardLayoutList(nbuff: i32, lplist: *mut super::super::TextServices::HKL) -> i32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyboardLayoutNameA(pwszklid: ::windows_sys::core::PSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyboardLayoutNameW(pwszklid: ::windows_sys::core::PWSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyboardState(lpkeystate: *mut u8) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn GetKeyboardType(ntypeflag: i32) -> i32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLastInputInfo(plii: *mut LASTINPUTINFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn GetMouseMovePointsEx(cbsize: u32, lppt: *const MOUSEMOVEPOINT, lpptbuf: *mut MOUSEMOVEPOINT, nbufpoints: i32, resolution: GET_MOUSE_MOVE_POINTS_EX_RESOLUTION) -> i32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWindowEnabled(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_UI_TextServices\"`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn LoadKeyboardLayoutA(pwszklid: ::windows_sys::core::PCSTR, flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS) -> super::super::TextServices::HKL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_UI_TextServices\"`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn LoadKeyboardLayoutW(pwszklid: ::windows_sys::core::PCWSTR, flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS) -> super::super::TextServices::HKL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn MapVirtualKeyA(ucode: u32, umaptype: u32) -> u32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_UI_TextServices\"`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn MapVirtualKeyExA(ucode: u32, umaptype: u32, dwhkl: super::super::TextServices::HKL) -> u32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_UI_TextServices\"`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn MapVirtualKeyExW(ucode: u32, umaptype: u32, dwhkl: super::super::TextServices::HKL) -> u32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn MapVirtualKeyW(ucode: u32, umaptype: u32) -> u32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn OemKeyScan(woemchar: u16) -> u32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterHotKey(hwnd: super::super::super::Foundation::HWND, id: i32, fsmodifiers: HOT_KEY_MODIFIERS, vk: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseCapture() -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn SendInput(cinputs: u32, pinputs: *const INPUT, cbsize: i32) -> u32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetActiveWindow(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCapture(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDoubleClickTime(param0: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFocus(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetKeyboardState(lpkeystate: *const u8) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SwapMouseButton(fswap: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn ToAscii(uvirtkey: u32, uscancode: u32, lpkeystate: *const u8, lpchar: *mut u16, uflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_UI_TextServices\"`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn ToAsciiEx(uvirtkey: u32, uscancode: u32, lpkeystate: *const u8, lpchar: *mut u16, uflags: u32, dwhkl: super::super::TextServices::HKL) -> i32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn ToUnicode(wvirtkey: u32, wscancode: u32, lpkeystate: *const u8, pwszbuff: ::windows_sys::core::PWSTR, cchbuff: i32, wflags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_UI_TextServices\"`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn ToUnicodeEx(wvirtkey: u32, wscancode: u32, lpkeystate: *const u8, pwszbuff: ::windows_sys::core::PWSTR, cchbuff: i32, wflags: u32, dwhkl: super::super::TextServices::HKL) -> i32;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TrackMouseEvent(lpeventtrack: *mut TRACKMOUSEEVENT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn UnloadKeyboardLayout(hkl: super::super::TextServices::HKL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterHotKey(hwnd: super::super::super::Foundation::HWND, id: i32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VkKeyScanA(ch: super::super::super::Foundation::CHAR) -> i16;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`, `\"Win32_UI_TextServices\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn VkKeyScanExA(ch: super::super::super::Foundation::CHAR, dwhkl: super::super::TextServices::HKL) -> i16;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_UI_TextServices\"`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn VkKeyScanExW(ch: u16, dwhkl: super::super::TextServices::HKL) -> i16;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn VkKeyScanW(ch: u16) -> i16;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn _TrackMouseEvent(lpeventtrack: *mut TRACKMOUSEEVENT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn keybd_event(bvk: u8, bscan: u8, dwflags: KEYBD_EVENT_FLAGS, dwextrainfo: usize);
    #[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
    pub fn mouse_event(dwflags: MOUSE_EVENT_FLAGS, dx: i32, dy: i32, dwdata: u32, dwextrainfo: usize);
}
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub type ACTIVATE_KEYBOARD_LAYOUT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KLF_REORDER: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KLF_RESET: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KLF_SETFORPROCESS: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KLF_SHIFTLOCK: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KLF_ACTIVATE: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KLF_NOTELLSHELL: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KLF_REPLACELANG: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KLF_SUBSTITUTE_OK: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const ACUTE: u32 = 769u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const AX_KBD_DESKTOP_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const BREVE: u32 = 774u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const CAPLOK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const CAPLOKALTGR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const CEDILLA: u32 = 807u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const CIRCUMFLEX: u32 = 770u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const DEC_KBD_ANSI_LAYOUT_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const DEC_KBD_JIS_LAYOUT_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const DIARESIS: u32 = 776u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const DIARESIS_TONOS: u32 = 901u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const DKF_DEAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const DONTCARE_BIT: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const DOT_ABOVE: u32 = 775u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const DOUBLE_ACUTE: u32 = 779u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const EXTENDED_BIT: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const FAKE_KEYSTROKE: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const FMR_KBD_JIS_TYPE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const FMR_KBD_OASYS_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const FMV_KBD_OASYS_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub type GET_MOUSE_MOVE_POINTS_EX_RESOLUTION = u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const GMMP_USE_DISPLAY_POINTS: GET_MOUSE_MOVE_POINTS_EX_RESOLUTION = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const GMMP_USE_HIGH_RESOLUTION_POINTS: GET_MOUSE_MOVE_POINTS_EX_RESOLUTION = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const GRAVE: u32 = 768u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const GRPSELTAP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const HACEK: u32 = 780u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const HOOK_ABOVE: u32 = 777u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub type HOT_KEY_MODIFIERS = u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOD_ALT: HOT_KEY_MODIFIERS = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOD_CONTROL: HOT_KEY_MODIFIERS = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOD_NOREPEAT: HOT_KEY_MODIFIERS = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOD_SHIFT: HOT_KEY_MODIFIERS = 4u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOD_WIN: HOT_KEY_MODIFIERS = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub type INPUT_TYPE = u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const INPUT_MOUSE: INPUT_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const INPUT_KEYBOARD: INPUT_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const INPUT_HARDWARE: INPUT_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KANALOK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDALT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDBASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDCTRL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDGRPSELTAP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDKANA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDLOYA: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_ALPHANUM: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_CODEINPUT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_CONV_OR_NONCONV: u32 = 15u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_HELP_OR_END: u32 = 11u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_HIRAGANA: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_HOME_OR_CLEAR: u32 = 12u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_INDEX_ALT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_INDEX_NORMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_KANAEVENT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_KANALOCK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_KATAKANA: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_NOEVENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_NUMPAD: u32 = 13u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_ROMAN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_SBCSDBCS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_SEND_BASE_VK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_SEND_PARAM_VK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_TYPE_NORMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_TYPE_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDNLS_TYPE_TOGGLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDROYA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDSHIFT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBDTABLE_MULTI_MAX: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBD_TYPE: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KBD_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub type KEYBD_EVENT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KEYEVENTF_EXTENDEDKEY: KEYBD_EVENT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KEYEVENTF_KEYUP: KEYBD_EVENT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KEYEVENTF_SCANCODE: KEYBD_EVENT_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KEYEVENTF_UNICODE: KEYBD_EVENT_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KEYBOARD_TYPE_GENERIC_101: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KEYBOARD_TYPE_JAPAN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KEYBOARD_TYPE_KOREA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KEYBOARD_TYPE_UNKNOWN: u32 = 81u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KLLF_ALTGR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KLLF_GLOBAL_ATTRS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KLLF_LRM_RLM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const KLLF_SHIFTLOCK: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MACRON: u32 = 772u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MICROSOFT_KBD_001_TYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MICROSOFT_KBD_002_TYPE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MICROSOFT_KBD_101A_TYPE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MICROSOFT_KBD_101B_TYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MICROSOFT_KBD_101C_TYPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MICROSOFT_KBD_101_TYPE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MICROSOFT_KBD_103_TYPE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MICROSOFT_KBD_106_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MICROSOFT_KBD_AX_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MICROSOFT_KBD_FUNC: u32 = 12u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub struct MOUSEINPUT {
    pub dx: i32,
    pub dy: i32,
    pub mouseData: i32,
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub type MOUSE_EVENT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOUSEEVENTF_ABSOLUTE: MOUSE_EVENT_FLAGS = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOUSEEVENTF_LEFTDOWN: MOUSE_EVENT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOUSEEVENTF_LEFTUP: MOUSE_EVENT_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOUSEEVENTF_MIDDLEDOWN: MOUSE_EVENT_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOUSEEVENTF_MIDDLEUP: MOUSE_EVENT_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOUSEEVENTF_MOVE: MOUSE_EVENT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOUSEEVENTF_RIGHTDOWN: MOUSE_EVENT_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOUSEEVENTF_RIGHTUP: MOUSE_EVENT_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOUSEEVENTF_WHEEL: MOUSE_EVENT_FLAGS = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOUSEEVENTF_XDOWN: MOUSE_EVENT_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOUSEEVENTF_XUP: MOUSE_EVENT_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOUSEEVENTF_HWHEEL: MOUSE_EVENT_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOUSEEVENTF_MOVE_NOCOALESCE: MOUSE_EVENT_FLAGS = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const MOUSEEVENTF_VIRTUALDESK: MOUSE_EVENT_FLAGS = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NEC_KBD_106_TYPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NEC_KBD_H_MODE_TYPE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NEC_KBD_LAPTOP_TYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NEC_KBD_NORMAL_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NEC_KBD_N_MODE_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NLSKBD_INFO_ACCESSIBILITY_KEYMAP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NLSKBD_INFO_EMURATE_101_KEYBOARD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NLSKBD_INFO_EMURATE_106_KEYBOARD: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NLSKBD_INFO_SEND_IME_NOTIFICATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NLSKBD_OEM_AX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NLSKBD_OEM_DEC: u32 = 24u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NLSKBD_OEM_EPSON: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NLSKBD_OEM_FUJITSU: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NLSKBD_OEM_IBM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NLSKBD_OEM_MATSUSHITA: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NLSKBD_OEM_MICROSOFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NLSKBD_OEM_NEC: u32 = 13u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const NLSKBD_OEM_TOSHIBA: u32 = 18u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const OGONEK: u32 = 808u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const OVERSCORE: u32 = 773u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const RING: u32 = 778u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const SCANCODE_ALT: u32 = 56u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const SCANCODE_CTRL: u32 = 29u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const SCANCODE_LSHIFT: u32 = 42u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const SCANCODE_LWIN: u32 = 91u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const SCANCODE_NUMPAD_FIRST: u32 = 71u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const SCANCODE_NUMPAD_LAST: u32 = 82u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const SCANCODE_RSHIFT: u32 = 54u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const SCANCODE_RWIN: u32 = 92u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const SCANCODE_THAI_LAYOUT_TOGGLE: u32 = 41u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const SGCAPS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const SHFT_INVALID: u32 = 15u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const TILDE: u32 = 771u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const TONOS: u32 = 900u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const TOSHIBA_KBD_DESKTOP_TYPE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const TOSHIBA_KBD_LAPTOP_TYPE: u32 = 15u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub type TRACKMOUSEEVENT_FLAGS = u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const TME_CANCEL: TRACKMOUSEEVENT_FLAGS = 2147483648u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const TME_HOVER: TRACKMOUSEEVENT_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const TME_LEAVE: TRACKMOUSEEVENT_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const TME_NONCLIENT: TRACKMOUSEEVENT_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const TME_QUERY: TRACKMOUSEEVENT_FLAGS = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const UMLAUT: u32 = 776u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub type VIRTUAL_KEY = u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_0: VIRTUAL_KEY = 48u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_1: VIRTUAL_KEY = 49u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_2: VIRTUAL_KEY = 50u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_3: VIRTUAL_KEY = 51u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_4: VIRTUAL_KEY = 52u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_5: VIRTUAL_KEY = 53u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_6: VIRTUAL_KEY = 54u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_7: VIRTUAL_KEY = 55u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_8: VIRTUAL_KEY = 56u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_9: VIRTUAL_KEY = 57u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_A: VIRTUAL_KEY = 65u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_B: VIRTUAL_KEY = 66u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_C: VIRTUAL_KEY = 67u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_D: VIRTUAL_KEY = 68u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_E: VIRTUAL_KEY = 69u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F: VIRTUAL_KEY = 70u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_G: VIRTUAL_KEY = 71u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_H: VIRTUAL_KEY = 72u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_I: VIRTUAL_KEY = 73u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_J: VIRTUAL_KEY = 74u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_K: VIRTUAL_KEY = 75u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_L: VIRTUAL_KEY = 76u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_M: VIRTUAL_KEY = 77u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_N: VIRTUAL_KEY = 78u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_O: VIRTUAL_KEY = 79u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_P: VIRTUAL_KEY = 80u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_Q: VIRTUAL_KEY = 81u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_R: VIRTUAL_KEY = 82u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_S: VIRTUAL_KEY = 83u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_T: VIRTUAL_KEY = 84u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_U: VIRTUAL_KEY = 85u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_V: VIRTUAL_KEY = 86u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_W: VIRTUAL_KEY = 87u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_X: VIRTUAL_KEY = 88u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_Y: VIRTUAL_KEY = 89u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_Z: VIRTUAL_KEY = 90u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_LBUTTON: VIRTUAL_KEY = 1u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_RBUTTON: VIRTUAL_KEY = 2u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_CANCEL: VIRTUAL_KEY = 3u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_MBUTTON: VIRTUAL_KEY = 4u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_XBUTTON1: VIRTUAL_KEY = 5u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_XBUTTON2: VIRTUAL_KEY = 6u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_BACK: VIRTUAL_KEY = 8u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_TAB: VIRTUAL_KEY = 9u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_CLEAR: VIRTUAL_KEY = 12u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_RETURN: VIRTUAL_KEY = 13u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_SHIFT: VIRTUAL_KEY = 16u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_CONTROL: VIRTUAL_KEY = 17u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_MENU: VIRTUAL_KEY = 18u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_PAUSE: VIRTUAL_KEY = 19u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_CAPITAL: VIRTUAL_KEY = 20u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_KANA: VIRTUAL_KEY = 21u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_HANGEUL: VIRTUAL_KEY = 21u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_HANGUL: VIRTUAL_KEY = 21u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_IME_ON: VIRTUAL_KEY = 22u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_JUNJA: VIRTUAL_KEY = 23u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_FINAL: VIRTUAL_KEY = 24u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_HANJA: VIRTUAL_KEY = 25u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_KANJI: VIRTUAL_KEY = 25u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_IME_OFF: VIRTUAL_KEY = 26u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_ESCAPE: VIRTUAL_KEY = 27u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_CONVERT: VIRTUAL_KEY = 28u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NONCONVERT: VIRTUAL_KEY = 29u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_ACCEPT: VIRTUAL_KEY = 30u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_MODECHANGE: VIRTUAL_KEY = 31u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_SPACE: VIRTUAL_KEY = 32u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_PRIOR: VIRTUAL_KEY = 33u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NEXT: VIRTUAL_KEY = 34u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_END: VIRTUAL_KEY = 35u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_HOME: VIRTUAL_KEY = 36u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_LEFT: VIRTUAL_KEY = 37u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_UP: VIRTUAL_KEY = 38u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_RIGHT: VIRTUAL_KEY = 39u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DOWN: VIRTUAL_KEY = 40u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_SELECT: VIRTUAL_KEY = 41u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_PRINT: VIRTUAL_KEY = 42u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_EXECUTE: VIRTUAL_KEY = 43u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_SNAPSHOT: VIRTUAL_KEY = 44u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_INSERT: VIRTUAL_KEY = 45u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DELETE: VIRTUAL_KEY = 46u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_HELP: VIRTUAL_KEY = 47u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_LWIN: VIRTUAL_KEY = 91u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_RWIN: VIRTUAL_KEY = 92u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_APPS: VIRTUAL_KEY = 93u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_SLEEP: VIRTUAL_KEY = 95u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NUMPAD0: VIRTUAL_KEY = 96u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NUMPAD1: VIRTUAL_KEY = 97u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NUMPAD2: VIRTUAL_KEY = 98u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NUMPAD3: VIRTUAL_KEY = 99u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NUMPAD4: VIRTUAL_KEY = 100u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NUMPAD5: VIRTUAL_KEY = 101u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NUMPAD6: VIRTUAL_KEY = 102u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NUMPAD7: VIRTUAL_KEY = 103u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NUMPAD8: VIRTUAL_KEY = 104u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NUMPAD9: VIRTUAL_KEY = 105u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_MULTIPLY: VIRTUAL_KEY = 106u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_ADD: VIRTUAL_KEY = 107u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_SEPARATOR: VIRTUAL_KEY = 108u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_SUBTRACT: VIRTUAL_KEY = 109u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DECIMAL: VIRTUAL_KEY = 110u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DIVIDE: VIRTUAL_KEY = 111u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F1: VIRTUAL_KEY = 112u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F2: VIRTUAL_KEY = 113u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F3: VIRTUAL_KEY = 114u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F4: VIRTUAL_KEY = 115u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F5: VIRTUAL_KEY = 116u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F6: VIRTUAL_KEY = 117u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F7: VIRTUAL_KEY = 118u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F8: VIRTUAL_KEY = 119u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F9: VIRTUAL_KEY = 120u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F10: VIRTUAL_KEY = 121u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F11: VIRTUAL_KEY = 122u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F12: VIRTUAL_KEY = 123u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F13: VIRTUAL_KEY = 124u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F14: VIRTUAL_KEY = 125u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F15: VIRTUAL_KEY = 126u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F16: VIRTUAL_KEY = 127u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F17: VIRTUAL_KEY = 128u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F18: VIRTUAL_KEY = 129u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F19: VIRTUAL_KEY = 130u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F20: VIRTUAL_KEY = 131u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F21: VIRTUAL_KEY = 132u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F22: VIRTUAL_KEY = 133u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F23: VIRTUAL_KEY = 134u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_F24: VIRTUAL_KEY = 135u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NAVIGATION_VIEW: VIRTUAL_KEY = 136u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NAVIGATION_MENU: VIRTUAL_KEY = 137u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NAVIGATION_UP: VIRTUAL_KEY = 138u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NAVIGATION_DOWN: VIRTUAL_KEY = 139u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NAVIGATION_LEFT: VIRTUAL_KEY = 140u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NAVIGATION_RIGHT: VIRTUAL_KEY = 141u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NAVIGATION_ACCEPT: VIRTUAL_KEY = 142u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NAVIGATION_CANCEL: VIRTUAL_KEY = 143u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NUMLOCK: VIRTUAL_KEY = 144u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_SCROLL: VIRTUAL_KEY = 145u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_NEC_EQUAL: VIRTUAL_KEY = 146u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_FJ_JISHO: VIRTUAL_KEY = 146u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_FJ_MASSHOU: VIRTUAL_KEY = 147u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_FJ_TOUROKU: VIRTUAL_KEY = 148u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_FJ_LOYA: VIRTUAL_KEY = 149u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_FJ_ROYA: VIRTUAL_KEY = 150u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_LSHIFT: VIRTUAL_KEY = 160u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_RSHIFT: VIRTUAL_KEY = 161u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_LCONTROL: VIRTUAL_KEY = 162u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_RCONTROL: VIRTUAL_KEY = 163u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_LMENU: VIRTUAL_KEY = 164u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_RMENU: VIRTUAL_KEY = 165u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_BROWSER_BACK: VIRTUAL_KEY = 166u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_BROWSER_FORWARD: VIRTUAL_KEY = 167u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_BROWSER_REFRESH: VIRTUAL_KEY = 168u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_BROWSER_STOP: VIRTUAL_KEY = 169u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_BROWSER_SEARCH: VIRTUAL_KEY = 170u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_BROWSER_FAVORITES: VIRTUAL_KEY = 171u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_BROWSER_HOME: VIRTUAL_KEY = 172u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_VOLUME_MUTE: VIRTUAL_KEY = 173u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_VOLUME_DOWN: VIRTUAL_KEY = 174u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_VOLUME_UP: VIRTUAL_KEY = 175u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_MEDIA_NEXT_TRACK: VIRTUAL_KEY = 176u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_MEDIA_PREV_TRACK: VIRTUAL_KEY = 177u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_MEDIA_STOP: VIRTUAL_KEY = 178u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_MEDIA_PLAY_PAUSE: VIRTUAL_KEY = 179u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_LAUNCH_MAIL: VIRTUAL_KEY = 180u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_LAUNCH_MEDIA_SELECT: VIRTUAL_KEY = 181u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_LAUNCH_APP1: VIRTUAL_KEY = 182u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_LAUNCH_APP2: VIRTUAL_KEY = 183u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_1: VIRTUAL_KEY = 186u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_PLUS: VIRTUAL_KEY = 187u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_COMMA: VIRTUAL_KEY = 188u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_MINUS: VIRTUAL_KEY = 189u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_PERIOD: VIRTUAL_KEY = 190u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_2: VIRTUAL_KEY = 191u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_3: VIRTUAL_KEY = 192u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_A: VIRTUAL_KEY = 195u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_B: VIRTUAL_KEY = 196u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_X: VIRTUAL_KEY = 197u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_Y: VIRTUAL_KEY = 198u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_RIGHT_SHOULDER: VIRTUAL_KEY = 199u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_LEFT_SHOULDER: VIRTUAL_KEY = 200u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_LEFT_TRIGGER: VIRTUAL_KEY = 201u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_RIGHT_TRIGGER: VIRTUAL_KEY = 202u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_DPAD_UP: VIRTUAL_KEY = 203u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_DPAD_DOWN: VIRTUAL_KEY = 204u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_DPAD_LEFT: VIRTUAL_KEY = 205u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_DPAD_RIGHT: VIRTUAL_KEY = 206u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_MENU: VIRTUAL_KEY = 207u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_VIEW: VIRTUAL_KEY = 208u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON: VIRTUAL_KEY = 209u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: VIRTUAL_KEY = 210u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_LEFT_THUMBSTICK_UP: VIRTUAL_KEY = 211u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_LEFT_THUMBSTICK_DOWN: VIRTUAL_KEY = 212u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT: VIRTUAL_KEY = 213u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_LEFT_THUMBSTICK_LEFT: VIRTUAL_KEY = 214u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_UP: VIRTUAL_KEY = 215u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN: VIRTUAL_KEY = 216u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: VIRTUAL_KEY = 217u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT: VIRTUAL_KEY = 218u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_4: VIRTUAL_KEY = 219u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_5: VIRTUAL_KEY = 220u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_6: VIRTUAL_KEY = 221u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_7: VIRTUAL_KEY = 222u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_8: VIRTUAL_KEY = 223u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_AX: VIRTUAL_KEY = 225u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_102: VIRTUAL_KEY = 226u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_ICO_HELP: VIRTUAL_KEY = 227u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_ICO_00: VIRTUAL_KEY = 228u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_PROCESSKEY: VIRTUAL_KEY = 229u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_ICO_CLEAR: VIRTUAL_KEY = 230u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_PACKET: VIRTUAL_KEY = 231u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_RESET: VIRTUAL_KEY = 233u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_JUMP: VIRTUAL_KEY = 234u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_PA1: VIRTUAL_KEY = 235u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_PA2: VIRTUAL_KEY = 236u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_PA3: VIRTUAL_KEY = 237u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_WSCTRL: VIRTUAL_KEY = 238u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_CUSEL: VIRTUAL_KEY = 239u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_ATTN: VIRTUAL_KEY = 240u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_FINISH: VIRTUAL_KEY = 241u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_COPY: VIRTUAL_KEY = 242u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_AUTO: VIRTUAL_KEY = 243u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_ENLW: VIRTUAL_KEY = 244u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_BACKTAB: VIRTUAL_KEY = 245u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_ATTN: VIRTUAL_KEY = 246u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_CRSEL: VIRTUAL_KEY = 247u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_EXSEL: VIRTUAL_KEY = 248u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_EREOF: VIRTUAL_KEY = 249u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_PLAY: VIRTUAL_KEY = 250u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_ZOOM: VIRTUAL_KEY = 251u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_NONAME: VIRTUAL_KEY = 252u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_PA1: VIRTUAL_KEY = 253u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_OEM_CLEAR: VIRTUAL_KEY = 254u16;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_ABNT_C1: u32 = 193u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_ABNT_C2: u32 = 194u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DBE_ALPHANUMERIC: u32 = 240u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DBE_CODEINPUT: u32 = 250u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DBE_DBCSCHAR: u32 = 244u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DBE_DETERMINESTRING: u32 = 252u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DBE_ENTERDLGCONVERSIONMODE: u32 = 253u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DBE_ENTERIMECONFIGMODE: u32 = 248u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DBE_ENTERWORDREGISTERMODE: u32 = 247u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DBE_FLUSHSTRING: u32 = 249u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DBE_HIRAGANA: u32 = 242u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DBE_KATAKANA: u32 = 241u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DBE_NOCODEINPUT: u32 = 251u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DBE_NOROMAN: u32 = 246u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DBE_ROMAN: u32 = 245u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK_DBE_SBCSCHAR: u32 = 243u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const VK__none_: u32 = 255u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub struct VSC_LPWSTR {
    pub vsc: u8,
    pub pwsz: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for VSC_LPWSTR {}
impl ::core::clone::Clone for VSC_LPWSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const WCH_DEAD: u32 = 61441u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const WCH_LGTR: u32 = 61442u32;
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const WCH_NONE: u32 = 61440u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
impl ::core::marker::Copy for tagKbdLayer {}
impl ::core::clone::Clone for tagKbdLayer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszACUTE: &str = "\u{301}";
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszBREVE: &str = "\u{306}";
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszCEDILLA: &str = "\u{327}";
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszCIRCUMFLEX: &str = "\u{302}";
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszDIARESIS_TONOS: &str = "\u{385}";
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszDOT_ABOVE: &str = "\u{307}";
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszDOUBLE_ACUTE: &str = "\u{30b}";
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszGRAVE: &str = "\u{300}";
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszHACEK: &str = "\u{30c}";
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszHOOK_ABOVE: &str = "\u{309}";
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszMACRON: &str = "\u{304}";
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszOGONEK: &str = "\u{328}";
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszOVERSCORE: &str = "\u{305}";
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszRING: &str = "\u{30a}";
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszTILDE: &str = "\u{303}";
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszTONOS: &str = "\u{384}";
#[doc = "*Required features: `\"Win32_UI_Input_KeyboardAndMouse\"`*"]
pub const wszUMLAUT: &str = "\u{308}";
