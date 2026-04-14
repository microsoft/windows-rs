//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 1992-1999.
//
//  File:       schannel.h
//
//  Contents:   Public Definitions for SCHANNEL Security Provider
//
//  Classes:
//
//  Functions:
//
//----------------------------------------------------------------------------



#ifndef __SCHANNEL_H__
#define __SCHANNEL_H__

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


#include <minschannel.h>
#include <wincrypt.h>


//
// Security package names.
//

#define UNISP_NAME_A    "Microsoft Unified Security Protocol Provider"
#define UNISP_NAME_W    L"Microsoft Unified Security Protocol Provider"

#define SSL2SP_NAME_A    "Microsoft SSL 2.0"
#define SSL2SP_NAME_W    L"Microsoft SSL 2.0"

#define SSL3SP_NAME_A    "Microsoft SSL 3.0"
#define SSL3SP_NAME_W    L"Microsoft SSL 3.0"

#define TLS1SP_NAME_A    "Microsoft TLS 1.0"
#define TLS1SP_NAME_W    L"Microsoft TLS 1.0"

#define PCT1SP_NAME_A    "Microsoft PCT 1.0"
#define PCT1SP_NAME_W    L"Microsoft PCT 1.0"

#define SCHANNEL_NAME_A  "Schannel"
#define SCHANNEL_NAME_W  L"Schannel"

#define DEFAULT_TLS_SSP_NAME_A  "Default TLS SSP"
#define DEFAULT_TLS_SSP_NAME_W  L"Default TLS SSP"

#ifdef UNICODE

#define UNISP_NAME  UNISP_NAME_W
#define PCT1SP_NAME  PCT1SP_NAME_W
#define SSL2SP_NAME  SSL2SP_NAME_W
#define SSL3SP_NAME  SSL3SP_NAME_W
#define TLS1SP_NAME  TLS1SP_NAME_W
#define SCHANNEL_NAME  SCHANNEL_NAME_W
#define DEFAULT_TLS_SSP_NAME  DEFAULT_TLS_SSP_NAME_W

#else

#define UNISP_NAME  UNISP_NAME_A
#define PCT1SP_NAME  PCT1SP_NAME_A
#define SSL2SP_NAME  SSL2SP_NAME_A
#define SSL3SP_NAME  SSL3SP_NAME_A
#define TLS1SP_NAME  TLS1SP_NAME_A
#define SCHANNEL_NAME  SCHANNEL_NAME_A
#define DEFAULT_TLS_SSP_NAME  DEFAULT_TLS_SSP_NAME_A

#endif


typedef enum _eTlsSignatureAlgorithm
{
    TlsSignatureAlgorithm_Anonymous         = 0,
    TlsSignatureAlgorithm_Rsa               = 1,
    TlsSignatureAlgorithm_Dsa               = 2,
    TlsSignatureAlgorithm_Ecdsa             = 3
} eTlsSignatureAlgorithm;

typedef enum _eTlsHashAlgorithm
{
    TlsHashAlgorithm_None                   = 0,
    TlsHashAlgorithm_Md5                    = 1,
    TlsHashAlgorithm_Sha1                   = 2,
    TlsHashAlgorithm_Sha224                 = 3,
    TlsHashAlgorithm_Sha256                 = 4,
    TlsHashAlgorithm_Sha384                 = 5,
    TlsHashAlgorithm_Sha512                 = 6
} eTlsHashAlgorithm;


//
// RPC constants.
//

#define UNISP_RPC_ID    14


// OBSOLETE - included here for backward compatibility only
typedef struct _SecPkgContext_RemoteCredentialInfo
{
    DWORD   cbCertificateChain;
    PBYTE   pbCertificateChain;
    DWORD   cCertificates;
    DWORD   fFlags;
    DWORD   dwBits;
} SecPkgContext_RemoteCredentialInfo, *PSecPkgContext_RemoteCredentialInfo;

typedef SecPkgContext_RemoteCredentialInfo SecPkgContext_RemoteCredenitalInfo, *PSecPkgContext_RemoteCredenitalInfo;

#define RCRED_STATUS_NOCRED          0x00000000
#define RCRED_CRED_EXISTS            0x00000001
#define RCRED_STATUS_UNKNOWN_ISSUER  0x00000002


// OBSOLETE - included here for backward compatibility only
typedef struct _SecPkgContext_LocalCredentialInfo
{
    DWORD   cbCertificateChain;
    PBYTE   pbCertificateChain;
    DWORD   cCertificates;
    DWORD   fFlags;
    DWORD   dwBits;
} SecPkgContext_LocalCredentialInfo, *PSecPkgContext_LocalCredentialInfo;

typedef SecPkgContext_LocalCredentialInfo SecPkgContext_LocalCredenitalInfo, *PSecPkgContext_LocalCredenitalInfo;

#define LCRED_STATUS_NOCRED          0x00000000
#define LCRED_CRED_EXISTS            0x00000001
#define LCRED_STATUS_UNKNOWN_ISSUER  0x00000002


typedef struct _SecPkgContext_ClientCertPolicyResult
{
    HRESULT dwPolicyResult;
    GUID    guidPolicyId;
} SecPkgContext_ClientCertPolicyResult, *PSecPkgContext_ClientCertPolicyResult;


typedef struct _SecPkgContext_IssuerListInfoEx
{
    PCERT_NAME_BLOB   	aIssuers;
    DWORD           	cIssuers;
} SecPkgContext_IssuerListInfoEx, *PSecPkgContext_IssuerListInfoEx;


typedef struct _SecPkgContext_ConnectionInfo
{
    DWORD   dwProtocol;
    ALG_ID  aiCipher;
    DWORD   dwCipherStrength;
    ALG_ID  aiHash;
    DWORD   dwHashStrength;
    ALG_ID  aiExch;
    DWORD   dwExchStrength;
} SecPkgContext_ConnectionInfo, *PSecPkgContext_ConnectionInfo;

#define SZ_ALG_MAX_SIZE 64

#define SECPKGCONTEXT_CONNECTION_INFO_EX_V1   1

typedef struct _SecPkgContext_ConnectionInfoEx
{
    DWORD   dwVersion;
    DWORD   dwProtocol;
    WCHAR   szCipher[SZ_ALG_MAX_SIZE];
    DWORD   dwCipherStrength;
    WCHAR   szHash[SZ_ALG_MAX_SIZE];
    DWORD   dwHashStrength;
    WCHAR   szExchange[SZ_ALG_MAX_SIZE];
    DWORD   dwExchStrength;
} SecPkgContext_ConnectionInfoEx, *PSecPkgContext_ConnectionInfoEx;

#define SECPKGCONTEXT_CIPHERINFO_V1 1

typedef struct _SecPkgContext_CipherInfo
{

    DWORD dwVersion;
    DWORD dwProtocol;
    DWORD dwCipherSuite;
    DWORD dwBaseCipherSuite;
    WCHAR szCipherSuite[SZ_ALG_MAX_SIZE];
    WCHAR szCipher[SZ_ALG_MAX_SIZE];
    DWORD dwCipherLen;
    DWORD dwCipherBlockLen;    // in bytes
    WCHAR szHash[SZ_ALG_MAX_SIZE];
    DWORD dwHashLen;
    WCHAR szExchange[SZ_ALG_MAX_SIZE];
    DWORD dwMinExchangeLen;
    DWORD dwMaxExchangeLen;
    WCHAR szCertificate[SZ_ALG_MAX_SIZE];
    DWORD dwKeyType;
} SecPkgContext_CipherInfo, *PSecPkgContext_CipherInfo;



typedef struct _SecPkgContext_EapKeyBlock
{
    BYTE    rgbKeys[128];
    BYTE    rgbIVs[64];
} SecPkgContext_EapKeyBlock, *PSecPkgContext_EapKeyBlock;


typedef struct _SecPkgContext_MappedCredAttr
{
    DWORD   dwAttribute;
    PVOID   pvBuffer;
} SecPkgContext_MappedCredAttr, *PSecPkgContext_MappedCredAttr;


// Flag values for SecPkgContext_SessionInfo
#define SSL_SESSION_RECONNECT   1

typedef struct _SecPkgContext_SessionInfo
{
    DWORD dwFlags;
    DWORD cbSessionId;
    BYTE  rgbSessionId[32];
} SecPkgContext_SessionInfo, *PSecPkgContext_SessionInfo;


typedef struct _SecPkgContext_SessionAppData
{
    DWORD dwFlags;
    DWORD cbAppData;
    _Field_size_bytes_(cbAppData) PBYTE pbAppData;
} SecPkgContext_SessionAppData, *PSecPkgContext_SessionAppData;

typedef struct _SecPkgContext_EapPrfInfo
{
    DWORD dwVersion;
    DWORD cbPrfData;
    _Field_size_bytes_(cbPrfData) PBYTE pbPrfData;
} SecPkgContext_EapPrfInfo, *PSecPkgContext_EapPrfInfo;


typedef struct _SecPkgContext_SupportedSignatures
{
    WORD cSignatureAndHashAlgorithms;

    //
    // Upper byte (from TLS 1.2, RFC 4346):
    //     enum {
    //         anonymous(0), rsa(1), dsa(2), ecdsa(3), (255)
    //     } SignatureAlgorithm;
    //
    // enum eTlsSignatureAlgorithm

    //
    // Lower byte (from TLS 1.2, RFC 4346):
    //     enum {
    //         none(0), md5(1), sha1(2), sha224(3), sha256(4), sha384(5),
    //         sha512(6), (255)
    //     } HashAlgorithm;
    //
    //
    // enum eTlsHashAlgorithm

    _Field_size_(cSignatureAndHashAlgorithms)
        WORD *pSignatureAndHashAlgorithms;
} SecPkgContext_SupportedSignatures, *PSecPkgContext_SupportedSignatures;


//
// This property returns the raw binary certificates that were received
// from the remote party. The format of the buffer that's returned is as
// follows.
//
//     <4 bytes> length of certificate #1
//     <n bytes> certificate #1
//     <4 bytes> length of certificate #2
//     <n bytes> certificate #2
//     ...
//
// After this data is processed, the caller of QueryContextAttributes
// must free the pbCertificateChain buffer using FreeContextBuffer.
//
typedef struct _SecPkgContext_Certificates
{
    DWORD   cCertificates;
    DWORD   cbCertificateChain;
    PBYTE   pbCertificateChain;
} SecPkgContext_Certificates, *PSecPkgContext_Certificates;


//
// This property returns information about a certificate. In particular
// it is useful (and only available) in the kernel where CAPI2 is not
// available.
//
typedef struct _SecPkgContext_CertInfo
{
    DWORD   dwVersion;
    DWORD   cbSubjectName;
    LPWSTR  pwszSubjectName;
    DWORD   cbIssuerName;
    LPWSTR  pwszIssuerName;
    DWORD   dwKeySize;
} SecPkgContext_CertInfo, *PSecPkgContext_CertInfo;

#define KERN_CONTEXT_CERT_INFO_V1 0x00000000

typedef struct _SecPkgContext_UiInfo
{
    HWND    hParentWindow;
} SecPkgContext_UiInfo, *PSecPkgContext_UiInfo;

typedef struct _SecPkgContext_EarlyStart
{
    DWORD dwEarlyStartFlags;
} SecPkgContext_EarlyStart, *PSecPkgContext_EarlyStart;

// Flag values for SecPkgContext_EarlyStart
#define ENABLE_TLS_CLIENT_EARLY_START           0x00000001

typedef struct _SecPkgContext_KeyingMaterialInfo
{
    WORD cbLabel;           // Disambiguating ASCII label length, in bytes, greater than 0.
    LPSTR pszLabel;         // Disambiguating ASCII label, NUL terminator will be removed by schannel.
    WORD cbContextValue;    // Application context value length, in bytes, can be 0.
    PBYTE pbContextValue;   // Application context value, NULL if cbContextValue == 0.
    DWORD cbKeyingMaterial; // Requested keying material length, in bytes, greater than 0.
} SecPkgContext_KeyingMaterialInfo, * PSecPkgContext_KeyingMaterialInfo;

typedef struct _SecPkgContext_KeyingMaterial
{
    DWORD cbKeyingMaterial; // Exported keying material length, in bytes.
    _Field_size_bytes_(cbKeyingMaterial) PBYTE pbKeyingMaterial; // Exported keying material.
} SecPkgContext_KeyingMaterial, * PSecPkgContext_KeyingMaterial;

typedef struct _SecPkgContext_KeyingMaterial_Inproc
{
    WORD cbLabel;           // Disambiguating ASCII label length, in bytes, greater than 0.
    LPSTR pszLabel;         // Disambiguating ASCII label, NUL terminator will be removed by schannel.
    WORD cbContextValue;    // Application context value length, in bytes, can be 0.
    PBYTE pbContextValue;   // Application context value, NULL if cbContextValue == 0.
    DWORD cbKeyingMaterial; // Requested keying material length, in bytes, greater than 0.
    PBYTE pbKeyingMaterial; // Exported keying material.
} SecPkgContext_KeyingMaterial_Inproc, * PSecPkgContext_KeyingMaterial_Inproc;

typedef struct _SecPkgContext_SrtpParameters
{
    WORD ProtectionProfile;         // Negotiated SRTP protection profile (0x0000 means no profile negotiated).
    BYTE MasterKeyIdentifierSize;   // Size in bytes of the SRTP master key identifier.
    _Field_size_bytes_(MasterKeyIdentifierSize) PBYTE MasterKeyIdentifier;// Negotiated SRTP master key identifier.
} SecPkgContext_SrtpParameters, * PSecPkgContext_SrtpParameters;

typedef struct _SecPkgContext_TokenBinding
{
    BYTE MajorVersion;      // Negotiated major version of the Token Binding protocol.
    BYTE MinorVersion;      // Negotiated minor version of the Token Binding protocol.
    WORD KeyParametersSize; // Size in bytes of the KeyParameters array.
    _Field_size_bytes_(KeyParametersSize) PBYTE KeyParameters; // IDs of the negotiated Token Binding key parameters.
} SecPkgContext_TokenBinding, * PSecPkgContext_TokenBinding;

typedef struct _SecPkgContext_CertificateValidationResult
{
    DWORD dwChainErrorStatus;    // Contains chain build error flags set by CertGetCertificateChain.
    HRESULT hrVerifyChainStatus; // Certificate validation policy error returned by CertVerifyCertificateChainPolicy.
} SecPkgContext_CertificateValidationResult, *PSecPkgContext_CertificateValidationResult;

//
// Schannel credentials data structure.
//

#define SCH_CRED_V1              0x00000001
#define SCH_CRED_V2              0x00000002  // for legacy code
#define SCH_CRED_VERSION         0x00000002  // for legacy code
#define SCH_CRED_V3              0x00000003  // for legacy code
#define SCHANNEL_CRED_VERSION    0x00000004  // for legacy code
#define SCH_CREDENTIALS_VERSION  0x00000005


struct _HMAPPER;

typedef struct _SCHANNEL_CRED
{
    DWORD           dwVersion;      // always SCHANNEL_CRED_VERSION
    DWORD           cCreds;
    PCCERT_CONTEXT *paCred;
    HCERTSTORE      hRootStore;

    DWORD           cMappers;
    struct _HMAPPER **aphMappers;

    DWORD           cSupportedAlgs;
    ALG_ID *        palgSupportedAlgs;

    DWORD           grbitEnabledProtocols;
    DWORD           dwMinimumCipherStrength;
    DWORD           dwMaximumCipherStrength;
    DWORD           dwSessionLifespan;
    DWORD           dwFlags;
    DWORD           dwCredFormat;
} SCHANNEL_CRED, *PSCHANNEL_CRED;


#ifdef SCHANNEL_USE_BLACKLISTS
    // Note, if you #define SCHANNEL_USE_BLACKLISTS
    // then you must define UNICODE_STRING and PUNICODE_STRING
    // or include Ntdef.h, SubAuth.h or Winternl.h.

// These values distinguish between the different RSA padding modes and can
// be specified in the CRYPTO_SETTINGS strCngAlgId field, in addition to the
// CNG algorithm identifiers.
#define SCHANNEL_RSA_PSS_PADDING_ALGORITHM L"SCH_RSA_PSS_PAD"
#define SCHANNEL_RSA_PKCS_PADDING_ALGORITHM L"SCH_RSA_PKCS_PAD"

typedef enum _eTlsAlgorithmUsage
{
    TlsParametersCngAlgUsageKeyExchange,          // Key exchange algorithm. RSA, ECHDE, DHE, etc.
    TlsParametersCngAlgUsageSignature,            // Signature algorithm. RSA, DSA, ECDSA, etc.
    TlsParametersCngAlgUsageCipher,               // Encryption algorithm. AES, DES, RC4, etc.
    TlsParametersCngAlgUsageDigest,               // Digest of cipher suite. SHA1, SHA256, SHA384, etc.
    TlsParametersCngAlgUsageCertSig               // Signature, hash and/or padding mode components of a TLS signature suite. RSA, DSA, ECDSA, SHA1, SHA256, PSS, etc.
} eTlsAlgorithmUsage;

//
// SCH_CREDENTIALS structures
//
typedef struct _CRYPTO_SETTINGS
{
    eTlsAlgorithmUsage  eAlgorithmUsage;         // How this algorithm is being used.
    UNICODE_STRING      strCngAlgId;             // CNG algorithm identifier.
    DWORD               cChainingModes;          // Set to 0 if CNG algorithm does not have a chaining mode.
    PUNICODE_STRING     rgstrChainingModes;      // Set to NULL if CNG algorithm does not have a chaining mode.
    DWORD               dwMinBitLength;          // Blacklist key sizes less than this. Set to 0 if not defined or CNG algorithm implies bit length.
    DWORD               dwMaxBitLength;          // Blacklist key sizes greater than this. Set to 0 if not defined or CNG algorithm implies bit length.
} CRYPTO_SETTINGS, *PCRYPTO_SETTINGS;

typedef struct _TLS_PARAMETERS
{
    DWORD               cAlpnIds;                // Valid for server applications only. Must be zero otherwise. Number of ALPN IDs in rgstrAlpnIds; set to 0 if applies to all.
    PUNICODE_STRING     rgstrAlpnIds;            // Valid for server applications only. Must be NULL otherwise. Array of ALPN IDs that the following settings apply to; set to NULL if applies to all.
    DWORD               grbitDisabledProtocols;  // List protocols you DO NOT want negotiated.
    DWORD               cDisabledCrypto;         // Number of CRYPTO_SETTINGS structures; set to 0 if there are none.
    PCRYPTO_SETTINGS    pDisabledCrypto;         // Array of CRYPTO_SETTINGS structures; set to NULL if there are none;
    DWORD               dwFlags;                 // Optional flags to pass; set to 0 if there are none.
} TLS_PARAMETERS, *PTLS_PARAMETERS;

#define TLS_PARAMS_OPTIONAL 0x00000001           // Valid for server applications only. Must be zero otherwise.
                                                 // TLS_PARAMETERS that will only be honored if they do not cause this server to terminate the handshake.

typedef struct _SCH_CREDENTIALS
{
    DWORD               dwVersion;               // Always SCH_CREDENTIALS_VERSION.
    DWORD               dwCredFormat;
    DWORD               cCreds;
    PCCERT_CONTEXT     *paCred;
    HCERTSTORE          hRootStore;

    DWORD               cMappers;
    struct _HMAPPER   **aphMappers;

    DWORD               dwSessionLifespan;
    DWORD               dwFlags;
    DWORD               cTlsParameters;
    PTLS_PARAMETERS     pTlsParameters;
} SCH_CREDENTIALS, *PSCH_CREDENTIALS;

#define SCH_CRED_MAX_SUPPORTED_PARAMETERS 16
#define SCH_CRED_MAX_SUPPORTED_ALPN_IDS 16
#define SCH_CRED_MAX_SUPPORTED_CRYPTO_SETTINGS 16
#define SCH_CRED_MAX_SUPPORTED_CHAINING_MODES 16

#endif

typedef struct _SEND_GENERIC_TLS_EXTENSION
{
    WORD  ExtensionType;            // Code point of extension.
    WORD  HandshakeType;            // Message type used to transport extension.
    DWORD Flags;                    // Flags used to modify behavior. Must be zero.
    WORD  BufferSize;               // Size in bytes of the extension data.
    UCHAR Buffer[ANYSIZE_ARRAY];    // Extension data.
} SEND_GENERIC_TLS_EXTENSION, *PSEND_GENERIC_TLS_EXTENSION;

typedef struct _TLS_EXTENSION_SUBSCRIPTION
{
    WORD ExtensionType; // Code point of extension.
    WORD HandshakeType; // Message type used to transport extension.
} TLS_EXTENSION_SUBSCRIPTION, *PTLS_EXTENSION_SUBSCRIPTION;

typedef struct _SUBSCRIBE_GENERIC_TLS_EXTENSION
{
    DWORD Flags;                                                // Flags used to modify behavior. Must be zero.
    DWORD SubscriptionsCount;                                   // Number of elements in the Subscriptions array.
    TLS_EXTENSION_SUBSCRIPTION Subscriptions[ANYSIZE_ARRAY];    // Array of TLS_EXTENSION_SUBSCRIPTION structures.
} SUBSCRIBE_GENERIC_TLS_EXTENSION, *PSUBSCRIBE_GENERIC_TLS_EXTENSION;

// Maximum number of TLS_EXTENSION_SUBSCRIPTION structures allowed.
#define SCH_MAX_EXT_SUBSCRIPTIONS 2

// Values for SCHANNEL_CRED dwCredFormat field.
#define SCH_CRED_FORMAT_CERT_CONTEXT    0x00000000
#define SCH_CRED_FORMAT_CERT_HASH       0x00000001
#define SCH_CRED_FORMAT_CERT_HASH_STORE 0x00000002

#define SCH_CRED_MAX_STORE_NAME_SIZE    128
#define SCH_CRED_MAX_SUPPORTED_ALGS     256
#define SCH_CRED_MAX_SUPPORTED_CERTS    100

typedef struct _SCHANNEL_CERT_HASH
{
    DWORD           dwLength;
    DWORD           dwFlags;
    HCRYPTPROV      hProv;
    BYTE            ShaHash[20];
} SCHANNEL_CERT_HASH, *PSCHANNEL_CERT_HASH;

typedef struct _SCHANNEL_CERT_HASH_STORE
{
    DWORD           dwLength;
    DWORD           dwFlags;
    HCRYPTPROV      hProv;
    BYTE            ShaHash[20];
    WCHAR           pwszStoreName[SCH_CRED_MAX_STORE_NAME_SIZE];
} SCHANNEL_CERT_HASH_STORE, *PSCHANNEL_CERT_HASH_STORE;

// Values for SCHANNEL_CERT_HASH dwFlags field.
#define SCH_MACHINE_CERT_HASH           0x00000001


