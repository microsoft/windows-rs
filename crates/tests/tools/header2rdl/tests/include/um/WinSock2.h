

/* Winsock2.h -- definitions to be used with the WinSock 2 DLL and
 *               WinSock 2 applications.
 *
 * This header file corresponds to version 2.2.x of the WinSock API
 * specification.
 *
 * This file includes parts which are Copyright (c) 1982-1986 Regents
 * of the University of California.  All rights reserved.  The
 * Berkeley Software License Agreement specifies the terms and
 * conditions for redistribution.
 */

#ifndef _WINSOCK2API_
#define _WINSOCK2API_
#define _WINSOCKAPI_   /* Prevent inclusion of winsock.h in windows.h */

#pragma once
#include <winapifamily.h>

#if !defined(_WINSOCK_DEPRECATED_BY)
#if ((defined(_WINSOCK_DEPRECATED_NO_WARNINGS) || defined(BUILD_WINDOWS)) && !defined(_WINSOCK_DEPRECATE_WARNINGS)) || defined(MIDL_PASS)
#define _WINSOCK_DEPRECATED_BY(replacement)
#else
#define _WINSOCK_DEPRECATED_BY(replacement) __declspec(deprecated("Use " replacement " instead or define _WINSOCK_DEPRECATED_NO_WARNINGS to disable deprecated API warnings"))
#endif
#endif
#if !defined(_WINSOCK_DEPRECATED)
#if ((defined(_WINSOCK_DEPRECATED_NO_WARNINGS) || defined(BUILD_WINDOWS)) && !defined(_WINSOCK_DEPRECATE_WARNINGS)) || defined(MIDL_PASS)
#define _WINSOCK_DEPRECATED
#else
#define _WINSOCK_DEPRECATED __declspec(deprecated("Define _WINSOCK_DEPRECATED_NO_WARNINGS to disable deprecated API warnings"))
#endif
#endif

/*
 * Default: include function prototypes, don't include function typedefs.
 */

#ifndef INCL_WINSOCK_API_PROTOTYPES
#define INCL_WINSOCK_API_PROTOTYPES 1
#endif

#ifndef INCL_WINSOCK_API_TYPEDEFS
#define INCL_WINSOCK_API_TYPEDEFS 0
#endif

/*
 * Pull in WINDOWS.H if necessary
 */
#ifndef _INC_WINDOWS
#include <windows.h>
#endif /* _INC_WINDOWS */

/*
 * Ensure structures are packed consistently.
 * Not necessary for WIN32, it is already packed >=4 and there are
 * no structures in this header that have alignment requirement
 * higher than 4.
 * For WIN64 we do not have compatibility requirement because it is
 * not possible to mix 32/16 bit code with 64 bit code in the same
 * process.
 */

#if (!defined(_WIN64) && !defined(WIN32))
#include <pshpack4.h>
/* WIN32 can be defined between here and the required poppack
   so define this special macro to ensure poppack */
#define _NEED_POPPACK
#endif

/*
 * Define the current Winsock version. To build an earlier Winsock version
 * application redefine this value prior to including Winsock2.h.
 */

#if !defined(MAKEWORD)
#define MAKEWORD(low,high) \
        ((WORD)(((BYTE)(low)) | ((WORD)((BYTE)(high))) << 8))
#endif

#ifndef WINSOCK_VERSION
#define WINSOCK_VERSION MAKEWORD(2,2)
#endif

/*
 * Establish DLL function linkage if supported by the current build
 * environment and not previously defined.
 */

#ifndef WINSOCK_API_LINKAGE
#ifdef DECLSPEC_IMPORT
#define WINSOCK_API_LINKAGE DECLSPEC_IMPORT
#else
#define WINSOCK_API_LINKAGE
#endif
#endif

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Basic system type definitions, taken from the BSD file sys/types.h.
 */
typedef unsigned char   u_char;
typedef unsigned short  u_short;
typedef unsigned int    u_int;
typedef unsigned long   u_long;

#if(_WIN32_WINNT >= 0x0501)
typedef unsigned __int64 u_int64;
#endif //(_WIN32_WINNT >= 0x0501)

#include <ws2def.h>

/*
 * The new type to be used in all
 * instances which refer to sockets.
 */
typedef UINT_PTR        SOCKET;

/*
 * Select uses arrays of SOCKETs.  These macros manipulate such
 * arrays.  FD_SETSIZE may be defined by the user before including
 * this file, but the default here should be >= 64.
 *
 * CAVEAT IMPLEMENTOR and USER: THESE MACROS AND TYPES MUST BE
 * INCLUDED IN WINSOCK2.H EXACTLY AS SHOWN HERE.
 */
#ifndef FD_SETSIZE
#define FD_SETSIZE      64
#endif /* FD_SETSIZE */

typedef struct fd_set {
        u_int fd_count;               /* how many are SET? */
        SOCKET  fd_array[FD_SETSIZE];   /* an array of SOCKETs */
} fd_set;

extern int PASCAL FAR __WSAFDIsSet(SOCKET fd, fd_set FAR *);

#define FD_CLR(fd, set) do { \
    u_int __i; \
    for (__i = 0; __i < ((fd_set FAR *)(set))->fd_count ; __i++) { \
        if (((fd_set FAR *)(set))->fd_array[__i] == fd) { \
            while (__i < ((fd_set FAR *)(set))->fd_count-1) { \
                ((fd_set FAR *)(set))->fd_array[__i] = \
                    ((fd_set FAR *)(set))->fd_array[__i+1]; \
                __i++; \
            } \
            ((fd_set FAR *)(set))->fd_count--; \
            break; \
        } \
    } \
} while(0)

#define FD_SET(fd, set) do { \
    u_int __i; \
    for (__i = 0; __i < ((fd_set FAR *)(set))->fd_count; __i++) { \
        if (((fd_set FAR *)(set))->fd_array[__i] == (fd)) { \
            break; \
        } \
    } \
    if (__i == ((fd_set FAR *)(set))->fd_count) { \
        if (((fd_set FAR *)(set))->fd_count < FD_SETSIZE) { \
            ((fd_set FAR *)(set))->fd_array[__i] = (fd); \
            ((fd_set FAR *)(set))->fd_count++; \
        } \
    } \
} while(0)

#define FD_ZERO(set) (((fd_set FAR *)(set))->fd_count=0)

#define FD_ISSET(fd, set) __WSAFDIsSet((SOCKET)(fd), (fd_set FAR *)(set))

/*
 * Structure used in select() call, taken from the BSD file sys/time.h.
 */
struct timeval {
        long    tv_sec;         /* seconds */
        long    tv_usec;        /* and microseconds */
};

/*
 * Operations on timevals.
 *
 * NB: timercmp does not work for >= or <=.
 */
#define timerisset(tvp)         ((tvp)->tv_sec || (tvp)->tv_usec)
#define timercmp(tvp, uvp, cmp) \
        ((tvp)->tv_sec cmp (uvp)->tv_sec || \
         (tvp)->tv_sec == (uvp)->tv_sec && (tvp)->tv_usec cmp (uvp)->tv_usec)
#define timerclear(tvp)         (tvp)->tv_sec = (tvp)->tv_usec = 0

/*
 * Commands for ioctlsocket(),  taken from the BSD file fcntl.h.
 *
 *
 * Ioctl's have the command encoded in the lower word,
 * and the size of any in or out parameters in the upper
 * word.  The high 2 bits of the upper word are used
 * to encode the in/out status of the parameter; for now
 * we restrict parameters to at most 128 bytes.
 */
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

#define FIONREAD    _IOR('f', 127, u_long) /* get # bytes to read */
#define FIONBIO     _IOW('f', 126, u_long) /* set/clear non-blocking i/o */
#define FIOASYNC    _IOW('f', 125, u_long) /* set/clear async i/o */

/* Socket I/O Controls */
#define SIOCSHIWAT  _IOW('s',  0, u_long)  /* set high watermark */
#define SIOCGHIWAT  _IOR('s',  1, u_long)  /* get high watermark */
#define SIOCSLOWAT  _IOW('s',  2, u_long)  /* set low watermark */
#define SIOCGLOWAT  _IOR('s',  3, u_long)  /* get low watermark */
#define SIOCATMARK  _IOR('s',  7, u_long)  /* at oob mark? */

/*
 * Structures returned by network data base library, taken from the
 * BSD file netdb.h.  All addresses are supplied in host order, and
 * returned in network order (suitable for use in system calls).
 */

struct  hostent {
        char    FAR * h_name;           /* official name of host */
        char    FAR * FAR * h_aliases;  /* alias list */
        short   h_addrtype;             /* host address type */
        short   h_length;               /* length of address */
        char    FAR * FAR * h_addr_list; /* list of addresses */
#define h_addr  h_addr_list[0]          /* address, for backward compat */
};

/*
 * It is assumed here that a network number
 * fits in 32 bits.
 */
struct  netent {
        char    FAR * n_name;           /* official name of net */
        char    FAR * FAR * n_aliases;  /* alias list */
        short   n_addrtype;             /* net address type */
        u_long  n_net;                  /* network # */
};

struct  servent {
        char    FAR * s_name;           /* official service name */
        char    FAR * FAR * s_aliases;  /* alias list */
#ifdef _WIN64
        char    FAR * s_proto;          /* protocol to use */
        short   s_port;                 /* port # */
#else
        short   s_port;                 /* port # */
        char    FAR * s_proto;          /* protocol to use */
#endif
};

struct  protoent {
        char    FAR * p_name;           /* official protocol name */
        char    FAR * FAR * p_aliases;  /* alias list */
        short   p_proto;                /* protocol # */
};

/*
 * Constants and structures defined by the internet system,
 * Per RFC 790, September 1981, taken from the BSD file netinet/in.h.
 * IPv6 additions per RFC 2292.
 */

/*
 * Port/socket numbers: network standard functions
 */
#define IPPORT_ECHO             7
#define IPPORT_DISCARD          9
#define IPPORT_SYSTAT           11
#define IPPORT_DAYTIME          13
#define IPPORT_NETSTAT          15
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

/*
 * Link numbers
 */
#define IMPLINK_IP              155
#define IMPLINK_LOWEXPER        156
#define IMPLINK_HIGHEXPER       158

#ifndef s_addr
/*
 * Internet address (old style... should be updated)
 */
struct in_addr {
        union {
                struct { u_char s_b1,s_b2,s_b3,s_b4; } S_un_b;
                struct { u_short s_w1,s_w2; } S_un_w;
                u_long S_addr;
        } S_un;
#define s_addr  S_un.S_addr
                                /* can be used for most tcp & ip code */
#define s_host  S_un.S_un_b.s_b2
                                /* host on imp */
#define s_net   S_un.S_un_b.s_b1
                                /* network */
#define s_imp   S_un.S_un_w.s_w2
                                /* imp */
#define s_impno S_un.S_un_b.s_b4
                                /* imp # */
#define s_lh    S_un.S_un_b.s_b3
                                /* logical host */
};
#endif

#define ADDR_ANY                INADDR_ANY

#define WSADESCRIPTION_LEN      256
#define WSASYS_STATUS_LEN       128

typedef struct WSAData {
        WORD                    wVersion;
        WORD                    wHighVersion;
#ifdef _WIN64
        unsigned short          iMaxSockets;
        unsigned short          iMaxUdpDg;
        char FAR *              lpVendorInfo;
        char                    szDescription[WSADESCRIPTION_LEN+1];
        char                    szSystemStatus[WSASYS_STATUS_LEN+1];
#else
        char                    szDescription[WSADESCRIPTION_LEN+1];
        char                    szSystemStatus[WSASYS_STATUS_LEN+1];
        unsigned short          iMaxSockets;
        unsigned short          iMaxUdpDg;
        char FAR *              lpVendorInfo;
#endif
} WSADATA, FAR * LPWSADATA;

/*
 * Definitions related to sockets: types, address families, options,
 * taken from the BSD file sys/socket.h.
 */

/*
 * This is used instead of -1, since the
 * SOCKET type is unsigned.
 */
#define INVALID_SOCKET  (SOCKET)(~0)
#define SOCKET_ERROR            (-1)

/*
 * The  following  may  be used in place of the address family, socket type, or
 * protocol  in  a  call  to WSASocket to indicate that the corresponding value
 * should  be taken from the supplied WSAPROTOCOL_INFO structure instead of the
 * parameter itself.
 */
#define FROM_PROTOCOL_INFO (-1)

/*
 * Types
 */
#define SOCK_STREAM     1               /* stream socket */
#define SOCK_DGRAM      2               /* datagram socket */
#define SOCK_RAW        3               /* raw-protocol interface */
#define SOCK_RDM        4               /* reliably-delivered message */
#define SOCK_SEQPACKET  5               /* sequenced packet stream */

/*
 * Option flags per-socket.
 */
#define SO_DEBUG        0x0001          /* turn on debugging info recording */
#define SO_ACCEPTCONN   0x0002          /* socket has had listen() */
#define SO_REUSEADDR    0x0004          /* allow local address reuse */
#define SO_KEEPALIVE    0x0008          /* keep connections alive */
#define SO_DONTROUTE    0x0010          /* just use interface addresses */
#define SO_BROADCAST    0x0020          /* permit sending of broadcast msgs */
#define SO_USELOOPBACK  0x0040          /* bypass hardware when possible */
#define SO_LINGER       0x0080          /* linger on close if data present */
#define SO_OOBINLINE    0x0100          /* leave received OOB data in line */

#define SO_DONTLINGER   (int)(~SO_LINGER)
#define SO_EXCLUSIVEADDRUSE ((int)(~SO_REUSEADDR)) /* disallow local address reuse */

/*
 * Additional options.
 */
#define SO_SNDBUF       0x1001          /* send buffer size */
#define SO_RCVBUF       0x1002          /* receive buffer size */
#define SO_SNDLOWAT     0x1003          /* send low-water mark */
#define SO_RCVLOWAT     0x1004          /* receive low-water mark */
#define SO_SNDTIMEO     0x1005          /* send timeout */
#define SO_RCVTIMEO     0x1006          /* receive timeout */
#define SO_ERROR        0x1007          /* get error status and clear */
#define SO_TYPE         0x1008          /* get socket type */

/*
 * WinSock 2 extension -- new options
 */
#define SO_GROUP_ID       0x2001      /* ID of a socket group */
#define SO_GROUP_PRIORITY 0x2002      /* the relative priority within a group*/
#define SO_MAX_MSG_SIZE   0x2003      /* maximum message size */
#define SO_PROTOCOL_INFOA 0x2004      /* WSAPROTOCOL_INFOA structure */
#define SO_PROTOCOL_INFOW 0x2005      /* WSAPROTOCOL_INFOW structure */
#ifdef UNICODE
#define SO_PROTOCOL_INFO  SO_PROTOCOL_INFOW
#else
#define SO_PROTOCOL_INFO  SO_PROTOCOL_INFOA
#endif /* UNICODE */
#define PVD_CONFIG        0x3001       /* configuration info for service provider */
#define SO_CONDITIONAL_ACCEPT 0x3002   /* enable true conditional accept: */
                                       /*  connection is not ack-ed to the */
                                       /*  other side until conditional */
                                       /*  function returns CF_ACCEPT */

/*
 * Structure used by kernel to pass protocol
 * information in raw sockets.
 */
struct sockproto {
        u_short sp_family;              /* address family */
        u_short sp_protocol;            /* protocol */
};

/*
 * Protocol families, same as address families for now.
 */
#define PF_UNSPEC       AF_UNSPEC
#define PF_UNIX         AF_UNIX
#define PF_INET         AF_INET
#define PF_IMPLINK      AF_IMPLINK
#define PF_PUP          AF_PUP
#define PF_CHAOS        AF_CHAOS
#define PF_NS           AF_NS
#define PF_IPX          AF_IPX
#define PF_ISO          AF_ISO
#define PF_OSI          AF_OSI
#define PF_ECMA         AF_ECMA
#define PF_DATAKIT      AF_DATAKIT
#define PF_CCITT        AF_CCITT
#define PF_SNA          AF_SNA
#define PF_DECnet       AF_DECnet
#define PF_DLI          AF_DLI
#define PF_LAT          AF_LAT
#define PF_HYLINK       AF_HYLINK
#define PF_APPLETALK    AF_APPLETALK
#define PF_VOICEVIEW    AF_VOICEVIEW
#define PF_FIREFOX      AF_FIREFOX
#define PF_UNKNOWN1     AF_UNKNOWN1
#define PF_BAN          AF_BAN
#define PF_ATM          AF_ATM
#define PF_INET6        AF_INET6
#if(_WIN32_WINNT >= 0x0600)
#define PF_BTH          AF_BTH
#endif //(_WIN32_WINNT >= 0x0600)

#define PF_MAX          AF_MAX

/*
 * Structure used for manipulating linger option.
 */
struct  linger {
        u_short l_onoff;                /* option on/off */
        u_short l_linger;               /* linger time */
};

/*
 * Level number for (get/set)sockopt() to apply to socket itself.
 */
#define SOL_SOCKET      0xffff          /* options for socket level */

/*
 * Maximum queue length specifiable by listen.
 */
#define SOMAXCONN       0x7fffffff
#define SOMAXCONN_HINT(b) (-(b))

#define MSG_OOB         0x1             /* process out-of-band data */
#define MSG_PEEK        0x2             /* peek at incoming message */
#define MSG_DONTROUTE   0x4             /* send without using routing tables */

#if(_WIN32_WINNT >= 0x0502)
#define MSG_WAITALL     0x8             /* do not complete until packet is completely filled */
#endif //(_WIN32_WINNT >= 0x0502)

#if(_WIN32_WINNT >= 0x0603)
#define MSG_PUSH_IMMEDIATE 0x20         /* Do not delay receive request completion if data is available */
#endif //(_WIN32_WINNT >= 0x0603)

#define MSG_PARTIAL     0x8000          /* partial send or recv for message xport */

/*
 * WinSock 2 extension -- new flags for WSASend(), WSASendTo(), WSARecv() and
 *                          WSARecvFrom()
 */
#define MSG_INTERRUPT   0x10            /* send/recv in the interrupt context */

#define MSG_MAXIOVLEN   16

/*
 * Define constant based on rfc883, used by gethostbyxxxx() calls.
 */
#define MAXGETHOSTSTRUCT        1024

/*
 * WinSock 2 extension -- bit values and indices for FD_XXX network events
 */
#define FD_READ_BIT      0
#define FD_READ          (1 << FD_READ_BIT)

#define FD_WRITE_BIT     1
#define FD_WRITE         (1 << FD_WRITE_BIT)

#define FD_OOB_BIT       2
#define FD_OOB           (1 << FD_OOB_BIT)

#define FD_ACCEPT_BIT    3
#define FD_ACCEPT        (1 << FD_ACCEPT_BIT)

#define FD_CONNECT_BIT   4
#define FD_CONNECT       (1 << FD_CONNECT_BIT)

#define FD_CLOSE_BIT     5
#define FD_CLOSE         (1 << FD_CLOSE_BIT)

#define FD_QOS_BIT       6
#define FD_QOS           (1 << FD_QOS_BIT)

#define FD_GROUP_QOS_BIT 7
#define FD_GROUP_QOS     (1 << FD_GROUP_QOS_BIT)

#define FD_ROUTING_INTERFACE_CHANGE_BIT 8
#define FD_ROUTING_INTERFACE_CHANGE     (1 << FD_ROUTING_INTERFACE_CHANGE_BIT)

#define FD_ADDRESS_LIST_CHANGE_BIT 9
#define FD_ADDRESS_LIST_CHANGE     (1 << FD_ADDRESS_LIST_CHANGE_BIT)

#define FD_MAX_EVENTS    10
#define FD_ALL_EVENTS    ((1 << FD_MAX_EVENTS) - 1)


/*
 * WinSock error codes are also defined in winerror.h
 * Hence the IFDEF.
 */
#ifndef WSABASEERR

/*
 * All Windows Sockets error constants are biased by WSABASEERR from
 * the "normal"
 */
#define WSABASEERR              10000

/*
 * Windows Sockets definitions of regular Microsoft C error constants
 */
#define WSAEINTR                (WSABASEERR+4)
#define WSAEBADF                (WSABASEERR+9)
#define WSAEACCES               (WSABASEERR+13)
#define WSAEFAULT               (WSABASEERR+14)
#define WSAEINVAL               (WSABASEERR+22)
#define WSAEMFILE               (WSABASEERR+24)

/*
 * Windows Sockets definitions of regular Berkeley error constants
 */
#define WSAEWOULDBLOCK          (WSABASEERR+35)
#define WSAEINPROGRESS          (WSABASEERR+36)
#define WSAEALREADY             (WSABASEERR+37)
#define WSAENOTSOCK             (WSABASEERR+38)
#define WSAEDESTADDRREQ         (WSABASEERR+39)
#define WSAEMSGSIZE             (WSABASEERR+40)
#define WSAEPROTOTYPE           (WSABASEERR+41)
#define WSAENOPROTOOPT          (WSABASEERR+42)
#define WSAEPROTONOSUPPORT      (WSABASEERR+43)
#define WSAESOCKTNOSUPPORT      (WSABASEERR+44)
#define WSAEOPNOTSUPP           (WSABASEERR+45)
#define WSAEPFNOSUPPORT         (WSABASEERR+46)
#define WSAEAFNOSUPPORT         (WSABASEERR+47)
#define WSAEADDRINUSE           (WSABASEERR+48)
#define WSAEADDRNOTAVAIL        (WSABASEERR+49)
#define WSAENETDOWN             (WSABASEERR+50)
#define WSAENETUNREACH          (WSABASEERR+51)
#define WSAENETRESET            (WSABASEERR+52)
#define WSAECONNABORTED         (WSABASEERR+53)
#define WSAECONNRESET           (WSABASEERR+54)
#define WSAENOBUFS              (WSABASEERR+55)
#define WSAEISCONN              (WSABASEERR+56)
#define WSAENOTCONN             (WSABASEERR+57)
#define WSAESHUTDOWN            (WSABASEERR+58)
#define WSAETOOMANYREFS         (WSABASEERR+59)
#define WSAETIMEDOUT            (WSABASEERR+60)
#define WSAECONNREFUSED         (WSABASEERR+61)
#define WSAELOOP                (WSABASEERR+62)
#define WSAENAMETOOLONG         (WSABASEERR+63)
#define WSAEHOSTDOWN            (WSABASEERR+64)
#define WSAEHOSTUNREACH         (WSABASEERR+65)
#define WSAENOTEMPTY            (WSABASEERR+66)
#define WSAEPROCLIM             (WSABASEERR+67)
#define WSAEUSERS               (WSABASEERR+68)
#define WSAEDQUOT               (WSABASEERR+69)
#define WSAESTALE               (WSABASEERR+70)
#define WSAEREMOTE              (WSABASEERR+71)

/*
 * Extended Windows Sockets error constant definitions
 */
#define WSASYSNOTREADY          (WSABASEERR+91)
#define WSAVERNOTSUPPORTED      (WSABASEERR+92)
#define WSANOTINITIALISED       (WSABASEERR+93)
#define WSAEDISCON              (WSABASEERR+101)
#define WSAENOMORE              (WSABASEERR+102)
#define WSAECANCELLED           (WSABASEERR+103)
#define WSAEINVALIDPROCTABLE    (WSABASEERR+104)
#define WSAEINVALIDPROVIDER     (WSABASEERR+105)
#define WSAEPROVIDERFAILEDINIT  (WSABASEERR+106)
#define WSASYSCALLFAILURE       (WSABASEERR+107)
#define WSASERVICE_NOT_FOUND    (WSABASEERR+108)
#define WSATYPE_NOT_FOUND       (WSABASEERR+109)
#define WSA_E_NO_MORE           (WSABASEERR+110)
#define WSA_E_CANCELLED         (WSABASEERR+111)
#define WSAEREFUSED             (WSABASEERR+112)

/*
 * Error return codes from gethostbyname() and gethostbyaddr()
 * (when using the resolver). Note that these errors are
 * retrieved via WSAGetLastError() and must therefore follow
 * the rules for avoiding clashes with error numbers from
 * specific implementations or language run-time systems.
 * For this reason the codes are based at WSABASEERR+1001.
 * Note also that [WSA]NO_ADDRESS is defined only for
 * compatibility purposes.
 */

/* Authoritative Answer: Host not found */
#define WSAHOST_NOT_FOUND       (WSABASEERR+1001)

/* Non-Authoritative: Host not found, or SERVERFAIL */
#define WSATRY_AGAIN            (WSABASEERR+1002)

/* Non-recoverable errors, FORMERR, REFUSED, NOTIMP */
#define WSANO_RECOVERY          (WSABASEERR+1003)

/* Valid name, no data record of requested type */
#define WSANO_DATA              (WSABASEERR+1004)

/*
 * Define QOS related error return codes
 *
 */
#define  WSA_QOS_RECEIVERS               (WSABASEERR + 1005)
         /* at least one Reserve has arrived */
#define  WSA_QOS_SENDERS                 (WSABASEERR + 1006)
         /* at least one Path has arrived */
#define  WSA_QOS_NO_SENDERS              (WSABASEERR + 1007)
         /* there are no senders */
#define  WSA_QOS_NO_RECEIVERS            (WSABASEERR + 1008)
         /* there are no receivers */
#define  WSA_QOS_REQUEST_CONFIRMED       (WSABASEERR + 1009)
         /* Reserve has been confirmed */
#define  WSA_QOS_ADMISSION_FAILURE       (WSABASEERR + 1010)
         /* error due to lack of resources */
#define  WSA_QOS_POLICY_FAILURE          (WSABASEERR + 1011)
         /* rejected for administrative reasons - bad credentials */
#define  WSA_QOS_BAD_STYLE               (WSABASEERR + 1012)
         /* unknown or conflicting style */
#define  WSA_QOS_BAD_OBJECT              (WSABASEERR + 1013)
         /* problem with some part of the filterspec or providerspecific
          * buffer in general */
#define  WSA_QOS_TRAFFIC_CTRL_ERROR      (WSABASEERR + 1014)
         /* problem with some part of the flowspec */
#define  WSA_QOS_GENERIC_ERROR           (WSABASEERR + 1015)
         /* general error */
#define  WSA_QOS_ESERVICETYPE            (WSABASEERR + 1016)
         /* invalid service type in flowspec */
#define  WSA_QOS_EFLOWSPEC               (WSABASEERR + 1017)
         /* invalid flowspec */
#define  WSA_QOS_EPROVSPECBUF            (WSABASEERR + 1018)
         /* invalid provider specific buffer */
#define  WSA_QOS_EFILTERSTYLE            (WSABASEERR + 1019)
         /* invalid filter style */
#define  WSA_QOS_EFILTERTYPE             (WSABASEERR + 1020)
         /* invalid filter type */
#define  WSA_QOS_EFILTERCOUNT            (WSABASEERR + 1021)
         /* incorrect number of filters */
#define  WSA_QOS_EOBJLENGTH              (WSABASEERR + 1022)
         /* invalid object length */
#define  WSA_QOS_EFLOWCOUNT              (WSABASEERR + 1023)
         /* incorrect number of flows */
#define  WSA_QOS_EUNKOWNPSOBJ            (WSABASEERR + 1024)
         /* unknown object in provider specific buffer */
#define  WSA_QOS_EPOLICYOBJ              (WSABASEERR + 1025)
         /* invalid policy object in provider specific buffer */
#define  WSA_QOS_EFLOWDESC               (WSABASEERR + 1026)
         /* invalid flow descriptor in the list */
#define  WSA_QOS_EPSFLOWSPEC             (WSABASEERR + 1027)
         /* inconsistent flow spec in provider specific buffer */
#define  WSA_QOS_EPSFILTERSPEC           (WSABASEERR + 1028)
         /* invalid filter spec in provider specific buffer */
#define  WSA_QOS_ESDMODEOBJ              (WSABASEERR + 1029)
         /* invalid shape discard mode object in provider specific buffer */
#define  WSA_QOS_ESHAPERATEOBJ           (WSABASEERR + 1030)
         /* invalid shaping rate object in provider specific buffer */
#define  WSA_QOS_RESERVED_PETYPE         (WSABASEERR + 1031)
         /* reserved policy element in provider specific buffer */



/*
 * WinSock error codes are also defined in winerror.h
 * Hence the IFDEF.
 */
#endif /* ifdef WSABASEERR */

/*
 * Compatibility macros.
 */

#define h_errno         WSAGetLastError()
#define HOST_NOT_FOUND          WSAHOST_NOT_FOUND
#define TRY_AGAIN               WSATRY_AGAIN
#define NO_RECOVERY             WSANO_RECOVERY
#define NO_DATA                 WSANO_DATA
/* no address, look for MX record */
#define WSANO_ADDRESS           WSANO_DATA
#define NO_ADDRESS              WSANO_ADDRESS



/*
 * Windows Sockets errors redefined as regular Berkeley error constants.
 * These are commented out in Windows NT to avoid conflicts with errno.h.
 * Use the WSA constants instead.
 */
#if 0
#define EWOULDBLOCK             WSAEWOULDBLOCK
#define EINPROGRESS             WSAEINPROGRESS
#define EALREADY                WSAEALREADY
#define ENOTSOCK                WSAENOTSOCK
#define EDESTADDRREQ            WSAEDESTADDRREQ
#define EMSGSIZE                WSAEMSGSIZE
#define EPROTOTYPE              WSAEPROTOTYPE
#define ENOPROTOOPT             WSAENOPROTOOPT
#define EPROTONOSUPPORT         WSAEPROTONOSUPPORT
#define ESOCKTNOSUPPORT         WSAESOCKTNOSUPPORT
#define EOPNOTSUPP              WSAEOPNOTSUPP
#define EPFNOSUPPORT            WSAEPFNOSUPPORT
#define EAFNOSUPPORT            WSAEAFNOSUPPORT
#define EADDRINUSE              WSAEADDRINUSE
#define EADDRNOTAVAIL           WSAEADDRNOTAVAIL
#define ENETDOWN                WSAENETDOWN
#define ENETUNREACH             WSAENETUNREACH
#define ENETRESET               WSAENETRESET
#define ECONNABORTED            WSAECONNABORTED
#define ECONNRESET              WSAECONNRESET
#define ENOBUFS                 WSAENOBUFS
#define EISCONN                 WSAEISCONN
#define ENOTCONN                WSAENOTCONN
#define ESHUTDOWN               WSAESHUTDOWN
#define ETOOMANYREFS            WSAETOOMANYREFS
#define ETIMEDOUT               WSAETIMEDOUT
#define ECONNREFUSED            WSAECONNREFUSED
#define ELOOP                   WSAELOOP
#define ENAMETOOLONG            WSAENAMETOOLONG
#define EHOSTDOWN               WSAEHOSTDOWN
#define EHOSTUNREACH            WSAEHOSTUNREACH
#define ENOTEMPTY               WSAENOTEMPTY
#define EPROCLIM                WSAEPROCLIM
#define EUSERS                  WSAEUSERS
#define EDQUOT                  WSAEDQUOT
#define ESTALE                  WSAESTALE
#define EREMOTE                 WSAEREMOTE
#endif

/*
 * WinSock 2 extension -- new error codes and type definition
 */

#ifdef WIN32

#define WSAAPI                  FAR PASCAL
#define WSAEVENT                HANDLE
#define LPWSAEVENT              LPHANDLE
#define WSAOVERLAPPED           OVERLAPPED
typedef struct _OVERLAPPED *    LPWSAOVERLAPPED;

#define WSA_IO_PENDING          (ERROR_IO_PENDING)
#define WSA_IO_INCOMPLETE       (ERROR_IO_INCOMPLETE)
#define WSA_INVALID_HANDLE      (ERROR_INVALID_HANDLE)
#define WSA_INVALID_PARAMETER   (ERROR_INVALID_PARAMETER)
#define WSA_NOT_ENOUGH_MEMORY   (ERROR_NOT_ENOUGH_MEMORY)
#define WSA_OPERATION_ABORTED   (ERROR_OPERATION_ABORTED)

#define WSA_INVALID_EVENT       ((WSAEVENT)NULL)
#define WSA_MAXIMUM_WAIT_EVENTS (MAXIMUM_WAIT_OBJECTS)
#define WSA_WAIT_FAILED         (WAIT_FAILED)
#define WSA_WAIT_EVENT_0        (WAIT_OBJECT_0)
#define WSA_WAIT_IO_COMPLETION  (WAIT_IO_COMPLETION)
#define WSA_WAIT_TIMEOUT        (WAIT_TIMEOUT)
#define WSA_INFINITE            (INFINITE)

#else /* WIN16 */

#define WSAAPI                  FAR PASCAL
typedef DWORD                   WSAEVENT, FAR * LPWSAEVENT;

typedef struct _WSAOVERLAPPED {
    DWORD    Internal;
    DWORD    InternalHigh;
    DWORD    Offset;
    DWORD    OffsetHigh;
    WSAEVENT hEvent;
} WSAOVERLAPPED, FAR * LPWSAOVERLAPPED;

