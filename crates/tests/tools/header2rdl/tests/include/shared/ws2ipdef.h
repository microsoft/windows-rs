/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ws2ipdef.h

Abstract:

    This file contains TCP/IP specific information for use
    by WinSock2 compatible applications.

   Copyright (c) Microsoft Corporation. All rights reserved.

    To provide the backward compatibility, all the TCP/IP
    specific definitions that were included in the WINSOCK.H
    file are now included in WINSOCK2.H file. WS2TCPIP.H
    file includes only the definitions  introduced in the
    "WinSock 2 Protocol-Specific Annex" document.

    Rev 0.3 Nov 13, 1995
        Rev 0.4 Dec 15, 1996

Environment:

    user mode or kernel mode

--*/

#ifndef _WS2IPDEF_
#define _WS2IPDEF_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#ifdef __cplusplus
extern "C" {
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma warning(push)
#pragma warning(disable:4201)
#pragma warning(disable:4127) // conditional expression is constant

#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable: 24002, "This code requires explicit usage of IPv4 address types.")
#endif

#ifndef WS2IPDEF_ASSERT
#define WS2IPDEF_ASSERT(exp) ((VOID) 0)
#endif

#ifdef _MSC_VER
#define WS2TCPIP_INLINE __inline
#else
#define WS2TCPIP_INLINE extern inline /* GNU style */
#endif

#include <in6addr.h>

//
// Old IPv6 socket address structure (retained for sockaddr_gen definition).
//

struct sockaddr_in6_old {
    SHORT sin6_family;          // AF_INET6.
    USHORT sin6_port;           // Transport level port number.
    ULONG sin6_flowinfo;        // IPv6 flow information.
    IN6_ADDR sin6_addr;         // IPv6 address.
};

typedef union sockaddr_gen {
    struct sockaddr Address;
    struct sockaddr_in AddressIn;
    struct sockaddr_in6_old AddressIn6;
} sockaddr_gen;

//
// Structure to keep interface specific information
//

typedef struct _INTERFACE_INFO {
    ULONG iiFlags;              // Interface flags.
    sockaddr_gen iiAddress;     // Interface address.
    sockaddr_gen iiBroadcastAddress; // Broadcast address.
    sockaddr_gen iiNetmask;     // Network mask.
} INTERFACE_INFO, FAR *LPINTERFACE_INFO;

//
// New structure that does not have dependency on the address size.
//

typedef struct _INTERFACE_INFO_EX {
    ULONG iiFlags;              // Interface flags.
    SOCKET_ADDRESS iiAddress;   // Interface address.
    SOCKET_ADDRESS iiBroadcastAddress; // Broadcast address.
    SOCKET_ADDRESS iiNetmask;   // Network mask.
} INTERFACE_INFO_EX, FAR *LPINTERFACE_INFO_EX;

//
// Possible flags for the  iiFlags - bitmask.
//

#define IFF_UP              0x00000001 // Interface is up.
#define IFF_BROADCAST       0x00000002 // Broadcast is  supported.
#define IFF_LOOPBACK        0x00000004 // This is loopback interface.
#define IFF_POINTTOPOINT    0x00000008 // This is point-to-point interface.
#define IFF_MULTICAST       0x00000010 // Multicast is supported.

//
// Path MTU discovery states.
//

typedef enum _PMTUD_STATE {
    IP_PMTUDISC_NOT_SET,
    IP_PMTUDISC_DO,
    IP_PMTUDISC_DONT,
    IP_PMTUDISC_PROBE,
    IP_PMTUDISC_MAX
} PMTUD_STATE, *PPMTUD_STATE;

//
// Options to use with [gs]etsockopt at the IPPROTO_IP level.
// The values should be consistent with the IPv6 equivalents.
//
#define IP_OPTIONS                 1 // Set/get IP options.
#define IP_HDRINCL                 2 // Header is included with data.
#define IP_TOS                     3 // IP type of service.
#define IP_TTL                     4 // IP TTL (hop limit).
#define IP_MULTICAST_IF            9 // IP multicast interface.
#define IP_MULTICAST_TTL          10 // IP multicast TTL (hop limit).
#define IP_MULTICAST_LOOP         11 // IP multicast loopback.
#define IP_ADD_MEMBERSHIP         12 // Add an IP group membership.
#define IP_DROP_MEMBERSHIP        13 // Drop an IP group membership.
#define IP_DONTFRAGMENT           14 // Don't fragment IP datagrams.
#define IP_ADD_SOURCE_MEMBERSHIP  15 // Join IP group/source.
#define IP_DROP_SOURCE_MEMBERSHIP 16 // Leave IP group/source.
#define IP_BLOCK_SOURCE           17 // Block IP group/source.
#define IP_UNBLOCK_SOURCE         18 // Unblock IP group/source.
#define IP_PKTINFO                19 // Receive packet information.
#define IP_HOPLIMIT               21 // Receive packet hop limit.
#define IP_RECVTTL                21 // Receive packet Time To Live (TTL).
#define IP_RECEIVE_BROADCAST      22 // Allow/block broadcast reception.
#define IP_RECVIF                 24 // Receive arrival interface.
#define IP_RECVDSTADDR            25 // Receive destination address.
#define IP_IFLIST                 28 // Enable/Disable an interface list.
#define IP_ADD_IFLIST             29 // Add an interface list entry.
#define IP_DEL_IFLIST             30 // Delete an interface list entry.
#define IP_UNICAST_IF             31 // IP unicast interface.
#define IP_RTHDR                  32 // Set/get IPv6 routing header.
#define IP_GET_IFLIST             33 // Get an interface list.
#define IP_RECVRTHDR              38 // Receive the routing header.
#define IP_TCLASS                 39 // Packet traffic class.
#define IP_RECVTCLASS             40 // Receive packet traffic class.
#define IP_RECVTOS                40 // Receive packet Type Of Service (TOS).
#define IP_ORIGINAL_ARRIVAL_IF    47 // Original Arrival Interface Index.
#define IP_ECN                    50 // IP ECN codepoint.
#define IP_RECVECN                50 // Receive ECN codepoints in the IP header.
#define IP_PKTINFO_EX             51 // Receive extended packet information.
#define IP_WFP_REDIRECT_RECORDS   60 // WFP's Connection Redirect Records.
#define IP_WFP_REDIRECT_CONTEXT   70 // WFP's Connection Redirect Context.
#define IP_MTU_DISCOVER           71 // Set/get path MTU discover state.
#define IP_MTU                    73 // Get path MTU.
#define IP_NRT_INTERFACE          74 // Set NRT interface constraint (outbound).
#define IP_RECVERR                75 // Receive ICMP errors.
#define IP_USER_MTU               76 // Set/get app defined upper bound IP layer MTU.

#define IP_UNSPECIFIED_TYPE_OF_SERVICE -1
#define IP_UNSPECIFIED_USER_MTU MAXULONG

#define IPV6_ADDRESS_BITS RTL_BITS_OF(IN6_ADDR)

//
// IPv6 socket address structure, RFC 3493.
//

//
// NB: The LH version of sockaddr_in6 has the struct tag sockaddr_in6 rather
// than sockaddr_in6_lh.  This is to make sure that standard sockets apps
// that conform to RFC 2553 (Basic Socket Interface Extensions for IPv6).
//
#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
typedef struct sockaddr_in6 {
    ADDRESS_FAMILY sin6_family; // AF_INET6.
    USHORT sin6_port;           // Transport level port number.
    ULONG  sin6_flowinfo;       // IPv6 flow information.
    IN6_ADDR sin6_addr;         // IPv6 address.
    union {
        ULONG sin6_scope_id;     // Set of interfaces for a scope.
        SCOPE_ID sin6_scope_struct;
    };
} SOCKADDR_IN6_LH, *PSOCKADDR_IN6_LH, FAR *LPSOCKADDR_IN6_LH;

typedef struct sockaddr_in6_w2ksp1 {
    short   sin6_family;        /* AF_INET6 */
    USHORT sin6_port;          /* Transport level port number */
    ULONG  sin6_flowinfo;      /* IPv6 flow information */
    struct in6_addr sin6_addr;  /* IPv6 address */
    ULONG sin6_scope_id;       /* set of interfaces for a scope */
} SOCKADDR_IN6_W2KSP1, *PSOCKADDR_IN6_W2KSP1, FAR *LPSOCKADDR_IN6_W2KSP1;

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef SOCKADDR_IN6_LH SOCKADDR_IN6;
typedef SOCKADDR_IN6_LH *PSOCKADDR_IN6;
typedef SOCKADDR_IN6_LH FAR *LPSOCKADDR_IN6;
#elif(NTDDI_VERSION >= NTDDI_WIN2KSP1)
typedef SOCKADDR_IN6_W2KSP1 SOCKADDR_IN6;
typedef SOCKADDR_IN6_W2KSP1 *PSOCKADDR_IN6;
typedef SOCKADDR_IN6_W2KSP1 FAR *LPSOCKADDR_IN6;
#else
typedef SOCKADDR_IN6_LH SOCKADDR_IN6;
typedef SOCKADDR_IN6_LH *PSOCKADDR_IN6;
typedef SOCKADDR_IN6_LH FAR *LPSOCKADDR_IN6;
#endif

typedef union _SOCKADDR_INET {
    SOCKADDR_IN Ipv4;
    SOCKADDR_IN6 Ipv6;
    ADDRESS_FAMILY si_family;
} SOCKADDR_INET, *PSOCKADDR_INET;

//
// Structure to hold a pair of source, destination addresses.
//
typedef struct _sockaddr_in6_pair
{
    PSOCKADDR_IN6 SourceAddress;
    PSOCKADDR_IN6 DestinationAddress;
} SOCKADDR_IN6_PAIR, *PSOCKADDR_IN6_PAIR;

//
// Macro that works for both IPv4 and IPv6
//
#define SS_PORT(ssp) (((PSOCKADDR_IN)(ssp))->sin_port)

#if (NTDDI_VERSION >= NTDDI_WIN2KSP1)
//
// N.B. These addresses are in network byte order.
//

#define IN6ADDR_ANY_INIT {{{ 0 }}}

#define IN6ADDR_LOOPBACK_INIT { 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1 }

#define IN6ADDR_ALLNODESONNODE_INIT { \
    0xff, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, \
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01 \
}

#define IN6ADDR_ALLNODESONLINK_INIT { \
    0xff, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, \
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01 \
}

#define IN6ADDR_ALLROUTERSONLINK_INIT { \
    0xff, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, \
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02 \
}

#define IN6ADDR_ALLMLDV2ROUTERSONLINK_INIT { \
    0xff, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, \
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16 \
}

#define IN6ADDR_TEREDOINITIALLINKLOCALADDRESS_INIT { \
    0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, \
    0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe \
}

//
// The old link local address for XP-SP2/Win2K3 machines.
//
#define IN6ADDR_TEREDOOLDLINKLOCALADDRESSXP_INIT {   \
    0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, \
    0x00, 0x00,  'T',   'E',  'R',  'E',  'D',  'O' \
}

//
// The old link local address for Vista Beta-2 and earlier machines.
//
#define IN6ADDR_TEREDOOLDLINKLOCALADDRESSVISTA_INIT {       \
    0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, \
    0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff \
}

#define IN6ADDR_LINKLOCALPREFIX_INIT { 0xfe, 0x80, }

#define IN6ADDR_MULTICASTPREFIX_INIT { 0xff, 0x00, }

#define IN6ADDR_SOLICITEDNODEMULTICASTPREFIX_INIT { \
    0xff, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, \
    0x00, 0x00, 0x00, 0x01, 0xff, \
}

#define IN6ADDR_V4MAPPEDPREFIX_INIT { \
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, \
    0x00, 0x00, 0xff, 0xff, \
}

#define IN6ADDR_6TO4PREFIX_INIT { 0x20, 0x02, }

#define IN6ADDR_TEREDOPREFIX_INIT { 0x20, 0x01, 0x00, 0x00, }

#define IN6ADDR_TEREDOPREFIX_INIT_OLD { 0x3f, 0xfe, 0x83, 0x1f, }

#define IN6ADDR_ULAPREFIX_INIT {0xfc }

#define IN6ADDR_SITELOCALPREFIX_INIT {0xfe, 0xc0 }

#define IN6ADDR_6BONETESTPREFIX_INIT { 0x3f, 0xfe }

#define IN6ADDR_LINKLOCALPREFIX_LENGTH 64

#define IN6ADDR_MULTICASTPREFIX_LENGTH 8

#define IN6ADDR_SOLICITEDNODEMULTICASTPREFIX_LENGTH 104

#define IN6ADDR_V4MAPPEDPREFIX_LENGTH 96

#define IN6ADDR_6TO4PREFIX_LENGTH 16

#define IN6ADDR_TEREDOPREFIX_LENGTH 32

#ifdef __cplusplus
extern "C" {
#endif

//
// N.B. These addresses are in network byte order.
//
extern CONST SCOPE_ID scopeid_unspecified;

extern CONST IN_ADDR in4addr_any;
extern CONST IN_ADDR in4addr_loopback;
extern CONST IN_ADDR in4addr_broadcast;
extern CONST IN_ADDR in4addr_allnodesonlink;
extern CONST IN_ADDR in4addr_allroutersonlink;
extern CONST IN_ADDR in4addr_alligmpv3routersonlink;
extern CONST IN_ADDR in4addr_allteredohostsonlink;
extern CONST IN_ADDR in4addr_linklocalprefix;
extern CONST IN_ADDR in4addr_multicastprefix;

extern CONST IN6_ADDR in6addr_any;
extern CONST IN6_ADDR in6addr_loopback;
extern CONST IN6_ADDR in6addr_allnodesonnode;
extern CONST IN6_ADDR in6addr_allnodesonlink;
extern CONST IN6_ADDR in6addr_allroutersonlink;
extern CONST IN6_ADDR in6addr_allmldv2routersonlink;
extern CONST IN6_ADDR in6addr_teredoinitiallinklocaladdress;
extern CONST IN6_ADDR in6addr_linklocalprefix;
extern CONST IN6_ADDR in6addr_multicastprefix;
extern CONST IN6_ADDR in6addr_solicitednodemulticastprefix;
extern CONST IN6_ADDR in6addr_v4mappedprefix;
extern CONST IN6_ADDR in6addr_6to4prefix;
extern CONST IN6_ADDR in6addr_teredoprefix;
extern CONST IN6_ADDR in6addr_teredoprefix_old;

#ifdef __cplusplus
}
#endif

#ifndef __midl

WS2TCPIP_INLINE
BOOLEAN
IN6_ADDR_EQUAL(CONST IN6_ADDR *x, CONST IN6_ADDR *y)
{
    __int64 UNALIGNED *a;
    __int64 UNALIGNED *b;

    a = (__int64 UNALIGNED *)x;
    b = (__int64 UNALIGNED *)y;

    return (BOOLEAN)((a[1] == b[1]) && (a[0] == b[0]));
}

