#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ALTNUMPAD_BIT: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ATTACH_PARENT_PROCESS: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddConsoleAliasA<'a, P0, P1, P2>(source: P0, target: P1, exename: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AddConsoleAliasA(source: ::windows::core::PCSTR, target: ::windows::core::PCSTR, exename: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    AddConsoleAliasA(source.into(), target.into(), exename.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddConsoleAliasW<'a, P0, P1, P2>(source: P0, target: P1, exename: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AddConsoleAliasW(source: ::windows::core::PCWSTR, target: ::windows::core::PCWSTR, exename: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    AddConsoleAliasW(source.into(), target.into(), exename.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllocConsole() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AllocConsole() -> super::super::Foundation::BOOL;
    }
    AllocConsole()
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AttachConsole(dwprocessid: u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AttachConsole(dwprocessid: u32) -> super::super::Foundation::BOOL;
    }
    AttachConsole(dwprocessid)
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const BACKGROUND_BLUE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const BACKGROUND_GREEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const BACKGROUND_INTENSITY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const BACKGROUND_RED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CAPSLOCK_ON: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHAR_INFO {
    pub Char: CHAR_INFO_0,
    pub Attributes: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHAR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHAR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CHAR_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHAR_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHAR_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHAR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHAR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CHAR_INFO_0 {
    pub UnicodeChar: u16,
    pub AsciiChar: super::super::Foundation::CHAR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHAR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHAR_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CHAR_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHAR_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHAR_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHAR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHAR_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const COMMON_LVB_GRID_HORIZONTAL: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const COMMON_LVB_GRID_LVERTICAL: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const COMMON_LVB_GRID_RVERTICAL: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const COMMON_LVB_LEADING_BYTE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const COMMON_LVB_REVERSE_VIDEO: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const COMMON_LVB_SBCSDBCS: u32 = 768u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const COMMON_LVB_TRAILING_BYTE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const COMMON_LVB_UNDERSCORE: u32 = 32768u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CONSOLE_CURSOR_INFO {
    pub dwSize: u32,
    pub bVisible: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONSOLE_CURSOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONSOLE_CURSOR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CONSOLE_CURSOR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_CURSOR_INFO").field("dwSize", &self.dwSize).field("bVisible", &self.bVisible).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CONSOLE_CURSOR_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CONSOLE_CURSOR_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONSOLE_CURSOR_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CONSOLE_CURSOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CONSOLE_CURSOR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct CONSOLE_FONT_INFO {
    pub nFont: u32,
    pub dwFontSize: COORD,
}
impl ::core::marker::Copy for CONSOLE_FONT_INFO {}
impl ::core::clone::Clone for CONSOLE_FONT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_FONT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_FONT_INFO").field("nFont", &self.nFont).field("dwFontSize", &self.dwFontSize).finish()
    }
}
unsafe impl ::windows::core::Abi for CONSOLE_FONT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONSOLE_FONT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONSOLE_FONT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONSOLE_FONT_INFO {}
impl ::core::default::Default for CONSOLE_FONT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct CONSOLE_FONT_INFOEX {
    pub cbSize: u32,
    pub nFont: u32,
    pub dwFontSize: COORD,
    pub FontFamily: u32,
    pub FontWeight: u32,
    pub FaceName: [u16; 32],
}
impl ::core::marker::Copy for CONSOLE_FONT_INFOEX {}
impl ::core::clone::Clone for CONSOLE_FONT_INFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_FONT_INFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_FONT_INFOEX").field("cbSize", &self.cbSize).field("nFont", &self.nFont).field("dwFontSize", &self.dwFontSize).field("FontFamily", &self.FontFamily).field("FontWeight", &self.FontWeight).field("FaceName", &self.FaceName).finish()
    }
}
unsafe impl ::windows::core::Abi for CONSOLE_FONT_INFOEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONSOLE_FONT_INFOEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONSOLE_FONT_INFOEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONSOLE_FONT_INFOEX {}
impl ::core::default::Default for CONSOLE_FONT_INFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_FULLSCREEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_FULLSCREEN_HARDWARE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_FULLSCREEN_MODE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct CONSOLE_HISTORY_INFO {
    pub cbSize: u32,
    pub HistoryBufferSize: u32,
    pub NumberOfHistoryBuffers: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for CONSOLE_HISTORY_INFO {}
impl ::core::clone::Clone for CONSOLE_HISTORY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_HISTORY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_HISTORY_INFO").field("cbSize", &self.cbSize).field("HistoryBufferSize", &self.HistoryBufferSize).field("NumberOfHistoryBuffers", &self.NumberOfHistoryBuffers).field("dwFlags", &self.dwFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for CONSOLE_HISTORY_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONSOLE_HISTORY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONSOLE_HISTORY_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONSOLE_HISTORY_INFO {}
impl ::core::default::Default for CONSOLE_HISTORY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONSOLE_MODE(pub u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_PROCESSED_INPUT: CONSOLE_MODE = CONSOLE_MODE(1u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_LINE_INPUT: CONSOLE_MODE = CONSOLE_MODE(2u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_ECHO_INPUT: CONSOLE_MODE = CONSOLE_MODE(4u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_WINDOW_INPUT: CONSOLE_MODE = CONSOLE_MODE(8u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_MOUSE_INPUT: CONSOLE_MODE = CONSOLE_MODE(16u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_INSERT_MODE: CONSOLE_MODE = CONSOLE_MODE(32u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_QUICK_EDIT_MODE: CONSOLE_MODE = CONSOLE_MODE(64u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_EXTENDED_FLAGS: CONSOLE_MODE = CONSOLE_MODE(128u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_AUTO_POSITION: CONSOLE_MODE = CONSOLE_MODE(256u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: CONSOLE_MODE = CONSOLE_MODE(512u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_PROCESSED_OUTPUT: CONSOLE_MODE = CONSOLE_MODE(1u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_WRAP_AT_EOL_OUTPUT: CONSOLE_MODE = CONSOLE_MODE(2u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: CONSOLE_MODE = CONSOLE_MODE(4u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const DISABLE_NEWLINE_AUTO_RETURN: CONSOLE_MODE = CONSOLE_MODE(8u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_LVB_GRID_WORLDWIDE: CONSOLE_MODE = CONSOLE_MODE(16u32);
impl ::core::marker::Copy for CONSOLE_MODE {}
impl ::core::clone::Clone for CONSOLE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONSOLE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CONSOLE_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONSOLE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONSOLE_MODE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CONSOLE_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CONSOLE_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CONSOLE_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CONSOLE_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CONSOLE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_MOUSE_DOWN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_MOUSE_SELECTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_NO_SELECTION: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct CONSOLE_READCONSOLE_CONTROL {
    pub nLength: u32,
    pub nInitialChars: u32,
    pub dwCtrlWakeupMask: u32,
    pub dwControlKeyState: u32,
}
impl ::core::marker::Copy for CONSOLE_READCONSOLE_CONTROL {}
impl ::core::clone::Clone for CONSOLE_READCONSOLE_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_READCONSOLE_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_READCONSOLE_CONTROL").field("nLength", &self.nLength).field("nInitialChars", &self.nInitialChars).field("dwCtrlWakeupMask", &self.dwCtrlWakeupMask).field("dwControlKeyState", &self.dwControlKeyState).finish()
    }
}
unsafe impl ::windows::core::Abi for CONSOLE_READCONSOLE_CONTROL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONSOLE_READCONSOLE_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONSOLE_READCONSOLE_CONTROL>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONSOLE_READCONSOLE_CONTROL {}
impl ::core::default::Default for CONSOLE_READCONSOLE_CONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: u16,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
}
impl ::core::marker::Copy for CONSOLE_SCREEN_BUFFER_INFO {}
impl ::core::clone::Clone for CONSOLE_SCREEN_BUFFER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_SCREEN_BUFFER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_SCREEN_BUFFER_INFO").field("dwSize", &self.dwSize).field("dwCursorPosition", &self.dwCursorPosition).field("wAttributes", &self.wAttributes).field("srWindow", &self.srWindow).field("dwMaximumWindowSize", &self.dwMaximumWindowSize).finish()
    }
}
unsafe impl ::windows::core::Abi for CONSOLE_SCREEN_BUFFER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONSOLE_SCREEN_BUFFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONSOLE_SCREEN_BUFFER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONSOLE_SCREEN_BUFFER_INFO {}
impl ::core::default::Default for CONSOLE_SCREEN_BUFFER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CONSOLE_SCREEN_BUFFER_INFOEX {
    pub cbSize: u32,
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: u16,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
    pub wPopupAttributes: u16,
    pub bFullscreenSupported: super::super::Foundation::BOOL,
    pub ColorTable: [u32; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONSOLE_SCREEN_BUFFER_INFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_SCREEN_BUFFER_INFOEX").field("cbSize", &self.cbSize).field("dwSize", &self.dwSize).field("dwCursorPosition", &self.dwCursorPosition).field("wAttributes", &self.wAttributes).field("srWindow", &self.srWindow).field("dwMaximumWindowSize", &self.dwMaximumWindowSize).field("wPopupAttributes", &self.wPopupAttributes).field("bFullscreenSupported", &self.bFullscreenSupported).field("ColorTable", &self.ColorTable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CONSOLE_SCREEN_BUFFER_INFOEX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONSOLE_SCREEN_BUFFER_INFOEX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CONSOLE_SCREEN_BUFFER_INFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct CONSOLE_SELECTION_INFO {
    pub dwFlags: u32,
    pub dwSelectionAnchor: COORD,
    pub srSelection: SMALL_RECT,
}
impl ::core::marker::Copy for CONSOLE_SELECTION_INFO {}
impl ::core::clone::Clone for CONSOLE_SELECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_SELECTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_SELECTION_INFO").field("dwFlags", &self.dwFlags).field("dwSelectionAnchor", &self.dwSelectionAnchor).field("srSelection", &self.srSelection).finish()
    }
}
unsafe impl ::windows::core::Abi for CONSOLE_SELECTION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONSOLE_SELECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONSOLE_SELECTION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONSOLE_SELECTION_INFO {}
impl ::core::default::Default for CONSOLE_SELECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_SELECTION_IN_PROGRESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_SELECTION_NOT_EMPTY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_TEXTMODE_BUFFER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_WINDOWED_MODE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct COORD {
    pub X: i16,
    pub Y: i16,
}
impl ::core::marker::Copy for COORD {}
impl ::core::clone::Clone for COORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COORD").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
unsafe impl ::windows::core::Abi for COORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for COORD {}
impl ::core::default::Default for COORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CTRL_BREAK_EVENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CTRL_CLOSE_EVENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CTRL_C_EVENT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CTRL_LOGOFF_EVENT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CTRL_SHUTDOWN_EVENT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn ClosePseudoConsole<'a, P0>(hpc: P0)
where
    P0: ::std::convert::Into<HPCON>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ClosePseudoConsole(hpc: HPCON);
    }
    ClosePseudoConsole(hpc.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateConsoleScreenBuffer(dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwflags: u32, lpscreenbufferdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateConsoleScreenBuffer(dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwflags: u32, lpscreenbufferdata: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
    }
    let result__ = CreateConsoleScreenBuffer(dwdesiredaccess, dwsharemode, ::core::mem::transmute(lpsecurityattributes), dwflags, ::core::mem::transmute(lpscreenbufferdata));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePseudoConsole<'a, P0, P1>(size: COORD, hinput: P0, houtput: P1, dwflags: u32) -> ::windows::core::Result<HPCON>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreatePseudoConsole(size: COORD, hinput: super::super::Foundation::HANDLE, houtput: super::super::Foundation::HANDLE, dwflags: u32, phpc: *mut HPCON) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreatePseudoConsole(::core::mem::transmute(size), hinput.into(), houtput.into(), dwflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<HPCON>(result__)
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const DOUBLE_CLICK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENHANCED_KEY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn ExpungeConsoleCommandHistoryA<'a, P0>(exename: P0)
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ExpungeConsoleCommandHistoryA(exename: ::windows::core::PCSTR);
    }
    ExpungeConsoleCommandHistoryA(exename.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn ExpungeConsoleCommandHistoryW<'a, P0>(exename: P0)
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ExpungeConsoleCommandHistoryW(exename: ::windows::core::PCWSTR);
    }
    ExpungeConsoleCommandHistoryW(exename.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FOCUS_EVENT: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FOCUS_EVENT_RECORD {
    pub bSetFocus: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FOCUS_EVENT_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FOCUS_EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FOCUS_EVENT_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FOCUS_EVENT_RECORD").field("bSetFocus", &self.bSetFocus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FOCUS_EVENT_RECORD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FOCUS_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FOCUS_EVENT_RECORD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FOCUS_EVENT_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FOCUS_EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FOREGROUND_BLUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FOREGROUND_GREEN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FOREGROUND_INTENSITY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FOREGROUND_RED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FROM_LEFT_1ST_BUTTON_PRESSED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FROM_LEFT_2ND_BUTTON_PRESSED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FROM_LEFT_3RD_BUTTON_PRESSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FROM_LEFT_4TH_BUTTON_PRESSED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FillConsoleOutputAttribute<'a, P0>(hconsoleoutput: P0, wattribute: u16, nlength: u32, dwwritecoord: COORD, lpnumberofattrswritten: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FillConsoleOutputAttribute(hconsoleoutput: super::super::Foundation::HANDLE, wattribute: u16, nlength: u32, dwwritecoord: COORD, lpnumberofattrswritten: *mut u32) -> super::super::Foundation::BOOL;
    }
    FillConsoleOutputAttribute(hconsoleoutput.into(), wattribute, nlength, ::core::mem::transmute(dwwritecoord), ::core::mem::transmute(lpnumberofattrswritten))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FillConsoleOutputCharacterA<'a, P0, P1>(hconsoleoutput: P0, ccharacter: P1, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FillConsoleOutputCharacterA(hconsoleoutput: super::super::Foundation::HANDLE, ccharacter: super::super::Foundation::CHAR, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> super::super::Foundation::BOOL;
    }
    FillConsoleOutputCharacterA(hconsoleoutput.into(), ccharacter.into(), nlength, ::core::mem::transmute(dwwritecoord), ::core::mem::transmute(lpnumberofcharswritten))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FillConsoleOutputCharacterW<'a, P0>(hconsoleoutput: P0, ccharacter: u16, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FillConsoleOutputCharacterW(hconsoleoutput: super::super::Foundation::HANDLE, ccharacter: u16, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> super::super::Foundation::BOOL;
    }
    FillConsoleOutputCharacterW(hconsoleoutput.into(), ccharacter, nlength, ::core::mem::transmute(dwwritecoord), ::core::mem::transmute(lpnumberofcharswritten))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushConsoleInputBuffer<'a, P0>(hconsoleinput: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FlushConsoleInputBuffer(hconsoleinput: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    }
    FlushConsoleInputBuffer(hconsoleinput.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeConsole() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FreeConsole() -> super::super::Foundation::BOOL;
    }
    FreeConsole()
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GenerateConsoleCtrlEvent(dwctrlevent: u32, dwprocessgroupid: u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GenerateConsoleCtrlEvent(dwctrlevent: u32, dwprocessgroupid: u32) -> super::super::Foundation::BOOL;
    }
    GenerateConsoleCtrlEvent(dwctrlevent, dwprocessgroupid)
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleAliasA<'a, P0, P1>(source: P0, targetbuffer: &mut [u8], exename: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleAliasA(source: ::windows::core::PCSTR, targetbuffer: ::windows::core::PSTR, targetbufferlength: u32, exename: ::windows::core::PCSTR) -> u32;
    }
    GetConsoleAliasA(source.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(targetbuffer)), targetbuffer.len() as _, exename.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleAliasExesA(exenamebuffer: &mut [u8]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleAliasExesA(exenamebuffer: ::windows::core::PSTR, exenamebufferlength: u32) -> u32;
    }
    GetConsoleAliasExesA(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(exenamebuffer)), exenamebuffer.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleAliasExesLengthA() -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleAliasExesLengthA() -> u32;
    }
    GetConsoleAliasExesLengthA()
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleAliasExesLengthW() -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleAliasExesLengthW() -> u32;
    }
    GetConsoleAliasExesLengthW()
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleAliasExesW(exenamebuffer: &mut [u16]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleAliasExesW(exenamebuffer: ::windows::core::PWSTR, exenamebufferlength: u32) -> u32;
    }
    GetConsoleAliasExesW(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(exenamebuffer)), exenamebuffer.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleAliasW<'a, P0, P1>(source: P0, targetbuffer: &mut [u16], exename: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleAliasW(source: ::windows::core::PCWSTR, targetbuffer: ::windows::core::PWSTR, targetbufferlength: u32, exename: ::windows::core::PCWSTR) -> u32;
    }
    GetConsoleAliasW(source.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(targetbuffer)), targetbuffer.len() as _, exename.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleAliasesA<'a, P0>(aliasbuffer: &mut [u8], exename: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleAliasesA(aliasbuffer: ::windows::core::PSTR, aliasbufferlength: u32, exename: ::windows::core::PCSTR) -> u32;
    }
    GetConsoleAliasesA(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(aliasbuffer)), aliasbuffer.len() as _, exename.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleAliasesLengthA<'a, P0>(exename: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleAliasesLengthA(exename: ::windows::core::PCSTR) -> u32;
    }
    GetConsoleAliasesLengthA(exename.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleAliasesLengthW<'a, P0>(exename: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleAliasesLengthW(exename: ::windows::core::PCWSTR) -> u32;
    }
    GetConsoleAliasesLengthW(exename.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleAliasesW<'a, P0>(aliasbuffer: &mut [u16], exename: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleAliasesW(aliasbuffer: ::windows::core::PWSTR, aliasbufferlength: u32, exename: ::windows::core::PCWSTR) -> u32;
    }
    GetConsoleAliasesW(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(aliasbuffer)), aliasbuffer.len() as _, exename.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleCP() -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleCP() -> u32;
    }
    GetConsoleCP()
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleCommandHistoryA<'a, P0>(commands: ::windows::core::PSTR, commandbufferlength: u32, exename: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleCommandHistoryA(commands: ::windows::core::PSTR, commandbufferlength: u32, exename: ::windows::core::PCSTR) -> u32;
    }
    GetConsoleCommandHistoryA(::core::mem::transmute(commands), commandbufferlength, exename.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleCommandHistoryLengthA<'a, P0>(exename: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleCommandHistoryLengthA(exename: ::windows::core::PCSTR) -> u32;
    }
    GetConsoleCommandHistoryLengthA(exename.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleCommandHistoryLengthW<'a, P0>(exename: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleCommandHistoryLengthW(exename: ::windows::core::PCWSTR) -> u32;
    }
    GetConsoleCommandHistoryLengthW(exename.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleCommandHistoryW<'a, P0>(commands: ::windows::core::PWSTR, commandbufferlength: u32, exename: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleCommandHistoryW(commands: ::windows::core::PWSTR, commandbufferlength: u32, exename: ::windows::core::PCWSTR) -> u32;
    }
    GetConsoleCommandHistoryW(::core::mem::transmute(commands), commandbufferlength, exename.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleCursorInfo<'a, P0>(hconsoleoutput: P0, lpconsolecursorinfo: *mut CONSOLE_CURSOR_INFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleCursorInfo(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolecursorinfo: *mut CONSOLE_CURSOR_INFO) -> super::super::Foundation::BOOL;
    }
    GetConsoleCursorInfo(hconsoleoutput.into(), ::core::mem::transmute(lpconsolecursorinfo))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleDisplayMode(lpmodeflags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleDisplayMode(lpmodeflags: *mut u32) -> super::super::Foundation::BOOL;
    }
    GetConsoleDisplayMode(::core::mem::transmute(lpmodeflags))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleFontSize<'a, P0>(hconsoleoutput: P0, nfont: u32) -> COORD
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleFontSize(hconsoleoutput: super::super::Foundation::HANDLE, nfont: u32) -> COORD;
    }
    GetConsoleFontSize(hconsoleoutput.into(), nfont)
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleHistoryInfo(lpconsolehistoryinfo: *mut CONSOLE_HISTORY_INFO) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleHistoryInfo(lpconsolehistoryinfo: *mut CONSOLE_HISTORY_INFO) -> super::super::Foundation::BOOL;
    }
    GetConsoleHistoryInfo(::core::mem::transmute(lpconsolehistoryinfo))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleMode<'a, P0>(hconsolehandle: P0, lpmode: *mut CONSOLE_MODE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleMode(hconsolehandle: super::super::Foundation::HANDLE, lpmode: *mut CONSOLE_MODE) -> super::super::Foundation::BOOL;
    }
    GetConsoleMode(hconsolehandle.into(), ::core::mem::transmute(lpmode))
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleOriginalTitleA(lpconsoletitle: &mut [u8]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleOriginalTitleA(lpconsoletitle: ::windows::core::PSTR, nsize: u32) -> u32;
    }
    GetConsoleOriginalTitleA(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpconsoletitle)), lpconsoletitle.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleOriginalTitleW(lpconsoletitle: &mut [u16]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleOriginalTitleW(lpconsoletitle: ::windows::core::PWSTR, nsize: u32) -> u32;
    }
    GetConsoleOriginalTitleW(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpconsoletitle)), lpconsoletitle.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleOutputCP() -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleOutputCP() -> u32;
    }
    GetConsoleOutputCP()
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleProcessList(lpdwprocesslist: &mut [u32]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleProcessList(lpdwprocesslist: *mut u32, dwprocesscount: u32) -> u32;
    }
    GetConsoleProcessList(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpdwprocesslist)), lpdwprocesslist.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleScreenBufferInfo<'a, P0>(hconsoleoutput: P0, lpconsolescreenbufferinfo: *mut CONSOLE_SCREEN_BUFFER_INFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleScreenBufferInfo(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolescreenbufferinfo: *mut CONSOLE_SCREEN_BUFFER_INFO) -> super::super::Foundation::BOOL;
    }
    GetConsoleScreenBufferInfo(hconsoleoutput.into(), ::core::mem::transmute(lpconsolescreenbufferinfo))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleScreenBufferInfoEx<'a, P0>(hconsoleoutput: P0, lpconsolescreenbufferinfoex: *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleScreenBufferInfoEx(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolescreenbufferinfoex: *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> super::super::Foundation::BOOL;
    }
    GetConsoleScreenBufferInfoEx(hconsoleoutput.into(), ::core::mem::transmute(lpconsolescreenbufferinfoex))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleSelectionInfo(lpconsoleselectioninfo: *mut CONSOLE_SELECTION_INFO) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleSelectionInfo(lpconsoleselectioninfo: *mut CONSOLE_SELECTION_INFO) -> super::super::Foundation::BOOL;
    }
    GetConsoleSelectionInfo(::core::mem::transmute(lpconsoleselectioninfo))
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleTitleA(lpconsoletitle: &mut [u8]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleTitleA(lpconsoletitle: ::windows::core::PSTR, nsize: u32) -> u32;
    }
    GetConsoleTitleA(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpconsoletitle)), lpconsoletitle.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn GetConsoleTitleW(lpconsoletitle: &mut [u16]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleTitleW(lpconsoletitle: ::windows::core::PWSTR, nsize: u32) -> u32;
    }
    GetConsoleTitleW(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpconsoletitle)), lpconsoletitle.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleWindow() -> super::super::Foundation::HWND {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetConsoleWindow() -> super::super::Foundation::HWND;
    }
    GetConsoleWindow()
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentConsoleFont<'a, P0, P1>(hconsoleoutput: P0, bmaximumwindow: P1, lpconsolecurrentfont: *mut CONSOLE_FONT_INFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCurrentConsoleFont(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: super::super::Foundation::BOOL, lpconsolecurrentfont: *mut CONSOLE_FONT_INFO) -> super::super::Foundation::BOOL;
    }
    GetCurrentConsoleFont(hconsoleoutput.into(), bmaximumwindow.into(), ::core::mem::transmute(lpconsolecurrentfont))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentConsoleFontEx<'a, P0, P1>(hconsoleoutput: P0, bmaximumwindow: P1, lpconsolecurrentfontex: *mut CONSOLE_FONT_INFOEX) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetCurrentConsoleFontEx(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: super::super::Foundation::BOOL, lpconsolecurrentfontex: *mut CONSOLE_FONT_INFOEX) -> super::super::Foundation::BOOL;
    }
    GetCurrentConsoleFontEx(hconsoleoutput.into(), bmaximumwindow.into(), ::core::mem::transmute(lpconsolecurrentfontex))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLargestConsoleWindowSize<'a, P0>(hconsoleoutput: P0) -> COORD
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetLargestConsoleWindowSize(hconsoleoutput: super::super::Foundation::HANDLE) -> COORD;
    }
    GetLargestConsoleWindowSize(hconsoleoutput.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumberOfConsoleInputEvents<'a, P0>(hconsoleinput: P0, lpnumberofevents: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetNumberOfConsoleInputEvents(hconsoleinput: super::super::Foundation::HANDLE, lpnumberofevents: *mut u32) -> super::super::Foundation::BOOL;
    }
    GetNumberOfConsoleInputEvents(hconsoleinput.into(), ::core::mem::transmute(lpnumberofevents))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons: *mut u32) -> super::super::Foundation::BOOL;
    }
    GetNumberOfConsoleMouseButtons(::core::mem::transmute(lpnumberofmousebuttons))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStdHandle(nstdhandle: STD_HANDLE) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetStdHandle(nstdhandle: STD_HANDLE) -> super::super::Foundation::HANDLE;
    }
    let result__ = GetStdHandle(nstdhandle);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const HISTORY_NO_DUP_FLAG: u32 = 1u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HPCON(pub isize);
