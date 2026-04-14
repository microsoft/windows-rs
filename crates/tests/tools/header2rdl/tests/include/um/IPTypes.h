/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    iptypes.h

--*/

#ifndef IP_TYPES_INCLUDED
#define IP_TYPES_INCLUDED

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifdef __cplusplus
extern "C" {
#endif

#pragma warning(push)
#pragma warning(disable:4201)
#pragma warning(disable:4214) // bit field types other than int

#include <time.h>
#include <ifdef.h>
#include <nldef.h>

// Definitions and structures used by getnetworkparams and getadaptersinfo apis

#define MAX_ADAPTER_DESCRIPTION_LENGTH  128 // arb.
#define MAX_ADAPTER_NAME_LENGTH         256 // arb.
#define MAX_ADAPTER_ADDRESS_LENGTH      8   // arb.
#define DEFAULT_MINIMUM_ENTITIES        32  // arb.
#define MAX_HOSTNAME_LEN                128 // arb.
#define MAX_DOMAIN_NAME_LEN             128 // arb.
#define MAX_SCOPE_ID_LEN                256 // arb.
#define MAX_DHCPV6_DUID_LENGTH          130 // RFC 3315.
#define MAX_DNS_SUFFIX_STRING_LENGTH    256

//
// types
//

// Node Type

#define BROADCAST_NODETYPE              1
#define PEER_TO_PEER_NODETYPE           2
#define MIXED_NODETYPE                  4
#define HYBRID_NODETYPE                 8

//
// IP_ADDRESS_STRING - store an IP address as a dotted decimal string
//

typedef struct {
    char String[4 * 4];
} IP_ADDRESS_STRING, *PIP_ADDRESS_STRING, IP_MASK_STRING, *PIP_MASK_STRING;

//
// IP_ADDR_STRING - store an IP address with its corresponding subnet mask,
// both as dotted decimal strings
//

typedef struct _IP_ADDR_STRING {
    struct _IP_ADDR_STRING* Next;
    IP_ADDRESS_STRING IpAddress;
    IP_MASK_STRING IpMask;
    DWORD Context;
} IP_ADDR_STRING, *PIP_ADDR_STRING;

//
// ADAPTER_INFO - per-adapter information. All IP addresses are stored as
// strings
//

typedef struct _IP_ADAPTER_INFO {
    struct _IP_ADAPTER_INFO* Next;
    DWORD ComboIndex;
    char AdapterName[MAX_ADAPTER_NAME_LENGTH + 4];
    char Description[MAX_ADAPTER_DESCRIPTION_LENGTH + 4];
    UINT AddressLength;
    BYTE Address[MAX_ADAPTER_ADDRESS_LENGTH];
    DWORD Index;
    UINT Type;
    UINT DhcpEnabled;
    PIP_ADDR_STRING CurrentIpAddress;
    IP_ADDR_STRING IpAddressList;
    IP_ADDR_STRING GatewayList;
    IP_ADDR_STRING DhcpServer;
    BOOL HaveWins;
    IP_ADDR_STRING PrimaryWinsServer;
    IP_ADDR_STRING SecondaryWinsServer;
    time_t LeaseObtained;
    time_t LeaseExpires;
} IP_ADAPTER_INFO, *PIP_ADAPTER_INFO;

#ifdef _WINSOCK2API_

//
// The following types require Winsock2.
//

typedef NL_PREFIX_ORIGIN IP_PREFIX_ORIGIN;
typedef NL_SUFFIX_ORIGIN IP_SUFFIX_ORIGIN;
typedef NL_DAD_STATE IP_DAD_STATE;

typedef struct _IP_ADAPTER_UNICAST_ADDRESS_LH {
    union {
        ULONGLONG Alignment;
        struct {
            ULONG Length;
            DWORD Flags;
        };
    };
    struct _IP_ADAPTER_UNICAST_ADDRESS_LH *Next;
    SOCKET_ADDRESS Address;

    IP_PREFIX_ORIGIN PrefixOrigin;
    IP_SUFFIX_ORIGIN SuffixOrigin;
    IP_DAD_STATE DadState;

    ULONG ValidLifetime;
    ULONG PreferredLifetime;
    ULONG LeaseLifetime;
    UINT8 OnLinkPrefixLength;
} IP_ADAPTER_UNICAST_ADDRESS_LH,
 *PIP_ADAPTER_UNICAST_ADDRESS_LH;

typedef struct _IP_ADAPTER_UNICAST_ADDRESS_XP {
    union {
        ULONGLONG Alignment;
        struct {
            ULONG Length;
            DWORD Flags;
        };
    };
    struct _IP_ADAPTER_UNICAST_ADDRESS_XP *Next;
    SOCKET_ADDRESS Address;

    IP_PREFIX_ORIGIN PrefixOrigin;
    IP_SUFFIX_ORIGIN SuffixOrigin;
    IP_DAD_STATE DadState;

    ULONG ValidLifetime;
    ULONG PreferredLifetime;
    ULONG LeaseLifetime;
} IP_ADAPTER_UNICAST_ADDRESS_XP, *PIP_ADAPTER_UNICAST_ADDRESS_XP;

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef  IP_ADAPTER_UNICAST_ADDRESS_LH IP_ADAPTER_UNICAST_ADDRESS;
typedef  IP_ADAPTER_UNICAST_ADDRESS_LH *PIP_ADAPTER_UNICAST_ADDRESS;
#elif (NTDDI_VERSION >= NTDDI_WINXP)
typedef  IP_ADAPTER_UNICAST_ADDRESS_XP IP_ADAPTER_UNICAST_ADDRESS;
typedef  IP_ADAPTER_UNICAST_ADDRESS_XP *PIP_ADAPTER_UNICAST_ADDRESS;
#endif

//
// Bit values of IP_ADAPTER_UNICAST_ADDRESS Flags field.
//
#define IP_ADAPTER_ADDRESS_DNS_ELIGIBLE 0x01
#define IP_ADAPTER_ADDRESS_TRANSIENT    0x02

typedef struct _IP_ADAPTER_ANYCAST_ADDRESS_XP {
    union {
        ULONGLONG Alignment;
        struct {
            ULONG Length;
            DWORD Flags;
        };
    };
    struct _IP_ADAPTER_ANYCAST_ADDRESS_XP *Next;
    SOCKET_ADDRESS Address;
} IP_ADAPTER_ANYCAST_ADDRESS_XP, *PIP_ADAPTER_ANYCAST_ADDRESS_XP;
#if (NTDDI_VERSION >= NTDDI_WINXP)
typedef IP_ADAPTER_ANYCAST_ADDRESS_XP IP_ADAPTER_ANYCAST_ADDRESS;
typedef IP_ADAPTER_ANYCAST_ADDRESS_XP *PIP_ADAPTER_ANYCAST_ADDRESS;
#endif

typedef struct _IP_ADAPTER_MULTICAST_ADDRESS_XP {
    union {
        ULONGLONG Alignment;
        struct {
            ULONG Length;
            DWORD Flags;
        };
    };
    struct _IP_ADAPTER_MULTICAST_ADDRESS_XP *Next;
    SOCKET_ADDRESS Address;
} IP_ADAPTER_MULTICAST_ADDRESS_XP, *PIP_ADAPTER_MULTICAST_ADDRESS_XP;
#if (NTDDI_VERSION >= NTDDI_WINXP)
typedef IP_ADAPTER_MULTICAST_ADDRESS_XP IP_ADAPTER_MULTICAST_ADDRESS;
typedef IP_ADAPTER_MULTICAST_ADDRESS_XP *PIP_ADAPTER_MULTICAST_ADDRESS;
#endif

typedef struct _IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    union {
        ULONGLONG Alignment;
        struct {
            ULONG Length;
            DWORD Reserved;
        };
    };
    struct _IP_ADAPTER_DNS_SERVER_ADDRESS_XP *Next;
    SOCKET_ADDRESS Address;
} IP_ADAPTER_DNS_SERVER_ADDRESS_XP, *PIP_ADAPTER_DNS_SERVER_ADDRESS_XP;
#if (NTDDI_VERSION >= NTDDI_WINXP)
typedef IP_ADAPTER_DNS_SERVER_ADDRESS_XP IP_ADAPTER_DNS_SERVER_ADDRESS;
typedef IP_ADAPTER_DNS_SERVER_ADDRESS_XP *PIP_ADAPTER_DNS_SERVER_ADDRESS;
#endif

typedef struct _IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    union {
        ULONGLONG Alignment;
        struct {
            ULONG Length;
            DWORD Reserved;
        };
    };
    struct _IP_ADAPTER_WINS_SERVER_ADDRESS_LH *Next;
    SOCKET_ADDRESS Address;
} IP_ADAPTER_WINS_SERVER_ADDRESS_LH, *PIP_ADAPTER_WINS_SERVER_ADDRESS_LH;
#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef IP_ADAPTER_WINS_SERVER_ADDRESS_LH IP_ADAPTER_WINS_SERVER_ADDRESS;
typedef IP_ADAPTER_WINS_SERVER_ADDRESS_LH *PIP_ADAPTER_WINS_SERVER_ADDRESS;
#endif


typedef struct _IP_ADAPTER_GATEWAY_ADDRESS_LH {
    union {
        ULONGLONG Alignment;
        struct {
            ULONG Length;
            DWORD Reserved;
        };
    };
    struct _IP_ADAPTER_GATEWAY_ADDRESS_LH *Next;
    SOCKET_ADDRESS Address;
} IP_ADAPTER_GATEWAY_ADDRESS_LH, *PIP_ADAPTER_GATEWAY_ADDRESS_LH;
#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef IP_ADAPTER_GATEWAY_ADDRESS_LH IP_ADAPTER_GATEWAY_ADDRESS;
typedef IP_ADAPTER_GATEWAY_ADDRESS_LH *PIP_ADAPTER_GATEWAY_ADDRESS;
#endif

