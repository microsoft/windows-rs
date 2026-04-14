//============================================================================
// Copyright (c) Microsoft Corporation. All rights reserved.
//
// File: Mgm.h
//
// History:
//      V Raman    June-25-1997  Created.
//
// Data structures and entry points into MGM.
//============================================================================


#ifndef _MGM_H_
#define _MGM_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//----------------------------------------------------------------------------
//
// typedefs for callback interface
//
//----------------------------------------------------------------------------


//
// MGM_IF_ENTRY
//
// structure used in MGM_CREATION_ALERT_CALLBACK.  In the process of
// creating an MFE the routing protocol needs to enable/disable
// multicast forwarding on each interface
//

typedef struct _MGM_IF_ENTRY {

    DWORD               dwIfIndex;
    DWORD               dwIfNextHopAddr;
    BOOL                bIGMP;
    BOOL                bIsEnabled;
        
} MGM_IF_ENTRY, *PMGM_IF_ENTRY;



//----------------------------------------------------------------------------
//
// Callbacks into routing protocols
//
//----------------------------------------------------------------------------

//
// call into a routing protocol to perform RPF check.
//
// Invoked in the context of MgmNewPacketReceived into protocol component
// owning the incoming interface.
//

typedef DWORD
(*PMGM_RPF_CALLBACK)(
    IN              DWORD           dwSourceAddr,
    IN              DWORD           dwSourceMask,
    IN              DWORD           dwGroupAddr,
    IN              DWORD           dwGroupMask,
    IN  OUT         PDWORD          pdwInIfIndex,
    IN  OUT         PDWORD          pdwInIfNextHopAddr,
    IN  OUT         PDWORD          pdwUpStreamNbr,
    IN              DWORD           dwHdrSize,
    IN              PBYTE           pbPacketHdr,
    IN  OUT         PBYTE           pbRoute
);


//
// call into a routing protocol to determine the subset of interfaces
// (owned by the routing protocol) on which a multicast packet from a
// "new" source should be forwarded.
//
// Invoked in the context of MgmNewPacketReceived into all
// routing protocols that have outgoing interfaces for this source.
//

typedef
DWORD (*PMGM_CREATION_ALERT_CALLBACK)(
    IN              DWORD           dwSourceAddr,
    IN              DWORD           dwSourceMask,
    IN              DWORD           dwGroupAddr,
    IN              DWORD           dwGroupMask,
    IN              DWORD           dwInIfIndex,
    IN              DWORD           dwInIfNextHopAddr,
    IN              DWORD           dwIfCount,
    IN  OUT         PMGM_IF_ENTRY   pmieOutIfList
);


//
// call into routing protocol to notify protocol that an interface has
// been deleted from the outgoing interface list of a group entry / MFE
//
// invoked in the context of MgmDeleteMembershipEntry()
//

typedef
DWORD (*PMGM_PRUNE_ALERT_CALLBACK)(
    IN              DWORD           dwSourceAddr,
    IN              DWORD           dwSourceMask,
    IN              DWORD           dwGroupAddr,
    IN              DWORD           dwGroupMask,
    IN              DWORD           dwIfIndex,
    IN              DWORD           dwIfNextHopAddr,
    IN              BOOL            bMemberDelete,
    IN  OUT         PDWORD          pdwTimeout
);


//
// call into routing protocol to notify protocol that an interface has
// been added to the outgoing interface list of a group entry / MFE
//
// invoked in the context of MgmAddMembershipEntry()
//

typedef
DWORD (*PMGM_JOIN_ALERT_CALLBACK)(
    IN              DWORD           dwSourceAddr,
    IN              DWORD           dwSourceMask,
    IN              DWORD           dwGroupAddr,
    IN              DWORD           dwGroupMask,
    IN              BOOL            bMemberUpdate
);


//
// call into routing protocol to notify protocol that a packet
// has been received from a (source, group) on a wrong interface
//

typedef
DWORD (*PMGM_WRONG_IF_CALLBACK)(
    IN              DWORD           dwSourceAddr,
    IN              DWORD           dwGroupAddr,
    IN              DWORD           dwIfIndex,
    IN              DWORD           dwIfNextHopAddr,
    IN              DWORD           dwHdrSize,
    IN              PBYTE           pbPacketHdr
);


//
// call into routing protocol to notify protocol that IGMP needs to add
// an interface to the outgoing interface list of a group entry / MFE
//
// invoked in the context of MgmLocalGroupJoin()
//

typedef DWORD
(*PMGM_LOCAL_JOIN_CALLBACK) (
    IN              DWORD           dwSourceAddr,
    IN              DWORD           dwSourceMask,
    IN              DWORD           dwGroupAddr,
    IN              DWORD           dwGroupMask,
    IN              DWORD           dwIfIndex,
    IN              DWORD           dwIfNextHopAddr
);


