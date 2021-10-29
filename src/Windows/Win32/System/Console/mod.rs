#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const ALTNUMPAD_BIT: u32 = 67108864u32;
pub const ATTACH_PARENT_PROCESS: u32 = 4294967295u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddConsoleAliasA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    source: Param0,
    target: Param1,
    exename: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddConsoleAliasA(
                source: super::super::Foundation::PSTR,
                target: super::super::Foundation::PSTR,
                exename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddConsoleAliasA(
            source.into_param().abi(),
            target.into_param().abi(),
            exename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddConsoleAliasW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    source: Param0,
    target: Param1,
    exename: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddConsoleAliasW(
                source: super::super::Foundation::PWSTR,
                target: super::super::Foundation::PWSTR,
                exename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddConsoleAliasW(
            source.into_param().abi(),
            target.into_param().abi(),
            exename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllocConsole() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AllocConsole() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AllocConsole())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AttachConsole(dwprocessid: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AttachConsole(dwprocessid: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AttachConsole(::std::mem::transmute(dwprocessid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const BACKGROUND_BLUE: u32 = 16u32;
pub const BACKGROUND_GREEN: u32 = 32u32;
pub const BACKGROUND_INTENSITY: u32 = 128u32;
pub const BACKGROUND_RED: u32 = 64u32;
pub const CAPSLOCK_ON: u32 = 128u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct CHAR_INFO {
    pub Char: CHAR_INFO_0,
    pub Attributes: u16,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl CHAR_INFO {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for CHAR_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for CHAR_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for CHAR_INFO {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for CHAR_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub union CHAR_INFO_0 {
    pub UnicodeChar: u16,
    pub AsciiChar: super::SystemServices::CHAR,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl CHAR_INFO_0 {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for CHAR_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for CHAR_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for CHAR_INFO_0 {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for CHAR_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const COMMON_LVB_GRID_HORIZONTAL: u32 = 1024u32;
pub const COMMON_LVB_GRID_LVERTICAL: u32 = 2048u32;
pub const COMMON_LVB_GRID_RVERTICAL: u32 = 4096u32;
pub const COMMON_LVB_LEADING_BYTE: u32 = 256u32;
pub const COMMON_LVB_REVERSE_VIDEO: u32 = 16384u32;
pub const COMMON_LVB_SBCSDBCS: u32 = 768u32;
pub const COMMON_LVB_TRAILING_BYTE: u32 = 512u32;
pub const COMMON_LVB_UNDERSCORE: u32 = 32768u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CONSOLE_CURSOR_INFO {
    pub dwSize: u32,
    pub bVisible: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl CONSOLE_CURSOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CONSOLE_CURSOR_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CONSOLE_CURSOR_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONSOLE_CURSOR_INFO")
            .field("dwSize", &self.dwSize)
            .field("bVisible", &self.bVisible)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CONSOLE_CURSOR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.bVisible == other.bVisible
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CONSOLE_CURSOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CONSOLE_CURSOR_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CONSOLE_FONT_INFO {
    pub nFont: u32,
    pub dwFontSize: COORD,
}
impl CONSOLE_FONT_INFO {}
impl ::std::default::Default for CONSOLE_FONT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CONSOLE_FONT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONSOLE_FONT_INFO")
            .field("nFont", &self.nFont)
            .field("dwFontSize", &self.dwFontSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CONSOLE_FONT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.nFont == other.nFont && self.dwFontSize == other.dwFontSize
    }
}
impl ::std::cmp::Eq for CONSOLE_FONT_INFO {}
unsafe impl ::windows::runtime::Abi for CONSOLE_FONT_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CONSOLE_FONT_INFOEX {
    pub cbSize: u32,
    pub nFont: u32,
    pub dwFontSize: COORD,
    pub FontFamily: u32,
    pub FontWeight: u32,
    pub FaceName: [u16; 32],
}
impl CONSOLE_FONT_INFOEX {}
impl ::std::default::Default for CONSOLE_FONT_INFOEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CONSOLE_FONT_INFOEX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONSOLE_FONT_INFOEX")
            .field("cbSize", &self.cbSize)
            .field("nFont", &self.nFont)
            .field("dwFontSize", &self.dwFontSize)
            .field("FontFamily", &self.FontFamily)
            .field("FontWeight", &self.FontWeight)
            .field("FaceName", &self.FaceName)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CONSOLE_FONT_INFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.nFont == other.nFont
            && self.dwFontSize == other.dwFontSize
            && self.FontFamily == other.FontFamily
            && self.FontWeight == other.FontWeight
            && self.FaceName == other.FaceName
    }
}
impl ::std::cmp::Eq for CONSOLE_FONT_INFOEX {}
unsafe impl ::windows::runtime::Abi for CONSOLE_FONT_INFOEX {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CONSOLE_FULLSCREEN: u32 = 1u32;
pub const CONSOLE_FULLSCREEN_HARDWARE: u32 = 2u32;
pub const CONSOLE_FULLSCREEN_MODE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CONSOLE_HISTORY_INFO {
    pub cbSize: u32,
    pub HistoryBufferSize: u32,
    pub NumberOfHistoryBuffers: u32,
    pub dwFlags: u32,
}
impl CONSOLE_HISTORY_INFO {}
impl ::std::default::Default for CONSOLE_HISTORY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CONSOLE_HISTORY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONSOLE_HISTORY_INFO")
            .field("cbSize", &self.cbSize)
            .field("HistoryBufferSize", &self.HistoryBufferSize)
            .field("NumberOfHistoryBuffers", &self.NumberOfHistoryBuffers)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CONSOLE_HISTORY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.HistoryBufferSize == other.HistoryBufferSize
            && self.NumberOfHistoryBuffers == other.NumberOfHistoryBuffers
            && self.dwFlags == other.dwFlags
    }
}
impl ::std::cmp::Eq for CONSOLE_HISTORY_INFO {}
unsafe impl ::windows::runtime::Abi for CONSOLE_HISTORY_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CONSOLE_MODE(pub u32);
pub const ENABLE_PROCESSED_INPUT: CONSOLE_MODE = CONSOLE_MODE(1u32);
pub const ENABLE_LINE_INPUT: CONSOLE_MODE = CONSOLE_MODE(2u32);
pub const ENABLE_ECHO_INPUT: CONSOLE_MODE = CONSOLE_MODE(4u32);
pub const ENABLE_WINDOW_INPUT: CONSOLE_MODE = CONSOLE_MODE(8u32);
pub const ENABLE_MOUSE_INPUT: CONSOLE_MODE = CONSOLE_MODE(16u32);
pub const ENABLE_INSERT_MODE: CONSOLE_MODE = CONSOLE_MODE(32u32);
pub const ENABLE_QUICK_EDIT_MODE: CONSOLE_MODE = CONSOLE_MODE(64u32);
pub const ENABLE_EXTENDED_FLAGS: CONSOLE_MODE = CONSOLE_MODE(128u32);
pub const ENABLE_AUTO_POSITION: CONSOLE_MODE = CONSOLE_MODE(256u32);
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: CONSOLE_MODE = CONSOLE_MODE(512u32);
pub const ENABLE_PROCESSED_OUTPUT: CONSOLE_MODE = CONSOLE_MODE(1u32);
pub const ENABLE_WRAP_AT_EOL_OUTPUT: CONSOLE_MODE = CONSOLE_MODE(2u32);
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: CONSOLE_MODE = CONSOLE_MODE(4u32);
pub const DISABLE_NEWLINE_AUTO_RETURN: CONSOLE_MODE = CONSOLE_MODE(8u32);
pub const ENABLE_LVB_GRID_WORLDWIDE: CONSOLE_MODE = CONSOLE_MODE(16u32);
impl ::std::convert::From<u32> for CONSOLE_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CONSOLE_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CONSOLE_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CONSOLE_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CONSOLE_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CONSOLE_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CONSOLE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CONSOLE_MOUSE_DOWN: u32 = 8u32;
pub const CONSOLE_MOUSE_SELECTION: u32 = 4u32;
pub const CONSOLE_NO_SELECTION: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CONSOLE_READCONSOLE_CONTROL {
    pub nLength: u32,
    pub nInitialChars: u32,
    pub dwCtrlWakeupMask: u32,
    pub dwControlKeyState: u32,
}
impl CONSOLE_READCONSOLE_CONTROL {}
impl ::std::default::Default for CONSOLE_READCONSOLE_CONTROL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CONSOLE_READCONSOLE_CONTROL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONSOLE_READCONSOLE_CONTROL")
            .field("nLength", &self.nLength)
            .field("nInitialChars", &self.nInitialChars)
            .field("dwCtrlWakeupMask", &self.dwCtrlWakeupMask)
            .field("dwControlKeyState", &self.dwControlKeyState)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CONSOLE_READCONSOLE_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.nLength == other.nLength
            && self.nInitialChars == other.nInitialChars
            && self.dwCtrlWakeupMask == other.dwCtrlWakeupMask
            && self.dwControlKeyState == other.dwControlKeyState
    }
}
impl ::std::cmp::Eq for CONSOLE_READCONSOLE_CONTROL {}
unsafe impl ::windows::runtime::Abi for CONSOLE_READCONSOLE_CONTROL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: u16,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
}
impl CONSOLE_SCREEN_BUFFER_INFO {}
impl ::std::default::Default for CONSOLE_SCREEN_BUFFER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CONSOLE_SCREEN_BUFFER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONSOLE_SCREEN_BUFFER_INFO")
            .field("dwSize", &self.dwSize)
            .field("dwCursorPosition", &self.dwCursorPosition)
            .field("wAttributes", &self.wAttributes)
            .field("srWindow", &self.srWindow)
            .field("dwMaximumWindowSize", &self.dwMaximumWindowSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CONSOLE_SCREEN_BUFFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCursorPosition == other.dwCursorPosition
            && self.wAttributes == other.wAttributes
            && self.srWindow == other.srWindow
            && self.dwMaximumWindowSize == other.dwMaximumWindowSize
    }
}
impl ::std::cmp::Eq for CONSOLE_SCREEN_BUFFER_INFO {}
unsafe impl ::windows::runtime::Abi for CONSOLE_SCREEN_BUFFER_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl CONSOLE_SCREEN_BUFFER_INFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONSOLE_SCREEN_BUFFER_INFOEX")
            .field("cbSize", &self.cbSize)
            .field("dwSize", &self.dwSize)
            .field("dwCursorPosition", &self.dwCursorPosition)
            .field("wAttributes", &self.wAttributes)
            .field("srWindow", &self.srWindow)
            .field("dwMaximumWindowSize", &self.dwMaximumWindowSize)
            .field("wPopupAttributes", &self.wPopupAttributes)
            .field("bFullscreenSupported", &self.bFullscreenSupported)
            .field("ColorTable", &self.ColorTable)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwSize == other.dwSize
            && self.dwCursorPosition == other.dwCursorPosition
            && self.wAttributes == other.wAttributes
            && self.srWindow == other.srWindow
            && self.dwMaximumWindowSize == other.dwMaximumWindowSize
            && self.wPopupAttributes == other.wPopupAttributes
            && self.bFullscreenSupported == other.bFullscreenSupported
            && self.ColorTable == other.ColorTable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CONSOLE_SCREEN_BUFFER_INFOEX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CONSOLE_SCREEN_BUFFER_INFOEX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CONSOLE_SELECTION_INFO {
    pub dwFlags: u32,
    pub dwSelectionAnchor: COORD,
    pub srSelection: SMALL_RECT,
}
impl CONSOLE_SELECTION_INFO {}
impl ::std::default::Default for CONSOLE_SELECTION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CONSOLE_SELECTION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONSOLE_SELECTION_INFO")
            .field("dwFlags", &self.dwFlags)
            .field("dwSelectionAnchor", &self.dwSelectionAnchor)
            .field("srSelection", &self.srSelection)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CONSOLE_SELECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.dwSelectionAnchor == other.dwSelectionAnchor
            && self.srSelection == other.srSelection
    }
}
impl ::std::cmp::Eq for CONSOLE_SELECTION_INFO {}
unsafe impl ::windows::runtime::Abi for CONSOLE_SELECTION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CONSOLE_SELECTION_IN_PROGRESS: u32 = 1u32;
pub const CONSOLE_SELECTION_NOT_EMPTY: u32 = 2u32;
pub const CONSOLE_TEXTMODE_BUFFER: u32 = 1u32;
pub const CONSOLE_WINDOWED_MODE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct COORD {
    pub X: i16,
    pub Y: i16,
}
impl COORD {}
impl ::std::default::Default for COORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COORD")
            .field("X", &self.X)
            .field("Y", &self.Y)
            .finish()
    }
}
impl ::std::cmp::PartialEq for COORD {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::std::cmp::Eq for COORD {}
unsafe impl ::windows::runtime::Abi for COORD {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CTRL_BREAK_EVENT: u32 = 1u32;
pub const CTRL_CLOSE_EVENT: u32 = 2u32;
pub const CTRL_C_EVENT: u32 = 0u32;
pub const CTRL_LOGOFF_EVENT: u32 = 5u32;
pub const CTRL_SHUTDOWN_EVENT: u32 = 6u32;
#[inline]
pub unsafe fn ClosePseudoConsole<'a, Param0: ::windows::runtime::IntoParam<'a, HPCON>>(
    hpc: Param0,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClosePseudoConsole(hpc: HPCON);
        }
        ::std::mem::transmute(ClosePseudoConsole(hpc.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateConsoleScreenBuffer(
    dwdesiredaccess: u32,
    dwsharemode: u32,
    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    dwflags: u32,
    lpscreenbufferdata: *mut ::std::ffi::c_void,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateConsoleScreenBuffer(
                dwdesiredaccess: u32,
                dwsharemode: u32,
                lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                dwflags: u32,
                lpscreenbufferdata: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateConsoleScreenBuffer(
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(lpsecurityattributes),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpscreenbufferdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreatePseudoConsole<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, COORD>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    size: Param0,
    hinput: Param1,
    houtput: Param2,
    dwflags: u32,
) -> ::windows::runtime::Result<HPCON> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePseudoConsole(
                size: COORD,
                hinput: super::super::Foundation::HANDLE,
                houtput: super::super::Foundation::HANDLE,
                dwflags: u32,
                phpc: *mut HPCON,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HPCON as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreatePseudoConsole(
            size.into_param().abi(),
            hinput.into_param().abi(),
            houtput.into_param().abi(),
            ::std::mem::transmute(dwflags),
            &mut result__,
        )
        .from_abi::<HPCON>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DOUBLE_CLICK: u32 = 2u32;
pub const ENHANCED_KEY: u32 = 256u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExpungeConsoleCommandHistoryA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    exename: Param0,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExpungeConsoleCommandHistoryA(exename: super::super::Foundation::PSTR);
        }
        ::std::mem::transmute(ExpungeConsoleCommandHistoryA(exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExpungeConsoleCommandHistoryW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    exename: Param0,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExpungeConsoleCommandHistoryW(exename: super::super::Foundation::PWSTR);
        }
        ::std::mem::transmute(ExpungeConsoleCommandHistoryW(exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FOCUS_EVENT: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FOCUS_EVENT_RECORD {
    pub bSetFocus: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl FOCUS_EVENT_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FOCUS_EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FOCUS_EVENT_RECORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FOCUS_EVENT_RECORD")
            .field("bSetFocus", &self.bSetFocus)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FOCUS_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.bSetFocus == other.bSetFocus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FOCUS_EVENT_RECORD {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FOCUS_EVENT_RECORD {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FOREGROUND_BLUE: u32 = 1u32;
pub const FOREGROUND_GREEN: u32 = 2u32;
pub const FOREGROUND_INTENSITY: u32 = 8u32;
pub const FOREGROUND_RED: u32 = 4u32;
pub const FROM_LEFT_1ST_BUTTON_PRESSED: u32 = 1u32;
pub const FROM_LEFT_2ND_BUTTON_PRESSED: u32 = 4u32;
pub const FROM_LEFT_3RD_BUTTON_PRESSED: u32 = 8u32;
pub const FROM_LEFT_4TH_BUTTON_PRESSED: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FillConsoleOutputAttribute<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    wattribute: u16,
    nlength: u32,
    dwwritecoord: Param3,
    lpnumberofattrswritten: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FillConsoleOutputAttribute(
                hconsoleoutput: super::super::Foundation::HANDLE,
                wattribute: u16,
                nlength: u32,
                dwwritecoord: COORD,
                lpnumberofattrswritten: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FillConsoleOutputAttribute(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(wattribute),
            ::std::mem::transmute(nlength),
            dwwritecoord.into_param().abi(),
            ::std::mem::transmute(lpnumberofattrswritten),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn FillConsoleOutputCharacterA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::SystemServices::CHAR>,
    Param3: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    ccharacter: Param1,
    nlength: u32,
    dwwritecoord: Param3,
    lpnumberofcharswritten: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FillConsoleOutputCharacterA(
                hconsoleoutput: super::super::Foundation::HANDLE,
                ccharacter: super::SystemServices::CHAR,
                nlength: u32,
                dwwritecoord: COORD,
                lpnumberofcharswritten: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FillConsoleOutputCharacterA(
            hconsoleoutput.into_param().abi(),
            ccharacter.into_param().abi(),
            ::std::mem::transmute(nlength),
            dwwritecoord.into_param().abi(),
            ::std::mem::transmute(lpnumberofcharswritten),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FillConsoleOutputCharacterW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    ccharacter: u16,
    nlength: u32,
    dwwritecoord: Param3,
    lpnumberofcharswritten: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FillConsoleOutputCharacterW(
                hconsoleoutput: super::super::Foundation::HANDLE,
                ccharacter: u16,
                nlength: u32,
                dwwritecoord: COORD,
                lpnumberofcharswritten: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FillConsoleOutputCharacterW(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(ccharacter),
            ::std::mem::transmute(nlength),
            dwwritecoord.into_param().abi(),
            ::std::mem::transmute(lpnumberofcharswritten),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushConsoleInputBuffer<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleinput: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlushConsoleInputBuffer(
                hconsoleinput: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FlushConsoleInputBuffer(hconsoleinput.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeConsole() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeConsole() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FreeConsole())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GenerateConsoleCtrlEvent(
    dwctrlevent: u32,
    dwprocessgroupid: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GenerateConsoleCtrlEvent(
                dwctrlevent: u32,
                dwprocessgroupid: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GenerateConsoleCtrlEvent(
            ::std::mem::transmute(dwctrlevent),
            ::std::mem::transmute(dwprocessgroupid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleAliasA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    source: Param0,
    targetbuffer: super::super::Foundation::PSTR,
    targetbufferlength: u32,
    exename: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasA(
                source: super::super::Foundation::PSTR,
                targetbuffer: super::super::Foundation::PSTR,
                targetbufferlength: u32,
                exename: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetConsoleAliasA(
            source.into_param().abi(),
            ::std::mem::transmute(targetbuffer),
            ::std::mem::transmute(targetbufferlength),
            exename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleAliasExesA(
    exenamebuffer: super::super::Foundation::PSTR,
    exenamebufferlength: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasExesA(
                exenamebuffer: super::super::Foundation::PSTR,
                exenamebufferlength: u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetConsoleAliasExesA(
            ::std::mem::transmute(exenamebuffer),
            ::std::mem::transmute(exenamebufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleAliasExesLengthA() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasExesLengthA() -> u32;
        }
        ::std::mem::transmute(GetConsoleAliasExesLengthA())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleAliasExesLengthW() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasExesLengthW() -> u32;
        }
        ::std::mem::transmute(GetConsoleAliasExesLengthW())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleAliasExesW(
    exenamebuffer: super::super::Foundation::PWSTR,
    exenamebufferlength: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasExesW(
                exenamebuffer: super::super::Foundation::PWSTR,
                exenamebufferlength: u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetConsoleAliasExesW(
            ::std::mem::transmute(exenamebuffer),
            ::std::mem::transmute(exenamebufferlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleAliasW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    source: Param0,
    targetbuffer: super::super::Foundation::PWSTR,
    targetbufferlength: u32,
    exename: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasW(
                source: super::super::Foundation::PWSTR,
                targetbuffer: super::super::Foundation::PWSTR,
                targetbufferlength: u32,
                exename: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetConsoleAliasW(
            source.into_param().abi(),
            ::std::mem::transmute(targetbuffer),
            ::std::mem::transmute(targetbufferlength),
            exename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleAliasesA<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    aliasbuffer: super::super::Foundation::PSTR,
    aliasbufferlength: u32,
    exename: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasesA(
                aliasbuffer: super::super::Foundation::PSTR,
                aliasbufferlength: u32,
                exename: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetConsoleAliasesA(
            ::std::mem::transmute(aliasbuffer),
            ::std::mem::transmute(aliasbufferlength),
            exename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleAliasesLengthA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    exename: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasesLengthA(exename: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(GetConsoleAliasesLengthA(exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleAliasesLengthW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    exename: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasesLengthW(exename: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(GetConsoleAliasesLengthW(exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleAliasesW<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    aliasbuffer: super::super::Foundation::PWSTR,
    aliasbufferlength: u32,
    exename: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasesW(
                aliasbuffer: super::super::Foundation::PWSTR,
                aliasbufferlength: u32,
                exename: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetConsoleAliasesW(
            ::std::mem::transmute(aliasbuffer),
            ::std::mem::transmute(aliasbufferlength),
            exename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleCP() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleCP() -> u32;
        }
        ::std::mem::transmute(GetConsoleCP())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleCommandHistoryA<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    commands: super::super::Foundation::PSTR,
    commandbufferlength: u32,
    exename: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleCommandHistoryA(
                commands: super::super::Foundation::PSTR,
                commandbufferlength: u32,
                exename: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetConsoleCommandHistoryA(
            ::std::mem::transmute(commands),
            ::std::mem::transmute(commandbufferlength),
            exename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleCommandHistoryLengthA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    exename: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleCommandHistoryLengthA(exename: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(GetConsoleCommandHistoryLengthA(exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleCommandHistoryLengthW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    exename: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleCommandHistoryLengthW(exename: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(GetConsoleCommandHistoryLengthW(exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleCommandHistoryW<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    commands: super::super::Foundation::PWSTR,
    commandbufferlength: u32,
    exename: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleCommandHistoryW(
                commands: super::super::Foundation::PWSTR,
                commandbufferlength: u32,
                exename: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetConsoleCommandHistoryW(
            ::std::mem::transmute(commands),
            ::std::mem::transmute(commandbufferlength),
            exename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleCursorInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleoutput: Param0,
    lpconsolecursorinfo: *mut CONSOLE_CURSOR_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleCursorInfo(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpconsolecursorinfo: *mut CONSOLE_CURSOR_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetConsoleCursorInfo(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpconsolecursorinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleDisplayMode(lpmodeflags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleDisplayMode(lpmodeflags: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetConsoleDisplayMode(::std::mem::transmute(lpmodeflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleFontSize<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleoutput: Param0,
    nfont: u32,
) -> COORD {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleFontSize(
                hconsoleoutput: super::super::Foundation::HANDLE,
                nfont: u32,
            ) -> COORD;
        }
        ::std::mem::transmute(GetConsoleFontSize(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(nfont),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleHistoryInfo(
    lpconsolehistoryinfo: *mut CONSOLE_HISTORY_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleHistoryInfo(
                lpconsolehistoryinfo: *mut CONSOLE_HISTORY_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetConsoleHistoryInfo(::std::mem::transmute(
            lpconsolehistoryinfo,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleMode<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsolehandle: Param0,
    lpmode: *mut CONSOLE_MODE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleMode(
                hconsolehandle: super::super::Foundation::HANDLE,
                lpmode: *mut CONSOLE_MODE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetConsoleMode(
            hconsolehandle.into_param().abi(),
            ::std::mem::transmute(lpmode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleOriginalTitleA(
    lpconsoletitle: super::super::Foundation::PSTR,
    nsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleOriginalTitleA(
                lpconsoletitle: super::super::Foundation::PSTR,
                nsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetConsoleOriginalTitleA(
            ::std::mem::transmute(lpconsoletitle),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleOriginalTitleW(
    lpconsoletitle: super::super::Foundation::PWSTR,
    nsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleOriginalTitleW(
                lpconsoletitle: super::super::Foundation::PWSTR,
                nsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetConsoleOriginalTitleW(
            ::std::mem::transmute(lpconsoletitle),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleOutputCP() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleOutputCP() -> u32;
        }
        ::std::mem::transmute(GetConsoleOutputCP())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleProcessList(lpdwprocesslist: *mut u32, dwprocesscount: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleProcessList(lpdwprocesslist: *mut u32, dwprocesscount: u32) -> u32;
        }
        ::std::mem::transmute(GetConsoleProcessList(
            ::std::mem::transmute(lpdwprocesslist),
            ::std::mem::transmute(dwprocesscount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleScreenBufferInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleoutput: Param0,
    lpconsolescreenbufferinfo: *mut CONSOLE_SCREEN_BUFFER_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleScreenBufferInfo(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpconsolescreenbufferinfo: *mut CONSOLE_SCREEN_BUFFER_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetConsoleScreenBufferInfo(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpconsolescreenbufferinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleScreenBufferInfoEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleoutput: Param0,
    lpconsolescreenbufferinfoex: *mut CONSOLE_SCREEN_BUFFER_INFOEX,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleScreenBufferInfoEx(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpconsolescreenbufferinfoex: *mut CONSOLE_SCREEN_BUFFER_INFOEX,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetConsoleScreenBufferInfoEx(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpconsolescreenbufferinfoex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleSelectionInfo(
    lpconsoleselectioninfo: *mut CONSOLE_SELECTION_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleSelectionInfo(
                lpconsoleselectioninfo: *mut CONSOLE_SELECTION_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetConsoleSelectionInfo(::std::mem::transmute(
            lpconsoleselectioninfo,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleTitleA(lpconsoletitle: super::super::Foundation::PSTR, nsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleTitleA(lpconsoletitle: super::super::Foundation::PSTR, nsize: u32) -> u32;
        }
        ::std::mem::transmute(GetConsoleTitleA(
            ::std::mem::transmute(lpconsoletitle),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleTitleW(lpconsoletitle: super::super::Foundation::PWSTR, nsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleTitleW(lpconsoletitle: super::super::Foundation::PWSTR, nsize: u32)
                -> u32;
        }
        ::std::mem::transmute(GetConsoleTitleW(
            ::std::mem::transmute(lpconsoletitle),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetConsoleWindow() -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleWindow() -> super::super::Foundation::HWND;
        }
        ::std::mem::transmute(GetConsoleWindow())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentConsoleFont<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hconsoleoutput: Param0,
    bmaximumwindow: Param1,
    lpconsolecurrentfont: *mut CONSOLE_FONT_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentConsoleFont(
                hconsoleoutput: super::super::Foundation::HANDLE,
                bmaximumwindow: super::super::Foundation::BOOL,
                lpconsolecurrentfont: *mut CONSOLE_FONT_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCurrentConsoleFont(
            hconsoleoutput.into_param().abi(),
            bmaximumwindow.into_param().abi(),
            ::std::mem::transmute(lpconsolecurrentfont),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentConsoleFontEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hconsoleoutput: Param0,
    bmaximumwindow: Param1,
    lpconsolecurrentfontex: *mut CONSOLE_FONT_INFOEX,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentConsoleFontEx(
                hconsoleoutput: super::super::Foundation::HANDLE,
                bmaximumwindow: super::super::Foundation::BOOL,
                lpconsolecurrentfontex: *mut CONSOLE_FONT_INFOEX,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCurrentConsoleFontEx(
            hconsoleoutput.into_param().abi(),
            bmaximumwindow.into_param().abi(),
            ::std::mem::transmute(lpconsolecurrentfontex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLargestConsoleWindowSize<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleoutput: Param0,
) -> COORD {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLargestConsoleWindowSize(
                hconsoleoutput: super::super::Foundation::HANDLE,
            ) -> COORD;
        }
        ::std::mem::transmute(GetLargestConsoleWindowSize(
            hconsoleoutput.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumberOfConsoleInputEvents<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleinput: Param0,
    lpnumberofevents: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumberOfConsoleInputEvents(
                hconsoleinput: super::super::Foundation::HANDLE,
                lpnumberofevents: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNumberOfConsoleInputEvents(
            hconsoleinput.into_param().abi(),
            ::std::mem::transmute(lpnumberofevents),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumberOfConsoleMouseButtons(
    lpnumberofmousebuttons: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumberOfConsoleMouseButtons(
                lpnumberofmousebuttons: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNumberOfConsoleMouseButtons(::std::mem::transmute(
            lpnumberofmousebuttons,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStdHandle(nstdhandle: STD_HANDLE) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStdHandle(nstdhandle: STD_HANDLE) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(GetStdHandle(::std::mem::transmute(nstdhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const HISTORY_NO_DUP_FLAG: u32 = 1u32;
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HPCON(pub isize);
impl ::std::default::Default for HPCON {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HPCON {}
unsafe impl ::windows::runtime::Abi for HPCON {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct INPUT_RECORD {
    pub EventType: u16,
    pub Event: INPUT_RECORD_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl INPUT_RECORD {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for INPUT_RECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for INPUT_RECORD {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for INPUT_RECORD {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for INPUT_RECORD {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub union INPUT_RECORD_0 {
    pub KeyEvent: KEY_EVENT_RECORD,
    pub MouseEvent: MOUSE_EVENT_RECORD,
    pub WindowBufferSizeEvent: WINDOW_BUFFER_SIZE_RECORD,
    pub MenuEvent: MENU_EVENT_RECORD,
    pub FocusEvent: FOCUS_EVENT_RECORD,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl INPUT_RECORD_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for INPUT_RECORD_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for INPUT_RECORD_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for INPUT_RECORD_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for INPUT_RECORD_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const KEY_EVENT: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct KEY_EVENT_RECORD {
    pub bKeyDown: super::super::Foundation::BOOL,
    pub wRepeatCount: u16,
    pub wVirtualKeyCode: u16,
    pub wVirtualScanCode: u16,
    pub uChar: KEY_EVENT_RECORD_0,
    pub dwControlKeyState: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl KEY_EVENT_RECORD {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for KEY_EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for KEY_EVENT_RECORD {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for KEY_EVENT_RECORD {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for KEY_EVENT_RECORD {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub union KEY_EVENT_RECORD_0 {
    pub UnicodeChar: u16,
    pub AsciiChar: super::SystemServices::CHAR,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl KEY_EVENT_RECORD_0 {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for KEY_EVENT_RECORD_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for KEY_EVENT_RECORD_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for KEY_EVENT_RECORD_0 {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for KEY_EVENT_RECORD_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const LEFT_ALT_PRESSED: u32 = 2u32;
pub const LEFT_CTRL_PRESSED: u32 = 8u32;
pub const MENU_EVENT: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MENU_EVENT_RECORD {
    pub dwCommandId: u32,
}
impl MENU_EVENT_RECORD {}
impl ::std::default::Default for MENU_EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MENU_EVENT_RECORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MENU_EVENT_RECORD")
            .field("dwCommandId", &self.dwCommandId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MENU_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwCommandId == other.dwCommandId
    }
}
impl ::std::cmp::Eq for MENU_EVENT_RECORD {}
unsafe impl ::windows::runtime::Abi for MENU_EVENT_RECORD {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MOUSE_EVENT: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MOUSE_EVENT_RECORD {
    pub dwMousePosition: COORD,
    pub dwButtonState: u32,
    pub dwControlKeyState: u32,
    pub dwEventFlags: u32,
}
impl MOUSE_EVENT_RECORD {}
impl ::std::default::Default for MOUSE_EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MOUSE_EVENT_RECORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MOUSE_EVENT_RECORD")
            .field("dwMousePosition", &self.dwMousePosition)
            .field("dwButtonState", &self.dwButtonState)
            .field("dwControlKeyState", &self.dwControlKeyState)
            .field("dwEventFlags", &self.dwEventFlags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MOUSE_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwMousePosition == other.dwMousePosition
            && self.dwButtonState == other.dwButtonState
            && self.dwControlKeyState == other.dwControlKeyState
            && self.dwEventFlags == other.dwEventFlags
    }
}
impl ::std::cmp::Eq for MOUSE_EVENT_RECORD {}
unsafe impl ::windows::runtime::Abi for MOUSE_EVENT_RECORD {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MOUSE_HWHEELED: u32 = 8u32;
pub const MOUSE_MOVED: u32 = 1u32;
pub const MOUSE_WHEELED: u32 = 4u32;
pub const NLS_ALPHANUMERIC: u32 = 0u32;
pub const NLS_DBCSCHAR: u32 = 65536u32;
pub const NLS_HIRAGANA: u32 = 262144u32;
pub const NLS_IME_CONVERSION: u32 = 8388608u32;
pub const NLS_IME_DISABLE: u32 = 536870912u32;
pub const NLS_KATAKANA: u32 = 131072u32;
pub const NLS_ROMAN: u32 = 4194304u32;
pub const NUMLOCK_ON: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
pub type PHANDLER_ROUTINE =
    unsafe extern "system" fn(ctrltype: u32) -> super::super::Foundation::BOOL;
pub const PSEUDOCONSOLE_INHERIT_CURSOR: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn PeekConsoleInputA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleinput: Param0,
    lpbuffer: *mut INPUT_RECORD,
    nlength: u32,
    lpnumberofeventsread: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeekConsoleInputA(
                hconsoleinput: super::super::Foundation::HANDLE,
                lpbuffer: *mut INPUT_RECORD,
                nlength: u32,
                lpnumberofeventsread: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PeekConsoleInputA(
            hconsoleinput.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nlength),
            ::std::mem::transmute(lpnumberofeventsread),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn PeekConsoleInputW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleinput: Param0,
    lpbuffer: *mut INPUT_RECORD,
    nlength: u32,
    lpnumberofeventsread: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeekConsoleInputW(
                hconsoleinput: super::super::Foundation::HANDLE,
                lpbuffer: *mut INPUT_RECORD,
                nlength: u32,
                lpnumberofeventsread: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PeekConsoleInputW(
            hconsoleinput.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nlength),
            ::std::mem::transmute(lpnumberofeventsread),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const RIGHTMOST_BUTTON_PRESSED: u32 = 2u32;
pub const RIGHT_ALT_PRESSED: u32 = 1u32;
pub const RIGHT_CTRL_PRESSED: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadConsoleA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleinput: Param0,
    lpbuffer: *mut ::std::ffi::c_void,
    nnumberofcharstoread: u32,
    lpnumberofcharsread: *mut u32,
    pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleA(
                hconsoleinput: super::super::Foundation::HANDLE,
                lpbuffer: *mut ::std::ffi::c_void,
                nnumberofcharstoread: u32,
                lpnumberofcharsread: *mut u32,
                pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadConsoleA(
            hconsoleinput.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nnumberofcharstoread),
            ::std::mem::transmute(lpnumberofcharsread),
            ::std::mem::transmute(pinputcontrol),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn ReadConsoleInputA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleinput: Param0,
    lpbuffer: *mut INPUT_RECORD,
    nlength: u32,
    lpnumberofeventsread: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleInputA(
                hconsoleinput: super::super::Foundation::HANDLE,
                lpbuffer: *mut INPUT_RECORD,
                nlength: u32,
                lpnumberofeventsread: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadConsoleInputA(
            hconsoleinput.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nlength),
            ::std::mem::transmute(lpnumberofeventsread),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn ReadConsoleInputW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleinput: Param0,
    lpbuffer: *mut INPUT_RECORD,
    nlength: u32,
    lpnumberofeventsread: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleInputW(
                hconsoleinput: super::super::Foundation::HANDLE,
                lpbuffer: *mut INPUT_RECORD,
                nlength: u32,
                lpnumberofeventsread: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadConsoleInputW(
            hconsoleinput.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nlength),
            ::std::mem::transmute(lpnumberofeventsread),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn ReadConsoleOutputA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, COORD>,
    Param3: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    lpbuffer: *mut CHAR_INFO,
    dwbuffersize: Param2,
    dwbuffercoord: Param3,
    lpreadregion: *mut SMALL_RECT,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleOutputA(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpbuffer: *mut CHAR_INFO,
                dwbuffersize: COORD,
                dwbuffercoord: COORD,
                lpreadregion: *mut SMALL_RECT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadConsoleOutputA(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            dwbuffersize.into_param().abi(),
            dwbuffercoord.into_param().abi(),
            ::std::mem::transmute(lpreadregion),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadConsoleOutputAttribute<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    lpattribute: *mut u16,
    nlength: u32,
    dwreadcoord: Param3,
    lpnumberofattrsread: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleOutputAttribute(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpattribute: *mut u16,
                nlength: u32,
                dwreadcoord: COORD,
                lpnumberofattrsread: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadConsoleOutputAttribute(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpattribute),
            ::std::mem::transmute(nlength),
            dwreadcoord.into_param().abi(),
            ::std::mem::transmute(lpnumberofattrsread),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadConsoleOutputCharacterA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    lpcharacter: super::super::Foundation::PSTR,
    nlength: u32,
    dwreadcoord: Param3,
    lpnumberofcharsread: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleOutputCharacterA(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpcharacter: super::super::Foundation::PSTR,
                nlength: u32,
                dwreadcoord: COORD,
                lpnumberofcharsread: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadConsoleOutputCharacterA(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpcharacter),
            ::std::mem::transmute(nlength),
            dwreadcoord.into_param().abi(),
            ::std::mem::transmute(lpnumberofcharsread),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadConsoleOutputCharacterW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    lpcharacter: super::super::Foundation::PWSTR,
    nlength: u32,
    dwreadcoord: Param3,
    lpnumberofcharsread: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleOutputCharacterW(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpcharacter: super::super::Foundation::PWSTR,
                nlength: u32,
                dwreadcoord: COORD,
                lpnumberofcharsread: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadConsoleOutputCharacterW(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpcharacter),
            ::std::mem::transmute(nlength),
            dwreadcoord.into_param().abi(),
            ::std::mem::transmute(lpnumberofcharsread),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn ReadConsoleOutputW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, COORD>,
    Param3: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    lpbuffer: *mut CHAR_INFO,
    dwbuffersize: Param2,
    dwbuffercoord: Param3,
    lpreadregion: *mut SMALL_RECT,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleOutputW(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpbuffer: *mut CHAR_INFO,
                dwbuffersize: COORD,
                dwbuffercoord: COORD,
                lpreadregion: *mut SMALL_RECT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadConsoleOutputW(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            dwbuffersize.into_param().abi(),
            dwbuffercoord.into_param().abi(),
            ::std::mem::transmute(lpreadregion),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadConsoleW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleinput: Param0,
    lpbuffer: *mut ::std::ffi::c_void,
    nnumberofcharstoread: u32,
    lpnumberofcharsread: *mut u32,
    pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleW(
                hconsoleinput: super::super::Foundation::HANDLE,
                lpbuffer: *mut ::std::ffi::c_void,
                nnumberofcharstoread: u32,
                lpnumberofcharsread: *mut u32,
                pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReadConsoleW(
            hconsoleinput.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nnumberofcharstoread),
            ::std::mem::transmute(lpnumberofcharsread),
            ::std::mem::transmute(pinputcontrol),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn ResizePseudoConsole<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HPCON>,
    Param1: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hpc: Param0,
    size: Param1,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResizePseudoConsole(hpc: HPCON, size: COORD) -> ::windows::runtime::HRESULT;
        }
        ResizePseudoConsole(hpc.into_param().abi(), size.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SCROLLLOCK_ON: u32 = 64u32;
pub const SHIFT_PRESSED: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
impl SMALL_RECT {}
impl ::std::default::Default for SMALL_RECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SMALL_RECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SMALL_RECT")
            .field("Left", &self.Left)
            .field("Top", &self.Top)
            .field("Right", &self.Right)
            .field("Bottom", &self.Bottom)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SMALL_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.Left == other.Left
            && self.Top == other.Top
            && self.Right == other.Right
            && self.Bottom == other.Bottom
    }
}
impl ::std::cmp::Eq for SMALL_RECT {}
unsafe impl ::windows::runtime::Abi for SMALL_RECT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct STD_HANDLE(pub u32);
pub const STD_INPUT_HANDLE: STD_HANDLE = STD_HANDLE(4294967286u32);
pub const STD_OUTPUT_HANDLE: STD_HANDLE = STD_HANDLE(4294967285u32);
pub const STD_ERROR_HANDLE: STD_HANDLE = STD_HANDLE(4294967284u32);
impl ::std::convert::From<u32> for STD_HANDLE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STD_HANDLE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for STD_HANDLE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for STD_HANDLE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for STD_HANDLE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for STD_HANDLE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for STD_HANDLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn ScrollConsoleScreenBufferA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    lpscrollrectangle: *const SMALL_RECT,
    lpcliprectangle: *const SMALL_RECT,
    dwdestinationorigin: Param3,
    lpfill: *const CHAR_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ScrollConsoleScreenBufferA(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpscrollrectangle: *const SMALL_RECT,
                lpcliprectangle: *const SMALL_RECT,
                dwdestinationorigin: COORD,
                lpfill: *const CHAR_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ScrollConsoleScreenBufferA(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpscrollrectangle),
            ::std::mem::transmute(lpcliprectangle),
            dwdestinationorigin.into_param().abi(),
            ::std::mem::transmute(lpfill),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn ScrollConsoleScreenBufferW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    lpscrollrectangle: *const SMALL_RECT,
    lpcliprectangle: *const SMALL_RECT,
    dwdestinationorigin: Param3,
    lpfill: *const CHAR_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ScrollConsoleScreenBufferW(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpscrollrectangle: *const SMALL_RECT,
                lpcliprectangle: *const SMALL_RECT,
                dwdestinationorigin: COORD,
                lpfill: *const CHAR_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ScrollConsoleScreenBufferW(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpscrollrectangle),
            ::std::mem::transmute(lpcliprectangle),
            dwdestinationorigin.into_param().abi(),
            ::std::mem::transmute(lpfill),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleActiveScreenBuffer<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleoutput: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleActiveScreenBuffer(
                hconsoleoutput: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleActiveScreenBuffer(
            hconsoleoutput.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleCP(wcodepageid: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleCP(wcodepageid: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleCP(::std::mem::transmute(wcodepageid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleCtrlHandler<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    handlerroutine: ::std::option::Option<PHANDLER_ROUTINE>,
    add: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleCtrlHandler(
                handlerroutine: ::windows::runtime::RawPtr,
                add: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleCtrlHandler(
            ::std::mem::transmute(handlerroutine),
            add.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleCursorInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleoutput: Param0,
    lpconsolecursorinfo: *const CONSOLE_CURSOR_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleCursorInfo(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpconsolecursorinfo: *const CONSOLE_CURSOR_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleCursorInfo(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpconsolecursorinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleCursorPosition<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    dwcursorposition: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleCursorPosition(
                hconsoleoutput: super::super::Foundation::HANDLE,
                dwcursorposition: COORD,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleCursorPosition(
            hconsoleoutput.into_param().abi(),
            dwcursorposition.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleDisplayMode<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleoutput: Param0,
    dwflags: u32,
    lpnewscreenbufferdimensions: *mut COORD,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleDisplayMode(
                hconsoleoutput: super::super::Foundation::HANDLE,
                dwflags: u32,
                lpnewscreenbufferdimensions: *mut COORD,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleDisplayMode(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpnewscreenbufferdimensions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleHistoryInfo(
    lpconsolehistoryinfo: *const CONSOLE_HISTORY_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleHistoryInfo(
                lpconsolehistoryinfo: *const CONSOLE_HISTORY_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleHistoryInfo(::std::mem::transmute(
            lpconsolehistoryinfo,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleMode<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsolehandle: Param0,
    dwmode: CONSOLE_MODE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleMode(
                hconsolehandle: super::super::Foundation::HANDLE,
                dwmode: CONSOLE_MODE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleMode(
            hconsolehandle.into_param().abi(),
            ::std::mem::transmute(dwmode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleNumberOfCommandsA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    number: u32,
    exename: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleNumberOfCommandsA(
                number: u32,
                exename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleNumberOfCommandsA(
            ::std::mem::transmute(number),
            exename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleNumberOfCommandsW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    number: u32,
    exename: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleNumberOfCommandsW(
                number: u32,
                exename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleNumberOfCommandsW(
            ::std::mem::transmute(number),
            exename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleOutputCP(wcodepageid: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleOutputCP(wcodepageid: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleOutputCP(::std::mem::transmute(wcodepageid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleScreenBufferInfoEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleoutput: Param0,
    lpconsolescreenbufferinfoex: *const CONSOLE_SCREEN_BUFFER_INFOEX,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleScreenBufferInfoEx(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpconsolescreenbufferinfoex: *const CONSOLE_SCREEN_BUFFER_INFOEX,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleScreenBufferInfoEx(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpconsolescreenbufferinfoex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleScreenBufferSize<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    dwsize: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleScreenBufferSize(
                hconsoleoutput: super::super::Foundation::HANDLE,
                dwsize: COORD,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleScreenBufferSize(
            hconsoleoutput.into_param().abi(),
            dwsize.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleTextAttribute<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleoutput: Param0,
    wattributes: u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleTextAttribute(
                hconsoleoutput: super::super::Foundation::HANDLE,
                wattributes: u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleTextAttribute(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(wattributes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleTitleA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpconsoletitle: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleTitleA(
                lpconsoletitle: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleTitleA(lpconsoletitle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleTitleW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpconsoletitle: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleTitleW(
                lpconsoletitle: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleTitleW(lpconsoletitle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetConsoleWindowInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hconsoleoutput: Param0,
    babsolute: Param1,
    lpconsolewindow: *const SMALL_RECT,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleWindowInfo(
                hconsoleoutput: super::super::Foundation::HANDLE,
                babsolute: super::super::Foundation::BOOL,
                lpconsolewindow: *const SMALL_RECT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetConsoleWindowInfo(
            hconsoleoutput.into_param().abi(),
            babsolute.into_param().abi(),
            ::std::mem::transmute(lpconsolewindow),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCurrentConsoleFontEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hconsoleoutput: Param0,
    bmaximumwindow: Param1,
    lpconsolecurrentfontex: *const CONSOLE_FONT_INFOEX,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCurrentConsoleFontEx(
                hconsoleoutput: super::super::Foundation::HANDLE,
                bmaximumwindow: super::super::Foundation::BOOL,
                lpconsolecurrentfontex: *const CONSOLE_FONT_INFOEX,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetCurrentConsoleFontEx(
            hconsoleoutput.into_param().abi(),
            bmaximumwindow.into_param().abi(),
            ::std::mem::transmute(lpconsolecurrentfontex),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetStdHandle<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    nstdhandle: STD_HANDLE,
    hhandle: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetStdHandle(
                nstdhandle: STD_HANDLE,
                hhandle: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetStdHandle(
            ::std::mem::transmute(nstdhandle),
            hhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetStdHandleEx<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    nstdhandle: STD_HANDLE,
    hhandle: Param1,
    phprevvalue: *mut super::super::Foundation::HANDLE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetStdHandleEx(
                nstdhandle: STD_HANDLE,
                hhandle: super::super::Foundation::HANDLE,
                phprevvalue: *mut super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetStdHandleEx(
            ::std::mem::transmute(nstdhandle),
            hhandle.into_param().abi(),
            ::std::mem::transmute(phprevvalue),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WINDOW_BUFFER_SIZE_EVENT: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WINDOW_BUFFER_SIZE_RECORD {
    pub dwSize: COORD,
}
impl WINDOW_BUFFER_SIZE_RECORD {}
impl ::std::default::Default for WINDOW_BUFFER_SIZE_RECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINDOW_BUFFER_SIZE_RECORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINDOW_BUFFER_SIZE_RECORD")
            .field("dwSize", &self.dwSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WINDOW_BUFFER_SIZE_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
    }
}
impl ::std::cmp::Eq for WINDOW_BUFFER_SIZE_RECORD {}
unsafe impl ::windows::runtime::Abi for WINDOW_BUFFER_SIZE_RECORD {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteConsoleA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleoutput: Param0,
    lpbuffer: *const ::std::ffi::c_void,
    nnumberofcharstowrite: u32,
    lpnumberofcharswritten: *mut u32,
    lpreserved: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleA(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpbuffer: *const ::std::ffi::c_void,
                nnumberofcharstowrite: u32,
                lpnumberofcharswritten: *mut u32,
                lpreserved: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteConsoleA(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nnumberofcharstowrite),
            ::std::mem::transmute(lpnumberofcharswritten),
            ::std::mem::transmute(lpreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn WriteConsoleInputA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleinput: Param0,
    lpbuffer: *const INPUT_RECORD,
    nlength: u32,
    lpnumberofeventswritten: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleInputA(
                hconsoleinput: super::super::Foundation::HANDLE,
                lpbuffer: *const INPUT_RECORD,
                nlength: u32,
                lpnumberofeventswritten: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteConsoleInputA(
            hconsoleinput.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nlength),
            ::std::mem::transmute(lpnumberofeventswritten),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn WriteConsoleInputW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleinput: Param0,
    lpbuffer: *const INPUT_RECORD,
    nlength: u32,
    lpnumberofeventswritten: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleInputW(
                hconsoleinput: super::super::Foundation::HANDLE,
                lpbuffer: *const INPUT_RECORD,
                nlength: u32,
                lpnumberofeventswritten: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteConsoleInputW(
            hconsoleinput.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nlength),
            ::std::mem::transmute(lpnumberofeventswritten),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn WriteConsoleOutputA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, COORD>,
    Param3: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    lpbuffer: *const CHAR_INFO,
    dwbuffersize: Param2,
    dwbuffercoord: Param3,
    lpwriteregion: *mut SMALL_RECT,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleOutputA(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpbuffer: *const CHAR_INFO,
                dwbuffersize: COORD,
                dwbuffercoord: COORD,
                lpwriteregion: *mut SMALL_RECT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteConsoleOutputA(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            dwbuffersize.into_param().abi(),
            dwbuffercoord.into_param().abi(),
            ::std::mem::transmute(lpwriteregion),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteConsoleOutputAttribute<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    lpattribute: *const u16,
    nlength: u32,
    dwwritecoord: Param3,
    lpnumberofattrswritten: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleOutputAttribute(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpattribute: *const u16,
                nlength: u32,
                dwwritecoord: COORD,
                lpnumberofattrswritten: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteConsoleOutputAttribute(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpattribute),
            ::std::mem::transmute(nlength),
            dwwritecoord.into_param().abi(),
            ::std::mem::transmute(lpnumberofattrswritten),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteConsoleOutputCharacterA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    lpcharacter: Param1,
    nlength: u32,
    dwwritecoord: Param3,
    lpnumberofcharswritten: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleOutputCharacterA(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpcharacter: super::super::Foundation::PSTR,
                nlength: u32,
                dwwritecoord: COORD,
                lpnumberofcharswritten: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteConsoleOutputCharacterA(
            hconsoleoutput.into_param().abi(),
            lpcharacter.into_param().abi(),
            ::std::mem::transmute(nlength),
            dwwritecoord.into_param().abi(),
            ::std::mem::transmute(lpnumberofcharswritten),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteConsoleOutputCharacterW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    lpcharacter: Param1,
    nlength: u32,
    dwwritecoord: Param3,
    lpnumberofcharswritten: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleOutputCharacterW(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpcharacter: super::super::Foundation::PWSTR,
                nlength: u32,
                dwwritecoord: COORD,
                lpnumberofcharswritten: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteConsoleOutputCharacterW(
            hconsoleoutput.into_param().abi(),
            lpcharacter.into_param().abi(),
            ::std::mem::transmute(nlength),
            dwwritecoord.into_param().abi(),
            ::std::mem::transmute(lpnumberofcharswritten),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn WriteConsoleOutputW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param2: ::windows::runtime::IntoParam<'a, COORD>,
    Param3: ::windows::runtime::IntoParam<'a, COORD>,
>(
    hconsoleoutput: Param0,
    lpbuffer: *const CHAR_INFO,
    dwbuffersize: Param2,
    dwbuffercoord: Param3,
    lpwriteregion: *mut SMALL_RECT,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleOutputW(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpbuffer: *const CHAR_INFO,
                dwbuffersize: COORD,
                dwbuffercoord: COORD,
                lpwriteregion: *mut SMALL_RECT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteConsoleOutputW(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            dwbuffersize.into_param().abi(),
            dwbuffercoord.into_param().abi(),
            ::std::mem::transmute(lpwriteregion),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteConsoleW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hconsoleoutput: Param0,
    lpbuffer: *const ::std::ffi::c_void,
    nnumberofcharstowrite: u32,
    lpnumberofcharswritten: *mut u32,
    lpreserved: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleW(
                hconsoleoutput: super::super::Foundation::HANDLE,
                lpbuffer: *const ::std::ffi::c_void,
                nnumberofcharstowrite: u32,
                lpnumberofcharswritten: *mut u32,
                lpreserved: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteConsoleW(
            hconsoleoutput.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nnumberofcharstowrite),
            ::std::mem::transmute(lpnumberofcharswritten),
            ::std::mem::transmute(lpreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
