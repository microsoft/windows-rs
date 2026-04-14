/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) 1996-1999  Microsoft Corporation

Module Name:

    dsgetdc.h

Abstract:

    This file contains structures, function prototypes, and definitions
    for the DsGetDcName API.

Environment:

    User Mode - Win32

Notes:

--*/


#ifndef _DSGETDC_
#define _DSGETDC_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#if !defined(_DSGETDCAPI_)
#define DSGETDCAPI DECLSPEC_IMPORT
#else
#define DSGETDCAPI
#endif

#ifdef __cplusplus
extern "C" {
#endif

//
// Structure definitions
//

//
// Flags to passed to DsGetDcName
//

#define DS_FORCE_REDISCOVERY            0x00000001

#define DS_DIRECTORY_SERVICE_REQUIRED   0x00000010
#define DS_DIRECTORY_SERVICE_PREFERRED  0x00000020
#define DS_GC_SERVER_REQUIRED           0x00000040
#define DS_PDC_REQUIRED                 0x00000080
#define DS_BACKGROUND_ONLY              0x00000100
#define DS_IP_REQUIRED                  0x00000200
#define DS_KDC_REQUIRED                 0x00000400
#define DS_TIMESERV_REQUIRED            0x00000800
#define DS_WRITABLE_REQUIRED            0x00001000
#define DS_GOOD_TIMESERV_PREFERRED      0x00002000
#define DS_AVOID_SELF                   0x00004000
#define DS_ONLY_LDAP_NEEDED             0x00008000


#define DS_IS_FLAT_NAME                 0x00010000
#define DS_IS_DNS_NAME                  0x00020000

#define DS_TRY_NEXTCLOSEST_SITE         0x00040000

#define DS_DIRECTORY_SERVICE_6_REQUIRED 0x00080000

#define DS_WEB_SERVICE_REQUIRED         0x00100000

#define DS_DIRECTORY_SERVICE_8_REQUIRED 0x00200000

#define DS_DIRECTORY_SERVICE_9_REQUIRED 0x00400000

#define DS_DIRECTORY_SERVICE_10_REQUIRED 0x00800000

#define DS_KEY_LIST_SUPPORT_REQUIRED     0x01000000

#define DS_DIRECTORY_SERVICE_13_REQUIRED 0x02000000

#define DS_RETURN_DNS_NAME              0x40000000
#define DS_RETURN_FLAT_NAME             0x80000000

#define DSGETDC_VALID_FLAGS ( \
            DS_FORCE_REDISCOVERY | \
            DS_DIRECTORY_SERVICE_REQUIRED | \
            DS_DIRECTORY_SERVICE_PREFERRED | \
            DS_GC_SERVER_REQUIRED | \
            DS_PDC_REQUIRED | \
            DS_BACKGROUND_ONLY | \
            DS_IP_REQUIRED | \
            DS_KDC_REQUIRED | \
            DS_TIMESERV_REQUIRED | \
            DS_WRITABLE_REQUIRED | \
            DS_GOOD_TIMESERV_PREFERRED | \
            DS_AVOID_SELF | \
            DS_ONLY_LDAP_NEEDED | \
            DS_IS_FLAT_NAME | \
            DS_IS_DNS_NAME | \
            DS_TRY_NEXTCLOSEST_SITE | \
            DS_DIRECTORY_SERVICE_6_REQUIRED | \
            DS_DIRECTORY_SERVICE_8_REQUIRED | \
            DS_DIRECTORY_SERVICE_9_REQUIRED | \
            DS_DIRECTORY_SERVICE_10_REQUIRED | \
            DS_KEY_LIST_SUPPORT_REQUIRED | \
            DS_DIRECTORY_SERVICE_13_REQUIRED | \
            DS_WEB_SERVICE_REQUIRED | \
            DS_RETURN_FLAT_NAME  | \
            DS_RETURN_DNS_NAME )

#define DS_DIRECTORY_SERVICE_ALL_VERSIONS ( \
            DS_DIRECTORY_SERVICE_REQUIRED | \
            DS_DIRECTORY_SERVICE_6_REQUIRED | \
            DS_DIRECTORY_SERVICE_8_REQUIRED | \
            DS_DIRECTORY_SERVICE_9_REQUIRED | \
            DS_DIRECTORY_SERVICE_10_REQUIRED | \
            DS_DIRECTORY_SERVICE_13_REQUIRED )

//
// Structure returned from DsGetDcName
//

typedef struct _DOMAIN_CONTROLLER_INFOA {
    LPSTR DomainControllerName;
    LPSTR DomainControllerAddress;
    ULONG DomainControllerAddressType;
    GUID DomainGuid;
    LPSTR DomainName;
    LPSTR DnsForestName;
    ULONG Flags;
    LPSTR DcSiteName;
    LPSTR ClientSiteName;
} DOMAIN_CONTROLLER_INFOA, *PDOMAIN_CONTROLLER_INFOA;

typedef struct _DOMAIN_CONTROLLER_INFOW {
#ifdef MIDL_PASS
    [string,unique] wchar_t *DomainControllerName;
#else // MIDL_PASS
    LPWSTR DomainControllerName;
#endif // MIDL_PASS
#ifdef MIDL_PASS
    [string,unique] wchar_t *DomainControllerAddress;
#else // MIDL_PASS
    LPWSTR DomainControllerAddress;
#endif // MIDL_PASS
    ULONG DomainControllerAddressType;
    GUID DomainGuid;
#ifdef MIDL_PASS
    [string,unique] wchar_t *DomainName;
#else // MIDL_PASS
    LPWSTR DomainName;
#endif // MIDL_PASS
#ifdef MIDL_PASS
    [string,unique] wchar_t *DnsForestName;
#else // MIDL_PASS
    LPWSTR DnsForestName;
#endif // MIDL_PASS
    ULONG Flags;
#ifdef MIDL_PASS
    [string,unique] wchar_t *DcSiteName;
#else // MIDL_PASS
    LPWSTR DcSiteName;
#endif // MIDL_PASS
#ifdef MIDL_PASS
    [string,unique] wchar_t *ClientSiteName;
#else // MIDL_PASS
    LPWSTR ClientSiteName;
#endif // MIDL_PASS
} DOMAIN_CONTROLLER_INFOW, *PDOMAIN_CONTROLLER_INFOW;

#ifdef UNICODE
#define DOMAIN_CONTROLLER_INFO DOMAIN_CONTROLLER_INFOW
#define PDOMAIN_CONTROLLER_INFO PDOMAIN_CONTROLLER_INFOW
#else
#define DOMAIN_CONTROLLER_INFO DOMAIN_CONTROLLER_INFOA
#define PDOMAIN_CONTROLLER_INFO PDOMAIN_CONTROLLER_INFOA
#endif // !UNICODE

//
// Values for DomainControllerAddressType
//

#define DS_INET_ADDRESS    1
#define DS_NETBIOS_ADDRESS 2

//
// Values for returned Flags
//

#define DS_PDC_FLAG            0x00000001    // DC is PDC of Domain
#define DS_GC_FLAG             0x00000004    // DC is a GC of forest
#define DS_LDAP_FLAG           0x00000008    // Server supports an LDAP server
#define DS_DS_FLAG             0x00000010    // DC supports a DS and is a Domain Controller
#define DS_KDC_FLAG            0x00000020    // DC is running KDC service
#define DS_TIMESERV_FLAG       0x00000040    // DC is running time service
#define DS_CLOSEST_FLAG        0x00000080    // DC is in closest site to client
#define DS_WRITABLE_FLAG       0x00000100    // DC has a writable DS
#define DS_GOOD_TIMESERV_FLAG  0x00000200    // DC is running time service (and has clock hardware)
#define DS_NDNC_FLAG           0x00000400    // DomainName is non-domain NC serviced by the LDAP server
#define DS_SELECT_SECRET_DOMAIN_6_FLAG  0x00000800  // DC has some secrets
#define DS_FULL_SECRET_DOMAIN_6_FLAG    0x00001000  // DC has all secrets
#define DS_WS_FLAG             0x00002000    // DC is running web service
#define DS_DS_8_FLAG           0x00004000    // DC is running Windows Server 2008 or later
#define DS_DS_9_FLAG           0x00008000    // DC is running Windows Server 2008R2 or later
#define DS_DS_10_FLAG          0x00010000    // DC is running Windows Server 2016 or later
#define DS_KEY_LIST_FLAG       0x00020000    // DC supports key list requests
#define DS_DS_13_FLAG          0x00040000    // DC is running Windows Server 2025 or later
#define DS_PING_FLAGS          0x000FFFFF    // Flags returned on ping

#define DS_DNS_CONTROLLER_FLAG 0x20000000    // DomainControllerName is a DNS name
#define DS_DNS_DOMAIN_FLAG     0x40000000    // DomainName is a DNS name
#define DS_DNS_FOREST_FLAG     0x80000000    // DnsForestName is a DNS name


//
// Function Prototypes
//

_Success_(return == ERROR_SUCCESS)
DSGETDCAPI
DWORD
WINAPI
DsGetDcNameA(
    _In_opt_ LPCSTR ComputerName,
    _In_opt_ LPCSTR DomainName,
    _In_opt_ GUID *DomainGuid,
    _In_opt_ LPCSTR SiteName,
    _In_ ULONG Flags,
    _Outptr_result_nullonfailure_ PDOMAIN_CONTROLLER_INFOA *DomainControllerInfo
    );

_Success_(return == ERROR_SUCCESS)
DSGETDCAPI
DWORD
WINAPI
DsGetDcNameW(
    _In_opt_ LPCWSTR ComputerName,
    _In_opt_ LPCWSTR DomainName,
    _In_opt_ GUID *DomainGuid,
    _In_opt_ LPCWSTR SiteName,
    _In_ ULONG Flags,
    _Outptr_result_nullonfailure_ PDOMAIN_CONTROLLER_INFOW *DomainControllerInfo
    );

#ifdef UNICODE
#define DsGetDcName DsGetDcNameW
#else
#define DsGetDcName DsGetDcNameA
#endif // !UNICODE

_Success_(return == ERROR_SUCCESS)
DSGETDCAPI
DWORD
WINAPI
DsGetSiteNameA(
    _In_opt_ LPCSTR ComputerName,
    _Outptr_result_nullonfailure_ LPSTR *SiteName
    );

_Success_(return == ERROR_SUCCESS)
DSGETDCAPI
DWORD
WINAPI
DsGetSiteNameW(
    _In_opt_ LPCWSTR ComputerName,
    _Outptr_result_nullonfailure_ LPWSTR *SiteName
    );

#ifdef UNICODE
#define DsGetSiteName DsGetSiteNameW
#else
#define DsGetSiteName DsGetSiteNameA
#endif // !UNICODE

DSGETDCAPI
DWORD
WINAPI
DsValidateSubnetNameW(
    _In_ LPCWSTR SubnetName
    );

DSGETDCAPI
DWORD
WINAPI
DsValidateSubnetNameA(
    _In_ LPCSTR SubnetName
    );

#ifdef UNICODE
#define DsValidateSubnetName DsValidateSubnetNameW
#else
#define DsValidateSubnetName DsValidateSubnetNameA
#endif // !UNICODE


//
// Only include if winsock2.h has been included
//
#ifdef _WINSOCK2API_

_Success_(return == ERROR_SUCCESS)
DSGETDCAPI
DWORD
WINAPI
DsAddressToSiteNamesW(
    _In_opt_ LPCWSTR ComputerName,
    _In_ DWORD EntryCount,
    _In_reads_(EntryCount) PSOCKET_ADDRESS SocketAddresses,
    _Outptr_result_nullonfailure_ _Outptr_result_buffer_(EntryCount) LPWSTR **SiteNames
    );

_Success_(return == ERROR_SUCCESS)
DSGETDCAPI
DWORD
WINAPI
DsAddressToSiteNamesA(
    _In_opt_ LPCSTR ComputerName,
    _In_ DWORD EntryCount,
    _In_reads_(EntryCount) PSOCKET_ADDRESS SocketAddresses,
    _Outptr_result_nullonfailure_ _Outptr_result_buffer_(EntryCount) LPSTR **SiteNames
    );

#ifdef UNICODE
#define DsAddressToSiteNames DsAddressToSiteNamesW
#else
#define DsAddressToSiteNames DsAddressToSiteNamesA
#endif // !UNICODE

_Success_(return == ERROR_SUCCESS)
DSGETDCAPI
DWORD
WINAPI
DsAddressToSiteNamesExW(
    _In_opt_ LPCWSTR ComputerName,
    _In_ DWORD EntryCount,
    _In_reads_(EntryCount) PSOCKET_ADDRESS SocketAddresses,
    _Outptr_result_nullonfailure_ _Outptr_result_buffer_(EntryCount) LPWSTR **SiteNames,
    _Outptr_result_nullonfailure_ _Outptr_result_buffer_(EntryCount) LPWSTR **SubnetNames
    );

_Success_(return == ERROR_SUCCESS)
DSGETDCAPI
DWORD
WINAPI
DsAddressToSiteNamesExA(
    _In_opt_ LPCSTR ComputerName,
    _In_ DWORD EntryCount,
    _In_reads_(EntryCount) PSOCKET_ADDRESS SocketAddresses,
    _Outptr_result_nullonfailure_ _Outptr_result_buffer_(EntryCount) LPSTR **SiteNames,
    _Outptr_result_nullonfailure_ _Outptr_result_buffer_(EntryCount) LPSTR **SubnetNames
    );

#ifdef UNICODE
#define DsAddressToSiteNamesEx DsAddressToSiteNamesExW
#else
#define DsAddressToSiteNamesEx DsAddressToSiteNamesExA
#endif // !UNICODE
#endif // _WINSOCK2API_

//
// API to enumerate trusted domains
//

typedef struct _DS_DOMAIN_TRUSTSW {

    //
    // Name of the trusted domain.
    //
#ifdef MIDL_PASS
    [string] wchar_t * NetbiosDomainName;
    [string] wchar_t * DnsDomainName;
#else // MIDL_PASS
    LPWSTR NetbiosDomainName;
    LPWSTR DnsDomainName;
#endif // MIDL_PASS


    //
    // Flags defining attributes of the trust.
    //
    ULONG Flags;
#define DS_DOMAIN_IN_FOREST           0x0001  // Domain is a member of the forest
#define DS_DOMAIN_DIRECT_OUTBOUND     0x0002  // Domain is directly trusted
#define DS_DOMAIN_TREE_ROOT           0x0004  // Domain is root of a tree in the forest
#define DS_DOMAIN_PRIMARY             0x0008  // Domain is the primary domain of queried server
#define DS_DOMAIN_NATIVE_MODE         0x0010  // Primary domain is running in native mode
#define DS_DOMAIN_DIRECT_INBOUND      0x0020  // Domain is directly trusting
#define DS_DOMAIN_VALID_FLAGS (         \
            DS_DOMAIN_IN_FOREST       | \
            DS_DOMAIN_DIRECT_OUTBOUND | \
            DS_DOMAIN_TREE_ROOT       | \
            DS_DOMAIN_PRIMARY         | \
            DS_DOMAIN_NATIVE_MODE     | \
            DS_DOMAIN_DIRECT_INBOUND )

    //
    // Index to the domain that is the parent of this domain.
    //  Only defined if NETLOGON_DOMAIN_IN_FOREST is set and
    //      NETLOGON_DOMAIN_TREE_ROOT is not set.
    //
    ULONG ParentIndex;

    //
    // The trust type and attributes of this trust.
    //
    // If NETLOGON_DOMAIN_DIRECTLY_TRUSTED is not set,
    //  these value are infered.
    //
    ULONG TrustType;
    ULONG TrustAttributes;

    //
    // The SID of the trusted domain.
    //
    // If NETLOGON_DOMAIN_DIRECTLY_TRUSTED is not set,
    //  this value will be NULL.
    //
#if defined(MIDL_PASS)
    PISID DomainSid;
#else
    PSID DomainSid;
#endif

    //
    // The GUID of the trusted domain.
    //

    GUID DomainGuid;

} DS_DOMAIN_TRUSTSW, *PDS_DOMAIN_TRUSTSW;

//
// ANSI version of the above struct
//
typedef struct _DS_DOMAIN_TRUSTSA {
    LPSTR NetbiosDomainName;
    LPSTR DnsDomainName;
    ULONG Flags;
    ULONG ParentIndex;
    ULONG TrustType;
    ULONG TrustAttributes;
    PSID DomainSid;
    GUID DomainGuid;
} DS_DOMAIN_TRUSTSA, *PDS_DOMAIN_TRUSTSA;

