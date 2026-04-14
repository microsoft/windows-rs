// begin_1_0
/********************************************************************************
*                                                                               *
* timezoneapi.h -- ApiSet Contract for api-ms-win-core-timezone-l1              *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _TIMEZONEAPI_H_
#define _TIMEZONEAPI_H_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#ifdef __cplusplus
extern "C" {
#endif

// end_1_0

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

// begin_1_0

#define TIME_ZONE_ID_INVALID ((DWORD)0xFFFFFFFF)

typedef struct _TIME_ZONE_INFORMATION {
    LONG Bias;
    WCHAR StandardName[ 32 ];
    SYSTEMTIME StandardDate;
    LONG StandardBias;
    WCHAR DaylightName[ 32 ];
    SYSTEMTIME DaylightDate;
    LONG DaylightBias;
} TIME_ZONE_INFORMATION, *PTIME_ZONE_INFORMATION, *LPTIME_ZONE_INFORMATION;

typedef struct _TIME_DYNAMIC_ZONE_INFORMATION {
    LONG Bias;
    WCHAR StandardName[ 32 ];
    SYSTEMTIME StandardDate;
    LONG StandardBias;
    WCHAR DaylightName[ 32 ];
    SYSTEMTIME DaylightDate;
    LONG DaylightBias;
    WCHAR TimeZoneKeyName[ 128 ];
    BOOLEAN DynamicDaylightTimeDisabled;
} DYNAMIC_TIME_ZONE_INFORMATION, *PDYNAMIC_TIME_ZONE_INFORMATION;

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
SystemTimeToTzSpecificLocalTime(
    _In_opt_ CONST TIME_ZONE_INFORMATION* lpTimeZoneInformation,
    _In_ CONST SYSTEMTIME* lpUniversalTime,
    _Out_ LPSYSTEMTIME lpLocalTime
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
TzSpecificLocalTimeToSystemTime(
    _In_opt_ CONST TIME_ZONE_INFORMATION* lpTimeZoneInformation,
    _In_ CONST SYSTEMTIME* lpLocalTime,
    _Out_ LPSYSTEMTIME lpUniversalTime
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
FileTimeToSystemTime(
    _In_ CONST FILETIME* lpFileTime,
    _Out_ LPSYSTEMTIME lpSystemTime
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
SystemTimeToFileTime(
    _In_ CONST SYSTEMTIME* lpSystemTime,
    _Out_ LPFILETIME lpFileTime
    );

WINBASEAPI
_Success_(return != TIME_ZONE_ID_INVALID)
DWORD
WINAPI
GetTimeZoneInformation(
    _Out_ LPTIME_ZONE_INFORMATION lpTimeZoneInformation
    );

WINBASEAPI
BOOL
WINAPI
SetTimeZoneInformation(
    _In_ CONST TIME_ZONE_INFORMATION* lpTimeZoneInformation
    );

#if (_WIN32_WINNT >= 0x0600)

WINBASEAPI
BOOL
WINAPI
SetDynamicTimeZoneInformation(
    _In_ CONST DYNAMIC_TIME_ZONE_INFORMATION* lpTimeZoneInformation
    );

#endif // _WIN32_WINNT >= 0x0600

#if (_WIN32_WINNT >= 0x0600)

WINBASEAPI
_Success_(return != TIME_ZONE_ID_INVALID)
DWORD
WINAPI
GetDynamicTimeZoneInformation(
    _Out_ PDYNAMIC_TIME_ZONE_INFORMATION pTimeZoneInformation
    );

#endif // _WIN32_WINNT >= 0x0600

#if (_WIN32_WINNT >= 0x0601)

_Success_(return != FALSE)
BOOL
WINAPI
GetTimeZoneInformationForYear(
    _In_ USHORT wYear,
    _In_opt_ PDYNAMIC_TIME_ZONE_INFORMATION pdtzi,
    _Out_ LPTIME_ZONE_INFORMATION ptzi
    );

#endif // _WIN32_WINNT >= 0x0601

// end_1_0

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
EnumDynamicTimeZoneInformation(
    _In_ CONST DWORD dwIndex,
    _Out_ PDYNAMIC_TIME_ZONE_INFORMATION lpTimeZoneInformation
    );

WINBASEAPI
_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
GetDynamicTimeZoneInformationEffectiveYears(
    _In_ CONST PDYNAMIC_TIME_ZONE_INFORMATION lpTimeZoneInformation,
    _Out_ LPDWORD FirstYear,
    _Out_ LPDWORD LastYear
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
SystemTimeToTzSpecificLocalTimeEx(
    _In_opt_ CONST DYNAMIC_TIME_ZONE_INFORMATION* lpTimeZoneInformation,
    _In_ CONST SYSTEMTIME* lpUniversalTime,
    _Out_ LPSYSTEMTIME lpLocalTime
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
TzSpecificLocalTimeToSystemTimeEx(
    _In_opt_ CONST DYNAMIC_TIME_ZONE_INFORMATION* lpTimeZoneInformation,
    _In_ CONST SYSTEMTIME* lpLocalTime,
    _Out_ LPSYSTEMTIME lpUniversalTime
    );

#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WIN8) */

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
LocalFileTimeToLocalSystemTime(
    _In_opt_ CONST TIME_ZONE_INFORMATION* timeZoneInformation,
    _In_ CONST FILETIME* localFileTime,
    _Out_ SYSTEMTIME* localSystemTime
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
LocalSystemTimeToLocalFileTime(
    _In_opt_ CONST TIME_ZONE_INFORMATION* timeZoneInformation,
    _In_ CONST SYSTEMTIME* localSystemTime,
    _Out_ FILETIME* localFileTime
    );

#endif /* (NTDDI_VERSION >= NTDDI_WIN10_RS5) */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

// begin_1_0

#ifdef __cplusplus
}
#endif

#endif // _TIMEZONEAPI_H_
// end_1_0