//+-------------------------------------------------------------------------
// Flags for use with SCHANNEL_CRED
//
// SCH_CRED_NO_SYSTEM_MAPPER
//      This flag is intended for use by server applications only. If this
//      flag is set, then schannel does *not* attempt to map received client
//      certificate chains to an NT user account using the built-in system
//      certificate mapper.This flag is ignored by non-NT5 versions of
//      schannel.
//
// SCH_CRED_NO_SERVERNAME_CHECK
//      This flag is intended for use by client applications only. If this
//      flag is set, then when schannel validates the received server
//      certificate chain, is does *not* compare the passed in target name
//      with the subject name embedded in the certificate. This flag is
//      ignored by non-NT5 versions of schannel. This flag is also ignored
//      if the SCH_CRED_MANUAL_CRED_VALIDATION flag is set.
//
// SCH_CRED_MANUAL_CRED_VALIDATION
//      This flag is intended for use by client applications only. If this
//      flag is set, then schannel will *not* automatically attempt to
//      validate the received server certificate chain. This flag is
//      ignored by non-NT5 versions of schannel, but all client applications
//      that wish to validate the certificate chain themselves should
//      specify this flag, so that there's at least a chance they'll run
//      correctly on NT5.
//
// SCH_CRED_DEFERRED_CRED_VALIDATION
//      This flag is intended for use by client applications only. If this
//      flag is set, then schannel will attempt to validate the received
//      server certificate chain, however, schannel will not terminate the
//      handshake with an error if the chain has any issues. Rather, the
//      client is expected to retrieve the the result of the chain build
//      using SECPKG_ATTR_CERT_CHECK_RESULT at any point after
//      receiving the server certificate or by using 
//      SECPKG_ATTR_CERT_CHECK_RESULT_INPROC after the handshake
//      has concluded.
//
// SCH_CRED_NO_DEFAULT_CREDS
//      This flag is intended for use by client applications only. If this
//      flag is set, and the server requests client authentication, then
//      schannel will *not* attempt to automatically acquire a suitable
//      default client certificate chain. This flag is ignored by non-NT5
//      versions of schannel, but all client applications that wish to
//      manually specify their certicate chains should specify this flag,
//      so that there's at least a chance they'll run correctly on NT5.
//
// SCH_CRED_AUTO_CRED_VALIDATION
//      This flag is the opposite of SCH_CRED_MANUAL_CRED_VALIDATION.
//      Conservatively written client applications will always specify one
//      flag or the other.
//
// SCH_CRED_USE_DEFAULT_CREDS
//      This flag is the opposite of SCH_CRED_NO_DEFAULT_CREDS.
//      Conservatively written client applications will always specify one
//      flag or the other.
//
// SCH_CRED_DISABLE_RECONNECTS
//      This flag is intended for use by server applications only. If this
//      flag is set, then full handshakes performed with this credential
//      will not be marked suitable for reconnects. A cache entry will still
//      be created, however, so the session can be made resumable later
//      via a call to ApplyControlToken.
//
//
// SCH_CRED_REVOCATION_CHECK_END_CERT
// SCH_CRED_REVOCATION_CHECK_CHAIN
// SCH_CRED_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT
//      These flags specify that when schannel automatically validates a
//      received certificate chain, some or all of the certificates are to
//      be checked for revocation. Only one of these flags may be specified.
//      See the CertGetCertificateChain function. These flags are ignored by
//      non-NT5 versions of schannel.
//
// SCH_CRED_IGNORE_NO_REVOCATION_CHECK
// SCH_CRED_IGNORE_REVOCATION_OFFLINE
//      These flags instruct schannel to ignore the
//      CRYPT_E_NO_REVOCATION_CHECK and CRYPT_E_REVOCATION_OFFLINE errors
//      respectively if they are encountered when attempting to check the
//      revocation status of a received certificate chain. These flags are
//      ignored if none of the above flags are set.
//
// SCH_CRED_CACHE_ONLY_URL_RETRIEVAL_ON_CREATE
//      This flag instructs schannel to pass CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL
//      flags to CertGetCertificateChain when validating the specified
//      credentials during a call to AcquireCredentialsHandle. The default for
//      vista is to not specify CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL. Use
//      SCH_CRED_CACHE_ONLY_URL_RETRIEVAL_ON_CREATE to override this behavior.
//      NOTE: Prior to Vista, this flag(CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL) was
//      specified by default.
//
//  SCH_SEND_ROOT_CERT
//      This flag instructs schannel to send the root cert as part of the
//      certificate message.
//
//  SCH_SEND_AUX_RECORD
//      This flag instructs schannel to split application records.
//
//  SCH_USE_STRONG_CRYPTO
//      This flag instructs schannel to disable known weak cryptographic
//      algorithms, cipher suites and SSL/TLS protocol versions that may be
//      otherwise enabled for better interoperability.
//
//  SCH_USE_PRESHAREDKEY_ONLY
//      This flag instructs schannel to select only PSK cipher suites and
//      disable all other cipher suites.
//
//  SCH_USE_DTLS_ONLY
//      This flag instructs schannel to select only DTLS protocols.
//
//  SCH_ALLOW_NULL_ENCRYPTION
//      This flag instructs schannel to allow NULL encryption cipher suites.
//      For example: TLS_RSA_WITH_NULL_SHA256.
//+-------------------------------------------------------------------------
#define SCH_CRED_NO_SYSTEM_MAPPER                    0x00000002
#define SCH_CRED_NO_SERVERNAME_CHECK                 0x00000004
#define SCH_CRED_MANUAL_CRED_VALIDATION              0x00000008
#define SCH_CRED_NO_DEFAULT_CREDS                    0x00000010
#define SCH_CRED_AUTO_CRED_VALIDATION                0x00000020
#define SCH_CRED_USE_DEFAULT_CREDS                   0x00000040
#define SCH_CRED_DISABLE_RECONNECTS                  0x00000080

#define SCH_CRED_REVOCATION_CHECK_END_CERT           0x00000100
#define SCH_CRED_REVOCATION_CHECK_CHAIN              0x00000200
#define SCH_CRED_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT 0x00000400
#define SCH_CRED_IGNORE_NO_REVOCATION_CHECK          0x00000800
#define SCH_CRED_IGNORE_REVOCATION_OFFLINE           0x00001000

#define SCH_CRED_RESTRICTED_ROOTS                    0x00002000
#define SCH_CRED_REVOCATION_CHECK_CACHE_ONLY         0x00004000
#define SCH_CRED_CACHE_ONLY_URL_RETRIEVAL            0x00008000

#define SCH_CRED_MEMORY_STORE_CERT                   0x00010000

#define SCH_CRED_CACHE_ONLY_URL_RETRIEVAL_ON_CREATE  0x00020000

#define SCH_SEND_ROOT_CERT                           0x00040000
#define SCH_CRED_SNI_CREDENTIAL                      0x00080000
#define SCH_CRED_SNI_ENABLE_OCSP                     0x00100000
#define SCH_SEND_AUX_RECORD                          0x00200000
#define SCH_USE_STRONG_CRYPTO                        0x00400000
#define SCH_USE_PRESHAREDKEY_ONLY                    0x00800000
#define SCH_USE_DTLS_ONLY                            0x01000000
#define SCH_ALLOW_NULL_ENCRYPTION                    0x02000000
#define SCH_CRED_DEFERRED_CRED_VALIDATION            0x04000000

//
//
// ApplyControlToken PkgParams types
//
// These identifiers are the DWORD types
// to be passed into ApplyControlToken
// through a PkgParams buffer.

#define SCHANNEL_RENEGOTIATE    0   // renegotiate a connection
#define SCHANNEL_SHUTDOWN       1   // gracefully close down a connection
#define SCHANNEL_ALERT          2   // build an error message
#define SCHANNEL_SESSION        3   // session control


// Alert token structure.
typedef struct _SCHANNEL_ALERT_TOKEN
{
    DWORD   dwTokenType;            // SCHANNEL_ALERT
    DWORD   dwAlertType;
    DWORD   dwAlertNumber;
} SCHANNEL_ALERT_TOKEN;

// Alert types.
#define TLS1_ALERT_WARNING              1
#define TLS1_ALERT_FATAL                2

// Alert messages.
#define TLS1_ALERT_CLOSE_NOTIFY                 0       // warning
#define TLS1_ALERT_UNEXPECTED_MESSAGE           10      // error
#define TLS1_ALERT_BAD_RECORD_MAC               20      // error
#define TLS1_ALERT_DECRYPTION_FAILED            21      // reserved
#define TLS1_ALERT_RECORD_OVERFLOW              22      // error
#define TLS1_ALERT_DECOMPRESSION_FAIL           30      // error
#define TLS1_ALERT_HANDSHAKE_FAILURE            40      // error
#define TLS1_ALERT_BAD_CERTIFICATE              42      // warning or error
#define TLS1_ALERT_UNSUPPORTED_CERT             43      // warning or error
#define TLS1_ALERT_CERTIFICATE_REVOKED          44      // warning or error
#define TLS1_ALERT_CERTIFICATE_EXPIRED          45      // warning or error
#define TLS1_ALERT_CERTIFICATE_UNKNOWN          46      // warning or error
#define TLS1_ALERT_ILLEGAL_PARAMETER            47      // error
#define TLS1_ALERT_UNKNOWN_CA                   48      // error
#define TLS1_ALERT_ACCESS_DENIED                49      // error
#define TLS1_ALERT_DECODE_ERROR                 50      // error
#define TLS1_ALERT_DECRYPT_ERROR                51      // error
#define TLS1_ALERT_EXPORT_RESTRICTION           60      // reserved
#define TLS1_ALERT_PROTOCOL_VERSION             70      // error
#define TLS1_ALERT_INSUFFIENT_SECURITY          71      // error
#define TLS1_ALERT_INTERNAL_ERROR               80      // error
#define TLS1_ALERT_USER_CANCELED                90      // warning or error
#define TLS1_ALERT_NO_RENEGOTIATION             100     // warning
#define TLS1_ALERT_UNSUPPORTED_EXT              110     // error
#define TLS1_ALERT_UNKNOWN_PSK_IDENTITY         115     // error
#define TLS1_ALERT_NO_APP_PROTOCOL              120     // error

// Session control flags
#define SSL_SESSION_ENABLE_RECONNECTS   1
#define SSL_SESSION_DISABLE_RECONNECTS  2

// Session control token structure.
typedef struct _SCHANNEL_SESSION_TOKEN
{
    DWORD   dwTokenType;        // SCHANNEL_SESSION
    DWORD   dwFlags;
} SCHANNEL_SESSION_TOKEN;


typedef struct _SCHANNEL_CLIENT_SIGNATURE
{
    DWORD       cbLength;
    ALG_ID      aiHash;
    DWORD       cbHash;
    BYTE        HashValue[36];
    BYTE        CertThumbprint[20];
} SCHANNEL_CLIENT_SIGNATURE, *PSCHANNEL_CLIENT_SIGNATURE;


//
// Flags for identifying the various different protocols.
//

/* flag/identifiers for protocols we support */
#define SP_PROT_PCT1_SERVER             0x00000001
#define SP_PROT_PCT1_CLIENT             0x00000002
#define SP_PROT_PCT1                    (SP_PROT_PCT1_SERVER | SP_PROT_PCT1_CLIENT)

#define SP_PROT_SSL2_SERVER             0x00000004
#define SP_PROT_SSL2_CLIENT             0x00000008
#define SP_PROT_SSL2                    (SP_PROT_SSL2_SERVER | SP_PROT_SSL2_CLIENT)

#define SP_PROT_SSL3_SERVER             0x00000010
#define SP_PROT_SSL3_CLIENT             0x00000020
#define SP_PROT_SSL3                    (SP_PROT_SSL3_SERVER | SP_PROT_SSL3_CLIENT)

#define SP_PROT_TLS1_SERVER             0x00000040
#define SP_PROT_TLS1_CLIENT             0x00000080
#define SP_PROT_TLS1                    (SP_PROT_TLS1_SERVER | SP_PROT_TLS1_CLIENT)

#define SP_PROT_SSL3TLS1_CLIENTS        (SP_PROT_TLS1_CLIENT | SP_PROT_SSL3_CLIENT)
#define SP_PROT_SSL3TLS1_SERVERS        (SP_PROT_TLS1_SERVER | SP_PROT_SSL3_SERVER)
#define SP_PROT_SSL3TLS1                (SP_PROT_SSL3 | SP_PROT_TLS1)

#define SP_PROT_UNI_SERVER              0x40000000
#define SP_PROT_UNI_CLIENT              0x80000000
#define SP_PROT_UNI                     (SP_PROT_UNI_SERVER | SP_PROT_UNI_CLIENT)

#define SP_PROT_ALL                     0xffffffff
#define SP_PROT_NONE                    0
#define SP_PROT_CLIENTS                 (SP_PROT_PCT1_CLIENT | SP_PROT_SSL2_CLIENT | SP_PROT_SSL3_CLIENT | SP_PROT_UNI_CLIENT | SP_PROT_TLS1_CLIENT)
#define SP_PROT_SERVERS                 (SP_PROT_PCT1_SERVER | SP_PROT_SSL2_SERVER | SP_PROT_SSL3_SERVER | SP_PROT_UNI_SERVER | SP_PROT_TLS1_SERVER)


#define SP_PROT_TLS1_0_SERVER           SP_PROT_TLS1_SERVER
#define SP_PROT_TLS1_0_CLIENT           SP_PROT_TLS1_CLIENT
#define SP_PROT_TLS1_0                  (SP_PROT_TLS1_0_SERVER | \
                                         SP_PROT_TLS1_0_CLIENT)

#define SP_PROT_TLS1_1_SERVER           0x00000100
#define SP_PROT_TLS1_1_CLIENT           0x00000200
#define SP_PROT_TLS1_1                  (SP_PROT_TLS1_1_SERVER | \
                                         SP_PROT_TLS1_1_CLIENT)

#define SP_PROT_TLS1_2_SERVER           0x00000400
#define SP_PROT_TLS1_2_CLIENT           0x00000800
#define SP_PROT_TLS1_2                  (SP_PROT_TLS1_2_SERVER | \
                                         SP_PROT_TLS1_2_CLIENT)

#define SP_PROT_TLS1_3_SERVER           0x00001000
#define SP_PROT_TLS1_3_CLIENT           0x00002000
#define SP_PROT_TLS1_3                  (SP_PROT_TLS1_3_SERVER | \
                                         SP_PROT_TLS1_3_CLIENT)

#define SP_PROT_DTLS_SERVER             0x00010000
#define SP_PROT_DTLS_CLIENT             0x00020000
#define SP_PROT_DTLS                    (SP_PROT_DTLS_SERVER | \
                                         SP_PROT_DTLS_CLIENT )

#define SP_PROT_DTLS1_0_SERVER          SP_PROT_DTLS_SERVER
#define SP_PROT_DTLS1_0_CLIENT          SP_PROT_DTLS_CLIENT
#define SP_PROT_DTLS1_0                 (SP_PROT_DTLS1_0_SERVER | SP_PROT_DTLS1_0_CLIENT)

#define SP_PROT_DTLS1_2_SERVER          0x00040000
#define SP_PROT_DTLS1_2_CLIENT          0x00080000
#define SP_PROT_DTLS1_2                 (SP_PROT_DTLS1_2_SERVER | SP_PROT_DTLS1_2_CLIENT)

#define SP_PROT_DTLS1_X_SERVER          (SP_PROT_DTLS1_0_SERVER | \
                                         SP_PROT_DTLS1_2_SERVER)

#define SP_PROT_DTLS1_X_CLIENT          (SP_PROT_DTLS1_0_CLIENT | \
                                         SP_PROT_DTLS1_2_CLIENT)

#define SP_PROT_DTLS1_X                 (SP_PROT_DTLS1_X_SERVER | \
                                         SP_PROT_DTLS1_X_CLIENT)

#define SP_PROT_TLS1_1PLUS_SERVER       (SP_PROT_TLS1_1_SERVER | \
                                         SP_PROT_TLS1_2_SERVER | \
                                         SP_PROT_TLS1_3_SERVER)
#define SP_PROT_TLS1_1PLUS_CLIENT       (SP_PROT_TLS1_1_CLIENT | \
                                         SP_PROT_TLS1_2_CLIENT | \
                                         SP_PROT_TLS1_3_CLIENT)

#define SP_PROT_TLS1_1PLUS              (SP_PROT_TLS1_1PLUS_SERVER | \
                                         SP_PROT_TLS1_1PLUS_CLIENT)

#define SP_PROT_TLS1_3PLUS_SERVER       SP_PROT_TLS1_3_SERVER
#define SP_PROT_TLS1_3PLUS_CLIENT       SP_PROT_TLS1_3_CLIENT
#define SP_PROT_TLS1_3PLUS              (SP_PROT_TLS1_3PLUS_SERVER | \
                                         SP_PROT_TLS1_3PLUS_CLIENT)

#define SP_PROT_TLS1_X_SERVER           (SP_PROT_TLS1_0_SERVER | \
                                         SP_PROT_TLS1_1_SERVER | \
                                         SP_PROT_TLS1_2_SERVER | \
                                         SP_PROT_TLS1_3_SERVER)
#define SP_PROT_TLS1_X_CLIENT           (SP_PROT_TLS1_0_CLIENT | \
                                         SP_PROT_TLS1_1_CLIENT | \
                                         SP_PROT_TLS1_2_CLIENT | \
                                         SP_PROT_TLS1_3_CLIENT)
#define SP_PROT_TLS1_X                  (SP_PROT_TLS1_X_SERVER | \
                                         SP_PROT_TLS1_X_CLIENT)

#define SP_PROT_SSL3TLS1_X_CLIENTS      (SP_PROT_TLS1_X_CLIENT | \
                                         SP_PROT_SSL3_CLIENT)
#define SP_PROT_SSL3TLS1_X_SERVERS      (SP_PROT_TLS1_X_SERVER | \
                                         SP_PROT_SSL3_SERVER)
#define SP_PROT_SSL3TLS1_X              (SP_PROT_SSL3 | SP_PROT_TLS1_X)

#define SP_PROT_X_CLIENTS               (SP_PROT_CLIENTS | \
                                         SP_PROT_TLS1_X_CLIENT | \
                                         SP_PROT_DTLS1_X_CLIENT )
#define SP_PROT_X_SERVERS               (SP_PROT_SERVERS | \
                                         SP_PROT_TLS1_X_SERVER | \
                                         SP_PROT_DTLS1_X_SERVER )



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


//
// Helper function used to flush the SSL session cache.
//

typedef BOOL
(WINAPI * SSL_EMPTY_CACHE_FN_A)(
    LPSTR  pszTargetName,
    DWORD  dwFlags);

BOOL
WINAPI
SslEmptyCacheA(_In_ LPSTR  pszTargetName,
               _In_ DWORD  dwFlags);

typedef BOOL
(WINAPI * SSL_EMPTY_CACHE_FN_W)(
    LPWSTR pszTargetName,
    DWORD  dwFlags);

BOOL
WINAPI
SslEmptyCacheW(_In_ LPWSTR pszTargetName,
               _In_ DWORD  dwFlags);

#ifdef UNICODE
#define SSL_EMPTY_CACHE_FN SSL_EMPTY_CACHE_FN_W
#define SslEmptyCache SslEmptyCacheW
#else
#define SSL_EMPTY_CACHE_FN SSL_EMPTY_CACHE_FN_A
#define SslEmptyCache SslEmptyCacheA
#endif



// Structures for compatability with the
// NT 4.0 SP2 / IE 3.0 schannel interface, do
// not use.

typedef struct _SSL_CREDENTIAL_CERTIFICATE {
    DWORD   cbPrivateKey;
    PBYTE   pPrivateKey;
    DWORD   cbCertificate;
    PBYTE   pCertificate;
    PSTR    pszPassword;
} SSL_CREDENTIAL_CERTIFICATE, * PSSL_CREDENTIAL_CERTIFICATE;




// Structures for use with the
// NT 4.0 SP3 Schannel interface,
// do not use.
#define SCHANNEL_SECRET_TYPE_CAPI   0x00000001
#define SCHANNEL_SECRET_PRIVKEY     0x00000002
#define SCH_CRED_X509_CERTCHAIN     0x00000001
#define SCH_CRED_X509_CAPI          0x00000002
#define SCH_CRED_CERT_CONTEXT       0x00000003

struct _HMAPPER;
typedef struct _SCH_CRED
{
    DWORD     dwVersion;                // always SCH_CRED_VERSION.
    DWORD     cCreds;                   // Number of credentials.
    PVOID     *paSecret;                // Array of SCH_CRED_SECRET_* pointers
    PVOID     *paPublic;                // Array of SCH_CRED_PUBLIC_* pointers
    DWORD     cMappers;                 // Number of credential mappers.
    struct _HMAPPER   **aphMappers;     // pointer to an array of pointers to credential mappers
} SCH_CRED, * PSCH_CRED;

// Structures for use with the
// NT 4.0 SP3 Schannel interface,
// do not use.
typedef struct _SCH_CRED_SECRET_CAPI
{
    DWORD           dwType;      // SCHANNEL_SECRET_TYPE_CAPI
    HCRYPTPROV      hProv;       // credential secret information.

} SCH_CRED_SECRET_CAPI, * PSCH_CRED_SECRET_CAPI;


// Structures for use with the
// NT 4.0 SP3 Schannel interface,
// do not use.
typedef struct _SCH_CRED_SECRET_PRIVKEY
{
    DWORD           dwType;       // SCHANNEL_SECRET_PRIVKEY
    PBYTE           pPrivateKey;   // Der encoded private key
    DWORD           cbPrivateKey;
    PSTR            pszPassword;  // Password to crack the private key.

} SCH_CRED_SECRET_PRIVKEY, * PSCH_CRED_SECRET_PRIVKEY;


// Structures for use with the
// NT 4.0 SP3 Schannel interface,
// do not use.
typedef struct _SCH_CRED_PUBLIC_CERTCHAIN
{
    DWORD       dwType;
    DWORD       cbCertChain;
    PBYTE       pCertChain;
} SCH_CRED_PUBLIC_CERTCHAIN, *PSCH_CRED_PUBLIC_CERTCHAIN;


// Structures needed for Pre NT4.0 SP2 calls.
typedef struct _PctPublicKey
{
    DWORD Type;
    DWORD cbKey;
    UCHAR pKey[1];
} PctPublicKey;

typedef struct _X509Certificate {
    DWORD           Version;
    DWORD           SerialNumber[4];
    ALG_ID          SignatureAlgorithm;
    FILETIME        ValidFrom;
    FILETIME        ValidUntil;
    PSTR            pszIssuer;
    PSTR            pszSubject;
    PctPublicKey    *pPublicKey;
} X509Certificate, * PX509Certificate;



// Pre NT4.0 SP2 calls.  Call CAPI1 or CAPI2
// to get the same functionality instead.
BOOL
WINAPI
SslGenerateKeyPair(
    PSSL_CREDENTIAL_CERTIFICATE pCerts,
    _In_ PSTR pszDN,
    _In_ PSTR pszPassword,
    DWORD Bits );

// Pre NT4.0 SP2 calls.  Call CAPI1 or CAPI2
// to get the same functionality instead.
VOID
WINAPI
SslGenerateRandomBits(
    PUCHAR      pRandomData,
    LONG        cRandomData
    );

// Pre NT4.0 SP2 calls.  Call CAPI1 or CAPI2
// to get the same functionality instead.
BOOL
WINAPI
SslCrackCertificate(
    PUCHAR              pbCertificate,
    DWORD               cbCertificate,
    DWORD               dwFlags,
    PX509Certificate *  ppCertificate
    );

// Pre NT4.0 SP2 calls.  Call CAPI1 or CAPI2
// to get the same functionality instead.
VOID
WINAPI
SslFreeCertificate(
    PX509Certificate    pCertificate
    );

DWORD
WINAPI
SslGetMaximumKeySize(
    DWORD   Reserved );

BOOL
WINAPI
SslGetDefaultIssuers(
    PBYTE pbIssuers,
    DWORD *pcbIssuers);

#define SSL_CRACK_CERTIFICATE_NAME  TEXT("SslCrackCertificate")
#define SSL_FREE_CERTIFICATE_NAME   TEXT("SslFreeCertificate")

// Pre NT4.0 SP2 calls.  Call CAPI1 or CAPI2
// to get the same functionality instead.
typedef BOOL
(WINAPI * SSL_CRACK_CERTIFICATE_FN)
(
    PUCHAR              pbCertificate,
    DWORD               cbCertificate,
    BOOL                VerifySignature,
    PX509Certificate *  ppCertificate
);


// Pre NT4.0 SP2 calls.  Call CAPI1 or CAPI2
// to get the same functionality instead.
typedef VOID
(WINAPI * SSL_FREE_CERTIFICATE_FN)
(
    PX509Certificate    pCertificate
);


typedef SECURITY_STATUS
(WINAPI * SslGetServerIdentityFn)
(
    _In_reads_bytes_(ClientHelloSize)            PBYTE       ClientHello,
    _In_                                    DWORD       ClientHelloSize,
    _Outptr_result_bytebuffer_(*ServerIdentitySize) PBYTE*      ServerIdentity,
    _Out_                                   PDWORD      ServerIdentitySize,
    _In_                                    DWORD       Flags
);

EXTERN_C
SECURITY_STATUS
WINAPI
SslGetServerIdentity(
    _In_reads_bytes_(ClientHelloSize)            PBYTE       ClientHello,
    _In_                                    DWORD       ClientHelloSize,
    _Outptr_result_bytebuffer_(*ServerIdentitySize) PBYTE*      ServerIdentity,
    _Out_                                   PDWORD      ServerIdentitySize,
    _In_                                    DWORD       Flags);

#if (NTDDI_VERSION >= NTDDI_WIN10_19H1)
typedef struct _SCH_EXTENSION_DATA
{
    WORD ExtensionType;
    const BYTE* pExtData;
    DWORD cbExtData;
}SCH_EXTENSION_DATA;

typedef enum _SchGetExtensionsOptions
{
    SCH_EXTENSIONS_OPTIONS_NONE = 0x0, 
    SCH_NO_RECORD_HEADER = 0x1  // Specifies that the ClientHello message does not contain the record header.
}SchGetExtensionsOptions;
DEFINE_ENUM_FLAG_OPERATORS(SchGetExtensionsOptions);

typedef SECURITY_STATUS
(WINAPI * SslGetExtensionsFn)
(
    _In_reads_(clientHelloByteSize) const BYTE* clientHello,
    _In_ DWORD clientHelloByteSize,
    _Inout_updates_(genericExtensionsCount) SCH_EXTENSION_DATA* genericExtensions,
    _In_ BYTE genericExtensionsCount,
    _Out_ DWORD* bytesToRead,
    _In_ SchGetExtensionsOptions flags
);

EXTERN_C
SECURITY_STATUS
WINAPI
SslGetExtensions(
    _In_reads_(clientHelloByteSize) const BYTE* clientHello,
    _In_ DWORD clientHelloByteSize,
    _Inout_updates_(genericExtensionsCount) SCH_EXTENSION_DATA* genericExtensions,
    _In_ BYTE genericExtensionsCount,
    _Out_ DWORD* bytesToRead,
    _In_ SchGetExtensionsOptions flags
);

typedef SECURITY_STATUS
(WINAPI * SslDeserializeCertificateStoreFn)
(
    _In_ CERT_BLOB SerializedCertificateStore,
    _Outptr_result_maybenull_ PCCERT_CONTEXT *ppCertContext
);

// Deserializes the certificate store provided by QueryContextAttributes for
// SECPKG_ATTR_SERIALIZED_REMOTE_CERT_CONTEXT[_INPROC]. The certificate context of the remote peer's
// leaf certificate is returned. The caller must free *ppCertContext when done by calling
// CertFreeCertificateContext.
EXTERN_C
SECURITY_STATUS
WINAPI
SslDeserializeCertificateStore(
    _In_ CERT_BLOB SerializedCertificateStore,
    _Outptr_result_maybenull_ PCCERT_CONTEXT *ppCertContext
);

#endif // (NTDDI_VERSION >= NTDDI_WIN10_19H1)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif //__SCHANNEL_H__