//
// RFC 3542 uses IN6_ARE_ADDR_EQUAL().
//
#define IN6_ARE_ADDR_EQUAL IN6_ADDR_EQUAL

WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_UNSPECIFIED(CONST IN6_ADDR *a)
{
    //
    // We can't use the in6addr_any variable, since that would
    // require existing callers to link with a specific library.
    //
    return (BOOLEAN)((a->s6_words[0] == 0) &&
                     (a->s6_words[1] == 0) &&
                     (a->s6_words[2] == 0) &&
                     (a->s6_words[3] == 0) &&
                     (a->s6_words[4] == 0) &&
                     (a->s6_words[5] == 0) &&
                     (a->s6_words[6] == 0) &&
                     (a->s6_words[7] == 0));
}

WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_LOOPBACK(CONST IN6_ADDR *a)
{
    //
    // We can't use the in6addr_loopback variable, since that would
    // require existing callers to link with a specific library.
    //
    return (BOOLEAN)((a->s6_words[0] == 0) &&
                     (a->s6_words[1] == 0) &&
                     (a->s6_words[2] == 0) &&
                     (a->s6_words[3] == 0) &&
                     (a->s6_words[4] == 0) &&
                     (a->s6_words[5] == 0) &&
                     (a->s6_words[6] == 0) &&
                     (a->s6_words[7] == 0x0100));
}

WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_MULTICAST(CONST IN6_ADDR *a)
{
    return (BOOLEAN)(a->s6_bytes[0] == 0xff);
}

//
//  Does the address have a format prefix
//  that indicates it uses EUI-64 interface identifiers?
//
WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_EUI64(CONST IN6_ADDR *a)
{
    //
    // Format prefixes 001 through 111, except for multicast.
    //
    return (BOOLEAN)(((a->s6_bytes[0] & 0xe0) != 0) &&
                     !IN6_IS_ADDR_MULTICAST(a));
}

//
//  Is this the subnet router anycast address?
//  See RFC 2373.
//
WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_SUBNET_ROUTER_ANYCAST(CONST IN6_ADDR *a)
{
    return (BOOLEAN)(IN6_IS_ADDR_EUI64(a) &&
                     (a->s6_words[4] == 0) &&
                     (a->s6_words[5] == 0) &&
                     (a->s6_words[6] == 0) &&
                     (a->s6_words[7] == 0));
}

//
//  Is this a subnet reserved anycast address?
//  See RFC 2526. It talks about non-EUI-64
//  addresses as well, but IMHO that part
//  of the RFC doesn't make sense. For example,
//  it shouldn't apply to multicast or v4-compatible
//  addresses.
//
WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_SUBNET_RESERVED_ANYCAST(CONST IN6_ADDR *a)
{
    return (BOOLEAN)(IN6_IS_ADDR_EUI64(a) &&
                     (a->s6_words[4] == 0xfffd) &&
                     (a->s6_words[5] == 0xffff) &&
                     (a->s6_words[6] == 0xffff) &&
                     ((a->s6_words[7] & 0x80ff) == 0x80ff));
}

//
//  As best we can tell from simple inspection,
//  is this an anycast address?
//
WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_ANYCAST(CONST IN6_ADDR *a)
{
    return (IN6_IS_ADDR_SUBNET_RESERVED_ANYCAST(a) ||
            IN6_IS_ADDR_SUBNET_ROUTER_ANYCAST(a));
}

WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_LINKLOCAL(CONST IN6_ADDR *a)
{
    return (BOOLEAN)((a->s6_bytes[0] == 0xfe) &&
                     ((a->s6_bytes[1] & 0xc0) == 0x80));
}

WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_SITELOCAL(CONST IN6_ADDR *a)
{
    return (BOOLEAN)((a->s6_bytes[0] == 0xfe) &&
                     ((a->s6_bytes[1] & 0xc0) == 0xc0));
}

WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_GLOBAL(CONST IN6_ADDR *a)
{
    //
    // Check the format prefix and exclude addresses
    // whose high 4 bits are all zero or all one.
    // This is a cheap way of excluding v4-compatible,
    // v4-mapped, loopback, multicast, link-local, site-local.
    //
    ULONG High = (a->s6_bytes[0] & 0xf0u);
    return (BOOLEAN)((High != 0) && (High != 0xf0));
}

WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_V4MAPPED(CONST IN6_ADDR *a)
{
    return (BOOLEAN)((a->s6_words[0] == 0) &&
                     (a->s6_words[1] == 0) &&
                     (a->s6_words[2] == 0) &&
                     (a->s6_words[3] == 0) &&
                     (a->s6_words[4] == 0) &&
                     (a->s6_words[5] == 0xffff));
}

WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_V4COMPAT(CONST IN6_ADDR *a)
{
    return (BOOLEAN)((a->s6_words[0] == 0) &&
                     (a->s6_words[1] == 0) &&
                     (a->s6_words[2] == 0) &&
                     (a->s6_words[3] == 0) &&
                     (a->s6_words[4] == 0) &&
                     (a->s6_words[5] == 0) &&
                     !((a->s6_words[6] == 0) &&
                       (a->s6_addr[14] == 0) &&
                       ((a->s6_addr[15] == 0) || (a->s6_addr[15] == 1))));
}

WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_V4TRANSLATED(CONST IN6_ADDR *a)
{
    return (BOOLEAN)((a->s6_words[0] == 0) &&
                     (a->s6_words[1] == 0) &&
                     (a->s6_words[2] == 0) &&
                     (a->s6_words[3] == 0) &&
                     (a->s6_words[4] == 0xffff) &&
                     (a->s6_words[5] == 0));
}

WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_MC_NODELOCAL(CONST IN6_ADDR *a)
{
    return (BOOLEAN)(IN6_IS_ADDR_MULTICAST(a) &&
                     ((a->s6_bytes[1] & 0xf) == 1));
}

WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_MC_LINKLOCAL(CONST IN6_ADDR *a)
{
    return (BOOLEAN)(IN6_IS_ADDR_MULTICAST(a) &&
                     ((a->s6_bytes[1] & 0xf) == 2));
}

WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_MC_SITELOCAL(CONST IN6_ADDR *a)
{
    return (BOOLEAN)(IN6_IS_ADDR_MULTICAST(a) &&
                     ((a->s6_bytes[1] & 0xf) == 5));
}

WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_MC_ORGLOCAL(CONST IN6_ADDR *a)
{
    return (BOOLEAN)(IN6_IS_ADDR_MULTICAST(a) &&
                     ((a->s6_bytes[1] & 0xf) == 8));
}

WS2TCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_MC_GLOBAL(CONST IN6_ADDR *a)
{
    return (BOOLEAN)(IN6_IS_ADDR_MULTICAST(a) &&
                     ((a->s6_bytes[1] & 0xf) == 0xe));
}

WS2TCPIP_INLINE
VOID
IN6_SET_ADDR_UNSPECIFIED(PIN6_ADDR a)
{
    //
    // We can't use the in6addr_any variable, since that would
    // require existing callers to link with a specific library.
    //
    memset(a->s6_bytes, 0, sizeof(IN6_ADDR));
}

WS2TCPIP_INLINE
VOID
IN6_SET_ADDR_LOOPBACK(PIN6_ADDR a)
{
    //
    // We can't use the in6addr_loopback variable, since that would
    // require existing callers to link with a specific library.
    //
    memset(a->s6_bytes, 0, sizeof(IN6_ADDR));
    a->s6_bytes[15] = 1;
}

WS2TCPIP_INLINE
VOID
IN6ADDR_SETANY(PSOCKADDR_IN6 a)
{
    a->sin6_family = AF_INET6;
    a->sin6_port = 0;
    a->sin6_flowinfo = 0;
    IN6_SET_ADDR_UNSPECIFIED(&a->sin6_addr);
    a->sin6_scope_id = 0;
}

WS2TCPIP_INLINE
VOID
IN6ADDR_SETLOOPBACK(PSOCKADDR_IN6 a)
{
    a->sin6_family = AF_INET6;
    a->sin6_port = 0;
    a->sin6_flowinfo = 0;
    IN6_SET_ADDR_LOOPBACK(&a->sin6_addr);
    a->sin6_scope_id = 0;
}

WS2TCPIP_INLINE
BOOLEAN
IN6ADDR_ISANY(CONST SOCKADDR_IN6 *a)
{
    WS2IPDEF_ASSERT(a->sin6_family == AF_INET6);
    return IN6_IS_ADDR_UNSPECIFIED(&a->sin6_addr);
}

WS2TCPIP_INLINE
BOOLEAN
IN6ADDR_ISLOOPBACK(CONST SOCKADDR_IN6 *a)
{
    WS2IPDEF_ASSERT(a->sin6_family == AF_INET6);
    return IN6_IS_ADDR_LOOPBACK(&a->sin6_addr);
}

WS2TCPIP_INLINE
BOOLEAN
IN6ADDR_ISEQUAL(CONST SOCKADDR_IN6 *a, CONST SOCKADDR_IN6 *b)
{
    WS2IPDEF_ASSERT(a->sin6_family == AF_INET6);
    return (BOOLEAN)(a->sin6_scope_id == b->sin6_scope_id &&
                     IN6_ADDR_EQUAL(&a->sin6_addr, &b->sin6_addr));
}

WS2TCPIP_INLINE
BOOLEAN
IN6ADDR_ISUNSPECIFIED(CONST SOCKADDR_IN6 *a)
{
    WS2IPDEF_ASSERT(a->sin6_family == AF_INET6);
    return (BOOLEAN)(a->sin6_scope_id == 0 &&
                     IN6_IS_ADDR_UNSPECIFIED(&a->sin6_addr));
}

#endif // __midl

