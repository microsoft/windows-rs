// computenetwork.ext: ApiSet Contract for ext-ms-win-hyperv-computenetwork-l1
// Copyright (c) Microsoft Corporation. All rights reserved.

#pragma once

#ifndef HCN_CLIENT_H
#define HCN_CLIENT_H

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifdef __cplusplus
extern "C" {
#endif

/////////////////////////////////////////////////////////////////////////
/// Notifications

/// Notifications indicated to callbacks
typedef enum HCN_NOTIFICATIONS
{
       HcnNotificationInvalid                                  = 0x00000000,

       /// Notifications for HCN_SERVICE handles
       HcnNotificationNetworkPreCreate                         = 0x00000001,
       HcnNotificationNetworkCreate                            = 0x00000002,
       HcnNotificationNetworkPreDelete                         = 0x00000003,
       HcnNotificationNetworkDelete                            = 0x00000004,

       /// Namespace Notifications
       HcnNotificationNamespaceCreate                          = 0x00000005,
       HcnNotificationNamespaceDelete                          = 0x00000006,

       /// Notifications for HCN_SERVICE handles
       HcnNotificationGuestNetworkServiceCreate                = 0x00000007,
       HcnNotificationGuestNetworkServiceDelete                = 0x00000008,

       /// Notifications for HCN_NETWORK handles
       HcnNotificationNetworkEndpointAttached                  = 0x00000009,
       HcnNotificationNetworkEndpointDetached                  = 0x00000010,

       /// Notifications for HCN_GUESTNETWORKSERVICE handles
       HcnNotificationGuestNetworkServiceStateChanged          = 0x00000011,
       HcnNotificationGuestNetworkServiceInterfaceStateChanged = 0x00000012,

       /// Common notifications
       HcnNotificationServiceDisconnect                        = 0x01000000,

       /// The upper 4 bits are reserved for flags
       HcnNotificationFlagsReserved                            = 0xF0000000
} HCN_NOTIFICATIONS;

/// Handle to a callback registered on a hns object
typedef void* HCN_CALLBACK;

/// Function type for HNS notification callbacks
typedef void (CALLBACK *HCN_NOTIFICATION_CALLBACK)(
    _In_            DWORD  NotificationType,
    _In_opt_        void*  Context,
    _In_            HRESULT NotificationStatus,
    _In_opt_        PCWSTR NotificationData
    );

/////////////////////////////////////////////////////////////////////////
/// Hcn Networks

/// Context handle referencing a Network in HNS
typedef void*           HCN_NETWORK;
typedef HCN_NETWORK*    PHCN_NETWORK;

/// Return a list of existing Networks
HRESULT
WINAPI
HcnEnumerateNetworks(
    _In_ PCWSTR Query,
    _Outptr_ PWSTR* Networks,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Create a Network
HRESULT
WINAPI
HcnCreateNetwork(
    _In_ REFGUID Id,
    _In_ PCWSTR Settings,
    _Out_ PHCN_NETWORK Network,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Lookup an existing network
HRESULT
WINAPI
HcnOpenNetwork(
    _In_ REFGUID Id,
    _Out_ PHCN_NETWORK Network,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Modify the settings of a Network
HRESULT
WINAPI
HcnModifyNetwork(
    _In_ HCN_NETWORK Network,
    _In_ PCWSTR Settings,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Query Network settings
HRESULT
WINAPI
HcnQueryNetworkProperties(
    _In_ HCN_NETWORK Network,
    _In_ PCWSTR Query,
    _Outptr_ PWSTR* Properties,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Delete a Network
HRESULT
WINAPI
HcnDeleteNetwork(
    _In_ REFGUID Id,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Close a handle to a Network
HRESULT
WINAPI
HcnCloseNetwork(
    _In_ HCN_NETWORK Network
    );

/////////////////////////////////////////////////////////////////////////
/// Hcn Namespace

/// Context handle referencing a Namespace in HNS
typedef void*           HCN_NAMESPACE;
typedef HCN_NAMESPACE*  PHCN_NAMESPACE;

/// Return a list of existing Namespaces
HRESULT
WINAPI
HcnEnumerateNamespaces(
    _In_ PCWSTR Query,
    _Outptr_ PWSTR* Namespaces,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Create a Namespace
HRESULT
WINAPI
HcnCreateNamespace(
    _In_ REFGUID Id,
    _In_ PCWSTR Settings,
    _Out_ PHCN_NAMESPACE Namespace,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Lookup an existing Namespace
HRESULT
WINAPI
HcnOpenNamespace(
    _In_ REFGUID Id,
    _Out_ PHCN_NAMESPACE Namespace,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Modify the settings of a Namespace
HRESULT
WINAPI
HcnModifyNamespace(
    _In_ HCN_NAMESPACE Namespace,
    _In_ PCWSTR Settings,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Query Namespace settings
HRESULT
WINAPI
HcnQueryNamespaceProperties(
    _In_ HCN_NAMESPACE Namespace,
    _In_ PCWSTR Query,
    _Outptr_ PWSTR* Properties,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Delete a Namespace
HRESULT
WINAPI
HcnDeleteNamespace(
    _In_ REFGUID Id,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Close a handle to a Namespace
HRESULT
WINAPI
HcnCloseNamespace(
    _In_ HCN_NAMESPACE Namespace
    );

/////////////////////////////////////////////////////////////////////////
/// Hcn Endpoint

/// Context handle referencing an Endpoint in HNS
typedef void*           HCN_ENDPOINT;
typedef HCN_ENDPOINT*   PHCN_ENDPOINT;

/// Return a list of existing Endpoints
HRESULT
WINAPI
HcnEnumerateEndpoints(
    _In_ PCWSTR Query,
    _Outptr_ PWSTR* Endpoints,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Create an Endpoint
HRESULT
WINAPI
HcnCreateEndpoint(
    _In_ HCN_NETWORK Network,
    _In_ REFGUID Id,
    _In_ PCWSTR Settings,
    _Out_ PHCN_ENDPOINT Endpoint,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Lookup an existing Endpoint
HRESULT
WINAPI
HcnOpenEndpoint(
    _In_ REFGUID Id,
    _Out_ PHCN_ENDPOINT Endpoint,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Modify the settings of an Endpoint
HRESULT
WINAPI
HcnModifyEndpoint(
    _In_ HCN_ENDPOINT Endpoint,
    _In_ PCWSTR Settings,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Query Endpoint properties
HRESULT
WINAPI
HcnQueryEndpointProperties(
    _In_ HCN_ENDPOINT Endpoint,
    _In_ PCWSTR Query,
    _Outptr_ PWSTR* Properties,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Delete an Endpoint
HRESULT
WINAPI
HcnDeleteEndpoint(
    _In_ REFGUID Id,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Close a handle to an Endpoint
HRESULT
WINAPI
HcnCloseEndpoint(
    _In_ HCN_ENDPOINT Endpoint
    );

/////////////////////////////////////////////////////////////////////////
/// Hcn LoadBalancer

/// Context handle referencing a LoadBalancer in HNS
typedef void*               HCN_LOADBALANCER;
typedef HCN_LOADBALANCER*     PHCN_LOADBALANCER;

/// Return a list of existing LoadBalancers
HRESULT
WINAPI
HcnEnumerateLoadBalancers(
    _In_ PCWSTR Query,
    _Outptr_ PWSTR* LoadBalancer,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Create a LoadBalancer
HRESULT
WINAPI
HcnCreateLoadBalancer(
    _In_ REFGUID Id,
    _In_ PCWSTR Settings,
    _Out_ PHCN_LOADBALANCER LoadBalancer,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Lookup an existing LoadBalancer
HRESULT
WINAPI
HcnOpenLoadBalancer(
    _In_ REFGUID Id,
    _Out_ PHCN_LOADBALANCER LoadBalancer,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Modify the settings of a PolcyList
HRESULT
WINAPI
HcnModifyLoadBalancer(
    _In_ HCN_LOADBALANCER LoadBalancer,
    _In_ PCWSTR Settings,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Query PolcyList settings
HRESULT
WINAPI
HcnQueryLoadBalancerProperties(
    _In_ HCN_LOADBALANCER LoadBalancer,
    _In_ PCWSTR Query,
    _Outptr_ PWSTR* Properties,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Delete a LoadBalancer
HRESULT
WINAPI
HcnDeleteLoadBalancer(
    _In_ REFGUID Id,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Close a handle to a LoadBalancer
HRESULT
WINAPI
HcnCloseLoadBalancer(
    _In_ HCN_LOADBALANCER LoadBalancer
    );

/////////////////////////////////////////////////////////////////////////
/// Hcn Service

/// Context handle referencing the HNS service
typedef void* HCN_SERVICE;
typedef HCN_SERVICE* PHCN_SERVICE;

/// Registers a callback function to receive notifications of service-wide events
HRESULT
WINAPI
HcnRegisterServiceCallback(
    _In_ HCN_NOTIFICATION_CALLBACK Callback,
    _In_ void* Context,
    _Outptr_ HCN_CALLBACK* CallbackHandle
    );

/// Unregisters from service-wide notifications
HRESULT
WINAPI
HcnUnregisterServiceCallback(
    _In_ HCN_CALLBACK CallbackHandle
    );

/////////////////////////////////////////////////////////////////////////
/// Hcn GuestNetworkService

/// Context handle referencing a GuestNetworkService in HNS
typedef void*                     HCN_GUESTNETWORKSERVICE;
typedef HCN_GUESTNETWORKSERVICE*  PHCN_GUESTNETWORKSERVICE;

/// Registers a callback function to receive GuestNetworkService notifications
HRESULT
WINAPI
HcnRegisterGuestNetworkServiceCallback(
    _In_ HCN_GUESTNETWORKSERVICE GuestNetworkService,
    _In_ HCN_NOTIFICATION_CALLBACK Callback,
    _In_ void* Context,
    _Outptr_ HCN_CALLBACK* CallbackHandle
    );

/// Unregisters from GuestNetworkService notifications
HRESULT
WINAPI
HcnUnregisterGuestNetworkServiceCallback(
    _In_ HCN_CALLBACK CallbackHandle
    );

/// Create a GuestNetworkService
HRESULT
WINAPI
HcnCreateGuestNetworkService(
    _In_ REFGUID Id,
    _In_ PCWSTR Settings,
    _Out_ PHCN_GUESTNETWORKSERVICE GuestNetworkService,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Close a handle to a GuestNetworkService
HRESULT
WINAPI
HcnCloseGuestNetworkService(
    _In_ HCN_GUESTNETWORKSERVICE GuestNetworkService
    );

/// Modify the settings of a PolcyList
HRESULT
WINAPI
HcnModifyGuestNetworkService(
    _In_ HCN_GUESTNETWORKSERVICE GuestNetworkService,
    _In_ PCWSTR Settings,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Delete a GuestNetworkService
HRESULT
WINAPI
HcnDeleteGuestNetworkService(
    _In_ REFGUID Id,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/////////////////////////////////////////////////////////////////////////
/// Hcn Port Reservation

typedef enum tagHCN_PORT_PROTOCOL
{
    HCN_PORT_PROTOCOL_TCP = 0x01,
    HCN_PORT_PROTOCOL_UDP = 0x02,
    HCN_PORT_PROTOCOL_BOTH = 0x03
} HCN_PORT_PROTOCOL;

typedef enum tagHCN_PORT_ACCESS
{
    HCN_PORT_ACCESS_EXCLUSIVE = 0x01,
    HCN_PORT_ACCESS_SHARED = 0x02
} HCN_PORT_ACCESS;

typedef struct tagHCN_PORT_RANGE_RESERVATION
{
    // start and end are inclusive
    USHORT startingPort;
    USHORT endingPort;
} HCN_PORT_RANGE_RESERVATION;

typedef struct tagHCN_PORT_RANGE_ENTRY {
    GUID OwningPartitionId;
    GUID TargetPartitionId;
    HCN_PORT_PROTOCOL Protocol;
    UINT64 Priority;
    UINT32 ReservationType;
    UINT32 SharingFlags;
    UINT32 DeliveryMode;
    UINT16 StartingPort;
    UINT16 EndingPort;
} HCN_PORT_RANGE_ENTRY, *PHCN_PORT_RANGE_ENTRY;

// reserves a single port for a target gnsHandle
HRESULT
WINAPI
HcnReserveGuestNetworkServicePort(
    _In_ HCN_GUESTNETWORKSERVICE GuestNetworkService,
    _In_ HCN_PORT_PROTOCOL Protocol,
    _In_ HCN_PORT_ACCESS Access,
    _In_ USHORT Port,
    _Out_ HANDLE* PortReservationHandle
    );

// range is always for exclusive access for both protocols
HRESULT
WINAPI
HcnReserveGuestNetworkServicePortRange(
    _In_ HCN_GUESTNETWORKSERVICE GuestNetworkService,
    _In_ USHORT PortCount,
    _Out_ HCN_PORT_RANGE_RESERVATION* PortRangeReservation,
    _Out_ HANDLE* PortReservationHandle
    );

HRESULT
WINAPI
HcnReleaseGuestNetworkServicePortReservationHandle(
    _In_ HANDLE PortReservationHandle
    );

HRESULT
WINAPI
HcnEnumerateGuestNetworkPortReservations(
    _Out_ ULONG* ReturnCount,
    _Out_writes_bytes_all_(*ReturnCount) HCN_PORT_RANGE_ENTRY** PortEntries
    );

VOID
WINAPI
HcnFreeGuestNetworkPortReservations(
    _Inout_opt_ HCN_PORT_RANGE_ENTRY* PortEntries
    );

/////////////////////////////////////////////////////////////////////////
/// Hcn Endpoint (added in NI)

/// Query Endpoint stats
HRESULT
WINAPI
HcnQueryEndpointStats(
    _In_ HCN_ENDPOINT Endpoint,
    _In_ PCWSTR Query,
    _Outptr_ PWSTR* Stats,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

/// Query Endpoint addresses
HRESULT
WINAPI
HcnQueryEndpointAddresses(
    _In_ HCN_ENDPOINT Endpoint,
    _In_ PCWSTR Query,
    _Outptr_ PWSTR* Addresses,
    _Outptr_opt_ PWSTR* ErrorRecord
    );

#ifdef __cplusplus
}
#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif

#ifndef ext_ms_win_hyperv_computenetwork_l1_1_1_query_routines
#define ext_ms_win_hyperv_computenetwork_l1_1_1_query_routines

//
//Private Extension API Query Routines
//

#ifdef __cplusplus
extern "C" {
#endif

BOOLEAN
__stdcall
IsHcnEnumerateNetworksPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnCreateNetworkPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnOpenNetworkPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnModifyNetworkPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnQueryNetworkPropertiesPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnDeleteNetworkPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnCloseNetworkPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnEnumerateNamespacesPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnCreateNamespacePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnOpenNamespacePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnModifyNamespacePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnQueryNamespacePropertiesPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnDeleteNamespacePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnCloseNamespacePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnEnumerateEndpointsPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnCreateEndpointPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnOpenEndpointPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnModifyEndpointPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnQueryEndpointPropertiesPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnDeleteEndpointPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnCloseEndpointPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnEnumerateLoadBalancersPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnCreateLoadBalancerPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnOpenLoadBalancerPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnModifyLoadBalancerPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnQueryLoadBalancerPropertiesPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnDeleteLoadBalancerPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnCloseLoadBalancerPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnRegisterServiceCallbackPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnUnregisterServiceCallbackPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnRegisterGuestNetworkServiceCallbackPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnUnregisterGuestNetworkServiceCallbackPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnCreateGuestNetworkServicePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnCloseGuestNetworkServicePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnModifyGuestNetworkServicePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnDeleteGuestNetworkServicePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnReserveGuestNetworkServicePortPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnReserveGuestNetworkServicePortRangePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnReleaseGuestNetworkServicePortReservationHandlePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnEnumerateGuestNetworkPortReservationsPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnFreeGuestNetworkPortReservationsPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnQueryEndpointStatsPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcnQueryEndpointAddressesPresent(
    VOID
    );

#ifdef __cplusplus
}
#endif

#endif // endof guard

