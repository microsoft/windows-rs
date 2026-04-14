#include <winapifamily.h>
//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 2004.
//
//  File:       ncrypt.h
//
//  Contents:   Cryptographic API Prototypes and Definitions
//
//----------------------------------------------------------------------------


#ifndef __NCRYPT_H__
#define __NCRYPT_H__

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

#ifdef __cplusplus
extern "C" {
#endif

#ifndef WINAPI
#define WINAPI __stdcall
#endif

#ifndef __SECSTATUS_DEFINED__
typedef LONG SECURITY_STATUS;
#define __SECSTATUS_DEFINED__
#endif

#include <bcrypt.h>

#ifndef HCRYPTPROV_DEFINED
#define HCRYPTPROV_DEFINED
typedef ULONG_PTR HCRYPTPROV;
typedef ULONG_PTR HCRYPTKEY;
typedef ULONG_PTR HCRYPTHASH;
#endif

//
// Maximum length of Key name, in characters
//
#define NCRYPT_MAX_KEY_NAME_LENGTH      512


//
// Maximum length of Algorithm name, in characters
//
#define NCRYPT_MAX_ALG_ID_LENGTH        512

//***************************************************************************
//
//  NCRYPT memory management routines for functions that require
//  the caller to allocate memory
//
//****************************************************************************
typedef LPVOID (WINAPI *PFN_NCRYPT_ALLOC)(
    _In_ SIZE_T cbSize
    );

typedef VOID (WINAPI *PFN_NCRYPT_FREE)(
    _In_ LPVOID pv
    );

typedef struct NCRYPT_ALLOC_PARA {
    DWORD                   cbSize;     // size of this structure
    PFN_NCRYPT_ALLOC        pfnAlloc;
    PFN_NCRYPT_FREE         pfnFree;
} NCRYPT_ALLOC_PARA;

//
// Microsoft built-in providers.
//

#define MS_KEY_STORAGE_PROVIDER             L"Microsoft Software Key Storage Provider"
#define MS_SMART_CARD_KEY_STORAGE_PROVIDER  L"Microsoft Smart Card Key Storage Provider"
#define MS_PLATFORM_KEY_STORAGE_PROVIDER    L"Microsoft Platform Crypto Provider"
#define MS_NGC_KEY_STORAGE_PROVIDER         L"Microsoft Passport Key Storage Provider"
#define MS_PLUTON_CRYPTO_PROVIDER           L"Microsoft Pluton Cryptographic Provider"

//
// Key name for sealing
//
#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
#define TPM_RSA_SRK_SEAL_KEY            L"MICROSOFT_PCP_KSP_RSA_SEAL_KEY_3BD1C4BF-004E-4E2F-8A4D-0BF633DCB074"
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS1)

//
// Common algorithm identifiers.
//

#define NCRYPT_RSA_ALGORITHM            BCRYPT_RSA_ALGORITHM
#define NCRYPT_RSA_SIGN_ALGORITHM       BCRYPT_RSA_SIGN_ALGORITHM
#define NCRYPT_DH_ALGORITHM             BCRYPT_DH_ALGORITHM
#define NCRYPT_DSA_ALGORITHM            BCRYPT_DSA_ALGORITHM
#define NCRYPT_MD2_ALGORITHM            BCRYPT_MD2_ALGORITHM
#define NCRYPT_MD4_ALGORITHM            BCRYPT_MD4_ALGORITHM
#define NCRYPT_MD5_ALGORITHM            BCRYPT_MD5_ALGORITHM
#define NCRYPT_SHA1_ALGORITHM           BCRYPT_SHA1_ALGORITHM
#define NCRYPT_SHA256_ALGORITHM         BCRYPT_SHA256_ALGORITHM
#define NCRYPT_SHA384_ALGORITHM         BCRYPT_SHA384_ALGORITHM
#define NCRYPT_SHA512_ALGORITHM         BCRYPT_SHA512_ALGORITHM
#define NCRYPT_ECDSA_P256_ALGORITHM     BCRYPT_ECDSA_P256_ALGORITHM
#define NCRYPT_ECDSA_P384_ALGORITHM     BCRYPT_ECDSA_P384_ALGORITHM
#define NCRYPT_ECDSA_P521_ALGORITHM     BCRYPT_ECDSA_P521_ALGORITHM
#define NCRYPT_ECDH_P256_ALGORITHM      BCRYPT_ECDH_P256_ALGORITHM
#define NCRYPT_ECDH_P384_ALGORITHM      BCRYPT_ECDH_P384_ALGORITHM
#define NCRYPT_ECDH_P521_ALGORITHM      BCRYPT_ECDH_P521_ALGORITHM

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define NCRYPT_AES_ALGORITHM            BCRYPT_AES_ALGORITHM
#define NCRYPT_RC2_ALGORITHM            BCRYPT_RC2_ALGORITHM
#define NCRYPT_3DES_ALGORITHM           BCRYPT_3DES_ALGORITHM
#define NCRYPT_DES_ALGORITHM            BCRYPT_DES_ALGORITHM
#define NCRYPT_DESX_ALGORITHM           BCRYPT_DESX_ALGORITHM
#define NCRYPT_3DES_112_ALGORITHM       BCRYPT_3DES_112_ALGORITHM

#define NCRYPT_SP800108_CTR_HMAC_ALGORITHM  BCRYPT_SP800108_CTR_HMAC_ALGORITHM
#define NCRYPT_SP80056A_CONCAT_ALGORITHM    BCRYPT_SP80056A_CONCAT_ALGORITHM
#define NCRYPT_PBKDF2_ALGORITHM             BCRYPT_PBKDF2_ALGORITHM
#define NCRYPT_CAPI_KDF_ALGORITHM           BCRYPT_CAPI_KDF_ALGORITHM
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define NCRYPT_ECDSA_ALGORITHM          BCRYPT_ECDSA_ALGORITHM
#define NCRYPT_ECDH_ALGORITHM           BCRYPT_ECDH_ALGORITHM
#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#define NCRYPT_KEY_STORAGE_ALGORITHM            L"KEY_STORAGE"

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
//
// This algorithm is not supported by any BCrypt provider. This identifier is for creating
// persistent stored HMAC keys in the TPM KSP.
//
#define NCRYPT_HMAC_SHA256_ALGORITHM            L"HMAC-SHA256"
#endif

#if (NTDDI_VERSION >= NTDDI_WIN11_GE)

// ML-KEM
#define NCRYPT_MLKEM_ALGORITHM   BCRYPT_MLKEM_ALGORITHM

// ML-DSA
#define NCRYPT_MLDSA_ALGORITHM    BCRYPT_MLDSA_ALGORITHM

// SLH-DSA
#define NCRYPT_SLHDSA_ALGORITHM   BCRYPT_SLHDSA_ALGORITHM

// LMS and XMSS
#define NCRYPT_LMS_ALGORITHM      BCRYPT_LMS_ALGORITHM
#define NCRYPT_XMSS_ALGORITHM     BCRYPT_XMSS_ALGORITHM

#endif

//
// Interfaces
//
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define NCRYPT_CIPHER_INTERFACE                 BCRYPT_CIPHER_INTERFACE
#endif // (NTDDI_VERSION >= NTDDI_WIN8)
#define NCRYPT_HASH_INTERFACE                   BCRYPT_HASH_INTERFACE
#define NCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE  BCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE
#define NCRYPT_SECRET_AGREEMENT_INTERFACE       BCRYPT_SECRET_AGREEMENT_INTERFACE
#define NCRYPT_SIGNATURE_INTERFACE              BCRYPT_SIGNATURE_INTERFACE
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define NCRYPT_KEY_DERIVATION_INTERFACE         BCRYPT_KEY_DERIVATION_INTERFACE
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WIN11_GE)
#define NCRYPT_KEY_ENCAPSULATION_INTERFACE      BCRYPT_KEY_ENCAPSULATION_INTERFACE
#endif

#define NCRYPT_KEY_STORAGE_INTERFACE            0x00010001
#define NCRYPT_SCHANNEL_INTERFACE               0x00010002
#if (NTDDI_VERSION >= NTDDI_WIN7)
#define NCRYPT_SCHANNEL_SIGNATURE_INTERFACE     0x00010003
#endif
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define NCRYPT_KEY_PROTECTION_INTERFACE         0x00010004
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

//
// algorithm groups.
//

#define NCRYPT_RSA_ALGORITHM_GROUP              NCRYPT_RSA_ALGORITHM
#define NCRYPT_DH_ALGORITHM_GROUP               NCRYPT_DH_ALGORITHM
#define NCRYPT_DSA_ALGORITHM_GROUP              NCRYPT_DSA_ALGORITHM
#define NCRYPT_ECDSA_ALGORITHM_GROUP            L"ECDSA"
#define NCRYPT_ECDH_ALGORITHM_GROUP             L"ECDH"

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define NCRYPT_AES_ALGORITHM_GROUP              NCRYPT_AES_ALGORITHM
#define NCRYPT_RC2_ALGORITHM_GROUP              NCRYPT_RC2_ALGORITHM
#define NCRYPT_DES_ALGORITHM_GROUP              L"DES"
#define NCRYPT_KEY_DERIVATION_GROUP             L"KEY_DERIVATION"
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

# if (NTDDI_VERSION >= NTDDI_WIN11_GE)
#define NCRYPT_MLKEM_ALGORITHM_GROUP            L"MLKEM"
#define NCRYPT_MLDSA_ALGORITHM_GROUP            L"MLDSA"
#define NCRYPT_SLHDSA_ALGORITHM_GROUP           L"SLHDSA"
#define NCRYPT_LMS_ALGORITHM_GROUP              NCRYPT_LMS_ALGORITHM
#define NCRYPT_XMSS_ALGORITHM_GROUP             NCRYPT_XMSS_ALGORITHM
#endif // (NTDDI_VERSION >= NTDDI_WIN11_GE)

//
// NCrypt generic memory descriptors
//

#define NCRYPTBUFFER_VERSION                                0

#define NCRYPTBUFFER_EMPTY                          0
#define NCRYPTBUFFER_DATA                           1

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define NCRYPTBUFFER_PROTECTION_DESCRIPTOR_STRING   3   // The buffer contains a null-terminated Unicode string that contains the Protection Descriptor.
#define NCRYPTBUFFER_PROTECTION_FLAGS               4   // DWORD flags to be passed to NCryptCreateProtectionDescriptor function.
#endif // (NTDDI_VERSION >= NTDDI_WIN8)


