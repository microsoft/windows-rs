windows_link::link!("kernel32.dll" "system" fn AddConsoleAliasA(source : windows_sys::core::PCSTR, target : windows_sys::core::PCSTR, exename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn AddConsoleAliasW(source : windows_sys::core::PCWSTR, target : windows_sys::core::PCWSTR, exename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ExpungeConsoleCommandHistoryA(exename : windows_sys::core::PCSTR));
windows_link::link!("kernel32.dll" "system" fn ExpungeConsoleCommandHistoryW(exename : windows_sys::core::PCWSTR));
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasA(source : windows_sys::core::PCSTR, targetbuffer : windows_sys::core::PSTR, targetbufferlength : super::DWORD, exename : windows_sys::core::PCSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasExesA(exenamebuffer : windows_sys::core::PSTR, exenamebufferlength : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasExesLengthA() -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasExesLengthW() -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasExesW(exenamebuffer : windows_sys::core::PWSTR, exenamebufferlength : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasW(source : windows_sys::core::PCWSTR, targetbuffer : windows_sys::core::PWSTR, targetbufferlength : super::DWORD, exename : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasesA(aliasbuffer : windows_sys::core::PSTR, aliasbufferlength : super::DWORD, exename : windows_sys::core::PCSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasesLengthA(exename : windows_sys::core::PCSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasesLengthW(exename : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasesW(aliasbuffer : windows_sys::core::PWSTR, aliasbufferlength : super::DWORD, exename : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryA(commands : windows_sys::core::PSTR, commandbufferlength : super::DWORD, exename : windows_sys::core::PCSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryLengthA(exename : windows_sys::core::PCSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryLengthW(exename : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryW(commands : windows_sys::core::PWSTR, commandbufferlength : super::DWORD, exename : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleDisplayMode(lpmodeflags : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetConsoleFontSize(hconsoleoutput : super::HANDLE, nfont : super::DWORD) -> super::COORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleHistoryInfo(lpconsolehistoryinfo : PCONSOLE_HISTORY_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetConsoleProcessList(lpdwprocesslist : super::LPDWORD, dwprocesscount : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "wincontypes"))]
windows_link::link!("kernel32.dll" "system" fn GetConsoleSelectionInfo(lpconsoleselectioninfo : PCONSOLE_SELECTION_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleWindow() -> super::HWND);
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetCurrentConsoleFont(hconsoleoutput : super::HANDLE, bmaximumwindow : windows_sys::core::BOOL, lpconsolecurrentfont : super::PCONSOLE_FONT_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetCurrentConsoleFontEx(hconsoleoutput : super::HANDLE, bmaximumwindow : windows_sys::core::BOOL, lpconsolecurrentfontex : PCONSOLE_FONT_INFOEX) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetConsoleDisplayMode(hconsoleoutput : super::HANDLE, dwflags : super::DWORD, lpnewscreenbufferdimensions : super::PCOORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn SetConsoleHistoryInfo(lpconsolehistoryinfo : PCONSOLE_HISTORY_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn SetConsoleNumberOfCommandsA(number : super::DWORD, exename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn SetConsoleNumberOfCommandsW(number : super::DWORD, exename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetCurrentConsoleFontEx(hconsoleoutput : super::HANDLE, bmaximumwindow : windows_sys::core::BOOL, lpconsolecurrentfontex : PCONSOLE_FONT_INFOEX) -> windows_sys::core::BOOL);
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CONSOLE_FONT_INFOEX {
    pub cbSize: super::ULONG,
    pub nFont: super::DWORD,
    pub dwFontSize: super::COORD,
    pub FontFamily: super::UINT,
    pub FontWeight: super::UINT,
    pub FaceName: [super::WCHAR; 32],
}
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "winnt"))]
impl Default for CONSOLE_FONT_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CONSOLE_FULLSCREEN: i32 = 1;
pub const CONSOLE_FULLSCREEN_HARDWARE: i32 = 2;
pub const CONSOLE_FULLSCREEN_MODE: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_HISTORY_INFO {
    pub cbSize: super::UINT,
    pub HistoryBufferSize: super::UINT,
    pub NumberOfHistoryBuffers: super::UINT,
    pub dwFlags: super::DWORD,
}
pub const CONSOLE_MOUSE_DOWN: i32 = 8;
pub const CONSOLE_MOUSE_SELECTION: i32 = 4;
pub const CONSOLE_NO_SELECTION: i32 = 0;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincontypes"))]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_SELECTION_INFO {
    pub dwFlags: super::DWORD,
    pub dwSelectionAnchor: super::COORD,
    pub srSelection: super::SMALL_RECT,
}
pub const CONSOLE_SELECTION_IN_PROGRESS: i32 = 1;
pub const CONSOLE_SELECTION_NOT_EMPTY: i32 = 2;
pub const CONSOLE_WINDOWED_MODE: i32 = 2;
pub const HISTORY_NO_DUP_FLAG: i32 = 1;
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "winnt"))]
pub type PCONSOLE_FONT_INFOEX = *mut CONSOLE_FONT_INFOEX;
#[cfg(feature = "shared")]
pub type PCONSOLE_HISTORY_INFO = *mut CONSOLE_HISTORY_INFO;
#[cfg(all(feature = "shared", feature = "wincontypes"))]
pub type PCONSOLE_SELECTION_INFO = *mut CONSOLE_SELECTION_INFO;
