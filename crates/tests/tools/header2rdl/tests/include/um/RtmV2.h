/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    rtmv2.h

Abstract:
    Interface for Routing Table Manager v2 DLL

Author:
    Chaitanya Kodeboyina (chaitk)  01-Jun-1998

Revision History:

--*/

#ifndef __ROUTING_RTMv2_H__
#define __ROUTING_RTMv2_H__

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <in6addr.h>

#ifdef __cplusplus
extern "C"
{
#endif

//
// General Constants defined by the API
//

// Max addr size for an address family
#define RTM_MAX_ADDRESS_SIZE         16

//
// Supported Route Table Views
//
#define RTM_MAX_VIEWS                 32

#define RTM_VIEW_ID_UCAST              0
#define RTM_VIEW_ID_MCAST              1

#define RTM_VIEW_MASK_SIZE          0x20

#define RTM_VIEW_MASK_NONE    0x00000000
#define RTM_VIEW_MASK_ANY     0x00000000

#define RTM_VIEW_MASK_UCAST   0x00000001
#define RTM_VIEW_MASK_MCAST   0x00000002

#define RTM_VIEW_MASK_ALL     0xFFFFFFFF

// Identifies a particular view
typedef INT   RTM_VIEW_ID, *PRTM_VIEW_ID;

// Set of views expressed as a mask
typedef DWORD RTM_VIEW_SET, *PRTM_VIEW_SET;


//
// Profile returned during registration
//
typedef struct _RTM_REGN_PROFILE
{
    UINT            MaxNextHopsInRoute; // Max. number of equal cost nexthops
                                        // in a route, & Max. number of local
                                        // nexthops in any one remote nexthop

    UINT            MaxHandlesInEnum;   // Max. handles returned in one call to
                                        // RtmGetEnumDests, RtmGetChangedDests,
                                        // RtmGetEnumRoutes,RtmGetRoutesInElist

    RTM_VIEW_SET    ViewsSupported;     // Views supported by this addr family

    UINT            NumberOfViews;      // Number of views (# 1s in above mask)
}
RTM_REGN_PROFILE, *PRTM_REGN_PROFILE;


//
// Handles pointing to RTMv2 blocks
//
typedef HANDLE      RTM_ENTITY_HANDLE,
                   *PRTM_ENTITY_HANDLE,
                    RTM_DEST_HANDLE,
                   *PRTM_DEST_HANDLE,
                    RTM_ROUTE_HANDLE,
                   *PRTM_ROUTE_HANDLE,
                    RTM_NEXTHOP_HANDLE,
                   *PRTM_NEXTHOP_HANDLE,
                    RTM_ENUM_HANDLE,
                   *PRTM_ENUM_HANDLE,
                    RTM_ROUTE_LIST_HANDLE,
                   *PRTM_ROUTE_LIST_HANDLE,
                    RTM_NOTIFY_HANDLE,
                   *PRTM_NOTIFY_HANDLE;

//
// Network Address struct for any
// address family that works with
// only contiguous address masks
//
typedef struct _RTM_NET_ADDRESS
{
    USHORT AddressFamily;                  // Type of this net address (IPv4..)

    USHORT NumBits;                        // Number of leading bits in prefix

    UCHAR  AddrBits[RTM_MAX_ADDRESS_SIZE]; // Array of bits that form prefix
}
RTM_NET_ADDRESS, *PRTM_NET_ADDRESS;


//
// IPv4 macros to work on addresses
//

#define RTM_IPV4_MAKE_NET_ADDRESS(NetAddress, Addr, Len)           \
        RTM_IPV4_SET_ADDR_AND_LEN(NetAddress, Addr, Len)


#define RTM_CHECK_NTH_BIT(Value, N, Len)                           \
        if ((Value) & (1 << (N)))                                  \
        {                                                          \
            (Len) += (N); (Value) <<= (N);                         \
        }                                                          \

#define RTM_IPV4_LEN_FROM_MASK(Len, Mask)                          \
        {                                                          \
            ULONG _Temp_ = ntohl(Mask);                            \
                                                                   \
            (Len) = 0;                                             \
                                                                   \
            RTM_CHECK_NTH_BIT(_Temp_, 16, (Len));                  \
            RTM_CHECK_NTH_BIT(_Temp_,  8, (Len));                  \
            RTM_CHECK_NTH_BIT(_Temp_,  4, (Len));                  \
                                                                   \
            while (_Temp_)                                         \
            {                                                      \
                (Len) +=  1; _Temp_ <<=  1;                        \
            }                                                      \
        }                                                          \

#define RTM_IPV4_MASK_FROM_LEN(Len)                                \
        ((Len) ? htonl(~0 << (32 - (Len))): 0);                    \


#define RTM_IPV4_SET_ADDR_AND_LEN(NetAddress, Addr, Len)           \
        (NetAddress)->AddressFamily = AF_INET;                     \
        (NetAddress)->NumBits  = (USHORT) (Len);                   \
        (* (ULONG *) ((NetAddress)->AddrBits)) = (Addr);           \

#define RTM_IPV4_GET_ADDR_AND_LEN(Addr, Len, NetAddress)           \
        (Len) = (NetAddress)->NumBits;                             \
        (Addr) = (* (ULONG *) ((NetAddress)->AddrBits));           \


#define RTM_IPV4_SET_ADDR_AND_MASK(NetAddress, Addr, Mask)         \
        (NetAddress)->AddressFamily = AF_INET;                     \
        (* (ULONG *) ((NetAddress)->AddrBits)) = (Addr);           \
        RTM_IPV4_LEN_FROM_MASK((NetAddress)->NumBits, Mask)

#define RTM_IPV4_GET_ADDR_AND_MASK(Addr, Mask, NetAddress)         \
        (Addr) = (* (ULONG *) ((NetAddress)->AddrBits));           \
        (Mask) = RTM_IPV4_MASK_FROM_LEN((NetAddress)->NumBits);    \

//
// IPv6 Helper functions on addresss
//

DWORD
RtmConvertNetAddressToIpv6AddressAndLength(
    IN  PRTM_NET_ADDRESS         pNetAddress,
    OUT PIN6_ADDR                pAddress,
    OUT PDWORD                   pLength,
    IN  DWORD                    dwAddressSize
    );

DWORD
RtmConvertIpv6AddressAndLengthToNetAddress(
    OUT  PRTM_NET_ADDRESS         pNetAddress,
    IN   IN6_ADDR                 Address,
    IN   DWORD                    dwLength,
    IN   DWORD                    dwAddressSize
    );

//
// IPv6 macros to work on addresses
//
#define IPV6_ADDRESS_LEN_IN_BYTES     16

#define RTM_IPV6_MAKE_NET_ADDRESS(NetAddress, Addr, Len)           \
        RTM_IPV6_SET_ADDR_AND_LEN(NetAddress, Addr, Len)

#define RTM_IPV6_SET_ADDR_AND_LEN(NetAddress, Addr, Len)           \
        (NetAddress)->AddressFamily = AF_INET6;                    \
        (RtmConvertIpv6AddressAndLengthToNetAddress(NetAddress, Addr, Len, IPV6_ADDRESS_LEN_IN_BYTES)) \

#define RTM_IPV6_GET_ADDR_AND_LEN(Addr, Len, NetAddress)           \
        (RtmConvertNetAddressToIpv6AddressAndLength(NetAddress, Addr, Len, IPV6_ADDRESS_LEN_IN_BYTES)) \

//
// This structure encapsulates info
// used in comparing any two routes
// [Preference is impt than metric]
//
typedef struct _RTM_PREF_INFO
{
    ULONG               Metric;         // Routing protocol specific metric
    ULONG               Preference;     // Determined by the router policy
}
RTM_PREF_INFO, *PRTM_PREF_INFO;


//
// List of nexthops used for equal
// cost path in a route or nexthop
//
typedef struct _RTM_NEXTHOP_LIST
{
    USHORT              NumNextHops;    // Num of equal cost next hops in list
    RTM_NEXTHOP_HANDLE  NextHops[1];    // NumNextHops num of next hop handles
}
RTM_NEXTHOP_LIST, *PRTM_NEXTHOP_LIST;


//
// Structure used to exchange dest
// information with RTM entities
//
typedef struct _RTM_DEST_INFO
{
    RTM_DEST_HANDLE     DestHandle;       // Handle to the destination

    RTM_NET_ADDRESS     DestAddress;      // Destination network Address

    FILETIME            LastChanged;      // Last time dest was modified

    RTM_VIEW_SET        BelongsToViews;   // View that dest belongs too

    UINT                NumberOfViews;    // Number of view info slots
    struct
    {
        RTM_VIEW_ID         ViewId;       // View ID for this view info block
        UINT                NumRoutes;    // Number of routes,
        RTM_ROUTE_HANDLE    Route;        // Best route with matching criteria
        RTM_ENTITY_HANDLE   Owner;        // Best Route's Owner,
        DWORD               DestFlags;    // Best Route's Flags, and
        RTM_ROUTE_HANDLE    HoldRoute;    // Holddown route,
    }                   ViewInfo[1];      // in each one of the supported views
}
RTM_DEST_INFO, *PRTM_DEST_INFO;

//
// Macros useful in working on dests
//
#define RTM_BASIC_DEST_INFO_SIZE                                         \
    FIELD_OFFSET(RTM_DEST_INFO, ViewInfo)

#define RTM_DEST_VIEW_INFO_SIZE                                          \
    (sizeof(RTM_DEST_INFO) - RTM_BASIC_DEST_INFO_SIZE)

#define RTM_SIZE_OF_DEST_INFO(NumViews)                                  \
    (RTM_BASIC_DEST_INFO_SIZE + (NumViews) * RTM_DEST_VIEW_INFO_SIZE)

//
// Destination Flags
//
#define RTM_DEST_FLAG_NATURAL_NET   0x01
#define RTM_DEST_FLAG_FWD_ENGIN_ADD 0x02
#define RTM_DEST_FLAG_DONT_FORWARD  0x04

//
// Structure used to exchange route
// information with RTM entities
//
typedef struct _RTM_ROUTE_INFO
{
    //
    // Information that the owner can
    // directly access for read only
    //

    RTM_DEST_HANDLE     DestHandle;       // Handle to owning destination

    RTM_ENTITY_HANDLE   RouteOwner;       // Entity the owns this route

    RTM_NEXTHOP_HANDLE  Neighbour;        // Neighbour we learnt route from

    UCHAR               State;            // See RTM_ROUTE_STATE_* below

    //
    // Information that the owner can
    // directly access for read/write
    //

    UCHAR               Flags1;           // RTM v1 compatibility flags (temp)

    USHORT              Flags;            // See RTM_ROUTE_FLAGS_* below

    RTM_PREF_INFO       PrefInfo;         // Preference and metric for route

    RTM_VIEW_SET        BelongsToViews;   // Views that route belongs to

    PVOID               EntitySpecificInfo; // Owning Entity's private info

    RTM_NEXTHOP_LIST    NextHopsList;     // List of equal cost next-hops
}
RTM_ROUTE_INFO, *PRTM_ROUTE_INFO;

//
// Macros useful in working on routes
//
#define RTM_BASIC_ROUTE_INFO_SIZE                                        \
    FIELD_OFFSET(RTM_ROUTE_INFO, NextHopsList.NumNextHops)

#define RTM_SIZE_OF_ROUTE_INFO(NumHops)                                  \
    (RTM_BASIC_ROUTE_INFO_SIZE + (NumHops) * sizeof(RTM_NEXTHOP_HANDLE))

//
// State of the Route
//
#define RTM_ROUTE_STATE_CREATED        0
#define RTM_ROUTE_STATE_DELETING       1
#define RTM_ROUTE_STATE_DELETED        2


//
// Route Information Flags
//

// Forwarding Flags

#define RTM_ROUTE_FLAGS_MARTIAN        0x0001
#define RTM_ROUTE_FLAGS_BLACKHOLE      0x0002
#define RTM_ROUTE_FLAGS_DISCARD        0x0004
#define RTM_ROUTE_FLAGS_INACTIVE       0x0008

// Unicast Flags

#define RTM_ROUTE_FLAGS_LOCAL          0x0010
#define RTM_ROUTE_FLAGS_REMOTE         0x0020
#define RTM_ROUTE_FLAGS_MYSELF         0x0040

#define RTM_ROUTE_FLAGS_LOOPBACK       0x0080

// Bcast, Mcast Flags

#define RTM_ROUTE_FLAGS_MCAST          0x0100
#define RTM_ROUTE_FLAGS_LOCAL_MCAST    0x0200

#define RTM_ROUTE_FLAGS_LIMITED_BC     0x0400

#define RTM_ROUTE_FLAGS_ZEROS_NETBC    0x1000
#define RTM_ROUTE_FLAGS_ZEROS_SUBNETBC 0x2000
#define RTM_ROUTE_FLAGS_ONES_NETBC     0x4000
#define RTM_ROUTE_FLAGS_ONES_SUBNETBC  0x8000

// Grouping of Flags

#define RTM_ROUTE_FLAGS_FORWARDING        \
        (RTM_ROUTE_FLAGS_MARTIAN        | \
         RTM_ROUTE_FLAGS_BLACKHOLE      | \
         RTM_ROUTE_FLAGS_DISCARD        | \
         RTM_ROUTE_FLAGS_INACTIVE)

#define RTM_ROUTE_FLAGS_ANY_UNICAST       \
        (RTM_ROUTE_FLAGS_LOCAL          | \
         RTM_ROUTE_FLAGS_REMOTE         | \
         RTM_ROUTE_FLAGS_MYSELF)

#define RTM_ROUTE_FLAGS_ANY_MCAST         \
        (RTM_ROUTE_FLAGS_MCAST          | \
         RTM_ROUTE_FLAGS_LOCAL_MCAST)

#define RTM_ROUTE_FLAGS_SUBNET_BCAST      \
        (RTM_ROUTE_FLAGS_ONES_SUBNET_BC | \
         RTM_ROUTE_FLAGS_ZEROS_SUBNETBC)

#define RTM_ROUTE_FLAGS_NET_BCAST         \
        (RTM_ROUTE_FLAGS_ONES_NETBC     | \
         RTM_ROUTE_FLAGS_ZEROS_NETBC)

#define RTM_ROUTE_FLAGS_ANY_BCAST         \
        (RTM_ROUTE_FLAGS_LIMITED_BC     | \
         RTM_ROUTE_FLAGS_ONES_NETBC     | \
         RTM_ROUTE_FLAGS_ONES_SUBNET_BC | \
         RTM_ROUTE_FLAGS_ZEROS_NETBC    | \
         RTM_ROUTE_FLAGS_ZEROS_SUBNETBC)

//
// Structure used to exchange next-hop
// information with RTM entities
//
typedef struct _RTM_NEXTHOP_INFO
{
    //
    // Information that the owner can
    // directly access for read only
    //

    RTM_NET_ADDRESS     NextHopAddress;   // Net Address for this next hop

    RTM_ENTITY_HANDLE   NextHopOwner;     // Entity that owns this next hop

    ULONG               InterfaceIndex;   // Outgoing interface index
                                          // '0' for a remote nexthop

    USHORT              State;            // See RTM_NEXTHOP_STATE_* below

    //
    // Information that the owner can
    // directly access for read/write
    //

    USHORT              Flags;            // See RTM_NEXTHOP_FLAGS_* below

    PVOID               EntitySpecificInfo; // Owning Entity's private info

    RTM_DEST_HANDLE     RemoteNextHop;    // Handle to dest with nexthop addr
                                          // [ Not used for a local nexthop ]
}
RTM_NEXTHOP_INFO, *PRTM_NEXTHOP_INFO;

//
// Next Hop State
//

#define RTM_NEXTHOP_STATE_CREATED      0
#define RTM_NEXTHOP_STATE_DELETED      1

//
// Next Hop Flags
//

#define RTM_NEXTHOP_FLAGS_REMOTE  0x0001
#define RTM_NEXTHOP_FLAGS_DOWN    0x0002


//
// Entity Registration Related Defns
//

//
// Info that uniquely identifies an entity
//

// Disable warnings for unnamed structs
#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable : 4201)

typedef struct _RTM_ENTITY_ID
{
    union
    {
        struct
        {
            ULONG    EntityProtocolId;  // Entity's Protocol ID (RIP,OSPF...)
            ULONG    EntityInstanceId;  // Entity's Protocol Instance
        };

        ULONGLONG    EntityId;          // Protocol ID and Instance
    };
}
RTM_ENTITY_ID, *PRTM_ENTITY_ID;

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default : 4201)
#endif

