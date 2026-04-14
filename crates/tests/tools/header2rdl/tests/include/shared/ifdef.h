

#ifndef _IFDEF_
#define _IFDEF_
#pragma once
#include <winapifamily.h>

#pragma region Desktop Application or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifdef __cplusplus
extern "C" {
#endif

//
// Get definitions for IFTYPE and IF_ACCESS_TYPE.
//
#include <ipifcons.h>

//
// Define compartment ID type:
//
typedef UINT32 NET_IF_COMPARTMENT_ID, *PNET_IF_COMPARTMENT_ID;

#define NET_IF_COMPARTMENT_ID_UNSPECIFIED       (NET_IF_COMPARTMENT_ID)0
#define NET_IF_COMPARTMENT_ID_PRIMARY           (NET_IF_COMPARTMENT_ID)1

//
// Define NetworkGUID type:
//
typedef GUID NET_IF_NETWORK_GUID, *PNET_IF_NETWORK_GUID;

typedef enum _NET_IF_OPER_STATUS   // ifOperStatus
{
    NET_IF_OPER_STATUS_UP = 1,
    NET_IF_OPER_STATUS_DOWN = 2,
    NET_IF_OPER_STATUS_TESTING = 3,
    NET_IF_OPER_STATUS_UNKNOWN = 4,
    NET_IF_OPER_STATUS_DORMANT = 5,
    NET_IF_OPER_STATUS_NOT_PRESENT = 6,
    NET_IF_OPER_STATUS_LOWER_LAYER_DOWN = 7
} NET_IF_OPER_STATUS, *PNET_IF_OPER_STATUS;

typedef ULONG32 NET_IF_OBJECT_ID, *PNET_IF_OBJECT_ID;

typedef enum _NET_IF_ADMIN_STATUS   // ifAdminStatus
{
    NET_IF_ADMIN_STATUS_UP = 1,
    NET_IF_ADMIN_STATUS_DOWN = 2,
    NET_IF_ADMIN_STATUS_TESTING = 3
} NET_IF_ADMIN_STATUS, *PNET_IF_ADMIN_STATUS;


//
// Flags to extend operational status
//
#define NET_IF_OPER_STATUS_DOWN_NOT_AUTHENTICATED           0x00000001
#define NET_IF_OPER_STATUS_DOWN_NOT_MEDIA_CONNECTED         0x00000002
#define NET_IF_OPER_STATUS_DORMANT_PAUSED                   0x00000004
#define NET_IF_OPER_STATUS_DORMANT_LOW_POWER                0x00000008

typedef UINT32 NET_IF_COMPARTMENT_SCOPE, *PNET_IF_COMPARTMENT_SCOPE;

#define NET_IF_COMPARTMENT_SCOPE_UNSPECIFIED    (NET_IF_COMPARTMENT_SCOPE)0
#define NET_IF_COMPARTMENT_SCOPE_ALL            ((NET_IF_COMPARTMENT_SCOPE)-1)

#define NET_IF_OID_IF_ALIAS             0x00000001  // identifies the ifAlias string for an interface
#define NET_IF_OID_COMPARTMENT_ID       0x00000002  // identifies the compartment ID for an interface.
#define NET_IF_OID_NETWORK_GUID         0x00000003  // identifies the NetworkGuid for an interface.
#define NET_IF_OID_IF_ENTRY             0x00000004  // identifies statistics for an interface.

//
// Define macros for an "unspecified" NetworkGUID value to be used in structures
// that haven't had the NET_LUID field filled in yet.
//
#define NET_SET_UNSPECIFIED_NETWORK_GUID(_pNetworkGuid)
#define NET_IS_UNSPECIFIED_NETWORK_GUID(_NetworkGuidValue)

//
// To prevent collisions between user-assigned and system-assigned site-ids,
// we partition the site-id space into two:
// 1. User-Assigned: NET_SITEID_UNSPECIFIED < SiteId < NET_SITEID_MAXUSER
// 2. System-Assigned: NET_SITEID_MAXUSER < SiteId < NET_SITEID_MAXSYSTEM
//
// Note: A network's SiteId doesn't really need to be settable.
// 1. The network profile manager creates a network per network profile.
// 2. NDIS/IF assigns a unique SiteId to each network.
//
#define NET_SITEID_UNSPECIFIED (0)
#define NET_SITEID_MAXUSER (0x07ffffff)
#define NET_SITEID_MAXSYSTEM (0x0fffffff)
#ifndef MIDL_PASS
C_ASSERT(NET_SITEID_MAXUSER < NET_SITEID_MAXSYSTEM);
#endif

typedef enum _NET_IF_RCV_ADDRESS_TYPE // ifRcvAddressType
{
    NET_IF_RCV_ADDRESS_TYPE_OTHER = 1,
    NET_IF_RCV_ADDRESS_TYPE_VOLATILE = 2,
    NET_IF_RCV_ADDRESS_TYPE_NON_VOLATILE = 3
} NET_IF_RCV_ADDRESS_TYPE, *PNET_IF_RCV_ADDRESS_TYPE;

typedef struct _NET_IF_RCV_ADDRESS_LH
{
    NET_IF_RCV_ADDRESS_TYPE ifRcvAddressType;
    USHORT                  ifRcvAddressLength;
    USHORT                  ifRcvAddressOffset; // from beginning of this struct
} NET_IF_RCV_ADDRESS_LH, *PNET_IF_RCV_ADDRESS_LH;

typedef struct _NET_IF_ALIAS_LH
{
    USHORT  ifAliasLength;  // in bytes, of ifAlias string
    USHORT  ifAliasOffset;  // in bytes, from beginning of this struct
} NET_IF_ALIAS_LH, *PNET_IF_ALIAS_LH;

#pragma warning(push)
#pragma warning(disable:4214) // bit field types other than int
typedef union _NET_LUID_LH
{
    ULONG64     Value;
    struct
    {
        ULONG64     Reserved:24;
        ULONG64     NetLuidIndex:24;
        ULONG64     IfType:16;                  // equal to IANA IF type
    }Info;
} NET_LUID_LH, *PNET_LUID_LH;
#pragma warning(pop)

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef NET_IF_RCV_ADDRESS_LH NET_IF_RCV_ADDRESS;
typedef NET_IF_RCV_ADDRESS* PNET_IF_RCV_ADDRESS;

typedef NET_IF_ALIAS_LH NET_IF_ALIAS;
typedef NET_IF_ALIAS* PNET_IF_ALIAS;
#endif //NTDDI_VISTA

//
// Need to make this visible on all platforms (for the purpose of IF_LUID).
//
typedef NET_LUID_LH NET_LUID;
typedef NET_LUID* PNET_LUID;

//
// IF_LUID
//
// Define the locally unique datalink interface identifier type.
// This type is persistable.
//
typedef NET_LUID IF_LUID, *PIF_LUID;


typedef ULONG NET_IFINDEX, *PNET_IFINDEX;       // Interface Index (ifIndex)
typedef UINT16 NET_IFTYPE, *PNET_IFTYPE;        // Interface Type (IANA ifType)

#define NET_IFINDEX_UNSPECIFIED (NET_IFINDEX)(0)    // Not a valid ifIndex
#define NET_IFLUID_UNSPECIFIED (0)    // Not a valid if Luid

#ifndef __IF_INDEX_DEFINED__
#define __IF_INDEX_DEFINED__

//
// IF_INDEX
//
// Define the interface index type.
// This type is not persistable.
// This must be unsigned (not an enum) to replace previous uses of
// an index that used a DWORD type.
//

#endif // __IF_INDEX_DEFINED__

typedef NET_IFINDEX IF_INDEX, *PIF_INDEX;


typedef enum _NET_IF_CONNECTION_TYPE
{
   NET_IF_CONNECTION_DEDICATED = 1,
   NET_IF_CONNECTION_PASSIVE = 2,
   NET_IF_CONNECTION_DEMAND = 3,
   NET_IF_CONNECTION_MAXIMUM = 4
} NET_IF_CONNECTION_TYPE, *PNET_IF_CONNECTION_TYPE;

//
// Types of tunnels (sub-type of IF_TYPE when IF_TYPE is IF_TYPE_TUNNEL).
// See http://www.iana.org/assignments/ianaiftype-mib.
//
typedef enum {
    TUNNEL_TYPE_NONE = 0,
    TUNNEL_TYPE_OTHER = 1,
    TUNNEL_TYPE_DIRECT = 2,
    TUNNEL_TYPE_6TO4 = 11,
    TUNNEL_TYPE_ISATAP = 13,
    TUNNEL_TYPE_TEREDO = 14,
    TUNNEL_TYPE_IPHTTPS = 15,
} TUNNEL_TYPE, *PTUNNEL_TYPE;

#define IFI_UNSPECIFIED NET_IFINDEX_UNSPECIFIED

//
// Definitions for NET_IF_INFORMATION.Flags
//
#define NIIF_HARDWARE_INTERFACE         0x00000001  // Set iff hardware
#define NIIF_FILTER_INTERFACE           0x00000002
#define NIIF_NDIS_RESERVED1             0x00000004
#define NIIF_NDIS_RESERVED2             0x00000008
#define NIIF_NDIS_RESERVED3             0x00000010
#define NIIF_NDIS_WDM_INTERFACE         0x00000020
#define NIIF_NDIS_ENDPOINT_INTERFACE    0x00000040
#define NIIF_NDIS_ISCSI_INTERFACE       0x00000080
#define NIIF_NDIS_RESERVED4             0x00000100

#define NIIF_WAN_TUNNEL_TYPE_UNKNOWN    ((ULONG)(-1))

//
// Define datalink interface access types.
//
typedef enum _NET_IF_ACCESS_TYPE
{
    NET_IF_ACCESS_LOOPBACK = 1,
    NET_IF_ACCESS_BROADCAST = 2,
    NET_IF_ACCESS_POINT_TO_POINT = 3,
    NET_IF_ACCESS_POINT_TO_MULTI_POINT = 4,
    NET_IF_ACCESS_MAXIMUM = 5
} NET_IF_ACCESS_TYPE, *PNET_IF_ACCESS_TYPE;


//
// Define datalink interface direction types.
//
typedef enum _NET_IF_DIRECTION_TYPE
{
    NET_IF_DIRECTION_SENDRECEIVE,
    NET_IF_DIRECTION_SENDONLY,
    NET_IF_DIRECTION_RECEIVEONLY,
    NET_IF_DIRECTION_MAXIMUM
} NET_IF_DIRECTION_TYPE, *PNET_IF_DIRECTION_TYPE;


typedef enum _NET_IF_MEDIA_CONNECT_STATE
{
    MediaConnectStateUnknown,
    MediaConnectStateConnected,
    MediaConnectStateDisconnected
} NET_IF_MEDIA_CONNECT_STATE, *PNET_IF_MEDIA_CONNECT_STATE;

#define NET_IF_LINK_SPEED_UNKNOWN   ((ULONG64)(-1))

//
// Defines the duplex state of media
//
typedef enum _NET_IF_MEDIA_DUPLEX_STATE
{
    MediaDuplexStateUnknown,
    MediaDuplexStateHalf,
    MediaDuplexStateFull
} NET_IF_MEDIA_DUPLEX_STATE, *PNET_IF_MEDIA_DUPLEX_STATE;


// Special values for fields in NET_PHYSICAL_LOCATION
#define NIIF_BUS_NUMBER_UNKNOWN         ((ULONG)(-1))
#define NIIF_SLOT_NUMBER_UNKNOWN        ((ULONG)(-1))
#define NIIF_FUNCTION_NUMBER_UNKNOWN    ((ULONG)(-1))

typedef struct _NET_PHYSICAL_LOCATION_LH
{
    ULONG                   BusNumber;          // Physical location
    ULONG                   SlotNumber;         // ... for hardware
    ULONG                   FunctionNumber;     // ... devices.
} NET_PHYSICAL_LOCATION_LH, *PNET_PHYSICAL_LOCATION_LH;

//
// maximum string size in -wchar- units
//
#define IF_MAX_STRING_SIZE 256

typedef struct _IF_COUNTED_STRING_LH
{
    _Field_range_(0, (IF_MAX_STRING_SIZE + 1) * sizeof(WCHAR))
    USHORT      Length; // in -Bytes-
    WCHAR       String[IF_MAX_STRING_SIZE + 1];
} IF_COUNTED_STRING_LH, *PIF_COUNTED_STRING_LH;

#define IF_MAX_PHYS_ADDRESS_LENGTH 32

typedef struct _IF_PHYSICAL_ADDRESS_LH
{
    _Field_range_(0, IF_MAX_PHYS_ADDRESS_LENGTH)
    USHORT      Length;
    UCHAR       Address[IF_MAX_PHYS_ADDRESS_LENGTH];
} IF_PHYSICAL_ADDRESS_LH, *PIF_PHYSICAL_ADDRESS_LH;

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef NET_PHYSICAL_LOCATION_LH NET_PHYSICAL_LOCATION;
typedef NET_PHYSICAL_LOCATION* PNET_PHYSICAL_LOCATION;

typedef IF_COUNTED_STRING_LH IF_COUNTED_STRING;
typedef IF_COUNTED_STRING* PIF_COUNTED_STRING;

typedef IF_PHYSICAL_ADDRESS_LH IF_PHYSICAL_ADDRESS;
typedef IF_PHYSICAL_ADDRESS* PIF_PHYSICAL_ADDRESS;
#endif



//
// IF_ADMINISTRATIVE_STATE
//
// Datalink Interface Administrative State.
// Indicates whether the interface has been administratively enabled.
//

typedef enum _IF_ADMINISTRATIVE_STATE {
    IF_ADMINISTRATIVE_DISABLED,
    IF_ADMINISTRATIVE_ENABLED,
    IF_ADMINISTRATIVE_DEMANDDIAL,
} IF_ADMINISTRATIVE_STATE, *PIF_ADMINISTRATIVE_STATE;

//
// Note: Interface is Operational iff
// ((MediaSense is Connected) and (AdministrativeState is Enabled))
// or
// ((MediaSense is Connected) and (AdministrativeState is OnDemand))
//
// !Operational iff
// ((MediaSense != Connected) or (AdministrativeState is Disabled))
//

//
// OperStatus values from RFC 2863
//
typedef enum {
    IfOperStatusUp = 1,
    IfOperStatusDown,
    IfOperStatusTesting,
    IfOperStatusUnknown,
    IfOperStatusDormant,
    IfOperStatusNotPresent,
    IfOperStatusLowerLayerDown
} IF_OPER_STATUS;

typedef struct _NDIS_INTERFACE_INFORMATION
{
    //
    //  rod fields
    //
    NET_IF_OPER_STATUS          ifOperStatus;
    ULONG                       ifOperStatusFlags;
    NET_IF_MEDIA_CONNECT_STATE  MediaConnectState;
    NET_IF_MEDIA_DUPLEX_STATE   MediaDuplexState;
    ULONG                       ifMtu;
    BOOLEAN                     ifPromiscuousMode;
    BOOLEAN                     ifDeviceWakeUpEnable;
    ULONG64                     XmitLinkSpeed;
    ULONG64                     RcvLinkSpeed;

    ULONG64                     ifLastChange;
    ULONG64                     ifCounterDiscontinuityTime;
    ULONG64                     ifInUnknownProtos;

    //
    // OID_GEN_STATISTICS
    //
    ULONG64                     ifInDiscards;           // OID_GEN_RCV_DISCARDS = OID_GEN_RCV_ERROR + OID_GEN_RCV_NO_BUFFER
    ULONG64                     ifInErrors;             // OID_GEN_RCV_ERROR
    ULONG64                     ifHCInOctets;           // OID_GEN_BYTES_RCV = OID_GEN_DIRECTED_BYTES_RCV + OID_GEN_MULTICAST_BYTES_RCV + OID_GEN_BROADCAST_BYTES_RCV
    ULONG64                     ifHCInUcastPkts;        // OID_GEN_DIRECTED_FRAMES_RCV
    ULONG64                     ifHCInMulticastPkts;    // OID_GEN_MULTICAST_FRAMES_RCV
    ULONG64                     ifHCInBroadcastPkts;    // OID_GEN_BROADCAST_FRAMES_RCV
    ULONG64                     ifHCOutOctets;          // OID_GEN_BYTES_XMIT = OID_GEN_DIRECTED_BYTES_XMIT + OID_GEN_MULTICAST_BYTES_XMIT + OID_GEN_BROADCAST_BYTES_XMIT
    ULONG64                     ifHCOutUcastPkts;       // OID_GEN_DIRECTED_FRAMES_XMIT
    ULONG64                     ifHCOutMulticastPkts;   // OID_GEN_MULTICAST_FRAMES_XMIT
    ULONG64                     ifHCOutBroadcastPkts;   // OID_GEN_BROADCAST_FRAMES_XMIT
    ULONG64                     ifOutErrors;            // OID_GEN_XMIT_ERROR
    ULONG64                     ifOutDiscards;          // OID_GEN_XMIT_DISCARDS
    ULONG64                     ifHCInUcastOctets;      // OID_GEN_DIRECTED_BYTES_RCV
    ULONG64                     ifHCInMulticastOctets;  // OID_GEN_MULTICAST_BYTES_RCV
    ULONG64                     ifHCInBroadcastOctets;  // OID_GEN_BROADCAST_BYTES_RCV
    ULONG64                     ifHCOutUcastOctets;     // OID_GEN_DIRECTED_BYTES_XMIT
    ULONG64                     ifHCOutMulticastOctets; // OID_GEN_MULTICAST_BYTES_XMIT
    ULONG64                     ifHCOutBroadcastOctets; // OID_GEN_BROADCAST_BYTES_XMIT
    NET_IF_COMPARTMENT_ID       CompartmentId;
    ULONG                       SupportedStatistics;
}NDIS_INTERFACE_INFORMATION, *PNDIS_INTERFACE_INFORMATION;

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // _IFDEF_

