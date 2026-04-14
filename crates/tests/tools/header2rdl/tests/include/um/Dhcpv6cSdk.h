#include <winapifamily.h>

/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    dhcpv6csdk.h

Abstract:

    These are the exported dhcpv6 client api function definitions

Author:

    Achint Setia (asetia)  July-1-2005

Environment:

    User Mode - Win32

Revision History:


--*/

#ifndef _DHCPV6CSDK_
#define _DHCPV6CSDK_
#ifdef __cplusplus
extern "C" {
#endif

#ifndef DHCPV6_OPTIONS_DEFINED
#define DHCPV6_OPTIONS_DEFINED

#if _MSC_VER > 1000
#pragma once
#endif

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

//
// DHCPv6 Standard Options(non-encapsulated).
//



#define    DHCPV6_OPTION_CLIENTID        1
#define    DHCPV6_OPTION_SERVERID       2
#define    DHCPV6_OPTION_IA_NA             3
#define    DHCPV6_OPTION_IA_TA             4
#define    DHCPV6_OPTION_ORO                6
#define    DHCPV6_OPTION_PREFERENCE  7
#define    DHCPV6_OPTION_UNICAST        12
#define    DHCPV6_OPTION_RAPID_COMMIT  14
#define    DHCPV6_OPTION_USER_CLASS  15
#define    DHCPV6_OPTION_VENDOR_CLASS  16
#define    DHCPV6_OPTION_VENDOR_OPTS  17
#define    DHCPV6_OPTION_RECONF_MSG  19

#define    DHCPV6_OPTION_SIP_SERVERS_NAMES 21
#define    DHCPV6_OPTION_SIP_SERVERS_ADDRS 22
#define    DHCPV6_OPTION_DNS_SERVERS 23
#define    DHCPV6_OPTION_DOMAIN_LIST 24
#define    DHCPV6_OPTION_IA_PD             25
#define    DHCPV6_OPTION_NIS_SERVERS 27
#define    DHCPV6_OPTION_NISP_SERVERS 28
#define    DHCPV6_OPTION_NIS_DOMAIN_NAME  29
#define    DHCPV6_OPTION_NISP_DOMAIN_NAME  30

#define    DHCPV6_OPTION_DNR    144

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // DHCPV6_OPTIONS_DEFINED

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#ifndef DHCPV6API_PARAMS_DEFINED
#define DHCPV6API_PARAMS_DEFINED

typedef struct _DHCPV6CAPI_PARAMS {                  // use this structure to request params
    ULONG                        Flags;         // for future use
    ULONG                        OptionId;      // what option is this?
    BOOL                         IsVendor;      // is this vendor specific?
    LPBYTE                       Data;          // the actual data
    DWORD                        nBytesData;    // how many bytes of data are there in Data?
} DHCPV6CAPI_PARAMS, *PDHCPV6CAPI_PARAMS, *LPDHCPV6CAPI_PARAMS;

#endif // DHCPV6API_PARAMS_DEFINED

typedef struct _DHCPV6CAPI_PARAMS_ARRAY {          // array of params..
    ULONG                            nParams;       // size of array
    LPDHCPV6CAPI_PARAMS              Params;        // actual array
} DHCPV6CAPI_PARAMS_ARRAY, *PDHCPV6CAPI_PARAMS_ARRAY, *LPDHCPV6CAPI_PARAMS_ARRAY;

typedef struct _DHCPV6CAPI_CLASSID {                // defines a client class id.
    ULONG                          Flags;         // must be zero currently.
    #ifdef __midl
        [size_is(nBytesData)] LPBYTE Data;
    #else
        _Field_size_bytes_(nBytesData) LPBYTE                         Data;          // classid binary data.
    #endif
    ULONG                          nBytesData;    // how many bytes of data are there?
} DHCPV6CAPI_CLASSID, *PDHCPV6CAPI_CLASSID, *LPDHCPV6CAPI_CLASSID;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

typedef
enum
{
    STATUS_NO_ERROR,                         //Prefix successfully leased, renewed.
    STATUS_UNSPECIFIED_FAILURE,  //Some unspecified failure occurred  while trying to process the prefix.
    STATUS_NO_BINDING = 3,          //The server doesn't have binding for the prefix.
    STATUS_NOPREFIX_AVAIL = 6   //The server doesn't have prefix to offer to the requesting client.
}StatusCode;

typedef struct _DHCPV6Prefix{
    UCHAR  prefix[16];               //128 bit prefix
    DWORD prefixLength;		//(48-64 bits)
    DWORD preferredLifeTime; 	//The Preferred Lifetime of the Prefix returned or requested in seconds
    DWORD validLifeTime; 	//The Valid Lifetime of the Prefix returned or requested in seconds
    StatusCode status;              //The status code returned by the server for the prefix
}DHCPV6Prefix, *PDHCPV6Prefix, *LPDHCPV6Prefix;

typedef struct _DHCPV6PrefixLeaseInformation {
        DWORD nPrefixes;		// number of prefixes.
        _Field_size_(nPrefixes) LPDHCPV6Prefix prefixArray;  // Array of prefixes
	DWORD iaid;                     //The 32 bit Identity Association identifier for the prefix option.
	time_t T1;			//The absolute renewal time for the prefixes in seconds
	time_t T2; 			//The absolute rebind time for the prefixes in seconds
	time_t MaxLeaseExpirationTime; //The absolute maximum lease expiration time of all the prefix leases in this structure.
	time_t LastRenewalTime;        // The absolute time at which the last renewal for the prefixes happened.
	StatusCode status;              //The status code returned by the server for the IAPD
        _Field_size_bytes_(ServerIdLen) LPBYTE ServerId;		// The server DUID from which the prefix is received. This is used in subsequent Renews.
	DWORD  ServerIdLen;		// The length of the above DUID data.
} DHCPV6PrefixLeaseInformation, *PDHCPV6PrefixLeaseInformation, *LPDHCPV6PrefixLeaseInformation;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

VOID
APIENTRY
Dhcpv6CApiInitialize(
    OUT     LPDWORD                Version
);

VOID
APIENTRY
Dhcpv6CApiCleanup(
    VOID
);

DWORD                                             // win32 status
APIENTRY
Dhcpv6RequestParams(                                // request parameters of client
IN      BOOL		     		              forceNewInform,
IN      LPVOID                 		reserved,
_In_ IN      LPWSTR                 		adapterName,
IN      LPDHCPV6CAPI_CLASSID     	classId,
IN OUT  DHCPV6CAPI_PARAMS_ARRAY 	recdParams,
IN      LPBYTE                 		buffer,
IN OUT  LPDWORD                		pSize
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

DWORD
APIENTRY
Dhcpv6RequestPrefix(
IN _In_ LPWSTR		adapterName,
IN LPDHCPV6CAPI_CLASSID 		pclassId,
IN OUT LPDHCPV6PrefixLeaseInformation	prefixleaseInfo,
_Out_ DWORD* 			pdwTimeToWait
);

DWORD
APIENTRY
Dhcpv6RenewPrefix(
IN _In_ LPWSTR		adapterName,
IN LPDHCPV6CAPI_CLASSID 		pclassId,
IN OUT LPDHCPV6PrefixLeaseInformation	prefixleaseInfo,
_Out_ DWORD* 			pdwTimeToWait,
IN DWORD 				bValidatePrefix
);

DWORD
APIENTRY
Dhcpv6ReleasePrefix(
_In_ IN LPWSTR				adapterName,
IN LPDHCPV6CAPI_CLASSID     		classId,
IN LPDHCPV6PrefixLeaseInformation	leaseInfo
);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef __cplusplus
}
#endif

//================================================================================
// end of file
//================================================================================
#endif // _DHCPV6CSDK_

