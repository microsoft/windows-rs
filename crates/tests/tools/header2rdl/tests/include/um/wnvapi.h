/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    wnvapi.h

Abstract:
    Header file for Windows Network Virtualization API's.

--*/

#ifndef __WNV_API_INCLUDED_
#define __WNV_API_INCLUDED_

#ifdef _MSC_VER
#pragma once
#endif  // _MSC_VER

#include <winapifamily.h>

#pragma region Desktop Family or WNV Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WNV)

#if NTDDI_VERSION >= NTDDI_WIN8
#define WNV_API_MAJOR_VERSION_1 1
#define WNV_API_MINOR_VERSION_0 0

typedef enum _WNV_NOTIFICATION_TYPE {
    WnvPolicyMismatchType,
    WnvRedirectType,
    WnvObjectChangeType,
    WnvNotificationTypeMax
} WNV_NOTIFICATION_TYPE, *PWNV_NOTIFICATION_TYPE;

typedef enum _WNV_OBJECT_TYPE {
    WnvProviderAddressType,
    WnvCustomerAddressType,
    WnvObjectTypeMax
} WNV_OBJECT_TYPE, *PWNV_OBJECT_TYPE;

typedef enum _WNV_CA_NOTIFICATION_TYPE {
    WnvCustomerAddressAdded,
    WnvCustomerAddressDeleted,
    WnvCustomerAddressMoved,
    WnvCustomerAddressMax
} WNV_CA_NOTIFICATION_TYPE, *PWNV_CA_NOTIFICATION_TYPE;

typedef struct _WNV_OBJECT_HEADER {
    UCHAR MajorVersion;
    UCHAR MinorVersion;
    ULONG Size;
} WNV_OBJECT_HEADER, *PWNV_OBJECT_HEADER;

typedef struct _WNV_NOTIFICATION_PARAM {
    WNV_OBJECT_HEADER Header;
    WNV_NOTIFICATION_TYPE NotificationType;
    ULONG PendingNotifications;
    PUCHAR Buffer;
} WNV_NOTIFICATION_PARAM, *PWNV_NOTIFICATION_PARAM;

typedef struct _WNV_IP_ADDRESS {
    union {
        IN_ADDR v4;
        IN6_ADDR v6;
        UCHAR Addr[sizeof(IN6_ADDR)];
    } IP;
} WNV_IP_ADDRESS, *PWNV_IP_ADDRESS;

typedef struct _WNV_POLICY_MISMATCH_PARAM {
    ADDRESS_FAMILY CAFamily;
    ADDRESS_FAMILY PAFamily;
    ULONG VirtualSubnetId;
    WNV_IP_ADDRESS CA;
    WNV_IP_ADDRESS PA;
} WNV_POLICY_MISMATCH_PARAM, *PWNV_POLICY_MISMATCH_PARAM;

typedef struct _WNV_PROVIDER_ADDRESS_CHANGE_PARAM {
    ADDRESS_FAMILY PAFamily;
    WNV_IP_ADDRESS PA;
    NL_DAD_STATE AddressState;
} WNV_PROVIDER_ADDRESS_CHANGE_PARAM, *PWNV_PROVIDER_ADDRESS_CHANGE_PARAM;

typedef struct _WNV_CUSTOMER_ADDRESS_CHANGE_PARAM {
    DL_EUI48 MACAddress;
    ADDRESS_FAMILY CAFamily;
    WNV_IP_ADDRESS CA;
    ULONG VirtualSubnetId;
    ADDRESS_FAMILY PAFamily;
    WNV_IP_ADDRESS PA;
    WNV_CA_NOTIFICATION_TYPE NotificationReason;
} WNV_CUSTOMER_ADDRESS_CHANGE_PARAM, *PWNV_CUSTOMER_ADDRESS_CHANGE_PARAM;

typedef struct _WNV_OBJECT_CHANGE_PARAM {
    WNV_OBJECT_TYPE ObjectType;
    union {
        WNV_PROVIDER_ADDRESS_CHANGE_PARAM ProviderAddressChange;
        WNV_CUSTOMER_ADDRESS_CHANGE_PARAM CustomerAddressChange;
    } ObjectParam;
} WNV_OBJECT_CHANGE_PARAM, *PWNV_OBJECT_CHANGE_PARAM;

typedef struct _WNV_REDIRECT_PARAM {
    ADDRESS_FAMILY CAFamily;
    ADDRESS_FAMILY PAFamily;
    ADDRESS_FAMILY NewPAFamily;
    ULONG VirtualSubnetId;
    WNV_IP_ADDRESS CA;
    WNV_IP_ADDRESS PA;
    WNV_IP_ADDRESS NewPA;
} WNV_REDIRECT_PARAM, *PWNV_REDIRECT_PARAM;

#ifndef WNV_KERNEL_MODE
#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus
HANDLE
WINAPI
WnvOpen(
    );

ULONG
WINAPI
WnvRequestNotification (
    HANDLE WnvHandle,
    PWNV_NOTIFICATION_PARAM NotificationParam,
    LPOVERLAPPED Overlapped,
    PULONG BytesTransferred
    );

#ifdef __cplusplus
}
#endif  // __cplusplus
#endif
#endif //#if NTDDI_VERSION >= NTDDI_WIN8

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WNV) */
#pragma endregion

#endif //__WNV_API_INCLUDED_
