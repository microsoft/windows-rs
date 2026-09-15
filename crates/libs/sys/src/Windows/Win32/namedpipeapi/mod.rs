#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn CallNamedPipeW(lpnamedpipename : windows_sys::core::PCWSTR, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : super::LPDWORD, ntimeout : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ConnectNamedPipe(hnamedpipe : super::HANDLE, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateNamedPipeW(lpname : windows_sys::core::PCWSTR, dwopenmode : u32, dwpipemode : u32, nmaxinstances : u32, noutbuffersize : u32, ninbuffersize : u32, ndefaulttimeout : u32, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreatePipe(hreadpipe : super::PHANDLE, hwritepipe : super::PHANDLE, lppipeattributes : super::LPSECURITY_ATTRIBUTES, nsize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DisconnectNamedPipe(hnamedpipe : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetNamedPipeClientComputerNameW(pipe : super::HANDLE, clientcomputername : windows_sys::core::PWSTR, clientcomputernamelength : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetNamedPipeHandleStateW(hnamedpipe : super::HANDLE, lpstate : super::LPDWORD, lpcurinstances : super::LPDWORD, lpmaxcollectioncount : super::LPDWORD, lpcollectdatatimeout : super::LPDWORD, lpusername : windows_sys::core::PWSTR, nmaxusernamesize : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetNamedPipeInfo(hnamedpipe : super::HANDLE, lpflags : super::LPDWORD, lpoutbuffersize : super::LPDWORD, lpinbuffersize : super::LPDWORD, lpmaxinstances : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ImpersonateNamedPipeClient(hnamedpipe : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn PeekNamedPipe(hnamedpipe : super::HANDLE, lpbuffer : *mut core::ffi::c_void, nbuffersize : u32, lpbytesread : super::LPDWORD, lptotalbytesavail : super::LPDWORD, lpbytesleftthismessage : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetNamedPipeHandleState(hnamedpipe : super::HANDLE, lpmode : super::LPDWORD, lpmaxcollectioncount : super::LPDWORD, lpcollectdatatimeout : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn TransactNamedPipe(hnamedpipe : super::HANDLE, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WaitNamedPipeW(lpnamedpipename : windows_sys::core::PCWSTR, ntimeout : u32) -> windows_sys::core::BOOL);
