/*++

Copyright (c) 2004 Microsoft Corporation

Module Name:

    wlanapi.h

Abstract:

    Definitions and data strcutures for wlan auto config client side API.

Environment:

    User mode only

Revision History:

    11/8/2004    created

--*/

#ifndef _WLAN_WLANAPI_H
#define _WLAN_WLANAPI_H

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <l2cmn.h>
#include <windot11.h>
#include <EapTypes.h>

#ifdef __cplusplus
extern "C" {
#endif

#ifndef _WIN32_WINNT
#error _WIN32_WINNT must be defined to use WLAN API. It could be either _WIN32_WINNT_WIN7, _WIN32_WINNT_VISTA, or _WIN32_WINNT_WINXP, and so on.
#endif

// major version is in low-order WORD,
// minor version is in high-order WORD
#define WLAN_API_VERSION_1_0    0x00000001
#define WLAN_API_VERSION_2_0    0x00000002
#define WLAN_API_VERSION_MAJOR(_v)  ((_v) & 0xffff)
#define WLAN_API_VERSION_MINOR(_v)  (((DWORD)(_v)) >> 16)
#define WLAN_API_MAKE_VERSION(_major, _minor)   (((DWORD)(_minor)) << 16 | (_major))

// WLAN API is version 1 in all WinXP
// and version 2 in Vista
#if (_WIN32_WINNT >= _WIN32_WINNT_VISTA)
    #define WLAN_API_VERSION        WLAN_API_VERSION_2_0
#else
    #if (_WIN32_WINNT >= _WIN32_WINNT_WINXP)
        #define WLAN_API_VERSION        WLAN_API_VERSION_1_0
    #else
        #error WLAN API is not supported on platform earlier than Windows XP.
    #endif // (_WIN32_WINNT >= _WIN32_WINNT_WINXP)
#endif  // (_WIN32_WINNT >= _WIN32_WINNT_VISTA)

// maximum length of name, in characters
#define WLAN_MAX_NAME_LENGTH L2_PROFILE_MAX_NAME_LENGTH

// profile flags
#define WLAN_PROFILE_GROUP_POLICY                   0x00000001
#define WLAN_PROFILE_USER                           0x00000002
#define WLAN_PROFILE_GET_PLAINTEXT_KEY              0x00000004
// the following flags are only used for WlanSaveTemporaryProfile API
#define WLAN_PROFILE_CONNECTION_MODE_SET_BY_CLIENT  0x00010000
#define WLAN_PROFILE_CONNECTION_MODE_AUTO           0x00020000

// EAPHost data storage flags
#define WLAN_SET_EAPHOST_DATA_ALL_USERS 0x00000001

// struct WLAN_PROFILE_INFO defines the basic information of an 802.11 network profile
typedef struct _WLAN_PROFILE_INFO {
    WCHAR strProfileName[WLAN_MAX_NAME_LENGTH];
    DWORD dwFlags;
} WLAN_PROFILE_INFO, *PWLAN_PROFILE_INFO;


typedef struct _DOT11_NETWORK {
    DOT11_SSID dot11Ssid;
    DOT11_BSS_TYPE dot11BssType;
} DOT11_NETWORK, *PDOT11_NETWORK;

// proximity service discovery IE specific definitions

// the maximum data size in one PSD IE data entry (payload), in bytes
#define DOT11_PSD_IE_MAX_DATA_SIZE 240
// the maximum number of PSD IE data entries
#define DOT11_PSD_IE_MAX_ENTRY_NUMBER 5

typedef struct _WLAN_RAW_DATA {
    // size of the data blob
    DWORD dwDataSize;
#ifdef __midl
    [unique, size_is(dwDataSize)] BYTE DataBlob[*];
#else
    BYTE DataBlob[1];
#endif
} WLAN_RAW_DATA, *PWLAN_RAW_DATA;

typedef struct _WLAN_RAW_DATA_LIST {
    DWORD dwTotalSize;
    DWORD dwNumberOfItems;
    struct {
        // the beginning of the data blob
        // the offset is w.r.t. the beginning of the entry
        DWORD dwDataOffset;
        // size of the data blob
        DWORD dwDataSize;
    } DataList[1];
} WLAN_RAW_DATA_LIST, *PWLAN_RAW_DATA_LIST;

typedef enum _WLAN_CONNECTION_MODE {
    wlan_connection_mode_profile = 0,
    wlan_connection_mode_temporary_profile,
    wlan_connection_mode_discovery_secure,
    wlan_connection_mode_discovery_unsecure,
    wlan_connection_mode_auto,
    wlan_connection_mode_invalid
} WLAN_CONNECTION_MODE, *PWLAN_CONNECTION_MODE;

// Wlan reason code
//
// They are put in the following range:
// each component got 0x1000 numbers, within which:
// the first half is for capability mismatch reason,
// the second half for connect/security error reason.
//
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// IMPORTANT: If you add/remove a reason code, please
// modify the corresponding entry in wlanres.h and wlanres.rc
// and the corresponding mapping in wlanapi.c
typedef DWORD WLAN_REASON_CODE, *PWLAN_REASON_CODE;
#define WLAN_REASON_CODE_SUCCESS                L2_REASON_CODE_SUCCESS
// general codes
#define WLAN_REASON_CODE_UNKNOWN                L2_REASON_CODE_UNKNOWN

#define WLAN_REASON_CODE_RANGE_SIZE             L2_REASON_CODE_GROUP_SIZE
#define WLAN_REASON_CODE_BASE                   L2_REASON_CODE_DOT11_AC_BASE

// range for Auto Config
//
#define WLAN_REASON_CODE_AC_BASE                L2_REASON_CODE_DOT11_AC_BASE
#define WLAN_REASON_CODE_AC_CONNECT_BASE        (WLAN_REASON_CODE_AC_BASE + WLAN_REASON_CODE_RANGE_SIZE / 2)
#define WLAN_REASON_CODE_AC_END                 (WLAN_REASON_CODE_AC_BASE + WLAN_REASON_CODE_RANGE_SIZE - 1)

// range for profile manager
// it has profile adding failure reason codes, but may not have
// connection reason codes
//
#define WLAN_REASON_CODE_PROFILE_BASE           L2_REASON_CODE_PROFILE_BASE
#define WLAN_REASON_CODE_PROFILE_CONNECT_BASE   (WLAN_REASON_CODE_PROFILE_BASE + WLAN_REASON_CODE_RANGE_SIZE / 2)
#define WLAN_REASON_CODE_PROFILE_END            (WLAN_REASON_CODE_PROFILE_BASE + WLAN_REASON_CODE_RANGE_SIZE - 1)

// range for MSM
//
#define WLAN_REASON_CODE_MSM_BASE               L2_REASON_CODE_DOT11_MSM_BASE
#define WLAN_REASON_CODE_MSM_CONNECT_BASE       (WLAN_REASON_CODE_MSM_BASE + WLAN_REASON_CODE_RANGE_SIZE / 2)
#define WLAN_REASON_CODE_MSM_END                (WLAN_REASON_CODE_MSM_BASE + WLAN_REASON_CODE_RANGE_SIZE - 1)

// range for MSMSEC
//
#define WLAN_REASON_CODE_MSMSEC_BASE            L2_REASON_CODE_DOT11_SECURITY_BASE
#define WLAN_REASON_CODE_MSMSEC_CONNECT_BASE    (WLAN_REASON_CODE_MSMSEC_BASE + WLAN_REASON_CODE_RANGE_SIZE / 2)
#define WLAN_REASON_CODE_MSMSEC_END             (WLAN_REASON_CODE_MSMSEC_BASE + WLAN_REASON_CODE_RANGE_SIZE - 1)

// range for codes reserved for system usage
//
#define WLAN_REASON_CODE_RESERVED_BASE          L2_REASON_CODE_RESERVED_BASE
#define WLAN_REASON_CODE_RESERVED_END           (WLAN_REASON_CODE_RESERVED_BASE + WLAN_REASON_CODE_RANGE_SIZE - 1)

// AC network incompatible reason codes
//
#define WLAN_REASON_CODE_NETWORK_NOT_COMPATIBLE (WLAN_REASON_CODE_AC_BASE +1)
#define WLAN_REASON_CODE_PROFILE_NOT_COMPATIBLE (WLAN_REASON_CODE_AC_BASE +2)

// AC connect reason code
//
#define WLAN_REASON_CODE_NO_AUTO_CONNECTION     (WLAN_REASON_CODE_AC_CONNECT_BASE +1)
#define WLAN_REASON_CODE_NOT_VISIBLE            (WLAN_REASON_CODE_AC_CONNECT_BASE +2)
#define WLAN_REASON_CODE_GP_DENIED              (WLAN_REASON_CODE_AC_CONNECT_BASE +3)
#define WLAN_REASON_CODE_USER_DENIED            (WLAN_REASON_CODE_AC_CONNECT_BASE +4)
#define WLAN_REASON_CODE_BSS_TYPE_NOT_ALLOWED   (WLAN_REASON_CODE_AC_CONNECT_BASE +5)
#define WLAN_REASON_CODE_IN_FAILED_LIST         (WLAN_REASON_CODE_AC_CONNECT_BASE +6)
#define WLAN_REASON_CODE_IN_BLOCKED_LIST        (WLAN_REASON_CODE_AC_CONNECT_BASE +7)
#define WLAN_REASON_CODE_SSID_LIST_TOO_LONG     (WLAN_REASON_CODE_AC_CONNECT_BASE +8)
#define WLAN_REASON_CODE_CONNECT_CALL_FAIL      (WLAN_REASON_CODE_AC_CONNECT_BASE +9)
#define WLAN_REASON_CODE_SCAN_CALL_FAIL         (WLAN_REASON_CODE_AC_CONNECT_BASE +10)
#define WLAN_REASON_CODE_NETWORK_NOT_AVAILABLE    (WLAN_REASON_CODE_AC_CONNECT_BASE +11)
#define WLAN_REASON_CODE_PROFILE_CHANGED_OR_DELETED \
                                                (WLAN_REASON_CODE_AC_CONNECT_BASE +12)
#define WLAN_REASON_CODE_KEY_MISMATCH           (WLAN_REASON_CODE_AC_CONNECT_BASE + 13)
#define WLAN_REASON_CODE_USER_NOT_RESPOND       (WLAN_REASON_CODE_AC_CONNECT_BASE + 14)
#define WLAN_REASON_CODE_AP_PROFILE_NOT_ALLOWED_FOR_CLIENT \
                                                (WLAN_REASON_CODE_AC_CONNECT_BASE + 15)
#define WLAN_REASON_CODE_AP_PROFILE_NOT_ALLOWED (WLAN_REASON_CODE_AC_CONNECT_BASE + 16)
#define WLAN_REASON_CODE_HOTSPOT2_PROFILE_DENIED \
                                                (WLAN_REASON_CODE_AC_CONNECT_BASE + 17)

// Profile validation errors
//
#define WLAN_REASON_CODE_INVALID_PROFILE_SCHEMA (WLAN_REASON_CODE_PROFILE_BASE +1)
#define WLAN_REASON_CODE_PROFILE_MISSING        (WLAN_REASON_CODE_PROFILE_BASE +2)
#define WLAN_REASON_CODE_INVALID_PROFILE_NAME   (WLAN_REASON_CODE_PROFILE_BASE +3)
#define WLAN_REASON_CODE_INVALID_PROFILE_TYPE   (WLAN_REASON_CODE_PROFILE_BASE +4)
#define WLAN_REASON_CODE_INVALID_PHY_TYPE       (WLAN_REASON_CODE_PROFILE_BASE +5)
#define WLAN_REASON_CODE_MSM_SECURITY_MISSING   (WLAN_REASON_CODE_PROFILE_BASE +6)
#define WLAN_REASON_CODE_IHV_SECURITY_NOT_SUPPORTED \
                                                (WLAN_REASON_CODE_PROFILE_BASE +7)
#define WLAN_REASON_CODE_IHV_OUI_MISMATCH       (WLAN_REASON_CODE_PROFILE_BASE +8)
        // IHV OUI not present but there is IHV settings in profile
#define WLAN_REASON_CODE_IHV_OUI_MISSING        (WLAN_REASON_CODE_PROFILE_BASE +9)
        // IHV OUI is present but there is no IHV settings in profile
#define WLAN_REASON_CODE_IHV_SETTINGS_MISSING   (WLAN_REASON_CODE_PROFILE_BASE +10)
        // both/conflict MSMSec and IHV security settings exist in profile
#define WLAN_REASON_CODE_CONFLICT_SECURITY      (WLAN_REASON_CODE_PROFILE_BASE +11)
        // no IHV or MSMSec security settings in profile
#define WLAN_REASON_CODE_SECURITY_MISSING       (WLAN_REASON_CODE_PROFILE_BASE +12)
#define WLAN_REASON_CODE_INVALID_BSS_TYPE       (WLAN_REASON_CODE_PROFILE_BASE +13)
#define WLAN_REASON_CODE_INVALID_ADHOC_CONNECTION_MODE \
                                                (WLAN_REASON_CODE_PROFILE_BASE +14)
#define WLAN_REASON_CODE_NON_BROADCAST_SET_FOR_ADHOC \
                                                (WLAN_REASON_CODE_PROFILE_BASE +15)
#define WLAN_REASON_CODE_AUTO_SWITCH_SET_FOR_ADHOC \
                                                (WLAN_REASON_CODE_PROFILE_BASE +16)
#define WLAN_REASON_CODE_AUTO_SWITCH_SET_FOR_MANUAL_CONNECTION \
                                                (WLAN_REASON_CODE_PROFILE_BASE +17)
#define WLAN_REASON_CODE_IHV_SECURITY_ONEX_MISSING \
                                                (WLAN_REASON_CODE_PROFILE_BASE +18)
#define WLAN_REASON_CODE_PROFILE_SSID_INVALID   (WLAN_REASON_CODE_PROFILE_BASE +19)
#define WLAN_REASON_CODE_TOO_MANY_SSID          (WLAN_REASON_CODE_PROFILE_BASE +20)
#define WLAN_REASON_CODE_IHV_CONNECTIVITY_NOT_SUPPORTED \
                                                (WLAN_REASON_CODE_PROFILE_BASE +21)
#define WLAN_REASON_CODE_BAD_MAX_NUMBER_OF_CLIENTS_FOR_AP \
                                                (WLAN_REASON_CODE_PROFILE_BASE +22)
#define WLAN_REASON_CODE_INVALID_CHANNEL        (WLAN_REASON_CODE_PROFILE_BASE +23)
#define WLAN_REASON_CODE_OPERATION_MODE_NOT_SUPPORTED \
                                                (WLAN_REASON_CODE_PROFILE_BASE +24)
#define WLAN_REASON_CODE_AUTO_AP_PROFILE_NOT_ALLOWED \
                                                (WLAN_REASON_CODE_PROFILE_BASE +25)
#define WLAN_REASON_CODE_AUTO_CONNECTION_NOT_ALLOWED \
                                                (WLAN_REASON_CODE_PROFILE_BASE +26)
#define WLAN_REASON_CODE_HOTSPOT2_PROFILE_NOT_ALLOWED \
                                                (WLAN_REASON_CODE_PROFILE_BASE +27)

// MSM network incompatible reasons
//
#define WLAN_REASON_CODE_UNSUPPORTED_SECURITY_SET_BY_OS \
                                                (WLAN_REASON_CODE_MSM_BASE +1)
#define WLAN_REASON_CODE_UNSUPPORTED_SECURITY_SET \
                                                (WLAN_REASON_CODE_MSM_BASE +2)
#define WLAN_REASON_CODE_BSS_TYPE_UNMATCH       (WLAN_REASON_CODE_MSM_BASE +3)
#define WLAN_REASON_CODE_PHY_TYPE_UNMATCH       (WLAN_REASON_CODE_MSM_BASE +4)
#define WLAN_REASON_CODE_DATARATE_UNMATCH       (WLAN_REASON_CODE_MSM_BASE +5)

// MSM connection failure reasons, to be defined
// failure reason codes
//
        // user called to disconnect
#define WLAN_REASON_CODE_USER_CANCELLED         (WLAN_REASON_CODE_MSM_CONNECT_BASE+1)
        // got disconnect while associating
#define WLAN_REASON_CODE_ASSOCIATION_FAILURE    (WLAN_REASON_CODE_MSM_CONNECT_BASE+2)
        // timeout for association
#define WLAN_REASON_CODE_ASSOCIATION_TIMEOUT    (WLAN_REASON_CODE_MSM_CONNECT_BASE+3)
        // pre-association security completed with failure
#define WLAN_REASON_CODE_PRE_SECURITY_FAILURE   (WLAN_REASON_CODE_MSM_CONNECT_BASE+4)
        // fail to start post-association security
#define WLAN_REASON_CODE_START_SECURITY_FAILURE (WLAN_REASON_CODE_MSM_CONNECT_BASE+5)
        // post-association security completed with failure
#define WLAN_REASON_CODE_SECURITY_FAILURE       (WLAN_REASON_CODE_MSM_CONNECT_BASE+6)
        // security watchdog timeout
#define WLAN_REASON_CODE_SECURITY_TIMEOUT       (WLAN_REASON_CODE_MSM_CONNECT_BASE+7)
        // got disconnect from driver when roaming
#define WLAN_REASON_CODE_ROAMING_FAILURE        (WLAN_REASON_CODE_MSM_CONNECT_BASE+8)
        // failed to start security for roaming
#define WLAN_REASON_CODE_ROAMING_SECURITY_FAILURE   (WLAN_REASON_CODE_MSM_CONNECT_BASE+9)
        // failed to start security for adhoc-join
#define WLAN_REASON_CODE_ADHOC_SECURITY_FAILURE     (WLAN_REASON_CODE_MSM_CONNECT_BASE+10)
        // got disconnection from driver
#define WLAN_REASON_CODE_DRIVER_DISCONNECTED        (WLAN_REASON_CODE_MSM_CONNECT_BASE+11)
        // driver operation failed
#define WLAN_REASON_CODE_DRIVER_OPERATION_FAILURE   (WLAN_REASON_CODE_MSM_CONNECT_BASE+12)
        // Ihv service is not available
#define WLAN_REASON_CODE_IHV_NOT_AVAILABLE    (WLAN_REASON_CODE_MSM_CONNECT_BASE+13)
        // Response from ihv timed out
#define WLAN_REASON_CODE_IHV_NOT_RESPONDING   (WLAN_REASON_CODE_MSM_CONNECT_BASE+14)
        // Timed out waiting for driver to disconnect
#define WLAN_REASON_CODE_DISCONNECT_TIMEOUT   (WLAN_REASON_CODE_MSM_CONNECT_BASE+15)
        // An internal error prevented the operation from being completed.
#define WLAN_REASON_CODE_INTERNAL_FAILURE     (WLAN_REASON_CODE_MSM_CONNECT_BASE+16)
        // UI Request timed out.
#define WLAN_REASON_CODE_UI_REQUEST_TIMEOUT   (WLAN_REASON_CODE_MSM_CONNECT_BASE+17)
        // Roaming too often, post security is not completed after 5 times.
#define WLAN_REASON_CODE_TOO_MANY_SECURITY_ATTEMPTS (WLAN_REASON_CODE_MSM_CONNECT_BASE+18)
        // Failed to start AP
#define WLAN_REASON_CODE_AP_STARTING_FAILURE    (WLAN_REASON_CODE_MSM_CONNECT_BASE+19)
        // Failed to connect because no connectable Access Point was visible
#define WLAN_REASON_CODE_NO_VISIBLE_AP          (WLAN_REASON_CODE_MSM_CONNECT_BASE+20)

// MSMSEC reason codes
//

#define WLAN_REASON_CODE_MSMSEC_MIN                         WLAN_REASON_CODE_MSMSEC_BASE

// Key index specified is not valid
#define WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_KEY_INDEX   (WLAN_REASON_CODE_MSMSEC_BASE+1)
// Key required, PSK present
#define WLAN_REASON_CODE_MSMSEC_PROFILE_PSK_PRESENT         (WLAN_REASON_CODE_MSMSEC_BASE+2)
// Invalid key length
#define WLAN_REASON_CODE_MSMSEC_PROFILE_KEY_LENGTH          (WLAN_REASON_CODE_MSMSEC_BASE+3)
// Invalid PSK length
#define WLAN_REASON_CODE_MSMSEC_PROFILE_PSK_LENGTH          (WLAN_REASON_CODE_MSMSEC_BASE+4)
// No auth/cipher specified
#define WLAN_REASON_CODE_MSMSEC_PROFILE_NO_AUTH_CIPHER_SPECIFIED        (WLAN_REASON_CODE_MSMSEC_BASE+5)
// Too many auth/cipher specified
#define WLAN_REASON_CODE_MSMSEC_PROFILE_TOO_MANY_AUTH_CIPHER_SPECIFIED  (WLAN_REASON_CODE_MSMSEC_BASE+6)
// Profile contains duplicate auth/cipher
#define WLAN_REASON_CODE_MSMSEC_PROFILE_DUPLICATE_AUTH_CIPHER           (WLAN_REASON_CODE_MSMSEC_BASE+7)
// Profile raw data is invalid (1x or key data)
#define WLAN_REASON_CODE_MSMSEC_PROFILE_RAWDATA_INVALID                 (WLAN_REASON_CODE_MSMSEC_BASE+8)
// Invalid auth/cipher combination
#define WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_AUTH_CIPHER             (WLAN_REASON_CODE_MSMSEC_BASE+9)
// 802.1x disabled when it's required to be enabled
#define WLAN_REASON_CODE_MSMSEC_PROFILE_ONEX_DISABLED                   (WLAN_REASON_CODE_MSMSEC_BASE+10)
// 802.1x enabled when it's required to be disabled
#define WLAN_REASON_CODE_MSMSEC_PROFILE_ONEX_ENABLED                    (WLAN_REASON_CODE_MSMSEC_BASE+11)
#define WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_MODE           (WLAN_REASON_CODE_MSMSEC_BASE+12)
#define WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_SIZE           (WLAN_REASON_CODE_MSMSEC_BASE+13)
#define WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_TTL            (WLAN_REASON_CODE_MSMSEC_BASE+14)
#define WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PREAUTH_MODE            (WLAN_REASON_CODE_MSMSEC_BASE+15)
#define WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PREAUTH_THROTTLE        (WLAN_REASON_CODE_MSMSEC_BASE+16)
// PreAuth enabled when PMK cache is disabled
#define WLAN_REASON_CODE_MSMSEC_PROFILE_PREAUTH_ONLY_ENABLED            (WLAN_REASON_CODE_MSMSEC_BASE+17)
// Capability matching failed at network
#define WLAN_REASON_CODE_MSMSEC_CAPABILITY_NETWORK          (WLAN_REASON_CODE_MSMSEC_BASE+18)
// Capability matching failed at NIC
#define WLAN_REASON_CODE_MSMSEC_CAPABILITY_NIC              (WLAN_REASON_CODE_MSMSEC_BASE+19)
// Capability matching failed at profile
#define WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE          (WLAN_REASON_CODE_MSMSEC_BASE+20)
// Network does not support specified discovery type
#define WLAN_REASON_CODE_MSMSEC_CAPABILITY_DISCOVERY        (WLAN_REASON_CODE_MSMSEC_BASE+21)
// Passphrase contains invalid character
#define WLAN_REASON_CODE_MSMSEC_PROFILE_PASSPHRASE_CHAR     (WLAN_REASON_CODE_MSMSEC_BASE+22)
// Key material contains invalid character
#define WLAN_REASON_CODE_MSMSEC_PROFILE_KEYMATERIAL_CHAR     (WLAN_REASON_CODE_MSMSEC_BASE+23)
// Wrong key type specified for the auth/cipher pair
#define WLAN_REASON_CODE_MSMSEC_PROFILE_WRONG_KEYTYPE     (WLAN_REASON_CODE_MSMSEC_BASE+24)
// "Mixed cell" suspected (AP not beaconing privacy, we have privacy enabled profile)
#define WLAN_REASON_CODE_MSMSEC_MIXED_CELL                (WLAN_REASON_CODE_MSMSEC_BASE+25)
// Auth timers or number of timeouts in profile is incorrect
#define WLAN_REASON_CODE_MSMSEC_PROFILE_AUTH_TIMERS_INVALID (WLAN_REASON_CODE_MSMSEC_BASE+26)
// Group key update interval in profile is incorrect
#define WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_GKEY_INTV   (WLAN_REASON_CODE_MSMSEC_BASE+27)
// "Transition network" suspected, trying legacy 802.11 security
#define WLAN_REASON_CODE_MSMSEC_TRANSITION_NETWORK          (WLAN_REASON_CODE_MSMSEC_BASE+28)
// Key contains characters which do not map to ASCII
#define WLAN_REASON_CODE_MSMSEC_PROFILE_KEY_UNMAPPED_CHAR   (WLAN_REASON_CODE_MSMSEC_BASE+29)
// Capability matching failed at profile (auth not found)
#define WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_AUTH     (WLAN_REASON_CODE_MSMSEC_BASE+30)
// Capability matching failed at profile (cipher not found)
#define WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_CIPHER   (WLAN_REASON_CODE_MSMSEC_BASE+31)
// Safe mode value is invalid
#define WLAN_REASON_CODE_MSMSEC_PROFILE_SAFE_MODE           (WLAN_REASON_CODE_MSMSEC_BASE+32)
// Profile requires safe mode, not supported by NIC
#define WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_SAFE_MODE_NIC (WLAN_REASON_CODE_MSMSEC_BASE+33)
// Profile requires safe mode, not supported by network
#define WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_SAFE_MODE_NW  (WLAN_REASON_CODE_MSMSEC_BASE+34)
// Profile has unsupported auth
#define WLAN_REASON_CODE_MSMSEC_PROFILE_UNSUPPORTED_AUTH    (WLAN_REASON_CODE_MSMSEC_BASE+35)
// Profile has unsupported cipher
#define WLAN_REASON_CODE_MSMSEC_PROFILE_UNSUPPORTED_CIPHER  (WLAN_REASON_CODE_MSMSEC_BASE+36)
//If Network requires MFP and NIC does not suppport MFP
#define WLAN_REASON_CODE_MSMSEC_CAPABILITY_MFP_NW_NIC       (WLAN_REASON_CODE_MSMSEC_BASE+37)

// Failed to queue UI request
#define WLAN_REASON_CODE_MSMSEC_UI_REQUEST_FAILURE          (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+1)
// 802.1x authentication did not start within configured time
#define WLAN_REASON_CODE_MSMSEC_AUTH_START_TIMEOUT          (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+2)
// 802.1x authentication did not complete within configured time
#define WLAN_REASON_CODE_MSMSEC_AUTH_SUCCESS_TIMEOUT        (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+3)
// Dynamic key exchange did not start within configured time
#define WLAN_REASON_CODE_MSMSEC_KEY_START_TIMEOUT           (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+4)
// Dynamic key exchange did not succeed within configured time
#define WLAN_REASON_CODE_MSMSEC_KEY_SUCCESS_TIMEOUT         (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+5)
// Message 3 of 4 way handshake has no key data (RSN/WPA)
#define WLAN_REASON_CODE_MSMSEC_M3_MISSING_KEY_DATA         (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+6)
// Message 3 of 4 way handshake has no IE (RSN/WPA)
#define WLAN_REASON_CODE_MSMSEC_M3_MISSING_IE               (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+7)
// Message 3 of 4 way handshake has no Group Key (RSN)
#define WLAN_REASON_CODE_MSMSEC_M3_MISSING_GRP_KEY          (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+8)
// Matching security capabilities of IE in M3 failed (RSN/WPA)
#define WLAN_REASON_CODE_MSMSEC_PR_IE_MATCHING              (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+9)
// Matching security capabilities of Secondary IE in M3 failed (RSN)
#define WLAN_REASON_CODE_MSMSEC_SEC_IE_MATCHING             (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+10)
// Required a pairwise key but AP configured only group keys
#define WLAN_REASON_CODE_MSMSEC_NO_PAIRWISE_KEY             (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+11)
// Message 1 of group key handshake has no key data (RSN/WPA)
#define WLAN_REASON_CODE_MSMSEC_G1_MISSING_KEY_DATA         (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+12)
// Message 1 of group key handshake has no group key
#define WLAN_REASON_CODE_MSMSEC_G1_MISSING_GRP_KEY          (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+13)
// AP reset secure bit after connection was secured
#define WLAN_REASON_CODE_MSMSEC_PEER_INDICATED_INSECURE     (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+14)
// 802.1x indicated there is no authenticator but profile requires 802.1x
#define WLAN_REASON_CODE_MSMSEC_NO_AUTHENTICATOR            (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+15)
// Plumbing settings to NIC failed
#define WLAN_REASON_CODE_MSMSEC_NIC_FAILURE                 (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+16)
// Operation was cancelled by caller
#define WLAN_REASON_CODE_MSMSEC_CANCELLED                   (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+17)
// Key was in incorrect format
#define WLAN_REASON_CODE_MSMSEC_KEY_FORMAT                  (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+18)
// Security downgrade detected
#define WLAN_REASON_CODE_MSMSEC_DOWNGRADE_DETECTED          (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+19)
// PSK mismatch suspected
#define WLAN_REASON_CODE_MSMSEC_PSK_MISMATCH_SUSPECTED      (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+20)
// Forced failure because connection method was not secure
#define WLAN_REASON_CODE_MSMSEC_FORCED_FAILURE              (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+21)
// Message 3 of 4 way handshake contains too many RSN IE (RSN)
#define WLAN_REASON_CODE_MSMSEC_M3_TOO_MANY_RSNIE           (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+22)
// Message 2 of 4 way handshake has no key data (RSN Adhoc)
#define WLAN_REASON_CODE_MSMSEC_M2_MISSING_KEY_DATA         (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+23)
// Message 2 of 4 way handshake has no IE (RSN Adhoc)
#define WLAN_REASON_CODE_MSMSEC_M2_MISSING_IE               (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+24)
#define WLAN_REASON_CODE_MSMSEC_AUTH_WCN_COMPLETED          (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+25)
// Message 3 of 4 way handshake has no Mgmt Group Key (RSN)
#define WLAN_REASON_CODE_MSMSEC_M3_MISSING_MGMT_GRP_KEY     (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+26)
// Message 1 of group key handshake has no group mgmt key
#define WLAN_REASON_CODE_MSMSEC_G1_MISSING_MGMT_GRP_KEY     (WLAN_REASON_CODE_MSMSEC_CONNECT_BASE+27)


#define WLAN_REASON_CODE_MSMSEC_MAX                         WLAN_REASON_CODE_MSMSEC_END

typedef ULONG WLAN_SIGNAL_QUALITY, *PWLAN_SIGNAL_QUALITY;

// available network flags
#define WLAN_AVAILABLE_NETWORK_CONNECTED                    0x00000001  // This network is currently connected
#define WLAN_AVAILABLE_NETWORK_HAS_PROFILE                  0x00000002  // There is a profile for this network
#define WLAN_AVAILABLE_NETWORK_CONSOLE_USER_PROFILE         0x00000004  // The profile is the active console user's per user profile
#define WLAN_AVAILABLE_NETWORK_INTERWORKING_SUPPORTED       0x00000008  // Interworking is supported
#define WLAN_AVAILABLE_NETWORK_HOTSPOT2_ENABLED             0x00000010  // Hotspot2 is enabled
#define WLAN_AVAILABLE_NETWORK_ANQP_SUPPORTED               0x00000020  // ANQP is supported
#define WLAN_AVAILABLE_NETWORK_HOTSPOT2_DOMAIN              0x00000040  // Domain network 
#define WLAN_AVAILABLE_NETWORK_HOTSPOT2_ROAMING             0x00000080  // Roaming network
#define WLAN_AVAILABLE_NETWORK_AUTO_CONNECT_FAILED          0x00000100  // This network failed to connect

// flags that control the list returned by WlanGetAvailableNetworkList
// include all ad hoc network profiles in the available network list, regardless they are visible or not
#define WLAN_AVAILABLE_NETWORK_INCLUDE_ALL_ADHOC_PROFILES           0x00000001
// include all hidden network profiles in the available network list, regardless they are visible or not
#define WLAN_AVAILABLE_NETWORK_INCLUDE_ALL_MANUAL_HIDDEN_PROFILES   0x00000002

typedef struct _WLAN_RATE_SET {
    ULONG uRateSetLength;
    _Field_size_part_(DOT11_RATE_SET_MAX_LENGTH, uRateSetLength) USHORT usRateSet[DOT11_RATE_SET_MAX_LENGTH];
} WLAN_RATE_SET, * PWLAN_RATE_SET;

#define WLAN_MAX_PHY_TYPE_NUMBER    8
//
// struct WLAN_AVAILABLE_NETWORK defines information needed for an available network
typedef struct _WLAN_AVAILABLE_NETWORK {
    WCHAR strProfileName[WLAN_MAX_NAME_LENGTH];
    DOT11_SSID dot11Ssid;
    DOT11_BSS_TYPE dot11BssType;
    ULONG uNumberOfBssids;
    BOOL bNetworkConnectable;
    WLAN_REASON_CODE wlanNotConnectableReason;
    ULONG uNumberOfPhyTypes;
    DOT11_PHY_TYPE dot11PhyTypes[WLAN_MAX_PHY_TYPE_NUMBER];
    // bMorePhyTypes is set to TRUE if the PHY types for the network
    // exceeds WLAN_MAX_PHY_TYPE_NUMBER.
    // In this case, uNumerOfPhyTypes is WLAN_MAX_PHY_TYPE_NUMBER and the
    // first WLAN_MAX_PHY_TYPE_NUMBER PHY types are returned.
    BOOL bMorePhyTypes;
    WLAN_SIGNAL_QUALITY wlanSignalQuality;
    BOOL bSecurityEnabled;
    DOT11_AUTH_ALGORITHM dot11DefaultAuthAlgorithm;
    DOT11_CIPHER_ALGORITHM dot11DefaultCipherAlgorithm;
    DWORD dwFlags;
    DWORD dwReserved;
} WLAN_AVAILABLE_NETWORK, *PWLAN_AVAILABLE_NETWORK;

typedef struct _WLAN_AVAILABLE_NETWORK_V2 {
    WCHAR strProfileName[WLAN_MAX_NAME_LENGTH];
    DOT11_SSID dot11Ssid;
    DOT11_BSS_TYPE dot11BssType;
    ULONG uNumberOfBssids;
    BOOL bNetworkConnectable;
    WLAN_REASON_CODE wlanNotConnectableReason;
    ULONG uNumberOfPhyTypes;
    DOT11_PHY_TYPE dot11PhyTypes[WLAN_MAX_PHY_TYPE_NUMBER];
    // bMorePhyTypes is set to TRUE if the PHY types for the network
    // exceeds WLAN_MAX_PHY_TYPE_NUMBER.
    // In this case, uNumerOfPhyTypes is WLAN_MAX_PHY_TYPE_NUMBER and the
    // first WLAN_MAX_PHY_TYPE_NUMBER PHY types are returned.
    BOOL bMorePhyTypes;
    WLAN_SIGNAL_QUALITY wlanSignalQuality;
    BOOL bSecurityEnabled;
    DOT11_AUTH_ALGORITHM dot11DefaultAuthAlgorithm;
    DOT11_CIPHER_ALGORITHM dot11DefaultCipherAlgorithm;
    DWORD dwFlags;

    // V2 fields
    DOT11_ACCESSNETWORKOPTIONS                 AccessNetworkOptions;
    DOT11_HESSID                               dot11HESSID;
    DOT11_VENUEINFO                            VenueInfo;

    DWORD dwReserved;
} WLAN_AVAILABLE_NETWORK_V2, *PWLAN_AVAILABLE_NETWORK_V2;

typedef struct _WLAN_BSS_ENTRY {
    DOT11_SSID dot11Ssid;
    ULONG uPhyId;
    DOT11_MAC_ADDRESS dot11Bssid;
    DOT11_BSS_TYPE dot11BssType;
    DOT11_PHY_TYPE dot11BssPhyType;
    LONG lRssi;
    ULONG uLinkQuality;
    BOOLEAN bInRegDomain;
    USHORT usBeaconPeriod;
    ULONGLONG ullTimestamp;
    ULONGLONG ullHostTimestamp;
    USHORT usCapabilityInformation;
    ULONG  ulChCenterFrequency;
    WLAN_RATE_SET wlanRateSet;
    // the beginning of the IE blob
    // the offset is w.r.t. the beginning of the entry
    ULONG ulIeOffset;
    // size of the IE blob
    ULONG ulIeSize;
} WLAN_BSS_ENTRY, * PWLAN_BSS_ENTRY;

// struct WLAN_VARIABLE_SIZE_ARRAY defines a list of entries,
// each of which may have different size
typedef struct _WLAN_BSS_LIST {
    // The total size of the data in BYTE
    DWORD dwTotalSize;
    DWORD dwNumberOfItems;
    WLAN_BSS_ENTRY wlanBssEntries[1];
} WLAN_BSS_LIST, *PWLAN_BSS_LIST;


// the states of the network (interface)
#ifdef __midl
// use the 4-byte enum
typedef [v1_enum] enum _WLAN_INTERFACE_STATE {
#else
typedef enum _WLAN_INTERFACE_STATE {
#endif
    wlan_interface_state_not_ready,
    wlan_interface_state_connected,
    wlan_interface_state_ad_hoc_network_formed,
    wlan_interface_state_disconnecting,
    wlan_interface_state_disconnected,
    wlan_interface_state_associating,
    wlan_interface_state_discovering,
    wlan_interface_state_authenticating
} WLAN_INTERFACE_STATE, *PWLAN_INTERFACE_STATE;


// Adhoc network states
#ifdef __midl
// use the 4-byte enum
typedef [v1_enum] enum _WLAN_ADHOC_NETWORK_STATE {
#else
typedef enum _WLAN_ADHOC_NETWORK_STATE {
#endif
    wlan_adhoc_network_state_formed = 0,
    wlan_adhoc_network_state_connected
} WLAN_ADHOC_NETWORK_STATE, *PWLAN_ADHOC_NETWORK_STATE;

// struct WLAN_INTERFACE_INFO defines the basic information for an interface
typedef struct _WLAN_INTERFACE_INFO {
    GUID InterfaceGuid;
    WCHAR strInterfaceDescription[WLAN_MAX_NAME_LENGTH];
    WLAN_INTERFACE_STATE isState;
} WLAN_INTERFACE_INFO, *PWLAN_INTERFACE_INFO;

// structure WLAN_ASSOCIATION_ATTRIBUTES defines attributes of a wireless
// association. The unit for Rx/Tx rate is Kbits/second.
typedef struct _WLAN_ASSOCIATION_ATTRIBUTES {
    DOT11_SSID dot11Ssid;
    DOT11_BSS_TYPE dot11BssType;
    DOT11_MAC_ADDRESS dot11Bssid;
    DOT11_PHY_TYPE dot11PhyType;
    ULONG uDot11PhyIndex;
    WLAN_SIGNAL_QUALITY wlanSignalQuality;
    ULONG ulRxRate;
    ULONG ulTxRate;
} WLAN_ASSOCIATION_ATTRIBUTES, *PWLAN_ASSOCIATION_ATTRIBUTES;

typedef struct _WLAN_SECURITY_ATTRIBUTES {
    BOOL bSecurityEnabled;
    BOOL bOneXEnabled;
    DOT11_AUTH_ALGORITHM dot11AuthAlgorithm;
    DOT11_CIPHER_ALGORITHM dot11CipherAlgorithm;
} WLAN_SECURITY_ATTRIBUTES, *PWLAN_SECURITY_ATTRIBUTES;

typedef struct _WLAN_QOS_CAPABILITIES {
    BOOL bMSCSSupported;
    BOOL bDSCPToUPMappingSupported;
    BOOL bSCSSupported;
    BOOL bDSCPPolicySupported;
} WLAN_QOS_CAPABILITIES, *PWLAN_QOS_CAPABILITIES;

typedef struct _WLAN_CONNECTION_QOS_INFO {
    WLAN_QOS_CAPABILITIES peerCapabilities;
    BOOL bMSCSConfigured;
    BOOL bDSCPToUPMappingConfigured;
    ULONG ulNumConfiguredSCSStreams;
    ULONG ulNumConfiguredDSCPPolicies;
} WLAN_CONNECTION_QOS_INFO, *PWLAN_CONNECTION_QOS_INFO;

typedef struct _WLAN_QOS_INFO {
    // QoS capabilities of interface 
    WLAN_QOS_CAPABILITIES interfaceCapabilities;
    // bConnected indicates whether or not there is an established connection and therefore whether the connection
    // QoS info is present in connectionQoSInfo.
    BOOL bConnected;
    // QoS info of the current connection. Meaningful only if bConnected is true; otherwise, if bConnected is false, 
    // connectionQoSInfo will be zeroed and should be ignored.
    WLAN_CONNECTION_QOS_INFO connectionQoSInfo;
} WLAN_QOS_INFO, *PWLAN_QOS_INFO;

// structure WLAN_CONNECTION_ATTRIBUTES defines attributes of a wireless connection
typedef struct _WLAN_CONNECTION_ATTRIBUTES {
    WLAN_INTERFACE_STATE isState;
    WLAN_CONNECTION_MODE wlanConnectionMode;
    WCHAR strProfileName[WLAN_MAX_NAME_LENGTH];
    WLAN_ASSOCIATION_ATTRIBUTES wlanAssociationAttributes;
    WLAN_SECURITY_ATTRIBUTES wlanSecurityAttributes;
} WLAN_CONNECTION_ATTRIBUTES, *PWLAN_CONNECTION_ATTRIBUTES;

typedef struct _WLAN_REALTIME_CONNECTION_QUALITY_LINK_INFO
{
    UCHAR ucLinkID;
    ULONG ulChannelCenterFrequencyMhz;
    ULONG ulBandwidth;
    LONG lRssi;
    WLAN_RATE_SET wlanRateSet;
} WLAN_REALTIME_CONNECTION_QUALITY_LINK_INFO, *PWLAN_REALTIME_CONNECTION_QUALITY_LINK_INFO;

typedef struct _WLAN_REALTIME_CONNECTION_QUALITY {
    DOT11_PHY_TYPE dot11PhyType;
    ULONG ulLinkQuality;
    ULONG ulRxRate;
    ULONG ulTxRate;
    BOOL bIsMLOConnection;
    ULONG ulNumLinks;
    // Array of size ulNumLinks
    WLAN_REALTIME_CONNECTION_QUALITY_LINK_INFO linksInfo[1];
} WLAN_REALTIME_CONNECTION_QUALITY, *PWLAN_REALTIME_CONNECTION_QUALITY;

// use the 4-byte enum
#ifdef __midl
typedef [v1_enum] enum _DOT11_RADIO_STATE {
#else
typedef enum _DOT11_RADIO_STATE {
#endif
    dot11_radio_state_unknown = 0,
    dot11_radio_state_on,
    dot11_radio_state_off
} DOT11_RADIO_STATE, *PDOT11_RADIO_STATE;

// the maximum number of PHYs supported by a NIC
#define WLAN_MAX_PHY_INDEX 64

typedef struct _WLAN_PHY_RADIO_STATE {
    DWORD dwPhyIndex;
    DOT11_RADIO_STATE dot11SoftwareRadioState;
    DOT11_RADIO_STATE dot11HardwareRadioState;
} WLAN_PHY_RADIO_STATE, *PWLAN_PHY_RADIO_STATE;

typedef struct _WLAN_RADIO_STATE {
    DWORD dwNumberOfPhys;
    WLAN_PHY_RADIO_STATE PhyRadioState[WLAN_MAX_PHY_INDEX];
} WLAN_RADIO_STATE, *PWLAN_RADIO_STATE;

// use the 4-byte enum
#ifdef __midl
typedef [v1_enum] enum _WLAN_OPERATIONAL_STATE {
#else
typedef enum _WLAN_OPERATIONAL_STATE {
#endif
    wlan_operational_state_unknown = 0,
    wlan_operational_state_off,
    wlan_operational_state_on,
    wlan_operational_state_going_off,
    wlan_operational_state_going_on
} WLAN_OPERATIONAL_STATE, *PWLAN_OPERATIONAL_STATE;

typedef enum _WLAN_INTERFACE_TYPE {
    wlan_interface_type_emulated_802_11 = 0,
    wlan_interface_type_native_802_11,
    wlan_interface_type_invalid
} WLAN_INTERFACE_TYPE, *PWLAN_INTERFACE_TYPE;

typedef struct _WLAN_INTERFACE_CAPABILITY {
    WLAN_INTERFACE_TYPE interfaceType;
    BOOL bDot11DSupported;
    DWORD dwMaxDesiredSsidListSize;
    DWORD dwMaxDesiredBssidListSize;
    DWORD dwNumberOfSupportedPhys;
    DOT11_PHY_TYPE dot11PhyTypes[WLAN_MAX_PHY_INDEX];
} WLAN_INTERFACE_CAPABILITY, *PWLAN_INTERFACE_CAPABILITY;


typedef struct _WLAN_AUTH_CIPHER_PAIR_LIST {
    DWORD dwNumberOfItems;
#ifdef __midl
    [unique, size_is(dwNumberOfItems)] DOT11_AUTH_CIPHER_PAIR pAuthCipherPairList[*];
#else
    DOT11_AUTH_CIPHER_PAIR pAuthCipherPairList[1];
#endif
} WLAN_AUTH_CIPHER_PAIR_LIST, *PWLAN_AUTH_CIPHER_PAIR_LIST;

typedef struct _WLAN_COUNTRY_OR_REGION_STRING_LIST {
    DWORD dwNumberOfItems;
#ifdef __midl
    [unique, size_is(dwNumberOfItems)] DOT11_COUNTRY_OR_REGION_STRING pCountryOrRegionStringList[*];
#else
    DOT11_COUNTRY_OR_REGION_STRING pCountryOrRegionStringList[1];
#endif
} WLAN_COUNTRY_OR_REGION_STRING_LIST, *PWLAN_COUNTRY_OR_REGION_STRING_LIST;

typedef struct _WLAN_PROFILE_INFO_LIST {
    DWORD dwNumberOfItems;
    DWORD dwIndex;

#ifdef __midl
    [unique, size_is(dwNumberOfItems)] WLAN_PROFILE_INFO ProfileInfo[*];
#else
    WLAN_PROFILE_INFO ProfileInfo[1];
#endif

} WLAN_PROFILE_INFO_LIST, *PWLAN_PROFILE_INFO_LIST;


typedef struct _WLAN_AVAILABLE_NETWORK_LIST {
    DWORD dwNumberOfItems;
    DWORD dwIndex;

#ifdef __midl
    [unique, size_is(dwNumberOfItems)] WLAN_AVAILABLE_NETWORK Network[*];
#else
    WLAN_AVAILABLE_NETWORK Network[1];
#endif

} WLAN_AVAILABLE_NETWORK_LIST, *PWLAN_AVAILABLE_NETWORK_LIST;

typedef struct _WLAN_AVAILABLE_NETWORK_LIST_V2 {
    DWORD dwNumberOfItems;
    DWORD dwIndex;

#ifdef __midl
    [unique, size_is(dwNumberOfItems)] WLAN_AVAILABLE_NETWORK_V2 Network[*];
#else
    WLAN_AVAILABLE_NETWORK_V2 Network[1];
#endif

} WLAN_AVAILABLE_NETWORK_LIST_V2, *PWLAN_AVAILABLE_NETWORK_LIST_V2;


typedef struct _WLAN_INTERFACE_INFO_LIST {
    DWORD dwNumberOfItems;
    DWORD dwIndex;

#ifdef __midl
    [unique, size_is(dwNumberOfItems)] WLAN_INTERFACE_INFO InterfaceInfo[*];
#else
    WLAN_INTERFACE_INFO InterfaceInfo[1];
#endif

} WLAN_INTERFACE_INFO_LIST, *PWLAN_INTERFACE_INFO_LIST;

// network list
typedef struct _DOT11_NETWORK_LIST {
    DWORD dwNumberOfItems;
    DWORD dwIndex;

#ifdef __midl
    [unique, size_is(dwNumberOfItems)] DOT11_NETWORK Network[*];
#else
    _Field_size_(dwNumberOfItems) DOT11_NETWORK Network[1];
#endif
} DOT11_NETWORK_LIST, *PDOT11_NETWORK_LIST;

// power settings
#ifdef __midl
// use the 4-byte enum
typedef [v1_enum] enum _WLAN_POWER_SETTING {
#else
typedef enum _WLAN_POWER_SETTING {
#endif
    wlan_power_setting_no_saving = 0,
    wlan_power_setting_low_saving,
    wlan_power_setting_medium_saving,
    wlan_power_setting_maximum_saving,
    wlan_power_setting_invalid
} WLAN_POWER_SETTING, *PWLAN_POWER_SETTING;

// Wlan connection flags used in WLAN_CONNECTION_PARAMETERS
// The network to be connected is a hidden network
// This flag cannnot be set if the network to connected is an ad hoc network
#define WLAN_CONNECTION_HIDDEN_NETWORK      0x00000001
// Only join an ad hoc network, do not form it if it doesn't exist
// This flag cannnot be set if the network to connected is not an ad hoc network
#define WLAN_CONNECTION_ADHOC_JOIN_ONLY     0x00000002
// Ignore the privacy bit for the association. This is used to support easy config.
// This flag is valid only for wlan_connection_mode_temporary_profile and infrastructure networks.
#define WLAN_CONNECTION_IGNORE_PRIVACY_BIT  0x00000004
// Exempt EAPOL traffic from encryption/decryption. This is used to
// support an application that needs to send EAPOL traffic in non-802.1x WEP
// networks. This flag is valid only for wlan_connection_mode_temporary_profile
// in infrastructure networks when using authentication algorithm Open and
// Cipher WEP with 802.1x disabled
#define WLAN_CONNECTION_EAPOL_PASSTHROUGH   0x00000008
// Automatically persist discovery profile on successful connection completion.
// This flag is only valid for wlan_connection_mode_discovery_secure or
// wlan_connection_mode_discovery_unsecure. The profile will be saved as an all 
// user profile, with the name generated from the SSID using WlanUtf8SsidToDisplayName. 
// If there is already a profile with the same name, a number will be appended 
// to the end of the profile name. The profile will be saved with manual connection mode,
// unless WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE_CONNECTION_MODE_AUTO is also specified.
#define WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE 0x00000010
// To be used in conjunction with WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE. The 
// discovery profile will be persisted with automatic connection mode.
#define WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE_CONNECTION_MODE_AUTO 0x00000020
// To be used in conjunction with WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE. The 
// discovery profile will be persisted and attempt to overwrite an existing profile
// with the same name.
#define WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE_OVERWRITE_EXISTING 0x00000040

// connection parameters
typedef struct _WLAN_CONNECTION_PARAMETERS {
    WLAN_CONNECTION_MODE wlanConnectionMode;
#ifdef __midl
    [string] LPCWSTR strProfile;
#else
    LPCWSTR strProfile;
#endif
    PDOT11_SSID pDot11Ssid;
    PDOT11_BSSID_LIST pDesiredBssidList;
    DOT11_BSS_TYPE dot11BssType;
    DWORD dwFlags;
} WLAN_CONNECTION_PARAMETERS, *PWLAN_CONNECTION_PARAMETERS;

typedef struct _WLAN_CONNECTION_PARAMETERS_V2 {
    WLAN_CONNECTION_MODE wlanConnectionMode;
#ifdef __midl
    [string] LPCWSTR strProfile;
#else
    LPCWSTR strProfile;
#endif
    PDOT11_SSID pDot11Ssid;
    PDOT11_HESSID pDot11Hessid;
    PDOT11_BSSID_LIST pDesiredBssidList;
    DOT11_BSS_TYPE dot11BssType;
    DWORD dwFlags;
    PDOT11_ACCESSNETWORKOPTIONS pDot11AccessNetworkOptions;
} WLAN_CONNECTION_PARAMETERS_V2, *PWLAN_CONNECTION_PARAMETERS_V2;

// data structure for connection-related notifications.
typedef struct _WLAN_MSM_NOTIFICATION_DATA {
    WLAN_CONNECTION_MODE wlanConnectionMode;
    WCHAR strProfileName[WLAN_MAX_NAME_LENGTH];
    DOT11_SSID dot11Ssid;
    DOT11_BSS_TYPE dot11BssType;
    DOT11_MAC_ADDRESS dot11MacAddr;
    BOOL bSecurityEnabled;
    BOOL bFirstPeer;
    BOOL bLastPeer;
    WLAN_REASON_CODE wlanReasonCode;
} WLAN_MSM_NOTIFICATION_DATA, *PWLAN_MSM_NOTIFICATION_DATA;


// flags for connection notifications
// whether an adhoc network is formed or joined
#define WLAN_CONNECTION_NOTIFICATION_ADHOC_NETWORK_FORMED    0x00000001     // Formed ad hoc network
#define WLAN_CONNECTION_NOTIFICATION_CONSOLE_USER_PROFILE    0x00000004     // The profile is the active console user's per user profile

typedef struct _WLAN_CONNECTION_NOTIFICATION_DATA {
    WLAN_CONNECTION_MODE wlanConnectionMode;
    WCHAR strProfileName[WLAN_MAX_NAME_LENGTH];
    DOT11_SSID dot11Ssid;
    DOT11_BSS_TYPE dot11BssType;
    BOOL bSecurityEnabled;
    WLAN_REASON_CODE wlanReasonCode;
    DWORD dwFlags;
    WCHAR strProfileXml[1];
} WLAN_CONNECTION_NOTIFICATION_DATA, *PWLAN_CONNECTION_NOTIFICATION_DATA;

// data structure for device service notifications.
typedef struct _WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    GUID DeviceService;
    DWORD dwOpCode;
    DWORD dwDataSize;
#ifdef __midl
    [unique, size_is(dwDataSize)] BYTE DataBlob[*];
#else
    BYTE DataBlob[1];
#endif
} WLAN_DEVICE_SERVICE_NOTIFICATION_DATA, *PWLAN_DEVICE_SERVICE_NOTIFICATION_DATA;

// the types of notification
// compatible with L2_NOTIFICATION_SOURCE
#define WLAN_NOTIFICATION_SOURCE_NONE           L2_NOTIFICATION_SOURCE_NONE
#define WLAN_NOTIFICATION_SOURCE_ALL            L2_NOTIFICATION_SOURCE_ALL

#define WLAN_NOTIFICATION_SOURCE_ACM            L2_NOTIFICATION_SOURCE_WLAN_ACM
#define WLAN_NOTIFICATION_SOURCE_MSM            L2_NOTIFICATION_SOURCE_WLAN_MSM
#define WLAN_NOTIFICATION_SOURCE_SECURITY       L2_NOTIFICATION_SOURCE_WLAN_SECURITY
#define WLAN_NOTIFICATION_SOURCE_IHV            L2_NOTIFICATION_SOURCE_WLAN_IHV
#define WLAN_NOTIFICATION_SOURCE_HNWK           L2_NOTIFICATION_SOURCE_WLAN_HNWK
#define WLAN_NOTIFICATION_SOURCE_ONEX           L2_NOTIFICATION_SOURCE_ONEX
#define WLAN_NOTIFICATION_SOURCE_DEVICE_SERVICE L2_NOTIFICATION_SOURCE_WLAN_DEVICE_SERVICE

#ifdef __midl
// use the 4-byte enum
typedef [v1_enum] enum _WLAN_NOTIFICATION_ACM {
#else
typedef enum _WLAN_NOTIFICATION_ACM {
#endif
    wlan_notification_acm_start = L2_NOTIFICATION_CODE_PUBLIC_BEGIN,
    wlan_notification_acm_autoconf_enabled,
    wlan_notification_acm_autoconf_disabled,
    wlan_notification_acm_background_scan_enabled,
    wlan_notification_acm_background_scan_disabled,
    wlan_notification_acm_bss_type_change,
    wlan_notification_acm_power_setting_change,
    wlan_notification_acm_scan_complete,
    wlan_notification_acm_scan_fail,
    wlan_notification_acm_connection_start,
    wlan_notification_acm_connection_complete,
    wlan_notification_acm_connection_attempt_fail,
    wlan_notification_acm_filter_list_change,
    wlan_notification_acm_interface_arrival,
    wlan_notification_acm_interface_removal,
    wlan_notification_acm_profile_change,
    wlan_notification_acm_profile_name_change,
    wlan_notification_acm_profiles_exhausted,
    wlan_notification_acm_network_not_available,
    wlan_notification_acm_network_available,
    wlan_notification_acm_disconnecting,
    wlan_notification_acm_disconnected,
    wlan_notification_acm_adhoc_network_state_change,
    wlan_notification_acm_profile_unblocked,
    wlan_notification_acm_screen_power_change,
    wlan_notification_acm_profile_blocked,
    wlan_notification_acm_scan_list_refresh,
    wlan_notification_acm_operational_state_change,
    wlan_notification_acm_end
} WLAN_NOTIFICATION_ACM, *PWLAN_NOTIFICATION_ACM;


#ifdef __midl
// use the 4-byte enum
typedef [v1_enum] enum _WLAN_NOTIFICATION_MSM {
#else
typedef enum _WLAN_NOTIFICATION_MSM {
#endif
    wlan_notification_msm_start = L2_NOTIFICATION_CODE_PUBLIC_BEGIN,
    wlan_notification_msm_associating,
    wlan_notification_msm_associated,
    wlan_notification_msm_authenticating,
    wlan_notification_msm_connected,
    wlan_notification_msm_roaming_start,
    wlan_notification_msm_roaming_end,
    wlan_notification_msm_radio_state_change,
    wlan_notification_msm_signal_quality_change,
    wlan_notification_msm_disassociating,
    wlan_notification_msm_disconnected,
    wlan_notification_msm_peer_join,
    wlan_notification_msm_peer_leave,
    wlan_notification_msm_adapter_removal,
    wlan_notification_msm_adapter_operation_mode_change,
    wlan_notification_msm_link_degraded,
    wlan_notification_msm_link_improved,
    wlan_notification_msm_end
} WLAN_NOTIFICATION_MSM, *PWLAN_NOTIFICATION_MSM;


#ifdef __midl
// use the 4-byte enum
typedef [v1_enum] enum _WLAN_NOTIFICATION_SECURITY {
#else
typedef enum _WLAN_NOTIFICATION_SECURITY {
#endif
    wlan_notification_security_start = L2_NOTIFICATION_CODE_PUBLIC_BEGIN,
    wlan_notification_security_end
} WLAN_NOTIFICATION_SECURITY, *PWLAN_NOTIFICATION_SECURITY;

typedef L2_NOTIFICATION_DATA WLAN_NOTIFICATION_DATA, *PWLAN_NOTIFICATION_DATA;

// the callback function for notifications
typedef VOID (WINAPI *WLAN_NOTIFICATION_CALLBACK) (PWLAN_NOTIFICATION_DATA, PVOID);

#ifdef __midl
// use the 4-byte enum
typedef [v1_enum] enum _WLAN_OPCODE_VALUE_TYPE {
#else
typedef enum _WLAN_OPCODE_VALUE_TYPE {
#endif
        wlan_opcode_value_type_query_only = 0,
        wlan_opcode_value_type_set_by_group_policy,
        wlan_opcode_value_type_set_by_user,
        wlan_opcode_value_type_invalid
} WLAN_OPCODE_VALUE_TYPE, *PWLAN_OPCODE_VALUE_TYPE;

// OpCodes for set/query interfaces
#ifdef __midl
// use the 4-byte enum
typedef [v1_enum] enum _WLAN_INTF_OPCODE {
#else
typedef enum _WLAN_INTF_OPCODE {
#endif
    wlan_intf_opcode_autoconf_start = 0x000000000,
    wlan_intf_opcode_autoconf_enabled,
    wlan_intf_opcode_background_scan_enabled,
    wlan_intf_opcode_media_streaming_mode,
    wlan_intf_opcode_radio_state,
    wlan_intf_opcode_bss_type,
    wlan_intf_opcode_interface_state,
    wlan_intf_opcode_current_connection,
    wlan_intf_opcode_channel_number,
    wlan_intf_opcode_supported_infrastructure_auth_cipher_pairs,
    wlan_intf_opcode_supported_adhoc_auth_cipher_pairs,
    wlan_intf_opcode_supported_country_or_region_string_list,
    wlan_intf_opcode_current_operation_mode,
    wlan_intf_opcode_supported_safe_mode,
    wlan_intf_opcode_certified_safe_mode,
    wlan_intf_opcode_hosted_network_capable,
    wlan_intf_opcode_management_frame_protection_capable,
    wlan_intf_opcode_secondary_sta_interfaces,
    wlan_intf_opcode_secondary_sta_synchronized_connections,
    wlan_intf_opcode_realtime_connection_quality,
    wlan_intf_opcode_qos_info,
    wlan_intf_opcode_autoconf_end = 0x0fffffff,
    wlan_intf_opcode_msm_start = 0x10000100,
    wlan_intf_opcode_statistics,
    wlan_intf_opcode_rssi,
    wlan_intf_opcode_msm_end = 0x1fffffff,
    wlan_intf_opcode_security_start = 0x20010000,
    wlan_intf_opcode_security_end = 0x2fffffff,
    wlan_intf_opcode_ihv_start = 0x30000000,
    wlan_intf_opcode_ihv_end = 0x3fffffff
} WLAN_INTF_OPCODE, *PWLAN_INTF_OPCODE;


// OpCodes for set/query auto config parameters
#ifdef __midl
// use the 4-byte enum
typedef [v1_enum] enum _WLAN_AUTOCONF_OPCODE {
#else
typedef enum _WLAN_AUTOCONF_OPCODE {
#endif
    wlan_autoconf_opcode_start = 0,
    wlan_autoconf_opcode_show_denied_networks,
    wlan_autoconf_opcode_power_setting,
    wlan_autoconf_opcode_only_use_gp_profiles_for_allowed_networks,
    wlan_autoconf_opcode_allow_explicit_creds,
    wlan_autoconf_opcode_block_period,
    wlan_autoconf_opcode_allow_virtual_station_extensibility,
    wlan_autoconf_opcode_end
} WLAN_AUTOCONF_OPCODE, *PWLAN_AUTOCONF_OPCODE;


// IHV control types
#ifdef __midl
// use the 4-byte enum
typedef [v1_enum] enum _WLAN_IHV_CONTROL_TYPE {
#else
typedef enum _WLAN_IHV_CONTROL_TYPE {
#endif
    wlan_ihv_control_type_service,
    wlan_ihv_control_type_driver
} WLAN_IHV_CONTROL_TYPE, *PWLAN_IHV_CONTROL_TYPE;

typedef enum _WLAN_FILTER_LIST_TYPE {
    wlan_filter_list_type_gp_permit,
    wlan_filter_list_type_gp_deny,
    wlan_filter_list_type_user_permit,
    wlan_filter_list_type_user_deny
} WLAN_FILTER_LIST_TYPE, *PWLAN_FILTER_LIST_TYPE;

// Driver statistics
typedef struct WLAN_PHY_FRAME_STATISTICS {
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
} WLAN_PHY_FRAME_STATISTICS, * PWLAN_PHY_FRAME_STATISTICS;

typedef struct WLAN_MAC_FRAME_STATISTICS {
    ULONGLONG ullTransmittedFrameCount;
    ULONGLONG ullReceivedFrameCount;
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
} WLAN_MAC_FRAME_STATISTICS, * PWLAN_MAC_FRAME_STATISTICS;

typedef struct WLAN_STATISTICS {
    ULONGLONG ullFourWayHandshakeFailures;
    ULONGLONG ullTKIPCounterMeasuresInvoked;
    ULONGLONG ullReserved;
    WLAN_MAC_FRAME_STATISTICS MacUcastCounters;
    WLAN_MAC_FRAME_STATISTICS MacMcastCounters;
    DWORD dwNumberOfPhys;
#ifdef __midl
    [unique, size_is(dwNumberOfPhys)] WLAN_PHY_FRAME_STATISTICS PhyCounters[*];
#else
    WLAN_PHY_FRAME_STATISTICS PhyCounters[1];
#endif
} WLAN_STATISTICS, * PWLAN_STATISTICS;

// API protection settings

// Definition of access masks for setting non-default security
// settings on WLAN configuration objects and connection profiles.

#define WLAN_READ_ACCESS    ( STANDARD_RIGHTS_READ | FILE_READ_DATA )
#define WLAN_EXECUTE_ACCESS ( WLAN_READ_ACCESS | STANDARD_RIGHTS_EXECUTE | FILE_EXECUTE )
#define WLAN_WRITE_ACCESS   ( WLAN_READ_ACCESS | WLAN_EXECUTE_ACCESS | STANDARD_RIGHTS_WRITE | FILE_WRITE_DATA | DELETE | WRITE_DAC )


typedef enum
_WLAN_SECURABLE_OBJECT
{
    wlan_secure_permit_list = 0,
    wlan_secure_deny_list,
    wlan_secure_ac_enabled,
    wlan_secure_bc_scan_enabled,
    wlan_secure_bss_type,
    wlan_secure_show_denied,
    wlan_secure_interface_properties,
    wlan_secure_ihv_control,
    wlan_secure_all_user_profiles_order,
    wlan_secure_add_new_all_user_profiles,
    wlan_secure_add_new_per_user_profiles,
    wlan_secure_media_streaming_mode_enabled,
    wlan_secure_current_operation_mode,
    wlan_secure_get_plaintext_key,
    wlan_secure_hosted_network_elevated_access,
    wlan_secure_virtual_station_extensibility,
    wlan_secure_wfd_elevated_access,

    WLAN_SECURABLE_OBJECT_COUNT
}
WLAN_SECURABLE_OBJECT, *PWLAN_SECURABLE_OBJECT;

// Data structure used for querying supported WLAN Device Services
typedef struct _WLAN_DEVICE_SERVICE_GUID_LIST {
    DWORD dwNumberOfItems;
    DWORD dwIndex;

#ifdef __midl
    [unique, size_is(dwNumberOfItems)] GUID DeviceService[*];
#else
    GUID DeviceService[1];
#endif

} WLAN_DEVICE_SERVICE_GUID_LIST, *PWLAN_DEVICE_SERVICE_GUID_LIST;

//
// WiFi Direct Definitions
//
#define WFD_API_VERSION_1_0    0x00000001

#if ( _WIN32_WINNT >= _WIN32_WINNT_WIN8 )
    #define WFD_API_SUPPORTED
    #define WFD_API_VERSION WFD_API_VERSION_1_0
#endif

#ifdef WFD_API_SUPPORTED

typedef enum _WFD_ROLE_TYPE {
    WFD_ROLE_TYPE_NONE = 0x00,
    WFD_ROLE_TYPE_DEVICE = 0x01,
    WFD_ROLE_TYPE_GROUP_OWNER = 0x02,
    WFD_ROLE_TYPE_CLIENT = 0x04,
    WFD_ROLE_TYPE_MAX = 0x05
} WFD_ROLE_TYPE,  *PWFD_ROLE_TYPE;

#endif //WFD_API_SUPPORTED

typedef struct _WFD_GROUP_ID
{
    DOT11_MAC_ADDRESS   DeviceAddress;
    DOT11_SSID          GroupSSID;
} WFD_GROUP_ID, *PWFD_GROUP_ID;


// public APIs
DWORD WINAPI
WlanOpenHandle(
    _In_ DWORD dwClientVersion,
    _Reserved_ PVOID pReserved,
    _Out_ PDWORD pdwNegotiatedVersion,
    _Out_ PHANDLE phClientHandle
);

DWORD WINAPI
WlanCloseHandle(
    _In_ HANDLE hClientHandle,
    _Reserved_ PVOID pReserved
);

DWORD WINAPI
WlanEnumInterfaces(
    _In_ HANDLE hClientHandle,
    _Reserved_ PVOID pReserved,
    _Outptr_ PWLAN_INTERFACE_INFO_LIST *ppInterfaceList
);

DWORD WINAPI
WlanSetAutoConfigParameter(
    _In_ HANDLE hClientHandle,
    _In_ WLAN_AUTOCONF_OPCODE OpCode,
    _In_ DWORD dwDataSize,
    _In_reads_bytes_(dwDataSize) CONST PVOID pData,
    _Reserved_ PVOID pReserved
);

DWORD WINAPI
WlanQueryAutoConfigParameter(
    _In_ HANDLE hClientHandle,
    _In_ WLAN_AUTOCONF_OPCODE OpCode,
    _Reserved_ PVOID pReserved,
    _Out_ PDWORD pdwDataSize,
    _Outptr_result_bytebuffer_(*pdwDataSize) PVOID *ppData,
    _Out_opt_ PWLAN_OPCODE_VALUE_TYPE pWlanOpcodeValueType
);

DWORD WINAPI
WlanGetInterfaceCapability(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _Reserved_ PVOID pReserved,
    _Outptr_ PWLAN_INTERFACE_CAPABILITY *ppCapability
);

DWORD WINAPI
WlanSetInterface(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_ WLAN_INTF_OPCODE OpCode,
    _In_ DWORD dwDataSize,
    _In_reads_bytes_(dwDataSize) CONST PVOID pData,
    _Reserved_ PVOID pReserved
);

DWORD WINAPI
WlanQueryInterface(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_ WLAN_INTF_OPCODE OpCode,
    _Reserved_ PVOID pReserved,
    _Out_ PDWORD pdwDataSize,
    _Outptr_result_bytebuffer_(*pdwDataSize) PVOID *ppData,
    _Out_opt_ PWLAN_OPCODE_VALUE_TYPE pWlanOpcodeValueType
);

DWORD WINAPI
WlanIhvControl(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_ WLAN_IHV_CONTROL_TYPE Type,
    _In_ DWORD dwInBufferSize,
    _In_reads_bytes_(dwInBufferSize) PVOID pInBuffer,
    _In_ DWORD dwOutBufferSize,
    _Inout_updates_bytes_opt_(dwOutBufferSize) PVOID pOutBuffer,
    _Out_ PDWORD pdwBytesReturned
);

DWORD WINAPI
WlanScan(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_opt_ CONST PDOT11_SSID pDot11Ssid,
    _In_opt_ CONST PWLAN_RAW_DATA pIeData,
    _Reserved_ PVOID pReserved
);

DWORD WINAPI
WlanGetAvailableNetworkList(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_ DWORD dwFlags,
    _Reserved_ PVOID pReserved,
    _Outptr_ PWLAN_AVAILABLE_NETWORK_LIST *ppAvailableNetworkList
);

DWORD WINAPI
WlanGetAvailableNetworkList2(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_ DWORD dwFlags,
    _Reserved_ PVOID pReserved,
    _Outptr_ PWLAN_AVAILABLE_NETWORK_LIST_V2 *ppAvailableNetworkList
);

DWORD WINAPI
WlanGetNetworkBssList(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_opt_ CONST PDOT11_SSID pDot11Ssid,
    _In_ DOT11_BSS_TYPE dot11BssType,
    _In_ BOOL bSecurityEnabled,
    _Reserved_ PVOID pReserved,
    _Outptr_ PWLAN_BSS_LIST *ppWlanBssList
);

DWORD WINAPI
WlanConnect(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_ CONST PWLAN_CONNECTION_PARAMETERS pConnectionParameters,
    _Reserved_ PVOID pReserved
);

DWORD WINAPI
WlanConnect2(
    _In_        HANDLE hClientHandle,
    _In_        const GUID *pInterfaceGuid,
    _In_        const PWLAN_CONNECTION_PARAMETERS_V2 pConnectionParameters,
    _Reserved_  PVOID pReserved
);

DWORD WINAPI
WlanDisconnect(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _Reserved_ PVOID pReserved
);

DWORD WINAPI
WlanRegisterNotification(
    _In_ HANDLE hClientHandle,
    _In_ DWORD dwNotifSource,
    _In_ BOOL bIgnoreDuplicate,
    _In_opt_ WLAN_NOTIFICATION_CALLBACK funcCallback,
    _In_opt_ PVOID pCallbackContext,
    _Reserved_ PVOID pReserved,
    _Out_opt_ PDWORD pdwPrevNotifSource
);


DWORD WINAPI
WlanGetProfile(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_ LPCWSTR strProfileName,
    _Reserved_ PVOID pReserved,
    _Outptr_ LPWSTR *pstrProfileXml,
    _Inout_opt_ DWORD *pdwFlags,
    _Out_opt_ DWORD *pdwGrantedAccess
);

DWORD WINAPI
WlanSetProfileEapUserData(
    _In_ HANDLE hClientHandle,
    _In_ const GUID *pInterfaceGuid,
    _In_ LPCWSTR strProfileName,
    _In_ EAP_METHOD_TYPE eapType,
    _In_ DWORD dwFlags,
    _In_ DWORD dwEapUserDataSize,
    _In_reads_bytes_opt_(dwEapUserDataSize) const LPBYTE pbEapUserData,
    _Reserved_ PVOID pReserved
);

DWORD WINAPI
WlanSetProfileEapXmlUserData(
    _In_ HANDLE hClientHandle,
    _In_ const GUID *pInterfaceGuid,
    _In_ LPCWSTR strProfileName,
    _In_ DWORD dwFlags,
    _In_ LPCWSTR strEapXmlUserData,
    _Reserved_ PVOID pReserved
);

DWORD WINAPI
WlanSetProfile(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_ DWORD dwFlags,
    _In_ LPCWSTR strProfileXml,
    _In_opt_ LPCWSTR strAllUserProfileSecurity,
    _In_ BOOL bOverwrite,
    _Reserved_ PVOID pReserved,
    _Out_ DWORD *pdwReasonCode
);

DWORD WINAPI
WlanDeleteProfile(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_ LPCWSTR strProfileName,
    _Reserved_ PVOID pReserved
);

DWORD WINAPI
WlanRenameProfile(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_ LPCWSTR strOldProfileName,
    _In_ LPCWSTR strNewProfileName,
    _Reserved_ PVOID pReserved
);

DWORD WINAPI
WlanGetProfileList(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _Reserved_ PVOID pReserved,
    _Outptr_ PWLAN_PROFILE_INFO_LIST *ppProfileList
);

DWORD WINAPI
WlanSetProfileList(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_ DWORD dwItems,
    _In_reads_(dwItems) LPCWSTR *strProfileNames,
    _Reserved_ PVOID pReserved
);

DWORD WINAPI
WlanSetProfilePosition(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_ LPCWSTR strProfileName,
    _In_ DWORD dwPosition,
    _Reserved_ PVOID pReserved
);

DWORD WINAPI
WlanSetProfileCustomUserData(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_ LPCWSTR strProfileName,
    _In_ DWORD dwDataSize,
    _In_reads_bytes_(dwDataSize) CONST PBYTE pData,
    _Reserved_ PVOID pReserved
);

DWORD WINAPI
WlanGetProfileCustomUserData(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_ LPCWSTR strProfileName,
    _Reserved_ PVOID pReserved,
    _Out_ DWORD *pdwDataSize,
    _Outptr_result_bytebuffer_(*pdwDataSize) PBYTE *ppData
);

DWORD WINAPI
WlanSetFilterList(
    _In_ HANDLE hClientHandle,
    _In_ WLAN_FILTER_LIST_TYPE wlanFilterListType,
    _In_opt_ CONST PDOT11_NETWORK_LIST pNetworkList,
    _Reserved_ PVOID pReserved
);

DWORD WINAPI
WlanGetFilterList(
    _In_ HANDLE hClientHandle,
    _In_ WLAN_FILTER_LIST_TYPE wlanFilterListType,
    _Reserved_ PVOID pReserved,
    _Outptr_ PDOT11_NETWORK_LIST *ppNetworkList
);

DWORD WINAPI
WlanSetPsdIEDataList(
    _In_ HANDLE hClientHandle,
    _In_opt_ LPCWSTR strFormat,
    _In_opt_ CONST PWLAN_RAW_DATA_LIST pPsdIEDataList,
    _Reserved_ PVOID pReserved
);

DWORD WINAPI
WlanSaveTemporaryProfile(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID* pInterfaceGuid,
    _In_ LPCWSTR strProfileName,
    _In_opt_ LPCWSTR strAllUserProfileSecurity,
    _In_ DWORD dwFlags,
    _In_ BOOL bOverWrite,
    _Reserved_ PVOID pReserved
);

DWORD WINAPI
WlanDeviceServiceCommand(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _In_ LPGUID pDeviceServiceGuid,
    _In_ DWORD dwOpCode,
    _In_ DWORD dwInBufferSize,
    _In_reads_bytes_opt_(dwInBufferSize) PVOID pInBuffer,
    _In_ DWORD dwOutBufferSize,
    _Inout_updates_bytes_opt_(dwOutBufferSize) PVOID pOutBuffer,
    _Out_ PDWORD pdwBytesReturned
);

DWORD WINAPI
WlanGetSupportedDeviceServices(
    _In_ HANDLE hClientHandle,
    _In_ CONST GUID *pInterfaceGuid,
    _Outptr_ PWLAN_DEVICE_SERVICE_GUID_LIST *ppDevSvcGuidList
);

DWORD WINAPI
WlanRegisterDeviceServiceNotification(
    _In_ HANDLE hClientHandle,
    _In_opt_ CONST PWLAN_DEVICE_SERVICE_GUID_LIST pDevSvcGuidList
);


#if !defined(__midl)

// client side APIs
DWORD WINAPI
WlanExtractPsdIEDataList(
    _In_ HANDLE hClientHandle,
    _In_ DWORD dwIeDataSize,
    _In_reads_bytes_(dwIeDataSize) CONST PBYTE pRawIeData,
    _In_ LPCWSTR strFormat,
    _Reserved_ PVOID pReserved,
    _Outptr_ PWLAN_RAW_DATA_LIST *ppPsdIEDataList
);


DWORD WINAPI
WlanReasonCodeToString(
    _In_ DWORD dwReasonCode,
    _In_ DWORD dwBufferSize,
    _In_reads_(dwBufferSize) PWCHAR pStringBuffer,
    _Reserved_ PVOID pReserved
);

PVOID WINAPI
WlanAllocateMemory(
    _In_ DWORD dwMemorySize
);

VOID WINAPI
WlanFreeMemory(
    _In_ PVOID pMemory
);

DWORD WINAPI
WlanSetSecuritySettings(
    _In_ HANDLE hClientHandle,
    _In_ WLAN_SECURABLE_OBJECT SecurableObject,
    _In_ LPCWSTR strModifiedSDDL
);

DWORD WINAPI
WlanGetSecuritySettings(
    _In_ HANDLE hClientHandle,
    _In_ WLAN_SECURABLE_OBJECT SecurableObject,
    _Out_opt_ PWLAN_OPCODE_VALUE_TYPE pValueType,
    _Outptr_ LPWSTR* pstrCurrentSDDL,
    _Out_ PDWORD pdwGrantedAccess
);

#endif


// the following structures and defs are
// for the UI related functions

// current version
#define WLAN_UI_API_VERSION         1
// earliest version supported
#define WLAN_UI_API_INITIAL_VERSION 1

// The list of pages displayed by the wireless profile UI
typedef enum _WL_DISPLAY_PAGES
{
    WLConnectionPage,
    WLSecurityPage,
    WLAdvPage
} WL_DISPLAY_PAGES, *PWL_DISPLAY_PAGES;

DWORD WINAPI
WlanUIEditProfile(
    _In_ DWORD dwClientVersion,
    _In_ LPCWSTR wstrProfileName,
    _In_ GUID *pInterfaceGuid,
    _In_ HWND hWnd,
    _In_ WL_DISPLAY_PAGES wlStartPage,
    _Reserved_ PVOID pReserved,
    _Out_opt_ PWLAN_REASON_CODE pWlanReasonCode
    );



#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)

// Hosted Network APIs

typedef
#ifdef __midl
[v1_enum]
#endif
enum _WLAN_HOSTED_NETWORK_STATE
{
    wlan_hosted_network_unavailable,
    wlan_hosted_network_idle,
    wlan_hosted_network_active,
}
WLAN_HOSTED_NETWORK_STATE, *PWLAN_HOSTED_NETWORK_STATE;




typedef
#ifdef __midl
[v1_enum]
#endif
enum _WLAN_HOSTED_NETWORK_REASON
{
    wlan_hosted_network_reason_success = 0,
    wlan_hosted_network_reason_unspecified,
    wlan_hosted_network_reason_bad_parameters,
    wlan_hosted_network_reason_service_shutting_down,
    wlan_hosted_network_reason_insufficient_resources,
    wlan_hosted_network_reason_elevation_required,
    wlan_hosted_network_reason_read_only,
    wlan_hosted_network_reason_persistence_failed,
    wlan_hosted_network_reason_crypt_error,
    wlan_hosted_network_reason_impersonation,
    wlan_hosted_network_reason_stop_before_start,

    wlan_hosted_network_reason_interface_available,
    wlan_hosted_network_reason_interface_unavailable,
    wlan_hosted_network_reason_miniport_stopped,
    wlan_hosted_network_reason_miniport_started,
    wlan_hosted_network_reason_incompatible_connection_started,
    wlan_hosted_network_reason_incompatible_connection_stopped,
    wlan_hosted_network_reason_user_action,
    wlan_hosted_network_reason_client_abort,
    wlan_hosted_network_reason_ap_start_failed,

    wlan_hosted_network_reason_peer_arrived,
    wlan_hosted_network_reason_peer_departed,
    wlan_hosted_network_reason_peer_timeout,
    wlan_hosted_network_reason_gp_denied,
    wlan_hosted_network_reason_service_unavailable,
    wlan_hosted_network_reason_device_change,
    wlan_hosted_network_reason_properties_change,
    wlan_hosted_network_reason_virtual_station_blocking_use,
    wlan_hosted_network_reason_service_available_on_virtual_station,

}
WLAN_HOSTED_NETWORK_REASON, *PWLAN_HOSTED_NETWORK_REASON;


typedef
#ifdef __midl
[v1_enum]
#endif
enum _WLAN_HOSTED_NETWORK_PEER_AUTH_STATE
{
    wlan_hosted_network_peer_state_invalid,
    wlan_hosted_network_peer_state_authenticated,
}
WLAN_HOSTED_NETWORK_PEER_AUTH_STATE, *PWLAN_HOSTED_NETWORK_PEER_AUTH_STATE;



DWORD
WINAPI
WlanHostedNetworkStartUsing
(
    _In_        HANDLE                          hClientHandle,
    _Out_opt_   PWLAN_HOSTED_NETWORK_REASON     pFailReason,
    _Reserved_  PVOID                           pvReserved
);



DWORD
WINAPI
WlanHostedNetworkStopUsing
(
    _In_        HANDLE                          hClientHandle,
    _Out_opt_   PWLAN_HOSTED_NETWORK_REASON     pFailReason,
    _Reserved_  PVOID                           pvReserved
);




DWORD
WINAPI
WlanHostedNetworkForceStart
(
    _In_        HANDLE                          hClientHandle,
    _Out_opt_   PWLAN_HOSTED_NETWORK_REASON     pFailReason,
    _Reserved_  PVOID                           pvReserved
);



DWORD
WINAPI
WlanHostedNetworkForceStop
(
    _In_        HANDLE                          hClientHandle,
    _Out_opt_   PWLAN_HOSTED_NETWORK_REASON     pFailReason,
    _Reserved_  PVOID                           pvReserved
);



typedef
struct _WLAN_HOSTED_NETWORK_PEER_STATE
{
    DOT11_MAC_ADDRESS                       PeerMacAddress;
    WLAN_HOSTED_NETWORK_PEER_AUTH_STATE     PeerAuthState;
}
WLAN_HOSTED_NETWORK_PEER_STATE, *PWLAN_HOSTED_NETWORK_PEER_STATE;



typedef
struct _WLAN_HOSTED_NETWORK_RADIO_STATE
{
    DOT11_RADIO_STATE   dot11SoftwareRadioState;
    DOT11_RADIO_STATE   dot11HardwareRadioState;
}
WLAN_HOSTED_NETWORK_RADIO_STATE, *PWLAN_HOSTED_NETWORK_RADIO_STATE;


// Definitions required for calling WlanRegisterNotification
// Notification source - L2_NOTIFICATION_SOURCE_WLAN_HNWK

// Notification code
typedef
#ifdef __midl
[v1_enum]
#endif
enum _WLAN_HOSTED_NETWORK_NOTIFICATION_CODE
{
    wlan_hosted_network_state_change = L2_NOTIFICATION_CODE_V2_BEGIN,
    wlan_hosted_network_peer_state_change,
    wlan_hosted_network_radio_state_change,
}
WLAN_HOSTED_NETWORK_NOTIFICATION_CODE, *PWLAN_HOSTED_NETWORK_NOTIFICATION_CODE;






// Notification data associated with wlan_hosted_network_state_change
typedef
struct _WLAN_HOSTED_NETWORK_STATE_CHANGE
{
    WLAN_HOSTED_NETWORK_STATE   OldState;
    WLAN_HOSTED_NETWORK_STATE   NewState;
    WLAN_HOSTED_NETWORK_REASON  StateChangeReason;
}
WLAN_HOSTED_NETWORK_STATE_CHANGE, *PWLAN_HOSTED_NETWORK_STATE_CHANGE;



// Notification data associated with wlan_hosted_network_peer_state_change
typedef
struct _WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE
{
    WLAN_HOSTED_NETWORK_PEER_STATE      OldState;
    WLAN_HOSTED_NETWORK_PEER_STATE      NewState;
    WLAN_HOSTED_NETWORK_REASON          PeerStateChangeReason;
}
WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE, *PWLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE;



// Notification data associated with wlan_hosted_network_radio_state_change
// WLAN_HOSTED_NETWORK_RADIO_STATE







// Definitions required for calling WlanHostedNetworkQueryProperty and WlanHostedNetworkSetProperty


typedef
#ifdef __midl
[v1_enum]
#endif
enum
_WLAN_HOSTED_NETWORK_OPCODE
{
    wlan_hosted_network_opcode_connection_settings,
    wlan_hosted_network_opcode_security_settings,
    wlan_hosted_network_opcode_station_profile,
    wlan_hosted_network_opcode_enable,
}
WLAN_HOSTED_NETWORK_OPCODE, *PWLAN_HOSTED_NETWORK_OPCODE;



// Data structure associated with wlan_hosted_network_opcode_connection_settings
// can be used for query and set
typedef
struct _WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS
{
    DOT11_SSID  hostedNetworkSSID;
    DWORD       dwMaxNumberOfPeers;
}
WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS, *PWLAN_HOSTED_NETWORK_CONNECTION_SETTINGS;


// Data structure associated with wlan_hosted_network_opcode_security_settings
// can be used for query only
typedef
struct _WLAN_HOSTED_NETWORK_SECURITY_SETTINGS
{
    DOT11_AUTH_ALGORITHM        dot11AuthAlgo;
    DOT11_CIPHER_ALGORITHM      dot11CipherAlgo;
}
WLAN_HOSTED_NETWORK_SECURITY_SETTINGS, *PWLAN_HOSTED_NETWORK_SECURITY_SETTINGS;



// Data structure associated with wlan_hosted_network_opcode_station_profile
// can be used for query only
// LPWSTR



// Data structure associated with wlan_hosted_network_opcode_enable
// can be used for query and set
// BOOL




//
// This function queries the static properties of the hosted network
//
DWORD
WINAPI
WlanHostedNetworkQueryProperty
(
    _In_                                HANDLE                      hClientHandle,
    _In_                                WLAN_HOSTED_NETWORK_OPCODE  OpCode,
    _Out_                               PDWORD                      pdwDataSize,
    _Outptr_result_bytebuffer_(*pdwDataSize)    PVOID*                      ppvData,
    _Out_                               PWLAN_OPCODE_VALUE_TYPE     pWlanOpcodeValueType,
    _Reserved_  PVOID                                               pvReserved
);




//
// This function sets the static properties of the hosted network
//
DWORD
WINAPI
WlanHostedNetworkSetProperty
(
    _In_                        HANDLE                          hClientHandle,
    _In_                        WLAN_HOSTED_NETWORK_OPCODE      OpCode,
    _In_                        DWORD                           dwDataSize,
    _In_reads_bytes_(dwDataSize)     PVOID                           pvData,
    _Out_opt_                   PWLAN_HOSTED_NETWORK_REASON     pFailReason,
    _Reserved_                  PVOID                           pvReserved
);



//
// This function initializes hosted network configuration
// on a machine. There are no effects if an initial
// configuration has already been created.
//
DWORD
WINAPI
WlanHostedNetworkInitSettings
(
    _In_        HANDLE                          hClientHandle,
    _Out_opt_   PWLAN_HOSTED_NETWORK_REASON     pFailReason,
    _Reserved_  PVOID                           pvReserved
);




DWORD
WINAPI
WlanHostedNetworkRefreshSecuritySettings
(
    _In_        HANDLE                          hClientHandle,
    _Out_opt_   PWLAN_HOSTED_NETWORK_REASON     pFailReason,
    _Reserved_  PVOID                           pvReserved
);


typedef
struct _WLAN_HOSTED_NETWORK_STATUS
{
    WLAN_HOSTED_NETWORK_STATE   HostedNetworkState;
    GUID                        IPDeviceID;
    DOT11_MAC_ADDRESS           wlanHostedNetworkBSSID;
    DOT11_PHY_TYPE              dot11PhyType;
    ULONG                       ulChannelFrequency;

    DWORD                       dwNumberOfPeers;
#ifdef __midl
    [unique, size_is(dwNumberOfPeers)] WLAN_HOSTED_NETWORK_PEER_STATE PeerList[*];
#else
    WLAN_HOSTED_NETWORK_PEER_STATE PeerList[1];
#endif
}
WLAN_HOSTED_NETWORK_STATUS, *PWLAN_HOSTED_NETWORK_STATUS;


//
// This function queries the runtime status of the hosted network
//
DWORD
WINAPI
WlanHostedNetworkQueryStatus
(
    _In_        HANDLE                          hClientHandle,
    _Outptr_ PWLAN_HOSTED_NETWORK_STATUS*    ppWlanHostedNetworkStatus,
    _Reserved_  PVOID                           pvReserved
);





//
// This function set the additional security key used by hosted network
// if it is passphrase, key length includes the terminating '\0',
// if not, key length is the number of bytes in the key data array.
//
DWORD
WINAPI
WlanHostedNetworkSetSecondaryKey
(
    _In_        HANDLE                          hClientHandle,
    _In_        DWORD                           dwKeyLength,
    _In_reads_bytes_(dwKeyLength) PUCHAR             pucKeyData,
    _In_        BOOL                            bIsPassPhrase,
    _In_        BOOL                            bPersistent,
    _Out_opt_   PWLAN_HOSTED_NETWORK_REASON     pFailReason,
    _Reserved_  PVOID                           pvReserved
);



//
// This function query the additional security key used by hosted network
// If it is passphrase, key length includes the terminating '\0',
// if not, key length is the number of bytes in the key data array.
//
DWORD
WINAPI
WlanHostedNetworkQuerySecondaryKey
(
    _In_        HANDLE                          hClientHandle,
    _Out_       PDWORD                          pdwKeyLength,
    _Outptr_result_buffer_(*pdwKeyLength)   PUCHAR  *ppucKeyData,
    _Out_       PBOOL                           pbIsPassPhrase,
    _Out_       PBOOL                           pbPersistent,
    _Out_opt_   PWLAN_HOSTED_NETWORK_REASON     pFailReason,
    _Reserved_  PVOID                           pvReserved
);

//
// This function is used to register and unregister notifications on virtual station.
//
DWORD
WINAPI
WlanRegisterVirtualStationNotification
(
    _In_        HANDLE hClientHandle,
    _In_        BOOL bRegister,
    _Reserved_  PVOID pReserved
);

#ifdef WFD_API_SUPPORTED

//
// Wi-Fi Direct Device Interface GUID
//
// {439B20AF-8955-405B-99F0-A62AF0C68D43}
//
// Description: Wi-Fi Direct Device Nodes will expose a device
// interface identified by this GUID.
//
DEFINE_GUID(
    GUID_DEVINTERFACE_WIFIDIRECT_DEVICE,
    0x439b20af, 0x8955, 0x405b, 0x99, 0xf0, 0xa6, 0x2a, 0xf0, 0xc6, 0x8d, 0x43
    );

//
// Wi-Fi Direct Device AEP Service Class GUID
//
// {CC29827C-9CAF-4928-99A9-18F7C2381389}
//
// Description: Wi-Fi Direct Devices will expose a default AEP Service
// identified by this GUID.
//
DEFINE_GUID(
    GUID_AEPSERVICE_WIFIDIRECT_DEVICE,
    0xcc29827c, 0x9caf, 0x4928, 0x99, 0xa9, 0x18, 0xf7, 0xc2, 0x38, 0x13, 0x89
    );

//
// ASP Infra Device Interface GUID
//
// {FF823995-7A72-4C80-8757-C67EE13D1A49}
//
// Description: Wi-Fi Direct Device Nodes will expose a device
// interface identified by this GUID.
//
DEFINE_GUID(
    GUID_DEVINTERFACE_ASP_INFRA_DEVICE,
    0xff823995, 0x7a72, 0x4c80, 0x87, 0x57, 0xc6, 0x7e, 0xe1, 0x3d, 0x1a, 0x49
    );


//
// Wi-Fi Direct Property Key Definitions
// These properties are exposed through Wi-Fi Direct Device Nodes when
// created through the inbox Windows pairing experience.
//

//
// Property: DEVPKEY_WiFiDirect_DeviceAddress
// Description: Binary representation of WFD Device's Device Address.
// Type: DEVPROP_TYPE_BINARY
// Binary Data: DOT11_MAC_ADDRESS (UCHAR[6])
// Availability: Always.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_DeviceAddress,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x01
    );

//
// Property: DEVPKEY_WiFiDirect_InterfaceAddress
// Description: Binary representation of WFD Device's Interface Address.
// Type: DEVPROP_TYPE_BINARY
// Binary Data: DOT11_MAC_ADDRESS (UCHAR[6])
// Availability: When DEVPKEY_WiFiDirect_IsConnected == DEVPROP_TRUE.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_InterfaceAddress,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x02
    );

//
// Property: DEVPKEY_WiFiDirect_InterfaceGuid
// Description: Local Network Interface GUID which hosts WFD Device's
//              current session.
// Type: DEVPROP_TYPE_GUID
// Availability: When DEVPKEY_WiFiDirect_IsConnected == DEVPROP_TRUE.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_InterfaceGuid,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x03
    );

//
// Property: DEVPKEY_WiFiDirect_GroupId
// Description: WFD Unique Group ID GUID. Each Wi-Fi Direct Group
// (SSID + GO Device Address) will correspond to a certain GroupID GUID.
// Type: DEVPROP_TYPE_GUID
// Availability: Always.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_GroupId,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x04
    );

//
// Property: DEVPKEY_WiFiDirect_IsConnected
// Description: A value indicating WFD Device's current connectivity state.
// Type: DEVPROP_TYPE_BOOLEAN
// Availability: If connected, always available and set to
//               DEVPROP_TRUE, otherwise either available and set to
//               DEVPROP_FALSE, or empty.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_IsConnected,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x05
    );

//
// Property: DEVPKEY_WiFiDirect_IsVisible
// Description: A value indicating WFD Device's current visibility.
// Type: DEVPROP_TYPE_BOOLEAN
// Availability: If visible, always available and set to DEVPROP_TRUE,
//               otherwise either available and set to DEVPROP_FALSE,
//               or empty.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_IsVisible,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x06
    );

//
// Property: DEVPKEY_WiFiDirect_IsLegacyDevice
// Description: Whether or not WFD Device is a legacy WPS device
//              connecting to the local PC.
// Type: DEVPROP_TYPE_BOOLEAN
// Availability: If Legacy, always. Otherwise DEVPROP_FALSE or empty.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_IsLegacyDevice,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x07
    );

//
// Property: DEVPKEY_WiFiDirect_MiracastVersion
// Description: A value indicating version of Miracast protocol if WFD Device is Miracast capable.
// Type: DEVPROP_TYPE_UINT32
// Availability: If Miracast capable, set to the highest version of protocol supported or empty
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_MiracastVersion,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x08
    );

//
// Property: DEVPKEY_WiFiDirect_IsMiracastLCPSupported
// Description: A value indicating if link content protection is supported by the device if WFD Device is Miracast capable.
// Type: DEVPROP_TYPE_BOOLEAN
// Availability: If link content protection is supported, set to DEVPROP_TRUE,
//               otherwise set to DEVPROP_FALSE,
//               or empty.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_IsMiracastLCPSupported,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x09
    );

//
// Property: DEVPKEY_WiFiDirect_Services
// Description: A value indicating services supported by the Wi Fi Direct device.
// Type: DEVPROP_TYPE_STRING_LIST
// Availability: If Miracast capable or advertising well-known service such as WSB, else empty
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_Services,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x0A
    );

//
// Property: DEVPKEY_WiFiDirect_SupportedChannelList
// Description: Binary representation of WFD Device's channel list attribute.
// Type: DEVPROP_TYPE_BINARY
// Binary Data: WFA P2P Spec channel list attribute value
// Availability: Optional, whenever provided by the remote device.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_SupportedChannelList,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x0B
    );

//
// Property: DEVPKEY_WiFiDirect_InformationElements
// Description: The full set of IEs provided by the Wi-Fi Direct Device
// Type: DEVPROP_TYPE_BINARY
// Binary Data: The IEs provided by the Wi-Fi Direct Device
// Availability: Always.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_InformationElements,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x0C
    );

