/********************************************************************************
*                                                                               *
* fibersapi.h - ApiSet Contract for api-ms-win-core-fibers-l1                   *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _FIBERS_H_
#define _FIBERS_H_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (_WIN32_WINNT >= 0x0600)

#ifndef FLS_OUT_OF_INDEXES
#define FLS_OUT_OF_INDEXES ((DWORD)0xFFFFFFFF)
#endif

WINBASEAPI
DWORD
WINAPI
FlsAlloc(
    _In_opt_ PFLS_CALLBACK_FUNCTION lpCallback
    );

WINBASEAPI
PVOID
WINAPI
FlsGetValue(
    _In_ DWORD dwFlsIndex
    );

WINBASEAPI
BOOL
WINAPI
FlsSetValue(
    _In_ DWORD dwFlsIndex,
    _In_opt_ PVOID lpFlsData
    );

WINBASEAPI
BOOL
WINAPI
FlsFree(
    _In_ DWORD dwFlsIndex
    );

#endif // (_WIN32_WINNT >= 0x0600)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (_WIN32_WINNT >= 0x0600)

WINBASEAPI
BOOL
WINAPI
IsThreadAFiber(
    VOID
    );

#endif // (_WIN32_WINNT >= 0x0600)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10)

WINBASEAPI
PVOID
WINAPI
FlsGetValue2(
    _In_ DWORD dwTlsIndex
    );

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN10)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _FIBERS_H_
