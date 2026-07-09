#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CallbackMayRunLong(pci: *mut super::winnt::TP_CALLBACK_INSTANCE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CallbackMayRunLong(pci : *mut super::winnt::TP_CALLBACK_INSTANCE) -> windows_core::BOOL);
    unsafe { CallbackMayRunLong(pci as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CancelThreadpoolIo(pio: *mut super::winnt::TP_IO) {
    windows_core::link!("kernel32.dll" "system" fn CancelThreadpoolIo(pio : *mut super::winnt::TP_IO));
    unsafe { CancelThreadpoolIo(pio as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CloseThreadpool(ptpp: *mut super::winnt::TP_POOL) {
    windows_core::link!("kernel32.dll" "system" fn CloseThreadpool(ptpp : *mut super::winnt::TP_POOL));
    unsafe { CloseThreadpool(ptpp as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CloseThreadpoolCleanupGroup(ptpcg: *mut super::winnt::TP_CLEANUP_GROUP) {
    windows_core::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroup(ptpcg : *mut super::winnt::TP_CLEANUP_GROUP));
    unsafe { CloseThreadpoolCleanupGroup(ptpcg as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CloseThreadpoolCleanupGroupMembers(ptpcg: *mut super::winnt::TP_CLEANUP_GROUP, fcancelpendingcallbacks: bool, pvcleanupcontext: Option<*mut core::ffi::c_void>) {
    windows_core::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroupMembers(ptpcg : *mut super::winnt::TP_CLEANUP_GROUP, fcancelpendingcallbacks : windows_core::BOOL, pvcleanupcontext : *mut core::ffi::c_void));
    unsafe { CloseThreadpoolCleanupGroupMembers(ptpcg as _, fcancelpendingcallbacks.into(), pvcleanupcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CloseThreadpoolIo(pio: *mut super::winnt::TP_IO) {
    windows_core::link!("kernel32.dll" "system" fn CloseThreadpoolIo(pio : *mut super::winnt::TP_IO));
    unsafe { CloseThreadpoolIo(pio as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CloseThreadpoolTimer(pti: *mut super::winnt::TP_TIMER) {
    windows_core::link!("kernel32.dll" "system" fn CloseThreadpoolTimer(pti : *mut super::winnt::TP_TIMER));
    unsafe { CloseThreadpoolTimer(pti as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CloseThreadpoolWait(pwa: *mut super::winnt::TP_WAIT) {
    windows_core::link!("kernel32.dll" "system" fn CloseThreadpoolWait(pwa : *mut super::winnt::TP_WAIT));
    unsafe { CloseThreadpoolWait(pwa as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CloseThreadpoolWork(pwk: *mut super::winnt::TP_WORK) {
    windows_core::link!("kernel32.dll" "system" fn CloseThreadpoolWork(pwk : *mut super::winnt::TP_WORK));
    unsafe { CloseThreadpoolWork(pwk as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CreateThreadpool(reserved: Option<*const core::ffi::c_void>) -> super::winnt::PTP_POOL {
    windows_core::link!("kernel32.dll" "system" fn CreateThreadpool(reserved : *const core::ffi::c_void) -> super::winnt::PTP_POOL);
    unsafe { CreateThreadpool(reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CreateThreadpoolCleanupGroup() -> super::winnt::PTP_CLEANUP_GROUP {
    windows_core::link!("kernel32.dll" "system" fn CreateThreadpoolCleanupGroup() -> super::winnt::PTP_CLEANUP_GROUP);
    unsafe { CreateThreadpoolCleanupGroup() }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CreateThreadpoolIo(fl: super::winnt::HANDLE, pfnio: PTP_WIN32_IO_CALLBACK, pv: Option<*mut core::ffi::c_void>, pcbe: Option<*const super::winnt::TP_CALLBACK_ENVIRON_V3>) -> super::winnt::PTP_IO {
    windows_core::link!("kernel32.dll" "system" fn CreateThreadpoolIo(fl : super::winnt::HANDLE, pfnio : PTP_WIN32_IO_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const super::winnt::TP_CALLBACK_ENVIRON_V3) -> super::winnt::PTP_IO);
    unsafe { CreateThreadpoolIo(fl, pfnio, pv.unwrap_or(core::mem::zeroed()) as _, pcbe.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CreateThreadpoolTimer(pfnti: super::winnt::PTP_TIMER_CALLBACK, pv: Option<*mut core::ffi::c_void>, pcbe: Option<*const super::winnt::TP_CALLBACK_ENVIRON_V3>) -> super::winnt::PTP_TIMER {
    windows_core::link!("kernel32.dll" "system" fn CreateThreadpoolTimer(pfnti : super::winnt::PTP_TIMER_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const super::winnt::TP_CALLBACK_ENVIRON_V3) -> super::winnt::PTP_TIMER);
    unsafe { CreateThreadpoolTimer(pfnti, pv.unwrap_or(core::mem::zeroed()) as _, pcbe.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CreateThreadpoolWait(pfnwa: super::winnt::PTP_WAIT_CALLBACK, pv: Option<*mut core::ffi::c_void>, pcbe: Option<*const super::winnt::TP_CALLBACK_ENVIRON_V3>) -> super::winnt::PTP_WAIT {
    windows_core::link!("kernel32.dll" "system" fn CreateThreadpoolWait(pfnwa : super::winnt::PTP_WAIT_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const super::winnt::TP_CALLBACK_ENVIRON_V3) -> super::winnt::PTP_WAIT);
    unsafe { CreateThreadpoolWait(pfnwa, pv.unwrap_or(core::mem::zeroed()) as _, pcbe.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CreateThreadpoolWork(pfnwk: super::winnt::PTP_WORK_CALLBACK, pv: Option<*mut core::ffi::c_void>, pcbe: Option<*const super::winnt::TP_CALLBACK_ENVIRON_V3>) -> super::winnt::PTP_WORK {
    windows_core::link!("kernel32.dll" "system" fn CreateThreadpoolWork(pfnwk : super::winnt::PTP_WORK_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const super::winnt::TP_CALLBACK_ENVIRON_V3) -> super::winnt::PTP_WORK);
    unsafe { CreateThreadpoolWork(pfnwk, pv.unwrap_or(core::mem::zeroed()) as _, pcbe.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DisassociateCurrentThreadFromCallback(pci: *mut super::winnt::TP_CALLBACK_INSTANCE) {
    windows_core::link!("kernel32.dll" "system" fn DisassociateCurrentThreadFromCallback(pci : *mut super::winnt::TP_CALLBACK_INSTANCE));
    unsafe { DisassociateCurrentThreadFromCallback(pci as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn FreeLibraryWhenCallbackReturns(pci: *mut super::winnt::TP_CALLBACK_INSTANCE, r#mod: super::minwindef::HMODULE) {
    windows_core::link!("kernel32.dll" "system" fn FreeLibraryWhenCallbackReturns(pci : *mut super::winnt::TP_CALLBACK_INSTANCE, r#mod : super::minwindef::HMODULE));
    unsafe { FreeLibraryWhenCallbackReturns(pci as _, r#mod) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn IsThreadpoolTimerSet(pti: *mut super::winnt::TP_TIMER) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsThreadpoolTimerSet(pti : *mut super::winnt::TP_TIMER) -> windows_core::BOOL);
    unsafe { IsThreadpoolTimerSet(pti as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn LeaveCriticalSectionWhenCallbackReturns(pci: *mut super::winnt::TP_CALLBACK_INSTANCE, pcs: super::minwinbase::PCRITICAL_SECTION) {
    windows_core::link!("kernel32.dll" "system" fn LeaveCriticalSectionWhenCallbackReturns(pci : *mut super::winnt::TP_CALLBACK_INSTANCE, pcs : super::minwinbase::PCRITICAL_SECTION));
    unsafe { LeaveCriticalSectionWhenCallbackReturns(pci as _, pcs as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn QueryThreadpoolStackInformation(ptpp: *const super::winnt::TP_POOL, ptpsi: *mut super::winnt::TP_POOL_STACK_INFORMATION) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryThreadpoolStackInformation(ptpp : *const super::winnt::TP_POOL, ptpsi : *mut super::winnt::TP_POOL_STACK_INFORMATION) -> windows_core::BOOL);
    unsafe { QueryThreadpoolStackInformation(ptpp, ptpsi as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn ReleaseMutexWhenCallbackReturns(pci: *mut super::winnt::TP_CALLBACK_INSTANCE, r#mut: super::winnt::HANDLE) {
    windows_core::link!("kernel32.dll" "system" fn ReleaseMutexWhenCallbackReturns(pci : *mut super::winnt::TP_CALLBACK_INSTANCE, r#mut : super::winnt::HANDLE));
    unsafe { ReleaseMutexWhenCallbackReturns(pci as _, r#mut) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn ReleaseSemaphoreWhenCallbackReturns(pci: *mut super::winnt::TP_CALLBACK_INSTANCE, sem: super::winnt::HANDLE, crel: u32) {
    windows_core::link!("kernel32.dll" "system" fn ReleaseSemaphoreWhenCallbackReturns(pci : *mut super::winnt::TP_CALLBACK_INSTANCE, sem : super::winnt::HANDLE, crel : u32));
    unsafe { ReleaseSemaphoreWhenCallbackReturns(pci as _, sem, crel) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetEventWhenCallbackReturns(pci: *mut super::winnt::TP_CALLBACK_INSTANCE, evt: super::winnt::HANDLE) {
    windows_core::link!("kernel32.dll" "system" fn SetEventWhenCallbackReturns(pci : *mut super::winnt::TP_CALLBACK_INSTANCE, evt : super::winnt::HANDLE));
    unsafe { SetEventWhenCallbackReturns(pci as _, evt) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetThreadpoolStackInformation(ptpp: *mut super::winnt::TP_POOL, ptpsi: *const super::winnt::TP_POOL_STACK_INFORMATION) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadpoolStackInformation(ptpp : *mut super::winnt::TP_POOL, ptpsi : *const super::winnt::TP_POOL_STACK_INFORMATION) -> windows_core::BOOL);
    unsafe { SetThreadpoolStackInformation(ptpp as _, ptpsi) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetThreadpoolThreadMaximum(ptpp: *mut super::winnt::TP_POOL, cthrdmost: u32) {
    windows_core::link!("kernel32.dll" "system" fn SetThreadpoolThreadMaximum(ptpp : *mut super::winnt::TP_POOL, cthrdmost : u32));
    unsafe { SetThreadpoolThreadMaximum(ptpp as _, cthrdmost) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetThreadpoolThreadMinimum(ptpp: *mut super::winnt::TP_POOL, cthrdmic: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadpoolThreadMinimum(ptpp : *mut super::winnt::TP_POOL, cthrdmic : u32) -> windows_core::BOOL);
    unsafe { SetThreadpoolThreadMinimum(ptpp as _, cthrdmic) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn SetThreadpoolTimer(pti: *mut super::winnt::TP_TIMER, pftduetime: Option<*const super::minwindef::FILETIME>, msperiod: u32, mswindowlength: Option<u32>) {
    windows_core::link!("kernel32.dll" "system" fn SetThreadpoolTimer(pti : *mut super::winnt::TP_TIMER, pftduetime : *const super::minwindef::FILETIME, msperiod : u32, mswindowlength : u32));
    unsafe { SetThreadpoolTimer(pti as _, pftduetime.unwrap_or(core::mem::zeroed()) as _, msperiod, mswindowlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn SetThreadpoolTimerEx(pti: *mut super::winnt::TP_TIMER, pftduetime: Option<*const super::minwindef::FILETIME>, msperiod: u32, mswindowlength: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadpoolTimerEx(pti : *mut super::winnt::TP_TIMER, pftduetime : *const super::minwindef::FILETIME, msperiod : u32, mswindowlength : u32) -> windows_core::BOOL);
    unsafe { SetThreadpoolTimerEx(pti as _, pftduetime.unwrap_or(core::mem::zeroed()) as _, msperiod, mswindowlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn SetThreadpoolWait(pwa: *mut super::winnt::TP_WAIT, h: Option<super::winnt::HANDLE>, pfttimeout: Option<*const super::minwindef::FILETIME>) {
    windows_core::link!("kernel32.dll" "system" fn SetThreadpoolWait(pwa : *mut super::winnt::TP_WAIT, h : super::winnt::HANDLE, pfttimeout : *const super::minwindef::FILETIME));
    unsafe { SetThreadpoolWait(pwa as _, h.unwrap_or(core::mem::zeroed()) as _, pfttimeout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn SetThreadpoolWaitEx(pwa: *mut super::winnt::TP_WAIT, h: Option<super::winnt::HANDLE>, pfttimeout: Option<*const super::minwindef::FILETIME>, reserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadpoolWaitEx(pwa : *mut super::winnt::TP_WAIT, h : super::winnt::HANDLE, pfttimeout : *const super::minwindef::FILETIME, reserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetThreadpoolWaitEx(pwa as _, h.unwrap_or(core::mem::zeroed()) as _, pfttimeout.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn StartThreadpoolIo(pio: *mut super::winnt::TP_IO) {
    windows_core::link!("kernel32.dll" "system" fn StartThreadpoolIo(pio : *mut super::winnt::TP_IO));
    unsafe { StartThreadpoolIo(pio as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SubmitThreadpoolWork(pwk: *mut super::winnt::TP_WORK) {
    windows_core::link!("kernel32.dll" "system" fn SubmitThreadpoolWork(pwk : *mut super::winnt::TP_WORK));
    unsafe { SubmitThreadpoolWork(pwk as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn TrySubmitThreadpoolCallback(pfns: super::winnt::PTP_SIMPLE_CALLBACK, pv: Option<*mut core::ffi::c_void>, pcbe: Option<*const super::winnt::TP_CALLBACK_ENVIRON_V3>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TrySubmitThreadpoolCallback(pfns : super::winnt::PTP_SIMPLE_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const super::winnt::TP_CALLBACK_ENVIRON_V3) -> windows_core::BOOL);
    unsafe { TrySubmitThreadpoolCallback(pfns, pv.unwrap_or(core::mem::zeroed()) as _, pcbe.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn WaitForThreadpoolIoCallbacks(pio: *mut super::winnt::TP_IO, fcancelpendingcallbacks: bool) {
    windows_core::link!("kernel32.dll" "system" fn WaitForThreadpoolIoCallbacks(pio : *mut super::winnt::TP_IO, fcancelpendingcallbacks : windows_core::BOOL));
    unsafe { WaitForThreadpoolIoCallbacks(pio as _, fcancelpendingcallbacks.into()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn WaitForThreadpoolTimerCallbacks(pti: *mut super::winnt::TP_TIMER, fcancelpendingcallbacks: bool) {
    windows_core::link!("kernel32.dll" "system" fn WaitForThreadpoolTimerCallbacks(pti : *mut super::winnt::TP_TIMER, fcancelpendingcallbacks : windows_core::BOOL));
    unsafe { WaitForThreadpoolTimerCallbacks(pti as _, fcancelpendingcallbacks.into()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn WaitForThreadpoolWaitCallbacks(pwa: *mut super::winnt::TP_WAIT, fcancelpendingcallbacks: bool) {
    windows_core::link!("kernel32.dll" "system" fn WaitForThreadpoolWaitCallbacks(pwa : *mut super::winnt::TP_WAIT, fcancelpendingcallbacks : windows_core::BOOL));
    unsafe { WaitForThreadpoolWaitCallbacks(pwa as _, fcancelpendingcallbacks.into()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn WaitForThreadpoolWorkCallbacks(pwk: *mut super::winnt::TP_WORK, fcancelpendingcallbacks: bool) {
    windows_core::link!("kernel32.dll" "system" fn WaitForThreadpoolWorkCallbacks(pwk : *mut super::winnt::TP_WORK, fcancelpendingcallbacks : windows_core::BOOL));
    unsafe { WaitForThreadpoolWorkCallbacks(pwk as _, fcancelpendingcallbacks.into()) }
}
#[cfg(feature = "Win32_winnt")]
pub type PTP_WIN32_IO_CALLBACK = Option<unsafe extern "system" fn(instance: *mut super::winnt::TP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, overlapped: *mut core::ffi::c_void, ioresult: u32, numberofbytestransferred: usize, io: *mut super::winnt::TP_IO)>;