#define NCRYPTBUFFER_SSL_CLIENT_RANDOM                      20
#define NCRYPTBUFFER_SSL_SERVER_RANDOM                      21
#define NCRYPTBUFFER_SSL_HIGHEST_VERSION                    22
#define NCRYPTBUFFER_SSL_CLEAR_KEY                          23
#define NCRYPTBUFFER_SSL_KEY_ARG_DATA                       24
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define NCRYPTBUFFER_SSL_SESSION_HASH                       25
#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#define NCRYPTBUFFER_PKCS_OID                               40
#define NCRYPTBUFFER_PKCS_ALG_OID                           41
#define NCRYPTBUFFER_PKCS_ALG_PARAM                         42
#define NCRYPTBUFFER_PKCS_ALG_ID                            43
#define NCRYPTBUFFER_PKCS_ATTRS                             44
#define NCRYPTBUFFER_PKCS_KEY_NAME                          45
#define NCRYPTBUFFER_PKCS_SECRET                            46

#define NCRYPTBUFFER_CERT_BLOB                              47

//for threshold key attestation
#define NCRYPTBUFFER_CLAIM_IDBINDING_NONCE                  48
#define NCRYPTBUFFER_CLAIM_KEYATTESTATION_NONCE             49
#define NCRYPTBUFFER_KEY_PROPERTY_FLAGS                     50
#define NCRYPTBUFFER_ATTESTATIONSTATEMENT_BLOB              51
#define NCRYPTBUFFER_ATTESTATION_CLAIM_TYPE                 52
#define NCRYPTBUFFER_ATTESTATION_CLAIM_CHALLENGE_REQUIRED   53

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
//for generic ecc
#define NCRYPTBUFFER_ECC_CURVE_NAME                         60
#define NCRYPTBUFFER_ECC_PARAMETERS                         61
#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
//for TPM seal
#define NCRYPTBUFFER_TPM_SEAL_PASSWORD                      70
#define NCRYPTBUFFER_TPM_SEAL_POLICYINFO                    71
#define NCRYPTBUFFER_TPM_SEAL_TICKET                        72
#define NCRYPTBUFFER_TPM_SEAL_NO_DA_PROTECTION              73
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS1)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
// for TPM platform attestation statements
#define NCRYPTBUFFER_TPM_PLATFORM_CLAIM_PCR_MASK            80
#define NCRYPTBUFFER_TPM_PLATFORM_CLAIM_NONCE               81
#define NCRYPTBUFFER_TPM_PLATFORM_CLAIM_STATIC_CREATE       82
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5)

#if (NTDDI_VERSION >= NTDDI_WIN11_SV3)

#define NCRYPTBUFFER_ATTESTATION_STATEMENT_SIGNATURE_HASH              90
#define NCRYPTBUFFER_ATTESTATION_STATEMENT_SIGNATURE_PADDING_SCHEME    91
#define NCRYPTBUFFER_ATTESTATION_STATEMENT_SIGNATURE_PADDING_ALGO      92
#define NCRYPTBUFFER_ATTESTATION_STATEMENT_SIGNATURE_PADDING_SALT_SIZE 93
#define NCRYPTBUFFER_ATTESTATION_STATEMENT_SIGNATURE_PADDING_SALT      NCRYPTBUFFER_ATTESTATION_STATEMENT_SIGNATURE_PADDING_SALT_SIZE // Added to avoid failure for existing users
#define NCRYPTBUFFER_ATTESTATION_STATEMENT_NONCE                       NCRYPTBUFFER_CLAIM_KEYATTESTATION_NONCE

#define NCRYPTBUFFER_VBS_ATTESTATION_STATEMENT_ROOT_DETAILS          94
#define NCRYPTBUFFER_VBS_ATTESTATION_STATEMENT_IDENTITY_DETAILS      95

#endif

#if (NTDDI_VERSION >= NTDDI_WIN11_GE)

#define NCRYPTBUFFER_PKCS_AES_KEY_BITS                              96
#define NCRYPTBUFFER_PKCS_PADDING_ALGO                              97
#define NCRYPTBUFFER_PKCS_PADDING_LABEL                             98

#endif // (NTDDI_VERSION >= NTDDI_WIN11_GE)


// NCRYPT shares the same BCRYPT definitions
typedef BCryptBuffer     NCryptBuffer;
typedef BCryptBuffer*    PNCryptBuffer;
typedef BCryptBufferDesc NCryptBufferDesc;
typedef BCryptBufferDesc* PNCryptBufferDesc;

//
// NCrypt handles
//

typedef ULONG_PTR NCRYPT_HANDLE;
typedef ULONG_PTR NCRYPT_PROV_HANDLE;
typedef ULONG_PTR NCRYPT_KEY_HANDLE;
typedef ULONG_PTR NCRYPT_HASH_HANDLE;
typedef ULONG_PTR NCRYPT_SECRET_HANDLE;

#if (NTDDI_VERSION >= NTDDI_WIN8)

typedef _Struct_size_bytes_(cbSize + cbIV + cbOtherInfo)
struct _NCRYPT_CIPHER_PADDING_INFO
{
    // size of this struct
    ULONG       cbSize;

    // See NCRYPT_CIPHER_ flag values
    DWORD       dwFlags;

    // [in, out, optional]
    // The address of a buffer that contains the initialization vector (IV) to use during encryption.
    // The cbIV parameter contains the size of this buffer. This function will modify the contents of this buffer.
    // If you need to reuse the IV later, make sure you make a copy of this buffer before calling this function.
    _Field_size_bytes_(cbIV)
    PUCHAR      pbIV;
    ULONG       cbIV;

    // [in, out, optional]
    // The address of a buffer that contains the algorithm specific info to use during encryption.
    // The cbOtherInfo parameter contains the size of this buffer. This function will modify the contents of this buffer.
    // If you need to reuse the buffer later, make sure you make a copy of this buffer before calling this function.
    //
    // For Microsoft providers, when an authenticated encryption mode is used,
    // this parameter must point to a serialized BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO structure.
    //
    // NOTE: All pointers inside a structure must be to a data allocated within pbOtherInfo buffer.
    //
    _Field_size_bytes_(cbOtherInfo)
    PUCHAR      pbOtherInfo;
    ULONG       cbOtherInfo;

} NCRYPT_CIPHER_PADDING_INFO, *PNCRYPT_CIPHER_PADDING_INFO;

//
// The following flags are used with NCRYPT_CIPHER_PADDING_INFO
//
#define NCRYPT_CIPHER_NO_PADDING_FLAG           0x00000000
#define NCRYPT_CIPHER_BLOCK_PADDING_FLAG        0x00000001
#define NCRYPT_CIPHER_OTHER_PADDING_FLAG        0x00000002

#endif  // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

#define NCRYPT_PLATFORM_ATTEST_MAGIC 0x44504150  // 'PAPD'

typedef struct _NCRYPT_PLATFORM_ATTEST_PADDING_INFO {
    ULONG  magic;  // 'PAPD'
    ULONG  pcrMask;
} NCRYPT_PLATFORM_ATTEST_PADDING_INFO;

#define NCRYPT_KEY_ATTEST_MAGIC 0x4450414b  // 'KAPD'

typedef struct _NCRYPT_KEY_ATTEST_PADDING_INFO {
    ULONG   magic;  // 'KAPD'
    PUCHAR  pbKeyBlob;
    ULONG   cbKeyBlob;
    PUCHAR  pbKeyAuth;
    ULONG   cbKeyAuth;
} NCRYPT_KEY_ATTEST_PADDING_INFO;

#endif  // (NTDDI_VERSION >= NTDDI_WINBLUE)

//
// key attestation claim type
//
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#define NCRYPT_CLAIM_AUTHORITY_ONLY                         0x00000001
#define NCRYPT_CLAIM_SUBJECT_ONLY                           0x00000002
#define NCRYPT_CLAIM_AUTHORITY_AND_SUBJECT                  0x00000003
#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
//
// The NCRYPT_CLAIM_VBS_KEY_ATTESTATION_STATEMENT claim type is automatically transformed to NCRYPT_CLAIM_VBS_ROOT in
// NCryptCreateClaim and NCryptVerifyClaim.
#define NCRYPT_CLAIM_VBS_KEY_ATTESTATION_STATEMENT          0x00000004
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)
#if (NTDDI_VERSION >= NTDDI_WIN11_SV3)
#define NCRYPT_CLAIM_VBS_ROOT                               0x00000005
#define NCRYPT_CLAIM_VBS_IDENTITY                           0x00000006
#endif // (NTDDI_VERSION >= NTDDI_WIN11_SV3)
#define NCRYPT_CLAIM_WEB_AUTH_SUBJECT_ONLY                  0x00000102
#define NCRYPT_CLAIM_UNKNOWN                                0x00001000
#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
#define NCRYPT_CLAIM_PLATFORM                               0x00010000
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5)

#if (NTDDI_VERSION >= NTDDI_WIN11_SV3)
#define NCRYPT_CLAIM_WEB_AUTH_SUBJECT_ONLY_V2               0x00000103
#endif // (NTDDI_VERSION >= NTDDI_WIN11_SV3)

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)

// NCryptCreateClaim claim types, flags and buffer types

#define NCRYPT_ISOLATED_KEY_FLAG_CREATED_IN_ISOLATION 0x00000001 // if set, this key was generated in isolation, not imported
#define NCRYPT_ISOLATED_KEY_FLAG_IMPORT_ONLY          0x00000002 // if set, this key can only be used for importing other keys

#if (NTDDI_VERSION >= NTDDI_WIN11_SV3)
#define NCRYPT_ISOLATED_KEY_FLAG_PER_BOOT_KEY         0x00000004 // Indicates renewed keys per boot
#define NCRYPT_VBS_KEY_FLAG_CREATED_IN_ISOLATION      NCRYPT_ISOLATED_KEY_FLAG_CREATED_IN_ISOLATION
#define NCRYPT_VBS_KEY_FLAG_IMPORT_ONLY               NCRYPT_ISOLATED_KEY_FLAG_IMPORT_ONLY
#define NCRYPT_VBS_KEY_FLAG_PER_BOOT_KEY              NCRYPT_ISOLATED_KEY_FLAG_PER_BOOT_KEY
#endif // (NTDDI_VERSION >= NTDDI_WIN11_SV3)

#if (NTDDI_VERSION >= NTDDI_WIN11_SV4)
// Indicates that the key may be used to import ephemeral keys. This is useful in pkcs11 import
#define NCRYPT_VBS_KEY_FLAG_IMPORT_EPHEMERAL_ONLY     0x00000008
#endif // (NTDDI_VERSION >= NTDDI_WIN11_SV4)
#define NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES_V0 0
#define NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES_CURRENT_VERSION NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES_V0

typedef struct _NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES
{
   ULONG Version; // set to NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES_V0
   ULONG Flags;   // NCRYPT_VBS_KEY_FLAG_... flags
   ULONG cbPublicKeyBlob;
   // pbPublicKeyBlob[cbPublicKeyBlob] - exported public key
} NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES, *PNCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES;

#if (NTDDI_VERSION >= NTDDI_WIN11_SV3)

#define NCRYPT_VBS_KEY_ATTESTED_ATTRIBUTES                 NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES
#define PNCRYPT_VBS_KEY_ATTESTED_ATTRIBUTES                PNCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES
#define NCRYPT_VBS_KEY_ATTESTED_ATTRIBUTES_CURRENT_VERSION NCRYPT_ISOLATED_KEY_ATTESTED_ATTRIBUTES_CURRENT_VERSION

#endif // (NTDDI_VERSION >= NTDDI_WIN11_SV3)

#pragma warning(default:4214) // bit fields type other than int

// Structures to assist with importation of isolated keys

#define NCRYPT_EXPORTED_ISOLATED_KEY_HEADER_V0 0
#define NCRYPT_EXPORTED_ISOLATED_KEY_HEADER_CURRENT_VERSION NCRYPT_EXPORTED_ISOLATED_KEY_HEADER_V0

#pragma warning(disable:4214) // bit fields type other than int
typedef struct _NCRYPT_EXPORTED_ISOLATED_KEY_HEADER
{
    ULONG Version;         // Set to NCRYPT_EXPORTED_ISOLATED_KEY_HEADER_V0
    ULONG KeyUsage;        // Set to NCRYPT_ALLOW_KEY_IMPORT_FLAG for import-only keys
    ULONG PerBootKey : 1;  // Set to TRUE if the key is to be valid in the current boot cycle only
    ULONG Reserved : 31;   // Leave as 0
    ULONG cbAlgName;       // Number of bytes in Unicode algorithm name following header + terminating NULL
    ULONG cbNonce;         // Number of bytes in the nonce used to encrypt the isolated key
    ULONG cbAuthTag;       // Number of bytes in authentication tag resulting from encrypting the isolated key
    ULONG cbWrappingKey;   // Number of bytes in encrypted wrapping key blob
    ULONG cbIsolatedKey;   // Number of bytes in encrypted isolated key blob
} NCRYPT_EXPORTED_ISOLATED_KEY_HEADER, *PNCRYPT_EXPORTED_ISOLATED_KEY_HEADER;
#pragma warning(default:4214) // bit fields type other than int

typedef struct _NCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE
{
    NCRYPT_EXPORTED_ISOLATED_KEY_HEADER Header;
    // UCHAR AlgorithmName[Header.cbAlgName]       -- Unicode algorithm name including terminating NULL
    // UCHAR Nonce[Header.cbNonce]                 -- Nonce buffer used when encrypting isolated key
    // ---- data after this point is not integrity protected in transit
    // UCHAR AesGcmAuthTag[Header.cbAuthTag]
    // UCHAR WrappingKeyBlob[Header.cbWrappingKey] -- RSA-OAEP encrypted AES wrapping key
    // UCHAR IsolatedKeyBlob[Header.cbIsolatedKey] -- AES-GCM encrypted key to import
} NCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE, *PNCRYPT_EXPORTED_ISOLATED_KEY_ENVELOPE;

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3) // && (NTDDI_VERSION < NTDDI_WIN11_SV3) // Keep on backward compitability for VSM_KEY_ATTESTATION

#endif // #if (NTDDI_VERSION >= NTDDI_WIN10_RS3)

#if (NTDDI_VERSION >= NTDDI_WIN11_SV3) // NTDDI_WIN11_SV3 is not final

// Header used for claims of type NCRYPT_CLAIM_VBS_ROOT
#define NCRYPT_VBS_ROOT_ATTESTATION_HEADER_V0 0
#define NCRYPT_VBS_ROOT_ATTESTATION_HEADER_CURRENT_VERSION NCRYPT_VBS_ROOT_ATTESTATION_HEADER_V0
#define VBS_ROOT_ATTESTATION_HEADER_MAGIC 0x48435256 // 'VRCH' = 0x56, 0x52, 0x43, 0x48.

typedef struct _NCRYPT_VBS_ROOT_ATTESTATION_HEADER
{
    ULONG Magic;        // {'H', 'C', 'R', 'V'} - 'VRCH' for VBS Root Claim Header
    ULONG Version;      // Set to NCRYPT_VBS_ROOT_ATTESTATION_HEADER_CURRENT_VERSION
    ULONG cbAttributes; // Number of bytes in attributes of the isolated key (including public key blob )
    ULONG cbNonce;      // Number of bytes in the provided nonce, can be 0 if nonce doesn't exist
    ULONG cbReport;     // Number of bytes in key isolation report from the secure kernel
    ULONG cbSignature;  // Number of bytes in the secure kernel signature over the isolation report
    // UCHAR Attributes[cbAttributes]  -- Trustlet-reported attributes of the key
    // UCHAR Nonce[cbNonce]            -- Nonce value to be used when hashing Attributes
    // UCHAR Report[cbReport]          -- Secure kernel report including hash of Attributes, dwFlags and nonce (if available)
    // UCHAR Signature[cbSignature]    -- Secure kernel signature of the report
} NCRYPT_VBS_ROOT_ATTESTATION_HEADER, * PNCRYPT_VBS_ROOT_ATTESTATION_HEADER;

// Padding structure for identity claims
#define NCRYPT_VBS_IDENTITY_ATTESTATION_PADDING_V0 0
#define NCRYPT_VBS_IDENTITY_ATTESTATION_PADDING_CURRENT_VERSION NCRYPT_VBS_IDENTITY_ATTESTATION_PADDING_V0

typedef struct _NCRYPT_VBS_IDENTITY_ATTESTATION_PADDING
{
    ULONG Version;         // Set to NCRYPT_VBS_IDENTITY_ATTESTATION_PADDING_CURRENT_VERSION
    ULONG ulPaddingScheme; // Padding scheme of the signing algorithm that was used through claim creation.
    ULONG cbHashAlg;       // Number of bytes in Unicode name that identifies the cryptographic padding algorithm. This algorithm must be a hashing algorithm.
    ULONG ulSalt;          // Number of bytes in of the random salt to use for the padding.
    // UCHAR HashAlg[cbHashAlg]  -- Unicode name of hash algorithm used through attributes signing to create padding
} NCRYPT_VBS_IDENTITY_ATTESTATION_PADDING, * PNCRYPT_VBS_IDENTITY_ATTESTATION_PADDING;

//  Header used for claims of type NCRYPT_CLAIM_VBS_IDENTITY
#define NCRYPT_VBS_IDENTITY_ATTESTATION_HEADER_V0 0
#define NCRYPT_VBS_IDENTITY_ATTESTATION_HEADER_CURRENT_VERSION NCRYPT_VBS_IDENTITY_ATTESTATION_HEADER_V0
#define VBS_IDENTITY_ATTESTATION_HEADER_MAGIC 0x48434956 // 'VICH' = 0x56, 0x49, 0x43, 0x48.

typedef struct _NCRYPT_VBS_IDENTITY_ATTESTATION_HEADER
{
    ULONG Magic;           // {'H', 'C', 'I', 'V'} - 'VICH' for VBS Identity Claim Header
    ULONG Version;         // Set to NCRYPT_VBS_IDENTITY_ATTESTATION_HEADER_CURRENT_VERSION
    ULONG cbAttributes;    // Number of bytes in attributes of the isolated key (including public key blob )
    ULONG cbNonce;         // Number of bytes in the provided nonce, can be 0 if nonce doesn't exist
    ULONG cbHashAlg;       // Number of bytes in Unicode name of hash algorithm used
    ULONG cbPadding;       // Padding info (scheme and more) of the signing algorithm that was used through claim creation
    ULONG cbSignatureAlg;  // Number of bytes in Unicode name of the signing hash algorithm used
    ULONG cbSignature;     // Number of bytes in the signature over the hash of Attributes, dwFlags and nonce (if available)
    // UCHAR Attributes[cbAttributes]     -- Trustlet-reported attributes of the subject key blob
    // UCHAR Nonce[cbNonce]               -- Nonce value to be used when hashing Attributes
    // UCHAR HashAlg[cbHashAlg]           -- Unicode name of hash algorithm used through attributes signing
    // UCHAR Padding[cbPadding]           -- Padding information that is set through identity hash signing
    // UCHAR SignatureAlg[cbSignatureAlg] -- Unicode name of signing algorithm used
    // UCHAR Signature[cbSignature]       -- Signature over the hash of Attributes and nonce (if available)
} NCRYPT_VBS_IDENTITY_ATTESTATION_HEADER, * PNCRYPT_VBS_IDENTITY_ATTESTATION_HEADER;


#define NCRYPT_VBS_KEY_ATTESTATION_STATEMENT_V1 1
#define NCRYPT_VBS_KEY_ATTESTATION_STATEMENT_V2 2
#define NCRYPT_VBS_KEY_ATTESTATION_STATEMENT_CURRENT_VERSION NCRYPT_VBS_KEY_ATTESTATION_STATEMENT_V2
#define VBS_KEY_ATTESTATION_STATEMENT_MAGIC 0x53414B56 // 'VKAS' = 0x56, 0x4B, 0x41, 0x53.

typedef struct _NCRYPT_VBS_KEY_ATTESTATION_STATEMENT
{
    ULONG Magic;        // {'S', 'A', 'K', 'V'} - 'VKAS' for VBS Key Attestation Statement
    ULONG Version;      // Set to NCRYPT_VBS_KEY_ATTESTATION_STATEMENT_CURRENT_VERSION
    ULONG ClaimType;    // The claim type in the memory that follows this structure
    // Root binary data - NCRYPT_VBS_ROOT_ATTESTATION_HEADER
    // UCHAR Attributes[cbAttributes]  -- Trustlet-reported attributes of the key
    // UCHAR Nonce[cbNonce]            -- Nonce value to be used when hashing Attributes
    // UCHAR Report[cbReport]          -- Secure kernel report including hash of Attributes, dwFlags and nonce (if available)
    // UCHAR Signature[cbSignature]    -- Secure kernel signature of the report
    // --------------------------------------------------------------------------------------
    // Identity binary data - NCRYPT_VBS_IDENTITY_ATTESTATION_HEADER
    // UCHAR Attributes[cbAttributes]      -- Trustlet-reported attributes of the subject key blob
    // UCHAR Nonce[cbNonce]                -- Nonce value to be used when hashing Attributes
    // UCHAR HashAlg[cbHashAlg]            -- Unicode name of hash algorithm used through attributes signing
    // UCHAR Padding[cbPadding]            -- Padding information that is set through identity hash signing
    // UCHAR SignatureAlg[cbSignatureAlg]  -- Unicode name of signing algorithm used
    // UCHAR Signature[cbSignature]        -- Signature over the hash of Attributes and nonce (if available)
} NCRYPT_VBS_KEY_ATTESTATION_STATEMENT, * PNCRYPT_VBS_KEY_ATTESTATION_STATEMENT;

typedef struct _NCRYPT_VBS_ROOT_KEY_ATTESTATION_CLAIM_DETAILS
{
    ULONG     ulKeyFlags;                       // NCRYPT_ISOLATED_KEY_ flags
    ULONGLONG ullTrustletId;                    // Trustlet ID
    ULONG     ulTrustletSecurityVersion;        // Trustlet Security Version Number
    ULONG     ulTrustletDebuggable;             // Indicates that the Trustlet can be debugged
} NCRYPT_VBS_ROOT_KEY_ATTESTATION_CLAIM_DETAILS, * PNCRYPT_VBS_ROOT_KEY_ATTESTATION_CLAIM_DETAILS;


typedef struct _NCRYPT_VBS_IDENTITY_KEY_ATTESTATION_CLAIM_DETAILS
{
    ULONG    ulKeyFlags;          // NCRYPT_ISOLATED_KEY_ flags
    LPCWSTR  pszSignatureHashAlg; // A pointer to a Unicode string of the claim signature hash algorithm
    ULONG    ulPaddingScheme;    // Padding scheme of the signing algorithm that was used through claim creation
    LPCWSTR  pszPaddingHashAlg;   // A pointer to a Unicode string of the claim padding hash algorithm
    ULONG    ulPaddingSalt;       // Padding salt of the signing algorithm that was used through claim creation
} NCRYPT_VBS_IDENTITY_KEY_ATTESTATION_CLAIM_DETAILS, * PNCRYPT_VBS_IDENTITY_KEY_ATTESTATION_CLAIM_DETAILS;

#endif  // (NTDDI_VERSION >= NTDDI_WIN11_SV3)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

typedef struct __NCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT
{
    UINT32 Magic;  // { 'A', 'W', 'A', 'K' } - 'KAWA'
    UINT32 Version;  // 1 for the statement defined in this specification
    UINT32 HeaderSize;  // 24
    UINT32 cbCertifyInfo;
    UINT32 cbSignature;
    UINT32 cbTpmPublic;
    // CertifyInfo[cbCertifyInfo];
    // Signature[cbSignature];
    // TpmPublic[cbTpmPublic];

} NCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT,*PNCRYPT_PCP_TPM_WEB_AUTHN_ATTESTATION_STATEMENT;

#endif// (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)

#define NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT_V0 0
#define NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT_CURRENT_VERSION NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT_V0

typedef struct _NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT
{
    ULONG Magic;        // {'A', 'L', 'P', 'T'} - 'TPLA' for TPM Platform
    ULONG Version;      // Set to NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT_CURRENT_VERSION
    ULONG pcrAlg;       // The TPM hash algorithm ID
    ULONG cbSignature;  // TPMT_SIGNATURE structure signature over the quote
    ULONG cbQuote;      // TPMS_ATTEST structure that was generated and signed
    ULONG cbPcrs;       // Raw concatenation of all 24 PCRs
    // UCHAR Signature[cbSignature]
    // UCHAR Quote[cbQuote]
    // UCHAR Pcrs[cbPcrs]
} NCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT, *PNCRYPT_TPM_PLATFORM_ATTESTATION_STATEMENT;

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5)

//
// NCrypt API Flags
//

#define NCRYPT_NO_PADDING_FLAG                  0x00000001  // NCryptEncrypt/Decrypt
#define NCRYPT_PAD_PKCS1_FLAG                   0x00000002  // NCryptEncrypt/Decrypt NCryptSignHash/VerifySignature
#define NCRYPT_PAD_OAEP_FLAG                    0x00000004  // BCryptEncrypt/Decrypt
#define NCRYPT_PAD_PSS_FLAG                     0x00000008  // BCryptSignHash/VerifySignature
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define NCRYPT_PAD_CIPHER_FLAG                  0x00000010  // NCryptEncrypt/Decrypt
#endif // (NTDDI_VERSION >= NTDDI_WIN8)
#if (NTDDI_VERSION >= NTDDI_WIN11_GE)
#define NCRYPT_PAD_PQDSA_FLAG	                BCRYPT_PAD_PQDSA  // BCryptSignHash/VerifySignature NCryptSignHash/VerifySignature
#endif

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define NCRYPT_ATTESTATION_FLAG                 0x00000020 // NCryptDecrypt for key attestation
#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
#define NCRYPT_SEALING_FLAG                     0x00000100 // NCryptEncrypt/Decrypt for sealing
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS1)

#define NCRYPT_REGISTER_NOTIFY_FLAG             0x00000001  // NCryptNotifyChangeKey
#define NCRYPT_UNREGISTER_NOTIFY_FLAG           0x00000002  // NCryptNotifyChangeKey
#define NCRYPT_NO_KEY_VALIDATION                BCRYPT_NO_KEY_VALIDATION
#define NCRYPT_MACHINE_KEY_FLAG                 0x00000020  // same as CAPI CRYPT_MACHINE_KEYSET
#define NCRYPT_SILENT_FLAG                      0x00000040  // same as CAPI CRYPT_SILENT
#define NCRYPT_OVERWRITE_KEY_FLAG               0x00000080
#define NCRYPT_WRITE_KEY_TO_LEGACY_STORE_FLAG   0x00000200
#define NCRYPT_DO_NOT_FINALIZE_FLAG             0x00000400
#define NCRYPT_EXPORT_LEGACY_FLAG               0x00000800
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
#define NCRYPT_IGNORE_DEVICE_STATE_FLAG         0x00001000  // NCryptOpenStorageProvider
#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define NCRYPT_TREAT_NIST_AS_GENERIC_ECC_FLAG   0x00002000
#define NCRYPT_NO_CACHED_PASSWORD               0x00004000
#define NCRYPT_PROTECT_TO_LOCAL_SYSTEM          0x00008000
#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define NCRYPT_REQUIRE_KDS_LRPC_BIND_FLAG       0x20000000
#define NCRYPT_PERSIST_ONLY_FLAG                0x40000000
#define NCRYPT_PERSIST_FLAG                     0x80000000


#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
#define NCRYPT_PREFER_VIRTUAL_ISOLATION_FLAG    0x00010000 // NCryptCreatePersistedKey NCryptImportKey
#define NCRYPT_USE_VIRTUAL_ISOLATION_FLAG       0x00020000 // NCryptCreatePersistedKey NCryptImportKey
#define NCRYPT_USE_PER_BOOT_KEY_FLAG            0x00040000 // NCryptCreatePersistedKey NCryptImportKey
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#if (NTDDI_VERSION >= NTDDI_WIN11_SV3)
#define NCRYPT_PREFER_VBS_FLAG                  NCRYPT_PREFER_VIRTUAL_ISOLATION_FLAG // NCryptCreatePersistedKey NCryptImportKey
#define NCRYPT_REQUIRE_VBS_FLAG                 NCRYPT_USE_VIRTUAL_ISOLATION_FLAG    // NCryptCreatePersistedKey NCryptImportKey
#define NCRYPT_USE_VBS_PER_BOOT_KEY_FLAG        NCRYPT_USE_PER_BOOT_KEY_FLAG         // NCryptCreatePersistedKey NCryptImportKey
#define NCRYPT_VBS_RETURN_CLAIM_DETAILS_FLAG    0x00100000 // NCryptVerifyClaim
#endif

//
// Functions used to manage persisted keys.
//

// NCryptOpenStorageProvider flags
#define NCRYPT_SILENT_FLAG                      0x00000040  // same as CAPI CRYPT_SILENT
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
#define NCRYPT_IGNORE_DEVICE_STATE_FLAG         0x00001000  // NCryptOpenStorageProvider
#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)

_Check_return_
SECURITY_STATUS
WINAPI
NCryptOpenStorageProvider(
    _Out_   NCRYPT_PROV_HANDLE *phProvider,
    _In_opt_ LPCWSTR pszProviderName,
    _In_    DWORD   dwFlags);



// AlgOperations flags for use with NCryptEnumAlgorithms()
#define NCRYPT_CIPHER_OPERATION                 BCRYPT_CIPHER_OPERATION
#define NCRYPT_HASH_OPERATION                   BCRYPT_HASH_OPERATION
#define NCRYPT_ASYMMETRIC_ENCRYPTION_OPERATION  BCRYPT_ASYMMETRIC_ENCRYPTION_OPERATION
#define NCRYPT_SECRET_AGREEMENT_OPERATION       BCRYPT_SECRET_AGREEMENT_OPERATION
#define NCRYPT_SIGNATURE_OPERATION              BCRYPT_SIGNATURE_OPERATION
#define NCRYPT_RNG_OPERATION                    BCRYPT_RNG_OPERATION
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define NCRYPT_KEY_DERIVATION_OPERATION         BCRYPT_KEY_DERIVATION_OPERATION
#endif  // (NTDDI_VERSION >= NTDDI_WIN8)
#if (NTDDI_VERSION >= NTDDI_WIN11_GE)
#define NCRYPT_KEY_ENCAPSULATION_OPERATION      BCRYPT_KEY_ENCAPSULATION_OPERATION
#endif
// USE EXTREME CAUTION: editing comments that contain "certenrolls_*" tokens
// could break building CertEnroll idl files:
// certenrolls_begin -- NCryptAlgorithmName
typedef struct _NCryptAlgorithmName
{
    LPWSTR  pszName;
    DWORD   dwClass;            // the CNG interface that supports this algorithm
    DWORD   dwAlgOperations;    // the types of operations supported by this algorithm
    DWORD   dwFlags;
} NCryptAlgorithmName;
// certenrolls_end

_Check_return_
SECURITY_STATUS
WINAPI
NCryptEnumAlgorithms(
    _In_    NCRYPT_PROV_HANDLE hProvider,
    _In_    DWORD   dwAlgOperations,
    _Out_   DWORD * pdwAlgCount,
    _Outptr_result_buffer_(*pdwAlgCount) NCryptAlgorithmName **ppAlgList,
    _In_    DWORD   dwFlags);



_Check_return_
SECURITY_STATUS
WINAPI
NCryptIsAlgSupported(
    _In_    NCRYPT_PROV_HANDLE hProvider,
    _In_    LPCWSTR pszAlgId,
    _In_    DWORD   dwFlags);



// NCryptEnumKeys flags
#define NCRYPT_MACHINE_KEY_FLAG         0x00000020

typedef struct NCryptKeyName
{
    LPWSTR  pszName;
    LPWSTR  pszAlgid;
    DWORD   dwLegacyKeySpec;
    DWORD   dwFlags;
} NCryptKeyName;

