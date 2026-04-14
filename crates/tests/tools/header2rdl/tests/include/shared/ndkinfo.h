/*++

Copyright (c) Microsoft Corporation

Module Name:

    ndkinfo.h

Abstract:

    NetworkDirect adapter information

Environment:

    User mode and kernel  mode

--*/

#ifndef _NDKINFO_H_
#define _NDKINFO_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct {
    USHORT Major;
    USHORT Minor;
} NDK_VERSION;

typedef enum _NDK_RDMA_TECHNOLOGY {
    NdkUndefined = 0,
    NdkiWarp,
    NdkInfiniBand,
    NdkRoCE,
    NdkRoCEv2,
    NdkMaxTechnology
} NDK_RDMA_TECHNOLOGY;

typedef struct _NDK_ADAPTER_INFO {

    NDK_VERSION Version;

    UINT32 VendorId;

    UINT32 DeviceId;

    SIZE_T MaxRegistrationSize;

    SIZE_T MaxWindowSize;

    ULONG FRMRPageCount;
    
    ULONG MaxInitiatorRequestSge;

    ULONG MaxReceiveRequestSge;

    ULONG MaxReadRequestSge;

    ULONG MaxTransferLength;

    ULONG MaxInlineDataSize;

    ULONG MaxInboundReadLimit;

    ULONG MaxOutboundReadLimit;

    ULONG MaxReceiveQueueDepth;

    ULONG MaxInitiatorQueueDepth;

    ULONG MaxSrqDepth;

    ULONG MaxCqDepth;

    ULONG LargeRequestThreshold;

    ULONG MaxCallerData;

    ULONG MaxCalleeData;

    ULONG AdapterFlags;

    NDK_RDMA_TECHNOLOGY RdmaTechnology;
} NDK_ADAPTER_INFO;

#define NDK_ADAPTER_FLAG_IN_ORDER_DMA_SUPPORTED            0x00000001
#define NDK_ADAPTER_FLAG_RDMA_READ_SINK_NOT_REQUIRED       0x00000002
#define NDK_ADAPTER_FLAG_CQ_INTERRUPT_MODERATION_SUPPORTED 0x00000004
#define NDK_ADAPTER_FLAG_MULTI_ENGINE_SUPPORTED            0x00000008
#define NDK_ADAPTER_FLAG_RDMA_READ_LOCAL_INVALIDATE_SUPPORTED 0x00000010
#define NDK_ADAPTER_FLAG_CQ_RESIZE_SUPPORTED               0x00000100
#define NDK_ADAPTER_FLAG_LOOPBACK_CONNECTIONS_SUPPORTED    0x00010000

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _NDKINFO_H_