//
// Structure used to exchange entity
// information with RTM entities
//
typedef struct _RTM_ENTITY_INFO
{
    USHORT         RtmInstanceId;       // RTM Instance that it registered with
    USHORT         AddressFamily;       // Entity's Address Family

    RTM_ENTITY_ID  EntityId;            // Uniquely identifies an entity
}
RTM_ENTITY_INFO, *PRTM_ENTITY_INFO;


//
// Event in the RTM involving an entity
//
typedef enum _RTM_EVENT_TYPE
{
    RTM_ENTITY_REGISTERED,
    RTM_ENTITY_DEREGISTERED,
    RTM_ROUTE_EXPIRED,
    RTM_CHANGE_NOTIFICATION
}
RTM_EVENT_TYPE, *PRTM_EVENT_TYPE;

//
// Entity event inform callback
//
// Used to inform entities of
// new entities registering,
// or entities deregistering
//
typedef
DWORD
(WINAPI * _EVENT_CALLBACK) (
     IN  RTM_ENTITY_HANDLE    RtmRegHandle,  // Callee's Registration Handle
     IN  RTM_EVENT_TYPE       EventType,
     IN  PVOID                Context1,
     IN  PVOID                Context2
     );

typedef _EVENT_CALLBACK RTM_EVENT_CALLBACK,
                      *PRTM_EVENT_CALLBACK;


