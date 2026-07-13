pub const ALTNUMPAD_BIT: u32 = 67108864;
pub const CAPSLOCK_ON: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CHAR_INFO {
    pub Char: CHAR_INFO_0,
    pub Attributes: u16,
}
impl Default for CHAR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CHAR_INFO_0 {
    pub UnicodeChar: u16,
    pub AsciiChar: i8,
}
impl Default for CHAR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_FONT_INFO {
    pub nFont: u32,
    pub dwFontSize: COORD,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct COORD {
    pub X: i16,
    pub Y: i16,
}
pub const DOUBLE_CLICK: u32 = 2;
pub const ENHANCED_KEY: u32 = 256;
pub const FOCUS_EVENT: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FOCUS_EVENT_RECORD {
    pub bSetFocus: windows_sys::core::BOOL,
}
pub const FROM_LEFT_1ST_BUTTON_PRESSED: u32 = 1;
pub const FROM_LEFT_2ND_BUTTON_PRESSED: u32 = 4;
pub const FROM_LEFT_3RD_BUTTON_PRESSED: u32 = 8;
pub const FROM_LEFT_4TH_BUTTON_PRESSED: u32 = 16;
pub type HPCON = *mut core::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INPUT_RECORD {
    pub EventType: u16,
    pub Event: INPUT_RECORD_0,
}
impl Default for INPUT_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INPUT_RECORD_0 {
    pub KeyEvent: KEY_EVENT_RECORD,
    pub MouseEvent: MOUSE_EVENT_RECORD,
    pub WindowBufferSizeEvent: WINDOW_BUFFER_SIZE_RECORD,
    pub MenuEvent: MENU_EVENT_RECORD,
    pub FocusEvent: FOCUS_EVENT_RECORD,
}
impl Default for INPUT_RECORD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KEY_EVENT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_EVENT_RECORD {
    pub bKeyDown: windows_sys::core::BOOL,
    pub wRepeatCount: u16,
    pub wVirtualKeyCode: u16,
    pub wVirtualScanCode: u16,
    pub uChar: KEY_EVENT_RECORD_0,
    pub dwControlKeyState: u32,
}
impl Default for KEY_EVENT_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KEY_EVENT_RECORD_0 {
    pub UnicodeChar: u16,
    pub AsciiChar: i8,
}
impl Default for KEY_EVENT_RECORD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LEFT_ALT_PRESSED: u32 = 2;
pub const LEFT_CTRL_PRESSED: u32 = 8;
pub const MENU_EVENT: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MENU_EVENT_RECORD {
    pub dwCommandId: u32,
}
pub const MOUSE_EVENT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MOUSE_EVENT_RECORD {
    pub dwMousePosition: COORD,
    pub dwButtonState: u32,
    pub dwControlKeyState: u32,
    pub dwEventFlags: u32,
}
pub const MOUSE_HWHEELED: u32 = 8;
pub const MOUSE_MOVED: u32 = 1;
pub const MOUSE_WHEELED: u32 = 4;
pub const NLS_ALPHANUMERIC: u32 = 0;
pub const NLS_DBCSCHAR: u32 = 65536;
pub const NLS_HIRAGANA: u32 = 262144;
pub const NLS_IME_CONVERSION: u32 = 8388608;
pub const NLS_IME_DISABLE: u32 = 536870912;
pub const NLS_KATAKANA: u32 = 131072;
pub const NLS_ROMAN: u32 = 4194304;
pub const NUMLOCK_ON: u32 = 32;
pub type PCHAR_INFO = *mut CHAR_INFO;
pub type PCONSOLE_FONT_INFO = *mut CONSOLE_FONT_INFO;
pub type PCOORD = *mut COORD;
pub type PFOCUS_EVENT_RECORD = *mut FOCUS_EVENT_RECORD;
pub type PINPUT_RECORD = *mut INPUT_RECORD;
pub type PKEY_EVENT_RECORD = *mut KEY_EVENT_RECORD;
pub type PMENU_EVENT_RECORD = *mut MENU_EVENT_RECORD;
pub type PMOUSE_EVENT_RECORD = *mut MOUSE_EVENT_RECORD;
pub type PSMALL_RECT = *mut SMALL_RECT;
pub type PWINDOW_BUFFER_SIZE_RECORD = *mut WINDOW_BUFFER_SIZE_RECORD;
pub const RIGHTMOST_BUTTON_PRESSED: u32 = 2;
pub const RIGHT_ALT_PRESSED: u32 = 1;
pub const RIGHT_CTRL_PRESSED: u32 = 4;
pub const SCROLLLOCK_ON: u32 = 64;
pub const SHIFT_PRESSED: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
pub const WINDOW_BUFFER_SIZE_EVENT: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINDOW_BUFFER_SIZE_RECORD {
    pub dwSize: COORD,
}
