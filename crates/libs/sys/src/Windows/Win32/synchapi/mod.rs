#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn AcquireSRWLockExclusive(srwlock : *mut super::winnt::RTL_SRWLOCK));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn AcquireSRWLockShared(srwlock : *mut super::winnt::RTL_SRWLOCK));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CancelWaitableTimer(htimer : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateEventA(lpeventattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, bmanualreset : windows_sys::core::BOOL, binitialstate : windows_sys::core::BOOL, lpname : windows_sys::core::PCSTR) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateEventExA(lpeventattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lpname : windows_sys::core::PCSTR, dwflags : u32, dwdesiredaccess : u32) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateEventExW(lpeventattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lpname : windows_sys::core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, bmanualreset : windows_sys::core::BOOL, binitialstate : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateMutexA(lpmutexattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, binitialowner : windows_sys::core::BOOL, lpname : windows_sys::core::PCSTR) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateMutexExA(lpmutexattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lpname : windows_sys::core::PCSTR, dwflags : u32, dwdesiredaccess : u32) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateMutexExW(lpmutexattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lpname : windows_sys::core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateMutexW(lpmutexattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, binitialowner : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateSemaphoreExW(lpsemaphoreattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, linitialcount : i32, lmaximumcount : i32, lpname : windows_sys::core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateSemaphoreW(lpsemaphoreattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, linitialcount : i32, lmaximumcount : i32, lpname : windows_sys::core::PCWSTR) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateWaitableTimerExW(lptimerattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lptimername : windows_sys::core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateWaitableTimerW(lptimerattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, bmanualreset : windows_sys::core::BOOL, lptimername : windows_sys::core::PCWSTR) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn DeleteCriticalSection(lpcriticalsection : super::minwinbase::LPCRITICAL_SECTION));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn DeleteSynchronizationBarrier(lpbarrier : LPSYNCHRONIZATION_BARRIER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn EnterCriticalSection(lpcriticalsection : super::minwinbase::LPCRITICAL_SECTION));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn EnterSynchronizationBarrier(lpbarrier : LPSYNCHRONIZATION_BARRIER, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn InitOnceBeginInitialize(lpinitonce : LPINIT_ONCE, dwflags : u32, fpending : *mut windows_sys::core::BOOL, lpcontext : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn InitOnceComplete(lpinitonce : LPINIT_ONCE, dwflags : u32, lpcontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn InitOnceExecuteOnce(initonce : PINIT_ONCE, initfn : PINIT_ONCE_FN, parameter : *mut core::ffi::c_void, context : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn InitOnceInitialize(initonce : PINIT_ONCE));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn InitializeConditionVariable(conditionvariable : *mut super::winnt::RTL_CONDITION_VARIABLE));
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn InitializeCriticalSection(lpcriticalsection : super::minwinbase::LPCRITICAL_SECTION));
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn InitializeCriticalSectionAndSpinCount(lpcriticalsection : super::minwinbase::LPCRITICAL_SECTION, dwspincount : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn InitializeCriticalSectionEx(lpcriticalsection : super::minwinbase::LPCRITICAL_SECTION, dwspincount : u32, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn InitializeSRWLock(srwlock : *mut super::winnt::RTL_SRWLOCK));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn InitializeSynchronizationBarrier(lpbarrier : LPSYNCHRONIZATION_BARRIER, ltotalthreads : i32, lspincount : i32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn LeaveCriticalSection(lpcriticalsection : super::minwinbase::LPCRITICAL_SECTION));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenEventA(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCSTR) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenEventW(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenMutexW(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenSemaphoreW(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenWaitableTimerW(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lptimername : windows_sys::core::PCWSTR) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseMutex(hmutex : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseSRWLockExclusive(srwlock : *mut super::winnt::RTL_SRWLOCK));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseSRWLockShared(srwlock : *mut super::winnt::RTL_SRWLOCK));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseSemaphore(hsemaphore : super::winnt::HANDLE, lreleasecount : i32, lppreviouscount : *mut i32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn ResetEvent(hevent : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetCriticalSectionSpinCount(lpcriticalsection : super::minwinbase::LPCRITICAL_SECTION, dwspincount : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn SetEvent(hevent : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn SetWaitableTimer(htimer : super::winnt::HANDLE, lpduetime : *const i64, lperiod : i32, pfncompletionroutine : PTIMERAPCROUTINE, lpargtocompletionroutine : *const core::ffi::c_void, fresume : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetWaitableTimerEx(htimer : super::winnt::HANDLE, lpduetime : *const i64, lperiod : i32, pfncompletionroutine : PTIMERAPCROUTINE, lpargtocompletionroutine : *const core::ffi::c_void, wakecontext : *const super::minwinbase::REASON_CONTEXT, tolerabledelay : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn SignalObjectAndWait(hobjecttosignal : super::winnt::HANDLE, hobjecttowaiton : super::winnt::HANDLE, dwmilliseconds : u32, balertable : windows_sys::core::BOOL) -> u32);
windows_link::link!("kernel32.dll" "system" fn Sleep(dwmilliseconds : u32));
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn SleepConditionVariableCS(conditionvariable : *mut super::winnt::RTL_CONDITION_VARIABLE, criticalsection : super::minwinbase::PCRITICAL_SECTION, dwmilliseconds : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn SleepConditionVariableSRW(conditionvariable : *mut super::winnt::RTL_CONDITION_VARIABLE, srwlock : *mut super::winnt::RTL_SRWLOCK, dwmilliseconds : u32, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SleepEx(dwmilliseconds : u32, balertable : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn TryAcquireSRWLockExclusive(srwlock : *mut super::winnt::RTL_SRWLOCK) -> bool);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn TryAcquireSRWLockShared(srwlock : *mut super::winnt::RTL_SRWLOCK) -> bool);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn TryEnterCriticalSection(lpcriticalsection : super::minwinbase::LPCRITICAL_SECTION) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForMultipleObjects(ncount : u32, lphandles : *const super::winnt::HANDLE, bwaitall : windows_sys::core::BOOL, dwmilliseconds : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForMultipleObjectsEx(ncount : u32, lphandles : *const super::winnt::HANDLE, bwaitall : windows_sys::core::BOOL, dwmilliseconds : u32, balertable : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForSingleObject(hhandle : super::winnt::HANDLE, dwmilliseconds : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn WaitForSingleObjectEx(hhandle : super::winnt::HANDLE, dwmilliseconds : u32, balertable : windows_sys::core::BOOL) -> u32);
windows_link::link!("vertdll.dll" "system" fn WaitOnAddress(address : *const core::ffi::c_void, compareaddress : *const core::ffi::c_void, addresssize : usize, dwmilliseconds : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn WakeAllConditionVariable(conditionvariable : *mut super::winnt::RTL_CONDITION_VARIABLE));
windows_link::link!("vertdll.dll" "system" fn WakeByAddressAll(address : *const core::ffi::c_void));
windows_link::link!("vertdll.dll" "system" fn WakeByAddressSingle(address : *const core::ffi::c_void));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn WakeConditionVariable(conditionvariable : *mut super::winnt::RTL_CONDITION_VARIABLE));
#[cfg(feature = "Win32_winnt")]
pub type CONDITION_VARIABLE = super::winnt::RTL_CONDITION_VARIABLE;
pub const CONDITION_VARIABLE_LOCKMODE_SHARED: u32 = 1;
pub const CREATE_EVENT_INITIAL_SET: u32 = 2;
pub const CREATE_EVENT_MANUAL_RESET: u32 = 1;
pub const CREATE_MUTEX_INITIAL_OWNER: u32 = 1;
pub const CREATE_WAITABLE_TIMER_HIGH_RESOLUTION: u32 = 2;
pub const CREATE_WAITABLE_TIMER_MANUAL_RESET: u32 = 1;
#[cfg(feature = "Win32_winnt")]
pub type INIT_ONCE = super::winnt::RTL_RUN_ONCE;
pub const INIT_ONCE_ASYNC: u32 = 2;
pub const INIT_ONCE_CHECK_ONLY: u32 = 1;
pub const INIT_ONCE_CTX_RESERVED_BITS: u32 = 2;
pub const INIT_ONCE_INIT_FAILED: u32 = 4;
#[cfg(feature = "Win32_winnt")]
pub type LPINIT_ONCE = super::winnt::PRTL_RUN_ONCE;
#[cfg(feature = "Win32_winnt")]
pub type LPSYNCHRONIZATION_BARRIER = super::winnt::PRTL_BARRIER;
pub const MUTEX_ALL_ACCESS: u32 = 2031617;
pub const MUTEX_MODIFY_STATE: u32 = 1;
#[cfg(feature = "Win32_winnt")]
pub type PCONDITION_VARIABLE = *mut super::winnt::RTL_CONDITION_VARIABLE;
#[cfg(feature = "Win32_winnt")]
pub type PINIT_ONCE = super::winnt::PRTL_RUN_ONCE;
#[cfg(feature = "Win32_winnt")]
pub type PINIT_ONCE_FN = Option<unsafe extern "system" fn(initonce: PINIT_ONCE, parameter: *mut core::ffi::c_void, context: *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "Win32_winnt")]
pub type PSRWLOCK = *mut super::winnt::RTL_SRWLOCK;
#[cfg(feature = "Win32_winnt")]
pub type PSYNCHRONIZATION_BARRIER = super::winnt::PRTL_BARRIER;
pub type PTIMERAPCROUTINE = Option<unsafe extern "system" fn(lpargtocompletionroutine: *const core::ffi::c_void, dwtimerlowvalue: u32, dwtimerhighvalue: u32)>;
#[cfg(feature = "Win32_winnt")]
pub type SRWLOCK = super::winnt::RTL_SRWLOCK;
#[cfg(feature = "Win32_winnt")]
pub type SYNCHRONIZATION_BARRIER = super::winnt::RTL_BARRIER;
pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: u32 = 2;
pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: u32 = 4;
pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: u32 = 1;
