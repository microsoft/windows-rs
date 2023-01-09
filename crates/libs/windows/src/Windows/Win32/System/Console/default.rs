#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHAR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CONSOLE_CHARACTER_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONSOLE_CHARACTER_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONSOLE_CHARACTER_ATTRIBUTES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CONSOLE_CHARACTER_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CONSOLE_CHARACTER_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CONSOLE_CHARACTER_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CONSOLE_CHARACTER_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CONSOLE_CHARACTER_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CONSOLE_CURSOR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CONSOLE_CURSOR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.bVisible == other.bVisible
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CONSOLE_CURSOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CONSOLE_CURSOR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_CURSOR_INFO").field("dwSize", &self.dwSize).field("bVisible", &self.bVisible).finish()
    }
}
impl ::core::default::Default for CONSOLE_FONT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONSOLE_FONT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.nFont == other.nFont && self.dwFontSize == other.dwFontSize
    }
}
impl ::core::cmp::Eq for CONSOLE_FONT_INFO {}
impl ::core::fmt::Debug for CONSOLE_FONT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_FONT_INFO").field("nFont", &self.nFont).field("dwFontSize", &self.dwFontSize).finish()
    }
}
impl ::core::default::Default for CONSOLE_FONT_INFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONSOLE_FONT_INFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.nFont == other.nFont && self.dwFontSize == other.dwFontSize && self.FontFamily == other.FontFamily && self.FontWeight == other.FontWeight && self.FaceName == other.FaceName
    }
}
impl ::core::cmp::Eq for CONSOLE_FONT_INFOEX {}
impl ::core::fmt::Debug for CONSOLE_FONT_INFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_FONT_INFOEX").field("cbSize", &self.cbSize).field("nFont", &self.nFont).field("dwFontSize", &self.dwFontSize).field("FontFamily", &self.FontFamily).field("FontWeight", &self.FontWeight).field("FaceName", &self.FaceName).finish()
    }
}
impl ::core::default::Default for CONSOLE_HISTORY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONSOLE_HISTORY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.HistoryBufferSize == other.HistoryBufferSize && self.NumberOfHistoryBuffers == other.NumberOfHistoryBuffers && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for CONSOLE_HISTORY_INFO {}
impl ::core::fmt::Debug for CONSOLE_HISTORY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_HISTORY_INFO").field("cbSize", &self.cbSize).field("HistoryBufferSize", &self.HistoryBufferSize).field("NumberOfHistoryBuffers", &self.NumberOfHistoryBuffers).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for CONSOLE_MODE {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for CONSOLE_READCONSOLE_CONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONSOLE_READCONSOLE_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.nLength == other.nLength && self.nInitialChars == other.nInitialChars && self.dwCtrlWakeupMask == other.dwCtrlWakeupMask && self.dwControlKeyState == other.dwControlKeyState
    }
}
impl ::core::cmp::Eq for CONSOLE_READCONSOLE_CONTROL {}
impl ::core::fmt::Debug for CONSOLE_READCONSOLE_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_READCONSOLE_CONTROL").field("nLength", &self.nLength).field("nInitialChars", &self.nInitialChars).field("dwCtrlWakeupMask", &self.dwCtrlWakeupMask).field("dwControlKeyState", &self.dwControlKeyState).finish()
    }
}
impl ::core::default::Default for CONSOLE_SCREEN_BUFFER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONSOLE_SCREEN_BUFFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCursorPosition == other.dwCursorPosition && self.wAttributes == other.wAttributes && self.srWindow == other.srWindow && self.dwMaximumWindowSize == other.dwMaximumWindowSize
    }
}
impl ::core::cmp::Eq for CONSOLE_SCREEN_BUFFER_INFO {}
impl ::core::fmt::Debug for CONSOLE_SCREEN_BUFFER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_SCREEN_BUFFER_INFO").field("dwSize", &self.dwSize).field("dwCursorPosition", &self.dwCursorPosition).field("wAttributes", &self.wAttributes).field("srWindow", &self.srWindow).field("dwMaximumWindowSize", &self.dwMaximumWindowSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwSize == other.dwSize && self.dwCursorPosition == other.dwCursorPosition && self.wAttributes == other.wAttributes && self.srWindow == other.srWindow && self.dwMaximumWindowSize == other.dwMaximumWindowSize && self.wPopupAttributes == other.wPopupAttributes && self.bFullscreenSupported == other.bFullscreenSupported && self.ColorTable == other.ColorTable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CONSOLE_SCREEN_BUFFER_INFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_SCREEN_BUFFER_INFOEX").field("cbSize", &self.cbSize).field("dwSize", &self.dwSize).field("dwCursorPosition", &self.dwCursorPosition).field("wAttributes", &self.wAttributes).field("srWindow", &self.srWindow).field("dwMaximumWindowSize", &self.dwMaximumWindowSize).field("wPopupAttributes", &self.wPopupAttributes).field("bFullscreenSupported", &self.bFullscreenSupported).field("ColorTable", &self.ColorTable).finish()
    }
}
impl ::core::default::Default for CONSOLE_SELECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONSOLE_SELECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwSelectionAnchor == other.dwSelectionAnchor && self.srSelection == other.srSelection
    }
}
impl ::core::cmp::Eq for CONSOLE_SELECTION_INFO {}
impl ::core::fmt::Debug for CONSOLE_SELECTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_SELECTION_INFO").field("dwFlags", &self.dwFlags).field("dwSelectionAnchor", &self.dwSelectionAnchor).field("srSelection", &self.srSelection).finish()
    }
}
impl ::core::default::Default for COORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COORD {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for COORD {}
impl ::core::fmt::Debug for COORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COORD").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FOCUS_EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FOCUS_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.bSetFocus == other.bSetFocus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FOCUS_EVENT_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FOCUS_EVENT_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FOCUS_EVENT_RECORD").field("bSetFocus", &self.bSetFocus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INPUT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KEY_EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MENU_EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MENU_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwCommandId == other.dwCommandId
    }
}
impl ::core::cmp::Eq for MENU_EVENT_RECORD {}
impl ::core::fmt::Debug for MENU_EVENT_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENU_EVENT_RECORD").field("dwCommandId", &self.dwCommandId).finish()
    }
}
impl ::core::default::Default for MOUSE_EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MOUSE_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwMousePosition == other.dwMousePosition && self.dwButtonState == other.dwButtonState && self.dwControlKeyState == other.dwControlKeyState && self.dwEventFlags == other.dwEventFlags
    }
}
impl ::core::cmp::Eq for MOUSE_EVENT_RECORD {}
impl ::core::fmt::Debug for MOUSE_EVENT_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSE_EVENT_RECORD").field("dwMousePosition", &self.dwMousePosition).field("dwButtonState", &self.dwButtonState).field("dwControlKeyState", &self.dwControlKeyState).field("dwEventFlags", &self.dwEventFlags).finish()
    }
}
impl ::core::default::Default for SMALL_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SMALL_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.Left == other.Left && self.Top == other.Top && self.Right == other.Right && self.Bottom == other.Bottom
    }
}
impl ::core::cmp::Eq for SMALL_RECT {}
impl ::core::fmt::Debug for SMALL_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMALL_RECT").field("Left", &self.Left).field("Top", &self.Top).field("Right", &self.Right).field("Bottom", &self.Bottom).finish()
    }
}
impl ::core::default::Default for STD_HANDLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STD_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STD_HANDLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINDOW_BUFFER_SIZE_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINDOW_BUFFER_SIZE_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
    }
}
impl ::core::cmp::Eq for WINDOW_BUFFER_SIZE_RECORD {}
impl ::core::fmt::Debug for WINDOW_BUFFER_SIZE_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOW_BUFFER_SIZE_RECORD").field("dwSize", &self.dwSize).finish()
    }
}