typedef struct _IP_ADAPTER_PREFIX_XP {
    union {
        ULONGLONG Alignment;
        struct {
            ULONG Length;
            DWORD Flags;
        };
    };
    struct _IP_ADAPTER_PREFIX_XP *Next;
    SOCKET_ADDRESS Address;
    ULONG PrefixLength;
} IP_ADAPTER_PREFIX_XP, *PIP_ADAPTER_PREFIX_XP;
#if (NTDDI_VERSION >= NTDDI_WINXP)
typedef IP_ADAPTER_PREFIX_XP IP_ADAPTER_PREFIX;
typedef IP_ADAPTER_PREFIX_XP *PIP_ADAPTER_PREFIX;
#endif

typedef struct _IP_ADAPTER_DNS_SUFFIX {
    struct _IP_ADAPTER_DNS_SUFFIX *Next;
    WCHAR String[MAX_DNS_SUFFIX_STRING_LENGTH];
}IP_ADAPTER_DNS_SUFFIX, *PIP_ADAPTER_DNS_SUFFIX;

//
// Bit values of IP_ADAPTER_ADDRESSES Flags field.
//
#define IP_ADAPTER_DDNS_ENABLED               0x00000001
#define IP_ADAPTER_REGISTER_ADAPTER_SUFFIX    0x00000002
#define IP_ADAPTER_DHCP_ENABLED               0x00000004
#define IP_ADAPTER_RECEIVE_ONLY               0x00000008
#define IP_ADAPTER_NO_MULTICAST               0x00000010
#define IP_ADAPTER_IPV6_OTHER_STATEFUL_CONFIG 0x00000020
#define IP_ADAPTER_NETBIOS_OVER_TCPIP_ENABLED 0x00000040
#define IP_ADAPTER_IPV4_ENABLED               0x00000080
#define IP_ADAPTER_IPV6_ENABLED               0x00000100
#define IP_ADAPTER_IPV6_MANAGE_ADDRESS_CONFIG 0x00000200

typedef struct _IP_ADAPTER_ADDRESSES_LH {
    union {
        ULONGLONG Alignment;
        struct {
            ULONG Length;
            IF_INDEX IfIndex;
        };
    };
    struct _IP_ADAPTER_ADDRESSES_LH *Next;
    PCHAR AdapterName;
    PIP_ADAPTER_UNICAST_ADDRESS_LH FirstUnicastAddress;
    PIP_ADAPTER_ANYCAST_ADDRESS_XP FirstAnycastAddress;
    PIP_ADAPTER_MULTICAST_ADDRESS_XP FirstMulticastAddress;
    PIP_ADAPTER_DNS_SERVER_ADDRESS_XP FirstDnsServerAddress;
    PWCHAR DnsSuffix;
    PWCHAR Description;
    PWCHAR FriendlyName;
    BYTE PhysicalAddress[MAX_ADAPTER_ADDRESS_LENGTH];
    ULONG PhysicalAddressLength;
    union {
        ULONG Flags;
        struct {
            ULONG DdnsEnabled : 1;
            ULONG RegisterAdapterSuffix : 1;
            ULONG Dhcpv4Enabled : 1;
            ULONG ReceiveOnly : 1;
            ULONG NoMulticast : 1;
            ULONG Ipv6OtherStatefulConfig : 1;
            ULONG NetbiosOverTcpipEnabled : 1;
            ULONG Ipv4Enabled : 1;
            ULONG Ipv6Enabled : 1;
            ULONG Ipv6ManagedAddressConfigurationSupported : 1;
        };
    };
    ULONG Mtu;
    IFTYPE IfType;
    IF_OPER_STATUS OperStatus;
    IF_INDEX Ipv6IfIndex;
    ULONG ZoneIndices[16];
    PIP_ADAPTER_PREFIX_XP FirstPrefix;

    ULONG64 TransmitLinkSpeed;
    ULONG64 ReceiveLinkSpeed;
    PIP_ADAPTER_WINS_SERVER_ADDRESS_LH FirstWinsServerAddress;
    PIP_ADAPTER_GATEWAY_ADDRESS_LH FirstGatewayAddress;
    ULONG Ipv4Metric;
    ULONG Ipv6Metric;
    IF_LUID Luid;
    SOCKET_ADDRESS Dhcpv4Server;
    NET_IF_COMPARTMENT_ID CompartmentId;
    NET_IF_NETWORK_GUID NetworkGuid;
    NET_IF_CONNECTION_TYPE ConnectionType;
    TUNNEL_TYPE TunnelType;
    //
    // DHCP v6 Info.
    //
    SOCKET_ADDRESS Dhcpv6Server;
    BYTE Dhcpv6ClientDuid[MAX_DHCPV6_DUID_LENGTH];
    ULONG Dhcpv6ClientDuidLength;
    ULONG Dhcpv6Iaid;
#if (NTDDI_VERSION >= NTDDI_VISTASP1)
    PIP_ADAPTER_DNS_SUFFIX FirstDnsSuffix;
#endif
} IP_ADAPTER_ADDRESSES_LH,
 *PIP_ADAPTER_ADDRESSES_LH;

typedef struct _IP_ADAPTER_ADDRESSES_XP {
    union {
        ULONGLONG Alignment;
        struct {
            ULONG Length;
            DWORD IfIndex;
        };
    };
    struct _IP_ADAPTER_ADDRESSES_XP *Next;
    PCHAR AdapterName;
    PIP_ADAPTER_UNICAST_ADDRESS_XP FirstUnicastAddress;
    PIP_ADAPTER_ANYCAST_ADDRESS_XP FirstAnycastAddress;
    PIP_ADAPTER_MULTICAST_ADDRESS_XP FirstMulticastAddress;
    PIP_ADAPTER_DNS_SERVER_ADDRESS_XP FirstDnsServerAddress;
    PWCHAR DnsSuffix;
    PWCHAR Description;
    PWCHAR FriendlyName;
    BYTE PhysicalAddress[MAX_ADAPTER_ADDRESS_LENGTH];
    DWORD PhysicalAddressLength;
    DWORD Flags;
    DWORD Mtu;
    DWORD IfType;
    IF_OPER_STATUS OperStatus;
    DWORD Ipv6IfIndex;
    DWORD ZoneIndices[16];
    PIP_ADAPTER_PREFIX_XP FirstPrefix;
} IP_ADAPTER_ADDRESSES_XP,
 *PIP_ADAPTER_ADDRESSES_XP;

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef  IP_ADAPTER_ADDRESSES_LH IP_ADAPTER_ADDRESSES;
typedef  IP_ADAPTER_ADDRESSES_LH *PIP_ADAPTER_ADDRESSES;
#elif (NTDDI_VERSION >= NTDDI_WINXP)
typedef  IP_ADAPTER_ADDRESSES_XP IP_ADAPTER_ADDRESSES;
typedef  IP_ADAPTER_ADDRESSES_XP *PIP_ADAPTER_ADDRESSES;
#else
//
// For platforms other platforms that are including
// the file but not using the types.
//
typedef  IP_ADAPTER_ADDRESSES_XP IP_ADAPTER_ADDRESSES;
typedef  IP_ADAPTER_ADDRESSES_XP *PIP_ADAPTER_ADDRESSES;
#endif


//
// Flags used as argument to GetAdaptersAddresses().
// "SKIP" flags are added when the default is to include the information.
// "INCLUDE" flags are added when the default is to skip the information.
//
#define GAA_FLAG_SKIP_UNICAST                   0x0001
#define GAA_FLAG_SKIP_ANYCAST                   0x0002
#define GAA_FLAG_SKIP_MULTICAST                 0x0004
#define GAA_FLAG_SKIP_DNS_SERVER                0x0008
#define GAA_FLAG_INCLUDE_PREFIX                 0x0010
#define GAA_FLAG_SKIP_FRIENDLY_NAME             0x0020
#define GAA_FLAG_INCLUDE_WINS_INFO              0x0040
#define GAA_FLAG_INCLUDE_GATEWAYS               0x0080
#define GAA_FLAG_INCLUDE_ALL_INTERFACES         0x0100
#define GAA_FLAG_INCLUDE_ALL_COMPARTMENTS       0x0200
#define GAA_FLAG_INCLUDE_TUNNEL_BINDINGORDER    0x0400
#define GAA_FLAG_SKIP_DNS_INFO                  0x0800

#endif /* _WINSOCK2API_ */

//
// IP_PER_ADAPTER_INFO - per-adapter IP information such as DNS server list.
//

typedef struct _IP_PER_ADAPTER_INFO_W2KSP1 {
    UINT AutoconfigEnabled;
    UINT AutoconfigActive;
    PIP_ADDR_STRING CurrentDnsServer;
    IP_ADDR_STRING DnsServerList;
} IP_PER_ADAPTER_INFO_W2KSP1, *PIP_PER_ADAPTER_INFO_W2KSP1;
#if (NTDDI_VERSION >= NTDDI_WIN2KSP1)
typedef  IP_PER_ADAPTER_INFO_W2KSP1 IP_PER_ADAPTER_INFO;
typedef  IP_PER_ADAPTER_INFO_W2KSP1 *PIP_PER_ADAPTER_INFO;
#endif

//
// FIXED_INFO - the set of IP-related information which does not depend on DHCP
//

typedef struct {
    char HostName[MAX_HOSTNAME_LEN + 4] ;
    char DomainName[MAX_DOMAIN_NAME_LEN + 4];
    PIP_ADDR_STRING CurrentDnsServer;
    IP_ADDR_STRING DnsServerList;
    UINT NodeType;
    char ScopeId[MAX_SCOPE_ID_LEN + 4];
    UINT EnableRouting;
    UINT EnableProxy;
    UINT EnableDns;
} FIXED_INFO_W2KSP1, *PFIXED_INFO_W2KSP1;
#if (NTDDI_VERSION >= NTDDI_WIN2KSP1)
typedef  FIXED_INFO_W2KSP1 FIXED_INFO;
typedef  FIXED_INFO_W2KSP1 *PFIXED_INFO;
#endif


#ifndef IP_INTERFACE_NAME_INFO_DEFINED
#define IP_INTERFACE_NAME_INFO_DEFINED

typedef struct ip_interface_name_info_w2ksp1 {
    ULONG           Index;      // Interface Index
    ULONG           MediaType;  // Interface Types - see ipifcons.h
    UCHAR           ConnectionType;
    UCHAR           AccessType;
    GUID            DeviceGuid; // Device GUID is the guid of the device
                                // that IP exposes
    GUID            InterfaceGuid; // Interface GUID, if not GUID_NULL is the
                                // GUID for the interface mapped to the device.
} IP_INTERFACE_NAME_INFO_W2KSP1, *PIP_INTERFACE_NAME_INFO_W2KSP1;

#if (NTDDI_VERSION >= NTDDI_WIN2KSP1)
typedef IP_INTERFACE_NAME_INFO_W2KSP1 IP_INTERFACE_NAME_INFO;
typedef IP_INTERFACE_NAME_INFO_W2KSP1 *PIP_INTERFACE_NAME_INFO;
#endif

#endif

#pragma warning(pop)

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif
