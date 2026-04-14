// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifndef NETCX_ADAPTER_2
#error include netadaptercx.h
#endif

#pragma warning(push)
#pragma warning(default:4820) // warn if the compiler inserted padding
#pragma warning(disable:4201) // nonstandard extension used: nameless struct/union

EXTERN_C_START

typedef enum _NET_EXTENSION_TYPE
{
    NetExtensionTypePacket = 1,
    NetExtensionTypeFragment,
    NetExtensionTypeBuffer,
} NET_EXTENSION_TYPE;

typedef struct _NET_EXTENSION
{
    void *
        Reserved[4];

    union
    {
        BOOLEAN
            Enabled;

        void *
            Reserved1;
    } DUMMYUNIONNAME;
} NET_EXTENSION;

#ifdef _WIN64
C_ASSERT(sizeof(NET_EXTENSION) == 40);
#else
C_ASSERT(sizeof(NET_EXTENSION) == 20);
#endif

#pragma warning(pop)

inline
void *
NetExtensionGetData(
    NET_EXTENSION const * Extension,
    UINT32 Index
)
{
    return (void *)((unsigned char *)(Extension->Reserved[0]) + (SIZE_T)Extension->Reserved[1] * Index);
}

EXTERN_C_END

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

