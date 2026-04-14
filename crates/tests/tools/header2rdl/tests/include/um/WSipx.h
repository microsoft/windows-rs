/*
 *   wsipx.h
 *
 *   Microsoft Windows
 *   Copyright (C) Microsoft Corporation, 1992-1999.
 *
 *   Windows Sockets include file for IPX/SPX.  This file contains all
 *   standardized IPX/SPX information.  Include this header file after
 *   winsock.h.
 *
 *   To open an IPX socket, call socket() with an address family of
 *   AF_IPX, a socket type of SOCK_DGRAM, and protocol NSPROTO_IPX.
 *   Note that the protocol value must be specified, it cannot be 0.
 *   All IPX packets are sent with the packet type field of the IPX
 *   header set to 0.
 *
 *   To open an SPX or SPXII socket, call socket() with an address
 *   family of AF_IPX, socket type of SOCK_SEQPACKET or SOCK_STREAM,
 *   and protocol of NSPROTO_SPX or NSPROTO_SPXII.  If SOCK_SEQPACKET
 *   is specified, then the end of message bit is respected, and
 *   recv() calls are not completed until a packet is received with
 *   the end of message bit set.  If SOCK_STREAM is specified, then
 *   the end of message bit is not respected, and recv() completes
 *   as soon as any data is received, regardless of the setting of the
 *   end of message bit.  Send coalescing is never performed, and sends
 *   smaller than a single packet are always sent with the end of
 *   message bit set.  Sends larger than a single packet are packetized
 *   with the end of message bit set on only the last packet of the
 *   send.
 *
 */

#ifndef _WSIPX_
#define _WSIPX_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*
 *   This is the structure of the SOCKADDR structure for IPX and SPX.
 *
 */

typedef struct sockaddr_ipx {
    short sa_family;
    char  sa_netnum[4];
    char  sa_nodenum[6];
    unsigned short sa_socket;
} SOCKADDR_IPX, *PSOCKADDR_IPX,FAR *LPSOCKADDR_IPX;

/*
 *   Protocol families used in the "protocol" parameter of the socket() API.
 *
 */

#define NSPROTO_IPX      1000
#define NSPROTO_SPX      1256
#define NSPROTO_SPXII    1257


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

