#include <winapifamily.h>
//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 1992-1999.
//
//  File:       winefs.h
//
//  Contents:   EFS Data and prototypes.
//
//----------------------------------------------------------------------------

#ifndef __WINEFS_H__
#define __WINEFS_H__

// winefs.h was not available at all pre-W2K
#if (NTDDI_VERSION >= NTDDI_WIN2K)


#if _MSC_VER > 1000
#pragma once
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#define WINEFS_SETUSERKEY_SET_CAPABILITIES      0x00000001

//+---------------------------------------------------------------------------------/
//                                                                                  /
//                                                                                  /
//                          Data Structures                                         /
//                                                                                  /
//                                                                                  /
//----------------------------------------------------------------------------------/

// ALG_ID define is used as part of EFS_KEY_INFO (not available pre-WXP)
#if (NTDDI_VERSION >= NTDDI_WINXP)

#ifndef ALGIDDEF
#define ALGIDDEF
typedef unsigned int ALG_ID;
#endif

#endif

//
//  Encoded Certificate
//


typedef struct _CERTIFICATE_BLOB {

    DWORD   dwCertEncodingType;

#ifdef MIDL_PASS
    [range(0,32768)] 
#endif // MIDL_PASS

    DWORD   cbData;

#ifdef MIDL_PASS
    [size_is(cbData)]
#endif // MIDL_PASS
    PBYTE    pbData;

} EFS_CERTIFICATE_BLOB, *PEFS_CERTIFICATE_BLOB;

//
//  Certificate Hash
//

typedef struct _EFS_HASH_BLOB {

#ifdef MIDL_PASS
    [range(0,100)] 
#endif // MIDL_PASS
    DWORD   cbData;

#ifdef MIDL_PASS
    [size_is(cbData)]
#endif // MIDL_PASS
    PBYTE    pbData;

} EFS_HASH_BLOB, *PEFS_HASH_BLOB;


// EFS_RPC_BLOB was not available pre-WXP
#if (NTDDI_VERSION >= NTDDI_WINXP)

//
//  RPC blob
//

typedef struct _EFS_RPC_BLOB {

#ifdef MIDL_PASS
    [range(0,266240)] 
#endif // MIDL_PASS
    DWORD   cbData;

#ifdef MIDL_PASS
    [size_is(cbData)]
#endif // MIDL_PASS
    PBYTE    pbData;

} EFS_RPC_BLOB, *PEFS_RPC_BLOB;

#endif // #if (NTDDI_VERSION >= NTDDI_WINXP)

// EFS_PIN_BLOB was not available pre-LH
#if (NTDDI_VERSION >= NTDDI_VISTA) 

typedef struct _EFS_PIN_BLOB {
    
#ifdef MIDL_PASS
    [range(0,8)] 
#endif // MIDL_PASS  
    DWORD   cbPadding; 

#ifdef MIDL_PASS
    [range(0,2048)] 
#endif // MIDL_PASS
    DWORD   cbData;

#ifdef MIDL_PASS
    [size_is(cbData+cbPadding)]
#endif // MIDL_PASS
    PBYTE    pbData;

} EFS_PIN_BLOB, *PEFS_PIN_BLOB;

#endif // #if (NTDDI_VERSION >= NTDDI_VISTA) 


// EFS_KEY_INFO was not available pre-WXP
#if (NTDDI_VERSION >= NTDDI_WINXP)

typedef struct _EFS_KEY_INFO {

    DWORD   dwVersion;
    ULONG   Entropy;
    ALG_ID  Algorithm;
    ULONG   KeyLength;
    
} EFS_KEY_INFO, *PEFS_KEY_INFO;

#endif // #if (NTDDI_VERSION >= NTDDI_WINXP)

// EFS_COMPATIBILITY_INFO was not available pre-Windows 7
#if (NTDDI_VERSION >= NTDDI_WIN7)

typedef struct _EFS_COMPATIBILITY_INFO {

    DWORD EfsVersion;
    
} EFS_COMPATIBILITY_INFO, *PEFS_COMPATIBILITY_INFO;

#endif // #if (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

#define EFS_COMPATIBILITY_VERSION_NCRYPT_PROTECTOR 5

#endif // #if (NTDDI_VERSION >= NTDDI_WINBLUE)

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#define EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR 6

#define EFS_IS_DESCRIPTOR_VERSION(__v)                       \
    ( (__v == EFS_COMPATIBILITY_VERSION_NCRYPT_PROTECTOR) || \
      (__v == EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR) )

#define EFS_SUBVER_UNKNOWN          0

#define EFS_EFS_SUBVER_EFS_CERT     1

#define EFS_PFILE_SUBVER_RMS        2
#define EFS_PFILE_SUBVER_APPX       3

typedef struct _EFS_VERSION_INFO {
    DWORD EfsVersion;
    DWORD SubVersion;
} EFS_VERSION_INFO, *PEFS_VERSION_INFO;

