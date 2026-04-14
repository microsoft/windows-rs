/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    mstcpip.h

Abstract:

    This module contains Microsoft-specific extensions to the core
    Winsock definitions.

Environment:

    user mode or kernel mode

--*/

#ifndef _MSTCPIP_
#define _MSTCPIP_

#if (NTDDI_VERSION >= NTDDI_WIN7)
// GUID definition for use with Secure Sockets API
// aec2ef9c-3a4d-4d3e-8842-239942e39a47
DEFINE_GUID(
   SOCKET_DEFAULT2_QM_POLICY,
   0xaec2ef9c,
   0x3a4d,
   0x4d3e,
   0x88, 0x42, 0x23, 0x99, 0x42, 0xe3, 0x9a, 0x47
);
// GUID definition for use with Real Time Notification setting API.
// 6b59819a-5cae-492d-a901-2a3c2c50164f
DEFINE_GUID(
   REAL_TIME_NOTIFICATION_CAPABILITY,
   0x6b59819a,
   0x5cae,
   0x492d,
   0xa9, 0x01, 0x2a, 0x3c, 0x2c, 0x50, 0x16, 0x4f
);
// GUID definition for use with Real Time Notification setting API.
// 6843da03-154a-4616-a508-44371295f96b
DEFINE_GUID(
   REAL_TIME_NOTIFICATION_CAPABILITY_EX,
   0x6843da03,
   0x154a,
   0x4616,
   0xa5, 0x08, 0x44, 0x37, 0x12, 0x95, 0xf9, 0x6b
);
// GUID definition for use with Associate Namres Context API.
// {59A38B67-D4FE-46E1-BA3C-87EA74CA3049}
DEFINE_GUID(
   ASSOCIATE_NAMERES_CONTEXT,
   0x59a38b67,
   0xd4fe,
   0x46e1,
   0xba, 0x3c, 0x87, 0xea, 0x74, 0xca, 0x30, 0x49
);
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#ifndef GUID_DEFS_ONLY
#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma warning(push)
#pragma warning(disable:4200)
#pragma warning(disable:4201)

#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable: 24002, "Requires explicit use of IPv4 address types.")
#endif

#ifndef ASSERT
#define MSTCPIP_ASSERT_UNDEFINED
#define ASSERT(exp) ((VOID) 0)
#endif

#ifdef _MSC_VER
#define MSTCPIP_INLINE __inline
#else
#define MSTCPIP_INLINE extern inline /* GNU style */
#endif

#include <nldef.h>

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
typedef enum _TCPSTATE {
    TCPSTATE_CLOSED,
    TCPSTATE_LISTEN,
    TCPSTATE_SYN_SENT,
    TCPSTATE_SYN_RCVD,
    TCPSTATE_ESTABLISHED,
    TCPSTATE_FIN_WAIT_1,
    TCPSTATE_FIN_WAIT_2,
    TCPSTATE_CLOSE_WAIT,
    TCPSTATE_CLOSING,
    TCPSTATE_LAST_ACK,
    TCPSTATE_TIME_WAIT,
    TCPSTATE_MAX
} TCPSTATE;
#endif // NTDDI_VERSION >= NTDDI_WIN10_RS2

#ifndef _TRANSPORT_SETTING_COMMON_
#define _TRANSPORT_SETTING_COMMON_

//
// This structure represents the key used in query and apply transport setting
// APIs.
//
typedef struct TRANSPORT_SETTING_ID
{
    GUID Guid;
} TRANSPORT_SETTING_ID, *PTRANSPORT_SETTING_ID;

#endif // _TRANSPORT_SETTING_COMMON_

//
// Argument structure for SIO_KEEPALIVE_VALS.
//
struct tcp_keepalive {
    ULONG onoff;
    ULONG keepalivetime;
    ULONG keepaliveinterval;
};

//
// Argument structures for SIO_APPLY_TRANSPORT_SETTING and
// SIO_QUERY_TRANSPORT_SETTING.
//
typedef enum
{
    CONTROL_CHANNEL_TRIGGER_STATUS_INVALID                 = 0,
    CONTROL_CHANNEL_TRIGGER_STATUS_SOFTWARE_SLOT_ALLOCATED = 1,
    CONTROL_CHANNEL_TRIGGER_STATUS_HARDWARE_SLOT_ALLOCATED = 2,
    CONTROL_CHANNEL_TRIGGER_STATUS_POLICY_ERROR            = 3,
    CONTROL_CHANNEL_TRIGGER_STATUS_SYSTEM_ERROR            = 4,
    CONTROL_CHANNEL_TRIGGER_STATUS_TRANSPORT_DISCONNECTED  = 5,
    CONTROL_CHANNEL_TRIGGER_STATUS_SERVICE_UNAVAILABLE     = 6,
} CONTROL_CHANNEL_TRIGGER_STATUS, *PCONTROL_CHANNEL_TRIGGER_STATUS;

#define CONTROL_CHANNEL_TRIGGER_STATUS_MAX CONTROL_CHANNEL_TRIGGER_STATUS_SYSTEM_ERROR

typedef struct _REAL_TIME_NOTIFICATION_SETTING_INPUT
{
    TRANSPORT_SETTING_ID TransportSettingId;
    GUID BrokerEventGuid;
} REAL_TIME_NOTIFICATION_SETTING_INPUT, *PREAL_TIME_NOTIFICATION_SETTING_INPUT;

typedef struct _REAL_TIME_NOTIFICATION_SETTING_INPUT_EX
{
    TRANSPORT_SETTING_ID TransportSettingId;
    GUID BrokerEventGuid;
    BOOLEAN Unmark;
} REAL_TIME_NOTIFICATION_SETTING_INPUT_EX, *PREAL_TIME_NOTIFICATION_SETTING_INPUT_EX;

typedef struct _REAL_TIME_NOTIFICATION_SETTING_OUTPUT
{
    CONTROL_CHANNEL_TRIGGER_STATUS ChannelStatus;
} REAL_TIME_NOTIFICATION_SETTING_OUTPUT, *PREAL_TIME_NOTIFICATION_SETTING_OUTPUT;

typedef struct _ASSOCIATE_NAMERES_CONTEXT_INPUT
{
    TRANSPORT_SETTING_ID TransportSettingId;

    //
    // Stores Handle as UINT64 to keep size consistent accross 32-bit and
    // 64-bit processes.
    //
    UINT64 Handle;
} ASSOCIATE_NAMERES_CONTEXT_INPUT, *PASSOCIATE_NAMERES_CONTEXT_INPUT;

//
// WSAIoctl Options
//
#define SIO_RCVALL                          _WSAIOW(IOC_VENDOR,1)
#define SIO_RCVALL_MCAST                    _WSAIOW(IOC_VENDOR,2)
#define SIO_RCVALL_IGMPMCAST                _WSAIOW(IOC_VENDOR,3)
#define SIO_KEEPALIVE_VALS                  _WSAIOW(IOC_VENDOR,4)
#define SIO_ABSORB_RTRALERT                 _WSAIOW(IOC_VENDOR,5)
#define SIO_UCAST_IF                        _WSAIOW(IOC_VENDOR,6)
#define SIO_LIMIT_BROADCASTS                _WSAIOW(IOC_VENDOR,7)
#define SIO_INDEX_BIND                      _WSAIOW(IOC_VENDOR,8)
#define SIO_INDEX_MCASTIF                   _WSAIOW(IOC_VENDOR,9)
#define SIO_INDEX_ADD_MCAST                 _WSAIOW(IOC_VENDOR,10)
#define SIO_INDEX_DEL_MCAST                 _WSAIOW(IOC_VENDOR,11)
#define SIO_RCVALL_MCAST_IF                 _WSAIOW(IOC_VENDOR,13)
#define SIO_RCVALL_IF                       _WSAIOW(IOC_VENDOR,14)
#define SIO_LOOPBACK_FAST_PATH              _WSAIOW(IOC_VENDOR,16)
#define SIO_TCP_INITIAL_RTO                 _WSAIOW(IOC_VENDOR,17)
#define SIO_APPLY_TRANSPORT_SETTING         _WSAIOW(IOC_VENDOR,19)
#define SIO_QUERY_TRANSPORT_SETTING         _WSAIOW(IOC_VENDOR,20)
#define SIO_TCP_SET_ICW                     _WSAIOW(IOC_VENDOR,22)
#define SIO_TCP_SET_ACK_FREQUENCY           _WSAIOW(IOC_VENDOR,23)
#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
#define SIO_SET_PRIORITY_HINT               _WSAIOW(IOC_VENDOR,24)
#endif // NTDDI_VERSION >= NTDDI_WIN10_RS3
#if (NTDDI_VERSION >= NTDDI_WIN10_FE)
#define SIO_PRIORITY_HINT SIO_SET_PRIORITY_HINT
#endif // NTDDI_VERSION >= NTDDI_WIN10_FE
#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
#define SIO_TCP_INFO                        _WSAIORW(IOC_VENDOR,39)
#endif // NTDDI_VERSION >= NTDDI_WIN10_RS2
#if (NTDDI_VERSION >= NTDDI_WIN10_VB)
//
// Socket IOCTL for port sharing per processor sockets.
//
#define SIO_CPU_AFFINITY                    _WSAIOW(IOC_VENDOR,21)
#endif // NTDDI_VERSION >= NTDDI_WIN10_VB

#if (NTDDI_VERSION >= NTDDI_WIN10_FE)
//
// Socket IOCTL used for configuring timestamp reception. Only valid for
// datagram sockets.
//
#define SIO_TIMESTAMPING _WSAIOW(IOC_VENDOR, 235)

