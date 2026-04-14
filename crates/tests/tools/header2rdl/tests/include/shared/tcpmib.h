/*++

Copyright (c) Microsoft Corporation

Module Name:

    tcpmib.h

Abstract:

    This module contains the public definitions and structures for the
    TCP-specific parts of MIB-II.  These definitions were previously
    in iprtrmib.h, which now includes this file.

Environment:

    user mode or kernel mode

--*/

#ifndef _TCPMIB_
#define _TCPMIB_
#pragma once
#include <winapifamily.h>

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifndef ANY_SIZE
#define ANY_SIZE 1
#endif

#define TCPIP_OWNING_MODULE_SIZE 16

//
// TCP states, as defined in the MIB.
//
typedef enum {
    MIB_TCP_STATE_CLOSED     =  1,
    MIB_TCP_STATE_LISTEN     =  2,
    MIB_TCP_STATE_SYN_SENT   =  3,
    MIB_TCP_STATE_SYN_RCVD   =  4,
    MIB_TCP_STATE_ESTAB      =  5,
    MIB_TCP_STATE_FIN_WAIT1  =  6,
    MIB_TCP_STATE_FIN_WAIT2  =  7,
    MIB_TCP_STATE_CLOSE_WAIT =  8,
    MIB_TCP_STATE_CLOSING    =  9,
    MIB_TCP_STATE_LAST_ACK   = 10,
    MIB_TCP_STATE_TIME_WAIT  = 11,
    MIB_TCP_STATE_DELETE_TCB = 12,
    //
    // Extra TCP states not defined in the MIB
    //
    MIB_TCP_STATE_RESERVED      = 100
} MIB_TCP_STATE;

//
// Various Offload states a TCP connection can be in.
//
typedef enum {
    TcpConnectionOffloadStateInHost,
    TcpConnectionOffloadStateOffloading,
    TcpConnectionOffloadStateOffloaded,
    TcpConnectionOffloadStateUploading,
    TcpConnectionOffloadStateMax
} TCP_CONNECTION_OFFLOAD_STATE, *PTCP_CONNECTION_OFFLOAD_STATE;

typedef struct _MIB_TCPROW_LH {
    union {
        DWORD dwState;       // Old field used DWORD type.
        MIB_TCP_STATE State; // New field uses enum type.
    };
    DWORD dwLocalAddr;
    DWORD dwLocalPort;
    DWORD dwRemoteAddr;
    DWORD dwRemotePort;
} MIB_TCPROW_LH, *PMIB_TCPROW_LH;

typedef struct _MIB_TCPROW_W2K {
    DWORD       dwState;
    DWORD       dwLocalAddr;
    DWORD       dwLocalPort;
    DWORD       dwRemoteAddr;
    DWORD       dwRemotePort;
} MIB_TCPROW_W2K, *PMIB_TCPROW_W2K;

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef MIB_TCPROW_LH MIB_TCPROW;
typedef MIB_TCPROW_LH *PMIB_TCPROW;
#elif (NTDDI_VERSION >= NTDDI_WIN2K)
typedef MIB_TCPROW_W2K MIB_TCPROW;
typedef MIB_TCPROW_W2K *PMIB_TCPROW;
#else
typedef MIB_TCPROW_LH MIB_TCPROW;
typedef MIB_TCPROW_LH *PMIB_TCPROW;
#endif

typedef struct _MIB_TCPTABLE {
    DWORD dwNumEntries;
    MIB_TCPROW table[ANY_SIZE];
} MIB_TCPTABLE, *PMIB_TCPTABLE;

#define SIZEOF_TCPTABLE(X) (FIELD_OFFSET(MIB_TCPTABLE,table[0]) + \
                            ((X) * sizeof(MIB_TCPROW)) + ALIGN_SIZE)

typedef struct _MIB_TCPROW2 {
    DWORD dwState;
    DWORD dwLocalAddr;
    DWORD dwLocalPort;
    DWORD dwRemoteAddr;
    DWORD dwRemotePort;
    DWORD dwOwningPid;
    TCP_CONNECTION_OFFLOAD_STATE dwOffloadState;
} MIB_TCPROW2, *PMIB_TCPROW2;

typedef struct _MIB_TCPTABLE2 {
    DWORD dwNumEntries;
    MIB_TCPROW2 table[ANY_SIZE];
} MIB_TCPTABLE2, *PMIB_TCPTABLE2;

#define SIZEOF_TCPTABLE2(X) (FIELD_OFFSET(MIB_TCPTABLE2,table[0]) + \
                            ((X) * sizeof(MIB_TCPROW2)) + ALIGN_SIZE)

typedef struct _MIB_TCPROW_OWNER_PID
{
    DWORD       dwState;
    DWORD       dwLocalAddr;
    DWORD       dwLocalPort;
    DWORD       dwRemoteAddr;
    DWORD       dwRemotePort;
    DWORD       dwOwningPid;
} MIB_TCPROW_OWNER_PID, *PMIB_TCPROW_OWNER_PID;

typedef struct _MIB_TCPTABLE_OWNER_PID
{
    DWORD                dwNumEntries;
    MIB_TCPROW_OWNER_PID table[ANY_SIZE];
} MIB_TCPTABLE_OWNER_PID, *PMIB_TCPTABLE_OWNER_PID;

#define SIZEOF_TCPTABLE_OWNER_PID(X) (FIELD_OFFSET(MIB_TCPTABLE_OWNER_PID, table[0]) + \
									  ((X) * sizeof(MIB_TCPROW_OWNER_PID)) + ALIGN_SIZE)

typedef struct _MIB_TCPROW_OWNER_MODULE {
    DWORD dwState;
    DWORD dwLocalAddr;
    DWORD dwLocalPort;
    DWORD dwRemoteAddr;
    DWORD dwRemotePort;
    DWORD dwOwningPid;
    LARGE_INTEGER liCreateTimestamp;
    ULONGLONG OwningModuleInfo [TCPIP_OWNING_MODULE_SIZE];
} MIB_TCPROW_OWNER_MODULE, *PMIB_TCPROW_OWNER_MODULE;

typedef struct _MIB_TCPTABLE_OWNER_MODULE
{
    DWORD                   dwNumEntries;
    MIB_TCPROW_OWNER_MODULE table[ANY_SIZE];
} MIB_TCPTABLE_OWNER_MODULE, *PMIB_TCPTABLE_OWNER_MODULE;

#define SIZEOF_TCPTABLE_OWNER_MODULE(X) (FIELD_OFFSET(MIB_TCPTABLE_OWNER_MODULE, table[0]) + \
										 ((X) * sizeof(MIB_TCPROW_OWNER_MODULE)) + ALIGN_SIZE)

#ifdef _WS2IPDEF_
//
// The following definitions require Winsock2.
//

typedef struct _MIB_TCP6ROW {
    MIB_TCP_STATE State;
    IN6_ADDR LocalAddr;
    DWORD dwLocalScopeId;
    DWORD dwLocalPort;
    IN6_ADDR RemoteAddr;
    DWORD dwRemoteScopeId;
    DWORD dwRemotePort;
} MIB_TCP6ROW, *PMIB_TCP6ROW;

typedef struct _MIB_TCP6TABLE {
    DWORD dwNumEntries;
    MIB_TCP6ROW table[ANY_SIZE];
} MIB_TCP6TABLE, *PMIB_TCP6TABLE;

#define SIZEOF_TCP6TABLE(X) (FIELD_OFFSET(MIB_TCP6TABLE,table[0]) + \
                             ((X) * sizeof(MIB_TCP6ROW)) + ALIGN_SIZE)

typedef struct _MIB_TCP6ROW2 {
    IN6_ADDR LocalAddr;
    DWORD dwLocalScopeId;
    DWORD dwLocalPort;
    IN6_ADDR RemoteAddr;
    DWORD dwRemoteScopeId;
    DWORD dwRemotePort;
    MIB_TCP_STATE State;
    DWORD dwOwningPid;
    TCP_CONNECTION_OFFLOAD_STATE dwOffloadState;
} MIB_TCP6ROW2, *PMIB_TCP6ROW2;

typedef struct _MIB_TCP6TABLE2 {
    DWORD dwNumEntries;
    MIB_TCP6ROW2 table[ANY_SIZE];
} MIB_TCP6TABLE2, *PMIB_TCP6TABLE2;

#define SIZEOF_TCP6TABLE2(X) (FIELD_OFFSET(MIB_TCP6TABLE2,table[0]) + \
							  ((X) * sizeof(MIB_TCP6ROW2)) + ALIGN_SIZE)

typedef struct _MIB_TCP6ROW_OWNER_PID
{
    UCHAR           ucLocalAddr[16];
    DWORD           dwLocalScopeId;
    DWORD           dwLocalPort;
    UCHAR           ucRemoteAddr[16];
    DWORD           dwRemoteScopeId;
    DWORD           dwRemotePort;
    DWORD           dwState;
    DWORD           dwOwningPid;
} MIB_TCP6ROW_OWNER_PID, *PMIB_TCP6ROW_OWNER_PID;

typedef struct _MIB_TCP6TABLE_OWNER_PID
{
    DWORD                   dwNumEntries;
    _Field_size_(dwNumEntries)
    MIB_TCP6ROW_OWNER_PID   table[ANY_SIZE];
} MIB_TCP6TABLE_OWNER_PID, *PMIB_TCP6TABLE_OWNER_PID;

#define SIZEOF_TCP6TABLE_OWNER_PID(X) (FIELD_OFFSET(MIB_TCP6TABLE_OWNER_PID, table[0]) + \
									   ((X) * sizeof(MIB_TCP6ROW_OWNER_PID)) + ALIGN_SIZE)

