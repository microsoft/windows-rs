// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region App, Games, or System family
#include <winapifamily.h>
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

EXTERN_C_START

//
// This is the type of an NDIS OID value.
//

typedef ULONG NDIS_OID, *PNDIS_OID;

//
// Request types used by NdisRequest; constants are added for
// all entry points in the MAC, for those that want to create
// their own internal requests.
//

typedef enum _NDIS_REQUEST_TYPE
{
    NdisRequestQueryInformation,
    NdisRequestSetInformation,
    NdisRequestQueryStatistics,
#ifdef NDIS_INCLUDE_LEGACY_NAMES
    NdisRequestOpen,
    NdisRequestClose,
    NdisRequestSend,
    NdisRequestTransferData,
    NdisRequestReset,
    NdisRequestGeneric1,
    NdisRequestGeneric2,
    NdisRequestGeneric3,
    NdisRequestGeneric4,
#endif // NDIS_INCLUDE_LEGACY_NAMES
#if NDIS_SUPPORT_NDIS6
    NdisRequestMethod = 12,
#endif
} NDIS_REQUEST_TYPE, *PNDIS_REQUEST_TYPE;

#define NDIS_OBJECT_TYPE_OID_REQUEST            0x96

EXTERN_C_END

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