//
// Input structure for SIO_TIMESTAMPING configuration option.
//
// Flags                - Enable/disable timestamp reception for rx/tx direction.
// TxTimestampsBuffered - Determines how many tx timestamps may be buffered.
//                        When TxTimestampsBuffered tx timestamps are buffered
//                        and a new tx timestamp has been generated, the new
//                        timestamp will be discarded.
//
typedef struct _TIMESTAMPING_CONFIG {
    ULONG Flags;
    USHORT TxTimestampsBuffered;
} TIMESTAMPING_CONFIG, *PTIMESTAMPING_CONFIG;

//
// Flags for the TIMESTAMPING_CONFIG struct. Specify a flag to enable, or omit
// to disable timestamp reception for that direction.
//
#define TIMESTAMPING_FLAG_RX 0x1
#define TIMESTAMPING_FLAG_TX 0x2

//
// Control message type for returning a rx timestamp through WSARecvMsg.
// The control message data is returned as a UINT64.
//
#define SO_TIMESTAMP 0x300A

//
// Control message type for specifying a tx timestamp ID through WSASendMsg.
// The control message data is supplied as a UINT32.
//
#define SO_TIMESTAMP_ID 0x300B

//
// Socket IOCTL to get timestamps for transmitted packets. Enable timestamp
// reception first by using the SIO_TIMESTAMPING socket IOCTL, then retrieve tx
// timestamps by ID using this IOCTL. Only valid for datagram sockets.
//
// Input is a UINT32 timestamp ID.
// Output is a UINT64 timestamp value.
// On success, the tx timestamp is available and is returned.
// On failure, the tx timestamp is unavailable. WSAGetLastError will return
//    WSAEWOULDBLOCK.
//
#define SIO_GET_TX_TIMESTAMP _WSAIOW(IOC_VENDOR, 234)

//
// This enum maps to the PRIORITY_HINT enum defined in WinBase.h.
//
typedef enum {
    SocketPriorityHintVeryLow = 0,
    SocketPriorityHintLow,
    SocketPriorityHintNormal,
    SocketMaximumPriorityHintType
} SOCKET_PRIORITY_HINT, *PSOCKET_PRIORITY_HINT;

typedef struct _PRIORITY_STATUS {
    SOCKET_PRIORITY_HINT Sender;
    SOCKET_PRIORITY_HINT Receiver;
} PRIORITY_STATUS, *PPRIORITY_STATUS;
#endif // NTDDI_VERSION >= NTDDI_WIN10_FE


//
// Values for use with SIO_RCVALL* options
//
typedef enum {
    RCVALL_OFF             = 0,
    RCVALL_ON              = 1,
    RCVALL_SOCKETLEVELONLY = 2,
    RCVALL_IPLEVEL         = 3,
} RCVALL_VALUE, *PRCVALL_VALUE;

#define RCVALL_MAX RCVALL_IPLEVEL

typedef struct {
    RCVALL_VALUE Mode;
    ULONG Interface;
} RCVALL_IF, *PRCVALL_IF;

//
// Parameters to configure the initial RTT.
//

#define TCP_INITIAL_RTO_UNSPECIFIED_RTT ((USHORT) -1)
#define TCP_INITIAL_RTO_UNSPECIFIED_MAX_SYN_RETRANSMISSIONS ((UCHAR) -1)

#define TCP_INITIAL_RTO_DEFAULT_RTT (0)
#define TCP_INITIAL_RTO_DEFAULT_MAX_SYN_RETRANSMISSIONS (0)
#define TCP_INITIAL_RTO_NO_SYN_RETRANSMISSIONS ((UCHAR) -2)

typedef struct _TCP_INITIAL_RTO_PARAMETERS {

    //
    // Supplies the initial RTT in milliseconds.
    //

    USHORT Rtt;

    //
    // Supplies the number of retransmissions attempted before the connection
    // setup fails.
    //

    UCHAR MaxSynRetransmissions;

} TCP_INITIAL_RTO_PARAMETERS, *PTCP_INITIAL_RTO_PARAMETERS;

//
// Values for ICW level
//
typedef enum {
    TCP_ICW_LEVEL_DEFAULT        = 0,
    TCP_ICW_LEVEL_HIGH           = 1,
    TCP_ICW_LEVEL_VERY_HIGH      = 2,
    TCP_ICW_LEVEL_AGGRESSIVE     = 3,
    TCP_ICW_LEVEL_EXPERIMENTAL   = 4,
    TCP_ICW_LEVEL_COMPAT         = 254,
    TCP_ICW_LEVEL_MAX            = 255
} TCP_ICW_LEVEL, *PTCP_ICW_LEVEL;

typedef struct _TCP_ICW_PARAMETERS {

    //
    // Supplies the level for choosing ICW.
    //

    TCP_ICW_LEVEL Level;

} TCP_ICW_PARAMETERS, *PTCP_ICW_PARAMETERS;

typedef struct _TCP_ACK_FREQUENCY_PARAMETERS {

    //
    // Supplies the Ack frequency.
    //

    UCHAR TcpDelayedAckFrequency;

} TCP_ACK_FREQUENCY_PARAMETERS, *PTCP_ACK_FREQUENCY_PARAMETERS;

//
// Output for SIO_TCP_INFO.
//

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
typedef struct _TCP_INFO_v0 {
    TCPSTATE State;
    ULONG Mss;
    ULONG64 ConnectionTimeMs;
    BOOLEAN TimestampsEnabled;
    ULONG RttUs;
    ULONG MinRttUs;
    ULONG BytesInFlight;
    ULONG Cwnd;
    ULONG SndWnd;
    ULONG RcvWnd;
    ULONG RcvBuf;
    ULONG64 BytesOut;
    ULONG64 BytesIn;
    ULONG BytesReordered;
    ULONG BytesRetrans;
    ULONG FastRetrans;
    ULONG DupAcksIn;
    ULONG TimeoutEpisodes;
    UCHAR SynRetrans;
} TCP_INFO_v0, *PTCP_INFO_v0;
#endif // NTDDI_VERSION >= NTDDI_WIN10_RS2

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
typedef struct _TCP_INFO_v1 {
    TCPSTATE State;
    ULONG Mss;
    ULONG64 ConnectionTimeMs;
    BOOLEAN TimestampsEnabled;
    ULONG RttUs;
    ULONG MinRttUs;
    ULONG BytesInFlight;
    ULONG Cwnd;
    ULONG SndWnd;
    ULONG RcvWnd;
    ULONG RcvBuf;
    ULONG64 BytesOut;
    ULONG64 BytesIn;
    ULONG BytesReordered;
    ULONG BytesRetrans;
    ULONG FastRetrans;
    ULONG DupAcksIn;
    ULONG TimeoutEpisodes;
    UCHAR SynRetrans;

    //
    // Info about the limiting factor in send throughput.
    //
    // States:
    // -Rwin: peer's receive window.
    // -Cwnd: congestion window.
    // -Snd: app not writing enough data to its socket.
    //
    // Per-state statistics:
    // -Trans: number of transitions into the state.
    // -Time: time spent in the state in milliseconds.
    // -Bytes: number of bytes sent while in the state.
    //
    // These fields match those in TCP_ESTATS_SND_CONG_ROD.
    //
    ULONG SndLimTransRwin;
    ULONG SndLimTimeRwin;
    ULONG64 SndLimBytesRwin;
    ULONG SndLimTransCwnd;
    ULONG SndLimTimeCwnd;
    ULONG64 SndLimBytesCwnd;
    ULONG SndLimTransSnd;
    ULONG SndLimTimeSnd;
    ULONG64 SndLimBytesSnd;

} TCP_INFO_v1, *PTCP_INFO_v1;
#endif // NTDDI_VERSION >= NTDDI_WIN10_RS5

#if (NTDDI_VERSION >= NTDDI_WIN11_GA)
typedef struct _TCP_INFO_v2 {
    TCPSTATE State;
    ULONG Mss;
    ULONG64 ConnectionTimeMs;
    BOOLEAN TimestampsEnabled;
    ULONG RttUs;
    ULONG MinRttUs;
    ULONG BytesInFlight;
    ULONG Cwnd;
    ULONG SndWnd;
    ULONG RcvWnd;
    ULONG RcvBuf;
    ULONG64 BytesOut;
    ULONG64 BytesIn;
    //
    // The number of bytes that are considered reordered from the sender side.
    //
    ULONG BytesReordered;
    ULONG BytesRetrans;
    ULONG FastRetrans;
    ULONG DupAcksIn;
    ULONG TimeoutEpisodes;
    UCHAR SynRetrans;

    //
    // Info about the limiting factor in send throughput.
    //
    // States:
    // -Rwin: peer's receive window.
    // -Cwnd: congestion window.
    // -Snd: app not writing enough data to its socket.
    //
    // Per-state statistics:
    // -Trans: number of transitions into the state.
    // -Time: time spent in the state in milliseconds.
    // -Bytes: number of bytes sent while in the state.
    //
    // These fields match those in TCP_ESTATS_SND_CONG_ROD.
    //
    ULONG SndLimTransRwin;
    ULONG SndLimTimeRwin;
    ULONG64 SndLimBytesRwin;
    ULONG SndLimTransCwnd;
    ULONG SndLimTimeCwnd;
    ULONG64 SndLimBytesCwnd;
    ULONG SndLimTransSnd;
    ULONG SndLimTimeSnd;
    ULONG64 SndLimBytesSnd;

    //
    // The number of out of order data packets received.
    //
    ULONG OutOfOrderPktsIn;

    BOOLEAN EcnNegotiated;
    ULONG EceAcksIn;
    ULONG PtoEpisodes;
} TCP_INFO_v2, *PTCP_INFO_v2;
#endif // NTDDI_VERSION >= NTDDI_WIN11_GA

//
// TCP/UDP port management definitions.
//
#define SIO_ACQUIRE_PORT_RESERVATION    _WSAIOW(IOC_VENDOR, 100)
#define SIO_RELEASE_PORT_RESERVATION    _WSAIOW(IOC_VENDOR, 101)
#define SIO_ASSOCIATE_PORT_RESERVATION  _WSAIOW(IOC_VENDOR, 102)

typedef struct _INET_PORT_RANGE {
    USHORT StartPort;
    USHORT NumberOfPorts;
} INET_PORT_RANGE, *PINET_PORT_RANGE;

typedef struct _INET_PORT_RANGE INET_PORT_RESERVATION, *PINET_PORT_RESERVATION;

typedef struct {
    ULONG64 Token;
} INET_PORT_RESERVATION_TOKEN, *PINET_PORT_RESERVATION_TOKEN;

#define INVALID_PORT_RESERVATION_TOKEN ((ULONG64)0)

typedef struct {
#ifdef __cplusplus
    INET_PORT_RESERVATION Reservation;
    INET_PORT_RESERVATION_TOKEN Token;
#else
    INET_PORT_RESERVATION;
    INET_PORT_RESERVATION_TOKEN;
#endif
} INET_PORT_RESERVATION_INSTANCE, *PINET_PORT_RESERVATION_INSTANCE;

typedef struct {
    ULONG OwningPid;
} INET_PORT_RESERVATION_INFORMATION, *PINET_PORT_RESERVATION_INFORMATION;


#ifdef _WS2DEF_

#if (NTDDI_VERSION >= NTDDI_VISTA)
#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#define _SECURE_SOCKET_TYPES_DEFINED_

//
// Secure socket API type definitions.
//

#define SIO_SET_SECURITY             _WSAIOW(IOC_VENDOR, 200)
#define SIO_QUERY_SECURITY           _WSAIORW(IOC_VENDOR, 201)
#define SIO_SET_PEER_TARGET_NAME     _WSAIOW(IOC_VENDOR, 202)
#define SIO_DELETE_PEER_TARGET_NAME  _WSAIOW(IOC_VENDOR, 203)


//
// WFP Proxy Connection Tracking API type definitions.
//
#define SIO_QUERY_WFP_CONNECTION_REDIRECT_RECORDS  _WSAIOW(IOC_VENDOR, 220)
#define SIO_QUERY_WFP_CONNECTION_REDIRECT_CONTEXT  _WSAIOW(IOC_VENDOR, 221)
#define SIO_SET_WFP_CONNECTION_REDIRECT_RECORDS    _WSAIOW(IOC_VENDOR, 222)

//
// iSCSI IOCTLS
//
#define SIO_SOCKET_USAGE_NOTIFICATION _WSAIOW(IOC_VENDOR, 204)
typedef enum _SOCKET_USAGE_TYPE
{
   SYSTEM_CRITICAL_SOCKET = 1
}SOCKET_USAGE_TYPE;

// Type of security protocol
typedef enum _SOCKET_SECURITY_PROTOCOL
{
   SOCKET_SECURITY_PROTOCOL_DEFAULT,
   SOCKET_SECURITY_PROTOCOL_IPSEC,
#if (NTDDI_VERSION >= NTDDI_WIN7)
   SOCKET_SECURITY_PROTOCOL_IPSEC2,      //version 2 of secure socket apis
#endif // (NTDDI_VERSION >= NTDDI_WIN7)
   SOCKET_SECURITY_PROTOCOL_INVALID
} SOCKET_SECURITY_PROTOCOL;

// Flags for generic security settings
#define SOCKET_SETTINGS_GUARANTEE_ENCRYPTION  0x1
#define SOCKET_SETTINGS_ALLOW_INSECURE  0x2

// Common security settings for the socket
typedef struct _SOCKET_SECURITY_SETTINGS
{
   SOCKET_SECURITY_PROTOCOL SecurityProtocol;
   ULONG SecurityFlags;
} SOCKET_SECURITY_SETTINGS;

// Flags specific to IPsec security settings.
// NOTE: these flags must be specified under the
// SOCKET_SECURITY_SETTINGS_IPSEC->IpsecFlags field.

#define SOCKET_SETTINGS_IPSEC_SKIP_FILTER_INSTANTIATION            0x1
#if (NTDDI_VERSION >= NTDDI_WIN7)
#define SOCKET_SETTINGS_IPSEC_OPTIONAL_PEER_NAME_VERIFICATION      0x2
#define SOCKET_SETTINGS_IPSEC_ALLOW_FIRST_INBOUND_PKT_UNENCRYPTED  0x4
#define SOCKET_SETTINGS_IPSEC_PEER_NAME_IS_RAW_FORMAT              0x8
#endif // (NTDDI_VERSION >= NTDDI_WIN7)


// IPsec specific security settings for the socket.
typedef struct _SOCKET_SECURITY_SETTINGS_IPSEC
{
   SOCKET_SECURITY_PROTOCOL SecurityProtocol;
   ULONG SecurityFlags;
   ULONG IpsecFlags;
   GUID AuthipMMPolicyKey;
   GUID AuthipQMPolicyKey;
   GUID Reserved;
   UINT64 Reserved2;
   ULONG UserNameStringLen;
   ULONG DomainNameStringLen;
   ULONG PasswordStringLen;
   wchar_t AllStrings[0];
} SOCKET_SECURITY_SETTINGS_IPSEC;

// Common peer target name for the socket
typedef struct _SOCKET_PEER_TARGET_NAME
{
   SOCKET_SECURITY_PROTOCOL SecurityProtocol;
   SOCKADDR_STORAGE PeerAddress;
   ULONG PeerTargetNameStringLen;
   wchar_t AllStrings[0];
} SOCKET_PEER_TARGET_NAME;

// Input for querying socket security
typedef struct _SOCKET_SECURITY_QUERY_TEMPLATE
{
   SOCKET_SECURITY_PROTOCOL SecurityProtocol;
   SOCKADDR_STORAGE PeerAddress;
   ULONG PeerTokenAccessMask;
} SOCKET_SECURITY_QUERY_TEMPLATE;

#if (NTDDI_VERSION >= NTDDI_WIN7)

#define SOCKET_QUERY_IPSEC2_ABORT_CONNECTION_ON_FIELD_CHANGE 0x1

#define SOCKET_QUERY_IPSEC2_FIELD_MASK_MM_SA_ID 0x1
#define SOCKET_QUERY_IPSEC2_FIELD_MASK_QM_SA_ID 0x2

// Input for querying socket security
typedef struct _SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2
{
   SOCKET_SECURITY_PROTOCOL SecurityProtocol;
   SOCKADDR_STORAGE PeerAddress;
   ULONG PeerTokenAccessMask;
   ULONG Flags;
   ULONG FieldMask;
} SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2;
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

// Flags corresponding to the security query info
#define SOCKET_INFO_CONNECTION_SECURED  0x1
#define SOCKET_INFO_CONNECTION_ENCRYPTED  0x2
#define SOCKET_INFO_CONNECTION_IMPERSONATED 0x4

// Common security query information from a socket
typedef struct _SOCKET_SECURITY_QUERY_INFO
{
   SOCKET_SECURITY_PROTOCOL SecurityProtocol;
   ULONG Flags;
   UINT64 PeerApplicationAccessTokenHandle;
   UINT64 PeerMachineAccessTokenHandle;
} SOCKET_SECURITY_QUERY_INFO;

#if (NTDDI_VERSION >= NTDDI_WIN7)
// Common security query information from a socket
typedef struct _SOCKET_SECURITY_QUERY_INFO_IPSEC2
{
   SOCKET_SECURITY_PROTOCOL SecurityProtocol;
   ULONG Flags;
   UINT64 PeerApplicationAccessTokenHandle;
   UINT64 PeerMachineAccessTokenHandle;
   UINT64 MmSaId;
   UINT64 QmSaId;
   UINT32 NegotiationWinerr;
   GUID SaLookupContext;
} SOCKET_SECURITY_QUERY_INFO_IPSEC2;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)

//
// WFP ALE endpoint handle query type definition
//

#define SIO_QUERY_WFP_ALE_ENDPOINT_HANDLE _WSAIOR(IOC_VENDOR, 205)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

//
// Scalability type definitions
//
#define SIO_QUERY_RSS_SCALABILITY_INFO  _WSAIOR(IOC_VENDOR, 210)

// Scalability info for the transport
typedef struct _RSS_SCALABILITY_INFO {
   BOOLEAN RssEnabled;
} RSS_SCALABILITY_INFO, *PRSS_SCALABILITY_INFO;

#endif // (NTDDI_VERSION >= NTDDI_VISTA)


//
// Microsoft-specific IPv4 definitions.
//

#define IN4_CLASSA(i) (((LONG)(i) & 0x00000080) == 0)
#define IN4_CLASSB(i) (((LONG)(i) & 0x000000c0) == 0x00000080)
#define IN4_CLASSC(i) (((LONG)(i) & 0x000000e0) == 0x000000c0)
#define IN4_CLASSD(i) (((LONG)(i) & 0x000000f0) == 0x000000e0)
#define IN4_MULTICAST(i) IN4_CLASSD(i)

#define IN4ADDR_ANY INADDR_ANY
#define IN4ADDR_LOOPBACK 0x0100007f
#define IN4ADDR_BROADCAST INADDR_BROADCAST
#define IN4ADDR_NONE INADDR_NONE
#define IN4ADDR_ANY_INIT { 0 }
#define IN4ADDR_LOOPBACK_INIT { 0x7f, 0, 0, 1 }
#define IN4ADDR_BROADCAST_INIT { 0xff, 0xff, 0xff, 0xff }
#define IN4ADDR_ALLNODESONLINK_INIT { 0xe0, 0, 0, 1 }
#define IN4ADDR_ALLROUTERSONLINK_INIT { 0xe0, 0, 0, 2 }
#define IN4ADDR_ALLIGMPV3ROUTERSONLINK_INIT { 0xe0, 0, 0, 0x16 }
#define IN4ADDR_ALLTEREDONODESONLINK_INIT { 0xe0, 0, 0, 0xfd }
#define IN4ADDR_LINKLOCALPREFIX_INIT { 0xa9, 0xfe, }
#define IN4ADDR_MULTICASTPREFIX_INIT { 0xe0, }