//
// Property: DEVPKEY_WiFiDirect_DeviceAddressCopy
// Description: Binary representation of WFD Device's Device Address.
//              This is a copy of the DEVPKEY_WiFiDirect_DeviceAddress so
//               that a canonical name can be added for device address property.
// Type: DEVPROP_TYPE_BINARY
// Binary Data: DOT11_MAC_ADDRESS (UCHAR[6])
// Availability: Always.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_DeviceAddressCopy,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x0D
    );

//
// Property: DEVPKEY_WiFiDirect_IsRecentlyAssociated
// Description: A value indicating if WFD Device was recently associated
// Type: DEVPROP_TYPE_BOOLEAN
// Availability: If recently associated, set to DEVPROP_TRUE,
//               otherwise always set to DEVPROP_FALSE
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_IsRecentlyAssociated,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x0E
    );

//
// Property: DEVPKEY_WiFiDirect_Service_Aeps
// Description: A value indicating AEP Service IDs of services supported by the Wi-Fi Direct device.
// Type: DEVPROP_TYPE_STRING_LIST
// Availability: If advertising well-known service such as WSB, else empty
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_Service_Aeps,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x0F
    );

//
// Property: DEVPKEY_WiFiDirect_NoAutoProject
// Description: A value indicating if WFDSConMgr initiated the association so hotplug auto-project should not be used.
//              When true, WFDSConMgr is responsible for calling StartMiracastDisplayDevice
// Type: DEVPROP_TYPE_BOOLEAN
// Availability: If WFDSConMgr initiated the association, set to DEVPROP_TRUE,
//               otherwise set to DEVPROP_FALSE,
//               or empty.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_NoMiracastAutoProject,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x10
    );

//
// Property: DEVPKEY_InfraCast_Supported
// Description: A value indicating if the remote Miracast Sink supports infrastructure connections
// Type: DEVPROP_TYPE_BOOLEAN
// Availability: If remote device is a Miracast sink and supports infrastructure connections, then this is set to DEVPROP_TRUE,
//               otherwise set to DEVPROP_FALSE, or empty.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_InfraCast_Supported,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x11
    );

//
// Property: DEVPKEY_InfraCast_StreamSecuritySupported
// Description: A value indicating if the remote Miracast Sink supports stream security
// Type: DEVPROP_TYPE_BOOLEAN
// Availability: If remote device is a Miracast sink and supports stream security, then this is set to DEVPROP_TRUE,
//               otherwise set to DEVPROP_FALSE, or empty.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_InfraCast_StreamSecuritySupported,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x12
    );

//
// Property: DEVPKEY_InfraCast_AccessPointBssid
// Description: A value indicating the BSSID of the Access Point the Miracast Sink is connected to, if the network is secure.
// Type: DEVPROP_TYPE_BINARY
// Binary Data: DOT11_MAC_ADDRESS (UCHAR[6])
// Availability: If the Miracast Sink connection to an Access Point is secure, this value is set else is empty.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_InfraCast_AccessPointBssid,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x13
    );

//
// Property: DEVPKEY_InfraCast_SinkHostName
// Description: A value indicating the DNS hostname of the Miracast Sink.
// Type: DEVPROP_TYPE_STRING
// Availability: If the Miracast Sink supports connection over infrastructure, this value is set else is empty.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_InfraCast_SinkHostName,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x14
    );

//
// Parameter: DEVPKEY_InfraCast_ChallengeAep
// Description: Tells InfraCast DAF provider which challenge is supposed to be used next
// Type: DEVPROP_TYPE_STRING
//
DEFINE_DEVPROPKEY(
    DEVPKEY_InfraCast_ChallengeAep,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x15
    );

//
// Property: DEVPKEY_WiFiDirect_IsDMGCapable
// Description: Indicates that the device was discovered over a Directional Multi-Gigabit (802.11ad) interface
// Type: DEVPROP_TYPE_BOOLEAN
// Availability: Always
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_IsDMGCapable,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x16
    );

//
// Parameter: DEVPKEY_InfraCast_DevnodeAep
// Description: Tells InfraCast DAF provider which AEP it needs to take offline when the association is closed
// Type: DEVPROP_TYPE_STRING
//
DEFINE_DEVPROPKEY(
    DEVPKEY_InfraCast_DevnodeAep,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x17
    );

//
// Property: DEVPKEY_WiFiDirect_FoundWsbService
// Description: A value indicating if this device was found with the WSB service. Persisted in the AEP store
//              so that challenges can add the WSB hash to the query if the device previously had the WSB service.
// Type: DEVPROP_TYPE_BOOLEAN
// Availability: If WSB service was found during discovery, set to DEVPROP_TRUE,
//               otherwise set to DEVPROP_FALSE,
//               or empty.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_FoundWsbService,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x18
    );

//
// Property: DEVPKEY_InfraCast_HostName_ResolutionMode
// Description: String representation of InfraCast Sink HostName resolution mode (Valid Values: Unknown, mDNS, DNS)
// Type: DEVPROP_TYPE_STRING
// Availability: Always.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_InfraCast_HostName_ResolutionMode,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x19
);

//
// Property: DEVPKEY_InfraCast_SinkIpAddress
// Description: A value containing an IPv4 or IPV6 IP address of the Miracast Sink.
// Type: DEVPROP_TYPE_STRING
// Availability: If the Miracast Sink supports connection over infrastructure, this value maybe set, else is empty.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_InfraCast_SinkIpAddress,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x1A
    );

//
// Property: DEVPKEY_WiFiDirect_TransientAssociation
// Description: Set when the association should not be persisted. Tells the DAF provider to remove the association
//              when the devnode goes offline
// Type: DEVPROP_TYPE_BOOLEAN
// Availability: If the pairing result indicated that the persistent group is not supported, set to DEVPROP_TRUE,
//               otherwise set to DEVPROP_FALSE,
//               or empty.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_TransientAssociation,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x1B
    );

