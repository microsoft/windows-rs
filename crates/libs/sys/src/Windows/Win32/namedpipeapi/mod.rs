#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn CallNamedPipeW(lpnamedpipename : windows_sys::core::PCWSTR, lpinbuffer : super::LPVOID, ninbuffersize : super::DWORD, lpoutbuffer : super::LPVOID, noutbuffersize : super::DWORD, lpbytesread : super::LPDWORD, ntimeout : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ConnectNamedPipe(hnamedpipe : super::HANDLE, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateNamedPipeW(lpname : windows_sys::core::PCWSTR, dwopenmode : super::DWORD, dwpipemode : super::DWORD, nmaxinstances : super::DWORD, noutbuffersize : super::DWORD, ninbuffersize : super::DWORD, ndefaulttimeout : super::DWORD, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreatePipe(hreadpipe : super::PHANDLE, hwritepipe : super::PHANDLE, lppipeattributes : super::LPSECURITY_ATTRIBUTES, nsize : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DisconnectNamedPipe(hnamedpipe : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetNamedPipeClientComputerNameW(pipe : super::HANDLE, clientcomputername : windows_sys::core::PWSTR, clientcomputernamelength : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetNamedPipeHandleStateW(hnamedpipe : super::HANDLE, lpstate : super::LPDWORD, lpcurinstances : super::LPDWORD, lpmaxcollectioncount : super::LPDWORD, lpcollectdatatimeout : super::LPDWORD, lpusername : windows_sys::core::PWSTR, nmaxusernamesize : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetNamedPipeInfo(hnamedpipe : super::HANDLE, lpflags : super::LPDWORD, lpoutbuffersize : super::LPDWORD, lpinbuffersize : super::LPDWORD, lpmaxinstances : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ImpersonateNamedPipeClient(hnamedpipe : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn PeekNamedPipe(hnamedpipe : super::HANDLE, lpbuffer : super::LPVOID, nbuffersize : super::DWORD, lpbytesread : super::LPDWORD, lptotalbytesavail : super::LPDWORD, lpbytesleftthismessage : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetNamedPipeHandleState(hnamedpipe : super::HANDLE, lpmode : super::LPDWORD, lpmaxcollectioncount : super::LPDWORD, lpcollectdatatimeout : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn TransactNamedPipe(hnamedpipe : super::HANDLE, lpinbuffer : super::LPVOID, ninbuffersize : super::DWORD, lpoutbuffer : super::LPVOID, noutbuffersize : super::DWORD, lpbytesread : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn WaitNamedPipeW(lpnamedpipename : windows_sys::core::PCWSTR, ntimeout : super::DWORD) -> windows_sys::core::BOOL);
