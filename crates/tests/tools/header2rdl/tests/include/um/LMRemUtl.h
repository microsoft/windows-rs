/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) 1991-1999  Microsoft Corporation

Module Name:

    lmremutl.h

Abstract:

    This file contains structures, function prototypes, and definitions
    for the NetRemote API.

Environment:

    User Mode - Win32
    Portable to any flat, 32-bit environment.  (Uses Win32 typedefs.)
    Requires ANSI C extensions: slash-slash comments, long external names.

--*/

#ifndef _LMREMUTL_
#define _LMREMUTL_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#ifdef __cplusplus
extern "C" {
#endif

//
// Type Definitions
//

#ifndef DESC_CHAR_UNICODE

typedef CHAR DESC_CHAR;
typedef LPSTR LPDESC;

#else // DESC_CHAR_UNICODE is defined

typedef WCHAR DESC_CHAR;
typedef LPWSTR LPDESC;

#endif // DESC_CHAR_UNICODE is defined



//
// Function Prototypes
//

_Check_return_
_Success_( return == 0 )
NET_API_STATUS NET_API_FUNCTION
NetRemoteTOD (
    _In_opt_ LPCWSTR UncServerName,
    _Outptr_result_bytebuffer_(sizeof(TIME_OF_DAY_INFO)) LPBYTE *BufferPtr
    );

NET_API_STATUS NET_API_FUNCTION
NetRemoteComputerSupports(
    IN LPCWSTR UncServerName OPTIONAL,   // Must start with "\\".
    IN DWORD OptionsWanted,             // Set SUPPORTS_ bits wanted.
    OUT LPDWORD OptionsSupported        // Supported features, masked.
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

NET_API_STATUS
__cdecl
RxRemoteApi(
    IN DWORD ApiNumber,
    _In_ IN LPCWSTR UncServerName,                    // Required, with \\name.
    _In_ IN LPDESC ParmDescString,
    _In_opt_ IN LPDESC DataDesc16 OPTIONAL,
    _In_opt_ IN LPDESC DataDesc32 OPTIONAL,
    _In_opt_ IN LPDESC DataDescSmb OPTIONAL,
    _In_opt_ IN LPDESC AuxDesc16 OPTIONAL,
    _In_opt_ IN LPDESC AuxDesc32 OPTIONAL,
    _In_opt_ IN LPDESC AuxDescSmb OPTIONAL,
    IN DWORD  Flags,
    ...                                         // rest of API's arguments
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

//
//  Data Structures
//

typedef struct _TIME_OF_DAY_INFO {
    DWORD      tod_elapsedt;
    DWORD      tod_msecs;
    DWORD      tod_hours;
    DWORD      tod_mins;
    DWORD      tod_secs;
    DWORD      tod_hunds;
    LONG       tod_timezone;
    DWORD      tod_tinterval;
    DWORD      tod_day;
    DWORD      tod_month;
    DWORD      tod_year;
    DWORD      tod_weekday;
} TIME_OF_DAY_INFO, *PTIME_OF_DAY_INFO, *LPTIME_OF_DAY_INFO;

//
// Special Values and Constants
//

//
// Mask bits for use with NetRemoteComputerSupports:
//

#define SUPPORTS_REMOTE_ADMIN_PROTOCOL  0x00000002L
#define SUPPORTS_RPC                    0x00000004L
#define SUPPORTS_SAM_PROTOCOL           0x00000008L
#define SUPPORTS_UNICODE                0x00000010L
#define SUPPORTS_LOCAL                  0x00000020L
#define SUPPORTS_ANY                    0xFFFFFFFFL

//
// Flag bits for RxRemoteApi:
//

#define NO_PERMISSION_REQUIRED  0x00000001      // set if use NULL session
#define ALLOCATE_RESPONSE       0x00000002      // set if RxRemoteApi allocates response buffer
#define USE_SPECIFIC_TRANSPORT  0x80000000

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif //_LMREMUTL_
