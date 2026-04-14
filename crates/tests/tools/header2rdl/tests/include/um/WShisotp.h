/* File: .streams.include.src/mod  wshisotp.h            Version: Initial */
/*
 *   wshisotp.h
 *   Copyright (c) 1994-1999, Microsoft Corp. All rights reserved.
 *
 *   Windows Sockets include file for ISO TP4.  This file contains all
 *   standardized ISO TP4 information.  Include this header file after
 *   winsock.h.
 *
 *   The information contained in this header file was originally
 *   created by Alcatel TITN Inc.
 */

#ifndef _WSHISOTP_
#define _WSHISOTP_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*
 * Protocol values for ISO transport protocols.
 */

#define ISOPROTO_TP0       25      /* connection orientated transport protocol */
#define ISOPROTO_TP1       26      /* not implemented */
#define ISOPROTO_TP2       27      /* not implemented */
#define ISOPROTO_TP3       28      /* not implemented */
#define ISOPROTO_TP4       29      /* connection orientated transport protocol */
#define ISOPROTO_TP        ISOPROTO_TP4
#define ISOPROTO_CLTP      30      /* connectionless transport */
#define ISOPROTO_CLNP      31      /* connectionless internetworking protocol */
#define ISOPROTO_X25       32      /* cons */
#define ISOPROTO_INACT_NL  33      /* inactive network layer */
#define ISOPROTO_ESIS      34      /* ES-IS protocol */
#define ISOPROTO_INTRAISIS 35      /* IS-IS protocol */

#define IPPROTO_RAW        255     /* raw clnp */
#define IPPROTO_MAX        256

/*
 *   The maximum size of the tranport address (tp_addr field of a
 *   sockaddr_tp structure) is 64.
 */

#define ISO_MAX_ADDR_LENGTH 64

/*
 *   There are two types of ISO addresses, hierarchical and
 *   non-hierarchical.  For hierarchical addresses, the tp_addr
 *   field contains both the transport selector and the network
 *   address.  For non-hierarchical addresses, tp_addr contains only
 *   the transport address, which must be translated by the ISO TP4
 *   transport provider into the transport selector and network address.
 */

#define ISO_HIERARCHICAL            0
#define ISO_NON_HIERARCHICAL        1

/*
 *   The format of the address structure (sockaddr) to pass to Windows
 *   Sockets APIs.
 *
 */

typedef struct sockaddr_tp {
   u_short tp_family;          /* Always AF_ISO */
   u_short tp_addr_type;       /* ISO_HIERARCHICAL or ISO_NON_HIERARCHICAL
*/
   u_short tp_taddr_len;       /* Length of transport address, <= 52 */
   u_short tp_tsel_len;        /* Length of transport selector, <= 32 */
                               /* 0 if ISO_NON_HIERARCHICAL */
   u_char tp_addr[ISO_MAX_ADDR_LENGTH];
} SOCKADDR_TP, *PSOCKADDR_TP, *LPSOCKADDR_TP;

#define ISO_SET_TP_ADDR(sa_tp, port, portlen, node, nodelen)              \
            (sa_tp)->tp_family = AF_ISO;                         \
            (sa_tp)->tp_addr_type = ISO_HIERARCHICAL;            \
            (sa_tp)->tp_tsel_len = (portlen);              \
            (sa_tp)->tp_taddr_len = (portlen) + (nodelen); \
            memcpy(&(sa_tp)->tp_addr, (port), (portlen)); \
            memcpy(&(sa_tp)->tp_addr[portlen], (node), (nodelen));


/*
 *   Expedited Data Usage Negotiation option.
 *   Default when the option is not present is be EXP_DATA_USE
 *   as per ISO 8073
 */

#define ISO_EXP_DATA_USE  00    /* Use of Expedited Data */
#define ISO_EXP_DATA_NUSE 01    /* Non-use of Expedited Data */


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

