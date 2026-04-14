/********************************************************************************
*                                                                               *
* threadpoolapi.h -- ApiSet Contract for api-ms-win-core-threadpool-l1          *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _THREADPOOLAPISET_H_
#define _THREADPOOLAPISET_H_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#ifdef __cplusplus
extern "C" {
#endif

//
// Thread pool API's
//

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

typedef
VOID
(WINAPI *PTP_WIN32_IO_CALLBACK)(
    _Inout_     PTP_CALLBACK_INSTANCE Instance,
    _Inout_opt_ PVOID                 Context,
    _Inout_opt_ PVOID                 Overlapped,
    _In_        ULONG                 IoResult,
    _In_        ULONG_PTR             NumberOfBytesTransferred,
    _Inout_     PTP_IO                Io
    );

#if (_WIN32_WINNT >= 0x0600)

WINBASEAPI
_Must_inspect_result_
PTP_POOL
WINAPI
CreateThreadpool(
    _Reserved_ PVOID reserved
    );

WINBASEAPI
VOID
WINAPI
SetThreadpoolThreadMaximum(
    _Inout_ PTP_POOL ptpp,
    _In_ DWORD cthrdMost
    );

WINBASEAPI
BOOL
WINAPI
SetThreadpoolThreadMinimum(
    _Inout_ PTP_POOL ptpp,
    _In_ DWORD cthrdMic
    );

WINBASEAPI
BOOL
WINAPI
SetThreadpoolStackInformation(
    _Inout_ PTP_POOL ptpp,
    _In_ PTP_POOL_STACK_INFORMATION ptpsi
    );

WINBASEAPI
BOOL
WINAPI
QueryThreadpoolStackInformation(
    _In_ PTP_POOL ptpp,
    _Out_ PTP_POOL_STACK_INFORMATION ptpsi
    );

WINBASEAPI
VOID
WINAPI
CloseThreadpool(
    _Inout_ PTP_POOL ptpp
    );

WINBASEAPI
_Must_inspect_result_
PTP_CLEANUP_GROUP
WINAPI
CreateThreadpoolCleanupGroup(
    VOID
    );

WINBASEAPI
VOID
WINAPI
CloseThreadpoolCleanupGroupMembers(
    _Inout_ PTP_CLEANUP_GROUP ptpcg,
    _In_ BOOL fCancelPendingCallbacks,
    _Inout_opt_ PVOID pvCleanupContext
    );

WINBASEAPI
VOID
WINAPI
CloseThreadpoolCleanupGroup(
    _Inout_ PTP_CLEANUP_GROUP ptpcg
    );

WINBASEAPI
VOID
WINAPI
SetEventWhenCallbackReturns(
    _Inout_ PTP_CALLBACK_INSTANCE pci,
    _In_ HANDLE evt
    );

WINBASEAPI
VOID
WINAPI
ReleaseSemaphoreWhenCallbackReturns(
    _Inout_ PTP_CALLBACK_INSTANCE pci,
    _In_ HANDLE sem,
    _In_ DWORD crel
    );

WINBASEAPI
VOID
WINAPI
ReleaseMutexWhenCallbackReturns(
    _Inout_ PTP_CALLBACK_INSTANCE pci,
    _In_ HANDLE mut
    );

WINBASEAPI
VOID
WINAPI
LeaveCriticalSectionWhenCallbackReturns(
    _Inout_ PTP_CALLBACK_INSTANCE pci,
    _Inout_ PCRITICAL_SECTION pcs
    );

WINBASEAPI
VOID
WINAPI
FreeLibraryWhenCallbackReturns(
    _Inout_ PTP_CALLBACK_INSTANCE pci,
    _In_ HMODULE mod
    );

WINBASEAPI
BOOL
WINAPI
CallbackMayRunLong(
    _Inout_ PTP_CALLBACK_INSTANCE pci
    );

WINBASEAPI
VOID
WINAPI
DisassociateCurrentThreadFromCallback(
    _Inout_ PTP_CALLBACK_INSTANCE pci
    );

WINBASEAPI
_Must_inspect_result_
BOOL
WINAPI
TrySubmitThreadpoolCallback(
    _In_ PTP_SIMPLE_CALLBACK pfns,
    _Inout_opt_ PVOID pv,
    _In_opt_ PTP_CALLBACK_ENVIRON pcbe
    );

WINBASEAPI
_Must_inspect_result_
PTP_WORK
WINAPI
CreateThreadpoolWork(
    _In_ PTP_WORK_CALLBACK pfnwk,
    _Inout_opt_ PVOID pv,
    _In_opt_ PTP_CALLBACK_ENVIRON pcbe
    );

WINBASEAPI
VOID
WINAPI
SubmitThreadpoolWork(
    _Inout_ PTP_WORK pwk
    );

WINBASEAPI
VOID
WINAPI
WaitForThreadpoolWorkCallbacks(
    _Inout_ PTP_WORK pwk,
    _In_ BOOL fCancelPendingCallbacks
    );

WINBASEAPI
VOID
WINAPI
CloseThreadpoolWork(
    _Inout_ PTP_WORK pwk
    );

WINBASEAPI
_Must_inspect_result_
PTP_TIMER
WINAPI
CreateThreadpoolTimer(
    _In_ PTP_TIMER_CALLBACK pfnti,
    _Inout_opt_ PVOID pv,
    _In_opt_ PTP_CALLBACK_ENVIRON pcbe
    );

WINBASEAPI
VOID
WINAPI
SetThreadpoolTimer(
    _Inout_ PTP_TIMER pti,
    _In_opt_ PFILETIME pftDueTime,
    _In_ DWORD msPeriod,
    _In_opt_ DWORD msWindowLength
    );

WINBASEAPI
BOOL
WINAPI
IsThreadpoolTimerSet(
    _Inout_ PTP_TIMER pti
    );

WINBASEAPI
VOID
WINAPI
WaitForThreadpoolTimerCallbacks(
    _Inout_ PTP_TIMER pti,
    _In_ BOOL fCancelPendingCallbacks
    );

WINBASEAPI
VOID
WINAPI
CloseThreadpoolTimer(
    _Inout_ PTP_TIMER pti
    );

WINBASEAPI
_Must_inspect_result_
PTP_WAIT
WINAPI
CreateThreadpoolWait(
    _In_ PTP_WAIT_CALLBACK pfnwa,
    _Inout_opt_ PVOID pv,
    _In_opt_ PTP_CALLBACK_ENVIRON pcbe
    );

WINBASEAPI
VOID
WINAPI
SetThreadpoolWait(
    _Inout_ PTP_WAIT pwa,
    _In_opt_ HANDLE h,
    _In_opt_ PFILETIME pftTimeout
    );

WINBASEAPI
VOID
WINAPI
WaitForThreadpoolWaitCallbacks(
    _Inout_ PTP_WAIT pwa,
    _In_ BOOL fCancelPendingCallbacks
    );

WINBASEAPI
VOID
WINAPI
CloseThreadpoolWait(
    _Inout_ PTP_WAIT pwa
    );

WINBASEAPI
_Must_inspect_result_
PTP_IO
WINAPI
CreateThreadpoolIo(
    _In_ HANDLE fl,
    _In_ PTP_WIN32_IO_CALLBACK pfnio,
    _Inout_opt_ PVOID pv,
    _In_opt_ PTP_CALLBACK_ENVIRON pcbe
    );

WINBASEAPI
VOID
WINAPI
StartThreadpoolIo(
    _Inout_ PTP_IO pio
    );

WINBASEAPI
VOID
WINAPI
CancelThreadpoolIo(
    _Inout_ PTP_IO pio
    );

WINBASEAPI
VOID
WINAPI
WaitForThreadpoolIoCallbacks(
    _Inout_ PTP_IO pio,
    _In_ BOOL fCancelPendingCallbacks
    );

WINBASEAPI
VOID
WINAPI
CloseThreadpoolIo(
    _Inout_ PTP_IO pio
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
BOOL
WINAPI
SetThreadpoolTimerEx(
    _Inout_ PTP_TIMER pti,
    _In_opt_ PFILETIME pftDueTime,
    _In_ DWORD msPeriod,
    _In_opt_ DWORD msWindowLength
    );

WINBASEAPI
BOOL
WINAPI
SetThreadpoolWaitEx(
    _Inout_ PTP_WAIT pwa,
    _In_opt_ HANDLE h,
    _In_opt_ PFILETIME pftTimeout,
    _Reserved_ PVOID Reserved
    );

#endif  // _WIN32_WINNT >= 0x0600

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (_WIN32_WINNT >= 0x0600)

#endif  // _WIN32_WINNT >= 0x0600

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _THREADPOOLAPISET_H_
