/********************************************************************************
*                                                                               *
* interlockedapi.h -- ApiSet Contract for api-ms-win-core-interlocked-l1        *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _INTERLOCKAPI_H_
#define _INTERLOCKAPI_H_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Application Family or OneCore Family

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#ifndef NOWINBASEINTERLOCK

#ifndef _NTOS_

#endif /* _NTOS_ */

#endif /* NOWINBASEINTERLOCK */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifndef NOWINBASEINTERLOCK

#ifndef _NTOS_

#if defined(_SLIST_HEADER_) && !defined(_NTOSP_)

WINBASEAPI
VOID
WINAPI
InitializeSListHead(
    _Out_ PSLIST_HEADER ListHead
    );

WINBASEAPI
PSLIST_ENTRY
WINAPI
InterlockedPopEntrySList(
    _Inout_ PSLIST_HEADER ListHead
    );

WINBASEAPI
PSLIST_ENTRY
WINAPI
InterlockedPushEntrySList(
    _Inout_ PSLIST_HEADER ListHead,
    _Inout_ __drv_aliasesMem PSLIST_ENTRY ListEntry
    );

#if (NTDDI_VERSION >= NTDDI_WIN8)

#define InterlockedPushListSList InterlockedPushListSListEx

WINBASEAPI
PSLIST_ENTRY
WINAPI
InterlockedPushListSListEx(
    _Inout_ PSLIST_HEADER ListHead,
    _Inout_ PSLIST_ENTRY List,
    _Inout_ PSLIST_ENTRY ListEnd,
    _In_ ULONG Count
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

WINBASEAPI
PSLIST_ENTRY
WINAPI
InterlockedFlushSList(
    _Inout_ PSLIST_HEADER ListHead
    );

WINBASEAPI
USHORT
WINAPI
QueryDepthSList(
    _In_ PSLIST_HEADER ListHead
    );

#endif /* SLIST_HEADER_ */

#endif /* _NTOS_ */

#endif /* NOWINBASEINTERLOCK */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _INTERLOCKAPI_H_
