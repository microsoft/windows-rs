//
// Copyright (C) Microsoft Corporation. All rights reserved.
//
// Contents: Windows Connection Manager API Prototypes and Definitions
//

#ifndef _WCMAPI_H
#define _WCMAPI_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if NTDDI_VERSION >= NTDDI_WIN8

#ifdef __cplusplus
extern "C"
{
#endif

#define WCM_API_VERSION_1_0 0x00000001
#define WCM_API_VERSION WCM_API_VERSION_1_0
#define WCM_UNKNOWN_DATAPLAN_STATUS 0xFFFFFFFF
#define WCM_MAX_PROFILE_NAME 256

    typedef enum _WCM_PROPERTY
    {
        wcm_global_property_domain_policy,
        wcm_global_property_minimize_policy,
        wcm_global_property_roaming_policy,
        wcm_global_property_powermanagement_policy,
        wcm_intf_property_connection_cost, // used to set/get cost level and flags for the connection
        wcm_intf_property_dataplan_status, // used by MNO to indicate plan data associated with new cost
        wcm_intf_property_hotspot_profile, // used to store hotspot profile (WISPr credentials)
    } WCM_PROPERTY, *PWCM_PROPERTY;

    typedef enum _WCM_MEDIA_TYPE
    {
        wcm_media_unknown,
        wcm_media_ethernet,
        wcm_media_wlan,
        wcm_media_mbn,
        wcm_media_invalid,
        wcm_media_max
    } WCM_MEDIA_TYPE, *PWCM_MEDIA_TYPE;

    typedef struct _WCM_POLICY_VALUE
    {
        BOOL fValue;
        BOOL fIsGroupPolicy;
    } WCM_POLICY_VALUE, *PWCM_POLICY_VALUE;

    typedef struct _WCM_PROFILE_INFO
    {
        WCHAR strProfileName[WCM_MAX_PROFILE_NAME];
        GUID AdapterGUID;
        WCM_MEDIA_TYPE Media;
    } WCM_PROFILE_INFO, *PWCM_PROFILE_INFO;

    typedef struct _WCM_PROFILE_INFO_LIST
    {
        DWORD dwNumberOfItems;

#ifdef __midl
        [ unique, size_is(dwNumberOfItems) ] WCM_PROFILE_INFO ProfileInfo[*];
#else
    _Field_size_(dwNumberOfItems) WCM_PROFILE_INFO ProfileInfo[1];
#endif

    } WCM_PROFILE_INFO_LIST, *PWCM_PROFILE_INFO_LIST;

    typedef enum _WCM_CONNECTION_COST
    {
        // cost level
        WCM_CONNECTION_COST_UNKNOWN = 0x0,
        WCM_CONNECTION_COST_UNRESTRICTED = 0x1,
        WCM_CONNECTION_COST_FIXED = 0x2,
        WCM_CONNECTION_COST_VARIABLE = 0x4,
        // cost flags
        WCM_CONNECTION_COST_OVERDATALIMIT = 0x10000,
        WCM_CONNECTION_COST_CONGESTED = 0x20000,
        WCM_CONNECTION_COST_ROAMING = 0x40000,
        WCM_CONNECTION_COST_APPROACHINGDATALIMIT = 0x80000
    } WCM_CONNECTION_COST, *PWCM_CONNECTION_COST;

    typedef enum _WCM_CONNECTION_COST_SOURCE
    {
        WCM_CONNECTION_COST_SOURCE_DEFAULT = 0,
        WCM_CONNECTION_COST_SOURCE_GP = 1,
        WCM_CONNECTION_COST_SOURCE_USER = 2,
        WCM_CONNECTION_COST_SOURCE_OPERATOR = 3
    } WCM_CONNECTION_COST_SOURCE, *PWCM_CONNECTION_COST_SOURCE;

    typedef struct _WCM_CONNECTION_COST_DATA
    {
        DWORD ConnectionCost; // a bit-or of WCM_CONNECTION_COST
        WCM_CONNECTION_COST_SOURCE CostSource;
        // other fields will be added later
    } WCM_CONNECTION_COST_DATA, *PWCM_CONNECTION_COST_DATA;

    typedef struct _WCM_TIME_INTERVAL
    {
        WORD wYear;         // [0, 30827]
        WORD wMonth;        // [0,12]
        WORD wDay;          // [0,31]
        WORD wHour;         // [0,23]
        WORD wMinute;       // [0,59]
        WORD wSecond;       // [0,59]
        WORD wMilliseconds; // [0,999]
    } WCM_TIME_INTERVAL;

    typedef struct _WCM_USAGE_DATA
    {
        DWORD UsageInMegabytes;
        FILETIME LastSyncTime;
    } WCM_USAGE_DATA, *PWCM_USAGE_DATA;

    typedef struct WCM_BILLING_CYCLE_INFO
    {
        FILETIME StartDate;
        WCM_TIME_INTERVAL Duration;
        BOOL Reset;
    } WCM_BILLING_CYCLE_INFO;

    typedef struct _WCM_DATAPLAN_STATUS
    {
        WCM_USAGE_DATA UsageData;
        DWORD DataLimitInMegabytes;
        DWORD InboundBandwidthInKbps;
        DWORD OutboundBandwidthInKbps;
        WCM_BILLING_CYCLE_INFO BillingCycle;
        DWORD MaxTransferSizeInMegabytes;
        DWORD Reserved;
    } WCM_DATAPLAN_STATUS, *PWCM_DATAPLAN_STATUS;

#ifndef __midl

    DWORD WINAPI WcmQueryProperty(
        _In_opt_ const GUID* pInterface, // NULL for global properties
        _In_opt_ LPCWSTR strProfileName,
        _In_ WCM_PROPERTY Property,
        _Reserved_ void* pReserved,
        _Out_ PDWORD pdwDataSize,
        _Outptr_result_buffer_maybenull_(*pdwDataSize) PBYTE* ppData);

    DWORD WINAPI WcmSetProperty(
        _In_opt_ const GUID* pInterface, // NULL for global properties
        _In_opt_ LPCWSTR strProfileName,
        _In_ WCM_PROPERTY Property,
        _Reserved_ void* pReserved,
        _In_ DWORD dwDataSize,
        _In_reads_opt_(dwDataSize) const BYTE* pbData);

    DWORD WINAPI WcmGetProfileList(_Reserved_ void* pReserved, _Outptr_ WCM_PROFILE_INFO_LIST** ppProfileList);

    // Ethernet pseudo-profiles should not be included in pProfileList
    DWORD WINAPI WcmSetProfileList(
        _In_ WCM_PROFILE_INFO_LIST* pProfileList, _In_ DWORD dwPosition, _In_ BOOL fIgnoreUnknownProfiles, _Reserved_ void* pReserved);

    void WINAPI WcmFreeMemory(__deallocate(Mem) void* pMemory);

#endif // not __midl

#ifdef __cplusplus
} // #ifdef __cplusplus
#endif

#endif // NTDDI_VERSION >= NTDDI_WIN8

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _WCMAPI_H
