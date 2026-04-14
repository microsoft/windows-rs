/*++

    Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    windot11.h

Abstract:

    Definitions for native 802.11 miniport driver specifications.

--*/

#ifndef __WINDOT11_H__
#define __WINDOT11_H__

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

//
// Disable C4201: nonstandard extension used : nameless struct/union
//
#pragma warning(disable:4201)

//
// Disable C4214: nonstandard extension used : bit field types other than int
//
#pragma warning(disable:4214)

#ifndef _NTDDNDIS_
#include <ntddndis.h>
#endif

#include <WlanTypes.h>

// These are needed for wlanapi.h for pre-vista targets
#ifdef __midl
    typedef struct _DOT11_MAC_ADDRESS {
        UCHAR ucDot11MacAddress[6];
    } DOT11_MAC_ADDRESS, * PDOT11_MAC_ADDRESS;
#else
    typedef UCHAR DOT11_MAC_ADDRESS[6];
    typedef DOT11_MAC_ADDRESS * PDOT11_MAC_ADDRESS;
#endif

// A list of DOT11_MAC_ADDRESS
typedef struct DOT11_BSSID_LIST {
    #define DOT11_BSSID_LIST_REVISION_1  1
    NDIS_OBJECT_HEADER Header;
    ULONG uNumOfEntries;
    ULONG uTotalNumOfEntries;
#ifdef __midl
    [unique, size_is(uTotalNumOfEntries)] DOT11_MAC_ADDRESS BSSIDs[*];
#else
    DOT11_MAC_ADDRESS BSSIDs[1];
#endif
} DOT11_BSSID_LIST, * PDOT11_BSSID_LIST;

#define DOT11_HESSID_LENGTH   6
typedef UCHAR DOT11_HESSID[DOT11_HESSID_LENGTH];
typedef DOT11_HESSID* PDOT11_HESSID;

#ifdef __midl
// use 4-byte enum
typedef [v1_enum] enum _DOT11_PHY_TYPE {
#else
typedef enum _DOT11_PHY_TYPE {
#endif
    dot11_phy_type_unknown = 0,
    dot11_phy_type_any = dot11_phy_type_unknown,
    dot11_phy_type_fhss = 1,
    dot11_phy_type_dsss = 2,
    dot11_phy_type_irbaseband = 3,
    dot11_phy_type_ofdm = 4,                    // 11a
    dot11_phy_type_hrdsss = 5,                  // 11b
    dot11_phy_type_erp = 6,                     // 11g
    dot11_phy_type_ht = 7,                      // 11n
    dot11_phy_type_vht = 8,                     // 11ac
    dot11_phy_type_dmg = 9,                     // 11ad
    dot11_phy_type_he = 10,                     // 11ax
    dot11_phy_type_eht = 11,                    // 11be
    dot11_phy_type_IHV_start = 0x80000000,
    dot11_phy_type_IHV_end = 0xffffffff
} DOT11_PHY_TYPE, * PDOT11_PHY_TYPE;

// The AKMs are defined in the IEEE 802.11 spec in Table 9-188-AKM suite selectors
// The mappings are defined in Table 12-11-Integrity and key wrap algorithms
// The KCK length for SAE is defined in the IEEE 802.11 spec in Table 12-1-Hash algorithm based on length of prime

#define AKM_FROM_TYPE(_prefix, _akm) (_prefix + (_akm << 24))

#define RSNA_OUI_PREFIX             0xac0f00
typedef enum
{
    rsna_akm_none                   = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 0),
    rsna_akm_1x                     = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 1),        // 1X + PRF-128
    rsna_akm_psk                    = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 2),        // PSK + PRF-128
    rsna_akm_ft_1x_sha256           = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 3),        // FT_1X + SHA-256
    rsna_akm_ft_psk_sha256          = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 4),        // FT_PSK + SHA-256
    rsna_akm_1x_sha256              = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 5),        // 1X + SHA-256
    rsna_akm_psk_sha256             = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 6),        // PSK + SHA-256
    rsna_akm_tdls_sha256            = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 7),        // TPK + SHA-256
    rsna_akm_sae_pmk256             = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 8),        // SAE + [PMK = 256]
    rsna_akm_ft_sae_pmk256          = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 9),        // FT_SAE + [PMK = 256]
    rsna_akm_peerkey_sha256         = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 10),
    rsna_akm_1x_suite_b_sha256      = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 11),       // 1X_Suite_B + SHA-256
    rsna_akm_1x_suite_b_sha384      = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 12),       // 1X_Suite_B/CSNA + SHA-384
    rsna_akm_ft_1x_sha384_cmp_256   = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 13),       // FT_1X + SHA-384 + CCMP-256/GCMP-256
    rsna_akm_fils_1x_sha256         = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 14),
    rsna_akm_fils_1x_sha384         = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 15),
    rsna_akm_ft_fils_1x_sha256      = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 16),
    rsna_akm_ft_fils_sha384         = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 17),
    rsna_akm_owe                    = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 18),       // Reserved
    rsna_akm_ft_psk_sha384          = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 19),       // FT_PSK + SHA-384
    rsna_akm_psk_sha384             = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 20),       // PSK + SHA-384
                                                                                    // 21 is not defined
    rsna_akm_ft_1x_sha384           = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 22),       // FT_1X + SHA-384
    rsna_akm_1x_sha384              = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 23),       // 1X + SHA-384
    rsna_akm_sae_pmk384             = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 24),       // SAE + [PMK = 384]
    rsna_akm_ft_sae_pmk384          = AKM_FROM_TYPE(RSNA_OUI_PREFIX, 25),       // FT_SAE + [PMK = 384]

    rsna_akm_max = rsna_akm_ft_sae_pmk384,
} RSNA_AKM_SUITE;

#define WPA_OUI_PREFIX              0xf25000
typedef enum
{
    wpa_akm_none                    = AKM_FROM_TYPE(WPA_OUI_PREFIX, 0),
    wpa_akm_1x                      = AKM_FROM_TYPE(WPA_OUI_PREFIX, 1),         // 1X + PRF-128
    wpa_akm_psk                     = AKM_FROM_TYPE(WPA_OUI_PREFIX, 2),         // PSK + PRF-128

    wpa_akm_max = wpa_akm_psk,
} WPA_AKM_SUITE;

// The ciphers are defined in the IEEE 802.11 spec in Table 9-186-Cipher suite selectors
#define CIPHER_FROM_TYPE(_prefix, _cipher) (_prefix + (_cipher << 24))
typedef enum
{
    rsna_cipher_group               = CIPHER_FROM_TYPE(RSNA_OUI_PREFIX, 0),
    rsna_cipher_wep40               = CIPHER_FROM_TYPE(RSNA_OUI_PREFIX, 1),
    rsna_cipher_tkip                = CIPHER_FROM_TYPE(RSNA_OUI_PREFIX, 2),
    rsna_cipher_reserved            = CIPHER_FROM_TYPE(RSNA_OUI_PREFIX, 3),
    rsna_cipher_ccmp_128            = CIPHER_FROM_TYPE(RSNA_OUI_PREFIX, 4),
    rsna_cipher_wep104              = CIPHER_FROM_TYPE(RSNA_OUI_PREFIX, 5),
    rsna_cipher_bip_cmac_128        = CIPHER_FROM_TYPE(RSNA_OUI_PREFIX, 6),
    rsna_cipher_no_group_traffic    = CIPHER_FROM_TYPE(RSNA_OUI_PREFIX, 7),
    rsna_cipher_gcmp_128            = CIPHER_FROM_TYPE(RSNA_OUI_PREFIX, 8),
    rsna_cipher_gcmp_256            = CIPHER_FROM_TYPE(RSNA_OUI_PREFIX, 9),
    rsna_cipher_ccmp_256            = CIPHER_FROM_TYPE(RSNA_OUI_PREFIX, 10),
    rsna_cipher_bip_gmac_128        = CIPHER_FROM_TYPE(RSNA_OUI_PREFIX, 11),
    rsna_cipher_bip_gmac_256        = CIPHER_FROM_TYPE(RSNA_OUI_PREFIX, 12),
    rsna_cipher_bip_cmac_256        = CIPHER_FROM_TYPE(RSNA_OUI_PREFIX, 13),

    rsna_cipher_max = rsna_cipher_bip_cmac_256,
} RSNA_CIPHER_SUITE;

typedef enum
{
    wpa_cipher_none                 = CIPHER_FROM_TYPE(WPA_OUI_PREFIX, 0),
    wpa_cipher_wep40                = CIPHER_FROM_TYPE(WPA_OUI_PREFIX, 1),
    wpa_cipher_tkip                 = CIPHER_FROM_TYPE(WPA_OUI_PREFIX, 2),
    wpa_cipher_ccmp_128             = CIPHER_FROM_TYPE(WPA_OUI_PREFIX, 4),
    wpa_cipher_wep104               = CIPHER_FROM_TYPE(WPA_OUI_PREFIX, 5),
    wpa_cipher_bip_cmac_128         = CIPHER_FROM_TYPE(WPA_OUI_PREFIX, 6),

    wpa_cipher_max = wpa_cipher_bip_cmac_128,
} WPA_CIPHER_SUITE;

typedef struct
{
    RSNA_AKM_SUITE akm;
    RSNA_CIPHER_SUITE cipher;
} RSNA_AKM_CIPHER_PAIR;

#define DOT11_RATE_SET_MAX_LENGTH               126 // 126 bytes
typedef struct _DOT11_RATE_SET {
    _Field_range_(<=, DOT11_RATE_SET_MAX_LENGTH) ULONG uRateSetLength;
    _Field_size_part_(DOT11_RATE_SET_MAX_LENGTH, uRateSetLength) UCHAR ucRateSet[DOT11_RATE_SET_MAX_LENGTH];
} DOT11_RATE_SET, * PDOT11_RATE_SET;

typedef struct
{
    RSNA_AKM_SUITE akm;
    RSNA_CIPHER_SUITE cipher;
} DOT11_AKM_CIPHER_PAIR;

typedef UCHAR DOT11_COUNTRY_OR_REGION_STRING[3];
typedef DOT11_COUNTRY_OR_REGION_STRING * PDOT11_COUNTRY_OR_REGION_STRING;

typedef UCHAR DOT11_DIALOG_TOKEN;
typedef UCHAR DOT11_WFD_STATUS_CODE;
typedef UCHAR DOT11_WFD_MINOR_REASON_CODE;

typedef UCHAR DOT11_WFD_SERVICE_HASH[6];

#define DOT11_WFD_SERVICE_NAME_MAX_LENGTH 255
#define DOT11_WFD_APS2_SERVICE_TYPE_MAX_LENGTH 21
#define DOT11_WFD_ASP2_INSTANCE_NAME_MAX_LENGTH 63
#define DOT11_WFD_SERVICE_INFORMATION_MAX_LENGTH 65535
#define DOT11_MAX_REQUESTED_SERVICE_INFORMATION_LENGTH 255
#define DOT11_WFD_SESSION_INFO_MAX_LENGTH 144
typedef struct _DOT11_WFD_SESSION_INFO {
    _Field_range_(<= , DOT11_WFD_SESSION_INFO_MAX_LENGTH) USHORT uSessionInfoLength;
    _Field_size_part_(DOT11_WFD_SESSION_INFO_MAX_LENGTH, uSessionInfoLength) UCHAR ucSessionInfo[DOT11_WFD_SESSION_INFO_MAX_LENGTH];
} DOT11_WFD_SESSION_INFO, *PDOT11_WFD_SESSION_INFO;

#if (NTDDI_VERSION >= NTDDI_WIN8 || NDIS_SUPPORT_NDIS630)
#define NWF_WFD_SUPPORTED
#define NWF_POWER_SAVE_SUPPORTED
#endif // (NTDDI_VERSION >= NTDDI_WIN8 || NDIS_SUPPORT_NDIS630)


#if (NTDDI_VERSION >= NTDDI_WIN7 || NDIS_SUPPORT_NDIS620)
#define NWF_EXTAP_SUPPORTED
#define NWF_VWIFI_SUPPORTED
#endif // (NTDDI_VERSION >= NTDDI_WIN7 || NDIS_SUPPORT_NDIS620)


#if (NTDDI_VERSION >= NTDDI_VISTA)

// ntddndis.h: new Flags for OID_GEN_CURRENT_PACKET_FILTER
#define NDIS_PACKET_TYPE_MEDIA_SPECIFIC_MASK        (0x0fff0000U)
    // Mask for media specific packet filters

#define NDIS_PACKET_TYPE_802_11_DIRECTED_DATA       NDIS_PACKET_TYPE_DIRECTED

#define NDIS_PACKET_TYPE_802_11_BROADCAST_DATA      NDIS_PACKET_TYPE_BROADCAST

#define NDIS_PACKET_TYPE_802_11_MULTICAST_DATA      NDIS_PACKET_TYPE_MULTICAST

#define NDIS_PACKET_TYPE_802_11_ALL_MULTICAST_DATA  NDIS_PACKET_TYPE_ALL_MULTICAST

#define NDIS_PACKET_TYPE_802_11_PROMISCUOUS_DATA    NDIS_PACKET_TYPE_PROMISCUOUS

#define NDIS_PACKET_TYPE_802_11_RAW_DATA            (0x00010000U)
    // Raw 802.11 data packets (MPDU)

#define NDIS_PACKET_TYPE_802_11_DIRECTED_MGMT       (0x00020000U)
    // Directed management packet

#define NDIS_PACKET_TYPE_802_11_BROADCAST_MGMT      (0x00040000U)
    // Broadcast management packet

#define NDIS_PACKET_TYPE_802_11_MULTICAST_MGMT      (0x00080000U)
    // Multicast management packet

#define NDIS_PACKET_TYPE_802_11_ALL_MULTICAST_MGMT  (0x00100000U)
    // All-multicast management packet

#define NDIS_PACKET_TYPE_802_11_PROMISCUOUS_MGMT    (0x00200000U)
    // Promiscuous management packet

#define NDIS_PACKET_TYPE_802_11_RAW_MGMT            (0x00400000U)
    // Raw management packet

#define NDIS_PACKET_TYPE_802_11_DIRECTED_CTRL       (0x00800000U)
    // Directed CTRL packet

#define NDIS_PACKET_TYPE_802_11_BROADCAST_CTRL      (0x01000000U)
    // Broadcast CTRL packet

#define NDIS_PACKET_TYPE_802_11_PROMISCUOUS_CTRL    (0x02000000U)
    // Promiscuous CTRL packet

#define NDIS_PACKET_TYPE_ALL_802_11_FILTERS         \
    (NDIS_PACKET_TYPE_DIRECTED |                    \
     NDIS_PACKET_TYPE_MULTICAST |                   \
     NDIS_PACKET_TYPE_ALL_MULTICAST |               \
     NDIS_PACKET_TYPE_BROADCAST |                   \
     NDIS_PACKET_TYPE_PROMISCUOUS |                 \
     NDIS_PACKET_TYPE_802_11_RAW_DATA |             \
     NDIS_PACKET_TYPE_802_11_DIRECTED_MGMT |        \
     NDIS_PACKET_TYPE_802_11_BROADCAST_MGMT |       \
     NDIS_PACKET_TYPE_802_11_MULTICAST_MGMT |       \
     NDIS_PACKET_TYPE_802_11_ALL_MULTICAST_MGMT |   \
     NDIS_PACKET_TYPE_802_11_PROMISCUOUS_MGMT |     \
     NDIS_PACKET_TYPE_802_11_RAW_MGMT |             \
     NDIS_PACKET_TYPE_802_11_DIRECTED_CTRL |        \
     NDIS_PACKET_TYPE_802_11_BROADCAST_CTRL |       \
     NDIS_PACKET_TYPE_802_11_PROMISCUOUS_CTRL)


//
// Max size of an 802.11 PDU, including the MAC header, frame body and FCS.
//
#define DOT11_MAX_PDU_SIZE                          2346

//
// Min size of an 802.11 PDU, including the MAC header, frame body and FCS.
//
#define DOT11_MIN_PDU_SIZE                          (256)

#define DOT11_MAX_NUM_DEFAULT_KEY                   4

#if (NTDDI_VERSION >= NTDDI_WIN8 || NDIS_SUPPORT_NDIS630)
#define DOT11_MAX_NUM_DEFAULT_KEY_MFP               DOT11_MAX_NUM_DEFAULT_KEY + 2
#endif // (NTDDI_VERSION >= NTDDI_WIN8 || NDIS_SUPPORT_NDIS630)


// Macros for defining native 802.11 OIDs
#define OID_DOT11_NDIS_START                        0x0D010300

#define NWF_MANDATORY_OID       (0x01U)
#define NWF_OPTIONAL_OID        (0x02U)

#define NWF_OPERATIONAL_OID     (0x01U)
#define NWF_STATISTICS_OID      (0x02U)

#define NWF_DEFINE_OID(Seq,o,m)     ((0x0E000000U) | ((o) << 16) | ((m) << 8) | (Seq))

//
// Offload Capability OIDs
//

#define OID_DOT11_OFFLOAD_CAPABILITY                (OID_DOT11_NDIS_START + 0)
    // Capability flags
    #define DOT11_HW_WEP_SUPPORTED_TX               0x00000001
    #define DOT11_HW_WEP_SUPPORTED_RX               0x00000002
    #define DOT11_HW_FRAGMENTATION_SUPPORTED        0x00000004
    #define DOT11_HW_DEFRAGMENTATION_SUPPORTED      0x00000008
    #define DOT11_HW_MSDU_AUTH_SUPPORTED_TX         0x00000010
    #define DOT11_HW_MSDU_AUTH_SUPPORTED_RX         0x00000020
    // WEP Algorithm flags
    #define DOT11_CONF_ALGO_WEP_RC4                 0x00000001  // WEP RC4
    #define DOT11_CONF_ALGO_TKIP                    0x00000002
    // Integrity Algorithm flags
    #define DOT11_AUTH_ALGO_MICHAEL                 0x00000001  // Michael
    typedef struct _DOT11_OFFLOAD_CAPABILITY {
        ULONG uReserved;
        ULONG uFlags;
        ULONG uSupportedWEPAlgorithms;
        ULONG uNumOfReplayWindows;
        ULONG uMaxWEPKeyMappingLength;
        ULONG uSupportedAuthAlgorithms;
        ULONG uMaxAuthKeyMappingLength;
    } DOT11_OFFLOAD_CAPABILITY, * PDOT11_OFFLOAD_CAPABILITY;

#define OID_DOT11_CURRENT_OFFLOAD_CAPABILITY        (OID_DOT11_NDIS_START + 1)
    typedef struct _DOT11_CURRENT_OFFLOAD_CAPABILITY {
        ULONG uReserved;
        ULONG uFlags;
    } DOT11_CURRENT_OFFLOAD_CAPABILITY, * PDOT11_CURRENT_OFFLOAD_CAPABILITY;


//
// WEP Offload
//

#define OID_DOT11_WEP_OFFLOAD                       (OID_DOT11_NDIS_START + 2)
    typedef enum _DOT11_OFFLOAD_TYPE {
        dot11_offload_type_wep = 1,
        dot11_offload_type_auth = 2
    } DOT11_OFFLOAD_TYPE, * PDOT11_OFFLOAD_TYPE;
    typedef struct _DOT11_IV48_COUNTER {
        ULONG uIV32Counter;
        USHORT usIV16Counter;
    } DOT11_IV48_COUNTER, * PDOT11_IV48_COUNTER;
    typedef struct _DOT11_WEP_OFFLOAD {
        ULONG uReserved;
        HANDLE hOffloadContext;
        HANDLE hOffload;
        DOT11_OFFLOAD_TYPE dot11OffloadType;
        ULONG dwAlgorithm;
        BOOLEAN bRowIsOutbound;
        BOOLEAN bUseDefault;
        ULONG uFlags;
        UCHAR ucMacAddress[6];
        ULONG uNumOfRWsOnPeer;
        ULONG uNumOfRWsOnMe;
        DOT11_IV48_COUNTER dot11IV48Counters[16];
        USHORT usDot11RWBitMaps[16];
        USHORT usKeyLength;
        UCHAR ucKey[1];             // Must be the last field.
    } DOT11_WEP_OFFLOAD, * PDOT11_WEP_OFFLOAD;

#define OID_DOT11_WEP_UPLOAD                        (OID_DOT11_NDIS_START + 3)
    typedef struct _DOT11_WEP_UPLOAD {
        ULONG uReserved;
        DOT11_OFFLOAD_TYPE dot11OffloadType;
        HANDLE hOffload;
        ULONG uNumOfRWsUsed;
        DOT11_IV48_COUNTER dot11IV48Counters[16];
        USHORT usDot11RWBitMaps[16];
    } DOT11_WEP_UPLOAD, * PDOT11_WEP_UPLOAD;

#define OID_DOT11_DEFAULT_WEP_OFFLOAD               (OID_DOT11_NDIS_START + 4)
    typedef enum _DOT11_KEY_DIRECTION {
        dot11_key_direction_both = 1,
        dot11_key_direction_inbound = 2,
        dot11_key_direction_outbound = 3
    } DOT11_KEY_DIRECTION, * PDOT11_KEY_DIRECTION;
    typedef struct _DOT11_DEFAULT_WEP_OFFLOAD {
        ULONG uReserved;
        HANDLE hOffloadContext;
        HANDLE hOffload;
        ULONG  dwIndex;
        DOT11_OFFLOAD_TYPE dot11OffloadType;
        ULONG dwAlgorithm;
        ULONG uFlags;
        DOT11_KEY_DIRECTION dot11KeyDirection;
        UCHAR ucMacAddress[6];
        ULONG uNumOfRWsOnMe;
        DOT11_IV48_COUNTER dot11IV48Counters[16];
        USHORT usDot11RWBitMaps[16];
        USHORT usKeyLength;
        UCHAR ucKey[1];             // Must be the last field.
    } DOT11_DEFAULT_WEP_OFFLOAD, * PDOT11_DEFAULT_WEP_OFFLOAD;

#define OID_DOT11_DEFAULT_WEP_UPLOAD                (OID_DOT11_NDIS_START + 5)
    typedef struct _DOT11_DEFAULT_WEP_UPLOAD {
        ULONG uReserved;
        DOT11_OFFLOAD_TYPE dot11OffloadType;
        HANDLE hOffload;
        ULONG uNumOfRWsUsed;
        DOT11_IV48_COUNTER dot11IV48Counters[16];
        USHORT usDot11RWBitMaps[16];
    } DOT11_DEFAULT_WEP_UPLOAD, * PDOT11_DEFAULT_WEP_UPLOAD;

//
// Fragmentation/Defragmentation Offload
//

#define OID_DOT11_MPDU_MAX_LENGTH                   (OID_DOT11_NDIS_START + 6)
    // ULONG (in bytes)

//
// 802.11 Configuration OIDs
//

//
// OIDs for Mandatory Functions
//

#define OID_DOT11_OPERATION_MODE_CAPABILITY         (OID_DOT11_NDIS_START + 7)
    #define DOT11_OPERATION_MODE_UNKNOWN            0x00000000
    #define DOT11_OPERATION_MODE_STATION            0x00000001
    #define DOT11_OPERATION_MODE_AP                 0x00000002
    #define DOT11_OPERATION_MODE_EXTENSIBLE_STATION 0x00000004
    #define DOT11_OPERATION_MODE_EXTENSIBLE_AP      0x00000008
    #define DOT11_OPERATION_MODE_WFD_DEVICE         0x00000010
    #define DOT11_OPERATION_MODE_WFD_GROUP_OWNER    0x00000020
    #define DOT11_OPERATION_MODE_WFD_CLIENT         0x00000040
    #define DOT11_OPERATION_MODE_MANUFACTURING      0x40000000
    #define DOT11_OPERATION_MODE_NETWORK_MONITOR    0x80000000
    typedef struct _DOT11_OPERATION_MODE_CAPABILITY {
        ULONG uReserved;
        ULONG uMajorVersion;
        ULONG uMinorVersion;
        ULONG uNumOfTXBuffers;
        ULONG uNumOfRXBuffers;
        ULONG uOpModeCapability;
    } DOT11_OPERATION_MODE_CAPABILITY, * PDOT11_OPERATION_MODE_CAPABILITY;

#define OID_DOT11_CURRENT_OPERATION_MODE            (OID_DOT11_NDIS_START + 8)
    typedef struct _DOT11_CURRENT_OPERATION_MODE {
        ULONG uReserved;
        ULONG uCurrentOpMode;
    } DOT11_CURRENT_OPERATION_MODE, * PDOT11_CURRENT_OPERATION_MODE;

#define OID_DOT11_CURRENT_PACKET_FILTER             (OID_DOT11_NDIS_START + 9)
    #define DOT11_PACKET_TYPE_DIRECTED_CTRL         0x00000001
    // Indicate all 802.11 unicast control packets.
    #define DOT11_PACKET_TYPE_DIRECTED_MGMT         0x00000002
    // Indicate all 802.11 unicast management packets.
    #define DOT11_PACKET_TYPE_DIRECTED_DATA         0x00000004
    // Indicate all 802.11 unicast data packets.
    #define DOT11_PACKET_TYPE_MULTICAST_CTRL        0x00000008
    // Indicate all 802.11 multicast control packets.
    #define DOT11_PACKET_TYPE_MULTICAST_MGMT        0x00000010
    // Indicate all 802.11 multicast management packets.
    #define DOT11_PACKET_TYPE_MULTICAST_DATA        0x00000020
    // Indicate all 802.11 multicast data packets.
    #define DOT11_PACKET_TYPE_BROADCAST_CTRL        0x00000040
    // Indicate all 802.11 broadcast control packets.
    #define DOT11_PACKET_TYPE_BROADCAST_MGMT        0x00000080
    // Indicate all 802.11 broadcast management packets.
    #define DOT11_PACKET_TYPE_BROADCAST_DATA        0x00000100
    // Indicate all 802.11 broadcast data packets.
    #define DOT11_PACKET_TYPE_PROMISCUOUS_CTRL      0x00000200
    // Move into promiscuous mode and indicate all 802.11 control packets.
    #define DOT11_PACKET_TYPE_PROMISCUOUS_MGMT      0x00000400
    // Move into promiscuous mode and indicate all 802.11 control packets.
    #define DOT11_PACKET_TYPE_PROMISCUOUS_DATA      0x00000800
    // Move into promiscuous mode and indicate all 802.11 control packets.
    #define DOT11_PACKET_TYPE_ALL_MULTICAST_CTRL    0x00001000
    // Indicate all 802.11 multicast control packets.
    #define DOT11_PACKET_TYPE_ALL_MULTICAST_MGMT    0x00002000
    // Indicate all 802.11 multicast management packets.
    #define DOT11_PACKET_TYPE_ALL_MULTICAST_DATA    0x00004000
    // Indicate all 802.11 multicast data packets.
    #define DOT11_PACKET_TYPE_RESERVED  (~(             \
                DOT11_PACKET_TYPE_DIRECTED_CTRL |       \
                DOT11_PACKET_TYPE_DIRECTED_MGMT |       \
                DOT11_PACKET_TYPE_DIRECTED_DATA |       \
                DOT11_PACKET_TYPE_MULTICAST_CTRL |      \
                DOT11_PACKET_TYPE_MULTICAST_MGMT |      \
                DOT11_PACKET_TYPE_MULTICAST_DATA |      \
                DOT11_PACKET_TYPE_BROADCAST_CTRL |      \
                DOT11_PACKET_TYPE_BROADCAST_MGMT |      \
                DOT11_PACKET_TYPE_BROADCAST_DATA |      \
                DOT11_PACKET_TYPE_PROMISCUOUS_CTRL |    \
                DOT11_PACKET_TYPE_PROMISCUOUS_MGMT |    \
                DOT11_PACKET_TYPE_PROMISCUOUS_DATA |    \
                DOT11_PACKET_TYPE_ALL_MULTICAST_CTRL |  \
                DOT11_PACKET_TYPE_ALL_MULTICAST_MGMT |  \
                DOT11_PACKET_TYPE_ALL_MULTICAST_DATA |  \
                0                                       \
                ))
    // All the reserved bits

