//
//    Copyright (C) Microsoft.  All rights reserved.
//
/*++

Module Name:

    netioapi.h

Abstract:

    This module contains definitions for version agnostic IP helper APIs.


Environment:

    User mode or kernel mode.

    In user mode this file should be included from iphlpapi.h,
    after including the following headers:

    #include <ws2def.h>
    #include <ws2ipdef.h>

    In kernel mode the following files should be included.

    #include <ntddk.h>

    For all APIs, the caller IRQL should be less than DISPATCH_LEVEL.

Notes:

    Please follow these notes to maintain consistency within this API set,
    as well as across the legacy IP helper API:

    1. Express objects and functions in terms of MIB operations if possible.
       E.g. MIB_UNICASTIPADDRESS_ROW structure is associated with the
       GetUnicastIpAddress() function.

    2. Use the NETIO/RTL coding convention for naming structures and functions.
       When modifying a structure with a legacy IP helper API counterpart,
       try to maintain the same name (without abbreviations).
       The hungarian convention must not be used and, for structure fields,
       the structure name need not prefix the field.

    3. For versioning a new structure or API,
       append a version number to the constructs.

    4. The functions in this file are grouped by the object they operate upon.
       Within the group, they are sorted alphabetically.

--*/

#ifndef _NETIOAPI_H_
#define _NETIOAPI_H_
#pragma once
#include <winapifamily.h>

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifdef __cplusplus
extern "C" {
#endif

//
// General types.
//

//
// Nameless structure/union.
//
#pragma warning(push)
#pragma warning(disable:4201)
#pragma warning(disable:4214)

#ifndef ANY_SIZE
#define ANY_SIZE 1
#endif

#ifdef __IPHLPAPI_H__

//
// User Mode.  This file should be included from iphlpapi.h
// User should include ws2ipdef.h to use these APIs.
//

#define NETIO_STATUS DWORD
#define NETIO_SUCCESS(x) ((x) == NO_ERROR)
#define NETIOAPI_API_ WINAPI
#define _NETIOAPI_SUCCESS_ _Success_(return==NO_ERROR)
#else

//
// Kernel Mode.
//

#include <ws2def.h>
#include <ws2ipdef.h>
#include <ifdef.h>
#include <nldef.h>

#define NETIO_STATUS NTSTATUS
#define NETIO_SUCCESS(x) NT_SUCCESS(x)
#define NETIOAPI_API_ NTAPI
#define _NETIOAPI_SUCCESS_

#ifdef IPHLPAPI_DLL_LINKAGE
#undef IPHLPAPI_DLL_LINKAGE
#endif
#define IPHLPAPI_DLL_LINKAGE

#endif /* __IPHLPAPI_H__ */

#define NETIOAPI_API NETIO_STATUS NETIOAPI_API_

typedef enum _MIB_NOTIFICATION_TYPE {
    //
    // ParameterChange.
    //
    MibParameterNotification,
    //
    // Addition.
    //
    MibAddInstance,
    //
    // Deletion.
    //
    MibDeleteInstance,
    //
    // Initial notification.
    //
    MibInitialNotification,
} MIB_NOTIFICATION_TYPE, *PMIB_NOTIFICATION_TYPE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

//
// Interface management routines.
//

//
// The MIB structure for Interface management routines.
//

#ifdef _WS2IPDEF_

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#include <ntddndis.h>

typedef struct _MIB_IF_ROW2 {
    //
    // Key structure.  Sorted by preference.
    //
    NET_LUID InterfaceLuid;
    NET_IFINDEX InterfaceIndex;

    //
    // Read-Only fields.
    //
    GUID InterfaceGuid;
    WCHAR Alias[IF_MAX_STRING_SIZE + 1];
    WCHAR Description[IF_MAX_STRING_SIZE + 1];
    ULONG PhysicalAddressLength;
    UCHAR PhysicalAddress[IF_MAX_PHYS_ADDRESS_LENGTH];
    UCHAR PermanentPhysicalAddress[IF_MAX_PHYS_ADDRESS_LENGTH];

    ULONG Mtu;
    IFTYPE Type;                // Interface Type.
    TUNNEL_TYPE TunnelType;     // Tunnel Type, if Type = IF_TUNNEL.
    NDIS_MEDIUM MediaType;
    NDIS_PHYSICAL_MEDIUM PhysicalMediumType;
    NET_IF_ACCESS_TYPE AccessType;
    NET_IF_DIRECTION_TYPE DirectionType;
    struct {
        BOOLEAN HardwareInterface : 1;
        BOOLEAN FilterInterface : 1;
        BOOLEAN ConnectorPresent : 1;
        BOOLEAN NotAuthenticated : 1;
        BOOLEAN NotMediaConnected : 1;
        BOOLEAN Paused : 1;
        BOOLEAN LowPower : 1;
        BOOLEAN EndPointInterface : 1;
    } InterfaceAndOperStatusFlags;

    IF_OPER_STATUS OperStatus;
    NET_IF_ADMIN_STATUS AdminStatus;
    NET_IF_MEDIA_CONNECT_STATE MediaConnectState;
    NET_IF_NETWORK_GUID NetworkGuid;
    NET_IF_CONNECTION_TYPE ConnectionType;

    //
    // Statistics.
    //
    ULONG64 TransmitLinkSpeed;
    ULONG64 ReceiveLinkSpeed;

    ULONG64 InOctets;
    ULONG64 InUcastPkts;
    ULONG64 InNUcastPkts;
    ULONG64 InDiscards;
    ULONG64 InErrors;
    ULONG64 InUnknownProtos;
    ULONG64 InUcastOctets;
    ULONG64 InMulticastOctets;
    ULONG64 InBroadcastOctets;
    ULONG64 OutOctets;
    ULONG64 OutUcastPkts;
    ULONG64 OutNUcastPkts;
    ULONG64 OutDiscards;
    ULONG64 OutErrors;
    ULONG64 OutUcastOctets;
    ULONG64 OutMulticastOctets;
    ULONG64 OutBroadcastOctets;
    ULONG64 OutQLen;
} MIB_IF_ROW2, *PMIB_IF_ROW2;

typedef struct _MIB_IF_TABLE2 {
    ULONG NumEntries;
    MIB_IF_ROW2 Table[ANY_SIZE];
} MIB_IF_TABLE2, *PMIB_IF_TABLE2;

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetIfEntry2(
    _Inout_ PMIB_IF_ROW2 Row
    );
/*++

Routine Description:

    Retrieves information for the specified interface on the local computer.

Arguments:

    Row - Supplies a MIB_IF_ROW2 structure with either the Luid or Index
        initialized to that of the interface for which to retrieve
        information.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1.  At least one of InterfaceLuid or InterfaceIndex must be specified.

    On output, the remaining fields of Row are filled in.

--*/

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

typedef enum _MIB_IF_ENTRY_LEVEL {
    MibIfEntryNormal = 0,
    MibIfEntryNormalWithoutStatistics = 2
} MIB_IF_ENTRY_LEVEL, *PMIB_IF_ENTRY_LEVEL;

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetIfEntry2Ex(
    _In_    MIB_IF_ENTRY_LEVEL Level,
    _Inout_ PMIB_IF_ROW2       Row
    );
/*++

Routine Description:

    Retrieves information for the specified interface on the local computer.

Arguments:

    Level - Value from MIB_IF_ENTRY_LEVEL indicating how much information
            should be retrieved

    Row - Supplies a MIB_IF_ROW2 structure with either the Luid or Index
          initialized to that of the interface for which to retrieve
          information.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1.  At least one of InterfaceLuid or InterfaceIndex must be specified.

    On output, the remaining fields of Row are filled in.

    The fields prefixed with "In" or "Out" will always be zero if
    MibIfEntryNormalWithoutStatistics is used.

--*/
#endif

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetIfTable2(
    _Outptr_ PMIB_IF_TABLE2 *Table
    );
/*++

Routine Description:

    Retrieves the MIB-II interface table.

Arguments:

    Table - Returns the table of interfaces in a MIB_IFTABLE2 structure.
        Use FreeMibTable to free this buffer.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    The API allocates the buffer for Table.  Use FreeMibTable to free it.

--*/

typedef enum _MIB_IF_TABLE_LEVEL {
    MibIfTableNormal,
    MibIfTableRaw,
    #if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
    MibIfTableNormalWithoutStatistics
    #endif
} MIB_IF_TABLE_LEVEL, *PMIB_IF_TABLE_LEVEL;

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetIfTable2Ex(
    _In_ MIB_IF_TABLE_LEVEL Level,
    _Outptr_ PMIB_IF_TABLE2 *Table
    );
/*++

Routine Description:

    Retrieves the MIB-II interface table.

Arguments:

    Table - Returns the table of interfaces in a MIB_IFTABLE2 structure.
        Use FreeMibTable to free this buffer.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    The API allocates the buffer for Table.  Use FreeMibTable to free it.

    The fields prefixed with "In" or "Out" will always be zero if
    MibIfEntryNormalWithoutStatistics is used.

--*/

//
// IpInterface management routines.
//

//
// The MIB structure for Network layer Interface management routines.
//
typedef struct _MIB_IPINTERFACE_ROW {
    //
    // Key Structure;
    //
    ADDRESS_FAMILY Family;
    NET_LUID InterfaceLuid;
    NET_IFINDEX InterfaceIndex;

    //
    // Read-Write fields.
    //

    //
    // Fields currently not exposed.
    //
    ULONG MaxReassemblySize;
    ULONG64 InterfaceIdentifier;
    ULONG MinRouterAdvertisementInterval;
    ULONG MaxRouterAdvertisementInterval;

    //
    // Fileds currently exposed.
    //
    BOOLEAN AdvertisingEnabled;
    BOOLEAN ForwardingEnabled;
    BOOLEAN WeakHostSend;
    BOOLEAN WeakHostReceive;
    BOOLEAN UseAutomaticMetric;
    BOOLEAN UseNeighborUnreachabilityDetection;
    BOOLEAN ManagedAddressConfigurationSupported;
    BOOLEAN OtherStatefulConfigurationSupported;
    BOOLEAN AdvertiseDefaultRoute;

    NL_ROUTER_DISCOVERY_BEHAVIOR RouterDiscoveryBehavior;
    ULONG DadTransmits;         // DupAddrDetectTransmits in RFC 4862.
    ULONG BaseReachableTime;
    ULONG RetransmitTime;
    ULONG PathMtuDiscoveryTimeout; // Path MTU discovery timeout (in ms).

    NL_LINK_LOCAL_ADDRESS_BEHAVIOR LinkLocalAddressBehavior;
    ULONG LinkLocalAddressTimeout; // In ms.
    ULONG ZoneIndices[ScopeLevelCount]; // Zone part of a SCOPE_ID.
    ULONG SitePrefixLength;
    ULONG Metric;
    ULONG NlMtu;

    //
    // Read Only fields.
    //
    BOOLEAN Connected;
    BOOLEAN SupportsWakeUpPatterns;
    BOOLEAN SupportsNeighborDiscovery;
    BOOLEAN SupportsRouterDiscovery;

    ULONG ReachableTime;

    NL_INTERFACE_OFFLOAD_ROD TransmitOffload;
    NL_INTERFACE_OFFLOAD_ROD ReceiveOffload;

    //
    // Disables using default route on the interface. This flag
    // can be used by VPN clients to restrict Split tunnelling.
    //
    BOOLEAN DisableDefaultRoutes;
} MIB_IPINTERFACE_ROW, *PMIB_IPINTERFACE_ROW;

typedef struct _MIB_IPINTERFACE_TABLE {
    ULONG NumEntries;
    MIB_IPINTERFACE_ROW Table[ANY_SIZE];
} MIB_IPINTERFACE_TABLE, *PMIB_IPINTERFACE_TABLE;

typedef struct _MIB_IFSTACK_ROW {
    NET_IFINDEX HigherLayerInterfaceIndex;
    NET_IFINDEX LowerLayerInterfaceIndex;
} MIB_IFSTACK_ROW, *PMIB_IFSTACK_ROW;

typedef struct _MIB_INVERTEDIFSTACK_ROW {
    NET_IFINDEX LowerLayerInterfaceIndex;
    NET_IFINDEX HigherLayerInterfaceIndex;
} MIB_INVERTEDIFSTACK_ROW, *PMIB_INVERTEDIFSTACK_ROW;

typedef struct _MIB_IFSTACK_TABLE {
    ULONG NumEntries;
    MIB_IFSTACK_ROW Table[ANY_SIZE];
} MIB_IFSTACK_TABLE, *PMIB_IFSTACK_TABLE;

typedef struct _MIB_INVERTEDIFSTACK_TABLE {
    ULONG NumEntries;
    MIB_INVERTEDIFSTACK_ROW Table[ANY_SIZE];
} MIB_INVERTEDIFSTACK_TABLE, *PMIB_INVERTEDIFSTACK_TABLE;

typedef
VOID
(NETIOAPI_API_ *PIPINTERFACE_CHANGE_CALLBACK) (
    _In_ PVOID CallerContext,
    _In_ PMIB_IPINTERFACE_ROW Row OPTIONAL,
    _In_ MIB_NOTIFICATION_TYPE NotificationType
    );

typedef struct _MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    NL_BANDWIDTH_INFORMATION InboundBandwidthInformation;
    NL_BANDWIDTH_INFORMATION OutboundBandwidthInformation;
} MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES,
  *PMIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES;

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetIfStackTable(
    _Outptr_ PMIB_IFSTACK_TABLE *Table
    );

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetInvertedIfStackTable(
    _Outptr_ PMIB_INVERTEDIFSTACK_TABLE *Table
    );

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetIpInterfaceEntry(
    _Inout_ PMIB_IPINTERFACE_ROW Row
    );
/*++

Routine Description:

    Retrieves IP information for the specified interface on the local computer.

Arguments:

    Row - Supplies a MIB_IPINTERFACE_ROW structure with either the Luid or
        Index initialized to that of the interface for which to retrieve
        information.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1. Family: it must be either AF_INET or AF_INET6
    2. At least one of InterfaceLuid or InterfaceIndex must be specified.

    On output, the remaining fields of Row are filled in.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetIpInterfaceTable(
    _In_ ADDRESS_FAMILY Family,
    _Outptr_ PMIB_IPINTERFACE_TABLE *Table
    );
/*++

Routine Description:

    Retrieves the network-layer interface table.

Arguments:

    Family - Supplies the address family.

        AF_INET: Only returns IPv4 MIB entries.

        AF_INET6: Only returns IPv6 MIB entries.

        AF_UNSPEC: Returns both IPv4 and IPv6 MIB entries.

    Table - Returns the table of interfaces in a MIB_IPINTERFACE_TABLE
        structure.  Use FreeMibTable to free this buffer.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    The API allocates the buffer for Table.  Use FreeMibTable to free it.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
VOID
NETIOAPI_API_
InitializeIpInterfaceEntry(
    _Inout_ PMIB_IPINTERFACE_ROW Row
    );
/*++

Routine Description:

    Initialize the MIB_IPINTERFACE_ROW entry for use in SetIpInterfaceRow.

Arguments:

    Row - Returns an initialized MIB_IPINTERFACE_ROW structure.

Return Value:

    None.

Notes:

    InitializeIpInterfaceEntry must be used to initialize the fields of
    MIB_IPINTERFACE_ROW with default values.  The caller can then update the
    fields it wishes to modify and invoke SetIpInterfaceEntry.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
NotifyIpInterfaceChange(
    _In_ ADDRESS_FAMILY Family,
    _In_ PIPINTERFACE_CHANGE_CALLBACK Callback,
    _In_opt_ PVOID CallerContext,
    _In_ BOOLEAN InitialNotification,
    _Inout_ HANDLE *NotificationHandle
    );
/*++

Routine Description:

    Register for notification for IP interface changes.

Arguments:

    Family - Supplies the address family.

        AF_INET: Only register for IPv4 change notifications.

        AF_INET6: Only register for IPv6 change notifications.

        AF_UNSPEC: Register for both IPv4 and IPv6 change notifications.

    Callback - Supplies a callback function.  This function will be invoked
        when an interface notification is received.

    CallerContext - Provides the user specific caller context.  This context
        will be supplied to the callback function.

    InitialNotification - Supplies a boolean to indicate whether an
        initialization notification should be provided.

    NotificationHandle - Returns a handle to the notification registration.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    1. Invokation of the callback function is serialized.

    2. Use CancelMibChangeNotify2 to deregister for change notifications.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
SetIpInterfaceEntry(
    _Inout_ PMIB_IPINTERFACE_ROW Row
    );
/*++

Routine Description:

    Set the properties of an IP interface.

Arguments:

    Row - Supplies a MIB_IPINTERFACE_ROW structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    InitializeIpInterfaceEntry must be used to initialize the fields of
    MIB_IPINTERFACE_ROW with default values.  The caller can then update the
    fields it wishes to modify and invoke SetIpInterfaceEntry.

    On input, the following key fields of Row must be initialized after
    invoking InitializeIpInterfaceEntry:
    1. Family: To AF_INET or AF_INET6.
    2. At least one of InterfaceLuid or InterfaceIndex must be specified.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetIpNetworkConnectionBandwidthEstimates(
    _In_ NET_IFINDEX InterfaceIndex,
    _In_ ADDRESS_FAMILY AddressFamily,
    _Out_ PMIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES BandwidthEstimates
    );
/*++

Routine Description:

    Retrieves historical bandwidth estimates for the network connection of
    the specified interface.

Arguments:

    InterfaceIndex - Supplies the interface index.

    AddressFamily - Supplies the address family associated with the interface.

    BandwidthEstimates - Returns the historical bandwidth estimates.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

//
// Unicast address management routines.
//

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// The structure for unicast IP Address management.
//
typedef struct _MIB_UNICASTIPADDRESS_ROW {
    //
    // Key Structure.
    //
    SOCKADDR_INET Address;
    NET_LUID InterfaceLuid;
    NET_IFINDEX InterfaceIndex;

    //
    // Read-Write Fileds.
    //
    NL_PREFIX_ORIGIN PrefixOrigin;
    NL_SUFFIX_ORIGIN SuffixOrigin;
    ULONG ValidLifetime;
    ULONG PreferredLifetime;
    UINT8 OnLinkPrefixLength;
    BOOLEAN SkipAsSource;

    //
    // Read-Only Fields.
    //
    NL_DAD_STATE DadState;
    SCOPE_ID ScopeId;
    LARGE_INTEGER CreationTimeStamp;
} MIB_UNICASTIPADDRESS_ROW, *PMIB_UNICASTIPADDRESS_ROW;

typedef struct _MIB_UNICASTIPADDRESS_TABLE {
    ULONG NumEntries;
    MIB_UNICASTIPADDRESS_ROW Table[ANY_SIZE];
} MIB_UNICASTIPADDRESS_TABLE, *PMIB_UNICASTIPADDRESS_TABLE;

typedef
VOID
(NETIOAPI_API_ *PUNICAST_IPADDRESS_CHANGE_CALLBACK) (
    _In_ PVOID CallerContext,
    _In_opt_ PMIB_UNICASTIPADDRESS_ROW Row,
    _In_ MIB_NOTIFICATION_TYPE NotificationType
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
CreateUnicastIpAddressEntry(
    _In_ CONST MIB_UNICASTIPADDRESS_ROW *Row
    );
/*++

Routine Description:

    Create a unicast IP address entry on the local computer.

Arguments:

    Row - Supplies a MIB_UNICASTIPADDRESS_ROW structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    InitializeUnicastIpAddressEntry must be used to initialize the fields of
    MIB_UNICASTIPADDRESS_ROW with default values.  The caller can then update
    the fields it wishes to modify and invoke CreateIpInterfaceEntry.

    On input, the following key fields of Row must be initialized after
    invoking InitializeUnicastIpAddressEntry:
    1. Address to a valid IPv4 or IPv6 unicast address.
    2. At least one of InterfaceLuid or InterfaceIndex must be specified.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
DeleteUnicastIpAddressEntry(
    _In_ CONST MIB_UNICASTIPADDRESS_ROW *Row
    );
/*++

Routine Description:

    Delete a unicast IP address entry on the local computer.

Arguments:

    Row - Supplies a MIB_UNICASTIPADDRESS_ROW structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1. Address to a valid IPv4 or IPv6 unicast address.
    2. At least one of InterfaceLuid or InterfaceIndex must be specified.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetUnicastIpAddressEntry(
    _Inout_ PMIB_UNICASTIPADDRESS_ROW Row
    );
/*++

Routine Description:

    Retrieves information for the specified unicast IP address entry on the
        local computer.

Arguments:

    Address - Supplies a MIB_UNICASTIPADDRESS_ROW structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1. Address to a valid IPv4 or IPv6 unicast address.
    2. At least one of InterfaceLuid or InterfaceIndex must be specified.

    On output, the remaining fields of Row are filled in.

--*/

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetUnicastIpAddressTable(
    _In_ ADDRESS_FAMILY Family,
    _Outptr_ PMIB_UNICASTIPADDRESS_TABLE *Table
    );
/*++

Routine Description:

    Retrieves the unicast IP address table on a local computer.

Arguments:

    Family - Supplies the address family.

        AF_INET: Only returns IPv4 unicast addresses.

        AF_INET6: Only returns IPv6 unicast addresses.

        AF_UNSPEC: Returns both IPv4 and IPv6 unicast addresses.

    Table - Returns the table of unicast IP addresses in a
        MIB_UNICASTIPADDRESS_TABLE Structure.  Use FreeMibTable to free this
        buffer.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    The API allocates the buffer for Table.  Use FreeMibTable to free it.

--*/

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
VOID
NETIOAPI_API_
InitializeUnicastIpAddressEntry(
    _Out_ PMIB_UNICASTIPADDRESS_ROW Row
    );
/*++

Routine Description:

    Initialize the MIB_UNICASTIPADDRESS_ROW entry for use in
    CreateUnicastIpAddressEntry and SetUnicastIpAddressEntry.

Arguments:

    Address - Returns an initialized MIB_UNICASTIPADDRESS_ROW structure.

Return Value:

    None.

Notes:

    InitializeUnicastIpAddressEntry must be used to initialize the fields of
    MIB_UNICASTIPADDRESS_ROW with default values.  The caller can then update
    the fields it wishes to modify and invoke CreateUnicastIpAddressEntry or
    SetUnicastIpAddressEntry.

--*/

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
NotifyUnicastIpAddressChange(
    _In_ ADDRESS_FAMILY Family,
    _In_ PUNICAST_IPADDRESS_CHANGE_CALLBACK Callback,
    _In_opt_ PVOID CallerContext,
    _In_ BOOLEAN InitialNotification,
    _Inout_ HANDLE *NotificationHandle
    );
/*++

Routine Description:

    Register for notification for unicast IP address changes.

Arguments:

    Family - Supplies the address family.

        AF_INET: Only register for IPv4 change notifications.

        AF_INET6: Only register for IPv6 change notifications.

        AF_UNSPEC: Register for both IPv4 and IPv6 change notifications.

    Callback - Supplies a callback function.  This function will be invoked
        when an unicast IP address notification is received.

    CallerContext - Provides the user specific caller context.  This context
        will be supplied to the callback function.

    InitialNotification - Supplies a boolean to indicate whether an
        initialization notification should be provided.

    NotificationHandle - Returns a handle to the notification registration.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    1. Invokation of the callback function is serialized.

    2. Use CancelMibChangeNotify2 to deregister for change notifications.

--*/

typedef
VOID
(NETIOAPI_API_ *PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK) (
    _In_ PVOID CallerContext,
    _In_ PMIB_UNICASTIPADDRESS_TABLE AddressTable
    );

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
NotifyStableUnicastIpAddressTable(
    _In_ ADDRESS_FAMILY Family,
    _Outptr_ PMIB_UNICASTIPADDRESS_TABLE* Table,
    _In_ PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK CallerCallback,
    _In_ PVOID CallerContext,
    _Inout_ HANDLE *NotificationHandle
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
SetUnicastIpAddressEntry(
    _In_ CONST MIB_UNICASTIPADDRESS_ROW *Row
    );
/*++

Routine Description:

    Set the properties of an unicast IP address.

Arguments:

    Address - Supplies a MIB_UNICASTIPADDRESS_ROW structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    InitializeUnicastIpAddressEntry must be used to initialize the fields of
    MIB_UNICASTIPADDRESS_ROW with default values.  The caller can then update
    the fields it wishes to modify and invoke SetUnicastIpAddressEntry.

    On input, the following key fields of Row must be initialized after
    invoking InitializeUnicastIpAddressEntry:
    1. Address to a valid IPv4 or IPv6 unicast address.
    2. At least one of InterfaceLuid or InterfaceIndex must be specified.

--*/

//
// Anycast address management routines.
//

typedef struct _MIB_ANYCASTIPADDRESS_ROW {
    //
    // Key Structure.
    //
    SOCKADDR_INET Address;
    NET_LUID InterfaceLuid;
    NET_IFINDEX InterfaceIndex;

    //
    // Read-Only Fields.
    //
    SCOPE_ID ScopeId;
} MIB_ANYCASTIPADDRESS_ROW, *PMIB_ANYCASTIPADDRESS_ROW;

typedef struct _MIB_ANYCASTIPADDRESS_TABLE {
    ULONG NumEntries;
    MIB_ANYCASTIPADDRESS_ROW Table[ANY_SIZE];
} MIB_ANYCASTIPADDRESS_TABLE, *PMIB_ANYCASTIPADDRESS_TABLE;

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
CreateAnycastIpAddressEntry(
    _In_ CONST MIB_ANYCASTIPADDRESS_ROW *Row
    );
/*++

Routine Description:

    Create an anycast IP address entry on the local computer.

Arguments:

    Address - Supplies a MIB_ANYCASTIPADDRESS_ROW structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1. Address to a valid IPv4 or IPv6 anycast address.
    2. At least one of InterfaceLuid or InterfaceIndex must be specified.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
DeleteAnycastIpAddressEntry(
    _In_ CONST MIB_ANYCASTIPADDRESS_ROW *Row
    );
/*++

Routine Description:

    Delete an anycast IP address entry on the local computer.

Arguments:

    Address - Supplies a MIB_ANYCASTIPADDRESS_ROW structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1. Address to a valid IPv4 or IPv6 anycast address.
    2. At least one of InterfaceLuid or InterfaceIndex must be specified.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetAnycastIpAddressEntry(
    _Inout_ PMIB_ANYCASTIPADDRESS_ROW Row
    );
/*++

Routine Description:

    Retrieves information for the specified anycast IP address entry on the
    local computer.

Arguments:

    Address - Supplies a MIB_ANYCASTIPADDRESS_ROW structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1. Address to a valid IPv4 or IPv6 anycast address.
    2. At least one of InterfaceLuid or InterfaceIndex must be specified.

    On output, the remaining fields of Row are filled in.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetAnycastIpAddressTable(
    _In_ ADDRESS_FAMILY Family,
    _Outptr_ PMIB_ANYCASTIPADDRESS_TABLE *Table
    );
/*++

Routine Description:

    Retrieves the anycast IP address table.

Arguments:

    Family - Supplies the address family.

        AF_INET: Only returns IPv4 anycast addresses.

        AF_INET6: Only returns IPv6 anycast addresses.

        AF_UNSPEC: Returns both IPv4 and IPv6 anycast addresses.

    Table - Returns the table of anycast IP addresses in a
        MIB_ANYCASTIPADDRESS_TABLE Structure.  Use FreeMibTable to free this
        buffer.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    The API allocates the buffer for Table.  Use FreeMibTable to free it.

--*/


//
// Multicast address management routines.
//
typedef struct _MIB_MULTICASTIPADDRESS_ROW {
    //
    // Key Structure.
    //
    SOCKADDR_INET Address;
    NET_IFINDEX InterfaceIndex;
    NET_LUID InterfaceLuid;

    //
    // Read-Only Fields.
    //
    SCOPE_ID ScopeId;
} MIB_MULTICASTIPADDRESS_ROW, *PMIB_MULTICASTIPADDRESS_ROW;

typedef struct _MIB_MULTICASTIPADDRESS_TABLE {
    ULONG NumEntries;
    MIB_MULTICASTIPADDRESS_ROW Table[ANY_SIZE];
} MIB_MULTICASTIPADDRESS_TABLE, *PMIB_MULTICASTIPADDRESS_TABLE;

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetMulticastIpAddressEntry(
    _Inout_ PMIB_MULTICASTIPADDRESS_ROW Row
    );
/*++

Routine Description:

    Retrieves information for the specified mulitcast IP address entry on the
    local computer.

Arguments:

    Row - Supplies a MIB_MULTICASTIPADDRESS_ROW structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized.
    1. Address to a valid IPv4 or IPv6 multicast address.
    2. At least one of InterfaceLuid or InterfaceIndex must be specified.

    On output, the remaining fields of Row are filled in.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetMulticastIpAddressTable(
    _In_ ADDRESS_FAMILY Family,
    _Outptr_ PMIB_MULTICASTIPADDRESS_TABLE *Table
    );
/*++

Routine Description:

    Retrieves the multicast IP address table on the local computer.

Arguments:

    Family - Supplies the address family.

        AF_INET: Only returns IPv4 multicast addresses.

        AF_INET6: Only returns IPv6 multicast addresses.

        AF_UNSPEC: Returns both IPv4 and IPv6 multicast addresses.

    Table - Returns the table of multicast IP addresses in a
        MIB_MULTICASTIPADDRESS_TABLE Structure.  Use FreeMibTable to free this
        buffer.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    The API allocates the buffer for Table.  Use FreeMibTable to free it.

--*/

//
// Route management routines.
//

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

typedef struct _IP_ADDRESS_PREFIX {
    SOCKADDR_INET Prefix;
    UINT8 PrefixLength;
} IP_ADDRESS_PREFIX, *PIP_ADDRESS_PREFIX;

typedef struct _MIB_IPFORWARD_ROW2 {
    //
    // Key Structure.
    //
    NET_LUID InterfaceLuid;
    NET_IFINDEX InterfaceIndex;
    IP_ADDRESS_PREFIX DestinationPrefix;
    SOCKADDR_INET NextHop;

    //
    // Read-Write Fields.
    //
    UCHAR SitePrefixLength;
    ULONG ValidLifetime;
    ULONG PreferredLifetime;
    ULONG Metric;
    NL_ROUTE_PROTOCOL Protocol;

    BOOLEAN Loopback;
    BOOLEAN AutoconfigureAddress;
    BOOLEAN Publish;
    BOOLEAN Immortal;

    //
    // Read-Only Fields.
    //
    ULONG Age;
    NL_ROUTE_ORIGIN Origin;
} MIB_IPFORWARD_ROW2, *PMIB_IPFORWARD_ROW2;

typedef struct _MIB_IPFORWARD_TABLE2 {
    ULONG NumEntries;
    MIB_IPFORWARD_ROW2 Table[ANY_SIZE];
} MIB_IPFORWARD_TABLE2, *PMIB_IPFORWARD_TABLE2;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

typedef
VOID
(NETIOAPI_API_ *PIPFORWARD_CHANGE_CALLBACK) (
    _In_ PVOID CallerContext,
    _In_opt_ PMIB_IPFORWARD_ROW2 Row,
    _In_ MIB_NOTIFICATION_TYPE NotificationType
    );

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
CreateIpForwardEntry2(
    _In_ CONST MIB_IPFORWARD_ROW2 *Row
    );
/*++

Routine Description:

    Create a route on the local computer.

Arguments:

    Row - Supplies a MIB_IPFORWARD_ROW2 structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    InitializeIpForwardEntry must be used to initialize the fields of
    MIB_IPFORWARD_ROW2 with default values.  The caller can then update the
    fields it wishes to modify and invoke CreateIpForwardEntry2.

    On input, the following key fields of Row must be initialized after
    invoking InitializeIpForwardEntry:
    1. At least one of InterfaceLuid or InterfaceIndex must be specified.
    2. DestinationPrefix.
    3. NextHop.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
DeleteIpForwardEntry2(
    _In_ CONST MIB_IPFORWARD_ROW2 *Row
    );
/*++

Routine Description:

    Delete a route on the local computer.

Arguments:

    Row - Supplies a MIB_IPFORWARD_ROW2 structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1. At least one of InterfaceLuid or InterfaceIndex must be specified.
    2. DestinationPrefix.
    3. NextHop.

--*/

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetBestRoute2(
    _In_opt_ NET_LUID *InterfaceLuid,
    _In_ NET_IFINDEX InterfaceIndex,
    _In_opt_ CONST SOCKADDR_INET *SourceAddress,
    _In_ CONST SOCKADDR_INET *DestinationAddress,
    _In_ ULONG AddressSortOptions,
    _Out_ PMIB_IPFORWARD_ROW2 BestRoute,
    _Out_ SOCKADDR_INET *BestSourceAddress
    );
/*++

Routine Description:

    Retrieve the best route between source and destination address on a local
        computer.

Arguments:

    InterfaceLuid - Supplies Luid to specify an interface.

    InterfaceIndex - Supplies Index to specify an interface.

    SourceAddress - Supplies source address.

    DestinationAddress - Supplies destination address.

    AddressSortOptions - Supplies AddressSortOptions.

    BestRoute - Returns the MIB structure that holds the best route.

    BestSourceAddress - Returns the source address of the best route.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following parameters must be supplied:
    1. At least one of InterfaceLuid or InterfaceIndex must be specified.
    2. SourceAddress.
    3. DestinationAddress.

--*/

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetIpForwardEntry2(
    _Inout_ PMIB_IPFORWARD_ROW2 Row
    );
/*++

Routine Description:

    Retrieves information for the specified route entry on the local computer.

Arguments:

    Route - Supplies a MIB_IPFORWARD_ROW2 structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1. At least one of InterfaceLuid or InterfaceIndex must be specified.
    2. DestinationPrefix and NextHop can be specified.

    On output, the remaining fields of Row are filled in.

    If one or more routes matches the specified criteria,
    this API matches the first entry.

--*/

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetIpForwardTable2(
    _In_ ADDRESS_FAMILY Family,
    _Outptr_ PMIB_IPFORWARD_TABLE2 *Table
    );
/*++

Routine Description:

    Retrieves the route table on a local computer.

Arguments:

    Family - Supplies the address family.

        AF_INET: Only returns IPv4 route entries.

        AF_INET6: Only returns IPv6 route entries.

        AF_UNSPEC: Returns both IPv4 and IPv6 route entries.

    Table - Returns the table of routes in a MIB_IPFORWARD_TABLE2 Structure.
        Use FreeMibTable to free this buffer.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    The API allocates the buffer for Table.  Use FreeMibTable to free it.

--*/

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
VOID
NETIOAPI_API_
InitializeIpForwardEntry(
    _Out_ PMIB_IPFORWARD_ROW2 Row
    );
/*++

Routine Description:

    Initialize the MIB_IPFORWARD_ROW2 entry for use in SetIpForwardEntry2.

Arguments:

    Row - Returns an initialized PMIB_IPFORWARD_ROW2 structure.

Return Value:

    None.

Notes:

    InitializeIpForwardEntry must be used to initialize the fields of
        MIB_IPFORWARD_ROW2 with default values.  The caller can then update the
        fields it wishes to modify and invoke SetIpForwardEntry2.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
NotifyRouteChange2(
    _In_ ADDRESS_FAMILY AddressFamily,
    _In_ PIPFORWARD_CHANGE_CALLBACK Callback,
    _In_ PVOID CallerContext,
    _In_ BOOLEAN InitialNotification,
    _Inout_ HANDLE *NotificationHandle
    );
/*++

Routine Description:

    Register for notification for route changes.

Arguments:

    Family - Supplies the address family.

        AF_INET: Only register for IPv4 route change notifications.

        AF_INET6: Only register for IPv6 route change notifications.

        AF_UNSPEC: Register for both IPv4 and IPv6 route change notifications.

    Callback - Supplies a callback function. This function will be invoked when
        an unicast IP address notification is received.

    CallerContext - Provides the user specific caller context. This context
        will be supplied to the callback function.

    InitialNotification - Supplies a boolean to indicate whether an
        initialization notification should be provided.

    NotificationHandle - Returns a handle to the notification registration.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    1. Invokation of the callback function is serialized.

    2. Use CancelMibChangeNotify2 to deregister for change notifications.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
SetIpForwardEntry2(
    _In_ CONST MIB_IPFORWARD_ROW2 *Route
    );
/*++

Routine Description:

    Set the properties of a route entry.

Arguments:

    Route - Supplies a MIB_UNICASTIPADDRESS_ROW structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    InitializeIpForwardEntry must be used to initialize the fields of
    MIB_IPFORWARD_ROW2 with default values.  The caller can then update the
    fields it wishes to modify and invoke SetIpForwardEntry2.

    On input, the following key fields of Row must be initialized after
    invoking InitializeIpForwardEntry:
    1. At least one of InterfaceLuid or InterfaceIndex must be specified.
    2. DestinationPrefix.
    3. NextHop.

--*/

//
// Path management routines.
//

typedef struct _MIB_IPPATH_ROW {
    //
    // Key.
    //

    SOCKADDR_INET Source;
    SOCKADDR_INET Destination;
    NET_LUID InterfaceLuid;
    NET_IFINDEX InterfaceIndex;

    //
    // RO.
    //
    //
    // The current next hop.  This can change over the lifetime of a path.
    //
    SOCKADDR_INET CurrentNextHop;

    //
    // MTU of path to destination. Includes the IP header length.
    //
    ULONG PathMtu;

    //
    // Estimated mean RTT.
    //
    ULONG RttMean;

    //
    // Mean deviation of RTT.
    //
    ULONG RttDeviation;
    union {
        ULONG LastReachable;    // Milliseconds.
        ULONG LastUnreachable;  // Milliseconds.
    };
    BOOLEAN IsReachable;

    //
    // Estimated speed.
    //
    ULONG64 LinkTransmitSpeed;
    ULONG64 LinkReceiveSpeed;

} MIB_IPPATH_ROW, *PMIB_IPPATH_ROW;

typedef struct _MIB_IPPATH_TABLE {
    ULONG NumEntries;
    MIB_IPPATH_ROW Table[ANY_SIZE];
} MIB_IPPATH_TABLE, *PMIB_IPPATH_TABLE;


_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
FlushIpPathTable(
    _In_ ADDRESS_FAMILY Family
    );
/*++

Routine Description:

    Flush the IP Path table on the local computer.

Arguments:

    Family - Supplies the address family.

        AF_INET: Only flush the IPv4 path table.

        AF_INET6: Only flush the IPv6 path table.

        AF_UNSPEC: Flush both IPv4 and IPv6 path table.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetIpPathEntry(
    _Inout_ PMIB_IPPATH_ROW Row
    );
/*++

Routine Description:

    Retrieves information for the specified path entry on the local computer.

Arguments:

    Row - Supplies a MIB_IPPATH_ROW structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1. At least one of InterfaceLuid or InterfaceIndex must be specified.
    2. Source.
    3. Destination.

    On output, the remaining fields of Row are filled in.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetIpPathTable(
    _In_ ADDRESS_FAMILY Family,
    _Outptr_ PMIB_IPPATH_TABLE *Table
    );
/*++

Routine Description:

    Retrieves the path table on a local computer.

Arguments:

    Family - Supplies the address family.

        AF_INET: Only returns IPv4 paths.

        AF_INET6: Only returns IPv6 paths.

        AF_UNSPEC: Returns both IPv4 and IPv6 paths.

    Table - Returns the table of paths in a MIB_IPPATH_TABLE
        structure.  Use FreeMibTable to free this buffer.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    The API allocates the buffer for Table.  Use FreeMibTable to free it.

--*/

//
// ARP and IPv6 Neighbor management routines.
//

typedef struct _MIB_IPNET_ROW2 {
    //
    // Key Struture.
    //
    SOCKADDR_INET Address;
    NET_IFINDEX InterfaceIndex;
    NET_LUID InterfaceLuid;

    //
    // Read-Write.
    //
    UCHAR PhysicalAddress[IF_MAX_PHYS_ADDRESS_LENGTH];

    //
    // Read-Only.
    //
    ULONG PhysicalAddressLength;
    NL_NEIGHBOR_STATE State;

    union {
        struct {
            BOOLEAN IsRouter : 1;
            BOOLEAN IsUnreachable : 1;
        };
        UCHAR Flags;
    };

    union {
        ULONG LastReachable;
        ULONG LastUnreachable;
    } ReachabilityTime;
} MIB_IPNET_ROW2, *PMIB_IPNET_ROW2;

typedef struct _MIB_IPNET_TABLE2 {
    ULONG NumEntries;
    MIB_IPNET_ROW2 Table[ANY_SIZE];
} MIB_IPNET_TABLE2, *PMIB_IPNET_TABLE2;

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
CreateIpNetEntry2(
    _In_ CONST MIB_IPNET_ROW2 *Row
    );
/*++

Routine Description:

    Create a neighbor entry on the local computer.

Arguments:

    Row - Supplies a MIB_IPNET_ROW2 structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1. At least one of InterfaceLuid or InterfaceIndex must be specified.
    2. Address.
    3. PhysicalAddress.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
DeleteIpNetEntry2(
    _In_ CONST MIB_IPNET_ROW2 *Row
    );
/*++

Routine Description:

    Delete a neighbor entry on the local computer.

Arguments:

    Row - Supplies a MIB_IPNET_ROW2 structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1. At least one of InterfaceLuid or InterfaceIndex must be specified.
    2. Address.

--*/


_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
FlushIpNetTable2(
    _In_ ADDRESS_FAMILY Family,
    _In_ NET_IFINDEX InterfaceIndex
    );
/*++

Routine Description:

    Flush the neighbor entry table on the local computer.

Arguments:

    Family - Supplies the address family.

        AF_INET: Only flush the IPv4 neighbor table.

        AF_INET6: Only flush the IPv6 neighbor table.

        AF_UNSPEC: Flush both IPv4 and IPv6 neighbor table.

    InterfaceIndex - Supplies the Interface index.  If the index is specified,
        flush the neighbor entries on a specific interface, otherwise flush the
        neighbor entries on all the interfaces.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/


_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetIpNetEntry2(
    _Inout_ PMIB_IPNET_ROW2 Row
    );
/*++

Routine Description:

    Retrieves information for the specified neighbor entry on the local
    computer.

Arguments:

    Row - Supplies a MIB_IPNET_ROW2 structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1. At least one of InterfaceLuid or InterfaceIndex must be specified.
    2. Address.

    On output, the remaining fields of Row are filled in.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetIpNetTable2(
    _In_ ADDRESS_FAMILY Family,
    _Outptr_ PMIB_IPNET_TABLE2 *Table
    );
/*++

Routine Description:

    Retrieves the neighbor table on the local computer.

Arguments:

    Family - Supplies the address family.

        AF_INET: Only returns IPv4 neighbor entries.

        AF_INET6: Only returns IPv6 neighbor entries.

        AF_UNSPEC: Returns both IPv4 and IPv6 neighbor entries.

    Table - Returns the table of neighbor entries in a MIB_IPNET_TABLE2
        structure.  Use FreeMibTable to free this buffer.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    The API allocates the buffer for Table.  Use FreeMibTable to free it.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
ResolveIpNetEntry2(
    _Inout_ PMIB_IPNET_ROW2 Row,
    _In_opt_ CONST SOCKADDR_INET *SourceAddress
    );
/*++

Routine Description:

    Resolve the physical address of a specific neighbor.

Arguments:

    NetEntry - Supplies a MIB_IPNET_ROW2 structure.

    SourceAddress - Supplies the source address.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    This API flushes any existing neighbor entry and resolves the MAC address
    by sending ARP requests (IPv4) or Neighbor Solicitation (IPv6).
    If source address is not provided, the API will automatically select the
    best interface to send the request on.

    On input, the following key fields of Row must be initialized:
    1. At least one of InterfaceLuid or InterfaceIndex must be specified.
    2. Address.

    On output, the remaining fields of Row are filled in.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
SetIpNetEntry2(
    _In_ PMIB_IPNET_ROW2 Row
    );
/*++

Routine Description:

    Set the physical address of a neighbor entry.

Arguments:

    NetEntry - Supplies a MIB_IPNET_ROW2 structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1. At least one of InterfaceLuid or InterfaceIndex must be specified.
    2. Address.
    3. PhysicalAddress.

--*/

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

//
// Teredo APIs.
//
#define MIB_INVALID_TEREDO_PORT_NUMBER 0

typedef
VOID
(NETIOAPI_API_ *PTEREDO_PORT_CHANGE_CALLBACK) (
    _In_ PVOID CallerContext,
    _In_ USHORT Port,
    _Inout_ MIB_NOTIFICATION_TYPE NotificationType
    );

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
NotifyTeredoPortChange(
    _In_ PTEREDO_PORT_CHANGE_CALLBACK Callback,
    _In_ PVOID CallerContext,
    _In_ BOOLEAN InitialNotification,
    _Inout_ HANDLE *NotificationHandle
    );

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetTeredoPort(
    _Out_ USHORT *Port
    );
/*++

Routine Description:

    Get the Teredo client port.

Arguments:

    Port - returns the Teredo port.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

#ifndef TEREDO_API_NO_DEPRECATE

//
// Deprecate all of the unsafe functions to generate compiletime errors.
// Use the following routines instead : FwpmSystemPortsGet
//

#pragma deprecated(NotifyTeredoPortChange)
#pragma deprecated(GetTeredoPort)

#endif // TEREDO_API_NO_DEPRECATE

//
// Generic (not IP-specific) interface definitions.
//

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
NETIOAPI_API
CancelMibChangeNotify2(
    _In_ HANDLE NotificationHandle
    );
/*++

Routine Description:

    Deregister for change notifications.

Arguments:

    NotificationHandle - Supplies the handle returned from a notification
        registration.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    Blocks until all callback have returned.

--*/

IPHLPAPI_DLL_LINKAGE
VOID
NETIOAPI_API_
FreeMibTable(
    _In_ PVOID Memory
    );
/*++

Routine Description:

    Free the buffer allocated by Get*Table APIs.

Arguments:

    Memory - Supplies the buffer to free.

Return Value:

    None.

--*/

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
CreateSortedAddressPairs(
    _In_opt_ const PSOCKADDR_IN6 SourceAddressList,
    _In_ ULONG SourceAddressCount,
    _In_ const PSOCKADDR_IN6 DestinationAddressList,
    _In_ ULONG DestinationAddressCount,
    _In_ ULONG AddressSortOptions,
    _Outptr_result_buffer_(*SortedAddressPairCount)
    PSOCKADDR_IN6_PAIR *SortedAddressPairList,
    _Out_ ULONG *SortedAddressPairCount
    );
/*++

Routine Description:

    Given a list of source and destination addresses, returns a list of
    pairs of addresses in sorted order.  The list is sorted by which address
    pair is best suited for communication between two peers.

    The list of source addresses is optional, in which case the function
    automatically uses all the host machine's local addresses.

Arguments:

    SourceAddressList - Supplies list of potential source addresses.
        If NULL the routine automatically uses all local addresses.
        IPv4 addresses can be specified in IPv4-mapped format.
        Reserved for future use.  Must be NULL.

    SourceAddressCount - Supplies the number of addresses in the
        SourceAddressList.
        Reserved for future use.  Must be 0.

    DestinationAddressList - Supplies list of potential destination addresses.
        IPv4 addresses can be specified in IPv4-mapped format.

    DestinationAddressCount -  Supplies the number of addresses in the
        DestinationAddressList.

    AddressSortOptions - Reserved for future use.  Must be 0.

    SortedAddressPairList - Returns a sorted list of pairs of addresses
        in prefered order of communication.  The list must be freed with a
        single call to NetioFreeMemory.

    SortedAddressPairCount - Returns the number of address pairs in
        SortedAddressPairList.

Return Value:

    ERROR_SUCCESS on success.  WIN32 error code on error.

--*/

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif //_WS2IPDEF_

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
ConvertCompartmentGuidToId(
    _In_ CONST GUID *CompartmentGuid,
    _Out_ PNET_IF_COMPARTMENT_ID CompartmentId
    );
/*++

Routine Description:

    Converts a compartment GUID to a compartment ID.

Arguments:

    CompartmentGuid - Supplies the compartment GUID.

    CompartmentId - Returns the compartment ID.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
ConvertCompartmentIdToGuid(
    _In_ NET_IF_COMPARTMENT_ID CompartmentId,
    _Out_ GUID *CompartmentGuid
    );
/*++

Routine Description:

    Converts a compartment ID to a compartment GUID.

Arguments:

    CompartmentId - Supplies the compartment ID.

    CompartmentGuid - Returns the compartment GUID.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
ConvertInterfaceNameToLuidA(
    _In_z_ CONST CHAR *InterfaceName,
    _Out_ NET_LUID *InterfaceLuid
    );
/*++

Routine Description:

    Convert an Interface Name to Luid.

Arguments:

    InterfaceName - Supplies the interface name to be converted.

    InterfaceLuid - Returns the interface Luid.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
ConvertInterfaceNameToLuidW(
    _In_z_ CONST WCHAR *InterfaceName,
    _Out_ NET_LUID *InterfaceLuid
    );
/*++

Routine Description:

    Convert an Interface Name to Luid.

Arguments:

    InterfaceName - Supplies the interface name to be converted.

    InterfaceLuid - Returns the interface Luid.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
ConvertInterfaceLuidToNameA(
    _In_ CONST NET_LUID *InterfaceLuid,
    _Out_writes_(Length) PSTR InterfaceName,
    _In_ SIZE_T Length
    );
/*++

Routine Description:

    Convert an Interface Luid to Name.

Arguments:

    InterfaceLuid - Supplies the interface Luid to be converted.

    InterfaceName - Returns the interface name.

    Lenght - Supplies the length of the InterfaceName buffer.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
ConvertInterfaceLuidToNameW(
    _In_ CONST NET_LUID *InterfaceLuid,
    _Out_writes_(Length) PWSTR InterfaceName,
    _In_ SIZE_T Length
    );
/*++

Routine Description:

    Convert an Interface Luid to Name.

Arguments:

    InterfaceLuid - Supplies the interface Luid to be converted.

    InterfaceName - Returns the interface name.

    Length - Supplies the length of the InterfaceName buffer.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
ConvertInterfaceLuidToIndex(
    _In_ CONST NET_LUID *InterfaceLuid,
    _Out_ PNET_IFINDEX InterfaceIndex
    );
/*++

Routine Description:

    Convert an Interface Luid to Index.

Arguments:

    InterfaceLuid - Supplies the interface Luid to be converted.

    InterfaceName - Returns the interface Index.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
ConvertInterfaceIndexToLuid(
    _In_ NET_IFINDEX InterfaceIndex,
    _Out_ PNET_LUID InterfaceLuid
    );
/*++

Routine Description:

    Convert an Interface Index to Luid.

Arguments:

    InterfaceName - Supplies the interface Index to be converted.

    InterfaceLuid - Returns the interface Luid.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
ConvertInterfaceLuidToAlias(
    _In_ CONST NET_LUID *InterfaceLuid,
    _Out_writes_(Length) PWSTR InterfaceAlias,
    _In_ SIZE_T Length
    );
/*++

Routine Description:

    Convert an Interface Luid to Alias.

Arguments:

    InterfaceLuid - Supplies the interface Luid to be converted.

    InterfaceAlias - Returns the interface Alias.

    Length - Supplies the length of InterfaceAlias buffer.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
ConvertInterfaceAliasToLuid(
    _In_z_ CONST WCHAR *InterfaceAlias,
    _Out_ PNET_LUID InterfaceLuid
    );
/*++

Routine Description:

    Convert an Interface Alias to Luid.

Arguments:

    InterfaceAlias - Supplies the null terminated interface Alias.

    InterfaceLuid - Returns the interface Luid.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
ConvertInterfaceLuidToGuid(
    _In_ CONST NET_LUID *InterfaceLuid,
    _Out_ GUID *InterfaceGuid
    );
/*++

Routine Description:

    Convert an Interface Luid to Guid.

Arguments:

    InterfaceLuid - Supplies the interface Luid to be converted.

    InterfaceGuid - Returns the interface Guid.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
ConvertInterfaceGuidToLuid(
    _In_ CONST GUID *InterfaceGuid,
    _Out_ PNET_LUID InterfaceLuid
    );
/*++

Routine Description:

    Convert an Interface Luid to Guid.

Arguments:

    InterfaceGuid - Supplies the interface Guid to be converted.

    InterfaceGuid - Returns the interface Luid.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

#define IF_NAMESIZE NDIS_IF_MAX_STRING_SIZE

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
NET_IFINDEX
NETIOAPI_API_
if_nametoindex(
    _In_ PCSTR InterfaceName
    );
/*++

Routine Description:

    Convert an Interface name to Index.

Arguments:

    InterfaceName - Supplies the null terminated interface name to convert.

Return Value:

    Interface index on success, 0 otherwise.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
PCHAR
NETIOAPI_API_
if_indextoname(
    _In_ NET_IFINDEX InterfaceIndex,
    _Out_writes_(IF_NAMESIZE) PCHAR InterfaceName
    );
/*++

Routine Description:

    Convert an Interface index to Name.

Arguments:

    InterfaceIndex - Supplies the Interface index to convert.

    InterfaceName - Returns the null terminated interface name.

Return Value:

    Interface name on success, NULL otherwise.

Notes:

    The length of InterfaceName buffer must be equal to or greater than
        IF_NAMESIZE.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
NET_IF_COMPARTMENT_ID
NETIOAPI_API_
GetCurrentThreadCompartmentId(
    VOID
    );
/*++

Routine Description:

    Get the compartment ID of current thread.

Arguments:

    None.

Return Value:

    The compartment ID of current thread.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
SetCurrentThreadCompartmentId(
    _In_ NET_IF_COMPARTMENT_ID CompartmentId
    );
/*++

Routine Description:

    Set the compartment ID of current thread.

Arguments:

    CompartmentId - Supplies the compartment ID to be set.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
VOID
NETIOAPI_API_
GetCurrentThreadCompartmentScope(
    _Out_ PNET_IF_COMPARTMENT_SCOPE CompartmentScope,
    _Out_ PNET_IF_COMPARTMENT_ID CompartmentId
    );
/*++

Routine Description:

    Returns the compartment scope and ID for the current thread.

Arguments:

    CompartmentScope - Receives the compartment scope.

    CompartmentId - Optionally receives the compartment ID.

Return Value:

    None.

Permissions:

    This is open to all callers.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
SetCurrentThreadCompartmentScope(
    _In_ NET_IF_COMPARTMENT_SCOPE CompartmentScope
    );
/*++

Routine Description:

    Sets the compartment scope for the current thread.

Arguments:

    CompartmentScope - Supplies the compartment scope.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Permissions:

    This is a privileged operation available only to admin-like accounts
    used by services (LocalSystem, NetworkService, LocalService).

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
NET_IF_COMPARTMENT_ID
NETIOAPI_API_
GetJobCompartmentId(
    _In_ HANDLE JobHandle
    );
/*++

Routine Description:

    Gets the compartment ID of a given job object.

Arguments:

    JobHandle - Supplies the job handle.

Return Value:

    Returns the compartment ID associated with the job.

Permissions:

    This is open to all callers.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
SetJobCompartmentId(
    _In_ HANDLE JobHandle,
    _In_ NET_IF_COMPARTMENT_ID CompartmentId
    );
/*++

Routine Description:

    Sets the compartment ID of the given job object.

Arguments:

    JobHandle - Supplies the job handle.

    CompartmentId - Supplies the new compartment ID.

Return Value:

    If the function succeeds, the return value is NO_ERROR.
    If the function fails, use FormatMessage to obtain the message string for
    the returned error.

Permissions:

    This is a privileged operation available only to network-admin-like
    accounts (LocalSystem, NetworkService, LocalService).

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
NET_IF_COMPARTMENT_ID
NETIOAPI_API_
GetSessionCompartmentId(
    _In_ ULONG SessionId
    );
/*++

Routine Description:

    Get the compartment ID of the session.

Arguments:

    SessionId - Supplies the session ID.

Return Value:

    The compartment ID of the session.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
SetSessionCompartmentId(
    _In_ ULONG SessionId,
    _In_ NET_IF_COMPARTMENT_ID CompartmentId
    );
/*++

Routine Description:

    Set the compartment ID of the session.

Arguments:

    SessionId - Supplies the session ID.

    CompartmentId - Supplies the compartment ID to be set.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
NET_IF_COMPARTMENT_ID
NETIOAPI_API_
GetDefaultCompartmentId(
    VOID
    );
/*++

Routine Description:

    Returns the default compartment ID of the current network namespace.

Return Value:

    Returns the default Compartment ID if successful,
    NET_IF_COMPARTMENT_ID_UNSPECIFIED otherwise.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetNetworkInformation(
    _In_ CONST NET_IF_NETWORK_GUID *NetworkGuid,
    _Out_ PNET_IF_COMPARTMENT_ID CompartmentId,
    _Out_ PULONG SiteId,
    _Out_writes_(Length) PWCHAR NetworkName,
    _In_ ULONG Length
    );
/*++

Routine Description:

    Get the network information.

Arguments:

    NetworkGuid - Supplies the Network GUID.

    CompartmentId - Returns the compartment ID.

    SiteId - Returns Site ID.

    NetowrkName - Returns the network name.

    Length - Supplies the length of NetworkName buffer.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
SetNetworkInformation(
    _In_ CONST NET_IF_NETWORK_GUID *NetworkGuid,
    _In_ NET_IF_COMPARTMENT_ID CompartmentId,
    _In_z_ CONST WCHAR *NetworkName
    );
/*++

Routine Description:

    Set the Network Information.

Arguments:

    NetworkGuid - Supplies the session ID.

    CompartmentId - Supplies the compartment ID to be set.

    NetworkName - Supplies the Network name to be set.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

#pragma warning(pop)

IPHLPAPI_DLL_LINKAGE
NETIOAPI_API
ConvertLengthToIpv4Mask(
    _In_ ULONG MaskLength,
    _Out_ PULONG Mask
    );
/*++

Routine Description:

    Converts a prefixLength to a subnet mask.

Arguments:

    MaskLength - Prefix Length.

    Mask - Mask generated.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

IPHLPAPI_DLL_LINKAGE
NETIOAPI_API
ConvertIpv4MaskToLength(
    _In_ ULONG Mask,
    _Out_ PUINT8 MaskLength
    );
/*++

Routine Description:

    Converts a subnet mask to a prefix length.

Arguments:

    Mask - Subnet mask to use.

    MaskLength - Prefix length computed.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

#define DNS_SETTINGS_VERSION1                     0x0001
#define DNS_SETTINGS_VERSION2                     0x0002

#define DNS_INTERFACE_SETTINGS_VERSION1           0x0001
#define DNS_INTERFACE_SETTINGS_VERSION2           0x0002
#define DNS_INTERFACE_SETTINGS_VERSION3           0x0003
#define DNS_INTERFACE_SETTINGS_VERSION4           0x0004


#define DNS_SETTING_IPV6                          0x0001
#define DNS_SETTING_NAMESERVER                    0x0002
#define DNS_SETTING_SEARCHLIST                    0x0004
#define DNS_SETTING_REGISTRATION_ENABLED          0x0008
#define DNS_SETTING_REGISTER_ADAPTER_NAME         0x0010
#define DNS_SETTING_DOMAIN                        0x0020
#define DNS_SETTING_HOSTNAME                      0x0040
#define DNS_SETTINGS_ENABLE_LLMNR                 0x0080
#define DNS_SETTINGS_QUERY_ADAPTER_NAME           0x0100
#define DNS_SETTING_PROFILE_NAMESERVER            0x0200
#define DNS_SETTING_DISABLE_UNCONSTRAINED_QUERIES 0x0400
#define DNS_SETTING_SUPPLEMENTAL_SEARCH_LIST      0x0800
#define DNS_SETTING_DOH                           0x1000
#define DNS_SETTING_DOH_PROFILE                   0x2000
#define DNS_SETTING_ENCRYPTED_DNS_ADAPTER_FLAGS   0x4000
#define DNS_SETTING_DDR                           0x8000
#define DNS_SETTING_DOT                           0x10000
#define DNS_SETTING_DOT_PROFILE                   0x20000

#define DNS_ENABLE_DOH                            0x0001

#define DNS_DOH_POLICY_NOT_CONFIGURED             0x0004
#define DNS_DOH_POLICY_DISABLE                    0x0008
#define DNS_DOH_POLICY_AUTO                       0x0010
#define DNS_DOH_POLICY_REQUIRED                   0x0020

//
//  For backwards compatibility reasons, we need to keep the old names as well.
//

#define DNS_ENCRYPTION_POLICY_NOT_CONFIGURED      DNS_DOH_POLICY_NOT_CONFIGURED
#define DNS_ENCRYPTION_POLICY_DISABLE             DNS_DOH_POLICY_DISABLE
#define DNS_ENCRYPTION_POLICY_AUTO                DNS_DOH_POLICY_AUTO
#define DNS_ENCRYPTION_POLICY_REQUIRED            DNS_DOH_POLICY_REQUIRED

#define DNS_ENABLE_DDR                            0x0040
#define DNS_ENABLE_DOT                            0x0080
#define DNS_DOT_POLICY_BLOCK                      0x0100
#define DNS_DOH_POLICY_BLOCK                      0x0200
#define DNS_ENABLE_DNR                            0x0400

#define DNS_SERVER_PROPERTY_VERSION1              0x0001

#define DNS_DOH_SERVER_SETTINGS_ENABLE_AUTO                         0x0001
#define DNS_DOH_SERVER_SETTINGS_ENABLE                              0x0002
#define DNS_DOH_SERVER_SETTINGS_FALLBACK_TO_UDP                     0x0004
#define DNS_DOH_AUTO_UPGRADE_SERVER                                 0x0008
#define DNS_DOH_SERVER_SETTINGS_ENABLE_DDR                          0x0010
#define DNS_DOH_SERVER_SETTINGS_MAKE_DDR_NON_BLOCKING               0x0020

#define DNS_DOT_SERVER_SETTINGS_ENABLE                              0x0001
#define DNS_DOT_SERVER_SETTINGS_FALLBACK_TO_UDP                     0x0002
#define DNS_DOT_AUTO_UPGRADE_SERVER                                 0x0004
#define DNS_DOT_SERVER_SETTINGS_ENABLE_AUTO                         0x0008
#define DNS_DOT_SERVER_SETTINGS_ENABLE_DDR                          0x0010
#define DNS_DOT_SERVER_SETTINGS_MAKE_DDR_NON_BLOCKING               0x0020

#define DNS_DDR_ADAPTER_ENABLE_DOH                0x0001
#define DNS_DDR_ADAPTER_ENABLE                    DNS_DDR_ADAPTER_ENABLE_DOH
#define DNS_DDR_ADAPTER_ENABLE_UDP_FALLBACK       0x0002
#define DNS_DDR_ADAPTER_MAKE_DDR_NON_BLOCKING     0x0004

typedef struct _DNS_SETTINGS
{
    ULONG Version;
    ULONG64 Flags;
    PWSTR Hostname;
    PWSTR Domain;
    PWSTR SearchList;
} DNS_SETTINGS;

typedef struct _DNS_SETTINGS2
{
    ULONG Version;
    ULONG64 Flags;
    PWSTR Hostname;
    PWSTR Domain;
    PWSTR SearchList;
    ULONG64 SettingFlags;
} DNS_SETTINGS2;

typedef struct _DNS_DOH_SERVER_SETTINGS
{
#ifdef MIDL_PASS
    [unique, string] PWSTR Template;
#else
    PWSTR Template;
#endif
    ULONG64 Flags;
} DNS_DOH_SERVER_SETTINGS;

typedef struct _DNS_DOT_SERVER_SETTINGS
{
#ifdef MIDL_PASS
    [unique, string] PWSTR Hostname;
#else
    PWSTR Hostname;
#endif

    ULONG64 Flags;
    USHORT Port;
} DNS_DOT_SERVER_SETTINGS;

typedef enum _DNS_SERVER_PROPERTY_TYPE
{
    DnsServerInvalidProperty = 0,
    DnsServerDohProperty,
    DnsServerDotProperty
} DNS_SERVER_PROPERTY_TYPE;

#ifdef MIDL_PASS
typedef [switch_type(DNS_SERVER_PROPERTY_TYPE)] union _DNS_SERVER_PROPERTY_TYPES
{
    [case(DnsServerDohProperty)] [unique] DNS_DOH_SERVER_SETTINGS *DohSettings;
    [case(DnsServerDotProperty)] [unique] DNS_DOT_SERVER_SETTINGS *DotSettings;
} DNS_SERVER_PROPERTY_TYPES;
#else
typedef union _DNS_SERVER_PROPERTY_TYPES
{
    DNS_DOH_SERVER_SETTINGS *DohSettings;
    DNS_DOT_SERVER_SETTINGS *DotSettings;
} DNS_SERVER_PROPERTY_TYPES;
#endif

typedef struct _DNS_SERVER_PROPERTY
{
    ULONG Version;
    ULONG ServerIndex;
    DNS_SERVER_PROPERTY_TYPE Type;
#ifdef MIDL_PASS
    [switch_is(Type)] DNS_SERVER_PROPERTY_TYPES Property;
#else
    DNS_SERVER_PROPERTY_TYPES Property;
#endif
} DNS_SERVER_PROPERTY;

typedef struct _DNS_INTERFACE_SETTINGS
{
    ULONG Version;
    ULONG64 Flags;
    PWSTR Domain;
    PWSTR NameServer;
    PWSTR SearchList;
    ULONG RegistrationEnabled;
    ULONG RegisterAdapterName;
    ULONG EnableLLMNR;
    ULONG QueryAdapterName;
    PWSTR ProfileNameServer;
} DNS_INTERFACE_SETTINGS;

typedef struct _DNS_INTERFACE_SETTINGS_EX
{
    DNS_INTERFACE_SETTINGS SettingsV1;
    ULONG DisableUnconstrainedQueries;
    PWSTR SupplementalSearchList;
} DNS_INTERFACE_SETTINGS_EX;

typedef struct _DNS_INTERFACE_SETTINGS3
{
    ULONG Version;
    ULONG64 Flags;
    PWSTR Domain;
    PWSTR NameServer;
    PWSTR SearchList;
    ULONG RegistrationEnabled;
    ULONG RegisterAdapterName;
    ULONG EnableLLMNR;
    ULONG QueryAdapterName;
    PWSTR ProfileNameServer;
    ULONG DisableUnconstrainedQueries;
    PWSTR SupplementalSearchList;
    ULONG cServerProperties;
    DNS_SERVER_PROPERTY *ServerProperties;
    ULONG cProfileServerProperties;
    DNS_SERVER_PROPERTY *ProfileServerProperties;
} DNS_INTERFACE_SETTINGS3;

typedef struct _DNS_INTERFACE_SETTINGS4
{
    ULONG Version;
    ULONG64 Flags;
    PWSTR Domain;
    PWSTR NameServer;
    PWSTR SearchList;
    ULONG RegistrationEnabled;
    ULONG RegisterAdapterName;
    ULONG EnableLLMNR;
    ULONG QueryAdapterName;
    PWSTR ProfileNameServer;
    ULONG DisableUnconstrainedQueries;
    PWSTR SupplementalSearchList;
    ULONG cServerProperties;
    DNS_SERVER_PROPERTY *ServerProperties;
    ULONG cProfileServerProperties;
    DNS_SERVER_PROPERTY *ProfileServerProperties;
    ULONG EncryptedDnsAdapterFlags;
} DNS_INTERFACE_SETTINGS4;

NETIOAPI_API
GetDnsSettings(
    _Inout_ DNS_SETTINGS *Settings
    );
/*++

Routine Description:

    Retrieves the DNS settings specified in the Settings parameter.

    The user must call FreeDnsSettings(Settings) afterwards.

Arguments:

    Settings - GetDnsSettings will populate all the settings
        in this structure. Version must be set.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

VOID
NETIOAPI_API_
FreeDnsSettings(
    _Inout_ DNS_SETTINGS *Settings
    );
/*++

Routine Description:

    Frees the settings previously retrieved by GetDnsSettings.
    This is done by invoking MIDL_user_free.

Arguments:

    Settings - The settings structure. Version must be set.

Return Value:

    None.

--*/

NETIOAPI_API
SetDnsSettings(
    _In_ const DNS_SETTINGS *Settings
    );
/*++

Routine Description:

    Set the DNS settings specified in the Settings parameter.

Arguments:

    Settings - Specifies the settings to be set.
        Version must be set.
        Flags is used to control which settings are to be set.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

NETIOAPI_API
GetInterfaceDnsSettings(
    _In_ GUID Interface,
    _Inout_ DNS_INTERFACE_SETTINGS *Settings
    );
/*++

Routine Description:

    Retrieves the DNS settings specified in the Settings parameter.

    The user must call FreeInterfaceDnsSettings(Settings) afterwards.

Arguments:

    Interface - The Interface GUID that the settings refer to.

    Settings - GetInterfaceDnsSettings will populate all the settings
        in this structure. Version must be set.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

VOID
NETIOAPI_API_
FreeInterfaceDnsSettings(
    _Inout_ DNS_INTERFACE_SETTINGS *Settings
    );
/*++

Routine Description:

    Frees the settings previously retrieved by GetInterfaceDnsSettings.
    This is done by invoking MIDL_user_free.

Arguments:

    Settings - The settings structure. Version must be set.

Return Value:

    None.

--*/

NETIOAPI_API
SetInterfaceDnsSettings(
    _In_ GUID Interface,
    _In_ const DNS_INTERFACE_SETTINGS *Settings
    );
/*++

Routine Description:

    Set the per interface DNS settings specified in the Settings parameter.

Arguments:

    Interface - The Interface GUID that the settings refer to.

    Settings - Specifies the settings to be set.
        Version must be set.
        Flags is used to control which settings are to be set.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetNetworkConnectivityHint(
    _Out_ NL_NETWORK_CONNECTIVITY_HINT *ConnectivityHint
    );
/*++

Routine Description:

    Queries for the aggregate level and cost of network connectivity that an
    app or service is likely to experience.

Arguments:

    ConnectivityHint - Receives the aggregate connectivity level and cost hints.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetNetworkConnectivityHintForInterface(
    _In_ NET_IFINDEX InterfaceIndex,
    _Out_ NL_NETWORK_CONNECTIVITY_HINT *ConnectivityHint
    );
/*++

Routine Description:

    Queries for the level and cost of network connectivity for the specified
    interface.

Arguments:

    InterfaceIndex - The index of the interface for which to retrieve
        connectivity information.

    ConnectivityHint - Receives the connectivity level and cost hints for the
        specified interface.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

--*/

typedef
VOID
(NETIOAPI_API_ *PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK) (
    _In_ PVOID CallerContext,
    _In_ NL_NETWORK_CONNECTIVITY_HINT ConnectivityHint
    );

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
NotifyNetworkConnectivityHintChange(
    _In_ PNETWORK_CONNECTIVITY_HINT_CHANGE_CALLBACK Callback,
    _In_opt_ PVOID CallerContext,
    _In_ BOOLEAN InitialNotification,
    _Out_ PHANDLE NotificationHandle
    );
/*++

Routine Description:

    Register for notification when the aggregate network connectivity level
    and cost hints change.

Arguments:

    Callback - Supplies a callback function.  This function will be invoked
        when a network connectivity level or cost change occurs.

    CallerContext - Provides the user specific caller context.  This context
        will be supplied to the callback function.

    InitialNotification - Supplies a boolean to indicate whether an
        initialization notification should be provided.

    NotificationHandle - Returns a handle to the notification registration.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    1. Invocation of the callback function is serialized.

    2. Use CancelMibChangeNotify2 to deregister for change notifications.

--*/

#ifdef _WS2DEF_

//
// FL virtual interface routines.
//

typedef enum _NET_FL_VIRTUAL_INTERFACE_ORIGIN {
    NetFlVirtualInterfaceOriginOid,
    NetFlVirtualInterfaceOriginApi,
    NetFlVirtualInterfaceOriginDefault,
} NET_FL_VIRTUAL_INTERFACE_ORIGIN;

typedef enum _NET_FL_ISOLATION_MODE {
    NetFlIsolationModeNone,
    NetFlIsolationModeVlan,
    NetFlIsolationModeVsid,
} NET_FL_ISOLATION_MODE;

typedef struct _MIB_FL_VIRTUAL_INTERFACE_ROW {
    //
    // Key fields.
    //

    ADDRESS_FAMILY Family;
    IF_LUID IfLuid;
    ULONG VirtualIfId;

    //
    // Mandatory fields for create, read-only for existing entries.
    //
    GUID CompartmentGuid;
    NET_FL_ISOLATION_MODE IsolationMode;

    //
    // Read-only fields.
    //

    NET_FL_VIRTUAL_INTERFACE_ORIGIN Origin;
    IF_LUID VirtualIfLuid;
    IF_INDEX VirtualIfIndex;

    //
    // Read-Write fields.
    //

    BOOLEAN AllowLocalNd;

    //
    // Statistics.
    //
    ULONG AttachedFlsnpiClients;
    ULONG FlsnpiClientConfigErrors;
    ULONG64 FlsnpiClientInjectErrors;
    ULONG64 FlsnpiClientCloneErrors;
    ULONG64 InFlsnpiIndicatedPackets;
    ULONG64 InFlsnpiClientReturnedPackets;
    ULONG64 InFlsnpiClientSilentlyDroppedPackets;
    ULONG64 InFlsnpiClientDroppedPackets;
    ULONG64 InFlsnpiClientInjectedPackets;
    ULONG64 InFlsnpiClientClonedPackets;
    ULONG64 OutFlsnpiIndicatedPackets;
    ULONG64 OutFlsnpiClientReturnedPackets;
    ULONG64 OutFlsnpiClientDroppedPackets;
    ULONG64 OutFlsnpiClientSilentlyDroppedPackets;
    ULONG64 OutFlsnpiClientInjectedPackets;
    ULONG64 OutFlsnpiClientClonedPackets;
    ULONG64 OutFlsnpiClientClonedPacketsForNbSplit;
} MIB_FL_VIRTUAL_INTERFACE_ROW, *PMIB_FL_VIRTUAL_INTERFACE_ROW;

typedef struct _MIB_FL_VIRTUAL_INTERFACE_TABLE {
    ULONG NumEntries;
    MIB_FL_VIRTUAL_INTERFACE_ROW Table[ANY_SIZE];
} MIB_FL_VIRTUAL_INTERFACE_TABLE, *PMIB_FL_VIRTUAL_INTERFACE_TABLE;

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
CreateFlVirtualInterface(
    _In_ CONST MIB_FL_VIRTUAL_INTERFACE_ROW *Row
    );
/*++

Routine Description:

    Create an FL virtual interface on the local computer.

Arguments:

    Row - Supplies a MIB_FL_VIRTUAL_INTERFACE_ROW structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    InitializeFlVirtualInterfaceEntry must be used to initialize the fields of
    MIB_FL_VIRTUAL_INTERFACE_ROW with default values.

    On input, the following fields of Row must be set:

    1. Family.
    2. IfLuid.
    3. VirtualIfId.
    4. CompartmentGuid.
    5. IsolationMode.

    Optionally, any read/write field may also be set.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
DeleteFlVirtualInterface(
    _In_ CONST MIB_FL_VIRTUAL_INTERFACE_ROW *Row
    );
/*++

Routine Description:

    Delete an FL virtual interface on the local computer.

Arguments:

    Row - Supplies a MIB_FL_VIRTUAL_INTERFACE_ROW structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1. Family.
    2. IfLuid.
    3. VirtualIfId.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
VOID
InitializeFlVirtualInterfaceEntry(
    _Out_ PMIB_FL_VIRTUAL_INTERFACE_ROW Row
    );
/*++

Routine Description:

    Initialize an MIB_FL_VIRTUAL_INTERFACE_ROW entry.

Arguments:

    Row - The MIB_FL_VIRTUAL_INTERFACE_ROW entry to initialize.

Return Value:

    None.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
SetFlVirtualInterface(
    _In_ CONST MIB_FL_VIRTUAL_INTERFACE_ROW *Row
    );
/*++

Routine Description:

    Sets information for the specified FL virtual interface on the local
    computer.

Arguments:

    Row - Supplies a PMIB_FL_VIRTUAL_INTERFACE_ROW structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    InitializeFlVirtualInterfaceEntry must be used to initialize the fields of
    MIB_FL_VIRTUAL_INTERFACE_ROW with default values.  The caller can then
    update the fields it wishes to modify and invoke SetFlVirtualInterface.

    On input, the following key fields of Row must be initialized:
    1. Family.
    2. IfLuid.
    3. VirtualIfId.

    The FL virtual interface is updated the with value of any modified
    read-write field.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetFlVirtualInterface(
    _Inout_ PMIB_FL_VIRTUAL_INTERFACE_ROW Row
    );
/*++

Routine Description:

    Retrieves information for the specified FL virtual interface on the local
    computer.

Arguments:

    Row - Supplies a PMIB_FL_VIRTUAL_INTERFACE_ROW structure.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    On input, the following key fields of Row must be initialized:
    1. Family.
    2. IfLuid.
    3. VirtualIfId.

    On output, the remaining fields of Row are filled in.

--*/

_IRQL_requires_max_(PASSIVE_LEVEL)
IPHLPAPI_DLL_LINKAGE
_NETIOAPI_SUCCESS_
NETIOAPI_API
GetFlVirtualInterfaceTable(
    _In_ ADDRESS_FAMILY Family,
    _Outptr_ PMIB_FL_VIRTUAL_INTERFACE_TABLE *Table
    );
/*++

Routine Description:

    Retrieves the FL virtual interface table on a local computer.

Arguments:

    Family - Supplies the address family.

        AF_INET: Only returns IPv4 FL virtual interfaces.

        AF_INET6: Only returns IPv6 FL virtual interfaces.

        AF_UNSPEC: Returns both IPv4 and IPv6 FL virtual interfaces.

    Table - Returns the table of FL virtual interfaces in a
        PMIB_FL_VIRTUAL_INTERFACE_TABLE structure.  Use FreeMibTable to free this
        buffer.

Return Value:

    User-Mode: NO_ERROR on success, error code on failure.

    Kernel-Mode: STATUS_SUCCESS on success, error code on failure.

Notes:

    The API allocates the buffer for Table. Use FreeMibTable to free it.

--*/

#endif // _WS2DEF_

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
//4201.
//
#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // _NETIOAPI_H_.
