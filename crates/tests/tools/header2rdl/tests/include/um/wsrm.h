/*	
**	wsrm.h - winsock extension for Reliable Multicast (RMCast) Transport
**
**	This file contains PGM specific information for use by WinSock2 compatible
**  applications that need Reliable Multicast Transport.
**
**  Copyright (c) Microsoft Corporation. All rights reserved.
**
**	Created: Mar 12, 2000
**
*/

#ifndef _WSRM_H_
#define _WSRM_H_

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define IPPROTO_RM      113
#define MAX_MCAST_TTL   255

//
// options for setsockopt, getsockopt
//
#define RM_OPTIONSBASE      1000

// Set/Query rate (Kb/Sec) + window size (Kb and/or MSec) -- described by RM_SEND_WINDOW below
#define RM_RATE_WINDOW_SIZE             (RM_OPTIONSBASE + 1)

// Set the size of the next message -- (ULONG)
#define RM_SET_MESSAGE_BOUNDARY         (RM_OPTIONSBASE + 2)

// flush the entire data (window) right now -- not implemented
#define RM_FLUSHCACHE                   (RM_OPTIONSBASE + 3)

// Set or Query the window advance method on the sender -- methods enumerated in eWINDOW_ADVANCE_METHOD
#define RM_SENDER_WINDOW_ADVANCE_METHOD (RM_OPTIONSBASE + 4)

// get sender statistics
#define RM_SENDER_STATISTICS            (RM_OPTIONSBASE + 5)

// allow a late-joiner to NAK any packet upto the lowest sequence Id
#define RM_LATEJOIN                     (RM_OPTIONSBASE + 6)

// set IP multicast outgoing interface
#define RM_SET_SEND_IF                  (RM_OPTIONSBASE + 7)

// add IP multicast incoming interface
#define RM_ADD_RECEIVE_IF               (RM_OPTIONSBASE + 8)

// delete IP multicast incoming interface
#define RM_DEL_RECEIVE_IF               (RM_OPTIONSBASE + 9)

// Set/Query the Window's Advance rate (has to be less that MAX_WINDOW_INCREMENT_PERCENTAGE)
#define RM_SEND_WINDOW_ADV_RATE         (RM_OPTIONSBASE + 10)

// Instruct to use parity-based forward error correction schemes
#define RM_USE_FEC                      (RM_OPTIONSBASE + 11)

// Set the Ttl of the MCast packets -- (ULONG)
#define RM_SET_MCAST_TTL                (RM_OPTIONSBASE + 12)

// get receiver statistics
#define RM_RECEIVER_STATISTICS          (RM_OPTIONSBASE + 13)

// get receiver statistics
#define RM_HIGH_SPEED_INTRANET_OPT      (RM_OPTIONSBASE + 14)

//==============================================================
//
// Definitions
//
#define     SENDER_DEFAULT_RATE_KBITS_PER_SEC        56             // 56 Kbits/Sec
#define     SENDER_DEFAULT_WINDOW_SIZE_BYTES         10 *1000*1000  // 10 Megs

#define     SENDER_DEFAULT_WINDOW_ADV_PERCENTAGE     15             // 15%
#define     MAX_WINDOW_INCREMENT_PERCENTAGE          25             // 25%

#define     SENDER_DEFAULT_LATE_JOINER_PERCENTAGE    0              // 0%
#define     SENDER_MAX_LATE_JOINER_PERCENTAGE        75             // 75%

#define     BITS_PER_BYTE                             8
#define     LOG2_BITS_PER_BYTE                        3

enum eWINDOW_ADVANCE_METHOD
{
    E_WINDOW_ADVANCE_BY_TIME = 1,       // Default mode
    E_WINDOW_USE_AS_DATA_CACHE
};

//==============================================================
//
// Structures
//
typedef struct _RM_SEND_WINDOW
{
    ULONG   RateKbitsPerSec;            // Send rate
    ULONG   WindowSizeInMSecs;
    ULONG   WindowSizeInBytes;
} RM_SEND_WINDOW;

typedef struct _RM_SENDER_STATS
{
    ULONGLONG   DataBytesSent;          // # client data bytes sent out so far
    ULONGLONG   TotalBytesSent;         // SPM, OData and RData bytes
    ULONGLONG   NaksReceived;           // # NAKs received so far
    ULONGLONG   NaksReceivedTooLate;    // # NAKs recvd after window advanced
    ULONGLONG   NumOutstandingNaks;     // # NAKs yet to be responded to
    ULONGLONG   NumNaksAfterRData;      // # NAKs yet to be responded to
    ULONGLONG   RepairPacketsSent;      // # Repairs (RDATA) sent so far
    ULONGLONG   BufferSpaceAvailable;   // # partial messages dropped
    ULONGLONG   TrailingEdgeSeqId;      // smallest (oldest) Sequence Id in the window
    ULONGLONG   LeadingEdgeSeqId;       // largest (newest) Sequence Id in the window
    ULONGLONG   RateKBitsPerSecOverall; // Internally calculated send-rate from the beginning
    ULONGLONG   RateKBitsPerSecLast;    // Send-rate calculated every INTERNAL_RATE_CALCULATION_FREQUENCY
    ULONGLONG   TotalODataPacketsSent;  // # ODATA packets sent so far
} RM_SENDER_STATS;


typedef struct _RM_RECEIVER_STATS
{
    ULONGLONG   NumODataPacketsReceived;// # OData sequences received
    ULONGLONG   NumRDataPacketsReceived;// # RData sequences received
    ULONGLONG   NumDuplicateDataPackets;// # RData sequences received

    ULONGLONG   DataBytesReceived;      // # client data bytes received out so far
    ULONGLONG   TotalBytesReceived;     // SPM, OData and RData bytes
    ULONGLONG   RateKBitsPerSecOverall; // Internally calculated Receive-rate from the beginning
    ULONGLONG   RateKBitsPerSecLast;    // Receive-rate calculated every INTERNAL_RATE_CALCULATION_FREQUENCY

    ULONGLONG   TrailingEdgeSeqId;      // smallest (oldest) Sequence Id in the window
    ULONGLONG   LeadingEdgeSeqId;       // largest (newest) Sequence Id in the window
    ULONGLONG   AverageSequencesInWindow;
    ULONGLONG   MinSequencesInWindow;
    ULONGLONG   MaxSequencesInWindow;

    ULONGLONG   FirstNakSequenceNumber; // # First Outstanding Nak
    ULONGLONG   NumPendingNaks;         // # Sequences waiting for Ncfs
    ULONGLONG   NumOutstandingNaks;     // # Sequences for which Ncfs have been received, but no data
    ULONGLONG   NumDataPacketsBuffered; // # Data packets currently buffered by transport
    ULONGLONG   TotalSelectiveNaksSent; // # Selective NAKs sent so far
    ULONGLONG   TotalParityNaksSent;    // # Parity NAKs sent so far
} RM_RECEIVER_STATS;


typedef struct _RM_FEC_INFO
{
    USHORT              FECBlockSize;
    USHORT              FECProActivePackets;
    UCHAR               FECGroupSize;
    BOOLEAN             fFECOnDemandParityEnabled;
} RM_FEC_INFO;


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  /* _WSRM_H_ */