//
// Methods exported by a registered entity
//

#define METHOD_TYPE_ALL_METHODS      0xFFFFFFFF

#define METHOD_RIP2_NEIGHBOUR_ADDR   0x00000001
#define METHOD_RIP2_OUTBOUND_INTF    0x00000002
#define METHOD_RIP2_ROUTE_TAG        0x00000004
#define METHOD_RIP2_ROUTE_TIMESTAMP  0x00000008

#define METHOD_BGP4_AS_PATH          0x00000001
#define METHOD_BGP4_PEER_ID          0x00000002
#define METHOD_BGP4_PA_ORIGIN        0x00000004
#define METHOD_BGP4_NEXTHOP_ATTR     0x00000008

typedef DWORD      RTM_ENTITY_METHOD_TYPE,
                 *PRTM_ENTITY_METHOD_TYPE;


//
// Generic Input Structure for entity methods
//
typedef struct _RTM_ENTITY_METHOD_INPUT
{
    RTM_ENTITY_METHOD_TYPE MethodType;    // Type identifying the method
    UINT                   InputSize;     // Input Data Size
    UCHAR                  InputData[1];  // Input Data Buffer
}
RTM_ENTITY_METHOD_INPUT, *PRTM_ENTITY_METHOD_INPUT;

//
// Generic Output Structure for entity methods
//
typedef struct _RTM_ENTITY_METHOD_OUTPUT
{
    RTM_ENTITY_METHOD_TYPE MethodType;    // Type identifying the method
    DWORD                  MethodStatus;  // Return Status of method
    UINT                   OutputSize;    // Output Data Size
    UCHAR                  OutputData[1]; // Output Data Buffer
}
RTM_ENTITY_METHOD_OUTPUT, *PRTM_ENTITY_METHOD_OUTPUT;

//
// Common prototype for entity methods
//
typedef
VOID
(WINAPI * _ENTITY_METHOD) (
    IN  RTM_ENTITY_HANDLE         CallerHandle,
    IN  RTM_ENTITY_HANDLE         CalleeHandle,
    IN  RTM_ENTITY_METHOD_INPUT  *Input,
    OUT RTM_ENTITY_METHOD_OUTPUT *Output
    );

typedef _ENTITY_METHOD RTM_ENTITY_EXPORT_METHOD,
                     *PRTM_ENTITY_EXPORT_METHOD;

//
// Set of exported entity methods
//
typedef struct _RTM_ENTITY_EXPORT_METHODS
{
    UINT                     NumMethods;
    RTM_ENTITY_EXPORT_METHOD Methods[1];
}
RTM_ENTITY_EXPORT_METHODS, *PRTM_ENTITY_EXPORT_METHODS;

//
// To toggle method blocking on dests, routes and nexthops
//
#define RTM_RESUME_METHODS             0
#define RTM_BLOCK_METHODS              1


//
// I/O Flags when route is added/updated
//
typedef DWORD    RTM_ROUTE_CHANGE_FLAGS,
               *PRTM_ROUTE_CHANGE_FLAGS;

#define RTM_ROUTE_CHANGE_FIRST      0x01
#define RTM_ROUTE_CHANGE_NEW        0x02
#define RTM_ROUTE_CHANGE_BEST 0x00010000

