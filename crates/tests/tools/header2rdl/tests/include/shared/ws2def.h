/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ws2def.h

Abstract:

    This file contains the core definitions for the Winsock2
    specification that can be used by both user-mode and
    kernel mode modules.

    This file is included in WINSOCK2.H. User mode applications
    should include WINSOCK2.H rather than including this file
    directly. This file can not be included by a module that also
    includes WINSOCK.H.

Environment:

    user mode or kernel mode

--*/

#ifndef _WS2DEF_
#define _WS2DEF_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#if !defined(_WINSOCK_DEPRECATED_BY)
#if ((defined(_WINSOCK_DEPRECATED_NO_WARNINGS) || defined(BUILD_WINDOWS)) && !defined(_WINSOCK_DEPRECATE_WARNINGS)) || defined(MIDL_PASS)
#define _WINSOCK_DEPRECATED_BY(replacement)
#else
#define _WINSOCK_DEPRECATED_BY(replacement) __declspec(deprecated("Use " replacement " instead or define _WINSOCK_DEPRECATED_NO_WARNINGS to disable deprecated API warnings"))
#endif
#endif

#ifdef __cplusplus
extern "C" {
#endif

#if !defined(_WINSOCK2API_) && defined(_WINSOCKAPI_)
#error Do not include winsock.h and ws2def.h in the same module. Instead include only winsock2.h.
#endif

#pragma warning(push)
#pragma warning(disable:4201)
#pragma warning(disable:4214) // bit field types other than int

//
// Allow Winsock components to disable PREfast errors.
//

#if defined(_PREFAST_) && defined(IPV6_PREFAST_SAFE)
#include <ipv6prefast.h>
#endif // _PREFAST_

#if(_WIN32_WINNT >= 0x0600)

#ifdef _MSC_VER
#define WS2DEF_INLINE __inline
#else
#define WS2DEF_INLINE extern inline /* GNU style */
#endif

#endif//(_WIN32_WINNT >= 0x0600)

#include <inaddr.h>

//#if(_WIN32_WINNT >= 0x0600)

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
//
// Address families.
//
typedef USHORT ADDRESS_FAMILY;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

//#endif//(_WIN32_WINNT >= 0x0600)

//
// Although AF_UNSPEC is defined for backwards compatibility, using
// AF_UNSPEC for the "af" parameter when creating a socket is STRONGLY
// DISCOURAGED.  The interpretation of the "protocol" parameter
// depends on the actual address family chosen.  As environments grow
// to include more and more address families that use overlapping
// protocol values there is more and more chance of choosing an
// undesired address family when AF_UNSPEC is used.
//
#define AF_UNSPEC       0               // unspecified
#define AF_UNIX         1               // local to host (pipes, portals)
#define AF_INET         2               // internetwork: UDP, TCP, etc.
#define AF_IMPLINK      3               // arpanet imp addresses
#define AF_PUP          4               // pup protocols: e.g. BSP
#define AF_CHAOS        5               // mit CHAOS protocols
#define AF_NS           6               // XEROX NS protocols
#define AF_IPX          AF_NS           // IPX protocols: IPX, SPX, etc.
#define AF_ISO          7               // ISO protocols
#define AF_OSI          AF_ISO          // OSI is ISO
#define AF_ECMA         8               // european computer manufacturers
#define AF_DATAKIT      9               // datakit protocols
#define AF_CCITT        10              // CCITT protocols, X.25 etc
#define AF_SNA          11              // IBM SNA
#define AF_DECnet       12              // DECnet
#define AF_DLI          13              // Direct data link interface
#define AF_LAT          14              // LAT
#define AF_HYLINK       15              // NSC Hyperchannel
#define AF_APPLETALK    16              // AppleTalk
#define AF_NETBIOS      17              // NetBios-style addresses
#define AF_VOICEVIEW    18              // VoiceView
#define AF_FIREFOX      19              // Protocols from Firefox
#define AF_UNKNOWN1     20              // Somebody is using this!
#define AF_BAN          21              // Banyan
#define AF_ATM          22              // Native ATM Services
#define AF_INET6        23              // Internetwork Version 6
#define AF_CLUSTER      24              // Microsoft Wolfpack
#define AF_12844        25              // IEEE 1284.4 WG AF
#define AF_IRDA         26              // IrDA
#define AF_NETDES       28              // Network Designers OSI & gateway

#if(_WIN32_WINNT < 0x0501)
#define AF_MAX          29
#else //(_WIN32_WINNT < 0x0501)

#define AF_TCNPROCESS   29
#define AF_TCNMESSAGE   30
#define AF_ICLFXBM      31

#if(_WIN32_WINNT < 0x0600)
#define AF_MAX          32
#else //(_WIN32_WINNT < 0x0600)
#define AF_BTH          32              // Bluetooth RFCOMM/L2CAP protocols
#if(_WIN32_WINNT < 0x0601)
#define AF_MAX          33
#else //(_WIN32_WINNT < 0x0601)
#define AF_LINK         33
#if(_WIN32_WINNT < 0x0604)
#define AF_MAX          34
#else //(_WIN32_WINNT < 0x0604)
#define AF_HYPERV       34
#define AF_MAX          35
#endif //(_WIN32_WINNT < 0x0604)
#endif //(_WIN32_WINNT < 0x0601)
#endif //(_WIN32_WINNT < 0x0600)

#endif //(_WIN32_WINNT < 0x0501)

//
// Socket types.
//

#define SOCK_STREAM     1
#define SOCK_DGRAM      2
#define SOCK_RAW        3
#define SOCK_RDM        4
#define SOCK_SEQPACKET  5

//
// Define a level for socket I/O controls in the same numbering space as
// IPPROTO_TCP, IPPROTO_IP, etc.
//

#define SOL_SOCKET 0xffff
#define SOL_IP     (SOL_SOCKET-4)
#define SOL_IPV6   (SOL_SOCKET-5)

//
// Define socket-level options.
//

#define SO_DEBUG        0x0001      // turn on debugging info recording
#define SO_ACCEPTCONN   0x0002      // socket has had listen()
#define SO_REUSEADDR    0x0004      // allow local address reuse
#define SO_KEEPALIVE    0x0008      // keep connections alive
#define SO_DONTROUTE    0x0010      // just use interface addresses
#define SO_BROADCAST    0x0020      // permit sending of broadcast msgs
#define SO_USELOOPBACK  0x0040      // bypass hardware when possible
#define SO_LINGER       0x0080      // linger on close if data present
#define SO_OOBINLINE    0x0100      // leave received OOB data in line

#define SO_DONTLINGER   (int)(~SO_LINGER)
#define SO_EXCLUSIVEADDRUSE \
    ((int)(~SO_REUSEADDR))          // disallow local address reuse

#define SO_SNDBUF       0x1001      // send buffer size
#define SO_RCVBUF       0x1002      // receive buffer size
#define SO_SNDLOWAT     0x1003      // send low-water mark
#define SO_RCVLOWAT     0x1004      // receive low-water mark
#define SO_SNDTIMEO     0x1005      // send timeout
#define SO_RCVTIMEO     0x1006      // receive timeout
#define SO_ERROR        0x1007      // get error status and clear
#define SO_TYPE         0x1008      // get socket type
#define SO_BSP_STATE    0x1009      // get socket 5-tuple state

#define SO_GROUP_ID     0x2001      // ID of a socket group
#define SO_GROUP_PRIORITY 0x2002    // the relative priority within a group
#define SO_MAX_MSG_SIZE 0x2003      // maximum message size

#define SO_CONDITIONAL_ACCEPT 0x3002 // enable true conditional accept:
                                    // connection is not ack-ed to the
                                    // other side until conditional
                                    // function returns CF_ACCEPT
#define SO_PAUSE_ACCEPT 0x3003      // pause accepting new connections
#define SO_COMPARTMENT_ID 0x3004    // get/set the compartment for a socket
#if (_WIN32_WINNT >= 0x0600)
#define SO_RANDOMIZE_PORT 0x3005    // randomize assignment of wildcard ports
#define SO_PORT_SCALABILITY 0x3006  // enable port scalability
#define SO_REUSE_UNICASTPORT 0x3007 // defer ephemeral port allocation for
                                    // outbound connections
#define SO_REUSE_MULTICASTPORT 0x3008 // enable port reuse and disable unicast
                                      //reception.
#define SO_ORIGINAL_DST 0x300F      // Query the original destination address
                                    // of a redirected connection.
#define IP6T_SO_ORIGINAL_DST SO_ORIGINAL_DST
#endif //(_WIN32_WINNT >= 0x0600)

#if (NTDDI_VERSION >= NTDDI_WIN11_GE)
#define SO_RECEIVED_HOPLIMIT 0x3010
#define SO_RECEIVED_PROCESSOR 0x3011 // Receive the processor number packets
                                     // have been processed by
#endif // NTDDI_VERSION >= NTDDI_WIN11_GE

//
// Base constant used for defining WSK-specific options.
//

#define WSK_SO_BASE  0x4000

//
// Options to use with [gs]etsockopt at the IPPROTO_TCP level.
//

#define TCP_NODELAY         0x0001

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
//
// Structure used to store most addresses.
//
typedef struct sockaddr {

#if (_WIN32_WINNT < 0x0600)
    u_short sa_family;
#else
    ADDRESS_FAMILY sa_family;           // Address family.
#endif //(_WIN32_WINNT < 0x0600)

    CHAR sa_data[14];                   // Up to 14 bytes of direct address.
} SOCKADDR, *PSOCKADDR, FAR *LPSOCKADDR;

#ifndef __CSADDR_DEFINED__
#define __CSADDR_DEFINED__


/*
 * SockAddr Information
 */
typedef struct _SOCKET_ADDRESS {
    _Field_size_bytes_(iSockaddrLength) LPSOCKADDR lpSockaddr;

//  ESP: 791.
//    _When_(
//        lpSockaddr->sa_family == AF_INET,
//        _Field_range_(>=, sizeof(SOCKADDR_IN)))
//    _When_(
//        lpSockaddr->sa_family == AF_INET6,
//        _Field_range_(>=, sizeof(SOCKADDR_IN6)))
    INT iSockaddrLength;
} SOCKET_ADDRESS, *PSOCKET_ADDRESS, *LPSOCKET_ADDRESS;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

/*
 * Address list returned via SIO_ADDRESS_LIST_QUERY
 */
typedef struct _SOCKET_ADDRESS_LIST {
    INT             iAddressCount;
    SOCKET_ADDRESS  Address[1];
} SOCKET_ADDRESS_LIST, *PSOCKET_ADDRESS_LIST, FAR *LPSOCKET_ADDRESS_LIST;

#if (_WIN32_WINNT >= 0x0600)

#define SIZEOF_SOCKET_ADDRESS_LIST(AddressCount) \
    (FIELD_OFFSET(SOCKET_ADDRESS_LIST, Address) + \
     AddressCount * sizeof(SOCKET_ADDRESS))

#endif //(_WIN32_WINNT >= 0x0600)

/*
 * CSAddr Information
 */
typedef struct _CSADDR_INFO {
    SOCKET_ADDRESS LocalAddr ;
    SOCKET_ADDRESS RemoteAddr ;
    INT iSocketType ;
    INT iProtocol ;
} CSADDR_INFO, *PCSADDR_INFO, FAR * LPCSADDR_INFO ;
#endif /* __CSADDR_DEFINED__ */

//
// Portable socket structure (RFC 2553).
//

//
// Desired design of maximum size and alignment.
// These are implementation specific.
//
#define _SS_MAXSIZE 128                 // Maximum size
#define _SS_ALIGNSIZE (sizeof(__int64)) // Desired alignment

//
// Definitions used for sockaddr_storage structure paddings design.
//

#if(_WIN32_WINNT >= 0x0600)
#define _SS_PAD1SIZE (_SS_ALIGNSIZE - sizeof(USHORT))
#define _SS_PAD2SIZE (_SS_MAXSIZE - (sizeof(USHORT) + _SS_PAD1SIZE + _SS_ALIGNSIZE))
#else
#define _SS_PAD1SIZE (_SS_ALIGNSIZE - sizeof (short))
#define _SS_PAD2SIZE (_SS_MAXSIZE - (sizeof (short) + _SS_PAD1SIZE \
                                                    + _SS_ALIGNSIZE))
