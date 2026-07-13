windows_link::link!("kernel32.dll" "system" fn AddConsoleAliasA(source : windows_sys::core::PCSTR, target : windows_sys::core::PCSTR, exename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn AddConsoleAliasW(source : windows_sys::core::PCWSTR, target : windows_sys::core::PCWSTR, exename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ExpungeConsoleCommandHistoryA(exename : windows_sys::core::PCSTR));
windows_link::link!("kernel32.dll" "system" fn ExpungeConsoleCommandHistoryW(exename : windows_sys::core::PCWSTR));
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasA(source : windows_sys::core::PCSTR, targetbuffer : windows_sys::core::PSTR, targetbufferlength : u32, exename : windows_sys::core::PCSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasExesA(exenamebuffer : windows_sys::core::PSTR, exenamebufferlength : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasExesLengthA() -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasExesLengthW() -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasExesW(exenamebuffer : windows_sys::core::PWSTR, exenamebufferlength : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasW(source : windows_sys::core::PCWSTR, targetbuffer : windows_sys::core::PWSTR, targetbufferlength : u32, exename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasesA(aliasbuffer : windows_sys::core::PSTR, aliasbufferlength : u32, exename : windows_sys::core::PCSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasesLengthA(exename : windows_sys::core::PCSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasesLengthW(exename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasesW(aliasbuffer : windows_sys::core::PWSTR, aliasbufferlength : u32, exename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryA(commands : windows_sys::core::PSTR, commandbufferlength : u32, exename : windows_sys::core::PCSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryLengthA(exename : windows_sys::core::PCSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryLengthW(exename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryW(commands : windows_sys::core::PWSTR, commandbufferlength : u32, exename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetConsoleDisplayMode(lpmodeflags : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetConsoleFontSize(hconsoleoutput : super::winnt::HANDLE, nfont : u32) -> super::wincontypes::COORD);
windows_link::link!("kernel32.dll" "system" fn GetConsoleHistoryInfo(lpconsolehistoryinfo : *mut CONSOLE_HISTORY_INFO) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetConsoleProcessList(lpdwprocesslist : *mut u32, dwprocesscount : u32) -> u32);
#[cfg(feature = "wincontypes")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleSelectionInfo(lpconsoleselectioninfo : *mut CONSOLE_SELECTION_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleWindow() -> super::windef::HWND);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetCurrentConsoleFont(hconsoleoutput : super::winnt::HANDLE, bmaximumwindow : windows_sys::core::BOOL, lpconsolecurrentfont : *mut super::wincontypes::CONSOLE_FONT_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetCurrentConsoleFontEx(hconsoleoutput : super::winnt::HANDLE, bmaximumwindow : windows_sys::core::BOOL, lpconsolecurrentfontex : *mut CONSOLE_FONT_INFOEX) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetConsoleDisplayMode(hconsoleoutput : super::winnt::HANDLE, dwflags : u32, lpnewscreenbufferdimensions : *mut super::wincontypes::COORD) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleHistoryInfo(lpconsolehistoryinfo : *const CONSOLE_HISTORY_INFO) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleNumberOfCommandsA(number : u32, exename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetConsoleNumberOfCommandsW(number : u32, exename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetCurrentConsoleFontEx(hconsoleoutput : super::winnt::HANDLE, bmaximumwindow : windows_sys::core::BOOL, lpconsolecurrentfontex : *const CONSOLE_FONT_INFOEX) -> windows_sys::core::BOOL);
#[repr(C)]
#[cfg(feature = "wincontypes")]
#[derive(Clone, Copy)]
pub struct CONSOLE_FONT_INFOEX {
    pub cbSize: u32,
    pub nFont: u32,
    pub dwFontSize: super::wincontypes::COORD,
    pub FontFamily: u32,
    pub FontWeight: u32,
    pub FaceName: [u16; 32],
}
#[cfg(feature = "wincontypes")]
impl Default for CONSOLE_FONT_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CONSOLE_FULLSCREEN: u32 = 1;
pub const CONSOLE_FULLSCREEN_HARDWARE: u32 = 2;
pub const CONSOLE_FULLSCREEN_MODE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_HISTORY_INFO {
    pub cbSize: u32,
    pub HistoryBufferSize: u32,
    pub NumberOfHistoryBuffers: u32,
    pub dwFlags: u32,
}
pub const CONSOLE_MOUSE_DOWN: u32 = 8;
pub const CONSOLE_MOUSE_SELECTION: u32 = 4;
pub const CONSOLE_NO_SELECTION: u32 = 0;
#[repr(C)]
#[cfg(feature = "wincontypes")]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_SELECTION_INFO {
    pub dwFlags: u32,
    pub dwSelectionAnchor: super::wincontypes::COORD,
    pub srSelection: super::wincontypes::SMALL_RECT,
}
pub const CONSOLE_SELECTION_IN_PROGRESS: u32 = 1;
pub const CONSOLE_SELECTION_NOT_EMPTY: u32 = 2;
pub const CONSOLE_WINDOWED_MODE: u32 = 2;
pub const HISTORY_NO_DUP_FLAG: u32 = 1;
#[cfg(feature = "wincontypes")]
pub type PCONSOLE_FONT_INFOEX = *mut CONSOLE_FONT_INFOEX;
pub type PCONSOLE_HISTORY_INFO = *mut CONSOLE_HISTORY_INFO;
#[cfg(feature = "wincontypes")]
pub type PCONSOLE_SELECTION_INFO = *mut CONSOLE_SELECTION_INFO;