//
// Property: DEVPKEY_WiFiDirect_LinkQuality
// Description: The signal quality value ranging from 0 through 100. A value of 100 specifies the highest link quality.
// Type: DEVPROP_TYPE_UINT32
// Availability: Always
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_LinkQuality,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x1C
    );

//
// Property: DEVPKEY_InfraCast_PinSupported
// Description: A value indicating if the remote Miracast over Infrastructure Sink supports PIN entry
// Type: DEVPROP_TYPE_BOOLEAN
// Availability: If remote device is a Miracast sink and supports Infrastructure and PIN entry, then this is set to DEVPROP_TRUE,
//               otherwise set to DEVPROP_FALSE, or empty.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_InfraCast_PinSupported,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x1D
    );

//
// Property: DEVPKEY_InfraCast_RtspTcpConnectionParametersSupported
// Description: A value indicating if the remote Miracast over Infrastructure Sink supports changing the RTSP TCP connection direction.
// Type: DEVPROP_TYPE_BOOLEAN
// Availability: If remote device is a Miracast sink and supports Infrastructure and changing the RTSP TCP connection direction, then this
//               is set to DEVPROP_TRUE, otherwise set to DEVPROP_FALSE, or empty.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_InfraCast_RtspTcpConnectionParametersSupported,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x1E
    );