#endif //(_WIN32_WINNT >= 0x0600)

typedef struct sockaddr_storage {
    ADDRESS_FAMILY ss_family;      // address family

    CHAR __ss_pad1[_SS_PAD1SIZE];  // 6 byte pad, this is to make
                                   //   implementation specific pad up to
                                   //   alignment field that follows explicit
                                   //   in the data structure
    __int64 __ss_align;            // Field to force desired structure
    CHAR __ss_pad2[_SS_PAD2SIZE];  // 112 byte pad to achieve desired size;
                                   //   _SS_MAXSIZE value minus size of
                                   //   ss_family, __ss_pad1, and
                                   //   __ss_align fields is 112
} SOCKADDR_STORAGE_LH, *PSOCKADDR_STORAGE_LH, FAR *LPSOCKADDR_STORAGE_LH;

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef struct sockaddr_storage_xp {
    short ss_family;               // Address family.

    CHAR __ss_pad1[_SS_PAD1SIZE];  // 6 byte pad, this is to make
                                   //   implementation specific pad up to
                                   //   alignment field that follows explicit
                                   //   in the data structure
    __int64 __ss_align;            // Field to force desired structure
    CHAR __ss_pad2[_SS_PAD2SIZE];  // 112 byte pad to achieve desired size;
                                   //   _SS_MAXSIZE value minus size of
                                   //   ss_family, __ss_pad1, and
                                   //   __ss_align fields is 112
} SOCKADDR_STORAGE_XP, *PSOCKADDR_STORAGE_XP, FAR *LPSOCKADDR_STORAGE_XP;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#if(_WIN32_WINNT >= 0x0600)
typedef SOCKADDR_STORAGE_LH SOCKADDR_STORAGE;
typedef SOCKADDR_STORAGE *PSOCKADDR_STORAGE, FAR *LPSOCKADDR_STORAGE;
#elif (_WIN32_WINNT >= 0x0501)
typedef SOCKADDR_STORAGE_XP SOCKADDR_STORAGE;
typedef SOCKADDR_STORAGE *PSOCKADDR_STORAGE, FAR *LPSOCKADDR_STORAGE;
#endif

