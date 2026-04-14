/*++

Copyright (c) 2001 Microsoft Corporation


Module Name:

    wlclient.h

Abstract:

    Header file for wireless windows APIs.

Environment:

    User Level: Windows

Revision History:


--*/


#ifndef __WLCLIENT_H__
#define __WLCLIENT_H__

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifndef __WINDOT11_H__
#include <windot11.h>
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)

#ifdef __cplusplus
extern "C" {
#endif



typedef struct _DOT11_ADAPTER {
    GUID gAdapterId;
#ifdef __midl
    [string] LPWSTR pszDescription;
#else
    LPWSTR pszDescription;
#endif
    DOT11_CURRENT_OPERATION_MODE Dot11CurrentOpMode;
} DOT11_ADAPTER, * PDOT11_ADAPTER;





typedef struct _DOT11_BSS_LIST {
    ULONG uNumOfBytes;
#ifdef __midl
    [size_is(uNumOfBytes)] PUCHAR pucBuffer;
#else
    _Field_size_opt_(uNumOfBytes) PUCHAR pucBuffer;
#endif
} DOT11_BSS_LIST, * PDOT11_BSS_LIST;





typedef struct _DOT11_PORT_STATE {
    DOT11_MAC_ADDRESS PeerMacAddress;   // Unicast mac address of the peer
    ULONG uSessionId;
    BOOL bPortControlled;               // TRUE, if the port is controlled by Security Module
    BOOL bPortAuthorized;               // TRUE, if the port is authorized for data packets
} DOT11_PORT_STATE, * PDOT11_PORT_STATE;




#include <packon.h>
typedef struct _DOT11_SECURITY_PACKET_HEADER {
    DOT11_MAC_ADDRESS PeerMac;
    USHORT usEtherType;
    UCHAR Data[1];
} DOT11_SECURITY_PACKET_HEADER, * PDOT11_SECURITY_PACKET_HEADER;
#include <packoff.h>


#ifdef __cplusplus
}
#endif

#endif  // (NTDDI_VERSION > NTDDI_VISTA)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __WLCLIENT_H__