_Check_return_
SECURITY_STATUS
WINAPI
NCryptEnumKeys(
    _In_    NCRYPT_PROV_HANDLE hProvider,
    _In_opt_ LPCWSTR pszScope,
    _Outptr_ NCryptKeyName **ppKeyName,
    _Inout_ PVOID * ppEnumState,
    _In_    DWORD   dwFlags);



typedef struct NCryptProviderName
{
    LPWSTR  pszName;
    LPWSTR  pszComment;
} NCryptProviderName;

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Check_return_
SECURITY_STATUS
WINAPI
NCryptEnumStorageProviders(
    _Out_   DWORD * pdwProviderCount,
    _Outptr_result_buffer_(*pdwProviderCount) NCryptProviderName **ppProviderList,
    _In_    DWORD   dwFlags);



#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

SECURITY_STATUS
WINAPI
NCryptFreeBuffer(
    _Pre_notnull_ PVOID   pvInput);



// NCryptOpenKey flags
#define NCRYPT_MACHINE_KEY_FLAG         0x00000020
#define NCRYPT_SILENT_FLAG              0x00000040
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define NCRYPT_AUTHORITY_KEY_FLAG       0x00000100
#endif
#if (NTDDI_VERSION >= NTDDI_WIN10_NI)
#define NCRYPT_EXTENDED_ERRORS_FLAG     0x10000000
#endif

_Check_return_
SECURITY_STATUS
WINAPI
NCryptOpenKey(
    _In_    NCRYPT_PROV_HANDLE hProvider,
    _Out_   NCRYPT_KEY_HANDLE *phKey,
    _In_    LPCWSTR pszKeyName,
    _In_opt_ DWORD  dwLegacyKeySpec,
    _In_    DWORD   dwFlags);



// NCryptCreatePersistedKey flags
#define NCRYPT_MACHINE_KEY_FLAG         0x00000020
#define NCRYPT_OVERWRITE_KEY_FLAG       0x00000080

_Check_return_
SECURITY_STATUS
WINAPI
NCryptCreatePersistedKey(
    _In_    NCRYPT_PROV_HANDLE hProvider,
    _Out_   NCRYPT_KEY_HANDLE *phKey,
    _In_    LPCWSTR pszAlgId,
    _In_opt_ LPCWSTR pszKeyName,
    _In_    DWORD   dwLegacyKeySpec,
    _In_    DWORD   dwFlags);



// Standard property names.
#define NCRYPT_NAME_PROPERTY                    L"Name"
#define NCRYPT_EPHEMERAL_NAME_PROPERTY          L"Ephemeral Name"
#define NCRYPT_UNIQUE_NAME_PROPERTY             L"Unique Name"
#define NCRYPT_ALGORITHM_PROPERTY               L"Algorithm Name"
#define NCRYPT_LENGTH_PROPERTY                  L"Length"
#define NCRYPT_LENGTHS_PROPERTY                 L"Lengths"
#define NCRYPT_BLOCK_LENGTH_PROPERTY            L"Block Length"

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define NCRYPT_PUBLIC_LENGTH_PROPERTY           BCRYPT_PUBLIC_KEY_LENGTH
#define NCRYPT_SIGNATURE_LENGTH_PROPERTY        BCRYPT_SIGNATURE_LENGTH
#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define NCRYPT_CHAINING_MODE_PROPERTY           L"Chaining Mode"
#define NCRYPT_AUTH_TAG_LENGTH                  L"AuthTagLength"
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#define NCRYPT_UI_POLICY_PROPERTY               L"UI Policy"
#define NCRYPT_EXPORT_POLICY_PROPERTY           L"Export Policy"
#define NCRYPT_WINDOW_HANDLE_PROPERTY           L"HWND Handle"
#define NCRYPT_USE_CONTEXT_PROPERTY             L"Use Context"
#define NCRYPT_IMPL_TYPE_PROPERTY               L"Impl Type"
#define NCRYPT_KEY_USAGE_PROPERTY               L"Key Usage"
#define NCRYPT_KEY_TYPE_PROPERTY                L"Key Type"
#define NCRYPT_VERSION_PROPERTY                 L"Version"
#define NCRYPT_SECURITY_DESCR_SUPPORT_PROPERTY  L"Security Descr Support"
#define NCRYPT_SECURITY_DESCR_PROPERTY          L"Security Descr"
#define NCRYPT_USE_COUNT_ENABLED_PROPERTY       L"Enabled Use Count"
#define NCRYPT_USE_COUNT_PROPERTY               L"Use Count"
#define NCRYPT_LAST_MODIFIED_PROPERTY           L"Modified"
#define NCRYPT_MAX_NAME_LENGTH_PROPERTY         L"Max Name Length"
#define NCRYPT_ALGORITHM_GROUP_PROPERTY         L"Algorithm Group"
#define NCRYPT_DH_PARAMETERS_PROPERTY           BCRYPT_DH_PARAMETERS

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define NCRYPT_ECC_PARAMETERS_PROPERTY          BCRYPT_ECC_PARAMETERS
#define NCRYPT_ECC_CURVE_NAME_PROPERTY          BCRYPT_ECC_CURVE_NAME
#define NCRYPT_ECC_CURVE_NAME_LIST_PROPERTY     BCRYPT_ECC_CURVE_NAME_LIST
#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
#define NCRYPT_USE_VIRTUAL_ISOLATION_PROPERTY   L"Virtual Iso"
#define NCRYPT_USE_PER_BOOT_KEY_PROPERTY        L"Per Boot Key"
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#define NCRYPT_PROVIDER_HANDLE_PROPERTY         L"Provider Handle"
#define NCRYPT_PIN_PROPERTY                     L"SmartCardPin"
#define NCRYPT_READER_PROPERTY                  L"SmartCardReader"
#define NCRYPT_SMARTCARD_GUID_PROPERTY          L"SmartCardGuid"
#define NCRYPT_CERTIFICATE_PROPERTY             L"SmartCardKeyCertificate"
#define NCRYPT_PIN_PROMPT_PROPERTY              L"SmartCardPinPrompt"
#define NCRYPT_USER_CERTSTORE_PROPERTY          L"SmartCardUserCertStore"
#define NCRYPT_ROOT_CERTSTORE_PROPERTY          L"SmartcardRootCertStore"
#define NCRYPT_SECURE_PIN_PROPERTY              L"SmartCardSecurePin"
#if (NTDDI_VERSION >= NTDDI_WIN7)
#define NCRYPT_ASSOCIATED_ECDH_KEY              L"SmartCardAssociatedECDHKey"
#define NCRYPT_SCARD_PIN_ID                     L"SmartCardPinId"
#define NCRYPT_SCARD_PIN_INFO                   L"SmartCardPinInfo"
#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define NCRYPT_READER_ICON_PROPERTY             L"SmartCardReaderIcon"
#define NCRYPT_KDF_SECRET_VALUE                 L"KDFKeySecret"
#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
#define NCRYPT_DISMISS_UI_TIMEOUT_SEC_PROPERTY  L"SmartCardDismissUITimeoutSeconds"
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5)
#if (NTDDI_VERSION >= NTDDI_WIN11_SV3) // NTDDI_WIN11_SV3 is not final
#define NCRYPT_VBS_ROOT_PUB_PROPERTY            L"VBS_ROOT_PUB"
#define NCRYPT_CERTIFICATE_FROM_NVRAM_PROPERTY  L"KeyCertificateFromTpmNvram"
#endif // (NTDDI_VERSION >= NTDDI_WIN11_SV3)

#if (NTDDI_VERSION >= NTDDI_WIN11_GE)

#define NCRYPT_PARAMETER_SET_NAME_PROPERTY        BCRYPT_PARAMETER_SET_NAME // For PQC keys

#define NCRYPT_KEM_SHARED_SECRET_LENGTH_PROPERTY BCRYPT_KEM_SHARED_SECRET_LENGTH
#define NCRYPT_KEM_CIPHERTEXT_LENGTH_PROPERTY    BCRYPT_KEM_CIPHERTEXT_LENGTH

#endif // (NTDDI_VERSION >= NTDDI_WIN11_GE)

//
// Additional property strings specific for the Platform Crypto Provider
//
#define NCRYPT_PCP_PLATFORM_TYPE_PROPERTY                  L"PCP_PLATFORM_TYPE"
#define NCRYPT_PCP_PROVIDER_VERSION_PROPERTY               L"PCP_PROVIDER_VERSION"
#define NCRYPT_PCP_EKPUB_PROPERTY                          L"PCP_EKPUB"
#define NCRYPT_PCP_EKCERT_PROPERTY                         L"PCP_EKCERT"
#define NCRYPT_PCP_EKNVCERT_PROPERTY                       L"PCP_EKNVCERT"
#define NCRYPT_PCP_RSA_EKPUB_PROPERTY                      L"PCP_RSA_EKPUB"
#define NCRYPT_PCP_RSA_EKCERT_PROPERTY                     L"PCP_RSA_EKCERT"
#define NCRYPT_PCP_RSA_EKNVCERT_PROPERTY                   L"PCP_RSA_EKNVCERT"
#define NCRYPT_PCP_ECC_EKPUB_PROPERTY                      L"PCP_ECC_EKPUB"
#define NCRYPT_PCP_ECC_EKCERT_PROPERTY                     L"PCP_ECC_EKCERT"
#define NCRYPT_PCP_ECC_EKNVCERT_PROPERTY                   L"PCP_ECC_EKNVCERT"
#define NCRYPT_PCP_SRKPUB_PROPERTY                         L"PCP_SRKPUB"
#define NCRYPT_PCP_PCRTABLE_PROPERTY                       L"PCP_PCRTABLE"
#define NCRYPT_PCP_CHANGEPASSWORD_PROPERTY                 L"PCP_CHANGEPASSWORD"
#define NCRYPT_PCP_PASSWORD_REQUIRED_PROPERTY              L"PCP_PASSWORD_REQUIRED"
#define NCRYPT_PCP_USAGEAUTH_PROPERTY                      L"PCP_USAGEAUTH"
#define NCRYPT_PCP_MIGRATIONPASSWORD_PROPERTY              L"PCP_MIGRATIONPASSWORD"
#define NCRYPT_PCP_EXPORT_ALLOWED_PROPERTY                 L"PCP_EXPORT_ALLOWED"
#define NCRYPT_PCP_STORAGEPARENT_PROPERTY                  L"PCP_STORAGEPARENT"
#define NCRYPT_PCP_PROVIDERHANDLE_PROPERTY                 L"PCP_PROVIDERMHANDLE"
#define NCRYPT_PCP_PLATFORMHANDLE_PROPERTY                 L"PCP_PLATFORMHANDLE"
#define NCRYPT_PCP_PLATFORM_BINDING_PCRMASK_PROPERTY       L"PCP_PLATFORM_BINDING_PCRMASK"
#define NCRYPT_PCP_PLATFORM_BINDING_PCRDIGESTLIST_PROPERTY L"PCP_PLATFORM_BINDING_PCRDIGESTLIST"
#define NCRYPT_PCP_PLATFORM_BINDING_PCRDIGEST_PROPERTY     L"PCP_PLATFORM_BINDING_PCRDIGEST"
#define NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY               L"PCP_KEY_USAGE_POLICY"
#define NCRYPT_PCP_RSA_SCHEME_PROPERTY                     L"PCP_RSA_SCHEME"
#define NCRYPT_PCP_TPM12_IDBINDING_PROPERTY                L"PCP_TPM12_IDBINDING"
#define NCRYPT_PCP_TPM12_IDBINDING_DYNAMIC_PROPERTY        L"PCP_TPM12_IDBINDING_DYNAMIC"
#define NCRYPT_PCP_TPM12_IDACTIVATION_PROPERTY             L"PCP_TPM12_IDACTIVATION"
#define NCRYPT_PCP_KEYATTESTATION_PROPERTY                 L"PCP_TPM12_KEYATTESTATION"
#define NCRYPT_PCP_ALTERNATE_KEY_STORAGE_LOCATION_PROPERTY L"PCP_ALTERNATE_KEY_STORAGE_LOCATION"