#if(_WIN32_WINNT >= 0x0602)
typedef struct _SOCKET_PROCESSOR_AFFINITY {
    PROCESSOR_NUMBER Processor;
    USHORT NumaNodeId;
    USHORT Reserved;
} SOCKET_PROCESSOR_AFFINITY, *PSOCKET_PROCESSOR_AFFINITY;
#endif //(_WIN32_WINNT >= 0x0602)


/*
 * WinSock 2 extension -- manifest constants for WSAIoctl()
 */
#define IOC_UNIX                      0x00000000
#define IOC_WS2                       0x08000000
#define IOC_PROTOCOL                  0x10000000
#define IOC_VENDOR                    0x18000000

#if (_WIN32_WINNT >= 0x0600)
/*
 * WSK-specific IO control codes are Winsock2 codes with the highest-order
 * 3 bits of the Vendor/AddressFamily-specific field set to 1.
 */
#define IOC_WSK                       (IOC_WS2|0x07000000)
#endif //(_WIN32_WINNT >= 0x0600)

#define _WSAIO(x,y)                   (IOC_VOID|(x)|(y))
#define _WSAIOR(x,y)                  (IOC_OUT|(x)|(y))
#define _WSAIOW(x,y)                  (IOC_IN|(x)|(y))
#define _WSAIORW(x,y)                 (IOC_INOUT|(x)|(y))

#define SIO_ASSOCIATE_HANDLE          _WSAIOW(IOC_WS2,1)
#define SIO_ENABLE_CIRCULAR_QUEUEING  _WSAIO(IOC_WS2,2)
#define SIO_FIND_ROUTE                _WSAIOR(IOC_WS2,3)
#define SIO_FLUSH                     _WSAIO(IOC_WS2,4)
#define SIO_GET_BROADCAST_ADDRESS     _WSAIOR(IOC_WS2,5)
#define SIO_GET_EXTENSION_FUNCTION_POINTER  _WSAIORW(IOC_WS2,6)
#define SIO_GET_QOS                   _WSAIORW(IOC_WS2,7)
#define SIO_GET_GROUP_QOS             _WSAIORW(IOC_WS2,8)
#define SIO_MULTIPOINT_LOOPBACK       _WSAIOW(IOC_WS2,9)
#define SIO_MULTICAST_SCOPE           _WSAIOW(IOC_WS2,10)
#define SIO_SET_QOS                   _WSAIOW(IOC_WS2,11)
#define SIO_SET_GROUP_QOS             _WSAIOW(IOC_WS2,12)
#define SIO_TRANSLATE_HANDLE          _WSAIORW(IOC_WS2,13)
#define SIO_ROUTING_INTERFACE_QUERY   _WSAIORW(IOC_WS2,20)
#define SIO_ROUTING_INTERFACE_CHANGE  _WSAIOW(IOC_WS2,21)
#define SIO_ADDRESS_LIST_QUERY        _WSAIOR(IOC_WS2,22)
#define SIO_ADDRESS_LIST_CHANGE       _WSAIO(IOC_WS2,23)
#define SIO_QUERY_TARGET_PNP_HANDLE   _WSAIOR(IOC_WS2,24)
#define SIO_QUERY_RSS_PROCESSOR_INFO  _WSAIOR(IOC_WS2,37)

#if(_WIN32_WINNT >= 0x0501)
#define SIO_ADDRESS_LIST_SORT         _WSAIORW(IOC_WS2,25)
#endif //(_WIN32_WINNT >= 0x0501)

#if (_WIN32_WINNT >= 0x0600)
#define SIO_RESERVED_1                _WSAIOW(IOC_WS2,26)
#define SIO_RESERVED_2                _WSAIOW(IOC_WS2,33)
#endif //(_WIN32_WINNT >= 0x0600)

#define SIO_GET_MULTIPLE_EXTENSION_FUNCTION_POINTER _WSAIORW(IOC_WS2,36)

//
// Constants and structures defined by the internet system (RFC 790)
//

//
// N.B. required for backwards compatability to support 0 = IP for the
// level argument to get/setsockopt.
//
#define IPPROTO_IP              0

