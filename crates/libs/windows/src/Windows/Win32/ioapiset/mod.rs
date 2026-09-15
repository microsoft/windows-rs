#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CancelIo(hfile: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CancelIo(hfile : super::HANDLE) -> windows_core::BOOL);
    unsafe { CancelIo(hfile) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CancelIoEx(hfile: super::HANDLE, lpoverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CancelIoEx(hfile : super::HANDLE, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { CancelIoEx(hfile, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CancelSynchronousIo(hthread: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CancelSynchronousIo(hthread : super::HANDLE) -> windows_core::BOOL);
    unsafe { CancelSynchronousIo(hthread) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateIoCompletionPort(filehandle: super::HANDLE, existingcompletionport: Option<super::HANDLE>, completionkey: usize, numberofconcurrentthreads: u32) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateIoCompletionPort(filehandle : super::HANDLE, existingcompletionport : super::HANDLE, completionkey : usize, numberofconcurrentthreads : u32) -> super::HANDLE);
    unsafe { CreateIoCompletionPort(filehandle, existingcompletionport.unwrap_or(core::mem::zeroed()) as _, completionkey, numberofconcurrentthreads) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn DeviceIoControl(hdevice: super::HANDLE, dwiocontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<super::LPDWORD>, lpoverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DeviceIoControl(hdevice : super::HANDLE, dwiocontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { DeviceIoControl(hdevice, dwiocontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetOverlappedResult(hfile: super::HANDLE, lpoverlapped: super::LPOVERLAPPED, lpnumberofbytestransferred: super::LPDWORD, bwait: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetOverlappedResult(hfile : super::HANDLE, lpoverlapped : super::LPOVERLAPPED, lpnumberofbytestransferred : super::LPDWORD, bwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetOverlappedResult(hfile, lpoverlapped, lpnumberofbytestransferred as _, bwait.into()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetOverlappedResultEx(hfile: super::HANDLE, lpoverlapped: super::LPOVERLAPPED, lpnumberofbytestransferred: super::LPDWORD, dwmilliseconds: u32, balertable: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetOverlappedResultEx(hfile : super::HANDLE, lpoverlapped : super::LPOVERLAPPED, lpnumberofbytestransferred : super::LPDWORD, dwmilliseconds : u32, balertable : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetOverlappedResultEx(hfile, lpoverlapped, lpnumberofbytestransferred as _, dwmilliseconds, balertable.into()) }
}
#[cfg(all(feature = "basetsd", feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetQueuedCompletionStatus(completionport: super::HANDLE, lpnumberofbytestransferred: super::LPDWORD, lpcompletionkey: super::PULONG_PTR, lpoverlapped: *mut super::LPOVERLAPPED, dwmilliseconds: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetQueuedCompletionStatus(completionport : super::HANDLE, lpnumberofbytestransferred : super::LPDWORD, lpcompletionkey : super::PULONG_PTR, lpoverlapped : *mut super::LPOVERLAPPED, dwmilliseconds : u32) -> windows_core::BOOL);
    unsafe { GetQueuedCompletionStatus(completionport, lpnumberofbytestransferred as _, lpcompletionkey as _, lpoverlapped as _, dwmilliseconds) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetQueuedCompletionStatusEx(completionport: super::HANDLE, lpcompletionportentries: super::LPOVERLAPPED_ENTRY, ulcount: u32, ulnumentriesremoved: super::PULONG, dwmilliseconds: u32, falertable: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetQueuedCompletionStatusEx(completionport : super::HANDLE, lpcompletionportentries : super::LPOVERLAPPED_ENTRY, ulcount : u32, ulnumentriesremoved : super::PULONG, dwmilliseconds : u32, falertable : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetQueuedCompletionStatusEx(completionport, lpcompletionportentries as _, ulcount, ulnumentriesremoved as _, dwmilliseconds, falertable.into()) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn PostQueuedCompletionStatus(completionport: super::HANDLE, dwnumberofbytestransferred: u32, dwcompletionkey: usize, lpoverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn PostQueuedCompletionStatus(completionport : super::HANDLE, dwnumberofbytestransferred : u32, dwcompletionkey : usize, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { PostQueuedCompletionStatus(completionport, dwnumberofbytestransferred, dwcompletionkey, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