#define WSA_IO_PENDING          (WSAEWOULDBLOCK)
#define WSA_IO_INCOMPLETE       (WSAEWOULDBLOCK)
#define WSA_INVALID_HANDLE      (WSAENOTSOCK)
#define WSA_INVALID_PARAMETER   (WSAEINVAL)
#define WSA_NOT_ENOUGH_MEMORY   (WSAENOBUFS)
#define WSA_OPERATION_ABORTED   (WSAEINTR)

#define WSA_INVALID_EVENT       ((WSAEVENT)NULL)
#define WSA_MAXIMUM_WAIT_EVENTS (MAXIMUM_WAIT_OBJECTS)
#define WSA_WAIT_FAILED         ((DWORD)-1L)
#define WSA_WAIT_EVENT_0        ((DWORD)0)
#define WSA_WAIT_TIMEOUT        ((DWORD)0x102L)
#define WSA_INFINITE            ((DWORD)-1L)

#endif  /* WIN32 */

#define LPQOS LPVOID

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#undef LPQOS

/*
 * Include qos.h to pull in FLOWSPEC and related definitions
 */
#include <qos.h>

typedef struct _QualityOfService
{
    FLOWSPEC      SendingFlowspec;       /* the flow spec for data sending */
    FLOWSPEC      ReceivingFlowspec;     /* the flow spec for data receiving */
    WSABUF        ProviderSpecific;      /* additional provider specific stuff */
} QOS, FAR * LPQOS;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

/*
 * WinSock 2 extension -- manifest constants for return values of the condition function
 */
#define CF_ACCEPT       0x0000
#define CF_REJECT       0x0001
#define CF_DEFER        0x0002

/*
 * WinSock 2 extension -- manifest constants for shutdown()
 */
#define SD_RECEIVE      0x00
#define SD_SEND         0x01
#define SD_BOTH         0x02

/*
 * WinSock 2 extension -- data type and manifest constants for socket groups
 */
typedef unsigned int             GROUP;

#define SG_UNCONSTRAINED_GROUP   0x01
#define SG_CONSTRAINED_GROUP     0x02

/*
 * WinSock 2 extension -- data type for WSAEnumNetworkEvents()
 */
typedef struct _WSANETWORKEVENTS {
       long lNetworkEvents;
       int iErrorCode[FD_MAX_EVENTS];
} WSANETWORKEVENTS, FAR * LPWSANETWORKEVENTS;

/*
 * WinSock 2 extension -- WSAPROTOCOL_INFO structure and associated
 * manifest constants
 */

#ifndef GUID_DEFINED
#include <guiddef.h>
#endif /* GUID_DEFINED */

#define MAX_PROTOCOL_CHAIN 7

#define BASE_PROTOCOL      1
#define LAYERED_PROTOCOL   0

typedef struct _WSAPROTOCOLCHAIN {
    int ChainLen;                                 /* the length of the chain,     */
                                                  /* length = 0 means layered protocol, */
                                                  /* length = 1 means base protocol, */
                                                  /* length > 1 means protocol chain */
    DWORD ChainEntries[MAX_PROTOCOL_CHAIN];       /* a list of dwCatalogEntryIds */
} WSAPROTOCOLCHAIN, FAR * LPWSAPROTOCOLCHAIN;

#define WSAPROTOCOL_LEN  255

typedef struct _WINSOCK_DEPRECATED_BY("WSAPROTOCOL_INFOW") _WSAPROTOCOL_INFOA {
    DWORD dwServiceFlags1;
    DWORD dwServiceFlags2;
    DWORD dwServiceFlags3;
    DWORD dwServiceFlags4;
    DWORD dwProviderFlags;
    GUID ProviderId;
    DWORD dwCatalogEntryId;
    WSAPROTOCOLCHAIN ProtocolChain;
    int iVersion;
    int iAddressFamily;
    int iMaxSockAddr;
    int iMinSockAddr;
    int iSocketType;
    int iProtocol;
    int iProtocolMaxOffset;
    int iNetworkByteOrder;
    int iSecurityScheme;
    DWORD dwMessageSize;
    DWORD dwProviderReserved;
    CHAR   szProtocol[WSAPROTOCOL_LEN+1];
} WSAPROTOCOL_INFOA, FAR * LPWSAPROTOCOL_INFOA;

typedef struct _WSAPROTOCOL_INFOW {
    DWORD dwServiceFlags1;
    DWORD dwServiceFlags2;
    DWORD dwServiceFlags3;
    DWORD dwServiceFlags4;
    DWORD dwProviderFlags;
    GUID ProviderId;
    DWORD dwCatalogEntryId;
    WSAPROTOCOLCHAIN ProtocolChain;
    int iVersion;
    int iAddressFamily;
    int iMaxSockAddr;
    int iMinSockAddr;
    int iSocketType;
    int iProtocol;
    int iProtocolMaxOffset;
    int iNetworkByteOrder;
    int iSecurityScheme;
    DWORD dwMessageSize;
    DWORD dwProviderReserved;
    WCHAR  szProtocol[WSAPROTOCOL_LEN+1];
} WSAPROTOCOL_INFOW, FAR * LPWSAPROTOCOL_INFOW;
#ifdef UNICODE
typedef WSAPROTOCOL_INFOW WSAPROTOCOL_INFO;
typedef LPWSAPROTOCOL_INFOW LPWSAPROTOCOL_INFO;
#else
typedef WSAPROTOCOL_INFOA WSAPROTOCOL_INFO;
typedef LPWSAPROTOCOL_INFOA LPWSAPROTOCOL_INFO;
#endif /* UNICODE */

/* Flag bit definitions for dwProviderFlags */
#define PFL_MULTIPLE_PROTO_ENTRIES          0x00000001
#define PFL_RECOMMENDED_PROTO_ENTRY         0x00000002
#define PFL_HIDDEN                          0x00000004
#define PFL_MATCHES_PROTOCOL_ZERO           0x00000008
#define PFL_NETWORKDIRECT_PROVIDER          0x00000010

/* Flag bit definitions for dwServiceFlags1 */
#define XP1_CONNECTIONLESS                  0x00000001
#define XP1_GUARANTEED_DELIVERY             0x00000002
#define XP1_GUARANTEED_ORDER                0x00000004
#define XP1_MESSAGE_ORIENTED                0x00000008
#define XP1_PSEUDO_STREAM                   0x00000010
#define XP1_GRACEFUL_CLOSE                  0x00000020
#define XP1_EXPEDITED_DATA                  0x00000040
#define XP1_CONNECT_DATA                    0x00000080
#define XP1_DISCONNECT_DATA                 0x00000100
#define XP1_SUPPORT_BROADCAST               0x00000200
#define XP1_SUPPORT_MULTIPOINT              0x00000400
#define XP1_MULTIPOINT_CONTROL_PLANE        0x00000800
#define XP1_MULTIPOINT_DATA_PLANE           0x00001000
#define XP1_QOS_SUPPORTED                   0x00002000
#define XP1_INTERRUPT                       0x00004000
#define XP1_UNI_SEND                        0x00008000
#define XP1_UNI_RECV                        0x00010000
#define XP1_IFS_HANDLES                     0x00020000
#define XP1_PARTIAL_MESSAGE                 0x00040000
#define XP1_SAN_SUPPORT_SDP                 0x00080000

#define BIGENDIAN                           0x0000
#define LITTLEENDIAN                        0x0001

#define SECURITY_PROTOCOL_NONE              0x0000

/*
 * WinSock 2 extension -- manifest constants for WSAJoinLeaf()
 */
#define JL_SENDER_ONLY    0x01
#define JL_RECEIVER_ONLY  0x02
#define JL_BOTH           0x04

/*
 * WinSock 2 extension -- manifest constants for WSASocket()
 */
#define WSA_FLAG_OVERLAPPED           0x01
#define WSA_FLAG_MULTIPOINT_C_ROOT    0x02
#define WSA_FLAG_MULTIPOINT_C_LEAF    0x04
#define WSA_FLAG_MULTIPOINT_D_ROOT    0x08
#define WSA_FLAG_MULTIPOINT_D_LEAF    0x10
#define WSA_FLAG_ACCESS_SYSTEM_SECURITY 0x40
#define WSA_FLAG_NO_HANDLE_INHERIT    0x80
#define WSA_FLAG_REGISTERED_IO       0x100

/*
 * WinSock 2 extensions -- data types for the condition function in
 * WSAAccept() and overlapped I/O completion routine.
 */

typedef
int
(CALLBACK * LPCONDITIONPROC)(
    IN LPWSABUF lpCallerId,
    IN LPWSABUF lpCallerData,
    IN OUT LPQOS lpSQOS,
    IN OUT LPQOS lpGQOS,
    IN LPWSABUF lpCalleeId,
    IN LPWSABUF lpCalleeData,
    OUT GROUP FAR * g,
    IN DWORD_PTR dwCallbackData
    );

typedef
void
(CALLBACK * LPWSAOVERLAPPED_COMPLETION_ROUTINE)(
    IN DWORD dwError,
    IN DWORD cbTransferred,
    IN LPWSAOVERLAPPED lpOverlapped,
    IN DWORD dwFlags
    );

#if(_WIN32_WINNT >= 0x0501)

/*
 * WinSock 2 extension -- manifest constants and associated structures
 * for WSANSPIoctl()
 */
#define SIO_NSP_NOTIFY_CHANGE         _WSAIOW(IOC_WS2,25)

typedef enum _WSACOMPLETIONTYPE {
    NSP_NOTIFY_IMMEDIATELY = 0,
    NSP_NOTIFY_HWND,
    NSP_NOTIFY_EVENT,
    NSP_NOTIFY_PORT,
    NSP_NOTIFY_APC,
} WSACOMPLETIONTYPE, *PWSACOMPLETIONTYPE, FAR * LPWSACOMPLETIONTYPE;

typedef struct _WSACOMPLETION {
    WSACOMPLETIONTYPE Type;
    union {
        struct {
            HWND hWnd;
            UINT uMsg;
            WPARAM context;
        } WindowMessage;
        struct {
            LPWSAOVERLAPPED lpOverlapped;
        } Event;
        struct {
            LPWSAOVERLAPPED lpOverlapped;
            LPWSAOVERLAPPED_COMPLETION_ROUTINE lpfnCompletionProc;
        } Apc;
        struct {
            LPWSAOVERLAPPED lpOverlapped;
            HANDLE hPort;
            ULONG_PTR Key;
        } Port;
    } Parameters;
} WSACOMPLETION, *PWSACOMPLETION, FAR *LPWSACOMPLETION;
#endif //(_WIN32_WINNT >= 0x0501)

/*
 * WinSock 2 extension -- manifest constants for SIO_TRANSLATE_HANDLE ioctl
 */
#define TH_NETDEV        0x00000001
#define TH_TAPI          0x00000002

/*
 * Manifest constants and type definitions related to name resolution and
 * registration (RNR) API
 */

#ifndef _tagBLOB_DEFINED
#define _tagBLOB_DEFINED
#define _BLOB_DEFINED
#define _LPBLOB_DEFINED
typedef struct _BLOB {
    ULONG cbSize ;
#ifdef MIDL_PASS
    [size_is(cbSize)] BYTE *pBlobData;
#else  /* MIDL_PASS */
    _Field_size_bytes_(cbSize) BYTE *pBlobData ;
#endif /* MIDL_PASS */
} BLOB, *LPBLOB ;
#endif

/*
 * Service Install Flags
 */

#define SERVICE_MULTIPLE       (0x00000001)

/*
 *& Name Spaces
 */

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

/*
 *& Name Spaces
 */

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

#define NS_LOCALNAME                (19)    /* Windows Live */

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

/*
 * Resolution flags for WSAGetAddressByName().
 * Note these are also used by the 1.1 API GetAddressByName, so
 * leave them around.
 */
#define RES_UNUSED_1                (0x00000001)
#define RES_FLUSH_CACHE             (0x00000002)
#ifndef RES_SERVICE
#define RES_SERVICE                 (0x00000004)
#endif /* RES_SERVICE */

/*
 * Well known value names for Service Types
 */

#define SERVICE_TYPE_VALUE_IPXPORTA      "IpxSocket"
#define SERVICE_TYPE_VALUE_IPXPORTW     L"IpxSocket"
#define SERVICE_TYPE_VALUE_SAPIDA        "SapId"
#define SERVICE_TYPE_VALUE_SAPIDW       L"SapId"

#define SERVICE_TYPE_VALUE_TCPPORTA      "TcpPort"
#define SERVICE_TYPE_VALUE_TCPPORTW     L"TcpPort"

#define SERVICE_TYPE_VALUE_UDPPORTA      "UdpPort"
#define SERVICE_TYPE_VALUE_UDPPORTW     L"UdpPort"

#define SERVICE_TYPE_VALUE_OBJECTIDA     "ObjectId"
#define SERVICE_TYPE_VALUE_OBJECTIDW    L"ObjectId"

#ifdef UNICODE

#define SERVICE_TYPE_VALUE_SAPID        SERVICE_TYPE_VALUE_SAPIDW
#define SERVICE_TYPE_VALUE_TCPPORT      SERVICE_TYPE_VALUE_TCPPORTW
#define SERVICE_TYPE_VALUE_UDPPORT      SERVICE_TYPE_VALUE_UDPPORTW
#define SERVICE_TYPE_VALUE_OBJECTID     SERVICE_TYPE_VALUE_OBJECTIDW

#else /* not UNICODE */

#define SERVICE_TYPE_VALUE_SAPID        SERVICE_TYPE_VALUE_SAPIDA
#define SERVICE_TYPE_VALUE_TCPPORT      SERVICE_TYPE_VALUE_TCPPORTA
#define SERVICE_TYPE_VALUE_UDPPORT      SERVICE_TYPE_VALUE_UDPPORTA
#define SERVICE_TYPE_VALUE_OBJECTID     SERVICE_TYPE_VALUE_OBJECTIDA

#endif

/*
 *  Address Family/Protocol Tuples
 */
typedef struct _AFPROTOCOLS {
    INT iAddressFamily;
    INT iProtocol;
} AFPROTOCOLS, *PAFPROTOCOLS, *LPAFPROTOCOLS;

/*
 * Client Query API Typedefs
 */

/*
 * The comparators
 */
typedef enum _WSAEcomparator
{
    COMP_EQUAL = 0,
    COMP_NOTLESS
} WSAECOMPARATOR, *PWSAECOMPARATOR, *LPWSAECOMPARATOR;

typedef struct _WSAVersion
{
    DWORD           dwVersion;
    WSAECOMPARATOR  ecHow;
}WSAVERSION, *PWSAVERSION, *LPWSAVERSION;

typedef struct _WINSOCK_DEPRECATED_BY("WSAQUERYSETW") _WSAQuerySetA
{
    DWORD           dwSize;
    LPSTR           lpszServiceInstanceName;
    LPGUID          lpServiceClassId;
    LPWSAVERSION    lpVersion;
    LPSTR           lpszComment;
    DWORD           dwNameSpace;
    LPGUID          lpNSProviderId;
    LPSTR           lpszContext;
    DWORD           dwNumberOfProtocols;
    _Field_size_(dwNumberOfProtocols) LPAFPROTOCOLS   lpafpProtocols;
    LPSTR           lpszQueryString;
    DWORD           dwNumberOfCsAddrs;
    _Field_size_(dwNumberOfCsAddrs) LPCSADDR_INFO   lpcsaBuffer;
    DWORD           dwOutputFlags;
    LPBLOB          lpBlob;
} WSAQUERYSETA, *PWSAQUERYSETA, *LPWSAQUERYSETA;

typedef _Struct_size_bytes_(dwSize) struct _WSAQuerySetW
{
    _Field_range_(>=,sizeof(struct _WSAQuerySetW)) DWORD           dwSize;
    LPWSTR          lpszServiceInstanceName;
    LPGUID          lpServiceClassId;
    LPWSAVERSION    lpVersion;
    LPWSTR          lpszComment;
    DWORD           dwNameSpace;
    LPGUID          lpNSProviderId;
    LPWSTR          lpszContext;
    DWORD           dwNumberOfProtocols;
    _Field_size_(dwNumberOfProtocols) LPAFPROTOCOLS   lpafpProtocols;
    LPWSTR          lpszQueryString;
    DWORD           dwNumberOfCsAddrs;
    _Field_size_(dwNumberOfCsAddrs) LPCSADDR_INFO   lpcsaBuffer;
    DWORD           dwOutputFlags;
    LPBLOB          lpBlob;
} WSAQUERYSETW, *PWSAQUERYSETW, *LPWSAQUERYSETW;

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