//
// Protocols.  The IPv6 defines are specified in RFC 2292.
//
typedef enum {
#if(_WIN32_WINNT >= 0x0501)
    IPPROTO_HOPOPTS       = 0,  // IPv6 Hop-by-Hop options
#endif//(_WIN32_WINNT >= 0x0501)
    IPPROTO_ICMP          = 1,
    IPPROTO_IGMP          = 2,
    IPPROTO_GGP           = 3,
#if(_WIN32_WINNT >= 0x0501)
    IPPROTO_IPV4          = 4,
#endif//(_WIN32_WINNT >= 0x0501)
#if(_WIN32_WINNT >= 0x0600)
    IPPROTO_ST            = 5,
#endif//(_WIN32_WINNT >= 0x0600)
    IPPROTO_TCP           = 6,
#if(_WIN32_WINNT >= 0x0600)
    IPPROTO_CBT           = 7,
    IPPROTO_EGP           = 8,
    IPPROTO_IGP           = 9,
#endif//(_WIN32_WINNT >= 0x0600)
    IPPROTO_PUP           = 12,
    IPPROTO_UDP           = 17,
    IPPROTO_IDP           = 22,
#if(_WIN32_WINNT >= 0x0600)
    IPPROTO_RDP           = 27,
#endif//(_WIN32_WINNT >= 0x0600)

#if(_WIN32_WINNT >= 0x0501)
    IPPROTO_IPV6          = 41, // IPv6 header
    IPPROTO_ROUTING       = 43, // IPv6 Routing header
    IPPROTO_FRAGMENT      = 44, // IPv6 fragmentation header
    IPPROTO_ESP           = 50, // encapsulating security payload
    IPPROTO_AH            = 51, // authentication header
    IPPROTO_ICMPV6        = 58, // ICMPv6
    IPPROTO_NONE          = 59, // IPv6 no next header
    IPPROTO_DSTOPTS       = 60, // IPv6 Destination options
#endif//(_WIN32_WINNT >= 0x0501)

    IPPROTO_ND            = 77,
#if(_WIN32_WINNT >= 0x0501)
    IPPROTO_ICLFXBM       = 78,
#endif//(_WIN32_WINNT >= 0x0501)
#if(_WIN32_WINNT >= 0x0600)
    IPPROTO_PIM           = 103,
    IPPROTO_PGM           = 113,
    IPPROTO_L2TP          = 115,
    IPPROTO_SCTP          = 132,
#endif//(_WIN32_WINNT >= 0x0600)
    IPPROTO_RAW           = 255,

    IPPROTO_MAX           = 256,
//
//  These are reserved for internal use by Windows.
//
    IPPROTO_RESERVED_RAW  = 257,
    IPPROTO_RESERVED_IPSEC  = 258,
    IPPROTO_RESERVED_IPSECOFFLOAD  = 259,
    IPPROTO_RESERVED_WNV = 260,
    IPPROTO_RESERVED_MAX  = 261
} IPPROTO, *PIPROTO;

//
// Port/socket numbers: network standard functions
//
#define IPPORT_TCPMUX           1
#define IPPORT_ECHO             7
#define IPPORT_DISCARD          9
#define IPPORT_SYSTAT           11
#define IPPORT_DAYTIME          13
#define IPPORT_NETSTAT          15
#define IPPORT_QOTD             17
#define IPPORT_MSP              18
#define IPPORT_CHARGEN          19
#define IPPORT_FTP_DATA         20
#define IPPORT_FTP              21
#define IPPORT_TELNET           23
#define IPPORT_SMTP             25
#define IPPORT_TIMESERVER       37
#define IPPORT_NAMESERVER       42
#define IPPORT_WHOIS            43
#define IPPORT_MTP              57

/*
 * Port/socket numbers: host specific functions
 */
#define IPPORT_TFTP             69
#define IPPORT_RJE              77
#define IPPORT_FINGER           79
#define IPPORT_TTYLINK          87
#define IPPORT_SUPDUP           95

/*
 * UNIX TCP sockets
 */
#define IPPORT_POP3             110
#define IPPORT_NTP              123
#define IPPORT_EPMAP            135
#define IPPORT_NETBIOS_NS       137
#define IPPORT_NETBIOS_DGM      138
#define IPPORT_NETBIOS_SSN      139
#define IPPORT_IMAP             143
#define IPPORT_SNMP             161
#define IPPORT_SNMP_TRAP        162
#define IPPORT_IMAP3            220
#define IPPORT_LDAP             389
#define IPPORT_HTTPS            443
#define IPPORT_MICROSOFT_DS     445
#define IPPORT_EXECSERVER       512
#define IPPORT_LOGINSERVER      513
#define IPPORT_CMDSERVER        514
#define IPPORT_EFSSERVER        520

/*
 * UNIX UDP sockets
 */
#define IPPORT_BIFFUDP          512
#define IPPORT_WHOSERVER        513
#define IPPORT_ROUTESERVER      520
                                        /* 520+1 also used */

/*
 * Ports < IPPORT_RESERVED are reserved for
 * privileged processes (e.g. root).
 */
#define IPPORT_RESERVED         1024
#if (_WIN32_WINNT >= 0x0600)
#define IPPORT_REGISTERED_MIN   IPPORT_RESERVED
#define IPPORT_REGISTERED_MAX   0xbfff
#define IPPORT_DYNAMIC_MIN      0xc000
#define IPPORT_DYNAMIC_MAX      0xffff
#endif //(_WIN32_WINNT >= 0x0600)

/*
 * Definitions of bits in internet address integers.
 * On subnets, the decomposition of addresses to host and net parts
 * is done according to subnet mask, not the masks here.
 *
 * N.B. RFC-compliant definitions for host-order elements are named IN_xxx,
 * while network-order elements are named IN4_xxx.
 */
#define IN_CLASSA(i)            (((LONG)(i) & 0x80000000) == 0)
#define IN_CLASSA_NET           0xff000000
#define IN_CLASSA_NSHIFT        24
#define IN_CLASSA_HOST          0x00ffffff
#define IN_CLASSA_MAX           128

#define IN_CLASSB(i)            (((LONG)(i) & 0xc0000000) == 0x80000000)
#define IN_CLASSB_NET           0xffff0000
#define IN_CLASSB_NSHIFT        16
#define IN_CLASSB_HOST          0x0000ffff
#define IN_CLASSB_MAX           65536

#define IN_CLASSC(i)            (((LONG)(i) & 0xe0000000) == 0xc0000000)
#define IN_CLASSC_NET           0xffffff00
#define IN_CLASSC_NSHIFT        8
#define IN_CLASSC_HOST          0x000000ff

#define IN_CLASSD(i)            (((long)(i) & 0xf0000000) == 0xe0000000)
#define IN_CLASSD_NET           0xf0000000       /* These ones aren't really */
#define IN_CLASSD_NSHIFT        28               /* net and host fields, but */
#define IN_CLASSD_HOST          0x0fffffff       /* routing needn't know.    */
#define IN_MULTICAST(i)         IN_CLASSD(i)

#define INADDR_ANY              (ULONG)0x00000000
#define INADDR_LOOPBACK         0x7f000001
#define INADDR_BROADCAST        (ULONG)0xffffffff
#define INADDR_NONE             0xffffffff

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// Scope ID definition
//
typedef enum {
    ScopeLevelInterface    = 1,
    ScopeLevelLink         = 2,
    ScopeLevelSubnet       = 3,
    ScopeLevelAdmin        = 4,
    ScopeLevelSite         = 5,
    ScopeLevelOrganization = 8,
    ScopeLevelGlobal       = 14,
    ScopeLevelCount        = 16
} SCOPE_LEVEL;

typedef struct {
    union {
        struct {
            ULONG Zone : 28;
            ULONG Level : 4;
        } DUMMYSTRUCTNAME;
        ULONG Value;
    } DUMMYUNIONNAME;
} SCOPE_ID, *PSCOPE_ID;

