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
#[inline]
pub unsafe fn GetConsoleAliasA<P0, P3>(source: P0, targetbuffer: &mut [u8], exename: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasA(source : windows_core::PCSTR, targetbuffer : windows_core::PSTR, targetbufferlength : u32, exename : windows_core::PCSTR) -> u32);
    unsafe { GetConsoleAliasA(source.param().abi(), core::mem::transmute(targetbuffer.as_ptr()), targetbuffer.len().try_into().unwrap(), exename.param().abi()) }
}
#[inline]
pub unsafe fn GetConsoleAliasExesA(exenamebuffer: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasExesA(exenamebuffer : windows_core::PSTR, exenamebufferlength : u32) -> u32);
    unsafe { GetConsoleAliasExesA(core::mem::transmute(exenamebuffer.as_ptr()), exenamebuffer.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetConsoleAliasExesLengthA() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasExesLengthA() -> u32);
    unsafe { GetConsoleAliasExesLengthA() }
}
#[inline]
pub unsafe fn GetConsoleAliasExesLengthW() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasExesLengthW() -> u32);
    unsafe { GetConsoleAliasExesLengthW() }
}
#[inline]
pub unsafe fn GetConsoleAliasExesW(exenamebuffer: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasExesW(exenamebuffer : windows_core::PWSTR, exenamebufferlength : u32) -> u32);
    unsafe { GetConsoleAliasExesW(core::mem::transmute(exenamebuffer.as_ptr()), exenamebuffer.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetConsoleAliasW<P0, P3>(source: P0, targetbuffer: &mut [u16], exename: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasW(source : windows_core::PCWSTR, targetbuffer : windows_core::PWSTR, targetbufferlength : u32, exename : windows_core::PCWSTR) -> u32);
    unsafe { GetConsoleAliasW(source.param().abi(), core::mem::transmute(targetbuffer.as_ptr()), targetbuffer.len().try_into().unwrap(), exename.param().abi()) }
}
#[inline]
pub unsafe fn GetConsoleAliasesA<P2>(aliasbuffer: &mut [u8], exename: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasesA(aliasbuffer : windows_core::PSTR, aliasbufferlength : u32, exename : windows_core::PCSTR) -> u32);
    unsafe { GetConsoleAliasesA(core::mem::transmute(aliasbuffer.as_ptr()), aliasbuffer.len().try_into().unwrap(), exename.param().abi()) }
}
#[inline]
pub unsafe fn GetConsoleAliasesLengthA<P0>(exename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasesLengthA(exename : windows_core::PCSTR) -> u32);
    unsafe { GetConsoleAliasesLengthA(exename.param().abi()) }
}
#[inline]
pub unsafe fn GetConsoleAliasesLengthW<P0>(exename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasesLengthW(exename : windows_core::PCWSTR) -> u32);
    unsafe { GetConsoleAliasesLengthW(exename.param().abi()) }
}
#[inline]
pub unsafe fn GetConsoleAliasesW<P2>(aliasbuffer: &mut [u16], exename: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleAliasesW(aliasbuffer : windows_core::PWSTR, aliasbufferlength : u32, exename : windows_core::PCWSTR) -> u32);
    unsafe { GetConsoleAliasesW(core::mem::transmute(aliasbuffer.as_ptr()), aliasbuffer.len().try_into().unwrap(), exename.param().abi()) }
}
#[inline]
pub unsafe fn GetConsoleCommandHistoryA<P2>(commands: &mut [u8], exename: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryA(commands : windows_core::PSTR, commandbufferlength : u32, exename : windows_core::PCSTR) -> u32);
    unsafe { GetConsoleCommandHistoryA(core::mem::transmute(commands.as_ptr()), commands.len().try_into().unwrap(), exename.param().abi()) }
}
#[inline]
pub unsafe fn GetConsoleCommandHistoryLengthA<P0>(exename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryLengthA(exename : windows_core::PCSTR) -> u32);
    unsafe { GetConsoleCommandHistoryLengthA(exename.param().abi()) }
}
#[inline]
pub unsafe fn GetConsoleCommandHistoryLengthW<P0>(exename: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryLengthW(exename : windows_core::PCWSTR) -> u32);
    unsafe { GetConsoleCommandHistoryLengthW(exename.param().abi()) }
}
#[inline]
pub unsafe fn GetConsoleCommandHistoryW<P2>(commands: windows_core::PWSTR, commandbufferlength: u32, exename: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryW(commands : windows_core::PWSTR, commandbufferlength : u32, exename : windows_core::PCWSTR) -> u32);
    unsafe { GetConsoleCommandHistoryW(core::mem::transmute(commands), commandbufferlength, exename.param().abi()) }
}
#[inline]
pub unsafe fn GetConsoleDisplayMode(lpmodeflags: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleDisplayMode(lpmodeflags : *mut u32) -> windows_core::BOOL);
    unsafe { GetConsoleDisplayMode(lpmodeflags as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn GetConsoleFontSize(hconsoleoutput: super::winnt::HANDLE, nfont: u32) -> super::wincontypes::COORD {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleFontSize(hconsoleoutput : super::winnt::HANDLE, nfont : u32) -> super::wincontypes::COORD);
    unsafe { GetConsoleFontSize(hconsoleoutput, nfont) }
}
#[inline]
pub unsafe fn GetConsoleHistoryInfo(lpconsolehistoryinfo: *mut CONSOLE_HISTORY_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleHistoryInfo(lpconsolehistoryinfo : *mut CONSOLE_HISTORY_INFO) -> windows_core::BOOL);
    unsafe { GetConsoleHistoryInfo(lpconsolehistoryinfo as _) }
}
#[inline]
pub unsafe fn GetConsoleProcessList(lpdwprocesslist: &mut [u32]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleProcessList(lpdwprocesslist : *mut u32, dwprocesscount : u32) -> u32);
    unsafe { GetConsoleProcessList(core::mem::transmute(lpdwprocesslist.as_ptr()), lpdwprocesslist.len().try_into().unwrap()) }
}
#[cfg(feature = "wincontypes")]
#[inline]
pub unsafe fn GetConsoleSelectionInfo(lpconsoleselectioninfo: *mut CONSOLE_SELECTION_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleSelectionInfo(lpconsoleselectioninfo : *mut CONSOLE_SELECTION_INFO) -> windows_core::BOOL);
    unsafe { GetConsoleSelectionInfo(lpconsoleselectioninfo as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetConsoleWindow() -> super::windef::HWND {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleWindow() -> super::windef::HWND);
    unsafe { GetConsoleWindow() }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn GetCurrentConsoleFont(hconsoleoutput: super::winnt::HANDLE, bmaximumwindow: bool, lpconsolecurrentfont: *mut super::wincontypes::CONSOLE_FONT_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentConsoleFont(hconsoleoutput : super::winnt::HANDLE, bmaximumwindow : windows_core::BOOL, lpconsolecurrentfont : *mut super::wincontypes::CONSOLE_FONT_INFO) -> windows_core::BOOL);
    unsafe { GetCurrentConsoleFont(hconsoleoutput, bmaximumwindow.into(), lpconsolecurrentfont as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn GetCurrentConsoleFontEx(hconsoleoutput: super::winnt::HANDLE, bmaximumwindow: bool, lpconsolecurrentfontex: *mut CONSOLE_FONT_INFOEX) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentConsoleFontEx(hconsoleoutput : super::winnt::HANDLE, bmaximumwindow : windows_core::BOOL, lpconsolecurrentfontex : *mut CONSOLE_FONT_INFOEX) -> windows_core::BOOL);
    unsafe { GetCurrentConsoleFontEx(hconsoleoutput, bmaximumwindow.into(), lpconsolecurrentfontex as _) }
}
#[inline]
pub unsafe fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons : *mut u32) -> windows_core::BOOL);
    unsafe { GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn SetConsoleDisplayMode(hconsoleoutput: super::winnt::HANDLE, dwflags: u32, lpnewscreenbufferdimensions: Option<*mut super::wincontypes::COORD>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleDisplayMode(hconsoleoutput : super::winnt::HANDLE, dwflags : u32, lpnewscreenbufferdimensions : *mut super::wincontypes::COORD) -> windows_core::BOOL);
    unsafe { SetConsoleDisplayMode(hconsoleoutput, dwflags, lpnewscreenbufferdimensions.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetConsoleHistoryInfo(lpconsolehistoryinfo: *const CONSOLE_HISTORY_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleHistoryInfo(lpconsolehistoryinfo : *const CONSOLE_HISTORY_INFO) -> windows_core::BOOL);
    unsafe { SetConsoleHistoryInfo(lpconsolehistoryinfo) }
}
#[inline]
pub unsafe fn SetConsoleNumberOfCommandsA<P1>(number: u32, exename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetConsoleNumberOfCommandsA(number : u32, exename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetConsoleNumberOfCommandsA(number, exename.param().abi()) }
}
#[inline]
pub unsafe fn SetConsoleNumberOfCommandsW<P1>(number: u32, exename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetConsoleNumberOfCommandsW(number : u32, exename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetConsoleNumberOfCommandsW(number, exename.param().abi()) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn SetCurrentConsoleFontEx(hconsoleoutput: super::winnt::HANDLE, bmaximumwindow: bool, lpconsolecurrentfontex: *const CONSOLE_FONT_INFOEX) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetCurrentConsoleFontEx(hconsoleoutput : super::winnt::HANDLE, bmaximumwindow : windows_core::BOOL, lpconsolecurrentfontex : *const CONSOLE_FONT_INFOEX) -> windows_core::BOOL);
    unsafe { SetCurrentConsoleFontEx(hconsoleoutput, bmaximumwindow.into(), lpconsolecurrentfontex) }
}
#[repr(C)]
#[cfg(feature = "wincontypes")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCONSOLE_FONT_INFOEX(pub *mut CONSOLE_FONT_INFOEX);
#[cfg(feature = "wincontypes")]
impl PCONSOLE_FONT_INFOEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "wincontypes")]
impl Default for PCONSOLE_FONT_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCONSOLE_HISTORY_INFO(pub *mut CONSOLE_HISTORY_INFO);
impl PCONSOLE_HISTORY_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCONSOLE_HISTORY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "wincontypes")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCONSOLE_SELECTION_INFO(pub *mut CONSOLE_SELECTION_INFO);
#[cfg(feature = "wincontypes")]
impl PCONSOLE_SELECTION_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "wincontypes")]
impl Default for PCONSOLE_SELECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
