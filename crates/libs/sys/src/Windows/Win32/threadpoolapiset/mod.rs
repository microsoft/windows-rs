#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CallbackMayRunLong(pci : super::PTP_CALLBACK_INSTANCE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CancelThreadpoolIo(pio : super::PTP_IO));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpool(ptpp : super::PTP_POOL));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroup(ptpcg : super::PTP_CLEANUP_GROUP));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroupMembers(ptpcg : super::PTP_CLEANUP_GROUP, fcancelpendingcallbacks : windows_sys::core::BOOL, pvcleanupcontext : super::PVOID));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolIo(pio : super::PTP_IO));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolTimer(pti : super::PTP_TIMER));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolWait(pwa : super::PTP_WAIT));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolWork(pwk : super::PTP_WORK));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpool(reserved : super::PVOID) -> super::PTP_POOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolCleanupGroup() -> super::PTP_CLEANUP_GROUP);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolIo(fl : super::HANDLE, pfnio : PTP_WIN32_IO_CALLBACK, pv : super::PVOID, pcbe : super::PTP_CALLBACK_ENVIRON) -> super::PTP_IO);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolTimer(pfnti : super::PTP_TIMER_CALLBACK, pv : super::PVOID, pcbe : super::PTP_CALLBACK_ENVIRON) -> super::PTP_TIMER);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolWait(pfnwa : super::PTP_WAIT_CALLBACK, pv : super::PVOID, pcbe : super::PTP_CALLBACK_ENVIRON) -> super::PTP_WAIT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateThreadpoolWork(pfnwk : super::PTP_WORK_CALLBACK, pv : super::PVOID, pcbe : super::PTP_CALLBACK_ENVIRON) -> super::PTP_WORK);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DisassociateCurrentThreadFromCallback(pci : super::PTP_CALLBACK_INSTANCE));
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FreeLibraryWhenCallbackReturns(pci : super::PTP_CALLBACK_INSTANCE, r#mod : super::HMODULE));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn IsThreadpoolTimerSet(pti : super::PTP_TIMER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LeaveCriticalSectionWhenCallbackReturns(pci : super::PTP_CALLBACK_INSTANCE, pcs : super::PCRITICAL_SECTION));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn QueryThreadpoolStackInformation(ptpp : super::PTP_POOL, ptpsi : super::PTP_POOL_STACK_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseMutexWhenCallbackReturns(pci : super::PTP_CALLBACK_INSTANCE, r#mut : super::HANDLE));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReleaseSemaphoreWhenCallbackReturns(pci : super::PTP_CALLBACK_INSTANCE, sem : super::HANDLE, crel : super::DWORD));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetEventWhenCallbackReturns(pci : super::PTP_CALLBACK_INSTANCE, evt : super::HANDLE));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolStackInformation(ptpp : super::PTP_POOL, ptpsi : super::PTP_POOL_STACK_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolThreadMaximum(ptpp : super::PTP_POOL, cthrdmost : super::DWORD));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolThreadMinimum(ptpp : super::PTP_POOL, cthrdmic : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolTimer(pti : super::PTP_TIMER, pftduetime : super::PFILETIME, msperiod : super::DWORD, mswindowlength : super::DWORD));
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolTimerEx(pti : super::PTP_TIMER, pftduetime : super::PFILETIME, msperiod : super::DWORD, mswindowlength : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolWait(pwa : super::PTP_WAIT, h : super::HANDLE, pfttimeout : super::PFILETIME));
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolWaitEx(pwa : super::PTP_WAIT, h : super::HANDLE, pfttimeout : super::PFILETIME, reserved : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn StartThreadpoolIo(pio : super::PTP_IO));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SubmitThreadpoolWork(pwk : super::PTP_WORK));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn TrySubmitThreadpoolCallback(pfns : super::PTP_SIMPLE_CALLBACK, pv : super::PVOID, pcbe : super::PTP_CALLBACK_ENVIRON) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolIoCallbacks(pio : super::PTP_IO, fcancelpendingcallbacks : windows_sys::core::BOOL));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolTimerCallbacks(pti : super::PTP_TIMER, fcancelpendingcallbacks : windows_sys::core::BOOL));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolWaitCallbacks(pwa : super::PTP_WAIT, fcancelpendingcallbacks : windows_sys::core::BOOL));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolWorkCallbacks(pwk : super::PTP_WORK, fcancelpendingcallbacks : windows_sys::core::BOOL));
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTP_WIN32_IO_CALLBACK = Option<unsafe extern "system" fn(instance: super::PTP_CALLBACK_INSTANCE, context: super::PVOID, overlapped: super::PVOID, ioresult: super::ULONG, numberofbytestransferred: super::ULONG_PTR, io: super::PTP_IO)>;
