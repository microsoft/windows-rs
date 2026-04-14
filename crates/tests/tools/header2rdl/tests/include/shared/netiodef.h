/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    netiodef.h

Abstract:

    This module contains the basic definitions and structures for the
    network I/O subsystem.

Environment:

    user mode or kernel mode

--*/

#ifndef _NETIODEF_
#define _NETIODEF_
#pragma once

#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#pragma warning(push)
#pragma warning(disable:4200) // zero-sized array in struct/union
#pragma warning(disable:4214) // bit field types other than int
#pragma warning(disable:4201) // nameless struct/union


#include <ws2def.h>
#include <ws2ipdef.h>
#include <mswsockdef.h>
#include <mstcpip.h>
#include <nldef.h>

#ifndef ASSERT
#define ASSERT(exp) ((void) 0)
#define NETIODEF_DEFINED_ASSERT
#endif

#ifndef NETIO_INLINE
#if DBG
#define NETIO_INLINE __inline
#else
#define NETIO_INLINE __forceinline
#endif
#endif


#define IS_POWER_OF_TWO(x) (((x) != 0) && (((x) & ((x) - 1)) == 0))

#if defined(_NTDDK_) || defined(_NTRTL_)

#define IS_VALID_IPV4_MASK(x) \
    ((x.S_un.S_addr == (ULONG)-1) || IS_POWER_OF_TWO(~RtlUlongByteSwap(x.S_un.S_addr)+1))

#endif

#ifndef BYTE_ORDER

#define _LITTLE_ENDIAN  1234    // least-significant byte first (vax)
#define _BIG_ENDIAN      4321    // most-significant byte first (IBM, net)
#define _PDP_ENDIAN      3412    // LSB first in word, MSW first in long (pdp)

#if defined(vax) || defined(ns32000) || defined(sun386) || defined(MIPSEL) || \
    defined(BIT_ZERO_ON_RIGHT)
#define BYTE_ORDER _LITTLE_ENDIAN
#endif

#if defined(sel) || defined(pyr) || defined(mc68000) || defined(sparc) || \
    defined(is68k) || defined(tahoe) || defined(ibm032) || defined(ibm370) || \
    defined(MIPSEB) || defined (BIT_ZERO_ON_LEFT)
#define BYTE_ORDER _BIG_ENDIAN
#endif

#ifndef BYTE_ORDER      // still not defined
#if defined(u3b2) || defined(m68k)
#define BYTE_ORDER _BIG_ENDIAN
#endif

#if defined(i286) || defined(_X86_) || defined(_AMD64_) || defined(_IA64_) || defined(_ARM_) || defined(_ARM64_)
#define BYTE_ORDER _LITTLE_ENDIAN
#endif

#endif
#endif

#ifndef BYTE_ORDER
    // you must determine what the correct bit order is for your compiler
    UNDEFINED_BIT_ORDER;
#endif

//
// Opaque handles that are a specific number of bits wide.
//

typedef UINT8 HANDLE8, *PHANDLE8;
typedef UINT16 HANDLE16, *PHANDLE16;
typedef UINT32 HANDLE32, *PHANDLE32;

#ifndef __HANDLE64_DEFINED__
#define __HANDLE64_DEFINED__
typedef void* POINTER_64 HANDLE64;
typedef HANDLE64 *PHANDLE64;
#endif


#define MAKE_DD_DEVICE_NAME(x)  (L"\\Device\\" x)
#define MAKE_WIN_DEVICE_NAME(x) (L"\\\\.\\" x)


#include <ifdef.h>

#if defined(_MSC_VER)
#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4001) /* nonstandard extension : single line comment */
#pragma warning(disable:4200) /* nonstandard extension used : zero-sized array in struct/union */
#pragma warning(disable:4201) /* nonstandard extension used : nameless struct/union */
#pragma warning(disable:4214) /* nonstandard extension used : bit field types other then int */
#endif // defined(_MSC_VER)

//
// For buffer sizing convenience, define a maximum datalink address length.
//
#define DL_ADDRESS_LENGTH_MAXIMUM IF_MAX_PHYS_ADDRESS_LENGTH
#define DL_HEADER_LENGTH_MAXIMUM 64

#define DL_ETHERNET_HEADER_LENGTH_MAXIMUM \
    (sizeof(ETHERNET_HEADER) + sizeof(SNAP_HEADER))

#define DL_TUNNEL_HEADER_LENGTH_MAXIMUM \
    max(sizeof(IPV4_HEADER), sizeof(IPV6_HEADER))


//
// DL_ADDRESS_TYPE
//
// Define datalink layer address types.
//
#ifndef _DEFINE_DL_ADDRESS_TYPE_
#define _DEFINE_DL_ADDRESS_TYPE_
typedef enum {
    DlUnicast,
    DlMulticast,
    DlBroadcast
} DL_ADDRESS_TYPE, *PDL_ADDRESS_TYPE;
#endif // _DEFINE_DL_ADDRESS_TYPE_

//
// DL_OUI
//
// Define Organizationally Unique Identifier.
//

union _DL_OUI {
    UINT8 Byte[3];
    struct {                    // 1st byte.  0bxxxxxxLG.
        UINT8 Group : 1;        // least significant bit.
        UINT8 Local : 1;
    };
};

typedef union _DL_OUI DL_OUI, *PDL_OUI;


//
// DL_EI48
//
// Define Extension Identifier for EUI-48 Addresses.
//

union _DL_EI48 {
    UINT8 Byte[3];
};

typedef union _DL_EI48 DL_EI48, *PDL_EI48;

//
// DL_EUI48
//
// Define EUI-48 Address.
//

union _DL_EUI48 {
    UINT8 Byte[6];
    struct {
        DL_OUI Oui;
        DL_EI48 Ei48;
    };
};

typedef union _DL_EUI48 DL_EUI48, *PDL_EUI48;

C_ASSERT(DL_ADDRESS_LENGTH_MAXIMUM >= sizeof(DL_EUI48));

#define EUI48_BROADCAST_INIT { 0xff, 0xff, 0xff, 0xff, 0xff, 0xff }

extern CONST DL_EUI48 eui48_broadcast;


//
// DL_EI64
//
// Define Extension Identifier for EUI-64 Addresses.
//

union _DL_EI64 {
    UINT8 Byte[5];
};

typedef union _DL_EI64 DL_EI64, *PDL_EI64;


//
// DL_EUI64
//
// Define EUI-64 Address.
//

union _DL_EUI64 {
    UINT8 Byte[8];
    UINT64 Value;
    struct {
        DL_OUI Oui;
        union {
            DL_EI64 Ei64;
            struct {            // draft-templin-ngtrans-v6v4compat-02.
                UINT8 Type;     // Determines interpretation of rest.
                UINT8 Tse;      // Type specific extention.
                DL_EI48 Ei48;   // Encapsulated EUI-48 EI.
            };
        };
    };
};

typedef union _DL_EUI64 DL_EUI64, *PDL_EUI64;


//
// SNAP_HEADER
//
// Define a structure for the SNAP header.
//

typedef struct _SNAP_HEADER {
    UINT8 Dsap;
    UINT8 Ssap;
    UINT8 Control;
    UINT8 Oui[3];
    UINT16 Type;
} SNAP_HEADER, *PSNAP_HEADER;

#define SNAP_DSAP                   0xaa
#define SNAP_SSAP                   0xaa
#define SNAP_CONTROL                0x03
#define SNAP_OUI                    0x00

//
// Since TYPE values are identical to those used by ethernet...
//

#define SNAP_TYPE_ARP               ETHERNET_TYPE_ARP
#define SNAP_TYPE_IPV4              ETHERNET_TYPE_IPV4
#define SNAP_TYPE_IPV6              ETHERNET_TYPE_IPV6


//
// ETHERNET_HEADER
//
// Define a common structure for the ethernet and IEEE 802 headers.
// NOTE: An IEEE 802 header is followed by a SNAP header.
//

typedef struct _ETHERNET_HEADER {
    DL_EUI48 Destination;
    DL_EUI48 Source;
    union {
        UINT16 Type;            // Ethernet
        UINT16 Length;          // IEEE 802
    };
} ETHERNET_HEADER, *PETHERNET_HEADER;

#define ETH_LENGTH_OF_HEADER        14
#define ETH_LENGTH_OF_VLAN_HEADER   4
#define ETH_LENGTH_OF_SNAP_HEADER   8

C_ASSERT(ETH_LENGTH_OF_HEADER == sizeof(ETHERNET_HEADER));
C_ASSERT(ETH_LENGTH_OF_SNAP_HEADER == sizeof(SNAP_HEADER));

#define ETHERNET_TYPE_MINIMUM 0x0600 // Minimum valid Type value.
#define ETHERNET_TYPE_IPV4 0x0800
#define ETHERNET_TYPE_ARP 0x0806
#define ETHERNET_TYPE_IPV6 0x86dd
#define ETHERNET_TYPE_802_1Q 0x8100
#define ETHERNET_TYPE_802_1AD 0x88a8

//
// VLAN_TAG
//

typedef struct _VLAN_TAG {
    union {
        UINT16 Tag;
        struct {
            UINT16 VID:12;            // least significant bit
            UINT16 CFI:1;
            UINT16 User_Priority:3;
        };
    };
    UINT16 Type;                     // this really is the type, since we come after the SNAP header
} VLAN_TAG;