#if (NTDDI_VERSION >= NTDDI_WIN10)
#define NCRYPT_PCP_PLATFORM_BINDING_PCRALGID_PROPERTY      L"PCP_PLATFORM_BINDING_PCRALGID"
#endif // (NTDDI_VERSION >= NTDDI_WIN10)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
#define NCRYPT_PCP_HMAC_AUTH_POLICYREF                     L"PCP_HMAC_AUTH_POLICYREF"
#define NCRYPT_PCP_HMAC_AUTH_POLICYINFO                    L"PCP_HMAC_AUTH_POLICYINFO"
#define NCRYPT_PCP_HMAC_AUTH_NONCE                         L"PCP_HMAC_AUTH_NONCE"
#define NCRYPT_PCP_HMAC_AUTH_SIGNATURE                     L"PCP_HMAC_AUTH_SIGNATURE"
#define NCRYPT_PCP_HMAC_AUTH_TICKET                        L"PCP_HMAC_AUTH_TICKET"
#define NCRYPT_PCP_NO_DA_PROTECTION_PROPERTY               L"PCP_NO_DA_PROTECTION"
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS1)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
#define NCRYPT_PCP_TPM_MANUFACTURER_ID_PROPERTY            L"PCP_TPM_MANUFACTURER_ID"
#define NCRYPT_PCP_TPM_FW_VERSION_PROPERTY                 L"PCP_TPM_FW_VERSION"
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
#define NCRYPT_PCP_TPM2BNAME_PROPERTY                      L"PCP_TPM2BNAME"
#define NCRYPT_PCP_TPM_VERSION_PROPERTY                    L"PCP_TPM_VERSION"
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
#define NCRYPT_PCP_RAW_POLICYDIGEST_PROPERTY               L"PCP_RAW_POLICYDIGEST"
#define NCRYPT_PCP_KEY_CREATIONHASH_PROPERTY               L"PCP_KEY_CREATIONHASH"
#define NCRYPT_PCP_KEY_CREATIONTICKET_PROPERTY             L"PCP_KEY_CREATIONTICKET"
#define NCRYPT_PCP_RSA_SCHEME_HASH_ALG_PROPERTY            L"PCP_RSA_SCHEME_HASH_ALG"
#define NCRYPT_PCP_TPM_IFX_RSA_KEYGEN_PROHIBITED_PROPERTY  L"PCP_TPM_IFX_RSA_KEYGEN_PROHIBITED"
#define NCRYPT_PCP_TPM_IFX_RSA_KEYGEN_VULNERABILITY_PROPERTY \
                                                           L"PCP_TPM_IFX_RSA_KEYGEN_VULNERABILITY"
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS4)

    // NCRYPT_PCP_TPM_IFX_RSA_KEYGEN_VULNERABILITY_PROPERTY values

    #define IFX_RSA_KEYGEN_VUL_NOT_AFFECTED     0
    #define IFX_RSA_KEYGEN_VUL_AFFECTED_LEVEL_1 1
    #define IFX_RSA_KEYGEN_VUL_AFFECTED_LEVEL_2 2

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
#define NCRYPT_PCP_SESSIONID_PROPERTY                      L"PCP_SESSIONID"
#define NCRYPT_PCP_PSS_SALT_SIZE_PROPERTY                  L"PSS Salt Size"
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5)

    // TPM RSAPSS Salt size types

    #if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
    #define NCRYPT_TPM_PSS_SALT_SIZE_UNKNOWN                 0x00000000
    #define NCRYPT_TPM_PSS_SALT_SIZE_MAXIMUM                 0x00000001 // Pre-TPM Spec-1.16: Max allowed salt size
    #define NCRYPT_TPM_PSS_SALT_SIZE_HASHSIZE                0x00000002 // Post-1.16: PSS salt = hashLen
    #endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5)

#if (NTDDI_VERSION >= NTDDI_WIN10_MN)
#define NCRYPT_PCP_INTERMEDIATE_CA_EKCERT_PROPERTY         L"PCP_INTERMEDIATE_CA_EKCERT"
#endif // (NTDDI_VERSION >= NTDDI_WIN10_MN)

#if (NTDDI_VERSION >= NTDDI_WIN10_FE)
#define NCRYPT_PCP_PCRTABLE_ALGORITHM_PROPERTY             L"PCP_PCRTABLE_ALGORITHM"
#endif // (NTDDI_VERSION >= NTDDI_WIN10_FE)

#if (NTDDI_VERSION >= NTDDI_WIN10_CO)
#define NCRYPT_PCP_SYMMETRIC_KEYBITS_PROPERTY              L"PCP_SYMMETRIC_KEYBITS"
#endif // (NTDDI_VERSION >= NTDDI_WIN10_CO)

// TPM NCryptSignHash Flag

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
#define NCRYPT_TPM_PAD_PSS_IGNORE_SALT              0x00000020  // NCryptSignHash
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5)

//
// BCRYPT_PCP_KEY_USAGE_POLICY values
//
#define NCRYPT_TPM12_PROVIDER                  (0x00010000)
#define NCRYPT_PCP_SIGNATURE_KEY               (0x00000001)
#define NCRYPT_PCP_ENCRYPTION_KEY              (0x00000002)
#define NCRYPT_PCP_GENERIC_KEY                 (NCRYPT_PCP_SIGNATURE_KEY | NCRYPT_PCP_ENCRYPTION_KEY)
#define NCRYPT_PCP_STORAGE_KEY                 (0x00000004)
#define NCRYPT_PCP_IDENTITY_KEY                (0x00000008)
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
#define NCRYPT_PCP_HMACVERIFICATION_KEY        (0x00000010)
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS1)

//
// Additional property strings specific for the Smart Card Key Storage Provider
//
#if (NTDDI_VERSION >= NTDDI_WIN10)
#define NCRYPT_SCARD_NGC_KEY_NAME       L"SmartCardNgcKeyName"
#endif // (NTDDI_VERSION >= NTDDI_WIN10)

//
// Additional property strings specific to the Pluton Cryptographic Provider
//
#define NCRYPT_PLUTON_EKCERT_PROPERTY                       L"PLUTON_EKCERT"
#define NCRYPT_PLUTON_EKPUB_PROPERTY                        L"PLUTON_EKPUB"
#define NCRYPT_PLUTON_RSA_EKCERT_PROPERTY                   L"PLUTON_RSA_EKCERT"
#define NCRYPT_PLUTON_RSA_EKPUB_PROPERTY                    L"PLUTON_RSA_EKPUB"
#define NCRYPT_PLUTON_ECC_EKCERT_PROPERTY                   L"PLUTON_ECC_EKCERT"
#define NCRYPT_PLUTON_ECC_EKPUB_PROPERTY                    L"PLUTON_ECC_EKPUB"
#define NCRYPT_PLUTON_SESSION_ID_PROPERTY                   L"PLUTON_SESSION_ID"
#define NCRYPT_PLUTON_KDF_PARENT_KEY_UNIQUE_NAME_PROPERTY   L"PlutonKdfParentKeyUniqueName"
#define NCRYPT_PLUTON_KDF_PARAMS_BUFFER_DESC_PROPERTY       L"PlutonKdfParamsBufferDesc"

#if (NTDDI_VERSION >= NTDDI_WIN8)
//
// Used to set IV for block ciphers, before calling NCryptEncrypt/NCryptDecrypt
//
#define NCRYPT_INITIALIZATION_VECTOR            BCRYPT_INITIALIZATION_VECTOR
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define NCRYPT_CHANGEPASSWORD_PROPERTY  NCRYPT_PCP_CHANGEPASSWORD_PROPERTY
#define NCRYPT_ALTERNATE_KEY_STORAGE_LOCATION_PROPERTY  NCRYPT_PCP_ALTERNATE_KEY_STORAGE_LOCATION_PROPERTY
#define NCRYPT_KEY_ACCESS_POLICY_PROPERTY   L"Key Access Policy"
#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

// Maximum length of property name (in characters)
#define NCRYPT_MAX_PROPERTY_NAME        64

// Maximum length of property data (in bytes)
#define NCRYPT_MAX_PROPERTY_DATA        0x100000

// NCRYPT_EXPORT_POLICY_PROPERTY property flags.
#define NCRYPT_ALLOW_EXPORT_FLAG                0x00000001
#define NCRYPT_ALLOW_PLAINTEXT_EXPORT_FLAG      0x00000002
#define NCRYPT_ALLOW_ARCHIVING_FLAG             0x00000004
#define NCRYPT_ALLOW_PLAINTEXT_ARCHIVING_FLAG   0x00000008
#if (NTDDI_VERSION >= NTDDI_WIN11_GA)
#define NCRYPT_ALLOW_PKCS11_RSA_AES_EXPORT_FLAG 0x00000010
#endif // (NTDDI_VERSION >= NTDDI_WIN11_GA)

// NCRYPT_IMPL_TYPE_PROPERTY property flags.
#define NCRYPT_IMPL_HARDWARE_FLAG               0x00000001
#define NCRYPT_IMPL_SOFTWARE_FLAG               0x00000002
#define NCRYPT_IMPL_REMOVABLE_FLAG              0x00000008
#define NCRYPT_IMPL_HARDWARE_RNG_FLAG           0x00000010
#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
#define NCRYPT_IMPL_VIRTUAL_ISOLATION_FLAG      0x00000020
#endif

