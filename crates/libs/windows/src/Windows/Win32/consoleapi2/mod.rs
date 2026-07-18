#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateConsoleScreenBuffer(dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: Option<*const super::SECURITY_ATTRIBUTES>, dwflags: u32, lpscreenbufferdata: Option<*const core::ffi::c_void>) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateConsoleScreenBuffer(dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, dwflags : u32, lpscreenbufferdata : *const core::ffi::c_void) -> super::HANDLE);
    unsafe { CreateConsoleScreenBuffer(dwdesiredaccess, dwsharemode, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, dwflags, lpscreenbufferdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn FillConsoleOutputAttribute(hconsoleoutput: super::HANDLE, wattribute: u16, nlength: u32, dwwritecoord: super::COORD, lpnumberofattrswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FillConsoleOutputAttribute(hconsoleoutput : super::HANDLE, wattribute : u16, nlength : u32, dwwritecoord : super::COORD, lpnumberofattrswritten : *mut u32) -> windows_core::BOOL);
    unsafe { FillConsoleOutputAttribute(hconsoleoutput, wattribute, nlength, dwwritecoord, lpnumberofattrswritten as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn FillConsoleOutputCharacterA(hconsoleoutput: super::HANDLE, ccharacter: i8, nlength: u32, dwwritecoord: super::COORD, lpnumberofcharswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FillConsoleOutputCharacterA(hconsoleoutput : super::HANDLE, ccharacter : i8, nlength : u32, dwwritecoord : super::COORD, lpnumberofcharswritten : *mut u32) -> windows_core::BOOL);
    unsafe { FillConsoleOutputCharacterA(hconsoleoutput, ccharacter, nlength, dwwritecoord, lpnumberofcharswritten as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn FillConsoleOutputCharacterW(hconsoleoutput: super::HANDLE, ccharacter: u16, nlength: u32, dwwritecoord: super::COORD, lpnumberofcharswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FillConsoleOutputCharacterW(hconsoleoutput : super::HANDLE, ccharacter : u16, nlength : u32, dwwritecoord : super::COORD, lpnumberofcharswritten : *mut u32) -> windows_core::BOOL);
    unsafe { FillConsoleOutputCharacterW(hconsoleoutput, ccharacter, nlength, dwwritecoord, lpnumberofcharswritten as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FlushConsoleInputBuffer(hconsoleinput: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FlushConsoleInputBuffer(hconsoleinput : super::HANDLE) -> windows_core::BOOL);
    unsafe { FlushConsoleInputBuffer(hconsoleinput) }
}
#[inline]
pub unsafe fn GenerateConsoleCtrlEvent(dwctrlevent: u32, dwprocessgroupid: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GenerateConsoleCtrlEvent(dwctrlevent : u32, dwprocessgroupid : u32) -> windows_core::BOOL);
    unsafe { GenerateConsoleCtrlEvent(dwctrlevent, dwprocessgroupid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetConsoleCursorInfo(hconsoleoutput: super::HANDLE, lpconsolecursorinfo: *mut CONSOLE_CURSOR_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleCursorInfo(hconsoleoutput : super::HANDLE, lpconsolecursorinfo : *mut CONSOLE_CURSOR_INFO) -> windows_core::BOOL);
    unsafe { GetConsoleCursorInfo(hconsoleoutput, lpconsolecursorinfo as _) }
}
#[inline]
pub unsafe fn GetConsoleOriginalTitleA(lpconsoletitle: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleOriginalTitleA(lpconsoletitle : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetConsoleOriginalTitleA(core::mem::transmute(lpconsoletitle.as_mut_ptr()), lpconsoletitle.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetConsoleOriginalTitleW(lpconsoletitle: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleOriginalTitleW(lpconsoletitle : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetConsoleOriginalTitleW(core::mem::transmute(lpconsoletitle.as_mut_ptr()), lpconsoletitle.len().try_into().unwrap()) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn GetConsoleScreenBufferInfo(hconsoleoutput: super::HANDLE, lpconsolescreenbufferinfo: *mut CONSOLE_SCREEN_BUFFER_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleScreenBufferInfo(hconsoleoutput : super::HANDLE, lpconsolescreenbufferinfo : *mut CONSOLE_SCREEN_BUFFER_INFO) -> windows_core::BOOL);
    unsafe { GetConsoleScreenBufferInfo(hconsoleoutput, lpconsolescreenbufferinfo as _) }
}
#[cfg(all(feature = "wincontypes", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetConsoleScreenBufferInfoEx(hconsoleoutput: super::HANDLE, lpconsolescreenbufferinfoex: *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleScreenBufferInfoEx(hconsoleoutput : super::HANDLE, lpconsolescreenbufferinfoex : *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_core::BOOL);
    unsafe { GetConsoleScreenBufferInfoEx(hconsoleoutput, lpconsolescreenbufferinfoex as _) }
}
#[inline]
pub unsafe fn GetConsoleTitleA(lpconsoletitle: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleTitleA(lpconsoletitle : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetConsoleTitleA(core::mem::transmute(lpconsoletitle.as_mut_ptr()), lpconsoletitle.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetConsoleTitleW(lpconsoletitle: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleTitleW(lpconsoletitle : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetConsoleTitleW(core::mem::transmute(lpconsoletitle.as_mut_ptr()), lpconsoletitle.len().try_into().unwrap()) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn GetLargestConsoleWindowSize(hconsoleoutput: super::HANDLE) -> super::COORD {
    windows_core::link!("kernel32.dll" "system" fn GetLargestConsoleWindowSize(hconsoleoutput : super::HANDLE) -> super::COORD);
    unsafe { GetLargestConsoleWindowSize(hconsoleoutput) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn ReadConsoleOutputA(hconsoleoutput: super::HANDLE, lpbuffer: *mut super::CHAR_INFO, dwbuffersize: super::COORD, dwbuffercoord: super::COORD, lpreadregion: *mut super::SMALL_RECT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleOutputA(hconsoleoutput : super::HANDLE, lpbuffer : *mut super::CHAR_INFO, dwbuffersize : super::COORD, dwbuffercoord : super::COORD, lpreadregion : *mut super::SMALL_RECT) -> windows_core::BOOL);
    unsafe { ReadConsoleOutputA(hconsoleoutput, lpbuffer as _, dwbuffersize, dwbuffercoord, lpreadregion as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn ReadConsoleOutputAttribute(hconsoleoutput: super::HANDLE, lpattribute: &mut [u16], dwreadcoord: super::COORD, lpnumberofattrsread: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleOutputAttribute(hconsoleoutput : super::HANDLE, lpattribute : *mut u16, nlength : u32, dwreadcoord : super::COORD, lpnumberofattrsread : *mut u32) -> windows_core::BOOL);
    unsafe { ReadConsoleOutputAttribute(hconsoleoutput, lpattribute.as_mut_ptr(), lpattribute.len().try_into().unwrap(), dwreadcoord, lpnumberofattrsread as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn ReadConsoleOutputCharacterA(hconsoleoutput: super::HANDLE, lpcharacter: &mut [u8], dwreadcoord: super::COORD, lpnumberofcharsread: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleOutputCharacterA(hconsoleoutput : super::HANDLE, lpcharacter : windows_core::PSTR, nlength : u32, dwreadcoord : super::COORD, lpnumberofcharsread : *mut u32) -> windows_core::BOOL);
    unsafe { ReadConsoleOutputCharacterA(hconsoleoutput, core::mem::transmute(lpcharacter.as_mut_ptr()), lpcharacter.len().try_into().unwrap(), dwreadcoord, lpnumberofcharsread as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn ReadConsoleOutputCharacterW(hconsoleoutput: super::HANDLE, lpcharacter: &mut [u16], dwreadcoord: super::COORD, lpnumberofcharsread: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleOutputCharacterW(hconsoleoutput : super::HANDLE, lpcharacter : windows_core::PWSTR, nlength : u32, dwreadcoord : super::COORD, lpnumberofcharsread : *mut u32) -> windows_core::BOOL);
    unsafe { ReadConsoleOutputCharacterW(hconsoleoutput, core::mem::transmute(lpcharacter.as_mut_ptr()), lpcharacter.len().try_into().unwrap(), dwreadcoord, lpnumberofcharsread as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn ReadConsoleOutputW(hconsoleoutput: super::HANDLE, lpbuffer: *mut super::CHAR_INFO, dwbuffersize: super::COORD, dwbuffercoord: super::COORD, lpreadregion: *mut super::SMALL_RECT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleOutputW(hconsoleoutput : super::HANDLE, lpbuffer : *mut super::CHAR_INFO, dwbuffersize : super::COORD, dwbuffercoord : super::COORD, lpreadregion : *mut super::SMALL_RECT) -> windows_core::BOOL);
    unsafe { ReadConsoleOutputW(hconsoleoutput, lpbuffer as _, dwbuffersize, dwbuffercoord, lpreadregion as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn ScrollConsoleScreenBufferA(hconsoleoutput: super::HANDLE, lpscrollrectangle: *const super::SMALL_RECT, lpcliprectangle: Option<*const super::SMALL_RECT>, dwdestinationorigin: super::COORD, lpfill: *const super::CHAR_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ScrollConsoleScreenBufferA(hconsoleoutput : super::HANDLE, lpscrollrectangle : *const super::SMALL_RECT, lpcliprectangle : *const super::SMALL_RECT, dwdestinationorigin : super::COORD, lpfill : *const super::CHAR_INFO) -> windows_core::BOOL);
    unsafe { ScrollConsoleScreenBufferA(hconsoleoutput, lpscrollrectangle, lpcliprectangle.unwrap_or(core::mem::zeroed()) as _, dwdestinationorigin, lpfill) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn ScrollConsoleScreenBufferW(hconsoleoutput: super::HANDLE, lpscrollrectangle: *const super::SMALL_RECT, lpcliprectangle: Option<*const super::SMALL_RECT>, dwdestinationorigin: super::COORD, lpfill: *const super::CHAR_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ScrollConsoleScreenBufferW(hconsoleoutput : super::HANDLE, lpscrollrectangle : *const super::SMALL_RECT, lpcliprectangle : *const super::SMALL_RECT, dwdestinationorigin : super::COORD, lpfill : *const super::CHAR_INFO) -> windows_core::BOOL);
    unsafe { ScrollConsoleScreenBufferW(hconsoleoutput, lpscrollrectangle, lpcliprectangle.unwrap_or(core::mem::zeroed()) as _, dwdestinationorigin, lpfill) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetConsoleActiveScreenBuffer(hconsoleoutput: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleActiveScreenBuffer(hconsoleoutput : super::HANDLE) -> windows_core::BOOL);
    unsafe { SetConsoleActiveScreenBuffer(hconsoleoutput) }
}
#[inline]
pub unsafe fn SetConsoleCP(wcodepageid: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleCP(wcodepageid : u32) -> windows_core::BOOL);
    unsafe { SetConsoleCP(wcodepageid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetConsoleCursorInfo(hconsoleoutput: super::HANDLE, lpconsolecursorinfo: *const CONSOLE_CURSOR_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleCursorInfo(hconsoleoutput : super::HANDLE, lpconsolecursorinfo : *const CONSOLE_CURSOR_INFO) -> windows_core::BOOL);
    unsafe { SetConsoleCursorInfo(hconsoleoutput, lpconsolecursorinfo) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn SetConsoleCursorPosition(hconsoleoutput: super::HANDLE, dwcursorposition: super::COORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleCursorPosition(hconsoleoutput : super::HANDLE, dwcursorposition : super::COORD) -> windows_core::BOOL);
    unsafe { SetConsoleCursorPosition(hconsoleoutput, dwcursorposition) }
}
#[inline]
pub unsafe fn SetConsoleOutputCP(wcodepageid: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleOutputCP(wcodepageid : u32) -> windows_core::BOOL);
    unsafe { SetConsoleOutputCP(wcodepageid) }
}
#[cfg(all(feature = "wincontypes", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn SetConsoleScreenBufferInfoEx(hconsoleoutput: super::HANDLE, lpconsolescreenbufferinfoex: *const CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleScreenBufferInfoEx(hconsoleoutput : super::HANDLE, lpconsolescreenbufferinfoex : *const CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_core::BOOL);
    unsafe { SetConsoleScreenBufferInfoEx(hconsoleoutput, lpconsolescreenbufferinfoex) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn SetConsoleScreenBufferSize(hconsoleoutput: super::HANDLE, dwsize: super::COORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleScreenBufferSize(hconsoleoutput : super::HANDLE, dwsize : super::COORD) -> windows_core::BOOL);
    unsafe { SetConsoleScreenBufferSize(hconsoleoutput, dwsize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetConsoleTextAttribute(hconsoleoutput: super::HANDLE, wattributes: u16) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleTextAttribute(hconsoleoutput : super::HANDLE, wattributes : u16) -> windows_core::BOOL);
    unsafe { SetConsoleTextAttribute(hconsoleoutput, wattributes) }
}
#[inline]
pub unsafe fn SetConsoleTitleA<P0>(lpconsoletitle: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetConsoleTitleA(lpconsoletitle : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetConsoleTitleA(lpconsoletitle.param().abi()) }
}
#[inline]
pub unsafe fn SetConsoleTitleW<P0>(lpconsoletitle: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetConsoleTitleW(lpconsoletitle : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetConsoleTitleW(lpconsoletitle.param().abi()) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn SetConsoleWindowInfo(hconsoleoutput: super::HANDLE, babsolute: bool, lpconsolewindow: *const super::SMALL_RECT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleWindowInfo(hconsoleoutput : super::HANDLE, babsolute : windows_core::BOOL, lpconsolewindow : *const super::SMALL_RECT) -> windows_core::BOOL);
    unsafe { SetConsoleWindowInfo(hconsoleoutput, babsolute.into(), lpconsolewindow) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn WriteConsoleInputA(hconsoleinput: super::HANDLE, lpbuffer: &[super::INPUT_RECORD], lpnumberofeventswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleInputA(hconsoleinput : super::HANDLE, lpbuffer : *const super::INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleInputA(hconsoleinput, lpbuffer.as_ptr(), lpbuffer.len().try_into().unwrap(), lpnumberofeventswritten as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn WriteConsoleInputW(hconsoleinput: super::HANDLE, lpbuffer: &[super::INPUT_RECORD], lpnumberofeventswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleInputW(hconsoleinput : super::HANDLE, lpbuffer : *const super::INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleInputW(hconsoleinput, lpbuffer.as_ptr(), lpbuffer.len().try_into().unwrap(), lpnumberofeventswritten as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn WriteConsoleOutputA(hconsoleoutput: super::HANDLE, lpbuffer: *const super::CHAR_INFO, dwbuffersize: super::COORD, dwbuffercoord: super::COORD, lpwriteregion: *mut super::SMALL_RECT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleOutputA(hconsoleoutput : super::HANDLE, lpbuffer : *const super::CHAR_INFO, dwbuffersize : super::COORD, dwbuffercoord : super::COORD, lpwriteregion : *mut super::SMALL_RECT) -> windows_core::BOOL);
    unsafe { WriteConsoleOutputA(hconsoleoutput, lpbuffer, dwbuffersize, dwbuffercoord, lpwriteregion as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn WriteConsoleOutputAttribute(hconsoleoutput: super::HANDLE, lpattribute: &[u16], dwwritecoord: super::COORD, lpnumberofattrswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleOutputAttribute(hconsoleoutput : super::HANDLE, lpattribute : *const u16, nlength : u32, dwwritecoord : super::COORD, lpnumberofattrswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleOutputAttribute(hconsoleoutput, lpattribute.as_ptr(), lpattribute.len().try_into().unwrap(), dwwritecoord, lpnumberofattrswritten as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn WriteConsoleOutputCharacterA(hconsoleoutput: super::HANDLE, lpcharacter: &[u8], dwwritecoord: super::COORD, lpnumberofcharswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleOutputCharacterA(hconsoleoutput : super::HANDLE, lpcharacter : windows_core::PCSTR, nlength : u32, dwwritecoord : super::COORD, lpnumberofcharswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleOutputCharacterA(hconsoleoutput, core::mem::transmute(lpcharacter.as_ptr()), lpcharacter.len().try_into().unwrap(), dwwritecoord, lpnumberofcharswritten as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn WriteConsoleOutputCharacterW(hconsoleoutput: super::HANDLE, lpcharacter: &[u16], dwwritecoord: super::COORD, lpnumberofcharswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleOutputCharacterW(hconsoleoutput : super::HANDLE, lpcharacter : windows_core::PCWSTR, nlength : u32, dwwritecoord : super::COORD, lpnumberofcharswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleOutputCharacterW(hconsoleoutput, core::mem::transmute(lpcharacter.as_ptr()), lpcharacter.len().try_into().unwrap(), dwwritecoord, lpnumberofcharswritten as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn WriteConsoleOutputW(hconsoleoutput: super::HANDLE, lpbuffer: *const super::CHAR_INFO, dwbuffersize: super::COORD, dwbuffercoord: super::COORD, lpwriteregion: *mut super::SMALL_RECT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleOutputW(hconsoleoutput : super::HANDLE, lpbuffer : *const super::CHAR_INFO, dwbuffersize : super::COORD, dwbuffercoord : super::COORD, lpwriteregion : *mut super::SMALL_RECT) -> windows_core::BOOL);
    unsafe { WriteConsoleOutputW(hconsoleoutput, lpbuffer, dwbuffersize, dwbuffercoord, lpwriteregion as _) }
}
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONSOLE_CURSOR_INFO {
    pub dwSize: u32,
    pub bVisible: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "wincontypes")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: super::COORD,
    pub dwCursorPosition: super::COORD,
    pub wAttributes: u16,
    pub srWindow: super::SMALL_RECT,
    pub dwMaximumWindowSize: super::COORD,
}
#[repr(C)]
#[cfg(all(feature = "wincontypes", feature = "windef"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONSOLE_SCREEN_BUFFER_INFOEX {
    pub cbSize: u32,
    pub dwSize: super::COORD,
    pub dwCursorPosition: super::COORD,
    pub wAttributes: u16,
    pub srWindow: super::SMALL_RECT,
    pub dwMaximumWindowSize: super::COORD,
    pub wPopupAttributes: u16,
    pub bFullscreenSupported: windows_core::BOOL,
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