typedef struct _WINSOCK_DEPRECATED_BY("WSAQUERYSET2W") _WSAQuerySet2A
{
    DWORD           dwSize;
    LPSTR           lpszServiceInstanceName;
    LPWSAVERSION    lpVersion;
    LPSTR           lpszComment;
    DWORD           dwNameSpace;
    LPGUID          lpNSProviderId;
    LPSTR           lpszContext;
    DWORD           dwNumberOfProtocols;
    LPAFPROTOCOLS   lpafpProtocols;
    LPSTR           lpszQueryString;
    DWORD           dwNumberOfCsAddrs;
    LPCSADDR_INFO   lpcsaBuffer;
    DWORD           dwOutputFlags;
    LPBLOB          lpBlob;
} WSAQUERYSET2A, *PWSAQUERYSET2A, *LPWSAQUERYSET2A;
typedef struct _WSAQuerySet2W
{
    DWORD           dwSize;
    LPWSTR          lpszServiceInstanceName;
    LPWSAVERSION    lpVersion;
    LPWSTR          lpszComment;
    DWORD           dwNameSpace;
    LPGUID          lpNSProviderId;
    LPWSTR          lpszContext;
    DWORD           dwNumberOfProtocols;
    _Field_size_(dwNumberOfProtocols) LPAFPROTOCOLS   lpafpProtocols;
    LPWSTR          lpszQueryString;
    DWORD           dwNumberOfCsAddrs;
    _Field_size_(dwNumberOfCsAddrs) LPCSADDR_INFO   lpcsaBuffer;
    DWORD           dwOutputFlags;
    LPBLOB          lpBlob;
} WSAQUERYSET2W, *PWSAQUERYSET2W, *LPWSAQUERYSET2W;

#ifdef UNICODE
typedef WSAQUERYSETW WSAQUERYSET;
typedef PWSAQUERYSETW PWSAQUERYSET;
typedef LPWSAQUERYSETW LPWSAQUERYSET;
typedef WSAQUERYSET2W WSAQUERYSET2;
typedef PWSAQUERYSET2W PWSAQUERYSET2;
typedef LPWSAQUERYSET2W LPWSAQUERYSET2;
#else
typedef WSAQUERYSETA WSAQUERYSET;
typedef PWSAQUERYSETA PWSAQUERYSET;
typedef LPWSAQUERYSETA LPWSAQUERYSET;
typedef WSAQUERYSET2A WSAQUERYSET2;
typedef PWSAQUERYSET2A PWSAQUERYSET2;
typedef LPWSAQUERYSET2A LPWSAQUERYSET2;
#endif /* UNICODE */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#define LUP_DEEP                     0x00000001
#define LUP_CONTAINERS               0x00000002
#define LUP_NOCONTAINERS             0x00000004
#define LUP_NEAREST                  0x00000008
#define LUP_RETURN_NAME              0x00000010
#define LUP_RETURN_TYPE              0x00000020
#define LUP_RETURN_VERSION           0x00000040
#define LUP_RETURN_COMMENT           0x00000080
#define LUP_RETURN_ADDR              0x00000100
#define LUP_RETURN_BLOB              0x00000200
#define LUP_RETURN_ALIASES           0x00000400
#define LUP_RETURN_QUERY_STRING      0x00000800
#define LUP_RETURN_ALL               0x00000FF0
#define LUP_RES_SERVICE              0x00008000

#define LUP_FLUSHCACHE               0x00001000
#define LUP_FLUSHPREVIOUS            0x00002000

#define LUP_NON_AUTHORITATIVE        0x00004000
#define LUP_SECURE                   0x00008000
#define LUP_RETURN_PREFERRED_NAMES   0x00010000
#define LUP_DNS_ONLY                 0x00020000
#define LUP_RETURN_RESPONSE_FLAGS    0x00040000

#define LUP_RESERVED_UNUSED          0x00080000

#define LUP_ADDRCONFIG               0x00100000
#define LUP_DUAL_ADDR                0x00200000
#define LUP_FILESERVER               0x00400000
#define LUP_DISABLE_IDN_ENCODING     0x00800000
#define LUP_API_ANSI                 0x01000000

#define LUP_EXTENDED_QUERYSET        0x02000000
#define LUP_SECURE_WITH_FALLBACK     0x04000000
#define LUP_EXCLUSIVE_CUSTOM_SERVERS 0x08000000
#define LUP_REQUIRE_SECURE           0x10000000
#define LUP_RETURN_TTL               0x20000000
#define LUP_FORCE_CLEAR_TEXT         0x40000000
#define LUP_RESOLUTION_HANDLE        0x80000000

/*
 * Return flags
 */

#define  RESULT_IS_ALIAS      0x0001
#if(_WIN32_WINNT >= 0x0501)
#define  RESULT_IS_ADDED      0x0010
#define  RESULT_IS_CHANGED    0x0020
#define  RESULT_IS_DELETED    0x0040
#endif //(_WIN32_WINNT >= 0x0501)

/*
 * Service Address Registration and Deregistration Data Types.
 */

typedef enum _WSAESETSERVICEOP
{
    RNRSERVICE_REGISTER=0,
    RNRSERVICE_DEREGISTER,
    RNRSERVICE_DELETE
} WSAESETSERVICEOP, *PWSAESETSERVICEOP, *LPWSAESETSERVICEOP;

/*
 * Service Installation/Removal Data Types.
 */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
typedef struct _WINSOCK_DEPRECATED_BY("WSANSCLASSINFOW") _WSANSClassInfoA
{
    LPSTR   lpszName;
    DWORD   dwNameSpace;
    DWORD   dwValueType;
    DWORD   dwValueSize;
    LPVOID  lpValue;
}WSANSCLASSINFOA, *PWSANSCLASSINFOA, *LPWSANSCLASSINFOA;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

typedef struct _WSANSClassInfoW
{
    LPWSTR  lpszName;
    DWORD   dwNameSpace;
    DWORD   dwValueType;
    DWORD   dwValueSize;
    LPVOID  lpValue;
}WSANSCLASSINFOW, *PWSANSCLASSINFOW, *LPWSANSCLASSINFOW;
#ifdef UNICODE
typedef WSANSCLASSINFOW WSANSCLASSINFO;
typedef PWSANSCLASSINFOW PWSANSCLASSINFO;
typedef LPWSANSCLASSINFOW LPWSANSCLASSINFO;
#else
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef WSANSCLASSINFOA WSANSCLASSINFO;
typedef PWSANSCLASSINFOA PWSANSCLASSINFO;
typedef LPWSANSCLASSINFOA LPWSANSCLASSINFO;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif /* UNICODE */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
typedef struct _WINSOCK_DEPRECATED_BY("WSASERVICECLASSINFOW") _WSAServiceClassInfoA
{
    LPGUID              lpServiceClassId;
    LPSTR               lpszServiceClassName;
    DWORD               dwCount;
    LPWSANSCLASSINFOA   lpClassInfos;
}WSASERVICECLASSINFOA, *PWSASERVICECLASSINFOA, *LPWSASERVICECLASSINFOA;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

typedef struct _WSAServiceClassInfoW
{
    LPGUID              lpServiceClassId;
    LPWSTR              lpszServiceClassName;
    DWORD               dwCount;
    LPWSANSCLASSINFOW   lpClassInfos;
}WSASERVICECLASSINFOW, *PWSASERVICECLASSINFOW, *LPWSASERVICECLASSINFOW;

#ifdef UNICODE
typedef WSASERVICECLASSINFOW WSASERVICECLASSINFO;
typedef PWSASERVICECLASSINFOW PWSASERVICECLASSINFO;
typedef LPWSASERVICECLASSINFOW LPWSASERVICECLASSINFO;
#else
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef WSASERVICECLASSINFOA WSASERVICECLASSINFO;
typedef PWSASERVICECLASSINFOA PWSASERVICECLASSINFO;
typedef LPWSASERVICECLASSINFOA LPWSASERVICECLASSINFO;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif /* UNICODE */

typedef struct _WINSOCK_DEPRECATED_BY("WSANAMESPACE_INFOW") _WSANAMESPACE_INFOA {
    GUID                NSProviderId;
    DWORD               dwNameSpace;
    BOOL                fActive;
    DWORD               dwVersion;
    LPSTR               lpszIdentifier;
} WSANAMESPACE_INFOA, *PWSANAMESPACE_INFOA, *LPWSANAMESPACE_INFOA;

typedef struct _WSANAMESPACE_INFOW {
    GUID                NSProviderId;
    DWORD               dwNameSpace;
    BOOL                fActive;
    DWORD               dwVersion;
    LPWSTR              lpszIdentifier;
} WSANAMESPACE_INFOW, *PWSANAMESPACE_INFOW, *LPWSANAMESPACE_INFOW;

typedef struct _WINSOCK_DEPRECATED_BY("WSANAMESPACE_INFOEXW") _WSANAMESPACE_INFOEXA {
    GUID                NSProviderId;
    DWORD               dwNameSpace;
    BOOL                fActive;
    DWORD               dwVersion;
    LPSTR               lpszIdentifier;
    BLOB                ProviderSpecific;
} WSANAMESPACE_INFOEXA, *PWSANAMESPACE_INFOEXA, *LPWSANAMESPACE_INFOEXA;

typedef struct _WSANAMESPACE_INFOEXW {
    GUID                NSProviderId;
    DWORD               dwNameSpace;
    BOOL                fActive;
    DWORD               dwVersion;
    LPWSTR              lpszIdentifier;
    BLOB                ProviderSpecific;
} WSANAMESPACE_INFOEXW, *PWSANAMESPACE_INFOEXW, *LPWSANAMESPACE_INFOEXW;

#ifdef UNICODE
typedef WSANAMESPACE_INFOW WSANAMESPACE_INFO;
typedef PWSANAMESPACE_INFOW PWSANAMESPACE_INFO;
typedef LPWSANAMESPACE_INFOW LPWSANAMESPACE_INFO;
typedef WSANAMESPACE_INFOEXW WSANAMESPACE_INFOEX;
typedef PWSANAMESPACE_INFOEXW PWSANAMESPACE_INFOEX;
typedef LPWSANAMESPACE_INFOEXW LPWSANAMESPACE_INFOEX;
#else
typedef WSANAMESPACE_INFOA WSANAMESPACE_INFO;
typedef PWSANAMESPACE_INFOA PWSANAMESPACE_INFO;
typedef LPWSANAMESPACE_INFOA LPWSANAMESPACE_INFO;
typedef WSANAMESPACE_INFOEXA WSANAMESPACE_INFOEX;
typedef PWSANAMESPACE_INFOEXA PWSANAMESPACE_INFOEX;
typedef LPWSANAMESPACE_INFOEXA LPWSANAMESPACE_INFOEX;
#endif /* UNICODE */


#if(_WIN32_WINNT >= 0x0600)

/* Event flag definitions for WSAPoll(). */

#define POLLRDNORM  0x0100
#define POLLRDBAND  0x0200
#define POLLIN      (POLLRDNORM | POLLRDBAND)
#define POLLPRI     0x0400

#define POLLWRNORM  0x0010
#define POLLOUT     (POLLWRNORM)
#define POLLWRBAND  0x0020

#define POLLERR     0x0001
#define POLLHUP     0x0002
#define POLLNVAL    0x0004

typedef struct pollfd {

    SOCKET  fd;
    SHORT   events;
    SHORT   revents;

} WSAPOLLFD, *PWSAPOLLFD, FAR *LPWSAPOLLFD;

#endif // (_WIN32_WINNT >= 0x0600)


