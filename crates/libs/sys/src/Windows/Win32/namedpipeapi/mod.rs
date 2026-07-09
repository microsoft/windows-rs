windows_link::link!("kernel32.dll" "system" fn CallNamedPipeW(lpnamedpipename : windows_sys::core::PCWSTR, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, ntimeout : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn ConnectNamedPipe(hnamedpipe : super::winnt::HANDLE, lpoverlapped : *mut super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateNamedPipeW(lpname : windows_sys::core::PCWSTR, dwopenmode : u32, dwpipemode : u32, nmaxinstances : u32, noutbuffersize : u32, ninbuffersize : u32, ndefaulttimeout : u32, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreatePipe(hreadpipe : *mut super::winnt::HANDLE, hwritepipe : *mut super::winnt::HANDLE, lppipeattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, nsize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn DisconnectNamedPipe(hnamedpipe : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn GetNamedPipeClientComputerNameW(pipe : super::winnt::HANDLE, clientcomputername : windows_sys::core::PWSTR, clientcomputernamelength : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn GetNamedPipeHandleStateW(hnamedpipe : super::winnt::HANDLE, lpstate : *mut u32, lpcurinstances : *mut u32, lpmaxcollectioncount : *mut u32, lpcollectdatatimeout : *mut u32, lpusername : windows_sys::core::PWSTR, nmaxusernamesize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn GetNamedPipeInfo(hnamedpipe : super::winnt::HANDLE, lpflags : *mut u32, lpoutbuffersize : *mut u32, lpinbuffersize : *mut u32, lpmaxinstances : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("advapi32.dll" "system" fn ImpersonateNamedPipeClient(hnamedpipe : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn PeekNamedPipe(hnamedpipe : super::winnt::HANDLE, lpbuffer : *mut core::ffi::c_void, nbuffersize : u32, lpbytesread : *mut u32, lptotalbytesavail : *mut u32, lpbytesleftthismessage : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn SetNamedPipeHandleState(hnamedpipe : super::winnt::HANDLE, lpmode : *const u32, lpmaxcollectioncount : *const u32, lpcollectdatatimeout : *const u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn TransactNamedPipe(hnamedpipe : super::winnt::HANDLE, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WaitNamedPipeW(lpnamedpipename : windows_sys::core::PCWSTR, ntimeout : u32) -> windows_sys::core::BOOL);
