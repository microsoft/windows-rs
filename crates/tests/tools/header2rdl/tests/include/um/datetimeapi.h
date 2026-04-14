/********************************************************************************
*                                                                               *
* datetimeapi.h -- ApiSet Contract for api-ms-win-core-datetime-l1              *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _DATETIMEAPI_H_
#define _DATETIMEAPI_H_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

// For Windows Vista and above GetDateFormatEx is preferred
WINBASEAPI
int
WINAPI
GetDateFormatA(
    _In_ LCID Locale,
    _In_ DWORD dwFlags,
    _In_opt_ CONST SYSTEMTIME* lpDate,
    _In_opt_ LPCSTR lpFormat,
    _Out_writes_opt_(cchDate) LPSTR lpDateStr,
    _In_ int cchDate
    );

WINBASEAPI
int
WINAPI
GetDateFormatW(
    _In_ LCID Locale,
    _In_ DWORD dwFlags,
    _In_opt_ CONST SYSTEMTIME* lpDate,
    _In_opt_ LPCWSTR lpFormat,
    _Out_writes_opt_(cchDate) LPWSTR lpDateStr,
    _In_ int cchDate
    );

#ifdef UNICODE
#define GetDateFormat  GetDateFormatW
#else
#define GetDateFormat  GetDateFormatA
#endif // !UNICODE

// For Windows Vista and above GetTimeFormatEx is preferred
WINBASEAPI
int
WINAPI
GetTimeFormatA(
    _In_ LCID Locale,
    _In_ DWORD dwFlags,
    _In_opt_ CONST SYSTEMTIME* lpTime,
    _In_opt_ LPCSTR lpFormat,
    _Out_writes_opt_(cchTime) LPSTR lpTimeStr,
    _In_ int cchTime
    );

WINBASEAPI
int
WINAPI
GetTimeFormatW(
    _In_ LCID Locale,
    _In_ DWORD dwFlags,
    _In_opt_ CONST SYSTEMTIME* lpTime,
    _In_opt_ LPCWSTR lpFormat,
    _Out_writes_opt_(cchTime) LPWSTR lpTimeStr,
    _In_ int cchTime
    );

#ifdef UNICODE
#define GetTimeFormat  GetTimeFormatW
#else
#define GetTimeFormat  GetTimeFormatA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
int
WINAPI
GetTimeFormatEx(
    _In_opt_ LPCWSTR lpLocaleName,
    _In_ DWORD dwFlags,
    _In_opt_ CONST SYSTEMTIME* lpTime,
    _In_opt_ LPCWSTR lpFormat,
    _Out_writes_opt_(cchTime) LPWSTR lpTimeStr,
    _In_ int cchTime
    );

WINBASEAPI
int
WINAPI
GetDateFormatEx(
    _In_opt_ LPCWSTR lpLocaleName,
    _In_ DWORD dwFlags,
    _In_opt_ CONST SYSTEMTIME* lpDate,
    _In_opt_ LPCWSTR lpFormat,
    _Out_writes_opt_(cchDate) LPWSTR lpDateStr,
    _In_ int cchDate,
    _In_opt_ LPCWSTR lpCalendar
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#define GetDurationFormatEx_DEFINED

WINBASEAPI
int
WINAPI
GetDurationFormatEx(
    _In_opt_ LPCWSTR lpLocaleName,
    _In_ DWORD dwFlags,
    _In_opt_ CONST SYSTEMTIME* lpDuration,
    _In_ ULONGLONG ullDuration,
    _In_opt_ LPCWSTR lpFormat,
    _Out_writes_opt_(cchDuration) LPWSTR lpDurationStr,
    _In_ int cchDuration
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // DATETIMEAPI

