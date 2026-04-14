#include <winapifamily.h>

/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    avrt.h

Abstract:

    This module contains the multimedia class scheduler APIs and any public data
    structures needed to call these APIs.

--*/

#ifndef _AVRT_H_
#define _AVRT_H_
#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop and Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

//
// AvRt Priorities
//

typedef enum _AVRT_PRIORITY
{
    AVRT_PRIORITY_VERYLOW = -2,
    AVRT_PRIORITY_LOW,
    AVRT_PRIORITY_NORMAL,
    AVRT_PRIORITY_HIGH,
    AVRT_PRIORITY_CRITICAL
} AVRT_PRIORITY, *PAVRT_PRIORITY;

//
//  Infinite timeout for a thread order group.
//

#define THREAD_ORDER_GROUP_INFINITE_TIMEOUT     (-1I64)

//
// Define API decoration for direct importing of DLL references.
//

#define AVRTAPI DECLSPEC_IMPORT

_Success_(return != NULL)
AVRTAPI
HANDLE
WINAPI
AvSetMmThreadCharacteristicsA (
    _In_ LPCSTR TaskName,
    _Inout_ LPDWORD TaskIndex
    );
_Success_(return != NULL)
AVRTAPI
HANDLE
WINAPI
AvSetMmThreadCharacteristicsW (
    _In_ LPCWSTR TaskName,
    _Inout_ LPDWORD TaskIndex
    );
#ifdef UNICODE
#define AvSetMmThreadCharacteristics  AvSetMmThreadCharacteristicsW
#else
#define AvSetMmThreadCharacteristics  AvSetMmThreadCharacteristicsA
#endif // !UNICODE

_Success_(return != NULL)
AVRTAPI
HANDLE
WINAPI
AvSetMmMaxThreadCharacteristicsA (
    _In_ LPCSTR FirstTask,
    _In_ LPCSTR SecondTask,
    _Inout_ LPDWORD TaskIndex
    );
_Success_(return != NULL)
AVRTAPI
HANDLE
WINAPI
AvSetMmMaxThreadCharacteristicsW (
    _In_ LPCWSTR FirstTask,
    _In_ LPCWSTR SecondTask,
    _Inout_ LPDWORD TaskIndex
    );
#ifdef UNICODE
#define AvSetMmMaxThreadCharacteristics  AvSetMmMaxThreadCharacteristicsW
#else
#define AvSetMmMaxThreadCharacteristics  AvSetMmMaxThreadCharacteristicsA
#endif // !UNICODE

_Success_(return != FALSE)
AVRTAPI
BOOL
WINAPI
AvRevertMmThreadCharacteristics (
    _In_ HANDLE AvrtHandle
    );

_Success_(return != FALSE)
AVRTAPI
BOOL
WINAPI
AvSetMmThreadPriority (
    _In_ HANDLE AvrtHandle,
    _In_ AVRT_PRIORITY Priority
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

_Success_(return != FALSE)
AVRTAPI
BOOL
WINAPI
AvRtCreateThreadOrderingGroup (
    _Out_ PHANDLE Context,
    _In_ PLARGE_INTEGER Period,
    _Inout_ GUID *ThreadOrderingGuid,
    _In_opt_ PLARGE_INTEGER Timeout
    );

_Success_(return != FALSE)
AVRTAPI
BOOL
WINAPI
AvRtCreateThreadOrderingGroupExA (
    _Out_ PHANDLE Context,
    _In_ PLARGE_INTEGER Period,
    _Inout_ GUID *ThreadOrderingGuid,
    _In_opt_ PLARGE_INTEGER Timeout,
    _In_ LPCSTR TaskName
    );
_Success_(return != FALSE)
AVRTAPI
BOOL
WINAPI
AvRtCreateThreadOrderingGroupExW (
    _Out_ PHANDLE Context,
    _In_ PLARGE_INTEGER Period,
    _Inout_ GUID *ThreadOrderingGuid,
    _In_opt_ PLARGE_INTEGER Timeout,
    _In_ LPCWSTR TaskName
    );
#ifdef UNICODE
#define AvRtCreateThreadOrderingGroupEx  AvRtCreateThreadOrderingGroupExW
#else
#define AvRtCreateThreadOrderingGroupEx  AvRtCreateThreadOrderingGroupExA
#endif // !UNICODE

_Success_(return != FALSE)
AVRTAPI
BOOL
WINAPI
AvRtJoinThreadOrderingGroup (
    _Out_ PHANDLE Context,
    _In_ GUID *ThreadOrderingGuid,
    _In_ BOOL Before
    );

_Success_(return != FALSE)
AVRTAPI
BOOL
WINAPI
AvRtWaitOnThreadOrderingGroup (
    _In_ HANDLE Context
    );

_Success_(return != FALSE)
AVRTAPI
BOOL
WINAPI
AvRtLeaveThreadOrderingGroup (
    _In_ HANDLE Context
    );

_Success_(return != FALSE)
AVRTAPI
BOOL
WINAPI
AvRtDeleteThreadOrderingGroup (
    _In_ HANDLE Context
    );

_Success_(return != FALSE)
AVRTAPI
BOOL
WINAPI
AvQuerySystemResponsiveness (
    _In_ HANDLE AvrtHandle,
    _Out_ PULONG SystemResponsivenessValue
    );


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _AVRT_H_