C_ASSERT(ETH_LENGTH_OF_VLAN_HEADER == sizeof(VLAN_TAG));


__inline
DL_ADDRESS_TYPE
EthernetAddressType(
    _In_reads_(sizeof(DL_EUI48)) CONST UCHAR *Address
    )
{
    if (((PDL_EUI48) Address)->Oui.Group) {
        if (RtlEqualMemory(Address, &eui48_broadcast, sizeof(DL_EUI48))) {
            return DlBroadcast;
        }
        return DlMulticast;
    } else {
        return DlUnicast;
    }
}

#if defined (_MSC_VER)
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4001) /* nonstandard extension : single line comment */
#pragma warning(default:4200) /* nonstandard extension used : zero-sized array in struct/union */
#pragma warning(default:4201) /* nonstandard extension used : nameless struct/union */
#pragma warning(default:4214) /* nonstandard extension used : bit field types other then int */
#endif
#endif


typedef struct _ICMP_HEADER {
    UINT8 Type;         // Type of message (high bit zero for error messages).
    UINT8 Code;         // Type-specific differentiater.
    UINT16 Checksum;    // Calculated over ICMP message and IPvx pseudo-header.
} ICMP_HEADER, *PICMP_HEADER;

typedef struct _ICMP_MESSAGE {
    ICMP_HEADER Header;
    union {
        UINT32 Data32[1];       // Type-specific field.
        UINT16 Data16[2];       // Type-specific field.
        UINT8 Data8[4];         // Type-specific field.
    } Data;
} ICMP_MESSAGE, *PICMP_MESSAGE;


#if defined(_MSC_VER)
#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4001) /* nonstandard extension : single line comment */
#pragma warning(disable:4200) /* nonstandard extension used : zero-sized array in struct/union */
#pragma warning(disable:4201) /* nonstandard extension used : nameless struct/union */
#pragma warning(disable:4214) /* nonstandard extension used : bit field types other then int */
#endif // defined(_MSC_VER)

//
// IPV4_HEADER
//
// Define the structure of an IPv4 header.
// The field names match those in section 3.1 of RFC 791.
// RFC 2474 redefines type of service to the 6 bit DSCP value. RFC 2780 and
// 3168 redefine the unused 2 bits in the traffic class octet as used by
// ECN.
//

typedef struct _IPV4_HEADER {
    union {
        UINT8 VersionAndHeaderLength;   // Version and header length.
        struct {
            UINT8 HeaderLength : 4;
            UINT8 Version : 4;
        };
    };
    union {
        UINT8 TypeOfServiceAndEcnField; // Type of service & ECN (RFC 3168).
        struct {
            UINT8 EcnField : 2;
            UINT8 TypeOfService : 6;
        };
    };
    UINT16 TotalLength;                 // Total length of datagram.
    UINT16 Identification;
    union {
        UINT16 FlagsAndOffset;          // Flags and fragment offset.
        struct {
            UINT16 DontUse1 : 5;        // High bits of fragment offset.
            UINT16 MoreFragments : 1;
            UINT16 DontFragment : 1;
            UINT16 Reserved : 1;
            UINT16 DontUse2 : 8;        // Low bits of fragment offset.
        };
    };
    UINT8 TimeToLive;
    UINT8 Protocol;
    UINT16 HeaderChecksum;
    IN_ADDR SourceAddress;
    IN_ADDR DestinationAddress;
} IPV4_HEADER, *PIPV4_HEADER;

#define ip4_hdr          _IPV4_HEADER
#define ip4_ver_hlen     VersionAndHeaderLength
#define ip4_ver          Version
#define ip4_hlen         HeaderLength
#define ip4_tos          TypeOfService
#define ip4_len          TotalLength
#define ip4_id           Identification
#define ip4_flags_offset FlagsAndOffset
#define ip4_flags        Flags
#define ip4_offset       FragmentOffset
#define ip4_ttl          TimeToLive
#define ip4_protocol     Protocol
#define ip4_xsum         HeaderChecksum
#define ip4_src          SourceAddress
#define ip4_dest         DestinationAddress

C_ASSERT(sizeof(IPV4_HEADER) == 20);

#define IP_VER_MASK 0xF0 // Version is high 4 bits of ip4_ver_hlen.
#define IPV4_VERSION 4
#define IPV4_VERSION_BYTE (IPV4_VERSION << 4)
#define IPV4_DEFAULT_VERHLEN ((IPV4_VERSION_BYTE) | \
                              (sizeof(IPV4_HEADER) / sizeof(UINT32)))

#define MAX_IPV4_PACKET     65535
#define MAX_IPV4_PAYLOAD    (MAX_IPV4_PACKET - sizeof(IPV4_HEADER))

#define MAX_IPV4_HLEN       60

#define IPV4_MINIMUM_MTU    576
#define IPV4_MINIMUM_ULMTU  (IPV4_MINIMUM_MTU - sizeof(IPV4_HEADER))

//
// The maximum and minimum values for the lower limit of the value the MTU can
// have during PMTU discovery.
//
// The minimum value was chosen to be a reasonable limit for real-world 
// scenarios, while at the same time minimizing security risks.
//

#define IPV4_MIN_MINIMUM_MTU 352
#define IPV4_MAX_MINIMUM_MTU IPV4_MINIMUM_MTU

//
// TODO: remove UNALIGNED qualifier once NetGetDataBuffer takes an
// alignment argument.
//
__inline
UCHAR
Ip4HeaderLengthInBytes(
    _In_ CONST UNALIGNED IPV4_HEADER *Header
    )
{
    return (Header->HeaderLength << 2);
}

//
// Maximum length of IP options.
//
#define MAX_IP_OPTIONS_LENGTH ((0xF * sizeof(UINT32)) - sizeof(IPV4_HEADER))

#define SIZEOF_IP_OPT_ROUTING_HEADER 3

#define SIZEOF_IP_OPT_TIMESTAMP_HEADER 4

#define SIZEOF_IP_OPT_SECURITY 11
#define SIZEOF_IP_OPT_STREAMIDENTIFIER 4
#define SIZEOF_IP_OPT_ROUTERALERT 4

#define IP4_OFF_MASK 0xff1f // Mask out offset from FlagsAndOffset.

#if defined(_NTDDK_) || defined(_NTRTL_)
//
// Only include this definition if RtlUshortByteSwap is defined.
//

__inline
UINT16
Ip4FragmentOffset(
    _In_ CONST UNALIGNED IPV4_HEADER *Header
    )
{
    return RtlUshortByteSwap(Header->FlagsAndOffset & IP4_OFF_MASK) << 3;
}

#endif

//
// IPV4_OPTION_HEADER
//
typedef struct _IPV4_OPTION_HEADER {
    union {
        UINT8 OptionType;   // Option type.
        struct {
            UINT8 OptionNumber : 5;
            UINT8 OptionClass : 2;
            UINT8 CopiedFlag : 1;
        };
    };
    UINT8 OptionLength; // Length in bytes, starting from the OptionType field.
} IPV4_OPTION_HEADER, *PIPV4_OPTION_HEADER;

C_ASSERT(sizeof(IPV4_OPTION_HEADER) == 2);

#ifndef IP_EXPORT_INCLUDED
typedef enum {
    IP_OPT_EOL              = 0x00, // End of list option.
    IP_OPT_NOP              = 0x01, // No operation.
    IP_OPT_SECURITY         = 0x82, // Security option (RFC 1108).
    IP_OPT_LSRR             = 0x83, // Loose source route.
    IP_OPT_TS               = 0x44, // Timestamp.
    IP_OPT_RR               = 0x07, // Record route.
    IP_OPT_SSRR             = 0x89, // Struct source route.
    IP_OPT_SID              = 0x88, // Stream ID (obsolete).
    IP_OPT_ROUTER_ALERT     = 0x94, // Router alert option (RFC 2113).
    IP_OPT_MULTIDEST        = 0x95, // Multi-destination option (RFC 1770).
} IPV4_OPTION_TYPE;

#else // IP_EXPORT_INCLUDED
typedef ULONG IPV4_OPTION_TYPE;

#endif // IP_EXPORT_INCLUDED

typedef struct _IPV4_TIMESTAMP_OPTION {
#ifdef __cplusplus
    IPV4_OPTION_HEADER OptionHeader;
#else
    IPV4_OPTION_HEADER;
#endif
    UINT8 Pointer;
    union {
        UINT8 FlagsOverflow;
        struct {
           UINT8 Flags : 4;
           UINT8 Overflow : 4;
        };
    };
} IPV4_TIMESTAMP_OPTION, *PIPV4_TIMESTAMP_OPTION;

typedef enum {
    IP_OPTION_TIMESTAMP_ONLY = 0,
    IP_OPTION_TIMESTAMP_ADDRESS = 1,
    IP_OPTION_TIMESTAMP_SPECIFIC_ADDRESS = 3
} IP_OPTION_TIMESTAMP_FLAGS;

//
// Structure to interpret the IPv4 loose and strict source routing options as
// a routing header similar to IPv6.
//
typedef struct _IPV4_ROUTING_HEADER {
#ifdef __cplusplus
    IPV4_OPTION_HEADER OptionHeader;
#else
    IPV4_OPTION_HEADER;
#endif
    UINT8 Pointer;
} IPV4_ROUTING_HEADER;
typedef IPV4_ROUTING_HEADER UNALIGNED *PIPV4_ROUTING_HEADER;

