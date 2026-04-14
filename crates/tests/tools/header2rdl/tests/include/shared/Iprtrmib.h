/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    iprtrmib.h

Abstract:
    This file contains:
        o Definitions of the MIB_XX structures passed to and from the IP Router Manager
            to query and set MIB variables handled by the IP Router Manager
        o The #defines for the MIB variables IDs  handled by the IP Router Manager
            and made accessible by the MprAdminMIBXXX APIs
        o The Routing PID of the IP Router Manager (as mentioned in ipinfoid.h)

--*/

#ifndef __ROUTING_IPRTRMIB_H__
#define __ROUTING_IPRTRMIB_H__

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201)

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Included to get the value of MAX_INTERFACE_NAME_LEN                      //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#include <mprapidef.h>

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Included to get the necessary constants                                  //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#include <ipifcons.h>

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// This is the Id for IP Router Manager.  The Router Manager handles        //
// MIB-II, Forwarding MIB and some enterprise specific information.         //
// Calls made with any other ID are passed on to the corresponding protocol //
// For example, an MprAdminMIBXXX call with a protocol ID of PID_IP and    //
// a routing Id of 0xD will be sent to the IP Router Manager and then       //
// forwarded to OSPF                                                        //
// This lives in the same number space as the protocol Ids of RIP, OSPF     //
// etc, so any change made to it should be done keeping this in mind        //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////


#define IPRTRMGR_PID 10000

#ifndef ANY_SIZE

#define ANY_SIZE 1

#endif

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// The following #defines are the Ids of the MIB variables made accessible  //
// to the user via MprAdminMIBXXX Apis.  It will be noticed that these are  //
// not the same as RFC 1213, since the MprAdminMIBXXX APIs work on rows and //
// groups instead of scalar variables                                       //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////


#define IF_NUMBER           0
#define IF_TABLE            (IF_NUMBER          + 1)
#define IF_ROW              (IF_TABLE           + 1)
#define IP_STATS            (IF_ROW             + 1)
#define IP_ADDRTABLE        (IP_STATS           + 1)
#define IP_ADDRROW          (IP_ADDRTABLE       + 1)
#define IP_FORWARDNUMBER    (IP_ADDRROW         + 1)
#define IP_FORWARDTABLE     (IP_FORWARDNUMBER   + 1)
#define IP_FORWARDROW       (IP_FORWARDTABLE    + 1)
#define IP_NETTABLE         (IP_FORWARDROW      + 1)
#define IP_NETROW           (IP_NETTABLE        + 1)
#define ICMP_STATS          (IP_NETROW          + 1)
#define TCP_STATS           (ICMP_STATS         + 1)
#define TCP_TABLE           (TCP_STATS          + 1)
#define TCP_ROW             (TCP_TABLE          + 1)
#define UDP_STATS           (TCP_ROW            + 1)
#define UDP_TABLE           (UDP_STATS          + 1)
#define UDP_ROW             (UDP_TABLE          + 1)
#define MCAST_MFE           (UDP_ROW            + 1)
#define MCAST_MFE_STATS     (MCAST_MFE          + 1)
#define BEST_IF             (MCAST_MFE_STATS    + 1)
#define BEST_ROUTE          (BEST_IF            + 1)
#define PROXY_ARP           (BEST_ROUTE         + 1)
#define MCAST_IF_ENTRY      (PROXY_ARP          + 1)
#define MCAST_GLOBAL        (MCAST_IF_ENTRY     + 1)
#define IF_STATUS           (MCAST_GLOBAL       + 1)
#define MCAST_BOUNDARY      (IF_STATUS          + 1)
#define MCAST_SCOPE         (MCAST_BOUNDARY     + 1)
#define DEST_MATCHING       (MCAST_SCOPE        + 1)
#define DEST_LONGER         (DEST_MATCHING      + 1)
#define DEST_SHORTER        (DEST_LONGER        + 1)
#define ROUTE_MATCHING      (DEST_SHORTER       + 1)
#define ROUTE_LONGER        (ROUTE_MATCHING     + 1)
#define ROUTE_SHORTER       (ROUTE_LONGER       + 1)
#define ROUTE_STATE         (ROUTE_SHORTER      + 1)
#define MCAST_MFE_STATS_EX  (ROUTE_STATE        + 1)
#define IP6_STATS           (MCAST_MFE_STATS_EX + 1)
#define UDP6_STATS          (IP6_STATS          + 1)
#define TCP6_STATS          (UDP6_STATS         + 1)

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define NUMBER_OF_EXPORTED_VARIABLES    (TCP6_STATS + 1)
#else
#define NUMBER_OF_EXPORTED_VARIABLES    (ROUTE_STATE + 1)
#endif


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// MIB_OPAQUE_QUERY is the structure filled in by the user to identify a    //
// MIB variable                                                             //
//                                                                          //
//  dwVarId     ID of MIB Variable (One of the Ids #defined above)          //
//  dwVarIndex  Variable sized array containing the indices needed to       //
//              identify a variable. NOTE: Unlike SNMP we dont require that //
//              a scalar variable be indexed by 0                           //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

