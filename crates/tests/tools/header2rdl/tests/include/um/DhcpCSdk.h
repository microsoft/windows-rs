#include <winapifamily.h>

/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    dhcpcsdk.h

Abstract:

    These are the exported dhcpv6 client api function definitions

Author:

    Achint Setia (asetia)  July-1-2005

Environment:

    User Mode - Win32

Revision History:


--*/

//================================================================================
//  Copyright (C) 1997-1999 Microsoft Corporation
//  Description: these are the exported dhcp client api function definitions
//================================================================================
#ifndef _DHCPCSDK_
#define _DHCPCSDK_

#ifdef __cplusplus
extern "C" {
#endif

#ifndef DHCP_OPTIONS_DEFINED
#define DHCP_OPTIONS_DEFINED

#if _MSC_VER > 1000
#pragma once
#endif

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

//
// DHCP Standard Options.
//

#define OPTION_PAD                      0
#define OPTION_SUBNET_MASK              1
#define OPTION_TIME_OFFSET              2
#define OPTION_ROUTER_ADDRESS           3
#define OPTION_TIME_SERVERS             4
#define OPTION_IEN116_NAME_SERVERS      5
#define OPTION_DOMAIN_NAME_SERVERS      6
#define OPTION_LOG_SERVERS              7
#define OPTION_COOKIE_SERVERS           8
#define OPTION_LPR_SERVERS              9
#define OPTION_IMPRESS_SERVERS          10
#define OPTION_RLP_SERVERS              11
#define OPTION_HOST_NAME                12
#define OPTION_BOOT_FILE_SIZE           13
#define OPTION_MERIT_DUMP_FILE          14
#define OPTION_DOMAIN_NAME              15
#define OPTION_SWAP_SERVER              16
#define OPTION_ROOT_DISK                17
#define OPTION_EXTENSIONS_PATH          18

//
// IP layer parameters - per host
//

#define OPTION_BE_A_ROUTER              19
#define OPTION_NON_LOCAL_SOURCE_ROUTING 20
#define OPTION_POLICY_FILTER_FOR_NLSR   21
#define OPTION_MAX_REASSEMBLY_SIZE      22
#define OPTION_DEFAULT_TTL              23
#define OPTION_PMTU_AGING_TIMEOUT       24
#define OPTION_PMTU_PLATEAU_TABLE       25

//
// Link layer parameters - per interface.
//

#define OPTION_MTU                      26
#define OPTION_ALL_SUBNETS_MTU          27
#define OPTION_BROADCAST_ADDRESS        28
#define OPTION_PERFORM_MASK_DISCOVERY   29
#define OPTION_BE_A_MASK_SUPPLIER       30
#define OPTION_PERFORM_ROUTER_DISCOVERY 31
#define OPTION_ROUTER_SOLICITATION_ADDR 32
#define OPTION_STATIC_ROUTES            33
#define OPTION_TRAILERS                 34
#define OPTION_ARP_CACHE_TIMEOUT        35
#define OPTION_ETHERNET_ENCAPSULATION   36

//
// TCP Paramters - per host
//

#define OPTION_TTL                      37
#define OPTION_KEEP_ALIVE_INTERVAL      38
#define OPTION_KEEP_ALIVE_DATA_SIZE     39

//
// Application Layer Parameters
//

#define OPTION_NETWORK_INFO_SERVICE_DOM 40
#define OPTION_NETWORK_INFO_SERVERS     41
#define OPTION_NETWORK_TIME_SERVERS     42

//
// Vender specific information option
//

#define OPTION_VENDOR_SPEC_INFO         43

//
// NetBIOS over TCP/IP Name server option
//

#define OPTION_NETBIOS_NAME_SERVER      44
#define OPTION_NETBIOS_DATAGRAM_SERVER  45
#define OPTION_NETBIOS_NODE_TYPE        46
#define OPTION_NETBIOS_SCOPE_OPTION     47

//
// X Window System Options.
//

#define OPTION_XWINDOW_FONT_SERVER      48
#define OPTION_XWINDOW_DISPLAY_MANAGER  49

//
// Other extensions
//

#define OPTION_REQUESTED_ADDRESS        50
#define OPTION_LEASE_TIME               51
#define OPTION_OK_TO_OVERLAY            52
#define OPTION_MESSAGE_TYPE             53
#define OPTION_SERVER_IDENTIFIER        54
#define OPTION_PARAMETER_REQUEST_LIST   55
#define OPTION_MESSAGE                  56
#define OPTION_MESSAGE_LENGTH           57
#define OPTION_RENEWAL_TIME             58      // T1
#define OPTION_REBIND_TIME              59      // T2
#define OPTION_CLIENT_CLASS_INFO        60
#define OPTION_CLIENT_ID                61

#define OPTION_TFTP_SERVER_NAME         66
#define OPTION_BOOTFILE_NAME            67

#define OPTION_IPV6_ONLY_PREFERRED      108

#define OPTION_DNR                      162

#define OPTION_MSFT_IE_PROXY            252
#define OPTION_END                      255

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // DHCP_OPTIONS_DEFINED

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#ifndef DHCPAPI_PARAMS_DEFINED
#define DHCPAPI_PARAMS_DEFINED
typedef struct _DHCPAPI_PARAMS {                  // use this structure to request params
    ULONG                          Flags;         // for future use
    ULONG                          OptionId;      // what option is this?
    BOOL                           IsVendor;      // is this vendor specific?
    #ifdef __midl
        [size_is(nBytesData)] LPBYTE                         Data;          // the actual data
    #else
        _Field_size_bytes_(nBytesData) LPBYTE Data;
    #endif
    DWORD                          nBytesData;    // how many bytes of data are there in Data?
} DHCPAPI_PARAMS, *PDHCPAPI_PARAMS, *LPDHCPAPI_PARAMS;
#endif // DHCPAPI_PARAMS_DEFINED

typedef struct _DHCPAPI_PARAMS
DHCPCAPI_PARAMS, *PDHCPCAPI_PARAMS, *LPDHCPCAPI_PARAMS;

typedef struct _DHCPCAPI_PARAMS_ARARAY {          // array of params..
    ULONG                          nParams;       // size of array
    #ifdef __midl
        [size_is(nParams)] LPDHCPCAPI_PARAMS              Params;        // actual array
    #else
        _Field_size_(nParams) LPDHCPCAPI_PARAMS Params;
    #endif
} DHCPCAPI_PARAMS_ARRAY, *PDHCPCAPI_PARAMS_ARRAY, *LPDHCPCAPI_PARAMS_ARRAY;

typedef struct _DHCPCAPI_CLASSID {                // defines a client class id.
    ULONG                          Flags;         // must be zero currently.
    #ifdef __midl
        [size_is(nBytesData)] LPBYTE                         Data;          // classid binary data.
    #else
        _Field_size_bytes_(nBytesData) LPBYTE Data;
    #endif
        ULONG                          nBytesData;    // how many bytes of data are there?
} DHCPCAPI_CLASSID, *PDHCPCAPI_CLASSID, *LPDHCPCAPI_CLASSID;


#define     DHCPCAPI_REQUEST_PERSISTENT           0x01 // request this options "permanently"
#define     DHCPCAPI_REQUEST_SYNCHRONOUS          0x02 // request and block on it
#define     DHCPCAPI_REQUEST_ASYNCHRONOUS         0x04 // request and return, set event on completion
#define     DHCPCAPI_REQUEST_CANCEL               0x08 // cancel request
#define     DHCPCAPI_REQUEST_MASK                 0x0F // allowed flags..

DWORD
APIENTRY
DhcpCApiInitialize(
    OUT     LPDWORD                Version
);

VOID
APIENTRY
DhcpCApiCleanup(
    VOID
);

DWORD                                             // win32 status
APIENTRY
DhcpRequestParams(                                // request parameters of client
    IN      DWORD                  Flags,         // must be DHCPCAPI_REQUEST_SYNCHRONOUS
    IN      LPVOID                 Reserved,      // this parameter is reserved
    _In_    LPWSTR                 AdapterName,   // adapter name to request for
    IN      LPDHCPCAPI_CLASSID     ClassId,       // reserved must be NULL
    IN      DHCPCAPI_PARAMS_ARRAY  SendParams,    // parameters to send.
    IN OUT  DHCPCAPI_PARAMS_ARRAY  RecdParams,    // parameters that are to be requested..
    _Out_writes_bytes_to_(*pSize, *pSize) LPBYTE  Buffer,   // a buffer to hold data for RecdParams
    _Inout_ LPDWORD                pSize,         // i/p: size of above in BYTES, o/p required bytes..
    _In_    LPWSTR                 RequestIdStr   // needed for persistent requests
);  // returns ERROR_MORE_DATA if o/p buffer is of insufficient size, and fills in reqd size in # of bytes

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

DWORD                                             // win32 status
APIENTRY
DhcpUndoRequestParams(                            // undo the effect of a persistent request -- currently undo from registry
    _Reserved_ DWORD                Flags,         // must be zero, reserved
    _Reserved_ LPVOID               Reserved,      // this parameter is reserved
    _In_       LPWSTR               AdapterName,   // the name of the adpater to delete for
    _In_       LPWSTR               RequestIdStr   // needed for persistent requests..
);

#define     DHCPCAPI_REGISTER_HANDLE_EVENT        0x01 // handle returned is to an event
DWORD                                             // win32 status
APIENTRY
DhcpRegisterParamChange(                          // notify if a parameter has changed
    IN      DWORD                  Flags,         // must be zero, reserved
    _Reserved_ LPVOID                 Reserved,      // this parameter is reserved
    _In_       LPWSTR                 AdapterName,   // adapter of interest
    IN      LPDHCPCAPI_CLASSID     ClassId,       // reserved must be NULL
    IN      DHCPCAPI_PARAMS_ARRAY  Params,        // parameters of interest
    IN OUT  LPVOID                 Handle         // handle to event that will be SetEvent'ed in case of param change
);

#define     DHCPCAPI_DEREGISTER_HANDLE_EVENT      0x01 // de-register handle that is an event
DWORD
APIENTRY
DhcpDeRegisterParamChange(                        // undo the registration
    IN      DWORD                  Flags,         // MUST BE ZERO --> No flags yet.
    IN      LPVOID                 Reserved,      // MUST BE NULL --> Reserved
    IN      LPVOID                 Event          // handle to event returned by DhcpRegisterParamChange.
);

DWORD
APIENTRY
DhcpRemoveDNSRegistrations(
    VOID
    );

DWORD
APIENTRY
DhcpGetOriginalSubnetMask(
    IN LPCWSTR sAdapterName,
    OUT DWORD *dwSubnetMask
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef __cplusplus
}
#endif

//================================================================================
// end of file
//================================================================================
#endif // _DHCPCSDK_

