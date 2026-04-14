//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 1992-1999.
//
//  File:       minschannel.h
//
//  Contents:   Public Definitions for MIN SCHANNEL Security Provider
//
//  Classes:
//
//  Functions:
//
//----------------------------------------------------------------------------



#ifndef __MINSCHANNEL_H__
#define __MINSCHANNEL_H__

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)



//
// Constants
//

//
// QueryContextAttributes/QueryCredentialsAttribute extensions
//

#define SECPKG_ATTR_ISSUER_LIST          0x50   // (OBSOLETE) returns SecPkgContext_IssuerListInfo
#define SECPKG_ATTR_REMOTE_CRED          0x51   // (OBSOLETE) returns SecPkgContext_RemoteCredentialInfo
#define SECPKG_ATTR_LOCAL_CRED           0x52   // (OBSOLETE) returns SecPkgContext_LocalCredentialInfo
#define SECPKG_ATTR_REMOTE_CERT_CONTEXT  0x53   // returns PCCERT_CONTEXT
#define SECPKG_ATTR_LOCAL_CERT_CONTEXT   0x54   // returns PCCERT_CONTEXT
#define SECPKG_ATTR_ROOT_STORE           0x55   // returns HCERTCONTEXT to the root store
#define SECPKG_ATTR_SUPPORTED_ALGS       0x56   // returns SecPkgCred_SupportedAlgs
#define SECPKG_ATTR_CIPHER_STRENGTHS     0x57   // returns SecPkgCred_CipherStrengths
#define SECPKG_ATTR_SUPPORTED_PROTOCOLS  0x58   // returns SecPkgCred_SupportedProtocols
#define SECPKG_ATTR_ISSUER_LIST_EX       0x59   // returns SecPkgContext_IssuerListInfoEx
#define SECPKG_ATTR_CONNECTION_INFO      0x5a   // returns SecPkgContext_ConnectionInfo
#define SECPKG_ATTR_EAP_KEY_BLOCK        0x5b   // returns SecPkgContext_EapKeyBlock
#define SECPKG_ATTR_MAPPED_CRED_ATTR     0x5c   // returns SecPkgContext_MappedCredAttr
#define SECPKG_ATTR_SESSION_INFO         0x5d   // returns SecPkgContext_SessionInfo
#define SECPKG_ATTR_APP_DATA             0x5e   // sets/returns SecPkgContext_SessionAppData
#define SECPKG_ATTR_REMOTE_CERTIFICATES  0x5F   // returns SecPkgContext_Certificates
#define SECPKG_ATTR_CLIENT_CERT_POLICY   0x60   // sets    SecPkgCred_ClientCertCtlPolicy
#define SECPKG_ATTR_CC_POLICY_RESULT     0x61   // returns SecPkgContext_ClientCertPolicyResult
#define SECPKG_ATTR_USE_NCRYPT           0x62   // Sets the CRED_FLAG_USE_NCRYPT_PROVIDER FLAG on cred group
#define SECPKG_ATTR_LOCAL_CERT_INFO      0x63   // returns SecPkgContext_CertInfo
#define SECPKG_ATTR_CIPHER_INFO          0x64   // returns new CNG SecPkgContext_CipherInfo
#define SECPKG_ATTR_EAP_PRF_INFO         0x65   // sets    SecPkgContext_EapPrfInfo
#define SECPKG_ATTR_SUPPORTED_SIGNATURES 0x66   // returns SecPkgContext_SupportedSignatures
#define SECPKG_ATTR_REMOTE_CERT_CHAIN    0x67   // returns PCCERT_CONTEXT
#define SECPKG_ATTR_UI_INFO              0x68   // sets SEcPkgContext_UiInfo
#define SECPKG_ATTR_EARLY_START          0x69   // sets SecPkgContext_EarlyStart
#define SECPKG_ATTR_KEYING_MATERIAL_INFO 0x6a   // sets SecPkgContext_KeyingMaterialInfo
#define SECPKG_ATTR_KEYING_MATERIAL      0x6b   // returns SecPkgContext_KeyingMaterial
#define SECPKG_ATTR_SRTP_PARAMETERS      0x6c   // returns negotiated SRTP parameters
#define SECPKG_ATTR_TOKEN_BINDING        0x6d   // returns SecPkgContext_TokenBinding
#define SECPKG_ATTR_CONNECTION_INFO_EX   0x6e   // returns SecPkgContext_ConnectionInfoEx
#define SECPKG_ATTR_KEYING_MATERIAL_TOKEN_BINDING 0x6f // returns SecPkgContext_KeyingMaterial specific to Token Binding
#define SECPKG_ATTR_KEYING_MATERIAL_INPROC        0x70 // returns SecPkgContext_KeyingMaterial_Inproc
#define SECPKG_ATTR_CERT_CHECK_RESULT        0x71 // returns SecPkgContext_CertificateValidationResult, use during and after SSPI handshake loop
#define SECPKG_ATTR_CERT_CHECK_RESULT_INPROC 0x72 // returns SecPkgContext_CertificateValidationResult, use only after SSPI handshake loop
#define SECPKG_ATTR_SESSION_TICKET_KEYS      0x73 // sets    SecPkgCred_SessionTicketKeys
#define SECPKG_ATTR_SERIALIZED_REMOTE_CERT_CONTEXT_INPROC 0x74 // returns CERT_BLOB, use only after SSPI handshake loop
#define SECPKG_ATTR_SERIALIZED_REMOTE_CERT_CONTEXT 0x75 // returns CERT_BLOB, use during and after SSPI handshake loop

//
// typedefs
//

typedef unsigned int ALG_ID;

typedef struct _SecPkgCred_SupportedAlgs
{
    DWORD		cSupportedAlgs;
    ALG_ID		*palgSupportedAlgs;
} SecPkgCred_SupportedAlgs, *PSecPkgCred_SupportedAlgs;


typedef struct _SecPkgCred_CipherStrengths
{
    DWORD       dwMinimumCipherStrength;
    DWORD       dwMaximumCipherStrength;
} SecPkgCred_CipherStrengths, *PSecPkgCred_CipherStrengths;


typedef struct _SecPkgCred_SupportedProtocols
{
    DWORD      	grbitProtocol;
} SecPkgCred_SupportedProtocols, *PSecPkgCred_SupportedProtocols;

//An IDL struct _SecPkgCred_ClientCertPolicy_RPC is defined in minio/security/base/lsa/idl/sspi/sspirpc.idl for rpc calls.
//The IDL struct should also be updated if there is any change on struct _SecPkgCred_ClientCertPolicy.

typedef struct _SecPkgCred_ClientCertPolicy
{
    DWORD   dwFlags;
    GUID    guidPolicyId;
    DWORD   dwCertFlags;
    DWORD   dwUrlRetrievalTimeout;
    BOOL    fCheckRevocationFreshnessTime;
    DWORD   dwRevocationFreshnessTime;
    BOOL    fOmitUsageCheck;
    LPWSTR  pwszSslCtlStoreName;
    LPWSTR  pwszSslCtlIdentifier;
} SecPkgCred_ClientCertPolicy, *PSecPkgCred_ClientCertPolicy;

// Session ticket protection version definitions.
#define SESSION_TICKET_INFO_V0 0
#define SESSION_TICKET_INFO_VERSION SESSION_TICKET_INFO_V0

typedef struct _SecPkgCred_SessionTicketKey
{
    DWORD   TicketInfoVersion;      // Set to SESSION_TICKET_INFO_VERSION for the current session ticket protection method.
    BYTE    KeyId[16];              // Uniquely identifies each session ticket key issued by a TLS server.
    BYTE    KeyingMaterial[64];     // Must be generated using a cryptographic RNG.
    BYTE    KeyingMaterialSize;     // Size in bytes of the keying material in the KeyingMaterial array. Must be between 32 and 64.
} SecPkgCred_SessionTicketKey, *PSecPkgCred_SessionTicketKey;

typedef struct _SecPkgCred_SessionTicketKeys
{
    DWORD                           cSessionTicketKeys; // Up to 16 keys.
    PSecPkgCred_SessionTicketKey    pSessionTicketKeys;
} SecPkgCred_SessionTicketKeys, *PSecPkgCred_SessionTicketKeys;


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // __MINSCHANNEL_H__
