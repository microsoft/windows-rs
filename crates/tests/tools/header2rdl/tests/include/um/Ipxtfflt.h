/*
Copyright (c) 1995-1999 Microsoft Corporation
    File  ipxfltdf.h

    Defines structures used with the ipx filter driver.
*/

#ifndef __ipxfltdf_h
#define __ipxfltdf_h

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//*** IPX Traffic Filters ***
typedef struct _IPX_TRAFFIC_FILTER_GLOBAL_INFO {
	ULONG	FilterAction;	// Action if there is a match with
					// any filter on the interface
} IPX_TRAFFIC_FILTER_GLOBAL_INFO, *PIPX_TRAFFIC_FILTER_GLOBAL_INFO;

// FilterAction

#define IPX_TRAFFIC_FILTER_ACTION_PERMIT	1
#define IPX_TRAFFIC_FILTER_ACTION_DENY	    2

// general traffic filter info structure

typedef struct _IPX_TRAFFIC_FILTER_INFO {

    ULONG	FilterDefinition;
    UCHAR	DestinationNetwork[4];
    UCHAR	DestinationNetworkMask[4];
    UCHAR	DestinationNode[6];
    UCHAR	DestinationSocket[2];
    UCHAR	SourceNetwork[4];
    UCHAR	SourceNetworkMask[4];
    UCHAR	SourceNode[6];
    UCHAR	SourceSocket[2];
	UCHAR	PacketType;
    } IPX_TRAFFIC_FILTER_INFO, *PIPX_TRAFFIC_FILTER_INFO;

// FilterDefinition - Flags to specify relevant IPX address fields to filter on
#define IPX_TRAFFIC_FILTER_ON_SRCNET	0x00000001
#define IPX_TRAFFIC_FILTER_ON_SRCNODE	0x00000002
#define IPX_TRAFFIC_FILTER_ON_SRCSOCKET	0x00000004

#define IPX_TRAFFIC_FILTER_ON_DSTNET	0x00000010
#define IPX_TRAFFIC_FILTER_ON_DSTNODE	0x00000020
#define IPX_TRAFFIC_FILTER_ON_DSTSOCKET	0x00000040

#define IPX_TRAFFIC_FILTER_ON_PKTTYPE	0x00000100
#define IPX_TRAFFIC_FILTER_LOG_MATCHES	0x80000000

typedef struct _FLT_IF_SET_PARAMS {
	ULONG			InterfaceIndex;	// Index of the interface
	ULONG			FilterAction;	// Filter action
	ULONG			FilterSize;	// sizeof (IPX_TRAFFIC_FILTER_INFO)
} FLT_IF_SET_PARAMS, *PFLT_IF_SET_PARAMS;

typedef struct _FLT_IF_GET_PARAMS {
	ULONG			FilterAction;	// Filter action
	ULONG			FilterSize;	// sizeof (IPX_TRAFFIC_FILTER_INFO)
	ULONG			TotalSize;	// Total size of filter description
						// array
} FLT_IF_GET_PARAMS, *PFLT_IF_GET_PARAMS;

typedef struct _FLT_PACKET_LOG {
	ULONG			SrcIfIdx;	// Index of source if (-1 - unknown)
	ULONG			DstIfIdx;	// Index of dest if (-1 - unknown)
	USHORT			DataSize;	// Total size of the data (at least 30)
	USHORT			SeqNum;		// Sequence number to account for
						// packets lost to lack of buffer space
	UCHAR			Header[30];	// IPX packet header followed by the
						// data if any
} FLT_PACKET_LOG, *PFLT_PACKET_LOG;


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif