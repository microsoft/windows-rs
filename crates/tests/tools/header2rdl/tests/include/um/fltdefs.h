/*++

Copyright (c) 1995-1999  Microsoft Corporation

Module Name:

   fltdefs.h

Abstract:

    Definitions for the WIN32 filter APIs

Author:

    Arnold Miller (arnoldm) 24-Sept-1997

Revision History:

--*/

#ifndef _FLTDEFS_H
#define _FLTDEFS_H

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

typedef PVOID  FILTER_HANDLE, *PFILTER_HANDLE;
typedef PVOID  INTERFACE_HANDLE, *PINTERFACE_HANDLE;

#ifdef _M_CEE_PURE
#define PFEXPORT
#else
#define PFEXPORT __declspec(dllexport)
#endif

#ifdef __cplusplus
#define EXTERNCDECL EXTERN_C
#else
#define EXTERNCDECL
#endif

#define PFAPIENTRY EXTERNCDECL DWORD PFEXPORT WINAPI

typedef enum _GlobalFilter
{
    GF_FRAGMENTS = 2,        // check consistency of fragments
    GF_STRONGHOST = 8,       // check destination address of input frames
    GF_FRAGCACHE = 9         // check fragments from cache
} GLOBAL_FILTER, *PGLOBAL_FILTER;

typedef enum _PfForwardAction
{
    PF_ACTION_FORWARD = 0,
    PF_ACTION_DROP
} PFFORWARD_ACTION, *PPFFORWARD_ACTION;

typedef enum _PfAddresType
{
    PF_IPV4,
    PF_IPV6
} PFADDRESSTYPE, *PPFADDRESSTYPE;

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// The constants that should be used to set up the FILTER_INFO_STRUCTURE    //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#define FILTER_PROTO(ProtoId)   MAKELONG(MAKEWORD((ProtoId),0x00),0x00000)

#define FILTER_PROTO_ANY        FILTER_PROTO(0x00)
#define FILTER_PROTO_ICMP       FILTER_PROTO(0x01)
#define FILTER_PROTO_TCP        FILTER_PROTO(0x06)
#define FILTER_PROTO_UDP        FILTER_PROTO(0x11)

#define FILTER_TCPUDP_PORT_ANY  (WORD)0x0000

#define FILTER_ICMP_TYPE_ANY    (BYTE)0xff
#define FILTER_ICMP_CODE_ANY    (BYTE)0xff

typedef struct _PF_FILTER_DESCRIPTOR
{
    DWORD           dwFilterFlags;    // see below
    DWORD           dwRule;           // copied into the log when appropriate
    PFADDRESSTYPE   pfatType;
    PBYTE           SrcAddr;
    PBYTE           SrcMask;
    PBYTE           DstAddr;
    PBYTE           DstMask;
    DWORD           dwProtocol;
    DWORD           fLateBound;
    WORD            wSrcPort;
    WORD            wDstPort;
    WORD            wSrcPortHighRange;
    WORD            wDstPortHighRange;
}PF_FILTER_DESCRIPTOR, *PPF_FILTER_DESCRIPTOR;


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Structure for PfGetInterfaceStatistics                                   //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

typedef struct _PF_FILTER_STATS
{
    DWORD       dwNumPacketsFiltered;
    PF_FILTER_DESCRIPTOR info;
}PF_FILTER_STATS, *PPF_FILTER_STATS;

typedef struct _PF_INTERFACE_STATS
{
    PVOID               pvDriverContext;
    DWORD               dwFlags;          // none as yet (28-Sept-1997)
    DWORD               dwInDrops;
    DWORD               dwOutDrops;
    PFFORWARD_ACTION    eaInAction;
    PFFORWARD_ACTION    eaOutAction;
    DWORD               dwNumInFilters;
    DWORD               dwNumOutFilters;
    DWORD               dwFrag;
    DWORD               dwSpoof;
    DWORD               dwReserved1;
    DWORD               dwReserved2;
    LARGE_INTEGER       liSYN;
    LARGE_INTEGER       liTotalLogged;
    DWORD               dwLostLogEntries;
    PF_FILTER_STATS     FilterInfo[1];
} PF_INTERFACE_STATS, *PPF_INTERFACE_STATS;


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// The number of bytes starting at SrcAddr. If you add something to the     //
// structure make sure this remains valid                                   //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#define FILTERSIZE                                      \
    (sizeof(PF_FILTER_DESCRIPTOR) -                     \
     (DWORD)(&((PPF_FILTER_DESCRIPTOR)0)->SrcAddr))


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Flags for PF_FILTER_DESCRIPTOR                                           //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

//
// Disallows incoming SYN
//

#define FD_FLAGS_NOSYN      0x1

//
// All legal flags
//

#define FD_FLAGS_ALLFLAGS   FD_FLAGS_NOSYN


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Late bound defs. Go in fLateBound in a PF_FILTER_DESCRIPTOR and          //
// describe which other fields of the filter are affected  by a             //
// PfRebindFilters call. In general such filters are on  WAN interfaces     //
// where one or the other address may change as the connection is           //
// reconnected.                                                             //
// The assumption is that such interfaces HAVE ONLY ONE ADDRESS.            //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////


#define LB_SRC_ADDR_USE_SRCADDR_FLAG     0x00000001
#define LB_SRC_ADDR_USE_DSTADDR_FLAG     0x00000002
#define LB_DST_ADDR_USE_SRCADDR_FLAG     0x00000004
#define LB_DST_ADDR_USE_DSTADDR_FLAG     0x00000008
#define LB_SRC_MASK_LATE_FLAG            0x00000010
#define LB_DST_MASK_LATE_FLAG            0x00000020

typedef struct _PF_LATEBIND_INFO
{
    PBYTE   SrcAddr;
    PBYTE   DstAddr;
    PBYTE   Mask;
}PF_LATEBIND_INFO, *PPF_LATEBIND_INFO;

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// The format of a logged frame and defs for it.                            //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

typedef enum _PfFrameType
{
    PFFT_FILTER = 1,                  // a filter violation
    PFFT_FRAG   = 2,                  // bad fragment
    PFFT_SPOOF   = 3                  // strong host failure
} PFFRAMETYPE, *PPFFRAMETYPE;

typedef struct _pfLogFrame
{
    LARGE_INTEGER  Timestamp;
    PFFRAMETYPE    pfeTypeOfFrame;
    DWORD          dwTotalSizeUsed;      // used to find the next frame
    DWORD          dwFilterRule;         // from the filter
    WORD           wSizeOfAdditionalData;
    WORD           wSizeOfIpHeader;
    DWORD          dwInterfaceName;      // the name of the interface
    DWORD          dwIPIndex;
    BYTE           bPacketData[1];       // the frame. wsizeOfIpHeader
                                         // and wsizeOfAdditionalData
                                         // describe this
} PFLOGFRAME, *PPFLOGFRAME;

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Error codes. These extend the WIN32 errors by having errors specific to  //
// these APIs. Besides these errors, the APIs may return any of the WIN32   //
// errors.                                                                  //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////


#define ERROR_BASE  23000

#define PFERROR_NO_PF_INTERFACE    (ERROR_BASE + 0)   // never returned.
#define PFERROR_NO_FILTERS_GIVEN   (ERROR_BASE + 1)
#define PFERROR_BUFFER_TOO_SMALL   (ERROR_BASE + 2)
#define ERROR_IPV6_NOT_IMPLEMENTED (ERROR_BASE + 3)


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// The API prototypes                                                       //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

PFAPIENTRY
PfCreateInterface(
    DWORD            dwName,
    PFFORWARD_ACTION inAction,
    PFFORWARD_ACTION outAction,
    BOOL             bUseLog,
    BOOL             bMustBeUnique,
    INTERFACE_HANDLE *ppInterface
    );

PFAPIENTRY
PfDeleteInterface(
    INTERFACE_HANDLE pInterface
    );

PFAPIENTRY
PfAddFiltersToInterface(
    INTERFACE_HANDLE      ih,
    DWORD                 cInFilters,
    PPF_FILTER_DESCRIPTOR pfiltIn,
    DWORD                 cOutFilters,
    PPF_FILTER_DESCRIPTOR pfiltOut,
    PFILTER_HANDLE        pfHandle
    );

PFAPIENTRY
PfRemoveFiltersFromInterface(
    INTERFACE_HANDLE      ih,
    DWORD                 cInFilters,
    PPF_FILTER_DESCRIPTOR pfiltIn,
    DWORD                 cOutFilters,
    PPF_FILTER_DESCRIPTOR pfiltOut
    );

PFAPIENTRY
PfRemoveFilterHandles(
    INTERFACE_HANDLE   pInterface,
    DWORD              cFilters,
    PFILTER_HANDLE     pvHandles
    );


PFAPIENTRY
PfUnBindInterface(
    INTERFACE_HANDLE   pInterface
    );

PFAPIENTRY
PfBindInterfaceToIndex(
    INTERFACE_HANDLE    pInterface,
    DWORD               dwIndex,
    PFADDRESSTYPE       pfatLinkType,
    PBYTE               LinkIPAddress
    );

PFAPIENTRY
PfBindInterfaceToIPAddress(
    INTERFACE_HANDLE    pInterface,
    PFADDRESSTYPE       pfatType,
    PBYTE               IPAddress
    );

PFAPIENTRY
PfRebindFilters(
    INTERFACE_HANDLE    pInterface,
    PPF_LATEBIND_INFO   pLateBindInfo
    );

PFAPIENTRY
PfAddGlobalFilterToInterface(
    INTERFACE_HANDLE   pInterface,
    GLOBAL_FILTER      gfFilter
    );

PFAPIENTRY
PfRemoveGlobalFilterFromInterface(
    INTERFACE_HANDLE   pInterface,
    GLOBAL_FILTER      gfFilter
    );


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Log APIs. Note that there is at most one log and it must be created      //
// before any interface needing it is created. There is no way to set a     //
// log onto an existing interface. The log can be applied to any or all of  //
// the interfaces.                                                          //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

PFAPIENTRY
PfMakeLog(
    HANDLE  hEvent
    );

//
// Provide a buffer, and notification parameters, and get back
// the old buffer and status.
//

PFAPIENTRY
PfSetLogBuffer(
    PBYTE   pbBuffer,
    DWORD   dwSize,
    DWORD   dwThreshold,
    DWORD   dwEntries,
    PDWORD  pdwLoggedEntries,
    PDWORD  pdwLostEntries,
    PDWORD  pdwSizeUsed
    );

//
// Doing this will disable the log on any of the interfaces. But if
// an interface was created with the log, the actual log will not be
// completely deleted until that interface is deleted. This is a small
// point, but it might explain a mystery or two.
//

PFAPIENTRY
PfDeleteLog(
    VOID
    );


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Get statistics. Note pdwBufferSize in an IN/OUT parameter. If            //
// ERROR_INSUFFICIENT_BUFFER is returned, the common statistics are         //
// available and the correct byte count is in *pdwBufferSize. If only the   //
// interface statistics are needed, provide a buffer of size                //
// PF_INTERFACE_STATS only.                                                 //
// If the filter descriptions are also needed, then supply a large buffer,  //
// or use the returned count from the first call to allocate a buffer of    //
// sufficient size. Note that for a shared interface, this second call may  //
// fail with ERROR_INSUFFICIENT_BUFFER. This can happen if the other        //
// sharers add filters in the interim. This should not happen for a UNIQUE  //
// interface.                                                               //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////


PFAPIENTRY
PfGetInterfaceStatistics(
    INTERFACE_HANDLE    pInterface,
    PPF_INTERFACE_STATS ppfStats,
    PDWORD              pdwBufferSize,
    BOOL                fResetCounters
    );


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Test a packet.                                                           //
// This call will evaluate the packet against the given interfaces          //
// and return the filtering action.                                         //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

PFAPIENTRY
PfTestPacket(
    INTERFACE_HANDLE   pInInterface  OPTIONAL,
    INTERFACE_HANDLE   pOutInterface OPTIONAL,
    DWORD              cBytes,
    PBYTE              pbPacket,
    PPFFORWARD_ACTION  ppAction
    );



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif
