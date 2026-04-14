/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    mswsock.h

Abstract:

    This module contains the Microsoft-specific extensions to the Windows
    Sockets API.

Revision History:

--*/

#ifndef _MSWSOCK_
#define _MSWSOCK_

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

#include <mswsockdef.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

/*
 * Options for connect and disconnect data and options.  Used only by
 * non-TCP/IP transports such as DECNet, OSI TP4, etc.
 */
#define SO_CONNDATA                 0x7000
#define SO_CONNOPT                  0x7001
#define SO_DISCDATA                 0x7002
#define SO_DISCOPT                  0x7003
#define SO_CONNDATALEN              0x7004
#define SO_CONNOPTLEN               0x7005
#define SO_DISCDATALEN              0x7006
#define SO_DISCOPTLEN               0x7007

/*
 * Option for opening sockets for synchronous access.
 */
#define SO_OPENTYPE                 0x7008

#define SO_SYNCHRONOUS_ALERT        0x10
#define SO_SYNCHRONOUS_NONALERT     0x20

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

/*
 * Other NT-specific options.
 */
#define SO_MAXDG                    0x7009
#define SO_MAXPATHDG                0x700A
#define SO_UPDATE_ACCEPT_CONTEXT    0x700B
#define SO_CONNECT_TIME             0x700C
#if(_WIN32_WINNT >= 0x0501)
#define SO_UPDATE_CONNECT_CONTEXT   0x7010
#endif //(_WIN32_WINNT >= 0x0501)

/*
 * TCP options.
 */
#define TCP_BSDURGENT               0x7000

/*
 * MS Transport Provider IOCTL to control
 * reporting PORT_UNREACHABLE messages
 * on UDP sockets via recv/WSARecv/etc.
 * Pass TRUE in input buffer to enable (default if supported),
 * FALSE to disable.
 */
#define SIO_UDP_CONNRESET           _WSAIOW(IOC_VENDOR,12)

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#if((_WIN32_WINNT < 0x0600) && (_WIN32_WINNT >= 0x0501))

/*
 * MS Transport Provider IOCTL to request
 * notification when a given socket is closed.
 * Input buffer must be a pointer to the socket handle.
 * Input buffer size must be exactly sizeof(HANDLE).
 * Output buffer and output buffer length must be
 * NULL and 0 respectively. This IOCTL must always
 * be issued with an overlapped structure.
 *
 * This Ioctl code is available only on WinXP SP2 and Win2k3 SP1.
 */
#define SIO_SOCKET_CLOSE_NOTIFY     _WSAIOW(IOC_VENDOR,13)

#endif //(_WIN32_WINNT < 0x0600 && _WIN32_WINNT >= 0x0501)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

/*
 * MS Transport Provider IOCTL to control
 * reporting NET_UNREACHABLE (TTL expired) messages
 * on UDP sockets via recv/WSARecv/Etc.
 * Pass TRUE in input buffer to enabled (default if supported),
 * FALSE to disable.
 */
#define SIO_UDP_NETRESET            _WSAIOW(IOC_VENDOR,15)

/*
 * Microsoft extended APIs.
 */

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
_WINSOCK_DEPRECATED_BY("WSARecv()")
#if(_WIN32_WINNT < 0x0600)
int
PASCAL FAR
WSARecvEx(
    _In_ SOCKET s,
    _Out_writes_bytes_to_(len, return) char FAR *buf,
    _In_ int len,
    _Inout_ int FAR *flags
    );
#else //(_WIN32_WINNT < 0x0600)
INT
PASCAL FAR
WSARecvEx(
    _In_ SOCKET s,
    _Out_writes_bytes_to_(len, return) CHAR FAR *buf,
    _In_ INT len,
    _Inout_ INT FAR *flags
    );
#endif //(_WIN32_WINNT < 0x0600)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

typedef struct _TRANSMIT_FILE_BUFFERS {
    LPVOID Head;
    DWORD HeadLength;
    LPVOID Tail;
    DWORD TailLength;
} TRANSMIT_FILE_BUFFERS, *PTRANSMIT_FILE_BUFFERS, FAR *LPTRANSMIT_FILE_BUFFERS;

