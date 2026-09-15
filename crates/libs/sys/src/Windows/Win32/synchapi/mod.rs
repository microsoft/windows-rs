#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn AcquireSRWLockExclusive(srwlock : PSRWLOCK));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn AcquireSRWLockShared(srwlock : PSRWLOCK));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CancelWaitableTimer(htimer : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateEventA(lpeventattributes : super::LPSECURITY_ATTRIBUTES, bmanualreset : windows_sys::core::BOOL, binitialstate : windows_sys::core::BOOL, lpname : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateEventExA(lpeventattributes : super::LPSECURITY_ATTRIBUTES, lpname : windows_sys::core::PCSTR, dwflags : u32, dwdesiredaccess : u32) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateEventExW(lpeventattributes : super::LPSECURITY_ATTRIBUTES, lpname : windows_sys::core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : super::LPSECURITY_ATTRIBUTES, bmanualreset : windows_sys::core::BOOL, binitialstate : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateMutexA(lpmutexattributes : super::LPSECURITY_ATTRIBUTES, binitialowner : windows_sys::core::BOOL, lpname : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateMutexExA(lpmutexattributes : super::LPSECURITY_ATTRIBUTES, lpname : windows_sys::core::PCSTR, dwflags : u32, dwdesiredaccess : u32) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateMutexExW(lpmutexattributes : super::LPSECURITY_ATTRIBUTES, lpname : windows_sys::core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateMutexW(lpmutexattributes : super::LPSECURITY_ATTRIBUTES, binitialowner : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateSemaphoreExW(lpsemaphoreattributes : super::LPSECURITY_ATTRIBUTES, linitialcount : i32, lmaximumcount : i32, lpname : windows_sys::core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateSemaphoreW(lpsemaphoreattributes : super::LPSECURITY_ATTRIBUTES, linitialcount : i32, lmaximumcount : i32, lpname : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateWaitableTimerExW(lptimerattributes : super::LPSECURITY_ATTRIBUTES, lptimername : windows_sys::core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateWaitableTimerW(lptimerattributes : super::LPSECURITY_ATTRIBUTES, bmanualreset : windows_sys::core::BOOL, lptimername : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn DeleteCriticalSection(lpcriticalsection : super::LPCRITICAL_SECTION));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DeleteSynchronizationBarrier(lpbarrier : LPSYNCHRONIZATION_BARRIER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn EnterCriticalSection(lpcriticalsection : super::LPCRITICAL_SECTION));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EnterSynchronizationBarrier(lpbarrier : LPSYNCHRONIZATION_BARRIER, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InitOnceBeginInitialize(lpinitonce : LPINIT_ONCE, dwflags : u32, fpending : super::PBOOL, lpcontext : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InitOnceComplete(lpinitonce : LPINIT_ONCE, dwflags : u32, lpcontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InitOnceExecuteOnce(initonce : PINIT_ONCE, initfn : PINIT_ONCE_FN, parameter : *mut core::ffi::c_void, context : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InitOnceInitialize(initonce : PINIT_ONCE));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InitializeConditionVariable(conditionvariable : PCONDITION_VARIABLE));
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InitializeCriticalSection(lpcriticalsection : super::LPCRITICAL_SECTION));
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InitializeCriticalSectionAndSpinCount(lpcriticalsection : super::LPCRITICAL_SECTION, dwspincount : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InitializeCriticalSectionEx(lpcriticalsection : super::LPCRITICAL_SECTION, dwspincount : u32, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InitializeSRWLock(srwlock : PSRWLOCK));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InitializeSynchronizationBarrier(lpbarrier : LPSYNCHRONIZATION_BARRIER, ltotalthreads : i32, lspincount : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LeaveCriticalSection(lpcriticalsection : super::LPCRITICAL_SECTION));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenEventA(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenEventW(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenMutexW(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenSemaphoreW(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenWaitableTimerW(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lptimername : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseMutex(hmutex : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseSRWLockExclusive(srwlock : PSRWLOCK));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseSRWLockShared(srwlock : PSRWLOCK));
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReleaseSemaphore(hsemaphore : super::HANDLE, lreleasecount : i32, lppreviouscount : super::LPLONG) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ResetEvent(hevent : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetCriticalSectionSpinCount(lpcriticalsection : super::LPCRITICAL_SECTION, dwspincount : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetEvent(hevent : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetWaitableTimer(htimer : super::HANDLE, lpduetime : *const super::LARGE_INTEGER, lperiod : i32, pfncompletionroutine : PTIMERAPCROUTINE, lpargtocompletionroutine : *const core::ffi::c_void, fresume : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetWaitableTimerEx(htimer : super::HANDLE, lpduetime : *const super::LARGE_INTEGER, lperiod : i32, pfncompletionroutine : PTIMERAPCROUTINE, lpargtocompletionroutine : *const core::ffi::c_void, wakecontext : super::PREASON_CONTEXT, tolerabledelay : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SignalObjectAndWait(hobjecttosignal : super::HANDLE, hobjecttowaiton : super::HANDLE, dwmilliseconds : u32, balertable : windows_sys::core::BOOL) -> u32);
windows_link::link!("kernel32.dll" "system" fn Sleep(dwmilliseconds : u32));
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SleepConditionVariableCS(conditionvariable : PCONDITION_VARIABLE, criticalsection : super::PCRITICAL_SECTION, dwmilliseconds : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SleepConditionVariableSRW(conditionvariable : PCONDITION_VARIABLE, srwlock : PSRWLOCK, dwmilliseconds : u32, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SleepEx(dwmilliseconds : u32, balertable : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn TryAcquireSRWLockExclusive(srwlock : PSRWLOCK) -> super::BOOLEAN);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn TryAcquireSRWLockShared(srwlock : PSRWLOCK) -> super::BOOLEAN);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn TryEnterCriticalSection(lpcriticalsection : super::LPCRITICAL_SECTION) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForMultipleObjects(ncount : u32, lphandles : *const super::HANDLE, bwaitall : windows_sys::core::BOOL, dwmilliseconds : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForMultipleObjectsEx(ncount : u32, lphandles : *const super::HANDLE, bwaitall : windows_sys::core::BOOL, dwmilliseconds : u32, balertable : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForSingleObject(hhandle : super::HANDLE, dwmilliseconds : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForSingleObjectEx(hhandle : super::HANDLE, dwmilliseconds : u32, balertable : windows_sys::core::BOOL) -> u32);
windows_link::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WaitOnAddress(address : *const core::ffi::c_void, compareaddress : *const core::ffi::c_void, addresssize : usize, dwmilliseconds : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WakeAllConditionVariable(conditionvariable : PCONDITION_VARIABLE));
windows_link::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WakeByAddressAll(address : *const core::ffi::c_void));
windows_link::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WakeByAddressSingle(address : *const core::ffi::c_void));
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
pub type LPINIT_ONCE = super::PRTL_RUN_ONCE;
#[cfg(feature = "winnt")]
pub type LPSYNCHRONIZATION_BARRIER = super::PRTL_BARRIER;
pub const MUTEX_ALL_ACCESS: i32 = 2031617;
pub const MUTEX_MODIFY_STATE: i32 = 1;
#[cfg(feature = "winnt")]
pub type PCONDITION_VARIABLE = *mut super::RTL_CONDITION_VARIABLE;
#[cfg(feature = "winnt")]
pub type PINIT_ONCE = super::PRTL_RUN_ONCE;
#[cfg(feature = "winnt")]
pub type PINIT_ONCE_FN = Option<unsafe extern "system" fn(initonce: PINIT_ONCE, parameter: *mut core::ffi::c_void, context: *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type PSRWLOCK = *mut super::RTL_SRWLOCK;
#[cfg(feature = "winnt")]
pub type PSYNCHRONIZATION_BARRIER = super::PRTL_BARRIER;
pub type PTIMERAPCROUTINE = Option<unsafe extern "system" fn(lpargtocompletionroutine: *const core::ffi::c_void, dwtimerlowvalue: u32, dwtimerhighvalue: u32)>;
#[cfg(feature = "winnt")]
pub type SRWLOCK = super::RTL_SRWLOCK;
#[cfg(feature = "winnt")]
pub type SYNCHRONIZATION_BARRIER = super::RTL_BARRIER;
pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: i32 = 2;
pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: i32 = 4;
pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: i32 = 1;