#define OID_DOT11_ATIM_WINDOW                       (OID_DOT11_NDIS_START + 10)
    // ULONG (in TUs)

#define OID_DOT11_SCAN_REQUEST                      (OID_DOT11_NDIS_START + 11)

    typedef enum _DOT11_SCAN_TYPE {
        dot11_scan_type_active = 1,
        dot11_scan_type_passive = 2,
        dot11_scan_type_auto = 3,
        dot11_scan_type_forced = 0x80000000
    } DOT11_SCAN_TYPE, * PDOT11_SCAN_TYPE;
    typedef struct _DOT11_SCAN_REQUEST {
        DOT11_BSS_TYPE dot11BSSType;
        DOT11_MAC_ADDRESS dot11BSSID;
        DOT11_SSID dot11SSID;
        DOT11_SCAN_TYPE dot11ScanType;
        BOOLEAN bRestrictedScan;
        BOOLEAN bUseRequestIE;
        ULONG uRequestIDsOffset;
        ULONG uNumOfRequestIDs;
        ULONG uPhyTypesOffset;
        ULONG uNumOfPhyTypes;
        ULONG uIEsOffset;
        ULONG uIEsLength;
        UCHAR ucBuffer[1];
    } DOT11_SCAN_REQUEST, * PDOT11_SCAN_REQUEST;

    // V2 SCAN REQUEST
    typedef enum _CH_DESCRIPTION_TYPE {
        ch_description_type_logical = 1,
        ch_description_type_center_frequency = 2,
        ch_description_type_phy_specific
    } CH_DESCRIPTION_TYPE, * PCH_DESCRIPTION_TYPE;
    typedef struct _DOT11_PHY_TYPE_INFO {
        DOT11_PHY_TYPE dot11PhyType;
        BOOLEAN bUseParameters;
        ULONG uProbeDelay;
        ULONG uMinChannelTime;
        ULONG uMaxChannelTime;
        CH_DESCRIPTION_TYPE ChDescriptionType;
        ULONG uChannelListSize;
        UCHAR ucChannelListBuffer[1];
    } DOT11_PHY_TYPE_INFO, * PDOT11_PHY_TYPE_INFO;

    typedef struct _DOT11_SCAN_REQUEST_V2 {
        DOT11_BSS_TYPE dot11BSSType;
        DOT11_MAC_ADDRESS dot11BSSID;
        DOT11_SCAN_TYPE dot11ScanType;
        BOOLEAN bRestrictedScan;
        ULONG udot11SSIDsOffset;
        ULONG uNumOfdot11SSIDs;
        BOOLEAN bUseRequestIE;
        ULONG uRequestIDsOffset;
        ULONG uNumOfRequestIDs;
        ULONG uPhyTypeInfosOffset;
        ULONG uNumOfPhyTypeInfos;
        ULONG uIEsOffset;
        ULONG uIEsLength;
        UCHAR ucBuffer[1];
    } DOT11_SCAN_REQUEST_V2, * PDOT11_SCAN_REQUEST_V2;

#define OID_DOT11_CURRENT_PHY_TYPE                  (OID_DOT11_NDIS_START + 12)
    typedef struct DOT11_PHY_TYPE_LIST {
        #define DOT11_PHY_TYPE_LIST_REVISION_1          1
        NDIS_OBJECT_HEADER Header;
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        DOT11_PHY_TYPE dot11PhyType[1];
    } DOT11_PHY_TYPE_LIST, * PDOT11_PHY_TYPE_LIST;

#define OID_DOT11_JOIN_REQUEST                      (OID_DOT11_NDIS_START + 13)

    // Capability Information Flags - Exactly maps to the bit positions
    // in the Capability Information field of the beacon and probe response frames.
    #define DOT11_CAPABILITY_INFO_ESS               0x0001
    #define DOT11_CAPABILITY_INFO_IBSS              0x0002
    #define DOT11_CAPABILITY_INFO_CF_POLLABLE       0x0004
    #define DOT11_CAPABILITY_INFO_CF_POLL_REQ       0x0008
    #define DOT11_CAPABILITY_INFO_PRIVACY           0x0010
    #define DOT11_CAPABILITY_SHORT_PREAMBLE         0x0020
    #define DOT11_CAPABILITY_PBCC                   0x0040
    #define DOT11_CAPABILITY_CHANNEL_AGILITY        0x0080
    #define DOT11_CAPABILITY_SHORT_SLOT_TIME        0x0400
    #define DOT11_CAPABILITY_DSSSOFDM               0x2000

    typedef struct _DOT11_BSS_DESCRIPTION {
        ULONG uReserved;                        // Passed-in as 0 and must be ignored for now.
        DOT11_MAC_ADDRESS dot11BSSID;
        DOT11_BSS_TYPE dot11BSSType;
        USHORT usBeaconPeriod;
        ULONGLONG ullTimestamp;
        USHORT usCapabilityInformation;
        ULONG uBufferLength;
#ifdef __midl
        [unique, size_is(uBufferLength)] UCHAR ucBuffer[*];
#else
        UCHAR ucBuffer[1];              // Must be the last field.
#endif
    } DOT11_BSS_DESCRIPTION, * PDOT11_BSS_DESCRIPTION;
    typedef struct _DOT11_JOIN_REQUEST {
        ULONG uJoinFailureTimeout;
        DOT11_RATE_SET OperationalRateSet;
        ULONG uChCenterFrequency;
        DOT11_BSS_DESCRIPTION dot11BSSDescription;  // Must be the last field.
    } DOT11_JOIN_REQUEST, * PDOT11_JOIN_REQUEST;

#define OID_DOT11_START_REQUEST                     (OID_DOT11_NDIS_START + 14)
    typedef struct _DOT11_START_REQUEST {
        ULONG uStartFailureTimeout;
        DOT11_RATE_SET OperationalRateSet;
        ULONG uChCenterFrequency;
        DOT11_BSS_DESCRIPTION dot11BSSDescription;  // Must be the last field.
    } DOT11_START_REQUEST, * PDOT11_START_REQUEST;

#define OID_DOT11_UPDATE_IE                         (OID_DOT11_NDIS_START + 15)
typedef enum _DOT11_UPDATE_IE_OP {
    dot11_update_ie_op_create_replace = 1,
    dot11_update_ie_op_delete = 2,
} DOT11_UPDATE_IE_OP, * PDOT11_UPDATE_IE_OP;

typedef struct _DOT11_UPDATE_IE {
    DOT11_UPDATE_IE_OP dot11UpdateIEOp;
    ULONG uBufferLength;
    UCHAR ucBuffer[1];          // Must be the last field.
} DOT11_UPDATE_IE, * PDOT11_UPDATE_IE;

#define OID_DOT11_RESET_REQUEST                     (OID_DOT11_NDIS_START + 16)
    typedef enum _DOT11_RESET_TYPE {
        dot11_reset_type_phy = 1,
        dot11_reset_type_mac = 2,
        dot11_reset_type_phy_and_mac = 3
    } DOT11_RESET_TYPE, * PDOT11_RESET_TYPE;
    typedef struct _DOT11_RESET_REQUEST {
        DOT11_RESET_TYPE dot11ResetType;
        DOT11_MAC_ADDRESS dot11MacAddress;
        BOOLEAN bSetDefaultMIB;
    } DOT11_RESET_REQUEST, * PDOT11_RESET_REQUEST;

#define OID_DOT11_NIC_POWER_STATE                   (OID_DOT11_NDIS_START + 17)
    // BOOL

//
// OIDs for Optional Functions
//

#define OID_DOT11_OPTIONAL_CAPABILITY               (OID_DOT11_NDIS_START + 18)
    typedef struct _DOT11_OPTIONAL_CAPABILITY {
        ULONG uReserved;
        BOOLEAN bDot11PCF;
        BOOLEAN bDot11PCFMPDUTransferToPC;
        BOOLEAN bStrictlyOrderedServiceClass;
    } DOT11_OPTIONAL_CAPABILITY, * PDOT11_OPTIONAL_CAPABILITY;

#define OID_DOT11_CURRENT_OPTIONAL_CAPABILITY       (OID_DOT11_NDIS_START + 19)
    typedef struct _DOT11_CURRENT_OPTIONAL_CAPABILITY {
        ULONG uReserved;
        BOOLEAN bDot11CFPollable;
        BOOLEAN bDot11PCF;
        BOOLEAN bDot11PCFMPDUTransferToPC;
        BOOLEAN bStrictlyOrderedServiceClass;
    } DOT11_CURRENT_OPTIONAL_CAPABILITY, * PDOT11_CURRENT_OPTIONAL_CAPABILITY;

//
// 802.11 MIB OIDs
//

//
// OIDs for dot11StationConfigEntry
//

#define OID_DOT11_STATION_ID                        (OID_DOT11_NDIS_START + 20)
    // DOT11_MAC_ADDRESS

#define OID_DOT11_MEDIUM_OCCUPANCY_LIMIT            (OID_DOT11_NDIS_START + 21)
    // ULONG (in TUs)

#define OID_DOT11_CF_POLLABLE                       (OID_DOT11_NDIS_START + 22)
    // BOOL

#define OID_DOT11_CFP_PERIOD                        (OID_DOT11_NDIS_START + 23)
    // ULONG (in DTIM intervals)

#define OID_DOT11_CFP_MAX_DURATION                  (OID_DOT11_NDIS_START + 24)
    // ULONG (in TUs)

#define OID_DOT11_POWER_MGMT_MODE                   (OID_DOT11_NDIS_START + 25)
    typedef enum _DOT11_POWER_MODE {
        dot11_power_mode_unknown = 0,
        dot11_power_mode_active = 1,
        dot11_power_mode_powersave = 2
    } DOT11_POWER_MODE, * PDOT11_POWER_MODE;
    #define DOT11_POWER_SAVE_LEVEL_MAX_PSP      1
    // Maximum power save polling.
    #define DOT11_POWER_SAVE_LEVEL_FAST_PSP     2
    // Fast power save polling.
    typedef struct _DOT11_POWER_MGMT_MODE {
        DOT11_POWER_MODE dot11PowerMode;
        ULONG uPowerSaveLevel;
        USHORT usListenInterval;
        USHORT usAID;
        BOOLEAN bReceiveDTIMs;
    } DOT11_POWER_MGMT_MODE, * PDOT11_POWER_MGMT_MODE;

#define OID_DOT11_OPERATIONAL_RATE_SET              (OID_DOT11_NDIS_START + 26)
    // DOT11_RATE_SET

#define OID_DOT11_BEACON_PERIOD                     (OID_DOT11_NDIS_START + 27)
    // ULONG (in TUs)

#define OID_DOT11_DTIM_PERIOD                       (OID_DOT11_NDIS_START + 28)
    // ULONG (in beacon intervals)

//
// OIDs for Dot11PrivacyEntry
//

#define OID_DOT11_WEP_ICV_ERROR_COUNT               (OID_DOT11_NDIS_START + 29)
    // ULONG

//
// OIDs for dot11OperationEntry
//

#define OID_DOT11_MAC_ADDRESS                       (OID_DOT11_NDIS_START + 30)
    // DOT11_MAC_ADDRESS

#define OID_DOT11_RTS_THRESHOLD                     (OID_DOT11_NDIS_START + 31)
    // ULONG (in number of octets)

#define OID_DOT11_SHORT_RETRY_LIMIT                 (OID_DOT11_NDIS_START + 32)
    // ULONG

#define OID_DOT11_LONG_RETRY_LIMIT                  (OID_DOT11_NDIS_START + 33)
    // ULONG

#define OID_DOT11_FRAGMENTATION_THRESHOLD           (OID_DOT11_NDIS_START + 34)
    // ULONG (in number of octets)

#define OID_DOT11_MAX_TRANSMIT_MSDU_LIFETIME        (OID_DOT11_NDIS_START + 35)
    // ULONG (in TUs)

#define OID_DOT11_MAX_RECEIVE_LIFETIME              (OID_DOT11_NDIS_START + 36)
    // ULONG (in TUs)

//
// OIDs for dot11CountersEntry
//

#define OID_DOT11_COUNTERS_ENTRY                    (OID_DOT11_NDIS_START + 37)
    typedef struct _DOT11_COUNTERS_ENTRY {
        ULONG uTransmittedFragmentCount;
        ULONG uMulticastTransmittedFrameCount;
        ULONG uFailedCount;
        ULONG uRetryCount;
        ULONG uMultipleRetryCount;
        ULONG uFrameDuplicateCount;
        ULONG uRTSSuccessCount;
        ULONG uRTSFailureCount;
        ULONG uACKFailureCount;
        ULONG uReceivedFragmentCount;
        ULONG uMulticastReceivedFrameCount;
        ULONG uFCSErrorCount;
        ULONG uTransmittedFrameCount;
    } DOT11_COUNTERS_ENTRY, * PDOT11_COUNTERS_ENTRY;

//
// OIDs for dot11PhyOperationEntry
//

#define OID_DOT11_SUPPORTED_PHY_TYPES               (OID_DOT11_NDIS_START + 38)
    typedef struct _DOT11_SUPPORTED_PHY_TYPES {
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        DOT11_PHY_TYPE dot11PHYType[1];
    } DOT11_SUPPORTED_PHY_TYPES, * PDOT11_SUPPORTED_PHY_TYPES;

#define OID_DOT11_CURRENT_REG_DOMAIN                (OID_DOT11_NDIS_START + 39)
    #define DOT11_REG_DOMAIN_OTHER                  0x00000000
    #define DOT11_REG_DOMAIN_FCC                    0x00000010
    #define DOT11_REG_DOMAIN_DOC                    0x00000020
    #define DOT11_REG_DOMAIN_ETSI                   0x00000030
    #define DOT11_REG_DOMAIN_SPAIN                  0x00000031
    #define DOT11_REG_DOMAIN_FRANCE                 0x00000032
    #define DOT11_REG_DOMAIN_MKK                    0x00000040
    // ULONG

#define OID_DOT11_TEMP_TYPE                         (OID_DOT11_NDIS_START + 40)
    typedef enum _DOT11_TEMP_TYPE {
        dot11_temp_type_unknown = 0,
        dot11_temp_type_1 = 1,
        dot11_temp_type_2 = 2
    } DOT11_TEMP_TYPE, * PDOT11_TEMP_TYPE;

//
// OIDs for dot11PhyAntennaEntry
//

#define OID_DOT11_CURRENT_TX_ANTENNA                (OID_DOT11_NDIS_START + 41)
    // ULONG

#define OID_DOT11_DIVERSITY_SUPPORT                 (OID_DOT11_NDIS_START + 42)
    typedef enum _DOT11_DIVERSITY_SUPPORT {
        dot11_diversity_support_unknown = 0,
        dot11_diversity_support_fixedlist = 1,
        dot11_diversity_support_notsupported = 2,
        dot11_diversity_support_dynamic = 3
    } DOT11_DIVERSITY_SUPPORT, * PDOT11_DIVERSITY_SUPPORT;

#define OID_DOT11_CURRENT_RX_ANTENNA                (OID_DOT11_NDIS_START + 43)
    // ULONG

//
// OIDs for dot11PhyTxPowerEntry
//

#define OID_DOT11_SUPPORTED_POWER_LEVELS            (OID_DOT11_NDIS_START + 44)
    typedef struct _DOT11_SUPPORTED_POWER_LEVELS {
        ULONG uNumOfSupportedPowerLevels;
        _Field_size_part_(8, uNumOfSupportedPowerLevels) ULONG uTxPowerLevelValues[8];
    } DOT11_SUPPORTED_POWER_LEVELS, * PDOT11_SUPPORTED_POWER_LEVELS;

#define OID_DOT11_CURRENT_TX_POWER_LEVEL            (OID_DOT11_NDIS_START + 45)
    // ULONG

//
// OIDs for dot11PhyFHSSEntry
//

#define OID_DOT11_HOP_TIME                          (OID_DOT11_NDIS_START + 46)
    // ULONG (in microseconds)

#define OID_DOT11_CURRENT_CHANNEL_NUMBER            (OID_DOT11_NDIS_START + 47)
    // ULONG

#define OID_DOT11_MAX_DWELL_TIME                    (OID_DOT11_NDIS_START + 48)
    // ULONG (in TUs)

#define OID_DOT11_CURRENT_DWELL_TIME                (OID_DOT11_NDIS_START + 49)
    // ULONG (in TUs)

#define OID_DOT11_CURRENT_SET                       (OID_DOT11_NDIS_START + 50)
    // ULONG

#define OID_DOT11_CURRENT_PATTERN                   (OID_DOT11_NDIS_START + 51)
    // ULONG

#define OID_DOT11_CURRENT_INDEX                     (OID_DOT11_NDIS_START + 52)
    // ULONG

//
// OIDs for dot11PhyDSSSEntry
//

#define OID_DOT11_CURRENT_CHANNEL                   (OID_DOT11_NDIS_START + 53)
    // ULONG

#define OID_DOT11_CCA_MODE_SUPPORTED                (OID_DOT11_NDIS_START + 54)
    #define DOT11_CCA_MODE_ED_ONLY                  0x00000001
    #define DOT11_CCA_MODE_CS_ONLY                  0x00000002
    #define DOT11_CCA_MODE_ED_and_CS                0x00000004
    #define DOT11_CCA_MODE_CS_WITH_TIMER            0x00000008
    #define DOT11_CCA_MODE_HRCS_AND_ED              0x00000010

    // ULONG

#define OID_DOT11_CURRENT_CCA_MODE                  (OID_DOT11_NDIS_START + 55)
    // ULONG

#define OID_DOT11_ED_THRESHOLD                      (OID_DOT11_NDIS_START + 56)
    // LONG (in "dBm"s)

//
// OIDs for dot11PhyIREntry
//

#define OID_DOT11_CCA_WATCHDOG_TIMER_MAX            (OID_DOT11_NDIS_START + 57)
    // ULONG (in nanoseconds)

#define OID_DOT11_CCA_WATCHDOG_COUNT_MAX            (OID_DOT11_NDIS_START + 58)
    // ULONG

#define OID_DOT11_CCA_WATCHDOG_TIMER_MIN            (OID_DOT11_NDIS_START + 59)
    // ULONG (in nanoseconds)

#define OID_DOT11_CCA_WATCHDOG_COUNT_MIN            (OID_DOT11_NDIS_START + 60)
    // ULONG

//
// OIDs for dot11RegDomainsSupportEntry
//

#define OID_DOT11_REG_DOMAINS_SUPPORT_VALUE         (OID_DOT11_NDIS_START + 61)
    typedef struct _DOT11_REG_DOMAIN_VALUE {
        ULONG uRegDomainsSupportIndex;
        ULONG uRegDomainsSupportValue;
    } DOT11_REG_DOMAIN_VALUE, * PDOT11_REG_DOMAIN_VALUE;
    typedef struct _DOT11_REG_DOMAINS_SUPPORT_VALUE {
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        DOT11_REG_DOMAIN_VALUE dot11RegDomainValue[1];
    } DOT11_REG_DOMAINS_SUPPORT_VALUE, * PDOT11_REG_DOMAINS_SUPPORT_VALUE;

//
// OIDs for dot11AntennaListEntry
//

#define OID_DOT11_SUPPORTED_TX_ANTENNA              (OID_DOT11_NDIS_START + 62)
    typedef struct _DOT11_SUPPORTED_ANTENNA {
        ULONG uAntennaListIndex;                    // Between 1 and 255.
        BOOLEAN bSupportedAntenna;
    } DOT11_SUPPORTED_ANTENNA, * PDOT11_SUPPORTED_ANTENNA;
    typedef struct _DOT11_SUPPORTED_ANTENNA_LIST {
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        DOT11_SUPPORTED_ANTENNA dot11SupportedAntenna[1];
    } DOT11_SUPPORTED_ANTENNA_LIST, * PDOT11_SUPPORTED_ANTENNA_LIST;

#define OID_DOT11_SUPPORTED_RX_ANTENNA              (OID_DOT11_NDIS_START + 63)
    // DOT11_SUPPORTED_ANTENNA_LIST

#define OID_DOT11_DIVERSITY_SELECTION_RX            (OID_DOT11_NDIS_START + 64)
    typedef struct _DOT11_DIVERSITY_SELECTION_RX {
        ULONG uAntennaListIndex;                    // Between 1 and 255.
        BOOLEAN bDiversitySelectionRX;
    } DOT11_DIVERSITY_SELECTION_RX, * PDOT11_DIVERSITY_SELECTION_RX;
    typedef struct _DOT11_DIVERSITY_SELECTION_RX_LIST {
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        DOT11_DIVERSITY_SELECTION_RX dot11DiversitySelectionRx[1];
    } DOT11_DIVERSITY_SELECTION_RX_LIST, * PDOT11_DIVERSITY_SELECTION_RX_LIST;

//
// OIDs for dot11SupportedDataRatesTxEntry and dot11SupportedDataRatesRxEntry
//

#define OID_DOT11_SUPPORTED_DATA_RATES_VALUE        (OID_DOT11_NDIS_START + 65)
    #define MAX_NUM_SUPPORTED_RATES                 8       // 8 data rates
    #define MAX_NUM_SUPPORTED_RATES_V2              255     // 255 data rates
    typedef struct _DOT11_SUPPORTED_DATA_RATES_VALUE {
        UCHAR ucSupportedTxDataRatesValue[MAX_NUM_SUPPORTED_RATES];
        UCHAR ucSupportedRxDataRatesValue[MAX_NUM_SUPPORTED_RATES];
    } DOT11_SUPPORTED_DATA_RATES_VALUE, * PDOT11_SUPPORTED_DATA_RATES_VALUE;

    typedef struct _DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
        UCHAR ucSupportedTxDataRatesValue[MAX_NUM_SUPPORTED_RATES_V2];
        UCHAR ucSupportedRxDataRatesValue[MAX_NUM_SUPPORTED_RATES_V2];
    } DOT11_SUPPORTED_DATA_RATES_VALUE_V2, * PDOT11_SUPPORTED_DATA_RATES_VALUE_V2;

    // keep the incorrect struct name to avoid build break
    typedef DOT11_SUPPORTED_DATA_RATES_VALUE_V2
        DOT11_SUPPORTED_DATA_RATES_VALUE_V1, * PDOT11_SUPPORTED_DATA_RATES_VALUE_V1;

//
// OIDs for dot11PhyOFDMEntry
//

#define OID_DOT11_CURRENT_FREQUENCY                 (OID_DOT11_NDIS_START + 66)
    // ULONG

#define OID_DOT11_TI_THRESHOLD                      (OID_DOT11_NDIS_START + 67)
    // LONG

#define OID_DOT11_FREQUENCY_BANDS_SUPPORTED         (OID_DOT11_NDIS_START + 68)
    #define DOT11_FREQUENCY_BANDS_LOWER    0x00000001
    #define DOT11_FREQUENCY_BANDS_MIDDLE   0x00000002
    #define DOT11_FREQUENCY_BANDS_UPPER    0x00000004
    // ULONG

//
// OIDs for dot11PhyHRDSSSEntry
//

#define OID_DOT11_SHORT_PREAMBLE_OPTION_IMPLEMENTED (OID_DOT11_NDIS_START + 69)
    // BOOL

#define OID_DOT11_PBCC_OPTION_IMPLEMENTED           (OID_DOT11_NDIS_START + 70)
    // BOOL

#define OID_DOT11_CHANNEL_AGILITY_PRESENT           (OID_DOT11_NDIS_START + 71)
    // BOOL

#define OID_DOT11_CHANNEL_AGILITY_ENABLED           (OID_DOT11_NDIS_START + 72)
    // BOOL

#define OID_DOT11_HR_CCA_MODE_SUPPORTED             (OID_DOT11_NDIS_START + 73)
    // HR-CCA mode bits
    #define DOT11_HR_CCA_MODE_ED_ONLY        0x00000001
    #define DOT11_HR_CCA_MODE_CS_ONLY        0x00000002
    #define DOT11_HR_CCA_MODE_CS_AND_ED      0x00000004
    #define DOT11_HR_CCA_MODE_CS_WITH_TIMER  0x00000008
    #define DOT11_HR_CCA_MODE_HRCS_AND_ED    0x00000010
    // ULONG


//
// OIDs for dot11StationConfigEntry (Cont)
//

#define OID_DOT11_MULTI_DOMAIN_CAPABILITY_IMPLEMENTED   (OID_DOT11_NDIS_START + 74)
    // BOOL

#define OID_DOT11_MULTI_DOMAIN_CAPABILITY_ENABLED       (OID_DOT11_NDIS_START + 75)
    // BOOL

#define OID_DOT11_COUNTRY_STRING                        (OID_DOT11_NDIS_START + 76)
    // UCHAR[3]

//
// OIDs for dot11MultiDomainCapabilityEntry
//

typedef struct _DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    ULONG uMultiDomainCapabilityIndex;
    ULONG uFirstChannelNumber;
    ULONG uNumberOfChannels;
    LONG lMaximumTransmitPowerLevel;
} DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY, *PDOT11_MULTI_DOMAIN_CAPABILITY_ENTRY;
typedef struct _DOT11_MD_CAPABILITY_ENTRY_LIST {
    ULONG uNumOfEntries;
    ULONG uTotalNumOfEntries;
    DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY dot11MDCapabilityEntry[1];
} DOT11_MD_CAPABILITY_ENTRY_LIST, *PDOT11_MD_CAPABILITY_ENTRY_LIST;


#define OID_DOT11_MULTI_DOMAIN_CAPABILITY           (OID_DOT11_NDIS_START + 77)
    // DOT11_MD_CAPABILITY_ENTRY_LIST

//
// OIDs for dot11PhyFHSSEntry
//

#define OID_DOT11_EHCC_PRIME_RADIX                  (OID_DOT11_NDIS_START + 78)
    // ULONG

#define OID_DOT11_EHCC_NUMBER_OF_CHANNELS_FAMILY_INDEX  (OID_DOT11_NDIS_START + 79)
    // ULONG

#define OID_DOT11_EHCC_CAPABILITY_IMPLEMENTED       (OID_DOT11_NDIS_START + 80)
    // BOOL

#define OID_DOT11_EHCC_CAPABILITY_ENABLED           (OID_DOT11_NDIS_START + 81)
    // BOOL

#define OID_DOT11_HOP_ALGORITHM_ADOPTED             (OID_DOT11_NDIS_START + 82)
    typedef enum _DOT11_HOP_ALGO_ADOPTED {
        dot11_hop_algo_current = 0,
        dot11_hop_algo_hop_index = 1,
        dot11_hop_algo_hcc = 2
    } DOT11_HOP_ALGO_ADOPTED, * PDOT11_HOP_ALGO_ADOPTED;

#define OID_DOT11_RANDOM_TABLE_FLAG                 (OID_DOT11_NDIS_START + 83)
    // BOOL

#define OID_DOT11_NUMBER_OF_HOPPING_SETS            (OID_DOT11_NDIS_START + 84)
    // ULONG

#define OID_DOT11_HOP_MODULUS                       (OID_DOT11_NDIS_START + 85)
    // ULONG

#define OID_DOT11_HOP_OFFSET                        (OID_DOT11_NDIS_START + 86)
    // ULONG


//
// OIDs for dot11HoppingPatternEntry
//
#define OID_DOT11_HOPPING_PATTERN                   (OID_DOT11_NDIS_START + 87)
typedef struct _DOT11_HOPPING_PATTERN_ENTRY {
    ULONG uHoppingPatternIndex;
    ULONG uRandomTableFieldNumber;
} DOT11_HOPPING_PATTERN_ENTRY, *PDOT11_HOPPING_PATTERN_ENTRY;
typedef struct _DOT11_HOPPING_PATTERN_ENTRY_LIST {
    ULONG uNumOfEntries;
    ULONG uTotalNumOfEntries;
    DOT11_HOPPING_PATTERN_ENTRY dot11HoppingPatternEntry[1];
} DOT11_HOPPING_PATTERN_ENTRY_LIST, *PDOT11_HOPPING_PATTERN_ENTRY_LIST;


#define OID_DOT11_RANDOM_TABLE_FIELD_NUMBER         (OID_DOT11_NDIS_START + 88)
    // ULONG

//
// WPA Extensions
//

#define OID_DOT11_WPA_TSC                           (OID_DOT11_NDIS_START + 89)
typedef struct _DOT11_WPA_TSC {
    ULONG uReserved;
    DOT11_OFFLOAD_TYPE dot11OffloadType;
    HANDLE hOffload;
    DOT11_IV48_COUNTER dot11IV48Counter;
} DOT11_WPA_TSC, * PDOT11_WPA_TSC;

//
// dot11.
//

#define OID_DOT11_RSSI_RANGE                        (OID_DOT11_NDIS_START + 90)
typedef struct _DOT11_RSSI_RANGE {
    DOT11_PHY_TYPE dot11PhyType;
    ULONG uRSSIMin; // Minimum caliberation value of RSSI in the NIC.
    ULONG uRSSIMax; // Maximum caliberation value of RSSI in the NIC.
} DOT11_RSSI_RANGE, * PDOT11_RSSI_RANGE;

#define OID_DOT11_RF_USAGE                          (OID_DOT11_NDIS_START + 91)
//ULONG

#define OID_DOT11_NIC_SPECIFIC_EXTENSION            (OID_DOT11_NDIS_START + 92)
typedef struct _DOT11_NIC_SPECIFIC_EXTENSION {
    ULONG uBufferLength;
    ULONG uTotalBufferLength;
    UCHAR ucBuffer[1];
} DOT11_NIC_SPECIFIC_EXTENSION, * PDOT11_NIC_SPECIFIC_EXTENSION;

//
// AP join request
//

#define OID_DOT11_AP_JOIN_REQUEST                   (OID_DOT11_NDIS_START + 93)
    typedef struct _DOT11_AP_JOIN_REQUEST {
        ULONG uJoinFailureTimeout;
        DOT11_RATE_SET OperationalRateSet;
        ULONG uChCenterFrequency;
        DOT11_BSS_DESCRIPTION dot11BSSDescription;  // Must be the last field.
    } DOT11_AP_JOIN_REQUEST, * PDOT11_AP_JOIN_REQUEST;

//
// dot11PhyERPEntry
//
#define OID_DOT11_ERP_PBCC_OPTION_IMPLEMENTED       (OID_DOT11_NDIS_START + 94)
    // BOOL

#define OID_DOT11_ERP_PBCC_OPTION_ENABLED           (OID_DOT11_NDIS_START + 95)
    // BOOL

#define OID_DOT11_DSSS_OFDM_OPTION_IMPLEMENTED      (OID_DOT11_NDIS_START + 96)
    // BOOL

#define OID_DOT11_DSSS_OFDM_OPTION_ENABLED          (OID_DOT11_NDIS_START + 97)
    // BOOL

#define OID_DOT11_SHORT_SLOT_TIME_OPTION_IMPLEMENTED    (OID_DOT11_NDIS_START + 98)
    // BOOL

#define OID_DOT11_SHORT_SLOT_TIME_OPTION_ENABLED    (OID_DOT11_NDIS_START + 99)
    // BOOL

#define OID_DOT11_MAX_MAC_ADDRESS_STATES            (OID_DOT11_NDIS_START + 100)
    // ULONG

#define OID_DOT11_RECV_SENSITIVITY_LIST             (OID_DOT11_NDIS_START + 101)
    // DOT11_RECV_SENSITIVITY_LIST

    typedef struct _DOT11_RECV_SENSITIVITY {
        UCHAR ucDataRate;
        LONG lRSSIMin;
        LONG lRSSIMax;
    } DOT11_RECV_SENSITIVITY, * PDOT11_RECV_SENSITIVITY;

    typedef struct _DOT11_RECV_SENSITIVITY_LIST {
        union {
            DOT11_PHY_TYPE dot11PhyType;
            ULONG uPhyId;
        };
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        DOT11_RECV_SENSITIVITY dot11RecvSensitivity[1];
    } DOT11_RECV_SENSITIVITY_LIST, * PDOT11_RECV_SENSITIVITY_LIST;


//
// WME
//

#define OID_DOT11_WME_IMPLEMENTED                   (OID_DOT11_NDIS_START + 102)
    // BOOL

#define OID_DOT11_WME_ENABLED                       (OID_DOT11_NDIS_START + 103)
    // BOOL

#define OID_DOT11_WME_AC_PARAMETERS                 (OID_DOT11_NDIS_START + 104)
    typedef enum _DOT11_AC_PARAM {
        dot11_AC_param_BE = 0,      // Best Effort
        dot11_AC_param_BK = 1,      // Background
        dot11_AC_param_VI = 2,      // Video
        dot11_AC_param_VO = 3,      // Voice
        dot11_AC_param_max
    } DOT11_AC_PARAM, * PDOT11_AC_PARAM;
    typedef struct _DOT11_WME_AC_PARAMETERS {
	    UCHAR ucAccessCategoryIndex;
	    UCHAR ucAIFSN;
	    UCHAR ucECWmin;
	    UCHAR ucECWmax;
	    USHORT usTXOPLimit;
	} DOT11_WME_AC_PARAMETERS, * PDOT11_WME_AC_PARAMETERS;
	typedef struct _DOT11_WME_AC_PARAMTERS_LIST {
	    ULONG uNumOfEntries;
	    ULONG uTotalNumOfEntries;
	    DOT11_WME_AC_PARAMETERS dot11WMEACParameters[1];
    } DOT11_WME_AC_PARAMETERS_LIST, * PDOT11_WME_AC_PARAMETERS_LIST;

#define OID_DOT11_WME_UPDATE_IE                    (OID_DOT11_NDIS_START + 105)
    typedef struct _DOT11_WME_UPDATE_IE {
        ULONG uParamElemMinBeaconIntervals;
        ULONG uWMEInfoElemOffset;
        ULONG uWMEInfoElemLength;
        ULONG uWMEParamElemOffset;
        ULONG uWMEParamElemLength;
        UCHAR ucBuffer[1];          // Must be the last field.
    } DOT11_WME_UPDATE_IE, * PDOT11_WME_UPDATE_IE;

//
// QoS
//
#define OID_DOT11_QOS_TX_QUEUES_SUPPORTED          (OID_DOT11_NDIS_START + 106)
    // ULONG

#define OID_DOT11_QOS_TX_DURATION                  (OID_DOT11_NDIS_START + 107)
    typedef struct _DOT11_QOS_TX_DURATION {
        ULONG uNominalMSDUSize;
        ULONG uMinPHYRate;
        ULONG uDuration;
    } DOT11_QOS_TX_DURATION, * PDOT11_QOS_TX_DURATION;

#define OID_DOT11_QOS_TX_MEDIUM_TIME               (OID_DOT11_NDIS_START + 108)
    typedef struct _DOT11_QOS_TX_MEDIUM_TIME {
        DOT11_MAC_ADDRESS dot11PeerAddress;
        UCHAR ucQoSPriority;
        ULONG uMediumTimeAdmited;
    } DOT11_QOS_TX_MEDIUM_TIME, * PDOT11_QOS_TX_MEDIUM_TIME;

//
// NIC supported channel/center frequency list
//
#define OID_DOT11_SUPPORTED_OFDM_FREQUENCY_LIST    (OID_DOT11_NDIS_START + 109)
    typedef struct _DOT11_SUPPORTED_OFDM_FREQUENCY {
        ULONG uCenterFrequency;
    } DOT11_SUPPORTED_OFDM_FREQUENCY, * PDOT11_SUPPORTED_OFDM_FREQUENCY;
    typedef struct _DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        DOT11_SUPPORTED_OFDM_FREQUENCY dot11SupportedOFDMFrequency[1];
    } DOT11_SUPPORTED_OFDM_FREQUENCY_LIST, * PDOT11_SUPPORTED_OFDM_FREQUENCY_LIST;

#define OID_DOT11_SUPPORTED_DSSS_CHANNEL_LIST      (OID_DOT11_NDIS_START + 110)
    typedef struct _DOT11_SUPPORTED_DSSS_CHANNEL {
        ULONG uChannel;
    } DOT11_SUPPORTED_DSSS_CHANNEL, * PDOT11_SUPPORTED_DSSS_CHANNEL;
    typedef struct _DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        DOT11_SUPPORTED_DSSS_CHANNEL dot11SupportedDSSSChannel[1];
    } DOT11_SUPPORTED_DSSS_CHANNEL_LIST, * PDOT11_SUPPORTED_DSSS_CHANNEL_LIST;


//
// Extensible STA
//

typedef struct DOT11_BYTE_ARRAY {
    NDIS_OBJECT_HEADER Header;
    ULONG uNumOfBytes;
    ULONG uTotalNumOfBytes;
    UCHAR ucBuffer[1];
} DOT11_BYTE_ARRAY, * PDOT11_BYTE_ARRAY;

#define OID_DOT11_AUTO_CONFIG_ENABLED               NWF_DEFINE_OID(0x78, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // ULONG
    #define DOT11_PHY_AUTO_CONFIG_ENABLED_FLAG          0x00000001U
    #define DOT11_MAC_AUTO_CONFIG_ENABLED_FLAG          0x00000002U

#define OID_DOT11_ENUM_BSS_LIST                     NWF_DEFINE_OID(0x79, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // DOT11_BYTE_ARRAY with DOT11_BSS_ENTRY
    #define DOT11_BSS_ENTRY_BYTE_ARRAY_REVISION_1   1

    // This structure is not supposed to be midl compliant because of
    // DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO. The selection of union is
    // *indirectly* determined from uPhyId. MIDL will not work here.
#ifndef __midl
    typedef union DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
        ULONG uChCenterFrequency;
        struct {
            ULONG uHopPattern;
            ULONG uHopSet;
            ULONG uDwellTime;
        } FHSS;
    } DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO, * PDOT11_BSS_ENTRY_PHY_SPECIFIC_INFO;

    typedef struct DOT11_BSS_ENTRY {
        ULONG uPhyId;
        DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO PhySpecificInfo;
        DOT11_MAC_ADDRESS dot11BSSID;
        DOT11_BSS_TYPE dot11BSSType;
        LONG lRSSI;
        ULONG uLinkQuality;
        BOOLEAN bInRegDomain;
        USHORT usBeaconPeriod;
        ULONGLONG ullTimestamp;
        ULONGLONG ullHostTimestamp;
        USHORT usCapabilityInformation;
        ULONG uBufferLength;
        UCHAR ucBuffer[1];			// Must be the last field.
    } DOT11_BSS_ENTRY, * PDOT11_BSS_ENTRY;
#endif

#define OID_DOT11_FLUSH_BSS_LIST                    NWF_DEFINE_OID(0x7A, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // VOID

#define OID_DOT11_POWER_MGMT_REQUEST                NWF_DEFINE_OID(0x7B, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // ULONG
    #define DOT11_POWER_SAVING_NO_POWER_SAVING  0
    #define DOT11_POWER_SAVING_FAST_PSP         8
    #define DOT11_POWER_SAVING_MAX_PSP          16
    #define DOT11_POWER_SAVING_MAXIMUM_LEVEL    24

#define OID_DOT11_DESIRED_SSID_LIST                 NWF_DEFINE_OID(0x7C, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // A list of DOT11_SSID
    typedef struct DOT11_SSID_LIST {
        #define DOT11_SSID_LIST_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
#ifdef __midl
        [unique, size_is(uTotalNumOfEntries)] DOT11_SSID SSIDs[*];
#else
        DOT11_SSID SSIDs[1];
#endif
    } DOT11_SSID_LIST, * PDOT11_SSID_LIST;

#define OID_DOT11_EXCLUDED_MAC_ADDRESS_LIST         NWF_DEFINE_OID(0x7D, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // A list of DOT11_MAC_ADDRESS
    typedef struct DOT11_MAC_ADDRESS_LIST {
        #define DOT11_MAC_ADDRESS_LIST_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        DOT11_MAC_ADDRESS MacAddrs[1];
    } DOT11_MAC_ADDRESS_LIST, * PDOT11_MAC_ADDRESS_LIST;

#define OID_DOT11_DESIRED_BSSID_LIST                NWF_DEFINE_OID(0x7E, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)

#define OID_DOT11_DESIRED_BSS_TYPE                  NWF_DEFINE_OID(0x7F, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // DOT11_BSS_TYPE

#define OID_DOT11_PMKID_LIST                        NWF_DEFINE_OID(0x80, NWF_OPERATIONAL_OID, NWF_OPTIONAL_OID)
    // A list of DOT11_PMKID_ENTRY
    typedef UCHAR DOT11_PMKID_VALUE[16];
    typedef struct DOT11_PMKID_ENTRY {
        DOT11_MAC_ADDRESS BSSID;
        DOT11_PMKID_VALUE PMKID;
        ULONG uFlags;
    } DOT11_PMKID_ENTRY, *PDOT11_PMKID_ENTRY;
    typedef struct DOT11_PMKID_LIST {
        #define DOT11_PMKID_LIST_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        DOT11_PMKID_ENTRY PMKIDs[1];
    } DOT11_PMKID_LIST, * PDOT11_PMKID_LIST;

#define OID_DOT11_CONNECT_REQUEST                   NWF_DEFINE_OID(0x81, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // no data type

#define OID_DOT11_EXCLUDE_UNENCRYPTED               NWF_DEFINE_OID(0x82, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // BOOLEAN

#define OID_DOT11_STATISTICS                        NWF_DEFINE_OID(0x83, NWF_STATISTICS_OID, NWF_MANDATORY_OID)
    // DOT11_STATISTICS structure
    #define DOT11_STATISTICS_UNKNOWN    (ULONGLONG)(-1LL)
    typedef struct DOT11_PHY_FRAME_STATISTICS {
        // TX counters (MSDU/MMPDU)
        ULONGLONG ullTransmittedFrameCount;
        ULONGLONG ullMulticastTransmittedFrameCount;
        ULONGLONG ullFailedCount;
        ULONGLONG ullRetryCount;
        ULONGLONG ullMultipleRetryCount;
        ULONGLONG ullMaxTXLifetimeExceededCount;

        // TX counters (MPDU)
        ULONGLONG ullTransmittedFragmentCount;
        ULONGLONG ullRTSSuccessCount;
        ULONGLONG ullRTSFailureCount;
        ULONGLONG ullACKFailureCount;

        // RX counters (MSDU/MMPDU)
        ULONGLONG ullReceivedFrameCount;
        ULONGLONG ullMulticastReceivedFrameCount;
        ULONGLONG ullPromiscuousReceivedFrameCount;
        ULONGLONG ullMaxRXLifetimeExceededCount;

        // RX counters (MPDU)
        ULONGLONG ullFrameDuplicateCount;
        ULONGLONG ullReceivedFragmentCount;
        ULONGLONG ullPromiscuousReceivedFragmentCount;
        ULONGLONG ullFCSErrorCount;
    } DOT11_PHY_FRAME_STATISTICS, * PDOT11_PHY_FRAME_STATISTICS;
    typedef struct DOT11_MAC_FRAME_STATISTICS {
        ULONGLONG ullTransmittedFrameCount;
        ULONGLONG ullReceivedFrameCount;
        ULONGLONG ullTransmittedFailureFrameCount;
        ULONGLONG ullReceivedFailureFrameCount;

        ULONGLONG ullWEPExcludedCount;
        ULONGLONG ullTKIPLocalMICFailures;
        ULONGLONG ullTKIPReplays;
        ULONGLONG ullTKIPICVErrorCount;
        ULONGLONG ullCCMPReplays;
        ULONGLONG ullCCMPDecryptErrors;
        ULONGLONG ullWEPUndecryptableCount;
        ULONGLONG ullWEPICVErrorCount;
        ULONGLONG ullDecryptSuccessCount;
        ULONGLONG ullDecryptFailureCount;
    } DOT11_MAC_FRAME_STATISTICS, * PDOT11_MAC_FRAME_STATISTICS;
    typedef struct DOT11_STATISTICS {
        #define DOT11_STATISTICS_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        ULONGLONG ullFourWayHandshakeFailures;
        ULONGLONG ullTKIPCounterMeasuresInvoked;
        ULONGLONG ullReserved;

        DOT11_MAC_FRAME_STATISTICS MacUcastCounters;
        DOT11_MAC_FRAME_STATISTICS MacMcastCounters;
        DOT11_PHY_FRAME_STATISTICS PhyCounters[1];
    } DOT11_STATISTICS, * PDOT11_STATISTICS;

#define OID_DOT11_PRIVACY_EXEMPTION_LIST            NWF_DEFINE_OID(0x84, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // A list of DOT11_PRIVACY_EXEMPTION
    typedef struct DOT11_PRIVACY_EXEMPTION {
        USHORT usEtherType;

        #define DOT11_EXEMPT_NO_EXEMPTION       0   // used only in DOT11_EXTSTA_SEND_CONTEXT
        #define DOT11_EXEMPT_ALWAYS             1
        #define DOT11_EXEMPT_ON_KEY_MAPPING_KEY_UNAVAILABLE 2

        USHORT usExemptionActionType;

        #define DOT11_EXEMPT_UNICAST		1
        #define DOT11_EXEMPT_MULTICAST	        2
        #define DOT11_EXEMPT_BOTH		3
        USHORT usExemptionPacketType;
    } DOT11_PRIVACY_EXEMPTION, *PDOT11_PRIVACY_EXEMPTION;
    typedef struct DOT11_PRIVACY_EXEMPTION_LIST {
        #define DOT11_PRIVACY_EXEMPTION_LIST_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
#ifdef __midl
        [unique, size_is(uTotalNumOfEntries)] DOT11_PRIVACY_EXEMPTION PrivacyExemptionEntries[*];
#else
        DOT11_PRIVACY_EXEMPTION PrivacyExemptionEntries[1];
#endif
    } DOT11_PRIVACY_EXEMPTION_LIST, * PDOT11_PRIVACY_EXEMPTION_LIST;

#define OID_DOT11_ENABLED_AUTHENTICATION_ALGORITHM  NWF_DEFINE_OID(0x85, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    typedef struct DOT11_AUTH_ALGORITHM_LIST {
        #define DOT11_AUTH_ALGORITHM_LIST_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        DOT11_AUTH_ALGORITHM AlgorithmIds[1];
    } DOT11_AUTH_ALGORITHM_LIST, * PDOT11_AUTH_ALGORITHM_LIST;

#define OID_DOT11_SUPPORTED_UNICAST_ALGORITHM_PAIR  NWF_DEFINE_OID(0x86, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    typedef struct DOT11_AUTH_CIPHER_PAIR_LIST {
        #define DOT11_AUTH_CIPHER_PAIR_LIST_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        _Field_size_(uNumOfEntries) DOT11_AUTH_CIPHER_PAIR AuthCipherPairs[1];
    } DOT11_AUTH_CIPHER_PAIR_LIST, * PDOT11_AUTH_CIPHER_PAIR_LIST;

#define OID_DOT11_ENABLED_UNICAST_CIPHER_ALGORITHM  NWF_DEFINE_OID(0x87, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // DOT11_CIPHER_ALGO_LIST
    typedef struct DOT11_CIPHER_ALGORITHM_LIST {
        #define DOT11_CIPHER_ALGORITHM_LIST_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        DOT11_CIPHER_ALGORITHM AlgorithmIds[1];
    } DOT11_CIPHER_ALGORITHM_LIST, * PDOT11_CIPHER_ALGORITHM_LIST;

#define OID_DOT11_SUPPORTED_MULTICAST_ALGORITHM_PAIR    NWF_DEFINE_OID(0x88, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)

#define OID_DOT11_ENABLED_MULTICAST_CIPHER_ALGORITHM    NWF_DEFINE_OID(0x89, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // DOT11_CIPHER_ALGORITHM_LIST

#define OID_DOT11_CIPHER_DEFAULT_KEY_ID             NWF_DEFINE_OID(0x8A, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // ULONG

#define OID_DOT11_CIPHER_DEFAULT_KEY                NWF_DEFINE_OID(0x8B, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    typedef struct DOT11_CIPHER_DEFAULT_KEY_VALUE {
        #define DOT11_CIPHER_DEFAULT_KEY_VALUE_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        ULONG uKeyIndex;
        DOT11_CIPHER_ALGORITHM AlgorithmId;
        DOT11_MAC_ADDRESS MacAddr;
        BOOLEAN bDelete;
        BOOLEAN bStatic;
        USHORT usKeyLength;
#ifdef __midl
        [unique, size_is(usKeyLength)] UCHAR ucKey[*];
#else
        UCHAR ucKey[1];                 // Must be the last field
#endif
    } DOT11_CIPHER_DEFAULT_KEY_VALUE, * PDOT11_CIPHER_DEFAULT_KEY_VALUE;
    typedef struct DOT11_KEY_ALGO_TKIP_MIC {
        UCHAR ucIV48Counter[6];
        ULONG ulTKIPKeyLength;
        ULONG ulMICKeyLength;
        UCHAR ucTKIPMICKeys[1];                     // Must be the last field.
    } DOT11_KEY_ALGO_TKIP_MIC, * PDOT11_KEY_ALGO_TKIP_MIC;
    typedef struct DOT11_KEY_ALGO_CCMP {
        UCHAR ucIV48Counter[6];
        ULONG ulCCMPKeyLength;
        UCHAR ucCCMPKey[1];
    } DOT11_KEY_ALGO_CCMP, * PDOT11_KEY_ALGO_CCMP;
    typedef struct DOT11_KEY_ALGO_GCMP {
        UCHAR ucIV48Counter[6];
        ULONG ulGCMPKeyLength;
        UCHAR ucGCMPKey[1];
    } DOT11_KEY_ALGO_GCMP, * PDOT11_KEY_ALGO_GCMP;
    typedef struct DOT11_KEY_ALGO_GCMP_256 {
        UCHAR ucIV48Counter[6];
        ULONG ulGCMP256KeyLength;
        UCHAR ucGCMP256Key[1];
    } DOT11_KEY_ALGO_GCMP_256, * PDOT11_KEY_ALGO_GCMP_256;
    typedef struct DOT11_KEY_ALGO_BIP {
        UCHAR ucIPN[6];
        ULONG ulBIPKeyLength;
        UCHAR ucBIPKey[1];
    } DOT11_KEY_ALGO_BIP, * PDOT11_KEY_ALGO_BIP;
    typedef struct DOT11_KEY_ALGO_BIP_GMAC_256 {
        UCHAR ucIPN[6];
        ULONG ulBIPGmac256KeyLength;
        UCHAR ucBIPGmac256Key[1];
    } DOT11_KEY_ALGO_BIP_GMAC_256, * PDOT11_KEY_ALGO_BIP_GMAC_256;


#define OID_DOT11_CIPHER_KEY_MAPPING_KEY            NWF_DEFINE_OID(0x8C, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // DOT11_BYTE_ARRAY
    typedef enum DOT11_DIRECTION {
        DOT11_DIR_INBOUND = 1,
        DOT11_DIR_OUTBOUND,
        DOT11_DIR_BOTH
    } DOT11_DIRECTION, * PDOT11_DIRECTION;
    #define DOT11_CIPHER_KEY_MAPPING_KEY_VALUE_BYTE_ARRAY_REVISION_1  1
    typedef struct DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
        DOT11_MAC_ADDRESS PeerMacAddr;
        DOT11_CIPHER_ALGORITHM AlgorithmId;
        DOT11_DIRECTION Direction;
        BOOLEAN bDelete;
        BOOLEAN bStatic;
        USHORT usKeyLength;
#ifdef __midl
        [unique, size_is(usKeyLength)] UCHAR ucKey[*];
#else
        UCHAR ucKey[1];			// Must be the last field
#endif
    } DOT11_CIPHER_KEY_MAPPING_KEY_VALUE, * PDOT11_CIPHER_KEY_MAPPING_KEY_VALUE;
#define OID_DOT11_ENUM_ASSOCIATION_INFO             NWF_DEFINE_OID(0x8D, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // a list of DOT11_ASSOCIATION_INFO
    typedef enum _DOT11_ASSOCIATION_STATE {
        dot11_assoc_state_zero = 0,
        dot11_assoc_state_unauth_unassoc = 1,
        dot11_assoc_state_auth_unassoc = 2,
        dot11_assoc_state_auth_assoc = 3
    } DOT11_ASSOCIATION_STATE, * PDOT11_ASSOCIATION_STATE;
    typedef struct _DOT11_ASSOCIATION_INFO_EX {
        DOT11_MAC_ADDRESS PeerMacAddress;
        DOT11_MAC_ADDRESS BSSID;
        USHORT usCapabilityInformation;
        USHORT usListenInterval;
        UCHAR ucPeerSupportedRates[MAX_NUM_SUPPORTED_RATES_V2];
        USHORT usAssociationID;
        DOT11_ASSOCIATION_STATE dot11AssociationState;
        DOT11_POWER_MODE dot11PowerMode;
        LARGE_INTEGER liAssociationUpTime;
        ULONGLONG ullNumOfTxPacketSuccesses;
        ULONGLONG ullNumOfTxPacketFailures;
        ULONGLONG ullNumOfRxPacketSuccesses;
        ULONGLONG ullNumOfRxPacketFailures;
    } DOT11_ASSOCIATION_INFO_EX, * PDOT11_ASSOCIATION_INFO_EX;
    typedef struct DOT11_ASSOCIATION_INFO_LIST {
        #define DOT11_ASSOCIATION_INFO_LIST_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        DOT11_ASSOCIATION_INFO_EX dot11AssocInfo[1];
    } DOT11_ASSOCIATION_INFO_LIST, * PDOT11_ASSOCIATION_INFO_LIST;


#define OID_DOT11_DISCONNECT_REQUEST                NWF_DEFINE_OID(0x8E, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)

#define OID_DOT11_UNICAST_USE_GROUP_ENABLED         NWF_DEFINE_OID(0x8F, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // BOOLEAN

#define OID_DOT11_HARDWARE_PHY_STATE                NWF_DEFINE_OID(0x90, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // BOOLEAN

#define OID_DOT11_DESIRED_PHY_LIST                  NWF_DEFINE_OID(0x91, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // DOT11_PHY_ID_LIST
    typedef struct DOT11_PHY_ID_LIST {
        #define DOT11_PHY_ID_LIST_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        ULONG dot11PhyId[1];
    } DOT11_PHY_ID_LIST, * PDOT11_PHY_ID_LIST;
    #define DOT11_PHY_ID_ANY        (0xffffffffU)

#define OID_DOT11_CURRENT_PHY_ID                    NWF_DEFINE_OID(0x92, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // ULONG

#define OID_DOT11_MEDIA_STREAMING_ENABLED           NWF_DEFINE_OID(0x93, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // BOOLEAN

#define OID_DOT11_UNREACHABLE_DETECTION_THRESHOLD   NWF_DEFINE_OID(0x94, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // ULONG

#define OID_DOT11_ACTIVE_PHY_LIST                   NWF_DEFINE_OID(0x95, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // DOT11_PHY_ID_LIST

#define OID_DOT11_EXTSTA_CAPABILITY                 NWF_DEFINE_OID(0x96, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // DOT11_EXTSTA_CAPABILITY
    typedef struct DOT11_EXTSTA_CAPABILITY {
        #define DOT11_EXTSTA_CAPABILITY_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        ULONG uScanSSIDListSize;
        ULONG uDesiredBSSIDListSize;
        ULONG uDesiredSSIDListSize;
        ULONG uExcludedMacAddressListSize;
        ULONG uPrivacyExemptionListSize;
        ULONG uKeyMappingTableSize;
        ULONG uDefaultKeyTableSize;
        ULONG uWEPKeyValueMaxLength;
        ULONG uPMKIDCacheSize;
        ULONG uMaxNumPerSTADefaultKeyTables;
    } DOT11_EXTSTA_CAPABILITY, * PDOT11_EXTSTA_CAPABILITY;

#define OID_DOT11_DATA_RATE_MAPPING_TABLE           NWF_DEFINE_OID(0x97, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // DOT11_DATA_RATE_MAPPING_ENTRY
    typedef struct DOT11_DATA_RATE_MAPPING_ENTRY {
        UCHAR ucDataRateIndex;
        UCHAR ucDataRateFlag;
        USHORT usDataRateValue;
    } DOT11_DATA_RATE_MAPPING_ENTRY, * PDOT11_DATA_RATE_MAPPING_ENTRY;
    typedef struct _DOT11_DATA_RATE_MAPPING_TABLE {
        #define DOT11_DATA_RATE_MAPPING_TABLE_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        ULONG uDataRateMappingLength;
        _Field_size_part_(DOT11_RATE_SET_MAX_LENGTH, uDataRateMappingLength)
            DOT11_DATA_RATE_MAPPING_ENTRY DataRateMappingEntries[DOT11_RATE_SET_MAX_LENGTH];
    } DOT11_DATA_RATE_MAPPING_TABLE, * PDOT11_DATA_RATE_MAPPING_TABLE;
    #define DOT11_DATA_RATE_NON_STANDARD        0x01U
    #define DOT11_DATA_RATE_INDEX_MASK          0x7fU

#define OID_DOT11_SUPPORTED_COUNTRY_OR_REGION_STRING    NWF_DEFINE_OID(0x98, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    typedef struct DOT11_COUNTRY_OR_REGION_STRING_LIST {
        #define DOT11_COUNTRY_OR_REGION_STRING_LIST_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        DOT11_COUNTRY_OR_REGION_STRING CountryOrRegionStrings[1];
    } DOT11_COUNTRY_OR_REGION_STRING_LIST, * PDOT11_COUNTRY_OR_REGION_STRING_LIST;

#define OID_DOT11_DESIRED_COUNTRY_OR_REGION_STRING      NWF_DEFINE_OID(0x99, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // DOT11_COUNTRY_OR_REGION_STRING

#define OID_DOT11_PORT_STATE_NOTIFICATION           NWF_DEFINE_OID(0x9A, NWF_OPERATIONAL_OID, NWF_OPTIONAL_OID)
    // DOT11_PORT_STATE_NOTIFICATION
    typedef struct DOT11_PORT_STATE_NOTIFICATION {
        #define DOT11_PORT_STATE_NOTIFICATION_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS PeerMac;
        BOOLEAN bOpen;
    } DOT11_PORT_STATE_NOTIFICATION, * PDOT11_PORT_STATE_NOTIFICATION;

#define OID_DOT11_IBSS_PARAMS                       NWF_DEFINE_OID(0x9B, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // DOT11_IBSS_PARAMS
    typedef struct DOT11_IBSS_PARAMS {
        #define DOT11_IBSS_PARAMS_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        BOOLEAN bJoinOnly;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_IBSS_PARAMS, * PDOT11_IBSS_PARAMS;

#define OID_DOT11_QOS_PARAMS                        NWF_DEFINE_OID(0x9C, NWF_OPERATIONAL_OID, NWF_OPTIONAL_OID)
    typedef struct DOT11_QOS_PARAMS {
        #define DOT11_QOS_PARAMS_REVISION_1     1
        NDIS_OBJECT_HEADER Header;

        #define DOT11_QOS_PROTOCOL_FLAG_WMM     (0x01U)         // WMM QoS protocol
        #define DOT11_QOS_PROTOCOL_FLAG_11E     (0x02U)         // 802.11e QoS protocol

        // Flags of the enabled QoS protocols.
        // It is either 0 or combination of DOT11_QOS_PROTOCOL_FLAG_WMM
        // and/or DOT11_QOS_PROTOCOL_FLAG_11E
        UCHAR ucEnabledQoSProtocolFlags;
    } DOT11_QOS_PARAMS, * PDOT11_QOS_PARAMS;

#define OID_DOT11_SAFE_MODE_ENABLED                 NWF_DEFINE_OID(0x9D, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // BOOLEAN

#define OID_DOT11_HIDDEN_NETWORK_ENABLED            NWF_DEFINE_OID(0x9E, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // BOOLEAN

#define OID_DOT11_ASSOCIATION_PARAMS                NWF_DEFINE_OID(0x9F, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // DOT11_ASSOCIATION_PARAMS
    typedef struct DOT11_ASSOCIATION_PARAMS {
        #define DOT11_ASSOCIATION_PARAMS_REVISION_1  1
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS  BSSID;
        ULONG uAssocRequestIEsOffset;
        ULONG uAssocRequestIEsLength;
    } DOT11_ASSOCIATION_PARAMS, *PDOT11_ASSOCIATION_PARAMS;

#define OID_DOT11_SAFE_MODE_HT_ENABLED             NWF_DEFINE_OID(0xA0, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)
    // BOOLEAN

//
// 802.11 Extensions to Standard NDIS Functions
//

//
// Miniport Send Path Extension
//

// Only 4 bits are present in the 802.11 header to track fragments.
#define DOT11_MAX_NUM_OF_FRAGMENTS                  16
// Priority Classes.
#define DOT11_PRIORITY_CONTENTION                   0
#define DOT11_PRIORITY_CONTENTION_FREE              1

// Service Classes.
#define DOT11_SERVICE_CLASS_REORDERABLE_MULTICAST   0
#define DOT11_SERVICE_CLASS_STRICTLY_ORDERED        1
// Flags.
#define DOT11_FLAGS_80211B_SHORT_PREAMBLE           0x00000001
#define DOT11_FLAGS_80211B_PBCC                     0x00000002
#define DOT11_FLAGS_80211B_CHANNEL_AGILITY          0x00000004
#define DOT11_FLAGS_PS_ON                           0x00000008
#define DOT11_FLAGS_80211G_DSSS_OFDM                0x00000010
#define DOT11_FLAGS_80211G_USE_PROTECTION           0x00000020
#define DOT11_FLAGS_80211G_NON_ERP_PRESENT          0x00000040
#define DOT11_FLAGS_80211G_BARKER_PREAMBLE_MODE     0x00000080
#define DOT11_WME_PACKET                            0x00000100

typedef struct _DOT11_FRAGMENT_DESCRIPTOR {
    ULONG uOffset;
    ULONG uLength;
} DOT11_FRAGMENT_DESCRIPTOR, * PDOT11_FRAGMENT_DESCRIPTOR;

typedef struct _DOT11_PER_MSDU_COUNTERS {
    ULONG uTransmittedFragmentCount;
    ULONG uRetryCount;
    ULONG uRTSSuccessCount;
    ULONG uRTSFailureCount;
    ULONG uACKFailureCount;
} DOT11_PER_MSDU_COUNTERS, * PDOT11_PER_MSDU_COUNTERS;

    typedef struct DOT11_PHY_ATTRIBUTES DOT11_PHY_ATTRIBUTES, * PDOT11_PHY_ATTRIBUTES;

    typedef struct DOT11_HRDSSS_PHY_ATTRIBUTES {
        BOOLEAN bShortPreambleOptionImplemented;
        BOOLEAN bPBCCOptionImplemented;
        BOOLEAN bChannelAgilityPresent;
        ULONG uHRCCAModeSupported;
    } DOT11_HRDSSS_PHY_ATTRIBUTES, * PDOT11_HRDSSS_PHY_ATTRIBUTES;

    typedef struct DOT11_OFDM_PHY_ATTRIBUTES {
        ULONG uFrequencyBandsSupported;
    } DOT11_OFDM_PHY_ATTRIBUTES, * PDOT11_OFDM_PHY_ATTRIBUTES;

    typedef struct DOT11_ERP_PHY_ATTRIBUTES {
        #ifdef __cplusplus
            DOT11_HRDSSS_PHY_ATTRIBUTES HRDSSSAttributes;
        #else
            DOT11_HRDSSS_PHY_ATTRIBUTES;
        #endif
        BOOLEAN bERPPBCCOptionImplemented;
        BOOLEAN bDSSSOFDMOptionImplemented;
        BOOLEAN bShortSlotTimeOptionImplemented;
    } DOT11_ERP_PHY_ATTRIBUTES, * PDOT11_ERP_PHY_ATTRIBUTES;

    struct DOT11_PHY_ATTRIBUTES {
        #define DOT11_PHY_ATTRIBUTES_REVISION_1  1
        NDIS_OBJECT_HEADER Header;

        DOT11_PHY_TYPE PhyType;
        BOOLEAN bHardwarePhyState;
        BOOLEAN bSoftwarePhyState;

        BOOLEAN bCFPollable;
        ULONG uMPDUMaxLength;
        DOT11_TEMP_TYPE TempType;
        DOT11_DIVERSITY_SUPPORT DiversitySupport;
        #ifdef __midl
        [switch_is(PhyType)]
        #endif
        union {
            #ifdef __midl
            [case(dot11_phy_type_hrdsss)]
            #endif
            DOT11_HRDSSS_PHY_ATTRIBUTES HRDSSSAttributes;

            #ifdef __midl
            [case(dot11_phy_type_ofdm)]
            #endif
            DOT11_OFDM_PHY_ATTRIBUTES OFDMAttributes;

            #ifdef __midl
            [case(dot11_phy_type_erp)]
            #endif
            DOT11_ERP_PHY_ATTRIBUTES ERPAttributes;

            #ifdef __midl
            [case(dot11_phy_type_ht)]
            ;
            #endif

            #ifdef __midl
            [default]
            ;
            #endif
        #ifdef __cplusplus
        } PhySpecificAttributes;
        #else
        };
        #endif
        ULONG uNumberSupportedPowerLevels;
        ULONG TxPowerLevels[8];

        ULONG uNumDataRateMappingEntries;
        DOT11_DATA_RATE_MAPPING_ENTRY DataRateMappingEntries[DOT11_RATE_SET_MAX_LENGTH];

        DOT11_SUPPORTED_DATA_RATES_VALUE_V2 SupportedDataRatesValue;
    };

#define DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_OID_SUPPORTED  0x1
#define DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_CERTIFIED      0x2
#define DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_RESERVED       0xC

typedef struct DOT11_EXTSTA_ATTRIBUTES DOT11_EXTSTA_ATTRIBUTES, * PDOT11_EXTSTA_ATTRIBUTES;
struct DOT11_EXTSTA_ATTRIBUTES {
    #define DOT11_EXTSTA_ATTRIBUTES_REVISION_1  1
    #define DOT11_EXTSTA_ATTRIBUTES_REVISION_2  2
    #define DOT11_EXTSTA_ATTRIBUTES_REVISION_3  3
    #define DOT11_EXTSTA_ATTRIBUTES_REVISION_4  4

    NDIS_OBJECT_HEADER Header;

    ULONG uScanSSIDListSize;
    ULONG uDesiredBSSIDListSize;
    ULONG uDesiredSSIDListSize;
    ULONG uExcludedMacAddressListSize;
    ULONG uPrivacyExemptionListSize;
    ULONG uKeyMappingTableSize;
    ULONG uDefaultKeyTableSize;
    ULONG uWEPKeyValueMaxLength;
    ULONG uPMKIDCacheSize;
    ULONG uMaxNumPerSTADefaultKeyTables;

    BOOLEAN bStrictlyOrderedServiceClassImplemented;

    // Flags of the supported QoS protocols.
    // It is either 0 or combination of DOT11_QOS_PROTOCOL_FLAG_WMM
    // and/or DOT11_QOS_PROTOCOL_FLAG_11E
    UCHAR ucSupportedQoSProtocolFlags;

    BOOLEAN bSafeModeImplemented;

    // Supported Country Strings
    ULONG uNumSupportedCountryOrRegionStrings;
    #ifdef __midl
    [size_is(uNumSupportedCountryOrRegionStrings)]
    #endif
    PDOT11_COUNTRY_OR_REGION_STRING pSupportedCountryOrRegionStrings;

    // Infra Capabilities
    ULONG uInfraNumSupportedUcastAlgoPairs;
    #ifdef __midl
    [size_is(uInfraNumSupportedUcastAlgoPairs)]
    #endif
    PDOT11_AUTH_CIPHER_PAIR pInfraSupportedUcastAlgoPairs;

    ULONG uInfraNumSupportedMcastAlgoPairs;
    #ifdef __midl
    [size_is(uInfraNumSupportedMcastAlgoPairs)]
    #endif
    PDOT11_AUTH_CIPHER_PAIR pInfraSupportedMcastAlgoPairs;

    // Ad hoc Capabilities
    ULONG uAdhocNumSupportedUcastAlgoPairs;
    #ifdef __midl
    [size_is(uAdhocNumSupportedUcastAlgoPairs)]
    #endif
    PDOT11_AUTH_CIPHER_PAIR pAdhocSupportedUcastAlgoPairs;

    ULONG uAdhocNumSupportedMcastAlgoPairs;
    #ifdef __midl
    [size_is(uAdhocNumSupportedMcastAlgoPairs)]
    #endif
    PDOT11_AUTH_CIPHER_PAIR pAdhocSupportedMcastAlgoPairs;
    BOOLEAN bAutoPowerSaveMode;			// revision 3 and above
    ULONG uMaxNetworkOffloadListSize;	// revision 3 and above

    //802.11w Capabilities
    BOOLEAN bMFPCapable;
    ULONG uInfraNumSupportedMcastMgmtAlgoPairs;
    #ifdef __midl
    [size_is(uInfraNumSupportedMcastMgmtAlgoPairs)]
    #endif
    PDOT11_AUTH_CIPHER_PAIR pInfraSupportedMcastMgmtAlgoPairs;

    //802.11k Capabilities
    BOOLEAN bNeighborReportSupported;
    BOOLEAN bAPChannelReportSupported;

    //802.11u Capabilities
    BOOLEAN bActionFramesSupported;
    BOOLEAN bANQPQueryOffloadSupported;
    BOOLEAN bHESSIDConnectionSupported;
};

#if (defined(_NDIS_) || defined(NDIS_WRAPPER)) // To avoid build break since NDIS_PACKET isn't defined for user mode app.
        // Send path extension for NativeWiFi NDIS 6.0 miniport driver
        typedef struct _DOT11_SEND_CONTEXT {
            #define DOT11_SEND_CONTEXT_REVISION_1  1
            NDIS_OBJECT_HEADER Header;
            PVOID pvReserved;
            ULONG uFlags;
            ULONG uPSLifetime;
            ULONG uDelayedSleepValue;
            UCHAR ucTXDataRates[8];
            BOOLEAN bIndicateAssociatedACKs;
            BOOLEAN bIndicateTXStatus;
            UCHAR ucPriority;
            BOOLEAN bDontFragment;
            ULONG   dwExtendedStatus;
            HANDLE hIntegrityOffload;
            HANDLE hWEPOffload;
            UCHAR ucWPAMSDUPriority;
            UCHAR ucNumOfRWsOnPeer;
            USHORT usAID;
            PDOT11_PER_MSDU_COUNTERS pDot11PerMSDUCounters;
        } DOT11_SEND_CONTEXT, * PDOT11_SEND_CONTEXT;


        #define DOT11_SEND_CONTEXT_SIZE             \
                ((sizeof(DOT11_SEND_CONTEXT) +      \
                  MEMORY_ALLOCATION_ALIGNMENT - 1) &  \
                 ~(MEMORY_ALLOCATION_ALIGNMENT-1))



        // Send path extension for NativeWiFi NDIS 5.1 miniport driver
        typedef struct _DOT11_SEND_EXTENSION_INFO {
            ULONG uVersion;
            NDIS_OBJECT_HEADER Header;
            PVOID pvReserved;
            ULONG uFlags;
            ULONG uPSLifetime;
            ULONG uDelayedSleepValue;
            UCHAR ucTXDataRates[8];
            BOOLEAN bIndicateAssociatedACKs;
            BOOLEAN bIndicateTXStatus;
            UCHAR ucPriority;
            BOOLEAN bDontFragment;
            ULONG   dwExtendedStatus;
            HANDLE hIntegrityOffload;
            HANDLE hWEPOffload;
            UCHAR ucWPAMSDUPriority;
            UCHAR ucNumOfRWsOnPeer;
            USHORT usAID;
            PDOT11_PER_MSDU_COUNTERS pDot11PerMSDUCounters;
            USHORT usNumberOfFragments;
            DOT11_FRAGMENT_DESCRIPTOR Dot11FragmentDescriptors[1];
        } DOT11_SEND_EXTENSION_INFO, * PDOT11_SEND_EXTENSION_INFO;

        typedef enum _DOT11_ACK_POLICY {
            dot11_ack_policy_none = 0,
            dot11_ack_policy_acknowledge = 1,
            dot11_ack_policy_do_not_acknowledge = 2,
        } DOT11_ACK_POLICY, * PDOT11_ACK_POLICY;

        // Send path extension for NativeWiFi NDIS 5.1 miniport driver
        typedef struct _DOT11_SEND_EXTENSION_INFO_V2 {
            UCHAR ucQoSPriority:4;
            UCHAR ucAckPolicy:4;
            USHORT usExtendedTXDataRatesOffset;
            UCHAR ucNumOfExtendedTXDataRates;
            UCHAR ucBuffer[1];
        } DOT11_SEND_EXTENSION_INFO_V2, * PDOT11_SEND_EXTENSION_INFO_V2;

        //
        // Miniport Receive Path Extension
        //

        // Recv path extension for NativeWiFi NDIS 6.0 miniport driver
        typedef struct _DOT11_RECV_CONTEXT {
            #define DOT11_RECV_CONTEXT_REVISION_1  1
            NDIS_OBJECT_HEADER Header;
            PVOID pvReserved;
            DOT11_PHY_TYPE dot11PhyType;
            ULONG uChCenterFrequency;
            LONG lRSSI;
            ULONG uRSSI;
            UCHAR ucPriority;
            UCHAR ucDataRate;
            UCHAR ucPeerMacAddress[6];
            ULONG dwExtendedStatus;
            HANDLE hWEPOffloadContext;
            HANDLE hAuthOffloadContext;
            USHORT usWEPAppliedMask;
            USHORT usWPAMSDUPriority;
            DOT11_IV48_COUNTER dot11LowestIV48Counter;
            USHORT usDot11LeftRWBitMap;
            DOT11_IV48_COUNTER dot11HighestIV48Counter;
            USHORT usDot11RightRWBitMap;
            USHORT usNumberOfMPDUsReceived;
        } DOT11_RECV_CONTEXT, * PDOT11_RECV_CONTEXT;
        #define DOT11_RECV_CONTEXT_SIZE             \
                ((sizeof(DOT11_RECV_CONTEXT) + \
                  MEMORY_ALLOCATION_ALIGNMENT - 1) &  \
                 ~(MEMORY_ALLOCATION_ALIGNMENT-1))
    #else
        // Recv path extension for NativeWiFi NDIS 5.1 miniport driver
        typedef struct _DOT11_RECV_EXTENSION_INFO {
            ULONG uVersion;
            PVOID pvReserved;
            DOT11_PHY_TYPE dot11PhyType;
            ULONG uChCenterFrequency;
            LONG lRSSI;
            LONG lRSSIMin;
            LONG lRSSIMax;
            ULONG uRSSI;
            UCHAR ucPriority;
            UCHAR ucDataRate;
            UCHAR ucPeerMacAddress[6];
            ULONG dwExtendedStatus;
            HANDLE hWEPOffloadContext;
            HANDLE hAuthOffloadContext;
            USHORT usWEPAppliedMask;
            USHORT usWPAMSDUPriority;
            DOT11_IV48_COUNTER dot11LowestIV48Counter;
            USHORT usDot11LeftRWBitMap;
            DOT11_IV48_COUNTER dot11HighestIV48Counter;
            USHORT usDot11RightRWBitMap;
            USHORT usNumberOfMPDUsReceived;
            USHORT usNumberOfFragments;
            // PNDIS_PACKET pNdisPackets[1];        // Must be the last field.
            PVOID pNdisPackets[1];                  // Must be the last field.
        } DOT11_RECV_EXTENSION_INFO, * PDOT11_RECV_EXTENSION_INFO;

        typedef struct _DOT11_RECV_EXTENSION_INFO_V2 {
            ULONG uVersion;
            PVOID pvReserved;
            DOT11_PHY_TYPE dot11PhyType;
            ULONG uChCenterFrequency;
            LONG lRSSI;
            ULONG uRSSI;
            UCHAR ucPriority;
            UCHAR ucDataRate;
            UCHAR ucPeerMacAddress[6];
            ULONG dwExtendedStatus;
            HANDLE hWEPOffloadContext;
            HANDLE hAuthOffloadContext;
            USHORT usWEPAppliedMask;
            USHORT usWPAMSDUPriority;
            DOT11_IV48_COUNTER dot11LowestIV48Counter;
            USHORT usDot11LeftRWBitMap;
            DOT11_IV48_COUNTER dot11HighestIV48Counter;
            USHORT usDot11RightRWBitMap;
            USHORT usNumberOfMPDUsReceived;
            USHORT usNumberOfFragments;
            // PNDIS_PACKET pNdisPackets[1];        // Must be the last field.
            PVOID pNdisPackets[1];                  // Must be the last field.
        } DOT11_RECV_EXTENSION_INFO_V2, * PDOT11_RECV_EXTENSION_INFO_V2;
#endif // NDIS_MINIPORT_DRIVER

//
// 802.11 Status Codes
//

#define DOT11_STATUS_SUCCESS                        0x00000001

#define DOT11_STATUS_RETRY_LIMIT_EXCEEDED           0x00000002

#define DOT11_STATUS_UNSUPPORTED_PRIORITY           0x00000004

#define DOT11_STATUS_UNSUPPORTED_SERVICE_CLASS      0x00000008

#define DOT11_STATUS_UNAVAILABLE_PRIORITY           0x00000010

#define DOT11_STATUS_UNAVAILABLE_SERVICE_CLASS      0x00000020

#define DOT11_STATUS_XMIT_MSDU_TIMER_EXPIRED        0x00000040

#define DOT11_STATUS_UNAVAILABLE_BSS                0x00000080

#define DOT11_STATUS_EXCESSIVE_DATA_LENGTH          0x00000100

#define DOT11_STATUS_ENCRYPTION_FAILED              0x00000200

#define DOT11_STATUS_WEP_KEY_UNAVAILABLE            0x00000400

#define DOT11_STATUS_ICV_VERIFIED                   0x00000800

#define DOT11_STATUS_PACKET_REASSEMBLED             0x00001000

#define DOT11_STATUS_PACKET_NOT_REASSEMBLED         0x00002000

#define DOT11_STATUS_GENERATE_AUTH_FAILED           0x00004000

#define DOT11_STATUS_AUTH_NOT_VERIFIED              0x00008000

#define DOT11_STATUS_AUTH_VERIFIED                  0x00010000

#define DOT11_STATUS_AUTH_FAILED                    0x00020000

#define DOT11_STATUS_PS_LIFETIME_EXPIRED            0x00040000

//
// Flags for NDIS_STATUS_MEDIA_SPECIFIC_INDICATION
//
typedef struct _DOT11_STATUS_INDICATION {
    ULONG uStatusType;
    NDIS_STATUS ndisStatus;
} DOT11_STATUS_INDICATION, * PDOT11_STATUS_INDICATION;

#define DOT11_STATUS_RESET_CONFIRM                      4

#define DOT11_STATUS_SCAN_CONFIRM                   1
#define DOT11_STATUS_JOIN_CONFIRM                   2
#define DOT11_STATUS_START_CONFIRM                  3
#define DOT11_STATUS_AP_JOIN_CONFIRM                5
#define DOT11_STATUS_MPDU_MAX_LENGTH_CHANGED        6

typedef struct DOT11_MPDU_MAX_LENGTH_INDICATION {
    #define DOT11_MPDU_MAX_LENGTH_INDICATION_REVISION_1  1
    NDIS_OBJECT_HEADER Header;
    ULONG uPhyId;
    ULONG uMPDUMaxLength;
} DOT11_MPDU_MAX_LENGTH_INDICATION, * PDOT11_MPDU_MAX_LENGTH_INDICATION;

typedef struct DOT11_ASSOCIATION_START_PARAMETERS {
    #define DOT11_ASSOCIATION_START_PARAMETERS_REVISION_1  1
    NDIS_OBJECT_HEADER Header;
    DOT11_MAC_ADDRESS MacAddr;
    DOT11_SSID SSID;
    ULONG uIHVDataOffset, uIHVDataSize;
} DOT11_ASSOCIATION_START_PARAMETERS, * PDOT11_ASSOCIATION_START_PARAMETERS;

#define DOT11_ENCAP_RFC_1042     1
#define DOT11_ENCAP_802_1H       2
typedef struct DOT11_ENCAP_ENTRY {
    USHORT usEtherType;
    USHORT usEncapType; // either DOT11_ENCAP_RFC_1042 or DOT11_ENCAP_802_1H
} DOT11_ENCAP_ENTRY, * PDOT11_ENCAP_ENTRY;

typedef enum DOT11_DS_INFO {
    DOT11_DS_CHANGED,
    DOT11_DS_UNCHANGED,
    DOT11_DS_UNKNOWN
} DOT11_DS_INFO, * PDOT11_DS_INFO;

/////////////////////////////////////////////
// Definitions of association status codes
//
typedef ULONG DOT11_ASSOC_STATUS;

// The association is successful
#define DOT11_ASSOC_STATUS_SUCCESS                          0

// Generic association failure
#define DOT11_ASSOC_STATUS_FAILURE                          0x00000001U

// The association fails because the peer is not responding.
// Scenarios:
//    1. the peer doesn't respond to 802.11 authentication frames or
//       802.11 association request frames or probe request frames.
//    2. the NIC hasn't received beacon from the peer for substantial
//       amount of time. The timeout value here is NIC specific.
//    3. any other cases in which NIC determines that the peer is not
//       responsive.
#define DOT11_ASSOC_STATUS_UNREACHABLE                      0x00000002U

// The association fails because the radio is turned off
#define DOT11_ASSOC_STATUS_RADIO_OFF                        0x00000003U

// The association fails because the PHY is disabled. Here the PHY
// entity becomes unavailable to the OS. But the radio itself is not
// necessarily turned off.
#define DOT11_ASSOC_STATUS_PHY_DISABLED                     0x00000004U

// The association is cancelled (for example, the NIC is reset)
#define DOT11_ASSOC_STATUS_CANCELLED                        0x00000005U

// The connection fails because all the candidate AP has been tried
// and none of the attempts succeeds.
#define DOT11_ASSOC_STATUS_CANDIDATE_LIST_EXHAUSTED         0x00000006U

// The current association is disassociated as requested by the OS
// (through either a OID_DOT11_RESET_REQUEST or OID_DOT11_DISCONNECT request)
#define DOT11_ASSOC_STATUS_DISASSOCIATED_BY_OS              0x00000007U

// The current association is disassociated because the NIC roams
// to new AP.
// This error code is used for indicating the implicit dissassociation
// done by the nwifi.sys. Miniport driver usually doesn't generate
// this error code (since the disassociation is automatically done
// by the nwifi.sys).
#define DOT11_ASSOC_STATUS_DISASSOCIATED_BY_ROAMING         0x00000008U

// The current association is disassocated because the NIC is reset
#define DOT11_ASSOC_STATUS_DISASSOCIATED_BY_RESET           0x00000009U

// The current association is disassocated because the NIC is reset
#define DOT11_ASSOC_STATUS_SYSTEM_ERROR                     0x0000000aU

// Roaming reason: find a better AP
#define DOT11_ASSOC_STATUS_ROAMING_BETTER_AP_FOUND          0x0000000bU

// Roaming reason: the association to the current BSS is lost
#define DOT11_ASSOC_STATUS_ROAMING_ASSOCIATION_LOST         0x0000000cU

// Roaming reason: adhoc roaming (network Coalescing)
#define DOT11_ASSOC_STATUS_ROAMING_ADHOC                    0x0000000dU

// The new association fails or the current association is disassocated
// because the NIC receives an 802.11 de-authentication frame from the
// peer. The lowest 16-bits are the reason code (2-byte) copied from
// the 802.11 DeAuthentication frame.
#define DOT11_ASSOC_STATUS_PEER_DEAUTHENTICATED             0x00010000U
#define DOT11_ASSOC_STATUS_PEER_DEAUTHENTICATED_START       DOT11_ASSOC_STATUS_PEER_DEAUTHENTICATED
#define DOT11_ASSOC_STATUS_PEER_DEAUTHENTICATED_END         0x0001ffffU

// The new association fails or the current association is disassocated
// because the NIC receives an 802.11 disassociation frame from the
// peer. The lowest 16-bits are the reason code (2-byte) copied from
// the 802.11 DisAssociation frame.
#define DOT11_ASSOC_STATUS_PEER_DISASSOCIATED               0x00020000U
#define DOT11_ASSOC_STATUS_PEER_DISASSOCIATED_START         DOT11_ASSOC_STATUS_PEER_DISASSOCIATED
#define DOT11_ASSOC_STATUS_PEER_DISASSOCIATED_END           0x0002ffffU

#define DOT11_ASSOC_STATUS_ASSOCIATION_RESPONSE             0x00030000U
#define DOT11_ASSOC_STATUS_ASSOCIATION_RESPONSE_START       DOT11_ASSOC_STATUS_ASSOCIATION_RESPONSE
#define DOT11_ASSOC_STATUS_ASSOCIATION_RESPONSE_END         0x0003ffffU

// The mask for extracting 802.11 deauthentication and disassociation
// reason code.
#define DOT11_ASSOC_STATUS_REASON_CODE_MASK                 0xffffU

// Define the range of IHV specific association status codes
#define DOT11_ASSOC_STATUS_IHV_START                        0x80000000U
#define DOT11_ASSOC_STATUS_IHV_END                          0xffffffffU

typedef struct DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    #define DOT11_ASSOCIATION_COMPLETION_PARAMETERS_REVISION_1  1
    #define DOT11_ASSOCIATION_COMPLETION_PARAMETERS_REVISION_2  2
    NDIS_OBJECT_HEADER Header;
    DOT11_MAC_ADDRESS MacAddr;

    DOT11_ASSOC_STATUS uStatus;

    BOOLEAN bReAssocReq;
    BOOLEAN bReAssocResp;
    ULONG uAssocReqOffset, uAssocReqSize;
    ULONG uAssocRespOffset, uAssocRespSize;
    ULONG uBeaconOffset, uBeaconSize;
    ULONG uIHVDataOffset, uIHVDataSize;

    // The following fields are applicable for successful association.
    // For association failure, they must be zero-ed out.
    DOT11_AUTH_ALGORITHM AuthAlgo;
    DOT11_CIPHER_ALGORITHM UnicastCipher;
    DOT11_CIPHER_ALGORITHM MulticastCipher;
    ULONG uActivePhyListOffset, uActivePhyListSize;
    BOOLEAN bFourAddressSupported;
    BOOLEAN bPortAuthorized;

    // The QoS protocol which is used in this association.
    // It is zero or combination of DOT11_QOS_PROTOCOL_FLAG_WMM and/or DOT11_QOS_PROTOCOL_FLAG_11E
    UCHAR ucActiveQoSProtocol;

    DOT11_DS_INFO DSInfo;
    ULONG uEncapTableOffset, uEncapTableSize;

#if (NTDDI_VERSION >= NTDDI_WIN8)
    //802.11w parameters
    DOT11_CIPHER_ALGORITHM MulticastMgmtCipher;
    ULONG uAssocComebackTime;
#endif
} DOT11_ASSOCIATION_COMPLETION_PARAMETERS, * PDOT11_ASSOCIATION_COMPLETION_PARAMETERS;

typedef struct DOT11_CONNECTION_START_PARAMETERS {
    #define DOT11_CONNECTION_START_PARAMETERS_REVISION_1  1
    NDIS_OBJECT_HEADER Header;
    DOT11_BSS_TYPE BSSType;

    DOT11_MAC_ADDRESS AdhocBSSID;   // applicable to adhoc mode only
    DOT11_SSID AdhocSSID;   // applicable to adhoc mode only
} DOT11_CONNECTION_START_PARAMETERS, * PDOT11_CONNECTION_START_PARAMETERS;

// For uStatus in DOT11_CONNECTION_COMPLETION_PARAMETERS and DOT11_ROAMING_COMPLETION_PARAMETERS
#define DOT11_CONNECTION_STATUS_SUCCESS                     DOT11_ASSOC_STATUS_SUCCESS
#define DOT11_CONNECTION_STATUS_FAILURE                     DOT11_ASSOC_STATUS_FAILURE
#define DOT11_CONNECTION_STATUS_CANDIDATE_LIST_EXHAUSTED    DOT11_ASSOC_STATUS_CANDIDATE_LIST_EXHAUSTED
#define DOT11_CONNECTION_STATUS_PHY_POWER_DOWN              DOT11_ASSOC_STATUS_RADIO_OFF
#define DOT11_CONNECTION_STATUS_CANCELLED                   DOT11_ASSOC_STATUS_CANCELLED
#define DOT11_CONNECTION_STATUS_IHV_START                   DOT11_ASSOC_STATUS_IHV_START
#define DOT11_CONNECTION_STATUS_IHV_END                     DOT11_ASSOC_STATUS_IHV_END
typedef struct DOT11_CONNECTION_COMPLETION_PARAMETERS {
    #define DOT11_CONNECTION_COMPLETION_PARAMETERS_REVISION_1  1
    NDIS_OBJECT_HEADER Header;
    DOT11_ASSOC_STATUS uStatus;  // DOT11_CONNECTION_STATUS_XXXX
} DOT11_CONNECTION_COMPLETION_PARAMETERS, * PDOT11_CONNECTION_COMPLETION_PARAMETERS;

#define DOT11_ROAMING_REASON_BETTER_AP_FOUND    DOT11_ASSOC_STATUS_ROAMING_BETTER_AP_FOUND
#define DOT11_ROAMING_REASON_ASSOCIATION_LOST   DOT11_ASSOC_STATUS_ROAMING_ASSOCIATION_LOST
#define DOT11_ROAMING_REASON_ADHOC              DOT11_ASSOC_STATUS_ROAMING_ADHOC
#define DOT11_ROAMING_REASON_IHV_START          DOT11_ASSOC_STATUS_IHV_START
#define DOT11_ROAMING_REASON_IHV_END            DOT11_ASSOC_STATUS_IHV_END
typedef struct DOT11_ROAMING_START_PARAMETERS {
    #define DOT11_ROAMING_START_PARAMETERS_REVISION_1  1
    NDIS_OBJECT_HEADER Header;
    DOT11_MAC_ADDRESS AdhocBSSID;   // applicable to adhoc mode only
    DOT11_SSID AdhocSSID;   // applicable to adhoc mode only
    DOT11_ASSOC_STATUS uRoamingReason;
} DOT11_ROAMING_START_PARAMETERS, * PDOT11_ROAMING_START_PARAMETERS;

typedef struct DOT11_ROAMING_COMPLETION_PARAMETERS {
    #define DOT11_ROAMING_COMPLETION_PARAMETERS_REVISION_1  1
    NDIS_OBJECT_HEADER Header;
    DOT11_ASSOC_STATUS uStatus;  // DOT11_CONNECTION_STATUS_XXXX
} DOT11_ROAMING_COMPLETION_PARAMETERS, * PDOT11_ROAMING_COMPLETION_PARAMETERS;

// Disassociation Reason Code
#define DOT11_DISASSOC_REASON_OS                    DOT11_ASSOC_STATUS_DISASSOCIATED_BY_OS
#define DOT11_DISASSOC_REASON_PEER_UNREACHABLE      DOT11_ASSOC_STATUS_UNREACHABLE

// Current association is disassocated because the NIC receives an
// 802.11 de-authentication frame from the peer. The lowest 16-bits
// are the reason code (2-byte) copied from the 802.11 DeAuthentication frame.
#define DOT11_DISASSOC_REASON_PEER_DEAUTHENTICATED  DOT11_ASSOC_STATUS_PEER_DEAUTHENTICATED

// Current association is disassocated because the NIC receives an
// 802.11 disassociation frame from the peer. The lowest 16-bits
// are the reason code (2-byte) copied from the 802.11 disassociation frame.
#define DOT11_DISASSOC_REASON_PEER_DISASSOCIATED    DOT11_ASSOC_STATUS_PEER_DISASSOCIATED

#define DOT11_DISASSOC_REASON_RADIO_OFF             DOT11_ASSOC_STATUS_RADIO_OFF
#define DOT11_DISASSOC_REASON_PHY_DISABLED          DOT11_ASSOC_STATUS_PHY_DISABLED
#define DOT11_DISASSOC_REASON_IHV_START             DOT11_ASSOC_STATUS_IHV_START
#define DOT11_DISASSOC_REASON_IHV_END               DOT11_ASSOC_STATUS_IHV_END
typedef struct DOT11_DISASSOCIATION_PARAMETERS {
    #define DOT11_DISASSOCIATION_PARAMETERS_REVISION_1  1
    NDIS_OBJECT_HEADER Header;
    DOT11_MAC_ADDRESS MacAddr;
    DOT11_ASSOC_STATUS uReason;
    ULONG uIHVDataOffset, uIHVDataSize;
} DOT11_DISASSOCIATION_PARAMETERS, * PDOT11_DISASSOCIATION_PARAMETERS;

typedef struct DOT11_TKIPMIC_FAILURE_PARAMETERS {
    #define DOT11_TKIPMIC_FAILURE_PARAMETERS_REVISION_1  1
    NDIS_OBJECT_HEADER Header;
    BOOLEAN bDefaultKeyFailure;
    ULONG uKeyIndex;
    DOT11_MAC_ADDRESS PeerMac;
} DOT11_TKIPMIC_FAILURE_PARAMETERS, * PDOT11_TKIPMIC_FAILURE_PARAMETERS;

typedef struct DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    #define DOT11_PMKID_CANDIDATE_LIST_PARAMETERS_REVISION_1  1
    NDIS_OBJECT_HEADER Header;
    ULONG uCandidateListSize;
    ULONG uCandidateListOffset;
} DOT11_PMKID_CANDIDATE_LIST_PARAMETERS, * PDOT11_PMKID_CANDIDATE_LIST_PARAMETERS;

typedef struct DOT11_BSSID_CANDIDATE {
    DOT11_MAC_ADDRESS BSSID;

    #define DOT11_PMKID_CANDIDATE_PREAUTH_ENABLED   0x00000001U
    ULONG uFlags;
} DOT11_BSSID_CANDIDATE, *PDOT11_BSSID_CANDIDATE;

typedef struct DOT11_PHY_STATE_PARAMETERS {
    #define DOT11_PHY_STATE_PARAMETERS_REVISION_1  1
    NDIS_OBJECT_HEADER Header;
    ULONG uPhyId;
    BOOLEAN bHardwarePhyState;
    BOOLEAN bSoftwarePhyState;
} DOT11_PHY_STATE_PARAMETERS, * PDOT11_PHY_STATE_PARAMETERS;

typedef struct DOT11_LINK_QUALITY_ENTRY {
    DOT11_MAC_ADDRESS PeerMacAddr;
    UCHAR ucLinkQuality;
} DOT11_LINK_QUALITY_ENTRY, *PDOT11_LINK_QUALITY_ENTRY;

typedef struct DOT11_LINK_QUALITY_PARAMETERS {
    #define DOT11_LINK_QUALITY_PARAMETERS_REVISION_1  1
    NDIS_OBJECT_HEADER Header;
    ULONG uLinkQualityListSize;
    ULONG uLinkQualityListOffset;
} DOT11_LINK_QUALITY_PARAMETERS, * PDOT11_LINK_QUALITY_PARAMETERS;


// Send OOB data for ExtSTA mode
typedef struct DOT11_EXTSTA_SEND_CONTEXT {
    #define DOT11_EXTSTA_SEND_CONTEXT_REVISION_1  1
    NDIS_OBJECT_HEADER Header;
    USHORT usExemptionActionType;
    ULONG uPhyId;
    ULONG uDelayedSleepValue;

#ifdef __midl
    // For nwifi test, which pass this structure using midl
    ULONG_PTR pvMediaSpecificInfo;
#else
    PVOID pvMediaSpecificInfo;
#endif

    ULONG uSendFlags;           // reserved field for safe mode wireless
} DOT11_EXTSTA_SEND_CONTEXT, * PDOT11_EXTSTA_SEND_CONTEXT;

// Recv OOB data for ExtSTA mode
#define DOT11_RECV_FLAG_RAW_PACKET	        0x00000001U
#define DOT11_RECV_FLAG_RAW_PACKET_FCS_FAILURE  0x00000002U
#define DOT11_RECV_FLAG_RAW_PACKET_TIMESTAMP    0x00000004U
typedef struct DOT11_EXTSTA_RECV_CONTEXT {
    #define DOT11_EXTSTA_RECV_CONTEXT_REVISION_1  1
    NDIS_OBJECT_HEADER Header;
    ULONG uReceiveFlags;
    ULONG uPhyId;
    ULONG uChCenterFrequency;
    USHORT usNumberOfMPDUsReceived;
    LONG lRSSI;
    UCHAR ucDataRate;

    ULONG uSizeMediaSpecificInfo;

#ifdef __midl
    // For nwifi test, which pass this structure using midl
    ULONG_PTR pvMediaSpecificInfo;
#else
    PVOID pvMediaSpecificInfo;
#endif

    ULONGLONG ullTimestamp;

} DOT11_EXTSTA_RECV_CONTEXT, * PDOT11_EXTSTA_RECV_CONTEXT;

//
// Private 802.11 OIDs: this should be the last section
//
// We reserve 1024 entries for real DOT11 OIDs
//

#define OID_DOT11_PRIVATE_OIDS_START                (OID_DOT11_NDIS_START + 1024)

#define OID_DOT11_CURRENT_ADDRESS                   (OID_DOT11_PRIVATE_OIDS_START + 2)
    // DOT11_MAC_ADDRESS

#define OID_DOT11_PERMANENT_ADDRESS                 (OID_DOT11_PRIVATE_OIDS_START + 3)
    // DOT11_MAC_ADDRESS

#define OID_DOT11_MULTICAST_LIST                    (OID_DOT11_PRIVATE_OIDS_START + 4)
    // OID_802_3_MULTICAST_LIST

#define OID_DOT11_MAXIMUM_LIST_SIZE                 (OID_DOT11_PRIVATE_OIDS_START + 5)


// GUIDs for WMI

#define DEFINE_NWF_GUID(name,ord)   \
    DEFINE_GUID(name, 0x6cb9a43e + (ord), 0xc45f, 0x4039, 0x9f, 0xe6, 0xd0, 0x8c, 0xb0, 0x57, 0x18, 0x4c)


DEFINE_NWF_GUID(GUID_NWF_OFFLOAD_CAPABILITY,0);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_OFFLOAD_CAPABILITY,1);
DEFINE_NWF_GUID(GUID_NWF_WEP_OFFLOAD,2);
DEFINE_NWF_GUID(GUID_NWF_WEP_UPLOAD,3);
DEFINE_NWF_GUID(GUID_NWF_DEFAULT_WEP_OFFLOAD,4);
DEFINE_NWF_GUID(GUID_NWF_DEFAULT_WEP_UPLOAD,5);
DEFINE_NWF_GUID(GUID_NWF_MPDU_MAX_LENGTH,6);
DEFINE_NWF_GUID(GUID_NWF_OPERATION_MODE_CAPABILITY,7);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_OPERATION_MODE,8);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_PACKET_FILTER,9);
DEFINE_NWF_GUID(GUID_NWF_ATIM_WINDOW,10);
DEFINE_NWF_GUID(GUID_NWF_SCAN_REQUEST,11);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_PHY_TYPE,12);
DEFINE_NWF_GUID(GUID_NWF_JOIN_REQUEST,13);
DEFINE_NWF_GUID(GUID_NWF_START_REQUEST,14);
DEFINE_NWF_GUID(GUID_NWF_UPDATE_IE,15);
DEFINE_NWF_GUID(GUID_NWF_RESET_REQUEST,16);
DEFINE_NWF_GUID(GUID_NWF_NIC_POWER_STATE,17);
DEFINE_NWF_GUID(GUID_NWF_OPTIONAL_CAPABILITY,18);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_OPTIONAL_CAPABILITY,19);
DEFINE_NWF_GUID(GUID_NWF_STATION_ID,20);
DEFINE_NWF_GUID(GUID_NWF_MEDIUM_OCCUPANCY_LIMIT,21);
DEFINE_NWF_GUID(GUID_NWF_CF_POLLABLE,22);
DEFINE_NWF_GUID(GUID_NWF_CFP_PERIOD,23);
DEFINE_NWF_GUID(GUID_NWF_CFP_MAX_DURATION,24);
DEFINE_NWF_GUID(GUID_NWF_POWER_MGMT_MODE,25);
DEFINE_NWF_GUID(GUID_NWF_OPERATIONAL_RATE_SET,26);
DEFINE_NWF_GUID(GUID_NWF_BEACON_PERIOD,27);
DEFINE_NWF_GUID(GUID_NWF_DTIM_PERIOD,28);
DEFINE_NWF_GUID(GUID_NWF_WEP_ICV_ERROR_COUNT,29);
DEFINE_NWF_GUID(GUID_NWF_MAC_ADDRESS,30);
DEFINE_NWF_GUID(GUID_NWF_RTS_THRESHOLD,31);
DEFINE_NWF_GUID(GUID_NWF_SHORT_RETRY_LIMIT,32);
DEFINE_NWF_GUID(GUID_NWF_LONG_RETRY_LIMIT,33);
DEFINE_NWF_GUID(GUID_NWF_FRAGMENTATION_THRESHOLD,34);
DEFINE_NWF_GUID(GUID_NWF_MAX_TRANSMIT_MSDU_LIFETIME,35);
DEFINE_NWF_GUID(GUID_NWF_MAX_RECEIVE_LIFETIME,36);
DEFINE_NWF_GUID(GUID_NWF_COUNTERS_ENTRY,37);
DEFINE_NWF_GUID(GUID_NWF_SUPPORTED_PHY_TYPES,38);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_REG_DOMAIN,39);
DEFINE_NWF_GUID(GUID_NWF_TEMP_TYPE,40);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_TX_ANTENNA,41);
DEFINE_NWF_GUID(GUID_NWF_DIVERSITY_SUPPORT,42);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_RX_ANTENNA,43);
DEFINE_NWF_GUID(GUID_NWF_SUPPORTED_POWER_LEVELS,44);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_TX_POWER_LEVEL,45);
DEFINE_NWF_GUID(GUID_NWF_HOP_TIME,46);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_CHANNEL_NUMBER,47);
DEFINE_NWF_GUID(GUID_NWF_MAX_DWELL_TIME,48);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_DWELL_TIME,49);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_SET,50);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_PATTERN,51);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_INDEX,52);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_CHANNEL,53);
DEFINE_NWF_GUID(GUID_NWF_CCA_MODE_SUPPORTED,54);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_CCA_MODE,55);
DEFINE_NWF_GUID(GUID_NWF_ED_THRESHOLD,56);
DEFINE_NWF_GUID(GUID_NWF_CCA_WATCHDOG_TIMER_MAX,57);
DEFINE_NWF_GUID(GUID_NWF_CCA_WATCHDOG_COUNT_MAX,58);
DEFINE_NWF_GUID(GUID_NWF_CCA_WATCHDOG_TIMER_MIN,59);
DEFINE_NWF_GUID(GUID_NWF_CCA_WATCHDOG_COUNT_MIN,60);
DEFINE_NWF_GUID(GUID_NWF_REG_DOMAINS_SUPPORT_VALUE,61);
DEFINE_NWF_GUID(GUID_NWF_SUPPORTED_TX_ANTENNA,62);
DEFINE_NWF_GUID(GUID_NWF_SUPPORTED_RX_ANTENNA,63);
DEFINE_NWF_GUID(GUID_NWF_DIVERSITY_SELECTION_RX,64);
DEFINE_NWF_GUID(GUID_NWF_SUPPORTED_DATA_RATES_VALUE,65);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_FREQUENCY,66);
DEFINE_NWF_GUID(GUID_NWF_TI_THRESHOLD,67);
DEFINE_NWF_GUID(GUID_NWF_FREQUENCY_BANDS_SUPPORTED,68);
DEFINE_NWF_GUID(GUID_NWF_SHORT_PREAMBLE_OPTION_IMPLEMENTED,69);
DEFINE_NWF_GUID(GUID_NWF_PBCC_OPTION_IMPLEMENTED,70);
DEFINE_NWF_GUID(GUID_NWF_CHANNEL_AGILITY_PRESENT,71);
DEFINE_NWF_GUID(GUID_NWF_CHANNEL_AGILITY_ENABLED,72);
DEFINE_NWF_GUID(GUID_NWF_HR_CCA_MODE_SUPPORTED,73);
DEFINE_NWF_GUID(GUID_NWF_MULTI_DOMAIN_CAPABILITY_IMPLEMENTED,74);
DEFINE_NWF_GUID(GUID_NWF_MULTI_DOMAIN_CAPABILITY_ENABLED,75);
DEFINE_NWF_GUID(GUID_NWF_COUNTRY_STRING,76);
DEFINE_NWF_GUID(GUID_NWF_MULTI_DOMAIN_CAPABILITY,77);
DEFINE_NWF_GUID(GUID_NWF_EHCC_PRIME_RADIX,78);
DEFINE_NWF_GUID(GUID_NWF_EHCC_NUMBER_OF_CHANNELS_FAMILY_INDEX,79);
DEFINE_NWF_GUID(GUID_NWF_EHCC_CAPABILITY_IMPLEMENTED,80);
DEFINE_NWF_GUID(GUID_NWF_EHCC_CAPABILITY_ENABLED,81);
DEFINE_NWF_GUID(GUID_NWF_HOP_ALGORITHM_ADOPTED,82);
DEFINE_NWF_GUID(GUID_NWF_RANDOM_TABLE_FLAG,83);
DEFINE_NWF_GUID(GUID_NWF_NUMBER_OF_HOPPING_SETS,84);
DEFINE_NWF_GUID(GUID_NWF_HOP_MODULUS,85);
DEFINE_NWF_GUID(GUID_NWF_HOP_OFFSET,86);
DEFINE_NWF_GUID(GUID_NWF_HOPPING_PATTERN,87);
DEFINE_NWF_GUID(GUID_NWF_RANDOM_TABLE_FIELD_NUMBER,88);
DEFINE_NWF_GUID(GUID_NWF_WPA_TSC,89);
DEFINE_NWF_GUID(GUID_NWF_RSSI_RANGE,90);
DEFINE_NWF_GUID(GUID_NWF_RF_USAGE,91);
DEFINE_NWF_GUID(GUID_NWF_NIC_SPECIFIC_EXTENSION,92);
DEFINE_NWF_GUID(GUID_NWF_AP_JOIN_REQUEST,93);
DEFINE_NWF_GUID(GUID_NWF_ERP_PBCC_OPTION_IMPLEMENTED,94);
DEFINE_NWF_GUID(GUID_NWF_ERP_PBCC_OPTION_ENABLED,95);
DEFINE_NWF_GUID(GUID_NWF_DSSS_OFDM_OPTION_IMPLEMENTED,96);
DEFINE_NWF_GUID(GUID_NWF_DSSS_OFDM_OPTION_ENABLED,97);
DEFINE_NWF_GUID(GUID_NWF_SHORT_SLOT_TIME_OPTION_IMPLEMENTED,98);
DEFINE_NWF_GUID(GUID_NWF_SHORT_SLOT_TIME_OPTION_ENABLED,99);
DEFINE_NWF_GUID(GUID_NWF_MAX_MAC_ADDRESS_STATES,100);
DEFINE_NWF_GUID(GUID_NWF_RECV_SENSITIVITY_LIST,101);
DEFINE_NWF_GUID(GUID_NWF_WME_IMPLEMENTED,102);
DEFINE_NWF_GUID(GUID_NWF_WME_ENABLED,103);
DEFINE_NWF_GUID(GUID_NWF_WME_AC_PARAMETERS,104);
DEFINE_NWF_GUID(GUID_NWF_WME_UPDATE_IE,105);
DEFINE_NWF_GUID(GUID_NWF_QOS_TX_QUEUES_SUPPORTED,106);
DEFINE_NWF_GUID(GUID_NWF_QOS_TX_DURATION,107);
DEFINE_NWF_GUID(GUID_NWF_QOS_TX_MEDIUM_TIME,108);
DEFINE_NWF_GUID(GUID_NWF_SUPPORTED_OFDM_FREQUENCY_LIST,109);
DEFINE_NWF_GUID(GUID_NWF_SUPPORTED_DSSS_CHANNEL_LIST,110);
DEFINE_NWF_GUID(GUID_NWF_AUTO_CONFIG_ENABLED,120);
DEFINE_NWF_GUID(GUID_NWF_ENUM_BSS_LIST,121);
DEFINE_NWF_GUID(GUID_NWF_FLUSH_BSS_LIST,122);
DEFINE_NWF_GUID(GUID_NWF_POWER_MGMT_REQUEST,123);
DEFINE_NWF_GUID(GUID_NWF_DESIRED_SSID_LIST,124);
DEFINE_NWF_GUID(GUID_NWF_EXCLUDED_MAC_ADDRESS_LIST,125);
DEFINE_NWF_GUID(GUID_NWF_DESIRED_BSSID_LIST,126);
DEFINE_NWF_GUID(GUID_NWF_DESIRED_BSS_TYPE,127);
DEFINE_NWF_GUID(GUID_NWF_PMKID_LIST,128);
DEFINE_NWF_GUID(GUID_NWF_CONNECT_REQUEST,129);
DEFINE_NWF_GUID(GUID_NWF_EXCLUDE_UNENCRYPTED,130);
DEFINE_NWF_GUID(GUID_NWF_STATISTICS,131);
DEFINE_NWF_GUID(GUID_NWF_PRIVACY_EXEMPTION_LIST,132);
DEFINE_NWF_GUID(GUID_NWF_ENABLED_AUTHENTICATION_ALGORITHM,133);
DEFINE_NWF_GUID(GUID_NWF_SUPPORTED_UNICAST_ALGORITHM_PAIR,134);
DEFINE_NWF_GUID(GUID_NWF_ENABLED_UNICAST_CIPHER_ALGORITHM,135);
DEFINE_NWF_GUID(GUID_NWF_SUPPORTED_MULTICAST_ALGORITHM_PAIR,136);
DEFINE_NWF_GUID(GUID_NWF_ENABLED_MULTICAST_CIPHER_ALGORITHM,137);
DEFINE_NWF_GUID(GUID_NWF_CIPHER_DEFAULT_KEY_ID,138);
DEFINE_NWF_GUID(GUID_NWF_CIPHER_DEFAULT_KEY,139);
DEFINE_NWF_GUID(GUID_NWF_CIPHER_KEY_MAPPING_KEY,140);
DEFINE_NWF_GUID(GUID_NWF_ENUM_ASSOCIATION_INFO,141);
DEFINE_NWF_GUID(GUID_NWF_DISCONNECT_REQUEST,142);
DEFINE_NWF_GUID(GUID_NWF_UNICAST_USE_GROUP_ENABLED,143);
DEFINE_NWF_GUID(GUID_NWF_PHY_STATE,144);
DEFINE_NWF_GUID(GUID_NWF_DESIRED_PHY_LIST,145);
DEFINE_NWF_GUID(GUID_NWF_CURRENT_PHY_ID,146);
DEFINE_NWF_GUID(GUID_NWF_MEDIA_STREAMING_ENABLED,147);
DEFINE_NWF_GUID(GUID_NWF_UNREACHABLE_DETECTION_THRESHOLD,148);
DEFINE_NWF_GUID(GUID_NWF_ACTIVE_PHY_LIST,149);
DEFINE_NWF_GUID(GUID_NWF_EXTSTA_CAPABILITY,150);
DEFINE_NWF_GUID(GUID_NWF_DATA_RATE_MAPPING_TABLE,151);
DEFINE_NWF_GUID(GUID_NWF_SUPPORTED_COUNTRY_OR_REGION_STRING,152);
DEFINE_NWF_GUID(GUID_NWF_DESIRED_COUNTRY_OR_REGION_STRING,153);
DEFINE_NWF_GUID(GUID_NWF_PORT_STATE_NOTIFICATION,154);
DEFINE_NWF_GUID(GUID_NWF_IBSS_PARAMS,155);
DEFINE_NWF_GUID(GUID_NWF_QOS_PARAMS,156);
DEFINE_NWF_GUID(GUID_NWF_SAFE_MODE_ENABLED,157);
DEFINE_NWF_GUID(GUID_NWF_HIDDEN_NETWORK_ENABLED,158);
DEFINE_NWF_GUID(GUID_NWF_ASSOCIATION_PARAMS,159);

DEFINE_NWF_GUID(GUID_NWF_CURRENT_ADDRESS,1024+2);
DEFINE_NWF_GUID(GUID_NWF_PERMANENT_ADDRESS,1024+3);
DEFINE_NWF_GUID(GUID_NWF_MULTICAST_LIST,1024+4);
DEFINE_NWF_GUID(GUID_NWF_MAXIMUM_LIST_SIZE,1024+5);

#endif  // (NTDDI_VERSION > NTDDI_VISTA)





#ifdef NWF_EXTAP_SUPPORTED



#define DOT11_EXTAP_ATTRIBUTES_REVISION_1  1
typedef
struct _DOT11_EXTAP_ATTRIBUTES
{
    NDIS_OBJECT_HEADER Header;

    ULONG           uScanSSIDListSize;
    ULONG           uDesiredSSIDListSize;
    ULONG           uPrivacyExemptionListSize;
    ULONG           uAssociationTableSize;
    ULONG           uDefaultKeyTableSize;
    ULONG           uWEPKeyValueMaxLength;
    BOOLEAN         bStrictlyOrderedServiceClassImplemented;

    // Supported Country Strings
    ULONG                               uNumSupportedCountryOrRegionStrings;
    #ifdef __midl
    [size_is(uNumSupportedCountryOrRegionStrings)]
    #endif
    PDOT11_COUNTRY_OR_REGION_STRING     pSupportedCountryOrRegionStrings;



    // Unicast algorithm capabilities
    ULONG                               uInfraNumSupportedUcastAlgoPairs;
    #ifdef __midl
    [size_is(uInfraNumSupportedUcastAlgoPairs)]
    #endif
    PDOT11_AUTH_CIPHER_PAIR             pInfraSupportedUcastAlgoPairs;


    // Multicast algorithm capabilities
    ULONG                               uInfraNumSupportedMcastAlgoPairs;
    #ifdef __midl
    [size_is(uInfraNumSupportedMcastAlgoPairs)]
    #endif
    PDOT11_AUTH_CIPHER_PAIR             pInfraSupportedMcastAlgoPairs;

}
DOT11_EXTAP_ATTRIBUTES, *PDOT11_EXTAP_ATTRIBUTES;

#define NDIS_SIZEOF_DOT11_EXTAP_ATTRIBUTES_REVISION_1 sizeof(DOT11_EXTAP_ATTRIBUTES)




// NDIS indications for Ext AP mode.




// Data type for NDIS_STATUS_DOT11_INCOMING_ASSOC_STARTED

#define DOT11_INCOMING_ASSOC_STARTED_PARAMETERS_REVISION_1  1
    typedef
    struct _DOT11_INCOMING_ASSOC_STARTED_PARAMETERS
    {
        NDIS_OBJECT_HEADER  Header;
        DOT11_MAC_ADDRESS   PeerMacAddr;
    }
    DOT11_INCOMING_ASSOC_STARTED_PARAMETERS, *PDOT11_INCOMING_ASSOC_STARTED_PARAMETERS;




// Data type for NDIS_STATUS_DOT11_INCOMING_ASSOC_REQUEST_RECEIVED

#define DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS_REVISION_1  1
    typedef
    struct _DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS
    {
        NDIS_OBJECT_HEADER  Header;
        DOT11_MAC_ADDRESS   PeerMacAddr;
        BOOLEAN             bReAssocReq;
        ULONG               uAssocReqOffset;
        ULONG               uAssocReqSize;
    }
    DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS, *PDOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS;






// Data type for NDIS_STATUS_DOT11_INCOMING_ASSOC_COMPLETION

#define DOT11_ASSOC_ERROR_SOURCE_OS	        0x0
#define DOT11_ASSOC_ERROR_SOURCE_REMOTE	    0x01
#define DOT11_ASSOC_ERROR_SOURCE_OTHER	    0xFF


#define DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS_REVISION_1  1
    typedef
    struct _DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS
    {
        NDIS_OBJECT_HEADER          Header;
        DOT11_MAC_ADDRESS           PeerMacAddr;

        ULONG                       uStatus;
        UCHAR                       ucErrorSource;

        BOOLEAN                     bReAssocReq;
        BOOLEAN                     bReAssocResp;

        ULONG                       uAssocReqOffset;
        ULONG                       uAssocReqSize;

        ULONG                       uAssocRespOffset;
        ULONG                       uAssocRespSize;

        // The following fields are applicable for successful association.
        // For association failure, they must be zero-ed out.

        DOT11_AUTH_ALGORITHM        AuthAlgo;
        DOT11_CIPHER_ALGORITHM      UnicastCipher;
        DOT11_CIPHER_ALGORITHM      MulticastCipher;
        ULONG                       uActivePhyListOffset;
        ULONG                       uActivePhyListSize;

        ULONG                       uBeaconOffset;
        ULONG                       uBeaconSize;
    }
    DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS, *PDOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS;


// Data type for NDIS_STATUS_DOT11_STOP_AP

#define DOT11_STOP_AP_PARAMETERS_REVISION_1  1
    typedef
    struct _DOT11_STOP_AP_PARAMETERS
    {
        NDIS_OBJECT_HEADER  Header;
        ULONG               ulReason;
    }
    DOT11_STOP_AP_PARAMETERS, *PDOT11_STOP_AP_PARAMETERS;


#define DOT11_STOP_AP_REASON_FREQUENCY_NOT_AVAILABLE    0x1
#define DOT11_STOP_AP_REASON_CHANNEL_NOT_AVAILABLE      0x2
#define DOT11_STOP_AP_REASON_AP_ACTIVE                  0x3


#define DOT11_STOP_AP_REASON_IHV_START  0xFF000000
#define DOT11_STOP_AP_REASON_IHV_END    0xFFFFFFFF








// Data type for NDIS_STATUS_DOT11_PHY_FREQUENCY_ADOPTED


#define DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_REVISION_1  1
typedef
struct _DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS
{
    NDIS_OBJECT_HEADER  Header;
    ULONG               ulPhyId;
    union
    {
        ULONG   ulChannel;
        ULONG   ulFrequency;
    };
}
DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS, *PDOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS;



// Data type for NDIS_STATUS_DOT11_CAN_SUSTAIN_AP

#define DOT11_CAN_SUSTAIN_AP_PARAMETERS_REVISION_1  1
    typedef
    struct _DOT11_CAN_SUSTAIN_AP_PARAMETERS
    {
        NDIS_OBJECT_HEADER  Header;
        ULONG               ulReason;
    }
    DOT11_CAN_SUSTAIN_AP_PARAMETERS, *PDOT11_CAN_SUSTAIN_AP_PARAMETERS;


#define DOT11_CAN_SUSTAIN_AP_REASON_IHV_START   0xFF000000
#define DOT11_CAN_SUSTAIN_AP_REASON_IHV_END     0xFFFFFFFF



// TAG for Ext AP specific OIDs.
#define NWF_EXTAP_OID      (0x03U)




#define OID_DOT11_WPS_ENABLED                       NWF_DEFINE_OID(0x01, NWF_EXTAP_OID, NWF_MANDATORY_OID)
    // BOOLEAN



#define OID_DOT11_START_AP_REQUEST                  NWF_DEFINE_OID(0x02, NWF_EXTAP_OID, NWF_MANDATORY_OID)
    // VOID



#define OID_DOT11_AVAILABLE_CHANNEL_LIST            NWF_DEFINE_OID(0x03, NWF_EXTAP_OID, NWF_MANDATORY_OID)
    // DOT11_AVAILABLE_CHANNEL_LIST

#define  DOT11_AVAILABLE_CHANNEL_LIST_REVISION_1 1

    typedef
    struct _DOT11_AVAILABLE_CHANNEL_LIST
    {
        NDIS_OBJECT_HEADER      Header;
        ULONG                   uNumOfEntries;
        ULONG                   uTotalNumOfEntries;
        ULONG                   uChannelNumber[1];
    }
    DOT11_AVAILABLE_CHANNEL_LIST, *PDOT11_AVAILABLE_CHANNEL_LIST;




#define OID_DOT11_AVAILABLE_FREQUENCY_LIST          NWF_DEFINE_OID(0x04, NWF_EXTAP_OID, NWF_MANDATORY_OID)
    // DOT11_AVAILABLE_FREQUENCY_LIST


#define DOT11_AVAILABLE_FREQUENCY_LIST_REVISION_1 1

    typedef
    struct _DOT11_AVAILABLE_FREQUENCY_LIST
    {
        NDIS_OBJECT_HEADER      Header;
        ULONG                   uNumOfEntries;
        ULONG                   uTotalNumOfEntries;
        ULONG                   uFrequencyValue[1];
    }
    DOT11_AVAILABLE_FREQUENCY_LIST, *PDOT11_AVAILABLE_FREQUENCY_LIST;




#define OID_DOT11_DISASSOCIATE_PEER_REQUEST         NWF_DEFINE_OID(0x05, NWF_EXTAP_OID, NWF_MANDATORY_OID)
    // DOT11_DISASSOCIATE_PEER_REQUEST

#define DOT11_DISASSOCIATE_PEER_REQUEST_REVISION_1 1

    typedef
    struct _DOT11_DISASSOCIATE_PEER_REQUEST
    {
        NDIS_OBJECT_HEADER      Header;
        DOT11_MAC_ADDRESS       PeerMacAddr;
        USHORT                  usReason;

    }
    DOT11_DISASSOCIATE_PEER_REQUEST, *PDOT11_DISASSOCIATE_PEER_REQUEST;








#define OID_DOT11_INCOMING_ASSOCIATION_DECISION     NWF_DEFINE_OID(0x06, NWF_EXTAP_OID, NWF_MANDATORY_OID)
    // DOT11_INCOMING_ASSOC_DECISION

#define DOT11_INCOMING_ASSOC_DECISION_REVISION_1 1

    typedef
    struct _DOT11_INCOMING_ASSOC_DECISION
    {
        NDIS_OBJECT_HEADER      Header;
        DOT11_MAC_ADDRESS       PeerMacAddr;
        BOOLEAN                 bAccept;
        USHORT                  usReasonCode;
        ULONG                   uAssocResponseIEsOffset;
        ULONG                   uAssocResponseIEsLength;
    }
    DOT11_INCOMING_ASSOC_DECISION, *PDOT11_INCOMING_ASSOC_DECISION;

#define DOT11_SIZEOF_INCOMING_ASSOC_DECISION_REVISION_1 sizeof(DOT11_INCOMING_ASSOC_DECISION)


#ifdef NWF_WFD_SUPPORTED

#define DOT11_INCOMING_ASSOC_DECISION_REVISION_2 2

    typedef
    struct _DOT11_INCOMING_ASSOC_DECISION_V2
    {
        NDIS_OBJECT_HEADER      Header;
        DOT11_MAC_ADDRESS       PeerMacAddr;
        BOOLEAN                 bAccept;
        USHORT                  usReasonCode;
        ULONG                   uAssocResponseIEsOffset;
        ULONG                   uAssocResponseIEsLength;
        DOT11_WFD_STATUS_CODE   WFDStatus;
    }
    DOT11_INCOMING_ASSOC_DECISION_V2, *PDOT11_INCOMING_ASSOC_DECISION_V2;

#define DOT11_SIZEOF_INCOMING_ASSOC_DECISION_REVISION_2 sizeof(DOT11_INCOMING_ASSOC_DECISION_V2)

#endif




#define OID_DOT11_ADDITIONAL_IE                     NWF_DEFINE_OID(0x07, NWF_EXTAP_OID, NWF_MANDATORY_OID)
    // DOT11_ADDITIONAL_IE

#define DOT11_ADDITIONAL_IE_REVISION_1 1

    typedef
    struct _DOT11_ADDITIONAL_IE
    {
        NDIS_OBJECT_HEADER      Header;
        ULONG                   uBeaconIEsOffset;
        ULONG                   uBeaconIEsLength;
        ULONG                   uResponseIEsOffset;
        ULONG                   uResponseIEsLength;
    }
    DOT11_ADDITIONAL_IE, *PDOT11_ADDITIONAL_IE;



//
// Ext AP Send context
//
#define DOT11_EXTAP_SEND_CONTEXT_REVISION_1  1


typedef
struct DOT11_EXTSTA_SEND_CONTEXT
DOT11_EXTAP_SEND_CONTEXT, *PDOT11_EXTAP_SEND_CONTEXT;



//
// Ext AP Receive context
//
#define DOT11_EXTAP_RECV_CONTEXT_REVISION_1  1


typedef
struct DOT11_EXTSTA_RECV_CONTEXT
DOT11_EXTAP_RECV_CONTEXT, *PDOT11_EXTAP_RECV_CONTEXT;




#define OID_DOT11_ENUM_PEER_INFO                    NWF_DEFINE_OID(0x08, NWF_EXTAP_OID, NWF_MANDATORY_OID)
    // DOT11_PEER_INFO_LIST

    typedef
    struct _DOT11_PEER_STATISTICS
    {
        ULONGLONG ullDecryptSuccessCount;
        ULONGLONG ullDecryptFailureCount;
        ULONGLONG ullTxPacketSuccessCount;
        ULONGLONG ullTxPacketFailureCount;
        ULONGLONG ullRxPacketSuccessCount;
        ULONGLONG ullRxPacketFailureCount;
    }
    DOT11_PEER_STATISTICS, *PDOT11_PEER_STATISTICS;

    typedef
    struct _DOT11_PEER_INFO
    {
        DOT11_MAC_ADDRESS       MacAddress;
        USHORT                  usCapabilityInformation;
        DOT11_AUTH_ALGORITHM    AuthAlgo;
        DOT11_CIPHER_ALGORITHM  UnicastCipherAlgo;
        DOT11_CIPHER_ALGORITHM  MulticastCipherAlgo;
        BOOLEAN                 bWpsEnabled;
        USHORT                  usListenInterval;
        UCHAR                   ucSupportedRates[MAX_NUM_SUPPORTED_RATES_V2];
        USHORT                  usAssociationID;
        DOT11_ASSOCIATION_STATE AssociationState;
        DOT11_POWER_MODE        PowerMode;
        LARGE_INTEGER           liAssociationUpTime;
        DOT11_PEER_STATISTICS Statistics;
    }
    DOT11_PEER_INFO, *PDOT11_PEER_INFO;


#define DOT11_PEER_INFO_LIST_REVISION_1  1
    typedef
    struct _DOT11_PEER_INFO_LIST
    {
        NDIS_OBJECT_HEADER      Header;
        ULONG                   uNumOfEntries;
        ULONG                   uTotalNumOfEntries;
        DOT11_PEER_INFO         PeerInfo[1];
    }
    DOT11_PEER_INFO_LIST, *PDOT11_PEER_INFO_LIST;





#endif // NWF_EXTAP_SUPPORTED

#ifdef NWF_VWIFI_SUPPORTED

#define DOT11_VWIFI_COMBINATION_REVISION_1     1
    typedef
    struct _DOT11_VWIFI_COMBINATION
    {
        NDIS_OBJECT_HEADER Header;
        ULONG uNumInfrastructure;
        ULONG uNumAdhoc;
        ULONG uNumSoftAP;
    }
    DOT11_VWIFI_COMBINATION, * PDOT11_VWIFI_COMBINATION;

#define DOT11_SIZEOF_VWIFI_COMBINATION_REVISION_1   sizeof(DOT11_VWIFI_COMBINATION)

#define DOT11_VWIFI_COMBINATION_REVISION_2     2
    typedef
    struct _DOT11_VWIFI_COMBINATION_V2
    {
        NDIS_OBJECT_HEADER Header;
        ULONG uNumInfrastructure;
        ULONG uNumAdhoc;
        ULONG uNumSoftAP;
        ULONG uNumVirtualStation;
    }
    DOT11_VWIFI_COMBINATION_V2, * PDOT11_VWIFI_COMBINATION_V2;

#define DOT11_SIZEOF_VWIFI_COMBINATION_REVISION_2   sizeof(DOT11_VWIFI_COMBINATION_V2)

#ifdef NWF_WFD_SUPPORTED

#define DOT11_VWIFI_COMBINATION_REVISION_3     3
    typedef
    struct _DOT11_VWIFI_COMBINATION_V3
    {
        NDIS_OBJECT_HEADER Header;
        ULONG uNumInfrastructure;
        ULONG uNumAdhoc;
        ULONG uNumSoftAP;
        ULONG uNumVirtualStation;
        ULONG uNumWFDGroup;
    }
    DOT11_VWIFI_COMBINATION_V3, * PDOT11_VWIFI_COMBINATION_V3;

#define DOT11_SIZEOF_VWIFI_COMBINATION_REVISION_3   sizeof(DOT11_VWIFI_COMBINATION_V3)

#endif  // NWF_WFD_SUPPORTED

#define DOT11_VWIFI_ATTRIBUTES_REVISION_1      1
    typedef
    struct DOT11_VWIFI_ATTRIBUTES
    {
        NDIS_OBJECT_HEADER Header;
        ULONG uTotalNumOfEntries;
        #ifdef __midl
        [unique, size_is(uTotalNumOfEntries)] DOT11_VWIFI_COMBINATION Combinations[*];
        #else
        DOT11_VWIFI_COMBINATION Combinations[1];
        #endif
    }
    DOT11_VWIFI_ATTRIBUTES, * PDOT11_VWIFI_ATTRIBUTES;

// TAG for Virtual WiFi specific OIDs.
#define NWF_VWIFI_OID      (0x04U)


#define OID_DOT11_CREATE_MAC                        NWF_DEFINE_OID(0x01, NWF_VWIFI_OID, NWF_MANDATORY_OID)

#define DOT11_MAC_PARAMETERS_REVISION_1         1

        typedef struct _DOT11_MAC_PARAMETERS{
            NDIS_OBJECT_HEADER Header;
            ULONG uOpmodeMask;
        } DOT11_MAC_PARAMETERS, *PDOT11_MAC_PARAMETERS;

#define DOT11_SIZEOF_MAC_PARAMETERS_REVISION_1      sizeof(DOT11_MAC_PARAMETERS)

        typedef struct DOT11_MAC_INFO
        {
            ULONG uReserved;
            ULONG uNdisPortNumber;
            DOT11_MAC_ADDRESS MacAddr;
        } DOT11_MAC_INFO, * PDOT11_MAC_INFO;

#define OID_DOT11_DELETE_MAC                        NWF_DEFINE_OID(0x02, NWF_VWIFI_OID, NWF_MANDATORY_OID)

#define OID_DOT11_PREFERRED_MAC                     NWF_DEFINE_OID(0x03, NWF_VWIFI_OID, NWF_MANDATORY_OID)

#define OID_DOT11_VIRTUAL_STATION_CAPABILITY        NWF_DEFINE_OID(0x04, NWF_VWIFI_OID, NWF_OPTIONAL_OID)

#endif // NWF_VWIFI_SUPPORTED


#ifdef NWF_WFD_SUPPORTED

#define DOT11_WFD_ATTRIBUTES_REVISION_1         1

    typedef
    struct _DOT11_WFD_ATTRIBUTES
    {
        NDIS_OBJECT_HEADER Header;
        ULONG uNumConcurrentGORole;
        ULONG uNumConcurrentClientRole;
        ULONG WPSVersionsSupported;

        // Device Capabilities
        BOOLEAN bServiceDiscoverySupported;
        BOOLEAN bClientDiscoverabilitySupported;
        BOOLEAN bInfrastructureManagementSupported;

        ULONG uMaxSecondaryDeviceTypeListSize;

        DOT11_MAC_ADDRESS DeviceAddress;
        ULONG uInterfaceAddressListCount;
        PDOT11_MAC_ADDRESS pInterfaceAddressList;

        ULONG uNumSupportedCountryOrRegionStrings;
        #ifdef __midl
        [size_is(uNumSupportedCountryOrRegionStrings)]
        #endif
        PDOT11_COUNTRY_OR_REGION_STRING pSupportedCountryOrRegionStrings;

        // Device Functionality
        ULONG uDiscoveryFilterListSize;

        // GO Capabilities
        ULONG uGORoleClientTableSize;

        // Client Capabilities
    }DOT11_WFD_ATTRIBUTES, *PDOT11_WFD_ATTRIBUTES;

#define DOT11_SIZEOF_WFD_ATTRIBUTES_REVISION_1   sizeof(DOT11_WFD_ATTRIBUTES)


// Common WFD data types
typedef UCHAR DOT11_WFD_GROUP_CAPABILITY;

// WFD Status Code
#define DOT11_WFD_STATUS_SUCCESS                                                0
#define DOT11_WFD_STATUS_FAILED_INFORMATION_IS_UNAVAILABLE                      1
#define DOT11_WFD_STATUS_FAILED_INCOMPATIBLE_PARAMETERS                         2
#define DOT11_WFD_STATUS_FAILED_LIMIT_REACHED                                   3
#define DOT11_WFD_STATUS_FAILED_INVALID_PARAMETERS                              4
#define DOT11_WFD_STATUS_FAILED_UNABLE_TO_ACCOMODATE_REQUEST                    5
#define DOT11_WFD_STATUS_FAILED_PREVIOUS_PROTOCOL_ERROR                         6
#define DOT11_WFD_STATUS_FAILED_NO_COMMON_CHANNELS                              7
#define DOT11_WFD_STATUS_FAILED_UNKNOWN_WFD_GROUP                               8
#define DOT11_WFD_STATUS_FAILED_MATCHING_MAX_INTENT                             9
#define DOT11_WFD_STATUS_FAILED_INCOMPATIBLE_PROVISIONING_METHOD                10
#define DOT11_WFD_STATUS_FAILED_REJECTED_BY_USER                                11
#define DOT11_WFD_STATUS_SUCCESS_ACCEPTED_BY_USER                               12

#define WFD_STATUS_SUCCEEDED(status)      (((DOT11_WFD_STATUS_CODE)(status)) == DOT11_WFD_STATUS_SUCCESS || \
                                           ((DOT11_WFD_STATUS_CODE)(status)) == DOT11_WFD_STATUS_SUCCESS_ACCEPTED_BY_USER)
#define WFD_STATUS_FAILED(status)         (((DOT11_WFD_STATUS_CODE)(status)) != DOT11_WFD_STATUS_SUCCESS && \
                                           ((DOT11_WFD_STATUS_CODE)(status)) != DOT11_WFD_STATUS_SUCCESS_ACCEPTED_BY_USER)

// WFD Minor Reason Code
#define DOT11_WFD_MINOR_REASON_SUCCESS                                          0
#define DOT11_WFD_MINOR_REASON_DISASSOCIATED_FROM_WLAN_CROSS_CONNECTION_POLICY  1
#define DOT11_WFD_MINOR_REASON_DISASSOCIATED_NOT_MANAGED_INFRASTRUCTURE_CAPABLE 2
#define DOT11_WFD_MINOR_REASON_DISASSOCIATED_WFD_COEXISTENCE_POLICY             3
#define DOT11_WFD_MINOR_REASON_DISASSOCIATED_INFRASTRUCTURE_MANAGED_POLICY      4

// WPS Version Information
#define DOT11_WPS_VERSION_1_0 0x01
#define DOT11_WPS_VERSION_2_0 0x02

// WFD Device Capability Bits
#define DOT11_WFD_DEVICE_CAPABILITY_SERVICE_DISCOVERY                           0x01
#define DOT11_WFD_DEVICE_CAPABILITY_P2P_CLIENT_DISCOVERABILITY                  0x02
#define DOT11_WFD_DEVICE_CAPABILITY_CONCURRENT_OPERATION                        0x04
#define DOT11_WFD_DEVICE_CAPABILITY_P2P_INFRASTRUCTURE_MANAGED                  0x08
#define DOT11_WFD_DEVICE_CAPABILITY_P2P_DEVICE_LIMIT                            0x10
#define DOT11_WFD_DEVICE_CAPABILITY_P2P_INVITATION_PROCEDURE                    0x20
#define DOT11_WFD_DEVICE_CAPABILITY_RESERVED_6                                  0x40
#define DOT11_WFD_DEVICE_CAPABILITY_RESERVED_7                                  0x80

// WFD Group Capability Bits
#define DOT11_WFD_GROUP_CAPABILITY_NONE                                         0x00
#define DOT11_WFD_GROUP_CAPABILITY_GROUP_OWNER                                  0x01
#define DOT11_WFD_GROUP_CAPABILITY_PERSISTENT_GROUP                             0x02
#define DOT11_WFD_GROUP_CAPABILITY_GROUP_LIMIT_REACHED                          0x04
#define DOT11_WFD_GROUP_CAPABILITY_INTRABSS_DISTRIBUTION_SUPPORTED              0x08
#define DOT11_WFD_GROUP_CAPABILITY_CROSS_CONNECTION_SUPPORTED                   0x10
#define DOT11_WFD_GROUP_CAPABILITY_PERSISTENT_RECONNECT_SUPPORTED               0x20
#define DOT11_WFD_GROUP_CAPABILITY_IN_GROUP_FORMATION                           0x40
#define DOT11_WFD_GROUP_CAPABILITY_RESERVED_7                                   0x80
#define DOT11_WFD_GROUP_CAPABILITY_EAPOL_KEY_IP_ADDRESS_ALLOCATION_SUPPORTED    0x80

#define DOT11_WPS_DEVICE_NAME_MAX_LENGTH   32      // 32 bytes
#define DOT11_WPS_MAX_PASSKEY_LENGTH       8       // 8 bytes
#define DOT11_WPS_MAX_MODEL_NAME_LENGTH    32      // 32 bytes
#define DOT11_WPS_MAX_MODEL_NUMBER_LENGTH  32      // 32 bytes

// Device Type
typedef struct _DOT11_WFD_DEVICE_TYPE
{
    USHORT  CategoryID;
    USHORT  SubCategoryID;
    UCHAR   OUI[4];
} DOT11_WFD_DEVICE_TYPE, * PDOT11_WFD_DEVICE_TYPE;


// WPS Device Name
typedef struct _DOT11_WPS_DEVICE_NAME {
#ifndef __midl
    _Field_range_(0,32)
#endif
    ULONG uDeviceNameLength;
    UCHAR ucDeviceName[DOT11_WPS_DEVICE_NAME_MAX_LENGTH];
} DOT11_WPS_DEVICE_NAME, * PDOT11_WPS_DEVICE_NAME;


// WFD Configuration Timeout
typedef
struct _DOT11_WFD_CONFIGURATION_TIMEOUT
{
    UCHAR GOTimeout;
    UCHAR ClientTimeout;
} DOT11_WFD_CONFIGURATION_TIMEOUT, * PDOT11_WFD_CONFIGURATION_TIMEOUT;


// WFD Group ID
typedef
struct _DOT11_WFD_GROUP_ID
{
    DOT11_MAC_ADDRESS DeviceAddress;
    DOT11_SSID SSID;
} DOT11_WFD_GROUP_ID, * PDOT11_WFD_GROUP_ID;


// WFD Group Owner Intent
typedef
struct _DOT11_WFD_GO_INTENT
{
    UCHAR TieBreaker:1;
    UCHAR Intent:7;
} DOT11_WFD_GO_INTENT, * PDOT11_WFD_GO_INTENT;


// WFD Channel
typedef
struct _DOT11_WFD_CHANNEL
{
    DOT11_COUNTRY_OR_REGION_STRING CountryRegionString;
    UCHAR OperatingClass;
    UCHAR ChannelNumber;
} DOT11_WFD_CHANNEL, * PDOT11_WFD_CHANNEL;


// WPS Config Method
typedef enum _DOT11_WPS_CONFIG_METHOD
{
    DOT11_WPS_CONFIG_METHOD_NULL = 0,
    DOT11_WPS_CONFIG_METHOD_DISPLAY = 0x0008,
    DOT11_WPS_CONFIG_METHOD_NFC_TAG = 0x0020,
    DOT11_WPS_CONFIG_METHOD_NFC_INTERFACE = 0x0040,
    DOT11_WPS_CONFIG_METHOD_PUSHBUTTON = 0x0080,
    DOT11_WPS_CONFIG_METHOD_KEYPAD = 0x0100,
    DOT11_WPS_CONFIG_METHOD_WFDS_DEFAULT = 0x1000
} DOT11_WPS_CONFIG_METHOD, *PDOT11_WPS_CONFIG_METHOD;

// WPS Device Password ID
typedef enum _DOT11_WPS_DEVICE_PASSWORD_ID
{
    DOT11_WPS_PASSWORD_ID_DEFAULT = 0x0000,
    DOT11_WPS_PASSWORD_ID_USER_SPECIFIED = 0x0001,
    DOT11_WPS_PASSWORD_ID_MACHINE_SPECIFIED = 0x0002,
    DOT11_WPS_PASSWORD_ID_REKEY = 0x0003,
    DOT11_WPS_PASSWORD_ID_PUSHBUTTON = 0x0004,
    DOT11_WPS_PASSWORD_ID_REGISTRAR_SPECIFIED = 0x0005,
        // 0x0006	Reserved(for IBSS with Wi-Fi Protected Setup Specification)
    DOT11_WPS_PASSWORD_ID_NFC_CONNECTION_HANDOVER = 0x0007,
    DOT11_WPS_PASSWORD_ID_WFD_SERVICES = 0x0008,
        // 0x0009 - 0x000F = Reserved
    DOT11_WPS_PASSWORD_ID_OOB_RANGE_MIN = 0x0010,
    DOT11_WPS_PASSWORD_ID_OOB_RANGE_MAX = 0xFFFF,

} DOT11_WPS_DEVICE_PASSWORD_ID, *PDOT11_WPS_DEVICE_PASSWORD_ID;

// WFD Services

typedef struct _WFDSVC_CONNECTION_CAPABILITY
{
    BOOLEAN bNew;
    BOOLEAN bClient;
    BOOLEAN bGO;
} WFDSVC_CONNECTION_CAPABILITY, *PWFDSVC_CONNECTION_CAPABILITY;

#define WFDSVC_CONNECTION_CAPABILITY_NEW 0x01
#define WFDSVC_CONNECTION_CAPABILITY_CLIENT 0x02
#define WFDSVC_CONNECTION_CAPABILITY_GO 0x04

// WFD Service Hash List
typedef struct _DOT11_WFD_SERVICE_HASH_LIST
{
    USHORT ServiceHashCount;
#ifdef __midl
    [size_is(ServiceHashCount)] DOT11_WFD_SERVICE_HASH ServiceHash[*];
#else
    DOT11_WFD_SERVICE_HASH ServiceHash[1];
#endif
} DOT11_WFD_SERVICE_HASH_LIST, *PDOT11_WFD_SERVICE_HASH_LIST;

// WFD Service Advertisement ID
typedef struct _DOT11_WFD_ADVERTISEMENT_ID {
    ULONG AdvertisementID;
    DOT11_MAC_ADDRESS ServiceAddress;
} DOT11_WFD_ADVERTISEMENT_ID, *PDOT11_WFD_ADVERTISEMENT_ID;

// WFD Service Session ID
typedef struct _DOT11_WFD_SESSION_ID {
    ULONG SessionID;
    DOT11_MAC_ADDRESS SessionAddress;
} DOT11_WFD_SESSION_ID, *PDOT11_WFD_SESSION_ID;

// WFD Services Advertised Service Info
typedef struct _DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    ULONG AdvertisementID;
    USHORT ConfigMethods;
#ifndef __midl
    _Field_range_(0, DOT11_WFD_SERVICE_NAME_MAX_LENGTH)
#endif
    UCHAR ServiceNameLength;
    UCHAR ServiceName[DOT11_WFD_SERVICE_NAME_MAX_LENGTH];
} DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR, *PDOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR;

typedef struct _DOT11_WFD_ADVERTISED_SERVICE_LIST
{
    USHORT ServiceCount;
#ifdef __midl
    [size_is(ServiceCount)] DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR AdvertisedService[*];
#else
    DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR AdvertisedService[1];
#endif
} DOT11_WFD_ADVERTISED_SERVICE_LIST, *PDOT11_WFD_ADVERTISED_SERVICE_LIST;

// NDIS indications for WFD mode.

// Data Type for NDIS_STATUS_DOT11_WFD_DISCOVER_COMPLETE
#define DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS_REVISION_1    1

#define DOT11_WFD_DISCOVER_COMPLETE_MAX_LIST_SIZE   128

    typedef
    struct _DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        NDIS_STATUS Status;
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        ULONG uListOffset;
        ULONG uListLength;
    } DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS, * PDOT11_WFD_DISCOVER_COMPLETE_PARAMETERS;

#define DOT11_SIZEOF_WFD_DISCOVER_COMPLETE_PARAMETERS_REVISION_1 sizeof(DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS)


// Data Type for NDIS_STATUS_DOT11_WFD_GO_NEGOTIATION_REQUEST_SEND_COMPLETE
#define DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1    1
    typedef
    struct _DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS PeerDeviceAddress;
        DOT11_DIALOG_TOKEN DialogToken;
        NDIS_STATUS Status;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS, * PDOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS;

#define DOT11_SIZEOF_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1 sizeof(DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS)


// Data Type for NDIS_STATUS_DOT11_WFD_RECEIVED_GO_NEGOTIATION_REQUEST
#define DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1    1
    typedef
    struct _DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS PeerDeviceAddress;
        DOT11_DIALOG_TOKEN DialogToken;
        PVOID RequestContext;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS, * PDOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS;

#define DOT11_SIZEOF_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1 sizeof(DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS)


// Data Type for NDIS_STATUS_DOT11_WFD_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE
#define DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1    1
    typedef
    struct _DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS PeerDeviceAddress;
        DOT11_DIALOG_TOKEN DialogToken;
        NDIS_STATUS Status;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS, * PDOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS;

#define DOT11_SIZEOF_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1 sizeof(DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS)


// Data Type for NDIS_STATUS_DOT11_WFD_RECEIVED_GO_NEGOTIATION_RESPONSE
#define DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1    1
    typedef
    struct _DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS PeerDeviceAddress;
        DOT11_DIALOG_TOKEN DialogToken;
        PVOID ResponseContext;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS, * PDOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS;

#define DOT11_SIZEOF_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1 sizeof(DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS)


// Data Type for NDIS_STATUS_DOT11_WFD_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE
#define DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS_REVISION_1    1
    typedef
    struct _DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS PeerDeviceAddress;
        DOT11_DIALOG_TOKEN DialogToken;
        NDIS_STATUS Status;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS, * PDOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS;

#define DOT11_SIZEOF_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS_REVISION_1 sizeof(DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS)


// Data Type for NDIS_STATUS_DOT11_WFD_RECEIVED_GO_NEGOTIATION_CONFIRMATION
#define DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1    1
    typedef
    struct _DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS PeerDeviceAddress;
        DOT11_DIALOG_TOKEN DialogToken;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS, * PDOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS;

#define DOT11_SIZEOF_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1 sizeof(DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS)


// Data Type for NDIS_STATUS_DOT11_WFD_INVITATION_REQUEST_SEND_COMPLETE
#define DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1    1
    typedef
    struct _DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS PeerDeviceAddress;
        DOT11_MAC_ADDRESS ReceiverAddress;
        DOT11_DIALOG_TOKEN DialogToken;
        NDIS_STATUS Status;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS, * PDOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS;

#define DOT11_SIZEOF_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1 sizeof(DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS)


// Data Type for NDIS_STATUS_DOT11_WFD_RECEIVED_INVITATION_REQUEST
#define DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS_REVISION_1    1
    typedef
    struct _DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS TransmitterDeviceAddress;
        DOT11_MAC_ADDRESS BSSID;
        DOT11_DIALOG_TOKEN DialogToken;
        PVOID RequestContext;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS, * PDOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS;

#define DOT11_SIZEOF_RECEIVED_INVITATION_REQUEST_PARAMETERS_REVISION_1 sizeof(DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS)


// Data Type for NDIS_STATUS_DOT11_WFD_INVITATION_RESPONSE_SEND_COMPLETE
#define DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1    1
    typedef
    struct _DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS ReceiverDeviceAddress;
        DOT11_DIALOG_TOKEN DialogToken;
        NDIS_STATUS Status;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS, * PDOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS;

#define DOT11_SIZEOF_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1 sizeof(DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS)


// Data Type for NDIS_STATUS_DOT11_WFD_RECEIVED_INVITATION_RESPONSE
#define DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS_REVISION_1    1
    typedef
    struct _DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS TransmitterDeviceAddress;
        DOT11_MAC_ADDRESS BSSID;
        DOT11_DIALOG_TOKEN DialogToken;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS, * PDOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS;

#define DOT11_SIZEOF_RECEIVED_INVITATION_RESPONSE_PARAMETERS_REVISION_1 sizeof(DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS)


// Data Type for NDIS_STATUS_DOT11_WFD_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE
#define DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1    1
    typedef
    struct _DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS PeerDeviceAddress;
        DOT11_MAC_ADDRESS ReceiverAddress;
        DOT11_DIALOG_TOKEN DialogToken;
        NDIS_STATUS Status;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS, * PDOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS;

#define DOT11_SIZEOF_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1 sizeof(DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS)


// Data Type for NDIS_STATUS_DOT11_WFD_RECEIVED_PROVISION_DISCOVERY_REQUEST
#define DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1    1
    typedef
    struct _DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS TransmitterDeviceAddress;
        DOT11_MAC_ADDRESS BSSID;
        DOT11_DIALOG_TOKEN DialogToken;
        PVOID RequestContext;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS, * PDOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS;

#define DOT11_SIZEOF_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1 sizeof(DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS)


// Data Type for NDIS_STATUS_DOT11_WFD_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE
#define DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1    1
    typedef
    struct _DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS ReceiverDeviceAddress;
        DOT11_DIALOG_TOKEN DialogToken;
        NDIS_STATUS Status;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS, * PDOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS;

#define DOT11_SIZEOF_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1 sizeof(DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS)


// Data Type for NDIS_STATUS_DOT11_WFD_RECEIVED_PROVISION_DISCOVERY_RESPONSE
#define DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1    1
    typedef
    struct _DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS TransmitterDeviceAddress;
        DOT11_MAC_ADDRESS BSSID;
        DOT11_DIALOG_TOKEN DialogToken;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS, * PDOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS;

#define DOT11_SIZEOF_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1 sizeof(DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS)


// Data Type for NDIS_STATUS_DOT11_ANQP_QUERY_COMPLETE
#define DOT11_ANQP_QUERY_COMPLETE_PARAMETERS_REVISION_1    1
#define DOT11_SIZEOF_ANQP_QUERY_COMPLETE_PARAMETERS_REVISION_1 sizeof(DOT11_ANQP_QUERY_COMPLETE_PARAMETERS)

    //
    // Following must match WIFI_ANQP_QUERY_STATUS
    //
    typedef enum _DOT11_ANQP_QUERY_RESULT
    {
        dot11_ANQP_query_result_success = 0,
        dot11_ANQP_query_result_failure = 1,
        dot11_ANQP_query_result_timed_out = 2,
        dot11_ANQP_query_result_resources = 3,
        dot11_ANQP_query_result_advertisement_protocol_not_supported_on_remote = 4,
        dot11_ANQP_query_result_gas_protocol_failure = 5,
        dot11_ANQP_query_result_advertisement_server_not_responding = 6,
        dot11_ANQP_query_result_access_issues = 7
    } DOT11_ANQP_QUERY_RESULT, *PDOT11_ANQP_QUERY_RESULT;

    typedef
    struct _DOT11_ANQP_QUERY_COMPLETE_PARAMETERS
    {
        NDIS_OBJECT_HEADER      Header;
        DOT11_ANQP_QUERY_RESULT Status;
        HANDLE                  hContext;
        ULONG                   uResponseLength;
    } DOT11_ANQP_QUERY_COMPLETE_PARAMETERS, * PDOT11_ANQP_QUERY_COMPLETE_PARAMETERS;



// TAG for WFD device specific OIDs.
#define NWF_WFD_DEVICE_OID  (0x05U)

// TAG for WFD role (GO/Client) specific OIDs.
#define NWF_WFD_ROLE_OID    (0x06U)


// OID_DOT11_WFD_DEVICE_CAPABILITY
#define OID_DOT11_WFD_DEVICE_CAPABILITY             NWF_DEFINE_OID(0x01, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)

#define DOT11_WFD_DEVICE_CAPABILITY_CONFIG_REVISION_1   1
    typedef
    struct _DOT11_WFD_DEVICE_CAPABILITY_CONFIG
    {
    	NDIS_OBJECT_HEADER Header;
    	BOOLEAN bServiceDiscoveryEnabled;
    	BOOLEAN bClientDiscoverabilityEnabled;
    	BOOLEAN bConcurrentOperationSupported;
    	BOOLEAN bInfrastructureManagementEnabled;
    	BOOLEAN bDeviceLimitReached;
    	BOOLEAN bInvitationProcedureEnabled;
    	ULONG WPSVersionsEnabled;
    } DOT11_WFD_DEVICE_CAPABILITY_CONFIG, * PDOT11_WFD_DEVICE_CAPABILITY_CONFIG;

#define DOT11_SIZEOF_WFD_DEVICE_CAPABILITY_CONFIG_1 sizeof(DOT11_WFD_DEVICE_CAPABILITY_CONFIG)


// OID_DOT11_WFD_GROUP_OWNER_CAPABILITY
#define OID_DOT11_WFD_GROUP_OWNER_CAPABILITY              NWF_DEFINE_OID(0x02, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)

#define DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_REVISION_1    1
    typedef
    struct _DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG
    {
    	NDIS_OBJECT_HEADER Header;
        BOOLEAN bPersistentGroupEnabled;
    	BOOLEAN bIntraBSSDistributionSupported;
    	BOOLEAN bCrossConnectionSupported;
    	BOOLEAN bPersistentReconnectSupported;
    	BOOLEAN bGroupFormationEnabled;
    	ULONG uMaximumGroupLimit;
    } DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG, *PDOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG;

#define DOT11_SIZEOF_WFD_GROUP_CAPABILITY_CONFIG_1  sizeof(DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG)

#define DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_REVISION_2    2

    typedef
    struct _DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2
    {
        NDIS_OBJECT_HEADER Header;
        BOOLEAN bPersistentGroupEnabled;
        BOOLEAN bIntraBSSDistributionSupported;
        BOOLEAN bCrossConnectionSupported;
        BOOLEAN bPersistentReconnectSupported;
        BOOLEAN bGroupFormationEnabled;
        ULONG uMaximumGroupLimit;
        BOOLEAN bEapolKeyIpAddressAllocationSupported;
    } DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2, *PDOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2;

#define DOT11_SIZEOF_WFD_GROUP_CAPABILITY_CONFIG_2  sizeof(DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2)

// OID_DOT11_WFD_DEVICE_INFO
#define OID_DOT11_WFD_DEVICE_INFO                   NWF_DEFINE_OID(0x03, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)

#define DOT11_WFD_DEVICE_INFO_REVISION_1            1
    typedef
    struct _DOT11_WFD_DEVICE_INFO
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS DeviceAddress;
        USHORT ConfigMethods;
        DOT11_WFD_DEVICE_TYPE PrimaryDeviceType;
        DOT11_WPS_DEVICE_NAME DeviceName;
    } DOT11_WFD_DEVICE_INFO, *PDOT11_WFD_DEVICE_INFO;

#define DOT11_SIZEOF_WFD_DEVICE_INFO_REVISION_1     sizeof(DOT11_WFD_DEVICE_INFO)


// OID_DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST
#define OID_DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST    NWF_DEFINE_OID(0x04, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)

#define DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST_REVISION_1     1
    typedef
    struct _DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
        NDIS_OBJECT_HEADER Header;
        ULONG uNumOfEntries;
        ULONG uTotalNumOfEntries;
        DOT11_WFD_DEVICE_TYPE SecondaryDeviceTypes[1];
    } DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST, * PDOT11_WFD_SECONDARY_DEVICE_TYPE_LIST;

#define DOT11_SIZEOF_WFD_SECONDARY_DEVICE_TYPE_LIST_REVISION_1  FIELD_OFFSET(DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST, SecondaryDeviceTypes)


// OID_DOT11_WFD_DISCOVER_REQUEST
#define OID_DOT11_WFD_DISCOVER_REQUEST          NWF_DEFINE_OID(0x05, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)

    typedef enum _DOT11_WFD_DISCOVER_TYPE
    {
        dot11_wfd_discover_type_scan_only = 1,
        dot11_wfd_discover_type_find_only = 2,
        dot11_wfd_discover_type_auto = 3,
        dot11_wfd_discover_type_scan_social_channels = 4,
        dot11_wfd_discover_type_forced = 0x80000000

    } DOT11_WFD_DISCOVER_TYPE, *PDOT11_WFD_DISCOVER_TYPE;

    typedef enum _DOT11_WFD_SCAN_TYPE
    {
        dot11_wfd_scan_type_active = 1,
        dot11_wfd_scan_type_passive = 2,
        dot11_wfd_scan_type_auto = 3

    } DOT11_WFD_SCAN_TYPE, *PDOT11_WFD_SCAN_TYPE;

#define DOT11_WFD_DISCOVER_DEVICE_FILTER_REVISION_1
#define DOT11_SIZEOF_WFD_DISCOVER_DEVICE_FILTER_REVISION_1

#define DISCOVERY_FILTER_BITMASK_DEVICE 0x1
#define DISCOVERY_FILTER_BITMASK_GO     0x2
#define DISCOVERY_FILTER_BITMASK_ANY    0xF

    typedef
    struct _DOT11_WFD_DISCOVER_DEVICE_FILTER
    {
        DOT11_MAC_ADDRESS DeviceID;
        UCHAR ucBitmask;
        DOT11_SSID GroupSSID;
    } DOT11_WFD_DISCOVER_DEVICE_FILTER, *PDOT11_WFD_DISCOVER_DEVICE_FILTER;

#define DOT11_WFD_DISCOVER_REQUEST_REVISION_1   1

    typedef
    struct _DOT11_WFD_DISCOVER_REQUEST
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_WFD_DISCOVER_TYPE DiscoverType;
        DOT11_WFD_SCAN_TYPE ScanType;
        ULONG uDiscoverTimeout;
        ULONG uDeviceFilterListOffset;
        ULONG uNumDeviceFilters;
        ULONG uIEsOffset;
        ULONG uIEsLength;
        BOOLEAN bForceScanLegacyNetworks;
    } DOT11_WFD_DISCOVER_REQUEST, *PDOT11_WFD_DISCOVER_REQUEST;

#define DOT11_SIZEOF_WFD_DISCOVER_REQUEST_REVISION_1    sizeof(DOT11_WFD_DISCOVER_REQUEST)

// OID_DOT11_WFD_ENUM_DEVICE_LIST
#define OID_DOT11_WFD_ENUM_DEVICE_LIST             NWF_DEFINE_OID(0x06, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)


//See comment about DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO above
#ifndef __midl

#define DOT11_DEVICE_ENTRY_BYTE_ARRAY_REVISION_1    1
    typedef
    struct _DOT11_WFD_DEVICE_ENTRY
    {
        ULONG uPhyId;
        DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO PhySpecificInfo;
        DOT11_MAC_ADDRESS dot11BSSID;
        DOT11_BSS_TYPE dot11BSSType;
        DOT11_MAC_ADDRESS TransmitterAddress;
        LONG lRSSI;
        ULONG uLinkQuality;
        USHORT usBeaconPeriod;
        ULONGLONG ullTimestamp;
        ULONGLONG ullBeaconHostTimestamp;
        ULONGLONG ullProbeResponseHostTimestamp;
        USHORT usCapabilityInformation;
        ULONG uBeaconIEsOffset;
        ULONG uBeaconIEsLength;	// Can be 0
        ULONG uProbeResponseIEsOffset;
        ULONG uProbeResponseIEsLength;	// Can be 0
    } DOT11_WFD_DEVICE_ENTRY, *PDOT11_WFD_DEVICE_ENTRY;

#endif

#define DOT11_WFD_DEVICE_ENTRY_GET_DEVICE_SIZE(_device_) \
        (sizeof(DOT11_WFD_DEVICE_ENTRY) + (_device_)->uBeaconIEsLength + (_device_)-> uProbeResponseIEsLength)

// OID_DOT11_WFD_LISTEN_STATE_DISCOVERABILITY
#define OID_DOT11_WFD_LISTEN_STATE_DISCOVERABILITY  NWF_DEFINE_OID(0x07, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)
    // ULONG
    #define DOT11_WFD_DEVICE_NOT_DISCOVERABLE   0
    #define DOT11_WFD_DEVICE_AUTO_AVAILABILITY  16
    #define DOT11_WFD_DEVICE_HIGH_AVAILABILITY  24


// OID_DOT11_WFD_ADDITIONAL_IE
#define OID_DOT11_WFD_ADDITIONAL_IE                 NWF_DEFINE_OID(0x08, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)

#define DOT11_WFD_ADDITIONAL_IE_REVISION_1          1
    typedef
    struct _DOT11_WFD_ADDITIONAL_IE
    {
        NDIS_OBJECT_HEADER Header;
        ULONG uBeaconIEsOffset;
        ULONG uBeaconIEsLength;
        ULONG uProbeResponseIEsOffset;
        ULONG uProbeResponseIEsLength;
        ULONG uDefaultRequestIEsOffset;
        ULONG uDefaultRequestIEsLength;

    } DOT11_WFD_ADDITIONAL_IE,
    *PDOT11_WFD_ADDITIONAL_IE;

#define DOT11_SIZEOF_WFD_ADDITIONAL_IE_REVISION_1   sizeof(DOT11_WFD_ADDITIONAL_IE)

#define DOT11_WFD_ADDITIONAL_IE_GET_HEADER_SIZE(_addie_) \
        (sizeof(DOT11_WFD_ADDITIONAL_IE))

#define DOT11_WFD_ADDITIONAL_IE_GET_HEADER_END(_addie_) \
        (((PCHAR)(_addie_))+sizeof(DOT11_WFD_ADDITIONAL_IE))

#define DOT11_WFD_ADDITIONAL_IE_GET_BEACON_IE(_addie_) \
        (((PCHAR)(_addie_))+(_addie_)->uBeaconIEsOffset)

#define DOT11_WFD_ADDITIONAL_IE_SET_BEACON_IE(_addie_, _beacon_) \
        (_addie_)->uBeaconIEsOffset = (DWORD)(((SIZE_T)(_beacon_))-((SIZE_T)(_addie_)))

#define DOT11_WFD_ADDITIONAL_IE_GET_BEACON_IE_SIZE(_addie_) \
        ((_addie_)->uBeaconIEsLength)

#define DOT11_WFD_ADDITIONAL_IE_SET_BEACON_IE_SIZE(_addie_, _size_) \
        (_addie_)->uBeaconIEsLength = (DWORD)(_size_)

#define DOT11_WFD_ADDITIONAL_IE_GET_BEACON_IE_END(_addie_) \
        (((PCHAR)(_addie_))+(_addie_)->uBeaconIEsOffset+DOT11_WFD_ADDITIONAL_IE_GET_BEACON_IE_SIZE(_addie_))

