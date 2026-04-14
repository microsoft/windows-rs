/*++

Copyright (c) Microsoft Corporation

Module Name:

    ipmib.h

Abstract:

    This module contains the public definitions and structures for the
    IP-specific parts of MIB-II.  These definitions were previously
    in iprtrmib.h, which now includes this file.

--*/

#ifndef _IPMIB_
#define _IPMIB_

#if defined(_MSC_VER) && (_MSC_VER > 1000)
#pragma once
#endif
#include <winapifamily.h>

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


#ifndef ANY_SIZE
#define ANY_SIZE 1
#endif

//
// Pick up definitions of MAXLEN_PHYSADDR, etc.
//
#include <ifmib.h>
#include <nldef.h>

#if defined(_MSC_VER)
#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201) /* nonstandard extension used : nameless struct/union */
#endif // defined(_MSC_VER)

#define MIB_IPADDR_PRIMARY      0x0001 // Primary ipaddr
#define MIB_IPADDR_DYNAMIC      0x0004 // Dynamic ipaddr
#define MIB_IPADDR_DISCONNECTED 0x0008 // Address is on disconnected interface
#define MIB_IPADDR_DELETED      0x0040 // Address being deleted
#define MIB_IPADDR_TRANSIENT    0x0080 // Transient address
#define MIB_IPADDR_DNS_ELIGIBLE 0X0100 // Address is published in DNS.

typedef struct _MIB_IPADDRROW_XP {
    DWORD dwAddr;
    IF_INDEX dwIndex;
    DWORD dwMask;
    DWORD dwBCastAddr;
    DWORD dwReasmSize;
    unsigned short unused1;
    unsigned short wType;
} MIB_IPADDRROW_XP, *PMIB_IPADDRROW_XP;

typedef struct _MIB_IPADDRROW_W2K {
    DWORD dwAddr;
    DWORD dwIndex;
    DWORD dwMask;
    DWORD dwBCastAddr;
    DWORD dwReasmSize;
    unsigned short unused1;
    unsigned short unused2;
} MIB_IPADDRROW_W2K, *PMIB_IPADDRROW_W2K;

#if (NTDDI_VERSION >= NTDDI_WINXP)
typedef MIB_IPADDRROW_XP MIB_IPADDRROW;
typedef MIB_IPADDRROW_XP *PMIB_IPADDRROW;
#elif (NTDDI_VERSION >= NTDDI_WIN2K)
typedef MIB_IPADDRROW_W2K MIB_IPADDRROW;
typedef MIB_IPADDRROW_W2K *PMIB_IPADDRROW;
#else
typedef MIB_IPADDRROW_XP MIB_IPADDRROW;
typedef MIB_IPADDRROW_XP *PMIB_IPADDRROW;
#endif

typedef struct _MIB_IPADDRTABLE {
    DWORD dwNumEntries;
    MIB_IPADDRROW table[ANY_SIZE];
} MIB_IPADDRTABLE, *PMIB_IPADDRTABLE;


#define SIZEOF_IPADDRTABLE(X) (FIELD_OFFSET(MIB_IPADDRTABLE,table[0]) + \
                               ((X) * sizeof(MIB_IPADDRROW)) + ALIGN_SIZE)

typedef struct _MIB_IPFORWARDNUMBER {
    DWORD dwValue;
} MIB_IPFORWARDNUMBER,*PMIB_IPFORWARDNUMBER;

typedef NL_ROUTE_PROTOCOL MIB_IPFORWARD_PROTO;

typedef enum {
    MIB_IPROUTE_TYPE_OTHER    = 1,
    MIB_IPROUTE_TYPE_INVALID  = 2,
    MIB_IPROUTE_TYPE_DIRECT   = 3,
    MIB_IPROUTE_TYPE_INDIRECT = 4,
} MIB_IPFORWARD_TYPE;

typedef struct _MIB_IPFORWARDROW {
    DWORD dwForwardDest;
    DWORD dwForwardMask;
    DWORD dwForwardPolicy;
    DWORD dwForwardNextHop;
    IF_INDEX dwForwardIfIndex;
    union {
        DWORD dwForwardType;              // Old field name uses DWORD type.
        MIB_IPFORWARD_TYPE ForwardType;   // New field name uses enum type.
    };
    union {
        DWORD dwForwardProto;             // Old field name uses DWORD type.
        MIB_IPFORWARD_PROTO ForwardProto; // New field name uses enum type.
    };
    DWORD dwForwardAge;
    DWORD dwForwardNextHopAS;
    DWORD dwForwardMetric1;
    DWORD dwForwardMetric2;
    DWORD dwForwardMetric3;
    DWORD dwForwardMetric4;
    DWORD dwForwardMetric5;
} MIB_IPFORWARDROW, *PMIB_IPFORWARDROW;

#define MIB_IPROUTE_TYPE_OTHER    1
#define MIB_IPROUTE_TYPE_INVALID  2
#define MIB_IPROUTE_TYPE_DIRECT   3
#define MIB_IPROUTE_TYPE_INDIRECT 4

#define MIB_IPROUTE_METRIC_UNUSED    (DWORD)-1


typedef struct _MIB_IPFORWARDTABLE {
    DWORD dwNumEntries;
    MIB_IPFORWARDROW table[ANY_SIZE];
} MIB_IPFORWARDTABLE, *PMIB_IPFORWARDTABLE;

#define SIZEOF_IPFORWARDTABLE(X) \
            (FIELD_OFFSET(MIB_IPFORWARDTABLE,table[0]) + \
             ((X) * sizeof(MIB_IPFORWARDROW)) + ALIGN_SIZE)


typedef enum {
    MIB_IPNET_TYPE_OTHER   = 1,
    MIB_IPNET_TYPE_INVALID = 2,
    MIB_IPNET_TYPE_DYNAMIC = 3,
    MIB_IPNET_TYPE_STATIC  = 4,
} MIB_IPNET_TYPE;

typedef struct _MIB_IPNETROW_LH {
    IF_INDEX dwIndex;
    DWORD dwPhysAddrLen;
    UCHAR bPhysAddr[MAXLEN_PHYSADDR];
    DWORD dwAddr;
    union {
        DWORD dwType;           // Old field name was just a DWORD.
        MIB_IPNET_TYPE Type;    // New field name uses the enum type.
    };
} MIB_IPNETROW_LH, *PMIB_IPNETROW_LH;

typedef struct _MIB_IPNETROW_W2K {
    IF_INDEX dwIndex;
    DWORD dwPhysAddrLen;
    UCHAR bPhysAddr[MAXLEN_PHYSADDR];
    DWORD dwAddr;
    DWORD dwType;
} MIB_IPNETROW_W2K, *PMIB_IPNETROW_W2K;


#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef MIB_IPNETROW_LH MIB_IPNETROW;
typedef MIB_IPNETROW_LH *PMIB_IPNETROW;
#elif (NTDDI_VERSION >= NTDDI_WIN2K)
typedef MIB_IPNETROW_W2K MIB_IPNETROW;
typedef MIB_IPNETROW_W2K *PMIB_IPNETROW;
#else
typedef MIB_IPNETROW_LH MIB_IPNETROW;
typedef MIB_IPNETROW_LH *PMIB_IPNETROW;
#endif

typedef struct _MIB_IPNETTABLE {
    DWORD dwNumEntries;
    MIB_IPNETROW table[ANY_SIZE];
} MIB_IPNETTABLE, *PMIB_IPNETTABLE;

#define SIZEOF_IPNETTABLE(X) (FIELD_OFFSET(MIB_IPNETTABLE, table[0]) + \
                              ((X) * sizeof(MIB_IPNETROW)) + ALIGN_SIZE)

typedef enum {
    MIB_IP_FORWARDING     = 1,
    MIB_IP_NOT_FORWARDING = 2,
} MIB_IPSTATS_FORWARDING, *PMIB_IPSTATS_FORWARDING;

#define MIB_USE_CURRENT_TTL         ((DWORD)-1)
#define MIB_USE_CURRENT_FORWARDING  ((DWORD)-1)

typedef struct _MIB_IPSTATS_LH {
    union {
        DWORD dwForwarding;
        MIB_IPSTATS_FORWARDING Forwarding;
    };
    DWORD dwDefaultTTL;
    DWORD dwInReceives;
    DWORD dwInHdrErrors;
    DWORD dwInAddrErrors;
    DWORD dwForwDatagrams;
    DWORD dwInUnknownProtos;
    DWORD dwInDiscards;
    DWORD dwInDelivers;
    DWORD dwOutRequests;
    DWORD dwRoutingDiscards;
    DWORD dwOutDiscards;
    DWORD dwOutNoRoutes;
    DWORD dwReasmTimeout;
    DWORD dwReasmReqds;
    DWORD dwReasmOks;
    DWORD dwReasmFails;
    DWORD dwFragOks;
    DWORD dwFragFails;
    DWORD dwFragCreates;
    DWORD dwNumIf;
    DWORD dwNumAddr;
    DWORD dwNumRoutes;
} MIB_IPSTATS_LH, *PMIB_IPSTATS_LH;

typedef struct _MIB_IPSTATS_W2K {
    DWORD dwForwarding;
    DWORD dwDefaultTTL;
    DWORD dwInReceives;
    DWORD dwInHdrErrors;
    DWORD dwInAddrErrors;
    DWORD dwForwDatagrams;
    DWORD dwInUnknownProtos;
    DWORD dwInDiscards;
    DWORD dwInDelivers;
    DWORD dwOutRequests;
    DWORD dwRoutingDiscards;
    DWORD dwOutDiscards;
    DWORD dwOutNoRoutes;
    DWORD dwReasmTimeout;
    DWORD dwReasmReqds;
    DWORD dwReasmOks;
    DWORD dwReasmFails;
    DWORD dwFragOks;
    DWORD dwFragFails;
    DWORD dwFragCreates;
    DWORD dwNumIf;
    DWORD dwNumAddr;
    DWORD dwNumRoutes;
} MIB_IPSTATS_W2K, *PMIB_IPSTATS_W2K;

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef MIB_IPSTATS_LH MIB_IPSTATS;
typedef MIB_IPSTATS_LH *PMIB_IPSTATS;
#elif (NTDDI_VERSION >= NTDDI_WIN2K)
typedef MIB_IPSTATS_W2K MIB_IPSTATS;
typedef MIB_IPSTATS_W2K *PMIB_IPSTATS;
#endif

typedef struct _MIBICMPSTATS {
    DWORD dwMsgs;
    DWORD dwErrors;
    DWORD dwDestUnreachs;
    DWORD dwTimeExcds;
    DWORD dwParmProbs;
    DWORD dwSrcQuenchs;
    DWORD dwRedirects;
    DWORD dwEchos;
    DWORD dwEchoReps;
    DWORD dwTimestamps;
    DWORD dwTimestampReps;
    DWORD dwAddrMasks;
    DWORD dwAddrMaskReps;
} MIBICMPSTATS, *PMIBICMPSTATS;

typedef struct _MIBICMPINFO {
    MIBICMPSTATS icmpInStats;
    MIBICMPSTATS icmpOutStats;
} MIBICMPINFO;

typedef struct _MIB_ICMP {
    MIBICMPINFO stats;
} MIB_ICMP,*PMIB_ICMP;

typedef struct _MIBICMPSTATS_EX_XPSP1 {
    DWORD dwMsgs;
    DWORD dwErrors;
    DWORD rgdwTypeCount[256];
} MIBICMPSTATS_EX_XPSP1, *PMIBICMPSTATS_EX_XPSP1;

typedef  MIBICMPSTATS_EX_XPSP1 MIBICMPSTATS_EX;
typedef  MIBICMPSTATS_EX_XPSP1 *PMIBICMPSTATS_EX;

typedef struct _MIB_ICMP_EX_XPSP1 {
    MIBICMPSTATS_EX icmpInStats;
    MIBICMPSTATS_EX icmpOutStats;
} MIB_ICMP_EX_XPSP1,*PMIB_ICMP_EX_XPSP1;

typedef  MIB_ICMP_EX_XPSP1 MIB_ICMP_EX;
typedef  MIB_ICMP_EX_XPSP1 *PMIB_ICMP_EX;


