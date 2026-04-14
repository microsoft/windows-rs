/*++

Copyright (c) 2000-2001  Microsoft Corporation

Module Name:

    tcpestats.w

Abstract:

    This module contains the definitions and structures for TCP extended
    statistics.

Author:

    Xinyan Zan (xinyanz) 30-June-2006

Environment:

    User mode and kernel mode

--*/
#ifndef _TCPESTATS_
#define _TCPESTATS_
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


//
// Please don't change the order of this enum. The order defined in this
// enum needs to match the order in EstatsToTcpObjectMappingTable.
//
typedef enum {
    TcpConnectionEstatsSynOpts,
    TcpConnectionEstatsData,
    TcpConnectionEstatsSndCong,
    TcpConnectionEstatsPath,
    TcpConnectionEstatsSendBuff,
    TcpConnectionEstatsRec,
    TcpConnectionEstatsObsRec,
    TcpConnectionEstatsBandwidth,
    TcpConnectionEstatsFineRtt,
    TcpConnectionEstatsMaximum,
} TCP_ESTATS_TYPE, *PTCP_ESTATS_TYPE;

//
// TCP_BOOLEAN_OPTIONAL
//
// Define the states that a caller can specify when updating a boolean field.
//

typedef enum _TCP_BOOLEAN_OPTIONAL {
    TcpBoolOptDisabled = 0,
    TcpBoolOptEnabled,
    TcpBoolOptUnchanged = -1
} TCP_BOOLEAN_OPTIONAL, *PTCP_BOOLEAN_OPTIONAL;

//
// TCP_ESTATS_SYN_OPTS_ROS
//
// Define extended SYN-exchange information maintained for TCP connections.
//

typedef struct _TCP_ESTATS_SYN_OPTS_ROS_v0 {
    BOOLEAN ActiveOpen;
    ULONG MssRcvd;
    ULONG MssSent;
} TCP_ESTATS_SYN_OPTS_ROS_v0, *PTCP_ESTATS_SYN_OPTS_ROS_v0;


//
// TCP_SOFT_ERROR
//
// Enumerate the non-fatal errors recorded on each connection.
//

typedef enum {
    TcpErrorNone = 0,
    TcpErrorBelowDataWindow,
    TcpErrorAboveDataWindow,
    TcpErrorBelowAckWindow,
    TcpErrorAboveAckWindow,
    TcpErrorBelowTsWindow,
    TcpErrorAboveTsWindow,
    TcpErrorDataChecksumError,
    TcpErrorDataLengthError,
    TcpErrorMaxSoftError
} TCP_SOFT_ERROR, *PTCP_SOFT_ERROR;

//
// TCP_ESTATS_DATA_ROD
//
// Define extended data-transfer information for TCP connections.
//

typedef struct _TCP_ESTATS_DATA_ROD_v0 {
    ULONG64 DataBytesOut;
    ULONG64 DataSegsOut;
    ULONG64 DataBytesIn;
    ULONG64 DataSegsIn;
    ULONG64 SegsOut;
    ULONG64 SegsIn;
    ULONG SoftErrors;
    ULONG SoftErrorReason;
    ULONG SndUna;
    ULONG SndNxt;
    ULONG SndMax;
    ULONG64 ThruBytesAcked;
    ULONG RcvNxt;
    ULONG64 ThruBytesReceived;
} TCP_ESTATS_DATA_ROD_v0, *PTCP_ESTATS_DATA_ROD_v0;

//
// TCP_ESTATS_DATA_RW
//
// Define structure for enabling extended data-transfer information.
//

typedef struct _TCP_ESTATS_DATA_RW_v0 {
    BOOLEAN EnableCollection;
} TCP_ESTATS_DATA_RW_v0, *PTCP_ESTATS_DATA_RW_v0;


//
// TCP_ESTATS_SND_CONG_ROD
//
// Define extended sender-congestion information for TCP connections.
//

typedef struct _TCP_ESTATS_SND_CONG_ROD_v0 {
    ULONG SndLimTransRwin;
    ULONG SndLimTimeRwin;
    SIZE_T SndLimBytesRwin;
    ULONG SndLimTransCwnd;
    ULONG SndLimTimeCwnd;
    SIZE_T SndLimBytesCwnd;
    ULONG SndLimTransSnd;
    ULONG SndLimTimeSnd;
    SIZE_T SndLimBytesSnd;
    ULONG SlowStart;
    ULONG CongAvoid;
    ULONG OtherReductions;
    ULONG CurCwnd;
    ULONG MaxSsCwnd;
    ULONG MaxCaCwnd;
    ULONG CurSsthresh;
    ULONG MaxSsthresh;
    ULONG MinSsthresh;
} TCP_ESTATS_SND_CONG_ROD_v0, *PTCP_ESTATS_SND_CONG_ROD_v0;

//
// TCP_ESTATS_SND_CONG_ROS
//
// Define static extended sender-congestion information for TCP connections.

typedef struct _TCP_ESTATS_SND_CONG_ROS_v0 {
    ULONG LimCwnd;
} TCP_ESTATS_SND_CONG_ROS_v0, *PTCP_ESTATS_SND_CONG_ROS_v0;

//
// TCP_ESTATS_SND_CONG_RW
//
// Define structure for enabling extended sender-congestion information.
//

typedef struct _TCP_ESTATS_SND_CONG_RW_v0 {
    BOOLEAN EnableCollection;
} TCP_ESTATS_SND_CONG_RW_v0, *PTCP_ESTATS_SND_CONG_RW_v0;

//
// TCP_ESTATS_PATH_ROD
//
// Define extended path-measurement information for TCP connections.
//

typedef struct _TCP_ESTATS_PATH_ROD_v0 {
    ULONG FastRetran;
    ULONG Timeouts;
    ULONG SubsequentTimeouts;
    ULONG CurTimeoutCount;
    ULONG AbruptTimeouts;
    ULONG PktsRetrans;
    ULONG BytesRetrans;
    ULONG DupAcksIn;
    ULONG SacksRcvd;
    ULONG SackBlocksRcvd;
    ULONG CongSignals;
    ULONG PreCongSumCwnd;
    ULONG PreCongSumRtt;
    ULONG PostCongSumRtt;
    ULONG PostCongCountRtt;
    ULONG EcnSignals;
    ULONG EceRcvd;
    ULONG SendStall;
    ULONG QuenchRcvd;
    ULONG RetranThresh;
    ULONG SndDupAckEpisodes;
    ULONG SumBytesReordered;
    ULONG NonRecovDa;
    ULONG NonRecovDaEpisodes;
    ULONG AckAfterFr;
    ULONG DsackDups;
    ULONG SampleRtt;
    ULONG SmoothedRtt;
    ULONG RttVar;
    ULONG MaxRtt;
    ULONG MinRtt;
    ULONG SumRtt;
    ULONG CountRtt;
    ULONG CurRto;
    ULONG MaxRto;
    ULONG MinRto;
    ULONG CurMss;
    ULONG MaxMss;
    ULONG MinMss;
    ULONG SpuriousRtoDetections;
} TCP_ESTATS_PATH_ROD_v0, *PTCP_ESTATS_PATH_ROD_v0;

//
// TCP_ESTATS_PATH_ROS
//
// Define structure for enabling path-measurement information.
//

typedef struct _TCP_ESTATS_PATH_RW_v0 {
    BOOLEAN EnableCollection;
} TCP_ESTATS_PATH_RW_v0, *PTCP_ESTATS_PATH_RW_v0;

//
// TCP_ESTATS_SEND_BUFF_ROD
//
// Define extended output-queuing information for TCP connections.
//

typedef struct _TCP_ESTATS_SEND_BUFF_ROD_v0 {
    SIZE_T CurRetxQueue;
    SIZE_T MaxRetxQueue;
    SIZE_T CurAppWQueue;
    SIZE_T MaxAppWQueue;
} TCP_ESTATS_SEND_BUFF_ROD_v0, *PTCP_ESTATS_SEND_BUFF_ROD_v0;

//
// TCP_ESTATS_SEND_BUFF_RW
//
// Define structure for enabling output-queuing information.
//

typedef struct _TCP_ESTATS_SEND_BUFF_RW_v0 {
    BOOLEAN EnableCollection;
} TCP_ESTATS_SEND_BUFF_RW_v0, *PTCP_ESTATS_SEND_BUFF_RW_v0;


