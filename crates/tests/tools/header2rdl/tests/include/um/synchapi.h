/********************************************************************************
*                                                                               *
* synchapi.h -- ApiSet Contract for api-ms-win-core-synch-l1                    *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _SYNCHAPI_H_
#define _SYNCHAPI_H_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Application or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// Define the slim R/W lock.
//

#define SRWLOCK_INIT RTL_SRWLOCK_INIT

typedef RTL_SRWLOCK SRWLOCK, *PSRWLOCK;

#if (_WIN32_WINNT >= 0x0600)

WINBASEAPI
VOID
WINAPI
InitializeSRWLock(
    _Out_ PSRWLOCK SRWLock
    );

WINBASEAPI
_Releases_exclusive_lock_(*SRWLock)
_Releases_nonreentrant_lock_(*SRWLock)
VOID
WINAPI
ReleaseSRWLockExclusive(
    _Inout_ PSRWLOCK SRWLock
    );

WINBASEAPI
_Releases_shared_lock_(*SRWLock)
_Releases_nonreentrant_lock_(*SRWLock)
VOID
WINAPI
ReleaseSRWLockShared(
    _Inout_ PSRWLOCK SRWLock
    );

WINBASEAPI
_Acquires_exclusive_lock_(*SRWLock)
_Acquires_nonreentrant_lock_(*SRWLock)
VOID
WINAPI
AcquireSRWLockExclusive(
    _Inout_ PSRWLOCK SRWLock
    );

WINBASEAPI
_Acquires_shared_lock_(*SRWLock)
_Acquires_nonreentrant_lock_(*SRWLock)
VOID
WINAPI
AcquireSRWLockShared(
    _Inout_ PSRWLOCK SRWLock
    );

WINBASEAPI
_When_(return!=0, _Acquires_exclusive_lock_(*SRWLock))
_When_(return!=0, _Acquires_nonreentrant_lock_(*SRWLock))
BOOLEAN
WINAPI
TryAcquireSRWLockExclusive(
    _Inout_ PSRWLOCK SRWLock
    );

WINBASEAPI
_When_(return!=0, _Acquires_shared_lock_(*SRWLock))
_When_(return!=0, _Acquires_nonreentrant_lock_(*SRWLock))
BOOLEAN
WINAPI
TryAcquireSRWLockShared(
    _Inout_ PSRWLOCK SRWLock
    );

#endif // (_WIN32_WINNT >= 0x0600)

#if (_WIN32_WINNT < 0x0600)

_Maybe_raises_SEH_exception_
WINBASEAPI
VOID
WINAPI
InitializeCriticalSection(
    _Out_ LPCRITICAL_SECTION lpCriticalSection
    );

#else

WINBASEAPI
VOID
WINAPI
InitializeCriticalSection(
    _Out_ LPCRITICAL_SECTION lpCriticalSection
    );

#endif  // (_WIN32_WINNT < 0x0600)

WINBASEAPI
VOID
WINAPI
EnterCriticalSection(
    _Inout_ LPCRITICAL_SECTION lpCriticalSection
    );

WINBASEAPI
VOID
WINAPI
LeaveCriticalSection(
    _Inout_ LPCRITICAL_SECTION lpCriticalSection
    );

WINBASEAPI
_Must_inspect_result_
BOOL
WINAPI
InitializeCriticalSectionAndSpinCount(
    _Out_ LPCRITICAL_SECTION lpCriticalSection,
    _In_ DWORD dwSpinCount
    );

#if (_WIN32_WINNT >= 0x0600)

WINBASEAPI
BOOL
WINAPI
InitializeCriticalSectionEx(
    _Out_ LPCRITICAL_SECTION lpCriticalSection,
    _In_ DWORD dwSpinCount,
    _In_ DWORD Flags
    );

#endif // (_WIN32_WINNT >= 0x0600)

WINBASEAPI
DWORD
WINAPI
SetCriticalSectionSpinCount(
    _Inout_ LPCRITICAL_SECTION lpCriticalSection,
    _In_ DWORD dwSpinCount
    );

#if (_WIN32_WINNT >= 0x0400)

WINBASEAPI
BOOL
WINAPI
TryEnterCriticalSection(
    _Inout_ LPCRITICAL_SECTION lpCriticalSection
    );

#endif /* _WIN32_WINNT >= 0x0400 */

WINBASEAPI
VOID
WINAPI
DeleteCriticalSection(
    _Inout_ LPCRITICAL_SECTION lpCriticalSection
    );

//
// Define one-time initialization primitive
//

typedef RTL_RUN_ONCE INIT_ONCE;
typedef PRTL_RUN_ONCE PINIT_ONCE;
typedef PRTL_RUN_ONCE LPINIT_ONCE;

#define INIT_ONCE_STATIC_INIT   RTL_RUN_ONCE_INIT

//
// Run once flags
//

#define INIT_ONCE_CHECK_ONLY        RTL_RUN_ONCE_CHECK_ONLY
#define INIT_ONCE_ASYNC             RTL_RUN_ONCE_ASYNC
#define INIT_ONCE_INIT_FAILED       RTL_RUN_ONCE_INIT_FAILED

//
// The context stored in the run once structure must leave the following number
// of low order bits unused.
//

#define INIT_ONCE_CTX_RESERVED_BITS RTL_RUN_ONCE_CTX_RESERVED_BITS

typedef
BOOL
(WINAPI *PINIT_ONCE_FN) (
    _Inout_ PINIT_ONCE InitOnce,
    _Inout_opt_ PVOID Parameter,
    _Outptr_opt_result_maybenull_ PVOID *Context
    );

#if (_WIN32_WINNT >= 0x0600)

WINBASEAPI
VOID
WINAPI
InitOnceInitialize(
    _Out_ PINIT_ONCE InitOnce
    );

WINBASEAPI
BOOL
WINAPI
InitOnceExecuteOnce(
    _Inout_ PINIT_ONCE InitOnce,
    _In_ __callback PINIT_ONCE_FN InitFn,
    _Inout_opt_ PVOID Parameter,
    _Outptr_opt_result_maybenull_ LPVOID* Context
    );

WINBASEAPI
BOOL
WINAPI
InitOnceBeginInitialize(
    _Inout_ LPINIT_ONCE lpInitOnce,
    _In_ DWORD dwFlags,
    _Out_ PBOOL fPending,
    _Outptr_opt_result_maybenull_ LPVOID* lpContext
    );

WINBASEAPI
BOOL
WINAPI
InitOnceComplete(
    _Inout_ LPINIT_ONCE lpInitOnce,
    _In_ DWORD dwFlags,
    _In_opt_ LPVOID lpContext
    );

#endif // (_WIN32_WINNT >= 0x0600)

//
// Define condition variable
//

typedef RTL_CONDITION_VARIABLE CONDITION_VARIABLE, *PCONDITION_VARIABLE;

//
// Static initializer for the condition variable
//

#define CONDITION_VARIABLE_INIT RTL_CONDITION_VARIABLE_INIT

//
// Flags for condition variables
//

#define CONDITION_VARIABLE_LOCKMODE_SHARED RTL_CONDITION_VARIABLE_LOCKMODE_SHARED

#if (_WIN32_WINNT >= 0x0600)

WINBASEAPI
VOID
WINAPI
InitializeConditionVariable(
    _Out_ PCONDITION_VARIABLE ConditionVariable
    );

WINBASEAPI
VOID
WINAPI
WakeConditionVariable(
    _Inout_ PCONDITION_VARIABLE ConditionVariable
    );

WINBASEAPI
VOID
WINAPI
WakeAllConditionVariable(
    _Inout_ PCONDITION_VARIABLE ConditionVariable
    );

WINBASEAPI
BOOL
WINAPI
SleepConditionVariableCS(
    _Inout_ PCONDITION_VARIABLE ConditionVariable,
    _Inout_ PCRITICAL_SECTION CriticalSection,
    _In_ DWORD dwMilliseconds
    );