typedef struct _MIB_OPAQUE_QUERY
{
    DWORD  dwVarId;
    DWORD  rgdwVarIndex[ANY_SIZE];
}MIB_OPAQUE_QUERY, *PMIB_OPAQUE_QUERY;

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// The following are the structures which are filled in and returned to the //
// user when a query is made, OR  are filled in BY THE USER when a set is   //
// done                                                                     //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#include <ipmib.h>
#include <tcpmib.h>
#include <udpmib.h>

typedef enum _TCP_TABLE_CLASS {
    TCP_TABLE_BASIC_LISTENER,
    TCP_TABLE_BASIC_CONNECTIONS,
    TCP_TABLE_BASIC_ALL,
    TCP_TABLE_OWNER_PID_LISTENER,
    TCP_TABLE_OWNER_PID_CONNECTIONS,
    TCP_TABLE_OWNER_PID_ALL,
    TCP_TABLE_OWNER_MODULE_LISTENER,
    TCP_TABLE_OWNER_MODULE_CONNECTIONS,
    TCP_TABLE_OWNER_MODULE_ALL
} TCP_TABLE_CLASS, *PTCP_TABLE_CLASS;

typedef enum _UDP_TABLE_CLASS {
    UDP_TABLE_BASIC,
    UDP_TABLE_OWNER_PID,
    UDP_TABLE_OWNER_MODULE
} UDP_TABLE_CLASS, *PUDP_TABLE_CLASS;

typedef enum _TCPIP_OWNER_MODULE_INFO_CLASS {
    TCPIP_OWNER_MODULE_INFO_BASIC
} TCPIP_OWNER_MODULE_INFO_CLASS, *PTCPIP_OWNER_MODULE_INFO_CLASS;

typedef struct _TCPIP_OWNER_MODULE_BASIC_INFO {
    PWCHAR pModuleName;
    PWCHAR pModulePath;
} TCPIP_OWNER_MODULE_BASIC_INFO, *PTCPIP_OWNER_MODULE_BASIC_INFO;

typedef struct _MIB_IPMCAST_BOUNDARY
{
    DWORD   dwIfIndex;
    DWORD   dwGroupAddress;
    DWORD   dwGroupMask;
    DWORD   dwStatus;
}MIB_IPMCAST_BOUNDARY, *PMIB_IPMCAST_BOUNDARY;

typedef struct _MIB_IPMCAST_BOUNDARY_TABLE
{
    DWORD       dwNumEntries;
    MIB_IPMCAST_BOUNDARY   table[ANY_SIZE];
}MIB_IPMCAST_BOUNDARY_TABLE, *PMIB_IPMCAST_BOUNDARY_TABLE;

#define SIZEOF_BOUNDARY_TABLE(X) (FIELD_OFFSET(MIB_IPMCAST_BOUNDARY_TABLE,table[0]) + ((X) * sizeof(MIB_IPMCAST_BOUNDARY)) + ALIGN_SIZE)

typedef struct {
    DWORD    dwGroupAddress;
    DWORD    dwGroupMask;
} MIB_BOUNDARYROW, *PMIB_BOUNDARYROW;

// Structure matching what goes in the registry in a block of type
// IP_MCAST_LIMIT_INFO.  This contains the fields of
// MIB_IPMCAST_IF_ENTRY which are configurable.

typedef struct {
    DWORD    dwTtl;
    DWORD    dwRateLimit;
} MIB_MCAST_LIMIT_ROW, *PMIB_MCAST_LIMIT_ROW;

