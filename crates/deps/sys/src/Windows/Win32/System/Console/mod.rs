#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddConsoleAliasA(source: super::super::Foundation::PSTR, target: super::super::Foundation::PSTR, exename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddConsoleAliasW(source: super::super::Foundation::PWSTR, target: super::super::Foundation::PWSTR, exename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocConsole() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AttachConsole(dwprocessid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`*"]
    pub fn ClosePseudoConsole(hpc: HPCON);
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateConsoleScreenBuffer(dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwflags: u32, lpscreenbufferdata: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePseudoConsole(size: COORD, hinput: super::super::Foundation::HANDLE, houtput: super::super::Foundation::HANDLE, dwflags: u32, phpc: *mut HPCON) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExpungeConsoleCommandHistoryA(exename: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExpungeConsoleCommandHistoryW(exename: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillConsoleOutputAttribute(hconsoleoutput: super::super::Foundation::HANDLE, wattribute: u16, nlength: u32, dwwritecoord: COORD, lpnumberofattrswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillConsoleOutputCharacterA(hconsoleoutput: super::super::Foundation::HANDLE, ccharacter: super::super::Foundation::CHAR, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillConsoleOutputCharacterW(hconsoleoutput: super::super::Foundation::HANDLE, ccharacter: u16, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushConsoleInputBuffer(hconsoleinput: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeConsole() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GenerateConsoleCtrlEvent(dwctrlevent: u32, dwprocessgroupid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleAliasA(source: super::super::Foundation::PSTR, targetbuffer: super::super::Foundation::PSTR, targetbufferlength: u32, exename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleAliasExesA(exenamebuffer: super::super::Foundation::PSTR, exenamebufferlength: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Console`*"]
    pub fn GetConsoleAliasExesLengthA() -> u32;
    #[doc = "*Required features: `Win32_System_Console`*"]
    pub fn GetConsoleAliasExesLengthW() -> u32;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleAliasExesW(exenamebuffer: super::super::Foundation::PWSTR, exenamebufferlength: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleAliasW(source: super::super::Foundation::PWSTR, targetbuffer: super::super::Foundation::PWSTR, targetbufferlength: u32, exename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleAliasesA(aliasbuffer: super::super::Foundation::PSTR, aliasbufferlength: u32, exename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleAliasesLengthA(exename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleAliasesLengthW(exename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleAliasesW(aliasbuffer: super::super::Foundation::PWSTR, aliasbufferlength: u32, exename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Console`*"]
    pub fn GetConsoleCP() -> u32;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleCommandHistoryA(commands: super::super::Foundation::PSTR, commandbufferlength: u32, exename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleCommandHistoryLengthA(exename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleCommandHistoryLengthW(exename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleCommandHistoryW(commands: super::super::Foundation::PWSTR, commandbufferlength: u32, exename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleCursorInfo(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolecursorinfo: *mut CONSOLE_CURSOR_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleDisplayMode(lpmodeflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleFontSize(hconsoleoutput: super::super::Foundation::HANDLE, nfont: u32) -> COORD;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleHistoryInfo(lpconsolehistoryinfo: *mut CONSOLE_HISTORY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleMode(hconsolehandle: super::super::Foundation::HANDLE, lpmode: *mut CONSOLE_MODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleOriginalTitleA(lpconsoletitle: super::super::Foundation::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleOriginalTitleW(lpconsoletitle: super::super::Foundation::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Console`*"]
    pub fn GetConsoleOutputCP() -> u32;
    #[doc = "*Required features: `Win32_System_Console`*"]
    pub fn GetConsoleProcessList(lpdwprocesslist: *mut u32, dwprocesscount: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleScreenBufferInfo(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolescreenbufferinfo: *mut CONSOLE_SCREEN_BUFFER_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleScreenBufferInfoEx(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolescreenbufferinfoex: *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleSelectionInfo(lpconsoleselectioninfo: *mut CONSOLE_SELECTION_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleTitleA(lpconsoletitle: super::super::Foundation::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleTitleW(lpconsoletitle: super::super::Foundation::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleWindow() -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentConsoleFont(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: super::super::Foundation::BOOL, lpconsolecurrentfont: *mut CONSOLE_FONT_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentConsoleFontEx(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: super::super::Foundation::BOOL, lpconsolecurrentfontex: *mut CONSOLE_FONT_INFOEX) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLargestConsoleWindowSize(hconsoleoutput: super::super::Foundation::HANDLE) -> COORD;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberOfConsoleInputEvents(hconsoleinput: super::super::Foundation::HANDLE, lpnumberofevents: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStdHandle(nstdhandle: STD_HANDLE) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeekConsoleInputA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeekConsoleInputW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleInputA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleInputW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleOutputA(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *mut CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpreadregion: *mut SMALL_RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleOutputAttribute(hconsoleoutput: super::super::Foundation::HANDLE, lpattribute: *mut u16, nlength: u32, dwreadcoord: COORD, lpnumberofattrsread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleOutputCharacterA(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: super::super::Foundation::PSTR, nlength: u32, dwreadcoord: COORD, lpnumberofcharsread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleOutputCharacterW(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: super::super::Foundation::PWSTR, nlength: u32, dwreadcoord: COORD, lpnumberofcharsread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleOutputW(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *mut CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpreadregion: *mut SMALL_RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`*"]
    pub fn ResizePseudoConsole(hpc: HPCON, size: COORD) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScrollConsoleScreenBufferA(hconsoleoutput: super::super::Foundation::HANDLE, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: *const SMALL_RECT, dwdestinationorigin: COORD, lpfill: *const CHAR_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScrollConsoleScreenBufferW(hconsoleoutput: super::super::Foundation::HANDLE, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: *const SMALL_RECT, dwdestinationorigin: COORD, lpfill: *const CHAR_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleActiveScreenBuffer(hconsoleoutput: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleCP(wcodepageid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleCtrlHandler(handlerroutine: PHANDLER_ROUTINE, add: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleCursorInfo(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolecursorinfo: *const CONSOLE_CURSOR_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleCursorPosition(hconsoleoutput: super::super::Foundation::HANDLE, dwcursorposition: COORD) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleDisplayMode(hconsoleoutput: super::super::Foundation::HANDLE, dwflags: u32, lpnewscreenbufferdimensions: *mut COORD) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleHistoryInfo(lpconsolehistoryinfo: *const CONSOLE_HISTORY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleMode(hconsolehandle: super::super::Foundation::HANDLE, dwmode: CONSOLE_MODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleNumberOfCommandsA(number: u32, exename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleNumberOfCommandsW(number: u32, exename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleOutputCP(wcodepageid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleScreenBufferInfoEx(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolescreenbufferinfoex: *const CONSOLE_SCREEN_BUFFER_INFOEX) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleScreenBufferSize(hconsoleoutput: super::super::Foundation::HANDLE, dwsize: COORD) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleTextAttribute(hconsoleoutput: super::super::Foundation::HANDLE, wattributes: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleTitleA(lpconsoletitle: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleTitleW(lpconsoletitle: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleWindowInfo(hconsoleoutput: super::super::Foundation::HANDLE, babsolute: super::super::Foundation::BOOL, lpconsolewindow: *const SMALL_RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCurrentConsoleFontEx(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: super::super::Foundation::BOOL, lpconsolecurrentfontex: *const CONSOLE_FONT_INFOEX) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetStdHandle(nstdhandle: STD_HANDLE, hhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetStdHandleEx(nstdhandle: STD_HANDLE, hhandle: super::super::Foundation::HANDLE, phprevvalue: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleA(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *const ::core::ffi::c_void, nnumberofcharstowrite: u32, lpnumberofcharswritten: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleInputA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *const INPUT_RECORD, nlength: u32, lpnumberofeventswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleInputW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *const INPUT_RECORD, nlength: u32, lpnumberofeventswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleOutputA(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *const CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpwriteregion: *mut SMALL_RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleOutputAttribute(hconsoleoutput: super::super::Foundation::HANDLE, lpattribute: *const u16, nlength: u32, dwwritecoord: COORD, lpnumberofattrswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleOutputCharacterA(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: super::super::Foundation::PSTR, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleOutputCharacterW(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: super::super::Foundation::PWSTR, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleOutputW(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *const CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpwriteregion: *mut SMALL_RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Console`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleW(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *const ::core::ffi::c_void, nnumberofcharstowrite: u32, lpnumberofcharswritten: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
}
#[doc = "*Required features: `Win32_System_Console`*"]
pub const ALTNUMPAD_BIT: u32 = 67108864u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const ATTACH_PARENT_PROCESS: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const BACKGROUND_BLUE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const BACKGROUND_GREEN: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const BACKGROUND_INTENSITY: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const BACKGROUND_RED: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const CAPSLOCK_ON: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CHAR_INFO(i32);
#[doc = "*Required features: `Win32_System_Console`*"]
pub const COMMON_LVB_GRID_HORIZONTAL: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const COMMON_LVB_GRID_LVERTICAL: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const COMMON_LVB_GRID_RVERTICAL: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const COMMON_LVB_LEADING_BYTE: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const COMMON_LVB_REVERSE_VIDEO: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const COMMON_LVB_SBCSDBCS: u32 = 768u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const COMMON_LVB_TRAILING_BYTE: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const COMMON_LVB_UNDERSCORE: u32 = 32768u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CONSOLE_CURSOR_INFO(i32);
pub struct CONSOLE_FONT_INFO(i32);
pub struct CONSOLE_FONT_INFOEX(i32);
#[doc = "*Required features: `Win32_System_Console`*"]
pub const CONSOLE_FULLSCREEN: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const CONSOLE_FULLSCREEN_HARDWARE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const CONSOLE_FULLSCREEN_MODE: u32 = 1u32;
pub struct CONSOLE_HISTORY_INFO(i32);
pub struct CONSOLE_MODE(i32);
#[doc = "*Required features: `Win32_System_Console`*"]
pub const CONSOLE_MOUSE_DOWN: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const CONSOLE_MOUSE_SELECTION: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const CONSOLE_NO_SELECTION: u32 = 0u32;
pub struct CONSOLE_READCONSOLE_CONTROL(i32);
pub struct CONSOLE_SCREEN_BUFFER_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CONSOLE_SCREEN_BUFFER_INFOEX(i32);
pub struct CONSOLE_SELECTION_INFO(i32);
#[doc = "*Required features: `Win32_System_Console`*"]
pub const CONSOLE_SELECTION_IN_PROGRESS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const CONSOLE_SELECTION_NOT_EMPTY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const CONSOLE_TEXTMODE_BUFFER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const CONSOLE_WINDOWED_MODE: u32 = 2u32;
pub struct COORD(i32);
#[doc = "*Required features: `Win32_System_Console`*"]
pub const CTRL_BREAK_EVENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const CTRL_CLOSE_EVENT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const CTRL_C_EVENT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const CTRL_LOGOFF_EVENT: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const CTRL_SHUTDOWN_EVENT: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const DOUBLE_CLICK: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const ENHANCED_KEY: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const FOCUS_EVENT: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
pub struct FOCUS_EVENT_RECORD(i32);
#[doc = "*Required features: `Win32_System_Console`*"]
pub const FOREGROUND_BLUE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const FOREGROUND_GREEN: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const FOREGROUND_INTENSITY: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const FOREGROUND_RED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const FROM_LEFT_1ST_BUTTON_PRESSED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const FROM_LEFT_2ND_BUTTON_PRESSED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const FROM_LEFT_3RD_BUTTON_PRESSED: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const FROM_LEFT_4TH_BUTTON_PRESSED: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const HISTORY_NO_DUP_FLAG: u32 = 1u32;
pub struct HPCON(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct INPUT_RECORD(i32);
#[doc = "*Required features: `Win32_System_Console`*"]
pub const KEY_EVENT: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct KEY_EVENT_RECORD(i32);
#[doc = "*Required features: `Win32_System_Console`*"]
pub const LEFT_ALT_PRESSED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const LEFT_CTRL_PRESSED: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const MENU_EVENT: u32 = 8u32;
pub struct MENU_EVENT_RECORD(i32);
#[doc = "*Required features: `Win32_System_Console`*"]
pub const MOUSE_EVENT: u32 = 2u32;
pub struct MOUSE_EVENT_RECORD(i32);
#[doc = "*Required features: `Win32_System_Console`*"]
pub const MOUSE_HWHEELED: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const MOUSE_MOVED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const MOUSE_WHEELED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const NLS_ALPHANUMERIC: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const NLS_DBCSCHAR: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const NLS_HIRAGANA: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const NLS_IME_CONVERSION: u32 = 8388608u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const NLS_IME_DISABLE: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const NLS_KATAKANA: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const NLS_ROMAN: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const NUMLOCK_ON: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
pub struct PHANDLER_ROUTINE(i32);
#[doc = "*Required features: `Win32_System_Console`*"]
pub const PSEUDOCONSOLE_INHERIT_CURSOR: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const RIGHTMOST_BUTTON_PRESSED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const RIGHT_ALT_PRESSED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const RIGHT_CTRL_PRESSED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const SCROLLLOCK_ON: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Console`*"]
pub const SHIFT_PRESSED: u32 = 16u32;
pub struct SMALL_RECT(i32);
pub struct STD_HANDLE(i32);
#[doc = "*Required features: `Win32_System_Console`*"]
pub const WINDOW_BUFFER_SIZE_EVENT: u32 = 4u32;
pub struct WINDOW_BUFFER_SIZE_RECORD(i32);
