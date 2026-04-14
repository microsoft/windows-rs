/*++

Copyright (c) 2004 Microsoft Corporation

Module Name:

    l2cmn.h

Abstract:

    Definitions and data strcutures for common layer 2

Environment:

    User mode only

--*/

#ifndef _L2CMN_H
#define _L2CMN_H

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

// Profile name max length, in characters
#define L2_PROFILE_MAX_NAME_LENGTH     256

// If a notification needs to be private that is only consumed by internal components 
// then it should be part of l2cmnpriv.h 


// the types of notification
// This signifies what kind of notification is to be passed on , the 
// 0X0000FFFF  signifies all public notifications , so all notfications that need to be public 
// should be in this range and have a single bit set as these flags can be combined.
#define L2_NOTIFICATION_SOURCE_NONE                0
#define L2_NOTIFICATION_SOURCE_DOT3_AUTO_CONFIG    0X00000001
#define L2_NOTIFICATION_SOURCE_SECURITY            0X00000002
#define L2_NOTIFICATION_SOURCE_ONEX                0X00000004
#define L2_NOTIFICATION_SOURCE_WLAN_ACM            0X00000008
#define L2_NOTIFICATION_SOURCE_WLAN_MSM            0X00000010
#define L2_NOTIFICATION_SOURCE_WLAN_SECURITY       0X00000020
#define L2_NOTIFICATION_SOURCE_WLAN_IHV            0X00000040
#define L2_NOTIFICATION_SOURCE_WLAN_HNWK           0X00000080
#define L2_NOTIFICATION_SOURCE_WCM                 0X00000100
#define L2_NOTIFICATION_SOURCE_WCM_CSP             0X00000200
#define L2_NOTIFICATION_SOURCE_WFD                 0X00000400
#define L2_NOTIFICATION_SOURCE_WLAN_DEVICE_SERVICE 0X00000800

#define L2_NOTIFICATION_SOURCE_ALL                 0X0000FFFF
#define L2_NOTIFICATION_CODE_PUBLIC_BEGIN          0X00000000
#define L2_NOTIFICATION_CODE_GROUP_SIZE            0x00001000
#define L2_NOTIFICATION_CODE_V2_BEGIN              (L2_NOTIFICATION_CODE_PUBLIC_BEGIN+L2_NOTIFICATION_CODE_GROUP_SIZE)

// L2 reason code 
// reason code bases
#define L2_REASON_CODE_GROUP_SIZE             0x10000
#define L2_REASON_CODE_GEN_BASE               0x10000
#define L2_REASON_CODE_DOT11_AC_BASE          (L2_REASON_CODE_GEN_BASE+L2_REASON_CODE_GROUP_SIZE)
#define L2_REASON_CODE_DOT11_MSM_BASE         (L2_REASON_CODE_DOT11_AC_BASE+L2_REASON_CODE_GROUP_SIZE)
#define L2_REASON_CODE_DOT11_SECURITY_BASE    (L2_REASON_CODE_DOT11_MSM_BASE+L2_REASON_CODE_GROUP_SIZE)
#define L2_REASON_CODE_ONEX_BASE              (L2_REASON_CODE_DOT11_SECURITY_BASE+L2_REASON_CODE_GROUP_SIZE)
#define L2_REASON_CODE_DOT3_AC_BASE           (L2_REASON_CODE_ONEX_BASE+L2_REASON_CODE_GROUP_SIZE)
#define L2_REASON_CODE_DOT3_MSM_BASE          (L2_REASON_CODE_DOT3_AC_BASE+L2_REASON_CODE_GROUP_SIZE)
#define L2_REASON_CODE_PROFILE_BASE           (L2_REASON_CODE_DOT3_MSM_BASE+L2_REASON_CODE_GROUP_SIZE)
#define L2_REASON_CODE_IHV_BASE               (L2_REASON_CODE_PROFILE_BASE+L2_REASON_CODE_GROUP_SIZE)
#define L2_REASON_CODE_WIMAX_BASE             (L2_REASON_CODE_IHV_BASE+L2_REASON_CODE_GROUP_SIZE)
#define L2_REASON_CODE_RESERVED_BASE          (L2_REASON_CODE_WIMAX_BASE+L2_REASON_CODE_GROUP_SIZE)

// the common reason codes
#define L2_REASON_CODE_SUCCESS			      0

// general reason code
#define L2_REASON_CODE_UNKNOWN                  (L2_REASON_CODE_GEN_BASE+1)

// common profile section missing
#define L2_REASON_CODE_PROFILE_MISSING          0x00000001

// This structure is the notification structure which needs to be filled in by each component 
// The interface guid is filled in only by the AC 
// The NotificationSource signifies the type above 
// and NotificationCode is one of the enum values 
typedef struct _L2_NOTIFICATION_DATA {
    DWORD NotificationSource;
    DWORD NotificationCode;
    GUID InterfaceGuid;
    DWORD dwDataSize;
    
#ifdef __midl
    [unique, size_is(dwDataSize)] PBYTE pData;
#else
    _Field_size_bytes_(dwDataSize) PVOID pData;
#endif

} L2_NOTIFICATION_DATA, *PL2_NOTIFICATION_DATA;

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // _L2CMN_H