//
// ICMPV4_HEADER
//
// Define the structure of an ICMPv4 header.
//

typedef ICMP_HEADER ICMPV4_HEADER, *PICMPV4_HEADER;

typedef ICMP_MESSAGE ICMPV4_MESSAGE, *PICMPV4_MESSAGE;

#define icmp4_hdr       _ICMPV4_MESSAGE
#define icmp4_type      Header.Type
#define icmp4_code      Header.Code
#define icmp4_cksum     Header.Checksum
#define icmp4_un_data32 Data32
#define icmp4_un_data16 Data16
#define icmp4_un_data8  Data8
#define icmp4_dataun    Data

#define icmp4_data32    icmp4_dataun.icmp4_un_data32
#define icmp4_data16    icmp4_dataun.icmp4_un_data16
#define icmp4_data8     icmp4_dataun.icmp4_un_data8
#define icmp4_pptr      icmp4_data32[0]  // Parameter problem.
#define icmp4_mtu       icmp4_data32[0]  // Packet too big.
#define icmp4_id        icmp4_data16[0]  // Echo request/reply.
#define icmp4_seq       icmp4_data16[1]  // Echo request/reply.
#define icmp4_maxdelay  icmp4_data16[0]  // Multicast group membership.

//
// ICMP4_UNREACH_CODE
//
// Define codes for the ICMPv4 destination-unreachable error message.
//

typedef enum {
    ICMP4_UNREACH_NET                =  0,
    ICMP4_UNREACH_HOST               =  1,
    ICMP4_UNREACH_PROTOCOL           =  2,
    ICMP4_UNREACH_PORT               =  3,
    ICMP4_UNREACH_FRAG_NEEDED        =  4,
    ICMP4_UNREACH_SOURCEROUTE_FAILED =  5,
    ICMP4_UNREACH_NET_UNKNOWN        =  6,
    ICMP4_UNREACH_HOST_UNKNOWN       =  7,
    ICMP4_UNREACH_ISOLATED           =  8,
    ICMP4_UNREACH_NET_ADMIN          =  9,
    ICMP4_UNREACH_HOST_ADMIN         = 10,
    ICMP4_UNREACH_NET_TOS            = 11,
    ICMP4_UNREACH_HOST_TOS           = 12,
    ICMP4_UNREACH_ADMIN              = 13,
} ICMP4_UNREACH_CODE, *PICMP4_UNREACH_CODE;

//
// ICMP4_TIME_EXCEED_CODE
//
// Define codes for the ICMPv4 time-exceeded error message.
//

typedef enum {
    ICMP4_TIME_EXCEED_TRANSIT        = 0, // TTL == 0 in transit
    ICMP4_TIME_EXCEED_REASSEMBLY     = 1, // Reassembly time out
} ICMP4_TIME_EXCEED_CODE, *PICMP4_TIME_EXCEED_CODE;

//
// Define a structure for IPv4 router solicitation.
//
typedef struct _ICMPV4_ROUTER_SOLICIT {
    ICMPV4_MESSAGE RsHeader;
} ICMPV4_ROUTER_SOLICIT, *PICMPV4_ROUTER_SOLICIT;

#define RsType      RsHeader.icmp4_type
#define RsCode      RsHeader.icmp4_code
#define RsCksum     RsHeader.icmp4_cksum
#define RsReserved  RsHeader.icmp4_data32[0]

typedef struct _ICMPV4_ROUTER_ADVERT_HEADER {
    ICMPV4_MESSAGE RaHeader;
    //
    // Could be followed by a list of router addresses.
    //
} ICMPV4_ROUTER_ADVERT_HEADER, *PICMPV4_ROUTER_ADVERT_HEADER;

#define RaType          RaHeader.icmp4_type
#define RaCode          RaHeader.icmp4_code
#define RaCksum         RaHeader.icmp4_cksum
#define RaNumAddr       RaHeader.icmp4_data8[0]
#define RaAddrEntrySize RaHeader.icmp4_data8[1]
#define RaAddrLifetime  RaHeader.icmp4_data16[1]

typedef struct _ICMPV4_ROUTER_ADVERT_ENTRY {
    IN_ADDR RouterAdvertAddr;
    LONG PreferenceLevel;
} ICMPV4_ROUTER_ADVERT_ENTRY, *PICMPV4_ROUTER_ADVERT_ENTRY;

#define ICMPV4_INVALID_PREFERENCE_LEVEL 0x80000000

typedef struct _ICMPV4_TIMESTAMP_MESSAGE {
    ICMPV4_MESSAGE Header;
    UINT32 OriginateTimestamp;
    UINT32 ReceiveTimestamp;
    UINT32 TransmitTimestamp;
} ICMPV4_TIMESTAMP_MESSAGE, *PICMPV4_TIMESTAMP_MESSAGE;

typedef struct _ICMPV4_ADDRESS_MASK_MESSAGE {
    ICMPV4_MESSAGE Header;
    UINT32 AddressMask;
} ICMPV4_ADDRESS_MASK_MESSAGE, *PICMPV4_ADDRESS_MASK_MESSAGE;

#define icmp4_ts_type         Header.icmp4_type
#define icmp4_ts_code         Header.icmp4_code
#define icmp4_ts_cksum        Header.icmp4_cksum
#define icmp4_ts_id           Header.icmp4_id
#define icmp4_ts_seq          Header.icmp4_seq
#define icmp4_ts_originate    OriginateTimestamp
#define icmp4_ts_receive      ReceiveTimestamp
#define icmp4_ts_transmit     TransmitTimestamp

//
// ARP_HEADER
//
// Define the structure of an ARP header.
// The field names are derived from those in RFC 826.
//

typedef struct _ARP_HEADER {
    USHORT HardwareAddressSpace;
    USHORT ProtocolAddressSpace;
    UCHAR HardwareAddressLength;
    UCHAR ProtocolAddressLength;
    USHORT Opcode;
    UCHAR SenderHardwareAddress[0];
    //
    // Followed by:
    //  SenderProtocolAddress,
    //  TargetHardwareAddress,
    //  TargetProtocolAddress.
    //
} ARP_HEADER, *PARP_HEADER;

//
// Opcode values in the ARP header.
// http://www.iana.org/assignments/arp-parameters
// is the authoritative source of these.
//

typedef enum {
    ARP_REQUEST = 1,
    ARP_RESPONSE = 2
} ARP_OPCODE;

//
// HardwareAddressSpace values in the ARP header.
// http://www.iana.org/assignments/arp-parameters
// is the authoritative source of these.
//

typedef enum {
    ARP_HW_ENET = 1,
    ARP_HW_802 = 6
} ARP_HARDWARE_TYPE;

//
// Types of IGMP messages.
//
#define IGMP_QUERY_TYPE           0x11
#define IGMP_VERSION1_REPORT_TYPE 0x12
#define IGMP_VERSION2_REPORT_TYPE 0x16
#define IGMP_LEAVE_GROUP_TYPE     0x17
#define IGMP_VERSION3_REPORT_TYPE 0x22

//
// IGMP_HEADER
//
// Define the structure of an IGMPv1/IGMPv2 header.
//
typedef struct _IGMP_HEADER {
    union {
        struct {
            UINT8 Type : 4;
            UINT8 Version : 4;
        };
        UINT8 VersionType;
    };
    union {
        UINT8 Reserved;         // IGMPv1.
        UINT8 MaxRespTime;      // IGMPv2.
        UINT8 Code;             // DVMRP.
    };
    UINT16 Checksum;
    IN_ADDR MulticastAddress;
} IGMP_HEADER, *PIGMP_HEADER;

typedef enum {
    IGMP_MAX_RESP_CODE_TYPE_NORMAL = 0,
    IGMP_MAX_RESP_CODE_TYPE_FLOAT
} IGMP_MAX_RESP_CODE_TYPE;

//
// IGMPV3_QUERY_HEADER
//
// Define the structure of an IGMPv3 query header. The fields names are derived
// from RFC 3376.
//
typedef struct _IGMPV3_QUERY_HEADER {
    UINT8 Type;
    union {
        UINT8 MaxRespCode;
        struct {
            UINT8 MaxRespCodeMantissa : 4;
            UINT8 MaxRespCodeExponent : 3;
            UINT8 MaxRespCodeType : 1;
        };
    };
    UINT16 Checksum;
    IN_ADDR MulticastAddress;
    UINT8 QuerierRobustnessVariable : 3;
    UINT8 SuppressRouterSideProcessing : 1;
    UINT8 Reserved : 4;
    union {
        UINT8 QueriersQueryInterfaceCode;
        struct {
            UINT8 QQCMantissa : 4;
            UINT8 QQCExponent : 3;
            UINT8 QQCType : 1;
        };
    };
    UINT16 SourceCount;
} IGMPV3_QUERY_HEADER, *PIGMPV3_QUERY_HEADER;

//
// IGMPV3_REPORT_RECORD_HEADER
//
// Defines the structure of the record header within a IGMPv3 report message
// (RFC 3376).
//
typedef struct _IGMPV3_REPORT_RECORD_HEADER {
    UINT8 Type;
    UINT8 AuxillaryDataLength;
    UINT16 SourceCount;
    IN_ADDR MulticastAddress;
} IGMPV3_REPORT_RECORD_HEADER, *PIGMPV3_REPORT_RECORD_HEADER;

//
// IGMPV3_HEADER
//
// Define the structure of an IGMPv3 report message header (RFC 3376).
//
typedef struct _IGMPV3_REPORT_HEADER_ {
    UINT8 Type;
    UINT8 Reserved;
    UINT16 Checksum;
    UINT16 Reserved2;
    UINT16 RecordCount;
} IGMPV3_REPORT_HEADER, *PIGMPV3_REPORT_HEADER;

#if defined (_MSC_VER)
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4001) /* nonstandard extension : single line comment */
#pragma warning(default:4200) /* nonstandard extension used : zero-sized array in struct/union */
#pragma warning(default:4201) /* nonstandard extension used : nameless struct/union */
#pragma warning(default:4214) /* nonstandard extension used : bit field types other then int */
#endif
#endif


//
// IPV6_HEADER
//
// The structure for an IPv6 header.
// RAW socket applications, packetization layer modules, and
// network-layer services all need access to this structure.
//
typedef struct _IPV6_HEADER {
    union {
        UINT32 VersionClassFlow; // 4 bits Version, 8 Traffic Class, 20 Flow Label.
        struct { // Convenience structure to access Version field only.
            UINT32 : 4;
            UINT32 Version : 4;
            UINT32 : 24;
        };
    };
    UINT16 PayloadLength;   // Zero indicates Jumbo Payload hop-by-hop option.
    UINT8 NextHeader;       // Values are superset of IPv4's Protocol field.
    UINT8 HopLimit;
    IN6_ADDR SourceAddress;
    IN6_ADDR DestinationAddress;
} IPV6_HEADER, *PIPV6_HEADER;

//
// Defines for RFC 3542 compatibility.
//
#define ip6_hdr _IPV6_HEADER
#define ip6_flow VersionClassFlow
#define ip6_plen PayloadLength
#define ip6_nxt NextHeader
#define ip6_hops HopLimit
#define ip6_hlim HopLimit
#define ip6_src SourceAddress
#define ip6_dst DestinationAddress

//
// Useful constants for working with various fields in the IPv6 header.
//
// NOTE: We keep the Version, Traffic Class and Flow Label fields as a single
// NOTE: 32 bit value (VersionClassFlow) in network byte order (big-endian).
// NOTE: Since NT is little-endian, this means all loads/stores to/from this
// NOTE: field need to be byte swapped.
//
#define IP_VER_MASK 0xF0        // Version is high 4 bits of VersionClassFlow.
#define IPV6_VERSION 0x60       // This is 6 << 4.

//
// RFC 2474 redefines traffic class to the 6 bit DSCP value. RFC 2780 and
// 3168 redefine the unused 2 bits in the traffic class octet as used by
// ECN.
//
#define IPV6_TRAFFIC_CLASS_MASK 0x0000C00F  // 0x0FC00000 (after byte swap).
#define IPV6_FULL_TRAFFIC_CLASS_MASK 0x0000F00F  // 0x0FF00000 (after byte swap).
#define IPV6_ECN_MASK 0x00003000            // 0x00300000 (after byte swap).
#define IPV6_FLOW_LABEL_MASK 0xFFFF0F00     // 0x000FFFFF (after byte swap).
#define MAX_IPV6_PAYLOAD 65535
#define MAX_IPV6_PACKET (MAX_IPV6_PAYLOAD + sizeof(IPV6_HEADER))

#define IPV6_ECN_SHIFT 12  // Bits to shift to recover ECN field.

#define IPV6_MINIMUM_MTU 1280
#define IPV6_MINIMUM_ULMTU (IPV6_MINIMUM_MTU - sizeof(IPV6_HEADER))

#define IPV6_TRAFFIC_CLASS(VersionClassFlow) \
    ((UCHAR)((((VersionClassFlow) & IPV6_TRAFFIC_CLASS_MASK) >> 12) + \
             (((VersionClassFlow) & IPV6_TRAFFIC_CLASS_MASK) << 4)))

#define IPV6_FULL_TRAFFIC_CLASS(VersionClassFlow) \
    ((UCHAR)((((VersionClassFlow) & IPV6_FULL_TRAFFIC_CLASS_MASK) >> 12) + \
             (((VersionClassFlow) & IPV6_FULL_TRAFFIC_CLASS_MASK) << 4)))

//
// IPV6_FRAGMENT_HEADER
//
// The structure for an IPv6 Fragment Header.
// RAW socket applications and network-layer services all need
// access to this structure.
//
typedef struct _IPV6_FRAGMENT_HEADER {
    UINT8 NextHeader;           // Next Header.
    UINT8 Reserved;             // Reserved field.
    union {
        struct {
            UINT16 DontUse1 : 8;
            UINT16 MoreFragments : 1; // Least significant bit.
            UINT16 ReservedBits : 2;
            UINT16 DontUse2 : 5;
        };
        UINT16 OffsetAndFlags;
    };
    UINT32 Id;                  // Identification.
} IPV6_FRAGMENT_HEADER, *PIPV6_FRAGMENT_HEADER;

C_ASSERT(sizeof(IPV6_FRAGMENT_HEADER) == 8);

//
// Defines for RFC 3542 compatibility.
//
#define ip6_frag _IPV6_FRAGMENT_HEADER
#define ip6f_nxt NextHeader
#define ip6f_reserved Reserved
#define ip6f_offlg OffsetAndFlags
#define ip6f_ident Id

//
// Useful constants from RFC 3542 for working with the ip6f_offlg field.
// These are in network byte order.
//
#define IP6F_OFF_MASK       0xf8ff  // Mask out offset from _offlg.
#define IP6F_RESERVED_MASK  0x0600  // Reserved bits in ip6f_offlg.
#define IP6F_MORE_FRAG      0x0100  // More-fragments flag.

#if defined(_NTDDK_) || defined(_NTRTL_)
//
// Only include this definition if RtlUshortByteSwap is defined.
//

__inline
UINT16
Ip6FragmentOffset(
    _In_ CONST UNALIGNED IPV6_FRAGMENT_HEADER *Header
    )
{
    return RtlUshortByteSwap(Header->OffsetAndFlags & IP6F_OFF_MASK);
}

#endif

//
// IPV6_EXTENSION_HEADER
//
typedef struct _IPV6_EXTENSION_HEADER {
    UINT8 NextHeader;           // Next header.
    UINT8 Length;               // In 8-byte units, not counting first 8.
} IPV6_EXTENSION_HEADER, *PIPV6_EXTENSION_HEADER;

#define EXT_LEN_UNIT 8          // 8-byte units used for extension hdr length.

#define IPV6_EXTENSION_HEADER_LENGTH(Blocks) ((Blocks + 1) * EXT_LEN_UNIT)
#define MAX_IPV6_EXTENSION_HEADER_LENGTH IPV6_EXTENSION_HEADER_LENGTH(0xFF)
#define IPV6_EXTENSION_HEADER_BLOCKS(Length) ((Length / EXT_LEN_UNIT) - 1)
#define IP_AUTHENTICATION_HEADER_LENGTH(Blocks) ((Blocks + 2) * 4)
#define IP_AUTHENTICATION_HEADER_BLOCKS(Length) \
    (((Length + sizeof(AUTHENTICATION_HEADER)) / 4) - 2)

#define IPV6_ROUTER_ALERT_LENGTH IPV6_EXTENSION_HEADER_LENGTH(0)

//
// Defines for RFC 3542 compatibility.
//
#define ip6_hbh  _IPV6_EXTENSION_HEADER
#define ip6h_nxt NextHeader
#define ip6h_len Length
#define ip6_dest _IPV6_EXTENSION_HEADER
#define ip6d_nxt NextHeader
#define ip6d_len Length


//
// IPV6_OPTION_HEADER
//

typedef struct _IPV6_OPTION_HEADER {
    UINT8 Type;
    UINT8 DataLength;           // In bytes, not counting two for the header.
} IPV6_OPTION_HEADER, *PIPV6_OPTION_HEADER;

typedef enum {
    IP6OPT_PAD1 = 0x00,         // 00 0 00000.  Single byte pad.
    IP6OPT_PADN = 0x01,         // 00 0 00001.  Multiple byte pad.
    IP6OPT_TUNNEL_LIMIT = 0x04, // 00 0 00100.
    IP6OPT_ROUTER_ALERT = 0x05, // 00 0 00101.  Router alert.
    IP6OPT_JUMBO = 0xc2,        // 11 0 00010.  Jumbogram, greater than 64KB.
    IP6OPT_NSAP_ADDR = 0xc3,    // 11 0 00011.
} IPV6_OPTION_TYPE, *PIPV6_OPTION_TYPE;

#define IP6OPT_TYPE(Type) ((Type) & 0xc0)
#define IP6OPT_TYPE_SKIP 0x00
#define IP6OPT_TYPE_DISCARD 0x40
#define IP6OPT_TYPE_FORCEICMP 0x80
#define IP6OPT_TYPE_ICMP 0xc0

#define IP6OPT_MUTABLE 0x20
#define IP6OPT_ISMUTABLE(Type) (((Type) & IP6OPT_MUTABLE) != 0)

typedef struct _IPV6_OPTION_JUMBOGRAM {
    IPV6_OPTION_HEADER Header;
    UINT8 JumbogramLength[4];
} IPV6_OPTION_JUMBOGRAM, *PIPV6_OPTION_JUMBOGRAM;

#define ip6_opt_jumbo  _IPV6_OPTION_JUMBOGRAM
#define ip6oj_type Header.Type
#define ip6oj_len Header.DataLength
#define ip6oj_jumbo_len JumbogramLength


typedef struct _IPV6_OPTION_ROUTER_ALERT {
    IPV6_OPTION_HEADER Header;
    UINT8 Value[2];
} IPV6_OPTION_ROUTER_ALERT, *PIPV6_OPTION_ROUTER_ALERT;

#define ip6_opt_router _IPV6_OPTION_ROUTER_ALERT
#define ip6or_type Header.Type
#define ip6or_len Header.DataLength
#define ip6or_value Value

#define SIZEOF_IPV6_ROUTERALERT IPV6_EXTENSION_HEADER_LENGTH(0)


typedef _Struct_size_bytes_(_Inexpressible_(Length)) struct _IPV6_ROUTING_HEADER {
    UCHAR NextHeader;
    UCHAR Length;           // In 8-byte units, not counting first 8.
    UCHAR RoutingType;
    UCHAR SegmentsLeft;     // Number of nodes still left to be visited.
    UCHAR Reserved[4];      // Not a u_int to avoid alignment.
} IPV6_ROUTING_HEADER, *PIPV6_ROUTING_HEADER;

//
// Defines for RFC 3542 compatibility.
//
#define ip6_rthdr _IPV6_ROUTING_HEADER
#define ip6r_nxt NextHeader
#define ip6r_len Length
#define ip6r_type RoutingType
#define ip6r_segleft SegmentsLeft

//
// ICMPV6_HEADER
//
// struct icmp6_hdr is the RFC 3542 structure for an ICMPv6 header.
// RAW socket applications and network-layer services all need
// access to this structure.
//

typedef ICMP_HEADER ICMPV6_HEADER, *PICMPV6_HEADER;

typedef ICMP_MESSAGE ICMPV6_MESSAGE, *PICMPV6_MESSAGE;

//
// Defines for RFC 3542 compatibility.
//
#define icmp6_hdr _ICMPV6_MESSAGE
#define icmp6_type Header.Type
#define icmp6_code Header.Code
#define icmp6_cksum Header.Checksum
#define icmp6_un_data32 Data32
#define icmp6_un_data16 Data16
#define icmp6_un_data8 Data8
#define icmp6_dataun Data

#define icmp6_data32 icmp6_dataun.icmp6_un_data32
#define icmp6_data16 icmp6_dataun.icmp6_un_data16
#define icmp6_data8 icmp6_dataun.icmp6_un_data8
#define icmp6_pptr icmp6_data32[0]      // Parameter problem.
#define icmp6_mtu icmp6_data32[0]       // Packet too big.
#define icmp6_id icmp6_data16[0]        // Echo request/reply.
#define icmp6_seq icmp6_data16[1]       // Echo request/reply.
#define icmp6_maxdelay icmp6_data16[0]  // Multicast group membership.

#define ICMP6_INFOMSG_MASK 0x80         // All informational messages.

#define ICMP6_DST_UNREACH_NOROUTE 0     // No route to destination.
#define ICMP6_DST_UNREACH_ADMIN 1       // Communication with destination
                                        // administratively prohibited.
#define ICMP6_DST_UNREACH_BEYONDSCOPE 2 // Beyond scope of source address.
#define ICMP6_DST_UNREACH_ADDR 3        // Address unreachable.
#define ICMP6_DST_UNREACH_NOPORT 4      // Bad port.

#define ICMP6_TIME_EXCEED_TRANSIT 0     // Hop Limit == 0 in transit.
#define ICMP6_TIME_EXCEED_REASSEMBLY 1  // Reassembly time out.

#define ICMP6_PARAMPROB_HEADER 0        // Erroneous header field.
#define ICMP6_PARAMPROB_NEXTHEADER 1    // Unrecognized Next Header.
#define ICMP6_PARAMPROB_OPTION 2        // Unrecognized IPv6 option.
#define ICMP6_PARAMPROB_FIRSTFRAGMENT 3 // IPv6 First Fragment is incomplete.

#define ICMPV6_ECHO_REQUEST_FLAG_REVERSE 0x1

//
// ND_ROUTER_SOLICIT_HEADER
//
// Define a structure for an IPv6 Router Solicitation header.
//

typedef struct nd_router_solicit {
    ICMPV6_MESSAGE nd_rs_hdr;
    //
    // Could be followed by options.
    //
} ND_ROUTER_SOLICIT_HEADER, *PND_ROUTER_SOLICIT_HEADER;

#define nd_rs_type nd_rs_hdr.icmp6_type
#define nd_rs_code nd_rs_hdr.icmp6_code
#define nd_rs_cksum nd_rs_hdr.icmp6_cksum
#define nd_rs_reserved nd_rs_hdr.icmp6_data32[0]


//
// ND_ROUTER_ADVERT_HEADER
//
// Define a structure for an IPv6 Router Advertisement header.
//

typedef struct nd_router_advert {
    ICMPV6_MESSAGE nd_ra_hdr;
    UINT32 nd_ra_reachable;     // Reachable Time.
    UINT32 nd_ra_retransmit;    // Retransmit Timer.
    //
    // Could be followed by options.
    //
} ND_ROUTER_ADVERT_HEADER, *PND_ROUTER_ADVERT_HEADER;

#define nd_ra_type nd_ra_hdr.icmp6_type
#define nd_ra_code nd_ra_hdr.icmp6_code
#define nd_ra_cksum nd_ra_hdr.icmp6_cksum
#define nd_ra_curhoplimit nd_ra_hdr.icmp6_data8[0]
#define nd_ra_flags_reserved nd_ra_hdr.icmp6_data8[1]
#define ND_RA_FLAG_MANAGED 0x80
#define ND_RA_FLAG_OTHER 0x40
#define ND_RA_FLAG_HOME_AGENT 0x20
#define ND_RA_FLAG_PREFERENCE 0x18
#define nd_ra_router_lifetime nd_ra_hdr.icmp6_data16[1]


//
// IPV6_ROUTER_ADVERTISEMENT_FLAGS
//
// Define flags included in an IPv6 Router Advertisement message.
//

typedef union _IPV6_ROUTER_ADVERTISEMENT_FLAGS {
    struct {
        UINT8 Reserved : 3;     // Least significant bits.
        UINT8 Preference : 2;
        UINT8 HomeAgent : 1;
        UINT8 OtherStatefulConfiguration : 1;
        UINT8 ManagedAddressConfiguration : 1;
    };
    UINT8 Value;
} IPV6_ROUTER_ADVERTISEMENT_FLAGS, *PIPV6_ROUTER_ADVERTISEMENT_FLAGS;


//
// ND_NEIGHBOR_SOLICIT_HEADER
//
// Define a structure for an IPv6 Neighbor Solicitation header.
//

typedef struct nd_neighbor_solicit {
    ICMPV6_MESSAGE nd_ns_hdr;
    IN6_ADDR nd_ns_target;      // Target Address.
    //
    // Could be followed by options.
    //
} ND_NEIGHBOR_SOLICIT_HEADER, *PND_NEIGHBOR_SOLICIT_HEADER;

#define nd_ns_type nd_ns_hdr.icmp6_type
#define nd_ns_code nd_ns_hdr.icmp6_code
#define nd_ns_cksum nd_ns_hdr.icmp6_cksum
#define nd_ns_reserved nd_ns_hdr.icmp6_data32[0]


//
// ND_NEIGHBOR_ADVERT_HEADER
//
// Define a structure for an IPv6 Neighbor Advertisement header.
//

typedef struct nd_neighbor_advert {
    ICMPV6_MESSAGE nd_na_hdr;
    IN6_ADDR nd_na_target;      // Target Address.
    //
    // Could be followed by options.
    //
} ND_NEIGHBOR_ADVERT_HEADER, *PND_NEIGHBOR_ADVERT_HEADER;

#define nd_na_type nd_na_hdr.icmp6_type
#define nd_na_code nd_na_hdr.icmp6_code
#define nd_na_cksum nd_na_hdr.icmp6_cksum
#define nd_na_flags_reserved nd_na_hdr.icmp6_data32[0]

#if (BYTE_ORDER == _BIG_ENDIAN)
#define ND_NA_FLAG_ROUTER 0x80000000
#define ND_NA_FLAG_SOLICITED 0x40000000
#define ND_NA_FLAG_OVERRIDE 0x20000000
#else  // BYTE_ORDER == _LITTLE_ENDIAN
#define ND_NA_FLAG_ROUTER 0x00000080
#define ND_NA_FLAG_SOLICITED 0x00000040
#define ND_NA_FLAG_OVERRIDE 0x00000020
#endif


//
// IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS
//
// Define flags included in an IPv6 Neighbor Advertisement message.
//

typedef union _IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {
    struct {
        UINT8 Reserved1 : 5;    // Least significant bits.
        UINT8 Override : 1;
        UINT8 Solicited : 1;
        UINT8 Router : 1;
        UINT8 Reserved2[3];
    };
    UINT32 Value;
} IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS, *PIPV6_NEIGHBOR_ADVERTISEMENT_FLAGS;


//
// ND_REDIRECT_HEADER
//
// Define a structure for an IPv6 Router Redirect header.
//

