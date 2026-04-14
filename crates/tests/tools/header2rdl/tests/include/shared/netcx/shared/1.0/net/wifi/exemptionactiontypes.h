// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

EXTERN_C_START

#pragma warning(push)
#pragma warning(default:4820) // warn if the compiler inserted padding

typedef struct _NET_PACKET_WIFI_EXEMPTION_ACTION
{
    UINT8 ExemptionAction;

} NET_PACKET_WIFI_EXEMPTION_ACTION;

C_ASSERT(sizeof(NET_PACKET_WIFI_EXEMPTION_ACTION) == sizeof(UINT8));

#pragma warning(pop)

EXTERN_C_END


#define NET_PACKET_EXTENSION_WIFI_EXEMPTION_ACTION_NAME L"ms_packet_wifiexemptionaction"
#define NET_PACKET_EXTENSION_WIFI_EXEMPTION_ACTION_VERSION_1 1U

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

