/*++ BUILD Version: 0006    // Increment this if a change has global effects

Copyright (c) 1992-1999  Microsoft Corporation

Module Name:

    lmat.h

Abstract:

    This file contains structures, function prototypes, and definitions
    for the schedule service API-s.

Environment:

    User Mode - Win32
    Portable to any flat, 32-bit environment.  (Uses Win32 typedefs.)
    Requires ANSI C extensions: slash-slash comments, long external names.

Notes:

    You must include NETCONS.H before this file, since this file depends
    on values defined in NETCONS.H.

Revision History:

--*/

#ifndef _LMAT_
#define _LMAT_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#ifdef __cplusplus
extern "C" {
#endif

//
//  The following bits are used with Flags field in structures below.
//

//
//  Do we exec programs for this job periodically (/EVERY switch)
//  or one time (/NEXT switch).
//
#define JOB_RUN_PERIODICALLY            0x01    //  set if EVERY


//
//  Was there an error last time we tried to exec a program on behalf of
//  this job.
//  This flag is meaningfull on output only!
//
#define JOB_EXEC_ERROR                  0x02    //  set if error

//
//  Will this job run today or tomorrow.
//  This flag is meaningfull on output only!
//
#define JOB_RUNS_TODAY                  0x04    //  set if today

//
//  Add current day of the month to DaysOfMonth input.
//  This flag is meaningfull on input only!
//
#define JOB_ADD_CURRENT_DATE            0x08    // set if to add current date


//
//  Will this job be run interactively or not.  Windows NT 3.1 do not
//  know about this bit, i.e. they submit interactive jobs only.
//
#define JOB_NONINTERACTIVE              0x10    // set for noninteractive


#define JOB_INPUT_FLAGS     (   JOB_RUN_PERIODICALLY        |   \
                                JOB_ADD_CURRENT_DATE        |   \
                                JOB_NONINTERACTIVE  )

#define JOB_OUTPUT_FLAGS    (   JOB_RUN_PERIODICALLY        |   \
                                JOB_EXEC_ERROR              |   \
                                JOB_RUNS_TODAY              |   \
                                JOB_NONINTERACTIVE  )



typedef struct _AT_INFO {
    DWORD_PTR   JobTime;
    DWORD   DaysOfMonth;
    UCHAR   DaysOfWeek;
    UCHAR   Flags;
    LPWSTR  Command;
} AT_INFO, *PAT_INFO, *LPAT_INFO;

typedef struct _AT_ENUM {
    DWORD   JobId;
    DWORD_PTR   JobTime;
    DWORD   DaysOfMonth;
    UCHAR   DaysOfWeek;
    UCHAR   Flags;
    LPWSTR  Command;
} AT_ENUM, *PAT_ENUM, *LPAT_ENUM;

NET_API_STATUS NET_API_FUNCTION
NetScheduleJobAdd(
    IN      LPCWSTR         Servername  OPTIONAL,
    IN      LPBYTE          Buffer,
    OUT     LPDWORD         JobId
    );

NET_API_STATUS NET_API_FUNCTION
NetScheduleJobDel(
    IN      LPCWSTR         Servername  OPTIONAL,
    IN      DWORD           MinJobId,
    IN      DWORD           MaxJobId
    );

NET_API_STATUS NET_API_FUNCTION
NetScheduleJobEnum(
    IN      LPCWSTR         Servername              OPTIONAL,
    OUT     LPBYTE *        PointerToBuffer,
    IN      DWORD           PrefferedMaximumLength,
    OUT     LPDWORD         EntriesRead,
    OUT     LPDWORD         TotalEntries,
    IN OUT  LPDWORD         ResumeHandle
    );

NET_API_STATUS NET_API_FUNCTION
NetScheduleJobGetInfo(
    IN      LPCWSTR         Servername              OPTIONAL,
    IN      DWORD           JobId,
    OUT     LPBYTE *        PointerToBuffer
    );

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // _LMAT_
