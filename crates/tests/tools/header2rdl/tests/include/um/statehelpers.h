/************************************************************************************
*                                                                                   *
* statehelpers.h -- ApiSet Contract for api-ms-win-core-state-helpers-l1            *
*                                                                                   *
* Copyright (c) Microsoft Corporation. All rights reserved.                         *
*                                                                                   *
************************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _CORESTATEHELPERSAPIS_H_
#define _CORESTATEHELPERSAPIS_H_

#include <apiset.h>
#include <apisetcconv.h>

#ifdef _CONTRACT_GEN
#include <minwindef.h>
#include <minwinbase.h>
#include <windows.h>
#endif // _CONTRACT_GEN

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10)

WINBASEAPI
LSTATUS
WINAPI
GetRegistryValueWithFallbackW(
    __in_opt HKEY hkeyPrimary,
    __in_opt LPCWSTR pwszPrimarySubKey,
    __in_opt HKEY hkeyFallback,
    __in_opt LPCWSTR pwszFallbackSubKey,
    __in PCWSTR pwszValue,
    __in DWORD dwFlags,
    __out_opt LPDWORD pdwType,
    _When_((dwFlags & 0x7F) == RRF_RT_REG_SZ ||
    (dwFlags & 0x7F) == RRF_RT_REG_EXPAND_SZ ||
        (dwFlags & 0x7F) == (RRF_RT_REG_SZ | RRF_RT_REG_EXPAND_SZ) ||
        *pdwType == REG_SZ ||
        *pdwType == REG_EXPAND_SZ, __out_z)
    _When_((dwFlags & 0x7F) == RRF_RT_REG_MULTI_SZ ||
        *pdwType == REG_MULTI_SZ, __post __nullnullterminated) __out_bcount_part_opt(cbDataIn, *pcbDataOut) PVOID pvData,
    __in DWORD cbDataIn,
    __out_opt LPDWORD pcbDataOut
    );

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN10)

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // _CORESTATEHELPERSAPIS_H_
