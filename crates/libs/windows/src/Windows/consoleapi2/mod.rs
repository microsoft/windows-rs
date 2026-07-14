#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateConsoleScreenBuffer(dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, dwflags: u32, lpscreenbufferdata: Option<*const core::ffi::c_void>) -> super::winnt::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateConsoleScreenBuffer(dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, dwflags : u32, lpscreenbufferdata : *const core::ffi::c_void) -> super::winnt::HANDLE);
    unsafe { CreateConsoleScreenBuffer(dwdesiredaccess, dwsharemode, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, dwflags, lpscreenbufferdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn FillConsoleOutputAttribute(hconsoleoutput: super::winnt::HANDLE, wattribute: u16, nlength: u32, dwwritecoord: super::wincontypes::COORD, lpnumberofattrswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FillConsoleOutputAttribute(hconsoleoutput : super::winnt::HANDLE, wattribute : u16, nlength : u32, dwwritecoord : super::wincontypes::COORD, lpnumberofattrswritten : *mut u32) -> windows_core::BOOL);
    unsafe { FillConsoleOutputAttribute(hconsoleoutput, wattribute, nlength, core::mem::transmute(dwwritecoord), lpnumberofattrswritten as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn FillConsoleOutputCharacterA(hconsoleoutput: super::winnt::HANDLE, ccharacter: i8, nlength: u32, dwwritecoord: super::wincontypes::COORD, lpnumberofcharswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FillConsoleOutputCharacterA(hconsoleoutput : super::winnt::HANDLE, ccharacter : i8, nlength : u32, dwwritecoord : super::wincontypes::COORD, lpnumberofcharswritten : *mut u32) -> windows_core::BOOL);
    unsafe { FillConsoleOutputCharacterA(hconsoleoutput, ccharacter, nlength, core::mem::transmute(dwwritecoord), lpnumberofcharswritten as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn FillConsoleOutputCharacterW(hconsoleoutput: super::winnt::HANDLE, ccharacter: u16, nlength: u32, dwwritecoord: super::wincontypes::COORD, lpnumberofcharswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FillConsoleOutputCharacterW(hconsoleoutput : super::winnt::HANDLE, ccharacter : u16, nlength : u32, dwwritecoord : super::wincontypes::COORD, lpnumberofcharswritten : *mut u32) -> windows_core::BOOL);
    unsafe { FillConsoleOutputCharacterW(hconsoleoutput, ccharacter, nlength, core::mem::transmute(dwwritecoord), lpnumberofcharswritten as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FlushConsoleInputBuffer(hconsoleinput: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FlushConsoleInputBuffer(hconsoleinput : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { FlushConsoleInputBuffer(hconsoleinput) }
}
#[inline]
pub unsafe fn GenerateConsoleCtrlEvent(dwctrlevent: u32, dwprocessgroupid: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GenerateConsoleCtrlEvent(dwctrlevent : u32, dwprocessgroupid : u32) -> windows_core::BOOL);
    unsafe { GenerateConsoleCtrlEvent(dwctrlevent, dwprocessgroupid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetConsoleCursorInfo(hconsoleoutput: super::winnt::HANDLE, lpconsolecursorinfo: *mut CONSOLE_CURSOR_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleCursorInfo(hconsoleoutput : super::winnt::HANDLE, lpconsolecursorinfo : *mut CONSOLE_CURSOR_INFO) -> windows_core::BOOL);
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
pub unsafe fn GetConsoleScreenBufferInfo(hconsoleoutput: super::winnt::HANDLE, lpconsolescreenbufferinfo: *mut CONSOLE_SCREEN_BUFFER_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleScreenBufferInfo(hconsoleoutput : super::winnt::HANDLE, lpconsolescreenbufferinfo : *mut CONSOLE_SCREEN_BUFFER_INFO) -> windows_core::BOOL);
    unsafe { GetConsoleScreenBufferInfo(hconsoleoutput, lpconsolescreenbufferinfo as _) }
}
#[cfg(all(feature = "wincontypes", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn GetConsoleScreenBufferInfoEx(hconsoleoutput: super::winnt::HANDLE, lpconsolescreenbufferinfoex: *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleScreenBufferInfoEx(hconsoleoutput : super::winnt::HANDLE, lpconsolescreenbufferinfoex : *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_core::BOOL);
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
pub unsafe fn GetLargestConsoleWindowSize(hconsoleoutput: super::winnt::HANDLE) -> super::wincontypes::COORD {
    windows_core::link!("kernel32.dll" "system" fn GetLargestConsoleWindowSize(hconsoleoutput : super::winnt::HANDLE) -> super::wincontypes::COORD);
    unsafe { GetLargestConsoleWindowSize(hconsoleoutput) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn ReadConsoleOutputA(hconsoleoutput: super::winnt::HANDLE, lpbuffer: *mut super::wincontypes::CHAR_INFO, dwbuffersize: super::wincontypes::COORD, dwbuffercoord: super::wincontypes::COORD, lpreadregion: *mut super::wincontypes::SMALL_RECT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleOutputA(hconsoleoutput : super::winnt::HANDLE, lpbuffer : *mut super::wincontypes::CHAR_INFO, dwbuffersize : super::wincontypes::COORD, dwbuffercoord : super::wincontypes::COORD, lpreadregion : *mut super::wincontypes::SMALL_RECT) -> windows_core::BOOL);
    unsafe { ReadConsoleOutputA(hconsoleoutput, lpbuffer as _, core::mem::transmute(dwbuffersize), core::mem::transmute(dwbuffercoord), lpreadregion as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn ReadConsoleOutputAttribute(hconsoleoutput: super::winnt::HANDLE, lpattribute: &mut [u16], dwreadcoord: super::wincontypes::COORD, lpnumberofattrsread: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleOutputAttribute(hconsoleoutput : super::winnt::HANDLE, lpattribute : *mut u16, nlength : u32, dwreadcoord : super::wincontypes::COORD, lpnumberofattrsread : *mut u32) -> windows_core::BOOL);
    unsafe { ReadConsoleOutputAttribute(hconsoleoutput, lpattribute.as_mut_ptr(), lpattribute.len().try_into().unwrap(), core::mem::transmute(dwreadcoord), lpnumberofattrsread as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn ReadConsoleOutputCharacterA(hconsoleoutput: super::winnt::HANDLE, lpcharacter: &mut [u8], dwreadcoord: super::wincontypes::COORD, lpnumberofcharsread: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleOutputCharacterA(hconsoleoutput : super::winnt::HANDLE, lpcharacter : windows_core::PSTR, nlength : u32, dwreadcoord : super::wincontypes::COORD, lpnumberofcharsread : *mut u32) -> windows_core::BOOL);
    unsafe { ReadConsoleOutputCharacterA(hconsoleoutput, core::mem::transmute(lpcharacter.as_mut_ptr()), lpcharacter.len().try_into().unwrap(), core::mem::transmute(dwreadcoord), lpnumberofcharsread as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn ReadConsoleOutputCharacterW(hconsoleoutput: super::winnt::HANDLE, lpcharacter: &mut [u16], dwreadcoord: super::wincontypes::COORD, lpnumberofcharsread: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleOutputCharacterW(hconsoleoutput : super::winnt::HANDLE, lpcharacter : windows_core::PWSTR, nlength : u32, dwreadcoord : super::wincontypes::COORD, lpnumberofcharsread : *mut u32) -> windows_core::BOOL);
    unsafe { ReadConsoleOutputCharacterW(hconsoleoutput, core::mem::transmute(lpcharacter.as_mut_ptr()), lpcharacter.len().try_into().unwrap(), core::mem::transmute(dwreadcoord), lpnumberofcharsread as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn ReadConsoleOutputW(hconsoleoutput: super::winnt::HANDLE, lpbuffer: *mut super::wincontypes::CHAR_INFO, dwbuffersize: super::wincontypes::COORD, dwbuffercoord: super::wincontypes::COORD, lpreadregion: *mut super::wincontypes::SMALL_RECT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleOutputW(hconsoleoutput : super::winnt::HANDLE, lpbuffer : *mut super::wincontypes::CHAR_INFO, dwbuffersize : super::wincontypes::COORD, dwbuffercoord : super::wincontypes::COORD, lpreadregion : *mut super::wincontypes::SMALL_RECT) -> windows_core::BOOL);
    unsafe { ReadConsoleOutputW(hconsoleoutput, lpbuffer as _, core::mem::transmute(dwbuffersize), core::mem::transmute(dwbuffercoord), lpreadregion as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn ScrollConsoleScreenBufferA(hconsoleoutput: super::winnt::HANDLE, lpscrollrectangle: *const super::wincontypes::SMALL_RECT, lpcliprectangle: Option<*const super::wincontypes::SMALL_RECT>, dwdestinationorigin: super::wincontypes::COORD, lpfill: *const super::wincontypes::CHAR_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ScrollConsoleScreenBufferA(hconsoleoutput : super::winnt::HANDLE, lpscrollrectangle : *const super::wincontypes::SMALL_RECT, lpcliprectangle : *const super::wincontypes::SMALL_RECT, dwdestinationorigin : super::wincontypes::COORD, lpfill : *const super::wincontypes::CHAR_INFO) -> windows_core::BOOL);
    unsafe { ScrollConsoleScreenBufferA(hconsoleoutput, lpscrollrectangle, lpcliprectangle.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(dwdestinationorigin), lpfill) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn ScrollConsoleScreenBufferW(hconsoleoutput: super::winnt::HANDLE, lpscrollrectangle: *const super::wincontypes::SMALL_RECT, lpcliprectangle: Option<*const super::wincontypes::SMALL_RECT>, dwdestinationorigin: super::wincontypes::COORD, lpfill: *const super::wincontypes::CHAR_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ScrollConsoleScreenBufferW(hconsoleoutput : super::winnt::HANDLE, lpscrollrectangle : *const super::wincontypes::SMALL_RECT, lpcliprectangle : *const super::wincontypes::SMALL_RECT, dwdestinationorigin : super::wincontypes::COORD, lpfill : *const super::wincontypes::CHAR_INFO) -> windows_core::BOOL);
    unsafe { ScrollConsoleScreenBufferW(hconsoleoutput, lpscrollrectangle, lpcliprectangle.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(dwdestinationorigin), lpfill) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetConsoleActiveScreenBuffer(hconsoleoutput: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleActiveScreenBuffer(hconsoleoutput : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { SetConsoleActiveScreenBuffer(hconsoleoutput) }
}
#[inline]
pub unsafe fn SetConsoleCP(wcodepageid: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleCP(wcodepageid : u32) -> windows_core::BOOL);
    unsafe { SetConsoleCP(wcodepageid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetConsoleCursorInfo(hconsoleoutput: super::winnt::HANDLE, lpconsolecursorinfo: *const CONSOLE_CURSOR_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleCursorInfo(hconsoleoutput : super::winnt::HANDLE, lpconsolecursorinfo : *const CONSOLE_CURSOR_INFO) -> windows_core::BOOL);
    unsafe { SetConsoleCursorInfo(hconsoleoutput, lpconsolecursorinfo) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn SetConsoleCursorPosition(hconsoleoutput: super::winnt::HANDLE, dwcursorposition: super::wincontypes::COORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleCursorPosition(hconsoleoutput : super::winnt::HANDLE, dwcursorposition : super::wincontypes::COORD) -> windows_core::BOOL);
    unsafe { SetConsoleCursorPosition(hconsoleoutput, core::mem::transmute(dwcursorposition)) }
}
#[inline]
pub unsafe fn SetConsoleOutputCP(wcodepageid: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleOutputCP(wcodepageid : u32) -> windows_core::BOOL);
    unsafe { SetConsoleOutputCP(wcodepageid) }
}
#[cfg(all(feature = "wincontypes", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn SetConsoleScreenBufferInfoEx(hconsoleoutput: super::winnt::HANDLE, lpconsolescreenbufferinfoex: *const CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleScreenBufferInfoEx(hconsoleoutput : super::winnt::HANDLE, lpconsolescreenbufferinfoex : *const CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_core::BOOL);
    unsafe { SetConsoleScreenBufferInfoEx(hconsoleoutput, lpconsolescreenbufferinfoex) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn SetConsoleScreenBufferSize(hconsoleoutput: super::winnt::HANDLE, dwsize: super::wincontypes::COORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleScreenBufferSize(hconsoleoutput : super::winnt::HANDLE, dwsize : super::wincontypes::COORD) -> windows_core::BOOL);
    unsafe { SetConsoleScreenBufferSize(hconsoleoutput, core::mem::transmute(dwsize)) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetConsoleTextAttribute(hconsoleoutput: super::winnt::HANDLE, wattributes: u16) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleTextAttribute(hconsoleoutput : super::winnt::HANDLE, wattributes : u16) -> windows_core::BOOL);
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
pub unsafe fn SetConsoleWindowInfo(hconsoleoutput: super::winnt::HANDLE, babsolute: bool, lpconsolewindow: *const super::wincontypes::SMALL_RECT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleWindowInfo(hconsoleoutput : super::winnt::HANDLE, babsolute : windows_core::BOOL, lpconsolewindow : *const super::wincontypes::SMALL_RECT) -> windows_core::BOOL);
    unsafe { SetConsoleWindowInfo(hconsoleoutput, babsolute.into(), lpconsolewindow) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn WriteConsoleInputA(hconsoleinput: super::winnt::HANDLE, lpbuffer: &[super::wincontypes::INPUT_RECORD], lpnumberofeventswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleInputA(hconsoleinput : super::winnt::HANDLE, lpbuffer : *const super::wincontypes::INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleInputA(hconsoleinput, lpbuffer.as_ptr(), lpbuffer.len().try_into().unwrap(), lpnumberofeventswritten as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn WriteConsoleInputW(hconsoleinput: super::winnt::HANDLE, lpbuffer: &[super::wincontypes::INPUT_RECORD], lpnumberofeventswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleInputW(hconsoleinput : super::winnt::HANDLE, lpbuffer : *const super::wincontypes::INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleInputW(hconsoleinput, lpbuffer.as_ptr(), lpbuffer.len().try_into().unwrap(), lpnumberofeventswritten as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn WriteConsoleOutputA(hconsoleoutput: super::winnt::HANDLE, lpbuffer: *const super::wincontypes::CHAR_INFO, dwbuffersize: super::wincontypes::COORD, dwbuffercoord: super::wincontypes::COORD, lpwriteregion: *mut super::wincontypes::SMALL_RECT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleOutputA(hconsoleoutput : super::winnt::HANDLE, lpbuffer : *const super::wincontypes::CHAR_INFO, dwbuffersize : super::wincontypes::COORD, dwbuffercoord : super::wincontypes::COORD, lpwriteregion : *mut super::wincontypes::SMALL_RECT) -> windows_core::BOOL);
    unsafe { WriteConsoleOutputA(hconsoleoutput, lpbuffer, core::mem::transmute(dwbuffersize), core::mem::transmute(dwbuffercoord), lpwriteregion as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn WriteConsoleOutputAttribute(hconsoleoutput: super::winnt::HANDLE, lpattribute: &[u16], dwwritecoord: super::wincontypes::COORD, lpnumberofattrswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleOutputAttribute(hconsoleoutput : super::winnt::HANDLE, lpattribute : *const u16, nlength : u32, dwwritecoord : super::wincontypes::COORD, lpnumberofattrswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleOutputAttribute(hconsoleoutput, lpattribute.as_ptr(), lpattribute.len().try_into().unwrap(), core::mem::transmute(dwwritecoord), lpnumberofattrswritten as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn WriteConsoleOutputCharacterA(hconsoleoutput: super::winnt::HANDLE, lpcharacter: &[u8], dwwritecoord: super::wincontypes::COORD, lpnumberofcharswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleOutputCharacterA(hconsoleoutput : super::winnt::HANDLE, lpcharacter : windows_core::PCSTR, nlength : u32, dwwritecoord : super::wincontypes::COORD, lpnumberofcharswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleOutputCharacterA(hconsoleoutput, core::mem::transmute(lpcharacter.as_ptr()), lpcharacter.len().try_into().unwrap(), core::mem::transmute(dwwritecoord), lpnumberofcharswritten as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn WriteConsoleOutputCharacterW(hconsoleoutput: super::winnt::HANDLE, lpcharacter: &[u16], dwwritecoord: super::wincontypes::COORD, lpnumberofcharswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleOutputCharacterW(hconsoleoutput : super::winnt::HANDLE, lpcharacter : windows_core::PCWSTR, nlength : u32, dwwritecoord : super::wincontypes::COORD, lpnumberofcharswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleOutputCharacterW(hconsoleoutput, core::mem::transmute(lpcharacter.as_ptr()), lpcharacter.len().try_into().unwrap(), core::mem::transmute(dwwritecoord), lpnumberofcharswritten as _) }
}
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
#[inline]
pub unsafe fn WriteConsoleOutputW(hconsoleoutput: super::winnt::HANDLE, lpbuffer: *const super::wincontypes::CHAR_INFO, dwbuffersize: super::wincontypes::COORD, dwbuffercoord: super::wincontypes::COORD, lpwriteregion: *mut super::wincontypes::SMALL_RECT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleOutputW(hconsoleoutput : super::winnt::HANDLE, lpbuffer : *const super::wincontypes::CHAR_INFO, dwbuffersize : super::wincontypes::COORD, dwbuffercoord : super::wincontypes::COORD, lpwriteregion : *mut super::wincontypes::SMALL_RECT) -> windows_core::BOOL);
    unsafe { WriteConsoleOutputW(hconsoleoutput, lpbuffer, core::mem::transmute(dwbuffersize), core::mem::transmute(dwbuffercoord), lpwriteregion as _) }
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONSOLE_CURSOR_INFO {
    pub dwSize: u32,
    pub bVisible: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "wincontypes")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: super::wincontypes::COORD,
    pub dwCursorPosition: super::wincontypes::COORD,
    pub wAttributes: u16,
    pub srWindow: super::wincontypes::SMALL_RECT,
    pub dwMaximumWindowSize: super::wincontypes::COORD,
}
#[repr(C)]
#[cfg(all(feature = "wincontypes", feature = "windef"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONSOLE_SCREEN_BUFFER_INFOEX {
    pub cbSize: u32,
    pub dwSize: super::wincontypes::COORD,
    pub dwCursorPosition: super::wincontypes::COORD,
    pub wAttributes: u16,
    pub srWindow: super::wincontypes::SMALL_RECT,
    pub dwMaximumWindowSize: super::wincontypes::COORD,
    pub wPopupAttributes: u16,
    pub bFullscreenSupported: windows_core::BOOL,
    pub ColorTable: [super::windef::COLORREF; 16],
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
