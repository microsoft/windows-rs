#[inline]
pub unsafe fn AddConsoleAliasA<P0, P1, P2>(source: P0, target: P1, exename: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn AddConsoleAliasA(source : windows_core::PCSTR, target : windows_core::PCSTR, exename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { AddConsoleAliasA(source.param().abi(), target.param().abi(), exename.param().abi()) }
}
#[inline]
pub unsafe fn AddConsoleAliasW<P0, P1, P2>(source: P0, target: P1, exename: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn AddConsoleAliasW(source : windows_core::PCWSTR, target : windows_core::PCWSTR, exename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { AddConsoleAliasW(source.param().abi(), target.param().abi(), exename.param().abi()) }
}
#[inline]
pub unsafe fn ExpungeConsoleCommandHistoryA<P0>(exename: P0)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn ExpungeConsoleCommandHistoryA(exename : windows_core::PCSTR));
    unsafe { ExpungeConsoleCommandHistoryA(exename.param().abi()) }
}
#[inline]
pub unsafe fn ExpungeConsoleCommandHistoryW<P0>(exename: P0)
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn ExpungeConsoleCommandHistoryW(exename : windows_core::PCWSTR));
    unsafe { ExpungeConsoleCommandHistoryW(exename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetConsoleAliasA<P0, P3>(source: P0, targetbuffer: windows_core::PSTR, targetbufferlength: super::DWORD, exename: P3) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasA(source : windows_core::PCSTR, targetbuffer : windows_core::PSTR, targetbufferlength : super::DWORD, exename : windows_core::PCSTR) -> super::DWORD);
    unsafe { GetConsoleAliasA(source.param().abi(), targetbuffer, targetbufferlength, exename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetConsoleAliasExesA(exenamebuffer: windows_core::PSTR, exenamebufferlength: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasExesA(exenamebuffer : windows_core::PSTR, exenamebufferlength : super::DWORD) -> super::DWORD);
    unsafe { GetConsoleAliasExesA(exenamebuffer, exenamebufferlength) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetConsoleAliasExesLengthA() -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasExesLengthA() -> super::DWORD);
    unsafe { GetConsoleAliasExesLengthA() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetConsoleAliasExesLengthW() -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasExesLengthW() -> super::DWORD);
    unsafe { GetConsoleAliasExesLengthW() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetConsoleAliasExesW(exenamebuffer: windows_core::PWSTR, exenamebufferlength: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasExesW(exenamebuffer : windows_core::PWSTR, exenamebufferlength : super::DWORD) -> super::DWORD);
    unsafe { GetConsoleAliasExesW(exenamebuffer, exenamebufferlength) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetConsoleAliasW<P0, P3>(source: P0, targetbuffer: windows_core::PWSTR, targetbufferlength: super::DWORD, exename: P3) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasW(source : windows_core::PCWSTR, targetbuffer : windows_core::PWSTR, targetbufferlength : super::DWORD, exename : windows_core::PCWSTR) -> super::DWORD);
    unsafe { GetConsoleAliasW(source.param().abi(), targetbuffer, targetbufferlength, exename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetConsoleAliasesA<P2>(aliasbuffer: windows_core::PSTR, aliasbufferlength: super::DWORD, exename: P2) -> super::DWORD
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasesA(aliasbuffer : windows_core::PSTR, aliasbufferlength : super::DWORD, exename : windows_core::PCSTR) -> super::DWORD);
    unsafe { GetConsoleAliasesA(aliasbuffer, aliasbufferlength, exename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetConsoleAliasesLengthA<P0>(exename: P0) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasesLengthA(exename : windows_core::PCSTR) -> super::DWORD);
    unsafe { GetConsoleAliasesLengthA(exename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetConsoleAliasesLengthW<P0>(exename: P0) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasesLengthW(exename : windows_core::PCWSTR) -> super::DWORD);
    unsafe { GetConsoleAliasesLengthW(exename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetConsoleAliasesW<P2>(aliasbuffer: windows_core::PWSTR, aliasbufferlength: super::DWORD, exename: P2) -> super::DWORD
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasesW(aliasbuffer : windows_core::PWSTR, aliasbufferlength : super::DWORD, exename : windows_core::PCWSTR) -> super::DWORD);
    unsafe { GetConsoleAliasesW(aliasbuffer, aliasbufferlength, exename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetConsoleCommandHistoryA<P2>(commands: windows_core::PSTR, commandbufferlength: super::DWORD, exename: P2) -> super::DWORD
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryA(commands : windows_core::PSTR, commandbufferlength : super::DWORD, exename : windows_core::PCSTR) -> super::DWORD);
    unsafe { GetConsoleCommandHistoryA(commands, commandbufferlength, exename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetConsoleCommandHistoryLengthA<P0>(exename: P0) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryLengthA(exename : windows_core::PCSTR) -> super::DWORD);
    unsafe { GetConsoleCommandHistoryLengthA(exename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetConsoleCommandHistoryLengthW<P0>(exename: P0) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryLengthW(exename : windows_core::PCWSTR) -> super::DWORD);
    unsafe { GetConsoleCommandHistoryLengthW(exename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetConsoleCommandHistoryW<P2>(commands: windows_core::PWSTR, commandbufferlength: super::DWORD, exename: P2) -> super::DWORD
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryW(commands : windows_core::PWSTR, commandbufferlength : super::DWORD, exename : windows_core::PCWSTR) -> super::DWORD);
    unsafe { GetConsoleCommandHistoryW(commands, commandbufferlength, exename.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetConsoleDisplayMode(lpmodeflags: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleDisplayMode(lpmodeflags : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetConsoleDisplayMode(lpmodeflags as _) }
}
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn GetConsoleFontSize(hconsoleoutput: super::HANDLE, nfont: super::DWORD) -> super::COORD {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleFontSize(hconsoleoutput : super::HANDLE, nfont : super::DWORD) -> super::COORD);
    unsafe { GetConsoleFontSize(hconsoleoutput, nfont) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetConsoleHistoryInfo(lpconsolehistoryinfo: PCONSOLE_HISTORY_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleHistoryInfo(lpconsolehistoryinfo : PCONSOLE_HISTORY_INFO) -> windows_core::BOOL);
    unsafe { GetConsoleHistoryInfo(lpconsolehistoryinfo as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetConsoleProcessList(lpdwprocesslist: super::LPDWORD, dwprocesscount: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleProcessList(lpdwprocesslist : super::LPDWORD, dwprocesscount : super::DWORD) -> super::DWORD);
    unsafe { GetConsoleProcessList(lpdwprocesslist as _, dwprocesscount) }
}
#[cfg(all(feature = "shared", feature = "wincontypes"))]
#[inline]
pub unsafe fn GetConsoleSelectionInfo(lpconsoleselectioninfo: PCONSOLE_SELECTION_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleSelectionInfo(lpconsoleselectioninfo : PCONSOLE_SELECTION_INFO) -> windows_core::BOOL);
    unsafe { GetConsoleSelectionInfo(lpconsoleselectioninfo as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetConsoleWindow() -> super::HWND {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleWindow() -> super::HWND);
    unsafe { GetConsoleWindow() }
}
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn GetCurrentConsoleFont(hconsoleoutput: super::HANDLE, bmaximumwindow: bool, lpconsolecurrentfont: super::PCONSOLE_FONT_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentConsoleFont(hconsoleoutput : super::HANDLE, bmaximumwindow : windows_core::BOOL, lpconsolecurrentfont : super::PCONSOLE_FONT_INFO) -> windows_core::BOOL);
    unsafe { GetCurrentConsoleFont(hconsoleoutput, bmaximumwindow.into(), lpconsolecurrentfont as _) }
}
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn GetCurrentConsoleFontEx(hconsoleoutput: super::HANDLE, bmaximumwindow: bool, lpconsolecurrentfontex: PCONSOLE_FONT_INFOEX) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentConsoleFontEx(hconsoleoutput : super::HANDLE, bmaximumwindow : windows_core::BOOL, lpconsolecurrentfontex : PCONSOLE_FONT_INFOEX) -> windows_core::BOOL);
    unsafe { GetCurrentConsoleFontEx(hconsoleoutput, bmaximumwindow.into(), lpconsolecurrentfontex as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons as _) }
}
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn SetConsoleDisplayMode(hconsoleoutput: super::HANDLE, dwflags: super::DWORD, lpnewscreenbufferdimensions: Option<super::PCOORD>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleDisplayMode(hconsoleoutput : super::HANDLE, dwflags : super::DWORD, lpnewscreenbufferdimensions : super::PCOORD) -> windows_core::BOOL);
    unsafe { SetConsoleDisplayMode(hconsoleoutput, dwflags, lpnewscreenbufferdimensions.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetConsoleHistoryInfo(lpconsolehistoryinfo: PCONSOLE_HISTORY_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleHistoryInfo(lpconsolehistoryinfo : PCONSOLE_HISTORY_INFO) -> windows_core::BOOL);
    unsafe { SetConsoleHistoryInfo(lpconsolehistoryinfo) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetConsoleNumberOfCommandsA<P1>(number: super::DWORD, exename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetConsoleNumberOfCommandsA(number : super::DWORD, exename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetConsoleNumberOfCommandsA(number, exename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetConsoleNumberOfCommandsW<P1>(number: super::DWORD, exename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetConsoleNumberOfCommandsW(number : super::DWORD, exename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetConsoleNumberOfCommandsW(number, exename.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn SetCurrentConsoleFontEx(hconsoleoutput: super::HANDLE, bmaximumwindow: bool, lpconsolecurrentfontex: PCONSOLE_FONT_INFOEX) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetCurrentConsoleFontEx(hconsoleoutput : super::HANDLE, bmaximumwindow : windows_core::BOOL, lpconsolecurrentfontex : PCONSOLE_FONT_INFOEX) -> windows_core::BOOL);
    unsafe { SetCurrentConsoleFontEx(hconsoleoutput, bmaximumwindow.into(), lpconsolecurrentfontex) }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wincontypes", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
