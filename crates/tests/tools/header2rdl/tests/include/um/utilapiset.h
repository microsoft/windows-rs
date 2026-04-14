/********************************************************************************
*                                                                               *
* UtilApiSet.h -- ApiSet Contract for api-ms-win-core-util-l1-1-0               *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _APISETUTIL_
#define _APISETUTIL_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
_Ret_maybenull_
PVOID
WINAPI
EncodePointer(
    _In_opt_ PVOID Ptr
    );

WINBASEAPI
_Ret_maybenull_
PVOID
WINAPI
DecodePointer(
    _In_opt_ PVOID Ptr
    );

WINBASEAPI
_Ret_maybenull_
PVOID
WINAPI
EncodeSystemPointer(
    _In_opt_ PVOID Ptr
    );

WINBASEAPI
_Ret_maybenull_
PVOID
WINAPI
DecodeSystemPointer(
    _In_opt_ PVOID Ptr
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
HRESULT
WINAPI
EncodeRemotePointer(
    _In_ HANDLE ProcessHandle,
    _In_opt_ PVOID Ptr,
    _Out_ PVOID* EncodedPtr
    );

WINBASEAPI
HRESULT
WINAPI
DecodeRemotePointer(
    _In_ HANDLE ProcessHandle,
    _In_opt_ PVOID Ptr,
    _Out_ PVOID* DecodedPtr
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region PC Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
BOOL
WINAPI
Beep(
    _In_ DWORD dwFreq,
    _In_ DWORD dwDuration
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _APISETUTIL_