#define MAX_SCOPE_NAME_LEN 255

//
// Scope names are unicode.  SNMP and MZAP use UTF-8 encoding.
//

#define SN_UNICODE
typedef WCHAR   SN_CHAR;
typedef SN_CHAR SCOPE_NAME_BUFFER[MAX_SCOPE_NAME_LEN+1], *SCOPE_NAME;

typedef struct _MIB_IPMCAST_SCOPE
{
    DWORD             dwGroupAddress;
    DWORD             dwGroupMask;
    SCOPE_NAME_BUFFER snNameBuffer;
    DWORD             dwStatus;
}MIB_IPMCAST_SCOPE, *PMIB_IPMCAST_SCOPE;

typedef struct _MIB_IPDESTROW
{
#ifdef __cplusplus
    MIB_IPFORWARDROW  ForwardRow;
#else
    MIB_IPFORWARDROW;
#endif

    DWORD             dwForwardPreference;
    DWORD             dwForwardViewSet;
}MIB_IPDESTROW, *PMIB_IPDESTROW;

typedef struct _MIB_IPDESTTABLE
{
    DWORD             dwNumEntries;
    MIB_IPDESTROW     table[ANY_SIZE];
}MIB_IPDESTTABLE, *PMIB_IPDESTTABLE;

typedef struct _MIB_BEST_IF
{
    DWORD       dwDestAddr;
    DWORD       dwIfIndex;
}MIB_BEST_IF, *PMIB_BEST_IF;

typedef struct _MIB_PROXYARP
{
    DWORD       dwAddress;
    DWORD       dwMask;
    DWORD       dwIfIndex;
}MIB_PROXYARP, *PMIB_PROXYARP;

typedef struct _MIB_IFSTATUS
{
    DWORD       dwIfIndex;
    DWORD       dwAdminStatus;
    DWORD       dwOperationalStatus;
    BOOL        bMHbeatActive;
    BOOL        bMHbeatAlive;
}MIB_IFSTATUS, *PMIB_IFSTATUS;

typedef struct _MIB_ROUTESTATE
{
    BOOL        bRoutesSetToStack;

}MIB_ROUTESTATE, *PMIB_ROUTESTATE;

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// All the info passed to (SET/CREATE) and from (GET/GETNEXT/GETFIRST)      //
// IP Router Manager is encapsulated in the following "discriminated"       //
// union.  To pass, say MIB_IFROW, use the following code                   //
//                                                                          //
//  PMIB_OPAQUE_INFO    pInfo;                                              //
//  PMIB_IFROW          pIfRow;                                             //
//  DWORD rgdwBuff[(MAX_MIB_OFFSET + sizeof(MIB_IFROW))/sizeof(DWORD) + 1]; //
//                                                                          //
//  pInfo   = (PMIB_OPAQUE_INFO)rgdwBuffer;                                 //
//  pIfRow  = (MIB_IFROW *)(pInfo->rgbyData);                               //
//                                                                          //
//  This can also be accomplished by using the following macro              //
//                                                                          //
//  DEFINE_MIB_BUFFER(pInfo,MIB_IFROW, pIfRow);                             //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////


typedef struct _MIB_OPAQUE_INFO
{
    DWORD  dwId;

    union
    {
        ULONGLONG   ullAlign;
        BYTE        rgbyData[1];
    };

}MIB_OPAQUE_INFO, *PMIB_OPAQUE_INFO;

#define MAX_MIB_OFFSET      8

#define MIB_INFO_SIZE(S)                \
    (MAX_MIB_OFFSET + sizeof(S))

#define MIB_INFO_SIZE_IN_DWORDS(S)      \
    ((MIB_INFO_SIZE(S))/sizeof(DWORD) + 1)

#define DEFINE_MIB_BUFFER(X,Y,Z)                                        \
    DWORD        __rgdwBuff[MIB_INFO_SIZE_IN_DWORDS(Y)]; \
    PMIB_OPAQUE_INFO    X = (PMIB_OPAQUE_INFO)__rgdwBuff;               \
    Y *                 Z = (Y *)(X->rgbyData)


#define CAST_MIB_INFO(X,Y,Z)    Z = (Y)(X->rgbyData)

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201)
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif //__ROUTING_IPRTRMIB_H__
