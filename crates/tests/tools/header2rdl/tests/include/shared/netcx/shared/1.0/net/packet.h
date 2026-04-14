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
#pragma warning(disable:4214) // nonstandard extension used: bit field types other than int

EXTERN_C_START

typedef enum _NET_PACKET_LAYER2_TYPE
{
    NetPacketLayer2TypeUnspecified,
    NetPacketLayer2TypeNull,
    NetPacketLayer2TypeEthernet,
    NetPacketLayer2TypeIeee80211,
} NET_PACKET_LAYER2_TYPE;

typedef enum _NET_PACKET_LAYER3_TYPE
{
    NetPacketLayer3TypeUnspecified,
    NetPacketLayer3TypeIPv4UnspecifiedOptions,
    NetPacketLayer3TypeIPv4WithOptions,
    NetPacketLayer3TypeIPv4NoOptions,
    NetPacketLayer3TypeIPv6UnspecifiedExtensions,
    NetPacketLayer3TypeIPv6WithExtensions,
    NetPacketLayer3TypeIPv6NoExtensions,
} NET_PACKET_LAYER3_TYPE;

typedef enum _NET_PACKET_LAYER4_TYPE
{
    NetPacketLayer4TypeUnspecified,
    NetPacketLayer4TypeTcp,
    NetPacketLayer4TypeUdp,
    NetPacketLayer4TypeIPFragment,
    NetPacketLayer4TypeIPNotFragment,
} NET_PACKET_LAYER4_TYPE;

#include <pshpack1.h>
typedef struct _NET_PACKET_LAYOUT
{
    UINT16
        Layer2HeaderLength : 7;

    UINT16
        Layer3HeaderLength : 9;

    UINT8
        Layer4HeaderLength : 8;

    /// One of the NET_PACKET_LAYER2_TYPE values
    UINT8
        Layer2Type : 4;

    /// One of the NET_PACKET_LAYER3_TYPE values
    UINT8
        Layer3Type : 4;

    /// One of the NET_PACKET_LAYER4_TYPE values
    UINT8
        Layer4Type : 4;

    UINT8
        Reserved0 : 4;

} NET_PACKET_LAYOUT;
#include <poppack.h>

C_ASSERT(sizeof(NET_PACKET_LAYOUT) == 5);

typedef struct _NET_PACKET
{
    UINT32
        FragmentIndex;

    UINT16
        FragmentCount;

    NET_PACKET_LAYOUT
        Layout;

    UINT8
        Ignore : 1;

    UINT8
        Scratch : 1;

    UINT8
        Reserved1 : 6;

} NET_PACKET;

C_ASSERT(sizeof(NET_PACKET) == 12);

#pragma warning(pop)

inline
BOOLEAN
NetPacketIsIpv4(
    const NET_PACKET * packet
)
{
    return (packet->Layout.Layer3Type == NetPacketLayer3TypeIPv4NoOptions ||
        packet->Layout.Layer3Type == NetPacketLayer3TypeIPv4UnspecifiedOptions ||
        packet->Layout.Layer3Type == NetPacketLayer3TypeIPv4WithOptions);
}

inline
BOOLEAN
NetPacketIsIpv6(
    const NET_PACKET * packet
)
{
    return (packet->Layout.Layer3Type == NetPacketLayer3TypeIPv6NoExtensions ||
        packet->Layout.Layer3Type == NetPacketLayer3TypeIPv6UnspecifiedExtensions ||
        packet->Layout.Layer3Type == NetPacketLayer3TypeIPv6WithExtensions);
}

inline
void *
NetPacketGetExtension(
    const NET_PACKET * packet,
    SIZE_T offset
)
{
    return (void *)((UCHAR*)(packet)+offset);
}

EXTERN_C_END

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