//
// Output flags when nexthop is added
//
typedef DWORD  RTM_NEXTHOP_CHANGE_FLAGS,
             *PRTM_NEXTHOP_CHANGE_FLAGS;

#define RTM_NEXTHOP_CHANGE_NEW      0x01


//
// Definitions relating to RIB queries
//

//
// Flags used to matching routes in RIB
//
typedef DWORD           RTM_MATCH_FLAGS,
                      *PRTM_MATCH_FLAGS;

#define RTM_MATCH_NONE        0x00000000
#define RTM_MATCH_OWNER       0x00000001
#define RTM_MATCH_NEIGHBOUR   0x00000002
#define RTM_MATCH_PREF        0x00000004
#define RTM_MATCH_NEXTHOP     0x00000008
#define RTM_MATCH_INTERFACE   0x00000010
#define RTM_MATCH_FULL        0x0000FFFF

//
// Flags to specify route being queried
//
#define RTM_BEST_PROTOCOL    (ULONG)   0
#define RTM_THIS_PROTOCOL    (ULONG)  ~0


//
// Definitions relating to enumerations
//

typedef DWORD            RTM_ENUM_FLAGS,
                       *PRTM_ENUM_FLAGS;

// Enumeration Flags

#define RTM_ENUM_START        0x00000000
#define RTM_ENUM_NEXT         0x00000001
#define RTM_ENUM_RANGE        0x00000002

#define RTM_ENUM_ALL_DESTS    0x00000000
#define RTM_ENUM_OWN_DESTS    0x01000000

#define RTM_ENUM_ALL_ROUTES   0x00000000
#define RTM_ENUM_OWN_ROUTES   0x00010000


//
// Definitions relating to notifications
//

// Notify Flags is composed as follows -
// (Change Types | Dests) interested in.

typedef DWORD          RTM_NOTIFY_FLAGS,
                     *PRTM_NOTIFY_FLAGS;

// Change Types to notify about

#define RTM_NUM_CHANGE_TYPES            3

#define RTM_CHANGE_TYPE_ALL        0x0001
#define RTM_CHANGE_TYPE_BEST       0x0002
#define RTM_CHANGE_TYPE_FORWARDING 0x0004

// Dests whose changes to notify

#define RTM_NOTIFY_ONLY_MARKED_DESTS 0x00010000


//
// Registration API Prototypes
//

DWORD
WINAPI
RtmRegisterEntity (
    IN      PRTM_ENTITY_INFO                RtmEntityInfo,
    IN      PRTM_ENTITY_EXPORT_METHODS      ExportMethods OPTIONAL,
    IN      RTM_EVENT_CALLBACK              EventCallback,
    IN      BOOL                            ReserveOpaquePointer,
    OUT     PRTM_REGN_PROFILE               RtmRegProfile,
    OUT     PRTM_ENTITY_HANDLE              RtmRegHandle
    );

DWORD
WINAPI
RtmDeregisterEntity (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle
    );