#define SCOPEID_UNSPECIFIED_INIT    { 0 }

//
// IPv4 Socket address, Internet style
//

typedef struct sockaddr_in {

#if(_WIN32_WINNT < 0x0600)
    short   sin_family;
#else //(_WIN32_WINNT < 0x0600)
    ADDRESS_FAMILY sin_family;
#endif //(_WIN32_WINNT < 0x0600)

    USHORT sin_port;
    IN_ADDR sin_addr;
    CHAR sin_zero[8];
} SOCKADDR_IN, *PSOCKADDR_IN;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

//
// Datalink (MAC) address
//
// If you don't use the entire sdl_data field, then fill it starting with the low
// bytes...
//

#if(_WIN32_WINNT >= 0x0601)

typedef struct sockaddr_dl {
    ADDRESS_FAMILY sdl_family;
    UCHAR sdl_data[8];
    UCHAR sdl_zero[4];
} SOCKADDR_DL, *PSOCKADDR_DL;

#endif //(_WIN32_WINNT >= 0x0601)

#define IOCPARM_MASK    0x7f            /* parameters must be < 128 bytes */
#define IOC_VOID        0x20000000      /* no parameters */
#define IOC_OUT         0x40000000      /* copy out parameters */
#define IOC_IN          0x80000000      /* copy in parameters */
#define IOC_INOUT       (IOC_IN|IOC_OUT)
                                        /* 0x20000000 distinguishes new &
                                           old ioctl's */
#define _IO(x,y)        (IOC_VOID|((x)<<8)|(y))

#define _IOR(x,y,t)     (IOC_OUT|(((long)sizeof(t)&IOCPARM_MASK)<<16)|((x)<<8)|(y))

#define _IOW(x,y,t)     (IOC_IN|(((long)sizeof(t)&IOCPARM_MASK)<<16)|((x)<<8)|(y))

/*
 * WinSock 2 extension -- WSABUF and QOS struct, include qos.h
 * to pull in FLOWSPEC and related definitions
 */

typedef struct _WSABUF {
    ULONG len;     /* the length of the buffer */
    _Field_size_bytes_(len) CHAR FAR *buf; /* the pointer to the buffer */
} WSABUF, FAR * LPWSABUF;

/*
 * WSAMSG -- for WSASendMsg
 */

typedef struct _WSAMSG {
    _Field_size_bytes_(namelen) LPSOCKADDR       name;              /* Remote address */
    INT              namelen;           /* Remote address length */
    LPWSABUF         lpBuffers;         /* Data buffer array */

#if(_WIN32_WINNT >= 0x0600)
    ULONG            dwBufferCount;     /* Number of elements in the array */
#else
    DWORD            dwBufferCount;     /* Number of elements in the array */
#endif //(_WIN32_WINNT>=0x0600)

    WSABUF           Control;           /* Control buffer */

#if(_WIN32_WINNT >= 0x0600)
    ULONG            dwFlags;           /* Flags */
#else
    DWORD            dwFlags;           /* Flags */
#endif //(_WIN32_WINNT>=0x0600)

} WSAMSG, *PWSAMSG, * FAR LPWSAMSG;

/*
 * Layout of ancillary data objects in the control buffer (RFC 2292).
 */
#if(_WIN32_WINNT >= 0x0600)
#define _WSACMSGHDR cmsghdr
#endif //(_WIN32_WINNT>=0x0600)

typedef struct _WSACMSGHDR {
    SIZE_T      cmsg_len;
    INT         cmsg_level;
    INT         cmsg_type;
    /* followed by UCHAR cmsg_data[] */
} WSACMSGHDR, *PWSACMSGHDR, FAR *LPWSACMSGHDR;

#if(_WIN32_WINNT >= 0x0600)
typedef WSACMSGHDR CMSGHDR, *PCMSGHDR;
#endif //(_WIN32_WINNT>=0x0600)

/*
 * Alignment macros for header and data members of
 * the control buffer.
 */
#define WSA_CMSGHDR_ALIGN(length)                           \
            ( ((length) + TYPE_ALIGNMENT(WSACMSGHDR)-1) &   \
                (~(TYPE_ALIGNMENT(WSACMSGHDR)-1)) )         \

#define WSA_CMSGDATA_ALIGN(length)                          \
            ( ((length) + MAX_NATURAL_ALIGNMENT-1) &        \
                (~(MAX_NATURAL_ALIGNMENT-1)) )

#if(_WIN32_WINNT >= 0x0600)
#define CMSGHDR_ALIGN WSA_CMSGHDR_ALIGN
#define CMSGDATA_ALIGN WSA_CMSGDATA_ALIGN
#endif //(_WIN32_WINNT>=0x0600)

/*
 *  WSA_CMSG_FIRSTHDR
 *
 *  Returns a pointer to the first ancillary data object,
 *  or a null pointer if there is no ancillary data in the
 *  control buffer of the WSAMSG structure.
 *
 *  LPCMSGHDR
 *  WSA_CMSG_FIRSTHDR (
 *      LPWSAMSG    msg
 *      );
 */
#define WSA_CMSG_FIRSTHDR(msg) \
    ( ((msg)->Control.len >= sizeof(WSACMSGHDR))            \
        ? (LPWSACMSGHDR)(msg)->Control.buf                  \
        : (LPWSACMSGHDR)NULL )

#if(_WIN32_WINNT >= 0x0600)
#define CMSG_FIRSTHDR WSA_CMSG_FIRSTHDR
#endif //(_WIN32_WINNT>=0x0600)

/*
 *  WSA_CMSG_NXTHDR
 *
 *  Returns a pointer to the next ancillary data object,
 *  or a null if there are no more data objects.
 *
 *  LPCMSGHDR
 *  WSA_CMSG_NEXTHDR (
 *      LPWSAMSG        msg,
 *      LPWSACMSGHDR    cmsg
 *      );
 */
#define WSA_CMSG_NXTHDR(msg, cmsg)                          \
    ( ((cmsg) == NULL)                                      \
        ? WSA_CMSG_FIRSTHDR(msg)                            \
        : ( ( ((PUCHAR)(cmsg) +                             \
                    WSA_CMSGHDR_ALIGN((cmsg)->cmsg_len) +   \
                    sizeof(WSACMSGHDR) ) >                  \
                (PUCHAR)((msg)->Control.buf) +              \
                    (msg)->Control.len )                    \
            ? (LPWSACMSGHDR)NULL                            \
            : (LPWSACMSGHDR)((PUCHAR)(cmsg) +               \
                WSA_CMSGHDR_ALIGN((cmsg)->cmsg_len)) ) )

#if(_WIN32_WINNT >= 0x0600)
#define CMSG_NXTHDR WSA_CMSG_NXTHDR
#endif //(_WIN32_WINNT>=0x0600)

/*
 *  WSA_CMSG_DATA
 *
 *  Returns a pointer to the first byte of data (what is referred
 *  to as the cmsg_data member though it is not defined in
 *  the structure).
 *
 *  Note that RFC 2292 defines this as CMSG_DATA, but that name
 *  is already used by wincrypt.h, and so Windows has used WSA_CMSG_DATA.
 *
 *  PUCHAR
 *  WSA_CMSG_DATA (
 *      LPWSACMSGHDR   pcmsg
 *      );
 */
#define WSA_CMSG_DATA(cmsg)             \
            ( (PUCHAR)(cmsg) + WSA_CMSGDATA_ALIGN(sizeof(WSACMSGHDR)) )

/*
 *  WSA_CMSG_SPACE
 *
 *  Returns total size of an ancillary data object given
 *  the amount of data. Used to allocate the correct amount
 *  of space.
 *
 *  SIZE_T
 *  WSA_CMSG_SPACE (
 *      SIZE_T length
 *      );
 */
#define WSA_CMSG_SPACE(length)  \
        (WSA_CMSGDATA_ALIGN(sizeof(WSACMSGHDR) + WSA_CMSGHDR_ALIGN(length)))

#if(_WIN32_WINNT >= 0x0600)
#define CMSG_SPACE WSA_CMSG_SPACE
#endif //(_WIN32_WINNT>=0x0600)

/*
 *  WSA_CMSG_LEN
 *
 *  Returns the value to store in cmsg_len given the amount of data.
 *
 *  SIZE_T
 *  WSA_CMSG_LEN (
 *      SIZE_T length
 *  );
 */
#define WSA_CMSG_LEN(length)    \
         (WSA_CMSGDATA_ALIGN(sizeof(WSACMSGHDR)) + length)

#if(_WIN32_WINNT >= 0x0600)
#define CMSG_LEN WSA_CMSG_LEN
#endif //(_WIN32_WINNT>=0x0600)

/*
 * Definition for flags member of the WSAMSG structure
 * This is in addition to other MSG_xxx flags defined
 * for recv/recvfrom/send/sendto.
 */
#define MSG_TRUNC       0x0100
#define MSG_CTRUNC      0x0200
#define MSG_BCAST       0x0400
#define MSG_MCAST       0x0800
#define MSG_ERRQUEUE    0x1000

//
//  Flags used in "hints" argument to getaddrinfo()
//      - AI_ADDRCONFIG is supported starting with Vista
//      - default is AI_ADDRCONFIG ON whether the flag is set or not
//        because the performance penalty in not having ADDRCONFIG in
//        the multi-protocol stack environment is severe;
//        this defaulting may be disabled by specifying the AI_ALL flag,
//        in that case AI_ADDRCONFIG must be EXPLICITLY specified to
//        enable ADDRCONFIG behavior
//

#define AI_PASSIVE                  0x00000001  // Socket address will be used in bind() call
#define AI_CANONNAME                0x00000002  // Return canonical name in first ai_canonname
#define AI_NUMERICHOST              0x00000004  // Nodename must be a numeric address string
#define AI_NUMERICSERV              0x00000008  // Servicename must be a numeric port number
#define AI_DNS_ONLY                 0x00000010  // Restrict queries to unicast DNS only (no LLMNR, netbios, etc.)
#define AI_FORCE_CLEAR_TEXT         0x00000020  // Force clear text DNS query
#define AI_BYPASS_DNS_CACHE         0x00000040  // Bypass DNS cache
#define AI_RETURN_TTL               0x00000080  // Return record TTL

#define AI_ALL                      0x00000100  // Query both IP6 and IP4 with AI_V4MAPPED
#define AI_ADDRCONFIG               0x00000400  // Resolution only if global address configured
#define AI_V4MAPPED                 0x00000800  // On v6 failure, query v4 and convert to V4MAPPED format

#define AI_NON_AUTHORITATIVE        0x00004000  // LUP_NON_AUTHORITATIVE
#define AI_SECURE                   0x00008000  // LUP_SECURE
#define AI_RETURN_PREFERRED_NAMES   0x00010000  // LUP_RETURN_PREFERRED_NAMES

#define AI_FQDN                     0x00020000  // Return the FQDN in ai_canonname
#define AI_FILESERVER               0x00040000  // Resolving fileserver name resolution
#define AI_DISABLE_IDN_ENCODING     0x00080000  // Disable Internationalized Domain Names handling
#define AI_SECURE_WITH_FALLBACK     0x00100000  // Forces clear text fallback if the secure DNS query fails
#define AI_EXCLUSIVE_CUSTOM_SERVERS 0x00200000  // Use exclusively the custom DNS servers
#define AI_RETURN_RESPONSE_FLAGS    0x10000000  // Requests extra information about the DNS results
#define AI_REQUIRE_SECURE           0x20000000  // Forces the DNS query to be done over seucre protocols
#define AI_RESOLUTION_HANDLE        0x40000000  // Request resolution handle
#define AI_EXTENDED                 0x80000000  // Indicates this is extended ADDRINFOEX(2/..) struct

//
//  Structure used in getaddrinfo() call
//

typedef struct addrinfo
{
    int                 ai_flags;       // AI_PASSIVE, AI_CANONNAME, AI_NUMERICHOST
    int                 ai_family;      // PF_xxx
    int                 ai_socktype;    // SOCK_xxx
    int                 ai_protocol;    // 0 or IPPROTO_xxx for IPv4 and IPv6
    size_t              ai_addrlen;     // Length of ai_addr
    char *              ai_canonname;   // Canonical name for nodename
    _Field_size_bytes_(ai_addrlen) struct sockaddr *   ai_addr;        // Binary address
    struct addrinfo *   ai_next;        // Next structure in linked list
}
ADDRINFOA, *PADDRINFOA;

typedef struct addrinfoW
{
    int                 ai_flags;       // AI_PASSIVE, AI_CANONNAME, AI_NUMERICHOST
    int                 ai_family;      // PF_xxx
    int                 ai_socktype;    // SOCK_xxx
    int                 ai_protocol;    // 0 or IPPROTO_xxx for IPv4 and IPv6
    size_t              ai_addrlen;     // Length of ai_addr
    PWSTR               ai_canonname;   // Canonical name for nodename
    _Field_size_bytes_(ai_addrlen) struct sockaddr *   ai_addr;        // Binary address
    struct addrinfoW *  ai_next;        // Next structure in linked list
}
ADDRINFOW, *PADDRINFOW;

#if (_WIN32_WINNT >= 0x0600)

#pragma region App Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
typedef struct _WINSOCK_DEPRECATED_BY("ADDRINFOEXW") addrinfoexA
{
    int                 ai_flags;       // AI_PASSIVE, AI_CANONNAME, AI_NUMERICHOST
    int                 ai_family;      // PF_xxx
    int                 ai_socktype;    // SOCK_xxx
    int                 ai_protocol;    // 0 or IPPROTO_xxx for IPv4 and IPv6
    size_t              ai_addrlen;     // Length of ai_addr
    char               *ai_canonname;   // Canonical name for nodename
    struct sockaddr    *ai_addr;        // Binary address
    void               *ai_blob;
    size_t              ai_bloblen;
    LPGUID              ai_provider;
    struct addrinfoexA *ai_next;        // Next structure in linked list
} ADDRINFOEXA, *PADDRINFOEXA, *LPADDRINFOEXA;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

typedef struct addrinfoexW
{
    int                 ai_flags;       // AI_PASSIVE, AI_CANONNAME, AI_NUMERICHOST
    int                 ai_family;      // PF_xxx
    int                 ai_socktype;    // SOCK_xxx
    int                 ai_protocol;    // 0 or IPPROTO_xxx for IPv4 and IPv6
    size_t              ai_addrlen;     // Length of ai_addr
    PWSTR               ai_canonname;   // Canonical name for nodename
    _Field_size_bytes_(ai_addrlen) struct sockaddr    *ai_addr;        // Binary address
    _Field_size_(ai_bloblen) void               *ai_blob;
    size_t              ai_bloblen;
    LPGUID              ai_provider;
    struct addrinfoexW *ai_next;        // Next structure in linked list
} ADDRINFOEXW, *PADDRINFOEXW, *LPADDRINFOEXW;

#endif

#if (_WIN32_WINNT >= 0x0602)

#define ADDRINFOEX_VERSION_2    2
#define ADDRINFOEX_VERSION_3    3
#define ADDRINFOEX_VERSION_4    4
#define ADDRINFOEX_VERSION_5    5
#define ADDRINFOEX_VERSION_6    6
#define ADDRINFOEX_VERSION_7    7

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef struct _WINSOCK_DEPRECATED_BY("ADDRINFOEX2W") addrinfoex2A
{
    int                  ai_flags;       // AI_PASSIVE, AI_CANONNAME, AI_NUMERICHOST
    int                  ai_family;      // PF_xxx
    int                  ai_socktype;    // SOCK_xxx
    int                  ai_protocol;    // 0 or IPPROTO_xxx for IPv4 and IPv6
    size_t               ai_addrlen;     // Length of ai_addr
    char                *ai_canonname;   // Canonical name for nodename
    struct sockaddr     *ai_addr;        // Binary address
    void                *ai_blob;
    size_t              ai_bloblen;
    LPGUID               ai_provider;
    struct addrinfoex2A *ai_next;        // Next structure in linked list
    int                  ai_version;
    char                *ai_fqdn;
} ADDRINFOEX2A, *PADDRINFOEX2A, *LPADDRINFOEX2A;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

typedef struct addrinfoex2W
{
    int                  ai_flags;       // AI_PASSIVE, AI_CANONNAME, AI_NUMERICHOST
    int                  ai_family;      // PF_xxx
    int                  ai_socktype;    // SOCK_xxx
    int                  ai_protocol;    // 0 or IPPROTO_xxx for IPv4 and IPv6
    size_t               ai_addrlen;     // Length of ai_addr
    PWSTR                ai_canonname;   // Canonical name for nodename
    _Field_size_bytes_(ai_addrlen) struct sockaddr    *ai_addr;        // Binary address
    _Field_size_(ai_bloblen) void               *ai_blob;
    size_t               ai_bloblen;
    LPGUID               ai_provider;
    struct addrinfoex2W *ai_next;        // Next structure in linked list
    int                  ai_version;
    PWSTR                ai_fqdn;
} ADDRINFOEX2W, *PADDRINFOEX2W, *LPADDRINFOEX2W;

typedef struct addrinfoex3
{
    int                  ai_flags;       // AI_PASSIVE, AI_CANONNAME, AI_NUMERICHOST
    int                  ai_family;      // PF_xxx
    int                  ai_socktype;    // SOCK_xxx
    int                  ai_protocol;    // 0 or IPPROTO_xxx for IPv4 and IPv6
    size_t               ai_addrlen;     // Length of ai_addr
    PWSTR                ai_canonname;   // Canonical name for nodename
    _Field_size_bytes_(ai_addrlen) struct sockaddr    *ai_addr;        // Binary address
    _Field_size_(ai_bloblen) void               *ai_blob;
    size_t               ai_bloblen;
    LPGUID                 ai_provider;
    struct addrinfoex3   *ai_next;        // Next structure in linked list
    int                  ai_version;
    PWSTR                ai_fqdn;
    int                  ai_interfaceindex;
} ADDRINFOEX3, *PADDRINFOEX3, *LPADDRINFOEX3;

typedef struct addrinfoex4
{
    int                  ai_flags;       // AI_PASSIVE, AI_CANONNAME, AI_NUMERICHOST
    int                  ai_family;      // PF_xxx
    int                  ai_socktype;    // SOCK_xxx
    int                  ai_protocol;    // 0 or IPPROTO_xxx for IPv4 and IPv6
    size_t               ai_addrlen;     // Length of ai_addr
    PWSTR                ai_canonname;   // Canonical name for nodename
    _Field_size_bytes_(ai_addrlen) struct sockaddr    *ai_addr;        // Binary address
    _Field_size_(ai_bloblen) void               *ai_blob;
    size_t               ai_bloblen;
    GUID                 *ai_provider;
    struct addrinfoex4   *ai_next;        // Next structure in linked list
    int                  ai_version;
    PWSTR                ai_fqdn;
    int                  ai_interfaceindex;
    HANDLE               ai_resolutionhandle;
} ADDRINFOEX4, *PADDRINFOEX4, *LPADDRINFOEX4;

typedef struct addrinfoex5
{
    int                  ai_flags;       // AI_PASSIVE, AI_CANONNAME, AI_NUMERICHOST
    int                  ai_family;      // PF_xxx
    int                  ai_socktype;    // SOCK_xxx
    int                  ai_protocol;    // 0 or IPPROTO_xxx for IPv4 and IPv6
    size_t               ai_addrlen;     // Length of ai_addr
    PWSTR                ai_canonname;   // Canonical name for nodename
    _Field_size_bytes_(ai_addrlen) struct sockaddr    *ai_addr;        // Binary address
    _Field_size_(ai_bloblen) void               *ai_blob;
    size_t               ai_bloblen;
    GUID                 *ai_provider;
    struct addrinfoex5   *ai_next;        // Next structure in linked list
    int                  ai_version;
    PWSTR                ai_fqdn;
    int                  ai_interfaceindex;
    HANDLE               ai_resolutionhandle;
    unsigned int         ai_ttl;          // Number of seconds for which this DNS record is valid
} ADDRINFOEX5, *PADDRINFOEX5, *LPADDRINFOEX5;


