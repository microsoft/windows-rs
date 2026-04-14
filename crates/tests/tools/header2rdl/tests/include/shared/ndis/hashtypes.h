// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region App, Games, or System family
#include <winapifamily.h>
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#include <ndis/version.h>

EXTERN_C_START

#define NdisHashFunctionToeplitz                0x00000001 // the primary RSS hash function
#define NdisHashFunctionReserved1               0x00000002 // supported hash function 2
#define NdisHashFunctionReserved2               0x00000004 // supported hash function 3
#define NdisHashFunctionReserved3               0x00000008 // supported hash function 4

#define NDIS_HASH_FUNCTION_MASK                 0x000000FF
#define NDIS_HASH_TYPE_MASK                     0x00FFFF00

//
// What kind of hash field type the protocol asks the miniport to do
//
#define NDIS_HASH_IPV4                          0x00000100
#define NDIS_HASH_TCP_IPV4                      0x00000200
#define NDIS_HASH_IPV6                          0x00000400
#define NDIS_HASH_IPV6_EX                       0x00000800
#define NDIS_HASH_TCP_IPV6                      0x00001000
#define NDIS_HASH_TCP_IPV6_EX                   0x00002000
#if (NDIS_SUPPORT_NDIS680)
#define NDIS_HASH_UDP_IPV4                      0x00004000
#define NDIS_HASH_UDP_IPV6                      0x00008000
#define NDIS_HASH_UDP_IPV6_EX                   0x00010000
#endif // (NDIS_SUPPORT_NDIS680)

//
// Typedef to use as flags holder to correlate to the NDIS_HAS_ prefixed flags above.
//
typedef ULONG NDIS_HASH_FLAGS;

#ifdef NDIS_INCLUDE_LEGACY_NAMES

#define NDIS_RSS_HASH_FUNC_FROM_HASH_INFO(_HashInfo)  \
        ((_HashInfo) & (NDIS_HASH_FUNCTION_MASK))

#define NDIS_RSS_HASH_TYPE_FROM_HASH_INFO(_HashInfo)  \
        ((_HashInfo) & (NDIS_HASH_TYPE_MASK))

#define NDIS_RSS_HASH_INFO_FROM_TYPE_AND_FUNC(_HashType, _HashFunction) \
        ((_HashType) | (_HashFunction))

#else // NDIS_INCLUDE_LEGACY_NAMES

inline
ULONG
NDIS_RSS_HASH_FUNC_FROM_HASH_INFO(
    _In_ ULONG                                  HashInfo
    )
{
    return HashInfo & NDIS_HASH_FUNCTION_MASK;
}

inline
ULONG
NDIS_RSS_HASH_TYPE_FROM_HASH_INFO(
    _In_ ULONG                                  HashInfo
    )
{
    return HashInfo & NDIS_HASH_TYPE_MASK;
}

inline
ULONG
NDIS_RSS_HASH_INFO_FROM_TYPE_AND_FUNC(
    _In_ ULONG                                  HashType,
    _In_ ULONG                                  HashFunction
    )
{
    return HashType | HashFunction;
}

#endif // NDIS_INCLUDE_LEGACY_NAMES

EXTERN_C_END

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

