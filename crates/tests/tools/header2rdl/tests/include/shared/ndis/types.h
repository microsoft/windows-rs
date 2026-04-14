// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region App, Games, or System family
#include <winapifamily.h>
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

EXTERN_C_START

typedef PVOID NDIS_HANDLE, *PNDIS_HANDLE;

typedef _Return_type_success_(return >= 0) int NDIS_STATUS, *PNDIS_STATUS; // note default size

#ifndef NDIS_EXPORTED_ROUTINE
#  define NDIS_EXPORTED_ROUTINE DECLSPEC_IMPORT
#endif // NDIS_EXPORTED_ROUTINE

EXTERN_C_END

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion
