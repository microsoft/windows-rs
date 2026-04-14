// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region System Family (kernel drivers) with Desktop Family for compat
#include <winapifamily.h>
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_DESKTOP)

#include <ndis/version.h>

EXTERN_C_START

#if NDIS_SUPPORT_NDIS684

//
// Per-NetBufferList information for UdpRecvSegCoalesceOffloadInfo.
//
typedef struct _NDIS_UDP_RSC_OFFLOAD_NET_BUFFER_LIST_INFO
{
    union
    {
        struct
        {
            USHORT SegCount;
            USHORT SegSize;
        } Receive;

        PVOID Value;
    };
} NDIS_UDP_RSC_OFFLOAD_NET_BUFFER_LIST_INFO, *PNDIS_UDP_RSC_OFFLOAD_NET_BUFFER_LIST_INFO;

#if (NDIS_SUPPORT_NDIS689)

//
// values used in UDP RSC offload
//
#define NDIS_OFFLOAD_PARAMETERS_UDP_RSC_NO_CHANGE       0
#define NDIS_OFFLOAD_PARAMETERS_UDP_RSC_DISABLED        1
#define NDIS_OFFLOAD_PARAMETERS_UDP_RSC_ENABLED         2

typedef struct _NDIS_UDP_RSC_OFFLOAD
{
    BOOLEAN Enabled;
} NDIS_UDP_RSC_OFFLOAD;

#endif // (NDIS_SUPPORT_NDIS689)

#if defined(NDIS_INCLUDE_LEGACY_NAMES) || !defined(__cplusplus)

#define NET_BUFFER_LIST_UDP_COALESCED_SEG_COUNT(_NBL) \
    (((NDIS_UDP_RSC_OFFLOAD_NET_BUFFER_LIST_INFO*) \
        &(_NBL)->NetBufferListInfo[UdpRecvSegCoalesceOffloadInfo])->Receive.SegCount)

#define NET_BUFFER_LIST_UDP_COALESCED_SEG_SIZE(_NBL) \
    (((NDIS_UDP_RSC_OFFLOAD_NET_BUFFER_LIST_INFO*) \
        &(_NBL)->NetBufferListInfo[UdpRecvSegCoalesceOffloadInfo])->Receive.SegSize)

#if NDIS_SUPPORT_NDIS689

#define NET_BUFFER_LIST_IS_URO_SET(_NBL) \
    ((((UINT_PTR)((NDIS_UDP_RSC_OFFLOAD_NET_BUFFER_LIST_INFO const *) \
        &(_NBL)->NetBufferListInfo[UdpRecvSegCoalesceOffloadInfo])->Value) & 0xFFFFFFFF) != 0)

#endif // NDIS_SUPPORT_NDIS689

#else // defined(NDIS_INCLUDE_LEGACY_NAMES) || !defined(__cplusplus)

inline
UINT32 &
NET_BUFFER_LIST_UDP_COALESCED_SEG_COUNT(
    _In_ NET_BUFFER_LIST                       *Nbl
    )
{
    NDIS_UDP_RSC_OFFLOAD_NET_BUFFER_LIST_INFO *Info = (NDIS_UDP_RSC_OFFLOAD_NET_BUFFER_LIST_INFO *)
        &Nbl->NetBufferListInfo[UdpRecvSegCoalesceOffloadInfo];
    return Info->Receive.SegCount;
}

inline
UINT32 const &
NET_BUFFER_LIST_UDP_COALESCED_SEG_COUNT(
    _In_ NET_BUFFER_LIST const                 *Nbl
    )
{
    NDIS_UDP_RSC_OFFLOAD_NET_BUFFER_LIST_INFO const *Info = (NDIS_UDP_RSC_OFFLOAD_NET_BUFFER_LIST_INFO const *)
        &Nbl->NetBufferListInfo[UdpRecvSegCoalesceOffloadInfo];
    return Info->Receive.SegCount;
}

inline
UINT32 &
NET_BUFFER_LIST_UDP_COALESCED_SEG_SIZE(
    _In_ NET_BUFFER_LIST                       *Nbl
    )
{
    NDIS_UDP_RSC_OFFLOAD_NET_BUFFER_LIST_INFO *Info = (NDIS_UDP_RSC_OFFLOAD_NET_BUFFER_LIST_INFO *)
        &Nbl->NetBufferListInfo[UdpRecvSegCoalesceOffloadInfo];
    return Info->Receive.SegSize;
}

inline
UINT32 const &
NET_BUFFER_LIST_UDP_COALESCED_SEG_SIZE(
    _In_ NET_BUFFER_LIST const                 *Nbl
    )
{
    NDIS_UDP_RSC_OFFLOAD_NET_BUFFER_LIST_INFO const *Info = (NDIS_UDP_RSC_OFFLOAD_NET_BUFFER_LIST_INFO const *)
        &Nbl->NetBufferListInfo[UdpRecvSegCoalesceOffloadInfo];
    return Info->Receive.SegSize;
}

#if NDIS_SUPPORT_NDIS689

inline
BOOLEAN
NET_BUFFER_LIST_IS_URO_SET(
    _In_ NET_BUFFER_LIST const *Nbl
    )
{
    NDIS_UDP_RSC_OFFLOAD_NET_BUFFER_LIST_INFO const *Info = (NDIS_UDP_RSC_OFFLOAD_NET_BUFFER_LIST_INFO const *)
        &Nbl->NetBufferListInfo[UdpRecvSegCoalesceOffloadInfo];
    return ((UINT_PTR)Info->Value & 0xFFFFFFFF) != 0;
}

#endif // NDIS_SUPPORT_NDIS689

#endif // defined(NDIS_INCLUDE_LEGACY_NAMES) || !defined(__cplusplus)

#endif // NDIS_SUPPORT_NDIS684

EXTERN_C_END

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_DESKTOP)
#pragma endregion

