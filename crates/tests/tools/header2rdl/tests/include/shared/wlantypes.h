/*++

    Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    wlantypes.h

Abstract:

    Structures used to hold wlan information.

--*/

#ifndef __WLANTYPES_H__
#define __WLANTYPES_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


typedef enum _DOT11_BSS_TYPE {
    dot11_BSS_type_infrastructure = 1,
    dot11_BSS_type_independent = 2,
    dot11_BSS_type_any = 3
} DOT11_BSS_TYPE, * PDOT11_BSS_TYPE;

#define DOT11_SSID_MAX_LENGTH   32      // 32 bytes
typedef struct _DOT11_SSID {
#ifndef __midl
    _Field_range_(0,32)
#endif
    ULONG uSSIDLength;
    UCHAR ucSSID[DOT11_SSID_MAX_LENGTH];
} DOT11_SSID, * PDOT11_SSID;


// DOT11_AUTH_ALGO_LIST
#ifdef __midl
// use the 4-byte enum
typedef [v1_enum] enum _DOT11_AUTH_ALGORITHM {
#else
typedef enum _DOT11_AUTH_ALGORITHM {
#endif
    DOT11_AUTH_ALGO_80211_OPEN          = 1,
    DOT11_AUTH_ALGO_80211_SHARED_KEY    = 2,
    DOT11_AUTH_ALGO_WPA                 = 3,
    DOT11_AUTH_ALGO_WPA_PSK             = 4,
    DOT11_AUTH_ALGO_WPA_NONE            = 5,               // used in NatSTA only
    DOT11_AUTH_ALGO_RSNA                = 6,
    DOT11_AUTH_ALGO_RSNA_PSK            = 7,
    DOT11_AUTH_ALGO_WPA3                = 8,               // means WPA3 Enterprise 192 bits
#if (NTDDI_VERSION >= NTDDI_WIN10_FE)
    DOT11_AUTH_ALGO_WPA3_ENT_192        = DOT11_AUTH_ALGO_WPA3,
#endif
    DOT11_AUTH_ALGO_WPA3_SAE            = 9,
#if (NTDDI_VERSION >= NTDDI_WIN10_VB)
    DOT11_AUTH_ALGO_OWE                 = 10,
#endif
#if (NTDDI_VERSION >= NTDDI_WIN10_FE)
    DOT11_AUTH_ALGO_WPA3_ENT            = 11,
#endif
    DOT11_AUTH_ALGO_IHV_START           = 0x80000000,
    DOT11_AUTH_ALGO_IHV_END             = 0xffffffff
} DOT11_AUTH_ALGORITHM, * PDOT11_AUTH_ALGORITHM;

#define DOT11_AUTH_ALGORITHM_OPEN_SYSTEM        DOT11_AUTH_ALGO_80211_OPEN
#define DOT11_AUTH_ALGORITHM_SHARED_KEY         DOT11_AUTH_ALGO_80211_SHARED_KEY
#define DOT11_AUTH_ALGORITHM_WPA                DOT11_AUTH_ALGO_WPA
#define DOT11_AUTH_ALGORITHM_WPA_PSK            DOT11_AUTH_ALGO_WPA_PSK
#define DOT11_AUTH_ALGORITHM_WPA_NONE           DOT11_AUTH_ALGO_WPA_NONE
#define DOT11_AUTH_ALGORITHM_RSNA               DOT11_AUTH_ALGO_RSNA
#define DOT11_AUTH_ALGORITHM_RSNA_PSK           DOT11_AUTH_ALGO_RSNA_PSK
#define DOT11_AUTH_ALGORITHM_WPA3               DOT11_AUTH_ALGO_WPA3
#if (NTDDI_VERSION >= NTDDI_WIN10_FE)
#define DOT11_AUTH_ALGORITHM_WPA3_ENT_192       DOT11_AUTH_ALGO_WPA3_ENT_192
#endif
#define DOT11_AUTH_ALGORITHM_WPA3_SAE           DOT11_AUTH_ALGO_WPA3_SAE
#if (NTDDI_VERSION >= NTDDI_WIN10_VB)
#define DOT11_AUTH_ALGORITHM_OWE                DOT11_AUTH_ALGO_OWE
#endif
#if (NTDDI_VERSION >= NTDDI_WIN10_FE)
#define DOT11_AUTH_ALGORITHM_WPA3_ENT           DOT11_AUTH_ALGO_WPA3_ENT
#endif
// Cipher algorithm Ids (for little endian platform)
#ifdef __midl
// use the 4-byte enum
typedef [v1_enum] enum _DOT11_CIPHER_ALGORITHM {
#else
typedef enum _DOT11_CIPHER_ALGORITHM {
#endif
    DOT11_CIPHER_ALGO_NONE = 0x00,
    DOT11_CIPHER_ALGO_WEP40 = 0x01,
    DOT11_CIPHER_ALGO_TKIP = 0x02,
    DOT11_CIPHER_ALGO_CCMP = 0x04,
    DOT11_CIPHER_ALGO_WEP104 = 0x05,
    DOT11_CIPHER_ALGO_BIP = 0x06,                       // BIP-CMAC-128
    DOT11_CIPHER_ALGO_GCMP = 0x08,                      // GCMP-128
    DOT11_CIPHER_ALGO_GCMP_256 = 0x09,                  // GCMP-256
    DOT11_CIPHER_ALGO_CCMP_256 = 0x0a,                  // CCMP-256
    DOT11_CIPHER_ALGO_BIP_GMAC_128 = 0x0b,              // BIP-GMAC-128
    DOT11_CIPHER_ALGO_BIP_GMAC_256 = 0x0c,              // BIP-GMAC-256
    DOT11_CIPHER_ALGO_BIP_CMAC_256 = 0x0d,              // BIP-CMAC-256
    DOT11_CIPHER_ALGO_WPA_USE_GROUP = 0x100,
    DOT11_CIPHER_ALGO_RSN_USE_GROUP = 0x100,
    DOT11_CIPHER_ALGO_WEP = 0x101,
    DOT11_CIPHER_ALGO_IHV_START = 0x80000000,
    DOT11_CIPHER_ALGO_IHV_END = 0xffffffff
} DOT11_CIPHER_ALGORITHM, * PDOT11_CIPHER_ALGORITHM;

typedef struct DOT11_AUTH_CIPHER_PAIR {
    DOT11_AUTH_ALGORITHM AuthAlgoId;
    DOT11_CIPHER_ALGORITHM CipherAlgoId;
} DOT11_AUTH_CIPHER_PAIR, * PDOT11_AUTH_CIPHER_PAIR;


#define DOT11_OI_MAX_LENGTH   5     // See 802.11-2012 Section 8.4.1.31
#define DOT11_OI_MIN_LENGTH   3     // See 802.11-2012 Section 8.4.1.31
typedef struct _DOT11_OI {
    USHORT OILength;    // Must be between DOT11_OI_MIN_LENGTH and DOT11_OI_MAX_LENGTH
    UCHAR  OI[DOT11_OI_MAX_LENGTH];
} DOT11_OI, *PDOT11_OI;

//
//  Similar to DOT11_INTERWORKING_ACCESSNETWORKOPTIONS, but it doesn't have bit fields, which are not allowed for RPC interfaces
//
typedef struct DOT11_ACCESSNETWORKOPTIONS { // 802.11-2012 Figure 8-352
    UINT8 AccessNetworkType;                 // 802.11-2012 Table 8-174
    UINT8 Internet;                          // 1 = Internet connectivity
    UINT8 ASRA;                              // Additional Step Required for Access
    UINT8 ESR;                               // Emergency Services Reachable
    UINT8 UESA;                              // Unauthenticated Emergency Services Available
} DOT11_ACCESSNETWORKOPTIONS, *PDOT11_ACCESSNETWORKOPTIONS;

typedef struct DOT11_VENUEINFO            { // 802.11-2012 Figure 8-72
    UINT8 VenueGroup;                       // 802.11-2012 Table 8-52
    UINT8 VenueType;                        // 802.11-2012 Table 8-53
} DOT11_VENUEINFO, *PDOT11_VENUEINFO;


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __WLANTYPES_H__