//
// ICMP6_TYPE
//
// ICMPv6 Type Values from RFC 2292, 2461 (ND), and 3810 (MLDv2)
//
typedef enum {
    ICMP6_DST_UNREACH          =   1,
    ICMP6_PACKET_TOO_BIG       =   2,
    ICMP6_TIME_EXCEEDED        =   3,
    ICMP6_PARAM_PROB           =   4,
    ICMP6_ECHO_REQUEST         = 128,
    ICMP6_ECHO_REPLY           = 129,
    ICMP6_MEMBERSHIP_QUERY     = 130,
    ICMP6_MEMBERSHIP_REPORT    = 131,
    ICMP6_MEMBERSHIP_REDUCTION = 132,
    ND_ROUTER_SOLICIT          = 133,
    ND_ROUTER_ADVERT           = 134,
    ND_NEIGHBOR_SOLICIT        = 135,
    ND_NEIGHBOR_ADVERT         = 136,
    ND_REDIRECT                = 137,
    ICMP6_V2_MEMBERSHIP_REPORT = 143,
} ICMP6_TYPE, *PICMP6_TYPE;


//
// Used to identify informational/error messages.
//
#define ICMP6_INFOMSG_MASK 0x80
#define ICMP6_ISTYPEINFORMATIONAL(Type) (((Type) & ICMP6_INFOMSG_MASK) != 0)
#define ICMP6_ISTYPEERROR(Type) (!ICMP6_ISTYPEINFORMATIONAL(Type))

//
// ICMP4_TYPE
//
// There are no RFC-specified defines for ICMPv4 message types, so we try to
// use the ICMP6 values from RFC 2292 modified to be prefixed with ICMP4.
//
typedef enum {
    ICMP4_ECHO_REPLY        =  0, // Echo Reply.
    ICMP4_DST_UNREACH       =  3, // Destination Unreachable.
    ICMP4_SOURCE_QUENCH     =  4, // Source Quench.
    ICMP4_REDIRECT          =  5, // Redirect.
    ICMP4_ECHO_REQUEST      =  8, // Echo Request.
    ICMP4_ROUTER_ADVERT     =  9, // Router Advertisement.
    ICMP4_ROUTER_SOLICIT    = 10, // Router Solicitation.
    ICMP4_TIME_EXCEEDED     = 11, // Time Exceeded.
    ICMP4_PARAM_PROB        = 12, // Parameter Problem.
    ICMP4_TIMESTAMP_REQUEST = 13, // Timestamp Request.
    ICMP4_TIMESTAMP_REPLY   = 14, // Timestamp Reply.
    ICMP4_MASK_REQUEST      = 17, // Address Mask Request.
    ICMP4_MASK_REPLY        = 18, // Address Mask Reply.
} ICMP4_TYPE, *PICMP4_TYPE;

//
// See RFC 1812, section 4.3.1.
//
#define ICMP4_ISTYPEERROR(Type) \
    (((Type) == ICMP4_DST_UNREACH) || \
     ((Type) == ICMP4_SOURCE_QUENCH) || \
     ((Type) == ICMP4_REDIRECT) || \
     ((Type) == ICMP4_PARAM_PROB) || \
     ((Type) == ICMP4_TIME_EXCEEDED)) \

typedef struct _MIB_IPMCAST_OIF_XP {
    DWORD   dwOutIfIndex;
    DWORD   dwNextHopAddr;
    DWORD   dwReserved;
    DWORD   dwReserved1;
} MIB_IPMCAST_OIF_XP, *PMIB_IPMCAST_OIF_XP ;

typedef struct _MIB_IPMCAST_OIF_W2K {
    DWORD   dwOutIfIndex;
    DWORD   dwNextHopAddr;
    PVOID   pvReserved;
    DWORD   dwReserved;
} MIB_IPMCAST_OIF_W2K, *PMIB_IPMCAST_OIF_W2K;

#if (NTDDI_VERSION >= NTDDI_WINXP)
typedef MIB_IPMCAST_OIF_XP MIB_IPMCAST_OIF;
typedef MIB_IPMCAST_OIF_XP *PMIB_IPMCAST_OIF;
#elif (NTDDI_VERSION >= NTDDI_WIN2K)
typedef MIB_IPMCAST_OIF_W2K MIB_IPMCAST_OIF;
typedef MIB_IPMCAST_OIF_W2K *PMIB_IPMCAST_OIF;
#else
typedef MIB_IPMCAST_OIF_XP MIB_IPMCAST_OIF;
typedef MIB_IPMCAST_OIF_XP *PMIB_IPMCAST_OIF;
#endif

typedef struct _MIB_IPMCAST_MFE {
    DWORD   dwGroup;
    DWORD   dwSource;
    DWORD   dwSrcMask;
    DWORD   dwUpStrmNgbr;
    DWORD   dwInIfIndex;
    DWORD   dwInIfProtocol;
    DWORD   dwRouteProtocol;
    DWORD   dwRouteNetwork;
    DWORD   dwRouteMask;
    ULONG   ulUpTime;
    ULONG   ulExpiryTime;
    ULONG   ulTimeOut;
    ULONG   ulNumOutIf;
    DWORD   fFlags;
    DWORD   dwReserved;
    MIB_IPMCAST_OIF rgmioOutInfo[ANY_SIZE];
} MIB_IPMCAST_MFE, *PMIB_IPMCAST_MFE;

typedef struct _MIB_MFE_TABLE {
    DWORD           dwNumEntries;
    MIB_IPMCAST_MFE table[ANY_SIZE];
} MIB_MFE_TABLE, *PMIB_MFE_TABLE;


#define SIZEOF_BASIC_MIB_MFE          \
    (ULONG)(FIELD_OFFSET(MIB_IPMCAST_MFE, rgmioOutInfo[0]))

#define SIZEOF_MIB_MFE(X)             \
    (SIZEOF_BASIC_MIB_MFE + ((X) * sizeof(MIB_IPMCAST_OIF)))


typedef struct _MIB_IPMCAST_OIF_STATS_LH {
    DWORD   dwOutIfIndex;
    DWORD   dwNextHopAddr;
    DWORD   dwDialContext;
    ULONG   ulTtlTooLow;
    ULONG   ulFragNeeded;
    ULONG   ulOutPackets;
    ULONG   ulOutDiscards;
} MIB_IPMCAST_OIF_STATS_LH, *PMIB_IPMCAST_OIF_STATS_LH;

typedef struct _MIB_IPMCAST_OIF_STATS_W2K {
    DWORD   dwOutIfIndex;
    DWORD   dwNextHopAddr;
    PVOID   pvDialContext;
    ULONG   ulTtlTooLow;
    ULONG   ulFragNeeded;
    ULONG   ulOutPackets;
    ULONG   ulOutDiscards;
} MIB_IPMCAST_OIF_STATS_W2K, *PMIB_IPMCAST_OIF_STATS_W2K;

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef  MIB_IPMCAST_OIF_STATS_LH MIB_IPMCAST_OIF_STATS;
typedef  MIB_IPMCAST_OIF_STATS_LH *PMIB_IPMCAST_OIF_STATS;
#elif (NTDDI_VERSION >= NTDDI_WIN2K)
typedef  MIB_IPMCAST_OIF_STATS_W2K MIB_IPMCAST_OIF_STATS;
typedef  MIB_IPMCAST_OIF_STATS_W2K *PMIB_IPMCAST_OIF_STATS;
#else
typedef  MIB_IPMCAST_OIF_STATS_LH MIB_IPMCAST_OIF_STATS;
typedef  MIB_IPMCAST_OIF_STATS_LH *PMIB_IPMCAST_OIF_STATS;
#endif

typedef struct _MIB_IPMCAST_MFE_STATS {
    DWORD   dwGroup;
    DWORD   dwSource;
    DWORD   dwSrcMask;
    DWORD   dwUpStrmNgbr;
    DWORD   dwInIfIndex;
    DWORD   dwInIfProtocol;
    DWORD   dwRouteProtocol;
    DWORD   dwRouteNetwork;
    DWORD   dwRouteMask;
    ULONG   ulUpTime;
    ULONG   ulExpiryTime;
    ULONG   ulNumOutIf;
    ULONG   ulInPkts;
    ULONG   ulInOctets;
    ULONG   ulPktsDifferentIf;
    ULONG   ulQueueOverflow;

    MIB_IPMCAST_OIF_STATS   rgmiosOutStats[ANY_SIZE];
} MIB_IPMCAST_MFE_STATS, *PMIB_IPMCAST_MFE_STATS;

typedef struct _MIB_MFE_STATS_TABLE {
    DWORD       dwNumEntries;
    MIB_IPMCAST_MFE_STATS   table[ANY_SIZE];
} MIB_MFE_STATS_TABLE, *PMIB_MFE_STATS_TABLE;