WINBASEAPI
BOOL
WINAPI
SleepConditionVariableSRW(
    _Inout_ PCONDITION_VARIABLE ConditionVariable,
    _Inout_ PSRWLOCK SRWLock,
    _In_ DWORD dwMilliseconds,
    _In_ ULONG Flags
    );

#endif // (_WIN32_WINNT >= 0x0600)

WINBASEAPI
BOOL
WINAPI
SetEvent(
    _In_ HANDLE hEvent
    );

WINBASEAPI
BOOL
WINAPI
ResetEvent(
    _In_ HANDLE hEvent
    );

WINBASEAPI
BOOL
WINAPI
ReleaseSemaphore(
    _In_ HANDLE hSemaphore,
    _In_ LONG lReleaseCount,
    _Out_opt_ LPLONG lpPreviousCount
    );

WINBASEAPI
BOOL
WINAPI
ReleaseMutex(
    _In_ HANDLE hMutex
    );

WINBASEAPI
DWORD
WINAPI
WaitForSingleObject(
    _In_ HANDLE hHandle,
    _In_ DWORD dwMilliseconds
    );

WINBASEAPI
DWORD
WINAPI
SleepEx(
    _In_ DWORD dwMilliseconds,
    _In_ BOOL bAlertable
    );

WINBASEAPI
DWORD
WINAPI
WaitForSingleObjectEx(
    _In_ HANDLE hHandle,
    _In_ DWORD dwMilliseconds,
    _In_ BOOL bAlertable
    );

WINBASEAPI
DWORD
WINAPI
WaitForMultipleObjectsEx(
    _In_ DWORD nCount,
    _In_reads_(nCount) CONST HANDLE* lpHandles,
    _In_ BOOL bWaitAll,
    _In_ DWORD dwMilliseconds,
    _In_ BOOL bAlertable
    );

//
// Synchronization APIs
//

#define MUTEX_MODIFY_STATE  MUTANT_QUERY_STATE
#define MUTEX_ALL_ACCESS    MUTANT_ALL_ACCESS

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
CreateMutexA(
    _In_opt_ LPSECURITY_ATTRIBUTES lpMutexAttributes,
    _In_ BOOL bInitialOwner,
    _In_opt_ LPCSTR lpName
    );

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
CreateMutexW(
    _In_opt_ LPSECURITY_ATTRIBUTES lpMutexAttributes,
    _In_ BOOL bInitialOwner,
    _In_opt_ LPCWSTR lpName
    );

#ifdef UNICODE
#define CreateMutex  CreateMutexW
#else
#define CreateMutex  CreateMutexA
#endif // !UNICODE

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
OpenMutexW(
    _In_ DWORD dwDesiredAccess,
    _In_ BOOL bInheritHandle,
    _In_ LPCWSTR lpName
    );

#ifdef UNICODE
#define OpenMutex  OpenMutexW
#endif

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
CreateEventA(
    _In_opt_ LPSECURITY_ATTRIBUTES lpEventAttributes,
    _In_ BOOL bManualReset,
    _In_ BOOL bInitialState,
    _In_opt_ LPCSTR lpName
    );

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
CreateEventW(
    _In_opt_ LPSECURITY_ATTRIBUTES lpEventAttributes,
    _In_ BOOL bManualReset,
    _In_ BOOL bInitialState,
    _In_opt_ LPCWSTR lpName
    );

#ifdef UNICODE
#define CreateEvent  CreateEventW
#else
#define CreateEvent  CreateEventA
#endif // !UNICODE

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
OpenEventA(
    _In_ DWORD dwDesiredAccess,
    _In_ BOOL bInheritHandle,
    _In_ LPCSTR lpName
    );

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
OpenEventW(
    _In_ DWORD dwDesiredAccess,
    _In_ BOOL bInheritHandle,
    _In_ LPCWSTR lpName
    );

#ifdef UNICODE
#define OpenEvent  OpenEventW
#else
#define OpenEvent  OpenEventA
#endif // !UNICODE

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
OpenSemaphoreW(
    _In_ DWORD dwDesiredAccess,
    _In_ BOOL bInheritHandle,
    _In_ LPCWSTR lpName
    );

#ifdef UNICODE
#define OpenSemaphore  OpenSemaphoreW
#endif

#if (_WIN32_WINNT >= 0x0400) || (_WIN32_WINDOWS > 0x0400)

typedef
VOID
(APIENTRY *PTIMERAPCROUTINE)(
    _In_opt_ LPVOID lpArgToCompletionRoutine,
    _In_     DWORD dwTimerLowValue,
    _In_     DWORD dwTimerHighValue
    );

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
OpenWaitableTimerW(
    _In_ DWORD dwDesiredAccess,
    _In_ BOOL bInheritHandle,
    _In_ LPCWSTR lpTimerName
    );

#ifdef UNICODE
#define OpenWaitableTimer  OpenWaitableTimerW
#endif

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)

BOOL
WINAPI
SetWaitableTimerEx(
    _In_ HANDLE hTimer,
    _In_ const LARGE_INTEGER* lpDueTime,
    _In_ LONG lPeriod,
    _In_opt_ PTIMERAPCROUTINE pfnCompletionRoutine,
    _In_opt_ LPVOID lpArgToCompletionRoutine,
    _In_opt_ PREASON_CONTEXT WakeContext,
    _In_ ULONG TolerableDelay
    );

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN7)

WINBASEAPI
BOOL
WINAPI
SetWaitableTimer(
    _In_ HANDLE hTimer,
    _In_ const LARGE_INTEGER* lpDueTime,
    _In_ LONG lPeriod,
    _In_opt_ PTIMERAPCROUTINE pfnCompletionRoutine,
    _In_opt_ LPVOID lpArgToCompletionRoutine,
    _In_ BOOL fResume
    );

WINBASEAPI
BOOL
WINAPI
CancelWaitableTimer(
    _In_ HANDLE hTimer
    );

#if (_WIN32_WINNT >= 0x0600)

#define CREATE_MUTEX_INITIAL_OWNER  0x00000001

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
CreateMutexExA(
    _In_opt_ LPSECURITY_ATTRIBUTES lpMutexAttributes,
    _In_opt_ LPCSTR lpName,
    _In_ DWORD dwFlags,
    _In_ DWORD dwDesiredAccess
    );

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
CreateMutexExW(
    _In_opt_ LPSECURITY_ATTRIBUTES lpMutexAttributes,
    _In_opt_ LPCWSTR lpName,
    _In_ DWORD dwFlags,
    _In_ DWORD dwDesiredAccess
    );

#ifdef UNICODE
#define CreateMutexEx  CreateMutexExW
#else
#define CreateMutexEx  CreateMutexExA
#endif // !UNICODE

#define CREATE_EVENT_MANUAL_RESET   0x00000001
#define CREATE_EVENT_INITIAL_SET    0x00000002

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
CreateEventExA(
    _In_opt_ LPSECURITY_ATTRIBUTES lpEventAttributes,
    _In_opt_ LPCSTR lpName,
    _In_ DWORD dwFlags,
    _In_ DWORD dwDesiredAccess
    );

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
CreateEventExW(
    _In_opt_ LPSECURITY_ATTRIBUTES lpEventAttributes,
    _In_opt_ LPCWSTR lpName,
    _In_ DWORD dwFlags,
    _In_ DWORD dwDesiredAccess
    );

#ifdef UNICODE
#define CreateEventEx  CreateEventExW
#else
#define CreateEventEx  CreateEventExA
#endif // !UNICODE

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
CreateSemaphoreExW(
    _In_opt_ LPSECURITY_ATTRIBUTES lpSemaphoreAttributes,
    _In_ LONG lInitialCount,
    _In_ LONG lMaximumCount,
    _In_opt_ LPCWSTR lpName,
    _Reserved_ DWORD dwFlags,
    _In_ DWORD dwDesiredAccess
    );

#ifdef UNICODE
#define CreateSemaphoreEx  CreateSemaphoreExW
#endif

#define CREATE_WAITABLE_TIMER_MANUAL_RESET  0x00000001
#if (_WIN32_WINNT >= _NT_TARGET_VERSION_WIN10_RS4)
#define CREATE_WAITABLE_TIMER_HIGH_RESOLUTION 0x00000002
#endif

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
CreateWaitableTimerExW(
    _In_opt_ LPSECURITY_ATTRIBUTES lpTimerAttributes,
    _In_opt_ LPCWSTR lpTimerName,
    _In_ DWORD dwFlags,
    _In_ DWORD dwDesiredAccess
    );

#ifdef UNICODE
#define CreateWaitableTimerEx  CreateWaitableTimerExW
#endif

#endif // (_WIN32_WINNT >= 0x0600)

#endif // (_WIN32_WINNT >= 0x0400) || (_WIN32_WINDOWS > 0x0400)

typedef RTL_BARRIER SYNCHRONIZATION_BARRIER;
typedef PRTL_BARRIER PSYNCHRONIZATION_BARRIER;
typedef PRTL_BARRIER LPSYNCHRONIZATION_BARRIER;

#define SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY  0x01
#define SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY 0x02
#define SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE  0x04

BOOL
WINAPI
EnterSynchronizationBarrier(
    _Inout_ LPSYNCHRONIZATION_BARRIER lpBarrier,
    _In_ DWORD dwFlags
    );

BOOL
WINAPI
InitializeSynchronizationBarrier(
    _Out_ LPSYNCHRONIZATION_BARRIER lpBarrier,
    _In_ LONG lTotalThreads,
    _In_ LONG lSpinCount
    );

BOOL
WINAPI
DeleteSynchronizationBarrier(
    _Inout_ LPSYNCHRONIZATION_BARRIER lpBarrier
    );

WINBASEAPI
VOID
WINAPI
Sleep(
    _In_ DWORD dwMilliseconds
    );

BOOL
WINAPI
WaitOnAddress(
    _In_reads_bytes_(AddressSize) volatile VOID* Address,
    _In_reads_bytes_(AddressSize) PVOID CompareAddress,
    _In_ SIZE_T AddressSize,
    _In_opt_ DWORD dwMilliseconds
    );

VOID
WINAPI
WakeByAddressSingle(
    _In_ PVOID Address
    );

VOID
WINAPI
WakeByAddressAll(
    _In_ PVOID Address
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifndef MIDL_PASS

WINBASEAPI
DWORD
WINAPI
SignalObjectAndWait(
    _In_ HANDLE hObjectToSignal,
    _In_ HANDLE hObjectToWaitOn,
    _In_ DWORD dwMilliseconds,
    _In_ BOOL bAlertable
    );

#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application or OneCore Family or Games Partition
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
DWORD
WINAPI
WaitForMultipleObjects(
    _In_ DWORD nCount,
    _In_reads_(nCount) CONST HANDLE* lpHandles,
    _In_ BOOL bWaitAll,
    _In_ DWORD dwMilliseconds
    );

WINBASEAPI
HANDLE
WINAPI
CreateSemaphoreW(
    _In_opt_ LPSECURITY_ATTRIBUTES lpSemaphoreAttributes,
    _In_ LONG lInitialCount,
    _In_ LONG lMaximumCount,
    _In_opt_ LPCWSTR lpName
    );

#ifdef UNICODE
#define CreateSemaphore  CreateSemaphoreW
#endif

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
CreateWaitableTimerW(
    _In_opt_ LPSECURITY_ATTRIBUTES lpTimerAttributes,
    _In_ BOOL bManualReset,
    _In_opt_ LPCWSTR lpTimerName
    );

#ifdef UNICODE
#define CreateWaitableTimer  CreateWaitableTimerW
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _SYNCHAPI_H_
