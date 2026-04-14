/***************************************************************************
*                                                                          *
*   DMusBuff.h -- This module defines the buffer format for DirectMusic    *
*                 Shared file between user mode and kernel mode components *
*                                                                          *
*   Copyright (c) 1998, Microsoft Corp. All rights reserved.               *
*                                                                          *
***************************************************************************/

#ifndef _DMusBuff_
#define _DMusBuff_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include "dmdls.h"

/* Format of DirectMusic events in a buffer
 *
 * A buffer contains 1 or more events, each with the following header.
 * Immediately following the header is the event data. The header+data
 * size is rounded to the nearest quadword (8 bytes).
 */
 
#include <pshpack4.h>                       /* Do not pad at end - that's where the data is */ 
typedef struct _DMUS_EVENTHEADER *LPDMUS_EVENTHEADER;
typedef struct _DMUS_EVENTHEADER
{
    DWORD           cbEvent;                /* Unrounded bytes in event */
    DWORD           dwChannelGroup;         /* Channel group of event */
    REFERENCE_TIME  rtDelta;                /* Delta from start time of entire buffer */
    DWORD           dwFlags;                /* Flags DMUS_EVENT_xxx */
} DMUS_EVENTHEADER;
#include <poppack.h>

#define DMUS_EVENT_STRUCTURED   0x00000001  /* Unstructured data (SysEx, etc.) */

/* The number of bytes to allocate for an event with 'cb' data bytes.
 */ 
#define QWORD_ALIGN(x) (((x) + 7) & ~7)
#define DMUS_EVENT_SIZE(cb) QWORD_ALIGN(sizeof(DMUS_EVENTHEADER) + cb)



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* _DMusBuff_ */
