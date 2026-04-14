// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region App, Games, or System family
#include <winapifamily.h>
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

EXTERN_C_START

#define NDIS_OFFLOAD_NOT_SUPPORTED              0
#define NDIS_OFFLOAD_SUPPORTED                  1

#define NDIS_OFFLOAD_SET_NO_CHANGE              0
#define NDIS_OFFLOAD_SET_ON                     1
#define NDIS_OFFLOAD_SET_OFF                    2

//
// Encapsulation types that are used during offload in query and set
//
#define NDIS_ENCAPSULATION_NOT_SUPPORTED        0x00000000
#define NDIS_ENCAPSULATION_NULL                 0x00000001
#define NDIS_ENCAPSULATION_IEEE_802_3           0x00000002
#define NDIS_ENCAPSULATION_IEEE_802_3_P_AND_Q   0x00000004
#define NDIS_ENCAPSULATION_IEEE_802_3_P_AND_Q_IN_OOB 0x00000008
#define NDIS_ENCAPSULATION_IEEE_LLC_SNAP_ROUTED 0x00000010

EXTERN_C_END

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

