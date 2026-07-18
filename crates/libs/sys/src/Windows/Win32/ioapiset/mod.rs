#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CancelIo(hfile : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CancelIoEx(hfile : super::HANDLE, lpoverlapped : *const super::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CancelSynchronousIo(hthread : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateIoCompletionPort(filehandle : super::HANDLE, existingcompletionport : super::HANDLE, completionkey : usize, numberofconcurrentthreads : u32) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn DeviceIoControl(hdevice : super::HANDLE, dwiocontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpoverlapped : *mut super::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetOverlappedResult(hfile : super::HANDLE, lpoverlapped : *const super::OVERLAPPED, lpnumberofbytestransferred : *mut u32, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetOverlappedResultEx(hfile : super::HANDLE, lpoverlapped : *const super::OVERLAPPED, lpnumberofbytestransferred : *mut u32, dwmilliseconds : u32, balertable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetQueuedCompletionStatus(completionport : super::HANDLE, lpnumberofbytestransferred : *mut u32, lpcompletionkey : *mut u32, lpoverlapped : *mut super::LPOVERLAPPED, dwmilliseconds : u32) -> windows_sys::core::BOOL);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetQueuedCompletionStatus(completionport : super::HANDLE, lpnumberofbytestransferred : *mut u32, lpcompletionkey : *mut u64, lpoverlapped : *mut super::LPOVERLAPPED, dwmilliseconds : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetQueuedCompletionStatusEx(completionport : super::HANDLE, lpcompletionportentries : *mut super::OVERLAPPED_ENTRY, ulcount : u32, ulnumentriesremoved : *mut u32, dwmilliseconds : u32, falertable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn PostQueuedCompletionStatus(completionport : super::HANDLE, dwnumberofbytestransferred : u32, dwcompletionkey : usize, lpoverlapped : *const super::OVERLAPPED) -> windows_sys::core::BOOL);
