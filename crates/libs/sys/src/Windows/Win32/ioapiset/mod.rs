#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CancelIo(hfile : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CancelIoEx(hfile : super::HANDLE, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CancelSynchronousIo(hthread : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateIoCompletionPort(filehandle : super::HANDLE, existingcompletionport : super::HANDLE, completionkey : usize, numberofconcurrentthreads : u32) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn DeviceIoControl(hdevice : super::HANDLE, dwiocontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetOverlappedResult(hfile : super::HANDLE, lpoverlapped : super::LPOVERLAPPED, lpnumberofbytestransferred : super::LPDWORD, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetOverlappedResultEx(hfile : super::HANDLE, lpoverlapped : super::LPOVERLAPPED, lpnumberofbytestransferred : super::LPDWORD, dwmilliseconds : u32, balertable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetQueuedCompletionStatus(completionport : super::HANDLE, lpnumberofbytestransferred : super::LPDWORD, lpcompletionkey : super::PULONG_PTR, lpoverlapped : *mut super::LPOVERLAPPED, dwmilliseconds : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetQueuedCompletionStatusEx(completionport : super::HANDLE, lpcompletionportentries : super::LPOVERLAPPED_ENTRY, ulcount : u32, ulnumentriesremoved : super::PULONG, dwmilliseconds : u32, falertable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn PostQueuedCompletionStatus(completionport : super::HANDLE, dwnumberofbytestransferred : u32, dwcompletionkey : usize, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
