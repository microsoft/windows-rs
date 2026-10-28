#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CancelIo(hfile : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CancelIoEx(hfile : super::HANDLE, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CancelSynchronousIo(hthread : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateIoCompletionPort(filehandle : super::HANDLE, existingcompletionport : super::HANDLE, completionkey : super::ULONG_PTR, numberofconcurrentthreads : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn DeviceIoControl(hdevice : super::HANDLE, dwiocontrolcode : super::DWORD, lpinbuffer : super::LPVOID, ninbuffersize : super::DWORD, lpoutbuffer : super::LPVOID, noutbuffersize : super::DWORD, lpbytesreturned : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetOverlappedResult(hfile : super::HANDLE, lpoverlapped : super::LPOVERLAPPED, lpnumberofbytestransferred : super::LPDWORD, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetOverlappedResultEx(hfile : super::HANDLE, lpoverlapped : super::LPOVERLAPPED, lpnumberofbytestransferred : super::LPDWORD, dwmilliseconds : super::DWORD, balertable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetQueuedCompletionStatus(completionport : super::HANDLE, lpnumberofbytestransferred : super::LPDWORD, lpcompletionkey : super::PULONG_PTR, lpoverlapped : *mut super::LPOVERLAPPED, dwmilliseconds : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetQueuedCompletionStatusEx(completionport : super::HANDLE, lpcompletionportentries : super::LPOVERLAPPED_ENTRY, ulcount : super::ULONG, ulnumentriesremoved : super::PULONG, dwmilliseconds : super::DWORD, falertable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn PostQueuedCompletionStatus(completionport : super::HANDLE, dwnumberofbytestransferred : super::DWORD, dwcompletionkey : super::ULONG_PTR, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
