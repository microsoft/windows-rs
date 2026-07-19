windows_link::link!("kernel32.dll" "system" fn AllocConsole() -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn AllocConsoleWithOptions(options : *const ALLOC_CONSOLE_OPTIONS, result : *mut ALLOC_CONSOLE_RESULT) -> windows_sys::core::HRESULT);
windows_link::link!("kernel32.dll" "system" fn AttachConsole(dwprocessid : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "wincontypes")]
windows_link::link!("kernel32.dll" "system" fn ClosePseudoConsole(hpc : super::HPCON));
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreatePseudoConsole(size : super::COORD, hinput : super::HANDLE, houtput : super::HANDLE, dwflags : u32, phpc : *mut super::HPCON) -> windows_sys::core::HRESULT);
windows_link::link!("kernel32.dll" "system" fn FreeConsole() -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetConsoleCP() -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetConsoleMode(hconsolehandle : super::HANDLE, lpmode : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetConsoleOutputCP() -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetNumberOfConsoleInputEvents(hconsoleinput : super::HANDLE, lpnumberofevents : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn PeekConsoleInputA(hconsoleinput : super::HANDLE, lpbuffer : *mut super::INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn PeekConsoleInputW(hconsoleinput : super::HANDLE, lpbuffer : *mut super::INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ReadConsoleA(hconsoleinput : super::HANDLE, lpbuffer : *mut core::ffi::c_void, nnumberofcharstoread : u32, lpnumberofcharsread : *mut u32, pinputcontrol : *const CONSOLE_READCONSOLE_CONTROL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReadConsoleInputA(hconsoleinput : super::HANDLE, lpbuffer : *mut super::INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wincontypes", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReadConsoleInputW(hconsoleinput : super::HANDLE, lpbuffer : *mut super::INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ReadConsoleW(hconsoleinput : super::HANDLE, lpbuffer : *mut core::ffi::c_void, nnumberofcharstoread : u32, lpnumberofcharsread : *mut u32, pinputcontrol : *const CONSOLE_READCONSOLE_CONTROL) -> windows_sys::core::BOOL);
#[cfg(feature = "wincontypes")]
windows_link::link!("kernel32.dll" "system" fn ReleasePseudoConsole(hpc : super::HPCON) -> windows_sys::core::HRESULT);
#[cfg(feature = "wincontypes")]
windows_link::link!("kernel32.dll" "system" fn ResizePseudoConsole(hpc : super::HPCON, size : super::COORD) -> windows_sys::core::HRESULT);
windows_link::link!("kernel32.dll" "system" fn SetConsoleCtrlHandler(handlerroutine : PHANDLER_ROUTINE, add : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetConsoleMode(hconsolehandle : super::HANDLE, dwmode : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WriteConsoleA(hconsoleoutput : super::HANDLE, lpbuffer : *const core::ffi::c_void, nnumberofcharstowrite : u32, lpnumberofcharswritten : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WriteConsoleW(hconsoleoutput : super::HANDLE, lpbuffer : *const core::ffi::c_void, nnumberofcharstowrite : u32, lpnumberofcharswritten : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
pub type ALLOC_CONSOLE_MODE = i32;
pub const ALLOC_CONSOLE_MODE_DEFAULT: ALLOC_CONSOLE_MODE = 0;
pub const ALLOC_CONSOLE_MODE_NEW_WINDOW: ALLOC_CONSOLE_MODE = 1;
pub const ALLOC_CONSOLE_MODE_NO_WINDOW: ALLOC_CONSOLE_MODE = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ALLOC_CONSOLE_OPTIONS {
    pub mode: ALLOC_CONSOLE_MODE,
    pub useShowWindow: windows_sys::core::BOOL,
    pub showWindow: u16,
}
pub type ALLOC_CONSOLE_RESULT = i32;
pub const ALLOC_CONSOLE_RESULT_EXISTING_CONSOLE: ALLOC_CONSOLE_RESULT = 2;
pub const ALLOC_CONSOLE_RESULT_NEW_CONSOLE: ALLOC_CONSOLE_RESULT = 1;
pub const ALLOC_CONSOLE_RESULT_NO_CONSOLE: ALLOC_CONSOLE_RESULT = 0;
pub const ATTACH_PARENT_PROCESS: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_READCONSOLE_CONTROL {
    pub nLength: u32,
    pub nInitialChars: u32,
    pub dwCtrlWakeupMask: u32,
    pub dwControlKeyState: u32,
}
pub const CTRL_BREAK_EVENT: u32 = 1;
pub const CTRL_CLOSE_EVENT: u32 = 2;
pub const CTRL_C_EVENT: u32 = 0;
pub const CTRL_LOGOFF_EVENT: u32 = 5;
pub const CTRL_SHUTDOWN_EVENT: u32 = 6;
pub const DISABLE_NEWLINE_AUTO_RETURN: u32 = 8;
pub const ENABLE_AUTO_POSITION: u32 = 256;
pub const ENABLE_ECHO_INPUT: u32 = 4;
pub const ENABLE_EXTENDED_FLAGS: u32 = 128;
pub const ENABLE_INSERT_MODE: u32 = 32;
pub const ENABLE_LINE_INPUT: u32 = 2;
pub const ENABLE_LVB_GRID_WORLDWIDE: u32 = 16;
pub const ENABLE_MOUSE_INPUT: u32 = 16;
pub const ENABLE_PROCESSED_INPUT: u32 = 1;
pub const ENABLE_PROCESSED_OUTPUT: u32 = 1;
pub const ENABLE_QUICK_EDIT_MODE: u32 = 64;
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: u32 = 512;
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 4;
pub const ENABLE_WINDOW_INPUT: u32 = 8;
pub const ENABLE_WRAP_AT_EOL_OUTPUT: u32 = 2;
pub type PALLOC_CONSOLE_OPTIONS = *mut ALLOC_CONSOLE_OPTIONS;
pub type PALLOC_CONSOLE_RESULT = *mut ALLOC_CONSOLE_RESULT;
pub type PCONSOLE_READCONSOLE_CONTROL = *mut CONSOLE_READCONSOLE_CONTROL;
pub type PHANDLER_ROUTINE = Option<unsafe extern "system" fn(ctrltype: u32) -> windows_sys::core::BOOL>;
pub const PSEUDOCONSOLE_INHERIT_CURSOR: u32 = 1;