#ifdef UNICODE
#define DS_DOMAIN_TRUSTS DS_DOMAIN_TRUSTSW
#define PDS_DOMAIN_TRUSTS PDS_DOMAIN_TRUSTSW
#else
#define DS_DOMAIN_TRUSTS DS_DOMAIN_TRUSTSA
#define PDS_DOMAIN_TRUSTS PDS_DOMAIN_TRUSTSA
#endif // !UNICODE

_Success_(return == ERROR_SUCCESS)
DSGETDCAPI
DWORD
WINAPI
DsEnumerateDomainTrustsW (
    _In_opt_ LPWSTR ServerName,
    _In_ ULONG Flags,
    _Outptr_result_nullonfailure_ _Outptr_result_buffer_(*DomainCount) PDS_DOMAIN_TRUSTSW *Domains,
    _Out_ PULONG DomainCount
    );

_Success_(return == ERROR_SUCCESS)
DSGETDCAPI
DWORD
WINAPI
DsEnumerateDomainTrustsA (
    _In_opt_ LPSTR ServerName,
    _In_ ULONG Flags,
    _Outptr_result_nullonfailure_ _Outptr_result_buffer_(*DomainCount) PDS_DOMAIN_TRUSTSA *Domains,
    _Out_ PULONG DomainCount
    );

#ifdef UNICODE
#define DsEnumerateDomainTrusts DsEnumerateDomainTrustsW
#else
#define DsEnumerateDomainTrusts DsEnumerateDomainTrustsA
#endif // !UNICODE

//
// Only define this API if the caller has #included the pre-requisite
// ntlsa.h or ntsecapi.h
//

#if defined(_NTLSA_) || defined(_NTSECAPI_)

_Success_(return == ERROR_SUCCESS)
DSGETDCAPI
DWORD
WINAPI
DsGetForestTrustInformationW (
    _In_opt_ LPCWSTR ServerName,
    _In_opt_ LPCWSTR TrustedDomainName,
    _In_ DWORD Flags,
    _Outptr_result_nullonfailure_ PLSA_FOREST_TRUST_INFORMATION *ForestTrustInfo
    );

#define DS_GFTI_UPDATE_TDO      0x1     // Update TDO with information returned
#define DS_GFTI_VALID_FLAGS     0x1     // All valid flags to DsGetForestTrustInformation

_Success_(return == ERROR_SUCCESS)
DSGETDCAPI
DWORD
WINAPI
DsMergeForestTrustInformationW(
    _In_ LPCWSTR DomainName,
    _In_ PLSA_FOREST_TRUST_INFORMATION NewForestTrustInfo,
    _In_opt_ PLSA_FOREST_TRUST_INFORMATION OldForestTrustInfo,
    _Outptr_result_nullonfailure_ PLSA_FOREST_TRUST_INFORMATION *MergedForestTrustInfo
    );

#endif // _NTLSA_ || _NTSECAPI_

_Success_(return == ERROR_SUCCESS)
DSGETDCAPI
DWORD
WINAPI
DsGetDcSiteCoverageW(
    _In_opt_ LPCWSTR ServerName,
    _Out_ PULONG EntryCount,
    _Outptr_result_nullonfailure_ _Outptr_result_buffer_(*EntryCount) LPWSTR **SiteNames
    );

_Success_(return == ERROR_SUCCESS)
DSGETDCAPI
DWORD
WINAPI
DsGetDcSiteCoverageA(
    _In_opt_ LPCSTR ServerName,
    _Out_ PULONG EntryCount,
    _Outptr_result_nullonfailure_ _Outptr_result_buffer_(*EntryCount) LPSTR **SiteNames
    );

#ifdef UNICODE
#define DsGetDcSiteCoverage DsGetDcSiteCoverageW
#else
#define DsGetDcSiteCoverage DsGetDcSiteCoverageA
#endif // !UNICODE

DSGETDCAPI
DWORD
WINAPI
DsDeregisterDnsHostRecordsW (
    _In_opt_ LPWSTR ServerName,
    _In_opt_ LPWSTR DnsDomainName,
    _In_opt_ GUID   *DomainGuid,
    _In_opt_ GUID   *DsaGuid,
    _In_ LPWSTR DnsHostName
    );