//
// Property: DEVPKEY_WiFiDirect_Miracast_SessionMgmtControlPort
// Description: The port a Miracast device will use for the RTSP session.
// Type: DEVPROP_TYPE_UINT32
// Availability: If Miracast capable, set to the value of the Session Management Control Port in the WFD Device Information sublement.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_Miracast_SessionMgmtControlPort,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x1F
    );

//
// Property: DEVPKEY_WiFiDirect_RtspTcpConnectionParametersSupported
// Description: A value indicating if the remote Miracast Sink supports changing the RTSP TCP connection direction.
// Type: DEVPROP_TYPE_BOOLEAN
// Availability: If remote device is a Miracast sink and supports changing the RTSP TCP connection direction, then this
//               is set to DEVPROP_TRUE, otherwise set to DEVPROP_FALSE, or empty.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirect_RtspTcpConnectionParametersSupported,
    0x1506935d, 0xe3e7, 0x450f, 0x86, 0x37, 0x82, 0x23, 0x3e, 0xbe, 0x5f, 0x6E,
    0x20
    );

//
// Wi-Fi Direct Services Property Key Definitions
// These properties are exposed through Wi-Fi Direct Aep service objects when
// enumerated through a DevQuery.
//

//
// Property: DEVPKEY_WiFiDirectServices_ServiceAddress
// Description: Binary representation of WFD Service's Address.
// Type: DEVPROP_TYPE_BINARY
// Binary Data: DOT11_MAC_ADDRESS (UCHAR[6])
// Availability: Always.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirectServices_ServiceAddress,
    0x31b37743, 0x7c5e, 0x4005, 0x93, 0xe6, 0xe9, 0x53, 0xf9, 0x2b, 0x82, 0xe9,
    0x02
    );

