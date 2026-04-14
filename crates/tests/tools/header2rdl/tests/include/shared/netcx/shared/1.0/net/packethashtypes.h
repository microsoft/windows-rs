// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

EXTERN_C_START

typedef enum _NET_PACKET_HASH_PROTOCOL_TYPE
{
    NetPacketHashProtocolTypeNone = 0x00000000,
    NetPacketHashProtocolTypeIPv4 = 0x00000001,
    NetPacketHashProtocolTypeIPv4Options = 0x00000002,
    NetPacketHashProtocolTypeIPv6 = 0x00000004,
    NetPacketHashProtocolTypeIPv6Extensions = 0x00000008,
    NetPacketHashProtocolTypeTcp = 0x00000010,
    NetPacketHashProtocolTypeUdp = 0x00000020,
} NET_PACKET_HASH_PROTOCOL_TYPE;

typedef struct _NET_PACKET_HASH
{
    UINT32
        HashValue;

    // NET_PACKET_HASH_PROTOCOL_TYPE
    UINT16
        ProtocolType;
} NET_PACKET_HASH;

C_ASSERT(sizeof(NET_PACKET_HASH) == 8);

EXTERN_C_END


#define NET_PACKET_EXTENSION_HASH_NAME L"ms_packet_hash"
#define NET_PACKET_EXTENSION_HASH_VERSION_1 1U

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

