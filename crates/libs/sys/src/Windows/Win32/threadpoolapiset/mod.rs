#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CallbackMayRunLong(pci : *mut super::winnt::TP_CALLBACK_INSTANCE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CancelThreadpoolIo(pio : *mut super::winnt::TP_IO));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpool(ptpp : *mut super::winnt::TP_POOL));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroup(ptpcg : *mut super::winnt::TP_CLEANUP_GROUP));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroupMembers(ptpcg : *mut super::winnt::TP_CLEANUP_GROUP, fcancelpendingcallbacks : windows_sys::core::BOOL, pvcleanupcontext : *mut core::ffi::c_void));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolIo(pio : *mut super::winnt::TP_IO));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolTimer(pti : *mut super::winnt::TP_TIMER));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolWait(pwa : *mut super::winnt::TP_WAIT));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolWork(pwk : *mut super::winnt::TP_WORK));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpool(reserved : *const core::ffi::c_void) -> super::winnt::PTP_POOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolCleanupGroup() -> super::winnt::PTP_CLEANUP_GROUP);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolIo(fl : super::winnt::HANDLE, pfnio : PTP_WIN32_IO_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const super::winnt::TP_CALLBACK_ENVIRON_V3) -> super::winnt::PTP_IO);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolTimer(pfnti : super::winnt::PTP_TIMER_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const super::winnt::TP_CALLBACK_ENVIRON_V3) -> super::winnt::PTP_TIMER);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolWait(pfnwa : super::winnt::PTP_WAIT_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const super::winnt::TP_CALLBACK_ENVIRON_V3) -> super::winnt::PTP_WAIT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolWork(pfnwk : super::winnt::PTP_WORK_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const super::winnt::TP_CALLBACK_ENVIRON_V3) -> super::winnt::PTP_WORK);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn DisassociateCurrentThreadFromCallback(pci : *mut super::winnt::TP_CALLBACK_INSTANCE));
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn FreeLibraryWhenCallbackReturns(pci : *mut super::winnt::TP_CALLBACK_INSTANCE, r#mod : super::minwindef::HMODULE));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn IsThreadpoolTimerSet(pti : *mut super::winnt::TP_TIMER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn LeaveCriticalSectionWhenCallbackReturns(pci : *mut super::winnt::TP_CALLBACK_INSTANCE, pcs : super::minwinbase::PCRITICAL_SECTION));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryThreadpoolStackInformation(ptpp : *const super::winnt::TP_POOL, ptpsi : *mut super::winnt::TP_POOL_STACK_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseMutexWhenCallbackReturns(pci : *mut super::winnt::TP_CALLBACK_INSTANCE, r#mut : super::winnt::HANDLE));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseSemaphoreWhenCallbackReturns(pci : *mut super::winnt::TP_CALLBACK_INSTANCE, sem : super::winnt::HANDLE, crel : u32));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn SetEventWhenCallbackReturns(pci : *mut super::winnt::TP_CALLBACK_INSTANCE, evt : super::winnt::HANDLE));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolStackInformation(ptpp : *mut super::winnt::TP_POOL, ptpsi : *const super::winnt::TP_POOL_STACK_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolThreadMaximum(ptpp : *mut super::winnt::TP_POOL, cthrdmost : u32));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolThreadMinimum(ptpp : *mut super::winnt::TP_POOL, cthrdmic : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolTimer(pti : *mut super::winnt::TP_TIMER, pftduetime : *const super::minwindef::FILETIME, msperiod : u32, mswindowlength : u32));
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolTimerEx(pti : *mut super::winnt::TP_TIMER, pftduetime : *const super::minwindef::FILETIME, msperiod : u32, mswindowlength : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolWait(pwa : *mut super::winnt::TP_WAIT, h : super::winnt::HANDLE, pfttimeout : *const super::minwindef::FILETIME));
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolWaitEx(pwa : *mut super::winnt::TP_WAIT, h : super::winnt::HANDLE, pfttimeout : *const super::minwindef::FILETIME, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn StartThreadpoolIo(pio : *mut super::winnt::TP_IO));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn SubmitThreadpoolWork(pwk : *mut super::winnt::TP_WORK));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn TrySubmitThreadpoolCallback(pfns : super::winnt::PTP_SIMPLE_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const super::winnt::TP_CALLBACK_ENVIRON_V3) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolIoCallbacks(pio : *mut super::winnt::TP_IO, fcancelpendingcallbacks : windows_sys::core::BOOL));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolTimerCallbacks(pti : *mut super::winnt::TP_TIMER, fcancelpendingcallbacks : windows_sys::core::BOOL));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolWaitCallbacks(pwa : *mut super::winnt::TP_WAIT, fcancelpendingcallbacks : windows_sys::core::BOOL));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolWorkCallbacks(pwk : *mut super::winnt::TP_WORK, fcancelpendingcallbacks : windows_sys::core::BOOL));
#[cfg(feature = "Win32_winnt")]
pub type PTP_WIN32_IO_CALLBACK = Option<unsafe extern "system" fn(instance: *mut super::winnt::TP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, overlapped: *mut core::ffi::c_void, ioresult: u32, numberofbytestransferred: usize, io: *mut super::winnt::TP_IO)>;
