/*++

Copyright (c) 1997 - 1999  Hewlett-Packard Company.
Copyright (c) 1997 - 1999  Microsoft Corporation
All rights reserved

Module Name:

   tcpxcv.h

--*/

#ifndef _TCPXCV_
#define _TCPXCV_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if (!defined(UNKNOWN_PROTOCOL))
    #define UNKNOWN_PROTOCOL        0
    #define PROTOCOL_UNKNOWN_TYPE   UNKNOWN_PROTOCOL
#endif

#if (!defined(RAWTCP))
#define RAWTCP                          1
#define PROTOCOL_RAWTCP_TYPE            RAWTCP
#endif

#if (!defined(LPR))
#define LPR                             2
#define PROTOCOL_LPR_TYPE               LPR
#endif

#define MAX_PORTNAME_LEN                63 +1       // port name length
#define MAX_NETWORKNAME_LEN             48 +1       // host name length
#define MAX_NETWORKNAME2_LEN            128         // host name or IPv6 address
#define MAX_SNMP_COMMUNITY_STR_LEN      32 +1       // SNMP Community String Name
#define MAX_QUEUENAME_LEN               32 +1       // lpr print que name
#define MAX_IPADDR_STR_LEN              15 +1       // ip address; string version
#define MAX_ADDRESS_STR_LEN             12 +1       // hw address length
#define MAX_DEVICEDESCRIPTION_STR_LEN   256+1



typedef struct _PORT_DATA_1
{
    WCHAR  sztPortName[MAX_PORTNAME_LEN];
    DWORD  dwVersion;
    DWORD  dwProtocol;
    DWORD  cbSize;
    DWORD  dwReserved;
    WCHAR  sztHostAddress[MAX_NETWORKNAME_LEN];
    WCHAR  sztSNMPCommunity[MAX_SNMP_COMMUNITY_STR_LEN];
    DWORD  dwDoubleSpool;
    WCHAR  sztQueue[MAX_QUEUENAME_LEN];
    WCHAR  sztIPAddress[MAX_IPADDR_STR_LEN];
    BYTE   Reserved[540];
    DWORD  dwPortNumber;
    DWORD  dwSNMPEnabled;
    DWORD  dwSNMPDevIndex;
}   PORT_DATA_1, *PPORT_DATA_1;

typedef struct _PORT_DATA_2
{
    WCHAR  sztPortName[MAX_PORTNAME_LEN];
    DWORD  dwVersion;
    DWORD  dwProtocol;
    DWORD  cbSize;
    DWORD  dwReserved;
    WCHAR  sztHostAddress [MAX_NETWORKNAME2_LEN];
    WCHAR  sztSNMPCommunity[MAX_SNMP_COMMUNITY_STR_LEN];
    DWORD  dwDoubleSpool;
    WCHAR  sztQueue[MAX_QUEUENAME_LEN];
    BYTE   Reserved[514];
    DWORD  dwPortNumber;
    DWORD  dwSNMPEnabled;
    DWORD  dwSNMPDevIndex;
    DWORD  dwPortMonitorMibIndex;
}   PORT_DATA_2, *PPORT_DATA_2;


typedef struct _PORT_DATA_LIST_1
{
    DWORD dwVersion;
    DWORD cPortData;
    PORT_DATA_2 pPortData[1];
}   PORT_DATA_LIST_1, *PPORT_DATA_LIST_1;


typedef struct _DELETE_PORT_DATA_1
{
    WCHAR  psztPortName[MAX_PORTNAME_LEN];
    BYTE   Reserved[98];
    DWORD  dwVersion;
    DWORD  dwReserved;
}   DELETE_PORT_DATA_1, *PDELETE_PORT_DATA_1;


typedef struct _CONFIG_INFO_DATA_1
{
    BYTE   Reserved[128];
    DWORD  dwVersion;
}   CONFIG_INFO_DATA_1, *PCONFIG_INFO_DATA_1;



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

