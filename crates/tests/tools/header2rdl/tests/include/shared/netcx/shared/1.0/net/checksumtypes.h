// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

EXTERN_C_START

typedef enum _NET_PACKET_TX_CHECKSUM_ACTION
{
    NetPacketTxChecksumActionPassthrough = 0,
    NetPacketTxChecksumActionRequired = 2,
} NET_PACKET_TX_CHECKSUM_ACTION;

typedef enum _NET_PACKET_RX_CHECKSUM_EVALUATION
{
    NetPacketRxChecksumEvaluationNotChecked = 0,
    NetPacketRxChecksumEvaluationValid = 1,
    NetPacketRxChecksumEvaluationInvalid = 2,
} NET_PACKET_RX_CHECKSUM_EVALUATION;

#pragma warning(push)
#pragma warning(default:4820) // warn if the compiler inserted padding

typedef struct _NET_PACKET_CHECKSUM
{
    // One of NET_PACKET_TX_CHECKSUM_ACTION or NET_PACKET_RX_CHECKSUM_EVALUATION
    UINT8
        Layer2 : 2;

    // One of NET_PACKET_TX_CHECKSUM_ACTION or NET_PACKET_RX_CHECKSUM_EVALUATION
    UINT8
        Layer3 : 2;

    // One of NET_PACKET_TX_CHECKSUM_ACTION or NET_PACKET_RX_CHECKSUM_EVALUATION
    UINT8
        Layer4 : 2;

    UINT8
        Reserved : 2;

} NET_PACKET_CHECKSUM;

C_ASSERT(sizeof(NET_PACKET_CHECKSUM) == 1);

#pragma warning(pop)

EXTERN_C_END


#define NET_PACKET_EXTENSION_CHECKSUM_NAME L"ms_packet_checksum"
#define NET_PACKET_EXTENSION_CHECKSUM_VERSION_1 1U

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