DSGETDCAPI
DWORD
WINAPI
DsDeregisterDnsHostRecordsA (
    _In_opt_ LPSTR ServerName,
    _In_opt_ LPSTR DnsDomainName,
    _In_opt_ GUID  *DomainGuid,
    _In_opt_ GUID  *DsaGuid,
    _In_ LPSTR DnsHostName
    );

#ifdef UNICODE
#define DsDeregisterDnsHostRecords DsDeregisterDnsHostRecordsW
#else
#define DsDeregisterDnsHostRecords DsDeregisterDnsHostRecordsA
#endif // !UNICODE


#ifdef _WINSOCK2API_  // DsGetDcOpen/Next/Close depend on winsock2.h be included

//
// Option flags passed to DsGetDcOpen
//

#define DS_ONLY_DO_SITE_NAME         0x01   // Non-site specific names should be avoided.
#define DS_NOTIFY_AFTER_SITE_RECORDS 0x02   // Return ERROR_FILEMARK_DETECTED after all
                                            //  site specific records have been processed.

#define DS_OPEN_VALID_OPTION_FLAGS ( DS_ONLY_DO_SITE_NAME | DS_NOTIFY_AFTER_SITE_RECORDS )

//
// Valid DcFlags for DsGetDcOpen
//

#define DS_OPEN_VALID_FLAGS (       \
            DS_FORCE_REDISCOVERY  | \
            DS_ONLY_LDAP_NEEDED   | \
            DS_KDC_REQUIRED       | \
            DS_PDC_REQUIRED       | \
            DS_GC_SERVER_REQUIRED | \
            DS_WRITABLE_REQUIRED )


DSGETDCAPI
DWORD
WINAPI
DsGetDcOpenW(
    _In_ LPCWSTR DnsName,
    _In_ ULONG OptionFlags,
    _In_opt_ LPCWSTR SiteName,
    _In_opt_ GUID *DomainGuid,
    _In_opt_ LPCWSTR DnsForestName,
    _In_ ULONG DcFlags,
    _Out_ PHANDLE RetGetDcContext
    );

DSGETDCAPI
DWORD
WINAPI
DsGetDcOpenA(
    _In_ LPCSTR DnsName,
    _In_ ULONG OptionFlags,
    _In_opt_ LPCSTR SiteName,
    _In_opt_ GUID *DomainGuid,
    _In_opt_ LPCSTR DnsForestName,
    _In_ ULONG DcFlags,
    _Out_ PHANDLE RetGetDcContext
    );

#ifdef UNICODE
#define DsGetDcOpen DsGetDcOpenW
#else
#define DsGetDcOpen DsGetDcOpenA
#endif // !UNICODE

_Success_(return == ERROR_SUCCESS)
DSGETDCAPI
DWORD
WINAPI
DsGetDcNextW(
    _In_ HANDLE GetDcContextHandle,
    _Out_opt_ PULONG SockAddressCount,
    _Outptr_opt_result_buffer_(*SockAddressCount) LPSOCKET_ADDRESS *SockAddresses,
    _Outptr_opt_result_nullonfailure_ LPWSTR *DnsHostName
    );

_Success_(return == ERROR_SUCCESS)
DSGETDCAPI
DWORD
WINAPI
DsGetDcNextA(
    _In_ HANDLE GetDcContextHandle,
    _Out_opt_ PULONG SockAddressCount,
    _Outptr_opt_result_buffer_(*SockAddressCount) LPSOCKET_ADDRESS *SockAddresses,
    _Outptr_opt_result_nullonfailure_ LPSTR *DnsHostName
    );

#ifdef UNICODE
#define DsGetDcNext DsGetDcNextW
#else
#define DsGetDcNext DsGetDcNextA
#endif // !UNICODE

DSGETDCAPI
VOID
WINAPI
DsGetDcCloseW(
    _In_ HANDLE GetDcContextHandle
    );

#ifdef UNICODE
#define DsGetDcClose DsGetDcCloseW
#else
#define DsGetDcClose DsGetDcCloseW  // same for ANSI
#endif // !UNICODE

#endif // _WINSOCK2API_

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // _DSGETDC_
