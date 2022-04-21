#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddConsoleAliasA(source: ::windows_sys::core::PCSTR, target: ::windows_sys::core::PCSTR, exename: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddConsoleAliasW(source: ::windows_sys::core::PCWSTR, target: ::windows_sys::core::PCWSTR, exename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocConsole() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AttachConsole(dwprocessid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn ClosePseudoConsole(hpc: HPCON);
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateConsoleScreenBuffer(dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwflags: u32, lpscreenbufferdata: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePseudoConsole(size: COORD, hinput: super::super::Foundation::HANDLE, houtput: super::super::Foundation::HANDLE, dwflags: u32, phpc: *mut HPCON) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn ExpungeConsoleCommandHistoryA(exename: ::windows_sys::core::PCSTR);
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn ExpungeConsoleCommandHistoryW(exename: ::windows_sys::core::PCWSTR);
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillConsoleOutputAttribute(hconsoleoutput: super::super::Foundation::HANDLE, wattribute: u16, nlength: u32, dwwritecoord: COORD, lpnumberofattrswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillConsoleOutputCharacterA(hconsoleoutput: super::super::Foundation::HANDLE, ccharacter: super::super::Foundation::CHAR, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillConsoleOutputCharacterW(hconsoleoutput: super::super::Foundation::HANDLE, ccharacter: u16, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushConsoleInputBuffer(hconsoleinput: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeConsole() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GenerateConsoleCtrlEvent(dwctrlevent: u32, dwprocessgroupid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleAliasA(source: ::windows_sys::core::PCSTR, targetbuffer: ::windows_sys::core::PSTR, targetbufferlength: u32, exename: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleAliasExesA(exenamebuffer: ::windows_sys::core::PSTR, exenamebufferlength: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleAliasExesLengthA() -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleAliasExesLengthW() -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleAliasExesW(exenamebuffer: ::windows_sys::core::PWSTR, exenamebufferlength: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleAliasW(source: ::windows_sys::core::PCWSTR, targetbuffer: ::windows_sys::core::PWSTR, targetbufferlength: u32, exename: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleAliasesA(aliasbuffer: ::windows_sys::core::PSTR, aliasbufferlength: u32, exename: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleAliasesLengthA(exename: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleAliasesLengthW(exename: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleAliasesW(aliasbuffer: ::windows_sys::core::PWSTR, aliasbufferlength: u32, exename: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleCP() -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleCommandHistoryA(commands: ::windows_sys::core::PSTR, commandbufferlength: u32, exename: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleCommandHistoryLengthA(exename: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleCommandHistoryLengthW(exename: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleCommandHistoryW(commands: ::windows_sys::core::PWSTR, commandbufferlength: u32, exename: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleCursorInfo(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolecursorinfo: *mut CONSOLE_CURSOR_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleDisplayMode(lpmodeflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleFontSize(hconsoleoutput: super::super::Foundation::HANDLE, nfont: u32) -> COORD;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleHistoryInfo(lpconsolehistoryinfo: *mut CONSOLE_HISTORY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleMode(hconsolehandle: super::super::Foundation::HANDLE, lpmode: *mut CONSOLE_MODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleOriginalTitleA(lpconsoletitle: ::windows_sys::core::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleOriginalTitleW(lpconsoletitle: ::windows_sys::core::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleOutputCP() -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleProcessList(lpdwprocesslist: *mut u32, dwprocesscount: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleScreenBufferInfo(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolescreenbufferinfo: *mut CONSOLE_SCREEN_BUFFER_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleScreenBufferInfoEx(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolescreenbufferinfoex: *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleSelectionInfo(lpconsoleselectioninfo: *mut CONSOLE_SELECTION_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleTitleA(lpconsoletitle: ::windows_sys::core::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn GetConsoleTitleW(lpconsoletitle: ::windows_sys::core::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetConsoleWindow() -> super::super::Foundation::HWND;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentConsoleFont(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: super::super::Foundation::BOOL, lpconsolecurrentfont: *mut CONSOLE_FONT_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentConsoleFontEx(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: super::super::Foundation::BOOL, lpconsolecurrentfontex: *mut CONSOLE_FONT_INFOEX) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLargestConsoleWindowSize(hconsoleoutput: super::super::Foundation::HANDLE) -> COORD;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberOfConsoleInputEvents(hconsoleinput: super::super::Foundation::HANDLE, lpnumberofevents: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStdHandle(nstdhandle: STD_HANDLE) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeekConsoleInputA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeekConsoleInputW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleInputA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleInputW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleOutputA(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *mut CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpreadregion: *mut SMALL_RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleOutputAttribute(hconsoleoutput: super::super::Foundation::HANDLE, lpattribute: *mut u16, nlength: u32, dwreadcoord: COORD, lpnumberofattrsread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleOutputCharacterA(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: ::windows_sys::core::PSTR, nlength: u32, dwreadcoord: COORD, lpnumberofcharsread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleOutputCharacterW(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: ::windows_sys::core::PWSTR, nlength: u32, dwreadcoord: COORD, lpnumberofcharsread: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleOutputW(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *mut CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpreadregion: *mut SMALL_RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadConsoleW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`*"]
    pub fn ResizePseudoConsole(hpc: HPCON, size: COORD) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScrollConsoleScreenBufferA(hconsoleoutput: super::super::Foundation::HANDLE, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: *const SMALL_RECT, dwdestinationorigin: COORD, lpfill: *const CHAR_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScrollConsoleScreenBufferW(hconsoleoutput: super::super::Foundation::HANDLE, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: *const SMALL_RECT, dwdestinationorigin: COORD, lpfill: *const CHAR_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleActiveScreenBuffer(hconsoleoutput: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleCP(wcodepageid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleCtrlHandler(handlerroutine: PHANDLER_ROUTINE, add: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleCursorInfo(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolecursorinfo: *const CONSOLE_CURSOR_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleCursorPosition(hconsoleoutput: super::super::Foundation::HANDLE, dwcursorposition: COORD) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleDisplayMode(hconsoleoutput: super::super::Foundation::HANDLE, dwflags: u32, lpnewscreenbufferdimensions: *mut COORD) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleHistoryInfo(lpconsolehistoryinfo: *const CONSOLE_HISTORY_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleMode(hconsolehandle: super::super::Foundation::HANDLE, dwmode: CONSOLE_MODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleNumberOfCommandsA(number: u32, exename: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleNumberOfCommandsW(number: u32, exename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleOutputCP(wcodepageid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleScreenBufferInfoEx(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolescreenbufferinfoex: *const CONSOLE_SCREEN_BUFFER_INFOEX) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleScreenBufferSize(hconsoleoutput: super::super::Foundation::HANDLE, dwsize: COORD) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleTextAttribute(hconsoleoutput: super::super::Foundation::HANDLE, wattributes: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleTitleA(lpconsoletitle: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleTitleW(lpconsoletitle: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetConsoleWindowInfo(hconsoleoutput: super::super::Foundation::HANDLE, babsolute: super::super::Foundation::BOOL, lpconsolewindow: *const SMALL_RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCurrentConsoleFontEx(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: super::super::Foundation::BOOL, lpconsolecurrentfontex: *const CONSOLE_FONT_INFOEX) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetStdHandle(nstdhandle: STD_HANDLE, hhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetStdHandleEx(nstdhandle: STD_HANDLE, hhandle: super::super::Foundation::HANDLE, phprevvalue: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleA(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *const ::core::ffi::c_void, nnumberofcharstowrite: u32, lpnumberofcharswritten: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleInputA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *const INPUT_RECORD, nlength: u32, lpnumberofeventswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleInputW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *const INPUT_RECORD, nlength: u32, lpnumberofeventswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleOutputA(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *const CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpwriteregion: *mut SMALL_RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleOutputAttribute(hconsoleoutput: super::super::Foundation::HANDLE, lpattribute: *const u16, nlength: u32, dwwritecoord: COORD, lpnumberofattrswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleOutputCharacterA(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: ::windows_sys::core::PCSTR, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleOutputCharacterW(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: ::windows_sys::core::PCWSTR, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleOutputW(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *const CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpwriteregion: *mut SMALL_RECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteConsoleW(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *const ::core::ffi::c_void, nnumberofcharstowrite: u32, lpnumberofcharswritten: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ALTNUMPAD_BIT: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ATTACH_PARENT_PROCESS: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const BACKGROUND_BLUE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const BACKGROUND_GREEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const BACKGROUND_INTENSITY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const BACKGROUND_RED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CAPSLOCK_ON: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHAR_INFO {
    pub Char: CHAR_INFO_0,
    pub Attributes: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHAR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHAR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CHAR_INFO_0 {
    pub UnicodeChar: u16,
    pub AsciiChar: super::super::Foundation::CHAR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHAR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHAR_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const COMMON_LVB_GRID_HORIZONTAL: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const COMMON_LVB_GRID_LVERTICAL: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const COMMON_LVB_GRID_RVERTICAL: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const COMMON_LVB_LEADING_BYTE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const COMMON_LVB_REVERSE_VIDEO: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const COMMON_LVB_SBCSDBCS: u32 = 768u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const COMMON_LVB_TRAILING_BYTE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const COMMON_LVB_UNDERSCORE: u32 = 32768u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CONSOLE_CURSOR_INFO {
    pub dwSize: u32,
    pub bVisible: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONSOLE_CURSOR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONSOLE_CURSOR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct CONSOLE_FONT_INFO {
    pub nFont: u32,
    pub dwFontSize: COORD,
}
impl ::core::marker::Copy for CONSOLE_FONT_INFO {}
impl ::core::clone::Clone for CONSOLE_FONT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct CONSOLE_FONT_INFOEX {
    pub cbSize: u32,
    pub nFont: u32,
    pub dwFontSize: COORD,
    pub FontFamily: u32,
    pub FontWeight: u32,
    pub FaceName: [u16; 32],
}
impl ::core::marker::Copy for CONSOLE_FONT_INFOEX {}
impl ::core::clone::Clone for CONSOLE_FONT_INFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_FULLSCREEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_FULLSCREEN_HARDWARE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_FULLSCREEN_MODE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct CONSOLE_HISTORY_INFO {
    pub cbSize: u32,
    pub HistoryBufferSize: u32,
    pub NumberOfHistoryBuffers: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for CONSOLE_HISTORY_INFO {}
impl ::core::clone::Clone for CONSOLE_HISTORY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub type CONSOLE_MODE = u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_PROCESSED_INPUT: CONSOLE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_LINE_INPUT: CONSOLE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_ECHO_INPUT: CONSOLE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_WINDOW_INPUT: CONSOLE_MODE = 8u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_MOUSE_INPUT: CONSOLE_MODE = 16u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_INSERT_MODE: CONSOLE_MODE = 32u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_QUICK_EDIT_MODE: CONSOLE_MODE = 64u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_EXTENDED_FLAGS: CONSOLE_MODE = 128u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_AUTO_POSITION: CONSOLE_MODE = 256u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: CONSOLE_MODE = 512u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_PROCESSED_OUTPUT: CONSOLE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_WRAP_AT_EOL_OUTPUT: CONSOLE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: CONSOLE_MODE = 4u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const DISABLE_NEWLINE_AUTO_RETURN: CONSOLE_MODE = 8u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENABLE_LVB_GRID_WORLDWIDE: CONSOLE_MODE = 16u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_MOUSE_DOWN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_MOUSE_SELECTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_NO_SELECTION: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct CONSOLE_READCONSOLE_CONTROL {
    pub nLength: u32,
    pub nInitialChars: u32,
    pub dwCtrlWakeupMask: u32,
    pub dwControlKeyState: u32,
}
impl ::core::marker::Copy for CONSOLE_READCONSOLE_CONTROL {}
impl ::core::clone::Clone for CONSOLE_READCONSOLE_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: u16,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
}
impl ::core::marker::Copy for CONSOLE_SCREEN_BUFFER_INFO {}
impl ::core::clone::Clone for CONSOLE_SCREEN_BUFFER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for CONSOLE_SCREEN_BUFFER_INFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct CONSOLE_SELECTION_INFO {
    pub dwFlags: u32,
    pub dwSelectionAnchor: COORD,
    pub srSelection: SMALL_RECT,
}
impl ::core::marker::Copy for CONSOLE_SELECTION_INFO {}
impl ::core::clone::Clone for CONSOLE_SELECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_SELECTION_IN_PROGRESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_SELECTION_NOT_EMPTY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_TEXTMODE_BUFFER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CONSOLE_WINDOWED_MODE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct COORD {
    pub X: i16,
    pub Y: i16,
}
impl ::core::marker::Copy for COORD {}
impl ::core::clone::Clone for COORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CTRL_BREAK_EVENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CTRL_CLOSE_EVENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CTRL_C_EVENT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CTRL_LOGOFF_EVENT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const CTRL_SHUTDOWN_EVENT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const DOUBLE_CLICK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const ENHANCED_KEY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FOCUS_EVENT: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FOCUS_EVENT_RECORD {
    pub bSetFocus: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FOCUS_EVENT_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FOCUS_EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FOREGROUND_BLUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FOREGROUND_GREEN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FOREGROUND_INTENSITY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FOREGROUND_RED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FROM_LEFT_1ST_BUTTON_PRESSED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FROM_LEFT_2ND_BUTTON_PRESSED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FROM_LEFT_3RD_BUTTON_PRESSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const FROM_LEFT_4TH_BUTTON_PRESSED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const HISTORY_NO_DUP_FLAG: u32 = 1u32;
pub type HPCON = isize;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct INPUT_RECORD {
    pub EventType: u16,
    pub Event: INPUT_RECORD_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INPUT_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INPUT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union INPUT_RECORD_0 {
    pub KeyEvent: KEY_EVENT_RECORD,
    pub MouseEvent: MOUSE_EVENT_RECORD,
    pub WindowBufferSizeEvent: WINDOW_BUFFER_SIZE_RECORD,
    pub MenuEvent: MENU_EVENT_RECORD,
    pub FocusEvent: FOCUS_EVENT_RECORD,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INPUT_RECORD_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INPUT_RECORD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const KEY_EVENT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KEY_EVENT_RECORD {
    pub bKeyDown: super::super::Foundation::BOOL,
    pub wRepeatCount: u16,
    pub wVirtualKeyCode: u16,
    pub wVirtualScanCode: u16,
    pub uChar: KEY_EVENT_RECORD_0,
    pub dwControlKeyState: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KEY_EVENT_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KEY_EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union KEY_EVENT_RECORD_0 {
    pub UnicodeChar: u16,
    pub AsciiChar: super::super::Foundation::CHAR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KEY_EVENT_RECORD_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KEY_EVENT_RECORD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const LEFT_ALT_PRESSED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const LEFT_CTRL_PRESSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const MENU_EVENT: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct MENU_EVENT_RECORD {
    pub dwCommandId: u32,
}
impl ::core::marker::Copy for MENU_EVENT_RECORD {}
impl ::core::clone::Clone for MENU_EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const MOUSE_EVENT: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct MOUSE_EVENT_RECORD {
    pub dwMousePosition: COORD,
    pub dwButtonState: u32,
    pub dwControlKeyState: u32,
    pub dwEventFlags: u32,
}
impl ::core::marker::Copy for MOUSE_EVENT_RECORD {}
impl ::core::clone::Clone for MOUSE_EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const MOUSE_HWHEELED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const MOUSE_MOVED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const MOUSE_WHEELED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const NLS_ALPHANUMERIC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const NLS_DBCSCHAR: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const NLS_HIRAGANA: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const NLS_IME_CONVERSION: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const NLS_IME_DISABLE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const NLS_KATAKANA: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const NLS_ROMAN: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const NUMLOCK_ON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Console\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PHANDLER_ROUTINE = ::core::option::Option<unsafe extern "system" fn(ctrltype: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const PSEUDOCONSOLE_INHERIT_CURSOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const RIGHTMOST_BUTTON_PRESSED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const RIGHT_ALT_PRESSED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const RIGHT_CTRL_PRESSED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const SCROLLLOCK_ON: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const SHIFT_PRESSED: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
impl ::core::marker::Copy for SMALL_RECT {}
impl ::core::clone::Clone for SMALL_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub type STD_HANDLE = u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const STD_INPUT_HANDLE: STD_HANDLE = 4294967286u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const STD_OUTPUT_HANDLE: STD_HANDLE = 4294967285u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const STD_ERROR_HANDLE: STD_HANDLE = 4294967284u32;
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub const WINDOW_BUFFER_SIZE_EVENT: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Console\"`*"]
pub struct WINDOW_BUFFER_SIZE_RECORD {
    pub dwSize: COORD,
}
impl ::core::marker::Copy for WINDOW_BUFFER_SIZE_RECORD {}
impl ::core::clone::Clone for WINDOW_BUFFER_SIZE_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