#define TF_DISCONNECT       0x01
#define TF_REUSE_SOCKET     0x02
#define TF_WRITE_BEHIND     0x04
#define TF_USE_DEFAULT_WORKER 0x00
#define TF_USE_SYSTEM_THREAD  0x10
#define TF_USE_KERNEL_APC     0x20

BOOL
PASCAL FAR
TransmitFile (
    _In_ SOCKET hSocket,
    _In_ HANDLE hFile,
    _In_ DWORD nNumberOfBytesToWrite,
    _In_ DWORD nNumberOfBytesPerSend,
    _Inout_opt_ LPOVERLAPPED lpOverlapped,
    _In_opt_ LPTRANSMIT_FILE_BUFFERS lpTransmitBuffers,
    _In_  DWORD dwReserved
    );

BOOL
PASCAL FAR
AcceptEx (
    _In_ SOCKET sListenSocket,
    _In_ SOCKET sAcceptSocket,
    _Out_writes_bytes_(dwReceiveDataLength+dwLocalAddressLength+dwRemoteAddressLength) PVOID lpOutputBuffer,
    _In_ DWORD dwReceiveDataLength,
    _In_ DWORD dwLocalAddressLength,
    _In_ DWORD dwRemoteAddressLength,
    _Out_ LPDWORD lpdwBytesReceived,
    _Inout_ LPOVERLAPPED lpOverlapped
    );

VOID
PASCAL FAR
GetAcceptExSockaddrs (
    _In_reads_bytes_(dwReceiveDataLength+dwLocalAddressLength+dwRemoteAddressLength) PVOID lpOutputBuffer,
    _In_ DWORD dwReceiveDataLength,
    _In_ DWORD dwLocalAddressLength,
    _In_ DWORD dwRemoteAddressLength,
    _Outptr_result_bytebuffer_(*LocalSockaddrLength) struct sockaddr **LocalSockaddr,
    _Out_ LPINT LocalSockaddrLength,
    _Outptr_result_bytebuffer_(*RemoteSockaddrLength) struct sockaddr **RemoteSockaddr,
    _Out_ LPINT RemoteSockaddrLength
    );

/*
 * "QueryInterface" versions of the above APIs.
 */

typedef
BOOL
(PASCAL FAR * LPFN_TRANSMITFILE)(
    _In_ SOCKET hSocket,
    _In_ HANDLE hFile,
    _In_ DWORD nNumberOfBytesToWrite,
    _In_ DWORD nNumberOfBytesPerSend,
    _Inout_opt_ LPOVERLAPPED lpOverlapped,
    _In_opt_ LPTRANSMIT_FILE_BUFFERS lpTransmitBuffers,
    _In_ DWORD dwReserved
    );

#define WSAID_TRANSMITFILE \
        {0xb5367df0,0xcbac,0x11cf,{0x95,0xca,0x00,0x80,0x5f,0x48,0xa1,0x92}}

typedef
BOOL
(PASCAL FAR * LPFN_ACCEPTEX)(
    _In_ SOCKET sListenSocket,
    _In_ SOCKET sAcceptSocket,
    _Out_writes_bytes_(dwReceiveDataLength+dwLocalAddressLength+dwRemoteAddressLength) PVOID lpOutputBuffer,
    _In_ DWORD dwReceiveDataLength,
    _In_ DWORD dwLocalAddressLength,
    _In_ DWORD dwRemoteAddressLength,
    _Out_ LPDWORD lpdwBytesReceived,
    _Inout_ LPOVERLAPPED lpOverlapped
    );

#define WSAID_ACCEPTEX \
        {0xb5367df1,0xcbac,0x11cf,{0x95,0xca,0x00,0x80,0x5f,0x48,0xa1,0x92}}

typedef
VOID
(PASCAL FAR * LPFN_GETACCEPTEXSOCKADDRS)(
    _In_reads_bytes_(dwReceiveDataLength+dwLocalAddressLength+dwRemoteAddressLength) PVOID lpOutputBuffer,
    _In_ DWORD dwReceiveDataLength,
    _In_ DWORD dwLocalAddressLength,
    _In_ DWORD dwRemoteAddressLength,
    _Outptr_result_bytebuffer_(*LocalSockaddrLength) struct sockaddr **LocalSockaddr,
    _Out_ LPINT LocalSockaddrLength,
    _Outptr_result_bytebuffer_(*RemoteSockaddrLength) struct sockaddr **RemoteSockaddr,
    _Out_ LPINT RemoteSockaddrLength
    );