#define IN4ADDR_LOOPBACKPREFIX_LENGTH 8
#define IN4ADDR_LINKLOCALPREFIX_LENGTH 16
#define IN4ADDR_MULTICASTPREFIX_LENGTH 4

#if (NTDDI_VERSION >= NTDDI_WIN2KSP1)
MSTCPIP_INLINE
BOOLEAN
IN4_ADDR_EQUAL(_In_ CONST IN_ADDR *a, _In_ CONST IN_ADDR *b)
{
    return (BOOLEAN)(a->s_addr == b->s_addr);
}

MSTCPIP_INLINE
BOOLEAN
IN4_UNALIGNED_ADDR_EQUAL(_In_ CONST IN_ADDR UNALIGNED *a, _In_ CONST IN_ADDR UNALIGNED *b)
{
    return (BOOLEAN)(a->s_addr == b->s_addr);
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_ADDR_UNSPECIFIED(_In_ CONST IN_ADDR *a)
{
    return (BOOLEAN)(a->s_addr == IN4ADDR_ANY);
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_UNALIGNED_ADDR_UNSPECIFIED(_In_ CONST IN_ADDR UNALIGNED *a)
{
    return (BOOLEAN)(a->s_addr == IN4ADDR_ANY);
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_ADDR_LOOPBACK(_In_ CONST IN_ADDR *a)
{
    return (BOOLEAN)(*((PUCHAR) a) == 0x7f); // 127/8
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_UNALIGNED_ADDR_LOOPBACK(_In_ CONST IN_ADDR UNALIGNED *a)
{
    return (BOOLEAN)(*((PUCHAR) a) == 0x7f); // 127/8
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_ADDR_BROADCAST(_In_ CONST IN_ADDR *a)
{
    return (BOOLEAN)(a->s_addr == IN4ADDR_BROADCAST);
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_UNALIGNED_ADDR_BROADCAST(_In_ CONST IN_ADDR UNALIGNED *a)
{
    return (BOOLEAN)(a->s_addr == IN4ADDR_BROADCAST);
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_ADDR_MULTICAST(_In_ CONST IN_ADDR *a)
{
    return (BOOLEAN)IN4_MULTICAST(a->s_addr);
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_UNALIGNED_ADDR_MULTICAST(_In_ CONST IN_ADDR UNALIGNED *a)
{
    return (BOOLEAN)IN4_MULTICAST(a->s_addr);
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_ADDR_LINKLOCAL(_In_ CONST IN_ADDR *a)
{
    return (BOOLEAN)((a->s_addr & 0xffff) == 0xfea9); // 169.254/16
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_UNALIGNED_ADDR_LINKLOCAL(_In_ CONST IN_ADDR UNALIGNED *a)
{
    return (BOOLEAN)((a->s_addr & 0xffff) == 0xfea9); // 169.254/16
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_ADDR_SITELOCAL(_In_ CONST IN_ADDR *a)
{
    //
    // For existing scenarios (e.g. ICS) to work as expected, RFC-1918 prefixes
    // are deemed to be global scoped.  When appropriate, site border routers
    // must explicitly filter packets with these addresses.
    //
    UNREFERENCED_PARAMETER(a);
    return FALSE;
}

#define IN4_IS_UNALIGNED_ADDR_SITELOCAL IN4_IS_ADDR_SITELOCAL

MSTCPIP_INLINE
BOOLEAN
IN4_IS_ADDR_RFC1918(_In_ CONST IN_ADDR *a)
{
    return (BOOLEAN)(((a->s_addr & 0x00ff) == 0x0a) ||   // 10/8
                     ((a->s_addr & 0xf0ff) == 0x10ac) || // 172.16/12
                     ((a->s_addr & 0xffff) == 0xa8c0));  // 192.168/16
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_UNALIGNED_ADDR_RFC1918(_In_ CONST IN_ADDR UNALIGNED *a)
{
    IN_ADDR Ipv4Address = *a;
    return IN4_IS_ADDR_RFC1918(&Ipv4Address);
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_ADDR_MC_LINKLOCAL(_In_ CONST IN_ADDR *a)
{
    return (BOOLEAN)((a->s_addr & 0xffffff) == 0xe0); // 224.0.0/24
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_ADDR_MC_ADMINLOCAL(_In_ CONST IN_ADDR *a)
{
    return (BOOLEAN)((a->s_addr & 0xffff) == 0xffef); // 239.255/16
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_ADDR_MC_SITELOCAL(_In_ CONST IN_ADDR *a)
{
    return (BOOLEAN)((a->s_addr & 0xff) == 0xef) &&
                    !IN4_IS_ADDR_MC_ADMINLOCAL(a);
}

MSTCPIP_INLINE
VOID
IN4ADDR_SETSOCKADDR(_Out_ PSOCKADDR_IN a, _In_ CONST IN_ADDR *addr, _In_ USHORT port)
{
    a->sin_family = AF_INET;
    a->sin_port = port;
    a->sin_addr = *addr;
    memset(a->sin_zero, 0, sizeof(a->sin_zero));
}

MSTCPIP_INLINE
VOID
IN4ADDR_SETANY(_Out_ PSOCKADDR_IN a)
{
    a->sin_family = AF_INET;
    a->sin_port = 0;
    a->sin_addr.s_addr = IN4ADDR_ANY;
    memset(a->sin_zero, 0, sizeof(a->sin_zero));
}

MSTCPIP_INLINE
VOID
IN4ADDR_SETLOOPBACK(_Out_ PSOCKADDR_IN a)
{
    a->sin_family = AF_INET;
    a->sin_port = 0;
    a->sin_addr.s_addr = IN4ADDR_LOOPBACK;
    memset(a->sin_zero, 0, sizeof(a->sin_zero));
}

MSTCPIP_INLINE
BOOLEAN
IN4ADDR_ISANY(_In_ CONST SOCKADDR_IN *a)
{
    ASSERT(a->sin_family == AF_INET);
    return IN4_IS_ADDR_UNSPECIFIED(&a->sin_addr);
}

MSTCPIP_INLINE
BOOLEAN
IN4ADDR_ISLOOPBACK(_In_ CONST SOCKADDR_IN *a)
{
    ASSERT(a->sin_family == AF_INET);
    return IN4_IS_ADDR_LOOPBACK(&a->sin_addr);
}

MSTCPIP_INLINE
SCOPE_ID
IN4ADDR_SCOPE_ID(_In_ CONST SOCKADDR_IN *a)
{
    SCOPE_ID UnspecifiedScopeId = {0};
    UNREFERENCED_PARAMETER(a);
    return UnspecifiedScopeId;
}

MSTCPIP_INLINE
BOOLEAN
IN4ADDR_ISEQUAL(_In_ CONST SOCKADDR_IN *a, _In_ CONST SOCKADDR_IN *b)
{
    ASSERT(a->sin_family == AF_INET);
    return (BOOLEAN)(IN4ADDR_SCOPE_ID(a).Value == IN4ADDR_SCOPE_ID(b).Value &&
                     IN4_ADDR_EQUAL(&a->sin_addr, &b->sin_addr));
}

MSTCPIP_INLINE
BOOLEAN
IN4ADDR_ISUNSPECIFIED(_In_ CONST SOCKADDR_IN *a)
{
    ASSERT(a->sin_family == AF_INET);
    return (BOOLEAN)(IN4ADDR_SCOPE_ID(a).Value == 0 &&
                     IN4_IS_ADDR_UNSPECIFIED(&a->sin_addr));
}

#define INET_IS_ALIGNED(Pointer, Type) \
   (((ULONG_PTR)Pointer & (__builtin_alignof(Type)-1)) == 0)

MSTCPIP_INLINE
SCOPE_LEVEL
Ipv4UnicastAddressScope(
    _In_reads_(_Inexpressible_("Varies")) CONST UCHAR *Address
    )
/*++

Routine Description:

    Determines the scope of an IPv4 unicast address.

    For existing scenarios (e.g. ICS) to work as expected, RFC-1918 prefixes
    are deemed to be global scoped.  When appropriate, site border routers
    must explicitly filter packets with these addresses.

Arguments:

    Address - Supplies the IPv4 unicast address.

Return Value:

    Returns the scope level of the address.

Caller IRQL:

    May be called at PASSIVE through DISPATCH level.

--*/
{
    IN_ADDR Ipv4Address;

    if (!INET_IS_ALIGNED(Address, IN_ADDR)) {
        Ipv4Address = *(CONST IN_ADDR UNALIGNED *)Address;
        Address = (CONST UCHAR *) &Ipv4Address;
    }

    if (IN4_IS_ADDR_LINKLOCAL((PIN_ADDR) Address) ||
        IN4_IS_ADDR_LOOPBACK((PIN_ADDR) Address)) {
        return ScopeLevelLink;
    }
    return ScopeLevelGlobal;
}

MSTCPIP_INLINE
SCOPE_LEVEL
Ipv4MulticastAddressScope(
    _In_reads_(_Inexpressible_("Varies")) CONST UCHAR *Address
    )
/*++

Routine Description:

    Determines the scope of an IPv4 multicast address.
    See RFC 2365.

Arguments:

    Address - Supplies the IPv4 multicast address.

Return Value:

    Returns the scope level of the multicast address.

Caller IRQL:

    May be called at PASSIVE through DISPATCH level.

--*/
{
    IN_ADDR Ipv4Address;

    if (!INET_IS_ALIGNED(Address, IN_ADDR)) {
        Ipv4Address = *(CONST IN_ADDR UNALIGNED *)Address;
        Address = (CONST UCHAR *) &Ipv4Address;
    }

    if (IN4_IS_ADDR_MC_LINKLOCAL((PIN_ADDR) Address)) {
        return ScopeLevelLink;
    } else if (IN4_IS_ADDR_MC_ADMINLOCAL((PIN_ADDR) Address)) {
        return ScopeLevelAdmin;
    } else if (IN4_IS_ADDR_MC_SITELOCAL((PIN_ADDR) Address)) {
        return ScopeLevelSite;
    } else {
        return ScopeLevelGlobal;
    }
}

MSTCPIP_INLINE
SCOPE_LEVEL
Ipv4AddressScope(
    _In_reads_(_Inexpressible_("Varies")) CONST UCHAR *Address
    )
/*++

Routine Description:

    Examines an IPv4 address and determines its scope.

Arguments:

    Address - Supplies the address to test.

Return Value:

    Returns the scope level of the address.

Caller IRQL:

    May be called at PASSIVE through DISPATCH level.

--*/
{
    IN CONST IN_ADDR Ipv4Address = *(CONST IN_ADDR UNALIGNED *)Address;

    if (IN4_IS_ADDR_BROADCAST(&Ipv4Address)) {
        return ScopeLevelLink;
    } else if (IN4_IS_ADDR_MULTICAST(&Ipv4Address)) {
        return Ipv4MulticastAddressScope((UCHAR *) &Ipv4Address);
    } else {
        return Ipv4UnicastAddressScope((UCHAR *) &Ipv4Address);
    }
}

MSTCPIP_INLINE
NL_ADDRESS_TYPE
Ipv4AddressType(
    _In_reads_(_Inexpressible_("Varies")) CONST UCHAR *Address
    )
{
    IN_ADDR Ipv4Address = *(CONST IN_ADDR UNALIGNED *) Address;

    if (IN4_IS_ADDR_MULTICAST(&Ipv4Address)) {
        return NlatMulticast;
    }

    if (IN4_IS_ADDR_BROADCAST(&Ipv4Address)) {
        return NlatBroadcast;
    }

    if (IN4_IS_ADDR_UNSPECIFIED(&Ipv4Address)) {
        return NlatUnspecified;
    }

    if (((Ipv4Address.s_addr & 0x000000ff) == 0) ||
        ((Ipv4Address.s_addr & 0x000000f0) == 240)) {
        //
        // Following prefixes are invalid:
        // 1. 0.0.0.0/8 (except 0.0.0.0/32).
        // 2. 240.0.0.0/4 (except 255.255.255.255/32).
        //
        return NlatInvalid;
    }

    //
    // Loopback and anycast addresses are treated as unicast.
    //
    return NlatUnicast;
}

MSTCPIP_INLINE
VOID
IN4_UNCANONICALIZE_SCOPE_ID(
    _In_ CONST IN_ADDR *Address,
    _Inout_ SCOPE_ID *ScopeId
    )
{
    SCOPE_LEVEL ScopeLevel = Ipv4AddressScope((CONST UCHAR *)Address);

    if ((IN4_IS_ADDR_LOOPBACK(Address)) || (ScopeLevel == ScopeLevelGlobal)) {
        ScopeId->Value = 0;
    }

    if ((SCOPE_LEVEL)ScopeId->Level == ScopeLevel) {
        ScopeId->Level = 0;
    }
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_ADDR_6TO4ELIGIBLE(_In_ CONST IN_ADDR *a)
{
    return (BOOLEAN)((Ipv4AddressType((CONST UCHAR *) a) == NlatUnicast) &&
                     !(IN4_IS_ADDR_LOOPBACK(a) ||
                       IN4_IS_ADDR_LINKLOCAL(a) ||
                       IN4_IS_ADDR_SITELOCAL(a) ||
                       IN4_IS_ADDR_RFC1918(a)));
}

MSTCPIP_INLINE
BOOLEAN
IN4_IS_UNALIGNED_ADDR_6TO4ELIGIBLE(_In_ CONST IN_ADDR UNALIGNED *a)
{
    IN_ADDR Ipv4Address = *a;
    return IN4_IS_ADDR_6TO4ELIGIBLE(&Ipv4Address);
}

#endif // _WS2DEF_


//
// Microsoft-specific IPv6 definitions.
//

#ifdef _WS2IPDEF_

#ifdef _PREFAST_
#pragma prefast(push)
#pragma prefast(disable:6385, "apparent one byte buffer overread in IN6_PREFIX_EQUAL")
#endif

MSTCPIP_INLINE
BOOLEAN
IN6_PREFIX_EQUAL(
    _In_ CONST IN6_ADDR *a,
    _In_ CONST IN6_ADDR *b,
    _In_ _In_range_(0, (sizeof(IN6_ADDR) * 8)) UINT8 len
    )
{
    UINT8 Bytes = len / 8;
    UINT8 Bits = len % 8;
    UINT8 Mask = 0xff << (8 - Bits);

    return (BOOLEAN)
        (((memcmp(a, b, Bytes)) == 0) &&
         ((Bits == 0) ||
          ((a->s6_bytes[Bytes] & Mask) == (b->s6_bytes[Bytes] & Mask))));
}

#ifdef _PREFAST_
#pragma prefast(pop)
#endif

MSTCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_ALLNODESONNODE(_In_ CONST IN6_ADDR *a)
{
    return IN6_ADDR_EQUAL(a, &in6addr_allnodesonnode);
}

MSTCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_ALLNODESONLINK(_In_ CONST IN6_ADDR *a)
{
    return IN6_ADDR_EQUAL(a, &in6addr_allnodesonlink);
}

MSTCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_ALLROUTERSONLINK(_In_ CONST IN6_ADDR *a)
{
    return IN6_ADDR_EQUAL(a, &in6addr_allroutersonlink);
}

MSTCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_SOLICITEDNODE(_In_ CONST IN6_ADDR *a)
{
    return
        IN6_PREFIX_EQUAL(
            a,
            &in6addr_solicitednodemulticastprefix,
            IN6ADDR_SOLICITEDNODEMULTICASTPREFIX_LENGTH);
}

MSTCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_ISATAP(_In_ CONST IN6_ADDR *a)
{
    //
    // Disregard the u/g bit and compare the first byte of the interface id.
    //
    return (BOOLEAN)(((a->s6_words[4] & 0xfffd) == 0x0000) &&
                     (a->s6_words[5] == 0xfe5e));
}

MSTCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_6TO4(_In_ CONST IN6_ADDR *a)
{
    C_ASSERT(IN6ADDR_6TO4PREFIX_LENGTH == RTL_BITS_OF(USHORT));
    return (BOOLEAN)(a->s6_words[0] == in6addr_6to4prefix.s6_words[0]);
}

MSTCPIP_INLINE
BOOLEAN
IN6_IS_ADDR_TEREDO(_In_ CONST IN6_ADDR *a)
{
    C_ASSERT(IN6ADDR_TEREDOPREFIX_LENGTH == 2 * RTL_BITS_OF(USHORT));
    return (BOOLEAN)
        (((a->s6_words[0] == in6addr_teredoprefix.s6_words[0]) &&
          (a->s6_words[1] == in6addr_teredoprefix.s6_words[1])) ||
         ((a->s6_words[0] == in6addr_teredoprefix_old.s6_words[0]) &&
          (a->s6_words[1] == in6addr_teredoprefix_old.s6_words[1])));
}

MSTCPIP_INLINE
BOOLEAN
IN6ADDR_ISV4MAPPED(_In_ CONST SOCKADDR_IN6 *a)
{
    ASSERT(a->sin6_family == AF_INET6);
    return IN6_IS_ADDR_V4MAPPED(&a->sin6_addr);
}

MSTCPIP_INLINE
BOOLEAN
IN6ADDR_ISISATAP(_In_ CONST SOCKADDR_IN6 *a)
{
    ASSERT(a->sin6_family == AF_INET6);
    return IN6_IS_ADDR_ISATAP(&a->sin6_addr);
}

MSTCPIP_INLINE
BOOLEAN
IN6ADDR_IS6TO4(_In_ CONST SOCKADDR_IN6 *a)
{
    ASSERT(a->sin6_family == AF_INET6);
    return IN6_IS_ADDR_6TO4(&a->sin6_addr);
}

MSTCPIP_INLINE
BOOLEAN
IN6ADDR_ISTEREDO(_In_ CONST SOCKADDR_IN6 *a)
{
    ASSERT(a->sin6_family == AF_INET6);
    return IN6_IS_ADDR_TEREDO(&a->sin6_addr);
}

MSTCPIP_INLINE
CONST UCHAR*
IN6_GET_ADDR_V4MAPPED(_In_ CONST IN6_ADDR *Ipv6Address)
{
    return (CONST UCHAR *) (Ipv6Address->s6_words + 6);
}

MSTCPIP_INLINE
CONST UCHAR*
IN6_GET_ADDR_V4COMPAT(_In_ CONST IN6_ADDR *Ipv6Address)
{
    return (CONST UCHAR *) (Ipv6Address->s6_words + 6);
}

MSTCPIP_INLINE
CONST UCHAR*
IN6_EXTRACT_V4ADDR_FROM_ISATAP(_In_ CONST IN6_ADDR *Ipv6Address)
{
    return (CONST UCHAR *) (Ipv6Address->s6_words + 6);
}

MSTCPIP_INLINE
CONST UCHAR*
IN6_EXTRACT_V4ADDR_FROM_6TO4(_In_ CONST IN6_ADDR *Ipv6Address)
{
    return (CONST UCHAR *) (Ipv6Address->s6_words + 1);
}

MSTCPIP_INLINE
VOID
IN6_SET_ADDR_V4MAPPED(_Out_ PIN6_ADDR a6, _In_ CONST IN_ADDR* a4)
{
    *a6 = in6addr_v4mappedprefix;
    a6->s6_bytes[12] = ((CONST UCHAR *) a4)[0];
    a6->s6_bytes[13] = ((CONST UCHAR *) a4)[1];
    a6->s6_bytes[14] = ((CONST UCHAR *) a4)[2];
    a6->s6_bytes[15] = ((CONST UCHAR *) a4)[3];
}

MSTCPIP_INLINE
VOID
IN6_SET_ADDR_V4COMPAT(_Out_ PIN6_ADDR a6, _In_ CONST IN_ADDR* a4)
{
    *a6 = in6addr_any;
    a6->s6_bytes[12] = ((CONST UCHAR *) a4)[0];
    a6->s6_bytes[13] = ((CONST UCHAR *) a4)[1];
    a6->s6_bytes[14] = ((CONST UCHAR *) a4)[2];
    a6->s6_bytes[15] = ((CONST UCHAR *) a4)[3];
}

MSTCPIP_INLINE
VOID
IN6_SET_ADDR_SOLICITEDNODE(_Out_ PIN6_ADDR Multicast, _In_ CONST IN6_ADDR *Unicast)
{
    *Multicast = in6addr_solicitednodemulticastprefix;
    Multicast->s6_bytes[13] = Unicast->s6_bytes[13];
    Multicast->s6_bytes[14] = Unicast->s6_bytes[14];
    Multicast->s6_bytes[15] = Unicast->s6_bytes[15];
}

MSTCPIP_INLINE
VOID
IN6_SET_ISATAP_IDENTIFIER(_Inout_ IN6_ADDR *Ipv6Address, _In_ CONST IN_ADDR *Ipv4Address)
{
    if (IN4_IS_ADDR_6TO4ELIGIBLE(Ipv4Address)) {
        Ipv6Address->s6_words[4] = 0x0002;
    } else {
        Ipv6Address->s6_words[4] = 0x0000;
    }

    //
    // 24-bit IANA OUI 00-00-5E and the 8-bit hex value 0xFE.
    // See section 6.1 of RFC 4214.
    //
    Ipv6Address->s6_words[5] = 0xFE5E;
    *((UNALIGNED IN_ADDR *) (Ipv6Address->s6_words + 6)) = *Ipv4Address;
}

MSTCPIP_INLINE
VOID
IN6_SET_6TO4_PREFIX(_Inout_ IN6_ADDR *Ipv6Address, _In_ CONST IN_ADDR *Ipv4Address)
{
    Ipv6Address->s6_words[0] = 0x0220;
    *((UNALIGNED IN_ADDR *) (Ipv6Address->s6_words + 1)) = *Ipv4Address;
    Ipv6Address->s6_words[3] = 0x0000;
}

MSTCPIP_INLINE
SCOPE_LEVEL
Ipv6UnicastAddressScope(
    _In_reads_(_Inexpressible_("Varies")) CONST UCHAR *Address
    )
/*++

Routine Description:

    Examines a unicast address and determines its scope.

    Note that v4-compatible and 6to4 addresses are deemed to have global scope;
    it is not legal to derive them from non IN4_IS_ADDR_6TO4ELIGIBLE addresses
    (IPv4 loopback, link-local, site-local, and RFC-1918 addresses).

Arguments:

    Address - Supplies an IPv6 unicast address.

Return Value:

    Returns the scope level of the address.

Caller IRQL:

    May be called at PASSIVE through DISPATCH level.

--*/
{
    IN6_ADDR Ipv6Address;

    if (!INET_IS_ALIGNED(Address, IN6_ADDR)) {
        Ipv6Address = *(CONST IN6_ADDR UNALIGNED *)Address;
        Address = (CONST UCHAR *) &Ipv6Address;
    }

    if (IN6_IS_ADDR_LINKLOCAL((PIN6_ADDR) Address) ||
        IN6_IS_ADDR_LOOPBACK((PIN6_ADDR) Address)) {
        return ScopeLevelLink;
    } else if (IN6_IS_ADDR_SITELOCAL((PIN6_ADDR) Address)) {
        return ScopeLevelSite;
    } else {
        return ScopeLevelGlobal;
    }
}

MSTCPIP_INLINE
SCOPE_LEVEL
IN6_MULTICAST_SCOPE(
    _In_reads_(_Inexpressible_("Varies")) CONST UCHAR *Address
    )
{
    PIN6_ADDR Ipv6Address = (PIN6_ADDR) Address;
    return (SCOPE_LEVEL)(Ipv6Address->s6_bytes[1] & 0xf);
}

MSTCPIP_INLINE
SCOPE_LEVEL
Ipv6AddressScope(
    _In_reads_(_Inexpressible_("Varies")) CONST UCHAR *Address
    )
/*++

Routine Description:

    Examines an IPv6 address and determines its scope.

Arguments:

    Address - Supplies an IPv6 address.

Return Value:

    Returns the scope level of the address.

Caller IRQL:

    May be called at PASSIVE through DISPATCH level.

--*/
{
    if (IN6_IS_ADDR_MULTICAST((CONST IN6_ADDR *) Address)) {
        return IN6_MULTICAST_SCOPE(Address);
    } else {
        return Ipv6UnicastAddressScope(Address);
    }
}

MSTCPIP_INLINE
NL_ADDRESS_TYPE
Ipv6AddressType(
    _In_reads_(_Inexpressible_("Varies")) CONST UCHAR *Address
    )
{
    CONST IN6_ADDR *Ipv6Address = (CONST IN6_ADDR *) Address;
    CONST UCHAR *Ipv4Address;

    if (IN6_IS_ADDR_MULTICAST(Ipv6Address)) {
        return NlatMulticast;
    }

    if (IN6_IS_ADDR_UNSPECIFIED(Ipv6Address)) {
        return NlatUnspecified;
    }

    //
    // Extract embedded IPv4 address, if any.
    //
    if (IN6_IS_ADDR_ISATAP(Ipv6Address) ||
        IN6_IS_ADDR_V4COMPAT(Ipv6Address) ||
        IN6_IS_ADDR_V4MAPPED(Ipv6Address) ||
        IN6_IS_ADDR_V4TRANSLATED(Ipv6Address)) {
        Ipv4Address = IN6_EXTRACT_V4ADDR_FROM_ISATAP(Ipv6Address);
    } else if (IN6_IS_ADDR_6TO4(Ipv6Address)) {
        Ipv4Address = IN6_EXTRACT_V4ADDR_FROM_6TO4(Ipv6Address);
    } else {
        //
        // Anycast and loopback addresses are treated unicast address.
        //
        return NlatUnicast;
    }

    //
    // Ensure that the embedded IPv4 address is unicast.
    //
    if (Ipv4AddressType(Ipv4Address) != NlatUnicast) {
        return NlatInvalid;
    }
    return NlatUnicast;
}

MSTCPIP_INLINE
VOID
IN6_UNCANONICALIZE_SCOPE_ID(
    _In_ CONST IN6_ADDR *Address,
    _Inout_ SCOPE_ID *ScopeId
    )
{
    SCOPE_LEVEL ScopeLevel = Ipv6AddressScope((CONST UCHAR *)Address);

    if ((IN6_IS_ADDR_LOOPBACK(Address)) || (ScopeLevel == ScopeLevelGlobal)) {
        ScopeId->Value = 0;
    }

    if ((SCOPE_LEVEL)ScopeId->Level == ScopeLevel) {
        ScopeId->Level = 0;
    }
}

#if (NTDDI_VERSION >= NTDDI_VISTA)
MSTCPIP_INLINE
VOID
IN6ADDR_SETSOCKADDR(_Out_ PSOCKADDR_IN6 a, _In_ CONST IN6_ADDR *addr, _In_ SCOPE_ID scope,
                    _In_ USHORT port)
{
    a->sin6_family = AF_INET6;
    a->sin6_port = port;
    a->sin6_flowinfo = 0;
    RtlCopyMemory(&a->sin6_addr, addr, sizeof(IN6_ADDR));
    a->sin6_scope_struct = scope;
    IN6_UNCANONICALIZE_SCOPE_ID(&a->sin6_addr, &a->sin6_scope_struct);
}

MSTCPIP_INLINE
VOID
IN6ADDR_SETV4MAPPED(_Out_ PSOCKADDR_IN6 a6, _In_ CONST IN_ADDR* a4, _In_ SCOPE_ID scope,
                    _In_ USHORT port)
{
    a6->sin6_family = AF_INET6;
    a6->sin6_port = port;
    a6->sin6_flowinfo = 0;
    IN6_SET_ADDR_V4MAPPED(&a6->sin6_addr, a4);
    a6->sin6_scope_struct = scope;
    IN4_UNCANONICALIZE_SCOPE_ID(a4, &a6->sin6_scope_struct);
}

#endif

//
// Define address-family-independent routines.
//

MSTCPIP_INLINE
BOOLEAN
INET_ADDR_EQUAL(_In_ ADDRESS_FAMILY af, _In_ CONST VOID* a, _In_ CONST VOID* b)
{
    if (af == AF_INET6) {
        return IN6_ADDR_EQUAL((CONST IN6_ADDR*)a, (CONST IN6_ADDR*)b);
    } else {
        ASSERT(af == AF_INET);
        return IN4_ADDR_EQUAL((CONST IN_ADDR*)a, (CONST IN_ADDR*)b);
    }
}

MSTCPIP_INLINE
BOOLEAN
INET_UNALIGNED_ADDR_EQUAL(_In_ ADDRESS_FAMILY af, _In_ CONST VOID* a, _In_ CONST VOID* b)
{
    if (af == AF_INET6) {
        return IN6_ADDR_EQUAL((CONST IN6_ADDR*)a, (CONST IN6_ADDR*)b);
    } else {
        ASSERT(af == AF_INET);
        return IN4_UNALIGNED_ADDR_EQUAL((CONST IN_ADDR*)a, (CONST IN_ADDR*)b);
    }
}

MSTCPIP_INLINE
BOOLEAN
INET_IS_ADDR_UNSPECIFIED(_In_ ADDRESS_FAMILY af, _In_ CONST VOID* a)
{
    if (af == AF_INET6) {
        return IN6_IS_ADDR_UNSPECIFIED((CONST IN6_ADDR*)a);
    } else {
        ASSERT(af == AF_INET);
        return IN4_IS_ADDR_UNSPECIFIED((CONST IN_ADDR*)a);
    }
}

MSTCPIP_INLINE
BOOLEAN
INET_IS_UNALIGNED_ADDR_UNSPECIFIED(_In_ ADDRESS_FAMILY af, _In_ CONST VOID* a)
{
    if (af == AF_INET6) {
        return IN6_IS_ADDR_UNSPECIFIED((CONST IN6_ADDR*)a);
    } else {
        ASSERT(af == AF_INET);
        return IN4_IS_UNALIGNED_ADDR_UNSPECIFIED((CONST IN_ADDR UNALIGNED*)a);
    }
}

MSTCPIP_INLINE
BOOLEAN
INET_IS_ADDR_LOOPBACK(_In_ ADDRESS_FAMILY af, _In_ CONST VOID* a)
{
    if (af == AF_INET6) {
        return IN6_IS_ADDR_LOOPBACK((CONST IN6_ADDR*)a);
    } else {
        ASSERT(af == AF_INET);
        return IN4_IS_ADDR_LOOPBACK((CONST IN_ADDR*)a);
    }
}

MSTCPIP_INLINE
BOOLEAN
INET_IS_ADDR_BROADCAST(_In_ ADDRESS_FAMILY af, _In_ CONST VOID* a)
{
    if (af == AF_INET6) {
        return FALSE;
    } else {
        ASSERT(af == AF_INET);
        return IN4_IS_ADDR_BROADCAST((CONST IN_ADDR*)a);
    }
}

MSTCPIP_INLINE
BOOLEAN
INET_IS_ADDR_MULTICAST(_In_ ADDRESS_FAMILY af, _In_ CONST VOID* a)
{
    if (af == AF_INET6) {
        return IN6_IS_ADDR_MULTICAST((CONST IN6_ADDR*)a);
    } else {
        ASSERT(af == AF_INET);
        return IN4_IS_ADDR_MULTICAST((CONST IN_ADDR*)a);
    }
}

MSTCPIP_INLINE
CONST UCHAR*
INET_ADDR_UNSPECIFIED(_In_ ADDRESS_FAMILY af)
{
    if (af == AF_INET6) {
        return (CONST UCHAR*)&in6addr_any;
    } else {
        ASSERT(af == AF_INET);
        return (CONST UCHAR*)&in4addr_any;
    }
}

MSTCPIP_INLINE
VOID
INET_SET_ADDRESS(
    _In_ ADDRESS_FAMILY Family,
    _Out_writes_(_Inexpressible_("Varies")) PUCHAR Address,
    _In_reads_(_Inexpressible_("Varies")) CONST UCHAR *Value)
{
    if (Family == AF_INET6) {
        *((PIN6_ADDR)Address) = *((PIN6_ADDR)Value);
    } else {
        ASSERT(Family == AF_INET);
        *((PIN_ADDR)Address) = *((PIN_ADDR)Value);
    }
}

_Ret_range_(sizeof(IN_ADDR), sizeof(IN6_ADDR))
MSTCPIP_INLINE
SIZE_T
INET_ADDR_LENGTH(_In_ ADDRESS_FAMILY af)
{
    if (af == AF_INET6) {
        return sizeof(IN6_ADDR);
    } else {
        ASSERT(af == AF_INET);
        return sizeof(IN_ADDR);
    }
}

_Ret_range_(sizeof(SOCKADDR_IN), sizeof(SOCKADDR_IN6))
MSTCPIP_INLINE
SIZE_T
INET_SOCKADDR_LENGTH(_In_ ADDRESS_FAMILY af)
{
    if (af == AF_INET6) {
        return sizeof(SOCKADDR_IN6);
    } else {
        ASSERT(af == AF_INET);
        return sizeof(SOCKADDR_IN);
    }
}

#if (NTDDI_VERSION >= NTDDI_VISTA)
_At_((PSOCKADDR_IN6)a, _When_(af == AF_INET6, _Out_))
MSTCPIP_INLINE
VOID
INETADDR_SETSOCKADDR(_In_ ADDRESS_FAMILY af, _Out_ PSOCKADDR a, _In_ CONST VOID* addr,
                     _In_ SCOPE_ID scope, _In_ USHORT port)
{
    if (af == AF_INET6) {
#pragma prefast (suppress: 6386, "Suppressing due to PREFast bug with _When_")
        IN6ADDR_SETSOCKADDR(
            (PSOCKADDR_IN6) a, (CONST IN6_ADDR *) addr, scope, port);
    } else {
        CONST IN_ADDR addr4 = *((IN_ADDR UNALIGNED *) addr);

        ASSERT(af == AF_INET);
        IN4ADDR_SETSOCKADDR(
            (PSOCKADDR_IN) a, (CONST IN_ADDR *) &addr4, port);
    }
}
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

MSTCPIP_INLINE
VOID
INETADDR_SETANY(_Inout_ PSOCKADDR a)
{
    if (a->sa_family == AF_INET6) {
        IN6ADDR_SETANY((PSOCKADDR_IN6)a);
    } else {
        ASSERT(a->sa_family == AF_INET);
        IN4ADDR_SETANY((PSOCKADDR_IN)a);
    }
}

MSTCPIP_INLINE
VOID
INETADDR_SETLOOPBACK(_Inout_ PSOCKADDR a)
{
    if (a->sa_family == AF_INET6) {
        IN6ADDR_SETLOOPBACK((PSOCKADDR_IN6)a);
    } else {
        ASSERT(a->sa_family == AF_INET);
        IN4ADDR_SETLOOPBACK((PSOCKADDR_IN)a);
    }
}

MSTCPIP_INLINE
BOOLEAN
INETADDR_ISANY(_In_ CONST SOCKADDR *a)
{
    if (a->sa_family == AF_INET6) {
        return IN6ADDR_ISANY((CONST SOCKADDR_IN6*)a);
    } else {
        ASSERT(a->sa_family == AF_INET);
        return IN4ADDR_ISANY((CONST SOCKADDR_IN*)a);
    }
}

MSTCPIP_INLINE
BOOLEAN
INETADDR_ISLOOPBACK(_In_ CONST SOCKADDR *a)
{
    if (a->sa_family == AF_INET6) {
        return IN6ADDR_ISLOOPBACK((CONST SOCKADDR_IN6*)a);
    } else {
        ASSERT(a->sa_family == AF_INET);
        return IN4ADDR_ISLOOPBACK((CONST SOCKADDR_IN*)a);
    }
}

MSTCPIP_INLINE
BOOLEAN
INETADDR_ISV4MAPPED(_In_ CONST SOCKADDR *a)
{
    if (a->sa_family == AF_INET6) {
        return IN6ADDR_ISV4MAPPED((CONST SOCKADDR_IN6*)a);
    } else {
        return FALSE;
    }
}

MSTCPIP_INLINE
BOOLEAN
NL_ADDR_EQUAL(
    _In_ ADDRESS_FAMILY af,
    _In_ SCOPE_ID sa,
    _In_ CONST UCHAR* aa,
    _In_ SCOPE_ID sb,
    _In_ CONST UCHAR* ab
    )
{
    return (BOOLEAN)((sa.Value == sb.Value) && INET_ADDR_EQUAL(af, aa, ab));
}

MSTCPIP_INLINE
BOOLEAN
NL_IS_ADDR_UNSPECIFIED(
    _In_ ADDRESS_FAMILY af,
    _In_ SCOPE_ID s,
    _In_ CONST UCHAR* a
    )
{
    return (BOOLEAN)((s.Value == 0) && INET_IS_ADDR_UNSPECIFIED(af, a));
}

MSTCPIP_INLINE
BOOLEAN
INETADDR_ISEQUAL(_In_ CONST SOCKADDR *a, _In_ CONST SOCKADDR *b)
{
    if (a->sa_family == AF_INET6) {
        return (BOOLEAN)
            (b->sa_family == AF_INET6 &&
             IN6ADDR_ISEQUAL((CONST SOCKADDR_IN6*)a, (CONST SOCKADDR_IN6*)b));
    } else {
        ASSERT(a->sa_family == AF_INET);
        return (BOOLEAN)
            (b->sa_family == AF_INET &&
             IN4ADDR_ISEQUAL((CONST SOCKADDR_IN*)a, (CONST SOCKADDR_IN*)b));
    }
}

MSTCPIP_INLINE
BOOLEAN
INETADDR_ISUNSPECIFIED(_In_ CONST SOCKADDR *a)
{
    if (a->sa_family == AF_INET6) {
        return IN6ADDR_ISUNSPECIFIED((CONST SOCKADDR_IN6*)a);
    } else {
        ASSERT(a->sa_family == AF_INET);
        return IN4ADDR_ISUNSPECIFIED((CONST SOCKADDR_IN*)a);
    }
}

#if (NTDDI_VERSION >= NTDDI_VISTA)
MSTCPIP_INLINE
SCOPE_ID
INETADDR_SCOPE_ID(_In_ CONST SOCKADDR *a)
{
    if (a->sa_family == AF_INET6) {
        return ((CONST SOCKADDR_IN6*)a)->sin6_scope_struct;
    } else {
        ASSERT(a->sa_family == AF_INET);
        return IN4ADDR_SCOPE_ID((CONST SOCKADDR_IN*)a);
    }
}
#endif

MSTCPIP_INLINE
USHORT
INETADDR_PORT(_In_ CONST SOCKADDR *a)
{
    if (a->sa_family == AF_INET6) {
        return ((CONST SOCKADDR_IN6*)a)->sin6_port;
    } else {
        ASSERT(a->sa_family == AF_INET);
        return ((CONST SOCKADDR_IN*)a)->sin_port;
    }
}

MSTCPIP_INLINE
PUCHAR
INETADDR_ADDRESS(_In_ CONST SOCKADDR* a)
{
    if (a->sa_family == AF_INET6) {
        return (PUCHAR)&((PSOCKADDR_IN6)a)->sin6_addr;
    } else {
        ASSERT(a->sa_family == AF_INET);
        return (PUCHAR)&((PSOCKADDR_IN)a)->sin_addr;
    }
}

MSTCPIP_INLINE
VOID
INETADDR_SET_PORT(_Inout_ PSOCKADDR a, _In_ USHORT Port)
{
    SS_PORT(a) = Port;
}

MSTCPIP_INLINE
VOID
INETADDR_SET_ADDRESS(
    _Inout_ PSOCKADDR a,
    _In_reads_(_Inexpressible_("Varies")) CONST UCHAR *Address)
{
    if (a->sa_family == AF_INET6) {
        ((PSOCKADDR_IN6)a)->sin6_addr = *((CONST IN6_ADDR*)Address);
    } else {
        ASSERT(a->sa_family == AF_INET);
        ((PSOCKADDR_IN)a)->sin_addr = *((CONST IN_ADDR*)Address);
    }
}

MSTCPIP_INLINE
VOID
INET_UNCANONICALIZE_SCOPE_ID(
    _In_ ADDRESS_FAMILY AddressFamily,
    _In_reads_(_Inexpressible_("Varies")) CONST UCHAR *Address,
    _Inout_ SCOPE_ID *ScopeId
    )
{
    if (AddressFamily == AF_INET6) {
        IN6_UNCANONICALIZE_SCOPE_ID((CONST IN6_ADDR*) Address, ScopeId);
    } else {
        IN4_UNCANONICALIZE_SCOPE_ID((CONST IN_ADDR*) Address, ScopeId);
    }
}

#endif // _WS2IPDEF_

#endif // (NTDDI_VERSION >= NTDDI_WIN2KSP1)


#ifndef __IP2STRING__
#define __IP2STRING__

#if (NTDDI_VERSION >= NTDDI_VISTA)

//
//  Some simple Rtl routines for IP address <-> string literal conversion
//
#ifdef _WS2DEF_

#pragma region Desktop Family or Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

NTSYSAPI
PSTR
NTAPI
RtlIpv4AddressToStringA (
    _In_ const struct in_addr *Addr,
    _Out_writes_(16) PSTR S
    );

NTSYSAPI
LONG
NTAPI
RtlIpv4AddressToStringExA(
    _In_ const struct in_addr *Address,
    _In_ USHORT Port,
    _Out_writes_to_(*AddressStringLength, *AddressStringLength) PSTR AddressString,
    _Inout_ PULONG AddressStringLength
    );

NTSYSAPI
PWSTR
NTAPI
RtlIpv4AddressToStringW (
    _In_ const struct in_addr *Addr,
    _Out_writes_(16) PWSTR S
    );

NTSYSAPI
LONG
NTAPI
RtlIpv4AddressToStringExW(
    _In_ const struct in_addr *Address,
    _In_ USHORT Port,
    _Out_writes_to_(*AddressStringLength, *AddressStringLength) PWSTR AddressString,
    _Inout_ PULONG AddressStringLength
    );

NTSYSAPI
LONG
NTAPI
RtlIpv4StringToAddressA (
    _In_ PCSTR S,
    _In_ BOOLEAN Strict,
    _Out_ PCSTR *Terminator,
    _Out_ struct in_addr *Addr
    );

NTSYSAPI
LONG
NTAPI
RtlIpv4StringToAddressExA (
    _In_ PCSTR AddressString,
    _In_ BOOLEAN Strict,
    _Out_ struct in_addr *Address,
    _Out_ PUSHORT Port
    );

NTSYSAPI
LONG
NTAPI
RtlIpv4StringToAddressW (
    _In_ PCWSTR S,
    _In_ BOOLEAN Strict,
    _Out_ LPCWSTR *Terminator,
    _Out_ struct in_addr *Addr
    );

NTSYSAPI
LONG
NTAPI
RtlIpv4StringToAddressExW (
    _In_ PCWSTR AddressString,
    _In_ BOOLEAN Strict,
    _Out_ struct in_addr *Address,
    _Out_ PUSHORT Port
    );

#ifdef UNICODE
#define RtlIpv4AddressToString RtlIpv4AddressToStringW
#define RtlIpv4StringToAddress RtlIpv4StringToAddressW
#define RtlIpv4AddressToStringEx RtlIpv4AddressToStringExW
#define RtlIpv4StringToAddressEx RtlIpv4StringToAddressExW
#else
#define RtlIpv4AddressToString RtlIpv4AddressToStringA
#define RtlIpv4StringToAddress RtlIpv4StringToAddressA
#define RtlIpv4AddressToStringEx RtlIpv4AddressToStringExA
#define RtlIpv4StringToAddressEx RtlIpv4StringToAddressExA
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */

#endif //_WS2DEF_

#ifdef _WS2IPDEF_

#pragma region Desktop Family or Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

NTSYSAPI
PSTR
NTAPI
RtlIpv6AddressToStringA (
    _In_ const struct in6_addr *Addr,
    _Out_writes_(46) PSTR S
    );

NTSYSAPI
LONG
NTAPI
RtlIpv6AddressToStringExA(
    _In_ const struct in6_addr *Address,
    _In_ ULONG ScopeId,
    _In_ USHORT Port,
    _Out_writes_to_(*AddressStringLength, *AddressStringLength) PSTR AddressString,
    _Inout_ PULONG AddressStringLength
    );

NTSYSAPI
PWSTR
NTAPI
RtlIpv6AddressToStringW (
    _In_ const struct in6_addr *Addr,
    _Out_writes_(46) PWSTR S
    );

NTSYSAPI
LONG
NTAPI
RtlIpv6AddressToStringExW(
    _In_ const struct in6_addr *Address,
    _In_ ULONG ScopeId,
    _In_ USHORT Port,
    _Out_writes_to_(*AddressStringLength, *AddressStringLength) PWSTR AddressString,
    _Inout_ PULONG AddressStringLength
    );

NTSYSAPI
LONG
NTAPI
RtlIpv6StringToAddressA (
    _In_ PCSTR S,
    _Out_ PCSTR *Terminator,
    _Out_ struct in6_addr *Addr
    );

NTSYSAPI
LONG
NTAPI
RtlIpv6StringToAddressExA (
    _In_ PCSTR AddressString,
    _Out_ struct in6_addr *Address,
    _Out_ PULONG ScopeId,
    _Out_ PUSHORT Port
    );

NTSYSAPI
LONG
NTAPI
RtlIpv6StringToAddressW (
    _In_ PCWSTR S,
    _Out_ PCWSTR *Terminator,
    _Out_ struct in6_addr *Addr
    );

NTSYSAPI
LONG
NTAPI
RtlIpv6StringToAddressExW (
    _In_ PCWSTR AddressString,
    _Out_ struct in6_addr *Address,
    _Out_ PULONG ScopeId,
    _Out_ PUSHORT Port
    );

#ifdef UNICODE
#define RtlIpv6AddressToString RtlIpv6AddressToStringW
#define RtlIpv6StringToAddress RtlIpv6StringToAddressW
#define RtlIpv6StringToAddressEx RtlIpv6StringToAddressExW
#define RtlIpv6AddressToStringEx RtlIpv6AddressToStringExW
#else
#define RtlIpv6AddressToString RtlIpv6AddressToStringA
#define RtlIpv6StringToAddress RtlIpv6StringToAddressA
#define RtlIpv6StringToAddressEx RtlIpv6StringToAddressExA
#define RtlIpv6AddressToStringEx RtlIpv6AddressToStringExA
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */

#endif // __WS2IPDEF__

//
//  Some simple Rtl routines for Ethernet address <-> string literal conversion
//
#ifdef _WS2DEF_

#pragma region Desktop Family or Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

union _DL_EUI48;
typedef union _DL_EUI48 DL_EUI48, *PDL_EUI48;

NTSYSAPI
PSTR
NTAPI
RtlEthernetAddressToStringA (
    _In_ const DL_EUI48 *Addr,
    _Out_writes_(18) PSTR S
    );

NTSYSAPI
PWSTR
NTAPI
RtlEthernetAddressToStringW (
    _In_ const DL_EUI48 *Addr,
    _Out_writes_(18) PWSTR S
    );

NTSYSAPI
LONG
NTAPI
RtlEthernetStringToAddressA (
    _In_ PCSTR S,
    _Out_ PCSTR *Terminator,
    _Out_ DL_EUI48 *Addr
    );

NTSYSAPI
LONG
NTAPI
RtlEthernetStringToAddressW (
    _In_ PCWSTR S,
    _Out_ LPCWSTR *Terminator,
    _Out_ DL_EUI48 *Addr
    );

#ifdef UNICODE
#define RtlEthernetAddressToString RtlEthernetAddressToStringW
#define RtlEthernetStringToAddress RtlEthernetStringToAddressW
#else
#define RtlEthernetAddressToString RtlEthernetAddressToStringA
#define RtlEthernetStringToAddress RtlEthernetStringToAddressA
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */

#endif //_WS2DEF_

#endif // (NTDDI >= NTDDI_VISTA)
#endif // __IP2STRING__

#ifdef _PREFAST_
#pragma prefast(pop)
#endif

#pragma warning(pop)

#ifdef MSTCPIPDEF_ASSERT_UNDEFINED
#undef ASSERT
#endif

#ifdef __cplusplus
}
#endif


#endif // GUID_DEFS_ONLY

#endif // !_MSTCPIP_

