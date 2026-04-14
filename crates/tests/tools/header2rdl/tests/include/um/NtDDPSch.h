/*++

Copyright (c) 1996-1999  Microsoft Corporation

Module Name:

    ntddpsch.h

Abstract:

    defines that are exported to user mode

Author:

Revision History:

--*/

#ifndef _NTDDPSCH_
#define _NTDDPSCH_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// flow data returned in the Stats structure
//

#pragma pack(1)
typedef struct _PS_COMPONENT_STATS {

#define PS_COMPONENT_ADAPTER        1
#define PS_COMPONENT_FLOW           2
#define PS_COMPONENT_CONFORMER      3
#define PS_COMPONENT_SHAPER         4
#define PS_COMPONENT_DRRSEQ         5

    ULONG Type;
    ULONG Length;
    UCHAR Stats[1];
} PS_COMPONENT_STATS, *PPS_COMPONENT_STATS;
#pragma pack()

#pragma pack(1)
typedef struct _PS_CONFORMER_STATS {
    ULONG NonconformingPacketsScheduled;
} PS_CONFORMER_STATS, *PPS_CONFORMER_STATS;
#pragma pack()

#pragma pack(1)
typedef struct _PS_SHAPER_STATS {
    ULONG MaxPacketsInShaper;
    ULONG AveragePacketsInShaper;
} PS_SHAPER_STATS, *PPS_SHAPER_STATS;
#pragma pack()

#pragma pack(1)
typedef struct _PS_DRRSEQ_STATS {
    ULONG MaxPacketsInNetcard;
    ULONG AveragePacketsInNetcard;
    ULONG MaxPacketsInSequencer;
    ULONG AveragePacketsInSequencer;
    ULONG NonconformingPacketsTransmitted;
} PS_DRRSEQ_STATS, *PPS_DRRSEQ_STATS;
#pragma pack()


#pragma pack(1)
typedef struct _PS_FLOW_STATS {

    ULONG DroppedPackets;
    ULONG PacketsScheduled;
	ULONG PacketsTransmitted;
    LARGE_INTEGER BytesScheduled;
    LARGE_INTEGER BytesTransmitted;
} PS_FLOW_STATS, *PPS_FLOW_STATS;
#pragma pack()

#pragma pack(1)
typedef struct _PS_ADAPTER_STATS {

    //
    // OutOfPackets is incremented when no packets for sending/receive packet
    // indications are available.
    //

    ULONG OutOfPackets;

    //
    // general flow stats
    //

    ULONG FlowsOpened;
    ULONG FlowsClosed;
    ULONG FlowsRejected;
    ULONG FlowsModified;
    ULONG FlowModsRejected;
    ULONG MaxSimultaneousFlows;

} PS_ADAPTER_STATS, *PPS_ADAPTER_STATS;
#pragma pack()

//
// Defines that can be used for OID_QOS_FLOW_MODE
//
#define ADAPTER_FLOW_MODE_DIFFSERV           1
#define ADAPTER_FLOW_MODE_STANDARD           2


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif /* _NTDDPSCH_ */
