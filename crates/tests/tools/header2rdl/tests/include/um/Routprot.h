/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    routprot.h

Abstract:
    Include file for Routing Protocol inteface to Router Managers

--*/


#ifndef _ROUTPROT_H_
#define _ROUTPROT_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include "stm.h"

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201)
#pragma warning(disable:4200)

#include <nldef.h>
#include <in6addr.h>

#ifdef __cplusplus
extern "C" {
#endif

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Supported functionality flags                                            //
//                                                                          //
// ROUTING 		    Imports Routing Table Manager APIs              //
// ROUTINGV6                Imports Routing Table Manager APIs              //
//                                      (indicated IPv6 router manager)     //
// SERVICES		    Exports Service Table Manager APIs              //
// DEMAND_UPDATE_ROUTES     IP and IPX RIP support for Autostatic           //
// DEMAND_UPDATE_SERVICES   IPX support for Autostatic                      //
// ADD_ALL_INTERFACES       Adds all interfaces, even if no info is present //
// MULTICAST                Supports multicast                              //
// POWER                    Power Manageable                                //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#define RF_ROUTING 		0x00000001
#define RF_ROUTINGV6            0x00000002
#define RF_DEMAND_UPDATE_ROUTES 0x00000004
#define RF_ADD_ALL_INTERFACES   0x00000010
#define RF_MULTICAST            0x00000020
#define RF_POWER                0x00000040

#if MPR60
    #define MS_ROUTER_VERSION       0x00000600
#elif MPR50
    #define MS_ROUTER_VERSION       0x00000500
#else
    #error Router version not defined
#endif

typedef enum _ROUTING_PROTOCOL_EVENTS
{
    ROUTER_STOPPED,              // Result is empty
    SAVE_GLOBAL_CONFIG_INFO,     // Result is empty
    SAVE_INTERFACE_CONFIG_INFO,  // Result is interface index
                                 // for which config info is to be saved.
    UPDATE_COMPLETE,             // Result is UPDATE_COMPLETE_MESSAGE structure
}ROUTING_PROTOCOL_EVENTS;

#if MPR60
#define ROUTING_DOMAIN_INFO_REVISION_1    0x1

#define INTERFACE_INFO_REVISION_1    0x1

typedef struct _ROUTING_DOMAIN_INFO_1
{
    WCHAR                 wszRoutingDomainName[MAX_ROUTING_DOMAIN_NAME_LEN+1];
    NET_IF_COMPARTMENT_ID compartmentId;
    USHORT                usRtmInstanceId;
}ROUTING_DOMAIN_INFO_1, *PROUTING_DOMAIN_INFO_1;

typedef struct _INTERFACE_INFO_1
{
    NET_IF_COMPARTMENT_ID compartmentId;
    GUID                  routingDomainId;
    DWORD                 dwOperationalState;
}INTERFACE_INFO_1, *PINTERFACE_INFO_1;

#endif

typedef enum _NET_INTERFACE_TYPE
{
    PERMANENT,
    DEMAND_DIAL,
    LOCAL_WORKSTATION_DIAL,
    REMOTE_WORKSTATION_DIAL
} NET_INTERFACE_TYPE;

//
// Interface Receive Types
//

#define IR_PROMISCUOUS                  0
#define IR_PROMISCUOUS_MULTICAST        1

typedef struct _SUPPORT_FUNCTIONS_50
{
    union
    {
        ULONGLONG   _Align8;

        struct
        {
            DWORD   dwVersion;
            DWORD   dwReserved;
        };
    };

    //
    // Function called by routing protocol to initiate demand dial connection
    //

    OUT DWORD
    (WINAPI *DemandDialRequest)(
        IN      DWORD           ProtocolId,
        IN      DWORD           InterfaceIndex
        ) ;

    //
    // Can be called to set the interface's receive capability
    // See IR_Xxx values above
    //

    OUT DWORD
    (WINAPI *SetInterfaceReceiveType)(
        IN      DWORD           ProtocolId,
        IN      DWORD           InterfaceIndex,
        IN      DWORD           InterfaceReceiveType,
        IN      BOOL            bActivate
        );

    //
    // Must be called by every protocol to set the route preference
    // and perform other validation
    //

    OUT DWORD
    (WINAPI *ValidateRoute)(
        IN      DWORD           ProtocolId,
        IN      PVOID           RouteInfo,
        IN      PVOID           DestAddress OPTIONAL
        );


    //
    // The following entrypoints are provided as a way for getting
    // information that spans components
    //

    OUT DWORD
    (WINAPI *MIBEntryCreate)(
        IN      DWORD           dwRoutingPid,
        IN      DWORD           dwEntrySize,
        IN      LPVOID          lpEntry
        );

    OUT DWORD
    (WINAPI *MIBEntryDelete)(
        IN      DWORD           dwRoutingPid,
        IN      DWORD           dwEntrySize,
        IN      LPVOID          lpEntry
        );

    OUT DWORD
    (WINAPI *MIBEntrySet)(
        IN      DWORD           dwRoutingPid,
        IN      DWORD           dwEntrySize,
        IN      LPVOID          lpEntry
        );

    OUT DWORD
    (WINAPI *MIBEntryGet)(
        IN      DWORD           dwRoutingPid,
        IN      DWORD           dwInEntrySize,
        IN      LPVOID          lpInEntry,
        IN OUT  LPDWORD         lpOutEntrySize,
        OUT     LPVOID          lpOutEntry
        );

    OUT DWORD
    (WINAPI *MIBEntryGetFirst)(
        IN      DWORD           dwRoutingPid,
        IN      DWORD           dwInEntrySize,
        IN      LPVOID          lpInEntry,
        IN OUT  LPDWORD         lpOutEntrySize,
        OUT     LPVOID          lpOutEntry
        );

    OUT DWORD
    (WINAPI *MIBEntryGetNext)(
        IN      DWORD           dwRoutingPid,
        IN      DWORD           dwInEntrySize,
        IN      LPVOID          lpInEntry,
        IN OUT  LPDWORD         lpOutEntrySize,
        OUT     LPVOID          lpOutEntry
        );

    //
    // Can be called to get the router ID value
    //

    OUT DWORD
    (WINAPI *GetRouterId)(VOID);

    OUT BOOL
    (WINAPI *HasMulticastBoundary)(
        IN      DWORD           dwIfIndex,
        IN      DWORD           dwGroupAddress
        );

} SUPPORT_FUNCTIONS_50;

typedef struct _SUPPORT_FUNCTIONS_60
{
    union
    {
        ULONGLONG   _Align8;

        struct
        {
            DWORD   dwVersion;
            DWORD   dwReserved;
        };
    };

    //
    // Function called by routing protocol to initiate demand dial connection
    //

    OUT DWORD
    (WINAPI *DemandDialRequest)(
        IN      DWORD           ProtocolId,
        IN      DWORD           InterfaceIndex
        ) ;

    //
    // Can be called to set the interface's receive capability
    // See IR_Xxx values above
    //

    OUT DWORD
    (WINAPI *SetInterfaceReceiveType)(
        IN      DWORD           ProtocolId,
        IN      DWORD           InterfaceIndex,
        IN      DWORD           InterfaceReceiveType,
        IN      BOOL            bActivate
        );

    //
    // Must be called by every protocol to set the route preference
    // and perform other validation
    //

    OUT DWORD
    (WINAPI *ValidateRoute)(
        IN      DWORD           ProtocolId,
        IN      PVOID           RouteInfo,
        IN      PVOID           DestAddress OPTIONAL
        );


    //
    // The following entrypoints are provided as a way for getting
    // information that spans components
    //

    OUT DWORD
    (WINAPI *MIBEntryCreate)(
        IN      DWORD           dwRoutingPid,
        IN      DWORD           dwEntrySize,
        IN      LPVOID          lpEntry
        );

    OUT DWORD
    (WINAPI *MIBEntryDelete)(
        IN      DWORD           dwRoutingPid,
        IN      DWORD           dwEntrySize,
        IN      LPVOID          lpEntry
        );

    OUT DWORD
    (WINAPI *MIBEntrySet)(
        IN      DWORD           dwRoutingPid,
        IN      DWORD           dwEntrySize,
        IN      LPVOID          lpEntry
        );

    OUT DWORD
    (WINAPI *MIBEntryGet)(
        IN      DWORD           dwRoutingPid,
        IN      DWORD           dwInEntrySize,
        IN      LPVOID          lpInEntry,
        IN OUT  LPDWORD         lpOutEntrySize,
        OUT     LPVOID          lpOutEntry
        );

    OUT DWORD
    (WINAPI *MIBEntryGetFirst)(
        IN      DWORD           dwRoutingPid,
        IN      DWORD           dwInEntrySize,
        IN      LPVOID          lpInEntry,
        IN OUT  LPDWORD         lpOutEntrySize,
        OUT     LPVOID          lpOutEntry
        );

    OUT DWORD
    (WINAPI *MIBEntryGetNext)(
        IN      DWORD           dwRoutingPid,
        IN      DWORD           dwInEntrySize,
        IN      LPVOID          lpInEntry,
        IN OUT  LPDWORD         lpOutEntrySize,
        OUT     LPVOID          lpOutEntry
        );

    //
    // Can be called to get the router ID value
    //

    OUT DWORD
    (WINAPI *GetRouterId)(VOID);

    OUT BOOL
    (WINAPI *HasMulticastBoundary)(
        IN      DWORD           dwIfIndex,
        IN      DWORD           dwGroupAddress
        );

    //
    // Must be called by every protocol to set the route preference
    // and perform other validation. 
    // Differs from ValidateRoute in SubProtocolId
    //

    OUT DWORD
    (WINAPI *ValidateRouteEx)(
        IN      DWORD           dwProtocolId,
        IN      DWORD           dwSubProtocolId,
        IN      DWORD           dwTransportId,
        IN      PVOID           pRouteInfo,
        IN      GUID*           pRoutingDomainId,
        IN      PVOID           pDestAddress OPTIONAL
        );

    //
    // Must be called by every multi-tenant protocol 
    // to find information related to a routing domain
    //

    OUT DWORD
    (WINAPI *GetRoutingDomainInfo)(
        IN      GUID*           pRoutingDomainId,
        IN      DWORD           dwRevision,
        IN      DWORD           dwBufSize,
        IN OUT  PVOID           pRoutingDomainInfo
        );

    //
    // Must be called by every multi-tenant protocol 
    // to find information related to an interface
    //

    OUT DWORD
    (WINAPI *GetInterfaceInformation)(
        IN      DWORD           dwInterfaceIndex,
        IN      DWORD           dwTransportId,
        IN      DWORD           dwRevision,
        IN      DWORD           dwBufSize,
        IN OUT  PVOID           pInterfaceInfo
        );


} SUPPORT_FUNCTIONS_60;


#if MPR60
typedef SUPPORT_FUNCTIONS_60 SUPPORT_FUNCTIONS;
#else
typedef SUPPORT_FUNCTIONS_50 SUPPORT_FUNCTIONS;
#endif

typedef SUPPORT_FUNCTIONS *PSUPPORT_FUNCTIONS;



//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// All IP Protocols must use the protocol ids defined in the range below.   //
// Protocols not identified below can use any unassigned number BELOW       //
// 0xffff0000                                                               //
//                                                                          //
// NOTE: These numbers have been chosen to coincide with MIB-II protocol    //
// numbers. Allocation should not be arbitrary.                             //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

//
// See nldef.h for the IANA assigned values of PROTO_IP_*.
//

//
// The multicast protocol IDs
//

#define PROTO_IP_MSDP        9
#define PROTO_IP_IGMP       10
#define PROTO_IP_BGMP       11

//
// The IPRTRMGR_PID is 10000 // 0x00002710
//

#define PROTO_IP_VRRP               112
#define PROTO_IP_BOOTP              9999    // 0x0000270F

//included for DHCPv6 Relay Agent
#define PROTO_IPV6_DHCP 			999	    // 0x000003E7

#define PROTO_IP_NT_AUTOSTATIC      10002   // 0x00002712
#define PROTO_IP_DNS_PROXY          10003   // 0x00002713
#define PROTO_IP_DHCP_ALLOCATOR     10004   // 0x00002714
#define PROTO_IP_NAT                10005   // 0x00002715
#define PROTO_IP_NT_STATIC          10006   // 0x00002716
#define PROTO_IP_NT_STATIC_NON_DOD  10007   // 0x00002717
#define PROTO_IP_DIFFSERV           10008   // 0x00002718
#define PROTO_IP_MGM                10009   // 0x00002719
#define PROTO_IP_ALG                10010   // 0x0000271A
#define PROTO_IP_H323               10011   // 0x0000271B
#define PROTO_IP_FTP                10012   // 0x0000271C
#define PROTO_IP_DTP                10013   // 0x0000271D

// For all future development, the following macro must be used to generate
// Ids
//

//
//  Type            -   2 bits
//  Vendor          -  14 bits
//  ProtocolId      -  16 bits
//

#define PROTOCOL_ID(Type, VendorId, ProtocolId) \
    (((Type & 0x03)<<30)|((VendorId & 0x3FFF)<<16)|(ProtocolId & 0xFFFF))

//
//  |----|----|----|----|----|----|----|----|
//   Ty*** Vendor Id *** StandardProtocolId
//

#define TYPE_FROM_PROTO_ID(X)       (((X) >> 30) & 0x03)
#define VENDOR_FROM_PROTO_ID(X)     (((X) >> 16) & 0x3FFF)
#define PROTO_FROM_PROTO_ID(X)      ((X) & 0xFFFF)

//
// Types MS0 and MS1 are Microsoft Reserved
// A protocol that supports both unicast and multicast should use type
// MCAST
//

#define PROTO_TYPE_UCAST            0
#define PROTO_TYPE_MCAST            1
#define PROTO_TYPE_MS0              2
#define PROTO_TYPE_MS1              3

#define PROTO_VENDOR_MS0            0x0000
#define PROTO_VENDOR_MS1            0x137   // 311
#define PROTO_VENDOR_MS2            0x3FFF


#define MS_IPV6_DHCP                 \
    PROTOCOL_ID(PROTO_TYPE_UCAST, PROTO_VENDOR_MS0, PROTO_IPV6_DHCP)

// included for IPV6 DHCP
#define MS_IP_BOOTP                 \
    PROTOCOL_ID(PROTO_TYPE_UCAST, PROTO_VENDOR_MS0, PROTO_IP_BOOTP)
    
#define MS_IP_RIP                   \
    PROTOCOL_ID(PROTO_TYPE_UCAST, PROTO_VENDOR_MS0, PROTO_IP_RIP)

#define MS_IP_OSPF                  \
    PROTOCOL_ID(PROTO_TYPE_UCAST, PROTO_VENDOR_MS0, PROTO_IP_OSPF)

#define MS_IP_BGP                   \
    PROTOCOL_ID(PROTO_TYPE_UCAST, PROTO_VENDOR_MS1, PROTO_IP_BGP)

#define MS_IP_IGMP                  \
    PROTOCOL_ID(PROTO_TYPE_MCAST, PROTO_VENDOR_MS1, PROTO_IP_IGMP)

#define MS_IP_BGMP                  \
    PROTOCOL_ID(PROTO_TYPE_MCAST, PROTO_VENDOR_MS1, PROTO_IP_BGMP)

#define MS_IP_MSDP                  \
    PROTOCOL_ID(PROTO_TYPE_MCAST, PROTO_VENDOR_MS1, PROTO_IP_MSDP)

#define MS_IP_DNS_PROXY             \
    PROTOCOL_ID(PROTO_TYPE_MS0, PROTO_VENDOR_MS1, PROTO_IP_DNS_PROXY)

#define MS_IP_DHCP_ALLOCATOR        \
    PROTOCOL_ID(PROTO_TYPE_MS0, PROTO_VENDOR_MS1, PROTO_IP_DHCP_ALLOCATOR)

#define MS_IP_NAT                   \
    PROTOCOL_ID(PROTO_TYPE_MS0, PROTO_VENDOR_MS1, PROTO_IP_NAT)

#define MS_IP_DIFFSERV              \
    PROTOCOL_ID(PROTO_TYPE_MS0, PROTO_VENDOR_MS1, PROTO_IP_DIFFSERV)

#define MS_IP_MGM                   \
    PROTOCOL_ID(PROTO_TYPE_MS0, PROTO_VENDOR_MS1, PROTO_IP_MGM)

#define MS_IP_VRRP                  \
    PROTOCOL_ID(PROTO_TYPE_MS0, PROTO_VENDOR_MS1, PROTO_IP_VRRP)

#define MS_IP_DTP                   \
    PROTOCOL_ID(PROTO_TYPE_MS0, PROTO_VENDOR_MS1, PROTO_IP_DTP)

#define MS_IP_H323                  \
    PROTOCOL_ID(PROTO_TYPE_MS0, PROTO_VENDOR_MS1, PROTO_IP_H323)

#define MS_IP_FTP                   \
    PROTOCOL_ID(PROTO_TYPE_MS0, PROTO_VENDOR_MS1, PROTO_IP_FTP)

#define MS_IP_ALG                   \
    PROTOCOL_ID(PROTO_TYPE_MS0, PROTO_VENDOR_MS1, PROTO_IP_ALG)

//
// All IPX Protocols must use the protocol ids defined in the range below.
// Protocols not identified below can use any unassigned number greater than
// IPX_PROTOCOL_BASE.
//

#define IPX_PROTOCOL_BASE   0x0001ffff
#define IPX_PROTOCOL_RIP    IPX_PROTOCOL_BASE + 1



typedef struct _UPDATE_COMPLETE_MESSAGE
{
    ULONG	InterfaceIndex;
    ULONG	UpdateType;	       // DEMAND_UPDATE_ROUTES, DEMAND_UPDATE_SERVICES
    ULONG	UpdateStatus;	   // NO_ERROR if successfull

}   UPDATE_COMPLETE_MESSAGE, *PUPDATE_COMPLETE_MESSAGE;

//
//  Message returned in Result parameter to GET_EVENT_MESSAGE api call.
//  UpdateCompleteMessage   returned for UPDATE_COMPLETE message
//  InterfaceIndex          returned for SAVE_INTERFACE_CONFIG_INFO message
//

typedef union _MESSAGE
{
    UPDATE_COMPLETE_MESSAGE UpdateCompleteMessage;
    DWORD                   InterfaceIndex;

}   MESSAGE, *PMESSAGE;

//
// Routing Interface Status types
//

#define RIS_INTERFACE_ADDRESS_CHANGE            0
#define RIS_INTERFACE_ENABLED                   1
#define RIS_INTERFACE_DISABLED                  2
#define RIS_INTERFACE_MEDIA_PRESENT             3
#define RIS_INTERFACE_MEDIA_ABSENT              4



//
// IP Adapter Binding Info
// This is the information associated with an ADDRESS_ARRIVAL event
// An address arrival may have AddressCount == 0, this implies a unnumbered
// interface
//

typedef struct IP_LOCAL_BINDING
{
    DWORD   Address;
    DWORD   Mask;
}IP_LOCAL_BINDING, *PIP_LOCAL_BINDING;

typedef struct IPV6_LOCAL_BINDING
{
    IN6_ADDR    Address;
    DWORD    PrefixLength;
}IPV6_LOCAL_BINDING, *PIPV6_LOCAL_BINDING;

typedef struct	IP_ADAPTER_BINDING_INFO
{
    ULONG               AddressCount;
    DWORD               RemoteAddress;
    ULONG               Mtu;
    ULONGLONG           Speed;
    _Field_size_(AddressCount) IP_LOCAL_BINDING    Address[0];
}IP_ADAPTER_BINDING_INFO, *PIP_ADAPTER_BINDING_INFO;


#define SIZEOF_IP_BINDING(X)                                \
    (FIELD_OFFSET(IP_ADAPTER_BINDING_INFO,Address[0]) +     \
     ((X) * sizeof(IP_LOCAL_BINDING)))

typedef struct  IPV6_ADAPTER_BINDING_INFO
{
    ULONG                   AddressCount;
    IN6_ADDR                RemoteAddress;
    ULONG                   Mtu;
    ULONGLONG               Speed;
    _Field_size_(AddressCount) IPV6_LOCAL_BINDING      Address[0];
}IPV6_ADAPTER_BINDING_INFO, *PIPV6_ADAPTER_BINDING_INFO;

#define SIZEOF_IPV6_BINDING(X)                                \
    (FIELD_OFFSET(IPV6_ADAPTER_BINDING_INFO,Address[0]) +     \
     ((X) * sizeof(IPV6_LOCAL_BINDING)))


typedef
DWORD
(WINAPI * PSTART_PROTOCOL) (
    IN HANDLE 	            NotificationEvent,
    IN PSUPPORT_FUNCTIONS   SupportFunctions,
    IN LPVOID               GlobalInfo,
    IN ULONG                StructureVersion,
    IN ULONG                StructureSize,
    IN ULONG                StructureCount
    );

typedef
DWORD
(WINAPI * PSTART_COMPLETE) (
    VOID
    );

typedef
DWORD
(WINAPI * PSTOP_PROTOCOL) (
    VOID
    );

typedef
DWORD
(WINAPI * PADD_INTERFACE) (
    IN LPWSTR               InterfaceName,
    IN ULONG	            InterfaceIndex,
    IN NET_INTERFACE_TYPE   InterfaceType,
    IN DWORD                MediaType,
    IN WORD                 AccessType,
    IN WORD                 ConnectionType,
    IN PVOID	            InterfaceInfo,
    IN ULONG                StructureVersion,
    IN ULONG                StructureSize,
    IN ULONG                StructureCount
    );

typedef
DWORD
(WINAPI * PDELETE_INTERFACE) (
    IN ULONG	InterfaceIndex
    );

typedef
DWORD
(WINAPI * PGET_EVENT_MESSAGE) (
    OUT ROUTING_PROTOCOL_EVENTS  *Event,
    OUT MESSAGE                  *Result
    );

typedef
DWORD
(WINAPI * PGET_INTERFACE_INFO) (
    IN      ULONG	InterfaceIndex,
    IN      PVOID   InterfaceInfo,
    IN  OUT PULONG  BufferSize,
    OUT     PULONG	StructureVersion,
    IN      PULONG	StructureSize,
    OUT     PULONG	StructureCount
    );

typedef
DWORD
(WINAPI * PSET_INTERFACE_INFO) (
    IN ULONG	InterfaceIndex,
    IN PVOID	InterfaceInfo,
    IN ULONG    StructureVersion,
    IN ULONG    StructureSize,
    IN ULONG    StructureCount
    );

typedef
DWORD
(WINAPI * PINTERFACE_STATUS) (
    IN ULONG	InterfaceIndex,
    IN BOOL     InterfaceActive,
    IN DWORD    StatusType,
    IN PVOID	StatusInfo
    );

typedef
DWORD
(WINAPI * PQUERY_POWER) (
    IN  DWORD   PowerType
    );

typedef
DWORD
(WINAPI * PSET_POWER) (
    IN  DWORD   PowerType
    );

typedef
DWORD
(WINAPI * PGET_GLOBAL_INFO) (
    IN     PVOID 	GlobalInfo,
    IN OUT PULONG   BufferSize,
    OUT    PULONG	StructureVersion,
    OUT    PULONG   StructureSize,
    OUT    PULONG   StructureCount
    );

typedef
DWORD
(WINAPI * PSET_GLOBAL_INFO) (
    IN  PVOID 	GlobalInfo,
    IN  ULONG	StructureVersion,
    IN  ULONG   StructureSize,
    IN  ULONG   StructureCount
    );

typedef
DWORD
(WINAPI * PDO_UPDATE_ROUTES) (
    IN ULONG	InterfaceIndex
    );

typedef
DWORD
(WINAPI * PMIB_CREATE) (
    IN ULONG 	InputDataSize,
    IN PVOID 	InputData
    );

typedef
DWORD
(WINAPI * PMIB_DELETE) (
    IN ULONG 	InputDataSize,
    IN PVOID 	InputData
    );

typedef
DWORD
(WINAPI * PMIB_GET) (
    IN  ULONG	InputDataSize,
    IN  PVOID	InputData,
    OUT PULONG	OutputDataSize,
    OUT PVOID	OutputData
    );

typedef
DWORD
(WINAPI * PMIB_SET) (
    IN ULONG 	InputDataSize,
    IN PVOID	InputData
    );

typedef
DWORD
(WINAPI * PMIB_GET_FIRST) (
    IN  ULONG	InputDataSize,
    IN  PVOID	InputData,
    OUT PULONG  OutputDataSize,
    OUT PVOID   OutputData
    );

typedef
DWORD
(WINAPI * PMIB_GET_NEXT) (
    IN  ULONG   InputDataSize,
    IN  PVOID	InputData,
    OUT PULONG  OutputDataSize,
    OUT PVOID	OutputData
    );

typedef
DWORD
(WINAPI * PMIB_SET_TRAP_INFO) (
    IN  HANDLE  Event,
    IN  ULONG   InputDataSize,
    IN  PVOID	InputData,
    OUT PULONG	OutputDataSize,
    OUT PVOID	OutputData
    );

typedef
DWORD
(WINAPI * PMIB_GET_TRAP_INFO) (
    IN  ULONG	InputDataSize,
    IN  PVOID	InputData,
    OUT PULONG  OutputDataSize,
    OUT PVOID	OutputData
    );

typedef
DWORD
(WINAPI *PCONNECT_CLIENT) (
    IN ULONG    InterfaceIndex,
    IN PVOID    ClientAddress
    );

typedef
DWORD
(WINAPI *PDISCONNECT_CLIENT) (
    IN ULONG    InterfaceIndex,
    IN PVOID    ClientAddress
    );

//
// InterfaceFlags used with the GetNeighbors() call below
//

#define MRINFO_TUNNEL_FLAG   0x01
#define MRINFO_PIM_FLAG      0x04
#define MRINFO_DOWN_FLAG     0x10
#define MRINFO_DISABLED_FLAG 0x20
#define MRINFO_QUERIER_FLAG  0x40
#define MRINFO_LEAF_FLAG     0x80

typedef
DWORD
(WINAPI *PGET_NEIGHBORS) (
    IN     DWORD  InterfaceIndex,
    IN     PDWORD NeighborList,
    IN OUT PDWORD NeighborListSize,
       OUT PBYTE  InterfaceFlags
    );

//
// StatusCode values used with the GetMfeStatus() call below.
// The protocol should return the highest-valued one that applies.
//

#define MFE_NO_ERROR          0 // none of the below events
#define MFE_REACHED_CORE      1 // this router is an RP/core for the group

//
// StatusCode values set by oif owner only
//

#define MFE_OIF_PRUNED        5 // no downstream receivers exist on oif

//
// StatusCode values set by iif owner only
//

#define MFE_PRUNED_UPSTREAM   4 // a prune was send upstream
#define MFE_OLD_ROUTER       11 // upstream nbr doesn't support mtrace

//
// StatusCode values which are used only by the Router Manager itself:
//

#define MFE_NOT_FORWARDING    2 // not fwding for an unspecified reason
#define MFE_WRONG_IF          3 // mtrace received on iif
#define MFE_BOUNDARY_REACHED  6 // iif or oif is admin scope boundary
#define MFE_NO_MULTICAST      7 // oif is not multicast-enabled
#define MFE_IIF               8 // mtrace arrived on iif
#define MFE_NO_ROUTE          9 // router has no route that matches
#define MFE_NOT_LAST_HOP     10 // router is not the proper last-hop router
#define MFE_PROHIBITED       12 // mtrace is administratively prohibited
#define MFE_NO_SPACE         13 // not enough room in packet

typedef
DWORD
(WINAPI *PGET_MFE_STATUS) (
    IN     DWORD  InterfaceIndex,
    IN     DWORD  GroupAddress,
    IN     DWORD  SourceAddress,
    OUT    PBYTE  StatusCode
    );

typedef
DWORD
(WINAPI * PPROTOCOL_ACTION) (
    IN  DWORD   ActionType,
    IN  PVOID 	ActionInfo,
    IN  DWORD   ActionSize
    );

typedef
DWORD
(WINAPI * PGET_ROUTING_DOMAIN_GLOBAL_INFO)(
    IN     GUID     *pRoutingDomainId,
    IN     PVOID    GlobalInfo,
    IN OUT PULONG   BufferSize,
    OUT    PULONG   StructureVersion,
    OUT    PULONG   StructureSize,
    OUT    PULONG   StructureCount
    );

typedef
DWORD
(WINAPI * PSET_ROUTING_DOMAIN_GLOBAL_INFO) (
    IN  GUID    *pRoutingDomainId,
    IN  PVOID   GlobalInfo,
    IN  ULONG   StructureVersion,
    IN  ULONG   StructureSize,
    IN  ULONG   StructureCount
    );


typedef
DWORD
(WINAPI * PGET_PROTOCOL_STATISTICS) (
    IN      DWORD   InEntryType,
    IN      PVOID   InEntry,
    IN      DWORD   InEntrySize,
    IN      DWORD   OutEntryType,
    OUT     PVOID*  OutEntry,
    OUT     PDWORD  OutEntrySize
    );

typedef
VOID
(WINAPI * PPROTOCOL_BUFFER_FREE) (
    IN      PVOID   Ptr
    );

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// This is the structure passed between the router manager and a protocol   //
// upon registration.                                                       //
//                                                                          //
// IN OUT DWORD dwVersion                                                   //
// This is filled by the router manager to indicate the version it supports.//
// The DLL MUST set this to the version that the protocol will support.     //
//                                                                          //
// IN DWORD dwProtocolId                                                    //
// This the protocol the router manager is expecting the DLL to register.   //
// If the DLL does not support this protocol, it MUST return                //
// ERROR_NOT_SUPPORTED                                                      //
// A DLL will be called once for every protocol it supports                 //
//                                                                          //
// IN OUT DWORD fSupportedFunctionality                                     //
// These are the flags denoting the functionality the router manager        //
// supports. The DLL MUST reset this to the functionality that it supports. //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////


typedef struct _MPR50_ROUTING_CHARACTERISTICS
{
    DWORD               dwVersion;
    DWORD               dwProtocolId;
    DWORD               fSupportedFunctionality;

    PSTART_PROTOCOL     pfnStartProtocol;
    PSTART_COMPLETE     pfnStartComplete;
    PSTOP_PROTOCOL      pfnStopProtocol;
    PGET_GLOBAL_INFO    pfnGetGlobalInfo;
    PSET_GLOBAL_INFO    pfnSetGlobalInfo;
    PQUERY_POWER        pfnQueryPower;
    PSET_POWER          pfnSetPower;

    PADD_INTERFACE      pfnAddInterface;
    PDELETE_INTERFACE   pfnDeleteInterface;
    PINTERFACE_STATUS   pfnInterfaceStatus;
    PGET_INTERFACE_INFO pfnGetInterfaceInfo;
    PSET_INTERFACE_INFO pfnSetInterfaceInfo;

    PGET_EVENT_MESSAGE  pfnGetEventMessage;

    PDO_UPDATE_ROUTES   pfnUpdateRoutes;

    PCONNECT_CLIENT     pfnConnectClient;
    PDISCONNECT_CLIENT  pfnDisconnectClient;

    PGET_NEIGHBORS      pfnGetNeighbors;
    PGET_MFE_STATUS     pfnGetMfeStatus;

    PMIB_CREATE         pfnMibCreateEntry;
    PMIB_DELETE         pfnMibDeleteEntry;
    PMIB_GET            pfnMibGetEntry;
    PMIB_SET            pfnMibSetEntry;
    PMIB_GET_FIRST      pfnMibGetFirstEntry;
    PMIB_GET_NEXT       pfnMibGetNextEntry;
    PMIB_SET_TRAP_INFO  pfnMibSetTrapInfo;
    PMIB_GET_TRAP_INFO  pfnMibGetTrapInfo;

}MPR50_ROUTING_CHARACTERISTICS;

typedef struct _MPR60_ROUTING_CHARACTERISTICS
{
    DWORD                           dwVersion;
    DWORD                           dwProtocolId;
    DWORD                           fSupportedFunctionality;

    PSTART_PROTOCOL                 pfnStartProtocol;
    PSTART_COMPLETE                 pfnStartComplete;
    PSTOP_PROTOCOL                  pfnStopProtocol;
    PGET_GLOBAL_INFO                pfnGetGlobalInfo;
    PSET_GLOBAL_INFO                pfnSetGlobalInfo;
    PQUERY_POWER                    pfnQueryPower;
    PSET_POWER                      pfnSetPower;

    PADD_INTERFACE                  pfnAddInterface;
    PDELETE_INTERFACE               pfnDeleteInterface;
    PINTERFACE_STATUS               pfnInterfaceStatus;
    PGET_INTERFACE_INFO             pfnGetInterfaceInfo;
    PSET_INTERFACE_INFO             pfnSetInterfaceInfo;

    PGET_EVENT_MESSAGE              pfnGetEventMessage;

    PDO_UPDATE_ROUTES               pfnUpdateRoutes;

    PCONNECT_CLIENT                 pfnConnectClient;
    PDISCONNECT_CLIENT              pfnDisconnectClient;

    PGET_NEIGHBORS                  pfnGetNeighbors;
    PGET_MFE_STATUS                 pfnGetMfeStatus;

    PMIB_CREATE                     pfnMibCreateEntry;
    PMIB_DELETE                     pfnMibDeleteEntry;
    PMIB_GET                        pfnMibGetEntry;
    PMIB_SET                        pfnMibSetEntry;
    PMIB_GET_FIRST                  pfnMibGetFirstEntry;
    PMIB_GET_NEXT                   pfnMibGetNextEntry;
    PMIB_SET_TRAP_INFO	            pfnMibSetTrapInfo;
    PMIB_GET_TRAP_INFO	            pfnMibGetTrapInfo;

    PPROTOCOL_ACTION                pfnProtocolAction;
    PGET_PROTOCOL_STATISTICS        pfnGetStatistics;
    PGET_ROUTING_DOMAIN_GLOBAL_INFO pfnGetRoutingDomainGlobalInfo;
    PSET_ROUTING_DOMAIN_GLOBAL_INFO pfnSetRoutingDomainGlobalInfo;
    PPROTOCOL_BUFFER_FREE           pfnBufferFree;
}MPR60_ROUTING_CHARACTERISTICS;

#if MPR60
    typedef MPR60_ROUTING_CHARACTERISTICS MPR_ROUTING_CHARACTERISTICS;
#elif MPR50
    typedef MPR50_ROUTING_CHARACTERISTICS MPR_ROUTING_CHARACTERISTICS;
#endif

typedef MPR_ROUTING_CHARACTERISTICS *PMPR_ROUTING_CHARACTERISTICS;


//
// All routing protocols must export the following entry point.
// The router manager calls this function to allow the routing
// protocol to register
//

#define REGISTER_PROTOCOL_ENTRY_POINT           RegisterProtocol
#define REGISTER_PROTOCOL_ENTRY_POINT_STRING    "RegisterProtocol"

typedef
DWORD
(WINAPI * PREGISTER_PROTOCOL) (
    IN OUT PMPR_ROUTING_CHARACTERISTICS pRoutingChar,
    IN OUT PMPR_SERVICE_CHARACTERISTICS pServiceChar
    );


#ifdef __cplusplus
}
#endif

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4200)
#pragma warning(default:4201)
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif      // _ROUTPROT_H_