//
// call into routing protocol to notify protocol that IGMP needs to
// delete an interface to the outgoing interface list of a group
// entry / MFE
//
// invoked in the context of MgmLocalGroupJoin()
//

typedef DWORD
(*PMGM_LOCAL_LEAVE_CALLBACK) (
    IN              DWORD           dwSourceAddr,
    IN              DWORD           dwSourceMask,
    IN              DWORD           dwGroupAddr,
    IN              DWORD           dwGroupMask,
    IN              DWORD           dwIfIndex,
    IN              DWORD           dwIfNextHopAddr
);


//
// call into IGMP to notify it that a protocol is taking or
// releasing ownership of an interface that has IGMP enabled on it.
//
// When this callback is invoked IGMP should stop adding/deleting
// group memberships on the specified interface.
//

typedef DWORD
(*PMGM_DISABLE_IGMP_CALLBACK) (
    IN              DWORD           dwIfIndex,
    IN              DWORD           dwIfNextHopAddr
);


//
// call into IGMP to notify it that a protocol has finished taking
// or releasing ownership of an interface.
//
// When this callback is invoked IGMP should add all its group memberships
// on the interface.
//

typedef DWORD
(*PMGM_ENABLE_IGMP_CALLBACK) (
    IN              DWORD           dwIfIndex,
    IN              DWORD           dwIfNextHopAddr
);


//----------------------------------------------------------------------------
//
// typedefs for MGM API interface
//
//----------------------------------------------------------------------------


//
// ROUTING_PROTOCOL_CONFIG
//
// routing protocol configuration that is passed to MGM at registration.
//
//
// Callbacks into routing protocols
//

typedef struct _ROUTING_PROTOCOL_CONFIG {

    DWORD                           dwCallbackFlags;

    PMGM_RPF_CALLBACK               pfnRpfCallback;

    PMGM_CREATION_ALERT_CALLBACK    pfnCreationAlertCallback;

    PMGM_PRUNE_ALERT_CALLBACK       pfnPruneAlertCallback;

    PMGM_JOIN_ALERT_CALLBACK        pfnJoinAlertCallback;

    PMGM_WRONG_IF_CALLBACK          pfnWrongIfCallback;


    //
    // callbacks into Routing protocols
    //

    PMGM_LOCAL_JOIN_CALLBACK         pfnLocalJoinCallback;

    PMGM_LOCAL_LEAVE_CALLBACK        pfnLocalLeaveCallback;


    //
    // callbacks into IGMP
    //

    PMGM_DISABLE_IGMP_CALLBACK      pfnDisableIgmpCallback;

    PMGM_ENABLE_IGMP_CALLBACK       pfnEnableIgmpCallback;

} ROUTING_PROTOCOL_CONFIG, *PROUTING_PROTOCOL_CONFIG;


//
// MGM_ENUM_TYPES
//
// Enumeration types to be specified when the
//

typedef enum _MGM_ENUM_TYPES
{
    ANY_SOURCE = 0,                 // enumerate group entries with
                                    // atleast one source

    ALL_SOURCES                     // enumerate all source entries
                                    // for a group entry
} MGM_ENUM_TYPES;


//
// SOURCE_GROUP_ENTRY
//
// (S, G) entry that is returned by the group entry enumeration API.
//

typedef struct _SOURCE_GROUP_ENTRY {

    DWORD                           dwSourceAddr;

    DWORD                           dwSourceMask;

    DWORD                           dwGroupAddr;

    DWORD                           dwGroupMask;

} SOURCE_GROUP_ENTRY, *PSOURCE_GROUP_ENTRY;



//----------------------------------------------------------------------------
//
// Entry points into MGM.
//
//----------------------------------------------------------------------------

//============================================================================
// Routing protocol registration / de-registration API
//============================================================================

DWORD
MgmRegisterMProtocol(
    IN          PROUTING_PROTOCOL_CONFIG    prpiInfo,
    IN          DWORD                       dwProtocolId,
    IN          DWORD                       dwComponentId,
    OUT         HANDLE  *                   phProtocol
);

DWORD
MgmDeRegisterMProtocol(
    IN          HANDLE                      hProtocol
);


//============================================================================
// Interface ownership API
//============================================================================

DWORD
MgmTakeInterfaceOwnership(
    IN          HANDLE                      hProtocol,
    IN          DWORD                       dwIfIndex,
    IN          DWORD                       dwIfNextHopAddr
);


DWORD
MgmReleaseInterfaceOwnership(
    IN          HANDLE                      hProtocol,
    IN          DWORD                       dwIfIndex,
    IN          DWORD                       dwIfNextHopAddr
);

DWORD
MgmGetProtocolOnInterface(
    IN          DWORD                       dwIfIndex,
    IN          DWORD                       dwIfNextHopAddr,
    IN  OUT     PDWORD                      pdwIfProtocolId,
    IN  OUT     PDWORD                      pdwIfComponentId
);


//============================================================================
// Group membership manipulation API. (addition / deletion )
//============================================================================

#define         MGM_JOIN_STATE_FLAG         0x00000001
#define         MGM_FORWARD_STATE_FLAG      0x00000002

DWORD
MgmAddGroupMembershipEntry(
    IN              HANDLE                  hProtocol,
    IN              DWORD                   dwSourceAddr,
    IN              DWORD                   dwSourceMask,
    IN              DWORD                   dwGroupAddr,
    IN              DWORD                   dwGroupMask,
    IN              DWORD                   dwIfIndex,
    IN              DWORD                   dwIfNextHopIPAddr,
    IN              DWORD                   dwFlags
);

DWORD
MgmDeleteGroupMembershipEntry(
    IN              HANDLE                  hProtocol,
    IN              DWORD                   dwSourceAddr,
    IN              DWORD                   dwSourceMask,
    IN              DWORD                   dwGroupAddr,
    IN              DWORD                   dwGroupMask,
    IN              DWORD                   dwIfIndex,
    IN              DWORD                   dwIfNextHopIPAddr,
    IN              DWORD                   dwFlags
);

//============================================================================
//
// Enumeration API
//
//============================================================================


//----------------------------------------------------------------------------
// MFE enumeration API
//----------------------------------------------------------------------------

DWORD
MgmGetMfe(
    IN              PMIB_IPMCAST_MFE        pimm,
    IN  OUT         PDWORD                  pdwBufferSize,
    IN  OUT         PBYTE                   pbBuffer
);

DWORD
MgmGetFirstMfe(
    IN  OUT         PDWORD                  pdwBufferSize,
    IN  OUT         PBYTE                   pbBuffer,
    IN  OUT         PDWORD                  pdwNumEntries
);


DWORD
MgmGetNextMfe(
    IN              PMIB_IPMCAST_MFE        pimmStart,
    IN  OUT         PDWORD                  pdwBufferSize,
    IN  OUT         PBYTE                   pbBuffer,
    IN  OUT         PDWORD                  pdwNumEntries
);


//
// Include statistics corresponding to MIB_IPMCAST_MFE_STATS
//

#define         MGM_MFE_STATS_0             0x00000001

//
// Include statistics corresponding to MIB_IPMCAST_MFE_STATS_EX
//

#define         MGM_MFE_STATS_1             0x00000002


DWORD
MgmGetMfeStats(
    IN              PMIB_IPMCAST_MFE        pimm,
    IN  OUT         PDWORD                  pdwBufferSize,
    IN  OUT         PBYTE                   pbBuffer,
    IN              DWORD                   dwFlags
);

DWORD
MgmGetFirstMfeStats(
    IN  OUT         PDWORD                  pdwBufferSize,
    IN  OUT         PBYTE                   pbBuffer,
    IN  OUT         PDWORD                  pdwNumEntries,
    IN              DWORD                   dwFlags
);


DWORD
MgmGetNextMfeStats(
    IN              PMIB_IPMCAST_MFE        pimmStart,
    IN  OUT         PDWORD                  pdwBufferSize,
    IN  OUT         PBYTE                   pbBuffer,
    IN  OUT         PDWORD                  pdwNumEntries,
    IN              DWORD                   dwFlags
);

//----------------------------------------------------------------------------
// Group menbership entry enumeration API
//----------------------------------------------------------------------------

DWORD
MgmGroupEnumerationStart(
    IN              HANDLE                  hProtocol,
    IN              MGM_ENUM_TYPES          metEnumType,
    OUT             HANDLE *                phEnumHandle
);

DWORD
MgmGroupEnumerationGetNext(
    IN              HANDLE                  hEnum,
    IN  OUT         PDWORD                  pdwBufferSize,
    IN  OUT         PBYTE                   pbBuffer,
    IN  OUT         PDWORD                  pdwNumEntries
);

DWORD
MgmGroupEnumerationEnd(
    IN              HANDLE                  hEnum
);



//-----------------------------------------------------------------
// Mgm MFE Update API.
//
//-----------------------------------------------------------------

DWORD
MgmSetMfe(
    IN              HANDLE                  hProtocol,
    IN              PMIB_IPMCAST_MFE        pmimm
);



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //_MGM_H_