#endif // (NTDDI_VERSION >= NTDDI_WIN2KSP1)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

//
// TCP/IP specific Ioctl codes.
//
#define SIO_GET_INTERFACE_LIST     _IOR('t', 127, ULONG)
#define SIO_GET_INTERFACE_LIST_EX  _IOR('t', 126, ULONG)
#define SIO_SET_MULTICAST_FILTER   _IOW('t', 125, ULONG)
#define SIO_GET_MULTICAST_FILTER   _IOW('t', 124 | IOC_IN, ULONG)
#define SIOCSIPMSFILTER            SIO_SET_MULTICAST_FILTER
#define SIOCGIPMSFILTER            SIO_GET_MULTICAST_FILTER

//
// Protocol independent ioctls for setting and retrieving multicast filters.
//
#define SIOCSMSFILTER     _IOW('t', 126, ULONG)
#define SIOCGMSFILTER     _IOW('t', 127 | IOC_IN, ULONG)

#if (NTDDI_VERSION >= NTDDI_VISTASP1)

#define IDEAL_SEND_BACKLOG_IOCTLS

//
// Query and change notification ioctls for the ideal send backlog size
// for a given connection. Clients should use the wrappers defined in
// ws2tcpip.h rather than using these ioctls directly.
//

#define SIO_IDEAL_SEND_BACKLOG_QUERY   _IOR('t', 123, ULONG)
#define SIO_IDEAL_SEND_BACKLOG_CHANGE   _IO('t', 122)

#endif

//
// Protocol independent multicast source filter options.
//
#define MCAST_JOIN_GROUP            41	// Join all sources for a group.
#define MCAST_LEAVE_GROUP           42  // Drop all sources for a group.
#define MCAST_BLOCK_SOURCE          43	// Block IP group/source.
#define MCAST_UNBLOCK_SOURCE        44	// Unblock IP group/source.
#define MCAST_JOIN_SOURCE_GROUP     45	// Join IP group/source.
#define MCAST_LEAVE_SOURCE_GROUP    46	// Leave IP group/source.

//
// Definitions of MCAST_INCLUDE and MCAST_EXCLUDE for multicast source filter.
//
typedef enum {
    MCAST_INCLUDE = 0,
    MCAST_EXCLUDE
} MULTICAST_MODE_TYPE;

//
// Structure for IP_MREQ (used by IP_ADD_MEMBERSHIP and IP_DROP_MEMBERSHIP).
//
typedef struct ip_mreq {
    IN_ADDR imr_multiaddr;  // IP multicast address of group.
    IN_ADDR imr_interface;  // Local IP address of interface.
} IP_MREQ, *PIP_MREQ;

//
// Structure for IP_MREQ_SOURCE (used by IP_BLOCK_SOURCE, IP_UNBLOCK_SOURCE
// etc.).
//
typedef struct ip_mreq_source {
    IN_ADDR imr_multiaddr;  // IP multicast address of group.
    IN_ADDR imr_sourceaddr; // IP address of source.
    IN_ADDR imr_interface;  // Local IP address of interface.
} IP_MREQ_SOURCE, *PIP_MREQ_SOURCE;

//
// Structure for IP_MSFILTER (used by SIOCSIPMSFILTER and SIOCGIPMSFILTER).
//
typedef struct ip_msfilter {
    IN_ADDR imsf_multiaddr;  // IP multicast address of group.
    IN_ADDR imsf_interface;  // Local IP address of interface.
    MULTICAST_MODE_TYPE imsf_fmode;        // Filter mode.
    ULONG imsf_numsrc;       // Number of sources in src_list.
    IN_ADDR imsf_slist[1];   // Start of source list.
} IP_MSFILTER, *PIP_MSFILTER;

#define IP_MSFILTER_SIZE(NumSources) \
    (sizeof(IP_MSFILTER) - sizeof(IN_ADDR) + (NumSources) * sizeof(IN_ADDR))

