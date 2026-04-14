//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 2009.
//
//  File:       dpapi.h
//
//  Contents:   Data Protection API
//
//----------------------------------------------------------------------------

#ifndef __DPAPI_H__
#define __DPAPI_H__

#if (_MSC_VER > 1020)
#pragma once
#endif

#if (NTDDI_VERSION >= NTDDI_WIN8)

#if !defined(DPAPI_IMP)
#define DPAPI_IMP
#endif

#else

#if !defined(_CRYPT32_)
#define DPAPI_IMP DECLSPEC_IMPORT
#else
#define DPAPI_IMP
#endif

#endif //(NTDDI_VERSION >= NTDDI_WIN7)

#ifdef __cplusplus
extern "C" {
#endif

#pragma region App Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifndef CRYPTO_BLOBS_DEFINED
#define CRYPTO_BLOBS_DEFINED
typedef struct _CRYPTOAPI_BLOB {
                            DWORD   cbData;
    _Field_size_bytes_(cbData)  BYTE    *pbData;
} CRYPT_INTEGER_BLOB, *PCRYPT_INTEGER_BLOB,
CRYPT_UINT_BLOB, *PCRYPT_UINT_BLOB,
CRYPT_OBJID_BLOB, *PCRYPT_OBJID_BLOB,
CERT_NAME_BLOB, *PCERT_NAME_BLOB,
CERT_RDN_VALUE_BLOB, *PCERT_RDN_VALUE_BLOB,
CERT_BLOB, *PCERT_BLOB,
CRL_BLOB, *PCRL_BLOB,
DATA_BLOB, *PDATA_BLOB,
CRYPT_DATA_BLOB, *PCRYPT_DATA_BLOB,
CRYPT_HASH_BLOB, *PCRYPT_HASH_BLOB,
CRYPT_DIGEST_BLOB, *PCRYPT_DIGEST_BLOB,
CRYPT_DER_BLOB, *PCRYPT_DER_BLOB,
CRYPT_ATTR_BLOB, *PCRYPT_ATTR_BLOB;
#endif

//
// Registry value for controlling Data Protection API (DPAPI) UI settings.
//

#define szFORCE_KEY_PROTECTION             "ForceKeyProtection"

#define dwFORCE_KEY_PROTECTION_DISABLED     0x0
#define dwFORCE_KEY_PROTECTION_USER_SELECT  0x1
#define dwFORCE_KEY_PROTECTION_HIGH         0x2

//
// Data protection APIs enable applications to easily secure data.
//
// The base provider provides protection based on the users' logon
// credentials. The data secured with these APIs follow the same
// roaming characteristics as HKCU -- if HKCU roams, the data
// protected by the base provider may roam as well. This makes
// the API ideal for the munging of data stored in the registry.
//

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region App Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// Prompt struct -- what to tell users about the access
//
typedef struct  _CRYPTPROTECT_PROMPTSTRUCT
{
    DWORD cbSize;
    DWORD dwPromptFlags;
    HWND  hwndApp;
    LPCWSTR szPrompt;
} CRYPTPROTECT_PROMPTSTRUCT, *PCRYPTPROTECT_PROMPTSTRUCT;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region App Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// base provider action
//
#define CRYPTPROTECT_DEFAULT_PROVIDER   { 0xdf9d8cd0, 0x1501, 0x11d1, {0x8c, 0x7a, 0x00, 0xc0, 0x4f, 0xc2, 0x97, 0xeb} }

//
// CryptProtect PromptStruct dwPromtFlags
//
//
// prompt on unprotect
#define CRYPTPROTECT_PROMPT_ON_UNPROTECT     0x1  // 1<<0
//
// prompt on protect
#define CRYPTPROTECT_PROMPT_ON_PROTECT       0x2  // 1<<1
#define CRYPTPROTECT_PROMPT_RESERVED         0x04 // reserved, do not use.

//
// default to strong variant UI protection (user supplied password currently).
#define CRYPTPROTECT_PROMPT_STRONG           0x08 // 1<<3

//
// require strong variant UI protection (user supplied password currently).
#define CRYPTPROTECT_PROMPT_REQUIRE_STRONG   0x10 // 1<<4

//
// CryptProtectData and CryptUnprotectData dwFlags
//
// for remote-access situations where ui is not an option
// if UI was specified on protect or unprotect operation, the call
// will fail and GetLastError() will indicate ERROR_PASSWORD_RESTRICTION
#define CRYPTPROTECT_UI_FORBIDDEN        0x1

//
// per machine protected data -- any user on machine where CryptProtectData
// took place may CryptUnprotectData
#define CRYPTPROTECT_LOCAL_MACHINE       0x4

//
// force credential synchronize during CryptProtectData()
// Synchronize is only operation that occurs during this operation
#define CRYPTPROTECT_CRED_SYNC           0x8

//
// Generate an Audit on protect and unprotect operations
//
#define CRYPTPROTECT_AUDIT              0x10

//
// Protect data with a non-recoverable key
//
#define CRYPTPROTECT_NO_RECOVERY        0x20


//
// Verify the protection of a protected blob
//
#define CRYPTPROTECT_VERIFY_PROTECTION  0x40

//
// Regenerate the local machine protection
//
#define CRYPTPROTECT_CRED_REGENERATE    0x80

// flags reserved for system use
#define CRYPTPROTECT_FIRST_RESERVED_FLAGVAL    0x0FFFFFFF
#define CRYPTPROTECT_LAST_RESERVED_FLAGVAL     0xFFFFFFFF

//
// flags specific to base provider
//
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region App Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

DPAPI_IMP
BOOL
WINAPI
CryptProtectData(
    _In_            DATA_BLOB*      pDataIn,
    _In_opt_        LPCWSTR         szDataDescr,
    _In_opt_        DATA_BLOB*      pOptionalEntropy,
    _Reserved_      PVOID           pvReserved,
    _In_opt_        CRYPTPROTECT_PROMPTSTRUCT*  pPromptStruct,
    _In_            DWORD           dwFlags,
    _Out_           DATA_BLOB*      pDataOut            // out encr blob
    );

DPAPI_IMP
BOOL
WINAPI
CryptUnprotectData(
    _In_            DATA_BLOB*      pDataIn,             // in encr blob
    _Outptr_opt_result_maybenull_ LPWSTR*     ppszDataDescr,       // out
    _In_opt_        DATA_BLOB*      pOptionalEntropy,
    _Reserved_      PVOID           pvReserved,
    _In_opt_        CRYPTPROTECT_PROMPTSTRUCT*  pPromptStruct,
    _In_            DWORD           dwFlags,
    _Out_           DATA_BLOB*      pDataOut
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_WIN8)
BOOL
WINAPI
CryptProtectDataNoUI(
    _In_            DATA_BLOB*      pDataIn,
    _In_opt_        LPCWSTR         szDataDescr,
    _In_opt_        DATA_BLOB*      pOptionalEntropy,
    _Reserved_      PVOID           pvReserved,
    _In_opt_        CRYPTPROTECT_PROMPTSTRUCT*  pPromptStruct,
    _In_            DWORD           dwFlags,
    _In_reads_bytes_opt_(cbOptionalPassword)
                    const BYTE      *pbOptionalPassword,
                    DWORD           cbOptionalPassword,
    _Out_           DATA_BLOB*      pDataOut            // out encr blob
    );

BOOL
WINAPI
CryptUnprotectDataNoUI(
    _In_            DATA_BLOB*      pDataIn,             // in encr blob
    _Outptr_opt_result_maybenull_ LPWSTR*     ppszDataDescr,       // out
    _In_opt_        DATA_BLOB*      pOptionalEntropy,
    _Reserved_      PVOID           pvReserved,
    _In_opt_        CRYPTPROTECT_PROMPTSTRUCT*  pPromptStruct,
    _In_            DWORD           dwFlags,
    _In_reads_bytes_opt_(cbOptionalPassword)
                    const BYTE      *pbOptionalPassword,
                    DWORD           cbOptionalPassword,
    _Out_           DATA_BLOB*      pDataOut
    );
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (NTDDI_VERSION >= NTDDI_VISTA)

DPAPI_IMP
BOOL
WINAPI
CryptUpdateProtectedState(
    _In_opt_        PSID            pOldSid,
    _In_opt_        LPCWSTR         pwszOldPassword,
    _In_            DWORD           dwFlags,
    _Out_opt_       DWORD           *pdwSuccessCount,
    _Out_opt_       DWORD           *pdwFailureCount);

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region App Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// The buffer length passed into CryptProtectMemory and CryptUnprotectMemory
// must be a multiple of this length (or zero).
//

#define CRYPTPROTECTMEMORY_BLOCK_SIZE           16


//
// CryptProtectMemory/CryptUnprotectMemory dwFlags
//

//
// Encrypt/Decrypt within current process context.
//

#define CRYPTPROTECTMEMORY_SAME_PROCESS         0x00

//
// Encrypt/Decrypt across process boundaries.
// eg: encrypted buffer passed across LPC to another process which calls CryptUnprotectMemory.
//

#define CRYPTPROTECTMEMORY_CROSS_PROCESS        0x01

//
// Encrypt/Decrypt across callers with same LogonId.
// eg: encrypted buffer passed across LPC to another process which calls CryptUnprotectMemory whilst impersonating.
//

#define CRYPTPROTECTMEMORY_SAME_LOGON           0x02

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region App Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

DPAPI_IMP
BOOL
WINAPI
CryptProtectMemory(
    _Inout_         LPVOID          pDataIn,             // in out data to encrypt
    _In_            DWORD           cbDataIn,            // multiple of CRYPTPROTECTMEMORY_BLOCK_SIZE
    _In_            DWORD           dwFlags
    );

DPAPI_IMP
BOOL
WINAPI
CryptUnprotectMemory(
    _Inout_         LPVOID          pDataIn,             // in out data to decrypt
    _In_            DWORD           cbDataIn,            // multiple of CRYPTPROTECTMEMORY_BLOCK_SIZE
    _In_            DWORD           dwFlags
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#ifdef __cplusplus
} // extern "C"
#endif

#endif // __DPAPI_H__
