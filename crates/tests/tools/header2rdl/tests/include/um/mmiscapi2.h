/********************************************************************************
*                                                                               *
* mmiscapi2.h -- ApiSet Contract for api-ms-win-mm-misc-l2-1-0                  *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _MMISCAPI2_H_
#define _MMISCAPI2_H_

#include <apiset.h>
#include <apisetcconv.h>

#include <mmsyscom.h> // mm common definitions

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef void (CALLBACK TIMECALLBACK)(UINT uTimerID, UINT uMsg, DWORD_PTR dwUser, DWORD_PTR dw1, DWORD_PTR dw2);
typedef TIMECALLBACK FAR *LPTIMECALLBACK;

/* flags for fuEvent parameter of timeSetEvent() function */
#define TIME_ONESHOT    0x0000   /* program timer for single event */
#define TIME_PERIODIC   0x0001   /* program for continuous periodic event */

#ifdef _WIN32
#define TIME_CALLBACK_FUNCTION      0x0000  /* callback is function */
#define TIME_CALLBACK_EVENT_SET     0x0010  /* callback is event - use SetEvent */
#define TIME_CALLBACK_EVENT_PULSE   0x0020  /* callback is event - use PulseEvent */
#endif

#if WINVER >= 0x0501
#define TIME_KILL_SYNCHRONOUS   0x0100  /* This flag prevents the event from occurring */
                                        /* after the user calls timeKillEvent() to */
                                        /* destroy it. */
#endif // WINVER >= 0x0501

WINMMAPI
MMRESULT
WINAPI
timeSetEvent(
    _In_ UINT uDelay,
    _In_ UINT uResolution,
    _In_ LPTIMECALLBACK fptc,
    _In_ DWORD_PTR dwUser,
    _In_ UINT fuEvent
    );

WINMMAPI
MMRESULT
WINAPI
timeKillEvent(
    _In_ UINT uTimerID
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _MMISCAPI2_H_

