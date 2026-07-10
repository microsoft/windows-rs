#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AcquireSRWLockExclusive(srwlock: *mut super::winnt::RTL_SRWLOCK) {
    windows_core::link!("kernel32.dll" "system" fn AcquireSRWLockExclusive(srwlock : *mut super::winnt::RTL_SRWLOCK));
    unsafe { AcquireSRWLockExclusive(srwlock as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AcquireSRWLockShared(srwlock: *mut super::winnt::RTL_SRWLOCK) {
    windows_core::link!("kernel32.dll" "system" fn AcquireSRWLockShared(srwlock : *mut super::winnt::RTL_SRWLOCK));
    unsafe { AcquireSRWLockShared(srwlock as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CancelWaitableTimer(htimer: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CancelWaitableTimer(htimer : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { CancelWaitableTimer(htimer) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateEventA<P3>(lpeventattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, bmanualreset: bool, binitialstate: bool, lpname: P3) -> super::winnt::HANDLE
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateEventA(lpeventattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, bmanualreset : windows_core::BOOL, binitialstate : windows_core::BOOL, lpname : windows_core::PCSTR) -> super::winnt::HANDLE);
    unsafe { CreateEventA(lpeventattributes.unwrap_or(core::mem::zeroed()) as _, bmanualreset.into(), binitialstate.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateEventExA<P1>(lpeventattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, lpname: P1, dwflags: u32, dwdesiredaccess: u32) -> super::winnt::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateEventExA(lpeventattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lpname : windows_core::PCSTR, dwflags : u32, dwdesiredaccess : u32) -> super::winnt::HANDLE);
    unsafe { CreateEventExA(lpeventattributes.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi(), dwflags, dwdesiredaccess) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateEventExW<P1>(lpeventattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, lpname: P1, dwflags: u32, dwdesiredaccess: u32) -> super::winnt::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateEventExW(lpeventattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lpname : windows_core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::winnt::HANDLE);
    unsafe { CreateEventExW(lpeventattributes.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi(), dwflags, dwdesiredaccess) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateEventW<P3>(lpeventattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, bmanualreset: bool, binitialstate: bool, lpname: P3) -> super::winnt::HANDLE
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, bmanualreset : windows_core::BOOL, binitialstate : windows_core::BOOL, lpname : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { CreateEventW(lpeventattributes.unwrap_or(core::mem::zeroed()) as _, bmanualreset.into(), binitialstate.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateMutexA<P2>(lpmutexattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, binitialowner: bool, lpname: P2) -> super::winnt::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateMutexA(lpmutexattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, binitialowner : windows_core::BOOL, lpname : windows_core::PCSTR) -> super::winnt::HANDLE);
    unsafe { CreateMutexA(lpmutexattributes.unwrap_or(core::mem::zeroed()) as _, binitialowner.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateMutexExA<P1>(lpmutexattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, lpname: P1, dwflags: u32, dwdesiredaccess: u32) -> super::winnt::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateMutexExA(lpmutexattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lpname : windows_core::PCSTR, dwflags : u32, dwdesiredaccess : u32) -> super::winnt::HANDLE);
    unsafe { CreateMutexExA(lpmutexattributes.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi(), dwflags, dwdesiredaccess) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateMutexExW<P1>(lpmutexattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, lpname: P1, dwflags: u32, dwdesiredaccess: u32) -> super::winnt::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateMutexExW(lpmutexattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lpname : windows_core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::winnt::HANDLE);
    unsafe { CreateMutexExW(lpmutexattributes.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi(), dwflags, dwdesiredaccess) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateMutexW<P2>(lpmutexattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, binitialowner: bool, lpname: P2) -> super::winnt::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateMutexW(lpmutexattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, binitialowner : windows_core::BOOL, lpname : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { CreateMutexW(lpmutexattributes.unwrap_or(core::mem::zeroed()) as _, binitialowner.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateSemaphoreExW<P3>(lpsemaphoreattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, linitialcount: i32, lmaximumcount: i32, lpname: P3, dwflags: Option<u32>, dwdesiredaccess: u32) -> super::winnt::HANDLE
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateSemaphoreExW(lpsemaphoreattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, linitialcount : i32, lmaximumcount : i32, lpname : windows_core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::winnt::HANDLE);
    unsafe { CreateSemaphoreExW(lpsemaphoreattributes.unwrap_or(core::mem::zeroed()) as _, linitialcount, lmaximumcount, lpname.param().abi(), dwflags.unwrap_or(core::mem::zeroed()) as _, dwdesiredaccess) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateSemaphoreW<P3>(lpsemaphoreattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, linitialcount: i32, lmaximumcount: i32, lpname: P3) -> super::winnt::HANDLE
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateSemaphoreW(lpsemaphoreattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, linitialcount : i32, lmaximumcount : i32, lpname : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { CreateSemaphoreW(lpsemaphoreattributes.unwrap_or(core::mem::zeroed()) as _, linitialcount, lmaximumcount, lpname.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateWaitableTimerExW<P1>(lptimerattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, lptimername: P1, dwflags: u32, dwdesiredaccess: u32) -> super::winnt::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateWaitableTimerExW(lptimerattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lptimername : windows_core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::winnt::HANDLE);
    unsafe { CreateWaitableTimerExW(lptimerattributes.unwrap_or(core::mem::zeroed()) as _, lptimername.param().abi(), dwflags, dwdesiredaccess) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateWaitableTimerW<P2>(lptimerattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, bmanualreset: bool, lptimername: P2) -> super::winnt::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateWaitableTimerW(lptimerattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, bmanualreset : windows_core::BOOL, lptimername : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { CreateWaitableTimerW(lptimerattributes.unwrap_or(core::mem::zeroed()) as _, bmanualreset.into(), lptimername.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn DeleteCriticalSection(lpcriticalsection: super::minwinbase::LPCRITICAL_SECTION) {
    windows_core::link!("kernel32.dll" "system" fn DeleteCriticalSection(lpcriticalsection : super::minwinbase::LPCRITICAL_SECTION));
    unsafe { DeleteCriticalSection(lpcriticalsection as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeleteSynchronizationBarrier(lpbarrier: LPSYNCHRONIZATION_BARRIER) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DeleteSynchronizationBarrier(lpbarrier : LPSYNCHRONIZATION_BARRIER) -> windows_core::BOOL);
    unsafe { DeleteSynchronizationBarrier(lpbarrier as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn EnterCriticalSection(lpcriticalsection: super::minwinbase::LPCRITICAL_SECTION) {
    windows_core::link!("kernel32.dll" "system" fn EnterCriticalSection(lpcriticalsection : super::minwinbase::LPCRITICAL_SECTION));
    unsafe { EnterCriticalSection(lpcriticalsection as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EnterSynchronizationBarrier(lpbarrier: LPSYNCHRONIZATION_BARRIER, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EnterSynchronizationBarrier(lpbarrier : LPSYNCHRONIZATION_BARRIER, dwflags : u32) -> windows_core::BOOL);
    unsafe { EnterSynchronizationBarrier(lpbarrier as _, dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitOnceBeginInitialize(lpinitonce: LPINIT_ONCE, dwflags: u32, fpending: *mut windows_core::BOOL, lpcontext: *mut *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitOnceBeginInitialize(lpinitonce : LPINIT_ONCE, dwflags : u32, fpending : *mut windows_core::BOOL, lpcontext : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { InitOnceBeginInitialize(lpinitonce as _, dwflags, fpending as _, lpcontext as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitOnceComplete(lpinitonce: LPINIT_ONCE, dwflags: u32, lpcontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitOnceComplete(lpinitonce : LPINIT_ONCE, dwflags : u32, lpcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { InitOnceComplete(lpinitonce as _, dwflags, lpcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitOnceExecuteOnce(initonce: PINIT_ONCE, initfn: PINIT_ONCE_FN, parameter: Option<*mut core::ffi::c_void>, context: *mut *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitOnceExecuteOnce(initonce : PINIT_ONCE, initfn : PINIT_ONCE_FN, parameter : *mut core::ffi::c_void, context : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { InitOnceExecuteOnce(initonce as _, initfn, parameter.unwrap_or(core::mem::zeroed()) as _, context as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitOnceInitialize(initonce: PINIT_ONCE) {
    windows_core::link!("kernel32.dll" "system" fn InitOnceInitialize(initonce : PINIT_ONCE));
    unsafe { InitOnceInitialize(initonce as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitializeConditionVariable() -> super::winnt::RTL_CONDITION_VARIABLE {
    windows_core::link!("kernel32.dll" "system" fn InitializeConditionVariable(conditionvariable : *mut super::winnt::RTL_CONDITION_VARIABLE));
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitializeConditionVariable(&mut result__);
        result__
    }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn InitializeCriticalSection(lpcriticalsection: super::minwinbase::LPCRITICAL_SECTION) {
    windows_core::link!("kernel32.dll" "system" fn InitializeCriticalSection(lpcriticalsection : super::minwinbase::LPCRITICAL_SECTION));
    unsafe { InitializeCriticalSection(lpcriticalsection as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn InitializeCriticalSectionAndSpinCount(lpcriticalsection: super::minwinbase::LPCRITICAL_SECTION, dwspincount: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitializeCriticalSectionAndSpinCount(lpcriticalsection : super::minwinbase::LPCRITICAL_SECTION, dwspincount : u32) -> windows_core::BOOL);
    unsafe { InitializeCriticalSectionAndSpinCount(lpcriticalsection as _, dwspincount) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn InitializeCriticalSectionEx(lpcriticalsection: super::minwinbase::LPCRITICAL_SECTION, dwspincount: u32, flags: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitializeCriticalSectionEx(lpcriticalsection : super::minwinbase::LPCRITICAL_SECTION, dwspincount : u32, flags : u32) -> windows_core::BOOL);
    unsafe { InitializeCriticalSectionEx(lpcriticalsection as _, dwspincount, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitializeSRWLock() -> super::winnt::RTL_SRWLOCK {
    windows_core::link!("kernel32.dll" "system" fn InitializeSRWLock(srwlock : *mut super::winnt::RTL_SRWLOCK));
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitializeSRWLock(&mut result__);
        result__
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitializeSynchronizationBarrier(lpbarrier: LPSYNCHRONIZATION_BARRIER, ltotalthreads: i32, lspincount: i32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitializeSynchronizationBarrier(lpbarrier : LPSYNCHRONIZATION_BARRIER, ltotalthreads : i32, lspincount : i32) -> windows_core::BOOL);
    unsafe { InitializeSynchronizationBarrier(lpbarrier as _, ltotalthreads, lspincount) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn LeaveCriticalSection(lpcriticalsection: super::minwinbase::LPCRITICAL_SECTION) {
    windows_core::link!("kernel32.dll" "system" fn LeaveCriticalSection(lpcriticalsection : super::minwinbase::LPCRITICAL_SECTION));
    unsafe { LeaveCriticalSection(lpcriticalsection as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn OpenEventA<P2>(dwdesiredaccess: u32, binherithandle: bool, lpname: P2) -> super::winnt::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenEventA(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, lpname : windows_core::PCSTR) -> super::winnt::HANDLE);
    unsafe { OpenEventA(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn OpenEventW<P2>(dwdesiredaccess: u32, binherithandle: bool, lpname: P2) -> super::winnt::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenEventW(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, lpname : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { OpenEventW(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn OpenMutexW<P2>(dwdesiredaccess: u32, binherithandle: bool, lpname: P2) -> super::winnt::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenMutexW(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, lpname : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { OpenMutexW(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn OpenSemaphoreW<P2>(dwdesiredaccess: u32, binherithandle: bool, lpname: P2) -> super::winnt::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenSemaphoreW(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, lpname : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { OpenSemaphoreW(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn OpenWaitableTimerW<P2>(dwdesiredaccess: u32, binherithandle: bool, lptimername: P2) -> super::winnt::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenWaitableTimerW(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, lptimername : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { OpenWaitableTimerW(dwdesiredaccess, binherithandle.into(), lptimername.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReleaseMutex(hmutex: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReleaseMutex(hmutex : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { ReleaseMutex(hmutex) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReleaseSRWLockExclusive(srwlock: *mut super::winnt::RTL_SRWLOCK) {
    windows_core::link!("kernel32.dll" "system" fn ReleaseSRWLockExclusive(srwlock : *mut super::winnt::RTL_SRWLOCK));
    unsafe { ReleaseSRWLockExclusive(srwlock as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReleaseSRWLockShared(srwlock: *mut super::winnt::RTL_SRWLOCK) {
    windows_core::link!("kernel32.dll" "system" fn ReleaseSRWLockShared(srwlock : *mut super::winnt::RTL_SRWLOCK));
    unsafe { ReleaseSRWLockShared(srwlock as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReleaseSemaphore(hsemaphore: super::winnt::HANDLE, lreleasecount: i32, lppreviouscount: Option<*mut i32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReleaseSemaphore(hsemaphore : super::winnt::HANDLE, lreleasecount : i32, lppreviouscount : *mut i32) -> windows_core::BOOL);
    unsafe { ReleaseSemaphore(hsemaphore, lreleasecount, lppreviouscount.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ResetEvent(hevent: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ResetEvent(hevent : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { ResetEvent(hevent) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn SetCriticalSectionSpinCount(lpcriticalsection: super::minwinbase::LPCRITICAL_SECTION, dwspincount: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn SetCriticalSectionSpinCount(lpcriticalsection : super::minwinbase::LPCRITICAL_SECTION, dwspincount : u32) -> u32);
    unsafe { SetCriticalSectionSpinCount(lpcriticalsection as _, dwspincount) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetEvent(hevent: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetEvent(hevent : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { SetEvent(hevent) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetWaitableTimer(htimer: super::winnt::HANDLE, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: Option<*const core::ffi::c_void>, fresume: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetWaitableTimer(htimer : super::winnt::HANDLE, lpduetime : *const i64, lperiod : i32, pfncompletionroutine : PTIMERAPCROUTINE, lpargtocompletionroutine : *const core::ffi::c_void, fresume : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetWaitableTimer(htimer, lpduetime, lperiod, pfncompletionroutine, lpargtocompletionroutine.unwrap_or(core::mem::zeroed()) as _, fresume.into()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetWaitableTimerEx(htimer: super::winnt::HANDLE, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: Option<*const core::ffi::c_void>, wakecontext: Option<*const super::minwinbase::REASON_CONTEXT>, tolerabledelay: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetWaitableTimerEx(htimer : super::winnt::HANDLE, lpduetime : *const i64, lperiod : i32, pfncompletionroutine : PTIMERAPCROUTINE, lpargtocompletionroutine : *const core::ffi::c_void, wakecontext : *const super::minwinbase::REASON_CONTEXT, tolerabledelay : u32) -> windows_core::BOOL);
    unsafe { SetWaitableTimerEx(htimer, lpduetime, lperiod, pfncompletionroutine, lpargtocompletionroutine.unwrap_or(core::mem::zeroed()) as _, wakecontext.unwrap_or(core::mem::zeroed()) as _, tolerabledelay) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SignalObjectAndWait(hobjecttosignal: super::winnt::HANDLE, hobjecttowaiton: super::winnt::HANDLE, dwmilliseconds: u32, balertable: bool) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn SignalObjectAndWait(hobjecttosignal : super::winnt::HANDLE, hobjecttowaiton : super::winnt::HANDLE, dwmilliseconds : u32, balertable : windows_core::BOOL) -> u32);
    unsafe { SignalObjectAndWait(hobjecttosignal, hobjecttowaiton, dwmilliseconds, balertable.into()) }
}
#[inline]
pub unsafe fn Sleep(dwmilliseconds: u32) {
    windows_core::link!("kernel32.dll" "system" fn Sleep(dwmilliseconds : u32));
    unsafe { Sleep(dwmilliseconds) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn SleepConditionVariableCS(conditionvariable: *mut super::winnt::RTL_CONDITION_VARIABLE, criticalsection: super::minwinbase::PCRITICAL_SECTION, dwmilliseconds: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SleepConditionVariableCS(conditionvariable : *mut super::winnt::RTL_CONDITION_VARIABLE, criticalsection : super::minwinbase::PCRITICAL_SECTION, dwmilliseconds : u32) -> windows_core::BOOL);
    unsafe { SleepConditionVariableCS(conditionvariable as _, criticalsection as _, dwmilliseconds) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SleepConditionVariableSRW(conditionvariable: *mut super::winnt::RTL_CONDITION_VARIABLE, srwlock: *mut super::winnt::RTL_SRWLOCK, dwmilliseconds: u32, flags: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SleepConditionVariableSRW(conditionvariable : *mut super::winnt::RTL_CONDITION_VARIABLE, srwlock : *mut super::winnt::RTL_SRWLOCK, dwmilliseconds : u32, flags : u32) -> windows_core::BOOL);
    unsafe { SleepConditionVariableSRW(conditionvariable as _, srwlock as _, dwmilliseconds, flags) }
}
#[inline]
pub unsafe fn SleepEx(dwmilliseconds: u32, balertable: bool) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn SleepEx(dwmilliseconds : u32, balertable : windows_core::BOOL) -> u32);
    unsafe { SleepEx(dwmilliseconds, balertable.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TryAcquireSRWLockExclusive(srwlock: *mut super::winnt::RTL_SRWLOCK) -> bool {
    windows_core::link!("kernel32.dll" "system" fn TryAcquireSRWLockExclusive(srwlock : *mut super::winnt::RTL_SRWLOCK) -> bool);
    unsafe { TryAcquireSRWLockExclusive(srwlock as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TryAcquireSRWLockShared(srwlock: *mut super::winnt::RTL_SRWLOCK) -> bool {
    windows_core::link!("kernel32.dll" "system" fn TryAcquireSRWLockShared(srwlock : *mut super::winnt::RTL_SRWLOCK) -> bool);
    unsafe { TryAcquireSRWLockShared(srwlock as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn TryEnterCriticalSection(lpcriticalsection: super::minwinbase::LPCRITICAL_SECTION) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TryEnterCriticalSection(lpcriticalsection : super::minwinbase::LPCRITICAL_SECTION) -> windows_core::BOOL);
    unsafe { TryEnterCriticalSection(lpcriticalsection as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WaitForMultipleObjects(lphandles: &[super::winnt::HANDLE], bwaitall: bool, dwmilliseconds: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn WaitForMultipleObjects(ncount : u32, lphandles : *const super::winnt::HANDLE, bwaitall : windows_core::BOOL, dwmilliseconds : u32) -> u32);
    unsafe { WaitForMultipleObjects(lphandles.len().try_into().unwrap(), core::mem::transmute(lphandles.as_ptr()), bwaitall.into(), dwmilliseconds) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WaitForMultipleObjectsEx(lphandles: &[super::winnt::HANDLE], bwaitall: bool, dwmilliseconds: u32, balertable: bool) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn WaitForMultipleObjectsEx(ncount : u32, lphandles : *const super::winnt::HANDLE, bwaitall : windows_core::BOOL, dwmilliseconds : u32, balertable : windows_core::BOOL) -> u32);
    unsafe { WaitForMultipleObjectsEx(lphandles.len().try_into().unwrap(), core::mem::transmute(lphandles.as_ptr()), bwaitall.into(), dwmilliseconds, balertable.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WaitForSingleObject(hhandle: super::winnt::HANDLE, dwmilliseconds: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn WaitForSingleObject(hhandle : super::winnt::HANDLE, dwmilliseconds : u32) -> u32);
    unsafe { WaitForSingleObject(hhandle, dwmilliseconds) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WaitForSingleObjectEx(hhandle: super::winnt::HANDLE, dwmilliseconds: u32, balertable: bool) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn WaitForSingleObjectEx(hhandle : super::winnt::HANDLE, dwmilliseconds : u32, balertable : windows_core::BOOL) -> u32);
    unsafe { WaitForSingleObjectEx(hhandle, dwmilliseconds, balertable.into()) }
}
#[inline]
pub unsafe fn WaitOnAddress(address: *const core::ffi::c_void, compareaddress: *const core::ffi::c_void, addresssize: usize, dwmilliseconds: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WaitOnAddress(address : *const core::ffi::c_void, compareaddress : *const core::ffi::c_void, addresssize : usize, dwmilliseconds : u32) -> windows_core::BOOL);
    unsafe { WaitOnAddress(address, compareaddress, addresssize, dwmilliseconds.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WakeAllConditionVariable(conditionvariable: *mut super::winnt::RTL_CONDITION_VARIABLE) {
    windows_core::link!("kernel32.dll" "system" fn WakeAllConditionVariable(conditionvariable : *mut super::winnt::RTL_CONDITION_VARIABLE));
    unsafe { WakeAllConditionVariable(conditionvariable as _) }
}
#[inline]
pub unsafe fn WakeByAddressAll(address: *const core::ffi::c_void) {
    windows_core::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WakeByAddressAll(address : *const core::ffi::c_void));
    unsafe { WakeByAddressAll(address) }
}
#[inline]
pub unsafe fn WakeByAddressSingle(address: *const core::ffi::c_void) {
    windows_core::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WakeByAddressSingle(address : *const core::ffi::c_void));
    unsafe { WakeByAddressSingle(address) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WakeConditionVariable(conditionvariable: *mut super::winnt::RTL_CONDITION_VARIABLE) {
    windows_core::link!("kernel32.dll" "system" fn WakeConditionVariable(conditionvariable : *mut super::winnt::RTL_CONDITION_VARIABLE));
    unsafe { WakeConditionVariable(conditionvariable as _) }
}
#[cfg(feature = "winnt")]
pub type CONDITION_VARIABLE = super::winnt::RTL_CONDITION_VARIABLE;
pub const CONDITION_VARIABLE_LOCKMODE_SHARED: u32 = 1;
pub const CREATE_EVENT_INITIAL_SET: u32 = 2;
pub const CREATE_EVENT_MANUAL_RESET: u32 = 1;
pub const CREATE_MUTEX_INITIAL_OWNER: u32 = 1;
pub const CREATE_WAITABLE_TIMER_HIGH_RESOLUTION: u32 = 2;
pub const CREATE_WAITABLE_TIMER_MANUAL_RESET: u32 = 1;
#[cfg(feature = "winnt")]
pub type INIT_ONCE = super::winnt::RTL_RUN_ONCE;
pub const INIT_ONCE_ASYNC: u32 = 2;
pub const INIT_ONCE_CHECK_ONLY: u32 = 1;
pub const INIT_ONCE_CTX_RESERVED_BITS: u32 = 2;
pub const INIT_ONCE_INIT_FAILED: u32 = 4;
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPINIT_ONCE(pub super::winnt::PRTL_RUN_ONCE);
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPSYNCHRONIZATION_BARRIER(pub super::winnt::PRTL_BARRIER);
pub const MUTEX_ALL_ACCESS: u32 = 2031617;
pub const MUTEX_MODIFY_STATE: u32 = 1;
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCONDITION_VARIABLE(pub *mut super::winnt::RTL_CONDITION_VARIABLE);
#[cfg(feature = "winnt")]
impl PCONDITION_VARIABLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PCONDITION_VARIABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PINIT_ONCE(pub super::winnt::PRTL_RUN_ONCE);
#[cfg(feature = "winnt")]
pub type PINIT_ONCE_FN = Option<unsafe extern "system" fn(initonce: PINIT_ONCE, parameter: *mut core::ffi::c_void, context: *mut *mut core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSRWLOCK(pub *mut super::winnt::RTL_SRWLOCK);
#[cfg(feature = "winnt")]
impl PSRWLOCK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PSRWLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PSYNCHRONIZATION_BARRIER(pub super::winnt::PRTL_BARRIER);
pub type PTIMERAPCROUTINE = Option<unsafe extern "system" fn(lpargtocompletionroutine: *const core::ffi::c_void, dwtimerlowvalue: u32, dwtimerhighvalue: u32)>;
#[cfg(feature = "winnt")]
pub type SRWLOCK = super::winnt::RTL_SRWLOCK;
#[cfg(feature = "winnt")]
pub type SYNCHRONIZATION_BARRIER = super::winnt::RTL_BARRIER;
pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: u32 = 2;
pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: u32 = 4;
pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: u32 = 1;
