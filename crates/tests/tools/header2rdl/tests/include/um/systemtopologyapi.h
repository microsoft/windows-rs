/********************************************************************************
*                                                                               *
* consoleapi.h -- ApiSet Contract for api-ms-win-core-systemtopology-l1         *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _SYSTEMTOPOLOGY_H_
#define _SYSTEMTOPOLOGY_H_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
BOOL
WINAPI
GetNumaHighestNodeNumber(
    _Out_ PULONG HighestNodeNumber
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if _WIN32_WINNT >= 0x0601

WINBASEAPI
BOOL
WINAPI
GetNumaNodeProcessorMaskEx(
    _In_ USHORT Node,
    _Out_ PGROUP_AFFINITY ProcessorMask
    );

#endif // (_WIN32_WINNT >=0x0601)

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_FE)

_Success_(return != false)
WINBASEAPI
BOOL
WINAPI
GetNumaNodeProcessorMask2(
    _In_ USHORT NodeNumber,
    _Out_writes_to_opt_(ProcessorMaskCount, *RequiredMaskCount) PGROUP_AFFINITY ProcessorMasks,
    _In_ USHORT ProcessorMaskCount,
    _Out_ PUSHORT RequiredMaskCount
    );

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN10_FE)

#if _WIN32_WINNT >= 0x0601

WINBASEAPI
BOOL
WINAPI
GetNumaProximityNodeEx(
    _In_ ULONG ProximityId,
    _Out_ PUSHORT NodeNumber
    );

#endif // (_WIN32_WINNT >=0x0601)

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _SYSTEMTOPOLOGY_H_
