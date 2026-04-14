/*******************************************************************************
 *
 *   wsvns.h
 *
 *  Copyright (C) Microsoft Corporation, 1992-1999.
 *
 *   Windows Sockets include file for VINES IP.  This file contains all
 *   standardized VINES IP information.  Include this header file after
 *   winsock.h.
 *
 *   To open an VINES IP socket, call socket() with an address family of
 *   AF_BAN, a socket type of SOCK_DGRAM, SOCK_STREAM, or SOCK_SEQPACKET,
 *   and protocol of 0.
 *
 ******************************************************************************/

#ifndef _WSVNS_
#define _WSVNS_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*
 * Socket address, VINES IP style.  Address fields and port field are defined
 * as a sequence of bytes.  This is done because they are byte ordered
 * BIG ENDIAN, ala most significant byte first.
 */
typedef struct sockaddr_vns {
    u_short sin_family;			// = AF_BAN
    u_char  net_address[4];		// network address
    u_char  subnet_addr[2];		// subnet address
    u_char  port[2];			// msb=port[0], lsb=port[1]
    u_char  hops;			// # hops for broadcasts
    u_char  filler[5];			// filler, zeros
} SOCKADDR_VNS, *PSOCKADDR_VNS, FAR *LPSOCKADDR_VNS;

#define VNSPROTO_IPC		1
#define VNSPROTO_RELIABLE_IPC	2
#define VNSPROTO_SPP		3


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _WSVNS_


