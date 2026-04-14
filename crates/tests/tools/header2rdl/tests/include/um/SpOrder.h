/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    sporder.h

Abstract:

    This header prototypes the 32-Bit Windows functions that are used
    to change the order or WinSock2 transport service providers and
    name space providers.

Revision History:

--*/

#ifndef __SPORDER_H__
#define __SPORDER_H__

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)



#ifdef __cplusplus
extern "C" {
#endif

int
WSPAPI
WSCWriteProviderOrder (
    IN LPDWORD lpwdCatalogEntryId,
    IN DWORD dwNumberOfEntries
    );

typedef
int
(WSPAPI * LPWSCWRITEPROVIDERORDER)(
    IN LPDWORD lpwdCatalogEntryId,
    IN DWORD dwNumberOfEntries
    );

#if(_WIN32_WINNT >= 0x0501)
#ifdef _WIN64
int
WSPAPI
WSCWriteProviderOrder32 (
    IN LPDWORD lpwdCatalogEntryId,
    IN DWORD dwNumberOfEntries
    );
#endif //_WIN64
#endif //_WIN32_WINNT >= 0x0501

int
WSPAPI
WSCWriteNameSpaceOrder (
    IN LPGUID lpProviderId,
    IN DWORD dwNumberOfEntries
    );

typedef 
int
(WSPAPI * LPWSCWRITENAMESPACEORDER)(
    IN LPGUID lpProviderId,
    IN DWORD dwNumberOfEntries
    );

#if(_WIN32_WINNT >= 0x0501)
#ifdef _WIN64
int
WSPAPI
WSCWriteNameSpaceOrder32 (
    IN LPGUID lpProviderId,
    IN DWORD dwNumberOfEntries
    );
#endif //_WIN64
#endif //_WIN32_WINNT >= 0x0501

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif      // __SPORDER_H__
