/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    Winreg.h

Abstract:

    This module contains the function prototypes and constant, type and
    structure definitions for the Windows 32-Bit Registry API.

--*/

#ifndef _WINREG_
#define _WINREG_

#include <winapifamily.h>

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#ifdef _MAC
#include <macwin32.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

#pragma region Application Family or Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifndef WINVER
#define WINVER 0x0500   // version 5.0
#endif /* !WINVER */

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

typedef _Return_type_success_(return==ERROR_SUCCESS) LONG LSTATUS;

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Application Family or Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

//
// RRF - Registry Routine Flags (for RegGetValue)
//

#define RRF_RT_REG_NONE        0x00000001  // restrict type to REG_NONE      (other data types will not return ERROR_SUCCESS)
#define RRF_RT_REG_SZ          0x00000002  // restrict type to REG_SZ        (other data types will not return ERROR_SUCCESS) (automatically converts REG_EXPAND_SZ to REG_SZ unless RRF_NOEXPAND is specified)
#define RRF_RT_REG_EXPAND_SZ   0x00000004  // restrict type to REG_EXPAND_SZ (other data types will not return ERROR_SUCCESS) (must specify RRF_NOEXPAND or RegGetValue will fail with ERROR_INVALID_PARAMETER)
#define RRF_RT_REG_BINARY      0x00000008  // restrict type to REG_BINARY    (other data types will not return ERROR_SUCCESS)
#define RRF_RT_REG_DWORD       0x00000010  // restrict type to REG_DWORD     (other data types will not return ERROR_SUCCESS)
#define RRF_RT_REG_MULTI_SZ    0x00000020  // restrict type to REG_MULTI_SZ  (other data types will not return ERROR_SUCCESS)
#define RRF_RT_REG_QWORD       0x00000040  // restrict type to REG_QWORD     (other data types will not return ERROR_SUCCESS)

#define RRF_RT_DWORD           (RRF_RT_REG_BINARY | RRF_RT_REG_DWORD) // restrict type to *32-bit* RRF_RT_REG_BINARY or RRF_RT_REG_DWORD (other data types will not return ERROR_SUCCESS)
#define RRF_RT_QWORD           (RRF_RT_REG_BINARY | RRF_RT_REG_QWORD) // restrict type to *64-bit* RRF_RT_REG_BINARY or RRF_RT_REG_DWORD (other data types will not return ERROR_SUCCESS)
#define RRF_RT_ANY             0x0000ffff                             // no type restriction

#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)
#define RRF_SUBKEY_WOW6464KEY  0x00010000  // when opening the subkey (if provided) force open from the 64bit location (only one SUBKEY_WOW64* flag can be set or RegGetValue will fail with ERROR_INVALID_PARAMETER)
#define RRF_SUBKEY_WOW6432KEY  0x00020000  // when opening the subkey (if provided) force open from the 32bit location (only one SUBKEY_WOW64* flag can be set or RegGetValue will fail with ERROR_INVALID_PARAMETER)
#define RRF_WOW64_MASK         0x00030000
#endif

#define RRF_NOEXPAND           0x10000000  // do not automatically expand environment strings if value is of type REG_EXPAND_SZ
#define RRF_ZEROONFAILURE      0x20000000  // if pvData is not NULL, set content to all zeros on failure

//
// Flags for RegLoadAppKey
//

#define REG_PROCESS_APPKEY                  0x00000001
#define REG_USE_CURRENT_SECURITY_CONTEXT    0x00000002

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// Requested Key access mask type.
//

typedef ACCESS_MASK REGSAM;

//
// Reserved Key Handles.
//

#define HKEY_CLASSES_ROOT                   (( HKEY ) (ULONG_PTR)((LONG)0x80000000) )
#define HKEY_CURRENT_USER                   (( HKEY ) (ULONG_PTR)((LONG)0x80000001) )
#define HKEY_LOCAL_MACHINE                  (( HKEY ) (ULONG_PTR)((LONG)0x80000002) )
#define HKEY_USERS                          (( HKEY ) (ULONG_PTR)((LONG)0x80000003) )
#define HKEY_PERFORMANCE_DATA               (( HKEY ) (ULONG_PTR)((LONG)0x80000004) )
#define HKEY_PERFORMANCE_TEXT               (( HKEY ) (ULONG_PTR)((LONG)0x80000050) )
#define HKEY_PERFORMANCE_NLSTEXT            (( HKEY ) (ULONG_PTR)((LONG)0x80000060) )
#if(WINVER >= 0x0400)
#define HKEY_CURRENT_CONFIG                 (( HKEY ) (ULONG_PTR)((LONG)0x80000005) )
#define HKEY_DYN_DATA                       (( HKEY ) (ULONG_PTR)((LONG)0x80000006) )
#define HKEY_CURRENT_USER_LOCAL_SETTINGS    (( HKEY ) (ULONG_PTR)((LONG)0x80000007) )

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Application Family or Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

/*NOINC*/
#ifndef _PROVIDER_STRUCTS_DEFINED
#define _PROVIDER_STRUCTS_DEFINED

#define PROVIDER_KEEPS_VALUE_LENGTH 0x1
struct val_context {
    int valuelen;       // the total length of this value
    LPVOID value_context;   // provider's context
    LPVOID val_buff_ptr;    // where in the ouput buffer the value is.
};

typedef struct val_context FAR *PVALCONTEXT;

typedef struct pvalueA {           // Provider supplied value/context.
    LPSTR   pv_valuename;          // The value name pointer
    int pv_valuelen;
    LPVOID pv_value_context;
    DWORD pv_type;
}PVALUEA, FAR *PPVALUEA;
typedef struct pvalueW {           // Provider supplied value/context.
    LPWSTR  pv_valuename;          // The value name pointer
    int pv_valuelen;
    LPVOID pv_value_context;
    DWORD pv_type;
}PVALUEW, FAR *PPVALUEW;
#ifdef UNICODE
typedef PVALUEW PVALUE;
typedef PPVALUEW PPVALUE;
#else
typedef PVALUEA PVALUE;
typedef PPVALUEA PPVALUE;
#endif // UNICODE

typedef
DWORD __cdecl
QUERYHANDLER (LPVOID keycontext, PVALCONTEXT val_list, DWORD num_vals,
          LPVOID outputbuffer, DWORD FAR *total_outlen, DWORD input_blen);

typedef QUERYHANDLER FAR *PQUERYHANDLER;

typedef struct provider_info {
    PQUERYHANDLER pi_R0_1val;
    PQUERYHANDLER pi_R0_allvals;
    PQUERYHANDLER pi_R3_1val;
    PQUERYHANDLER pi_R3_allvals;
    DWORD pi_flags;    // capability flags (none defined yet).
    LPVOID pi_key_context;
}REG_PROVIDER;

typedef struct provider_info FAR *PPROVIDER;

typedef struct value_entA {
    LPSTR   ve_valuename;
    DWORD ve_valuelen;
    DWORD_PTR ve_valueptr;
    DWORD ve_type;
}VALENTA, FAR *PVALENTA;
typedef struct value_entW {
    LPWSTR  ve_valuename;
    DWORD ve_valuelen;
    DWORD_PTR ve_valueptr;
    DWORD ve_type;
}VALENTW, FAR *PVALENTW;
#ifdef UNICODE
typedef VALENTW VALENT;
typedef PVALENTW PVALENT;
#else
typedef VALENTA VALENT;
typedef PVALENTA PVALENT;
#endif // UNICODE

#endif // not(_PROVIDER_STRUCTS_DEFINED)
/*INC*/

#endif /* WINVER >= 0x0400 */

//
// Default values for parameters that do not exist in the Win 3.1
// compatible APIs.
//

#define WIN31_CLASS                 NULL

//
// Flags for RegLoadMUIString
//
#define REG_MUI_STRING_TRUNCATE     0x00000001

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if(WINVER >= 0x0400)

//
// RegConnectRegistryEx supported flags
//
#define REG_SECURE_CONNECTION         0x1
#define REG_ALLOW_TRANSPORT_FALLBACK  0x2
#define REG_ALLOW_UNSECURE_CONNECTION 0x4

#endif /* WINVER >= 0x0400 */

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

//
// API Prototypes.
//

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINADVAPI
LSTATUS
APIENTRY
RegCloseKey(
    _In_ HKEY hKey
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
LSTATUS
APIENTRY
RegOverridePredefKey (
    _In_ HKEY hKey,
    _In_opt_ HKEY hNewHKey
    );

WINADVAPI
LSTATUS
APIENTRY
RegOpenUserClassesRoot(
    _In_ HANDLE hToken,
    _Reserved_ DWORD dwOptions,
    _In_ REGSAM samDesired,
    _Out_ PHKEY phkResult
    );

WINADVAPI
LSTATUS
APIENTRY
RegOpenCurrentUser(
    _In_ REGSAM samDesired,
    _Out_ PHKEY phkResult
    );

WINADVAPI
LSTATUS
APIENTRY
RegDisablePredefinedCache(
    VOID
    );

WINADVAPI
LSTATUS
APIENTRY
RegDisablePredefinedCacheEx(
    VOID
    );

WINADVAPI
LSTATUS
APIENTRY
RegConnectRegistryA (
    _In_opt_ LPCSTR lpMachineName,
    _In_ HKEY hKey,
    _Out_ PHKEY phkResult
    );
WINADVAPI
LSTATUS
APIENTRY
RegConnectRegistryW (
    _In_opt_ LPCWSTR lpMachineName,
    _In_ HKEY hKey,
    _Out_ PHKEY phkResult
    );
#ifdef UNICODE
#define RegConnectRegistry  RegConnectRegistryW
#else
#define RegConnectRegistry  RegConnectRegistryA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
LSTATUS
APIENTRY
RegConnectRegistryExA (
    _In_opt_ LPCSTR lpMachineName,
    _In_ HKEY hKey,
    _In_ ULONG Flags,
    _Out_ PHKEY phkResult
    );
WINADVAPI
LSTATUS
APIENTRY
RegConnectRegistryExW (
    _In_opt_ LPCWSTR lpMachineName,
    _In_ HKEY hKey,
    _In_ ULONG Flags,
    _Out_ PHKEY phkResult
    );
#ifdef UNICODE
#define RegConnectRegistryEx  RegConnectRegistryExW
#else
#define RegConnectRegistryEx  RegConnectRegistryExA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINADVAPI
LSTATUS
APIENTRY
RegCreateKeyA (
    _In_ HKEY hKey,
    _In_opt_ LPCSTR lpSubKey,
    _Out_ PHKEY phkResult
    );
WINADVAPI
LSTATUS
APIENTRY
RegCreateKeyW (
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR lpSubKey,
    _Out_ PHKEY phkResult
    );
#ifdef UNICODE
#define RegCreateKey  RegCreateKeyW
#else
#define RegCreateKey  RegCreateKeyA
#endif // !UNICODE

WINADVAPI
LSTATUS
APIENTRY
RegCreateKeyExA(
    _In_ HKEY hKey,
    _In_ LPCSTR lpSubKey,
    _Reserved_ DWORD Reserved,
    _In_opt_ LPSTR lpClass,
    _In_ DWORD dwOptions,
    _In_ REGSAM samDesired,
    _In_opt_ CONST LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    _Out_ PHKEY phkResult,
    _Out_opt_ LPDWORD lpdwDisposition
    );

WINADVAPI
LSTATUS
APIENTRY
RegCreateKeyExW(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpSubKey,
    _Reserved_ DWORD Reserved,
    _In_opt_ LPWSTR lpClass,
    _In_ DWORD dwOptions,
    _In_ REGSAM samDesired,
    _In_opt_ CONST LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    _Out_ PHKEY phkResult,
    _Out_opt_ LPDWORD lpdwDisposition
    );

#ifdef UNICODE
#define RegCreateKeyEx  RegCreateKeyExW
#else
#define RegCreateKeyEx  RegCreateKeyExA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
LSTATUS
APIENTRY
RegCreateKeyTransactedA (
    _In_ HKEY hKey,
    _In_ LPCSTR lpSubKey,
    _Reserved_ DWORD Reserved,
    _In_opt_ LPSTR lpClass,
    _In_ DWORD dwOptions,
    _In_ REGSAM samDesired,
    _In_opt_ CONST LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    _Out_ PHKEY phkResult,
    _Out_opt_ LPDWORD lpdwDisposition,
    _In_        HANDLE hTransaction,
    _Reserved_ PVOID  pExtendedParemeter
    );
WINADVAPI
LSTATUS
APIENTRY
RegCreateKeyTransactedW (
    _In_ HKEY hKey,
    _In_ LPCWSTR lpSubKey,
    _Reserved_ DWORD Reserved,
    _In_opt_ LPWSTR lpClass,
    _In_ DWORD dwOptions,
    _In_ REGSAM samDesired,
    _In_opt_ CONST LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    _Out_ PHKEY phkResult,
    _Out_opt_ LPDWORD lpdwDisposition,
    _In_        HANDLE hTransaction,
    _Reserved_ PVOID  pExtendedParemeter
    );
#ifdef UNICODE
#define RegCreateKeyTransacted  RegCreateKeyTransactedW
#else
#define RegCreateKeyTransacted  RegCreateKeyTransactedA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINADVAPI
LSTATUS
APIENTRY
RegDeleteKeyA (
    _In_ HKEY hKey,
    _In_ LPCSTR lpSubKey
    );
WINADVAPI
LSTATUS
APIENTRY
RegDeleteKeyW (
    _In_ HKEY hKey,
    _In_ LPCWSTR lpSubKey
    );
#ifdef UNICODE
#define RegDeleteKey  RegDeleteKeyW
#else
#define RegDeleteKey  RegDeleteKeyA
#endif // !UNICODE

WINADVAPI
LSTATUS
APIENTRY
RegDeleteKeyExA(
    _In_ HKEY hKey,
    _In_ LPCSTR lpSubKey,
    _In_ REGSAM samDesired,
    _Reserved_ DWORD Reserved
    );

WINADVAPI
LSTATUS
APIENTRY
RegDeleteKeyExW(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpSubKey,
    _In_ REGSAM samDesired,
    _Reserved_ DWORD Reserved
    );

#ifdef UNICODE
#define RegDeleteKeyEx  RegDeleteKeyExW
#else
#define RegDeleteKeyEx  RegDeleteKeyExA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
LSTATUS
APIENTRY
RegDeleteKeyTransactedA (
    _In_ HKEY hKey,
    _In_ LPCSTR lpSubKey,
    _In_ REGSAM samDesired,
    _Reserved_ DWORD Reserved,
    _In_        HANDLE hTransaction,
    _Reserved_ PVOID  pExtendedParameter
    );
WINADVAPI
LSTATUS
APIENTRY
RegDeleteKeyTransactedW (
    _In_ HKEY hKey,
    _In_ LPCWSTR lpSubKey,
    _In_ REGSAM samDesired,
    _Reserved_ DWORD Reserved,
    _In_        HANDLE hTransaction,
    _Reserved_ PVOID  pExtendedParameter
    );
#ifdef UNICODE
#define RegDeleteKeyTransacted  RegDeleteKeyTransactedW
#else
#define RegDeleteKeyTransacted  RegDeleteKeyTransactedA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
LONG
APIENTRY
RegDisableReflectionKey (
    _In_ HKEY hBase
    );

WINADVAPI
LONG
APIENTRY
RegEnableReflectionKey (
    _In_ HKEY hBase
    );

WINADVAPI
LONG
APIENTRY
RegQueryReflectionKey (
    _In_ HKEY hBase,
    _Out_ BOOL *bIsReflectionDisabled
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINADVAPI
LSTATUS
APIENTRY
RegDeleteValueA(
    _In_ HKEY hKey,
    _In_opt_ LPCSTR lpValueName
    );

WINADVAPI
LSTATUS
APIENTRY
RegDeleteValueW(
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR lpValueName
    );

#ifdef UNICODE
#define RegDeleteValue  RegDeleteValueW
#else
#define RegDeleteValue  RegDeleteValueA
#endif // !UNICODE

WINADVAPI
LSTATUS
APIENTRY
RegEnumKeyA (
    _In_ HKEY hKey,
    _In_ DWORD dwIndex,
    _Out_writes_opt_(cchName) LPSTR lpName,
    _In_ DWORD cchName
    );
WINADVAPI
LSTATUS
APIENTRY
RegEnumKeyW (
    _In_ HKEY hKey,
    _In_ DWORD dwIndex,
    _Out_writes_opt_(cchName) LPWSTR lpName,
    _In_ DWORD cchName
    );
#ifdef UNICODE
#define RegEnumKey  RegEnumKeyW
#else
#define RegEnumKey  RegEnumKeyA
#endif // !UNICODE

WINADVAPI
LSTATUS
APIENTRY
RegEnumKeyExA(
    _In_ HKEY hKey,
    _In_ DWORD dwIndex,
    _Out_writes_to_opt_(*lpcchName, *lpcchName + 1) LPSTR lpName,
    _Inout_ LPDWORD lpcchName,
    _Reserved_ LPDWORD lpReserved,
    _Out_writes_to_opt_(*lpcchClass,*lpcchClass + 1) LPSTR lpClass,
    _Inout_opt_ LPDWORD lpcchClass,
    _Out_opt_ PFILETIME lpftLastWriteTime
    );

WINADVAPI
LSTATUS
APIENTRY
RegEnumKeyExW(
    _In_ HKEY hKey,
    _In_ DWORD dwIndex,
    _Out_writes_to_opt_(*lpcchName, *lpcchName + 1) LPWSTR lpName,
    _Inout_ LPDWORD lpcchName,
    _Reserved_ LPDWORD lpReserved,
    _Out_writes_to_opt_(*lpcchClass,*lpcchClass + 1) LPWSTR lpClass,
    _Inout_opt_ LPDWORD lpcchClass,
    _Out_opt_ PFILETIME lpftLastWriteTime
    );

#ifdef UNICODE
#define RegEnumKeyEx  RegEnumKeyExW
#else
#define RegEnumKeyEx  RegEnumKeyExA
#endif // !UNICODE

WINADVAPI
LSTATUS
APIENTRY
RegEnumValueA(
    _In_ HKEY hKey,
    _In_ DWORD dwIndex,
    _Out_writes_to_opt_(*lpcchValueName, *lpcchValueName + 1) LPSTR lpValueName,
    _Inout_ LPDWORD lpcchValueName,
    _Reserved_ LPDWORD lpReserved,
    _Out_opt_ LPDWORD lpType,
    _Out_writes_bytes_to_opt_(*lpcbData, *lpcbData) __out_data_source(REGISTRY) LPBYTE lpData,
    _Inout_opt_ LPDWORD lpcbData
    );

WINADVAPI
LSTATUS
APIENTRY
RegEnumValueW(
    _In_ HKEY hKey,
    _In_ DWORD dwIndex,
    _Out_writes_to_opt_(*lpcchValueName, *lpcchValueName + 1) LPWSTR lpValueName,
    _Inout_ LPDWORD lpcchValueName,
    _Reserved_ LPDWORD lpReserved,
    _Out_opt_ LPDWORD lpType,
    _Out_writes_bytes_to_opt_(*lpcbData, *lpcbData) __out_data_source(REGISTRY) LPBYTE lpData,
    _Inout_opt_ LPDWORD lpcbData
    );

#ifdef UNICODE
#define RegEnumValue  RegEnumValueW
#else
#define RegEnumValue  RegEnumValueA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
LSTATUS
APIENTRY
RegFlushKey(
    _In_ HKEY hKey
    );

WINADVAPI
LSTATUS
APIENTRY
RegGetKeySecurity(
    _In_ HKEY hKey,
    _In_ SECURITY_INFORMATION SecurityInformation,
    _Out_writes_bytes_opt_(*lpcbSecurityDescriptor) PSECURITY_DESCRIPTOR pSecurityDescriptor,
    _Inout_ LPDWORD lpcbSecurityDescriptor
    );

WINADVAPI
LSTATUS
APIENTRY
RegLoadKeyA(
    _In_ HKEY hKey,
    _In_opt_ LPCSTR lpSubKey,
    _In_ LPCSTR lpFile
    );

WINADVAPI
LSTATUS
APIENTRY
RegLoadKeyW(
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR lpSubKey,
    _In_ LPCWSTR lpFile
    );

#ifdef UNICODE
#define RegLoadKey  RegLoadKeyW
#else
#define RegLoadKey  RegLoadKeyA
#endif // !UNICODE

WINADVAPI
LSTATUS
APIENTRY
RegNotifyChangeKeyValue(
    _In_ HKEY hKey,
    _In_ BOOL bWatchSubtree,
    _In_ DWORD dwNotifyFilter,
    _In_opt_ HANDLE hEvent,
    _In_ BOOL fAsynchronous
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINADVAPI
LSTATUS
APIENTRY
RegOpenKeyA (
    _In_ HKEY hKey,
    _In_opt_ LPCSTR lpSubKey,
    _Out_ PHKEY phkResult
    );
WINADVAPI
LSTATUS
APIENTRY
RegOpenKeyW (
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR lpSubKey,
    _Out_ PHKEY phkResult
    );
#ifdef UNICODE
#define RegOpenKey  RegOpenKeyW
#else
#define RegOpenKey  RegOpenKeyA
#endif // !UNICODE

WINADVAPI
LSTATUS
APIENTRY
RegOpenKeyExA(
    _In_ HKEY hKey,
    _In_opt_ LPCSTR lpSubKey,
    _In_opt_ DWORD ulOptions,
    _In_ REGSAM samDesired,
    _Out_ PHKEY phkResult
    );

WINADVAPI
LSTATUS
APIENTRY
RegOpenKeyExW(
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR lpSubKey,
    _In_opt_ DWORD ulOptions,
    _In_ REGSAM samDesired,
    _Out_ PHKEY phkResult
    );

#ifdef UNICODE
#define RegOpenKeyEx  RegOpenKeyExW
#else
#define RegOpenKeyEx  RegOpenKeyExA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
LSTATUS
APIENTRY
RegOpenKeyTransactedA (
    _In_ HKEY hKey,
    _In_opt_ LPCSTR lpSubKey,
    _In_opt_ DWORD ulOptions,
    _In_ REGSAM samDesired,
    _Out_ PHKEY phkResult,
    _In_        HANDLE hTransaction,
    _Reserved_ PVOID  pExtendedParemeter
    );
WINADVAPI
LSTATUS
APIENTRY
RegOpenKeyTransactedW (
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR lpSubKey,
    _In_opt_ DWORD ulOptions,
    _In_ REGSAM samDesired,
    _Out_ PHKEY phkResult,
    _In_        HANDLE hTransaction,
    _Reserved_ PVOID  pExtendedParemeter
    );
#ifdef UNICODE
#define RegOpenKeyTransacted  RegOpenKeyTransactedW
#else
#define RegOpenKeyTransacted  RegOpenKeyTransactedA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINADVAPI
LSTATUS
APIENTRY
RegQueryInfoKeyA(
    _In_ HKEY hKey,
    _Out_writes_to_opt_(*lpcchClass, *lpcchClass + 1) LPSTR lpClass,
    _Inout_opt_ LPDWORD lpcchClass,
    _Reserved_ LPDWORD lpReserved,
    _Out_opt_ LPDWORD lpcSubKeys,
    _Out_opt_ LPDWORD lpcbMaxSubKeyLen,
    _Out_opt_ LPDWORD lpcbMaxClassLen,
    _Out_opt_ LPDWORD lpcValues,
    _Out_opt_ LPDWORD lpcbMaxValueNameLen,
    _Out_opt_ LPDWORD lpcbMaxValueLen,
    _Out_opt_ LPDWORD lpcbSecurityDescriptor,
    _Out_opt_ PFILETIME lpftLastWriteTime
    );

WINADVAPI
LSTATUS
APIENTRY
RegQueryInfoKeyW(
    _In_ HKEY hKey,
    _Out_writes_to_opt_(*lpcchClass, *lpcchClass + 1) LPWSTR lpClass,
    _Inout_opt_ LPDWORD lpcchClass,
    _Reserved_ LPDWORD lpReserved,
    _Out_opt_ LPDWORD lpcSubKeys,
    _Out_opt_ LPDWORD lpcbMaxSubKeyLen,
    _Out_opt_ LPDWORD lpcbMaxClassLen,
    _Out_opt_ LPDWORD lpcValues,
    _Out_opt_ LPDWORD lpcbMaxValueNameLen,
    _Out_opt_ LPDWORD lpcbMaxValueLen,
    _Out_opt_ LPDWORD lpcbSecurityDescriptor,
    _Out_opt_ PFILETIME lpftLastWriteTime
    );

#ifdef UNICODE
#define RegQueryInfoKey  RegQueryInfoKeyW
#else
#define RegQueryInfoKey  RegQueryInfoKeyA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
LSTATUS
APIENTRY
RegQueryValueA (
    _In_ HKEY hKey,
    _In_opt_ LPCSTR lpSubKey,
    _Out_writes_bytes_to_opt_(*lpcbData, *lpcbData) __out_data_source(REGISTRY) LPSTR lpData,
    _Inout_opt_ PLONG lpcbData
    );
WINADVAPI
LSTATUS
APIENTRY
RegQueryValueW (
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR lpSubKey,
    _Out_writes_bytes_to_opt_(*lpcbData, *lpcbData) __out_data_source(REGISTRY) LPWSTR lpData,
    _Inout_opt_ PLONG lpcbData
    );
#ifdef UNICODE
#define RegQueryValue  RegQueryValueW
#else
#define RegQueryValue  RegQueryValueA
#endif // !UNICODE

#if(WINVER >= 0x0400)

WINADVAPI
LSTATUS
APIENTRY
RegQueryMultipleValuesA(
    _In_ HKEY hKey,
    _Out_writes_(num_vals) PVALENTA val_list,
    _In_ DWORD num_vals,
    _Out_writes_bytes_to_opt_(*ldwTotsize, *ldwTotsize) __out_data_source(REGISTRY) LPSTR lpValueBuf,
    _Inout_opt_ LPDWORD ldwTotsize
    );

WINADVAPI
LSTATUS
APIENTRY
RegQueryMultipleValuesW(
    _In_ HKEY hKey,
    _Out_writes_(num_vals) PVALENTW val_list,
    _In_ DWORD num_vals,
    _Out_writes_bytes_to_opt_(*ldwTotsize, *ldwTotsize) __out_data_source(REGISTRY) LPWSTR lpValueBuf,
    _Inout_opt_ LPDWORD ldwTotsize
    );

#ifdef UNICODE
#define RegQueryMultipleValues  RegQueryMultipleValuesW
#else
#define RegQueryMultipleValues  RegQueryMultipleValuesA
#endif // !UNICODE

#endif /* WINVER >= 0x0400 */

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINADVAPI
LSTATUS
APIENTRY
RegQueryValueExA(
    _In_ HKEY hKey,
    _In_opt_ LPCSTR lpValueName,
    _Reserved_ LPDWORD lpReserved,
    _Out_opt_ LPDWORD lpType,
    _Out_writes_bytes_to_opt_(*lpcbData, *lpcbData) __out_data_source(REGISTRY) LPBYTE lpData,
    _When_(lpData == NULL, _Out_opt_) _When_(lpData != NULL, _Inout_opt_) LPDWORD lpcbData
    );

WINADVAPI
LSTATUS
APIENTRY
RegQueryValueExW(
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR lpValueName,
    _Reserved_ LPDWORD lpReserved,
    _Out_opt_ LPDWORD lpType,
    _Out_writes_bytes_to_opt_(*lpcbData, *lpcbData) __out_data_source(REGISTRY) LPBYTE lpData,
    _When_(lpData == NULL, _Out_opt_) _When_(lpData != NULL, _Inout_opt_) LPDWORD lpcbData
    );

#ifdef UNICODE
#define RegQueryValueEx  RegQueryValueExW
#else
#define RegQueryValueEx  RegQueryValueExA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
LSTATUS
APIENTRY
RegReplaceKeyA (
    _In_ HKEY hKey,
    _In_opt_ LPCSTR lpSubKey,
    _In_ LPCSTR lpNewFile,
    _In_ LPCSTR lpOldFile
    );
WINADVAPI
LSTATUS
APIENTRY
RegReplaceKeyW (
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR lpSubKey,
    _In_ LPCWSTR lpNewFile,
    _In_ LPCWSTR lpOldFile
    );
#ifdef UNICODE
#define RegReplaceKey  RegReplaceKeyW
#else
#define RegReplaceKey  RegReplaceKeyA
#endif // !UNICODE

WINADVAPI
LSTATUS
APIENTRY
RegRestoreKeyA(
    _In_ HKEY hKey,
    _In_ LPCSTR lpFile,
    _In_ DWORD dwFlags
    );

WINADVAPI
LSTATUS
APIENTRY
RegRestoreKeyW(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpFile,
    _In_ DWORD dwFlags
    );

#ifdef UNICODE
#define RegRestoreKey  RegRestoreKeyW
#else
#define RegRestoreKey  RegRestoreKeyA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if(WINVER >= 0x0600)

WINADVAPI
LSTATUS
APIENTRY
RegRenameKey(
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR lpSubKeyName,
    _In_ LPCWSTR lpNewKeyName
    );

#endif /* WINVER >= 0x0600 */

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
LSTATUS
APIENTRY
RegSaveKeyA (
    _In_ HKEY hKey,
    _In_ LPCSTR lpFile,
    _In_opt_ CONST LPSECURITY_ATTRIBUTES lpSecurityAttributes
    );
WINADVAPI
LSTATUS
APIENTRY
RegSaveKeyW (
    _In_ HKEY hKey,
    _In_ LPCWSTR lpFile,
    _In_opt_ CONST LPSECURITY_ATTRIBUTES lpSecurityAttributes
    );
#ifdef UNICODE
#define RegSaveKey  RegSaveKeyW
#else
#define RegSaveKey  RegSaveKeyA
#endif // !UNICODE

WINADVAPI
LSTATUS
APIENTRY
RegSetKeySecurity(
    _In_ HKEY hKey,
    _In_ SECURITY_INFORMATION SecurityInformation,
    _In_ PSECURITY_DESCRIPTOR pSecurityDescriptor
    );

WINADVAPI
LSTATUS
APIENTRY
RegSetValueA (
    _In_ HKEY hKey,
    _In_opt_ LPCSTR lpSubKey,
    _In_ DWORD dwType,
    _In_reads_bytes_opt_(cbData) LPCSTR lpData,
    _In_ DWORD cbData
    );
WINADVAPI
LSTATUS
APIENTRY
RegSetValueW (
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR lpSubKey,
    _In_ DWORD dwType,
    _In_reads_bytes_opt_(cbData) LPCWSTR lpData,
    _In_ DWORD cbData
    );
#ifdef UNICODE
#define RegSetValue  RegSetValueW
#else
#define RegSetValue  RegSetValueA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINADVAPI
LSTATUS
APIENTRY
RegSetValueExA(
    _In_ HKEY hKey,
    _In_opt_ LPCSTR lpValueName,
    _Reserved_ DWORD Reserved,
    _In_ DWORD dwType,
    _In_reads_bytes_opt_(cbData) CONST BYTE* lpData,
    _In_ DWORD cbData
    );

WINADVAPI
LSTATUS
APIENTRY
RegSetValueExW(
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR lpValueName,
    _Reserved_ DWORD Reserved,
    _In_ DWORD dwType,
    _In_reads_bytes_opt_(cbData) CONST BYTE* lpData,
    _In_ DWORD cbData
    );

#ifdef UNICODE
#define RegSetValueEx  RegSetValueExW
#else
#define RegSetValueEx  RegSetValueExA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
LSTATUS
APIENTRY
RegUnLoadKeyA(
    _In_ HKEY hKey,
    _In_opt_ LPCSTR lpSubKey
    );

WINADVAPI
LSTATUS
APIENTRY
RegUnLoadKeyW(
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR lpSubKey
    );

#ifdef UNICODE
#define RegUnLoadKey  RegUnLoadKeyW
#else
#define RegUnLoadKey  RegUnLoadKeyA
#endif // !UNICODE

//
// Utils wrappers
//
#if _WIN32_WINNT >= 0x0600

WINADVAPI
LSTATUS
APIENTRY
RegDeleteKeyValueA(
    _In_ HKEY hKey,
    _In_opt_ LPCSTR lpSubKey,
    _In_opt_ LPCSTR lpValueName
    );

WINADVAPI
LSTATUS
APIENTRY
RegDeleteKeyValueW(
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR lpSubKey,
    _In_opt_ LPCWSTR lpValueName
    );

#ifdef UNICODE
#define RegDeleteKeyValue  RegDeleteKeyValueW
#else
#define RegDeleteKeyValue  RegDeleteKeyValueA
#endif // !UNICODE

WINADVAPI
LSTATUS
APIENTRY
RegSetKeyValueA(
    _In_ HKEY hKey,
    _In_opt_ LPCSTR lpSubKey,
    _In_opt_ LPCSTR lpValueName,
    _In_ DWORD dwType,
    _In_reads_bytes_opt_(cbData) LPCVOID lpData,
    _In_ DWORD cbData
    );

WINADVAPI
LSTATUS
APIENTRY
RegSetKeyValueW(
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR lpSubKey,
    _In_opt_ LPCWSTR lpValueName,
    _In_ DWORD dwType,
    _In_reads_bytes_opt_(cbData) LPCVOID lpData,
    _In_ DWORD cbData
    );

#ifdef UNICODE
#define RegSetKeyValue  RegSetKeyValueW
#else
#define RegSetKeyValue  RegSetKeyValueA
#endif // !UNICODE

WINADVAPI
LSTATUS
APIENTRY
RegDeleteTreeA(
    _In_ HKEY hKey,
    _In_opt_ LPCSTR lpSubKey
    );

WINADVAPI
LSTATUS
APIENTRY
RegDeleteTreeW(
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR lpSubKey
    );

#ifdef UNICODE
#define RegDeleteTree  RegDeleteTreeW
#else
#define RegDeleteTree  RegDeleteTreeA
#endif // !UNICODE

WINADVAPI
LSTATUS
APIENTRY
RegCopyTreeA (
    _In_        HKEY     hKeySrc,
    _In_opt_    LPCSTR  lpSubKey,
    _In_        HKEY     hKeyDest
    );
#ifndef UNICODE
#define RegCopyTree  RegCopyTreeA
#endif // !UNICODE

#endif // _WIN32_WINNT >= 0x0600

#if (_WIN32_WINNT >= 0x0502)

WINADVAPI
LSTATUS
APIENTRY
RegGetValueA(
    _In_ HKEY hkey,
    _In_opt_ LPCSTR lpSubKey,
    _In_opt_ LPCSTR lpValue,
    _In_ DWORD dwFlags,
    _Out_opt_ LPDWORD pdwType,
    _When_((dwFlags & 0x7F) == RRF_RT_REG_SZ ||
               (dwFlags & 0x7F) == RRF_RT_REG_EXPAND_SZ ||
               (dwFlags & 0x7F) == (RRF_RT_REG_SZ | RRF_RT_REG_EXPAND_SZ) ||
               *pdwType == REG_SZ ||
               *pdwType == REG_EXPAND_SZ, _Post_z_)
        _When_((dwFlags & 0x7F) == RRF_RT_REG_MULTI_SZ ||
               *pdwType == REG_MULTI_SZ, _Post_ _NullNull_terminated_)
    _Out_writes_bytes_to_opt_(*pcbData,*pcbData) PVOID pvData,
    _Inout_opt_ LPDWORD pcbData
    );

WINADVAPI
LSTATUS
APIENTRY
RegGetValueW(
    _In_ HKEY hkey,
    _In_opt_ LPCWSTR lpSubKey,
    _In_opt_ LPCWSTR lpValue,
    _In_ DWORD dwFlags,
    _Out_opt_ LPDWORD pdwType,
    _When_((dwFlags & 0x7F) == RRF_RT_REG_SZ ||
               (dwFlags & 0x7F) == RRF_RT_REG_EXPAND_SZ ||
               (dwFlags & 0x7F) == (RRF_RT_REG_SZ | RRF_RT_REG_EXPAND_SZ) ||
               *pdwType == REG_SZ ||
               *pdwType == REG_EXPAND_SZ, _Post_z_)
        _When_((dwFlags & 0x7F) == RRF_RT_REG_MULTI_SZ ||
               *pdwType == REG_MULTI_SZ, _Post_ _NullNull_terminated_)
    _Out_writes_bytes_to_opt_(*pcbData,*pcbData) PVOID pvData,
    _Inout_opt_ LPDWORD pcbData
    );

#ifdef UNICODE
#define RegGetValue  RegGetValueW
#else
#define RegGetValue  RegGetValueA
#endif // !UNICODE

#endif // (_WIN32_WINNT >= 0x0502)

#if (_WIN32_WINNT >= 0x0600)

WINADVAPI
LSTATUS
APIENTRY
RegCopyTreeW(
    _In_ HKEY hKeySrc,
    _In_opt_ LPCWSTR lpSubKey,
    _In_ HKEY hKeyDest
    );

#ifdef UNICODE
#define RegCopyTree  RegCopyTreeW
#endif

WINADVAPI
LSTATUS
APIENTRY
RegLoadMUIStringA(
    _In_ HKEY hKey,
    _In_opt_ LPCSTR pszValue,
    _Out_writes_bytes_opt_(cbOutBuf) LPSTR pszOutBuf,
    _In_ DWORD cbOutBuf,
    _Out_opt_ LPDWORD pcbData,
    _In_ DWORD Flags,
    _In_opt_ LPCSTR pszDirectory
    );

WINADVAPI
LSTATUS
APIENTRY
RegLoadMUIStringW(
    _In_ HKEY hKey,
    _In_opt_ LPCWSTR pszValue,
    _Out_writes_bytes_opt_(cbOutBuf) LPWSTR pszOutBuf,
    _In_ DWORD cbOutBuf,
    _Out_opt_ LPDWORD pcbData,
    _In_ DWORD Flags,
    _In_opt_ LPCWSTR pszDirectory
    );

#ifdef UNICODE
#define RegLoadMUIString  RegLoadMUIStringW
#else
#define RegLoadMUIString  RegLoadMUIStringA
#endif // !UNICODE

WINADVAPI
LSTATUS
APIENTRY
RegLoadAppKeyA(
    _In_ LPCSTR lpFile,
    _Out_ PHKEY phkResult,
    _In_ REGSAM samDesired,
    _In_ DWORD dwOptions,
    _Reserved_ DWORD Reserved
    );

WINADVAPI
LSTATUS
APIENTRY
RegLoadAppKeyW(
    _In_ LPCWSTR lpFile,
    _Out_ PHKEY phkResult,
    _In_ REGSAM samDesired,
    _In_ DWORD dwOptions,
    _Reserved_ DWORD Reserved
    );

#ifdef UNICODE
#define RegLoadAppKey  RegLoadAppKeyW
#else
#define RegLoadAppKey  RegLoadAppKeyA
#endif // !UNICODE

#endif // _WIN32_WINNT >= 0x0600

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

//
// Remoteable System Shutdown APIs
//

__drv_preferredFunction("InitiateSystemShutdownEx", "Legacy API. Rearchitect to avoid Reboot")
WINADVAPI
BOOL
APIENTRY
InitiateSystemShutdownA(
    _In_opt_ LPSTR lpMachineName,
    _In_opt_ LPSTR lpMessage,
    _In_ DWORD dwTimeout,
    _In_ BOOL bForceAppsClosed,
    _In_ BOOL bRebootAfterShutdown
    );
__drv_preferredFunction("InitiateSystemShutdownEx", "Legacy API. Rearchitect to avoid Reboot")
WINADVAPI
BOOL
APIENTRY
InitiateSystemShutdownW(
    _In_opt_ LPWSTR lpMachineName,
    _In_opt_ LPWSTR lpMessage,
    _In_ DWORD dwTimeout,
    _In_ BOOL bForceAppsClosed,
    _In_ BOOL bRebootAfterShutdown
    );
#ifdef UNICODE
#define InitiateSystemShutdown  InitiateSystemShutdownW
#else
#define InitiateSystemShutdown  InitiateSystemShutdownA
#endif // !UNICODE

WINADVAPI
BOOL
APIENTRY
AbortSystemShutdownA(
    _In_opt_ LPSTR lpMachineName
    );
WINADVAPI
BOOL
APIENTRY
AbortSystemShutdownW(
    _In_opt_ LPWSTR lpMachineName
    );
#ifdef UNICODE
#define AbortSystemShutdown  AbortSystemShutdownW
#else
#define AbortSystemShutdown  AbortSystemShutdownA
#endif // !UNICODE

//
// defines for InitiateSystemShutdownEx reason codes
//

#include <reason.h>             // get the public reasons
//
// Then for Historical reasons support some old symbols, internal only

#define REASON_SWINSTALL    (SHTDN_REASON_MAJOR_SOFTWARE|SHTDN_REASON_MINOR_INSTALLATION)
#define REASON_HWINSTALL    (SHTDN_REASON_MAJOR_HARDWARE|SHTDN_REASON_MINOR_INSTALLATION)
#define REASON_SERVICEHANG  (SHTDN_REASON_MAJOR_SOFTWARE|SHTDN_REASON_MINOR_HUNG)
#define REASON_UNSTABLE     (SHTDN_REASON_MAJOR_SYSTEM|SHTDN_REASON_MINOR_UNSTABLE)
#define REASON_SWHWRECONF   (SHTDN_REASON_MAJOR_SOFTWARE|SHTDN_REASON_MINOR_RECONFIG)
#define REASON_OTHER        (SHTDN_REASON_MAJOR_OTHER|SHTDN_REASON_MINOR_OTHER)
#define REASON_UNKNOWN      SHTDN_REASON_UNKNOWN
#define REASON_LEGACY_API   SHTDN_REASON_LEGACY_API
#define REASON_PLANNED_FLAG SHTDN_REASON_FLAG_PLANNED

//
// MAX Shutdown TimeOut == 10 Years in seconds
//
#define MAX_SHUTDOWN_TIMEOUT (10*365*24*60*60)

__drv_preferredFunction("a design alternative", "Rearchitect to avoid Reboot")
_When_(((dwReason==0 && lpMessage==0)) || dwReason>=0xd0000000,
    __drv_reportError("Requires a valid dwReason or lpMessage"))
WINADVAPI
BOOL
APIENTRY
InitiateSystemShutdownExA(
    _In_opt_ LPSTR lpMachineName,
    _In_opt_ LPSTR lpMessage,
    _In_ DWORD dwTimeout,
    _In_ BOOL bForceAppsClosed,
    _In_ BOOL bRebootAfterShutdown,
    _In_ DWORD dwReason
    );
__drv_preferredFunction("a design alternative", "Rearchitect to avoid Reboot")
_When_(((dwReason==0 && lpMessage==0)) || dwReason>=0xd0000000,
    __drv_reportError("Requires a valid dwReason or lpMessage"))
WINADVAPI
BOOL
APIENTRY
InitiateSystemShutdownExW(
    _In_opt_ LPWSTR lpMachineName,
    _In_opt_ LPWSTR lpMessage,
    _In_ DWORD dwTimeout,
    _In_ BOOL bForceAppsClosed,
    _In_ BOOL bRebootAfterShutdown,
    _In_ DWORD dwReason
    );
#ifdef UNICODE
#define InitiateSystemShutdownEx  InitiateSystemShutdownExW
#else
#define InitiateSystemShutdownEx  InitiateSystemShutdownExA
#endif // !UNICODE

//
// Shutdown flags
//

#define SHUTDOWN_FORCE_OTHERS           0x00000001
#define SHUTDOWN_FORCE_SELF             0x00000002
#define SHUTDOWN_RESTART                0x00000004
#define SHUTDOWN_POWEROFF               0x00000008
#define SHUTDOWN_NOREBOOT               0x00000010
#define SHUTDOWN_GRACE_OVERRIDE         0x00000020
#define SHUTDOWN_INSTALL_UPDATES        0x00000040
#define SHUTDOWN_RESTARTAPPS            0x00000080
#define SHUTDOWN_SKIP_SVC_PRESHUTDOWN   0x00000100
#define SHUTDOWN_HYBRID                 0x00000200
#define SHUTDOWN_RESTART_BOOTOPTIONS    0x00000400
#define SHUTDOWN_SOFT_REBOOT            0x00000800
#define SHUTDOWN_MOBILE_UI              0x00001000
#define SHUTDOWN_ARSO                   0x00002000
#define SHUTDOWN_CHECK_SAFE_FOR_SERVER  0x00004000
#define SHUTDOWN_VAIL_CONTAINER         0x00008000
#define SHUTDOWN_SYSTEM_INITIATED       0x00010000
#define SHUTDOWN_UPDATE_POWEROFF        0x00020000

WINADVAPI
DWORD
APIENTRY
InitiateShutdownA(
    _In_opt_ LPSTR lpMachineName,
    _In_opt_ LPSTR lpMessage,
    _In_     DWORD dwGracePeriod,
    _In_     DWORD dwShutdownFlags,
    _In_     DWORD dwReason
    );
WINADVAPI
DWORD
APIENTRY
InitiateShutdownW(
    _In_opt_ LPWSTR lpMachineName,
    _In_opt_ LPWSTR lpMessage,
    _In_     DWORD dwGracePeriod,
    _In_     DWORD dwShutdownFlags,
    _In_     DWORD dwReason
    );
#ifdef UNICODE
#define InitiateShutdown  InitiateShutdownW
#else
#define InitiateShutdown  InitiateShutdownA
#endif // !UNICODE

WINADVAPI
DWORD
APIENTRY
CheckForHiberboot(
    _Inout_ PBOOLEAN pHiberboot,
    _In_ BOOLEAN bClearFlag
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINADVAPI
LSTATUS
APIENTRY
RegSaveKeyExA(
    _In_ HKEY hKey,
    _In_ LPCSTR lpFile,
    _In_opt_ CONST LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    _In_ DWORD Flags
    );

WINADVAPI
LSTATUS
APIENTRY
RegSaveKeyExW(
    _In_ HKEY hKey,
    _In_ LPCWSTR lpFile,
    _In_opt_ CONST LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    _In_ DWORD Flags
    );

#ifdef UNICODE
#define RegSaveKeyEx  RegSaveKeyExW
#else
#define RegSaveKeyEx  RegSaveKeyExA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#ifdef __cplusplus
}
#endif

#endif // _WINREG_

