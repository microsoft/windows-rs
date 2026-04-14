/*++ BUILD Version: 0003    // Increment this if a change has global effects

Copyright (c) 1991-1999  Microsoft Corporation

Module Name:

    lmconfig.h

Abstract:

    This module defines the API function prototypes and data structures
    for the following groups of NT API functions:
        NetConfig

Environment:

    User Mode - Win32

Notes:

    You must include NETCONS.H before this file, since this file depends
    on values defined in NETCONS.H.

--*/

#ifndef _LMCONFIG_
#define _LMCONFIG_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

#define REVISED_CONFIG_APIS

//
// Function Prototypes - Config
//

#pragma warning(push)
  // SAL Annotations not available for obsolete APIs 
#pragma warning(disable:28740)

NET_API_STATUS NET_API_FUNCTION
NetConfigGet (
    IN  LPCWSTR  server OPTIONAL,
    IN  LPCWSTR  component,
    IN  LPCWSTR  parameter,
#ifdef REVISED_CONFIG_APIS
    OUT LPBYTE  *bufptr
#else
    OUT LPBYTE  *bufptr,
    OUT LPDWORD totalavailable
#endif
    );

NET_API_STATUS NET_API_FUNCTION
NetConfigGetAll (
    IN  LPCWSTR  server OPTIONAL,
    IN  LPCWSTR  component,
#ifdef REVISED_CONFIG_APIS
    OUT LPBYTE  *bufptr
#else
    OUT LPBYTE  *bufptr,
    OUT LPDWORD totalavailable
#endif
    );


NET_API_STATUS NET_API_FUNCTION
NetConfigSet (
    IN  LPCWSTR  server OPTIONAL,
    IN  LPCWSTR  reserved1 OPTIONAL,
    IN  LPCWSTR  component,
    IN  DWORD   level,
    IN  DWORD   reserved2,
    IN  LPBYTE  buf,
    IN  DWORD   reserved3
    );

#pragma warning(pop)

//
// Data Structures - Config
//

typedef struct _CONFIG_INFO_0 {
     LPWSTR         cfgi0_key;
     LPWSTR         cfgi0_data;
} CONFIG_INFO_0, *PCONFIG_INFO_0, *LPCONFIG_INFO_0;


#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // _LMCONFIG_
