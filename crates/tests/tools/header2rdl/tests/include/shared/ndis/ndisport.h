// Copyright (C) Microsoft Corporation. All rights reserved.
//
// Definitions for NDIS PORTs
//

#pragma once

#pragma region App, Games, or System family
#include <winapifamily.h>
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#include <ndis/version.h>

EXTERN_C_START

typedef ULONG NDIS_PORT_NUMBER, *PNDIS_PORT_NUMBER;

#define NDIS_DEFAULT_PORT_NUMBER ((NDIS_PORT_NUMBER)0)
#define NDIS_MAXIMUM_PORTS 0x1000000

//
// NDIS_PORT_TYPE defines the application of a port
//

typedef enum _NDIS_PORT_TYPE
{
    NdisPortTypeUndefined,
    NdisPortTypeBridge,
    NdisPortTypeRasConnection,
    NdisPortType8021xSupplicant,
#if NDIS_SUPPORT_NDIS630
    NdisPortTypeNdisImPlatform,
#endif // NDIS_SUPPORT_NDIS630
    NdisPortTypeMax,
} NDIS_PORT_TYPE, *PNDIS_PORT_TYPE;

EXTERN_C_END

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