#define EFS_IS_APPX_VERSION(__v, __subV)                    \
    ( (__v == EFS_COMPATIBILITY_VERSION_PFILE_PROTECTOR) && \
      (__subV == EFS_PFILE_SUBVER_APPX) )

#endif // #if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)


// EFS_(EN/DE)CRYPTION_STATUS_INFO was not available pre-LH
#if (NTDDI_VERSION >= NTDDI_VISTA) 

typedef struct _EFS_DECRYPTION_STATUS_INFO { 

    DWORD   dwDecryptionError;
    DWORD   dwHashOffset;
    DWORD   cbHash;

} EFS_DECRYPTION_STATUS_INFO, *PEFS_DECRYPTION_STATUS_INFO;

typedef struct _EFS_ENCRYPTION_STATUS_INFO { 

    BOOL    bHasCurrentKey;
    DWORD   dwEncryptionError;

} EFS_ENCRYPTION_STATUS_INFO, *PEFS_ENCRYPTION_STATUS_INFO;

#endif // #if (NTDDI_VERSION >= NTDDI_VISTA) 

//
// Input to add a user to an encrypted file
//

typedef struct _ENCRYPTION_CERTIFICATE {
    DWORD cbTotalLength;
    SID * pUserSid;
    PEFS_CERTIFICATE_BLOB pCertBlob;
} ENCRYPTION_CERTIFICATE, *PENCRYPTION_CERTIFICATE;

#define MAX_SID_SIZE 256


typedef struct _ENCRYPTION_CERTIFICATE_HASH {
    DWORD cbTotalLength;
    SID * pUserSid;
    PEFS_HASH_BLOB  pHash;

#ifdef MIDL_PASS
    [string]
#endif // MIDL_PASS
    LPWSTR lpDisplayInformation;

} ENCRYPTION_CERTIFICATE_HASH, *PENCRYPTION_CERTIFICATE_HASH;

typedef struct _ENCRYPTION_CERTIFICATE_HASH_LIST {
#ifdef MIDL_PASS
    [range(0,500)] 
#endif // MIDL_PASS
    DWORD nCert_Hash;
#ifdef MIDL_PASS
    [size_is(nCert_Hash)]
#endif // MIDL_PASS
     PENCRYPTION_CERTIFICATE_HASH * pUsers;
} ENCRYPTION_CERTIFICATE_HASH_LIST, *PENCRYPTION_CERTIFICATE_HASH_LIST;



typedef struct _ENCRYPTION_CERTIFICATE_LIST {    
#ifdef MIDL_PASS
    [range(0,500)] 
#endif // MIDL_PASS
    DWORD nUsers;
#ifdef MIDL_PASS
    [size_is(nUsers)]
#endif // MIDL_PASS
     PENCRYPTION_CERTIFICATE * pUsers;
} ENCRYPTION_CERTIFICATE_LIST, *PENCRYPTION_CERTIFICATE_LIST;

// ENCRYPTED_FILE_METADATA_SIGNATURE was not available pre-LH
#if (NTDDI_VERSION >= NTDDI_VISTA) 

#define		EFS_METADATA_ADD_USER		0x00000001
#define		EFS_METADATA_REMOVE_USER	0x00000002
#define		EFS_METADATA_REPLACE_USER	0x00000004
#define		EFS_METADATA_GENERAL_OP		0x00000008

typedef struct _ENCRYPTED_FILE_METADATA_SIGNATURE { 

	DWORD								dwEfsAccessType;	
	PENCRYPTION_CERTIFICATE_HASH_LIST	pCertificatesAdded;
    PENCRYPTION_CERTIFICATE				pEncryptionCertificate;
    PEFS_RPC_BLOB						pEfsStreamSignature;

} ENCRYPTED_FILE_METADATA_SIGNATURE, *PENCRYPTED_FILE_METADATA_SIGNATURE;

#endif // #if (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN8)

typedef struct _ENCRYPTION_PROTECTOR{
    DWORD cbTotalLength;
    SID * pUserSid;
#ifdef MIDL_PASS
    [string]
#endif // MIDL_PASS
    LPWSTR lpProtectorDescriptor;
} ENCRYPTION_PROTECTOR, *PENCRYPTION_PROTECTOR;

typedef struct _ENCRYPTION_PROTECTOR_LIST {
    DWORD nProtectors;
#ifdef MIDL_PASS
    [size_is(nProtectors)]
#endif // MIDL_PASS
    PENCRYPTION_PROTECTOR *pProtectors;
} ENCRYPTION_PROTECTOR_LIST, *PENCRYPTION_PROTECTOR_LIST;

#endif

//+---------------------------------------------------------------------------------/
//                                                                                  /
//                                                                                  /
//                               Prototypes                                         /
//                                                                                  /
//                                                                                  /
//----------------------------------------------------------------------------------/


WINADVAPI
DWORD
WINAPI
QueryUsersOnEncryptedFile(
    _In_            LPCWSTR                                 lpFileName,
    _Outptr_     PENCRYPTION_CERTIFICATE_HASH_LIST      *pUsers
    );


WINADVAPI
DWORD
WINAPI
QueryRecoveryAgentsOnEncryptedFile(
    _In_            LPCWSTR                                 lpFileName,
    _Outptr_     PENCRYPTION_CERTIFICATE_HASH_LIST      *pRecoveryAgents
    );


WINADVAPI
DWORD
WINAPI
RemoveUsersFromEncryptedFile(
    _In_    LPCWSTR                             lpFileName,
    _In_    PENCRYPTION_CERTIFICATE_HASH_LIST   pHashes
    );

WINADVAPI
DWORD
WINAPI
AddUsersToEncryptedFile(
    _In_    LPCWSTR                         lpFileName,
    _In_    PENCRYPTION_CERTIFICATE_LIST    pEncryptionCertificates
    );

//
// SetUserFileEncryptionKey signature change in Vista to allow NULL certificate
//

#if (NTDDI_VERSION >= NTDDI_VISTA)

WINADVAPI
DWORD
WINAPI
SetUserFileEncryptionKey(
    _In_opt_    PENCRYPTION_CERTIFICATE     pEncryptionCertificate
    );

#else

WINADVAPI
DWORD
WINAPI
SetUserFileEncryptionKey(
    _In_    PENCRYPTION_CERTIFICATE     pEncryptionCertificate
    );

#endif

// SetUserFileEncryptionKeyEx was not available pre-LH
#if (NTDDI_VERSION >= NTDDI_VISTA) 

WINADVAPI
DWORD
WINAPI
SetUserFileEncryptionKeyEx(
    _In_opt_        PENCRYPTION_CERTIFICATE     pEncryptionCertificate,
                    DWORD                       dwCapabilities, 
                    DWORD                       dwFlags,
    _Reserved_      LPVOID                      pvReserved
    );

#endif // #if (NTDDI_VERSION >= NTDDI_VISTA) 

WINADVAPI
VOID
WINAPI
FreeEncryptionCertificateHashList(
    _In_    PENCRYPTION_CERTIFICATE_HASH_LIST       pUsers
    );

WINADVAPI
BOOL
WINAPI
EncryptionDisable(
    _In_ LPCWSTR DirPath,
         BOOL        Disable
    );



// DuplicateEncryptionInfoFile signature changed
// in WS03 to use a CONST SECURITY_ATTRIBUTES structure
#if (NTDDI_VERSION >= NTDDI_WS03)

WINADVAPI
DWORD
WINAPI
DuplicateEncryptionInfoFile(
     _In_       LPCWSTR SrcFileName,
     _In_       LPCWSTR DstFileName, 
                DWORD                           dwCreationDistribution, 
                DWORD                           dwAttributes, 
     _In_opt_   CONST LPSECURITY_ATTRIBUTES     lpSecurityAttributes
     );

// DuplicateEncryptionInfoFile didn't exist pre-WXP
#elif (NTDDI_VERSION >= NTDDI_WINXP)


WINADVAPI
DWORD
WINAPI
DuplicateEncryptionInfoFile(
     _In_       LPCWSTR SrcFileName,
     _In_       LPCWSTR DstFileName, 
                DWORD                   dwCreationDistribution, 
                DWORD                   dwAttributes, 
     _In_opt_   LPSECURITY_ATTRIBUTES   lpSecurityAttributes
     );


#endif 

// *EncryptedFileMetadata routines were not available pre-LH
#if (NTDDI_VERSION >= NTDDI_VISTA) 

__declspec(deprecated)
WINADVAPI
DWORD
WINAPI
GetEncryptedFileMetadata(
    _In_                                LPCWSTR     lpFileName,
    _Out_                               PDWORD      pcbMetadata, 
    _Outptr_result_bytebuffer_(*pcbMetadata)    PBYTE      *ppbMetadata
    );

__declspec(deprecated)
WINADVAPI
DWORD
WINAPI
SetEncryptedFileMetadata(
    _In_        LPCWSTR                             lpFileName,
    _In_opt_    PBYTE                               pbOldMetadata,
    _In_        PBYTE                               pbNewMetadata,
    _In_        PENCRYPTION_CERTIFICATE_HASH        pOwnerHash,
                DWORD                               dwOperation,
    _In_opt_    PENCRYPTION_CERTIFICATE_HASH_LIST   pCertificatesAdded
    ); 
    
__declspec(deprecated)
WINADVAPI
VOID
WINAPI
FreeEncryptedFileMetadata(
    _In_    PBYTE   pbMetadata
    ); 

#endif // #if (NTDDI_VERSION >= NTDDI_VISTA) 


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
}       // Balance extern "C" above
#endif

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif // #if (NTDDI_VERSION >= NTDDI_WIN2K)

#endif // __WINEFS_H__
