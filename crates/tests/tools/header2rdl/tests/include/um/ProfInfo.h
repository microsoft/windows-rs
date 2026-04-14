//=============================================================================
//  profinfo.h   -   Header file for profile info structure.
//
//  Copyright (c) Microsoft Corporation 2000
//  All rights reserved
//
//=============================================================================

#ifndef _INC_PROFINFO
#define _INC_PROFINFO

#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifdef __midl
#define FAR
#define MIDL_STRING [string, unique]
#else
#define MIDL_STRING
#endif  // __midl

typedef struct _PROFILEINFOA {
    DWORD       dwSize;                 // Set to sizeof(PROFILEINFO) before calling
    DWORD       dwFlags;                // See PI_ flags defined in userenv.h
    MIDL_STRING LPSTR       lpUserName;             // User name (required)
    MIDL_STRING LPSTR       lpProfilePath;          // Roaming profile path (optional, can be NULL)
    MIDL_STRING LPSTR       lpDefaultPath;          // Default user profile path (optional, can be NULL)
    MIDL_STRING LPSTR       lpServerName;           // Validating domain controller name in netbios format (optional, can be NULL but group NT4 style policy won't be applied)
    MIDL_STRING LPSTR       lpPolicyPath;           // Path to the NT4 style policy file (optional, can be NULL)
#ifdef __midl
    ULONG_PTR   hProfile;               // Filled in by the function.  Registry key handle open to the root.
#else
    HANDLE      hProfile;               // Filled in by the function.  Registry key handle open to the root.
#endif
    } PROFILEINFOA, FAR * LPPROFILEINFOA;
typedef struct _PROFILEINFOW {
    DWORD       dwSize;                 // Set to sizeof(PROFILEINFO) before calling
    DWORD       dwFlags;                // See PI_ flags defined in userenv.h
    MIDL_STRING LPWSTR      lpUserName;             // User name (required)
    MIDL_STRING LPWSTR      lpProfilePath;          // Roaming profile path (optional, can be NULL)
    MIDL_STRING LPWSTR      lpDefaultPath;          // Default user profile path (optional, can be NULL)
    MIDL_STRING LPWSTR      lpServerName;           // Validating domain controller name in netbios format (optional, can be NULL but group NT4 style policy won't be applied)
    MIDL_STRING LPWSTR      lpPolicyPath;           // Path to the NT4 style policy file (optional, can be NULL)
#ifdef __midl
    ULONG_PTR   hProfile;               // Filled in by the function.  Registry key handle open to the root.
#else
    HANDLE      hProfile;               // Filled in by the function.  Registry key handle open to the root.
#endif
    } PROFILEINFOW, FAR * LPPROFILEINFOW;
#ifdef UNICODE
typedef PROFILEINFOW PROFILEINFO;
typedef LPPROFILEINFOW LPPROFILEINFO;
#else
typedef PROFILEINFOA PROFILEINFO;
typedef LPPROFILEINFOA LPPROFILEINFO;
#endif // UNICODE


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif  // _INC_PROFINFO