DWORD
WINAPI
RtmGetRegisteredEntities (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN OUT  PUINT                           NumEntities,
    OUT     PRTM_ENTITY_HANDLE              EntityHandles,
    OUT     PRTM_ENTITY_INFO                EntityInfos OPTIONAL
    );

DWORD
WINAPI
RtmReleaseEntities (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      UINT                            NumEntities,
    IN      PRTM_ENTITY_HANDLE              EntityHandles
    );

//
// Opaque Ptr APIs
//

DWORD
WINAPI
RtmLockDestination(
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_DEST_HANDLE                 DestHandle,
    IN      BOOL                            Exclusive,
    IN      BOOL                            LockDest
    );

DWORD
WINAPI
RtmGetOpaqueInformationPointer (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_DEST_HANDLE                 DestHandle,
    OUT     PVOID                          *OpaqueInfoPointer
    );

//
// Export Method API Prototypes
//

DWORD
WINAPI
RtmGetEntityMethods (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ENTITY_HANDLE               EntityHandle,
    IN OUT  PUINT                           NumMethods,
    OUT     PRTM_ENTITY_EXPORT_METHOD       ExptMethods
    );

DWORD
WINAPI
RtmInvokeMethod (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ENTITY_HANDLE               EntityHandle,
    IN      PRTM_ENTITY_METHOD_INPUT        Input,
    IN OUT  PUINT                           OutputSize,
    OUT     PRTM_ENTITY_METHOD_OUTPUT       Output
    );

DWORD
WINAPI
RtmBlockMethods (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      HANDLE                          TargetHandle OPTIONAL,
    IN      UCHAR                           TargetType   OPTIONAL,
    IN      DWORD                           BlockingFlag
    );

//
// Handle to Info Structures
//

DWORD
WINAPI
RtmGetEntityInfo (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ENTITY_HANDLE               EntityHandle,
    OUT     PRTM_ENTITY_INFO                EntityInfo
    );

DWORD
WINAPI
RtmGetDestInfo (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_DEST_HANDLE                 DestHandle,
    IN      ULONG                           ProtocolId,
    IN      RTM_VIEW_SET                    TargetViews,
    OUT     PRTM_DEST_INFO                  DestInfo
    );

DWORD
WINAPI
RtmGetRouteInfo (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ROUTE_HANDLE                RouteHandle,
    OUT     PRTM_ROUTE_INFO                 RouteInfo   OPTIONAL,
    OUT     PRTM_NET_ADDRESS                DestAddress OPTIONAL
    );

DWORD
WINAPI
RtmGetNextHopInfo (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_NEXTHOP_HANDLE              NextHopHandle,
    OUT     PRTM_NEXTHOP_INFO               NextHopInfo
    );

DWORD
WINAPI
RtmReleaseEntityInfo (
    IN      RTM_ENTITY_HANDLE              RtmRegHandle,
    IN      PRTM_ENTITY_INFO               EntityInfo
    );

DWORD
WINAPI
RtmReleaseDestInfo (
    IN      RTM_ENTITY_HANDLE              RtmRegHandle,
    IN      PRTM_DEST_INFO                 DestInfo
    );

DWORD
WINAPI
RtmReleaseRouteInfo (
    IN      RTM_ENTITY_HANDLE              RtmRegHandle,
    IN      PRTM_ROUTE_INFO                RouteInfo
    );

DWORD
WINAPI
RtmReleaseNextHopInfo (
    IN      RTM_ENTITY_HANDLE              RtmRegHandle,
    IN      PRTM_NEXTHOP_INFO              NextHopInfo
    );


//
// RIB Insert/Delete API Prototypes
//

DWORD
WINAPI
RtmAddRouteToDest (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN OUT  PRTM_ROUTE_HANDLE               RouteHandle     OPTIONAL,
    IN      PRTM_NET_ADDRESS                DestAddress,
    IN      PRTM_ROUTE_INFO                 RouteInfo,
    IN      ULONG                           TimeToLive,
    IN      RTM_ROUTE_LIST_HANDLE           RouteListHandle OPTIONAL,
    IN      RTM_NOTIFY_FLAGS                NotifyType,
    IN      RTM_NOTIFY_HANDLE               NotifyHandle    OPTIONAL,
    IN OUT  PRTM_ROUTE_CHANGE_FLAGS         ChangeFlags
    );

DWORD
WINAPI
RtmDeleteRouteToDest (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ROUTE_HANDLE                RouteHandle,
    OUT     PRTM_ROUTE_CHANGE_FLAGS         ChangeFlags
    );

DWORD
WINAPI
RtmHoldDestination (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_DEST_HANDLE                 DestHandle,
    IN      RTM_VIEW_SET                    TargetViews,
    IN      ULONG                           HoldTime
    );

DWORD
WINAPI
RtmGetRoutePointer (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ROUTE_HANDLE                RouteHandle,
    OUT     PRTM_ROUTE_INFO                *RoutePointer
    );

