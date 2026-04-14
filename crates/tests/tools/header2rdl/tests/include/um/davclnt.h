/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    DavClnt.h

Abstract:

    This module defines the DAV specific functions that are exposed to the user

Revision History:

--*/

#ifndef _DAV_CLNT_H_
#define _DAV_CLNT_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus
extern "C" {
#endif

#define OPAQUE_HANDLE DWORD

// Certificates or other Auth types will be sent through this structure
typedef struct _DAV_CALLBACK_AUTH_BLOB{
    PVOID  pBuffer;
    ULONG  ulSize;              // Size of AuthBlob,
    ULONG  ulType;              // Type of Cred sent in ppBuffer. Currently the only value it supports is 1 for PCCERT_CONTEXT.
}DAV_CALLBACK_AUTH_BLOB, *PDAV_CALLBACK_AUTH_BLOB;

// Username and password will be sent through this structure
typedef struct _DAV_CALLBACK_AUTH_UNP{
   LPWSTR pszUserName;          // UserName . Memory to be allocated by the callback
   ULONG  ulUserNameLength;     // Length in WCHAR(Doesnt include terminating NULL)
   LPWSTR pszPassword;          // Password . Memory to be allocated by the callback
   ULONG  ulPasswordLength;     // Length in WCHAR(Doesnt include terminating NULL)
}DAV_CALLBACK_AUTH_UNP, *PDAV_CALLBACK_AUTH_UNP;

typedef struct _DAV_CALLBACK_CRED{
    DAV_CALLBACK_AUTH_BLOB AuthBlob;
    DAV_CALLBACK_AUTH_UNP UNPBlob;
    BOOL  bAuthBlobValid;       // Indicates if AuthBlob contents is valid. If false UNPBlob is valid
    BOOL  bSave;                // Indicates the value of bsave flag. True indicates that the creds were written to credman false indicates that creds were not written to credman
}DAV_CALLBACK_CRED, *PDAV_CALLBACK_CRED;

#define DAV_AUTHN_SCHEME_BASIC      0x00000001
#define DAV_AUTHN_SCHEME_NTLM       0x00000002
#define DAV_AUTHN_SCHEME_PASSPORT   0x00000004
#define DAV_AUTHN_SCHEME_DIGEST     0x00000008
#define DAV_AUTHN_SCHEME_NEGOTIATE  0x00000010
#define DAV_AUTHN_SCHEME_CERT       0x00010000
#define DAV_AUTHN_SCHEME_FBA        0x00100000

typedef enum
{
  DefaultBehavior,  // Try the default behaviour. Dont use callback
  RetryRequest,     // Retry the connection with the newly gathered creds
  CancelRequest     // Cancel connection
} AUTHNEXTSTEP;


//  Function signature for the callback to delete the memory allocated by the creds
typedef DWORD (CALLBACK *PFNDAVAUTHCALLBACK_FREECRED)(_In_ PVOID pbuffer);

//  Function signature for the Callback that will harvest the credentials
typedef DWORD (CALLBACK *PFNDAVAUTHCALLBACK)(
    _In_ LPWSTR lpwzServerName,     // Server Name
    _In_ LPWSTR lpwzRemoteName,     // Remote Name
    _In_ DWORD  dwAuthScheme,       // Bitmap of DAV_AUTHN_SCHEME* values
    _In_ DWORD  dwFlags,            // The flags that was passed to NPADDConnection
    _Inout_ PDAV_CALLBACK_CRED pCallbackCred,       // Creds collected by the callback
    _Inout_ AUTHNEXTSTEP *NextStep,                 // Next step for DavClnt
    _Out_ PFNDAVAUTHCALLBACK_FREECRED *pFreeCred    // Callback to free the memory allocated for creds by the callback
);

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
DavAddConnection(
    _Inout_ HANDLE *ConnectionHandle,
    _In_ LPCWSTR RemoteName,
    _In_opt_ LPCWSTR UserName,
    _In_opt_ LPCWSTR Password,
    _In_reads_bytes_(CertSize) PBYTE ClientCert,
    _In_ DWORD CertSize
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
DavDeleteConnection(
    _In_ HANDLE ConnectionHandle
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
DavGetUNCFromHTTPPath (
    _In_ LPCWSTR Url,
    _Out_writes_to_opt_(*lpSize, *lpSize) LPWSTR UncPath,
    _Inout_ LPDWORD lpSize
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
DavGetHTTPFromUNCPath (
    _In_ LPCWSTR  UncPath,
    _Out_writes_to_opt_(*lpSize, *lpSize) LPWSTR Url,
    _Inout_ LPDWORD lpSize
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
DavGetTheLockOwnerOfTheFile(
    _In_ LPCWSTR FileName,
    _Out_writes_bytes_opt_(*LockOwnerNameLengthInBytes) PWSTR LockOwnerName,
    _Inout_ PULONG LockOwnerNameLengthInBytes
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
DavGetExtendedError(
    _In_ HANDLE hFile,
    _Out_ DWORD *ExtError,
    _Out_writes_(*cChSize) LPWSTR ExtErrorString,
    _Inout_ DWORD *cChSize
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
DavFlushFile(
    _In_ HANDLE hFile
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
DavInvalidateCache(
    _In_ LPCWSTR URLName
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
DavCancelConnectionsToServer(
    _In_ LPWSTR lpName,
    _In_ BOOL fForce
    );

OPAQUE_HANDLE
WINAPI
DavRegisterAuthCallback(_In_ PFNDAVAUTHCALLBACK CallBack,
                        _In_ ULONG Version);

VOID
WINAPI
DavUnregisterAuthCallback(_In_ OPAQUE_HANDLE hCallback);

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
