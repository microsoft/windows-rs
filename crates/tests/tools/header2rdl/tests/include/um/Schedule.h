/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) 1997-1999 Microsoft Corporation

Module Name:

    schedule.h

Abstract:

    This file defines a common schedule structure for use by various NT
    components.

--*/

#ifndef _SCHEDULE_H_
#define _SCHEDULE_H_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

//
// The DS and FRS use the same structure to represent different schedules.
// The DS uses a 15-minute polling schedule. FRS uses a 60-minute
// start/stop schedule. Hence, the schedule for the system volume is
// a special case because we only have the DS schedule to work from.
// We will work around this problem by treating the 15
// -minute polling schedule as a 60-minute start/stop schedule.
// Replication is enabled for any hour that has any of the four
// 15-minute bits set.
//
// When the ReplicationSchedule is not present the default is
// "always replicate."
//

//
// Only the interval schedule is currently implemented. Others are ignored.
//
#define SCHEDULE_INTERVAL       0 // schedule as understood by NT5
#define SCHEDULE_BANDWIDTH      1 // bandwidth as understood by NT5
#define SCHEDULE_PRIORITY       2 // priority as understood by NT5

//
// Schedule Header
//
// Each schedule blob begins with n array of schedule headers that
// specify the number and type of schedules contained in the blob.
//
typedef struct _SCHEDULE_HEADER {
    ULONG   Type;       // one of the SCHEDULE_ ordinals
    ULONG   Offset;     // offset from start of schedule structure
} SCHEDULE_HEADER, *PSCHEDULE_HEADER;

//
// Schedule
//
typedef struct _SCHEDULE {
    ULONG           Size;           // inclusive size in bytes
    ULONG           Bandwidth;
    ULONG           NumberOfSchedules;
    SCHEDULE_HEADER Schedules[1];
} SCHEDULE, *PSCHEDULE;
// The above structure is followed by the Data buffer and the
// SCHEDULE_HEADER contains offsets to refer to the appropriate
// parts in the data buffer.

#define SCHEDULE_DATA_ENTRIES   (7 * 24)    // 7 days X 24 hours

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // _SCHEDULE_H_