impl HPCON {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HPCON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HPCON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HPCON {}
impl ::core::fmt::Debug for HPCON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HPCON").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HPCON>> for HPCON {
    fn from(optional: ::core::option::Option<HPCON>) -> HPCON {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HPCON {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INPUT_RECORD {
    pub EventType: u16,
    pub Event: INPUT_RECORD_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INPUT_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INPUT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INPUT_RECORD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INPUT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INPUT_RECORD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INPUT_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INPUT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union INPUT_RECORD_0 {
    pub KeyEvent: KEY_EVENT_RECORD,
    pub MouseEvent: MOUSE_EVENT_RECORD,
    pub WindowBufferSizeEvent: WINDOW_BUFFER_SIZE_RECORD,
    pub MenuEvent: MENU_EVENT_RECORD,
    pub FocusEvent: FOCUS_EVENT_RECORD,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INPUT_RECORD_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INPUT_RECORD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INPUT_RECORD_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INPUT_RECORD_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INPUT_RECORD_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INPUT_RECORD_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INPUT_RECORD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const KEY_EVENT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KEY_EVENT_RECORD {
    pub bKeyDown: super::super::Foundation::BOOL,
    pub wRepeatCount: u16,
    pub wVirtualKeyCode: u16,
    pub wVirtualScanCode: u16,
    pub uChar: KEY_EVENT_RECORD_0,
    pub dwControlKeyState: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KEY_EVENT_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KEY_EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KEY_EVENT_RECORD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KEY_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KEY_EVENT_RECORD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KEY_EVENT_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KEY_EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union KEY_EVENT_RECORD_0 {
    pub UnicodeChar: u16,
    pub AsciiChar: super::super::Foundation::CHAR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KEY_EVENT_RECORD_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KEY_EVENT_RECORD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for KEY_EVENT_RECORD_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KEY_EVENT_RECORD_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KEY_EVENT_RECORD_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KEY_EVENT_RECORD_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KEY_EVENT_RECORD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const LEFT_ALT_PRESSED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const LEFT_CTRL_PRESSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const MENU_EVENT: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct MENU_EVENT_RECORD {
    pub dwCommandId: u32,
}
impl ::core::marker::Copy for MENU_EVENT_RECORD {}
impl ::core::clone::Clone for MENU_EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MENU_EVENT_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENU_EVENT_RECORD").field("dwCommandId", &self.dwCommandId).finish()
    }
}
unsafe impl ::windows::core::Abi for MENU_EVENT_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MENU_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MENU_EVENT_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for MENU_EVENT_RECORD {}
impl ::core::default::Default for MENU_EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const MOUSE_EVENT: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct MOUSE_EVENT_RECORD {
    pub dwMousePosition: COORD,
    pub dwButtonState: u32,
    pub dwControlKeyState: u32,
    pub dwEventFlags: u32,
}
impl ::core::marker::Copy for MOUSE_EVENT_RECORD {}
impl ::core::clone::Clone for MOUSE_EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MOUSE_EVENT_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSE_EVENT_RECORD").field("dwMousePosition", &self.dwMousePosition).field("dwButtonState", &self.dwButtonState).field("dwControlKeyState", &self.dwControlKeyState).field("dwEventFlags", &self.dwEventFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for MOUSE_EVENT_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MOUSE_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MOUSE_EVENT_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for MOUSE_EVENT_RECORD {}
impl ::core::default::Default for MOUSE_EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const MOUSE_HWHEELED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const MOUSE_MOVED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const MOUSE_WHEELED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const NLS_ALPHANUMERIC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const NLS_DBCSCHAR: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const NLS_HIRAGANA: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const NLS_IME_CONVERSION: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const NLS_IME_DISABLE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const NLS_KATAKANA: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const NLS_ROMAN: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const NUMLOCK_ON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PHANDLER_ROUTINE = ::core::option::Option<unsafe extern "system" fn(ctrltype: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const PSEUDOCONSOLE_INHERIT_CURSOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeekConsoleInputA<'a, P0>(hconsoleinput: P0, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PeekConsoleInputA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> super::super::Foundation::BOOL;
    }
    PeekConsoleInputA(hconsoleinput.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpbuffer)), lpbuffer.len() as _, ::core::mem::transmute(lpnumberofeventsread))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PeekConsoleInputW<'a, P0>(hconsoleinput: P0, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PeekConsoleInputW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> super::super::Foundation::BOOL;
    }
    PeekConsoleInputW(hconsoleinput.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpbuffer)), lpbuffer.len() as _, ::core::mem::transmute(lpnumberofeventsread))
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const RIGHTMOST_BUTTON_PRESSED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const RIGHT_ALT_PRESSED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const RIGHT_CTRL_PRESSED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadConsoleA<'a, P0>(hconsoleinput: P0, lpbuffer: *mut ::core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReadConsoleA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL) -> super::super::Foundation::BOOL;
    }
    ReadConsoleA(hconsoleinput.into(), ::core::mem::transmute(lpbuffer), nnumberofcharstoread, ::core::mem::transmute(lpnumberofcharsread), ::core::mem::transmute(pinputcontrol))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadConsoleInputA<'a, P0>(hconsoleinput: P0, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReadConsoleInputA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> super::super::Foundation::BOOL;
    }
    ReadConsoleInputA(hconsoleinput.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpbuffer)), lpbuffer.len() as _, ::core::mem::transmute(lpnumberofeventsread))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadConsoleInputW<'a, P0>(hconsoleinput: P0, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReadConsoleInputW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> super::super::Foundation::BOOL;
    }
    ReadConsoleInputW(hconsoleinput.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpbuffer)), lpbuffer.len() as _, ::core::mem::transmute(lpnumberofeventsread))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadConsoleOutputA<'a, P0>(hconsoleoutput: P0, lpbuffer: *mut CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpreadregion: *mut SMALL_RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReadConsoleOutputA(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *mut CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpreadregion: *mut SMALL_RECT) -> super::super::Foundation::BOOL;
    }
    ReadConsoleOutputA(hconsoleoutput.into(), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbuffersize), ::core::mem::transmute(dwbuffercoord), ::core::mem::transmute(lpreadregion))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadConsoleOutputAttribute<'a, P0>(hconsoleoutput: P0, lpattribute: &mut [u16], dwreadcoord: COORD, lpnumberofattrsread: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReadConsoleOutputAttribute(hconsoleoutput: super::super::Foundation::HANDLE, lpattribute: *mut u16, nlength: u32, dwreadcoord: COORD, lpnumberofattrsread: *mut u32) -> super::super::Foundation::BOOL;
    }
    ReadConsoleOutputAttribute(hconsoleoutput.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpattribute)), lpattribute.len() as _, ::core::mem::transmute(dwreadcoord), ::core::mem::transmute(lpnumberofattrsread))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadConsoleOutputCharacterA<'a, P0>(hconsoleoutput: P0, lpcharacter: &mut [u8], dwreadcoord: COORD, lpnumberofcharsread: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReadConsoleOutputCharacterA(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: ::windows::core::PSTR, nlength: u32, dwreadcoord: COORD, lpnumberofcharsread: *mut u32) -> super::super::Foundation::BOOL;
    }
    ReadConsoleOutputCharacterA(hconsoleoutput.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpcharacter)), lpcharacter.len() as _, ::core::mem::transmute(dwreadcoord), ::core::mem::transmute(lpnumberofcharsread))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadConsoleOutputCharacterW<'a, P0>(hconsoleoutput: P0, lpcharacter: &mut [u16], dwreadcoord: COORD, lpnumberofcharsread: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReadConsoleOutputCharacterW(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: ::windows::core::PWSTR, nlength: u32, dwreadcoord: COORD, lpnumberofcharsread: *mut u32) -> super::super::Foundation::BOOL;
    }
    ReadConsoleOutputCharacterW(hconsoleoutput.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpcharacter)), lpcharacter.len() as _, ::core::mem::transmute(dwreadcoord), ::core::mem::transmute(lpnumberofcharsread))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadConsoleOutputW<'a, P0>(hconsoleoutput: P0, lpbuffer: *mut CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpreadregion: *mut SMALL_RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReadConsoleOutputW(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *mut CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpreadregion: *mut SMALL_RECT) -> super::super::Foundation::BOOL;
    }
    ReadConsoleOutputW(hconsoleoutput.into(), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbuffersize), ::core::mem::transmute(dwbuffercoord), ::core::mem::transmute(lpreadregion))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadConsoleW<'a, P0>(hconsoleinput: P0, lpbuffer: *mut ::core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ReadConsoleW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL) -> super::super::Foundation::BOOL;
    }
    ReadConsoleW(hconsoleinput.into(), ::core::mem::transmute(lpbuffer), nnumberofcharstoread, ::core::mem::transmute(lpnumberofcharsread), ::core::mem::transmute(pinputcontrol))
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[inline]
pub unsafe fn ResizePseudoConsole<'a, P0>(hpc: P0, size: COORD) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HPCON>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ResizePseudoConsole(hpc: HPCON, size: COORD) -> ::windows::core::HRESULT;
    }
    ResizePseudoConsole(hpc.into(), ::core::mem::transmute(size)).ok()
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const SCROLLLOCK_ON: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const SHIFT_PRESSED: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
impl ::core::marker::Copy for SMALL_RECT {}
impl ::core::clone::Clone for SMALL_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SMALL_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMALL_RECT").field("Left", &self.Left).field("Top", &self.Top).field("Right", &self.Right).field("Bottom", &self.Bottom).finish()
    }
}
unsafe impl ::windows::core::Abi for SMALL_RECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SMALL_RECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SMALL_RECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SMALL_RECT {}
impl ::core::default::Default for SMALL_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STD_HANDLE(pub u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const STD_INPUT_HANDLE: STD_HANDLE = STD_HANDLE(4294967286u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const STD_OUTPUT_HANDLE: STD_HANDLE = STD_HANDLE(4294967285u32);
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const STD_ERROR_HANDLE: STD_HANDLE = STD_HANDLE(4294967284u32);
impl ::core::marker::Copy for STD_HANDLE {}
impl ::core::clone::Clone for STD_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STD_HANDLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STD_HANDLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STD_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STD_HANDLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ScrollConsoleScreenBufferA<'a, P0>(hconsoleoutput: P0, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: *const SMALL_RECT, dwdestinationorigin: COORD, lpfill: *const CHAR_INFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScrollConsoleScreenBufferA(hconsoleoutput: super::super::Foundation::HANDLE, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: *const SMALL_RECT, dwdestinationorigin: COORD, lpfill: *const CHAR_INFO) -> super::super::Foundation::BOOL;
    }
    ScrollConsoleScreenBufferA(hconsoleoutput.into(), ::core::mem::transmute(lpscrollrectangle), ::core::mem::transmute(lpcliprectangle), ::core::mem::transmute(dwdestinationorigin), ::core::mem::transmute(lpfill))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ScrollConsoleScreenBufferW<'a, P0>(hconsoleoutput: P0, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: *const SMALL_RECT, dwdestinationorigin: COORD, lpfill: *const CHAR_INFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ScrollConsoleScreenBufferW(hconsoleoutput: super::super::Foundation::HANDLE, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: *const SMALL_RECT, dwdestinationorigin: COORD, lpfill: *const CHAR_INFO) -> super::super::Foundation::BOOL;
    }
    ScrollConsoleScreenBufferW(hconsoleoutput.into(), ::core::mem::transmute(lpscrollrectangle), ::core::mem::transmute(lpcliprectangle), ::core::mem::transmute(dwdestinationorigin), ::core::mem::transmute(lpfill))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleActiveScreenBuffer<'a, P0>(hconsoleoutput: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleActiveScreenBuffer(hconsoleoutput: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    }
    SetConsoleActiveScreenBuffer(hconsoleoutput.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleCP(wcodepageid: u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleCP(wcodepageid: u32) -> super::super::Foundation::BOOL;
    }
    SetConsoleCP(wcodepageid)
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleCtrlHandler<'a, P0>(handlerroutine: PHANDLER_ROUTINE, add: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleCtrlHandler(handlerroutine: *mut ::core::ffi::c_void, add: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    SetConsoleCtrlHandler(::core::mem::transmute(handlerroutine), add.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleCursorInfo<'a, P0>(hconsoleoutput: P0, lpconsolecursorinfo: *const CONSOLE_CURSOR_INFO) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleCursorInfo(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolecursorinfo: *const CONSOLE_CURSOR_INFO) -> super::super::Foundation::BOOL;
    }
    SetConsoleCursorInfo(hconsoleoutput.into(), ::core::mem::transmute(lpconsolecursorinfo))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleCursorPosition<'a, P0>(hconsoleoutput: P0, dwcursorposition: COORD) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleCursorPosition(hconsoleoutput: super::super::Foundation::HANDLE, dwcursorposition: COORD) -> super::super::Foundation::BOOL;
    }
    SetConsoleCursorPosition(hconsoleoutput.into(), ::core::mem::transmute(dwcursorposition))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleDisplayMode<'a, P0>(hconsoleoutput: P0, dwflags: u32, lpnewscreenbufferdimensions: *mut COORD) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleDisplayMode(hconsoleoutput: super::super::Foundation::HANDLE, dwflags: u32, lpnewscreenbufferdimensions: *mut COORD) -> super::super::Foundation::BOOL;
    }
    SetConsoleDisplayMode(hconsoleoutput.into(), dwflags, ::core::mem::transmute(lpnewscreenbufferdimensions))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleHistoryInfo(lpconsolehistoryinfo: *const CONSOLE_HISTORY_INFO) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleHistoryInfo(lpconsolehistoryinfo: *const CONSOLE_HISTORY_INFO) -> super::super::Foundation::BOOL;
    }
    SetConsoleHistoryInfo(::core::mem::transmute(lpconsolehistoryinfo))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleMode<'a, P0>(hconsolehandle: P0, dwmode: CONSOLE_MODE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleMode(hconsolehandle: super::super::Foundation::HANDLE, dwmode: CONSOLE_MODE) -> super::super::Foundation::BOOL;
    }
    SetConsoleMode(hconsolehandle.into(), dwmode)
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleNumberOfCommandsA<'a, P0>(number: u32, exename: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleNumberOfCommandsA(number: u32, exename: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    SetConsoleNumberOfCommandsA(number, exename.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleNumberOfCommandsW<'a, P0>(number: u32, exename: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleNumberOfCommandsW(number: u32, exename: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    SetConsoleNumberOfCommandsW(number, exename.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleOutputCP(wcodepageid: u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleOutputCP(wcodepageid: u32) -> super::super::Foundation::BOOL;
    }
    SetConsoleOutputCP(wcodepageid)
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleScreenBufferInfoEx<'a, P0>(hconsoleoutput: P0, lpconsolescreenbufferinfoex: *const CONSOLE_SCREEN_BUFFER_INFOEX) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleScreenBufferInfoEx(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolescreenbufferinfoex: *const CONSOLE_SCREEN_BUFFER_INFOEX) -> super::super::Foundation::BOOL;
    }
    SetConsoleScreenBufferInfoEx(hconsoleoutput.into(), ::core::mem::transmute(lpconsolescreenbufferinfoex))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleScreenBufferSize<'a, P0>(hconsoleoutput: P0, dwsize: COORD) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleScreenBufferSize(hconsoleoutput: super::super::Foundation::HANDLE, dwsize: COORD) -> super::super::Foundation::BOOL;
    }
    SetConsoleScreenBufferSize(hconsoleoutput.into(), ::core::mem::transmute(dwsize))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleTextAttribute<'a, P0>(hconsoleoutput: P0, wattributes: u16) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleTextAttribute(hconsoleoutput: super::super::Foundation::HANDLE, wattributes: u16) -> super::super::Foundation::BOOL;
    }
    SetConsoleTextAttribute(hconsoleoutput.into(), wattributes)
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleTitleA<'a, P0>(lpconsoletitle: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleTitleA(lpconsoletitle: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    SetConsoleTitleA(lpconsoletitle.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleTitleW<'a, P0>(lpconsoletitle: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleTitleW(lpconsoletitle: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    SetConsoleTitleW(lpconsoletitle.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleWindowInfo<'a, P0, P1>(hconsoleoutput: P0, babsolute: P1, lpconsolewindow: *const SMALL_RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetConsoleWindowInfo(hconsoleoutput: super::super::Foundation::HANDLE, babsolute: super::super::Foundation::BOOL, lpconsolewindow: *const SMALL_RECT) -> super::super::Foundation::BOOL;
    }
    SetConsoleWindowInfo(hconsoleoutput.into(), babsolute.into(), ::core::mem::transmute(lpconsolewindow))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCurrentConsoleFontEx<'a, P0, P1>(hconsoleoutput: P0, bmaximumwindow: P1, lpconsolecurrentfontex: *const CONSOLE_FONT_INFOEX) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetCurrentConsoleFontEx(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: super::super::Foundation::BOOL, lpconsolecurrentfontex: *const CONSOLE_FONT_INFOEX) -> super::super::Foundation::BOOL;
    }
    SetCurrentConsoleFontEx(hconsoleoutput.into(), bmaximumwindow.into(), ::core::mem::transmute(lpconsolecurrentfontex))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetStdHandle<'a, P0>(nstdhandle: STD_HANDLE, hhandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetStdHandle(nstdhandle: STD_HANDLE, hhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    }
    SetStdHandle(nstdhandle, hhandle.into())
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetStdHandleEx<'a, P0>(nstdhandle: STD_HANDLE, hhandle: P0, phprevvalue: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetStdHandleEx(nstdhandle: STD_HANDLE, hhandle: super::super::Foundation::HANDLE, phprevvalue: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    }
    SetStdHandleEx(nstdhandle, hhandle.into(), ::core::mem::transmute(phprevvalue))
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const WINDOW_BUFFER_SIZE_EVENT: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct WINDOW_BUFFER_SIZE_RECORD {
    pub dwSize: COORD,
}
impl ::core::marker::Copy for WINDOW_BUFFER_SIZE_RECORD {}
impl ::core::clone::Clone for WINDOW_BUFFER_SIZE_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINDOW_BUFFER_SIZE_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOW_BUFFER_SIZE_RECORD").field("dwSize", &self.dwSize).finish()
    }
}
unsafe impl ::windows::core::Abi for WINDOW_BUFFER_SIZE_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINDOW_BUFFER_SIZE_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINDOW_BUFFER_SIZE_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINDOW_BUFFER_SIZE_RECORD {}
impl ::core::default::Default for WINDOW_BUFFER_SIZE_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteConsoleA<'a, P0>(hconsoleoutput: P0, lpbuffer: &[u8], lpnumberofcharswritten: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WriteConsoleA(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *const ::core::ffi::c_void, nnumberofcharstowrite: u32, lpnumberofcharswritten: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    WriteConsoleA(hconsoleoutput.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(lpbuffer)), lpbuffer.len() as _, ::core::mem::transmute(lpnumberofcharswritten), ::core::mem::transmute(lpreserved))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteConsoleInputA<'a, P0>(hconsoleinput: P0, lpbuffer: &[INPUT_RECORD], lpnumberofeventswritten: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WriteConsoleInputA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *const INPUT_RECORD, nlength: u32, lpnumberofeventswritten: *mut u32) -> super::super::Foundation::BOOL;
    }
    WriteConsoleInputA(hconsoleinput.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(lpbuffer)), lpbuffer.len() as _, ::core::mem::transmute(lpnumberofeventswritten))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteConsoleInputW<'a, P0>(hconsoleinput: P0, lpbuffer: &[INPUT_RECORD], lpnumberofeventswritten: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WriteConsoleInputW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *const INPUT_RECORD, nlength: u32, lpnumberofeventswritten: *mut u32) -> super::super::Foundation::BOOL;
    }
    WriteConsoleInputW(hconsoleinput.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(lpbuffer)), lpbuffer.len() as _, ::core::mem::transmute(lpnumberofeventswritten))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteConsoleOutputA<'a, P0>(hconsoleoutput: P0, lpbuffer: *const CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpwriteregion: *mut SMALL_RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WriteConsoleOutputA(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *const CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpwriteregion: *mut SMALL_RECT) -> super::super::Foundation::BOOL;
    }
    WriteConsoleOutputA(hconsoleoutput.into(), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbuffersize), ::core::mem::transmute(dwbuffercoord), ::core::mem::transmute(lpwriteregion))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteConsoleOutputAttribute<'a, P0>(hconsoleoutput: P0, lpattribute: &[u16], dwwritecoord: COORD, lpnumberofattrswritten: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WriteConsoleOutputAttribute(hconsoleoutput: super::super::Foundation::HANDLE, lpattribute: *const u16, nlength: u32, dwwritecoord: COORD, lpnumberofattrswritten: *mut u32) -> super::super::Foundation::BOOL;
    }
    WriteConsoleOutputAttribute(hconsoleoutput.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(lpattribute)), lpattribute.len() as _, ::core::mem::transmute(dwwritecoord), ::core::mem::transmute(lpnumberofattrswritten))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteConsoleOutputCharacterA<'a, P0>(hconsoleoutput: P0, lpcharacter: &[u8], dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WriteConsoleOutputCharacterA(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: ::windows::core::PCSTR, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> super::super::Foundation::BOOL;
    }
    WriteConsoleOutputCharacterA(hconsoleoutput.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(lpcharacter)), lpcharacter.len() as _, ::core::mem::transmute(dwwritecoord), ::core::mem::transmute(lpnumberofcharswritten))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteConsoleOutputCharacterW<'a, P0>(hconsoleoutput: P0, lpcharacter: &[u16], dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WriteConsoleOutputCharacterW(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: ::windows::core::PCWSTR, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> super::super::Foundation::BOOL;
    }
    WriteConsoleOutputCharacterW(hconsoleoutput.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(lpcharacter)), lpcharacter.len() as _, ::core::mem::transmute(dwwritecoord), ::core::mem::transmute(lpnumberofcharswritten))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteConsoleOutputW<'a, P0>(hconsoleoutput: P0, lpbuffer: *const CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpwriteregion: *mut SMALL_RECT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WriteConsoleOutputW(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *const CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpwriteregion: *mut SMALL_RECT) -> super::super::Foundation::BOOL;
    }
    WriteConsoleOutputW(hconsoleoutput.into(), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbuffersize), ::core::mem::transmute(dwbuffercoord), ::core::mem::transmute(lpwriteregion))
}
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteConsoleW<'a, P0>(hconsoleoutput: P0, lpbuffer: &[u8], lpnumberofcharswritten: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WriteConsoleW(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *const ::core::ffi::c_void, nnumberofcharstowrite: u32, lpnumberofcharswritten: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    WriteConsoleW(hconsoleoutput.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(lpbuffer)), lpbuffer.len() as _, ::core::mem::transmute(lpnumberofcharswritten), ::core::mem::transmute(lpreserved))
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
