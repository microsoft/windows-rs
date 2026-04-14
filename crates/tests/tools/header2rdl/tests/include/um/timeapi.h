/********************************************************************************
*                                                                               *
* timerapi.h -- ApiSet Contract for api-ms-win-mm-time-l1-1-0                   *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _TIMERAPI_H_
#define _TIMERAPI_H_

#include <apiset.h>
#include <apisetcconv.h>

#include <mmsyscom.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifndef MMNOTIMER
/****************************************************************************

                            Timer support

****************************************************************************/

/* timer error return values */
#define TIMERR_NOERROR        (0)                  /* no error */
#define TIMERR_NOCANDO        (TIMERR_BASE+1)      /* request not completed */
#define TIMERR_STRUCT         (TIMERR_BASE+33)     /* time struct size */

/* timer device capabilities data structure */
typedef struct timecaps_tag {
    UINT    wPeriodMin;     /* minimum period supported  */
    UINT    wPeriodMax;     /* maximum period supported  */
} TIMECAPS, *PTIMECAPS, NEAR *NPTIMECAPS, FAR *LPTIMECAPS;

/* timer function prototypes */
WINMMAPI
MMRESULT
WINAPI
timeGetSystemTime(
    _Out_writes_bytes_(cbmmt) LPMMTIME pmmt,
    _In_ UINT cbmmt
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
WINMMAPI
DWORD
WINAPI
timeGetTime(
    void
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
WINMMAPI
MMRESULT
WINAPI
timeGetDevCaps(
    _Out_writes_bytes_(cbtc) LPTIMECAPS ptc,
    _In_ UINT cbtc
    );

WINMMAPI
MMRESULT
WINAPI
timeBeginPeriod(
    _In_ UINT uPeriod
    );

WINMMAPI
MMRESULT
WINAPI
timeEndPeriod(
    _In_ UINT uPeriod
    );

#endif  /* ifndef MMNOTIMER */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _TIMERAPI_H_

