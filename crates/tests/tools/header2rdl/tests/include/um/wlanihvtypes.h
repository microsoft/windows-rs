/*++

    Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    wlanihvtypes.h

Abstract:

    Structures used to hold information for IHV.

--*/

#ifndef __WLANIHVTYPES_H__
#define __WLANIHVTYPES_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <EapTypes.h>
#include <WlanTypes.h>

// Select MS Security settings 
// as may be needed by the IHVs
typedef
struct _DOT11_MSSECURITY_SETTINGS
{
   DOT11_AUTH_ALGORITHM dot11AuthAlgorithm;
   DOT11_CIPHER_ALGORITHM dot11CipherAlgorithm;
   BOOL fOneXEnabled;
   EAP_METHOD_TYPE eapMethodType;
   DWORD dwEapConnectionDataLen;
#ifdef __midl
   [size_is(dwEapConnectionDataLen)] 
#endif
   BYTE* pEapConnectionData;
}
DOT11_MSSECURITY_SETTINGS, *PDOT11_MSSECURITY_SETTINGS;


typedef
struct _DOT11EXT_IHV_SSID_LIST
{
    ULONG                                   ulCount;
#ifdef __midl
    [unique, size_is(ulCount)] DOT11_SSID   SSIDs[*];
#else
    DOT11_SSID                              SSIDs[1];
#endif
}
DOT11EXT_IHV_SSID_LIST, * PDOT11EXT_IHV_SSID_LIST;


// Data structure to hold
// additional information regarding
// IHV profile.
typedef
struct _DOT11EXT_IHV_PROFILE_PARAMS
{
   PDOT11EXT_IHV_SSID_LIST      pSsidList;
   DOT11_BSS_TYPE               BssType;
   PDOT11_MSSECURITY_SETTINGS   pMSSecuritySettings;
}
DOT11EXT_IHV_PROFILE_PARAMS, *PDOT11EXT_IHV_PROFILE_PARAMS;

// the following shoud be the same as WLAN_MAX_NAME_LENGTH
#define MS_MAX_PROFILE_NAME_LENGTH 256

// profile flags
#define MS_PROFILE_GROUP_POLICY    0x00000001
#define MS_PROFILE_USER            0x00000002

// Data structure to hold
// all params passed to IHV
// for the UI extension
typedef
struct _DOT11EXT_IHV_PARAMS
{
   DOT11EXT_IHV_PROFILE_PARAMS  dot11ExtIhvProfileParams;
   WCHAR                        wstrProfileName[MS_MAX_PROFILE_NAME_LENGTH];
   DWORD                        dwProfileTypeFlags;
   GUID                         interfaceGuid;
}
DOT11EXT_IHV_PARAMS, *PDOT11EXT_IHV_PARAMS;



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __WLANIHVTYPES_H__
