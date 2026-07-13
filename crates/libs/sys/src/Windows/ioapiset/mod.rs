#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CancelIo(hfile : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CancelIoEx(hfile : super::winnt::HANDLE, lpoverlapped : *const super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CancelSynchronousIo(hthread : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateIoCompletionPort(filehandle : super::winnt::HANDLE, existingcompletionport : super::winnt::HANDLE, completionkey : usize, numberofconcurrentthreads : u32) -> super::winnt::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn DeviceIoControl(hdevice : super::winnt::HANDLE, dwiocontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetOverlappedResult(hfile : super::winnt::HANDLE, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpnumberofbytestransferred : *mut u32, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetOverlappedResultEx(hfile : super::winnt::HANDLE, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpnumberofbytestransferred : *mut u32, dwmilliseconds : u32, balertable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetQueuedCompletionStatus(completionport : super::winnt::HANDLE, lpnumberofbytestransferred : *mut u32, lpcompletionkey : *mut u32, lpoverlapped : *mut super::minwinbase::LPOVERLAPPED, dwmilliseconds : u32) -> windows_sys::core::BOOL);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetQueuedCompletionStatus(completionport : super::winnt::HANDLE, lpnumberofbytestransferred : *mut u32, lpcompletionkey : *mut u64, lpoverlapped : *mut super::minwinbase::LPOVERLAPPED, dwmilliseconds : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetQueuedCompletionStatusEx(completionport : super::winnt::HANDLE, lpcompletionportentries : *mut super::minwinbase::OVERLAPPED_ENTRY, ulcount : u32, ulnumentriesremoved : *mut u32, dwmilliseconds : u32, falertable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn PostQueuedCompletionStatus(completionport : super::winnt::HANDLE, dwnumberofbytestransferred : u32, dwcompletionkey : usize, lpoverlapped : *const super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