//
// Options to use with [gs]etsockopt at the IPPROTO_IPV6 level.
// These are specified in RFCs 3493 and 3542.
// The values should be consistent with the IPv6 equivalents.
//
#define IPV6_HOPOPTS           1 // Set/get IPv6 hop-by-hop options.
#define IPV6_HDRINCL           2 // Header is included with data.
#define IPV6_UNICAST_HOPS      4 // IP unicast hop limit.
#define IPV6_MULTICAST_IF      9 // IP multicast interface.
#define IPV6_MULTICAST_HOPS   10 // IP multicast hop limit.
#define IPV6_MULTICAST_LOOP   11 // IP multicast loopback.
#define IPV6_ADD_MEMBERSHIP   12 // Add an IP group membership.
#define IPV6_JOIN_GROUP       IPV6_ADD_MEMBERSHIP
#define IPV6_DROP_MEMBERSHIP  13 // Drop an IP group membership.
#define IPV6_LEAVE_GROUP      IPV6_DROP_MEMBERSHIP
#define IPV6_DONTFRAG         14 // Don't fragment IP datagrams.
#define IPV6_PKTINFO          19 // Receive packet information.
#define IPV6_HOPLIMIT         21 // Receive packet hop limit.
#define IPV6_PROTECTION_LEVEL 23 // Set/get IPv6 protection level.
#define IPV6_RECVIF           24 // Receive arrival interface.
#define IPV6_RECVDSTADDR      25 // Receive destination address.
#define IPV6_CHECKSUM         26 // Offset to checksum for raw IP socket send.
#define IPV6_V6ONLY           27 // Treat wildcard bind as AF_INET6-only.
#define IPV6_IFLIST           28 // Enable/Disable an interface list.
#define IPV6_ADD_IFLIST       29 // Add an interface list entry.
#define IPV6_DEL_IFLIST       30 // Delete an interface list entry.
#define IPV6_UNICAST_IF       31 // IP unicast interface.
#define IPV6_RTHDR            32 // Set/get IPv6 routing header.
#define IPV6_GET_IFLIST       33 // Get an interface list.
#define IPV6_RECVRTHDR        38 // Receive the routing header.
#define IPV6_TCLASS           39 // Packet traffic class.
#define IPV6_RECVTCLASS       40 // Receive packet traffic class.
#define IPV6_ECN              50 // IPv6 ECN codepoint.
#define IPV6_RECVECN          50 // Receive ECN codepoints in the IPv6 header.
#define IPV6_PKTINFO_EX       51 // Receive extended packet information.
#define IPV6_WFP_REDIRECT_RECORDS   60 // WFP's Connection Redirect Records
#define IPV6_WFP_REDIRECT_CONTEXT   70 // WFP's Connection Redirect Context
#define IPV6_MTU_DISCOVER           71 // Set/get path MTU discover state.
#define IPV6_MTU                    72 // Get path MTU.
#define IPV6_NRT_INTERFACE          74 // Set NRT interface constraint (outbound).
#define IPV6_RECVERR                75 // Receive ICMPv6 errors.
#define IPV6_USER_MTU               76 // Set/get app defined upper bound IP layer MTU.

#define IP_UNSPECIFIED_HOP_LIMIT -1

#define IP_PROTECTION_LEVEL   IPV6_PROTECTION_LEVEL
//
// Values of IPV6_PROTECTION_LEVEL.
//
#define PROTECTION_LEVEL_UNRESTRICTED   10 // For peer-to-peer apps.
#define PROTECTION_LEVEL_EDGERESTRICTED 20 // Same as unrestricted. Except for
                                           // Teredo.
#define PROTECTION_LEVEL_RESTRICTED     30 // For Intranet apps.

#if (NTDDI_VERSION < NTDDI_VISTA)
#define PROTECTION_LEVEL_DEFAULT        PROTECTION_LEVEL_EDGERESTRICTED
#else
#define PROTECTION_LEVEL_DEFAULT        ((UINT)-1)
#endif
//
// Structure for IPV6_JOIN_GROUP and IPV6_LEAVE_GROUP (also,
// IPV6_ADD_MEMBERSHIP and IPV6_DROP_MEMBERSHIP).
//
typedef struct ipv6_mreq {
    IN6_ADDR ipv6mr_multiaddr;  // IPv6 multicast address.
    ULONG ipv6mr_interface;     // Interface index.
} IPV6_MREQ, *PIPV6_MREQ;

#if (NTDDI_VERSION >= NTDDI_WINXP)
//
// Structure for GROUP_REQ used by protocol independent source filters
// (MCAST_JOIN_GROUP and MCAST_LEAVE_GROUP).
//
typedef struct group_req {
    ULONG gr_interface;         // Interface index.
    SOCKADDR_STORAGE gr_group;  // Multicast address.
} GROUP_REQ, *PGROUP_REQ;

//
// Structure for GROUP_SOURCE_REQ used by protocol independent source filters
// (MCAST_JOIN_SOURCE_GROUP, MCAST_LEAVE_SOURCE_GROUP etc.).
//
typedef struct group_source_req {
    ULONG gsr_interface;        // Interface index.
    SOCKADDR_STORAGE gsr_group; // Group address.
    SOCKADDR_STORAGE gsr_source; // Source address.
} GROUP_SOURCE_REQ, *PGROUP_SOURCE_REQ;

//
// Structure for GROUP_FILTER used by protocol independent source filters
// (SIOCSMSFILTER and SIOCGMSFILTER).
//
typedef struct group_filter {
    ULONG gf_interface;         // Interface index.
    SOCKADDR_STORAGE gf_group;  // Multicast address.
    MULTICAST_MODE_TYPE gf_fmode; // Filter mode.
    ULONG gf_numsrc;            // Number of sources.
    SOCKADDR_STORAGE gf_slist[1]; // Source address.
} GROUP_FILTER, *PGROUP_FILTER;

#define GROUP_FILTER_SIZE(numsrc) \
   (sizeof(GROUP_FILTER) - sizeof(SOCKADDR_STORAGE) \
   + (numsrc) * sizeof(SOCKADDR_STORAGE))
#endif

