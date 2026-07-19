#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CallbackMayRunLong(pci : *mut super::TP_CALLBACK_INSTANCE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CancelThreadpoolIo(pio : *mut super::TP_IO));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpool(ptpp : *mut super::TP_POOL));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroup(ptpcg : *mut super::TP_CLEANUP_GROUP));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroupMembers(ptpcg : *mut super::TP_CLEANUP_GROUP, fcancelpendingcallbacks : windows_sys::core::BOOL, pvcleanupcontext : *mut core::ffi::c_void));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolIo(pio : *mut super::TP_IO));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolTimer(pti : *mut super::TP_TIMER));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolWait(pwa : *mut super::TP_WAIT));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolWork(pwk : *mut super::TP_WORK));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpool(reserved : *const core::ffi::c_void) -> super::PTP_POOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolCleanupGroup() -> super::PTP_CLEANUP_GROUP);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolIo(fl : super::HANDLE, pfnio : PTP_WIN32_IO_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const super::TP_CALLBACK_ENVIRON_V3) -> super::PTP_IO);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolTimer(pfnti : super::PTP_TIMER_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const super::TP_CALLBACK_ENVIRON_V3) -> super::PTP_TIMER);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolWait(pfnwa : super::PTP_WAIT_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const super::TP_CALLBACK_ENVIRON_V3) -> super::PTP_WAIT);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolWork(pfnwk : super::PTP_WORK_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const super::TP_CALLBACK_ENVIRON_V3) -> super::PTP_WORK);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DisassociateCurrentThreadFromCallback(pci : *mut super::TP_CALLBACK_INSTANCE));
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FreeLibraryWhenCallbackReturns(pci : *mut super::TP_CALLBACK_INSTANCE, r#mod : super::HMODULE));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn IsThreadpoolTimerSet(pti : *mut super::TP_TIMER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LeaveCriticalSectionWhenCallbackReturns(pci : *mut super::TP_CALLBACK_INSTANCE, pcs : super::PCRITICAL_SECTION));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryThreadpoolStackInformation(ptpp : *const super::TP_POOL, ptpsi : *mut super::TP_POOL_STACK_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseMutexWhenCallbackReturns(pci : *mut super::TP_CALLBACK_INSTANCE, r#mut : super::HANDLE));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseSemaphoreWhenCallbackReturns(pci : *mut super::TP_CALLBACK_INSTANCE, sem : super::HANDLE, crel : u32));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetEventWhenCallbackReturns(pci : *mut super::TP_CALLBACK_INSTANCE, evt : super::HANDLE));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolStackInformation(ptpp : *mut super::TP_POOL, ptpsi : *const super::TP_POOL_STACK_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolThreadMaximum(ptpp : *mut super::TP_POOL, cthrdmost : u32));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolThreadMinimum(ptpp : *mut super::TP_POOL, cthrdmic : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolTimer(pti : *mut super::TP_TIMER, pftduetime : *const super::FILETIME, msperiod : u32, mswindowlength : u32));
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolTimerEx(pti : *mut super::TP_TIMER, pftduetime : *const super::FILETIME, msperiod : u32, mswindowlength : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolWait(pwa : *mut super::TP_WAIT, h : super::HANDLE, pfttimeout : *const super::FILETIME));
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolWaitEx(pwa : *mut super::TP_WAIT, h : super::HANDLE, pfttimeout : *const super::FILETIME, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn StartThreadpoolIo(pio : *mut super::TP_IO));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SubmitThreadpoolWork(pwk : *mut super::TP_WORK));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn TrySubmitThreadpoolCallback(pfns : super::PTP_SIMPLE_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const super::TP_CALLBACK_ENVIRON_V3) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolIoCallbacks(pio : *mut super::TP_IO, fcancelpendingcallbacks : windows_sys::core::BOOL));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolTimerCallbacks(pti : *mut super::TP_TIMER, fcancelpendingcallbacks : windows_sys::core::BOOL));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolWaitCallbacks(pwa : *mut super::TP_WAIT, fcancelpendingcallbacks : windows_sys::core::BOOL));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolWorkCallbacks(pwk : *mut super::TP_WORK, fcancelpendingcallbacks : windows_sys::core::BOOL));
#[cfg(feature = "winnt")]
pub type PTP_WIN32_IO_CALLBACK = Option<unsafe extern "system" fn(instance: *mut super::TP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, overlapped: *mut core::ffi::c_void, ioresult: u32, numberofbytestransferred: usize, io: *mut super::TP_IO)>;
