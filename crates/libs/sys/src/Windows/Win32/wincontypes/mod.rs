pub const ALTNUMPAD_BIT: i32 = 67108864;
pub const CAPSLOCK_ON: i32 = 128;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CHAR_INFO {
    pub Char: CHAR_INFO_0,
    pub Attributes: super::WORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CHAR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union CHAR_INFO_0 {
    pub UnicodeChar: super::WCHAR,
    pub AsciiChar: super::CHAR,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CHAR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_FONT_INFO {
    pub nFont: super::DWORD,
    pub dwFontSize: COORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct COORD {
    pub X: super::SHORT,
    pub Y: super::SHORT,
}
pub const DOUBLE_CLICK: i32 = 2;
pub const ENHANCED_KEY: i32 = 256;
pub const FOCUS_EVENT: i32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FOCUS_EVENT_RECORD {
    pub bSetFocus: windows_sys::core::BOOL,
}
pub const FROM_LEFT_1ST_BUTTON_PRESSED: i32 = 1;
pub const FROM_LEFT_2ND_BUTTON_PRESSED: i32 = 4;
pub const FROM_LEFT_3RD_BUTTON_PRESSED: i32 = 8;
pub const FROM_LEFT_4TH_BUTTON_PRESSED: i32 = 16;
pub type HPCON = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct INPUT_RECORD {
    pub EventType: super::WORD,
    pub Event: INPUT_RECORD_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for INPUT_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union INPUT_RECORD_0 {
    pub KeyEvent: KEY_EVENT_RECORD,
    pub MouseEvent: MOUSE_EVENT_RECORD,
    pub WindowBufferSizeEvent: WINDOW_BUFFER_SIZE_RECORD,
    pub MenuEvent: MENU_EVENT_RECORD,
    pub FocusEvent: FOCUS_EVENT_RECORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for INPUT_RECORD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KEY_EVENT: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KEY_EVENT_RECORD {
    pub bKeyDown: windows_sys::core::BOOL,
    pub wRepeatCount: super::WORD,
    pub wVirtualKeyCode: super::WORD,
    pub wVirtualScanCode: super::WORD,
    pub uChar: KEY_EVENT_RECORD_0,
    pub dwControlKeyState: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for KEY_EVENT_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KEY_EVENT_RECORD_0 {
    pub UnicodeChar: super::WCHAR,
    pub AsciiChar: super::CHAR,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for KEY_EVENT_RECORD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LEFT_ALT_PRESSED: i32 = 2;
pub const LEFT_CTRL_PRESSED: i32 = 8;
pub const MENU_EVENT: i32 = 8;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MENU_EVENT_RECORD {
    pub dwCommandId: super::UINT,
}
pub const MOUSE_EVENT: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MOUSE_EVENT_RECORD {
    pub dwMousePosition: COORD,
    pub dwButtonState: super::DWORD,
    pub dwControlKeyState: super::DWORD,
    pub dwEventFlags: super::DWORD,
}
pub const MOUSE_HWHEELED: i32 = 8;
pub const MOUSE_MOVED: i32 = 1;
pub const MOUSE_WHEELED: i32 = 4;
pub const NLS_ALPHANUMERIC: i32 = 0;
pub const NLS_DBCSCHAR: i32 = 65536;
pub const NLS_HIRAGANA: i32 = 262144;
pub const NLS_IME_CONVERSION: i32 = 8388608;
pub const NLS_IME_DISABLE: i32 = 536870912;
pub const NLS_KATAKANA: i32 = 131072;
pub const NLS_ROMAN: i32 = 4194304;
pub const NUMLOCK_ON: i32 = 32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCHAR_INFO = *mut CHAR_INFO;
#[cfg(feature = "shared")]
pub type PCONSOLE_FONT_INFO = *mut CONSOLE_FONT_INFO;
#[cfg(feature = "shared")]
pub type PCOORD = *mut COORD;
pub type PFOCUS_EVENT_RECORD = *mut FOCUS_EVENT_RECORD;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PINPUT_RECORD = *mut INPUT_RECORD;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PKEY_EVENT_RECORD = *mut KEY_EVENT_RECORD;
#[cfg(feature = "shared")]
pub type PMENU_EVENT_RECORD = *mut MENU_EVENT_RECORD;
#[cfg(feature = "shared")]
pub type PMOUSE_EVENT_RECORD = *mut MOUSE_EVENT_RECORD;
#[cfg(feature = "shared")]
pub type PSMALL_RECT = *mut SMALL_RECT;
#[cfg(feature = "shared")]
pub type PWINDOW_BUFFER_SIZE_RECORD = *mut WINDOW_BUFFER_SIZE_RECORD;
pub const RIGHTMOST_BUTTON_PRESSED: i32 = 2;
pub const RIGHT_ALT_PRESSED: i32 = 1;
pub const RIGHT_CTRL_PRESSED: i32 = 4;
pub const SCROLLLOCK_ON: i32 = 64;
pub const SHIFT_PRESSED: i32 = 16;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SMALL_RECT {
    pub Left: super::SHORT,
    pub Top: super::SHORT,
    pub Right: super::SHORT,
    pub Bottom: super::SHORT,
}
pub const WINDOW_BUFFER_SIZE_EVENT: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WINDOW_BUFFER_SIZE_RECORD {
    pub dwSize: COORD,
}