//
// Structure for IP_PKTINFO option.
//
typedef struct in_pktinfo {
    IN_ADDR ipi_addr;     // Source/destination IPv4 address.
    ULONG ipi_ifindex;    // Send/receive interface index.
} IN_PKTINFO, *PIN_PKTINFO;

#ifndef MIDL_PASS
C_ASSERT(sizeof(IN_PKTINFO) == 8);
#endif

//
// Structure for IPV6_PKTINFO option.
//
typedef struct in6_pktinfo {
    IN6_ADDR ipi6_addr;    // Source/destination IPv6 address.
    ULONG ipi6_ifindex;    // Send/receive interface index.
} IN6_PKTINFO, *PIN6_PKTINFO;

#ifndef MIDL_PASS
C_ASSERT(sizeof(IN6_PKTINFO) == 20);
#endif

//
// Structure for IP_PKTINFO_EX option.
//
typedef struct in_pktinfo_ex {
    IN_PKTINFO pkt_info;
    SCOPE_ID scope_id;
} IN_PKTINFO_EX, *PIN_PKTINFO_EX;

#ifndef MIDL_PASS
C_ASSERT(sizeof(IN_PKTINFO_EX) == 12);
#endif

//
// Structure for IPV6_PKTINFO_EX option.
//
typedef struct in6_pktinfo_ex {
    IN6_PKTINFO pkt_info;
    SCOPE_ID scope_id;
} IN6_PKTINFO_EX, *PIN6_PKTINFO_EX;

#ifndef MIDL_PASS
C_ASSERT(sizeof(IN6_PKTINFO_EX) == 24);
#endif

//
// Structure for IP_RECVERR option.
//
typedef struct in_recverr {
    IPPROTO protocol;   // IPPROTO_ICMP or IPPROTO_ICMPV6.
    ULONG info;         // MTU if frag needed or pkt too big message.
    UINT8 type;
    UINT8 code;
} IN_RECVERR, *PIN_RECVERR;

//
// Maximum length of address literals (potentially including a port number)
// generated by any address-to-string conversion routine.  This length can
// be used when declaring buffers used with getnameinfo, WSAAddressToString,
// inet_ntoa, etc.  We just provide one define, rather than one per api,
// to avoid confusion.
//
// The totals are derived from the following data:
//  15: IPv4 address
//  45: IPv6 address including embedded IPv4 address
//  11: Scope Id
//   2: Brackets around IPv6 address when port is present
//   6: Port (including colon)
//   1: Terminating null byte
//
#define INET_ADDRSTRLEN  22
#define INET6_ADDRSTRLEN 65



//
// Options to use with [gs]etsockopt at the IPPROTO_TCP level.
// TCP_NODELAY is defined in ws2def.h for historical reasons.
//

//
// Offload preferences supported.
//
#define TCP_OFFLOAD_NO_PREFERENCE	0
#define	TCP_OFFLOAD_NOT_PREFERRED	1
#define TCP_OFFLOAD_PREFERRED		2

//      TCP_NODELAY         	 0x0001
#define TCP_EXPEDITED_1122  	 0x0002
#define TCP_KEEPALIVE       	 3
#define TCP_MAXSEG          	 4
#define TCP_MAXRT           	 5
#define TCP_STDURG          	 6
#define TCP_NOURG           	 7
#define TCP_ATMARK          	 8
#define TCP_NOSYNRETRIES    	 9
#define TCP_TIMESTAMPS      	 10
#define TCP_OFFLOAD_PREFERENCE	 11
#define TCP_CONGESTION_ALGORITHM 12
#define TCP_DELAY_FIN_ACK        13
#define TCP_MAXRTMS              14
#define TCP_FASTOPEN             15
#define TCP_KEEPCNT              16
#define TCP_KEEPIDLE             TCP_KEEPALIVE
#define TCP_KEEPINTVL            17
#define TCP_FAIL_CONNECT_ON_ICMP_ERROR 18
#define TCP_ICMP_ERROR_INFO      19

//
// Structure for TCP_ICMP_ERROR_INFO option.
//
typedef struct icmp_error_info {
    SOCKADDR_INET srcaddress;
    IPPROTO protocol;
    UINT8 type;
    UINT8 code;
} ICMP_ERROR_INFO, *PICMP_ERROR_INFO;


//
// Options to use with [gs]etsockopt at the IPPROTO_UDP level.
// UDP_NOCHECKSUM is defined in ws2tcpip.h for historical reasons.
// UDP_CHECKSUM_COVERAGE is defined in ws2tcpip.h for historical reasons.
//

//      UDP_NOCHECKSUM              1
#define UDP_SEND_MSG_SIZE           2
#define UDP_RECV_MAX_COALESCED_SIZE 3
//      UDP_CHECKSUM_COVERAGE       20

//
// Control message types at the IPPROTO_UDP level.
//

#define UDP_COALESCED_INFO          3


#ifdef _PREFAST_
#pragma prefast(pop)
#endif

#pragma warning(pop)

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#ifdef __cplusplus
}
#endif /* __cplusplus */
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif
