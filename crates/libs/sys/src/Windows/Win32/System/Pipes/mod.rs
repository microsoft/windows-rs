windows_targets::link!("kernel32.dll" "system" fn CallNamedPipeA(lpnamedpipename : windows_sys::core::PCSTR, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, ntimeout : u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn CallNamedPipeW(lpnamedpipename : windows_sys::core::PCWSTR, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, ntimeout : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("kernel32.dll" "system" fn ConnectNamedPipe(hnamedpipe : super::super::Foundation:: HANDLE, lpoverlapped : *mut super::IO:: OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
windows_targets::link!("kernel32.dll" "system" fn CreateNamedPipeA(lpname : windows_sys::core::PCSTR, dwopenmode : super::super::Storage::FileSystem:: FILE_FLAGS_AND_ATTRIBUTES, dwpipemode : NAMED_PIPE_MODE, nmaxinstances : u32, noutbuffersize : u32, ninbuffersize : u32, ndefaulttimeout : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
windows_targets::link!("kernel32.dll" "system" fn CreateNamedPipeW(lpname : windows_sys::core::PCWSTR, dwopenmode : super::super::Storage::FileSystem:: FILE_FLAGS_AND_ATTRIBUTES, dwpipemode : NAMED_PIPE_MODE, nmaxinstances : u32, noutbuffersize : u32, ninbuffersize : u32, ndefaulttimeout : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("kernel32.dll" "system" fn CreatePipe(hreadpipe : *mut super::super::Foundation:: HANDLE, hwritepipe : *mut super::super::Foundation:: HANDLE, lppipeattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, nsize : u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn DisconnectNamedPipe(hnamedpipe : super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeClientComputerNameA(pipe : super::super::Foundation:: HANDLE, clientcomputername : windows_sys::core::PSTR, clientcomputernamelength : u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeClientComputerNameW(pipe : super::super::Foundation:: HANDLE, clientcomputername : windows_sys::core::PWSTR, clientcomputernamelength : u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeClientProcessId(pipe : super::super::Foundation:: HANDLE, clientprocessid : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeClientSessionId(pipe : super::super::Foundation:: HANDLE, clientsessionid : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeHandleStateA(hnamedpipe : super::super::Foundation:: HANDLE, lpstate : *mut NAMED_PIPE_MODE, lpcurinstances : *mut u32, lpmaxcollectioncount : *mut u32, lpcollectdatatimeout : *mut u32, lpusername : windows_sys::core::PSTR, nmaxusernamesize : u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeHandleStateW(hnamedpipe : super::super::Foundation:: HANDLE, lpstate : *mut NAMED_PIPE_MODE, lpcurinstances : *mut u32, lpmaxcollectioncount : *mut u32, lpcollectdatatimeout : *mut u32, lpusername : windows_sys::core::PWSTR, nmaxusernamesize : u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeInfo(hnamedpipe : super::super::Foundation:: HANDLE, lpflags : *mut NAMED_PIPE_MODE, lpoutbuffersize : *mut u32, lpinbuffersize : *mut u32, lpmaxinstances : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeServerProcessId(pipe : super::super::Foundation:: HANDLE, serverprocessid : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeServerSessionId(pipe : super::super::Foundation:: HANDLE, serversessionid : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("advapi32.dll" "system" fn ImpersonateNamedPipeClient(hnamedpipe : super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn PeekNamedPipe(hnamedpipe : super::super::Foundation:: HANDLE, lpbuffer : *mut core::ffi::c_void, nbuffersize : u32, lpbytesread : *mut u32, lptotalbytesavail : *mut u32, lpbytesleftthismessage : *mut u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn SetNamedPipeHandleState(hnamedpipe : super::super::Foundation:: HANDLE, lpmode : *const NAMED_PIPE_MODE, lpmaxcollectioncount : *const u32, lpcollectdatatimeout : *const u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("kernel32.dll" "system" fn TransactNamedPipe(hnamedpipe : super::super::Foundation:: HANDLE, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, lpoverlapped : *mut super::IO:: OVERLAPPED) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn WaitNamedPipeA(lpnamedpipename : windows_sys::core::PCSTR, ntimeout : u32) -> windows_sys::core::BOOL);
windows_targets::link!("kernel32.dll" "system" fn WaitNamedPipeW(lpnamedpipename : windows_sys::core::PCWSTR, ntimeout : u32) -> windows_sys::core::BOOL);
pub type NAMED_PIPE_MODE = u32;
pub const NMPWAIT_NOWAIT: u32 = 1u32;
pub const NMPWAIT_USE_DEFAULT_WAIT: u32 = 0u32;
pub const NMPWAIT_WAIT_FOREVER: u32 = 4294967295u32;
pub const PIPE_ACCEPT_REMOTE_CLIENTS: NAMED_PIPE_MODE = 0u32;
pub const PIPE_CLIENT_END: NAMED_PIPE_MODE = 0u32;
pub const PIPE_NOWAIT: NAMED_PIPE_MODE = 1u32;
pub const PIPE_READMODE_BYTE: NAMED_PIPE_MODE = 0u32;
pub const PIPE_READMODE_MESSAGE: NAMED_PIPE_MODE = 2u32;
pub const PIPE_REJECT_REMOTE_CLIENTS: NAMED_PIPE_MODE = 8u32;
pub const PIPE_SERVER_END: NAMED_PIPE_MODE = 1u32;
pub const PIPE_TYPE_BYTE: NAMED_PIPE_MODE = 0u32;
pub const PIPE_TYPE_MESSAGE: NAMED_PIPE_MODE = 4u32;
pub const PIPE_UNLIMITED_INSTANCES: u32 = 255u32;
pub const PIPE_WAIT: NAMED_PIPE_MODE = 0u32;
