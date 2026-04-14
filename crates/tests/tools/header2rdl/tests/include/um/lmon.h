/*++

Copyright (c) 1991-1999  Microsoft Corporation
All rights reserved

Module Name:

    lmon.h

--*/


#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct _PORT_INFO_FFA {
    LPSTR   pName;
    DWORD   cbMonitorData;
    LPBYTE  pMonitorData;
} PORT_INFO_FFA, *PPORT_INFO_FFA, *LPPORT_INFO_FFA;

typedef struct _PORT_INFO_FFW {
    LPWSTR  pName;
    DWORD   cbMonitorData;
    LPBYTE  pMonitorData;
} PORT_INFO_FFW, *PPORT_INFO_FFW, *LPPORT_INFO_FFW;

#ifdef UNICODE
#define PORT_INFO_FF PORT_INFO_FFW
#define PPORT_INFO_FF PPORT_INFO_FFW
#define LPPORT_INFO_FF LPPORT_INFO_FFW
#else
#define PORT_INFO_FF PORT_INFO_FFA
#define PPORT_INFO_FF PPORT_INFO_FFA
#define LPPORT_INFO_FF LPPORT_INFO_FFA
#endif // UNICODE


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