//
// Types of custom DNS servers specified in the ai_servers parameter.
// These options will be set in the ai_servertype field in the ADDRINFO_DNS_SERVER struct.
//

#define AI_DNS_SERVER_TYPE_UDP 0x1
#define AI_DNS_SERVER_TYPE_DOH 0x2
#define AI_DNS_SERVER_TYPE_DOT 0x3


//
// Flags for custom servers.
// These options will be set in the ai_flags field in the ADDRINFO_DNS_SERVER struct.
//

#define AI_DNS_SERVER_UDP_FALLBACK 0x1

typedef struct addrinfo_dns_server
{
    unsigned int     ai_servertype;
    unsigned __int64 ai_flags;
    unsigned int     ai_addrlen;
    _Field_size_bytes_(ai_addrlen) struct sockaddr *ai_addr;

    union
    {
        PWSTR ai_template;
        PWSTR ai_hostname;
    };
} ADDRINFO_DNS_SERVER;


//
// Flags returned through ai_returnflags, when AI_RETURN_RESPONSE_FLAGS is set
//

#define AI_DNS_RESPONSE_SECURE   0x1     // Present if the resolution was done through secure protocols
#define AI_DNS_RESPONSE_HOSTFILE 0x2

typedef struct addrinfoex6
{
    int                  ai_flags;       // AI_PASSIVE, AI_CANONNAME, AI_NUMERICHOST
    int                  ai_family;      // PF_xxx
    int                  ai_socktype;    // SOCK_xxx
    int                  ai_protocol;    // 0 or IPPROTO_xxx for IPv4 and IPv6
    size_t               ai_addrlen;     // Length of ai_addr
    PWSTR                ai_canonname;   // Canonical name for nodename
    _Field_size_bytes_(ai_addrlen) struct sockaddr    *ai_addr;        // Binary address
    _Field_size_(ai_bloblen) void               *ai_blob;
    size_t               ai_bloblen;
    GUID                 *ai_provider;
    struct addrinfoex5   *ai_next;        // Next structure in linked list
    int                  ai_version;
    PWSTR                ai_fqdn;
    int                  ai_interfaceindex;
    HANDLE               ai_resolutionhandle;
    unsigned int         ai_ttl;          // Number of seconds for which this DNS record is valid
    unsigned int         ai_numservers;
    ADDRINFO_DNS_SERVER  *ai_servers;
    ULONG64              ai_responseflags;
} ADDRINFOEX6, *PADDRINFOEX6;

//
//  Flags for ai_extraflags
//

#define AI_EXTRA_DNSSEC_REQUIRED    0x0000000000000001 // Set DO and AD bits in request, returns IP address info IF DNSSEC authenticated, NOTE: only the IP address will be returned not the DNSSEC specific records

typedef struct addrinfoex7
{
    int                  ai_flags;       // AI_PASSIVE, AI_CANONNAME, AI_NUMERICHOST
    int                  ai_family;      // PF_xxx
    int                  ai_socktype;    // SOCK_xxx
    int                  ai_protocol;    // 0 or IPPROTO_xxx for IPv4 and IPv6
    size_t               ai_addrlen;     // Length of ai_addr
    PWSTR                ai_canonname;   // Canonical name for nodename
    _Field_size_bytes_(ai_addrlen) struct sockaddr    *ai_addr;        // Binary address
    _Field_size_(ai_bloblen) void               *ai_blob;
    size_t               ai_bloblen;
    GUID                 *ai_provider;
    struct addrinfoex7   *ai_next;        // Next structure in linked list
    int                  ai_version;
    PWSTR                ai_fqdn;
    int                  ai_interfaceindex;
    HANDLE               ai_resolutionhandle;
    unsigned int         ai_ttl;          // Number of seconds for which this DNS record is valid
    unsigned int         ai_numservers;
    ADDRINFO_DNS_SERVER  *ai_servers;
    ULONG64              ai_responseflags;
    ULONG64              ai_extraflags;   // additional dns options
} ADDRINFOEX7, *PADDRINFOEX7;

#endif

//
// Flags for getaddrinfo()
//

// Name Spaces

#define NS_ALL                      (0)

#define NS_SAP                      (1)
#define NS_NDS                      (2)
#define NS_PEER_BROWSE              (3)
#define NS_SLP                      (5)
#define NS_DHCP                     (6)

#define NS_TCPIP_LOCAL              (10)
#define NS_TCPIP_HOSTS              (11)
#define NS_DNS                      (12)
#define NS_NETBT                    (13)
#define NS_WINS                     (14)

#if(_WIN32_WINNT >= 0x0501)
#define NS_NLA                      (15)    /* Network Location Awareness */
#endif //(_WIN32_WINNT >= 0x0501)

#if(_WIN32_WINNT >= 0x0600)
#define NS_BTH                      (16)    /* Bluetooth SDP Namespace */
#endif //(_WIN32_WINNT >= 0x0600)

#define NS_NBP                      (20)

#define NS_MS                       (30)
#define NS_STDA                     (31)
#define NS_NTDS                     (32)

#if(_WIN32_WINNT >= 0x0600)
#define NS_EMAIL                    (37)
#define NS_PNRPNAME                 (38)
#define NS_PNRPCLOUD                (39)
#endif //(_WIN32_WINNT >= 0x0600)

#define NS_X500                     (40)
#define NS_NIS                      (41)
#define NS_NISPLUS                  (42)

#define NS_WRQ                      (50)

#define NS_NETDES                   (60)    /* Network Designers Limited */

//
// Flags for getnameinfo()
//

#define NI_NOFQDN       0x01  /* Only return nodename portion for local hosts */
#define NI_NUMERICHOST  0x02  /* Return numeric form of the host's address */
#define NI_NAMEREQD     0x04  /* Error if the host's name not in DNS */
#define NI_NUMERICSERV  0x08  /* Return numeric form of the service (port #) */
#define NI_DGRAM        0x10  /* Service is a datagram service */

#define NI_MAXHOST      1025  /* Max size of a fully-qualified domain name */
#define NI_MAXSERV      32    /* Max size of a service name */

#pragma warning(pop)

#ifdef __cplusplus
}
#endif
#endif
