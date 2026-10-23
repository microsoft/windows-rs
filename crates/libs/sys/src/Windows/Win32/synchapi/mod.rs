#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn AcquireSRWLockExclusive(srwlock : PSRWLOCK));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn AcquireSRWLockShared(srwlock : PSRWLOCK));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CancelWaitableTimer(htimer : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateEventA(lpeventattributes : super::LPSECURITY_ATTRIBUTES, bmanualreset : windows_sys::core::BOOL, binitialstate : windows_sys::core::BOOL, lpname : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateEventExA(lpeventattributes : super::LPSECURITY_ATTRIBUTES, lpname : windows_sys::core::PCSTR, dwflags : super::DWORD, dwdesiredaccess : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateEventExW(lpeventattributes : super::LPSECURITY_ATTRIBUTES, lpname : windows_sys::core::PCWSTR, dwflags : super::DWORD, dwdesiredaccess : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : super::LPSECURITY_ATTRIBUTES, bmanualreset : windows_sys::core::BOOL, binitialstate : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateMutexA(lpmutexattributes : super::LPSECURITY_ATTRIBUTES, binitialowner : windows_sys::core::BOOL, lpname : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateMutexExA(lpmutexattributes : super::LPSECURITY_ATTRIBUTES, lpname : windows_sys::core::PCSTR, dwflags : super::DWORD, dwdesiredaccess : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateMutexExW(lpmutexattributes : super::LPSECURITY_ATTRIBUTES, lpname : windows_sys::core::PCWSTR, dwflags : super::DWORD, dwdesiredaccess : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateMutexW(lpmutexattributes : super::LPSECURITY_ATTRIBUTES, binitialowner : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateSemaphoreExW(lpsemaphoreattributes : super::LPSECURITY_ATTRIBUTES, linitialcount : super::LONG, lmaximumcount : super::LONG, lpname : windows_sys::core::PCWSTR, dwflags : super::DWORD, dwdesiredaccess : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateSemaphoreW(lpsemaphoreattributes : super::LPSECURITY_ATTRIBUTES, linitialcount : super::LONG, lmaximumcount : super::LONG, lpname : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateWaitableTimerExW(lptimerattributes : super::LPSECURITY_ATTRIBUTES, lptimername : windows_sys::core::PCWSTR, dwflags : super::DWORD, dwdesiredaccess : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateWaitableTimerW(lptimerattributes : super::LPSECURITY_ATTRIBUTES, bmanualreset : windows_sys::core::BOOL, lptimername : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn DeleteCriticalSection(lpcriticalsection : super::LPCRITICAL_SECTION));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn DeleteSynchronizationBarrier(lpbarrier : LPSYNCHRONIZATION_BARRIER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn EnterCriticalSection(lpcriticalsection : super::LPCRITICAL_SECTION));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn EnterSynchronizationBarrier(lpbarrier : LPSYNCHRONIZATION_BARRIER, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InitOnceBeginInitialize(lpinitonce : LPINIT_ONCE, dwflags : super::DWORD, fpending : super::PBOOL, lpcontext : *mut super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InitOnceComplete(lpinitonce : LPINIT_ONCE, dwflags : super::DWORD, lpcontext : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InitOnceExecuteOnce(initonce : PINIT_ONCE, initfn : PINIT_ONCE_FN, parameter : super::PVOID, context : *mut super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InitOnceInitialize(initonce : PINIT_ONCE));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InitializeConditionVariable(conditionvariable : PCONDITION_VARIABLE));
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InitializeCriticalSection(lpcriticalsection : super::LPCRITICAL_SECTION));
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InitializeCriticalSectionAndSpinCount(lpcriticalsection : super::LPCRITICAL_SECTION, dwspincount : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InitializeCriticalSectionEx(lpcriticalsection : super::LPCRITICAL_SECTION, dwspincount : super::DWORD, flags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InitializeSRWLock(srwlock : PSRWLOCK));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InitializeSynchronizationBarrier(lpbarrier : LPSYNCHRONIZATION_BARRIER, ltotalthreads : super::LONG, lspincount : super::LONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LeaveCriticalSection(lpcriticalsection : super::LPCRITICAL_SECTION));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn OpenEventA(dwdesiredaccess : super::DWORD, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn OpenEventW(dwdesiredaccess : super::DWORD, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn OpenMutexW(dwdesiredaccess : super::DWORD, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn OpenSemaphoreW(dwdesiredaccess : super::DWORD, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn OpenWaitableTimerW(dwdesiredaccess : super::DWORD, binherithandle : windows_sys::core::BOOL, lptimername : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseMutex(hmutex : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseSRWLockExclusive(srwlock : PSRWLOCK));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseSRWLockShared(srwlock : PSRWLOCK));
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReleaseSemaphore(hsemaphore : super::HANDLE, lreleasecount : super::LONG, lppreviouscount : super::LPLONG) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ResetEvent(hevent : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetCriticalSectionSpinCount(lpcriticalsection : super::LPCRITICAL_SECTION, dwspincount : super::DWORD) -> super::DWORD);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetEvent(hevent : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetWaitableTimer(htimer : super::HANDLE, lpduetime : *const super::LARGE_INTEGER, lperiod : super::LONG, pfncompletionroutine : PTIMERAPCROUTINE, lpargtocompletionroutine : super::LPVOID, fresume : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetWaitableTimerEx(htimer : super::HANDLE, lpduetime : *const super::LARGE_INTEGER, lperiod : super::LONG, pfncompletionroutine : PTIMERAPCROUTINE, lpargtocompletionroutine : super::LPVOID, wakecontext : super::PREASON_CONTEXT, tolerabledelay : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn Sleep(dwmilliseconds : super::DWORD));
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SleepConditionVariableCS(conditionvariable : PCONDITION_VARIABLE, criticalsection : super::PCRITICAL_SECTION, dwmilliseconds : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SleepConditionVariableSRW(conditionvariable : PCONDITION_VARIABLE, srwlock : PSRWLOCK, dwmilliseconds : super::DWORD, flags : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn SleepEx(dwmilliseconds : super::DWORD, balertable : windows_sys::core::BOOL) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn TryAcquireSRWLockExclusive(srwlock : PSRWLOCK) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn TryAcquireSRWLockShared(srwlock : PSRWLOCK) -> super::BOOLEAN);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn TryEnterCriticalSection(lpcriticalsection : super::LPCRITICAL_SECTION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WaitForMultipleObjects(ncount : super::DWORD, lphandles : *const super::HANDLE, bwaitall : windows_sys::core::BOOL, dwmilliseconds : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WaitForMultipleObjectsEx(ncount : super::DWORD, lphandles : *const super::HANDLE, bwaitall : windows_sys::core::BOOL, dwmilliseconds : super::DWORD, balertable : windows_sys::core::BOOL) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WaitForSingleObject(hhandle : super::HANDLE, dwmilliseconds : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WaitForSingleObjectEx(hhandle : super::HANDLE, dwmilliseconds : super::DWORD, balertable : windows_sys::core::BOOL) -> super::DWORD);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WakeAllConditionVariable(conditionvariable : PCONDITION_VARIABLE));
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WakeByAddressAll(address : super::PVOID));
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WakeByAddressSingle(address : super::PVOID));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WakeConditionVariable(conditionvariable : PCONDITION_VARIABLE));
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
pub type PINIT_ONCE_FN = Option<unsafe extern "system" fn(initonce: PINIT_ONCE, parameter: super::PVOID, context: *mut super::PVOID) -> windows_sys::core::BOOL>;
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
