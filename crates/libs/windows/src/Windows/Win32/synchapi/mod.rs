#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AcquireSRWLockExclusive(srwlock: PSRWLOCK) {
    windows_core::link!("kernel32.dll" "system" fn AcquireSRWLockExclusive(srwlock : PSRWLOCK));
    unsafe { AcquireSRWLockExclusive(srwlock as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AcquireSRWLockShared(srwlock: PSRWLOCK) {
    windows_core::link!("kernel32.dll" "system" fn AcquireSRWLockShared(srwlock : PSRWLOCK));
    unsafe { AcquireSRWLockShared(srwlock as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CancelWaitableTimer(htimer: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CancelWaitableTimer(htimer : super::HANDLE) -> windows_core::BOOL);
    unsafe { CancelWaitableTimer(htimer) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateEventA<P3>(lpeventattributes: Option<super::LPSECURITY_ATTRIBUTES>, bmanualreset: bool, binitialstate: bool, lpname: P3) -> super::HANDLE
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateEventA(lpeventattributes : super::LPSECURITY_ATTRIBUTES, bmanualreset : windows_core::BOOL, binitialstate : windows_core::BOOL, lpname : windows_core::PCSTR) -> super::HANDLE);
    unsafe { CreateEventA(lpeventattributes.unwrap_or(core::mem::zeroed()) as _, bmanualreset.into(), binitialstate.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateEventExA<P1>(lpeventattributes: Option<super::LPSECURITY_ATTRIBUTES>, lpname: P1, dwflags: super::DWORD, dwdesiredaccess: super::DWORD) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateEventExA(lpeventattributes : super::LPSECURITY_ATTRIBUTES, lpname : windows_core::PCSTR, dwflags : super::DWORD, dwdesiredaccess : super::DWORD) -> super::HANDLE);
    unsafe { CreateEventExA(lpeventattributes.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi(), dwflags, dwdesiredaccess) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateEventExW<P1>(lpeventattributes: Option<super::LPSECURITY_ATTRIBUTES>, lpname: P1, dwflags: super::DWORD, dwdesiredaccess: super::DWORD) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateEventExW(lpeventattributes : super::LPSECURITY_ATTRIBUTES, lpname : windows_core::PCWSTR, dwflags : super::DWORD, dwdesiredaccess : super::DWORD) -> super::HANDLE);
    unsafe { CreateEventExW(lpeventattributes.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi(), dwflags, dwdesiredaccess) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateEventW<P3>(lpeventattributes: Option<super::LPSECURITY_ATTRIBUTES>, bmanualreset: bool, binitialstate: bool, lpname: P3) -> super::HANDLE
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : super::LPSECURITY_ATTRIBUTES, bmanualreset : windows_core::BOOL, binitialstate : windows_core::BOOL, lpname : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { CreateEventW(lpeventattributes.unwrap_or(core::mem::zeroed()) as _, bmanualreset.into(), binitialstate.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateMutexA<P2>(lpmutexattributes: Option<super::LPSECURITY_ATTRIBUTES>, binitialowner: bool, lpname: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateMutexA(lpmutexattributes : super::LPSECURITY_ATTRIBUTES, binitialowner : windows_core::BOOL, lpname : windows_core::PCSTR) -> super::HANDLE);
    unsafe { CreateMutexA(lpmutexattributes.unwrap_or(core::mem::zeroed()) as _, binitialowner.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateMutexExA<P1>(lpmutexattributes: Option<super::LPSECURITY_ATTRIBUTES>, lpname: P1, dwflags: super::DWORD, dwdesiredaccess: super::DWORD) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateMutexExA(lpmutexattributes : super::LPSECURITY_ATTRIBUTES, lpname : windows_core::PCSTR, dwflags : super::DWORD, dwdesiredaccess : super::DWORD) -> super::HANDLE);
    unsafe { CreateMutexExA(lpmutexattributes.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi(), dwflags, dwdesiredaccess) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateMutexExW<P1>(lpmutexattributes: Option<super::LPSECURITY_ATTRIBUTES>, lpname: P1, dwflags: super::DWORD, dwdesiredaccess: super::DWORD) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateMutexExW(lpmutexattributes : super::LPSECURITY_ATTRIBUTES, lpname : windows_core::PCWSTR, dwflags : super::DWORD, dwdesiredaccess : super::DWORD) -> super::HANDLE);
    unsafe { CreateMutexExW(lpmutexattributes.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi(), dwflags, dwdesiredaccess) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateMutexW<P2>(lpmutexattributes: Option<super::LPSECURITY_ATTRIBUTES>, binitialowner: bool, lpname: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateMutexW(lpmutexattributes : super::LPSECURITY_ATTRIBUTES, binitialowner : windows_core::BOOL, lpname : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { CreateMutexW(lpmutexattributes.unwrap_or(core::mem::zeroed()) as _, binitialowner.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateSemaphoreExW<P3>(lpsemaphoreattributes: Option<super::LPSECURITY_ATTRIBUTES>, linitialcount: super::LONG, lmaximumcount: super::LONG, lpname: P3, dwflags: Option<super::DWORD>, dwdesiredaccess: super::DWORD) -> super::HANDLE
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateSemaphoreExW(lpsemaphoreattributes : super::LPSECURITY_ATTRIBUTES, linitialcount : super::LONG, lmaximumcount : super::LONG, lpname : windows_core::PCWSTR, dwflags : super::DWORD, dwdesiredaccess : super::DWORD) -> super::HANDLE);
    unsafe { CreateSemaphoreExW(lpsemaphoreattributes.unwrap_or(core::mem::zeroed()) as _, linitialcount, lmaximumcount, lpname.param().abi(), dwflags.unwrap_or(core::mem::zeroed()) as _, dwdesiredaccess) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateSemaphoreW<P3>(lpsemaphoreattributes: Option<super::LPSECURITY_ATTRIBUTES>, linitialcount: super::LONG, lmaximumcount: super::LONG, lpname: P3) -> super::HANDLE
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateSemaphoreW(lpsemaphoreattributes : super::LPSECURITY_ATTRIBUTES, linitialcount : super::LONG, lmaximumcount : super::LONG, lpname : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { CreateSemaphoreW(lpsemaphoreattributes.unwrap_or(core::mem::zeroed()) as _, linitialcount, lmaximumcount, lpname.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateWaitableTimerExW<P1>(lptimerattributes: Option<super::LPSECURITY_ATTRIBUTES>, lptimername: P1, dwflags: super::DWORD, dwdesiredaccess: super::DWORD) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateWaitableTimerExW(lptimerattributes : super::LPSECURITY_ATTRIBUTES, lptimername : windows_core::PCWSTR, dwflags : super::DWORD, dwdesiredaccess : super::DWORD) -> super::HANDLE);
    unsafe { CreateWaitableTimerExW(lptimerattributes.unwrap_or(core::mem::zeroed()) as _, lptimername.param().abi(), dwflags, dwdesiredaccess) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateWaitableTimerW<P2>(lptimerattributes: Option<super::LPSECURITY_ATTRIBUTES>, bmanualreset: bool, lptimername: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateWaitableTimerW(lptimerattributes : super::LPSECURITY_ATTRIBUTES, bmanualreset : windows_core::BOOL, lptimername : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { CreateWaitableTimerW(lptimerattributes.unwrap_or(core::mem::zeroed()) as _, bmanualreset.into(), lptimername.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn DeleteCriticalSection(lpcriticalsection: super::LPCRITICAL_SECTION) {
    windows_core::link!("kernel32.dll" "system" fn DeleteCriticalSection(lpcriticalsection : super::LPCRITICAL_SECTION));
    unsafe { DeleteCriticalSection(lpcriticalsection as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn DeleteSynchronizationBarrier(lpbarrier: LPSYNCHRONIZATION_BARRIER) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DeleteSynchronizationBarrier(lpbarrier : LPSYNCHRONIZATION_BARRIER) -> windows_core::BOOL);
    unsafe { DeleteSynchronizationBarrier(lpbarrier as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EnterCriticalSection(lpcriticalsection: super::LPCRITICAL_SECTION) {
    windows_core::link!("kernel32.dll" "system" fn EnterCriticalSection(lpcriticalsection : super::LPCRITICAL_SECTION));
    unsafe { EnterCriticalSection(lpcriticalsection as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EnterSynchronizationBarrier(lpbarrier: LPSYNCHRONIZATION_BARRIER, dwflags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EnterSynchronizationBarrier(lpbarrier : LPSYNCHRONIZATION_BARRIER, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { EnterSynchronizationBarrier(lpbarrier as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InitOnceBeginInitialize(lpinitonce: LPINIT_ONCE, dwflags: super::DWORD, fpending: super::PBOOL, lpcontext: Option<*mut super::LPVOID>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitOnceBeginInitialize(lpinitonce : LPINIT_ONCE, dwflags : super::DWORD, fpending : super::PBOOL, lpcontext : *mut super::LPVOID) -> windows_core::BOOL);
    unsafe { InitOnceBeginInitialize(lpinitonce as _, dwflags, fpending as _, lpcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InitOnceComplete(lpinitonce: LPINIT_ONCE, dwflags: super::DWORD, lpcontext: Option<super::LPVOID>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitOnceComplete(lpinitonce : LPINIT_ONCE, dwflags : super::DWORD, lpcontext : super::LPVOID) -> windows_core::BOOL);
    unsafe { InitOnceComplete(lpinitonce as _, dwflags, lpcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn InitOnceExecuteOnce(initonce: PINIT_ONCE, initfn: PINIT_ONCE_FN, parameter: Option<super::PVOID>, context: Option<*mut super::LPVOID>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitOnceExecuteOnce(initonce : PINIT_ONCE, initfn : PINIT_ONCE_FN, parameter : super::PVOID, context : *mut super::LPVOID) -> windows_core::BOOL);
    unsafe { InitOnceExecuteOnce(initonce as _, initfn, parameter.unwrap_or(core::mem::zeroed()) as _, context.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitOnceInitialize(initonce: PINIT_ONCE) {
    windows_core::link!("kernel32.dll" "system" fn InitOnceInitialize(initonce : PINIT_ONCE));
    unsafe { InitOnceInitialize(initonce as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitializeConditionVariable(conditionvariable: PCONDITION_VARIABLE) {
    windows_core::link!("kernel32.dll" "system" fn InitializeConditionVariable(conditionvariable : PCONDITION_VARIABLE));
    unsafe { InitializeConditionVariable(conditionvariable as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InitializeCriticalSection(lpcriticalsection: super::LPCRITICAL_SECTION) {
    windows_core::link!("kernel32.dll" "system" fn InitializeCriticalSection(lpcriticalsection : super::LPCRITICAL_SECTION));
    unsafe { InitializeCriticalSection(lpcriticalsection as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InitializeCriticalSectionAndSpinCount(lpcriticalsection: super::LPCRITICAL_SECTION, dwspincount: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitializeCriticalSectionAndSpinCount(lpcriticalsection : super::LPCRITICAL_SECTION, dwspincount : super::DWORD) -> windows_core::BOOL);
    unsafe { InitializeCriticalSectionAndSpinCount(lpcriticalsection as _, dwspincount) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InitializeCriticalSectionEx(lpcriticalsection: super::LPCRITICAL_SECTION, dwspincount: super::DWORD, flags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitializeCriticalSectionEx(lpcriticalsection : super::LPCRITICAL_SECTION, dwspincount : super::DWORD, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { InitializeCriticalSectionEx(lpcriticalsection as _, dwspincount, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitializeSRWLock(srwlock: PSRWLOCK) {
    windows_core::link!("kernel32.dll" "system" fn InitializeSRWLock(srwlock : PSRWLOCK));
    unsafe { InitializeSRWLock(srwlock as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InitializeSynchronizationBarrier(lpbarrier: LPSYNCHRONIZATION_BARRIER, ltotalthreads: super::LONG, lspincount: super::LONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitializeSynchronizationBarrier(lpbarrier : LPSYNCHRONIZATION_BARRIER, ltotalthreads : super::LONG, lspincount : super::LONG) -> windows_core::BOOL);
    unsafe { InitializeSynchronizationBarrier(lpbarrier as _, ltotalthreads, lspincount) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn LeaveCriticalSection(lpcriticalsection: super::LPCRITICAL_SECTION) {
    windows_core::link!("kernel32.dll" "system" fn LeaveCriticalSection(lpcriticalsection : super::LPCRITICAL_SECTION));
    unsafe { LeaveCriticalSection(lpcriticalsection as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenEventA<P2>(dwdesiredaccess: super::DWORD, binherithandle: bool, lpname: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenEventA(dwdesiredaccess : super::DWORD, binherithandle : windows_core::BOOL, lpname : windows_core::PCSTR) -> super::HANDLE);
    unsafe { OpenEventA(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenEventW<P2>(dwdesiredaccess: super::DWORD, binherithandle: bool, lpname: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenEventW(dwdesiredaccess : super::DWORD, binherithandle : windows_core::BOOL, lpname : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { OpenEventW(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenMutexW<P2>(dwdesiredaccess: super::DWORD, binherithandle: bool, lpname: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenMutexW(dwdesiredaccess : super::DWORD, binherithandle : windows_core::BOOL, lpname : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { OpenMutexW(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenSemaphoreW<P2>(dwdesiredaccess: super::DWORD, binherithandle: bool, lpname: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenSemaphoreW(dwdesiredaccess : super::DWORD, binherithandle : windows_core::BOOL, lpname : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { OpenSemaphoreW(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenWaitableTimerW<P2>(dwdesiredaccess: super::DWORD, binherithandle: bool, lptimername: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenWaitableTimerW(dwdesiredaccess : super::DWORD, binherithandle : windows_core::BOOL, lptimername : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { OpenWaitableTimerW(dwdesiredaccess, binherithandle.into(), lptimername.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReleaseMutex(hmutex: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReleaseMutex(hmutex : super::HANDLE) -> windows_core::BOOL);
    unsafe { ReleaseMutex(hmutex) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReleaseSRWLockExclusive(srwlock: PSRWLOCK) {
    windows_core::link!("kernel32.dll" "system" fn ReleaseSRWLockExclusive(srwlock : PSRWLOCK));
    unsafe { ReleaseSRWLockExclusive(srwlock as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReleaseSRWLockShared(srwlock: PSRWLOCK) {
    windows_core::link!("kernel32.dll" "system" fn ReleaseSRWLockShared(srwlock : PSRWLOCK));
    unsafe { ReleaseSRWLockShared(srwlock as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReleaseSemaphore(hsemaphore: super::HANDLE, lreleasecount: super::LONG, lppreviouscount: Option<super::LPLONG>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReleaseSemaphore(hsemaphore : super::HANDLE, lreleasecount : super::LONG, lppreviouscount : super::LPLONG) -> windows_core::BOOL);
    unsafe { ReleaseSemaphore(hsemaphore, lreleasecount, lppreviouscount.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ResetEvent(hevent: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ResetEvent(hevent : super::HANDLE) -> windows_core::BOOL);
    unsafe { ResetEvent(hevent) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetCriticalSectionSpinCount(lpcriticalsection: super::LPCRITICAL_SECTION, dwspincount: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn SetCriticalSectionSpinCount(lpcriticalsection : super::LPCRITICAL_SECTION, dwspincount : super::DWORD) -> super::DWORD);
    unsafe { SetCriticalSectionSpinCount(lpcriticalsection as _, dwspincount) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetEvent(hevent: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetEvent(hevent : super::HANDLE) -> windows_core::BOOL);
    unsafe { SetEvent(hevent) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetWaitableTimer(htimer: super::HANDLE, lpduetime: *const super::LARGE_INTEGER, lperiod: super::LONG, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: Option<super::LPVOID>, fresume: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetWaitableTimer(htimer : super::HANDLE, lpduetime : *const super::LARGE_INTEGER, lperiod : super::LONG, pfncompletionroutine : PTIMERAPCROUTINE, lpargtocompletionroutine : super::LPVOID, fresume : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetWaitableTimer(htimer, lpduetime, lperiod, pfncompletionroutine, lpargtocompletionroutine.unwrap_or(core::mem::zeroed()) as _, fresume.into()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetWaitableTimerEx(htimer: super::HANDLE, lpduetime: *const super::LARGE_INTEGER, lperiod: super::LONG, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: Option<super::LPVOID>, wakecontext: Option<super::PREASON_CONTEXT>, tolerabledelay: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetWaitableTimerEx(htimer : super::HANDLE, lpduetime : *const super::LARGE_INTEGER, lperiod : super::LONG, pfncompletionroutine : PTIMERAPCROUTINE, lpargtocompletionroutine : super::LPVOID, wakecontext : super::PREASON_CONTEXT, tolerabledelay : super::ULONG) -> windows_core::BOOL);
    unsafe { SetWaitableTimerEx(htimer, lpduetime, lperiod, pfncompletionroutine, lpargtocompletionroutine.unwrap_or(core::mem::zeroed()) as _, wakecontext.unwrap_or(core::mem::zeroed()) as _, tolerabledelay) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn Sleep(dwmilliseconds: super::DWORD) {
    windows_core::link!("kernel32.dll" "system" fn Sleep(dwmilliseconds : super::DWORD));
    unsafe { Sleep(dwmilliseconds) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SleepConditionVariableCS(conditionvariable: PCONDITION_VARIABLE, criticalsection: super::PCRITICAL_SECTION, dwmilliseconds: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SleepConditionVariableCS(conditionvariable : PCONDITION_VARIABLE, criticalsection : super::PCRITICAL_SECTION, dwmilliseconds : super::DWORD) -> windows_core::BOOL);
    unsafe { SleepConditionVariableCS(conditionvariable as _, criticalsection as _, dwmilliseconds) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SleepConditionVariableSRW(conditionvariable: PCONDITION_VARIABLE, srwlock: PSRWLOCK, dwmilliseconds: super::DWORD, flags: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SleepConditionVariableSRW(conditionvariable : PCONDITION_VARIABLE, srwlock : PSRWLOCK, dwmilliseconds : super::DWORD, flags : super::ULONG) -> windows_core::BOOL);
    unsafe { SleepConditionVariableSRW(conditionvariable as _, srwlock as _, dwmilliseconds, flags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SleepEx(dwmilliseconds: super::DWORD, balertable: bool) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn SleepEx(dwmilliseconds : super::DWORD, balertable : windows_core::BOOL) -> super::DWORD);
    unsafe { SleepEx(dwmilliseconds, balertable.into()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TryAcquireSRWLockExclusive(srwlock: PSRWLOCK) -> super::BOOLEAN {
    windows_core::link!("kernel32.dll" "system" fn TryAcquireSRWLockExclusive(srwlock : PSRWLOCK) -> super::BOOLEAN);
    unsafe { TryAcquireSRWLockExclusive(srwlock as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TryAcquireSRWLockShared(srwlock: PSRWLOCK) -> super::BOOLEAN {
    windows_core::link!("kernel32.dll" "system" fn TryAcquireSRWLockShared(srwlock : PSRWLOCK) -> super::BOOLEAN);
    unsafe { TryAcquireSRWLockShared(srwlock as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TryEnterCriticalSection(lpcriticalsection: super::LPCRITICAL_SECTION) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TryEnterCriticalSection(lpcriticalsection : super::LPCRITICAL_SECTION) -> windows_core::BOOL);
    unsafe { TryEnterCriticalSection(lpcriticalsection as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WaitForMultipleObjects(lphandles: &[super::HANDLE], bwaitall: bool, dwmilliseconds: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn WaitForMultipleObjects(ncount : super::DWORD, lphandles : *const super::HANDLE, bwaitall : windows_core::BOOL, dwmilliseconds : super::DWORD) -> super::DWORD);
    unsafe { WaitForMultipleObjects(lphandles.len().try_into().unwrap(), lphandles.as_ptr(), bwaitall.into(), dwmilliseconds) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WaitForMultipleObjectsEx(lphandles: &[super::HANDLE], bwaitall: bool, dwmilliseconds: super::DWORD, balertable: bool) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn WaitForMultipleObjectsEx(ncount : super::DWORD, lphandles : *const super::HANDLE, bwaitall : windows_core::BOOL, dwmilliseconds : super::DWORD, balertable : windows_core::BOOL) -> super::DWORD);
    unsafe { WaitForMultipleObjectsEx(lphandles.len().try_into().unwrap(), lphandles.as_ptr(), bwaitall.into(), dwmilliseconds, balertable.into()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WaitForSingleObject(hhandle: super::HANDLE, dwmilliseconds: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn WaitForSingleObject(hhandle : super::HANDLE, dwmilliseconds : super::DWORD) -> super::DWORD);
    unsafe { WaitForSingleObject(hhandle, dwmilliseconds) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WaitForSingleObjectEx(hhandle: super::HANDLE, dwmilliseconds: super::DWORD, balertable: bool) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn WaitForSingleObjectEx(hhandle : super::HANDLE, dwmilliseconds : super::DWORD, balertable : windows_core::BOOL) -> super::DWORD);
    unsafe { WaitForSingleObjectEx(hhandle, dwmilliseconds, balertable.into()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WaitOnAddress(address: *const core::ffi::c_void, compareaddress: super::PVOID, addresssize: super::SIZE_T, dwmilliseconds: Option<super::DWORD>) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WaitOnAddress(address : *const core::ffi::c_void, compareaddress : super::PVOID, addresssize : super::SIZE_T, dwmilliseconds : super::DWORD) -> windows_core::BOOL);
    unsafe { WaitOnAddress(address, compareaddress, addresssize, dwmilliseconds.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WakeAllConditionVariable(conditionvariable: PCONDITION_VARIABLE) {
    windows_core::link!("kernel32.dll" "system" fn WakeAllConditionVariable(conditionvariable : PCONDITION_VARIABLE));
    unsafe { WakeAllConditionVariable(conditionvariable as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WakeByAddressAll(address: super::PVOID) {
    windows_core::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WakeByAddressAll(address : super::PVOID));
    unsafe { WakeByAddressAll(address) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WakeByAddressSingle(address: super::PVOID) {
    windows_core::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WakeByAddressSingle(address : super::PVOID));
    unsafe { WakeByAddressSingle(address) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WakeConditionVariable(conditionvariable: PCONDITION_VARIABLE) {
    windows_core::link!("kernel32.dll" "system" fn WakeConditionVariable(conditionvariable : PCONDITION_VARIABLE));
    unsafe { WakeConditionVariable(conditionvariable as _) }
}
#[cfg(feature = "winnt")]
pub type CONDITION_VARIABLE = super::RTL_CONDITION_VARIABLE;
pub const CONDITION_VARIABLE_LOCKMODE_SHARED: i32 = 1;
pub const CREATE_EVENT_INITIAL_SET: i32 = 2;
pub const CREATE_EVENT_MANUAL_RESET: i32 = 1;
pub const CREATE_MUTEX_INITIAL_OWNER: i32 = 1;
pub const CREATE_WAITABLE_TIMER_HIGH_RESOLUTION: i32 = 2;
pub const CREATE_WAITABLE_TIMER_MANUAL_RESET: i32 = 1;
#[cfg(feature = "winnt")]
pub type INIT_ONCE = super::RTL_RUN_ONCE;
pub const INIT_ONCE_ASYNC: u32 = 2;
pub const INIT_ONCE_CHECK_ONLY: u32 = 1;
pub const INIT_ONCE_CTX_RESERVED_BITS: i32 = 2;
pub const INIT_ONCE_INIT_FAILED: u32 = 4;
#[cfg(feature = "winnt")]
pub type LPINIT_ONCE = *mut super::RTL_RUN_ONCE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSYNCHRONIZATION_BARRIER = *mut super::RTL_BARRIER;
pub const MUTEX_ALL_ACCESS: i32 = 2031617;
pub const MUTEX_MODIFY_STATE: i32 = 1;
#[cfg(feature = "winnt")]
pub type PCONDITION_VARIABLE = *mut super::RTL_CONDITION_VARIABLE;
#[cfg(feature = "winnt")]
pub type PINIT_ONCE = *mut super::RTL_RUN_ONCE;
#[cfg(feature = "winnt")]
pub type PINIT_ONCE_FN = Option<unsafe extern "system" fn(initonce: PINIT_ONCE, parameter: super::PVOID, context: *mut super::PVOID) -> windows_core::BOOL>;
#[cfg(feature = "winnt")]
pub type PSRWLOCK = *mut super::RTL_SRWLOCK;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYNCHRONIZATION_BARRIER = *mut super::RTL_BARRIER;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PTIMERAPCROUTINE = Option<unsafe extern "system" fn(lpargtocompletionroutine: super::LPVOID, dwtimerlowvalue: super::DWORD, dwtimerhighvalue: super::DWORD)>;
#[cfg(feature = "winnt")]
pub type SRWLOCK = super::RTL_SRWLOCK;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type SYNCHRONIZATION_BARRIER = super::RTL_BARRIER;
pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: i32 = 2;
pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: i32 = 4;
pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: i32 = 1;