//
// TCP_ESTATS_REC_ROD
//
// Define extended local-receiver information for TCP connections.
//

typedef struct _TCP_ESTATS_REC_ROD_v0 {
    ULONG CurRwinSent;
    ULONG MaxRwinSent;
    ULONG MinRwinSent;
    ULONG LimRwin;
    ULONG DupAckEpisodes;
    ULONG DupAcksOut;
    ULONG CeRcvd;
    ULONG EcnSent;
    ULONG EcnNoncesRcvd;
    ULONG CurReasmQueue;
    ULONG MaxReasmQueue;
    SIZE_T CurAppRQueue;
    SIZE_T MaxAppRQueue;
    UCHAR WinScaleSent;
} TCP_ESTATS_REC_ROD_v0, *PTCP_ESTATS_REC_ROD_v0;

//
// TCP_ESTATS_REC_RW
//
// Define structure for enabling local-receiver information.
//

typedef struct _TCP_ESTATS_REC_RW_v0 {
    BOOLEAN EnableCollection;
} TCP_ESTATS_REC_RW_v0, *PTCP_ESTATS_REC_RW_v0;


//
// TCP_ESTATS_OBS_REC_ROD
//
// Define extended remote-receiver information for TCP connections.
//

typedef struct _TCP_ESTATS_OBS_REC_ROD_v0 {
    ULONG CurRwinRcvd;
    ULONG MaxRwinRcvd;
    ULONG MinRwinRcvd;
    UCHAR WinScaleRcvd;
} TCP_ESTATS_OBS_REC_ROD_v0, *PTCP_ESTATS_OBS_REC_ROD_v0;


//
// TCP_ESTATS_OBS_REC_RW
//
// Define structure for enabling remote-receiver information.
//

typedef struct _TCP_ESTATS_OBS_REC_RW_v0 {
    BOOLEAN EnableCollection;
} TCP_ESTATS_OBS_REC_RW_v0, *PTCP_ESTATS_OBS_REC_RW_v0;


//
// TCP_ESTATS_BW_RW
//
// Define the structure for enabling bandwidth estimation for TCP connections.
//

typedef struct _TCP_ESTATS_BANDWIDTH_RW_v0 {
    TCP_BOOLEAN_OPTIONAL EnableCollectionOutbound;
    TCP_BOOLEAN_OPTIONAL EnableCollectionInbound;
} TCP_ESTATS_BANDWIDTH_RW_v0, *PTCP_ESTATS_BANDWIDTH_RW_v0;

//
// TCP_ESTATS_BW_ROD
//
// Define bandwidth estimation statistics for TCP connections.
//
// Bandwidth and Instability metrics are expressed as bits per second.
//

typedef struct _TCP_ESTATS_BANDWIDTH_ROD_v0 {
    ULONG64 OutboundBandwidth;
    ULONG64 InboundBandwidth;
    ULONG64 OutboundInstability;
    ULONG64 InboundInstability;
    BOOLEAN OutboundBandwidthPeaked;
    BOOLEAN InboundBandwidthPeaked;
} TCP_ESTATS_BANDWIDTH_ROD_v0, *PTCP_ESTATS_BANDWIDTH_ROD_v0;

//
// TCP_ESTATS_FINE_RTT_RW
//
// Define the structure for enabling fine-grained RTT estimation for TCP
// connections.
//

typedef struct _TCP_ESTATS_FINE_RTT_RW_v0 {
    BOOLEAN EnableCollection;
} TCP_ESTATS_FINE_RTT_RW_v0, *PTCP_ESTATS_FINE_RTT_RW_v0;


//
// TCP_ESTATS_FINE_RTT_ROD
//
// Define fine-grained RTT estimation statistics for TCP connections.
//

typedef struct _TCP_ESTATS_FINE_RTT_ROD_v0 {
    ULONG RttVar;
    ULONG MaxRtt;
    ULONG MinRtt;
    ULONG SumRtt;
} TCP_ESTATS_FINE_RTT_ROD_v0, *PTCP_ESTATS_FINE_RTT_ROD_v0;


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // _TCPESTATS_