#define WSAID_GETACCEPTEXSOCKADDRS \
        {0xb5367df2,0xcbac,0x11cf,{0x95,0xca,0x00,0x80,0x5f,0x48,0xa1,0x92}}

#if(_WIN32_WINNT >= 0x0501)

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201) /* Nonstandard extension, nameless struct/union */

typedef struct _TRANSMIT_PACKETS_ELEMENT {
    ULONG dwElFlags;
#define TP_ELEMENT_MEMORY   1
#define TP_ELEMENT_FILE     2
#define TP_ELEMENT_EOP      4
    ULONG cLength;
    union {
        struct {
            LARGE_INTEGER nFileOffset;
            HANDLE        hFile;
        };
        PVOID             pBuffer;
    };
} TRANSMIT_PACKETS_ELEMENT, *PTRANSMIT_PACKETS_ELEMENT, FAR *LPTRANSMIT_PACKETS_ELEMENT;

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201)
#endif
#define TP_DISCONNECT           TF_DISCONNECT
#define TP_REUSE_SOCKET         TF_REUSE_SOCKET
#define TP_USE_DEFAULT_WORKER   TF_USE_DEFAULT_WORKER
#define TP_USE_SYSTEM_THREAD    TF_USE_SYSTEM_THREAD
#define TP_USE_KERNEL_APC       TF_USE_KERNEL_APC

typedef
BOOL
(PASCAL FAR * LPFN_TRANSMITPACKETS) (
    _In_ SOCKET hSocket,
    _In_opt_ LPTRANSMIT_PACKETS_ELEMENT lpPacketArray,
    _In_ DWORD nElementCount,
    _In_ DWORD nSendSize,
    _Inout_opt_ LPOVERLAPPED lpOverlapped,
    _In_ DWORD dwFlags
    );

#define WSAID_TRANSMITPACKETS \
    {0xd9689da0,0x1f90,0x11d3,{0x99,0x71,0x00,0xc0,0x4f,0x68,0xc8,0x76}}

typedef
BOOL
(PASCAL FAR * LPFN_CONNECTEX) (
    _In_ SOCKET s,
    _In_reads_bytes_(namelen) const struct sockaddr FAR *name,
    _In_ int namelen,
    _In_reads_bytes_opt_(dwSendDataLength) PVOID lpSendBuffer,
    _In_ DWORD dwSendDataLength,
    _Out_ LPDWORD lpdwBytesSent,
    _Inout_ LPOVERLAPPED lpOverlapped
    );

#define WSAID_CONNECTEX \
    {0x25a207b9,0xddf3,0x4660,{0x8e,0xe9,0x76,0xe5,0x8c,0x74,0x06,0x3e}}

typedef
BOOL
(PASCAL FAR * LPFN_DISCONNECTEX) (
    _In_ SOCKET s,
    _Inout_opt_ LPOVERLAPPED lpOverlapped,
    _In_ DWORD  dwFlags,
    _In_ DWORD  dwReserved
    );

#define WSAID_DISCONNECTEX \
    {0x7fda2e11,0x8630,0x436f,{0xa0, 0x31, 0xf5, 0x36, 0xa6, 0xee, 0xc1, 0x57}}

#define DE_REUSE_SOCKET TF_REUSE_SOCKET

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

/*
 * Network-location awareness -- Name registration values for use
 * with WSASetService and other structures.
 */

// {6642243A-3BA8-4aa6-BAA5-2E0BD71FDD83}
#define NLA_NAMESPACE_GUID \
    {0x6642243a,0x3ba8,0x4aa6,{0xba,0xa5,0x2e,0xb,0xd7,0x1f,0xdd,0x83}}

// {6642243A-3BA8-4aa6-BAA5-2E0BD71FDD83}
#define NLA_SERVICE_CLASS_GUID \
    {0x37e515,0xb5c9,0x4a43,{0xba,0xda,0x8b,0x48,0xa8,0x7a,0xd2,0x39}}

#define NLA_ALLUSERS_NETWORK   0x00000001
#define NLA_FRIENDLY_NAME      0x00000002

