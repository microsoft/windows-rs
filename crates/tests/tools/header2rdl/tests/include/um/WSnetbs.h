/*
 *   wsnetbs.h
 *   Copyright (c) 1994-1999, Microsoft Corp. All rights reserved.
 *
 *   Windows Sockets include file for NETBIOS.  This file contains all
 *   standardized NETBIOS information.  Include this header file after
 *   winsock.h.
 *
 */

#ifndef _WSNETBS_
#define _WSNETBS_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*
 *   This is the structure of the SOCKADDR structure for NETBIOS.
 *
 */

#define NETBIOS_NAME_LENGTH 16

typedef struct sockaddr_nb {
    short   snb_family;
    u_short snb_type;
    char    snb_name[NETBIOS_NAME_LENGTH];
} SOCKADDR_NB, *PSOCKADDR_NB,FAR *LPSOCKADDR_NB;

/*
 * Bit values for the snb_type field of SOCKADDR_NB.
 *
 */

#define NETBIOS_UNIQUE_NAME         (0x0000)
#define NETBIOS_GROUP_NAME          (0x0001)
#define NETBIOS_TYPE_QUICK_UNIQUE   (0x0002)
#define NETBIOS_TYPE_QUICK_GROUP    (0x0003)

/*
 * A macro convenient for setting up NETBIOS SOCKADDRs.
 *
 */

#define SET_NETBIOS_SOCKADDR(_snb,_type,_name,_port)                          \
    {                                                                         \
        int _i;                                                               \
        (_snb)->snb_family = AF_NETBIOS;                                      \
        (_snb)->snb_type = (_type);                                           \
        for (_i=0; _i<NETBIOS_NAME_LENGTH-1; _i++) {                          \
            (_snb)->snb_name[_i] = ' ';                                       \
        }                                                                     \
        for (_i=0; *((_name)+_i) != '\0' && _i<NETBIOS_NAME_LENGTH-1; _i++) { \
            (_snb)->snb_name[_i] = *((_name)+_i);                             \
        }                                                                     \
        (_snb)->snb_name[NETBIOS_NAME_LENGTH-1] = (_port);                    \
    }

/*
 *   To open a NetBIOS socket, call the socket() function as follows:
 *
 *       s = socket( AF_NETBIOS, {SOCK_SEQPACKET|SOCK_DGRAM}, -Lana );
 *
 *   where Lana is the NetBIOS Lana number of interest.  For example, to
 *   open a socket for Lana 2, specify -2 as the "protocol" parameter
 *   to the socket() function.
 *
 */


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