typedef struct nd_redirect {
    ICMPV6_MESSAGE nd_rd_hdr;
    IN6_ADDR nd_rd_target;      // Target Address.
    IN6_ADDR nd_rd_dst;         // Destination Address.
    //
    // Could be followed by options.
    //
} ND_REDIRECT_HEADER, *PND_REDIRECT_HEADER;

#define nd_rd_type nd_rd_hdr.icmp6_type
#define nd_rd_code nd_rd_hdr.icmp6_code
#define nd_rd_cksum nd_rd_hdr.icmp6_cksum
#define nd_rd_reserved nd_rd_hdr.icmp6_data32[0]


//
// ND_OPTION_HDR
//
// Define a structure for neighbor discovery option header.
//

typedef struct nd_opt_hdr {
    UINT8 nd_opt_type;
    UINT8 nd_opt_len;           // In units of 8 octets.
    //
    // Followed by option specific data.
    //
} ND_OPTION_HDR, *PND_OPTION_HDR;


//
// ND_OPTION_TYPE
//
// Define values for neighbor discovery option type.
//

typedef enum {
    ND_OPT_SOURCE_LINKADDR = 1,
    ND_OPT_TARGET_LINKADDR = 2,
    ND_OPT_PREFIX_INFORMATION = 3,
    ND_OPT_REDIRECTED_HEADER = 4,
    ND_OPT_MTU = 5,
    ND_OPT_NBMA_SHORTCUT_LIMIT = 6,
    ND_OPT_ADVERTISEMENT_INTERVAL = 7,
    ND_OPT_HOME_AGENT_INFORMATION = 8,
    ND_OPT_SOURCE_ADDR_LIST = 9,
    ND_OPT_TARGET_ADDR_LIST = 10,
    ND_OPT_ROUTE_INFO = 24,
    ND_OPT_RDNSS = 25,
    ND_OPT_DNSSL = 31,
    ND_OPT_PREF64 = 38,
} ND_OPTION_TYPE, *PND_OPTION_TYPE;


//
// ND_OPTION_PREFIX_INFO
//
// Define a structure for a Prefix Information option.
//

typedef struct nd_opt_prefix_info {
    UINT8 nd_opt_pi_type;
    UINT8 nd_opt_pi_len;
    UINT8 nd_opt_pi_prefix_len;
    union {
        UINT8 nd_opt_pi_flags_reserved;
        struct {
            UINT8 Route : 1;    // Least significant bit.
            UINT8 Reserved1 : 3;
            UINT8 SitePrefix : 1;
            UINT8 RouterAddress : 1;
            UINT8 Autonomous : 1;
            UINT8 OnLink : 1;
        } Flags;
    };
    UINT32 nd_opt_pi_valid_time;
    UINT32 nd_opt_pi_preferred_time;
    union {
        UINT32 nd_opt_pi_reserved2;
        struct {
            UINT8 nd_opt_pi_reserved3[3];
            UINT8 nd_opt_pi_site_prefix_len;
        };
    };
    IN6_ADDR nd_opt_pi_prefix;
} ND_OPTION_PREFIX_INFO, *PND_OPTION_PREFIX_INFO;

#define ND_OPT_PI_FLAG_ONLINK 0x80
#define ND_OPT_PI_FLAG_AUTO 0x40
#define ND_OPT_PI_FLAG_ROUTER_ADDR 0x20
#define ND_OPT_PI_FLAG_SITE_PREFIX 0x10
#define ND_OPT_PI_FLAG_ROUTE 0x01


//
// ND_OPTION_RD_HDR
//
// Define a structure for a Redirected Header option.
//

typedef struct nd_opt_rd_hdr {
    UINT8 nd_opt_rh_type;
    UINT8 nd_opt_rh_len;
    UINT16 nd_opt_rh_reserved1;
    UINT32 nd_opt_rh_reserved2;
    //
    // Followed by IP header and data.
    //
} ND_OPTION_RD_HDR, *PND_OPTION_RD_HDR;


//
// ND_OPTION_MTU
//
// Define a structure for an MTU option.
//

typedef struct nd_opt_mtu {
    UINT8 nd_opt_mtu_type;
    UINT8 nd_opt_mtu_len;
    UINT16 nd_opt_mtu_reserved;
    UINT32 nd_opt_mtu_mtu;
} ND_OPTION_MTU, *PND_OPTION_MTU;


//
// ND_OPTION_ROUTE_INFO
//
// Define a structure for a Route Information option.
//
// If PrefixLength is zero, then the Prefix field may be 0 bytes.
// If PrefixLength is <= 64, then the Prefix field may be 8 bytes.
// Otherwise the Prefix field is a full 16 bytes.
//

typedef struct nd_opt_route_info {
    UINT8 nd_opt_ri_type;
    UINT8 nd_opt_ri_len;
    UINT8 nd_opt_ri_prefix_len;
    union {
        UINT8 nd_opt_ri_flags_reserved;
        struct {
            UINT8 Reserved : 3; // Least significant bits.
            UINT8 Preference : 2;
        } Flags;
    };
    UINT32 nd_opt_ri_route_lifetime;
    IN6_ADDR nd_opt_ri_prefix;
} ND_OPTION_ROUTE_INFO, *PND_OPTION_ROUTE_INFO;

#define ND_OPT_RI_FLAG_PREFERENCE   0x18

//
// ND_OPTION_RDNSS
//
// Define a structure for a RDNSS option.
//

typedef struct nd_opt_rdnss {
    UINT8 nd_opt_rdnss_type;
    UINT8 nd_opt_rdnss_len;
    UINT16 nd_opt_rdnss_reserved;
    UINT32 nd_opt_rdnss_lifetime;
    //
    // Followed by one or more 128-bit IPv6 addresses of
    // the recursive DNS servers. The number of addresses
    // is determined by the Length field. i.e. the number
    // of addresses is equal to (nd_opt_rdnss_len - 1) / 2.
    //
} ND_OPTION_RDNSS, *PND_OPTION_RDNSS;

//
// Minimum length in bytes of the RDNSS option if there is
// at least one IPv6 address (3 * 8 = 24 octets).
//
#define ND_OPT_RDNSS_MIN_LEN 24

//
// ND_OPTION_DNSSL
//
// Define a structure for a DNSSL option.
//

typedef struct nd_opt_dnssl {
    UINT8 nd_opt_dnssl_type;
    UINT8 nd_opt_dnssl_len;
    UINT16 nd_opt_dnssl_reserved;
    UINT32 nd_opt_dnssl_lifetime;
    //
    // Followed by one or more domain names of DNS Search List
    // that MUST be encoded using the technique described in
    // Section 3.1 of [RFC1035]. By this technique, each domain
    // name is represented as a sequence of labels ending in
    // a zero octet, defined as domain name representation.
    // For more than one domain name, the corresponding
    // domain name representations are concatenated as they
    // are. Because the size of this field MUST be a multiple
    // of 8 octets, for the minimum multiple including the
    // domain name representations, the remaining octets other
    // than the encoding parts of the domain name representations
    // MUST be padded with zeros.
    //
} ND_OPTION_DNSSL, *PND_OPTION_DNSSL;

//
// Minimum length in bytes of the DNSSL option if there is
// atleast one suffix (2 * 8 = 16 octets).
//
#define ND_OPT_DNSSL_MIN_LEN 16

//
// ND_OPTION_PREF64
//
// Define a structure for a PREF64 option.
//

typedef struct nd_opt_pref64_info {
    UINT8 nd_opt_p64_type;
    UINT8 nd_opt_p64_len;
    union {
        UINT16 nd_opt_p64_lifetime_plc;
        struct {
            UINT16 nd_opt_p64_prefix_length_code : 3; // Least significant bits.
            UINT16 nd_opt_p64_scaled_lifetime : 13;
        };
    };
    UINT8 nd_opt_p64_prefix[12];
} ND_OPTION_PREF64, *PND_OPTION_PREF64;

//
// PREF64 prefix length encoding.
//

typedef enum {
    ND_OPT_PREF64_PREFIX_LENGTH_96 = 0,    // prefix length is 96 bits
    ND_OPT_PREF64_PREFIX_LENGTH_64,        // prefix length is 64 bits
    ND_OPT_PREF64_PREFIX_LENGTH_56,        // prefix length is 56 bits
    ND_OPT_PREF64_PREFIX_LENGTH_48,        // prefix length is 48 bits
    ND_OPT_PREF64_PREFIX_LENGTH_40,        // prefix length is 40 bits
    ND_OPT_PREF64_PREFIX_LENGTH_32,        // prefix length is 32 bits
} ND_OPT_PREF64_PREFIX_LENGTH_CODE;

//
// MLD_HEADER.
//
// Defines the structure of the MLD (version 1) header.
//
typedef struct _MLD_HEADER {
    ICMPV6_HEADER IcmpHeader;
    UINT16 MaxRespTime;
    UINT16 Reserved;
    IN6_ADDR MulticastAddress;
} MLD_HEADER, *PMLD_HEADER;

#define mld_type IcmpHeader.Type
#define mld_checksum IcmpHeader.Checksum

typedef enum {
    MLD_MAX_RESP_CODE_TYPE_NORMAL = 0,
    MLD_MAX_RESP_CODE_TYPE_FLOAT
} MLD_MAX_RESP_CODE_TYPE;

//
// MLDV2_QUERY_HEADER.
//
// Defines the structure of MLDv2 query header.
//
typedef struct _MLDV2_QUERY_HEADER {
    ICMPV6_HEADER IcmpHeader;
    union {
        UINT16 MaxRespCode;
        struct {
            UINT16 MaxRespCodeMantissaHi : 4;
            UINT16 MaxRespCodeExponent : 3;
            UINT16 MaxRespCodeType : 1;
            UINT16 MaxRespCodeMantissaLo : 8;
        };
    };
    UINT16 Reserved;
    IN6_ADDR MulticastAddress;
    UINT8 QuerierRobustnessVariable : 3;
    UINT8 SuppressRouterSideProcessing : 1;
    UINT8 QueryReserved : 4;
    union {
        UINT8 QueriersQueryInterfaceCode;
        struct {
            UINT8 QQCMantissa : 4;
            UINT8 QQCExponent : 3;
            UINT8 QQCType : 1;
        };
    };
    UINT16 SourceCount;
} MLDV2_QUERY_HEADER, *PMLDV2_QUERY_HEADER;

//
// MLDV2_REPORT_RECORD_HEADER.
//
// Defines the structure of the MLDv2 record header contained in a MLDv2
// report. There can be multiple record headers in a single MLDv2 report.
//
typedef struct _MLDV2_REPORT_RECORD_HEADER {
    UINT8 Type;
    UINT8 AuxillaryDataLength;
    UINT16 SourceCount;
    IN6_ADDR MulticastAddress;
} MLDV2_REPORT_RECORD_HEADER, *PMLDV2_REPORT_RECORD_HEADER;

//
// MLDV2_REPORT_HEADER.
//
// Defines the structure of a MLDv2 report.
//
typedef struct _MLDV2_REPORT_HEADER {
    ICMPV6_HEADER IcmpHeader;
    UINT16 Reserved;
    UINT16 RecordCount;
} MLDV2_REPORT_HEADER, *PMLDV2_REPORT_HEADER;

#if defined(_NTDDK_) || defined(_NTRTL_)

__inline
UINT32
Ipv6pGetVersionClassEcnFlow(
    IN UINT8 Class,
    IN UINT8 EcnField,
    IN UINT32 Flow
    )
{
    //
    // IPv6 Version, Traffic Class, ECN Field and Flow Label fields in host
    // byte order.
    //
    union {
        struct {
            UINT32 Flow : 20;
            UINT32 EcnField : 2;
            UINT32 Class : 6;
            UINT32 Version : 4; // Most significant bits.
        };
        UINT32 Value;
    } VersionClassEcnFlow = {0};

    VersionClassEcnFlow.Version = 6;
    VersionClassEcnFlow.Class = Class;
    VersionClassEcnFlow.EcnField = EcnField;
    VersionClassEcnFlow.Flow = Flow;
    if ((Flow != 0) && (VersionClassEcnFlow.Flow == 0)) {
        //
        // If LSBs (20 bits) of Flow are all 0s, use its MSBs.
        //
        VersionClassEcnFlow.Flow = (Flow >> 12);
    }
    return RtlUlongByteSwap(VersionClassEcnFlow.Value);
}

#endif

//
// Shared definitions for NAT64 / DNS64
//

#define IN6_EMBEDDEDV4_UOCTET_POSITION 8
#define IN6_EMBEDDEDV4_BITS_IN_BYTE 8

__inline
UINT32
In6IsEmbeddedV4AddrPrefixLengthValid(
    _In_ ULONG PrefixLength
    )
{
    //
    // Checks per draft-ietf-behave-address-format-06.
    //
    if ((PrefixLength % IN6_EMBEDDEDV4_BITS_IN_BYTE) != 0) {
        return FALSE;
    }

    if ((PrefixLength != 32) &&
        (PrefixLength != 40) &&
        (PrefixLength != 48) &&
        (PrefixLength != 56) &&
        (PrefixLength != 64) &&
        (PrefixLength != 96)) {
        return FALSE;
    }

    return TRUE;
}

_Success_(return)
__inline
UINT32
In6ExtractEmbeddedV4AddrFromV6(
    _In_ const IN6_ADDR *Ipv6Address,
    _In_ ULONG PrefixLength,
    _Out_ IN_ADDR *Ipv4Address
    )
{
    IN6_ADDR TmpAddress;

    if (!In6IsEmbeddedV4AddrPrefixLengthValid(PrefixLength)) {
        return FALSE;
    }

    if (PrefixLength < 96) {
        RtlCopyMemory(
            &TmpAddress,
            Ipv6Address,
            sizeof(IN6_ADDR));

        RtlMoveMemory(
            (((PUCHAR)&TmpAddress) + IN6_EMBEDDEDV4_UOCTET_POSITION),
            (((PUCHAR)&TmpAddress) + IN6_EMBEDDEDV4_UOCTET_POSITION + 1),
            sizeof(IN6_ADDR) - (IN6_EMBEDDEDV4_UOCTET_POSITION + 1));

        RtlCopyMemory(
            Ipv4Address,
            (((PUCHAR) &TmpAddress) + PrefixLength / IN6_EMBEDDEDV4_BITS_IN_BYTE),
            sizeof(IN_ADDR));
    } else {
        RtlCopyMemory(
            Ipv4Address,
            (((PUCHAR) Ipv6Address) + PrefixLength / IN6_EMBEDDEDV4_BITS_IN_BYTE),
            sizeof(IN_ADDR));
    }

    return TRUE;
}

_Success_(return)
__inline
UINT32
In6SetAddrV4Embedded(
    _In_ const IN6_ADDR *Prefix,
    _In_ ULONG PrefixLength,
    _In_ IN_ADDR *Ipv4Address,
    _Out_ IN6_ADDR *Ipv6Address
    )
{
    ULONG PrefixByteLength;

    if (!In6IsEmbeddedV4AddrPrefixLengthValid(PrefixLength)) {
        return FALSE;
    }

    PrefixByteLength = PrefixLength / IN6_EMBEDDEDV4_BITS_IN_BYTE;

    RtlCopyMemory(Ipv6Address, Prefix, PrefixByteLength);

    RtlCopyMemory(
        (((PUCHAR)Ipv6Address) + PrefixByteLength),
        Ipv4Address,
        sizeof(IN_ADDR));

    if (PrefixLength < 96) {
        RtlZeroMemory(
            (((PUCHAR)Ipv6Address) + PrefixByteLength + sizeof(IN_ADDR)),
            sizeof(IN6_ADDR) - (PrefixByteLength + sizeof(IN_ADDR)));
        RtlMoveMemory(
            (((PUCHAR)Ipv6Address) + IN6_EMBEDDEDV4_UOCTET_POSITION + 1),
            (((PUCHAR)Ipv6Address) + IN6_EMBEDDEDV4_UOCTET_POSITION),
            sizeof(IN6_ADDR) - (IN6_EMBEDDEDV4_UOCTET_POSITION + 1));

        *(((PUCHAR)Ipv6Address) + IN6_EMBEDDEDV4_UOCTET_POSITION) = 0;
    }

    return TRUE;
}


//
// SEQ_NUM
//
// Define a type for TCP sequence numbers.
//

typedef UINT32 SEQ_NUM, *PSEQ_NUM;

//
// TCP_HDR
//
// Define the structure of a TCP header.
//
// $REVIEW: We shouldn't have two different structures for representing
// a TCP header.  Can we get rid of this one?
//

#pragma pack(push, 1)

typedef struct tcp_hdr {
    UINT16 th_sport;
    UINT16 th_dport;
    SEQ_NUM th_seq;
    SEQ_NUM th_ack;
    UINT8 th_x2 : 4;
    UINT8 th_len : 4;
    UINT8 th_flags;
    UINT16 th_win;
    UINT16 th_sum;
    UINT16 th_urp;
} TCP_HDR
#if NDIS_RECEIVE_UNALIGNED
    ;
typedef TCP_HDR UNALIGNED *PTCP_HDR;
#else
    , *PTCP_HDR;
#endif

#pragma pack(pop)

//
// Define the maximum length of a TCP header with options.
//

#define TH_MAX_LEN  (0x0F << 2)

//
// Define the bits in TCP_HDR.th_flags.
//

#define TH_FIN 0x01
#define TH_SYN 0x02
#define TH_RST 0x04
#define TH_PSH 0x08
#define TH_ACK 0x10
#define TH_URG 0x20
#define TH_ECE 0x40
#define TH_CWR 0x80
#define TH_ALL \
    (TH_FIN | TH_SYN | TH_RST | TH_PSH | TH_ACK | TH_URG | TH_ECE | TH_CWR)
#define TH_SYN_ALL \
    (TH_FIN | TH_SYN | TH_RST | TH_ACK)

//
// Define the options appearing in a TCP header.
//

#define TH_OPT_EOL              0x00
#define TH_OPT_NOP              0x01
#define TH_OPT_MSS              0x02
#define TH_OPT_WS               0x03
#define TH_OPT_SACK_PERMITTED   0x04
#define TH_OPT_SACK             0x05
#define TH_OPT_TS               0x08
#define TH_OPT_FASTOPEN         0x22

#pragma pack(push, 1)

//
// TCP_OPT_MSS
//

typedef struct tcp_opt_mss {
    UINT8 Kind;
    UINT8 Length;
    UINT16 Mss;
} TCP_OPT_MSS;

//
// TCP_OPT_WS
//

typedef struct tcp_opt_ws {
    UINT8 Kind;
    UINT8 Length;
    UINT8 ShiftCnt;
} TCP_OPT_WS;

//
// TCP_OPT_SACK_PERMITTED
//

typedef struct tcp_opt_sack_permitted {
    UINT8 Kind;
    UINT8 Length;
} TCP_OPT_SACK_PERMITTED;

//
// TCP_OPT_SACK
//

typedef struct tcp_opt_sack {
    UINT8 Kind;
    UINT8 Length;
    struct tcp_opt_sack_block {
        SEQ_NUM Left;
        SEQ_NUM Right;
    } Block[0];
} TCP_OPT_SACK;

//
// TCP_OPT_TS
//

typedef struct tcp_opt_ts {
    UINT8 Kind;
    UINT8 Length;
    UINT32 Val;
    UINT32 EcR;
} TCP_OPT_TS;

//
// TCP_OPT_UNKNOWN
//

typedef struct tcp_opt_unknown {
    UINT8 Kind;
    UINT8 Length;
} TCP_OPT_UNKNOWN;

//
// TCP_OPT_FASTOPEN
//
// A fastopen cookie can be zero bytes (for a request) or 4 to 16 bytes.
//

typedef struct tcp_opt_fastopen {
    UINT8 Kind;
    UINT8 Length;
    UINT8 Cookie[0];
} TCP_OPT_FASTOPEN;

#pragma pack(pop)


#include <ifdef.h>

#if defined(_MSC_VER)
#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4001) /* nonstandard extension : single line comment */
#pragma warning(disable:4200) /* nonstandard extension used : zero-sized array in struct/union */
#pragma warning(disable:4201) /* nonstandard extension used : nameless struct/union */
#pragma warning(disable:4214) /* nonstandard extension used : bit field types other then int */
#endif // defined(_MSC_VER)

//
// DL_TUNNEL_ADDRESS
//
// Define IP in IP tunnel datalink address.
//

typedef struct DL_TUNNEL_ADDRESS {
    COMPARTMENT_ID CompartmentId;
    SCOPE_ID ScopeId;
    UCHAR IpAddress[0];
} DL_TUNNEL_ADDRESS, *PDL_TUNNEL_ADDRESS;

#define DL_SIZEOF_TUNNEL_ADDRESS(AddressBytes) \
    (FIELD_OFFSET(DL_TUNNEL_ADDRESS, IpAddress) + (AddressBytes))
#define DL_SIZEOF_IPV4_TUNNEL_ADDRESS \
    DL_SIZEOF_TUNNEL_ADDRESS(sizeof(IN_ADDR))
#define DL_SIZEOF_IPV6_TUNNEL_ADDRESS \
    DL_SIZEOF_TUNNEL_ADDRESS(sizeof(IN6_ADDR))

C_ASSERT(DL_ADDRESS_LENGTH_MAXIMUM >= DL_SIZEOF_IPV4_TUNNEL_ADDRESS);

C_ASSERT(DL_ADDRESS_LENGTH_MAXIMUM >= DL_SIZEOF_IPV6_TUNNEL_ADDRESS);


//
// Tunnel sub-type used to differentiate different tunnel
// technologies when the tunnel-type is TUNNEL_TYPE_OTHER
//
typedef enum _TUNNEL_SUB_TYPE {
    TUNNEL_SUB_TYPE_NONE = 0,
    TUNNEL_SUB_TYPE_CP = 1,
    TUNNEL_SUB_TYPE_IPTLS = 2,
    TUNNEL_SUB_TYPE_HA = 3
} TUNNEL_SUB_TYPE, *PTUNNEL_SUB_TYPE;


//
// DL_TEREDO_ADDRESS
//
// Define Teredo tunnel datalink address.
//

#pragma pack(push, 1)
typedef struct DL_TEREDO_ADDRESS {
    UINT8 Reserved[6];
    union {
        DL_EUI64 Eui64;
        struct {
            USHORT Flags;
            USHORT MappedPort;
            IN_ADDR MappedAddress;
        };
    };
} DL_TEREDO_ADDRESS, *PDL_TEREDO_ADDRESS;

typedef struct DL_TEREDO_ADDRESS_PRV {
    UINT8 Reserved[6];
    union {
        DL_EUI64 Eui64;
        struct {
            USHORT Flags;
            USHORT MappedPort;
            IN_ADDR MappedAddress;
            //
            // FL shunt
            //
            IN_ADDR LocalAddress;
            IF_INDEX InterfaceIndex;
            USHORT LocalPort;
            DL_EUI48 DlDestination;
        };
    };
} DL_TEREDO_ADDRESS_PRV, *PDL_TEREDO_ADDRESS_PRV;
#pragma pack(pop)

#pragma pack(push, 1)
typedef struct _IPTLS_METADATA {
    ULONGLONG SequenceNumber;
} IPTLS_METADATA,*PIPTLS_METADATA;
#pragma pack(pop)

C_ASSERT(DL_ADDRESS_LENGTH_MAXIMUM >= sizeof(DL_TEREDO_ADDRESS));


#include <ifdef.h>

#define NMR_REG_KEY_PATH L"\\Registry\\Machine\\System\\CurrentControlSet\\Control\\NMR\\providers"

typedef enum _NPI_MODULEID_TYPE {
    MIT_GUID = 1,
    MIT_IF_LUID,
} NPI_MODULEID_TYPE;


//
// Network Module Identifier.
// This type is persistable.
//
typedef struct _NPI_MODULEID {
    USHORT Length;

    NPI_MODULEID_TYPE Type;
#ifdef __midl
    [switch_type(NPI_MODULEID_TYPE), switch_is(Type)]
#endif
    union
    {
        //
        // Valid for MIT_GUID
        //
    #ifdef __midl
        [case(MIT_GUID)]
    #endif
        GUID Guid;

        //
        // Valid for MIT_IF_LUID
        // TODO: NET_IFLUID is not an "RPC'able" define yet.
        //

    #ifdef __midl
        [case(MIT_IF_LUID)]
    #endif
        LUID IfLuid;
    #ifdef __midl
        [default];
    #endif
    };
} NPI_MODULEID;
typedef CONST NPI_MODULEID *PNPI_MODULEID;

#ifndef __midl
__inline
BOOLEAN
NmrIsEqualNpiModuleId(
    _In_ PNPI_MODULEID ModuleId1,
    _In_ PNPI_MODULEID ModuleId2
    )
{
    if (ModuleId1->Type == ModuleId2->Type) {
        if (ModuleId1->Type == MIT_GUID) {
#ifdef __cplusplus
            return !!InlineIsEqualGUID(ModuleId1->Guid, ModuleId2->Guid);
#else
            return (BOOLEAN)InlineIsEqualGUID(&ModuleId1->Guid, &ModuleId2->Guid);
#endif
        } else if (ModuleId1->Type == MIT_IF_LUID) {
            return (BOOLEAN)RtlEqualMemory(&ModuleId1->IfLuid,
                                           &ModuleId2->IfLuid,
                                           sizeof(LUID));
        }
    }
    return FALSE;
}
#endif

//
// Network Programming Interface Identifier: an immutable, globally
// unique identifier for the programming interface.
//
typedef GUID NPIID;
typedef CONST NPIID *PNPIID;

extern CONST NPI_MODULEID NPI_MS_NMR_MODULEID;


//
// "Requested Packet Filter" Bits.
//

#define FL_PACKET_TYPE_FLAGS \
    (NDIS_PACKET_TYPE_ALL_MULTICAST | NDIS_PACKET_TYPE_PROMISCUOUS | \
     NDIS_PACKET_TYPE_NO_LOCAL)

extern CONST NPIID NPI_FRAMING_LAYER_ID;

extern CONST NPI_MODULEID NPI_MS_TUN_MODULEID;
extern CONST NPI_MODULEID NPI_MS_FL48_MODULEID;
extern CONST NPI_MODULEID NPI_MS_FL68_MODULEID;
extern CONST NPI_MODULEID NPI_MS_FL4L_MODULEID;
extern CONST NPI_MODULEID NPI_MS_FL6L_MODULEID;
extern CONST NPI_MODULEID NPI_MS_FL4T_MODULEID;
extern CONST NPI_MODULEID NPI_MS_FL6T_MODULEID;
extern CONST NPI_MODULEID NPI_MS_WANARPV4_MODULEID;
extern CONST NPI_MODULEID NPI_MS_WANARPV6_MODULEID;
extern CONST NPI_MODULEID NPI_MS_FL_WANARP_MODULEID;
extern CONST NPI_MODULEID NPI_MS_FLRDMA_MODULEID;

//
// TODO: remove this.
//
extern CONST NPI_MODULEID NPI_MS_NDIS_MODULEID;


extern CONST NPIID NPI_NETWORK_LAYER_ID;

extern CONST NPI_MODULEID NPI_MS_IPV6_MODULEID;
extern CONST NPI_MODULEID NPI_MS_IPV4_MODULEID;

typedef enum {
    FallbackIndexTcpFastopen,
    FallbackIndexMax
} FALLBACK_INDEX;



#ifdef NETIODEF_DEFINED_ASSERT
//
// ASSERT wasn't defined before, so undefine it now to restore the original 
// state.
//
#undef ASSERT
#endif

#pragma warning(pop)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _NETIODEF_