typedef enum _NLA_BLOB_DATA_TYPE {
    NLA_RAW_DATA          = 0,
    NLA_INTERFACE         = 1,
    NLA_802_1X_LOCATION   = 2,
    NLA_CONNECTIVITY      = 3,
    NLA_ICS               = 4,
} NLA_BLOB_DATA_TYPE, *PNLA_BLOB_DATA_TYPE;

typedef enum _NLA_CONNECTIVITY_TYPE {
    NLA_NETWORK_AD_HOC    = 0,
    NLA_NETWORK_MANAGED   = 1,
    NLA_NETWORK_UNMANAGED = 2,
    NLA_NETWORK_UNKNOWN   = 3,
} NLA_CONNECTIVITY_TYPE, *PNLA_CONNECTIVITY_TYPE;

typedef enum _NLA_INTERNET {
    NLA_INTERNET_UNKNOWN  = 0,
    NLA_INTERNET_NO       = 1,
    NLA_INTERNET_YES      = 2,
} NLA_INTERNET, *PNLA_INTERNET;

typedef struct _NLA_BLOB {

    struct {
        NLA_BLOB_DATA_TYPE type;
        DWORD dwSize;
        DWORD nextOffset;
    } header;

    union {

        // header.type -> NLA_RAW_DATA
        CHAR rawData[1];

        // header.type -> NLA_INTERFACE
        struct {
            DWORD dwType;
            DWORD dwSpeed;
            CHAR adapterName[1];
        } interfaceData;

        // header.type -> NLA_802_1X_LOCATION
        struct {
            CHAR information[1];
        } locationData;

        // header.type -> NLA_CONNECTIVITY
        struct {
            NLA_CONNECTIVITY_TYPE type;
            NLA_INTERNET internet;
        } connectivity;

        // header.type -> NLA_ICS
        struct {
            struct {
                DWORD speed;
                DWORD type;
                DWORD state;
                WCHAR machineName[256];
                WCHAR sharedAdapterName[256];
            } remote;
        } ICS;

    } data;

} NLA_BLOB, *PNLA_BLOB, * FAR LPNLA_BLOB;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

/*
 * WSARecvMsg -- support for receiving ancilliary
 * data/control information with a message.
 */
typedef
INT
(PASCAL FAR * LPFN_WSARECVMSG) (
    _In_ SOCKET s,
    _Inout_ LPWSAMSG lpMsg,
    _Out_opt_ LPDWORD lpdwNumberOfBytesRecvd,
    _Inout_opt_ LPWSAOVERLAPPED lpOverlapped,
    _In_opt_ LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine
    );

#define WSAID_WSARECVMSG \
    {0xf689d7c8,0x6f1f,0x436b,{0x8a,0x53,0xe5,0x4f,0xe3,0x51,0xc3,0x22}}

#endif //(_WIN32_WINNT >= 0x0501)

#if(_WIN32_WINNT >= 0x0600)

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

/*
 * Ioctl codes for translating socket handles to the base provider handle.
 * This is performed to prevent breaking non-IFS LSPs when new Winsock extension
 * funtions are added.
 */
#define SIO_BSP_HANDLE          _WSAIOR(IOC_WS2,27)
#define SIO_BSP_HANDLE_SELECT   _WSAIOR(IOC_WS2,28)
#define SIO_BSP_HANDLE_POLL     _WSAIOR(IOC_WS2,29)

/*
 * Ioctl code used to translate a socket handle into the base provider's handle.
 * This is not used by any Winsock extension function and should not be intercepted
 * by Winsock LSPs.
 */
#define SIO_BASE_HANDLE         _WSAIOR(IOC_WS2,34)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

/*
 * Ioctl codes for Winsock extension functions.
 */
#define SIO_EXT_SELECT          _WSAIORW(IOC_WS2,30)
#define SIO_EXT_POLL            _WSAIORW(IOC_WS2,31)
#define SIO_EXT_SENDMSG         _WSAIORW(IOC_WS2,32)

#pragma warning(push)
#pragma warning(disable:4200) /* zero-sized array in struct/union */

/*
 * Data structure for passing WSAPoll arugments through WSAIoctl
 */
typedef struct {
    int result;
    ULONG fds;
    INT timeout;
    WSAPOLLFD fdArray[0];
} WSAPOLLDATA, *LPWSAPOLLDATA;

#pragma warning(pop)

/*
 * Data structure for passing WSASendMsg arguments through WSAIoctl
 */
typedef struct {
    LPWSAMSG lpMsg;
    DWORD dwFlags;
    LPDWORD lpNumberOfBytesSent;
    LPWSAOVERLAPPED lpOverlapped;
    LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine;
} WSASENDMSG, *LPWSASENDMSG;


/*
 * WSASendMsg -- send data to a specific destination, with options, using
 *    overlapped I/O where applicable.
 *
 * Valid flags for dwFlags parameter:
 *    MSG_DONTROUTE
 *    MSG_PARTIAL (a.k.a. MSG_EOR) (only for non-stream sockets)
 *    MSG_OOB (only for stream style sockets) (NYI)
 *
 * Caller must provide either lpOverlapped or lpCompletionRoutine
 * or neither (both NULL).
 */
typedef
INT
(PASCAL FAR * LPFN_WSASENDMSG) (
    _In_ SOCKET s,
    _In_ LPWSAMSG lpMsg,
    _In_ DWORD dwFlags,
    _Out_opt_ LPDWORD lpNumberOfBytesSent,
    _Inout_opt_ LPWSAOVERLAPPED lpOverlapped OPTIONAL,
    _In_opt_ LPWSAOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine OPTIONAL
    );

#define WSAID_WSASENDMSG /* a441e712-754f-43ca-84a7-0dee44cf606d */ \
    {0xa441e712,0x754f,0x43ca,{0x84,0xa7,0x0d,0xee,0x44,0xcf,0x60,0x6d}}

//
// WSAPoll
//
typedef
INT
(WSAAPI *LPFN_WSAPOLL)(
    _Inout_ LPWSAPOLLFD fdarray,
    _In_ ULONG nfds,
    _In_ INT timeout
    );

#define WSAID_WSAPOLL \
        {0x18C76F85,0xDC66,0x4964,{0x97,0x2E,0x23,0xC2,0x72,0x38,0x31,0x2B}}

#endif //(_WIN32_WINNT >= 0x0600)

typedef BOOL (PASCAL FAR * LPFN_RIORECEIVE)(
    _In_ RIO_RQ SocketQueue,
    _In_reads_(DataBufferCount) PRIO_BUF pData,
    _In_ ULONG DataBufferCount,
    _In_ DWORD Flags,
    _In_opt_ PVOID RequestContext
    );

typedef int (PASCAL FAR * LPFN_RIORECEIVEEX)(
    _In_ RIO_RQ SocketQueue,
    _In_reads_(DataBufferCount) PRIO_BUF pData,
    _In_ ULONG DataBufferCount,
    _In_opt_ PRIO_BUF pLocalAddress,
    _In_opt_ PRIO_BUF pRemoteAddress,
    _In_opt_ PRIO_BUF pControlContext,
    _In_opt_ PRIO_BUF pFlags,
    _In_ DWORD Flags,
    _In_opt_ PVOID RequestContext
);

typedef BOOL (PASCAL FAR * LPFN_RIOSEND)(
    _In_ RIO_RQ SocketQueue,
    _In_reads_(DataBufferCount) PRIO_BUF pData,
    _In_ ULONG DataBufferCount,
    _In_ DWORD Flags,
    _In_opt_ PVOID RequestContext
);

typedef BOOL (PASCAL FAR * LPFN_RIOSENDEX)(
    _In_ RIO_RQ SocketQueue,
    _In_reads_(DataBufferCount) PRIO_BUF pData,
    _In_ ULONG DataBufferCount,
    _In_opt_ PRIO_BUF pLocalAddress,
    _In_opt_ PRIO_BUF pRemoteAddress,
    _In_opt_ PRIO_BUF pControlContext,
    _In_opt_ PRIO_BUF pFlags,
    _In_ DWORD Flags,
    _In_opt_ PVOID RequestContext
);

typedef VOID (PASCAL FAR * LPFN_RIOCLOSECOMPLETIONQUEUE)(
    _In_ RIO_CQ CQ
);

typedef enum _RIO_NOTIFICATION_COMPLETION_TYPE {
    RIO_EVENT_COMPLETION      = 1,
    RIO_IOCP_COMPLETION       = 2,
} RIO_NOTIFICATION_COMPLETION_TYPE, *PRIO_NOTIFICATION_COMPLETION_TYPE;

#pragma warning(push)
#pragma warning(disable : 4201) /* Nonstandard extension: nameless struct/union */

typedef struct _RIO_NOTIFICATION_COMPLETION {
    RIO_NOTIFICATION_COMPLETION_TYPE Type;
    union {
        struct {
            HANDLE EventHandle;
            BOOL NotifyReset;
        } Event;
        struct {
            HANDLE IocpHandle;
            PVOID CompletionKey;
            PVOID Overlapped;
        } Iocp;
    };
} RIO_NOTIFICATION_COMPLETION, *PRIO_NOTIFICATION_COMPLETION;

#pragma warning(pop)

typedef RIO_CQ (PASCAL FAR * LPFN_RIOCREATECOMPLETIONQUEUE)(
    _In_ DWORD QueueSize,
    _In_opt_ PRIO_NOTIFICATION_COMPLETION NotificationCompletion
);

typedef RIO_RQ (PASCAL FAR * LPFN_RIOCREATEREQUESTQUEUE)(
    _In_ SOCKET Socket,
    _In_ ULONG MaxOutstandingReceive,
    _In_ ULONG MaxReceiveDataBuffers,
    _In_ ULONG MaxOutstandingSend,
    _In_ ULONG MaxSendDataBuffers,
    _In_ RIO_CQ ReceiveCQ,
    _In_ RIO_CQ SendCQ,
    _In_opt_ PVOID SocketContext
);

typedef ULONG (PASCAL FAR * LPFN_RIODEQUEUECOMPLETION)(
    _In_ RIO_CQ CQ,
    _Out_writes_to_(ArraySize, return) PRIORESULT Array,
    _In_ ULONG ArraySize
);

typedef VOID (PASCAL FAR * LPFN_RIODEREGISTERBUFFER)(
    _In_ RIO_BUFFERID BufferId
);

typedef INT (PASCAL FAR * LPFN_RIONOTIFY)(
    _In_ RIO_CQ CQ
);

typedef RIO_BUFFERID (PASCAL FAR * LPFN_RIOREGISTERBUFFER)(
    _In_ PCHAR DataBuffer,
    _In_ DWORD DataLength
);

typedef BOOL (PASCAL FAR * LPFN_RIORESIZECOMPLETIONQUEUE) (
    _In_ RIO_CQ CQ,
    _In_ DWORD QueueSize
);

typedef BOOL (PASCAL FAR * LPFN_RIORESIZEREQUESTQUEUE) (
    _In_ RIO_RQ RQ,
    _In_ DWORD MaxOutstandingReceive,
    _In_ DWORD MaxOutstandingSend
);

typedef struct _RIO_EXTENSION_FUNCTION_TABLE {
    DWORD cbSize;

    LPFN_RIORECEIVE RIOReceive;
    LPFN_RIORECEIVEEX RIOReceiveEx;
    LPFN_RIOSEND RIOSend;
    LPFN_RIOSENDEX RIOSendEx;
    LPFN_RIOCLOSECOMPLETIONQUEUE RIOCloseCompletionQueue;
    LPFN_RIOCREATECOMPLETIONQUEUE RIOCreateCompletionQueue;
    LPFN_RIOCREATEREQUESTQUEUE RIOCreateRequestQueue;
    LPFN_RIODEQUEUECOMPLETION RIODequeueCompletion;
    LPFN_RIODEREGISTERBUFFER RIODeregisterBuffer;
    LPFN_RIONOTIFY RIONotify;
    LPFN_RIOREGISTERBUFFER RIORegisterBuffer;
    LPFN_RIORESIZECOMPLETIONQUEUE RIOResizeCompletionQueue;
    LPFN_RIORESIZEREQUESTQUEUE RIOResizeRequestQueue;
} RIO_EXTENSION_FUNCTION_TABLE, *PRIO_EXTENSION_FUNCTION_TABLE;

#define WSAID_MULTIPLE_RIO /* 8509e081-96dd-4005-b165-9e2ee8c79e3f */ \
    {0x8509e081,0x96dd,0x4005,{0xb1,0x65,0x9e,0x2e,0xe8,0xc7,0x9e,0x3f}}

#ifdef __cplusplus
}
#endif

#endif  /* _MSWSOCK_ */