//
// Property: DEVPKEY_WiFiDirectServices_ServiceName
// Description: A value indicating name ofthe Wi Fi Direct service.
// Type: DEVPROP_TYPE_STRING
// Availability: Always
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirectServices_ServiceName,
    0x31b37743, 0x7c5e, 0x4005, 0x93, 0xe6, 0xe9, 0x53, 0xf9, 0x2b, 0x82, 0xe9,
    0x03
    );

//
// Property: DEVPKEY_WiFiDirectServices_ServiceInformation
// Description: Service information blob for the Wi Fi Direct Service.
// Type: DEVPROP_TYPE_BINARY
// Availability: Optional - returned when requested and available.
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirectServices_ServiceInformation,
    0x31b37743, 0x7c5e, 0x4005, 0x93, 0xe6, 0xe9, 0x53, 0xf9, 0x2b, 0x82, 0xe9,
    0x04
    );

//
// Property: DEVPKEY_WiFiDirectServices_AdvertisementId
// Description: Service advertisement ID for the Wi Fi Direct Service.
// Type: DEVPROP_TYPE_UINT32
// Availability: Always
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirectServices_AdvertisementId,
    0x31b37743, 0x7c5e, 0x4005, 0x93, 0xe6, 0xe9, 0x53, 0xf9, 0x2b, 0x82, 0xe9,
    0x05
    );

//
// Property: DEVPKEY_WiFiDirectServices_ServiceConfigMethods
// Description: Service advertisement ID for the Wi Fi Direct Service.
// Type: DEVPROP_TYPE_UINT32
// Availability: Always
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirectServices_ServiceConfigMethods,
    0x31b37743, 0x7c5e, 0x4005, 0x93, 0xe6, 0xe9, 0x53, 0xf9, 0x2b, 0x82, 0xe9,
    0x06
    );

//
// Property: DEVPKEY_WiFiDirectServices_RequestServiceInformation
// Description: A service information string that will be used to do a substring search in the advertiser's service information.
//              This property is only allowed as a filter in the query and is never returned for the Aep Service object.
// Type: DEVPROP_TYPE_STRING
// Availability: Optional - specified when app requested service information
//
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFiDirectServices_RequestServiceInformation,
    0x31b37743, 0x7c5e, 0x4005, 0x93, 0xe6, 0xe9, 0x53, 0xf9, 0x2b, 0x82, 0xe9,
    0x07
    );

DWORD
WINAPI
WFDOpenHandle(
    _In_ DWORD              dwClientVersion,
    _Out_ PDWORD            pdwNegotiatedVersion,
    _Out_ PHANDLE           phClientHandle
    );

DWORD
WINAPI
WFDCloseHandle(
    _In_ HANDLE             hClientHandle
    );

typedef VOID (WINAPI *WFD_OPEN_SESSION_COMPLETE_CALLBACK) (
    _In_ HANDLE         hSessionHandle,
    _In_ PVOID          pvContext,
    _In_ GUID           guidSessionInterface,
    _In_ DWORD          dwError,
    _In_ DWORD          dwReasonCode
    );

DWORD
WINAPI
WFDStartOpenSession(
    _In_ HANDLE                                 hClientHandle,
    _In_ PDOT11_MAC_ADDRESS                     pDeviceAddress,
    _In_opt_ PVOID                              pvContext,
    _In_ WFD_OPEN_SESSION_COMPLETE_CALLBACK     pfnCallback,
    _Out_ PHANDLE                               phSessionHandle
    );

DWORD
WINAPI
WFDCancelOpenSession(
    _In_ HANDLE                         hSessionHandle
    );

DWORD
WINAPI
WFDOpenLegacySession(
    _In_ HANDLE hClientHandle,
    _In_ PDOT11_MAC_ADDRESS pLegacyMacAddress,
    _Out_ HANDLE* phSessionHandle,
    _Out_ GUID* pGuidSessionInterface
    );

DWORD
WINAPI
WFDCloseSession(
    _In_ HANDLE                         hSessionHandle
    );

DWORD
WINAPI
WFDUpdateDeviceVisibility(
    _In_ PDOT11_MAC_ADDRESS pDeviceAddress
    );

#endif // WFD_API_SUPPORTED

//
// Property: DEVPKEY_WiFi_InterfaceGuid
// Description: Wi-Fi interfaces will have this value set to the Wlan Interface
//     Guid by WlanSvc
// Type: DEVPROP_TYPE_GUID
// Availability: This will be set after first time WlanSvc brings up its interfaces
// 
DEFINE_DEVPROPKEY(
    DEVPKEY_WiFi_InterfaceGuid, 
    0xef1167eb, 0xcbfc, 0x4341, 0xa5, 0x68, 0xa7, 0xc9, 0x1a, 0x68, 0x98, 0x2c,
    0x02
    );

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN7)

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // _WLAN_WLANAPI_H
