pub type AddConsoleAliasA = unsafe extern "system" fn(source: windows_sys::core::PCSTR, target: windows_sys::core::PCSTR, exename: windows_sys::core::PCSTR) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn AddConsoleAliasA(source : windows_sys::core::PCSTR, target : windows_sys::core::PCSTR, exename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
pub type AddConsoleAliasW = unsafe extern "system" fn(source: windows_sys::core::PCWSTR, target: windows_sys::core::PCWSTR, exename: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn AddConsoleAliasW(source : windows_sys::core::PCWSTR, target : windows_sys::core::PCWSTR, exename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
pub type AllocConsole = unsafe extern "system" fn() -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn AllocConsole() -> windows_sys::core::BOOL);
pub type AllocConsoleWithOptions = unsafe extern "system" fn(options: *const ALLOC_CONSOLE_OPTIONS, result: *mut ALLOC_CONSOLE_RESULT) -> windows_sys::core::HRESULT;
windows_link::link!("kernel32.dll" "system" fn AllocConsoleWithOptions(options : *const ALLOC_CONSOLE_OPTIONS, result : *mut ALLOC_CONSOLE_RESULT) -> windows_sys::core::HRESULT);
pub type AttachConsole = unsafe extern "system" fn(dwprocessid: u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn AttachConsole(dwprocessid : u32) -> windows_sys::core::BOOL);
pub type CloseConsoleHandle = unsafe extern "system" fn(hconsole: super::super::Foundation::HANDLE) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn CloseConsoleHandle(hconsole : super::super::Foundation::HANDLE) -> windows_sys::core::BOOL);
pub type ClosePseudoConsole = unsafe extern "system" fn(hpc: HPCON);
windows_link::link!("kernel32.dll" "system" fn ClosePseudoConsole(hpc : HPCON));
pub type ConsoleControl = unsafe extern "system" fn(command: CONSOLECONTROL, consoleinformation: *const core::ffi::c_void, consoleinformationlength: u32) -> super::super::Foundation::NTSTATUS;
windows_link::link!("user32.dll" "system" fn ConsoleControl(command : CONSOLECONTROL, consoleinformation : *const core::ffi::c_void, consoleinformationlength : u32) -> super::super::Foundation::NTSTATUS);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type ConsoleMenuControl = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, dwcommandidlow: u32, dwcommandidhigh: u32) -> super::super::UI::WindowsAndMessaging::HMENU;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_link::link!("kernel32.dll" "system" fn ConsoleMenuControl(hconsoleoutput : super::super::Foundation::HANDLE, dwcommandidlow : u32, dwcommandidhigh : u32) -> super::super::UI::WindowsAndMessaging::HMENU);
#[cfg(feature = "Win32_Security")]
pub type CreateConsoleScreenBuffer = unsafe extern "system" fn(dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwflags: u32, lpscreenbufferdata: *const core::ffi::c_void) -> super::super::Foundation::HANDLE;
#[cfg(feature = "Win32_Security")]
windows_link::link!("kernel32.dll" "system" fn CreateConsoleScreenBuffer(dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::super::Security::SECURITY_ATTRIBUTES, dwflags : u32, lpscreenbufferdata : *const core::ffi::c_void) -> super::super::Foundation::HANDLE);
pub type CreatePseudoConsole = unsafe extern "system" fn(size: COORD, hinput: super::super::Foundation::HANDLE, houtput: super::super::Foundation::HANDLE, dwflags: u32, phpc: *mut HPCON) -> windows_sys::core::HRESULT;
windows_link::link!("kernel32.dll" "system" fn CreatePseudoConsole(size : COORD, hinput : super::super::Foundation::HANDLE, houtput : super::super::Foundation::HANDLE, dwflags : u32, phpc : *mut HPCON) -> windows_sys::core::HRESULT);
pub type DuplicateConsoleHandle = unsafe extern "system" fn(hsourcehandle: super::super::Foundation::HANDLE, dwdesiredaccess: u32, binherithandle: windows_sys::core::BOOL, dwoptions: u32) -> super::super::Foundation::HANDLE;
windows_link::link!("kernel32.dll" "system" fn DuplicateConsoleHandle(hsourcehandle : super::super::Foundation::HANDLE, dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, dwoptions : u32) -> super::super::Foundation::HANDLE);
pub type ExpungeConsoleCommandHistoryA = unsafe extern "system" fn(exename: windows_sys::core::PCSTR);
windows_link::link!("kernel32.dll" "system" fn ExpungeConsoleCommandHistoryA(exename : windows_sys::core::PCSTR));
pub type ExpungeConsoleCommandHistoryW = unsafe extern "system" fn(exename: windows_sys::core::PCWSTR);
windows_link::link!("kernel32.dll" "system" fn ExpungeConsoleCommandHistoryW(exename : windows_sys::core::PCWSTR));
pub type FillConsoleOutputAttribute = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, wattribute: u16, nlength: u32, dwwritecoord: COORD, lpnumberofattrswritten: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn FillConsoleOutputAttribute(hconsoleoutput : super::super::Foundation::HANDLE, wattribute : u16, nlength : u32, dwwritecoord : COORD, lpnumberofattrswritten : *mut u32) -> windows_sys::core::BOOL);
pub type FillConsoleOutputCharacterA = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, ccharacter: i8, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn FillConsoleOutputCharacterA(hconsoleoutput : super::super::Foundation::HANDLE, ccharacter : i8, nlength : u32, dwwritecoord : COORD, lpnumberofcharswritten : *mut u32) -> windows_sys::core::BOOL);
pub type FillConsoleOutputCharacterW = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, ccharacter: u16, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn FillConsoleOutputCharacterW(hconsoleoutput : super::super::Foundation::HANDLE, ccharacter : u16, nlength : u32, dwwritecoord : COORD, lpnumberofcharswritten : *mut u32) -> windows_sys::core::BOOL);
pub type FlushConsoleInputBuffer = unsafe extern "system" fn(hconsoleinput: super::super::Foundation::HANDLE) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn FlushConsoleInputBuffer(hconsoleinput : super::super::Foundation::HANDLE) -> windows_sys::core::BOOL);
pub type FreeConsole = unsafe extern "system" fn() -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn FreeConsole() -> windows_sys::core::BOOL);
pub type GenerateConsoleCtrlEvent = unsafe extern "system" fn(dwctrlevent: u32, dwprocessgroupid: u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GenerateConsoleCtrlEvent(dwctrlevent : u32, dwprocessgroupid : u32) -> windows_sys::core::BOOL);
pub type GetConsoleAliasA = unsafe extern "system" fn(source: windows_sys::core::PCSTR, targetbuffer: windows_sys::core::PSTR, targetbufferlength: u32, exename: windows_sys::core::PCSTR) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasA(source : windows_sys::core::PCSTR, targetbuffer : windows_sys::core::PSTR, targetbufferlength : u32, exename : windows_sys::core::PCSTR) -> u32);
pub type GetConsoleAliasExesA = unsafe extern "system" fn(exenamebuffer: windows_sys::core::PSTR, exenamebufferlength: u32) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasExesA(exenamebuffer : windows_sys::core::PSTR, exenamebufferlength : u32) -> u32);
pub type GetConsoleAliasExesLengthA = unsafe extern "system" fn() -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasExesLengthA() -> u32);
pub type GetConsoleAliasExesLengthW = unsafe extern "system" fn() -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasExesLengthW() -> u32);
pub type GetConsoleAliasExesW = unsafe extern "system" fn(exenamebuffer: windows_sys::core::PWSTR, exenamebufferlength: u32) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasExesW(exenamebuffer : windows_sys::core::PWSTR, exenamebufferlength : u32) -> u32);
pub type GetConsoleAliasW = unsafe extern "system" fn(source: windows_sys::core::PCWSTR, targetbuffer: windows_sys::core::PWSTR, targetbufferlength: u32, exename: windows_sys::core::PCWSTR) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasW(source : windows_sys::core::PCWSTR, targetbuffer : windows_sys::core::PWSTR, targetbufferlength : u32, exename : windows_sys::core::PCWSTR) -> u32);
pub type GetConsoleAliasesA = unsafe extern "system" fn(aliasbuffer: windows_sys::core::PSTR, aliasbufferlength: u32, exename: windows_sys::core::PCSTR) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasesA(aliasbuffer : windows_sys::core::PSTR, aliasbufferlength : u32, exename : windows_sys::core::PCSTR) -> u32);
pub type GetConsoleAliasesLengthA = unsafe extern "system" fn(exename: windows_sys::core::PCSTR) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasesLengthA(exename : windows_sys::core::PCSTR) -> u32);
pub type GetConsoleAliasesLengthW = unsafe extern "system" fn(exename: windows_sys::core::PCWSTR) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasesLengthW(exename : windows_sys::core::PCWSTR) -> u32);
pub type GetConsoleAliasesW = unsafe extern "system" fn(aliasbuffer: windows_sys::core::PWSTR, aliasbufferlength: u32, exename: windows_sys::core::PCWSTR) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleAliasesW(aliasbuffer : windows_sys::core::PWSTR, aliasbufferlength : u32, exename : windows_sys::core::PCWSTR) -> u32);
pub type GetConsoleCP = unsafe extern "system" fn() -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleCP() -> u32);
pub type GetConsoleCharType = unsafe extern "system" fn(hconsole: super::super::Foundation::HANDLE, coordcheck: COORD, pdwtype: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GetConsoleCharType(hconsole : super::super::Foundation::HANDLE, coordcheck : COORD, pdwtype : *mut u32) -> windows_sys::core::BOOL);
pub type GetConsoleCommandHistoryA = unsafe extern "system" fn(commands: windows_sys::core::PSTR, commandbufferlength: u32, exename: windows_sys::core::PCSTR) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryA(commands : windows_sys::core::PSTR, commandbufferlength : u32, exename : windows_sys::core::PCSTR) -> u32);
pub type GetConsoleCommandHistoryLengthA = unsafe extern "system" fn(exename: windows_sys::core::PCSTR) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryLengthA(exename : windows_sys::core::PCSTR) -> u32);
pub type GetConsoleCommandHistoryLengthW = unsafe extern "system" fn(exename: windows_sys::core::PCWSTR) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryLengthW(exename : windows_sys::core::PCWSTR) -> u32);
pub type GetConsoleCommandHistoryW = unsafe extern "system" fn(commands: windows_sys::core::PWSTR, commandbufferlength: u32, exename: windows_sys::core::PCWSTR) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleCommandHistoryW(commands : windows_sys::core::PWSTR, commandbufferlength : u32, exename : windows_sys::core::PCWSTR) -> u32);
pub type GetConsoleCursorInfo = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolecursorinfo: *mut CONSOLE_CURSOR_INFO) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GetConsoleCursorInfo(hconsoleoutput : super::super::Foundation::HANDLE, lpconsolecursorinfo : *mut CONSOLE_CURSOR_INFO) -> windows_sys::core::BOOL);
pub type GetConsoleCursorMode = unsafe extern "system" fn(hconsolehandle: super::super::Foundation::HANDLE, pbblink: *mut windows_sys::core::BOOL, pbdbenable: *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GetConsoleCursorMode(hconsolehandle : super::super::Foundation::HANDLE, pbblink : *mut windows_sys::core::BOOL, pbdbenable : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
pub type GetConsoleDisplayMode = unsafe extern "system" fn(lpmodeflags: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GetConsoleDisplayMode(lpmodeflags : *mut u32) -> windows_sys::core::BOOL);
pub type GetConsoleFontInfo = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: windows_sys::core::BOOL, nlength: u32, lpconsolefontinfo: *mut CONSOLE_FONT_INFO) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleFontInfo(hconsoleoutput : super::super::Foundation::HANDLE, bmaximumwindow : windows_sys::core::BOOL, nlength : u32, lpconsolefontinfo : *mut CONSOLE_FONT_INFO) -> u32);
pub type GetConsoleFontSize = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, nfont: u32) -> COORD;
windows_link::link!("kernel32.dll" "system" fn GetConsoleFontSize(hconsoleoutput : super::super::Foundation::HANDLE, nfont : u32) -> COORD);
pub type GetConsoleHardwareState = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpresolution: *mut COORD, lpfontsize: *mut COORD) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GetConsoleHardwareState(hconsoleoutput : super::super::Foundation::HANDLE, lpresolution : *mut COORD, lpfontsize : *mut COORD) -> windows_sys::core::BOOL);
pub type GetConsoleHistoryInfo = unsafe extern "system" fn(lpconsolehistoryinfo: *mut CONSOLE_HISTORY_INFO) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GetConsoleHistoryInfo(lpconsolehistoryinfo : *mut CONSOLE_HISTORY_INFO) -> windows_sys::core::BOOL);
pub type GetConsoleInputExeNameA = unsafe extern "system" fn(nbufferlength: u32, lpbuffer: windows_sys::core::PSTR) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleInputExeNameA(nbufferlength : u32, lpbuffer : windows_sys::core::PSTR) -> u32);
pub type GetConsoleInputExeNameW = unsafe extern "system" fn(nbufferlength: u32, lpbuffer: windows_sys::core::PWSTR) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleInputExeNameW(nbufferlength : u32, lpbuffer : windows_sys::core::PWSTR) -> u32);
pub type GetConsoleInputWaitHandle = unsafe extern "system" fn() -> super::super::Foundation::HANDLE;
windows_link::link!("kernel32.dll" "system" fn GetConsoleInputWaitHandle() -> super::super::Foundation::HANDLE);
pub type GetConsoleKeyboardLayoutNameA = unsafe extern "system" fn(pszlayout: windows_sys::core::PSTR) -> windows_sys::core::BOOL;
windows_link::link!("user32.dll" "system" fn GetConsoleKeyboardLayoutNameA(pszlayout : windows_sys::core::PSTR) -> windows_sys::core::BOOL);
pub type GetConsoleKeyboardLayoutNameW = unsafe extern "system" fn(pszlayout: windows_sys::core::PWSTR) -> windows_sys::core::BOOL;
windows_link::link!("user32.dll" "system" fn GetConsoleKeyboardLayoutNameW(pszlayout : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
pub type GetConsoleMode = unsafe extern "system" fn(hconsolehandle: super::super::Foundation::HANDLE, lpmode: *mut CONSOLE_MODE) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GetConsoleMode(hconsolehandle : super::super::Foundation::HANDLE, lpmode : *mut CONSOLE_MODE) -> windows_sys::core::BOOL);
pub type GetConsoleNlsMode = unsafe extern "system" fn(hconsole: super::super::Foundation::HANDLE, lpdwnlsmode: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GetConsoleNlsMode(hconsole : super::super::Foundation::HANDLE, lpdwnlsmode : *mut u32) -> windows_sys::core::BOOL);
pub type GetConsoleOriginalTitleA = unsafe extern "system" fn(lpconsoletitle: windows_sys::core::PSTR, nsize: u32) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleOriginalTitleA(lpconsoletitle : windows_sys::core::PSTR, nsize : u32) -> u32);
pub type GetConsoleOriginalTitleW = unsafe extern "system" fn(lpconsoletitle: windows_sys::core::PWSTR, nsize: u32) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleOriginalTitleW(lpconsoletitle : windows_sys::core::PWSTR, nsize : u32) -> u32);
pub type GetConsoleOutputCP = unsafe extern "system" fn() -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleOutputCP() -> u32);
pub type GetConsoleProcessList = unsafe extern "system" fn(lpdwprocesslist: *mut u32, dwprocesscount: u32) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleProcessList(lpdwprocesslist : *mut u32, dwprocesscount : u32) -> u32);
pub type GetConsoleScreenBufferInfo = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolescreenbufferinfo: *mut CONSOLE_SCREEN_BUFFER_INFO) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GetConsoleScreenBufferInfo(hconsoleoutput : super::super::Foundation::HANDLE, lpconsolescreenbufferinfo : *mut CONSOLE_SCREEN_BUFFER_INFO) -> windows_sys::core::BOOL);
pub type GetConsoleScreenBufferInfoEx = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolescreenbufferinfoex: *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GetConsoleScreenBufferInfoEx(hconsoleoutput : super::super::Foundation::HANDLE, lpconsolescreenbufferinfoex : *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_sys::core::BOOL);
pub type GetConsoleSelectionInfo = unsafe extern "system" fn(lpconsoleselectioninfo: *mut CONSOLE_SELECTION_INFO) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GetConsoleSelectionInfo(lpconsoleselectioninfo : *mut CONSOLE_SELECTION_INFO) -> windows_sys::core::BOOL);
pub type GetConsoleTitleA = unsafe extern "system" fn(lpconsoletitle: windows_sys::core::PSTR, nsize: u32) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleTitleA(lpconsoletitle : windows_sys::core::PSTR, nsize : u32) -> u32);
pub type GetConsoleTitleW = unsafe extern "system" fn(lpconsoletitle: windows_sys::core::PWSTR, nsize: u32) -> u32;
windows_link::link!("kernel32.dll" "system" fn GetConsoleTitleW(lpconsoletitle : windows_sys::core::PWSTR, nsize : u32) -> u32);
pub type GetConsoleWindow = unsafe extern "system" fn() -> super::super::Foundation::HWND;
windows_link::link!("kernel32.dll" "system" fn GetConsoleWindow() -> super::super::Foundation::HWND);
pub type GetCurrentConsoleFont = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: windows_sys::core::BOOL, lpconsolecurrentfont: *mut CONSOLE_FONT_INFO) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GetCurrentConsoleFont(hconsoleoutput : super::super::Foundation::HANDLE, bmaximumwindow : windows_sys::core::BOOL, lpconsolecurrentfont : *mut CONSOLE_FONT_INFO) -> windows_sys::core::BOOL);
pub type GetCurrentConsoleFontEx = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: windows_sys::core::BOOL, lpconsolecurrentfontex: *mut CONSOLE_FONT_INFOEX) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GetCurrentConsoleFontEx(hconsoleoutput : super::super::Foundation::HANDLE, bmaximumwindow : windows_sys::core::BOOL, lpconsolecurrentfontex : *mut CONSOLE_FONT_INFOEX) -> windows_sys::core::BOOL);
pub type GetLargestConsoleWindowSize = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE) -> COORD;
windows_link::link!("kernel32.dll" "system" fn GetLargestConsoleWindowSize(hconsoleoutput : super::super::Foundation::HANDLE) -> COORD);
pub type GetNumberOfConsoleFonts = unsafe extern "system" fn() -> u32;
windows_link::link!("kernel32.dll" "system" fn GetNumberOfConsoleFonts() -> u32);
pub type GetNumberOfConsoleInputEvents = unsafe extern "system" fn(hconsoleinput: super::super::Foundation::HANDLE, lpnumberofevents: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GetNumberOfConsoleInputEvents(hconsoleinput : super::super::Foundation::HANDLE, lpnumberofevents : *mut u32) -> windows_sys::core::BOOL);
pub type GetNumberOfConsoleMouseButtons = unsafe extern "system" fn(lpnumberofmousebuttons: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons : *mut u32) -> windows_sys::core::BOOL);
pub type GetStdHandle = unsafe extern "system" fn(nstdhandle: STD_HANDLE) -> super::super::Foundation::HANDLE;
windows_link::link!("kernel32.dll" "system" fn GetStdHandle(nstdhandle : STD_HANDLE) -> super::super::Foundation::HANDLE);
pub type InvalidateConsoleDIBits = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lprect: *const SMALL_RECT) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn InvalidateConsoleDIBits(hconsoleoutput : super::super::Foundation::HANDLE, lprect : *const SMALL_RECT) -> windows_sys::core::BOOL);
pub type OpenConsoleW = unsafe extern "system" fn(lpconsoledevice: windows_sys::core::PCWSTR, dwdesiredaccess: u32, binherithandle: windows_sys::core::BOOL, dwsharemode: u32) -> super::super::Foundation::HANDLE;
windows_link::link!("kernel32.dll" "system" fn OpenConsoleW(lpconsoledevice : windows_sys::core::PCWSTR, dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, dwsharemode : u32) -> super::super::Foundation::HANDLE);
pub type PeekConsoleInputA = unsafe extern "system" fn(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn PeekConsoleInputA(hconsoleinput : super::super::Foundation::HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_sys::core::BOOL);
pub type PeekConsoleInputW = unsafe extern "system" fn(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn PeekConsoleInputW(hconsoleinput : super::super::Foundation::HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_sys::core::BOOL);
pub type ReadConsoleA = unsafe extern "system" fn(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn ReadConsoleA(hconsoleinput : super::super::Foundation::HANDLE, lpbuffer : *mut core::ffi::c_void, nnumberofcharstoread : u32, lpnumberofcharsread : *mut u32, pinputcontrol : *const CONSOLE_READCONSOLE_CONTROL) -> windows_sys::core::BOOL);
pub type ReadConsoleInputA = unsafe extern "system" fn(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn ReadConsoleInputA(hconsoleinput : super::super::Foundation::HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_sys::core::BOOL);
pub type ReadConsoleInputExA = unsafe extern "system" fn(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32, wflags: u16) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn ReadConsoleInputExA(hconsoleinput : super::super::Foundation::HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32, wflags : u16) -> windows_sys::core::BOOL);
pub type ReadConsoleInputExW = unsafe extern "system" fn(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32, wflags: u16) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn ReadConsoleInputExW(hconsoleinput : super::super::Foundation::HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32, wflags : u16) -> windows_sys::core::BOOL);
pub type ReadConsoleInputW = unsafe extern "system" fn(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn ReadConsoleInputW(hconsoleinput : super::super::Foundation::HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_sys::core::BOOL);
pub type ReadConsoleOutputA = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *mut CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpreadregion: *mut SMALL_RECT) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn ReadConsoleOutputA(hconsoleoutput : super::super::Foundation::HANDLE, lpbuffer : *mut CHAR_INFO, dwbuffersize : COORD, dwbuffercoord : COORD, lpreadregion : *mut SMALL_RECT) -> windows_sys::core::BOOL);
pub type ReadConsoleOutputAttribute = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpattribute: *mut u16, nlength: u32, dwreadcoord: COORD, lpnumberofattrsread: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn ReadConsoleOutputAttribute(hconsoleoutput : super::super::Foundation::HANDLE, lpattribute : *mut u16, nlength : u32, dwreadcoord : COORD, lpnumberofattrsread : *mut u32) -> windows_sys::core::BOOL);
pub type ReadConsoleOutputCharacterA = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: windows_sys::core::PSTR, nlength: u32, dwreadcoord: COORD, lpnumberofcharsread: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn ReadConsoleOutputCharacterA(hconsoleoutput : super::super::Foundation::HANDLE, lpcharacter : windows_sys::core::PSTR, nlength : u32, dwreadcoord : COORD, lpnumberofcharsread : *mut u32) -> windows_sys::core::BOOL);
pub type ReadConsoleOutputCharacterW = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: windows_sys::core::PWSTR, nlength: u32, dwreadcoord: COORD, lpnumberofcharsread: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn ReadConsoleOutputCharacterW(hconsoleoutput : super::super::Foundation::HANDLE, lpcharacter : windows_sys::core::PWSTR, nlength : u32, dwreadcoord : COORD, lpnumberofcharsread : *mut u32) -> windows_sys::core::BOOL);
pub type ReadConsoleOutputW = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *mut CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpreadregion: *mut SMALL_RECT) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn ReadConsoleOutputW(hconsoleoutput : super::super::Foundation::HANDLE, lpbuffer : *mut CHAR_INFO, dwbuffersize : COORD, dwbuffercoord : COORD, lpreadregion : *mut SMALL_RECT) -> windows_sys::core::BOOL);
pub type ReadConsoleW = unsafe extern "system" fn(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn ReadConsoleW(hconsoleinput : super::super::Foundation::HANDLE, lpbuffer : *mut core::ffi::c_void, nnumberofcharstoread : u32, lpnumberofcharsread : *mut u32, pinputcontrol : *const CONSOLE_READCONSOLE_CONTROL) -> windows_sys::core::BOOL);
pub type RegisterConsoleIME = unsafe extern "system" fn(hwndconsoleime: super::super::Foundation::HWND, lpdwconsolethreadid: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn RegisterConsoleIME(hwndconsoleime : super::super::Foundation::HWND, lpdwconsolethreadid : *mut u32) -> windows_sys::core::BOOL);
pub type RegisterConsoleOS2 = unsafe extern "system" fn(fos2register: windows_sys::core::BOOL) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn RegisterConsoleOS2(fos2register : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
pub type RegisterConsoleVDM = unsafe extern "system" fn(dwregisterflags: u32, hstarthardwareevent: super::super::Foundation::HANDLE, hendhardwareevent: super::super::Foundation::HANDLE, herrorhardwareevent: super::super::Foundation::HANDLE, reserved: u32, lpstatelength: *mut u32, lpstate: *mut *mut core::ffi::c_void, vdmbuffersize: COORD, lpvdmbuffer: *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn RegisterConsoleVDM(dwregisterflags : u32, hstarthardwareevent : super::super::Foundation::HANDLE, hendhardwareevent : super::super::Foundation::HANDLE, herrorhardwareevent : super::super::Foundation::HANDLE, reserved : u32, lpstatelength : *mut u32, lpstate : *mut *mut core::ffi::c_void, vdmbuffersize : COORD, lpvdmbuffer : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
pub type ReleasePseudoConsole = unsafe extern "system" fn(hpc: HPCON) -> windows_sys::core::HRESULT;
windows_link::link!("kernel32.dll" "system" fn ReleasePseudoConsole(hpc : HPCON) -> windows_sys::core::HRESULT);
pub type ResizePseudoConsole = unsafe extern "system" fn(hpc: HPCON, size: COORD) -> windows_sys::core::HRESULT;
windows_link::link!("kernel32.dll" "system" fn ResizePseudoConsole(hpc : HPCON, size : COORD) -> windows_sys::core::HRESULT);
pub type ScrollConsoleScreenBufferA = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: *const SMALL_RECT, dwdestinationorigin: COORD, lpfill: *const CHAR_INFO) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn ScrollConsoleScreenBufferA(hconsoleoutput : super::super::Foundation::HANDLE, lpscrollrectangle : *const SMALL_RECT, lpcliprectangle : *const SMALL_RECT, dwdestinationorigin : COORD, lpfill : *const CHAR_INFO) -> windows_sys::core::BOOL);
pub type ScrollConsoleScreenBufferW = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: *const SMALL_RECT, dwdestinationorigin: COORD, lpfill: *const CHAR_INFO) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn ScrollConsoleScreenBufferW(hconsoleoutput : super::super::Foundation::HANDLE, lpscrollrectangle : *const SMALL_RECT, lpcliprectangle : *const SMALL_RECT, dwdestinationorigin : COORD, lpfill : *const CHAR_INFO) -> windows_sys::core::BOOL);
pub type SetConsoleActiveScreenBuffer = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleActiveScreenBuffer(hconsoleoutput : super::super::Foundation::HANDLE) -> windows_sys::core::BOOL);
pub type SetConsoleCP = unsafe extern "system" fn(wcodepageid: u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleCP(wcodepageid : u32) -> windows_sys::core::BOOL);
pub type SetConsoleCtrlHandler = unsafe extern "system" fn(handlerroutine: PHANDLER_ROUTINE, add: windows_sys::core::BOOL) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleCtrlHandler(handlerroutine : PHANDLER_ROUTINE, add : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type SetConsoleCursor = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, hcursor: super::super::UI::WindowsAndMessaging::HCURSOR) -> windows_sys::core::BOOL;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_link::link!("kernel32.dll" "system" fn SetConsoleCursor(hconsoleoutput : super::super::Foundation::HANDLE, hcursor : super::super::UI::WindowsAndMessaging::HCURSOR) -> windows_sys::core::BOOL);
pub type SetConsoleCursorInfo = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolecursorinfo: *const CONSOLE_CURSOR_INFO) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleCursorInfo(hconsoleoutput : super::super::Foundation::HANDLE, lpconsolecursorinfo : *const CONSOLE_CURSOR_INFO) -> windows_sys::core::BOOL);
pub type SetConsoleCursorMode = unsafe extern "system" fn(hconsolehandle: super::super::Foundation::HANDLE, blink: windows_sys::core::BOOL, dbenable: windows_sys::core::BOOL) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleCursorMode(hconsolehandle : super::super::Foundation::HANDLE, blink : windows_sys::core::BOOL, dbenable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
pub type SetConsoleCursorPosition = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, dwcursorposition: COORD) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleCursorPosition(hconsoleoutput : super::super::Foundation::HANDLE, dwcursorposition : COORD) -> windows_sys::core::BOOL);
pub type SetConsoleDisplayMode = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, dwflags: u32, lpnewscreenbufferdimensions: *mut COORD) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleDisplayMode(hconsoleoutput : super::super::Foundation::HANDLE, dwflags : u32, lpnewscreenbufferdimensions : *mut COORD) -> windows_sys::core::BOOL);
pub type SetConsoleFont = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, nfont: u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleFont(hconsoleoutput : super::super::Foundation::HANDLE, nfont : u32) -> windows_sys::core::BOOL);
pub type SetConsoleHardwareState = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, dwresolution: COORD, dwfontsize: COORD) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleHardwareState(hconsoleoutput : super::super::Foundation::HANDLE, dwresolution : COORD, dwfontsize : COORD) -> windows_sys::core::BOOL);
pub type SetConsoleHistoryInfo = unsafe extern "system" fn(lpconsolehistoryinfo: *const CONSOLE_HISTORY_INFO) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleHistoryInfo(lpconsolehistoryinfo : *const CONSOLE_HISTORY_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type SetConsoleIcon = unsafe extern "system" fn(hicon: super::super::UI::WindowsAndMessaging::HICON) -> windows_sys::core::BOOL;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_link::link!("kernel32.dll" "system" fn SetConsoleIcon(hicon : super::super::UI::WindowsAndMessaging::HICON) -> windows_sys::core::BOOL);
pub type SetConsoleInputExeNameA = unsafe extern "system" fn(lpexename: windows_sys::core::PCSTR) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleInputExeNameA(lpexename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
pub type SetConsoleInputExeNameW = unsafe extern "system" fn(lpexename: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleInputExeNameW(lpexename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
pub type SetConsoleKeyShortcuts = unsafe extern "system" fn(bset: windows_sys::core::BOOL, breservekeys: u8, lpappkeys: *const APPKEY, dwnumappkeys: u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleKeyShortcuts(bset : windows_sys::core::BOOL, breservekeys : u8, lpappkeys : *const APPKEY, dwnumappkeys : u32) -> windows_sys::core::BOOL);
pub type SetConsoleLocalEUDC = unsafe extern "system" fn(hconsolehandle: super::super::Foundation::HANDLE, wcodepoint: u16, cfontsize: COORD, lpsb: windows_sys::core::PCSTR) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleLocalEUDC(hconsolehandle : super::super::Foundation::HANDLE, wcodepoint : u16, cfontsize : COORD, lpsb : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
pub type SetConsoleMenuClose = unsafe extern "system" fn(benable: windows_sys::core::BOOL) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleMenuClose(benable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
pub type SetConsoleMode = unsafe extern "system" fn(hconsolehandle: super::super::Foundation::HANDLE, dwmode: CONSOLE_MODE) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleMode(hconsolehandle : super::super::Foundation::HANDLE, dwmode : CONSOLE_MODE) -> windows_sys::core::BOOL);
pub type SetConsoleNlsMode = unsafe extern "system" fn(hconsole: super::super::Foundation::HANDLE, fdwnlsmode: u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleNlsMode(hconsole : super::super::Foundation::HANDLE, fdwnlsmode : u32) -> windows_sys::core::BOOL);
pub type SetConsoleNumberOfCommandsA = unsafe extern "system" fn(number: u32, exename: windows_sys::core::PCSTR) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleNumberOfCommandsA(number : u32, exename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
pub type SetConsoleNumberOfCommandsW = unsafe extern "system" fn(number: u32, exename: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleNumberOfCommandsW(number : u32, exename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
pub type SetConsoleOS2OemFormat = unsafe extern "system" fn(fos2oemformat: windows_sys::core::BOOL) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleOS2OemFormat(fos2oemformat : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
pub type SetConsoleOutputCP = unsafe extern "system" fn(wcodepageid: u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleOutputCP(wcodepageid : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type SetConsolePalette = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, hpalette: super::super::Graphics::Gdi::HPALETTE, dwusage: u32) -> windows_sys::core::BOOL;
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("kernel32.dll" "system" fn SetConsolePalette(hconsoleoutput : super::super::Foundation::HANDLE, hpalette : super::super::Graphics::Gdi::HPALETTE, dwusage : u32) -> windows_sys::core::BOOL);
pub type SetConsoleScreenBufferInfoEx = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolescreenbufferinfoex: *const CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleScreenBufferInfoEx(hconsoleoutput : super::super::Foundation::HANDLE, lpconsolescreenbufferinfoex : *const CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_sys::core::BOOL);
pub type SetConsoleScreenBufferSize = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, dwsize: COORD) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleScreenBufferSize(hconsoleoutput : super::super::Foundation::HANDLE, dwsize : COORD) -> windows_sys::core::BOOL);
pub type SetConsoleTextAttribute = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, wattributes: CONSOLE_CHARACTER_ATTRIBUTES) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleTextAttribute(hconsoleoutput : super::super::Foundation::HANDLE, wattributes : CONSOLE_CHARACTER_ATTRIBUTES) -> windows_sys::core::BOOL);
pub type SetConsoleTitleA = unsafe extern "system" fn(lpconsoletitle: windows_sys::core::PCSTR) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleTitleA(lpconsoletitle : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
pub type SetConsoleTitleW = unsafe extern "system" fn(lpconsoletitle: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleTitleW(lpconsoletitle : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
pub type SetConsoleWindowInfo = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, babsolute: windows_sys::core::BOOL, lpconsolewindow: *const SMALL_RECT) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetConsoleWindowInfo(hconsoleoutput : super::super::Foundation::HANDLE, babsolute : windows_sys::core::BOOL, lpconsolewindow : *const SMALL_RECT) -> windows_sys::core::BOOL);
pub type SetCurrentConsoleFontEx = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: windows_sys::core::BOOL, lpconsolecurrentfontex: *const CONSOLE_FONT_INFOEX) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetCurrentConsoleFontEx(hconsoleoutput : super::super::Foundation::HANDLE, bmaximumwindow : windows_sys::core::BOOL, lpconsolecurrentfontex : *const CONSOLE_FONT_INFOEX) -> windows_sys::core::BOOL);
pub type SetLastConsoleEventActive = unsafe extern "system" fn();
windows_link::link!("kernel32.dll" "system" fn SetLastConsoleEventActive());
pub type SetStdHandle = unsafe extern "system" fn(nstdhandle: STD_HANDLE, hhandle: super::super::Foundation::HANDLE) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetStdHandle(nstdhandle : STD_HANDLE, hhandle : super::super::Foundation::HANDLE) -> windows_sys::core::BOOL);
pub type SetStdHandleEx = unsafe extern "system" fn(nstdhandle: STD_HANDLE, hhandle: super::super::Foundation::HANDLE, phprevvalue: *mut super::super::Foundation::HANDLE) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn SetStdHandleEx(nstdhandle : STD_HANDLE, hhandle : super::super::Foundation::HANDLE, phprevvalue : *mut super::super::Foundation::HANDLE) -> windows_sys::core::BOOL);
pub type ShowConsoleCursor = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, bshow: windows_sys::core::BOOL) -> i32;
windows_link::link!("kernel32.dll" "system" fn ShowConsoleCursor(hconsoleoutput : super::super::Foundation::HANDLE, bshow : windows_sys::core::BOOL) -> i32);
pub type UnregisterConsoleIME = unsafe extern "system" fn() -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn UnregisterConsoleIME() -> windows_sys::core::BOOL);
pub type VDMConsoleOperation = unsafe extern "system" fn(ifunction: u32, lpdata: *mut core::ffi::c_void) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn VDMConsoleOperation(ifunction : u32, lpdata : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
pub type VerifyConsoleIoHandle = unsafe extern "system" fn(hiohandle: super::super::Foundation::HANDLE) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn VerifyConsoleIoHandle(hiohandle : super::super::Foundation::HANDLE) -> windows_sys::core::BOOL);
pub type WriteConsoleA = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: windows_sys::core::PCSTR, nnumberofcharstowrite: u32, lpnumberofcharswritten: *mut u32, lpreserved: *const core::ffi::c_void) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn WriteConsoleA(hconsoleoutput : super::super::Foundation::HANDLE, lpbuffer : windows_sys::core::PCSTR, nnumberofcharstowrite : u32, lpnumberofcharswritten : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
pub type WriteConsoleInputA = unsafe extern "system" fn(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *const INPUT_RECORD, nlength: u32, lpnumberofeventswritten: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn WriteConsoleInputA(hconsoleinput : super::super::Foundation::HANDLE, lpbuffer : *const INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> windows_sys::core::BOOL);
pub type WriteConsoleInputVDMA = unsafe extern "system" fn(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *const INPUT_RECORD, nlength: u32, lpnumberofeventswritten: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn WriteConsoleInputVDMA(hconsoleinput : super::super::Foundation::HANDLE, lpbuffer : *const INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> windows_sys::core::BOOL);
pub type WriteConsoleInputVDMW = unsafe extern "system" fn(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *const INPUT_RECORD, nlength: u32, lpnumberofeventswritten: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn WriteConsoleInputVDMW(hconsoleinput : super::super::Foundation::HANDLE, lpbuffer : *const INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> windows_sys::core::BOOL);
pub type WriteConsoleInputW = unsafe extern "system" fn(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *const INPUT_RECORD, nlength: u32, lpnumberofeventswritten: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn WriteConsoleInputW(hconsoleinput : super::super::Foundation::HANDLE, lpbuffer : *const INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> windows_sys::core::BOOL);
pub type WriteConsoleOutputA = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *const CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpwriteregion: *mut SMALL_RECT) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn WriteConsoleOutputA(hconsoleoutput : super::super::Foundation::HANDLE, lpbuffer : *const CHAR_INFO, dwbuffersize : COORD, dwbuffercoord : COORD, lpwriteregion : *mut SMALL_RECT) -> windows_sys::core::BOOL);
pub type WriteConsoleOutputAttribute = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpattribute: *const u16, nlength: u32, dwwritecoord: COORD, lpnumberofattrswritten: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn WriteConsoleOutputAttribute(hconsoleoutput : super::super::Foundation::HANDLE, lpattribute : *const u16, nlength : u32, dwwritecoord : COORD, lpnumberofattrswritten : *mut u32) -> windows_sys::core::BOOL);
pub type WriteConsoleOutputCharacterA = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: windows_sys::core::PCSTR, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn WriteConsoleOutputCharacterA(hconsoleoutput : super::super::Foundation::HANDLE, lpcharacter : windows_sys::core::PCSTR, nlength : u32, dwwritecoord : COORD, lpnumberofcharswritten : *mut u32) -> windows_sys::core::BOOL);
pub type WriteConsoleOutputCharacterW = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: windows_sys::core::PCWSTR, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn WriteConsoleOutputCharacterW(hconsoleoutput : super::super::Foundation::HANDLE, lpcharacter : windows_sys::core::PCWSTR, nlength : u32, dwwritecoord : COORD, lpnumberofcharswritten : *mut u32) -> windows_sys::core::BOOL);
pub type WriteConsoleOutputW = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *const CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpwriteregion: *mut SMALL_RECT) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn WriteConsoleOutputW(hconsoleoutput : super::super::Foundation::HANDLE, lpbuffer : *const CHAR_INFO, dwbuffersize : COORD, dwbuffercoord : COORD, lpwriteregion : *mut SMALL_RECT) -> windows_sys::core::BOOL);
pub type WriteConsoleW = unsafe extern "system" fn(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: windows_sys::core::PCWSTR, nnumberofcharstowrite: u32, lpnumberofcharswritten: *mut u32, lpreserved: *const core::ffi::c_void) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn WriteConsoleW(hconsoleoutput : super::super::Foundation::HANDLE, lpbuffer : windows_sys::core::PCWSTR, nnumberofcharstowrite : u32, lpnumberofcharswritten : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
pub type ALLOC_CONSOLE_MODE = i32;
pub const ALLOC_CONSOLE_MODE_DEFAULT: ALLOC_CONSOLE_MODE = 0i32;
pub const ALLOC_CONSOLE_MODE_NEW_WINDOW: ALLOC_CONSOLE_MODE = 1i32;
pub const ALLOC_CONSOLE_MODE_NO_WINDOW: ALLOC_CONSOLE_MODE = 2i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ALLOC_CONSOLE_OPTIONS {
    pub mode: ALLOC_CONSOLE_MODE,
    pub useShowWindow: windows_sys::core::BOOL,
    pub showWindow: u16,
}
pub type ALLOC_CONSOLE_RESULT = i32;
pub const ALLOC_CONSOLE_RESULT_EXISTING_CONSOLE: ALLOC_CONSOLE_RESULT = 2i32;
pub const ALLOC_CONSOLE_RESULT_NEW_CONSOLE: ALLOC_CONSOLE_RESULT = 1i32;
pub const ALLOC_CONSOLE_RESULT_NO_CONSOLE: ALLOC_CONSOLE_RESULT = 0i32;
pub const ALTNUMPAD_BIT: u32 = 67108864u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct APPKEY {
    pub Modifier: u16,
    pub ScanCode: u16,
}
pub const ATTACH_PARENT_PROCESS: u32 = 4294967295u32;
pub const BACKGROUND_BLUE: CONSOLE_CHARACTER_ATTRIBUTES = 16u16;
pub const BACKGROUND_GREEN: CONSOLE_CHARACTER_ATTRIBUTES = 32u16;
pub const BACKGROUND_INTENSITY: CONSOLE_CHARACTER_ATTRIBUTES = 128u16;
pub const BACKGROUND_RED: CONSOLE_CHARACTER_ATTRIBUTES = 64u16;
pub const CAPSLOCK_ON: u32 = 128u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CHAR_INFO {
    pub Char: CHAR_INFO_0,
    pub Attributes: u16,
}
impl Default for CHAR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CHAR_INFO_0 {
    pub UnicodeChar: u16,
    pub AsciiChar: i8,
}
impl Default for CHAR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CHAR_TYPE_LEADING: u32 = 2u32;
pub const CHAR_TYPE_SBCS: u32 = 0u32;
pub const CHAR_TYPE_TRAILING: u32 = 3u32;
pub const COMMON_LVB_GRID_HORIZONTAL: CONSOLE_CHARACTER_ATTRIBUTES = 1024u16;
pub const COMMON_LVB_GRID_LVERTICAL: CONSOLE_CHARACTER_ATTRIBUTES = 2048u16;
pub const COMMON_LVB_GRID_RVERTICAL: CONSOLE_CHARACTER_ATTRIBUTES = 4096u16;
pub const COMMON_LVB_LEADING_BYTE: CONSOLE_CHARACTER_ATTRIBUTES = 256u16;
pub const COMMON_LVB_REVERSE_VIDEO: CONSOLE_CHARACTER_ATTRIBUTES = 16384u16;
pub const COMMON_LVB_SBCSDBCS: CONSOLE_CHARACTER_ATTRIBUTES = 768u16;
pub const COMMON_LVB_TRAILING_BYTE: CONSOLE_CHARACTER_ATTRIBUTES = 512u16;
pub const COMMON_LVB_UNDERSCORE: CONSOLE_CHARACTER_ATTRIBUTES = 32768u16;
pub type CONSOLECONTROL = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONSOLEENDTASK {
    pub ProcessId: super::super::Foundation::HANDLE,
    pub hwnd: super::super::Foundation::HWND,
    pub ConsoleEventCode: u32,
    pub ConsoleFlags: u32,
}
impl Default for CONSOLEENDTASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONSOLESETFOREGROUND {
    pub hProcess: super::super::Foundation::HANDLE,
    pub bForeground: windows_sys::core::BOOL,
}
impl Default for CONSOLESETFOREGROUND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONSOLEWINDOWOWNER {
    pub hwnd: super::super::Foundation::HWND,
    pub ProcessId: u32,
    pub ThreadId: u32,
}
impl Default for CONSOLEWINDOWOWNER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CONSOLE_ALTENTER: u32 = 8u32;
pub const CONSOLE_ALTESC: u32 = 2u32;
pub const CONSOLE_ALTPRTSC: u32 = 16u32;
pub const CONSOLE_ALTSPACE: u32 = 4u32;
pub const CONSOLE_ALTTAB: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONSOLE_CARET_INFO {
    pub hwnd: super::super::Foundation::HWND,
    pub rc: super::super::Foundation::RECT,
}
impl Default for CONSOLE_CARET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CONSOLE_CHARACTER_ATTRIBUTES = u16;
pub const CONSOLE_CTRLESC: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_CURSOR_INFO {
    pub dwSize: u32,
    pub bVisible: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_FONT_INFO {
    pub nFont: u32,
    pub dwFontSize: COORD,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONSOLE_FONT_INFOEX {
    pub cbSize: u32,
    pub nFont: u32,
    pub dwFontSize: COORD,
    pub FontFamily: u32,
    pub FontWeight: u32,
    pub FaceName: [u16; 32],
}
impl Default for CONSOLE_FONT_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CONSOLE_FULLSCREEN: u32 = 1u32;
pub const CONSOLE_FULLSCREEN_HARDWARE: u32 = 2u32;
pub const CONSOLE_FULLSCREEN_MODE: u32 = 1u32;
pub const CONSOLE_GENERIC: windows_sys::core::PCWSTR = windows_sys::core::w!("CON");
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct CONSOLE_GRAPHICS_BUFFER_INFO {
    pub dwBitMapInfoLength: u32,
    pub lpBitMapInfo: *mut super::super::Graphics::Gdi::BITMAPINFO,
    pub dwUsage: u32,
    pub hMutex: super::super::Foundation::HANDLE,
    pub lpBitMap: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CONSOLE_GRAPHICS_BUFFER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CONSOLE_HANDLE_NEVERSET: u32 = 268435456u32;
pub const CONSOLE_HANDLE_SIGNATURE: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_HISTORY_INFO {
    pub cbSize: u32,
    pub HistoryBufferSize: u32,
    pub NumberOfHistoryBuffers: u32,
    pub dwFlags: u32,
}
pub const CONSOLE_INPUT_STRING: windows_sys::core::PCWSTR = windows_sys::core::w!("CONIN$");
pub type CONSOLE_MODE = u32;
pub const CONSOLE_MODIFIER_ALT: u32 = 8u32;
pub const CONSOLE_MODIFIER_CONTROL: u32 = 4u32;
pub const CONSOLE_MODIFIER_SHIFT: u32 = 3u32;
pub const CONSOLE_MOUSE_DOWN: u32 = 8u32;
pub const CONSOLE_MOUSE_SELECTION: u32 = 4u32;
pub const CONSOLE_NOSHORTCUTKEY: u32 = 0u32;
pub const CONSOLE_NO_SELECTION: u32 = 0u32;
pub const CONSOLE_OUTPUT_STRING: windows_sys::core::PCWSTR = windows_sys::core::w!("CONOUT$");
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_PROCESS_INFO {
    pub dwProcessID: u32,
    pub dwFlags: u32,
}
pub const CONSOLE_PRTSC: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_READCONSOLE_CONTROL {
    pub nLength: u32,
    pub nInitialChars: u32,
    pub dwCtrlWakeupMask: u32,
    pub dwControlKeyState: u32,
}
pub const CONSOLE_REGISTER_VDM: u32 = 1u32;
pub const CONSOLE_REGISTER_WOW: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: CONSOLE_CHARACTER_ATTRIBUTES,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONSOLE_SCREEN_BUFFER_INFOEX {
    pub cbSize: u32,
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: CONSOLE_CHARACTER_ATTRIBUTES,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
    pub wPopupAttributes: u16,
    pub bFullscreenSupported: windows_sys::core::BOOL,
    pub ColorTable: [super::super::Foundation::COLORREF; 16],
}
impl Default for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_SELECTION_INFO {
    pub dwFlags: u32,
    pub dwSelectionAnchor: COORD,
    pub srSelection: SMALL_RECT,
}
pub const CONSOLE_SELECTION_INVERTED: u32 = 16u32;
pub const CONSOLE_SELECTION_IN_PROGRESS: u32 = 1u32;
pub const CONSOLE_SELECTION_NOT_EMPTY: u32 = 2u32;
pub const CONSOLE_TEXTMODE_BUFFER: u32 = 1u32;
pub const CONSOLE_UNREGISTER_VDM: u32 = 0u32;
pub const CONSOLE_WINDOWED_MODE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct COORD {
    pub X: i16,
    pub Y: i16,
}
pub const CTRL_BREAK_EVENT: u32 = 1u32;
pub const CTRL_CLOSE_EVENT: u32 = 2u32;
pub const CTRL_C_EVENT: u32 = 0u32;
pub const CTRL_LOGOFF_EVENT: u32 = 5u32;
pub const CTRL_SHUTDOWN_EVENT: u32 = 6u32;
pub const ConsoleEndTask: CONSOLECONTROL = 7i32;
pub const ConsoleNotifyConsoleApplication: CONSOLECONTROL = 1i32;
pub const ConsoleSetCaretInfo: CONSOLECONTROL = 3i32;
pub const ConsoleSetForeground: CONSOLECONTROL = 5i32;
pub const ConsoleSetWindowOwner: CONSOLECONTROL = 6i32;
pub const DISABLE_NEWLINE_AUTO_RETURN: CONSOLE_MODE = 8u32;
pub const DOUBLE_CLICK: u32 = 2u32;
pub const ENABLE_AUTO_POSITION: CONSOLE_MODE = 256u32;
pub const ENABLE_ECHO_INPUT: CONSOLE_MODE = 4u32;
pub const ENABLE_EXTENDED_FLAGS: CONSOLE_MODE = 128u32;
pub const ENABLE_INSERT_MODE: CONSOLE_MODE = 32u32;
pub const ENABLE_LINE_INPUT: CONSOLE_MODE = 2u32;
pub const ENABLE_LVB_GRID_WORLDWIDE: CONSOLE_MODE = 16u32;
pub const ENABLE_MOUSE_INPUT: CONSOLE_MODE = 16u32;
pub const ENABLE_PROCESSED_INPUT: CONSOLE_MODE = 1u32;
pub const ENABLE_PROCESSED_OUTPUT: CONSOLE_MODE = 1u32;
pub const ENABLE_QUICK_EDIT_MODE: CONSOLE_MODE = 64u32;
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: CONSOLE_MODE = 512u32;
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: CONSOLE_MODE = 4u32;
pub const ENABLE_WINDOW_INPUT: CONSOLE_MODE = 8u32;
pub const ENABLE_WRAP_AT_EOL_OUTPUT: CONSOLE_MODE = 2u32;
pub const ENHANCED_KEY: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ExtKeyDef {
    pub keys: [ExtKeySubst; 3],
}
impl Default for ExtKeyDef {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct ExtKeyDefBuf {
    pub dwVersion: u32,
    pub dwCheckSum: u32,
    pub table: [ExtKeyDef; 26],
}
impl Default for ExtKeyDefBuf {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ExtKeySubst {
    pub wMod: u16,
    pub wVirKey: u16,
    pub wUnicodeChar: u16,
}
pub const FOCUS_EVENT: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FOCUS_EVENT_RECORD {
    pub bSetFocus: windows_sys::core::BOOL,
}
pub const FOREGROUND_BLUE: CONSOLE_CHARACTER_ATTRIBUTES = 1u16;
pub const FOREGROUND_GREEN: CONSOLE_CHARACTER_ATTRIBUTES = 2u16;
pub const FOREGROUND_INTENSITY: CONSOLE_CHARACTER_ATTRIBUTES = 8u16;
pub const FOREGROUND_RED: CONSOLE_CHARACTER_ATTRIBUTES = 4u16;
pub const FROM_LEFT_1ST_BUTTON_PRESSED: u32 = 1u32;
pub const FROM_LEFT_2ND_BUTTON_PRESSED: u32 = 4u32;
pub const FROM_LEFT_3RD_BUTTON_PRESSED: u32 = 8u32;
pub const FROM_LEFT_4TH_BUTTON_PRESSED: u32 = 16u32;
pub const HISTORY_NO_DUP_FLAG: u32 = 1u32;
pub type HPCON = isize;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INPUT_RECORD {
    pub EventType: u16,
    pub Event: INPUT_RECORD_0,
}
impl Default for INPUT_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INPUT_RECORD_0 {
    pub KeyEvent: KEY_EVENT_RECORD,
    pub MouseEvent: MOUSE_EVENT_RECORD,
    pub WindowBufferSizeEvent: WINDOW_BUFFER_SIZE_RECORD,
    pub MenuEvent: MENU_EVENT_RECORD,
    pub FocusEvent: FOCUS_EVENT_RECORD,
}
impl Default for INPUT_RECORD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KEY_EVENT: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_EVENT_RECORD {
    pub bKeyDown: windows_sys::core::BOOL,
    pub wRepeatCount: u16,
    pub wVirtualKeyCode: u16,
    pub wVirtualScanCode: u16,
    pub uChar: KEY_EVENT_RECORD_0,
    pub dwControlKeyState: u32,
}
impl Default for KEY_EVENT_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KEY_EVENT_RECORD_0 {
    pub UnicodeChar: u16,
    pub AsciiChar: i8,
}
impl Default for KEY_EVENT_RECORD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LEFT_ALT_PRESSED: u32 = 2u32;
pub const LEFT_CTRL_PRESSED: u32 = 8u32;
pub const MENU_EVENT: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MENU_EVENT_RECORD {
    pub dwCommandId: u32,
}
pub const MOUSE_EVENT: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MOUSE_EVENT_RECORD {
    pub dwMousePosition: COORD,
    pub dwButtonState: u32,
    pub dwControlKeyState: u32,
    pub dwEventFlags: u32,
}
pub const MOUSE_HWHEELED: u32 = 8u32;
pub const MOUSE_MOVED: u32 = 1u32;
pub const MOUSE_WHEELED: u32 = 4u32;
pub const NLS_ALPHANUMERIC: u32 = 0u32;
pub const NLS_DBCSCHAR: u32 = 65536u32;
pub const NLS_HIRAGANA: u32 = 262144u32;
pub const NLS_IME_CONVERSION: u32 = 8388608u32;
pub const NLS_IME_DISABLE: u32 = 536870912u32;
pub const NLS_KATAKANA: u32 = 131072u32;
pub const NLS_ROMAN: u32 = 4194304u32;
pub const NUMLOCK_ON: u32 = 32u32;
pub type PHANDLER_ROUTINE = Option<unsafe extern "system" fn(ctrltype: u32) -> windows_sys::core::BOOL>;
pub const PID_CONSOLE_CTRLKEYSDISABLED: u32 = 4u32;
pub const PID_CONSOLE_CURSOR_COLOR: u32 = 9u32;
pub const PID_CONSOLE_CURSOR_TYPE: u32 = 8u32;
pub const PID_CONSOLE_DEFAULTBACKGROUND: u32 = 12u32;
pub const PID_CONSOLE_DEFAULTFOREGROUND: u32 = 11u32;
pub const PID_CONSOLE_FILTERONPASTE: u32 = 3u32;
pub const PID_CONSOLE_FORCEV2: u32 = 1u32;
pub const PID_CONSOLE_INTERCEPT_COPY_PASTE: u32 = 10u32;
pub const PID_CONSOLE_LINESELECTION: u32 = 5u32;
pub const PID_CONSOLE_TERMINALSCROLLING: u32 = 13u32;
pub const PID_CONSOLE_WINDOWMAXIMIZED: u32 = 7u32;
pub const PID_CONSOLE_WINDOWTRANSPARENCY: u32 = 6u32;
pub const PID_CONSOLE_WRAPTEXT: u32 = 2u32;
pub const PSEUDOCONSOLE_INHERIT_CURSOR: u32 = 1u32;
pub const RIGHTMOST_BUTTON_PRESSED: u32 = 2u32;
pub const RIGHT_ALT_PRESSED: u32 = 1u32;
pub const RIGHT_CTRL_PRESSED: u32 = 4u32;
pub const Reserved1: CONSOLECONTROL = 0i32;
pub const Reserved2: CONSOLECONTROL = 2i32;
pub const Reserved3: CONSOLECONTROL = 4i32;
pub const SCROLLLOCK_ON: u32 = 64u32;
pub const SHIFT_PRESSED: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
pub const STD_ERROR_HANDLE: STD_HANDLE = 4294967284u32;
pub type STD_HANDLE = u32;
pub const STD_INPUT_HANDLE: STD_HANDLE = 4294967286u32;
pub const STD_OUTPUT_HANDLE: STD_HANDLE = 4294967285u32;
pub const VDM_CLIENT_RECT: u32 = 3u32;
pub const VDM_CLIENT_TO_SCREEN: u32 = 4u32;
pub const VDM_FULLSCREEN_NOPAINT: u32 = 7u32;
pub const VDM_HIDE_WINDOW: u32 = 1u32;
pub const VDM_IS_HIDDEN: u32 = 6u32;
pub const VDM_IS_ICONIC: u32 = 2u32;
pub const VDM_SCREEN_TO_CLIENT: u32 = 5u32;
pub const VDM_SET_VIDEO_MODE: u32 = 8u32;
pub const WINDOW_BUFFER_SIZE_EVENT: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINDOW_BUFFER_SIZE_RECORD {
    pub dwSize: COORD,
}