// NCRYPT_KEY_USAGE_PROPERTY property flags.
#define NCRYPT_ALLOW_DECRYPT_FLAG               0x00000001
#define NCRYPT_ALLOW_SIGNING_FLAG               0x00000002
#define NCRYPT_ALLOW_KEY_AGREEMENT_FLAG         0x00000004
#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
#define NCRYPT_ALLOW_KEY_ENVELOPE_FLAG          0x00000008
#define NCRYPT_ALLOW_KEY_IMPORT_FLAG            NCRYPT_ALLOW_KEY_ENVELOPE_FLAG
#endif
#if (NTDDI_VERSION >= NTDDI_WIN11_SV3)
#define NCRYPT_ALLOW_KEY_ATTESTATION_FLAG       0x00000010
#endif
#if (NTDDI_VERSION >= NTDDI_WIN11_SV4)
#define NCRYPT_ALLOW_KEY_IMPORT_EPHEMERAL_FLAG  0x00000020
#endif
#define NCRYPT_ALLOW_ALL_USAGES                 0x00ffffff

// NCRYPT_UI_POLICY_PROPERTY property flags and structure
#define NCRYPT_UI_PROTECT_KEY_FLAG              0x00000001
#define NCRYPT_UI_FORCE_HIGH_PROTECTION_FLAG    0x00000002
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
#define NCRYPT_UI_FINGERPRINT_PROTECTION_FLAG   0x00000004
#define NCRYPT_UI_APPCONTAINER_ACCESS_MEDIUM_FLAG 0x00000008
#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)



#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

//
// Pin Cache Provider Properties
//

#define NCRYPT_PIN_CACHE_FREE_APPLICATION_TICKET_PROPERTY   L"PinCacheFreeApplicationTicket"

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)

#define NCRYPT_PIN_CACHE_FLAGS_PROPERTY                     L"PinCacheFlags"

// The NCRYPT_PIN_CACHE_FLAGS_PROPERTY property is a DWORD value that can be set from a trusted process. The
// following flags can be set.
#define NCRYPT_PIN_CACHE_DISABLE_DPL_FLAG                   0x00000001

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS1)


//
// Pin Cache Key Properties
//

#define NCRYPT_PIN_CACHE_APPLICATION_TICKET_PROPERTY        L"PinCacheApplicationTicket"
#define NCRYPT_PIN_CACHE_APPLICATION_IMAGE_PROPERTY         L"PinCacheApplicationImage"
#define NCRYPT_PIN_CACHE_APPLICATION_STATUS_PROPERTY        L"PinCacheApplicationStatus"
#define NCRYPT_PIN_CACHE_PIN_PROPERTY                       L"PinCachePin"
#define NCRYPT_PIN_CACHE_IS_GESTURE_REQUIRED_PROPERTY       L"PinCacheIsGestureRequired"


#define NCRYPT_PIN_CACHE_REQUIRE_GESTURE_FLAG               0x00000001

// The NCRYPT_PIN_CACHE_PIN_PROPERTY and NCRYPT_PIN_CACHE_APPLICATION_TICKET_PROPERTY properties
// return a 32 byte random unique ID encoded as a null terminated base64 Unicode string. The string length
// is 32 * 4/3 + 1 characters = 45 characters, 90 bytes
#define NCRYPT_PIN_CACHE_PIN_BYTE_LENGTH                    90
#define NCRYPT_PIN_CACHE_APPLICATION_TICKET_BYTE_LENGTH     90

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)

#define NCRYPT_PIN_CACHE_CLEAR_PROPERTY                     L"PinCacheClear"

// The NCRYPT_PIN_CACHE_CLEAR_PROPERTY property is a DWORD value. The following option can be set:
#define NCRYPT_PIN_CACHE_CLEAR_FOR_CALLING_PROCESS_OPTION   0x00000001

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS1)



typedef struct __NCRYPT_UI_POLICY
{
    DWORD   dwVersion;
    DWORD   dwFlags;
    LPCWSTR pszCreationTitle;
    LPCWSTR pszFriendlyName;
    LPCWSTR pszDescription;
} NCRYPT_UI_POLICY;

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
#define NCRYPT_KEY_ACCESS_POLICY_VERSION 1
#define NCRYPT_ALLOW_SILENT_KEY_ACCESS   0x00000001
typedef struct __NCRYPT_KEY_ACCESS_POLICY_BLOB
{
    DWORD   dwVersion;
    DWORD   dwPolicyFlags;
    DWORD   cbUserSid;
    DWORD   cbApplicationSid;
    //  User Sid
    //  Application Sid
}NCRYPT_KEY_ACCESS_POLICY_BLOB;
#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

// NCRYPT_LENGTHS_PROPERTY property structure.
typedef struct __NCRYPT_SUPPORTED_LENGTHS
{
    DWORD   dwMinLength;
    DWORD   dwMaxLength;
    DWORD   dwIncrement;
    DWORD   dwDefaultLength;
} NCRYPT_SUPPORTED_LENGTHS;

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
// NCRYPT_PCP_HMAC_AUTH_SIGNATURE property structure.
typedef struct __NCRYPT_PCP_HMAC_AUTH_SIGNATURE_INFO
{
    DWORD       dwVersion;
    INT32       iExpiration;
    BYTE        pabNonce[32];
    BYTE        pabPolicyRef[32];
    BYTE        pabHMAC[32];
} NCRYPT_PCP_HMAC_AUTH_SIGNATURE_INFO;
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS1)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
// NCRYPT_PCP_TPM_FW_VERSION property structure.
typedef struct __NCRYPT_PCP_TPM_FW_VERSION_INFO
{
    UINT16      major1;
    UINT16      major2;
    UINT16      minor1;
    UINT16      minor2;
} NCRYPT_PCP_TPM_FW_VERSION_INFO;
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
// NCRYPT_PCP_RAW_POLICYDIGEST_PROPERTY structure
typedef struct __NCRYPT_PCP_RAW_POLICYDIGEST
{
    DWORD   dwVersion;
    DWORD   cbDigest;
} NCRYPT_PCP_RAW_POLICYDIGEST_INFO;
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS2)

// NCryptGetProperty flags
#define NCRYPT_PERSIST_ONLY_FLAG        0x40000000

_Check_return_
_Success_( return == 0 )
SECURITY_STATUS
WINAPI
NCryptGetProperty(
    _In_    NCRYPT_HANDLE hObject,
    _In_    LPCWSTR pszProperty,
    _Out_writes_bytes_to_opt_(cbOutput, *pcbResult) PBYTE pbOutput,
    _In_    DWORD   cbOutput,
    _Out_   DWORD * pcbResult,
    _In_    DWORD   dwFlags);



// NCryptSetProperty flags
#define NCRYPT_PERSIST_FLAG             0x80000000
#define NCRYPT_PERSIST_ONLY_FLAG        0x40000000

_Check_return_
SECURITY_STATUS
WINAPI
NCryptSetProperty(
    _In_    NCRYPT_HANDLE hObject,
    _In_    LPCWSTR pszProperty,
    _In_reads_bytes_(cbInput) PBYTE pbInput,
    _In_    DWORD   cbInput,
    _In_    DWORD   dwFlags);



// NCryptFinalizeKey flags
#define     NCRYPT_WRITE_KEY_TO_LEGACY_STORE_FLAG   0x00000200

_Check_return_
SECURITY_STATUS
WINAPI
NCryptFinalizeKey(
    _In_    NCRYPT_KEY_HANDLE hKey,
    _In_    DWORD   dwFlags);



_Check_return_
SECURITY_STATUS
WINAPI
NCryptEncrypt(
    _In_    NCRYPT_KEY_HANDLE hKey,
    _In_reads_bytes_opt_(cbInput) PBYTE pbInput,
    _In_    DWORD   cbInput,
    _In_opt_    VOID *pPaddingInfo,
    _Out_writes_bytes_to_opt_(cbOutput, *pcbResult) PBYTE pbOutput,
    _In_    DWORD   cbOutput,
    _Out_   DWORD * pcbResult,
    _In_    DWORD   dwFlags);



_Check_return_
SECURITY_STATUS
WINAPI
NCryptDecrypt(
    _In_    NCRYPT_KEY_HANDLE hKey,
    _In_reads_bytes_opt_(cbInput) PBYTE pbInput,
    _In_    DWORD   cbInput,
    _In_opt_    VOID *pPaddingInfo,
    _Out_writes_bytes_to_opt_(cbOutput, *pcbResult) PBYTE pbOutput,
    _In_    DWORD   cbOutput,
    _Out_   DWORD * pcbResult,
    _In_    DWORD   dwFlags);



#if (NTDDI_VERSION >= NTDDI_WIN11_GE)

_Check_return_
SECURITY_STATUS
WINAPI
NCryptEncapsulate(
    _In_                              NCRYPT_KEY_HANDLE       hKey,
    _Out_writes_bytes_to_opt_(cbSecretKey ,*pcbSecretKey)
                                      PBYTE                   pbSecretKey,
    _In_                              ULONG                   cbSecretKey,
    _Out_                             ULONG                  *pcbSecretKey,
    _Out_writes_bytes_to_opt_(cbCipherText ,*pcbCipherText)
                                      PBYTE                   pbCipherText,
    _In_                              ULONG                   cbCipherText,
    _Out_                             ULONG                  *pcbCipherText,
    _In_                              ULONG                   dwFlags);


_Check_return_
SECURITY_STATUS
WINAPI
NCryptDecapsulate(
    _In_                              NCRYPT_KEY_HANDLE       hKey,
    _In_reads_bytes_(cbCipherText)    PBYTE                   pbCipherText,
    _In_                              ULONG                   cbCipherText,
    _Out_writes_bytes_to_opt_(cbSecretKey, *pcbSecretKey)
                                      PBYTE                   pbSecretKey,
    _In_                              ULONG                   cbSecretKey,
    _Out_                             ULONG                  *pcbSecretKey,
    _In_                              ULONG                   dwFlags);


#endif // (NTDDI_VERSION >= NTDDI_WIN11_GE)

#if (NTDDI_VERSION >= NTDDI_WIN8)

typedef struct _NCRYPT_KEY_BLOB_HEADER
{
    ULONG   cbSize;             // size of this structure
    ULONG   dwMagic;
    ULONG   cbAlgName;          // size of the algorithm, in bytes, including terminating 0
    ULONG   cbKeyData;
} NCRYPT_KEY_BLOB_HEADER, *PNCRYPT_KEY_BLOB_HEADER;

#define NCRYPT_CIPHER_KEY_BLOB_MAGIC    0x52485043      // CPHR
#define NCRYPT_KDF_KEY_BLOB_MAGIC       0x3146444B      // KDF1
#define NCRYPT_PROTECTED_KEY_BLOB_MAGIC 0x4B545250      // PRTK

#define NCRYPT_CIPHER_KEY_BLOB          L"CipherKeyBlob"
#define NCRYPT_KDF_KEY_BLOB             L"KDFKeyBlob"
#define NCRYPT_PROTECTED_KEY_BLOB       L"ProtectedKeyBlob"

