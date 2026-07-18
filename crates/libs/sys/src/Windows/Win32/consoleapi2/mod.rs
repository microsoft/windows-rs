#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateConsoleScreenBuffer(dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, dwflags : u32, lpscreenbufferdata : *const core::ffi::c_void) -> super::HANDLE);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FillConsoleOutputAttribute(hconsoleoutput : super::HANDLE, wattribute : u16, nlength : u32, dwwritecoord : super::COORD, lpnumberofattrswritten : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FillConsoleOutputCharacterA(hconsoleoutput : super::HANDLE, ccharacter : i8, nlength : u32, dwwritecoord : super::COORD, lpnumberofcharswritten : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FillConsoleOutputCharacterW(hconsoleoutput : super::HANDLE, ccharacter : u16, nlength : u32, dwwritecoord : super::COORD, lpnumberofcharswritten : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FlushConsoleInputBuffer(hconsoleinput : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GenerateConsoleCtrlEvent(dwctrlevent : u32, dwprocessgroupid : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleCursorInfo(hconsoleoutput : super::HANDLE, lpconsolecursorinfo : *mut CONSOLE_CURSOR_INFO) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetConsoleOriginalTitleA(lpconsoletitle : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleOriginalTitleW(lpconsoletitle : windows_sys::core::PWSTR, nsize : u32) -> u32);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetConsoleScreenBufferInfo(hconsoleoutput : super::HANDLE, lpconsolescreenbufferinfo : *mut CONSOLE_SCREEN_BUFFER_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "windef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetConsoleScreenBufferInfoEx(hconsoleoutput : super::HANDLE, lpconsolescreenbufferinfoex : *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetConsoleTitleA(lpconsoletitle : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleTitleW(lpconsoletitle : windows_sys::core::PWSTR, nsize : u32) -> u32);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetLargestConsoleWindowSize(hconsoleoutput : super::HANDLE) -> super::COORD);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReadConsoleOutputA(hconsoleoutput : super::HANDLE, lpbuffer : *mut super::CHAR_INFO, dwbuffersize : super::COORD, dwbuffercoord : super::COORD, lpreadregion : *mut super::SMALL_RECT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReadConsoleOutputAttribute(hconsoleoutput : super::HANDLE, lpattribute : *mut u16, nlength : u32, dwreadcoord : super::COORD, lpnumberofattrsread : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReadConsoleOutputCharacterA(hconsoleoutput : super::HANDLE, lpcharacter : windows_sys::core::PSTR, nlength : u32, dwreadcoord : super::COORD, lpnumberofcharsread : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReadConsoleOutputCharacterW(hconsoleoutput : super::HANDLE, lpcharacter : windows_sys::core::PWSTR, nlength : u32, dwreadcoord : super::COORD, lpnumberofcharsread : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReadConsoleOutputW(hconsoleoutput : super::HANDLE, lpbuffer : *mut super::CHAR_INFO, dwbuffersize : super::COORD, dwbuffercoord : super::COORD, lpreadregion : *mut super::SMALL_RECT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ScrollConsoleScreenBufferA(hconsoleoutput : super::HANDLE, lpscrollrectangle : *const super::SMALL_RECT, lpcliprectangle : *const super::SMALL_RECT, dwdestinationorigin : super::COORD, lpfill : *const super::CHAR_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ScrollConsoleScreenBufferW(hconsoleoutput : super::HANDLE, lpscrollrectangle : *const super::SMALL_RECT, lpcliprectangle : *const super::SMALL_RECT, dwdestinationorigin : super::COORD, lpfill : *const super::CHAR_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetConsoleActiveScreenBuffer(hconsoleoutput : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleCP(wcodepageid : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetConsoleCursorInfo(hconsoleoutput : super::HANDLE, lpconsolecursorinfo : *const CONSOLE_CURSOR_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetConsoleCursorPosition(hconsoleoutput : super::HANDLE, dwcursorposition : super::COORD) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleOutputCP(wcodepageid : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "windef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetConsoleScreenBufferInfoEx(hconsoleoutput : super::HANDLE, lpconsolescreenbufferinfoex : *const CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetConsoleScreenBufferSize(hconsoleoutput : super::HANDLE, dwsize : super::COORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetConsoleTextAttribute(hconsoleoutput : super::HANDLE, wattributes : u16) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleTitleA(lpconsoletitle : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleTitleW(lpconsoletitle : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetConsoleWindowInfo(hconsoleoutput : super::HANDLE, babsolute : windows_sys::core::BOOL, lpconsolewindow : *const super::SMALL_RECT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WriteConsoleInputA(hconsoleinput : super::HANDLE, lpbuffer : *const super::INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WriteConsoleInputW(hconsoleinput : super::HANDLE, lpbuffer : *const super::INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WriteConsoleOutputA(hconsoleoutput : super::HANDLE, lpbuffer : *const super::CHAR_INFO, dwbuffersize : super::COORD, dwbuffercoord : super::COORD, lpwriteregion : *mut super::SMALL_RECT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WriteConsoleOutputAttribute(hconsoleoutput : super::HANDLE, lpattribute : *const u16, nlength : u32, dwwritecoord : super::COORD, lpnumberofattrswritten : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WriteConsoleOutputCharacterA(hconsoleoutput : super::HANDLE, lpcharacter : windows_sys::core::PCSTR, nlength : u32, dwwritecoord : super::COORD, lpnumberofcharswritten : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WriteConsoleOutputCharacterW(hconsoleoutput : super::HANDLE, lpcharacter : windows_sys::core::PCWSTR, nlength : u32, dwwritecoord : super::COORD, lpnumberofcharswritten : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WriteConsoleOutputW(hconsoleoutput : super::HANDLE, lpbuffer : *const super::CHAR_INFO, dwbuffersize : super::COORD, dwbuffercoord : super::COORD, lpwriteregion : *mut super::SMALL_RECT) -> windows_sys::core::BOOL);
pub const BACKGROUND_BLUE: u32 = 16;
pub const BACKGROUND_GREEN: u32 = 32;
pub const BACKGROUND_INTENSITY: u32 = 128;
pub const BACKGROUND_RED: u32 = 64;
pub const COMMON_LVB_GRID_HORIZONTAL: u32 = 1024;
pub const COMMON_LVB_GRID_LVERTICAL: u32 = 2048;
pub const COMMON_LVB_GRID_RVERTICAL: u32 = 4096;
pub const COMMON_LVB_LEADING_BYTE: u32 = 256;
pub const COMMON_LVB_REVERSE_VIDEO: u32 = 16384;
pub const COMMON_LVB_SBCSDBCS: u32 = 768;
pub const COMMON_LVB_TRAILING_BYTE: u32 = 512;
pub const COMMON_LVB_UNDERSCORE: u32 = 32768;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_CURSOR_INFO {
    pub dwSize: u32,
    pub bVisible: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "wincontypes")]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: super::COORD,
    pub dwCursorPosition: super::COORD,
    pub wAttributes: u16,
    pub srWindow: super::SMALL_RECT,
    pub dwMaximumWindowSize: super::COORD,
}
#[repr(C)]
#[cfg(all(feature = "wincontypes", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct CONSOLE_SCREEN_BUFFER_INFOEX {
    pub cbSize: u32,
    pub dwSize: super::COORD,
    pub dwCursorPosition: super::COORD,
    pub wAttributes: u16,
    pub srWindow: super::SMALL_RECT,
    pub dwMaximumWindowSize: super::COORD,
    pub wPopupAttributes: u16,
    pub bFullscreenSupported: windows_sys::core::BOOL,
    pub ColorTable: [super::COLORREF; 16],
}
#[cfg(all(feature = "wincontypes", feature = "windef"))]
impl Default for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FOREGROUND_BLUE: u32 = 1;
pub const FOREGROUND_GREEN: u32 = 2;
pub const FOREGROUND_INTENSITY: u32 = 8;
pub const FOREGROUND_RED: u32 = 4;
pub type PCONSOLE_CURSOR_INFO = *mut CONSOLE_CURSOR_INFO;
#[cfg(feature = "wincontypes")]
pub type PCONSOLE_SCREEN_BUFFER_INFO = *mut CONSOLE_SCREEN_BUFFER_INFO;
#[cfg(all(feature = "wincontypes", feature = "windef"))]
pub type PCONSOLE_SCREEN_BUFFER_INFOEX = *mut CONSOLE_SCREEN_BUFFER_INFOEX;