#define SIZEOF_BASIC_MIB_MFE_STATS    \
    (ULONG)(FIELD_OFFSET(MIB_IPMCAST_MFE_STATS, rgmiosOutStats[0]))

#define SIZEOF_MIB_MFE_STATS(X)       \
    (SIZEOF_BASIC_MIB_MFE_STATS + ((X) * sizeof(MIB_IPMCAST_OIF_STATS)))


typedef struct _MIB_IPMCAST_MFE_STATS_EX_XP {
    DWORD   dwGroup;
    DWORD   dwSource;
    DWORD   dwSrcMask;
    DWORD   dwUpStrmNgbr;
    DWORD   dwInIfIndex;
    DWORD   dwInIfProtocol;
    DWORD   dwRouteProtocol;
    DWORD   dwRouteNetwork;
    DWORD   dwRouteMask;
    ULONG   ulUpTime;
    ULONG   ulExpiryTime;
    ULONG   ulNumOutIf;
    ULONG   ulInPkts;
    ULONG   ulInOctets;
    ULONG   ulPktsDifferentIf;
    ULONG   ulQueueOverflow;
    ULONG   ulUninitMfe;
    ULONG   ulNegativeMfe;
    ULONG   ulInDiscards;
    ULONG   ulInHdrErrors;
    ULONG   ulTotalOutPackets;

    MIB_IPMCAST_OIF_STATS   rgmiosOutStats[ANY_SIZE];
} MIB_IPMCAST_MFE_STATS_EX_XP,
 *PMIB_IPMCAST_MFE_STATS_EX_XP;
#if (NTDDI_VERSION >= NTDDI_WINXP)
typedef MIB_IPMCAST_MFE_STATS_EX_XP MIB_IPMCAST_MFE_STATS_EX;
typedef MIB_IPMCAST_MFE_STATS_EX_XP *PMIB_IPMCAST_MFE_STATS_EX;
#endif

typedef struct _MIB_MFE_STATS_TABLE_EX_XP {
    DWORD       dwNumEntries;
    PMIB_IPMCAST_MFE_STATS_EX_XP   table[ANY_SIZE];
} MIB_MFE_STATS_TABLE_EX_XP, *PMIB_MFE_STATS_TABLE_EX_XP;
#if (NTDDI_VERSION >= NTDDI_WINXP)
typedef MIB_MFE_STATS_TABLE_EX_XP MIB_MFE_STATS_TABLE_EX;
typedef MIB_MFE_STATS_TABLE_EX_XP *PMIB_MFE_STATS_TABLE_EX;

#define SIZEOF_BASIC_MIB_MFE_STATS_EX    \
    (ULONG)(FIELD_OFFSET(MIB_IPMCAST_MFE_STATS_EX, rgmiosOutStats[0]))

#define SIZEOF_MIB_MFE_STATS_EX(X)       \
    (SIZEOF_BASIC_MIB_MFE_STATS_EX + ((X) * sizeof(MIB_IPMCAST_OIF_STATS)))
#endif

typedef struct _MIB_IPMCAST_GLOBAL {
    DWORD   dwEnable;
} MIB_IPMCAST_GLOBAL, *PMIB_IPMCAST_GLOBAL;

typedef struct _MIB_IPMCAST_IF_ENTRY {
    DWORD   dwIfIndex;
    DWORD   dwTtl;
    DWORD   dwProtocol;
    DWORD   dwRateLimit;
    ULONG   ulInMcastOctets;
    ULONG   ulOutMcastOctets;
} MIB_IPMCAST_IF_ENTRY, *PMIB_IPMCAST_IF_ENTRY;

typedef struct _MIB_IPMCAST_IF_TABLE {
    DWORD       dwNumEntries;
    MIB_IPMCAST_IF_ENTRY   table[ANY_SIZE];
} MIB_IPMCAST_IF_TABLE, *PMIB_IPMCAST_IF_TABLE;

#define SIZEOF_MCAST_IF_TABLE(X) \
    (FIELD_OFFSET(MIB_IPMCAST_IF_TABLE, table[0]) +  \
     ((X) * sizeof(MIB_IPMCAST_IF_ENTRY)) +  \
     ALIGN_SIZE)

#if defined (_MSC_VER)
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201) /* nonstandard extension used : nameless struct/union */
#endif
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // _IPMIB_