#define DOT11_WFD_ADDITIONAL_IE_GET_PROBE_RESPONSE_IE(_addie_) \
        (((PCHAR)(_addie_))+(_addie_)->uProbeResponseIEsOffset)

#define DOT11_WFD_ADDITIONAL_IE_SET_PROBE_RESPONSE_IE(_addie_, _response_) \
        (_addie_)->uProbeResponseIEsOffset = (DWORD)(((SIZE_T)(_response_))-((SIZE_T)(_addie_)))

#define DOT11_WFD_ADDITIONAL_IE_GET_PROBE_RESPONSE_IE_SIZE(_addie_) \
        ((_addie_)->uProbeResponseIEsLength)

#define DOT11_WFD_ADDITIONAL_IE_SET_PROBE_RESPONSE_IE_SIZE(_addie_, _size_) \
        (_addie_)->uProbeResponseIEsLength = (DWORD)(_size_)

#define DOT11_WFD_ADDITIONAL_IE_GET_PROBE_RESPONSE_IE_END(_addie_) \
        (((PCHAR)(_addie_))+(_addie_)->uProbeResponseIEsOffset+DOT11_WFD_ADDITIONAL_IE_GET_PROBE_RESPONSE_IE_SIZE(_addie_))

#define DOT11_WFD_ADDITIONAL_IE_GET_PROBE_REQUEST_IE(_addie_) \
        (((PCHAR)(_addie_))+(_addie_)->uDefaultRequestIEsOffset)

#define DOT11_WFD_ADDITIONAL_IE_SET_PROBE_REQUEST_IE(_addie_, _request_) \
        (_addie_)->uDefaultRequestIEsOffset = (DWORD)(((SIZE_T)(_request_))-((SIZE_T)(_addie_)))

#define DOT11_WFD_ADDITIONAL_IE_GET_PROBE_REQUEST_IE_SIZE(_addie_) \
        ((_addie_)->uDefaultRequestIEsLength)

#define DOT11_WFD_ADDITIONAL_IE_SET_PROBE_REQUEST_IE_SIZE(_addie_, _size_) \
        (_addie_)->uDefaultRequestIEsLength = (DWORD)(_size_)

#define DOT11_WFD_ADDITIONAL_IE_GET_ALL_IE_SIZE(_addie_) \
        ((_addie_)->uBeaconIEsLength + \
         (_addie_)->uProbeResponseIEsLength + \
         (_addie_)->uDefaultRequestIEsLength)

#define DOT11_WFD_ADDITIONAL_IE_GET_TOTAL_SIZE(_addie_) \
        (DOT11_WFD_ADDITIONAL_IE_GET_HEADER_SIZE(_addie_) + \
         DOT11_WFD_ADDITIONAL_IE_GET_ALL_IE_SIZE(_addie_))

// OID_DOT11_WFD_FLUSH_DEVICE_LIST
#define OID_DOT11_WFD_FLUSH_DEVICE_LIST             NWF_DEFINE_OID(0x09, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)
// No data


// OID_DOT11_WFD_SEND_GO_NEGOTIATION_REQUEST
#define OID_DOT11_WFD_SEND_GO_NEGOTIATION_REQUEST       NWF_DEFINE_OID(0x0A, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)

#define DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1     1
    typedef
    struct _DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS PeerDeviceAddress;
        DOT11_DIALOG_TOKEN DialogToken;
        ULONG uSendTimeout;
        DOT11_WFD_GO_INTENT GroupOwnerIntent;
        DOT11_WFD_CONFIGURATION_TIMEOUT MinimumConfigTimeout;
        DOT11_MAC_ADDRESS IntendedInterfaceAddress;
        DOT11_WFD_GROUP_CAPABILITY GroupCapability;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS, * PDOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS;

#define DOT11_SIZEOF_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1  sizeof(DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS)


// OID_DOT11_WFD_SEND_GO_NEGOTIATION_RESPONSE
#define OID_DOT11_WFD_SEND_GO_NEGOTIATION_RESPONSE  NWF_DEFINE_OID(0x0B, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)

#define DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1    1
    typedef
    struct _DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS PeerDeviceAddress;
        DOT11_DIALOG_TOKEN DialogToken;
        PVOID RequestContext;
        ULONG uSendTimeout;
        DOT11_WFD_STATUS_CODE Status;
        DOT11_WFD_GO_INTENT GroupOwnerIntent;
        DOT11_WFD_CONFIGURATION_TIMEOUT MinimumConfigTimeout;
        DOT11_MAC_ADDRESS IntendedInterfaceAddress;
        DOT11_WFD_GROUP_CAPABILITY GroupCapability;
        DOT11_WFD_GROUP_ID GroupID;
        BOOLEAN bUseGroupID;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS, * PDOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS;

#define DOT11_SIZEOF_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1 sizeof(DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS)


// OID_DOT11_WFD_SEND_GO_NEGOTIATION_CONFIRMATION
#define OID_DOT11_WFD_SEND_GO_NEGOTIATION_CONFIRMATION NWF_DEFINE_OID(0x0C, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)

#define DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1    1
    typedef struct _DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS PeerDeviceAddress;
        DOT11_DIALOG_TOKEN DialogToken;
        PVOID ResponseContext;
        ULONG uSendTimeout;
        DOT11_WFD_STATUS_CODE Status;
        DOT11_WFD_GROUP_CAPABILITY GroupCapability;
        DOT11_WFD_GROUP_ID GroupID;
        BOOLEAN bUseGroupID;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS, * PDOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS;
#define DOT11_SIZEOF_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1 sizeof(DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS)


// OID_DOT11_WFD_SEND_INVITATION_REQUEST
#define OID_DOT11_WFD_SEND_INVITATION_REQUEST           NWF_DEFINE_OID(0x0D, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)

    typedef
    struct _DOT11_WFD_INVITATION_FLAGS
    {
        UCHAR InvitationType:1; // 0 = Join active group, 1 = Reinvoke
        UCHAR Reserved:7;
    } DOT11_WFD_INVITATION_FLAGS, * PDOT11_WFD_INVITATION_FLAGS;

#define DOT11_SEND_INVITATION_REQUEST_PARAMETERS_REVISION_1     1
    typedef
    struct _DOT11_SEND_INVITATION_REQUEST_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_DIALOG_TOKEN DialogToken;
        DOT11_MAC_ADDRESS PeerDeviceAddress;
        ULONG uSendTimeout;
        DOT11_WFD_CONFIGURATION_TIMEOUT MinimumConfigTimeout;
        DOT11_WFD_INVITATION_FLAGS InvitationFlags;
        DOT11_MAC_ADDRESS GroupBSSID; 	// Optional
        BOOLEAN bUseGroupBSSID;
        DOT11_WFD_CHANNEL OperatingChannel;
        BOOLEAN bUseSpecifiedOperatingChannel;
        DOT11_WFD_GROUP_ID GroupID;
        BOOLEAN bLocalGO;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_SEND_INVITATION_REQUEST_PARAMETERS, * PDOT11_SEND_INVITATION_REQUEST_PARAMETERS;

#define DOT11_SIZEOF_SEND_INVITATION_REQUEST_PARAMETERS_REVISION_1  sizeof(DOT11_SEND_INVITATION_REQUEST_PARAMETERS)


// OID_DOT11_WFD_SEND_INVITATION_RESPONSE
#define OID_DOT11_WFD_SEND_INVITATION_RESPONSE  NWF_DEFINE_OID(0x0E, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)

#define DOT11_SEND_INVITATION_RESPONSE_PARAMETERS_REVISION_1    1
    typedef
    struct _DOT11_SEND_INVITATION_RESPONSE_PARAMETERS
    {
    	NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS ReceiverDeviceAddress;
        DOT11_DIALOG_TOKEN DialogToken;
        PVOID RequestContext;
        ULONG uSendTimeout;
        DOT11_WFD_STATUS_CODE Status;
        DOT11_WFD_CONFIGURATION_TIMEOUT MinimumConfigTimeout;
        DOT11_MAC_ADDRESS GroupBSSID;	// Optional
        BOOLEAN bUseGroupBSSID;
        DOT11_WFD_CHANNEL OperatingChannel;
        BOOLEAN bUseSpecifiedOperatingChannel;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_SEND_INVITATION_RESPONSE_PARAMETERS, * PDOT11_SEND_INVITATION_RESPONSE_PARAMETERS;

#define DOT11_SIZEOF_SEND_INVITATION_RESPONSE_PARAMETERS_REVISION_1 sizeof(DOT11_SEND_INVITATION_RESPONSE_PARAMETERS)


// OID_DOT11_WFD_SEND_PROVISION_DISCOVERY_REQUEST
#define OID_DOT11_WFD_SEND_PROVISION_DISCOVERY_REQUEST  NWF_DEFINE_OID(0x0F, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)

#define DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1     1
    typedef
    struct _DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        DOT11_DIALOG_TOKEN DialogToken;
        DOT11_MAC_ADDRESS PeerDeviceAddress;
        ULONG uSendTimeout;
        DOT11_WFD_GROUP_CAPABILITY GroupCapability;
        DOT11_WFD_GROUP_ID GroupID;
        BOOLEAN bUseGroupID;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS, * PDOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS;

#define DOT11_SIZEOF_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1  sizeof(DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS)


// OID_DOT11_WFD_SEND_PROVISION_DISCOVERY_RESPONSE
#define OID_DOT11_WFD_SEND_PROVISION_DISCOVERY_RESPONSE NWF_DEFINE_OID(0x10, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)

#define DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1     1
    typedef
    struct _DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS
    {
    	NDIS_OBJECT_HEADER Header;
        DOT11_MAC_ADDRESS ReceiverDeviceAddress;
        DOT11_DIALOG_TOKEN DialogToken;
        PVOID RequestContext;
        ULONG uSendTimeout;
        ULONG uIEsOffset;
        ULONG uIEsLength;
    } DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS, * PDOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS;

#define DOT11_SIZEOF_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1  sizeof(DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS)

// OID_DOT11_WFD_GET_DIALOG_TOKEN
#define OID_DOT11_WFD_GET_DIALOG_TOKEN       NWF_DEFINE_OID(0x11, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)
// DOT11_DIALOG_TOKEN

// OID_DOT11_WFD_STOP_DISCOVERY
#define OID_DOT11_WFD_STOP_DISCOVERY NWF_DEFINE_OID(0x12, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)

// OID_DOT11_WFD_ENABLE_HRDSSS_DEVICES
#define OID_DOT11_WFD_ENABLE_HRDSSS_DEVICES NWF_DEFINE_OID(0x13, NWF_WFD_DEVICE_OID, NWF_OPTIONAL_OID)

// OID_DOT11_WFD_DEVICE_LISTEN_CHANNEL
#define OID_DOT11_WFD_DEVICE_LISTEN_CHANNEL NWF_DEFINE_OID(0x14, NWF_WFD_DEVICE_OID, NWF_MANDATORY_OID)
#define DOT11_WFD_DEVICE_LISTEN_CHANNEL_REVISION_1  1
#define DOT11_SIZEOF_WFD_DEVICE_LISTEN_CHANNEL_REVISION_1 sizeof(DOT11_WFD_DEVICE_LISTEN_CHANNEL)
    typedef
    struct _DOT11_WFD_DEVICE_LISTEN_CHANNEL
    {
        NDIS_OBJECT_HEADER Header;
        UCHAR ChannelNumber;
    } DOT11_WFD_DEVICE_LISTEN_CHANNEL, *PDOT11_WFD_DEVICE_LISTEN_CHANNEL;


// OID_DOT11_WFD_DESIRED_GROUP_ID
#define OID_DOT11_WFD_DESIRED_GROUP_ID              NWF_DEFINE_OID(0x01, NWF_WFD_ROLE_OID, NWF_MANDATORY_OID)
// DOT11_WFD_GROUP_ID


// OID_DOT11_WFD_START_GO_REQUEST
#define OID_DOT11_WFD_START_GO_REQUEST                  NWF_DEFINE_OID(0x02, NWF_WFD_ROLE_OID, NWF_MANDATORY_OID)
// No data


// OID_DOT11_WFD_GROUP_START_PARAMETERS
#define OID_DOT11_WFD_GROUP_START_PARAMETERS            NWF_DEFINE_OID(0x03, NWF_WFD_ROLE_OID, NWF_MANDATORY_OID)

#define DOT11_WFD_GROUP_START_PARAMETERS_REVISION_1     1
    typedef
    struct _DOT11_WFD_GROUP_START_PARAMETERS {
        NDIS_OBJECT_HEADER Header;
        DOT11_WFD_CHANNEL AdvertisedOperatingChannel;
    } DOT11_WFD_GROUP_START_PARAMETERS, * PDOT11_WFD_GROUP_START_PARAMETERS;

#define DOT11_SIZEOF_WFD_GROUP_START_PARAMETERS_REVISION_1  sizeof(DOT11_WFD_GROUP_START_PARAMETERS)


// OID_DOT11_WFD_CONNECT_TO_GROUP_REQUEST
#define OID_DOT11_WFD_CONNECT_TO_GROUP_REQUEST          NWF_DEFINE_OID(0x04, NWF_WFD_ROLE_OID, NWF_MANDATORY_OID)
// No data


// OID_DOT11_WFD_DISCONNECT_FROM_GROUP_REQUEST
#define OID_DOT11_WFD_DISCONNECT_FROM_GROUP_REQUEST     NWF_DEFINE_OID(0x05, NWF_WFD_ROLE_OID, NWF_MANDATORY_OID)
// No data


// OID_DOT11_WFD_GROUP_JOIN_PARAMETERS
#define OID_DOT11_WFD_GROUP_JOIN_PARAMETERS             NWF_DEFINE_OID(0x06, NWF_WFD_ROLE_OID, NWF_MANDATORY_OID)

#define DOT11_WFD_GROUP_JOIN_PARAMETERS_REVISION_1      1
    typedef
    struct _DOT11_WFD_GROUP_JOIN_PARAMETERS {
        NDIS_OBJECT_HEADER Header;
        DOT11_WFD_CHANNEL GOOperatingChannel;
        ULONG GOConfigTime;
        BOOLEAN bInGroupFormation;
        BOOLEAN bWaitForWPSReady;
    } DOT11_WFD_GROUP_JOIN_PARAMETERS, * PDOT11_WFD_GROUP_JOIN_PARAMETERS;

#define DOT11_SIZEOF_WFD_GROUP_JOIN_PARAMETERS_REVISION_1  sizeof(DOT11_WFD_GROUP_JOIN_PARAMETERS)



#endif // NWF_WFD_SUPPORTED

#ifdef NWF_POWER_SAVE_SUPPORTED
#define NWF_POWER_SAVE_OID    (0x07U)

// OID_DOT11_POWER_MGMT_MODE_AUTO_ENABLED
#define OID_DOT11_POWER_MGMT_MODE_AUTO_ENABLED NWF_DEFINE_OID(0x01, NWF_POWER_SAVE_OID, NWF_MANDATORY_OID)

#define DOT11_POWER_MGMT_AUTO_MODE_ENABLED_REVISION_1 1
#define DOT11_SIZEOF_POWER_MGMT_AUTO_MODE_ENABLE_INFO_REVISION_1 sizeof(DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO)
    typedef struct _DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
        NDIS_OBJECT_HEADER  Header;
        BOOLEAN             bEnabled;
    } DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO, * PDOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO;

// OID_DOT11_POWER_MGMT_MODE_STATUS
#define OID_DOT11_POWER_MGMT_MODE_STATUS NWF_DEFINE_OID(0x02, NWF_POWER_SAVE_OID, NWF_MANDATORY_OID)
    typedef enum _DOT11_POWER_MODE_REASON {
        dot11_power_mode_reason_no_change           =0, // initially in this state, has not changed since
        dot11_power_mode_reason_noncompliant_AP     =1, // AP is not compliant. As to be in CAM
        dot11_power_mode_reason_legacy_WFD_device   =2, // WFD device is legacy
        dot11_power_mode_reason_compliant_AP 	    =3, // AP connected does PSM correctly, PSM can works
        dot11_power_mode_reason_compliant_WFD_device=4, // All connected WFD device can do PSM
        dot11_power_mode_reason_others              =5
    } DOT11_POWER_MODE_REASON;

#define DOT11_POWER_MGMT_MODE_STATUS_INFO_REVISION_1 1
#define DOT11_SIZEOF_POWER_MGMT_MODE_STATUS_INFO_REVISION_1 sizeof(DOT11_POWER_MGMT_MODE_STATUS_INFO)
    typedef struct _DOT11_POWER_MGMT_MODE_STATUS_INFO {
        NDIS_OBJECT_HEADER      Header;
        DOT11_POWER_MODE        PowerSaveMode;
        ULONG                   uPowerSaveLevel;
        DOT11_POWER_MODE_REASON Reason;
    } DOT11_POWER_MGMT_MODE_STATUS_INFO, * PDOT11_POWER_MGMT_MODE_STATUSINFO;

//OID_DOT11_OFFLOAD_NETWORK_LIST
#define OID_DOT11_OFFLOAD_NETWORK_LIST NWF_DEFINE_OID(0x03, NWF_POWER_SAVE_OID, NWF_MANDATORY_OID)

    #define DOT11_MAX_CHANNEL_HINTS 4
    #define DOT11_INVALID_CHANNEL_NUMBER 0
    typedef struct DOT11_CHANNEL_HINT {
    DOT11_PHY_TYPE  Dot11PhyType;
    ULONG           uChannelNumber;
    } DOT11_CHANNEL_HINT, * PDOT11_CHANNEL_HINT;

    typedef struct DOT11_OFFLOAD_NETWORK {
        DOT11_SSID              Ssid;
        DOT11_CIPHER_ALGORITHM  UnicastCipher;
        DOT11_AUTH_ALGORITHM    AuthAlgo;
        DOT11_CHANNEL_HINT      Dot11ChannelHints[DOT11_MAX_CHANNEL_HINTS];
    } DOT11_OFFLOAD_NETWORK, * PDOT11_OFFLOAD_NETWORK;

#define DOT11_NLO_FLAG_STOP_NLO_INDICATION 	 0x00000001 // stop NLO scan and NLO discovery indications
#define DOT11_NLO_FLAG_SCAN_ON_AOAC_PLATFORM 0x00000002 // typically used on AOAC platform in connected standby
#define DOT11_NLO_FLAG_SCAN_AT_SYSTEM_RESUME 0x00000004 // used at system resume once for instant connect

#define DOT11_OFFLOAD_NETWORK_LIST_REVISION_1 1
#define DOT11_SIZEOF_OFFLOAD_NETWORK_LIST_INFO_REVISION_1(_uNumOfEntry) \
	(FIELD_OFFSET(DOT11_OFFLOAD_NETWORK_LIST_INFO, offloadNetworkList)+ \
		sizeof(DOT11_OFFLOAD_NETWORK)*(_uNumOfEntry))

//
// when NLO has 0 entry, the minimum size is this. Drivers can safely access fields except entries.
//
#define DOT11_MIN_SIZEOF_OFFLOAD_NETWORK_LIST_INFO_REVISION_1 \
	(FIELD_OFFSET(DOT11_OFFLOAD_NETWORK_LIST_INFO, offloadNetworkList))

    typedef struct _DOT11_OFFLOAD_NETWORK_LIST_INFO {
        NDIS_OBJECT_HEADER      Header;
        ULONG   ulFlags;					// DOT11_NLO_FLAG_*
        ULONG   FastScanPeriod;
        ULONG   FastScanIterations;
        ULONG   SlowScanPeriod;
        ULONG   uNumOfEntries;
        DOT11_OFFLOAD_NETWORK   offloadNetworkList[1];
    } DOT11_OFFLOAD_NETWORK_LIST_INFO, * PDOT11_OFFLOAD_NETWORK_LIST_INFO;

#endif // NWF_POWER_SAVE_SUPPORTED

// NDIS_STATUS_DOT11_OFFLOAD_NETWORK_STATUS_CHANGED
#define DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS_REVISION_1    1

    typedef
    struct _DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS
    {
        NDIS_OBJECT_HEADER Header;
        NDIS_STATUS Status;
    } DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS, * PDOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS;

#define DOT11_SIZEOF_OFFLOAD_NETWORK_STATUS_PARAMETERS_REVISION_1 sizeof(DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS)


#define NWF_MANUFACTURING_OID     (0x08U)

#define OID_DOT11_MANUFACTURING_TEST NWF_DEFINE_OID(0x01, NWF_MANUFACTURING_OID, NWF_OPTIONAL_OID)
#define DOT11_MANUFACTURING_TEST_REVISION_1 1

    typedef enum _DOT11_MANUFACTURING_TEST_TYPE {
        dot11_manufacturing_test_unknown = 0,
        dot11_manufacturing_test_self_start = 1,
        dot11_manufacturing_test_self_query_result = 2,
        dot11_manufacturing_test_rx = 3,
        dot11_manufacturing_test_tx = 4,
        dot11_manufacturing_test_query_adc = 5,
        dot11_manufacturing_test_set_data = 6,
        dot11_manufacturing_test_query_data = 7,
        dot11_manufacturing_test_sleep = 8,
        dot11_manufacturing_test_awake = 9,
        dot11_manufacturing_test_IHV_start = 0x80000000,
        dot11_manufacturing_test_IHV_end = 0xffffffff
    } DOT11_MANUFACTURING_TEST_TYPE, * PDOT11_MANUFACTURING_TEST_TYPE;

    typedef struct _DOT11_MANUFACTURING_TEST {
        DOT11_MANUFACTURING_TEST_TYPE dot11ManufacturingTestType;
        ULONG uBufferLength;
        UCHAR ucBuffer[1];
    } DOT11_MANUFACTURING_TEST, * PDOT11_MANUFACTURING_TEST;


    // dot11_manufacturing_test_self
    typedef enum DOT11_MANUFACTURING_SELF_TEST_TYPE {
        DOT11_MANUFACTURING_SELF_TEST_TYPE_INTERFACE = 1,
        DOT11_MANUFACTURING_SELF_TEST_TYPE_RF_INTERFACE,
        DOT11_MANUFACTURING_SELF_TEST_TYPE_BT_COEXISTENCE
    } DOT11_MANUFACTURING_SELF_TEST_TYPE, * PDOT11_MANUFACTURING_SELF_TEST_TYPE;

    typedef struct _DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
        DOT11_MANUFACTURING_SELF_TEST_TYPE SelfTestType;
        ULONG uTestID;
        ULONG uPinBitMask;
        PVOID pvContext;
        ULONG uBufferLength;
        UCHAR ucBufferIn[1];                // Additional input for self-test (optional)
    } DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS, *PDOT11_MANUFACTURING_SELF_TEST_SET_PARAMS;

    typedef struct _DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
        DOT11_MANUFACTURING_SELF_TEST_TYPE SelfTestType;
        ULONG uTestID;
        BOOLEAN bResult;                    // PASS/FAIL
        ULONG uPinFailedBitMask;            // Detected PIN faults
        PVOID pvContext;
        ULONG uBytesWrittenOut;
        UCHAR ucBufferOut[1];               // Additional output from self-test (optional)
    } DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS, *PDOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS;

    // Band
    typedef enum DOT11_BAND {
        dot11_band_2p4g = 1,
        dot11_band_4p9g,
        dot11_band_5g
    } DOT11_BAND, * PDOT11_BAND;

    // dot11_manufacturing_test_rx
    typedef struct _DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
        BOOLEAN     bEnabled;
        DOT11_BAND  Dot11Band;
        ULONG       uChannel;
        LONG        PowerLevel;
    } DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX, * PDOT11_MANUFACTURING_FUNCTIONAL_TEST_RX;

    // dot11_manufacturing_test_tx
    typedef struct _DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
        BOOLEAN     bEnable;
        BOOLEAN     bOpenLoop;
        DOT11_BAND  Dot11Band;
        ULONG       uChannel;
        ULONG       uSetPowerLevel;
        LONG        ADCPowerLevel;
    } DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX, * PDOT11_MANUFACTURING_FUNCTIONAL_TEST_TX;

    // dot11_manufacturing_test_query_adc
    typedef struct _DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
        DOT11_BAND  Dot11Band;
        ULONG       uChannel;
        LONG        ADCPowerLevel;
    } DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC, * PDOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC;


    // dot11_manufacturing_test_set_data
    typedef struct _DOT11_MANUFACTURING_TEST_SET_DATA {
        ULONG uKey;
        ULONG uOffset;
        ULONG uBufferLength;
        UCHAR ucBufferIn[1];
    } DOT11_MANUFACTURING_TEST_SET_DATA, * PDOT11_MANUFACTURING_TEST_SET_DATA;


    // dot11_manufacturing_test_query_data
    typedef struct _DOT11_MANUFACTURING_TEST_QUERY_DATA {
        ULONG uKey;
        ULONG uOffset;
        ULONG uBufferLength;
        ULONG uBytesRead;
        UCHAR ucBufferOut[1];
    } DOT11_MANUFACTURING_TEST_QUERY_DATA, * PDOT11_MANUFACTURING_TEST_QUERY_DATA;


    // dot11_manufacturing_test_sleep
    typedef struct _DOT11_MANUFACTURING_TEST_SLEEP {
        ULONG uSleepTime;
        PVOID pvContext;
    } DOT11_MANUFACTURING_TEST_SLEEP, * PDOT11_MANUFACTURING_TEST_SLEEP;


    // dot11_manufacturing_test_awake
    // There is no data associated with this call.

    // Manufacturing notifications
    typedef enum _DOT11_MANUFACTURING_CALLBACK_TYPE {
        dot11_manufacturing_callback_unknown = 0,
        dot11_manufacturing_callback_self_test_complete = 1,
        dot11_manufacturing_callback_sleep_complete = 2,
        dot11_manufacturing_callback_IHV_start = 0x80000000,
        dot11_manufacturing_callback_IHV_end = 0xffffffff
    } DOT11_MANUFACTURING_CALLBACK_TYPE, * PDOT11_MANUFACTURING_CALLBACK_TYPE;

    typedef struct DOT11_MANUFACTURING_CALLBACK_PARAMETERS
    {
        #define DOT11_MANUFACTURING_CALLBACK_REVISION_1  1
        NDIS_OBJECT_HEADER              Header;
        DOT11_MANUFACTURING_CALLBACK_TYPE dot11ManufacturingCallbackType;
        ULONG                           uStatus;
        PVOID                           pvContext;
    } DOT11_MANUFACTURING_CALLBACK_PARAMETERS, * PDOT11_MANUFACTURING_CALLBACK_PARAMETERS;

//
// OID_DOT11_SET_FT_REASSOCIATION_PARAMETERS is a new Oid for WDI.  It is defined here
// because it is a Direct Oid that needs to be visible to Ndis
//
#define OID_DOT11_SET_FT_REASSOCIATION_PARAMETERS NWF_DEFINE_OID(0x68, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)

//
// OID_DOT11_SET_SAE_AUTH_PARAMS is a new Oid for WDI.  It is defined here
// because it is a Direct Oid that needs to be visible to Ndis
//
#define OID_DOT11_SET_SAE_AUTH_PARAMS NWF_DEFINE_OID(0x72, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)

//
// OID_DOT11_SET_NWF_PMKID_LIST is used for returning PmkIds computed by nwifi.
// It is defined here because it is a Direct Oid that needs to be visible to Ndis
//
#define OID_DOT11_SET_NWF_PMKID_LIST NWF_DEFINE_OID(0x75, NWF_OPERATIONAL_OID, NWF_MANDATORY_OID)

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // __WINDOT11_H__