DWORD
WINAPI
RtmLockRoute(
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ROUTE_HANDLE                RouteHandle,
    IN      BOOL                            Exclusive,
    IN      BOOL                            LockRoute,
    OUT     PRTM_ROUTE_INFO                *RoutePointer OPTIONAL
    );

DWORD
WINAPI
RtmUpdateAndUnlockRoute(
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ROUTE_HANDLE                RouteHandle,
    IN      ULONG                           TimeToLive,
    IN      RTM_ROUTE_LIST_HANDLE           RouteListHandle OPTIONAL,
    IN      RTM_NOTIFY_FLAGS                NotifyType,
    IN      RTM_NOTIFY_HANDLE               NotifyHandle    OPTIONAL,
    OUT     PRTM_ROUTE_CHANGE_FLAGS         ChangeFlags
    );

//
// RIB Query API Prototypes
//

DWORD
WINAPI
RtmGetExactMatchDestination (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      PRTM_NET_ADDRESS                DestAddress,
    IN      ULONG                           ProtocolId,
    IN      RTM_VIEW_SET                    TargetViews,
    OUT     PRTM_DEST_INFO                  DestInfo
    );

DWORD
WINAPI
RtmGetMostSpecificDestination (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      PRTM_NET_ADDRESS                DestAddress,
    IN      ULONG                           ProtocolId,
    IN      RTM_VIEW_SET                    TargetViews,
    OUT     PRTM_DEST_INFO                  DestInfo
    );

DWORD
WINAPI
RtmGetLessSpecificDestination (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_DEST_HANDLE                 DestHandle,
    IN      ULONG                           ProtocolId,
    IN      RTM_VIEW_SET                    TargetViews,
    OUT     PRTM_DEST_INFO                  DestInfo
    );

DWORD
WINAPI
RtmGetExactMatchRoute (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      PRTM_NET_ADDRESS                DestAddress,
    IN      RTM_MATCH_FLAGS                 MatchingFlags,
    IN OUT  PRTM_ROUTE_INFO                 RouteInfo,
    IN      ULONG                           InterfaceIndex,
    IN      RTM_VIEW_SET                    TargetViews,
    OUT     PRTM_ROUTE_HANDLE               RouteHandle
    );

DWORD
WINAPI
RtmIsBestRoute (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ROUTE_HANDLE                RouteHandle,
    OUT     PRTM_VIEW_SET                   BestInViews
    );

//
// NextHop Object API Prototypes
//

DWORD
WINAPI
RtmAddNextHop (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      PRTM_NEXTHOP_INFO               NextHopInfo,
    IN OUT  PRTM_NEXTHOP_HANDLE             NextHopHandle OPTIONAL,
    OUT     PRTM_NEXTHOP_CHANGE_FLAGS       ChangeFlags
    );

DWORD
WINAPI
RtmFindNextHop (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      PRTM_NEXTHOP_INFO               NextHopInfo,
    OUT     PRTM_NEXTHOP_HANDLE             NextHopHandle,
    OUT     PRTM_NEXTHOP_INFO              *NextHopPointer OPTIONAL
    );

DWORD
WINAPI
RtmDeleteNextHop (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_NEXTHOP_HANDLE              NextHopHandle  OPTIONAL,
    IN      PRTM_NEXTHOP_INFO               NextHopInfo
    );

DWORD
WINAPI
RtmGetNextHopPointer (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_NEXTHOP_HANDLE              NextHopHandle,
    OUT     PRTM_NEXTHOP_INFO              *NextHopPointer
    );

DWORD
WINAPI
RtmLockNextHop(
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_NEXTHOP_HANDLE              NextHopHandle,
    IN      BOOL                            Exclusive,
    IN      BOOL                            LockNextHop,
    OUT     PRTM_NEXTHOP_INFO              *NextHopPointer OPTIONAL
    );


//
// Enumeration API Prototypes
//

DWORD
WINAPI
RtmCreateDestEnum (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_VIEW_SET                    TargetViews,
    IN      RTM_ENUM_FLAGS                  EnumFlags,
    IN      PRTM_NET_ADDRESS                NetAddress,
    IN      ULONG                           ProtocolId,
    OUT     PRTM_ENUM_HANDLE                RtmEnumHandle
    );

DWORD
WINAPI
RtmGetEnumDests (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ENUM_HANDLE                 EnumHandle,
    IN OUT  PUINT                           NumDests,
    OUT     PRTM_DEST_INFO                  DestInfos
    );

DWORD
WINAPI
RtmReleaseDests (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      UINT                            NumDests,
    IN      PRTM_DEST_INFO                  DestInfos
    );

DWORD
WINAPI
RtmCreateRouteEnum (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_DEST_HANDLE                 DestHandle        OPTIONAL,
    IN      RTM_VIEW_SET                    TargetViews,
    IN      RTM_ENUM_FLAGS                  EnumFlags,
    IN      PRTM_NET_ADDRESS                StartDest         OPTIONAL,
    IN      RTM_MATCH_FLAGS                 MatchingFlags,
    IN      PRTM_ROUTE_INFO                 CriteriaRoute     OPTIONAL,
    IN      ULONG                           CriteriaInterface OPTIONAL,
    OUT     PRTM_ENUM_HANDLE                RtmEnumHandle
    );

DWORD
WINAPI
RtmGetEnumRoutes (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ENUM_HANDLE                 EnumHandle,
    IN OUT  PUINT                           NumRoutes,
    OUT     PRTM_ROUTE_HANDLE               RouteHandles
    );

DWORD
WINAPI
RtmReleaseRoutes (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      UINT                            NumRoutes,
    IN      PRTM_ROUTE_HANDLE               RouteHandles
    );

DWORD
WINAPI
RtmCreateNextHopEnum (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ENUM_FLAGS                  EnumFlags,
    IN      PRTM_NET_ADDRESS                NetAddress,
    OUT     PRTM_ENUM_HANDLE                RtmEnumHandle
    );

DWORD
WINAPI
RtmGetEnumNextHops (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ENUM_HANDLE                 EnumHandle,
    IN OUT  PUINT                           NumNextHops,
    OUT     PRTM_NEXTHOP_HANDLE             NextHopHandles
    );

DWORD
WINAPI
RtmReleaseNextHops (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      UINT                            NumNextHops,
    IN      PRTM_NEXTHOP_HANDLE             NextHopHandles
    );

DWORD
WINAPI
RtmDeleteEnumHandle (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ENUM_HANDLE                 EnumHandle
    );


//
// Change Notification APIs
//

DWORD
WINAPI
RtmRegisterForChangeNotification (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_VIEW_SET                    TargetViews,
    IN      RTM_NOTIFY_FLAGS                NotifyFlags,
    IN      PVOID                           NotifyContext,
    OUT     PRTM_NOTIFY_HANDLE              NotifyHandle
    );

DWORD
WINAPI
RtmGetChangedDests (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_NOTIFY_HANDLE               NotifyHandle,
    IN OUT  PUINT                           NumDests,
    OUT     PRTM_DEST_INFO                  ChangedDests
    );

DWORD
WINAPI
RtmReleaseChangedDests (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_NOTIFY_HANDLE               NotifyHandle,
    IN      UINT                            NumDests,
    IN      PRTM_DEST_INFO                  ChangedDests
    );

DWORD
WINAPI
RtmIgnoreChangedDests (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_NOTIFY_HANDLE               NotifyHandle,
    IN      UINT                            NumDests,
    IN      PRTM_DEST_HANDLE                ChangedDests
    );

DWORD
WINAPI
RtmGetChangeStatus (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_NOTIFY_HANDLE               NotifyHandle,
    IN      RTM_DEST_HANDLE                 DestHandle,
    OUT     PBOOL                           ChangeStatus
    );

DWORD
WINAPI
RtmMarkDestForChangeNotification (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_NOTIFY_HANDLE               NotifyHandle,
    IN      RTM_DEST_HANDLE                 DestHandle,
    IN      BOOL                            MarkDest
    );

DWORD
WINAPI
RtmIsMarkedForChangeNotification (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_NOTIFY_HANDLE               NotifyHandle,
    IN      RTM_DEST_HANDLE                 DestHandle,
    OUT     PBOOL                           DestMarked
    );

DWORD
WINAPI
RtmDeregisterFromChangeNotification (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_NOTIFY_HANDLE               NotifyHandle
    );


//
// Entity Specific List APIs
//

DWORD
WINAPI
RtmCreateRouteList (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    OUT     PRTM_ROUTE_LIST_HANDLE          RouteListHandle
    );

DWORD
WINAPI
RtmInsertInRouteList (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ROUTE_LIST_HANDLE           RouteListHandle OPTIONAL,
    IN      UINT                            NumRoutes,
    IN      PRTM_ROUTE_HANDLE               RouteHandles
    );

DWORD
WINAPI
RtmCreateRouteListEnum (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ROUTE_LIST_HANDLE           RouteListHandle,
    OUT     PRTM_ENUM_HANDLE                RtmEnumHandle
    );

DWORD
WINAPI
RtmGetListEnumRoutes (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ENUM_HANDLE                 EnumHandle,
    IN OUT  PUINT                           NumRoutes,
    _Out_writes_(*NumRoutes) OUT     PRTM_ROUTE_HANDLE               RouteHandles
    );

DWORD
WINAPI
RtmDeleteRouteList (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      RTM_ROUTE_LIST_HANDLE           RouteListHandle
    );

//
// Handle Management APIs
//

DWORD
WINAPI
RtmReferenceHandles (
    IN      RTM_ENTITY_HANDLE               RtmRegHandle,
    IN      UINT                            NumHandles,
    IN      HANDLE                         *RtmHandles
    );

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //__ROUTING_RTMv2_H__