/* Socket function prototypes */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
_Must_inspect_result_
SOCKET
WSAAPI
accept(
    _In_ SOCKET s,
    _Out_writes_bytes_opt_(*addrlen) struct sockaddr FAR * addr,
    _Inout_opt_ int FAR * addrlen
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
_Must_inspect_result_
SOCKET
(WSAAPI * LPFN_ACCEPT)(
    _In_ SOCKET s,
    _Out_writes_bytes_opt_(*addrlen) struct sockaddr FAR * addr,
    _Inout_opt_ int FAR * addrlen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
bind(
    _In_ SOCKET s,
    _In_reads_bytes_(namelen) const struct sockaddr FAR * name,
    _In_ int namelen
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_BIND)(
    _In_ SOCKET s,
    _In_reads_bytes_(namelen) const struct sockaddr FAR * name,
    _In_ int namelen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
closesocket(
    _In_ SOCKET s
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_CLOSESOCKET)(
    _In_ SOCKET s
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
connect(
    _In_ SOCKET s,
    _In_reads_bytes_(namelen) const struct sockaddr FAR * name,
    _In_ int namelen
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_CONNECT)(
    _In_ SOCKET s,
    _In_reads_bytes_(namelen) const struct sockaddr FAR * name,
    _In_ int namelen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
ioctlsocket(
    _In_ SOCKET s,
    _In_ long cmd,
    _When_(cmd != FIONREAD, _Inout_)
    _When_(cmd == FIONREAD, _Out_)
    u_long FAR * argp
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_IOCTLSOCKET)(
    _In_ SOCKET s,
    _In_ long cmd,
    _Inout_ u_long FAR * argp
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
getpeername(
    _In_ SOCKET s,
    _Out_writes_bytes_to_(*namelen,*namelen) struct sockaddr FAR * name,
    _Inout_ int FAR * namelen
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_GETPEERNAME)(
    _In_ SOCKET s,
    _Out_writes_bytes_to_(*namelen,*namelen) struct sockaddr FAR * name,
    _Inout_ int FAR * namelen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
getsockname(
    _In_ SOCKET s,
    _Out_writes_bytes_to_(*namelen,*namelen) struct sockaddr FAR * name,
    _Inout_ int FAR * namelen
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_GETSOCKNAME)(
    _In_ SOCKET s,
    _Out_writes_bytes_to_(*namelen,*namelen) struct sockaddr FAR * name,
    _Inout_ int FAR * namelen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
getsockopt(
    _In_ SOCKET s,
    _In_ int level,
    _In_ int optname,
    _Out_writes_bytes_(*optlen) char FAR * optval,
    _Inout_ int FAR * optlen
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_GETSOCKOPT)(
    _In_ SOCKET s,
    _In_ int level,
    _In_ int optname,
    _Out_writes_bytes_(*optlen) char FAR * optval,
    _Inout_ int FAR * optlen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
u_long
WSAAPI
htonl(
    _In_ u_long hostlong
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
u_long
(WSAAPI * LPFN_HTONL)(
    _In_ u_long hostlong
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
u_short
WSAAPI
htons(
    _In_ u_short hostshort
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
u_short
(WSAAPI * LPFN_HTONS)(
    _In_ u_short hostshort
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("inet_pton() or InetPton()")
WINSOCK_API_LINKAGE
unsigned long
WSAAPI
inet_addr(
    _In_z_ const char FAR * cp
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
unsigned long
(WSAAPI * LPFN_INET_ADDR)(
    _In_ const char FAR * cp
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("inet_ntop() or InetNtop()")
WINSOCK_API_LINKAGE
char FAR *
WSAAPI
inet_ntoa(
    _In_ struct in_addr in
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
char FAR *
(WSAAPI * LPFN_INET_NTOA)(
    _In_ struct in_addr in
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */


#if !defined(NO_EXTRA_HTON_FUNCTIONS) && !defined(__midl) && (defined(INCL_EXTRA_HTON_FUNCTIONS) || NTDDI_VERSION>=NTDDI_WIN8)
/*
 * Byte order conversion functions for 64-bit integers and 32 + 64 bit
 * floating-point numbers.  IEEE big-endian format is used for the
 * network floating point format.
 */
#define _WS2_32_WINSOCK_SWAP_LONG(l)                \
            ( ( ((l) >> 24) & 0x000000FFL ) |       \
              ( ((l) >>  8) & 0x0000FF00L ) |       \
              ( ((l) <<  8) & 0x00FF0000L ) |       \
              ( ((l) << 24) & 0xFF000000L ) )

#define _WS2_32_WINSOCK_SWAP_LONGLONG(l)            \
            ( ( ((l) >> 56) & 0x00000000000000FFLL ) |       \
              ( ((l) >> 40) & 0x000000000000FF00LL ) |       \
              ( ((l) >> 24) & 0x0000000000FF0000LL ) |       \
              ( ((l) >>  8) & 0x00000000FF000000LL ) |       \
              ( ((l) <<  8) & 0x000000FF00000000LL ) |       \
              ( ((l) << 24) & 0x0000FF0000000000LL ) |       \
              ( ((l) << 40) & 0x00FF000000000000LL ) |       \
              ( ((l) << 56) & 0xFF00000000000000LL ) )


#ifndef htonll
__inline unsigned __int64 htonll ( unsigned __int64 Value )
{
	const unsigned __int64 Retval = _WS2_32_WINSOCK_SWAP_LONGLONG (Value);
	return Retval;
}
#endif /* htonll */

#ifndef ntohll
__inline unsigned __int64 ntohll ( unsigned __int64 Value )
{
	const unsigned __int64 Retval = _WS2_32_WINSOCK_SWAP_LONGLONG (Value);
	return Retval;
}
#endif /* ntohll */

#ifndef htonf
__inline unsigned __int32 htonf ( float Value )
{
	unsigned __int32 Tempval;
	unsigned __int32 Retval;
	Tempval = *(unsigned __int32*)(&Value);
	Retval = _WS2_32_WINSOCK_SWAP_LONG (Tempval);
	return Retval;
}
#endif /* htonf */

#ifndef ntohf
__inline float ntohf ( unsigned __int32 Value )
{
	const unsigned __int32 Tempval = _WS2_32_WINSOCK_SWAP_LONG (Value);
	float Retval;
	*((unsigned __int32*)&Retval) = Tempval;
	return Retval;
}
#endif /* ntohf */

#ifndef htond
__inline unsigned __int64 htond ( double Value )
{
	unsigned __int64 Tempval;
	unsigned __int64 Retval;
	Tempval = *(unsigned __int64*)(&Value);
	Retval = _WS2_32_WINSOCK_SWAP_LONGLONG (Tempval);
	return Retval;
}
#endif /* htond */

#ifndef ntohd
__inline double ntohd ( unsigned __int64 Value )
{
	const unsigned __int64 Tempval = _WS2_32_WINSOCK_SWAP_LONGLONG (Value);
	double Retval;
	*((unsigned __int64*)&Retval) = Tempval;
	return Retval;
}
#endif /* ntohd */
#endif /* NO_EXTRA_HTON_FUNCTIONS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
listen(
    _In_ SOCKET s,
    _In_ int backlog
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_LISTEN)(
    _In_ SOCKET s,
    _In_ int backlog
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
u_long
WSAAPI
ntohl(
    _In_ u_long netlong
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
u_long
(WSAAPI * LPFN_NTOHL)(
    _In_ u_long netlong
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
u_short
WSAAPI
ntohs(
    _In_ u_short netshort
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
u_short
(WSAAPI * LPFN_NTOHS)(
    _In_ u_short netshort
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
recv(
    _In_ SOCKET s,
    _Out_writes_bytes_to_(len, return) __out_data_source(NETWORK) char FAR * buf,
    _In_ int len,
    _In_ int flags
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_RECV)(
    _In_ SOCKET s,
    _Out_writes_bytes_to_(len, return) char FAR * buf,
    _In_ int len,
    _In_ int flags
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
recvfrom(
    _In_ SOCKET s,
    _Out_writes_bytes_to_(len, return) __out_data_source(NETWORK) char FAR * buf,
    _In_ int len,
    _In_ int flags,
    _Out_writes_bytes_to_opt_(*fromlen, *fromlen) struct sockaddr FAR * from,
    _Inout_opt_ int FAR * fromlen
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_RECVFROM)(
    _In_ SOCKET s,
    _Out_writes_bytes_to_(len, return) char FAR * buf,
    _In_ int len,
    _In_ int flags,
    _Out_writes_bytes_to_opt_(*fromlen, *fromlen) struct sockaddr FAR * from,
    _Inout_opt_ int FAR * fromlen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
select(
    _In_ int nfds,
    _Inout_opt_ fd_set FAR * readfds,
    _Inout_opt_ fd_set FAR * writefds,
    _Inout_opt_ fd_set FAR * exceptfds,
    _In_opt_ const struct timeval FAR * timeout
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_SELECT)(
    _In_ int nfds,
    _Inout_opt_ fd_set FAR * readfds,
    _Inout_opt_ fd_set FAR * writefds,
    _Inout_opt_ fd_set FAR *exceptfds,
    _In_opt_ const struct timeval FAR * timeout
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
send(
    _In_ SOCKET s,
    _In_reads_bytes_(len) const char FAR * buf,
    _In_ int len,
    _In_ int flags
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_SEND)(
    _In_ SOCKET s,
    _In_reads_bytes_(len) const char FAR * buf,
    _In_ int len,
    _In_ int flags
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
sendto(
    _In_ SOCKET s,
    _In_reads_bytes_(len) const char FAR * buf,
    _In_ int len,
    _In_ int flags,
    _In_reads_bytes_(tolen) const struct sockaddr FAR * to,
    _In_ int tolen
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_SENDTO)(
    _In_ SOCKET s,
    _In_reads_bytes_(len) const char FAR * buf,
    _In_ int len,
    _In_ int flags,
    _In_reads_bytes_(tolen) const struct sockaddr FAR * to,
    _In_ int tolen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
setsockopt(
    _In_ SOCKET s,
    _In_ int level,
    _In_ int optname,
    _In_reads_bytes_opt_(optlen) const char FAR * optval,
    _In_ int optlen
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_SETSOCKOPT)(
    _In_ SOCKET s,
    _In_ int level,
    _In_ int optname,
    _In_reads_bytes_(optlen) const char FAR * optval,
    _In_ int optlen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
shutdown(
    _In_ SOCKET s,
    _In_ int how
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_SHUTDOWN)(
    _In_ SOCKET s,
    _In_ int how
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
_Must_inspect_result_
SOCKET
WSAAPI
socket(
    _In_ int af,
    _In_ int type,
    _In_ int protocol
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
_Must_inspect_result_
SOCKET
(WSAAPI * LPFN_SOCKET)(
    _In_ int af,
    _In_ int type,
    _In_ int protocol
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

/* Database function prototypes */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("getnameinfo() or GetNameInfoW()")
WINSOCK_API_LINKAGE
struct hostent FAR *
WSAAPI
gethostbyaddr(
    _In_reads_bytes_(len) const char FAR * addr,
    _In_ int len,
    _In_ int type
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
struct hostent FAR *
(WSAAPI * LPFN_GETHOSTBYADDR)(
    _In_reads_bytes_(len) const char FAR * addr,
    _In_ int len,
    _In_ int type
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("getaddrinfo() or GetAddrInfoW()")
WINSOCK_API_LINKAGE
struct hostent FAR *
WSAAPI
gethostbyname(
    _In_z_ const char FAR * name
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
struct hostent FAR *
(WSAAPI * LPFN_GETHOSTBYNAME)(
    _In_ const char FAR * name
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
gethostname(
    _Out_writes_bytes_(namelen) char FAR * name,
    _In_ int namelen
    );

#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_GETHOSTNAME)(
    _Out_writes_bytes_(namelen) char FAR * name,
    _In_ int namelen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
GetHostNameW(
    _Out_writes_(namelen) PWSTR name,
    _In_ int namelen
    );

#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_GETHOSTNAMEW)(
    _Out_writes_(namelen) PWSTR name,
    _In_ int namelen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
struct servent FAR *
WSAAPI
getservbyport(
    _In_ int port,
    _In_opt_z_ const char FAR * proto
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
struct servent FAR *
(WSAAPI * LPFN_GETSERVBYPORT)(
    _In_ int port,
    _In_opt_z_ const char FAR * proto
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
struct servent FAR *
WSAAPI
getservbyname(
    _In_z_ const char FAR * name,
    _In_opt_z_ const char FAR * proto
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
struct servent FAR *
(WSAAPI * LPFN_GETSERVBYNAME)(
    _In_z_ const char FAR * name,
    _In_opt_z_ const char FAR * proto
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
struct protoent FAR *
WSAAPI
getprotobynumber(
    _In_ int number
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
struct protoent FAR *
(WSAAPI * LPFN_GETPROTOBYNUMBER)(
    _In_ int number
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
struct protoent FAR *
WSAAPI
getprotobyname(
    _In_z_ const char FAR * name
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
struct protoent FAR *
(WSAAPI * LPFN_GETPROTOBYNAME)(
    _In_z_ const char FAR * name
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

/* Microsoft Windows Extension function prototypes */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
_Must_inspect_result_
int
WSAAPI
WSAStartup(
    _In_ WORD wVersionRequested,
    _Out_ LPWSADATA lpWSAData
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
_Must_inspect_result_
int
(WSAAPI * LPFN_WSASTARTUP)(
    _In_ WORD wVersionRequested,
    _Out_ LPWSADATA lpWSAData
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
WSACleanup(
    void
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSACLEANUP)(
    void
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
void
WSAAPI
WSASetLastError(
    _In_ int iError
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
void
(WSAAPI * LPFN_WSASETLASTERROR)(
    int iError
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
WSAGetLastError(
    void
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSAGETLASTERROR)(
    void
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("Winsock 2")
WINSOCK_API_LINKAGE
BOOL
WSAAPI
WSAIsBlocking(
    void
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
BOOL
(WSAAPI * LPFN_WSAISBLOCKING)(
    void
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("Winsock 2")
WINSOCK_API_LINKAGE
int
WSAAPI
WSAUnhookBlockingHook(
    void
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSAUNHOOKBLOCKINGHOOK)(
    void
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("Winsock 2")
WINSOCK_API_LINKAGE
FARPROC
WSAAPI
WSASetBlockingHook(
    _In_ FARPROC lpBlockFunc
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
FARPROC
(WSAAPI * LPFN_WSASETBLOCKINGHOOK)(
    _In_ FARPROC lpBlockFunc
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("Winsock 2")
WINSOCK_API_LINKAGE
int
WSAAPI
WSACancelBlockingCall(
    void
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSACANCELBLOCKINGCALL)(
    void
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("getservbyname()")
WINSOCK_API_LINKAGE
HANDLE
WSAAPI
WSAAsyncGetServByName(
    _In_ HWND hWnd,
    _In_ u_int wMsg,
    _In_z_ const char FAR * name,
    _In_z_ const char FAR * proto,
    _Out_writes_bytes_(buflen) char FAR * buf,
    _In_ int buflen
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
HANDLE
(WSAAPI * LPFN_WSAASYNCGETSERVBYNAME)(
    _In_ HWND hWnd,
    _In_ u_int wMsg,
    _In_z_ const char FAR * name,
    _In_z_ const char FAR * proto,
    _Out_writes_bytes_(buflen) char FAR * buf,
    _In_ int buflen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("getservbyport()")
WINSOCK_API_LINKAGE
HANDLE
WSAAPI
WSAAsyncGetServByPort(
    _In_ HWND hWnd,
    _In_ u_int wMsg,
    _In_ int port,
    _In_ const char FAR * proto,
    _Out_writes_bytes_(buflen) char FAR * buf,
    _In_ int buflen
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
HANDLE
(WSAAPI * LPFN_WSAASYNCGETSERVBYPORT)(
    _In_ HWND hWnd,
    _In_ u_int wMsg,
    _In_ int port,
    _In_ const char FAR * proto,
    _Out_writes_bytes_(buflen) char FAR * buf,
    _In_ int buflen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("getprotobyname()")
WINSOCK_API_LINKAGE
HANDLE
WSAAPI
WSAAsyncGetProtoByName(
    _In_ HWND hWnd,
    _In_ u_int wMsg,
    _In_z_ const char FAR * name,
    _Out_writes_bytes_(buflen) char FAR * buf,
    _In_ int buflen
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
HANDLE
(WSAAPI * LPFN_WSAASYNCGETPROTOBYNAME)(
    _In_ HWND hWnd,
    _In_ u_int wMsg,
    _In_z_ const char FAR * name,
    _Out_writes_bytes_(buflen) char FAR * buf,
    _In_ int buflen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("getprotobynumber()")
WINSOCK_API_LINKAGE
HANDLE
WSAAPI
WSAAsyncGetProtoByNumber(
    _In_ HWND hWnd,
    _In_ u_int wMsg,
    _In_ int number,
    _Out_writes_bytes_(buflen) char FAR * buf,
    _In_ int buflen
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
HANDLE
(WSAAPI * LPFN_WSAASYNCGETPROTOBYNUMBER)(
    _In_ HWND hWnd,
    _In_ u_int wMsg,
    _In_ int number,
    _Out_writes_bytes_(buflen) char FAR * buf,
    _In_ int buflen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("GetAddrInfoExW()")
WINSOCK_API_LINKAGE
HANDLE
WSAAPI
WSAAsyncGetHostByName(
    _In_ HWND hWnd,
    _In_ u_int wMsg,
    _In_z_ const char FAR * name,
    _Out_writes_bytes_(buflen) char FAR * buf,
    _In_ int buflen
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
HANDLE
(WSAAPI * LPFN_WSAASYNCGETHOSTBYNAME)(
    _In_ HWND hWnd,
    _In_ u_int wMsg,
    _In_z_ const char FAR * name,
    _Out_writes_bytes_(buflen) char FAR * buf,
    _In_ int buflen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("getnameinfo() or GetNameInfoW()")
WINSOCK_API_LINKAGE
HANDLE
WSAAPI
WSAAsyncGetHostByAddr(
    _In_ HWND hWnd,
    _In_ u_int wMsg,
    _In_reads_bytes_(len) const char FAR * addr,
    _In_ int len,
    _In_ int type,
    _Out_writes_bytes_(buflen) char FAR * buf,
    _In_ int buflen
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
HANDLE
(WSAAPI * LPFN_WSAASYNCGETHOSTBYADDR)(
    _In_ HWND hWnd,
    _In_ u_int wMsg,
    _In_reads_bytes_(len) const char FAR * addr,
    _In_ int len,
    _In_ int type,
    _Out_writes_bytes_(buflen) char FAR * buf,
    _In_ int buflen
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED
WINSOCK_API_LINKAGE
int
WSAAPI
WSACancelAsyncRequest(
    _In_ HANDLE hAsyncTaskHandle
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSACANCELASYNCREQUEST)(
    _In_ HANDLE hAsyncTaskHandle
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("WSAEventSelect()")
WINSOCK_API_LINKAGE
int
WSAAPI
WSAAsyncSelect(
    _In_ SOCKET s,
    _In_ HWND hWnd,
    _In_ u_int wMsg,
    _In_ long lEvent
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSAASYNCSELECT)(
    _In_ SOCKET s,
    _In_ HWND hWnd,
    _In_ u_int wMsg,
    _In_ long lEvent
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

/* WinSock 2 API new function prototypes */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
_Must_inspect_result_
SOCKET
WSAAPI
WSAAccept(
    _In_ SOCKET s,
    _Out_writes_bytes_to_opt_(*addrlen,*addrlen) struct sockaddr FAR * addr,
    _Inout_opt_ LPINT addrlen,
    _In_opt_ LPCONDITIONPROC lpfnCondition,
    _In_opt_ DWORD_PTR dwCallbackData
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
_Must_inspect_result_
SOCKET
(WSAAPI * LPFN_WSAACCEPT)(
    _In_ SOCKET s,
    _Out_writes_bytes_to_opt_(*addrlen,*addrlen) struct sockaddr FAR * addr,
    _Inout_opt_ LPINT addrlen,
    _In_opt_ LPCONDITIONPROC lpfnCondition,
    _In_opt_ DWORD_PTR dwCallbackData
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
BOOL
WSAAPI
WSACloseEvent(
    _In_ WSAEVENT hEvent
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
BOOL
(WSAAPI * LPFN_WSACLOSEEVENT)(
    _In_ WSAEVENT hEvent
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
WSAConnect(
    _In_ SOCKET s,
    _In_reads_bytes_(namelen) const struct sockaddr FAR * name,
    _In_ int namelen,
    _In_opt_ LPWSABUF lpCallerData,
    _Out_opt_ LPWSABUF lpCalleeData,
    _In_opt_ LPQOS lpSQOS,
    _In_opt_ LPQOS lpGQOS
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_PROTOTYPES

#ifdef UNICODE
#define WSAConnectByName    WSAConnectByNameW
#else
#define WSAConnectByName    WSAConnectByNameA
#endif

BOOL
PASCAL
WSAConnectByNameW(
    _In_ SOCKET s,
    _In_ LPWSTR nodename,
    _In_ LPWSTR servicename,
    _Inout_opt_ LPDWORD LocalAddressLength,
    _Out_writes_bytes_to_opt_(*LocalAddressLength,*LocalAddressLength) LPSOCKADDR LocalAddress,
    _Inout_opt_ LPDWORD RemoteAddressLength,
    _Out_writes_bytes_to_opt_(*RemoteAddressLength,*RemoteAddressLength) LPSOCKADDR RemoteAddress,
    _In_opt_ const struct timeval * timeout,
    _Reserved_ LPWSAOVERLAPPED Reserved);

_WINSOCK_DEPRECATED_BY("WSAConnectByNameW()")
BOOL
PASCAL
WSAConnectByNameA(
    _In_ SOCKET s,
    _In_ LPCSTR nodename,
    _In_ LPCSTR servicename,
    _Inout_opt_ LPDWORD LocalAddressLength,
    _Out_writes_bytes_to_opt_(*LocalAddressLength,*LocalAddressLength) LPSOCKADDR LocalAddress,
    _Inout_opt_ LPDWORD RemoteAddressLength,
    _Out_writes_bytes_to_opt_(*RemoteAddressLength,*RemoteAddressLength) LPSOCKADDR RemoteAddress,
    _In_opt_ const struct timeval * timeout,
    _Reserved_ LPWSAOVERLAPPED Reserved);

BOOL
PASCAL
WSAConnectByList(
    _In_ SOCKET s,
    _In_ PSOCKET_ADDRESS_LIST SocketAddress,
    _Inout_opt_ LPDWORD LocalAddressLength,
    _Out_writes_bytes_to_opt_(*LocalAddressLength,*LocalAddressLength) LPSOCKADDR LocalAddress,
    _Inout_opt_ LPDWORD RemoteAddressLength,
    _Out_writes_bytes_to_opt_(*RemoteAddressLength,*RemoteAddressLength) LPSOCKADDR RemoteAddress,
    _In_opt_ const struct timeval * timeout,
    _Reserved_ LPWSAOVERLAPPED Reserved);
#endif

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSACONNECT)(
    _In_ SOCKET s,
    _In_reads_bytes_(namelen) const struct sockaddr FAR * name,
    _In_ int namelen,
    _In_opt_ LPWSABUF lpCallerData,
    _Out_opt_ LPWSABUF lpCalleeData,
    _In_opt_ LPQOS lpSQOS,
    _In_opt_ LPQOS lpGQOS
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
WSAEVENT
WSAAPI
WSACreateEvent(
    void
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
WSAEVENT
(WSAAPI * LPFN_WSACREATEEVENT)(
    void
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("WSADuplicateSocketW()")
WINSOCK_API_LINKAGE
int
WSAAPI
WSADuplicateSocketA(
    _In_ SOCKET s,
    _In_ DWORD dwProcessId,
    _Out_ LPWSAPROTOCOL_INFOA lpProtocolInfo
    );

WINSOCK_API_LINKAGE
int
WSAAPI
WSADuplicateSocketW(
    _In_ SOCKET s,
    _In_ DWORD dwProcessId,
    _Out_ LPWSAPROTOCOL_INFOW lpProtocolInfo
    );
#ifdef UNICODE
#define WSADuplicateSocket  WSADuplicateSocketW
#else
#define WSADuplicateSocket  WSADuplicateSocketA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSADUPLICATESOCKETA)(
    _In_ SOCKET s,
    _In_ DWORD dwProcessId,
    _Out_ LPWSAPROTOCOL_INFOA lpProtocolInfo
    );

typedef
int
(WSAAPI * LPFN_WSADUPLICATESOCKETW)(
    _In_ SOCKET s,
    _In_ DWORD dwProcessId,
    _Out_ LPWSAPROTOCOL_INFOW lpProtocolInfo
    );
#ifdef UNICODE
#define LPFN_WSADUPLICATESOCKET  LPFN_WSADUPLICATESOCKETW
#else
#define LPFN_WSADUPLICATESOCKET  LPFN_WSADUPLICATESOCKETA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
WSAEnumNetworkEvents(
    _In_ SOCKET s,
    _In_ WSAEVENT hEventObject,
    _Out_ LPWSANETWORKEVENTS lpNetworkEvents
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSAENUMNETWORKEVENTS)(
    _In_ SOCKET s,
    _In_ WSAEVENT hEventObject,
    _Out_ LPWSANETWORKEVENTS lpNetworkEvents
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("WSAEnumProtocolsW()")
WINSOCK_API_LINKAGE
int
WSAAPI
WSAEnumProtocolsA(
    _In_opt_ LPINT lpiProtocols,
    _Out_writes_bytes_to_opt_(*lpdwBufferLength,*lpdwBufferLength) LPWSAPROTOCOL_INFOA lpProtocolBuffer,
    _Inout_ LPDWORD lpdwBufferLength
    );

WINSOCK_API_LINKAGE
int
WSAAPI
WSAEnumProtocolsW(
    _In_opt_ LPINT lpiProtocols,
    _Out_writes_bytes_to_opt_(*lpdwBufferLength,*lpdwBufferLength) LPWSAPROTOCOL_INFOW lpProtocolBuffer,
    _Inout_ LPDWORD lpdwBufferLength
    );
#ifdef UNICODE
#define WSAEnumProtocols  WSAEnumProtocolsW
#else
#define WSAEnumProtocols  WSAEnumProtocolsA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSAENUMPROTOCOLSA)(
    _In_opt_ LPINT lpiProtocols,
    _Out_writes_bytes_to_opt_(*lpdwBufferLength,*lpdwBufferLength) LPWSAPROTOCOL_INFOA lpProtocolBuffer,
    _Inout_ LPDWORD lpdwBufferLength
    );

typedef
int
(WSAAPI * LPFN_WSAENUMPROTOCOLSW)(
    _In_opt_ LPINT lpiProtocols,
    _Out_writes_bytes_to_opt_(*lpdwBufferLength,*lpdwBufferLength) LPWSAPROTOCOL_INFOW lpProtocolBuffer,
    _Inout_ LPDWORD lpdwBufferLength
    );
#ifdef UNICODE
#define LPFN_WSAENUMPROTOCOLS  LPFN_WSAENUMPROTOCOLSW
#else
#define LPFN_WSAENUMPROTOCOLS  LPFN_WSAENUMPROTOCOLSA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
WSAEventSelect(
    _In_ SOCKET s,
    _In_opt_ WSAEVENT hEventObject,
    _In_ long lNetworkEvents
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSAEVENTSELECT)(
    _In_ SOCKET s,
    _In_opt_ WSAEVENT hEventObject,
    _In_ long lNetworkEvents
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
BOOL
WSAAPI
WSAGetOverlappedResult(
    _In_ SOCKET s,
    _In_ LPWSAOVERLAPPED lpOverlapped,
    _Out_ LPDWORD lpcbTransfer,
    _In_ BOOL fWait,
    _Out_ LPDWORD lpdwFlags
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
BOOL
(WSAAPI * LPFN_WSAGETOVERLAPPEDRESULT)(
    _In_ SOCKET s,
    _In_ LPWSAOVERLAPPED lpOverlapped,
    _Out_ LPDWORD lpcbTransfer,
    _In_ BOOL fWait,
    _Out_ LPDWORD lpdwFlags
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED
WINSOCK_API_LINKAGE
BOOL
WSAAPI
WSAGetQOSByName(
    _In_ SOCKET s,
    _In_ LPWSABUF lpQOSName,
    _Out_ LPQOS lpQOS
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
BOOL
(WSAAPI * LPFN_WSAGETQOSBYNAME)(
    _In_ SOCKET s,
    _In_ LPWSABUF lpQOSName,
    _Out_ LPQOS lpQOS
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
WSAHtonl(
    _In_ IN SOCKET s,
    _In_ IN u_long hostlong,
    _Out_ OUT u_long FAR * lpnetlong
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSAHTONL)(
    _In_ SOCKET s,
    _In_ u_long hostlong,
    _Out_ u_long FAR * lpnetlong
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
WSAHtons(
    _In_ IN SOCKET s,
    _In_ IN u_short hostshort,
    _Out_ OUT u_short FAR * lpnetshort
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSAHTONS)(
    _In_ SOCKET s,
    _In_ u_short hostshort,
    _Out_ u_short FAR * lpnetshort
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
WSAIoctl(
    _In_ SOCKET s,
    _In_ DWORD dwIoControlCode,
    _In_reads_bytes_opt_(cbInBuffer) LPVOID lpvInBuffer,
    _In_ DWORD cbInBuffer,
    _Out_writes_bytes_to_opt_(cbOutBuffer, *lpcbBytesReturned) LPVOID lpvOutBuffer,
    _In_ DWORD cbOutBuffer,
    _Out_ LPDWORD lpcbBytesReturned,
    _Inout_opt_ LPWSAOVERLAPPED lpOverlapped,
    _In_opt_ LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSAIOCTL)(
    _In_ SOCKET s,
    _In_ DWORD dwIoControlCode,
    _In_reads_bytes_opt_(cbInBuffer) LPVOID lpvInBuffer,
    _In_ DWORD cbInBuffer,
    _Out_writes_bytes_to_opt_(cbOutBuffer, *lpcbBytesReturned) LPVOID lpvOutBuffer,
    _In_ DWORD cbOutBuffer,
    _Out_ LPDWORD lpcbBytesReturned,
    _Inout_opt_ LPWSAOVERLAPPED lpOverlapped,
    _In_opt_ LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
SOCKET
WSAAPI
WSAJoinLeaf(
    _In_ SOCKET s,
    _In_reads_bytes_(namelen) const struct sockaddr FAR * name,
    _In_ int namelen,
    _In_opt_ LPWSABUF lpCallerData,
    _Out_opt_ LPWSABUF lpCalleeData,
    _In_opt_ LPQOS lpSQOS,
    _In_opt_ LPQOS lpGQOS,
    _In_ DWORD dwFlags
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
SOCKET
(WSAAPI * LPFN_WSAJOINLEAF)(
    _In_ SOCKET s,
    _In_reads_bytes_(namelen) const struct sockaddr FAR * name,
    _In_ int namelen,
    _In_opt_ LPWSABUF lpCallerData,
    _Out_opt_ LPWSABUF lpCalleeData,
    _In_opt_ LPQOS lpSQOS,
    _In_opt_ LPQOS lpGQOS,
    _In_ DWORD dwFlags
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
WSANtohl(
    _In_ SOCKET s,
    _In_ u_long netlong,
    _Out_ u_long FAR * lphostlong
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSANTOHL)(
    _In_ SOCKET s,
    _In_ u_long netlong,
    _Out_ u_long FAR * lphostlong
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
WSANtohs(
    _In_ SOCKET s,
    _In_ u_short netshort,
    _Out_ u_short FAR * lphostshort
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSANTOHS)(
    _In_ SOCKET s,
    _In_ u_short netshort,
    _Out_ u_short FAR * lphostshort
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
WSARecv(
    _In_ SOCKET s,
    _In_reads_(dwBufferCount) __out_data_source(NETWORK) LPWSABUF lpBuffers,
    _In_ DWORD dwBufferCount,
    _Out_opt_ LPDWORD lpNumberOfBytesRecvd,
    _Inout_ LPDWORD lpFlags,
    _Inout_opt_ LPWSAOVERLAPPED lpOverlapped,
    _In_opt_ LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSARECV)(
    _In_ SOCKET s,
    _In_reads_(dwBufferCount) LPWSABUF lpBuffers,
    _In_ DWORD dwBufferCount,
    _Out_opt_ LPDWORD lpNumberOfBytesRecvd,
    _Inout_ LPDWORD lpFlags,
    _Inout_opt_ LPWSAOVERLAPPED lpOverlapped,
    _In_opt_ LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("WSARecv()")
WINSOCK_API_LINKAGE
int
WSAAPI
WSARecvDisconnect(
    _In_ SOCKET s,
    _In_opt_ __out_data_source(NETWORK) LPWSABUF lpInboundDisconnectData
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSARECVDISCONNECT)(
    _In_ SOCKET s,
    __out_data_source(NETWORK) LPWSABUF lpInboundDisconnectData
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
WSARecvFrom(
    _In_ SOCKET s,
    _In_reads_(dwBufferCount) __out_data_source(NETWORK) LPWSABUF lpBuffers,
    _In_ DWORD dwBufferCount,
    _Out_opt_ LPDWORD lpNumberOfBytesRecvd,
    _Inout_ LPDWORD lpFlags,
    _Out_writes_bytes_to_opt_(*lpFromlen,*lpFromlen) struct sockaddr FAR * lpFrom,
    _Inout_opt_ LPINT lpFromlen,
    _Inout_opt_ LPWSAOVERLAPPED lpOverlapped,
    _In_opt_ LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSARECVFROM)(
    _In_ SOCKET s,
    _In_reads_(dwBufferCount) LPWSABUF lpBuffers,
    _In_ DWORD dwBufferCount,
    _Out_opt_ LPDWORD lpNumberOfBytesRecvd,
    _Inout_ LPDWORD lpFlags,
    _Out_writes_bytes_to_opt_(*lpFromlen,*lpFromLen) struct sockaddr FAR * lpFrom,
    _Inout_opt_ LPINT lpFromlen,
    _Inout_opt_ LPWSAOVERLAPPED lpOverlapped,
    _In_opt_ LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
BOOL
WSAAPI
WSAResetEvent(
    _In_ WSAEVENT hEvent
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
BOOL
(WSAAPI * LPFN_WSARESETEVENT)(
    _In_ WSAEVENT hEvent
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
WSASend(
    _In_ SOCKET s,
    _In_reads_(dwBufferCount) LPWSABUF lpBuffers,
    _In_ DWORD dwBufferCount,
    _Out_opt_ LPDWORD lpNumberOfBytesSent,
    _In_ DWORD dwFlags,
    _Inout_opt_ LPWSAOVERLAPPED lpOverlapped,
    _In_opt_ LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSASEND)(
    _In_ SOCKET s,
    _In_reads_(dwBufferCount) LPWSABUF lpBuffers,
    _In_ DWORD dwBufferCount,
    _Out_opt_ LPDWORD lpNumberOfBytesSent,
    _In_  DWORD dwFlags,
    _Inout_opt_ LPWSAOVERLAPPED lpOverlapped,
    _In_opt_ LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if(_WIN32_WINNT >= 0x0600)
#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
WSASendMsg(
    _In_ SOCKET Handle,
    _In_ LPWSAMSG lpMsg,
    _In_ DWORD dwFlags,
    _Out_opt_ LPDWORD lpNumberOfBytesSent,
    _Inout_opt_ LPWSAOVERLAPPED lpOverlapped,
    _In_opt_ LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */
#endif // (_WIN32_WINNT >= 0x0600)

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("WSASend()")
WINSOCK_API_LINKAGE
int
WSAAPI
WSASendDisconnect(
    _In_ SOCKET s,
    _In_opt_ LPWSABUF lpOutboundDisconnectData
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSASENDDISCONNECT)(
    _In_ SOCKET s,
    _In_opt_ LPWSABUF lpOutboundDisconnectData
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
WSASendTo(
    _In_ SOCKET s,
    _In_reads_(dwBufferCount) LPWSABUF lpBuffers,
    _In_ DWORD dwBufferCount,
    _Out_opt_ LPDWORD lpNumberOfBytesSent,
    _In_ DWORD dwFlags,
    _In_reads_bytes_opt_(iTolen) const struct sockaddr FAR * lpTo,
    _In_ int iTolen,
    _Inout_opt_ LPWSAOVERLAPPED lpOverlapped,
    _In_opt_ LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
int
(WSAAPI * LPFN_WSASENDTO)(
    _In_ SOCKET s,
    _In_reads_(dwBufferCount) LPWSABUF lpBuffers,
    _In_ DWORD dwBufferCount,
    _Out_opt_ LPDWORD lpNumberOfBytesSent,
    _In_ DWORD dwFlags,
    _In_reads_bytes_opt_(iTolen) const struct sockaddr FAR * lpTo,
    _In_ int iTolen,
    _Inout_opt_ LPWSAOVERLAPPED lpOverlapped,
    _In_opt_ LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
BOOL
WSAAPI
WSASetEvent(
    _In_ WSAEVENT hEvent
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
BOOL
(WSAAPI * LPFN_WSASETEVENT)(
    _In_ WSAEVENT hEvent
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES

_WINSOCK_DEPRECATED_BY("WSASocketW()")
WINSOCK_API_LINKAGE
_Must_inspect_result_
SOCKET
WSAAPI
WSASocketA(
    _In_ int af,
    _In_ int type,
    _In_ int protocol,
    _In_opt_ LPWSAPROTOCOL_INFOA lpProtocolInfo,
    _In_ GROUP g,
    _In_ DWORD dwFlags
    );

WINSOCK_API_LINKAGE
_Must_inspect_result_
SOCKET
WSAAPI
WSASocketW(
    _In_ int af,
    _In_ int type,
    _In_ int protocol,
    _In_opt_ LPWSAPROTOCOL_INFOW lpProtocolInfo,
    _In_ GROUP g,
    _In_ DWORD dwFlags
    );
#ifdef UNICODE
#define WSASocket  WSASocketW
#else
#define WSASocket  WSASocketA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS

typedef
_Must_inspect_result_
SOCKET
(WSAAPI * LPFN_WSASOCKETA)(
    _In_ int af,
    _In_ int type,
    _In_ int protocol,
    _In_opt_ LPWSAPROTOCOL_INFOA lpProtocolInfo,
    _In_ GROUP g,
    _In_ DWORD dwFlags
    );

typedef
_Must_inspect_result_
SOCKET
(WSAAPI * LPFN_WSASOCKETW)(
    _In_ int af,
    _In_ int type,
    _In_ int protocol,
    _In_opt_ LPWSAPROTOCOL_INFOW lpProtocolInfo,
    _In_ GROUP g,
    _In_ DWORD dwFlags
    );
#ifdef UNICODE
#define LPFN_WSASOCKET  LPFN_WSASOCKETW
#else
#define LPFN_WSASOCKET  LPFN_WSASOCKETA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
DWORD
WSAAPI
WSAWaitForMultipleEvents(
    _In_ DWORD cEvents,
    _In_reads_(cEvents) const WSAEVENT FAR * lphEvents,
    _In_ BOOL fWaitAll,
    _In_ DWORD dwTimeout,
    _In_ BOOL fAlertable
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
DWORD
(WSAAPI * LPFN_WSAWAITFORMULTIPLEEVENTS)(
    _In_ DWORD cEvents,
    _In_reads_(cEvents) const WSAEVENT FAR * lphEvents,
    _In_ BOOL fWaitAll,
    _In_ DWORD dwTimeout,
    _In_ BOOL fAlertable
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES

_WINSOCK_DEPRECATED_BY("WSAAddressToStringW()")
WINSOCK_API_LINKAGE
INT
WSAAPI
WSAAddressToStringA(
    _In_reads_bytes_(dwAddressLength) LPSOCKADDR lpsaAddress,
    _In_     DWORD               dwAddressLength,
    _In_opt_ LPWSAPROTOCOL_INFOA lpProtocolInfo,
    _Out_writes_to_(*lpdwAddressStringLength,*lpdwAddressStringLength) LPSTR lpszAddressString,
    _Inout_  LPDWORD             lpdwAddressStringLength
    );

WINSOCK_API_LINKAGE
INT
WSAAPI
WSAAddressToStringW(
    _In_reads_bytes_(dwAddressLength) LPSOCKADDR lpsaAddress,
    _In_     DWORD               dwAddressLength,
    _In_opt_ LPWSAPROTOCOL_INFOW lpProtocolInfo,
    _Out_writes_to_(*lpdwAddressStringLength,*lpdwAddressStringLength) LPWSTR lpszAddressString,
    _Inout_  LPDWORD             lpdwAddressStringLength
    );
#ifdef UNICODE
#define WSAAddressToString  WSAAddressToStringW
#else
#define WSAAddressToString  WSAAddressToStringA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS

typedef
INT
(WSAAPI * LPFN_WSAADDRESSTOSTRINGA)(
    _In_reads_bytes_(dwAddressLength) LPSOCKADDR lpsaAddress,
    _In_     DWORD               dwAddressLength,
    _In_opt_ LPWSAPROTOCOL_INFOA lpProtocolInfo,
    _Out_writes_to_(*lpdwAddressStringLength,*lpdwAddressStringLength) LPSTR lpszAddressString,
    _Inout_  LPDWORD             lpdwAddressStringLength
    );

typedef
INT
(WSAAPI * LPFN_WSAADDRESSTOSTRINGW)(
    _In_reads_bytes_(dwAddressLength) LPSOCKADDR lpsaAddress,
    _In_     DWORD               dwAddressLength,
    _In_opt_ LPWSAPROTOCOL_INFOW lpProtocolInfo,
    _Out_writes_to_(*lpdwAddressStringLength,*lpdwAddressStringLength) LPWSTR lpszAddressString,
    _Inout_  LPDWORD             lpdwAddressStringLength
    );
#ifdef UNICODE
#define LPFN_WSAADDRESSTOSTRING  LPFN_WSAADDRESSTOSTRINGW
#else
#define LPFN_WSAADDRESSTOSTRING  LPFN_WSAADDRESSTOSTRINGA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES

_WINSOCK_DEPRECATED_BY("WSAStringToAddressW()")
WINSOCK_API_LINKAGE
INT
WSAAPI
WSAStringToAddressA(
    _In_    LPSTR               AddressString,
    _In_    INT                 AddressFamily,
    _In_opt_ LPWSAPROTOCOL_INFOA lpProtocolInfo,
    _Out_writes_bytes_to_(*lpAddressLength,*lpAddressLength) LPSOCKADDR lpAddress,
    _Inout_ LPINT               lpAddressLength
    );

WINSOCK_API_LINKAGE
INT
WSAAPI
WSAStringToAddressW(
    _In_    LPWSTR             AddressString,
    _In_    INT                AddressFamily,
    _In_opt_ LPWSAPROTOCOL_INFOW lpProtocolInfo,
    _Out_writes_bytes_to_(*lpAddressLength,*lpAddressLength) LPSOCKADDR lpAddress,
    _Inout_ LPINT              lpAddressLength
    );
#ifdef UNICODE
#define WSAStringToAddress  WSAStringToAddressW
#else
#define WSAStringToAddress  WSAStringToAddressA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS

typedef
INT
(WSAAPI * LPFN_WSASTRINGTOADDRESSA)(
    _In_    LPSTR              AddressString,
    _In_    INT                AddressFamily,
    _In_opt_ LPWSAPROTOCOL_INFOA lpProtocolInfo,
    _Out_writes_bytes_to_(*lpAddressLength,*lpAddressLength) LPSOCKADDR lpAddress,
    _Inout_ LPINT              lpAddressLength
    );

typedef
INT
(WSAAPI * LPFN_WSASTRINGTOADDRESSW)(
    _In_    LPWSTR             AddressString,
    _In_    INT                AddressFamily,
    _In_opt_ LPWSAPROTOCOL_INFOW lpProtocolInfo,
    _Out_writes_bytes_to_(*lpAddressLength,*lpAddressLength) LPSOCKADDR lpAddress,
    _Inout_ LPINT              lpAddressLength
    );
#ifdef UNICODE
#define LPFN_WSASTRINGTOADDRESS  LPFN_WSASTRINGTOADDRESSW
#else
#define LPFN_WSASTRINGTOADDRESS  LPFN_WSASTRINGTOADDRESSA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_TYPEDEFS */

/* Registration and Name Resolution API functions */


#if INCL_WINSOCK_API_PROTOTYPES

_WINSOCK_DEPRECATED_BY("WSALookupServiceBeginW()")
WINSOCK_API_LINKAGE
INT
WSAAPI
WSALookupServiceBeginA(
    _In_ LPWSAQUERYSETA lpqsRestrictions,
    _In_ DWORD          dwControlFlags,
    _Out_ LPHANDLE       lphLookup
    );

WINSOCK_API_LINKAGE
INT
WSAAPI
WSALookupServiceBeginW(
    _In_ LPWSAQUERYSETW lpqsRestrictions,
    _In_ DWORD          dwControlFlags,
    _Out_ LPHANDLE       lphLookup
    );
#ifdef UNICODE
#define WSALookupServiceBegin  WSALookupServiceBeginW
#else
#define WSALookupServiceBegin  WSALookupServiceBeginA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS

typedef
INT
(WSAAPI * LPFN_WSALOOKUPSERVICEBEGINA)(
    _In_  LPWSAQUERYSETA lpqsRestrictions,
    _In_  DWORD          dwControlFlags,
    _Out_ LPHANDLE       lphLookup
    );

typedef
INT
(WSAAPI * LPFN_WSALOOKUPSERVICEBEGINW)(
    _In_  LPWSAQUERYSETW lpqsRestrictions,
    _In_  DWORD          dwControlFlags,
    _Out_ LPHANDLE       lphLookup
    );
#ifdef UNICODE
#define LPFN_WSALOOKUPSERVICEBEGIN  LPFN_WSALOOKUPSERVICEBEGINW
#else
#define LPFN_WSALOOKUPSERVICEBEGIN  LPFN_WSALOOKUPSERVICEBEGINA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES

_WINSOCK_DEPRECATED_BY("WSALookupServiceNextW()")
WINSOCK_API_LINKAGE
INT
WSAAPI
WSALookupServiceNextA(
    _In_ HANDLE           hLookup,
    _In_ DWORD            dwControlFlags,
    _Inout_ LPDWORD       lpdwBufferLength,
    _Out_writes_bytes_to_(*lpdwBufferLength,*lpdwBufferLength) LPWSAQUERYSETA lpqsResults
    );

WINSOCK_API_LINKAGE
INT
WSAAPI
WSALookupServiceNextW(
    _In_ HANDLE           hLookup,
    _In_ DWORD            dwControlFlags,
    _Inout_ LPDWORD       lpdwBufferLength,
    _Out_writes_bytes_to_opt_(*lpdwBufferLength,*lpdwBufferLength) LPWSAQUERYSETW lpqsResults
    );
#ifdef UNICODE
#define WSALookupServiceNext  WSALookupServiceNextW
#else
#define WSALookupServiceNext  WSALookupServiceNextA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS

typedef
INT
(WSAAPI * LPFN_WSALOOKUPSERVICENEXTA)(
    _In_   HANDLE           hLookup,
    _In_   DWORD            dwControlFlags,
    _Inout_ LPDWORD         lpdwBufferLength,
    _Out_writes_bytes_to_(*lpdwBufferLength,*lpdwBufferLength) LPWSAQUERYSETA   lpqsResults
    );

typedef
INT
(WSAAPI * LPFN_WSALOOKUPSERVICENEXTW)(
    _In_   HANDLE           hLookup,
    _In_   DWORD            dwControlFlags,
    _Inout_ LPDWORD         lpdwBufferLength,
    _Out_writes_bytes_to_opt_(*lpdwBufferLength,*lpdwBufferLength) LPWSAQUERYSETW   lpqsResults
    );
#ifdef UNICODE
#define LPFN_WSALOOKUPSERVICENEXT  LPFN_WSALOOKUPSERVICENEXTW
#else
#define LPFN_WSALOOKUPSERVICENEXT  LPFN_WSALOOKUPSERVICENEXTA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if(_WIN32_WINNT >= 0x0501)
#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
INT
WSAAPI
WSANSPIoctl(
    _In_ HANDLE           hLookup,
    _In_ DWORD            dwControlCode,
    _In_reads_bytes_opt_(cbInBuffer) LPVOID lpvInBuffer,
    _In_ DWORD            cbInBuffer,
    _Out_writes_bytes_to_opt_(cbOutBuffer, *lpcbBytesReturned) LPVOID lpvOutBuffer,
    _In_ DWORD            cbOutBuffer,
    _Out_ LPDWORD        lpcbBytesReturned,
    _In_opt_ LPWSACOMPLETION lpCompletion
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
INT
(WSAAPI * LPFN_WSANSPIOCTL)(
    IN  HANDLE           hLookup,
    IN  DWORD            dwControlCode,
    _In_reads_bytes_opt_(cbInBuffer) LPVOID lpvInBuffer,
    IN  DWORD            cbInBuffer,
    _Out_writes_bytes_to_opt_(cbOutBuffer, *lpcbBytesReturned) LPVOID lpvOutBuffer,
    IN  DWORD            cbOutBuffer,
    _Out_ LPDWORD        lpcbBytesReturned,
    _In_opt_ LPWSACOMPLETION lpCompletion
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */
#endif //(_WIN32_WINNT >= 0x0501)

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
INT
WSAAPI
WSALookupServiceEnd(
    _In_ HANDLE  hLookup
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
INT
(WSAAPI * LPFN_WSALOOKUPSERVICEEND)(
    _In_ HANDLE  hLookup
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("WSAInstallServiceClassW()")
WINSOCK_API_LINKAGE
INT
WSAAPI
WSAInstallServiceClassA(
    _In_  LPWSASERVICECLASSINFOA   lpServiceClassInfo
    );
WINSOCK_API_LINKAGE
INT
WSAAPI
WSAInstallServiceClassW(
    _In_  LPWSASERVICECLASSINFOW   lpServiceClassInfo
    );
#ifdef UNICODE
#define WSAInstallServiceClass  WSAInstallServiceClassW
#else
#define WSAInstallServiceClass  WSAInstallServiceClassA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
INT
(WSAAPI * LPFN_WSAINSTALLSERVICECLASSA)(
    _In_  LPWSASERVICECLASSINFOA   lpServiceClassInfo
    );
typedef
INT
(WSAAPI * LPFN_WSAINSTALLSERVICECLASSW)(
    _In_  LPWSASERVICECLASSINFOW   lpServiceClassInfo
    );
#ifdef UNICODE
#define LPFN_WSAINSTALLSERVICECLASS  LPFN_WSAINSTALLSERVICECLASSW
#else
#define LPFN_WSAINSTALLSERVICECLASS  LPFN_WSAINSTALLSERVICECLASSA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
INT
WSAAPI
WSARemoveServiceClass(
    _In_  LPGUID  lpServiceClassId
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
INT
(WSAAPI * LPFN_WSAREMOVESERVICECLASS)(
    _In_  LPGUID  lpServiceClassId
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if INCL_WINSOCK_API_PROTOTYPES
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
_WINSOCK_DEPRECATED_BY("WSAGetServiceClassInfoW()")
WINSOCK_API_LINKAGE
INT
WSAAPI
WSAGetServiceClassInfoA(
    _In_  LPGUID  lpProviderId,
    _In_  LPGUID  lpServiceClassId,
    _Inout_ LPDWORD  lpdwBufSize,
    _Out_writes_bytes_to_(*lpdwBufSize,*lpdwBufSize) LPWSASERVICECLASSINFOA lpServiceClassInfo
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

WINSOCK_API_LINKAGE
INT
WSAAPI
WSAGetServiceClassInfoW(
    _In_  LPGUID  lpProviderId,
    _In_  LPGUID  lpServiceClassId,
    _Inout_ LPDWORD  lpdwBufSize,
    _Out_writes_bytes_to_(*lpdwBufSize,*lpdwBufSize) LPWSASERVICECLASSINFOW lpServiceClassInfo
    );
#ifdef UNICODE
#define WSAGetServiceClassInfo  WSAGetServiceClassInfoW
#else
#define WSAGetServiceClassInfo  WSAGetServiceClassInfoA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
typedef
INT
(WSAAPI * LPFN_WSAGETSERVICECLASSINFOA)(
    _In_  LPGUID  lpProviderId,
    _In_  LPGUID  lpServiceClassId,
    _Inout_ LPDWORD  lpdwBufSize,
    _Out_writes_bytes_to_(*lpdwBufSize,*lpdwBufSize) LPWSASERVICECLASSINFOA lpServiceClassInfo
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

typedef
INT
(WSAAPI * LPFN_WSAGETSERVICECLASSINFOW)(
    _In_  LPGUID  lpProviderId,
    _In_  LPGUID  lpServiceClassId,
    _Inout_ LPDWORD  lpdwBufSize,
    _Out_writes_bytes_to_(*lpdwBufSize,*lpdwBufSize) LPWSASERVICECLASSINFOW lpServiceClassInfo
    );
#ifdef UNICODE
#define LPFN_WSAGETSERVICECLASSINFO  LPFN_WSAGETSERVICECLASSINFOW
#else
#define LPFN_WSAGETSERVICECLASSINFO  LPFN_WSAGETSERVICECLASSINFOA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("WSAEnumNameSpaceProvidersW()")
WINSOCK_API_LINKAGE
INT
WSAAPI
WSAEnumNameSpaceProvidersA(
    _Inout_ LPDWORD             lpdwBufferLength,
    _Out_writes_bytes_to_(*lpdwBufferLength,*lpdwBufferLength) LPWSANAMESPACE_INFOA lpnspBuffer
    );

WINSOCK_API_LINKAGE
INT
WSAAPI
WSAEnumNameSpaceProvidersW(
    _Inout_ LPDWORD             lpdwBufferLength,
    _Out_writes_bytes_to_(*lpdwBufferLength,*lpdwBufferLength) LPWSANAMESPACE_INFOW lpnspBuffer
    );

#ifdef UNICODE
#define WSAEnumNameSpaceProviders   WSAEnumNameSpaceProvidersW
#else
#define WSAEnumNameSpaceProviders   WSAEnumNameSpaceProvidersA
#endif /* !UNICODE */

#if(_WIN32_WINNT >= 0x0600 )
_WINSOCK_DEPRECATED_BY("WSAEnumNameSpaceProvidersW()")
WINSOCK_API_LINKAGE
INT
WSAAPI
WSAEnumNameSpaceProvidersExA(
    _Inout_ LPDWORD             lpdwBufferLength,
    _Out_writes_bytes_to_(*lpdwBufferLength,*lpdwBufferLength) LPWSANAMESPACE_INFOEXA lpnspBuffer
    );

WINSOCK_API_LINKAGE
INT
WSAAPI
WSAEnumNameSpaceProvidersExW(
    _Inout_ LPDWORD             lpdwBufferLength,
    _Out_writes_bytes_to_(*lpdwBufferLength,*lpdwBufferLength) LPWSANAMESPACE_INFOEXW lpnspBuffer
    );

#ifdef UNICODE
#define WSAEnumNameSpaceProvidersEx WSAEnumNameSpaceProvidersExW
#else
#define WSAEnumNameSpaceProvidersEx WSAEnumNameSpaceProvidersExA
#endif /* !UNICODE */

#endif //(_WIN32_WINNT >= 0x0600 )


#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
INT
(WSAAPI * LPFN_WSAENUMNAMESPACEPROVIDERSA)(
    _Inout_ LPDWORD              lpdwBufferLength,
    _Out_writes_bytes_to_(*lpdwBufferLength,*lpdwBufferLength) LPWSANAMESPACE_INFOA lpnspBuffer
    );

typedef
INT
(WSAAPI * LPFN_WSAENUMNAMESPACEPROVIDERSW)(
    _Inout_ LPDWORD              lpdwBufferLength,
    _Out_writes_bytes_to_(*lpdwBufferLength,*lpdwBufferLength) LPWSANAMESPACE_INFOW lpnspBuffer
    );
#ifdef UNICODE
#define LPFN_WSAENUMNAMESPACEPROVIDERS  LPFN_WSAENUMNAMESPACEPROVIDERSW
#else
#define LPFN_WSAENUMNAMESPACEPROVIDERS  LPFN_WSAENUMNAMESPACEPROVIDERSA
#endif /* !UNICODE */

#if (_WIN32_WINNT >= 0x0600)
typedef
INT
(WSAAPI * LPFN_WSAENUMNAMESPACEPROVIDERSEXA)(
    _Inout_ LPDWORD              lpdwBufferLength,
    _Out_writes_bytes_to_(*lpdwBufferLength,*lpdwBufferLength) LPWSANAMESPACE_INFOEXA lpnspBuffer
    );

typedef
INT
(WSAAPI * LPFN_WSAENUMNAMESPACEPROVIDERSEXW)(
    _Inout_ LPDWORD              lpdwBufferLength,
    _Out_writes_bytes_to_(*lpdwBufferLength,*lpdwBufferLength) LPWSANAMESPACE_INFOEXW lpnspBuffer
    );
#ifdef UNICODE
#define LPFN_WSAENUMNAMESPACEPROVIDERSEX  LPFN_WSAENUMNAMESPACEPROVIDERSEXW
#else
#define LPFN_WSAENUMNAMESPACEPROVIDERSEX  LPFN_WSAENUMNAMESPACEPROVIDERSEXA
#endif /* !UNICODE */

#endif //(_WIN32_WINNT >= 0x600)

#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
_WINSOCK_DEPRECATED_BY("WSAGetServiceClassNameByClassIdW()")
WINSOCK_API_LINKAGE
_Success_(return == 0) INT
WSAAPI
WSAGetServiceClassNameByClassIdA(
    _In_       LPGUID  lpServiceClassId,
    _Out_writes_bytes_to_(*lpdwBufferLength,*lpdwBufferLength) LPSTR lpszServiceClassName,
    _Inout_ LPDWORD lpdwBufferLength
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

WINSOCK_API_LINKAGE
_Success_(return == 0) INT
WSAAPI
WSAGetServiceClassNameByClassIdW(
    _In_       LPGUID  lpServiceClassId,
    _Out_writes_bytes_to_(*lpdwBufferLength,*lpdwBufferLength) LPWSTR lpszServiceClassName,
    _Inout_ LPDWORD lpdwBufferLength
    );
#ifdef UNICODE
#define WSAGetServiceClassNameByClassId  WSAGetServiceClassNameByClassIdW
#else
#define WSAGetServiceClassNameByClassId  WSAGetServiceClassNameByClassIdA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
typedef
INT
(WSAAPI * LPFN_WSAGETSERVICECLASSNAMEBYCLASSIDA)(
    _In_      LPGUID  lpServiceClassId,
    _Out_writes_bytes_to_(*lpdwBufferLength,*lpdwBufferLength) LPSTR lpszServiceClassName,
    _Inout_ LPDWORD lpdwBufferLength
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

typedef
INT
(WSAAPI * LPFN_WSAGETSERVICECLASSNAMEBYCLASSIDW)(
    _In_      LPGUID  lpServiceClassId,
    _Out_writes_bytes_to_(*lpdwBufferLength,*lpdwBufferLength) LPWSTR lpszServiceClassName,
    _Inout_ LPDWORD lpdwBufferLength
    );
#ifdef UNICODE
#define LPFN_WSAGETSERVICECLASSNAMEBYCLASSID  LPFN_WSAGETSERVICECLASSNAMEBYCLASSIDW
#else
#define LPFN_WSAGETSERVICECLASSNAMEBYCLASSID  LPFN_WSAGETSERVICECLASSNAMEBYCLASSIDA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
_WINSOCK_DEPRECATED_BY("WSASetServiceW()")
WINSOCK_API_LINKAGE
INT
WSAAPI
WSASetServiceA(
    _In_ LPWSAQUERYSETA lpqsRegInfo,
    _In_ WSAESETSERVICEOP essoperation,
    _In_ DWORD dwControlFlags
    );

WINSOCK_API_LINKAGE
INT
WSAAPI
WSASetServiceW(
    _In_ LPWSAQUERYSETW lpqsRegInfo,
    _In_ WSAESETSERVICEOP essoperation,
    _In_ DWORD dwControlFlags
    );
#ifdef UNICODE
#define WSASetService  WSASetServiceW
#else
#define WSASetService  WSASetServiceA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
INT
(WSAAPI * LPFN_WSASETSERVICEA)(
    _In_ LPWSAQUERYSETA lpqsRegInfo,
    _In_ WSAESETSERVICEOP essoperation,
    _In_ DWORD dwControlFlags
    );

typedef
INT
(WSAAPI * LPFN_WSASETSERVICEW)(
    _In_ LPWSAQUERYSETW lpqsRegInfo,
    _In_ WSAESETSERVICEOP essoperation,
    _In_ DWORD dwControlFlags
    );
#ifdef UNICODE
#define LPFN_WSASETSERVICE  LPFN_WSASETSERVICEW
#else
#define LPFN_WSASETSERVICE  LPFN_WSASETSERVICEA
#endif /* !UNICODE */
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
INT
WSAAPI
WSAProviderConfigChange(
    _Inout_ LPHANDLE lpNotificationHandle,
    _Inout_opt_ LPWSAOVERLAPPED lpOverlapped,
    _In_opt_ LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */

#if INCL_WINSOCK_API_TYPEDEFS
typedef
INT
(WSAAPI * LPFN_WSAPROVIDERCONFIGCHANGE)(
    _Inout_ LPHANDLE lpNotificationHandle,
    _Inout_opt_ LPWSAOVERLAPPED lpOverlapped,
    _In_opt_ LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine
    );
#endif /* INCL_WINSOCK_API_TYPEDEFS */

#if(_WIN32_WINNT >= 0x0600)
#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
int
WSAAPI
WSAPoll(
    _Inout_ LPWSAPOLLFD fdArray,
    _In_ ULONG fds,
    _In_ INT timeout
    );
#endif /* INCL_WINSOCK_API_PROTOTYPES */
#endif // (_WIN32_WINNT >= 0x0600)


typedef struct SOCK_NOTIFY_REGISTRATION {
    SOCKET socket;
    PVOID completionKey;
    UINT16 eventFilter;
    UINT8 operation;
    UINT8 triggerFlags;
    DWORD registrationResult;
} SOCK_NOTIFY_REGISTRATION;

//
// Socket notification registration events. Supplied during registration.
// Indicates that a notification should be issued for the event if its
// condition holds.
//
#define SOCK_NOTIFY_REGISTER_EVENT_NONE     0x00
#define SOCK_NOTIFY_REGISTER_EVENT_IN       0x01 // Input is available from the socket without blocking.
#define SOCK_NOTIFY_REGISTER_EVENT_OUT      0x02 // Output can be provided to the socket without blocking.
#define SOCK_NOTIFY_REGISTER_EVENT_HANGUP   0x04 // The socket connection has been terminated.

#define SOCK_NOTIFY_REGISTER_EVENTS_ALL \
    (SOCK_NOTIFY_REGISTER_EVENT_IN | SOCK_NOTIFY_REGISTER_EVENT_OUT | SOCK_NOTIFY_REGISTER_EVENT_HANGUP)

//
// Socket notification events. These are the events possible when a notification
// is received.
//
// The SOCK_NOTIFY_EVENT_ERR and SOCK_NOTIFY_EVENT_REMOVE events
// may be indicated regardless of registration.
//
// If a SOCK_NOTIFY_EVENT_REMOVE event is indicated, no more notifications will
// be provided.
//
#define SOCK_NOTIFY_EVENT_IN        SOCK_NOTIFY_REGISTER_EVENT_IN     // Input is available from the socket without blocking.
#define SOCK_NOTIFY_EVENT_OUT       SOCK_NOTIFY_REGISTER_EVENT_OUT    // Output can be provided to the socket without blocking.
#define SOCK_NOTIFY_EVENT_HANGUP    SOCK_NOTIFY_REGISTER_EVENT_HANGUP // The socket connection has been terminated.
#define SOCK_NOTIFY_EVENT_ERR       0x40                              // The socket is in an error state.
#define SOCK_NOTIFY_EVENT_REMOVE    0x80                              // The notification has been deregistered.

#define SOCK_NOTIFY_EVENTS_ALL \
    (SOCK_NOTIFY_REGISTER_EVENTS_ALL | SOCK_NOTIFY_EVENT_ERR | SOCK_NOTIFY_EVENT_REMOVE)

//
// Socket notification registration operations. One operation must be supplied at
// a time.
//
// A SOCK_NOTIFY_OP_DISABLE operation will not destroy the underlying structures.
//
// A SOCK_NOTIFY_OP_REMOVE operation will cause a SOCK_NOTIFY_REMOVE notification
// to be delivered when the operation completes successfully.
//
#define SOCK_NOTIFY_OP_NONE         0x00
#define SOCK_NOTIFY_OP_ENABLE       0x01 // Enables the registration.
#define SOCK_NOTIFY_OP_DISABLE      0x02 // Disables the registration.
#define SOCK_NOTIFY_OP_REMOVE       0x04 // Removes the registration.

//
// Socket notification trigger behaviors.
//
// When operation is SOCK_NOTIFY_OP_ENABLE:
//    - One of SOCK_NOTIFY_TRIGGER_PERSISTENT or SOCK_NOTIFY_TRIGGER_ONESHOT must be supplied
//    - One of SOCK_NOTIFY_TRIGGER_LEVEL or SOCK_NOTIFY_TRIGGER_EDGE must be supplied
//
// SOCK_NOTIFY_TRIGGER_PERSISTENT is not compatible with SOCK_NOTIFY_TRIGGER_ONESHOT.
// SOCK_NOTIFY_TRIGGER_LEVEL is not compatible with SOCK_NOTIFY_TRIGGER_EDGE.
//
#define SOCK_NOTIFY_TRIGGER_ONESHOT    0x01 // The registration will be disabled (not removed) upon delivery of the next notification.
#define SOCK_NOTIFY_TRIGGER_PERSISTENT 0x02 // The registration will remain active until it is explicitly disabled or removed.
#define SOCK_NOTIFY_TRIGGER_LEVEL      0x04 // The registration is for level-triggered notifications.
#define SOCK_NOTIFY_TRIGGER_EDGE       0x08 // The registration is for edge-triggered notifications.

#define SOCK_NOTIFY_TRIGGER_ALL \
    (SOCK_NOTIFY_TRIGGER_ONESHOT | SOCK_NOTIFY_TRIGGER_PERSISTENT | SOCK_NOTIFY_TRIGGER_LEVEL | SOCK_NOTIFY_TRIGGER_EDGE)

#if (NTDDI_VERSION >= NTDDI_WIN10_MN)
#if INCL_WINSOCK_API_PROTOTYPES
WINSOCK_API_LINKAGE
DWORD
WSAAPI
ProcessSocketNotifications(
    _In_ HANDLE completionPort,
    _In_ UINT32 registrationCount,
    _Inout_updates_opt_(registrationCount)
        SOCK_NOTIFY_REGISTRATION* registrationInfos,
    _In_ UINT32 timeoutMs,
    _In_ ULONG completionCount,
    _Out_writes_to_opt_(completionCount, *receivedEntryCount)
        OVERLAPPED_ENTRY* completionPortEntries,
    _Out_opt_ UINT32* receivedEntryCount
    );

#if !defined(__midl)

inline
UINT32
SocketNotificationRetrieveEvents (
    _In_ OVERLAPPED_ENTRY* notification
    )
{
    return (UINT32)notification->dwNumberOfBytesTransferred;
}

#endif // __midl

#endif // INCL_WINSOCK_API_PROTOTYPES
#endif // (NTDDI_VERSION >= NTDDI_WIN10_MN)


/* Microsoft Windows Extended data types */
typedef struct sockaddr_in FAR *LPSOCKADDR_IN;

typedef struct linger LINGER;
typedef struct linger *PLINGER;
typedef struct linger FAR *LPLINGER;

typedef struct fd_set FD_SET;
typedef struct fd_set *PFD_SET;
typedef struct fd_set FAR *LPFD_SET;

typedef struct hostent HOSTENT;
typedef struct hostent *PHOSTENT;
typedef struct hostent FAR *LPHOSTENT;

typedef struct servent SERVENT;
typedef struct servent *PSERVENT;
typedef struct servent FAR *LPSERVENT;

typedef struct protoent PROTOENT;
typedef struct protoent *PPROTOENT;
typedef struct protoent FAR *LPPROTOENT;

typedef struct timeval TIMEVAL;
typedef struct timeval *PTIMEVAL;
typedef struct timeval FAR *LPTIMEVAL;

/*
 * Windows message parameter composition and decomposition
 * macros.
 *
 * WSAMAKEASYNCREPLY is intended for use by the Windows Sockets implementation
 * when constructing the response to a WSAAsyncGetXByY() routine.
 */
#define WSAMAKEASYNCREPLY(buflen,error)     MAKELONG(buflen,error)
/*
 * WSAMAKESELECTREPLY is intended for use by the Windows Sockets implementation
 * when constructing the response to WSAAsyncSelect().
 */
#define WSAMAKESELECTREPLY(event,error)     MAKELONG(event,error)
/*
 * WSAGETASYNCBUFLEN is intended for use by the Windows Sockets application
 * to extract the buffer length from the lParam in the response
 * to a WSAAsyncGetXByY().
 */
#define WSAGETASYNCBUFLEN(lParam)           LOWORD(lParam)
/*
 * WSAGETASYNCERROR is intended for use by the Windows Sockets application
 * to extract the error code from the lParam in the response
 * to a WSAGetXByY().
 */
#define WSAGETASYNCERROR(lParam)            HIWORD(lParam)
/*
 * WSAGETSELECTEVENT is intended for use by the Windows Sockets application
 * to extract the event code from the lParam in the response
 * to a WSAAsyncSelect().
 */
#define WSAGETSELECTEVENT(lParam)           LOWORD(lParam)
/*
 * WSAGETSELECTERROR is intended for use by the Windows Sockets application
 * to extract the error code from the lParam in the response
 * to a WSAAsyncSelect().
 */
#define WSAGETSELECTERROR(lParam)           HIWORD(lParam)

#ifdef __cplusplus
}
#endif

#ifdef _NEED_POPPACK
#include <poppack.h>
#endif

#if(_WIN32_WINNT >= 0x0501)
#ifdef IPV6STRICT
#include <wsipv6ok.h>
#endif // IPV6STRICT
#endif //(_WIN32_WINNT >= 0x0501)

#endif  /* _WINSOCK2API_ */