typedef struct _MIB_TCP6ROW_OWNER_MODULE
{
    UCHAR           ucLocalAddr[16];
    DWORD           dwLocalScopeId;
    DWORD           dwLocalPort;
    UCHAR           ucRemoteAddr[16];
    DWORD           dwRemoteScopeId;
    DWORD           dwRemotePort;
    DWORD           dwState;
    DWORD           dwOwningPid;
    LARGE_INTEGER   liCreateTimestamp;
    ULONGLONG       OwningModuleInfo[TCPIP_OWNING_MODULE_SIZE];
} MIB_TCP6ROW_OWNER_MODULE, *PMIB_TCP6ROW_OWNER_MODULE;

typedef struct _MIB_TCP6TABLE_OWNER_MODULE
{
    DWORD                       dwNumEntries;
    _Field_size_(dwNumEntries)
    MIB_TCP6ROW_OWNER_MODULE    table[ANY_SIZE];
} MIB_TCP6TABLE_OWNER_MODULE, *PMIB_TCP6TABLE_OWNER_MODULE;

#define SIZEOF_TCP6TABLE_OWNER_MODULE(X) (FIELD_OFFSET(MIB_TCP6TABLE_OWNER_MODULE, table[0]) + \
										  ((X) * sizeof(MIB_TCP6ROW_OWNER_MODULE)) + ALIGN_SIZE)

#endif // _WS2IPDEF_

#define MIB_TCP_MAXCONN_DYNAMIC ((ULONG)-1)

//
// The algorithm used to determine the timeout value used for retransmitting
// unacknowledged octets.
//
typedef enum {
    TcpRtoAlgorithmOther  = 1,
    TcpRtoAlgorithmConstant,
    TcpRtoAlgorithmRsre,
    TcpRtoAlgorithmVanj,

    MIB_TCP_RTO_OTHER     = 1,
    MIB_TCP_RTO_CONSTANT  = 2,
    MIB_TCP_RTO_RSRE      = 3,
    MIB_TCP_RTO_VANJ      = 4,
} TCP_RTO_ALGORITHM, *PTCP_RTO_ALGORITHM;

typedef struct _MIB_TCPSTATS_LH {
    union {
        DWORD dwRtoAlgorithm;
        TCP_RTO_ALGORITHM RtoAlgorithm;
    };
    DWORD dwRtoMin;
    DWORD dwRtoMax;
    DWORD dwMaxConn;
    DWORD dwActiveOpens;
    DWORD dwPassiveOpens;
    DWORD dwAttemptFails;
    DWORD dwEstabResets;
    DWORD dwCurrEstab;
    DWORD dwInSegs;
    DWORD dwOutSegs;
    DWORD dwRetransSegs;
    DWORD dwInErrs;
    DWORD dwOutRsts;
    DWORD dwNumConns;
} MIB_TCPSTATS_LH, *PMIB_TCPSTATS_LH;

typedef struct _MIB_TCPSTATS_W2K {
    DWORD       dwRtoAlgorithm;
    DWORD       dwRtoMin;
    DWORD       dwRtoMax;
    DWORD       dwMaxConn;
    DWORD       dwActiveOpens;
    DWORD       dwPassiveOpens;
    DWORD       dwAttemptFails;
    DWORD       dwEstabResets;
    DWORD       dwCurrEstab;
    DWORD       dwInSegs;
    DWORD       dwOutSegs;
    DWORD       dwRetransSegs;
    DWORD       dwInErrs;
    DWORD       dwOutRsts;
    DWORD       dwNumConns;
} MIB_TCPSTATS_W2K, *PMIB_TCPSTATS_W2K;

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef MIB_TCPSTATS_LH MIB_TCPSTATS;
typedef MIB_TCPSTATS_LH *PMIB_TCPSTATS;
#elif (NTDDI_VERSION >= NTDDI_WIN2K)
typedef MIB_TCPSTATS_W2K MIB_TCPSTATS;
typedef MIB_TCPSTATS_W2K *PMIB_TCPSTATS;
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
typedef struct _MIB_TCPSTATS2 {
    TCP_RTO_ALGORITHM RtoAlgorithm;
    DWORD dwRtoMin;
    DWORD dwRtoMax;
    DWORD dwMaxConn;
    DWORD dwActiveOpens;
    DWORD dwPassiveOpens;
    DWORD dwAttemptFails;
    DWORD dwEstabResets;
    DWORD dwCurrEstab;
    DWORD64 dw64InSegs;
    DWORD64 dw64OutSegs;
    DWORD dwRetransSegs;
    DWORD dwInErrs;
    DWORD dwOutRsts;
    DWORD dwNumConns;
} MIB_TCPSTATS2, *PMIB_TCPSTATS2;
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // _TCPMIB_
