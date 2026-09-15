#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CallbackMayRunLong(pci: super::PTP_CALLBACK_INSTANCE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CallbackMayRunLong(pci : super::PTP_CALLBACK_INSTANCE) -> windows_core::BOOL);
    unsafe { CallbackMayRunLong(pci as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CancelThreadpoolIo(pio: super::PTP_IO) {
    windows_core::link!("kernel32.dll" "system" fn CancelThreadpoolIo(pio : super::PTP_IO));
    unsafe { CancelThreadpoolIo(pio as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CloseThreadpool(ptpp: super::PTP_POOL) {
    windows_core::link!("kernel32.dll" "system" fn CloseThreadpool(ptpp : super::PTP_POOL));
    unsafe { CloseThreadpool(ptpp as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CloseThreadpoolCleanupGroup(ptpcg: super::PTP_CLEANUP_GROUP) {
    windows_core::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroup(ptpcg : super::PTP_CLEANUP_GROUP));
    unsafe { CloseThreadpoolCleanupGroup(ptpcg as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CloseThreadpoolCleanupGroupMembers(ptpcg: super::PTP_CLEANUP_GROUP, fcancelpendingcallbacks: bool, pvcleanupcontext: Option<*mut core::ffi::c_void>) {
    windows_core::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroupMembers(ptpcg : super::PTP_CLEANUP_GROUP, fcancelpendingcallbacks : windows_core::BOOL, pvcleanupcontext : *mut core::ffi::c_void));
    unsafe { CloseThreadpoolCleanupGroupMembers(ptpcg as _, fcancelpendingcallbacks.into(), pvcleanupcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CloseThreadpoolIo(pio: super::PTP_IO) {
    windows_core::link!("kernel32.dll" "system" fn CloseThreadpoolIo(pio : super::PTP_IO));
    unsafe { CloseThreadpoolIo(pio as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CloseThreadpoolTimer(pti: super::PTP_TIMER) {
    windows_core::link!("kernel32.dll" "system" fn CloseThreadpoolTimer(pti : super::PTP_TIMER));
    unsafe { CloseThreadpoolTimer(pti as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CloseThreadpoolWait(pwa: super::PTP_WAIT) {
    windows_core::link!("kernel32.dll" "system" fn CloseThreadpoolWait(pwa : super::PTP_WAIT));
    unsafe { CloseThreadpoolWait(pwa as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CloseThreadpoolWork(pwk: super::PTP_WORK) {
    windows_core::link!("kernel32.dll" "system" fn CloseThreadpoolWork(pwk : super::PTP_WORK));
    unsafe { CloseThreadpoolWork(pwk as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateThreadpool(reserved: Option<*mut core::ffi::c_void>) -> super::PTP_POOL {
    windows_core::link!("kernel32.dll" "system" fn CreateThreadpool(reserved : *mut core::ffi::c_void) -> super::PTP_POOL);
    unsafe { CreateThreadpool(reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateThreadpoolCleanupGroup() -> super::PTP_CLEANUP_GROUP {
    windows_core::link!("kernel32.dll" "system" fn CreateThreadpoolCleanupGroup() -> super::PTP_CLEANUP_GROUP);
    unsafe { CreateThreadpoolCleanupGroup() }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateThreadpoolIo(fl: super::HANDLE, pfnio: PTP_WIN32_IO_CALLBACK, pv: Option<*mut core::ffi::c_void>, pcbe: Option<super::PTP_CALLBACK_ENVIRON>) -> super::PTP_IO {
    windows_core::link!("kernel32.dll" "system" fn CreateThreadpoolIo(fl : super::HANDLE, pfnio : PTP_WIN32_IO_CALLBACK, pv : *mut core::ffi::c_void, pcbe : super::PTP_CALLBACK_ENVIRON) -> super::PTP_IO);
    unsafe { CreateThreadpoolIo(fl, pfnio, pv.unwrap_or(core::mem::zeroed()) as _, pcbe.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateThreadpoolTimer(pfnti: super::PTP_TIMER_CALLBACK, pv: Option<*mut core::ffi::c_void>, pcbe: Option<super::PTP_CALLBACK_ENVIRON>) -> super::PTP_TIMER {
    windows_core::link!("kernel32.dll" "system" fn CreateThreadpoolTimer(pfnti : super::PTP_TIMER_CALLBACK, pv : *mut core::ffi::c_void, pcbe : super::PTP_CALLBACK_ENVIRON) -> super::PTP_TIMER);
    unsafe { CreateThreadpoolTimer(pfnti, pv.unwrap_or(core::mem::zeroed()) as _, pcbe.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateThreadpoolWait(pfnwa: super::PTP_WAIT_CALLBACK, pv: Option<*mut core::ffi::c_void>, pcbe: Option<super::PTP_CALLBACK_ENVIRON>) -> super::PTP_WAIT {
    windows_core::link!("kernel32.dll" "system" fn CreateThreadpoolWait(pfnwa : super::PTP_WAIT_CALLBACK, pv : *mut core::ffi::c_void, pcbe : super::PTP_CALLBACK_ENVIRON) -> super::PTP_WAIT);
    unsafe { CreateThreadpoolWait(pfnwa, pv.unwrap_or(core::mem::zeroed()) as _, pcbe.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateThreadpoolWork(pfnwk: super::PTP_WORK_CALLBACK, pv: Option<*mut core::ffi::c_void>, pcbe: Option<super::PTP_CALLBACK_ENVIRON>) -> super::PTP_WORK {
    windows_core::link!("kernel32.dll" "system" fn CreateThreadpoolWork(pfnwk : super::PTP_WORK_CALLBACK, pv : *mut core::ffi::c_void, pcbe : super::PTP_CALLBACK_ENVIRON) -> super::PTP_WORK);
    unsafe { CreateThreadpoolWork(pfnwk, pv.unwrap_or(core::mem::zeroed()) as _, pcbe.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DisassociateCurrentThreadFromCallback(pci: super::PTP_CALLBACK_INSTANCE) {
    windows_core::link!("kernel32.dll" "system" fn DisassociateCurrentThreadFromCallback(pci : super::PTP_CALLBACK_INSTANCE));
    unsafe { DisassociateCurrentThreadFromCallback(pci as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn FreeLibraryWhenCallbackReturns(pci: super::PTP_CALLBACK_INSTANCE, r#mod: super::HMODULE) {
    windows_core::link!("kernel32.dll" "system" fn FreeLibraryWhenCallbackReturns(pci : super::PTP_CALLBACK_INSTANCE, r#mod : super::HMODULE));
    unsafe { FreeLibraryWhenCallbackReturns(pci as _, r#mod) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IsThreadpoolTimerSet(pti: super::PTP_TIMER) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsThreadpoolTimerSet(pti : super::PTP_TIMER) -> windows_core::BOOL);
    unsafe { IsThreadpoolTimerSet(pti as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn LeaveCriticalSectionWhenCallbackReturns(pci: super::PTP_CALLBACK_INSTANCE, pcs: super::PCRITICAL_SECTION) {
    windows_core::link!("kernel32.dll" "system" fn LeaveCriticalSectionWhenCallbackReturns(pci : super::PTP_CALLBACK_INSTANCE, pcs : super::PCRITICAL_SECTION));
    unsafe { LeaveCriticalSectionWhenCallbackReturns(pci as _, pcs as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryThreadpoolStackInformation(ptpp: super::PTP_POOL, ptpsi: super::PTP_POOL_STACK_INFORMATION) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryThreadpoolStackInformation(ptpp : super::PTP_POOL, ptpsi : super::PTP_POOL_STACK_INFORMATION) -> windows_core::BOOL);
    unsafe { QueryThreadpoolStackInformation(ptpp, ptpsi as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReleaseMutexWhenCallbackReturns(pci: super::PTP_CALLBACK_INSTANCE, r#mut: super::HANDLE) {
    windows_core::link!("kernel32.dll" "system" fn ReleaseMutexWhenCallbackReturns(pci : super::PTP_CALLBACK_INSTANCE, r#mut : super::HANDLE));
    unsafe { ReleaseMutexWhenCallbackReturns(pci as _, r#mut) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReleaseSemaphoreWhenCallbackReturns(pci: super::PTP_CALLBACK_INSTANCE, sem: super::HANDLE, crel: u32) {
    windows_core::link!("kernel32.dll" "system" fn ReleaseSemaphoreWhenCallbackReturns(pci : super::PTP_CALLBACK_INSTANCE, sem : super::HANDLE, crel : u32));
    unsafe { ReleaseSemaphoreWhenCallbackReturns(pci as _, sem, crel) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetEventWhenCallbackReturns(pci: super::PTP_CALLBACK_INSTANCE, evt: super::HANDLE) {
    windows_core::link!("kernel32.dll" "system" fn SetEventWhenCallbackReturns(pci : super::PTP_CALLBACK_INSTANCE, evt : super::HANDLE));
    unsafe { SetEventWhenCallbackReturns(pci as _, evt) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetThreadpoolStackInformation(ptpp: super::PTP_POOL, ptpsi: super::PTP_POOL_STACK_INFORMATION) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadpoolStackInformation(ptpp : super::PTP_POOL, ptpsi : super::PTP_POOL_STACK_INFORMATION) -> windows_core::BOOL);
    unsafe { SetThreadpoolStackInformation(ptpp as _, ptpsi) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetThreadpoolThreadMaximum(ptpp: super::PTP_POOL, cthrdmost: u32) {
    windows_core::link!("kernel32.dll" "system" fn SetThreadpoolThreadMaximum(ptpp : super::PTP_POOL, cthrdmost : u32));
    unsafe { SetThreadpoolThreadMaximum(ptpp as _, cthrdmost) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetThreadpoolThreadMinimum(ptpp: super::PTP_POOL, cthrdmic: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadpoolThreadMinimum(ptpp : super::PTP_POOL, cthrdmic : u32) -> windows_core::BOOL);
    unsafe { SetThreadpoolThreadMinimum(ptpp as _, cthrdmic) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetThreadpoolTimer(pti: super::PTP_TIMER, pftduetime: Option<super::PFILETIME>, msperiod: u32, mswindowlength: Option<u32>) {
    windows_core::link!("kernel32.dll" "system" fn SetThreadpoolTimer(pti : super::PTP_TIMER, pftduetime : super::PFILETIME, msperiod : u32, mswindowlength : u32));
    unsafe { SetThreadpoolTimer(pti as _, pftduetime.unwrap_or(core::mem::zeroed()) as _, msperiod, mswindowlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetThreadpoolTimerEx(pti: super::PTP_TIMER, pftduetime: Option<super::PFILETIME>, msperiod: u32, mswindowlength: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadpoolTimerEx(pti : super::PTP_TIMER, pftduetime : super::PFILETIME, msperiod : u32, mswindowlength : u32) -> windows_core::BOOL);
    unsafe { SetThreadpoolTimerEx(pti as _, pftduetime.unwrap_or(core::mem::zeroed()) as _, msperiod, mswindowlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetThreadpoolWait(pwa: super::PTP_WAIT, h: Option<super::HANDLE>, pfttimeout: Option<super::PFILETIME>) {
    windows_core::link!("kernel32.dll" "system" fn SetThreadpoolWait(pwa : super::PTP_WAIT, h : super::HANDLE, pfttimeout : super::PFILETIME));
    unsafe { SetThreadpoolWait(pwa as _, h.unwrap_or(core::mem::zeroed()) as _, pfttimeout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetThreadpoolWaitEx(pwa: super::PTP_WAIT, h: Option<super::HANDLE>, pfttimeout: Option<super::PFILETIME>, reserved: Option<*mut core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadpoolWaitEx(pwa : super::PTP_WAIT, h : super::HANDLE, pfttimeout : super::PFILETIME, reserved : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetThreadpoolWaitEx(pwa as _, h.unwrap_or(core::mem::zeroed()) as _, pfttimeout.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn StartThreadpoolIo(pio: super::PTP_IO) {
    windows_core::link!("kernel32.dll" "system" fn StartThreadpoolIo(pio : super::PTP_IO));
    unsafe { StartThreadpoolIo(pio as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SubmitThreadpoolWork(pwk: super::PTP_WORK) {
    windows_core::link!("kernel32.dll" "system" fn SubmitThreadpoolWork(pwk : super::PTP_WORK));
    unsafe { SubmitThreadpoolWork(pwk as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TrySubmitThreadpoolCallback(pfns: super::PTP_SIMPLE_CALLBACK, pv: Option<*mut core::ffi::c_void>, pcbe: Option<super::PTP_CALLBACK_ENVIRON>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TrySubmitThreadpoolCallback(pfns : super::PTP_SIMPLE_CALLBACK, pv : *mut core::ffi::c_void, pcbe : super::PTP_CALLBACK_ENVIRON) -> windows_core::BOOL);
    unsafe { TrySubmitThreadpoolCallback(pfns, pv.unwrap_or(core::mem::zeroed()) as _, pcbe.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WaitForThreadpoolIoCallbacks(pio: super::PTP_IO, fcancelpendingcallbacks: bool) {
    windows_core::link!("kernel32.dll" "system" fn WaitForThreadpoolIoCallbacks(pio : super::PTP_IO, fcancelpendingcallbacks : windows_core::BOOL));
    unsafe { WaitForThreadpoolIoCallbacks(pio as _, fcancelpendingcallbacks.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WaitForThreadpoolTimerCallbacks(pti: super::PTP_TIMER, fcancelpendingcallbacks: bool) {
    windows_core::link!("kernel32.dll" "system" fn WaitForThreadpoolTimerCallbacks(pti : super::PTP_TIMER, fcancelpendingcallbacks : windows_core::BOOL));
    unsafe { WaitForThreadpoolTimerCallbacks(pti as _, fcancelpendingcallbacks.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WaitForThreadpoolWaitCallbacks(pwa: super::PTP_WAIT, fcancelpendingcallbacks: bool) {
    windows_core::link!("kernel32.dll" "system" fn WaitForThreadpoolWaitCallbacks(pwa : super::PTP_WAIT, fcancelpendingcallbacks : windows_core::BOOL));
    unsafe { WaitForThreadpoolWaitCallbacks(pwa as _, fcancelpendingcallbacks.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WaitForThreadpoolWorkCallbacks(pwk: super::PTP_WORK, fcancelpendingcallbacks: bool) {
    windows_core::link!("kernel32.dll" "system" fn WaitForThreadpoolWorkCallbacks(pwk : super::PTP_WORK, fcancelpendingcallbacks : windows_core::BOOL));
    unsafe { WaitForThreadpoolWorkCallbacks(pwk as _, fcancelpendingcallbacks.into()) }
}
#[cfg(feature = "winnt")]
pub type PTP_WIN32_IO_CALLBACK = Option<unsafe extern "system" fn(instance: super::PTP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, overlapped: *mut core::ffi::c_void, ioresult: u32, numberofbytestransferred: usize, io: super::PTP_IO)>;
