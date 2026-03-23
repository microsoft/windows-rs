#[inline]
pub unsafe fn AddConsoleAliasA<P0, P1, P2>(source: P0, target: P1, exename: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn AddConsoleAliasA(source : windows_core::PCSTR, target : windows_core::PCSTR, exename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { AddConsoleAliasA(source.param().abi(), target.param().abi(), exename.param().abi()).ok() }
}
#[inline]
pub unsafe fn AddConsoleAliasW<P0, P1, P2>(source: P0, target: P1, exename: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn AddConsoleAliasW(source : windows_core::PCWSTR, target : windows_core::PCWSTR, exename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { AddConsoleAliasW(source.param().abi(), target.param().abi(), exename.param().abi()).ok() }
}
#[inline]
pub unsafe fn AllocConsole() -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn AllocConsole() -> windows_core::BOOL);
    unsafe { AllocConsole().ok() }
}
#[inline]
pub unsafe fn AllocConsoleWithOptions(options: Option<*const ALLOC_CONSOLE_OPTIONS>, result: Option<*mut ALLOC_CONSOLE_RESULT>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn AllocConsoleWithOptions(options : *const ALLOC_CONSOLE_OPTIONS, result : *mut ALLOC_CONSOLE_RESULT) -> windows_core::HRESULT);
    unsafe { AllocConsoleWithOptions(options.unwrap_or(core::mem::zeroed()) as _, result.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn AttachConsole(dwprocessid: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn AttachConsole(dwprocessid : u32) -> windows_core::BOOL);
    unsafe { AttachConsole(dwprocessid).ok() }
}
#[inline]
pub unsafe fn CloseConsoleHandle(hconsole: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CloseConsoleHandle(hconsole : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { CloseConsoleHandle(hconsole) }
}
#[inline]
pub unsafe fn ClosePseudoConsole(hpc: HPCON) {
    windows_core::link!("kernel32.dll" "system" fn ClosePseudoConsole(hpc : HPCON));
    unsafe { ClosePseudoConsole(hpc) }
}
#[inline]
pub unsafe fn ConsoleControl(command: CONSOLECONTROL, consoleinformation: *const core::ffi::c_void, consoleinformationlength: u32) -> windows_core::NTSTATUS {
    windows_core::link!("user32.dll" "system" fn ConsoleControl(command : CONSOLECONTROL, consoleinformation : *const core::ffi::c_void, consoleinformationlength : u32) -> windows_core:: NTSTATUS);
    unsafe { ConsoleControl(command, consoleinformation, consoleinformationlength) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn ConsoleMenuControl(hconsoleoutput: super::super::Foundation::HANDLE, dwcommandidlow: u32, dwcommandidhigh: u32) -> super::super::UI::WindowsAndMessaging::HMENU {
    windows_core::link!("kernel32.dll" "system" fn ConsoleMenuControl(hconsoleoutput : super::super::Foundation:: HANDLE, dwcommandidlow : u32, dwcommandidhigh : u32) -> super::super::UI::WindowsAndMessaging:: HMENU);
    unsafe { ConsoleMenuControl(hconsoleoutput, dwcommandidlow, dwcommandidhigh) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateConsoleScreenBuffer(dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwflags: u32, lpscreenbufferdata: Option<*const core::ffi::c_void>) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("kernel32.dll" "system" fn CreateConsoleScreenBuffer(dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwflags : u32, lpscreenbufferdata : *const core::ffi::c_void) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { CreateConsoleScreenBuffer(dwdesiredaccess, dwsharemode, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, dwflags, lpscreenbufferdata.unwrap_or(core::mem::zeroed()) as _) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn CreatePseudoConsole(size: COORD, hinput: super::super::Foundation::HANDLE, houtput: super::super::Foundation::HANDLE, dwflags: u32) -> windows_core::Result<HPCON> {
    windows_core::link!("kernel32.dll" "system" fn CreatePseudoConsole(size : COORD, hinput : super::super::Foundation:: HANDLE, houtput : super::super::Foundation:: HANDLE, dwflags : u32, phpc : *mut HPCON) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreatePseudoConsole(core::mem::transmute(size), hinput, houtput, dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DuplicateConsoleHandle(hsourcehandle: super::super::Foundation::HANDLE, dwdesiredaccess: u32, binherithandle: bool, dwoptions: u32) -> super::super::Foundation::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn DuplicateConsoleHandle(hsourcehandle : super::super::Foundation:: HANDLE, dwdesiredaccess : u32, binherithandle : windows_core::BOOL, dwoptions : u32) -> super::super::Foundation:: HANDLE);
    unsafe { DuplicateConsoleHandle(hsourcehandle, dwdesiredaccess, binherithandle.into(), dwoptions) }
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
pub unsafe fn FillConsoleOutputAttribute(hconsoleoutput: super::super::Foundation::HANDLE, wattribute: u16, nlength: u32, dwwritecoord: COORD, lpnumberofattrswritten: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn FillConsoleOutputAttribute(hconsoleoutput : super::super::Foundation:: HANDLE, wattribute : u16, nlength : u32, dwwritecoord : COORD, lpnumberofattrswritten : *mut u32) -> windows_core::BOOL);
    unsafe { FillConsoleOutputAttribute(hconsoleoutput, wattribute, nlength, core::mem::transmute(dwwritecoord), lpnumberofattrswritten as _).ok() }
}
#[inline]
pub unsafe fn FillConsoleOutputCharacterA(hconsoleoutput: super::super::Foundation::HANDLE, ccharacter: i8, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn FillConsoleOutputCharacterA(hconsoleoutput : super::super::Foundation:: HANDLE, ccharacter : i8, nlength : u32, dwwritecoord : COORD, lpnumberofcharswritten : *mut u32) -> windows_core::BOOL);
    unsafe { FillConsoleOutputCharacterA(hconsoleoutput, ccharacter, nlength, core::mem::transmute(dwwritecoord), lpnumberofcharswritten as _).ok() }
}
#[inline]
pub unsafe fn FillConsoleOutputCharacterW(hconsoleoutput: super::super::Foundation::HANDLE, ccharacter: u16, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn FillConsoleOutputCharacterW(hconsoleoutput : super::super::Foundation:: HANDLE, ccharacter : u16, nlength : u32, dwwritecoord : COORD, lpnumberofcharswritten : *mut u32) -> windows_core::BOOL);
    unsafe { FillConsoleOutputCharacterW(hconsoleoutput, ccharacter, nlength, core::mem::transmute(dwwritecoord), lpnumberofcharswritten as _).ok() }
}
#[inline]
pub unsafe fn FlushConsoleInputBuffer(hconsoleinput: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn FlushConsoleInputBuffer(hconsoleinput : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { FlushConsoleInputBuffer(hconsoleinput).ok() }
}
#[inline]
pub unsafe fn FreeConsole() -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn FreeConsole() -> windows_core::BOOL);
    unsafe { FreeConsole().ok() }
}
#[inline]
pub unsafe fn GenerateConsoleCtrlEvent(dwctrlevent: u32, dwprocessgroupid: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GenerateConsoleCtrlEvent(dwctrlevent : u32, dwprocessgroupid : u32) -> windows_core::BOOL);
    unsafe { GenerateConsoleCtrlEvent(dwctrlevent, dwprocessgroupid).ok() }
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
pub unsafe fn GetConsoleCP() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleCP() -> u32);
    unsafe { GetConsoleCP() }
}
#[inline]
pub unsafe fn GetConsoleCharType(hconsole: super::super::Foundation::HANDLE, coordcheck: COORD, pdwtype: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleCharType(hconsole : super::super::Foundation:: HANDLE, coordcheck : COORD, pdwtype : *mut u32) -> windows_core::BOOL);
    unsafe { GetConsoleCharType(hconsole, core::mem::transmute(coordcheck), pdwtype as _) }
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
pub unsafe fn GetConsoleCursorInfo(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolecursorinfo: *mut CONSOLE_CURSOR_INFO) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleCursorInfo(hconsoleoutput : super::super::Foundation:: HANDLE, lpconsolecursorinfo : *mut CONSOLE_CURSOR_INFO) -> windows_core::BOOL);
    unsafe { GetConsoleCursorInfo(hconsoleoutput, lpconsolecursorinfo as _).ok() }
}
#[inline]
pub unsafe fn GetConsoleCursorMode(hconsolehandle: super::super::Foundation::HANDLE, pbblink: *mut windows_core::BOOL, pbdbenable: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleCursorMode(hconsolehandle : super::super::Foundation:: HANDLE, pbblink : *mut windows_core::BOOL, pbdbenable : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetConsoleCursorMode(hconsolehandle, pbblink as _, pbdbenable as _) }
}
#[inline]
pub unsafe fn GetConsoleDisplayMode(lpmodeflags: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleDisplayMode(lpmodeflags : *mut u32) -> windows_core::BOOL);
    unsafe { GetConsoleDisplayMode(lpmodeflags as _).ok() }
}
#[inline]
pub unsafe fn GetConsoleFontInfo(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: bool, nlength: u32, lpconsolefontinfo: *mut CONSOLE_FONT_INFO) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleFontInfo(hconsoleoutput : super::super::Foundation:: HANDLE, bmaximumwindow : windows_core::BOOL, nlength : u32, lpconsolefontinfo : *mut CONSOLE_FONT_INFO) -> u32);
    unsafe { GetConsoleFontInfo(hconsoleoutput, bmaximumwindow.into(), nlength, lpconsolefontinfo as _) }
}
#[inline]
pub unsafe fn GetConsoleFontSize(hconsoleoutput: super::super::Foundation::HANDLE, nfont: u32) -> COORD {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleFontSize(hconsoleoutput : super::super::Foundation:: HANDLE, nfont : u32) -> COORD);
    unsafe { GetConsoleFontSize(hconsoleoutput, nfont) }
}
#[inline]
pub unsafe fn GetConsoleHardwareState(hconsoleoutput: super::super::Foundation::HANDLE, lpresolution: *mut COORD, lpfontsize: *mut COORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleHardwareState(hconsoleoutput : super::super::Foundation:: HANDLE, lpresolution : *mut COORD, lpfontsize : *mut COORD) -> windows_core::BOOL);
    unsafe { GetConsoleHardwareState(hconsoleoutput, lpresolution as _, lpfontsize as _) }
}
#[inline]
pub unsafe fn GetConsoleHistoryInfo(lpconsolehistoryinfo: *mut CONSOLE_HISTORY_INFO) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleHistoryInfo(lpconsolehistoryinfo : *mut CONSOLE_HISTORY_INFO) -> windows_core::BOOL);
    unsafe { GetConsoleHistoryInfo(lpconsolehistoryinfo as _).ok() }
}
#[inline]
pub unsafe fn GetConsoleInputExeNameA(lpbuffer: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleInputExeNameA(nbufferlength : u32, lpbuffer : windows_core::PSTR) -> u32);
    unsafe { GetConsoleInputExeNameA(lpbuffer.len().try_into().unwrap(), core::mem::transmute(lpbuffer.as_ptr())) }
}
#[inline]
pub unsafe fn GetConsoleInputExeNameW(lpbuffer: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleInputExeNameW(nbufferlength : u32, lpbuffer : windows_core::PWSTR) -> u32);
    unsafe { GetConsoleInputExeNameW(lpbuffer.len().try_into().unwrap(), core::mem::transmute(lpbuffer.as_ptr())) }
}
#[inline]
pub unsafe fn GetConsoleInputWaitHandle() -> super::super::Foundation::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleInputWaitHandle() -> super::super::Foundation:: HANDLE);
    unsafe { GetConsoleInputWaitHandle() }
}
#[inline]
pub unsafe fn GetConsoleKeyboardLayoutNameA(pszlayout: &mut [u8; 9]) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetConsoleKeyboardLayoutNameA(pszlayout : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { GetConsoleKeyboardLayoutNameA(core::mem::transmute(pszlayout.as_ptr())) }
}
#[inline]
pub unsafe fn GetConsoleKeyboardLayoutNameW(pszlayout: &mut [u16; 9]) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetConsoleKeyboardLayoutNameW(pszlayout : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { GetConsoleKeyboardLayoutNameW(core::mem::transmute(pszlayout.as_ptr())) }
}
#[inline]
pub unsafe fn GetConsoleMode(hconsolehandle: super::super::Foundation::HANDLE, lpmode: *mut CONSOLE_MODE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleMode(hconsolehandle : super::super::Foundation:: HANDLE, lpmode : *mut CONSOLE_MODE) -> windows_core::BOOL);
    unsafe { GetConsoleMode(hconsolehandle, lpmode as _).ok() }
}
#[inline]
pub unsafe fn GetConsoleNlsMode(hconsole: super::super::Foundation::HANDLE, lpdwnlsmode: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleNlsMode(hconsole : super::super::Foundation:: HANDLE, lpdwnlsmode : *mut u32) -> windows_core::BOOL);
    unsafe { GetConsoleNlsMode(hconsole, lpdwnlsmode as _) }
}
#[inline]
pub unsafe fn GetConsoleOriginalTitleA(lpconsoletitle: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleOriginalTitleA(lpconsoletitle : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetConsoleOriginalTitleA(core::mem::transmute(lpconsoletitle.as_ptr()), lpconsoletitle.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetConsoleOriginalTitleW(lpconsoletitle: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleOriginalTitleW(lpconsoletitle : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetConsoleOriginalTitleW(core::mem::transmute(lpconsoletitle.as_ptr()), lpconsoletitle.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetConsoleOutputCP() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleOutputCP() -> u32);
    unsafe { GetConsoleOutputCP() }
}
#[inline]
pub unsafe fn GetConsoleProcessList(lpdwprocesslist: &mut [u32]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleProcessList(lpdwprocesslist : *mut u32, dwprocesscount : u32) -> u32);
    unsafe { GetConsoleProcessList(core::mem::transmute(lpdwprocesslist.as_ptr()), lpdwprocesslist.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetConsoleScreenBufferInfo(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolescreenbufferinfo: *mut CONSOLE_SCREEN_BUFFER_INFO) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleScreenBufferInfo(hconsoleoutput : super::super::Foundation:: HANDLE, lpconsolescreenbufferinfo : *mut CONSOLE_SCREEN_BUFFER_INFO) -> windows_core::BOOL);
    unsafe { GetConsoleScreenBufferInfo(hconsoleoutput, lpconsolescreenbufferinfo as _).ok() }
}
#[inline]
pub unsafe fn GetConsoleScreenBufferInfoEx(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolescreenbufferinfoex: *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleScreenBufferInfoEx(hconsoleoutput : super::super::Foundation:: HANDLE, lpconsolescreenbufferinfoex : *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_core::BOOL);
    unsafe { GetConsoleScreenBufferInfoEx(hconsoleoutput, lpconsolescreenbufferinfoex as _).ok() }
}
#[inline]
pub unsafe fn GetConsoleSelectionInfo(lpconsoleselectioninfo: *mut CONSOLE_SELECTION_INFO) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleSelectionInfo(lpconsoleselectioninfo : *mut CONSOLE_SELECTION_INFO) -> windows_core::BOOL);
    unsafe { GetConsoleSelectionInfo(lpconsoleselectioninfo as _).ok() }
}
#[inline]
pub unsafe fn GetConsoleTitleA(lpconsoletitle: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleTitleA(lpconsoletitle : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetConsoleTitleA(core::mem::transmute(lpconsoletitle.as_ptr()), lpconsoletitle.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetConsoleTitleW(lpconsoletitle: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleTitleW(lpconsoletitle : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetConsoleTitleW(core::mem::transmute(lpconsoletitle.as_ptr()), lpconsoletitle.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetConsoleWindow() -> super::super::Foundation::HWND {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleWindow() -> super::super::Foundation:: HWND);
    unsafe { GetConsoleWindow() }
}
#[inline]
pub unsafe fn GetCurrentConsoleFont(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: bool, lpconsolecurrentfont: *mut CONSOLE_FONT_INFO) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentConsoleFont(hconsoleoutput : super::super::Foundation:: HANDLE, bmaximumwindow : windows_core::BOOL, lpconsolecurrentfont : *mut CONSOLE_FONT_INFO) -> windows_core::BOOL);
    unsafe { GetCurrentConsoleFont(hconsoleoutput, bmaximumwindow.into(), lpconsolecurrentfont as _).ok() }
}
#[inline]
pub unsafe fn GetCurrentConsoleFontEx(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: bool, lpconsolecurrentfontex: *mut CONSOLE_FONT_INFOEX) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentConsoleFontEx(hconsoleoutput : super::super::Foundation:: HANDLE, bmaximumwindow : windows_core::BOOL, lpconsolecurrentfontex : *mut CONSOLE_FONT_INFOEX) -> windows_core::BOOL);
    unsafe { GetCurrentConsoleFontEx(hconsoleoutput, bmaximumwindow.into(), lpconsolecurrentfontex as _).ok() }
}
#[inline]
pub unsafe fn GetLargestConsoleWindowSize(hconsoleoutput: super::super::Foundation::HANDLE) -> COORD {
    windows_core::link!("kernel32.dll" "system" fn GetLargestConsoleWindowSize(hconsoleoutput : super::super::Foundation:: HANDLE) -> COORD);
    unsafe { GetLargestConsoleWindowSize(hconsoleoutput) }
}
#[inline]
pub unsafe fn GetNumberOfConsoleFonts() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetNumberOfConsoleFonts() -> u32);
    unsafe { GetNumberOfConsoleFonts() }
}
#[inline]
pub unsafe fn GetNumberOfConsoleInputEvents(hconsoleinput: super::super::Foundation::HANDLE, lpnumberofevents: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetNumberOfConsoleInputEvents(hconsoleinput : super::super::Foundation:: HANDLE, lpnumberofevents : *mut u32) -> windows_core::BOOL);
    unsafe { GetNumberOfConsoleInputEvents(hconsoleinput, lpnumberofevents as _).ok() }
}
#[inline]
pub unsafe fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons : *mut u32) -> windows_core::BOOL);
    unsafe { GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons as _).ok() }
}
#[inline]
pub unsafe fn GetStdHandle(nstdhandle: STD_HANDLE) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("kernel32.dll" "system" fn GetStdHandle(nstdhandle : STD_HANDLE) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { GetStdHandle(nstdhandle) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn InvalidateConsoleDIBits(hconsoleoutput: super::super::Foundation::HANDLE, lprect: *const SMALL_RECT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InvalidateConsoleDIBits(hconsoleoutput : super::super::Foundation:: HANDLE, lprect : *const SMALL_RECT) -> windows_core::BOOL);
    unsafe { InvalidateConsoleDIBits(hconsoleoutput, lprect) }
}
#[inline]
pub unsafe fn OpenConsoleW<P0>(lpconsoledevice: P0, dwdesiredaccess: u32, binherithandle: bool, dwsharemode: u32) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenConsoleW(lpconsoledevice : windows_core::PCWSTR, dwdesiredaccess : u32, binherithandle : windows_core::BOOL, dwsharemode : u32) -> super::super::Foundation:: HANDLE);
    unsafe { OpenConsoleW(lpconsoledevice.param().abi(), dwdesiredaccess, binherithandle.into(), dwsharemode) }
}
#[inline]
pub unsafe fn PeekConsoleInputA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn PeekConsoleInputA(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_core::BOOL);
    unsafe { PeekConsoleInputA(hconsoleinput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventsread as _).ok() }
}
#[inline]
pub unsafe fn PeekConsoleInputW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn PeekConsoleInputW(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_core::BOOL);
    unsafe { PeekConsoleInputW(hconsoleinput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventsread as _).ok() }
}
#[inline]
pub unsafe fn ReadConsoleA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: Option<*const CONSOLE_READCONSOLE_CONTROL>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleA(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut core::ffi::c_void, nnumberofcharstoread : u32, lpnumberofcharsread : *mut u32, pinputcontrol : *const CONSOLE_READCONSOLE_CONTROL) -> windows_core::BOOL);
    unsafe { ReadConsoleA(hconsoleinput, lpbuffer as _, nnumberofcharstoread, lpnumberofcharsread as _, pinputcontrol.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn ReadConsoleInputA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleInputA(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_core::BOOL);
    unsafe { ReadConsoleInputA(hconsoleinput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventsread as _).ok() }
}
#[inline]
pub unsafe fn ReadConsoleInputExA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32, wflags: u16) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleInputExA(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32, wflags : u16) -> windows_core::BOOL);
    unsafe { ReadConsoleInputExA(hconsoleinput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventsread as _, wflags) }
}
#[inline]
pub unsafe fn ReadConsoleInputExW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32, wflags: u16) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleInputExW(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32, wflags : u16) -> windows_core::BOOL);
    unsafe { ReadConsoleInputExW(hconsoleinput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventsread as _, wflags) }
}
#[inline]
pub unsafe fn ReadConsoleInputW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleInputW(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_core::BOOL);
    unsafe { ReadConsoleInputW(hconsoleinput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventsread as _).ok() }
}
#[inline]
pub unsafe fn ReadConsoleOutputA(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *mut CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpreadregion: *mut SMALL_RECT) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleOutputA(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : *mut CHAR_INFO, dwbuffersize : COORD, dwbuffercoord : COORD, lpreadregion : *mut SMALL_RECT) -> windows_core::BOOL);
    unsafe { ReadConsoleOutputA(hconsoleoutput, lpbuffer as _, core::mem::transmute(dwbuffersize), core::mem::transmute(dwbuffercoord), lpreadregion as _).ok() }
}
#[inline]
pub unsafe fn ReadConsoleOutputAttribute(hconsoleoutput: super::super::Foundation::HANDLE, lpattribute: &mut [u16], dwreadcoord: COORD, lpnumberofattrsread: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleOutputAttribute(hconsoleoutput : super::super::Foundation:: HANDLE, lpattribute : *mut u16, nlength : u32, dwreadcoord : COORD, lpnumberofattrsread : *mut u32) -> windows_core::BOOL);
    unsafe { ReadConsoleOutputAttribute(hconsoleoutput, core::mem::transmute(lpattribute.as_ptr()), lpattribute.len().try_into().unwrap(), core::mem::transmute(dwreadcoord), lpnumberofattrsread as _).ok() }
}
#[inline]
pub unsafe fn ReadConsoleOutputCharacterA(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: &mut [u8], dwreadcoord: COORD, lpnumberofcharsread: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleOutputCharacterA(hconsoleoutput : super::super::Foundation:: HANDLE, lpcharacter : windows_core::PSTR, nlength : u32, dwreadcoord : COORD, lpnumberofcharsread : *mut u32) -> windows_core::BOOL);
    unsafe { ReadConsoleOutputCharacterA(hconsoleoutput, core::mem::transmute(lpcharacter.as_ptr()), lpcharacter.len().try_into().unwrap(), core::mem::transmute(dwreadcoord), lpnumberofcharsread as _).ok() }
}
#[inline]
pub unsafe fn ReadConsoleOutputCharacterW(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: &mut [u16], dwreadcoord: COORD, lpnumberofcharsread: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleOutputCharacterW(hconsoleoutput : super::super::Foundation:: HANDLE, lpcharacter : windows_core::PWSTR, nlength : u32, dwreadcoord : COORD, lpnumberofcharsread : *mut u32) -> windows_core::BOOL);
    unsafe { ReadConsoleOutputCharacterW(hconsoleoutput, core::mem::transmute(lpcharacter.as_ptr()), lpcharacter.len().try_into().unwrap(), core::mem::transmute(dwreadcoord), lpnumberofcharsread as _).ok() }
}
#[inline]
pub unsafe fn ReadConsoleOutputW(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *mut CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpreadregion: *mut SMALL_RECT) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleOutputW(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : *mut CHAR_INFO, dwbuffersize : COORD, dwbuffercoord : COORD, lpreadregion : *mut SMALL_RECT) -> windows_core::BOOL);
    unsafe { ReadConsoleOutputW(hconsoleoutput, lpbuffer as _, core::mem::transmute(dwbuffersize), core::mem::transmute(dwbuffercoord), lpreadregion as _).ok() }
}
#[inline]
pub unsafe fn ReadConsoleW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: *mut core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: Option<*const CONSOLE_READCONSOLE_CONTROL>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleW(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *mut core::ffi::c_void, nnumberofcharstoread : u32, lpnumberofcharsread : *mut u32, pinputcontrol : *const CONSOLE_READCONSOLE_CONTROL) -> windows_core::BOOL);
    unsafe { ReadConsoleW(hconsoleinput, lpbuffer as _, nnumberofcharstoread, lpnumberofcharsread as _, pinputcontrol.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn RegisterConsoleIME(hwndconsoleime: super::super::Foundation::HWND, lpdwconsolethreadid: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn RegisterConsoleIME(hwndconsoleime : super::super::Foundation:: HWND, lpdwconsolethreadid : *mut u32) -> windows_core::BOOL);
    unsafe { RegisterConsoleIME(hwndconsoleime, lpdwconsolethreadid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RegisterConsoleOS2(fos2register: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn RegisterConsoleOS2(fos2register : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { RegisterConsoleOS2(fos2register.into()) }
}
#[inline]
pub unsafe fn RegisterConsoleVDM(dwregisterflags: u32, hstarthardwareevent: super::super::Foundation::HANDLE, hendhardwareevent: super::super::Foundation::HANDLE, herrorhardwareevent: super::super::Foundation::HANDLE, reserved: Option<u32>, lpstatelength: *mut u32, lpstate: *mut *mut core::ffi::c_void, vdmbuffersize: Option<COORD>, lpvdmbuffer: *mut *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn RegisterConsoleVDM(dwregisterflags : u32, hstarthardwareevent : super::super::Foundation:: HANDLE, hendhardwareevent : super::super::Foundation:: HANDLE, herrorhardwareevent : super::super::Foundation:: HANDLE, reserved : u32, lpstatelength : *mut u32, lpstate : *mut *mut core::ffi::c_void, vdmbuffersize : COORD, lpvdmbuffer : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RegisterConsoleVDM(dwregisterflags, hstarthardwareevent, hendhardwareevent, herrorhardwareevent, reserved.unwrap_or(core::mem::zeroed()) as _, lpstatelength as _, lpstate as _, vdmbuffersize.unwrap_or(core::mem::zeroed()) as _, lpvdmbuffer as _) }
}
#[inline]
pub unsafe fn ReleasePseudoConsole(hpc: HPCON) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ReleasePseudoConsole(hpc : HPCON) -> windows_core::HRESULT);
    unsafe { ReleasePseudoConsole(hpc).ok() }
}
#[inline]
pub unsafe fn ResizePseudoConsole(hpc: HPCON, size: COORD) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ResizePseudoConsole(hpc : HPCON, size : COORD) -> windows_core::HRESULT);
    unsafe { ResizePseudoConsole(hpc, core::mem::transmute(size)).ok() }
}
#[inline]
pub unsafe fn ScrollConsoleScreenBufferA(hconsoleoutput: super::super::Foundation::HANDLE, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: Option<*const SMALL_RECT>, dwdestinationorigin: COORD, lpfill: *const CHAR_INFO) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ScrollConsoleScreenBufferA(hconsoleoutput : super::super::Foundation:: HANDLE, lpscrollrectangle : *const SMALL_RECT, lpcliprectangle : *const SMALL_RECT, dwdestinationorigin : COORD, lpfill : *const CHAR_INFO) -> windows_core::BOOL);
    unsafe { ScrollConsoleScreenBufferA(hconsoleoutput, lpscrollrectangle, lpcliprectangle.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(dwdestinationorigin), lpfill).ok() }
}
#[inline]
pub unsafe fn ScrollConsoleScreenBufferW(hconsoleoutput: super::super::Foundation::HANDLE, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: Option<*const SMALL_RECT>, dwdestinationorigin: COORD, lpfill: *const CHAR_INFO) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ScrollConsoleScreenBufferW(hconsoleoutput : super::super::Foundation:: HANDLE, lpscrollrectangle : *const SMALL_RECT, lpcliprectangle : *const SMALL_RECT, dwdestinationorigin : COORD, lpfill : *const CHAR_INFO) -> windows_core::BOOL);
    unsafe { ScrollConsoleScreenBufferW(hconsoleoutput, lpscrollrectangle, lpcliprectangle.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(dwdestinationorigin), lpfill).ok() }
}
#[inline]
pub unsafe fn SetConsoleActiveScreenBuffer(hconsoleoutput: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleActiveScreenBuffer(hconsoleoutput : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { SetConsoleActiveScreenBuffer(hconsoleoutput).ok() }
}
#[inline]
pub unsafe fn SetConsoleCP(wcodepageid: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleCP(wcodepageid : u32) -> windows_core::BOOL);
    unsafe { SetConsoleCP(wcodepageid).ok() }
}
#[inline]
pub unsafe fn SetConsoleCtrlHandler(handlerroutine: PHANDLER_ROUTINE, add: bool) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleCtrlHandler(handlerroutine : PHANDLER_ROUTINE, add : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetConsoleCtrlHandler(handlerroutine, add.into()).ok() }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn SetConsoleCursor(hconsoleoutput: super::super::Foundation::HANDLE, hcursor: super::super::UI::WindowsAndMessaging::HCURSOR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleCursor(hconsoleoutput : super::super::Foundation:: HANDLE, hcursor : super::super::UI::WindowsAndMessaging:: HCURSOR) -> windows_core::BOOL);
    unsafe { SetConsoleCursor(hconsoleoutput, hcursor) }
}
#[inline]
pub unsafe fn SetConsoleCursorInfo(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolecursorinfo: *const CONSOLE_CURSOR_INFO) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleCursorInfo(hconsoleoutput : super::super::Foundation:: HANDLE, lpconsolecursorinfo : *const CONSOLE_CURSOR_INFO) -> windows_core::BOOL);
    unsafe { SetConsoleCursorInfo(hconsoleoutput, lpconsolecursorinfo).ok() }
}
#[inline]
pub unsafe fn SetConsoleCursorMode(hconsolehandle: super::super::Foundation::HANDLE, blink: bool, dbenable: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleCursorMode(hconsolehandle : super::super::Foundation:: HANDLE, blink : windows_core::BOOL, dbenable : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetConsoleCursorMode(hconsolehandle, blink.into(), dbenable.into()) }
}
#[inline]
pub unsafe fn SetConsoleCursorPosition(hconsoleoutput: super::super::Foundation::HANDLE, dwcursorposition: COORD) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleCursorPosition(hconsoleoutput : super::super::Foundation:: HANDLE, dwcursorposition : COORD) -> windows_core::BOOL);
    unsafe { SetConsoleCursorPosition(hconsoleoutput, core::mem::transmute(dwcursorposition)).ok() }
}
#[inline]
pub unsafe fn SetConsoleDisplayMode(hconsoleoutput: super::super::Foundation::HANDLE, dwflags: u32, lpnewscreenbufferdimensions: Option<*mut COORD>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleDisplayMode(hconsoleoutput : super::super::Foundation:: HANDLE, dwflags : u32, lpnewscreenbufferdimensions : *mut COORD) -> windows_core::BOOL);
    unsafe { SetConsoleDisplayMode(hconsoleoutput, dwflags, lpnewscreenbufferdimensions.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn SetConsoleFont(hconsoleoutput: super::super::Foundation::HANDLE, nfont: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleFont(hconsoleoutput : super::super::Foundation:: HANDLE, nfont : u32) -> windows_core::BOOL);
    unsafe { SetConsoleFont(hconsoleoutput, nfont) }
}
#[inline]
pub unsafe fn SetConsoleHardwareState(hconsoleoutput: super::super::Foundation::HANDLE, dwresolution: COORD, dwfontsize: COORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleHardwareState(hconsoleoutput : super::super::Foundation:: HANDLE, dwresolution : COORD, dwfontsize : COORD) -> windows_core::BOOL);
    unsafe { SetConsoleHardwareState(hconsoleoutput, core::mem::transmute(dwresolution), core::mem::transmute(dwfontsize)) }
}
#[inline]
pub unsafe fn SetConsoleHistoryInfo(lpconsolehistoryinfo: *const CONSOLE_HISTORY_INFO) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleHistoryInfo(lpconsolehistoryinfo : *const CONSOLE_HISTORY_INFO) -> windows_core::BOOL);
    unsafe { SetConsoleHistoryInfo(lpconsolehistoryinfo).ok() }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn SetConsoleIcon(hicon: super::super::UI::WindowsAndMessaging::HICON) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleIcon(hicon : super::super::UI::WindowsAndMessaging:: HICON) -> windows_core::BOOL);
    unsafe { SetConsoleIcon(hicon) }
}
#[inline]
pub unsafe fn SetConsoleInputExeNameA<P0>(lpexename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetConsoleInputExeNameA(lpexename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetConsoleInputExeNameA(lpexename.param().abi()) }
}
#[inline]
pub unsafe fn SetConsoleInputExeNameW<P0>(lpexename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetConsoleInputExeNameW(lpexename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetConsoleInputExeNameW(lpexename.param().abi()) }
}
#[inline]
pub unsafe fn SetConsoleKeyShortcuts(bset: bool, breservekeys: u8, lpappkeys: &[APPKEY]) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleKeyShortcuts(bset : windows_core::BOOL, breservekeys : u8, lpappkeys : *const APPKEY, dwnumappkeys : u32) -> windows_core::BOOL);
    unsafe { SetConsoleKeyShortcuts(bset.into(), breservekeys, core::mem::transmute(lpappkeys.as_ptr()), lpappkeys.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SetConsoleLocalEUDC<P3>(hconsolehandle: super::super::Foundation::HANDLE, wcodepoint: u16, cfontsize: COORD, lpsb: P3) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetConsoleLocalEUDC(hconsolehandle : super::super::Foundation:: HANDLE, wcodepoint : u16, cfontsize : COORD, lpsb : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetConsoleLocalEUDC(hconsolehandle, wcodepoint, core::mem::transmute(cfontsize), lpsb.param().abi()) }
}
#[inline]
pub unsafe fn SetConsoleMenuClose(benable: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleMenuClose(benable : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetConsoleMenuClose(benable.into()) }
}
#[inline]
pub unsafe fn SetConsoleMode(hconsolehandle: super::super::Foundation::HANDLE, dwmode: CONSOLE_MODE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleMode(hconsolehandle : super::super::Foundation:: HANDLE, dwmode : CONSOLE_MODE) -> windows_core::BOOL);
    unsafe { SetConsoleMode(hconsolehandle, dwmode).ok() }
}
#[inline]
pub unsafe fn SetConsoleNlsMode(hconsole: super::super::Foundation::HANDLE, fdwnlsmode: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleNlsMode(hconsole : super::super::Foundation:: HANDLE, fdwnlsmode : u32) -> windows_core::BOOL);
    unsafe { SetConsoleNlsMode(hconsole, fdwnlsmode) }
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
#[inline]
pub unsafe fn SetConsoleOS2OemFormat(fos2oemformat: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleOS2OemFormat(fos2oemformat : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetConsoleOS2OemFormat(fos2oemformat.into()) }
}
#[inline]
pub unsafe fn SetConsoleOutputCP(wcodepageid: u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleOutputCP(wcodepageid : u32) -> windows_core::BOOL);
    unsafe { SetConsoleOutputCP(wcodepageid).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetConsolePalette(hconsoleoutput: super::super::Foundation::HANDLE, hpalette: super::super::Graphics::Gdi::HPALETTE, dwusage: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsolePalette(hconsoleoutput : super::super::Foundation:: HANDLE, hpalette : super::super::Graphics::Gdi:: HPALETTE, dwusage : u32) -> windows_core::BOOL);
    unsafe { SetConsolePalette(hconsoleoutput, hpalette, dwusage) }
}
#[inline]
pub unsafe fn SetConsoleScreenBufferInfoEx(hconsoleoutput: super::super::Foundation::HANDLE, lpconsolescreenbufferinfoex: *const CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleScreenBufferInfoEx(hconsoleoutput : super::super::Foundation:: HANDLE, lpconsolescreenbufferinfoex : *const CONSOLE_SCREEN_BUFFER_INFOEX) -> windows_core::BOOL);
    unsafe { SetConsoleScreenBufferInfoEx(hconsoleoutput, lpconsolescreenbufferinfoex).ok() }
}
#[inline]
pub unsafe fn SetConsoleScreenBufferSize(hconsoleoutput: super::super::Foundation::HANDLE, dwsize: COORD) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleScreenBufferSize(hconsoleoutput : super::super::Foundation:: HANDLE, dwsize : COORD) -> windows_core::BOOL);
    unsafe { SetConsoleScreenBufferSize(hconsoleoutput, core::mem::transmute(dwsize)).ok() }
}
#[inline]
pub unsafe fn SetConsoleTextAttribute(hconsoleoutput: super::super::Foundation::HANDLE, wattributes: CONSOLE_CHARACTER_ATTRIBUTES) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleTextAttribute(hconsoleoutput : super::super::Foundation:: HANDLE, wattributes : CONSOLE_CHARACTER_ATTRIBUTES) -> windows_core::BOOL);
    unsafe { SetConsoleTextAttribute(hconsoleoutput, wattributes).ok() }
}
#[inline]
pub unsafe fn SetConsoleTitleA<P0>(lpconsoletitle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetConsoleTitleA(lpconsoletitle : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetConsoleTitleA(lpconsoletitle.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetConsoleTitleW<P0>(lpconsoletitle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetConsoleTitleW(lpconsoletitle : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetConsoleTitleW(lpconsoletitle.param().abi()).ok() }
}
#[inline]
pub unsafe fn SetConsoleWindowInfo(hconsoleoutput: super::super::Foundation::HANDLE, babsolute: bool, lpconsolewindow: *const SMALL_RECT) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleWindowInfo(hconsoleoutput : super::super::Foundation:: HANDLE, babsolute : windows_core::BOOL, lpconsolewindow : *const SMALL_RECT) -> windows_core::BOOL);
    unsafe { SetConsoleWindowInfo(hconsoleoutput, babsolute.into(), lpconsolewindow).ok() }
}
#[inline]
pub unsafe fn SetCurrentConsoleFontEx(hconsoleoutput: super::super::Foundation::HANDLE, bmaximumwindow: bool, lpconsolecurrentfontex: *const CONSOLE_FONT_INFOEX) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetCurrentConsoleFontEx(hconsoleoutput : super::super::Foundation:: HANDLE, bmaximumwindow : windows_core::BOOL, lpconsolecurrentfontex : *const CONSOLE_FONT_INFOEX) -> windows_core::BOOL);
    unsafe { SetCurrentConsoleFontEx(hconsoleoutput, bmaximumwindow.into(), lpconsolecurrentfontex).ok() }
}
#[inline]
pub unsafe fn SetLastConsoleEventActive() {
    windows_core::link!("kernel32.dll" "system" fn SetLastConsoleEventActive());
    unsafe { SetLastConsoleEventActive() }
}
#[inline]
pub unsafe fn SetStdHandle(nstdhandle: STD_HANDLE, hhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetStdHandle(nstdhandle : STD_HANDLE, hhandle : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { SetStdHandle(nstdhandle, hhandle).ok() }
}
#[inline]
pub unsafe fn SetStdHandleEx(nstdhandle: STD_HANDLE, hhandle: super::super::Foundation::HANDLE, phprevvalue: Option<*mut super::super::Foundation::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetStdHandleEx(nstdhandle : STD_HANDLE, hhandle : super::super::Foundation:: HANDLE, phprevvalue : *mut super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { SetStdHandleEx(nstdhandle, hhandle, phprevvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ShowConsoleCursor(hconsoleoutput: super::super::Foundation::HANDLE, bshow: bool) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn ShowConsoleCursor(hconsoleoutput : super::super::Foundation:: HANDLE, bshow : windows_core::BOOL) -> i32);
    unsafe { ShowConsoleCursor(hconsoleoutput, bshow.into()) }
}
#[inline]
pub unsafe fn UnregisterConsoleIME() -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UnregisterConsoleIME() -> windows_core::BOOL);
    unsafe { UnregisterConsoleIME() }
}
#[inline]
pub unsafe fn VDMConsoleOperation(ifunction: u32, lpdata: Option<*mut core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn VDMConsoleOperation(ifunction : u32, lpdata : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { VDMConsoleOperation(ifunction, lpdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn VerifyConsoleIoHandle(hiohandle: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn VerifyConsoleIoHandle(hiohandle : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { VerifyConsoleIoHandle(hiohandle) }
}
#[inline]
pub unsafe fn WriteConsoleA(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: &[u8], lpnumberofcharswritten: Option<*mut u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleA(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : windows_core::PCSTR, nnumberofcharstowrite : u32, lpnumberofcharswritten : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WriteConsoleA(hconsoleoutput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofcharswritten.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WriteConsoleInputA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: &[INPUT_RECORD], lpnumberofeventswritten: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleInputA(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *const INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleInputA(hconsoleinput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventswritten as _).ok() }
}
#[inline]
pub unsafe fn WriteConsoleInputVDMA(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: &[INPUT_RECORD], lpnumberofeventswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleInputVDMA(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *const INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleInputVDMA(hconsoleinput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventswritten as _) }
}
#[inline]
pub unsafe fn WriteConsoleInputVDMW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: &[INPUT_RECORD], lpnumberofeventswritten: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleInputVDMW(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *const INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleInputVDMW(hconsoleinput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventswritten as _) }
}
#[inline]
pub unsafe fn WriteConsoleInputW(hconsoleinput: super::super::Foundation::HANDLE, lpbuffer: &[INPUT_RECORD], lpnumberofeventswritten: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleInputW(hconsoleinput : super::super::Foundation:: HANDLE, lpbuffer : *const INPUT_RECORD, nlength : u32, lpnumberofeventswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleInputW(hconsoleinput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventswritten as _).ok() }
}
#[inline]
pub unsafe fn WriteConsoleOutputA(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *const CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpwriteregion: *mut SMALL_RECT) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleOutputA(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : *const CHAR_INFO, dwbuffersize : COORD, dwbuffercoord : COORD, lpwriteregion : *mut SMALL_RECT) -> windows_core::BOOL);
    unsafe { WriteConsoleOutputA(hconsoleoutput, lpbuffer, core::mem::transmute(dwbuffersize), core::mem::transmute(dwbuffercoord), lpwriteregion as _).ok() }
}
#[inline]
pub unsafe fn WriteConsoleOutputAttribute(hconsoleoutput: super::super::Foundation::HANDLE, lpattribute: &[u16], dwwritecoord: COORD, lpnumberofattrswritten: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleOutputAttribute(hconsoleoutput : super::super::Foundation:: HANDLE, lpattribute : *const u16, nlength : u32, dwwritecoord : COORD, lpnumberofattrswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleOutputAttribute(hconsoleoutput, core::mem::transmute(lpattribute.as_ptr()), lpattribute.len().try_into().unwrap(), core::mem::transmute(dwwritecoord), lpnumberofattrswritten as _).ok() }
}
#[inline]
pub unsafe fn WriteConsoleOutputCharacterA(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: &[u8], dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleOutputCharacterA(hconsoleoutput : super::super::Foundation:: HANDLE, lpcharacter : windows_core::PCSTR, nlength : u32, dwwritecoord : COORD, lpnumberofcharswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleOutputCharacterA(hconsoleoutput, core::mem::transmute(lpcharacter.as_ptr()), lpcharacter.len().try_into().unwrap(), core::mem::transmute(dwwritecoord), lpnumberofcharswritten as _).ok() }
}
#[inline]
pub unsafe fn WriteConsoleOutputCharacterW(hconsoleoutput: super::super::Foundation::HANDLE, lpcharacter: &[u16], dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleOutputCharacterW(hconsoleoutput : super::super::Foundation:: HANDLE, lpcharacter : windows_core::PCWSTR, nlength : u32, dwwritecoord : COORD, lpnumberofcharswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WriteConsoleOutputCharacterW(hconsoleoutput, core::mem::transmute(lpcharacter.as_ptr()), lpcharacter.len().try_into().unwrap(), core::mem::transmute(dwwritecoord), lpnumberofcharswritten as _).ok() }
}
#[inline]
pub unsafe fn WriteConsoleOutputW(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: *const CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpwriteregion: *mut SMALL_RECT) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleOutputW(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : *const CHAR_INFO, dwbuffersize : COORD, dwbuffercoord : COORD, lpwriteregion : *mut SMALL_RECT) -> windows_core::BOOL);
    unsafe { WriteConsoleOutputW(hconsoleoutput, lpbuffer, core::mem::transmute(dwbuffersize), core::mem::transmute(dwbuffercoord), lpwriteregion as _).ok() }
}
#[inline]
pub unsafe fn WriteConsoleW(hconsoleoutput: super::super::Foundation::HANDLE, lpbuffer: &[u16], lpnumberofcharswritten: Option<*mut u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleW(hconsoleoutput : super::super::Foundation:: HANDLE, lpbuffer : windows_core::PCWSTR, nnumberofcharstowrite : u32, lpnumberofcharswritten : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WriteConsoleW(hconsoleoutput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofcharswritten.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ALLOC_CONSOLE_MODE(pub i32);
pub const ALLOC_CONSOLE_MODE_DEFAULT: ALLOC_CONSOLE_MODE = ALLOC_CONSOLE_MODE(0i32);
pub const ALLOC_CONSOLE_MODE_NEW_WINDOW: ALLOC_CONSOLE_MODE = ALLOC_CONSOLE_MODE(1i32);
pub const ALLOC_CONSOLE_MODE_NO_WINDOW: ALLOC_CONSOLE_MODE = ALLOC_CONSOLE_MODE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ALLOC_CONSOLE_OPTIONS {
    pub mode: ALLOC_CONSOLE_MODE,
    pub useShowWindow: windows_core::BOOL,
    pub showWindow: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ALLOC_CONSOLE_RESULT(pub i32);
pub const ALLOC_CONSOLE_RESULT_EXISTING_CONSOLE: ALLOC_CONSOLE_RESULT = ALLOC_CONSOLE_RESULT(2i32);
pub const ALLOC_CONSOLE_RESULT_NEW_CONSOLE: ALLOC_CONSOLE_RESULT = ALLOC_CONSOLE_RESULT(1i32);
pub const ALLOC_CONSOLE_RESULT_NO_CONSOLE: ALLOC_CONSOLE_RESULT = ALLOC_CONSOLE_RESULT(0i32);
pub const ALTNUMPAD_BIT: u32 = 67108864u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct APPKEY {
    pub Modifier: u16,
    pub ScanCode: u16,
}
pub const ATTACH_PARENT_PROCESS: u32 = 4294967295u32;
pub const BACKGROUND_BLUE: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(16u16);
pub const BACKGROUND_GREEN: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(32u16);
pub const BACKGROUND_INTENSITY: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(128u16);
pub const BACKGROUND_RED: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(64u16);
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
pub const COMMON_LVB_GRID_HORIZONTAL: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(1024u16);
pub const COMMON_LVB_GRID_LVERTICAL: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(2048u16);
pub const COMMON_LVB_GRID_RVERTICAL: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(4096u16);
pub const COMMON_LVB_LEADING_BYTE: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(256u16);
pub const COMMON_LVB_REVERSE_VIDEO: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(16384u16);
pub const COMMON_LVB_SBCSDBCS: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(768u16);
pub const COMMON_LVB_TRAILING_BYTE: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(512u16);
pub const COMMON_LVB_UNDERSCORE: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(32768u16);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONSOLECONTROL(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONSOLEENDTASK {
    pub ProcessId: super::super::Foundation::HANDLE,
    pub hwnd: super::super::Foundation::HWND,
    pub ConsoleEventCode: u32,
    pub ConsoleFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONSOLESETFOREGROUND {
    pub hProcess: super::super::Foundation::HANDLE,
    pub bForeground: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONSOLEWINDOWOWNER {
    pub hwnd: super::super::Foundation::HWND,
    pub ProcessId: u32,
    pub ThreadId: u32,
}
pub const CONSOLE_ALTENTER: u32 = 8u32;
pub const CONSOLE_ALTESC: u32 = 2u32;
pub const CONSOLE_ALTPRTSC: u32 = 16u32;
pub const CONSOLE_ALTSPACE: u32 = 4u32;
pub const CONSOLE_ALTTAB: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONSOLE_CARET_INFO {
    pub hwnd: super::super::Foundation::HWND,
    pub rc: super::super::Foundation::RECT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONSOLE_CHARACTER_ATTRIBUTES(pub u16);
impl CONSOLE_CHARACTER_ATTRIBUTES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CONSOLE_CHARACTER_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CONSOLE_CHARACTER_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CONSOLE_CHARACTER_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CONSOLE_CHARACTER_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CONSOLE_CHARACTER_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CONSOLE_CTRLESC: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONSOLE_CURSOR_INFO {
    pub dwSize: u32,
    pub bVisible: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONSOLE_FONT_INFO {
    pub nFont: u32,
    pub dwFontSize: COORD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const CONSOLE_GENERIC: windows_core::PCWSTR = windows_core::w!("CON");
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONSOLE_HISTORY_INFO {
    pub cbSize: u32,
    pub HistoryBufferSize: u32,
    pub NumberOfHistoryBuffers: u32,
    pub dwFlags: u32,
}
pub const CONSOLE_INPUT_STRING: windows_core::PCWSTR = windows_core::w!("CONIN$");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONSOLE_MODE(pub u32);
impl CONSOLE_MODE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CONSOLE_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CONSOLE_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CONSOLE_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CONSOLE_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CONSOLE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CONSOLE_MODIFIER_ALT: u32 = 8u32;
pub const CONSOLE_MODIFIER_CONTROL: u32 = 4u32;
pub const CONSOLE_MODIFIER_SHIFT: u32 = 3u32;
pub const CONSOLE_MOUSE_DOWN: u32 = 8u32;
pub const CONSOLE_MOUSE_SELECTION: u32 = 4u32;
pub const CONSOLE_NOSHORTCUTKEY: u32 = 0u32;
pub const CONSOLE_NO_SELECTION: u32 = 0u32;
pub const CONSOLE_OUTPUT_STRING: windows_core::PCWSTR = windows_core::w!("CONOUT$");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONSOLE_PROCESS_INFO {
    pub dwProcessID: u32,
    pub dwFlags: u32,
}
pub const CONSOLE_PRTSC: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONSOLE_READCONSOLE_CONTROL {
    pub nLength: u32,
    pub nInitialChars: u32,
    pub dwCtrlWakeupMask: u32,
    pub dwControlKeyState: u32,
}
pub const CONSOLE_REGISTER_VDM: u32 = 1u32;
pub const CONSOLE_REGISTER_WOW: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: CONSOLE_CHARACTER_ATTRIBUTES,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONSOLE_SCREEN_BUFFER_INFOEX {
    pub cbSize: u32,
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: CONSOLE_CHARACTER_ATTRIBUTES,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
    pub wPopupAttributes: u16,
    pub bFullscreenSupported: windows_core::BOOL,
    pub ColorTable: [super::super::Foundation::COLORREF; 16],
}
impl Default for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COORD {
    pub X: i16,
    pub Y: i16,
}
pub const CTRL_BREAK_EVENT: u32 = 1u32;
pub const CTRL_CLOSE_EVENT: u32 = 2u32;
pub const CTRL_C_EVENT: u32 = 0u32;
pub const CTRL_LOGOFF_EVENT: u32 = 5u32;
pub const CTRL_SHUTDOWN_EVENT: u32 = 6u32;
pub const ConsoleEndTask: CONSOLECONTROL = CONSOLECONTROL(7i32);
pub const ConsoleNotifyConsoleApplication: CONSOLECONTROL = CONSOLECONTROL(1i32);
pub const ConsoleSetCaretInfo: CONSOLECONTROL = CONSOLECONTROL(3i32);
pub const ConsoleSetForeground: CONSOLECONTROL = CONSOLECONTROL(5i32);
pub const ConsoleSetWindowOwner: CONSOLECONTROL = CONSOLECONTROL(6i32);
pub const DISABLE_NEWLINE_AUTO_RETURN: CONSOLE_MODE = CONSOLE_MODE(8u32);
pub const DOUBLE_CLICK: u32 = 2u32;
pub const ENABLE_AUTO_POSITION: CONSOLE_MODE = CONSOLE_MODE(256u32);
pub const ENABLE_ECHO_INPUT: CONSOLE_MODE = CONSOLE_MODE(4u32);
pub const ENABLE_EXTENDED_FLAGS: CONSOLE_MODE = CONSOLE_MODE(128u32);
pub const ENABLE_INSERT_MODE: CONSOLE_MODE = CONSOLE_MODE(32u32);
pub const ENABLE_LINE_INPUT: CONSOLE_MODE = CONSOLE_MODE(2u32);
pub const ENABLE_LVB_GRID_WORLDWIDE: CONSOLE_MODE = CONSOLE_MODE(16u32);
pub const ENABLE_MOUSE_INPUT: CONSOLE_MODE = CONSOLE_MODE(16u32);
pub const ENABLE_PROCESSED_INPUT: CONSOLE_MODE = CONSOLE_MODE(1u32);
pub const ENABLE_PROCESSED_OUTPUT: CONSOLE_MODE = CONSOLE_MODE(1u32);
pub const ENABLE_QUICK_EDIT_MODE: CONSOLE_MODE = CONSOLE_MODE(64u32);
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: CONSOLE_MODE = CONSOLE_MODE(512u32);
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: CONSOLE_MODE = CONSOLE_MODE(4u32);
pub const ENABLE_WINDOW_INPUT: CONSOLE_MODE = CONSOLE_MODE(8u32);
pub const ENABLE_WRAP_AT_EOL_OUTPUT: CONSOLE_MODE = CONSOLE_MODE(2u32);
pub const ENHANCED_KEY: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ExtKeySubst {
    pub wMod: u16,
    pub wVirKey: u16,
    pub wUnicodeChar: u16,
}
pub const FOCUS_EVENT: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FOCUS_EVENT_RECORD {
    pub bSetFocus: windows_core::BOOL,
}
pub const FOREGROUND_BLUE: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(1u16);
pub const FOREGROUND_GREEN: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(2u16);
pub const FOREGROUND_INTENSITY: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(8u16);
pub const FOREGROUND_RED: CONSOLE_CHARACTER_ATTRIBUTES = CONSOLE_CHARACTER_ATTRIBUTES(4u16);
pub const FROM_LEFT_1ST_BUTTON_PRESSED: u32 = 1u32;
pub const FROM_LEFT_2ND_BUTTON_PRESSED: u32 = 4u32;
pub const FROM_LEFT_3RD_BUTTON_PRESSED: u32 = 8u32;
pub const FROM_LEFT_4TH_BUTTON_PRESSED: u32 = 16u32;
pub const HISTORY_NO_DUP_FLAG: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HPCON(pub isize);
impl HPCON {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl windows_core::Free for HPCON {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("kernel32.dll" "system" fn ClosePseudoConsole(hpc : isize));
            unsafe {
                ClosePseudoConsole(self.0);
            }
        }
    }
}
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
    pub bKeyDown: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MENU_EVENT_RECORD {
    pub dwCommandId: u32,
}
pub const MOUSE_EVENT: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub type PHANDLER_ROUTINE = Option<unsafe extern "system" fn(ctrltype: u32) -> windows_core::BOOL>;
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
pub const Reserved1: CONSOLECONTROL = CONSOLECONTROL(0i32);
pub const Reserved2: CONSOLECONTROL = CONSOLECONTROL(2i32);
pub const Reserved3: CONSOLECONTROL = CONSOLECONTROL(4i32);
pub const SCROLLLOCK_ON: u32 = 64u32;
pub const SHIFT_PRESSED: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
pub const STD_ERROR_HANDLE: STD_HANDLE = STD_HANDLE(4294967284u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STD_HANDLE(pub u32);
pub const STD_INPUT_HANDLE: STD_HANDLE = STD_HANDLE(4294967286u32);
pub const STD_OUTPUT_HANDLE: STD_HANDLE = STD_HANDLE(4294967285u32);
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WINDOW_BUFFER_SIZE_RECORD {
    pub dwSize: COORD,
}
