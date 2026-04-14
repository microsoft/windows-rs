// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region App, Games, or System family
#include <winapifamily.h>
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

EXTERN_C_START

#pragma warning(push)
#pragma warning(default:4820) // warn if the compiler inserted padding

typedef struct _NDIS_OBJECT_HEADER
{
    UCHAR
        Type;

    UCHAR
        Revision;

    USHORT
        Size;

} NDIS_OBJECT_HEADER, *PNDIS_OBJECT_HEADER;

#ifdef C_ASSERT
C_ASSERT(sizeof(NDIS_OBJECT_HEADER) == 4);
#endif

#pragma warning(pop)

EXTERN_C_END

#define NDIS_OBJECT_REVISION_1 1

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion
