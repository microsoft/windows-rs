/*++

Copyright (c) 2002-2003  Microsoft Corporation

Module Name:

    tcpioctl.h

Abstract:

    This module contains definitions for IOCTL_TCP_QUERY_INFORMATION_EX.

    CAVEAT: IPHLPAPI provides cleaner API's to access this information.
    
--*/

#ifndef _TCPIOCTL_
#define _TCPIOCTL_

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define FSCTL_TCP_BASE                  FILE_DEVICE_NETWORK

#define _TCP_CTL_CODE(function, method, access) \
            CTL_CODE(FSCTL_TCP_BASE, function, method, access)

#define IOCTL_TCP_QUERY_INFORMATION_EX  \
            _TCP_CTL_CODE(0, METHOD_NEITHER, FILE_ANY_ACCESS)

#define DD_TCP_DEVICE_NAME              L"\\Device\\Tcp"

#define IF_MIB_STATS_ID                 1
#define IP_MIB_STATS_ID                 1
#define IP_MIB_ADDRTABLE_ENTRY_ID       0x102
#define IP_INTFC_INFO_ID                0x103

#define MAX_PHYSADDR_SIZE   8

typedef struct IFEntry {
    ulong           if_index;
    ulong           if_type;
    ulong           if_mtu;
    ulong           if_speed;
    ulong           if_physaddrlen;
    uchar           if_physaddr[MAX_PHYSADDR_SIZE];
    ulong           if_adminstatus;
    ulong           if_operstatus;
    ulong           if_lastchange;
    ulong           if_inoctets;
    ulong           if_inucastpkts;
    ulong           if_innucastpkts;
    ulong           if_indiscards;
    ulong           if_inerrors;
    ulong           if_inunknownprotos;
    ulong           if_outoctets;
    ulong           if_outucastpkts;
    ulong           if_outnucastpkts;
    ulong           if_outdiscards;
    ulong           if_outerrors;
    ulong           if_outqlen;
    ulong           if_descrlen;
    uchar           if_descr[1];
} IFEntry;

typedef struct IPSNMPInfo {
    ulong       ipsi_forwarding;
    ulong       ipsi_defaultttl;
    ulong       ipsi_inreceives;
    ulong       ipsi_inhdrerrors;
    ulong       ipsi_inaddrerrors;
    ulong       ipsi_forwdatagrams;
    ulong       ipsi_inunknownprotos;
    ulong       ipsi_indiscards;
    ulong       ipsi_indelivers;
    ulong       ipsi_outrequests;
    ulong       ipsi_routingdiscards;
    ulong       ipsi_outdiscards;
    ulong       ipsi_outnoroutes;
    ulong       ipsi_reasmtimeout;
    ulong       ipsi_reasmreqds;
    ulong       ipsi_reasmoks;
    ulong       ipsi_reasmfails;
    ulong       ipsi_fragoks;
    ulong       ipsi_fragfails;
    ulong       ipsi_fragcreates;
    ulong       ipsi_numif;
    ulong       ipsi_numaddr;
    ulong       ipsi_numroutes;
} IPSNMPInfo;

typedef struct IPAddrEntry {
    ulong       iae_addr;
    ulong       iae_index;
    ulong       iae_mask;
    ulong       iae_bcastaddr;
    ulong       iae_reasmsize;
    ushort      iae_context;
    ushort      iae_pad;
} IPAddrEntry;

typedef struct IPInterfaceInfo {
    ulong       iii_flags;
    ulong       iii_mtu;
    ulong       iii_speed;
    ulong       iii_addrlength;
    uchar       iii_addr[1];
} IPInterfaceInfo;


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _TCPIOCTL_
