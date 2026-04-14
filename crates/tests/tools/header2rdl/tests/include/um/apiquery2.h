/********************************************************************************
*                                                                               *
* apiquery2.h -- ApiSet Contract for api-ms-win-core-apiquery-l2                *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _APIQUERY2_H_
#define _APIQUERY2_H_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>

#undef APICONTRACT
#ifdef API_QUERY_IMPORT
#define APICONTRACT DECLSPEC_IMPORT
#else
#define APICONTRACT
#endif

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

APICONTRACT
BOOL
APIENTRY
IsApiSetImplemented(
    _In_ PCSTR Contract
    );

APICONTRACT
HRESULT
APIENTRY
GetApiSetModuleBaseName(
    _In_ PCSTR contractName,
    _In_ UINT32 bufferLength,
    _Out_writes_to_(bufferLength,*actualNameLength) PWSTR moduleBaseName,
    _Out_opt_ UINT32* actualNameLength
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _APIQUERY2_H_