#endif  // (NTDDI_VERSION >= NTDDI_WIN8)

typedef struct NCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER
{
    DWORD magic;
    DWORD cbHeader;
    DWORD cbPublic;
    DWORD cbPrivate;
    DWORD cbName;
} NCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER, *PNCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER;

#define NCRYPT_TPM_LOADABLE_KEY_BLOB_MIN_SIZE sizeof(NCRYPT_TPM_LOADABLE_KEY_BLOB_HEADER)
#define NCRYPT_TPM_LOADABLE_KEY_BLOB L"PcpTpmProtectedKeyBlob"
#define NCRYPT_TPM_LOADABLE_KEY_BLOB_MAGIC 0x4D54504B //'MTPK'

typedef struct NCRYPT_TPM_PERSISTENT_KEY_BLOB_HEADER
{
    DWORD magic;
    DWORD cbHeader;
    DWORD tpmHandle;
} NCRYPT_TPM_PERSISTENT_KEY_BLOB_HEADER, *PNCRYPT_TPM_PERSISTENT_KEY_BLOB_HEADER;

#define NCRYPT_TPM_PERSISTENT_KEY_BLOB_MIN_SIZE sizeof(NCRYPT_TPM_PERSISTENT_KEY_BLOB_HEADER)
#define NCRYPT_TPM_PERSISTENT_KEY_BLOB L"PcpTpmPersistentKeyBlob"
#define NCRYPT_TPM_PERSISTENT_KEY_BLOB_MAGIC 0x4D54504B //'MTPE'

#define NCRYPT_PKCS7_ENVELOPE_BLOB      L"PKCS7_ENVELOPE"
#define NCRYPT_PKCS8_PRIVATE_KEY_BLOB   L"PKCS8_PRIVATEKEY"
#define NCRYPT_OPAQUETRANSPORT_BLOB     L"OpaqueTransport"

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
#define NCRYPT_ISOLATED_KEY_ENVELOPE_BLOB   L"ISOLATED_KEY_ENVELOPE"
#endif

#if (NTDDI_VERSION >= NTDDI_WIN11_GE)

typedef struct _NCRYPT_PQ_BLOB
{
    ULONG   dwMagic;
    ULONG   cbBCryptType;   // Number of bytes in BCrypt blob type.
    ULONG   cbBCryptBlob;   // Number of bytes in BCrypt blob data.
    // UCHAR BCryptType[cbBCryptType]  -- The BCrypt blob type including '\0'. For example, BCRYPT_PQDSA_PRIVATE_BLOB or BCRYPT_PQDSA_PUBLIC_BLOB .
    // UCHAR BCryptBlob[cbBCryptBlob]  -- BCrypt blob data. Currently it's BCRYPT_PQDSA_KEY_BLOB.
} NCRYPT_PQ_BLOB, *PNCRYPT_PQ_BLOB;

#define NCRYPT_PQ_PRIVATE_BLOB_MAGIC   0x52505150      // PQPR

#define NCRYPT_PQ_PRIVATE_KEY_BLOB     L"PQPrivateKeyBlob"
#define NCRYPT_PQ_PUBLIC_KEY_BLOB      BCRYPT_PUBLIC_KEY_BLOB


#endif // (NTDDI_VERSION >= NTDDI_WIN11_GE)

// NCryptImportKey flags
#define NCRYPT_MACHINE_KEY_FLAG         0x00000020
#define NCRYPT_DO_NOT_FINALIZE_FLAG     0x00000400
#define NCRYPT_EXPORT_LEGACY_FLAG       0x00000800

_Check_return_
SECURITY_STATUS
WINAPI
NCryptImportKey(
    _In_    NCRYPT_PROV_HANDLE hProvider,
    _In_opt_ NCRYPT_KEY_HANDLE hImportKey,
    _In_    LPCWSTR pszBlobType,
    _In_opt_ NCryptBufferDesc *pParameterList,
    _Out_   NCRYPT_KEY_HANDLE *phKey,
    _In_reads_bytes_(cbData) PBYTE pbData,
    _In_    DWORD   cbData,
    _In_    DWORD   dwFlags);



_Check_return_
SECURITY_STATUS
WINAPI
NCryptExportKey(
    _In_    NCRYPT_KEY_HANDLE hKey,
    _In_opt_ NCRYPT_KEY_HANDLE hExportKey,
    _In_    LPCWSTR pszBlobType,
    _In_opt_ NCryptBufferDesc *pParameterList,
    _Out_writes_bytes_to_opt_(cbOutput, *pcbResult) PBYTE pbOutput,
    _In_    DWORD   cbOutput,
    _Out_   DWORD * pcbResult,
    _In_    DWORD   dwFlags);



_Check_return_
SECURITY_STATUS
WINAPI
NCryptSignHash(
    _In_    NCRYPT_KEY_HANDLE hKey,
    _In_opt_    VOID *pPaddingInfo,
    _In_reads_bytes_(cbHashValue) PBYTE pbHashValue,
    _In_    DWORD   cbHashValue,
    _Out_writes_bytes_to_opt_(cbSignature, *pcbResult) PBYTE pbSignature,
    _In_    DWORD   cbSignature,
    _Out_   DWORD * pcbResult,
    _In_    DWORD   dwFlags);



_Check_return_
SECURITY_STATUS
WINAPI
NCryptVerifySignature(
    _In_    NCRYPT_KEY_HANDLE hKey,
    _In_opt_    VOID *pPaddingInfo,
    _In_reads_bytes_(cbHashValue) PBYTE pbHashValue,
    _In_    DWORD   cbHashValue,
    _In_reads_bytes_(cbSignature) PBYTE pbSignature,
    _In_    DWORD   cbSignature,
    _In_    DWORD   dwFlags);



SECURITY_STATUS
WINAPI
NCryptDeleteKey(
    _In_    NCRYPT_KEY_HANDLE hKey,
    _In_    DWORD   dwFlags);



SECURITY_STATUS
WINAPI
NCryptFreeObject(
    _In_    NCRYPT_HANDLE hObject);



#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
BOOL
WINAPI
NCryptIsKeyHandle(
    _In_    NCRYPT_KEY_HANDLE hKey);

_Check_return_
SECURITY_STATUS
WINAPI
NCryptTranslateHandle(
    _Out_opt_ NCRYPT_PROV_HANDLE *phProvider,
    _Out_   NCRYPT_KEY_HANDLE *phKey,
    _In_    HCRYPTPROV hLegacyProv,
    _In_opt_ HCRYPTKEY hLegacyKey,
    _In_opt_ DWORD  dwLegacyKeySpec,
    _In_    DWORD   dwFlags);

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion


// NCryptNotifyChangeKey flags
#define NCRYPT_REGISTER_NOTIFY_FLAG     0x00000001
#define NCRYPT_UNREGISTER_NOTIFY_FLAG   0x00000002
#define NCRYPT_MACHINE_KEY_FLAG         0x00000020

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Check_return_
SECURITY_STATUS
WINAPI
NCryptNotifyChangeKey(
    _In_    NCRYPT_PROV_HANDLE hProvider,
    _Inout_ HANDLE *phEvent,
    _In_    DWORD   dwFlags);



#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

_Check_return_
SECURITY_STATUS
WINAPI
NCryptSecretAgreement(
    _In_    NCRYPT_KEY_HANDLE hPrivKey,
    _In_    NCRYPT_KEY_HANDLE hPubKey,
    _Out_   NCRYPT_SECRET_HANDLE *phAgreedSecret,
    _In_    DWORD   dwFlags);



_Check_return_
SECURITY_STATUS
WINAPI
NCryptDeriveKey(
    _In_        NCRYPT_SECRET_HANDLE hSharedSecret,
    _In_        LPCWSTR              pwszKDF,
    _In_opt_    NCryptBufferDesc     *pParameterList,
    _Out_writes_bytes_to_opt_(cbDerivedKey, *pcbResult) PBYTE pbDerivedKey,
    _In_        DWORD                cbDerivedKey,
    _Out_       DWORD                *pcbResult,
    _In_        ULONG                dwFlags);



#if (NTDDI_VERSION >= NTDDI_WIN8)

_Check_return_
SECURITY_STATUS
WINAPI
NCryptKeyDerivation(
    _In_        NCRYPT_KEY_HANDLE   hKey,
    _In_opt_    NCryptBufferDesc    *pParameterList,
    _Out_writes_bytes_to_(cbDerivedKey, *pcbResult) PUCHAR pbDerivedKey,
    _In_        DWORD               cbDerivedKey,
    _Out_       DWORD               *pcbResult,
    _In_        ULONG               dwFlags);

#endif  // (NTDDI_VERSION >= NTDDI_WIN8)


#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

_Check_return_
SECURITY_STATUS
WINAPI
NCryptCreateClaim(
    _In_opt_    NCRYPT_KEY_HANDLE   hSubjectKey,
    _In_opt_    NCRYPT_KEY_HANDLE   hAuthorityKey,
    _In_        DWORD               dwClaimType,
    _In_opt_    NCryptBufferDesc    *pParameterList,
    _Out_writes_bytes_to_opt_(cbClaimBlob, *pcbResult) PBYTE pbClaimBlob,
    _In_        DWORD               cbClaimBlob,
    _Out_       DWORD               *pcbResult,
    _In_        DWORD               dwFlags);

#endif  // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)


#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

_Check_return_
SECURITY_STATUS
WINAPI
NCryptVerifyClaim(
    _In_        NCRYPT_KEY_HANDLE   hSubjectKey,
    _In_opt_    NCRYPT_KEY_HANDLE   hAuthorityKey,
    _In_        DWORD               dwClaimType,
    _In_opt_    NCryptBufferDesc    *pParameterList,
    _In_reads_bytes_(cbClaimBlob) PBYTE pbClaimBlob,
    _In_        DWORD               cbClaimBlob,
    _Out_       NCryptBufferDesc    *pOutput,
    _In_        DWORD               dwFlags);

#endif  // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)


#define NCRYPT_KEY_STORAGE_INTERFACE_VERSION BCRYPT_MAKE_INTERFACE_VERSION(1,0)
#define NCRYPT_KEY_STORAGE_INTERFACE_VERSION_2 BCRYPT_MAKE_INTERFACE_VERSION(2,0)
#define NCRYPT_KEY_STORAGE_INTERFACE_VERSION_3 BCRYPT_MAKE_INTERFACE_VERSION(3,0)
#define NCRYPT_KEY_STORAGE_INTERFACE_VERSION_4 BCRYPT_MAKE_INTERFACE_VERSION(4,0)


#ifdef __cplusplus
}       // Balance extern "C" above
#endif

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif // __NCRYPT_H__

