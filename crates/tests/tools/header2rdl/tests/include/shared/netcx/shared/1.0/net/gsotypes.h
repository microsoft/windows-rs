// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

EXTERN_C_START

#pragma warning(push)
#pragma warning(default:4820) // warn if the compiler inserted padding
#pragma warning(disable:4214) // nonstandard extension used: bit field types other than int
#pragma warning(disable:4201) // nonstandard extension used: nameless struct/union

typedef struct _NET_PACKET_GSO
{
    union {
        struct {
            UINT32
                Mss : 20;
            UINT32
                Reserved0 : 12;
        } TCP;
        struct {
            UINT32
                Mss : 20;
            UINT32
                Reserved0 : 12;
        } UDP;
    } DUMMYUNIONNAME;
} NET_PACKET_GSO;

C_ASSERT(sizeof(NET_PACKET_GSO) == 4);

#pragma warning(pop)

EXTERN_C_END


#define NET_PACKET_EXTENSION_GSO_NAME L"ms_packet_gso"
#define NET_PACKET_EXTENSION_GSO_VERSION_1 1U

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

