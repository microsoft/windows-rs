#[inline]
pub unsafe fn AllocConsole() -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AllocConsole() -> windows_core::BOOL);
    unsafe { AllocConsole() }
}
#[inline]
pub unsafe fn AllocConsoleWithOptions(options: Option<*const ALLOC_CONSOLE_OPTIONS>, result: Option<*mut ALLOC_CONSOLE_RESULT>) -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn AllocConsoleWithOptions(options : *const ALLOC_CONSOLE_OPTIONS, result : *mut ALLOC_CONSOLE_RESULT) -> windows_core::HRESULT);
    unsafe { AllocConsoleWithOptions(options.unwrap_or(core::mem::zeroed()) as _, result.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AttachConsole(dwprocessid: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AttachConsole(dwprocessid : u32) -> windows_core::BOOL);
    unsafe { AttachConsole(dwprocessid) }
}
#[cfg(feature = "Win32_wincontypes")]
#[inline]
pub unsafe fn ClosePseudoConsole(hpc: super::wincontypes::HPCON) {
    windows_core::link!("kernel32.dll" "system" fn ClosePseudoConsole(hpc : super::wincontypes::HPCON));
    unsafe { ClosePseudoConsole(hpc) }
}
#[cfg(all(feature = "Win32_wincontypes", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreatePseudoConsole(size: super::wincontypes::COORD, hinput: super::winnt::HANDLE, houtput: super::winnt::HANDLE, dwflags: u32) -> windows_core::Result<super::wincontypes::HPCON> {
    windows_core::link!("kernel32.dll" "system" fn CreatePseudoConsole(size : super::wincontypes::COORD, hinput : super::winnt::HANDLE, houtput : super::winnt::HANDLE, dwflags : u32, phpc : *mut super::wincontypes::HPCON) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreatePseudoConsole(core::mem::transmute(size), hinput, houtput, dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn FreeConsole() -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FreeConsole() -> windows_core::BOOL);
    unsafe { FreeConsole() }
}
#[inline]
pub unsafe fn GetConsoleCP() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleCP() -> u32);
    unsafe { GetConsoleCP() }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetConsoleMode(hconsolehandle: super::winnt::HANDLE, lpmode: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleMode(hconsolehandle : super::winnt::HANDLE, lpmode : *mut u32) -> windows_core::BOOL);
    unsafe { GetConsoleMode(hconsolehandle, lpmode as _) }
}
#[inline]
pub unsafe fn GetConsoleOutputCP() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetConsoleOutputCP() -> u32);
    unsafe { GetConsoleOutputCP() }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetNumberOfConsoleInputEvents(hconsoleinput: super::winnt::HANDLE, lpnumberofevents: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumberOfConsoleInputEvents(hconsoleinput : super::winnt::HANDLE, lpnumberofevents : *mut u32) -> windows_core::BOOL);
    unsafe { GetNumberOfConsoleInputEvents(hconsoleinput, lpnumberofevents as _) }
}
#[cfg(all(feature = "Win32_wincontypes", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn PeekConsoleInputA(hconsoleinput: super::winnt::HANDLE, lpbuffer: &mut [super::wincontypes::INPUT_RECORD], lpnumberofeventsread: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn PeekConsoleInputA(hconsoleinput : super::winnt::HANDLE, lpbuffer : *mut super::wincontypes::INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_core::BOOL);
    unsafe { PeekConsoleInputA(hconsoleinput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventsread as _) }
}
#[cfg(all(feature = "Win32_wincontypes", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn PeekConsoleInputW(hconsoleinput: super::winnt::HANDLE, lpbuffer: &mut [super::wincontypes::INPUT_RECORD], lpnumberofeventsread: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn PeekConsoleInputW(hconsoleinput : super::winnt::HANDLE, lpbuffer : *mut super::wincontypes::INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_core::BOOL);
    unsafe { PeekConsoleInputW(hconsoleinput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventsread as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn ReadConsoleA(hconsoleinput: super::winnt::HANDLE, lpbuffer: *mut core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: Option<*const CONSOLE_READCONSOLE_CONTROL>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleA(hconsoleinput : super::winnt::HANDLE, lpbuffer : *mut core::ffi::c_void, nnumberofcharstoread : u32, lpnumberofcharsread : *mut u32, pinputcontrol : *const CONSOLE_READCONSOLE_CONTROL) -> windows_core::BOOL);
    unsafe { ReadConsoleA(hconsoleinput, lpbuffer as _, nnumberofcharstoread, lpnumberofcharsread as _, pinputcontrol.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_wincontypes", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn ReadConsoleInputA(hconsoleinput: super::winnt::HANDLE, lpbuffer: &mut [super::wincontypes::INPUT_RECORD], lpnumberofeventsread: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleInputA(hconsoleinput : super::winnt::HANDLE, lpbuffer : *mut super::wincontypes::INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_core::BOOL);
    unsafe { ReadConsoleInputA(hconsoleinput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventsread as _) }
}
#[cfg(all(feature = "Win32_wincontypes", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn ReadConsoleInputW(hconsoleinput: super::winnt::HANDLE, lpbuffer: &mut [super::wincontypes::INPUT_RECORD], lpnumberofeventsread: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleInputW(hconsoleinput : super::winnt::HANDLE, lpbuffer : *mut super::wincontypes::INPUT_RECORD, nlength : u32, lpnumberofeventsread : *mut u32) -> windows_core::BOOL);
    unsafe { ReadConsoleInputW(hconsoleinput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofeventsread as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn ReadConsoleW(hconsoleinput: super::winnt::HANDLE, lpbuffer: *mut core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: Option<*const CONSOLE_READCONSOLE_CONTROL>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadConsoleW(hconsoleinput : super::winnt::HANDLE, lpbuffer : *mut core::ffi::c_void, nnumberofcharstoread : u32, lpnumberofcharsread : *mut u32, pinputcontrol : *const CONSOLE_READCONSOLE_CONTROL) -> windows_core::BOOL);
    unsafe { ReadConsoleW(hconsoleinput, lpbuffer as _, nnumberofcharstoread, lpnumberofcharsread as _, pinputcontrol.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_wincontypes")]
#[inline]
pub unsafe fn ReleasePseudoConsole(hpc: super::wincontypes::HPCON) -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn ReleasePseudoConsole(hpc : super::wincontypes::HPCON) -> windows_core::HRESULT);
    unsafe { ReleasePseudoConsole(hpc) }
}
#[cfg(feature = "Win32_wincontypes")]
#[inline]
pub unsafe fn ResizePseudoConsole(hpc: super::wincontypes::HPCON, size: super::wincontypes::COORD) -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn ResizePseudoConsole(hpc : super::wincontypes::HPCON, size : super::wincontypes::COORD) -> windows_core::HRESULT);
    unsafe { ResizePseudoConsole(hpc, core::mem::transmute(size)) }
}
#[inline]
pub unsafe fn SetConsoleCtrlHandler(handlerroutine: PHANDLER_ROUTINE, add: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleCtrlHandler(handlerroutine : PHANDLER_ROUTINE, add : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetConsoleCtrlHandler(handlerroutine, add.into()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetConsoleMode(hconsolehandle: super::winnt::HANDLE, dwmode: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetConsoleMode(hconsolehandle : super::winnt::HANDLE, dwmode : u32) -> windows_core::BOOL);
    unsafe { SetConsoleMode(hconsolehandle, dwmode) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn WriteConsoleA(hconsoleoutput: super::winnt::HANDLE, lpbuffer: &[u8], lpnumberofcharswritten: Option<*mut u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleA(hconsoleoutput : super::winnt::HANDLE, lpbuffer : *const core::ffi::c_void, nnumberofcharstowrite : u32, lpnumberofcharswritten : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WriteConsoleA(hconsoleoutput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofcharswritten.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn WriteConsoleW(hconsoleoutput: super::winnt::HANDLE, lpbuffer: &[u8], lpnumberofcharswritten: Option<*mut u32>, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteConsoleW(hconsoleoutput : super::winnt::HANDLE, lpbuffer : *const core::ffi::c_void, nnumberofcharstowrite : u32, lpnumberofcharswritten : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WriteConsoleW(hconsoleoutput, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofcharswritten.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
pub type ALLOC_CONSOLE_MODE = i32;
pub const ALLOC_CONSOLE_MODE_DEFAULT: ALLOC_CONSOLE_MODE = 0;
pub const ALLOC_CONSOLE_MODE_NEW_WINDOW: ALLOC_CONSOLE_MODE = 1;
pub const ALLOC_CONSOLE_MODE_NO_WINDOW: ALLOC_CONSOLE_MODE = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ALLOC_CONSOLE_OPTIONS {
    pub mode: ALLOC_CONSOLE_MODE,
    pub useShowWindow: windows_core::BOOL,
    pub showWindow: u16,
}
pub type ALLOC_CONSOLE_RESULT = i32;
pub const ALLOC_CONSOLE_RESULT_EXISTING_CONSOLE: ALLOC_CONSOLE_RESULT = 2;
pub const ALLOC_CONSOLE_RESULT_NEW_CONSOLE: ALLOC_CONSOLE_RESULT = 1;
pub const ALLOC_CONSOLE_RESULT_NO_CONSOLE: ALLOC_CONSOLE_RESULT = 0;
pub const ATTACH_PARENT_PROCESS: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PALLOC_CONSOLE_OPTIONS(pub *mut ALLOC_CONSOLE_OPTIONS);
impl PALLOC_CONSOLE_OPTIONS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PALLOC_CONSOLE_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PALLOC_CONSOLE_RESULT(pub *mut ALLOC_CONSOLE_RESULT);
impl PALLOC_CONSOLE_RESULT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PALLOC_CONSOLE_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCONSOLE_READCONSOLE_CONTROL(pub *mut CONSOLE_READCONSOLE_CONTROL);
impl PCONSOLE_READCONSOLE_CONTROL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCONSOLE_READCONSOLE_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PHANDLER_ROUTINE = Option<unsafe extern "system" fn(ctrltype: u32) -> windows_core::BOOL>;
pub const PSEUDOCONSOLE_INHERIT_CURSOR: u32 = 1;
