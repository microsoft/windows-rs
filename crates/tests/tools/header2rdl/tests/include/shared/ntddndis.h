/*++ BUILD Version: 0001        // Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ntddndis.h

Abstract:

    This is the include file that defines constants and types for interfacing
    with network drivers.

Environment:

    User mode or Kernel mode

Notes:

    If you are writing kernel mode code, you should #include <ndis.h> instead
    of including this header directly.

    If you are writing user mode code, you can declare the NDIS contract
    version that you are targeting.  Available versions are:

        Version     First available in
        ------------------------------------------------------------------
        689         Windows 11, gallium release
        688         Windows 11, copper release
        687         Windows 11, nickel release
        686         Windows 11, cobalt release
        685         Windows 10, iron release / Windows Server 2022
        684         Windows 10, vibranium release
        683         Windows 10, version 1903
        682         Windows 10, version 1809
        681         Windows 10, version 1803
        680         Windows 10, version 1709
        670         Windows 10, version 1703
        660         Windows 10, version 1607 / Windows Server 2016
        651         Windows 10, version 1511
        650         Windows 10, version 1507
        640         Windows 8.1 / Windows Server 2012 R2
        630         Windows 8 / Windows Server 2012
        620         Windows 7 / Windows Server 2008 R2
        61          Windows Vista SP1 / Windows Server 2008 RTM
        60          Windows Vista RTM

    To declare the version, use a macro like this (where '630' is any version
    from the above table):

        #define UM_NDIS630

    Alternatively, add the definition to your compiler's options:

        -DUM_NDIS630=1

    Versions are cumulative; defining 630 will automatically include 620 and
    below.  There's no need to manually define them all.

--*/

#ifndef _NTDDNDIS_
#define _NTDDNDIS_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region App Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//disable warnings
#if _MSC_VER >= 1200

#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member

#endif

#pragma warning(disable:4201) // named type definition in parentheses

#ifdef __cplusplus
extern "C" {
#endif

#if ((NTDDI_VERSION >= NTDDI_VISTA) || NDIS_SUPPORT_NDIS6)
#include <ifdef.h>
#include <devpkey.h>
#include <pciprop.h>
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#define NDIS_INCLUDE_LEGACY_NAMES

#include <ndis/version.h>

#include <ndis/ioctltypes.h>

#define IOCTL_NDIS_QUERY_GLOBAL_STATS   _NDIS_CONTROL_CODE(0, METHOD_OUT_DIRECT)
#define IOCTL_NDIS_QUERY_ALL_STATS      _NDIS_CONTROL_CODE(1, METHOD_OUT_DIRECT)
#define IOCTL_NDIS_DO_PNP_OPERATION     _NDIS_CONTROL_CODE(2, METHOD_BUFFERED)
#define IOCTL_NDIS_QUERY_SELECTED_STATS _NDIS_CONTROL_CODE(3, METHOD_OUT_DIRECT)
#define IOCTL_NDIS_ENUMERATE_INTERFACES _NDIS_CONTROL_CODE(4, METHOD_BUFFERED)
#define IOCTL_NDIS_ADD_TDI_DEVICE       _NDIS_CONTROL_CODE(5, METHOD_BUFFERED)
#define IOCTL_NDIS_GET_LOG_DATA         _NDIS_CONTROL_CODE(7, METHOD_OUT_DIRECT)
#define IOCTL_NDIS_GET_VERSION          _NDIS_CONTROL_CODE(8, METHOD_BUFFERED)

#define IOCTL_NDIS_RESERVED1            _NDIS_CONTROL_CODE(9, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED2            _NDIS_CONTROL_CODE(0xA, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED3            _NDIS_CONTROL_CODE(0xB, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED4            _NDIS_CONTROL_CODE(0xC, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED5            CTL_CODE(FILE_DEVICE_PHYSICAL_NETCARD, 0xD, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_NDIS_RESERVED6            CTL_CODE(FILE_DEVICE_PHYSICAL_NETCARD, 0xE, METHOD_BUFFERED, FILE_WRITE_ACCESS)
#define IOCTL_NDIS_RESERVED7            _NDIS_CONTROL_CODE(0xF, METHOD_OUT_DIRECT)
#define IOCTL_NDIS_RESERVED8            _NDIS_CONTROL_CODE(0x10, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED9            _NDIS_CONTROL_CODE(0x11, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED10           _NDIS_CONTROL_CODE(0x12, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED11           _NDIS_CONTROL_CODE(0x13, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED12           _NDIS_CONTROL_CODE(0x14, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED13           _NDIS_CONTROL_CODE(0x15, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED14           _NDIS_CONTROL_CODE(0x16, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED15           _NDIS_CONTROL_CODE(0x17, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED16           _NDIS_CONTROL_CODE(0x18, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED17           _NDIS_CONTROL_CODE(0x19, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED18           _NDIS_CONTROL_CODE(0x1A, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED19           _NDIS_CONTROL_CODE(0x1B, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED20           _NDIS_CONTROL_CODE(0x1C, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED21           _NDIS_CONTROL_CODE(0x1D, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED22           _NDIS_CONTROL_CODE(0x1E, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED23           _NDIS_CONTROL_CODE(0x1F, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED24           _NDIS_CONTROL_CODE(0x20, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED25           _NDIS_CONTROL_CODE(0x21, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED26           _NDIS_CONTROL_CODE(0x22, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED27           _NDIS_CONTROL_CODE(0x23, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED28           _NDIS_CONTROL_CODE(0x24, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED29           _NDIS_CONTROL_CODE(0x25, METHOD_BUFFERED)
#define IOCTL_NDIS_RESERVED30           _NDIS_CONTROL_CODE(0x26, METHOD_BUFFERED)

//
// NtDeviceIoControlFile InputBuffer/OutputBuffer record structures for
// this device.
//

#include <ndis/oidtypes.h>

//
// IOCTL_NDIS_QUERY_ALL_STATS returns a sequence of these, packed
// together.  This structure is unaligned because not all statistics
// have a length that is a ULONG multiple.
//

typedef struct _NDIS_STATISTICS_VALUE
{
    NDIS_OID    Oid;
    ULONG       DataLength;
    _Field_size_bytes_(DataLength)
    UCHAR       Data[1];            // variable length
} NDIS_STATISTICS_VALUE;

typedef NDIS_STATISTICS_VALUE UNALIGNED *PNDIS_STATISTICS_VALUE;

#if ((NTDDI_VERSION >= NTDDI_VISTA) || NDIS_SUPPORT_NDIS6)
typedef _Struct_size_bytes_(Length) struct _NDIS_STATISTICS_VALUE_EX
{
    NDIS_OID    Oid;
    ULONG       DataLength;         // the length of the OID data
    ULONG       Length;             // the length of this instance of NDIS_STATISTICS_VALUE_EX
    _Field_size_bytes_(DataLength)
    UCHAR       Data[1];            // variable length
} NDIS_STATISTICS_VALUE_EX;

typedef NDIS_STATISTICS_VALUE_EX UNALIGNED *PNDIS_STATISTICS_VALUE_EX;
#endif // (NTDDI_VERSION >= NTDDI_VISTA)


//
// Structure used to define a self-contained variable data structure
//
typedef struct _NDIS_VAR_DATA_DESC
{
    USHORT      Length;         // # of octects of data
    USHORT      MaximumLength;  // # of octects available
    ULONG_PTR   Offset;         // Offset of data relative to the descriptor
} NDIS_VAR_DATA_DESC, *PNDIS_VAR_DATA_DESC;

#ifndef GUID_DEFINED
#include <guiddef.h>
#endif // !GUID_DEFINED

//
// NDIS Object Types used in NDIS_OBJECT_HEADER
//
#define NDIS_OBJECT_TYPE_DEFAULT                            0x80    // used when object type is implicit in the API call
#define NDIS_OBJECT_TYPE_MINIPORT_INIT_PARAMETERS           0x81    // used by NDIS in NDIS_MINIPORT_INIT_PARAMETERS
#define NDIS_OBJECT_TYPE_SG_DMA_DESCRIPTION                 0x83    // used by miniport drivers in NDIS_SG_DMA_DESCRIPTION
#define NDIS_OBJECT_TYPE_MINIPORT_INTERRUPT                 0x84    // used by miniport drivers in NDIS_MINIPORT_INTERRUPT_EX
#define NDIS_OBJECT_TYPE_DEVICE_OBJECT_ATTRIBUTES           0x85    // used by miniport or filter drivers in NDIS_DEVICE_OBJECT_ATTRIBUTES
#define NDIS_OBJECT_TYPE_BIND_PARAMETERS                    0x86    // used by NDIS in NDIS_BIND_PARAMETERS
#define NDIS_OBJECT_TYPE_OPEN_PARAMETERS                    0x87    // used by protocols in NDIS_OPEN_PARAMETERS
#define NDIS_OBJECT_TYPE_RSS_CAPABILITIES                   0x88    // used by miniport in NDIS_RECEIVE_SCALE_CAPABILITIES
#define NDIS_OBJECT_TYPE_RSS_PARAMETERS                     0x89    // used by miniport and protocol in NDIS_RECEIVE_SCALE_PARAMETERS
#define NDIS_OBJECT_TYPE_MINIPORT_DRIVER_CHARACTERISTICS    0x8A
#define NDIS_OBJECT_TYPE_FILTER_DRIVER_CHARACTERISTICS      0x8B
#define NDIS_OBJECT_TYPE_FILTER_PARTIAL_CHARACTERISTICS     0x8C
#define NDIS_OBJECT_TYPE_FILTER_ATTRIBUTES                  0x8D
#define NDIS_OBJECT_TYPE_CLIENT_CHIMNEY_OFFLOAD_GENERIC_CHARACTERISTICS     0x8E
#define NDIS_OBJECT_TYPE_PROVIDER_CHIMNEY_OFFLOAD_GENERIC_CHARACTERISTICS   0x8F
#define NDIS_OBJECT_TYPE_CO_PROTOCOL_CHARACTERISTICS        0x90
#define NDIS_OBJECT_TYPE_CO_MINIPORT_CHARACTERISTICS        0x91
#define NDIS_OBJECT_TYPE_MINIPORT_PNP_CHARACTERISTICS       0x92
#define NDIS_OBJECT_TYPE_CLIENT_CHIMNEY_OFFLOAD_CHARACTERISTICS     0x93
#define NDIS_OBJECT_TYPE_PROVIDER_CHIMNEY_OFFLOAD_CHARACTERISTICS   0x94
#define NDIS_OBJECT_TYPE_PROTOCOL_DRIVER_CHARACTERISTICS    0x95
#define NDIS_OBJECT_TYPE_REQUEST_EX                         0x96 // deprecated
#define NDIS_OBJECT_TYPE_TIMER_CHARACTERISTICS              0x97
#define NDIS_OBJECT_TYPE_STATUS_INDICATION                  0x98
#define NDIS_OBJECT_TYPE_FILTER_ATTACH_PARAMETERS           0x99
#define NDIS_OBJECT_TYPE_FILTER_PAUSE_PARAMETERS            0x9A
#define NDIS_OBJECT_TYPE_FILTER_RESTART_PARAMETERS          0x9B
#define NDIS_OBJECT_TYPE_PORT_CHARACTERISTICS               0x9C
#define NDIS_OBJECT_TYPE_PORT_STATE                         0x9D
#define NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_REGISTRATION_ATTRIBUTES       0x9E
#define NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_GENERAL_ATTRIBUTES            0x9F
#define NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_OFFLOAD_ATTRIBUTES            0xA0
#define NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_NATIVE_802_11_ATTRIBUTES      0xA1
#define NDIS_OBJECT_TYPE_RESTART_GENERAL_ATTRIBUTES                     0xA2
#define NDIS_OBJECT_TYPE_PROTOCOL_RESTART_PARAMETERS                    0xA3
#define NDIS_OBJECT_TYPE_MINIPORT_ADD_DEVICE_REGISTRATION_ATTRIBUTES    0xA4
#define NDIS_OBJECT_TYPE_CO_CALL_MANAGER_OPTIONAL_HANDLERS              0xA5
#define NDIS_OBJECT_TYPE_CO_CLIENT_OPTIONAL_HANDLERS                    0xA6
#define NDIS_OBJECT_TYPE_OFFLOAD                                        0xA7
#define NDIS_OBJECT_TYPE_OFFLOAD_ENCAPSULATION                          0xA8
#define NDIS_OBJECT_TYPE_CONFIGURATION_OBJECT                           0xA9
#define NDIS_OBJECT_TYPE_DRIVER_WRAPPER_OBJECT                          0xAA
#if (NDIS_SUPPORT_NDIS61)
#define NDIS_OBJECT_TYPE_HD_SPLIT_ATTRIBUTES                            0xAB
#endif // (NDIS_SUPPORT_NDIS61)
#define NDIS_OBJECT_TYPE_NSI_NETWORK_RW_STRUCT                          0xAC
#define NDIS_OBJECT_TYPE_NSI_COMPARTMENT_RW_STRUCT                      0xAD
#define NDIS_OBJECT_TYPE_NSI_INTERFACE_PERSIST_RW_STRUCT                0xAE
#if (NDIS_SUPPORT_NDIS61)
#define NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_HARDWARE_ASSIST_ATTRIBUTES    0xAF
#endif // (NDIS_SUPPORT_NDIS61)
#if (NDIS_SUPPORT_NDIS620)
#define NDIS_OBJECT_TYPE_SHARED_MEMORY_PROVIDER_CHARACTERISTICS         0xB0
#define NDIS_OBJECT_TYPE_RSS_PROCESSOR_INFO                             0xB1
#endif // (NDIS_SUPPORT_NDIS620)
#if (NDIS_SUPPORT_NDIS630)
#define NDIS_OBJECT_TYPE_NDK_PROVIDER_CHARACTERISTICS                   0xB2
#define NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_NDK_ATTRIBUTES                0xB3
#define NDIS_OBJECT_TYPE_MINIPORT_SS_CHARACTERISTICS                    0xB4
#define NDIS_OBJECT_TYPE_QOS_CAPABILITIES                               0xB5
#define NDIS_OBJECT_TYPE_QOS_PARAMETERS                                 0xB6
#define NDIS_OBJECT_TYPE_QOS_CLASSIFICATION_ELEMENT                     0xB7
#define NDIS_OBJECT_TYPE_SWITCH_OPTIONAL_HANDLERS                       0xB8

#endif // (NDIS_SUPPORT_NDIS630)


#if (NDIS_SUPPORT_NDIS650)
#define NDIS_OBJECT_TYPE_PD_TRANSMIT_QUEUE                              0xBE
#define NDIS_OBJECT_TYPE_PD_RECEIVE_QUEUE                               0xBF
#endif // (NDIS_SUPPORT_NDIS650)

#if (NDIS_SUPPORT_NDIS650)
#define NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_PACKET_DIRECT_ATTRIBUTES      0xC5
#endif // (NDIS_SUPPORT_NDIS650)

#if (NDIS_SUPPORT_NDIS670)
#define NDIS_OBJECT_TYPE_MINIPORT_DEVICE_POWER_NOTIFICATION             0xC6
#endif // (NDIS_SUPPORT_NDIS670)


#define NDIS_OBJECT_TYPE_RSS_PARAMETERS_V2                              0xC8    // used by miniport and protocol in NDIS_RECEIVE_SCALE_PARAMETERS_V2
#define NDIS_OBJECT_TYPE_RSS_SET_INDIRECTION_ENTRIES                    0xC9    // used by miniport and protocol in NDIS_RSS_SET_INDIRECTION_ENTRIES

#include <ndis/objectheader.h>

#if ((NTDDI_VERSION >= NTDDI_VISTA) || NDIS_SUPPORT_NDIS6)

//
// flags for NDIS_STATISTICS_INFO->SupportedStatistics structure
//
#define NDIS_STATISTICS_FLAGS_VALID_DIRECTED_FRAMES_RCV             0x00000001
#define NDIS_STATISTICS_FLAGS_VALID_MULTICAST_FRAMES_RCV            0x00000002
#define NDIS_STATISTICS_FLAGS_VALID_BROADCAST_FRAMES_RCV            0x00000004
#define NDIS_STATISTICS_FLAGS_VALID_BYTES_RCV                       0x00000008
#define NDIS_STATISTICS_FLAGS_VALID_RCV_DISCARDS                    0x00000010
#define NDIS_STATISTICS_FLAGS_VALID_RCV_ERROR                       0x00000020
#define NDIS_STATISTICS_FLAGS_VALID_DIRECTED_FRAMES_XMIT            0x00000040
#define NDIS_STATISTICS_FLAGS_VALID_MULTICAST_FRAMES_XMIT           0x00000080
#define NDIS_STATISTICS_FLAGS_VALID_BROADCAST_FRAMES_XMIT           0x00000100
#define NDIS_STATISTICS_FLAGS_VALID_BYTES_XMIT                      0x00000200
#define NDIS_STATISTICS_FLAGS_VALID_XMIT_ERROR                      0x00000400
#define NDIS_STATISTICS_FLAGS_VALID_XMIT_DISCARDS                   0x00008000
#define NDIS_STATISTICS_FLAGS_VALID_DIRECTED_BYTES_RCV              0x00010000
#define NDIS_STATISTICS_FLAGS_VALID_MULTICAST_BYTES_RCV             0x00020000
#define NDIS_STATISTICS_FLAGS_VALID_BROADCAST_BYTES_RCV             0x00040000
#define NDIS_STATISTICS_FLAGS_VALID_DIRECTED_BYTES_XMIT             0x00080000
#define NDIS_STATISTICS_FLAGS_VALID_MULTICAST_BYTES_XMIT            0x00100000
#define NDIS_STATISTICS_FLAGS_VALID_BROADCAST_BYTES_XMIT            0x00200000


#define NDIS_STATISTICS_INFO_REVISION_1  1

//
// structure used in OID_GEN_STATISTICS
//
typedef struct _NDIS_STATISTICS_INFO
{
    NDIS_OBJECT_HEADER          Header;
    ULONG                       SupportedStatistics;
    ULONG64                     ifInDiscards;           // OID_GEN_RCV_DISCARDS
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

}NDIS_STATISTICS_INFO, *PNDIS_STATISTICS_INFO;

#define NDIS_SIZEOF_STATISTICS_INFO_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_STATISTICS_INFO, ifHCOutBroadcastOctets)

#if (NDIS_SUPPORT_NDIS630)

//
// structure used in OID_TCP_RSC_STATISTICS
//
typedef struct _NDIS_RSC_STATISTICS_INFO
{
    NDIS_OBJECT_HEADER          Header;
    ULONG64                     CoalescedPkts;         // Total number of packets that were coalesced
    ULONG64                     CoalescedOctets;       // Total number of bytes that were coalesced
    ULONG64                     CoalesceEvents;        // Total number of coalescing events (i.e. packets formed from coalescing)
    ULONG64                     Aborts;                // Total number of Abort events

}NDIS_RSC_STATISTICS_INFO, *PNDIS_RSC_STATISTICS_INFO;

#define NDIS_RSC_STATISTICS_REVISION_1              1

#define NDIS_SIZEOF_RSC_STATISTICS_REVISION_1       \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RSC_STATISTICS_INFO, Aborts)

#endif // (NDIS_SUPPORT_NDIS630)

//
//  interrupt moderation structures and definitions
//

//
// enum value used in NDIS_INTERRUPT_MODERATION_PARAMETERS structure
//
typedef enum _NDIS_INTERRUPT_MODERATION
{
    NdisInterruptModerationUnknown,
    NdisInterruptModerationNotSupported,
    NdisInterruptModerationEnabled,
    NdisInterruptModerationDisabled
} NDIS_INTERRUPT_MODERATION, *PNDIS_INTERRUPT_MODERATION;

//
// Bits used in Flags parameter of NDIS_INTERRUPT_MODERATION_PARAMETERS structure:
//
#define NDIS_INTERRUPT_MODERATION_CHANGE_NEEDS_RESET            0x00000001
#define NDIS_INTERRUPT_MODERATION_CHANGE_NEEDS_REINITIALIZE     0x00000002


//
// structure used in OID_GEN_INTERRUPT_MODERATION
//

#define NDIS_INTERRUPT_MODERATION_PARAMETERS_REVISION_1    1

typedef struct _NDIS_INTERRUPT_MODERATION_PARAMETERS
{
    NDIS_OBJECT_HEADER Header;
    ULONG Flags;
    NDIS_INTERRUPT_MODERATION InterruptModeration;
}NDIS_INTERRUPT_MODERATION_PARAMETERS, *PNDIS_INTERRUPT_MODERATION_PARAMETERS;

#define NDIS_SIZEOF_INTERRUPT_MODERATION_PARAMETERS_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_INTERRUPT_MODERATION_PARAMETERS, InterruptModeration)

//
// structure used in OID_GEN_TIMEOUT_DPC_REQUEST_CAPABILITIES
//

#define NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES_REVISION_1  1

typedef struct _NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES
{
    NDIS_OBJECT_HEADER      Header;
    ULONG                   Flags;
    ULONG                   TimeoutArrayLength;
    ULONG                   TimeoutArray[1];
}NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES, *PNDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES;

#define NDIS_SIZEOF_TIMEOUT_DPC_REQUEST_CAPABILITIES_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES, TimeoutArray)

//
// PCI custom properties. used in OID_GEN_PCI_DEVICE_CUSTOM_PROPERTIES
//

#define NDIS_OBJECT_TYPE_PCI_DEVICE_CUSTOM_PROPERTIES_REVISION_1  1

#if ((NTDDI_VERSION >= NTDDI_WIN7) || NDIS_SUPPORT_NDIS620)
#define NDIS_OBJECT_TYPE_PCI_DEVICE_CUSTOM_PROPERTIES_REVISION_2  2
#endif

typedef struct _NDIS_PCI_DEVICE_CUSTOM_PROPERTIES
{
    NDIS_OBJECT_HEADER              Header;
    UINT32                          DeviceType;             // conventional, PCI-X, PCI-E, etc.
    UINT32                          CurrentSpeedAndMode;    // PCI_DEVICE_CONVENTIONAL_xx or PCIX_MODExxx. valid only for conventional and PCI-X devices
    UINT32                          CurrentPayloadSize;     // PCI_EXPRESS_MAX_PAYLOAD_SIZE, valid only for PCI-E devices
    UINT32                          MaxPayloadSize;         // PCI_EXPRESS_MAX_PAYLOAD_SIZE, valid only for PCI-E devices
    UINT32                          MaxReadRequestSize;     // PCI_EXPRESS_MAX_PAYLOAD_SIZE, valid only for PCI-E devices
    UINT32                          CurrentLinkSpeed;       // PCI_EXPRESS_LINK_SPEED_xxx. valid only for PCI-E devices
    UINT32                          CurrentLinkWidth;       // PCI_EXPRESS_LINK_WIDTH_xx. valid only for PCI-E devices
    UINT32                          MaxLinkSpeed;           // PCI_EXPRESS_LINK_SPEED_xxx. valid only for PCI-E devices
    UINT32                          MaxLinkWidth;           // PCI_EXPRESS_LINK_WIDTH_xx. valid only for PCI-E devices
#if ((NTDDI_VERSION >= NTDDI_WIN7) || NDIS_SUPPORT_NDIS620)
    UINT32                          PciExpressVersion;      // DevProp_PciExpressDevice_Spec_Version_xx, valid for PCI -E devices
    UINT32                          InterruptType;          // DevProp_PciDevice_InterruptType_xx
    UINT32                          MaxInterruptMessages;
#endif
} NDIS_PCI_DEVICE_CUSTOM_PROPERTIES, *PNDIS_PCI_DEVICE_CUSTOM_PROPERTIES;

#define NDIS_SIZEOF_PCI_DEVICE_CUSTOM_PROPERTIES_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_PCI_DEVICE_CUSTOM_PROPERTIES, MaxLinkWidth)

#if ((NTDDI_VERSION >= NTDDI_WIN7) || NDIS_SUPPORT_NDIS620)
#define NDIS_SIZEOF_PCI_DEVICE_CUSTOM_PROPERTIES_REVISION_2    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_PCI_DEVICE_CUSTOM_PROPERTIES, MaxInterruptMessages)
#endif

#endif // (NTDDI_VERSION >= NTDDI_VISTA)



//
// General Objects
//

//
//  Required OIDs
//
#define OID_GEN_SUPPORTED_LIST                  0x00010101
#define OID_GEN_HARDWARE_STATUS                 0x00010102
#define OID_GEN_MEDIA_SUPPORTED                 0x00010103
#define OID_GEN_MEDIA_IN_USE                    0x00010104
#define OID_GEN_MAXIMUM_LOOKAHEAD               0x00010105
#define OID_GEN_MAXIMUM_FRAME_SIZE              0x00010106
#define OID_GEN_LINK_SPEED                      0x00010107
#define OID_GEN_TRANSMIT_BUFFER_SPACE           0x00010108
#define OID_GEN_RECEIVE_BUFFER_SPACE            0x00010109
#define OID_GEN_TRANSMIT_BLOCK_SIZE             0x0001010A
#define OID_GEN_RECEIVE_BLOCK_SIZE              0x0001010B
#define OID_GEN_VENDOR_ID                       0x0001010C
#define OID_GEN_VENDOR_DESCRIPTION              0x0001010D
#define OID_GEN_CURRENT_PACKET_FILTER           0x0001010E
#define OID_GEN_CURRENT_LOOKAHEAD               0x0001010F
#define OID_GEN_DRIVER_VERSION                  0x00010110
#define OID_GEN_MAXIMUM_TOTAL_SIZE              0x00010111
#define OID_GEN_PROTOCOL_OPTIONS                0x00010112
#define OID_GEN_MAC_OPTIONS                     0x00010113
#define OID_GEN_MEDIA_CONNECT_STATUS            0x00010114
#define OID_GEN_MAXIMUM_SEND_PACKETS            0x00010115

//
//  Optional OIDs
//
#define OID_GEN_VENDOR_DRIVER_VERSION           0x00010116
#define OID_GEN_SUPPORTED_GUIDS                 0x00010117
#define OID_GEN_NETWORK_LAYER_ADDRESSES         0x00010118  // Set only
#define OID_GEN_TRANSPORT_HEADER_OFFSET         0x00010119  // Set only
#define OID_GEN_MEDIA_CAPABILITIES              0x00010201
#define OID_GEN_PHYSICAL_MEDIUM                 0x00010202

#if ((NTDDI_VERSION >= NTDDI_VISTA) || NDIS_SUPPORT_NDIS6)
//
// new optional for NDIS 6.0
//
#define OID_GEN_RECEIVE_SCALE_CAPABILITIES      0x00010203  // query only
#define OID_GEN_RECEIVE_SCALE_PARAMETERS        0x00010204  // query and set

//
// new for NDIS 6.0. NDIS will handle on behalf of the miniports
//
#define OID_GEN_MAC_ADDRESS                     0x00010205  // query and set
#define OID_GEN_MAX_LINK_SPEED                  0x00010206  // query only
#define OID_GEN_LINK_STATE                      0x00010207  // query only

//
// new and required for NDIS 6 miniports
//
#define OID_GEN_LINK_PARAMETERS                 0x00010208  // set only
#define OID_GEN_INTERRUPT_MODERATION            0x00010209  // query and set
#define OID_GEN_NDIS_RESERVED_3                 0x0001020A
#define OID_GEN_NDIS_RESERVED_4                 0x0001020B
#define OID_GEN_NDIS_RESERVED_5                 0x0001020C


//
// Port related OIDs
//
#define OID_GEN_ENUMERATE_PORTS                 0x0001020D  // query only, handled by NDIS
#define OID_GEN_PORT_STATE                      0x0001020E  // query only, handled by NDIS
#define OID_GEN_PORT_AUTHENTICATION_PARAMETERS  0x0001020F  // Set only

//
// optional OID for NDIS 6 miniports
//
#define OID_GEN_TIMEOUT_DPC_REQUEST_CAPABILITIES 0x00010210 // query only

//
// this OID is handled by NDIS for PCI devices
//
#define OID_GEN_PCI_DEVICE_CUSTOM_PROPERTIES    0x00010211  // query only
#define OID_GEN_NDIS_RESERVED_6                 0x00010212
#define OID_GEN_PHYSICAL_MEDIUM_EX              0x00010213  // query only


#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if ((NTDDI_VERSION >= NTDDI_WIN10_RS3) || NDIS_SUPPORT_NDIS680)
// RSSv2 standard OID: control scaling parameters
#define OID_GEN_RECEIVE_SCALE_PARAMETERS_V2     0x00010214  // query and set
#endif  // ((NTDDI_VERSION >= NTDDI_WIN10_RS3) || NDIS_SUPPORT_NDIS680)

#define OID_GEN_MACHINE_NAME                    0x0001021A  // set only
#define OID_GEN_RNDIS_CONFIG_PARAMETER          0x0001021B  // Set only
#define OID_GEN_VLAN_ID                         0x0001021C

#if ((NTDDI_VERSION >= NTDDI_VISTA) || NDIS_SUPPORT_NDIS6)
#define OID_GEN_RECEIVE_HASH                    0x0001021F  // query and set
#define OID_GEN_MINIPORT_RESTART_ATTRIBUTES     0x0001021D

#if (NDIS_SUPPORT_NDIS61)
//
// Optional OID for header data/split
//
#define OID_GEN_HD_SPLIT_PARAMETERS             0x0001021E  // Set only
#define OID_GEN_HD_SPLIT_CURRENT_CONFIG         0x00010220  // Query only
#endif // (NDIS_SUPPORT_NDIS61)

//
// the following OIDs are used in querying interfaces
//
#define OID_GEN_PROMISCUOUS_MODE                0x00010280  // used in querying interfaces
#define OID_GEN_LAST_CHANGE                     0x00010281  // used in querying interfaces
#define OID_GEN_DISCONTINUITY_TIME              0x00010282  // used in querying interfaces
#define OID_GEN_OPERATIONAL_STATUS              0x00010283  // used in querying interfaces
#define OID_GEN_XMIT_LINK_SPEED                 0x00010284  // used in querying interfaces
#define OID_GEN_RCV_LINK_SPEED                  0x00010285  // used in querying interfaces
#define OID_GEN_UNKNOWN_PROTOS                  0x00010286  // used in querying interfaces
#define OID_GEN_INTERFACE_INFO                  0x00010287  // used in querying interfaces
#define OID_GEN_ADMIN_STATUS                    0x00010288  // used in querying interfaces
#define OID_GEN_ALIAS                           0x00010289  // used in querying interfaces
#define OID_GEN_MEDIA_CONNECT_STATUS_EX         0x0001028A  // used in querying interfaces
#define OID_GEN_LINK_SPEED_EX                   0x0001028B  // used in querying interfaces
#define OID_GEN_MEDIA_DUPLEX_STATE              0x0001028C  // used in querying interfaces
#define OID_GEN_IP_OPER_STATUS                  0x0001028D  // used in querying interfaces

//
// WWAN specific oids
//
#define OID_WWAN_DRIVER_CAPS                    0x0e010100
#define OID_WWAN_DEVICE_CAPS                    0x0e010101
#define OID_WWAN_READY_INFO                     0x0e010102
#define OID_WWAN_RADIO_STATE                    0x0e010103
#define OID_WWAN_PIN                            0x0e010104
#define OID_WWAN_PIN_LIST                       0x0e010105
#define OID_WWAN_HOME_PROVIDER                  0x0e010106
#define OID_WWAN_PREFERRED_PROVIDERS            0x0e010107
#define OID_WWAN_VISIBLE_PROVIDERS              0x0e010108
#define OID_WWAN_REGISTER_STATE                 0x0e010109
#define OID_WWAN_PACKET_SERVICE                 0x0e01010a
#define OID_WWAN_SIGNAL_STATE                   0x0e01010b
#define OID_WWAN_CONNECT                        0x0e01010c
#define OID_WWAN_PROVISIONED_CONTEXTS           0x0e01010d
#define OID_WWAN_SERVICE_ACTIVATION             0x0e01010e
#define OID_WWAN_SMS_CONFIGURATION              0x0e01010f
#define OID_WWAN_SMS_READ                       0x0e010110
#define OID_WWAN_SMS_SEND                       0x0e010111
#define OID_WWAN_SMS_DELETE                     0x0e010112
#define OID_WWAN_SMS_STATUS                     0x0e010113
#define OID_WWAN_VENDOR_SPECIFIC                0x0e010114

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if ((NTDDI_VERSION >= NTDDI_WIN8) || NDIS_SUPPORT_NDIS630)

//
// More WWAN specific oids
//
#define OID_WWAN_AUTH_CHALLENGE                     0x0e010115
#define OID_WWAN_ENUMERATE_DEVICE_SERVICES          0x0e010116
#define OID_WWAN_SUBSCRIBE_DEVICE_SERVICE_EVENTS    0x0e010117
#define OID_WWAN_DEVICE_SERVICE_COMMAND             0x0e010118
#define OID_WWAN_USSD                               0x0e010119
#define OID_WWAN_PIN_EX                             0x0e010121
#define OID_WWAN_ENUMERATE_DEVICE_SERVICE_COMMANDS  0x0e010122
#define OID_WWAN_DEVICE_SERVICE_SESSION             0x0e010123
#define OID_WWAN_DEVICE_SERVICE_SESSION_WRITE       0x0e010124
#define OID_WWAN_PREFERRED_MULTICARRIER_PROVIDERS   0x0e010125
#define OID_WWAN_CREATE_MAC                         0x0e010126
#define OID_WWAN_DELETE_MAC                         0x0e010127
#endif // ((NTDDI_VERSION >= NTDDI_WIN8) || NDIS_SUPPORT_NDIS630)

#if ((NTDDI_VERSION >= NTDDI_WINTHRESHOLD) || NDIS_SUPPORT_NDIS650)
//
// More WWAN specific oids
//
#define OID_WWAN_UICC_FILE_STATUS                   0x0e010128
#define OID_WWAN_UICC_ACCESS_BINARY                 0x0e010129
#define OID_WWAN_UICC_ACCESS_RECORD                 0x0e01012a
#define OID_WWAN_PIN_EX2                            0x0e01012b
#define OID_WWAN_MBIM_VERSION                       0x0e01012c
#define OID_WWAN_SYS_CAPS                           0x0e01012d
#define OID_WWAN_DEVICE_CAPS_EX                     0x0e01012e
#define OID_WWAN_SYS_SLOTMAPPINGS                   0x0e01012f
#define OID_WWAN_SLOT_INFO_STATUS                   0x0e010130
#define OID_WWAN_DEVICE_BINDINGS                    0x0e010131
#define OID_WWAN_REGISTER_STATE_EX                  0x0e010132
#define OID_WWAN_IMS_VOICE_STATE                    0x0e010133
#define OID_WWAN_SIGNAL_STATE_EX                    0x0e010134
#define OID_WWAN_LOCATION_STATE                     0x0e010135
#define OID_WWAN_NITZ                               0x0e010136
#define OID_WWAN_NETWORK_IDLE_HINT                  0x0e010137
#endif // ((NTDDI_VERSION >= NTDDI_WINTHRESHOLD) || NDIS_SUPPORT_NDIS650)

#if ((NTDDI_VERSION >= NTDDI_WIN10) || NDIS_SUPPORT_NDIS651)
//
// More WWAN specific oids
//
#define OID_WWAN_PRESHUTDOWN                        0x0e010138
#endif // ((NTDDI_VERSION >= NTDDI_WIN10) || NDIS_SUPPORT_NDIS651)

#if ((NTDDI_VERSION >= NTDDI_WIN10_RS1) || NDIS_SUPPORT_NDIS660)
//
// More WWAN specific oids
//
#define OID_WWAN_UICC_ATR                           0x0e010139
#define OID_WWAN_UICC_OPEN_CHANNEL                  0x0e01013a
#define OID_WWAN_UICC_CLOSE_CHANNEL                 0x0e01013b
#define OID_WWAN_UICC_APDU                          0x0e01013c
#define OID_WWAN_UICC_TERMINAL_CAPABILITY           0x0e01013d
#define OID_WWAN_PS_MEDIA_CONFIG                    0x0e01013e
#endif // ((NTDDI_VERSION >= NTDDI_WIN10_RS1) || NDIS_SUPPORT_NDIS660)

#if ((NTDDI_VERSION >= NTDDI_WIN10_RS2) || NDIS_SUPPORT_NDIS670)
//
// More WWAN specific oids
//
#define OID_WWAN_SAR_CONFIG                         0x0e01013f
#define OID_WWAN_SAR_TRANSMISSION_STATUS            0x0e010140
#define OID_WWAN_NETWORK_BLACKLIST                  0x0e010141
#define OID_WWAN_LTE_ATTACH_CONFIG                  0x0e010142
#define OID_WWAN_LTE_ATTACH_STATUS                  0x0e010143
#endif // ((NTDDI_VERSION >= NTDDI_WIN10_RS2) || NDIS_SUPPORT_NDIS670)

#if ((NTDDI_VERSION >= NTDDI_WIN10_RS3) || NDIS_SUPPORT_NDIS680)
//
// Even more WWAN specific oids
//
#define OID_WWAN_MODEM_CONFIG_INFO                  0x0e010144
#define OID_WWAN_PCO                                0x0e010145
#define OID_WWAN_UICC_RESET                         0x0e010146
#define OID_WWAN_DEVICE_RESET                       0x0e010147
#define OID_WWAN_BASE_STATIONS_INFO                 0x0e010148
#endif // ((NTDDI_VERSION >= NTDDI_WIN10_RS3) || NDIS_SUPPORT_NDIS680)

#if ((NTDDI_VERSION >= NTDDI_WIN10_RS4) || NDIS_SUPPORT_NDIS680)
//
// more WWAN specific oids (may need to restrict to higher NDIS version such as 7.0)
//
#define OID_WWAN_MPDP                               0x0e010149
#endif // ((NTDDI_VERSION >= NTDDI_WIN10_RS4) || NDIS_SUPPORT_NDIS680)

#if ((NTDDI_VERSION >= NTDDI_WIN10_19H1) || NDIS_SUPPORT_NDIS683)
#define OID_WWAN_UICC_APP_LIST                      0x0e01014a
#define OID_WWAN_MODEM_LOGGING_CONFIG               0x0e01014b
#endif // ((NTDDI_VERSION >= NTDDI_WIN10_19H1) || NDIS_SUPPORT_NDIS683)

#if ((NTDDI_VERSION >= NTDDI_WIN10_VB) || NDIS_SUPPORT_NDIS684)
#define OID_WWAN_REGISTER_PARAMS                    0x0e01014c
#define OID_WWAN_NETWORK_PARAMS                     0x0e01014d
#endif // ((NTDDI_VERSION >= NTDDI_WIN10_VB) || NDIS_SUPPORT_NDIS684)

#if (NTDDI_VERSION >= NTDDI_WIN10_CO)
#define OID_WWAN_UE_POLICY                          0x0e01014e
#endif

//
//  Required statistics
//
#define OID_GEN_XMIT_OK                         0x00020101
#define OID_GEN_RCV_OK                          0x00020102
#define OID_GEN_XMIT_ERROR                      0x00020103
#define OID_GEN_RCV_ERROR                       0x00020104
#define OID_GEN_RCV_NO_BUFFER                   0x00020105

#if ((NTDDI_VERSION >= NTDDI_VISTA) || NDIS_SUPPORT_NDIS6)
//
// mandatory for NDIS 6.0 and higher miniports
//
#define OID_GEN_STATISTICS                      0x00020106
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

//
//  Optional statistics
//
#define OID_GEN_DIRECTED_BYTES_XMIT             0x00020201
#define OID_GEN_DIRECTED_FRAMES_XMIT            0x00020202
#define OID_GEN_MULTICAST_BYTES_XMIT            0x00020203
#define OID_GEN_MULTICAST_FRAMES_XMIT           0x00020204
#define OID_GEN_BROADCAST_BYTES_XMIT            0x00020205
#define OID_GEN_BROADCAST_FRAMES_XMIT           0x00020206
#define OID_GEN_DIRECTED_BYTES_RCV              0x00020207
#define OID_GEN_DIRECTED_FRAMES_RCV             0x00020208
#define OID_GEN_MULTICAST_BYTES_RCV             0x00020209
#define OID_GEN_MULTICAST_FRAMES_RCV            0x0002020A
#define OID_GEN_BROADCAST_BYTES_RCV             0x0002020B
#define OID_GEN_BROADCAST_FRAMES_RCV            0x0002020C
#define OID_GEN_RCV_CRC_ERROR                   0x0002020D
#define OID_GEN_TRANSMIT_QUEUE_LENGTH           0x0002020E

#define OID_GEN_GET_TIME_CAPS                   0x0002020F
#define OID_GEN_GET_NETCARD_TIME                0x00020210
#define OID_GEN_NETCARD_LOAD                    0x00020211
#define OID_GEN_DEVICE_PROFILE                  0x00020212

//
// The following are exported by NDIS itself and are only queryable.
//

//
// the time in milliseconds a driver took to initialize.
//
#define OID_GEN_INIT_TIME_MS                    0x00020213

//
// the number of times the miniport adapter was reset
//
#define OID_GEN_RESET_COUNTS                    0x00020214

//
// the number of timer the miniport reported a media state change
//
#define OID_GEN_MEDIA_SENSE_COUNTS              0x00020215

//
// the friendly name of the adapter
//
#define OID_GEN_FRIENDLY_NAME                   0x00020216

//
// returns miniport information such as whether the driver is serialized or not
// if it supports sending multiple packets, etc. refer to NDIS_MINIPORT_XXX flags
//
#define OID_GEN_NDIS_RESERVED_1                 0x00020217

//
// handled by NDIS to reset the test verification parameters on the
// miniport
//
#define OID_GEN_NDIS_RESERVED_2                 0x00020218
#define OID_GEN_NDIS_RESERVED_5                 0x0001020C


#if ((NTDDI_VERSION >= NTDDI_VISTA) || NDIS_SUPPORT_NDIS6)
//
// more optional OIDs new for NDIS 6.0
//

#define OID_GEN_BYTES_RCV                       0x00020219
#define OID_GEN_BYTES_XMIT                      0x0002021A
#define OID_GEN_RCV_DISCARDS                    0x0002021B
#define OID_GEN_XMIT_DISCARDS                   0x0002021C
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN8) || NDIS_SUPPORT_NDIS630
//
// Optional OIDs for NDIS 6.30
//
#define OID_TCP_RSC_STATISTICS                  0x0002021D

#define OID_GEN_NDIS_RESERVED_7                 0x0002021E


#endif

//
//  These are connection-oriented general OIDs.
//  These replace the above OIDs for connection-oriented media.
//
#define OID_GEN_CO_SUPPORTED_LIST               OID_GEN_SUPPORTED_LIST
#define OID_GEN_CO_HARDWARE_STATUS              OID_GEN_HARDWARE_STATUS
#define OID_GEN_CO_MEDIA_SUPPORTED              OID_GEN_MEDIA_SUPPORTED
#define OID_GEN_CO_MEDIA_IN_USE                 OID_GEN_MEDIA_IN_USE
#define OID_GEN_CO_LINK_SPEED                   OID_GEN_LINK_SPEED
#define OID_GEN_CO_VENDOR_ID                    OID_GEN_VENDOR_ID
#define OID_GEN_CO_VENDOR_DESCRIPTION           OID_GEN_VENDOR_DESCRIPTION
#define OID_GEN_CO_DRIVER_VERSION               OID_GEN_DRIVER_VERSION
#define OID_GEN_CO_PROTOCOL_OPTIONS             OID_GEN_PROTOCOL_OPTIONS
#define OID_GEN_CO_MAC_OPTIONS                  OID_GEN_MAC_OPTIONS
#define OID_GEN_CO_MEDIA_CONNECT_STATUS         OID_GEN_MEDIA_CONNECT_STATUS
#define OID_GEN_CO_VENDOR_DRIVER_VERSION        OID_GEN_VENDOR_DRIVER_VERSION
#define OID_GEN_CO_SUPPORTED_GUIDS              OID_GEN_SUPPORTED_GUIDS
#define OID_GEN_CO_GET_TIME_CAPS                OID_GEN_GET_TIME_CAPS
#define OID_GEN_CO_GET_NETCARD_TIME             OID_GEN_GET_NETCARD_TIME
#define OID_GEN_CO_MINIMUM_LINK_SPEED           0x00020120

//
//  These are connection-oriented statistics OIDs.
//
#define OID_GEN_CO_XMIT_PDUS_OK                 OID_GEN_XMIT_OK
#define OID_GEN_CO_RCV_PDUS_OK                  OID_GEN_RCV_OK
#define OID_GEN_CO_XMIT_PDUS_ERROR              OID_GEN_XMIT_ERROR
#define OID_GEN_CO_RCV_PDUS_ERROR               OID_GEN_RCV_ERROR
#define OID_GEN_CO_RCV_PDUS_NO_BUFFER           OID_GEN_RCV_NO_BUFFER


#define OID_GEN_CO_RCV_CRC_ERROR                OID_GEN_RCV_CRC_ERROR
#define OID_GEN_CO_TRANSMIT_QUEUE_LENGTH        OID_GEN_TRANSMIT_QUEUE_LENGTH
#define OID_GEN_CO_BYTES_XMIT                   OID_GEN_DIRECTED_BYTES_XMIT
#define OID_GEN_CO_BYTES_RCV                    OID_GEN_DIRECTED_BYTES_RCV
#define OID_GEN_CO_NETCARD_LOAD                 OID_GEN_NETCARD_LOAD
#define OID_GEN_CO_DEVICE_PROFILE               OID_GEN_DEVICE_PROFILE
#define OID_GEN_CO_BYTES_XMIT_OUTSTANDING       0x00020221


#if (NDIS_SUPPORT_NDIS686)

//
// Optional OIDs to handle network multiple PF feature.
//
#define OID_KDNET_ENUMERATE_PFS                 0x00020222
#define OID_KDNET_ADD_PF                        0x00020223
#define OID_KDNET_REMOVE_PF                     0x00020224
#define OID_KDNET_QUERY_PF_INFORMATION          0x00020225

#endif // (NDIS_SUPPORT_NDIS686)

//
// 802.3 Objects (Ethernet)
//
#define OID_802_3_PERMANENT_ADDRESS             0x01010101
#define OID_802_3_CURRENT_ADDRESS               0x01010102
#define OID_802_3_MULTICAST_LIST                0x01010103
#define OID_802_3_MAXIMUM_LIST_SIZE             0x01010104
//
// This OID has been deprecated for NDIS 6 drivers.
//
#define OID_802_3_MAC_OPTIONS                   0x01010105  // deprecated


//
// This Flag has been deprecated for NDIS 6 drivers.
//
#define NDIS_802_3_MAC_OPTION_PRIORITY          0x00000001  // deprecated

#define OID_802_3_RCV_ERROR_ALIGNMENT           0x01020101
#define OID_802_3_XMIT_ONE_COLLISION            0x01020102
#define OID_802_3_XMIT_MORE_COLLISIONS          0x01020103

#define OID_802_3_XMIT_DEFERRED                 0x01020201
#define OID_802_3_XMIT_MAX_COLLISIONS           0x01020202
#define OID_802_3_RCV_OVERRUN                   0x01020203
#define OID_802_3_XMIT_UNDERRUN                 0x01020204
#define OID_802_3_XMIT_HEARTBEAT_FAILURE        0x01020205
#define OID_802_3_XMIT_TIMES_CRS_LOST           0x01020206
#define OID_802_3_XMIT_LATE_COLLISIONS          0x01020207

#if ((NTDDI_VERSION >= NTDDI_VISTA) || NDIS_SUPPORT_NDIS6)
//
// new for NDIS 6
//
#define OID_802_3_ADD_MULTICAST_ADDRESS         0x01010208
#define OID_802_3_DELETE_MULTICAST_ADDRESS      0x01010209
#endif // (NTDDI_VERSION >= NTDDI_VISTA)


//
// 802.5 Objects (Token-Ring)
//
#define OID_802_5_PERMANENT_ADDRESS             0x02010101
#define OID_802_5_CURRENT_ADDRESS               0x02010102
#define OID_802_5_CURRENT_FUNCTIONAL            0x02010103
#define OID_802_5_CURRENT_GROUP                 0x02010104
#define OID_802_5_LAST_OPEN_STATUS              0x02010105
#define OID_802_5_CURRENT_RING_STATUS           0x02010106
#define OID_802_5_CURRENT_RING_STATE            0x02010107


#define OID_802_5_LINE_ERRORS                   0x02020101
#define OID_802_5_LOST_FRAMES                   0x02020102

#define OID_802_5_BURST_ERRORS                  0x02020201
#define OID_802_5_AC_ERRORS                     0x02020202
#define OID_802_5_ABORT_DELIMETERS              0x02020203
#define OID_802_5_FRAME_COPIED_ERRORS           0x02020204
#define OID_802_5_FREQUENCY_ERRORS              0x02020205
#define OID_802_5_TOKEN_ERRORS                  0x02020206
#define OID_802_5_INTERNAL_ERRORS               0x02020207


//
// FDDI Objects
//
#define OID_FDDI_LONG_PERMANENT_ADDR            0x03010101
#define OID_FDDI_LONG_CURRENT_ADDR              0x03010102
#define OID_FDDI_LONG_MULTICAST_LIST            0x03010103
#define OID_FDDI_LONG_MAX_LIST_SIZE             0x03010104
#define OID_FDDI_SHORT_PERMANENT_ADDR           0x03010105
#define OID_FDDI_SHORT_CURRENT_ADDR             0x03010106
#define OID_FDDI_SHORT_MULTICAST_LIST           0x03010107
#define OID_FDDI_SHORT_MAX_LIST_SIZE            0x03010108

#define OID_FDDI_ATTACHMENT_TYPE                0x03020101
#define OID_FDDI_UPSTREAM_NODE_LONG             0x03020102
#define OID_FDDI_DOWNSTREAM_NODE_LONG           0x03020103
#define OID_FDDI_FRAME_ERRORS                   0x03020104
#define OID_FDDI_FRAMES_LOST                    0x03020105
#define OID_FDDI_RING_MGT_STATE                 0x03020106
#define OID_FDDI_LCT_FAILURES                   0x03020107
#define OID_FDDI_LEM_REJECTS                    0x03020108
#define OID_FDDI_LCONNECTION_STATE              0x03020109

#define OID_FDDI_SMT_STATION_ID                 0x03030201
#define OID_FDDI_SMT_OP_VERSION_ID              0x03030202
#define OID_FDDI_SMT_HI_VERSION_ID              0x03030203
#define OID_FDDI_SMT_LO_VERSION_ID              0x03030204
#define OID_FDDI_SMT_MANUFACTURER_DATA          0x03030205
#define OID_FDDI_SMT_USER_DATA                  0x03030206
#define OID_FDDI_SMT_MIB_VERSION_ID             0x03030207
#define OID_FDDI_SMT_MAC_CT                     0x03030208
#define OID_FDDI_SMT_NON_MASTER_CT              0x03030209
#define OID_FDDI_SMT_MASTER_CT                  0x0303020A
#define OID_FDDI_SMT_AVAILABLE_PATHS            0x0303020B
#define OID_FDDI_SMT_CONFIG_CAPABILITIES        0x0303020C
#define OID_FDDI_SMT_CONFIG_POLICY              0x0303020D
#define OID_FDDI_SMT_CONNECTION_POLICY          0x0303020E
#define OID_FDDI_SMT_T_NOTIFY                   0x0303020F
#define OID_FDDI_SMT_STAT_RPT_POLICY            0x03030210
#define OID_FDDI_SMT_TRACE_MAX_EXPIRATION       0x03030211
#define OID_FDDI_SMT_PORT_INDEXES               0x03030212
#define OID_FDDI_SMT_MAC_INDEXES                0x03030213
#define OID_FDDI_SMT_BYPASS_PRESENT             0x03030214
#define OID_FDDI_SMT_ECM_STATE                  0x03030215
#define OID_FDDI_SMT_CF_STATE                   0x03030216
#define OID_FDDI_SMT_HOLD_STATE                 0x03030217
#define OID_FDDI_SMT_REMOTE_DISCONNECT_FLAG     0x03030218
#define OID_FDDI_SMT_STATION_STATUS             0x03030219
#define OID_FDDI_SMT_PEER_WRAP_FLAG             0x0303021A
#define OID_FDDI_SMT_MSG_TIME_STAMP             0x0303021B
#define OID_FDDI_SMT_TRANSITION_TIME_STAMP      0x0303021C
#define OID_FDDI_SMT_SET_COUNT                  0x0303021D
#define OID_FDDI_SMT_LAST_SET_STATION_ID        0x0303021E
#define OID_FDDI_MAC_FRAME_STATUS_FUNCTIONS     0x0303021F
#define OID_FDDI_MAC_BRIDGE_FUNCTIONS           0x03030220
#define OID_FDDI_MAC_T_MAX_CAPABILITY           0x03030221
#define OID_FDDI_MAC_TVX_CAPABILITY             0x03030222
#define OID_FDDI_MAC_AVAILABLE_PATHS            0x03030223
#define OID_FDDI_MAC_CURRENT_PATH               0x03030224
#define OID_FDDI_MAC_UPSTREAM_NBR               0x03030225
#define OID_FDDI_MAC_DOWNSTREAM_NBR             0x03030226
#define OID_FDDI_MAC_OLD_UPSTREAM_NBR           0x03030227
#define OID_FDDI_MAC_OLD_DOWNSTREAM_NBR         0x03030228
#define OID_FDDI_MAC_DUP_ADDRESS_TEST           0x03030229
#define OID_FDDI_MAC_REQUESTED_PATHS            0x0303022A
#define OID_FDDI_MAC_DOWNSTREAM_PORT_TYPE       0x0303022B
#define OID_FDDI_MAC_INDEX                      0x0303022C
#define OID_FDDI_MAC_SMT_ADDRESS                0x0303022D
#define OID_FDDI_MAC_LONG_GRP_ADDRESS           0x0303022E
#define OID_FDDI_MAC_SHORT_GRP_ADDRESS          0x0303022F
#define OID_FDDI_MAC_T_REQ                      0x03030230
#define OID_FDDI_MAC_T_NEG                      0x03030231
#define OID_FDDI_MAC_T_MAX                      0x03030232
#define OID_FDDI_MAC_TVX_VALUE                  0x03030233
#define OID_FDDI_MAC_T_PRI0                     0x03030234
#define OID_FDDI_MAC_T_PRI1                     0x03030235
#define OID_FDDI_MAC_T_PRI2                     0x03030236
#define OID_FDDI_MAC_T_PRI3                     0x03030237
#define OID_FDDI_MAC_T_PRI4                     0x03030238
#define OID_FDDI_MAC_T_PRI5                     0x03030239
#define OID_FDDI_MAC_T_PRI6                     0x0303023A
#define OID_FDDI_MAC_FRAME_CT                   0x0303023B
#define OID_FDDI_MAC_COPIED_CT                  0x0303023C
#define OID_FDDI_MAC_TRANSMIT_CT                0x0303023D
#define OID_FDDI_MAC_TOKEN_CT                   0x0303023E
#define OID_FDDI_MAC_ERROR_CT                   0x0303023F
#define OID_FDDI_MAC_LOST_CT                    0x03030240
#define OID_FDDI_MAC_TVX_EXPIRED_CT             0x03030241
#define OID_FDDI_MAC_NOT_COPIED_CT              0x03030242
#define OID_FDDI_MAC_LATE_CT                    0x03030243
#define OID_FDDI_MAC_RING_OP_CT                 0x03030244
#define OID_FDDI_MAC_FRAME_ERROR_THRESHOLD      0x03030245
#define OID_FDDI_MAC_FRAME_ERROR_RATIO          0x03030246
#define OID_FDDI_MAC_NOT_COPIED_THRESHOLD       0x03030247
#define OID_FDDI_MAC_NOT_COPIED_RATIO           0x03030248
#define OID_FDDI_MAC_RMT_STATE                  0x03030249
#define OID_FDDI_MAC_DA_FLAG                    0x0303024A
#define OID_FDDI_MAC_UNDA_FLAG                  0x0303024B
#define OID_FDDI_MAC_FRAME_ERROR_FLAG           0x0303024C
#define OID_FDDI_MAC_NOT_COPIED_FLAG            0x0303024D
#define OID_FDDI_MAC_MA_UNITDATA_AVAILABLE      0x0303024E
#define OID_FDDI_MAC_HARDWARE_PRESENT           0x0303024F
#define OID_FDDI_MAC_MA_UNITDATA_ENABLE         0x03030250
#define OID_FDDI_PATH_INDEX                     0x03030251
#define OID_FDDI_PATH_RING_LATENCY              0x03030252
#define OID_FDDI_PATH_TRACE_STATUS              0x03030253
#define OID_FDDI_PATH_SBA_PAYLOAD               0x03030254
#define OID_FDDI_PATH_SBA_OVERHEAD              0x03030255
#define OID_FDDI_PATH_CONFIGURATION             0x03030256
#define OID_FDDI_PATH_T_R_MODE                  0x03030257
#define OID_FDDI_PATH_SBA_AVAILABLE             0x03030258
#define OID_FDDI_PATH_TVX_LOWER_BOUND           0x03030259
#define OID_FDDI_PATH_T_MAX_LOWER_BOUND         0x0303025A
#define OID_FDDI_PATH_MAX_T_REQ                 0x0303025B
#define OID_FDDI_PORT_MY_TYPE                   0x0303025C
#define OID_FDDI_PORT_NEIGHBOR_TYPE             0x0303025D
#define OID_FDDI_PORT_CONNECTION_POLICIES       0x0303025E
#define OID_FDDI_PORT_MAC_INDICATED             0x0303025F
#define OID_FDDI_PORT_CURRENT_PATH              0x03030260
#define OID_FDDI_PORT_REQUESTED_PATHS           0x03030261
#define OID_FDDI_PORT_MAC_PLACEMENT             0x03030262
#define OID_FDDI_PORT_AVAILABLE_PATHS           0x03030263
#define OID_FDDI_PORT_MAC_LOOP_TIME             0x03030264
#define OID_FDDI_PORT_PMD_CLASS                 0x03030265
#define OID_FDDI_PORT_CONNECTION_CAPABILITIES   0x03030266
#define OID_FDDI_PORT_INDEX                     0x03030267
#define OID_FDDI_PORT_MAINT_LS                  0x03030268
#define OID_FDDI_PORT_BS_FLAG                   0x03030269
#define OID_FDDI_PORT_PC_LS                     0x0303026A
#define OID_FDDI_PORT_EB_ERROR_CT               0x0303026B
#define OID_FDDI_PORT_LCT_FAIL_CT               0x0303026C
#define OID_FDDI_PORT_LER_ESTIMATE              0x0303026D
#define OID_FDDI_PORT_LEM_REJECT_CT             0x0303026E
#define OID_FDDI_PORT_LEM_CT                    0x0303026F
#define OID_FDDI_PORT_LER_CUTOFF                0x03030270
#define OID_FDDI_PORT_LER_ALARM                 0x03030271
#define OID_FDDI_PORT_CONNNECT_STATE            0x03030272
#define OID_FDDI_PORT_PCM_STATE                 0x03030273
#define OID_FDDI_PORT_PC_WITHHOLD               0x03030274
#define OID_FDDI_PORT_LER_FLAG                  0x03030275
#define OID_FDDI_PORT_HARDWARE_PRESENT          0x03030276
#define OID_FDDI_SMT_STATION_ACTION             0x03030277
#define OID_FDDI_PORT_ACTION                    0x03030278
#define OID_FDDI_IF_DESCR                       0x03030279
#define OID_FDDI_IF_TYPE                        0x0303027A
#define OID_FDDI_IF_MTU                         0x0303027B
#define OID_FDDI_IF_SPEED                       0x0303027C
#define OID_FDDI_IF_PHYS_ADDRESS                0x0303027D
#define OID_FDDI_IF_ADMIN_STATUS                0x0303027E
#define OID_FDDI_IF_OPER_STATUS                 0x0303027F
#define OID_FDDI_IF_LAST_CHANGE                 0x03030280
#define OID_FDDI_IF_IN_OCTETS                   0x03030281
#define OID_FDDI_IF_IN_UCAST_PKTS               0x03030282
#define OID_FDDI_IF_IN_NUCAST_PKTS              0x03030283
#define OID_FDDI_IF_IN_DISCARDS                 0x03030284
#define OID_FDDI_IF_IN_ERRORS                   0x03030285
#define OID_FDDI_IF_IN_UNKNOWN_PROTOS           0x03030286
#define OID_FDDI_IF_OUT_OCTETS                  0x03030287
#define OID_FDDI_IF_OUT_UCAST_PKTS              0x03030288
#define OID_FDDI_IF_OUT_NUCAST_PKTS             0x03030289
#define OID_FDDI_IF_OUT_DISCARDS                0x0303028A
#define OID_FDDI_IF_OUT_ERRORS                  0x0303028B
#define OID_FDDI_IF_OUT_QLEN                    0x0303028C
#define OID_FDDI_IF_SPECIFIC                    0x0303028D

//
// WAN objects
//
#define OID_WAN_PERMANENT_ADDRESS               0x04010101
#define OID_WAN_CURRENT_ADDRESS                 0x04010102
#define OID_WAN_QUALITY_OF_SERVICE              0x04010103
#define OID_WAN_PROTOCOL_TYPE                   0x04010104
#define OID_WAN_MEDIUM_SUBTYPE                  0x04010105
#define OID_WAN_HEADER_FORMAT                   0x04010106

#define OID_WAN_GET_INFO                        0x04010107
#define OID_WAN_SET_LINK_INFO                   0x04010108
#define OID_WAN_GET_LINK_INFO                   0x04010109

#define OID_WAN_LINE_COUNT                      0x0401010A
#define OID_WAN_PROTOCOL_CAPS                   0x0401010B

#define OID_WAN_GET_BRIDGE_INFO                 0x0401020A
#define OID_WAN_SET_BRIDGE_INFO                 0x0401020B
#define OID_WAN_GET_COMP_INFO                   0x0401020C
#define OID_WAN_SET_COMP_INFO                   0x0401020D
#define OID_WAN_GET_STATS_INFO                  0x0401020E

//
//  These are connection-oriented WAN OIDs.
//  These replace the above OIDs for CoNDIS WAN Miniports
//
#define OID_WAN_CO_GET_INFO                     0x04010180
#define OID_WAN_CO_SET_LINK_INFO                0x04010181
#define OID_WAN_CO_GET_LINK_INFO                0x04010182
#define OID_WAN_CO_GET_COMP_INFO                0x04010280
#define OID_WAN_CO_SET_COMP_INFO                0x04010281
#define OID_WAN_CO_GET_STATS_INFO               0x04010282


//
// LocalTalk objects
//
#define OID_LTALK_CURRENT_NODE_ID               0x05010102

#define OID_LTALK_IN_BROADCASTS                 0x05020101
#define OID_LTALK_IN_LENGTH_ERRORS              0x05020102

#define OID_LTALK_OUT_NO_HANDLERS               0x05020201
#define OID_LTALK_COLLISIONS                    0x05020202
#define OID_LTALK_DEFERS                        0x05020203
#define OID_LTALK_NO_DATA_ERRORS                0x05020204
#define OID_LTALK_RANDOM_CTS_ERRORS             0x05020205
#define OID_LTALK_FCS_ERRORS                    0x05020206


//
// Arcnet objects
//
#define OID_ARCNET_PERMANENT_ADDRESS            0x06010101
#define OID_ARCNET_CURRENT_ADDRESS              0x06010102

#define OID_ARCNET_RECONFIGURATIONS             0x06020201

//
// TAPI objects
//
#define OID_TAPI_ACCEPT                         0x07030101
#define OID_TAPI_ANSWER                         0x07030102
#define OID_TAPI_CLOSE                          0x07030103
#define OID_TAPI_CLOSE_CALL                     0x07030104
#define OID_TAPI_CONDITIONAL_MEDIA_DETECTION    0x07030105
#define OID_TAPI_CONFIG_DIALOG                  0x07030106
#define OID_TAPI_DEV_SPECIFIC                   0x07030107
#define OID_TAPI_DIAL                           0x07030108
#define OID_TAPI_DROP                           0x07030109
#define OID_TAPI_GET_ADDRESS_CAPS               0x0703010A
#define OID_TAPI_GET_ADDRESS_ID                 0x0703010B
#define OID_TAPI_GET_ADDRESS_STATUS             0x0703010C
#define OID_TAPI_GET_CALL_ADDRESS_ID            0x0703010D
#define OID_TAPI_GET_CALL_INFO                  0x0703010E
#define OID_TAPI_GET_CALL_STATUS                0x0703010F
#define OID_TAPI_GET_DEV_CAPS                   0x07030110
#define OID_TAPI_GET_DEV_CONFIG                 0x07030111
#define OID_TAPI_GET_EXTENSION_ID               0x07030112
#define OID_TAPI_GET_ID                         0x07030113
#define OID_TAPI_GET_LINE_DEV_STATUS            0x07030114
#define OID_TAPI_MAKE_CALL                      0x07030115
#define OID_TAPI_NEGOTIATE_EXT_VERSION          0x07030116
#define OID_TAPI_OPEN                           0x07030117
#define OID_TAPI_PROVIDER_INITIALIZE            0x07030118
#define OID_TAPI_PROVIDER_SHUTDOWN              0x07030119
#define OID_TAPI_SECURE_CALL                    0x0703011A
#define OID_TAPI_SELECT_EXT_VERSION             0x0703011B
#define OID_TAPI_SEND_USER_USER_INFO            0x0703011C
#define OID_TAPI_SET_APP_SPECIFIC               0x0703011D
#define OID_TAPI_SET_CALL_PARAMS                0x0703011E
#define OID_TAPI_SET_DEFAULT_MEDIA_DETECTION    0x0703011F
#define OID_TAPI_SET_DEV_CONFIG                 0x07030120
#define OID_TAPI_SET_MEDIA_MODE                 0x07030121
#define OID_TAPI_SET_STATUS_MESSAGES            0x07030122
#define OID_TAPI_GATHER_DIGITS                  0x07030123
#define OID_TAPI_MONITOR_DIGITS                 0x07030124

//
// ATM Connection Oriented OIDs
//
#define OID_ATM_SUPPORTED_VC_RATES              0x08010101
#define OID_ATM_SUPPORTED_SERVICE_CATEGORY      0x08010102
#define OID_ATM_SUPPORTED_AAL_TYPES             0x08010103
#define OID_ATM_HW_CURRENT_ADDRESS              0x08010104
#define OID_ATM_MAX_ACTIVE_VCS                  0x08010105
#define OID_ATM_MAX_ACTIVE_VCI_BITS             0x08010106
#define OID_ATM_MAX_ACTIVE_VPI_BITS             0x08010107
#define OID_ATM_MAX_AAL0_PACKET_SIZE            0x08010108
#define OID_ATM_MAX_AAL1_PACKET_SIZE            0x08010109
#define OID_ATM_MAX_AAL34_PACKET_SIZE           0x0801010A
#define OID_ATM_MAX_AAL5_PACKET_SIZE            0x0801010B

#define OID_ATM_SIGNALING_VPIVCI                0x08010201
#define OID_ATM_ASSIGNED_VPI                    0x08010202
#define OID_ATM_ACQUIRE_ACCESS_NET_RESOURCES    0x08010203
#define OID_ATM_RELEASE_ACCESS_NET_RESOURCES    0x08010204
#define OID_ATM_ILMI_VPIVCI                     0x08010205
#define OID_ATM_DIGITAL_BROADCAST_VPIVCI        0x08010206
#define OID_ATM_GET_NEAREST_FLOW                0x08010207
#define OID_ATM_ALIGNMENT_REQUIRED              0x08010208
#define OID_ATM_LECS_ADDRESS                    0x08010209
#define OID_ATM_SERVICE_ADDRESS                 0x0801020A

#define OID_ATM_CALL_PROCEEDING                 0x0801020B  // UNI 4.0
#define OID_ATM_CALL_ALERTING                   0x0801020C  // UNI 4.0
#define OID_ATM_PARTY_ALERTING                  0x0801020D  // UNI 4.0
#define OID_ATM_CALL_NOTIFY                     0x0801020E  // UNI 4.0

#define OID_ATM_MY_IP_NM_ADDRESS                0x0801020F


//
//  ATM specific statistics OIDs.
//
#define OID_ATM_RCV_CELLS_OK                    0x08020101
#define OID_ATM_XMIT_CELLS_OK                   0x08020102
#define OID_ATM_RCV_CELLS_DROPPED               0x08020103

#define OID_ATM_RCV_INVALID_VPI_VCI             0x08020201
#define OID_ATM_CELLS_HEC_ERROR                 0x08020202
#define OID_ATM_RCV_REASSEMBLY_ERROR            0x08020203


//
// IEEE 802.11 OIDs
//
#define OID_802_11_BSSID                        0x0D010101
#define OID_802_11_SSID                         0x0D010102
#define OID_802_11_NETWORK_TYPES_SUPPORTED      0x0D010203
#define OID_802_11_NETWORK_TYPE_IN_USE          0x0D010204
#define OID_802_11_TX_POWER_LEVEL               0x0D010205
#define OID_802_11_RSSI                         0x0D010206
#define OID_802_11_RSSI_TRIGGER                 0x0D010207
#define OID_802_11_INFRASTRUCTURE_MODE          0x0D010108
#define OID_802_11_FRAGMENTATION_THRESHOLD      0x0D010209
#define OID_802_11_RTS_THRESHOLD                0x0D01020A
#define OID_802_11_NUMBER_OF_ANTENNAS           0x0D01020B
#define OID_802_11_RX_ANTENNA_SELECTED          0x0D01020C
#define OID_802_11_TX_ANTENNA_SELECTED          0x0D01020D
#define OID_802_11_SUPPORTED_RATES              0x0D01020E
#define OID_802_11_DESIRED_RATES                0x0D010210
#define OID_802_11_CONFIGURATION                0x0D010211
#define OID_802_11_STATISTICS                   0x0D020212
#define OID_802_11_ADD_WEP                      0x0D010113
#define OID_802_11_REMOVE_WEP                   0x0D010114
#define OID_802_11_DISASSOCIATE                 0x0D010115
#define OID_802_11_POWER_MODE                   0x0D010216
#define OID_802_11_BSSID_LIST                   0x0D010217
#define OID_802_11_AUTHENTICATION_MODE          0x0D010118
#define OID_802_11_PRIVACY_FILTER               0x0D010119
#define OID_802_11_BSSID_LIST_SCAN              0x0D01011A
#define OID_802_11_WEP_STATUS                   0x0D01011B
// Renamed to reflect better the extended set of encryption status
#define OID_802_11_ENCRYPTION_STATUS            OID_802_11_WEP_STATUS
#define OID_802_11_RELOAD_DEFAULTS              0x0D01011C
// Added to allow key mapping and default keys
#define OID_802_11_ADD_KEY                      0x0D01011D
#define OID_802_11_REMOVE_KEY                   0x0D01011E
#define OID_802_11_ASSOCIATION_INFORMATION      0x0D01011F
#define OID_802_11_TEST                         0x0D010120
#define OID_802_11_MEDIA_STREAM_MODE            0x0D010121
#define OID_802_11_CAPABILITY                   0x0D010122
#define OID_802_11_PMKID                        0x0D010123
#define OID_802_11_NON_BCAST_SSID_LIST          0x0D010124
#define OID_802_11_RADIO_STATUS                 0x0D010125

//
// some of well known Ethernet frame types (in big endian notation)
//
#define NDIS_ETH_TYPE_IPV4              0x0800  // IPV4
#define NDIS_ETH_TYPE_ARP               0x0806  // ARP
#define NDIS_ETH_TYPE_IPV6              0x86dd  // IPV6
#define NDIS_ETH_TYPE_802_1X            0x888e  // 802.1x
#define NDIS_ETH_TYPE_802_1Q            0x8100  // 802.1p
#define NDIS_ETH_TYPE_SLOW_PROTOCOL     0x8809  // Slow protocols (LACP, etc.)

#define NDIS_802_11_LENGTH_SSID         32
#define NDIS_802_11_LENGTH_RATES        8
#define NDIS_802_11_LENGTH_RATES_EX     16

//
// IEEE 802.11 Structures and definitions
//
// new types for Media Specific Indications
typedef enum _NDIS_802_11_STATUS_TYPE
{
    Ndis802_11StatusType_Authentication,
    Ndis802_11StatusType_MediaStreamMode,
    Ndis802_11StatusType_PMKID_CandidateList,
    Ndis802_11StatusTypeMax    // not a real type, defined as an upper bound
} NDIS_802_11_STATUS_TYPE, *PNDIS_802_11_STATUS_TYPE;

typedef UCHAR   NDIS_802_11_MAC_ADDRESS[6];

typedef struct _NDIS_802_11_STATUS_INDICATION
{
    NDIS_802_11_STATUS_TYPE StatusType;
} NDIS_802_11_STATUS_INDICATION, *PNDIS_802_11_STATUS_INDICATION;

// mask for authentication/integrity fields
#define NDIS_802_11_AUTH_REQUEST_AUTH_FIELDS            0x0f

#define NDIS_802_11_AUTH_REQUEST_REAUTH                 0x01
#define NDIS_802_11_AUTH_REQUEST_KEYUPDATE              0x02
#define NDIS_802_11_AUTH_REQUEST_PAIRWISE_ERROR         0x06
#define NDIS_802_11_AUTH_REQUEST_GROUP_ERROR            0x0E

typedef struct _NDIS_802_11_AUTHENTICATION_REQUEST
{
    ULONG Length;            // Length of structure
    NDIS_802_11_MAC_ADDRESS Bssid;
    ULONG Flags;
} NDIS_802_11_AUTHENTICATION_REQUEST, *PNDIS_802_11_AUTHENTICATION_REQUEST;

//Added new types for PMKID Candidate lists.
typedef struct _PMKID_CANDIDATE {
    NDIS_802_11_MAC_ADDRESS BSSID;
    ULONG Flags;
} PMKID_CANDIDATE, *PPMKID_CANDIDATE;

typedef struct _NDIS_802_11_PMKID_CANDIDATE_LIST
{
    ULONG Version;       // Version of the structure
    ULONG NumCandidates; // No. of pmkid candidates
    PMKID_CANDIDATE CandidateList[1];
} NDIS_802_11_PMKID_CANDIDATE_LIST, *PNDIS_802_11_PMKID_CANDIDATE_LIST;

//Flags for PMKID Candidate list structure
#define NDIS_802_11_PMKID_CANDIDATE_PREAUTH_ENABLED    0x01

// Added new types for OFDM 5G and 2.4G
typedef enum _NDIS_802_11_NETWORK_TYPE
{
    Ndis802_11FH,
    Ndis802_11DS,
    Ndis802_11OFDM5,
    Ndis802_11OFDM24,
    Ndis802_11Automode,
    Ndis802_11NetworkTypeMax    // not a real type, defined as an upper bound
} NDIS_802_11_NETWORK_TYPE, *PNDIS_802_11_NETWORK_TYPE;

typedef struct _NDIS_802_11_NETWORK_TYPE_LIST
{
    ULONG                       NumberOfItems;  // in list below, at least 1
    NDIS_802_11_NETWORK_TYPE    NetworkType [1];
} NDIS_802_11_NETWORK_TYPE_LIST, *PNDIS_802_11_NETWORK_TYPE_LIST;

typedef enum _NDIS_802_11_POWER_MODE
{
    Ndis802_11PowerModeCAM,
    Ndis802_11PowerModeMAX_PSP,
    Ndis802_11PowerModeFast_PSP,
    Ndis802_11PowerModeMax      // not a real mode, defined as an upper bound
} NDIS_802_11_POWER_MODE, *PNDIS_802_11_POWER_MODE;

typedef ULONG   NDIS_802_11_TX_POWER_LEVEL; // in milliwatts

//
// Received Signal Strength Indication
//
typedef LONG   NDIS_802_11_RSSI;           // in dBm

typedef struct _NDIS_802_11_CONFIGURATION_FH
{
    ULONG           Length;             // Length of structure
    ULONG           HopPattern;         // As defined by 802.11, MSB set
    ULONG           HopSet;             // to one if non-802.11
    ULONG           DwellTime;          // units are Kusec
} NDIS_802_11_CONFIGURATION_FH, *PNDIS_802_11_CONFIGURATION_FH;

typedef struct _NDIS_802_11_CONFIGURATION
{
    ULONG           Length;             // Length of structure
    ULONG           BeaconPeriod;       // units are Kusec
    ULONG           ATIMWindow;         // units are Kusec
    ULONG           DSConfig;           // Frequency, units are kHz
    NDIS_802_11_CONFIGURATION_FH    FHConfig;
} NDIS_802_11_CONFIGURATION, *PNDIS_802_11_CONFIGURATION;

typedef struct _NDIS_802_11_STATISTICS
{
    ULONG           Length;             // Length of structure
    LARGE_INTEGER   TransmittedFragmentCount;
    LARGE_INTEGER   MulticastTransmittedFrameCount;
    LARGE_INTEGER   FailedCount;
    LARGE_INTEGER   RetryCount;
    LARGE_INTEGER   MultipleRetryCount;
    LARGE_INTEGER   RTSSuccessCount;
    LARGE_INTEGER   RTSFailureCount;
    LARGE_INTEGER   ACKFailureCount;
    LARGE_INTEGER   FrameDuplicateCount;
    LARGE_INTEGER   ReceivedFragmentCount;
    LARGE_INTEGER   MulticastReceivedFrameCount;
    LARGE_INTEGER   FCSErrorCount;
    LARGE_INTEGER   TKIPLocalMICFailures;
    LARGE_INTEGER   TKIPICVErrorCount;
    LARGE_INTEGER   TKIPCounterMeasuresInvoked;
    LARGE_INTEGER   TKIPReplays;
    LARGE_INTEGER   CCMPFormatErrors;
    LARGE_INTEGER   CCMPReplays;
    LARGE_INTEGER   CCMPDecryptErrors;
    LARGE_INTEGER   FourWayHandshakeFailures;
    LARGE_INTEGER   WEPUndecryptableCount;
    LARGE_INTEGER   WEPICVErrorCount;
    LARGE_INTEGER   DecryptSuccessCount;
    LARGE_INTEGER   DecryptFailureCount;
} NDIS_802_11_STATISTICS, *PNDIS_802_11_STATISTICS;

typedef  ULONG  NDIS_802_11_KEY_INDEX;
typedef ULONGLONG   NDIS_802_11_KEY_RSC;

// Key mapping keys require a BSSID
typedef struct _NDIS_802_11_KEY
{
    ULONG           Length;             // Length of this structure
    ULONG           KeyIndex;
    ULONG           KeyLength;          // length of key in bytes
    NDIS_802_11_MAC_ADDRESS BSSID;
    NDIS_802_11_KEY_RSC KeyRSC;
    _Field_size_bytes_(KeyLength) UCHAR KeyMaterial[1];     // variable length depending on above field
} NDIS_802_11_KEY, *PNDIS_802_11_KEY;

typedef struct _NDIS_802_11_REMOVE_KEY
{
    ULONG           Length;             // Length of this structure
    ULONG           KeyIndex;
    NDIS_802_11_MAC_ADDRESS BSSID;
} NDIS_802_11_REMOVE_KEY, *PNDIS_802_11_REMOVE_KEY;

typedef struct _NDIS_802_11_WEP
{
    ULONG           Length;             // Length of this structure
    ULONG           KeyIndex;           // 0 is the per-client key, 1-N are the
                                        // global keys
    ULONG           KeyLength;          // length of key in bytes
    UCHAR           KeyMaterial[1];     // variable length depending on above field
} NDIS_802_11_WEP, *PNDIS_802_11_WEP;


typedef enum _NDIS_802_11_NETWORK_INFRASTRUCTURE
{
    Ndis802_11IBSS,
    Ndis802_11Infrastructure,
    Ndis802_11AutoUnknown,
    Ndis802_11InfrastructureMax         // Not a real value, defined as upper bound
} NDIS_802_11_NETWORK_INFRASTRUCTURE, *PNDIS_802_11_NETWORK_INFRASTRUCTURE;

// Add new authentication modes
typedef enum _NDIS_802_11_AUTHENTICATION_MODE
{
    Ndis802_11AuthModeOpen,
    Ndis802_11AuthModeShared,
    Ndis802_11AuthModeAutoSwitch,
    Ndis802_11AuthModeWPA,
    Ndis802_11AuthModeWPAPSK,
    Ndis802_11AuthModeWPANone,
    Ndis802_11AuthModeWPA2,
    Ndis802_11AuthModeWPA2PSK,
    Ndis802_11AuthModeWPA3,
#if (NTDDI_VERSION >= NTDDI_WIN10_FE)
    Ndis802_11AuthModeWPA3Ent192 = Ndis802_11AuthModeWPA3,
#endif
    Ndis802_11AuthModeWPA3SAE,
#if(NDIS_SUPPORT_NDIS684)
    Ndis802_11AuthModeOWE,
#endif // (NDIS_SUPPORT_NDIS684)
#if (NTDDI_VERSION >= NTDDI_WIN10_FE)
    Ndis802_11AuthModeWPA3Ent,
#endif
    Ndis802_11AuthModeMax               // Not a real mode, defined as upper bound
} NDIS_802_11_AUTHENTICATION_MODE, *PNDIS_802_11_AUTHENTICATION_MODE;

typedef UCHAR   NDIS_802_11_RATES[NDIS_802_11_LENGTH_RATES];        // Set of 8 data rates
typedef UCHAR   NDIS_802_11_RATES_EX[NDIS_802_11_LENGTH_RATES_EX];  // Set of 16 data rates

typedef struct _NDIS_802_11_SSID
{
    ULONG   SsidLength;         // length of SSID field below, in bytes;
                                // this can be zero.
    UCHAR   Ssid[NDIS_802_11_LENGTH_SSID];           // SSID information field
} NDIS_802_11_SSID, *PNDIS_802_11_SSID;


typedef struct _NDIS_WLAN_BSSID
{
    ULONG                               Length;             // Length of this structure
    NDIS_802_11_MAC_ADDRESS             MacAddress;         // BSSID
    UCHAR                               Reserved[2];
    NDIS_802_11_SSID                    Ssid;               // SSID
    ULONG                               Privacy;            // WEP encryption requirement
    NDIS_802_11_RSSI                    Rssi;               // receive signal
                                                            // strength in dBm
    NDIS_802_11_NETWORK_TYPE            NetworkTypeInUse;
    NDIS_802_11_CONFIGURATION           Configuration;
    NDIS_802_11_NETWORK_INFRASTRUCTURE  InfrastructureMode;
    NDIS_802_11_RATES                   SupportedRates;
} NDIS_WLAN_BSSID, *PNDIS_WLAN_BSSID;

typedef struct _NDIS_802_11_BSSID_LIST
{
    ULONG           NumberOfItems;      // in list below, at least 1
    NDIS_WLAN_BSSID Bssid[1];
} NDIS_802_11_BSSID_LIST, *PNDIS_802_11_BSSID_LIST;

// Added Capabilities, IELength and IEs for each BSSID
typedef struct _NDIS_WLAN_BSSID_EX
{
    ULONG                               Length;             // Length of this structure
    NDIS_802_11_MAC_ADDRESS             MacAddress;         // BSSID
    UCHAR                               Reserved[2];
    NDIS_802_11_SSID                    Ssid;               // SSID
    ULONG                               Privacy;            // WEP encryption requirement
    NDIS_802_11_RSSI                    Rssi;               // receive signal
                                                            // strength in dBm
    NDIS_802_11_NETWORK_TYPE            NetworkTypeInUse;
    NDIS_802_11_CONFIGURATION           Configuration;
    NDIS_802_11_NETWORK_INFRASTRUCTURE  InfrastructureMode;
    NDIS_802_11_RATES_EX                SupportedRates;
    ULONG                               IELength;
    UCHAR                               IEs[1];
} NDIS_WLAN_BSSID_EX, *PNDIS_WLAN_BSSID_EX;

typedef struct _NDIS_802_11_BSSID_LIST_EX
{
    ULONG           NumberOfItems;      // in list below, at least 1
    NDIS_WLAN_BSSID_EX Bssid[1];
} NDIS_802_11_BSSID_LIST_EX, *PNDIS_802_11_BSSID_LIST_EX;

typedef struct _NDIS_802_11_FIXED_IEs
{
    UCHAR Timestamp[8];
    USHORT BeaconInterval;
    USHORT Capabilities;
} NDIS_802_11_FIXED_IEs, *PNDIS_802_11_FIXED_IEs;

typedef struct _NDIS_802_11_VARIABLE_IEs
{
    UCHAR ElementID;
    UCHAR Length;    // Number of bytes in data field
    UCHAR data[1];
} NDIS_802_11_VARIABLE_IEs, *PNDIS_802_11_VARIABLE_IEs;

typedef  ULONG   NDIS_802_11_FRAGMENTATION_THRESHOLD;

typedef  ULONG   NDIS_802_11_RTS_THRESHOLD;

typedef  ULONG   NDIS_802_11_ANTENNA;

typedef enum _NDIS_802_11_PRIVACY_FILTER
{
    Ndis802_11PrivFilterAcceptAll,
    Ndis802_11PrivFilter8021xWEP
} NDIS_802_11_PRIVACY_FILTER, *PNDIS_802_11_PRIVACY_FILTER;

// Added new encryption types
// Also aliased typedef to new name
typedef enum _NDIS_802_11_WEP_STATUS
{
    Ndis802_11WEPEnabled,
    Ndis802_11Encryption1Enabled = Ndis802_11WEPEnabled,
    Ndis802_11WEPDisabled,
    Ndis802_11EncryptionDisabled = Ndis802_11WEPDisabled,
    Ndis802_11WEPKeyAbsent,
    Ndis802_11Encryption1KeyAbsent = Ndis802_11WEPKeyAbsent,
    Ndis802_11WEPNotSupported,
    Ndis802_11EncryptionNotSupported = Ndis802_11WEPNotSupported,
    Ndis802_11Encryption2Enabled,
    Ndis802_11Encryption2KeyAbsent,
    Ndis802_11Encryption3Enabled,
    Ndis802_11Encryption3KeyAbsent
} NDIS_802_11_WEP_STATUS, *PNDIS_802_11_WEP_STATUS,
  NDIS_802_11_ENCRYPTION_STATUS, *PNDIS_802_11_ENCRYPTION_STATUS;

typedef enum _NDIS_802_11_RELOAD_DEFAULTS
{
    Ndis802_11ReloadWEPKeys
} NDIS_802_11_RELOAD_DEFAULTS, *PNDIS_802_11_RELOAD_DEFAULTS;

#define NDIS_802_11_AI_REQFI_CAPABILITIES      1
#define NDIS_802_11_AI_REQFI_LISTENINTERVAL    2
#define NDIS_802_11_AI_REQFI_CURRENTAPADDRESS  4

#define NDIS_802_11_AI_RESFI_CAPABILITIES      1
#define NDIS_802_11_AI_RESFI_STATUSCODE        2
#define NDIS_802_11_AI_RESFI_ASSOCIATIONID     4

typedef struct _NDIS_802_11_AI_REQFI
{
    USHORT Capabilities;
    USHORT ListenInterval;
    NDIS_802_11_MAC_ADDRESS  CurrentAPAddress;
} NDIS_802_11_AI_REQFI, *PNDIS_802_11_AI_REQFI;

typedef struct _NDIS_802_11_AI_RESFI
{
    USHORT Capabilities;
    USHORT StatusCode;
    USHORT AssociationId;
} NDIS_802_11_AI_RESFI, *PNDIS_802_11_AI_RESFI;

typedef struct _NDIS_802_11_ASSOCIATION_INFORMATION
{
    ULONG Length;
    USHORT AvailableRequestFixedIEs;
    NDIS_802_11_AI_REQFI    RequestFixedIEs;
    ULONG RequestIELength;
    ULONG OffsetRequestIEs;
    USHORT AvailableResponseFixedIEs;
    NDIS_802_11_AI_RESFI    ResponseFixedIEs;
    ULONG ResponseIELength;
    ULONG OffsetResponseIEs;
} NDIS_802_11_ASSOCIATION_INFORMATION, *PNDIS_802_11_ASSOCIATION_INFORMATION;

typedef struct _NDIS_802_11_AUTHENTICATION_EVENT
{
    NDIS_802_11_STATUS_INDICATION       Status;
    NDIS_802_11_AUTHENTICATION_REQUEST  Request[1];
} NDIS_802_11_AUTHENTICATION_EVENT, *PNDIS_802_11_AUTHENTICATION_EVENT;

typedef struct _NDIS_802_11_TEST
{
    ULONG Length;
    ULONG Type;
    union
    {
        NDIS_802_11_AUTHENTICATION_EVENT AuthenticationEvent;
        NDIS_802_11_RSSI RssiTrigger;
    };
} NDIS_802_11_TEST, *PNDIS_802_11_TEST;

// 802.11 Media stream constraints, associated with OID_802_11_MEDIA_STREAM_MODE
typedef enum _NDIS_802_11_MEDIA_STREAM_MODE
{
    Ndis802_11MediaStreamOff,
    Ndis802_11MediaStreamOn,
} NDIS_802_11_MEDIA_STREAM_MODE, *PNDIS_802_11_MEDIA_STREAM_MODE;

// PMKID Structures
typedef UCHAR   NDIS_802_11_PMKID_VALUE[16];

typedef struct _BSSID_INFO
{
    NDIS_802_11_MAC_ADDRESS BSSID;
    NDIS_802_11_PMKID_VALUE PMKID;
} BSSID_INFO, *PBSSID_INFO;

typedef struct _NDIS_802_11_PMKID
{
    ULONG Length;
    ULONG BSSIDInfoCount;
    BSSID_INFO BSSIDInfo[1];
} NDIS_802_11_PMKID, *PNDIS_802_11_PMKID;

typedef struct _NDIS_802_11_AUTHENTICATION_ENCRYPTION
{
    NDIS_802_11_AUTHENTICATION_MODE AuthModeSupported;
    NDIS_802_11_ENCRYPTION_STATUS EncryptStatusSupported;
} NDIS_802_11_AUTHENTICATION_ENCRYPTION, *PNDIS_802_11_AUTHENTICATION_ENCRYPTION;

typedef struct _NDIS_802_11_CAPABILITY
{
    ULONG Length;
    ULONG Version;
    ULONG NoOfPMKIDs;
    ULONG NoOfAuthEncryptPairsSupported;
    NDIS_802_11_AUTHENTICATION_ENCRYPTION AuthenticationEncryptionSupported[1];
} NDIS_802_11_CAPABILITY, *PNDIS_802_11_CAPABILITY;

typedef struct _NDIS_802_11_NON_BCAST_SSID_LIST
{
    ULONG               NumberOfItems;
    NDIS_802_11_SSID    Non_Bcast_Ssid[1];
} NDIS_802_11_NON_BCAST_SSID_LIST, *PNDIS_802_11_NON_BCAST_SSID_LIST;

// for OID_802_11_RADIO_STATUS
typedef enum _NDIS_802_11_RADIO_STATUS
{
    Ndis802_11RadioStatusOn,
    Ndis802_11RadioStatusHardwareOff,
    Ndis802_11RadioStatusSoftwareOff,
    Ndis802_11RadioStatusHardwareSoftwareOff,
    Ndis802_11RadioStatusMax      // not a real mode, defined as an upper bound
}
NDIS_802_11_RADIO_STATUS, *PNDIS_802_11_RADIO_STATUS;

//
// IRDA objects
//
#define OID_IRDA_RECEIVING                      0x0A010100
#define OID_IRDA_TURNAROUND_TIME                0x0A010101
#define OID_IRDA_SUPPORTED_SPEEDS               0x0A010102
#define OID_IRDA_LINK_SPEED                     0x0A010103
#define OID_IRDA_MEDIA_BUSY                     0x0A010104

#define OID_IRDA_EXTRA_RCV_BOFS                 0x0A010200
#define OID_IRDA_RATE_SNIFF                     0x0A010201
#define OID_IRDA_UNICAST_LIST                   0x0A010202
#define OID_IRDA_MAX_UNICAST_LIST_SIZE          0x0A010203
#define OID_IRDA_MAX_RECEIVE_WINDOW_SIZE        0x0A010204
#define OID_IRDA_MAX_SEND_WINDOW_SIZE           0x0A010205
#define OID_IRDA_RESERVED1                      0x0A01020A  // The range between OID_IRDA_RESERVED1
#define OID_IRDA_RESERVED2                      0x0A01020F  // and OID_IRDA_RESERVED2 is reserved


//
// IEEE1394 mandatory general OIDs.
//
#define OID_1394_LOCAL_NODE_INFO                0x0C010101
#define OID_1394_VC_INFO                        0x0C010102

//
// The following OIDs are not specific to a media.
//

//
// These are objects for Connection-oriented media call-managers.
//
#define OID_CO_ADD_PVC                          0xFE000001
#define OID_CO_DELETE_PVC                       0xFE000002
#define OID_CO_GET_CALL_INFORMATION             0xFE000003
#define OID_CO_ADD_ADDRESS                      0xFE000004
#define OID_CO_DELETE_ADDRESS                   0xFE000005
#define OID_CO_GET_ADDRESSES                    0xFE000006
#define OID_CO_ADDRESS_CHANGE                   0xFE000007
#define OID_CO_SIGNALING_ENABLED                0xFE000008
#define OID_CO_SIGNALING_DISABLED               0xFE000009
#define OID_CO_AF_CLOSE                         0xFE00000A

//
// Objects for call-managers and MCMs that support TAPI access.
//
#define OID_CO_TAPI_CM_CAPS                     0xFE001001
#define OID_CO_TAPI_LINE_CAPS                   0xFE001002
#define OID_CO_TAPI_ADDRESS_CAPS                0xFE001003
#define OID_CO_TAPI_TRANSLATE_TAPI_CALLPARAMS   0xFE001004
#define OID_CO_TAPI_TRANSLATE_NDIS_CALLPARAMS   0xFE001005
#define OID_CO_TAPI_TRANSLATE_TAPI_SAP          0xFE001006
#define OID_CO_TAPI_GET_CALL_DIAGNOSTICS        0xFE001007
#define OID_CO_TAPI_REPORT_DIGITS               0xFE001008
#define OID_CO_TAPI_DONT_REPORT_DIGITS          0xFE001009

//
//  PnP and PM OIDs
//
#define OID_PNP_CAPABILITIES                    0xFD010100
#define OID_PNP_SET_POWER                       0xFD010101
#define OID_PNP_QUERY_POWER                     0xFD010102
#define OID_PNP_ADD_WAKE_UP_PATTERN             0xFD010103
#define OID_PNP_REMOVE_WAKE_UP_PATTERN          0xFD010104
#define OID_PNP_WAKE_UP_PATTERN_LIST            0xFD010105
#define OID_PNP_ENABLE_WAKE_UP                  0xFD010106

//
//  PnP/PM Statistics (Optional).
//
#define OID_PNP_WAKE_UP_OK                      0xFD020200
#define OID_PNP_WAKE_UP_ERROR                   0xFD020201

#if ((NTDDI_VERSION >= NTDDI_WIN7) || NDIS_SUPPORT_NDIS620)
//
// new power management OIDs for NDIS 6.20 drivers
//

#define OID_PM_CURRENT_CAPABILITIES             0xFD010107
#define OID_PM_HARDWARE_CAPABILITIES            0xFD010108
#define OID_PM_PARAMETERS                       0xFD010109
#define OID_PM_ADD_WOL_PATTERN                  0xFD01010A
#define OID_PM_REMOVE_WOL_PATTERN               0xFD01010B
#define OID_PM_WOL_PATTERN_LIST                 0xFD01010C
#define OID_PM_ADD_PROTOCOL_OFFLOAD             0xFD01010D
#define OID_PM_GET_PROTOCOL_OFFLOAD             0xFD01010E
#define OID_PM_REMOVE_PROTOCOL_OFFLOAD          0xFD01010F
#define OID_PM_PROTOCOL_OFFLOAD_LIST            0xFD010110
#define OID_PM_RESERVED_1                       0xFD010111


//
// new NDIS 6.20 OIDs for generic packet filtering
//
#define OID_RECEIVE_FILTER_HARDWARE_CAPABILITIES        0x00010221  // query only
#define OID_RECEIVE_FILTER_GLOBAL_PARAMETERS            0x00010222  // query only
#define OID_RECEIVE_FILTER_ALLOCATE_QUEUE               0x00010223  // method only
#define OID_RECEIVE_FILTER_FREE_QUEUE                   0x00010224  // set only
#define OID_RECEIVE_FILTER_ENUM_QUEUES                  0x00010225  // query only
#define OID_RECEIVE_FILTER_QUEUE_PARAMETERS             0x00010226  // method and set
#define OID_RECEIVE_FILTER_SET_FILTER                   0x00010227  // method only
#define OID_RECEIVE_FILTER_CLEAR_FILTER                 0x00010228  // set only
#define OID_RECEIVE_FILTER_ENUM_FILTERS                 0x00010229  // method only
#define OID_RECEIVE_FILTER_PARAMETERS                   0x0001022A  // method only
#define OID_RECEIVE_FILTER_QUEUE_ALLOCATION_COMPLETE    0x0001022B  // method only
#define OID_RECEIVE_FILTER_CURRENT_CAPABILITIES         0x0001022D  // query only
#define OID_NIC_SWITCH_HARDWARE_CAPABILITIES            0x0001022E  // query only
#define OID_NIC_SWITCH_CURRENT_CAPABILITIES             0x0001022F  // query only

#if (NDIS_SUPPORT_NDIS630)
#define OID_RECEIVE_FILTER_MOVE_FILTER                  0x00010230  // set only
#endif // (NDIS_SUPPORT_NDIS630)

#define OID_VLAN_RESERVED1                              0x00010231
#define OID_VLAN_RESERVED2                              0x00010232
#define OID_VLAN_RESERVED3                              0x00010233
#define OID_VLAN_RESERVED4                              0x00010234

#if (NDIS_SUPPORT_NDIS630)
#define OID_PACKET_COALESCING_FILTER_MATCH_COUNT        0x00010235  // query only
#endif // (NDIS_SUPPORT_NDIS630)

#endif // ((NTDDI_VERSION >= NTDDI_WIN7) || NDIS_SUPPORT_NDIS620)

#if ((NTDDI_VERSION >= NTDDI_WIN8) || NDIS_SUPPORT_NDIS630)
//
// OIDs used for SRIOV and NIC switch
//
#define OID_NIC_SWITCH_CREATE_SWITCH                  0x00010237  // method only
#define OID_NIC_SWITCH_PARAMETERS                     0x00010238  // method and set only
#define OID_NIC_SWITCH_DELETE_SWITCH                  0x00010239  // set only
#define OID_NIC_SWITCH_ENUM_SWITCHES                  0x00010240  // query only
#define OID_NIC_SWITCH_CREATE_VPORT                   0x00010241  // method only
#define OID_NIC_SWITCH_VPORT_PARAMETERS               0x00010242  // query and set only
#define OID_NIC_SWITCH_ENUM_VPORTS                    0x00010243  // method only
#define OID_NIC_SWITCH_DELETE_VPORT                   0x00010244  // set only
#define OID_NIC_SWITCH_ALLOCATE_VF                    0x00010245  // method only
#define OID_NIC_SWITCH_FREE_VF                        0x00010246  // set only
#define OID_NIC_SWITCH_VF_PARAMETERS                  0x00010247  // method only
#define OID_NIC_SWITCH_ENUM_VFS                       0x00010248  // method only

#define OID_SRIOV_HARDWARE_CAPABILITIES               0x00010249    // query only
#define OID_SRIOV_CURRENT_CAPABILITIES                0x00010250    // query only
#define OID_SRIOV_READ_VF_CONFIG_SPACE                0x00010251    // method only
#define OID_SRIOV_WRITE_VF_CONFIG_SPACE               0x00010252    // set only
#define OID_SRIOV_READ_VF_CONFIG_BLOCK                0x00010253    // method only
#define OID_SRIOV_WRITE_VF_CONFIG_BLOCK               0x00010254    // set only
#define OID_SRIOV_RESET_VF                            0x00010255    // set only
#define OID_SRIOV_SET_VF_POWER_STATE                  0x00010256    // set only
#define OID_SRIOV_VF_VENDOR_DEVICE_ID                 0x00010257    // method only
#define OID_SRIOV_PROBED_BARS                         0x00010258    // query only
#define OID_SRIOV_BAR_RESOURCES                       0x00010259    // method only
#define OID_SRIOV_PF_LUID                             0x00010260    // query only

//
// These OIDs are applicable to the VF only
//
#define OID_SRIOV_CONFIG_STATE                        0x00010261    // set only
#define OID_SRIOV_VF_SERIAL_NUMBER                    0x00010262    // query only
#if (NDIS_SUPPORT_NDIS670)
#define OID_SRIOV_OVERLYING_ADAPTER_INFO              0x00010268    // set only
#endif //(NDIS_SUPPORT_NDIS670)
#define OID_SRIOV_VF_INVALIDATE_CONFIG_BLOCK          0x00010269    // method only

//
// OID's used for Hyper-V extensible switch
//
#define OID_SWITCH_PROPERTY_ADD                       0x00010263   // set only
#define OID_SWITCH_PROPERTY_UPDATE                    0x00010264   // set only
#define OID_SWITCH_PROPERTY_DELETE                    0x00010265   // set only
#define OID_SWITCH_PROPERTY_ENUM                      0x00010266   // method only
#define OID_SWITCH_FEATURE_STATUS_QUERY               0x00010267   // method only

#define OID_SWITCH_NIC_REQUEST                        0x00010270   // method only
#define OID_SWITCH_PORT_PROPERTY_ADD                  0x00010271   // set only
#define OID_SWITCH_PORT_PROPERTY_UPDATE               0x00010272   // set only
#define OID_SWITCH_PORT_PROPERTY_DELETE               0x00010273   // set only
#define OID_SWITCH_PORT_PROPERTY_ENUM                 0x00010274   // method only
#define OID_SWITCH_PARAMETERS                         0x00010275   // query only
#define OID_SWITCH_PORT_ARRAY                         0x00010276   // query only
#define OID_SWITCH_NIC_ARRAY                          0x00010277   // query only
#define OID_SWITCH_PORT_CREATE                        0x00010278   // set only
#define OID_SWITCH_PORT_DELETE                        0x00010279   // set only
#define OID_SWITCH_NIC_CREATE                         0x0001027A   // set only
#define OID_SWITCH_NIC_CONNECT                        0x0001027B   // set only
#define OID_SWITCH_NIC_DISCONNECT                     0x0001027C   // set only
#define OID_SWITCH_NIC_DELETE                         0x0001027D   // set only
#define OID_SWITCH_PORT_FEATURE_STATUS_QUERY          0x0001027E   // method only
#define OID_SWITCH_PORT_TEARDOWN                      0x0001027F   // set only
#define OID_SWITCH_NIC_SAVE                           0x00010290   // method only
#define OID_SWITCH_NIC_SAVE_COMPLETE                  0x00010291   // set only
#define OID_SWITCH_NIC_RESTORE                        0x00010292   // set only
#define OID_SWITCH_NIC_RESTORE_COMPLETE               0x00010293   // set only
#define OID_SWITCH_NIC_UPDATED                        0x00010294   // set only
#define OID_SWITCH_PORT_UPDATED                       0x00010295   // set only

#endif  // ((NTDDI_VERSION >= NTDDI_WIN8) || NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS650)
#define OID_SWITCH_NIC_DIRECT_REQUEST                 0x00010296   // method only
#define OID_SWITCH_NIC_SUSPEND                        0x00010297   // set only
#define OID_SWITCH_NIC_RESUME                         0x00010298   // set only
#endif //(NDIS_SUPPORT_NDIS650)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3) || (NDIS_SUPPORT_NDIS680)

//
// OIDs issued at source node when a suspended LM for a NIC is starting and
// when it it is finished.
//
#define OID_SWITCH_NIC_SUSPENDED_LM_SOURCE_STARTED    0x00010299   // set only
#define OID_SWITCH_NIC_SUSPENDED_LM_SOURCE_FINISHED   0x0001029A   // set only

// RSSv2 direct OID: move an array of indirection table entries
#define OID_GEN_RSS_SET_INDIRECTION_TABLE_ENTRIES     0x000102C0    // method only


#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3) || (NDIS_SUPPORT_NDIS680)

#if (NTDDI_VERSION >= NTDDI_WINBLUE) ||(NDIS_SUPPORT_NDIS640)

#define OID_GEN_ISOLATION_PARAMETERS                  0x00010300   // query only

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE) ||(NDIS_SUPPORT_NDIS640)

#if (NDIS_SUPPORT_NDIS650)

#define OID_GFT_HARDWARE_CAPABILITIES                   0x00010401  // query only
#define OID_GFT_CURRENT_CAPABILITIES                    0x00010402  // query only
#define OID_GFT_GLOBAL_PARAMETERS                       0x00010403  // query and set

#define OID_GFT_CREATE_TABLE                            0x00010404  // method only
#define OID_GFT_DELETE_TABLE                            0x00010405  // set only
#define OID_GFT_ENUM_TABLES                             0x00010406  // query only

#define OID_GFT_ALLOCATE_COUNTERS                       0x00010407  // method only
#define OID_GFT_FREE_COUNTERS                           0x00010408  // set only
#define OID_GFT_ENUM_COUNTERS                           0x00010409  // method
#define OID_GFT_COUNTER_VALUES                          0x0001040A  // direct OID. method

#define OID_GFT_STATISTICS                              0x0001040B  // query only, direct OID

#define OID_GFT_ADD_FLOW_ENTRIES                        0x0001040C  // method only, direct OID
#define OID_GFT_DELETE_FLOW_ENTRIES                     0x0001040D  // method only, direct OID
#define OID_GFT_ENUM_FLOW_ENTRIES                       0x0001040E  // method only
#define OID_GFT_ACTIVATE_FLOW_ENTRIES                   0x0001040F  // set only, direct OID
#define OID_GFT_DEACTIVATE_FLOW_ENTRIES                 0x00010410  // set only, direct OID
#define OID_GFT_FLOW_ENTRY_PARAMETERS                   0x00010411  // method only, direct OID

#define OID_GFT_EXACT_MATCH_PROFILE                     0x00010412  // method
#define OID_GFT_HEADER_TRANSPOSITION_PROFILE            0x00010413  // method
#define OID_GFT_WILDCARD_MATCH_PROFILE                  0x00010414  // method
#define OID_GFT_ENUM_PROFILES                           0x00010415  // query only
#define OID_GFT_DELETE_PROFILE                          0x00010416  // set only

#define OID_GFT_VPORT_PARAMETERS                        0x00010417  // query and set

#define OID_GFT_CREATE_LOGICAL_VPORT                    0x00010418  // method
#define OID_GFT_DELETE_LOGICAL_VPORT                    0x00010419  // set
#define OID_GFT_ENUM_LOGICAL_VPORTS                     0x0001041A  // query

#define OID_QOS_OFFLOAD_HARDWARE_CAPABILITIES           0x00010601  // query only
#define OID_QOS_OFFLOAD_CURRENT_CAPABILITIES            0x00010602  // query only

#define OID_QOS_OFFLOAD_CREATE_SQ                       0x00010603  // set only
#define OID_QOS_OFFLOAD_DELETE_SQ                       0x00010604  // set only
#define OID_QOS_OFFLOAD_UPDATE_SQ                       0x00010605  // set only
#define OID_QOS_OFFLOAD_ENUM_SQS                        0x00010606  // method only
#if (NDIS_SUPPORT_NDIS684)
#define OID_QOS_OFFLOAD_SQ_STATS                        0x00010607  // method only
#endif

#define OID_PD_OPEN_PROVIDER                            0x00010501  // method
#define OID_PD_CLOSE_PROVIDER                           0x00010502  // set only
#define OID_PD_QUERY_CURRENT_CONFIG                     0x00010503  // query

#endif // (NDIS_SUPPORT_NDIS650)


//
//  The following bits are defined for OID_PNP_ENABLE_WAKE_UP
//
#define NDIS_PNP_WAKE_UP_MAGIC_PACKET           0x00000001
#define NDIS_PNP_WAKE_UP_PATTERN_MATCH          0x00000002
#define NDIS_PNP_WAKE_UP_LINK_CHANGE            0x00000004

//
//  TCP/IP OIDs
//
#define OID_TCP_TASK_OFFLOAD                    0xFC010201
#define OID_TCP_TASK_IPSEC_ADD_SA               0xFC010202
#define OID_TCP_TASK_IPSEC_DELETE_SA            0xFC010203
#define OID_TCP_SAN_SUPPORT                     0xFC010204
#define OID_TCP_TASK_IPSEC_ADD_UDPESP_SA        0xFC010205
#define OID_TCP_TASK_IPSEC_DELETE_UDPESP_SA     0xFC010206
#define OID_TCP4_OFFLOAD_STATS                  0xFC010207
#define OID_TCP6_OFFLOAD_STATS                  0xFC010208
#define OID_IP4_OFFLOAD_STATS                   0xFC010209
#define OID_IP6_OFFLOAD_STATS                   0xFC01020A

//
// new offload OIDs for NDIS 6
//
#define OID_TCP_OFFLOAD_CURRENT_CONFIG          0xFC01020B          // query only, handled by NDIS
#define OID_TCP_OFFLOAD_PARAMETERS              0xFC01020C          // set only
#define OID_TCP_OFFLOAD_HARDWARE_CAPABILITIES   0xFC01020D          // query only
#define OID_TCP_CONNECTION_OFFLOAD_CURRENT_CONFIG 0xFC01020E        // query only
#define OID_TCP_CONNECTION_OFFLOAD_HARDWARE_CAPABILITIES 0xFC01020F // query only
#define OID_OFFLOAD_ENCAPSULATION               0x0101010A

#if (NDIS_SUPPORT_NDIS61)
//
// IPsec Task offload V2 OIDs
//
#define OID_TCP_TASK_IPSEC_OFFLOAD_V2_ADD_SA    0xFC030202
#define OID_TCP_TASK_IPSEC_OFFLOAD_V2_DELETE_SA 0xFC030203
#define OID_TCP_TASK_IPSEC_OFFLOAD_V2_UPDATE_SA 0xFC030204
#endif // (NDIS_SUPPORT_NDIS61)
#if (NDIS_SUPPORT_NDIS630)
#define OID_TCP_TASK_IPSEC_OFFLOAD_V2_ADD_SA_EX    0xFC030205
#endif // (NDIS_SUPPORT_NDIS630)

//
//  Defines for FFP, obsolete
//
#define OID_FFP_SUPPORT                         0xFC010210
#define OID_FFP_FLUSH                           0xFC010211
#define OID_FFP_CONTROL                         0xFC010212
#define OID_FFP_PARAMS                          0xFC010213
#define OID_FFP_DATA                            0xFC010214

#define OID_FFP_DRIVER_STATS                    0xFC020210
#define OID_FFP_ADAPTER_STATS                   0xFC020211


//
// TCP Connection offload OID
//
#define OID_TCP_CONNECTION_OFFLOAD_PARAMETERS   0xFC030201

#if ((NTDDI_VERSION >= NTDDI_WIN7) || NDIS_SUPPORT_NDIS620)
//
// New Tunnel driver direct OIDs for NDIS 6.20
//
#define OID_TUNNEL_INTERFACE_SET_OID            0x0f010106
#define OID_TUNNEL_INTERFACE_RELEASE_OID        0x0f010107
#endif // ((NTDDI_VERSION >= NTDDI_WIN7) || NDIS_SUPPORT_NDIS620)


//
//  Defines for QOS
//
#define OID_QOS_RESERVED1                       0xFB010100
#define OID_QOS_RESERVED2                       0xFB010101
#define OID_QOS_RESERVED3                       0xFB010102
#define OID_QOS_RESERVED4                       0xFB010103
#define OID_QOS_RESERVED5                       0xFB010104
#define OID_QOS_RESERVED6                       0xFB010105
#define OID_QOS_RESERVED7                       0xFB010106
#define OID_QOS_RESERVED8                       0xFB010107
#define OID_QOS_RESERVED9                       0xFB010108
#define OID_QOS_RESERVED10                      0xFB010109
#define OID_QOS_RESERVED11                      0xFB01010A
#define OID_QOS_RESERVED12                      0xFB01010B
#define OID_QOS_RESERVED13                      0xFB01010C
#define OID_QOS_RESERVED14                      0xFB01010D
#define OID_QOS_RESERVED15                      0xFB01010E
#define OID_QOS_RESERVED16                      0xFB01010F
#define OID_QOS_RESERVED17                      0xFB010110
#define OID_QOS_RESERVED18                      0xFB010111
#define OID_QOS_RESERVED19                      0xFB010112
#define OID_QOS_RESERVED20                      0xFB010113


#if ((NTDDI_VERSION >= NTDDI_WINTHRESHOLD) || NDIS_SUPPORT_NDIS650)

//
// OIDs reserved for Xbox accessories
//
#define OID_XBOX_ACC_RESERVED0                  0xFA000000
// OIDs 0xFA000000 through 0xFAFFFFFF are reserved
// #define OID_XBOX_ACC_RESERVED_LAST           0xFAFFFFFF


#endif // THRESHOLD


//
// NDIS Proxy OID_GEN_CO_DEVICE_PROFILE structure. The optional OID and
// this structure is a generic means of describing a CO device's
// capabilities, and is used by the NDIS Proxy to construct a TAPI device
// capabilities structure.
//
typedef struct NDIS_CO_DEVICE_PROFILE
{
    NDIS_VAR_DATA_DESC  DeviceDescription;  // e.g. 'GigabitATMNet'
    NDIS_VAR_DATA_DESC  DevSpecificInfo;    // special features

    ULONG   ulTAPISupplementaryPassThru;// reserved in NT5
    ULONG   ulAddressModes;
    ULONG   ulNumAddresses;
    ULONG   ulBearerModes;
    ULONG   ulMaxTxRate; // bytes per second
    ULONG   ulMinTxRate; // bytes per second
    ULONG   ulMaxRxRate; // bytes per second
    ULONG   ulMinRxRate; // bytes per second
    ULONG   ulMediaModes;

    //
    // Tone/digit generation and recognition capabilities
    //
    ULONG   ulGenerateToneModes;
    ULONG   ulGenerateToneMaxNumFreq;
    ULONG   ulGenerateDigitModes;
    ULONG   ulMonitorToneMaxNumFreq;
    ULONG   ulMonitorToneMaxNumEntries;
    ULONG   ulMonitorDigitModes;
    ULONG   ulGatherDigitsMinTimeout;// milliseconds
    ULONG   ulGatherDigitsMaxTimeout;// milliseconds

    ULONG   ulDevCapFlags;          // Misc. capabilities
    ULONG   ulMaxNumActiveCalls;    // (This * ulMinRate) = total bandwidth (which may equal ulMaxRate)
    ULONG   ulAnswerMode;           // Effect of answering a new call when an
                                    // existing call is non-idle
    //
    // User-User info sizes allowed to accompany each operation
    //
    ULONG   ulUUIAcceptSize;    // bytes
    ULONG   ulUUIAnswerSize;    // bytes
    ULONG   ulUUIMakeCallSize;  // bytes
    ULONG   ulUUIDropSize;      // bytes
    ULONG   ulUUISendUserUserInfoSize; // bytes
    ULONG   ulUUICallInfoSize;  // bytes

} NDIS_CO_DEVICE_PROFILE, *PNDIS_CO_DEVICE_PROFILE;

//
//  Structures for TCP IPsec.
//
#ifndef IP_EXPORT_INCLUDED
typedef ULONG   IPAddr, IPMask;
#endif
typedef ULONG   SPI_TYPE;

typedef enum    _OFFLOAD_OPERATION_E
{
    AUTHENTICATE = 1,
    ENCRYPT
}
    OFFLOAD_OPERATION_E;

typedef struct _OFFLOAD_ALGO_INFO
{
    ULONG   algoIdentifier;
    ULONG   algoKeylen;
    ULONG   algoRounds;
}
    OFFLOAD_ALGO_INFO,
    *POFFLOAD_ALGO_INFO;

typedef enum _OFFLOAD_CONF_ALGO
{
    OFFLOAD_IPSEC_CONF_NONE,
    OFFLOAD_IPSEC_CONF_DES,
    OFFLOAD_IPSEC_CONF_RESERVED,
    OFFLOAD_IPSEC_CONF_3_DES,
    OFFLOAD_IPSEC_CONF_MAX
}
    OFFLOAD_CONF_ALGO;

typedef enum _OFFLOAD_INTEGRITY_ALGO
{
    OFFLOAD_IPSEC_INTEGRITY_NONE,
    OFFLOAD_IPSEC_INTEGRITY_MD5,
    OFFLOAD_IPSEC_INTEGRITY_SHA,
    OFFLOAD_IPSEC_INTEGRITY_MAX
}
    OFFLOAD_INTEGRITY_ALGO;

typedef struct _OFFLOAD_SECURITY_ASSOCIATION
{
    OFFLOAD_OPERATION_E     Operation;
    SPI_TYPE                SPI;
    OFFLOAD_ALGO_INFO       IntegrityAlgo;
    OFFLOAD_ALGO_INFO       ConfAlgo;
    OFFLOAD_ALGO_INFO       Reserved;
}
    OFFLOAD_SECURITY_ASSOCIATION,
    *POFFLOAD_SECURITY_ASSOCIATION;

#define OFFLOAD_MAX_SAS             3

#define OFFLOAD_INBOUND_SA          0x0001
#define OFFLOAD_OUTBOUND_SA         0x0002

typedef struct _OFFLOAD_IPSEC_ADD_SA
{
    IPAddr                          SrcAddr;
    IPMask                          SrcMask;
    IPAddr                          DestAddr;
    IPMask                          DestMask;
    ULONG                           Protocol;
    USHORT                          SrcPort;
    USHORT                          DestPort;
    IPAddr                          SrcTunnelAddr;
    IPAddr                          DestTunnelAddr;
    USHORT                          Flags;
    SHORT                           NumSAs;
    OFFLOAD_SECURITY_ASSOCIATION    SecAssoc[OFFLOAD_MAX_SAS];
    HANDLE                          OffloadHandle;
    ULONG                           KeyLen;
    UCHAR                           KeyMat[1];
} OFFLOAD_IPSEC_ADD_SA, *POFFLOAD_IPSEC_ADD_SA;

typedef struct _OFFLOAD_IPSEC_DELETE_SA
{
    HANDLE                          OffloadHandle;
} OFFLOAD_IPSEC_DELETE_SA, *POFFLOAD_IPSEC_DELETE_SA;


typedef enum _UDP_ENCAP_TYPE
{
    OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_IKE,
    OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_OTHER
} UDP_ENCAP_TYPE, * PUDP_ENCAP_TYPE;


typedef struct _OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY
{
    UDP_ENCAP_TYPE                  UdpEncapType;
    USHORT                          DstEncapPort;
} OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY, * POFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY;


typedef struct _OFFLOAD_IPSEC_ADD_UDPESP_SA
{
    IPAddr                                  SrcAddr;
    IPMask                                  SrcMask;
    IPAddr                                  DstAddr;
    IPMask                                  DstMask;
    ULONG                                   Protocol;
    USHORT                                  SrcPort;
    USHORT                                  DstPort;
    IPAddr                                  SrcTunnelAddr;
    IPAddr                                  DstTunnelAddr;
    USHORT                                  Flags;
    SHORT                                   NumSAs;
    OFFLOAD_SECURITY_ASSOCIATION            SecAssoc[OFFLOAD_MAX_SAS];
    HANDLE                                  OffloadHandle;
    OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY    EncapTypeEntry;
    HANDLE                                  EncapTypeEntryOffldHandle;
    ULONG                                   KeyLen;
    UCHAR                                   KeyMat[1];
} OFFLOAD_IPSEC_ADD_UDPESP_SA, * POFFLOAD_IPSEC_ADD_UDPESP_SA;


typedef struct _OFFLOAD_IPSEC_DELETE_UDPESP_SA
{
    HANDLE                                  OffloadHandle;
    HANDLE                                  EncapTypeEntryOffldHandle;
} OFFLOAD_IPSEC_DELETE_UDPESP_SA, * POFFLOAD_IPSEC_DELETE_UDPESP_SA;


//
// Type to go with OID_GEN_VLAN_ID: the least significant 12 bits are
// used as the VLAN ID (VID) per IEEE 802.1Q. Higher order bits are
// reserved and must be set to 0.
//
typedef ULONG NDIS_VLAN_ID;

//
// Medium the Ndis Driver is running on (OID_GEN_MEDIA_SUPPORTED/ OID_GEN_MEDIA_IN_USE).
//
typedef enum _NDIS_MEDIUM
{
    NdisMedium802_3,
    NdisMedium802_5,
    NdisMediumFddi,
    NdisMediumWan,
    NdisMediumLocalTalk,
    NdisMediumDix,              // defined for convenience, not a real medium
    NdisMediumArcnetRaw,
    NdisMediumArcnet878_2,
    NdisMediumAtm,
    NdisMediumWirelessWan,
    NdisMediumIrda,
    NdisMediumBpc,
    NdisMediumCoWan,
    NdisMedium1394,
    NdisMediumInfiniBand,
#if ((NTDDI_VERSION >= NTDDI_VISTA) || NDIS_SUPPORT_NDIS6)
    NdisMediumTunnel,
    NdisMediumNative802_11,
    NdisMediumLoopback,
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN7)
    NdisMediumWiMAX,
    NdisMediumIP,
#endif

    NdisMediumMax               // Not a real medium, defined as an upper-bound
} NDIS_MEDIUM, *PNDIS_MEDIUM;


//
// Physical Medium Type definitions. Used with OID_GEN_PHYSICAL_MEDIUM.
//
typedef enum _NDIS_PHYSICAL_MEDIUM
{
    NdisPhysicalMediumUnspecified,
    NdisPhysicalMediumWirelessLan,
    NdisPhysicalMediumCableModem,
    NdisPhysicalMediumPhoneLine,
    NdisPhysicalMediumPowerLine,
    NdisPhysicalMediumDSL,      // includes ADSL and UADSL (G.Lite)
    NdisPhysicalMediumFibreChannel,
    NdisPhysicalMedium1394,
    NdisPhysicalMediumWirelessWan,
    NdisPhysicalMediumNative802_11,
    NdisPhysicalMediumBluetooth,
    NdisPhysicalMediumInfiniband,
    NdisPhysicalMediumWiMax,
    NdisPhysicalMediumUWB,
    NdisPhysicalMedium802_3,
    NdisPhysicalMedium802_5,
    NdisPhysicalMediumIrda,
    NdisPhysicalMediumWiredWAN,
    NdisPhysicalMediumWiredCoWan,
    NdisPhysicalMediumOther,
    NdisPhysicalMediumNative802_15_4,
    NdisPhysicalMediumMax       // Not a real physical type, defined as an upper-bound
} NDIS_PHYSICAL_MEDIUM, *PNDIS_PHYSICAL_MEDIUM;


//
//  Protocol types supported by ndis. These values need to be consistent with ADDRESS_TYPE_XXX defined in TDI.H
//
#define NDIS_PROTOCOL_ID_DEFAULT        0x00
#define NDIS_PROTOCOL_ID_TCP_IP         0x02
#define NDIS_PROTOCOL_ID_IP6            0x03
#define NDIS_PROTOCOL_ID_IPX            0x06
#define NDIS_PROTOCOL_ID_NBF            0x07
#define NDIS_PROTOCOL_ID_MAX            0x0F
#define NDIS_PROTOCOL_ID_MASK           0x0F

//
// The following is used with OID_GEN_TRANSPORT_HEADER_OFFSET to indicate the length of the layer-2 header
// for packets sent by a particular protocol.
//
typedef struct _TRANSPORT_HEADER_OFFSET
{
    USHORT      ProtocolType;       // The protocol that is sending this OID (NDIS_PROTOCOL_ID_XXX above)
    USHORT      HeaderOffset;       // The header offset
} TRANSPORT_HEADER_OFFSET, *PTRANSPORT_HEADER_OFFSET;


//
// The structures below need to be consistent with TRANSPORT_ADDRESS structures in TDI.H
//
typedef struct _NETWORK_ADDRESS
{
    USHORT      AddressLength;      // length in bytes of Address[] in this
    USHORT      AddressType;        // type of this address (NDIS_PROTOCOL_ID_XXX above)
    UCHAR       Address[1];         // actually AddressLength bytes long
} NETWORK_ADDRESS, *PNETWORK_ADDRESS;

//
// The following is used with OID_GEN_NETWORK_LAYER_ADDRESSES to set network layer addresses on an interface
//
typedef struct _NETWORK_ADDRESS_LIST
{
    LONG        AddressCount;       // number of addresses following
    USHORT      AddressType;        // type of this address (NDIS_PROTOCOL_ID_XXX above)
    NETWORK_ADDRESS Address[1];     // actually AddressCount elements long
} NETWORK_ADDRESS_LIST, *PNETWORK_ADDRESS_LIST;

//
// IP address - This must remain consistent with TDI_ADDRESS_IP in tdi.h
//
typedef struct _NETWORK_ADDRESS_IP
{
    USHORT      sin_port;
    ULONG       in_addr;
    UCHAR       sin_zero[8];
} NETWORK_ADDRESS_IP, *PNETWORK_ADDRESS_IP;

#define NETWORK_ADDRESS_LENGTH_IP sizeof (NETWORK_ADDRESS_IP)

//
// IPv6 address - This must remain consistent with TDI_ADDRESS_IP6 in tdi.h
//
typedef struct _NETWORK_ADDRESS_IP6 {
    USHORT      sin6_port;
    ULONG       sin6_flowinfo;
    USHORT      sin6_addr[8];
    ULONG       sin6_scope_id;
} NETWORK_ADDRESS_IP6, *PNETWORK_ADDRESS_IP6;

#define NETWORK_ADDRESS_LENGTH_IP6 sizeof (NETWORK_ADDRESS_IP6)

//
// IPX address - This must remain consistent with TDI_ADDRESS_IPX in tdi.h.
//
typedef struct _NETWORK_ADDRESS_IPX
{
    ULONG       NetworkAddress;
    UCHAR       NodeAddress[6];
    USHORT      Socket;
} NETWORK_ADDRESS_IPX, *PNETWORK_ADDRESS_IPX;

#define NETWORK_ADDRESS_LENGTH_IPX sizeof (NETWORK_ADDRESS_IPX)

//
// Hardware status codes (OID_GEN_HARDWARE_STATUS).
//

typedef enum _NDIS_HARDWARE_STATUS
{
    NdisHardwareStatusReady,
    NdisHardwareStatusInitializing,
    NdisHardwareStatusReset,
    NdisHardwareStatusClosing,
    NdisHardwareStatusNotReady
} NDIS_HARDWARE_STATUS, *PNDIS_HARDWARE_STATUS;


//
// this is the type passed in the OID_GEN_GET_TIME_CAPS request
//
typedef struct _GEN_GET_TIME_CAPS
{
    ULONG                       Flags;  // Bits defined below
    ULONG                       ClockPrecision;
} GEN_GET_TIME_CAPS, *PGEN_GET_TIME_CAPS;

#define READABLE_LOCAL_CLOCK                    0x00000001
#define CLOCK_NETWORK_DERIVED                   0x00000002
#define CLOCK_PRECISION                         0x00000004
#define RECEIVE_TIME_INDICATION_CAPABLE         0x00000008
#define TIMED_SEND_CAPABLE                      0x00000010
#define TIME_STAMP_CAPABLE                      0x00000020

//
// this is the type passed in the OID_GEN_GET_NETCARD_TIME request
//
typedef struct _GEN_GET_NETCARD_TIME
{
    ULONGLONG                   ReadTime;
} GEN_GET_NETCARD_TIME, *PGEN_GET_NETCARD_TIME;

//
//  NDIS PnP routines and definitions.
//
typedef struct _NDIS_PM_PACKET_PATTERN
{
    ULONG   Priority;                   // Importance of the given pattern.
    ULONG   Reserved;                   // Context information for transports.
    ULONG   MaskSize;                   // Size in bytes of the pattern mask.
    ULONG   PatternOffset;              // Offset from beginning of this
                                        // structure to the pattern bytes.
    ULONG   PatternSize;                // Size in bytes of the pattern.
    ULONG   PatternFlags;               // Flags (TBD).
} NDIS_PM_PACKET_PATTERN, *PNDIS_PM_PACKET_PATTERN;


//
//  The following structure defines the device power states.
//
typedef enum _NDIS_DEVICE_POWER_STATE
{
    NdisDeviceStateUnspecified = 0,
    NdisDeviceStateD0,
    NdisDeviceStateD1,
    NdisDeviceStateD2,
    NdisDeviceStateD3,
    NdisDeviceStateMaximum
} NDIS_DEVICE_POWER_STATE, *PNDIS_DEVICE_POWER_STATE;

//
//  The following structure defines the wake-up capabilities of the device.
//
typedef struct _NDIS_PM_WAKE_UP_CAPABILITIES
{
    NDIS_DEVICE_POWER_STATE MinMagicPacketWakeUp;
    NDIS_DEVICE_POWER_STATE MinPatternWakeUp;
    NDIS_DEVICE_POWER_STATE MinLinkChangeWakeUp;
} NDIS_PM_WAKE_UP_CAPABILITIES, *PNDIS_PM_WAKE_UP_CAPABILITIES;

//
// the following flags define the -enabled- wake-up capabilities of the device
// passed in the Flags field of NDIS_PNP_CAPABILITIES structure
//
#define NDIS_DEVICE_WAKE_UP_ENABLE                          0x00000001
#define NDIS_DEVICE_WAKE_ON_PATTERN_MATCH_ENABLE            0x00000002
#define NDIS_DEVICE_WAKE_ON_MAGIC_PACKET_ENABLE             0x00000004

//
//  This structure defines general PnP capabilities of the miniport driver.
//
typedef struct _NDIS_PNP_CAPABILITIES
{
    ULONG                           Flags;
    NDIS_PM_WAKE_UP_CAPABILITIES    WakeUpCapabilities;
} NDIS_PNP_CAPABILITIES, *PNDIS_PNP_CAPABILITIES;

//
// Defines the attachment types for FDDI (OID_FDDI_ATTACHMENT_TYPE).
//
typedef enum _NDIS_FDDI_ATTACHMENT_TYPE
{
    NdisFddiTypeIsolated = 1,
    NdisFddiTypeLocalA,
    NdisFddiTypeLocalB,
    NdisFddiTypeLocalAB,
    NdisFddiTypeLocalS,
    NdisFddiTypeWrapA,
    NdisFddiTypeWrapB,
    NdisFddiTypeWrapAB,
    NdisFddiTypeWrapS,
    NdisFddiTypeCWrapA,
    NdisFddiTypeCWrapB,
    NdisFddiTypeCWrapS,
    NdisFddiTypeThrough
} NDIS_FDDI_ATTACHMENT_TYPE, *PNDIS_FDDI_ATTACHMENT_TYPE;


//
// Defines the ring management states for FDDI (OID_FDDI_RING_MGT_STATE).
//
typedef enum _NDIS_FDDI_RING_MGT_STATE
{
    NdisFddiRingIsolated = 1,
    NdisFddiRingNonOperational,
    NdisFddiRingOperational,
    NdisFddiRingDetect,
    NdisFddiRingNonOperationalDup,
    NdisFddiRingOperationalDup,
    NdisFddiRingDirected,
    NdisFddiRingTrace
} NDIS_FDDI_RING_MGT_STATE, *PNDIS_FDDI_RING_MGT_STATE;


//
// Defines the Lconnection state for FDDI (OID_FDDI_LCONNECTION_STATE).
//
typedef enum _NDIS_FDDI_LCONNECTION_STATE
{
    NdisFddiStateOff = 1,
    NdisFddiStateBreak,
    NdisFddiStateTrace,
    NdisFddiStateConnect,
    NdisFddiStateNext,
    NdisFddiStateSignal,
    NdisFddiStateJoin,
    NdisFddiStateVerify,
    NdisFddiStateActive,
    NdisFddiStateMaintenance
} NDIS_FDDI_LCONNECTION_STATE, *PNDIS_FDDI_LCONNECTION_STATE;


//
// Defines the medium subtypes for WAN medium (OID_WAN_MEDIUM_SUBTYPE).
// Sub-medium used only by connection-oriented WAN devices
// i.e. NdisMediumWan, NdisMediumCoWan.
//
typedef enum _NDIS_WAN_MEDIUM_SUBTYPE
{
    NdisWanMediumHub,
    NdisWanMediumX_25,
    NdisWanMediumIsdn,
    NdisWanMediumSerial,
    NdisWanMediumFrameRelay,
    NdisWanMediumAtm,
    NdisWanMediumSonet,
    NdisWanMediumSW56K,
    NdisWanMediumPPTP,
    NdisWanMediumL2TP,
    NdisWanMediumIrda,
    NdisWanMediumParallel,
    NdisWanMediumPppoe,
#if (NTDDI_VERSION >= NTDDI_VISTA)
    NdisWanMediumSSTP,
    NdisWanMediumAgileVPN,
#endif //(NTDDI_VERSION >= NTDDI_VISTA)
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
    NdisWanMediumGre,
#endif //(NTDDI_VERSION >= NTDDI_WINBLUE)
    NdisWanMediumSubTypeMax
} NDIS_WAN_MEDIUM_SUBTYPE, *PNDIS_WAN_MEDIUM_SUBTYPE;


//
// Defines the header format for WAN medium (OID_WAN_HEADER_FORMAT).
//
typedef enum _NDIS_WAN_HEADER_FORMAT
{
    NdisWanHeaderNative,        // src/dest based on subtype, followed by NLPID
    NdisWanHeaderEthernet       // emulation of ethernet header
} NDIS_WAN_HEADER_FORMAT, *PNDIS_WAN_HEADER_FORMAT;


//
// Defines the line quality on a WAN line (OID_WAN_QUALITY_OF_SERVICE).
//
typedef enum _NDIS_WAN_QUALITY
{
    NdisWanRaw,
    NdisWanErrorControl,
    NdisWanReliable
} NDIS_WAN_QUALITY, *PNDIS_WAN_QUALITY;


//
// Defines a protocol's WAN specific capabilities (OID_WAN_PROTOCOL_CAPS).
//
typedef struct _NDIS_WAN_PROTOCOL_CAPS
{
    _In_  ULONG   Flags;
    _In_  ULONG   Reserved;
} NDIS_WAN_PROTOCOL_CAPS, *PNDIS_WAN_PROTOCOL_CAPS;


//
// Flags used in NDIS_WAN_PROTOCOL_CAPS
//
#define WAN_PROTOCOL_KEEPS_STATS    0x00000001


//
// Defines the state of a token-ring adapter (OID_802_5_CURRENT_RING_STATE).
//
typedef enum _NDIS_802_5_RING_STATE
{
    NdisRingStateOpened = 1,
    NdisRingStateClosed,
    NdisRingStateOpening,
    NdisRingStateClosing,
    NdisRingStateOpenFailure,
    NdisRingStateRingFailure
} NDIS_802_5_RING_STATE, *PNDIS_802_5_RING_STATE;

//
// Defines the state of the LAN media
//
typedef enum _NDIS_MEDIA_STATE
{
    NdisMediaStateConnected,
    NdisMediaStateDisconnected
} NDIS_MEDIA_STATE, *PNDIS_MEDIA_STATE;

//
// The following is set on a per-packet basis as OOB data with NdisClass802_3Priority
//
typedef ULONG   Priority_802_3;         // 0-7 priority levels

//
//  The following structure is used to query OID_GEN_CO_LINK_SPEED and
//  OID_GEN_CO_MINIMUM_LINK_SPEED. The first OID will return the current
//  link speed of the adapter. The second will return the minimum link speed
//  the adapter is capable of.
//
typedef struct _NDIS_CO_LINK_SPEED
{
    ULONG   Outbound;
    ULONG   Inbound;
} NDIS_CO_LINK_SPEED, *PNDIS_CO_LINK_SPEED;

#if ((NTDDI_VERSION >= NTDDI_VISTA) || NDIS_SUPPORT_NDIS6)

//
// The following structure is used in OID_GEN_LINK_SPEED_EX for interfaces
// and is expressed in bits per second.
//
typedef struct _NDIS_LINK_SPEED
{
    ULONG64     XmitLinkSpeed;
    ULONG64     RcvLinkSpeed;
} NDIS_LINK_SPEED, *PNDIS_LINK_SPEED;



// miniports or interfaces that do not know their current link speed
// can report NDIS_LINK_SPEED_UNKNOWN == ((ULONG64)-1)
#define NDIS_LINK_SPEED_UNKNOWN NET_IF_LINK_SPEED_UNKNOWN
#endif

#include <ndis/types.h>

#if ((NTDDI_VERSION >= NTDDI_VISTA) || NDIS_SUPPORT_NDIS6)
// Native 802.11 Definitions
#ifndef __WINDOT11_H__
#include <windot11.h>
#endif
#endif // (NTDDI_VERSION >= NTDDI_VISTA)


//
//  Structure to be used for OID_GEN_SUPPORTED_GUIDS.
//  This structure describes an OID to GUID mapping.
//  Or a Status to GUID mapping.
//  When ndis receives a request for a give GUID it will
//  query the miniport with the supplied OID.
//
typedef struct _NDIS_GUID
{
    GUID            Guid;
    union
    {
        NDIS_OID    Oid;
        NDIS_STATUS Status;
    };
    ULONG       Size;               //  Size of the data element. If the GUID
                                    //  represents an array then this is the
                                    //  size of an element in the array.
                                    //  This is -1 for strings.
    ULONG       Flags;
} NDIS_GUID, *PNDIS_GUID;

#define fNDIS_GUID_TO_OID                   0x00000001
#define fNDIS_GUID_TO_STATUS                0x00000002
#define fNDIS_GUID_ANSI_STRING              0x00000004
#define fNDIS_GUID_UNICODE_STRING           0x00000008
#define fNDIS_GUID_ARRAY                    0x00000010
#define fNDIS_GUID_ALLOW_READ               0x00000020
#define fNDIS_GUID_ALLOW_WRITE              0x00000040
#define fNDIS_GUID_METHOD                   0x00000080
#define fNDIS_GUID_NDIS_RESERVED            0x00000100
#define fNDIS_GUID_SUPPORT_COMMON_HEADER    0x00000200

//
// Ndis Packet Filter Bits (OID_GEN_CURRENT_PACKET_FILTER).
//
#define NDIS_PACKET_TYPE_DIRECTED               0x00000001
#define NDIS_PACKET_TYPE_MULTICAST              0x00000002
#define NDIS_PACKET_TYPE_ALL_MULTICAST          0x00000004
#define NDIS_PACKET_TYPE_BROADCAST              0x00000008
#define NDIS_PACKET_TYPE_SOURCE_ROUTING         0x00000010
#define NDIS_PACKET_TYPE_PROMISCUOUS            0x00000020
#define NDIS_PACKET_TYPE_SMT                    0x00000040
#define NDIS_PACKET_TYPE_ALL_LOCAL              0x00000080
#define NDIS_PACKET_TYPE_GROUP                  0x00001000
#define NDIS_PACKET_TYPE_ALL_FUNCTIONAL         0x00002000
#define NDIS_PACKET_TYPE_FUNCTIONAL             0x00004000
#define NDIS_PACKET_TYPE_MAC_FRAME              0x00008000
#define NDIS_PACKET_TYPE_NO_LOCAL               0x00010000


//
// Ndis Token-Ring Ring Status Codes (OID_802_5_CURRENT_RING_STATUS).
//
#define NDIS_RING_SIGNAL_LOSS                   0x00008000
#define NDIS_RING_HARD_ERROR                    0x00004000
#define NDIS_RING_SOFT_ERROR                    0x00002000
#define NDIS_RING_TRANSMIT_BEACON               0x00001000
#define NDIS_RING_LOBE_WIRE_FAULT               0x00000800
#define NDIS_RING_AUTO_REMOVAL_ERROR            0x00000400
#define NDIS_RING_REMOVE_RECEIVED               0x00000200
#define NDIS_RING_COUNTER_OVERFLOW              0x00000100
#define NDIS_RING_SINGLE_STATION                0x00000080
#define NDIS_RING_RING_RECOVERY                 0x00000040


//
// Ndis protocol option bits (OID_GEN_PROTOCOL_OPTIONS).
//
#define NDIS_PROT_OPTION_ESTIMATED_LENGTH               0x00000001
#define NDIS_PROT_OPTION_NO_LOOPBACK                    0x00000002
#define NDIS_PROT_OPTION_NO_RSVD_ON_RCVPKT              0x00000004
#define NDIS_PROT_OPTION_SEND_RESTRICTED                0x00000008

//
// Ndis MAC option bits (OID_GEN_MAC_OPTIONS).
//
#define NDIS_MAC_OPTION_COPY_LOOKAHEAD_DATA             0x00000001
#define NDIS_MAC_OPTION_RECEIVE_SERIALIZED              0x00000002
#define NDIS_MAC_OPTION_TRANSFERS_NOT_PEND              0x00000004
#define NDIS_MAC_OPTION_NO_LOOPBACK                     0x00000008

//
// This flag has been deprecated. Deserialized drivers are
// full duplex drivers
//
#define NDIS_MAC_OPTION_FULL_DUPLEX                     0x00000010  // deprecated


#define NDIS_MAC_OPTION_EOTX_INDICATION                 0x00000020
#define NDIS_MAC_OPTION_8021P_PRIORITY                  0x00000040
#define NDIS_MAC_OPTION_SUPPORTS_MAC_ADDRESS_OVERWRITE  0x00000080
#define NDIS_MAC_OPTION_RECEIVE_AT_DPC                  0x00000100
#define NDIS_MAC_OPTION_8021Q_VLAN                      0x00000200
#define NDIS_MAC_OPTION_RESERVED                        0x80000000

//
//  NDIS media capabilities bits (OID_GEN_MEDIA_CAPABILITIES).
//
#define NDIS_MEDIA_CAP_TRANSMIT                 0x00000001  // Supports sending data
#define NDIS_MEDIA_CAP_RECEIVE                  0x00000002  // Supports receiving data

//
//  NDIS MAC option bits for OID_GEN_CO_MAC_OPTIONS.
//
#define NDIS_CO_MAC_OPTION_DYNAMIC_LINK_SPEED   0x00000001

//
// The following is set on a per-packet basis as OOB data with NdisClassIrdaPacketInfo
// This is the per-packet info specified on a per-packet basis
//
typedef struct _NDIS_IRDA_PACKET_INFO
{
    ULONG                       ExtraBOFs;
    ULONG                       MinTurnAroundTime;
} NDIS_IRDA_PACKET_INFO, *PNDIS_IRDA_PACKET_INFO;



#if ((NTDDI_VERSION >= NTDDI_VISTA) || NDIS_SUPPORT_NDIS6)

//
// The following macro is used to build a NET_LUID
//

#define NDIS_MAKE_NET_LUID(_pNetLuid, _IfType, _NetLuidIndex)       \
{                                                                   \
    (_pNetLuid)->Info.IfType = _IfType;                             \
    (_pNetLuid)->Info.NetLuidIndex = _NetLuidIndex;                 \
    (_pNetLuid)->Info.Reserved = 0;                                 \
}

#define NDIS_IF_MAX_STRING_SIZE IF_MAX_STRING_SIZE
typedef IF_COUNTED_STRING NDIS_IF_COUNTED_STRING, *PNDIS_IF_COUNTED_STRING;

#define NDIS_MAX_PHYS_ADDRESS_LENGTH IF_MAX_PHYS_ADDRESS_LENGTH
typedef IF_PHYSICAL_ADDRESS NDIS_IF_PHYSICAL_ADDRESS, *PNDIS_IF_PHYSICAL_ADDRESS;

//
// NDIS_MEDIA_CONNECT_STATE enum type is used in OID_GEN_MEDIA_CONNECT_STATUS_EX
//
typedef NET_IF_MEDIA_CONNECT_STATE NDIS_MEDIA_CONNECT_STATE, *PNDIS_MEDIA_CONNECT_STATE;

//
// NET_IF_MEDIA_DUPLEX_STATE enum type is used in OID_GEN_MEDIA_DUPLEX_STATE
//
typedef NET_IF_MEDIA_DUPLEX_STATE NDIS_MEDIA_DUPLEX_STATE, *PNDIS_MEDIA_DUPLEX_STATE;

typedef enum _NDIS_SUPPORTED_PAUSE_FUNCTIONS
{
    NdisPauseFunctionsUnsupported,
    NdisPauseFunctionsSendOnly,
    NdisPauseFunctionsReceiveOnly,
    NdisPauseFunctionsSendAndReceive,
    NdisPauseFunctionsUnknown
} NDIS_SUPPORTED_PAUSE_FUNCTIONS, *PNDIS_SUPPORTED_PAUSE_FUNCTIONS;

#define NDIS_LINK_STATE_XMIT_LINK_SPEED_AUTO_NEGOTIATED         0x00000001
#define NDIS_LINK_STATE_RCV_LINK_SPEED_AUTO_NEGOTIATED          0x00000002
#define NDIS_LINK_STATE_DUPLEX_AUTO_NEGOTIATED                  0x00000004
#define NDIS_LINK_STATE_PAUSE_FUNCTIONS_AUTO_NEGOTIATED         0x00000008



//
// structure used in NDIS_STATUS_LINK_STATE and OID_GEN_LINK_STATE
//

#define NDIS_LINK_STATE_REVISION_1      1

typedef struct _NDIS_LINK_STATE
{
    NDIS_OBJECT_HEADER              Header;
    NDIS_MEDIA_CONNECT_STATE        MediaConnectState;
    NDIS_MEDIA_DUPLEX_STATE         MediaDuplexState;
    ULONG64                         XmitLinkSpeed;
    ULONG64                         RcvLinkSpeed;
    NDIS_SUPPORTED_PAUSE_FUNCTIONS  PauseFunctions;
    ULONG                           AutoNegotiationFlags;
} NDIS_LINK_STATE, *PNDIS_LINK_STATE;

#define NDIS_SIZEOF_LINK_STATE_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_LINK_STATE, AutoNegotiationFlags)

//
// the following structure is used in OID_GEN_LINK_PARAMETERS
//

#define NDIS_LINK_PARAMETERS_REVISION_1      1

typedef struct _NDIS_LINK_PARAMETERS
{
    NDIS_OBJECT_HEADER              Header;
    NDIS_MEDIA_DUPLEX_STATE         MediaDuplexState;
    ULONG64                         XmitLinkSpeed;
    ULONG64                         RcvLinkSpeed;
    NDIS_SUPPORTED_PAUSE_FUNCTIONS  PauseFunctions;
    ULONG                           AutoNegotiationFlags;
} NDIS_LINK_PARAMETERS, *PNDIS_LINK_PARAMETERS;

#define NDIS_SIZEOF_LINK_PARAMETERS_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_LINK_PARAMETERS, AutoNegotiationFlags)

//
// structure used in NDIS_STATUS_LINK_STATE and OID_GEN_LINK_STATE
//

#define NDIS_OPER_STATE_REVISION_1      1

typedef struct _NDIS_OPER_STATE
{
    NDIS_OBJECT_HEADER              Header;
    NET_IF_OPER_STATUS              OperationalStatus;
    ULONG                           OperationalStatusFlags;
} NDIS_OPER_STATE, *PNDIS_OPER_STATE;

#define NDIS_SIZEOF_OPER_STATE_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_OPER_STATE, OperationalStatusFlags)


//
// Structure used in OID_GEN_IP_OPER_STATUS.
//

typedef struct _NDIS_IP_OPER_STATUS
{
    ULONG AddressFamily;
    NET_IF_OPER_STATUS OperationalStatus;
    ULONG OperationalStatusFlags;
} NDIS_IP_OPER_STATUS, *PNDIS_IP_OPER_STATUS;

//
// Don't change the value for this macro definition.
//
#define MAXIMUM_IP_OPER_STATUS_ADDRESS_FAMILIES_SUPPORTED 32

#define NDIS_IP_OPER_STATUS_INFO_REVISION_1 1

typedef struct _NDIS_IP_OPER_STATUS_INFO
{
    NDIS_OBJECT_HEADER Header;
    ULONG Flags;
    ULONG NumberofAddressFamiliesReturned;
    NDIS_IP_OPER_STATUS IpOperationalStatus[MAXIMUM_IP_OPER_STATUS_ADDRESS_FAMILIES_SUPPORTED];
} NDIS_IP_OPER_STATUS_INFO, *PNDIS_IP_OPER_STATUS_INFO;

#define NDIS_SIZEOF_IP_OPER_STATUS_INFO_REVISION_1    \
        FIELD_OFFSET(NDIS_IP_OPER_STATUS_INFO, IpOperationalStatus) + \
        MAXIMUM_IP_OPER_STATUS_ADDRESS_FAMILIES_SUPPORTED * sizeof(NDIS_IP_OPER_STATUS)

//
// structure used in NDIS_STATUS_IP_OPER_STATUS
//

#define NDIS_IP_OPER_STATE_REVISION_1    1

typedef struct _NDIS_IP_OPER_STATE
{
    NDIS_OBJECT_HEADER Header;
    ULONG Flags;
    NDIS_IP_OPER_STATUS IpOperationalStatus;
} NDIS_IP_OPER_STATE, *PNDIS_IP_OPER_STATE;

#define NDIS_SIZEOF_IP_OPER_STATE_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_IP_OPER_STATE, IpOperationalStatus)

//
// These defines and structures are used with
// OID_TCP_OFFLOAD_PARAMETERS
//

#define NDIS_OFFLOAD_PARAMETERS_NO_CHANGE                  0

//
// values used in IPv4Checksum, TCPIPv4Checksum, UDPIPv4Checksum
// TCPIPv6Checksum and UDPIPv6Checksum
//
#define NDIS_OFFLOAD_PARAMETERS_TX_RX_DISABLED             1
#define NDIS_OFFLOAD_PARAMETERS_TX_ENABLED_RX_DISABLED     2
#define NDIS_OFFLOAD_PARAMETERS_RX_ENABLED_TX_DISABLED     3
#define NDIS_OFFLOAD_PARAMETERS_TX_RX_ENABLED              4

//
// values used in LsoV1
//
#define NDIS_OFFLOAD_PARAMETERS_LSOV1_DISABLED             1
#define NDIS_OFFLOAD_PARAMETERS_LSOV1_ENABLED              2

//
// values used in IPsecV1
//
#define NDIS_OFFLOAD_PARAMETERS_IPSECV1_DISABLED             1
#define NDIS_OFFLOAD_PARAMETERS_IPSECV1_AH_ENABLED           2
#define NDIS_OFFLOAD_PARAMETERS_IPSECV1_ESP_ENABLED          3
#define NDIS_OFFLOAD_PARAMETERS_IPSECV1_AH_AND_ESP_ENABLED   4

//
// values used in LsoV2
//
#define NDIS_OFFLOAD_PARAMETERS_LSOV2_DISABLED             1
#define NDIS_OFFLOAD_PARAMETERS_LSOV2_ENABLED              2

#if (NDIS_SUPPORT_NDIS61)
//
// values used in IPsecV2 and IPsecV2IPv4
//
#define NDIS_OFFLOAD_PARAMETERS_IPSECV2_DISABLED             1
#define NDIS_OFFLOAD_PARAMETERS_IPSECV2_AH_ENABLED           2
#define NDIS_OFFLOAD_PARAMETERS_IPSECV2_ESP_ENABLED          3
#define NDIS_OFFLOAD_PARAMETERS_IPSECV2_AH_AND_ESP_ENABLED   4
#endif // (NDIS_SUPPORT_NDIS61)

#if (NDIS_SUPPORT_NDIS630)

#define NDIS_OFFLOAD_PARAMETERS_RSC_DISABLED             1
#define NDIS_OFFLOAD_PARAMETERS_RSC_ENABLED              2

//
// Flags used in EncapsulationTypes field of NDIS_OFFLOAD_PARAMETERS
//
#define NDIS_ENCAPSULATION_TYPE_GRE_MAC          0x00000001

#if (NDIS_SUPPORT_NDIS650)

#define NDIS_ENCAPSULATION_TYPE_VXLAN            0x00000002

#endif // (NDIS_SUPPORT_NDIS650)

#endif // (NDIS_SUPPORT_NDIS630)


//
// values used in TcpConnectionIPv4 and TcpConnectionIPv6 fields
// of NDIS_OFFLOAD_PARAMETERS
//
#define NDIS_OFFLOAD_PARAMETERS_CONNECTION_OFFLOAD_DISABLED     1
#define NDIS_OFFLOAD_PARAMETERS_CONNECTION_OFFLOAD_ENABLED      2

#if (NDIS_SUPPORT_NDIS683)
//
// values used in UDP segmentation offload
//
#define NDIS_OFFLOAD_PARAMETERS_USO_DISABLED            1
#define NDIS_OFFLOAD_PARAMETERS_USO_ENABLED             2
#endif // (NDIS_SUPPORT_NDIS683)

//
// Used in OID_TCP_OFFLOAD_PARAMETERS for setting
// the offload parameters of a NIC
//

#define NDIS_OFFLOAD_PARAMETERS_REVISION_1            1

#if (NDIS_SUPPORT_NDIS61)
#define NDIS_OFFLOAD_PARAMETERS_REVISION_2            2
#endif // (NDIS_SUPPORT_NDIS61)

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_OFFLOAD_PARAMETERS_REVISION_3            3
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS650)
#define NDIS_OFFLOAD_PARAMETERS_REVISION_4            4
#endif // (NDIS_SUPPORT_NDIS650)

#if (NDIS_SUPPORT_NDIS683)
#define NDIS_OFFLOAD_PARAMETERS_REVISION_5            5
#endif // (NDIS_SUPPORT_NDIS683)

#if (NDIS_SUPPORT_NDIS689)
#define NDIS_OFFLOAD_PARAMETERS_REVISION_6            6
#endif // (NDIS_SUPPORT_NDIS689)

//
// Bits used in Flags parameter of NDIS_OFFLOAD_PARAMETERS structure:
//
#if (NDIS_SUPPORT_NDIS630)
#define NDIS_OFFLOAD_PARAMETERS_SKIP_REGISTRY_UPDATE    0x00000001
#endif // (NDIS_SUPPORT_NDIS630)

typedef struct _NDIS_OFFLOAD_PARAMETERS
{
    NDIS_OBJECT_HEADER      Header;

    UCHAR                   IPv4Checksum;
    UCHAR                   TCPIPv4Checksum;
    UCHAR                   UDPIPv4Checksum;

    UCHAR                   TCPIPv6Checksum;
    UCHAR                   UDPIPv6Checksum;

    UCHAR                   LsoV1;
    UCHAR                   IPsecV1;

    UCHAR                   LsoV2IPv4;
    UCHAR                   LsoV2IPv6;

    UCHAR                   TcpConnectionIPv4;
    UCHAR                   TcpConnectionIPv6;

    ULONG                   Flags;

#if (NDIS_SUPPORT_NDIS61)
    UCHAR                   IPsecV2;
    UCHAR                   IPsecV2IPv4;
#endif // (NDIS_SUPPORT_NDIS61)

#if (NDIS_SUPPORT_NDIS630)
    struct
    {
        UCHAR               RscIPv4;
        UCHAR               RscIPv6;
    };
    struct
    {
        UCHAR               EncapsulatedPacketTaskOffload;
        UCHAR               EncapsulationTypes;
    };
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS650)
    union _ENCAPSULATION_PROTOCOL_PARAMETERS {
        struct _VXLAN_PARAMETERS {
            USHORT VxlanUDPPortNumber;
        } VxlanParameters;

        ULONG Value;

    } EncapsulationProtocolParameters;

#endif // (NDIS_SUPPORT_NDIS650)

#if (NDIS_SUPPORT_NDIS683)
    struct
    {
        UCHAR               IPv4;
        UCHAR               IPv6;
    } UdpSegmentation;
#endif // (NDIS_SUPPORT_NDIS683)

#if (NDIS_SUPPORT_NDIS689)
    struct
    {
        UCHAR               Enabled;
    } UdpRsc;
#endif // (NDIS_SUPPORT_NDIS689)
} NDIS_OFFLOAD_PARAMETERS, *PNDIS_OFFLOAD_PARAMETERS;

#define NDIS_SIZEOF_OFFLOAD_PARAMETERS_REVISION_1 RTL_SIZEOF_THROUGH_FIELD(NDIS_OFFLOAD_PARAMETERS, Flags)

#if (NDIS_SUPPORT_NDIS61)
#define NDIS_SIZEOF_OFFLOAD_PARAMETERS_REVISION_2 RTL_SIZEOF_THROUGH_FIELD(NDIS_OFFLOAD_PARAMETERS, IPsecV2IPv4)
#endif // (NDIS_SUPPORT_NDIS61)

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_SIZEOF_OFFLOAD_PARAMETERS_REVISION_3 RTL_SIZEOF_THROUGH_FIELD(NDIS_OFFLOAD_PARAMETERS, EncapsulationTypes)
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS650)
#define NDIS_SIZEOF_OFFLOAD_PARAMETERS_REVISION_4 RTL_SIZEOF_THROUGH_FIELD(NDIS_OFFLOAD_PARAMETERS, EncapsulationProtocolParameters)
#endif // (NDIS_SUPPORT_NDIS650)

#if (NDIS_SUPPORT_NDIS683)
#define NDIS_SIZEOF_OFFLOAD_PARAMETERS_REVISION_5 RTL_SIZEOF_THROUGH_FIELD(NDIS_OFFLOAD_PARAMETERS, UdpSegmentation)
#endif // (NDIS_SUPPORT_NDIS683)

#if (NDIS_SUPPORT_NDIS689)
#define NDIS_SIZEOF_OFFLOAD_PARAMETERS_REVISION_6 RTL_SIZEOF_THROUGH_FIELD(NDIS_OFFLOAD_PARAMETERS, UdpRsc)
#endif // (NDIS_SUPPORT_NDIS689)

#include <ndis/offloadtypes.h>
#include <ndis/nbluro.h>

#pragma warning(push)
#pragma warning(disable:4214) //nonstandard extension used : bit field types other than int

//
// Describes the large send offload version 1 capabilities
// or configuration of the NIC. Used in NDIS_OFFLOAD structure
//
typedef struct _NDIS_TCP_LARGE_SEND_OFFLOAD_V1
{

    struct
    {
        ULONG     Encapsulation;
        ULONG     MaxOffLoadSize;
        ULONG     MinSegmentCount;
        ULONG     TcpOptions:2;
        ULONG     IpOptions:2;
    } IPv4;

} NDIS_TCP_LARGE_SEND_OFFLOAD_V1, *PNDIS_TCP_LARGE_SEND_OFFLOAD_V1;


//
// Describes the checksum task offload capabilities or configuration
// of the NIC. used in NDIS_OFFLOAD structure
//
typedef struct _NDIS_TCP_IP_CHECKSUM_OFFLOAD
{

    struct
    {
        ULONG       Encapsulation;
        ULONG       IpOptionsSupported:2;
        ULONG       TcpOptionsSupported:2;
        ULONG       TcpChecksum:2;
        ULONG       UdpChecksum:2;
        ULONG       IpChecksum:2;
    } IPv4Transmit;

    struct
    {
        ULONG       Encapsulation;
        ULONG       IpOptionsSupported:2;
        ULONG       TcpOptionsSupported:2;
        ULONG       TcpChecksum:2;
        ULONG       UdpChecksum:2;
        ULONG       IpChecksum:2;
    } IPv4Receive;


    struct
    {
        ULONG       Encapsulation;
        ULONG       IpExtensionHeadersSupported:2;
        ULONG       TcpOptionsSupported:2;
        ULONG       TcpChecksum:2;
        ULONG       UdpChecksum:2;

    } IPv6Transmit;

    struct
    {
        ULONG       Encapsulation;
        ULONG       IpExtensionHeadersSupported:2;
        ULONG       TcpOptionsSupported:2;
        ULONG       TcpChecksum:2;
        ULONG       UdpChecksum:2;

    } IPv6Receive;

} NDIS_TCP_IP_CHECKSUM_OFFLOAD, *PNDIS_TCP_IP_CHECKSUM_OFFLOAD;


//
// Describes the IPsec task offload version 1 capabilities
// or configuration of the NIC. Used in NDIS_OFFLOAD structure
//
typedef struct _NDIS_IPSEC_OFFLOAD_V1
{
    struct
    {
        ULONG   Encapsulation;
        ULONG   AhEspCombined;
        ULONG   TransportTunnelCombined;
        ULONG   IPv4Options;
        ULONG   Flags;
    } Supported;

    struct
    {
        ULONG   Md5:2;
        ULONG   Sha_1:2;
        ULONG   Transport:2;
        ULONG   Tunnel:2;
        ULONG   Send:2;
        ULONG   Receive:2;
    } IPv4AH;

    struct
    {
        ULONG   Des:2;
        ULONG   Reserved:2;
        ULONG   TripleDes:2;
        ULONG   NullEsp:2;
        ULONG   Transport:2;
        ULONG   Tunnel:2;
        ULONG   Send:2;
        ULONG   Receive:2;
    } IPv4ESP;

} NDIS_IPSEC_OFFLOAD_V1, *PNDIS_IPSEC_OFFLOAD_V1;

//
// Describes the large send offload version 2 capabilities
// or configuration of the NIC. Used in NDIS_OFFLOAD structure
//
typedef struct _NDIS_TCP_LARGE_SEND_OFFLOAD_V2
{
    struct
    {
         ULONG     Encapsulation;
         ULONG     MaxOffLoadSize;
         ULONG     MinSegmentCount;
    }IPv4;

    struct
    {
         ULONG     Encapsulation;
         ULONG     MaxOffLoadSize;
         ULONG     MinSegmentCount;
         ULONG     IpExtensionHeadersSupported:2;
         ULONG     TcpOptionsSupported:2;
    }IPv6;

} NDIS_TCP_LARGE_SEND_OFFLOAD_V2, *PNDIS_TCP_LARGE_SEND_OFFLOAD_V2;

#if (NDIS_SUPPORT_NDIS61)
//
//  Structures for IPSec Task Offload V2.
//

//
// IPsec Algorithms for Authentication used in AuthenticationAlgorithms field
// of NDIS_IPSEC_OFFLOAD_V2 structure
//
#define IPSEC_OFFLOAD_V2_AUTHENTICATION_MD5                      0x00000001
#define IPSEC_OFFLOAD_V2_AUTHENTICATION_SHA_1                    0x00000002
#define IPSEC_OFFLOAD_V2_AUTHENTICATION_SHA_256                  0x00000004
#define IPSEC_OFFLOAD_V2_AUTHENTICATION_AES_GCM_128              0x00000008
#define IPSEC_OFFLOAD_V2_AUTHENTICATION_AES_GCM_192              0x00000010
#define IPSEC_OFFLOAD_V2_AUTHENTICATION_AES_GCM_256              0x00000020

//
// IPsec Algorithms for Encryption used in EncryptionAlgorithms field of
// NDIS_IPSEC_OFFLOAD_V2 structure
//
#define IPSEC_OFFLOAD_V2_ENCRYPTION_NONE                          0x00000001
#define IPSEC_OFFLOAD_V2_ENCRYPTION_DES_CBC                       0x00000002
#define IPSEC_OFFLOAD_V2_ENCRYPTION_3_DES_CBC                     0x00000004
#define IPSEC_OFFLOAD_V2_ENCRYPTION_AES_GCM_128                   0x00000008
#define IPSEC_OFFLOAD_V2_ENCRYPTION_AES_GCM_192                   0x00000010
#define IPSEC_OFFLOAD_V2_ENCRYPTION_AES_GCM_256                   0x00000020
#define IPSEC_OFFLOAD_V2_ENCRYPTION_AES_CBC_128                   0x00000040
#define IPSEC_OFFLOAD_V2_ENCRYPTION_AES_CBC_192                   0x00000080
#define IPSEC_OFFLOAD_V2_ENCRYPTION_AES_CBC_256                   0x00000100

//
// IPsec offload V2 capabilities used in  NDIS_OFFLOAD
//
typedef struct _NDIS_IPSEC_OFFLOAD_V2
{
    ULONG       Encapsulation;      // MAC encap types supported
    BOOLEAN     IPv6Supported;      // IPv6 Supported
    BOOLEAN     IPv4Options;                           // Supports offload of packets with IPv4 options
    BOOLEAN     IPv6NonIPsecExtensionHeaders;          // Supports offload of packets with non IPsec Extension headers
    BOOLEAN     Ah;
    BOOLEAN     Esp;
    BOOLEAN     AhEspCombined;
    BOOLEAN     Transport;
    BOOLEAN     Tunnel;
    BOOLEAN     TransportTunnelCombined;
    BOOLEAN     LsoSupported;
    BOOLEAN     ExtendedSequenceNumbers;
    ULONG       UdpEsp;
    ULONG       AuthenticationAlgorithms;     // Bit Mask of Authentication Algorithms
    ULONG       EncryptionAlgorithms;         // Bit Mask of Encryption Algorithms
    ULONG       SaOffloadCapacity;            // Number of SAs that can be offloaded
} NDIS_IPSEC_OFFLOAD_V2, *PNDIS_IPSEC_OFFLOAD_V2;

#endif // (NDIS_SUPPORT_NDIS61)

#pragma warning(pop)


#if (NDIS_SUPPORT_NDIS630)

typedef struct _NDIS_TCP_RECV_SEG_COALESCE_OFFLOAD
{
    struct
    {
        BOOLEAN Enabled;
    } IPv4;
    struct
    {
        BOOLEAN Enabled;
    } IPv6;
} NDIS_TCP_RECV_SEG_COALESCE_OFFLOAD, *PNDIS_TCP_RECV_SEG_COALESCE_OFFLOAD;

#define NDIS_TCP_RECV_SEG_COALESC_OFFLOAD_REVISION_1            1

#define NDIS_SIZEOF_TCP_RECV_SEG_COALESC_OFFLOAD_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_TCP_RECV_SEG_COALESCE_OFFLOAD, IPv6.Enabled)

#define NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_NOT_SUPPORTED    0x00000000
#define NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_INNER_IPV4       0x00000001
#define NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_OUTER_IPV4       0x00000002
#define NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_INNER_IPV6       0x00000004
#define NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_OUTER_IPV6       0x00000008

#pragma warning(push)
#pragma warning(disable:4214) //nonstandard extension used : bit field types other than int

typedef struct _NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD {
    ULONG TransmitChecksumOffloadSupported:4;
    ULONG ReceiveChecksumOffloadSupported:4;
    ULONG LsoV2Supported:4;
    ULONG RssSupported:4;
    ULONG VmqSupported:4;
#if (NDIS_SUPPORT_NDIS685)
    ULONG UsoSupported:4;
    ULONG Reserved:8;
#endif // (NDIS_SUPPORT_NDIS685)
    ULONG MaxHeaderSizeSupported;
} NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD, *PNDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD;

#pragma warning(pop)

#define NDIS_SIZEOF_ENCAPSULATED_PACKET_TASK_OFFLOAD_REVISION_1     \
        RTL_SIZEOF_THROUGH_FIELD(                                   \
            NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD, MaxHeaderSizeSupported)

#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS650)

typedef struct _NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_V2 {
    ULONG TransmitChecksumOffloadSupported:4;
    ULONG ReceiveChecksumOffloadSupported:4;
    ULONG LsoV2Supported:4;
    ULONG RssSupported:4;
    ULONG VmqSupported:4;
#if (NDIS_SUPPORT_NDIS685)
    ULONG UsoSupported:4;
    ULONG Reserved:8;
#else
    ULONG Reserved:12;
#endif // (NDIS_SUPPORT_NDIS685)
    ULONG MaxHeaderSizeSupported;

    union _ENCAPSULATION_PROTOCOL_INFO {
        struct _VXLAN_INFO {
           USHORT VxlanUDPPortNumber;
           USHORT VxlanUDPPortNumberConfigurable  :1;
        } VxlanInfo;

        ULONG Value;

    } EncapsulationProtocolInfo;

    ULONG Reserved1;
    ULONG Reserved2;
} NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_V2, *PNDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_V2;

#endif // (NDIS_SUPPORT_NDIS650)

#if (NDIS_SUPPORT_NDIS670)

typedef enum _NDIS_RFC6877_464XLAT_OFFLOAD_OPTIONS
{
    NDIS_RFC6877_464XLAT_OFFLOAD_NOT_SUPPORTED = 0,    // incapable of 464XLAT hardware offload
    NDIS_RFC6877_464XLAT_OFFLOAD_DISABLED,             // capable but disabled
    NDIS_RFC6877_464XLAT_OFFLOAD_ENABLED,              // capable and enabled all time
    NDIS_RFC6877_464XLAT_OFFLOAD_ON_DEMAND,            // capable and enabled only on-demand
} NDIS_RFC6877_464XLAT_OFFLOAD_OPTIONS;

typedef struct _NDIS_RFC6877_464XLAT_OFFLOAD
{
    NDIS_RFC6877_464XLAT_OFFLOAD_OPTIONS   XlatOffload;
    ULONG Flags;                                       // Reserved, always 0
} NDIS_RFC6877_464XLAT_OFFLOAD, *PNDIS_RFC6877_464XLAT_OFFLOAD;

#endif // (NDIS_SUPPORT_NDIS670)

#if (NDIS_SUPPORT_NDIS683)

typedef struct _NDIS_UDP_SEGMENTATION_OFFLOAD
{
    struct
    {
        ULONG     Encapsulation;
        ULONG     MaxOffLoadSize;
        ULONG     MinSegmentCount : 6;
#if (NDIS_SUPPORT_NDIS684)
        ULONG     SubMssFinalSegmentSupported : 1;
        ULONG     Reserved : 25;
#else
        ULONG     Reserved : 26;
#endif // (NDIS_SUPPORT_NDIS684)
    } IPv4;

    struct
    {
        ULONG     Encapsulation;
        ULONG     MaxOffLoadSize;
        ULONG     MinSegmentCount : 6;
#if (NDIS_SUPPORT_NDIS684)
        ULONG     SubMssFinalSegmentSupported : 1;
        ULONG     Reserved1 : 25;
#else
        ULONG     Reserved1 : 26;
#endif // (NDIS_SUPPORT_NDIS684)
        ULONG     IpExtensionHeadersSupported : 2;
        ULONG     Reserved2 : 30;
    } IPv6;
} NDIS_UDP_SEGMENTATION_OFFLOAD, *PNDIS_UDP_SEGMENTATION_OFFLOAD;

#endif // (NDIS_SUPPORT_NDIS683)

//
// flags used in Flags field of NDIS_OFFLOAD structure
//
#define NDIS_OFFLOAD_FLAGS_GROUP_CHECKSUM_CAPABILITIES  0x00000001

#if (NDIS_SUPPORT_NDIS630)
#define IPSEC_OFFLOAD_V2_AND_TCP_CHECKSUM_COEXISTENCE  0x00000002
#define IPSEC_OFFLOAD_V2_AND_UDP_CHECKSUM_COEXISTENCE  0x00000004
#endif // (NDIS_SUPPORT_NDIS630)

//
// Describes TCP/IP task offload capabilities or configuration
// of the NIC. Used in OID_TCP_OFFLOAD_CURRENT_CONFIG
// and OID_TCP_OFFLOAD_HARDWARE_CAPABILITIES
//
#define NDIS_OFFLOAD_REVISION_1    1
#if (NDIS_SUPPORT_NDIS61)
#define NDIS_OFFLOAD_REVISION_2    2
#endif // (NDIS_SUPPORT_NDIS61)

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_OFFLOAD_REVISION_3    3
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS650)
#define NDIS_OFFLOAD_REVISION_4    4
#endif // (NDIS_SUPPORT_NDIS650)

#if (NDIS_SUPPORT_NDIS670)
#define NDIS_OFFLOAD_REVISION_5    5
#endif // (NDIS_SUPPORT_NDIS670)

#if (NDIS_SUPPORT_NDIS683)
#define NDIS_OFFLOAD_REVISION_6    6
#endif // (NDIS_SUPPORT_NDIS683)

#if (NDIS_SUPPORT_NDIS685)
#define NDIS_OFFLOAD_REVISION_7    7
#endif // (NDIS_SUPPORT_NDIS685)

#if (NDIS_SUPPORT_NDIS689)
#define NDIS_OFFLOAD_REVISION_8    8
#endif // (NDIS_SUPPORT_NDIS689)

typedef struct _NDIS_OFFLOAD
{
    NDIS_OBJECT_HEADER                       Header;

    //
    // Checksum Offload information
    //
    NDIS_TCP_IP_CHECKSUM_OFFLOAD             Checksum;

    //
    // Large Send Offload information
    //
    NDIS_TCP_LARGE_SEND_OFFLOAD_V1           LsoV1;

    //
    // IPsec Offload Information
    //
    NDIS_IPSEC_OFFLOAD_V1                    IPsecV1;

    //
    // Large Send Offload version 2Information
    //
    NDIS_TCP_LARGE_SEND_OFFLOAD_V2           LsoV2;

    ULONG                                    Flags;

#if (NDIS_SUPPORT_NDIS61)
    //
    //IPsec offload V2
    //
    NDIS_IPSEC_OFFLOAD_V2                    IPsecV2;
#endif // (NDIS_SUPPORT_NDIS61)

#if (NDIS_SUPPORT_NDIS630)
    //
    // Receive Segment Coalescing information
    //
    NDIS_TCP_RECV_SEG_COALESCE_OFFLOAD       Rsc;

    //
    // NVGRE Encapsulated packet task offload information
    //
    NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD    EncapsulatedPacketTaskOffloadGre;
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS650)
    //
    // VXLAN Encapsulated packet task offload information
    //
    NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_V2 EncapsulatedPacketTaskOffloadVxlan;

    //
    // Enabled encapsulation types for Encapsulated packet task offload
    //
    UCHAR                                    EncapsulationTypes;
#endif // (NDIS_SUPPORT_NDIS650)

#if (NDIS_SUPPORT_NDIS670)
    //
    // 464XLAT hardward offload information.
    //
    NDIS_RFC6877_464XLAT_OFFLOAD             Rfc6877Xlat;

#endif // (NDIS_SUPPORT_NDIS670)

#if (NDIS_SUPPORT_NDIS683)
    //
    // UDP segmentation offload.
    //
    NDIS_UDP_SEGMENTATION_OFFLOAD            UdpSegmentation;
#endif // (NDIS_SUPPORT_NDIS683)

#if (NDIS_SUPPORT_NDIS689)
    //
    // UDP RSC offload.
    //
    NDIS_UDP_RSC_OFFLOAD                     UdpRsc;
#endif // (NDIS_SUPPORT_NDIS689)

} NDIS_OFFLOAD, *PNDIS_OFFLOAD;

#define NDIS_SIZEOF_NDIS_OFFLOAD_REVISION_1   RTL_SIZEOF_THROUGH_FIELD(NDIS_OFFLOAD, Flags)

#if (NDIS_SUPPORT_NDIS61)
#define NDIS_SIZEOF_NDIS_OFFLOAD_REVISION_2   RTL_SIZEOF_THROUGH_FIELD(NDIS_OFFLOAD, IPsecV2)
#endif // (NDIS_SUPPORT_NDIS61)

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_SIZEOF_NDIS_OFFLOAD_REVISION_3   RTL_SIZEOF_THROUGH_FIELD(NDIS_OFFLOAD, EncapsulatedPacketTaskOffloadGre)
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS650)
#define NDIS_SIZEOF_NDIS_OFFLOAD_REVISION_4   RTL_SIZEOF_THROUGH_FIELD(NDIS_OFFLOAD, EncapsulationTypes)
#endif // (NDIS_SUPPORT_NDIS650)

#if (NDIS_SUPPORT_NDIS670)
#define NDIS_SIZEOF_NDIS_OFFLOAD_REVISION_5   RTL_SIZEOF_THROUGH_FIELD(NDIS_OFFLOAD, Rfc6877Xlat)
#endif // (NDIS_SUPPORT_NDIS670)

#if (NDIS_SUPPORT_NDIS683)
#define NDIS_SIZEOF_NDIS_OFFLOAD_REVISION_6   RTL_SIZEOF_THROUGH_FIELD(NDIS_OFFLOAD, UdpSegmentation)
#endif // (NDIS_SUPPORT_NDIS683)

#if (NDIS_SUPPORT_NDIS685)
#define NDIS_SIZEOF_NDIS_OFFLOAD_REVISION_7   RTL_SIZEOF_THROUGH_FIELD(NDIS_OFFLOAD, UdpSegmentation)
#endif // (NDIS_SUPPORT_NDIS685)

#if (NDIS_SUPPORT_NDIS689)
#define NDIS_SIZEOF_NDIS_OFFLOAD_REVISION_8   RTL_SIZEOF_THROUGH_FIELD(NDIS_OFFLOAD, UdpRsc)
#endif // (NDIS_SUPPORT_NDIS689)

//
// The following data structures are used with offload related WMI
// guids. NDIS will trnaslate these data structures to those used
// in OID and status indications
//
typedef struct _NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1
{
    struct
    {
        ULONG     Encapsulation;
        ULONG     MaxOffLoadSize;
        ULONG     MinSegmentCount;
        ULONG     TcpOptions;
        ULONG     IpOptions;
    } IPv4;

} NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1, *PNDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1;

typedef struct _NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD
{

    struct
    {
        ULONG       Encapsulation;
        ULONG       IpOptionsSupported;
        ULONG       TcpOptionsSupported;
        ULONG       TcpChecksum;
        ULONG       UdpChecksum;
        ULONG       IpChecksum;
    } IPv4Transmit;

    struct
    {
        ULONG       Encapsulation;
        ULONG       IpOptionsSupported;
        ULONG       TcpOptionsSupported;
        ULONG       TcpChecksum;
        ULONG       UdpChecksum;
        ULONG       IpChecksum;
    } IPv4Receive;


    struct
    {
        ULONG       Encapsulation;
        ULONG       IpExtensionHeadersSupported;
        ULONG       TcpOptionsSupported;
        ULONG       TcpChecksum;
        ULONG       UdpChecksum;

    } IPv6Transmit;

    struct
    {
        ULONG       Encapsulation;
        ULONG       IpExtensionHeadersSupported;
        ULONG       TcpOptionsSupported;
        ULONG       TcpChecksum;
        ULONG       UdpChecksum;

    } IPv6Receive;

} NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD, *PNDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD;

typedef struct _NDIS_WMI_IPSEC_OFFLOAD_V1
{
    struct
    {
        ULONG   Encapsulation;
        ULONG   AhEspCombined;
        ULONG   TransportTunnelCombined;
        ULONG   IPv4Options;
        ULONG   Flags;
    } Supported;

    struct
    {
        ULONG   Md5;
        ULONG   Sha_1;
        ULONG   Transport;
        ULONG   Tunnel;
        ULONG   Send;
        ULONG   Receive;
    } IPv4AH;

    struct
    {
        ULONG   Des;
        ULONG   Reserved;
        ULONG   TripleDes;
        ULONG   NullEsp;
        ULONG   Transport;
        ULONG   Tunnel;
        ULONG   Send;
        ULONG   Receive;
    } IPv4ESP;

} NDIS_WMI_IPSEC_OFFLOAD_V1, *PNDIS_WMI_IPSEC_OFFLOAD_V1;

typedef struct _NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2
{
    struct
    {
         ULONG     Encapsulation;
         ULONG     MaxOffLoadSize;
         ULONG     MinSegmentCount;
    }IPv4;

    struct
    {
         ULONG     Encapsulation;
         ULONG     MaxOffLoadSize;
         ULONG     MinSegmentCount;
         ULONG     IpExtensionHeadersSupported;
         ULONG     TcpOptionsSupported;
    }IPv6;

} NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2, *PNDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2;


typedef struct _NDIS_WMI_OFFLOAD
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_OFFLOAD;
    // Header.Size = sizeof(NDIS_OFFLOAD);
    // Header.Revision  = NDIS_OFFLOAD_REVISION_1;
    //
    NDIS_OBJECT_HEADER                  Header;

    //
    // Checksum Offload information
    //
    NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD    Checksum;

    //
    // Large Send Offload information
    //
    NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1   LsoV1;

    //
    // IPsec Offload Information
    //
    NDIS_WMI_IPSEC_OFFLOAD_V1           IPsecV1;
    //
    // Large Send Offload version 2 Information
    //
    NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2  LsoV2;

    ULONG                               Flags;

#if (NDIS_SUPPORT_NDIS61)
    //
    // IPsec offload version 2 information
    //
    NDIS_IPSEC_OFFLOAD_V2               IPsecV2;
#endif // (NDIS_SUPPORT_NDIS61)


#if (NDIS_SUPPORT_NDIS630)
    //
    // Recieve Segment Coalescing information
    //
    NDIS_TCP_RECV_SEG_COALESCE_OFFLOAD  Rsc;

    //
    // GRE Encapsulated packet task offload information
    //
    NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD EncapsulatedPacketTaskOffloadGre;
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS683)
    //
    // UDP segmentation offload.
    //
    NDIS_UDP_SEGMENTATION_OFFLOAD            UdpSegmentation;
#endif // (NDIS_SUPPORT_NDIS683)

#if (NDIS_SUPPORT_NDIS689)
    //
    // UDP RSC offload.
    //
    NDIS_UDP_RSC_OFFLOAD              UdpRsc;
#endif // (NDIS_SUPPORT_NDIS689)

} NDIS_WMI_OFFLOAD, *PNDIS_WMI_OFFLOAD;

#define NDIS_SIZEOF_NDIS_WMI_OFFLOAD_REVISION_1   RTL_SIZEOF_THROUGH_FIELD(NDIS_WMI_OFFLOAD, Flags)

#if (NDIS_SUPPORT_NDIS61)
#define NDIS_SIZEOF_NDIS_WMI_OFFLOAD_REVISION_2   RTL_SIZEOF_THROUGH_FIELD(NDIS_WMI_OFFLOAD, IPsecV2)
#endif // (NDIS_SUPPORT_NDIS61)

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_SIZEOF_NDIS_WMI_OFFLOAD_REVISION_3   RTL_SIZEOF_THROUGH_FIELD(NDIS_WMI_OFFLOAD, EncapsulatedPacketTaskOffloadGre)
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS683)
#define NDIS_SIZEOF_NDIS_WMI_OFFLOAD_REVISION_4   RTL_SIZEOF_THROUGH_FIELD(NDIS_WMI_OFFLOAD, UdpSegmentation)
#endif // (NDIS_SUPPORT_NDIS683)

#if (NDIS_SUPPORT_NDIS689)
#define NDIS_SIZEOF_NDIS_WMI_OFFLOAD_REVISION_5   RTL_SIZEOF_THROUGH_FIELD(NDIS_WMI_OFFLOAD, UdpRsc)
#endif // (NDIS_SUPPORT_NDIS689)


#pragma warning(push)
#pragma warning(disable:4214) //nonstandard extension used : bit field types other than int

//
// Describes TCP connection offload capabilities or configuration
// of the NIC. Used in OID_TCP_CONNECTION_OFFLOAD_CURRENT_CONFIG,
// OID_TCP_CONNECTION_OFFLOAD_HARDWARE_CAPABILITIES and
// NDIS_STATUS_OFFLOAD_RESUME
//
#define NDIS_TCP_CONNECTION_OFFLOAD_REVISION_1              1
#if (NDIS_SUPPORT_NDIS61)
#define NDIS_TCP_CONNECTION_OFFLOAD_REVISION_2              2
#endif // (NDIS_SUPPORT_NDIS61)

typedef struct _NDIS_TCP_CONNECTION_OFFLOAD
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_TCP_CONNECTION_OFFLOAD_REVISION_1;
    // Header.Size = sizeof(NDIS_TCP_CONNECTION_OFFLOAD);
    //
    NDIS_OBJECT_HEADER Header;
    ULONG Encapsulation;
    ULONG SupportIPv4:2;
    ULONG SupportIPv6:2;
    ULONG SupportIPv6ExtensionHeaders:2;
    ULONG SupportSack:2;
#if (NDIS_SUPPORT_NDIS61)
    ULONG CongestionAlgorithm:4;
#endif // (NDIS_SUPPORT_NDIS61)
    ULONG TcpConnectionOffloadCapacity;
    ULONG Flags;
} NDIS_TCP_CONNECTION_OFFLOAD, *PNDIS_TCP_CONNECTION_OFFLOAD;

#define NDIS_SIZEOF_TCP_CONNECTION_OFFLOAD_REVISION_1 RTL_SIZEOF_THROUGH_FIELD(NDIS_TCP_CONNECTION_OFFLOAD, Flags)
#if (NDIS_SUPPORT_NDIS61)
#define NDIS_SIZEOF_TCP_CONNECTION_OFFLOAD_REVISION_2 RTL_SIZEOF_THROUGH_FIELD(NDIS_TCP_CONNECTION_OFFLOAD, Flags)
#endif // (NDIS_SUPPORT_NDIS61)

#pragma warning(pop)

typedef struct _NDIS_WMI_TCP_CONNECTION_OFFLOAD
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_TCP_CONNECTION_OFFLOAD_REVISION_1;
    // Header.Size = sizeof(NDIS_TCP_CONNECTION_OFFLOAD);
    //
    NDIS_OBJECT_HEADER Header;
    ULONG Encapsulation;
    ULONG SupportIPv4;
    ULONG SupportIPv6;
    ULONG SupportIPv6ExtensionHeaders;
    ULONG SupportSack;
    ULONG TcpConnectionOffloadCapacity;
    ULONG Flags;
} NDIS_WMI_TCP_CONNECTION_OFFLOAD, *PNDIS_WMI_TCP_CONNECTION_OFFLOAD;

#define NDIS_SIZEOF_WMI_TCP_CONNECTION_OFFLOAD_REVISION_1 RTL_SIZEOF_THROUGH_FIELD(NDIS_WMI_TCP_CONNECTION_OFFLOAD, Flags)

#include <ndis/ndisport.h>

//
// NDIS_PORT_AUTHENTICATION_STATE defines the authentication state of a port
// on a particular data path
//
typedef enum _NDIS_PORT_AUTHORIZATION_STATE
{
    NdisPortAuthorizationUnknown,
    NdisPortAuthorized,
    NdisPortUnauthorized,
    NdisPortReauthorizing
} NDIS_PORT_AUTHORIZATION_STATE, *PNDIS_PORT_AUTHORIZATION_STATE;


//
// NDIS_PORT_CONTROL_STATE specifies if a port is "controlled"
// i.e. if it needs authentication, on a particular data path
//

typedef enum _NDIS_PORT_CONTROL_STATE
{
    NdisPortControlStateUnknown,
    NdisPortControlStateControlled,
    NdisPortControlStateUncontrolled
} NDIS_PORT_CONTROL_STATE, *PNDIS_PORT_CONTROL_STATE;

#ifdef NDIS_INCLUDE_LEGACY_NAMES
// Legacy spelling errors
typedef NDIS_PORT_CONTROL_STATE  NDIS_PORT_CONTROLL_STATE;
typedef PNDIS_PORT_CONTROL_STATE PNDIS_PORT_CONTROLL_STATE;
#endif // NDIS_INCLUDE_LEGACY_NAMES

//
// NDIS_PORT_PARAMETERS is used in OID_GEN_PORT_PARAMETERS set OID
//
#define NDIS_PORT_AUTHENTICATION_PARAMETERS_REVISION_1     1

typedef struct _NDIS_PORT_AUTHENTICATION_PARAMETERS
{
    NDIS_OBJECT_HEADER              Header;
    NDIS_PORT_CONTROL_STATE         SendControlState;
    NDIS_PORT_CONTROL_STATE         RcvControlState;
    NDIS_PORT_AUTHORIZATION_STATE   SendAuthorizationState;
    NDIS_PORT_AUTHORIZATION_STATE   RcvAuthorizationState;
}NDIS_PORT_AUTHENTICATION_PARAMETERS, *PNDIS_PORT_AUTHENTICATION_PARAMETERS;

#define NDIS_SIZEOF_PORT_AUTHENTICATION_PARAMETERS_REVISION_1     \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_PORT_AUTHENTICATION_PARAMETERS, RcvAuthorizationState)

typedef enum _NDIS_NETWORK_CHANGE_TYPE
{
    NdisPossibleNetworkChange = 1,
    NdisDefinitelyNetworkChange,
    NdisNetworkChangeFromMediaConnect,
    NdisNetworkChangeMax
} NDIS_NETWORK_CHANGE_TYPE, *PNDIS_NETWORK_CHANGE_TYPE;


#define  NDIS_WMI_DEFAULT_METHOD_ID             1

#define  NDIS_WMI_OBJECT_TYPE_SET               0x01
#define  NDIS_WMI_OBJECT_TYPE_METHOD            0x02
#define  NDIS_WMI_OBJECT_TYPE_EVENT             0x03
#define  NDIS_WMI_OBJECT_TYPE_ENUM_ADAPTER      0x04
#define  NDIS_WMI_OBJECT_TYPE_OUTPUT_INFO       0x05

#define NDIS_WMI_METHOD_HEADER_REVISION_1       1

typedef struct _NDIS_WMI_METHOD_HEADER
{
    NDIS_OBJECT_HEADER   Header;
    NDIS_PORT_NUMBER     PortNumber;
    NET_LUID             NetLuid;
    ULONG64              RequestId;
    ULONG                Timeout;
    UCHAR                Padding[4];
} NDIS_WMI_METHOD_HEADER, *PNDIS_WMI_METHOD_HEADER;

#define NDIS_SIZEOF_WMI_METHOD_HEADER_REVISION_1     \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_WMI_METHOD_HEADER, Padding)

#define NDIS_WMI_SET_HEADER_REVISION_1         1

typedef struct _NDIS_WMI_SET_HEADER
{
    NDIS_OBJECT_HEADER   Header;
    NDIS_PORT_NUMBER     PortNumber;
    NET_LUID             NetLuid;
    ULONG64              RequestId;
    ULONG                Timeout;
    UCHAR                Padding[4];
} NDIS_WMI_SET_HEADER, *PNDIS_WMI_SET_HEADER;

#define NDIS_SIZEOF_WMI_SET_HEADER_REVISION_1     \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_WMI_SET_HEADER, Padding)

#define NDIS_WMI_EVENT_HEADER_REVISION_1      1

typedef struct _NDIS_WMI_EVENT_HEADER
{
    NDIS_OBJECT_HEADER   Header;
    NET_IFINDEX          IfIndex;
    NET_LUID             NetLuid;
    ULONG64              RequestId;
    NDIS_PORT_NUMBER     PortNumber;
    ULONG                DeviceNameLength;
    ULONG                DeviceNameOffset;
    UCHAR                Padding[4];
} NDIS_WMI_EVENT_HEADER, *PNDIS_WMI_EVENT_HEADER;

#define NDIS_SIZEOF_WMI_EVENT_HEADER_REVISION_1     \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_WMI_EVENT_HEADER, Padding)

#define NDIS_WMI_ENUM_ADAPTER_REVISION_1       1

typedef struct _NDIS_WMI_ENUM_ADAPTER
{
    NDIS_OBJECT_HEADER   Header;
    NET_IFINDEX          IfIndex;
    NET_LUID             NetLuid;
    USHORT               DeviceNameLength;
    CHAR                 DeviceName[1];
}NDIS_WMI_ENUM_ADAPTER, *PNDIS_WMI_ENUM_ADAPTER;

#define NDIS_SIZEOF_WMI_ENUM_ADAPTER_REVISION_1     \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_WMI_ENUM_ADAPTER, DeviceName)

//
// Flags used in standardized keyword *NdisDeviceType
//
#define NDIS_DEVICE_TYPE_ENDPOINT           0x00000001


#if (NDIS_SUPPORT_NDIS61)

//
// Structure and defines for
// OID_GEN_HD_SPLIT_PARAMETERS
//
#define NDIS_HD_SPLIT_PARAMETERS_REVISION_1      1

typedef struct _NDIS_HD_SPLIT_PARAMETERS
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_HD_SPLIT_PARAMETERS_REVISION_1;
    // Header.Size = sizeof(NDIS_HD_SPLIT_PARAMETERS);
    //
    NDIS_OBJECT_HEADER      Header;
    ULONG                   HDSplitCombineFlags;
}NDIS_HD_SPLIT_PARAMETERS, *PNDIS_HD_SPLIT_PARAMETERS;

#define NDIS_SIZEOF_HD_SPLIT_PARAMETERS_REVISION_1     \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_HD_SPLIT_PARAMETERS, HDSplitCombineFlags)

//
// Flags used in NDIS_HD_SPLIT_PARAMETERS->HDSplitCombineFlags
// and NDIS_HD_SPLIT_CURRENT_CONFIG->HDSplitCombineFlags
//
#define NDIS_HD_SPLIT_COMBINE_ALL_HEADERS          0x00000001

//
// Structure and defines for
// OID_GEN_HD_SPLIT_CURRENT_CONFIG
//
#define NDIS_HD_SPLIT_CURRENT_CONFIG_REVISION_1      1

typedef struct _NDIS_HD_SPLIT_CURRENT_CONFIG
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_HD_SPLIT_CURRENT_CONFIG_REVISION_1;
    // Header.Size = sizeof(NDIS_HD_SPLIT_CURRENT_CONFIG);
    //
    NDIS_OBJECT_HEADER          Header;
    ULONG                       HardwareCapabilities;
    ULONG                       CurrentCapabilities;
    ULONG                       HDSplitFlags;
    ULONG                       HDSplitCombineFlags;
    ULONG                       BackfillSize;
    ULONG                       MaxHeaderSize;
} NDIS_HD_SPLIT_CURRENT_CONFIG, *PNDIS_HD_SPLIT_CURRENT_CONFIG;

#define NDIS_SIZEOF_HD_SPLIT_CURRENT_CONFIG_REVISION_1     \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_HD_SPLIT_CURRENT_CONFIG, MaxHeaderSize)

//
// Flags used in NDIS_HD_SPLIT_ATTRIBUTES->HardwareCapabilities
// and NDIS_HD_SPLIT_ATTRIBUTES->CurrentCapabilities. They are also
// used in NDIS_HD_SPLIT_CURRENT_CONFIG->HardwareCapabilities
// and NDIS_HD_SPLIT_CURRENT_CONFIG->CurrentCapabilities.
//
#define NDIS_HD_SPLIT_CAPS_SUPPORTS_HEADER_DATA_SPLIT         0x00000001
#define NDIS_HD_SPLIT_CAPS_SUPPORTS_IPV4_OPTIONS              0x00000002
#define NDIS_HD_SPLIT_CAPS_SUPPORTS_IPV6_EXTENSION_HEADERS    0x00000004
#define NDIS_HD_SPLIT_CAPS_SUPPORTS_TCP_OPTIONS               0x00000008

//
// Flags used in NDIS_HD_SPLIT_ATTRIBUTES->HDSplitFlags and
// NDIS_HD_SPLIT_CURRENT_CONFIG->HDSplitFlags
//
#define NDIS_HD_SPLIT_ENABLE_HEADER_DATA_SPLIT     0x00000001

#endif // (NDIS_SUPPORT_NDIS61)

#define NDIS_WMI_OUTPUT_INFO_REVISION_1    1;

typedef struct NDIS_WMI_OUTPUT_INFO
{
    NDIS_OBJECT_HEADER   Header;
    ULONG                Flags;
    UCHAR                SupportedRevision;
    ULONG                DataOffset;

} NDIS_WMI_OUTPUT_INFO, *PNDIS_WMI_OUTPUT_INFO;

#define NDIS_SIZEOF_WMI_OUTPUT_INFO_REVISION_1     \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_WMI_OUTPUT_INFO, DataOffset)


#if (NDIS_SUPPORT_NDIS620)

//
// NDIS 6.20 power management
//

//
// Flags used in NDIS_PM_CAPABILITIES struct
//
// Flags for NDIS_PM_CAPABILITIES.SupportedWoLPacketPatterns field
//
#define NDIS_PM_WOL_BITMAP_PATTERN_SUPPORTED                    0x00000001
#define NDIS_PM_WOL_MAGIC_PACKET_SUPPORTED                      0x00000002
#define NDIS_PM_WOL_IPV4_TCP_SYN_SUPPORTED                      0x00000004
#define NDIS_PM_WOL_IPV6_TCP_SYN_SUPPORTED                      0x00000008
#define NDIS_PM_WOL_IPV4_DEST_ADDR_WILDCARD_SUPPORTED           0x00000200
#define NDIS_PM_WOL_IPV6_DEST_ADDR_WILDCARD_SUPPORTED           0x00000800
#define NDIS_PM_WOL_EAPOL_REQUEST_ID_MESSAGE_SUPPORTED          0x00010000

//
// Flags for NDIS_PM_CAPABILITIES.SupportedProtocolOffloads field
//
#define NDIS_PM_PROTOCOL_OFFLOAD_ARP_SUPPORTED                  0x00000001
#define NDIS_PM_PROTOCOL_OFFLOAD_NS_SUPPORTED                   0x00000002
#define NDIS_PM_PROTOCOL_OFFLOAD_80211_RSN_REKEY_SUPPORTED      0x00000080

//
// Flags for NDIS_PM_CAPABILITIES.SupportedWakeUpEvents field
// to advertise media-agnostic wake capabilities
//
#if (NDIS_SUPPORT_NDIS630)
#define NDIS_PM_WAKE_ON_MEDIA_CONNECT_SUPPORTED                   0x00000001
#define NDIS_PM_WAKE_ON_MEDIA_DISCONNECT_SUPPORTED                0x00000002
#endif // (NDIS_SUPPORT_NDIS630)

//
// Flags for NDIS_PM_CAPABILITIES.MediaSpecificWakeUpEvents field
// to advertise media-specific wake capabilities for an adapter
// with physical media type NdisPhysicalMediumNative802_11
//
#if (NDIS_SUPPORT_NDIS630)
#define NDIS_WLAN_WAKE_ON_NLO_DISCOVERY_SUPPORTED                 0x00000001
#define NDIS_WLAN_WAKE_ON_AP_ASSOCIATION_LOST_SUPPORTED           0x00000002
#define NDIS_WLAN_WAKE_ON_GTK_HANDSHAKE_ERROR_SUPPORTED           0x00000004
#define NDIS_WLAN_WAKE_ON_4WAY_HANDSHAKE_REQUEST_SUPPORTED        0x00000008
#endif // (NDIS_SUPPORT_NDIS630)
#if (NDIS_SUPPORT_NDIS688)
#define NDIS_WLAN_WAKE_ON_INCOMING_ACTION_FRAME_SUPPORTED         0x00000010
#endif // (NDIS_SUPPORT_NDIS688)
#if (NDIS_SUPPORT_NDIS689)
#define NDIS_WLAN_WAKE_ON_CLIENT_DRIVER_DIAGNOSTIC_SUPPORTED     0x00000020
#endif // (NDIS_SUPPORT_NDIS689)

//
// Flags for NDIS_PM_CAPABILITIES.MediaSpecificWakeUpEvents field
// to advertise media-specific wake capabilities for an adapter
// with physical media type NdisPhysicalMediumWirelessWan
//
#if (NDIS_SUPPORT_NDIS630)
#define NDIS_WWAN_WAKE_ON_REGISTER_STATE_SUPPORTED                0x00000001
#define NDIS_WWAN_WAKE_ON_SMS_RECEIVE_SUPPORTED                   0x00000002
#define NDIS_WWAN_WAKE_ON_USSD_RECEIVE_SUPPORTED                  0x00000004
#define NDIS_WWAN_WAKE_ON_PACKET_STATE_SUPPORTED                  0x00000008
#define NDIS_WWAN_WAKE_ON_UICC_CHANGE_SUPPORTED                   0x00000010
#endif // (NDIS_SUPPORT_NDIS630)

//
// Flags for NDIS_PM_CAPABILITIES.Flags
//
#if (NDIS_SUPPORT_NDIS630)
#define NDIS_PM_WAKE_PACKET_INDICATION_SUPPORTED                0x00000001
#define NDIS_PM_SELECTIVE_SUSPEND_SUPPORTED                     0x00000002
#endif // (NDIS_SUPPORT_NDIS630)


//
// Flags used in NDIS_PM_PARAMETERS struct
//
// Flags for NDIS_PM_PARAMETERS.EnabledWoLPacketPatterns field
//
#define NDIS_PM_WOL_BITMAP_PATTERN_ENABLED                      0x00000001
#define NDIS_PM_WOL_MAGIC_PACKET_ENABLED                        0x00000002
#define NDIS_PM_WOL_IPV4_TCP_SYN_ENABLED                        0x00000004
#define NDIS_PM_WOL_IPV6_TCP_SYN_ENABLED                        0x00000008
#define NDIS_PM_WOL_IPV4_DEST_ADDR_WILDCARD_ENABLED             0x00000200
#define NDIS_PM_WOL_IPV6_DEST_ADDR_WILDCARD_ENABLED             0x00000800
#define NDIS_PM_WOL_EAPOL_REQUEST_ID_MESSAGE_ENABLED            0x00010000

//
// Flags for NDIS_PM_PARAMETERS.EnabledProtocolOffloads field
//
#define NDIS_PM_PROTOCOL_OFFLOAD_ARP_ENABLED                    0x00000001
#define NDIS_PM_PROTOCOL_OFFLOAD_NS_ENABLED                     0x00000002
#define NDIS_PM_PROTOCOL_OFFLOAD_80211_RSN_REKEY_ENABLED        0x00000080

//
// Flags for NDIS_PM_PARAMETERS.WakeUpFlags field
//
#define NDIS_PM_WAKE_ON_LINK_CHANGE_ENABLED                     0x00000001

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_PM_WAKE_ON_MEDIA_DISCONNECT_ENABLED                0x00000002
#define NDIS_PM_SELECTIVE_SUSPEND_ENABLED                       0x00000010
#endif // (NDIS_SUPPORT_NDIS630)


//
// Flags for NDIS_PM_PARAMETERS.MediaSpecificWakeUpEvents field
// when miniport's physical media type is NdisPhysicalMediumNative802_11
//
#if (NDIS_SUPPORT_NDIS630)
#define NDIS_WLAN_WAKE_ON_NLO_DISCOVERY_ENABLED                 0x00000001
#define NDIS_WLAN_WAKE_ON_AP_ASSOCIATION_LOST_ENABLED           0x00000002
#define NDIS_WLAN_WAKE_ON_GTK_HANDSHAKE_ERROR_ENABLED           0x00000004
#define NDIS_WLAN_WAKE_ON_4WAY_HANDSHAKE_REQUEST_ENABLED        0x00000008
#endif // (NDIS_SUPPORT_NDIS630)
#if (NDIS_SUPPORT_NDIS688)
#define NDIS_WLAN_WAKE_ON_INCOMING_ACTION_FRAME_ENABLED         0x00000010
#endif // (NDIS_SUPPORT_NDIS688)
#if (NDIS_SUPPORT_NDIS689)
#define NDIS_WLAN_WAKE_ON_CLIENT_DRIVER_DIAGNOSTIC_ENABLED     0x00000020
#endif // (NDIS_SUPPORT_NDIS689)
//
// Flags for NDIS_PM_PARAMETERS.MediaSpecificWakeUpEvents field
// when miniport's physical media type is NdisPhysicalMediumWirelessWan
//
#if (NDIS_SUPPORT_NDIS630)
#define NDIS_WWAN_WAKE_ON_REGISTER_STATE_ENABLED                0x00000001
#define NDIS_WWAN_WAKE_ON_SMS_RECEIVE_ENABLED                   0x00000002
#define NDIS_WWAN_WAKE_ON_USSD_RECEIVE_ENABLED                  0x00000004
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS650)
#define NDIS_WWAN_WAKE_ON_PACKET_STATE_ENABLED                  0x00000008
#define NDIS_WWAN_WAKE_ON_UICC_CHANGE_ENABLED                   0x00000010
#endif // (NDIS_SUPPORT_NDIS650)


//
// Values used in Priority field of NDIS_PM_WOL_PATTERN
//
#define NDIS_PM_WOL_PRIORITY_LOWEST                             0xFFFFFFFF
#define NDIS_PM_WOL_PRIORITY_NORMAL                             0x10000000
#define NDIS_PM_WOL_PRIORITY_HIGHEST                            0x00000001


//
// Values used in Priority field of NDIS_PM_PROTOCOL_OFFLOAD
//
#define NDIS_PM_PROTOCOL_OFFLOAD_PRIORITY_LOWEST                0xFFFFFFFF
#define NDIS_PM_PROTOCOL_OFFLOAD_PRIORITY_NORMAL                0x10000000
#define NDIS_PM_PROTOCOL_OFFLOAD_PRIORITY_HIGHEST               0x00000001


//
// enum type for wake on lan patterns based on packet type
// used in WoLPacketType field of NDIS_PM_WOL_PATTERN structure
//
typedef enum _NDIS_PM_WOL_PACKET
{
    NdisPMWoLPacketUnspecified,
    NdisPMWoLPacketBitmapPattern,
    NdisPMWoLPacketMagicPacket,
    NdisPMWoLPacketIPv4TcpSyn,
    NdisPMWoLPacketIPv6TcpSyn,
    NdisPMWoLPacketEapolRequestIdMessage,
    NdisPMWoLPacketMaximum
}NDIS_PM_WOL_PACKET, *PNDIS_PM_WOL_PACKET;

//
// enum types for offloaded protocols used in ProtocolOffloadType
// field of NDIS_PM_PROTOCOL_OFFLOAD structure
//
typedef enum _NDIS_PM_PROTOCOL_OFFLOAD_TYPE
{
    NdisPMProtocolOffloadIdUnspecified,
    NdisPMProtocolOffloadIdIPv4ARP,
    NdisPMProtocolOffloadIdIPv6NS,
    NdisPMProtocolOffload80211RSNRekey,
#if (NDIS_SUPPORT_NDIS684)
    NdisPMProtocolOffload80211RSNRekeyV2,
#endif // (NDIS_SUPPORT_NDIS684)
    NdisPMProtocolOffloadIdMaximum
}NDIS_PM_PROTOCOL_OFFLOAD_TYPE, *PNDIS_PM_PROTOCOL_OFFLOAD_TYPE;

//
// Limited size string used in NSID power management structures
//
#define NDIS_PM_MAX_STRING_SIZE 64

typedef struct _NDIS_PM_COUNTED_STRING
{
    USHORT      Length; // in -Bytes-
    WCHAR       String[NDIS_PM_MAX_STRING_SIZE + 1];
} NDIS_PM_COUNTED_STRING, *PNDIS_PM_COUNTED_STRING;


//
// NDIS_PM_CAPABILITIES structure is used in PowerManagementCapabilitiesEx field of
// NDIS_MINIPORT_ADAPTER_GENERAL_ATTRIBUTES and NDIS_BIND_PARAMETERS
//
#define NDIS_PM_CAPABILITIES_REVISION_1              1

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_PM_CAPABILITIES_REVISION_2              2
#endif // (NDIS_SUPPORT_NDIS630)

typedef struct _NDIS_PM_CAPABILITIES
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_PM_CAPABILITIES_REVISION_2;
    // Header.Size = NDIS_SIZEOF_NDIS_PM_CAPABILITIES_REVISION_2;
    //
    NDIS_OBJECT_HEADER      Header;

    ULONG                   Flags;                          // NDIS_PM_XXX_SUPPORTED flags
    ULONG                   SupportedWoLPacketPatterns;     // NDIS_PM_WOL_XXX_SUPPORTED flags
    ULONG                   NumTotalWoLPatterns;
    ULONG                   MaxWoLPatternSize;              // maximum bytes that can be compared against a pattern
    ULONG                   MaxWoLPatternOffset;            // strting from MAC header, how many bytes in the packet can be examined
    ULONG                   MaxWoLPacketSaveBuffer;         // how many bytes of WOL packet can be saved to a buffer and indicated up
    ULONG                   SupportedProtocolOffloads;      // NDIS_PM_PROTOCOL_OFFLOAD_XXX_SUPPORTED flags
    ULONG                   NumArpOffloadIPv4Addresses;
    ULONG                   NumNSOffloadIPv6Addresses;
    NDIS_DEVICE_POWER_STATE MinMagicPacketWakeUp;
    NDIS_DEVICE_POWER_STATE MinPatternWakeUp;
    NDIS_DEVICE_POWER_STATE MinLinkChangeWakeUp;

#if (NDIS_SUPPORT_NDIS630)

    ULONG                   SupportedWakeUpEvents;          // NDIS_PM_WAKE_ON_XXX_SUPPORTED flags
    ULONG                   MediaSpecificWakeUpEvents;      // NDIS_{WLAN|WWAN}_WAKE_ON_XXX_SUPPORTED flags

#endif // (NDIS_SUPPORT_NDIS630)

}NDIS_PM_CAPABILITIES, *PNDIS_PM_CAPABILITIES;

#define NDIS_SIZEOF_NDIS_PM_CAPABILITIES_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_PM_CAPABILITIES, MinLinkChangeWakeUp)

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_SIZEOF_NDIS_PM_CAPABILITIES_REVISION_2     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_PM_CAPABILITIES, MediaSpecificWakeUpEvents)
#endif // (NDIS_SUPPORT_NDIS630)


//
// NDIS_PM_PARAMETERS structure is used in OID_PM_PARAMETERS for quering and
// updating currently enabled power management hardware capabilities.
//
#define NDIS_PM_PARAMETERS_REVISION_1              1

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_PM_PARAMETERS_REVISION_2              2
#endif // (NDIS_SUPPORT_NDIS630)

typedef struct _NDIS_PM_PARAMETERS
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_PM_PARAMETERS_REVISION_2;
    // Header.Size = NDIS_SIZEOF_NDIS_PM_PARAMETERS_REVISION_2;
    //
    NDIS_OBJECT_HEADER      Header;

    ULONG                   EnabledWoLPacketPatterns; // NDIS_PM_WOL_XXX_ENABLED flags
    ULONG                   EnabledProtocolOffloads;  // NDIS_PM_PROTOCOL_OFFLOAD_XXX_ENABLED flags
    ULONG                   WakeUpFlags;              // NDIS_PM_WAKE_ON_XXX_ENABLED flags

#if (NDIS_SUPPORT_NDIS630)
    ULONG                   MediaSpecificWakeUpEvents; // NDIS_{WLAN|WWAN}_WAKE_ON_XXX_ENABLED flags
#endif // (NDIS_SUPPORT_NDIS630)

} NDIS_PM_PARAMETERS, *PNDIS_PM_PARAMETERS;

#define NDIS_SIZEOF_NDIS_PM_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_PM_PARAMETERS, WakeUpFlags)

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_SIZEOF_NDIS_PM_PARAMETERS_REVISION_2     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_PM_PARAMETERS, MediaSpecificWakeUpEvents)
#endif // (NDIS_SUPPORT_NDIS630)


//
// Used in _EAPOL_REQUEST_ID_MESSAGE_WOL_PACKET_PARAMETERS
// When the following flag is set, the packet must be encrypted.
//
#define EAPOL_REQUEST_ID_WOL_FLAG_MUST_ENCRYPT        0x00000001

//
// Pattern IDs above this number are treated as pre-assigned
//
#define NDIS_PM_MAX_PATTERN_ID  0x0000FFFF


//
// This value is used in the Flags field of the NDIS_PM_WOL_PATTERN structure
// It indicates that the a pre-assigned ID is used
//
#define NDIS_PM_PRIVATE_PATTERN_ID  0x00000001


//
// structure used in OID_PM_ADD_WOL_PATTERN and OID_PM_WOL_PATTERN_LIST
//
#define NDIS_PM_WOL_PATTERN_REVISION_1              1

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_PM_WOL_PATTERN_REVISION_2              2
#endif // (NDIS_SUPPORT_NDIS630)


typedef struct _NDIS_PM_WOL_PATTERN
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_PM_WOL_PATTERN_REVISION_2;
    // Header.Size = NDIS_SIZEOF_NDIS_PM_WOL_PATTERN_REVISION_2;
    //
    NDIS_OBJECT_HEADER          Header;

    ULONG                       Flags;
    ULONG                       Priority;
    NDIS_PM_WOL_PACKET          WoLPacketType;
    NDIS_PM_COUNTED_STRING      FriendlyName;
    ULONG                       PatternId;   // Pattern ID set by NDIS
    ULONG                       NextWoLPatternOffset;

    union _WOL_PATTERN
    {
        //
        // IPv4 TCP SYN information
        //
        struct _IPV4_TCP_SYN_WOL_PACKET_PARAMETERS
        {
            ULONG   Flags;
            UCHAR   IPv4SourceAddress[4];   // IPv4 source address
            UCHAR   IPv4DestAddress[4];     // IPv4 destination address
            USHORT  TCPSourcePortNumber;    // TCP source port
            USHORT  TCPDestPortNumber;      // TCP destination port

        }IPv4TcpSynParameters;

        //
        // IPv6 TCP SYN information
        //
        struct _IPV6_TCP_SYN_WOL_PACKET_PARAMETERS
        {
            ULONG   Flags;
            UCHAR   IPv6SourceAddress[16];  // source IPv6 address
            UCHAR   IPv6DestAddress[16];    // destination IPv6 address
            USHORT  TCPSourcePortNumber;    // source TCP port
            USHORT  TCPDestPortNumber;      // destination TCP port
        }IPv6TcpSynParameters;

        //
        // 802.1X EAPOL request identity message parameters
        //
        struct _EAPOL_REQUEST_ID_MESSAGE_WOL_PACKET_PARAMETERS
        {
            ULONG   Flags;
        } EapolRequestIdMessageParameters;

        struct _WOL_BITMAP_PATTERN
        {
            ULONG   Flags;
            ULONG   MaskOffset;     // offset for mask buffer from the beginning of NDIS_PM_WOL_PATTERN structure
            ULONG   MaskSize;       // Mask size
            ULONG   PatternOffset;  // offset for pattern buffer from the beginning of NDIS_PM_WOL_PATTERN structure
            ULONG   PatternSize;    // pattern size
        }WoLBitMapPattern;
    }WoLPattern;

}NDIS_PM_WOL_PATTERN, *PNDIS_PM_WOL_PATTERN;


#define NDIS_SIZEOF_NDIS_PM_WOL_PATTERN_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_PM_WOL_PATTERN, WoLPattern)

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_SIZEOF_NDIS_PM_WOL_PATTERN_REVISION_2     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_PM_WOL_PATTERN, WoLPattern)
#endif // (NDIS_SUPPORT_NDIS630)


//
// 802.11 RSN handshake values
// Used in DOT11_RSN_REKEY_PARAMETERS structure
//
#define DOT11_RSN_KCK_LENGTH                  16
#define DOT11_RSN_KEK_LENGTH                  16

#if (NDIS_SUPPORT_NDIS684)
#define DOT11_RSN_MAX_CIPHER_KEY_LENGTH       32
#endif // (NDIS_SUPPORT_NDIS684)

//
// structure is used in OID_PM_ADD_PROTOCOL_OFFLOAD and
// OID_PM_PROTOCOL_OFFLOAD_LIST OID requests
//
#define NDIS_PM_PROTOCOL_OFFLOAD_REVISION_1             1
#if (NDIS_SUPPORT_NDIS684)
#define NDIS_PM_PROTOCOL_OFFLOAD_REVISION_2             2
#endif // (NDIS_SUPPORT_NDIS684)

typedef struct _NDIS_PM_PROTOCOL_OFFLOAD
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_PM_PROTOCOL_OFFLOAD_REVISION_1;
    // Header.Size = NDIS_SIZEOF_NDIS_PM_PROTOCOL_OFFLOAD_REVISION_1;
    //
    NDIS_OBJECT_HEADER            Header;
    ULONG                         Flags;
    ULONG                         Priority;
    NDIS_PM_PROTOCOL_OFFLOAD_TYPE ProtocolOffloadType;
    NDIS_PM_COUNTED_STRING        FriendlyName;
    ULONG                         ProtocolOffloadId; // offloaded protocol ID set by NDIS
    ULONG                         NextProtocolOffloadOffset;

    union _PROTOCOL_OFFLOAD_PARAMETERS
    {
        //
        // IPv4 ARP
        //
        struct _IPV4_ARP_PARAMETERS
        {
            ULONG   Flags;
            UCHAR   RemoteIPv4Address[4];   // source IPv4 address (optional)
            UCHAR   HostIPv4Address[4];     // destination IPv4 address
            UCHAR   MacAddress[6];          // MAC address
        }IPv4ARPParameters;

        //
        // IPv6 NS. ICMPv6 type 135
        // flags determine link local or global address. Discovery, reachability or dup address detection
        // multicat MAC address: 3333WXYZ where WXYZ is the least significant 4 bytes from
        // the solicited node IPv6 address
        //
        struct  _IPV6_NS_PARAMETERS
        {
            ULONG   Flags;
            UCHAR   RemoteIPv6Address[16];              // source IPv6 address (optional)
            UCHAR   SolicitedNodeIPv6Address[16];       // solicited node IPv6 address
            UCHAR   MacAddress[6];                      // MAC address
            UCHAR   TargetIPv6Addresses[2][16];         // An array of local IPv6 addesses
        }IPv6NSParameters;

        //
        // 802.11 RSN handshake
        //
        struct _DOT11_RSN_REKEY_PARAMETERS
        {
            ULONG   Flags;
            UCHAR   KCK[DOT11_RSN_KCK_LENGTH];
            UCHAR   KEK[DOT11_RSN_KEK_LENGTH];
            ULONGLONG KeyReplayCounter;
        }Dot11RSNRekeyParameters;

#if (NDIS_SUPPORT_NDIS684)
        //
        // KCK and KEK keys for drivers that support GTK offload- V2 is for WDI drivers only
        //
        struct _DOT11_RSN_REKEY_PARAMETERS_V2
        {
            ULONG       Flags;
            ULONGLONG   KeyReplayCounter;

            // KCK and KEK lengths are auth-specific
            ULONG       AuthAlgo;
            ULONG       KCKLength;
            ULONG       KEKLength;
            UCHAR       KCK [DOT11_RSN_MAX_CIPHER_KEY_LENGTH];
            UCHAR       KEK [DOT11_RSN_MAX_CIPHER_KEY_LENGTH];
        }Dot11RSNRekeyParametersV2;
#endif // (NDIS_SUPPORT_NDIS684)

    }ProtocolOffloadParameters;

}NDIS_PM_PROTOCOL_OFFLOAD, *PNDIS_PM_PROTOCOL_OFFLOAD;

//
// The size of the NDIS_PM_PROTOCOL_OFFLOAD struct for Rev1 was struct-aligned based on the union ProtocolOffloadParameters.
// However any macro that tries to recalculate this size from the new struct based on the size of the largest sub-struct in the older
// union (IPv6NSParameters) does not match the size of the overall struct, so its size is different from the original value.
// Hence we have to hard-code the size based on the value from the original struct.
//
#define NDIS_SIZEOF_NDIS_PM_PROTOCOL_OFFLOAD_REVISION_1     \
    0xF0

#if (NDIS_SUPPORT_NDIS684)
#define NDIS_SIZEOF_NDIS_PM_PROTOCOL_OFFLOAD_REVISION_2     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_PM_PROTOCOL_OFFLOAD, ProtocolOffloadParameters)   // 256 bytes
#endif // (NDIS_SUPPORT_NDIS684)

#if (NDIS_SUPPORT_NDIS630)

typedef enum _NDIS_PM_WAKE_REASON_TYPE
{
    NdisWakeReasonUnspecified               = 0x0000,
    NdisWakeReasonPacket                    = 0x0001,
    NdisWakeReasonMediaDisconnect           = 0x0002,
    NdisWakeReasonMediaConnect              = 0x0003,

    // WLAN-specific wake reasons
    NdisWakeReasonWlanNLODiscovery          = 0x1000,
    NdisWakeReasonWlanAPAssociationLost     = 0x1001,
    NdisWakeReasonWlanGTKHandshakeError     = 0x1002,
    NdisWakeReasonWlan4WayHandshakeRequest  = 0x1003,
#if (NDIS_SUPPORT_NDIS688)
    NdisWakeReasonWlanIncomingActionFrame   = 0x1004,
#endif // (NDIS_SUPPORT_NDIS688)
#if (NDIS_SUPPORT_NDIS689)
    NdisWakeReasonWlanClientDriverDiagnostic = 0x1005,
#endif // (NDIS_SUPPORT_NDIS689)
    // WWAN-specific wake reasons
    NdisWakeReasonWwanRegisterState         = 0x2000,
    NdisWakeReasonWwanSMSReceive            = 0x2001,
    NdisWakeReasonWwanUSSDReceive           = 0x2002,
    //0x2003 has been used for WakeReason.VendorSpecific in nids-nw.man
#if (NDIS_SUPPORT_NDIS684)
    NdisWakeReasonWwanPacketState           = 0x2004,
    NdisWakeReasonWwanUiccChange            = 0x2005,
#endif
} NDIS_PM_WAKE_REASON_TYPE, *PNDIS_PM_WAKE_REASON_TYPE;

//
// NDIS_PM_WAKE_REASON struct is used with
// the NDIS_STATUS_PM_WAKE_REASON status indication
//
#define NDIS_PM_WAKE_REASON_REVISION_1     1

typedef struct _NDIS_PM_WAKE_REASON
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_PM_WAKE_REASON_REVISION_1;
    // Header.Size = NDIS_SIZEOF_PM_WAKE_REASON_REVISION_1;
    //
    NDIS_OBJECT_HEADER          Header;

    ULONG                       Flags;                  // Reserved
    NDIS_PM_WAKE_REASON_TYPE    WakeReason;
    ULONG                       InfoBufferOffset;
    ULONG                       InfoBufferSize;
} NDIS_PM_WAKE_REASON, *PNDIS_PM_WAKE_REASON;

#define NDIS_SIZEOF_PM_WAKE_REASON_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_PM_WAKE_REASON, InfoBufferSize)


//
// NDIS_PM_WAKE_PACKET struct is used within NDIS_PM_WAKE_REASON's
// InfoBuffer, if the WakeReason is NdisWakeReasonPacket
//
#define NDIS_PM_WAKE_PACKET_REVISION_1     1

typedef struct _NDIS_PM_WAKE_PACKET
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_PM_WAKE_PACKET_REVISION_1;
    // Header.Size = NDIS_SIZEOF_PM_WAKE_PACKET_REVISION_1;
    //
    NDIS_OBJECT_HEADER      Header;

    ULONG                   Flags;                  // Reserved
    ULONG                   PatternId;              // ID of a corresponding wake pattern
    NDIS_PM_COUNTED_STRING  PatternFriendlyName;    // Pattern name filled by NDIS
    ULONG                   OriginalPacketSize;
    ULONG                   SavedPacketSize;
    ULONG                   SavedPacketOffset;
} NDIS_PM_WAKE_PACKET, *PNDIS_PM_WAKE_PACKET;

#define NDIS_SIZEOF_PM_WAKE_PACKET_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_PM_WAKE_PACKET, SavedPacketOffset)

#endif // (NDIS_SUPPORT_NDIS630)


//
// enum values used in NDIS_WMI_PM_ADMIN_CONFIG structure
//
typedef enum _NDIS_PM_ADMIN_CONFIG_STATE
{
    NdisPMAdminConfigUnspecified = 0,
    NdisPMAdminConfigDisabled = 1,
    NdisPMAdminConfigEnabled = 2
} NDIS_PM_ADMIN_CONFIG_STATE, *PNDIS_PM_ADMIN_CONFIG_STATE;

//
// NDIS_WMI_PM_ADMIN_CONFIG struct is used with
// GUID_NDIS_PM_ADMIN_CONFIG WMI method
//
#define NDIS_WMI_PM_ADMIN_CONFIG_REVISION_1     1

typedef struct _NDIS_WMI_PM_ADMIN_CONFIG
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_WMI_PM_ADMIN_CONFIG_REVISION_1;
    // Header.Size = NDIS_SIZEOF_WMI_PM_ADMIN_CONFIG_REVISION_1;
    //
    NDIS_OBJECT_HEADER Header;

    NDIS_PM_ADMIN_CONFIG_STATE   WakeOnPattern;
    NDIS_PM_ADMIN_CONFIG_STATE   WakeOnMagicPacket;
    NDIS_PM_ADMIN_CONFIG_STATE   DeviceSleepOnDisconnect;
    NDIS_PM_ADMIN_CONFIG_STATE   PMARPOffload;
    NDIS_PM_ADMIN_CONFIG_STATE   PMNSOffload;
    NDIS_PM_ADMIN_CONFIG_STATE   PMWiFiRekeyOffload;

} NDIS_WMI_PM_ADMIN_CONFIG, *PNDIS_WMI_PM_ADMIN_CONFIG;

#define NDIS_SIZEOF_WMI_PM_ADMIN_CONFIG_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_WMI_PM_ADMIN_CONFIG, PMWiFiRekeyOffload)

//
// enum values used in NDIS_WMI_PM_ACTIVE_CAPABILITIES structure
//
typedef enum _NDIS_PM_CAPABILITY_STATE
{
    NdisPMAdminConfigUnsupported = 0,
    NdisPMAdminConfigInactive = 1,
    NdisPMAdminConfigActive = 2
} NDIS_PM_CAPABILITY_STATE, *PNDIS_PM_CAPABILITY_STATE;

//
// NDIS_WMI_PM_ACTIVE_CAPABILITIES struct is used with
// GUID_NDIS_PM_ACTIVE_CAPABILITIES WMI method
//
#define NDIS_WMI_PM_ACTIVE_CAPABILITIES_REVISION_1     1

typedef struct _NDIS_WMI_PM_ACTIVE_CAPABILITIES
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_WMI_PM_ACTIVE_CAPABILITIES_REVISION_1;
    // Header.Size = NDIS_SIZEOF_WMI_PM_ACTIVE_CAPABILITIES_REVISION_1;
    //
    NDIS_OBJECT_HEADER Header;

    NDIS_PM_CAPABILITY_STATE   WakeOnPattern;
    NDIS_PM_CAPABILITY_STATE   WakeOnMagicPacket;
    NDIS_PM_CAPABILITY_STATE   DeviceSleepOnDisconnect;
    NDIS_PM_CAPABILITY_STATE   PMARPOffload;
    NDIS_PM_CAPABILITY_STATE   PMNSOffload;
    NDIS_PM_CAPABILITY_STATE   PMWiFiRekeyOffload;

} NDIS_WMI_PM_ACTIVE_CAPABILITIES, *PNDIS_WMI_PM_ACTIVE_CAPABILITIES;

#define NDIS_SIZEOF_WMI_PM_ACTIVE_CAPABILITIES_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_WMI_PM_ACTIVE_CAPABILITIES, PMWiFiRekeyOffload)

//
// receive filter data structures and definitions
//


//
// used in SupportedHeaders field of NDIS_RECEIVE_FILTER_CAPABILITIES
// structure
//
#define NDIS_RECEIVE_FILTER_MAC_HEADER_SUPPORTED            0x00000001
#define NDIS_RECEIVE_FILTER_IPV4_HEADER_SUPPORTED           0x00000002
#define NDIS_RECEIVE_FILTER_IPV6_HEADER_SUPPORTED           0x00000004
#define NDIS_RECEIVE_FILTER_ARP_HEADER_SUPPORTED            0x00000008
#define NDIS_RECEIVE_FILTER_UDP_HEADER_SUPPORTED            0x00000010

//
// used in SupportedMacHeaderFields field of NDIS_RECEIVE_FILTER_CAPABILITIES
// structure
//
#define NDIS_RECEIVE_FILTER_MAC_HEADER_DEST_ADDR_SUPPORTED      0x00000001
#define NDIS_RECEIVE_FILTER_MAC_HEADER_SOURCE_ADDR_SUPPORTED    0x00000002
#define NDIS_RECEIVE_FILTER_MAC_HEADER_PROTOCOL_SUPPORTED       0x00000004
#define NDIS_RECEIVE_FILTER_MAC_HEADER_VLAN_ID_SUPPORTED        0x00000008
#define NDIS_RECEIVE_FILTER_MAC_HEADER_PRIORITY_SUPPORTED       0x00000010
#define NDIS_RECEIVE_FILTER_MAC_HEADER_PACKET_TYPE_SUPPORTED    0x00000020

//
// used in SupportedARPHeaderFields field of NDIS_RECEIVE_FILTER_CAPABILITIES
// structure
//
#define NDIS_RECEIVE_FILTER_ARP_HEADER_OPERATION_SUPPORTED      0x00000001
#define NDIS_RECEIVE_FILTER_ARP_HEADER_SPA_SUPPORTED            0x00000002
#define NDIS_RECEIVE_FILTER_ARP_HEADER_TPA_SUPPORTED            0x00000004

//
// used in SupportedIPv4HeaderFields field of NDIS_RECEIVE_FILTER_CAPABILITIES
// structure
//
#define NDIS_RECEIVE_FILTER_IPV4_HEADER_PROTOCOL_SUPPORTED      0x00000001

//
// used in SupportedIPv6HeaderFields field of NDIS_RECEIVE_FILTER_CAPABILITIES
// structure
//
#define NDIS_RECEIVE_FILTER_IPV6_HEADER_PROTOCOL_SUPPORTED      0x00000001

//
// used in SupportedUdpHeaderFields field of NDIS_RECEIVE_FILTER_CAPABILITIES
// structure
//
#define NDIS_RECEIVE_FILTER_UDP_HEADER_DEST_PORT_SUPPORTED      0x00000001

//
// used in SupportedFilterTests field of NDIS_RECEIVE_FILTER_CAPABILITIES
// structure
//
#define NDIS_RECEIVE_FILTER_TEST_HEADER_FIELD_EQUAL_SUPPORTED               0x00000001
#define NDIS_RECEIVE_FILTER_TEST_HEADER_FIELD_MASK_EQUAL_SUPPORTED          0x00000002
#define NDIS_RECEIVE_FILTER_TEST_HEADER_FIELD_NOT_EQUAL_SUPPORTED           0x00000004

//
// used in SupportedQueueProperties field of NDIS_RECEIVE_FILTER_CAPABILITIES
// structure
//
#define NDIS_RECEIVE_FILTER_MSI_X_SUPPORTED                         0x00000001
#define NDIS_RECEIVE_FILTER_VM_QUEUE_SUPPORTED                      0x00000002
#define NDIS_RECEIVE_FILTER_LOOKAHEAD_SPLIT_SUPPORTED               0x00000004
#if (NDIS_SUPPORT_NDIS630)
#define NDIS_RECEIVE_FILTER_DYNAMIC_PROCESSOR_AFFINITY_CHANGE_SUPPORTED 0x00000008
#define NDIS_RECEIVE_FILTER_INTERRUPT_VECTOR_COALESCING_SUPPORTED       0x00000010
#define NDIS_RECEIVE_FILTER_IMPLAT_MIN_OF_QUEUES_MODE                   0x00000040
#define NDIS_RECEIVE_FILTER_IMPLAT_SUM_OF_QUEUES_MODE                   0x00000080
#define NDIS_RECEIVE_FILTER_PACKET_COALESCING_SUPPORTED_ON_DEFAULT_QUEUE 0x00000100

//
// This flag has been deprecated
//
#define NDIS_RECEIVE_FILTER_ANY_VLAN_SUPPORTED                          0x00000020  // deprecated
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS680)
#define NDIS_RECEIVE_FILTER_DYNAMIC_PROCESSOR_AFFINITY_CHANGE_FOR_DEFAULT_QUEUE_SUPPORTED 0x00000040
#endif // (NDIS_SUPPORT_NDIS680)

//
// The following bits are used in EnabledFilterTypes field of
// NDIS_RECEIVE_FILTER_GLOBAL_PARAMETERS structure and
// EnabledFilterTypes field of NDIS_RECEIVE_FILTER_CAPABILITIES structure
//
#define NDIS_RECEIVE_FILTER_VMQ_FILTERS_ENABLED                     0x00000001
#define NDIS_RECEIVE_FILTER_PACKET_COALESCING_FILTERS_ENABLED       0x00000002

//
// The following bits are used in EnabledQueueTypes field of
// NDIS_RECEIVE_FILTER_GLOBAL_PARAMETERS structure and
// EnabledQueueTypes field of NDIS_RECEIVE_FILTER_CAPABILITIES structure
//
#define NDIS_RECEIVE_FILTER_VM_QUEUES_ENABLED                       0x00000001

//
// Data structures for advertising generic filtering capabilities
// in ReceiveFilterCapabilities field of NDIS_MINIPORT_ADAPTER_HARDWARE_ASSIST_ATTRIBUTES
// structure, OID_RECEIVE_FILTER_HARDWARE_CAPABILITIES  and
// OID_RECEIVE_FILTER_CURRENT_CAPABILITIES query OIDs. This structure is also
// used in NDIS_STATUS_RECEIVE_FILTER_CURRENT_CAPABILITIES and
// NDIS_STATUS_RECEIVE_FILTER_HARDWARE_CAPABILITIES structure
//
#define NDIS_RECEIVE_FILTER_CAPABILITIES_REVISION_1     1
#if (NDIS_SUPPORT_NDIS630)
#define NDIS_RECEIVE_FILTER_CAPABILITIES_REVISION_2     2
#endif // (NDIS_SUPPORT_NDIS630)

typedef struct _NDIS_RECEIVE_FILTER_CAPABILITIES
{
    _In_  NDIS_OBJECT_HEADER          Header;
    _In_  ULONG                       Flags;
    _In_  ULONG                       EnabledFilterTypes;
    _In_  ULONG                       EnabledQueueTypes;
    _In_  ULONG                       NumQueues;
    _In_  ULONG                       SupportedQueueProperties;
    _In_  ULONG                       SupportedFilterTests;
    _In_  ULONG                       SupportedHeaders;
    _In_  ULONG                       SupportedMacHeaderFields;
    _In_  ULONG                       MaxMacHeaderFilters;
    _In_  ULONG                       MaxQueueGroups;
    _In_  ULONG                       MaxQueuesPerQueueGroup;
    _In_  ULONG                       MinLookaheadSplitSize;
    _In_  ULONG                       MaxLookaheadSplitSize;
#if (NDIS_SUPPORT_NDIS630)
    _In_  ULONG                       SupportedARPHeaderFields;
    _In_  ULONG                       SupportedIPv4HeaderFields;
    _In_  ULONG                       SupportedIPv6HeaderFields;
    _In_  ULONG                       SupportedUdpHeaderFields;
    _In_  ULONG                       MaxFieldTestsPerPacketCoalescingFilter;
    _In_  ULONG                       MaxPacketCoalescingFilters;
    _In_  ULONG                       NdisReserved;
#endif // (NDIS_SUPPORT_NDIS630)
} NDIS_RECEIVE_FILTER_CAPABILITIES, *PNDIS_RECEIVE_FILTER_CAPABILITIES;

#define NDIS_SIZEOF_RECEIVE_FILTER_CAPABILITIES_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_FILTER_CAPABILITIES, MaxLookaheadSplitSize)

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_SIZEOF_RECEIVE_FILTER_CAPABILITIES_REVISION_2     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_FILTER_CAPABILITIES, NdisReserved)
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS630)
//
// used in NicSwitchCapabilities field of NDIS_NIC_SWITCH_CAPABILITIES
// structure
//
#define NDIS_NIC_SWITCH_CAPS_VLAN_SUPPORTED                                                       0x00000001
#define NDIS_NIC_SWITCH_CAPS_PER_VPORT_INTERRUPT_MODERATION_SUPPORTED                             0x00000002
#define NDIS_NIC_SWITCH_CAPS_ASYMMETRIC_QUEUE_PAIRS_FOR_NONDEFAULT_VPORT_SUPPORTED                0x00000004
#define NDIS_NIC_SWITCH_CAPS_VF_RSS_SUPPORTED                                                     0x00000008
#define NDIS_NIC_SWITCH_CAPS_SINGLE_VPORT_POOL                                                    0x00000010
#if (NDIS_SUPPORT_NDIS650)
#define NDIS_NIC_SWITCH_CAPS_RSS_PARAMETERS_PER_PF_VPORT_SUPPORTED                                0x00000020
#endif // (NDIS_SUPPORT_NDIS650)
#if (NDIS_SUPPORT_NDIS660)
#define NDIS_NIC_SWITCH_CAPS_NIC_SWITCH_WITHOUT_IOV_SUPPORTED                                     0x00000040
#define NDIS_NIC_SWITCH_CAPS_RSS_ON_PF_VPORTS_SUPPORTED                                           0x00000080
#define NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_INDIRECTION_TABLE_SUPPORTED                         0x00000100
#define NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_HASH_FUNCTION_SUPPORTED                             0x00000200
#define NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_HASH_TYPE_SUPPORTED                                 0x00000400
#define NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_HASH_KEY_SUPPORTED                                  0x00000800
#define NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_INDIRECTION_TABLE_SIZE_RESTRICTED                   0x00001000

#define NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_MASK                             \
            (NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_HASH_KEY_SUPPORTED |        \
             NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_HASH_FUNCTION_SUPPORTED |   \
             NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_HASH_TYPE_SUPPORTED |       \
             NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_INDIRECTION_TABLE_SUPPORTED)
#endif // (NDIS_SUPPORT_NDIS660)

#endif // (NDIS_SUPPORT_NDIS630)

//
// Data structure for advertising the NIC Switch capabilities
// Used in HardwareNicSwitchCapabilities and CurrentNicSwitchCapabilities
// fields of NDIS_MINIPORT_ADAPTER_HARDWARE_ASSIST_ATTRIBUTES,
// OID_NIC_SWITCH_HARDWARE_CAPABILITIES and OID_NIC_SWITCH_CURRENT_CAPABILITIES. Also used
// in NicSwitchCapabilities field of NDIS_BIND_PARAMETERS and NDIS_FILTER_ATTACH_PARAMETERS structure
// Also used in NDIS_STATUS_NIC_SWITCH_CURRENT_CAPABILITIES and NDIS_STATUS_NIC_SWITCH_HARDWARE_CAPABILITIES
//
#define NDIS_NIC_SWITCH_CAPABILITIES_REVISION_1        1

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_NIC_SWITCH_CAPABILITIES_REVISION_2        2
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS660)
#define NDIS_NIC_SWITCH_CAPABILITIES_REVISION_3        3
#endif // (NDIS_SUPPORT_NDIS660)

typedef struct _NDIS_NIC_SWITCH_CAPABILITIES
{
    _In_  NDIS_OBJECT_HEADER          Header;
    _In_  ULONG                       Flags;
    _In_  ULONG                       NdisReserved1;
    _In_  ULONG                       NumTotalMacAddresses;
    _In_  ULONG                       NumMacAddressesPerPort;
    _In_  ULONG                       NumVlansPerPort;
    _In_  ULONG                       NdisReserved2;
    _In_  ULONG                       NdisReserved3;
#if (NDIS_SUPPORT_NDIS630)
    _In_  ULONG                       NicSwitchCapabilities;
    _In_  ULONG                       MaxNumSwitches;
    _In_  ULONG                       MaxNumVPorts;
    _In_  ULONG                       NdisReserved4;
    _In_  ULONG                       MaxNumVFs;
    _In_  ULONG                       MaxNumQueuePairs;
    _In_  ULONG                       NdisReserved5;
    _In_  ULONG                       NdisReserved6;
    _In_  ULONG                       NdisReserved7;
    _In_  ULONG                       MaxNumQueuePairsPerNonDefaultVPort;
    _In_  ULONG                       NdisReserved8;
    _In_  ULONG                       NdisReserved9;
    _In_  ULONG                       NdisReserved10;
    _In_  ULONG                       NdisReserved11;
    _In_  ULONG                       NdisReserved12;
    _In_  ULONG                       MaxNumMacAddresses;
    _In_  ULONG                       NdisReserved13;
    _In_  ULONG                       NdisReserved14;
    _In_  ULONG                       NdisReserved15;
    _In_  ULONG                       NdisReserved16;
    _In_  ULONG                       NdisReserved17;
#endif // (NDIS_SUPPORT_NDIS630)
#if (NDIS_SUPPORT_NDIS660)
    _In_  ULONG                       MaxNumRssCapableNonDefaultPFVPorts;
    _In_  ULONG                       NumberOfIndirectionTableEntriesForDefaultVPort;
    _In_  ULONG                       NumberOfIndirectionTableEntriesPerNonDefaultPFVPort;
    _In_  ULONG                       MaxNumQueuePairsForDefaultVPort;
#endif // (NDIS_SUPPORT_NDIS660)
}NDIS_NIC_SWITCH_CAPABILITIES, *PNDIS_NIC_SWITCH_CAPABILITIES;

#define NDIS_SIZEOF_NIC_SWITCH_CAPABILITIES_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_CAPABILITIES, NdisReserved3)

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_SIZEOF_NIC_SWITCH_CAPABILITIES_REVISION_2     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_CAPABILITIES, NdisReserved17)
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS660)
#define NDIS_SIZEOF_NIC_SWITCH_CAPABILITIES_REVISION_3     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_CAPABILITIES, \
                             MaxNumQueuePairsForDefaultVPort)
#endif // (NDIS_SUPPORT_NDIS660)

//
// NDIS_RECEIVE_FILTER_GLOBAL_PARAMETERS is used in
// OID_RECEIVE_FILTER_GLOBAL_PARAMETERS query OID
// in order to query particular aspects
// of receive filtering for a miniport adapter
//
#define NDIS_RECEIVE_FILTER_GLOBAL_PARAMETERS_REVISION_1       1
typedef struct _NDIS_RECEIVE_FILTER_GLOBAL_PARAMETERS
{
    _In_  NDIS_OBJECT_HEADER          Header;
    _In_  ULONG                       Flags;
    _In_  ULONG                       EnabledFilterTypes;
    _In_  ULONG                       EnabledQueueTypes;
} NDIS_RECEIVE_FILTER_GLOBAL_PARAMETERS, *PNDIS_RECEIVE_FILTER_GLOBAL_PARAMETERS;

#define NDIS_SIZEOF_RECEIVE_FILTER_GLOBAL_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_FILTER_GLOBAL_PARAMETERS, EnabledQueueTypes)

typedef ULONG NDIS_RECEIVE_QUEUE_ID, *PNDIS_RECEIVE_QUEUE_ID;
typedef ULONG NDIS_RECEIVE_QUEUE_GROUP_ID, *PNDIS_RECEIVE_QUEUE_GROUP_ID;

#define NDIS_DEFAULT_RECEIVE_QUEUE_ID               0
#define NDIS_DEFAULT_RECEIVE_QUEUE_GROUP_ID         0
#define NDIS_DEFAULT_RECEIVE_FILTER_ID              0

typedef ULONG NDIS_RECEIVE_FILTER_ID, *PNDIS_RECEIVE_FILTER_ID;

typedef enum _NDIS_RECEIVE_FILTER_TYPE
{
    NdisReceiveFilterTypeUndefined,
    NdisReceiveFilterTypeVMQueue,
    NdisReceiveFilterTypePacketCoalescing,
    NdisReceiveFilterTypeMaximum
}NDIS_RECEIVE_FILTER_TYPE, *PNDIS_RECEIVE_FILTER_TYPE;

typedef enum _NDIS_FRAME_HEADER
{
    NdisFrameHeaderUndefined,
    NdisFrameHeaderMac,
    NdisFrameHeaderArp,
    NdisFrameHeaderIPv4,
    NdisFrameHeaderIPv6,
    NdisFrameHeaderUdp,
    NdisFrameHeaderMaximum
}NDIS_FRAME_HEADER, *PNDIS_FRAME_HEADER;

typedef enum _NDIS_MAC_HEADER_FIELD
{
    NdisMacHeaderFieldUndefined,
    NdisMacHeaderFieldDestinationAddress,
    NdisMacHeaderFieldSourceAddress,
    NdisMacHeaderFieldProtocol,
    NdisMacHeaderFieldVlanId,
    NdisMacHeaderFieldPriority,
    NdisMacHeaderFieldPacketType,
    NdisMacHeaderFieldMaximum
}NDIS_MAC_HEADER_FIELD, *PNDIS_MAC_HEADER_FIELD;

typedef enum _NDIS_MAC_PACKET_TYPE
{
    NdisMacPacketTypeUndefined = 0,
    NdisMacPacketTypeUnicast = 1,
    NdisMacPacketTypeMulticast = 2,
    NdisMacPacketTypeBroadcast = 3,
    NdisMacPacketTypeMaximum
} NDIS_MAC_PACKET_TYPE, *PNDIS_MAC_PACKET_TYPE;

typedef enum _NDIS_ARP_HEADER_FIELD
{
    NdisARPHeaderFieldUndefined,
    NdisARPHeaderFieldOperation, // request or response
    NdisARPHeaderFieldSPA, // source protocol address
    NdisARPHeaderFieldTPA, //  target protocol address
    NdisARPHeaderFieldMaximum
} NDIS_ARP_HEADER_FIELD, *PNDIS_ARP_HEADER_FIELD;

typedef enum _NDIS_IPV4_HEADER_FIELD
{
    NdisIPv4HeaderFieldUndefined,
    NdisIPv4HeaderFieldProtocol,
    NdisIPv4HeaderFieldMaximum
}NDIS_IPV4_HEADER_FIELD, *PNDIS_IPV4_HEADER_FIELD;

typedef enum _NDIS_IPV6_HEADER_FIELD
{
    NdisIPv6HeaderFieldUndefined,
    NdisIPv6HeaderFieldProtocol,
    NdisIPv6HeaderFieldMaximum
}NDIS_IPV6_HEADER_FIELD, *PNDIS_IPV6_HEADER_FIELD;

typedef enum _NDIS_UDP_HEADER_FIELD
{
    NdisUdpHeaderFieldUndefined,
    NdisUdpHeaderFieldDestinationPort,
    NdisUdpHeaderFieldMaximum
}NDIS_UDP_HEADER_FIELD, *PNDIS_UDP_HEADER_FIELD;

typedef enum _NDIS_RECEIVE_FILTER_TEST
{
    NdisReceiveFilterTestUndefined,
    NdisReceiveFilterTestEqual,
    NdisReceiveFilterTestMaskEqual,
    NdisReceiveFilterTestNotEqual,
    NdisReceiveFilterTestMaximum
}NDIS_RECEIVE_FILTER_TEST, *PNDIS_RECEIVE_FILTER_TEST;

//
// Flags used in NDIS_RECEIVE_FILTER_FIELD_PARAMETERS.Flags field
//
#define NDIS_RECEIVE_FILTER_FIELD_MAC_HEADER_VLAN_UNTAGGED_OR_ZERO  0x00000001
#define NDIS_RECEIVE_FILTER_RESERVED                                0x000000FE

//
// NDIS_RECEIVE_FILTER_FIELD_PARAMETERS is used in
// NDIS_RECEIVE_FILTER_PARAMETERS structure
//
#define NDIS_RECEIVE_FILTER_FIELD_PARAMETERS_REVISION_1       1
#if (NDIS_SUPPORT_NDIS630)
#define NDIS_RECEIVE_FILTER_FIELD_PARAMETERS_REVISION_2       2
#endif // (NDIS_SUPPORT_NDIS630)

typedef struct _NDIS_RECEIVE_FILTER_FIELD_PARAMETERS
{
    _In_ NDIS_OBJECT_HEADER       Header;
    _In_ ULONG                    Flags;
    _In_ NDIS_FRAME_HEADER        FrameHeader;
    _In_ NDIS_RECEIVE_FILTER_TEST ReceiveFilterTest;
    _In_ union _HEADER_FIELD
    {
        NDIS_MAC_HEADER_FIELD       MacHeaderField;
        NDIS_ARP_HEADER_FIELD       ArpHeaderField;
        NDIS_IPV4_HEADER_FIELD      IPv4HeaderField;
        NDIS_IPV6_HEADER_FIELD      IPv6HeaderField;
        NDIS_UDP_HEADER_FIELD       UdpHeaderField;
    }HeaderField;

    _In_ union _FIELD_VALUE
    {
        UCHAR               FieldByteValue;
        USHORT              FieldShortValue;
        ULONG               FieldLongValue;
        ULONG64             FieldLong64Value;
        UCHAR               FieldByteArrayValue[16];
    }FieldValue;

    _In_ union _RESULT_VALUE                          // used when test operation is MaskEqual
    {
        UCHAR               ResultByteValue;
        USHORT              ResultShortValue;
        ULONG               ResultLongValue;
        ULONG64             ResultLong64Value;
        UCHAR               ResultByteArrayValue[16];
    }ResultValue;

}NDIS_RECEIVE_FILTER_FIELD_PARAMETERS, *PNDIS_RECEIVE_FILTER_FIELD_PARAMETERS;

#define NDIS_SIZEOF_RECEIVE_FILTER_FIELD_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_FILTER_FIELD_PARAMETERS, ResultValue)

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_SIZEOF_RECEIVE_FILTER_FIELD_PARAMETERS_REVISION_2     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_FILTER_FIELD_PARAMETERS, ResultValue)
#endif // (NDIS_SUPPORT_NDIS630)

#include <ndis/nicswitchtypes.h>

//
// Flags used in NDIS_RECEIVE_FILTER_PARAMETERS.Flags field
//
#define NDIS_RECEIVE_FILTER_FLAGS_RESERVED                          0x00000001
#define NDIS_RECEIVE_FILTER_PACKET_ENCAPSULATION_GRE                0x00000002
#if (NDIS_SUPPORT_NDIS650)
//
// NDIS_RECEIVE_FILTER_PACKET_ENCAPSULATION replaces the use of
// NDIS_RECEIVE_FILTER_PACKET_ENCAPSULATION_GRE as a generic flag to indicate
// an inner MAC receive filter for encapsulated packet task offloads.
//
#define NDIS_RECEIVE_FILTER_PACKET_ENCAPSULATION                    0x00000002
#endif // (NDIS_SUPPORT_NDIS650)

//
// NDIS_RECEIVE_FILTER_PARAMETERS is used in
// OID_RECEIVE_FILTER_PARAMETERS and OID_RECEIVE_FILTER_SET_FILTER
//
#define NDIS_RECEIVE_FILTER_PARAMETERS_REVISION_1       1

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_RECEIVE_FILTER_PARAMETERS_REVISION_2       2
#endif // (NDIS_SUPPORT_NDIS630)

typedef
  _When_(FieldParametersArrayNumElements == 0, _Struct_size_bytes_(sizeof(NDIS_RECEIVE_FILTER_PARAMETERS)))
  _When_(FieldParametersArrayNumElements > 0, _Struct_size_bytes_(FieldParametersArrayOffset +FieldParametersArrayNumElements*FieldParametersArrayElementSize))
    struct _NDIS_RECEIVE_FILTER_PARAMETERS
{
    _In_    NDIS_OBJECT_HEADER                     Header;
    _In_    ULONG                                  Flags;
    _In_    NDIS_RECEIVE_FILTER_TYPE               FilterType;
    _In_    NDIS_RECEIVE_QUEUE_ID                  QueueId;
    _Inout_ NDIS_RECEIVE_FILTER_ID                 FilterId;
    _In_    ULONG                                  FieldParametersArrayOffset; // from the beginning of this structure
    _In_    ULONG                                  FieldParametersArrayNumElements;
    _In_    ULONG                                  FieldParametersArrayElementSize;
    _In_    ULONG                                  RequestedFilterIdBitCount;
#if (NDIS_SUPPORT_NDIS630)
    _In_    ULONG                                  MaxCoalescingDelay; // valid only for a packet coalescing filter
    _In_    NDIS_NIC_SWITCH_VPORT_ID               VPortId;
#endif // (NDIS_SUPPORT_NDIS630)
}NDIS_RECEIVE_FILTER_PARAMETERS, *PNDIS_RECEIVE_FILTER_PARAMETERS;

#define NDIS_SIZEOF_RECEIVE_FILTER_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_FILTER_PARAMETERS, RequestedFilterIdBitCount)

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_SIZEOF_RECEIVE_FILTER_PARAMETERS_REVISION_2     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_FILTER_PARAMETERS, VPortId)
#endif // (NDIS_SUPPORT_NDIS630)

//
// NDIS_RECEIVE_FILTER_CLEAR_PARAMETERS is used in
// OID_RECEIVE_FILTER_CLEAR_FILTER
//
#define NDIS_RECEIVE_FILTER_CLEAR_PARAMETERS_REVISION_1       1

typedef struct _NDIS_RECEIVE_FILTER_CLEAR_PARAMETERS
{
    _In_ NDIS_OBJECT_HEADER                           Header;
    _In_ ULONG                                        Flags;
    _In_    NDIS_RECEIVE_QUEUE_ID                     QueueId;
    _In_ NDIS_RECEIVE_FILTER_ID                       FilterId;
}NDIS_RECEIVE_FILTER_CLEAR_PARAMETERS, *PNDIS_RECEIVE_FILTER_CLEAR_PARAMETERS;

#define NDIS_SIZEOF_RECEIVE_FILTER_CLEAR_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_FILTER_CLEAR_PARAMETERS, FilterId)

//
// enum value used in QueueType field of NDIS_RECEIVE_QUEUE_PARAMETERS structure
//
typedef enum _NDIS_RECEIVE_QUEUE_TYPE
{
    NdisReceiveQueueTypeUnspecified,
    NdisReceiveQueueTypeVMQueue,
    NdisReceiveQueueTypeMaximum
}NDIS_RECEIVE_QUEUE_TYPE, *PNDIS_RECEIVE_QUEUE_TYPE;

//
// bits used in Flags field of NDIS_RECEIVE_QUEUE_PARAMETERS in OID_RECEIVE_FILTER_ALLOCATE_QUEUE
// and OID_RECEIVE_FILTER_QUEUE_PARAMETERS -query- OID
//
#define NDIS_RECEIVE_QUEUE_PARAMETERS_PER_QUEUE_RECEIVE_INDICATION              0x00000001
#define NDIS_RECEIVE_QUEUE_PARAMETERS_LOOKAHEAD_SPLIT_REQUIRED                  0x00000002

//
// bits used in Flags field of NDIS_RECEIVE_QUEUE_PARAMETERS in OID_RECEIVE_FILTER_QUEUE_PARAMETERS -set- OID
//
#define NDIS_RECEIVE_QUEUE_PARAMETERS_FLAGS_CHANGED                             0x00010000
#define NDIS_RECEIVE_QUEUE_PARAMETERS_PROCESSOR_AFFINITY_CHANGED                0x00020000
#define NDIS_RECEIVE_QUEUE_PARAMETERS_SUGGESTED_RECV_BUFFER_NUMBERS_CHANGED     0x00040000
#define NDIS_RECEIVE_QUEUE_PARAMETERS_NAME_CHANGED                              0x00080000

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_RECEIVE_QUEUE_PARAMETERS_INTERRUPT_COALESCING_DOMAIN_ID_CHANGED    0x00100000
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS650)
#define NDIS_RECEIVE_QUEUE_PARAMETERS_QOS_SQ_ID_CHANGED                         0x00200000
#endif //(NDIS_SUPPORT_NDIS650)

#define NDIS_RECEIVE_QUEUE_PARAMETERS_CHANGE_MASK                               0xFFFF0000

typedef NDIS_IF_COUNTED_STRING NDIS_QUEUE_NAME, *PNDIS_QUEUE_NAME;
typedef NDIS_IF_COUNTED_STRING NDIS_VM_NAME, *PNDIS_VM_NAME;
typedef NDIS_IF_COUNTED_STRING NDIS_VM_FRIENDLYNAME, *PNDIS_VM_FRIENDLYNAME;
typedef NDIS_IF_COUNTED_STRING NDIS_SWITCH_PORT_PROPERTY_PROFILE_NAME, *PNDIS_SWITCH_PORT_PROPERTY_PROFILE_NAME;
typedef NDIS_IF_COUNTED_STRING NDIS_SWITCH_PORT_PROPERTY_PROFILE_CDN_LABEL, *PNDIS_SWITCH_PORT_PROPERTY_PROFILE_CDN_LABEL;
typedef NDIS_IF_COUNTED_STRING NDIS_SWITCH_NAME, *PNDIS_SWITCH_NAME;
typedef NDIS_IF_COUNTED_STRING NDIS_SWITCH_FRIENDLYNAME, *PNDIS_SWITCH_FRIENDLYNAME;
typedef NDIS_IF_COUNTED_STRING NDIS_SWITCH_PORT_NAME, *PNDIS_SWITCH_PORT_NAME;
typedef NDIS_IF_COUNTED_STRING NDIS_SWITCH_PORT_FRIENDLYNAME, *PNDIS_SWITCH_PORT_FRIENDLYNAME;
typedef NDIS_IF_COUNTED_STRING NDIS_SWITCH_NIC_NAME, *PNDIS_SWITCH_NIC_NAME;
typedef NDIS_IF_COUNTED_STRING NDIS_SWITCH_NIC_FRIENDLYNAME, *PNDIS_SWITCH_NIC_FRIENDLYNAME;
typedef NDIS_IF_COUNTED_STRING NDIS_SWITCH_EXTENSION_FRIENDLYNAME, *PNDIS_SWITCH_EXTENSION_FRIENDLYNAME;
typedef NDIS_IF_COUNTED_STRING NDIS_VENDOR_NAME, *PNDIS_VENDOR_NAME;

#if (NDIS_SUPPORT_NDIS650)
typedef ULONG NDIS_QOS_SQ_ID, *PNDIS_QOS_SQ_ID;
#endif //(NDIS_SUPPORT_NDIS650)

//
// NDIS_RECEIVE_QUEUE_PARAMETERS is used in
// OID_RECEIVE_FILTER_ALLOCATE_QUEUE and OID_RECEIVE_FILTER_QUEUE_PARAMETERS.
// This structure is also used in NDIS_STATUS_RECEIVE_FILTER_QUEUE_PARAMETERS
// status indication
//
#define NDIS_RECEIVE_QUEUE_PARAMETERS_REVISION_1       1
#if (NDIS_SUPPORT_NDIS630)
#define NDIS_RECEIVE_QUEUE_PARAMETERS_REVISION_2       2
#endif // (NDIS_SUPPORT_NDIS630)
#if (NDIS_SUPPORT_NDIS650)
#define NDIS_RECEIVE_QUEUE_PARAMETERS_REVISION_3       3
#endif // (NDIS_SUPPORT_NDIS650)

typedef struct _NDIS_RECEIVE_QUEUE_PARAMETERS
{
    _In_    NDIS_OBJECT_HEADER               Header;
    _Inout_ ULONG                            Flags;
    _In_    NDIS_RECEIVE_QUEUE_TYPE          QueueType;
    _Inout_ NDIS_RECEIVE_QUEUE_ID            QueueId;
    _Inout_ NDIS_RECEIVE_QUEUE_GROUP_ID      QueueGroupId;
    _In_    GROUP_AFFINITY                   ProcessorAffinity;
    _In_    ULONG                            NumSuggestedReceiveBuffers;
   _Out_    ULONG                            MSIXTableEntry;
    _In_    ULONG                            LookaheadSize;
    _In_    NDIS_VM_NAME                     VmName;
    _In_    NDIS_QUEUE_NAME                  QueueName;
#if (NDIS_SUPPORT_NDIS630)
    _In_    ULONG                            PortId;
    _Out_   ULONG                            InterruptCoalescingDomainId;
#endif // (NDIS_SUPPORT_NDIS630)
#if (NDIS_SUPPORT_NDIS650)
    _In_    NDIS_QOS_SQ_ID                   QosSqId;
#endif // (NDIS_SUPPORT_NDIS650)
}NDIS_RECEIVE_QUEUE_PARAMETERS, *PNDIS_RECEIVE_QUEUE_PARAMETERS;

#define NDIS_SIZEOF_RECEIVE_QUEUE_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_QUEUE_PARAMETERS, QueueName)

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_SIZEOF_RECEIVE_QUEUE_PARAMETERS_REVISION_2     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_QUEUE_PARAMETERS, InterruptCoalescingDomainId)
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS650)
#define NDIS_SIZEOF_RECEIVE_QUEUE_PARAMETERS_REVISION_3     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_QUEUE_PARAMETERS, QosSqId)
#endif // (NDIS_SUPPORT_NDIS650)

//
// NDIS_RECEIVE_QUEUE_FREE_PARAMETERS is used in
// OID_RECEIVE_FILTER_FREE_QUEUE
//
#define NDIS_RECEIVE_QUEUE_FREE_PARAMETERS_REVISION_1       1

typedef struct _NDIS_RECEIVE_QUEUE_FREE_PARAMETERS
{
    _In_ NDIS_OBJECT_HEADER                   Header;
    _In_ ULONG                                Flags;
    _In_ NDIS_RECEIVE_QUEUE_ID                QueueId;
}NDIS_RECEIVE_QUEUE_FREE_PARAMETERS, *PNDIS_RECEIVE_QUEUE_FREE_PARAMETERS;

#define NDIS_SIZEOF_RECEIVE_QUEUE_FREE_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_QUEUE_FREE_PARAMETERS, QueueId)

//
// the following enum type is used in NDIS_RECEIVE_QUEUE_STATE
// and NDIS_RECEIVE_QUEUE_INFO
//
typedef enum _NDIS_RECEIVE_QUEUE_OPERATIONAL_STATE
{
    NdisReceiveQueueOperationalStateUndefined,
    NdisReceiveQueueOperationalStateRunning,
    NdisReceiveQueueOperationalStatePaused,
    NdisReceiveQueueOperationalStateDmaStopped,
    NdisReceiveQueueOperationalStateMaximum
}NDIS_RECEIVE_QUEUE_OPERATIONAL_STATE, *PNDIS_RECEIVE_QUEUE_OPERATIONAL_STATE;

//
// NDIS_RECEIVE_QUEUE_INFO is used with NDIS_RECEIVE_QUEUE_INFO_ARRAY
// in OID_RECEIVE_FILTER_ENUM_QUEUES that enumerates receive queues
// on a miniport or open
//
#define NDIS_RECEIVE_QUEUE_INFO_REVISION_1              1

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_RECEIVE_QUEUE_INFO_REVISION_2              2
#endif // (NDIS_SUPPORT_NDIS630)


typedef struct _NDIS_RECEIVE_QUEUE_INFO
{
    NDIS_OBJECT_HEADER                  Header;
    ULONG                               Flags;
    NDIS_RECEIVE_QUEUE_TYPE             QueueType;
    NDIS_RECEIVE_QUEUE_ID               QueueId;
    NDIS_RECEIVE_QUEUE_GROUP_ID         QueueGroupId;
    NDIS_RECEIVE_QUEUE_OPERATIONAL_STATE QueueState;
    GROUP_AFFINITY                      ProcessorAffinity;
    ULONG                               NumSuggestedReceiveBuffers;
    ULONG                               MSIXTableEntry;
    ULONG                               LookaheadSize;
    NDIS_VM_NAME                        VmName;
    NDIS_QUEUE_NAME                     QueueName;
#if (NDIS_SUPPORT_NDIS630)
    ULONG                               NumFilters;
    ULONG                               InterruptCoalescingDomainId;
#endif // (NDIS_SUPPORT_NDIS630)
}NDIS_RECEIVE_QUEUE_INFO, *PNDIS_RECEIVE_QUEUE_INFO;

#define NDIS_SIZEOF_RECEIVE_QUEUE_INFO_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_QUEUE_INFO, QueueName)


#if (NDIS_SUPPORT_NDIS630)
#define NDIS_SIZEOF_RECEIVE_QUEUE_INFO_REVISION_2     \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_QUEUE_INFO, InterruptCoalescingDomainId)
#endif // (NDIS_SUPPORT_NDIS630)


//
// NDIS_RECEIVE_QUEUE_INFO_ARRAY is used in OID_RECEIVE_FILTER_ENUM_QUEUES
// that enumerates receive queues on a miniport or open. Each element in the
// array is an NDIS_RECEIVE_QUEUE_INFO structure
//
#define NDIS_RECEIVE_QUEUE_INFO_ARRAY_REVISION_1             1

typedef struct _NDIS_RECEIVE_QUEUE_INFO_ARRAY
{
    NDIS_OBJECT_HEADER                          Header;
    ULONG                                       FirstElementOffset;
    ULONG                                       NumElements;
    ULONG                                       ElementSize;
}NDIS_RECEIVE_QUEUE_INFO_ARRAY, *PNDIS_RECEIVE_QUEUE_INFO_ARRAY;

#define NDIS_SIZEOF_RECEIVE_QUEUE_INFO_ARRAY_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_QUEUE_INFO_ARRAY, ElementSize)


//
// NDIS_RECEIVE_FILTER_INFO is used with NDIS_RECEIVE_FILTER_INFO_ARRAY
// in OID_RECEIVE_FILTER_ENUM_FILTERS that enumerates receive
// filters on a queue
//
#define NDIS_RECEIVE_FILTER_INFO_REVISION_1             1

typedef struct _NDIS_RECEIVE_FILTER_INFO
{
    NDIS_OBJECT_HEADER                       Header;
    ULONG                                    Flags;
    NDIS_RECEIVE_FILTER_TYPE                 FilterType;
    NDIS_RECEIVE_FILTER_ID                   FilterId;
}NDIS_RECEIVE_FILTER_INFO, *PNDIS_RECEIVE_FILTER_INFO;

#define NDIS_SIZEOF_RECEIVE_FILTER_INFO_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_FILTER_INFO, FilterId)


//
// NDIS_RECEIVE_FILTER_INFO_ARRAY is used in OID_RECEIVE_FILTER_ENUM_FILTERS
// that enumerates receive filters on a queue or a VPort. Each element in the array
// is an NDIS_RECEIVE_FILTER_INFO structure
//
#define NDIS_RECEIVE_FILTER_INFO_ARRAY_REVISION_1             1
#if (NDIS_SUPPORT_NDIS630)
#define NDIS_RECEIVE_FILTER_INFO_ARRAY_REVISION_2             2
#endif // (NDIS_SUPPORT_NDIS630)

//
// flags used in NDIS_RECEIVE_FILTER_INFO_ARRAY to specify whether
// QueueId or VPortId is specified
//
#if (NDIS_SUPPORT_NDIS630)
#define NDIS_RECEIVE_FILTER_INFO_ARRAY_VPORT_ID_SPECIFIED       0x00000001
#endif

typedef struct _NDIS_RECEIVE_FILTER_INFO_ARRAY
{
    NDIS_OBJECT_HEADER                          Header;
    NDIS_RECEIVE_QUEUE_ID                       QueueId;
    ULONG                                       FirstElementOffset;
    ULONG                                       NumElements;
    ULONG                                       ElementSize;
#if (NDIS_SUPPORT_NDIS630)
    ULONG                                       Flags;
    NDIS_NIC_SWITCH_VPORT_ID                    VPortId;
#endif // (NDIS_SUPPORT_NDIS630)
}NDIS_RECEIVE_FILTER_INFO_ARRAY, *PNDIS_RECEIVE_FILTER_INFO_ARRAY;

#define NDIS_SIZEOF_RECEIVE_FILTER_INFO_ARRAY_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_FILTER_INFO_ARRAY, ElementSize)

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_SIZEOF_RECEIVE_FILTER_INFO_ARRAY_REVISION_2      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_FILTER_INFO_ARRAY, VPortId)
#endif // (NDIS_SUPPORT_NDIS630)

//
// NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_PARAMETERS is used
// in OID_RECEIVE_FILTER_QUEUE_ALLOCATION_COMPLETE OID.
//
#define NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_PARAMETERS_REVISION_1    1

typedef struct _NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_PARAMETERS
{
    _In_  NDIS_OBJECT_HEADER      Header;
    _In_  ULONG                   Flags;
    _In_  NDIS_RECEIVE_QUEUE_ID   QueueId;
    _Out_ NDIS_STATUS             CompletionStatus;
}NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_PARAMETERS, *PNDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_PARAMETERS;

#define NDIS_SIZEOF_RECEIVE_QUEUE_ALLOCATION_COMPLETE_PARAMETERS_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_PARAMETERS, CompletionStatus)

//
// NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_ARRAY is used in
// OID_RECEIVE_FILTER_QUEUE_ALLOCATION_COMPLETE OID request.
// Each element in array is an NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_PARAMETERS
// structure.
//
#define NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_ARRAY_REVISION_1         1

typedef
        _When_(NumElements >= 1,
            _Struct_size_bytes_(FirstElementOffset + NumElements*ElementSize))
        _When_(NumElements == 0,
            _Struct_size_bytes_(sizeof(NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_ARRAY)))
    struct _NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_ARRAY
{
    NDIS_OBJECT_HEADER                          Header;
    ULONG                                       Flags;
    ULONG                                       FirstElementOffset; // offset to an array of NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_PARAMETERS
    ULONG                                       NumElements;
    ULONG                                       ElementSize;
}NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_ARRAY, *PNDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_ARRAY;

#define NDIS_SIZEOF_RECEIVE_QUEUE_ALLOCATION_COMPLETE_ARRAY_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_ARRAY, ElementSize)

#endif // #if (NDIS_SUPPORT_NDIS620)

#if ((NTDDI_VERSION >= NTDDI_VISTA) || NDIS_SUPPORT_NDIS6)

#define NDIS_RECEIVE_SCALE_CAPABILITIES_REVISION_1     1

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_RECEIVE_SCALE_CAPABILITIES_REVISION_2     2
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS660)
#define NDIS_RECEIVE_SCALE_CAPABILITIES_REVISION_3     3
#endif // (NDIS_SUPPORT_NDIS660)


//
// What kind of hash field type the miniport can support.
// Important, the defined values for NDIS_RSS_CAPS_HASH_TYPE must allow for
// matching the NDIS_HASH_TYPE_MASK.  Referrence the NDIS_HASH_TYPE_MASK
// for more info.
//
#define NDIS_RSS_CAPS_HASH_TYPE_TCP_IPV4                    0x00000100
#define NDIS_RSS_CAPS_HASH_TYPE_TCP_IPV6                    0x00000200
#define NDIS_RSS_CAPS_HASH_TYPE_TCP_IPV6_EX                 0x00000400
#if (NDIS_SUPPORT_NDIS680)
    #define NDIS_RSS_CAPS_HASH_TYPE_UDP_IPV4                0x00000800
    #define NDIS_RSS_CAPS_HASH_TYPE_UDP_IPV6                0x00001000
    #define NDIS_RSS_CAPS_HASH_TYPE_UDP_IPV6_EX             0x00002000
#endif // (NDIS_SUPPORT_NDIS680)

//
// What kind the receive scale capabilities the miniport can support, miniport drivers return
// some of these flags as CapabilitiesFlags in a structure _NDIS_RECEIVE_SCALE_CAPABILITIES
// when they get a query about theire receive side scale capabilities.
//
#define NDIS_RSS_CAPS_MESSAGE_SIGNALED_INTERRUPTS           0x01000000
#define NDIS_RSS_CAPS_CLASSIFICATION_AT_ISR                 0x02000000
#define NDIS_RSS_CAPS_CLASSIFICATION_AT_DPC                 0x04000000
#if NDIS_SUPPORT_NDIS620
    #define NDIS_RSS_CAPS_USING_MSI_X                       0x08000000
#endif
#if (NDIS_SUPPORT_NDIS630)
    #define NDIS_RSS_CAPS_RSS_AVAILABLE_ON_PORTS            0x10000000
    #define NDIS_RSS_CAPS_SUPPORTS_MSI_X                    0x20000000
#endif // (NDIS_SUPPORT_NDIS630)
#if (NDIS_SUPPORT_NDIS680)
    #define NDIS_RSS_CAPS_SUPPORTS_INDEPENDENT_ENTRY_MOVE   0x40000000
#endif // (NDIS_SUPPORT_NDIS680)
//
// Typedef to use as flags holder to correlate to the NDIS_RSS_CAPS_ prefixed flags above.
//
typedef ULONG NDIS_RSS_CAPS_FLAGS;

//
// the following structure defines the Receive scale capabilities of the miniport
//
typedef struct _NDIS_RECEIVE_SCALE_CAPABILITIES
{
    NDIS_OBJECT_HEADER    Header;
    NDIS_RSS_CAPS_FLAGS   CapabilitiesFlags;
    ULONG                 NumberOfInterruptMessages;
    ULONG                 NumberOfReceiveQueues;
    #if (NDIS_SUPPORT_NDIS630)
    USHORT                NumberOfIndirectionTableEntries;
    #endif // (NDIS_SUPPORT_NDIS630)
} NDIS_RECEIVE_SCALE_CAPABILITIES, *PNDIS_RECEIVE_SCALE_CAPABILITIES;


#define NDIS_SIZEOF_RECEIVE_SCALE_CAPABILITIES_REVISION_1   \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_SCALE_CAPABILITIES, NumberOfReceiveQueues)

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_SIZEOF_RECEIVE_SCALE_CAPABILITIES_REVISION_2   \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_SCALE_CAPABILITIES, NumberOfIndirectionTableEntries)
#endif // (NDIS_SUPPORT_NDIS630)

#if (NDIS_SUPPORT_NDIS660)
#define NDIS_SIZEOF_RECEIVE_SCALE_CAPABILITIES_REVISION_3   \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_SCALE_CAPABILITIES, NumberOfIndirectionTableEntries)
#endif // (NDIS_SUPPORT_NDIS660)

#include <ndis/hashtypes.h>

//
// Flags to denote the parameters that are kept unmodified.
//
#define NDIS_RSS_PARAM_FLAG_BASE_CPU_UNCHANGED              0x0001
#define NDIS_RSS_PARAM_FLAG_HASH_INFO_UNCHANGED             0x0002
#define NDIS_RSS_PARAM_FLAG_ITABLE_UNCHANGED                0x0004
#define NDIS_RSS_PARAM_FLAG_HASH_KEY_UNCHANGED              0x0008
#define NDIS_RSS_PARAM_FLAG_DISABLE_RSS                     0x0010
#if (NDIS_SUPPORT_NDIS660)
#define NDIS_RSS_PARAM_FLAG_DEFAULT_PROCESSOR_UNCHANGED     0x0020
#endif // NDIS_SUPPORT_NDIS660

#define NDIS_RSS_INDIRECTION_TABLE_SIZE_REVISION_1          128
#define NDIS_RSS_HASH_SECRET_KEY_SIZE_REVISION_1            40

//
// used in OID_GEN_RECEIVE_SCALE_PARAMETERS
//
#define NDIS_RECEIVE_SCALE_PARAMETERS_REVISION_1     1

#if NDIS_SUPPORT_NDIS620

#define NDIS_RECEIVE_SCALE_PARAMETERS_REVISION_2     2

#endif

#if NDIS_SUPPORT_NDIS660

#define NDIS_RECEIVE_SCALE_PARAMETERS_REVISION_3     3

#endif

typedef struct _NDIS_RECEIVE_SCALE_PARAMETERS
{
    NDIS_OBJECT_HEADER      Header;

    // Qualifies the rest of the information.
    USHORT                  Flags;

    // The base CPU number to do receive processing. not used.
    USHORT                  BaseCpuNumber;

    // This describes the hash function and type being enabled.
    ULONG                   HashInformation;

    // The size of indirection table array.
    USHORT                  IndirectionTableSize;
    // The offset of the indirection table from the beginning of this structure.
    ULONG                   IndirectionTableOffset;

    // The size of the secret key.
    USHORT                  HashSecretKeySize;
    // The offset of the secret key from the beginning of this structure.
    ULONG                   HashSecretKeyOffset;

#if NDIS_SUPPORT_NDIS620
    ULONG                   ProcessorMasksOffset;     //
    ULONG                   NumberOfProcessorMasks;   // Array of type GROUP_AFFINITY representing procs used in the indirection table
    ULONG                   ProcessorMasksEntrySize;  //
#endif

    // The hash map table is a CCHAR array for Revision 1.
    // It is a PROCESSOR_NUMBER array for Revision 2

#if NDIS_SUPPORT_NDIS660
    // Specifies default RSS processor.
    PROCESSOR_NUMBER        DefaultProcessorNumber;
#endif
} NDIS_RECEIVE_SCALE_PARAMETERS, *PNDIS_RECEIVE_SCALE_PARAMETERS;

#define NDIS_SIZEOF_RECEIVE_SCALE_PARAMETERS_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_SCALE_PARAMETERS, HashSecretKeyOffset)

//
// Maximum indirection table and private key sizes for revision 1
//
#define NDIS_RSS_INDIRECTION_TABLE_MAX_SIZE_REVISION_1      128
#define NDIS_RSS_HASH_SECRET_KEY_MAX_SIZE_REVISION_1        40

#if NDIS_SUPPORT_NDIS620

#define NDIS_SIZEOF_RECEIVE_SCALE_PARAMETERS_REVISION_2    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_SCALE_PARAMETERS, ProcessorMasksEntrySize)

//
// Maximum indirection table and private key sizes for revision 2
//
#define NDIS_RSS_INDIRECTION_TABLE_MAX_SIZE_REVISION_2      (128*sizeof(PROCESSOR_NUMBER))
#define NDIS_RSS_HASH_SECRET_KEY_MAX_SIZE_REVISION_2        40

#endif

#if NDIS_SUPPORT_NDIS660

#define NDIS_SIZEOF_RECEIVE_SCALE_PARAMETERS_REVISION_3    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_SCALE_PARAMETERS, DefaultProcessorNumber)

//
// Maximum indirection table and private key sizes for revision 3
//
#define NDIS_RSS_INDIRECTION_TABLE_MAX_SIZE_REVISION_3      (128*sizeof(PROCESSOR_NUMBER))
#define NDIS_RSS_HASH_SECRET_KEY_MAX_SIZE_REVISION_3        40

#endif


#if NDIS_SUPPORT_NDIS680

//
// Parameter structure used with OID_GEN_RECEIVE_SCALE_PARAMETERS_V2
//
#define NDIS_RECEIVE_SCALE_PARAMETERS_V2_REVISION_1         1

#define NDIS_RECEIVE_SCALE_PARAM_ENABLE_RSS                 0x00000001
#define NDIS_RECEIVE_SCALE_PARAM_HASH_INFO_CHANGED          0x00000002
#define NDIS_RECEIVE_SCALE_PARAM_HASH_KEY_CHANGED           0x00000004
#define NDIS_RECEIVE_SCALE_PARAM_NUMBER_OF_QUEUES_CHANGED   0x00000008
#define NDIS_RECEIVE_SCALE_PARAM_NUMBER_OF_ENTRIES_CHANGED  0x00000010

typedef struct _NDIS_RECEIVE_SCALE_PARAMETERS_V2
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_RSS_PARAMETERS_V2;
    // Header.Revision = NDIS_RECEIVE_SCALE_PARAMETERS_V2_REVISION_1;
    // Header.Size = sizeof(NDIS_RECEIVE_SCALE_PARAMETERS_V2);
    //
    NDIS_OBJECT_HEADER      Header;

    // Qualifies the rest of the information.
    ULONG                   Flags;

    // This describes the hash function and type being enabled.
    ULONG                   HashInformation;

    // The size of the secret key.
    ULONG                   HashSecretKeySize;
    // The offset of the secret key from the beginning of this structure.
    ULONG                   HashSecretKeyOffset;

    // Number of queues
    ULONG                   NumberOfQueues;

    // Number of entries in indirection table. Must be power of two.
    ULONG                   NumberOfIndirectionTableEntries;

} NDIS_RECEIVE_SCALE_PARAMETERS_V2, *PNDIS_RECEIVE_SCALE_PARAMETERS_V2;

#define NDIS_SIZEOF_RECEIVE_SCALE_PARAMETERS_V2_REVISION_1 \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_SCALE_PARAMETERS_V2, \
                                 NumberOfIndirectionTableEntries)

//
// Flag which tells that NDIS_RSS_SET_INDIRECTION_ENTRY is referring to
// "Primary Processor" of the scaling entity (e.g. VPort).
// IndirectionTableIndex is not used.
//
#define NDIS_RSS_SET_INDIRECTION_ENTRY_FLAG_PRIMARY_PROCESSOR   0x00000001

//
// Flag which tells that NDIS_RSS_SET_INDIRECTION_ENTRY is referring to
// "Default Processor" of the scaling entity (e.g. VPort).
// IndirectionTableIndex is not used.
//
#define NDIS_RSS_SET_INDIRECTION_ENTRY_FLAG_DEFAULT_PROCESSOR   0x00000002

//
// Command to set a single indirection table entry.
//
typedef struct _NDIS_RSS_SET_INDIRECTION_ENTRY
{
    // Nic Switch where VPort is residing
    NDIS_NIC_SWITCH_ID          SwitchId;

    // VPort idintifier
    NDIS_NIC_SWITCH_VPORT_ID    VPortId;

    // Qualifies the information in this structure.
    ULONG                       Flags;

    // Indirection table entry which is being moved.
    USHORT                      IndirectionTableIndex;

    // Target processor number
    PROCESSOR_NUMBER            TargetProcessorNumber;

    // Result of the set operation described by this entry.
    NDIS_STATUS                 EntryStatus;

} NDIS_RSS_SET_INDIRECTION_ENTRY, *PNDIS_RSS_SET_INDIRECTION_ENTRY;

#define NDIS_SIZEOF_RSS_SET_INDIRECTION_ENTRY_REVISION_1 \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RSS_SET_INDIRECTION_ENTRY, EntryStatus)

//
// This structure is a parameter for synchronous direct
// OID_GEN_RSS_SET_INDIRECTION_TABLE_ENTRIES.
//
#define NDIS_RSS_SET_INDIRECTION_ENTRIES_REVISION_1     1

typedef struct _NDIS_RSS_SET_INDIRECTION_ENTRIES
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_RSS_SET_INDIRECTION_ENTRIES;
    // Header.Revision = NDIS_RSS_SET_INDIRECTION_ENTRIES_REVISION_1;
    // Header.Size = sizeof(NDIS_RSS_SET_INDIRECTION_ENTRIES);
    //
    NDIS_OBJECT_HEADER      Header;

    // Qualifies the rest of the information, as well as array processing policy
    ULONG                   Flags;

    // RssEntrySize is an opaque number that needs to be added to the pointer
    // during array traversal.
    ULONG                   RssEntrySize;

    // Offset of NDIS_RSS_SET_INDIRECTION_ENTRY array from the beginning of
    // this structure.
    ULONG                   RssEntryTableOffset;

    // The number of NDIS_RSS_SET_INDIRECTION_ENTRY elements in the array.
    ULONG                   NumberOfRssEntries;

} NDIS_RSS_SET_INDIRECTION_ENTRIES, *PNDIS_RSS_SET_INDIRECTION_ENTRIES;

#define NDIS_SIZEOF_RSS_SET_INDIRECTION_ENTRIES_REVISION_1 \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RSS_SET_INDIRECTION_ENTRIES, \
                                 NumberOfRssEntries)

#endif // NDIS_SUPPORT_NDIS680

//
// Used in OID_GEN_RECEIVE_HASH
//
#define NDIS_RECEIVE_HASH_FLAG_ENABLE_HASH                      0x00000001
#define NDIS_RECEIVE_HASH_FLAG_HASH_INFO_UNCHANGED              0x00000002
#define NDIS_RECEIVE_HASH_FLAG_HASH_KEY_UNCHANGED               0x00000004

#define NDIS_RECEIVE_HASH_PARAMETERS_REVISION_1                 1

typedef struct _NDIS_RECEIVE_HASH_PARAMETERS
{
    NDIS_OBJECT_HEADER      Header;

    // Qualifies the rest of the information.
    ULONG                   Flags;

    // This describes the hash function and type being enabled.
    ULONG                   HashInformation;

    // The size of the secret key.
    USHORT                  HashSecretKeySize;

    // The offset of the secret key from the beginning of this structure.
    ULONG                   HashSecretKeyOffset;
} NDIS_RECEIVE_HASH_PARAMETERS, *PNDIS_RECEIVE_HASH_PARAMETERS;

#define NDIS_SIZEOF_RECEIVE_HASH_PARAMETERS_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_HASH_PARAMETERS, HashSecretKeyOffset)


typedef enum _NDIS_PROCESSOR_VENDOR
{
    NdisProcessorVendorUnknown,
    NdisProcessorVendorGenuinIntel,
    NdisProcessorVendorGenuineIntel = NdisProcessorVendorGenuinIntel,
    NdisProcessorVendorAuthenticAMD
} NDIS_PROCESSOR_VENDOR, *PNDIS_PROCESSOR_VENDOR;

#if NDIS_SUPPORT_NDIS620

//
// Used in the RssProcessorArray in NDIS_RSS_PROCESSOR_INFO
//
typedef struct _NDIS_RSS_PROCESSOR
{
    PROCESSOR_NUMBER ProcNum;
    USHORT           PreferenceIndex;
    USHORT           Reserved;
} NDIS_RSS_PROCESSOR, *PNDIS_RSS_PROCESSOR;

#define NDIS_SIZEOF_RSS_PROCESSOR_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RSS_PROCESSOR, PreferenceIndex)

#define NDIS_RSS_PROCESSOR_INFO_REVISION_1      1

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_RSS_PROCESSOR_INFO_REVISION_2      2
typedef enum _NDIS_RSS_PROFILE
{
    NdisRssProfileClosest = 1,
    NdisRssProfileClosestStatic,
    NdisRssProfileNuma,
    NdisRssProfileNumaStatic,
    NdisRssProfileConservative,

#if (NDIS_SUPPORT_NDIS688)
    NdisRssProfileBalanced,
#endif // (NDIS_SUPPORT_NDIS688)

    NdisRssProfileMaximum,
} NDIS_RSS_PROFILE, *PNDIS_RSS_PROFILE;
#endif // (NDIS_SUPPORT_NDIS630)


//
// Used in NdisGetRssProcessorInformation API and OID_IMPLAT_RSS_PROCESSOR_INFO
//
typedef struct _NDIS_RSS_PROCESSOR_INFO
{
    NDIS_OBJECT_HEADER      Header;
    ULONG                   Flags;
    PROCESSOR_NUMBER        RssBaseProcessor;
    ULONG                   MaxNumRssProcessors;
    USHORT                  PreferredNumaNode;

    ULONG                   RssProcessorArrayOffset;// |
    ULONG                   RssProcessorCount;      // | Array of NDIS_RSS_PROCESSOR
    ULONG                   RssProcessorEntrySize;  // |

#if (NDIS_SUPPORT_NDIS630)
    PROCESSOR_NUMBER        RssMaxProcessor;
    NDIS_RSS_PROFILE        RssProfile;
#endif // (NDIS_SUPPORT_NDIS630)

} NDIS_RSS_PROCESSOR_INFO, *PNDIS_RSS_PROCESSOR_INFO;

C_ASSERT(sizeof(NDIS_RSS_PROCESSOR_INFO) % __alignof(NDIS_RSS_PROCESSOR) == 0);

#define NDIS_SIZEOF_RSS_PROCESSOR_INFO_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RSS_PROCESSOR_INFO, RssProcessorEntrySize)

#if (NDIS_SUPPORT_NDIS630)
#define NDIS_SIZEOF_RSS_PROCESSOR_INFO_REVISION_2    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RSS_PROCESSOR_INFO, RssProfile)
#endif // (NDIS_SUPPORT_NDIS630)

//
// Used in ProcessorInfo array in NDIS_SYSTEM_PROCESSOR_INFO_EX
//
typedef struct _NDIS_PROCESSOR_INFO_EX
{
    PROCESSOR_NUMBER ProcNum;
    ULONG            SocketId;
    ULONG            CoreId;
    ULONG            HyperThreadId;
    USHORT           NodeId;
    USHORT           NodeDistance;
}NDIS_PROCESSOR_INFO_EX, *PNDIS_PROCESSOR_INFO_EX;

#define NDIS_SYSTEM_PROCESSOR_INFO_EX_REVISION_1 1

//
// Used in NdisGetProcessorInformationEx API
//
typedef struct _NDIS_SYSTEM_PROCESSOR_INFO_EX
{
    NDIS_OBJECT_HEADER      Header;
    ULONG                   Flags;
    NDIS_PROCESSOR_VENDOR   ProcessorVendor;
    ULONG                   NumSockets;
    ULONG                   NumCores;
    ULONG                   NumCoresPerSocket;
    ULONG                   MaxHyperThreadingProcsPerCore;

    ULONG                   ProcessorInfoOffset;      // |
    ULONG                   NumberOfProcessors;       // | Array of NDIS_PROCESSOR_INFO_EX
    ULONG                   ProcessorInfoEntrySize;   // |

} NDIS_SYSTEM_PROCESSOR_INFO_EX, *PNDIS_SYSTEM_PROCESSOR_INFO_EX;

C_ASSERT(sizeof(NDIS_SYSTEM_PROCESSOR_INFO_EX) % __alignof(NDIS_PROCESSOR_INFO_EX) == 0);

#define NDIS_SIZEOF_SYSTEM_PROCESSOR_INFO_EX_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SYSTEM_PROCESSOR_INFO_EX, ProcessorInfoEntrySize)

//
// Enum to identify Hypervisor Parition type
// Used in NDIS_HYPERVISOR_INFO.PartitionType
//
typedef enum _NDIS_HYPERVISOR_PARTITION_TYPE
{
    NdisHypervisorPartitionTypeUnknown,
    NdisHypervisorPartitionTypeMsHvParent,
    NdisHypervisorPartitionMsHvChild,
    NdisHypervisorPartitionTypeMax
} NDIS_HYPERVISOR_PARTITION_TYPE, *PNDIS_HYPERVISOR_PARTITION_TYPE;

//
// Flags used in NDIS_HYPERVISOR_INFO.Flags field
//
#define NDIS_HYPERVISOR_INFO_FLAG_HYPERVISOR_PRESENT            0x00000001

#define NDIS_HYPERVISOR_INFO_REVISION_1 1

//
// Structure used to return Hypervisor related information
// in NdisGetHypervisorInfo API
//
typedef struct _NDIS_HYPERVISOR_INFO
{
    NDIS_OBJECT_HEADER                  Header;
    ULONG                               Flags;
    NDIS_HYPERVISOR_PARTITION_TYPE      PartitionType;
} NDIS_HYPERVISOR_INFO, *PNDIS_HYPERVISOR_INFO;

#define NDIS_SIZEOF_HYPERVISOR_INFO_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_HYPERVISOR_INFO, PartitionType)

//
// The following data structures are used with Receive Queue related WMI
// guids. NDIS will translate these data structures to those used
// in OIDs
//

typedef struct _NDIS_WMI_GROUP_AFFINITY {
    ULONG64             Mask;
    USHORT              Group;
    USHORT              Reserved[3];
} NDIS_WMI_GROUP_AFFINITY, *PNDIS_WMI_GROUP_AFFINITY;

//
// NDIS_WMI_RECEIVE_QUEUE_PARAMETERS is used in GUID_NDIS_RECEIVE_FILTER_QUEUE_PARAMETERS
//
#define NDIS_WMI_RECEIVE_QUEUE_PARAMETERS_REVISION_1       1

typedef struct _NDIS_WMI_RECEIVE_QUEUE_PARAMETERS
{
    NDIS_OBJECT_HEADER               Header;
    ULONG                            Flags;
    NDIS_RECEIVE_QUEUE_TYPE          QueueType;
    NDIS_RECEIVE_QUEUE_ID            QueueId;
    NDIS_RECEIVE_QUEUE_GROUP_ID      QueueGroupId;
    NDIS_WMI_GROUP_AFFINITY          ProcessorAffinity;
    ULONG                            NumSuggestedReceiveBuffers;
    ULONG                            MSIXTableEntry;
    ULONG                            LookaheadSize;
    NDIS_VM_NAME                     VmName;
    NDIS_QUEUE_NAME                  QueueName;
}NDIS_WMI_RECEIVE_QUEUE_PARAMETERS, *PNDIS_WMI_RECEIVE_QUEUE_PARAMETERS;

#define NDIS_SIZEOF_WMI_RECEIVE_QUEUE_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_WMI_RECEIVE_QUEUE_PARAMETERS, QueueName)

//
// NDIS_WMI_RECEIVE_QUEUE_INFO is used in GUID_NDIS_RECEIVE_FILTER_ENUM_QUEUES
//
#define NDIS_WMI_RECEIVE_QUEUE_INFO_REVISION_1              1

typedef struct _NDIS_WMI_RECEIVE_QUEUE_INFO
{
    NDIS_OBJECT_HEADER                  Header;
    ULONG                               Flags;
    NDIS_RECEIVE_QUEUE_TYPE             QueueType;
    NDIS_RECEIVE_QUEUE_ID               QueueId;
    NDIS_RECEIVE_QUEUE_GROUP_ID         QueueGroupId;
    NDIS_RECEIVE_QUEUE_OPERATIONAL_STATE QueueState;
    NDIS_WMI_GROUP_AFFINITY             ProcessorAffinity;
    ULONG                               NumSuggestedReceiveBuffers;
    ULONG                               MSIXTableEntry;
    ULONG                               LookaheadSize;
    NDIS_VM_NAME                        VmName;
    NDIS_QUEUE_NAME                     QueueName;
}NDIS_WMI_RECEIVE_QUEUE_INFO, *PNDIS_WMI_RECEIVE_QUEUE_INFO;

#define NDIS_SIZEOF_WMI_RECEIVE_QUEUE_INFO_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_WMI_RECEIVE_QUEUE_INFO, QueueName)

#endif // NDIS_SUPPORT_NDIS620

#if (NDIS_SUPPORT_NDIS630)

#include <ndkinfo.h>

//
// NetworkDirect performance counters structure for PCW support. The fields
// (and their order) in this structure must always match the corresponding
// information in the NDIS PCW manifest.
// If this structure is changed, then the NDIS_NDK_STATISTICS_INFO structure
// revision must also be updated accordingly since this structure is embedded in
// NDIS_NDK_STATISTICS_INFO.
//

typedef struct _NDIS_NDK_PERFORMANCE_COUNTERS {

    // Connection statistics
    ULONG64 Connect;          // 0
    ULONG64 Accept;           // 1
    ULONG64 ConnectFailure;   // 2
    ULONG64 ConnectionError;  // 3
    ULONG64 ActiveConnection; // 4

    // Reserved counters
    ULONG64 Reserved01; // 5
    ULONG64 Reserved02; // 6
    ULONG64 Reserved03; // 7
    ULONG64 Reserved04; // 8
    ULONG64 Reserved05; // 9
    ULONG64 Reserved06; // 10
    ULONG64 Reserved07; // 11
    ULONG64 Reserved08; // 12
    ULONG64 Reserved09; // 13
    ULONG64 Reserved10; // 14
    ULONG64 Reserved11; // 15
    ULONG64 Reserved12; // 16
    ULONG64 Reserved13; // 17
    ULONG64 Reserved14; // 18
    ULONG64 Reserved15; // 19
    ULONG64 Reserved16; // 20
    ULONG64 Reserved17; // 21
    ULONG64 Reserved18; // 22
    ULONG64 Reserved19; // 23
    ULONG64 Reserved20; // 24

    // Errors
    ULONG64 CQError;    // 25

    // Byte and frame counts
    ULONG64 RDMAInOctets;  // 26
    ULONG64 RDMAOutOctets; // 27
    ULONG64 RDMAInFrames;  // 28
    ULONG64 RDMAOutFrames; // 29
} NDIS_NDK_PERFORMANCE_COUNTERS, *PNDIS_NDK_PERFORMANCE_COUNTERS;

#define NDIS_NDK_PERFORMANCE_COUNTER_MASK(CounterFieldName) \
    (1ui64 << (FIELD_OFFSET(NDIS_NDK_PERFORMANCE_COUNTERS, CounterFieldName) / sizeof(ULONG64)))

#define NDIS_NDK_CAPABILITIES_REVISION_1 1

typedef struct _NDIS_NDK_CAPABILITIES {

    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_NDK_CAPABILITIES_REVISION_1;
    // Header.Size = sizeof(NDIS_NDK_CAPABILITIES);
    //
    NDIS_OBJECT_HEADER Header;

    ULONG Flags; // reserved, must be set to 0

    //
    // Following fields denote NDK adapter capacity limits that are relevant for
    // system-wide management/partitioning of resouces available on a given NDK adapter.
    //

    // Maximum number of queue pairs supported by the adapter
    ULONG MaxQpCount;

    // Maximum number of completion queues supported by the adapter
    ULONG MaxCqCount;

    // Maximum number of memory regions supported by the adapter
    ULONG MaxMrCount;

    // Maximum number of protection domains supported by the adapter
    ULONG MaxPdCount;

    // Maximum number of incoming outstanding read requests supported by the adapter
    // If this field is 0, there's no adapter-wide limit. There is still a limit per queue pair,
    // which is indicated by the NdkInfo.MaxInboundReadLimit field.
    ULONG MaxInboundReadLimit;

    // Maximum number of outgoing outstanding read requests supported by the adapter
    // If this field is 0, there's no adapter-wide limit. There is still a limit per queue pair,
    // which is indicated by the NdkInfo.MaxOutboundReadLimit field.
    ULONG MaxOutboundReadLimit;

    // Maximum number of memory windows supported by the adapter
    ULONG MaxMwCount;

    // Maximum number of shared receive queues supported by the adapter
    // If this field is 0, adapter does not support shared receive queues.
    ULONG MaxSrqCount;

    // ND providers are required to support all the performance counters included
    // in NDIS_NDK_PERFORMANCE_COUNTERS. However, in the rare case that
    // a provider cannot support a counter due to extreme implementation difficulties,
    // it must indicate so via the following field by OR'ing the mask value for the
    // counter which it cannot support.
    ULONG64 MissingCounterMask;

    //
    // Following field points to a structure that denotes NDK adapter capabilities and
    // limits that are relevant for an individual NDKPI client (in contrast to system-wide limits).
    //
    NDK_ADAPTER_INFO *NdkInfo;

} NDIS_NDK_CAPABILITIES, * PNDIS_NDK_CAPABILITIES;

#define NDIS_SIZEOF_NDK_CAPABILITIES_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_NDK_CAPABILITIES, NdkInfo)

#define OID_NDK_SET_STATE                0xFC040201
#define OID_NDK_STATISTICS               0xFC040202
#define OID_NDK_CONNECTIONS              0xFC040203
#define OID_NDK_LOCAL_ENDPOINTS          0xFC040204

// Embedded output structure for GUID_NDIS_NDK_CAPABILITIES
typedef struct _NDK_WMI_ADAPTER_INFO {
    NDK_VERSION Version;
    UINT32 VendorId;
    UINT32 DeviceId;
    ULONGLONG MaxRegistrationSize;
    ULONGLONG MaxWindowSize;
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
#if (NDIS_SUPPORT_NDIS650)
    NDK_RDMA_TECHNOLOGY RdmaTechnology;
#endif
} NDK_WMI_ADAPTER_INFO, *PNDK_WMI_ADAPTER_INFO;

#if (NDIS_SUPPORT_NDIS650)
#define NDIS_SIZEOF_NDK_WMI_ADAPTER_INFO_REVISION_1    \
        FIELD_OFFSET(NDK_WMI_ADAPTER_INFO, RdmaTechnology)

#define NDIS_SIZEOF_NDK_WMI_ADAPTER_INFO_REVISION_2    \
        sizeof(NDK_WMI_ADAPTER_INFO)
#endif

// Output structure for GUID_NDIS_NDK_CAPABILITIES
typedef struct _NDIS_WMI_NDK_CAPABILITIES {
    ULONG MaxQpCount;
    ULONG MaxCqCount;
    ULONG MaxMrCount;
    ULONG MaxPdCount;
    ULONG MaxInboundReadLimit;
    ULONG MaxOutboundReadLimit;
    ULONG MaxMwCount;
    ULONG MaxSrqCount;
    ULONG64 MissingCounterMask;
    NDK_WMI_ADAPTER_INFO NdkInfo;
} NDIS_WMI_NDK_CAPABILITIES, * PNDIS_WMI_NDK_CAPABILITIES;

#if (NDIS_SUPPORT_NDIS650)
#define NDIS_SIZEOF_NDK_WMI_CAPABILITIES_REVISION_1    \
        (FIELD_OFFSET(NDIS_WMI_NDK_CAPABILITIES, NdkInfo) + \
         NDIS_SIZEOF_NDK_WMI_ADAPTER_INFO_REVISION_1)

#define NDIS_SIZEOF_NDK_WMI_CAPABILITIES_REVISION_2    \
        (FIELD_OFFSET(NDIS_WMI_NDK_CAPABILITIES, NdkInfo) + \
         NDIS_SIZEOF_NDK_WMI_ADAPTER_INFO_REVISION_2)
#endif

// Query ND capabilities of an NDK adapter
DEFINE_GUID(GUID_NDIS_NDK_CAPABILITIES, 0x7969ba4d, 0xdd80, 0x4bc7, 0xb3, 0xe6, 0x68, 0x04, 0x39, 0x97, 0xe5, 0x19);

// Query/set ND enabled/disabled state of an NDK adapter
DEFINE_GUID(GUID_NDIS_NDK_STATE, 0x530c69c9, 0x2f51, 0x49de, 0xa1, 0xaf, 0x08, 0x8d, 0x54, 0xff, 0xa4, 0x74);

//
// Structure returned for an OID_NDK_STATISTICS request
//

#define NDIS_NDK_STATISTICS_INFO_REVISION_1 1

typedef struct _NDIS_NDK_STATISTICS_INFO {
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT
    // Header.Revision = NDIS_NDK_STATISTICS_INFO_REVISION_1;
    // Header.Size = NDIS_SIZEOF_NDK_STATISTICS_INFO_REVISION_1;
    //
    NDIS_OBJECT_HEADER Header;
    ULONG Flags; // reserved, must be set to 0

    NDIS_NDK_PERFORMANCE_COUNTERS CounterSet;
} NDIS_NDK_STATISTICS_INFO;

#define NDIS_SIZEOF_NDK_STATISTICS_INFO_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_NDK_STATISTICS_INFO, CounterSet)

#include <ws2def.h>
#include <ws2ipdef.h>

typedef struct _NDIS_NDK_CONNECTION_ENTRY {
    SOCKADDR_INET Local; // Local IP address and port
    SOCKADDR_INET Remote; // Remote IP address and port
    BOOLEAN UserModeOwner; // TRUE if NDSPI (user-mode) connection, FALSE if NDKPI (kernel-mode)
    ULONG OwnerPid; // Process ID for NDSPI (user-mode) connection
} NDIS_NDK_CONNECTION_ENTRY;

//
// Structure returned for an OID_NDK_CONNECTIONS request.
// This structure is variable-sized based on the actual number of connections being returned.
// The actual size of the connection array (as element count) is indicated by the Count field.
// If the RDMA technology for the NDK provider requires the provider to map ND connections
// to TCP connections, then the NDK provider must also report the underlying TCP connection
// 4-tuple for each ND connection as follows: set the NDConnectionsMappedtoTCPConnections
// field to TRUE, and use two consecutive entries for each ND connection where the first entry
// contains the ND addressing information (local & remote IP address/ND port number) and
// the immediate next entry contains the corresponding TCP connection's addressing
// information (local & remote IP address/TCP port number). Fields other than addressing
// information (UserModeOwner, OwnerPid) must be set only in the first entry, and left
// untouched in the next entry. So, entries at index 0, 2, 4, ... will contain ND addressing info (plus
// properly filled-in other fields) and entries at index 1, 3, 5, ... will contain TCP addressing info
// (where other fields are left untouched) when NDConnectionsMappedtoTCPConnections is set to TRUE.
//

#define NDIS_NDK_CONNECTIONS_REVISION_1 1

typedef struct _NDIS_NDK_CONNECTIONS {
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_NDK_CONNECTIONS_REVISION_1;
    // Header.Size = (USHORT)min(MAXUSHORT, NDIS_SIZEOF_NDK_CONNECTIONS_REVISION_1(n));
    //
    NDIS_OBJECT_HEADER Header;
    ULONG Flags; // reserved, must be set to 0
    ULONG Count;
    BOOLEAN NDConnectionsMappedtoTCPConnections;
    _Field_size_(Count) NDIS_NDK_CONNECTION_ENTRY Connections[1];
} NDIS_NDK_CONNECTIONS;

#define NDIS_SIZEOF_NDK_CONNECTIONS_REVISION_1(n)    \
        FIELD_OFFSET(NDIS_NDK_CONNECTIONS, Connections[n])

typedef struct _NDIS_NDK_LOCAL_ENDPOINT_ENTRY {
    SOCKADDR_INET Local; // Local IP address and port
    BOOLEAN UserModeOwner; // TRUE if NDSPI (user-mode) listener, FALSE if NDKPI (kernel-mode)
    BOOLEAN Listener; // TRUE if a listener, FALSE if a SharedEndpoint (used for connections)
    ULONG OwnerPid; // Process ID for NDSPI (user-mode) local endpoint
} NDIS_NDK_LOCAL_ENDPOINT_ENTRY;

//
// Structure returned for an OID_NDK_LOCAL_ENDPOINTS request.
// This structure is variable-sized based on the actual number of local endpoints being returned.
// The actual size of the local endpoint array (as element count) is indicated by the Count field.
// If the RDMA technology for the NDK provider requires the provider to map ND local endpoints
// to TCP local endpoints, then the NDK provider must also report the underlying TCP local endpoint
// for each ND local endpoint as follows: set the NDLocalEndpointsMappedtoTCPLocalEndpoints
// field to TRUE, and use two consecutive entries for each ND local endpoint where the first entry
// contains the ND addressing information (local IP address/ND port number) and
// the immediate next entry contains the corresponding TCP local endpoint's addressing
// information (local IP address/TCP port number). Fields other than addressing
// information (UserModeOwner, Listener, OwnerPid) must be set only in the first entry, and left
// untouched in the next entry. So, entries at index 0, 2, 4, ... will contain ND addressing info (plus
// properly filled-in other fields) and entries at index 1, 3, 5, ... will contain TCP addressing info
// (where other fields are left untouched) when NDLocalEndpointsMappedtoTCPLocalEndpoints is set to TRUE.
//

#define NDIS_NDK_LOCAL_ENDPOINTS_REVISION_1 1

typedef struct _NDIS_NDK_LOCAL_ENDPOINTS {
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_NDK_LOCAL_ENDPOINTS_REVISION_1;
    // Header.Size = (USHORT)min(MAXUSHORT, NDIS_SIZEOF_NDK_LOCAL_ENDPOINTS_REVISION_1(n));
    //
    NDIS_OBJECT_HEADER Header;
    ULONG Flags; // reserved, must be set to 0
    ULONG Count;
    BOOLEAN NDLocalEndpointsMappedtoTCPLocalEndpoints;
    _Field_size_(Count) NDIS_NDK_LOCAL_ENDPOINT_ENTRY LocalEndpoints[1];
} NDIS_NDK_LOCAL_ENDPOINTS;

#define NDIS_SIZEOF_NDK_LOCAL_ENDPOINTS_REVISION_1(n)    \
        FIELD_OFFSET(NDIS_NDK_LOCAL_ENDPOINTS, LocalEndpoints[n])


//
// Quality of Service (QoS) OIDs
//
#define OID_QOS_HARDWARE_CAPABILITIES           0xFC050001  // query only
#define OID_QOS_CURRENT_CAPABILITIES            0xFC050002  // query only
#define OID_QOS_PARAMETERS                      0xFC050003  // method only
#define OID_QOS_OPERATIONAL_PARAMETERS          0xFC050004  // query only
#define OID_QOS_REMOTE_PARAMETERS               0xFC050005  // query only


#define NDIS_QOS_MAXIMUM_PRIORITIES         8
#define NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES    8

//
// The following bits are used in NDIS_QOS_CAPABILITIES.Flags field.
//
#define NDIS_QOS_CAPABILITIES_STRICT_TSA_SUPPORTED      0x00000001
#define NDIS_QOS_CAPABILITIES_MACSEC_BYPASS_SUPPORTED   0x00000002
#define NDIS_QOS_CAPABILITIES_CEE_DCBX_SUPPORTED        0x00000004
#define NDIS_QOS_CAPABILITIES_IEEE_DCBX_SUPPORTED       0x00000008

#define NDIS_QOS_CAPABILITIES_REVISION_1    1

//
// NDIS_QOS_CAPABILITIES is used as the return result of the OID_QOS_CURRENT_CAPABILITIES
// and OID_QOS_HARDWARE_CAPABILITIES OID queries. It is also specified as the
// HardwareQosCapabilities and CurrentQosCapabilities fields in the
// NDIS_MINIPORT_ADAPTER_HARDWARE_ASSIST_ATTRIBUTES structure.
//
typedef _Struct_size_bytes_(Header.Size) struct _NDIS_QOS_CAPABILITIES
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_QOS_CAPABILITIES
    // Header.Revision = NDIS_QOS_CAPABILITIES_REVISION_1
    // Header.Size = NDIS_SIZEOF_QOS_CAPABILITIES_REVISION_1
    //
    _In_ NDIS_OBJECT_HEADER Header;
    _In_ ULONG              Flags;

    _In_ ULONG              MaxNumTrafficClasses;
    _In_ _Field_range_(<=, MaxNumTrafficClasses)
         ULONG              MaxNumEtsCapableTrafficClasses;
    _In_ _Field_range_(<=, MaxNumTrafficClasses)
         ULONG              MaxNumPfcEnabledTrafficClasses;
} NDIS_QOS_CAPABILITIES, *PNDIS_QOS_CAPABILITIES;

#define NDIS_SIZEOF_QOS_CAPABILITIES_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_QOS_CAPABILITIES, MaxNumPfcEnabledTrafficClasses)

//
// The following bits are used in NDIS_QOS_CLASSIFICATION_ELEMENT.Flags field.
//
#define NDIS_QOS_CLASSIFICATION_SET_BY_MINIPORT_MASK    0xFF000000  // Mask of flags set by miniport only
#define NDIS_QOS_CLASSIFICATION_ENFORCED_BY_MINIPORT    0x01000000  // Set by miniport

//
// The following values are used in
// NDIS_QOS_CLASSIFICATION_ELEMENT.ConditionSelector field.
//
#define NDIS_QOS_CONDITION_RESERVED         0x0
#define NDIS_QOS_CONDITION_DEFAULT          0x1
#define NDIS_QOS_CONDITION_TCP_PORT         0x2
#define NDIS_QOS_CONDITION_UDP_PORT         0x3
#define NDIS_QOS_CONDITION_TCP_OR_UDP_PORT  0x4
#define NDIS_QOS_CONDITION_ETHERTYPE        0x5
#define NDIS_QOS_CONDITION_NETDIRECT_PORT   0x6
#define NDIS_QOS_CONDITION_MAXIMUM          0x7

//
// The following values are used in
// NDIS_QOS_CLASSIFICATION_ELEMENT.ActionSelector field.
//
#define NDIS_QOS_ACTION_PRIORITY            0x0     // 802.1p priority tagging
#define NDIS_QOS_ACTION_MAXIMUM             0x1

#define NDIS_QOS_CLASSIFICATION_ELEMENT_REVISION_1   1

//
// NDIS_QOS_CLASSIFICATION_ELEMENT elements are specified by
// NDIS_QOS_PARAMETERS.FirstClassificationElementOffset.
//
typedef _Struct_size_bytes_(Header.Size) struct _NDIS_QOS_CLASSIFICATION_ELEMENT
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_QOS_CLASSIFICATION_ELEMENT
    // Header.Revision = NDIS_QOS_CLASSIFICATION_ELEMENT_REVISION_1
    // Header.Size = NDIS_SIZEOF_QOS_CLASSIFICATION_ELEMENT_REVISION_1
    //
    _In_    NDIS_OBJECT_HEADER  Header;
    _Inout_ ULONG               Flags;

    _In_ _Field_range_(<, NDIS_QOS_CONDITION_MAXIMUM)
            USHORT              ConditionSelector;
    _In_    USHORT              ConditionField;

    _In_ _Field_range_(<, NDIS_QOS_ACTION_MAXIMUM)
            USHORT              ActionSelector;
    _In_    USHORT              ActionField;
} NDIS_QOS_CLASSIFICATION_ELEMENT, *PNDIS_QOS_CLASSIFICATION_ELEMENT;

#define NDIS_SIZEOF_QOS_CLASSIFICATION_ELEMENT_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_QOS_CLASSIFICATION_ELEMENT, ActionField)


//
// The following bits are used in NDIS_QOS_PARAMETERS.Flags field.
//
#define NDIS_QOS_PARAMETERS_ETS_CHANGED                 0x00000001  // Used in OID_QOS_PARAMETERS,
                                                                    // optional in NDIS_STATUS_QOS_XXX_PARAMETERS_CHANGE
#define NDIS_QOS_PARAMETERS_ETS_CONFIGURED              0x00000002
#define NDIS_QOS_PARAMETERS_PFC_CHANGED                 0x00000100  // Used in OID_QOS_PARAMETERS,
                                                                    // optional in NDIS_STATUS_QOS_XXX_PARAMETERS_CHANGE
#define NDIS_QOS_PARAMETERS_PFC_CONFIGURED              0x00000200
#define NDIS_QOS_PARAMETERS_CLASSIFICATION_CHANGED      0x00010000  // Used in OID_QOS_PARAMETERS,
                                                                    // optional in NDIS_STATUS_QOS_XXX_PARAMETERS_CHANGE
#define NDIS_QOS_PARAMETERS_CLASSIFICATION_CONFIGURED   0x00020000
#define NDIS_QOS_PARAMETERS_WILLING                     0x80000000  // Used in OID_QOS_PARAMETERS

//
// The following values are used in NDIS_QOS_PARAMETERS.TsaAssignmentTable field.
//
#define NDIS_QOS_TSA_STRICT                 0x0     // Strict priority
#define NDIS_QOS_TSA_CBS                    0x1     // Credit-Based Shaper
#define NDIS_QOS_TSA_ETS                    0x2     // Enhanced Transmission Selection
#define NDIS_QOS_TSA_MAXIMUM                0x3

#define NDIS_QOS_PARAMETERS_REVISION_1   1

//
// NDIS_QOS_PARAMETERS is used by OID_QOS_PARAMETERS.
//
typedef _Struct_size_bytes_(Header.Size) struct _NDIS_QOS_PARAMETERS
{
    //
    // Header.Type = NDIS_OBJECT_TYPE_QOS_PARAMETERS
    // Header.Revision = NDIS_QOS_PARAMETERS_REVISION_1
    // Header.Size = NDIS_SIZEOF_QOS_PARAMETERS_REVISION_1 + sizeof_all_classification_entries
    //
    _In_ NDIS_OBJECT_HEADER Header;
    _In_ ULONG              Flags;

    //
    // The following fields are to configure ETS.
    //
    _In_ _Field_range_(0, NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES)
         ULONG              NumTrafficClasses;
    _In_ UCHAR              PriorityAssignmentTable[NDIS_QOS_MAXIMUM_PRIORITIES];
    _In_ UCHAR              TcBandwidthAssignmentTable[NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES];
    _In_ UCHAR              TsaAssignmentTable[NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES];

    //
    // The following fields are to configure PFC.
    //
    _In_ _Field_range_(0, (1ul << NDIS_QOS_MAXIMUM_PRIORITIES) - 1)
         ULONG              PfcEnable;

    //
    // The following fields are to configure the packet classification and tagging
    // mechanism.
    //
    // Each element in the array specified by FirstClassificationElementOffset
    // is of type NDIS_QOS_CLASSIFICATION_ELEMENT. Each element occupies exactly
    // ClassificationElementSize bytes in the array.
    //
    _In_ ULONG              NumClassificationElements;
    _In_ ULONG              ClassificationElementSize;
    _In_ ULONG              FirstClassificationElementOffset;
} NDIS_QOS_PARAMETERS, *PNDIS_QOS_PARAMETERS;

#define NDIS_SIZEOF_QOS_PARAMETERS_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_QOS_PARAMETERS, FirstClassificationElementOffset)


typedef NDIS_IF_COUNTED_STRING NDIS_NIC_SWITCH_FRIENDLYNAME, *PNDIS_NIC_SWITCH_FRIENDLYNAME;
typedef NDIS_IF_COUNTED_STRING NDIS_VPORT_NAME, *PNDIS_VPORT_NAME;
typedef USHORT NDIS_SRIOV_FUNCTION_ID, *PNDIS_SRIOV_FUNCTION_ID;
typedef ULONG NDIS_VF_RID, *PNDIS_VF_RID;

#define NDIS_PF_FUNCTION_ID                 (USHORT) -1
#define NDIS_INVALID_VF_FUNCTION_ID         (USHORT) -1
#define NDIS_INVALID_RID                    (ULONG) -1
#define NDIS_DEFAULT_VPORT_ID               0
#define NDIS_INVALID_VPORT_ID               (ULONG) -1

//
// This is the default switch identifier. This is the only valid
// switch identifier in Windows 8
//
#define NDIS_DEFAULT_SWITCH_ID              0
#define NDIS_INVALID_SWITCH_ID              (ULONG) -1

//
// Enum to specify the type of NIC embedded switch
// Used in SwitchType fields of NDIS_NIC_SWITCH_PARAMETERS and
// NDIS_NIC_SWITCH_PARAMETERS structures
//
typedef enum _NDIS_NIC_SWITCH_TYPE
{
    NdisNicSwitchTypeUnspecified,
    NdisNicSwitchTypeExternal,
    NdisNicSwitchTypeMax
} NDIS_NIC_SWITCH_TYPE, *PNDIS_NIC_SWITCH_TYPE;

//
// The following flags are used in NDIS_NIC_SWITCH_PARAMETERS.Flags field
//
#define NDIS_NIC_SWITCH_PARAMETERS_CHANGE_MASK                                                               0xFFFF0000
#define NDIS_NIC_SWITCH_PARAMETERS_SWITCH_NAME_CHANGED                                                       0x00010000


//
// The following value must be used for
// NDIS_NIC_SWITCH_PARAMETERS.NumQueuePairsForDefaultVPort field when HW vRSS
// is not availabile or not used.
//
#define NDIS_NIC_SWITCH_PARAMETERS_DEFAULT_NUMBER_OF_QUEUE_PAIRS_FOR_DEFAULT_VPORT 1

//
// This structure is used in OID_NIC_SWITCH_CREATE_SWITCH and OID_NIC_SWITCH_PARAMETERS OIDs
//
#define NDIS_NIC_SWITCH_PARAMETERS_REVISION_1       1

#if (NDIS_SUPPORT_NDIS660)
#define NDIS_NIC_SWITCH_PARAMETERS_REVISION_2       2
#endif // (NDIS_SUPPORT_NDIS660)

typedef struct _NDIS_NIC_SWITCH_PARAMETERS
{
    _In_        NDIS_OBJECT_HEADER              Header;
    _In_        ULONG                           Flags;
    _In_        NDIS_NIC_SWITCH_TYPE            SwitchType;
    _In_        NDIS_NIC_SWITCH_ID              SwitchId;
    _In_        NDIS_NIC_SWITCH_FRIENDLYNAME    SwitchFriendlyName;
    _In_        ULONG                           NumVFs;
    _In_        ULONG                           NdisReserved1;
    _In_        ULONG                           NdisReserved2;
    _In_        ULONG                           NdisReserved3;
#if (NDIS_SUPPORT_NDIS660)
    _In_        ULONG                           NumQueuePairsForDefaultVPort;
#endif // (NDIS_SUPPORT_NDIS660)
} NDIS_NIC_SWITCH_PARAMETERS, *PNDIS_NIC_SWITCH_PARAMETERS;

#define NDIS_SIZEOF_NIC_SWITCH_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_PARAMETERS, NdisReserved3)

#if (NDIS_SUPPORT_NDIS660)
#define NDIS_SIZEOF_NIC_SWITCH_PARAMETERS_REVISION_2     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_PARAMETERS, NumQueuePairsForDefaultVPort)
#endif // (NDIS_SUPPORT_NDIS660)


//
// This structure is used in OID_NIC_SWITCH_DELETE_SWITCH OID
//
#define NDIS_NIC_SWITCH_DELETE_SWITCH_PARAMETERS_REVISION_1       1

typedef struct _NDIS_NIC_SWITCH_DELETE_SWITCH_PARAMETERS
{
    _In_        NDIS_OBJECT_HEADER          Header;
    _In_        ULONG                       Flags;
    _In_        NDIS_NIC_SWITCH_ID          SwitchId;
} NDIS_NIC_SWITCH_DELETE_SWITCH_PARAMETERS, *PNDIS_NIC_SWITCH_DELETE_SWITCH_PARAMETERS ;

#define NDIS_SIZEOF_NIC_SWITCH_DELETE_SWITCH_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_DELETE_SWITCH_PARAMETERS, SwitchId)

//
// This structure is used for each array element in
// NDIS_NIC_SWITCH_INFO_ARRAY structure
//
#define NDIS_NIC_SWITCH_INFO_REVISION_1              1

typedef struct _NDIS_NIC_SWITCH_INFO
{
    _In_        NDIS_OBJECT_HEADER              Header;
    _In_        ULONG                           Flags;
    _In_        NDIS_NIC_SWITCH_TYPE            SwitchType;
    _In_        NDIS_NIC_SWITCH_ID              SwitchId;
    _In_        NDIS_NIC_SWITCH_FRIENDLYNAME    SwitchFriendlyName;
    _In_        ULONG                           NumVFs;
    _In_        ULONG                           NumAllocatedVFs;
    _In_        ULONG                           NumVPorts;
    _In_        ULONG                           NumActiveVPorts;
    _In_        ULONG                           NumQueuePairsForDefaultVPort;
    _In_        ULONG                           NumQueuePairsForNonDefaultVPorts;
    _In_        ULONG                           NumActiveDefaultVPortMacAddresses;
    _In_        ULONG                           NumActiveNonDefaultVPortMacAddresses;
    _In_        ULONG                           NumActiveDefaultVPortVlanIds;
    _In_        ULONG                           NumActiveNonDefaultVPortVlanIds;
}NDIS_NIC_SWITCH_INFO, *PNDIS_NIC_SWITCH_INFO;

#define NDIS_SIZEOF_NIC_SWITCH_INFO_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_INFO, NumActiveNonDefaultVPortVlanIds)

//
// This structure is used in OID_NIC_SWITCH_ENUM_SWITCHES OID and in
// NicSwitches field of NDIS_BIND_PARAMETERS and NDIS_FILTER_ATTACH_PARAMETERS structures.
// Each element in the array is of type NDIS_NIC_SWITCH_INFO structure
//
#define NDIS_NIC_SWITCH_INFO_ARRAY_REVISION_1             1

typedef struct _NDIS_NIC_SWITCH_INFO_ARRAY
{
    NDIS_OBJECT_HEADER                          Header;
    ULONG                                       FirstElementOffset;
    ULONG                                       NumElements;
    ULONG                                       ElementSize;
}NDIS_NIC_SWITCH_INFO_ARRAY, *PNDIS_NIC_SWITCH_INFO_ARRAY;

#define NDIS_SIZEOF_NIC_SWITCH_INFO_ARRAY_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_INFO_ARRAY, ElementSize)

//
// The following enum type is used in VPortState field of
// NDIS_NIC_SWITCH_VPORT_INFO and NDIS_NIC_SWITCH_VPORT_PARAMETERS
//
typedef enum _NDIS_NIC_SWITCH_VPORT_STATE
{
    NdisNicSwitchVPortStateUndefined,
    NdisNicSwitchVPortStateActivated,
    NdisNicSwitchVPortStateDeactivated,
    NdisNicSwitchVPortStateMaximum
}NDIS_NIC_SWITCH_VPORT_STATE, *PNDIS_NIC_SWITCH_VPORT_STATE;

//
// The following enum type is used in InterruptModeration field of
// NDIS_NIC_SWITCH_VPORT_INFO and NDIS_NIC_SWITCH_VPORT_PARAMETERS
//
typedef enum _NDIS_NIC_SWITCH_VPORT_INTERRUPT_MODERATION
{
    NdisNicSwitchVPortInterruptModerationUndefined = 0,
    NdisNicSwitchVPortInterruptModerationAdaptive = 1,
    NdisNicSwitchVPortInterruptModerationOff = 2,
    NdisNicSwitchVPortInterruptModerationLow = 100,
    NdisNicSwitchVPortInterruptModerationMedium = 200,
    NdisNicSwitchVPortInterruptModerationHigh = 300,
}NDIS_NIC_SWITCH_VPORT_INTERRUPT_MODERATION, *PNDIS_NIC_SWITCH_VPORT_INTERRUPT_MODERATION;

#if (NDIS_SUPPORT_NDIS650)
//
// NDIS_SWITCH_PORT_ID and NDIS_SWITCH_NIC_INDEX are needed
// in user and kernel level headers without a common include,
// so it is defined in multiple places.
//
#ifndef _NDIS_SWITCH_PORT_ID
#define _NDIS_SWITCH_PORT_ID NDIS_SWITCH_PORT_ID
typedef UINT32 NDIS_SWITCH_PORT_ID, *PNDIS_SWITCH_PORT_ID;
typedef USHORT NDIS_SWITCH_NIC_INDEX, *PNDIS_SWITCH_NIC_INDEX;
#else
//
// If already defined, make sure sizes match.
//
C_ASSERT(sizeof(NDIS_SWITCH_PORT_ID) == sizeof(UINT32));
C_ASSERT(sizeof(NDIS_SWITCH_NIC_INDEX) == sizeof(USHORT));
#endif

typedef struct _NDIS_NDK_REQUEST_PARAMETERS {
  BOOLEAN RdmaRequested;
  NDIS_SWITCH_PORT_ID SwitchPortId;
  NET_IFINDEX IfIndex;
  UCHAR MacAddress[NDIS_MAX_PHYS_ADDRESS_LENGTH];
  UINT32 VlanId;
  NDIS_NDK_CAPABILITIES NdkReserved;
} NDIS_NDK_REQUEST_PARAMETERS, *PNDIS_NDK_REQUEST_PARAMETERS;

typedef struct _NDIS_NDK_RESPONSE_PARAMETERS {
  BOOLEAN CapabilitiesAvailable;
  NDIS_NDK_CAPABILITIES ReceivedCapabilities;
} NDIS_NDK_RESPONSE_PARAMETERS, *PNDIS_NDK_RESPONSE_PARAMETERS;

typedef struct _NDIS_NDK_PARAMETERS {
  NDIS_NDK_REQUEST_PARAMETERS NdkRequest;
  NDIS_NDK_RESPONSE_PARAMETERS NdkResponse;
} NDIS_NDK_PARAMETERS,*PNDIS_NDK_PARAMETERS;
#endif // NDIS_SUPPORT_NDIS650

//
// These flags are used in NDIS_NIC_SWITCH_VPORT_PARAMETERS.Flags field
//
#define NDIS_NIC_SWITCH_VPORT_PARAMS_LOOKAHEAD_SPLIT_ENABLED                            0x00000001

#if (NDIS_SUPPORT_NDIS650)
#define NDIS_NIC_SWITCH_VPORT_PARAMS_PACKET_DIRECT_RX_ONLY                              0x00000002
#endif // (NDIS_SUPPORT_NDIS650)

#define NDIS_NIC_SWITCH_VPORT_PARAMS_ENFORCE_MAX_SG_LIST                                0x00008000

#define NDIS_NIC_SWITCH_VPORT_PARAMS_CHANGE_MASK                                        0xFFFF0000
#define NDIS_NIC_SWITCH_VPORT_PARAMS_FLAGS_CHANGED                                      0x00010000
#define NDIS_NIC_SWITCH_VPORT_PARAMS_NAME_CHANGED                                       0x00020000
#define NDIS_NIC_SWITCH_VPORT_PARAMS_INT_MOD_CHANGED                                    0x00040000
#define NDIS_NIC_SWITCH_VPORT_PARAMS_STATE_CHANGED                                      0x00080000
#define NDIS_NIC_SWITCH_VPORT_PARAMS_PROCESSOR_AFFINITY_CHANGED                         0x00100000
#if (NDIS_SUPPORT_NDIS650)
#define NDIS_NIC_SWITCH_VPORT_PARAMS_NDK_PARAMS_CHANGED                                 0x00200000
#define NDIS_NIC_SWITCH_VPORT_PARAMS_QOS_SQ_ID_CHANGED                                  0x00400000
#endif // (NDIS_SUPPORT_NDIS650)
#if (NDIS_SUPPORT_NDIS660)
#define NDIS_NIC_SWITCH_VPORT_PARAMS_NUM_QUEUE_PAIRS_CHANGED                            0x00800000
#endif // (NDIS_SUPPORT_NDIS660)

//
// This structure is used in OID_NIC_SWITCH_CREATE_VPORT and
// OID_NIC_SWITCH_VPORT_PARAMETERS
//
#define NDIS_NIC_SWITCH_VPORT_PARAMETERS_REVISION_1       1
#if (NDIS_SUPPORT_NDIS650)
#define NDIS_NIC_SWITCH_VPORT_PARAMETERS_REVISION_2       2
#endif // NDIS_SUPPORT_NDIS650

typedef struct _NDIS_NIC_SWITCH_VPORT_PARAMETERS
{
    _In_        NDIS_OBJECT_HEADER          Header;
    _In_        ULONG                       Flags;
    _In_        NDIS_NIC_SWITCH_ID          SwitchId;
    _Inout_     NDIS_NIC_SWITCH_VPORT_ID    VPortId;
    _In_        NDIS_VPORT_NAME             VPortName;
    _In_        NDIS_SRIOV_FUNCTION_ID      AttachedFunctionId;
    _In_        ULONG                       NumQueuePairs;
    _In_        NDIS_NIC_SWITCH_VPORT_INTERRUPT_MODERATION    InterruptModeration;
    _In_        NDIS_NIC_SWITCH_VPORT_STATE VPortState;
    _In_        GROUP_AFFINITY              ProcessorAffinity;
    _In_        ULONG                       LookaheadSize;
#if (NDIS_SUPPORT_NDIS650)
    _In_        NDIS_NDK_PARAMETERS         NdkParams;
    _In_        NDIS_QOS_SQ_ID              QosSqId;
#endif // (NDIS_SUPPORT_NDIS650)
}NDIS_NIC_SWITCH_VPORT_PARAMETERS, *PNDIS_NIC_SWITCH_VPORT_PARAMETERS;

#define NDIS_SIZEOF_NIC_SWITCH_VPORT_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_VPORT_PARAMETERS, LookaheadSize)

#if (NDIS_SUPPORT_NDIS650)
#define NDIS_SIZEOF_NIC_SWITCH_VPORT_PARAMETERS_REVISION_2     \
   RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_VPORT_PARAMETERS, QosSqId)
#endif // (NDIS_SUPPORT_NDIS650)

//
// This structure is used in OID_NIC_SWITCH_DELETE_VPORT OID
//
#define NDIS_NIC_SWITCH_DELETE_VPORT_PARAMETERS_REVISION_1       1

typedef struct _NDIS_NIC_SWITCH_DELETE_VPORT_PARAMETERS
{
    _In_        NDIS_OBJECT_HEADER          Header;
    _In_        ULONG                       Flags;
    _In_        NDIS_NIC_SWITCH_VPORT_ID    VPortId;
} NDIS_NIC_SWITCH_DELETE_VPORT_PARAMETERS, *PNDIS_NIC_SWITCH_DELETE_VPORT_PARAMETERS ;

#define NDIS_SIZEOF_NIC_SWITCH_DELETE_VPORT_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_DELETE_VPORT_PARAMETERS, VPortId)

//
// Flags used in NDIS_NIC_SWITCH_VPORT_INFO.Flags field
//
#define NDIS_NIC_SWITCH_VPORT_INFO_LOOKAHEAD_SPLIT_ENABLED      0x00000001

#if (NDIS_SUPPORT_NDIS650)
#define NDIS_NIC_SWITCH_VPORT_INFO_PACKET_DIRECT_RX_ONLY        0x00000002
#define NDIS_NIC_SWITCH_VPORT_INFO_GFT_ENABLED                  0x00000004
#endif // (NDIS_SUPPORT_NDIS650)


//
// This structure is used in OID_NIC_SWITCH_ENUM_VPORTS OID
// Each element in the array is of type NDIS_NIC_SWITCH_VPORT_INFO structure
//
#define NDIS_NIC_SWITCH_VPORT_INFO_REVISION_1       1

typedef struct _NDIS_NIC_SWITCH_VPORT_INFO
{
    _In_        NDIS_OBJECT_HEADER              Header;
    _In_        NDIS_NIC_SWITCH_VPORT_ID        VPortId;
    _In_        ULONG                           Flags;
    _In_        NDIS_NIC_SWITCH_ID              SwitchId;
    _In_        NDIS_VPORT_NAME                 VPortName;
    _In_        NDIS_SRIOV_FUNCTION_ID          AttachedFunctionId;
    _In_        ULONG                           NumQueuePairs;
    _In_        NDIS_NIC_SWITCH_VPORT_INTERRUPT_MODERATION    InterruptModeration;
    _In_        NDIS_NIC_SWITCH_VPORT_STATE     VPortState;
    _In_        GROUP_AFFINITY                  ProcessorAffinity;
    _In_        ULONG                           LookaheadSize;
    _In_        ULONG                           NumFilters;

}NDIS_NIC_SWITCH_VPORT_INFO, *PNDIS_NIC_SWITCH_VPORT_INFO;

#define NDIS_SIZEOF_NIC_SWITCH_VPORT_INFO_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_VPORT_INFO, NumFilters)

//
// The following flags are used in NDIS_NIC_SWITCH_VPORT_INFO_ARRAY.Flags field
//
#define NDIS_NIC_SWITCH_VPORT_INFO_ARRAY_ENUM_ON_SPECIFIC_FUNCTION      0x00000001
#define NDIS_NIC_SWITCH_VPORT_INFO_ARRAY_ENUM_ON_SPECIFIC_SWITCH        0x00000002
//
// This structure is used in OID_NIC_SWITCH_ENUM_VPORTS OID
// Each element in the array is of type NDIS_NIC_SWITCH_VPORT_INFO structure
//
#define NDIS_NIC_SWITCH_VPORT_INFO_ARRAY_REVISION_1       1

typedef struct _NDIS_NIC_SWITCH_VPORT_INFO_ARRAY
{
    _In_        NDIS_OBJECT_HEADER      Header;
    _In_        ULONG                   Flags;
    _In_        NDIS_NIC_SWITCH_ID      SwitchId;
    _In_        NDIS_SRIOV_FUNCTION_ID  AttachedFunctionId;
    _In_        ULONG                   FirstElementOffset;
    _In_        ULONG                   NumElements;
    _In_        ULONG                   ElementSize;
}NDIS_NIC_SWITCH_VPORT_INFO_ARRAY, *PNDIS_NIC_SWITCH_VPORT_INFO_ARRAY;

#define NDIS_SIZEOF_NIC_SWITCH_VPORT_INFO_ARRAY_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_VPORT_INFO_ARRAY, ElementSize)

//
// This structure is used in OID_NIC_SWITCH_ALLOCATE_VF and
// OID_NIC_SWITCH_VF_PARAMETERS OIDs
//

#define NDIS_NIC_SWITCH_VF_PARAMETERS_REVISION_1       1

typedef struct _NDIS_NIC_SWITCH_VF_PARAMETERS
{
    _In_    NDIS_OBJECT_HEADER          Header;
    _In_    ULONG                       Flags;
    _In_    NDIS_NIC_SWITCH_ID          SwitchId;
    _In_    NDIS_VM_NAME                VMName;
    _In_    NDIS_VM_FRIENDLYNAME        VMFriendlyName;
    _In_    NDIS_SWITCH_NIC_NAME        NicName;
    _In_    USHORT                      MacAddressLength;
    _In_    UCHAR                       PermanentMacAddress[NDIS_MAX_PHYS_ADDRESS_LENGTH];
    _In_    UCHAR                       CurrentMacAddress[NDIS_MAX_PHYS_ADDRESS_LENGTH];
    _Inout_ NDIS_SRIOV_FUNCTION_ID      VFId;
    _Inout_ NDIS_VF_RID                 RequestorId;
} NDIS_NIC_SWITCH_VF_PARAMETERS, *PNDIS_NIC_SWITCH_VF_PARAMETERS;

#define NDIS_SIZEOF_NIC_SWITCH_VF_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_VF_PARAMETERS, RequestorId)

//
// This structure is used in OID_NIC_SWITCH_FREE_VF OID
//
#define NDIS_NIC_SWITCH_FREE_VF_PARAMETERS_REVISION_1       1

typedef struct _NDIS_NIC_SWITCH_FREE_VF_PARAMETERS
{
    _In_    NDIS_OBJECT_HEADER          Header;
    _In_    ULONG                       Flags;
    _In_    NDIS_SRIOV_FUNCTION_ID      VFId;
} NDIS_NIC_SWITCH_FREE_VF_PARAMETERS, *PNDIS_NIC_SWITCH_FREE_VF_PARAMETERS;

#define NDIS_SIZEOF_NIC_SWITCH_FREE_VF_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_FREE_VF_PARAMETERS, VFId)

//
// Thus structure is used in OID_NIC_SWITCH_ENUM_VFS OID
// Each element in the array is of type NDIS_NIC_SWITCH_VF_INFO structure
//
#define NDIS_NIC_SWITCH_VF_INFO_REVISION_1       1

typedef struct _NDIS_NIC_SWITCH_VF_INFO
{
    _In_    NDIS_OBJECT_HEADER          Header;
    _In_    ULONG                       Flags;
    _In_    NDIS_NIC_SWITCH_ID          SwitchId;
    _In_    NDIS_VM_NAME                VMName;
    _In_    NDIS_VM_FRIENDLYNAME        VMFriendlyName;
    _In_    NDIS_SWITCH_NIC_NAME        NicName;
    _In_    USHORT                      MacAddressLength;
    _In_    UCHAR                       PermanentMacAddress[NDIS_MAX_PHYS_ADDRESS_LENGTH];
    _In_    UCHAR                       CurrentMacAddress[NDIS_MAX_PHYS_ADDRESS_LENGTH];
    _In_    NDIS_SRIOV_FUNCTION_ID      VFId;
    _In_    NDIS_VF_RID                 RequestorId;
} NDIS_NIC_SWITCH_VF_INFO, *PNDIS_NIC_SWITCH_VF_INFO;

#define NDIS_SIZEOF_NIC_SWITCH_VF_INFO_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_VF_INFO, RequestorId)

//
// The following flags are used in _NDIS_NIC_SWITCH_VF_INFO_ARRAY.Flags field
//
#define NDIS_NIC_SWITCH_VF_INFO_ARRAY_ENUM_ON_SPECIFIC_SWITCH        0x00000001

//
// This structure is used in OID_NIC_SWITCH_ENUM_VFS OID
// Each element in the array is of type NDIS_NIC_SWITCH_VF_INFO structure
//
#define NDIS_NIC_SWITCH_VF_INFO_ARRAY_REVISION_1             1

typedef struct _NDIS_NIC_SWITCH_VF_INFO_ARRAY
{
    _In_    NDIS_OBJECT_HEADER              Header;
    _In_    ULONG                           Flags;
    _In_    NDIS_NIC_SWITCH_ID              SwitchId;
    _Out_   ULONG                           FirstElementOffset;
    _Out_   ULONG                           NumElements;
    _Out_   ULONG                           ElementSize;
}NDIS_NIC_SWITCH_VF_INFO_ARRAY, *PNDIS_NIC_SWITCH_VF_INFO_ARRAY;

#define NDIS_SIZEOF_NIC_SWITCH_VF_INFO_ARRAY_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_NIC_SWITCH_VF_INFO_ARRAY, ElementSize)

//
// Following flags are used in NDIS_SRIOV_CAPABILITIES.SriovCapabilities field
//
#define NDIS_SRIOV_CAPS_SRIOV_SUPPORTED         0x00000001
#define NDIS_SRIOV_CAPS_PF_MINIPORT             0x00000002
#define NDIS_SRIOV_CAPS_VF_MINIPORT             0x00000004

//
// This structure is used in OID_SRIOV_CURRENT_CAPABILITIES and OID_SRIOV_HARDWARE_CAPABILITIES.
// It is also used in the CurrentSriovCapabilities and HardwareSriovCapabilities fields of
// NDIS_MINIPORT_ADAPTER_HARDWARE_ASSIST_ATTRIBUTES structure .
// It is also used in SriovCapabilities field of NDIS_BIND_PARAMETERS and NDIS_FILTER_ATTACH_PARAMETERS
//
#define NDIS_SRIOV_CAPABILITIES_REVISION_1             1

typedef struct _NDIS_SRIOV_CAPABILITIES
{
    _In_    NDIS_OBJECT_HEADER              Header;
    _In_    ULONG                           Flags;
    _In_    ULONG                           SriovCapabilities;
} NDIS_SRIOV_CAPABILITIES, *PNDIS_SRIOV_CAPABILITIES;

#define NDIS_SIZEOF_SRIOV_CAPABILITIES_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SRIOV_CAPABILITIES, SriovCapabilities)

//
// This structure is used in OID_SRIOV_READ_VF_CONFIG_SPACE
//
#define NDIS_SRIOV_READ_VF_CONFIG_SPACE_PARAMETERS_REVISION_1             1

typedef struct _NDIS_SRIOV_READ_VF_CONFIG_SPACE_PARAMETERS
{
    _In_    NDIS_OBJECT_HEADER              Header;
    _In_    NDIS_SRIOV_FUNCTION_ID          VFId;
    _In_    ULONG                           Offset;
    _In_    ULONG                           Length;
    _In_    ULONG                           BufferOffset;
} NDIS_SRIOV_READ_VF_CONFIG_SPACE_PARAMETERS, *PNDIS_SRIOV_READ_VF_CONFIG_SPACE_PARAMETERS;

#define NDIS_SIZEOF_SRIOV_READ_VF_CONFIG_SPACE_PARAMETERS_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SRIOV_READ_VF_CONFIG_SPACE_PARAMETERS, BufferOffset)

//
// This structure is used in OID_SRIOV_WRITE_VF_CONFIG_SPACE
//
#define NDIS_SRIOV_WRITE_VF_CONFIG_SPACE_PARAMETERS_REVISION_1             1

typedef struct _NDIS_SRIOV_WRITE_VF_CONFIG_SPACE_PARAMETERS
{
    _In_    NDIS_OBJECT_HEADER              Header;
    _In_    NDIS_SRIOV_FUNCTION_ID          VFId;
    _In_    ULONG                           Offset;
    _In_    ULONG                           Length;
    _In_    ULONG                           BufferOffset;
} NDIS_SRIOV_WRITE_VF_CONFIG_SPACE_PARAMETERS, *PNDIS_SRIOV_WRITE_VF_CONFIG_SPACE_PARAMETERS;

#define NDIS_SIZEOF_SRIOV_WRITE_VF_CONFIG_SPACE_PARAMETERS_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SRIOV_WRITE_VF_CONFIG_SPACE_PARAMETERS, BufferOffset)

//
// This structure is used in OID_SRIOV_READ_VF_CONFIG_BLOCK
//
#define NDIS_SRIOV_READ_VF_CONFIG_BLOCK_PARAMETERS_REVISION_1             1

typedef struct _NDIS_SRIOV_READ_VF_CONFIG_BLOCK_PARAMETERS
{
    _In_    NDIS_OBJECT_HEADER              Header;
    _In_    NDIS_SRIOV_FUNCTION_ID          VFId;
    _In_    ULONG                           BlockId;
    _In_    ULONG                           Length;
    _In_    ULONG                           BufferOffset;
} NDIS_SRIOV_READ_VF_CONFIG_BLOCK_PARAMETERS, *PNDIS_SRIOV_READ_VF_CONFIG_BLOCK_PARAMETERS;

#define NDIS_SIZEOF_SRIOV_READ_VF_CONFIG_BLOCK_PARAMETERS_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SRIOV_READ_VF_CONFIG_BLOCK_PARAMETERS, BufferOffset)

//
// This structure is used in OID_SRIOV_WRITE_VF_CONFIG_BLOCK
//
#define NDIS_SRIOV_WRITE_VF_CONFIG_BLOCK_PARAMETERS_REVISION_1             1

typedef struct _NDIS_SRIOV_WRITE_VF_CONFIG_BLOCK_PARAMETERS
{
    _In_    NDIS_OBJECT_HEADER              Header;
    _In_    NDIS_SRIOV_FUNCTION_ID          VFId;
    _In_    ULONG                           BlockId;
    _In_    ULONG                           Length;
    _In_    ULONG                           BufferOffset;
} NDIS_SRIOV_WRITE_VF_CONFIG_BLOCK_PARAMETERS, *PNDIS_SRIOV_WRITE_VF_CONFIG_BLOCK_PARAMETERS;

#define NDIS_SIZEOF_SRIOV_WRITE_VF_CONFIG_BLOCK_PARAMETERS_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SRIOV_WRITE_VF_CONFIG_BLOCK_PARAMETERS, BufferOffset)

//
// This structure is used in OID_SRIOV_RESET_VF
//
#define NDIS_SRIOV_RESET_VF_PARAMETERS_REVISION_1             1

typedef struct _NDIS_SRIOV_RESET_VF_PARAMETERS
{
    _In_    NDIS_OBJECT_HEADER              Header;
    _In_    NDIS_SRIOV_FUNCTION_ID          VFId;
} NDIS_SRIOV_RESET_VF_PARAMETERS, *PNDIS_SRIOV_RESET_VF_PARAMETERS;

#define NDIS_SIZEOF_SRIOV_RESET_VF_PARAMETERS_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SRIOV_RESET_VF_PARAMETERS, VFId)

//
// This structure is used in OID_SRIOV_SET_VF_POWER_STATE
//
#define NDIS_SRIOV_SET_VF_POWER_STATE_PARAMETERS_REVISION_1             1

typedef struct _NDIS_SRIOV_SET_VF_POWER_STATE_PARAMETERS
{
    _In_    NDIS_OBJECT_HEADER              Header;
    _In_    NDIS_SRIOV_FUNCTION_ID          VFId;
    _In_    NDIS_DEVICE_POWER_STATE         PowerState;
    _In_    BOOLEAN                         WakeEnable;
} NDIS_SRIOV_SET_VF_POWER_STATE_PARAMETERS, *PNDIS_SRIOV_SET_VF_POWER_STATE_PARAMETERS;

#define NDIS_SIZEOF_SRIOV_SET_VF_POWER_STATE_PARAMETERS_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SRIOV_SET_VF_POWER_STATE_PARAMETERS, WakeEnable)

//
// This structure is used in OID_SRIOV_CONFIG_STATE
// This can be sent only to the VF miniport driver
//
#define NDIS_SRIOV_CONFIG_STATE_PARAMETERS_REVISION_1             1

typedef struct _NDIS_SRIOV_CONFIG_STATE_PARAMETERS
{
    _In_    NDIS_OBJECT_HEADER              Header;
    _In_    ULONG                           BlockId;
    _In_    ULONG                           Length;
} NDIS_SRIOV_CONFIG_STATE_PARAMETERS, *PNDIS_SRIOV_CONFIG_STATE_PARAMETERS;

#define NDIS_SIZEOF_SRIOV_CONFIG_STATE_PARAMETERS_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SRIOV_CONFIG_STATE_PARAMETERS, Length)

//
// This structure is used in OID_SRIOV_VF_VENDOR_DEVICE_ID
//
#define NDIS_SRIOV_VF_VENDOR_DEVICE_ID_INFO_REVISION_1             1

typedef struct _NDIS_SRIOV_VF_VENDOR_DEVICE_ID_INFO
{
    _In_    NDIS_OBJECT_HEADER              Header;
    _In_    NDIS_SRIOV_FUNCTION_ID          VFId;
    _Out_   USHORT                          VendorId;
    _Out_   USHORT                          DeviceId;
} NDIS_SRIOV_VF_VENDOR_DEVICE_ID_INFO, *PNDIS_SRIOV_VF_VENDOR_DEVICE_ID_INFO;

#define NDIS_SIZEOF_SRIOV_VF_VENDOR_DEVICE_ID_INFO_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SRIOV_VF_VENDOR_DEVICE_ID_INFO, DeviceId)

//
// This structure is used in OID_SRIOV_PROBED_BARS
//
#define NDIS_SRIOV_PROBED_BARS_INFO_REVISION_1             1

typedef struct _NDIS_SRIOV_PROBED_BARS_INFO
{
    _In_   NDIS_OBJECT_HEADER   Header;
    _In_   ULONG                BaseRegisterValuesOffset;
} NDIS_SRIOV_PROBED_BARS_INFO, *PNDIS_SRIOV_PROBED_BARS_INFO;

#define NDIS_SIZEOF_SRIOV_PROBED_BARS_INFO_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SRIOV_PROBED_BARS_INFO, BaseRegisterValuesOffset)

//
// This structure is used in OID_RECEIVE_FILTER_MOVE_FILTER
//
#define NDIS_RECEIVE_FILTER_MOVE_FILTER_PARAMETERS_REVISION_1             1

typedef struct _NDIS_RECEIVE_FILTER_MOVE_FILTER_PARAMETERS
{
    _In_    NDIS_OBJECT_HEADER          Header;
    _In_    NDIS_RECEIVE_FILTER_ID      FilterId;
    _In_    NDIS_RECEIVE_QUEUE_ID       SourceQueueId;
    _In_    NDIS_NIC_SWITCH_VPORT_ID    SourceVPortId;
    _In_    NDIS_RECEIVE_QUEUE_ID       DestQueueId;
    _In_    NDIS_NIC_SWITCH_VPORT_ID    DestVPortId;
} NDIS_RECEIVE_FILTER_MOVE_FILTER_PARAMETERS, *PNDIS_RECEIVE_FILTER_MOVE_FILTER_PARAMETERS;

#define NDIS_SIZEOF_RECEIVE_FILTER_MOVE_FILTER_PARAMETERS_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_RECEIVE_FILTER_MOVE_FILTER_PARAMETERS, DestVPortId)

//
// This structure is used in OID_SRIOV_BAR_RESOURCES
//
#define NDIS_SRIOV_BAR_RESOURCES_INFO_REVISION_1             1

typedef struct _NDIS_SRIOV_BAR_RESOURCES_INFO
{
    _In_    NDIS_OBJECT_HEADER      Header;
    _In_    NDIS_SRIOV_FUNCTION_ID  VFId;
    _In_    USHORT                  BarIndex;
    _In_    ULONG                   BarResourcesOffset;
} NDIS_SRIOV_BAR_RESOURCES_INFO, *PNDIS_SRIOV_BAR_RESOURCES_INFO;

#define NDIS_SIZEOF_SRIOV_BAR_RESOURCES_INFO_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SRIOV_BAR_RESOURCES_INFO, BarResourcesOffset)

//
// This structure is used in OID_SRIOV_PF_LUID
//
#define NDIS_SRIOV_PF_LUID_INFO_REVISION_1             1

typedef struct _NDIS_SRIOV_PF_LUID_INFO
{
    _In_    NDIS_OBJECT_HEADER      Header;
    _Out_   LUID                    Luid;
} NDIS_SRIOV_PF_LUID_INFO, *PNDIS_SRIOV_PF_LUID_INFO;

#define NDIS_SIZEOF_SRIOV_PF_LUID_INFO_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SRIOV_PF_LUID_INFO, Luid)

//
// This structure is used in OID_SRIOV_VF_SERIAL_NUMBER
//
#define NDIS_SRIOV_VF_SERIAL_NUMBER_INFO_REVISION_1             1

typedef struct _NDIS_SRIOV_VF_SERIAL_NUMBER_INFO
{
    _In_    NDIS_OBJECT_HEADER      Header;
    _Out_   ULONG                   SerialNumber;
} NDIS_SRIOV_VF_SERIAL_NUMBER_INFO, *PNDIS_SRIOV_VF_SERIAL_NUMBER_INFO;

#define NDIS_SIZEOF_SRIOV_VF_SERIAL_NUMBER_INFO_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SRIOV_VF_SERIAL_NUMBER_INFO, SerialNumber)

//
// This structure is used in OID_SRIOV_VF_INVALIDATE_CONFIG_BLOCK
//
#define NDIS_SRIOV_VF_INVALIDATE_CONFIG_BLOCK_INFO_REVISION_1   1

typedef struct _NDIS_SRIOV_VF_INVALIDATE_CONFIG_BLOCK_INFO
{
    _In_    NDIS_OBJECT_HEADER      Header;
    _In_    ULONG64 BlockMask;
} NDIS_SRIOV_VF_INVALIDATE_CONFIG_BLOCK_INFO, *PNDIS_SRIOV_VF_INVALIDATE_CONFIG_BLOCK_INFO;

#define NDIS_SIZEOF_SRIOV_VF_INVALIDATE_CONFIG_BLOCK_INFO_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SRIOV_VF_INVALIDATE_CONFIG_BLOCK_INFO, BlockMask)

#if (NDIS_SUPPORT_NDIS670)

typedef struct _NDIS_SRIOV_OVERLYING_ADAPTER_INFO
{
    NDIS_OBJECT_HEADER              Header;
    ULONG                           Flags;
    ULONG                           IfIndex;
    ULONG                           NdisReserved1;
    ULONG                           NdisReserved2;
} NDIS_SRIOV_OVERLYING_ADAPTER_INFO, *PNDIS_SRIOV_OVERLYING_ADAPTER_INFO;

#define NDIS_SRIOV_OVERLYING_ADAPTER_INFO_VERSION_1 1

#define NDIS_SIZEOF_SRIOV_OVERLYING_ADAPTER_INFO_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SRIOV_OVERLYING_ADAPTER_INFO, NdisReserved2)

#endif //(NDIS_SUPPORT_NDIS670)

#if (NTDDI_VERSION >= NTDDI_WINBLUE) || (NDIS_SUPPORT_NDIS640)

//
// Supported isolation modes.
//
typedef enum _NDIS_ISOLATION_MODE
{
    NdisIsolationModeNone = 0,
    //
    // VirtualSubnetId based isolation provided by
    // in-box by Windows Network Virtualization.
    //
    NdisIsolationModeNativeVirtualSubnet = 1,
    //
    // VirtualSubnetId based isolation provided by a
    // switch extension.
    //
    NdisIsolationModeExternalVirtualSubnet = 2,
    //
    // VLAN based isolation.
    //
    NdisIsolationModeVlan = 3,
} NDIS_ISOLATION_MODE, *PNDIS_ISOLATION_MODE;

typedef GUID NDIS_ROUTING_DOMAIN_ID, *PNDIS_ROUTING_DOMAIN_ID;

#define NDIS_ISOLATION_NAME_MAX_STRING_SIZE 127
typedef struct _NDIS_ISOLATION_NAME
{
    USHORT      Length; // in -Bytes-
    WCHAR       String[NDIS_ISOLATION_NAME_MAX_STRING_SIZE + 1];
} NDIS_ISOLATION_NAME;

typedef NDIS_ISOLATION_NAME NDIS_ISOLATION_ID_NAME, *PNDIS_ISOLATION_ID_NAME;
typedef NDIS_ISOLATION_NAME NDIS_ROUTING_DOMAIN_NAME, *PNDIS_ROUTING_DOMAIN_NAME;

//
// Information relating to a single Isolation ID within a
// Routing domain.
//
typedef struct _NDIS_ROUTING_DOMAIN_ISOLATION_ENTRY
{
    NDIS_OBJECT_HEADER          Header;
    ULONG                       Flags;
    NDIS_ISOLATION_ID_NAME      IsolationIdName;
    union
    {
        struct
        {
            UINT32              VirtualSubnetId:24;
        };
        struct
        {
            UINT32              VlanId:12;
        };
        UINT32                  IsolationId;
    };
} NDIS_ROUTING_DOMAIN_ISOLATION_ENTRY, *PNDIS_ROUTING_DOMAIN_ISOLATION_ENTRY;

#define NDIS_ROUTING_DOMAIN_ISOLATION_ENTRY_REVISION_1      1

#define NDIS_SIZEOF_NDIS_ROUTING_DOMAIN_ISOLATION_ENTRY_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_ROUTING_DOMAIN_ISOLATION_ENTRY,  IsolationId)

//
// Retrieves the next NDIS_ROUTING_DOMAIN_ISOLATION_ENTRY object in the list.
//
#define NDIS_ROUTING_DOMAIN_ISOLATION_ENTRY_GET_NEXT(_IsolationInfoEntry_)\
                ((PNDIS_ROUTING_DOMAIN_ISOLATION_ENTRY)((PUCHAR)(_IsolationInfoEntry_) +\
                        (_IsolationInfoEntry_)->Header.Size))

//
// Information relating to a single Routing Domain entry.
//
typedef struct _NDIS_ROUTING_DOMAIN_ENTRY
{
    NDIS_OBJECT_HEADER          Header;
    ULONG                       Flags;
    NDIS_ROUTING_DOMAIN_ID      RoutingDomainId;
    NDIS_ROUTING_DOMAIN_NAME    RoutingDomainName;
    ULONG                       NumIsolationEntries;

    ULONG                       FirstIsolationEntryOffset;
} NDIS_ROUTING_DOMAIN_ENTRY, *PNDIS_ROUTING_DOMAIN_ENTRY;

#define NDIS_ROUTING_DOMAIN_ENTRY_REVISION_1      1

#define NDIS_SIZEOF_NDIS_ROUTING_DOMAIN_ENTRY_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_ROUTING_DOMAIN_ENTRY,  FirstIsolationEntryOffset)

//
// Retrieves the next NDIS_ROUTING_DOMAIN_ENTRY object in the list.
//
#define NDIS_ROUTING_DOMAIN_ENTRY_GET_NEXT(_RoutingDomainEntry_)\
            ((PNDIS_ROUTING_DOMAIN_ENTRY)((PUCHAR)(_RoutingDomainEntry_) +\
                    (_RoutingDomainEntry_)->Header.Size))

//
// Retrieves the first NDIS_ROUTING_DOMAIN_ISOLATION_ENTRY object in the Isolation info.
//
#define NDIS_ROUTING_DOMAIN_ENTRY_GET_FIRST_ISOLATION_ENTRY(_RoutingDomainEntry_)\
            ((PNDIS_ROUTING_DOMAIN_ISOLATION_ENTRY)((PUCHAR)(_RoutingDomainEntry_) + \
                (_RoutingDomainEntry_)->FirstIsolationEntryOffset))

//
// Isolation configuration for a given network adapter.
//
typedef struct _NDIS_ISOLATION_PARAMETERS
{
    NDIS_OBJECT_HEADER                  Header;
    ULONG                               Flags;
    NDIS_ISOLATION_MODE                 IsolationMode;
    BOOLEAN                             AllowUntaggedTraffic;
    ULONG                               NumRoutingDomainEntries;
    ULONG                               FirstRoutingDomainEntryOffset;
} NDIS_ISOLATION_PARAMETERS, *PNDIS_ISOLATION_PARAMETERS;

#define NDIS_ISOLATION_PARAMETERS_REVISION_1      1

#define NDIS_SIZEOF_NDIS_ISOLATION_PARAMETERS_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_ISOLATION_PARAMETERS,  FirstRoutingDomainEntryOffset)

//
// Retrieves the first NDIS_ROUTING_DOMAIN_ENTRY object in the isolation info.
//
#define NDIS_ISOLATION_PARAMETERS_GET_FIRST_ROUTING_DOMAIN_ENTRY(_MultiTenancyInfo_)\
            ((PNDIS_ROUTING_DOMAIN_ENTRY)((PUCHAR)(_MultiTenancyInfo_) + \
                (_MultiTenancyInfo_)->FirstRoutingDomainEntryOffset))

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE) || (NDIS_SUPPORT_NDIS640)

//
// NDIS_SWITCH_PORT_ID and NDIS_SWITCH_NIC_INDEX are needed
// in user and kernel level headers without a common include,
// so it is defined in multiple places.
//
#ifndef _NDIS_SWITCH_PORT_ID
#define _NDIS_SWITCH_PORT_ID NDIS_SWITCH_PORT_ID
typedef UINT32 NDIS_SWITCH_PORT_ID, *PNDIS_SWITCH_PORT_ID;
typedef USHORT NDIS_SWITCH_NIC_INDEX, *PNDIS_SWITCH_NIC_INDEX;
#else
//
// If already defined, make sure sizes match.
//
C_ASSERT(sizeof(NDIS_SWITCH_PORT_ID) == sizeof(UINT32));
C_ASSERT(sizeof(NDIS_SWITCH_NIC_INDEX) == sizeof(USHORT));
#endif

typedef GUID NDIS_SWITCH_OBJECT_INSTANCE_ID, *PNDIS_SWITCH_OBJECT_INSTANCE_ID;
typedef GUID NDIS_SWITCH_OBJECT_ID, *PNDIS_SWITCH_OBJECT_ID;
typedef USHORT NDIS_SWITCH_OBJECT_VERSION, *PNDIS_SWITCH_OBJECT_VERSION;
typedef USHORT NDIS_SWITCH_OBJECT_SERIALIZATION_VERSION, *PNDIS_SWITCH_OBJECT_SERIALIZATION_VERSION;

#define NDIS_SWITCH_OBJECT_SERIALIZATION_VERSION_1       1

typedef enum _NDIS_SWITCH_PORT_PROPERTY_TYPE
{
    NdisSwitchPortPropertyTypeUndefined,
    NdisSwitchPortPropertyTypeCustom,
    NdisSwitchPortPropertyTypeSecurity,
    NdisSwitchPortPropertyTypeVlan,
    NdisSwitchPortPropertyTypeProfile,
    NdisSwitchPortPropertyTypeIsolation,
    NdisSwitchPortPropertyTypeRoutingDomain,
    NdisSwitchPortPropertyTypeMaximum
} NDIS_SWITCH_PORT_PROPERTY_TYPE, *PNDIS_SWITCH_PORT_PROPERTY_TYPE;

//
// This structure is used for NdisSwitchPortPropertyTypeSecurity.
//

#define NDIS_SWITCH_PORT_PROPERTY_SECURITY_REVISION_1       1

#if (NDIS_SUPPORT_NDIS640)
#define NDIS_SWITCH_PORT_PROPERTY_SECURITY_REVISION_2       2
#endif // (NDIS_SUPPORT_NDIS640)

typedef struct _NDIS_SWITCH_PORT_PROPERTY_SECURITY
{
    NDIS_OBJECT_HEADER      Header;
    ULONG                   Flags;
    BOOLEAN                 AllowMacSpoofing;
    BOOLEAN                 AllowIeeePriorityTag;
    UINT32                  VirtualSubnetId;
    BOOLEAN                 AllowTeaming;

#if (NDIS_SUPPORT_NDIS640)
    UINT32                  DynamicIPAddressLimit;
#endif // (NDIS_SUPPORT_NDIS640)

} NDIS_SWITCH_PORT_PROPERTY_SECURITY, *PNDIS_SWITCH_PORT_PROPERTY_SECURITY;

#define NDIS_SIZEOF_NDIS_SWITCH_PORT_PROPERTY_SECURITY_REVISION_1       \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PORT_PROPERTY_SECURITY, AllowTeaming)

#if (NDIS_SUPPORT_NDIS640)
#define NDIS_SIZEOF_NDIS_SWITCH_PORT_PROPERTY_SECURITY_REVISION_2       \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PORT_PROPERTY_SECURITY, DynamicIPAddressLimit)
#endif // (NDIS_SUPPORT_NDIS640)

typedef enum _NDIS_SWITCH_PORT_VLAN_MODE
{
    NdisSwitchPortVlanModeUnknown      = 0,
    NdisSwitchPortVlanModeAccess       = 1,
    NdisSwitchPortVlanModeTrunk        = 2,
    NdisSwitchPortVlanModePrivate      = 3,
    NdisSwitchPortVlanModeMax          = 4
} NDIS_SWITCH_PORT_VLAN_MODE, *PNDIS_SWITCH_PORT_VLAN_MODE;

typedef enum _NDIS_SWITCH_PORT_PVLAN_MODE
{
    NdisSwitchPortPvlanModeUndefined = 0,
    NdisSwitchPortPvlanModeIsolated,
    NdisSwitchPortPvlanModeCommunity,
    NdisSwitchPortPvlanModePromiscuous
} NDIS_SWITCH_PORT_PVLAN_MODE, *PNDIS_SWITCH_PORT_PVLAN_MODE;

//
// This structure is used for NdisSwitchPortPropertyTypeVlan.
//

#define NDIS_SWITCH_PORT_PROPERTY_VLAN_REVISION_1       1

typedef struct _NDIS_SWITCH_PORT_PROPERTY_VLAN
{
    NDIS_OBJECT_HEADER          Header;
    ULONG                       Flags;

    NDIS_SWITCH_PORT_VLAN_MODE  OperationMode;
    union
    {
        struct
        {
            UINT16              AccessVlanId;
            UINT16              NativeVlanId;
            UINT64              PruneVlanIdArray[64];
            UINT64              TrunkVlanIdArray[64];
        } VlanProperties;
        struct
        {
            NDIS_SWITCH_PORT_PVLAN_MODE
                                PvlanMode;
            UINT16              PrimaryVlanId;
            union
            {
                UINT16          SecondaryVlanId;
                UINT64          SecondaryVlanIdArray[64];
            };
        } PvlanProperties;
    };
} NDIS_SWITCH_PORT_PROPERTY_VLAN, *PNDIS_SWITCH_PORT_PROPERTY_VLAN;

#define NDIS_SIZEOF_NDIS_SWITCH_PORT_PROPERTY_VLAN_REVISION_1       \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PORT_PROPERTY_VLAN, VlanProperties)

//
// This structure is used for NdisSwitchPortPropertyTypeProfile.
//

#define NDIS_SWITCH_PORT_PROPERTY_PROFILE_REVISION_1       1

typedef struct _NDIS_SWITCH_PORT_PROPERTY_PROFILE
{
    NDIS_OBJECT_HEADER                          Header;
    ULONG                                       Flags;
    NDIS_SWITCH_PORT_PROPERTY_PROFILE_NAME      ProfileName;
    GUID                                        ProfileId;
    NDIS_VENDOR_NAME                            VendorName;
    GUID                                        VendorId;
    UINT32                                      ProfileData;
    GUID                                        NetCfgInstanceId;
    struct
    {
        UINT32                                  PciSegmentNumber:16;
        UINT32                                  PciBusNumber:8;
        UINT32                                  PciDeviceNumber:5;
        UINT32                                  PciFunctionNumber:3;
    } PciLocation;
    UINT32                                      CdnLabelId;
    NDIS_SWITCH_PORT_PROPERTY_PROFILE_CDN_LABEL CdnLabel;
} NDIS_SWITCH_PORT_PROPERTY_PROFILE, *PNDIS_SWITCH_PORT_PROPERTY_PROFILE;

#define NDIS_SIZEOF_NDIS_SWITCH_PORT_PROPERTY_PROFILE_REVISION_1       \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PORT_PROPERTY_PROFILE, CdnLabel)

#if (NTDDI_VERSION >= NTDDI_WINBLUE) || (NDIS_SUPPORT_NDIS640)

//
// This structure is used for NdisSwitchPortPropertyTypeIsolation.
//

#define NDIS_SWITCH_PORT_PROPERTY_ISOLATION_REVISION_1       1

typedef struct _NDIS_SWITCH_PORT_PROPERTY_ISOLATION
{
    NDIS_OBJECT_HEADER                  Header;
    ULONG                               Flags;
    NDIS_ISOLATION_MODE                 IsolationMode;
    BOOLEAN                             AllowUntaggedTraffic;
    UINT32                              DefaultIsolationId;
} NDIS_SWITCH_PORT_PROPERTY_ISOLATION, *PNDIS_SWITCH_PORT_PROPERTY_ISOLATION;

#define NDIS_SIZEOF_NDIS_SWITCH_PORT_PROPERTY_ISOLATION_REVISION_1       \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PORT_PROPERTY_ISOLATION, DefaultIsolationId)

//
// This structure is used for NdisSwitchPortPropertyTypeRoutingDomain.
//

#define NDIS_SWITCH_PORT_PROPERTY_ROUTING_DOMAIN_REVISION_1       1

typedef struct _NDIS_SWITCH_PORT_PROPERTY_ROUTING_DOMAIN
{
    NDIS_OBJECT_HEADER                  Header;
    ULONG                               Flags;
    NDIS_ROUTING_DOMAIN_ID              RoutingDomainId;
    NDIS_ROUTING_DOMAIN_NAME            RoutingDomainName;
    ULONG                               NumIsolationEntries;
    USHORT                              FirstIsolationEntryOffset;
} NDIS_SWITCH_PORT_PROPERTY_ROUTING_DOMAIN, *PNDIS_SWITCH_PORT_PROPERTY_ROUTING_DOMAIN;

#define NDIS_SIZEOF_NDIS_SWITCH_PORT_PROPERTY_ROUTING_DOMAIN_REVISION_1       \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PORT_PROPERTY_ROUTING_DOMAIN, FirstIsolationEntryOffset)

//
// Retrieves the first NDIS_ROUTING_DOMAIN_ISOLATION_ENTRY object in the Isolation info.
//
#define NDIS_SWITCH_PORT_PROPERTY_ROUTING_DOMAIN_GET_FIRST_ISOLATION_ENTRY(_RoutingDomainProperty_)\
            ((PNDIS_ROUTING_DOMAIN_ISOLATION_ENTRY)((PUCHAR)(_RoutingDomainProperty_) + \
                (_RoutingDomainProperty_)->FirstIsolationEntryOffset))

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE) || (NDIS_SUPPORT_NDIS640)

//
// This structure is used for NdisSwitchPortPropertyTypeCustom.
//

#define NDIS_SWITCH_PORT_PROPERTY_CUSTOM_REVISION_1       1

typedef struct _NDIS_SWITCH_PORT_PROPERTY_CUSTOM
{
    NDIS_OBJECT_HEADER      Header;
    ULONG                   Flags;
    ULONG                   PropertyBufferLength;
    ULONG                   PropertyBufferOffset;
} NDIS_SWITCH_PORT_PROPERTY_CUSTOM, *PNDIS_SWITCH_PORT_PROPERTY_CUSTOM;

#define NDIS_SIZEOF_NDIS_SWITCH_PORT_PROPERTY_CUSTOM_REVISION_1       \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PORT_PROPERTY_CUSTOM, PropertyBufferOffset)

//
// Retrieves the custom property buffer described by the
// NDIS_SWITCH_PORT_PROPERTY_CUSTOM structure.
//
#define NDIS_SWITCH_PORT_PROPERTY_CUSTOM_GET_BUFFER(_PortPropertyCustom_)\
            ((PVOID)((PUCHAR)(_PortPropertyCustom_) + (_PortPropertyCustom_)->PropertyBufferOffset))

//
// Since the property buffer may need 8 byte alignment, we require this struct
// to be 8 byte aligned.
C_ASSERT((sizeof(NDIS_SWITCH_PORT_PROPERTY_CUSTOM) % sizeof(UINT64)) == 0);

//
// This structure is used in OID_SWITCH_PORT_PROPERTY_ADD,
// OID_SWITCH_PORT_PROPERTY_UPDATE.
//

#define NDIS_SWITCH_PORT_PROPERTY_PARAMETERS_REVISION_1     1

typedef struct _NDIS_SWITCH_PORT_PROPERTY_PARAMETERS
{
    NDIS_OBJECT_HEADER                      Header;
    ULONG                                   Flags;
    NDIS_SWITCH_PORT_ID                     PortId;
    NDIS_SWITCH_PORT_PROPERTY_TYPE          PropertyType;
    NDIS_SWITCH_OBJECT_ID                   PropertyId;
    NDIS_SWITCH_OBJECT_VERSION              PropertyVersion;
    NDIS_SWITCH_OBJECT_SERIALIZATION_VERSION SerializationVersion;
    NDIS_SWITCH_OBJECT_INSTANCE_ID          PropertyInstanceId;
    ULONG                                   PropertyBufferLength;
    ULONG                                   PropertyBufferOffset;
    ULONG                                   Reserved;
} NDIS_SWITCH_PORT_PROPERTY_PARAMETERS, *PNDIS_SWITCH_PORT_PROPERTY_PARAMETERS;

#define NDIS_SIZEOF_NDIS_SWITCH_PORT_PROPERTY_PARAMETERS_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PORT_PROPERTY_PARAMETERS, Reserved)

//
// Returns the property described by the NDIS_SWITCH_PORT_PROPERTY_PARAMETERS structure.
//
#define NDIS_SWITCH_PORT_PROPERTY_PARAMETERS_GET_PROPERTY(_PortParameters_)\
            ((PVOID)((PUCHAR)(_PortParameters_) + (_PortParameters_)->PropertyBufferOffset))

//
// Converts the version defined from the property MOF files (e.g. "1.0") to the serialized
// value passed to vSwitch extensions (e.g. 0x100).
//
#define NDIS_SWITCH_CREATE_PROPERTY_VERSION(_VersionMajor_,_VersionMinor_)\
            (((_VersionMajor_) << 8) + (_VersionMinor_))

//
// Since the property buffer may need 8 byte alignment, we require this struct
// to be 8 byte aligned.
//
C_ASSERT((sizeof(NDIS_SWITCH_PORT_PROPERTY_PARAMETERS) % sizeof(UINT64)) == 0);

//
// This structure is used in OID_SWITCH_PORT_PROPERTY_DELETE.
//

#define NDIS_SWITCH_PORT_PROPERTY_DELETE_PARAMETERS_REVISION_1  1

typedef struct _NDIS_SWITCH_PORT_PROPERTY_DELETE_PARAMETERS
{
    NDIS_OBJECT_HEADER                      Header;
    ULONG                                   Flags;
    NDIS_SWITCH_PORT_ID                     PortId;
    NDIS_SWITCH_PORT_PROPERTY_TYPE          PropertyType;
    NDIS_SWITCH_OBJECT_ID                   PropertyId;
    NDIS_SWITCH_OBJECT_INSTANCE_ID          PropertyInstanceId;
} NDIS_SWITCH_PORT_PROPERTY_DELETE_PARAMETERS, *PNDIS_SWITCH_PORT_PROPERTY_DELETE_PARAMETERS;

#define NDIS_SIZEOF_NDIS_SWITCH_PORT_PROPERTY_DELETE_PARAMETERS_REVISION_1  \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PORT_PROPERTY_DELETE_PARAMETERS, PropertyInstanceId)

//
// The following structures are used for OID_SWITCH_PORT_PROPERTY_ENUM.
//

#define NDIS_SWITCH_PORT_PROPERTY_ENUM_PARAMETERS_REVISION_1    1

typedef struct _NDIS_SWITCH_PORT_PROPERTY_ENUM_PARAMETERS
{
    NDIS_OBJECT_HEADER                      Header;
    ULONG                                   Flags;
    NDIS_SWITCH_PORT_ID                     PortId;
    NDIS_SWITCH_PORT_PROPERTY_TYPE          PropertyType;
    NDIS_SWITCH_OBJECT_ID                   PropertyId;
    NDIS_SWITCH_OBJECT_SERIALIZATION_VERSION SerializationVersion;
    ULONG                                   FirstPropertyOffset;
    ULONG                                   NumProperties;
    USHORT                                  Reserved;
} NDIS_SWITCH_PORT_PROPERTY_ENUM_PARAMETERS, *PNDIS_SWITCH_PORT_PROPERTY_ENUM_PARAMETERS;

#define NDIS_SIZEOF_NDIS_SWITCH_PORT_PROPERTY_ENUM_PARAMETERS_REVISION_1    \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PORT_PROPERTY_ENUM_PARAMETERS, Reserved)

//
// Retrieves the first NDIS_SWITCH_PORT_PROPERTY_ENUM_INFO object in the enumeration.
//
#define NDIS_SWITCH_PORT_PROPERTY_ENUM_PARAMETERS_GET_FIRST_INFO(_PortEnumParams_)\
            ((PNDIS_SWITCH_PORT_PROPERTY_ENUM_INFO)((PUCHAR)(_PortEnumParams_) + \
                (_PortEnumParams_)->FirstPropertyOffset))

//
// Since the property buffer may need 8 byte alignment, we require this struct
// to be 8 byte aligned.
//
C_ASSERT((sizeof(NDIS_SWITCH_PORT_PROPERTY_ENUM_PARAMETERS) % sizeof(UINT64)) == 0);

#define NDIS_SWITCH_PORT_PROPERTY_ENUM_INFO_REVISION_1  1

typedef struct _NDIS_SWITCH_PORT_PROPERTY_ENUM_INFO
{
    NDIS_OBJECT_HEADER                      Header;
    ULONG                                   Flags;
    NDIS_SWITCH_OBJECT_VERSION              PropertyVersion;
    NDIS_SWITCH_OBJECT_INSTANCE_ID          PropertyInstanceId;
    ULONG                                   QwordAlignedPropertyBufferLength;
    ULONG                                   PropertyBufferLength;
    ULONG                                   PropertyBufferOffset;
} NDIS_SWITCH_PORT_PROPERTY_ENUM_INFO, *PNDIS_SWITCH_PORT_PROPERTY_ENUM_INFO;

#define NDIS_SIZEOF_NDIS_SWITCH_PORT_PROPERTY_ENUM_INFO_REVISION_1   \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PORT_PROPERTY_ENUM_INFO, PropertyBufferOffset)

//
// Retrieves the next NDIS_SWITCH_PORT_PROPERTY_ENUM_INFO object in the enumeration.
//
#define NDIS_SWITCH_PORT_PROPERTY_ENUM_INFO_GET_NEXT(_PortEnumInfo_)\
            ((PNDIS_SWITCH_PORT_PROPERTY_ENUM_INFO)\
                ((ULONG_PTR)(_PortEnumInfo_) +\
                (_PortEnumInfo_)->QwordAlignedPropertyBufferLength +\
                 sizeof(NDIS_SWITCH_PORT_PROPERTY_ENUM_INFO)))

//
// Returns the property described by the NDIS_SWITCH_PORT_PROPERTY_ENUM_INFO entry.
//
#define NDIS_SWITCH_PORT_PROPERTY_ENUM_INFO_GET_PROPERTY(_PortEnumInfo_)\
            ((PVOID)((PUCHAR)(_PortEnumInfo_) + (_PortEnumInfo_)->PropertyBufferOffset))

//
// Since the property buffer may need 8 byte alignment, we require this struct
// to be 8 byte aligned.
//
C_ASSERT((sizeof(NDIS_SWITCH_PORT_PROPERTY_ENUM_INFO) % sizeof(UINT64)) == 0);

//
// This structure is used in OID_SWITCH_PORT_FEATURE_STATUS_QUERY.
//

typedef enum _NDIS_SWITCH_PORT_FEATURE_STATUS_TYPE
{
    NdisSwitchPortFeatureStatusTypeUndefined,
    NdisSwitchPortFeatureStatusTypeCustom,
    NdisSwitchPortFeatureStatusTypeMaximum
} NDIS_SWITCH_PORT_FEATURE_STATUS_TYPE, *PNDIS_SWITCH_PORT_FEATURE_STATUS_TYPE;

#define NDIS_SWITCH_PORT_FEATURE_STATUS_PARAMETERS_REVISION_1     1

typedef struct _NDIS_SWITCH_PORT_FEATURE_STATUS_PARAMETERS
{
    NDIS_OBJECT_HEADER                            Header;
    ULONG                                         Flags;
    NDIS_SWITCH_PORT_ID                           PortId;
    NDIS_SWITCH_PORT_FEATURE_STATUS_TYPE          FeatureStatusType;
    NDIS_SWITCH_OBJECT_ID                         FeatureStatusId;
    NDIS_SWITCH_OBJECT_VERSION                    FeatureStatusVersion;
    NDIS_SWITCH_OBJECT_SERIALIZATION_VERSION      SerializationVersion;
    NDIS_SWITCH_OBJECT_INSTANCE_ID                FeatureStatusInstanceId;
    ULONG                                         FeatureStatusBufferLength;
    ULONG                                         FeatureStatusBufferOffset;
    ULONG                                         Reserved;
} NDIS_SWITCH_PORT_FEATURE_STATUS_PARAMETERS, *PNDIS_SWITCH_PORT_FEATURE_STATUS_PARAMETERS;

#define NDIS_SIZEOF_NDIS_SWITCH_PORT_FEATURE_STATUS_PARAMETERS_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PORT_FEATURE_STATUS_PARAMETERS, Reserved)

//
// Since the resource buffer may need 8 byte alignment, we require this struct
// to be 8 byte aligned.
//
C_ASSERT((sizeof(NDIS_SWITCH_PORT_FEATURE_STATUS_PARAMETERS) % sizeof(UINT64)) == 0);

//
// Converts the version defined from the property MOF files (e.g. "1.0") to the serialized
// value passed to vSwitch extensions (e.g. 0x100).
//
#define NDIS_SWITCH_CREATE_FEATURE_STATUS_VERSION(_VersionMajor_,_VersionMinor_)\
            (((_VersionMajor_) << 8) + (_VersionMinor_))

//
// This structure is used for the resource status for NdisSwitchPortPropertyTypeCustom.
//

#define NDIS_SWITCH_PORT_FEATURE_STATUS_CUSTOM_REVISION_1       1

typedef struct _NDIS_SWITCH_PORT_FEATURE_STATUS_CUSTOM
{
    NDIS_OBJECT_HEADER      Header;
    ULONG                   Flags;
    ULONG                   FeatureStatusBufferLength;
    ULONG                   FeatureStatusBufferOffset;
} NDIS_SWITCH_PORT_FEATURE_STATUS_CUSTOM, *PNDIS_SWITCH_PORT_FEATURE_STATUS_CUSTOM;

#define NDIS_SIZEOF_NDIS_SWITCH_PORT_FEATURE_STATUS_CUSTOM_REVISION_1       \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PORT_FEATURE_STATUS_CUSTOM, FeatureStatusBufferOffset)

typedef enum _NDIS_SWITCH_PROPERTY_TYPE
{
    NdisSwitchPropertyTypeUndefined,
    NdisSwitchPropertyTypeCustom,
    NdisSwitchPropertyTypeMaximum
} NDIS_SWITCH_PROPERTY_TYPE, *PNDIS_SWITCH_PROPERTY_TYPE;

//
// This structure is used for NdisSwitchPropertyTypeCustom.
//

#define NDIS_SWITCH_PROPERTY_CUSTOM_REVISION_1       1

typedef struct _NDIS_SWITCH_PROPERTY_CUSTOM
{
    NDIS_OBJECT_HEADER      Header;
    ULONG                   Flags;
    ULONG                   PropertyBufferLength;
    ULONG                   PropertyBufferOffset;
} NDIS_SWITCH_PROPERTY_CUSTOM, *PNDIS_SWITCH_PROPERTY_CUSTOM;

#define NDIS_SIZEOF_NDIS_SWITCH_PROPERTY_CUSTOM_REVISION_1       \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PROPERTY_CUSTOM, PropertyBufferOffset)

//
// Retrieves the custom property buffer described by the
// NDIS_SWITCH_PROPERTY_CUSTOM structure.
//
#define NDIS_SWITCH_PROPERTY_CUSTOM_GET_BUFFER(_SwitchPropertyCustom_)\
            ((PVOID)((PUCHAR)(_SwitchPropertyCustom_) + (_SwitchPropertyCustom_)->PropertyBufferOffset))


#define NDIS_SWITCH_PROPERTY_PARAMETERS_REVISION_1  1

//
// This structure is used in OID_SWITCH_PROPERTY_ADD and
// OID_SWITCH_PROPERTY_UPDATE
//
typedef struct _NDIS_SWITCH_PROPERTY_PARAMETERS
{
    NDIS_OBJECT_HEADER                         Header;
    ULONG                                      Flags;
    NDIS_SWITCH_PROPERTY_TYPE                  PropertyType;
    NDIS_SWITCH_OBJECT_ID                      PropertyId;
    NDIS_SWITCH_OBJECT_VERSION                 PropertyVersion;
    NDIS_SWITCH_OBJECT_SERIALIZATION_VERSION   SerializationVersion;
    NDIS_SWITCH_OBJECT_INSTANCE_ID             PropertyInstanceId;
    ULONG                                      PropertyBufferLength;
    ULONG                                      PropertyBufferOffset;
} NDIS_SWITCH_PROPERTY_PARAMETERS, *PNDIS_SWITCH_PROPERTY_PARAMETERS;

#define NDIS_SIZEOF_NDIS_SWITCH_PROPERTY_PARAMETERS_REVISION_1   \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PROPERTY_PARAMETERS, PropertyBufferOffset)

//
// Returns the property described by the NNDIS_SWITCH_PROPERTY_PARAMETERS structure.
//
#define NDIS_SWITCH_PROPERTY_PARAMETERS_GET_PROPERTY(_SwitchParameters_)\
            ((PVOID)((PUCHAR)(_SwitchParameters_) + (_SwitchParameters_)->PropertyBufferOffset))

C_ASSERT((sizeof(NDIS_SWITCH_PROPERTY_PARAMETERS) % sizeof(UINT64)) == 0);

#define NDIS_SWITCH_PROPERTY_DELETE_PARAMETERS_REVISION_1  1

//
// This structure is used in OID_SWITCH_PROPERTY_DELETE.
//
typedef struct _NDIS_SWITCH_PROPERTY_DELETE_PARAMETERS
{
    NDIS_OBJECT_HEADER                 Header;
    ULONG                              Flags;
    NDIS_SWITCH_PROPERTY_TYPE          PropertyType;
    NDIS_SWITCH_OBJECT_ID              PropertyId;
    NDIS_SWITCH_OBJECT_INSTANCE_ID     PropertyInstanceId;
} NDIS_SWITCH_PROPERTY_DELETE_PARAMETERS, *PNDIS_SWITCH_PROPERTY_DELETE_PARAMETERS;

#define NDIS_SIZEOF_NDIS_SWITCH_PROPERTY_DELETE_PARAMETERS_REVISION_1   \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PROPERTY_DELETE_PARAMETERS, PropertyInstanceId)

#define NDIS_SWITCH_PROPERTY_ENUM_INFO_REVISION_1  1

//
// This structure is used in OID_SWITCH_PROPERTY_ENUM.
//
typedef struct _NDIS_SWITCH_PROPERTY_ENUM_INFO
{
    NDIS_OBJECT_HEADER               Header;
    ULONG                            Flags;
    NDIS_SWITCH_OBJECT_INSTANCE_ID   PropertyInstanceId;
    NDIS_SWITCH_OBJECT_VERSION       PropertyVersion;
    ULONG                            QwordAlignedPropertyBufferLength;
    ULONG                            PropertyBufferLength;
    ULONG                            PropertyBufferOffset;
} NDIS_SWITCH_PROPERTY_ENUM_INFO, *PNDIS_SWITCH_PROPERTY_ENUM_INFO;

#define NDIS_SIZEOF_NDIS_SWITCH_PROPERTY_ENUM_INFO_REVISION_1   \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PROPERTY_ENUM_INFO, PropertyBufferOffset)

//
// Retrieves the next NDIS_SWITCH_PROPERTY_ENUM_INFO object in the enumeration.
//
#define NDIS_SWITCH_PROPERTY_ENUM_INFO_GET_NEXT(_SwitchEnumInfo_)\
            ((PNDIS_SWITCH_PROPERTY_ENUM_INFO)\
                ((ULONG_PTR)(_SwitchEnumInfo_) +\
                (_SwitchEnumInfo_)->QwordAlignedPropertyBufferLength +\
                 sizeof(NDIS_SWITCH_PROPERTY_ENUM_INFO)))

//
// Returns the property described by the NDIS_SWITCH_PROPERTY_ENUM_INFO entry.
//
#define NDIS_SWITCH_PROPERTY_ENUM_INFO_GET_PROPERTY(_SwitchEnumInfo_)\
            ((PVOID)((PUCHAR)(_SwitchEnumInfo_) + (_SwitchEnumInfo_)->PropertyBufferOffset))

C_ASSERT((sizeof(NDIS_SWITCH_PROPERTY_ENUM_INFO) % sizeof(UINT64)) == 0);

#define NDIS_SWITCH_PROPERTY_ENUM_PARAMETERS_REVISION_1  1

//
// This structure is used in OID_SWITCH_PROPERTY_ENUM.
//
typedef struct _NDIS_SWITCH_PROPERTY_ENUM_PARAMETERS
{
    NDIS_OBJECT_HEADER                         Header;
    ULONG                                      Flags;
    NDIS_SWITCH_PROPERTY_TYPE                  PropertyType;
    NDIS_SWITCH_OBJECT_ID                      PropertyId;
    NDIS_SWITCH_OBJECT_SERIALIZATION_VERSION   SerializationVersion;
    ULONG                                      FirstPropertyOffset;
    ULONG                                      NumProperties;
} NDIS_SWITCH_PROPERTY_ENUM_PARAMETERS, *PNDIS_SWITCH_PROPERTY_ENUM_PARAMETERS;

#define NDIS_SIZEOF_NDIS_SWITCH_PROPERTY_ENUM_PARAMETERS_REVISION_1   \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PROPERTY_ENUM_PARAMETERS, NumProperties)

//
// Retrieves the first NDIS_SWITCH_PROPERTY_ENUM_INFO object in the enumeration.
//
#define NDIS_SWITCH_PROPERTY_ENUM_PARAMETERS_GET_FIRST_INFO(_SwitchEnumParams_)\
            ((PNDIS_SWITCH_PROPERTY_ENUM_INFO)((PUCHAR)(_SwitchEnumParams_) + \
                (_SwitchEnumParams_)->FirstPropertyOffset))

//
// Since the resource buffer may need 8 byte alignment, we require this struct
// to be 8 byte aligned.
//
C_ASSERT((sizeof(NDIS_SWITCH_PROPERTY_ENUM_PARAMETERS) % sizeof(UINT64)) == 0);


//
// This structure is used in OID_SWITCH_FEATURE_STATUS_QUERY.
//
#define NDIS_SWITCH_FEATURE_STATUS_PARAMETERS_REVISION_1         1

typedef enum _NDIS_SWITCH_FEATURE_STATUS_TYPE
{
    NdisSwitchFeatureStatusTypeUndefined,
    NdisSwitchFeatureStatusTypeCustom,
    NdisSwitchFeatureStatusTypeMaximum
} NDIS_SWITCH_FEATURE_STATUS_TYPE, *PNDIS_SWITCH_FEATURE_STATUS_TYPE;

typedef struct _NDIS_SWITCH_FEATURE_STATUS_PARAMETERS
{
    NDIS_OBJECT_HEADER                               Header;
    ULONG                                            Flags;
    NDIS_SWITCH_FEATURE_STATUS_TYPE                  FeatureStatusType;
    NDIS_SWITCH_OBJECT_ID                            FeatureStatusId;
    NDIS_SWITCH_OBJECT_INSTANCE_ID                   FeatureStatusInstanceId;
    NDIS_SWITCH_OBJECT_VERSION                       FeatureStatusVersion;
    NDIS_SWITCH_OBJECT_SERIALIZATION_VERSION         SerializationVersion;
    ULONG                                            FeatureStatusBufferOffset;
    ULONG                                            FeatureStatusBufferLength;
} NDIS_SWITCH_FEATURE_STATUS_PARAMETERS, *PNDIS_SWITCH_FEATURE_STATUS_PARAMETERS;

#define NDIS_SIZEOF_NDIS_SWITCH_FEATURE_STATUS_PARAMETERS_REVISION_1       \
            RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_FEATURE_STATUS_PARAMETERS, FeatureStatusBufferLength)

//
// Since the feature status buffer may need 8 byte alignment, we require this struct
// to be 8 byte aligned.
//
C_ASSERT((sizeof(NDIS_SWITCH_FEATURE_STATUS_PARAMETERS) % sizeof(UINT64)) == 0);

//
// This structure is used for the resource status for NdisSwitchFeatureStatusTypeCustom.
//
#define NDIS_SWITCH_FEATURE_STATUS_CUSTOM_REVISION_1       1

typedef struct _NDIS_SWITCH_FEATURE_STATUS_CUSTOM
{
    NDIS_OBJECT_HEADER      Header;
    ULONG                   Flags;
    ULONG                   FeatureStatusCustomBufferLength;
    ULONG                   FeatureStatusCustomBufferOffset;
} NDIS_SWITCH_FEATURE_STATUS_CUSTOM, *PNDIS_SWITCH_FEATURE_STATUS_CUSTOM;

#define NDIS_SIZEOF_NDIS_SWITCH_FEATURE_STATUS_CUSTOM_REVISION_1       \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_FEATURE_STATUS_CUSTOM, FeatureStatusCustomBufferOffset)

//
// Since the feature status buffer may need 8 byte alignment, we require this struct
// to be 8 byte aligned.
//
C_ASSERT((sizeof(NDIS_SWITCH_FEATURE_STATUS_CUSTOM) % sizeof(UINT64)) == 0);

//
// This structure is used in OID_SWITCH_PARAMETERS
//
#define NDIS_SWITCH_PARAMETERS_REVISION_1      1

typedef struct _NDIS_SWITCH_PARAMETERS
{
  NDIS_OBJECT_HEADER        Header;
  //
  // Reserved for any future structure flags
  //
  ULONG                     Flags;
  //
  // Switch ID as used by management (e.g. WMI)
  //
  NDIS_SWITCH_NAME          SwitchName;
  //
  // Name configured by user, displayed in UI
  //
  NDIS_SWITCH_FRIENDLYNAME  SwitchFriendlyName;
  //
  // The number of ports attached to the switch
  //
  UINT32                    NumSwitchPorts;
  //
  // Whether or not the switch has been activated yet.
  //
  BOOLEAN                   IsActive;
} NDIS_SWITCH_PARAMETERS, *PNDIS_SWITCH_PARAMETERS;

#define NDIS_SIZEOF_NDIS_SWITCH_PARAMETERS_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PARAMETERS, IsActive)

//
// These structures are used in OID_SWITCH_PORT_ARRAY
//

typedef enum _NDIS_SWITCH_PORT_TYPE
{
    //
    // The port type is not known ahead of time. This can occur in
    // ports created through legacy interfaces that do not define
    // the type ahead of time.
    //
    NdisSwitchPortTypeGeneric    = 0,
    //
    // An external port, ultimately connected to
    // an external Nic.
    //
    NdisSwitchPortTypeExternal   = 1,
    //
    // A port ultimately connected to a Synthetic VM
    // Nic.
    //
    NdisSwitchPortTypeSynthetic  = 2,
    //
    // A port ultimately connected to an Emulated VM
    // Nic.
    //
    NdisSwitchPortTypeEmulated   = 3,
    //
    // An internal port, ultimately connected
    // to an Internal NIC.
    //
    NdisSwitchPortTypeInternal   = 4
} NDIS_SWITCH_PORT_TYPE;


typedef enum _NDIS_SWITCH_PORT_STATE
{
    //
    // The port state when state is not known ahead of time.
    //
    NdisSwitchPortStateUnknown      = 0,

    //
    // The port state for created ports.
    //
    NdisSwitchPortStateCreated      = 1,

    //
    // The port state for ports currently being torn down.
    //
    NdisSwitchPortStateTeardown     = 2,

    //
    // The port state for deleted ports.
    //
    NdisSwitchPortStateDeleted      = 3
} NDIS_SWITCH_PORT_STATE;

#if (NDIS_SUPPORT_NDIS650)
//
// Indicates that the internal port should be treated as an untrusted entity.
//
#define NDIS_SWITCH_PORT_PARAMETERS_FLAG_UNTRUSTED_INTERNAL_PORT 0x1
#endif // (NDIS_SUPPORT_NDIS650)

#if (NDIS_SUPPORT_NDIS660)
//
// Indicates that the port has pending saved state to be restored
//
#define NDIS_SWITCH_PORT_PARAMETERS_FLAG_RESTORING_PORT 0x2
#endif // (NDIS_SUPPORT_NDIS660)

typedef struct _NDIS_SWITCH_PORT_PARAMETERS
{
  NDIS_OBJECT_HEADER                Header;
  //
  // Reserved for any future structure flags
  //
  ULONG                             Flags;
  //
  // Number based ID for the port
  //
  NDIS_SWITCH_PORT_ID               PortId;
  //
  // Static port identifier as used by management, such as WMI
  //
  NDIS_SWITCH_PORT_NAME             PortName;
  //
  // Port friendly name
  //
  NDIS_SWITCH_PORT_FRIENDLYNAME     PortFriendlyName;
  //
  // Port type
  //
  NDIS_SWITCH_PORT_TYPE             PortType;
  //
  // If TRUE, the port is for validation and there won't be a
  // corresponding NIC connection. If FALSE, this is a concrete
  // port that will have a NIC connection.
  //
  BOOLEAN                           IsValidationPort;
  //
  // The current state of the port
  //
  NDIS_SWITCH_PORT_STATE            PortState;
} NDIS_SWITCH_PORT_PARAMETERS, *PNDIS_SWITCH_PORT_PARAMETERS;

#define NDIS_SWITCH_PORT_PARAMETERS_REVISION_1      1

#define NDIS_SIZEOF_NDIS_SWITCH_PORT_PARAMETERS_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PORT_PARAMETERS, PortState)

typedef struct _NDIS_SWITCH_PORT_ARRAY
{
  NDIS_OBJECT_HEADER        Header;
  ULONG                     Flags;
  USHORT                    FirstElementOffset;
  ULONG                     NumElements;
  ULONG                     ElementSize;
} NDIS_SWITCH_PORT_ARRAY, *PNDIS_SWITCH_PORT_ARRAY;

#define NDIS_SWITCH_PORT_ARRAY_REVISION_1      1

#define NDIS_SIZEOF_NDIS_SWITCH_PORT_ARRAY_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_PORT_ARRAY, ElementSize)

#define NDIS_SWITCH_PORT_AT_ARRAY_INDEX(_PortArray_, _Index_)\
    ((PNDIS_SWITCH_PORT_PARAMETERS)((PUCHAR)(_PortArray_) + \
                        (_PortArray_)->FirstElementOffset + \
                        ((_PortArray_)->ElementSize * (_Index_))))

#if defined(NDIS_SUPPORT_NDIS640)

//
// Flags for NDIS_SWITCH_NIC_PARAMETERS structure
//
#define NDIS_SWITCH_NIC_FLAGS_NIC_INITIALIZING 0x00000001

#endif

#if defined(NDIS_SUPPORT_NDIS650)
//
// The NIC is currently in suspended state.
//
#define NDIS_SWITCH_NIC_FLAGS_NIC_SUSPENDED    0x00000002

#endif

#if defined(NDIS_SUPPORT_NDIS660)
//
// Preferred mapped NIC for a vNIC has changed.
//
#define NDIS_SWITCH_NIC_FLAGS_MAPPED_NIC_UPDATED    0x00000004
#endif

#if defined(NDIS_SUPPORT_NDIS680)
//
// Preferred mapped NIC for a vNIC has changed.
//
#define NDIS_SWITCH_NIC_FLAGS_NIC_SUSPENDED_LM      0x00000010
#endif

//
// These structures are used in OID_SWITCH_NIC_ARRAY
//

typedef enum _NDIS_SWITCH_NIC_TYPE
{
    //
    // An External NIC
    //
    NdisSwitchNicTypeExternal      = 0,
    //
    // A Synthetic NIC in the VM
    //
    NdisSwitchNicTypeSynthetic     = 1,
    //
    // An Emulated NIC in the VM
    //
    NdisSwitchNicTypeEmulated      = 2,
    //
    // An Internal NIC on the Host partition
    //
    NdisSwitchNicTypeInternal      = 3
} NDIS_SWITCH_NIC_TYPE;


typedef enum _NDIS_SWITCH_NIC_STATE
{
    //
    // The NIC state when state is not known ahead of time.
    //
    NdisSwitchNicStateUnknown       = 0,

    //
    // The NIC state for NICs that have been created.
    //
    NdisSwitchNicStateCreated       = 1,

    //
    // The NIC state for NICs that have been connected.
    //
    NdisSwitchNicStateConnected     = 2,

    //
    // The NIC state for NICs that have been disconnected.
    //
    NdisSwitchNicStateDisconnected  = 3,

    //
    // The NIC state for NICs that have been delted.
    //
    NdisSwitchNicStateDeleted       = 4
} NDIS_SWITCH_NIC_STATE;


typedef struct _NDIS_SWITCH_NIC_PARAMETERS
{
    NDIS_OBJECT_HEADER              Header;
    //
    // Flags that can be used to pass certain additional info / state.
    //
    ULONG                           Flags;
    //
    // Nic identifier as used by management, such as WMI. For Nics of type
    // SwitchNicTypeExternal, this corresponds to the device InstanceId. For Nics
    // of type SwitchNicTypeInternal, this corresponds to an automatically
    // generated GUID.
    // For other Nics the value is set by the management layer.
    //
    NDIS_SWITCH_NIC_NAME            NicName;
    //
    // Nic friendly name. For Nics of type SwitchNicTypeExternal, this corresponds
    // to the device friendly name. For Nics of type SwitchNicTypeExternalInternal, this
    // corresponds to the device display name. For other Nics the value is set by the
    // management layer.
    //
    NDIS_SWITCH_NIC_FRIENDLYNAME    NicFriendlyName;
    //
    // ID of the associated Port
    //
    NDIS_SWITCH_PORT_ID             PortId;
    //
    // In teaming scenarios, this is the sub-interface index.
    // Zero otherwise.
    //
    NDIS_SWITCH_NIC_INDEX           NicIndex;
    //
    // The type of NIC
    //
    NDIS_SWITCH_NIC_TYPE            NicType;
    //
    // The current state of the NIC
    //
    NDIS_SWITCH_NIC_STATE           NicState;
    //
    // If NicType is SwitchNicTypeSynthetic or SwitchNicTypeEmulated,
    // this field is valid and contains the identifier of the
    // Virtual Machine owning the NIC
    //
    NDIS_VM_NAME                    VmName;
    //
    // VM Friendly Name
    //
    NDIS_VM_FRIENDLYNAME            VmFriendlyName;
    //
    // If NicType is SwitchNicTypeExternal or SwitchNicTypeInternal,
    // this field is valid and contains the NetCfgInstanceId of the
    // underlying interface.
    //
    GUID                            NetCfgInstanceId;
    //
    // MTU of the NIC
    //
    ULONG                           MTU;
    //
    // Preferred NUMA Node
    //
    USHORT                          NumaNodeId;
    //
    // Permanent MAC address of the NIC
    //
    UCHAR                           PermanentMacAddress[NDIS_MAX_PHYS_ADDRESS_LENGTH];
    //
    // MAC address of the NIC as set by the VM
    //
    UCHAR                           VMMacAddress[NDIS_MAX_PHYS_ADDRESS_LENGTH];
    //
    // Current MAC address of the NIC, to enforce policy against
    //
    UCHAR                           CurrentMacAddress[NDIS_MAX_PHYS_ADDRESS_LENGTH];
    //
    // If TRUE, the NIC is currently assigned to a VF on the physical adapter.
    //
    BOOLEAN                         VFAssigned;
#if defined(NDIS_SUPPORT_NDIS640)
    //
    // New fields available in NDIS_SWITCH_NIC_PARAMETERS_2
    //
    ULONG64                         NdisReserved[2];
#endif
} NDIS_SWITCH_NIC_PARAMETERS, *PNDIS_SWITCH_NIC_PARAMETERS;

#define NDIS_SWITCH_NIC_PARAMETERS_REVISION_1      1

#define NDIS_SIZEOF_NDIS_SWITCH_NIC_PARAMETERS_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_NIC_PARAMETERS, VFAssigned)

#if defined(NDIS_SUPPORT_NDIS640)
    #define NDIS_SWITCH_NIC_PARAMETERS_REVISION_2      2

    #define NDIS_SIZEOF_NDIS_SWITCH_NIC_PARAMETERS_REVISION_2 \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_NIC_PARAMETERS, NdisReserved)
#endif

typedef struct _NDIS_SWITCH_NIC_ARRAY
{
  NDIS_OBJECT_HEADER        Header;
  ULONG                     Flags;
  USHORT                    FirstElementOffset;
  ULONG                     NumElements;
  ULONG                     ElementSize;
} NDIS_SWITCH_NIC_ARRAY, *PNDIS_SWITCH_NIC_ARRAY;

#define NDIS_SWITCH_NIC_ARRAY_REVISION_1      1

#define NDIS_SIZEOF_NDIS_SWITCH_NIC_ARRAY_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_NIC_ARRAY, ElementSize)

#define NDIS_SWITCH_NIC_AT_ARRAY_INDEX(_NicArray_, _Index_)\
    ((PNDIS_SWITCH_NIC_PARAMETERS)((PUCHAR)(_NicArray_) + \
                        (_NicArray_)->FirstElementOffset + \
                        ((_NicArray_)->ElementSize * (_Index_))))

//
// Forward declaration of OID request pointer.
//
typedef struct _NDIS_OID_REQUEST NDIS_OID_REQUEST, *PNDIS_OID_REQUEST;

typedef struct _NDIS_SWITCH_NIC_OID_REQUEST
{
    NDIS_OBJECT_HEADER          Header;
    ULONG                       Flags;
    NDIS_SWITCH_PORT_ID         SourcePortId;
    NDIS_SWITCH_NIC_INDEX       SourceNicIndex;
    NDIS_SWITCH_PORT_ID         DestinationPortId;
    NDIS_SWITCH_NIC_INDEX       DestinationNicIndex;
    PNDIS_OID_REQUEST           OidRequest;
} NDIS_SWITCH_NIC_OID_REQUEST, *PNDIS_SWITCH_NIC_OID_REQUEST;

#define NDIS_SWITCH_NIC_OID_REQUEST_REVISION_1     1

#define NDIS_SIZEOF_NDIS_SWITCH_NIC_OID_REQUEST_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_NIC_OID_REQUEST, OidRequest)

//
// These structures are used in OID_SWITCH_NIC_SAVE and OID_SWITCH_NIC_RESTORE
//

typedef struct _NDIS_SWITCH_NIC_SAVE_STATE
{
  NDIS_OBJECT_HEADER                    Header;
  //
  // Reserved for any future structure flags.
  //
  ULONG                                 Flags;
  //
  // Port Identifier
  //
  NDIS_SWITCH_PORT_ID                   PortId;
  //
  // NIC Index
  //
  NDIS_SWITCH_NIC_INDEX                 NicIndex;
  //
  // Identifier for extension, used by extension on restore to recognize its data.
  //
  GUID                                  ExtensionId;
  //
  // Extension name, used for diagnostic purposes.
  //
  NDIS_SWITCH_EXTENSION_FRIENDLYNAME    ExtensionFriendlyName;
  //
  // Optional FeatureClass identifier to further recognize its data.
  //
  GUID                                  FeatureClassId;
  //
  // Size of data in SaveData buffer.
  //
  USHORT                                SaveDataSize;
  //
  // Offset to save data from start of this buffer.
  //
  USHORT                                SaveDataOffset;
#if (NDIS_SUPPORT_NDIS650)
  //
  // Size of data in SaveData buffer in addition to SaveDataSize
  //
  ULONG                                 SaveDataSizeOverflow;
#endif
} NDIS_SWITCH_NIC_SAVE_STATE, *PNDIS_SWITCH_NIC_SAVE_STATE;

#define NDIS_SWITCH_NIC_SAVE_STATE_REVISION_1      1

#define NDIS_SIZEOF_NDIS_SWITCH_NIC_SAVE_STATE_REVISION_1 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_NIC_SAVE_STATE,  SaveDataOffset)

#if (NDIS_SUPPORT_NDIS650)

#define NDIS_SWITCH_NIC_SAVE_STATE_REVISION_2      2

#define NDIS_SIZEOF_NDIS_SWITCH_NIC_SAVE_STATE_REVISION_2 \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_SWITCH_NIC_SAVE_STATE,  SaveDataSizeOverflow)

#endif

#endif // (NDIS_SUPPORT_NDIS630)

//
// NDIS_PORT_STATE is used in port state change status indications
// as well as OID_GEN_PORT_STATE query OID
//

#define NDIS_PORT_STATE_REVISION_1     1

typedef struct _NDIS_PORT_STATE
{
    NDIS_OBJECT_HEADER              Header;
    NDIS_MEDIA_CONNECT_STATE        MediaConnectState;
    ULONG64                         XmitLinkSpeed;
    ULONG64                         RcvLinkSpeed;
    NET_IF_DIRECTION_TYPE           Direction;
    NDIS_PORT_CONTROL_STATE         SendControlState;
    NDIS_PORT_CONTROL_STATE         RcvControlState;
    NDIS_PORT_AUTHORIZATION_STATE   SendAuthorizationState;
    NDIS_PORT_AUTHORIZATION_STATE   RcvAuthorizationState;
    ULONG                           Flags;
}NDIS_PORT_STATE, *PNDIS_PORT_STATE;

#define NDIS_SIZEOF_PORT_STATE_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_PORT_STATE, Flags)

//
// Flags used in NDIS_PORT_CHARACTERISTICS Flags field.
//
#define  NDIS_PORT_CHAR_USE_DEFAULT_AUTH_SETTINGS             0x00000001

//
// NDIS_PORT_CHARACTERISTICS is used in port allocation and
// arrival notification as part of NDIS_PORT structure
//
#define NDIS_PORT_CHARACTERISTICS_REVISION_1     1

typedef struct _NDIS_PORT_CHARACTERISTICS
{
    NDIS_OBJECT_HEADER                  Header;
    NDIS_PORT_NUMBER                    PortNumber;
    ULONG                               Flags;
    NDIS_PORT_TYPE                      Type;
    NDIS_MEDIA_CONNECT_STATE            MediaConnectState;
    ULONG64                             XmitLinkSpeed;
    ULONG64                             RcvLinkSpeed;
    NET_IF_DIRECTION_TYPE               Direction;
    NDIS_PORT_CONTROL_STATE             SendControlState;
    NDIS_PORT_CONTROL_STATE             RcvControlState;
    NDIS_PORT_AUTHORIZATION_STATE       SendAuthorizationState;
    NDIS_PORT_AUTHORIZATION_STATE       RcvAuthorizationState;
} NDIS_PORT_CHARACTERISTICS, *PNDIS_PORT_CHARACTERISTICS;

#define NDIS_SIZEOF_PORT_CHARACTERISTICS_REVISION_1    \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_PORT_CHARACTERISTICS, RcvAuthorizationState)

//
// NDIS_PORT structure is used in port activation
// PnP notification
//
typedef struct _NDIS_PORT NDIS_PORT, *PNDIS_PORT;

struct _NDIS_PORT
{
    PNDIS_PORT                  Next;
    PVOID                       NdisReserved;
    PVOID                       MiniportReserved;
    PVOID                       ProtocolReserved;
    NDIS_PORT_CHARACTERISTICS   PortCharacteristics;
};


#define NDIS_PORT_ARRAY_REVISION_1                1
//
// NDIS_PORT_ARRAY is used in enumerating the ports
//
typedef struct _NDIS_PORT_ARRAY
{
    NDIS_OBJECT_HEADER              Header;
    ULONG                           NumberOfPorts;
    ULONG                           OffsetFirstPort;
    ULONG                           ElementSize;
    NDIS_PORT_CHARACTERISTICS       Ports[1];
}NDIS_PORT_ARRAY, *PNDIS_PORT_ARRAY;

#define NDIS_SIZEOF_PORT_ARRAY_REVISION_1      \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_PORT_ARRAY, Ports)


#endif // ((NTDDI_VERSION >= NTDDI_VISTA) || NDIS_SUPPORT_NDIS6)

#endif // NTDDI_VERSION >= NTDDI_VISTA

#if (NDIS_SUPPORT_NDIS650)

#include <inaddr.h>
#include <in6addr.h>

//
// Data structures for NDIS Generic Filtering Platform (GFP) used in
// Generic Flow Table (GFT) offload and Packet Direct
//
#define ETHERNET_LENGTH_OF_ADDRESS  6

//
// NDIS_GFP_TABLE_TYPE enum value is used to define the type of table
// For GFT, at the minimum we have an ingress and egress exact match tables.
// we may or may not have an ingress or egress wildcard match table
//

typedef enum _NDIS_GFP_TABLE_TYPE
{
    NdisGfpTableTypeUndefined,
    NdisGfpTableTypeWildcardIngress,
    NdisGfpTableTypeWildcardEgress,
    NdisGfpTableTypeExactMatchIngress,
    NdisGfpTableTypeExactMatchEgress,
    NdisGfpTableTypePacketDirect,
    NdisGfpTableTypeMax
} NDIS_GFP_TABLE_TYPE, *PNDIS_GFP_TABLE_TYPE;


//
// flags used to specify which headers are present in a header group
//
// These flags are used in the HeadersPresent field of
// NDIS_GFP_HEADER_GROUP_EXACT_MATCH_PROFILE, NDIS_GFP_HEADER_GROUP_EXACT_MATCH,
// NDIS_GFT_HEADER_GROUP_TRANSPOSITION_PROFILE, NDIS_GFT_HEADER_GROUP_TRANSPOSITION,
// NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILE, and NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH
//
#define NDIS_GFP_HEADER_PRESENT_ETHERNET                0x00000001
#define NDIS_GFP_HEADER_PRESENT_IPV4                    0x00000002
#define NDIS_GFP_HEADER_PRESENT_IPV6                    0x00000004
#define NDIS_GFP_HEADER_PRESENT_TCP                     0x00000008
#define NDIS_GFP_HEADER_PRESENT_UDP                     0x00000010
#define NDIS_GFP_HEADER_PRESENT_ICMP                    0x00000020
#define NDIS_GFP_HEADER_PRESENT_NO_ENCAP                0x00000040
#define NDIS_GFP_HEADER_PRESENT_IP_IN_IP_ENCAP          0x00000080
#define NDIS_GFP_HEADER_PRESENT_IP_IN_GRE_ENCAP         0x00000100
#define NDIS_GFP_HEADER_PRESENT_NVGRE_ENCAP             0x00000200
#define NDIS_GFP_HEADER_PRESENT_VXLAN_ENCAP             0x00000400
#define NDIS_GFP_HEADER_PRESENT_ESP                     0x00000800

//
// flags used to specify fields in the Ethernet, IP, Transport and encapsulation headers
//
#define NDIS_GFP_HEADER_FIELD_DEST_MAC_ADDR             0x00000001ULL
#define NDIS_GFP_HEADER_FIELD_SRC_MAC_ADDR              0x00000002ULL
#define NDIS_GFP_HEADER_FIELD_ETH_TYPE                  0x00000004ULL
#define NDIS_GFP_HEADER_FIELD_CUSTOMER_VLAN_ID          0x00000008ULL
#define NDIS_GFP_HEADER_FIELD_PROVIDER_VLAN_ID          0x00000010ULL
#define NDIS_GFP_HEADER_FIELD_8021P_PRIORITY            0x00000020ULL
#define NDIS_GFP_HEADER_FIELD_SRC_IP_ADDR               0x00000040ULL
#define NDIS_GFP_HEADER_FIELD_DEST_IP_ADDR              0x00000080ULL
#define NDIS_GFP_HEADER_FIELD_TTL                       0x00000100ULL
#define NDIS_GFP_HEADER_FIELD_IP_PROTOCOL               0x00000200ULL
#define NDIS_GFP_HEADER_FIELD_IP_DSCP                   0x00000400ULL
#define NDIS_GFP_HEADER_FIELD_TRANSPORT_SRC_PORT        0x00000800ULL
#define NDIS_GFP_HEADER_FIELD_TRANSPORT_DEST_PORT       0x00001000ULL
#define NDIS_GFP_HEADER_FIELD_TCP_FLAGS                 0x00002000ULL
#define NDIS_GFP_HEADER_FIELD_TENANT_ID                 0x00004000ULL
#define NDIS_GFP_HEADER_FIELD_ENTROPY                   0x00008000ULL
#define NDIS_GFP_HEADER_FIELD_ICMP_TYPE                 0x00010000ULL
#define NDIS_GFP_HEADER_FIELD_ICMP_CODE                 0x00020000ULL
#define NDIS_GFP_HEADER_FIELD_OOB_VLAN                  0x00040000ULL
#define NDIS_GFP_HEADER_FIELD_OOB_TENANT_ID             0x00080000ULL
#define NDIS_GFP_HEADER_FIELD_GRE_PROTOCOL              0x00100000ULL


//
// flags used to specify encapsulation types, they are also used
// in SupportedEncapsulationTypes field of NDIS_GFT_OFFLOAD_CAPABILITIES structure
//
#define NDIS_GFP_ENCAPSULATION_TYPE_NOT_ENCAPSULATED    0x00000001
#define NDIS_GFP_ENCAPSULATION_TYPE_IP_IN_IP            0x00000002
#define NDIS_GFP_ENCAPSULATION_TYPE_IP_IN_GRE           0x00000004
#define NDIS_GFP_ENCAPSULATION_TYPE_NVGRE               0x00000008
#define NDIS_GFP_ENCAPSULATION_TYPE_VXLAN               0x00000010

typedef ULONG NDIS_GFP_PROFILE_ID, *PNDIS_GFP_PROFILE_ID;
#define NDIS_GFP_UNDEFINED_PROFILE_ID               0

//
// flags used in Flags field of NDIS_GFP_HEADER_GROUP_EXACT_MATCH_PROFILE
//
#define NDIS_GFP_HEADER_GROUP_EXACT_MATCH_PROFILE_IS_TTL_ONE    0x00000001

//
// NDIS_GFP_HEADER_GROUP_EXACT_MATCH_PROFILE defines a match profile for a
// -single header group-.
// An array of NDIS_GFP_HEADER_GROUP_EXACT_MATCH_PROFILEs are used in
// NDIS_GFP_EXACT_MATCH_PROFILE structure to define a full match profile
//

#define NDIS_GFP_HEADER_GROUP_EXACT_MATCH_PROFILE_REVISION_1    1

typedef struct DECLSPEC_ALIGN(8) _NDIS_GFP_HEADER_GROUP_EXACT_MATCH_PROFILE
{
    NDIS_OBJECT_HEADER                  Header;

    ULONG                               Flags;

    //
    // NDIS_GFP_HEADER_PRESENT_xxx flags to specify which headers
    // are present in the header group
    //
    ULONG                               HeadersPresent;

    //
    // NDIS_GFP_HEADER_FIELD_xxx flags except NDIS_GFP_HEADER_FIELD_ENTROPY
    // specify which fields of the network headers present in the header group
    // we should match against.
    //
    ULONG64                             MatchFields;

} NDIS_GFP_HEADER_GROUP_EXACT_MATCH_PROFILE, *PNDIS_GFP_HEADER_GROUP_EXACT_MATCH_PROFILE;

#define NDIS_SIZEOF_GFP_HEADER_GROUP_EXACT_MATCH_PROFILE_REVISION_1    \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFP_HEADER_GROUP_EXACT_MATCH_PROFILE, MatchFields)



//
// flags used in Flags field of NDIS_GFP_EXACT_MATCH_PROFILE
//
#define NDIS_GFP_EXACT_MATCH_PROFILE_RDMA_FLOW      0x00000001

//
// NDIS_GFP_EXACT_MATCH_PROFILE is used in OID_GFT_EXACT_MATCH_PROFILE
// and NDIS_GFT_PROFILE_INFO.
//
// This structure is used to fully define a match profile that can span multiple header groups
//

#define NDIS_GFP_EXACT_MATCH_PROFILE_REVISION_1         1

typedef struct DECLSPEC_ALIGN(8) _NDIS_GFP_EXACT_MATCH_PROFILE
{
    _In_ NDIS_OBJECT_HEADER                 Header;
    _In_ ULONG                              Flags;
    _In_ NDIS_GFP_TABLE_TYPE                TableType;

    _In_ NDIS_GFP_PROFILE_ID                ProfileId;

    _Out_ ULONG                             NumSupportedEntries;


    //
    // the following fields are used to describe an array of NDIS_GFP_HEADER_GROUP_EXACT_MATCH_PROFILE structures
    //
    _In_ ULONG                              HeaderGroupExactMatchProfileArrayOffset; //  from the beginning of this structure
    _In_ ULONG                              HeaderGroupExactMatchProfileArrayNumElements;
    _In_ ULONG                              HeaderGroupExactMatchProfileArrayElementSize;

} NDIS_GFP_EXACT_MATCH_PROFILE, *PNDIS_GFP_EXACT_MATCH_PROFILE;

#define NDIS_SIZEOF_GFP_EXACT_MATCH_PROFILE_REVISION_1    \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFP_EXACT_MATCH_PROFILE, HeaderGroupExactMatchProfileArrayElementSize)

typedef struct _NDIS_GFP_ETHERNET_HEADER_FIELDS
{
    UINT8                               DestinationMacAddress[ETHERNET_LENGTH_OF_ADDRESS];
    UINT8                               SourceMacAddress[ETHERNET_LENGTH_OF_ADDRESS];
    UINT16                              EthType;
    UINT16                              CustomerVlanId;
    UINT16                              ProviderVlanId;
    UINT8                               Priority;
} NDIS_GFP_ETHERNET_HEADER_FIELDS, *PNDIS_GFP_ETHERNET_HEADER_FIELDS;


//
// flags used in Flags field of NDIS_GFP_HEADER_GROUP_EXACT_MATCH
//
#define NDIS_GFP_HEADER_GROUP_EXACT_MATCH_IS_TTL_ONE        0x00000001

//
// NDIS_GFP_HEADER_GROUP_EXACT_MATCH structure is used with
// NDIS_GFT_EXACT_MATCH_FLOW_ENTRY. This structure defines
// the exact match for a single header group
//
#define NDIS_GFP_HEADER_GROUP_EXACT_MATCH_REVISION_1       1

typedef struct DECLSPEC_ALIGN(8) _NDIS_GFP_HEADER_GROUP_EXACT_MATCH
{

    NDIS_OBJECT_HEADER                  Header;
    ULONG                               Flags;

    //
    // NDIS_GFP_HEADER_PRESENT_xxx
    //
    ULONG                               HeadersPresent;

    //
    // NDIS_GFP_HEADER_FIELD_xxx except NDIS_GFP_HEADER_FIELD_ENTROPY
    //
    ULONG64                             MatchFields;

    //
    // Ethernet header
    //
    NDIS_GFP_ETHERNET_HEADER_FIELDS     EthernetFields;

    //
    // IP header
    //
    union
    {
        struct
        {
            IN_ADDR                     SourceIPAddress;
            IN_ADDR                     DestinationIPAddress;
        } IPv4Address;

        struct
        {
            IN6_ADDR                    SourceIPAddress;
            IN6_ADDR                    DestinationIPAddress;
        } IPv6Address;

    } IPAddress;

    UINT8                               Dscp;
    UINT8                               IPProtocol;

    //
    // transport/encapsulation header
    //
    union
    {
        struct
        {
            USHORT                      SourcePort;
            USHORT                      DestinationPort;
        } Udp;

        struct
        {
            USHORT                      SourcePort;
            USHORT                      DestinationPort;
            UINT8                       TcpFlags;
        } Tcp;

        struct
        {
            UINT8                       Type;
            UINT8                       Code;
        } Icmp;

        struct
        {
            ULONG                       TenantId;
            USHORT                      GreProtocol;
        } Encapsulation;
    } TransportOrEncapsulation;

} NDIS_GFP_HEADER_GROUP_EXACT_MATCH, *PNDIS_GFP_HEADER_GROUP_EXACT_MATCH;

#define NDIS_SIZEOF_GFP_HEADER_GROUP_EXACT_MATCH_REVISION_1    \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFP_HEADER_GROUP_EXACT_MATCH, TransportOrEncapsulation)


//
// wildcard GFP entries
//

//
// NDIS_GFP_WILDCARD_MATCH_TYPE enum value specifies the type of matches that
// we do in a wildcard match
//
typedef enum _NDIS_GFP_WILDCARD_MATCH_TYPE
{
    NdisGfpWildcardMatchTypeUndefined,
    NdisGfpWildcardMatchTypeEqual,
    NdisGfpWildcardMatchTypeMaskEqual,
    NdisGfpWildcardMatchTypeInRange,
    NdisGfpWildcardMatchTypeMax
} NDIS_GFP_WILDCARD_MATCH_TYPE, *PNDIS_GFP_WILDCARD_MATCH_TYPE;

//
// flags used in Flags field of NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILE
//
#define NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILE_IS_TTL_ONE    0x00000001

//
// NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILE structure is used with
// NDIS_GFP_WILDCARD_MATCH_PROFILE. This structure defines a wildcard
// match profile for a single header group
//

#define NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILE_REVISION_1            1

typedef struct DECLSPEC_ALIGN(8) _NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILE
{
    NDIS_OBJECT_HEADER                  Header;
    ULONG                               Flags;

    //
    // NDIS_GFP_HEADER_PRESENT_xxx
    //
    ULONG                               HeadersPresent;

    //
    // NDIS_GFP_HEADER_FIELD_xxx
    //
    ULONG64                             MatchFields;

    NDIS_GFP_WILDCARD_MATCH_TYPE        SourceIPv4AddressMatchType;
    NDIS_GFP_WILDCARD_MATCH_TYPE        DestinationIPv4AddressMatchType;
    NDIS_GFP_WILDCARD_MATCH_TYPE        SourceIPv6AddressMatchType;
    NDIS_GFP_WILDCARD_MATCH_TYPE        DestinationIPv6AddressMatchType;

    NDIS_GFP_WILDCARD_MATCH_TYPE        SourcePortMatchType;
    NDIS_GFP_WILDCARD_MATCH_TYPE        DestinationPortMatchType;

    UINT8                               TcpFlags;
} NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILE, *PNDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILE;

#define NDIS_SIZEOF_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILE_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILE, TcpFlags)


//
// NDIS_GFP_WILDCARD_MATCH_PROFILE structure is used with OID_GFT_WILDCARD_MATCH_PROFILE
// and NDIS_GFT_WILDCARD_MATCH_FLOW_ENTRY.
//
// It uses an array of NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILEs in order to describe
// a full wildcard match profile
//

#define NDIS_GFP_WILDCARD_MATCH_PROFILE_REVISION_1            1

typedef struct DECLSPEC_ALIGN(8) _NDIS_GFP_WILDCARD_MATCH_PROFILE
{
    _In_ NDIS_OBJECT_HEADER                 Header;
    _In_ ULONG                              Flags;

    _In_ NDIS_GFP_TABLE_TYPE                TableType;

    _In_ NDIS_GFP_PROFILE_ID                ProfileId;

    _Out_ ULONG                             NumSupportedEntries;

    //
    // The following fields are used to describe an array of NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILE structures
    //
    _In_ ULONG                              HeaderGroupWildcardMatchProfileArrayOffset;        // from the beginning of this structure
    _In_ ULONG                              HeaderGroupWildcardMatchProfileArrayNumElements;
    _In_ ULONG                              HeaderGroupWildcardMatchProfileArrayElementSize;
} NDIS_GFP_WILDCARD_MATCH_PROFILE, *PNDIS_GFP_WILDCARD_MATCH_PROFILE;

#define NDIS_SIZEOF_GFP_WILDCARD_MATCH_PROFILE_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFP_WILDCARD_MATCH_PROFILE, HeaderGroupWildcardMatchProfileArrayElementSize)

//
// structures used in NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH
//
typedef struct _NDIS_GFP_IPV4_ADDRESS_WILDCARD_MATCH
{
    NDIS_GFP_WILDCARD_MATCH_TYPE    MatchType;
    IN_ADDR                         IPAddress;
    union
    {
        ULONG   PrefixLength;
        ULONG   RangeSize;
    } MatchValue;

} NDIS_GFP_IPV4_ADDRESS_WILDCARD_MATCH, *PNDIS_GFP_IPV4_ADDRESS_WILDCARD_MATCH;

typedef struct _NDIS_GFP_IPV4_SRC_AND_DEST_ADDRESS_WILDCARD_MATCH
{
    NDIS_GFP_IPV4_ADDRESS_WILDCARD_MATCH    SourceIPv4Address;
    NDIS_GFP_IPV4_ADDRESS_WILDCARD_MATCH    DestinationIPv4Address;
} NDIS_GFP_IPV4_SRC_AND_DEST_ADDRESS_WILDCARD_MATCH, *PNDIS_GFP_IPV4_SRC_AND_DEST_ADDRESS_WILDCARD_MATCH;


typedef struct _NDIS_GFP_IPV6_ADDRESS_WILDCARD_MATCH
{
    NDIS_GFP_WILDCARD_MATCH_TYPE    MatchType;
    IN6_ADDR                        IPAddress;
    union
    {
        ULONG   PrefixLength;
        ULONG   RangeSize;
    } MatchValue;

} NDIS_GFP_IPV6_ADDRESS_WILDCARD_MATCH, *PNDIS_GFP_IPV6_ADDRESS_WILDCARD_MATCH;

typedef struct _NDIS_GFP_IPV6_SRC_AND_DEST_ADDRESS_WILDCARD_MATCH
{
    NDIS_GFP_IPV6_ADDRESS_WILDCARD_MATCH    SourceIPv6Address;
    NDIS_GFP_IPV6_ADDRESS_WILDCARD_MATCH    DestinationIPv6Address;
} NDIS_GFP_IPV6_SRC_AND_DEST_ADDRESS_WILDCARD_MATCH, *PNDIS_GFP_IPV6_SRC_AND_DEST_ADDRESS_WILDCARD_MATCH;

typedef union _NDIS_GFP_IP_SRC_AND_DEST_ADDRESS_WILDCARD_MATCH
{
    NDIS_GFP_IPV4_SRC_AND_DEST_ADDRESS_WILDCARD_MATCH IPv4SrcAndDestAddress;
    NDIS_GFP_IPV6_SRC_AND_DEST_ADDRESS_WILDCARD_MATCH IPv6SrcAndDestAddress;
} NDIS_GFP_IP_SRC_AND_DEST_ADDRESS_WILDCARD_MATCH, *PNDIS_GFP_IP_SRC_AND_DEST_ADDRESS_WILDCARD_MATCH;

typedef struct _NDIS_GFP_TRANSPORT_PORT_WILDCARD_MATCH
{
    NDIS_GFP_WILDCARD_MATCH_TYPE    MatchType;
    USHORT                          Port;
    USHORT                          Range;
} NDIS_GFP_TRANSPORT_PORT_WILDCARD_MATCH, *PNDIS_GFP_TRANSPORT_PORT_WILDCARD_MATCH;

typedef struct _NDIS_GFP_TRANSPORT_SRC_AND_DEST_PORT_WILDCARD_MATCH
{
    NDIS_GFP_TRANSPORT_PORT_WILDCARD_MATCH  SourcePort;
    NDIS_GFP_TRANSPORT_PORT_WILDCARD_MATCH  DestinationPort;
} NDIS_GFP_TRANSPORT_SRC_AND_DEST_PORT_WILDCARD_MATCH, *PNDIS_GFP_TRANSPORT_SRC_AND_DEST_PORT_WILDCARD_MATCH;

//
// flags used in FLags field of NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH
//
#define NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_IS_TTL_ONE        0x00000001

//
// NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH structure is used in NDIS_GFT_WILDCARD_MATCH_FLOW_ENTRY
//
#define NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_REVISION_1            1

typedef struct DECLSPEC_ALIGN(8) _NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH
{
    NDIS_OBJECT_HEADER                  Header;
    ULONG                               Flags;

    NDIS_GFP_PROFILE_ID                 ProfileId;


    //
    // NDIS_GFP_HEADER_PRESENT_xxx
    //
    ULONG                               HeadersPresent;

    //
    // NDIS_GFP_HEADER_FIELD_xxx
    //
    ULONG64                             MatchFields;

    //
    // Ethernet header
    //
    NDIS_GFP_ETHERNET_HEADER_FIELDS     EthernetFields;

    //
    // IP header
    //
    NDIS_GFP_IP_SRC_AND_DEST_ADDRESS_WILDCARD_MATCH IPSrcAndDestAddress;

    UINT8                               Dscp;
    UINT8                               IPProtocol;

    //
    // transport/encapsulation header
    //
    union
    {

        struct
        {
            NDIS_GFP_TRANSPORT_SRC_AND_DEST_PORT_WILDCARD_MATCH UdpSrcAndDestPort;
        } UdpHeader;

        struct
        {
            NDIS_GFP_TRANSPORT_SRC_AND_DEST_PORT_WILDCARD_MATCH TcpSrcAndDestPort;

            //
            // implied "<and> != 0"
            //
            UINT8                       TcpFlags;

        } TcpHeader;

        struct
        {
            UINT8                       Type;
            UINT8                       Code;
        } IcmpHeader;

        struct
        {
            ULONG                       TenantId;
            USHORT                      GreProtocol;
        } Encapsulation;

    } TransportOrEncapsulation;

} NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH, *PNDIS_GFP_HEADER_GROUP_WILDCARD_MATCH;

#define NDIS_SIZEOF_GFP_HEADER_GROUP_WILDCARD_MATCH_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH, TransportOrEncapsulation)


//
// Packet Direct data structures
//

//
// NDIS PACKETDIRECT Capability/Configuration advertisement and discovery
// definitions.
//

#define NDIS_PD_QUEUE_COUNT_DYNAMIC ((ULONG)-1)

//
// NDIS_PD_CAPABILITIES.CapabilityFlags definitions
//

//
// Indicates that the PD provider supports creation of counters
// with the PDCounterTypeReceiveFilter type.
//
#define NDIS_PD_CAPS_RECEIVE_FILTER_COUNTERS_SUPPORTED              0x00000001

//
// Indicates that the PD provider allows PD clients to request a notification
// to be delivered on a per NDIS_PD_QUEUE basis for PD_BUFFER completions.
//
#define NDIS_PD_CAPS_DRAIN_NOTIFICATIONS_SUPPORTED                  0x00000002

//
// Indicates that the PD provider supports moderating drain notifications
// based on a moderation interval value per PD queue.
// This flag can be set only if NDIS_PD_CAPS_DRAIN_NOTIFICATIONS_SUPPORTED
// is set.
//
#define NDIS_PD_CAPS_NOTIFICATION_MODERATION_INTERVAL_SUPPORTED     0x00000004

//
// Indicates that the PD provider supports moderating drain notifications
// based on a moderation count value per PD queue.
// This flag can be set only if NDIS_PD_CAPS_NOTIFICATION_MODERATION_INTERVAL_SUPPORTED
// is set (i.e., there's no moderation support based on purely count).
//
#define NDIS_PD_CAPS_NOTIFICATION_MODERATION_COUNT_SUPPORTED        0x00000008


//
// NDIS_PD_CAPABILITIES structure indicates PD capabilities for
// a miniport adapter.
//
typedef struct DECLSPEC_ALIGN(8) _NDIS_PD_CAPABILITIES {
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_PD_CAPABILITIES_REVISION_1;
    // Header.Size >= NDIS_SIZEOF_PD_CAPABILITIES_REVISION_1;
    //
    NDIS_OBJECT_HEADER Header;
    ULONG Flags; // Reserved. Must be set to 0.

    //
    // Maximum partial PD_BUFFER chain lengths supported by the provider
    // for transmit and receive.
    //
    ULONG MaximumTxPartialBufferCount;
    ULONG MaximumRxPartialBufferCount;

    //
    // When creating a packet match filter, PD client can specify a filter
    // context value to be returned (via the PD_BUFFER.RxFilterContext field)
    // for each packet that matches the filter. PD allows the width of this
    // context value to be as large as 64 bits, however not every PD provider
    // may be able to support a 64-bit wide context value. PD providers report
    // the filter context width that they support via the field below in
    // number of bits.
    //
    UCHAR RxFilterContextWidth;

    ULONG CapabilityFlags;

    //
    // Max number of Rx or Tx queues supported. These reflect the max number
    // of queues that can be created via the NdisPDAllocateQueue function (hence
    // not including any existing RSS receive queues for which PD client can
    // ask for PD-mode access) assuming the queues are NOT used for another
    // purpose (e.g., VMQ). So, the max number is NOT a guarantee to the client.
    // A PD provider must NEVER report 0 for any Tx queue limit. However, it's
    // possible for a PD provider to report 0 for a Rx queue limit if it only
    // supports existing RSS queues to be put into PD-mode (i.e., PD client
    // cannot create any PD Rx queues via NdisPDAllocateQueue, but can only
    // request PD-mode access to existing RSS queues via
    // NdisPDAcquireReceiveQueues).
    // If a PD provider has a flexible implementation where it supports a
    // maximum of N queues where any queue can be purposed flexibly as either
    // an RX or TX queue, the PD provider should advertise ~N/2 for
    // MaxNumberOfRxQueues*** and ~N/2 MaxNumberOfTxQueues*** (because PD
    // clients typically create a matching number of RX and TX queues).
    // If a PD provider can partition Rx or TX queues flexibly between
    // Vports, then it can advertise NDIS_PD_QUEUE_COUNT_DYNAMIC for the
    // Vport-specific max counts.
    //
    ULONG MaxNumberOfRxQueues; // miniport-adapter-wide total maximum
    ULONG MaxNumberOfTxQueues; // miniport-adapter-wide total maximum
    ULONG MaxNumberOfRxQueuesForDefaultVport;
    ULONG MaxNumberOfTxQueuesForDefaultVport;
    ULONG MaxNumberOfRxQueuesPerNonDefaultVport;
    ULONG MaxNumberOfTxQueuesPerNonDefaultVport;

    //
    // The following fields are used to describe an array of
    // NDIS_GFP_EXACT_MATCH_PROFILE structures
    //
    ULONG ExactMatchProfileArrayOffset; // from the beginning of this struct
    ULONG ExactMatchProfileArrayNumElements;
    ULONG ExactMatchProfileArrayElementSize;

    //
    // The following fields are used to describe an array of
    // NDIS_GFP_WILDCARD_MATCH_PROFILE structures
    //
    ULONG WildcardMatchProfileArrayOffset; // from the beginning of this struct
    ULONG WildcardMatchProfileArrayNumElements;
    ULONG WildcardMatchProfileArrayElementSize;

    //
    // Providers that support interrupt moderation advertise their
    // ModerationInterval limits and granularity via the following fields.
    // All values are in nanoseconds. MinimumModerationInterval and
    // MaximumModerationInterval values must be exact multiple of the
    // ModerationIntervalGranularity value (which must be >= 1). Example:
    // a min value of 200, a max value of 1000, and a granularity value of
    // 100 means the provider can support only the following values for
    // the ModerationInterval: 200, 300, 400, 500, 600, 700, 800, 900, 1000.
    // Note that 0 is always a valid ModerationInterval value and must be
    // honored by the PD provider to disable interrupt moderation on the queue.
    // For any other ModerationInterval value that does not match any of the
    // supported granularity values, PD provider must round-down the input
    // ModerationInterval value to the nearest supported value.
    //
    ULONG MinimumModerationInterval;
    ULONG MaximumModerationInterval;
    ULONG ModerationIntervalGranularity;

    //
    // Maximum Rx and Tx queue sizes supported by the provider.
    // 0 means the provider does not have a hard limit.
    //
    ULONG MaxRxQueueSize;
    ULONG MaxTxQueueSize;
} NDIS_PD_CAPABILITIES;

#define NDIS_PD_CAPABILITIES_REVISION_1 1
#define NDIS_SIZEOF_PD_CAPABILITIES_REVISION_1 \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_PD_CAPABILITIES, \
        MaxTxQueueSize)

//
// The following structure is used by PD provider to make an NDIS status
// indication, NDIS_STATUS_PD_CURRENT_CONFIG, for conveying the
// PD status and capabilities up to NDIS. PD client drivers can also use
// OID_PD_QUERY_CURRENT_CONFIG to query the current PD status and capabilities.
// Both NDIS_STATUS_PD_CURRENT_CONFIG and OID_PD_QUERY_CURRENT_CONFIG are
// Vport-aware. However, in the current PD version, the only valid Vport
// ID that can be used with this status indication and OID is
// NDIS_INVALID_VPORT_ID, i.e., the scope and granularity of PD status and
// capability advertisement is the whole PF on the miniport adapter (in contrast
// to each individual Vport of the PF).
//


typedef struct DECLSPEC_ALIGN(8) _NDIS_PD_CONFIG {
    //
    // Header.Type = NDIS_OBJECT_TYPE_DEFAULT;
    // Header.Revision = NDIS_PD_CONFIG_REVISION_1;
    // Header.Size >= NDIS_SIZEOF_PD_CONFIG_REVISION_1;
    //
    NDIS_OBJECT_HEADER Header;
    ULONG Flags;
    BOOLEAN Enabled;
    //
    // A NDIS_PD_CAPABILITIES struct is at CapabilitiesOffset.
    // CapabilitiesSize == 0 means no PD capability exists currently.
    //
    ULONG CapabilitiesOffset;
    ULONG CapabilitiesSize;
} NDIS_PD_CONFIG;

#define NDIS_PD_CONFIG_REVISION_1 1
#define NDIS_SIZEOF_PD_CONFIG_REVISION_1 \
        RTL_SIZEOF_THROUGH_FIELD(NDIS_PD_CONFIG, CapabilitiesSize)

//
// NDIS miniport driver package adds this NDIS keyword in its INF file
// for devices that support PacketDirect. Mere presence of this keyword
// indicates PacketDirect capability (independent of its value). Value 0
// means PD functionality of the miniport adapter is disabled. Any non-zero
// value means PD functionality of the miniport adapter is enabled.
//
#define PACKET_DIRECT_KEYWORD NDIS_STRING_CONST("*PacketDirect")


//
// GFT flow entry ID
//
typedef ULONG_PTR NDIS_GFT_FLOW_ENTRY_ID, *PNDIS_GFT_FLOW_ENTRY_ID;
#define NDIS_GFT_UNDEFINED_FLOW_ENTRY_ID    0

//
// GFT table data structures
//
typedef ULONG NDIS_GFT_TABLE_ID, *PNDIS_GFT_TABLE_ID;
#define NDIS_GFT_UNDEFINED_TABLE_ID         0


//
// flags used in Flags field of NDIS_GFT_TABLE_PARAMETERS
//
#define NDIS_GFT_TABLE_INCLUDE_EXTERNAL_VPPORT      0x00000001

//
// NDIS_GFT_TABLE_PARAMETERS is used in OID_GFT_CREATE_TABLE
//
#define NDIS_GFT_TABLE_PARAMETERS_REVISION_1            1

typedef struct _NDIS_GFT_TABLE_PARAMETERS
{
    NDIS_OBJECT_HEADER                 Header;
    ULONG                              Flags;
    NDIS_GFT_TABLE_ID                  TableId;
    NDIS_GFP_TABLE_TYPE                TableType;
    NDIS_GFT_TABLE_ID                  NextTableId;
} NDIS_GFT_TABLE_PARAMETERS, *PNDIS_GFT_TABLE_PARAMETERS;

#define NDIS_SIZEOF_GFT_TABLE_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_TABLE_PARAMETERS, NextTableId)

//
// NDIS_GFT_TABLE_INFO is used with NDIS_GFT_TABLE_INFO_ARRAY
// in OID_GFT_ENUM_TABLES that enumerates GFT tables
// on a miniport adapter
//
#define NDIS_GFT_TABLE_INFO_REVISION_1              1

typedef struct _NDIS_GFT_TABLE_INFO
{
    NDIS_OBJECT_HEADER              Header;
    ULONG                           Flags;

    NDIS_GFT_TABLE_ID               TableId;
    NDIS_GFP_TABLE_TYPE             TableType;
    NDIS_GFT_TABLE_ID               NextTableId;
    ULONG                           NumFlowEntries;
} NDIS_GFT_TABLE_INFO, *PNDIS_GFT_TABLE_INFO;

#define NDIS_SIZEOF_GFT_TABLE_INFO_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_TABLE_INFO, NumFlowEntries)

//
// NDIS_GFT_TABLE_INFO_ARRAY is used in OID_GFT_ENUM_TABLES
//
#define NDIS_GFT_TABLE_INFO_ARRAY_REVISION_1        1

typedef struct _NDIS_GFT_TABLE_INFO_ARRAY
{
    _In_ NDIS_OBJECT_HEADER                 Header;
    _In_ ULONG                              Flags;

    //
    // The following fields are used to describe an array of NDIS_GFT_TABLE_INFO structures
    //
    _Out_ ULONG                             TableArrayOffset; //  from the beginning of this structure
    _Out_ ULONG                             TableArrayNumElements;
    _Out_ ULONG                             TableArrayElementSize;
} NDIS_GFT_TABLE_INFO_ARRAY, *PNDIS_GFT_TABLE_INFO_ARRAY;

#define NDIS_SIZEOF_GFT_TABLE_INFO_ARRAY_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_TABLE_INFO_ARRAY, TableArrayElementSize)

//
// NDIS_GFT_DELETE_TABLE_PARAMETERS is used in
// OID_GFT_DELETE_TABLE
//
#define NDIS_GFT_DELETE_TABLE_PARAMETERS_REVISION_1       1

typedef struct _NDIS_GFT_DELETE_TABLE_PARAMETERS
{
    _In_ NDIS_OBJECT_HEADER                 Header;
    _In_ ULONG                              Flags;
    _In_ NDIS_GFT_TABLE_ID                  TableId;
} NDIS_GFT_DELETE_TABLE_PARAMETERS, *PNDIS_GFT_DELETE_TABLE_PARAMETERS;

#define NDIS_SIZEOF_GFT_DELETE_TABLE_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_DELETE_TABLE_PARAMETERS, TableId)

//
// GFT counter structures
//

typedef ULONG NDIS_GFT_COUNTER_ID, *PNDIS_GFT_COUNTER_ID;

#define NDIS_GFT_UNDEFINED_COUNTER_ID       0


//
// maximum number of counter objects associated with a flow entry
//
#define NDIS_GFT_MAX_COUNTER_OBJECTS_PER_FLOW_ENTRY     8

//
// NDIS_GFT_COUNTER_UPDATE_FREQUENCY is used with NDIS_GFT_COUNTER in
// OID_GFT_ALLOCATE_COUNTERS. This enum is also used in NDIS_GFT_EXACT_MATCH_FLOW_ENTRY and
// NDIS_GFT_WILDCARD_MATCH_FLOW_ENTRY structures used in OID_GFT_ADD_FLOW_ENTRIES
//
typedef enum _NDIS_GFT_COUNTER_UPDATE_FREQUENCY
{
    NdisGftCounterUpdateFrequencyUndefined,
    NdisGftCounterUpdateFrequencyOnQuery,
    NdisGftCounterUpdateFrequencyPerUpdatePeriod,
    NdisGftCounterUpdateFrequencyPerPacket,
    NdisGftCounterUpdateFrequencyMax
} NDIS_GFT_COUNTER_UPDATE_FREQUENCY, *PNDIS_GFT_COUNTER_UPDATE_FREQUENCY;

//
// NDIS_GFT_COUNTER_TYPE enum value defines the type of the GFT counter
//
typedef enum _NDIS_GFT_COUNTER_TYPE
{
    NdisGftCounterTypeUndefined,
    NdisGftCounterTypePacket,
    NdisGftCounterTypeByte,
    NdisGftCounterTypePacketByte,
    NdisGftCounterTypePacketByteAndState,
    NdisGftCounterTypeMax
} NDIS_GFT_COUNTER_TYPE, *PNDIS_GFT_COUNTER_TYPE;

typedef volatile struct _NDIS_GFT_PACKET_COUNTER_VALUE
{
    ULONG64                         Packets;
    LARGE_INTEGER                   LastUpdate; // Absolute system time returned from KeQuerySystemTime
} NDIS_GFT_PACKET_COUNTER_VALUE, *PNDIS_GFT_PACKET_COUNTER_VALUE;

typedef volatile struct _NDIS_GFT_BYTE_COUNTER_VALUE
{
    ULONG64                         Bytes;
    LARGE_INTEGER                   LastUpdate; // Absolute system time returned from KeQuerySystemTime
} NDIS_GFT_BYTE_COUNTER_VALUE, *PNDIS_GFT_BYTE_COUNTER_VALUE;


typedef volatile struct _NDIS_GFT_PACKET_BYTE_COUNTER_VALUE
{
    ULONG64                         Packets;
    ULONG64                         Bytes;
    LARGE_INTEGER                   LastUpdate; // Absolute system time returned from KeQuerySystemTime
} NDIS_GFT_PACKET_BYTE_COUNTER_VALUE, *PNDIS_GFT_PACKET_BYTE_COUNTER_VALUE;

typedef volatile struct _NDIS_GFT_FLOW_STATE
{
    ULONG                           TcpSeqNum;
    ULONG                           TcpAckNum;
} NDIS_GFT_FLOW_STATE, *PNDIS_GFT_FLOW_STATE;

typedef volatile struct _NDIS_GFT_PACKET_BYTE_COUNTER_VALUE_AND_STATE
{
    NDIS_GFT_PACKET_BYTE_COUNTER_VALUE      PacketByteCounterValue;
    NDIS_GFT_FLOW_STATE                     FlowState;
} NDIS_GFT_PACKET_BYTE_COUNTER_VALUE_AND_STATE, *PNDIS_GFT_PACKET_BYTE_COUNTER_VALUE_AND_STATE;

//
// NDIS_GFT_COUNTER_PARAMETERS is used in OID_GFT_ALLOCATE_COUNTERS
//

#define NDIS_GFT_COUNTER_PARAMETERS_CLIENT_SPECIFIED_ADDRESS    0x00000001

#define NDIS_GFT_COUNTER_PARAMETERS_REVISION_1       1

typedef struct _NDIS_GFT_COUNTER_PARAMETERS
{
    _In_ NDIS_OBJECT_HEADER                 Header;
    _In_ ULONG                              Flags;

    _In_ NDIS_GFT_TABLE_ID                  TableId;
    _In_ NDIS_GFT_COUNTER_UPDATE_FREQUENCY  CounterUpdateFrequency;
    _In_ NDIS_GFT_COUNTER_TYPE              CounterType;
    _Inout_ NDIS_GFT_COUNTER_ID             CounterIdStart;
    _In_ ULONG                              NumCounters;
    _In_ ULONG                              UpdatePeriod; // in milliseconds

    union
    {
        _Inout_ PNDIS_GFT_PACKET_COUNTER_VALUE                  PacketCounters;
        _Inout_ PNDIS_GFT_BYTE_COUNTER_VALUE                    ByteCounters;
        _Inout_ PNDIS_GFT_PACKET_BYTE_COUNTER_VALUE             PacketByteCounters;
        _Inout_ PNDIS_GFT_PACKET_BYTE_COUNTER_VALUE_AND_STATE   PacketByteCountersAndState;
    } CounterValuesBufferStart;

} NDIS_GFT_COUNTER_PARAMETERS, *PNDIS_GFT_COUNTER_PARAMETERS;

#define NDIS_SIZEOF_GFT_COUNTER_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_COUNTER_PARAMETERS, CounterValuesBufferStart)


//
// NDIS_GFT_FREE_COUNTER_PARAMETERS is used in OID_GFT_FREE_COUNTERS
//
#define NDIS_GFT_FREE_COUNTER_PARAMETERS_REVISION_1       1

typedef struct _NDIS_GFT_FREE_COUNTER_PARAMETERS
{
    _In_ NDIS_OBJECT_HEADER                 Header;
    _In_ ULONG                              Flags;

    _In_ NDIS_GFT_TABLE_ID                  TableId;
    _In_ NDIS_GFT_COUNTER_ID                CounterIdStart;
    _In_ ULONG                              NumCounters;
    //
    // Last counter values
    // The following fields are used to describe an array of NDIS_GFT_COUNTER_VALUE
    //
    _Out_ ULONG                             LastCounterValueAndStateArrayOffset; //  from the beginning of this structure
    _Out_ ULONG                             LastCounterValueAndStateArrayNumElements;
    _Out_ ULONG                             LastCounterValueAndStateArrayElementSize;
} NDIS_GFT_FREE_COUNTER_PARAMETERS, *PNDIS_GFT_FREE_COUNTER_PARAMETERS;

#define NDIS_SIZEOF_GFT_FREE_COUNTER_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_FREE_COUNTER_PARAMETERS, LastCounterValueAndStateArrayElementSize)


//
// NDIS_GFT_COUNTER_INFO is used with NDIS_GFT_COUNTER_INFO_ARRAY
// in OID_GFT_ENUM_COUNTERS. OID_GFT_ENUM_COUNTERS enumerates the
// allocated counters (types, frequency, etc.) and not their values
//
#define NDIS_GFT_COUNTER_INFO_REVISION_1       1

typedef struct _NDIS_GFT_COUNTER_INFO
{
    _In_ NDIS_OBJECT_HEADER                 Header;
    _In_ ULONG                              Flags;

    _Out_ NDIS_GFT_COUNTER_ID               CounterIdStart;
    _Out_ ULONG                             NumCounters;
    _Out_ NDIS_GFT_COUNTER_UPDATE_FREQUENCY CounterUpdateFrequency;
    _Out_ NDIS_GFT_COUNTER_TYPE             CounterType;
    _Out_ ULONG                             UpdatePeriod; // in milliseconds
} NDIS_GFT_COUNTER_INFO, *PNDIS_GFT_COUNTER_INFO;

#define NDIS_SIZEOF_GFT_COUNTER_INFO_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_COUNTER_INFO, UpdatePeriod)


//
// NDIS_GFT_COUNTER_INFO_ARRAY is used in OID_GFT_ENUM_COUNTERS
//
#define NDIS_GFT_COUNTER_INFO_ARRAY_REVISION_1        1

typedef struct _NDIS_GFT_COUNTER_INFO_ARRAY
{
    _In_ NDIS_OBJECT_HEADER                 Header;
    _In_ ULONG                              Flags;

    _In_ NDIS_GFT_TABLE_ID                  TableId;
    //
    // The following fields are used to describe an array of NDIS_GFT_COUNTER_INFO
    //
    _Out_ ULONG                             CounterInfoArrayOffset; //  from the beginning of this structure
    _Out_ ULONG                             CounterInfoArrayNumElements;
    _Out_ ULONG                             CounterInfoArrayElementSize;
} NDIS_GFT_COUNTER_INFO_ARRAY, *PNDIS_GFT_COUNTER_INFO_ARRAY;

#define NDIS_SIZEOF_GFT_COUNTER_INFO_ARRAY_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_COUNTER_INFO_ARRAY, CounterInfoArrayElementSize)


//
// NDIS_GFT_COUNTER_VALUE is used with NDIS_GFT_COUNTER_VALUE_ARRAY
// in OID_GFT_COUNTER_VALUES
//

typedef struct _NDIS_GFT_COUNTER_VALUE
{
    NDIS_GFT_COUNTER_TYPE                       CounterType;
    union
    {
        NDIS_GFT_PACKET_COUNTER_VALUE                   Packets;
        NDIS_GFT_BYTE_COUNTER_VALUE                     Bytes;
        NDIS_GFT_PACKET_BYTE_COUNTER_VALUE              PacketsBytes;
        NDIS_GFT_PACKET_BYTE_COUNTER_VALUE_AND_STATE    PacketsBytesAndState;
    } CounterValue;
} NDIS_GFT_COUNTER_VALUE, *PNDIS_GFT_COUNTER_VALUE;

//
// NDIS_GFT_COUNTER_VALUE_ARRAY is used in OID_GFT_COUNTER_VALUES
//

//
// flags used in Flags field of NDIS_GFT_COUNTER_VALUE_ARRAY
//
#define NDIS_GFT_COUNTER_VALUE_ARRAY_UPDATE_MEMORY_MAPPED_COUNTERS      0x00000001
#define NDIS_GFT_COUNTER_VALUE_ARRAY_GET_VALUES                         0x00000002

typedef enum _NDIS_GFT_COUNTER_VALUE_QUERY_METHOD
{
    NdisGftQueryValueQueryMethodUndefined,
    NdisGftQueryValueQueryMethodUsingCounterIds,
    NdisGftQueryValueQueryMethodUsingFlowEntryIds,
    NdisGftQueryValueQueryMethodVPortFlowEntries,
    NdisGftQueryValueQueryMethodAllFlowEntries,
    NdisGftQueryValueQueryMethodMax
} NDIS_GFT_COUNTER_VALUE_QUERY_METHOD, *PNDIS_GFT_COUNTER_VALUE_QUERY_METHOD;

#define NDIS_GFT_COUNTER_VALUE_ARRAY_REVISION_1       1

typedef struct _NDIS_GFT_COUNTER_VALUE_ARRAY
{
    _In_ NDIS_OBJECT_HEADER                 Header;
    _In_ ULONG                              Flags;

    _In_ NDIS_GFT_TABLE_ID                  TableId;

    union
    {
        _In_ NDIS_GFT_FLOW_ENTRY_ID             FlowEntryId;
        _In_ NDIS_GFT_COUNTER_ID                CounterId;
    } StartId;

    _In_ ULONG                              NumCounters;

    //
    // The following fields are used to describe an array of NDIS_GFT_COUNTER_VALUE
    //
    _Out_ ULONG                             CounterValueArrayOffset; //  from the beginning of this structure
    _Out_ ULONG                             CounterValueArrayNumElements;
    _Out_ ULONG                             CounterValueArrayElementSize;

} NDIS_GFT_COUNTER_VALUE_ARRAY, *PNDIS_GFT_COUNTER_VALUE_ARRAY;

#define NDIS_SIZEOF_GFT_COUNTER_VALUE_ARRAY_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_COUNTER_VALUE_ARRAY, CounterValueArrayElementSize)

//
// NDIS_GFT_STATISTICS is used in OID_GFT_STATISTICS
//

#define NDIS_GFT_STATISTICS_REVISION_1       1

typedef struct _NDIS_GFT_STATISTICS
{
    _In_ NDIS_OBJECT_HEADER                 Header;
    _In_ ULONG                              Flags;

    _In_ NDIS_GFT_TABLE_ID                  TableId; // required
    _In_ NDIS_NIC_SWITCH_VPORT_ID           VPortId; // optional
    _In_ NDIS_GFT_FLOW_ENTRY_ID             ProviderFlowEntryId; // optional

    _Out_ ULONG64                           Packets;
    _Out_ ULONG64                           Bytes;

    _Out_ ULONG64                           L1CacheHits; // used with TableId and VPortId
    _Out_ ULONG64                           L2CacheHits; // used with TableId and VPortId
    _Out_ ULONG64                           L3CacheHits; // used with TableId and VPortId

    _Out_ ULONG64                           L1CacheMisses; // used with TableId and VPortId
    _Out_ ULONG64                           L2CacheMisses; // used with TableId and VPortId
    _Out_ ULONG64                           L3CacheMisses; // used with TableId and VPortId

} NDIS_GFT_STATISTICS, *PNDIS_GFT_STATISTICS;

#define NDIS_SIZEOF_GFT_STATISTICS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_STATISTICS, L3CacheMisses)


//
// NDIS_GFT_HEADER_GROUP_TRANSPOSITION_ACTION specifies the action that must
// be taken on a single header group
//
typedef enum _NDIS_GFT_HEADER_GROUP_TRANSPOSITION_ACTION
{
    NdisGftHeaderGroupTranspositionActionUndefined,
    NdisGftHeaderGroupTranspositionActionModify,
    NdisGftHeaderGroupTranspositionActionIgnore,
    NdisGftHeaderGroupTranspositionActionPush,
    NdisGftHeaderGroupTranspositionActionPop,
    NdisGftHeaderGroupTranspositionActionMax
} NDIS_GFT_HEADER_GROUP_TRANSPOSITION_ACTION, *PNDIS_GFT_HEADER_GROUP_TRANSPOSITION_ACTION;


//
// flags used in Flags fields of NDIS_GFT_HEADER_GROUP_TRANSPOSITION_PROFILE
//
#define NDIS_GFT_HEADER_GROUP_TRANSPOSITION_PROFILE_DECREMENT_TTL_IF_NOT_ONE    0x00000001

//
// NDIS_GFT_HEADER_GROUP_TRANSPOSITION_PROFILE defines a header transposition profile
// for a single header group
//
// An array of NDIS_GFT_HEADER_GROUP_TRANSPOSITION_ACTIONs are used in
// NDIS_GFT_HEADER_TRANSPOSITION_PROFILE structure to define a
// full transposition profile
//

#define NDIS_GFT_HEADER_GROUP_TRANSPOSITION_PROFILE_REVISION_1       1

typedef struct _NDIS_GFT_HEADER_GROUP_TRANSPOSITION_PROFILE
{
    NDIS_OBJECT_HEADER                  Header;

    ULONG                               Flags;

    NDIS_GFT_HEADER_GROUP_TRANSPOSITION_ACTION Action;

    //
    // NDIS_GFP_HEADER_PRESENT_xxx
    //
    ULONG                               HeadersPresent;

    //
    // NDIS_GFP_HEADER_FIELD_xxx
    //
    ULONG                               HeaderFields;
} NDIS_GFT_HEADER_GROUP_TRANSPOSITION_PROFILE, *PNDIS_GFT_HEADER_GROUP_TRANSPOSITION_PROFILE;

#define NDIS_SIZEOF_GFT_HEADER_GROUP_TRANSPOSITION_PROFILE_REVISION_1    \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_HEADER_GROUP_TRANSPOSITION_PROFILE, HeaderFields)


typedef ULONG NDIS_GFT_CUSTOM_ACTION_TYPE, *PNDIS_GFT_CUSTOM_ACTION_TYPE;

#define NDIS_GFT_UNDEFINED_CUSTOM_ACTION    0

//
// custom action codes up to 256 are reserved
//
#define NDIS_GFT_RESERVED_CUSTOM_ACTIONS    256

//
// NDIS_GFT_CUSTOM_ACTION_PROFILE is used in NDIS_GFT_HEADER_TRANSPOSITION_PROFILE
// structure in order to describe a custom action profile. Custom actions profiles follow
// a TLV (Type Length Value) format to allow linking multiple custom action profiles.
//

#define NDIS_GFT_CUSTOM_ACTION_PROFILE_REVISION_1       1

typedef struct _NDIS_GFT_CUSTOM_ACTION_PROFILE
{
    NDIS_OBJECT_HEADER                  Header;
    ULONG                               Flags;
    NDIS_GFP_TABLE_TYPE                 TableType;
    NDIS_GFP_PROFILE_ID                 ProfileId;

    NDIS_GFT_CUSTOM_ACTION_TYPE         Type;
    ULONG                               Length;

    union
    {
        ULONG_PTR                       Alignment;
        _Field_size_bytes_(Length)
            UCHAR                           ActionProfileData[1];
    };
} NDIS_GFT_CUSTOM_ACTION_PROFILE, *PNDIS_GFT_CUSTOM_ACTION_PROFILE;

#define NDIS_SIZEOF_NDIS_GFT_CUSTOM_ACTION_PROFILE_REVISION_1    \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_CUSTOM_ACTION_PROFILE, Alignment)

//
// NDIS_GFT_HEADER_TRANSPOSITION_PROFILE is used in OID_GFT_HEADER_TRANSPOSITION_PROFILE
//
// This structure defines a transposition profile for multiple header groups
//

//
// flags used with Flags field of NDIS_GFT_HEADER_TRANSPOSITION_PROFILE
// HTP=Header Transposition Profile
//
#define NDIS_GFT_HTP_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT                 0x00000001
#define NDIS_GFT_HTP_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT                  0x00000002
#define NDIS_GFT_HTP_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE   0x00000004
#define NDIS_GFT_HTP_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE    0x00000008
#define NDIS_GFT_HTP_COPY_ALL_PACKETS                                   0x00000010
#define NDIS_GFT_HTP_COPY_FIRST_PACKET                                  0x00000020
#define NDIS_GFT_HTP_COPY_WHEN_TCP_FLAG_SET                             0x00000040
#define NDIS_GFT_HTP_CUSTOM_ACTION_PRESENT                              0x00000080
#define NDIS_GFT_HTP_META_ACTION_BEFORE_HEADER_TRANSPOSITION            0x00000100

#define NDIS_GFT_HEADER_TRANSPOSITION_PROFILE_REVISION_1       1

typedef struct _NDIS_GFT_HEADER_TRANSPOSITION_PROFILE
{
    NDIS_OBJECT_HEADER                  Header;
    ULONG                               Flags;
    NDIS_GFP_TABLE_TYPE                 TableType;

    NDIS_GFP_PROFILE_ID                 ProfileId;

    //
    // The following fields are used to describe an array of NDIS_GFT_HEADER_GROUP_TRANSPOSITION_PROFILE structures
    //
    ULONG                               HeaderGroupTranspositionProfileArrayOffset;      //  from the beginning of this structure
    ULONG                               HeaderGroupTranspositionProfileArrayNumElements;
    ULONG                               HeaderGroupTranspositionProfileArrayElementSize;

    //
    // the offset to the beginning of the custom action profile TLVs
    //
    ULONG                               CustomActionProfileOffset;
} NDIS_GFT_HEADER_TRANSPOSITION_PROFILE, *PNDIS_GFT_HEADER_TRANSPOSITION_PROFILE;

#define NDIS_SIZEOF_GFT_HEADER_TRANSPOSITION_PROFILE_REVISION_1    \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_HEADER_TRANSPOSITION_PROFILE, CustomActionProfileOffset)


//
// flags used in Flags fields of NDIS_GFT_HEADER_GROUP_TRANSPOSITION
//
#define NDIS_GFT_HEADER_GROUP_TRANSPOSITION_DECREMENT_TTL_IF_NOT_ONE    0x00000001


//
// NDIS_GFT_HEADER_GROUP_TRANSPOSITION is used in NDIS_GFT_EXACT_MATCH_FLOW_ENTRY
//
#define NDIS_GFT_HEADER_GROUP_TRANSPOSITION_REVISION_1       1

typedef struct _NDIS_GFT_HEADER_GROUP_TRANSPOSITION
{

    NDIS_OBJECT_HEADER                  Header;
    ULONG                               Flags;


    NDIS_GFT_HEADER_GROUP_TRANSPOSITION_ACTION Action;

    //
    // NDIS_GFP_HEADER_PRESENT_xxx
    //
    ULONG                               HeadersPresent;

    //
    // NDIS_GFP_HEADER_FIELD_xxx
    //
    ULONG64                             HeaderFields;

    //
    // Ethernet header
    //
    NDIS_GFP_ETHERNET_HEADER_FIELDS     EthernetFields;

    //
    // IP header
    //
    union
    {
        struct
        {
            IN_ADDR                     SourceIPAddress;
            IN_ADDR                     DestinationIPAddress;
        } IPv4Address;

        struct
        {
            IN6_ADDR                    SourceIPAddress;
            IN6_ADDR                    DestinationIPAddress;
        } IPv6Address;
    } IPAddress;

    UINT8                               Ttl;
    UINT8                               Dscp;
    UINT8                               IPProtocol;

    //
    // transport / encapsulation header
    //
    union
    {
        struct
        {
            USHORT                      SourcePort;
            USHORT                      DestinationPort;
        } Udp;

        struct
        {
            USHORT                      SourcePort;
            USHORT                      DestinationPort;
        } Tcp;

        struct
        {
            ULONG                       TenantId;
            USHORT                      GreProtocol;
            USHORT                      Entropy;
        } Encapsulation;
    } TransportOrEncapsulation;

} NDIS_GFT_HEADER_GROUP_TRANSPOSITION, *PNDIS_GFT_HEADER_GROUP_TRANSPOSITION;

#define NDIS_SIZEOF_GFT_HEADER_GROUP_TRANSPOSITION_REVISION_1    \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_HEADER_GROUP_TRANSPOSITION, TransportOrEncapsulation)

//
// NDIS_GFT_FLOW_ENTRY_CACHE_HINT is used to provide a hint to the GFT offload provider
// so it can use its resources more efficiently
//
typedef enum _NDIS_GFT_FLOW_ENTRY_CACHE_HINT
{
    NdisGftFlowEntryCacheHintUndefined = 0,
    NdisGftFlowEntryCacheHintLowFrequency = 100,
    NdisGftFlowEntryCacheHintMediumFrequency = 200,
    NdisGftFlowEntryCacheHintHighFrequency = 300,
    NdisGftFlowEntryCacheHintMax = 1000
} NDIS_GFT_FLOW_ENTRY_CACHE_HINT, *PNDIS_GFT_FLOW_ENTRY_CACHE_HINT;


typedef enum _NDIS_GFT_FLOW_ENTRY_STATE
{
    NdisGftFlowEntryStateUndefined,
    NdisGftFlowEntryStateDeactivated,
    NdisGftFlowEntryStateActivated,
    NdisGftFlowEntryStateMax
} NDIS_GFT_FLOW_ENTRY_STATE, *PNDIS_GFT_FLOW_ENTRY_STATE;

//
// NDIS_GFT_CUSTOM_ACTION is used in NDIS_GFT_EXACT_MATCH_FLOW_ENTRY
// structure in order to describe a custom action. Custom actions follow
// a TLV (Type Length Value) format to allow linking multiple custom actions.
//
//
// flags used in NDIS_GFT_CUSTOM_ACTION
//
#define NDIS_GFT_CUSTOM_ACTION_LAST_ACTION      0x00000001

#define NDIS_GFT_CUSTOM_ACTION_REVISION_1       1

typedef struct _NDIS_GFT_CUSTOM_ACTION
{
    NDIS_OBJECT_HEADER                  Header;
    ULONG                               Flags;

    NDIS_GFP_PROFILE_ID                 ProfileId;

    NDIS_GFT_CUSTOM_ACTION_TYPE         Type;
    ULONG                               Length;

    union
    {
        ULONG_PTR                       Alignment;
        _Field_size_bytes_(Length)
            UCHAR                           ActionData[1];
    };
} NDIS_GFT_CUSTOM_ACTION, *PNDIS_GFT_CUSTOM_ACTION;

#define NDIS_SIZEOF_NDIS_GFT_CUSTOM_ACTION_REVISION_1    \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_CUSTOM_ACTION, Alignment)


//
// NDIS_GFT_EXACT_MATCH_FLOW_ENTRY is used with NDIS_GFT_FLOW_ENTRY_ARRAY
// that is used in OID_GFT_ADD_FLOW_ENTRIES and OID_GFT_FLOW_ENTRY_PARAMETERS.
// And with NDIS_GFT_FLOW_ENTRY_INFO_ARRAY that is used in
// OID_GFT_ENUM_FLOW_ENTRIES
//

//
// flags used in Flags field of NDIS_GFT_EXACT_MATCH_FLOW_ENTRY.
// EMFE = Exact Match Flow Entry
//
// used with OID_GFT_ADD_FLOW_ENTRIES
//
#define NDIS_GFT_EMFE_ADD_IN_ACTIVATED_STATE                            0x00000001
#define NDIS_GFT_EMFE_MATCH_AND_ACTION_MUST_BE_SUPPORTED                0x00000002

//
// used with OID_GFT_ADD_FLOW_ENTRIES, OID_GFT_ENUM_FLOW_ENTRIES
//
#define NDIS_GFT_EMFE_RDMA_FLOW                                         0x00000004
//
// used with OID_GFT_ADD_FLOW_ENTRIES, OID_GFT_ENUM_FLOW_ENTRIES, OID_GFT_FLOW_ENTRY_PARAMETERS
//
#define NDIS_GFT_EMFE_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT                0x00001000
#define NDIS_GFT_EMFE_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT                 0x00002000
#define NDIS_GFT_EMFE_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE  0x00004000
#define NDIS_GFT_EMFE_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE   0x00008000
#define NDIS_GFT_EMFE_COPY_ALL_PACKETS                                  0x00010000
#define NDIS_GFT_EMFE_COPY_FIRST_PACKET                                 0x00020000
#define NDIS_GFT_EMFE_COPY_WHEN_TCP_FLAG_SET                            0x00040000
#define NDIS_GFT_EMFE_CUSTOM_ACTION_PRESENT                             0x00080000
#define NDIS_GFT_EMFE_META_ACTION_BEFORE_HEADER_TRANSPOSITION           0x00100000
#define NDIS_GFT_EMFE_COPY_AFTER_TCP_FIN_FLAG_SET                       0x00200000
#define NDIS_GFT_EMFE_COPY_AFTER_TCP_RST_FLAG_SET                       0x00400000

//
// used with OID_GFT_FLOW_ENTRY_PARAMETERS
//
#define NDIS_GFT_EMFE_COPY_CONDITION_CHANGED                            0x01000000
#define NDIS_GFT_EMFE_ALL_VPORT_FLOW_ENTRIES                            0x02000000


//
// flags used in CounterFlags field of of NDIS_GFT_EXACT_MATCH_FLOW_ENTRY.
// EMFE = Exact Match Flow Entry
//
#define NDIS_GFT_EMFE_COUNTER_ALLOCATE                                  0x00000001
#define NDIS_GFT_EMFE_COUNTER_MEMORY_MAPPED                             0x00000002
#define NDIS_GFT_EMFE_COUNTER_CLIENT_SPECIFIED_ADDRESS                  0x00000004
#define NDIS_GFT_EMFE_COUNTER_TRACK_TCP_FLOW                            0x00000008

#define NDIS_GFT_EXACT_MATCH_FLOW_ENTRY_REVISION_1       1

typedef struct _NDIS_GFT_EXACT_MATCH_FLOW_ENTRY
{
    _In_ NDIS_OBJECT_HEADER                 Header;
    _In_ ULONG                              Flags;

    _In_ NDIS_GFT_TABLE_ID                  TableId;

    //
    // the ID of either ingress or egress VPort
    //
    _In_ NDIS_NIC_SWITCH_VPORT_ID           VPortId;

    _In_ NDIS_GFP_PROFILE_ID                MatchProfileId;

    //
    //
    // allocation or activation status for this flow entry
    //
    _Out_ NDIS_STATUS                       MatchRequestStatus;
    _Out_ NDIS_STATUS                       ActionRequestStatus;


    _In_ ULONG                              CounterFlags;
    _In_ NDIS_GFT_COUNTER_UPDATE_FREQUENCY  CounterUpdateFrequency;
    _In_ NDIS_GFT_COUNTER_TYPE              CounterType;
    _In_ ULONG                              UpdatePeriod; // in milliseconds
    union
    {
        _Inout_ PNDIS_GFT_PACKET_COUNTER_VALUE                  PacketCounterAddress;
        _Inout_ PNDIS_GFT_BYTE_COUNTER_VALUE                    ByteCounterAddress;
        _Inout_ PNDIS_GFT_PACKET_BYTE_COUNTER_VALUE             PacketByteCounterAddress;
        _Inout_ PNDIS_GFT_PACKET_BYTE_COUNTER_VALUE_AND_STATE   PacketByteCounterAndStateAddress;
    } CounterValueBuffer;

    //
    // optional
    //
    _In_ NDIS_GFP_PROFILE_ID                HeaderTranspositionProfileId;


    //
    // valid only if NDIS_GFT_EMFE_REDIRECT_TO_xxx_VPORT is set
    //
    _In_ NDIS_NIC_SWITCH_VPORT_ID           RedirectionVPortId;

    //
    // valid only if NDIS_GFT_EMFE_REDIRECT_TO_xxx_IF_TTL_IS_ONE is set
    //
    _In_ NDIS_NIC_SWITCH_VPORT_ID           TtlIsOneRedirectionVPortId;

    _In_ ULONG                              NumCounterObjects;
    _In_ NDIS_GFT_COUNTER_ID                CounterIdArray[NDIS_GFT_MAX_COUNTER_OBJECTS_PER_FLOW_ENTRY];

    _In_ NDIS_GFT_FLOW_ENTRY_CACHE_HINT     CacheHint;

    _Inout_ NDIS_GFT_FLOW_ENTRY_ID          ClientFlowEntryId;
    _Inout_ NDIS_GFT_FLOW_ENTRY_ID          ProviderFlowEntryId;
    _Inout_ NDIS_GFT_FLOW_ENTRY_STATE       FlowEntryState; // _Out_ for Enum

    _Inout_ ULONG                           TcpSequenceNumber;

    //
    // The following fields are used to describe an array of NDIS_GFP_HEADER_GROUP_EXACT_MATCH structures
    // if a match profile ID is specified in MatchProfileId, then these fields must be consistent
    // with the specified profile ID
    //
    _In_ ULONG                              HeaderGroupExactMatchArrayOffset; //  from the beginning of this structure
    _In_ ULONG                              HeaderGroupExactMatchArrayNumElements;
    _In_ ULONG                              HeaderGroupExactMatchArrayElementSize;


    //
    // The following fields are used to describe an array of NDIS_GFT_HEADER_GROUP_TRANSPOSITION structures
    // if a header transposition profile ID is specified in HeaderTranspositionProfileId, then these fields must be consistent
    // with the specified profile ID
    //
    _In_ ULONG                              HeaderGroupTranspositionArrayOffset;      //  from the beginning of this structure
    _In_ ULONG                              HeaderGroupTranspositionArrayNumElements;
    _In_ ULONG                              HeaderGroupTranspositionArrayElementSize;

    //
    // the offset to the beginning of custom action TLVs
    //
    _In_ ULONG                              CustomActionOffset;
} NDIS_GFT_EXACT_MATCH_FLOW_ENTRY, *PNDIS_GFT_EXACT_MATCH_FLOW_ENTRY;

#define NDIS_SIZEOF_GFT_EXACT_MATCH_FLOW_ENTRY_REVISION_1    \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_EXACT_MATCH_FLOW_ENTRY, CustomActionOffset)

//
// wildcard GFT entries
//



typedef enum _NDIS_GFT_WILDCARD_ACTION
{
    NdisGftWildcardActionUndefined,
    NdisGftWildcardActionAllow,
    NdisGftWildcardActionDrop,
    NdisGftWildcardActionMax
} NDIS_GFT_WILDCARD_ACTION, *PNDIS_GFT_WILDCARD_ACTION;


//
// flags used in Flags field of NDIS_GFT_WILDCARD_MATCH_FLOW_ENTRY
// WCFE = Wildcard Flow Entry
//
#define NDIS_GFT_WCFE_ADD_IN_ACTIVATED_STATE                            0x00000001
#define NDIS_GFT_WCFE_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT                0x00000002
#define NDIS_GFT_WCFE_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT                 0x00000004
#define NDIS_GFT_WCFE_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE  0x00000008
#define NDIS_GFT_WCFE_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE   0x00000010
#define NDIS_GFT_WCFE_COPY_ALL_PACKETS                                  0x00000020
#define NDIS_GFT_WCFE_CUSTOM_ACTION_PRESENT                             0x00000040


//
// flags used in CounterFlags field of of NDIS_GFT_WILDCARD_MATCH_FLOW_ENTRY.
// WCFE = Wildcard Flow Entry
//
#define NDIS_GFT_WCFE_COUNTER_ALLOCATE                                  0x00000001
#define NDIS_GFT_WCFE_COUNTER_MEMORY_MAPPED                             0x00000002
#define NDIS_GFT_WCFE_COUNTER_CLIENT_SPECIFIED_ADDRESS                  0x00000004

//
// NDIS_GFT_WILDCARD_MATCH_FLOW_ENTRY is used with NDIS_GFT_FLOW_ENTRY_ARRAY
// that is used in OID_GFT_ADD_FLOW_ENTRT and OID_GFT_FLOW_ENTRY_PARAMETERS
// and with NDIS_GFT_FLOW_ENTRY_INFO_ARRAY that is used in
// OID_GFT_ENUM_FLOW_ENTRIES
//

#define NDIS_GFT_WILDCARD_MATCH_FLOW_ENTRY_REVISION_1                   1

typedef struct _NDIS_GFT_WILDCARD_MATCH_FLOW_ENTRY
{
    _In_ NDIS_OBJECT_HEADER                 Header;
    _In_ ULONG                              Flags;

    _In_ NDIS_GFT_TABLE_ID                  TableId;

    //
    // the ID of either ingress or egress VPort
    //
    _In_ NDIS_NIC_SWITCH_VPORT_ID           VPortId;

    _In_ NDIS_GFP_PROFILE_ID                MatchProfileId;

    //
    //
    // allocation or activation status for this flow entry
    //
    _Out_ NDIS_STATUS                       MatchRequestStatus;
    _Out_ NDIS_STATUS                       ActionRequestStatus;


    _In_ ULONG                              CounterFlags;
    _In_ NDIS_GFT_COUNTER_UPDATE_FREQUENCY  CounterUpdateFrequency;
    _In_ NDIS_GFT_COUNTER_TYPE              CounterType;
    _In_ ULONG                              UpdatePeriod; // in milliseconds
    union
    {
        _Inout_ PNDIS_GFT_PACKET_COUNTER_VALUE              PacketCounterAddress;
        _Inout_ PNDIS_GFT_BYTE_COUNTER_VALUE                ByteCounterAddress;
        _Inout_ PNDIS_GFT_PACKET_BYTE_COUNTER_VALUE         PacketByteCounterAddress;
    } CounterValueBuffer;

    _In_ ULONG                              Priority;
    _In_ NDIS_GFT_WILDCARD_ACTION           Action;

    //
    // valid only if NDIS_GFT_WCFE_REDIRECT_TO_xxx_QUEUE_OF_VPORT is set
    //
    _In_ NDIS_NIC_SWITCH_VPORT_ID           RedirectionVPortId;

    //
    // valid only if NDIS_GFT_WCFE_REDIRECT_TO_xxx_IF_TTL_IS_ONE is set
    //
    _In_ NDIS_NIC_SWITCH_VPORT_ID           TtlIsOneRedirectionVPortId;

    _In_ ULONG                              NumCounterObjects;
    _In_ NDIS_GFT_COUNTER_ID                CounterIdArray[NDIS_GFT_MAX_COUNTER_OBJECTS_PER_FLOW_ENTRY];

    _In_ NDIS_GFT_FLOW_ENTRY_CACHE_HINT     CacheHint;

    _Inout_ NDIS_GFT_FLOW_ENTRY_ID          ClientFlowEntryId;
    _Inout_ NDIS_GFT_FLOW_ENTRY_ID          ProviderFlowEntryId;

    _Out_ NDIS_GFT_FLOW_ENTRY_STATE         FlowEntryState;

    //
    // The following fields are used to describe an array of NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH structures
    // if a match profile ID is specified in MatchProfileId, then these fields must be consistent
    // with the specified profile ID
    //
    _In_ ULONG                              HeaderGroupWildcardMatchArrayOffset; //  from the beginning of this structure
    _In_ ULONG                              HeaderGroupWildcardMatchArrayNumElements;
    _In_ ULONG                              HeaderGroupWildcardMatchArrayElementSize;

    //
    // the offset to the beginning of custom action TLVs
    //
    _In_ ULONG                              CustomActionOffset;
} NDIS_GFT_WILDCARD_MATCH_FLOW_ENTRY, *PNDIS_GFT_WILDCARD_MATCH_FLOW_ENTRY;

#define NDIS_SIZEOF_GFT_WILDCARD_MATCH_FLOW_ENTRY_REVISION_1    \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_WILDCARD_MATCH_FLOW_ENTRY, CustomActionOffset)

//
// NDIS_GFT_PROFILE_TYPE enum type is used as ProfileType field in
// NDIS_GFT_PROFILE_INFO
//

typedef enum _NDIS_GFT_PROFILE_TYPE
{
    NdisGftProfileTypeUndefined,
    NdisGftProfileTypeExactMatch,
    NdisGftProfileTypeHeaderTransposition,
    NdisGftProfileTypeWildcardMatch,
    NdisGftProfileTypeMax
} NDIS_GFT_PROFILE_TYPE, *PNDIS_GFT_PROFILE_TYPE;

//
// NDIS_GFT_PROFILE_INFO is used in NDIS_GFT_PROFILE_INFO_ARRAY
//

#define NDIS_GFT_PROFILE_INFO_REVISION_1       1

typedef struct _NDIS_GFT_PROFILE_INFO
{
    _Out_ NDIS_OBJECT_HEADER                Header;
    _Out_ ULONG                             Flags;

    _Out_ NDIS_GFT_PROFILE_TYPE             ProfileType;
    _Out_ NDIS_GFP_TABLE_TYPE               TableType;
    _Out_ NDIS_GFP_PROFILE_ID               ProfileId;
    _Out_ ULONG                             NumSupportedFlowEntries;
    _Out_ ULONG                             NumCurrentFlowEntries;

    //
    // The following fields are used to describe an array of NDIS_GFP_HEADER_GROUP_EXACT_MATCH_PROFILE,
    // NDIS_GFT_HEADER_GROUP_TRANSPOSITION_PROFILE, or NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILE structures
    //
    // The value of ProfileType field above determines the type of profile the structure
    // describes
    //
    _Out_ ULONG                             HeaderGroupArrayOffset; //  from the beginning of this structure
    _Out_ ULONG                             HeaderGroupArrayNumElements;
    _Out_ ULONG                             HeaderGroupArrayElementSize;

} NDIS_GFT_PROFILE_INFO, *PNDIS_GFT_PROFILE_INFO;

#define NDIS_SIZEOF_GFT_PROFILE_INFO_REVISION_1    \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_PROFILE_INFO, HeaderGroupArrayElementSize)

//
// NDIS_GFT_PROFILE_INFO_ARRAY is used in OID_GFT_ENUM_PROFILES
//
#define NDIS_GFT_PROFILE_INFO_ARRAY_REVISION_1        1

typedef struct _NDIS_GFT_PROFILE_INFO_ARRAY
{
    _Out_ NDIS_OBJECT_HEADER                 Header;
    _Out_ ULONG                              Flags;

    //
    // The following fields are used to describe an array of NDIS_GFT_PROFILE_INFO
    //
    _Out_ ULONG                             ProfileInfoArrayOffset; //  from the beginning of this structure
    _Out_ ULONG                             ProfileInfoArrayNumElements;
    _Out_ ULONG                             ProfileInfoArrayElementSize;
} NDIS_GFT_PROFILE_INFO_ARRAY, *PNDIS_GFT_PROFILE_INFO_ARRAY;

#define NDIS_SIZEOF_GFT_PROFILE_INFO_ARRAY_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_PROFILE_INFO_ARRAY, ProfileArrayElementSize)

//
// flags used in NDIS_GFT_DELETE_PROFILE_PARAMETERS structure
//
#define NDIS_GFT_DELETE_PROFILE_ALL_PROFILES    0x00000001

//
// NDIS_GFT_DELETE_PROFILE_PARAMETERS is used in
// OID_GFT_DELETE_PROFILE
//
#define NDIS_GFT_DELETE_PROFILE_PARAMETERS_REVISION_1       1

typedef struct _NDIS_GFT_DELETE_PROFILE_PARAMETERS
{
    NDIS_OBJECT_HEADER                  Header;
    ULONG                               Flags;
    NDIS_GFP_PROFILE_ID                 ProfileId;
} NDIS_GFT_DELETE_PROFILE_PARAMETERS, *PNDIS_GFT_DELETE_PROFILE_PARAMETERS;

#define NDIS_SIZEOF_GFT_DELETE_PROFILE_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_DELETE_PROFILE_PARAMETERS, ProfileId)

typedef enum _NDIS_GFT_FLOW_ENTRY_TYPE
{
    NdisGftFlowEntryTypeUndefined,
    NdisGftFlowEntryTypeExactMatch,
    NdisGftFlowEntryTypeWildcard,
    NdisGftFlowEntryTypeMax
} NDIS_GFT_FLOW_ENTRY_TYPE, *PNDIS_GFT_FLOW_ENTRY_TYPE;


//
// NDIS_GFT_FLOW_ENTRY_ARRAY is used in OID_GFT_ADD_FLOW_ENTRIES
// OID_GFT_FLOW_ENTRY_PARAMETERS
//
#define NDIS_GFT_FLOW_ENTRY_ARRAY_REVISION_1       1

typedef struct _NDIS_GFT_FLOW_ENTRY_ARRAY
{
    NDIS_OBJECT_HEADER                 Header;
    ULONG                              Flags;

    NDIS_GFT_FLOW_ENTRY_TYPE           EntryType;

    //
    // The following fields are used to describe an array of NDIS_GFT_EXACT_MATCH_FLOW_ENTRY
    // or NDIS_GFT_WILDCARD_MATCH_FLOW_ENTRY structures. The table type of the table identified by
    // TableId field above determines the type of the flow entries that are added.
    //
    ULONG                              FlowEntryArrayOffset; //  from the beginning of this structure
    ULONG                              FlowEntryArrayNumElements;
    ULONG                              FlowEntryArrayElementSize;
} NDIS_GFT_FLOW_ENTRY_ARRAY, *PNDIS_GFT_FLOW_ENTRY_ARRAY;

#define NDIS_SIZEOF_GFT_FLOW_ENTRY_ARRAY_REVISION_1    \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_FLOW_ENTRY_ARRAY, FlowEntryArrayElementSize)

//
// flags used in NDIS_GFT_FLOW_ENTRY_INFO_ARRAY structure
//
#define NDIS_GFT_FLOW_ENTRY_INFO_ALL_FLOW_ENTRIES           0x00000001

//
// NDIS_GFT_FLOW_ENTRY_INFO_ARRAY is used in OID_GFT_ENUM_FLOW_ENTRIES
//
#define NDIS_GFT_FLOW_ENTRY_INFO_ARRAY_REVISION_1       1

typedef struct _NDIS_GFT_FLOW_ENTRY_INFO_ARRAY
{
    _In_ NDIS_OBJECT_HEADER                 Header;
    _In_ ULONG                              Flags;

    _In_ NDIS_GFT_TABLE_ID                  TableId;

    //
    // optional fields for limiting the number of flows enumerated in
    // a single call
    //
    _In_ NDIS_NIC_SWITCH_VPORT_ID           VPortId;
    _In_ NDIS_GFT_FLOW_ENTRY_ID             ProviderFlowEntryIdStart;
    _In_ ULONG                              FlowEntryIdCount;

    //
    // The following fields are used to describe an array of NDIS_GFT_EXACT_MATCH_FLOW_ENTRY
    // or NDIS_GFT_WILDCARD_MATCH_FLOW_ENTRY structures. The table type of the table identified by
    // TableId field above determines the type of the flows that are enumerated.
    //
    _Out_ ULONG                             FlowEntryInfoArrayOffset; //  from the beginning of this structure
    _Out_ ULONG                             FlowEntryInfoArrayNumElements;
    _Out_ ULONG                             FlowEntryInfoArrayElementSize;
} NDIS_GFT_FLOW_ENTRY_INFO_ARRAY, *PNDIS_GFT_FLOW_ENTRY_INFO_ARRAY;

#define NDIS_SIZEOF_GFT_FLOW_ENTRY_INFO_ARRAY_REVISION_1    \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_FLOW_ENTRY_INFO_ARRAY, FlowEntryInfoArrayElementSize)


//
// flags used in NDIS_GFT_FLOW_ENTRY_ID_ARRAY structure
//
#define NDIS_GFT_FLOW_ENTRY_ID_ALL_NIC_SWITCH_FLOW_ENTRIES      0x00000001
#define NDIS_GFT_FLOW_ENTRY_ID_ALL_TABLE_FLOW_ENTRIES           0x00000002
#define NDIS_GFT_FLOW_ENTRY_ID_ALL_VPORT_FLOW_ENTRIES           0x00000004
#define NDIS_GFT_FLOW_ENTRY_ID_RANGE_DEFINED                    0x00000008
#define NDIS_GFT_FLOW_ENTRY_ID_ARRAY_DEFINED                    0x00000010
#define NDIS_GFT_FLOW_ENTRY_ID_ARRAY_COUNTER_VALUES             0x00010000

//
// In NDIS 6.50, NDIS_GFT_FLOW_ENTRY_ID_AND_COUNTER structure is used in
// NDIS_GFT_FLOW_ENTRY_ID_ARRAY with OID_GFT_DELETE_FLOW_ENTRIES or OID_GFT_DEACTIVATE_FLOW_ENTRIES
//
typedef struct _NDIS_GFT_FLOW_ENTRY_ID_AND_COUNTER
{
    NDIS_GFT_FLOW_ENTRY_ID      FlowEntryId;
    NDIS_GFT_COUNTER_VALUE      CounterValue;
} NDIS_GFT_FLOW_ENTRY_ID_AND_COUNTER, *PNDIS_GFT_FLOW_ENTRY_ID_AND_COUNTER;

//
// NDIS_GFT_FLOW_ENTRY_ID_ARRAY is used in OID_GFT_DELETE_FLOW_ENTRIES, OID_GFT_ACTIVATE_FLOW_ENTRIES
// and OID_GFT_DEACTIVATE_FLOW_ENTRIES
//
#define NDIS_GFT_FLOW_ENTRY_ID_ARRAY_REVISION_1       1

typedef struct _NDIS_GFT_FLOW_ENTRY_ID_ARRAY
{
    _In_ NDIS_OBJECT_HEADER                  Header;
    _In_ ULONG                               Flags;

    _In_ NDIS_GFT_TABLE_ID                   TableId;

    //
    // optional fields for limiting the number of flows affected in
    // a single call
    //
    _In_ NDIS_NIC_SWITCH_VPORT_ID            VPortId;
    _In_ union
    {
        struct
        {
            //
            // The following fields are used to describe an array of NDIS_GFT_FLOW_ENTRY_IDs
            //
            ULONG                               ProviderFlowEntryIdArrayOffset; //  from the beginning of this structure
            ULONG                               ProviderFlowEntryIdArrayNumElements;
            ULONG                               ProviderFlowEntryIdArrayElementSize;
        } FlowEntryIdArray;

        struct
        {
            //
            // The following fields are used to describe an array of NDIS_GFT_FLOW_ENTRY_ID_AND_COUNTERs
            //
            ULONG                               ProviderFlowEntryIdAndCounterArrayOffset; //  from the beginning of this structure
            ULONG                               ProviderFlowEntryIdAndCounterArrayNumElements;
            ULONG                               ProviderFlowEntryIdAndCounterArrayElementSize;
        } FlowEntryIdAndCounterArray;

    } IdArray;

} NDIS_GFT_FLOW_ENTRY_ID_ARRAY, *PNDIS_GFT_FLOW_ENTRY_ID_ARRAY;

#define NDIS_SIZEOF_GFT_FLOW_ENTRY_ID_ARRAY_REVISION_1    \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_FLOW_ENTRY_ID_ARRAY, IdArray)



//
// Flags used in NDIS_GFT_OFFLOAD_PARAMETERS
//
#define NDIS_GFT_OFFLOAD_PARAMETERS_ENABLE_OFFLOAD              0x00000001
#define NDIS_GFT_OFFLOAD_PARAMETERS_CUSTOM_PROVIDER_RESERVED    0xFF000000

//
// NDIS_GFT_OFFLOAD_PARAMETERS is used in OID_GFT_GLOBAL_PARAMETERS
// and NDIS_STATUS_GFT_OFFLOAD_CURRENT_CONFIG status indication
//

#define NDIS_GFT_OFFLOAD_PARAMETERS_REVISION_1            1

typedef struct _NDIS_GFT_OFFLOAD_PARAMETERS
{
    NDIS_OBJECT_HEADER                      Header;
    ULONG                                   Flags;

    //
    // Copy and sample VPorts are the same as the exception VPort
    //
    NDIS_NIC_SWITCH_VPORT_ID                ExceptionVPortId;

    //
    // The aggregated length of the headers that are copied
    //
    ULONG                                   CopyLookaheadLength;
    ULONG                                   SampleLookaheadLength;

} NDIS_GFT_OFFLOAD_PARAMETERS, *PNDIS_GFT_OFFLOAD_PARAMETERS;

#define NDIS_SIZEOF_GFT_OFFLOAD_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_OFFLOAD_PARAMETERS, SampleLookaheadLength)

//
// NDIS_GFT_OFFLOAD_CAPABILITIES is used in OID_GFT_HARDWARE_CAPABILITIES,
// OID_GFT_CURRENT_CAPABILITIES, NDIS_STATUS_GFT_OFFLOAD_CURRENT_CAPABILITIES
// and NDIS_STATUS_GFT_OFFLOAD_HARDWARE_CAPABILITIES
//

//
// flags used in Flags field of NDIS_GFT_OFFLOAD_CAPABILITIES
//
#define NDIS_GFT_OFFLOAD_CAPS_ADD_FLOW_ENTRY_DEACTIVATED_PREFERRED              0x00000001
#define NDIS_GFT_OFFLOAD_CAPS_RATE_LIMITING_QUEUE_SUPPORTED                     0x00000002


//
// flags used in CounterCapabilities field of NDIS_GFT_OFFLOAD_CAPABILITIES
//
#define NDIS_GFT_OFFLOAD_CAPS_MEMORY_MAPPED_COUNTERS                            0x00000001
#define NDIS_GFT_OFFLOAD_CAPS_MEMORY_MAPPED_PAKCET_AND_BYTE_COUNTERS            0x00000002
#define NDIS_GFT_OFFLOAD_CAPS_PER_FLOW_ENTRY_COUNTERS                           0x00000004
#define NDIS_GFT_OFFLOAD_CAPS_PER_PACKET_COUNTER_UPDATE                         0x00000008
#define NDIS_GFT_OFFLOAD_CAPS_CLIENT_SPECIFIED_MEMORY_MAPPED_COUNTERS           0x00000010
#define NDIS_GFT_OFFLOAD_CAPS_INGRESS_AGGREGATE_COUNTERS                        0x00000020
#define NDIS_GFT_OFFLOAD_CAPS_EGRESS_AGGREGATE_COUNTERS                         0x00000040
#define NDIS_GFT_OFFLOAD_CAPS_TRACK_TCP_FLOW_STATE                              0x00000080
#define NDIS_GFT_OFFLOAD_CAPS_COMBINED_COUNTER_AND_STATE                        0x00000100


//
// flags used in SupportedTableTypes field of NDIS_GFT_OFFLOAD_CAPABILITIES
//
#define NDIS_GFT_OFFLOAD_CAPS_INGRESS_WILDCARD_MATCH                            0x00000001
#define NDIS_GFT_OFFLOAD_CAPS_EGRESS_WILDCARD_MATCH                             0x00000002
#define NDIS_GFT_OFFLOAD_CAPS_INGRESS_EXACT_MATCH                               0x00000004
#define NDIS_GFT_OFFLOAD_CAPS_EGRESS_EXACT_MATCH                                0x00000008
#define NDIS_GFT_OFFLOAD_CAPS_EXT_VPORT_INGRESS_WILDCARD_MATCH                  0x00000010
#define NDIS_GFT_OFFLOAD_CAPS_EXT_VPORT_EGRESS_WILDCARD_MATCH                   0x00000020
#define NDIS_GFT_OFFLOAD_CAPS_EXT_VPORT_INGRESS_EXACT_MATCH                     0x00000040
#define NDIS_GFT_OFFLOAD_CAPS_EXT_VPORT_EGRESS_EXACT_MATCH                      0x00000080

//
// supported actions
//
#define NDIS_GFT_OFFLOAD_CAPS_POP                                               0x00000001
#define NDIS_GFT_OFFLOAD_CAPS_PUSH                                              0x00000002
#define NDIS_GFT_OFFLOAD_CAPS_MODIFY                                            0x00000004
#define NDIS_GFT_OFFLOAD_CAPS_IGNORE_ACTION_SUPPORTED                           0x00000008
#define NDIS_GFT_OFFLOAD_CAPS_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT                0x00000010
#define NDIS_GFT_OFFLOAD_CAPS_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT                 0x00000020
#define NDIS_GFT_OFFLOAD_CAPS_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE  0x00000040
#define NDIS_GFT_OFFLOAD_CAPS_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE   0x00000080
#define NDIS_GFT_OFFLOAD_CAPS_COPY_ALL                                          0x00000100
#define NDIS_GFT_OFFLOAD_CAPS_COPY_FIRST                                        0x00000200
#define NDIS_GFT_OFFLOAD_CAPS_COPY_WHEN_TCP_FLAG_SET                            0x00000400
#define NDIS_GFT_OFFLOAD_CAPS_SAMPLE                                            0x00000800
#define NDIS_GFT_OFFLOAD_CAPS_META_ACTION_BEFORE_HEADER_TRANSPOSITION           0x00001000
#define NDIS_GFT_OFFLOAD_CAPS_META_ACTION_AFTER_HEADER_TRANSPOSITION            0x00002000
#define NDIS_GFT_OFFLOAD_CAPS_PER_VPORT_EXCEPTION_VPORT                         0x00004000
#define NDIS_GFT_OFFLOAD_CAPS_DESIGNATED_EXCEPTION_VPORT                        0x00008000
#define NDIS_GFT_OFFLOAD_CAPS_DSCP_MASK                                         0x00010000
#define NDIS_GFT_OFFLOAD_CAPS_8021P_PRIORITY_MASK                               0x00020000
#define NDIS_GFT_OFFLOAD_CAPS_ALLOW                                             0x00040000
#define NDIS_GFT_OFFLOAD_CAPS_DROP                                              0x00080000

//
// NDIS_GFT_OFFLOAD_CAPABILITIES structure advertises the GFT offload capabilities of
// the miniport adapter. This structure is used in NDIS_MINIPORT_ADAPTER_HARDWARE_ASSIST_ATTRIBUTES
//

#define NDIS_GFT_OFFLOAD_CAPABILITIES_REVISION_1            1

typedef struct _NDIS_GFT_OFFLOAD_CAPABILITIES
{
    NDIS_OBJECT_HEADER                  Header;
    ULONG                               Flags;
    ULONG                               CounterCapabilities;
    ULONG                               SupportedTableTypes;

    // NDIS_GFP_ENCAPSULATION_TYPE_xxx
    ULONG                               SupportedEncapsulationTypes;

    ULONG                               SupportedIngressExactMatchTableActions;
    ULONG                               SupportedEgressExactMatchTableActions;
    ULONG                               SoftwareSupportedIngressExactMatchTableActions;
    ULONG                               SoftwareSupportedEgressExactMatchTableActions;

    ULONG                               SupportedIngressWildcardMatchTableActions;
    ULONG                               SupportedEgressWildcardMatchTableActions;
    ULONG                               SoftwareSupportedIngressWildcardMatchTableActions;
    ULONG                               SoftwareSupportedEgressWildcardMatchTableActions;

    //
    // Number of discrete counter objects. does not include per flow entry counters that the provider
    // might support.
    //
    ULONG                               NumPacketCounterObjects;
    ULONG                               NumByteCounterObjects;
    ULONG                               NumPacketByteCounterObjects;
    ULONG                               NumPacketByteCounterAndStateObjects;
    ULONG                               NumCounterObjectsPerIngressExactMatchFlowEntry;
    ULONG                               NumCounterObjectsPerEgressExactMatchFlowEntry;
    ULONG                               NumCounterObjectsPerIngressWildcardMatchFlowEntry;
    ULONG                               NumCounterObjectsPerEgressWildcardMatchFlowEntry;
} NDIS_GFT_OFFLOAD_CAPABILITIES, *PNDIS_GFT_OFFLOAD_CAPABILITIES;

#define NDIS_SIZEOF_GFT_OFFLOAD_CAPABILITIES_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_OFFLOAD_CAPABILITIES, NumCounterObjectsPerEgressWildcardMatchFlowEntry)


//
// flags used in Flags field of NDIS_GFT_VPORT_PARAMETERS
//
#define NDIS_GFT_VPORT_ENABLE                               0x00000001
#define NDIS_GFT_VPORT_PARSE_VXLAN                          0x00000002
#define NDIS_GFT_VPORT_PARSE_VXLAN_NOT_IN_SRC_PORT_RANGE    0x00000004

#define NDIS_GFT_VPORT_ENABLE_STATE_CHANGED                 0x00100000
#define NDIS_GFT_VPORT_EXCEPTION_VPORT_CHANGED              0x00200000
#define NDIS_GFT_VPORT_SAMPLING_RATE_CHANGED                0x00400000
#define NDIS_GFT_VPORT_DSCP_MASK_CHANGED                    0x00800000
#define NDIS_GFT_VPORT_PRIORITY_MASK_CHANGED                0x01000000
#define NDIS_GFT_VPORT_VXLAN_SETTINGS_CHANGED               0x02000000
#define NDIS_GFT_VPORT_DSCP_FLAGS_CHANGED                   0x04000000
#define NDIS_GFT_VPORT_PARAMS_CHANGE_MASK                   0xFFF00000
#define NDIS_GFT_VPORT_PARAMS_CUSTOM_PROVIDER_RESERVED      0x000FF000

#define NDIS_GFT_VPORT_MAX_DSCP_MASK_COUNTER_OBJECTS        64
#define NDIS_GFT_VPORT_MAX_PRIORITY_MASK_COUNTER_OBJECTS    8


//
// Dscp flags used in DscpFlags field of NDIS_GFT_VPORT_PARAMETERS
//
#define NDIS_GFT_VPORT_DSCP_GUARD_ENABLE_RX                 0x00000001
#define NDIS_GFT_VPORT_DSCP_GUARD_ENABLE_TX                 0x00000002
#define NDIS_GFT_VPORT_DSCP_MASK_ENABLE_RX                  0x00000004
#define NDIS_GFT_VPORT_DSCP_MASK_ENABLE_TX                  0x00000008

//
// NDIS_GFT_VPORT_PARAMETERS is used in OID_GFT_VPORT_PARAMETERS
//

#define NDIS_GFT_VPORT_PARAMETERS_REVISION_1    1

typedef struct _NDIS_GFT_VPORT_PARAMETERS
{
    _In_ NDIS_OBJECT_HEADER                 Header;
    _In_ ULONG                              Flags;
    _In_ NDIS_NIC_SWITCH_VPORT_ID           VPortId;

    //
    // Copy and sample VPorts are the same as the exception VPort
    //
    _In_ NDIS_NIC_SWITCH_VPORT_ID           ExceptionVPortId;

    //
    // on query, sampling rate is an out parameter and returns the current sampling rate
    // on set, sampling rate is an "in" parameter
    // 0 means no sampling. Any value "n" other than 0 means one sample for every "n" packet
    //
    _Inout_ ULONG                           SamplingRate;

    _In_ ULONG64                            DscpMask;
    _In_ ULONG                              NumDscpMaskCounterObjects;
    _In_ NDIS_GFT_COUNTER_ID                DscpMaskCounterIdArray[NDIS_GFT_VPORT_MAX_DSCP_MASK_COUNTER_OBJECTS];

    _In_ ULONG64							PriorityMask;
    _In_ ULONG                              NumPriorityMaskCounterObjects;
    _In_ NDIS_GFT_COUNTER_ID                PriorityMaskCounterIdArray[NDIS_GFT_VPORT_MAX_PRIORITY_MASK_COUNTER_OBJECTS];
    _In_ USHORT                             VxLanSrcPortBase;
    _In_ USHORT                             VxLanSrcPortRange;
    _In_ ULONG                              DscpFlags;
    union {
        _In_ PVOID                          CustomProviderReservedPointer;
        _In_ ULONG64                        CustomProviderReservedValue;
    };
} NDIS_GFT_VPORT_PARAMETERS, *PNDIS_GFT_VPORT_PARAMETERS;

#define NDIS_SIZEOF_GFT_VPORT_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_GFT_VPORT_PARAMETERS, CustomProviderReservedValue)

//
// QOS Offload Scheduler Queue data structures
//
typedef ULONG NDIS_QOS_SQ_ID, *PNDIS_QOS_SQ_ID;
#define NDIS_QOS_DEFAULT_SQ_ID         0

//
// NDIS_QOS_SQ_TYPE specifies the type of SQ. Used with NDIS_QOS_SQ_PARAMETERS
// in OID_QOS_OFFLOAD_ENUM_SQS.
//
typedef enum _NDIS_QOS_SQ_TYPE
{
    NdisQosSqTypeUndefined,
    NdisQosSqTypeStandard,
    NdisQosSqTypeGFT,
    NdisQosSqTypeMax
} NDIS_QOS_SQ_TYPE, *PNDIS_QOS_SQ_TYPE;

//
// NDIS_QOS_SQ_PARAMETERS is used in OID_QOS_OFFLOAD_ENUM_SQS.
//
#define NDIS_QOS_SQ_PARAMETERS_REVISION_1        1
#if (NDIS_SUPPORT_NDIS684)
#define NDIS_QOS_SQ_PARAMETERS_REVISION_2        2
#endif // (NDIS_SUPPORT_NDIS684)

typedef struct _NDIS_QOS_SQ_PARAMETERS
{
    NDIS_OBJECT_HEADER  Header;
    ULONG               Flags;
    NDIS_QOS_SQ_ID      SqId;
    NDIS_QOS_SQ_TYPE    SqType;
    BOOLEAN             TcEnabledTable[NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES];
    ULONG               TcTransmitBandwidthCapTable[NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES];
    ULONG               TcTransmitBandwidthReservationTable[NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES];
    ULONG               TcReceiveBandwidthCapTable[NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES];
#if (NDIS_SUPPORT_NDIS684)
    ULONG               CrossTcTransmitBandwidthCap;
    ULONG               MaxNumSqInputs;
#endif // (NDIS_SUPPORT_NDIS684)
} NDIS_QOS_SQ_PARAMETERS, *PNDIS_QOS_SQ_PARAMETERS;

#define NDIS_SIZEOF_QOS_SQ_PARAMETERS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_QOS_SQ_PARAMETERS, TcReceiveBandwidthCapTable)
#if (NDIS_SUPPORT_NDIS684)
#define NDIS_SIZEOF_QOS_SQ_PARAMETERS_REVISION_2     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_QOS_SQ_PARAMETERS, MaxNumSqInputs)
#endif // (NDIS_SUPPORT_NDIS684)

//
// Used in NDIS_QOS_SQ_PARAMETERS Flags Field
//
#define NDIS_QOS_SQ_TRANSMIT_CAP_ENABLED            0x00000001
#define NDIS_QOS_SQ_TRANSMIT_RESERVATION_ENABLED    0x00000002
#define NDIS_QOS_SQ_RECEIVE_CAP_ENABLED             0x00000004


//
// NDIS_QOS_SQ_PARAMETERS_ARRAY name is deprecated in favor of
// the more generice NDIS_QOS_SQ_ARRAY name, since it's used
// for more than just OID_QOS_OFFLOAD_ENUM_SQS.
//
// Used in OID_QOS_OFFLOAD_ENUM_SQS and OID_QOS_OFFLOAD_SQ_STATS
//
#define NDIS_QOS_SQ_PARAMETERS_ARRAY_REVISION_1        1

#if (NDIS_SUPPORT_NDIS684)
#define NDIS_QOS_SQ_ARRAY_REVISION_1                   1
#endif // (NDIS_SUPPORT_NDIS684)

typedef struct _NDIS_QOS_SQ_PARAMETERS_ENUM_ARRAY
{
    NDIS_OBJECT_HEADER                 Header;
    ULONG                              Flags;
    NDIS_QOS_SQ_TYPE                   SqType;
    NDIS_QOS_SQ_ID                     FirstSqId;
    ULONG                              MaxSqsToReturn;
#if (NDIS_SUPPORT_NDIS684)
    union
    {
        ULONG                          SqArrayOffset;
        ULONG                          SqParamsArrayOffset;
    };
    union
    {
        ULONG                          SqArrayNumElements;
        ULONG                          SqParamsArrayNumElements;
    };
    union
    {
        ULONG                          SqArrayElementSize;
        ULONG                          SqParamsArrayElementSize;
    };
#else
    ULONG                              SqParamsArrayOffset;
    ULONG                              SqParamsArrayNumElements;
    ULONG                              SqParamsArrayElementSize;
#endif // (NDIS_SUPPORT_NDIS684)
} NDIS_QOS_SQ_PARAMETERS_ENUM_ARRAY, *PNDIS_QOS_SQ_PARAMETERS_ENUM_ARRAY;

#if (NDIS_SUPPORT_NDIS684)
typedef struct _NDIS_QOS_SQ_PARAMETERS_ENUM_ARRAY NDIS_QOS_SQ_ARRAY, *PNDIS_QOS_SQ_ARRAY;
#define NDIS_SIZEOF_QOS_SQ_ARRAY_REVISION_1                     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_QOS_SQ_ARRAY, SqArrayElementSize)
#endif // (NDIS_SUPPORT_NDIS684)

#define NDIS_SIZEOF_QOS_SQ_PARAMETERS_ENUM_ARRAY_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_QOS_SQ_PARAMETERS_ENUM_ARRAY, SqParamsArrayElementSize)


//
// NDIS_QOS_OFFLOAD_CAPABILITIES is used in the
// NDIS_MINIPORT_ADAPTER_HARDWARE_ASSIST_ATTRIBUTES structure and
// OID_QOS_OFFLOAD_HARDWARE_CAPABILITIES and
// OID_QOS_OFFLOAD_CURRENT_CAPABILITIES.
//
#define NDIS_QOS_OFFLOAD_CAPABILITIES_REVISION_1            1
#if (NDIS_SUPPORT_NDIS684)
#define NDIS_QOS_OFFLOAD_CAPABILITIES_REVISION_2            2
#endif // (NDIS_SUPPORT_NDIS684)

typedef struct _NDIS_QOS_OFFLOAD_CAPABILITIES
{
    NDIS_OBJECT_HEADER         Header;
    ULONG                      Flags;
    ULONG                      SupportedSqTypes;
    BOOLEAN                    TransmitCapSupported[NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES];
    BOOLEAN                    TransmitReservationSupported[NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES];
    BOOLEAN                    ReceiveCapSupported[NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES];
    BOOLEAN                    TransmitGftCapSupported[NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES];
    BOOLEAN                    ReceiveGftCapSupported[NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES];
    BOOLEAN                    TcSupportedTable[NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES];
    ULONG                      NumStandardSqsSupported;
    ULONG                      NumGftSqsSupported;
    ULONG                      ReservationGranularitySupported;
    ULONG                      MaxNumSqInputs;
#if (NDIS_SUPPORT_NDIS684)
    BOOLEAN                    CrossTcTransmitMaxCapSupported;
#endif // (NDIS_SUPPORT_NDIS684)
} NDIS_QOS_OFFLOAD_CAPABILITIES, *PNDIS_QOS_OFFLOAD_CAPABILITIES;

#define NDIS_SIZEOF_QOS_OFFLOAD_CAPABILITIES_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_QOS_OFFLOAD_CAPABILITIES, MaxNumSqInputs)
#if (NDIS_SUPPORT_NDIS684)
#define NDIS_SIZEOF_QOS_OFFLOAD_CAPABILITIES_REVISION_2     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_QOS_OFFLOAD_CAPABILITIES, CrossTcTransmitMaxCapSupported)
#endif // (NDIS_SUPPORT_NDIS684)

//
// Used in NDIS_QOS_OFFLOAD_CAPABILITIES SupportedSqTypes Field
//
#define NDIS_QOS_OFFLOAD_CAPS_STANDARD_SQ   0x00000001
#define NDIS_QOS_OFFLOAD_CAPS_GFT_SQ        0x00000002

#endif // (NDIS_SUPPORT_NDIS650)

#if (NDIS_SUPPORT_NDIS685)

#define NDIS_QOS_SQ_STATS_REVISION_1            1

typedef struct _NDIS_QOS_SQ_STATS
{
    NDIS_OBJECT_HEADER  Header;
    ULONG               Flags;
    NDIS_QOS_SQ_ID      SqId;
    NDIS_QOS_SQ_TYPE    SqType;
    UINT64              BytesTransmitted[NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES];
    UINT64              PktsTransmitted[NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES];
} NDIS_QOS_SQ_STATS, *PNDIS_QOS_SQ_STATS;

#define NDIS_SIZEOF_QOS_SQ_STATS_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_QOS_SQ_STATS, PktsTransmitted)

#endif // (NDIS_SUPPORT_NDIS685)

#if ((NTDDI_VERSION >= NTDDI_WIN10_RS5) || NDIS_SUPPORT_NDIS682)


#define NDIS_TIMESTAMP_CAPABILITIES_REVISION_1 1

typedef struct _NDIS_TIMESTAMP_CAPABILITY_FLAGS
{
    BOOLEAN PtpV2OverUdpIPv4EventMsgReceiveHw;
    BOOLEAN PtpV2OverUdpIPv4AllMsgReceiveHw;
    BOOLEAN PtpV2OverUdpIPv4EventMsgTransmitHw;
    BOOLEAN PtpV2OverUdpIPv4AllMsgTransmitHw;
    BOOLEAN PtpV2OverUdpIPv6EventMsgReceiveHw;
    BOOLEAN PtpV2OverUdpIPv6AllMsgReceiveHw;
    BOOLEAN PtpV2OverUdpIPv6EventMsgTransmitHw;
    BOOLEAN PtpV2OverUdpIPv6AllMsgTransmitHw;
    BOOLEAN AllReceiveHw;
    BOOLEAN AllTransmitHw;
    BOOLEAN TaggedTransmitHw;
    BOOLEAN AllReceiveSw;
    BOOLEAN AllTransmitSw;
    BOOLEAN TaggedTransmitSw;
} NDIS_TIMESTAMP_CAPABILITY_FLAGS, *PNDIS_TIMESTAMP_CAPABILITY_FLAGS;

typedef struct _NDIS_TIMESTAMP_CAPABILITIES
{
    NDIS_OBJECT_HEADER Header;

    ULONG64 HardwareClockFrequencyHz;
    BOOLEAN CrossTimestamp;
    ULONG64 Reserved1;
    ULONG64 Reserved2;
    NDIS_TIMESTAMP_CAPABILITY_FLAGS TimestampFlags;

} NDIS_TIMESTAMP_CAPABILITIES, *PNDIS_TIMESTAMP_CAPABILITIES;


#define NDIS_SIZEOF_TIMESTAMP_CAPABILITIES_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_TIMESTAMP_CAPABILITIES, TimestampFlags)


#define OID_TIMESTAMP_CAPABILITY 0x00A00001
#define OID_TIMESTAMP_CURRENT_CONFIG 0x00A00002

#define NDIS_HARDWARE_CROSSTIMESTAMP_REVISION_1 1

typedef struct _NDIS_HARDWARE_CROSSTIMESTAMP
{
    NDIS_OBJECT_HEADER Header;

    ULONG Flags;
    ULONG64 SystemTimestamp1;
    ULONG64 HardwareClockTimestamp;
    ULONG64 SystemTimestamp2;

} NDIS_HARDWARE_CROSSTIMESTAMP, *PNDIS_HARDWARE_CROSSTIMESTAMP;

#define NDIS_SIZEOF_HARDWARE_CROSSTIMESTAMP_REVISION_1     \
    RTL_SIZEOF_THROUGH_FIELD(NDIS_HARDWARE_CROSSTIMESTAMP, SystemTimestamp2)

#define OID_TIMESTAMP_GET_CROSSTIMESTAMP 0x00A00003


#endif //((NTDDI_VERSION >= NTDDI_WIN10_RS5) || NDIS_SUPPORT_NDIS682)



#if (NDIS_SUPPORT_NDIS685)

//
// OID_QUIC_CONNECTION_ENCRYPTION is a direct OID used for QUIC connection
// encryption offload.
//
#define OID_QUIC_CONNECTION_ENCRYPTION      0xFC010215

#endif // (NDIS_SUPPORT_NDIS685)

#ifdef __cplusplus
}
#endif

#if _MSC_VER >= 1200

#pragma warning(pop)

#else

#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // _NTDDNDIS_

