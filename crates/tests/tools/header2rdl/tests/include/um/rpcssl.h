//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//  File:       rpcssl.h
//
//  Contents:   prototypes for RemoteProcedure Call API functions
//              that use types from wincrypt.h
//
//----------------------------------------------------------------------------

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#ifdef __cplusplus
extern "C" {
#endif

#ifdef RPC_UNICODE_SUPPORTED

#ifdef UNICODE
#define RpcCertGeneratePrincipalName RpcCertGeneratePrincipalNameW
#else /* UNICODE */
#define RpcCertGeneratePrincipalName RpcCertGeneratePrincipalNameA
#endif /* UNICODE */

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcCertGeneratePrincipalNameW(
    _In_ PCCERT_CONTEXT Context,
    _In_ DWORD Flags,
    _Outptr_ RPC_WSTR *pBuffer
    );


RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcCertGeneratePrincipalNameA(
    _In_ PCCERT_CONTEXT       Context,
    _In_ DWORD                Flags,
    _Outptr_ RPC_CSTR *    pBuffer
    );

#else /* RPC_UNICODE_SUPPORTED */

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcCertGeneratePrincipalName(
    _In_ PCCERT_CONTEXT Context,
    _In_ DWORD Flags,
    _Outptr_ RPC_CSTR *pBuffer
    );

#endif /* RPC_UNICODE_SUPPORTED */

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

