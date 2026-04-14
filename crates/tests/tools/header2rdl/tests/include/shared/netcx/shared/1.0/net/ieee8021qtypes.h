// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

EXTERN_C_START

typedef enum _NET_PACKET_TX_IEEE8021Q_ACTION_FLAGS
{
    NetPacketTxIeee8021qActionFlagPriorityRequired = 1,
    NetPacketTxIeee8021qActionFlagVlanRequired = 2,
} NET_PACKET_TX_IEEE8021Q_ACTION_FLAGS;


typedef struct _NET_PACKET_IEEE8021Q
{
    UINT16
        PriorityCodePoint : 3;

    UINT16
        VlanIdentifier : 12;

    UINT8
        TxTagging : 2;

} NET_PACKET_IEEE8021Q;

C_ASSERT(sizeof(NET_PACKET_IEEE8021Q) == 4);

EXTERN_C_END


#define NET_PACKET_EXTENSION_IEEE8021Q_NAME L"ms_packet_ieee8021q"
#define NET_PACKET_EXTENSION_IEEE8021Q_VERSION_1 1U

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

