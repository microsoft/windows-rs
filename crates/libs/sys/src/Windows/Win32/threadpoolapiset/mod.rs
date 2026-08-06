windows_link::link!("kernel32.dll" "system" fn CallbackMayRunLong(pci : *mut TP_CALLBACK_INSTANCE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn CancelThreadpoolIo(pio : *mut TP_IO));
windows_link::link!("kernel32.dll" "system" fn CloseThreadpool(ptpp : *mut TP_POOL));
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroup(ptpcg : *mut TP_CLEANUP_GROUP));
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroupMembers(ptpcg : *mut TP_CLEANUP_GROUP, fcancelpendingcallbacks : windows_sys::core::BOOL, pvcleanupcontext : *mut core::ffi::c_void));
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolIo(pio : *mut TP_IO));
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolTimer(pti : *mut TP_TIMER));
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolWait(pwa : *mut TP_WAIT));
windows_link::link!("kernel32.dll" "system" fn CloseThreadpoolWork(pwk : *mut TP_WORK));
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
windows_link::link!("kernel32.dll" "system" fn DisassociateCurrentThreadFromCallback(pci : *mut TP_CALLBACK_INSTANCE));
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn FreeLibraryWhenCallbackReturns(pci : *mut TP_CALLBACK_INSTANCE, r#mod : super::HMODULE));
windows_link::link!("kernel32.dll" "system" fn IsThreadpoolTimerSet(pti : *mut TP_TIMER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LeaveCriticalSectionWhenCallbackReturns(pci : *mut TP_CALLBACK_INSTANCE, pcs : super::PCRITICAL_SECTION));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryThreadpoolStackInformation(ptpp : *const TP_POOL, ptpsi : *mut super::TP_POOL_STACK_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseMutexWhenCallbackReturns(pci : *mut TP_CALLBACK_INSTANCE, r#mut : super::HANDLE));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseSemaphoreWhenCallbackReturns(pci : *mut TP_CALLBACK_INSTANCE, sem : super::HANDLE, crel : u32));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetEventWhenCallbackReturns(pci : *mut TP_CALLBACK_INSTANCE, evt : super::HANDLE));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolStackInformation(ptpp : *mut TP_POOL, ptpsi : *const super::TP_POOL_STACK_INFORMATION) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolThreadMaximum(ptpp : *mut TP_POOL, cthrdmost : u32));
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolThreadMinimum(ptpp : *mut TP_POOL, cthrdmic : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolTimer(pti : *mut TP_TIMER, pftduetime : *const super::FILETIME, msperiod : u32, mswindowlength : u32));
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolTimerEx(pti : *mut TP_TIMER, pftduetime : *const super::FILETIME, msperiod : u32, mswindowlength : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolWait(pwa : *mut TP_WAIT, h : super::HANDLE, pfttimeout : *const super::FILETIME));
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadpoolWaitEx(pwa : *mut TP_WAIT, h : super::HANDLE, pfttimeout : *const super::FILETIME, reserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn StartThreadpoolIo(pio : *mut TP_IO));
windows_link::link!("kernel32.dll" "system" fn SubmitThreadpoolWork(pwk : *mut TP_WORK));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn TrySubmitThreadpoolCallback(pfns : super::PTP_SIMPLE_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const super::TP_CALLBACK_ENVIRON_V3) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolIoCallbacks(pio : *mut TP_IO, fcancelpendingcallbacks : windows_sys::core::BOOL));
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolTimerCallbacks(pti : *mut TP_TIMER, fcancelpendingcallbacks : windows_sys::core::BOOL));
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolWaitCallbacks(pwa : *mut TP_WAIT, fcancelpendingcallbacks : windows_sys::core::BOOL));
windows_link::link!("kernel32.dll" "system" fn WaitForThreadpoolWorkCallbacks(pwk : *mut TP_WORK, fcancelpendingcallbacks : windows_sys::core::BOOL));
pub type PTP_WIN32_IO_CALLBACK = Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, overlapped: *mut core::ffi::c_void, ioresult: u32, numberofbytestransferred: usize, io: *mut TP_IO)>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TP_CALLBACK_INSTANCE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TP_CLEANUP_GROUP(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TP_IO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TP_POOL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TP_TIMER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TP_WAIT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TP_WORK(pub u8);
