//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 1992-1999.
//
//  File:       wincrypt.h
//
//  Contents:   Cryptographic API Prototypes and Definitions
//
//----------------------------------------------------------------------------

#ifndef __WINCRYPT_H__
#define __WINCRYPT_H__

#include <specstrings.h>        /* for SAL annotations */

#if defined (_MSC_VER)

#if ( _MSC_VER >= 800 )
#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4668)    /* #if not_defined treated as #if 0 */
#pragma warning(disable:4820)    /* padding added after data member */
#endif
#pragma warning(disable:4201)    /* Nameless struct/union */
#endif

#if (_MSC_VER > 1020)
#pragma once
#endif

#endif

#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
* The relationship between partitions and families (from: winapifamily.h) 
*
* The mapping between families and partitions is summarized here.
* An X indicates that the given partition is active for the given
* platform/family.
*
*                             +-------------------+
*                             |      *Partition*  |
*                             +---+---+---+---+---+
*                             |   |   |   | P |   |
*                             |   |   |   | H |   |
*                             | D |   |   | O | O |
*                             | E |   | P | N | N |
*                             | S |   | C | E | E |
*                             | K |   | _ | _ | C |
*                             | T | A | A | A | O |
* +-------------------------+-+ O | P | P | P | R |
* |     *Platform/Family*    \| P | P | P | P | E |
* +---------------------------+---+---+---+---+---+
* | WINAPI_FAMILY_DESKTOP_APP | X | X | X |   |   |
* +---------------------------+---+---+---+---+---+
* |      WINAPI_FAMILY_PC_APP |   | X | X |   |   |
* +---------------------------+---+---+---+---+---+
* |   WINAPI_FAMILY_PHONE_APP |   | X |   | X |   |
* +---------------------------+---+---+---+---+---+
* | WINAPI_FAMILY_ONECORE_APP |   |   |   |   | X |
* +---------------------------+---+---+---+---+---+
*
* The table above is encoded in the following expressions,
* each of which evaluates to 1 or 0.
*
* Whenever a new family is added, all of these expressions
* need to be reconsidered.
*/

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifndef _HRESULT_DEFINED
#define _HRESULT_DEFINED
typedef _Return_type_success_(return >= 0) long HRESULT;
#endif

#ifndef WINADVAPI
#define WINADVAPI
#endif

#ifndef WINAPI
#define WINAPI __stdcall
#endif

#ifndef CALLBACK
#define CALLBACK __stdcall
#endif

#ifndef DECLSPEC_IMPORT
#define DECLSPEC_IMPORT
#endif

#ifndef CONST
#define CONST const
#endif

#ifndef IN
#define IN
#endif

#ifndef OUT
#define OUT
#endif

#ifndef OPTIONAL
#define OPTIONAL
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)

#if !defined(WINCRYPT32API)
#if !defined(_CRYPT32_)
#define WINCRYPT32API DECLSPEC_IMPORT
#else
#define WINCRYPT32API
#endif
#endif

#else

#if !defined(_CRYPT32_)
#define WINCRYPT32API DECLSPEC_IMPORT
#else
#define WINCRYPT32API
#endif

#endif //(NTDDI_VERSION >= NTDDI_VISTA)

#if !defined(WINCRYPT32STRINGAPI)
#if !defined(_CRYPT32STRING_)
#define WINCRYPT32STRINGAPI WINCRYPT32API
#else
#define WINCRYPT32STRINGAPI
#endif
#endif

//
// Algorithm IDs and Flags
//

// ALG_ID crackers
#define GET_ALG_CLASS(x)                (x & (7 << 13))
#define GET_ALG_TYPE(x)                 (x & (15 << 9))
#define GET_ALG_SID(x)                  (x & (511))

// Algorithm classes
// certenrolld_begin -- ALG_CLASS_*
#define ALG_CLASS_ANY                   (0)
#define ALG_CLASS_SIGNATURE             (1 << 13)
#define ALG_CLASS_MSG_ENCRYPT           (2 << 13)
#define ALG_CLASS_DATA_ENCRYPT          (3 << 13)
#define ALG_CLASS_HASH                  (4 << 13)
#define ALG_CLASS_KEY_EXCHANGE          (5 << 13)
#define ALG_CLASS_ALL                   (7 << 13)
// certenrolld_end

// Algorithm types
#define ALG_TYPE_ANY                    (0)
#define ALG_TYPE_DSS                    (1 << 9)
#define ALG_TYPE_RSA                    (2 << 9)
#define ALG_TYPE_BLOCK                  (3 << 9)
#define ALG_TYPE_STREAM                 (4 << 9)
#define ALG_TYPE_DH                     (5 << 9)
#define ALG_TYPE_SECURECHANNEL          (6 << 9)
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define ALG_TYPE_ECDH                   (7 << 9)
#endif //(NTDDI_VERSION >= NTDDI_VISTA)
#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
#define ALG_TYPE_THIRDPARTY             (8 << 9)
#endif //(NTDDI_VERSION >= NTDDI_WIN10_RS1)

// Generic sub-ids
#define ALG_SID_ANY                     (0)

// Generic ThirdParty sub-ids
#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
#define ALG_SID_THIRDPARTY_ANY          (0)
#endif //(NTDDI_VERSION >= NTDDI_WIN10_RS1)

// Some RSA sub-ids
#define ALG_SID_RSA_ANY                 0
#define ALG_SID_RSA_PKCS                1
#define ALG_SID_RSA_MSATWORK            2
#define ALG_SID_RSA_ENTRUST             3
#define ALG_SID_RSA_PGP                 4

// Some DSS sub-ids
//
#define ALG_SID_DSS_ANY                 0
#define ALG_SID_DSS_PKCS                1
#define ALG_SID_DSS_DMS                 2
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define ALG_SID_ECDSA                   3
#endif //(NTDDI_VERSION >= NTDDI_VISTA)

// Block cipher sub ids
// DES sub_ids
#define ALG_SID_DES                     1
#define ALG_SID_3DES                    3
#define ALG_SID_DESX                    4
#define ALG_SID_IDEA                    5
#define ALG_SID_CAST                    6
#define ALG_SID_SAFERSK64               7
#define ALG_SID_SAFERSK128              8
#define ALG_SID_3DES_112                9
#define ALG_SID_CYLINK_MEK              12
#define ALG_SID_RC5                     13
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define ALG_SID_AES_128                 14
#define ALG_SID_AES_192                 15
#define ALG_SID_AES_256                 16
#define ALG_SID_AES                     17
#endif //(NTDDI_VERSION >= NTDDI_WINXP)

// Fortezza sub-ids
#define ALG_SID_SKIPJACK                10
#define ALG_SID_TEK                     11

// KP_MODE
#define CRYPT_MODE_CBCI                 6       // ANSI CBC Interleaved
#define CRYPT_MODE_CFBP                 7       // ANSI CFB Pipelined
#define CRYPT_MODE_OFBP                 8       // ANSI OFB Pipelined
#define CRYPT_MODE_CBCOFM               9       // ANSI CBC + OF Masking
#define CRYPT_MODE_CBCOFMI              10      // ANSI CBC + OFM Interleaved

// RC2 sub-ids
#define ALG_SID_RC2                     2

// Stream cipher sub-ids
#define ALG_SID_RC4                     1
#define ALG_SID_SEAL                    2

// Diffie-Hellman sub-ids
#define ALG_SID_DH_SANDF                1
#define ALG_SID_DH_EPHEM                2
#define ALG_SID_AGREED_KEY_ANY          3
#define ALG_SID_KEA                     4
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define ALG_SID_ECDH                    5
#define ALG_SID_ECDH_EPHEM              6
#endif //(NTDDI_VERSION >= NTDDI_VISTA)

// Hash sub ids
#define ALG_SID_MD2                     1
#define ALG_SID_MD4                     2
#define ALG_SID_MD5                     3
#define ALG_SID_SHA                     4
#define ALG_SID_SHA1                    4
#define ALG_SID_MAC                     5
#define ALG_SID_RIPEMD                  6
#define ALG_SID_RIPEMD160               7
#define ALG_SID_SSL3SHAMD5              8
#define ALG_SID_HMAC                    9
#define ALG_SID_TLS1PRF                 10
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define ALG_SID_HASH_REPLACE_OWF        11
#endif //(NTDDI_VERSION >= NTDDI_WINXP)
#if (NTDDI_VERSION > NTDDI_WINXPSP2)
#define ALG_SID_SHA_256                 12
#define ALG_SID_SHA_384                 13
#define ALG_SID_SHA_512                 14
#endif //(NTDDI_VERSION > NTDDI_WINXPSP2)

// secure channel sub ids
#define ALG_SID_SSL3_MASTER             1
#define ALG_SID_SCHANNEL_MASTER_HASH    2
#define ALG_SID_SCHANNEL_MAC_KEY        3
#define ALG_SID_PCT1_MASTER             4
#define ALG_SID_SSL2_MASTER             5
#define ALG_SID_TLS1_MASTER             6
#define ALG_SID_SCHANNEL_ENC_KEY        7

#if (NTDDI_VERSION >= NTDDI_VISTA)
// misc ECC sub ids
#define ALG_SID_ECMQV                   1
#endif //(NTDDI_VERSION >= NTDDI_VISTA)

// Our silly example sub-id
#define ALG_SID_EXAMPLE                 80

// certenrolls_begin -- PROV_ENUMALGS_EX
#ifndef ALGIDDEF
#define ALGIDDEF
typedef unsigned int ALG_ID;
#endif
// certenrolls_end

// algorithm identifier definitions
#define CALG_MD2                (ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_MD2)
#define CALG_MD4                (ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_MD4)
#define CALG_MD5                (ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_MD5)
#define CALG_SHA                (ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA)
#define CALG_SHA1               (ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA1)
#define CALG_MAC                (ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_MAC)           // Deprecated. Don't use.
#define CALG_RSA_SIGN           (ALG_CLASS_SIGNATURE | ALG_TYPE_RSA | ALG_SID_RSA_ANY)
#define CALG_DSS_SIGN           (ALG_CLASS_SIGNATURE | ALG_TYPE_DSS | ALG_SID_DSS_ANY)
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define CALG_NO_SIGN            (ALG_CLASS_SIGNATURE | ALG_TYPE_ANY | ALG_SID_ANY)
#endif //(NTDDI_VERSION >= NTDDI_WINXP)
#define CALG_RSA_KEYX           (ALG_CLASS_KEY_EXCHANGE|ALG_TYPE_RSA|ALG_SID_RSA_ANY)
#define CALG_DES                (ALG_CLASS_DATA_ENCRYPT|ALG_TYPE_BLOCK|ALG_SID_DES)
#define CALG_3DES_112           (ALG_CLASS_DATA_ENCRYPT|ALG_TYPE_BLOCK|ALG_SID_3DES_112)
#define CALG_3DES               (ALG_CLASS_DATA_ENCRYPT|ALG_TYPE_BLOCK|ALG_SID_3DES)
#define CALG_DESX               (ALG_CLASS_DATA_ENCRYPT|ALG_TYPE_BLOCK|ALG_SID_DESX)
#define CALG_RC2                (ALG_CLASS_DATA_ENCRYPT|ALG_TYPE_BLOCK|ALG_SID_RC2)
#define CALG_RC4                (ALG_CLASS_DATA_ENCRYPT|ALG_TYPE_STREAM|ALG_SID_RC4)
#define CALG_SEAL               (ALG_CLASS_DATA_ENCRYPT|ALG_TYPE_STREAM|ALG_SID_SEAL)
#define CALG_DH_SF              (ALG_CLASS_KEY_EXCHANGE|ALG_TYPE_DH|ALG_SID_DH_SANDF)
#define CALG_DH_EPHEM           (ALG_CLASS_KEY_EXCHANGE|ALG_TYPE_DH|ALG_SID_DH_EPHEM)
#define CALG_AGREEDKEY_ANY      (ALG_CLASS_KEY_EXCHANGE|ALG_TYPE_DH|ALG_SID_AGREED_KEY_ANY)
#define CALG_KEA_KEYX           (ALG_CLASS_KEY_EXCHANGE|ALG_TYPE_DH|ALG_SID_KEA)
#define CALG_HUGHES_MD5         (ALG_CLASS_KEY_EXCHANGE|ALG_TYPE_ANY|ALG_SID_MD5)
#define CALG_SKIPJACK           (ALG_CLASS_DATA_ENCRYPT|ALG_TYPE_BLOCK|ALG_SID_SKIPJACK)
#define CALG_TEK                (ALG_CLASS_DATA_ENCRYPT|ALG_TYPE_BLOCK|ALG_SID_TEK)
#define CALG_CYLINK_MEK         (ALG_CLASS_DATA_ENCRYPT|ALG_TYPE_BLOCK|ALG_SID_CYLINK_MEK)  // Deprecated. Do not use
#define CALG_SSL3_SHAMD5        (ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SSL3SHAMD5)
#define CALG_SSL3_MASTER        (ALG_CLASS_MSG_ENCRYPT|ALG_TYPE_SECURECHANNEL|ALG_SID_SSL3_MASTER)
#define CALG_SCHANNEL_MASTER_HASH   (ALG_CLASS_MSG_ENCRYPT|ALG_TYPE_SECURECHANNEL|ALG_SID_SCHANNEL_MASTER_HASH)
#define CALG_SCHANNEL_MAC_KEY   (ALG_CLASS_MSG_ENCRYPT|ALG_TYPE_SECURECHANNEL|ALG_SID_SCHANNEL_MAC_KEY)
#define CALG_SCHANNEL_ENC_KEY   (ALG_CLASS_MSG_ENCRYPT|ALG_TYPE_SECURECHANNEL|ALG_SID_SCHANNEL_ENC_KEY)
#define CALG_PCT1_MASTER        (ALG_CLASS_MSG_ENCRYPT|ALG_TYPE_SECURECHANNEL|ALG_SID_PCT1_MASTER)
#define CALG_SSL2_MASTER        (ALG_CLASS_MSG_ENCRYPT|ALG_TYPE_SECURECHANNEL|ALG_SID_SSL2_MASTER)
#define CALG_TLS1_MASTER        (ALG_CLASS_MSG_ENCRYPT|ALG_TYPE_SECURECHANNEL|ALG_SID_TLS1_MASTER)
#define CALG_RC5                (ALG_CLASS_DATA_ENCRYPT|ALG_TYPE_BLOCK|ALG_SID_RC5)
#define CALG_HMAC               (ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_HMAC)
#define CALG_TLS1PRF            (ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_TLS1PRF)
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define CALG_HASH_REPLACE_OWF   (ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_HASH_REPLACE_OWF)
#define CALG_AES_128            (ALG_CLASS_DATA_ENCRYPT|ALG_TYPE_BLOCK|ALG_SID_AES_128)
#define CALG_AES_192            (ALG_CLASS_DATA_ENCRYPT|ALG_TYPE_BLOCK|ALG_SID_AES_192)
#define CALG_AES_256            (ALG_CLASS_DATA_ENCRYPT|ALG_TYPE_BLOCK|ALG_SID_AES_256)
#define CALG_AES                (ALG_CLASS_DATA_ENCRYPT|ALG_TYPE_BLOCK|ALG_SID_AES)
#endif //(NTDDI_VERSION >= NTDDI_WINXP)
#if (NTDDI_VERSION > NTDDI_WINXPSP2)
#define CALG_SHA_256            (ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA_256)
#define CALG_SHA_384            (ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA_384)
#define CALG_SHA_512            (ALG_CLASS_HASH | ALG_TYPE_ANY | ALG_SID_SHA_512)
#endif //(NTDDI_VERSION > NTDDI_WINXPSP2)
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define CALG_ECDH               (ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_DH | ALG_SID_ECDH)
#define CALG_ECDH_EPHEM         (ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_ECDH | ALG_SID_ECDH_EPHEM)
#define CALG_ECMQV              (ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_ANY | ALG_SID_ECMQV)
#define CALG_ECDSA              (ALG_CLASS_SIGNATURE | ALG_TYPE_DSS | ALG_SID_ECDSA)
#define CALG_NULLCIPHER         (ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_ANY | 0)
#endif //(NTDDI_VERSION >= NTDDI_VISTA)
#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
#define CALG_THIRDPARTY_KEY_EXCHANGE    (ALG_CLASS_KEY_EXCHANGE | ALG_TYPE_THIRDPARTY | ALG_SID_THIRDPARTY_ANY)
#define CALG_THIRDPARTY_SIGNATURE       (ALG_CLASS_SIGNATURE    | ALG_TYPE_THIRDPARTY | ALG_SID_THIRDPARTY_ANY)
#define CALG_THIRDPARTY_CIPHER          (ALG_CLASS_DATA_ENCRYPT | ALG_TYPE_THIRDPARTY | ALG_SID_THIRDPARTY_ANY)
#define CALG_THIRDPARTY_HASH            (ALG_CLASS_HASH         | ALG_TYPE_THIRDPARTY | ALG_SID_THIRDPARTY_ANY)
#endif //(NTDDI_VERSION >= NTDDI_WIN10_RS1)


#if (NTDDI_VERSION < NTDDI_WINXP)
// resource number for signatures in the CSP
#define SIGNATURE_RESOURCE_NUMBER       0x29A

typedef struct _VTableProvStruc {
    DWORD   Version;
    FARPROC FuncVerifyImage;
    FARPROC FuncReturnhWnd;
    DWORD   dwProvType;
    BYTE        *pbContextInfo;
    DWORD       cbContextInfo;
    LPSTR   pszProvName;
} VTableProvStruc, *PVTableProvStruc;
#endif //(NTDDI_VERSION < NTDDI_WINXP)

// Used for certenroll.idl:
// certenrolls_begin -- HCRYPT*
#ifndef HCRYPTPROV_DEFINED
#define HCRYPTPROV_DEFINED
typedef ULONG_PTR HCRYPTPROV;
typedef ULONG_PTR HCRYPTKEY;
typedef ULONG_PTR HCRYPTHASH;
#endif
// certenrolls_end

// dwFlags definitions for CryptAcquireContext
#define CRYPT_VERIFYCONTEXT     0xF0000000
#define CRYPT_NEWKEYSET         0x00000008
#define CRYPT_DELETEKEYSET      0x00000010
#define CRYPT_MACHINE_KEYSET    0x00000020
#define CRYPT_SILENT            0x00000040
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define CRYPT_DEFAULT_CONTAINER_OPTIONAL 0x00000080
#endif //(NTDDI_VERSION >= NTDDI_VISTA)

// dwFlag definitions for CryptGenKey
#define CRYPT_EXPORTABLE        0x00000001
#define CRYPT_USER_PROTECTED    0x00000002
#define CRYPT_CREATE_SALT       0x00000004
#define CRYPT_UPDATE_KEY        0x00000008
#define CRYPT_NO_SALT           0x00000010
#define CRYPT_PREGEN            0x00000040
#define CRYPT_RECIPIENT         0x00000010
#define CRYPT_INITIATOR         0x00000040
#define CRYPT_ONLINE            0x00000080
#define CRYPT_SF                0x00000100
#define CRYPT_CREATE_IV         0x00000200
#define CRYPT_KEK               0x00000400
#define CRYPT_DATA_KEY          0x00000800
#define CRYPT_VOLATILE          0x00001000
#define CRYPT_SGCKEY            0x00002000
//PKCS12_ALLOW_OVERWRITE_KEY 0x00004000
//PKCS12_NO_PERSIST_KEY 0x00008000
//should use other than these two
#define CRYPT_USER_PROTECTED_STRONG    0x00100000
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define CRYPT_ARCHIVABLE        0x00004000
#endif //(NTDDI_VERSION >= NTDDI_WINXP)
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define CRYPT_FORCE_KEY_PROTECTION_HIGH 0x00008000
#endif //(NTDDI_VERSION >= NTDDI_VISTA)

#define RSA1024BIT_KEY          0x04000000

// dwFlags definitions for CryptDeriveKey
#define CRYPT_SERVER            0x00000400

#define KEY_LENGTH_MASK         0xFFFF0000

// dwFlag definitions for CryptExportKey
#define CRYPT_Y_ONLY            0x00000001
#define CRYPT_SSL2_FALLBACK     0x00000002
#define CRYPT_DESTROYKEY        0x00000004
#define CRYPT_OAEP              0x00000040  // used with RSA encryptions/decryptions
                                            // CryptExportKey, CryptImportKey,
                                            // CryptEncrypt and CryptDecrypt

#define CRYPT_BLOB_VER3         0x00000080  // export version 3 of a blob type
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define CRYPT_IPSEC_HMAC_KEY    0x00000100  // CryptImportKey only
#endif //(NTDDI_VERSION >= NTDDI_WINXP)

#if (NTDDI_VERSION >= NTDDI_WS03)
// dwFlags definitions for CryptDecrypt
//  See also CRYPT_OAEP, above.
//  Note, the following flag is not supported for CryptEncrypt
#define CRYPT_DECRYPT_RSA_NO_PADDING_CHECK      0x00000020
#endif //(NTDDI_VERSION >= NTDDI_WS03)

// dwFlags definitions for CryptCreateHash
#define CRYPT_SECRETDIGEST      0x00000001

#if (NTDDI_VERSION >= NTDDI_WINXP)
// dwFlags definitions for CryptHashData
#define CRYPT_OWF_REPL_LM_HASH  0x00000001  // this is only for the OWF replacement CSP
#endif //(NTDDI_VERSION >= NTDDI_WINXP)

// dwFlags definitions for CryptHashSessionKey
#define CRYPT_LITTLE_ENDIAN     0x00000001

// dwFlags definitions for CryptSignHash and CryptVerifySignature
#define CRYPT_NOHASHOID         0x00000001
#define CRYPT_TYPE2_FORMAT      0x00000002  // Not supported
#define CRYPT_X931_FORMAT       0x00000004  // Not supported

// dwFlag definitions for CryptSetProviderEx and CryptGetDefaultProvider
#define CRYPT_MACHINE_DEFAULT   0x00000001
#define CRYPT_USER_DEFAULT      0x00000002
#define CRYPT_DELETE_DEFAULT    0x00000004

// exported key blob definitions
// certenrolld_begin -- *BLOB
#define SIMPLEBLOB              0x1
#define PUBLICKEYBLOB           0x6
#define PRIVATEKEYBLOB          0x7
#define PLAINTEXTKEYBLOB        0x8
#define OPAQUEKEYBLOB           0x9
#define PUBLICKEYBLOBEX         0xA
#define SYMMETRICWRAPKEYBLOB    0xB
#if (NTDDI_VERSION >= NTDDI_WS03)
#define KEYSTATEBLOB            0xC
#endif //(NTDDI_VERSION >= NTDDI_WS03)
// certenrolld_end

// certenrolld_begin -- AT_*
#define AT_KEYEXCHANGE          1
#define AT_SIGNATURE            2
// certenrolld_end

#define CRYPT_USERDATA          1

// dwParam
#define KP_IV                   1       // Initialization vector
#define KP_SALT                 2       // Salt value
#define KP_PADDING              3       // Padding values
#define KP_MODE                 4       // Mode of the cipher
#define KP_MODE_BITS            5       // Number of bits to feedback
#define KP_PERMISSIONS          6       // Key permissions DWORD
#define KP_ALGID                7       // Key algorithm
#define KP_BLOCKLEN             8       // Block size of the cipher
#define KP_KEYLEN               9       // Length of key in bits
#define KP_SALT_EX              10      // Length of salt in bytes
#define KP_P                    11      // DSS/Diffie-Hellman P value
#define KP_G                    12      // DSS/Diffie-Hellman G value
#define KP_Q                    13      // DSS Q value
#define KP_X                    14      // Diffie-Hellman X value
#define KP_Y                    15      // Y value
#define KP_RA                   16      // Fortezza RA value
#define KP_RB                   17      // Fortezza RB value
#define KP_INFO                 18      // for putting information into an RSA envelope
#define KP_EFFECTIVE_KEYLEN     19      // setting and getting RC2 effective key length
#define KP_SCHANNEL_ALG         20      // for setting the Secure Channel algorithms
#define KP_CLIENT_RANDOM        21      // for setting the Secure Channel client random data
#define KP_SERVER_RANDOM        22      // for setting the Secure Channel server random data
#define KP_RP                   23
#define KP_PRECOMP_MD5          24
#define KP_PRECOMP_SHA          25
#define KP_CERTIFICATE          26      // for setting Secure Channel certificate data (PCT1)
#define KP_CLEAR_KEY            27      // for setting Secure Channel clear key data (PCT1)
#define KP_PUB_EX_LEN           28
#define KP_PUB_EX_VAL           29
#define KP_KEYVAL               30
#define KP_ADMIN_PIN            31
#define KP_KEYEXCHANGE_PIN      32
#define KP_SIGNATURE_PIN        33
#define KP_PREHASH              34
#if (NTDDI_VERSION >= NTDDI_WS03)
#define KP_ROUNDS               35
#endif //(NTDDI_VERSION >= NTDDI_WS03)
#define KP_OAEP_PARAMS          36      // for setting OAEP params on RSA keys
#define KP_CMS_KEY_INFO         37
#define KP_CMS_DH_KEY_INFO      38
#define KP_PUB_PARAMS           39      // for setting public parameters
#define KP_VERIFY_PARAMS        40      // for verifying DSA and DH parameters
#define KP_HIGHEST_VERSION      41      // for TLS protocol version setting
#if (NTDDI_VERSION >= NTDDI_WS03)
#define KP_GET_USE_COUNT        42      // for use with PP_CRYPT_COUNT_KEY_USE contexts
#endif //(NTDDI_VERSION >= NTDDI_WS03)
#define KP_PIN_ID               43
#define KP_PIN_INFO             44

// KP_PADDING
#define PKCS5_PADDING           1       // PKCS 5 (sec 6.2) padding method
#define RANDOM_PADDING          2
#define ZERO_PADDING            3

// KP_MODE
#define CRYPT_MODE_CBC          1       // Cipher block chaining
#define CRYPT_MODE_ECB          2       // Electronic code book
#define CRYPT_MODE_OFB          3       // Output feedback mode
#define CRYPT_MODE_CFB          4       // Cipher feedback mode
#define CRYPT_MODE_CTS          5       // Ciphertext stealing mode

// KP_PERMISSIONS
#define CRYPT_ENCRYPT           0x0001  // Allow encryption
#define CRYPT_DECRYPT           0x0002  // Allow decryption
#define CRYPT_EXPORT            0x0004  // Allow key to be exported
#define CRYPT_READ              0x0008  // Allow parameters to be read
#define CRYPT_WRITE             0x0010  // Allow parameters to be set
#define CRYPT_MAC               0x0020  // Allow MACs to be used with key
#define CRYPT_EXPORT_KEY        0x0040  // Allow key to be used for exporting keys
#define CRYPT_IMPORT_KEY        0x0080  // Allow key to be used for importing keys
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define CRYPT_ARCHIVE           0x0100  // Allow key to be exported at creation only
#endif //(NTDDI_VERSION >= NTDDI_WINXP)

#define HP_ALGID                0x0001  // Hash algorithm
#define HP_HASHVAL              0x0002  // Hash value
#define HP_HASHSIZE             0x0004  // Hash value size
#define HP_HMAC_INFO            0x0005  // information for creating an HMAC
#define HP_TLS1PRF_LABEL        0x0006  // label for TLS1 PRF
#define HP_TLS1PRF_SEED         0x0007  // seed for TLS1 PRF

#define CRYPT_FAILED            FALSE
#define CRYPT_SUCCEED           TRUE

#define RCRYPT_SUCCEEDED(rt)     ((rt) == CRYPT_SUCCEED)
#define RCRYPT_FAILED(rt)        ((rt) == CRYPT_FAILED)

//
// CryptGetProvParam
//
#define PP_ENUMALGS             1
#define PP_ENUMCONTAINERS       2
#define PP_IMPTYPE              3
#define PP_NAME                 4
#define PP_VERSION              5
#define PP_CONTAINER            6
#define PP_CHANGE_PASSWORD      7
#define PP_KEYSET_SEC_DESCR     8       // get/set security descriptor of keyset
#define PP_CERTCHAIN            9       // for retrieving certificates from tokens
#define PP_KEY_TYPE_SUBTYPE     10
#define PP_PROVTYPE             16
#define PP_KEYSTORAGE           17
#define PP_APPLI_CERT           18
#define PP_SYM_KEYSIZE          19
#define PP_SESSION_KEYSIZE      20
#define PP_UI_PROMPT            21
#define PP_ENUMALGS_EX          22
#define PP_ENUMMANDROOTS        25
#define PP_ENUMELECTROOTS       26
#define PP_KEYSET_TYPE          27
#define PP_ADMIN_PIN            31
#define PP_KEYEXCHANGE_PIN      32
#define PP_SIGNATURE_PIN        33
#define PP_SIG_KEYSIZE_INC      34
#define PP_KEYX_KEYSIZE_INC     35
#define PP_UNIQUE_CONTAINER     36
#define PP_SGC_INFO             37
#define PP_USE_HARDWARE_RNG     38
#define PP_KEYSPEC              39
#define PP_ENUMEX_SIGNING_PROT  40
#if (NTDDI_VERSION >= NTDDI_WS03)
#define PP_CRYPT_COUNT_KEY_USE  41
#endif //(NTDDI_VERSION >= NTDDI_WS03)
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define PP_USER_CERTSTORE       42
#define PP_SMARTCARD_READER     43
#define PP_SMARTCARD_GUID       45
#define PP_ROOT_CERTSTORE       46
#endif //(NTDDI_VERSION >= NTDDI_VISTA)
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define PP_SMARTCARD_READER_ICON	47
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#define CRYPT_FIRST             1
#define CRYPT_NEXT              2
#define CRYPT_SGC_ENUM          4

#define CRYPT_IMPL_HARDWARE     1
#define CRYPT_IMPL_SOFTWARE     2
#define CRYPT_IMPL_MIXED        3
#define CRYPT_IMPL_UNKNOWN      4
#define CRYPT_IMPL_REMOVABLE    8

// key storage flags
#define CRYPT_SEC_DESCR         0x00000001
#define CRYPT_PSTORE            0x00000002
#define CRYPT_UI_PROMPT         0x00000004

// protocol flags
#define CRYPT_FLAG_PCT1         0x0001
#define CRYPT_FLAG_SSL2         0x0002
#define CRYPT_FLAG_SSL3         0x0004
#define CRYPT_FLAG_TLS1         0x0008
#define CRYPT_FLAG_IPSEC        0x0010
#define CRYPT_FLAG_SIGNING      0x0020

// SGC flags
#define CRYPT_SGC               0x0001
#define CRYPT_FASTSGC           0x0002

//
// CryptSetProvParam
//
#define PP_CLIENT_HWND          1
#define PP_CONTEXT_INFO         11
#define PP_KEYEXCHANGE_KEYSIZE  12
#define PP_SIGNATURE_KEYSIZE    13
#define PP_KEYEXCHANGE_ALG      14
#define PP_SIGNATURE_ALG        15
#define PP_DELETEKEY            24
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define PP_PIN_PROMPT_STRING      44
#define PP_SECURE_KEYEXCHANGE_PIN 47
#define PP_SECURE_SIGNATURE_PIN   48
#endif //(NTDDI_VERSION >= NTDDI_VISTA)
#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
#define PP_DISMISS_PIN_UI_SEC   49
#define PP_IS_PFX_EPHEMERAL     50
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5)

// certenrolld_begin -- PROV_RSA_*
#define PROV_RSA_FULL           1
#define PROV_RSA_SIG            2
#define PROV_DSS                3
#define PROV_FORTEZZA           4
#define PROV_MS_EXCHANGE        5
#define PROV_SSL                6
#define PROV_RSA_SCHANNEL       12
#define PROV_DSS_DH             13
#define PROV_EC_ECDSA_SIG       14
#define PROV_EC_ECNRA_SIG       15
#define PROV_EC_ECDSA_FULL      16
#define PROV_EC_ECNRA_FULL      17
#define PROV_DH_SCHANNEL        18
#define PROV_SPYRUS_LYNKS       20
#define PROV_RNG                21
#define PROV_INTEL_SEC          22
#if (NTDDI_VERSION >= NTDDI_WINXP)
#define PROV_REPLACE_OWF        23
#define PROV_RSA_AES            24
#endif //(NTDDI_VERSION >= NTDDI_WINXP)
// certenrolld_end

#if (NTDDI_VERSION <= NTDDI_WINXP)
//
// STT defined Providers
//
#define PROV_STT_MER            7
#define PROV_STT_ACQ            8
#define PROV_STT_BRND           9
#define PROV_STT_ROOT           10
#define PROV_STT_ISS            11
#endif //(NTDDI_VERSION <= NTDDI_WINXP)

//
// Provider friendly names
//
#define MS_DEF_PROV_A           "Microsoft Base Cryptographic Provider v1.0"
#define MS_DEF_PROV_W           L"Microsoft Base Cryptographic Provider v1.0"
#ifdef UNICODE
#define MS_DEF_PROV             MS_DEF_PROV_W
#else
#define MS_DEF_PROV             MS_DEF_PROV_A
#endif

#define MS_ENHANCED_PROV_A      "Microsoft Enhanced Cryptographic Provider v1.0"
#define MS_ENHANCED_PROV_W      L"Microsoft Enhanced Cryptographic Provider v1.0"
#ifdef UNICODE
#define MS_ENHANCED_PROV        MS_ENHANCED_PROV_W
#else
#define MS_ENHANCED_PROV        MS_ENHANCED_PROV_A
#endif

#define MS_STRONG_PROV_A        "Microsoft Strong Cryptographic Provider"
#define MS_STRONG_PROV_W        L"Microsoft Strong Cryptographic Provider"
#ifdef UNICODE
#define MS_STRONG_PROV          MS_STRONG_PROV_W
#else
#define MS_STRONG_PROV          MS_STRONG_PROV_A
#endif

#define MS_DEF_RSA_SIG_PROV_A   "Microsoft RSA Signature Cryptographic Provider"
#define MS_DEF_RSA_SIG_PROV_W   L"Microsoft RSA Signature Cryptographic Provider"
#ifdef UNICODE
#define MS_DEF_RSA_SIG_PROV     MS_DEF_RSA_SIG_PROV_W
#else
#define MS_DEF_RSA_SIG_PROV     MS_DEF_RSA_SIG_PROV_A
#endif

#define MS_DEF_RSA_SCHANNEL_PROV_A  "Microsoft RSA SChannel Cryptographic Provider"
#define MS_DEF_RSA_SCHANNEL_PROV_W  L"Microsoft RSA SChannel Cryptographic Provider"
#ifdef UNICODE
#define MS_DEF_RSA_SCHANNEL_PROV    MS_DEF_RSA_SCHANNEL_PROV_W
#else
#define MS_DEF_RSA_SCHANNEL_PROV    MS_DEF_RSA_SCHANNEL_PROV_A
#endif

#define MS_DEF_DSS_PROV_A       "Microsoft Base DSS Cryptographic Provider"
#define MS_DEF_DSS_PROV_W       L"Microsoft Base DSS Cryptographic Provider"
#ifdef UNICODE
#define MS_DEF_DSS_PROV         MS_DEF_DSS_PROV_W
#else
#define MS_DEF_DSS_PROV         MS_DEF_DSS_PROV_A
#endif

#define MS_DEF_DSS_DH_PROV_A    "Microsoft Base DSS and Diffie-Hellman Cryptographic Provider"
#define MS_DEF_DSS_DH_PROV_W    L"Microsoft Base DSS and Diffie-Hellman Cryptographic Provider"
#ifdef UNICODE
#define MS_DEF_DSS_DH_PROV      MS_DEF_DSS_DH_PROV_W
#else
#define MS_DEF_DSS_DH_PROV      MS_DEF_DSS_DH_PROV_A
#endif

#define MS_ENH_DSS_DH_PROV_A    "Microsoft Enhanced DSS and Diffie-Hellman Cryptographic Provider"
#define MS_ENH_DSS_DH_PROV_W    L"Microsoft Enhanced DSS and Diffie-Hellman Cryptographic Provider"
#ifdef UNICODE
#define MS_ENH_DSS_DH_PROV      MS_ENH_DSS_DH_PROV_W
#else
#define MS_ENH_DSS_DH_PROV      MS_ENH_DSS_DH_PROV_A
#endif

#define MS_DEF_DH_SCHANNEL_PROV_A  "Microsoft DH SChannel Cryptographic Provider"
#define MS_DEF_DH_SCHANNEL_PROV_W  L"Microsoft DH SChannel Cryptographic Provider"
#ifdef UNICODE
#define MS_DEF_DH_SCHANNEL_PROV MS_DEF_DH_SCHANNEL_PROV_W
#else
#define MS_DEF_DH_SCHANNEL_PROV MS_DEF_DH_SCHANNEL_PROV_A
#endif

#define MS_SCARD_PROV_A         "Microsoft Base Smart Card Crypto Provider"
#define MS_SCARD_PROV_W         L"Microsoft Base Smart Card Crypto Provider"
#ifdef UNICODE
#define MS_SCARD_PROV           MS_SCARD_PROV_W
#else
#define MS_SCARD_PROV           MS_SCARD_PROV_A
#endif

#if (NTDDI_VERSION >= NTDDI_WINXP)
#define MS_ENH_RSA_AES_PROV_A   "Microsoft Enhanced RSA and AES Cryptographic Provider"
#define MS_ENH_RSA_AES_PROV_W   L"Microsoft Enhanced RSA and AES Cryptographic Provider"
#define MS_ENH_RSA_AES_PROV_XP_A "Microsoft Enhanced RSA and AES Cryptographic Provider (Prototype)"
#define MS_ENH_RSA_AES_PROV_XP_W L"Microsoft Enhanced RSA and AES Cryptographic Provider (Prototype)"
#ifdef UNICODE
#define MS_ENH_RSA_AES_PROV_XP  MS_ENH_RSA_AES_PROV_XP_W
#define MS_ENH_RSA_AES_PROV     MS_ENH_RSA_AES_PROV_W
#else
#define MS_ENH_RSA_AES_PROV_XP  MS_ENH_RSA_AES_PROV_XP_A
#define MS_ENH_RSA_AES_PROV     MS_ENH_RSA_AES_PROV_A
#endif
#endif //(NTDDI_VERSION >= NTDDI_WINXP)

#define MAXUIDLEN               64

// Exponentiation Offload Reg Location
#define EXPO_OFFLOAD_REG_VALUE "ExpoOffload"
#define EXPO_OFFLOAD_FUNC_NAME "OffloadModExpo"

//
// Registry key in which the following private key-related
// values are created.
//
#ifndef szKEY_CRYPTOAPI_PRIVATE_KEY_OPTIONS
#define szKEY_CRYPTOAPI_PRIVATE_KEY_OPTIONS \
    "Software\\Policies\\Microsoft\\Cryptography"
#endif

//
// Registry values for enabling and controlling the caching (and timeout)
// of private keys.  This feature is intended for UI-protected private
// keys.
//
// Note that in Windows 2000 and later, private keys, once read from storage,
// are cached in the associated HCRYPTPROV structure for subsequent use.
//
// In Server 2003 and XP SP1, new key caching behavior is available.  Keys
// that have been read from storage and cached may now be considered "stale"
// if a period of time has elapsed since the key was last used.  This forces
// the key to be re-read from storage (which will make the DPAPI UI appear
// again).
//
// Optional Key Timeouts:
//
// In Windows Server 2003, XP SP1, and later, new key caching behavior is
// available.  Keys that have been read from storage and cached per-context
// may now be considered "stale" if a period of time has elapsed since the
// key was last used.  This forces the key to be re-read from storage (which
// will make the Data Protection API dialog appear again if the key is
// UI-protected).
//
// To enable the new behavior, create the registry DWORD value
// szKEY_CACHE_ENABLED and set it to 1.  The registry DWORD value
// szKEY_CACHE_SECONDS must also be created and set to the number of seconds
// that a cached private key may still be considered usable.
//
#define szKEY_CACHE_ENABLED                     "CachePrivateKeys"
#define szKEY_CACHE_SECONDS                     "PrivateKeyLifetimeSeconds"

#if (NTDDI_VERSION >= NTDDI_WINXP)
//
// In platforms later than (and not including) Windows Server 2003, private
// keys are always cached for a period of time per-process, even when
// not being used in any context.
//
// The differences between the process-wide caching settings described below
// and the Optional Key Timeouts described above are subtle.
//
//  - The Optional Key Timeout policy is applied only when an attempt is made
//    to use a specific private key with an open context handle (HCRYPTPROV).
//    If szKEY_CACHE_SECONDS have elapsed since the key was last used, the
//    private key will be re-read from storage.
//
//  - The Cache Purge Interval policy, below, is applied whenever any
//    non-ephemeral private key is used or read from storage.  If
//    szPRIV_KEY_CACHE_PURGE_INTERVAL_SECONDS have elapsed since the last
//    purge occurred, all cached keys that have not been referenced since the
//    last purge will be removed from the cache.
//
//    If a private key that is purged from the cache is currently
//    referenced in an open context, then the key will be re-read from storage
//    the next time an attempt is made to use it (via any context).
//
// The following two registry DWORD values control this behavior.
//

//
// Registry value for controlling the maximum number of persisted
// (non-ephemeral) private keys that can be cached per-process.  If the cache
// fills up, keys will be replaced on a least-recently-used basis.  If the
// maximum number of cached keys is set to zero, no keys will be globally
// cached.
//
#define szPRIV_KEY_CACHE_MAX_ITEMS              "PrivKeyCacheMaxItems"
#define cPRIV_KEY_CACHE_MAX_ITEMS_DEFAULT       20

//
// Registry value for controlling the interval at which the private key
// cache is proactively purged of outdated keys.
//
#define szPRIV_KEY_CACHE_PURGE_INTERVAL_SECONDS "PrivKeyCachePurgeIntervalSeconds"
#define cPRIV_KEY_CACHE_PURGE_INTERVAL_SECONDS_DEFAULT 86400 // 1 day
#endif //(NTDDI_VERSION >= NTDDI_WINXP)

#define CUR_BLOB_VERSION        2

// structure for use with CryptSetKeyParam for CMS keys
// DO NOT USE THIS STRUCTURE!!!!!
typedef struct _CMS_KEY_INFO {
    DWORD       dwVersion;                      // sizeof(CMS_KEY_INFO)
    ALG_ID  Algid;                              // algorithmm id for the key to be converted
    BYTE    *pbOID;                             // pointer to OID to hash in with Z
    DWORD   cbOID;                              // length of OID to hash in with Z
} CMS_KEY_INFO, *PCMS_KEY_INFO;

// structure for use with CryptSetHashParam with CALG_HMAC
typedef struct _HMAC_Info {
    ALG_ID  HashAlgid;
    BYTE    *pbInnerString;
    DWORD   cbInnerString;
    BYTE    *pbOuterString;
    DWORD   cbOuterString;
} HMAC_INFO, *PHMAC_INFO;

// structure for use with CryptSetKeyParam with KP_SCHANNEL_ALG
typedef struct _SCHANNEL_ALG {
    DWORD   dwUse;
    ALG_ID  Algid;
    DWORD   cBits;
    DWORD   dwFlags;
    DWORD   dwReserved;
} SCHANNEL_ALG, *PSCHANNEL_ALG;

// uses of algortihms for SCHANNEL_ALG structure
#define     SCHANNEL_MAC_KEY    0x00000000
#define     SCHANNEL_ENC_KEY    0x00000001

// uses of dwFlags SCHANNEL_ALG structure
#define     INTERNATIONAL_USAGE 0x00000001

typedef struct _PROV_ENUMALGS {
    ALG_ID    aiAlgid;
    DWORD     dwBitLen;
    DWORD     dwNameLen;
    CHAR      szName[20];
} PROV_ENUMALGS;

// certenrolls_begin -- PROV_ENUMALGS_EX
typedef struct _PROV_ENUMALGS_EX {
    ALG_ID    aiAlgid;
    DWORD     dwDefaultLen;
    DWORD     dwMinLen;
    DWORD     dwMaxLen;
    DWORD     dwProtocols;
    DWORD     dwNameLen;
    CHAR      szName[20];
    DWORD     dwLongNameLen;
    CHAR      szLongName[40];
} PROV_ENUMALGS_EX;
// certenrolls_end

typedef struct _PUBLICKEYSTRUC {
        BYTE    bType;
        BYTE    bVersion;
        WORD    reserved;
        ALG_ID  aiKeyAlg;
} BLOBHEADER, PUBLICKEYSTRUC;

typedef struct _RSAPUBKEY {
        DWORD   magic;                  // Has to be RSA1
        DWORD   bitlen;                 // # of bits in modulus
        DWORD   pubexp;                 // public exponent
                                        // Modulus data follows
} RSAPUBKEY;

typedef struct _PUBKEY {
        DWORD   magic;
        DWORD   bitlen;                 // # of bits in modulus
} DHPUBKEY, DSSPUBKEY, KEAPUBKEY, TEKPUBKEY;

typedef struct _DSSSEED {
        DWORD   counter;
        BYTE    seed[20];
} DSSSEED;

typedef struct _PUBKEYVER3 {
        DWORD   magic;
        DWORD   bitlenP;                // # of bits in prime modulus
        DWORD   bitlenQ;                // # of bits in prime q, 0 if not available
        DWORD   bitlenJ;                // # of bits in (p-1)/q, 0 if not available
        DSSSEED DSSSeed;
} DHPUBKEY_VER3, DSSPUBKEY_VER3;

typedef struct _PRIVKEYVER3 {
        DWORD   magic;
        DWORD   bitlenP;                // # of bits in prime modulus
        DWORD   bitlenQ;                // # of bits in prime q, 0 if not available
        DWORD   bitlenJ;                // # of bits in (p-1)/q, 0 if not available
        DWORD   bitlenX;                // # of bits in X
        DSSSEED DSSSeed;
} DHPRIVKEY_VER3, DSSPRIVKEY_VER3;

typedef struct _KEY_TYPE_SUBTYPE {
        DWORD   dwKeySpec;
        GUID    Type;
        GUID    Subtype;
} KEY_TYPE_SUBTYPE, *PKEY_TYPE_SUBTYPE;

typedef struct _CERT_FORTEZZA_DATA_PROP {
    unsigned char   SerialNumber[8];
    int             CertIndex;
    unsigned char   CertLabel[36];
} CERT_FORTEZZA_DATA_PROP;

#if (NTDDI_VERSION >= NTDDI_WS03)
typedef struct _CRYPT_RC4_KEY_STATE {
    unsigned char Key[16];
    unsigned char SBox[256];
    unsigned char i;
    unsigned char j;
} CRYPT_RC4_KEY_STATE, *PCRYPT_RC4_KEY_STATE;

typedef struct _CRYPT_DES_KEY_STATE {
    unsigned char Key[8];
    unsigned char IV[8];
    unsigned char Feedback[8];
} CRYPT_DES_KEY_STATE, *PCRYPT_DES_KEY_STATE;

typedef struct _CRYPT_3DES_KEY_STATE {
    unsigned char Key[24];
    unsigned char IV[8];
    unsigned char Feedback[8];
} CRYPT_3DES_KEY_STATE, *PCRYPT_3DES_KEY_STATE;
#endif //(NTDDI_VERSION >= NTDDI_WS03)

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef struct _CRYPT_AES_128_KEY_STATE {
    unsigned char Key[16];
    unsigned char IV[16];
    unsigned char EncryptionState[11][16];      // 10 rounds + 1
    unsigned char DecryptionState[11][16];
    unsigned char Feedback[16];
} CRYPT_AES_128_KEY_STATE, *PCRYPT_AES_128_KEY_STATE;

typedef struct _CRYPT_AES_256_KEY_STATE {
    unsigned char Key[32];
    unsigned char IV[16];
    unsigned char EncryptionState[15][16];      // 14 rounds + 1
    unsigned char DecryptionState[15][16];
    unsigned char Feedback[16];
} CRYPT_AES_256_KEY_STATE, *PCRYPT_AES_256_KEY_STATE;
#endif //(NTDDI_VERSION >= NTDDI_VISTA)

//+-------------------------------------------------------------------------
//  CRYPTOAPI BLOB definitions
//--------------------------------------------------------------------------
// certenrolls_begin -- *_BLOB
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
// certenrolls_end

// structure for use with CryptSetKeyParam for CMS keys
typedef struct _CMS_DH_KEY_INFO {
    DWORD               dwVersion;                      // sizeof(CMS_DH_KEY_INFO)
    ALG_ID          Algid;                              // algorithmm id for the key to be converted
    LPSTR           pszContentEncObjId; // pointer to OID to hash in with Z
    CRYPT_DATA_BLOB PubInfo;            // OPTIONAL - public information
    void            *pReserved;         // reserved - should be NULL
} CMS_DH_KEY_INFO, *PCMS_DH_KEY_INFO;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINADVAPI
BOOL
WINAPI
CryptAcquireContextA(
    _Out_       HCRYPTPROV  *phProv,
    _In_opt_    LPCSTR    szContainer,
    _In_opt_    LPCSTR    szProvider,
    _In_        DWORD       dwProvType,
    _In_        DWORD       dwFlags
    );
WINADVAPI
BOOL
WINAPI
CryptAcquireContextW(
    _Out_       HCRYPTPROV  *phProv,
    _In_opt_    LPCWSTR    szContainer,
    _In_opt_    LPCWSTR    szProvider,
    _In_        DWORD       dwProvType,
    _In_        DWORD       dwFlags
    );
#ifdef UNICODE
#define CryptAcquireContext  CryptAcquireContextW
#else
#define CryptAcquireContext  CryptAcquireContextA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINADVAPI
BOOL
WINAPI
CryptReleaseContext(
    _In_    HCRYPTPROV  hProv,
    _In_    DWORD       dwFlags
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINADVAPI
BOOL
WINAPI
CryptGenKey(
    _In_    HCRYPTPROV  hProv,
    _In_    ALG_ID      Algid,
    _In_    DWORD       dwFlags,
    _Out_   HCRYPTKEY   *phKey
    );

WINADVAPI
BOOL
WINAPI
CryptDeriveKey(
    _In_    HCRYPTPROV  hProv,
    _In_    ALG_ID      Algid,
    _In_    HCRYPTHASH  hBaseData,
    _In_    DWORD       dwFlags,
    _Out_   HCRYPTKEY   *phKey
    );

WINADVAPI
BOOL
WINAPI
CryptDestroyKey(
    _In_    HCRYPTKEY   hKey
    );

WINADVAPI
BOOL
WINAPI
CryptSetKeyParam(
    _In_    HCRYPTKEY   hKey,
    _In_    DWORD       dwParam,
    _In_    CONST BYTE  *pbData,
    _In_    DWORD       dwFlags
    );

WINADVAPI
BOOL
WINAPI
CryptGetKeyParam(
    _In_                                            HCRYPTKEY   hKey,
    _In_                                            DWORD   dwParam,
    _Out_writes_bytes_to_opt_(*pdwDataLen, *pdwDataLen) BYTE    *pbData,
    _Inout_                                         DWORD   *pdwDataLen,
    _In_                                            DWORD   dwFlags
    );

WINADVAPI
BOOL
WINAPI
CryptSetHashParam(
    _In_    HCRYPTHASH  hHash,
    _In_    DWORD       dwParam,
    _In_    CONST BYTE  *pbData,
    _In_    DWORD       dwFlags
    );

WINADVAPI
BOOL
WINAPI
CryptGetHashParam(
    _In_                                        HCRYPTHASH  hHash,
    _In_                                        DWORD   dwParam,
    _Out_writes_bytes_to_opt_(*pdwDataLen, *pdwDataLen) BYTE    *pbData,
    _Inout_                                     DWORD   *pdwDataLen,
    _In_                                        DWORD   dwFlags
    );

WINADVAPI
BOOL
WINAPI
CryptSetProvParam(
    _In_    HCRYPTPROV  hProv,
    _In_    DWORD       dwParam,
    _In_    CONST BYTE  *pbData,
    _In_    DWORD       dwFlags
    );

WINADVAPI
BOOL
WINAPI
CryptGetProvParam(
    _In_                                            HCRYPTPROV  hProv,
    _In_                                            DWORD   dwParam,
    _Out_writes_bytes_to_opt_(*pdwDataLen, *pdwDataLen) BYTE    *pbData,
    _Inout_                                         DWORD   *pdwDataLen,
    _In_                                            DWORD   dwFlags
    );

WINADVAPI
BOOL
WINAPI
CryptGenRandom(
    _In_                    HCRYPTPROV  hProv,
    _In_                    DWORD   dwLen,
    _Inout_updates_bytes_(dwLen)   BYTE    *pbBuffer
    );

WINADVAPI
BOOL
WINAPI
CryptGetUserKey(
    _In_    HCRYPTPROV  hProv,
    _In_    DWORD       dwKeySpec,
    _Out_   HCRYPTKEY   *phUserKey
    );

WINADVAPI
BOOL
WINAPI
CryptExportKey(
    _In_                                            HCRYPTKEY   hKey,
    _In_                                            HCRYPTKEY   hExpKey,
    _In_                                            DWORD   dwBlobType,
    _In_                                            DWORD   dwFlags,
    _Out_writes_bytes_to_opt_(*pdwDataLen, *pdwDataLen) BYTE    *pbData,
    _Inout_                                         DWORD   *pdwDataLen
    );

WINADVAPI
BOOL
WINAPI
CryptImportKey(
    _In_                    HCRYPTPROV  hProv,
    _In_reads_bytes_(dwDataLen)  CONST BYTE  *pbData,
    _In_                    DWORD       dwDataLen,
    _In_                    HCRYPTKEY   hPubKey,
    _In_                    DWORD       dwFlags,
    _Out_                   HCRYPTKEY   *phKey
    );

WINADVAPI
_Success_(0 != return) BOOL
WINAPI
CryptEncrypt(
    _In_                                            HCRYPTKEY   hKey,
    _In_                                            HCRYPTHASH  hHash,
    _In_                                            BOOL    Final,
    _In_                                            DWORD   dwFlags,
    _Inout_updates_bytes_to_opt_(dwBufLen, *pdwDataLen)  BYTE    *pbData,
    _Inout_                                         DWORD   *pdwDataLen,
    _In_                                            DWORD   dwBufLen
    );

WINADVAPI
_Success_(0 != return) BOOL
WINAPI
CryptDecrypt(
    _In_                                            HCRYPTKEY   hKey,
    _In_                                            HCRYPTHASH  hHash,
    _In_                                            BOOL        Final,
    _In_                                            DWORD       dwFlags,
    _Inout_updates_bytes_to_(*pdwDataLen, *pdwDataLen)   BYTE        *pbData,
    _Inout_                                         DWORD       *pdwDataLen
    );

WINADVAPI
BOOL
WINAPI
CryptCreateHash(
    _In_    HCRYPTPROV  hProv,
    _In_    ALG_ID      Algid,
    _In_    HCRYPTKEY   hKey,
    _In_    DWORD       dwFlags,
    _Out_   HCRYPTHASH  *phHash
    );

WINADVAPI
BOOL
WINAPI
CryptHashData(
    _In_                    HCRYPTHASH  hHash,
    _In_reads_bytes_(dwDataLen)  CONST BYTE  *pbData,
    _In_                    DWORD   dwDataLen,
    _In_                    DWORD   dwFlags
    );

WINADVAPI
BOOL
WINAPI
CryptHashSessionKey(
    _In_    HCRYPTHASH  hHash,
    _In_    HCRYPTKEY   hKey,
    _In_    DWORD   dwFlags
    );

WINADVAPI
BOOL
WINAPI
CryptDestroyHash(
    _In_    HCRYPTHASH  hHash
    );

WINADVAPI
BOOL
WINAPI
CryptSignHashA(
    _In_                                          HCRYPTHASH  hHash,
    _In_                                          DWORD       dwKeySpec,
    _In_opt_                                      LPCSTR    szDescription,
    _In_                                          DWORD       dwFlags,
    _Out_writes_bytes_to_opt_(*pdwSigLen, *pdwSigLen) BYTE        *pbSignature,
    _Inout_                                       DWORD       *pdwSigLen
    );
WINADVAPI
BOOL
WINAPI
CryptSignHashW(
    _In_                                          HCRYPTHASH  hHash,
    _In_                                          DWORD       dwKeySpec,
    _In_opt_                                      LPCWSTR    szDescription,
    _In_                                          DWORD       dwFlags,
    _Out_writes_bytes_to_opt_(*pdwSigLen, *pdwSigLen) BYTE        *pbSignature,
    _Inout_                                       DWORD       *pdwSigLen
    );
#ifdef UNICODE
#define CryptSignHash  CryptSignHashW
#else
#define CryptSignHash  CryptSignHashA
#endif // !UNICODE

WINADVAPI
BOOL
WINAPI
CryptVerifySignatureA(
    _In_                    HCRYPTHASH  hHash,
    _In_reads_bytes_(dwSigLen)   CONST BYTE  *pbSignature,
    _In_                    DWORD       dwSigLen,
    _In_                    HCRYPTKEY   hPubKey,
    _In_opt_                LPCSTR    szDescription,
    _In_                    DWORD       dwFlags
    );
WINADVAPI
BOOL
WINAPI
CryptVerifySignatureW(
    _In_                    HCRYPTHASH  hHash,
    _In_reads_bytes_(dwSigLen)   CONST BYTE  *pbSignature,
    _In_                    DWORD       dwSigLen,
    _In_                    HCRYPTKEY   hPubKey,
    _In_opt_                LPCWSTR    szDescription,
    _In_                    DWORD       dwFlags
    );
#ifdef UNICODE
#define CryptVerifySignature  CryptVerifySignatureW
#else
#define CryptVerifySignature  CryptVerifySignatureA
#endif // !UNICODE

WINADVAPI
BOOL
WINAPI
CryptSetProviderA(
    _In_    LPCSTR    pszProvName,
    _In_    DWORD       dwProvType
    );
WINADVAPI
BOOL
WINAPI
CryptSetProviderW(
    _In_    LPCWSTR    pszProvName,
    _In_    DWORD       dwProvType
    );
#ifdef UNICODE
#define CryptSetProvider  CryptSetProviderW
#else
#define CryptSetProvider  CryptSetProviderA
#endif // !UNICODE

WINADVAPI
BOOL
WINAPI
CryptSetProviderExA(
    _In_        LPCSTR pszProvName,
    _In_        DWORD dwProvType,
    _Reserved_  DWORD *pdwReserved,
    _In_        DWORD dwFlags
    );
WINADVAPI
BOOL
WINAPI
CryptSetProviderExW(
    _In_        LPCWSTR pszProvName,
    _In_        DWORD dwProvType,
    _Reserved_  DWORD *pdwReserved,
    _In_        DWORD dwFlags
    );
#ifdef UNICODE
#define CryptSetProviderEx  CryptSetProviderExW
#else
#define CryptSetProviderEx  CryptSetProviderExA
#endif // !UNICODE

WINADVAPI
_Success_(0 != return) BOOL
WINAPI
CryptGetDefaultProviderA(
    _In_                                            DWORD   dwProvType,
    _Reserved_                                      DWORD   *pdwReserved,
    _In_                                            DWORD   dwFlags,
    _Out_writes_bytes_to_opt_(*pcbProvName, *pcbProvName)   LPSTR pszProvName,
    _Inout_                                         DWORD   *pcbProvName
    );
WINADVAPI
_Success_(0 != return) BOOL
WINAPI
CryptGetDefaultProviderW(
    _In_                                            DWORD   dwProvType,
    _Reserved_                                      DWORD   *pdwReserved,
    _In_                                            DWORD   dwFlags,
    _Out_writes_bytes_to_opt_(*pcbProvName, *pcbProvName)   LPWSTR pszProvName,
    _Inout_                                         DWORD   *pcbProvName
    );
#ifdef UNICODE
#define CryptGetDefaultProvider  CryptGetDefaultProviderW
#else
#define CryptGetDefaultProvider  CryptGetDefaultProviderA
#endif // !UNICODE

WINADVAPI
_Success_(0 != return) BOOL
WINAPI
CryptEnumProviderTypesA(
    _In_                                            DWORD   dwIndex,
    _Reserved_                                      DWORD   *pdwReserved,
    _In_                                            DWORD   dwFlags,
    _Out_                                           DWORD   *pdwProvType,
    _Out_writes_bytes_to_opt_(*pcbTypeName, *pcbTypeName)   LPSTR szTypeName,
    _Inout_                                         DWORD   *pcbTypeName
    );
WINADVAPI
_Success_(0 != return) BOOL
WINAPI
CryptEnumProviderTypesW(
    _In_                                            DWORD   dwIndex,
    _Reserved_                                      DWORD   *pdwReserved,
    _In_                                            DWORD   dwFlags,
    _Out_                                           DWORD   *pdwProvType,
    _Out_writes_bytes_to_opt_(*pcbTypeName, *pcbTypeName)   LPWSTR szTypeName,
    _Inout_                                         DWORD   *pcbTypeName
    );
#ifdef UNICODE
#define CryptEnumProviderTypes  CryptEnumProviderTypesW
#else
#define CryptEnumProviderTypes  CryptEnumProviderTypesA
#endif // !UNICODE

WINADVAPI
_Success_(0 != return) BOOL
WINAPI
CryptEnumProvidersA(
    _In_                                                DWORD   dwIndex,
    _Reserved_                                          DWORD   *pdwReserved,
    _In_                                                DWORD   dwFlags,
    _Out_                                               DWORD   *pdwProvType,
    _Out_writes_bytes_to_opt_(*pcbProvName, *pcbProvName)   LPSTR szProvName,
    _Inout_                                             DWORD   *pcbProvName
    );
WINADVAPI
_Success_(0 != return) BOOL
WINAPI
CryptEnumProvidersW(
    _In_                                                DWORD   dwIndex,
    _Reserved_                                          DWORD   *pdwReserved,
    _In_                                                DWORD   dwFlags,
    _Out_                                               DWORD   *pdwProvType,
    _Out_writes_bytes_to_opt_(*pcbProvName, *pcbProvName)   LPWSTR szProvName,
    _Inout_                                             DWORD   *pcbProvName
    );
#ifdef UNICODE
#define CryptEnumProviders  CryptEnumProvidersW
#else
#define CryptEnumProviders  CryptEnumProvidersA
#endif // !UNICODE

WINADVAPI
BOOL
WINAPI
CryptContextAddRef(
    _In_        HCRYPTPROV  hProv,
    _Reserved_  DWORD       *pdwReserved,
    _In_        DWORD       dwFlags
    );

WINADVAPI
BOOL
WINAPI
CryptDuplicateKey(
    _In_        HCRYPTKEY   hKey,
    _Reserved_  DWORD   *pdwReserved,
    _In_        DWORD   dwFlags,
    _Out_       HCRYPTKEY   *phKey
    );

WINADVAPI
BOOL
WINAPI
CryptDuplicateHash(
    _In_        HCRYPTHASH  hHash,
    _Reserved_  DWORD       *pdwReserved,
    _In_        DWORD       dwFlags,
    _Out_       HCRYPTHASH  *phHash
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or Games
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_GAMES)

#if (NTDDI_VERSION >= NTDDI_WS03)
//
// This function is provided in Microsoft Windows 2000 as a means of
// installing the 128-bit encryption provider. This function is unavailable
// in Microsoft Windows XP, because Windows XP ships with the 128-bit
// encryption provider.
//
BOOL
__cdecl
GetEncSChannel(
    _Outptr_result_buffer_(*dwDecSize) BYTE **pData,
    _Out_ DWORD *dwDecSize
    );
#endif //(NTDDI_VERSION >= NTDDI_WS03)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_GAMES) */
#pragma endregion

#if !defined(_DDK_DRIVER_)

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

// In Vista, the following APIs were updated to support the new
// CNG (Cryptography Next Generation) BCrypt* and NCrypt* APIs in addition
// to the above CAPI1 APIs.

// Include the definitions for the CNG APIs
#include <bcrypt.h>

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#include <ncrypt.h>

// This type is used when the API can take either the CAPI1 HCRYPTPROV or
// the CNG NCRYPT_KEY_HANDLE. Where appropriate, the HCRYPTPROV will be
// converted to a NCRYPT_KEY_HANDLE via the CNG NCryptTranslateHandle().
typedef ULONG_PTR HCRYPTPROV_OR_NCRYPT_KEY_HANDLE;

// This type is used where the HCRYPTPROV parameter is no longer used.
// The caller should always pass in NULL.
typedef ULONG_PTR HCRYPTPROV_LEGACY;

//+-------------------------------------------------------------------------
//  In a CRYPT_BIT_BLOB the last byte may contain 0-7 unused bits. Therefore, the
//  overall bit length is cbData * 8 - cUnusedBits.
//--------------------------------------------------------------------------
// certenrolls_begin -- CERT_CONTEXT
typedef struct _CRYPT_BIT_BLOB {
    DWORD   cbData;
    BYTE    *pbData;
    DWORD   cUnusedBits;
} CRYPT_BIT_BLOB, *PCRYPT_BIT_BLOB;

//+-------------------------------------------------------------------------
//  Type used for any algorithm
//
//  Where the Parameters CRYPT_OBJID_BLOB is in its encoded representation. For most
//  algorithm types, the Parameters CRYPT_OBJID_BLOB is NULL (Parameters.cbData = 0).
//--------------------------------------------------------------------------
typedef struct _CRYPT_ALGORITHM_IDENTIFIER {
    LPSTR               pszObjId;
    CRYPT_OBJID_BLOB    Parameters;
} CRYPT_ALGORITHM_IDENTIFIER, *PCRYPT_ALGORITHM_IDENTIFIER;
// certenrolls_end

// Following are the definitions of various algorithm object identifiers
// RSA
#define szOID_RSA               "1.2.840.113549"
#define szOID_PKCS              "1.2.840.113549.1"
#define szOID_RSA_HASH          "1.2.840.113549.2"
#define szOID_RSA_ENCRYPT       "1.2.840.113549.3"

#define szOID_PKCS_1            "1.2.840.113549.1.1"
#define szOID_PKCS_2            "1.2.840.113549.1.2"
#define szOID_PKCS_3            "1.2.840.113549.1.3"
#define szOID_PKCS_4            "1.2.840.113549.1.4"
#define szOID_PKCS_5            "1.2.840.113549.1.5"
#define szOID_PKCS_6            "1.2.840.113549.1.6"
#define szOID_PKCS_7            "1.2.840.113549.1.7"
#define szOID_PKCS_8            "1.2.840.113549.1.8"
#define szOID_PKCS_9            "1.2.840.113549.1.9"
#define szOID_PKCS_10           "1.2.840.113549.1.10"
#define szOID_PKCS_12           "1.2.840.113549.1.12"

#define szOID_RSA_RSA           "1.2.840.113549.1.1.1"
#define szOID_RSA_MD2RSA        "1.2.840.113549.1.1.2"
#define szOID_RSA_MD4RSA        "1.2.840.113549.1.1.3"
#define szOID_RSA_MD5RSA        "1.2.840.113549.1.1.4"
#define szOID_RSA_SHA1RSA       "1.2.840.113549.1.1.5"
#define szOID_RSA_SETOAEP_RSA   "1.2.840.113549.1.1.6"

#define szOID_RSAES_OAEP        "1.2.840.113549.1.1.7"
#define szOID_RSA_MGF1          "1.2.840.113549.1.1.8"
#define szOID_RSA_PSPECIFIED    "1.2.840.113549.1.1.9"
#define szOID_RSA_SSA_PSS       "1.2.840.113549.1.1.10"
#define szOID_RSA_SHA256RSA     "1.2.840.113549.1.1.11"
#define szOID_RSA_SHA384RSA     "1.2.840.113549.1.1.12"
#define szOID_RSA_SHA512RSA     "1.2.840.113549.1.1.13"

#define szOID_RSA_DH            "1.2.840.113549.1.3.1"

#define szOID_RSA_data          "1.2.840.113549.1.7.1"
#define szOID_RSA_signedData    "1.2.840.113549.1.7.2"
#define szOID_RSA_envelopedData "1.2.840.113549.1.7.3"
#define szOID_RSA_signEnvData   "1.2.840.113549.1.7.4"
#define szOID_RSA_digestedData  "1.2.840.113549.1.7.5"
#define szOID_RSA_hashedData    "1.2.840.113549.1.7.5"
#define szOID_RSA_encryptedData "1.2.840.113549.1.7.6"

#define szOID_RSA_emailAddr     "1.2.840.113549.1.9.1"
#define szOID_RSA_unstructName  "1.2.840.113549.1.9.2"
#define szOID_RSA_contentType   "1.2.840.113549.1.9.3"
#define szOID_RSA_messageDigest "1.2.840.113549.1.9.4"
#define szOID_RSA_signingTime   "1.2.840.113549.1.9.5"
#define szOID_RSA_counterSign   "1.2.840.113549.1.9.6"
#define szOID_RSA_challengePwd  "1.2.840.113549.1.9.7"
#define szOID_RSA_unstructAddr  "1.2.840.113549.1.9.8"
#define szOID_RSA_extCertAttrs  "1.2.840.113549.1.9.9"
#define szOID_RSA_certExtensions "1.2.840.113549.1.9.14"
#define szOID_RSA_SMIMECapabilities "1.2.840.113549.1.9.15"
#define szOID_RSA_preferSignedData "1.2.840.113549.1.9.15.1"

#define szOID_TIMESTAMP_TOKEN           "1.2.840.113549.1.9.16.1.4"
#define szOID_RFC3161_counterSign "1.3.6.1.4.1.311.3.3.1"
#define szOID_RFC3161v21_counterSign "1.3.6.1.4.1.311.3.3.2"
#define szOID_RFC3161v21_thumbprints "1.3.6.1.4.1.311.3.3.3"

#define szOID_RSA_SMIMEalg              "1.2.840.113549.1.9.16.3"
#define szOID_RSA_SMIMEalgESDH          "1.2.840.113549.1.9.16.3.5"
#define szOID_RSA_SMIMEalgCMS3DESwrap   "1.2.840.113549.1.9.16.3.6"
#define szOID_RSA_SMIMEalgCMSRC2wrap    "1.2.840.113549.1.9.16.3.7"

#define szOID_RSA_MD2           "1.2.840.113549.2.2"
#define szOID_RSA_MD4           "1.2.840.113549.2.4"
#define szOID_RSA_MD5           "1.2.840.113549.2.5"

#define szOID_RSA_RC2CBC        "1.2.840.113549.3.2"
#define szOID_RSA_RC4           "1.2.840.113549.3.4"
#define szOID_RSA_DES_EDE3_CBC  "1.2.840.113549.3.7"
#define szOID_RSA_RC5_CBCPad    "1.2.840.113549.3.9"


#define szOID_ANSI_X942         "1.2.840.10046"
#define szOID_ANSI_X942_DH      "1.2.840.10046.2.1"

#define szOID_X957              "1.2.840.10040"
#define szOID_X957_DSA          "1.2.840.10040.4.1"
#define szOID_X957_SHA1DSA      "1.2.840.10040.4.3"


// iso(1) member-body(2) us(840) 10045 keyType(2) unrestricted(1)
#define szOID_ECC_PUBLIC_KEY    "1.2.840.10045.2.1"

// iso(1) member-body(2) us(840) 10045 curves(3) prime(1) 7
#define szOID_ECC_CURVE_P256    "1.2.840.10045.3.1.7"

// iso(1) identified-organization(3) certicom(132) curve(0) 34
#define szOID_ECC_CURVE_P384    "1.3.132.0.34"

// iso(1) identified-organization(3) certicom(132) curve(0) 35
#define szOID_ECC_CURVE_P521    "1.3.132.0.35"


//
// Generic ECC Curve OIDS
//
#define szOID_ECC_CURVE_BRAINPOOLP160R1     "1.3.36.3.3.2.8.1.1.1"
#define szOID_ECC_CURVE_BRAINPOOLP160T1     "1.3.36.3.3.2.8.1.1.2"
#define szOID_ECC_CURVE_BRAINPOOLP192R1     "1.3.36.3.3.2.8.1.1.3"
#define szOID_ECC_CURVE_BRAINPOOLP192T1     "1.3.36.3.3.2.8.1.1.4"
#define szOID_ECC_CURVE_BRAINPOOLP224R1     "1.3.36.3.3.2.8.1.1.5"
#define szOID_ECC_CURVE_BRAINPOOLP224T1     "1.3.36.3.3.2.8.1.1.6"
#define szOID_ECC_CURVE_BRAINPOOLP256R1     "1.3.36.3.3.2.8.1.1.7"
#define szOID_ECC_CURVE_BRAINPOOLP256T1     "1.3.36.3.3.2.8.1.1.8"
#define szOID_ECC_CURVE_BRAINPOOLP320R1     "1.3.36.3.3.2.8.1.1.9"
#define szOID_ECC_CURVE_BRAINPOOLP320T1     "1.3.36.3.3.2.8.1.1.10"
#define szOID_ECC_CURVE_BRAINPOOLP384R1     "1.3.36.3.3.2.8.1.1.11"
#define szOID_ECC_CURVE_BRAINPOOLP384T1     "1.3.36.3.3.2.8.1.1.12"
#define szOID_ECC_CURVE_BRAINPOOLP512R1     "1.3.36.3.3.2.8.1.1.13"
#define szOID_ECC_CURVE_BRAINPOOLP512T1     "1.3.36.3.3.2.8.1.1.14"

#define szOID_ECC_CURVE_EC192WAPI           "1.2.156.11235.1.1.2.1"
#define szOID_CN_ECDSA_SHA256               "1.2.156.11235.1.1.1"

#define szOID_ECC_CURVE_NISTP192            "1.2.840.10045.3.1.1"
#define szOID_ECC_CURVE_NISTP224            "1.3.132.0.33"
#define szOID_ECC_CURVE_NISTP256            szOID_ECC_CURVE_P256
#define szOID_ECC_CURVE_NISTP384            szOID_ECC_CURVE_P384
#define szOID_ECC_CURVE_NISTP521            szOID_ECC_CURVE_P521

#define szOID_ECC_CURVE_SECP160K1           "1.3.132.0.9"
#define szOID_ECC_CURVE_SECP160R1           "1.3.132.0.8"
#define szOID_ECC_CURVE_SECP160R2           "1.3.132.0.30"
#define szOID_ECC_CURVE_SECP192K1           "1.3.132.0.31"
#define szOID_ECC_CURVE_SECP192R1           szOID_ECC_CURVE_NISTP192
#define szOID_ECC_CURVE_SECP224K1           "1.3.132.0.32"
#define szOID_ECC_CURVE_SECP224R1           szOID_ECC_CURVE_NISTP224
#define szOID_ECC_CURVE_SECP256K1           "1.3.132.0.10"
#define szOID_ECC_CURVE_SECP256R1           szOID_ECC_CURVE_P256
#define szOID_ECC_CURVE_SECP384R1           szOID_ECC_CURVE_P384
#define szOID_ECC_CURVE_SECP521R1           szOID_ECC_CURVE_P521

#define szOID_ECC_CURVE_WTLS7               szOID_ECC_CURVE_SECP160R2
#define szOID_ECC_CURVE_WTLS9               "2.23.43.1.4.9"
#define szOID_ECC_CURVE_WTLS12              szOID_ECC_CURVE_NISTP224

#define szOID_ECC_CURVE_X962P192V1          "1.2.840.10045.3.1.1"
#define szOID_ECC_CURVE_X962P192V2          "1.2.840.10045.3.1.2"
#define szOID_ECC_CURVE_X962P192V3          "1.2.840.10045.3.1.3"
#define szOID_ECC_CURVE_X962P239V1          "1.2.840.10045.3.1.4"
#define szOID_ECC_CURVE_X962P239V2          "1.2.840.10045.3.1.5"
#define szOID_ECC_CURVE_X962P239V3          "1.2.840.10045.3.1.6"
#define szOID_ECC_CURVE_X962P256V1          szOID_ECC_CURVE_P256


// iso(1) member-body(2) us(840) 10045 signatures(4) sha1(1)
#define szOID_ECDSA_SHA1        "1.2.840.10045.4.1"

// iso(1) member-body(2) us(840) 10045 signatures(4) specified(3)
#define szOID_ECDSA_SPECIFIED   "1.2.840.10045.4.3"

// iso(1) member-body(2) us(840) 10045 signatures(4) specified(3) 2
#define szOID_ECDSA_SHA256      "1.2.840.10045.4.3.2"

// iso(1) member-body(2) us(840) 10045 signatures(4) specified(3) 3
#define szOID_ECDSA_SHA384      "1.2.840.10045.4.3.3"

// iso(1) member-body(2) us(840) 10045 signatures(4) specified(3) 4
#define szOID_ECDSA_SHA512      "1.2.840.10045.4.3.4"


// NIST AES CBC Algorithms
// joint-iso-itu-t(2) country(16) us(840) organization(1) gov(101) csor(3) nistAlgorithms(4)  aesAlgs(1) }

#define szOID_NIST_AES128_CBC        "2.16.840.1.101.3.4.1.2"
#define szOID_NIST_AES192_CBC        "2.16.840.1.101.3.4.1.22"
#define szOID_NIST_AES256_CBC        "2.16.840.1.101.3.4.1.42"

// For the above Algorithms, the AlgorithmIdentifier parameters must be
// present and the parameters field MUST contain an AES-IV:
//
//  AES-IV ::= OCTET STRING (SIZE(16))

// NIST AES WRAP Algorithms
#define szOID_NIST_AES128_WRAP       "2.16.840.1.101.3.4.1.5"
#define szOID_NIST_AES192_WRAP       "2.16.840.1.101.3.4.1.25"
#define szOID_NIST_AES256_WRAP       "2.16.840.1.101.3.4.1.45"


//      x9-63-scheme OBJECT IDENTIFIER ::= { iso(1)
//         identified-organization(3) tc68(133) country(16) x9(840)
//         x9-63(63) schemes(0) }


// ECDH single pass ephemeral-static KeyAgreement KeyEncryptionAlgorithm
#define szOID_DH_SINGLE_PASS_STDDH_SHA1_KDF   "1.3.133.16.840.63.0.2"
#define szOID_DH_SINGLE_PASS_STDDH_SHA256_KDF "1.3.132.1.11.1"
#define szOID_DH_SINGLE_PASS_STDDH_SHA384_KDF "1.3.132.1.11.2"

// For the above KeyEncryptionAlgorithm the following wrap algorithms are
// supported:
//  szOID_RSA_SMIMEalgCMS3DESwrap
//  szOID_RSA_SMIMEalgCMSRC2wrap
//  szOID_NIST_AES128_WRAP
//  szOID_NIST_AES192_WRAP
//  szOID_NIST_AES256_WRAP



// ITU-T UsefulDefinitions
#define szOID_DS                "2.5"
#define szOID_DSALG             "2.5.8"
#define szOID_DSALG_CRPT        "2.5.8.1"
#define szOID_DSALG_HASH        "2.5.8.2"
#define szOID_DSALG_SIGN        "2.5.8.3"
#define szOID_DSALG_RSA         "2.5.8.1.1"
// NIST OSE Implementors' Workshop (OIW)
// http://nemo.ncsl.nist.gov/oiw/agreements/stable/OSI/12s_9506.w51
// http://nemo.ncsl.nist.gov/oiw/agreements/working/OSI/12w_9503.w51
#define szOID_OIW               "1.3.14"
// NIST OSE Implementors' Workshop (OIW) Security SIG algorithm identifiers
#define szOID_OIWSEC            "1.3.14.3.2"
#define szOID_OIWSEC_md4RSA     "1.3.14.3.2.2"
#define szOID_OIWSEC_md5RSA     "1.3.14.3.2.3"
#define szOID_OIWSEC_md4RSA2    "1.3.14.3.2.4"
#define szOID_OIWSEC_desECB     "1.3.14.3.2.6"
#define szOID_OIWSEC_desCBC     "1.3.14.3.2.7"
#define szOID_OIWSEC_desOFB     "1.3.14.3.2.8"
#define szOID_OIWSEC_desCFB     "1.3.14.3.2.9"
#define szOID_OIWSEC_desMAC     "1.3.14.3.2.10"
#define szOID_OIWSEC_rsaSign    "1.3.14.3.2.11"
#define szOID_OIWSEC_dsa        "1.3.14.3.2.12"
#define szOID_OIWSEC_shaDSA     "1.3.14.3.2.13"
#define szOID_OIWSEC_mdc2RSA    "1.3.14.3.2.14"
#define szOID_OIWSEC_shaRSA     "1.3.14.3.2.15"
#define szOID_OIWSEC_dhCommMod  "1.3.14.3.2.16"
#define szOID_OIWSEC_desEDE     "1.3.14.3.2.17"
#define szOID_OIWSEC_sha        "1.3.14.3.2.18"
#define szOID_OIWSEC_mdc2       "1.3.14.3.2.19"
#define szOID_OIWSEC_dsaComm    "1.3.14.3.2.20"
#define szOID_OIWSEC_dsaCommSHA "1.3.14.3.2.21"
#define szOID_OIWSEC_rsaXchg    "1.3.14.3.2.22"
#define szOID_OIWSEC_keyHashSeal "1.3.14.3.2.23"
#define szOID_OIWSEC_md2RSASign "1.3.14.3.2.24"
#define szOID_OIWSEC_md5RSASign "1.3.14.3.2.25"
#define szOID_OIWSEC_sha1       "1.3.14.3.2.26"
#define szOID_OIWSEC_dsaSHA1    "1.3.14.3.2.27"
#define szOID_OIWSEC_dsaCommSHA1 "1.3.14.3.2.28"
#define szOID_OIWSEC_sha1RSASign "1.3.14.3.2.29"
// NIST OSE Implementors' Workshop (OIW) Directory SIG algorithm identifiers
#define szOID_OIWDIR            "1.3.14.7.2"
#define szOID_OIWDIR_CRPT       "1.3.14.7.2.1"
#define szOID_OIWDIR_HASH       "1.3.14.7.2.2"
#define szOID_OIWDIR_SIGN       "1.3.14.7.2.3"
#define szOID_OIWDIR_md2        "1.3.14.7.2.2.1"
#define szOID_OIWDIR_md2RSA     "1.3.14.7.2.3.1"


// INFOSEC Algorithms
// joint-iso-ccitt(2) country(16) us(840) organization(1) us-government(101) dod(2) id-infosec(1)
#define szOID_INFOSEC                       "2.16.840.1.101.2.1"
#define szOID_INFOSEC_sdnsSignature         "2.16.840.1.101.2.1.1.1"
#define szOID_INFOSEC_mosaicSignature       "2.16.840.1.101.2.1.1.2"
#define szOID_INFOSEC_sdnsConfidentiality   "2.16.840.1.101.2.1.1.3"
#define szOID_INFOSEC_mosaicConfidentiality "2.16.840.1.101.2.1.1.4"
#define szOID_INFOSEC_sdnsIntegrity         "2.16.840.1.101.2.1.1.5"
#define szOID_INFOSEC_mosaicIntegrity       "2.16.840.1.101.2.1.1.6"
#define szOID_INFOSEC_sdnsTokenProtection   "2.16.840.1.101.2.1.1.7"
#define szOID_INFOSEC_mosaicTokenProtection "2.16.840.1.101.2.1.1.8"
#define szOID_INFOSEC_sdnsKeyManagement     "2.16.840.1.101.2.1.1.9"
#define szOID_INFOSEC_mosaicKeyManagement   "2.16.840.1.101.2.1.1.10"
#define szOID_INFOSEC_sdnsKMandSig          "2.16.840.1.101.2.1.1.11"
#define szOID_INFOSEC_mosaicKMandSig        "2.16.840.1.101.2.1.1.12"
#define szOID_INFOSEC_SuiteASignature       "2.16.840.1.101.2.1.1.13"
#define szOID_INFOSEC_SuiteAConfidentiality "2.16.840.1.101.2.1.1.14"
#define szOID_INFOSEC_SuiteAIntegrity       "2.16.840.1.101.2.1.1.15"
#define szOID_INFOSEC_SuiteATokenProtection "2.16.840.1.101.2.1.1.16"
#define szOID_INFOSEC_SuiteAKeyManagement   "2.16.840.1.101.2.1.1.17"
#define szOID_INFOSEC_SuiteAKMandSig        "2.16.840.1.101.2.1.1.18"
#define szOID_INFOSEC_mosaicUpdatedSig      "2.16.840.1.101.2.1.1.19"
#define szOID_INFOSEC_mosaicKMandUpdSig     "2.16.840.1.101.2.1.1.20"
#define szOID_INFOSEC_mosaicUpdatedInteg    "2.16.840.1.101.2.1.1.21"

// NIST Hash Algorithms
// joint-iso-itu-t(2) country(16) us(840) organization(1) gov(101) csor(3) nistalgorithm(4) hashalgs(2)

#define szOID_NIST_sha256                   "2.16.840.1.101.3.4.2.1"
#define szOID_NIST_sha384                   "2.16.840.1.101.3.4.2.2"
#define szOID_NIST_sha512                   "2.16.840.1.101.3.4.2.3"
#define szOID_NIST_shake128                 "2.16.840.1.101.3.4.2.11"
#define szOID_NIST_shake256                 "2.16.840.1.101.3.4.2.12"

// NIST PQ Algorithms
// joint-iso-itu-t(2) country(16) us(840) organization(1) gov(101) csor(3) nistAlgorithms(4) 3

// "Pure" ML-DSA 
#define szOID_NIST_ml_dsa_44            "2.16.840.1.101.3.4.3.17"
#define szOID_NIST_ml_dsa_65            "2.16.840.1.101.3.4.3.18"
#define szOID_NIST_ml_dsa_87            "2.16.840.1.101.3.4.3.19"

// "Pre-Hash" ML-DSA
#define szOID_NIST_hash_ml_dsa_44_with_sha512   "2.16.840.1.101.3.4.3.32"
#define szOID_NIST_hash_ml_dsa_65_with_sha512   "2.16.840.1.101.3.4.3.33"
#define szOID_NIST_hash_ml_dsa_87_with_sha512   "2.16.840.1.101.3.4.3.34"

// ML-KEM
#define szOID_NIST_ml_kem_512           "2.16.840.1.101.3.4.4.1"
#define szOID_NIST_ml_kem_768           "2.16.840.1.101.3.4.4.2"
#define szOID_NIST_ml_kem_1024          "2.16.840.1.101.3.4.4.3"

// "Pure" SLH-DSA
#define szOID_NIST_slh_dsa_sha2_128s                "2.16.840.1.101.3.4.3.20"
#define szOID_NIST_slh_dsa_sha2_128f                "2.16.840.1.101.3.4.3.21"
#define szOID_NIST_slh_dsa_sha2_192s                "2.16.840.1.101.3.4.3.22"
#define szOID_NIST_slh_dsa_sha2_192f                "2.16.840.1.101.3.4.3.23"
#define szOID_NIST_slh_dsa_sha2_256s                "2.16.840.1.101.3.4.3.24"
#define szOID_NIST_slh_dsa_sha2_256f                "2.16.840.1.101.3.4.3.25"
#define szOID_NIST_slh_dsa_shake_128s               "2.16.840.1.101.3.4.3.26"
#define szOID_NIST_slh_dsa_shake_128f               "2.16.840.1.101.3.4.3.27"
#define szOID_NIST_slh_dsa_shake_192s               "2.16.840.1.101.3.4.3.28"
#define szOID_NIST_slh_dsa_shake_192f               "2.16.840.1.101.3.4.3.29"
#define szOID_NIST_slh_dsa_shake_256s               "2.16.840.1.101.3.4.3.30"
#define szOID_NIST_slh_dsa_shake_256f               "2.16.840.1.101.3.4.3.31"

// "PreHash" SLH-DSA
#define szOID_NIST_hash_slh_dsa_sha2_128s_with_sha256    "2.16.840.1.101.3.4.3.35"
#define szOID_NIST_hash_slh_dsa_sha2_128f_with_sha256    "2.16.840.1.101.3.4.3.36"
#define szOID_NIST_hash_slh_dsa_sha2_192s_with_sha512    "2.16.840.1.101.3.4.3.37"
#define szOID_NIST_hash_slh_dsa_sha2_192f_with_sha512    "2.16.840.1.101.3.4.3.38"
#define szOID_NIST_hash_slh_dsa_sha2_256s_with_sha512    "2.16.840.1.101.3.4.3.39"
#define szOID_NIST_hash_slh_dsa_sha2_256f_with_sha512    "2.16.840.1.101.3.4.3.40"
#define szOID_NIST_hash_slh_dsa_shake_128s_with_shake128    "2.16.840.1.101.3.4.3.41"
#define szOID_NIST_hash_slh_dsa_shake_128f_with_shake128    "2.16.840.1.101.3.4.3.42"
#define szOID_NIST_hash_slh_dsa_shake_192s_with_shake256    "2.16.840.1.101.3.4.3.43"
#define szOID_NIST_hash_slh_dsa_shake_192f_with_shake256    "2.16.840.1.101.3.4.3.44"
#define szOID_NIST_hash_slh_dsa_shake_256s_with_shake256    "2.16.840.1.101.3.4.3.45"
#define szOID_NIST_hash_slh_dsa_shake_256f_with_shake256    "2.16.840.1.101.3.4.3.46"

typedef struct _CRYPT_OBJID_TABLE {
    DWORD   dwAlgId;
    LPCSTR  pszObjId;
} CRYPT_OBJID_TABLE, *PCRYPT_OBJID_TABLE;


//+-------------------------------------------------------------------------
//  PKCS #1 HashInfo (DigestInfo)
//--------------------------------------------------------------------------
typedef struct _CRYPT_HASH_INFO {
    CRYPT_ALGORITHM_IDENTIFIER  HashAlgorithm;
    CRYPT_HASH_BLOB             Hash;
} CRYPT_HASH_INFO, *PCRYPT_HASH_INFO;

//+-------------------------------------------------------------------------
//  Type used for an extension to an encoded content
//
//  Where the Value's CRYPT_OBJID_BLOB is in its encoded representation.
//--------------------------------------------------------------------------
// certenrolls_begin -- CERT_CONTEXT
typedef struct _CERT_EXTENSION {
    LPSTR               pszObjId;
    BOOL                fCritical;
    CRYPT_OBJID_BLOB    Value;
} CERT_EXTENSION, *PCERT_EXTENSION;
typedef const CERT_EXTENSION* PCCERT_EXTENSION;
// certenrolls_end

//+-------------------------------------------------------------------------
//  AttributeTypeValue
//
//  Where the Value's CRYPT_OBJID_BLOB is in its encoded representation.
//--------------------------------------------------------------------------
// certenrolls_begin -- CRYPT_ATTRIBUTE_TYPE_VALUE
typedef struct _CRYPT_ATTRIBUTE_TYPE_VALUE {
    LPSTR               pszObjId;
    CRYPT_OBJID_BLOB    Value;
} CRYPT_ATTRIBUTE_TYPE_VALUE, *PCRYPT_ATTRIBUTE_TYPE_VALUE;
// certenrolls_end

//+-------------------------------------------------------------------------
//  Attributes
//
//  Where the Value's PATTR_BLOBs are in their encoded representation.
//--------------------------------------------------------------------------
// certenrolls_begin -- CRYPT_ATTRIBUTE
typedef struct _CRYPT_ATTRIBUTE {
    LPSTR               pszObjId;
    DWORD               cValue;
    PCRYPT_ATTR_BLOB    rgValue;
} CRYPT_ATTRIBUTE, *PCRYPT_ATTRIBUTE;

typedef struct _CRYPT_ATTRIBUTES {
    DWORD                cAttr;
    PCRYPT_ATTRIBUTE     rgAttr;
} CRYPT_ATTRIBUTES, *PCRYPT_ATTRIBUTES;
// certenrolls_end

//+-------------------------------------------------------------------------
//  Attributes making up a Relative Distinguished Name (CERT_RDN)
//
//  The interpretation of the Value depends on the dwValueType.
//  See below for a list of the types.
//--------------------------------------------------------------------------
typedef struct _CERT_RDN_ATTR {
    LPSTR                   pszObjId;
    DWORD                   dwValueType;
    CERT_RDN_VALUE_BLOB     Value;
} CERT_RDN_ATTR, *PCERT_RDN_ATTR;

//+-------------------------------------------------------------------------
//  CERT_RDN attribute Object Identifiers
//--------------------------------------------------------------------------
// Labeling attribute types:
#define szOID_COMMON_NAME                   "2.5.4.3"  // case-ignore string
#define szOID_SUR_NAME                      "2.5.4.4"  // case-ignore string
#define szOID_DEVICE_SERIAL_NUMBER          "2.5.4.5"  // printable string

// Geographic attribute types:
#define szOID_COUNTRY_NAME                  "2.5.4.6"  // printable 2char string
#define szOID_LOCALITY_NAME                 "2.5.4.7"  // case-ignore string
#define szOID_STATE_OR_PROVINCE_NAME        "2.5.4.8"  // case-ignore string
#define szOID_STREET_ADDRESS                "2.5.4.9"  // case-ignore string

// Organizational attribute types:
#define szOID_ORGANIZATION_NAME             "2.5.4.10" // case-ignore string
#define szOID_ORGANIZATIONAL_UNIT_NAME      "2.5.4.11" // case-ignore string
#define szOID_TITLE                         "2.5.4.12" // case-ignore string

// Explanatory attribute types:
#define szOID_DESCRIPTION                   "2.5.4.13" // case-ignore string
#define szOID_SEARCH_GUIDE                  "2.5.4.14"
#define szOID_BUSINESS_CATEGORY             "2.5.4.15" // case-ignore string

// Postal addressing attribute types:
#define szOID_POSTAL_ADDRESS                "2.5.4.16"
#define szOID_POSTAL_CODE                   "2.5.4.17" // case-ignore string
#define szOID_POST_OFFICE_BOX               "2.5.4.18" // case-ignore string
#define szOID_PHYSICAL_DELIVERY_OFFICE_NAME "2.5.4.19" // case-ignore string

// Telecommunications addressing attribute types:
#define szOID_TELEPHONE_NUMBER              "2.5.4.20" // telephone number
#define szOID_TELEX_NUMBER                  "2.5.4.21"
#define szOID_TELETEXT_TERMINAL_IDENTIFIER  "2.5.4.22"
#define szOID_FACSIMILE_TELEPHONE_NUMBER    "2.5.4.23"
#define szOID_X21_ADDRESS                   "2.5.4.24" // numeric string
#define szOID_INTERNATIONAL_ISDN_NUMBER     "2.5.4.25" // numeric string
#define szOID_REGISTERED_ADDRESS            "2.5.4.26"
#define szOID_DESTINATION_INDICATOR         "2.5.4.27" // printable string

// Preference attribute types:
#define szOID_PREFERRED_DELIVERY_METHOD     "2.5.4.28"

// OSI application attribute types:
#define szOID_PRESENTATION_ADDRESS          "2.5.4.29"
#define szOID_SUPPORTED_APPLICATION_CONTEXT "2.5.4.30"

// Relational application attribute types:
#define szOID_MEMBER                        "2.5.4.31"
#define szOID_OWNER                         "2.5.4.32"
#define szOID_ROLE_OCCUPANT                 "2.5.4.33"
#define szOID_SEE_ALSO                      "2.5.4.34"

// Security attribute types:
#define szOID_USER_PASSWORD                 "2.5.4.35"
#define szOID_USER_CERTIFICATE              "2.5.4.36"
#define szOID_CA_CERTIFICATE                "2.5.4.37"
#define szOID_AUTHORITY_REVOCATION_LIST     "2.5.4.38"
#define szOID_CERTIFICATE_REVOCATION_LIST   "2.5.4.39"
#define szOID_CROSS_CERTIFICATE_PAIR        "2.5.4.40"

// Undocumented attribute types???
//#define szOID_???                         "2.5.4.41"
#define szOID_GIVEN_NAME                    "2.5.4.42" // case-ignore string
#define szOID_INITIALS                      "2.5.4.43" // case-ignore string

// The DN Qualifier attribute type specifies disambiguating information to add
// to the relative distinguished name of an entry. It is intended to be used
// for entries held in multiple DSAs which would otherwise have the same name,
// and that its value be the same in a given DSA for all entries to which
// the information has been added.
#define szOID_DN_QUALIFIER                  "2.5.4.46"

// Pilot user attribute types:
#define szOID_DOMAIN_COMPONENT  "0.9.2342.19200300.100.1.25" // IA5, UTF8 string

// used for PKCS 12 attributes
#define szOID_PKCS_12_FRIENDLY_NAME_ATTR     "1.2.840.113549.1.9.20"
#define szOID_PKCS_12_LOCAL_KEY_ID           "1.2.840.113549.1.9.21"
#define szOID_PKCS_12_KEY_PROVIDER_NAME_ATTR "1.3.6.1.4.1.311.17.1"
#define szOID_LOCAL_MACHINE_KEYSET           "1.3.6.1.4.1.311.17.2"
#define szOID_PKCS_12_EXTENDED_ATTRIBUTES    "1.3.6.1.4.1.311.17.3"
#define szOID_PKCS_12_PROTECTED_PASSWORD_SECRET_BAG_TYPE_ID "1.3.6.1.4.1.311.17.4"

//+-------------------------------------------------------------------------
//  Microsoft CERT_RDN attribute Object Identifiers
//--------------------------------------------------------------------------
// Special RDN containing the KEY_ID. Its value type is CERT_RDN_OCTET_STRING.
#define szOID_KEYID_RDN                     "1.3.6.1.4.1.311.10.7.1"

//+-------------------------------------------------------------------------
//  EV RDN OIDs
//--------------------------------------------------------------------------
#define szOID_EV_RDN_LOCALE                         "1.3.6.1.4.1.311.60.2.1.1"
#define szOID_EV_RDN_STATE_OR_PROVINCE              "1.3.6.1.4.1.311.60.2.1.2"
#define szOID_EV_RDN_COUNTRY                        "1.3.6.1.4.1.311.60.2.1.3"

//+-------------------------------------------------------------------------
//  CERT_RDN Attribute Value Types
//
//  For RDN_ENCODED_BLOB, the Value's CERT_RDN_VALUE_BLOB is in its encoded
//  representation. Otherwise, its an array of bytes.
//
//  For all CERT_RDN types, Value.cbData is always the number of bytes, not
//  necessarily the number of elements in the string. For instance,
//  RDN_UNIVERSAL_STRING is an array of ints (cbData == intCnt * 4) and
//  RDN_BMP_STRING is an array of unsigned shorts (cbData == ushortCnt * 2).
//
//  A RDN_UTF8_STRING is an array of UNICODE characters (cbData == charCnt *2).
//  These UNICODE characters are encoded as UTF8 8 bit characters.
//
//  For CertDecodeName, two 0 bytes are always appended to the end of the
//  string (ensures a CHAR or WCHAR string is null terminated).
//  These added 0 bytes are't included in the BLOB.cbData.
//--------------------------------------------------------------------------
#define CERT_RDN_ANY_TYPE                0
#define CERT_RDN_ENCODED_BLOB            1
#define CERT_RDN_OCTET_STRING            2
#define CERT_RDN_NUMERIC_STRING          3
#define CERT_RDN_PRINTABLE_STRING        4
#define CERT_RDN_TELETEX_STRING          5
#define CERT_RDN_T61_STRING              5
#define CERT_RDN_VIDEOTEX_STRING         6
#define CERT_RDN_IA5_STRING              7
#define CERT_RDN_GRAPHIC_STRING          8
#define CERT_RDN_VISIBLE_STRING          9
#define CERT_RDN_ISO646_STRING           9
#define CERT_RDN_GENERAL_STRING          10
#define CERT_RDN_UNIVERSAL_STRING        11
#define CERT_RDN_INT4_STRING             11
#define CERT_RDN_BMP_STRING              12
#define CERT_RDN_UNICODE_STRING          12
#define CERT_RDN_UTF8_STRING             13

#define CERT_RDN_TYPE_MASK                  0x000000FF
#define CERT_RDN_FLAGS_MASK                 0xFF000000

//+-------------------------------------------------------------------------
//  Flags that can be or'ed with the above Value Type when encoding/decoding
//--------------------------------------------------------------------------
// For encoding: when set, CERT_RDN_T61_STRING is selected instead of
// CERT_RDN_UNICODE_STRING if all the unicode characters are <= 0xFF
#define CERT_RDN_ENABLE_T61_UNICODE_FLAG    0x80000000

// For encoding: when set, CERT_RDN_UTF8_STRING is selected instead of
// CERT_RDN_UNICODE_STRING.
#define CERT_RDN_ENABLE_UTF8_UNICODE_FLAG   0x20000000

// For encoding: when set, CERT_RDN_UTF8_STRING is selected instead of
// CERT_RDN_PRINTABLE_STRING for DirectoryString types. Also,
// enables CERT_RDN_ENABLE_UTF8_UNICODE_FLAG.
#define CERT_RDN_FORCE_UTF8_UNICODE_FLAG    0x10000000

// For encoding: when set, the characters aren't checked to see if they
// are valid for the Value Type.
#define CERT_RDN_DISABLE_CHECK_TYPE_FLAG    0x40000000

// For decoding: by default, CERT_RDN_T61_STRING values are initially decoded
// as UTF8. If the UTF8 decoding fails, then, decoded as 8 bit characters.
// Setting this flag skips the initial attempt to decode as UTF8.
#define CERT_RDN_DISABLE_IE4_UTF8_FLAG      0x01000000

// For encoding: If the string contains E/Email RDN, and the email-address
// (in RDN value) contains unicode characters outside of ASCII character set,
// the localpart and the hostname portion of the email-address would be first
// encoded in punycode and then the resultant Email-Address would be attempted
// to be encoded as IA5String. Punycode encoding of hostname is done on
// label-by-label basis.
// For decoding: If the name contains E/Email RDN, and local part or hostname
// portion of the email-address contains punycode encoded IA5String,
// The RDN string value is converted to its unicode equivalent.
#define CERT_RDN_ENABLE_PUNYCODE_FLAG       0x02000000

// Macro to check that the dwValueType is a character string and not an
// encoded blob or octet string
#define IS_CERT_RDN_CHAR_STRING(X)      \
                (((X) & CERT_RDN_TYPE_MASK) >= CERT_RDN_NUMERIC_STRING)


//+-------------------------------------------------------------------------
//  A CERT_RDN consists of an array of the above attributes
//--------------------------------------------------------------------------
typedef struct _CERT_RDN {
    DWORD           cRDNAttr;
    PCERT_RDN_ATTR  rgRDNAttr;
} CERT_RDN, *PCERT_RDN;

//+-------------------------------------------------------------------------
//  Information stored in a subject's or issuer's name. The information
//  is represented as an array of the above RDNs.
//--------------------------------------------------------------------------
typedef struct _CERT_NAME_INFO {
    DWORD       cRDN;
    PCERT_RDN   rgRDN;
} CERT_NAME_INFO, *PCERT_NAME_INFO;

//+-------------------------------------------------------------------------
//  Name attribute value without the Object Identifier
//
//  The interpretation of the Value depends on the dwValueType.
//  See above for a list of the types.
//--------------------------------------------------------------------------
typedef struct _CERT_NAME_VALUE {
    DWORD               dwValueType;
    CERT_RDN_VALUE_BLOB Value;
} CERT_NAME_VALUE, *PCERT_NAME_VALUE;

//+-------------------------------------------------------------------------
//  Public Key Info
//
//  The PublicKey is the encoded representation of the information as it is
//  stored in the bit string
//--------------------------------------------------------------------------
// certenrolls_begin -- CERT_CONTEXT
typedef struct _CERT_PUBLIC_KEY_INFO {
    CRYPT_ALGORITHM_IDENTIFIER    Algorithm;
    CRYPT_BIT_BLOB                PublicKey;
} CERT_PUBLIC_KEY_INFO, *PCERT_PUBLIC_KEY_INFO;
// certenrolls_end

#define CERT_RSA_PUBLIC_KEY_OBJID            szOID_RSA_RSA
#define CERT_DEFAULT_OID_PUBLIC_KEY_SIGN     szOID_RSA_RSA
#define CERT_DEFAULT_OID_PUBLIC_KEY_XCHG     szOID_RSA_RSA

//+-------------------------------------------------------------------------
//  ECC Private Key Info
//--------------------------------------------------------------------------
typedef struct _CRYPT_ECC_PRIVATE_KEY_INFO{
    DWORD                       dwVersion;  // ecPrivKeyVer1(1)
    CRYPT_DER_BLOB              PrivateKey; // d
    LPSTR                       szCurveOid; // Optional
    CRYPT_BIT_BLOB              PublicKey;  // Optional (x, y)
}  CRYPT_ECC_PRIVATE_KEY_INFO, *PCRYPT_ECC_PRIVATE_KEY_INFO;

#define CRYPT_ECC_PRIVATE_KEY_INFO_v1       1

//+-------------------------------------------------------------------------
//  structure that contains all the information in a PKCS#8 PrivateKeyInfo
//--------------------------------------------------------------------------
typedef struct _CRYPT_PRIVATE_KEY_INFO{
    DWORD                       Version;
    CRYPT_ALGORITHM_IDENTIFIER  Algorithm;
    CRYPT_DER_BLOB              PrivateKey;
    PCRYPT_ATTRIBUTES           pAttributes;
}  CRYPT_PRIVATE_KEY_INFO, *PCRYPT_PRIVATE_KEY_INFO;

//+-------------------------------------------------------------------------
//  structure that contains all the information in a PKCS#8
//  EncryptedPrivateKeyInfo
//--------------------------------------------------------------------------
typedef struct _CRYPT_ENCRYPTED_PRIVATE_KEY_INFO{
    CRYPT_ALGORITHM_IDENTIFIER  EncryptionAlgorithm;
    CRYPT_DATA_BLOB             EncryptedPrivateKey;
} CRYPT_ENCRYPTED_PRIVATE_KEY_INFO, *PCRYPT_ENCRYPTED_PRIVATE_KEY_INFO;

//+-------------------------------------------------------------------------
// this callback is given when an EncryptedProvateKeyInfo structure is
// encountered during ImportPKCS8.  the caller is then expected to decrypt
// the private key and hand back the decrypted contents.
//
// the parameters are:
// Algorithm - the algorithm used to encrypt the PrivateKeyInfo
// EncryptedPrivateKey - the encrypted private key blob
// pClearTextKey - a buffer to receive the clear text
// cbClearTextKey - the number of bytes of the pClearTextKey buffer
//                  note the if this is zero then this should be
//                  filled in with the size required to decrypt the
//                  key into, and pClearTextKey should be ignored
// pVoidDecryptFunc - this is the pVoid that was passed into the call
//                    and is preserved and passed back as context
//+-------------------------------------------------------------------------
typedef BOOL (CALLBACK *PCRYPT_DECRYPT_PRIVATE_KEY_FUNC)(
    _In_ CRYPT_ALGORITHM_IDENTIFIER Algorithm,
    _In_ CRYPT_DATA_BLOB EncryptedPrivateKey,
    _Out_writes_bytes_opt_ (*pcbClearTextKey) BYTE* pbClearTextKey,
    _Inout_ DWORD* pcbClearTextKey,
    _In_ LPVOID pVoidDecryptFunc);

//+-------------------------------------------------------------------------
// this callback is given when creating a PKCS8 EncryptedPrivateKeyInfo.
// The caller is then expected to encrypt the private key and hand back
// the encrypted contents.
//
// the parameters are:
// Algorithm - the algorithm used to encrypt the PrivateKeyInfo
// pClearTextPrivateKey - the cleartext private key to be encrypted
// pbEncryptedKey - the output encrypted private key blob
// cbEncryptedKey - the number of bytes of the pbEncryptedKey buffer
//                  note the if this is zero then this should be
//                  filled in with the size required to encrypt the
//                  key into, and pbEncryptedKey should be ignored
// pVoidEncryptFunc - this is the pVoid that was passed into the call
//                    and is preserved and passed back as context
//+-------------------------------------------------------------------------
typedef BOOL (CALLBACK *PCRYPT_ENCRYPT_PRIVATE_KEY_FUNC)(
    _Out_ CRYPT_ALGORITHM_IDENTIFIER* pAlgorithm,
    _In_ CRYPT_DATA_BLOB* pClearTextPrivateKey,
    _Out_writes_bytes_opt_ (*pcbEncryptedKey) BYTE* pbEncryptedKey,
    _Inout_ DWORD* pcbEncryptedKey,
    _In_ LPVOID pVoidEncryptFunc);

//+-------------------------------------------------------------------------
// this callback is given from the context of a ImportPKCS8 calls.  the caller
// is then expected to hand back an HCRYPTPROV to receive the key being imported
//
// the parameters are:
// pPrivateKeyInfo - pointer to a CRYPT_PRIVATE_KEY_INFO structure which
//                   describes the key being imported
// EncryptedPrivateKey - the encrypted private key blob
// phCryptProv - a pointer to a HCRRYPTPROV to be filled in
// pVoidResolveFunc - this is the pVoidResolveFunc passed in by the caller in the
//                    CRYPT_PRIVATE_KEY_BLOB_AND_PARAMS struct
//+-------------------------------------------------------------------------
typedef BOOL (CALLBACK *PCRYPT_RESOLVE_HCRYPTPROV_FUNC)(
                                                       CRYPT_PRIVATE_KEY_INFO      *pPrivateKeyInfo,
                                                       HCRYPTPROV                  *phCryptProv,
                                                       LPVOID                      pVoidResolveFunc);

//+-------------------------------------------------------------------------
// this struct contains a PKCS8 private key and two pointers to callback
// functions, with a corresponding pVoids.  the first callback is used to give
// the caller the opportunity to specify where the key is imported to.  the callback
// passes the caller the algoroithm OID and key size to use in making the decision.
// the other callback is used to decrypt the private key if the PKCS8 contains an
// EncryptedPrivateKeyInfo.  both pVoids are preserved and passed back to the caller
// in the respective callback
//+-------------------------------------------------------------------------
typedef struct _CRYPT_PKCS8_IMPORT_PARAMS{
    CRYPT_DIGEST_BLOB               PrivateKey;             // PKCS8 blob
    PCRYPT_RESOLVE_HCRYPTPROV_FUNC  pResolvehCryptProvFunc; // optional
    LPVOID                          pVoidResolveFunc;       // optional
    PCRYPT_DECRYPT_PRIVATE_KEY_FUNC pDecryptPrivateKeyFunc;
    LPVOID                          pVoidDecryptFunc;
} CRYPT_PKCS8_IMPORT_PARAMS, *PCRYPT_PKCS8_IMPORT_PARAMS, CRYPT_PRIVATE_KEY_BLOB_AND_PARAMS, *PCRYPT_PRIVATE_KEY_BLOB_AND_PARAMS;


//+-------------------------------------------------------------------------
// this struct contains information identifying a private key and a pointer
// to a callback function, with a corresponding pVoid. The callback is used
// to encrypt the private key. If the pEncryptPrivateKeyFunc is NULL, the
// key will not be encrypted and an EncryptedPrivateKeyInfo will not be generated.
// The pVoid is preserved and passed back to the caller in the respective callback
//+-------------------------------------------------------------------------
typedef struct _CRYPT_PKCS8_EXPORT_PARAMS{
    HCRYPTPROV                      hCryptProv;
    DWORD                           dwKeySpec;
    LPSTR                           pszPrivateKeyObjId;

    PCRYPT_ENCRYPT_PRIVATE_KEY_FUNC pEncryptPrivateKeyFunc;
    LPVOID                          pVoidEncryptFunc;
} CRYPT_PKCS8_EXPORT_PARAMS, *PCRYPT_PKCS8_EXPORT_PARAMS;

//+-------------------------------------------------------------------------
//  Information stored in a certificate
//
//  The Issuer, Subject, Algorithm, PublicKey and Extension BLOBs are the
//  encoded representation of the information.
//--------------------------------------------------------------------------
// certenrolls_begin -- CERT_CONTEXT
typedef struct _CERT_INFO {
    DWORD                       dwVersion;
    CRYPT_INTEGER_BLOB          SerialNumber;
    CRYPT_ALGORITHM_IDENTIFIER  SignatureAlgorithm;
    CERT_NAME_BLOB              Issuer;
    FILETIME                    NotBefore;
    FILETIME                    NotAfter;
    CERT_NAME_BLOB              Subject;
    CERT_PUBLIC_KEY_INFO        SubjectPublicKeyInfo;
    CRYPT_BIT_BLOB              IssuerUniqueId;
    CRYPT_BIT_BLOB              SubjectUniqueId;
    DWORD                       cExtension;
    PCERT_EXTENSION             rgExtension;
} CERT_INFO, *PCERT_INFO;
// certenrolls_end

//+-------------------------------------------------------------------------
//  Certificate versions
//--------------------------------------------------------------------------
#define CERT_V1     0
#define CERT_V2     1
#define CERT_V3     2

//+-------------------------------------------------------------------------
//  Certificate Information Flags
//--------------------------------------------------------------------------
#define CERT_INFO_VERSION_FLAG                      1
#define CERT_INFO_SERIAL_NUMBER_FLAG                2
#define CERT_INFO_SIGNATURE_ALGORITHM_FLAG          3
#define CERT_INFO_ISSUER_FLAG                       4
#define CERT_INFO_NOT_BEFORE_FLAG                   5
#define CERT_INFO_NOT_AFTER_FLAG                    6
#define CERT_INFO_SUBJECT_FLAG                      7
#define CERT_INFO_SUBJECT_PUBLIC_KEY_INFO_FLAG      8
#define CERT_INFO_ISSUER_UNIQUE_ID_FLAG             9
#define CERT_INFO_SUBJECT_UNIQUE_ID_FLAG            10
#define CERT_INFO_EXTENSION_FLAG                    11

//+-------------------------------------------------------------------------
//  An entry in a CRL
//
//  The Extension BLOBs are the encoded representation of the information.
//--------------------------------------------------------------------------
typedef struct _CRL_ENTRY {
    CRYPT_INTEGER_BLOB  SerialNumber;
    FILETIME            RevocationDate;
    DWORD               cExtension;
    PCERT_EXTENSION     rgExtension;
} CRL_ENTRY, *PCRL_ENTRY;

//+-------------------------------------------------------------------------
//  Information stored in a CRL
//
//  The Issuer, Algorithm and Extension BLOBs are the encoded
//  representation of the information.
//--------------------------------------------------------------------------
typedef struct _CRL_INFO {
    DWORD                       dwVersion;
    CRYPT_ALGORITHM_IDENTIFIER  SignatureAlgorithm;
    CERT_NAME_BLOB              Issuer;
    FILETIME                    ThisUpdate;
    FILETIME                    NextUpdate;
    DWORD                       cCRLEntry;
    PCRL_ENTRY                  rgCRLEntry;
    DWORD                       cExtension;
    PCERT_EXTENSION             rgExtension;
} CRL_INFO, *PCRL_INFO;

//+-------------------------------------------------------------------------
//  CRL versions
//--------------------------------------------------------------------------
#define CRL_V1     0
#define CRL_V2     1

//+-------------------------------------------------------------------------
// Certificate Bundle
//--------------------------------------------------------------------------
#define CERT_BUNDLE_CERTIFICATE 0
#define CERT_BUNDLE_CRL         1

typedef struct _CERT_OR_CRL_BLOB {
    DWORD                   dwChoice;
    DWORD                   cbEncoded;
    _Field_size_bytes_(cbEncoded)
    BYTE                    *pbEncoded;
} CERT_OR_CRL_BLOB, * PCERT_OR_CRL_BLOB;

typedef struct _CERT_OR_CRL_BUNDLE {
    DWORD                   cItem;
    _Field_size_(cItem)
    PCERT_OR_CRL_BLOB       rgItem;
} CERT_OR_CRL_BUNDLE, *PCERT_OR_CRL_BUNDLE;

//+-------------------------------------------------------------------------
//  Information stored in a certificate request
//
//  The Subject, Algorithm, PublicKey and Attribute BLOBs are the encoded
//  representation of the information.
//--------------------------------------------------------------------------
typedef struct _CERT_REQUEST_INFO {
    DWORD                   dwVersion;
    CERT_NAME_BLOB          Subject;
    CERT_PUBLIC_KEY_INFO    SubjectPublicKeyInfo;
    DWORD                   cAttribute;
    PCRYPT_ATTRIBUTE        rgAttribute;
} CERT_REQUEST_INFO, *PCERT_REQUEST_INFO;

//+-------------------------------------------------------------------------
//  Certificate Request versions
//--------------------------------------------------------------------------
#define CERT_REQUEST_V1     0

//+-------------------------------------------------------------------------
//  Information stored in Netscape's Keygen request
//--------------------------------------------------------------------------
typedef struct _CERT_KEYGEN_REQUEST_INFO {
    DWORD                   dwVersion;
    CERT_PUBLIC_KEY_INFO    SubjectPublicKeyInfo;
    LPWSTR                  pwszChallengeString;        // encoded as IA5
} CERT_KEYGEN_REQUEST_INFO, *PCERT_KEYGEN_REQUEST_INFO;

#define CERT_KEYGEN_REQUEST_V1     0


//+-------------------------------------------------------------------------
//  Certificate, CRL, Certificate Request or Keygen Request Signed Content
//
//  The "to be signed" encoded content plus its signature. The ToBeSigned
//  is the encoded CERT_INFO, CRL_INFO, CERT_REQUEST_INFO or
//  CERT_KEYGEN_REQUEST_INFO.
//--------------------------------------------------------------------------
typedef struct _CERT_SIGNED_CONTENT_INFO {
    CRYPT_DER_BLOB              ToBeSigned;
    CRYPT_ALGORITHM_IDENTIFIER  SignatureAlgorithm;
    CRYPT_BIT_BLOB              Signature;
} CERT_SIGNED_CONTENT_INFO, *PCERT_SIGNED_CONTENT_INFO;


//+-------------------------------------------------------------------------
//  Certificate Trust List (CTL)
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CTL Usage. Also used for EnhancedKeyUsage extension.
//--------------------------------------------------------------------------
typedef struct _CTL_USAGE {
    DWORD               cUsageIdentifier;
    LPSTR               *rgpszUsageIdentifier;      // array of pszObjId
} CTL_USAGE, *PCTL_USAGE,
CERT_ENHKEY_USAGE, *PCERT_ENHKEY_USAGE;
typedef const CTL_USAGE* PCCTL_USAGE;
typedef const CERT_ENHKEY_USAGE* PCCERT_ENHKEY_USAGE;


//+-------------------------------------------------------------------------
//  An entry in a CTL
//--------------------------------------------------------------------------
typedef struct _CTL_ENTRY {
    CRYPT_DATA_BLOB     SubjectIdentifier;          // For example, its hash
    DWORD               cAttribute;
    PCRYPT_ATTRIBUTE    rgAttribute;                // OPTIONAL
} CTL_ENTRY, *PCTL_ENTRY;

//+-------------------------------------------------------------------------
//  Information stored in a CTL
//--------------------------------------------------------------------------
typedef struct _CTL_INFO {
    DWORD                       dwVersion;
    CTL_USAGE                   SubjectUsage;
    CRYPT_DATA_BLOB             ListIdentifier;     // OPTIONAL
    CRYPT_INTEGER_BLOB          SequenceNumber;     // OPTIONAL
    FILETIME                    ThisUpdate;
    FILETIME                    NextUpdate;         // OPTIONAL
    CRYPT_ALGORITHM_IDENTIFIER  SubjectAlgorithm;
    DWORD                       cCTLEntry;
    PCTL_ENTRY                  rgCTLEntry;         // OPTIONAL
    DWORD                       cExtension;
    PCERT_EXTENSION             rgExtension;        // OPTIONAL
} CTL_INFO, *PCTL_INFO;

//+-------------------------------------------------------------------------
//  CTL versions
//--------------------------------------------------------------------------
#define CTL_V1     0


//+-------------------------------------------------------------------------
//  TimeStamp Request
//
//  The pszTimeStamp is the OID for the Time type requested
//  The pszContentType is the Content Type OID for the content, usually DATA
//  The Content is a un-decoded blob
//--------------------------------------------------------------------------
typedef struct _CRYPT_TIME_STAMP_REQUEST_INFO {
    LPSTR                   pszTimeStampAlgorithm;   // pszObjId
    LPSTR                   pszContentType;          // pszObjId
    CRYPT_OBJID_BLOB        Content;
    DWORD                   cAttribute;
    PCRYPT_ATTRIBUTE        rgAttribute;
} CRYPT_TIME_STAMP_REQUEST_INFO, *PCRYPT_TIME_STAMP_REQUEST_INFO;

//+-------------------------------------------------------------------------
//  Name Value Attribute
//--------------------------------------------------------------------------
typedef struct _CRYPT_ENROLLMENT_NAME_VALUE_PAIR {
    LPWSTR      pwszName;
    LPWSTR      pwszValue;
} CRYPT_ENROLLMENT_NAME_VALUE_PAIR, * PCRYPT_ENROLLMENT_NAME_VALUE_PAIR;

//+-------------------------------------------------------------------------
//  CSP Provider
//--------------------------------------------------------------------------
typedef struct _CRYPT_CSP_PROVIDER {
    DWORD           dwKeySpec;
    LPWSTR          pwszProviderName;
    CRYPT_BIT_BLOB  Signature;
} CRYPT_CSP_PROVIDER, * PCRYPT_CSP_PROVIDER;

//+-------------------------------------------------------------------------
//  Certificate and Message encoding types
//
//  The encoding type is a DWORD containing both the certificate and message
//  encoding types. The certificate encoding type is stored in the LOWORD.
//  The message encoding type is stored in the HIWORD. Some functions or
//  structure fields require only one of the encoding types. The following
//  naming convention is used to indicate which encoding type(s) are
//  required:
//      dwEncodingType              (both encoding types are required)
//      dwMsgAndCertEncodingType    (both encoding types are required)
//      dwMsgEncodingType           (only msg encoding type is required)
//      dwCertEncodingType          (only cert encoding type is required)
//
//  Its always acceptable to specify both.
//--------------------------------------------------------------------------
#define CERT_ENCODING_TYPE_MASK     0x0000FFFF
#define CMSG_ENCODING_TYPE_MASK     0xFFFF0000
#define GET_CERT_ENCODING_TYPE(X)   (X & CERT_ENCODING_TYPE_MASK)
#define GET_CMSG_ENCODING_TYPE(X)   (X & CMSG_ENCODING_TYPE_MASK)

#define CRYPT_ASN_ENCODING          0x00000001
#define CRYPT_NDR_ENCODING          0x00000002
#define X509_ASN_ENCODING           0x00000001
#define X509_NDR_ENCODING           0x00000002
#define PKCS_7_ASN_ENCODING         0x00010000
#define PKCS_7_NDR_ENCODING         0x00020000

//+-------------------------------------------------------------------------
//  format the specified data structure according to the certificate
//  encoding type.
//
//  The default behavior of CryptFormatObject is to return single line
//  display of the encoded data, that is, each subfield will be concatenated with
//  a ", " on one line.  If user prefers to display the data in multiple line,
//  set the flag CRYPT_FORMAT_STR_MULTI_LINE, that is, each subfield will be displayed
//  on a seperate line.
//
//  If there is no formatting routine installed or registered
//  for the lpszStructType, the hex dump of the encoded BLOB will be returned.
//  User can set the flag CRYPT_FORMAT_STR_NO_HEX to disable the hex dump.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptFormatObject(
    _In_ DWORD dwCertEncodingType,
    _In_ DWORD dwFormatType,
    _In_ DWORD dwFormatStrType,
    _In_opt_ void *pFormatStruct,
    _In_opt_ LPCSTR lpszStructType,
    _In_reads_bytes_(cbEncoded) const BYTE *pbEncoded,
    _In_ DWORD cbEncoded,
    _At_((WCHAR *)pbFormat, _Out_writes_bytes_to_opt_(*pcbFormat, *pcbFormat)) void *pbFormat,
    _Inout_ DWORD *pcbFormat
    );
//-------------------------------------------------------------------------
// constants for dwFormatStrType of function CryptFormatObject
//-------------------------------------------------------------------------
#define         CRYPT_FORMAT_STR_MULTI_LINE         0x0001
#define         CRYPT_FORMAT_STR_NO_HEX             0x0010

//-------------------------------------------------------------------------
// constants for dwFormatType of function CryptFormatObject
// when format X509_NAME or X509_UNICODE_NAME
//-------------------------------------------------------------------------
// Just get the simple string
#define         CRYPT_FORMAT_SIMPLE                 0x0001

//Put an attribute name infront of the attribute
//such as "O=Microsoft,DN=xiaohs"
#define         CRYPT_FORMAT_X509                   0x0002

//Put an OID infront of the simple string, such as
//"2.5.4.22=Microsoft,2.5.4.3=xiaohs"
#define         CRYPT_FORMAT_OID                    0x0004

//Put a ";" between each RDN.  The default is ","
#define         CRYPT_FORMAT_RDN_SEMICOLON          0x0100

//Put a "\n" between each RDN.
#define         CRYPT_FORMAT_RDN_CRLF               0x0200


//Unquote the DN value, which is quoated by default va the following
//rules: if the DN contains leading or trailing
//white space or one of the following characters: ",", "+", "=",
//""", "\n",  "<", ">", "#" or ";". The quoting character is ".
//If the DN Value contains a " it is double quoted ("").
#define         CRYPT_FORMAT_RDN_UNQUOTE            0x0400

//reverse the order of the RDNs before converting to the string
#define         CRYPT_FORMAT_RDN_REVERSE            0x0800


//-------------------------------------------------------------------------
//  contants dwFormatType of function CryptFormatObject when format a DN.:
//
//  The following three values are defined in the section above:
//  CRYPT_FORMAT_SIMPLE:    Just a simple string
//                          such as  "Microsoft+xiaohs+NT"
//  CRYPT_FORMAT_X509       Put an attribute name infront of the attribute
//                          such as "O=Microsoft+xiaohs+NT"
//
//  CRYPT_FORMAT_OID        Put an OID infront of the simple string,
//                          such as "2.5.4.22=Microsoft+xiaohs+NT"
//
//  Additional values are defined as following:
//----------------------------------------------------------------------------
//Put a "," between each value.  Default is "+"
#define         CRYPT_FORMAT_COMMA                  0x1000

//Put a ";" between each value
#define         CRYPT_FORMAT_SEMICOLON              CRYPT_FORMAT_RDN_SEMICOLON

//Put a "\n" between each value
#define         CRYPT_FORMAT_CRLF                   CRYPT_FORMAT_RDN_CRLF

//+-------------------------------------------------------------------------
//  Encode / decode the specified data structure according to the certificate
//  encoding type.
//
//  See below for a list of the predefined data structures.
//--------------------------------------------------------------------------

typedef LPVOID (WINAPI *PFN_CRYPT_ALLOC)(
    _In_ size_t cbSize
    );

typedef VOID (WINAPI *PFN_CRYPT_FREE)(
    _In_ LPVOID pv
    );


typedef struct _CRYPT_ENCODE_PARA {
    DWORD                   cbSize;
    PFN_CRYPT_ALLOC         pfnAlloc;           // OPTIONAL
    PFN_CRYPT_FREE          pfnFree;            // OPTIONAL
} CRYPT_ENCODE_PARA, *PCRYPT_ENCODE_PARA;


WINCRYPT32API
BOOL
WINAPI
CryptEncodeObjectEx(
    _In_ DWORD dwCertEncodingType,
    _In_ LPCSTR lpszStructType,
    _In_ const void *pvStructInfo,
    _In_ DWORD dwFlags,
    _In_opt_ PCRYPT_ENCODE_PARA pEncodePara,
    _Out_opt_ void *pvEncoded,
    _Inout_ DWORD *pcbEncoded
    );

WINCRYPT32API
BOOL
WINAPI
CryptEncodeObject(
    _In_ DWORD dwCertEncodingType,
    _In_ LPCSTR lpszStructType,
    _In_ const void *pvStructInfo,
    _Out_writes_bytes_to_opt_(*pcbEncoded, *pcbEncoded) BYTE *pbEncoded,
    _Inout_ DWORD *pcbEncoded
    );


// By default the signature bytes are reversed. The following flag can
// be set to inhibit the byte reversal.
//
// This flag is applicable to
//      X509_CERT_TO_BE_SIGNED
#define CRYPT_ENCODE_NO_SIGNATURE_BYTE_REVERSAL_FLAG    0x8


//  When the following flag is set the called encode function allocates
//  memory for the encoded bytes. A pointer to the allocated bytes
//  is returned in pvEncoded. If pEncodePara or pEncodePara->pfnAlloc is
//  NULL, then, LocalAlloc is called for the allocation and LocalFree must
//  be called to do the free. Otherwise, pEncodePara->pfnAlloc is called
//  for the allocation.
//
//  *pcbEncoded is ignored on input and updated with the length of the
//  allocated, encoded bytes.
//
//  If pfnAlloc is set, then, pfnFree should also be set.
#define CRYPT_ENCODE_ALLOC_FLAG             0x8000


//  The following flag is applicable when encoding X509_UNICODE_NAME.
//  When set, CERT_RDN_T61_STRING is selected instead of
//  CERT_RDN_UNICODE_STRING if all the unicode characters are <= 0xFF
#define CRYPT_UNICODE_NAME_ENCODE_ENABLE_T61_UNICODE_FLAG   \
            CERT_RDN_ENABLE_T61_UNICODE_FLAG

//  The following flag is applicable when encoding X509_UNICODE_NAME.
//  When set, CERT_RDN_UTF8_STRING is selected instead of
//  CERT_RDN_UNICODE_STRING.
#define CRYPT_UNICODE_NAME_ENCODE_ENABLE_UTF8_UNICODE_FLAG   \
            CERT_RDN_ENABLE_UTF8_UNICODE_FLAG

//  The following flag is applicable when encoding X509_UNICODE_NAME.
//  When set, CERT_RDN_UTF8_STRING is selected instead of
//  CERT_RDN_PRINTABLE_STRING for DirectoryString types. Also,
//  enables CRYPT_UNICODE_NAME_ENCODE_ENABLE_UTF8_UNICODE_FLAG.
#define CRYPT_UNICODE_NAME_ENCODE_FORCE_UTF8_UNICODE_FLAG     \
            CERT_RDN_FORCE_UTF8_UNICODE_FLAG

//  The following flag is applicable when encoding X509_UNICODE_NAME,
//  X509_UNICODE_NAME_VALUE or X509_UNICODE_ANY_STRING.
//  When set, the characters aren't checked to see if they
//  are valid for the specified Value Type.
#define CRYPT_UNICODE_NAME_ENCODE_DISABLE_CHECK_TYPE_FLAG   \
            CERT_RDN_DISABLE_CHECK_TYPE_FLAG

//  The following flag is applicable when encoding the PKCS_SORTED_CTL. This
//  flag should be set if the identifier for the TrustedSubjects is a hash,
//  such as, MD5 or SHA1.
#define CRYPT_SORTED_CTL_ENCODE_HASHED_SUBJECT_IDENTIFIER_FLAG     0x10000

// The following flag is applicable when encoding structures that require
// IA5String encoding of host name(in DNS Name/ URL/ EmailAddress) containing
// non-IA5 characters by encoding the host name in punycode first.
#define CRYPT_ENCODE_ENABLE_PUNYCODE_FLAG   0x20000

// The following flag is applicable when encoding structures that require
// IA5String encoding of a path (http URL/Ldap query) containing non-IA5
// characters by encoding the path part as UTF8 percent encoding.
#define CRYPT_ENCODE_ENABLE_UTF8PERCENT_FLAG   0x40000

// The following flag is applicable when encoding structures that require
// IA5String encoding of the host name (URL) and path. If the data to be encoded
// contains non-IA5 characters then using this flag in during encoding will cause
// the hostname to be punycode and the path as UTF8-percent encoding
// For example: http://www.zzzzzz.com/yyyyy/qqqqq/rrrrrr.sssss
// If zzzzzz contains non-IA5 characters then using this flag will punycode 
// encode the zzzzzz component.
// If yyyyy or qqqqq or rrrrrr or sssss contain non-IA5 characters then using 
// this flag will UTF8 percent encode those characters which are not IA5.
#define CRYPT_ENCODE_ENABLE_IA5CONVERSION_FLAG (CRYPT_ENCODE_ENABLE_PUNYCODE_FLAG | CRYPT_ENCODE_ENABLE_UTF8PERCENT_FLAG)

typedef struct _CRYPT_DECODE_PARA {
    DWORD                   cbSize;
    PFN_CRYPT_ALLOC         pfnAlloc;           // OPTIONAL
    PFN_CRYPT_FREE          pfnFree;            // OPTIONAL
} CRYPT_DECODE_PARA, *PCRYPT_DECODE_PARA;

WINCRYPT32API
BOOL
WINAPI
CryptDecodeObjectEx(
    _In_ DWORD dwCertEncodingType,
    _In_ LPCSTR lpszStructType,
    _In_reads_bytes_(cbEncoded) const BYTE *pbEncoded,
    _In_ DWORD cbEncoded,
    _In_ DWORD dwFlags,
    _In_opt_ PCRYPT_DECODE_PARA pDecodePara,
    _Out_opt_ void *pvStructInfo,
    _Inout_ DWORD *pcbStructInfo
    );


WINCRYPT32API
BOOL
WINAPI
CryptDecodeObject(
    _In_ DWORD dwCertEncodingType,
    _In_ LPCSTR lpszStructType,
    _In_reads_bytes_(cbEncoded) const BYTE *pbEncoded,
    _In_ DWORD cbEncoded,
    _In_ DWORD dwFlags,
    _Out_writes_bytes_to_opt_(*pcbStructInfo, *pcbStructInfo) void *pvStructInfo,
    _Inout_ DWORD *pcbStructInfo
    );

// When the following flag is set the nocopy optimization is enabled.
// This optimization where appropriate, updates the pvStructInfo fields
// to point to content residing within pbEncoded instead of making a copy
// of and appending to pvStructInfo.
//
// Note, when set, pbEncoded can't be freed until pvStructInfo is freed.
#define CRYPT_DECODE_NOCOPY_FLAG            0x1

// For CryptDecodeObject(), by default the pbEncoded is the "to be signed"
// plus its signature. Set the following flag, if pbEncoded points to only
// the "to be signed".
//
// This flag is applicable to
//      X509_CERT_TO_BE_SIGNED
//      X509_CERT_CRL_TO_BE_SIGNED
//      X509_CERT_REQUEST_TO_BE_SIGNED
//      X509_KEYGEN_REQUEST_TO_BE_SIGNED
#define CRYPT_DECODE_TO_BE_SIGNED_FLAG      0x2

// When the following flag is set, the OID strings are allocated in
// crypt32.dll and shared instead of being copied into the returned
// data structure. This flag may be set if crypt32.dll isn't unloaded
// before the caller is unloaded.
#define CRYPT_DECODE_SHARE_OID_STRING_FLAG  0x4

// By default the signature bytes are reversed. The following flag can
// be set to inhibit the byte reversal.
//
// This flag is applicable to
//      X509_CERT_TO_BE_SIGNED
#define CRYPT_DECODE_NO_SIGNATURE_BYTE_REVERSAL_FLAG    0x8


// When the following flag is set the called decode function allocates
// memory for the decoded structure. A pointer to the allocated structure
// is returned in pvStructInfo. If pDecodePara or pDecodePara->pfnAlloc is
// NULL, then, LocalAlloc is called for the allocation and LocalFree must
// be called to do the free. Otherwise, pDecodePara->pfnAlloc is called
// for the allocation.
//
// *pcbStructInfo is ignored on input and updated with the length of the
// allocated, decoded structure.
//
// This flag may also be set in the CryptDecodeObject API. Since
// CryptDecodeObject doesn't take a pDecodePara, LocalAlloc is always
// called for the allocation which must be freed by calling LocalFree.
#define CRYPT_DECODE_ALLOC_FLAG             0x8000

// The following flag is applicable when decoding X509_UNICODE_NAME,
// X509_UNICODE_NAME_VALUE or X509_UNICODE_ANY_STRING.
// By default, CERT_RDN_T61_STRING values are initially decoded
// as UTF8. If the UTF8 decoding fails, then, decoded as 8 bit characters.
// Setting this flag skips the initial attempt to decode as UTF8.
#define CRYPT_UNICODE_NAME_DECODE_DISABLE_IE4_UTF8_FLAG     \
            CERT_RDN_DISABLE_IE4_UTF8_FLAG

// The following flag is applicable when decoding structures that contain
// IA5String encoding of punycode encoded host name (in DNS Name/ URL/
// EmailAddress). Decoded value contains the the unicode equivalent of
// punycode encoded data.
#define CRYPT_DECODE_ENABLE_PUNYCODE_FLAG   0x02000000


// The following flag is applicable when decoding structures that contain
// IA5String that is UTF8 percent encoded in the path part of a url.
#define CRYPT_DECODE_ENABLE_UTF8PERCENT_FLAG 0x04000000

// The following flag is applicable when decoding structures that contain
// an IA5String that is a punycode and UTF8-percent encoded host name and path (URL). The decoded
// value contains the Unicode equivalent of the punycode encoded host name and UTF8 percent 
// encoded path.
#define CRYPT_DECODE_ENABLE_IA5CONVERSION_FLAG (CRYPT_DECODE_ENABLE_PUNYCODE_FLAG | CRYPT_DECODE_ENABLE_UTF8PERCENT_FLAG)



//+-------------------------------------------------------------------------
//  Predefined X509 certificate data structures that can be encoded / decoded.
//--------------------------------------------------------------------------
#define CRYPT_ENCODE_DECODE_NONE            0
#define X509_CERT                           ((LPCSTR) 1)
#define X509_CERT_TO_BE_SIGNED              ((LPCSTR) 2)
#define X509_CERT_CRL_TO_BE_SIGNED          ((LPCSTR) 3)
#define X509_CERT_REQUEST_TO_BE_SIGNED      ((LPCSTR) 4)
#define X509_NAME_VALUE                     ((LPCSTR) 6)
#define X509_PUBLIC_KEY_INFO                ((LPCSTR) 8)

// WINCRYPT_USE_SYMBOL_PREFIX defined to avoid symbol collision with OpenSSL
// Only used for specific symbols which have collisions today (Apr 2022) to avoid excessive duplication
#ifndef WINCRYPT_USE_SYMBOL_PREFIX
    #define X509_EXTENSIONS                 ((LPCSTR) 5)
    #define X509_NAME                       ((LPCSTR) 7)
#else
    #define WINCRYPT_X509_EXTENSIONS        ((LPCSTR) 5)
    #define WINCRYPT_X509_NAME              ((LPCSTR) 7)
#endif

//+-------------------------------------------------------------------------
//  Predefined X509 certificate extension data structures that can be
//  encoded / decoded.
//--------------------------------------------------------------------------
#define X509_AUTHORITY_KEY_ID               ((LPCSTR) 9)
#define X509_KEY_ATTRIBUTES                 ((LPCSTR) 10)
#define X509_KEY_USAGE_RESTRICTION          ((LPCSTR) 11)
#define X509_ALTERNATE_NAME                 ((LPCSTR) 12)
#define X509_BASIC_CONSTRAINTS              ((LPCSTR) 13)
#define X509_KEY_USAGE                      ((LPCSTR) 14)
#define X509_BASIC_CONSTRAINTS2             ((LPCSTR) 15)
#define X509_CERT_POLICIES                  ((LPCSTR) 16)

//+-------------------------------------------------------------------------
//  Additional predefined data structures that can be encoded / decoded.
//--------------------------------------------------------------------------
#define PKCS_UTC_TIME                       ((LPCSTR) 17)
#define PKCS_TIME_REQUEST                   ((LPCSTR) 18)
#define RSA_CSP_PUBLICKEYBLOB               ((LPCSTR) 19)
#define X509_UNICODE_NAME                   ((LPCSTR) 20)

#define X509_KEYGEN_REQUEST_TO_BE_SIGNED    ((LPCSTR) 21)
#define PKCS_ATTRIBUTE                      ((LPCSTR) 22)
#define PKCS_CONTENT_INFO_SEQUENCE_OF_ANY   ((LPCSTR) 23)

//+-------------------------------------------------------------------------
//  Predefined primitive data structures that can be encoded / decoded.
//--------------------------------------------------------------------------
#define X509_UNICODE_NAME_VALUE             ((LPCSTR) 24)
#define X509_ANY_STRING                     X509_NAME_VALUE
#define X509_UNICODE_ANY_STRING             X509_UNICODE_NAME_VALUE
#define X509_OCTET_STRING                   ((LPCSTR) 25)
#define X509_BITS                           ((LPCSTR) 26)
#define X509_INTEGER                        ((LPCSTR) 27)
#define X509_MULTI_BYTE_INTEGER             ((LPCSTR) 28)
#define X509_ENUMERATED                     ((LPCSTR) 29)
#define X509_CHOICE_OF_TIME                 ((LPCSTR) 30)

//+-------------------------------------------------------------------------
//  More predefined X509 certificate extension data structures that can be
//  encoded / decoded.
//--------------------------------------------------------------------------
#define X509_AUTHORITY_KEY_ID2              ((LPCSTR) 31)
#define X509_AUTHORITY_INFO_ACCESS          ((LPCSTR) 32)
#define X509_SUBJECT_INFO_ACCESS            X509_AUTHORITY_INFO_ACCESS
#define X509_CRL_REASON_CODE                X509_ENUMERATED
#define PKCS_CONTENT_INFO                   ((LPCSTR) 33)
#define X509_SEQUENCE_OF_ANY                ((LPCSTR) 34)
#define X509_CRL_DIST_POINTS                ((LPCSTR) 35)
#define X509_ENHANCED_KEY_USAGE             ((LPCSTR) 36)
#define PKCS_CTL                            ((LPCSTR) 37)

#define X509_MULTI_BYTE_UINT                ((LPCSTR) 38)
#define X509_DSS_PUBLICKEY                  X509_MULTI_BYTE_UINT
#define X509_DSS_PARAMETERS                 ((LPCSTR) 39)
#define X509_DSS_SIGNATURE                  ((LPCSTR) 40)
#define PKCS_RC2_CBC_PARAMETERS             ((LPCSTR) 41)
#define PKCS_SMIME_CAPABILITIES             ((LPCSTR) 42)

// Qualified Certificate Statements Extension uses the same encode/decode
// function as PKCS_SMIME_CAPABILITIES. Its data structures are identical
// except for the names of the fields.
#define X509_QC_STATEMENTS_EXT              ((LPCSTR) 42)

//+-------------------------------------------------------------------------
//  data structures for private keys
//--------------------------------------------------------------------------
#define PKCS_RSA_PRIVATE_KEY                ((LPCSTR) 43)
#define PKCS_PRIVATE_KEY_INFO               ((LPCSTR) 44)
#define PKCS_ENCRYPTED_PRIVATE_KEY_INFO     ((LPCSTR) 45)

//+-------------------------------------------------------------------------
//  certificate policy qualifier
//--------------------------------------------------------------------------
#define X509_PKIX_POLICY_QUALIFIER_USERNOTICE ((LPCSTR) 46)

//+-------------------------------------------------------------------------
//  Diffie-Hellman Key Exchange
//--------------------------------------------------------------------------
#define X509_DH_PUBLICKEY                   X509_MULTI_BYTE_UINT
#define X509_DH_PARAMETERS                  ((LPCSTR) 47)
#define PKCS_ATTRIBUTES                     ((LPCSTR) 48)
#define PKCS_SORTED_CTL                     ((LPCSTR) 49)

//+-------------------------------------------------------------------------
//  ECC Signature
//--------------------------------------------------------------------------
// Uses the same encode/decode function as X509_DH_PARAMETERS. Its data
// structure is identical except for the names of the fields.
#define X509_ECC_SIGNATURE                  ((LPCSTR) 47)

//+-------------------------------------------------------------------------
//  X942 Diffie-Hellman
//--------------------------------------------------------------------------
#define X942_DH_PARAMETERS                  ((LPCSTR) 50)

//+-------------------------------------------------------------------------
//  The following is the same as X509_BITS, except before encoding,
//  the bit length is decremented to exclude trailing zero bits.
//--------------------------------------------------------------------------
#define X509_BITS_WITHOUT_TRAILING_ZEROES   ((LPCSTR) 51)

//+-------------------------------------------------------------------------
//  X942 Diffie-Hellman Other Info
//--------------------------------------------------------------------------
#define X942_OTHER_INFO                     ((LPCSTR) 52)

#define X509_CERT_PAIR                      ((LPCSTR) 53)
#define X509_ISSUING_DIST_POINT             ((LPCSTR) 54)
#define X509_NAME_CONSTRAINTS               ((LPCSTR) 55)
#define X509_POLICY_MAPPINGS                ((LPCSTR) 56)
#define X509_POLICY_CONSTRAINTS             ((LPCSTR) 57)
#define X509_CROSS_CERT_DIST_POINTS         ((LPCSTR) 58)

//+-------------------------------------------------------------------------
//  Certificate Management Messages over CMS (CMC) Data Structures
//--------------------------------------------------------------------------
#define CMC_DATA                            ((LPCSTR) 59)
#define CMC_RESPONSE                        ((LPCSTR) 60)
#define CMC_STATUS                          ((LPCSTR) 61)
#define CMC_ADD_EXTENSIONS                  ((LPCSTR) 62)
#define CMC_ADD_ATTRIBUTES                  ((LPCSTR) 63)

//+-------------------------------------------------------------------------
//  Certificate Template
//--------------------------------------------------------------------------
#define X509_CERTIFICATE_TEMPLATE           ((LPCSTR) 64)

//+-------------------------------------------------------------------------
//  Online Certificate Status Protocol (OCSP) Data Structures
//--------------------------------------------------------------------------
#define OCSP_SIGNED_REQUEST                 ((LPCSTR) 65)
#define OCSP_BASIC_SIGNED_RESPONSE          ((LPCSTR) 68)
#define OCSP_BASIC_RESPONSE                 ((LPCSTR) 69)

// WINCRYPT_USE_SYMBOL_PREFIX defined to avoid symbol collision with OpenSSL
// Only used for specific symbols which have collisions today (Apr 2022) to avoid excessive duplication
#ifndef WINCRYPT_USE_SYMBOL_PREFIX
    #define OCSP_REQUEST                    ((LPCSTR) 66)
    #define OCSP_RESPONSE                   ((LPCSTR) 67)
#else
    #define WINCRYPT_OCSP_REQUEST           ((LPCSTR) 66)
    #define WINCRYPT_OCSP_RESPONSE          ((LPCSTR) 67)
#endif

//+-------------------------------------------------------------------------
//  Logotype and Biometric Extensions
//--------------------------------------------------------------------------
#define X509_LOGOTYPE_EXT                   ((LPCSTR) 70)
#define X509_BIOMETRIC_EXT                  ((LPCSTR) 71)

#define CNG_RSA_PUBLIC_KEY_BLOB             ((LPCSTR) 72)
#define X509_OBJECT_IDENTIFIER              ((LPCSTR) 73)
#define X509_ALGORITHM_IDENTIFIER           ((LPCSTR) 74)
#define PKCS_RSA_SSA_PSS_PARAMETERS         ((LPCSTR) 75)
#define PKCS_RSAES_OAEP_PARAMETERS          ((LPCSTR) 76)

#define ECC_CMS_SHARED_INFO                 ((LPCSTR) 77)

//+-------------------------------------------------------------------------
//  TIMESTAMP
//--------------------------------------------------------------------------
#define TIMESTAMP_REQUEST                  ((LPCSTR) 78)
#define TIMESTAMP_RESPONSE                 ((LPCSTR) 79)
#define TIMESTAMP_INFO                     ((LPCSTR) 80)

//+-------------------------------------------------------------------------
//  CertificateBundle
//--------------------------------------------------------------------------
#define X509_CERT_BUNDLE                   ((LPCSTR) 81)

//+-------------------------------------------------------------------------
//  ECC Keys
//--------------------------------------------------------------------------
#define X509_ECC_PRIVATE_KEY                ((LPCSTR) 82)   // CRYPT_ECC_PRIVATE_KEY_INFO

#define CNG_RSA_PRIVATE_KEY_BLOB            ((LPCSTR) 83)   // BCRYPT_RSAKEY_BLOB

//+-------------------------------------------------------------------------
//  Subject Directory Attributes extension
//--------------------------------------------------------------------------
#define X509_SUBJECT_DIR_ATTRS              ((LPCSTR) 84)

//+-------------------------------------------------------------------------
//  Generic ECC Parameters
//--------------------------------------------------------------------------
#define X509_ECC_PARAMETERS                 ((LPCSTR) 85)

//+-------------------------------------------------------------------------
//  Predefined PKCS #7 data structures that can be encoded / decoded.
//--------------------------------------------------------------------------

// WINCRYPT_USE_SYMBOL_PREFIX defined to avoid symbol collision with OpenSSL
// Only used for specific symbols which have collisions today (Apr 2022) to avoid excessive duplication
#ifndef WINCRYPT_USE_SYMBOL_PREFIX
    #define PKCS7_SIGNER_INFO               ((LPCSTR) 500)
#else
    #define WINCRYPT_PKCS7_SIGNER_INFO      ((LPCSTR) 500)
#endif

//+-------------------------------------------------------------------------
//  Predefined PKCS #7 data structures that can be encoded / decoded.
//--------------------------------------------------------------------------
#define CMS_SIGNER_INFO                     ((LPCSTR) 501)

//+-------------------------------------------------------------------------
//  Predefined Software Publishing Credential (SPC)  data structures that
//  can be encoded / decoded.
//
//  Predefined values: 2000 .. 2999
//
//  See spc.h for value and data structure definitions.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  Extension Object Identifiers
//--------------------------------------------------------------------------
#define szOID_AUTHORITY_KEY_IDENTIFIER  "2.5.29.1"
#define szOID_KEY_ATTRIBUTES            "2.5.29.2"
#define szOID_CERT_POLICIES_95          "2.5.29.3"
#define szOID_KEY_USAGE_RESTRICTION     "2.5.29.4"
#define szOID_SUBJECT_ALT_NAME          "2.5.29.7"
#define szOID_ISSUER_ALT_NAME           "2.5.29.8"
#define szOID_BASIC_CONSTRAINTS         "2.5.29.10"
#define szOID_KEY_USAGE                 "2.5.29.15"
#define szOID_PRIVATEKEY_USAGE_PERIOD   "2.5.29.16"
#define szOID_BASIC_CONSTRAINTS2        "2.5.29.19"

#define szOID_CERT_POLICIES             "2.5.29.32"
#define szOID_ANY_CERT_POLICY           "2.5.29.32.0"
#define szOID_INHIBIT_ANY_POLICY        "2.5.29.54"

#define szOID_AUTHORITY_KEY_IDENTIFIER2 "2.5.29.35"
#define szOID_SUBJECT_KEY_IDENTIFIER    "2.5.29.14"
#define szOID_SUBJECT_ALT_NAME2         "2.5.29.17"
#define szOID_ISSUER_ALT_NAME2          "2.5.29.18"
#define szOID_CRL_REASON_CODE           "2.5.29.21"
#define szOID_REASON_CODE_HOLD          "2.5.29.23"
#define szOID_CRL_DIST_POINTS           "2.5.29.31"
#define szOID_ENHANCED_KEY_USAGE        "2.5.29.37"

#define szOID_ANY_ENHANCED_KEY_USAGE    "2.5.29.37.0"

// szOID_CRL_NUMBER -- Base CRLs only.  Monotonically increasing sequence
// number for each CRL issued by a CA.
#define szOID_CRL_NUMBER                "2.5.29.20"
// szOID_DELTA_CRL_INDICATOR -- Delta CRLs only.  Marked critical.
// Contains the minimum base CRL Number that can be used with a delta CRL.
#define szOID_DELTA_CRL_INDICATOR       "2.5.29.27"
#define szOID_ISSUING_DIST_POINT        "2.5.29.28"
// szOID_FRESHEST_CRL -- Base CRLs only.  Formatted identically to a CDP
// extension that holds URLs to fetch the delta CRL.
#define szOID_FRESHEST_CRL              "2.5.29.46"
#define szOID_NAME_CONSTRAINTS          "2.5.29.30"

// Note on 1/1/2000 szOID_POLICY_MAPPINGS was changed from "2.5.29.5"
#define szOID_POLICY_MAPPINGS           "2.5.29.33"
#define szOID_LEGACY_POLICY_MAPPINGS    "2.5.29.5"
#define szOID_POLICY_CONSTRAINTS        "2.5.29.36"


// Microsoft PKCS10 Attributes
#define szOID_RENEWAL_CERTIFICATE           "1.3.6.1.4.1.311.13.1"
#define szOID_ENROLLMENT_NAME_VALUE_PAIR    "1.3.6.1.4.1.311.13.2.1"
#define szOID_ENROLLMENT_CSP_PROVIDER       "1.3.6.1.4.1.311.13.2.2"
#define szOID_OS_VERSION                    "1.3.6.1.4.1.311.13.2.3"

//
// Extension contain certificate type
#define szOID_ENROLLMENT_AGENT              "1.3.6.1.4.1.311.20.2.1"

// Internet Public Key Infrastructure (PKIX)
#define szOID_PKIX                      "1.3.6.1.5.5.7"
#define szOID_PKIX_PE                   "1.3.6.1.5.5.7.1"
#define szOID_AUTHORITY_INFO_ACCESS     "1.3.6.1.5.5.7.1.1"
#define szOID_SUBJECT_INFO_ACCESS       "1.3.6.1.5.5.7.1.11"
#define szOID_BIOMETRIC_EXT             "1.3.6.1.5.5.7.1.2"
#define szOID_QC_STATEMENTS_EXT         "1.3.6.1.5.5.7.1.3"
#define szOID_LOGOTYPE_EXT              "1.3.6.1.5.5.7.1.12"

// Following is encoded as a SEQUENCE OF INTEGER.
// For OCSP Must-Staple, one of the integers will be set to 5
// which corresponds to the OCSP status_request TLS extension,
// See RFC 7633 for more details.
#define szOID_TLS_FEATURES_EXT          "1.3.6.1.5.5.7.1.24"

// Microsoft extensions or attributes
#define szOID_CERT_EXTENSIONS           "1.3.6.1.4.1.311.2.1.14"
#define szOID_NEXT_UPDATE_LOCATION      "1.3.6.1.4.1.311.10.2"
#define szOID_REMOVE_CERTIFICATE            "1.3.6.1.4.1.311.10.8.1"
#define szOID_CROSS_CERT_DIST_POINTS    "1.3.6.1.4.1.311.10.9.1"

//  Microsoft PKCS #7 ContentType Object Identifiers
#define szOID_CTL                       "1.3.6.1.4.1.311.10.1"

//  Microsoft Sorted CTL Extension Object Identifier
#define szOID_SORTED_CTL                "1.3.6.1.4.1.311.10.1.1"

// serialized serial numbers for PRS
#ifndef szOID_SERIALIZED
#define szOID_SERIALIZED                "1.3.6.1.4.1.311.10.3.3.1"
#endif

// UPN principal name in SubjectAltName
#ifndef szOID_NT_PRINCIPAL_NAME
#define szOID_NT_PRINCIPAL_NAME         "1.3.6.1.4.1.311.20.2.3"
#endif

// Internationalized Email Address in SubjectAltName (OtherName:UTF8)
#ifndef szOID_INTERNATIONALIZED_EMAIL_ADDRESS
#define szOID_INTERNATIONALIZED_EMAIL_ADDRESS   "1.3.6.1.4.1.311.20.2.4"
#endif

// Windows product update unauthenticated attribute
#ifndef szOID_PRODUCT_UPDATE
#define szOID_PRODUCT_UPDATE            "1.3.6.1.4.1.311.31.1"
#endif

// CryptUI
#define szOID_ANY_APPLICATION_POLICY    "1.3.6.1.4.1.311.10.12.1"

//+-------------------------------------------------------------------------
//  Object Identifiers for use with Auto Enrollment
//--------------------------------------------------------------------------
#define szOID_AUTO_ENROLL_CTL_USAGE     "1.3.6.1.4.1.311.20.1"

// Extension contain certificate type
// AKA Certificate template extension (v1)
#define szOID_ENROLL_CERTTYPE_EXTENSION "1.3.6.1.4.1.311.20.2"


#define szOID_CERT_MANIFOLD             "1.3.6.1.4.1.311.20.3"

//+-------------------------------------------------------------------------
//  Object Identifiers for use with the MS Certificate Server
//--------------------------------------------------------------------------
#ifndef szOID_CERTSRV_CA_VERSION
#define szOID_CERTSRV_CA_VERSION        "1.3.6.1.4.1.311.21.1"
#endif


// szOID_CERTSRV_PREVIOUS_CERT_HASH -- Contains the sha1 hash of the previous
// version of the CA certificate.
#define szOID_CERTSRV_PREVIOUS_CERT_HASH    "1.3.6.1.4.1.311.21.2"

// szOID_CRL_VIRTUAL_BASE -- Delta CRLs only.  Contains the base CRL Number
// of the corresponding base CRL.
#define szOID_CRL_VIRTUAL_BASE          "1.3.6.1.4.1.311.21.3"

// szOID_CRL_NEXT_PUBLISH -- Contains the time when the next CRL is expected
// to be published.  This may be sooner than the CRL's NextUpdate field.
#define szOID_CRL_NEXT_PUBLISH          "1.3.6.1.4.1.311.21.4"

// Enhanced Key Usage for CA encryption certificate
#define szOID_KP_CA_EXCHANGE            "1.3.6.1.4.1.311.21.5"

// Enhanced Key Usage for Privacy CA encryption certificate
#define szOID_KP_PRIVACY_CA             "1.3.6.1.4.1.311.21.36"

// Enhanced Key Usage for key recovery agent certificate
#define szOID_KP_KEY_RECOVERY_AGENT     "1.3.6.1.4.1.311.21.6"

// Certificate template extension (v2)
#define szOID_CERTIFICATE_TEMPLATE      "1.3.6.1.4.1.311.21.7"

// The root oid for all enterprise specific oids
#define szOID_ENTERPRISE_OID_ROOT       "1.3.6.1.4.1.311.21.8"

// Dummy signing Subject RDN
#define szOID_RDN_DUMMY_SIGNER          "1.3.6.1.4.1.311.21.9"

// Application Policies extension -- same encoding as szOID_CERT_POLICIES
#define szOID_APPLICATION_CERT_POLICIES     "1.3.6.1.4.1.311.21.10"

// Application Policy Mappings -- same encoding as szOID_POLICY_MAPPINGS
#define szOID_APPLICATION_POLICY_MAPPINGS   "1.3.6.1.4.1.311.21.11"

// Application Policy Constraints -- same encoding as szOID_POLICY_CONSTRAINTS
#define szOID_APPLICATION_POLICY_CONSTRAINTS    "1.3.6.1.4.1.311.21.12"

#define szOID_ARCHIVED_KEY_ATTR                "1.3.6.1.4.1.311.21.13"
#define szOID_CRL_SELF_CDP                     "1.3.6.1.4.1.311.21.14"


// Requires all certificates below the root to have a non-empty intersecting
// issuance certificate policy usage.
#define szOID_REQUIRE_CERT_CHAIN_POLICY        "1.3.6.1.4.1.311.21.15"
#define szOID_ARCHIVED_KEY_CERT_HASH           "1.3.6.1.4.1.311.21.16"
#define szOID_ISSUED_CERT_HASH                 "1.3.6.1.4.1.311.21.17"

// Enhanced key usage for DS email replication
#define szOID_DS_EMAIL_REPLICATION             "1.3.6.1.4.1.311.21.19"

#define szOID_REQUEST_CLIENT_INFO              "1.3.6.1.4.1.311.21.20"
#define szOID_ENCRYPTED_KEY_HASH               "1.3.6.1.4.1.311.21.21"
#define szOID_CERTSRV_CROSSCA_VERSION          "1.3.6.1.4.1.311.21.22"

//+-------------------------------------------------------------------------
//  Object Identifiers for use with the MS Directory Service
//--------------------------------------------------------------------------
#define szOID_NTDS_REPLICATION      "1.3.6.1.4.1.311.25.1"
#define szOID_NTDS_CA_SECURITY_EXT  "1.3.6.1.4.1.311.25.2"    // OID arc for Microsoft CA custom security extension
#define szOID_NTDS_OBJECTSID        "1.3.6.1.4.1.311.25.2.1"  // OID for objectSid info

//+-------------------------------------------------------------------------
//  URI Prefixes for use with the MS Directory Service
//--------------------------------------------------------------------------
#define wszURI_NTDS_OBJECTSID_PREFIX L"tag:microsoft.com,2022-09-14:sid:" // URI for objectSid info in the SAN, to be followed by a string SID

//+-------------------------------------------------------------------------
//  Extension Object Identifiers
//--------------------------------------------------------------------------
#define szOID_SUBJECT_DIR_ATTRS         "2.5.29.9"

//+-------------------------------------------------------------------------
//  Enhanced Key Usage (Purpose) Object Identifiers
//--------------------------------------------------------------------------
#define szOID_PKIX_KP                   "1.3.6.1.5.5.7.3"

// Consistent key usage bits: DIGITAL_SIGNATURE, KEY_ENCIPHERMENT
// or KEY_AGREEMENT
#define szOID_PKIX_KP_SERVER_AUTH       "1.3.6.1.5.5.7.3.1"

// Consistent key usage bits: DIGITAL_SIGNATURE
#define szOID_PKIX_KP_CLIENT_AUTH       "1.3.6.1.5.5.7.3.2"

// Consistent key usage bits: DIGITAL_SIGNATURE
#define szOID_PKIX_KP_CODE_SIGNING      "1.3.6.1.5.5.7.3.3"

// Consistent key usage bits: DIGITAL_SIGNATURE, NON_REPUDIATION and/or
// (KEY_ENCIPHERMENT or KEY_AGREEMENT)
#define szOID_PKIX_KP_EMAIL_PROTECTION  "1.3.6.1.5.5.7.3.4"

// Consistent key usage bits: DIGITAL_SIGNATURE and/or
// (KEY_ENCIPHERMENT or KEY_AGREEMENT)
#define szOID_PKIX_KP_IPSEC_END_SYSTEM  "1.3.6.1.5.5.7.3.5"

// Consistent key usage bits: DIGITAL_SIGNATURE and/or
// (KEY_ENCIPHERMENT or KEY_AGREEMENT)
#define szOID_PKIX_KP_IPSEC_TUNNEL      "1.3.6.1.5.5.7.3.6"

// Consistent key usage bits: DIGITAL_SIGNATURE and/or
// (KEY_ENCIPHERMENT or KEY_AGREEMENT)
#define szOID_PKIX_KP_IPSEC_USER        "1.3.6.1.5.5.7.3.7"

// Consistent key usage bits: DIGITAL_SIGNATURE or NON_REPUDIATION
#define szOID_PKIX_KP_TIMESTAMP_SIGNING "1.3.6.1.5.5.7.3.8"

// OCSP response signer
#define szOID_PKIX_KP_OCSP_SIGNING      "1.3.6.1.5.5.7.3.9"

// Following extension is present to indicate no revocation checking
// for the OCSP signer certificate
#define szOID_PKIX_OCSP_NOCHECK         "1.3.6.1.5.5.7.48.1.5"

// OCSP Nonce
#define szOID_PKIX_OCSP_NONCE		"1.3.6.1.5.5.7.48.1.2"

// IKE (Internet Key Exchange) Intermediate KP for an IPsec end entity.
// Defined in draft-ietf-ipsec-pki-req-04.txt, December 14, 1999.
#define szOID_IPSEC_KP_IKE_INTERMEDIATE "1.3.6.1.5.5.8.2.2"


// iso (1) org (3) dod (6) internet (1) security (5) kerberosv5 (2) pkinit (3) 5
#define szOID_PKINIT_KP_KDC             "1.3.6.1.5.2.3.5"

//+-------------------------------------------------------------------------
//  Microsoft Enhanced Key Usage (Purpose) Object Identifiers
//+-------------------------------------------------------------------------

//  Signer of CTLs
#define szOID_KP_CTL_USAGE_SIGNING      "1.3.6.1.4.1.311.10.3.1"

//  Signer of TimeStamps
#define szOID_KP_TIME_STAMP_SIGNING     "1.3.6.1.4.1.311.10.3.2"

#ifndef szOID_SERVER_GATED_CRYPTO
#define szOID_SERVER_GATED_CRYPTO       "1.3.6.1.4.1.311.10.3.3"
#endif

#ifndef szOID_SGC_NETSCAPE
#define szOID_SGC_NETSCAPE              "2.16.840.1.113730.4.1"
#endif

#define szOID_KP_EFS                    "1.3.6.1.4.1.311.10.3.4"
#define szOID_EFS_RECOVERY              "1.3.6.1.4.1.311.10.3.4.1"

// Signed by Microsoft through hardware certification (WHQL)
#define szOID_WHQL_CRYPTO               "1.3.6.1.4.1.311.10.3.5"

// Signed by Microsoft after the developer attests it is valid (Attested WHQL)
#define szOID_ATTEST_WHQL_CRYPTO        "1.3.6.1.4.1.311.10.3.5.1"

// Signed by the NT5 build lab
#define szOID_NT5_CRYPTO                "1.3.6.1.4.1.311.10.3.6"

// Signed by and OEM of WHQL
#define szOID_OEM_WHQL_CRYPTO           "1.3.6.1.4.1.311.10.3.7"

// Signed by the Embedded NT
#define szOID_EMBEDDED_NT_CRYPTO        "1.3.6.1.4.1.311.10.3.8"

// Signer of a CTL containing trusted roots
#define szOID_ROOT_LIST_SIGNER      "1.3.6.1.4.1.311.10.3.9"

// Can sign cross-cert and subordinate CA requests with qualified
// subordination (name constraints, policy mapping, etc.)
#define szOID_KP_QUALIFIED_SUBORDINATION    "1.3.6.1.4.1.311.10.3.10"

// Can be used to encrypt/recover escrowed keys
#define szOID_KP_KEY_RECOVERY               "1.3.6.1.4.1.311.10.3.11"

// Signer of documents
#define szOID_KP_DOCUMENT_SIGNING           "1.3.6.1.4.1.311.10.3.12"


// The default WinVerifyTrust Authenticode policy is to treat all time stamped
// signatures as being valid forever. This OID limits the valid lifetime of the
// signature to the lifetime of the certificate. This allows timestamped
// signatures to expire. Normally this OID will be used in conjunction with
// szOID_PKIX_KP_CODE_SIGNING to indicate new time stamp semantics should be
// used. Support for this OID was added in WXP.
#define szOID_KP_LIFETIME_SIGNING           "1.3.6.1.4.1.311.10.3.13"

#define szOID_KP_MOBILE_DEVICE_SOFTWARE     "1.3.6.1.4.1.311.10.3.14"

#define szOID_KP_SMART_DISPLAY          "1.3.6.1.4.1.311.10.3.15"

#define szOID_KP_CSP_SIGNATURE          "1.3.6.1.4.1.311.10.3.16"

#define szOID_KP_FLIGHT_SIGNING         "1.3.6.1.4.1.311.10.3.27"

#define szOID_PLATFORM_MANIFEST_BINARY_ID   "1.3.6.1.4.1.311.10.3.28"

#ifndef szOID_DRM
#define szOID_DRM                       "1.3.6.1.4.1.311.10.5.1"
#endif


// Microsoft DRM EKU
#ifndef szOID_DRM_INDIVIDUALIZATION
#define szOID_DRM_INDIVIDUALIZATION "1.3.6.1.4.1.311.10.5.2"
#endif


#ifndef szOID_LICENSES
#define szOID_LICENSES                  "1.3.6.1.4.1.311.10.6.1"
#endif

#ifndef szOID_LICENSE_SERVER
#define szOID_LICENSE_SERVER            "1.3.6.1.4.1.311.10.6.2"
#endif

#ifndef szOID_KP_SMARTCARD_LOGON
#define szOID_KP_SMARTCARD_LOGON        "1.3.6.1.4.1.311.20.2.2"
#endif


#define szOID_KP_KERNEL_MODE_CODE_SIGNING   "1.3.6.1.4.1.311.61.1.1"

#define szOID_KP_KERNEL_MODE_TRUSTED_BOOT_SIGNING "1.3.6.1.4.1.311.61.4.1"

// Signer of CRL
#define szOID_REVOKED_LIST_SIGNER       "1.3.6.1.4.1.311.10.3.19"

// Signer of Kits-built code
#define szOID_WINDOWS_KITS_SIGNER       "1.3.6.1.4.1.311.10.3.20"

// Signer of Windows RT code
#define szOID_WINDOWS_RT_SIGNER         "1.3.6.1.4.1.311.10.3.21"

// Signer of Protected Process Light code
#define szOID_PROTECTED_PROCESS_LIGHT_SIGNER "1.3.6.1.4.1.311.10.3.22"

// Signer of Windows TCB code
#define szOID_WINDOWS_TCB_SIGNER        "1.3.6.1.4.1.311.10.3.23"

// Signer of Protected Process code
#define szOID_PROTECTED_PROCESS_SIGNER  "1.3.6.1.4.1.311.10.3.24"

// Signer of third-party components that are Windows in box
#define szOID_WINDOWS_THIRD_PARTY_COMPONENT_SIGNER  "1.3.6.1.4.1.311.10.3.25"

// Signed by the Windows Software Portal
#define szOID_WINDOWS_SOFTWARE_EXTENSION_SIGNER  "1.3.6.1.4.1.311.10.3.26"

// CTL containing disallowed entries
#define szOID_DISALLOWED_LIST           "1.3.6.1.4.1.311.10.3.30"

// Signer of a CTL containing Pin Rules.
// The szOID_ROOT_LIST_SIGNER OID can also be used
#define szOID_PIN_RULES_SIGNER          "1.3.6.1.4.1.311.10.3.31"

// CTL containing Site Pin Rules
#define szOID_PIN_RULES_CTL             "1.3.6.1.4.1.311.10.3.32"

// Pin Rules CTL extension
#define szOID_PIN_RULES_EXT             "1.3.6.1.4.1.311.10.3.33"

// SubjectAlgorithm for Pin Rules CTL entries
#define szOID_PIN_RULES_DOMAIN_NAME     "1.3.6.1.4.1.311.10.3.34"

// Pin Rules Log End Date CTL extension
#define szOID_PIN_RULES_LOG_END_DATE_EXT "1.3.6.1.4.1.311.10.3.35"

// Image can be executed in Isolated User Mode (IUM)
#define szOID_IUM_SIGNING               "1.3.6.1.4.1.311.10.3.37"

// Signed by Microsoft through EV hardware certification (EV WHQL)
#define szOID_EV_WHQL_CRYPTO            "1.3.6.1.4.1.311.10.3.39"

// Signer of Biometric code
#define szOID_BIOMETRIC_SIGNING         "1.3.6.1.4.1.311.10.3.41"

// Image can be executed in a VSM Enclave
#define szOID_ENCLAVE_SIGNING           "1.3.6.1.4.1.311.10.3.42"

// The following extension is set in the disallowed CTL to trigger
// a quicker sync of the autorootupdate CTL
#define szOID_SYNC_ROOT_CTL_EXT         "1.3.6.1.4.1.311.10.3.50"

// The following extension is set to identify flighted CTLs
#define szOID_FLIGHT_CTL_EXT            "1.3.6.1.4.1.311.10.3.51"

//The following extension contains the list of logging servers for CT
#define szOID_CERT_LOG_LIST_EXT          "1.3.6.1.4.1.311.10.3.52"

// CTL containing HPKP Domain Names
#define szOID_HPKP_DOMAIN_NAME_CTL      "1.3.6.1.4.1.311.10.3.60"
// SubjectAlgorithm for HPKP Domain CTL entries: szOID_PIN_RULES_DOMAIN_NAME

// CTL containing HPKP Header Values. Stored as an extension in the
// Hpkp Domain Name CTL. This OID is also used to identify
// the extension.
#define szOID_HPKP_HEADER_VALUE_CTL     "1.3.6.1.4.1.311.10.3.61"
// SubjectAlgorithm for HPKP Header Value CTL entries: szOID_NIST_sha256
// Only the first 16 bytes of the SHA256 hash are used


// HAL Extensions
#define szOID_KP_KERNEL_MODE_HAL_EXTENSION_SIGNING "1.3.6.1.4.1.311.61.5.1"

// Signer of Windows Store applications
#define szOID_WINDOWS_STORE_SIGNER      "1.3.6.1.4.1.311.76.3.1"

// Signer of dynamic code generators
#define szOID_DYNAMIC_CODE_GEN_SIGNER   "1.3.6.1.4.1.311.76.5.1"

// Signer of Microsoft code
#define szOID_MICROSOFT_PUBLISHER_SIGNER "1.3.6.1.4.1.311.76.8.1"

//+-------------------------------------------------------------------------
//  Microsoft Attribute Object Identifiers
//+-------------------------------------------------------------------------
#define szOID_YESNO_TRUST_ATTR          "1.3.6.1.4.1.311.10.4.1"
#define szOID_SITE_PIN_RULES_INDEX_ATTR "1.3.6.1.4.1.311.10.4.2"
#define szOID_SITE_PIN_RULES_FLAGS_ATTR "1.3.6.1.4.1.311.10.4.3"

#define SITE_PIN_RULES_ALL_SUBDOMAINS_FLAG  0x1

//+-------------------------------------------------------------------------
//  Qualifiers that may be part of the szOID_CERT_POLICIES and
//  szOID_CERT_POLICIES95 extensions
//+-------------------------------------------------------------------------
#define szOID_PKIX_POLICY_QUALIFIER_CPS               "1.3.6.1.5.5.7.2.1"
#define szOID_PKIX_POLICY_QUALIFIER_USERNOTICE        "1.3.6.1.5.5.7.2.2"

#define szOID_ROOT_PROGRAM_FLAGS                      "1.3.6.1.4.1.311.60.1.1"

//+-------------------------------------------------------------------------
//  Root program qualifier flags, used in pbData field of
//  CERT_POLICY_QUALIFIER_INFO structure.
//+-------------------------------------------------------------------------

// Validation of the Organization (O) field in the subject name meets
// Root Program Requirements for display.
#define CERT_ROOT_PROGRAM_FLAG_ORG          0x80

// Validation of the Locale (L), State (S), and Country (C) fields in
// the subject name meets Program Requirements for display.
#define CERT_ROOT_PROGRAM_FLAG_LSC          0x40

// Subject logotype
#define CERT_ROOT_PROGRAM_FLAG_SUBJECT_LOGO 0x20

// Validation of the OrganizationalUnit (OU) field in the subject name
// meets Root Program Requirements for display.
#define CERT_ROOT_PROGRAM_FLAG_OU           0x10

// Validation of the address field in the subject name meets Root
// Program Requirements for display.
#define CERT_ROOT_PROGRAM_FLAG_ADDRESS      0x08


// OID for old qualifer
#define szOID_CERT_POLICIES_95_QUALIFIER1             "2.16.840.1.113733.1.7.1.1"

//+=========================================================================
//  TPM Object Identifiers
//-=========================================================================

// Subject Alt Name Directory Name RDNs
#define szOID_RDN_TPM_MANUFACTURER          "2.23.133.2.1"
#define szOID_RDN_TPM_MODEL                 "2.23.133.2.2"
#define szOID_RDN_TPM_VERSION               "2.23.133.2.3"

#define szOID_RDN_TCG_PLATFORM_MANUFACTURER "2.23.133.2.4"
#define szOID_RDN_TCG_PLATFORM_MODEL        "2.23.133.2.5"
#define szOID_RDN_TCG_PLATFORM_VERSION      "2.23.133.2.6"

// TPM Manufacturer ASCII Hex Strings
//  AMD                     "AMD"   0x41 0x4D 0x44 0x00
//  Atmel                   "ATML"  0x41 0x54 0x4D 0x4C
//  Broadcom                "BRCM"  0x42 0x52 0x43 0x4D
//  Cisco                   "CSCO"  0x43 0x53 0x43 0x4F
//  Flyslice Technologies   "FLYS"  0x46 0x4C 0x59 0x53
//  HPE                     "HPE"   0x48 0x50 0x45 0x00
//  IBM                     "IBM"   0x49 0x42 0x4d 0x00
//  Infineon                "IFX"   0x49 0x46 0x58 0x00
//  Intel                   "INTC"  0x49 0x4E 0x54 0x43
//  Lenovo                  "LEN"   0x4C 0x45 0x4E 0x00
//  Microsoft               "MSFT"  0x4D 0x53 0x46 0x54
//  National Semiconductor  "NSM "  0x4E 0x53 0x4D 0x20
//  Nationz                 "NTZ"   0x4E 0x54 0x5A 0x00
//  Nuvoton Technology      "NTC"   0x4E 0x54 0x43 0x00
//  Qualcomm                "QCOM"  0x51 0x43 0x4F 0x4D
//  SMSC                    "SMSC"  0x53 0x4D 0x53 0x43
//  ST Microelectronics     "STM "  0x53 0x54 0x4D 0x20
//  Samsung                 "SMSN"  0x53 0x4D 0x53 0x4E
//  Sinosun                 "SNS"   0x53 0x4E 0x53 0x00
//  Texas Instruments       "TXN"   0x54 0x58 0x4E 0x00
//  Winbond                 "WEC"   0x57 0x45 0x43 0x00
//  Fuzhou Rockchip         "ROCC"  0x52 0x4F 0x43 0x43
//  Google                  "GOOG"  0x47 0x4F 0x4F 0x47
//  VMWare                  "VMW"   0x56 0x4D 0x57 0x00
//
// Obtained from: https://trustedcomputinggroup.org/wp-content/uploads/TCG-TPM-Vendor-ID-Registry-Version-1.02-Revision-1.00.pdf

#define szOID_CT_CERT_SCTLIST               "1.3.6.1.4.1.11129.2.4.2" // OCTET string


// pkcs10 attributes
#define szOID_ENROLL_EK_INFO                "1.3.6.1.4.1.311.21.23" // EKInfo
#define szOID_ENROLL_AIK_INFO		    "1.3.6.1.4.1.311.21.39" // EKInfo
#define szOID_ENROLL_ATTESTATION_STATEMENT  "1.3.6.1.4.1.311.21.24"

// pkcs10 and CMC Full Response Tagged Attribute containing the KSP name.
// Encoded as a unicode string, which must be null terminated.
// See CERT_RDN_UNICODE_STRING in the CERT_NAME_VALUE structure.
#define szOID_ENROLL_KSP_NAME               "1.3.6.1.4.1.311.21.25"

// CMC Full Response Tagged Attributes
#define szOID_ENROLL_EKPUB_CHALLENGE        "1.3.6.1.4.1.311.21.26"
#define szOID_ENROLL_CAXCHGCERT_HASH        "1.3.6.1.4.1.311.21.27"
#define szOID_ENROLL_ATTESTATION_CHALLENGE  "1.3.6.1.4.1.311.21.28"
#define szOID_ENROLL_ENCRYPTION_ALGORITHM   "1.3.6.1.4.1.311.21.29" // algorithm oid

// TPM certificate EKU OIDs
#define szOID_KP_TPM_EK_CERTIFICATE         "2.23.133.8.1"
#define szOID_KP_TPM_PLATFORM_CERTIFICATE   "2.23.133.8.2"
#define szOID_KP_TPM_AIK_CERTIFICATE        "2.23.133.8.3"

// EK validation Issuance Policy OIDs
#define szOID_ENROLL_EKVERIFYKEY            "1.3.6.1.4.1.311.21.30"
#define szOID_ENROLL_EKVERIFYCERT           "1.3.6.1.4.1.311.21.31"
#define szOID_ENROLL_EKVERIFYCREDS          "1.3.6.1.4.1.311.21.32"

// Signed decimal string encoded as a Printable String
#define szOID_ENROLL_SCEP_ERROR             "1.3.6.1.4.1.311.21.33" // HRESULT

// SCEP attestation attributes
#define szOID_ENROLL_SCEP_SERVER_STATE      "1.3.6.1.4.1.311.21.34" // blob
#define szOID_ENROLL_SCEP_CHALLENGE_ANSWER  "1.3.6.1.4.1.311.21.35" // blob
#define szOID_ENROLL_SCEP_CLIENT_REQUEST    "1.3.6.1.4.1.311.21.37" // Pkcs10
#define szOID_ENROLL_SCEP_SERVER_MESSAGE    "1.3.6.1.4.1.311.21.38" // String
#define szOID_ENROLL_SCEP_SERVER_SECRET     "1.3.6.1.4.1.311.21.40" // blob

// key affinity extension: ASN NULL in requests, SEQUENCE of ANY containing
// two OCTET strings in issued certs: a salt blob and a hash value.
#define szOID_ENROLL_KEY_AFFINITY           "1.3.6.1.4.1.311.21.41"

// SCEP pkcs10 attribute: signer cert thumbprint
#define szOID_ENROLL_SCEP_SIGNER_HASH       "1.3.6.1.4.1.311.21.42" // blob

// TPM line specific EK CA KeyId
#define szOID_ENROLL_EK_CA_KEYID            "1.3.6.1.4.1.311.21.43" // blob

// Subject Directory Attributes
#define szOID_ATTR_SUPPORTED_ALGORITHMS     "2.5.4.52"
#define szOID_ATTR_TPM_SPECIFICATION        "2.23.133.2.16"
#define szOID_ATTR_PLATFORM_SPECIFICATION   "2.23.133.2.17"
#define szOID_ATTR_TPM_SECURITY_ASSERTIONS  "2.23.133.2.18"

//+-------------------------------------------------------------------------
//  X509_CERT
//
//  The "to be signed" encoded content plus its signature. The ToBeSigned
//  content is the CryptEncodeObject() output for one of the following:
//  X509_CERT_TO_BE_SIGNED, X509_CERT_CRL_TO_BE_SIGNED or
//  X509_CERT_REQUEST_TO_BE_SIGNED.
//
//  pvStructInfo points to CERT_SIGNED_CONTENT_INFO.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_CERT_TO_BE_SIGNED
//
//  pvStructInfo points to CERT_INFO.
//
//  For CryptDecodeObject(), the pbEncoded is the "to be signed" plus its
//  signature (output of a X509_CERT CryptEncodeObject()).
//
//  For CryptEncodeObject(), the pbEncoded is just the "to be signed".
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_CERT_CRL_TO_BE_SIGNED
//
//  pvStructInfo points to CRL_INFO.
//
//  For CryptDecodeObject(), the pbEncoded is the "to be signed" plus its
//  signature (output of a X509_CERT CryptEncodeObject()).
//
//  For CryptEncodeObject(), the pbEncoded is just the "to be signed".
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_CERT_REQUEST_TO_BE_SIGNED
//
//  pvStructInfo points to CERT_REQUEST_INFO.
//
//  For CryptDecodeObject(), the pbEncoded is the "to be signed" plus its
//  signature (output of a X509_CERT CryptEncodeObject()).
//
//  For CryptEncodeObject(), the pbEncoded is just the "to be signed".
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_EXTENSIONS (WINCRYPT_X509_EXTENSIONS)
//  szOID_CERT_EXTENSIONS
//
//  pvStructInfo points to following CERT_EXTENSIONS.
//--------------------------------------------------------------------------
// certenrolls_begin -- CERT_CONTEXTS
typedef struct _CERT_EXTENSIONS {
    DWORD           cExtension;
    PCERT_EXTENSION rgExtension;
} CERT_EXTENSIONS, *PCERT_EXTENSIONS;
// certenrolls_end

//+-------------------------------------------------------------------------
//  X509_NAME_VALUE
//  X509_ANY_STRING
//
//  pvStructInfo points to CERT_NAME_VALUE.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_UNICODE_NAME_VALUE
//  X509_UNICODE_ANY_STRING
//
//  pvStructInfo points to CERT_NAME_VALUE.
//
//  The name values are unicode strings.
//
//  For CryptEncodeObject:
//    Value.pbData points to the unicode string.
//    If Value.cbData = 0, then, the unicode string is NULL terminated.
//    Otherwise, Value.cbData is the unicode string byte count. The byte count
//    is twice the character count.
//
//    If the unicode string contains an invalid character for the specified
//    dwValueType, then, *pcbEncoded is updated with the unicode character
//    index of the first invalid character. LastError is set to:
//    CRYPT_E_INVALID_NUMERIC_STRING, CRYPT_E_INVALID_PRINTABLE_STRING or
//    CRYPT_E_INVALID_IA5_STRING.
//
//    To disable the above check, either set CERT_RDN_DISABLE_CHECK_TYPE_FLAG
//    in dwValueType or set CRYPT_UNICODE_NAME_ENCODE_DISABLE_CHECK_TYPE_FLAG
//    in dwFlags passed to CryptEncodeObjectEx.
//
//    The unicode string is converted before being encoded according to
//    the specified dwValueType. If dwValueType is set to 0, LastError
//    is set to E_INVALIDARG.
//
//    If the dwValueType isn't one of the character strings (its a
//    CERT_RDN_ENCODED_BLOB or CERT_RDN_OCTET_STRING), then, CryptEncodeObject
//    will return FALSE with LastError set to CRYPT_E_NOT_CHAR_STRING.
//
//  For CryptDecodeObject:
//    Value.pbData points to a NULL terminated unicode string. Value.cbData
//    contains the byte count of the unicode string excluding the NULL
//    terminator. dwValueType contains the type used in the encoded object.
//    Its not forced to CERT_RDN_UNICODE_STRING. The encoded value is
//    converted to the unicode string according to the dwValueType.
//
//    If the encoded object isn't one of the character string types, then,
//    CryptDecodeObject will return FALSE with LastError set to
//    CRYPT_E_NOT_CHAR_STRING. For a non character string, decode using
//    X509_NAME_VALUE or X509_ANY_STRING.
//
//    By default, CERT_RDN_T61_STRING values are initially decoded
//    as UTF8. If the UTF8 decoding fails, then, decoded as 8 bit characters.
//    Set CRYPT_UNICODE_NAME_DECODE_DISABLE_IE4_UTF8_FLAG in dwFlags
//    passed to either CryptDecodeObject or CryptDecodeObjectEx to
//    skip the initial attempt to decode as UTF8.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_NAME (WINCRYPT_X509_NAME)
//
//  pvStructInfo points to CERT_NAME_INFO.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_UNICODE_NAME
//
//  pvStructInfo points to CERT_NAME_INFO.
//
//  The RDN attribute values are unicode strings except for the dwValueTypes of
//  CERT_RDN_ENCODED_BLOB or CERT_RDN_OCTET_STRING. These dwValueTypes are
//  the same as for a X509_NAME. Their values aren't converted to/from unicode.
//
//  For CryptEncodeObject:
//    Value.pbData points to the unicode string.
//    If Value.cbData = 0, then, the unicode string is NULL terminated.
//    Otherwise, Value.cbData is the unicode string byte count. The byte count
//    is twice the character count.
//
//    If dwValueType = 0 (CERT_RDN_ANY_TYPE), the pszObjId is used to find
//    an acceptable dwValueType. If the unicode string contains an
//    invalid character for the found or specified dwValueType, then,
//    *pcbEncoded is updated with the error location of the invalid character.
//    See below for details. LastError is set to:
//    CRYPT_E_INVALID_NUMERIC_STRING, CRYPT_E_INVALID_PRINTABLE_STRING or
//    CRYPT_E_INVALID_IA5_STRING.
//
//    To disable the above check, either set CERT_RDN_DISABLE_CHECK_TYPE_FLAG
//    in dwValueType or set CRYPT_UNICODE_NAME_ENCODE_DISABLE_CHECK_TYPE_FLAG
//    in dwFlags passed to CryptEncodeObjectEx.
//
//    Set CERT_RDN_UNICODE_STRING in dwValueType or set
//    CRYPT_UNICODE_NAME_ENCODE_ENABLE_T61_UNICODE_FLAG in dwFlags passed
//    to CryptEncodeObjectEx to select CERT_RDN_T61_STRING instead of
//    CERT_RDN_UNICODE_STRING if all the unicode characters are <= 0xFF.
//
//    Set CERT_RDN_ENABLE_UTF8_UNICODE_STRING in dwValueType or set
//    CRYPT_UNICODE_NAME_ENCODE_ENABLE_UTF8_UNICODE_FLAG in dwFlags passed
//    to CryptEncodeObjectEx to select CERT_RDN_UTF8_STRING instead of
//    CERT_RDN_UNICODE_STRING.
//
//    The unicode string is converted before being encoded according to
//    the specified or ObjId matching dwValueType.
//
//  For CryptDecodeObject:
//    Value.pbData points to a NULL terminated unicode string. Value.cbData
//    contains the byte count of the unicode string excluding the NULL
//    terminator. dwValueType contains the type used in the encoded object.
//    Its not forced to CERT_RDN_UNICODE_STRING. The encoded value is
//    converted to the unicode string according to the dwValueType.
//
//    If the dwValueType of the encoded value isn't a character string
//    type, then, it isn't converted to UNICODE. Use the
//    IS_CERT_RDN_CHAR_STRING() macro on the dwValueType to check
//    that Value.pbData points to a converted unicode string.
//
//    By default, CERT_RDN_T61_STRING values are initially decoded
//    as UTF8. If the UTF8 decoding fails, then, decoded as 8 bit characters.
//    Set CRYPT_UNICODE_NAME_DECODE_DISABLE_IE4_UTF8_FLAG in dwFlags
//    passed to either CryptDecodeObject or CryptDecodeObjectEx to
//    skip the initial attempt to decode as UTF8.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  Unicode Name Value Error Location Definitions
//
//  Error location is returned in *pcbEncoded by
//  CryptEncodeObject(X509_UNICODE_NAME)
//
//  Error location consists of:
//    RDN_INDEX     - 10 bits << 22
//    ATTR_INDEX    - 6 bits << 16
//    VALUE_INDEX   - 16 bits (unicode character index)
//--------------------------------------------------------------------------
#define CERT_UNICODE_RDN_ERR_INDEX_MASK     0x3FF
#define CERT_UNICODE_RDN_ERR_INDEX_SHIFT    22
#define CERT_UNICODE_ATTR_ERR_INDEX_MASK    0x003F
#define CERT_UNICODE_ATTR_ERR_INDEX_SHIFT   16
#define CERT_UNICODE_VALUE_ERR_INDEX_MASK   0x0000FFFF
#define CERT_UNICODE_VALUE_ERR_INDEX_SHIFT  0

#define GET_CERT_UNICODE_RDN_ERR_INDEX(X)   \
    ((X >> CERT_UNICODE_RDN_ERR_INDEX_SHIFT) & CERT_UNICODE_RDN_ERR_INDEX_MASK)
#define GET_CERT_UNICODE_ATTR_ERR_INDEX(X)  \
    ((X >> CERT_UNICODE_ATTR_ERR_INDEX_SHIFT) & CERT_UNICODE_ATTR_ERR_INDEX_MASK)
#define GET_CERT_UNICODE_VALUE_ERR_INDEX(X) \
    (X & CERT_UNICODE_VALUE_ERR_INDEX_MASK)

//+-------------------------------------------------------------------------
//  X509_PUBLIC_KEY_INFO
//
//  pvStructInfo points to CERT_PUBLIC_KEY_INFO.
//--------------------------------------------------------------------------


//+-------------------------------------------------------------------------
//  X509_AUTHORITY_KEY_ID
//  szOID_AUTHORITY_KEY_IDENTIFIER
//
//  pvStructInfo points to following CERT_AUTHORITY_KEY_ID_INFO.
//--------------------------------------------------------------------------
typedef struct _CERT_AUTHORITY_KEY_ID_INFO {
    CRYPT_DATA_BLOB     KeyId;
    CERT_NAME_BLOB      CertIssuer;
    CRYPT_INTEGER_BLOB  CertSerialNumber;
} CERT_AUTHORITY_KEY_ID_INFO, *PCERT_AUTHORITY_KEY_ID_INFO;

//+-------------------------------------------------------------------------
//  X509_KEY_ATTRIBUTES
//  szOID_KEY_ATTRIBUTES
//
//  pvStructInfo points to following CERT_KEY_ATTRIBUTES_INFO.
//--------------------------------------------------------------------------
typedef struct _CERT_PRIVATE_KEY_VALIDITY {
    FILETIME            NotBefore;
    FILETIME            NotAfter;
} CERT_PRIVATE_KEY_VALIDITY, *PCERT_PRIVATE_KEY_VALIDITY;

typedef struct _CERT_KEY_ATTRIBUTES_INFO {
    CRYPT_DATA_BLOB             KeyId;
    CRYPT_BIT_BLOB              IntendedKeyUsage;
    PCERT_PRIVATE_KEY_VALIDITY  pPrivateKeyUsagePeriod;     // OPTIONAL
} CERT_KEY_ATTRIBUTES_INFO, *PCERT_KEY_ATTRIBUTES_INFO;

// certenrolld_begin -- CERT_*_KEY_USAGE
// Byte[0]
#define CERT_DIGITAL_SIGNATURE_KEY_USAGE     0x80
#define CERT_NON_REPUDIATION_KEY_USAGE       0x40
#define CERT_KEY_ENCIPHERMENT_KEY_USAGE      0x20
#define CERT_DATA_ENCIPHERMENT_KEY_USAGE     0x10
#define CERT_KEY_AGREEMENT_KEY_USAGE         0x08
#define CERT_KEY_CERT_SIGN_KEY_USAGE         0x04
#define CERT_OFFLINE_CRL_SIGN_KEY_USAGE      0x02
#define CERT_CRL_SIGN_KEY_USAGE              0x02
#define CERT_ENCIPHER_ONLY_KEY_USAGE         0x01
// Byte[1]
#define CERT_DECIPHER_ONLY_KEY_USAGE         0x80
// certenrolld_end

//+-------------------------------------------------------------------------
//  X509_KEY_USAGE_RESTRICTION
//  szOID_KEY_USAGE_RESTRICTION
//
//  pvStructInfo points to following CERT_KEY_USAGE_RESTRICTION_INFO.
//--------------------------------------------------------------------------
typedef struct _CERT_POLICY_ID {
    DWORD                   cCertPolicyElementId;
    LPSTR                   *rgpszCertPolicyElementId;  // pszObjId
} CERT_POLICY_ID, *PCERT_POLICY_ID;

typedef struct _CERT_KEY_USAGE_RESTRICTION_INFO {
    DWORD                   cCertPolicyId;
    PCERT_POLICY_ID         rgCertPolicyId;
    CRYPT_BIT_BLOB          RestrictedKeyUsage;
} CERT_KEY_USAGE_RESTRICTION_INFO, *PCERT_KEY_USAGE_RESTRICTION_INFO;

// See CERT_KEY_ATTRIBUTES_INFO for definition of the RestrictedKeyUsage bits

//+-------------------------------------------------------------------------
//  X509_ALTERNATE_NAME
//  szOID_SUBJECT_ALT_NAME
//  szOID_ISSUER_ALT_NAME
//  szOID_SUBJECT_ALT_NAME2
//  szOID_ISSUER_ALT_NAME2
//
//  pvStructInfo points to following CERT_ALT_NAME_INFO.
//--------------------------------------------------------------------------

// certenrolls_begin -- CERT_ALT_NAME_INFO
typedef struct _CERT_OTHER_NAME {
    LPSTR               pszObjId;
    CRYPT_OBJID_BLOB    Value;
} CERT_OTHER_NAME, *PCERT_OTHER_NAME;

typedef struct _CERT_ALT_NAME_ENTRY {
    DWORD   dwAltNameChoice;
    union {                                             // certenrolls_skip
        PCERT_OTHER_NAME            pOtherName;         // 1
        LPWSTR                      pwszRfc822Name;     // 2  (encoded IA5)
        LPWSTR                      pwszDNSName;        // 3  (encoded IA5)
        // Not implemented          x400Address;        // 4
        CERT_NAME_BLOB              DirectoryName;      // 5
        // Not implemented          pEdiPartyName;      // 6
        LPWSTR                      pwszURL;            // 7  (encoded IA5)
        CRYPT_DATA_BLOB             IPAddress;          // 8  (Octet String)
        LPSTR                       pszRegisteredID;    // 9  (Object Identifer)
    } DUMMYUNIONNAME;                                   // certenrolls_skip
} CERT_ALT_NAME_ENTRY, *PCERT_ALT_NAME_ENTRY;
// certenrolls_end

// certenrolld_begin -- CERT_ALT_NAME_*
#define CERT_ALT_NAME_OTHER_NAME         1
#define CERT_ALT_NAME_RFC822_NAME        2
#define CERT_ALT_NAME_DNS_NAME           3
#define CERT_ALT_NAME_X400_ADDRESS       4
#define CERT_ALT_NAME_DIRECTORY_NAME     5
#define CERT_ALT_NAME_EDI_PARTY_NAME     6
#define CERT_ALT_NAME_URL                7
#define CERT_ALT_NAME_IP_ADDRESS         8
#define CERT_ALT_NAME_REGISTERED_ID      9
// certenrolld_end

// certenrolls_begin -- CERT_ALT_NAME_INFO
typedef struct _CERT_ALT_NAME_INFO {
    DWORD                   cAltEntry;
    PCERT_ALT_NAME_ENTRY    rgAltEntry;
} CERT_ALT_NAME_INFO, *PCERT_ALT_NAME_INFO;
// certenrolls_end

//+-------------------------------------------------------------------------
//  Alternate name IA5 Error Location Definitions for
//  CRYPT_E_INVALID_IA5_STRING.
//
//  Error location is returned in *pcbEncoded by
//  CryptEncodeObject(X509_ALTERNATE_NAME)
//
//  Error location consists of:
//    ENTRY_INDEX   - 8 bits << 16
//    VALUE_INDEX   - 16 bits (unicode character index)
//--------------------------------------------------------------------------
#define CERT_ALT_NAME_ENTRY_ERR_INDEX_MASK  0xFF
#define CERT_ALT_NAME_ENTRY_ERR_INDEX_SHIFT 16
#define CERT_ALT_NAME_VALUE_ERR_INDEX_MASK  0x0000FFFF
#define CERT_ALT_NAME_VALUE_ERR_INDEX_SHIFT 0

#define GET_CERT_ALT_NAME_ENTRY_ERR_INDEX(X)   \
    ((X >> CERT_ALT_NAME_ENTRY_ERR_INDEX_SHIFT) & \
    CERT_ALT_NAME_ENTRY_ERR_INDEX_MASK)
#define GET_CERT_ALT_NAME_VALUE_ERR_INDEX(X) \
    (X & CERT_ALT_NAME_VALUE_ERR_INDEX_MASK)


//+-------------------------------------------------------------------------
//  X509_BASIC_CONSTRAINTS
//  szOID_BASIC_CONSTRAINTS
//
//  pvStructInfo points to following CERT_BASIC_CONSTRAINTS_INFO.
//--------------------------------------------------------------------------
typedef struct _CERT_BASIC_CONSTRAINTS_INFO {
    CRYPT_BIT_BLOB          SubjectType;
    BOOL                    fPathLenConstraint;
    DWORD                   dwPathLenConstraint;
    DWORD                   cSubtreesConstraint;
    CERT_NAME_BLOB          *rgSubtreesConstraint;
} CERT_BASIC_CONSTRAINTS_INFO, *PCERT_BASIC_CONSTRAINTS_INFO;

#define CERT_CA_SUBJECT_FLAG         0x80
#define CERT_END_ENTITY_SUBJECT_FLAG 0x40

//+-------------------------------------------------------------------------
//  X509_BASIC_CONSTRAINTS2
//  szOID_BASIC_CONSTRAINTS2
//
//  pvStructInfo points to following CERT_BASIC_CONSTRAINTS2_INFO.
//--------------------------------------------------------------------------
typedef struct _CERT_BASIC_CONSTRAINTS2_INFO {
    BOOL                    fCA;
    BOOL                    fPathLenConstraint;
    DWORD                   dwPathLenConstraint;
} CERT_BASIC_CONSTRAINTS2_INFO, *PCERT_BASIC_CONSTRAINTS2_INFO;

//+-------------------------------------------------------------------------
//  X509_KEY_USAGE
//  szOID_KEY_USAGE
//
//  pvStructInfo points to a CRYPT_BIT_BLOB. Has same bit definitions as
//  CERT_KEY_ATTRIBUTES_INFO's IntendedKeyUsage.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_CERT_POLICIES
//  szOID_CERT_POLICIES
//  szOID_CERT_POLICIES_95   NOTE--Only allowed for decoding!!!
//
//  pvStructInfo points to following CERT_POLICIES_INFO.
//
//  NOTE: when decoding using szOID_CERT_POLICIES_95 the pszPolicyIdentifier
//        may contain an empty string
//--------------------------------------------------------------------------
// certenrolls_begin -- CERT_POLICY_QUALIFIER_INFO
typedef struct _CERT_POLICY_QUALIFIER_INFO {
    LPSTR                       pszPolicyQualifierId;   // pszObjId
    CRYPT_OBJID_BLOB            Qualifier;              // optional
} CERT_POLICY_QUALIFIER_INFO, *PCERT_POLICY_QUALIFIER_INFO;

typedef struct _CERT_POLICY_INFO {
    LPSTR                       pszPolicyIdentifier;    // pszObjId
    DWORD                       cPolicyQualifier;       // optional
    CERT_POLICY_QUALIFIER_INFO  *rgPolicyQualifier;
} CERT_POLICY_INFO, *PCERT_POLICY_INFO;

typedef struct _CERT_POLICIES_INFO {
    DWORD                       cPolicyInfo;
    CERT_POLICY_INFO            *rgPolicyInfo;
} CERT_POLICIES_INFO, *PCERT_POLICIES_INFO;
// certenrolls_end

//+-------------------------------------------------------------------------
//  X509_PKIX_POLICY_QUALIFIER_USERNOTICE
//  szOID_PKIX_POLICY_QUALIFIER_USERNOTICE
//
//  pvStructInfo points to following CERT_POLICY_QUALIFIER_USER_NOTICE.
//
//--------------------------------------------------------------------------
typedef struct _CERT_POLICY_QUALIFIER_NOTICE_REFERENCE {
    LPSTR   pszOrganization;
    DWORD   cNoticeNumbers;
    int     *rgNoticeNumbers;
} CERT_POLICY_QUALIFIER_NOTICE_REFERENCE, *PCERT_POLICY_QUALIFIER_NOTICE_REFERENCE;

typedef struct _CERT_POLICY_QUALIFIER_USER_NOTICE {
    CERT_POLICY_QUALIFIER_NOTICE_REFERENCE  *pNoticeReference;  // optional
    LPWSTR                                  pszDisplayText;     // optional
} CERT_POLICY_QUALIFIER_USER_NOTICE, *PCERT_POLICY_QUALIFIER_USER_NOTICE;

//+-------------------------------------------------------------------------
//  szOID_CERT_POLICIES_95_QUALIFIER1 - Decode Only!!!!
//
//  pvStructInfo points to following CERT_POLICY95_QUALIFIER1.
//
//--------------------------------------------------------------------------
typedef struct _CPS_URLS {
    LPWSTR                      pszURL;
    CRYPT_ALGORITHM_IDENTIFIER  *pAlgorithm; // optional
    CRYPT_DATA_BLOB             *pDigest;    // optional
} CPS_URLS, *PCPS_URLS;

typedef struct _CERT_POLICY95_QUALIFIER1 {
    LPWSTR      pszPracticesReference;      // optional
    LPSTR       pszNoticeIdentifier;        // optional
    LPSTR       pszNSINoticeIdentifier;     // optional
    DWORD       cCPSURLs;
    CPS_URLS    *rgCPSURLs;                 // optional
} CERT_POLICY95_QUALIFIER1, *PCERT_POLICY95_QUALIFIER1;

//+-------------------------------------------------------------------------
//  szOID_INHIBIT_ANY_POLICY data structure
//
//  pvStructInfo points to an int.
//--------------------------------------------------------------------------


//+-------------------------------------------------------------------------
//  X509_POLICY_MAPPINGS
//  szOID_POLICY_MAPPINGS
//  szOID_LEGACY_POLICY_MAPPINGS
//
//  pvStructInfo points to following CERT_POLICY_MAPPINGS_INFO.
//--------------------------------------------------------------------------
typedef struct _CERT_POLICY_MAPPING {
    LPSTR                       pszIssuerDomainPolicy;      // pszObjId
    LPSTR                       pszSubjectDomainPolicy;     // pszObjId
} CERT_POLICY_MAPPING, *PCERT_POLICY_MAPPING;

typedef struct _CERT_POLICY_MAPPINGS_INFO {
    DWORD                       cPolicyMapping;
    PCERT_POLICY_MAPPING        rgPolicyMapping;
} CERT_POLICY_MAPPINGS_INFO, *PCERT_POLICY_MAPPINGS_INFO;

//+-------------------------------------------------------------------------
//  X509_POLICY_CONSTRAINTS
//  szOID_POLICY_CONSTRAINTS
//
//  pvStructInfo points to following CERT_POLICY_CONSTRAINTS_INFO.
//--------------------------------------------------------------------------
typedef struct _CERT_POLICY_CONSTRAINTS_INFO {
    BOOL                        fRequireExplicitPolicy;
    DWORD                       dwRequireExplicitPolicySkipCerts;

    BOOL                        fInhibitPolicyMapping;
    DWORD                       dwInhibitPolicyMappingSkipCerts;
} CERT_POLICY_CONSTRAINTS_INFO, *PCERT_POLICY_CONSTRAINTS_INFO;

//+-------------------------------------------------------------------------
//  RSA_CSP_PUBLICKEYBLOB
//
//  pvStructInfo points to a PUBLICKEYSTRUC immediately followed by a
//  RSAPUBKEY and the modulus bytes.
//
//  CryptExportKey outputs the above StructInfo for a dwBlobType of
//  PUBLICKEYBLOB. CryptImportKey expects the above StructInfo when
//  importing a public key.
//
//  For dwCertEncodingType = X509_ASN_ENCODING, the RSA_CSP_PUBLICKEYBLOB is
//  encoded as a PKCS #1 RSAPublicKey consisting of a SEQUENCE of a
//  modulus INTEGER and a publicExponent INTEGER. The modulus is encoded
//  as being a unsigned integer. When decoded, if the modulus was encoded
//  as unsigned integer with a leading 0 byte, the 0 byte is removed before
//  converting to the CSP modulus bytes.
//
//  For decode, the aiKeyAlg field of PUBLICKEYSTRUC is always set to
//  CALG_RSA_KEYX.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CNG_RSA_PUBLIC_KEY_BLOB
//
//  pvStructInfo points to a BCRYPT_RSAKEY_BLOB immediately followed by the
//  exponent and the modulus bytes. Both the exponent and modulus are
//  big endian. The private key fields consisting of cbPrime1 and cbPrime2
//  are set to zero.
//
//  For dwCertEncodingType = X509_ASN_ENCODING, the CNG_RSA_PUBLIC_KEY_BLOB is
//  encoded as a PKCS #1 RSAPublicKey consisting of a SEQUENCE of a
//  modulus HUGEINTEGER and a publicExponent HUGEINTEGER.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_KEYGEN_REQUEST_TO_BE_SIGNED
//
//  pvStructInfo points to CERT_KEYGEN_REQUEST_INFO.
//
//  For CryptDecodeObject(), the pbEncoded is the "to be signed" plus its
//  signature (output of a X509_CERT CryptEncodeObject()).
//
//  For CryptEncodeObject(), the pbEncoded is just the "to be signed".
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  PKCS_ATTRIBUTE data structure
//
//  pvStructInfo points to a CRYPT_ATTRIBUTE.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  PKCS_ATTRIBUTES data structure
//
//  pvStructInfo points to a CRYPT_ATTRIBUTES.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  szOID_SUBJECT_DIR_ATTRS
//  X509_SUBJECT_DIR_ATTRS
//
//  pvStructInfo points to a CRYPT_ATTRIBUTES.
//
//  Encoded as a "SEQUENCE OF" instead of "SET OF"
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_ECC_PARAMETERS
//
//  pvStructInfo points to a CRYPT_DATA_BLOB which contains CNG Parameters
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  PKCS_CONTENT_INFO_SEQUENCE_OF_ANY data structure
//
//  pvStructInfo points to following CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY.
//
//  For X509_ASN_ENCODING: encoded as a PKCS#7 ContentInfo structure wrapping
//  a sequence of ANY. The value of the contentType field is pszObjId,
//  while the content field is the following structure:
//      SequenceOfAny ::= SEQUENCE OF ANY
//
//  The CRYPT_DER_BLOBs point to the already encoded ANY content.
//--------------------------------------------------------------------------
typedef struct _CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY {
    LPSTR               pszObjId;
    DWORD               cValue;
    PCRYPT_DER_BLOB     rgValue;
} CRYPT_CONTENT_INFO_SEQUENCE_OF_ANY, *PCRYPT_CONTENT_INFO_SEQUENCE_OF_ANY;

//+-------------------------------------------------------------------------
//  PKCS_CONTENT_INFO data structure
//
//  pvStructInfo points to following CRYPT_CONTENT_INFO.
//
//  For X509_ASN_ENCODING: encoded as a PKCS#7 ContentInfo structure.
//  The CRYPT_DER_BLOB points to the already encoded ANY content.
//--------------------------------------------------------------------------
typedef struct _CRYPT_CONTENT_INFO {
    LPSTR               pszObjId;
    CRYPT_DER_BLOB      Content;
} CRYPT_CONTENT_INFO, *PCRYPT_CONTENT_INFO;


//+-------------------------------------------------------------------------
//  X509_OCTET_STRING data structure
//
//  pvStructInfo points to a CRYPT_DATA_BLOB.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_BITS data structure
//
//  pvStructInfo points to a CRYPT_BIT_BLOB.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_BITS_WITHOUT_TRAILING_ZEROES data structure
//
//  pvStructInfo points to a CRYPT_BIT_BLOB.
//
//  The same as X509_BITS, except before encoding, the bit length is
//  decremented to exclude trailing zero bits.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_INTEGER data structure
//
//  pvStructInfo points to an int.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_MULTI_BYTE_INTEGER data structure
//
//  pvStructInfo points to a CRYPT_INTEGER_BLOB.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_ENUMERATED data structure
//
//  pvStructInfo points to an int containing the enumerated value
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_CHOICE_OF_TIME data structure
//
//  pvStructInfo points to a FILETIME.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_SEQUENCE_OF_ANY data structure
//
//  pvStructInfo points to following CRYPT_SEQUENCE_OF_ANY.
//
//  The CRYPT_DER_BLOBs point to the already encoded ANY content.
//--------------------------------------------------------------------------
typedef struct _CRYPT_SEQUENCE_OF_ANY {
    DWORD               cValue;
    PCRYPT_DER_BLOB     rgValue;
} CRYPT_SEQUENCE_OF_ANY, *PCRYPT_SEQUENCE_OF_ANY;


//+-------------------------------------------------------------------------
//  X509_AUTHORITY_KEY_ID2
//  szOID_AUTHORITY_KEY_IDENTIFIER2
//
//  pvStructInfo points to following CERT_AUTHORITY_KEY_ID2_INFO.
//
//  For CRYPT_E_INVALID_IA5_STRING, the error location is returned in
//  *pcbEncoded by CryptEncodeObject(X509_AUTHORITY_KEY_ID2)
//
//  See X509_ALTERNATE_NAME for error location defines.
//--------------------------------------------------------------------------
typedef struct _CERT_AUTHORITY_KEY_ID2_INFO {
    CRYPT_DATA_BLOB     KeyId;
    CERT_ALT_NAME_INFO  AuthorityCertIssuer;    // Optional, set cAltEntry
                                                // to 0 to omit.
    CRYPT_INTEGER_BLOB  AuthorityCertSerialNumber;
} CERT_AUTHORITY_KEY_ID2_INFO, *PCERT_AUTHORITY_KEY_ID2_INFO;

//+-------------------------------------------------------------------------
//  szOID_SUBJECT_KEY_IDENTIFIER
//
//  pvStructInfo points to a CRYPT_DATA_BLOB.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_AUTHORITY_INFO_ACCESS
//  szOID_AUTHORITY_INFO_ACCESS
//
//  X509_SUBJECT_INFO_ACCESS
//  szOID_SUBJECT_INFO_ACCESS
//
//  pvStructInfo points to following CERT_AUTHORITY_INFO_ACCESS.
//
//  For CRYPT_E_INVALID_IA5_STRING, the error location is returned in
//  *pcbEncoded by CryptEncodeObject(X509_AUTHORITY_INFO_ACCESS)
//
//  Error location consists of:
//    ENTRY_INDEX   - 8 bits << 16
//    VALUE_INDEX   - 16 bits (unicode character index)
//
//  See X509_ALTERNATE_NAME for ENTRY_INDEX and VALUE_INDEX error location
//  defines.
//
//  Note, the szOID_SUBJECT_INFO_ACCESS extension has the same ASN.1
//  encoding as the szOID_AUTHORITY_INFO_ACCESS extension.
//--------------------------------------------------------------------------

typedef struct _CERT_ACCESS_DESCRIPTION {
    LPSTR               pszAccessMethod;        // pszObjId
    CERT_ALT_NAME_ENTRY AccessLocation;
} CERT_ACCESS_DESCRIPTION, *PCERT_ACCESS_DESCRIPTION;


typedef struct _CERT_AUTHORITY_INFO_ACCESS {
    DWORD                       cAccDescr;
    PCERT_ACCESS_DESCRIPTION    rgAccDescr;
} CERT_AUTHORITY_INFO_ACCESS, *PCERT_AUTHORITY_INFO_ACCESS,
  CERT_SUBJECT_INFO_ACCESS, *PCERT_SUBJECT_INFO_ACCESS;


//+-------------------------------------------------------------------------
//  PKIX Access Description: Access Method Object Identifiers
//--------------------------------------------------------------------------
#define szOID_PKIX_ACC_DESCR            "1.3.6.1.5.5.7.48"

// For szOID_AUTHORITY_INFO_ACCESS
#define szOID_PKIX_OCSP                 "1.3.6.1.5.5.7.48.1"
#define szOID_PKIX_CA_ISSUERS           "1.3.6.1.5.5.7.48.2"

// For szOID_SUBJECT_INFO_ACCESS
#define szOID_PKIX_TIME_STAMPING        "1.3.6.1.5.5.7.48.3"
#define szOID_PKIX_CA_REPOSITORY        "1.3.6.1.5.5.7.48.5"


//+-------------------------------------------------------------------------
//  X509_CRL_REASON_CODE
//  szOID_CRL_REASON_CODE
//
//  pvStructInfo points to an int which can be set to one of the following
//  enumerated values:
//--------------------------------------------------------------------------
#define CRL_REASON_UNSPECIFIED              0
#define CRL_REASON_KEY_COMPROMISE           1
#define CRL_REASON_CA_COMPROMISE            2
#define CRL_REASON_AFFILIATION_CHANGED      3
#define CRL_REASON_SUPERSEDED               4
#define CRL_REASON_CESSATION_OF_OPERATION   5
#define CRL_REASON_CERTIFICATE_HOLD         6
#define CRL_REASON_REMOVE_FROM_CRL          8
#define CRL_REASON_PRIVILEGE_WITHDRAWN      9
#define CRL_REASON_AA_COMPROMISE            10


//+-------------------------------------------------------------------------
//  X509_CRL_DIST_POINTS
//  szOID_CRL_DIST_POINTS
//
//  pvStructInfo points to following CRL_DIST_POINTS_INFO.
//
//  For CRYPT_E_INVALID_IA5_STRING, the error location is returned in
//  *pcbEncoded by CryptEncodeObject(X509_CRL_DIST_POINTS)
//
//  Error location consists of:
//    CRL_ISSUER_BIT    - 1 bit  << 31 (0 for FullName, 1 for CRLIssuer)
//    POINT_INDEX       - 7 bits << 24
//    ENTRY_INDEX       - 8 bits << 16
//    VALUE_INDEX       - 16 bits (unicode character index)
//
//  See X509_ALTERNATE_NAME for ENTRY_INDEX and VALUE_INDEX error location
//  defines.
//--------------------------------------------------------------------------
typedef struct _CRL_DIST_POINT_NAME {
    DWORD   dwDistPointNameChoice;
    union {
        CERT_ALT_NAME_INFO      FullName;       // 1
        // Not implemented      IssuerRDN;      // 2
    } DUMMYUNIONNAME;
} CRL_DIST_POINT_NAME, *PCRL_DIST_POINT_NAME;

#define CRL_DIST_POINT_NO_NAME          0
#define CRL_DIST_POINT_FULL_NAME        1
#define CRL_DIST_POINT_ISSUER_RDN_NAME  2

typedef struct _CRL_DIST_POINT {
    CRL_DIST_POINT_NAME     DistPointName;      // OPTIONAL
    CRYPT_BIT_BLOB          ReasonFlags;        // OPTIONAL
    CERT_ALT_NAME_INFO      CRLIssuer;          // OPTIONAL
} CRL_DIST_POINT, *PCRL_DIST_POINT;

// Byte[0]
#define CRL_REASON_UNUSED_FLAG                  0x80
#define CRL_REASON_KEY_COMPROMISE_FLAG          0x40
#define CRL_REASON_CA_COMPROMISE_FLAG           0x20
#define CRL_REASON_AFFILIATION_CHANGED_FLAG     0x10
#define CRL_REASON_SUPERSEDED_FLAG              0x08
#define CRL_REASON_CESSATION_OF_OPERATION_FLAG  0x04
#define CRL_REASON_CERTIFICATE_HOLD_FLAG        0x02
#define CRL_REASON_PRIVILEGE_WITHDRAWN_FLAG     0x01
// Byte[1]
#define CRL_REASON_AA_COMPROMISE_FLAG           0x80

typedef struct _CRL_DIST_POINTS_INFO {
    DWORD                   cDistPoint;
    PCRL_DIST_POINT         rgDistPoint;
} CRL_DIST_POINTS_INFO, *PCRL_DIST_POINTS_INFO;

#define CRL_DIST_POINT_ERR_INDEX_MASK          0x7F
#define CRL_DIST_POINT_ERR_INDEX_SHIFT         24
#define GET_CRL_DIST_POINT_ERR_INDEX(X)   \
    ((X >> CRL_DIST_POINT_ERR_INDEX_SHIFT) & CRL_DIST_POINT_ERR_INDEX_MASK)

#define CRL_DIST_POINT_ERR_CRL_ISSUER_BIT      0x80000000L
#define IS_CRL_DIST_POINT_ERR_CRL_ISSUER(X)   \
    (0 != (X & CRL_DIST_POINT_ERR_CRL_ISSUER_BIT))

//+-------------------------------------------------------------------------
//  X509_CROSS_CERT_DIST_POINTS
//  szOID_CROSS_CERT_DIST_POINTS
//
//  pvStructInfo points to following CROSS_CERT_DIST_POINTS_INFO.
//
//  For CRYPT_E_INVALID_IA5_STRING, the error location is returned in
//  *pcbEncoded by CryptEncodeObject(X509_CRL_DIST_POINTS)
//
//  Error location consists of:
//    POINT_INDEX       - 8 bits << 24
//    ENTRY_INDEX       - 8 bits << 16
//    VALUE_INDEX       - 16 bits (unicode character index)
//
//  See X509_ALTERNATE_NAME for ENTRY_INDEX and VALUE_INDEX error location
//  defines.
//--------------------------------------------------------------------------
typedef struct _CROSS_CERT_DIST_POINTS_INFO {
    // Seconds between syncs. 0 implies use client default.
    DWORD                   dwSyncDeltaTime;

    DWORD                   cDistPoint;
    PCERT_ALT_NAME_INFO     rgDistPoint;
} CROSS_CERT_DIST_POINTS_INFO, *PCROSS_CERT_DIST_POINTS_INFO;

#define CROSS_CERT_DIST_POINT_ERR_INDEX_MASK   0xFF
#define CROSS_CERT_DIST_POINT_ERR_INDEX_SHIFT  24
#define GET_CROSS_CERT_DIST_POINT_ERR_INDEX(X)   \
    ((X >> CROSS_CERT_DIST_POINT_ERR_INDEX_SHIFT) & \
                CROSS_CERT_DIST_POINT_ERR_INDEX_MASK)



//+-------------------------------------------------------------------------
//  X509_ENHANCED_KEY_USAGE
//  szOID_ENHANCED_KEY_USAGE
//
//  pvStructInfo points to a CERT_ENHKEY_USAGE, CTL_USAGE.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_CERT_PAIR
//
//  pvStructInfo points to the following CERT_PAIR.
//--------------------------------------------------------------------------
typedef struct _CERT_PAIR {
   CERT_BLOB    Forward;        // OPTIONAL, if Forward.cbData == 0, omitted
   CERT_BLOB    Reverse;        // OPTIONAL, if Reverse.cbData == 0, omitted
} CERT_PAIR, *PCERT_PAIR;

//+-------------------------------------------------------------------------
//  szOID_CRL_NUMBER
//
//  pvStructInfo points to an int.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  szOID_DELTA_CRL_INDICATOR
//
//  pvStructInfo points to an int.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  szOID_ISSUING_DIST_POINT
//  X509_ISSUING_DIST_POINT
//
//  pvStructInfo points to the following CRL_ISSUING_DIST_POINT.
//
//  For CRYPT_E_INVALID_IA5_STRING, the error location is returned in
//  *pcbEncoded by CryptEncodeObject(X509_ISSUING_DIST_POINT)
//
//  Error location consists of:
//    ENTRY_INDEX       - 8 bits << 16
//    VALUE_INDEX       - 16 bits (unicode character index)
//
//  See X509_ALTERNATE_NAME for ENTRY_INDEX and VALUE_INDEX error location
//  defines.
//--------------------------------------------------------------------------
typedef struct _CRL_ISSUING_DIST_POINT {
    CRL_DIST_POINT_NAME     DistPointName;              // OPTIONAL
    BOOL                    fOnlyContainsUserCerts;
    BOOL                    fOnlyContainsCACerts;
    CRYPT_BIT_BLOB          OnlySomeReasonFlags;        // OPTIONAL
    BOOL                    fIndirectCRL;
} CRL_ISSUING_DIST_POINT, *PCRL_ISSUING_DIST_POINT;

//+-------------------------------------------------------------------------
//  szOID_FRESHEST_CRL
//
//  pvStructInfo points to CRL_DIST_POINTS_INFO.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  szOID_NAME_CONSTRAINTS
//  X509_NAME_CONSTRAINTS
//
//  pvStructInfo points to the following CERT_NAME_CONSTRAINTS_INFO
//
//  For CRYPT_E_INVALID_IA5_STRING, the error location is returned in
//  *pcbEncoded by CryptEncodeObject(X509_NAME_CONSTRAINTS)
//
//  Error location consists of:
//    EXCLUDED_SUBTREE_BIT  - 1 bit  << 31 (0 for permitted, 1 for excluded)
//    ENTRY_INDEX           - 8 bits << 16
//    VALUE_INDEX           - 16 bits (unicode character index)
//
//  See X509_ALTERNATE_NAME for ENTRY_INDEX and VALUE_INDEX error location
//  defines.
//--------------------------------------------------------------------------
typedef struct _CERT_GENERAL_SUBTREE {
    CERT_ALT_NAME_ENTRY     Base;
    DWORD                   dwMinimum;
    BOOL                    fMaximum;
    DWORD                   dwMaximum;
} CERT_GENERAL_SUBTREE, *PCERT_GENERAL_SUBTREE;

typedef struct _CERT_NAME_CONSTRAINTS_INFO {
    DWORD                   cPermittedSubtree;
    PCERT_GENERAL_SUBTREE   rgPermittedSubtree;
    DWORD                   cExcludedSubtree;
    PCERT_GENERAL_SUBTREE   rgExcludedSubtree;
} CERT_NAME_CONSTRAINTS_INFO, *PCERT_NAME_CONSTRAINTS_INFO;

#define CERT_EXCLUDED_SUBTREE_BIT       0x80000000L
#define IS_CERT_EXCLUDED_SUBTREE(X)     \
    (0 != (X & CERT_EXCLUDED_SUBTREE_BIT))

//+-------------------------------------------------------------------------
//  szOID_NEXT_UPDATE_LOCATION
//
//  pvStructInfo points to a CERT_ALT_NAME_INFO.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  szOID_REMOVE_CERTIFICATE
//
//  pvStructInfo points to an int which can be set to one of the following
//   0 - Add certificate
//   1 - Remove certificate
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  PKCS_CTL
//  szOID_CTL
//
//  pvStructInfo points to a CTL_INFO.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  PKCS_SORTED_CTL
//
//  pvStructInfo points to a CTL_INFO.
//
//  Same as for PKCS_CTL, except, the CTL entries are sorted. The following
//  extension containing the sort information is inserted as the first
//  extension in the encoded CTL.
//
//  Only supported for Encoding. CRYPT_ENCODE_ALLOC_FLAG flag must be
//  set.
//--------------------------------------------------------------------------


//+-------------------------------------------------------------------------
// Sorted CTL TrustedSubjects extension
//
//  Array of little endian DWORDs:
//   [0] - Flags
//   [1] - Count of HashBucket entry offsets
//   [2] - Maximum HashBucket entry collision count
//   [3 ..] (Count + 1) HashBucket entry offsets
//
//  When this extension is present in the CTL,
//  the ASN.1 encoded sequence of TrustedSubjects are HashBucket ordered.
//
//  The entry offsets point to the start of the first encoded TrustedSubject
//  sequence for the HashBucket. The encoded TrustedSubjects for a HashBucket
//  continue until the encoded offset of the next HashBucket. A HashBucket has
//  no entries if HashBucket[N] == HashBucket[N + 1].
//
//  The HashBucket offsets are from the start of the ASN.1 encoded CTL_INFO.
//--------------------------------------------------------------------------
#define SORTED_CTL_EXT_FLAGS_OFFSET         (0*4)
#define SORTED_CTL_EXT_COUNT_OFFSET         (1*4)
#define SORTED_CTL_EXT_MAX_COLLISION_OFFSET (2*4)
#define SORTED_CTL_EXT_HASH_BUCKET_OFFSET   (3*4)

// If the SubjectIdentifiers are a MD5 or SHA1 hash, the following flag is
// set. When set, the first 4 bytes of the SubjectIdentifier are used as
// the dwhash. Otherwise, the SubjectIdentifier bytes are hashed into dwHash.
// In either case the HashBucket index = dwHash % cHashBucket.
#define SORTED_CTL_EXT_HASHED_SUBJECT_IDENTIFIER_FLAG       0x1

//+-------------------------------------------------------------------------
//  X509_MULTI_BYTE_UINT
//
//  pvStructInfo points to a CRYPT_UINT_BLOB. Before encoding, inserts a
//  leading 0x00. After decoding, removes a leading 0x00.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_DSS_PUBLICKEY
//
//  pvStructInfo points to a CRYPT_UINT_BLOB.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_DSS_PARAMETERS
//
//  pvStructInfo points to following CERT_DSS_PARAMETERS data structure.
//--------------------------------------------------------------------------
typedef struct _CERT_DSS_PARAMETERS {
    CRYPT_UINT_BLOB     p;
    CRYPT_UINT_BLOB     q;
    CRYPT_UINT_BLOB     g;
} CERT_DSS_PARAMETERS, *PCERT_DSS_PARAMETERS;

//+-------------------------------------------------------------------------
//  X509_DSS_SIGNATURE
//
//  pvStructInfo is a BYTE rgbSignature[CERT_DSS_SIGNATURE_LEN]. The
//  bytes are ordered as output by the DSS CSP's CryptSignHash().
//--------------------------------------------------------------------------
#define CERT_DSS_R_LEN          20
#define CERT_DSS_S_LEN          20
#define CERT_DSS_SIGNATURE_LEN  (CERT_DSS_R_LEN + CERT_DSS_S_LEN)

// Sequence of 2 unsigned integers (the extra +1 is for a potential leading
// 0x00 to make the integer unsigned)
#define CERT_MAX_ASN_ENCODED_DSS_SIGNATURE_LEN  (2 + 2*(2 + 20 +1))

//+-------------------------------------------------------------------------
//  X509_DH_PUBLICKEY
//
//  pvStructInfo points to a CRYPT_UINT_BLOB.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_DH_PARAMETERS
//
//  pvStructInfo points to following CERT_DH_PARAMETERS data structure.
//--------------------------------------------------------------------------
typedef struct _CERT_DH_PARAMETERS {
    CRYPT_UINT_BLOB     p;
    CRYPT_UINT_BLOB     g;
} CERT_DH_PARAMETERS, *PCERT_DH_PARAMETERS;

//+-------------------------------------------------------------------------
//  X509_ECC_SIGNATURE
//
//  pvStructInfo points to following CERT_ECC_SIGNATURE data structure.
//
//  Note, identical to the above except for the names of the fields. Same
//  underlying encode/decode functions are used.
//--------------------------------------------------------------------------
typedef struct _CERT_ECC_SIGNATURE {
    CRYPT_UINT_BLOB     r;
    CRYPT_UINT_BLOB     s;
} CERT_ECC_SIGNATURE, *PCERT_ECC_SIGNATURE;

//+-------------------------------------------------------------------------
//  X942_DH_PARAMETERS
//
//  pvStructInfo points to following CERT_X942_DH_PARAMETERS data structure.
//
//  If q.cbData == 0, then, the following fields are zero'ed.
//--------------------------------------------------------------------------
typedef struct _CERT_X942_DH_VALIDATION_PARAMS {
    CRYPT_BIT_BLOB      seed;
    DWORD               pgenCounter;
} CERT_X942_DH_VALIDATION_PARAMS, *PCERT_X942_DH_VALIDATION_PARAMS;

typedef struct _CERT_X942_DH_PARAMETERS {
    CRYPT_UINT_BLOB     p;          // odd prime, p = jq + 1
    CRYPT_UINT_BLOB     g;          // generator, g
    CRYPT_UINT_BLOB     q;          // factor of p - 1, OPTIONAL
    CRYPT_UINT_BLOB     j;          // subgroup factor, OPTIONAL
    PCERT_X942_DH_VALIDATION_PARAMS pValidationParams;  // OPTIONAL
} CERT_X942_DH_PARAMETERS, *PCERT_X942_DH_PARAMETERS;

//+-------------------------------------------------------------------------
//  X942_OTHER_INFO
//
//  pvStructInfo points to following CRYPT_X942_OTHER_INFO data structure.
//
//  rgbCounter and rgbKeyLength are in Little Endian order.
//--------------------------------------------------------------------------
#define CRYPT_X942_COUNTER_BYTE_LENGTH      4
#define CRYPT_X942_KEY_LENGTH_BYTE_LENGTH   4
#define CRYPT_X942_PUB_INFO_BYTE_LENGTH     (512/8)
typedef struct _CRYPT_X942_OTHER_INFO {
    LPSTR               pszContentEncryptionObjId;
    BYTE                rgbCounter[CRYPT_X942_COUNTER_BYTE_LENGTH];
    BYTE                rgbKeyLength[CRYPT_X942_KEY_LENGTH_BYTE_LENGTH];
    CRYPT_DATA_BLOB     PubInfo;    // OPTIONAL
} CRYPT_X942_OTHER_INFO, *PCRYPT_X942_OTHER_INFO;


//+-------------------------------------------------------------------------
//  ECC_CMS_SHARED_INFO
//
//  pvStructInfo points to following ECC_CMS_SHARED_INFO data structure.
//
//  rgbSuppPubInfo is in Little Endian order.
//--------------------------------------------------------------------------
#define CRYPT_ECC_CMS_SHARED_INFO_SUPPPUBINFO_BYTE_LENGTH   4
typedef struct _CRYPT_ECC_CMS_SHARED_INFO {
    CRYPT_ALGORITHM_IDENTIFIER  Algorithm;
    CRYPT_DATA_BLOB             EntityUInfo;    // OPTIONAL
    BYTE                        rgbSuppPubInfo[CRYPT_ECC_CMS_SHARED_INFO_SUPPPUBINFO_BYTE_LENGTH];
} CRYPT_ECC_CMS_SHARED_INFO, *PCRYPT_ECC_CMS_SHARED_INFO;


//+-------------------------------------------------------------------------
//  PKCS_RC2_CBC_PARAMETERS
//  szOID_RSA_RC2CBC
//
//  pvStructInfo points to following CRYPT_RC2_CBC_PARAMETERS data structure.
//--------------------------------------------------------------------------
typedef struct _CRYPT_RC2_CBC_PARAMETERS {
    DWORD               dwVersion;
    BOOL                fIV;            // set if has following IV
    BYTE                rgbIV[8];
} CRYPT_RC2_CBC_PARAMETERS, *PCRYPT_RC2_CBC_PARAMETERS;

#define CRYPT_RC2_40BIT_VERSION     160
#define CRYPT_RC2_56BIT_VERSION     52
#define CRYPT_RC2_64BIT_VERSION     120
#define CRYPT_RC2_128BIT_VERSION    58


//+-------------------------------------------------------------------------
//  PKCS_SMIME_CAPABILITIES
//  szOID_RSA_SMIMECapabilities
//
//  pvStructInfo points to following CRYPT_SMIME_CAPABILITIES data structure.
//
//  Note, for CryptEncodeObject(X509_ASN_ENCODING), Parameters.cbData == 0
//  causes the encoded parameters to be omitted and not encoded as a NULL
//  (05 00) as is done when encoding a CRYPT_ALGORITHM_IDENTIFIER. This
//  is per the SMIME specification for encoding capabilities.
//--------------------------------------------------------------------------
// certenrolls_begin -- CRYPT_SMIME_CAPABILITY
typedef struct _CRYPT_SMIME_CAPABILITY {
    LPSTR               pszObjId;
    CRYPT_OBJID_BLOB    Parameters;
} CRYPT_SMIME_CAPABILITY, *PCRYPT_SMIME_CAPABILITY;

typedef struct _CRYPT_SMIME_CAPABILITIES {
    DWORD                   cCapability;
    PCRYPT_SMIME_CAPABILITY rgCapability;
} CRYPT_SMIME_CAPABILITIES, *PCRYPT_SMIME_CAPABILITIES;
// certenrolls_end



//+-------------------------------------------------------------------------
//  Qualified Certificate Statements Extension Data Structures
//
//  X509_QC_STATEMENTS_EXT
//  szOID_QC_STATEMENTS_EXT
//
//  pvStructInfo points to following CERT_QC_STATEMENTS_EXT_INFO
//  data structure.
//
//  Note, identical to the above except for the names of the fields. Same
//  underlying encode/decode functions are used.
//--------------------------------------------------------------------------
typedef struct _CERT_QC_STATEMENT {
    LPSTR               pszStatementId;     // pszObjId
    CRYPT_OBJID_BLOB    StatementInfo;      // OPTIONAL
} CERT_QC_STATEMENT, *PCERT_QC_STATEMENT;

typedef struct _CERT_QC_STATEMENTS_EXT_INFO {
    DWORD                   cStatement;
    PCERT_QC_STATEMENT      rgStatement;
} CERT_QC_STATEMENTS_EXT_INFO, *PCERT_QC_STATEMENTS_EXT_INFO;


// QC Statment Ids

// European Union
#define szOID_QC_EU_COMPLIANCE          "0.4.0.1862.1.1"
// Secure Signature Creation Device
#define szOID_QC_SSCD                   "0.4.0.1862.1.4"

//+-------------------------------------------------------------------------
//  X509_OBJECT_IDENTIFIER
//  szOID_ECC_PUBLIC_KEY
//
//  pvStructInfo points to a LPSTR of the dot representation.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  X509_ALGORITHM_IDENTIFIER
//  szOID_ECDSA_SPECIFIED
//
//  pvStructInfo points to a CRYPT_ALGORITHM_IDENTIFIER.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  PKCS_RSA_SSA_PSS_PARAMETERS
//  szOID_RSA_SSA_PSS
//
//  pvStructInfo points to the following CRYPT_RSA_SSA_PSS_PARAMETERS
//  data structure.
//
//  For encoding uses the following defaults if the corresponding field
//  is set to NULL or 0:
//      HashAlgorithm.pszObjId : szOID_OIWSEC_sha1
//      MaskGenAlgorithm.pszObjId : szOID_RSA_MGF1
//      MaskGenAlgorithm.HashAlgorithm.pszObjId : HashAlgorithm.pszObjId
//      dwSaltLength: cbHash
//      dwTrailerField : PKCS_RSA_SSA_PSS_TRAILER_FIELD_BC
//
//  Normally for encoding, only the HashAlgorithm.pszObjId field will
//  need to be set.
//
//  For decoding, all of fields are explicitly set.
//--------------------------------------------------------------------------
typedef struct _CRYPT_MASK_GEN_ALGORITHM {
    LPSTR                       pszObjId;
    CRYPT_ALGORITHM_IDENTIFIER  HashAlgorithm;
} CRYPT_MASK_GEN_ALGORITHM, *PCRYPT_MASK_GEN_ALGORITHM;

typedef struct _CRYPT_RSA_SSA_PSS_PARAMETERS {
    CRYPT_ALGORITHM_IDENTIFIER  HashAlgorithm;
    CRYPT_MASK_GEN_ALGORITHM    MaskGenAlgorithm;
    DWORD                       dwSaltLength;
    DWORD                       dwTrailerField;
} CRYPT_RSA_SSA_PSS_PARAMETERS, *PCRYPT_RSA_SSA_PSS_PARAMETERS;

#define PKCS_RSA_SSA_PSS_TRAILER_FIELD_BC       1

//+-------------------------------------------------------------------------
//  PKCS_RSAES_OAEP_PARAMETERS
//  szOID_RSAES_OAEP
//
//  pvStructInfo points to the following CRYPT_RSAES_OAEP_PARAMETERS
//  data structure.
//
//  For encoding uses the following defaults if the corresponding field
//  is set to NULL or 0:
//      HashAlgorithm.pszObjId : szOID_OIWSEC_sha1
//      MaskGenAlgorithm.pszObjId : szOID_RSA_MGF1
//      MaskGenAlgorithm.HashAlgorithm.pszObjId : HashAlgorithm.pszObjId
//      PSourceAlgorithm.pszObjId : szOID_RSA_PSPECIFIED
//      PSourceAlgorithm.EncodingParameters.cbData : 0
//      PSourceAlgorithm.EncodingParameters.pbData : NULL
//
//  Normally for encoding, only the HashAlgorithm.pszObjId field will
//  need to be set.
//
//  For decoding, all of fields are explicitly set.
//--------------------------------------------------------------------------
typedef struct _CRYPT_PSOURCE_ALGORITHM {
    LPSTR                       pszObjId;
    CRYPT_DATA_BLOB             EncodingParameters;
} CRYPT_PSOURCE_ALGORITHM, *PCRYPT_PSOURCE_ALGORITHM;

typedef struct _CRYPT_RSAES_OAEP_PARAMETERS {
    CRYPT_ALGORITHM_IDENTIFIER  HashAlgorithm;
    CRYPT_MASK_GEN_ALGORITHM    MaskGenAlgorithm;
    CRYPT_PSOURCE_ALGORITHM     PSourceAlgorithm;
} CRYPT_RSAES_OAEP_PARAMETERS, *PCRYPT_RSAES_OAEP_PARAMETERS;


//+-------------------------------------------------------------------------
//  PKCS7_SIGNER_INFO (WINCRYPT_PKCS7_SIGNER_INFO)
//
//  pvStructInfo points to CMSG_SIGNER_INFO.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMS_SIGNER_INFO
//
//  pvStructInfo points to CMSG_CMS_SIGNER_INFO.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  Verisign Certificate Extension Object Identifiers
//--------------------------------------------------------------------------

// Octet String containing Boolean
#define szOID_VERISIGN_PRIVATE_6_9       "2.16.840.1.113733.1.6.9"

// Octet String containing IA5 string: lower case 32 char hex string
#define szOID_VERISIGN_ONSITE_JURISDICTION_HASH "2.16.840.1.113733.1.6.11"

// Octet String containing Bit string
#define szOID_VERISIGN_BITSTRING_6_13    "2.16.840.1.113733.1.6.13"

// EKU
#define szOID_VERISIGN_ISS_STRONG_CRYPTO "2.16.840.1.113733.1.8.1"

//+-------------------------------------------------------------------------
//  Verisign SCEP Signed Pkcs7 authenticated attribute Object Identifiers
//--------------------------------------------------------------------------

// Signed decimal strings encoded as Printable String
#define szOIDVerisign_MessageType	"2.16.840.1.113733.1.9.2"
#define szOIDVerisign_PkiStatus		"2.16.840.1.113733.1.9.3"
#define szOIDVerisign_FailInfo		"2.16.840.1.113733.1.9.4"

// Binary data encoded as Octet String
#define szOIDVerisign_SenderNonce	"2.16.840.1.113733.1.9.5"
#define szOIDVerisign_RecipientNonce	"2.16.840.1.113733.1.9.6"

// Binary data converted to hexadecimal string and encoded as Printable String
#define szOIDVerisign_TransactionID	"2.16.840.1.113733.1.9.7"

//+-------------------------------------------------------------------------
//  Netscape Certificate Extension Object Identifiers
//--------------------------------------------------------------------------
#define szOID_NETSCAPE                  "2.16.840.1.113730"
#define szOID_NETSCAPE_CERT_EXTENSION   "2.16.840.1.113730.1"
#define szOID_NETSCAPE_CERT_TYPE        "2.16.840.1.113730.1.1"
#define szOID_NETSCAPE_BASE_URL         "2.16.840.1.113730.1.2"
#define szOID_NETSCAPE_REVOCATION_URL   "2.16.840.1.113730.1.3"
#define szOID_NETSCAPE_CA_REVOCATION_URL "2.16.840.1.113730.1.4"
#define szOID_NETSCAPE_CERT_RENEWAL_URL "2.16.840.1.113730.1.7"
#define szOID_NETSCAPE_CA_POLICY_URL    "2.16.840.1.113730.1.8"
#define szOID_NETSCAPE_SSL_SERVER_NAME  "2.16.840.1.113730.1.12"
#define szOID_NETSCAPE_COMMENT          "2.16.840.1.113730.1.13"

//+-------------------------------------------------------------------------
//  Netscape Certificate Data Type Object Identifiers
//--------------------------------------------------------------------------
#define szOID_NETSCAPE_DATA_TYPE        "2.16.840.1.113730.2"
#define szOID_NETSCAPE_CERT_SEQUENCE    "2.16.840.1.113730.2.5"


//+-------------------------------------------------------------------------
//  szOID_NETSCAPE_CERT_TYPE extension
//
//  Its value is a bit string. CryptDecodeObject/CryptEncodeObject using
//  X509_BITS or X509_BITS_WITHOUT_TRAILING_ZEROES.
//
//  The following bits are defined:
//--------------------------------------------------------------------------
#define NETSCAPE_SSL_CLIENT_AUTH_CERT_TYPE  0x80
#define NETSCAPE_SSL_SERVER_AUTH_CERT_TYPE  0x40
#define NETSCAPE_SMIME_CERT_TYPE            0x20
#define NETSCAPE_SIGN_CERT_TYPE             0x10
#define NETSCAPE_SSL_CA_CERT_TYPE           0x04
#define NETSCAPE_SMIME_CA_CERT_TYPE         0x02
#define NETSCAPE_SIGN_CA_CERT_TYPE          0x01

//+-------------------------------------------------------------------------
//  szOID_NETSCAPE_BASE_URL extension
//
//  Its value is an IA5_STRING. CryptDecodeObject/CryptEncodeObject using
//  X509_ANY_STRING or X509_UNICODE_ANY_STRING, where,
//  dwValueType = CERT_RDN_IA5_STRING.
//
//  When present this string is added to the beginning of all relative URLs
//  in the certificate.  This extension can be considered an optimization
//  to reduce the size of the URL extensions.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  szOID_NETSCAPE_REVOCATION_URL extension
//
//  Its value is an IA5_STRING. CryptDecodeObject/CryptEncodeObject using
//  X509_ANY_STRING or X509_UNICODE_ANY_STRING, where,
//  dwValueType = CERT_RDN_IA5_STRING.
//
//  It is a relative or absolute URL that can be used to check the
//  revocation status of a certificate. The revocation check will be
//  performed as an HTTP GET method using a url that is the concatenation of
//  revocation-url and certificate-serial-number.
//  Where the certificate-serial-number is encoded as a string of
//  ascii hexadecimal digits. For example, if the netscape-base-url is
//  https://www.certs-r-us.com/, the netscape-revocation-url is
//  cgi-bin/check-rev.cgi?, and the certificate serial number is 173420,
//  the resulting URL would be:
//  https://www.certs-r-us.com/cgi-bin/check-rev.cgi?02a56c
//
//  The server should return a document with a Content-Type of
//  application/x-netscape-revocation.  The document should contain
//  a single ascii digit, '1' if the certificate is not curently valid,
//  and '0' if it is curently valid.
//
//  Note: for all of the URLs that include the certificate serial number,
//  the serial number will be encoded as a string which consists of an even
//  number of hexadecimal digits.  If the number of significant digits is odd,
//  the string will have a single leading zero to ensure an even number of
//  digits is generated.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  szOID_NETSCAPE_CA_REVOCATION_URL extension
//
//  Its value is an IA5_STRING. CryptDecodeObject/CryptEncodeObject using
//  X509_ANY_STRING or X509_UNICODE_ANY_STRING, where,
//  dwValueType = CERT_RDN_IA5_STRING.
//
//  It is a relative or absolute URL that can be used to check the
//  revocation status of any certificates that are signed by the CA that
//  this certificate belongs to. This extension is only valid in CA
//  certificates.  The use of this extension is the same as the above
//  szOID_NETSCAPE_REVOCATION_URL extension.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  szOID_NETSCAPE_CERT_RENEWAL_URL extension
//
//  Its value is an IA5_STRING. CryptDecodeObject/CryptEncodeObject using
//  X509_ANY_STRING or X509_UNICODE_ANY_STRING, where,
//  dwValueType = CERT_RDN_IA5_STRING.
//
//  It is a relative or absolute URL that points to a certificate renewal
//  form. The renewal form will be accessed with an HTTP GET method using a
//  url that is the concatenation of renewal-url and
//  certificate-serial-number. Where the certificate-serial-number is
//  encoded as a string of ascii hexadecimal digits. For example, if the
//  netscape-base-url is https://www.certs-r-us.com/, the
//  netscape-cert-renewal-url is cgi-bin/check-renew.cgi?, and the
//  certificate serial number is 173420, the resulting URL would be:
//  https://www.certs-r-us.com/cgi-bin/check-renew.cgi?02a56c
//  The document returned should be an HTML form that will allow the user
//  to request a renewal of their certificate.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  szOID_NETSCAPE_CA_POLICY_URL extension
//
//  Its value is an IA5_STRING. CryptDecodeObject/CryptEncodeObject using
//  X509_ANY_STRING or X509_UNICODE_ANY_STRING, where,
//  dwValueType = CERT_RDN_IA5_STRING.
//
//  It is a relative or absolute URL that points to a web page that
//  describes the policies under which the certificate was issued.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  szOID_NETSCAPE_SSL_SERVER_NAME extension
//
//  Its value is an IA5_STRING. CryptDecodeObject/CryptEncodeObject using
//  X509_ANY_STRING or X509_UNICODE_ANY_STRING, where,
//  dwValueType = CERT_RDN_IA5_STRING.
//
//  It is a "shell expression" that can be used to match the hostname of the
//  SSL server that is using this certificate.  It is recommended that if
//  the server's hostname does not match this pattern the user be notified
//  and given the option to terminate the SSL connection.  If this extension
//  is not present then the CommonName in the certificate subject's
//  distinguished name is used for the same purpose.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  szOID_NETSCAPE_COMMENT extension
//
//  Its value is an IA5_STRING. CryptDecodeObject/CryptEncodeObject using
//  X509_ANY_STRING or X509_UNICODE_ANY_STRING, where,
//  dwValueType = CERT_RDN_IA5_STRING.
//
//  It is a comment that may be displayed to the user when the certificate
//  is viewed.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  szOID_NETSCAPE_CERT_SEQUENCE
//
//  Its value is a PKCS#7 ContentInfo structure wrapping a sequence of
//  certificates. The value of the contentType field is
//  szOID_NETSCAPE_CERT_SEQUENCE, while the content field is the following
//  structure:
//      CertificateSequence ::= SEQUENCE OF Certificate.
//
//  CryptDecodeObject/CryptEncodeObject using
//  PKCS_CONTENT_INFO_SEQUENCE_OF_ANY, where,
//  pszObjId = szOID_NETSCAPE_CERT_SEQUENCE and the CRYPT_DER_BLOBs point
//  to encoded X509 certificates.
//--------------------------------------------------------------------------

//+=========================================================================
//  Certificate Management Messages over CMS (CMC) Data Structures
//==========================================================================

// Content Type (request)
#define szOID_CT_PKI_DATA               "1.3.6.1.5.5.7.12.2"

// Content Type (response)
#define szOID_CT_PKI_RESPONSE           "1.3.6.1.5.5.7.12.3"

// Signature value that only contains the hash octets. The parameters for
// this algorithm must be present and must be encoded as NULL.
#define szOID_PKIX_NO_SIGNATURE         "1.3.6.1.5.5.7.6.2"

#define szOID_CMC                       "1.3.6.1.5.5.7.7"
#define szOID_CMC_STATUS_INFO           "1.3.6.1.5.5.7.7.1"
#define szOID_CMC_IDENTIFICATION        "1.3.6.1.5.5.7.7.2"
#define szOID_CMC_IDENTITY_PROOF        "1.3.6.1.5.5.7.7.3"
#define szOID_CMC_DATA_RETURN           "1.3.6.1.5.5.7.7.4"

// Transaction Id (integer)
#define szOID_CMC_TRANSACTION_ID        "1.3.6.1.5.5.7.7.5"

// Sender Nonce (octet string)
#define szOID_CMC_SENDER_NONCE          "1.3.6.1.5.5.7.7.6"

// Recipient Nonce (octet string)
#define szOID_CMC_RECIPIENT_NONCE       "1.3.6.1.5.5.7.7.7"

#define szOID_CMC_ADD_EXTENSIONS        "1.3.6.1.5.5.7.7.8"
#define szOID_CMC_ENCRYPTED_POP         "1.3.6.1.5.5.7.7.9"
#define szOID_CMC_DECRYPTED_POP         "1.3.6.1.5.5.7.7.10"
#define szOID_CMC_LRA_POP_WITNESS       "1.3.6.1.5.5.7.7.11"

// Issuer Name + Serial
#define szOID_CMC_GET_CERT              "1.3.6.1.5.5.7.7.15"

// Issuer Name [+ CRL Name] + Time [+ Reasons]
#define szOID_CMC_GET_CRL               "1.3.6.1.5.5.7.7.16"

// Issuer Name + Serial [+ Reason] [+ Effective Time] [+ Secret] [+ Comment]
#define szOID_CMC_REVOKE_REQUEST        "1.3.6.1.5.5.7.7.17"

// (octet string) URL-style parameter list (IA5?)
#define szOID_CMC_REG_INFO              "1.3.6.1.5.5.7.7.18"

#define szOID_CMC_RESPONSE_INFO         "1.3.6.1.5.5.7.7.19"

// (octet string)
#define szOID_CMC_QUERY_PENDING         "1.3.6.1.5.5.7.7.21"
#define szOID_CMC_ID_POP_LINK_RANDOM    "1.3.6.1.5.5.7.7.22"
#define szOID_CMC_ID_POP_LINK_WITNESS   "1.3.6.1.5.5.7.7.23"

// optional Name + Integer
#define szOID_CMC_ID_CONFIRM_CERT_ACCEPTANCE "1.3.6.1.5.5.7.7.24"

#define szOID_CMC_ADD_ATTRIBUTES        "1.3.6.1.4.1.311.10.10.1"

//+-------------------------------------------------------------------------
//  CMC_DATA
//  CMC_RESPONSE
//
//  Certificate Management Messages over CMS (CMC) PKIData and Response
//  messages.
//
//  For CMC_DATA, pvStructInfo points to a CMC_DATA_INFO.
//  CMC_DATA_INFO contains optional arrays of tagged attributes, requests,
//  content info and/or arbitrary other messages.
//
//  For CMC_RESPONSE, pvStructInfo points to a CMC_RESPONSE_INFO.
//  CMC_RESPONSE_INFO is the same as CMC_DATA_INFO without the tagged
//  requests.
//--------------------------------------------------------------------------
typedef struct _CMC_TAGGED_ATTRIBUTE {
    DWORD               dwBodyPartID;
    CRYPT_ATTRIBUTE     Attribute;
} CMC_TAGGED_ATTRIBUTE, *PCMC_TAGGED_ATTRIBUTE;

typedef struct _CMC_TAGGED_CERT_REQUEST {
    DWORD               dwBodyPartID;
    CRYPT_DER_BLOB      SignedCertRequest;
} CMC_TAGGED_CERT_REQUEST, *PCMC_TAGGED_CERT_REQUEST;

typedef struct _CMC_TAGGED_REQUEST {
    DWORD               dwTaggedRequestChoice;
    union {
        // CMC_TAGGED_CERT_REQUEST_CHOICE
        PCMC_TAGGED_CERT_REQUEST   pTaggedCertRequest;
    } DUMMYUNIONNAME;
} CMC_TAGGED_REQUEST, *PCMC_TAGGED_REQUEST;

#define CMC_TAGGED_CERT_REQUEST_CHOICE      1

typedef struct _CMC_TAGGED_CONTENT_INFO {
    DWORD               dwBodyPartID;
    CRYPT_DER_BLOB      EncodedContentInfo;
} CMC_TAGGED_CONTENT_INFO, *PCMC_TAGGED_CONTENT_INFO;

typedef struct _CMC_TAGGED_OTHER_MSG {
    DWORD               dwBodyPartID;
    LPSTR               pszObjId;
    CRYPT_OBJID_BLOB    Value;
} CMC_TAGGED_OTHER_MSG, *PCMC_TAGGED_OTHER_MSG;


// All the tagged arrays are optional
typedef struct _CMC_DATA_INFO {
    DWORD                       cTaggedAttribute;
    PCMC_TAGGED_ATTRIBUTE       rgTaggedAttribute;
    DWORD                       cTaggedRequest;
    PCMC_TAGGED_REQUEST         rgTaggedRequest;
    DWORD                       cTaggedContentInfo;
    PCMC_TAGGED_CONTENT_INFO    rgTaggedContentInfo;
    DWORD                       cTaggedOtherMsg;
    PCMC_TAGGED_OTHER_MSG       rgTaggedOtherMsg;
} CMC_DATA_INFO, *PCMC_DATA_INFO;


// All the tagged arrays are optional
typedef struct _CMC_RESPONSE_INFO {
    DWORD                       cTaggedAttribute;
    PCMC_TAGGED_ATTRIBUTE       rgTaggedAttribute;
    DWORD                       cTaggedContentInfo;
    PCMC_TAGGED_CONTENT_INFO    rgTaggedContentInfo;
    DWORD                       cTaggedOtherMsg;
    PCMC_TAGGED_OTHER_MSG       rgTaggedOtherMsg;
} CMC_RESPONSE_INFO, *PCMC_RESPONSE_INFO;


//+-------------------------------------------------------------------------
//  CMC_STATUS
//
//  Certificate Management Messages over CMS (CMC) Status.
//
//  pvStructInfo points to a CMC_STATUS_INFO.
//--------------------------------------------------------------------------
typedef struct _CMC_PEND_INFO {
    CRYPT_DATA_BLOB             PendToken;
    FILETIME                    PendTime;
} CMC_PEND_INFO, *PCMC_PEND_INFO;

typedef struct _CMC_STATUS_INFO {
    DWORD                       dwStatus;
    DWORD                       cBodyList;
    DWORD                       *rgdwBodyList;
    LPWSTR                      pwszStatusString;   // OPTIONAL
    DWORD                       dwOtherInfoChoice;
    union  {
        // CMC_OTHER_INFO_NO_CHOICE
        //  none
        // CMC_OTHER_INFO_FAIL_CHOICE
        DWORD                       dwFailInfo;
        // CMC_OTHER_INFO_PEND_CHOICE
        PCMC_PEND_INFO              pPendInfo;
    } DUMMYUNIONNAME;
} CMC_STATUS_INFO, *PCMC_STATUS_INFO;

#define CMC_OTHER_INFO_NO_CHOICE        0
#define CMC_OTHER_INFO_FAIL_CHOICE      1
#define CMC_OTHER_INFO_PEND_CHOICE      2

//
// dwStatus values
//

// Request was granted
#define CMC_STATUS_SUCCESS          0

// Request failed, more information elsewhere in the message
#define CMC_STATUS_FAILED           2

// The request body part has not yet been processed. Requester is responsible
// to poll back. May only be returned for certificate request operations.
#define CMC_STATUS_PENDING          3

// The requested operation is not supported
#define CMC_STATUS_NO_SUPPORT       4

// Confirmation using the idConfirmCertAcceptance control is required
// before use of certificate
#define CMC_STATUS_CONFIRM_REQUIRED 5


//
// dwFailInfo values
//

// Unrecognized or unsupported algorithm
#define CMC_FAIL_BAD_ALG            0

// Integrity check failed
#define CMC_FAIL_BAD_MESSAGE_CHECK  1

// Transaction not permitted or supported
#define CMC_FAIL_BAD_REQUEST        2

// Message time field was not sufficiently close to the system time
#define CMC_FAIL_BAD_TIME           3

// No certificate could be identified matching the provided criteria
#define CMC_FAIL_BAD_CERT_ID        4

// A requested X.509 extension is not supported by the recipient CA.
#define CMC_FAIL_UNSUPORTED_EXT     5

// Private key material must be supplied
#define CMC_FAIL_MUST_ARCHIVE_KEYS  6

// Identification Attribute failed to verify
#define CMC_FAIL_BAD_IDENTITY       7

// Server requires a POP proof before issuing certificate
#define CMC_FAIL_POP_REQUIRED       8

// POP processing failed
#define CMC_FAIL_POP_FAILED         9

// Server policy does not allow key re-use
#define CMC_FAIL_NO_KEY_REUSE       10

#define CMC_FAIL_INTERNAL_CA_ERROR  11

#define CMC_FAIL_TRY_LATER          12


//+-------------------------------------------------------------------------
//  CMC_ADD_EXTENSIONS
//
//  Certificate Management Messages over CMS (CMC) Add Extensions control
//  attribute.
//
//  pvStructInfo points to a CMC_ADD_EXTENSIONS_INFO.
//--------------------------------------------------------------------------
typedef struct _CMC_ADD_EXTENSIONS_INFO {
    DWORD                       dwCmcDataReference;
    DWORD                       cCertReference;
    DWORD                       *rgdwCertReference;
    DWORD                       cExtension;
    PCERT_EXTENSION             rgExtension;
} CMC_ADD_EXTENSIONS_INFO, *PCMC_ADD_EXTENSIONS_INFO;


//+-------------------------------------------------------------------------
//  CMC_ADD_ATTRIBUTES
//
//  Certificate Management Messages over CMS (CMC) Add Attributes control
//  attribute.
//
//  pvStructInfo points to a CMC_ADD_ATTRIBUTES_INFO.
//--------------------------------------------------------------------------
typedef struct _CMC_ADD_ATTRIBUTES_INFO {
    DWORD                       dwCmcDataReference;
    DWORD                       cCertReference;
    DWORD                       *rgdwCertReference;
    DWORD                       cAttribute;
    PCRYPT_ATTRIBUTE            rgAttribute;
} CMC_ADD_ATTRIBUTES_INFO, *PCMC_ADD_ATTRIBUTES_INFO;


//+-------------------------------------------------------------------------
//  X509_CERTIFICATE_TEMPLATE
//  szOID_CERTIFICATE_TEMPLATE
//
//  pvStructInfo points to following CERT_TEMPLATE_EXT data structure.
//
//--------------------------------------------------------------------------
typedef struct _CERT_TEMPLATE_EXT {
    LPSTR               pszObjId;
    DWORD               dwMajorVersion;
    BOOL                fMinorVersion;      // TRUE for a minor version
    DWORD               dwMinorVersion;
} CERT_TEMPLATE_EXT, *PCERT_TEMPLATE_EXT;


//+=========================================================================
//  Logotype Extension Data Structures
//
//  X509_LOGOTYPE_EXT
//  szOID_LOGOTYPE_EXT
//
//  pvStructInfo points to a CERT_LOGOTYPE_EXT_INFO.
//==========================================================================
typedef struct _CERT_HASHED_URL {
    CRYPT_ALGORITHM_IDENTIFIER  HashAlgorithm;
    CRYPT_HASH_BLOB             Hash;
    LPWSTR                      pwszUrl;    // Encoded as IA5, Optional for
                                            // biometric data
} CERT_HASHED_URL, *PCERT_HASHED_URL;

typedef struct _CERT_LOGOTYPE_DETAILS {
    LPWSTR                      pwszMimeType;   // Encoded as IA5
    DWORD                       cHashedUrl;
    PCERT_HASHED_URL            rgHashedUrl;
} CERT_LOGOTYPE_DETAILS, *PCERT_LOGOTYPE_DETAILS;

typedef struct _CERT_LOGOTYPE_REFERENCE {
    DWORD                       cHashedUrl;
    PCERT_HASHED_URL            rgHashedUrl;
} CERT_LOGOTYPE_REFERENCE, *PCERT_LOGOTYPE_REFERENCE;

typedef struct _CERT_LOGOTYPE_IMAGE_INFO {
    // CERT_LOGOTYPE_GRAY_SCALE_IMAGE_INFO_CHOICE or
    // CERT_LOGOTYPE_COLOR_IMAGE_INFO_CHOICE
    DWORD                       dwLogotypeImageInfoChoice;

    DWORD                       dwFileSize;     // In octets
    DWORD                       dwXSize;        // Horizontal size in pixels
    DWORD                       dwYSize;        // Vertical size in pixels

    DWORD                       dwLogotypeImageResolutionChoice;
    union {
        // CERT_LOGOTYPE_NO_IMAGE_RESOLUTION_CHOICE
        // No resolution value

        // CERT_LOGOTYPE_BITS_IMAGE_RESOLUTION_CHOICE
        DWORD                       dwNumBits;      // Resolution in bits

        // CERT_LOGOTYPE_TABLE_SIZE_IMAGE_RESOLUTION_CHOICE
        DWORD                       dwTableSize;    // Number of color or grey tones
    } DUMMYUNIONNAME;
    LPWSTR                      pwszLanguage;   // Optional. Encoded as IA5.
                                                // RFC 3066 Language Tag
} CERT_LOGOTYPE_IMAGE_INFO, *PCERT_LOGOTYPE_IMAGE_INFO;

#define CERT_LOGOTYPE_GRAY_SCALE_IMAGE_INFO_CHOICE          1
#define CERT_LOGOTYPE_COLOR_IMAGE_INFO_CHOICE               2

#define CERT_LOGOTYPE_NO_IMAGE_RESOLUTION_CHOICE            0
#define CERT_LOGOTYPE_BITS_IMAGE_RESOLUTION_CHOICE          1
#define CERT_LOGOTYPE_TABLE_SIZE_IMAGE_RESOLUTION_CHOICE    2

typedef struct _CERT_LOGOTYPE_IMAGE {
    CERT_LOGOTYPE_DETAILS       LogotypeDetails;

    PCERT_LOGOTYPE_IMAGE_INFO   pLogotypeImageInfo; // Optional
} CERT_LOGOTYPE_IMAGE, *PCERT_LOGOTYPE_IMAGE;


typedef struct _CERT_LOGOTYPE_AUDIO_INFO {
    DWORD                       dwFileSize;     // In octets
    DWORD                       dwPlayTime;     // In milliseconds
    DWORD                       dwChannels;     // 1=mono, 2=stereo, 4=quad
    DWORD                       dwSampleRate;   // Optional. 0 => not present.
                                                // Samples per second
    LPWSTR                      pwszLanguage;   // Optional. Encoded as IA5.
                                                // RFC 3066 Language Tag
} CERT_LOGOTYPE_AUDIO_INFO, *PCERT_LOGOTYPE_AUDIO_INFO;

typedef struct _CERT_LOGOTYPE_AUDIO {
    CERT_LOGOTYPE_DETAILS       LogotypeDetails;

    PCERT_LOGOTYPE_AUDIO_INFO   pLogotypeAudioInfo; // Optional
} CERT_LOGOTYPE_AUDIO, *PCERT_LOGOTYPE_AUDIO;


typedef struct _CERT_LOGOTYPE_DATA {
    DWORD                       cLogotypeImage;
    PCERT_LOGOTYPE_IMAGE        rgLogotypeImage;

    DWORD                       cLogotypeAudio;
    PCERT_LOGOTYPE_AUDIO        rgLogotypeAudio;
} CERT_LOGOTYPE_DATA, *PCERT_LOGOTYPE_DATA;


typedef struct _CERT_LOGOTYPE_INFO {
    DWORD                       dwLogotypeInfoChoice;
    union {
        // CERT_LOGOTYPE_DIRECT_INFO_CHOICE
        PCERT_LOGOTYPE_DATA         pLogotypeDirectInfo;

        // CERT_LOGOTYPE_INDIRECT_INFO_CHOICE
        PCERT_LOGOTYPE_REFERENCE    pLogotypeIndirectInfo;
    } DUMMYUNIONNAME;
} CERT_LOGOTYPE_INFO, *PCERT_LOGOTYPE_INFO;

#define CERT_LOGOTYPE_DIRECT_INFO_CHOICE    1
#define CERT_LOGOTYPE_INDIRECT_INFO_CHOICE  2

typedef struct _CERT_OTHER_LOGOTYPE_INFO {
    LPSTR                       pszObjId;
    CERT_LOGOTYPE_INFO          LogotypeInfo;
} CERT_OTHER_LOGOTYPE_INFO, *PCERT_OTHER_LOGOTYPE_INFO;

#define szOID_LOYALTY_OTHER_LOGOTYPE                "1.3.6.1.5.5.7.20.1"
#define szOID_BACKGROUND_OTHER_LOGOTYPE             "1.3.6.1.5.5.7.20.2"

typedef struct _CERT_LOGOTYPE_EXT_INFO {
    DWORD                       cCommunityLogo;
    PCERT_LOGOTYPE_INFO         rgCommunityLogo;
    PCERT_LOGOTYPE_INFO         pIssuerLogo;        // Optional
    PCERT_LOGOTYPE_INFO         pSubjectLogo;       // Optional
    DWORD                       cOtherLogo;
    PCERT_OTHER_LOGOTYPE_INFO   rgOtherLogo;
} CERT_LOGOTYPE_EXT_INFO, *PCERT_LOGOTYPE_EXT_INFO;


//+=========================================================================
//  Biometric Extension Data Structures
//
//  X509_BIOMETRIC_EXT
//  szOID_BIOMETRIC_EXT
//
//  pvStructInfo points to following CERT_BIOMETRIC_EXT_INFO data structure.
//==========================================================================

typedef struct _CERT_BIOMETRIC_DATA {
    DWORD                       dwTypeOfBiometricDataChoice;
    union {
        // CERT_BIOMETRIC_PREDEFINED_DATA_CHOICE
        DWORD                       dwPredefined;

        // CERT_BIOMETRIC_OID_DATA_CHOICE
        LPSTR                       pszObjId;
    } DUMMYUNIONNAME;

    CERT_HASHED_URL             HashedUrl;      // pwszUrl is Optional.
} CERT_BIOMETRIC_DATA, *PCERT_BIOMETRIC_DATA;

#define CERT_BIOMETRIC_PREDEFINED_DATA_CHOICE   1
#define CERT_BIOMETRIC_OID_DATA_CHOICE          2

#define CERT_BIOMETRIC_PICTURE_TYPE             0
#define CERT_BIOMETRIC_SIGNATURE_TYPE           1


typedef struct _CERT_BIOMETRIC_EXT_INFO {
    DWORD                       cBiometricData;
    PCERT_BIOMETRIC_DATA        rgBiometricData;
} CERT_BIOMETRIC_EXT_INFO, *PCERT_BIOMETRIC_EXT_INFO;



//+=========================================================================
//  Online Certificate Status Protocol (OCSP) Data Structures
//==========================================================================

//+-------------------------------------------------------------------------
//  OCSP_SIGNED_REQUEST
//
//  OCSP signed request.
//
//  Note, in most instances, pOptionalSignatureInfo will be NULL indicating
//  no signature is present.
//--------------------------------------------------------------------------

typedef struct _OCSP_SIGNATURE_INFO {
    CRYPT_ALGORITHM_IDENTIFIER  SignatureAlgorithm;
    CRYPT_BIT_BLOB              Signature;
    DWORD                       cCertEncoded;
    PCERT_BLOB                  rgCertEncoded;
} OCSP_SIGNATURE_INFO, *POCSP_SIGNATURE_INFO;

typedef struct _OCSP_SIGNED_REQUEST_INFO {
    CRYPT_DER_BLOB              ToBeSigned;             // Encoded OCSP_REQUEST
    POCSP_SIGNATURE_INFO        pOptionalSignatureInfo; // NULL, no signature
} OCSP_SIGNED_REQUEST_INFO, *POCSP_SIGNED_REQUEST_INFO;

//+-------------------------------------------------------------------------
//  OCSP_REQUEST (WINCRYPT_OCSP_REQUEST)
//
//  ToBeSigned OCSP request.
//--------------------------------------------------------------------------

typedef struct _OCSP_CERT_ID {
    CRYPT_ALGORITHM_IDENTIFIER  HashAlgorithm;  // Normally SHA1
    CRYPT_HASH_BLOB             IssuerNameHash; // Hash of encoded name
    CRYPT_HASH_BLOB             IssuerKeyHash;  // Hash of PublicKey bits
    CRYPT_INTEGER_BLOB          SerialNumber;
} OCSP_CERT_ID, *POCSP_CERT_ID;

typedef struct _OCSP_REQUEST_ENTRY {
    OCSP_CERT_ID                CertId;
    DWORD                       cExtension;
    PCERT_EXTENSION             rgExtension;
} OCSP_REQUEST_ENTRY, *POCSP_REQUEST_ENTRY;

typedef struct _OCSP_REQUEST_INFO {
    DWORD                       dwVersion;
    PCERT_ALT_NAME_ENTRY        pRequestorName;     // OPTIONAL
    DWORD                       cRequestEntry;
    POCSP_REQUEST_ENTRY         rgRequestEntry;
    DWORD                       cExtension;
    PCERT_EXTENSION             rgExtension;
} OCSP_REQUEST_INFO, *POCSP_REQUEST_INFO;

#define OCSP_REQUEST_V1     0

//+-------------------------------------------------------------------------
//  OCSP_RESPONSE (WINCRYPT_OCSP_RESPONSE)
//
//  OCSP outer, unsigned response wrapper.
//--------------------------------------------------------------------------
typedef struct _OCSP_RESPONSE_INFO {
    DWORD                       dwStatus;
    LPSTR                       pszObjId;   // OPTIONAL, may be NULL
    CRYPT_OBJID_BLOB            Value;      // OPTIONAL
} OCSP_RESPONSE_INFO, *POCSP_RESPONSE_INFO;

#define OCSP_SUCCESSFUL_RESPONSE            0
#define OCSP_MALFORMED_REQUEST_RESPONSE     1
#define OCSP_INTERNAL_ERROR_RESPONSE        2
#define OCSP_TRY_LATER_RESPONSE             3
// 4 is not used
#define OCSP_SIG_REQUIRED_RESPONSE          5
#define OCSP_UNAUTHORIZED_RESPONSE          6


#define szOID_PKIX_OCSP_BASIC_SIGNED_RESPONSE   "1.3.6.1.5.5.7.48.1.1"

//+-------------------------------------------------------------------------
//  OCSP_BASIC_SIGNED_RESPONSE
//  szOID_PKIX_OCSP_BASIC_SIGNED_RESPONSE
//
//  OCSP basic signed response.
//--------------------------------------------------------------------------
typedef struct _OCSP_BASIC_SIGNED_RESPONSE_INFO {
    CRYPT_DER_BLOB              ToBeSigned;     // Encoded OCSP_BASIC_RESPONSE
    OCSP_SIGNATURE_INFO         SignatureInfo;
} OCSP_BASIC_SIGNED_RESPONSE_INFO, *POCSP_BASIC_SIGNED_RESPONSE_INFO;

//+-------------------------------------------------------------------------
//  OCSP_BASIC_RESPONSE
//
//  ToBeSigned OCSP basic response.
//--------------------------------------------------------------------------

typedef struct _OCSP_BASIC_REVOKED_INFO {
    FILETIME                    RevocationDate;

    // See X509_CRL_REASON_CODE for list of reason codes
    DWORD                       dwCrlReasonCode;
} OCSP_BASIC_REVOKED_INFO, *POCSP_BASIC_REVOKED_INFO;

typedef struct _OCSP_BASIC_RESPONSE_ENTRY {
    OCSP_CERT_ID                CertId;
    DWORD                       dwCertStatus;
    union {
        // OCSP_BASIC_GOOD_CERT_STATUS
        // OCSP_BASIC_UNKNOWN_CERT_STATUS
        //  No additional information

        // OCSP_BASIC_REVOKED_CERT_STATUS
        POCSP_BASIC_REVOKED_INFO    pRevokedInfo;

    } DUMMYUNIONNAME;
    FILETIME                    ThisUpdate;
    FILETIME                    NextUpdate; // Optional, zero filetime implies
                                            // never expires
    DWORD                       cExtension;
    PCERT_EXTENSION             rgExtension;
} OCSP_BASIC_RESPONSE_ENTRY, *POCSP_BASIC_RESPONSE_ENTRY;

#define OCSP_BASIC_GOOD_CERT_STATUS         0
#define OCSP_BASIC_REVOKED_CERT_STATUS      1
#define OCSP_BASIC_UNKNOWN_CERT_STATUS      2


typedef struct _OCSP_BASIC_RESPONSE_INFO {
    DWORD                       dwVersion;
    DWORD                       dwResponderIdChoice;
    union {
        // OCSP_BASIC_BY_NAME_RESPONDER_ID
        CERT_NAME_BLOB              ByNameResponderId;
        // OCSP_BASIC_BY_KEY_RESPONDER_ID
        CRYPT_HASH_BLOB              ByKeyResponderId;
    } DUMMYUNIONNAME;
    FILETIME                    ProducedAt;
    DWORD                       cResponseEntry;
    POCSP_BASIC_RESPONSE_ENTRY  rgResponseEntry;
    DWORD                       cExtension;
    PCERT_EXTENSION             rgExtension;
} OCSP_BASIC_RESPONSE_INFO, *POCSP_BASIC_RESPONSE_INFO;

#define OCSP_BASIC_RESPONSE_V1  0

#define OCSP_BASIC_BY_NAME_RESPONDER_ID     1
#define OCSP_BASIC_BY_KEY_RESPONDER_ID      2


//+=========================================================================
//  TPM CryptEncodeObject/CryptDecodeObject Data Structures
//==========================================================================

//+-------------------------------------------------------------------------
//  szOID_ATTR_SUPPORTED_ALGORITHMS
//
//  pvStructInfo points to following CERT_SUPPORTED_ALGORITHM_INFO.
//--------------------------------------------------------------------------
typedef struct _CERT_SUPPORTED_ALGORITHM_INFO {
    CRYPT_ALGORITHM_IDENTIFIER  Algorithm;
    CRYPT_BIT_BLOB              IntendedKeyUsage;       // OPTIONAL
    CERT_POLICIES_INFO          IntendedCertPolicies;   // OPTIONAL
} CERT_SUPPORTED_ALGORITHM_INFO, *PCERT_SUPPORTED_ALGORITHM_INFO;

//+-------------------------------------------------------------------------
//  szOID_ATTR_TPM_SPECIFICATION
//
//  pvStructInfo points to following CERT_TPM_SPECIFICATION_INFO.
//--------------------------------------------------------------------------
typedef struct _CERT_TPM_SPECIFICATION_INFO {
    LPWSTR                      pwszFamily;             // Encoded as UTF8
    DWORD                       dwLevel;
    DWORD                       dwRevision;
} CERT_TPM_SPECIFICATION_INFO, *PCERT_TPM_SPECIFICATION_INFO;

//+-------------------------------------------------------------------------
//  szOID_ENROLL_KEY_AFFINITY  -- certificate extension
//
//  pvStructInfo points to a CRYPT_SEQUENCE_OF_ANY.
//
//  The two resulting CRYPT_DER_BLOBs point to a salt blob and a hash result.
//  In Pkcs10 requests, the extension will contain an ASN NULL.
//--------------------------------------------------------------------------

//+=========================================================================
//  Object IDentifier (OID) Installable Functions:  Data Structures and APIs
//==========================================================================
typedef void *HCRYPTOIDFUNCSET;
typedef void *HCRYPTOIDFUNCADDR;

// Predefined OID Function Names
#define CRYPT_OID_ENCODE_OBJECT_FUNC        "CryptDllEncodeObject"
#define CRYPT_OID_DECODE_OBJECT_FUNC        "CryptDllDecodeObject"
#define CRYPT_OID_ENCODE_OBJECT_EX_FUNC     "CryptDllEncodeObjectEx"
#define CRYPT_OID_DECODE_OBJECT_EX_FUNC     "CryptDllDecodeObjectEx"
#define CRYPT_OID_CREATE_COM_OBJECT_FUNC    "CryptDllCreateCOMObject"
#define CRYPT_OID_VERIFY_REVOCATION_FUNC    "CertDllVerifyRevocation"
#define CRYPT_OID_VERIFY_CTL_USAGE_FUNC     "CertDllVerifyCTLUsage"
#define CRYPT_OID_FORMAT_OBJECT_FUNC        "CryptDllFormatObject"
#define CRYPT_OID_FIND_OID_INFO_FUNC        "CryptDllFindOIDInfo"
#define CRYPT_OID_FIND_LOCALIZED_NAME_FUNC  "CryptDllFindLocalizedName"


// CryptDllEncodeObject has same function signature as CryptEncodeObject.

// CryptDllDecodeObject has same function signature as CryptDecodeObject.

// CryptDllEncodeObjectEx has same function signature as CryptEncodeObjectEx.
// The Ex version MUST support the CRYPT_ENCODE_ALLOC_FLAG option.
//
// If an Ex function isn't installed or registered, then, attempts to find
// a non-EX version. If the ALLOC flag is set, then, CryptEncodeObjectEx,
// does the allocation and calls the non-EX version twice.

// CryptDllDecodeObjectEx has same function signature as CryptDecodeObjectEx.
// The Ex version MUST support the CRYPT_DECODE_ALLOC_FLAG option.
//
// If an Ex function isn't installed or registered, then, attempts to find
// a non-EX version. If the ALLOC flag is set, then, CryptDecodeObjectEx,
// does the allocation and calls the non-EX version twice.

// CryptDllCreateCOMObject has the following signature:
//      BOOL WINAPI CryptDllCreateCOMObject(
//          _In_ DWORD dwEncodingType,
//          _In_ LPCSTR pszOID,
//          __In PCRYPT_DATA_BLOB pEncodedContent,
//          _In_ DWORD dwFlags,
//          _In_ REFIID riid,
//          _Outptr_ void **ppvObj);

// CertDllVerifyRevocation has the same signature as CertVerifyRevocation
//  (See CertVerifyRevocation for details on when called)

// CertDllVerifyCTLUsage has the same signature as CertVerifyCTLUsage

// CryptDllFindOIDInfo currently is only used to store values used by
// CryptFindOIDInfo. See CryptFindOIDInfo() for more details.

// CryptDllFindLocalizedName is only used to store localized string
// values used by CryptFindLocalizedName. See CryptFindLocalizedName() for
// more details.

//  Example of a complete OID Function Registry Name:
//    HKEY_LOCAL_MACHINE\Software\Microsoft\Cryptography\OID
//      Encoding Type 1\CryptDllEncodeObject\1.2.3
//
//  The key's L"Dll" value contains the name of the Dll.
//  The key's L"FuncName" value overrides the default function name
#define CRYPT_OID_REGPATH "Software\\Microsoft\\Cryptography\\OID"
#define CRYPT_OID_REG_ENCODING_TYPE_PREFIX  "EncodingType "
#define CRYPT_OID_REG_DLL_VALUE_NAME        L"Dll"
#define CRYPT_OID_REG_FUNC_NAME_VALUE_NAME  L"FuncName"
#define CRYPT_OID_REG_FUNC_NAME_VALUE_NAME_A "FuncName"

// CRYPT_INSTALL_OID_FUNC_BEFORE_FLAG can be set in the key's L"CryptFlags"
// value to register the functions before the installed functions.
//
// CryptSetOIDFunctionValue must be called to set this value. L"CryptFlags"
// must be set using a dwValueType of REG_DWORD.
#define CRYPT_OID_REG_FLAGS_VALUE_NAME      L"CryptFlags"

// OID used for Default OID functions
#define CRYPT_DEFAULT_OID                   "DEFAULT"

typedef struct _CRYPT_OID_FUNC_ENTRY {
    LPCSTR  pszOID;
    void    *pvFuncAddr;
} CRYPT_OID_FUNC_ENTRY, *PCRYPT_OID_FUNC_ENTRY;

#define CRYPT_INSTALL_OID_FUNC_BEFORE_FLAG  1

//+-------------------------------------------------------------------------
//  Install a set of callable OID function addresses.
//
//  By default the functions are installed at end of the list.
//  Set CRYPT_INSTALL_OID_FUNC_BEFORE_FLAG to install at beginning of list.
//
//  hModule should be updated with the hModule passed to DllMain to prevent
//  the Dll containing the function addresses from being unloaded by
//  CryptGetOIDFuncAddress/CryptFreeOIDFunctionAddress. This would be the
//  case when the Dll has also regsvr32'ed OID functions via
//  CryptRegisterOIDFunction.
//
//  DEFAULT functions are installed by setting rgFuncEntry[].pszOID =
//  CRYPT_DEFAULT_OID.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptInstallOIDFunctionAddress(
    _In_opt_ HMODULE hModule,         // hModule passed to DllMain
    _In_ DWORD dwEncodingType,
    _In_ LPCSTR pszFuncName,
    _In_ DWORD cFuncEntry,
    _In_reads_(cFuncEntry) const CRYPT_OID_FUNC_ENTRY rgFuncEntry[],
    _In_ DWORD dwFlags
    );

//+-------------------------------------------------------------------------
//  Initialize and return handle to the OID function set identified by its
//  function name.
//
//  If the set already exists, a handle to the existing set is returned.
//--------------------------------------------------------------------------
WINCRYPT32API
HCRYPTOIDFUNCSET
WINAPI
CryptInitOIDFunctionSet(
    _In_ LPCSTR pszFuncName,
    _In_ DWORD dwFlags
    );

//+-------------------------------------------------------------------------
//  Search the list of installed functions for an encoding type and OID match.
//  If not found, search the registry.
//
//  For success, returns TRUE with *ppvFuncAddr updated with the function's
//  address and *phFuncAddr updated with the function address's handle.
//  The function's handle is AddRef'ed. CryptFreeOIDFunctionAddress needs to
//  be called to release it.
//
//  For a registry match, the Dll containing the function is loaded.
//
//  By default, both the registered and installed function lists are searched.
//  Set CRYPT_GET_INSTALLED_OID_FUNC_FLAG to only search the installed list
//  of functions. This flag would be set by a registered function to get
//  the address of a pre-installed function it was replacing. For example,
//  the registered function might handle a new special case and call the
//  pre-installed function to handle the remaining cases.
//--------------------------------------------------------------------------
WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CryptGetOIDFunctionAddress(
    _In_ HCRYPTOIDFUNCSET hFuncSet,
    _In_ DWORD dwEncodingType,
    _In_ LPCSTR pszOID,
    _In_ DWORD dwFlags,
    _Outptr_ void **ppvFuncAddr,
    _Out_ HCRYPTOIDFUNCADDR *phFuncAddr
    );

#define CRYPT_GET_INSTALLED_OID_FUNC_FLAG       0x1

//+-------------------------------------------------------------------------
//  Get the list of registered default Dll entries for the specified
//  function set and encoding type.
//
//  The returned list consists of none, one or more null terminated Dll file
//  names. The list is terminated with an empty (L"\0") Dll file name.
//  For example: L"first.dll" L"\0" L"second.dll" L"\0" L"\0"
//--------------------------------------------------------------------------
WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CryptGetDefaultOIDDllList(
    _In_ HCRYPTOIDFUNCSET hFuncSet,
    _In_ DWORD dwEncodingType,
    _Out_writes_to_opt_(*pcchDllList, *pcchDllList) _Post_ _NullNull_terminated_ WCHAR *pwszDllList,
    _Inout_ DWORD *pcchDllList
    );

//+-------------------------------------------------------------------------
//  Either: get the first or next installed DEFAULT function OR
//  load the Dll containing the DEFAULT function.
//
//  If pwszDll is NULL, search the list of installed DEFAULT functions.
//  *phFuncAddr must be set to NULL to get the first installed function.
//  Successive installed functions are returned by setting *phFuncAddr
//  to the hFuncAddr returned by the previous call.
//
//  If pwszDll is NULL, the input *phFuncAddr
//  is always CryptFreeOIDFunctionAddress'ed by this function, even for
//  an error.
//
//  If pwszDll isn't NULL, then, attempts to load the Dll and the DEFAULT
//  function. *phFuncAddr is ignored upon entry and isn't
//  CryptFreeOIDFunctionAddress'ed.
//
//  For success, returns TRUE with *ppvFuncAddr updated with the function's
//  address and *phFuncAddr updated with the function address's handle.
//  The function's handle is AddRef'ed. CryptFreeOIDFunctionAddress needs to
//  be called to release it or CryptGetDefaultOIDFunctionAddress can also
//  be called for a NULL pwszDll.
//--------------------------------------------------------------------------
WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CryptGetDefaultOIDFunctionAddress(
    _In_ HCRYPTOIDFUNCSET hFuncSet,
    _In_ DWORD dwEncodingType,
    _In_opt_ LPCWSTR pwszDll,
    _In_ DWORD dwFlags,
    _Outptr_ void **ppvFuncAddr,
    _Inout_ HCRYPTOIDFUNCADDR *phFuncAddr
    );

//+-------------------------------------------------------------------------
//  Releases the handle AddRef'ed and returned by CryptGetOIDFunctionAddress
//  or CryptGetDefaultOIDFunctionAddress.
//
//  If a Dll was loaded for the function its unloaded. However, before doing
//  the unload, the DllCanUnloadNow function exported by the loaded Dll is
//  called. It should return S_FALSE to inhibit the unload or S_TRUE to enable
//  the unload. If the Dll doesn't export DllCanUnloadNow, the Dll is unloaded.
//
//  DllCanUnloadNow has the following signature:
//      STDAPI  DllCanUnloadNow(void);
//--------------------------------------------------------------------------

WINCRYPT32API
BOOL
WINAPI
CryptFreeOIDFunctionAddress(
    _In_ HCRYPTOIDFUNCADDR hFuncAddr,
    _In_ DWORD dwFlags
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Register the Dll containing the function to be called for the specified
//  encoding type, function name and OID.
//
//  pwszDll may contain environment-variable strings
//  which are ExpandEnvironmentStrings()'ed before loading the Dll.
//
//  In addition to registering the DLL, you may override the
//  name of the function to be called. For example,
//      pszFuncName = "CryptDllEncodeObject",
//      pszOverrideFuncName = "MyEncodeXyz".
//  This allows a Dll to export multiple OID functions for the same
//  function name without needing to interpose its own OID dispatcher function.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptRegisterOIDFunction(
    _In_ DWORD dwEncodingType,
    _In_ LPCSTR pszFuncName,
    _In_ LPCSTR pszOID,
    _In_opt_ LPCWSTR pwszDll,
    _In_opt_ LPCSTR pszOverrideFuncName
    );

//+-------------------------------------------------------------------------
//  Unregister the Dll containing the function to be called for the specified
//  encoding type, function name and OID.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptUnregisterOIDFunction(
    _In_ DWORD dwEncodingType,
    _In_ LPCSTR pszFuncName,
    _In_ LPCSTR pszOID
    );

//+-------------------------------------------------------------------------
//  Register the Dll containing the default function to be called for the
//  specified encoding type and function name.
//
//  Unlike CryptRegisterOIDFunction, you can't override the function name
//  needing to be exported by the Dll.
//
//  The Dll is inserted before the entry specified by dwIndex.
//    dwIndex == 0, inserts at the beginning.
//    dwIndex == CRYPT_REGISTER_LAST_INDEX, appends at the end.
//
//  pwszDll may contain environment-variable strings
//  which are ExpandEnvironmentStrings()'ed before loading the Dll.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptRegisterDefaultOIDFunction(
    _In_ DWORD dwEncodingType,
    _In_ LPCSTR pszFuncName,
    _In_ DWORD dwIndex,
    _In_ LPCWSTR pwszDll
    );

#define CRYPT_REGISTER_FIRST_INDEX   0
#define CRYPT_REGISTER_LAST_INDEX    0xFFFFFFFF

//+-------------------------------------------------------------------------
//  Unregister the Dll containing the default function to be called for
//  the specified encoding type and function name.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptUnregisterDefaultOIDFunction(
    _In_ DWORD dwEncodingType,
    _In_ LPCSTR pszFuncName,
    _In_ LPCWSTR pwszDll
    );

//+-------------------------------------------------------------------------
//  Set the value for the specified encoding type, function name, OID and
//  value name.
//
//  See RegSetValueEx for the possible value types.
//
//  String types are UNICODE.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptSetOIDFunctionValue(
    _In_ DWORD dwEncodingType,
    _In_ LPCSTR pszFuncName,
    _In_ LPCSTR pszOID,
    _In_opt_ LPCWSTR pwszValueName,
    _In_ DWORD dwValueType,
    _In_reads_bytes_opt_(cbValueData) const BYTE *pbValueData,
    _In_ DWORD cbValueData
    );

//+-------------------------------------------------------------------------
//  Get the value for the specified encoding type, function name, OID and
//  value name.
//
//  See RegEnumValue for the possible value types.
//
//  String types are UNICODE.
//--------------------------------------------------------------------------

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINCRYPT32API
BOOL
WINAPI
CryptGetOIDFunctionValue(
    _In_ DWORD dwEncodingType,
    _In_ LPCSTR pszFuncName,
    _In_ LPCSTR pszOID,
    _In_opt_ LPCWSTR pwszValueName,
    _Out_opt_ DWORD *pdwValueType,
    _Out_writes_bytes_to_opt_(*pcbValueData, *pcbValueData) BYTE *pbValueData,
    _Inout_opt_ DWORD *pcbValueData
    );

typedef BOOL (WINAPI *PFN_CRYPT_ENUM_OID_FUNC)(
    _In_ DWORD dwEncodingType,
    _In_ LPCSTR pszFuncName,
    _In_ LPCSTR pszOID,
    _In_ DWORD cValue,
    _In_reads_(cValue) const DWORD rgdwValueType[],
    _In_reads_(cValue) LPCWSTR const rgpwszValueName[],
    _In_reads_(cValue) const BYTE * const rgpbValueData[],
    _In_reads_(cValue) const DWORD rgcbValueData[],
    _Inout_opt_ void *pvArg
    );

//+-------------------------------------------------------------------------
//  Enumerate the OID functions identified by their encoding type,
//  function name and OID.
//
//  pfnEnumOIDFunc is called for each registry key matching the input
//  parameters. Setting dwEncodingType to CRYPT_MATCH_ANY_ENCODING_TYPE matches
//  any. Setting pszFuncName or pszOID to NULL matches any.
//
//  Set pszOID == CRYPT_DEFAULT_OID to restrict the enumeration to only the
//  DEFAULT functions
//
//  String types are UNICODE.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptEnumOIDFunction(
    _In_ DWORD dwEncodingType,
    _In_opt_ LPCSTR pszFuncName,
    _In_opt_ LPCSTR pszOID,
    _In_ DWORD dwFlags,
    _Inout_opt_ void *pvArg,
    __callback PFN_CRYPT_ENUM_OID_FUNC pfnEnumOIDFunc
    );

#define CRYPT_MATCH_ANY_ENCODING_TYPE   0xFFFFFFFF

//+=========================================================================
//  Object IDentifier (OID) Information:  Data Structures and APIs
//==========================================================================

//+-------------------------------------------------------------------------
//  Special ALG_ID's used in CRYPT_OID_INFO
//--------------------------------------------------------------------------
// Algorithm is only implemented in CNG.
#define CALG_OID_INFO_CNG_ONLY                   0xFFFFFFFF

// Algorithm is defined in the encoded parameters. Only supported
// using CNG.
#define CALG_OID_INFO_PARAMETERS                 0xFFFFFFFE

// PQ Algorithm 
#define CALG_OID_INFO_PQ                         0xFFFFFFFD

// Macro to check for a special ALG_ID used in CRYPT_OID_INFO
#define IS_SPECIAL_OID_INFO_ALGID(Algid)        (Algid >= CALG_OID_INFO_PQ)


//+-------------------------------------------------------------------------
// Special CNG Algorithms used in CRYPT_OID_INFO
//--------------------------------------------------------------------------
#define CRYPT_OID_INFO_HASH_PARAMETERS_ALGORITHM L"CryptOIDInfoHashParameters"
#define CRYPT_OID_INFO_ECC_PARAMETERS_ALGORITHM  L"CryptOIDInfoECCParameters"
#define CRYPT_OID_INFO_MGF1_PARAMETERS_ALGORITHM L"CryptOIDInfoMgf1Parameters"
#define CRYPT_OID_INFO_NO_SIGN_ALGORITHM         L"CryptOIDInfoNoSign"
#define CRYPT_OID_INFO_OAEP_PARAMETERS_ALGORITHM L"CryptOIDInfoOAEPParameters"
#define CRYPT_OID_INFO_ECC_WRAP_PARAMETERS_ALGORITHM L"CryptOIDInfoECCWrapParameters"
#define CRYPT_OID_INFO_NO_PARAMETERS_ALGORITHM   L"CryptOIDInfoNoParameters"
#define CRYPT_OID_INFO_NO_HASH_ALGORITHM         L"NoHash"
#define CRYPT_OID_INFO_PREHASH_ALGORITHM         L"PreHash"
#define CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM(pwszCNGAlgid, pwszCNGParaSetName) \
    pwszCNGAlgid L":" pwszCNGParaSetName

// L"ML-DSA:44"
#define CRYPT32_MLDSA_44_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_MLDSA_ALGORITHM, BCRYPT_MLDSA_PARAMETER_SET_44)
// L"ML-DSA:65"
#define CRYPT32_MLDSA_65_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_MLDSA_ALGORITHM, BCRYPT_MLDSA_PARAMETER_SET_65)
// L"ML-DSA:87"
#define CRYPT32_MLDSA_87_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_MLDSA_ALGORITHM, BCRYPT_MLDSA_PARAMETER_SET_87)

// L"ML-KEM:512"
#define CRYPT32_MLKEM_512_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_MLKEM_ALGORITHM, BCRYPT_MLKEM_PARAMETER_SET_512)
// L"ML-KEM:768"
#define CRYPT32_MLKEM_768_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_MLKEM_ALGORITHM, BCRYPT_MLKEM_PARAMETER_SET_768)
// L"ML-KEM:1024"
#define CRYPT32_MLKEM_1024_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_MLKEM_ALGORITHM, BCRYPT_MLKEM_PARAMETER_SET_1024)

// There will be one BCrypt SLHDSA parameter set and one Crypt32
// SLHDSA algorithm applicable to both the "Pure" and "PreHash"

// L"SLH-DSA:SHA2-128s"
#define CRYPT32_SLHDSA_SHA2_128S_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_SLHDSA_ALGORITHM, BCRYPT_SLHDSA_PARAMETER_SET_SHA2_128S)
// L"SLH-DSA:SHAKE-128s"
#define CRYPT32_SLHDSA_SHAKE_128S_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_SLHDSA_ALGORITHM, BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_128S)
// L"SLH-DSA:SHA2-128f"
#define CRYPT32_SLHDSA_SHA2_128F_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_SLHDSA_ALGORITHM, BCRYPT_SLHDSA_PARAMETER_SET_SHA2_128F)
// L"SLH-DSA:SHAKE-128f"
#define CRYPT32_SLHDSA_SHAKE_128F_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_SLHDSA_ALGORITHM, BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_128F)
// L"SLH-DSA:SHA2-192s"
#define CRYPT32_SLHDSA_SHA2_192S_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_SLHDSA_ALGORITHM, BCRYPT_SLHDSA_PARAMETER_SET_SHA2_192S)
// L"SLH-DSA:SHAKE-192s"
#define CRYPT32_SLHDSA_SHAKE_192S_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_SLHDSA_ALGORITHM, BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_192S)
// L"SLH-DSA:SHA2-192f"
#define CRYPT32_SLHDSA_SHA2_192F_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_SLHDSA_ALGORITHM, BCRYPT_SLHDSA_PARAMETER_SET_SHA2_192F)
// L"SLH-DSA:SHAKE-192f"
#define CRYPT32_SLHDSA_SHAKE_192F_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_SLHDSA_ALGORITHM, BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_192F)
// L"SLH-DSA:SHA2-256s"
#define CRYPT32_SLHDSA_SHA2_256S_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_SLHDSA_ALGORITHM, BCRYPT_SLHDSA_PARAMETER_SET_SHA2_256S)
// L"SLH-DSA:SHAKE-256s"
#define CRYPT32_SLHDSA_SHAKE_256S_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_SLHDSA_ALGORITHM, BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_256S)
// L"SLH-DSA:SHA2-256f"
#define CRYPT32_SLHDSA_SHA2_256F_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_SLHDSA_ALGORITHM, BCRYPT_SLHDSA_PARAMETER_SET_SHA2_256F)
// L"SLH-DSA:SHAKE-256f"
#define CRYPT32_SLHDSA_SHAKE_256F_ALGORITHM CRYPT_OID_INFO_MAKE_PARAMETER_SET_ALGORITHM( \
    BCRYPT_SLHDSA_ALGORITHM, BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_256F)

//+-------------------------------------------------------------------------
//  OID Information
//--------------------------------------------------------------------------
typedef struct _CRYPT_OID_INFO {
    DWORD           cbSize;
    LPCSTR          pszOID;
    LPCWSTR         pwszName;
    DWORD           dwGroupId;
    union {
        DWORD       dwValue;
        ALG_ID      Algid;
        DWORD       dwLength;
    } DUMMYUNIONNAME;
    CRYPT_DATA_BLOB ExtraInfo;


#ifdef CRYPT_OID_INFO_HAS_EXTRA_FIELDS
    // Note, if you #define CRYPT_OID_INFO_HAS_EXTRA_FIELDS, then, you
    // must zero all unused fields in this data structure.
    // More fields could be added in a future release.

    // The following 2 fields are set to an empty string, L"", if not defined.

    // This is the Algid string passed to the BCrypt* and NCrypt* APIs
    // defined in bcrypt.h and ncrypt.h.
    //
    // Its only applicable to the following groups:
    //  CRYPT_HASH_ALG_OID_GROUP_ID
    //  CRYPT_ENCRYPT_ALG_OID_GROUP_ID
    //  CRYPT_PUBKEY_ALG_OID_GROUP_ID
    //      For PQ, assume pwszCNGAlgid has a 1-to-1
    //      mapping between OID and CNG PQ algorithm name.
    //  CRYPT_SIGN_ALG_OID_GROUP_ID
    //      For PQ, for hash then sign, the hash algorithm.
    //      Otherwise for no hash, L"NoHash". 
    LPCWSTR         pwszCNGAlgid;

    // Following is only applicable to the following groups:
    //  CRYPT_SIGN_ALG_OID_GROUP_ID
    //      The public key pwszCNGAlgid. For ECC,
    //      CRYPT_OID_INFO_ECC_PARAMETERS_ALGORITHM.
    //      For PQ, pwszCNGExtraAlgid will be the CNG PQ Algorithm name.
    //  CRYPT_PUBKEY_ALG_OID_GROUP_ID
    //      For the ECC algorithms, CRYPT_OID_INFO_ECC_PARAMETERS_ALGORITHM.
    LPCWSTR         pwszCNGExtraAlgid;
#endif
} CRYPT_OID_INFO, *PCRYPT_OID_INFO;
typedef const CRYPT_OID_INFO CCRYPT_OID_INFO, *PCCRYPT_OID_INFO;

// certenrolld_begin -- CRYPT_*_OID_GROUP_ID
//+-------------------------------------------------------------------------
//  OID Group IDs
//--------------------------------------------------------------------------
#define CRYPT_HASH_ALG_OID_GROUP_ID             1
#define CRYPT_ENCRYPT_ALG_OID_GROUP_ID          2
#define CRYPT_PUBKEY_ALG_OID_GROUP_ID           3
#define CRYPT_SIGN_ALG_OID_GROUP_ID             4
#define CRYPT_RDN_ATTR_OID_GROUP_ID             5
#define CRYPT_EXT_OR_ATTR_OID_GROUP_ID          6
#define CRYPT_ENHKEY_USAGE_OID_GROUP_ID         7
#define CRYPT_POLICY_OID_GROUP_ID               8
#define CRYPT_TEMPLATE_OID_GROUP_ID             9
#define CRYPT_KDF_OID_GROUP_ID                  10
#define CRYPT_LAST_OID_GROUP_ID                 10

#define CRYPT_FIRST_ALG_OID_GROUP_ID            CRYPT_HASH_ALG_OID_GROUP_ID
#define CRYPT_LAST_ALG_OID_GROUP_ID             CRYPT_SIGN_ALG_OID_GROUP_ID
// certenrolld_end


// The CRYPT_*_ALG_OID_GROUP_ID's have an Algid. The CRYPT_RDN_ATTR_OID_GROUP_ID
// has a dwLength. The CRYPT_EXT_OR_ATTR_OID_GROUP_ID,
// CRYPT_ENHKEY_USAGE_OID_GROUP_ID, CRYPT_POLICY_OID_GROUP_ID or
// CRYPT_TEMPLATE_OID_GROUP_ID don't have a dwValue.
//

// CRYPT_ENCRYPT_ALG_OID_GROUP_ID has the following optional ExtraInfo
// for AES algorithms:
//  DWORD[0] - dwBitLength

// CRYPT_PUBKEY_ALG_OID_GROUP_ID has the following optional ExtraInfo:
//  DWORD[0] - Flags. CRYPT_OID_INHIBIT_SIGNATURE_FORMAT_FLAG can be set to
//             inhibit the reformatting of the signature before
//             CryptVerifySignature is called or after CryptSignHash
//             is called. CRYPT_OID_USE_PUBKEY_PARA_FOR_PKCS7_FLAG can
//             be set to include the public key algorithm's parameters
//             in the PKCS7's digestEncryptionAlgorithm's parameters.
//             CRYPT_OID_NO_NULL_ALGORITHM_PARA_FLAG can be set to omit
//             NULL parameters when encoding.
//
// For the ECC named curve public keys
//  DWORD[1] - BCRYPT_ECCKEY_BLOB dwMagic field value
//  DWORD[2] - dwBitLength. Where BCRYPT_ECCKEY_BLOB's
//             cbKey = dwBitLength / 8 + ((dwBitLength % 8) ? 1 : 0)
//
// For PQ
//  DWORD[0] - Flags
//  DWORD[1] - Public Magic
//  DWORD[2] - Private Magic
//  DWORD[3] - Public Key Byte Length
//  DWORD[4] - Private Key Byte Length
//  DWORD[5] - Signature Byte Length

#define CRYPT_OID_PQ_EXTRA_INFO_FLAGS_INDEX                   0
#define CRYPT_OID_PQ_EXTRA_INFO_PUBLIC_MAGIC_INDEX            1
#define CRYPT_OID_PQ_EXTRA_INFO_PRIVATE_MAGIC_INDEX           2
#define CRYPT_OID_PQ_EXTRA_INFO_PUBLIC_KEY_LENGTH_INDEX       3
#define CRYPT_OID_PQ_EXTRA_INFO_PRIVATE_KEY_LENGTH_INDEX      4
#define CRYPT_OID_PQ_EXTRA_INFO_SIGNATURE_LENGTH_INDEX        5
#define CRYPT_OID_PQ_EXTRA_INFO_MAX_LENGTH                    6

#define CRYPT_OID_INHIBIT_SIGNATURE_FORMAT_FLAG         0x00000001
#define CRYPT_OID_USE_PUBKEY_PARA_FOR_PKCS7_FLAG        0x00000002
#define CRYPT_OID_NO_NULL_ALGORITHM_PARA_FLAG           0x00000004

#define CRYPT_OID_PUBKEY_SIGN_ONLY_FLAG                 0x80000000
#define CRYPT_OID_PUBKEY_ENCRYPT_ONLY_FLAG              0x40000000
#define CRYPT_OID_USE_CURVE_NAME_FOR_ENCODE_FLAG        0x20000000
#define CRYPT_OID_USE_CURVE_PARAMETERS_FOR_ENCODE_FLAG  0x10000000

// A PQ key can only be used for either "Pure" or "PreHash" signing.
// For "PreHash" a single hash algorithm per PQ parameter set
#define CRYPT_OID_PUBKEY_PURE_ONLY_FLAG                 0x08000000
#define CRYPT_OID_PUBKEY_PREHASH_ONLY_FLAG              0x04000000

// CRYPT_SIGN_ALG_OID_GROUP_ID has the following optional ExtraInfo:
//  DWORD[0] - Public Key Algid.
//  DWORD[1] - Flags. Same as above for CRYPT_PUBKEY_ALG_OID_GROUP_ID.
//  DWORD[2] - Optional CryptAcquireContext(CRYPT_VERIFYCONTEXT)'s dwProvType.
//             If omitted or 0, uses Public Key Algid to select
//             appropriate dwProvType for signature verification.
//
// For PQ
//  DWORD[1] - Flags
//  DWORD[2] - Signature Byte Length

// CRYPT_RDN_ATTR_OID_GROUP_ID has the following optional ExtraInfo:
//  Array of DWORDs:
//   [0 ..] - Null terminated list of acceptable RDN attribute
//            value types. An empty list implies CERT_RDN_PRINTABLE_STRING,
//            CERT_RDN_UNICODE_STRING, 0.

//+-------------------------------------------------------------------------
//  Find OID information. Returns NULL if unable to find any information
//  for the specified key and group. Note, returns a pointer to a constant
//  data structure. The returned pointer MUST NOT be freed.
//
//  dwKeyType's:
//    CRYPT_OID_INFO_OID_KEY, pvKey points to a szOID
//    CRYPT_OID_INFO_NAME_KEY, pvKey points to a wszName
//    CRYPT_OID_INFO_ALGID_KEY, pvKey points to an ALG_ID
//    CRYPT_OID_INFO_SIGN_KEY, pvKey points to an array of two ALG_ID's:
//      ALG_ID[0] - Hash Algid
//      ALG_ID[1] - PubKey Algid
//    CRYPT_OID_INFO_CNG_ALGID_KEY, pvKey points to a wszCNGAlgid
//    CRYPT_OID_INFO_CNG_SIGN_KEY, pvKey is an array of two
//     pwszCNGAlgid's:
//      Algid[0] - Hash pwszCNGAlgid
//      Algid[1] - PubKey pwszCNGAlgid
//
//  For CRYPT_OID_INFO_NAME_KEY, CRYPT_OID_INFO_CNG_ALGID_KEY and
//  CRYPT_OID_INFO_CNG_SIGN_KEY the string comparison is case insensitive.
//
//  Setting dwGroupId to 0, searches all groups according to the dwKeyType.
//  Otherwise, only the dwGroupId is searched.
//--------------------------------------------------------------------------
WINCRYPT32API
PCCRYPT_OID_INFO
WINAPI
CryptFindOIDInfo(
    _In_ DWORD dwKeyType,
    _In_ void *pvKey,
    _In_ DWORD dwGroupId
    );

#define CRYPT_OID_INFO_OID_KEY           1
#define CRYPT_OID_INFO_NAME_KEY          2
#define CRYPT_OID_INFO_ALGID_KEY         3
#define CRYPT_OID_INFO_SIGN_KEY          4
#define CRYPT_OID_INFO_CNG_ALGID_KEY     5
#define CRYPT_OID_INFO_CNG_SIGN_KEY      6

// Set the following in the above dwKeyType parameter to restrict public keys
// valid for signing or encrypting
// certenrolld_begin -- CRYPT_*_KEY_FLAG
#define CRYPT_OID_INFO_OID_KEY_FLAGS_MASK           0xFFFF0000
#define CRYPT_OID_INFO_PUBKEY_SIGN_KEY_FLAG         0x80000000
#define CRYPT_OID_INFO_PUBKEY_ENCRYPT_KEY_FLAG      0x40000000

// Set the following in the above dwKeyType parameter to restrict public keys
// valid for "Pure" or "PreHash" signing
#define CRYPT_OID_INFO_PUBKEY_PURE_KEY_FLAG         0x08000000
#define CRYPT_OID_INFO_PUBKEY_PREHASH_KEY_FLAG      0x04000000

// The following flag can be set in above dwGroupId parameter to disable
// searching the directory server
#define CRYPT_OID_DISABLE_SEARCH_DS_FLAG            0x80000000

#ifdef CRYPT_OID_INFO_HAS_EXTRA_FIELDS

// The following flag can be set in above dwGroupId parameter to search
// through CRYPT_OID_INFO records. If there are multiple records that meet
// the search criteria, the first record with defined pwszCNGAlgid would be
// returned. If none of the records (meeting the search criteria) have
// pwszCNGAlgid defined, first record (meeting the search criteria) would be
// returned.
#define CRYPT_OID_PREFER_CNG_ALGID_FLAG            0x40000000

#endif

// certenrolld_end -- CRYPT_*_KEY_FLAG

// The bit length shifted left 16 bits can be OR'ed into the above
// dwGroupId parameter. Only applicable to the CRYPT_ENCRYPT_ALG_OID_GROUP_ID.
// Also, only applicable to encryption algorithms having a dwBitLen ExtraInfo.
// Currently, only the AES encryption algorithms have this.
//
// For example, to find the OIDInfo for BCRYPT_AES_ALGORITHM, bit length 192,
// CryptFindOIDInfo would be called as follows:
//  PCCRYPT_OID_INFO pOIDInfo =
//      CryptFindOIDInfo(
//          CRYPT_OID_INFO_CNG_ALGID_KEY,
//          (void *) BCRYPT_AES_ALGORITHM,
//          CRYPT_ENCRYPT_ALG_OID_GROUP_ID |
//              (192 << CRYPT_OID_INFO_OID_GROUP_BIT_LEN_SHIFT)
//          );

#define CRYPT_OID_INFO_OID_GROUP_BIT_LEN_MASK       0x0FFF0000
#define CRYPT_OID_INFO_OID_GROUP_BIT_LEN_SHIFT      16

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Register OID information. The OID information specified in the
//  CCRYPT_OID_INFO structure is persisted to the registry.
//
//  crypt32.dll contains information for the commonly known OIDs. This function
//  allows applications to augment crypt32.dll's OID information. During
//  CryptFindOIDInfo's first call, the registered OID information is installed.
//
//  By default the registered OID information is installed after crypt32.dll's
//  OID entries. Set CRYPT_INSTALL_OID_INFO_BEFORE_FLAG to install before.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptRegisterOIDInfo(
    _In_ PCCRYPT_OID_INFO pInfo,
    _In_ DWORD dwFlags
    );

#define CRYPT_INSTALL_OID_INFO_BEFORE_FLAG  1

//+-------------------------------------------------------------------------
//  Unregister OID information. Only the pszOID and dwGroupId fields are
//  used to identify the OID information to be unregistered.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptUnregisterOIDInfo(
    _In_ PCCRYPT_OID_INFO pInfo
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

// If the callback returns FALSE, stops the enumeration.
typedef BOOL (WINAPI *PFN_CRYPT_ENUM_OID_INFO)(
    _In_ PCCRYPT_OID_INFO pInfo,
    _Inout_opt_ void *pvArg
    );

//+-------------------------------------------------------------------------
//  Enumerate the OID information.
//
//  pfnEnumOIDInfo is called for each OID information entry.
//
//  Setting dwGroupId to 0 matches all groups. Otherwise, only enumerates
//  entries in the specified group.
//
//  dwFlags currently isn't used and must be set to 0.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptEnumOIDInfo(
    _In_ DWORD dwGroupId,
    _In_ DWORD dwFlags,
    _Inout_opt_ void *pvArg,
    __callback PFN_CRYPT_ENUM_OID_INFO pfnEnumOIDInfo
    );

//+-------------------------------------------------------------------------
//  Find the localized name for the specified name. For example, find the
//  localized name for the "Root" system store name. A case insensitive
//  string comparison is done.
//
//  Returns NULL if unable to find the the specified name.
//
//  Localized names for the predefined system stores ("Root", "My") and
//  predefined physical stores (".Default", ".LocalMachine") are pre-installed
//  as resource strings in crypt32.dll. CryptSetOIDFunctionValue can be called
//  as follows to register additional localized strings:
//      dwEncodingType = CRYPT_LOCALIZED_NAME_ENCODING_TYPE
//      pszFuncName = CRYPT_OID_FIND_LOCALIZED_NAME_FUNC
//      pszOID = CRYPT_LOCALIZED_NAME_OID
//      pwszValueName = Name to be localized, for example, L"ApplicationStore"
//      dwValueType = REG_SZ
//      pbValueData = pointer to the UNICODE localized string
//      cbValueData = (wcslen(UNICODE localized string) + 1) * sizeof(WCHAR)
//
//  To unregister, set pbValueData to NULL and cbValueData to 0.
//
//  The registered names are searched before the pre-installed names.
//--------------------------------------------------------------------------
WINCRYPT32API
LPCWSTR
WINAPI
CryptFindLocalizedName(
    _In_ LPCWSTR pwszCryptName
    );

#define CRYPT_LOCALIZED_NAME_ENCODING_TYPE  0
#define CRYPT_LOCALIZED_NAME_OID            "LocalizedNames"

//+=========================================================================
//  Certificate Strong Signature Defines and Data Structures
//==========================================================================

typedef struct _CERT_STRONG_SIGN_SERIALIZED_INFO {
    DWORD                   dwFlags;
    LPWSTR                  pwszCNGSignHashAlgids;
    LPWSTR                  pwszCNGPubKeyMinBitLengths; // Optional
} CERT_STRONG_SIGN_SERIALIZED_INFO, *PCERT_STRONG_SIGN_SERIALIZED_INFO;


#define CERT_STRONG_SIGN_ECDSA_ALGORITHM          L"ECDSA"

//
// Following CNG Signature Algorithms are supported
//
//  #define BCRYPT_RSA_ALGORITHM                    L"RSA"
//  #define BCRYPT_DSA_ALGORITHM                    L"DSA"
//  #define CERT_STRONG_SIGN_ECDSA_ALGORITHM        L"ECDSA"
//


//
// Following CNG Hash Algorithms are supported
//
//  #define BCRYPT_MD5_ALGORITHM                    L"MD5"
//  #define BCRYPT_SHA1_ALGORITHM                   L"SHA1"
//  #define BCRYPT_SHA256_ALGORITHM                 L"SHA256"
//  #define BCRYPT_SHA384_ALGORITHM                 L"SHA384"
//  #define BCRYPT_SHA512_ALGORITHM                 L"SHA512"
//

typedef struct _CERT_STRONG_SIGN_PARA {
    DWORD                   cbSize;

    DWORD                   dwInfoChoice;
    union  {
        void                                *pvInfo;

        // CERT_STRONG_SIGN_SERIALIZED_INFO_CHOICE
        PCERT_STRONG_SIGN_SERIALIZED_INFO   pSerializedInfo;

        // CERT_STRONG_SIGN_OID_INFO_CHOICE
        LPSTR                               pszOID;
        
    } DUMMYUNIONNAME;
} CERT_STRONG_SIGN_PARA, *PCERT_STRONG_SIGN_PARA;

typedef const CERT_STRONG_SIGN_PARA *PCCERT_STRONG_SIGN_PARA;


#define CERT_STRONG_SIGN_SERIALIZED_INFO_CHOICE     1
#define CERT_STRONG_SIGN_OID_INFO_CHOICE            2

// By default, strong signature checking isn't enabled for either
// CRLs or OCSP responses.
#define CERT_STRONG_SIGN_ENABLE_CRL_CHECK           0x1
#define CERT_STRONG_SIGN_ENABLE_OCSP_CHECK          0x2


//
// OID Strong Sign Parameters used by Windows OS Components
//

#define szOID_CERT_STRONG_SIGN_OS_PREFIX            "1.3.6.1.4.1.311.72.1."

// OS_1 was supported starting with Windows 8
//   Requires
//     RSA keys >= 2047 or ECDSA >= 256 (DSA not allowed)
//     SHA2 hashes (MD2, MD4, MD5 or SHA1 not allowed)
//   Both CERT_STRONG_SIGN_ENABLE_CRL_CHECK and
//        CERT_STRONG_SIGN_ENABLE_OCSP_CHECK are set
#define szOID_CERT_STRONG_SIGN_OS_1                 "1.3.6.1.4.1.311.72.1.1"
#define szOID_CERT_STRONG_SIGN_OS_CURRENT           szOID_CERT_STRONG_SIGN_OS_1


#define CERT_STRONG_SIGN_PARA_OS_1 \
{ \
    sizeof(CERT_STRONG_SIGN_PARA), \
    CERT_STRONG_SIGN_OID_INFO_CHOICE, \
    szOID_CERT_STRONG_SIGN_OS_1 \
}

#define CERT_STRONG_SIGN_PARA_OS_CURRENT \
{ \
    sizeof(CERT_STRONG_SIGN_PARA), \
    CERT_STRONG_SIGN_OID_INFO_CHOICE, \
    szOID_CERT_STRONG_SIGN_OS_CURRENT \
}

#define szOID_CERT_STRONG_KEY_OS_PREFIX             "1.3.6.1.4.1.311.72.2."

// OS_1 was supported starting with Windows 8
//   Requires
//     RSA keys >= 2047 or ECDSA >= 256 (DSA not allowed)
//     SHA1 or SHA2 hashes  (MD2, MD4 or MD5 not allowed)
//   Both CERT_STRONG_SIGN_ENABLE_CRL_CHECK and
//        CERT_STRONG_SIGN_ENABLE_OCSP_CHECK are set
#define szOID_CERT_STRONG_KEY_OS_1                  "1.3.6.1.4.1.311.72.2.1"
#define szOID_CERT_STRONG_KEY_OS_CURRENT            szOID_CERT_STRONG_KEY_OS_1


#define CERT_STRONG_KEY_PARA_OS_1 \
{ \
    sizeof(CERT_STRONG_SIGN_PARA), \
    CERT_STRONG_SIGN_OID_INFO_CHOICE, \
    szOID_CERT_STRONG_KEY_OS_1 \
}

#define CERT_STRONG_KEY_PARA_OS_CURRENT \
{ \
    sizeof(CERT_STRONG_SIGN_PARA), \
    CERT_STRONG_SIGN_OID_INFO_CHOICE, \
    szOID_CERT_STRONG_KEY_OS_CURRENT \
}

#define szOID_NO_HASH                   "1.3.6.1.4.1.311.73.1"


//+=========================================================================
//  Low Level Cryptographic Message Data Structures and APIs
//==========================================================================

typedef void *HCRYPTMSG;

#define szOID_PKCS_7_DATA               "1.2.840.113549.1.7.1"
#define szOID_PKCS_7_SIGNED             "1.2.840.113549.1.7.2"
#define szOID_PKCS_7_ENVELOPED          "1.2.840.113549.1.7.3"
#define szOID_PKCS_7_SIGNEDANDENVELOPED "1.2.840.113549.1.7.4"
#define szOID_PKCS_7_DIGESTED           "1.2.840.113549.1.7.5"
#define szOID_PKCS_7_ENCRYPTED          "1.2.840.113549.1.7.6"

#define szOID_PKCS_9_CONTENT_TYPE       "1.2.840.113549.1.9.3"
#define szOID_PKCS_9_MESSAGE_DIGEST     "1.2.840.113549.1.9.4"

//+-------------------------------------------------------------------------
//  Message types
//--------------------------------------------------------------------------
#define CMSG_DATA                    1
#define CMSG_SIGNED                  2
#define CMSG_ENVELOPED               3
#define CMSG_SIGNED_AND_ENVELOPED    4
#define CMSG_HASHED                  5
#define CMSG_ENCRYPTED               6

//+-------------------------------------------------------------------------
//  Message Type Bit Flags
//--------------------------------------------------------------------------
#define CMSG_ALL_FLAGS                   (~0UL)
#define CMSG_DATA_FLAG                   (1 << CMSG_DATA)
#define CMSG_SIGNED_FLAG                 (1 << CMSG_SIGNED)
#define CMSG_ENVELOPED_FLAG              (1 << CMSG_ENVELOPED)
#define CMSG_SIGNED_AND_ENVELOPED_FLAG   (1 << CMSG_SIGNED_AND_ENVELOPED)
#define CMSG_HASHED_FLAG                 (1 << CMSG_HASHED)
#define CMSG_ENCRYPTED_FLAG              (1 << CMSG_ENCRYPTED)


//+-------------------------------------------------------------------------
//  Certificate Issuer and SerialNumber
//--------------------------------------------------------------------------
typedef struct _CERT_ISSUER_SERIAL_NUMBER {
    CERT_NAME_BLOB      Issuer;
    CRYPT_INTEGER_BLOB  SerialNumber;
} CERT_ISSUER_SERIAL_NUMBER, *PCERT_ISSUER_SERIAL_NUMBER;

//+-------------------------------------------------------------------------
//  Certificate Identifier
//--------------------------------------------------------------------------
typedef struct _CERT_ID {
    DWORD   dwIdChoice;
    union {
        // CERT_ID_ISSUER_SERIAL_NUMBER
        CERT_ISSUER_SERIAL_NUMBER   IssuerSerialNumber;
        // CERT_ID_KEY_IDENTIFIER
        CRYPT_HASH_BLOB             KeyId;
        // CERT_ID_SHA1_HASH
        CRYPT_HASH_BLOB             HashId;
    } DUMMYUNIONNAME;
} CERT_ID, *PCERT_ID;

#define CERT_ID_ISSUER_SERIAL_NUMBER    1
#define CERT_ID_KEY_IDENTIFIER          2
#define CERT_ID_SHA1_HASH               3

//+-------------------------------------------------------------------------
//  The message encode information (pvMsgEncodeInfo) is message type dependent
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_DATA: pvMsgEncodeInfo = NULL
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_SIGNED
//
//  The pCertInfo in the CMSG_SIGNER_ENCODE_INFO provides the Issuer, SerialNumber
//  and PublicKeyInfo.Algorithm. The PublicKeyInfo.Algorithm implicitly
//  specifies the HashEncryptionAlgorithm to be used.
//
//  If the SignerId is present with a nonzero dwIdChoice its used instead
//  of the Issuer and SerialNumber in pCertInfo.
//
//  CMS supports the KEY_IDENTIFIER and ISSUER_SERIAL_NUMBER CERT_IDs. PKCS #7
//  version 1.5 only supports the ISSUER_SERIAL_NUMBER CERT_ID choice.
//
//  If HashEncryptionAlgorithm is present and not NULL its used instead of
//  the PublicKeyInfo.Algorithm.
//
//  Note, for RSA, the hash encryption algorithm is normally the same as
//  the public key algorithm. For DSA, the hash encryption algorithm is
//  normally a DSS signature algorithm.
//
//  pvHashEncryptionAuxInfo currently isn't used and must be set to NULL if
//  present in the data structure.
//
//  The hCryptProv and dwKeySpec specify the private key to use. If dwKeySpec
//  == 0, then, defaults to AT_SIGNATURE.
//
//  If the HashEncryptionAlgorithm is set to szOID_PKIX_NO_SIGNATURE, then,
//  the signature value only contains the hash octets. hCryptProv must still
//  be specified. However, since a private key isn't used the hCryptProv can be
//  acquired using CRYPT_VERIFYCONTEXT.
//
//  If CMSG_CRYPT_RELEASE_CONTEXT_FLAG is set in the dwFlags
//  passed to CryptMsgOpenToEncode(), the signer hCryptProv's are released.
//
//  For CNG, this applies to the hNCryptKey.
//
//  pvHashAuxInfo currently isn't used and must be set to NULL.
//
//  CMS signed messages allow the inclusion of Attribute Certs.
//--------------------------------------------------------------------------
typedef struct _CMSG_SIGNER_ENCODE_INFO {
    DWORD                       cbSize;
    PCERT_INFO                  pCertInfo;

    // NCryptIsKeyHandle() is called to determine the union choice.
    union {
        HCRYPTPROV                  hCryptProv;
        NCRYPT_KEY_HANDLE           hNCryptKey;
#ifdef CMSG_SIGNER_ENCODE_INFO_HAS_IUM_FIELDS
        BCRYPT_KEY_HANDLE           hBCryptKey;
#endif
    } DUMMYUNIONNAME;

    // not applicable for hNCryptKey choice
    DWORD                       dwKeySpec;

    CRYPT_ALGORITHM_IDENTIFIER  HashAlgorithm;
    void                        *pvHashAuxInfo;
    DWORD                       cAuthAttr;
    PCRYPT_ATTRIBUTE            rgAuthAttr;
    DWORD                       cUnauthAttr;
    PCRYPT_ATTRIBUTE            rgUnauthAttr;

#ifdef CMSG_SIGNER_ENCODE_INFO_HAS_CMS_FIELDS
    CERT_ID                     SignerId;

    // This is also referred to as the SignatureAlgorithm
    CRYPT_ALGORITHM_IDENTIFIER  HashEncryptionAlgorithm;
    void                        *pvHashEncryptionAuxInfo;
#endif
} CMSG_SIGNER_ENCODE_INFO, *PCMSG_SIGNER_ENCODE_INFO;

typedef struct _CMSG_SIGNED_ENCODE_INFO {
    DWORD                       cbSize;
    DWORD                       cSigners;
    PCMSG_SIGNER_ENCODE_INFO    rgSigners;
    DWORD                       cCertEncoded;
    PCERT_BLOB                  rgCertEncoded;
    DWORD                       cCrlEncoded;
    PCRL_BLOB                   rgCrlEncoded;

#ifdef CMSG_SIGNED_ENCODE_INFO_HAS_CMS_FIELDS
    DWORD                       cAttrCertEncoded;
    PCERT_BLOB                  rgAttrCertEncoded;
#endif
} CMSG_SIGNED_ENCODE_INFO, *PCMSG_SIGNED_ENCODE_INFO;

//+-------------------------------------------------------------------------
//  CMSG_ENVELOPED
//
//  The PCERT_INFO for the rgRecipients provides the Issuer, SerialNumber
//  and PublicKeyInfo. The PublicKeyInfo.Algorithm implicitly
//  specifies the KeyEncryptionAlgorithm to be used.
//
//  The PublicKeyInfo.PublicKey in PCERT_INFO is used to encrypt the content
//  encryption key for the recipient.
//
//  hCryptProv is used to do the content encryption, recipient key encryption
//  and export. The hCryptProv's private keys aren't used. If hCryptProv
//  is NULL, a default hCryptProv is chosen according to the
//  ContentEncryptionAlgorithm and the first recipient KeyEncryptionAlgorithm.
//
//  If CMSG_CRYPT_RELEASE_CONTEXT_FLAG is set in the dwFlags
//  passed to CryptMsgOpenToEncode(), the envelope's hCryptProv is released.
//
//  Note: CAPI currently doesn't support more than one KeyEncryptionAlgorithm
//  per provider. This will need to be fixed.
//
//  Currently, pvEncryptionAuxInfo is only defined for RC2 or RC4 encryption
//  algorithms. Otherwise, its not used and must be set to NULL.
//  See CMSG_RC2_AUX_INFO for the RC2 encryption algorithms.
//  See CMSG_RC4_AUX_INFO for the RC4 encryption algorithms.
//
//  To enable SP3 compatible encryption, pvEncryptionAuxInfo should point to
//  a CMSG_SP3_COMPATIBLE_AUX_INFO data structure.
//
//  To enable the CMS envelope enhancements, rgpRecipients must be set to
//  NULL, and rgCmsRecipients updated to point to an array of
//  CMSG_RECIPIENT_ENCODE_INFO's.
//
//  Also, CMS envelope enhancements support the inclusion of a bag of
//  Certs, CRLs, Attribute Certs and/or Unprotected Attributes.
//
//  AES ContentEncryption and ECC KeyAgreement recipients are only supported
//  via CNG. DH KeyAgreement or mail list recipients are only supported via
//  CAPI1. SP3 compatible encryption and RC4 are only supported via CAPI1.
//
//  For an RSA recipient identified via PCERT_INFO, for AES ContentEncryption,
//  szOID_RSAES_OAEP will be implicitly used for the KeyEncryptionAlgorithm.
//--------------------------------------------------------------------------
typedef struct _CMSG_RECIPIENT_ENCODE_INFO CMSG_RECIPIENT_ENCODE_INFO,
    *PCMSG_RECIPIENT_ENCODE_INFO;

typedef struct _CMSG_ENVELOPED_ENCODE_INFO {
    DWORD                       cbSize;
    HCRYPTPROV_LEGACY           hCryptProv;
    CRYPT_ALGORITHM_IDENTIFIER  ContentEncryptionAlgorithm;
    void                        *pvEncryptionAuxInfo;
    DWORD                       cRecipients;

    // The following array may only be used for transport recipients identified
    // by their IssuereAndSerialNumber. If rgpRecipients != NULL, then,
    // the rgCmsRecipients must be NULL.
    PCERT_INFO                  *rgpRecipients;

#ifdef CMSG_ENVELOPED_ENCODE_INFO_HAS_CMS_FIELDS
    // If rgCmsRecipients != NULL, then, the above rgpRecipients must be
    // NULL.
    PCMSG_RECIPIENT_ENCODE_INFO rgCmsRecipients;
    DWORD                       cCertEncoded;
    PCERT_BLOB                  rgCertEncoded;
    DWORD                       cCrlEncoded;
    PCRL_BLOB                   rgCrlEncoded;
    DWORD                       cAttrCertEncoded;
    PCERT_BLOB                  rgAttrCertEncoded;
    DWORD                       cUnprotectedAttr;
    PCRYPT_ATTRIBUTE            rgUnprotectedAttr;
#endif
} CMSG_ENVELOPED_ENCODE_INFO, *PCMSG_ENVELOPED_ENCODE_INFO;

//+-------------------------------------------------------------------------
//  Key Transport Recipient Encode Info
//
//  hCryptProv is used to do the recipient key encryption
//  and export. The hCryptProv's private keys aren't used.
//
//  If hCryptProv is NULL, then, the hCryptProv specified in
//  CMSG_ENVELOPED_ENCODE_INFO is used.
//
//  Note, even if CMSG_CRYPT_RELEASE_CONTEXT_FLAG is set in the dwFlags
//  passed to CryptMsgOpenToEncode(), this hCryptProv isn't released.
//
//  CMS supports the KEY_IDENTIFIER and ISSUER_SERIAL_NUMBER CERT_IDs. PKCS #7
//  version 1.5 only supports the ISSUER_SERIAL_NUMBER CERT_ID choice.
//
//  For RSA AES, KeyEncryptionAlgorithm.pszObjId should be set to
//  szOID_RSAES_OAEP. KeyEncryptionAlgorithm.Parameters should be set
//  to the encoded PKCS_RSAES_OAEP_PARAMETERS. If
//  KeyEncryptionAlgorithm.Parameters.cbData == 0, then, the default
//  parameters are used and encoded.
//--------------------------------------------------------------------------
typedef struct _CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO {
    DWORD                       cbSize;
    CRYPT_ALGORITHM_IDENTIFIER  KeyEncryptionAlgorithm;
    void                        *pvKeyEncryptionAuxInfo;
    HCRYPTPROV_LEGACY           hCryptProv;
    CRYPT_BIT_BLOB              RecipientPublicKey;
    CERT_ID                     RecipientId;
} CMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO, *PCMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO;


//+-------------------------------------------------------------------------
//  Key Agreement Recipient Encode Info
//
//  If hCryptProv is NULL, then, the hCryptProv specified in
//  CMSG_ENVELOPED_ENCODE_INFO is used.
//
//  For the CMSG_KEY_AGREE_STATIC_KEY_CHOICE, both the hCryptProv and
//  dwKeySpec must be specified to select the sender's private key.
//
//  Note, even if CMSG_CRYPT_RELEASE_CONTEXT_FLAG is set in the dwFlags
//  passed to CryptMsgOpenToEncode(), this hCryptProv isn't released.
//
//  CMS supports the KEY_IDENTIFIER and ISSUER_SERIAL_NUMBER CERT_IDs.
//
//  There is 1 key choice, ephemeral originator. The originator's ephemeral
//  key is generated using the public key algorithm parameters shared
//  amongst all the recipients.
//
//  There are 2 key choices: ephemeral originator or static sender. The
//  originator's ephemeral key is generated using the public key algorithm
//  parameters shared amongst all the recipients. For the static sender its
//  private key is used. The hCryptProv and dwKeySpec specify the private key.
//  The pSenderId identifies the certificate containing the sender's public key.
//
//  Currently, pvKeyEncryptionAuxInfo isn't used and must be set to NULL.
//
//  If KeyEncryptionAlgorithm.Parameters.cbData == 0, then, its Parameters
//  are updated with the encoded KeyWrapAlgorithm.
//
//  Currently, pvKeyWrapAuxInfo is only defined for algorithms with
//  RC2. Otherwise, its not used and must be set to NULL.
//  When set for RC2 algorithms, points to a CMSG_RC2_AUX_INFO containing
//  the RC2 effective key length.
//
//  Note, key agreement recipients are not supported in PKCS #7 version 1.5.
//
//  For the ECC szOID_DH_SINGLE_PASS_STDDH_SHA1_KDF KeyEncryptionAlgorithm
//  the CMSG_KEY_AGREE_EPHEMERAL_KEY_CHOICE must be specified.
//--------------------------------------------------------------------------
typedef struct _CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO {
    DWORD                       cbSize;
    CRYPT_BIT_BLOB              RecipientPublicKey;
    CERT_ID                     RecipientId;

    // Following fields are optional and only applicable to KEY_IDENTIFIER
    // CERT_IDs.
    FILETIME                    Date;
    PCRYPT_ATTRIBUTE_TYPE_VALUE pOtherAttr;
} CMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO,
    *PCMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO;

typedef struct _CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO {
    DWORD                       cbSize;
    CRYPT_ALGORITHM_IDENTIFIER  KeyEncryptionAlgorithm;
    void                        *pvKeyEncryptionAuxInfo;
    CRYPT_ALGORITHM_IDENTIFIER  KeyWrapAlgorithm;
    void                        *pvKeyWrapAuxInfo;

    // The following hCryptProv and dwKeySpec must be specified for the
    // CMSG_KEY_AGREE_STATIC_KEY_CHOICE.
    //
    // For CMSG_KEY_AGREE_EPHEMERAL_KEY_CHOICE, dwKeySpec isn't applicable
    // and hCryptProv is optional.

    HCRYPTPROV_LEGACY           hCryptProv;
    DWORD                       dwKeySpec;

    DWORD                       dwKeyChoice;
    union {
        // CMSG_KEY_AGREE_EPHEMERAL_KEY_CHOICE
        //
        // The ephemeral public key algorithm and parameters.
        PCRYPT_ALGORITHM_IDENTIFIER pEphemeralAlgorithm;

        // CMSG_KEY_AGREE_STATIC_KEY_CHOICE
        //
        // The CertId of the sender's certificate
        PCERT_ID                    pSenderId;
    } DUMMYUNIONNAME;
    CRYPT_DATA_BLOB             UserKeyingMaterial;     // OPTIONAL

    DWORD                                       cRecipientEncryptedKeys;
    PCMSG_RECIPIENT_ENCRYPTED_KEY_ENCODE_INFO   *rgpRecipientEncryptedKeys;
} CMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO, *PCMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO;

#define CMSG_KEY_AGREE_EPHEMERAL_KEY_CHOICE         1
#define CMSG_KEY_AGREE_STATIC_KEY_CHOICE            2

//+-------------------------------------------------------------------------
//  Mail List Recipient Encode Info
//
//  There is 1 choice for the KeyEncryptionKey: an already created CSP key
//  handle. For the key handle choice, hCryptProv must be nonzero. This key
//  handle isn't destroyed.
//
//  Note, even if CMSG_CRYPT_RELEASE_CONTEXT_FLAG is set in the dwFlags
//  passed to CryptMsgOpenToEncode(), this hCryptProv isn't released.
//
//  Currently, pvKeyEncryptionAuxInfo is only defined for RC2 key wrap
//  algorithms. Otherwise, its not used and must be set to NULL.
//  When set for RC2 algorithms, points to a CMSG_RC2_AUX_INFO containing
//  the RC2 effective key length.
//
//  Note, mail list recipients are not supported in PKCS #7 version 1.5.
//
//  Mail list recipients aren't supported using CNG.
//--------------------------------------------------------------------------
typedef struct _CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO {
    DWORD                       cbSize;
    CRYPT_ALGORITHM_IDENTIFIER  KeyEncryptionAlgorithm;
    void                        *pvKeyEncryptionAuxInfo;
    HCRYPTPROV                  hCryptProv;
    DWORD                       dwKeyChoice;
    union {
        // CMSG_MAIL_LIST_HANDLE_KEY_CHOICE
        HCRYPTKEY                   hKeyEncryptionKey;
        // Reserve space for a potential pointer choice
        void                        *pvKeyEncryptionKey;
    } DUMMYUNIONNAME;
    CRYPT_DATA_BLOB             KeyId;

    // Following fields are optional.
    FILETIME                    Date;
    PCRYPT_ATTRIBUTE_TYPE_VALUE pOtherAttr;
} CMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO, *PCMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO;

#define CMSG_MAIL_LIST_HANDLE_KEY_CHOICE    1

//+-------------------------------------------------------------------------
//  Recipient Encode Info
//
//  Note, only key transport recipients are supported in PKCS #7 version 1.5.
//--------------------------------------------------------------------------
struct _CMSG_RECIPIENT_ENCODE_INFO {
    DWORD   dwRecipientChoice;
    union {
        // CMSG_KEY_TRANS_RECIPIENT
        PCMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO   pKeyTrans;
        // CMSG_KEY_AGREE_RECIPIENT
        PCMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO   pKeyAgree;
        // CMSG_MAIL_LIST_RECIPIENT
        PCMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO   pMailList;
    } DUMMYUNIONNAME;
};


#define CMSG_KEY_TRANS_RECIPIENT         1
#define CMSG_KEY_AGREE_RECIPIENT         2
#define CMSG_MAIL_LIST_RECIPIENT         3


//+-------------------------------------------------------------------------
//  CMSG_RC2_AUX_INFO
//
//  AuxInfo for RC2 encryption algorithms. The pvEncryptionAuxInfo field
//  in CMSG_ENCRYPTED_ENCODE_INFO should be updated to point to this
//  structure. If not specified, defaults to 40 bit.
//
//  Note, this AuxInfo is only used when, the ContentEncryptionAlgorithm's
//  Parameter.cbData is zero. Otherwise, the Parameters is decoded to
//  get the bit length.
//
//  If CMSG_SP3_COMPATIBLE_ENCRYPT_FLAG is set in dwBitLen, then, SP3
//  compatible encryption is done and the bit length is ignored.
//--------------------------------------------------------------------------
typedef struct _CMSG_RC2_AUX_INFO {
    DWORD                       cbSize;
    DWORD                       dwBitLen;
} CMSG_RC2_AUX_INFO, *PCMSG_RC2_AUX_INFO;

//+-------------------------------------------------------------------------
//  CMSG_SP3_COMPATIBLE_AUX_INFO
//
//  AuxInfo for enabling SP3 compatible encryption.
//
//  The CMSG_SP3_COMPATIBLE_ENCRYPT_FLAG is set in dwFlags to enable SP3
//  compatible encryption. When set, uses zero salt instead of no salt,
//  the encryption algorithm parameters are NULL instead of containing the
//  encoded RC2 parameters or encoded IV octet string and the encrypted
//  symmetric key is encoded little endian instead of big endian.
//
//  SP3 compatible encryption isn't supported using CNG.
//--------------------------------------------------------------------------
typedef struct _CMSG_SP3_COMPATIBLE_AUX_INFO {
    DWORD                       cbSize;
    DWORD                       dwFlags;
} CMSG_SP3_COMPATIBLE_AUX_INFO, *PCMSG_SP3_COMPATIBLE_AUX_INFO;

#define CMSG_SP3_COMPATIBLE_ENCRYPT_FLAG    0x80000000

//+-------------------------------------------------------------------------
//  CMSG_RC4_AUX_INFO
//
//  AuxInfo for RC4 encryption algorithms. The pvEncryptionAuxInfo field
//  in CMSG_ENCRYPTED_ENCODE_INFO should be updated to point to this
//  structure. If not specified, uses the CSP's default bit length with no
//  salt. Note, the base CSP has a 40 bit default and the enhanced CSP has
//  a 128 bit default.
//
//  If CMSG_RC4_NO_SALT_FLAG is set in dwBitLen, then, no salt is generated.
//  Otherwise, (128 - dwBitLen)/8 bytes of salt are generated and encoded
//  as an OCTET STRING in the algorithm parameters field.
//
//  RC4 isn't supported using CNG.
//--------------------------------------------------------------------------
typedef struct _CMSG_RC4_AUX_INFO {
    DWORD                       cbSize;
    DWORD                       dwBitLen;
} CMSG_RC4_AUX_INFO, *PCMSG_RC4_AUX_INFO;

#define CMSG_RC4_NO_SALT_FLAG               0x40000000

//+-------------------------------------------------------------------------
//  CMSG_SIGNED_AND_ENVELOPED
//
//  For PKCS #7, a signed and enveloped message doesn't have the
//  signer's authenticated or unauthenticated attributes. Otherwise, a
//  combination of the CMSG_SIGNED_ENCODE_INFO and CMSG_ENVELOPED_ENCODE_INFO.
//--------------------------------------------------------------------------
typedef struct _CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO {
    DWORD                       cbSize;
    CMSG_SIGNED_ENCODE_INFO     SignedInfo;
    CMSG_ENVELOPED_ENCODE_INFO  EnvelopedInfo;
} CMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO, *PCMSG_SIGNED_AND_ENVELOPED_ENCODE_INFO;

//+-------------------------------------------------------------------------
//  CMSG_HASHED
//
//  hCryptProv is used to do the hash. Doesn't need to use a private key.
//
//  If CMSG_CRYPT_RELEASE_CONTEXT_FLAG is set in the dwFlags
//  passed to CryptMsgOpenToEncode(), the hCryptProv is released.
//
//  IN LH, the hCryptProv isn't used. However, its still released if the
//  above flag is set.
//
//  If fDetachedHash is set, then, the encoded message doesn't contain
//  any content (its treated as NULL Data)
//
//  pvHashAuxInfo currently isn't used and must be set to NULL.
//--------------------------------------------------------------------------
typedef struct _CMSG_HASHED_ENCODE_INFO {
    DWORD                       cbSize;
    HCRYPTPROV_LEGACY           hCryptProv;
    CRYPT_ALGORITHM_IDENTIFIER  HashAlgorithm;
    void                        *pvHashAuxInfo;
} CMSG_HASHED_ENCODE_INFO, *PCMSG_HASHED_ENCODE_INFO;

//+-------------------------------------------------------------------------
//  CMSG_ENCRYPTED
//
//  The key used to encrypt the message is identified outside of the message
//  content (for example, password).
//
//  The content input to CryptMsgUpdate has already been encrypted.
//
//  pvEncryptionAuxInfo currently isn't used and must be set to NULL.
//--------------------------------------------------------------------------
typedef struct _CMSG_ENCRYPTED_ENCODE_INFO {
    DWORD                       cbSize;
    CRYPT_ALGORITHM_IDENTIFIER  ContentEncryptionAlgorithm;
    void                        *pvEncryptionAuxInfo;
} CMSG_ENCRYPTED_ENCODE_INFO, *PCMSG_ENCRYPTED_ENCODE_INFO;

//+-------------------------------------------------------------------------
//  This parameter allows messages to be of variable length with streamed
//  output.
//
//  By default, messages are of a definite length and
//  CryptMsgGetParam(CMSG_CONTENT_PARAM) is
//  called to get the cryptographically processed content. Until closed,
//  the handle keeps a copy of the processed content.
//
//  With streamed output, the processed content can be freed as its streamed.
//
//  If the length of the content to be updated is known at the time of the
//  open, then, ContentLength should be set to that length. Otherwise, it
//  should be set to CMSG_INDEFINITE_LENGTH.
//--------------------------------------------------------------------------
typedef BOOL (WINAPI *PFN_CMSG_STREAM_OUTPUT)(
    _In_opt_ const void *pvArg,
    _In_reads_bytes_opt_(cbData) BYTE *pbData,
    _In_ DWORD cbData,
    _In_ BOOL fFinal
    );

#define CMSG_INDEFINITE_LENGTH       (0xFFFFFFFF)

typedef struct _CMSG_STREAM_INFO {
    DWORD                   cbContent;
    PFN_CMSG_STREAM_OUTPUT  pfnStreamOutput;
    void                    *pvArg;
} CMSG_STREAM_INFO, *PCMSG_STREAM_INFO;

//+-------------------------------------------------------------------------
//  Open dwFlags
//--------------------------------------------------------------------------
#define CMSG_BARE_CONTENT_FLAG              0x00000001
#define CMSG_LENGTH_ONLY_FLAG               0x00000002
#define CMSG_DETACHED_FLAG                  0x00000004
#define CMSG_AUTHENTICATED_ATTRIBUTES_FLAG  0x00000008
#define CMSG_CONTENTS_OCTETS_FLAG           0x00000010
#define CMSG_MAX_LENGTH_FLAG                0x00000020

// When set, nonData type inner content is encapsulated within an
// OCTET STRING. Applicable to both Signed and Enveloped messages.
#define CMSG_CMS_ENCAPSULATED_CONTENT_FLAG  0x00000040

// If set then the message will not have a signature in the final PKCS7
// of SignedData type. Instead the signature will contain plain text of
// the to-be-signed hash. It is used with digest signing.
#define CMSG_SIGNED_DATA_NO_SIGN_FLAG       0x00000080

// If set, then, the hCryptProv passed to CryptMsgOpenToEncode or
// CryptMsgOpenToDecode is released on the final CryptMsgClose.
// Not released if CryptMsgOpenToEncode or CryptMsgOpenToDecode fails.
//
// Also applies to hNCryptKey where applicable.
//
// Note, the envelope recipient hCryptProv's aren't released.
#define CMSG_CRYPT_RELEASE_CONTEXT_FLAG     0x00008000

//+-------------------------------------------------------------------------
//  Open a cryptographic message for encoding
//
//  If CMSG_BARE_CONTENT_FLAG is specified for a streamed message,
//  the streamed output will not have an outer ContentInfo wrapper. This
//  makes it suitable to be streamed into an enclosing message.
//
//  The pStreamInfo parameter needs to be set to stream the encoded message
//  output.
//--------------------------------------------------------------------------
WINCRYPT32API
HCRYPTMSG
WINAPI
CryptMsgOpenToEncode(
    _In_ DWORD dwMsgEncodingType,
    _In_ DWORD dwFlags,
    _In_ DWORD dwMsgType,
    _In_ void const *pvMsgEncodeInfo,
    _In_opt_ LPSTR pszInnerContentObjID,
    _In_opt_ PCMSG_STREAM_INFO pStreamInfo
    );

//+-------------------------------------------------------------------------
//  Calculate the length of an encoded cryptographic message.
//
//  Calculates the length of the encoded message given the
//  message type, encoding parameters and total length of
//  the data to be updated. Note, this might not be the exact length. However,
//  it will always be greater than or equal to the actual length.
//--------------------------------------------------------------------------
WINCRYPT32API
DWORD
WINAPI
CryptMsgCalculateEncodedLength(
    _In_ DWORD dwMsgEncodingType,
    _In_ DWORD dwFlags,
    _In_ DWORD dwMsgType,
    _In_ void const *pvMsgEncodeInfo,
    _In_opt_ LPSTR pszInnerContentObjID,
    _In_ DWORD cbData
    );

//+-------------------------------------------------------------------------
//  Open a cryptographic message for decoding
//
//  hCryptProv specifies the crypto provider to use for hashing and/or
//  decrypting the message. If hCryptProv is NULL, a default crypt provider
//  is used.
//
//  Currently pRecipientInfo isn't used and should be set to NULL.
//
//  The pStreamInfo parameter needs to be set to stream the decoded content
//  output.
//--------------------------------------------------------------------------
WINCRYPT32API
HCRYPTMSG
WINAPI
CryptMsgOpenToDecode(
    _In_ DWORD dwMsgEncodingType,
    _In_ DWORD dwFlags,
    _In_ DWORD dwMsgType,
    _In_opt_ HCRYPTPROV_LEGACY hCryptProv,
    _Reserved_ PCERT_INFO pRecipientInfo,
    _In_opt_ PCMSG_STREAM_INFO pStreamInfo
    );

//+-------------------------------------------------------------------------
//  Duplicate a cryptographic message handle
//--------------------------------------------------------------------------
WINCRYPT32API
HCRYPTMSG
WINAPI
CryptMsgDuplicate(
    _In_opt_ HCRYPTMSG hCryptMsg
    );

//+-------------------------------------------------------------------------
//  Close a cryptographic message handle
//
//  LastError is preserved unless FALSE is returned.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptMsgClose(
    _In_opt_ HCRYPTMSG hCryptMsg
    );

//+-------------------------------------------------------------------------
//  Update the content of a cryptographic message. Depending on how the
//  message was opened, the content is either encoded or decoded.
//
//  This function is repetitively called to append to the message content.
//  fFinal is set to identify the last update. On fFinal, the encode/decode
//  is completed. The encoded/decoded content and the decoded parameters
//  are valid until the open and all duplicated handles are closed.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptMsgUpdate(
    _In_ HCRYPTMSG hCryptMsg,
    _In_reads_bytes_opt_(cbData) const BYTE *pbData,
    _In_ DWORD cbData,
    _In_ BOOL fFinal
    );

//+-------------------------------------------------------------------------
//  Get a parameter after encoding/decoding a cryptographic message. Called
//  after the final CryptMsgUpdate. Only the CMSG_CONTENT_PARAM and
//  CMSG_COMPUTED_HASH_PARAM are valid for an encoded message.
//
//  For an encoded HASHED message, the CMSG_COMPUTED_HASH_PARAM can be got
//  before any CryptMsgUpdates to get its length.
//
//  The pvData type definition depends on the dwParamType value.
//
//  Elements pointed to by fields in the pvData structure follow the
//  structure. Therefore, *pcbData may exceed the size of the structure.
//
//  Upon input, if *pcbData == 0, then, *pcbData is updated with the length
//  of the data and the pvData parameter is ignored.
//
//  Upon return, *pcbData is updated with the length of the data.
//
//  The OBJID BLOBs returned in the pvData structures point to
//  their still encoded representation. The appropriate functions
//  must be called to decode the information.
//
//  See below for a list of the parameters to get.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptMsgGetParam(
    _In_ HCRYPTMSG hCryptMsg,
    _In_ DWORD dwParamType,
    _In_ DWORD dwIndex,
    _Out_writes_bytes_to_opt_(*pcbData, *pcbData) void *pvData,
    _Inout_ DWORD *pcbData
    );

//+-------------------------------------------------------------------------
//  Get parameter types and their corresponding data structure definitions.
//--------------------------------------------------------------------------
#define CMSG_TYPE_PARAM                              1
#define CMSG_CONTENT_PARAM                           2
#define CMSG_BARE_CONTENT_PARAM                      3
#define CMSG_INNER_CONTENT_TYPE_PARAM                4
#define CMSG_SIGNER_COUNT_PARAM                      5
#define CMSG_SIGNER_INFO_PARAM                       6
#define CMSG_SIGNER_CERT_INFO_PARAM                  7
#define CMSG_SIGNER_HASH_ALGORITHM_PARAM             8
#define CMSG_SIGNER_AUTH_ATTR_PARAM                  9
#define CMSG_SIGNER_UNAUTH_ATTR_PARAM                10
#define CMSG_CERT_COUNT_PARAM                        11
#define CMSG_CERT_PARAM                              12
#define CMSG_CRL_COUNT_PARAM                         13
#define CMSG_CRL_PARAM                               14
#define CMSG_ENVELOPE_ALGORITHM_PARAM                15
#define CMSG_RECIPIENT_COUNT_PARAM                   17
#define CMSG_RECIPIENT_INDEX_PARAM                   18
#define CMSG_RECIPIENT_INFO_PARAM                    19
#define CMSG_HASH_ALGORITHM_PARAM                    20
#define CMSG_HASH_DATA_PARAM                         21
#define CMSG_COMPUTED_HASH_PARAM                     22
#define CMSG_ENCRYPT_PARAM                           26
#define CMSG_ENCRYPTED_DIGEST                        27
#define CMSG_ENCODED_SIGNER                          28
#define CMSG_ENCODED_MESSAGE                         29
#define CMSG_VERSION_PARAM                           30
#define CMSG_ATTR_CERT_COUNT_PARAM                   31
#define CMSG_ATTR_CERT_PARAM                         32
#define CMSG_CMS_RECIPIENT_COUNT_PARAM               33
#define CMSG_CMS_RECIPIENT_INDEX_PARAM               34
#define CMSG_CMS_RECIPIENT_ENCRYPTED_KEY_INDEX_PARAM 35
#define CMSG_CMS_RECIPIENT_INFO_PARAM                36
#define CMSG_UNPROTECTED_ATTR_PARAM                  37
#define CMSG_SIGNER_CERT_ID_PARAM                    38
#define CMSG_CMS_SIGNER_INFO_PARAM                   39

//+-------------------------------------------------------------------------
//  CMSG_TYPE_PARAM
//
//  The type of the decoded message.
//
//  pvData points to a DWORD
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CONTENT_PARAM
//
//  The encoded content of a cryptographic message. Depending on how the
//  message was opened, the content is either the whole PKCS#7
//  message (opened to encode) or the inner content (opened to decode).
//  In the decode case, the decrypted content is returned, if enveloped.
//  If not enveloped, and if the inner content is of type DATA, the returned
//  data is the contents octets of the inner content.
//
//  pvData points to the buffer receiving the content bytes
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_BARE_CONTENT_PARAM
//
//  The encoded content of an encoded cryptographic message, without the
//  outer layer of ContentInfo. That is, only the encoding of the
//  ContentInfo.content field is returned.
//
//  pvData points to the buffer receiving the content bytes
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_INNER_CONTENT_TYPE_PARAM
//
//  The type of the inner content of a decoded cryptographic message,
//  in the form of a NULL-terminated object identifier string
//  (eg. "1.2.840.113549.1.7.1").
//
//  pvData points to the buffer receiving the object identifier string
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_SIGNER_COUNT_PARAM
//
//  Count of signers in a SIGNED or SIGNED_AND_ENVELOPED message
//
//  pvData points to a DWORD
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_SIGNER_CERT_INFO_PARAM
//
//  To get all the signers, repetitively call CryptMsgGetParam, with
//  dwIndex set to 0 .. SignerCount - 1.
//
//  pvData points to a CERT_INFO struct.
//
//  Only the following fields have been updated in the CERT_INFO struct:
//  Issuer and SerialNumber.
//
//  Note, if the KEYID choice was selected for a CMS SignerId, then, the
//  SerialNumber is 0 and the Issuer is encoded containing a single RDN with a
//  single Attribute whose OID is szOID_KEYID_RDN, value type is
//  CERT_RDN_OCTET_STRING and value is the KEYID. When the
//  CertGetSubjectCertificateFromStore and
//  CertFindCertificateInStore(CERT_FIND_SUBJECT_CERT) APIs see this
//  special KEYID Issuer and SerialNumber, they do a KEYID match.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_SIGNER_INFO_PARAM
//
//  To get all the signers, repetitively call CryptMsgGetParam, with
//  dwIndex set to 0 .. SignerCount - 1.
//
//  pvData points to a CMSG_SIGNER_INFO struct.
//
//  Note, if the KEYID choice was selected for a CMS SignerId, then, the
//  SerialNumber is 0 and the Issuer is encoded containing a single RDN with a
//  single Attribute whose OID is szOID_KEYID_RDN, value type is
//  CERT_RDN_OCTET_STRING and value is the KEYID. When the
//  CertGetSubjectCertificateFromStore and
//  CertFindCertificateInStore(CERT_FIND_SUBJECT_CERT) APIs see this
//  special KEYID Issuer and SerialNumber, they do a KEYID match.
//--------------------------------------------------------------------------
typedef struct _CMSG_SIGNER_INFO {
    DWORD                       dwVersion;
    CERT_NAME_BLOB              Issuer;
    CRYPT_INTEGER_BLOB          SerialNumber;
    CRYPT_ALGORITHM_IDENTIFIER  HashAlgorithm;

    // This is also referred to as the SignatureAlgorithm
    CRYPT_ALGORITHM_IDENTIFIER  HashEncryptionAlgorithm;

    CRYPT_DATA_BLOB             EncryptedHash;
    CRYPT_ATTRIBUTES            AuthAttrs;
    CRYPT_ATTRIBUTES            UnauthAttrs;
} CMSG_SIGNER_INFO, *PCMSG_SIGNER_INFO;


//+-------------------------------------------------------------------------
//  CMSG_SIGNER_CERT_ID_PARAM
//
//  To get all the signers, repetitively call CryptMsgGetParam, with
//  dwIndex set to 0 .. SignerCount - 1.
//
//  pvData points to a CERT_ID struct.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CMS_SIGNER_INFO_PARAM
//
//  Same as CMSG_SIGNER_INFO_PARAM, except, contains SignerId instead of
//  Issuer and SerialNumber.
//
//  To get all the signers, repetitively call CryptMsgGetParam, with
//  dwIndex set to 0 .. SignerCount - 1.
//
//  pvData points to a CMSG_CMS_SIGNER_INFO struct.
//--------------------------------------------------------------------------
typedef struct _CMSG_CMS_SIGNER_INFO {
    DWORD                       dwVersion;
    CERT_ID                     SignerId;
    CRYPT_ALGORITHM_IDENTIFIER  HashAlgorithm;

    // This is also referred to as the SignatureAlgorithm
    CRYPT_ALGORITHM_IDENTIFIER  HashEncryptionAlgorithm;

    CRYPT_DATA_BLOB             EncryptedHash;
    CRYPT_ATTRIBUTES            AuthAttrs;
    CRYPT_ATTRIBUTES            UnauthAttrs;
} CMSG_CMS_SIGNER_INFO, *PCMSG_CMS_SIGNER_INFO;


//+-------------------------------------------------------------------------
//  CMSG_SIGNER_HASH_ALGORITHM_PARAM
//
//  This parameter specifies the HashAlgorithm that was used for the signer.
//
//  Set dwIndex to iterate through all the signers.
//
//  pvData points to an CRYPT_ALGORITHM_IDENTIFIER struct.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_SIGNER_AUTH_ATTR_PARAM
//
//  The authenticated attributes for the signer.
//
//  Set dwIndex to iterate through all the signers.
//
//  pvData points to a CMSG_ATTR struct.
//--------------------------------------------------------------------------
typedef CRYPT_ATTRIBUTES CMSG_ATTR;
typedef CRYPT_ATTRIBUTES *PCMSG_ATTR;

//+-------------------------------------------------------------------------
//  CMSG_SIGNER_UNAUTH_ATTR_PARAM
//
//  The unauthenticated attributes for the signer.
//
//  Set dwIndex to iterate through all the signers.
//
//  pvData points to a CMSG_ATTR struct.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CERT_COUNT_PARAM
//
//  Count of certificates in a SIGNED or SIGNED_AND_ENVELOPED message.
//
//  CMS, also supports certificates in an ENVELOPED message.
//
//  pvData points to a DWORD
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CERT_PARAM
//
//  To get all the certificates, repetitively call CryptMsgGetParam, with
//  dwIndex set to 0 .. CertCount - 1.
//
//  pvData points to an array of the certificate's encoded bytes.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CRL_COUNT_PARAM
//
//  Count of CRLs in a SIGNED or SIGNED_AND_ENVELOPED message.
//
//  CMS, also supports CRLs in an ENVELOPED message.
//
//  pvData points to a DWORD
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CRL_PARAM
//
//  To get all the CRLs, repetitively call CryptMsgGetParam, with
//  dwIndex set to 0 .. CrlCount - 1.
//
//  pvData points to an array of the CRL's encoded bytes.
//--------------------------------------------------------------------------


//+-------------------------------------------------------------------------
//  CMSG_ENVELOPE_ALGORITHM_PARAM
//
//  The ContentEncryptionAlgorithm that was used in
//  an ENVELOPED or SIGNED_AND_ENVELOPED message.
//
//  For streaming you must be able to successfully get this parameter before
//  doing a CryptMsgControl decrypt.
//
//  pvData points to an CRYPT_ALGORITHM_IDENTIFIER struct.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_RECIPIENT_COUNT_PARAM
//
//  Count of recipients in an ENVELOPED or SIGNED_AND_ENVELOPED message.
//
//  Count of key transport recepients.
//
//  The CMSG_CMS_RECIPIENT_COUNT_PARAM has the total count of
//  recipients (it also includes key agree and mail list recipients).
//
//  pvData points to a DWORD
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_RECIPIENT_INDEX_PARAM
//
//  Index of the recipient used to decrypt an ENVELOPED or SIGNED_AND_ENVELOPED
//  message.
//
//  Index of a key transport recipient. If a non key transport
//  recipient was used to decrypt, fails with LastError set to
//  CRYPT_E_INVALID_INDEX.
//
//  pvData points to a DWORD
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_RECIPIENT_INFO_PARAM
//
//  To get all the recipients, repetitively call CryptMsgGetParam, with
//  dwIndex set to 0 .. RecipientCount - 1.
//
//  Only returns the key transport recepients.
//
//  The CMSG_CMS_RECIPIENT_INFO_PARAM returns all recipients.
//
//  pvData points to a CERT_INFO struct.
//
//  Only the following fields have been updated in the CERT_INFO struct:
//  Issuer, SerialNumber and PublicKeyAlgorithm. The PublicKeyAlgorithm
//  specifies the KeyEncryptionAlgorithm that was used.
//
//  Note, if the KEYID choice was selected for a key transport recipient, then,
//  the SerialNumber is 0 and the Issuer is encoded containing a single RDN
//  with a single Attribute whose OID is szOID_KEYID_RDN, value type is
//  CERT_RDN_OCTET_STRING and value is the KEYID. When the
//  CertGetSubjectCertificateFromStore and
//  CertFindCertificateInStore(CERT_FIND_SUBJECT_CERT) APIs see this
//  special KEYID Issuer and SerialNumber, they do a KEYID match.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_HASH_ALGORITHM_PARAM
//
//  The HashAlgorithm in a HASHED message.
//
//  pvData points to an CRYPT_ALGORITHM_IDENTIFIER struct.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_HASH_DATA_PARAM
//
//  The hash in a HASHED message.
//
//  pvData points to an array of bytes.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_COMPUTED_HASH_PARAM
//
//  The computed hash for a HASHED message.
//  This may be called for either an encoded or decoded message.
//
//  Also, the computed hash for one of the signer's in a SIGNED message.
//  It may be called for either an encoded or decoded message after the
//  final update.  Set dwIndex to iterate through all the signers.
//
//  pvData points to an array of bytes.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_ENCRYPT_PARAM
//
//  The ContentEncryptionAlgorithm that was used in an ENCRYPTED message.
//
//  pvData points to an CRYPT_ALGORITHM_IDENTIFIER struct.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_ENCODED_MESSAGE
//
//  The full encoded message. This is useful in the case of a decoded
//  message which has been modified (eg. a signed-data or
//  signed-and-enveloped-data message which has been countersigned).
//
//  pvData points to an array of the message's encoded bytes.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_VERSION_PARAM
//
//  The version of the decoded message.
//
//  pvData points to a DWORD
//--------------------------------------------------------------------------

#define CMSG_SIGNED_DATA_V1                     1
#define CMSG_SIGNED_DATA_V3                     3
#define CMSG_SIGNED_DATA_PKCS_1_5_VERSION       CMSG_SIGNED_DATA_V1
#define CMSG_SIGNED_DATA_CMS_VERSION            CMSG_SIGNED_DATA_V3

#define CMSG_SIGNER_INFO_V1                     1
#define CMSG_SIGNER_INFO_V3                     3
#define CMSG_SIGNER_INFO_PKCS_1_5_VERSION       CMSG_SIGNER_INFO_V1
#define CMSG_SIGNER_INFO_CMS_VERSION            CMSG_SIGNER_INFO_V3

#define CMSG_HASHED_DATA_V0                     0
#define CMSG_HASHED_DATA_V2                     2
#define CMSG_HASHED_DATA_PKCS_1_5_VERSION       CMSG_HASHED_DATA_V0
#define CMSG_HASHED_DATA_CMS_VERSION            CMSG_HASHED_DATA_V2

#define CMSG_ENVELOPED_DATA_V0                  0
#define CMSG_ENVELOPED_DATA_V2                  2
#define CMSG_ENVELOPED_DATA_PKCS_1_5_VERSION    CMSG_ENVELOPED_DATA_V0
#define CMSG_ENVELOPED_DATA_CMS_VERSION         CMSG_ENVELOPED_DATA_V2

//+-------------------------------------------------------------------------
//  CMSG_ATTR_CERT_COUNT_PARAM
//
//  Count of attribute certificates in a SIGNED or ENVELOPED message.
//
//  pvData points to a DWORD
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_ATTR_CERT_PARAM
//
//  To get all the attribute certificates, repetitively call CryptMsgGetParam,
//  with dwIndex set to 0 .. AttrCertCount - 1.
//
//  pvData points to an array of the attribute certificate's encoded bytes.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CMS_RECIPIENT_COUNT_PARAM
//
//  Count of all CMS recipients in an ENVELOPED message.
//
//  pvData points to a DWORD
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CMS_RECIPIENT_INDEX_PARAM
//
//  Index of the CMS recipient used to decrypt an ENVELOPED message.
//
//  pvData points to a DWORD
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CMS_RECIPIENT_ENCRYPTED_KEY_INDEX_PARAM
//
//  For a CMS key agreement recipient, the index of the encrypted key
//  used to decrypt an ENVELOPED message.
//
//  pvData points to a DWORD
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CMS_RECIPIENT_INFO_PARAM
//
//  To get all the CMS recipients, repetitively call CryptMsgGetParam, with
//  dwIndex set to 0 .. CmsRecipientCount - 1.
//
//  pvData points to a CMSG_CMS_RECIPIENT_INFO struct.
//--------------------------------------------------------------------------

typedef struct _CMSG_KEY_TRANS_RECIPIENT_INFO {
    DWORD                       dwVersion;

    // Currently, only ISSUER_SERIAL_NUMBER or KEYID choices
    CERT_ID                     RecipientId;

    CRYPT_ALGORITHM_IDENTIFIER  KeyEncryptionAlgorithm;
    CRYPT_DATA_BLOB             EncryptedKey;
} CMSG_KEY_TRANS_RECIPIENT_INFO, *PCMSG_KEY_TRANS_RECIPIENT_INFO;

typedef struct _CMSG_RECIPIENT_ENCRYPTED_KEY_INFO {
    // Currently, only ISSUER_SERIAL_NUMBER or KEYID choices
    CERT_ID                     RecipientId;

    CRYPT_DATA_BLOB             EncryptedKey;

    // The following optional fields are only applicable to KEYID choice
    FILETIME                    Date;
    PCRYPT_ATTRIBUTE_TYPE_VALUE pOtherAttr;
} CMSG_RECIPIENT_ENCRYPTED_KEY_INFO, *PCMSG_RECIPIENT_ENCRYPTED_KEY_INFO;

typedef struct _CMSG_KEY_AGREE_RECIPIENT_INFO {
    DWORD                       dwVersion;
    DWORD                       dwOriginatorChoice;
    union {
        // CMSG_KEY_AGREE_ORIGINATOR_CERT
        CERT_ID                     OriginatorCertId;
        // CMSG_KEY_AGREE_ORIGINATOR_PUBLIC_KEY
        CERT_PUBLIC_KEY_INFO        OriginatorPublicKeyInfo;
    } DUMMYUNIONNAME;
    CRYPT_DATA_BLOB             UserKeyingMaterial;
    CRYPT_ALGORITHM_IDENTIFIER  KeyEncryptionAlgorithm;

    DWORD                                cRecipientEncryptedKeys;
    PCMSG_RECIPIENT_ENCRYPTED_KEY_INFO   *rgpRecipientEncryptedKeys;
} CMSG_KEY_AGREE_RECIPIENT_INFO, *PCMSG_KEY_AGREE_RECIPIENT_INFO;

#define CMSG_KEY_AGREE_ORIGINATOR_CERT         1
#define CMSG_KEY_AGREE_ORIGINATOR_PUBLIC_KEY   2


typedef struct _CMSG_MAIL_LIST_RECIPIENT_INFO {
    DWORD                       dwVersion;
    CRYPT_DATA_BLOB             KeyId;
    CRYPT_ALGORITHM_IDENTIFIER  KeyEncryptionAlgorithm;
    CRYPT_DATA_BLOB             EncryptedKey;

    // The following fields are optional
    FILETIME                    Date;
    PCRYPT_ATTRIBUTE_TYPE_VALUE pOtherAttr;
} CMSG_MAIL_LIST_RECIPIENT_INFO, *PCMSG_MAIL_LIST_RECIPIENT_INFO;

typedef struct _CMSG_CMS_RECIPIENT_INFO {
    DWORD   dwRecipientChoice;
    union {
        // CMSG_KEY_TRANS_RECIPIENT
        PCMSG_KEY_TRANS_RECIPIENT_INFO   pKeyTrans;
        // CMSG_KEY_AGREE_RECIPIENT
        PCMSG_KEY_AGREE_RECIPIENT_INFO   pKeyAgree;
        // CMSG_MAIL_LIST_RECIPIENT
        PCMSG_MAIL_LIST_RECIPIENT_INFO   pMailList;
    } DUMMYUNIONNAME;
} CMSG_CMS_RECIPIENT_INFO, *PCMSG_CMS_RECIPIENT_INFO;


// dwVersion numbers for the KeyTrans, KeyAgree and MailList recipients
#define CMSG_ENVELOPED_RECIPIENT_V0             0
#define CMSG_ENVELOPED_RECIPIENT_V2             2
#define CMSG_ENVELOPED_RECIPIENT_V3             3
#define CMSG_ENVELOPED_RECIPIENT_V4             4
#define CMSG_KEY_TRANS_PKCS_1_5_VERSION         CMSG_ENVELOPED_RECIPIENT_V0
#define CMSG_KEY_TRANS_CMS_VERSION              CMSG_ENVELOPED_RECIPIENT_V2
#define CMSG_KEY_AGREE_VERSION                  CMSG_ENVELOPED_RECIPIENT_V3
#define CMSG_MAIL_LIST_VERSION                  CMSG_ENVELOPED_RECIPIENT_V4

//+-------------------------------------------------------------------------
//  CMSG_UNPROTECTED_ATTR_PARAM
//
//  The unprotected attributes in the envelped message.
//
//  pvData points to a CMSG_ATTR struct.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  Perform a special "control" function after the final CryptMsgUpdate of a
//  encoded/decoded cryptographic message.
//
//  The dwCtrlType parameter specifies the type of operation to be performed.
//
//  The pvCtrlPara definition depends on the dwCtrlType value.
//
//  See below for a list of the control operations and their pvCtrlPara
//  type definition.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptMsgControl(
    _In_ HCRYPTMSG hCryptMsg,
    _In_ DWORD dwFlags,
    _In_ DWORD dwCtrlType,
    _In_opt_ void const *pvCtrlPara
    );

//+-------------------------------------------------------------------------
//  Message control types
//--------------------------------------------------------------------------
#define CMSG_CTRL_VERIFY_SIGNATURE       1
#define CMSG_CTRL_DECRYPT                2
#define CMSG_CTRL_VERIFY_HASH            5
#define CMSG_CTRL_ADD_SIGNER             6
#define CMSG_CTRL_DEL_SIGNER             7
#define CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR 8
#define CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR 9
#define CMSG_CTRL_ADD_CERT               10
#define CMSG_CTRL_DEL_CERT               11
#define CMSG_CTRL_ADD_CRL                12
#define CMSG_CTRL_DEL_CRL                13
#define CMSG_CTRL_ADD_ATTR_CERT          14
#define CMSG_CTRL_DEL_ATTR_CERT          15
#define CMSG_CTRL_KEY_TRANS_DECRYPT      16
#define CMSG_CTRL_KEY_AGREE_DECRYPT      17
#define CMSG_CTRL_MAIL_LIST_DECRYPT      18
#define CMSG_CTRL_VERIFY_SIGNATURE_EX    19
#define CMSG_CTRL_ADD_CMS_SIGNER_INFO    20
#define CMSG_CTRL_ENABLE_STRONG_SIGNATURE 21

//+-------------------------------------------------------------------------
//  CMSG_CTRL_VERIFY_SIGNATURE
//
//  Verify the signature of a SIGNED or SIGNED_AND_ENVELOPED
//  message after it has been decoded.
//
//  For a SIGNED_AND_ENVELOPED message, called after
//  CryptMsgControl(CMSG_CTRL_DECRYPT), if CryptMsgOpenToDecode was called
//  with a NULL pRecipientInfo.
//
//  pvCtrlPara points to a CERT_INFO struct.
//
//  The CERT_INFO contains the Issuer and SerialNumber identifying
//  the Signer of the message. The CERT_INFO also contains the
//  PublicKeyInfo
//  used to verify the signature. The cryptographic provider specified
//  in CryptMsgOpenToDecode is used.
//
//  Note, if the message contains CMS signers identified by KEYID, then,
//  the CERT_INFO's Issuer and SerialNumber is ignored and only the public
//  key is used to find a signer whose signature verifies.
//
//  The following CMSG_CTRL_VERIFY_SIGNATURE_EX should be used instead.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CTRL_VERIFY_SIGNATURE_EX
//
//  Verify the signature of a SIGNED message after it has been decoded.
//
//  pvCtrlPara points to the following CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA.
//
//  If hCryptProv is NULL, uses the cryptographic provider specified in
//  CryptMsgOpenToDecode. If CryptMsgOpenToDecode's hCryptProv is also NULL,
//  gets default provider according to the signer's public key OID.
//
//  dwSignerIndex is the index of the signer to use to verify the signature.
//
//  The signer can be a pointer to a CERT_PUBLIC_KEY_INFO, certificate
//  context or a chain context.
//
//  If the signer's HashEncryptionAlgorithm is szOID_PKIX_NO_SIGNATURE, then,
//  the signature is expected to contain the hash octets. Only dwSignerType
//  of CMSG_VERIFY_SIGNER_NULL may be specified to verify this no signature
//  case.
//--------------------------------------------------------------------------
typedef struct _CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA {
    DWORD               cbSize;
    HCRYPTPROV_LEGACY   hCryptProv;
    DWORD               dwSignerIndex;
    DWORD               dwSignerType;
    void                *pvSigner;
} CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA, *PCMSG_CTRL_VERIFY_SIGNATURE_EX_PARA;

// Signer Types
#define CMSG_VERIFY_SIGNER_PUBKEY                   1
    // pvSigner :: PCERT_PUBLIC_KEY_INFO
#define CMSG_VERIFY_SIGNER_CERT                     2
    // pvSigner :: PCCERT_CONTEXT
#define CMSG_VERIFY_SIGNER_CHAIN                    3
    // pvSigner :: PCCERT_CHAIN_CONTEXT
#define CMSG_VERIFY_SIGNER_NULL                     4
    // pvSigner :: NULL


//+-------------------------------------------------------------------------
//  CMSG_CTRL_ENABLE_STRONG_SIGNATURE
//
//  Enables Strong Signature Checking for subsequent verifies.
//
//  pvCtrlPara points to a const CERT_STRONG_SIGN_PARA struct.
//--------------------------------------------------------------------------


//+-------------------------------------------------------------------------
//  CMSG_CTRL_DECRYPT
//
//  Decrypt an ENVELOPED or SIGNED_AND_ENVELOPED message after it has been
//  decoded.
//
//  This decrypt is only applicable to key transport recipients.
//
//  hCryptProv and dwKeySpec specify the private key to use. For dwKeySpec ==
//  0, defaults to AT_KEYEXCHANGE.
//
//  hNCryptKey can be set to decrypt using a CNG private key.
//
//  If CMSG_CRYPT_RELEASE_CONTEXT_FLAG is set in the dwFlags passed
//  to CryptMsgControl, then, the hCryptProv is released on the final
//  CryptMsgClose. Not released if CryptMsgControl fails. Also applies
//  to freeing the hNCryptKey.
//
//  dwRecipientIndex is the index of the recipient in the message associated
//  with the hCryptProv's or hNCryptKey's private key.
//
//  The dwRecipientIndex is the index of a key transport recipient.
//
//  Note, the message can only be decrypted once.
//--------------------------------------------------------------------------
typedef struct _CMSG_CTRL_DECRYPT_PARA {
    DWORD       cbSize;

    // NCryptIsKeyHandle() is called to determine the union choice.
    union {
        HCRYPTPROV                  hCryptProv;
        NCRYPT_KEY_HANDLE           hNCryptKey;
    } DUMMYUNIONNAME;

    // not applicable for hNCryptKey choice
    DWORD       dwKeySpec;

    DWORD       dwRecipientIndex;
} CMSG_CTRL_DECRYPT_PARA, *PCMSG_CTRL_DECRYPT_PARA;


//+-------------------------------------------------------------------------
//  CMSG_CTRL_KEY_TRANS_DECRYPT
//
//  Decrypt an ENVELOPED message after it has been decoded for a key
//  transport recipient.
//
//  hCryptProv and dwKeySpec specify the private key to use. For dwKeySpec ==
//  0, defaults to AT_KEYEXCHANGE.
//
//  hNCryptKey can be set to decrypt using a CNG private key.
//
//  If CMSG_CRYPT_RELEASE_CONTEXT_FLAG is set in the dwFlags passed
//  to CryptMsgControl, then, the hCryptProv is released on the final
//  CryptMsgClose. Not released if CryptMsgControl fails. Also applies
//  to freeing the hNCryptKey.
//
//  pKeyTrans points to the CMSG_KEY_TRANS_RECIPIENT_INFO obtained via
//  CryptMsgGetParam(CMSG_CMS_RECIPIENT_INFO_PARAM)
//
//  dwRecipientIndex is the index of the recipient in the message associated
//  with the hCryptProv's or hNCryptKey's private key.
//
//  Note, the message can only be decrypted once.
//--------------------------------------------------------------------------
typedef struct _CMSG_CTRL_KEY_TRANS_DECRYPT_PARA {
    DWORD                           cbSize;
    // NCryptIsKeyHandle() is called to determine the union choice.
    union {
        HCRYPTPROV                  hCryptProv;
        NCRYPT_KEY_HANDLE           hNCryptKey;
    } DUMMYUNIONNAME;

    // not applicable for hNCryptKey choice
    DWORD                           dwKeySpec;

    PCMSG_KEY_TRANS_RECIPIENT_INFO  pKeyTrans;
    DWORD                           dwRecipientIndex;
} CMSG_CTRL_KEY_TRANS_DECRYPT_PARA, *PCMSG_CTRL_KEY_TRANS_DECRYPT_PARA;

//+-------------------------------------------------------------------------
//  CMSG_CTRL_KEY_AGREE_DECRYPT
//
//  Decrypt an ENVELOPED message after it has been decoded for a key
//  agreement recipient.
//
//  hCryptProv and dwKeySpec specify the private key to use. For dwKeySpec ==
//  0, defaults to AT_KEYEXCHANGE.
//
//  hNCryptKey can be set to decrypt using a CNG private key.
//
//  If CMSG_CRYPT_RELEASE_CONTEXT_FLAG is set in the dwFlags passed
//  to CryptMsgControl, then, the hCryptProv is released on the final
//  CryptMsgClose. Not released if CryptMsgControl fails. Also applies
//  to freeing the hNCryptKey.
//
//  pKeyAgree points to the CMSG_KEY_AGREE_RECIPIENT_INFO obtained via
//  CryptMsgGetParam(CMSG_CMS_RECIPIENT_INFO_PARAM) for dwRecipientIndex.
//
//  dwRecipientIndex, dwRecipientEncryptedKeyIndex are the indices of the
//  recipient's encrypted key in the message associated with the hCryptProv's
//  or hNCryptKey's private key.
//
//  OriginatorPublicKey is the originator's public key obtained from either
//  the originator's certificate or the CMSG_KEY_AGREE_RECIPIENT_INFO obtained
//  via the CMSG_CMS_RECIPIENT_INFO_PARAM.
//
//  Note, the message can only be decrypted once.
//--------------------------------------------------------------------------
typedef struct _CMSG_CTRL_KEY_AGREE_DECRYPT_PARA {
    DWORD                           cbSize;

    // NCryptIsKeyHandle() is called to determine the union choice.
    union {
        HCRYPTPROV                  hCryptProv;
        NCRYPT_KEY_HANDLE           hNCryptKey;
    } DUMMYUNIONNAME;

    // not applicable for hNCryptKey choice
    DWORD                           dwKeySpec;

    PCMSG_KEY_AGREE_RECIPIENT_INFO  pKeyAgree;
    DWORD                           dwRecipientIndex;
    DWORD                           dwRecipientEncryptedKeyIndex;
    CRYPT_BIT_BLOB                  OriginatorPublicKey;
} CMSG_CTRL_KEY_AGREE_DECRYPT_PARA, *PCMSG_CTRL_KEY_AGREE_DECRYPT_PARA;


//+-------------------------------------------------------------------------
//  CMSG_CTRL_MAIL_LIST_DECRYPT
//
//  Decrypt an ENVELOPED message after it has been decoded for a mail
//  list recipient.
//
//  pMailList points to the CMSG_MAIL_LIST_RECIPIENT_INFO obtained via
//  CryptMsgGetParam(CMSG_CMS_RECIPIENT_INFO_PARAM) for dwRecipientIndex.
//
//  There is 1 choice for the KeyEncryptionKey: an already created CSP key
//  handle. For the key handle choice, hCryptProv must be nonzero. This key
//  handle isn't destroyed.
//
//  If CMSG_CRYPT_RELEASE_CONTEXT_FLAG is set in the dwFlags passed
//  to CryptMsgControl, then, the hCryptProv is released on the final
//  CryptMsgClose. Not released if CryptMsgControl fails.
//
//  For RC2 wrap, the effective key length is obtained from the
//  KeyEncryptionAlgorithm parameters and set on the hKeyEncryptionKey before
//  decrypting.
//
//  Note, the message can only be decrypted once.
//
//  Mail list recipients aren't supported using CNG.
//--------------------------------------------------------------------------
typedef struct _CMSG_CTRL_MAIL_LIST_DECRYPT_PARA {
    DWORD                           cbSize;
    HCRYPTPROV                      hCryptProv;
    PCMSG_MAIL_LIST_RECIPIENT_INFO  pMailList;
    DWORD                           dwRecipientIndex;
    DWORD                           dwKeyChoice;
    union {
        // CMSG_MAIL_LIST_HANDLE_KEY_CHOICE
        HCRYPTKEY                       hKeyEncryptionKey;
        // Reserve space for a potential pointer choice
        void                            *pvKeyEncryptionKey;
    } DUMMYUNIONNAME;
} CMSG_CTRL_MAIL_LIST_DECRYPT_PARA, *PCMSG_CTRL_MAIL_LIST_DECRYPT_PARA;



//+-------------------------------------------------------------------------
//  CMSG_CTRL_VERIFY_HASH
//
//  Verify the hash of a HASHED message after it has been decoded.
//
//  Only the hCryptMsg parameter is used, to specify the message whose
//  hash is being verified.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CTRL_ADD_SIGNER
//
//  Add a signer to a signed-data message.
//
//  pvCtrlPara points to a CMSG_SIGNER_ENCODE_INFO.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CTRL_ADD_CMS_SIGNER_INFO
//
//  Add a signer to a signed-data message.
//
//  Differs from the above, CMSG_CTRL_ADD_SIGNER, wherein, the signer info
//  already contains the signature.
//
//  pvCtrlPara points to a CMSG_CMS_SIGNER_INFO.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CTRL_DEL_SIGNER
//
//  Remove a signer from a signed-data or signed-and-enveloped-data message.
//
//  pvCtrlPara points to a DWORD containing the 0-based index of the
//  signer to be removed.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR
//
//  Add an unauthenticated attribute to the SignerInfo of a signed-data or
//  signed-and-enveloped-data message.
//
//  The unauthenticated attribute is input in the form of an encoded blob.
//--------------------------------------------------------------------------

typedef struct _CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA {
    DWORD               cbSize;
    DWORD               dwSignerIndex;
    CRYPT_DATA_BLOB     blob;
} CMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA, *PCMSG_CTRL_ADD_SIGNER_UNAUTH_ATTR_PARA;

//+-------------------------------------------------------------------------
//  CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR
//
//  Delete an unauthenticated attribute from the SignerInfo of a signed-data
//  or signed-and-enveloped-data message.
//
//  The unauthenticated attribute to be removed is specified by
//  a 0-based index.
//--------------------------------------------------------------------------

typedef struct _CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA {
    DWORD               cbSize;
    DWORD               dwSignerIndex;
    DWORD               dwUnauthAttrIndex;
} CMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA, *PCMSG_CTRL_DEL_SIGNER_UNAUTH_ATTR_PARA;

//+-------------------------------------------------------------------------
//  CMSG_CTRL_ADD_CERT
//
//  Add a certificate to a signed-data or signed-and-enveloped-data message.
//
//  pvCtrlPara points to a CRYPT_DATA_BLOB containing the certificate's
//  encoded bytes.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CTRL_DEL_CERT
//
//  Delete a certificate from a signed-data or signed-and-enveloped-data
//  message.
//
//  pvCtrlPara points to a DWORD containing the 0-based index of the
//  certificate to be removed.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CTRL_ADD_CRL
//
//  Add a CRL to a signed-data or signed-and-enveloped-data message.
//
//  pvCtrlPara points to a CRYPT_DATA_BLOB containing the CRL's
//  encoded bytes.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CTRL_DEL_CRL
//
//  Delete a CRL from a signed-data or signed-and-enveloped-data message.
//
//  pvCtrlPara points to a DWORD containing the 0-based index of the CRL
//  to be removed.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CTRL_ADD_ATTR_CERT
//
//  Add an attribute certificate to a signed-data message.
//
//  pvCtrlPara points to a CRYPT_DATA_BLOB containing the attribute
//  certificate's encoded bytes.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CMSG_CTRL_DEL_ATTR_CERT
//
//  Delete an attribute certificate from a signed-data message.
//
//  pvCtrlPara points to a DWORD containing the 0-based index of the
//  attribute certificate to be removed.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  Verify a countersignature, at the SignerInfo level.
//  ie. verify that pbSignerInfoCountersignature contains the encrypted
//  hash of the encryptedDigest field of pbSignerInfo.
//
//  hCryptProv is used to hash the encryptedDigest field of pbSignerInfo.
//  The only fields referenced from pciCountersigner are SerialNumber, Issuer,
//  and SubjectPublicKeyInfo.
//--------------------------------------------------------------------------
BOOL
WINAPI
CryptMsgVerifyCountersignatureEncoded(
    _In_opt_ HCRYPTPROV_LEGACY hCryptProv,
    _In_ DWORD dwEncodingType,
    _In_reads_bytes_(cbSignerInfo) PBYTE pbSignerInfo,
    _In_ DWORD cbSignerInfo,
    _In_reads_bytes_(cbSignerInfoCountersignature) PBYTE pbSignerInfoCountersignature,
    _In_ DWORD cbSignerInfoCountersignature,
    _In_ PCERT_INFO pciCountersigner
    );


//+-------------------------------------------------------------------------
//  Verify a countersignature, at the SignerInfo level.
//  ie. verify that pbSignerInfoCountersignature contains the encrypted
//  hash of the encryptedDigest field of pbSignerInfo.
//
//  hCryptProv is used to hash the encryptedDigest field of pbSignerInfo.
//
//  The signer can be a CERT_PUBLIC_KEY_INFO, certificate context or a
//  chain context.
//--------------------------------------------------------------------------
BOOL
WINAPI
CryptMsgVerifyCountersignatureEncodedEx(
    _In_opt_ HCRYPTPROV_LEGACY hCryptProv,
    _In_ DWORD dwEncodingType,
    _In_reads_bytes_(cbSignerInfo) PBYTE pbSignerInfo,
    _In_ DWORD cbSignerInfo,
    _In_reads_bytes_(cbSignerInfoCountersignature) PBYTE pbSignerInfoCountersignature,
    _In_ DWORD cbSignerInfoCountersignature,
    _In_ DWORD dwSignerType,
    _In_ void *pvSigner,
    _In_ DWORD dwFlags,
    _Inout_opt_ void *pvExtra
    );

// See CMSG_CTRL_VERIFY_SIGNATURE_EX_PARA for dwSignerType definitions

// When set, pvExtra points to const CERT_STRONG_SIGN_PARA struct
#define CMSG_VERIFY_COUNTER_SIGN_ENABLE_STRONG_FLAG         0x00000001


//+-------------------------------------------------------------------------
//  Countersign an already-existing signature in a message
//
//  dwIndex is a zero-based index of the SignerInfo to be countersigned.
//--------------------------------------------------------------------------
BOOL
WINAPI
CryptMsgCountersign(
    _In_ HCRYPTMSG hCryptMsg,
    _In_ DWORD dwIndex,
    _In_ DWORD cCountersigners,
    _In_reads_(cCountersigners) PCMSG_SIGNER_ENCODE_INFO rgCountersigners
    );

//+-------------------------------------------------------------------------
//  Countersign an already-existing signature (encoded SignerInfo).
//  Output an encoded SignerInfo blob, suitable for use as a countersignature
//  attribute in the unauthenticated attributes of a signed-data or
//  signed-and-enveloped-data message.
//--------------------------------------------------------------------------
BOOL
WINAPI
CryptMsgCountersignEncoded(
    _In_ DWORD dwEncodingType,
    _In_reads_bytes_(cbSignerInfo) PBYTE pbSignerInfo,
    _In_ DWORD cbSignerInfo,
    _In_ DWORD cCountersigners,
    _In_reads_(cCountersigners) PCMSG_SIGNER_ENCODE_INFO rgCountersigners,
    _Out_writes_bytes_to_opt_(*pcbCountersignature, *pcbCountersignature) PBYTE pbCountersignature,
    _Inout_ PDWORD pcbCountersignature
    );

//+-------------------------------------------------------------------------
//  CryptMsg OID installable functions
//--------------------------------------------------------------------------

typedef void * (WINAPI *PFN_CMSG_ALLOC) (
    _In_ size_t cb
    );

typedef void (WINAPI *PFN_CMSG_FREE)(
    _Inout_ void *pv
    );

// Note, the following 3 installable functions are obsolete and have been
// replaced with GenContentEncryptKey, ExportKeyTrans, ExportKeyAgree,
// ExportMailList, ImportKeyTrans, ImportKeyAgree and ImportMailList
// installable functions.

// If *phCryptProv is NULL upon entry, then, if supported, the installable
// function should acquire a default provider and return. Note, its up
// to the installable function to release at process detach.
//
// If paiEncrypt->Parameters.cbData is 0, then, the callback may optionally
// return default encoded parameters in *ppbEncryptParameters and
// *pcbEncryptParameters. pfnAlloc must be called for the allocation.
#define CMSG_OID_GEN_ENCRYPT_KEY_FUNC   "CryptMsgDllGenEncryptKey"
typedef _Success_(return != FALSE) BOOL (WINAPI *PFN_CMSG_GEN_ENCRYPT_KEY) (
    _Inout_ HCRYPTPROV *phCryptProv,
    _In_ PCRYPT_ALGORITHM_IDENTIFIER paiEncrypt,
    _In_opt_ PVOID pvEncryptAuxInfo,
    _In_ PCERT_PUBLIC_KEY_INFO pPublicKeyInfo,
    __callback PFN_CMSG_ALLOC pfnAlloc,
    _Out_ HCRYPTKEY *phEncryptKey,
    _Outptr_result_bytebuffer_(*pcbEncryptParameters) PBYTE *ppbEncryptParameters,
    _Out_ PDWORD pcbEncryptParameters
    );

#define CMSG_OID_EXPORT_ENCRYPT_KEY_FUNC   "CryptMsgDllExportEncryptKey"
typedef BOOL (WINAPI *PFN_CMSG_EXPORT_ENCRYPT_KEY) (
    _In_ HCRYPTPROV hCryptProv,
    _In_ HCRYPTKEY hEncryptKey,
    _In_ PCERT_PUBLIC_KEY_INFO pPublicKeyInfo,
    _Out_writes_bytes_to_opt_(*pcbData, *pcbData) PBYTE pbData,
    _Inout_ PDWORD pcbData
    );

#define CMSG_OID_IMPORT_ENCRYPT_KEY_FUNC   "CryptMsgDllImportEncryptKey"
typedef BOOL (WINAPI *PFN_CMSG_IMPORT_ENCRYPT_KEY) (
    _In_ HCRYPTPROV hCryptProv,
    _In_ DWORD dwKeySpec,
    _In_ PCRYPT_ALGORITHM_IDENTIFIER paiEncrypt,
    _In_ PCRYPT_ALGORITHM_IDENTIFIER paiPubKey,
    _In_reads_bytes_(cbEncodedKey) PBYTE pbEncodedKey,
    _In_ DWORD cbEncodedKey,
    _Out_ HCRYPTKEY *phEncryptKey
    );


// To get the default installable function for GenContentEncryptKey,
// ExportKeyTrans, ExportKeyAgree, ExportMailList, ImportKeyTrans,
// ImportKeyAgree or ImportMailList call CryptGetOIDFunctionAddress()
// with the pszOID argument set to the following constant. dwEncodingType
// should be set to CRYPT_ASN_ENCODING or X509_ASN_ENCODING.
#define CMSG_DEFAULT_INSTALLABLE_FUNC_OID   ((LPCSTR) 1)

//+-------------------------------------------------------------------------
//  Content Encrypt Info
//
//  The following data structure contains the information shared between
//  the GenContentEncryptKey and the ExportKeyTrans, ExportKeyAgree and
//  ExportMailList installable functions.
//
//  For a ContentEncryptionAlgorithm.pszObjId having a "Special" algid, only
//  supported via CNG, for example, AES, then, fCNG will be set.
//  fCNG will also be set to TRUE for any ECC agreement or OAEP RSA transport
//  recipients.
//
//  When, fCNG is TRUE, the hCNGContentEncryptKey choice is selected and
//  pbCNGContentEncryptKeyObject and pbContentEncryptKey will be pfnAlloc'ed.
//--------------------------------------------------------------------------
typedef struct _CMSG_CONTENT_ENCRYPT_INFO {
    DWORD                       cbSize;
    HCRYPTPROV_LEGACY           hCryptProv;
    CRYPT_ALGORITHM_IDENTIFIER  ContentEncryptionAlgorithm;
    void                        *pvEncryptionAuxInfo;
    DWORD                       cRecipients;
    PCMSG_RECIPIENT_ENCODE_INFO rgCmsRecipients;
    PFN_CMSG_ALLOC              pfnAlloc;
    PFN_CMSG_FREE               pfnFree;
    DWORD                       dwEncryptFlags;
    union {
        // fCNG == FALSE
        HCRYPTKEY                   hContentEncryptKey;
        // fCNG == TRUE
        BCRYPT_KEY_HANDLE           hCNGContentEncryptKey;
    } DUMMYUNIONNAME;
    DWORD                       dwFlags;

    BOOL                        fCNG;
    // When fCNG == TRUE, pfnAlloc'ed
    BYTE                        *pbCNGContentEncryptKeyObject;
    BYTE                        *pbContentEncryptKey;
    DWORD                       cbContentEncryptKey;
} CMSG_CONTENT_ENCRYPT_INFO, *PCMSG_CONTENT_ENCRYPT_INFO;

#define CMSG_CONTENT_ENCRYPT_PAD_ENCODED_LEN_FLAG   0x00000001

#define CMSG_CONTENT_ENCRYPT_FREE_PARA_FLAG         0x00000001
#define CMSG_CONTENT_ENCRYPT_FREE_OBJID_FLAG        0x00000002
#define CMSG_CONTENT_ENCRYPT_RELEASE_CONTEXT_FLAG   0x00008000

//+-------------------------------------------------------------------------
// Upon input, ContentEncryptInfo has been initialized from the
// EnvelopedEncodeInfo.
//
// Note, if rgpRecipients instead of rgCmsRecipients are set in the
// EnvelopedEncodeInfo, then, the rgpRecipients have been converted
// to rgCmsRecipients in the ContentEncryptInfo.
//
// For fCNG == FALSE, the following fields may be changed in ContentEncryptInfo:
//      hContentEncryptKey
//      hCryptProv
//      ContentEncryptionAlgorithm.pszObjId
//      ContentEncryptionAlgorithm.Parameters
//      dwFlags
//
// For fCNG == TRUE, the following fields may be changed in ContentEncryptInfo:
//      hCNGContentEncryptKey
//      pbCNGContentEncryptKeyObject
//      pbContentEncryptKey
//      cbContentEncryptKey
//      ContentEncryptionAlgorithm.pszObjId
//      ContentEncryptionAlgorithm.Parameters
//      dwFlags
//
// All other fields in the ContentEncryptInfo are READONLY.
//
// If CMSG_CONTENT_ENCRYPT_PAD_ENCODED_LEN_FLAG is set upon entry
// in dwEncryptFlags, then, any potentially variable length encoded
// output should be padded with zeroes to always obtain the
// same maximum encoded length. This is necessary for
// CryptMsgCalculateEncodedLength() or CryptMsgOpenToEncode() with
// definite length streaming.
//
// For fCNG == FALSE:
//      The hContentEncryptKey must be updated.
//
//      If hCryptProv is NULL upon input, then, it must be updated.
//      If a HCRYPTPROV is acquired that must be released, then, the
//      CMSG_CONTENT_ENCRYPT_RELEASE_CONTEXT_FLAG must be set in dwFlags.
// Otherwise, for fCNG == TRUE:
//      The hCNGContentEncryptKey and cbContentEncryptKey must be updated and
//      pbCNGContentEncryptKeyObject and pbContentEncryptKey pfnAlloc'ed.
//      This key will be freed and destroyed when hCryptMsg is closed.
//
// If ContentEncryptionAlgorithm.pszObjId is changed, then, the
// CMSG_CONTENT_ENCRYPT_FREE_OBJID_FLAG must be set in dwFlags.
// If ContentEncryptionAlgorithm.Parameters is updated, then, the
// CMSG_CONTENT_ENCRYPT_FREE_PARA_FLAG must be set in dwFlags. pfnAlloc and
// pfnFree must be used for doing the allocation.
//
// ContentEncryptionAlgorithm.pszObjId is used to get the OIDFunctionAddress.
//--------------------------------------------------------------------------

// The following CAPI1 installable function is called when fCNG == FALSE.
#define CMSG_OID_GEN_CONTENT_ENCRYPT_KEY_FUNC  "CryptMsgDllGenContentEncryptKey"
#define CMSG_OID_CAPI1_GEN_CONTENT_ENCRYPT_KEY_FUNC CMSG_OID_GEN_CONTENT_ENCRYPT_KEY_FUNC

typedef BOOL (WINAPI *PFN_CMSG_GEN_CONTENT_ENCRYPT_KEY) (
    _Inout_ PCMSG_CONTENT_ENCRYPT_INFO pContentEncryptInfo,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved
    );

// The following installable function is called when fCNG == TRUE. It has the
// same API signature as for the above
// CMSG_OID_CAPI1_GEN_CONTENT_ENCRYPT_KEY_FUNC.
#define CMSG_OID_CNG_GEN_CONTENT_ENCRYPT_KEY_FUNC  "CryptMsgDllCNGGenContentEncryptKey"

//+-------------------------------------------------------------------------
//  Key Transport Encrypt Info
//
//  The following data structure contains the information updated by the
//  ExportKeyTrans installable function.
//--------------------------------------------------------------------------
typedef struct _CMSG_KEY_TRANS_ENCRYPT_INFO {
    DWORD                       cbSize;
    DWORD                       dwRecipientIndex;
    CRYPT_ALGORITHM_IDENTIFIER  KeyEncryptionAlgorithm;
    CRYPT_DATA_BLOB             EncryptedKey;
    DWORD                       dwFlags;
} CMSG_KEY_TRANS_ENCRYPT_INFO, *PCMSG_KEY_TRANS_ENCRYPT_INFO;

#define CMSG_KEY_TRANS_ENCRYPT_FREE_PARA_FLAG       0x00000001
#define CMSG_KEY_TRANS_ENCRYPT_FREE_OBJID_FLAG      0x00000002


//+-------------------------------------------------------------------------
// Upon input, KeyTransEncryptInfo has been initialized from the
// KeyTransEncodeInfo.
//
// The following fields may be changed in KeyTransEncryptInfo:
//      EncryptedKey
//      KeyEncryptionAlgorithm.pszObjId
//      KeyEncryptionAlgorithm.Parameters
//      dwFlags
//
// All other fields in the KeyTransEncryptInfo are READONLY.
//
// The EncryptedKey must be updated. The pfnAlloc and pfnFree specified in
// ContentEncryptInfo must be used for doing the allocation.
//
// If the KeyEncryptionAlgorithm.pszObjId is changed, then, the
// CMSG_KEY_TRANS_ENCRYPT_FREE_OBJID_FLAG  must be set in dwFlags.
// If the KeyEncryptionAlgorithm.Parameters is updated, then, the
// CMSG_KEY_TRANS_ENCRYPT_FREE_PARA_FLAG must be set in dwFlags.
// The pfnAlloc and pfnFree specified in ContentEncryptInfo must be used
// for doing the allocation.
//
// KeyEncryptionAlgorithm.pszObjId is used to get the OIDFunctionAddress.
//--------------------------------------------------------------------------

// The following CAPI1 installable function is called when
// pContentEncryptInfo->fCNG == FALSE.
#define CMSG_OID_EXPORT_KEY_TRANS_FUNC  "CryptMsgDllExportKeyTrans"
#define CMSG_OID_CAPI1_EXPORT_KEY_TRANS_FUNC CMSG_OID_EXPORT_KEY_TRANS_FUNC
typedef BOOL (WINAPI *PFN_CMSG_EXPORT_KEY_TRANS) (
    _In_ PCMSG_CONTENT_ENCRYPT_INFO pContentEncryptInfo,
    _In_ PCMSG_KEY_TRANS_RECIPIENT_ENCODE_INFO pKeyTransEncodeInfo,
    _Inout_ PCMSG_KEY_TRANS_ENCRYPT_INFO pKeyTransEncryptInfo,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved
    );

// The following CNG installable function is called when
// pContentEncryptInfo->fCNG == TRUE. It has the same API signature as for
// the above CMSG_OID_CAPI1_EXPORT_KEY_TRANS_FUNC.
#define CMSG_OID_CNG_EXPORT_KEY_TRANS_FUNC  "CryptMsgDllCNGExportKeyTrans"

//+-------------------------------------------------------------------------
//  Key Agree Key Encrypt Info
//
//  The following data structure contains the information updated by the
//  ExportKeyAgree installable function for each encrypted key agree
//  recipient.
//--------------------------------------------------------------------------
typedef struct _CMSG_KEY_AGREE_KEY_ENCRYPT_INFO {
    DWORD                       cbSize;
    CRYPT_DATA_BLOB             EncryptedKey;
} CMSG_KEY_AGREE_KEY_ENCRYPT_INFO, *PCMSG_KEY_AGREE_KEY_ENCRYPT_INFO;

//+-------------------------------------------------------------------------
//  Key Agree Encrypt Info
//
//  The following data structure contains the information applicable to
//  all recipients. Its updated by the ExportKeyAgree installable function.
//--------------------------------------------------------------------------
typedef struct _CMSG_KEY_AGREE_ENCRYPT_INFO {
    DWORD                       cbSize;
    DWORD                       dwRecipientIndex;
    CRYPT_ALGORITHM_IDENTIFIER  KeyEncryptionAlgorithm;
    CRYPT_DATA_BLOB             UserKeyingMaterial;
    DWORD                       dwOriginatorChoice;
    union {
        // CMSG_KEY_AGREE_ORIGINATOR_CERT
        CERT_ID                     OriginatorCertId;
        // CMSG_KEY_AGREE_ORIGINATOR_PUBLIC_KEY
        CERT_PUBLIC_KEY_INFO        OriginatorPublicKeyInfo;
    } DUMMYUNIONNAME;
    DWORD                       cKeyAgreeKeyEncryptInfo;
    PCMSG_KEY_AGREE_KEY_ENCRYPT_INFO *rgpKeyAgreeKeyEncryptInfo;
    DWORD                       dwFlags;
} CMSG_KEY_AGREE_ENCRYPT_INFO, *PCMSG_KEY_AGREE_ENCRYPT_INFO;

#define CMSG_KEY_AGREE_ENCRYPT_FREE_PARA_FLAG           0x00000001
#define CMSG_KEY_AGREE_ENCRYPT_FREE_MATERIAL_FLAG       0x00000002
#define CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_ALG_FLAG     0x00000004
#define CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_PARA_FLAG    0x00000008
#define CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_BITS_FLAG    0x00000010
#define CMSG_KEY_AGREE_ENCRYPT_FREE_OBJID_FLAG          0x00000020


//+-------------------------------------------------------------------------
// Upon input, KeyAgreeEncryptInfo has been initialized from the
// KeyAgreeEncodeInfo.
//
// The following fields may be changed in KeyAgreeEncryptInfo:
//      KeyEncryptionAlgorithm.pszObjId
//      KeyEncryptionAlgorithm.Parameters
//      UserKeyingMaterial
//      dwOriginatorChoice
//      OriginatorCertId
//      OriginatorPublicKeyInfo
//      dwFlags
//
// All other fields in the KeyAgreeEncryptInfo are READONLY.
//
// If the KeyEncryptionAlgorithm.pszObjId is changed, then, the
// CMSG_KEY_AGREE_ENCRYPT_FREE_OBJID_FLAG  must be set in dwFlags.
// If the KeyEncryptionAlgorithm.Parameters is updated, then, the
// CMSG_KEY_AGREE_ENCRYPT_FREE_PARA_FLAG must be set in dwFlags.
// The pfnAlloc and pfnFree specified in ContentEncryptInfo must be used
// for doing the allocation.
//
// If the UserKeyingMaterial is updated, then, the
// CMSG_KEY_AGREE_ENCRYPT_FREE_MATERIAL_FLAG must be set in dwFlags.
// pfnAlloc and pfnFree must be used for doing the allocation.
//
// The dwOriginatorChoice must be updated to either
// CMSG_KEY_AGREE_ORIGINATOR_CERT or CMSG_KEY_AGREE_ORIGINATOR_PUBLIC_KEY.
//
// If the OriginatorPublicKeyInfo is updated, then, the appropriate
// CMSG_KEY_AGREE_ENCRYPT_FREE_PUBKEY_*_FLAG must be set in dwFlags and
// pfnAlloc and pfnFree must be used for doing the allocation.
//
// If CMSG_CONTENT_ENCRYPT_PAD_ENCODED_LEN_FLAG is set upon entry
// in pContentEncryptInfo->dwEncryptFlags, then, the OriginatorPublicKeyInfo's
// Ephemeral PublicKey should be padded with zeroes to always obtain the
// same maximum encoded length. Note, the length of the generated ephemeral Y
// public key can vary depending on the number of leading zero bits.
//
// Upon input, the array of *rgpKeyAgreeKeyEncryptInfo has been initialized.
// The EncryptedKey must be updated for each recipient key.
// The pfnAlloc and pfnFree specified in
// ContentEncryptInfo must be used for doing the allocation.
//
// KeyEncryptionAlgorithm.pszObjId is used to get the OIDFunctionAddress.
//--------------------------------------------------------------------------

// The following CAPI1 installable function is called when
// pContentEncryptInfo->fCNG == FALSE.
#define CMSG_OID_EXPORT_KEY_AGREE_FUNC  "CryptMsgDllExportKeyAgree"
#define CMSG_OID_CAPI1_EXPORT_KEY_AGREE_FUNC CMSG_OID_EXPORT_KEY_AGREE_FUNC
typedef BOOL (WINAPI *PFN_CMSG_EXPORT_KEY_AGREE) (
    _In_ PCMSG_CONTENT_ENCRYPT_INFO pContentEncryptInfo,
    _In_ PCMSG_KEY_AGREE_RECIPIENT_ENCODE_INFO pKeyAgreeEncodeInfo,
    _Inout_ PCMSG_KEY_AGREE_ENCRYPT_INFO pKeyAgreeEncryptInfo,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved
    );

// The following CNG installable function is called when
// pContentEncryptInfo->fCNG == TRUE. It has the same API signature as for
// the above CMSG_OID_CAPI1_EXPORT_KEY_AGREE_FUNC.
#define CMSG_OID_CNG_EXPORT_KEY_AGREE_FUNC  "CryptMsgDllCNGExportKeyAgree"

//+-------------------------------------------------------------------------
//  Mail List Encrypt Info
//
//  The following data structure contains the information updated by the
//  ExportMailList installable function.
//--------------------------------------------------------------------------
typedef struct _CMSG_MAIL_LIST_ENCRYPT_INFO {
    DWORD                       cbSize;
    DWORD                       dwRecipientIndex;
    CRYPT_ALGORITHM_IDENTIFIER  KeyEncryptionAlgorithm;
    CRYPT_DATA_BLOB             EncryptedKey;
    DWORD                       dwFlags;
} CMSG_MAIL_LIST_ENCRYPT_INFO, *PCMSG_MAIL_LIST_ENCRYPT_INFO;

#define CMSG_MAIL_LIST_ENCRYPT_FREE_PARA_FLAG       0x00000001
#define CMSG_MAIL_LIST_ENCRYPT_FREE_OBJID_FLAG      0x00000002


//+-------------------------------------------------------------------------
// Upon input, MailListEncryptInfo has been initialized from the
// MailListEncodeInfo.
//
// The following fields may be changed in MailListEncryptInfo:
//      EncryptedKey
//      KeyEncryptionAlgorithm.pszObjId
//      KeyEncryptionAlgorithm.Parameters
//      dwFlags
//
// All other fields in the MailListEncryptInfo are READONLY.
//
// The EncryptedKey must be updated. The pfnAlloc and pfnFree specified in
// ContentEncryptInfo must be used for doing the allocation.
//
// If the KeyEncryptionAlgorithm.pszObjId is changed, then, the
// CMSG_MAIL_LIST_ENCRYPT_FREE_OBJID_FLAG must be set in dwFlags.
// If the KeyEncryptionAlgorithm.Parameters is updated, then, the
// CMSG_MAIL_LIST_ENCRYPT_FREE_PARA_FLAG must be set in dwFlags.
// The pfnAlloc and pfnFree specified in ContentEncryptInfo must be used
// for doing the allocation.
//
// KeyEncryptionAlgorithm.pszObjId is used to get the OIDFunctionAddress.
//
// Note, only has a CAPI1 installable function. No CNG installable function.
//--------------------------------------------------------------------------
// The following CAPI1 installable function is called when
// pContentEncryptInfo->fCNG == FALSE.
#define CMSG_OID_EXPORT_MAIL_LIST_FUNC  "CryptMsgDllExportMailList"
#define CMSG_OID_CAPI1_EXPORT_MAIL_LIST_FUNC CMSG_OID_EXPORT_MAIL_LIST_FUNC
typedef BOOL (WINAPI *PFN_CMSG_EXPORT_MAIL_LIST) (
    _In_ PCMSG_CONTENT_ENCRYPT_INFO pContentEncryptInfo,
    _In_ PCMSG_MAIL_LIST_RECIPIENT_ENCODE_INFO pMailListEncodeInfo,
    _Inout_ PCMSG_MAIL_LIST_ENCRYPT_INFO pMailListEncryptInfo,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved
    );


//+-------------------------------------------------------------------------
// CAPI1 OID Installable functions for importing an encoded and encrypted
// content encryption key.
//
// There's a different installable function for each CMS Recipient choice:
//  ImportKeyTrans
//  ImportKeyAgree
//  ImportMailList
//
// Iterates through the following OIDs to get the OID installable function:
//   KeyEncryptionOID!ContentEncryptionOID
//   KeyEncryptionOID
//   ContentEncryptionOID
//
// If the OID installable function doesn't support the specified
// KeyEncryption and ContentEncryption OIDs, then, return FALSE with
// LastError set to E_NOTIMPL.
//--------------------------------------------------------------------------
#define CMSG_OID_IMPORT_KEY_TRANS_FUNC   "CryptMsgDllImportKeyTrans"
#define CMSG_OID_CAPI1_IMPORT_KEY_TRANS_FUNC CMSG_OID_IMPORT_KEY_TRANS_FUNC
typedef BOOL (WINAPI *PFN_CMSG_IMPORT_KEY_TRANS) (
    _In_ PCRYPT_ALGORITHM_IDENTIFIER pContentEncryptionAlgorithm,
    _In_ PCMSG_CTRL_KEY_TRANS_DECRYPT_PARA pKeyTransDecryptPara,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved,
    _Out_ HCRYPTKEY *phContentEncryptKey
    );

#define CMSG_OID_IMPORT_KEY_AGREE_FUNC   "CryptMsgDllImportKeyAgree"
#define CMSG_OID_CAPI1_IMPORT_KEY_AGREE_FUNC CMSG_OID_IMPORT_KEY_AGREE_FUNC
typedef BOOL (WINAPI *PFN_CMSG_IMPORT_KEY_AGREE) (
    _In_ PCRYPT_ALGORITHM_IDENTIFIER pContentEncryptionAlgorithm,
    _In_ PCMSG_CTRL_KEY_AGREE_DECRYPT_PARA pKeyAgreeDecryptPara,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved,
    _Out_ HCRYPTKEY *phContentEncryptKey
    );

#define CMSG_OID_IMPORT_MAIL_LIST_FUNC   "CryptMsgDllImportMailList"
#define CMSG_OID_CAPI1_IMPORT_MAIL_LIST_FUNC  CMSG_OID_IMPORT_MAIL_LIST_FUNC
typedef BOOL (WINAPI *PFN_CMSG_IMPORT_MAIL_LIST) (
    _In_ PCRYPT_ALGORITHM_IDENTIFIER pContentEncryptionAlgorithm,
    _In_ PCMSG_CTRL_MAIL_LIST_DECRYPT_PARA pMailListDecryptPara,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved,
    _Out_ HCRYPTKEY *phContentEncryptKey
    );

//+-------------------------------------------------------------------------
//  CNG Content Decrypt Info
//
//  The following data structure contains the information shared between
//  CNGImportKeyTrans, CNGImportKeyAgree and CNGImportContentEncryptKey
//  installable functions.
//
//  pbContentEncryptKey and pbCNGContentEncryptKeyObject are allocated
//  and freed via pfnAlloc and pfnFree.
//--------------------------------------------------------------------------
typedef struct _CMSG_CNG_CONTENT_DECRYPT_INFO {
    DWORD                       cbSize;
    CRYPT_ALGORITHM_IDENTIFIER  ContentEncryptionAlgorithm;
    PFN_CMSG_ALLOC              pfnAlloc;
    PFN_CMSG_FREE               pfnFree;

    // This key must be used over the one in the DecryptPara. An
    // HCRYPTPROV in the DecryptPara may have been converted to a
    // NCRYPT_KEY_HANDLE.
    NCRYPT_KEY_HANDLE           hNCryptKey;

    BYTE                        *pbContentEncryptKey;
    DWORD                       cbContentEncryptKey;

    BCRYPT_KEY_HANDLE           hCNGContentEncryptKey;
    BYTE                        *pbCNGContentEncryptKeyObject;
} CMSG_CNG_CONTENT_DECRYPT_INFO, *PCMSG_CNG_CONTENT_DECRYPT_INFO;


//+-------------------------------------------------------------------------
// CNG OID Installable function for importing and decrypting a key transport
// recipient encrypted content encryption key.
//
// Upon input, CNGContentDecryptInfo has been initialized.
//
// The following fields must be updated using hNCryptKey to decrypt
// pKeyTransDecryptPara->pKeyTrans->EncryptedKey.
//      pbContentEncryptKey (pfnAlloc'ed)
//      cbContentEncryptKey
//
// All other fields in the CNGContentEncryptInfo are READONLY.
//
// pKeyTransDecryptPara->pKeyTrans->KeyEncryptionAlgorithm.pszObjId is used
// to get the OIDFunctionAddress.
//--------------------------------------------------------------------------
#define CMSG_OID_CNG_IMPORT_KEY_TRANS_FUNC  "CryptMsgDllCNGImportKeyTrans"
typedef BOOL (WINAPI *PFN_CMSG_CNG_IMPORT_KEY_TRANS) (
    _Inout_ PCMSG_CNG_CONTENT_DECRYPT_INFO pCNGContentDecryptInfo,
    _In_ PCMSG_CTRL_KEY_TRANS_DECRYPT_PARA pKeyTransDecryptPara,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved
    );

//+-------------------------------------------------------------------------
// CNG OID Installable function for importing and decrypting a key agreement
// recipient encrypted content encryption key.
//
// Upon input, CNGContentDecryptInfo has been initialized.
//
// The following fields must be updated using hNCryptKey to decrypt
// pKeyAgreeDecryptPara->pKeyAgree->rgpRecipientEncryptedKeys[
//  pKeyAgreeDecryptPara->dwRecipientEncryptedKeyIndex]->EncryptedKey.
//      pbContentEncryptKey (pfnAlloc'ed)
//      cbContentEncryptKey
//
// All other fields in the CNGContentEncryptInfo are READONLY.
//
// pKeyAgreeDecryptPara->pKeyAgree->KeyEncryptionAlgorithm.pszObjId is used
// to get the OIDFunctionAddress.
//--------------------------------------------------------------------------
#define CMSG_OID_CNG_IMPORT_KEY_AGREE_FUNC   "CryptMsgDllCNGImportKeyAgree"
typedef BOOL (WINAPI *PFN_CMSG_CNG_IMPORT_KEY_AGREE) (
    _Inout_ PCMSG_CNG_CONTENT_DECRYPT_INFO pCNGContentDecryptInfo,
    _In_ PCMSG_CTRL_KEY_AGREE_DECRYPT_PARA pKeyAgreeDecryptPara,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved
    );

//+-------------------------------------------------------------------------
// CNG OID Installable function for importing an already decrypted
// content encryption key.
//
// Upon input, CNGContentDecryptInfo has been initialized.
//
// The following fields must be updated using pbContentEncryptKey and
// cbContentEncryptKey:
//      hCNGContentEncryptKey
//      pbCNGContentEncryptKeyObject (pfnAlloc'ed)
//
// The hCNGContentEncryptKey will be destroyed when hCryptMsg is closed.
//
// All other fields in the CNGContentEncryptInfo are READONLY.
//
// ContentEncryptionAlgorithm.pszObjId is used to get the OIDFunctionAddress.
//--------------------------------------------------------------------------
#define CMSG_OID_CNG_IMPORT_CONTENT_ENCRYPT_KEY_FUNC  "CryptMsgDllCNGImportContentEncryptKey"
typedef BOOL (WINAPI *PFN_CMSG_CNG_IMPORT_CONTENT_ENCRYPT_KEY) (
    _Inout_ PCMSG_CNG_CONTENT_DECRYPT_INFO pCNGContentDecryptInfo,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved
    );


//+=========================================================================
//  Certificate Store Data Structures and APIs
//==========================================================================

//+-------------------------------------------------------------------------
//              In its most basic implementation, a cert store is simply a
//              collection of certificates and/or CRLs. This is the case when
//              a cert store is opened with all of its certificates and CRLs
//              coming from a PKCS #7 encoded cryptographic message.
//
//              Nonetheless, all cert stores have the following properties:
//               - A public key may have more than one certificate in the store.
//                 For example, a private/public key used for signing may have a
//                 certificate issued for VISA and another issued for
//                 Mastercard. Also, when a certificate is renewed there might
//                 be more than one certificate with the same subject and
//                 issuer.
//               - However, each certificate in the store is uniquely
//                 identified by its Issuer and SerialNumber.
//               - There's an issuer of subject certificate relationship. A
//                 certificate's issuer is found by doing a match of
//                 pSubjectCert->Issuer with pIssuerCert->Subject.
//                 The relationship is verified by using
//                 the issuer's public key to verify the subject certificate's
//                 signature. Note, there might be X.509 v3 extensions
//                 to assist in finding the issuer certificate.
//               - Since issuer certificates might be renewed, a subject
//                 certificate might have more than one issuer certificate.
//               - There's an issuer of CRL relationship. An
//                 issuer's CRL is found by doing a match of
//                 pIssuerCert->Subject with pCrl->Issuer.
//                 The relationship is verified by using
//                 the issuer's public key to verify the CRL's
//                 signature. Note, there might be X.509 v3 extensions
//                 to assist in finding the CRL.
//               - Since some issuers might support the X.509 v3 delta CRL
//                 extensions, an issuer might have more than one CRL.
//               - The store shouldn't have any redundant certificates or
//                 CRLs. There shouldn't be two certificates with the same
//                 Issuer and SerialNumber. There shouldn't be two CRLs with
//                 the same Issuer, ThisUpdate and NextUpdate.
//               - The store has NO policy or trust information. No
//                 certificates are tagged as being "root". Its up to
//                 the application to maintain a list of CertIds (Issuer +
//                 SerialNumber) for certificates it trusts.
//               - The store might contain bad certificates and/or CRLs.
//                 The issuer's signature of a subject certificate or CRL may
//                 not verify. Certificates or CRLs may not satisfy their
//                 time validity requirements. Certificates may be
//                 revoked.
//
//              In addition to the certificates and CRLs, properties can be
//              stored. There are two predefined property IDs for a user
//              certificate: CERT_KEY_PROV_HANDLE_PROP_ID and
//              CERT_KEY_PROV_INFO_PROP_ID. The CERT_KEY_PROV_HANDLE_PROP_ID
//              is a HCRYPTPROV handle to the private key assoicated
//              with the certificate. The CERT_KEY_PROV_INFO_PROP_ID contains
//              information to be used to call
//              CryptAcquireContext and CryptSetProvParam to get a handle
//              to the private key associated with the certificate.
//
//              There exists two more predefined property IDs for certificates
//              and CRLs, CERT_SHA1_HASH_PROP_ID and CERT_MD5_HASH_PROP_ID.
//              If these properties don't already exist, then, a hash of the
//              content is computed. (CERT_HASH_PROP_ID maps to the default
//              hash algorithm, currently, CERT_SHA1_HASH_PROP_ID).
//
//              There are additional APIs for creating certificate and CRL
//      contexts not in a store (CertCreateCertificateContext and
//      CertCreateCRLContext).
//
//--------------------------------------------------------------------------

typedef void *HCERTSTORE;

//+-------------------------------------------------------------------------
//  Certificate context.
//
//  A certificate context contains both the encoded and decoded representation
//  of a certificate. A certificate context returned by a cert store function
//  must be freed by calling the CertFreeCertificateContext function. The
//  CertDuplicateCertificateContext function can be called to make a duplicate
//  copy (which also must be freed by calling CertFreeCertificateContext).
//--------------------------------------------------------------------------
// certenrolls_begin -- CERT_CONTEXT
typedef struct _CERT_CONTEXT {
    DWORD                   dwCertEncodingType;
    BYTE                    *pbCertEncoded;
    DWORD                   cbCertEncoded;
    PCERT_INFO              pCertInfo;
    HCERTSTORE              hCertStore;
} CERT_CONTEXT, *PCERT_CONTEXT;
typedef const CERT_CONTEXT *PCCERT_CONTEXT;
// certenrolls_end

//+-------------------------------------------------------------------------
//  CRL context.
//
//  A CRL context contains both the encoded and decoded representation
//  of a CRL. A CRL context returned by a cert store function
//  must be freed by calling the CertFreeCRLContext function. The
//  CertDuplicateCRLContext function can be called to make a duplicate
//  copy (which also must be freed by calling CertFreeCRLContext).
//--------------------------------------------------------------------------
typedef struct _CRL_CONTEXT {
    DWORD                   dwCertEncodingType;
    BYTE                    *pbCrlEncoded;
    DWORD                   cbCrlEncoded;
    PCRL_INFO               pCrlInfo;
    HCERTSTORE              hCertStore;
} CRL_CONTEXT, *PCRL_CONTEXT;
typedef const CRL_CONTEXT *PCCRL_CONTEXT;

//+-------------------------------------------------------------------------
//  Certificate Trust List (CTL) context.
//
//  A CTL context contains both the encoded and decoded representation
//  of a CTL. Also contains an opened HCRYPTMSG handle to the decoded
//  cryptographic signed message containing the CTL_INFO as its inner content.
//  pbCtlContent is the encoded inner content of the signed message.
//
//  The CryptMsg APIs can be used to extract additional signer information.
//--------------------------------------------------------------------------
typedef struct _CTL_CONTEXT {
    DWORD                   dwMsgAndCertEncodingType;
    BYTE                    *pbCtlEncoded;
    DWORD                   cbCtlEncoded;
    PCTL_INFO               pCtlInfo;
    HCERTSTORE              hCertStore;
    HCRYPTMSG               hCryptMsg;
    BYTE                    *pbCtlContent;
    DWORD                   cbCtlContent;
} CTL_CONTEXT, *PCTL_CONTEXT;
typedef const CTL_CONTEXT *PCCTL_CONTEXT;


// certenrolld_begin -- CERT_*_PROP_ID
//+-------------------------------------------------------------------------
//  Certificate, CRL and CTL property IDs
//
//  See CertSetCertificateContextProperty or CertGetCertificateContextProperty
//  for usage information.
//--------------------------------------------------------------------------
#define CERT_KEY_PROV_HANDLE_PROP_ID        1
#define CERT_KEY_PROV_INFO_PROP_ID          2	// CRYPT_KEY_PROV_INFO
#define CERT_SHA1_HASH_PROP_ID              3
#define CERT_MD5_HASH_PROP_ID               4
#define CERT_HASH_PROP_ID                   CERT_SHA1_HASH_PROP_ID
#define CERT_KEY_CONTEXT_PROP_ID            5	// CERT_KEY_CONTEXT
#define CERT_KEY_SPEC_PROP_ID               6
#define CERT_IE30_RESERVED_PROP_ID          7
#define CERT_PUBKEY_HASH_RESERVED_PROP_ID   8
#define CERT_ENHKEY_USAGE_PROP_ID           9
#define CERT_CTL_USAGE_PROP_ID              CERT_ENHKEY_USAGE_PROP_ID
#define CERT_NEXT_UPDATE_LOCATION_PROP_ID   10
#define CERT_FRIENDLY_NAME_PROP_ID          11	// string
#define CERT_PVK_FILE_PROP_ID               12
#define CERT_DESCRIPTION_PROP_ID            13	// string
#define CERT_ACCESS_STATE_PROP_ID           14
#define CERT_SIGNATURE_HASH_PROP_ID         15
#define CERT_SMART_CARD_DATA_PROP_ID        16
#define CERT_EFS_PROP_ID                    17
#define CERT_FORTEZZA_DATA_PROP_ID          18
#define CERT_ARCHIVED_PROP_ID               19
#define CERT_KEY_IDENTIFIER_PROP_ID         20
#define CERT_AUTO_ENROLL_PROP_ID            21	// string:Template name
#define CERT_PUBKEY_ALG_PARA_PROP_ID        22
#define CERT_CROSS_CERT_DIST_POINTS_PROP_ID 23
#define CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID     24
#define CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID    25
#define CERT_ENROLLMENT_PROP_ID             26	// RequestId+CADNS+CACN+Friendly Name
#define CERT_DATE_STAMP_PROP_ID             27	// FILETIME
#define CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID  28
#define CERT_SUBJECT_NAME_MD5_HASH_PROP_ID  29
#define CERT_EXTENDED_ERROR_INFO_PROP_ID    30	// string

// Note, 32 - 35 are reserved for the CERT, CRL, CTL and KeyId file element IDs.
//       36 - 62 are reserved for future element IDs.

#define CERT_RENEWAL_PROP_ID                64
#define CERT_ARCHIVED_KEY_HASH_PROP_ID      65	// Encrypted key hash
#define CERT_AUTO_ENROLL_RETRY_PROP_ID      66	// AE_RETRY_INFO:cb+cRetry+FILETIME
#define CERT_AIA_URL_RETRIEVED_PROP_ID      67
#define CERT_AUTHORITY_INFO_ACCESS_PROP_ID  68
#define CERT_BACKED_UP_PROP_ID              69	// VARIANT_BOOL+FILETIME
#define CERT_OCSP_RESPONSE_PROP_ID          70
#define CERT_REQUEST_ORIGINATOR_PROP_ID     71	// string:machine DNS name
#define CERT_SOURCE_LOCATION_PROP_ID        72	// string
#define CERT_SOURCE_URL_PROP_ID             73	// string
#define CERT_NEW_KEY_PROP_ID                74
#define CERT_OCSP_CACHE_PREFIX_PROP_ID      75	// string
#define CERT_SMART_CARD_ROOT_INFO_PROP_ID   76	// CRYPT_SMART_CARD_ROOT_INFO
#define CERT_NO_AUTO_EXPIRE_CHECK_PROP_ID   77
#define CERT_NCRYPT_KEY_HANDLE_PROP_ID      78
#define CERT_HCRYPTPROV_OR_NCRYPT_KEY_HANDLE_PROP_ID   79

#define CERT_SUBJECT_INFO_ACCESS_PROP_ID    80
#define CERT_CA_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID  81
#define CERT_CA_DISABLE_CRL_PROP_ID         82
#define CERT_ROOT_PROGRAM_CERT_POLICIES_PROP_ID    83
#define CERT_ROOT_PROGRAM_NAME_CONSTRAINTS_PROP_ID 84
#define CERT_SUBJECT_OCSP_AUTHORITY_INFO_ACCESS_PROP_ID  85
#define CERT_SUBJECT_DISABLE_CRL_PROP_ID    86
#define CERT_CEP_PROP_ID                    87	// Version+PropFlags+AuthType+UrlFlags+CESAuthType+Url+Id+CESUrl+ReqId
// 88 reserved, originally used for CERT_CEP_PROP_ID

#define CERT_SIGN_HASH_CNG_ALG_PROP_ID      89	// eg: "RSA/SHA1"

#define CERT_SCARD_PIN_ID_PROP_ID           90
#define CERT_SCARD_PIN_INFO_PROP_ID         91

#define CERT_SUBJECT_PUB_KEY_BIT_LENGTH_PROP_ID 92
#define CERT_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID 93
#define CERT_ISSUER_PUB_KEY_BIT_LENGTH_PROP_ID 94
#define CERT_ISSUER_CHAIN_SIGN_HASH_CNG_ALG_PROP_ID 95
#define CERT_ISSUER_CHAIN_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID 96

#define CERT_NO_EXPIRE_NOTIFICATION_PROP_ID 97

// Following property isn't implicitly created via a GetProperty.
#define CERT_AUTH_ROOT_SHA256_HASH_PROP_ID  98

#define CERT_NCRYPT_KEY_HANDLE_TRANSFER_PROP_ID 99
#define CERT_HCRYPTPROV_TRANSFER_PROP_ID    100

// Smart card reader image path
#define CERT_SMART_CARD_READER_PROP_ID      101 //string

// Send as trusted issuer
#define CERT_SEND_AS_TRUSTED_ISSUER_PROP_ID 102 //boolean

#define CERT_KEY_REPAIR_ATTEMPTED_PROP_ID   103 // FILETME

#define CERT_DISALLOWED_FILETIME_PROP_ID    104
#define CERT_ROOT_PROGRAM_CHAIN_POLICIES_PROP_ID 105

// Smart card reader removable capabilities
#define CERT_SMART_CARD_READER_NON_REMOVABLE_PROP_ID      106 // boolean

#define CERT_SHA256_HASH_PROP_ID               107

#define CERT_SCEP_SERVER_CERTS_PROP_ID         108	// Pkcs7
#define CERT_SCEP_RA_SIGNATURE_CERT_PROP_ID    109	// sha1 Thumbprint
#define CERT_SCEP_RA_ENCRYPTION_CERT_PROP_ID   110	// sha1 Thumbprint
#define CERT_SCEP_CA_CERT_PROP_ID	       111	// sha1 Thumbprint
#define CERT_SCEP_SIGNER_CERT_PROP_ID          112	// sha1 Thumbprint
#define CERT_SCEP_NONCE_PROP_ID                113	// blob

// string: "CNGEncryptAlgId/CNGHashAlgId"  example: "3DES/SHA1"
#define CERT_SCEP_ENCRYPT_HASH_CNG_ALG_PROP_ID 114
#define CERT_SCEP_FLAGS_PROP_ID                115	// DWORD
#define CERT_SCEP_GUID_PROP_ID                 116	// string
#define CERT_SERIALIZABLE_KEY_CONTEXT_PROP_ID  117	// CERT_KEY_CONTEXT

// Binary: isolated 
#define CERT_ISOLATED_KEY_PROP_ID              118      // blob

#define CERT_SERIAL_CHAIN_PROP_ID              119
#define CERT_KEY_CLASSIFICATION_PROP_ID        120      // DWORD CertKeyType

// 1 byte value. Set to 1 if the certificate has the
// szOID_TLS_FEATURES_EXT extension and an integer set to 5
// correpsonding to the OCSP status_request TLS extension.
#define CERT_OCSP_MUST_STAPLE_PROP_ID          121

#define CERT_DISALLOWED_ENHKEY_USAGE_PROP_ID   122
#define CERT_NONCOMPLIANT_ROOT_URL_PROP_ID     123      // NULL terminated UNICODE string

#define CERT_PIN_SHA256_HASH_PROP_ID           124
#define CERT_CLR_DELETE_KEY_PROP_ID            125
#define CERT_NOT_BEFORE_FILETIME_PROP_ID       126
#define CERT_NOT_BEFORE_ENHKEY_USAGE_PROP_ID   127
#define CERT_DISALLOWED_CA_FILETIME_PROP_ID    128

#define CERT_SHA1_SHA256_HASH_PROP_ID          129

#define CERT_FIRST_RESERVED_PROP_ID            130



#define CERT_LAST_RESERVED_PROP_ID          0x00007FFF
#define CERT_FIRST_USER_PROP_ID             0x00008000
#define CERT_LAST_USER_PROP_ID              0x0000FFFF
// certenrolld_end


// Values for CERT_KEY_CLASSIFICATION_PROP_ID.
// Must be stored as a DWORD.

#if defined(__cplusplus) && !defined(SORTPP_PASS)
#define WINCRYPT_DWORD_CPP_ONLY		: DWORD
#else
#define WINCRYPT_DWORD_CPP_ONLY
#endif
typedef enum CertKeyType WINCRYPT_DWORD_CPP_ONLY
{
    KeyTypeOther             = 0,	// Unknown
    KeyTypeVirtualSmartCard  = 1,
    KeyTypePhysicalSmartCard = 2,
    KeyTypePassport          = 3,
    KeyTypePassportRemote    = 4,
    KeyTypePassportSmartCard = 5,
    KeyTypeHardware          = 6,
    KeyTypeSoftware          = 7,
    KeyTypeSelfSigned        = 8,
} CertKeyType;


#define IS_CERT_HASH_PROP_ID(X)     (CERT_SHA1_HASH_PROP_ID == (X) || \
                                        CERT_MD5_HASH_PROP_ID == (X) || \
                                        CERT_SHA256_HASH_PROP_ID == (X) || \
                                        CERT_SHA1_SHA256_HASH_PROP_ID == (X) || \
                                        CERT_SIGNATURE_HASH_PROP_ID == (X))

#define IS_PUBKEY_HASH_PROP_ID(X)     (CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID == (X) || \
                                        CERT_PIN_SHA256_HASH_PROP_ID == (X) || \
                                        CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID == (X))

#define IS_CHAIN_HASH_PROP_ID(X)     (CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID == (X) || \
                                        CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID == (X) || \
                                        CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID == (X) || \
                                        CERT_SUBJECT_NAME_MD5_HASH_PROP_ID == (X))


#define IS_STRONG_SIGN_PROP_ID(X)     (CERT_SIGN_HASH_CNG_ALG_PROP_ID == (X) || \
                                        CERT_SUBJECT_PUB_KEY_BIT_LENGTH_PROP_ID == (X) || \
                                        CERT_PUB_KEY_CNG_ALG_BIT_LENGTH_PROP_ID == (X))


//+-------------------------------------------------------------------------
//  Property OIDs
//--------------------------------------------------------------------------
// The OID component following the prefix contains the PROP_ID (decimal)
#define szOID_CERT_PROP_ID_PREFIX           "1.3.6.1.4.1.311.10.11."

#define _szPROP_ID(PropId)       #PropId

// Ansi OID string from Property Id:
#define szOID_CERT_PROP_ID(PropId) szOID_CERT_PROP_ID_PREFIX _szPROP_ID(PropId)

// Unicode OID string from Property Id:
#define __CRYPT32WTEXT(quote)           L##quote
#define _CRYPT32WTEXT(quote)            __CRYPT32WTEXT(quote)
#define wszOID_CERT_PROP_ID(PropId) \
        _CRYPT32WTEXT(szOID_CERT_PROP_ID_PREFIX) _CRYPT32WTEXT(_szPROP_ID(PropId))

// Use szOID_CERT_PROP_ID(CERT_KEY_IDENTIFIER_PROP_ID) instead:
#define szOID_CERT_KEY_IDENTIFIER_PROP_ID   "1.3.6.1.4.1.311.10.11.20"

// Use szOID_CERT_PROP_ID(CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID) instead:
#define szOID_CERT_ISSUER_SERIAL_NUMBER_MD5_HASH_PROP_ID \
                                            "1.3.6.1.4.1.311.10.11.28"
// Use szOID_CERT_PROP_ID(CERT_SUBJECT_NAME_MD5_HASH_PROP_ID) instead:
#define szOID_CERT_SUBJECT_NAME_MD5_HASH_PROP_ID \
                                            "1.3.6.1.4.1.311.10.11.29"

// Use szOID_CERT_PROP_ID(CERT_MD5_HASH_PROP_ID) instead:
#define szOID_CERT_MD5_HASH_PROP_ID         "1.3.6.1.4.1.311.10.11.4"

// Use szOID_CERT_PROP_ID(CERT_SHA256_HASH_PROP_ID) instead:
#define szOID_CERT_SHA256_HASH_PROP_ID         "1.3.6.1.4.1.311.10.11.107"

// Use szOID_CERT_PROP_ID(CERT_SIGNATURE_HASH_PROP_ID) instead:
#define szOID_CERT_SIGNATURE_HASH_PROP_ID   "1.3.6.1.4.1.311.10.11.15"


// The CERT_SIGNATURE_HASH_PROP_ID and CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID
// properties are used for disallowed hashes.
#define szOID_DISALLOWED_HASH               szOID_CERT_SIGNATURE_HASH_PROP_ID

// Use szOID_CERT_PROP_ID(CERT_DISALLOWED_FILETIME_PROP_ID) instead:
#define szOID_CERT_DISALLOWED_FILETIME_PROP_ID \
                                            "1.3.6.1.4.1.311.10.11.104"
// Use szOID_CERT_PROP_ID(CERT_DISALLOWED_CA_FILETIME_PROP_ID) instead:
#define szOID_CERT_DISALLOWED_CA_FILETIME_PROP_ID \
                                            "1.3.6.1.4.1.311.10.11.128"

//+-------------------------------------------------------------------------
//  Access State flags returned by CERT_ACCESS_STATE_PROP_ID. Note,
//  CERT_ACCESS_PROP_ID is read only.
//--------------------------------------------------------------------------

// Set if context property writes are persisted. For instance, not set for
// memory store contexts. Set for registry based stores opened as read or write.
// Not set for registry based stores opened as read only.
#define CERT_ACCESS_STATE_WRITE_PERSIST_FLAG    0x1

// Set if context resides in a SYSTEM or SYSTEM_REGISTRY store.
#define CERT_ACCESS_STATE_SYSTEM_STORE_FLAG     0x2

// Set if context resides in a LocalMachine SYSTEM or SYSTEM_REGISTRY store.
#define CERT_ACCESS_STATE_LM_SYSTEM_STORE_FLAG  0x4

// Set if context resides in a GroupPolicy SYSTEM or SYSTEM_REGISTRY store.
#define CERT_ACCESS_STATE_GP_SYSTEM_STORE_FLAG  0x8

// Set if context resides in a SHARED_USER physical store.
#define CERT_ACCESS_STATE_SHARED_USER_FLAG      0x10

//+-------------------------------------------------------------------------
//  CERT_ROOT_PROGRAM_CHAIN_POLICIES_PROP_ID Property
//
//  Encoded as an X509_ENHANCED_KEY_USAGE: sequence of Policy OIDs.
//--------------------------------------------------------------------------

// Supported Root Program Chain Policies:
#define szOID_ROOT_PROGRAM_AUTO_UPDATE_CA_REVOCATION    "1.3.6.1.4.1.311.60.3.1"
#define szOID_ROOT_PROGRAM_AUTO_UPDATE_END_REVOCATION   "1.3.6.1.4.1.311.60.3.2"
#define szOID_ROOT_PROGRAM_NO_OCSP_FAILOVER_TO_CRL      "1.3.6.1.4.1.311.60.3.3"

//+-------------------------------------------------------------------------
//  Cryptographic Key Provider Information
//
//  CRYPT_KEY_PROV_INFO defines the CERT_KEY_PROV_INFO_PROP_ID's pvData.
//
//  The CRYPT_KEY_PROV_INFO fields are passed to CryptAcquireContext
//  to get a HCRYPTPROV handle. The optional CRYPT_KEY_PROV_PARAM fields are
//  passed to CryptSetProvParam to further initialize the provider.
//
//  The dwKeySpec field identifies the private key to use from the container
//  For example, AT_KEYEXCHANGE or AT_SIGNATURE.
//--------------------------------------------------------------------------
typedef struct _CRYPT_KEY_PROV_PARAM {
    DWORD           dwParam;
    BYTE            *pbData;
    DWORD           cbData;
    DWORD           dwFlags;
} CRYPT_KEY_PROV_PARAM, *PCRYPT_KEY_PROV_PARAM;

typedef struct _CRYPT_KEY_PROV_INFO {
    LPWSTR                  pwszContainerName;
    LPWSTR                  pwszProvName;
    DWORD                   dwProvType;
    DWORD                   dwFlags;
    DWORD                   cProvParam;
    PCRYPT_KEY_PROV_PARAM   rgProvParam;
    DWORD                   dwKeySpec;
} CRYPT_KEY_PROV_INFO, *PCRYPT_KEY_PROV_INFO;

//+-------------------------------------------------------------------------
//  The following flag should be set in the above dwFlags to enable
//  a CertSetCertificateContextProperty(CERT_KEY_CONTEXT_PROP_ID) after a
//  CryptAcquireContext is done in the Sign or Decrypt Message functions.
//
//  The following define must not collide with any of the
//  CryptAcquireContext dwFlag defines.
//--------------------------------------------------------------------------
#define CERT_SET_KEY_PROV_HANDLE_PROP_ID    0x00000001
#define CERT_SET_KEY_CONTEXT_PROP_ID        0x00000001

// Special dwKeySpec indicating a CNG NCRYPT_KEY_HANDLE instead of a CAPI1
// HCRYPTPROV
#define CERT_NCRYPT_KEY_SPEC                0xFFFFFFFF

//+-------------------------------------------------------------------------
//  Certificate Key Context
//
//  CERT_KEY_CONTEXT defines the CERT_KEY_CONTEXT_PROP_ID's pvData.
//
//  dwKeySpec is set to the special CERT_NCRYPT_KEY_SPEC to select the
//  hNCryptKey choice.
//--------------------------------------------------------------------------
typedef struct _CERT_KEY_CONTEXT {
    DWORD           cbSize;           // sizeof(CERT_KEY_CONTEXT)
    union {
        HCRYPTPROV          hCryptProv;

        // dwKeySpec == CERT_NCRYPT_KEY_SPEC
        NCRYPT_KEY_HANDLE   hNCryptKey;
    } DUMMYUNIONNAME;
    DWORD           dwKeySpec;
} CERT_KEY_CONTEXT, *PCERT_KEY_CONTEXT;

//+-------------------------------------------------------------------------
//  Cryptographic Smart Card Root Information
//
//  CRYPT_SMART_CARD_ROOT_INFO defines the
//  CERT_SMART_CARD_ROOT_INFO_PROP_ID's pvData.
//--------------------------------------------------------------------------
typedef struct _ROOT_INFO_LUID {
    DWORD LowPart;
    LONG HighPart;
} ROOT_INFO_LUID, *PROOT_INFO_LUID;

typedef struct _CRYPT_SMART_CARD_ROOT_INFO {
    BYTE                rgbCardID [16];
    ROOT_INFO_LUID      luid;
} CRYPT_SMART_CARD_ROOT_INFO, *PCRYPT_SMART_CARD_ROOT_INFO;

//+-------------------------------------------------------------------------
//  Certificate Store Provider Types
//--------------------------------------------------------------------------
#define CERT_STORE_PROV_MSG                 ((LPCSTR) 1)
#define CERT_STORE_PROV_MEMORY              ((LPCSTR) 2)
#define CERT_STORE_PROV_FILE                ((LPCSTR) 3)
#define CERT_STORE_PROV_REG                 ((LPCSTR) 4)

#define CERT_STORE_PROV_PKCS7               ((LPCSTR) 5)
#define CERT_STORE_PROV_SERIALIZED          ((LPCSTR) 6)
#define CERT_STORE_PROV_FILENAME_A          ((LPCSTR) 7)
#define CERT_STORE_PROV_FILENAME_W          ((LPCSTR) 8)
#define CERT_STORE_PROV_FILENAME            CERT_STORE_PROV_FILENAME_W
#define CERT_STORE_PROV_SYSTEM_A            ((LPCSTR) 9)
#define CERT_STORE_PROV_SYSTEM_W            ((LPCSTR) 10)
#define CERT_STORE_PROV_SYSTEM              CERT_STORE_PROV_SYSTEM_W

#define CERT_STORE_PROV_COLLECTION          ((LPCSTR) 11)
#define CERT_STORE_PROV_SYSTEM_REGISTRY_A   ((LPCSTR) 12)
#define CERT_STORE_PROV_SYSTEM_REGISTRY_W   ((LPCSTR) 13)
#define CERT_STORE_PROV_SYSTEM_REGISTRY     CERT_STORE_PROV_SYSTEM_REGISTRY_W
#define CERT_STORE_PROV_PHYSICAL_W          ((LPCSTR) 14)
#define CERT_STORE_PROV_PHYSICAL            CERT_STORE_PROV_PHYSICAL_W

// SmartCard Store Provider isn't supported
#define CERT_STORE_PROV_SMART_CARD_W        ((LPCSTR) 15)
#define CERT_STORE_PROV_SMART_CARD          CERT_STORE_PROV_SMART_CARD_W

#define CERT_STORE_PROV_LDAP_W              ((LPCSTR) 16)
#define CERT_STORE_PROV_LDAP                CERT_STORE_PROV_LDAP_W
#define CERT_STORE_PROV_PKCS12              ((LPCSTR) 17)

#define sz_CERT_STORE_PROV_MEMORY           "Memory"
#define sz_CERT_STORE_PROV_FILENAME_W       "File"
#define sz_CERT_STORE_PROV_FILENAME         sz_CERT_STORE_PROV_FILENAME_W
#define sz_CERT_STORE_PROV_SYSTEM_W         "System"
#define sz_CERT_STORE_PROV_SYSTEM           sz_CERT_STORE_PROV_SYSTEM_W
#define sz_CERT_STORE_PROV_PKCS7            "PKCS7"
#define sz_CERT_STORE_PROV_PKCS12           "PKCS12"
#define sz_CERT_STORE_PROV_SERIALIZED       "Serialized"

#define sz_CERT_STORE_PROV_COLLECTION       "Collection"
#define sz_CERT_STORE_PROV_SYSTEM_REGISTRY_W "SystemRegistry"
#define sz_CERT_STORE_PROV_SYSTEM_REGISTRY  sz_CERT_STORE_PROV_SYSTEM_REGISTRY_W
#define sz_CERT_STORE_PROV_PHYSICAL_W       "Physical"
#define sz_CERT_STORE_PROV_PHYSICAL         sz_CERT_STORE_PROV_PHYSICAL_W

// SmartCard Store Provider isn't supported
#define sz_CERT_STORE_PROV_SMART_CARD_W     "SmartCard"
#define sz_CERT_STORE_PROV_SMART_CARD       sz_CERT_STORE_PROV_SMART_CARD_W

#define sz_CERT_STORE_PROV_LDAP_W           "Ldap"
#define sz_CERT_STORE_PROV_LDAP             sz_CERT_STORE_PROV_LDAP_W

//+-------------------------------------------------------------------------
//  Certificate Store verify/results flags
//--------------------------------------------------------------------------
#define CERT_STORE_SIGNATURE_FLAG           0x00000001
#define CERT_STORE_TIME_VALIDITY_FLAG       0x00000002
#define CERT_STORE_REVOCATION_FLAG          0x00000004
#define CERT_STORE_NO_CRL_FLAG              0x00010000
#define CERT_STORE_NO_ISSUER_FLAG           0x00020000

#define CERT_STORE_BASE_CRL_FLAG            0x00000100
#define CERT_STORE_DELTA_CRL_FLAG           0x00000200


//+-------------------------------------------------------------------------
//  Certificate Store open/property flags
//--------------------------------------------------------------------------
#define CERT_STORE_NO_CRYPT_RELEASE_FLAG                0x00000001
#define CERT_STORE_SET_LOCALIZED_NAME_FLAG              0x00000002
#define CERT_STORE_DEFER_CLOSE_UNTIL_LAST_FREE_FLAG     0x00000004
#define CERT_STORE_DELETE_FLAG                          0x00000010
#define CERT_STORE_UNSAFE_PHYSICAL_FLAG                 0x00000020
#define CERT_STORE_SHARE_STORE_FLAG                     0x00000040
#define CERT_STORE_SHARE_CONTEXT_FLAG                   0x00000080
#define CERT_STORE_MANIFOLD_FLAG                        0x00000100
#define CERT_STORE_ENUM_ARCHIVED_FLAG                   0x00000200
#define CERT_STORE_UPDATE_KEYID_FLAG                    0x00000400
#define CERT_STORE_BACKUP_RESTORE_FLAG                  0x00000800
#define CERT_STORE_READONLY_FLAG                        0x00008000
#define CERT_STORE_OPEN_EXISTING_FLAG                   0x00004000
#define CERT_STORE_CREATE_NEW_FLAG                      0x00002000
#define CERT_STORE_MAXIMUM_ALLOWED_FLAG                 0x00001000

//+-------------------------------------------------------------------------
//  Certificate Store Provider flags are in the HiWord (0xFFFF0000)
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  Certificate System Store Flag Values
//--------------------------------------------------------------------------
// Includes flags and location
#define CERT_SYSTEM_STORE_MASK                  0xFFFF0000

// Set if pvPara points to a CERT_SYSTEM_STORE_RELOCATE_PARA structure
#define CERT_SYSTEM_STORE_RELOCATE_FLAG         0x80000000

typedef struct _CERT_SYSTEM_STORE_RELOCATE_PARA {
    union {
        HKEY                hKeyBase;
        void                *pvBase;
    } DUMMYUNIONNAME;
    union {
        void                *pvSystemStore;
        LPCSTR              pszSystemStore;
        LPCWSTR             pwszSystemStore;
    } DUMMYUNIONNAME2;
} CERT_SYSTEM_STORE_RELOCATE_PARA, *PCERT_SYSTEM_STORE_RELOCATE_PARA;

// By default, when the CurrentUser "Root" store is opened, any SystemRegistry
// roots not also on the protected root list are deleted from the cache before
// CertOpenStore() returns. Set the following flag to return all the roots
// in the SystemRegistry without checking the protected root list.
#define CERT_SYSTEM_STORE_UNPROTECTED_FLAG      0x40000000

#define CERT_SYSTEM_STORE_DEFER_READ_FLAG       0x20000000

// Location of the system store:
#define CERT_SYSTEM_STORE_LOCATION_MASK         0x00FF0000
#define CERT_SYSTEM_STORE_LOCATION_SHIFT        16


//  Registry: HKEY_CURRENT_USER or HKEY_LOCAL_MACHINE
#define CERT_SYSTEM_STORE_CURRENT_USER_ID       1
#define CERT_SYSTEM_STORE_LOCAL_MACHINE_ID      2
//  Registry: HKEY_LOCAL_MACHINE\Software\Microsoft\Cryptography\Services
#define CERT_SYSTEM_STORE_CURRENT_SERVICE_ID    4
#define CERT_SYSTEM_STORE_SERVICES_ID           5
//  Registry: HKEY_USERS
#define CERT_SYSTEM_STORE_USERS_ID              6

//  Registry: HKEY_CURRENT_USER\Software\Policies\Microsoft\SystemCertificates
#define CERT_SYSTEM_STORE_CURRENT_USER_GROUP_POLICY_ID    7
//  Registry: HKEY_LOCAL_MACHINE\Software\Policies\Microsoft\SystemCertificates
#define CERT_SYSTEM_STORE_LOCAL_MACHINE_GROUP_POLICY_ID   8

//  Registry: HKEY_LOCAL_MACHINE\Software\Microsoft\EnterpriseCertificates
#define CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE_ID     9

//  Registry: HKEY_LOCAL_MACHINE\OSDATA\Software\Microsoft\WCOSCertificates
#define CERT_SYSTEM_STORE_LOCAL_MACHINE_WCOS_ID           10

#define CERT_SYSTEM_STORE_CURRENT_USER          \
    (CERT_SYSTEM_STORE_CURRENT_USER_ID << CERT_SYSTEM_STORE_LOCATION_SHIFT)
#define CERT_SYSTEM_STORE_LOCAL_MACHINE         \
    (CERT_SYSTEM_STORE_LOCAL_MACHINE_ID << CERT_SYSTEM_STORE_LOCATION_SHIFT)
#define CERT_SYSTEM_STORE_CURRENT_SERVICE       \
    (CERT_SYSTEM_STORE_CURRENT_SERVICE_ID << CERT_SYSTEM_STORE_LOCATION_SHIFT)
#define CERT_SYSTEM_STORE_SERVICES              \
    (CERT_SYSTEM_STORE_SERVICES_ID << CERT_SYSTEM_STORE_LOCATION_SHIFT)
#define CERT_SYSTEM_STORE_USERS                 \
    (CERT_SYSTEM_STORE_USERS_ID << CERT_SYSTEM_STORE_LOCATION_SHIFT)

#define CERT_SYSTEM_STORE_CURRENT_USER_GROUP_POLICY   \
    (CERT_SYSTEM_STORE_CURRENT_USER_GROUP_POLICY_ID << \
        CERT_SYSTEM_STORE_LOCATION_SHIFT)
#define CERT_SYSTEM_STORE_LOCAL_MACHINE_GROUP_POLICY  \
    (CERT_SYSTEM_STORE_LOCAL_MACHINE_GROUP_POLICY_ID << \
        CERT_SYSTEM_STORE_LOCATION_SHIFT)

#define CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE  \
    (CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE_ID << \
        CERT_SYSTEM_STORE_LOCATION_SHIFT)

#define CERT_SYSTEM_STORE_LOCAL_MACHINE_WCOS  \
    (CERT_SYSTEM_STORE_LOCAL_MACHINE_WCOS_ID << \
        CERT_SYSTEM_STORE_LOCATION_SHIFT)


//+-------------------------------------------------------------------------
//  Group Policy Store Defines
//--------------------------------------------------------------------------
// Registry path to the Group Policy system stores
#define CERT_GROUP_POLICY_SYSTEM_STORE_REGPATH \
    L"Software\\Policies\\Microsoft\\SystemCertificates"

//+-------------------------------------------------------------------------
//  EFS Defines
//--------------------------------------------------------------------------
// Registry path to the EFS EFSBlob SubKey - Value type is REG_BINARY
#define CERT_EFSBLOB_REGPATH    \
    CERT_GROUP_POLICY_SYSTEM_STORE_REGPATH L"\\EFS"
#define CERT_EFSBLOB_VALUE_NAME L"EFSBlob"

//+-------------------------------------------------------------------------
//  Protected Root Defines
//--------------------------------------------------------------------------
// Registry path to the Protected Roots Flags SubKey
#define CERT_PROT_ROOT_FLAGS_REGPATH    \
    CERT_GROUP_POLICY_SYSTEM_STORE_REGPATH L"\\Root\\ProtectedRoots"

// The following is a REG_DWORD. The bit definitions follow.
#define CERT_PROT_ROOT_FLAGS_VALUE_NAME L"Flags"

// Set the following flag to inhibit the opening of the CurrentUser's
// .Default physical store when opening the CurrentUser's "Root" system store.
// The .Default physical store open's the CurrentUser SystemRegistry "Root"
// store.
#define CERT_PROT_ROOT_DISABLE_CURRENT_USER_FLAG    0x1

// Set the following flag to inhibit the adding of roots from the
// CurrentUser SystemRegistry "Root" store to the protected root list
// when the "Root" store is initially protected.
#define CERT_PROT_ROOT_INHIBIT_ADD_AT_INIT_FLAG     0x2

// Set the following flag to inhibit the purging of protected roots from the
// CurrentUser SystemRegistry "Root" store that are
// also in the LocalMachine SystemRegistry "Root" store. Note, when not
// disabled, the purging is done silently without UI.
#define CERT_PROT_ROOT_INHIBIT_PURGE_LM_FLAG        0x4

// Set the following flag to inhibit the opening of the LocalMachine's
// .AuthRoot physical store when opening the LocalMachine's "Root" system store.
// The .AuthRoot physical store open's the LocalMachine SystemRegistry
// "AuthRoot" store. The "AuthRoot" store contains the pre-installed
// SSL ServerAuth and the ActiveX Authenticode "root" certificates.
#define CERT_PROT_ROOT_DISABLE_LM_AUTH_FLAG         0x8

// The semantics for the following legacy definition has been changed to be
// the same as for the CERT_PROT_ROOT_DISABLE_LM_AUTH_FLAG.
#define CERT_PROT_ROOT_ONLY_LM_GPT_FLAG             0x8

// Set the following flag to disable the requiring of the issuing CA
// certificate being in the "NTAuth" system registry store found in the
// CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE store location.
//
// When set, CertVerifyCertificateChainPolicy(CERT_CHAIN_POLICY_NT_AUTH)
// will check that the chain has a valid name constraint for all name
// spaces, including UPN if the issuing CA isn't in the "NTAuth" store.
#define CERT_PROT_ROOT_DISABLE_NT_AUTH_REQUIRED_FLAG 0x10

// Set the following flag to disable checking for not defined name
// constraints.
//
// When set, CertGetCertificateChain won't check for or set the following
// dwErrorStatus: CERT_TRUST_HAS_NOT_DEFINED_NAME_CONSTRAINT.
//
// In LH, checking for not defined name constraints is always disabled.
#define CERT_PROT_ROOT_DISABLE_NOT_DEFINED_NAME_CONSTRAINT_FLAG 0x20

// Set the following flag to disallow the users to trust peer-trust
#define CERT_PROT_ROOT_DISABLE_PEER_TRUST                       0x10000

// The following is a REG_MULTI_SZ containing the list of user allowed
// Enhanced Key Usages for peer trust.
#define CERT_PROT_ROOT_PEER_USAGES_VALUE_NAME     L"PeerUsages"
#define CERT_PROT_ROOT_PEER_USAGES_VALUE_NAME_A   "PeerUsages"

// If the above REG_MULTI_SZ isn't defined or is empty, defaults to
// the following multi-string value
#define CERT_PROT_ROOT_PEER_USAGES_DEFAULT_A      \
    szOID_PKIX_KP_CLIENT_AUTH "\0"                  \
    szOID_PKIX_KP_EMAIL_PROTECTION "\0"             \
    szOID_KP_EFS "\0"

//+-------------------------------------------------------------------------
//  Trusted Publisher Definitions
//--------------------------------------------------------------------------
// Registry path to the trusted publisher "Safer" group policy subkey
#define CERT_TRUST_PUB_SAFER_GROUP_POLICY_REGPATH    \
    CERT_GROUP_POLICY_SYSTEM_STORE_REGPATH L"\\TrustedPublisher\\Safer"


// Registry path to the Local Machine system stores
#define CERT_LOCAL_MACHINE_SYSTEM_STORE_REGPATH \
    L"Software\\Microsoft\\SystemCertificates"

// Registry path to the trusted publisher "Safer" local machine subkey
#define CERT_TRUST_PUB_SAFER_LOCAL_MACHINE_REGPATH    \
    CERT_LOCAL_MACHINE_SYSTEM_STORE_REGPATH L"\\TrustedPublisher\\Safer"


// "Safer" subkey value names. All values are DWORDs.
#define CERT_TRUST_PUB_AUTHENTICODE_FLAGS_VALUE_NAME    L"AuthenticodeFlags"


// AuthenticodeFlags definitions

// Definition of who is allowed to trust publishers
//
// Setting allowed trust to MACHINE_ADMIN or ENTERPRISE_ADMIN disables UI,
// only trusts publishers in the "TrustedPublisher" system store and
// inhibits the opening of the CurrentUser's .Default physical store when
// opening the CurrentUsers's "TrustedPublisher" system store.
//
// The .Default physical store open's the CurrentUser SystemRegistry
// "TrustedPublisher" store.
//
// Setting allowed trust to ENTERPRISE_ADMIN only opens the
// LocalMachine's .GroupPolicy and .Enterprise physical stores when opening
// the CurrentUser's "TrustedPublisher" system store or when opening the
// LocalMachine's "TrustedPublisher" system store.

#define CERT_TRUST_PUB_ALLOW_TRUST_MASK                 0x00000003
#define CERT_TRUST_PUB_ALLOW_END_USER_TRUST             0x00000000
#define CERT_TRUST_PUB_ALLOW_MACHINE_ADMIN_TRUST        0x00000001
#define CERT_TRUST_PUB_ALLOW_ENTERPRISE_ADMIN_TRUST     0x00000002

// Set the following flag to enable revocation checking of the publisher
// chain.
#define CERT_TRUST_PUB_CHECK_PUBLISHER_REV_FLAG         0x00000100

// Set the following flag to enable revocation checking of the time stamp
// chain.
#define CERT_TRUST_PUB_CHECK_TIMESTAMP_REV_FLAG         0x00000200


//+-------------------------------------------------------------------------
//  OCM Subcomponents Definitions
//
//  Reading of the following registry key has been deprecated on Vista.
//--------------------------------------------------------------------------

// Registry path to the OCM Subcomponents local machine subkey
#define CERT_OCM_SUBCOMPONENTS_LOCAL_MACHINE_REGPATH        \
    L"SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Setup\\OC Manager\\Subcomponents"

// REG_DWORD, 1 is installed, 0 is NOT installed
#define CERT_OCM_SUBCOMPONENTS_ROOT_AUTO_UPDATE_VALUE_NAME  L"RootAutoUpdate"


//+-------------------------------------------------------------------------
//  Root, Disallowed Certificate and Pin Rules AutoUpdate Defines
//--------------------------------------------------------------------------
// Registry path to the DisableRootAutoUpdate SubKey
#define CERT_DISABLE_ROOT_AUTO_UPDATE_REGPATH    \
    CERT_GROUP_POLICY_SYSTEM_STORE_REGPATH L"\\AuthRoot"

// The following disables Root, Disallowed Certificate and Pin Rules AutoUpdate
// REG_DWORD Value Name, 1 - disables, 0 - enables
#define CERT_DISABLE_ROOT_AUTO_UPDATE_VALUE_NAME L"DisableRootAutoUpdate"

// The following enables Disallowed Certificate and Pin Rules AutoUpdate.
// It takes precedence over the above registry setting.
// REG_DWORD Value Name, 1 - enables
#define CERT_ENABLE_DISALLOWED_CERT_AUTO_UPDATE_VALUE_NAME L"EnableDisallowedCertAutoUpdate"

// The following disables Pin Rules AutoUpdate.
// It takes precedence over the above registry setting.
// REG_DWORD Value Name, 1 - disables
#define CERT_DISABLE_PIN_RULES_AUTO_UPDATE_VALUE_NAME L"DisablePinRulesAutoUpdate"

//+-------------------------------------------------------------------------
//  Auto Update Definitions
//--------------------------------------------------------------------------

// Registry path to the "Auto Update" local machine subkey
#define CERT_AUTO_UPDATE_LOCAL_MACHINE_REGPATH    \
    CERT_LOCAL_MACHINE_SYSTEM_STORE_REGPATH L"\\AuthRoot\\AutoUpdate"

// Auto Update subkey value names.

// REG_SZ, URL to the directory containing the AutoUpdate files
#define CERT_AUTO_UPDATE_ROOT_DIR_URL_VALUE_NAME                L"RootDirUrl"

// REG_SZ, URL to the AutoUpdate test staging directory containing the
// AutoUpdate files. certutil.exe will use for its -syncWithWU and
// -generateSSTFromWU verbs to override the default Windows Update URL.
#define CERT_AUTO_UPDATE_SYNC_FROM_DIR_URL_VALUE_NAME           L"SyncFromDirUrl"



//+-------------------------------------------------------------------------
//  AuthRoot Auto Update Definitions
//--------------------------------------------------------------------------

// Registry path to the AuthRoot "Auto Update" local machine subkey
#define CERT_AUTH_ROOT_AUTO_UPDATE_LOCAL_MACHINE_REGPATH        \
    CERT_AUTO_UPDATE_LOCAL_MACHINE_REGPATH


// AuthRoot Auto Update subkey value names.

// REG_SZ, URL to the directory containing the AuthRoots, CTL and Seq files
#define CERT_AUTH_ROOT_AUTO_UPDATE_ROOT_DIR_URL_VALUE_NAME      \
    CERT_AUTO_UPDATE_ROOT_DIR_URL_VALUE_NAME

// REG_DWORD, seconds between syncs. 0 implies use default.
#define CERT_AUTH_ROOT_AUTO_UPDATE_SYNC_DELTA_TIME_VALUE_NAME   L"SyncDeltaTime"

// REG_DWORD, misc flags
#define CERT_AUTH_ROOT_AUTO_UPDATE_FLAGS_VALUE_NAME             L"Flags"

#define CERT_AUTH_ROOT_AUTO_UPDATE_DISABLE_UNTRUSTED_ROOT_LOGGING_FLAG  0x1
#define CERT_AUTH_ROOT_AUTO_UPDATE_DISABLE_PARTIAL_CHAIN_LOGGING_FLAG   0x2

// By default a random query string is appended to the Auto Update URLs
// passed to CryptRetrieveObjectByUrlW. See the
// CRYPT_RANDOM_QUERY_STRING_RETRIEVAL flag for more details. Set
// this flag to not set this random query string. This might be the 
// case when setting CERT_AUTO_UPDATE_ROOT_DIR_URL_VALUE_NAME where the
// server doesn't strip off the query string.
#define CERT_AUTO_UPDATE_DISABLE_RANDOM_QUERY_STRING_FLAG               0x4

// REG_BINARY, updated with FILETIME of last wire retrieval of authroot cab/ctl
#define CERT_AUTH_ROOT_AUTO_UPDATE_LAST_SYNC_TIME_VALUE_NAME    L"LastSyncTime"

// REG_BINARY, updated with last retrieved and verified authroot ctl
#define CERT_AUTH_ROOT_AUTO_UPDATE_ENCODED_CTL_VALUE_NAME       L"EncodedCtl"


// AuthRoot Auto Update filenames

// CTL containing the list of certs in the AuthRoot store
#define CERT_AUTH_ROOT_CTL_FILENAME                             L"authroot.stl"
#define CERT_AUTH_ROOT_CTL_FILENAME_A                           "authroot.stl"

// Cab containing the above CTL
#define CERT_AUTH_ROOT_CAB_FILENAME                             L"authrootstl.cab"

// SequenceNumber (Formatted as big endian ascii hex)
#define CERT_AUTH_ROOT_SEQ_FILENAME                             L"authrootseq.txt"

// Root certs extension
#define CERT_AUTH_ROOT_CERT_EXT                                 L".crt"


//+-------------------------------------------------------------------------
//  DisallowedCert Auto Update Definitions
//--------------------------------------------------------------------------

//
// DisallowedCert Auto Update subkey value names.
//


// REG_DWORD, seconds between syncs. 0 implies use default.
#define CERT_DISALLOWED_CERT_AUTO_UPDATE_SYNC_DELTA_TIME_VALUE_NAME \
    L"DisallowedCertSyncDeltaTime"

// REG_BINARY, updated with FILETIME of last wire retrieval of disallowed cert
// CTL
#define CERT_DISALLOWED_CERT_AUTO_UPDATE_LAST_SYNC_TIME_VALUE_NAME  \
    L"DisallowedCertLastSyncTime"

// REG_BINARY, updated with last retrieved and verified disallowed cert ctl
#define CERT_DISALLOWED_CERT_AUTO_UPDATE_ENCODED_CTL_VALUE_NAME     \
    L"DisallowedCertEncodedCtl"

//
// DisallowedCert Auto Update filenames
//

// CTL containing the list of disallowed certs
#define CERT_DISALLOWED_CERT_CTL_FILENAME               L"disallowedcert.stl"
#define CERT_DISALLOWED_CERT_CTL_FILENAME_A             "disallowedcert.stl"

// Cab containing disallowed certs  CTL
#define CERT_DISALLOWED_CERT_CAB_FILENAME               L"disallowedcertstl.cab"

//
// DisallowedCert Auto Update CTL List Identifiers
//

// Disallowed Cert CTL List Identifier
#define CERT_DISALLOWED_CERT_AUTO_UPDATE_LIST_IDENTIFIER            \
    L"DisallowedCert_AutoUpdate_1"


//+-------------------------------------------------------------------------
//  PinRules Auto Update Definitions
//--------------------------------------------------------------------------

//
// PinRules Auto Update subkey value names.
//


// REG_DWORD, seconds between syncs. 0 implies use default.
#define CERT_PIN_RULES_AUTO_UPDATE_SYNC_DELTA_TIME_VALUE_NAME \
    L"PinRulesSyncDeltaTime"

// REG_BINARY, updated with FILETIME of last wire retrieval of pin rules 
// CTL
#define CERT_PIN_RULES_AUTO_UPDATE_LAST_SYNC_TIME_VALUE_NAME  \
    L"PinRulesLastSyncTime"

// REG_BINARY, updated with last retrieved and verified pin rules ctl
#define CERT_PIN_RULES_AUTO_UPDATE_ENCODED_CTL_VALUE_NAME     \
    L"PinRulesEncodedCtl"

//
// PinRules Auto Update filenames
//

// CTL containing the list of pin rules
#define CERT_PIN_RULES_CTL_FILENAME                 L"pinrules.stl"
#define CERT_PIN_RULES_CTL_FILENAME_A               "pinrules.stl"

// Cab containing pin rules  CTL
#define CERT_PIN_RULES_CAB_FILENAME                 L"pinrulesstl.cab"

//
// Pin Rules Auto Update CTL List Identifiers
//

// Pin Rules CTL List Identifier
#define CERT_PIN_RULES_AUTO_UPDATE_LIST_IDENTIFIER            \
    L"PinRules_AutoUpdate_1"


//+-------------------------------------------------------------------------
//  Certificate Registry Store Flag Values (CERT_STORE_REG)
//--------------------------------------------------------------------------

// Set this flag if the HKEY passed in pvPara points to a remote computer
// registry key.
#define CERT_REGISTRY_STORE_REMOTE_FLAG         0x10000

// Set this flag if the contexts are to be persisted as a single serialized
// store in the registry. Mainly used for stores downloaded from the GPT.
// Such as the CurrentUserGroupPolicy or LocalMachineGroupPolicy stores.
#define CERT_REGISTRY_STORE_SERIALIZED_FLAG     0x20000

// The following flags are for internal use. When set, the
// pvPara parameter passed to CertOpenStore is a pointer to the following
// data structure and not the HKEY. The above CERT_REGISTRY_STORE_REMOTE_FLAG
// is also set if hKeyBase was obtained via RegConnectRegistry().
#define CERT_REGISTRY_STORE_CLIENT_GPT_FLAG     0x80000000
#define CERT_REGISTRY_STORE_LM_GPT_FLAG         0x01000000

typedef struct _CERT_REGISTRY_STORE_CLIENT_GPT_PARA {
    HKEY                hKeyBase;
    LPWSTR              pwszRegPath;
} CERT_REGISTRY_STORE_CLIENT_GPT_PARA, *PCERT_REGISTRY_STORE_CLIENT_GPT_PARA;

// The following flag is for internal use. When set, the contexts are
// persisted into roaming files instead of the registry. Such as, the
// CurrentUser "My" store. When this flag is set, the following data structure
// is passed to CertOpenStore instead of HKEY.
#define CERT_REGISTRY_STORE_ROAMING_FLAG        0x40000

// hKey may be NULL or non-NULL. When non-NULL, existing contexts are
// moved from the registry to roaming files.
typedef struct _CERT_REGISTRY_STORE_ROAMING_PARA {
    HKEY                hKey;
    LPWSTR              pwszStoreDirectory;
} CERT_REGISTRY_STORE_ROAMING_PARA, *PCERT_REGISTRY_STORE_ROAMING_PARA;

// The following flag is for internal use. When set, the "My" DWORD value
// at HKLM\Software\Microsoft\Cryptography\IEDirtyFlags is set to 0x1
// whenever a certificate is added to the registry store.
//
// Legacy definition, no longer supported after 01-May-02 (Server 2003)
#define CERT_REGISTRY_STORE_MY_IE_DIRTY_FLAG    0x80000

#define CERT_REGISTRY_STORE_EXTERNAL_FLAG      0x100000

// Registry path to the subkey containing the "My" DWORD value to be set
//
// Legacy definition, no longer supported after 01-May-02 (Server 2003)
#define CERT_IE_DIRTY_FLAGS_REGPATH \
    L"Software\\Microsoft\\Cryptography\\IEDirtyFlags"


//+-------------------------------------------------------------------------
//  Certificate File Store Flag Values for the providers:
//      CERT_STORE_PROV_FILE
//      CERT_STORE_PROV_FILENAME
//      CERT_STORE_PROV_FILENAME_A
//      CERT_STORE_PROV_FILENAME_W
//      sz_CERT_STORE_PROV_FILENAME_W
//--------------------------------------------------------------------------

// Set this flag if any store changes are to be committed to the file.
// The changes are committed at CertCloseStore or by calling
// CertControlStore(CERT_STORE_CTRL_COMMIT).
//
// The open fails with E_INVALIDARG if both CERT_FILE_STORE_COMMIT_ENABLE_FLAG
// and CERT_STORE_READONLY_FLAG are set in dwFlags.
//
// For the FILENAME providers:  if the file contains an X509 encoded
// certificate, the open fails with ERROR_ACCESS_DENIED.
//
// For the FILENAME providers: if CERT_STORE_CREATE_NEW_FLAG is set, the
// CreateFile uses CREATE_NEW. If CERT_STORE_OPEN_EXISTING is set, uses
// OPEN_EXISTING. Otherwise, defaults to OPEN_ALWAYS.
//
// For the FILENAME providers:  the file is committed as either a PKCS7 or
// serialized store depending on the type read at open. However, if the
// file is empty then, if the filename has either a ".p7c" or ".spc"
// extension its committed as a PKCS7. Otherwise, its committed as a
// serialized store.
//
// For CERT_STORE_PROV_FILE, the file handle is duplicated. Its always
// committed as a serialized store.
//
#define CERT_FILE_STORE_COMMIT_ENABLE_FLAG              0x10000


//+-------------------------------------------------------------------------
//  Certificate LDAP Store Flag Values for the providers:
//      CERT_STORE_PROV_LDAP
//      CERT_STORE_PROV_LDAP_W
//      sz_CERT_STORE_PROV_LDAP_W
//      sz_CERT_STORE_PROV_LDAP
//--------------------------------------------------------------------------

// Set this flag to digitally sign all of the ldap traffic to and from a
// Windows 2000 LDAP server using the Kerberos authentication protocol.
// This feature provides integrity required by some applications.
//
#define CERT_LDAP_STORE_SIGN_FLAG               0x10000

// Performs an A-Record only DNS lookup on the supplied host string.
// This prevents bogus DNS queries from being generated when resolving host
// names. Use this flag whenever passing a hostname as opposed to a
// domain name for the hostname parameter.
//
// See LDAP_OPT_AREC_EXCLUSIVE defined in winldap.h for more details.
#define CERT_LDAP_STORE_AREC_EXCLUSIVE_FLAG     0x20000

// Set this flag if the LDAP session handle has already been opened. When
// set, pvPara points to the following CERT_LDAP_STORE_OPENED_PARA structure.
#define CERT_LDAP_STORE_OPENED_FLAG             0x40000

typedef struct _CERT_LDAP_STORE_OPENED_PARA {
    void        *pvLdapSessionHandle;   // The (LDAP *) handle returned by
                                        // ldap_init
    LPCWSTR     pwszLdapUrl;
} CERT_LDAP_STORE_OPENED_PARA, *PCERT_LDAP_STORE_OPENED_PARA;


// Set this flag if the above CERT_LDAP_STORE_OPENED_FLAG is set and
// you want an ldap_unbind() of the above pvLdapSessionHandle when the
// store is closed. Note, if CertOpenStore() fails, then, ldap_unbind()
// isn't called.
#define CERT_LDAP_STORE_UNBIND_FLAG             0x80000

//+-------------------------------------------------------------------------
//  Open the cert store using the specified store provider.
//
//  If CERT_STORE_DELETE_FLAG is set, then, the store is deleted. NULL is
//  returned for both success and failure. However, GetLastError() returns 0
//  for success and nonzero for failure.
//
//  If CERT_STORE_SET_LOCALIZED_NAME_FLAG is set, then, if supported, the
//  provider sets the store's CERT_STORE_LOCALIZED_NAME_PROP_ID property.
//  The store's localized name can be retrieved by calling
//  CertSetStoreProperty(dwPropID = CERT_STORE_LOCALIZED_NAME_PROP_ID).
//  This flag is supported by the following providers (and their sz_
//  equivalent):
//      CERT_STORE_PROV_FILENAME_A
//      CERT_STORE_PROV_FILENAME_W
//      CERT_STORE_PROV_SYSTEM_A
//      CERT_STORE_PROV_SYSTEM_W
//      CERT_STORE_PROV_SYSTEM_REGISTRY_A
//      CERT_STORE_PROV_SYSTEM_REGISTRY_W
//      CERT_STORE_PROV_PHYSICAL_W
//
//  If CERT_STORE_DEFER_CLOSE_UNTIL_LAST_FREE_FLAG is set, then, the
//  closing of the store's provider is deferred until all certificate,
//  CRL and CTL contexts obtained from the store are freed. Also,
//  if a non NULL HCRYPTPROV was passed, then, it will continue to be used.
//  By default, the store's provider is closed on the final CertCloseStore.
//  If this flag isn't set, then, any property changes made to previously
//  duplicated contexts after the final CertCloseStore will not be persisted.
//  By setting this flag, property changes made
//  after the CertCloseStore will be persisted. Note, setting this flag
//  causes extra overhead in doing context duplicates and frees.
//  If CertCloseStore is called with CERT_CLOSE_STORE_FORCE_FLAG, then,
//  the CERT_STORE_DEFER_CLOSE_UNTIL_LAST_FREE_FLAG flag is ignored.
//
//  CERT_STORE_MANIFOLD_FLAG can be set to check for certificates having the
//  manifold extension and archive the "older" certificates with the same
//  manifold extension value. A certificate is archived by setting the
//  CERT_ARCHIVED_PROP_ID.
//
//  By default, contexts having the CERT_ARCHIVED_PROP_ID, are skipped
//  during enumeration. CERT_STORE_ENUM_ARCHIVED_FLAG can be set to include
//  archived contexts when enumerating. Note, contexts having the
//  CERT_ARCHIVED_PROP_ID are still found for explicit finds, such as,
//  finding a context with a specific hash or finding a certificate having
//  a specific issuer and serial number.
//
//  CERT_STORE_UPDATE_KEYID_FLAG can be set to also update the Key Identifier's
//  CERT_KEY_PROV_INFO_PROP_ID property whenever a certificate's
//  CERT_KEY_IDENTIFIER_PROP_ID or CERT_KEY_PROV_INFO_PROP_ID property is set
//  and the other property already exists. If the Key Identifier's
//  CERT_KEY_PROV_INFO_PROP_ID already exists, it isn't updated. Any
//  errors encountered are silently ignored.
//
//  By default, this flag is implicitly set for the "My\.Default" CurrentUser
//  and LocalMachine physical stores.
//
//  CERT_STORE_READONLY_FLAG can be set to open the store as read only.
//  Otherwise, the store is opened as read/write.
//
//  CERT_STORE_OPEN_EXISTING_FLAG can be set to only open an existing
//  store. CERT_STORE_CREATE_NEW_FLAG can be set to create a new store and
//  fail if the store already exists. Otherwise, the default is to open
//  an existing store or create a new store if it doesn't already exist.
//
//  hCryptProv specifies the crypto provider to use to create the hash
//  properties or verify the signature of a subject certificate or CRL.
//  The store doesn't need to use a private
//  key. If the CERT_STORE_NO_CRYPT_RELEASE_FLAG isn't set, hCryptProv is
//  CryptReleaseContext'ed on the final CertCloseStore.
//
//  Note, if the open fails, hCryptProv is released if it would have been
//  released when the store was closed.
//
//  If hCryptProv is zero, then, the default provider and container for the
//  PROV_RSA_FULL provider type is CryptAcquireContext'ed with
//  CRYPT_VERIFYCONTEXT access. The CryptAcquireContext is deferred until
//  the first create hash or verify signature. In addition, once acquired,
//  the default provider isn't released until process exit when crypt32.dll
//  is unloaded. The acquired default provider is shared across all stores
//  and threads.
//
//  After initializing the store's data structures and optionally acquiring a
//  default crypt provider, CertOpenStore calls CryptGetOIDFunctionAddress to
//  get the address of the CRYPT_OID_OPEN_STORE_PROV_FUNC specified by
//  lpszStoreProvider. Since a store can contain certificates with different
//  encoding types, CryptGetOIDFunctionAddress is called with dwEncodingType
//  set to 0 and not the dwEncodingType passed to CertOpenStore.
//  PFN_CERT_DLL_OPEN_STORE_FUNC specifies the signature of the provider's
//  open function. This provider open function is called to load the
//  store's certificates and CRLs. Optionally, the provider may return an
//  array of functions called before a certificate or CRL is added or deleted
//  or has a property that is set.
//
//  Use of the dwEncodingType parameter is provider dependent. The type
//  definition for pvPara also depends on the provider.
//
//  Store providers are installed or registered via
//  CryptInstallOIDFunctionAddress or CryptRegisterOIDFunction, where,
//  dwEncodingType is 0 and pszFuncName is CRYPT_OID_OPEN_STORE_PROV_FUNC.
//
//  Here's a list of the predefined provider types (implemented in crypt32.dll):
//
//  CERT_STORE_PROV_MSG:
//      Gets the certificates and CRLs from the specified cryptographic message.
//      dwEncodingType contains the message and certificate encoding types.
//      The message's handle is passed in pvPara. Given,
//          HCRYPTMSG hCryptMsg; pvPara = (const void *) hCryptMsg;
//
//  CERT_STORE_PROV_MEMORY
//  sz_CERT_STORE_PROV_MEMORY:
//      Opens a store without any initial certificates or CRLs. pvPara
//      isn't used.
//
//  CERT_STORE_PROV_FILE:
//      Reads the certificates and CRLs from the specified file. The file's
//      handle is passed in pvPara. Given,
//          HANDLE hFile; pvPara = (const void *) hFile;
//
//      For a successful open, the file pointer is advanced past
//      the certificates and CRLs and their properties read from the file.
//      Note, only expects a serialized store and not a file containing
//      either a PKCS #7 signed message or a single encoded certificate.
//
//      The hFile isn't closed.
//
//  CERT_STORE_PROV_REG:
//      Reads the certificates and CRLs from the registry. The registry's
//      key handle is passed in pvPara. Given,
//          HKEY hKey; pvPara = (const void *) hKey;
//
//      The input hKey isn't closed by the provider. Before returning, the
//      provider opens it own copy of the hKey.
//
//      If CERT_STORE_READONLY_FLAG is set, then, the registry subkeys are
//      RegOpenKey'ed with KEY_READ_ACCESS. Otherwise, the registry subkeys
//      are RegCreateKey'ed with KEY_ALL_ACCESS.
//
//      This provider returns the array of functions for reading, writing,
//      deleting and property setting certificates and CRLs.
//      Any changes to the opened store are immediately pushed through to
//      the registry. However, if CERT_STORE_READONLY_FLAG is set, then,
//      writing, deleting or property setting results in a
//      SetLastError(E_ACCESSDENIED).
//
//      Note, all the certificates and CRLs are read from the registry
//      when the store is opened. The opened store serves as a write through
//      cache.
//
//      If CERT_REGISTRY_STORE_SERIALIZED_FLAG is set, then, the
//      contexts are persisted as a single serialized store subkey in the
//      registry.
//
//  CERT_STORE_PROV_PKCS7:
//  sz_CERT_STORE_PROV_PKCS7:
//      Gets the certificates and CRLs from the encoded PKCS #7 signed message.
//      dwEncodingType specifies the message and certificate encoding types.
//      The pointer to the encoded message's blob is passed in pvPara. Given,
//          CRYPT_DATA_BLOB EncodedMsg; pvPara = (const void *) &EncodedMsg;
//
//      Note, also supports the IE3.0 special version of a
//      PKCS #7 signed message referred to as a "SPC" formatted message.
//
//  CERT_STORE_PROV_SERIALIZED:
//  sz_CERT_STORE_PROV_SERIALIZED:
//      Gets the certificates and CRLs from memory containing a serialized
//      store.  The pointer to the serialized memory blob is passed in pvPara.
//      Given,
//          CRYPT_DATA_BLOB Serialized; pvPara = (const void *) &Serialized;
//
//  CERT_STORE_PROV_FILENAME_A:
//  CERT_STORE_PROV_FILENAME_W:
//  CERT_STORE_PROV_FILENAME:
//  sz_CERT_STORE_PROV_FILENAME_W:
//  sz_CERT_STORE_PROV_FILENAME:
//      Opens the file and first attempts to read as a serialized store. Then,
//      as a PKCS #7 signed message. Finally, as a single encoded certificate.
//      The filename is passed in pvPara. The filename is UNICODE for the
//      "_W" provider and ASCII for the "_A" provider. For "_W": given,
//          LPCWSTR pwszFilename; pvPara = (const void *) pwszFilename;
//      For "_A": given,
//          LPCSTR pszFilename; pvPara = (const void *) pszFilename;
//
//      Note, the default (without "_A" or "_W") is unicode.
//
//      Note, also supports the reading of the IE3.0 special version of a
//      PKCS #7 signed message file referred to as a "SPC" formatted file.
//
//  CERT_STORE_PROV_SYSTEM_A:
//  CERT_STORE_PROV_SYSTEM_W:
//  CERT_STORE_PROV_SYSTEM:
//  sz_CERT_STORE_PROV_SYSTEM_W:
//  sz_CERT_STORE_PROV_SYSTEM:
//      Opens the specified logical "System" store. The upper word of the
//      dwFlags parameter is used to specify the location of the system store.
//
//      A "System" store is a collection consisting of one or more "Physical"
//      stores. A "Physical" store is registered via the
//      CertRegisterPhysicalStore API. Each of the registered physical stores
//      is CertStoreOpen'ed and added to the collection via
//      CertAddStoreToCollection.
//
//      The CERT_SYSTEM_STORE_CURRENT_USER, CERT_SYSTEM_STORE_LOCAL_MACHINE,
//      CERT_SYSTEM_STORE_CURRENT_SERVICE, CERT_SYSTEM_STORE_SERVICES,
//      CERT_SYSTEM_STORE_USERS, CERT_SYSTEM_STORE_CURRENT_USER_GROUP_POLICY,
//      CERT_SYSTEM_STORE_LOCAL_MACHINE_GROUP_POLICY and
//      CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRSE
//      system stores by default have a "SystemRegistry" store that is
//      opened and added to the collection.
//
//      The system store name is passed in pvPara. The name is UNICODE for the
//      "_W" provider and ASCII for the "_A" provider. For "_W": given,
//          LPCWSTR pwszSystemName; pvPara = (const void *) pwszSystemName;
//      For "_A": given,
//          LPCSTR pszSystemName; pvPara = (const void *) pszSystemName;
//
//      Note, the default (without "_A" or "_W") is UNICODE.
//
//      The system store name can't contain any backslashes.
//
//      If CERT_SYSTEM_STORE_RELOCATE_FLAG is set in dwFlags, pvPara
//      points to a CERT_SYSTEM_STORE_RELOCATE_PARA data structure instead
//      of pointing to a null terminated UNICODE or ASCII string.
//      Sibling physical stores are also opened as relocated using
//      pvPara's hKeyBase.
//
//      The CERT_SYSTEM_STORE_SERVICES or CERT_SYSTEM_STORE_USERS system
//      store name must be prefixed with the ServiceName or UserName.
//      For example, "ServiceName\Trust".
//
//      Stores on remote computers can be accessed for the
//      CERT_SYSTEM_STORE_LOCAL_MACHINE, CERT_SYSTEM_STORE_SERVICES,
//      CERT_SYSTEM_STORE_USERS, CERT_SYSTEM_STORE_LOCAL_MACHINE_GROUP_POLICY
//      or CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE
//      locations by prepending the computer name. For example, a remote
//      local machine store is accessed via "\\ComputerName\Trust" or
//      "ComputerName\Trust". A remote service store is accessed via
//      "\\ComputerName\ServiceName\Trust". The leading "\\" backslashes are
//      optional in the ComputerName.
//
//      If CERT_STORE_READONLY_FLAG is set, then, the registry is
//      RegOpenKey'ed with KEY_READ_ACCESS. Otherwise, the registry is
//      RegCreateKey'ed with KEY_ALL_ACCESS.
//
//      The "root" store is treated differently from the other system
//      stores. Before a certificate is added to or deleted from the "root"
//      store, a pop up message box is displayed. The certificate's subject,
//      issuer, serial number, time validity, sha1 and md5 thumbprints are
//      displayed. The user is given the option to do the add or delete.
//      If they don't allow the operation, LastError is set to E_ACCESSDENIED.
//
//  CERT_STORE_PROV_SYSTEM_REGISTRY_A
//  CERT_STORE_PROV_SYSTEM_REGISTRY_W
//  CERT_STORE_PROV_SYSTEM_REGISTRY
//  sz_CERT_STORE_PROV_SYSTEM_REGISTRY_W
//  sz_CERT_STORE_PROV_SYSTEM_REGISTRY
//      Opens the "System" store's default "Physical" store residing in the
//      registry. The upper word of the dwFlags
//      parameter is used to specify the location of the system store.
//
//      After opening the registry key associated with the system name,
//      the CERT_STORE_PROV_REG provider is called to complete the open.
//
//      The system store name is passed in pvPara. The name is UNICODE for the
//      "_W" provider and ASCII for the "_A" provider. For "_W": given,
//          LPCWSTR pwszSystemName; pvPara = (const void *) pwszSystemName;
//      For "_A": given,
//          LPCSTR pszSystemName; pvPara = (const void *) pszSystemName;
//
//      Note, the default (without "_A" or "_W") is UNICODE.
//
//      If CERT_SYSTEM_STORE_RELOCATE_FLAG is set in dwFlags, pvPara
//      points to a CERT_SYSTEM_STORE_RELOCATE_PARA data structure instead
//      of pointing to a null terminated UNICODE or ASCII string.
//
//      See above for details on prepending a ServiceName and/or ComputerName
//      to the store name.
//
//      If CERT_STORE_READONLY_FLAG is set, then, the registry is
//      RegOpenKey'ed with KEY_READ_ACCESS. Otherwise, the registry is
//      RegCreateKey'ed with KEY_ALL_ACCESS.
//
//      The "root" store is treated differently from the other system
//      stores. Before a certificate is added to or deleted from the "root"
//      store, a pop up message box is displayed. The certificate's subject,
//      issuer, serial number, time validity, sha1 and md5 thumbprints are
//      displayed. The user is given the option to do the add or delete.
//      If they don't allow the operation, LastError is set to E_ACCESSDENIED.
//
//  CERT_STORE_PROV_PHYSICAL_W
//  CERT_STORE_PROV_PHYSICAL
//  sz_CERT_STORE_PROV_PHYSICAL_W
//  sz_CERT_STORE_PROV_PHYSICAL
//      Opens the specified "Physical" store in the "System" store.
//
//      Both the system store and physical names are passed in pvPara. The
//      names are separated with an intervening "\". For example,
//      "Root\.Default". The string is UNICODE.
//
//      The system and physical store names can't contain any backslashes.
//
//      If CERT_SYSTEM_STORE_RELOCATE_FLAG is set in dwFlags, pvPara
//      points to a CERT_SYSTEM_STORE_RELOCATE_PARA data structure instead
//      of pointing to a null terminated UNICODE string.
//      The specified physical store is opened as relocated using pvPara's
//      hKeyBase.
//
//      For CERT_SYSTEM_STORE_SERVICES or CERT_SYSTEM_STORE_USERS,
//      the system and physical store names
//      must be prefixed with the ServiceName or UserName. For example,
//      "ServiceName\Root\.Default".
//
//      Physical stores on remote computers can be accessed for the
//      CERT_SYSTEM_STORE_LOCAL_MACHINE, CERT_SYSTEM_STORE_SERVICES,
//      CERT_SYSTEM_STORE_USERS, CERT_SYSTEM_STORE_LOCAL_MACHINE_GROUP_POLICY
//      or CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE
//      locations by prepending the computer name. For example, a remote
//      local machine store is accessed via "\\ComputerName\Root\.Default"
//      or "ComputerName\Root\.Default". A remote service store is
//      accessed via "\\ComputerName\ServiceName\Root\.Default". The
//      leading "\\" backslashes are optional in the ComputerName.
//
//  CERT_STORE_PROV_COLLECTION
//  sz_CERT_STORE_PROV_COLLECTION
//      Opens a store that is a collection of other stores. Stores are
//      added or removed to/from the collection via the CertAddStoreToCollection
//      and CertRemoveStoreFromCollection APIs.
//
//  CERT_STORE_PROV_SMART_CARD_W
//  CERT_STORE_PROV_SMART_CARD
//  sz_CERT_STORE_PROV_SMART_CARD_W
//  sz_CERT_STORE_PROV_SMART_CARD
//      Opens a store instantiated over a particular smart card storage.  pvPara
//      identifies where on the card the store is located and is of the
//      following format:
//
//                Card Name\Provider Name\Provider Type[\Container Name]
//
//      Container Name is optional and if NOT specified the Card Name is used
//      as the Container Name.  Future versions of the provider will support
//      instantiating the store over the entire card in which case just
//      Card Name ( or id ) will be sufficient.
//
//  Here's a list of the predefined provider types (implemented in
//  cryptnet.dll):
//
//  CERT_STORE_PROV_LDAP_W
//  CERT_STORE_PROV_LDAP
//  sz_CERT_STORE_PROV_LDAP_W
//  sz_CERT_STORE_PROV_LDAP
//      Opens a store over the results of the query specified by and LDAP
//      URL which is passed in via pvPara.  In order to do writes to the
//      store the URL must specify a BASE query, no filter and a single
//      attribute.
//
//--------------------------------------------------------------------------
WINCRYPT32API
_Must_inspect_result_
HCERTSTORE
WINAPI
CertOpenStore(
    _In_ LPCSTR lpszStoreProvider,
    _In_ DWORD dwEncodingType,
    _In_opt_ HCRYPTPROV_LEGACY hCryptProv,
    _In_ DWORD dwFlags,
    _In_opt_ const void *pvPara
    );

//+-------------------------------------------------------------------------
//  OID Installable Certificate Store Provider Data Structures
//--------------------------------------------------------------------------

// Handle returned by the store provider when opened.
typedef void *HCERTSTOREPROV;

// Store Provider OID function's pszFuncName.
#define CRYPT_OID_OPEN_STORE_PROV_FUNC   "CertDllOpenStoreProv"

// Note, the Store Provider OID function's dwEncodingType is always 0.

// The following information is returned by the provider when opened. Its
// zeroed with cbSize set before the provider is called. If the provider
// doesn't need to be called again after the open it doesn't need to
// make any updates to the CERT_STORE_PROV_INFO.
typedef struct _CERT_STORE_PROV_INFO {
    DWORD               cbSize;
    DWORD               cStoreProvFunc;
    void                **rgpvStoreProvFunc;
    HCERTSTOREPROV      hStoreProv;
    DWORD               dwStoreProvFlags;
    HCRYPTOIDFUNCADDR   hStoreProvFuncAddr2;
} CERT_STORE_PROV_INFO, *PCERT_STORE_PROV_INFO;

// Definition of the store provider's open function.
//
// *pStoreProvInfo has been zeroed before the call.
//
// Note, pStoreProvInfo->cStoreProvFunc should be set last.  Once set,
// all subsequent store calls, such as CertAddSerializedElementToStore will
// call the appropriate provider callback function.
typedef BOOL (WINAPI *PFN_CERT_DLL_OPEN_STORE_PROV_FUNC)(
    _In_ LPCSTR lpszStoreProvider,
    _In_ DWORD dwEncodingType,
    _In_opt_ HCRYPTPROV_LEGACY hCryptProv,
    _In_ DWORD dwFlags,
    _In_opt_ const void *pvPara,
    _In_ HCERTSTORE hCertStore,
    _Inout_ PCERT_STORE_PROV_INFO pStoreProvInfo
    );

// The open callback sets the following flag, if it maintains its
// contexts externally and not in the cached store.
#define CERT_STORE_PROV_EXTERNAL_FLAG           0x1

// The open callback sets the following flag for a successful delete.
// When set, the close callback isn't called.
#define CERT_STORE_PROV_DELETED_FLAG            0x2

// The open callback sets the following flag if it doesn't persist store
// changes.
#define CERT_STORE_PROV_NO_PERSIST_FLAG         0x4

// The open callback sets the following flag if the contexts are persisted
// to a system store.
#define CERT_STORE_PROV_SYSTEM_STORE_FLAG       0x8

// The open callback sets the following flag if the contexts are persisted
// to a LocalMachine system store.
#define CERT_STORE_PROV_LM_SYSTEM_STORE_FLAG    0x10

// The open callback sets the following flag if the contexts are persisted
// to a GroupPolicy system store.
#define CERT_STORE_PROV_GP_SYSTEM_STORE_FLAG    0x20

// The open callback sets the following flag if the contexts are from
// a Shared User physical store.
#define CERT_STORE_PROV_SHARED_USER_FLAG        0x40

// Indices into the store provider's array of callback functions.
//
// The provider can implement any subset of the following functions. It
// sets pStoreProvInfo->cStoreProvFunc to the last index + 1 and any
// preceding not implemented functions to NULL.
#define CERT_STORE_PROV_CLOSE_FUNC              0
#define CERT_STORE_PROV_READ_CERT_FUNC          1
#define CERT_STORE_PROV_WRITE_CERT_FUNC         2
#define CERT_STORE_PROV_DELETE_CERT_FUNC        3
#define CERT_STORE_PROV_SET_CERT_PROPERTY_FUNC  4
#define CERT_STORE_PROV_READ_CRL_FUNC           5
#define CERT_STORE_PROV_WRITE_CRL_FUNC          6
#define CERT_STORE_PROV_DELETE_CRL_FUNC         7
#define CERT_STORE_PROV_SET_CRL_PROPERTY_FUNC   8
#define CERT_STORE_PROV_READ_CTL_FUNC           9
#define CERT_STORE_PROV_WRITE_CTL_FUNC          10
#define CERT_STORE_PROV_DELETE_CTL_FUNC         11
#define CERT_STORE_PROV_SET_CTL_PROPERTY_FUNC   12
#define CERT_STORE_PROV_CONTROL_FUNC            13
#define CERT_STORE_PROV_FIND_CERT_FUNC          14
#define CERT_STORE_PROV_FREE_FIND_CERT_FUNC     15
#define CERT_STORE_PROV_GET_CERT_PROPERTY_FUNC  16
#define CERT_STORE_PROV_FIND_CRL_FUNC           17
#define CERT_STORE_PROV_FREE_FIND_CRL_FUNC      18
#define CERT_STORE_PROV_GET_CRL_PROPERTY_FUNC   19
#define CERT_STORE_PROV_FIND_CTL_FUNC           20
#define CERT_STORE_PROV_FREE_FIND_CTL_FUNC      21
#define CERT_STORE_PROV_GET_CTL_PROPERTY_FUNC   22


// Called by CertCloseStore when the store's reference count is
// decremented to 0.
typedef void (WINAPI *PFN_CERT_STORE_PROV_CLOSE)(
    _Inout_opt_ HCERTSTOREPROV hStoreProv,
    _In_ DWORD dwFlags
    );

// Currently not called directly by the store APIs. However, may be exported
// to support other providers based on it.
//
// Reads the provider's copy of the certificate context. If it exists,
// creates a new certificate context.
typedef _Success_(return != FALSE) BOOL (WINAPI *PFN_CERT_STORE_PROV_READ_CERT)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCERT_CONTEXT pStoreCertContext,
    _In_ DWORD dwFlags,
    _Outptr_ PCCERT_CONTEXT *ppProvCertContext
    );

#define CERT_STORE_PROV_WRITE_ADD_FLAG      0x1

// Called by CertAddEncodedCertificateToStore,
// CertAddCertificateContextToStore or CertAddSerializedElementToStore before
// adding to the store. The CERT_STORE_PROV_WRITE_ADD_FLAG is set. In
// addition to the encoded certificate, the added pCertContext might also
// have properties.
//
// Returns TRUE if its OK to update the the store.
typedef BOOL (WINAPI *PFN_CERT_STORE_PROV_WRITE_CERT)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ DWORD dwFlags
    );

// Called by CertDeleteCertificateFromStore before deleting from the
// store.
//
// Returns TRUE if its OK to delete from the store.
typedef BOOL (WINAPI *PFN_CERT_STORE_PROV_DELETE_CERT)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ DWORD dwFlags
    );

// Called by CertSetCertificateContextProperty before setting the
// certificate's property. Also called by CertGetCertificateContextProperty,
// when getting a hash property that needs to be created and then persisted
// via the set.
//
// Upon input, the property hasn't been set for the pCertContext parameter.
//
// Returns TRUE if its OK to set the property.
typedef BOOL (WINAPI *PFN_CERT_STORE_PROV_SET_CERT_PROPERTY)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ DWORD dwPropId,
    _In_ DWORD dwFlags,
    _In_opt_ const void *pvData
    );

// Currently not called directly by the store APIs. However, may be exported
// to support other providers based on it.
//
// Reads the provider's copy of the CRL context. If it exists,
// creates a new CRL context.
typedef _Success_(return != FALSE) BOOL (WINAPI *PFN_CERT_STORE_PROV_READ_CRL)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCRL_CONTEXT pStoreCrlContext,
    _In_ DWORD dwFlags,
    _Outptr_ PCCRL_CONTEXT *ppProvCrlContext
    );

// Called by CertAddEncodedCRLToStore,
// CertAddCRLContextToStore or CertAddSerializedElementToStore before
// adding to the store. The CERT_STORE_PROV_WRITE_ADD_FLAG is set. In
// addition to the encoded CRL, the added pCertContext might also
// have properties.
//
// Returns TRUE if its OK to update the the store.
typedef BOOL (WINAPI *PFN_CERT_STORE_PROV_WRITE_CRL)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCRL_CONTEXT pCrlContext,
    _In_ DWORD dwFlags
    );

// Called by CertDeleteCRLFromStore before deleting from the store.
//
// Returns TRUE if its OK to delete from the store.
typedef BOOL (WINAPI *PFN_CERT_STORE_PROV_DELETE_CRL)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCRL_CONTEXT pCrlContext,
    _In_ DWORD dwFlags
    );

// Called by CertSetCRLContextProperty before setting the
// CRL's property. Also called by CertGetCRLContextProperty,
// when getting a hash property that needs to be created and then persisted
// via the set.
//
// Upon input, the property hasn't been set for the pCrlContext parameter.
//
// Returns TRUE if its OK to set the property.
typedef BOOL (WINAPI *PFN_CERT_STORE_PROV_SET_CRL_PROPERTY)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCRL_CONTEXT pCrlContext,
    _In_ DWORD dwPropId,
    _In_ DWORD dwFlags,
    _In_opt_ const void *pvData
    );

// Currently not called directly by the store APIs. However, may be exported
// to support other providers based on it.
//
// Reads the provider's copy of the CTL context. If it exists,
// creates a new CTL context.
typedef _Success_(return != FALSE) BOOL (WINAPI *PFN_CERT_STORE_PROV_READ_CTL)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCTL_CONTEXT pStoreCtlContext,
    _In_ DWORD dwFlags,
    _Outptr_ PCCTL_CONTEXT *ppProvCtlContext
    );

// Called by CertAddEncodedCTLToStore,
// CertAddCTLContextToStore or CertAddSerializedElementToStore before
// adding to the store. The CERT_STORE_PROV_WRITE_ADD_FLAG is set. In
// addition to the encoded CTL, the added pCertContext might also
// have properties.
//
// Returns TRUE if its OK to update the the store.
typedef BOOL (WINAPI *PFN_CERT_STORE_PROV_WRITE_CTL)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCTL_CONTEXT pCtlContext,
    _In_ DWORD dwFlags
    );

// Called by CertDeleteCTLFromStore before deleting from the store.
//
// Returns TRUE if its OK to delete from the store.
typedef BOOL (WINAPI *PFN_CERT_STORE_PROV_DELETE_CTL)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCTL_CONTEXT pCtlContext,
    _In_ DWORD dwFlags
                                                     );

// Called by CertSetCTLContextProperty before setting the
// CTL's property. Also called by CertGetCTLContextProperty,
// when getting a hash property that needs to be created and then persisted
// via the set.
//
// Upon input, the property hasn't been set for the pCtlContext parameter.
//
// Returns TRUE if its OK to set the property.
typedef BOOL (WINAPI *PFN_CERT_STORE_PROV_SET_CTL_PROPERTY)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCTL_CONTEXT pCtlContext,
    _In_ DWORD dwPropId,
    _In_ DWORD dwFlags,
    _In_opt_ const void *pvData
    );

typedef BOOL (WINAPI *PFN_CERT_STORE_PROV_CONTROL)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ DWORD dwFlags,
    _In_ DWORD dwCtrlType,
    _In_opt_ void const *pvCtrlPara
    );

typedef struct _CERT_STORE_PROV_FIND_INFO {
    DWORD               cbSize;
    DWORD               dwMsgAndCertEncodingType;
    DWORD               dwFindFlags;
    DWORD               dwFindType;
    const void          *pvFindPara;
} CERT_STORE_PROV_FIND_INFO, *PCERT_STORE_PROV_FIND_INFO;
typedef const CERT_STORE_PROV_FIND_INFO CCERT_STORE_PROV_FIND_INFO,
*PCCERT_STORE_PROV_FIND_INFO;

typedef _Success_(return != FALSE) BOOL (WINAPI *PFN_CERT_STORE_PROV_FIND_CERT)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCERT_STORE_PROV_FIND_INFO pFindInfo,
    _In_ PCCERT_CONTEXT pPrevCertContext,
    _In_ DWORD dwFlags,
    _Inout_ void **ppvStoreProvFindInfo,
    _Outptr_ PCCERT_CONTEXT *ppProvCertContext
    );

typedef BOOL (WINAPI *PFN_CERT_STORE_PROV_FREE_FIND_CERT)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ void *pvStoreProvFindInfo,
    _In_ DWORD dwFlags
    );

typedef BOOL (WINAPI *PFN_CERT_STORE_PROV_GET_CERT_PROPERTY)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ DWORD dwPropId,
    _In_ DWORD dwFlags,
    _Out_writes_bytes_to_opt_(*pcbData, *pcbData) void *pvData,
    _Inout_ DWORD *pcbData
    );


typedef _Success_(return != FALSE) BOOL (WINAPI *PFN_CERT_STORE_PROV_FIND_CRL)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCERT_STORE_PROV_FIND_INFO pFindInfo,
    _In_ PCCRL_CONTEXT pPrevCrlContext,
    _In_ DWORD dwFlags,
    _Inout_ void **ppvStoreProvFindInfo,
    _Outptr_ PCCRL_CONTEXT *ppProvCrlContext
    );

typedef BOOL (WINAPI *PFN_CERT_STORE_PROV_FREE_FIND_CRL)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCRL_CONTEXT pCrlContext,
    _In_ void *pvStoreProvFindInfo,
    _In_ DWORD dwFlags
    );

typedef BOOL (WINAPI *PFN_CERT_STORE_PROV_GET_CRL_PROPERTY)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCRL_CONTEXT pCrlContext,
    _In_ DWORD dwPropId,
    _In_ DWORD dwFlags,
    _Out_writes_bytes_to_opt_(*pcbData, *pcbData) void *pvData,
    _Inout_ DWORD *pcbData
    );

typedef _Success_(return != FALSE) BOOL (WINAPI *PFN_CERT_STORE_PROV_FIND_CTL)(
    _In_ HCERTSTOREPROV hStoreProv,
    _In_ PCCERT_STORE_PROV_FIND_INFO pFindInfo,
    _In_ PCCTL_CONTEXT pPrevCtlContext,
    _In_ DWORD dwFlags,
    _Inout_ void **ppvStoreProvFindInfo,
    _Outptr_ PCCTL_CONTEXT *ppProvCtlContext
    );

typedef BOOL (WINAPI *PFN_CERT_STORE_PROV_FREE_FIND_CTL)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCTL_CONTEXT pCtlContext,
    _In_ void *pvStoreProvFindInfo,
    _In_ DWORD dwFlags
    );

typedef BOOL (WINAPI *PFN_CERT_STORE_PROV_GET_CTL_PROPERTY)(
    _Inout_ HCERTSTOREPROV hStoreProv,
    _In_ PCCTL_CONTEXT pCtlContext,
    _In_ DWORD dwPropId,
    _In_ DWORD dwFlags,
    _Out_writes_bytes_to_opt_(*pcbData, *pcbData) void *pvData,
    _Inout_ DWORD *pcbData
    );


//+-------------------------------------------------------------------------
//  Duplicate a cert store handle
//--------------------------------------------------------------------------
WINCRYPT32API
HCERTSTORE
WINAPI
CertDuplicateStore(
    _In_ HCERTSTORE hCertStore
    );

#define CERT_STORE_SAVE_AS_STORE        1
#define CERT_STORE_SAVE_AS_PKCS7        2
#define CERT_STORE_SAVE_AS_PKCS12       3

#define CERT_STORE_SAVE_TO_FILE         1
#define CERT_STORE_SAVE_TO_MEMORY       2
#define CERT_STORE_SAVE_TO_FILENAME_A   3
#define CERT_STORE_SAVE_TO_FILENAME_W   4
#define CERT_STORE_SAVE_TO_FILENAME     CERT_STORE_SAVE_TO_FILENAME_W

//+-------------------------------------------------------------------------
//  Save the cert store. Extended version with lots of options.
//
//  According to the dwSaveAs parameter, the store can be saved as a
//  serialized store (CERT_STORE_SAVE_AS_STORE) containing properties in
//  addition to encoded certificates, CRLs and CTLs or the store can be saved
//  as a PKCS #7 signed message (CERT_STORE_SAVE_AS_PKCS7) which doesn't
//  include the properties or CTLs.
//
//  Note, the CERT_KEY_CONTEXT_PROP_ID property (and its
//  CERT_KEY_PROV_HANDLE_PROP_ID or CERT_KEY_SPEC_PROP_ID) isn't saved into
//  a serialized store.
//
//  For CERT_STORE_SAVE_AS_PKCS7, the dwEncodingType specifies the message
//  encoding type. The dwEncodingType parameter isn't used for
//  CERT_STORE_SAVE_AS_STORE.
//
//  The dwFlags parameter currently isn't used and should be set to 0.
//
//  The dwSaveTo and pvSaveToPara parameters specify where to save the
//  store as follows:
//    CERT_STORE_SAVE_TO_FILE:
//      Saves to the specified file. The file's handle is passed in
//      pvSaveToPara. Given,
//          HANDLE hFile; pvSaveToPara = (void *) hFile;
//
//      For a successful save, the file pointer is positioned after the
//      last write.
//
//    CERT_STORE_SAVE_TO_MEMORY:
//      Saves to the specified memory blob. The pointer to
//      the memory blob is passed in pvSaveToPara. Given,
//          CRYPT_DATA_BLOB SaveBlob; pvSaveToPara = (void *) &SaveBlob;
//      Upon entry, the SaveBlob's pbData and cbData need to be initialized.
//      Upon return, cbData is updated with the actual length.
//      For a length only calculation, pbData should be set to NULL. If
//      pbData is non-NULL and cbData isn't large enough, FALSE is returned
//      with a last error of ERRROR_MORE_DATA.
//
//    CERT_STORE_SAVE_TO_FILENAME_A:
//    CERT_STORE_SAVE_TO_FILENAME_W:
//    CERT_STORE_SAVE_TO_FILENAME:
//      Opens the file and saves to it. The filename is passed in pvSaveToPara.
//      The filename is UNICODE for the "_W" option and ASCII for the "_A"
//      option. For "_W": given,
//          LPCWSTR pwszFilename; pvSaveToPara = (void *) pwszFilename;
//      For "_A": given,
//          LPCSTR pszFilename; pvSaveToPara = (void *) pszFilename;
//
//      Note, the default (without "_A" or "_W") is UNICODE.
//
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertSaveStore(
    _In_ HCERTSTORE hCertStore,
    _In_ DWORD dwEncodingType,
    _In_ DWORD dwSaveAs,
    _In_ DWORD dwSaveTo,
    _Inout_ void *pvSaveToPara,
    _In_ DWORD dwFlags
    );

//+-------------------------------------------------------------------------
//  Certificate Store close flags
//--------------------------------------------------------------------------
#define CERT_CLOSE_STORE_FORCE_FLAG         0x00000001
#define CERT_CLOSE_STORE_CHECK_FLAG         0x00000002

//+-------------------------------------------------------------------------
//  Close a cert store handle.
//
//  There needs to be a corresponding close for each open and duplicate.
//
//  Even on the final close, the cert store isn't freed until all of its
//  certificate and CRL contexts have also been freed.
//
//  On the final close, the hCryptProv passed to CertStoreOpen is
//  CryptReleaseContext'ed.
//
//  To force the closure of the store with all of its memory freed, set the
//  CERT_STORE_CLOSE_FORCE_FLAG. This flag should be set when the caller does
//  its own reference counting and wants everything to vanish.
//
//  To check if all the store's certificates and CRLs have been freed and that
//  this is the last CertCloseStore, set the CERT_CLOSE_STORE_CHECK_FLAG. If
//  set and certs, CRLs or stores still need to be freed/closed, FALSE is
//  returned with LastError set to CRYPT_E_PENDING_CLOSE. Note, for FALSE,
//  the store is still closed. This is a diagnostic flag.
//
//  LastError is preserved unless CERT_CLOSE_STORE_CHECK_FLAG is set and FALSE
//  is returned.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertCloseStore(
    _In_opt_ HCERTSTORE hCertStore,
    _In_ DWORD dwFlags
    );

//+-------------------------------------------------------------------------
//  Get the subject certificate context uniquely identified by its Issuer and
//  SerialNumber from the store.
//
//  If the certificate isn't found, NULL is returned. Otherwise, a pointer to
//  a read only CERT_CONTEXT is returned. CERT_CONTEXT must be freed by calling
//  CertFreeCertificateContext. CertDuplicateCertificateContext can be called to make a
//  duplicate.
//
//  The returned certificate might not be valid. Normally, it would be
//  verified when getting its issuer certificate (CertGetIssuerCertificateFromStore).
//--------------------------------------------------------------------------
WINCRYPT32API
PCCERT_CONTEXT
WINAPI
CertGetSubjectCertificateFromStore(
    _In_ HCERTSTORE hCertStore,
    _In_ DWORD dwCertEncodingType,
    _In_ PCERT_INFO pCertId           // Only the Issuer and SerialNumber
                                      // fields are used
    );

//+-------------------------------------------------------------------------
//  Enumerate the certificate contexts in the store.
//
//  If a certificate isn't found, NULL is returned.
//  Otherwise, a pointer to a read only CERT_CONTEXT is returned. CERT_CONTEXT
//  must be freed by calling CertFreeCertificateContext or is freed when passed as the
//  pPrevCertContext on a subsequent call. CertDuplicateCertificateContext
//  can be called to make a duplicate.
//
//  pPrevCertContext MUST BE NULL to enumerate the first
//  certificate in the store. Successive certificates are enumerated by setting
//  pPrevCertContext to the CERT_CONTEXT returned by a previous call.
//
//  NOTE: a NON-NULL pPrevCertContext is always CertFreeCertificateContext'ed by
//  this function, even for an error.
//--------------------------------------------------------------------------
WINCRYPT32API
PCCERT_CONTEXT
WINAPI
CertEnumCertificatesInStore(
    _In_ HCERTSTORE hCertStore,
    _In_opt_ PCCERT_CONTEXT pPrevCertContext
    );

//+-------------------------------------------------------------------------
//  Find the first or next certificate context in the store.
//
//  The certificate is found according to the dwFindType and its pvFindPara.
//  See below for a list of the find types and its parameters.
//
//  Currently dwFindFlags is only used for CERT_FIND_SUBJECT_ATTR,
//  CERT_FIND_ISSUER_ATTR or CERT_FIND_CTL_USAGE. Otherwise, must be set to 0.
//
//  Usage of dwCertEncodingType depends on the dwFindType.
//
//  If the first or next certificate isn't found, NULL is returned.
//  Otherwise, a pointer to a read only CERT_CONTEXT is returned. CERT_CONTEXT
//  must be freed by calling CertFreeCertificateContext or is freed when passed as the
//  pPrevCertContext on a subsequent call. CertDuplicateCertificateContext
//  can be called to make a duplicate.
//
//  pPrevCertContext MUST BE NULL on the first
//  call to find the certificate. To find the next certificate, the
//  pPrevCertContext is set to the CERT_CONTEXT returned by a previous call.
//
//  NOTE: a NON-NULL pPrevCertContext is always CertFreeCertificateContext'ed by
//  this function, even for an error.
//--------------------------------------------------------------------------
WINCRYPT32API
PCCERT_CONTEXT
WINAPI
CertFindCertificateInStore(
    _In_ HCERTSTORE hCertStore,
    _In_ DWORD dwCertEncodingType,
    _In_ DWORD dwFindFlags,
    _In_ DWORD dwFindType,
    _In_opt_ const void *pvFindPara,
    _In_opt_ PCCERT_CONTEXT pPrevCertContext
    );

//+-------------------------------------------------------------------------
// Certificate comparison functions
//--------------------------------------------------------------------------
#define CERT_COMPARE_MASK           0xFFFF
#define CERT_COMPARE_SHIFT          16
#define CERT_COMPARE_ANY            0
#define CERT_COMPARE_SHA1_HASH      1
#define CERT_COMPARE_NAME           2
#define CERT_COMPARE_ATTR           3
#define CERT_COMPARE_MD5_HASH       4
#define CERT_COMPARE_PROPERTY       5
#define CERT_COMPARE_PUBLIC_KEY     6
#define CERT_COMPARE_HASH           CERT_COMPARE_SHA1_HASH
#define CERT_COMPARE_NAME_STR_A     7
#define CERT_COMPARE_NAME_STR_W     8
#define CERT_COMPARE_KEY_SPEC       9
#define CERT_COMPARE_ENHKEY_USAGE   10
#define CERT_COMPARE_CTL_USAGE      CERT_COMPARE_ENHKEY_USAGE
#define CERT_COMPARE_SUBJECT_CERT   11
#define CERT_COMPARE_ISSUER_OF      12
#define CERT_COMPARE_EXISTING       13
#define CERT_COMPARE_SIGNATURE_HASH 14
#define CERT_COMPARE_KEY_IDENTIFIER 15
#define CERT_COMPARE_CERT_ID        16
#define CERT_COMPARE_CROSS_CERT_DIST_POINTS 17

#define CERT_COMPARE_PUBKEY_MD5_HASH 18

#define CERT_COMPARE_SUBJECT_INFO_ACCESS 19
#define CERT_COMPARE_HASH_STR       20
#define CERT_COMPARE_HAS_PRIVATE_KEY 21

#define CERT_COMPARE_SHA256_HASH    22
#define CERT_COMPARE_SHA1_SHA256_HASH 23
//+-------------------------------------------------------------------------
//  dwFindType
//
//  The dwFindType definition consists of two components:
//   - comparison function
//   - certificate information flag
//--------------------------------------------------------------------------
#define CERT_FIND_ANY           (CERT_COMPARE_ANY << CERT_COMPARE_SHIFT)
#define CERT_FIND_SHA1_HASH     (CERT_COMPARE_SHA1_HASH << CERT_COMPARE_SHIFT)
#define CERT_FIND_SHA256_HASH   (CERT_COMPARE_SHA256_HASH << CERT_COMPARE_SHIFT)
#define CERT_FIND_SHA1_SHA256_HASH   (CERT_COMPARE_SHA1_SHA256_HASH << CERT_COMPARE_SHIFT)
#define CERT_FIND_MD5_HASH      (CERT_COMPARE_MD5_HASH << CERT_COMPARE_SHIFT)
#define CERT_FIND_SIGNATURE_HASH (CERT_COMPARE_SIGNATURE_HASH << CERT_COMPARE_SHIFT)
#define CERT_FIND_KEY_IDENTIFIER (CERT_COMPARE_KEY_IDENTIFIER << CERT_COMPARE_SHIFT)
#define CERT_FIND_HASH          CERT_FIND_SHA1_HASH
#define CERT_FIND_PROPERTY      (CERT_COMPARE_PROPERTY << CERT_COMPARE_SHIFT)
#define CERT_FIND_PUBLIC_KEY    (CERT_COMPARE_PUBLIC_KEY << CERT_COMPARE_SHIFT)
#define CERT_FIND_SUBJECT_NAME  (CERT_COMPARE_NAME << CERT_COMPARE_SHIFT | \
                                 CERT_INFO_SUBJECT_FLAG)
#define CERT_FIND_SUBJECT_ATTR  (CERT_COMPARE_ATTR << CERT_COMPARE_SHIFT | \
                                 CERT_INFO_SUBJECT_FLAG)
#define CERT_FIND_ISSUER_NAME   (CERT_COMPARE_NAME << CERT_COMPARE_SHIFT | \
                                 CERT_INFO_ISSUER_FLAG)
#define CERT_FIND_ISSUER_ATTR   (CERT_COMPARE_ATTR << CERT_COMPARE_SHIFT | \
                                 CERT_INFO_ISSUER_FLAG)
#define CERT_FIND_SUBJECT_STR_A (CERT_COMPARE_NAME_STR_A << CERT_COMPARE_SHIFT | \
                                 CERT_INFO_SUBJECT_FLAG)
#define CERT_FIND_SUBJECT_STR_W (CERT_COMPARE_NAME_STR_W << CERT_COMPARE_SHIFT | \
                                 CERT_INFO_SUBJECT_FLAG)
#define CERT_FIND_SUBJECT_STR   CERT_FIND_SUBJECT_STR_W
#define CERT_FIND_ISSUER_STR_A  (CERT_COMPARE_NAME_STR_A << CERT_COMPARE_SHIFT | \
                                 CERT_INFO_ISSUER_FLAG)
#define CERT_FIND_ISSUER_STR_W  (CERT_COMPARE_NAME_STR_W << CERT_COMPARE_SHIFT | \
                                 CERT_INFO_ISSUER_FLAG)
#define CERT_FIND_ISSUER_STR    CERT_FIND_ISSUER_STR_W
#define CERT_FIND_KEY_SPEC      (CERT_COMPARE_KEY_SPEC << CERT_COMPARE_SHIFT)
#define CERT_FIND_ENHKEY_USAGE  (CERT_COMPARE_ENHKEY_USAGE << CERT_COMPARE_SHIFT)
#define CERT_FIND_CTL_USAGE     CERT_FIND_ENHKEY_USAGE

#define CERT_FIND_SUBJECT_CERT  (CERT_COMPARE_SUBJECT_CERT << CERT_COMPARE_SHIFT)
#define CERT_FIND_ISSUER_OF     (CERT_COMPARE_ISSUER_OF << CERT_COMPARE_SHIFT)
#define CERT_FIND_EXISTING      (CERT_COMPARE_EXISTING << CERT_COMPARE_SHIFT)
#define CERT_FIND_CERT_ID       (CERT_COMPARE_CERT_ID << CERT_COMPARE_SHIFT)
#define CERT_FIND_CROSS_CERT_DIST_POINTS \
                    (CERT_COMPARE_CROSS_CERT_DIST_POINTS << CERT_COMPARE_SHIFT)


#define CERT_FIND_PUBKEY_MD5_HASH \
                    (CERT_COMPARE_PUBKEY_MD5_HASH << CERT_COMPARE_SHIFT)

#define CERT_FIND_SUBJECT_INFO_ACCESS \
                    (CERT_COMPARE_SUBJECT_INFO_ACCESS << CERT_COMPARE_SHIFT)

#define CERT_FIND_HASH_STR      (CERT_COMPARE_HASH_STR << CERT_COMPARE_SHIFT)
#define CERT_FIND_HAS_PRIVATE_KEY (CERT_COMPARE_HAS_PRIVATE_KEY << CERT_COMPARE_SHIFT)

//+-------------------------------------------------------------------------
//  CERT_FIND_ANY
//
//  Find any certificate.
//
//  pvFindPara isn't used.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_FIND_HASH
//  CERT_FIND_SHA1_HASH
//  CERT_FIND_SHA256_HASH
//  CERT_FIND_SHA1_SHA256_HASH
//
//  Find a certificate with the specified hash. 
//
//  For the SHA1_SHA256 case, the hash property is a concatenation 
//  of both the SHA1 and SHA256 hash for 20+32 bytes. SHA1_SHA256 is applicable 
//  to the CERT_SYSTEM_STORE_DEFER_READ_FLAG where we first find in the registry 
//  via the SHA1 thumbprint name and then do a SHA256 confirmation.
//
//  pvFindPara points to a CRYPT_HASH_BLOB.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_FIND_KEY_IDENTIFIER
//
//  Find a certificate with the specified KeyIdentifier. Gets the
//  CERT_KEY_IDENTIFIER_PROP_ID property and compares with the input
//  CRYPT_HASH_BLOB.
//
//  pvFindPara points to a CRYPT_HASH_BLOB.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_FIND_PROPERTY
//
//  Find a certificate having the specified property.
//
//  pvFindPara points to a DWORD containing the PROP_ID
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_FIND_PUBLIC_KEY
//
//  Find a certificate matching the specified public key.
//
//  pvFindPara points to a CERT_PUBLIC_KEY_INFO containing the public key
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_FIND_SUBJECT_NAME
//  CERT_FIND_ISSUER_NAME
//
//  Find a certificate with the specified subject/issuer name. Does an exact
//  match of the entire name.
//
//  Restricts search to certificates matching the dwCertEncodingType.
//
//  pvFindPara points to a CERT_NAME_BLOB.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_FIND_SUBJECT_ATTR
//  CERT_FIND_ISSUER_ATTR
//
//  Find a certificate with the specified subject/issuer attributes.
//
//  Compares the attributes in the subject/issuer name with the
//  Relative Distinguished Name's (CERT_RDN) array of attributes specified in
//  pvFindPara. The comparison iterates through the CERT_RDN attributes and looks
//  for an attribute match in any of the subject/issuer's RDNs.
//
//  The CERT_RDN_ATTR fields can have the following special values:
//    pszObjId == NULL              - ignore the attribute object identifier
//    dwValueType == RDN_ANY_TYPE   - ignore the value type
//    Value.pbData == NULL          - match any value
//
//  CERT_CASE_INSENSITIVE_IS_RDN_ATTRS_FLAG should be set in dwFindFlags to do
//  a case insensitive match. Otherwise, defaults to an exact, case sensitive
//  match.
//
//  CERT_UNICODE_IS_RDN_ATTRS_FLAG should be set in dwFindFlags if the RDN was
//  initialized with unicode strings as for
//  CryptEncodeObject(X509_UNICODE_NAME).
//
//  Restricts search to certificates matching the dwCertEncodingType.
//
//  pvFindPara points to a CERT_RDN (defined in wincert.h).
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_FIND_SUBJECT_STR_A
//  CERT_FIND_SUBJECT_STR_W | CERT_FIND_SUBJECT_STR
//  CERT_FIND_ISSUER_STR_A
//  CERT_FIND_ISSUER_STR_W  | CERT_FIND_ISSUER_STR
//
//  Find a certificate containing the specified subject/issuer name string.
//
//  First, the certificate's subject/issuer is converted to a name string
//  via CertNameToStrA/CertNameToStrW(CERT_SIMPLE_NAME_STR). Then, a
//  case insensitive substring within string match is performed.
//
//  Restricts search to certificates matching the dwCertEncodingType.
//
//  For *_STR_A, pvFindPara points to a null terminated character string.
//  For *_STR_W, pvFindPara points to a null terminated wide character string.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_FIND_KEY_SPEC
//
//  Find a certificate having a CERT_KEY_SPEC_PROP_ID property matching
//  the specified KeySpec.
//
//  pvFindPara points to a DWORD containing the KeySpec.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_FIND_ENHKEY_USAGE
//
//  Find a certificate having the szOID_ENHANCED_KEY_USAGE extension or
//  the CERT_ENHKEY_USAGE_PROP_ID and matching the specified pszUsageIdentifers.
//
//  pvFindPara points to a CERT_ENHKEY_USAGE data structure. If pvFindPara
//  is NULL or CERT_ENHKEY_USAGE's cUsageIdentifier is 0, then, matches any
//  certificate having enhanced key usage.
//
//  If the CERT_FIND_VALID_ENHKEY_USAGE_FLAG is set, then, only does a match
//  for certificates that are valid for the specified usages. By default,
//  the ceriticate must be valid for all usages. CERT_FIND_OR_ENHKEY_USAGE_FLAG
//  can be set, if the certificate only needs to be valid for one of the
//  specified usages. Note, CertGetValidUsages() is called to get the
//  certificate's list of valid usages. Only the CERT_FIND_OR_ENHKEY_USAGE_FLAG
//  is applicable when this flag is set.
//
//  The CERT_FIND_OPTIONAL_ENHKEY_USAGE_FLAG can be set in dwFindFlags to
//  also match a certificate without either the extension or property.
//
//  If CERT_FIND_NO_ENHKEY_USAGE_FLAG is set in dwFindFlags, finds
//  certificates without the key usage extension or property. Setting this
//  flag takes precedence over pvFindPara being NULL.
//
//  If the CERT_FIND_EXT_ONLY_ENHKEY_USAGE_FLAG is set, then, only does a match
//  using the extension. If pvFindPara is NULL or cUsageIdentifier is set to
//  0, finds certificates having the extension. If
//  CERT_FIND_OPTIONAL_ENHKEY_USAGE_FLAG is set, also matches a certificate
//  without the extension. If CERT_FIND_NO_ENHKEY_USAGE_FLAG is set, finds
//  certificates without the extension.
//
//  If the CERT_FIND_PROP_ONLY_ENHKEY_USAGE_FLAG is set, then, only does a match
//  using the property. If pvFindPara is NULL or cUsageIdentifier is set to
//  0, finds certificates having the property. If
//  CERT_FIND_OPTIONAL_ENHKEY_USAGE_FLAG is set, also matches a certificate
//  without the property. If CERT_FIND_NO_ENHKEY_USAGE_FLAG is set, finds
//  certificates without the property.
//
//  If CERT_FIND_OR_ENHKEY_USAGE_FLAG is set, does an "OR" match of any of
//  the specified pszUsageIdentifiers. If not set, then, does an "AND" match
//  of all of the specified pszUsageIdentifiers.
//--------------------------------------------------------------------------

#define CERT_FIND_OPTIONAL_ENHKEY_USAGE_FLAG  0x1
#define CERT_FIND_EXT_ONLY_ENHKEY_USAGE_FLAG  0x2
#define CERT_FIND_PROP_ONLY_ENHKEY_USAGE_FLAG 0x4
#define CERT_FIND_NO_ENHKEY_USAGE_FLAG        0x8
#define CERT_FIND_OR_ENHKEY_USAGE_FLAG        0x10
#define CERT_FIND_VALID_ENHKEY_USAGE_FLAG     0x20

#define CERT_FIND_OPTIONAL_CTL_USAGE_FLAG   CERT_FIND_OPTIONAL_ENHKEY_USAGE_FLAG

#define CERT_FIND_EXT_ONLY_CTL_USAGE_FLAG \
        CERT_FIND_EXT_ONLY_ENHKEY_USAGE_FLAG

#define CERT_FIND_PROP_ONLY_CTL_USAGE_FLAG \
        CERT_FIND_PROP_ONLY_ENHKEY_USAGE_FLAG

#define CERT_FIND_NO_CTL_USAGE_FLAG         CERT_FIND_NO_ENHKEY_USAGE_FLAG
#define CERT_FIND_OR_CTL_USAGE_FLAG         CERT_FIND_OR_ENHKEY_USAGE_FLAG
#define CERT_FIND_VALID_CTL_USAGE_FLAG      CERT_FIND_VALID_ENHKEY_USAGE_FLAG

//+-------------------------------------------------------------------------
//  CERT_FIND_CERT_ID
//
//  Find a certificate with the specified CERT_ID.
//
//  pvFindPara points to a CERT_ID.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_FIND_CROSS_CERT_DIST_POINTS
//
//  Find a certificate having either a cross certificate distribution
//  point extension or property.
//
//  pvFindPara isn't used.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_FIND_SUBJECT_INFO_ACCESS
//
//  Find a certificate having either a SubjectInfoAccess extension or
//  property.
//
//  pvFindPara isn't used.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_FIND_HASH_STR
//
//  Find a certificate with the specified hash.
//
//  pvFindPara points to a null terminated wide character string, containing
//  40 hexadecimal digits that CryptStringToBinary(CRYPT_STRING_HEXRAW) can
//  convert to a 20 byte SHA1 CRYPT_HASH_BLOB.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  Get the certificate context from the store for the first or next issuer
//  of the specified subject certificate. Perform the enabled
//  verification checks on the subject. (Note, the checks are on the subject
//  using the returned issuer certificate.)
//
//  If the first or next issuer certificate isn't found, NULL is returned.
//  Otherwise, a pointer to a read only CERT_CONTEXT is returned. CERT_CONTEXT
//  must be freed by calling CertFreeCertificateContext or is freed when passed as the
//  pPrevIssuerContext on a subsequent call. CertDuplicateCertificateContext
//  can be called to make a duplicate.
//
//  For a self signed subject certificate, NULL is returned with LastError set
//  to CERT_STORE_SELF_SIGNED. The enabled verification checks are still done.
//
//  The pSubjectContext may have been obtained from this store, another store
//  or created by the caller application. When created by the caller, the
//  CertCreateCertificateContext function must have been called.
//
//  An issuer may have multiple certificates. This may occur when the validity
//  period is about to change. pPrevIssuerContext MUST BE NULL on the first
//  call to get the issuer. To get the next certificate for the issuer, the
//  pPrevIssuerContext is set to the CERT_CONTEXT returned by a previous call.
//
//  NOTE: a NON-NULL pPrevIssuerContext is always CertFreeCertificateContext'ed by
//  this function, even for an error.
//
//  The following flags can be set in *pdwFlags to enable verification checks
//  on the subject certificate context:
//      CERT_STORE_SIGNATURE_FLAG     - use the public key in the returned
//                                      issuer certificate to verify the
//                                      signature on the subject certificate.
//                                      Note, if pSubjectContext->hCertStore ==
//                                      hCertStore, the store provider might
//                                      be able to eliminate a redo of
//                                      the signature verify.
//      CERT_STORE_TIME_VALIDITY_FLAG - get the current time and verify that
//                                      its within the subject certificate's
//                                      validity period
//      CERT_STORE_REVOCATION_FLAG    - check if the subject certificate is on
//                                      the issuer's revocation list
//
//  If an enabled verification check fails, then, its flag is set upon return.
//  If CERT_STORE_REVOCATION_FLAG was enabled and the issuer doesn't have a
//  CRL in the store, then, CERT_STORE_NO_CRL_FLAG is set in addition to
//  the CERT_STORE_REVOCATION_FLAG.
//
//  If CERT_STORE_SIGNATURE_FLAG or CERT_STORE_REVOCATION_FLAG is set, then,
//  CERT_STORE_NO_ISSUER_FLAG is set if it doesn't have an issuer certificate
//  in the store.
//
//  For a verification check failure, a pointer to the issuer's CERT_CONTEXT
//  is still returned and SetLastError isn't updated.
//--------------------------------------------------------------------------
WINCRYPT32API
PCCERT_CONTEXT
WINAPI
CertGetIssuerCertificateFromStore(
    _In_ HCERTSTORE hCertStore,
    _In_ PCCERT_CONTEXT pSubjectContext,
    _In_opt_ PCCERT_CONTEXT pPrevIssuerContext,
    _Inout_ DWORD *pdwFlags
    );

//+-------------------------------------------------------------------------
//  Perform the enabled verification checks on the subject certificate
//  using the issuer. Same checks and flags definitions as for the above
//  CertGetIssuerCertificateFromStore.
//
//  If you are only checking CERT_STORE_TIME_VALIDITY_FLAG, then, the
//  issuer can be NULL.
//
//  For a verification check failure, SUCCESS is still returned.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertVerifySubjectCertificateContext(
    _In_ PCCERT_CONTEXT pSubject,
    _In_opt_ PCCERT_CONTEXT pIssuer,
    _Inout_ DWORD *pdwFlags
    );

//+-------------------------------------------------------------------------
//  Duplicate a certificate context
//--------------------------------------------------------------------------
WINCRYPT32API
PCCERT_CONTEXT
WINAPI
CertDuplicateCertificateContext(
    _In_opt_ PCCERT_CONTEXT pCertContext
    );

//+-------------------------------------------------------------------------
//  Create a certificate context from the encoded certificate. The created
//  context isn't put in a store.
//
//  Makes a copy of the encoded certificate in the created context.
//
//  If unable to decode and create the certificate context, NULL is returned.
//  Otherwise, a pointer to a read only CERT_CONTEXT is returned.
//  CERT_CONTEXT must be freed by calling CertFreeCertificateContext.
//  CertDuplicateCertificateContext can be called to make a duplicate.
//
//  CertSetCertificateContextProperty and CertGetCertificateContextProperty can be called
//  to store properties for the certificate.
//--------------------------------------------------------------------------
WINCRYPT32API
PCCERT_CONTEXT
WINAPI
CertCreateCertificateContext(
    _In_ DWORD dwCertEncodingType,
    _In_reads_bytes_(cbCertEncoded) const BYTE *pbCertEncoded,
    _In_ DWORD cbCertEncoded
    );

//+-------------------------------------------------------------------------
//  Free a certificate context
//
//  There needs to be a corresponding free for each context obtained by a
//  get, find, duplicate or create.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertFreeCertificateContext(
    _In_opt_ PCCERT_CONTEXT pCertContext
    );

//+-------------------------------------------------------------------------
//  Set the property for the specified certificate context.
//
//  The type definition for pvData depends on the dwPropId value. There are
//  five predefined types:
//      CERT_KEY_PROV_HANDLE_PROP_ID - a HCRYPTPROV for the certificate's
//      private key is passed in pvData. Updates the hCryptProv field
//      of the CERT_KEY_CONTEXT_PROP_ID. If the CERT_KEY_CONTEXT_PROP_ID
//      doesn't exist, its created with all the other fields zeroed out. If
//      CERT_STORE_NO_CRYPT_RELEASE_FLAG isn't set, HCRYPTPROV is implicitly
//      released when either the property is set to NULL or on the final
//      free of the CertContext.
//
//      CERT_NCRYPT_KEY_HANDLE_PROP_ID - a NCRYPT_KEY_HANDLE for the
//      certificate's private key is passed in pvData. The dwKeySpec is
//      set to CERT_NCRYPT_KEY_SPEC.
//
//      CERT_HCRYPTPROV_OR_NCRYPT_KEY_HANDLE_PROP_ID - a
//      HCRYPTPROV_OR_NCRYPT_KEY_HANDLE for the certificates's private
//      key is passed in pvData.  NCryptIsKeyHandle()
//      is called to determine if this is a CNG NCRYPT_KEY_HANDLE.
//      For a NCRYPT_KEY_HANDLE does a CERT_NCRYPT_KEY_HANDLE_PROP_ID set.
//      Otherwise, does a CERT_KEY_PROV_HANDLE_PROP_ID set.
//
//      CERT_KEY_PROV_INFO_PROP_ID - a PCRYPT_KEY_PROV_INFO for the certificate's
//      private key is passed in pvData.
//
//      CERT_SHA1_HASH_PROP_ID       -
//      CERT_MD5_HASH_PROP_ID        -
//      CERT_SIGNATURE_HASH_PROP_ID  - normally, a hash property is implicitly
//      set by doing a CertGetCertificateContextProperty. pvData points to a
//      CRYPT_HASH_BLOB.
//
//      CERT_KEY_CONTEXT_PROP_ID - a PCERT_KEY_CONTEXT for the certificate's
//      private key is passed in pvData. The CERT_KEY_CONTEXT contains both the
//      hCryptProv and dwKeySpec for the private key. A dwKeySpec of
//      CERT_NCRYPT_KEY_SPEC selects the hNCryptKey choice.
//      See the CERT_KEY_PROV_HANDLE_PROP_ID for more information about
//      the hCryptProv field and dwFlags settings. Note, more fields may
//      be added for this property. The cbSize field value will be adjusted
//      accordingly.
//
//      CERT_KEY_SPEC_PROP_ID - the dwKeySpec for the private key. pvData
//      points to a DWORD containing the KeySpec
//
//      CERT_ENHKEY_USAGE_PROP_ID - enhanced key usage definition for the
//      certificate. pvData points to a CRYPT_DATA_BLOB containing an
//      ASN.1 encoded CERT_ENHKEY_USAGE (encoded via
//      CryptEncodeObject(X509_ENHANCED_KEY_USAGE).
//
//      CERT_NEXT_UPDATE_LOCATION_PROP_ID - location of the next update.
//      Currently only applicable to CTLs. pvData points to a CRYPT_DATA_BLOB
//      containing an ASN.1 encoded CERT_ALT_NAME_INFO (encoded via
//      CryptEncodeObject(X509_ALTERNATE_NAME)).
//
//      CERT_FRIENDLY_NAME_PROP_ID - friendly name for the cert, CRL or CTL.
//      pvData points to a CRYPT_DATA_BLOB. pbData is a pointer to a NULL
//      terminated unicode, wide character string.
//      cbData = (wcslen((LPWSTR) pbData) + 1) * sizeof(WCHAR).
//
//      CERT_DESCRIPTION_PROP_ID - description for the cert, CRL or CTL.
//      pvData points to a CRYPT_DATA_BLOB. pbData is a pointer to a NULL
//      terminated unicode, wide character string.
//      cbData = (wcslen((LPWSTR) pbData) + 1) * sizeof(WCHAR).
//
//      CERT_ARCHIVED_PROP_ID - when this property is set, the certificate
//      is skipped during enumeration. Note, certificates having this property
//      are still found for explicit finds, such as, finding a certificate
//      with a specific hash or finding a certificate having a specific issuer
//      and serial number. pvData points to a CRYPT_DATA_BLOB. This blob
//      can be NULL (pbData = NULL, cbData = 0).
//
//      CERT_PUBKEY_ALG_PARA_PROP_ID - for public keys supporting
//      algorithm parameter inheritance. pvData points to a CRYPT_OBJID_BLOB
//      containing the ASN.1 encoded PublicKey Algorithm Parameters. For
//      DSS this would be the parameters encoded via
//      CryptEncodeObject(X509_DSS_PARAMETERS). This property may be set
//      by CryptVerifyCertificateSignatureEx().
//
//      CERT_CROSS_CERT_DIST_POINTS_PROP_ID - location of the cross certs.
//      Currently only applicable to certs. pvData points to a CRYPT_DATA_BLOB
//      containing an ASN.1 encoded CROSS_CERT_DIST_POINTS_INFO (encoded via
//      CryptEncodeObject(X509_CROSS_CERT_DIST_POINTS)).
//
//      CERT_ENROLLMENT_PROP_ID - enrollment information of the pending request.
//      It contains RequestID, CADNSName, CAName, and FriendlyName.
//      The data format is defined as: the first 4 bytes - pending request ID,
//      next 4 bytes - CADNSName size in characters including null-terminator
//      followed by CADNSName string with null-terminator,
//      next 4 bytes - CAName size in characters including null-terminator
//      followed by CAName string with null-terminator,
//      next 4 bytes - FriendlyName size in characters including null-terminator
//      followed by FriendlyName string with null-terminator.
//
//      CERT_DATE_STAMP_PROP_ID - contains the time when added to the store
//      by an admin tool. pvData points to a CRYPT_DATA_BLOB containing
//      the FILETIME.
//
//      CERT_RENEWAL_PROP_ID - contains the hash of renewed certificate
//
//      CERT_OCSP_RESPONSE_PROP_ID - contains the encoded OCSP response.
//      CryptDecodeObject/CryptEncodeObject using
//      lpszStructType = OCSP_RESPONSE.
//      pvData points to a CRYPT_DATA_BLOB containing the encoded OCSP response.
//      If this property is present, CertVerifyRevocation() will first attempt
//      to use before doing an URL retrieval.
//
//      CERT_SOURCE_LOCATION_PROP_ID - contains source location of the CRL or
//      OCSP. pvData points to a CRYPT_DATA_BLOB. pbData is a pointer to a NULL
//      terminated unicode, wide character string. Where,
//      cbData = (wcslen((LPWSTR) pbData) + 1) * sizeof(WCHAR).
//
//      CERT_SOURCE_URL_PROP_ID - contains URL for the CRL or OCSP. pvData
//      is the same as for CERT_SOURCE_LOCATION_PROP_ID.
//
//      CERT_CEP_PROP_ID - contains Version, PropertyFlags, AuthType,
//      UrlFlags and CESAuthType, followed by the CEPUrl, CEPId, CESUrl and
//      RequestId strings
//      The data format is defined as: the first 4 bytes - property version,
//      next 4 bytes - Property Flags
//      next 4 bytes - Authentication Type
//      next 4 bytes - Url Flags
//      next 4 bytes - CES Authentication Type
//      followed by Url string with null-terminator,
//      followed by Id string with null-terminator,
//      followed by CES Url string with null-terminator,
//      followed by RequestId string with null-terminator.
//      a single null-terminator indicates no string is present.
//
//      CERT_KEY_REPAIR_ATTEMPTED_PROP_ID - contains the time when repair of
//	a missing CERT_KEY_PROV_INFO_PROP_ID property was attempted and failed.
//      pvData points to a CRYPT_DATA_BLOB containing the FILETIME.
//
//  For all the other PROP_IDs: an encoded PCRYPT_DATA_BLOB is passed in pvData.
//
//  If the property already exists, then, the old value is deleted and silently
//  replaced. Setting, pvData to NULL, deletes the property.
//
//  CERT_SET_PROPERTY_IGNORE_PERSIST_ERROR_FLAG can be set to ignore any
//  provider write errors and always update the cached context's property.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertSetCertificateContextProperty(
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ DWORD dwPropId,
    _In_ DWORD dwFlags,
    _In_opt_ const void *pvData
    );

// Set this flag to ignore any store provider write errors and always update
// the cached context's property
#define CERT_SET_PROPERTY_IGNORE_PERSIST_ERROR_FLAG     0x80000000

// Set this flag to inhibit the persisting of this property
#define CERT_SET_PROPERTY_INHIBIT_PERSIST_FLAG          0x40000000

//+-------------------------------------------------------------------------
//  Get the property for the specified certificate context.
//
//  For CERT_KEY_PROV_HANDLE_PROP_ID, pvData points to a HCRYPTPROV.
//  The CERT_NCRYPT_KEY_SPEC NCRYPT_KEY_HANDLE choice isn't returned.
//
//  For CERT_NCRYPT_KEY_HANDLE_PROP_ID, pvData points to a NCRYPT_KEY_HANDLE.
//  Only returned for the CERT_NCRYPT_KEY_SPEC choice.
//
//  For CERT_HCRYPTPROV_OR_NCRYPT_KEY_HANDLE_PROP_ID, pvData points to a
//  HCRYPTPROV_OR_NCRYPT_KEY_HANDLE. Returns either the HCRYPTPROV or
//  NCRYPT_KEY_HANDLE choice.
//
//  For CERT_KEY_PROV_INFO_PROP_ID, pvData points to a CRYPT_KEY_PROV_INFO structure.
//  Elements pointed to by fields in the pvData structure follow the
//  structure. Therefore, *pcbData may exceed the size of the structure.
//
//  For CERT_KEY_CONTEXT_PROP_ID, pvData points to a CERT_KEY_CONTEXT structure.
//
//  For CERT_KEY_SPEC_PROP_ID, pvData points to a DWORD containing the KeySpec.
//  If the CERT_KEY_CONTEXT_PROP_ID exists, the KeySpec is obtained from there.
//  Otherwise, if the CERT_KEY_PROV_INFO_PROP_ID exists, its the source
//  of the KeySpec. CERT_NCRYPT_KEY_SPEC is returned if the
//  CERT_NCRYPT_KEY_HANDLE_PROP_ID has been set.
//
//  For CERT_SHA1_HASH_PROP_ID or CERT_MD5_HASH_PROP_ID, if the hash
//  doesn't already exist, then, its computed via CryptHashCertificate()
//  and then set. pvData points to the computed hash. Normally, the length
//  is 20 bytes for SHA and 16 for MD5.
//
//  For CERT_SIGNATURE_HASH_PROP_ID, if the hash
//  doesn't already exist, then, its computed via CryptHashToBeSigned()
//  and then set. pvData points to the computed hash. Normally, the length
//  is 20 bytes for SHA and 16 for MD5.
//
//  For CERT_ACCESS_STATE_PROP_ID, pvData points to a DWORD containing the
//  access state flags. The appropriate CERT_ACCESS_STATE_*_FLAG's are set
//  in the returned DWORD. See the CERT_ACCESS_STATE_*_FLAG definitions
//  above. Note, this property is read only. It can't be set.
//
//  For CERT_KEY_IDENTIFIER_PROP_ID, if property doesn't already exist,
//  first searches for the szOID_SUBJECT_KEY_IDENTIFIER extension. Next,
//  does SHA1 hash of the certficate's SubjectPublicKeyInfo. pvData
//  points to the key identifier bytes. Normally, the length is 20 bytes.
//
//  For CERT_PUBKEY_ALG_PARA_PROP_ID, pvPara points to the ASN.1 encoded
//  PublicKey Algorithm Parameters. This property will only be set
//  for public keys supporting algorithm parameter inheritance and when the
//  parameters have been omitted from the encoded and signed certificate.
//
//  For CERT_DATE_STAMP_PROP_ID, pvPara points to a FILETIME updated by
//  an admin tool to indicate when the certificate was added to the store.
//
//  For CERT_OCSP_RESPONSE_PROP_ID, pvPara points to an encoded OCSP response.
//
//  For CERT_SOURCE_LOCATION_PROP_ID and CERT_SOURCE_URL_PROP_ID,
//  pvPara points to a NULL terminated unicode, wide character string.
//
//  For all other PROP_IDs, pvData points to an encoded array of bytes.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertGetCertificateContextProperty(
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ DWORD dwPropId,
    _Out_writes_bytes_to_opt_(*pcbData, *pcbData) void *pvData,
    _Inout_ DWORD *pcbData
    );

//+-------------------------------------------------------------------------
//  Enumerate the properties for the specified certificate context.
//
//  To get the first property, set dwPropId to 0. The ID of the first
//  property is returned. To get the next property, set dwPropId to the
//  ID returned by the last call. To enumerate all the properties continue
//  until 0 is returned.
//
//  CertGetCertificateContextProperty is called to get the property's data.
//
//  Note, since, the CERT_KEY_PROV_HANDLE_PROP_ID and CERT_KEY_SPEC_PROP_ID
//  properties are stored as fields in the CERT_KEY_CONTEXT_PROP_ID
//  property, they aren't enumerated individually.
//--------------------------------------------------------------------------
WINCRYPT32API
DWORD
WINAPI
CertEnumCertificateContextProperties(
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ DWORD dwPropId
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Creates a CTL entry whose attributes are the certificate context's
//  properties.
//
//  The SubjectIdentifier in the CTL entry is the SHA1 hash of the certificate.
//
//  The certificate properties are added as attributes. The property attribute
//  OID is the decimal PROP_ID preceded by szOID_CERT_PROP_ID_PREFIX. Each
//  property value is copied as a single attribute value.
//
//  Any additional attributes to be included in the CTL entry can be passed
//  in via the cOptAttr and rgOptAttr parameters.
//
//  CTL_ENTRY_FROM_PROP_CHAIN_FLAG can be set in dwFlags, to force the
//  inclusion of the chain building hash properties as attributes.
//--------------------------------------------------------------------------
WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertCreateCTLEntryFromCertificateContextProperties(
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ DWORD cOptAttr,
    _In_reads_opt_(cOptAttr) PCRYPT_ATTRIBUTE rgOptAttr,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved,
    _Out_writes_bytes_to_opt_(*pcbCtlEntry, *pcbCtlEntry) PCTL_ENTRY pCtlEntry,
    _Inout_ DWORD *pcbCtlEntry
    );

// Set this flag to get and include the chain building hash properties
// as attributes in the CTL entry
#define CTL_ENTRY_FROM_PROP_CHAIN_FLAG                  0x1


//+-------------------------------------------------------------------------
//  Sets properties on the certificate context using the attributes in
//  the CTL entry.
//
//  The property attribute OID is the decimal PROP_ID preceded by
//  szOID_CERT_PROP_ID_PREFIX. Only attributes containing such an OID are
//  copied.
//
//  CERT_SET_PROPERTY_IGNORE_PERSIST_ERROR_FLAG may be set in dwFlags.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertSetCertificateContextPropertiesFromCTLEntry(
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ PCTL_ENTRY pCtlEntry,
    _In_ DWORD dwFlags
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Get the first or next CRL context from the store for the specified
//  issuer certificate. Perform the enabled verification checks on the CRL.
//
//  If the first or next CRL isn't found, NULL is returned.
//  Otherwise, a pointer to a read only CRL_CONTEXT is returned. CRL_CONTEXT
//  must be freed by calling CertFreeCRLContext. However, the free must be
//  pPrevCrlContext on a subsequent call. CertDuplicateCRLContext
//  can be called to make a duplicate.
//
//  The pIssuerContext may have been obtained from this store, another store
//  or created by the caller application. When created by the caller, the
//  CertCreateCertificateContext function must have been called.
//
//  If pIssuerContext == NULL, finds all the CRLs in the store.
//
//  An issuer may have multiple CRLs. For example, it generates delta CRLs
//  using a X.509 v3 extension. pPrevCrlContext MUST BE NULL on the first
//  call to get the CRL. To get the next CRL for the issuer, the
//  pPrevCrlContext is set to the CRL_CONTEXT returned by a previous call.
//
//  NOTE: a NON-NULL pPrevCrlContext is always CertFreeCRLContext'ed by
//  this function, even for an error.
//
//  The following flags can be set in *pdwFlags to enable verification checks
//  on the returned CRL:
//      CERT_STORE_SIGNATURE_FLAG     - use the public key in the
//                                      issuer's certificate to verify the
//                                      signature on the returned CRL.
//                                      Note, if pIssuerContext->hCertStore ==
//                                      hCertStore, the store provider might
//                                      be able to eliminate a redo of
//                                      the signature verify.
//      CERT_STORE_TIME_VALIDITY_FLAG - get the current time and verify that
//                                      its within the CRL's ThisUpdate and
//                                      NextUpdate validity period.
//      CERT_STORE_BASE_CRL_FLAG      - get base CRL.
//      CERT_STORE_DELTA_CRL_FLAG     - get delta CRL.
//
//  If only one of CERT_STORE_BASE_CRL_FLAG or CERT_STORE_DELTA_CRL_FLAG is
//  set, then, only returns either a base or delta CRL. In any case, the
//  appropriate base or delta flag will be cleared upon returned. If both
//  flags are set, then, only one of flags will be cleared.
//
//  If an enabled verification check fails, then, its flag is set upon return.
//
//  If pIssuerContext == NULL, then, an enabled CERT_STORE_SIGNATURE_FLAG
//  always fails and the CERT_STORE_NO_ISSUER_FLAG is also set.
//
//  For a verification check failure, a pointer to the first or next
//  CRL_CONTEXT is still returned and SetLastError isn't updated.
//--------------------------------------------------------------------------
WINCRYPT32API
PCCRL_CONTEXT
WINAPI
CertGetCRLFromStore(
    _In_ HCERTSTORE hCertStore,
    _In_opt_ PCCERT_CONTEXT pIssuerContext,
    _In_opt_ PCCRL_CONTEXT pPrevCrlContext,
    _Inout_ DWORD *pdwFlags
    );

//+-------------------------------------------------------------------------
//  Enumerate the CRL contexts in the store.
//
//  If a CRL isn't found, NULL is returned.
//  Otherwise, a pointer to a read only CRL_CONTEXT is returned. CRL_CONTEXT
//  must be freed by calling CertFreeCRLContext or is freed when passed as the
//  pPrevCrlContext on a subsequent call. CertDuplicateCRLContext
//  can be called to make a duplicate.
//
//  pPrevCrlContext MUST BE NULL to enumerate the first
//  CRL in the store. Successive CRLs are enumerated by setting
//  pPrevCrlContext to the CRL_CONTEXT returned by a previous call.
//
//  NOTE: a NON-NULL pPrevCrlContext is always CertFreeCRLContext'ed by
//  this function, even for an error.
//--------------------------------------------------------------------------
WINCRYPT32API
PCCRL_CONTEXT
WINAPI
CertEnumCRLsInStore(
    _In_ HCERTSTORE hCertStore,
    _In_opt_ PCCRL_CONTEXT pPrevCrlContext
    );

//+-------------------------------------------------------------------------
//  Find the first or next CRL context in the store.
//
//  The CRL is found according to the dwFindType and its pvFindPara.
//  See below for a list of the find types and its parameters.
//
//  Currently dwFindFlags isn't used and must be set to 0.
//
//  Usage of dwCertEncodingType depends on the dwFindType.
//
//  If the first or next CRL isn't found, NULL is returned.
//  Otherwise, a pointer to a read only CRL_CONTEXT is returned. CRL_CONTEXT
//  must be freed by calling CertFreeCRLContext or is freed when passed as the
//  pPrevCrlContext on a subsequent call. CertDuplicateCRLContext
//  can be called to make a duplicate.
//
//  pPrevCrlContext MUST BE NULL on the first
//  call to find the CRL. To find the next CRL, the
//  pPrevCrlContext is set to the CRL_CONTEXT returned by a previous call.
//
//  NOTE: a NON-NULL pPrevCrlContext is always CertFreeCRLContext'ed by
//  this function, even for an error.
//--------------------------------------------------------------------------
WINCRYPT32API
PCCRL_CONTEXT
WINAPI
CertFindCRLInStore(
    _In_ HCERTSTORE hCertStore,
    _In_ DWORD dwCertEncodingType,
    _In_ DWORD dwFindFlags,
    _In_ DWORD dwFindType,
    _In_opt_ const void *pvFindPara,
    _In_opt_ PCCRL_CONTEXT pPrevCrlContext
    );

#define CRL_FIND_ANY                0
#define CRL_FIND_ISSUED_BY          1
#define CRL_FIND_EXISTING           2
#define CRL_FIND_ISSUED_FOR         3

//+-------------------------------------------------------------------------
//  CRL_FIND_ANY
//
//  Find any CRL.
//
//  pvFindPara isn't used.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CRL_FIND_ISSUED_BY
//
//  Find CRL matching the specified issuer.
//
//  pvFindPara is the PCCERT_CONTEXT of the CRL issuer. May be NULL to
//  match any issuer.
//
//  By default, only does issuer name matching. The following flags can be
//  set in dwFindFlags to do additional filtering.
//
//  If CRL_FIND_ISSUED_BY_AKI_FLAG is set in dwFindFlags, then, checks if the
//  CRL has an Authority Key Identifier (AKI) extension. If the CRL has an
//  AKI, then, only returns a CRL whose AKI matches the issuer.
//
//  Note, the AKI extension has the following OID:
//  szOID_AUTHORITY_KEY_IDENTIFIER2 and its corresponding data structure.
//
//  If CRL_FIND_ISSUED_BY_SIGNATURE_FLAG is set in dwFindFlags, then,
//  uses the public key in the issuer's certificate to verify the
//  signature on the CRL. Only returns a CRL having a valid signature.
//
//  If CRL_FIND_ISSUED_BY_DELTA_FLAG is set in dwFindFlags, then, only
//  returns a delta CRL.
//
//  If CRL_FIND_ISSUED_BY_BASE_FLAG is set in dwFindFlags, then, only
//  returns a base CRL.
//--------------------------------------------------------------------------
#define CRL_FIND_ISSUED_BY_AKI_FLAG         0x1
#define CRL_FIND_ISSUED_BY_SIGNATURE_FLAG   0x2
#define CRL_FIND_ISSUED_BY_DELTA_FLAG       0x4
#define CRL_FIND_ISSUED_BY_BASE_FLAG        0x8


//+-------------------------------------------------------------------------
//  CRL_FIND_EXISTING
//
//  Find existing CRL in the store.
//
//  pvFindPara is the PCCRL_CONTEXT of the CRL to check if it already
//  exists in the store.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CRL_FIND_ISSUED_FOR
//
//  Find CRL for the specified subject and its issuer.
//
//  pvFindPara points to the following CRL_FIND_ISSUED_FOR_PARA which contains
//  both the subject and issuer certificates. Not optional.
//
//  The subject's issuer name is used to match the CRL's issuer name. Otherwise,
//  the issuer's certificate is used the same as in the above
//  CRL_FIND_ISSUED_BY.
//
//  Note, when cross certificates are used, the subject name in the issuer's
//  certificate may not match the issuer name in the subject certificate and
//  its corresponding CRL.
//
//  All of the above CRL_FIND_ISSUED_BY_*_FLAGS apply to this find type.
//--------------------------------------------------------------------------
typedef struct _CRL_FIND_ISSUED_FOR_PARA {
    PCCERT_CONTEXT              pSubjectCert;
    PCCERT_CONTEXT              pIssuerCert;
} CRL_FIND_ISSUED_FOR_PARA, *PCRL_FIND_ISSUED_FOR_PARA;

//
// When the following flag is set, the strong signature properties
// are also set on the returned CRL.
//
//  The strong signature properties are:
//    - CERT_SIGN_HASH_CNG_ALG_PROP_ID
//    - CERT_ISSUER_PUB_KEY_BIT_LENGTH_PROP_ID
//
#define CRL_FIND_ISSUED_FOR_SET_STRONG_PROPERTIES_FLAG        0x10

//+-------------------------------------------------------------------------
//  Duplicate a CRL context
//--------------------------------------------------------------------------
WINCRYPT32API
PCCRL_CONTEXT
WINAPI
CertDuplicateCRLContext(
    _In_opt_ PCCRL_CONTEXT pCrlContext
    );

//+-------------------------------------------------------------------------
//  Create a CRL context from the encoded CRL. The created
//  context isn't put in a store.
//
//  Makes a copy of the encoded CRL in the created context.
//
//  If unable to decode and create the CRL context, NULL is returned.
//  Otherwise, a pointer to a read only CRL_CONTEXT is returned.
//  CRL_CONTEXT must be freed by calling CertFreeCRLContext.
//  CertDuplicateCRLContext can be called to make a duplicate.
//
//  CertSetCRLContextProperty and CertGetCRLContextProperty can be called
//  to store properties for the CRL.
//--------------------------------------------------------------------------
WINCRYPT32API
PCCRL_CONTEXT
WINAPI
CertCreateCRLContext(
    _In_ DWORD dwCertEncodingType,
    _In_reads_bytes_(cbCrlEncoded) const BYTE *pbCrlEncoded,
    _In_ DWORD cbCrlEncoded
    );

//+-------------------------------------------------------------------------
//  Free a CRL context
//
//  There needs to be a corresponding free for each context obtained by a
//  get, duplicate or create.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertFreeCRLContext(
    _In_opt_ PCCRL_CONTEXT pCrlContext
    );

//+-------------------------------------------------------------------------
//  Set the property for the specified CRL context.
//
//  Same Property Ids and semantics as CertSetCertificateContextProperty.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertSetCRLContextProperty(
    _In_ PCCRL_CONTEXT pCrlContext,
    _In_ DWORD dwPropId,
    _In_ DWORD dwFlags,
    _In_opt_ const void *pvData
    );

//+-------------------------------------------------------------------------
//  Get the property for the specified CRL context.
//
//  Same Property Ids and semantics as CertGetCertificateContextProperty.
//
//  CERT_SHA1_HASH_PROP_ID, CERT_MD5_HASH_PROP_ID or
//  CERT_SIGNATURE_HASH_PROP_ID is the predefined property of most interest.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertGetCRLContextProperty(
    _In_ PCCRL_CONTEXT pCrlContext,
    _In_ DWORD dwPropId,
    _Out_writes_bytes_to_opt_(*pcbData, *pcbData) void *pvData,
    _Inout_ DWORD *pcbData
    );

//+-------------------------------------------------------------------------
//  Enumerate the properties for the specified CRL context.
//
//  To get the first property, set dwPropId to 0. The ID of the first
//  property is returned. To get the next property, set dwPropId to the
//  ID returned by the last call. To enumerate all the properties continue
//  until 0 is returned.
//
//  CertGetCRLContextProperty is called to get the property's data.
//--------------------------------------------------------------------------
WINCRYPT32API
DWORD
WINAPI
CertEnumCRLContextProperties(
    _In_ PCCRL_CONTEXT pCrlContext,
    _In_ DWORD dwPropId
    );


//+-------------------------------------------------------------------------
//  Search the CRL's list of entries for the specified certificate.
//
//  TRUE is returned if we were able to search the list. Otherwise, FALSE is
//  returned,
//
//  For success, if the certificate was found in the list, *ppCrlEntry is
//  updated with a pointer to the entry. Otherwise, *ppCrlEntry is set to NULL.
//  The returned entry isn't allocated and must not be freed.
//
//  dwFlags and pvReserved currently aren't used and must be set to 0 or NULL.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertFindCertificateInCRL(
    _In_ PCCERT_CONTEXT pCert,
    _In_ PCCRL_CONTEXT pCrlContext,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved,
    _Outptr_result_maybenull_ PCRL_ENTRY *ppCrlEntry
    );

//+-------------------------------------------------------------------------
//  Is the specified CRL valid for the certificate.
//
//  Returns TRUE if the CRL's list of entries would contain the certificate
//  if it was revoked. Note, doesn't check that the certificate is in the
//  list of entries.
//
//  If the CRL has an Issuing Distribution Point (IDP) extension, checks
//  that it's valid for the subject certificate.
//
//  dwFlags and pvReserved currently aren't used and must be set to 0 and NULL.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertIsValidCRLForCertificate(
    _In_ PCCERT_CONTEXT pCert,
    _In_ PCCRL_CONTEXT pCrl,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved
    );

//+-------------------------------------------------------------------------
// Add certificate/CRL, encoded, context or element disposition values.
//--------------------------------------------------------------------------

#define CERT_STORE_ADD_NEW                                  1
#define CERT_STORE_ADD_USE_EXISTING                         2
#define CERT_STORE_ADD_REPLACE_EXISTING                     3
#define CERT_STORE_ADD_ALWAYS                               4
#define CERT_STORE_ADD_REPLACE_EXISTING_INHERIT_PROPERTIES  5
#define CERT_STORE_ADD_NEWER                                6
#define CERT_STORE_ADD_NEWER_INHERIT_PROPERTIES             7


//+-------------------------------------------------------------------------
//  Add the encoded certificate to the store according to the specified
//  disposition action.
//
//  Makes a copy of the encoded certificate before adding to the store.
//
//  dwAddDispostion specifies the action to take if the certificate
//  already exists in the store. This parameter must be one of the following
//  values:
//    CERT_STORE_ADD_NEW
//      Fails if the certificate already exists in the store. LastError
//      is set to CRYPT_E_EXISTS.
//    CERT_STORE_ADD_USE_EXISTING
//      If the certifcate already exists, then, its used and if ppCertContext
//      is non-NULL, the existing context is duplicated.
//    CERT_STORE_ADD_REPLACE_EXISTING
//      If the certificate already exists, then, the existing certificate
//      context is deleted before creating and adding the new context.
//    CERT_STORE_ADD_ALWAYS
//      No check is made to see if the certificate already exists. A
//      new certificate context is always created. This may lead to
//      duplicates in the store.
//    CERT_STORE_ADD_REPLACE_EXISTING_INHERIT_PROPERTIES
//      If the certificate already exists, then, its used.
//    CERT_STORE_ADD_NEWER
//      Fails if the certificate already exists in the store AND the NotBefore
//      time of the existing certificate is equal to or greater than the
//      NotBefore time of the new certificate being added. LastError
//      is set to CRYPT_E_EXISTS.
//
//      If an older certificate is replaced, same as
//      CERT_STORE_ADD_REPLACE_EXISTING.
//
//      For CRLs or CTLs compares the ThisUpdate times.
//
//    CERT_STORE_ADD_NEWER_INHERIT_PROPERTIES
//      Same as CERT_STORE_ADD_NEWER. However, if an older certificate is
//      replaced, same as CERT_STORE_ADD_REPLACE_EXISTING_INHERIT_PROPERTIES.
//
//  CertGetSubjectCertificateFromStore is called to determine if the
//  certificate already exists in the store.
//
//  ppCertContext can be NULL, indicating the caller isn't interested
//  in getting the CERT_CONTEXT of the added or existing certificate.
//--------------------------------------------------------------------------
WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertAddEncodedCertificateToStore(
    _In_opt_ HCERTSTORE hCertStore,
    _In_ DWORD dwCertEncodingType,
    _In_reads_bytes_(cbCertEncoded) const BYTE *pbCertEncoded,
    _In_ DWORD cbCertEncoded,
    _In_ DWORD dwAddDisposition,
    _Outptr_opt_ PCCERT_CONTEXT *ppCertContext
    );

//+-------------------------------------------------------------------------
//  Add the certificate context to the store according to the specified
//  disposition action.
//
//  In addition to the encoded certificate, the context's properties are
//  also copied.  Note, the CERT_KEY_CONTEXT_PROP_ID property (and its
//  CERT_KEY_PROV_HANDLE_PROP_ID or CERT_KEY_SPEC_PROP_ID) isn't copied.
//
//  Makes a copy of the certificate context before adding to the store.
//
//  dwAddDispostion specifies the action to take if the certificate
//  already exists in the store. This parameter must be one of the following
//  values:
//    CERT_STORE_ADD_NEW
//      Fails if the certificate already exists in the store. LastError
//      is set to CRYPT_E_EXISTS.
//    CERT_STORE_ADD_USE_EXISTING
//      If the certifcate already exists, then, its used and if ppStoreContext
//      is non-NULL, the existing context is duplicated. Iterates
//      through pCertContext's properties and only copies the properties
//      that don't already exist. The SHA1 and MD5 hash properties aren't
//      copied.
//    CERT_STORE_ADD_REPLACE_EXISTING
//      If the certificate already exists, then, the existing certificate
//      context is deleted before creating and adding a new context.
//      Properties are copied before doing the add.
//    CERT_STORE_ADD_ALWAYS
//      No check is made to see if the certificate already exists. A
//      new certificate context is always created and added. This may lead to
//      duplicates in the store. Properties are
//      copied before doing the add.
//    CERT_STORE_ADD_REPLACE_EXISTING_INHERIT_PROPERTIES
//      If the certificate already exists, then, the existing certificate
//      context is used. Properties from the added context are copied and
//      replace existing properties. However, any existing properties not
//      in the added context remain and aren't deleted.
//    CERT_STORE_ADD_NEWER
//      Fails if the certificate already exists in the store AND the NotBefore
//      time of the existing context is equal to or greater than the
//      NotBefore time of the new context being added. LastError
//      is set to CRYPT_E_EXISTS.
//
//      If an older context is replaced, same as
//      CERT_STORE_ADD_REPLACE_EXISTING.
//
//      For CRLs or CTLs compares the ThisUpdate times.
//
//    CERT_STORE_ADD_NEWER_INHERIT_PROPERTIES
//      Same as CERT_STORE_ADD_NEWER. However, if an older context is
//      replaced, same as CERT_STORE_ADD_REPLACE_EXISTING_INHERIT_PROPERTIES.
//
//  CertGetSubjectCertificateFromStore is called to determine if the
//  certificate already exists in the store.
//
//  ppStoreContext can be NULL, indicating the caller isn't interested
//  in getting the CERT_CONTEXT of the added or existing certificate.
//--------------------------------------------------------------------------
WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertAddCertificateContextToStore(
    _In_opt_ HCERTSTORE hCertStore,
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ DWORD dwAddDisposition,
    _Outptr_opt_ PCCERT_CONTEXT *ppStoreContext
    );

//+-------------------------------------------------------------------------
//  Certificate Store Context Types
//--------------------------------------------------------------------------
#define CERT_STORE_CERTIFICATE_CONTEXT  1
#define CERT_STORE_CRL_CONTEXT          2
#define CERT_STORE_CTL_CONTEXT          3

//+-------------------------------------------------------------------------
//  Certificate Store Context Bit Flags
//--------------------------------------------------------------------------
#define CERT_STORE_ALL_CONTEXT_FLAG             (~0UL)
#define CERT_STORE_CERTIFICATE_CONTEXT_FLAG     \
                (1 << CERT_STORE_CERTIFICATE_CONTEXT)
#define CERT_STORE_CRL_CONTEXT_FLAG             \
                (1 << CERT_STORE_CRL_CONTEXT)
#define CERT_STORE_CTL_CONTEXT_FLAG             \
                (1 << CERT_STORE_CTL_CONTEXT)

//+-------------------------------------------------------------------------
//  Add the serialized certificate or CRL element to the store.
//
//  The serialized element contains the encoded certificate, CRL or CTL and
//  its properties, such as, CERT_KEY_PROV_INFO_PROP_ID.
//
//  If hCertStore is NULL, creates a certificate, CRL or CTL context not
//  residing in any store.
//
//  dwAddDispostion specifies the action to take if the certificate or CRL
//  already exists in the store. See CertAddCertificateContextToStore for a
//  list of and actions taken.
//
//  dwFlags currently isn't used and should be set to 0.
//
//  dwContextTypeFlags specifies the set of allowable contexts. For example, to
//  add either a certificate or CRL, set dwContextTypeFlags to:
//      CERT_STORE_CERTIFICATE_CONTEXT_FLAG | CERT_STORE_CRL_CONTEXT_FLAG
//
//  *pdwContextType is updated with the type of the context returned in
//  *ppvContxt. pdwContextType or ppvContext can be NULL, indicating the
//  caller isn't interested in getting the output. If *ppvContext is
//  returned it must be freed by calling CertFreeCertificateContext or
//  CertFreeCRLContext.
//--------------------------------------------------------------------------
WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertAddSerializedElementToStore(
    _In_opt_ HCERTSTORE hCertStore,
    _In_reads_bytes_(cbElement) const BYTE *pbElement,
    _In_ DWORD cbElement,
    _In_ DWORD dwAddDisposition,
    _In_ DWORD dwFlags,
    _In_ DWORD dwContextTypeFlags,
    _Out_opt_ DWORD *pdwContextType,
    _Outptr_opt_ const void **ppvContext
    );

//+-------------------------------------------------------------------------
//  Delete the specified certificate from the store.
//
//  All subsequent gets or finds for the certificate will fail. However,
//  memory allocated for the certificate isn't freed until all of its contexts
//  have also been freed.
//
//  The pCertContext is obtained from a get, enum, find or duplicate.
//
//  Some store provider implementations might also delete the issuer's CRLs
//  if this is the last certificate for the issuer in the store.
//
//  NOTE: the pCertContext is always CertFreeCertificateContext'ed by
//  this function, even for an error.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertDeleteCertificateFromStore(
    _In_ PCCERT_CONTEXT pCertContext
    );

//+-------------------------------------------------------------------------
//  Add the encoded CRL to the store according to the specified
//  disposition option.
//
//  Makes a copy of the encoded CRL before adding to the store.
//
//  dwAddDispostion specifies the action to take if the CRL
//  already exists in the store. See CertAddEncodedCertificateToStore for a
//  list of and actions taken.
//
//  Compares the CRL's Issuer to determine if the CRL already exists in the
//  store.
//
//  ppCrlContext can be NULL, indicating the caller isn't interested
//  in getting the CRL_CONTEXT of the added or existing CRL.
//--------------------------------------------------------------------------
WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertAddEncodedCRLToStore(
    _In_opt_ HCERTSTORE hCertStore,
    _In_ DWORD dwCertEncodingType,
    _In_reads_bytes_(cbCrlEncoded) const BYTE *pbCrlEncoded,
    _In_ DWORD cbCrlEncoded,
    _In_ DWORD dwAddDisposition,
    _Outptr_opt_ PCCRL_CONTEXT *ppCrlContext
    );

//+-------------------------------------------------------------------------
//  Add the CRL context to the store according to the specified
//  disposition option.
//
//  In addition to the encoded CRL, the context's properties are
//  also copied.  Note, the CERT_KEY_CONTEXT_PROP_ID property (and its
//  CERT_KEY_PROV_HANDLE_PROP_ID or CERT_KEY_SPEC_PROP_ID) isn't copied.
//
//  Makes a copy of the encoded CRL before adding to the store.
//
//  dwAddDispostion specifies the action to take if the CRL
//  already exists in the store. See CertAddCertificateContextToStore for a
//  list of and actions taken.
//
//  Compares the CRL's Issuer, ThisUpdate and NextUpdate to determine
//  if the CRL already exists in the store.
//
//  ppStoreContext can be NULL, indicating the caller isn't interested
//  in getting the CRL_CONTEXT of the added or existing CRL.
//--------------------------------------------------------------------------
WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertAddCRLContextToStore(
    _In_opt_ HCERTSTORE hCertStore,
    _In_ PCCRL_CONTEXT pCrlContext,
    _In_ DWORD dwAddDisposition,
    _Outptr_opt_ PCCRL_CONTEXT *ppStoreContext
    );

//+-------------------------------------------------------------------------
//  Delete the specified CRL from the store.
//
//  All subsequent gets for the CRL will fail. However,
//  memory allocated for the CRL isn't freed until all of its contexts
//  have also been freed.
//
//  The pCrlContext is obtained from a get or duplicate.
//
//  NOTE: the pCrlContext is always CertFreeCRLContext'ed by
//  this function, even for an error.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertDeleteCRLFromStore(
    _In_ PCCRL_CONTEXT pCrlContext
    );

//+-------------------------------------------------------------------------
//  Serialize the certificate context's encoded certificate and its
//  properties.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertSerializeCertificateStoreElement(
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ DWORD dwFlags,
    _Out_writes_bytes_to_opt_(*pcbElement, *pcbElement) BYTE *pbElement,
    _Inout_ DWORD *pcbElement
    );


//+-------------------------------------------------------------------------
//  Serialize the CRL context's encoded CRL and its properties.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertSerializeCRLStoreElement(
    _In_ PCCRL_CONTEXT pCrlContext,
    _In_ DWORD dwFlags,
    _Out_writes_bytes_to_opt_(*pcbElement, *pcbElement) BYTE *pbElement,
    _Inout_ DWORD *pcbElement
    );



//+=========================================================================
//  Certificate Trust List (CTL) Store Data Structures and APIs
//==========================================================================

//+-------------------------------------------------------------------------
//  Duplicate a CTL context
//--------------------------------------------------------------------------
WINCRYPT32API
PCCTL_CONTEXT
WINAPI
CertDuplicateCTLContext(
    _In_opt_ PCCTL_CONTEXT pCtlContext
    );

//+-------------------------------------------------------------------------
//  Create a CTL context from the encoded CTL. The created
//  context isn't put in a store.
//
//  Makes a copy of the encoded CTL in the created context.
//
//  If unable to decode and create the CTL context, NULL is returned.
//  Otherwise, a pointer to a read only CTL_CONTEXT is returned.
//  CTL_CONTEXT must be freed by calling CertFreeCTLContext.
//  CertDuplicateCTLContext can be called to make a duplicate.
//
//  CertSetCTLContextProperty and CertGetCTLContextProperty can be called
//  to store properties for the CTL.
//--------------------------------------------------------------------------
WINCRYPT32API
PCCTL_CONTEXT
WINAPI
CertCreateCTLContext(
    _In_ DWORD dwMsgAndCertEncodingType,
    _In_reads_bytes_(cbCtlEncoded) const BYTE *pbCtlEncoded,
    _In_ DWORD cbCtlEncoded
    );

//+-------------------------------------------------------------------------
//  Free a CTL context
//
//  There needs to be a corresponding free for each context obtained by a
//  get, duplicate or create.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertFreeCTLContext(
    _In_opt_ PCCTL_CONTEXT pCtlContext
    );

//+-------------------------------------------------------------------------
//  Set the property for the specified CTL context.
//
//  Same Property Ids and semantics as CertSetCertificateContextProperty.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertSetCTLContextProperty(
    _In_ PCCTL_CONTEXT pCtlContext,
    _In_ DWORD dwPropId,
    _In_ DWORD dwFlags,
    _In_opt_ const void *pvData
    );

//+-------------------------------------------------------------------------
//  Get the property for the specified CTL context.
//
//  Same Property Ids and semantics as CertGetCertificateContextProperty.
//
//  CERT_SHA1_HASH_PROP_ID or CERT_NEXT_UPDATE_LOCATION_PROP_ID are the
//  predefined properties of most interest.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertGetCTLContextProperty(
    _In_ PCCTL_CONTEXT pCtlContext,
    _In_ DWORD dwPropId,
    _Out_writes_bytes_to_opt_(*pcbData, *pcbData) void *pvData,
    _Inout_ DWORD *pcbData
    );

//+-------------------------------------------------------------------------
//  Enumerate the properties for the specified CTL context.
//--------------------------------------------------------------------------
WINCRYPT32API
DWORD
WINAPI
CertEnumCTLContextProperties(
    _In_ PCCTL_CONTEXT pCtlContext,
    _In_ DWORD dwPropId
    );

//+-------------------------------------------------------------------------
//  Enumerate the CTL contexts in the store.
//
//  If a CTL isn't found, NULL is returned.
//  Otherwise, a pointer to a read only CTL_CONTEXT is returned. CTL_CONTEXT
//  must be freed by calling CertFreeCTLContext or is freed when passed as the
//  pPrevCtlContext on a subsequent call. CertDuplicateCTLContext
//  can be called to make a duplicate.
//
//  pPrevCtlContext MUST BE NULL to enumerate the first
//  CTL in the store. Successive CTLs are enumerated by setting
//  pPrevCtlContext to the CTL_CONTEXT returned by a previous call.
//
//  NOTE: a NON-NULL pPrevCtlContext is always CertFreeCTLContext'ed by
//  this function, even for an error.
//--------------------------------------------------------------------------
WINCRYPT32API
PCCTL_CONTEXT
WINAPI
CertEnumCTLsInStore(
    _In_ HCERTSTORE hCertStore,
    _In_opt_ PCCTL_CONTEXT pPrevCtlContext
    );

//+-------------------------------------------------------------------------
//  Attempt to find the specified subject in the CTL.
//
//  For CTL_CERT_SUBJECT_TYPE, pvSubject points to a CERT_CONTEXT. The CTL's
//  SubjectAlgorithm is examined to determine the representation of the
//  subject's identity. Initially, only SHA1 or MD5 hash will be supported.
//  The appropriate hash property is obtained from the CERT_CONTEXT.
//
//  For CTL_ANY_SUBJECT_TYPE, pvSubject points to the CTL_ANY_SUBJECT_INFO
//  structure which contains the SubjectAlgorithm to be matched in the CTL
//  and the SubjectIdentifer to be matched in one of the CTL entries.
//
//  The certificate's hash or the CTL_ANY_SUBJECT_INFO's SubjectIdentifier
//  is used as the key in searching the subject entries. A binary
//  memory comparison is done between the key and the entry's SubjectIdentifer.
//
//  dwEncodingType isn't used for either of the above SubjectTypes.
//--------------------------------------------------------------------------
WINCRYPT32API
PCTL_ENTRY
WINAPI
CertFindSubjectInCTL(
    _In_ DWORD dwEncodingType,
    _In_ DWORD dwSubjectType,
    _In_ void *pvSubject,
    _In_ PCCTL_CONTEXT pCtlContext,
    _In_ DWORD dwFlags
    );

// Subject Types:
//  CTL_ANY_SUBJECT_TYPE, pvSubject points to following CTL_ANY_SUBJECT_INFO.
//  CTL_CERT_SUBJECT_TYPE, pvSubject points to CERT_CONTEXT.
#define CTL_ANY_SUBJECT_TYPE            1
#define CTL_CERT_SUBJECT_TYPE           2

typedef struct _CTL_ANY_SUBJECT_INFO {
    CRYPT_ALGORITHM_IDENTIFIER  SubjectAlgorithm;
    CRYPT_DATA_BLOB             SubjectIdentifier;
} CTL_ANY_SUBJECT_INFO, *PCTL_ANY_SUBJECT_INFO;

//+-------------------------------------------------------------------------
//  Find the first or next CTL context in the store.
//
//  The CTL is found according to the dwFindType and its pvFindPara.
//  See below for a list of the find types and its parameters.
//
//  Currently dwFindFlags isn't used and must be set to 0.
//
//  Usage of dwMsgAndCertEncodingType depends on the dwFindType.
//
//  If the first or next CTL isn't found, NULL is returned.
//  Otherwise, a pointer to a read only CTL_CONTEXT is returned. CTL_CONTEXT
//  must be freed by calling CertFreeCTLContext or is freed when passed as the
//  pPrevCtlContext on a subsequent call. CertDuplicateCTLContext
//  can be called to make a duplicate.
//
//  pPrevCtlContext MUST BE NULL on the first
//  call to find the CTL. To find the next CTL, the
//  pPrevCtlContext is set to the CTL_CONTEXT returned by a previous call.
//
//  NOTE: a NON-NULL pPrevCtlContext is always CertFreeCTLContext'ed by
//  this function, even for an error.
//--------------------------------------------------------------------------
WINCRYPT32API
PCCTL_CONTEXT
WINAPI
CertFindCTLInStore(
    _In_ HCERTSTORE hCertStore,
    _In_ DWORD dwMsgAndCertEncodingType,
    _In_ DWORD dwFindFlags,
    _In_ DWORD dwFindType,
    _In_opt_ const void *pvFindPara,
    _In_opt_ PCCTL_CONTEXT pPrevCtlContext
    );

#define CTL_FIND_ANY                0
#define CTL_FIND_SHA1_HASH          1
#define CTL_FIND_MD5_HASH           2
#define CTL_FIND_USAGE              3
#define CTL_FIND_SUBJECT            4
#define CTL_FIND_EXISTING           5

typedef struct _CTL_FIND_USAGE_PARA {
    DWORD               cbSize;
    CTL_USAGE           SubjectUsage;   // optional
    CRYPT_DATA_BLOB     ListIdentifier; // optional
    PCERT_INFO          pSigner;        // optional
} CTL_FIND_USAGE_PARA, *PCTL_FIND_USAGE_PARA;

#define CTL_FIND_NO_LIST_ID_CBDATA  0xFFFFFFFF
#define CTL_FIND_NO_SIGNER_PTR      ((PCERT_INFO) -1)

#define CTL_FIND_SAME_USAGE_FLAG    0x1


typedef struct _CTL_FIND_SUBJECT_PARA {
    DWORD                   cbSize;
    PCTL_FIND_USAGE_PARA    pUsagePara; // optional
    DWORD                   dwSubjectType;
    void                    *pvSubject;
} CTL_FIND_SUBJECT_PARA, *PCTL_FIND_SUBJECT_PARA;

//+-------------------------------------------------------------------------
//  CTL_FIND_ANY
//
//  Find any CTL.
//
//  pvFindPara isn't used.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CTL_FIND_SHA1_HASH
//  CTL_FIND_MD5_HASH
//
//  Find a CTL with the specified hash.
//
//  pvFindPara points to a CRYPT_HASH_BLOB.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CTL_FIND_USAGE
//
//  Find a CTL having the specified usage identifiers, list identifier or
//  signer. The CertEncodingType of the signer is obtained from the
//  dwMsgAndCertEncodingType parameter.
//
//  pvFindPara points to a CTL_FIND_USAGE_PARA data structure. The
//  SubjectUsage.cUsageIdentifer can be 0 to match any usage. The
//  ListIdentifier.cbData can be 0 to match any list identifier. To only match
//  CTLs without a ListIdentifier, cbData must be set to
//  CTL_FIND_NO_LIST_ID_CBDATA. pSigner can be NULL to match any signer. Only
//  the Issuer and SerialNumber fields of the pSigner's PCERT_INFO are used.
//  To only match CTLs without a signer, pSigner must be set to
//  CTL_FIND_NO_SIGNER_PTR.
//
//  The CTL_FIND_SAME_USAGE_FLAG can be set in dwFindFlags to
//  only match CTLs with the same usage identifiers. CTLs having additional
//  usage identifiers aren't matched. For example, if only "1.2.3" is specified
//  in CTL_FIND_USAGE_PARA, then, for a match, the CTL must only contain
//  "1.2.3" and not any additional usage identifers.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CTL_FIND_SUBJECT
//
//  Find a CTL having the specified subject. CertFindSubjectInCTL can be
//  called to get a pointer to the subject's entry in the CTL.  pUsagePara can
//  optionally be set to enable the above CTL_FIND_USAGE matching.
//
//  pvFindPara points to a CTL_FIND_SUBJECT_PARA data structure.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  Add the encoded CTL to the store according to the specified
//  disposition option.
//
//  Makes a copy of the encoded CTL before adding to the store.
//
//  dwAddDispostion specifies the action to take if the CTL
//  already exists in the store. See CertAddEncodedCertificateToStore for a
//  list of and actions taken.
//
//  Compares the CTL's SubjectUsage, ListIdentifier and any of its signers
//  to determine if the CTL already exists in the store.
//
//  ppCtlContext can be NULL, indicating the caller isn't interested
//  in getting the CTL_CONTEXT of the added or existing CTL.
//--------------------------------------------------------------------------
WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertAddEncodedCTLToStore(
    _In_opt_ HCERTSTORE hCertStore,
    _In_ DWORD dwMsgAndCertEncodingType,
    _In_reads_bytes_(cbCtlEncoded) const BYTE *pbCtlEncoded,
    _In_ DWORD cbCtlEncoded,
    _In_ DWORD dwAddDisposition,
    _Outptr_opt_ PCCTL_CONTEXT *ppCtlContext
    );

//+-------------------------------------------------------------------------
//  Add the CTL context to the store according to the specified
//  disposition option.
//
//  In addition to the encoded CTL, the context's properties are
//  also copied.  Note, the CERT_KEY_CONTEXT_PROP_ID property (and its
//  CERT_KEY_PROV_HANDLE_PROP_ID or CERT_KEY_SPEC_PROP_ID) isn't copied.
//
//  Makes a copy of the encoded CTL before adding to the store.
//
//  dwAddDispostion specifies the action to take if the CTL
//  already exists in the store. See CertAddCertificateContextToStore for a
//  list of and actions taken.
//
//  Compares the CTL's SubjectUsage, ListIdentifier and any of its signers
//  to determine if the CTL already exists in the store.
//
//  ppStoreContext can be NULL, indicating the caller isn't interested
//  in getting the CTL_CONTEXT of the added or existing CTL.
//--------------------------------------------------------------------------
WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertAddCTLContextToStore(
    _In_opt_ HCERTSTORE hCertStore,
    _In_ PCCTL_CONTEXT pCtlContext,
    _In_ DWORD dwAddDisposition,
    _Outptr_opt_ PCCTL_CONTEXT *ppStoreContext
    );

//+-------------------------------------------------------------------------
//  Serialize the CTL context's encoded CTL and its properties.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertSerializeCTLStoreElement(
    _In_ PCCTL_CONTEXT pCtlContext,
    _In_ DWORD dwFlags,
    _Out_writes_bytes_to_opt_(*pcbElement, *pcbElement) BYTE *pbElement,
    _Inout_ DWORD *pcbElement
    );

//+-------------------------------------------------------------------------
//  Delete the specified CTL from the store.
//
//  All subsequent gets for the CTL will fail. However,
//  memory allocated for the CTL isn't freed until all of its contexts
//  have also been freed.
//
//  The pCtlContext is obtained from a get or duplicate.
//
//  NOTE: the pCtlContext is always CertFreeCTLContext'ed by
//  this function, even for an error.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertDeleteCTLFromStore(
    _In_ PCCTL_CONTEXT pCtlContext
    );


WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertAddCertificateLinkToStore(
    _In_ HCERTSTORE hCertStore,
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ DWORD dwAddDisposition,
    _Outptr_opt_ PCCERT_CONTEXT *ppStoreContext
    );

WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertAddCRLLinkToStore(
    _In_ HCERTSTORE hCertStore,
    _In_ PCCRL_CONTEXT pCrlContext,
    _In_ DWORD dwAddDisposition,
    _Outptr_opt_ PCCRL_CONTEXT *ppStoreContext
    );

WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertAddCTLLinkToStore(
    _In_ HCERTSTORE hCertStore,
    _In_ PCCTL_CONTEXT pCtlContext,
    _In_ DWORD dwAddDisposition,
    _Outptr_opt_ PCCTL_CONTEXT *ppStoreContext
    );

WINCRYPT32API
BOOL
WINAPI
CertAddStoreToCollection(
    _In_ HCERTSTORE hCollectionStore,
    _In_opt_ HCERTSTORE hSiblingStore,
    _In_ DWORD dwUpdateFlags,
    _In_ DWORD dwPriority
    );

WINCRYPT32API
void
WINAPI
CertRemoveStoreFromCollection(
    _In_ HCERTSTORE hCollectionStore,
    _In_ HCERTSTORE hSiblingStore
    );


WINCRYPT32API
BOOL
WINAPI
CertControlStore(
    _In_ HCERTSTORE hCertStore,
    _In_ DWORD dwFlags,
    _In_ DWORD dwCtrlType,
    _In_opt_ void const *pvCtrlPara
    );

//+-------------------------------------------------------------------------
//  Certificate Store control types
//--------------------------------------------------------------------------
#define CERT_STORE_CTRL_RESYNC              1
#define CERT_STORE_CTRL_NOTIFY_CHANGE       2
#define CERT_STORE_CTRL_COMMIT              3
#define CERT_STORE_CTRL_AUTO_RESYNC         4
#define CERT_STORE_CTRL_CANCEL_NOTIFY       5

#define CERT_STORE_CTRL_INHIBIT_DUPLICATE_HANDLE_FLAG   0x1

//+-------------------------------------------------------------------------
//  CERT_STORE_CTRL_RESYNC
//
//  Re-synchronize the store.
//
//  The pvCtrlPara points to the event HANDLE to be signaled on
//  the next store change. Normally, this would be the same
//  event HANDLE passed to CERT_STORE_CTRL_NOTIFY_CHANGE during initialization.
//
//  If pvCtrlPara is NULL, no events are re-armed.
//
//  By default the event HANDLE is DuplicateHandle'd.
//  CERT_STORE_CTRL_INHIBIT_DUPLICATE_HANDLE_FLAG can be set in dwFlags
//  to inhibit a DupicateHandle of the event HANDLE. If this flag
//  is set, then, CertControlStore(CERT_STORE_CTRL_CANCEL_NOTIFY) must be
//  called for this event HANDLE before closing the hCertStore.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_STORE_CTRL_NOTIFY_CHANGE
//
//  Signal the event when the underlying store is changed.
//
//  pvCtrlPara points to the event HANDLE to be signaled.
//
//  pvCtrlPara can be NULL to inform the store of a subsequent
//  CERT_STORE_CTRL_RESYNC and allow it to optimize by only doing a resync
//  if the store has changed. For the registry based stores, an internal
//  notify change event is created and registered to be signaled.
//
//  Recommend calling CERT_STORE_CTRL_NOTIFY_CHANGE once for each event to
//  be passed to CERT_STORE_CTRL_RESYNC. This should only happen after
//  the event has been created. Not after each time the event is signaled.
//
//  By default the event HANDLE is DuplicateHandle'd.
//  CERT_STORE_CTRL_INHIBIT_DUPLICATE_HANDLE_FLAG can be set in dwFlags
//  to inhibit a DupicateHandle of the event HANDLE. If this flag
//  is set, then, CertControlStore(CERT_STORE_CTRL_CANCEL_NOTIFY) must be
//  called for this event HANDLE before closing the hCertStore.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_STORE_CTRL_CANCEL_NOTIFY
//
//  Cancel notification signaling of the event HANDLE passed in a previous
//  CERT_STORE_CTRL_NOTIFY_CHANGE or CERT_STORE_CTRL_RESYNC.
//
//  pvCtrlPara points to the event HANDLE to be canceled.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_STORE_CTRL_AUTO_RESYNC
//
//  At the start of every enumeration or find store API call, check if the
//  underlying store has changed. If it has changed, re-synchronize.
//
//  This check is only done in the enumeration or find APIs when the
//  pPrevContext is NULL.
//
//  The pvCtrlPara isn't used and must be set to NULL.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_STORE_CTRL_COMMIT
//
//  If any changes have been to the cached store, they are committed to
//  persisted storage. If no changes have been made since the store was
//  opened or the last commit, this call is ignored. May also be ignored by
//  store providers that persist changes immediately.
//
//  CERT_STORE_CTRL_COMMIT_FORCE_FLAG can be set to force the store
//  to be committed even if it hasn't been touched.
//
//  CERT_STORE_CTRL_COMMIT_CLEAR_FLAG can be set to inhibit a commit on
//  store close.
//--------------------------------------------------------------------------

#define CERT_STORE_CTRL_COMMIT_FORCE_FLAG   0x1
#define CERT_STORE_CTRL_COMMIT_CLEAR_FLAG   0x2


//+=========================================================================
//  Cert Store Property Defines and APIs
//==========================================================================

//+-------------------------------------------------------------------------
//  Store property IDs. This is a property applicable to the entire store.
//  Its not a property on an individual certificate, CRL or CTL context.
//
//  Currently, no store properties are persisted. (This differs from
//  most context properties which are persisted.)
//
//  See CertSetStoreProperty or CertGetStoreProperty for usage information.
//
//  Note, the range for predefined store properties should be outside
//  the range of predefined context properties. We will start at 4096.
//--------------------------------------------------------------------------
// certenrolld_begin -- CERT_*_PROP_ID
#define CERT_STORE_LOCALIZED_NAME_PROP_ID   0x1000
// certenrolld_end

//+-------------------------------------------------------------------------
//  Set a store property.
//
//  The type definition for pvData depends on the dwPropId value.
//      CERT_STORE_LOCALIZED_NAME_PROP_ID - localized name of the store.
//      pvData points to a CRYPT_DATA_BLOB. pbData is a pointer to a NULL
//      terminated unicode, wide character string.
//      cbData = (wcslen((LPWSTR) pbData) + 1) * sizeof(WCHAR).
//
//  For all the other PROP_IDs: an encoded PCRYPT_DATA_BLOB is passed in pvData.
//
//  If the property already exists, then, the old value is deleted and silently
//  replaced. Setting, pvData to NULL, deletes the property.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertSetStoreProperty(
    _In_ HCERTSTORE hCertStore,
    _In_ DWORD dwPropId,
    _In_ DWORD dwFlags,
    _In_opt_ const void *pvData
    );

//+-------------------------------------------------------------------------
//  Get a store property.
//
//  The type definition for pvData depends on the dwPropId value.
//      CERT_STORE_LOCALIZED_NAME_PROP_ID - localized name of the store.
//      pvData points to a NULL terminated unicode, wide character string.
//      cbData = (wcslen((LPWSTR) pvData) + 1) * sizeof(WCHAR).
//
//  For all other PROP_IDs, pvData points to an array of bytes.
//
//  If the property doesn't exist, returns FALSE and sets LastError to
//  CRYPT_E_NOT_FOUND.
//--------------------------------------------------------------------------
WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertGetStoreProperty(
    _In_ HCERTSTORE hCertStore,
    _In_ DWORD dwPropId,
    _Out_writes_bytes_to_opt_(*pcbData, *pcbData) void *pvData,
    _Inout_ DWORD *pcbData
    );

//+-------------------------------------------------------------------------
// If the callback returns FALSE, stops the sort. CertCreateContext
// will return FALSE and set last error to ERROR_CANCELLED if the sort
// was stopped.
//
// Where:
//  cbTotalEncoded  - total byte count of the encoded entries.
//  cbRemainEncoded - remaining byte count of the encoded entries.
//  cEntry          - running count of sorted entries
//  pvSort          - value passed in pCreatePara
//--------------------------------------------------------------------------
typedef BOOL (WINAPI *PFN_CERT_CREATE_CONTEXT_SORT_FUNC)(
    _In_ DWORD cbTotalEncoded,
    _In_ DWORD cbRemainEncoded,
    _In_ DWORD cEntry,
    _Inout_opt_ void *pvSort
    );

typedef struct _CERT_CREATE_CONTEXT_PARA {
    DWORD                               cbSize;
    PFN_CRYPT_FREE                      pfnFree;    // OPTIONAL
    void                                *pvFree;    // OPTIONAL

    // Only applicable to CERT_STORE_CTL_CONTEXT when
    // CERT_CREATE_CONTEXT_SORTED_FLAG is set in dwFlags.
    PFN_CERT_CREATE_CONTEXT_SORT_FUNC   pfnSort;    // OPTIONAL
    void                                *pvSort;    // OPTIONAL
} CERT_CREATE_CONTEXT_PARA, *PCERT_CREATE_CONTEXT_PARA;

//+-------------------------------------------------------------------------
//  Creates the specified context from the encoded bytes. The created
//  context isn't put in a store.
//
//  dwContextType values:
//      CERT_STORE_CERTIFICATE_CONTEXT
//      CERT_STORE_CRL_CONTEXT
//      CERT_STORE_CTL_CONTEXT
//
//  If CERT_CREATE_CONTEXT_NOCOPY_FLAG is set, the created context points
//  directly to the pbEncoded instead of an allocated copy. See flag
//  definition for more details.
//
//  If CERT_CREATE_CONTEXT_SORTED_FLAG is set, the context is created
//  with sorted entries. This flag may only be set for CERT_STORE_CTL_CONTEXT.
//  Setting this flag implicitly sets CERT_CREATE_CONTEXT_NO_HCRYPTMSG_FLAG and
//  CERT_CREATE_CONTEXT_NO_ENTRY_FLAG. See flag definition for
//  more details.
//
//  If CERT_CREATE_CONTEXT_NO_HCRYPTMSG_FLAG is set, the context is created
//  without creating a HCRYPTMSG handle for the context. This flag may only be
//  set for CERT_STORE_CTL_CONTEXT.  See flag definition for more details.
//
//  If CERT_CREATE_CONTEXT_NO_ENTRY_FLAG is set, the context is created
//  without decoding the entries. This flag may only be set for
//  CERT_STORE_CTL_CONTEXT.  See flag definition for more details.
//
//  If unable to decode and create the context, NULL is returned.
//  Otherwise, a pointer to a read only CERT_CONTEXT, CRL_CONTEXT or
//  CTL_CONTEXT is returned. The context must be freed by the appropriate
//  free context API. The context can be duplicated by calling the
//  appropriate duplicate context API.
//--------------------------------------------------------------------------
WINCRYPT32API
const void *
WINAPI
CertCreateContext(
    _In_ DWORD dwContextType,
    _In_ DWORD dwEncodingType,
    _In_reads_bytes_(cbEncoded) const BYTE *pbEncoded,
    _In_ DWORD cbEncoded,
    _In_ DWORD dwFlags,
    _In_opt_ PCERT_CREATE_CONTEXT_PARA pCreatePara
    );

// When the following flag is set, the created context points directly to the
// pbEncoded instead of an allocated copy. If pCreatePara and
// pCreatePara->pfnFree are non-NULL, then, pfnFree is called to free
// the pbEncoded when the context is last freed. Otherwise, no attempt is
// made to free the pbEncoded. If pCreatePara->pvFree is non-NULL, then its
// passed to pfnFree instead of pbEncoded.
//
// Note, if CertCreateContext fails, pfnFree is still called.
#define CERT_CREATE_CONTEXT_NOCOPY_FLAG     0x1

// When the following flag is set, a context with sorted entries is created.
// Currently only applicable to a CTL context.
//
// For CTLs: the cCTLEntry in the returned CTL_INFO is always
// 0. CertFindSubjectInSortedCTL and CertEnumSubjectInSortedCTL must be called
// to find or enumerate the CTL entries.
//
// The Sorted CTL TrustedSubjects extension isn't returned in the created
// context's CTL_INFO.
//
// pfnSort and pvSort can be set in the pCreatePara parameter to be called for
// each sorted entry. pfnSort can return FALSE to stop the sorting.
#define CERT_CREATE_CONTEXT_SORTED_FLAG     0x2

// By default when a CTL context is created, a HCRYPTMSG handle to its
// SignedData message is created. This flag can be set to improve performance
// by not creating the HCRYPTMSG handle.
//
// This flag is only applicable to a CTL context.
#define CERT_CREATE_CONTEXT_NO_HCRYPTMSG_FLAG   0x4

// By default when a CTL context is created, its entries are decoded.
// This flag can be set to improve performance by not decoding the
// entries.
//
// This flag is only applicable to a CTL context.
#define CERT_CREATE_CONTEXT_NO_ENTRY_FLAG       0x8


//+=========================================================================
//  Certificate System Store Data Structures and APIs
//==========================================================================

//+-------------------------------------------------------------------------
//  System Store Information
//
//  Currently, no system store information is persisted.
//--------------------------------------------------------------------------
typedef struct _CERT_SYSTEM_STORE_INFO {
    DWORD   cbSize;
} CERT_SYSTEM_STORE_INFO, *PCERT_SYSTEM_STORE_INFO;

//+-------------------------------------------------------------------------
//  Physical Store Information
//
//  The Open fields are passed directly to CertOpenStore() to open
//  the physical store.
//
//  By default all system stores located in the registry have an
//  implicit SystemRegistry physical store that is opened. To disable the
//  opening of this store, the SystemRegistry
//  physical store corresponding to the System store must be registered with
//  CERT_PHYSICAL_STORE_OPEN_DISABLE_FLAG set in dwFlags. Alternatively,
//  a physical store with the name of ".Default" may be registered.
//
//  Depending on the store location and store name, additional predefined
//  physical stores may be opened. For example, system stores in
//  CURRENT_USER have the predefined physical store, .LocalMachine.
//  To disable the opening of these predefined physical stores, the
//  corresponding physical store must be registered with
//  CERT_PHYSICAL_STORE_OPEN_DISABLE_FLAG set in dwFlags.
//
//  The CERT_PHYSICAL_STORE_ADD_ENABLE_FLAG must be set in dwFlags
//  to enable the adding of a context to the store.
//
//  When a system store is opened via the SERVICES or USERS store location,
//  the ServiceName\ is prepended to the OpenParameters
//  for CERT_SYSTEM_STORE_CURRENT_USER or CERT_SYSTEM_STORE_CURRENT_SERVICE
//  physical stores and the dwOpenFlags store location is changed to
//  CERT_SYSTEM_STORE_USERS or CERT_SYSTEM_STORE_SERVICES.
//
//  By default the SYSTEM, SYSTEM_REGISTRY and PHYSICAL provider
//  stores are also opened remotely when the outer system store is opened.
//  The CERT_PHYSICAL_STORE_REMOTE_OPEN_DISABLE_FLAG may be set in dwFlags
//  to disable remote opens.
//
//  When opened remotely, the \\ComputerName is implicitly prepended to the
//  OpenParameters for the SYSTEM, SYSTEM_REGISTRY and PHYSICAL provider types.
//  To also prepend the \\ComputerName to other provider types, set the
//  CERT_PHYSICAL_STORE_INSERT_COMPUTER_NAME_ENABLE_FLAG in dwFlags.
//
//  When the system store is opened, its physical stores are ordered
//  according to the dwPriority. A larger dwPriority indicates higher priority.
//--------------------------------------------------------------------------
typedef struct _CERT_PHYSICAL_STORE_INFO {
    DWORD               cbSize;
    LPSTR               pszOpenStoreProvider;   // REG_SZ
    DWORD               dwOpenEncodingType;     // REG_DWORD
    DWORD               dwOpenFlags;            // REG_DWORD
    CRYPT_DATA_BLOB     OpenParameters;         // REG_BINARY
    DWORD               dwFlags;                // REG_DWORD
    DWORD               dwPriority;             // REG_DWORD
} CERT_PHYSICAL_STORE_INFO, *PCERT_PHYSICAL_STORE_INFO;

//+-------------------------------------------------------------------------
//  Physical Store Information dwFlags
//--------------------------------------------------------------------------
#define CERT_PHYSICAL_STORE_ADD_ENABLE_FLAG                     0x1
#define CERT_PHYSICAL_STORE_OPEN_DISABLE_FLAG                   0x2
#define CERT_PHYSICAL_STORE_REMOTE_OPEN_DISABLE_FLAG            0x4
#define CERT_PHYSICAL_STORE_INSERT_COMPUTER_NAME_ENABLE_FLAG    0x8


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Register a system store.
//
//  The upper word of the dwFlags parameter is used to specify the location of
//  the system store.
//
//  If CERT_SYSTEM_STORE_RELOCATE_FLAG is set in dwFlags, pvSystemStore
//  points to a CERT_SYSTEM_STORE_RELOCATE_PARA data structure. Otherwise,
//  pvSystemStore points to a null terminated UNICODE string.
//
//  The CERT_SYSTEM_STORE_SERVICES or CERT_SYSTEM_STORE_USERS system store
//  name must be prefixed with the ServiceName or UserName. For example,
//  "ServiceName\Trust".
//
//  Stores on remote computers can be registered for the
//  CERT_SYSTEM_STORE_LOCAL_MACHINE, CERT_SYSTEM_STORE_SERVICES,
//  CERT_SYSTEM_STORE_USERS, CERT_SYSTEM_STORE_LOCAL_MACHINE_GROUP_POLICY
//  or CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE
//  locations by prepending the computer name. For example, a remote
//  local machine store is registered via "\\ComputerName\Trust" or
//  "ComputerName\Trust". A remote service store is registered via
//  "\\ComputerName\ServiceName\Trust". The leading "\\" backslashes are
//  optional in the ComputerName.
//
//  Set CERT_STORE_CREATE_NEW_FLAG to cause a failure if the system store
//  already exists in the store location.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertRegisterSystemStore(
    _In_ const void *pvSystemStore,
    _In_ DWORD dwFlags,
    _In_opt_ PCERT_SYSTEM_STORE_INFO pStoreInfo,
    _Reserved_ void *pvReserved
    );

//+-------------------------------------------------------------------------
//  Register a physical store for the specified system store.
//
//  The upper word of the dwFlags parameter is used to specify the location of
//  the system store.
//
//  If CERT_SYSTEM_STORE_RELOCATE_FLAG is set in dwFlags, pvSystemStore
//  points to a CERT_SYSTEM_STORE_RELOCATE_PARA data structure. Otherwise,
//  pvSystemStore points to a null terminated UNICODE string.
//
//  See CertRegisterSystemStore for details on prepending a ServiceName
//  and/or ComputerName to the system store name.
//
//  Set CERT_STORE_CREATE_NEW_FLAG to cause a failure if the physical store
//  already exists in the system store.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertRegisterPhysicalStore(
    _In_ const void *pvSystemStore,
    _In_ DWORD dwFlags,
    _In_ LPCWSTR pwszStoreName,
    _In_ PCERT_PHYSICAL_STORE_INFO pStoreInfo,
    _Reserved_ void *pvReserved
    );

//+-------------------------------------------------------------------------
//  Unregister the specified system store.
//
//  The upper word of the dwFlags parameter is used to specify the location of
//  the system store.
//
//  If CERT_SYSTEM_STORE_RELOCATE_FLAG is set in dwFlags, pvSystemStore
//  points to a CERT_SYSTEM_STORE_RELOCATE_PARA data structure. Otherwise,
//  pvSystemStore points to a null terminated UNICODE string.
//
//  See CertRegisterSystemStore for details on prepending a ServiceName
//  and/or ComputerName to the system store name.
//
//  CERT_STORE_DELETE_FLAG can optionally be set in dwFlags.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertUnregisterSystemStore(
    _In_ const void *pvSystemStore,
    _In_ DWORD dwFlags
    );

//+-------------------------------------------------------------------------
//  Unregister the physical store from the specified system store.
//
//  The upper word of the dwFlags parameter is used to specify the location of
//  the system store.
//
//  If CERT_SYSTEM_STORE_RELOCATE_FLAG is set in dwFlags, pvSystemStore
//  points to a CERT_SYSTEM_STORE_RELOCATE_PARA data structure. Otherwise,
//  pvSystemStore points to a null terminated UNICODE string.
//
//  See CertRegisterSystemStore for details on prepending a ServiceName
//  and/or ComputerName to the system store name.
//
//  CERT_STORE_DELETE_FLAG can optionally be set in dwFlags.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertUnregisterPhysicalStore(
    _In_ const void *pvSystemStore,
    _In_ DWORD dwFlags,
    _In_ LPCWSTR pwszStoreName
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Enum callbacks
//
//  The CERT_SYSTEM_STORE_LOCATION_MASK bits in the dwFlags parameter
//  specifies the location of the system store
//
//  If CERT_SYSTEM_STORE_RELOCATE_FLAG is set in dwFlags, pvSystemStore
//  points to a CERT_SYSTEM_STORE_RELOCATE_PARA data structure. Otherwise,
//  pvSystemStore points to a null terminated UNICODE string.
//
//  The callback returns FALSE and sets LAST_ERROR to stop the enumeration.
//  The LAST_ERROR is returned to the caller of the enumeration.
//
//  The pvSystemStore passed to the callback has leading ComputerName and/or
//  ServiceName prefixes where appropriate.
//--------------------------------------------------------------------------

typedef BOOL (WINAPI *PFN_CERT_ENUM_SYSTEM_STORE_LOCATION)(
    _In_ LPCWSTR pwszStoreLocation,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved,
    _Inout_opt_ void *pvArg
    );

typedef BOOL (WINAPI *PFN_CERT_ENUM_SYSTEM_STORE)(
    _In_ const void *pvSystemStore,
    _In_ DWORD dwFlags,
    _In_ PCERT_SYSTEM_STORE_INFO pStoreInfo,
    _Reserved_ void *pvReserved,
    _Inout_opt_ void *pvArg
    );

typedef BOOL (WINAPI *PFN_CERT_ENUM_PHYSICAL_STORE)(
    _In_ const void *pvSystemStore,
    _In_ DWORD dwFlags,
    _In_ LPCWSTR pwszStoreName,
    _In_ PCERT_PHYSICAL_STORE_INFO pStoreInfo,
    _Reserved_ void *pvReserved,
    _Inout_opt_ void *pvArg
    );

// In the PFN_CERT_ENUM_PHYSICAL_STORE callback the following flag is
// set if the physical store wasn't registered and is an implicitly created
// predefined physical store.
#define CERT_PHYSICAL_STORE_PREDEFINED_ENUM_FLAG    0x1

// Names of implicitly created predefined physical stores
#define CERT_PHYSICAL_STORE_DEFAULT_NAME            L".Default"
#define CERT_PHYSICAL_STORE_GROUP_POLICY_NAME       L".GroupPolicy"
#define CERT_PHYSICAL_STORE_LOCAL_MACHINE_NAME      L".LocalMachine"
#define CERT_PHYSICAL_STORE_DS_USER_CERTIFICATE_NAME L".UserCertificate"
#define CERT_PHYSICAL_STORE_LOCAL_MACHINE_GROUP_POLICY_NAME \
            L".LocalMachineGroupPolicy"
#define CERT_PHYSICAL_STORE_ENTERPRISE_NAME         L".Enterprise"
#define CERT_PHYSICAL_STORE_AUTH_ROOT_NAME          L".AuthRoot"
#define CERT_PHYSICAL_STORE_SMART_CARD_NAME         L".SmartCard"

//+-------------------------------------------------------------------------
//  Enumerate the system store locations.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertEnumSystemStoreLocation(
    _In_ DWORD dwFlags,
    _Inout_opt_ void *pvArg,
    __callback PFN_CERT_ENUM_SYSTEM_STORE_LOCATION pfnEnum
    );

//+-------------------------------------------------------------------------
//  Enumerate the system stores.
//
//  The upper word of the dwFlags parameter is used to specify the location of
//  the system store.
//
//  If CERT_SYSTEM_STORE_RELOCATE_FLAG is set in dwFlags,
//  pvSystemStoreLocationPara points to a CERT_SYSTEM_STORE_RELOCATE_PARA
//  data structure. Otherwise, pvSystemStoreLocationPara points to a null
//  terminated UNICODE string.
//
//  For CERT_SYSTEM_STORE_LOCAL_MACHINE,
//  CERT_SYSTEM_STORE_LOCAL_MACHINE_GROUP_POLICY or
//  CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE, pvSystemStoreLocationPara can
//  optionally be set to a unicode computer name for enumerating local machine
//  stores on a remote computer. For example, "\\ComputerName" or
//  "ComputerName".  The leading "\\" backslashes are optional in the
//  ComputerName.
//
//  For CERT_SYSTEM_STORE_SERVICES or CERT_SYSTEM_STORE_USERS,
//  if pvSystemStoreLocationPara is NULL, then,
//  enumerates both the service/user names and the stores for each service/user
//  name. Otherwise, pvSystemStoreLocationPara is a unicode string specifying a
//  remote computer name and/or service/user name. For example:
//      "ServiceName"
//      "\\ComputerName" or "ComputerName\"
//      "ComputerName\ServiceName"
//  Note, if only the ComputerName is specified, then, it must have either
//  the leading "\\" backslashes or a trailing backslash. Otherwise, its
//  interpretted as the ServiceName or UserName.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertEnumSystemStore(
    _In_ DWORD dwFlags,
    _In_opt_ void *pvSystemStoreLocationPara,
    _Inout_opt_ void *pvArg,
    __callback PFN_CERT_ENUM_SYSTEM_STORE pfnEnum
    );

//+-------------------------------------------------------------------------
//  Enumerate the physical stores for the specified system store.
//
//  The upper word of the dwFlags parameter is used to specify the location of
//  the system store.
//
//  If CERT_SYSTEM_STORE_RELOCATE_FLAG is set in dwFlags, pvSystemStore
//  points to a CERT_SYSTEM_STORE_RELOCATE_PARA data structure. Otherwise,
//  pvSystemStore points to a null terminated UNICODE string.
//
//  See CertRegisterSystemStore for details on prepending a ServiceName
//  and/or ComputerName to the system store name.
//
//  If the system store location only supports system stores and doesn't
//  support physical stores, LastError is set to ERROR_CALL_NOT_IMPLEMENTED.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertEnumPhysicalStore(
    _In_ const void *pvSystemStore,
    _In_ DWORD dwFlags,
    _Inout_opt_ void *pvArg,
    __callback PFN_CERT_ENUM_PHYSICAL_STORE pfnEnum
    );

//+-------------------------------------------------------------------------
//  Certificate System Store Installable Functions
//
//  The CERT_SYSTEM_STORE_LOCATION_MASK bits in the dwFlags parameter passed
//  to the CertOpenStore(for "System", "SystemRegistry" or "Physical"
//  Provider), CertRegisterSystemStore,
//  CertUnregisterSystemStore, CertEnumSystemStore, CertRegisterPhysicalStore,
//  CertUnregisterPhysicalStore and CertEnumPhysicalStore APIs is used as the
//  constant pszOID value passed to the OID installable functions.
//  Therefore, the pszOID is restricted to a constant <= (LPCSTR) 0x0FFF.
//
//  The EncodingType is 0.
//--------------------------------------------------------------------------

// Installable System Store Provider OID pszFuncNames.
#define CRYPT_OID_OPEN_SYSTEM_STORE_PROV_FUNC   "CertDllOpenSystemStoreProv"
#define CRYPT_OID_REGISTER_SYSTEM_STORE_FUNC    "CertDllRegisterSystemStore"
#define CRYPT_OID_UNREGISTER_SYSTEM_STORE_FUNC  "CertDllUnregisterSystemStore"
#define CRYPT_OID_ENUM_SYSTEM_STORE_FUNC        "CertDllEnumSystemStore"
#define CRYPT_OID_REGISTER_PHYSICAL_STORE_FUNC  "CertDllRegisterPhysicalStore"
#define CRYPT_OID_UNREGISTER_PHYSICAL_STORE_FUNC "CertDllUnregisterPhysicalStore"
#define CRYPT_OID_ENUM_PHYSICAL_STORE_FUNC      "CertDllEnumPhysicalStore"

// CertDllOpenSystemStoreProv has the same function signature as the
// installable "CertDllOpenStoreProv" function. See CertOpenStore for
// more details.

// CertDllRegisterSystemStore has the same function signature as
// CertRegisterSystemStore.
//
// The "SystemStoreLocation" REG_SZ value must also be set for registered
// CertDllEnumSystemStore OID functions.
#define CRYPT_OID_SYSTEM_STORE_LOCATION_VALUE_NAME  L"SystemStoreLocation"

// The remaining Register, Enum and Unregister OID installable functions
// have the same signature as their Cert Store API counterpart.


//+=========================================================================
//  Enhanced Key Usage Helper Functions
//==========================================================================

//+-------------------------------------------------------------------------
//  Get the enhanced key usage extension or property from the certificate
//  and decode.
//
//  If the CERT_FIND_EXT_ONLY_ENHKEY_USAGE_FLAG is set, then, only get the
//  extension.
//
//  If the CERT_FIND_PROP_ONLY_ENHKEY_USAGE_FLAG is set, then, only get the
//  property.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertGetEnhancedKeyUsage(
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ DWORD dwFlags,
    _Out_writes_bytes_to_opt_(*pcbUsage, *pcbUsage) PCERT_ENHKEY_USAGE pUsage,
    _Inout_ DWORD *pcbUsage
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Set the enhanced key usage property for the certificate.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertSetEnhancedKeyUsage(
    _In_ PCCERT_CONTEXT pCertContext,
    _In_opt_ PCERT_ENHKEY_USAGE pUsage
    );

//+-------------------------------------------------------------------------
//  Add the usage identifier to the certificate's enhanced key usage property.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertAddEnhancedKeyUsageIdentifier(
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ LPCSTR pszUsageIdentifier
    );


//+-------------------------------------------------------------------------
//  Remove the usage identifier from the certificate's enhanced key usage
//  property.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertRemoveEnhancedKeyUsageIdentifier(
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ LPCSTR pszUsageIdentifier
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+---------------------------------------------------------------------------
//
//
//  Takes an array of certs and returns an array of usages
//  which consists of the intersection of the valid usages for each cert.
//  If each cert is good for all possible usages then the cNumOIDs is set to -1.
//
//----------------------------------------------------------------------------
WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertGetValidUsages(
    _In_ DWORD cCerts,
    _In_reads_(cCerts) PCCERT_CONTEXT *rghCerts,
    _Out_ int *cNumOIDs,
    _Out_writes_bytes_to_opt_(*pcbOIDs, *pcbOIDs) LPSTR *rghOIDs,
    _Inout_ DWORD *pcbOIDs);

//+=========================================================================
//  Cryptographic Message helper functions for verifying and signing a
//  CTL.
//==========================================================================

//+-------------------------------------------------------------------------
//  Get and verify the signer of a cryptographic message.
//
//  To verify a CTL, the hCryptMsg is obtained from the CTL_CONTEXT's
//  hCryptMsg field.
//
//  If CMSG_TRUSTED_SIGNER_FLAG is set, then, treat the Signer stores as being
//  trusted and only search them to find the certificate corresponding to the
//  signer's issuer and serial number.  Otherwise, the SignerStores are
//  optionally provided to supplement the message's store of certificates.
//  If a signer certificate is found, its public key is used to verify
//  the message signature. The CMSG_SIGNER_ONLY_FLAG can be set to
//  return the signer without doing the signature verify.
//
//  If CMSG_USE_SIGNER_INDEX_FLAG is set, then, only get the signer specified
//  by *pdwSignerIndex. Otherwise, iterate through all the signers
//  until a signer verifies or no more signers.
//
//  For a verified signature, *ppSigner is updated with certificate context
//  of the signer and *pdwSignerIndex is updated with the index of the signer.
//  ppSigner and/or pdwSignerIndex can be NULL, indicating the caller isn't
//  interested in getting the CertContext and/or index of the signer.
//--------------------------------------------------------------------------

WINCRYPT32API
_Success_(return == TRUE)
BOOL
WINAPI
CryptMsgGetAndVerifySigner(
    _In_ HCRYPTMSG hCryptMsg,
    _In_ DWORD cSignerStore,
    _In_reads_opt_(cSignerStore) HCERTSTORE *rghSignerStore,
    _In_ DWORD dwFlags,
    _Outptr_opt_ PCCERT_CONTEXT *ppSigner,
    _Inout_opt_ DWORD *pdwSignerIndex
    );

#define CMSG_TRUSTED_SIGNER_FLAG            0x1
#define CMSG_SIGNER_ONLY_FLAG               0x2
#define CMSG_USE_SIGNER_INDEX_FLAG          0x4

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Sign an encoded CTL.
//
//  The pbCtlContent can be obtained via a CTL_CONTEXT's pbCtlContent
//  field or via a CryptEncodeObject(PKCS_CTL or PKCS_SORTED_CTL).
//
//  CMSG_CMS_ENCAPSULATED_CTL_FLAG can be set to encode a CMS compatible
//  V3 SignedData message.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptMsgSignCTL(
    _In_ DWORD dwMsgEncodingType,
    _In_reads_bytes_(cbCtlContent) BYTE *pbCtlContent,
    _In_ DWORD cbCtlContent,
    _In_ PCMSG_SIGNED_ENCODE_INFO pSignInfo,
    _In_ DWORD dwFlags,
    _Out_writes_bytes_to_opt_(*pcbEncoded, *pcbEncoded) BYTE *pbEncoded,
    _Inout_ DWORD *pcbEncoded
    );

// When set, CTL inner content is encapsulated within an OCTET STRING
#define CMSG_CMS_ENCAPSULATED_CTL_FLAG  0x00008000

//+-------------------------------------------------------------------------
//  Encode the CTL and create a signed message containing the encoded CTL.
//
//  Set CMSG_ENCODE_SORTED_CTL_FLAG if the CTL entries are to be sorted
//  before encoding. This flag should be set, if the
//  CertFindSubjectInSortedCTL or CertEnumSubjectInSortedCTL APIs will
//  be called. If the identifier for the CTL entries is a hash, such as,
//  MD5 or SHA1, then, CMSG_ENCODE_HASHED_SUBJECT_IDENTIFIER_FLAG should
//  also be set.
//
//  CMSG_CMS_ENCAPSULATED_CTL_FLAG can be set to encode a CMS compatible
//  V3 SignedData message.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptMsgEncodeAndSignCTL(
    _In_ DWORD dwMsgEncodingType,
    _In_ PCTL_INFO pCtlInfo,
    _In_ PCMSG_SIGNED_ENCODE_INFO pSignInfo,
    _In_ DWORD dwFlags,
    _Out_writes_bytes_to_opt_(*pcbEncoded, *pcbEncoded) BYTE *pbEncoded,
    _Inout_ DWORD *pcbEncoded
    );

//  The following flag is set if the CTL is to be encoded with sorted
//  trusted subjects and the szOID_SORTED_CTL extension is inserted containing
//  sorted offsets to the encoded subjects.
#define CMSG_ENCODE_SORTED_CTL_FLAG                     0x1

//  If the above sorted flag is set, then, the following flag should also
//  be set if the identifier for the TrustedSubjects is a hash,
//  such as, MD5 or SHA1.
#define CMSG_ENCODE_HASHED_SUBJECT_IDENTIFIER_FLAG      0x2


//+-------------------------------------------------------------------------
//  Returns TRUE if the SubjectIdentifier exists in the CTL. Optionally
//  returns a pointer to and byte count of the Subject's encoded attributes.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertFindSubjectInSortedCTL(
    _In_ PCRYPT_DATA_BLOB pSubjectIdentifier,
    _In_ PCCTL_CONTEXT pCtlContext,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved,
    _Out_opt_ PCRYPT_DER_BLOB pEncodedAttributes
    );

//+-------------------------------------------------------------------------
//  Enumerates through the sequence of TrustedSubjects in a CTL context
//  created with CERT_CREATE_CONTEXT_SORTED_FLAG set.
//
//  To start the enumeration, *ppvNextSubject must be NULL. Upon return,
//  *ppvNextSubject is updated to point to the next TrustedSubject in
//  the encoded sequence.
//
//  Returns FALSE for no more subjects or invalid arguments.
//
//  Note, the returned DER_BLOBs point directly into the encoded
//  bytes (not allocated, and must not be freed).
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertEnumSubjectInSortedCTL(
    _In_ PCCTL_CONTEXT pCtlContext,
    _Inout_ void **ppvNextSubject,
    _Out_opt_ PCRYPT_DER_BLOB pSubjectIdentifier,
    _Out_opt_ PCRYPT_DER_BLOB pEncodedAttributes
    );


//+=========================================================================
//  Certificate Verify CTL Usage Data Structures and APIs
//==========================================================================

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

typedef struct _CTL_VERIFY_USAGE_PARA {
    DWORD                   cbSize;
    CRYPT_DATA_BLOB         ListIdentifier;     // OPTIONAL
    DWORD                   cCtlStore;
    HCERTSTORE              *rghCtlStore;       // OPTIONAL
    DWORD                   cSignerStore;
    HCERTSTORE              *rghSignerStore;    // OPTIONAL
} CTL_VERIFY_USAGE_PARA, *PCTL_VERIFY_USAGE_PARA;

typedef struct _CTL_VERIFY_USAGE_STATUS {
    DWORD                   cbSize;
    DWORD                   dwError;
    DWORD                   dwFlags;
    PCCTL_CONTEXT           *ppCtl;             // IN OUT OPTIONAL
    DWORD                   dwCtlEntryIndex;
    PCCERT_CONTEXT          *ppSigner;          // IN OUT OPTIONAL
    DWORD                   dwSignerIndex;
} CTL_VERIFY_USAGE_STATUS, *PCTL_VERIFY_USAGE_STATUS;

#define CERT_VERIFY_INHIBIT_CTL_UPDATE_FLAG     0x1
#define CERT_VERIFY_TRUSTED_SIGNERS_FLAG        0x2
#define CERT_VERIFY_NO_TIME_CHECK_FLAG          0x4
#define CERT_VERIFY_ALLOW_MORE_USAGE_FLAG       0x8

#define CERT_VERIFY_UPDATED_CTL_FLAG            0x1

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Verify that a subject is trusted for the specified usage by finding a
//  signed and time valid CTL with the usage identifiers and containing the
//  the subject. A subject can be identified by either its certificate context
//  or any identifier such as its SHA1 hash.
//
//  See CertFindSubjectInCTL for definition of dwSubjectType and pvSubject
//  parameters.
//
//  Via pVerifyUsagePara, the caller can specify the stores to be searched
//  to find the CTL. The caller can also specify the stores containing
//  acceptable CTL signers. By setting the ListIdentifier, the caller
//  can also restrict to a particular signer CTL list.
//
//  Via pVerifyUsageStatus, the CTL containing the subject, the subject's
//  index into the CTL's array of entries, and the signer of the CTL
//  are returned. If the caller is not interested, ppCtl and ppSigner can be set
//  to NULL. Returned contexts must be freed via the store's free context APIs.
//
//  If the CERT_VERIFY_INHIBIT_CTL_UPDATE_FLAG isn't set, then, a time
//  invalid CTL in one of the CtlStores may be replaced. When replaced, the
//  CERT_VERIFY_UPDATED_CTL_FLAG is set in pVerifyUsageStatus->dwFlags.
//
//  If the CERT_VERIFY_TRUSTED_SIGNERS_FLAG is set, then, only the
//  SignerStores specified in pVerifyUsageStatus are searched to find
//  the signer. Otherwise, the SignerStores provide additional sources
//  to find the signer's certificate.
//
//  If CERT_VERIFY_NO_TIME_CHECK_FLAG is set, then, the CTLs aren't checked
//  for time validity.
//
//  If CERT_VERIFY_ALLOW_MORE_USAGE_FLAG is set, then, the CTL may contain
//  additional usage identifiers than specified by pSubjectUsage. Otherwise,
//  the found CTL will contain the same usage identifers and no more.
//
//  CertVerifyCTLUsage will be implemented as a dispatcher to OID installable
//  functions. First, it will try to find an OID function matching the first
//  usage object identifier in the pUsage sequence. Next, it will dispatch
//  to the default CertDllVerifyCTLUsage functions.
//
//  If the subject is trusted for the specified usage, then, TRUE is
//  returned. Otherwise, FALSE is returned with dwError set to one of the
//  following:
//      CRYPT_E_NO_VERIFY_USAGE_DLL
//      CRYPT_E_NO_VERIFY_USAGE_CHECK
//      CRYPT_E_VERIFY_USAGE_OFFLINE
//      CRYPT_E_NOT_IN_CTL
//      CRYPT_E_NO_TRUSTED_SIGNER
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertVerifyCTLUsage(
    _In_ DWORD dwEncodingType,
    _In_ DWORD dwSubjectType,
    _In_ void *pvSubject,
    _In_ PCTL_USAGE pSubjectUsage,
    _In_ DWORD dwFlags,
    _In_opt_ PCTL_VERIFY_USAGE_PARA pVerifyUsagePara,
    _Inout_ PCTL_VERIFY_USAGE_STATUS pVerifyUsageStatus
    );


//+=========================================================================
//  Certificate Revocation Data Structures and APIs
//==========================================================================

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  This data structure is updated by a CRL revocation type handler
//  with the base and possibly the delta CRL used.
//--------------------------------------------------------------------------
typedef struct _CERT_REVOCATION_CRL_INFO {
    DWORD                   cbSize;
    PCCRL_CONTEXT           pBaseCrlContext;
    PCCRL_CONTEXT           pDeltaCrlContext;

    // When revoked, points to entry in either of the above CRL contexts.
    // Don't free.
    PCRL_ENTRY              pCrlEntry;
    BOOL                    fDeltaCrlEntry; // TRUE if in pDeltaCrlContext
} CERT_REVOCATION_CRL_INFO, *PCERT_REVOCATION_CRL_INFO;


//+-------------------------------------------------------------------------
//  This data structure is optionally pointed to by the pChainPara field
//  in the CERT_REVOCATION_PARA and CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO
//  data structures.
//
//  Its struct definition follows the CertGetCertificateChain() API
//  definition below.
//--------------------------------------------------------------------------
typedef struct _CERT_REVOCATION_CHAIN_PARA
    CERT_REVOCATION_CHAIN_PARA,
    *PCERT_REVOCATION_CHAIN_PARA;

//+-------------------------------------------------------------------------
//  The following data structure may be passed to CertVerifyRevocation to
//  assist in finding the issuer of the context to be verified.
//
//  When pIssuerCert is specified, pIssuerCert is the issuer of
//  rgpvContext[cContext - 1].
//
//  When cCertStore and rgCertStore are specified, these stores may contain
//  an issuer certificate.
//
//  When hCrlStore is specified then a handler which uses CRLs can search this
//  store for them
//
//  When pftTimeToUse is specified then the handler (if possible) must determine
//  revocation status relative to the time given otherwise the answer may be
//  independent of time or relative to current time
//--------------------------------------------------------------------------
typedef struct _CERT_REVOCATION_PARA {
    DWORD                       cbSize;
    PCCERT_CONTEXT              pIssuerCert;
    DWORD                       cCertStore;
    HCERTSTORE                  *rgCertStore;
    HCERTSTORE                  hCrlStore;
    LPFILETIME                  pftTimeToUse;

#ifdef CERT_REVOCATION_PARA_HAS_EXTRA_FIELDS
    // Note, if you #define CERT_REVOCATION_PARA_HAS_EXTRA_FIELDS, then, you
    // must zero all unused fields in this data structure.
    // More fields could be added in a future release.

    // 0 uses revocation handler's default timeout.
    DWORD                       dwUrlRetrievalTimeout;  // milliseconds

    // When set, checks and attempts to retrieve a CRL where
    // ThisUpdate >= (CurrentTime - dwFreshnessTime). Otherwise, defaults
    // to using the CRL's NextUpdate.
    BOOL                        fCheckFreshnessTime;
    DWORD                       dwFreshnessTime;        // seconds

    // If NULL, revocation handler gets the current time
    LPFILETIME                  pftCurrentTime;

    // If nonNULL, a CRL revocation type handler updates with the base and
    // possibly the delta CRL used. Note, *pCrlInfo must be initialized
    // by the caller. Any nonNULL CRL contexts are freed. Any updated
    // CRL contexts must be freed by the caller.
    //
    // The CRL info is only applicable to the last context checked. If
    // interested in this information, then, CertVerifyRevocation should be
    // called with cContext = 1.
    PCERT_REVOCATION_CRL_INFO   pCrlInfo;

    // If nonNULL, any cached information before this time is considered
    // time invalid and forces a wire retrieval.
    LPFILETIME                  pftCacheResync;

    // If nonNULL, CertGetCertificateChain() parameters used by the caller.
    // Enables independent OCSP signer certificate chain verification.
    PCERT_REVOCATION_CHAIN_PARA pChainPara;
#endif
} CERT_REVOCATION_PARA, *PCERT_REVOCATION_PARA;



//+-------------------------------------------------------------------------
//  The following data structure is returned by CertVerifyRevocation to
//  specify the status of the revoked or unchecked context. Review the
//  following CertVerifyRevocation comments for details.
//
//  Upon input to CertVerifyRevocation, cbSize must be set to a size
//  >= (offsetof(CERT_REVOCATION_STATUS, dwReason) + sizeof(DWORD) ).
//  Otherwise, CertVerifyRevocation returns FALSE and sets LastError to
//  E_INVALIDARG.
//
//  Upon input to the installed or registered CRYPT_OID_VERIFY_REVOCATION_FUNC
//  functions, the dwIndex, dwError and dwReason have been zero'ed.
//  If present, fHasFreshnessTime and dwFreshnessTime have been zero'ed.
//--------------------------------------------------------------------------
typedef struct _CERT_REVOCATION_STATUS {
    DWORD                   cbSize;
    DWORD                   dwIndex;
    DWORD                   dwError;
    DWORD                   dwReason;

    // Depending on cbSize, the following fields may optionally be returned.

    // The Freshness time is only applicable to the last context checked. If
    // interested in this information, then, CertVerifyRevocation should be
    // called with cContext = 1.
    //
    // fHasFreshnessTime is only set if we are able to retrieve revocation
    // information. For a CRL its CurrentTime - ThisUpdate.
    BOOL                    fHasFreshnessTime;
    DWORD                   dwFreshnessTime;    // seconds
} CERT_REVOCATION_STATUS, *PCERT_REVOCATION_STATUS;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Verifies the array of contexts for revocation. The dwRevType parameter
//  indicates the type of the context data structure passed in rgpvContext.
//  Currently only the revocation of certificates is defined.
//
//  If the CERT_VERIFY_REV_CHAIN_FLAG flag is set, then, CertVerifyRevocation
//  is verifying a chain of certs where, rgpvContext[i + 1] is the issuer
//  of rgpvContext[i]. Otherwise, CertVerifyRevocation makes no assumptions
//  about the order of the contexts.
//
//  To assist in finding the issuer, the pRevPara may optionally be set. See
//  the CERT_REVOCATION_PARA data structure for details.
//
//  The contexts must contain enough information to allow the
//  installable or registered revocation DLLs to find the revocation server. For
//  certificates, this information would normally be conveyed in an
//  extension such as the IETF's AuthorityInfoAccess extension.
//
//  CertVerifyRevocation returns TRUE if all of the contexts were successfully
//  checked and none were revoked. Otherwise, returns FALSE and updates the
//  returned pRevStatus data structure as follows:
//    dwIndex
//      Index of the first context that was revoked or unable to
//      be checked for revocation
//    dwError
//      Error status. LastError is also set to this error status.
//      dwError can be set to one of the following error codes defined
//      in winerror.h:
//        ERROR_SUCCESS - good context
//        CRYPT_E_REVOKED - context was revoked. dwReason contains the
//           reason for revocation
//        CRYPT_E_REVOCATION_OFFLINE - unable to connect to the
//           revocation server
//        CRYPT_E_NOT_IN_REVOCATION_DATABASE - the context to be checked
//           was not found in the revocation server's database.
//        CRYPT_E_NO_REVOCATION_CHECK - the called revocation function
//           wasn't able to do a revocation check on the context
//        CRYPT_E_NO_REVOCATION_DLL - no installed or registered Dll was
//           found to verify revocation
//    dwReason
//      The dwReason is currently only set for CRYPT_E_REVOKED and contains
//      the reason why the context was revoked. May be one of the following
//      CRL reasons defined by the CRL Reason Code extension ("2.5.29.21")
//          CRL_REASON_UNSPECIFIED              0
//          CRL_REASON_KEY_COMPROMISE           1
//          CRL_REASON_CA_COMPROMISE            2
//          CRL_REASON_AFFILIATION_CHANGED      3
//          CRL_REASON_SUPERSEDED               4
//          CRL_REASON_CESSATION_OF_OPERATION   5
//          CRL_REASON_CERTIFICATE_HOLD         6
//
//  For each entry in rgpvContext, CertVerifyRevocation iterates
//  through the CRYPT_OID_VERIFY_REVOCATION_FUNC
//  function set's list of installed DEFAULT functions.
//  CryptGetDefaultOIDFunctionAddress is called with pwszDll = NULL. If no
//  installed functions are found capable of doing the revocation verification,
//  CryptVerifyRevocation iterates through CRYPT_OID_VERIFY_REVOCATION_FUNC's
//  list of registered DEFAULT Dlls. CryptGetDefaultOIDDllList is called to
//  get the list. CryptGetDefaultOIDFunctionAddress is called to load the Dll.
//
//  The called functions have the same signature as CertVerifyRevocation. A
//  called function returns TRUE if it was able to successfully check all of
//  the contexts and none were revoked. Otherwise, the called function returns
//  FALSE and updates pRevStatus. dwIndex is set to the index of
//  the first context that was found to be revoked or unable to be checked.
//  dwError and LastError are updated. For CRYPT_E_REVOKED, dwReason
//  is updated. Upon input to the called function, dwIndex, dwError and
//  dwReason have been zero'ed. cbSize has been checked to be >=
//  sizeof(CERT_REVOCATION_STATUS).
//
//  If the called function returns FALSE, and dwError isn't set to
//  CRYPT_E_REVOKED, then, CertVerifyRevocation either continues on to the
//  next DLL in the list for a returned dwIndex of 0 or for a returned
//  dwIndex > 0, restarts the process of finding a verify function by
//  advancing the start of the context array to the returned dwIndex and
//  decrementing the count of remaining contexts.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertVerifyRevocation(
    _In_ DWORD dwEncodingType,
    _In_ DWORD dwRevType,
    _In_ DWORD cContext,
    _In_reads_(cContext) PVOID rgpvContext[],
    _In_ DWORD dwFlags,
    _In_opt_ PCERT_REVOCATION_PARA pRevPara,
    _Inout_ PCERT_REVOCATION_STATUS pRevStatus
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Revocation types
//--------------------------------------------------------------------------
#define CERT_CONTEXT_REVOCATION_TYPE        1

//+-------------------------------------------------------------------------
//  When the following flag is set, rgpvContext[] consists of a chain
//  of certificates, where rgpvContext[i + 1] is the issuer of rgpvContext[i].
//--------------------------------------------------------------------------
#define CERT_VERIFY_REV_CHAIN_FLAG                      0x00000001

//+-------------------------------------------------------------------------
// CERT_VERIFY_CACHE_ONLY_BASED_REVOCATION prevents the revocation handler from
// accessing any network based resources for revocation checking
//--------------------------------------------------------------------------
#define CERT_VERIFY_CACHE_ONLY_BASED_REVOCATION         0x00000002

//+-------------------------------------------------------------------------
//  By default, the dwUrlRetrievalTimeout in pRevPara is the timeout used
//  for each URL wire retrieval. When the following flag is set,
//  dwUrlRetrievalTimeout is the accumulative timeout across all URL wire
//  retrievals.
//--------------------------------------------------------------------------
#define CERT_VERIFY_REV_ACCUMULATIVE_TIMEOUT_FLAG       0x00000004

//+-------------------------------------------------------------------------
//  When the following flag is set, only OCSP responses are used for
//  doing revocation checking. If the certificate doesn't have any
//  OCSP AIA URLs, dwError is set to CRYPT_E_NOT_IN_REVOCATION_DATABASE.
//--------------------------------------------------------------------------
#define CERT_VERIFY_REV_SERVER_OCSP_FLAG                0x00000008

//+-------------------------------------------------------------------------
//  When the following flag is set, only the OCSP AIA URL is used if
//  present in the subject. If the subject doesn't have an OCSP AIA URL, then,
//  the CDP URLs are used.
//--------------------------------------------------------------------------
#define CERT_VERIFY_REV_NO_OCSP_FAILOVER_TO_CRL_FLAG    0x00000010

//+-------------------------------------------------------------------------
//  When the following flag is set, only wire retrieval for OCSP responses.
//--------------------------------------------------------------------------
#define CERT_VERIFY_REV_SERVER_OCSP_WIRE_ONLY_FLAG      0x00000020


//+-------------------------------------------------------------------------
//  CERT_CONTEXT_REVOCATION_TYPE
//
//  pvContext points to a const CERT_CONTEXT.
//--------------------------------------------------------------------------

//+=========================================================================
//  Certificate Helper APIs
//==========================================================================

//+-------------------------------------------------------------------------
//  Compare two multiple byte integer blobs to see if they are identical.
//
//  Before doing the comparison, leading zero bytes are removed from a
//  positive number and leading 0xFF bytes are removed from a negative
//  number.
//
//  The multiple byte integers are treated as Little Endian. pbData[0] is the
//  least significant byte and pbData[cbData - 1] is the most significant
//  byte.
//
//  Returns TRUE if the integer blobs are identical after removing leading
//  0 or 0xFF bytes.
//--------------------------------------------------------------------------
BOOL
WINAPI
CertCompareIntegerBlob(
    _In_ PCRYPT_INTEGER_BLOB pInt1,
    _In_ PCRYPT_INTEGER_BLOB pInt2
    );

//+-------------------------------------------------------------------------
//  Compare two certificates to see if they are identical.
//
//  Since a certificate is uniquely identified by its Issuer and SerialNumber,
//  these are the only fields needing to be compared.
//
//  Returns TRUE if the certificates are identical.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertCompareCertificate(
    _In_ DWORD dwCertEncodingType,
    _In_ PCERT_INFO pCertId1,
    _In_ PCERT_INFO pCertId2
    );

//+-------------------------------------------------------------------------
//  Compare two certificate names to see if they are identical.
//
//  Returns TRUE if the names are identical.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertCompareCertificateName(
    _In_ DWORD dwCertEncodingType,
    _In_ PCERT_NAME_BLOB pCertName1,
    _In_ PCERT_NAME_BLOB pCertName2
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Compare the attributes in the certificate name with the specified
//  Relative Distinguished Name's (CERT_RDN) array of attributes.
//  The comparison iterates through the CERT_RDN attributes and looks for an
//  attribute match in any of the certificate name's RDNs.
//  Returns TRUE if all the attributes are found and match.
//
//  The CERT_RDN_ATTR fields can have the following special values:
//    pszObjId == NULL              - ignore the attribute object identifier
//    dwValueType == RDN_ANY_TYPE   - ignore the value type
//
//  CERT_CASE_INSENSITIVE_IS_RDN_ATTRS_FLAG should be set to do
//  a case insensitive match. Otherwise, defaults to an exact, case sensitive
//  match.
//
//  CERT_UNICODE_IS_RDN_ATTRS_FLAG should be set if the pRDN was initialized
//  with unicode strings as for CryptEncodeObject(X509_UNICODE_NAME).
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertIsRDNAttrsInCertificateName(
    _In_ DWORD dwCertEncodingType,
    _In_ DWORD dwFlags,
    _In_ PCERT_NAME_BLOB pCertName,
    _In_ PCERT_RDN pRDN
    );

#define CERT_UNICODE_IS_RDN_ATTRS_FLAG              0x1
#define CERT_CASE_INSENSITIVE_IS_RDN_ATTRS_FLAG     0x2

//+-------------------------------------------------------------------------
//  Compare two public keys to see if they are identical.
//
//  Returns TRUE if the keys are identical.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertComparePublicKeyInfo(
    _In_ DWORD dwCertEncodingType,
    _In_ PCERT_PUBLIC_KEY_INFO pPublicKey1,
    _In_ PCERT_PUBLIC_KEY_INFO pPublicKey2
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Get the public/private key's bit length.
//
//  Returns 0 if unable to determine the key's length.
//--------------------------------------------------------------------------
WINCRYPT32API
DWORD
WINAPI
CertGetPublicKeyLength(
    _In_ DWORD dwCertEncodingType,
    _In_ PCERT_PUBLIC_KEY_INFO pPublicKey
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Verify the signature of a subject certificate or a CRL using the
//  public key info
//
//  Returns TRUE for a valid signature.
//
//  hCryptProv specifies the crypto provider to use to verify the signature.
//  It doesn't need to use a private key.
//--------------------------------------------------------------------------
WINCRYPT32API
_Must_inspect_result_
BOOL
WINAPI
CryptVerifyCertificateSignature(
    _In_opt_ HCRYPTPROV_LEGACY hCryptProv,
    _In_ DWORD dwCertEncodingType,
    _In_reads_bytes_(cbEncoded) const BYTE *pbEncoded,
    _In_ DWORD cbEncoded,
    _In_ PCERT_PUBLIC_KEY_INFO pPublicKey
    );

//+-------------------------------------------------------------------------
//  Verify the signature of a subject certificate, CRL, certificate request
//  or keygen request using the issuer's public key.
//
//  Returns TRUE for a valid signature.
//
//  The subject can be an encoded blob or a context for a certificate or CRL.
//  For a subject certificate context, if the certificate is missing
//  inheritable PublicKey Algorithm Parameters, the context's
//  CERT_PUBKEY_ALG_PARA_PROP_ID is updated with the issuer's public key
//  algorithm parameters for a valid signature.
//
//  The issuer can be a pointer to a CERT_PUBLIC_KEY_INFO, certificate
//  context or a chain context.
//
//  hCryptProv specifies the crypto provider to use to verify the signature.
//  Its private key isn't used. If hCryptProv is NULL, a default
//  provider is picked according to the PublicKey Algorithm OID.
//
//  If the signature algorithm is a hashing algorithm, then, the
//  signature is expected to contain the hash octets. Only dwIssuerType
//  of CRYPT_VERIFY_CERT_SIGN_ISSUER_NULL may be specified
//  to verify this no signature case. If any other dwIssuerType is
//  specified, the verify will fail with LastError set to E_INVALIDARG.
//--------------------------------------------------------------------------
WINCRYPT32API
_Must_inspect_result_
BOOL
WINAPI
CryptVerifyCertificateSignatureEx(
    _In_opt_ HCRYPTPROV_LEGACY hCryptProv,
    _In_ DWORD dwCertEncodingType,
    _In_ DWORD dwSubjectType,
    _In_ void *pvSubject,
    _In_ DWORD dwIssuerType,
    _In_opt_ void *pvIssuer,
    _In_ DWORD dwFlags,
    _Inout_opt_ void *pvExtra
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

// Subject Types
#define CRYPT_VERIFY_CERT_SIGN_SUBJECT_BLOB         1
    // pvSubject :: PCRYPT_DATA_BLOB
#define CRYPT_VERIFY_CERT_SIGN_SUBJECT_CERT         2
    // pvSubject :: PCCERT_CONTEXT
#define CRYPT_VERIFY_CERT_SIGN_SUBJECT_CRL          3
    // pvSubject :: PCCRL_CONTEXT
#define CRYPT_VERIFY_CERT_SIGN_SUBJECT_OCSP_BASIC_SIGNED_RESPONSE   4
    // pvSubject :: POCSP_BASIC_SIGNED_RESPONSE_INFO

// Issuer Types
#define CRYPT_VERIFY_CERT_SIGN_ISSUER_PUBKEY        1
    // pvIssuer :: PCERT_PUBLIC_KEY_INFO
#define CRYPT_VERIFY_CERT_SIGN_ISSUER_CERT          2
    // pvIssuer :: PCCERT_CONTEXT
#define CRYPT_VERIFY_CERT_SIGN_ISSUER_CHAIN         3
    // pvIssuer :: PCCERT_CHAIN_CONTEXT
#define CRYPT_VERIFY_CERT_SIGN_ISSUER_NULL          4
    // pvIssuer :: NULL

//
// If the following flag is set and a MD2 or MD4 signature hash is
// detected, then, this API fails and sets LastError to NTE_BAD_ALGID
//
// This API first does the signature verification check. If the signature 
// verification succeeds and the following flag is set, it then checks for a
// MD2 or MD4 hash. For a MD2 or MD4 hash FALSE is returned with LastError set
// to NTE_BAD_ALGID. This error will only be set if MD2 or MD4 is detected.
// If NTE_BAD_ALGID is returned, then, the MD2 or MD4 signature verified.
// This allows the caller to conditionally allow MD2 or MD4.
//
#define CRYPT_VERIFY_CERT_SIGN_DISABLE_MD2_MD4_FLAG     0x00000001

//
// When the following flag is set, the strong signature properties are
// also set on the Subject. Only applicable to the
// CRYPT_VERIFY_CERT_SIGN_SUBJECT_CRL Subject Type.
//
//  The strong signature properties are:
//    - CERT_SIGN_HASH_CNG_ALG_PROP_ID
//    - CERT_ISSUER_PUB_KEY_BIT_LENGTH_PROP_ID
//
#define CRYPT_VERIFY_CERT_SIGN_SET_STRONG_PROPERTIES_FLAG 0x00000002

//
// When the following flag is set, the strong signature properties are also
// returned. Only applicable to the
// CRYPT_VERIFY_CERT_SIGN_SUBJECT_OCSP_BASIC_SIGNED_RESPONSE Subject Type.
//
// pvExtra points to a pointer to CRYPT_VERIFY_CERT_SIGN_VERIFY_PROPERTIES_INFO.
//  ie, PCRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO *ppStrongPropertiesInfo.
// The returned pointer is freed via CryptMemFree().
//
//  The strong signature properties are:
//    - CERT_SIGN_HASH_CNG_ALG_PROP_ID
//    - CERT_ISSUER_PUB_KEY_BIT_LENGTH_PROP_ID
//
#define CRYPT_VERIFY_CERT_SIGN_RETURN_STRONG_PROPERTIES_FLAG 0x00000004

typedef struct _CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO {
    // CERT_SIGN_HASH_CNG_ALG_PROP_ID
    CRYPT_DATA_BLOB CertSignHashCNGAlgPropData;

    // CERT_ISSUER_PUB_KEY_BIT_LENGTH_PROP_ID
    CRYPT_DATA_BLOB CertIssuerPubKeyBitLengthPropData;
} CRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO,
    *PCRYPT_VERIFY_CERT_SIGN_STRONG_PROPERTIES_INFO;

#define CRYPT_VERIFY_CERT_SIGN_CHECK_WEAK_HASH_FLAG         0x00000008
typedef struct _CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO {
    DWORD   cCNGHashAlgid;
    PCWSTR  *rgpwszCNGHashAlgid;

    // If not weak, dwWeakIndex is set to cCNGHashAlgid. Otherwise,
    // index into the above array.
    DWORD   dwWeakIndex;
} CRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO,
    *PCRYPT_VERIFY_CERT_SIGN_WEAK_HASH_INFO;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Checks if the specified hash algorithm and the signing certificate's
//  public key algorithm can be used to do a strong signature.
//
//  Returns TRUE if the hash algorithm and certificate public key algorithm
//  satisfy the strong signature requirements.
//
//  pwszCNGHashAlgid is the CNG hash algorithm identifier string, for example,
//  BCRYPT_SHA256_ALGORITHM (L"SHA256")
//
//  The CNG hash algorithm identifier string can be empty (L"") to only check
//  if the certificate's public key is strong.
//
//  The SigningCert can be NULL to only check if the CNG hash algorithm is
//  strong.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertIsStrongHashToSign(
    _In_ PCCERT_STRONG_SIGN_PARA pStrongSignPara,
    _In_ LPCWSTR pwszCNGHashAlgid,
    _In_opt_ PCCERT_CONTEXT pSigningCert
    );

//+-------------------------------------------------------------------------
//  Compute the hash of the "to be signed" information in the encoded
//  signed content (CERT_SIGNED_CONTENT_INFO).
//
//  hCryptProv specifies the crypto provider to use to compute the hash.
//  It doesn't need to use a private key.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptHashToBeSigned(
    _In_opt_ HCRYPTPROV_LEGACY hCryptProv,
    _In_ DWORD dwCertEncodingType,
    _In_reads_bytes_(cbEncoded) const BYTE *pbEncoded,
    _In_ DWORD cbEncoded,
    _Out_writes_bytes_to_opt_(*pcbComputedHash, *pcbComputedHash) BYTE *pbComputedHash,
    _Inout_ DWORD *pcbComputedHash
    );

//+-------------------------------------------------------------------------
//  Hash the encoded content.
//
//  hCryptProv specifies the crypto provider to use to compute the hash.
//  It doesn't need to use a private key.
//
//  Algid specifies the CAPI hash algorithm to use. If Algid is 0, then, the
//  default hash algorithm (currently SHA1) is used.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptHashCertificate(
    _In_opt_ HCRYPTPROV_LEGACY hCryptProv,
    _In_ ALG_ID Algid,
    _In_ DWORD dwFlags,
    _In_reads_bytes_(cbEncoded) const BYTE *pbEncoded,
    _In_ DWORD cbEncoded,
    _Out_writes_bytes_to_opt_(*pcbComputedHash, *pcbComputedHash) BYTE *pbComputedHash,
    _Inout_ DWORD *pcbComputedHash
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (NTDDI_VERSION >= NTDDI_VISTA)

//+-------------------------------------------------------------------------
//  Hash the encoded content using the CNG hash algorithm provider.
//--------------------------------------------------------------------------
WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CryptHashCertificate2(
    _In_ LPCWSTR pwszCNGHashAlgid,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved,
    _In_reads_bytes_opt_(cbEncoded) const BYTE *pbEncoded,
    _In_ DWORD cbEncoded,
    _Out_writes_bytes_to_opt_(*pcbComputedHash, *pcbComputedHash) BYTE *pbComputedHash,
    _Inout_ DWORD *pcbComputedHash
    );

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Sign the "to be signed" information in the encoded signed content.
//
//  hCryptProvOrNCryptKey specifies the crypto provider to use to do the
//  signature.  It uses the specified private key.
//
//  If the SignatureAlgorithm is a hash algorithm, then, the signature
//  contains the hash octets. A private key isn't used to encrypt the hash.
//  dwKeySpec isn't used and hCryptProvOrNCryptKey can be NULL where an
//  appropriate default provider will be used for hashing.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptSignCertificate(
#ifdef CMSG_SIGNER_ENCODE_INFO_HAS_IUM_FIELDS
    _In_ BCRYPT_KEY_HANDLE hBCryptKey,
#else
    _In_opt_ HCRYPTPROV_OR_NCRYPT_KEY_HANDLE hCryptProvOrNCryptKey,
#endif
    _In_opt_ DWORD dwKeySpec,       // not applicable for NCRYPT_KEY_HANDLE
    _In_ DWORD dwCertEncodingType,
    _In_reads_bytes_(cbEncodedToBeSigned) const BYTE *pbEncodedToBeSigned,
    _In_ DWORD cbEncodedToBeSigned,
    _In_ PCRYPT_ALGORITHM_IDENTIFIER pSignatureAlgorithm,
    _In_opt_ const void *pvHashAuxInfo,
    _Out_writes_bytes_to_opt_(*pcbSignature, *pcbSignature) BYTE *pbSignature,
    _Inout_ DWORD *pcbSignature
    );

//+-------------------------------------------------------------------------
//  Encode the "to be signed" information. Sign the encoded "to be signed".
//  Encode the "to be signed" and the signature.
//
//  hCryptProv specifies the crypto provider to use to do the signature.
//  It uses the specified private key.
//
//  If the SignatureAlgorithm is a hash algorithm, then, the signature
//  contains the hash octets. A private key isn't used to encrypt the hash.
//  dwKeySpec isn't used and hCryptProv can be NULL where an appropriate
//  default provider will be used for hashing.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptSignAndEncodeCertificate(
#ifdef CMSG_SIGNER_ENCODE_INFO_HAS_IUM_FIELDS
    _In_ BCRYPT_KEY_HANDLE hBCryptKey,
#else
    _In_opt_ HCRYPTPROV_OR_NCRYPT_KEY_HANDLE hCryptProvOrNCryptKey,
#endif
    _In_opt_ DWORD dwKeySpec,       // not applicable for NCRYPT_KEY_HANDLE
    _In_ DWORD dwCertEncodingType,
    _In_ LPCSTR lpszStructType,       // "to be signed"
    _In_ const void *pvStructInfo,
    _In_ PCRYPT_ALGORITHM_IDENTIFIER pSignatureAlgorithm,
    _In_opt_ const void *pvHashAuxInfo,
    _Out_writes_bytes_to_opt_(*pcbEncoded, *pcbEncoded) BYTE *pbEncoded,
    _Inout_ DWORD *pcbEncoded
    );


//+-------------------------------------------------------------------------
//  Certificate and CryptMsg encoded signature OID installable functions
//--------------------------------------------------------------------------

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

// The dwCertEncodingType and pSignatureAlgorithm->pszObjId are used
// to call the signature OID installable functions.
//
// If the OID installable function doesn't support the signature,
// it should return FALSE with LastError set to ERROR_NOT_SUPPORTED.

// Called if the signature has encoded parameters. Returns the CNG
// hash algorithm identifier string. Optionally returns the decoded
// signature parameters passed to either the SignAndEncodeHash or
// VerifyEncodedSignature OID installable function.
//
// Returned allocated parameters are freed via LocalFree().
#define CRYPT_OID_EXTRACT_ENCODED_SIGNATURE_PARAMETERS_FUNC \
    "CryptDllExtractEncodedSignatureParameters"
typedef BOOL (WINAPI *PFN_CRYPT_EXTRACT_ENCODED_SIGNATURE_PARAMETERS_FUNC)(
    _In_ DWORD dwCertEncodingType,
    _In_ PCRYPT_ALGORITHM_IDENTIFIER pSignatureAlgorithm,
    _Outptr_result_maybenull_ void **ppvDecodedSignPara, // LocalFree()
    _Outptr_ LPWSTR *ppwszCNGHashAlgid      // LocalFree()
    );

// Called to sign the computed hash and encode it.
#define CRYPT_OID_SIGN_AND_ENCODE_HASH_FUNC \
    "CryptDllSignAndEncodeHash"
typedef BOOL (WINAPI *PFN_CRYPT_SIGN_AND_ENCODE_HASH_FUNC)(
    _In_ NCRYPT_KEY_HANDLE hKey,
    _In_ DWORD dwCertEncodingType,
    _In_ PCRYPT_ALGORITHM_IDENTIFIER pSignatureAlgorithm,
    _In_opt_ void *pvDecodedSignPara,
    _In_ LPCWSTR pwszCNGPubKeyAlgid,    // obtained from signature OID
    _In_ LPCWSTR pwszCNGHashAlgid,
    _In_reads_bytes_(cbComputedHash) BYTE *pbComputedHash,
    _In_ DWORD cbComputedHash,
    _Out_writes_bytes_to_opt_(*pcbSignature, *pcbSignature) BYTE *pbSignature,
    _Inout_ DWORD *pcbSignature
    );

// Called to decode and decrypt the encoded signature and compare it with the
// computed hash.
#define CRYPT_OID_VERIFY_ENCODED_SIGNATURE_FUNC \
    "CryptDllVerifyEncodedSignature"
typedef BOOL (WINAPI *PFN_CRYPT_VERIFY_ENCODED_SIGNATURE_FUNC)(
    _In_ DWORD dwCertEncodingType,
    _In_ PCERT_PUBLIC_KEY_INFO pPubKeyInfo,
    _In_ PCRYPT_ALGORITHM_IDENTIFIER pSignatureAlgorithm,
    _In_opt_ void *pvDecodedSignPara,
    _In_ LPCWSTR pwszCNGPubKeyAlgid,    // obtained from signature OID
    _In_ LPCWSTR pwszCNGHashAlgid,
    _In_reads_bytes_(cbComputedHash) BYTE *pbComputedHash,
    _In_ DWORD cbComputedHash,
    _In_reads_bytes_(cbSignature) BYTE *pbSignature,
    _In_ DWORD cbSignature
    );


//+-------------------------------------------------------------------------
//  Verify the time validity of a certificate.
//
//  Returns -1 if before NotBefore, +1 if after NotAfter and otherwise 0 for
//  a valid certificate
//
//  If pTimeToVerify is NULL, uses the current time.
//--------------------------------------------------------------------------
WINCRYPT32API
LONG
WINAPI
CertVerifyTimeValidity(
    _In_opt_ LPFILETIME pTimeToVerify,
    _In_ PCERT_INFO pCertInfo
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Verify the time validity of a CRL.
//
//  Returns -1 if before ThisUpdate, +1 if after NextUpdate and otherwise 0 for
//  a valid CRL
//
//  If pTimeToVerify is NULL, uses the current time.
//--------------------------------------------------------------------------
WINCRYPT32API
LONG
WINAPI
CertVerifyCRLTimeValidity(
    _In_opt_ LPFILETIME pTimeToVerify,
    _In_ PCRL_INFO pCrlInfo
    );

//+-------------------------------------------------------------------------
//  Verify that the subject's time validity nests within the issuer's time
//  validity.
//
//  Returns TRUE if it nests. Otherwise, returns FALSE.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertVerifyValidityNesting(
    _In_ PCERT_INFO pSubjectInfo,
    _In_ PCERT_INFO pIssuerInfo
    );

//+-------------------------------------------------------------------------
//  Verify that the subject certificate isn't on its issuer CRL.
//
//  Returns true if the certificate isn't on the CRL.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertVerifyCRLRevocation(
    _In_ DWORD dwCertEncodingType,
    _In_ PCERT_INFO pCertId,          // Only the Issuer and SerialNumber
                                      // fields are used
    _In_ DWORD cCrlInfo,
    _In_reads_(cCrlInfo) PCRL_INFO rgpCrlInfo[]
    );

//+-------------------------------------------------------------------------
//  Convert the CAPI AlgId to the ASN.1 Object Identifier string
//
//  Returns NULL if there isn't an ObjId corresponding to the AlgId.
//--------------------------------------------------------------------------
WINCRYPT32API
LPCSTR
WINAPI
CertAlgIdToOID(
    _In_ DWORD dwAlgId
    );

//+-------------------------------------------------------------------------
//  Convert the ASN.1 Object Identifier string to the CAPI AlgId.
//
//  Returns 0 if there isn't an AlgId corresponding to the ObjId.
//--------------------------------------------------------------------------
WINCRYPT32API
DWORD
WINAPI
CertOIDToAlgId(
    _In_ LPCSTR pszObjId
    );

//+-------------------------------------------------------------------------
//  Find an extension identified by its Object Identifier.
//
//  If found, returns pointer to the extension. Otherwise, returns NULL.
//--------------------------------------------------------------------------
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINCRYPT32API
PCERT_EXTENSION
WINAPI
CertFindExtension(
    _In_ LPCSTR pszObjId,
    _In_ DWORD cExtensions,
    _In_reads_(cExtensions) CERT_EXTENSION rgExtensions[]
    );

//+-------------------------------------------------------------------------
//  Find the first attribute identified by its Object Identifier.
//
//  If found, returns pointer to the attribute. Otherwise, returns NULL.
//--------------------------------------------------------------------------
WINCRYPT32API
PCRYPT_ATTRIBUTE
WINAPI
CertFindAttribute(
    _In_ LPCSTR pszObjId,
    _In_ DWORD cAttr,
    _In_reads_(cAttr) CRYPT_ATTRIBUTE rgAttr[]
    );

//+-------------------------------------------------------------------------
//  Find the first CERT_RDN attribute identified by its Object Identifier in
//  the name's list of Relative Distinguished Names.
//
//  If found, returns pointer to the attribute. Otherwise, returns NULL.
//--------------------------------------------------------------------------
WINCRYPT32API
PCERT_RDN_ATTR
WINAPI
CertFindRDNAttr(
    _In_ LPCSTR pszObjId,
    _In_ PCERT_NAME_INFO pName
    );

//+-------------------------------------------------------------------------
//  Get the intended key usage bytes from the certificate.
//
//  If the certificate doesn't have any intended key usage bytes, returns FALSE
//  and *pbKeyUsage is zeroed. Otherwise, returns TRUE and up through
//  cbKeyUsage bytes are copied into *pbKeyUsage. Any remaining uncopied
//  bytes are zeroed.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertGetIntendedKeyUsage(
    _In_ DWORD dwCertEncodingType,
    _In_ PCERT_INFO pCertInfo,
    _Out_writes_bytes_all_(cbKeyUsage) BYTE *pbKeyUsage,
    _In_ DWORD cbKeyUsage
    );

typedef void *HCRYPTDEFAULTCONTEXT;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Install a previously CryptAcquiredContext'ed HCRYPTPROV to be used as
//  a default context.
//
//  dwDefaultType and pvDefaultPara specify where the default context is used.
//  For example, install the HCRYPTPROV to be used to verify certificate's
//  having szOID_OIWSEC_md5RSA signatures.
//
//  By default, the installed HCRYPTPROV is only applicable to the current
//  thread. Set CRYPT_DEFAULT_CONTEXT_PROCESS_FLAG to allow the HCRYPTPROV
//  to be used by all threads in the current process.
//
//  For a successful install, TRUE is returned and *phDefaultContext is
//  updated with the HANDLE to be passed to CryptUninstallDefaultContext.
//
//  The installed HCRYPTPROVs are stack ordered (the last installed
//  HCRYPTPROV is checked first). All thread installed HCRYPTPROVs are
//  checked before any process HCRYPTPROVs.
//
//  The installed HCRYPTPROV remains available for default usage until
//  CryptUninstallDefaultContext is called or the thread or process exits.
//
//  If CRYPT_DEFAULT_CONTEXT_AUTO_RELEASE_FLAG is set, then, the HCRYPTPROV
//  is CryptReleaseContext'ed at thread or process exit. However,
//  not CryptReleaseContext'ed if CryptUninstallDefaultContext is
//  called.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptInstallDefaultContext(
    _In_ HCRYPTPROV hCryptProv,
    _In_ DWORD dwDefaultType,
    _In_opt_ const void *pvDefaultPara,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved,
    _Out_ HCRYPTDEFAULTCONTEXT *phDefaultContext
    );

// dwFlags
#define CRYPT_DEFAULT_CONTEXT_AUTO_RELEASE_FLAG             0x00000001
#define CRYPT_DEFAULT_CONTEXT_PROCESS_FLAG                  0x00000002

// List of dwDefaultType's
#define CRYPT_DEFAULT_CONTEXT_CERT_SIGN_OID         1
#define CRYPT_DEFAULT_CONTEXT_MULTI_CERT_SIGN_OID   2

//+-------------------------------------------------------------------------
//  CRYPT_DEFAULT_CONTEXT_CERT_SIGN_OID
//
//  Install a default HCRYPTPROV used to verify a certificate
//  signature. pvDefaultPara points to the szOID of the certificate
//  signature algorithm, for example, szOID_OIWSEC_md5RSA. If
//  pvDefaultPara is NULL, then, the HCRYPTPROV is used to verify all
//  certificate signatures. Note, pvDefaultPara can't be NULL when
//  CRYPT_DEFAULT_CONTEXT_PROCESS_FLAG is set.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CRYPT_DEFAULT_CONTEXT_MULTI_CERT_SIGN_OID
//
//  Same as CRYPT_DEFAULT_CONTEXT_CERT_SIGN_OID. However, the default
//  HCRYPTPROV is to be used for multiple signature szOIDs. pvDefaultPara
//  points to a CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA structure containing
//  an array of szOID pointers.
//--------------------------------------------------------------------------

typedef struct _CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA {
    DWORD               cOID;
    LPSTR               *rgpszOID;
} CRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA, *PCRYPT_DEFAULT_CONTEXT_MULTI_OID_PARA;

//+-------------------------------------------------------------------------
//  Uninstall a default context previously installed by
//  CryptInstallDefaultContext.
//
//  For a default context installed with CRYPT_DEFAULT_CONTEXT_PROCESS_FLAG
//  set, if any other threads are currently using this context,
//  this function will block until they finish.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptUninstallDefaultContext(
    _In_opt_ HCRYPTDEFAULTCONTEXT hDefaultContext,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved
    );

//+-------------------------------------------------------------------------
//  Export the public key info associated with the provider's corresponding
//  private key.
//
//  Calls CryptExportPublicKeyInfoEx with pszPublicKeyObjId = NULL,
//  dwFlags = 0 and pvAuxInfo = NULL.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptExportPublicKeyInfo(
    _In_ HCRYPTPROV_OR_NCRYPT_KEY_HANDLE hCryptProvOrNCryptKey,
    _In_opt_ DWORD dwKeySpec,       // not applicable for NCRYPT_KEY_HANDLE
    _In_ DWORD dwCertEncodingType,
    _Out_writes_bytes_to_opt_(*pcbInfo, *pcbInfo) PCERT_PUBLIC_KEY_INFO pInfo,
    _Inout_ DWORD *pcbInfo
    );

//+-------------------------------------------------------------------------
//  Export the public key info associated with the provider's corresponding
//  private key.
//
//  Uses the dwCertEncodingType and pszPublicKeyObjId to call the
//  installable CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_FUNC. The called function
//  has the same signature as CryptExportPublicKeyInfoEx.
//
//  If unable to find an installable OID function for the pszPublicKeyObjId,
//  attempts to export as a RSA Public Key (szOID_RSA_RSA).
//
//  The dwFlags and pvAuxInfo aren't used for szOID_RSA_RSA.
//
//  dwFlags can be set with the following 2 flags passed directly to
//  CryptFindOIDInfo:
//      CRYPT_OID_INFO_PUBKEY_SIGN_KEY_FLAG
//      CRYPT_OID_INFO_PUBKEY_ENCRYPT_KEY_FLAG
//
//  dwFlags can be set with the following 2 flags passed directly to
//  CryptFindOIDInfo to restrict PQ key to either "Pure" or "PreHash":
//      CRYPT_OID_INFO_PUBKEY_PURE_KEY_FLAG
//      CRYPT_OID_INFO_PUBKEY_PREHASH_KEY_FLAG
//
//  dwFlags can be set with the following 2 flags to encode either Curve OID
//  or ECC Parameters in Algorithm's Parameters section:
//      CRYPT_OID_USE_CURVE_NAME_FOR_ENCODE_FLAG
//      CRYPT_OID_USE_CURVE_PARAMETERS_FOR_ENCODE_FLAG
//--------------------------------------------------------------------------

WINCRYPT32API
BOOL
WINAPI
CryptExportPublicKeyInfoEx(
    _In_ HCRYPTPROV_OR_NCRYPT_KEY_HANDLE hCryptProvOrNCryptKey,
    _In_opt_ DWORD dwKeySpec,       // not applicable for NCRYPT_KEY_HANDLE
    _In_ DWORD dwCertEncodingType,
    _In_opt_ LPSTR pszPublicKeyObjId,
    _In_ DWORD dwFlags,
    _In_opt_ void *pvAuxInfo,
    _Out_writes_bytes_to_opt_(*pcbInfo, *pcbInfo) PCERT_PUBLIC_KEY_INFO pInfo,
    _Inout_ DWORD *pcbInfo
    );

// Legacy define used for exporting CAPI1 HCRYPTPROV public keys.
#define CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_FUNC   "CryptDllExportPublicKeyInfoEx"

//+-------------------------------------------------------------------------
//  Export CNG PublicKeyInfo OID installable function. Note, not called
//  for a HCRYPTPROV choice.
//--------------------------------------------------------------------------
#define CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_EX2_FUNC \
    "CryptDllExportPublicKeyInfoEx2"
typedef BOOL (WINAPI *PFN_CRYPT_EXPORT_PUBLIC_KEY_INFO_EX2_FUNC) (
    _In_ NCRYPT_KEY_HANDLE hNCryptKey,
    _In_ DWORD dwCertEncodingType,
    _In_ LPSTR pszPublicKeyObjId,
    _In_ DWORD dwFlags,
    _In_opt_ void *pvAuxInfo,
    _Out_writes_bytes_to_opt_(*pcbInfo, *pcbInfo) PCERT_PUBLIC_KEY_INFO pInfo,
    _Inout_ DWORD *pcbInfo
    );

#if (NTDDI_VERSION >= NTDDI_WIN7)

//+-------------------------------------------------------------------------
//  Export the public key info associated with the provider's corresponding
//  private key.
//
//  Uses the dwCertEncodingType and pszPublicKeyObjId to call the
//  installable CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_FROM_BCRYPT_HANDLE_FUNC. The
//  called function has the same signature as
//  CryptExportPublicKeyInfoFromBCryptKeyHandle.
//
//  If unable to find an installable OID function for the pszPublicKeyObjId,
//  attempts to export as a RSA Public Key (szOID_RSA_RSA).
//
//  The dwFlags and pvAuxInfo aren't used for szOID_RSA_RSA.
//
//  In addition dwFlags can be set with the following 2 flags passed directly
//  to CryptFindOIDInfo:
//      CRYPT_OID_INFO_PUBKEY_SIGN_KEY_FLAG
//      CRYPT_OID_INFO_PUBKEY_ENCRYPT_KEY_FLAG
//--------------------------------------------------------------------------

WINCRYPT32API
BOOL
WINAPI
CryptExportPublicKeyInfoFromBCryptKeyHandle(
    _In_ BCRYPT_KEY_HANDLE hBCryptKey,
    _In_ DWORD dwCertEncodingType,
    _In_opt_ LPSTR pszPublicKeyObjId,
    _In_ DWORD dwFlags,
    _In_opt_ void *pvAuxInfo,
    _Out_writes_bytes_to_opt_(*pcbInfo, *pcbInfo) PCERT_PUBLIC_KEY_INFO pInfo,
    _Inout_ DWORD *pcbInfo
    );

//+-------------------------------------------------------------------------
//  Export CNG PublicKeyInfo OID installable function. Note, not called
//  for a HCRYPTPROV or NCRYPT_KEY_HANDLE choice.
//--------------------------------------------------------------------------

#define CRYPT_OID_EXPORT_PUBLIC_KEY_INFO_FROM_BCRYPT_HANDLE_FUNC \
    "CryptDllExportPublicKeyInfoFromBCryptKeyHandle"
typedef BOOL (WINAPI *PFN_CRYPT_EXPORT_PUBLIC_KEY_INFO_FROM_BCRYPT_HANDLE_FUNC) (
    _In_ BCRYPT_KEY_HANDLE hBCryptKey,
    _In_ DWORD dwCertEncodingType,
    _In_ LPSTR pszPublicKeyObjId,
    _In_ DWORD dwFlags,
    _In_opt_ void *pvAuxInfo,
    _Out_writes_bytes_to_opt_(*pcbInfo, *pcbInfo) PCERT_PUBLIC_KEY_INFO pInfo,
    _Inout_ DWORD *pcbInfo
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN7)

//+-------------------------------------------------------------------------
//  Convert and import the public key info into the provider and return a
//  handle to the public key.
//
//  Calls CryptImportPublicKeyInfoEx with aiKeyAlg = 0, dwFlags = 0 and
//  pvAuxInfo = NULL.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptImportPublicKeyInfo(
    _In_ HCRYPTPROV hCryptProv,
    _In_ DWORD dwCertEncodingType,
    _In_ PCERT_PUBLIC_KEY_INFO pInfo,
    _Out_ HCRYPTKEY *phKey
    );

//+-------------------------------------------------------------------------
//  Convert and import the public key info into the provider and return a
//  handle to the public key.
//
//  Uses the dwCertEncodingType and pInfo->Algorithm.pszObjId to call the
//  installable CRYPT_OID_IMPORT_PUBLIC_KEY_INFO_FUNC. The called function
//  has the same signature as CryptImportPublicKeyInfoEx.
//
//  If unable to find an installable OID function for the pszObjId,
//  attempts to import as a RSA Public Key (szOID_RSA_RSA).
//
//  For szOID_RSA_RSA: aiKeyAlg may be set to CALG_RSA_SIGN or CALG_RSA_KEYX.
//  Defaults to CALG_RSA_KEYX. The dwFlags and pvAuxInfo aren't used.
//--------------------------------------------------------------------------
#define CRYPT_OID_IMPORT_PUBLIC_KEY_INFO_FUNC   "CryptDllImportPublicKeyInfoEx"

WINCRYPT32API
BOOL
WINAPI
CryptImportPublicKeyInfoEx(
    _In_ HCRYPTPROV hCryptProv,
    _In_ DWORD dwCertEncodingType,
    _In_ PCERT_PUBLIC_KEY_INFO pInfo,
    _In_ ALG_ID aiKeyAlg,
    _In_ DWORD dwFlags,
    _In_opt_ void *pvAuxInfo,
    _Out_ HCRYPTKEY *phKey
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (NTDDI_VERSION >= NTDDI_VISTA)

//+-------------------------------------------------------------------------
//  Convert and import the public key info into the CNG asymmetric or
//  signature algorithm provider and return a BCRYPT_KEY_HANDLE to it.
//
//  Uses the dwCertEncodingType and pInfo->Algorithm.pszObjId to call the
//  installable CRYPT_OID_IMPORT_PUBLIC_KEY_INFO_EX2_FUNC. The called function
//  has the same signature as CryptImportPublicKeyInfoEx2.
//
//  dwFlags can be set with the following 2 flags passed directly to
//  CryptFindOIDInfo:
//      CRYPT_OID_INFO_PUBKEY_SIGN_KEY_FLAG
//      CRYPT_OID_INFO_PUBKEY_ENCRYPT_KEY_FLAG
//  dwFlags can also have BCRYPT_NO_KEY_VALIDATION OR'd in. This flag is
//  passed to BCryptImportKeyPair.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptImportPublicKeyInfoEx2(
    _In_ DWORD dwCertEncodingType,
    _In_ PCERT_PUBLIC_KEY_INFO pInfo,
    _In_ DWORD dwFlags,
    _In_opt_ void *pvAuxInfo,
    _Out_ BCRYPT_KEY_HANDLE *phKey
    );

//+-------------------------------------------------------------------------
//  Import CNG PublicKeyInfo OID installable function
//--------------------------------------------------------------------------
#define CRYPT_OID_IMPORT_PUBLIC_KEY_INFO_EX2_FUNC \
    "CryptDllImportPublicKeyInfoEx2"
typedef BOOL (WINAPI *PFN_IMPORT_PUBLIC_KEY_INFO_EX2_FUNC) (
    _In_ DWORD dwCertEncodingType,
    _In_ PCERT_PUBLIC_KEY_INFO pInfo,
    _In_ DWORD dwFlags,
    _In_opt_ void *pvAuxInfo,
    _Out_ BCRYPT_KEY_HANDLE *phKey
    );

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Acquire a HCRYPTPROV and dwKeySpec or NCRYPT_KEY_HANDLE for the
//  specified certificate context. Uses the certificate's
//  CERT_KEY_PROV_INFO_PROP_ID property.
//  The returned HCRYPTPROV or NCRYPT_KEY_HANDLE handle may optionally be
//  cached using the certificate's CERT_KEY_CONTEXT_PROP_ID property.
//
//  If CRYPT_ACQUIRE_CACHE_FLAG is set, then, if an already acquired and
//  cached HCRYPTPROV or NCRYPT_KEY_HANDLE exists for the certificate, its
//  returned. Otherwise, a HCRYPTPROV or NCRYPT_KEY_HANDLE is acquired and
//  then cached via the certificate's CERT_KEY_CONTEXT_PROP_ID.
//
//  The CRYPT_ACQUIRE_USE_PROV_INFO_FLAG can be set to use the dwFlags field of
//  the certificate's CERT_KEY_PROV_INFO_PROP_ID property's CRYPT_KEY_PROV_INFO
//  data structure to determine if the returned HCRYPTPROV or
//  NCRYPT_KEY_HANDLE should be cached.
//  Caching is enabled if the CERT_SET_KEY_CONTEXT_PROP_ID flag was
//  set.
//
//  If CRYPT_ACQUIRE_COMPARE_KEY_FLAG is set, then,
//  the public key in the certificate is compared with the public
//  key returned by the cryptographic provider. If the keys don't match, the
//  acquire fails and LastError is set to NTE_BAD_PUBLIC_KEY. Note, if
//  a cached HCRYPTPROV or NCRYPT_KEY_HANDLE is returned, the comparison isn't
//  done. We assume the comparison was done on the initial acquire.
//
//  The CRYPT_ACQUIRE_NO_HEALING flags prohibits this function from
//  attempting to recreate the CERT_KEY_PROV_INFO_PROP_ID in the certificate
//  context if it fails to retrieve this property.
//
//  The CRYPT_ACQUIRE_SILENT_FLAG can be set to suppress any UI by the CSP.
//  See CryptAcquireContext's CRYPT_SILENT flag for more details.
//
//  The CRYPT_ACQUIRE_WINDOW_HANDLE_FLAG can be set when a pointer to a window handle (HWND*)
//  is passed in as the pvParameters. The window handle will be used
//  by calling CryptSetProvParam with a NULL HCRYPTPROV and dwParam
//  is PP_CLIENT_HWND before the call to CryptAcquireContext. 
//  This will set the window handle for all CAPI calls in this process.
//  The caller should make sure the window handle is valid or clear it out by
//  calling CryptSetProvParam with PP_CLIENT_HWND with a NULL hWnd.
//  Or for cng, the hwnd will be used by calling NCryptSetProperty on the storage provider
//  handle provider with property NCRYPT_WINDOW_HANDLE_PROPERTY and
//  by calling NCryptSetPRoperty on the key handle with property NCRYPT_WINDOW_HANDLE_PROPERTY.
//  If both calls to NCryptSetProperty fail then the function will return the failure of 
//  setting the NCRYPT_WINDOW_HANDLE_PROPERTY on the key handle.
//  Do not use this flag with CRYPT_ACQUIRE_SILENT_FLAG.
//
//  The following flags can be set to optionally open and return a CNG
//  NCRYPT_KEY_HANDLE instead of a HCRYPTPROV. *pdwKeySpec is set to
//  CERT_NCRYPT_KEY_SPEC when a NCRYPT_KEY_HANDLE is returned.
//      CRYPT_ACQUIRE_ALLOW_NCRYPT_KEY_FLAG - if the CryptAcquireContext
//      fails, then, an NCryptOpenKey is attempted.
//
//      CRYPT_ACQUIRE_PREFER_NCRYPT_KEY_FLAG - the NCryptOpenKey is
//      first attempted and its handle returned for success.
//
//      CRYPT_ACQUIRE_ONLY_NCRYPT_KEY_FLAG - only the NCryptOpenKey is
//      attempted.
//
//  *pfCallerFreeProvOrNCryptKey is returned set to FALSE for:
//    - Acquire or public key comparison fails.
//    - CRYPT_ACQUIRE_CACHE_FLAG is set.
//    - CRYPT_ACQUIRE_USE_PROV_INFO_FLAG is set AND
//      CERT_SET_KEY_CONTEXT_PROP_ID flag is set in the dwFlags field of the
//      certificate's CERT_KEY_PROV_INFO_PROP_ID property's
//      CRYPT_KEY_PROV_INFO data structure.
//  When *pfCallerFreeProvOrNCryptKey is FALSE, the caller must not release. The
//  returned HCRYPTPROV or NCRYPT_KEY_HANDLE will be released on the last
//  free of the certificate context.
//
//  Otherwise, *pfCallerFreeProvOrNCryptKey is TRUE and a returned
//  HCRYPTPROV must be released by the caller by calling CryptReleaseContext.
//  A returned NCRYPT_KEY_HANDLE is freed by calling NCryptFreeObject.
//  *pdwKeySpec MUST be checked when CRYPT_ACQUIRE_ALLOW_NCRYPT_KEY_FLAG
//  or CRYPT_ACQUIRE_PREFER_NCRYPT_KEY_FLAG is set.
//
//--------------------------------------------------------------------------

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINCRYPT32API
BOOL
WINAPI
CryptAcquireCertificatePrivateKey(
    _In_ PCCERT_CONTEXT pCert,
    _In_ DWORD dwFlags,
    _In_opt_ void *pvParameters,
    _Out_ HCRYPTPROV_OR_NCRYPT_KEY_HANDLE *phCryptProvOrNCryptKey,
    _Out_opt_ DWORD *pdwKeySpec,
    _Out_opt_ BOOL *pfCallerFreeProvOrNCryptKey
    );

#define CRYPT_ACQUIRE_CACHE_FLAG                0x00000001
#define CRYPT_ACQUIRE_USE_PROV_INFO_FLAG        0x00000002
#define CRYPT_ACQUIRE_COMPARE_KEY_FLAG          0x00000004
#define CRYPT_ACQUIRE_NO_HEALING                0x00000008

#define CRYPT_ACQUIRE_SILENT_FLAG               0x00000040
#define CRYPT_ACQUIRE_WINDOW_HANDLE_FLAG        0x00000080

#define CRYPT_ACQUIRE_NCRYPT_KEY_FLAGS_MASK     0x00070000
#define CRYPT_ACQUIRE_ALLOW_NCRYPT_KEY_FLAG     0x00010000
#define CRYPT_ACQUIRE_PREFER_NCRYPT_KEY_FLAG    0x00020000
#define CRYPT_ACQUIRE_ONLY_NCRYPT_KEY_FLAG      0x00040000


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Enumerates the cryptographic providers and their containers to find the
//  private key corresponding to the certificate's public key. For a match,
//  the certificate's CERT_KEY_PROV_INFO_PROP_ID property is updated.
//
//  If the CERT_KEY_PROV_INFO_PROP_ID is already set, then, its checked to
//  see if it matches the provider's public key. For a match, the above
//  enumeration is skipped.
//
//  By default both the user and machine key containers are searched.
//  The CRYPT_FIND_USER_KEYSET_FLAG or CRYPT_FIND_MACHINE_KEYSET_FLAG
//  can be set in dwFlags to restrict the search to either of the containers.
//
//  The CRYPT_FIND_SILENT_KEYSET_FLAG can be set to suppress any UI by the CSP.
//  See CryptAcquireContext's CRYPT_SILENT flag for more details.
//
//  If a container isn't found, returns FALSE with LastError set to
//  NTE_NO_KEY.
//
//  The above CRYPT_ACQUIRE_NCRYPT_KEY_FLAGS can also be set. The default
//  is CRYPT_ACQUIRE_ALLOW_NCRYPT_KEY_FLAG.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptFindCertificateKeyProvInfo(
    _In_ PCCERT_CONTEXT pCert,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved
    );

#define CRYPT_FIND_USER_KEYSET_FLAG        0x00000001
#define CRYPT_FIND_MACHINE_KEYSET_FLAG     0x00000002
#define CRYPT_FIND_SILENT_KEYSET_FLAG      0x00000040


//+-------------------------------------------------------------------------
//  This is the prototype for the installable function which is called to
//  actually import a key into a CSP.  an installable of this type is called
//  from CryptImportPKCS8.  the algorithm OID of the private key is used
//  to look up the proper installable function to call.
//
//  hCryptProv - the provider to import the key to
//  pPrivateKeyInfo - describes the key to be imported
//  dwFlags - The available flags are:
//              CRYPT_EXPORTABLE
//              this flag is used when importing private keys, for a full
//              explanation please see the documentation for CryptImportKey.
//  pvAuxInfo - reserved for future, must be NULL
//--------------------------------------------------------------------------
typedef BOOL (WINAPI *PFN_IMPORT_PRIV_KEY_FUNC) (
    _In_ HCRYPTPROV hCryptProv,                     // in
    _In_ CRYPT_PRIVATE_KEY_INFO* pPrivateKeyInfo,   // in
    _In_ DWORD dwFlags,                             // in
    _In_opt_ void* pvAuxInfo                        // in, optional
    );

#define CRYPT_OID_IMPORT_PRIVATE_KEY_INFO_FUNC   "CryptDllImportPrivateKeyInfoEx"

//+-------------------------------------------------------------------------
// Convert (from PKCS8 format) and import the private key into a provider
// and return a handle to the provider as well as the KeySpec used to import to.
//
// This function will call the PRESOLVE_HCRYPTPROV_FUNC in the
// privateKeyAndParams to obtain a handle of provider to import the key to.
// if the PRESOLVE_HCRYPTPROV_FUNC is NULL then the default provider will be used.
//
// privateKeyAndParams - private key blob and corresponding parameters
// dwFlags - The available flags are:
//              CRYPT_EXPORTABLE
//              this flag is used when importing private keys, for a full
//              explanation please see the documentation for CryptImportKey.
// phCryptProv - filled in with the handle of the provider the key was
//               imported to, the caller is responsible for freeing it
// pvAuxInfo - This parameter is reserved for future use and should be set
//             to NULL in the interim.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptImportPKCS8(
    _In_ CRYPT_PKCS8_IMPORT_PARAMS sPrivateKeyAndParams,    // in
    _In_ DWORD dwFlags,                                     // in
    _Out_opt_ HCRYPTPROV *phCryptProv,                      // out, optional
    _In_opt_ void* pvAuxInfo                                // in, optional
    );

//+-------------------------------------------------------------------------
// this is the prototype for installable functions for exporting the private key
//--------------------------------------------------------------------------
typedef BOOL (WINAPI *PFN_EXPORT_PRIV_KEY_FUNC) (
    _In_ HCRYPTPROV hCryptProv,         // in
    _In_ DWORD dwKeySpec,               // in
    _In_ LPSTR pszPrivateKeyObjId,      // in
    _In_ DWORD dwFlags,                 // in
    _In_opt_ void* pvAuxInfo,           // in
    _Out_writes_bytes_opt_ (*pcbPrivateKeyInfo) CRYPT_PRIVATE_KEY_INFO* pPrivateKeyInfo,  // out
    _Inout_ DWORD* pcbPrivateKeyInfo    // in, out
    );

#define CRYPT_OID_EXPORT_PRIVATE_KEY_INFO_FUNC   "CryptDllExportPrivateKeyInfoEx"

#define CRYPT_DELETE_KEYSET CRYPT_DELETEKEYSET
//+-------------------------------------------------------------------------
//  CryptExportPKCS8 -- superseded by CryptExportPKCS8Ex
//
//  Export the private key in PKCS8 format
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptExportPKCS8(
    _In_ HCRYPTPROV hCryptProv,                                     // in
    _In_ DWORD dwKeySpec,                                           // in
    _In_ LPSTR pszPrivateKeyObjId,                                  // in
    _In_ DWORD dwFlags,                                             // in
    _In_opt_ void* pvAuxInfo,                                       // in
    _Out_writes_bytes_opt_ (*pcbPrivateKeyBlob) BYTE* pbPrivateKeyBlob,   // out
    _Inout_ DWORD *pcbPrivateKeyBlob                                // in, out
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
// CryptExportPKCS8Ex
//
//  Export the private key in PKCS8 format
//
//
//  Uses the pszPrivateKeyObjId to call the
//  installable CRYPT_OID_EXPORT_PRIVATE_KEY_INFO_FUNC. The called function
//  has the signature defined by PFN_EXPORT_PRIV_KEY_FUNC.
//
//  If unable to find an installable OID function for the pszPrivateKeyObjId,
//  attempts to export as a RSA Private Key (szOID_RSA_RSA).
//
// psExportParams - specifies information about the key to export
// dwFlags - The flag values. None currently supported
// pvAuxInfo - This parameter is reserved for future use and should be set to
//                         NULL in the interim.
// pbPrivateKeyBlob - A pointer to the private key blob.  It will be encoded
//                                        as a PKCS8 PrivateKeyInfo.
// pcbPrivateKeyBlob - A pointer to a DWORD that contains the size, in bytes,
//                                         of the private key blob being exported.
//+-------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptExportPKCS8Ex(
    _In_ CRYPT_PKCS8_EXPORT_PARAMS* psExportParams,                 // in
    _In_ DWORD dwFlags,                                             // in
    _In_opt_ void* pvAuxInfo,                                       // in
    _Out_writes_bytes_opt_ (*pcbPrivateKeyBlob) BYTE* pbPrivateKeyBlob,   // out
    _Inout_ DWORD* pcbPrivateKeyBlob                                // in, out
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Compute the hash of the encoded public key info.
//
//  The public key info is encoded and then hashed.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptHashPublicKeyInfo(
    _In_opt_ HCRYPTPROV_LEGACY hCryptProv,
    _In_ ALG_ID Algid,
    _In_ DWORD dwFlags,
    _In_ DWORD dwCertEncodingType,
    _In_ PCERT_PUBLIC_KEY_INFO pInfo,
    _Out_writes_bytes_to_opt_(*pcbComputedHash, *pcbComputedHash) BYTE *pbComputedHash,
    _Inout_ DWORD *pcbComputedHash
    );

//+-------------------------------------------------------------------------
//  Convert a Name Value to a null terminated char string
//
//  Returns the number of characters converted including the terminating null
//  character. If psz is NULL or csz is 0, returns the required size of the
//  destination string (including the terminating null char).
//
//  If psz != NULL && csz != 0, returned psz is always NULL terminated.
//
//  Note: csz includes the NULL char.
//--------------------------------------------------------------------------
WINCRYPT32API
DWORD
WINAPI
CertRDNValueToStrA(
    _In_ DWORD dwValueType,
    _In_ PCERT_RDN_VALUE_BLOB pValue,
    _Out_writes_to_opt_(csz, return) LPSTR psz,
    _In_ DWORD csz
    );
//+-------------------------------------------------------------------------
//  Convert a Name Value to a null terminated char string
//
//  Returns the number of characters converted including the terminating null
//  character. If psz is NULL or csz is 0, returns the required size of the
//  destination string (including the terminating null char).
//
//  If psz != NULL && csz != 0, returned psz is always NULL terminated.
//
//  Note: csz includes the NULL char.
//--------------------------------------------------------------------------
WINCRYPT32API
DWORD
WINAPI
CertRDNValueToStrW(
    _In_ DWORD dwValueType,
    _In_ PCERT_RDN_VALUE_BLOB pValue,
    _Out_writes_to_opt_(csz, return) LPWSTR psz,
    _In_ DWORD csz
    );
#ifdef UNICODE
#define CertRDNValueToStr  CertRDNValueToStrW
#else
#define CertRDNValueToStr  CertRDNValueToStrA
#endif // !UNICODE

//+-------------------------------------------------------------------------
//  Convert the certificate name blob to a null terminated char string.
//
//  Follows the string representation of distinguished names specified in
//  RFC 1779. (Note, added double quoting "" for embedded quotes, quote
//  empty strings and don't quote strings containing consecutive spaces).
//  RDN values of type CERT_RDN_ENCODED_BLOB or CERT_RDN_OCTET_STRING are
//  formatted in hexadecimal (e.g. #0A56CF).
//
//  The name string is formatted according to the dwStrType:
//    CERT_SIMPLE_NAME_STR
//      The object identifiers are discarded. CERT_RDN entries are separated
//      by ", ". Multiple attributes per CERT_RDN are separated by " + ".
//      For example:
//          Microsoft, Joe Cool + Programmer
//    CERT_OID_NAME_STR
//      The object identifiers are included with a "=" separator from their
//      attribute value. CERT_RDN entries are separated by ", ".
//      Multiple attributes per CERT_RDN are separated by " + ". For example:
//          2.5.4.11=Microsoft, 2.5.4.3=Joe Cool + 2.5.4.12=Programmer
//    CERT_X500_NAME_STR
//      The object identifiers are converted to their X500 key name. Otherwise,
//      same as CERT_OID_NAME_STR. If the object identifier doesn't have
//      a corresponding X500 key name, then, the object identifier is used with
//      a "OID." prefix. For example:
//          OU=Microsoft, CN=Joe Cool + T=Programmer, OID.1.2.3.4.5.6=Unknown
//    CERT_XML_NAME_STR
//      The object identifiers are converted the same as the above
//      CERT_X500_NAME_STR. However, formatted as sequence of XML elements.
//      Here's an example:
//          <CN>cart.barnesandnoble.com</CN>
//          <OU>Terms of use at www.verisign.com/rpa (c)00</OU>
//          <OU rDNAttribute="true">IT Operations</OU>
//          <O>Barnesandnoble.com</O>
//          <L>New York</L>
//          <S>New York</S>
//          <C>US</C>
//          <RDN oid="1.2.3.4" type="string">name</RDN>
//          <RDN rDNAttribute="true" oid="1.2.1.3" type="encoded">0500</RDN>
//          <RDN oid="1.2.1.4" type="encoded">020135</RDN>
//          <RDN oid="1.2.2.5.3" type="octet">01FF7F</RDN>
//      Where:
//          Any XML markup characters are escaped:
//             L'&'   - L"&amp;"
//             L'<'   - L"&lt;"
//             L'>'   - L"&gt;"
//             L'\''  - L"&apos;"
//             L'\"'  - L"&quot;"
//          Will escape characters > 0x7F via chararacter references,
//          L"&#xXXXX;"
//
//          CERT_NAME_STR_REVERSE_FLAG and CERT_NAME_STR_CRLF_FLAG can be set.
//          The following quoting, semicolon and plus semantics aren't
//          applicable. The "+" is replaced with rDNAttribute="true".
//
//
//  We quote the RDN value if it contains leading or trailing whitespace
//  or one of the following characters: ",", "+", "=", """, "\n",  "<", ">",
//  "#" or ";". The quoting character is ". If the the RDN Value contains
//  a " it is double quoted (""). For example:
//      OU="  Microsoft", CN="Joe ""Cool""" + T="Programmer, Manager"
//
//  CERT_NAME_STR_SEMICOLON_FLAG can be or'ed into dwStrType to replace
//  the ", " separator with a "; " separator.
//
//  CERT_NAME_STR_CRLF_FLAG can be or'ed into dwStrType to replace
//  the ", " separator with a "\r\n" separator.
//
//  CERT_NAME_STR_NO_PLUS_FLAG can be or'ed into dwStrType to replace the
//  " + " separator with a single space, " ".
//
//  CERT_NAME_STR_NO_QUOTING_FLAG can be or'ed into dwStrType to inhibit
//  the above quoting.
//
//  CERT_NAME_STR_REVERSE_FLAG can be or'ed into dwStrType to reverse the
//  order of the RDNs before converting to the string.
//
//  By default, CERT_RDN_T61_STRING encoded values are initially decoded
//  as UTF8. If the UTF8 decoding fails, then, decoded as 8 bit characters.
//  CERT_NAME_STR_DISABLE_IE4_UTF8_FLAG can be or'ed into dwStrType to
//  skip the initial attempt to decode as UTF8.
//
//  CERT_NAME_STR_ENABLE_PUNYCODE_FLAG can be or'ed into dwStrType to enable
//  encoding/decoding of unicode characters in email RDN value.
//
//  Returns the number of characters converted including the terminating null
//  character. If psz is NULL or csz is 0, returns the required size of the
//  destination string (including the terminating null char).
//
//  If psz != NULL && csz != 0, returned psz is always NULL terminated.
//
//  Note: csz includes the NULL char.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//--------------------------------------------------------------------------

WINCRYPT32API
DWORD
WINAPI
CertNameToStrA(
    _In_ DWORD dwCertEncodingType,
    _In_ PCERT_NAME_BLOB pName,
    _In_ DWORD dwStrType,
    _Out_writes_to_opt_(csz, return) LPSTR psz,
    _In_ DWORD csz
    );
WINCRYPT32API
DWORD
WINAPI
CertNameToStrW(
    _In_ DWORD dwCertEncodingType,
    _In_ PCERT_NAME_BLOB pName,
    _In_ DWORD dwStrType,
    _Out_writes_to_opt_(csz, return) LPWSTR psz,
    _In_ DWORD csz
    );
#ifdef UNICODE
#define CertNameToStr  CertNameToStrW
#else
#define CertNameToStr  CertNameToStrA
#endif // !UNICODE

// certenrolld_begin -- CERT_NAME_STR_*_FLAG
//+-------------------------------------------------------------------------
//  Certificate name string types
//--------------------------------------------------------------------------
#define CERT_SIMPLE_NAME_STR        1
#define CERT_OID_NAME_STR           2
#define CERT_X500_NAME_STR          3
#define CERT_XML_NAME_STR           4

//+-------------------------------------------------------------------------
//  Certificate name string type flags OR'ed with the above types
//--------------------------------------------------------------------------
#define CERT_NAME_STR_SEMICOLON_FLAG    0x40000000
#define CERT_NAME_STR_NO_PLUS_FLAG      0x20000000
#define CERT_NAME_STR_NO_QUOTING_FLAG   0x10000000
#define CERT_NAME_STR_CRLF_FLAG         0x08000000
#define CERT_NAME_STR_COMMA_FLAG        0x04000000
#define CERT_NAME_STR_REVERSE_FLAG      0x02000000
#define CERT_NAME_STR_FORWARD_FLAG      0x01000000

#define CERT_NAME_STR_DISABLE_IE4_UTF8_FLAG     0x00010000
#define CERT_NAME_STR_ENABLE_T61_UNICODE_FLAG   0x00020000
#define CERT_NAME_STR_ENABLE_UTF8_UNICODE_FLAG  0x00040000
#define CERT_NAME_STR_FORCE_UTF8_DIR_STR_FLAG   0x00080000
#define CERT_NAME_STR_DISABLE_UTF8_DIR_STR_FLAG 0x00100000
#define CERT_NAME_STR_ENABLE_PUNYCODE_FLAG      0x00200000
//#define CERT_NAME_STR_RESERVED00800000          0x00800000
// certenrolld_end


//+-------------------------------------------------------------------------
//  Convert the null terminated X500 string to an encoded certificate name.
//
//  The input string is expected to be formatted the same as the output
//  from the above CertNameToStr API.
//
//  The CERT_SIMPLE_NAME_STR type and CERT_XML_NAME_STR aren't supported.
//  Otherwise, when dwStrType
//  is set to 0, CERT_OID_NAME_STR or CERT_X500_NAME_STR, allow either a
//  case insensitive X500 key (CN=), case insensitive "OID." prefixed
//  object identifier (OID.1.2.3.4.5.6=) or an object identifier (1.2.3.4=).
//
//  If no flags are OR'ed into dwStrType, then, allow "," or ";" as RDN
//  separators and "+" as the multiple RDN value separator. Quoting is
//  supported. A quote may be included in a quoted value by double quoting,
//  for example (CN="Joe ""Cool"""). A value starting with a "#" is treated
//  as ascii hex and converted to a CERT_RDN_OCTET_STRING. Embedded whitespace
//  is skipped (1.2.3 = # AB CD 01  is the same as 1.2.3=#ABCD01).
//
//  Whitespace surrounding the keys, object identifers and values is removed.
//
//  CERT_NAME_STR_COMMA_FLAG can be or'ed into dwStrType to only allow the
//  "," as the RDN separator.
//
//  CERT_NAME_STR_SEMICOLON_FLAG can be or'ed into dwStrType to only allow the
//  ";" as the RDN separator.
//
//  CERT_NAME_STR_CRLF_FLAG can be or'ed into dwStrType to only allow
//  "\r" or "\n" as the RDN separator.
//
//  CERT_NAME_STR_NO_PLUS_FLAG can be or'ed into dwStrType to ignore "+"
//  as a separator and not allow multiple values per RDN.
//
//  CERT_NAME_STR_NO_QUOTING_FLAG can be or'ed into dwStrType to inhibit
//  quoting.
//
//  CERT_NAME_STR_REVERSE_FLAG can be or'ed into dwStrType to reverse the
//  order of the RDNs after converting from the string and before encoding.
//
//  CERT_NAME_STR_FORWARD_FLAG can be or'ed into dwStrType to defeat setting
//  CERT_NAME_STR_REVERSE_FLAG, if reverse order becomes the default.
//
//  CERT_NAME_STR_ENABLE_T61_UNICODE_FLAG can be or'ed into dwStrType to
//  to select the CERT_RDN_T61_STRING encoded value type instead of
//  CERT_RDN_UNICODE_STRING if all the UNICODE characters are <= 0xFF.
//
//  CERT_NAME_STR_ENABLE_UTF8_UNICODE_FLAG can be or'ed into dwStrType to
//  to select the CERT_RDN_UTF8_STRING encoded value type instead of
//  CERT_RDN_UNICODE_STRING.
//
//  CERT_NAME_STR_FORCE_UTF8_DIR_STR_FLAG can be or'ed into dwStrType
//  to force the CERT_RDN_UTF8_STRING encoded value type instead of
//  allowing CERT_RDN_PRINTABLE_STRING for DirectoryString types.
//  Applies to the X500 Keys below which allow "Printable, Unicode".
//  Also, enables CERT_NAME_STR_ENABLE_UTF8_UNICODE_FLAG.
//
//  CERT_NAME_STR_DISABLE_UTF8_DIR_STR_FLAG can be or'ed into dwStrType to
//  defeat setting CERT_NAME_STR_FORCE_UTF8_DIR_STR_FLAG, if forcing UTF-8
//  becomes the default.
//
//  Support the following X500 Keys:
//
//  Key         Object Identifier               RDN Value Type(s)
//  ---         -----------------               -----------------
//  CN          szOID_COMMON_NAME               Printable, Unicode
//  L           szOID_LOCALITY_NAME             Printable, Unicode
//  O           szOID_ORGANIZATION_NAME         Printable, Unicode
//  OU          szOID_ORGANIZATIONAL_UNIT_NAME  Printable, Unicode
//  E           szOID_RSA_emailAddr             Only IA5
//  Email       szOID_RSA_emailAddr             Only IA5
//  C           szOID_COUNTRY_NAME              Only Printable
//  S           szOID_STATE_OR_PROVINCE_NAME    Printable, Unicode
//  ST          szOID_STATE_OR_PROVINCE_NAME    Printable, Unicode
//  STREET      szOID_STREET_ADDRESS            Printable, Unicode
//  T           szOID_TITLE                     Printable, Unicode
//  Title       szOID_TITLE                     Printable, Unicode
//  G           szOID_GIVEN_NAME                Printable, Unicode
//  GN          szOID_GIVEN_NAME                Printable, Unicode
//  GivenName   szOID_GIVEN_NAME                Printable, Unicode
//  I           szOID_INITIALS                  Printable, Unicode
//  Initials    szOID_INITIALS                  Printable, Unicode
//  SN          szOID_SUR_NAME                  Printable, Unicode
//  DC          szOID_DOMAIN_COMPONENT          IA5, UTF8
//  SERIALNUMBER szOID_DEVICE_SERIAL_NUMBER     Only Printable
//
//  Note, T61 is selected instead of Unicode if
//  CERT_NAME_STR_ENABLE_T61_UNICODE_FLAG is set and all the unicode
//  characters are <= 0xFF.
//
//  Note, UTF8 is selected instead of Unicode if
//  CERT_NAME_STR_ENABLE_UTF8_UNICODE_FLAG is set.
//
//  Returns TRUE if successfully parsed the input string and encoded
//  the name.
//
//  If the input string is detected to be invalid, *ppszError is updated
//  to point to the beginning of the invalid character sequence. Otherwise,
//  *ppszError is set to NULL. *ppszError is updated with a non-NULL pointer
//  for the following errors:
//      CRYPT_E_INVALID_X500_STRING
//      CRYPT_E_INVALID_NUMERIC_STRING
//      CRYPT_E_INVALID_PRINTABLE_STRING
//      CRYPT_E_INVALID_IA5_STRING
//
//  ppszError can be set to NULL if not interested in getting a pointer
//  to the invalid character sequence.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertStrToNameA(
    _In_ DWORD dwCertEncodingType,
    _In_ LPCSTR pszX500,
    _In_ DWORD dwStrType,
    _Reserved_ void *pvReserved,
    _Out_writes_bytes_to_opt_(*pcbEncoded, *pcbEncoded) BYTE *pbEncoded,
    _Inout_ DWORD *pcbEncoded,
    _Outptr_opt_result_maybenull_ LPCSTR *ppszError
    );
//+-------------------------------------------------------------------------
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertStrToNameW(
    _In_ DWORD dwCertEncodingType,
    _In_ LPCWSTR pszX500,
    _In_ DWORD dwStrType,
    _Reserved_ void *pvReserved,
    _Out_writes_bytes_to_opt_(*pcbEncoded, *pcbEncoded) BYTE *pbEncoded,
    _Inout_ DWORD *pcbEncoded,
    _Outptr_opt_result_maybenull_ LPCWSTR *ppszError
    );
#ifdef UNICODE
#define CertStrToName  CertStrToNameW
#else
#define CertStrToName  CertStrToNameA
#endif // !UNICODE


//+-------------------------------------------------------------------------
//  Get the subject or issuer name from the certificate and
//  according to the specified format type, convert to a null terminated
//  character string.
//
//  CERT_NAME_ISSUER_FLAG can be set to get the issuer's name. Otherwise,
//  gets the subject's name.
//
//  By default, CERT_RDN_T61_STRING encoded values are initially decoded
//  as UTF8. If the UTF8 decoding fails, then, decoded as 8 bit characters.
//  CERT_NAME_DISABLE_IE4_UTF8_FLAG can be set in dwFlags to
//  skip the initial attempt to decode as UTF8.
//
//  The name string is formatted according to the dwType:
//    CERT_NAME_EMAIL_TYPE
//      If the certificate has a Subject Alternative Name extension (for
//      issuer, Issuer Alternative Name), searches for first rfc822Name choice.
//      If the rfc822Name choice isn't found in the extension, searches the
//      Subject Name field for the Email OID, "1.2.840.113549.1.9.1".
//      If the rfc822Name or Email OID is found, returns the string. Otherwise,
//      returns an empty string (returned character count is 1).
//    CERT_NAME_DNS_TYPE
//      If the certificate has a Subject Alternative Name extension (for
//      issuer, Issuer Alternative Name), searches for first DNSName choice.
//      If the DNSName choice isn't found in the extension, searches the
//      Subject Name field for the CN OID, "2.5.4.3".
//      If the DNSName or CN OID is found, returns the string. Otherwise,
//      returns an empty string.
//    CERT_NAME_URL_TYPE
//      If the certificate has a Subject Alternative Name extension (for
//      issuer, Issuer Alternative Name), searches for first URL choice.
//      If the URL choice is found, returns the string. Otherwise,
//      returns an empty string.
//    CERT_NAME_UPN_TYPE
//      If the certificate has a Subject Alternative Name extension,
//      searches the OtherName choices looking for a
//      pszObjId == szOID_NT_PRINCIPAL_NAME, "1.3.6.1.4.1.311.20.2.3".
//      If the UPN OID is found, the blob is decoded as a
//      X509_UNICODE_ANY_STRING and the decoded string is returned.
//      Otherwise, returns an empty string.
//    CERT_NAME_RDN_TYPE
//      Converts the Subject Name blob by calling CertNameToStr. pvTypePara
//      points to a DWORD containing the dwStrType passed to CertNameToStr.
//      If the Subject Name field is empty and the certificate has a
//      Subject Alternative Name extension, searches for and converts
//      the first directoryName choice.
//    CERT_NAME_ATTR_TYPE
//      pvTypePara points to the Object Identifier specifying the name attribute
//      to be returned. For example, to get the CN,
//      pvTypePara = szOID_COMMON_NAME ("2.5.4.3"). Searches, the Subject Name
//      field for the attribute.
//      If the Subject Name field is empty and the certificate has a
//      Subject Alternative Name extension, checks for
//      the first directoryName choice and searches it.
//
//      Note, searches the RDNs in reverse order.
//
//    CERT_NAME_SIMPLE_DISPLAY_TYPE
//      Iterates through the following list of name attributes and searches
//      the Subject Name and then the Subject Alternative Name extension
//      for the first occurrence of:
//          szOID_COMMON_NAME ("2.5.4.3")
//          szOID_ORGANIZATIONAL_UNIT_NAME ("2.5.4.11")
//          szOID_ORGANIZATION_NAME ("2.5.4.10")
//          szOID_RSA_emailAddr ("1.2.840.113549.1.9.1")
//
//      If none of the above attributes is found, then, searches the
//      Subject Alternative Name extension for a rfc822Name choice.
//
//      If still no match, then, returns the first attribute.
//
//      Note, like CERT_NAME_ATTR_TYPE, searches the RDNs in reverse order.
//
//    CERT_NAME_FRIENDLY_DISPLAY_TYPE
//      First checks if the certificate has a CERT_FRIENDLY_NAME_PROP_ID
//      property. If it does, then, this property is returned. Otherwise,
//      returns the above CERT_NAME_SIMPLE_DISPLAY_TYPE.
//
//  Returns the number of characters converted including the terminating null
//  character. If pwszNameString is NULL or cchNameString is 0, returns the
//  required size of the destination string (including the terminating null
//  char). If the specified name type isn't found. returns an empty string
//  with a returned character count of 1.
//
//  If pwszNameString != NULL && cwszNameString != 0, returned pwszNameString
//  is always NULL terminated.
//
//  Note: cchNameString includes the NULL char.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//--------------------------------------------------------------------------
WINCRYPT32API
DWORD
WINAPI
CertGetNameStringA(
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ DWORD dwType,
    _In_ DWORD dwFlags,
    _In_opt_ void *pvTypePara,
    _Out_writes_to_opt_(cchNameString, return) LPSTR pszNameString,
    _In_ DWORD cchNameString
    );
//+-------------------------------------------------------------------------
//--------------------------------------------------------------------------
WINCRYPT32API
DWORD
WINAPI
CertGetNameStringW(
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ DWORD dwType,
    _In_ DWORD dwFlags,
    _In_opt_ void *pvTypePara,
    _Out_writes_to_opt_(cchNameString, return) LPWSTR pszNameString,
    _In_ DWORD cchNameString
    );
#ifdef UNICODE
#define CertGetNameString  CertGetNameStringW
#else
#define CertGetNameString  CertGetNameStringA
#endif // !UNICODE

//+-------------------------------------------------------------------------
//  Certificate name types
//--------------------------------------------------------------------------
#define CERT_NAME_EMAIL_TYPE            1
#define CERT_NAME_RDN_TYPE              2
#define CERT_NAME_ATTR_TYPE             3
#define CERT_NAME_SIMPLE_DISPLAY_TYPE   4
#define CERT_NAME_FRIENDLY_DISPLAY_TYPE 5
#define CERT_NAME_DNS_TYPE              6
#define CERT_NAME_URL_TYPE              7
#define CERT_NAME_UPN_TYPE              8

//+-------------------------------------------------------------------------
//  Certificate name flags
//--------------------------------------------------------------------------
#define CERT_NAME_ISSUER_FLAG           0x1
#define CERT_NAME_DISABLE_IE4_UTF8_FLAG 0x00010000


// Following is only applicable to CERT_NAME_DNS_TYPE. When set returns
// all names not just the first one. Returns a multi-string. Each string
// will be null terminated. The last string will be double null terminated. 
#define CERT_NAME_SEARCH_ALL_NAMES_FLAG 0x2


//+=========================================================================
//  Simplified Cryptographic Message Data Structures and APIs
//==========================================================================


//+-------------------------------------------------------------------------
//              Conventions for the *pb and *pcb output parameters:
//
//              Upon entry to the function:
//                  if pcb is OPTIONAL && pcb == NULL, then,
//                      No output is returned
//                  else if pb == NULL && pcb != NULL, then,
//                      Length only determination. No length error is
//                      returned.
//                  otherwise where (pb != NULL && pcb != NULL && *pcb != 0)
//                      Output is returned. If *pcb isn't big enough a
//                      length error is returned. In all cases *pcb is updated
//                      with the actual length needed/returned.
//--------------------------------------------------------------------------


//+-------------------------------------------------------------------------
//  Type definitions of the parameters used for doing the cryptographic
//  operations.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  Callback to get and verify the signer's certificate.
//
//  Passed the CertId of the signer (its Issuer and SerialNumber) and a
//  handle to its cryptographic signed message's cert store.
//
//  For CRYPT_E_NO_SIGNER, called with pSignerId == NULL.
//
//  For a valid signer certificate, returns a pointer to a read only
//  CERT_CONTEXT. The returned CERT_CONTEXT is either obtained from a
//  cert store or was created via CertCreateCertificateContext. For either case,
//  its freed via CertFreeCertificateContext.
//
//  If a valid certificate isn't found, this callback returns NULL with
//  LastError set via SetLastError().
//
//  The NULL implementation tries to get the Signer certificate from the
//  message cert store. It doesn't verify the certificate.
//
//  Note, if the KEYID choice was selected for a CMS SignerId, then, the
//  SerialNumber is 0 and the Issuer is encoded containing a single RDN with a
//  single Attribute whose OID is szOID_KEYID_RDN, value type is
//  CERT_RDN_OCTET_STRING and value is the KEYID. When the
//  CertGetSubjectCertificateFromStore and
//  CertFindCertificateInStore(CERT_FIND_SUBJECT_CERT) APIs see this
//  special KEYID Issuer and SerialNumber, they do a KEYID match.
//--------------------------------------------------------------------------
typedef PCCERT_CONTEXT (WINAPI *PFN_CRYPT_GET_SIGNER_CERTIFICATE)(
    _Inout_opt_ void *pvGetArg,
    _In_ DWORD dwCertEncodingType,
    _In_ PCERT_INFO pSignerId,    // Only the Issuer and SerialNumber
                                // fields have been updated
    _In_ HCERTSTORE hMsgCertStore
    );

//+-------------------------------------------------------------------------
//  The CRYPT_SIGN_MESSAGE_PARA are used for signing messages using the
//  specified signing certificate context.
//
//  Either the CERT_KEY_PROV_HANDLE_PROP_ID or CERT_KEY_PROV_INFO_PROP_ID must
//  be set for each rgpSigningCert[]. Either one specifies the private
//  signature key to use.
//
//  If any certificates and/or CRLs are to be included in the signed message,
//  then, the MsgCert and MsgCrl parameters need to be updated. If the
//  rgpSigningCerts are to be included, then, they must also be in the
//  rgpMsgCert array.
//
//  cbSize must be set to the sizeof(CRYPT_SIGN_MESSAGE_PARA) or else
//  LastError will be updated with E_INVALIDARG.
//
//  pvHashAuxInfo currently isn't used and must be set to NULL.
//
//  dwFlags normally is set to 0. However, if the encoded output
//  is to be a CMSG_SIGNED inner content of an outer cryptographic message,
//  such as a CMSG_ENVELOPED, then, the CRYPT_MESSAGE_BARE_CONTENT_OUT_FLAG
//  should be set. If not set, then it would be encoded as an inner content
//  type of CMSG_DATA.
//
//  dwInnerContentType is normally set to 0. It needs to be set if the
//  ToBeSigned input is the encoded output of another cryptographic
//  message, such as, an CMSG_ENVELOPED. When set, it's one of the cryptographic
//  message types, for example, CMSG_ENVELOPED.
//
//  If the inner content of a nested cryptographic message is data (CMSG_DATA
//  the default), then, neither dwFlags or dwInnerContentType need to be set.
//
//  For CMS messages, CRYPT_MESSAGE_ENCAPSULATED_CONTENT_OUT_FLAG may be
//  set to encapsulate nonData inner content within an OCTET STRING.
//
//  For CMS messages, CRYPT_MESSAGE_KEYID_SIGNER_FLAG may be set to identify
//  signers by their Key Identifier and not their Issuer and Serial Number.
//
//  The CRYPT_MESSAGE_SILENT_KEYSET_FLAG can be set to suppress any UI by the
//  CSP. See CryptAcquireContext's CRYPT_SILENT flag for more details.
//
//  If HashEncryptionAlgorithm is present and not NULL its used instead of
//  the SigningCert's PublicKeyInfo.Algorithm.
//
//  Note, for RSA, the hash encryption algorithm is normally the same as
//  the public key algorithm. For DSA, the hash encryption algorithm is
//  normally a DSS signature algorithm.
//
//  pvHashEncryptionAuxInfo currently isn't used and must be set to NULL if
//  present in the data structure.
//--------------------------------------------------------------------------
typedef struct _CRYPT_SIGN_MESSAGE_PARA {
    DWORD                       cbSize;
    DWORD                       dwMsgEncodingType;
    PCCERT_CONTEXT              pSigningCert;
    CRYPT_ALGORITHM_IDENTIFIER  HashAlgorithm;
    void                        *pvHashAuxInfo;
    DWORD                       cMsgCert;
    PCCERT_CONTEXT              *rgpMsgCert;
    DWORD                       cMsgCrl;
    PCCRL_CONTEXT               *rgpMsgCrl;
    DWORD                       cAuthAttr;
    PCRYPT_ATTRIBUTE            rgAuthAttr;
    DWORD                       cUnauthAttr;
    PCRYPT_ATTRIBUTE            rgUnauthAttr;
    DWORD                       dwFlags;
    DWORD                       dwInnerContentType;

#ifdef CRYPT_SIGN_MESSAGE_PARA_HAS_CMS_FIELDS
    // This is also referred to as the SignatureAlgorithm
    CRYPT_ALGORITHM_IDENTIFIER  HashEncryptionAlgorithm;
    void                        *pvHashEncryptionAuxInfo;
#endif
} CRYPT_SIGN_MESSAGE_PARA, *PCRYPT_SIGN_MESSAGE_PARA;

#define CRYPT_MESSAGE_BARE_CONTENT_OUT_FLAG         0x00000001

// When set, nonData type inner content is encapsulated within an
// OCTET STRING
#define CRYPT_MESSAGE_ENCAPSULATED_CONTENT_OUT_FLAG 0x00000002

// When set, signers are identified by their Key Identifier and not
// their Issuer and Serial Number.
#define CRYPT_MESSAGE_KEYID_SIGNER_FLAG             0x00000004

// When set, suppresses any UI by the CSP.
// See CryptAcquireContext's CRYPT_SILENT flag for more details.
#define CRYPT_MESSAGE_SILENT_KEYSET_FLAG            0x00000040

//+-------------------------------------------------------------------------
//  The CRYPT_VERIFY_MESSAGE_PARA are used to verify signed messages.
//
//  hCryptProv is used to do hashing and signature verification.
//
//  The dwCertEncodingType specifies the encoding type of the certificates
//  and/or CRLs in the message.
//
//  pfnGetSignerCertificate is called to get and verify the message signer's
//  certificate.
//
//  cbSize must be set to the sizeof(CRYPT_VERIFY_MESSAGE_PARA) or else
//  LastError will be updated with E_INVALIDARG.
//--------------------------------------------------------------------------
typedef struct _CRYPT_VERIFY_MESSAGE_PARA {
    DWORD                               cbSize;
    DWORD                               dwMsgAndCertEncodingType;
    HCRYPTPROV_LEGACY                   hCryptProv;
    PFN_CRYPT_GET_SIGNER_CERTIFICATE    pfnGetSignerCertificate;
    void                                *pvGetArg;

#ifdef CRYPT_VERIFY_MESSAGE_PARA_HAS_EXTRA_FIELDS

    // Note, if you #define CRYPT_VERIFY_MESSAGE_PARA_HAS_EXTRA_FIELDS,
    // then, you must zero all unused fields in this data structure.
    // More fields could be added in a future release.

    //
    // The following is set to check for Strong and Restricted Signatures
    //
    PCCERT_STRONG_SIGN_PARA             pStrongSignPara;

#endif
} CRYPT_VERIFY_MESSAGE_PARA, *PCRYPT_VERIFY_MESSAGE_PARA;

//+-------------------------------------------------------------------------
//  The CRYPT_ENCRYPT_MESSAGE_PARA are used for encrypting messages.
//
//  hCryptProv is used to do content encryption, recipient key
//  encryption, and recipient key export. Its private key
//  isn't used.
//
//  Currently, pvEncryptionAuxInfo is only defined for RC2 or RC4 encryption
//  algorithms. Otherwise, its not used and must be set to NULL.
//  See CMSG_RC2_AUX_INFO for the RC2 encryption algorithms.
//  See CMSG_RC4_AUX_INFO for the RC4 encryption algorithms.
//
//  To enable SP3 compatible encryption, pvEncryptionAuxInfo should point to
//  a CMSG_SP3_COMPATIBLE_AUX_INFO data structure.
//
//  cbSize must be set to the sizeof(CRYPT_ENCRYPT_MESSAGE_PARA) or else
//  LastError will be updated with E_INVALIDARG.
//
//  dwFlags normally is set to 0. However, if the encoded output
//  is to be a CMSG_ENVELOPED inner content of an outer cryptographic message,
//  such as a CMSG_SIGNED, then, the CRYPT_MESSAGE_BARE_CONTENT_OUT_FLAG
//  should be set. If not set, then it would be encoded as an inner content
//  type of CMSG_DATA.
//
//  dwInnerContentType is normally set to 0. It needs to be set if the
//  ToBeEncrypted input is the encoded output of another cryptographic
//  message, such as, an CMSG_SIGNED. When set, it's one of the cryptographic
//  message types, for example, CMSG_SIGNED.
//
//  If the inner content of a nested cryptographic message is data (CMSG_DATA
//  the default), then, neither dwFlags or dwInnerContentType need to be set.
//
//  For CMS messages, CRYPT_MESSAGE_ENCAPSULATED_CONTENT_OUT_FLAG may be
//  set to encapsulate nonData inner content within an OCTET STRING before
//  encrypting.
//
//  For CMS messages, CRYPT_MESSAGE_KEYID_RECIPIENT_FLAG may be set to identify
//  recipients by their Key Identifier and not their Issuer and Serial Number.
//--------------------------------------------------------------------------
typedef struct _CRYPT_ENCRYPT_MESSAGE_PARA {
    DWORD                       cbSize;
    DWORD                       dwMsgEncodingType;
    HCRYPTPROV_LEGACY           hCryptProv;
    CRYPT_ALGORITHM_IDENTIFIER  ContentEncryptionAlgorithm;
    void                        *pvEncryptionAuxInfo;
    DWORD                       dwFlags;
    DWORD                       dwInnerContentType;
} CRYPT_ENCRYPT_MESSAGE_PARA, *PCRYPT_ENCRYPT_MESSAGE_PARA;

// When set, recipients are identified by their Key Identifier and not
// their Issuer and Serial Number.
#define CRYPT_MESSAGE_KEYID_RECIPIENT_FLAG          0x4

//+-------------------------------------------------------------------------
//  The CRYPT_DECRYPT_MESSAGE_PARA are used for decrypting messages.
//
//  The CertContext to use for decrypting a message is obtained from one
//  of the specified cert stores. An encrypted message can have one or
//  more recipients. The recipients are identified by their CertId (Issuer
//  and SerialNumber). The cert stores are searched to find the CertContext
//  corresponding to the CertId.
//
//  For CMS, the recipients may also be identified by their KeyId.
//  CMS also allows Key Agreement (Diffie Hellman) in addition to
//  Key Transport (RSA) recipients.
//
//  Only CertContexts in the store with either
//  the CERT_KEY_PROV_HANDLE_PROP_ID or CERT_KEY_PROV_INFO_PROP_ID set
//  can be used. Either property specifies the private exchange key to use.
//
//  cbSize must be set to the sizeof(CRYPT_DECRYPT_MESSAGE_PARA) or else
//  LastError will be updated with E_INVALIDARG.
//--------------------------------------------------------------------------
typedef struct _CRYPT_DECRYPT_MESSAGE_PARA {
    DWORD                   cbSize;
    DWORD                   dwMsgAndCertEncodingType;
    DWORD                   cCertStore;
    HCERTSTORE              *rghCertStore;

#ifdef CRYPT_DECRYPT_MESSAGE_PARA_HAS_EXTRA_FIELDS
// The above defined, CRYPT_MESSAGE_SILENT_KEYSET_FLAG, can be set to
// suppress UI by the CSP.  See CryptAcquireContext's CRYPT_SILENT
// flag for more details.

    DWORD                   dwFlags;
#endif

} CRYPT_DECRYPT_MESSAGE_PARA, *PCRYPT_DECRYPT_MESSAGE_PARA;

//+-------------------------------------------------------------------------
//  The CRYPT_HASH_MESSAGE_PARA are used for hashing or unhashing
//  messages.
//
//  hCryptProv is used to compute the hash.
//
//  pvHashAuxInfo currently isn't used and must be set to NULL.
//
//  cbSize must be set to the sizeof(CRYPT_HASH_MESSAGE_PARA) or else
//  LastError will be updated with E_INVALIDARG.
//--------------------------------------------------------------------------
typedef struct _CRYPT_HASH_MESSAGE_PARA {
    DWORD                       cbSize;
    DWORD                       dwMsgEncodingType;
    HCRYPTPROV_LEGACY           hCryptProv;
    CRYPT_ALGORITHM_IDENTIFIER  HashAlgorithm;
    void                        *pvHashAuxInfo;
} CRYPT_HASH_MESSAGE_PARA, *PCRYPT_HASH_MESSAGE_PARA;


//+-------------------------------------------------------------------------
//  The CRYPT_KEY_SIGN_MESSAGE_PARA are used for signing messages until a
//  certificate has been created for the signature key.
//
//  pvHashAuxInfo currently isn't used and must be set to NULL.
//
//  If PubKeyAlgorithm isn't set, defaults to szOID_RSA_RSA.
//
//  cbSize must be set to the sizeof(CRYPT_KEY_SIGN_MESSAGE_PARA) or else
//  LastError will be updated with E_INVALIDARG.
//--------------------------------------------------------------------------
typedef struct _CRYPT_KEY_SIGN_MESSAGE_PARA {
    DWORD                       cbSize;
    DWORD                       dwMsgAndCertEncodingType;

    // NCryptIsKeyHandle() is called to determine the union choice.
    union {
        HCRYPTPROV                  hCryptProv;
        NCRYPT_KEY_HANDLE           hNCryptKey;
    } DUMMYUNIONNAME;

    // not applicable for hNCryptKey choice
    DWORD                       dwKeySpec;

    CRYPT_ALGORITHM_IDENTIFIER  HashAlgorithm;
    void                        *pvHashAuxInfo;
    // This is also referred to as the SignatureAlgorithm
    CRYPT_ALGORITHM_IDENTIFIER  PubKeyAlgorithm;
} CRYPT_KEY_SIGN_MESSAGE_PARA, *PCRYPT_KEY_SIGN_MESSAGE_PARA;

//+-------------------------------------------------------------------------
//  The CRYPT_KEY_VERIFY_MESSAGE_PARA are used to verify signed messages without
//  a certificate for the signer.
//
//  Normally used until a certificate has been created for the key.
//
//  hCryptProv is used to do hashing and signature verification.
//
//  cbSize must be set to the sizeof(CRYPT_KEY_VERIFY_MESSAGE_PARA) or else
//  LastError will be updated with E_INVALIDARG.
//--------------------------------------------------------------------------
typedef struct _CRYPT_KEY_VERIFY_MESSAGE_PARA {
    DWORD                   cbSize;
    DWORD                   dwMsgEncodingType;
    HCRYPTPROV_LEGACY       hCryptProv;
} CRYPT_KEY_VERIFY_MESSAGE_PARA, *PCRYPT_KEY_VERIFY_MESSAGE_PARA;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Sign the message.
//
//  If fDetachedSignature is TRUE, the "to be signed" content isn't included
//  in the encoded signed blob.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptSignMessage(
    _In_ PCRYPT_SIGN_MESSAGE_PARA pSignPara,
    _In_ BOOL fDetachedSignature,
    _In_ DWORD cToBeSigned,
    _In_reads_opt_(cToBeSigned) const BYTE *rgpbToBeSigned[],
    _In_reads_(cToBeSigned) DWORD rgcbToBeSigned[],
    _Out_writes_bytes_to_opt_(*pcbSignedBlob, *pcbSignedBlob) BYTE *pbSignedBlob,
    _Inout_ DWORD *pcbSignedBlob
    );

//+-------------------------------------------------------------------------
//  Verify a signed message.
//
//  If pbDecoded == NULL, then, *pcbDecoded is implicitly set to 0 on input.
//  For *pcbDecoded == 0 && ppSignerCert == NULL on input, the signer isn't
//  verified.
//
//  A message might have more than one signer. Set dwSignerIndex to iterate
//  through all the signers. dwSignerIndex == 0 selects the first signer.
//
//  pVerifyPara's pfnGetSignerCertificate is called to get the signer's
//  certificate.
//
//  For a verified signer and message, *ppSignerCert is updated
//  with the CertContext of the signer. It must be freed by calling
//  CertFreeCertificateContext. Otherwise, *ppSignerCert is set to NULL.
//
//  ppSignerCert can be NULL, indicating the caller isn't interested
//  in getting the CertContext of the signer.
//
//  pcbDecoded can be NULL, indicating the caller isn't interested in getting
//  the decoded content. Furthermore, if the message doesn't contain any
//  content or signers, then, pcbDecoded must be set to NULL, to allow the
//  pVerifyPara->pfnGetCertificate to be called. Normally, this would be
//  the case when the signed message contains only certficates and CRLs.
//  If pcbDecoded is NULL and the message doesn't have the indicated signer,
//  pfnGetCertificate is called with pSignerId set to NULL.
//
//  If the message doesn't contain any signers || dwSignerIndex > message's
//  SignerCount, then, an error is returned with LastError set to
//  CRYPT_E_NO_SIGNER. Also, for CRYPT_E_NO_SIGNER, pfnGetSignerCertificate
//  is still called with pSignerId set to NULL.
//
//  Note, an alternative way to get the certificates and CRLs from a
//  signed message is to call CryptGetMessageCertificates.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptVerifyMessageSignature(
    _In_ PCRYPT_VERIFY_MESSAGE_PARA pVerifyPara,
    _In_ DWORD dwSignerIndex,
    _In_reads_bytes_(cbSignedBlob) const BYTE *pbSignedBlob,
    _In_ DWORD cbSignedBlob,
    _Out_writes_bytes_to_opt_(*pcbDecoded, *pcbDecoded) BYTE *pbDecoded,
    _Inout_opt_ DWORD *pcbDecoded,
    _Outptr_opt_result_maybenull_ PCCERT_CONTEXT *ppSignerCert
    );

//+-------------------------------------------------------------------------
//  Returns the count of signers in the signed message. For no signers, returns
//  0. For an error returns -1 with LastError updated accordingly.
//--------------------------------------------------------------------------
WINCRYPT32API
LONG
WINAPI
CryptGetMessageSignerCount(
    _In_ DWORD dwMsgEncodingType,
    _In_reads_bytes_(cbSignedBlob) const BYTE *pbSignedBlob,
    _In_ DWORD cbSignedBlob
    );

//+-------------------------------------------------------------------------
//  Returns the cert store containing the message's certs and CRLs.
//  For an error, returns NULL with LastError updated.
//--------------------------------------------------------------------------
WINCRYPT32API
HCERTSTORE
WINAPI
CryptGetMessageCertificates(
    _In_ DWORD dwMsgAndCertEncodingType,
    _In_opt_ HCRYPTPROV_LEGACY hCryptProv,           // passed to CertOpenStore
    _In_ DWORD dwFlags,                   // passed to CertOpenStore
    _In_reads_bytes_(cbSignedBlob) const BYTE *pbSignedBlob,
    _In_ DWORD cbSignedBlob
    );

//+-------------------------------------------------------------------------
//  Verify a signed message containing detached signature(s).
//  The "to be signed" content is passed in separately. No
//  decoded output. Otherwise, identical to CryptVerifyMessageSignature.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptVerifyDetachedMessageSignature(
    _In_ PCRYPT_VERIFY_MESSAGE_PARA pVerifyPara,
    _In_ DWORD dwSignerIndex,
    _In_reads_bytes_(cbDetachedSignBlob) const BYTE *pbDetachedSignBlob,
    _In_ DWORD cbDetachedSignBlob,
    _In_ DWORD cToBeSigned,
    _In_reads_(cToBeSigned) const BYTE *rgpbToBeSigned[],
    _In_reads_(cToBeSigned) DWORD rgcbToBeSigned[],
    _Outptr_opt_result_maybenull_ PCCERT_CONTEXT *ppSignerCert
    );

//+-------------------------------------------------------------------------
//  Encrypts the message for the recipient(s).
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptEncryptMessage(
    _In_ PCRYPT_ENCRYPT_MESSAGE_PARA pEncryptPara,
    _In_ DWORD cRecipientCert,
    _In_reads_(cRecipientCert) PCCERT_CONTEXT rgpRecipientCert[],
    _In_reads_bytes_opt_(cbToBeEncrypted) const BYTE *pbToBeEncrypted,
    _In_ DWORD cbToBeEncrypted,
    _Out_writes_bytes_to_opt_(*pcbEncryptedBlob, *pcbEncryptedBlob) BYTE *pbEncryptedBlob,
    _Inout_ DWORD *pcbEncryptedBlob
    );

//+-------------------------------------------------------------------------
//  Decrypts the message.
//
//  If pbDecrypted == NULL, then, *pcbDecrypted is implicitly set to 0 on input.
//  For *pcbDecrypted == 0 && ppXchgCert == NULL on input, the message isn't
//  decrypted.
//
//  For a successfully decrypted message, *ppXchgCert is updated
//  with the CertContext used to decrypt. It must be freed by calling
//  CertStoreFreeCert. Otherwise, *ppXchgCert is set to NULL.
//
//  ppXchgCert can be NULL, indicating the caller isn't interested
//  in getting the CertContext used to decrypt.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptDecryptMessage(
    _In_ PCRYPT_DECRYPT_MESSAGE_PARA pDecryptPara,
    _In_reads_bytes_(cbEncryptedBlob) const BYTE *pbEncryptedBlob,
    _In_ DWORD cbEncryptedBlob,
    _Out_writes_bytes_to_opt_(*pcbDecrypted, *pcbDecrypted) BYTE *pbDecrypted,
    _Inout_opt_ DWORD *pcbDecrypted,
    _Outptr_opt_result_maybenull_ PCCERT_CONTEXT *ppXchgCert
    );

//+-------------------------------------------------------------------------
//  Sign the message and encrypt for the recipient(s). Does a CryptSignMessage
//  followed with a CryptEncryptMessage.
//
//  Note: this isn't the CMSG_SIGNED_AND_ENVELOPED. Its a CMSG_SIGNED
//  inside of an CMSG_ENVELOPED.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptSignAndEncryptMessage(
    _In_ PCRYPT_SIGN_MESSAGE_PARA pSignPara,
    _In_ PCRYPT_ENCRYPT_MESSAGE_PARA pEncryptPara,
    _In_ DWORD cRecipientCert,
    _In_reads_(cRecipientCert) PCCERT_CONTEXT rgpRecipientCert[],
    _In_reads_bytes_(cbToBeSignedAndEncrypted) const BYTE *pbToBeSignedAndEncrypted,
    _In_ DWORD cbToBeSignedAndEncrypted,
    _Out_writes_bytes_to_opt_(*pcbSignedAndEncryptedBlob, *pcbSignedAndEncryptedBlob) BYTE *pbSignedAndEncryptedBlob,
    _Inout_ DWORD *pcbSignedAndEncryptedBlob
    );

//+-------------------------------------------------------------------------
//  Decrypts the message and verifies the signer. Does a CryptDecryptMessage
//  followed with a CryptVerifyMessageSignature.
//
//  If pbDecrypted == NULL, then, *pcbDecrypted is implicitly set to 0 on input.
//  For *pcbDecrypted == 0 && ppSignerCert == NULL on input, the signer isn't
//  verified.
//
//  A message might have more than one signer. Set dwSignerIndex to iterate
//  through all the signers. dwSignerIndex == 0 selects the first signer.
//
//  The pVerifyPara's VerifySignerPolicy is called to verify the signer's
//  certificate.
//
//  For a successfully decrypted and verified message, *ppXchgCert and
//  *ppSignerCert are updated. They must be freed by calling
//  CertStoreFreeCert. Otherwise, they are set to NULL.
//
//  ppXchgCert and/or ppSignerCert can be NULL, indicating the
//  caller isn't interested in getting the CertContext.
//
//  Note: this isn't the CMSG_SIGNED_AND_ENVELOPED. Its a CMSG_SIGNED
//  inside of an CMSG_ENVELOPED.
//
//  The message always needs to be decrypted to allow access to the
//  signed message. Therefore, if ppXchgCert != NULL, its always updated.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptDecryptAndVerifyMessageSignature(
    _In_ PCRYPT_DECRYPT_MESSAGE_PARA pDecryptPara,
    _In_ PCRYPT_VERIFY_MESSAGE_PARA pVerifyPara,
    _In_ DWORD dwSignerIndex,
    _In_reads_bytes_(cbEncryptedBlob) const BYTE *pbEncryptedBlob,
    _In_ DWORD cbEncryptedBlob,
    _Out_writes_bytes_to_opt_(*pcbDecrypted, *pcbDecrypted) BYTE *pbDecrypted,
    _Inout_opt_ DWORD *pcbDecrypted,
    _Outptr_opt_result_maybenull_ PCCERT_CONTEXT *ppXchgCert,
    _Outptr_opt_result_maybenull_ PCCERT_CONTEXT *ppSignerCert
    );

//+-------------------------------------------------------------------------
//  Decodes a cryptographic message which may be one of the following types:
//    CMSG_DATA
//    CMSG_SIGNED
//    CMSG_ENVELOPED
//    CMSG_SIGNED_AND_ENVELOPED
//    CMSG_HASHED
//
//  dwMsgTypeFlags specifies the set of allowable messages. For example, to
//  decode either SIGNED or ENVELOPED messages, set dwMsgTypeFlags to:
//      CMSG_SIGNED_FLAG | CMSG_ENVELOPED_FLAG.
//
//  dwProvInnerContentType is only applicable when processing nested
//  crytographic messages. When processing an outer crytographic message
//  it must be set to 0. When decoding a nested cryptographic message
//  its the dwInnerContentType returned by a previous CryptDecodeMessage
//  of the outer message. The InnerContentType can be any of the CMSG types,
//  for example, CMSG_DATA, CMSG_SIGNED, ...
//
//  The optional *pdwMsgType is updated with the type of message.
//
//  The optional *pdwInnerContentType is updated with the type of the inner
//  message. Unless there is cryptographic message nesting, CMSG_DATA
//  is returned.
//
//  For CMSG_DATA: returns decoded content.
//  For CMSG_SIGNED: same as CryptVerifyMessageSignature.
//  For CMSG_ENVELOPED: same as CryptDecryptMessage.
//  For CMSG_SIGNED_AND_ENVELOPED: same as CryptDecryptMessage plus
//      CryptVerifyMessageSignature.
//  For CMSG_HASHED: verifies the hash and returns decoded content.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptDecodeMessage(
    _In_ DWORD dwMsgTypeFlags,
    _In_opt_ PCRYPT_DECRYPT_MESSAGE_PARA pDecryptPara,
    _In_opt_ PCRYPT_VERIFY_MESSAGE_PARA pVerifyPara,
    _In_ DWORD dwSignerIndex,
    _In_reads_bytes_(cbEncodedBlob) const BYTE *pbEncodedBlob,
    _In_ DWORD cbEncodedBlob,
    _In_ DWORD dwPrevInnerContentType,
    _Out_opt_ DWORD *pdwMsgType,
    _Out_opt_ DWORD *pdwInnerContentType,
    _Out_writes_bytes_to_opt_(*pcbDecoded, *pcbDecoded) BYTE *pbDecoded,
    _Inout_opt_ DWORD *pcbDecoded,
    _Outptr_opt_result_maybenull_ PCCERT_CONTEXT *ppXchgCert,
    _Outptr_opt_result_maybenull_ PCCERT_CONTEXT *ppSignerCert
    );

//+-------------------------------------------------------------------------
//  Hash the message.
//
//  If fDetachedHash is TRUE, only the ComputedHash is encoded in the
//  pbHashedBlob. Otherwise, both the ToBeHashed and ComputedHash
//  are encoded.
//
//  pcbHashedBlob or pcbComputedHash can be NULL, indicating the caller
//  isn't interested in getting the output.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptHashMessage(
    _In_ PCRYPT_HASH_MESSAGE_PARA pHashPara,
    _In_ BOOL fDetachedHash,
    _In_ DWORD cToBeHashed,
    _In_reads_(cToBeHashed) const BYTE *rgpbToBeHashed[],
    _In_reads_(cToBeHashed) DWORD rgcbToBeHashed[],
    _Out_writes_bytes_to_opt_(*pcbHashedBlob, *pcbHashedBlob) BYTE *pbHashedBlob,
    _Inout_opt_ DWORD *pcbHashedBlob,
    _Out_writes_bytes_to_opt_(*pcbComputedHash, *pcbComputedHash) BYTE *pbComputedHash,
    _Inout_opt_ DWORD *pcbComputedHash
    );

//+-------------------------------------------------------------------------
//  Verify a hashed message.
//
//  pcbToBeHashed or pcbComputedHash can be NULL,
//  indicating the caller isn't interested in getting the output.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptVerifyMessageHash(
    _In_ PCRYPT_HASH_MESSAGE_PARA pHashPara,
    _In_reads_bytes_(cbHashedBlob) BYTE *pbHashedBlob,
    _In_ DWORD cbHashedBlob,
    _Out_writes_bytes_to_opt_(*pcbToBeHashed, *pcbToBeHashed) BYTE *pbToBeHashed,
    _Inout_opt_ DWORD *pcbToBeHashed,
    _Out_writes_bytes_to_opt_(*pcbComputedHash, *pcbComputedHash) BYTE *pbComputedHash,
    _Inout_opt_ DWORD *pcbComputedHash
    );

//+-------------------------------------------------------------------------
//  Verify a hashed message containing a detached hash.
//  The "to be hashed" content is passed in separately. No
//  decoded output. Otherwise, identical to CryptVerifyMessageHash.
//
//  pcbComputedHash can be NULL, indicating the caller isn't interested
//  in getting the output.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptVerifyDetachedMessageHash(
    _In_ PCRYPT_HASH_MESSAGE_PARA pHashPara,
    _In_reads_bytes_(cbDetachedHashBlob) BYTE *pbDetachedHashBlob,
    _In_ DWORD cbDetachedHashBlob,
    _In_ DWORD cToBeHashed,
    _In_reads_(cToBeHashed) const BYTE *rgpbToBeHashed[],
    _In_reads_(cToBeHashed) DWORD rgcbToBeHashed[],
    _Out_writes_bytes_to_opt_(*pcbComputedHash, *pcbComputedHash) BYTE *pbComputedHash,
    _Inout_opt_ DWORD *pcbComputedHash
    );

//+-------------------------------------------------------------------------
//  Sign the message using the provider's private key specified in the
//  parameters. A dummy SignerId is created and stored in the message.
//
//  Normally used until a certificate has been created for the key.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptSignMessageWithKey(
    _In_ PCRYPT_KEY_SIGN_MESSAGE_PARA pSignPara,
    _In_reads_bytes_(cbToBeSigned) const BYTE *pbToBeSigned,
    _In_ DWORD cbToBeSigned,
    _Out_writes_bytes_to_opt_(*pcbSignedBlob, *pcbSignedBlob) BYTE *pbSignedBlob,
    _Inout_ DWORD *pcbSignedBlob
    );

//+-------------------------------------------------------------------------
//  Verify a signed message using the specified public key info.
//
//  Normally called by a CA until it has created a certificate for the
//  key.
//
//  pPublicKeyInfo contains the public key to use to verify the signed
//  message. If NULL, the signature isn't verified (for instance, the decoded
//  content may contain the PublicKeyInfo).
//
//  pcbDecoded can be NULL, indicating the caller isn't interested
//  in getting the decoded content.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptVerifyMessageSignatureWithKey(
    _In_ PCRYPT_KEY_VERIFY_MESSAGE_PARA pVerifyPara,
    _In_opt_ PCERT_PUBLIC_KEY_INFO pPublicKeyInfo,
    _In_reads_bytes_(cbSignedBlob) const BYTE *pbSignedBlob,
    _In_ DWORD cbSignedBlob,
    _Out_writes_bytes_to_opt_(*pcbDecoded, *pcbDecoded) BYTE *pbDecoded,
    _Inout_opt_ DWORD *pcbDecoded
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or Wintrust Package or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+=========================================================================
//  System Certificate Store Data Structures and APIs
//==========================================================================


//+-------------------------------------------------------------------------
//  Get a system certificate store based on a subsystem protocol.
//
//  Current examples of subsystems protocols are:
//      "MY"    Cert Store hold certs with associated Private Keys
//      "CA"    Certifying Authority certs
//      "ROOT"  Root Certs
//      "SPC"   Software publisher certs
//
//
//  If hProv is NULL the default provider "1" is opened for you.
//  When the store is closed the provider is release. Otherwise
//  if hProv is not NULL, no provider is created or released.
//
//  The returned Cert Store can be searched for an appropriate Cert
//  using the Cert Store API's (see certstor.h)
//
//  When done, the cert store should be closed using CertStoreClose
//--------------------------------------------------------------------------


WINCRYPT32API
HCERTSTORE
WINAPI
CertOpenSystemStoreA(
    _In_opt_ HCRYPTPROV_LEGACY      hProv,
    _In_ LPCSTR            szSubsystemProtocol
    );
WINCRYPT32API
HCERTSTORE
WINAPI
CertOpenSystemStoreW(
    _In_opt_ HCRYPTPROV_LEGACY      hProv,
    _In_ LPCWSTR            szSubsystemProtocol
    );
#ifdef UNICODE
#define CertOpenSystemStore  CertOpenSystemStoreW
#else
#define CertOpenSystemStore  CertOpenSystemStoreA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINCRYPT32API
BOOL
WINAPI
CertAddEncodedCertificateToSystemStoreA(
    _In_ LPCSTR            szCertStoreName,
    _In_reads_bytes_(cbCertEncoded) const BYTE *    pbCertEncoded,
    _In_ DWORD           cbCertEncoded
    );
WINCRYPT32API
BOOL
WINAPI
CertAddEncodedCertificateToSystemStoreW(
    _In_ LPCWSTR            szCertStoreName,
    _In_reads_bytes_(cbCertEncoded) const BYTE *    pbCertEncoded,
    _In_ DWORD           cbCertEncoded
    );
#ifdef UNICODE
#define CertAddEncodedCertificateToSystemStore  CertAddEncodedCertificateToSystemStoreW
#else
#define CertAddEncodedCertificateToSystemStore  CertAddEncodedCertificateToSystemStoreA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or Wintrust Package or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP |WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_PKG_WINTRUST | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Find all certificate chains tying the given issuer name to any certificate
//  that the current user has a private key for.
//
//  If no certificate chain is found, FALSE is returned with LastError set
//  to CRYPT_E_NOT_FOUND and the counts zeroed.
//
//  IE 3.0 ASSUMPTION:
//   The client certificates are in the "My" system store. The issuer
//   cerificates may be in the "Root", "CA" or "My" system stores.
//--------------------------------------------------------------------------
typedef struct _CERT_CHAIN {
    DWORD                   cCerts;     // number of certs in chain
    PCERT_BLOB              certs;      // pointer to array of cert chain blobs
                                        // representing the certs
    CRYPT_KEY_PROV_INFO     keyLocatorInfo; // key locator for cert
} CERT_CHAIN, *PCERT_CHAIN;


// WINCRYPT32API    This is not exported by crypt32, it is exported by softpub
HRESULT
WINAPI
FindCertsByIssuer(
    _Out_writes_bytes_to_opt_(*pcbCertChains, *pcbCertChains) PCERT_CHAIN pCertChains,
    _Inout_ DWORD *pcbCertChains,
    _Out_ DWORD *pcCertChains,        // count of certificates chains returned
    _In_reads_bytes_opt_(cbEncodedIssuerName) BYTE* pbEncodedIssuerName,   // DER encoded issuer name
    _In_ DWORD cbEncodedIssuerName,   // count in bytes of encoded issuer name
    _In_opt_ LPCWSTR pwszPurpose,     // "ClientAuth" or "CodeSigning"
    _In_ DWORD dwKeySpec              // only return signers supporting this
                                      // keyspec
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP |WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_PKG_WINTRUST | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//-------------------------------------------------------------------------
//
//  CryptQueryObject takes a CERT_BLOB or a file name and returns the
//  information about the content in the blob or in the file.
//
//  Parameters:
//  INPUT   dwObjectType:
//                       Indicate the type of the object.  Should be one of the
//                       following:
//                          CERT_QUERY_OBJECT_FILE
//                          CERT_QUERY_OBJECT_BLOB
//
//  INPUT   pvObject:
//                        If dwObjectType == CERT_QUERY_OBJECT_FILE, it is a
//                        LPWSTR, that is, the pointer to a wchar file name
//                        if dwObjectType == CERT_QUERY_OBJECT_BLOB, it is a
//                        PCERT_BLOB, that is, a pointer to a CERT_BLOB
//
//  INPUT   dwExpectedContentTypeFlags:
//                        Indicate the expected contenet type.
//                        Can be one of the following:
//                              CERT_QUERY_CONTENT_FLAG_ALL  (the content can be any type)
//                              CERT_QUERY_CONTENT_FLAG_CERT
//                              CERT_QUERY_CONTENT_FLAG_CTL
//                              CERT_QUERY_CONTENT_FLAG_CRL
//                              CERT_QUERY_CONTENT_FLAG_SERIALIZED_STORE
//                              CERT_QUERY_CONTENT_FLAG_SERIALIZED_CERT
//                              CERT_QUERY_CONTENT_FLAG_SERIALIZED_CTL
//                              CERT_QUERY_CONTENT_FLAG_SERIALIZED_CRL
//                              CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED
//                              CERT_QUERY_CONTENT_FLAG_PKCS7_UNSIGNED
//                              CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED_EMBED
//                              CERT_QUERY_CONTENT_FLAG_PKCS10
//                              CERT_QUERY_CONTENT_FLAG_PFX
//                              CERT_QUERY_CONTENT_FLAG_CERT_PAIR
//                              CERT_QUERY_CONTENT_FLAG_PFX_AND_LOAD
//
//  INPUT   dwExpectedFormatTypeFlags:
//                        Indicate the expected format type.
//                        Can be one of the following:
//                              CERT_QUERY_FORMAT_FLAG_ALL (the content can be any format)
//                              CERT_QUERY_FORMAT_FLAG_BINARY
//                              CERT_QUERY_FORMAT_FLAG_BASE64_ENCODED
//                              CERT_QUERY_FORMAT_FLAG_ASN_ASCII_HEX_ENCODED
//
//
//  INPUT   dwFlags
//                        Reserved flag.  Should always set to 0
//
//  OUTPUT  pdwMsgAndCertEncodingType
//                        Optional output.  If NULL != pdwMsgAndCertEncodingType,
//                        it contains the encoding type of the content as any
//                        combination of the following:
//                              X509_ASN_ENCODING
//                              PKCS_7_ASN_ENCODING
//
//  OUTPUT  pdwContentType
//                        Optional output.  If NULL!=pdwContentType, it contains
//                        the content type as one of the the following:
//                              CERT_QUERY_CONTENT_CERT
//                              CERT_QUERY_CONTENT_CTL
//                              CERT_QUERY_CONTENT_CRL
//                              CERT_QUERY_CONTENT_SERIALIZED_STORE
//                              CERT_QUERY_CONTENT_SERIALIZED_CERT
//                              CERT_QUERY_CONTENT_SERIALIZED_CTL
//                              CERT_QUERY_CONTENT_SERIALIZED_CRL
//                              CERT_QUERY_CONTENT_PKCS7_SIGNED
//                              CERT_QUERY_CONTENT_PKCS7_UNSIGNED
//                              CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED
//                              CERT_QUERY_CONTENT_PKCS10
//                              CERT_QUERY_CONTENT_PFX
//                              CERT_QUERY_CONTENT_CERT_PAIR
//                              CERT_QUERY_CONTENT_PFX_AND_LOAD
//
//  OUTPUT  pdwFormatType
//                        Optional output.  If NULL !=pdwFormatType, it
//                        contains the format type of the content as one of the
//                        following:
//                              CERT_QUERY_FORMAT_BINARY
//                              CERT_QUERY_FORMAT_BASE64_ENCODED
//                              CERT_QUERY_FORMAT_ASN_ASCII_HEX_ENCODED
//
//
//  OUTPUT  phCertStore
//                        Optional output.  If NULL !=phStore,
//                        it contains a cert store that includes all of certificates,
//                        CRL, and CTL in the object if the object content type is
//                        one of the following:
//                              CERT_QUERY_CONTENT_CERT
//                              CERT_QUERY_CONTENT_CTL
//                              CERT_QUERY_CONTENT_CRL
//                              CERT_QUERY_CONTENT_SERIALIZED_STORE
//                              CERT_QUERY_CONTENT_SERIALIZED_CERT
//                              CERT_QUERY_CONTENT_SERIALIZED_CTL
//                              CERT_QUERY_CONTENT_SERIALIZED_CRL
//                              CERT_QUERY_CONTENT_PKCS7_SIGNED
//                              CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED
//                              CERT_QUERY_CONTENT_CERT_PAIR
//
//                       Caller should free *phCertStore via CertCloseStore.
//
//
//  OUTPUT  phMsg        Optional output.  If NULL != phMsg,
//                        it contains a handle to a opened message if
//                        the content type is one of the following:
//                              CERT_QUERY_CONTENT_PKCS7_SIGNED
//                              CERT_QUERY_CONTENT_PKCS7_UNSIGNED
//                              CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED
//
//                       Caller should free *phMsg via CryptMsgClose.
//
//  OUTPUT pContext     Optional output.  If NULL != pContext,
//                      it contains either a PCCERT_CONTEXT or PCCRL_CONTEXT,
//                      or PCCTL_CONTEXT based on the content type.
//
//                      If the content type is CERT_QUERY_CONTENT_CERT or
//                      CERT_QUERY_CONTENT_SERIALIZED_CERT, it is a PCCERT_CONTEXT;
//                      Caller should free the pContext via CertFreeCertificateContext.
//
//                      If the content type is CERT_QUERY_CONTENT_CRL or
//                      CERT_QUERY_CONTENT_SERIALIZED_CRL, it is a PCCRL_CONTEXT;
//                      Caller should free the pContext via CertFreeCRLContext.
//
//                      If the content type is CERT_QUERY_CONTENT_CTL or
//                      CERT_QUERY_CONTENT_SERIALIZED_CTL, it is a PCCTL_CONTEXT;
//                      Caller should free the pContext via CertFreeCTLContext.
//
//  If the *pbObject is of type CERT_QUERY_CONTENT_PKCS10 or CERT_QUERY_CONTENT_PFX, CryptQueryObject
//  will not return anything in *phCertstore, *phMsg, or *ppvContext.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptQueryObject(
    _In_ DWORD                    dwObjectType,
    _In_ const void               *pvObject,
    _In_ DWORD                    dwExpectedContentTypeFlags,
    _In_ DWORD                    dwExpectedFormatTypeFlags,
    _In_ DWORD                    dwFlags,
    _Out_opt_ DWORD               *pdwMsgAndCertEncodingType,
    _Out_opt_ DWORD               *pdwContentType,
    _Out_opt_ DWORD               *pdwFormatType,
    _Out_opt_ HCERTSTORE          *phCertStore,
    _Out_opt_ HCRYPTMSG           *phMsg,
    _Outptr_opt_result_maybenull_ const void **ppvContext
    );


//-------------------------------------------------------------------------
//dwObjectType for CryptQueryObject
//-------------------------------------------------------------------------
#define     CERT_QUERY_OBJECT_FILE         0x00000001
#define     CERT_QUERY_OBJECT_BLOB         0x00000002

//-------------------------------------------------------------------------
//dwContentType for CryptQueryObject
//-------------------------------------------------------------------------
//encoded single certificate
#define     CERT_QUERY_CONTENT_CERT                 1
//encoded single CTL
#define     CERT_QUERY_CONTENT_CTL                  2
//encoded single CRL
#define     CERT_QUERY_CONTENT_CRL                  3
//serialized store
#define     CERT_QUERY_CONTENT_SERIALIZED_STORE     4
//serialized single certificate
#define     CERT_QUERY_CONTENT_SERIALIZED_CERT      5
//serialized single CTL
#define     CERT_QUERY_CONTENT_SERIALIZED_CTL       6
//serialized single CRL
#define     CERT_QUERY_CONTENT_SERIALIZED_CRL       7
//a PKCS#7 signed message
#define     CERT_QUERY_CONTENT_PKCS7_SIGNED         8
//a PKCS#7 message, such as enveloped message.  But it is not a signed message,
#define     CERT_QUERY_CONTENT_PKCS7_UNSIGNED       9
//a PKCS7 signed message embedded in a file
#define     CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED   10
//an encoded PKCS#10
#define     CERT_QUERY_CONTENT_PKCS10               11
//an encoded PFX BLOB
#define     CERT_QUERY_CONTENT_PFX                  12
//an encoded CertificatePair (contains forward and/or reverse cross certs)
#define     CERT_QUERY_CONTENT_CERT_PAIR            13
//an encoded PFX BLOB, which was loaded to phCertStore
#define     CERT_QUERY_CONTENT_PFX_AND_LOAD         14


//-------------------------------------------------------------------------
//dwExpectedConentTypeFlags for CryptQueryObject
//-------------------------------------------------------------------------

//encoded single certificate
#define     CERT_QUERY_CONTENT_FLAG_CERT   \
                ( 1 << CERT_QUERY_CONTENT_CERT)

//encoded single CTL
#define     CERT_QUERY_CONTENT_FLAG_CTL   \
                ( 1 << CERT_QUERY_CONTENT_CTL)

//encoded single CRL
#define     CERT_QUERY_CONTENT_FLAG_CRL   \
                ( 1 << CERT_QUERY_CONTENT_CRL)

//serialized store
#define     CERT_QUERY_CONTENT_FLAG_SERIALIZED_STORE   \
                ( 1 << CERT_QUERY_CONTENT_SERIALIZED_STORE)

//serialized single certificate
#define     CERT_QUERY_CONTENT_FLAG_SERIALIZED_CERT   \
                ( 1 << CERT_QUERY_CONTENT_SERIALIZED_CERT)

//serialized single CTL
#define     CERT_QUERY_CONTENT_FLAG_SERIALIZED_CTL   \
                ( 1 << CERT_QUERY_CONTENT_SERIALIZED_CTL)

//serialized single CRL
#define     CERT_QUERY_CONTENT_FLAG_SERIALIZED_CRL   \
                ( 1 << CERT_QUERY_CONTENT_SERIALIZED_CRL)

//an encoded PKCS#7 signed message
#define     CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED   \
                ( 1 << CERT_QUERY_CONTENT_PKCS7_SIGNED)

//an encoded PKCS#7 message.  But it is not a signed message
#define     CERT_QUERY_CONTENT_FLAG_PKCS7_UNSIGNED   \
                ( 1 << CERT_QUERY_CONTENT_PKCS7_UNSIGNED)

//the content includes an embedded PKCS7 signed message
#define     CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED_EMBED  \
                ( 1 << CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED)

//an encoded PKCS#10
#define     CERT_QUERY_CONTENT_FLAG_PKCS10   \
                ( 1 << CERT_QUERY_CONTENT_PKCS10)

//an encoded PFX BLOB
#define     CERT_QUERY_CONTENT_FLAG_PFX      \
                ( 1 << CERT_QUERY_CONTENT_PFX)

//an encoded CertificatePair (contains forward and/or reverse cross certs)
#define     CERT_QUERY_CONTENT_FLAG_CERT_PAIR   \
                ( 1 << CERT_QUERY_CONTENT_CERT_PAIR)

//an encoded PFX BLOB, and we do want to load it (not included in
//CERT_QUERY_CONTENT_FLAG_ALL)
#define     CERT_QUERY_CONTENT_FLAG_PFX_AND_LOAD    \
                ( 1 << CERT_QUERY_CONTENT_PFX_AND_LOAD)

//content can be any type
#define     CERT_QUERY_CONTENT_FLAG_ALL                         \
              ( CERT_QUERY_CONTENT_FLAG_CERT |                  \
                CERT_QUERY_CONTENT_FLAG_CTL  |                  \
                CERT_QUERY_CONTENT_FLAG_CRL  |                  \
                CERT_QUERY_CONTENT_FLAG_SERIALIZED_STORE |      \
                CERT_QUERY_CONTENT_FLAG_SERIALIZED_CERT  |      \
                CERT_QUERY_CONTENT_FLAG_SERIALIZED_CTL   |      \
                CERT_QUERY_CONTENT_FLAG_SERIALIZED_CRL   |      \
                CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED     |      \
                CERT_QUERY_CONTENT_FLAG_PKCS7_UNSIGNED   |      \
                CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED_EMBED |    \
                CERT_QUERY_CONTENT_FLAG_PKCS10                     |    \
                CERT_QUERY_CONTENT_FLAG_PFX                |    \
                CERT_QUERY_CONTENT_FLAG_CERT_PAIR )

//content types allowed for Issuer certificates
#define     CERT_QUERY_CONTENT_FLAG_ALL_ISSUER_CERT             \
              ( CERT_QUERY_CONTENT_FLAG_CERT             |      \
                CERT_QUERY_CONTENT_FLAG_SERIALIZED_STORE |      \
                CERT_QUERY_CONTENT_FLAG_SERIALIZED_CERT  |      \
                CERT_QUERY_CONTENT_FLAG_PKCS7_SIGNED     |      \
                CERT_QUERY_CONTENT_FLAG_PKCS7_UNSIGNED   )


//-------------------------------------------------------------------------
//dwFormatType for CryptQueryObject
//-------------------------------------------------------------------------
//the content is in binary format
#define     CERT_QUERY_FORMAT_BINARY                1

//the content is base64 encoded
#define     CERT_QUERY_FORMAT_BASE64_ENCODED        2

//the content is ascii hex encoded with "{ASN}" prefix
#define     CERT_QUERY_FORMAT_ASN_ASCII_HEX_ENCODED 3
//-------------------------------------------------------------------------
//dwExpectedFormatTypeFlags for CryptQueryObject
//-------------------------------------------------------------------------
//the content is in binary format
#define     CERT_QUERY_FORMAT_FLAG_BINARY         \
                ( 1 << CERT_QUERY_FORMAT_BINARY)

//the content is base64 encoded
#define     CERT_QUERY_FORMAT_FLAG_BASE64_ENCODED \
                ( 1 << CERT_QUERY_FORMAT_BASE64_ENCODED)

//the content is ascii hex encoded with "{ASN}" prefix
#define     CERT_QUERY_FORMAT_FLAG_ASN_ASCII_HEX_ENCODED \
                ( 1 << CERT_QUERY_FORMAT_ASN_ASCII_HEX_ENCODED)

//the content can be of any format
#define     CERT_QUERY_FORMAT_FLAG_ALL              \
          ( CERT_QUERY_FORMAT_FLAG_BINARY   |       \
            CERT_QUERY_FORMAT_FLAG_BASE64_ENCODED | \
            CERT_QUERY_FORMAT_FLAG_ASN_ASCII_HEX_ENCODED )

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// Crypt32 Memory Management Routines.  All Crypt32 API which return allocated
// buffers will do so via CryptMemAlloc, CryptMemRealloc.  Clients can free
// those buffers using CryptMemFree.  Also included is CryptMemSize
//

WINCRYPT32API
LPVOID
WINAPI
CryptMemAlloc (
    _In_ ULONG cbSize
    );

WINCRYPT32API
LPVOID
WINAPI
CryptMemRealloc (
    _In_opt_ LPVOID pv,
    _In_ ULONG cbSize
    );

WINCRYPT32API
VOID
WINAPI
CryptMemFree (
    _In_opt_ LPVOID pv
    );

//
// Crypt32 Asynchronous Parameter Management Routines.  All Crypt32 API which
// expose asynchronous mode operation use a Crypt32 Async Handle to pass
// around information about the operation e.g. callback routines.  The
// following API are used for manipulation of the async handle
//

// Following functions were never used. If called, will fail with LastError
// set to ERROR_CALL_NOT_IMPLEMENTED.

typedef HANDLE HCRYPTASYNC, *PHCRYPTASYNC;

typedef VOID (WINAPI *PFN_CRYPT_ASYNC_PARAM_FREE_FUNC) (
    _In_ LPSTR pszParamOid,
    _In_ LPVOID pvParam
    );

WINCRYPT32API
BOOL
WINAPI
CryptCreateAsyncHandle (
    _In_ DWORD dwFlags,
    _Out_ PHCRYPTASYNC phAsync
    );

WINCRYPT32API
BOOL
WINAPI
CryptSetAsyncParam (
    _In_ HCRYPTASYNC hAsync,
    _In_ LPSTR pszParamOid,
    _In_opt_ LPVOID pvParam,
    __callback PFN_CRYPT_ASYNC_PARAM_FREE_FUNC pfnFree
    );

WINCRYPT32API
BOOL
WINAPI
CryptGetAsyncParam (
    _In_ HCRYPTASYNC hAsync,
    _In_ LPSTR pszParamOid,
    _Outptr_opt_result_maybenull_ LPVOID* ppvParam,
    _Outptr_opt_result_maybenull_ __callback  PFN_CRYPT_ASYNC_PARAM_FREE_FUNC* ppfnFree
    );

WINCRYPT32API
BOOL
WINAPI
CryptCloseAsyncHandle (
    _In_opt_ HCRYPTASYNC hAsync
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// Crypt32 Remote Object Retrieval Routines.  This API allows retrieval of
// remote PKI objects where the location is given by an URL.  The remote
// object retrieval manager exposes two provider models.  One is the "Scheme
// Provider" model which allows for installable protocol providers as defined
// by the URL scheme e.g. ldap, http, ftp.  The scheme provider entry point is
// the same as the CryptRetrieveObjectByUrl however the *ppvObject returned
// is ALWAYS a counted array of encoded bits (one per object retrieved).  The
// second provider model is the "Context Provider" model which allows for
// installable creators of CAPI2 context handles (objects) based on the
// retrieved encoded bits.  These are dispatched based on the object OID given
// in the call to CryptRetrieveObjectByUrl.
//

typedef struct _CRYPT_BLOB_ARRAY {
    DWORD            cBlob;
    PCRYPT_DATA_BLOB rgBlob;
} CRYPT_BLOB_ARRAY, *PCRYPT_BLOB_ARRAY;

typedef struct _CRYPT_CREDENTIALS {
    DWORD  cbSize;
    LPCSTR pszCredentialsOid;
    LPVOID pvCredentials;
} CRYPT_CREDENTIALS, *PCRYPT_CREDENTIALS;

#define CREDENTIAL_OID_PASSWORD_CREDENTIALS_A ((LPCSTR)1)
#define CREDENTIAL_OID_PASSWORD_CREDENTIALS_W ((LPCSTR)2)

#ifdef UNICODE
#define CREDENTIAL_OID_PASSWORD_CREDENTIALS CREDENTIAL_OID_PASSWORD_CREDENTIALS_W
#else
#define CREDENTIAL_OID_PASSWORD_CREDENTIALS CREDENTIAL_OID_PASSWORD_CREDENTIALS_A
#endif //UNICODE

typedef struct _CRYPT_PASSWORD_CREDENTIALSA {
    DWORD   cbSize;
    LPSTR   pszUsername;
    LPSTR   pszPassword;
} CRYPT_PASSWORD_CREDENTIALSA, *PCRYPT_PASSWORD_CREDENTIALSA;
typedef struct _CRYPT_PASSWORD_CREDENTIALSW {
    DWORD   cbSize;
    LPWSTR  pszUsername;
    LPWSTR  pszPassword;
} CRYPT_PASSWORD_CREDENTIALSW, *PCRYPT_PASSWORD_CREDENTIALSW;
#ifdef UNICODE
typedef CRYPT_PASSWORD_CREDENTIALSW CRYPT_PASSWORD_CREDENTIALS;
typedef PCRYPT_PASSWORD_CREDENTIALSW PCRYPT_PASSWORD_CREDENTIALS;
#else
typedef CRYPT_PASSWORD_CREDENTIALSA CRYPT_PASSWORD_CREDENTIALS;
typedef PCRYPT_PASSWORD_CREDENTIALSA PCRYPT_PASSWORD_CREDENTIALS;
#endif // UNICODE

//
// Scheme Provider Signatures
//

// The following is obsolete and has been replaced with the following
// definition
#define SCHEME_OID_RETRIEVE_ENCODED_OBJECT_FUNC "SchemeDllRetrieveEncodedObject"

// 2-8-02 Server 2003 changed to use UNICODE Url strings instead of multibyte
#define SCHEME_OID_RETRIEVE_ENCODED_OBJECTW_FUNC "SchemeDllRetrieveEncodedObjectW"

typedef VOID (WINAPI *PFN_FREE_ENCODED_OBJECT_FUNC) (
    _In_opt_ LPCSTR pszObjectOid,
    _Inout_ PCRYPT_BLOB_ARRAY pObject,
    _Inout_opt_ LPVOID pvFreeContext
    );

//
// SchemeDllRetrieveEncodedObject was replaced in Server 2003 with
// the following. (Changed to use UNICODE Url Strings.)
//

//
// SchemeDllRetrieveEncodedObjectW has the following signature:
//
// _Success_(return != FALSE)
// BOOL WINAPI SchemeDllRetrieveEncodedObjectW (
//                   _In_ LPCWSTR pwszUrl,
//                   _In_opt_ LPCSTR pszObjectOid,
//                   _In_ DWORD dwRetrievalFlags,
//                   _In_ DWORD dwTimeout,                // milliseconds
//                   _Out_ PCRYPT_BLOB_ARRAY pObject,
//                   _Outptr_ __callback PFN_FREE_ENCODED_OBJECT_FUNC* ppfnFreeObject,
//                   _Outptr_result_maybenull_ LPVOID* ppvFreeContext,
//                   _In_opt_ HCRYPTASYNC hAsyncRetrieve,
//                   _In_opt_ PCRYPT_CREDENTIALS pCredentials,
//                   _Inout_opt_ PCRYPT_RETRIEVE_AUX_INFO pAuxInfo
//                   )
//

//
// Context Provider Signatures
//

#define CONTEXT_OID_CREATE_OBJECT_CONTEXT_FUNC "ContextDllCreateObjectContext"

#define CONTEXT_OID_CERTIFICATE ((LPCSTR)1)
#define CONTEXT_OID_CRL         ((LPCSTR)2)
#define CONTEXT_OID_CTL         ((LPCSTR)3)
#define CONTEXT_OID_PKCS7       ((LPCSTR)4)
#define CONTEXT_OID_CAPI2_ANY   ((LPCSTR)5)
#define CONTEXT_OID_OCSP_RESP   ((LPCSTR)6)

//
// ContextDllCreateObjectContext has the following signature:
//
// _Success_(return != FALSE)
// BOOL WINAPI ContextDllCreateObjectContext (
//                    _In_opt_ LPCSTR pszObjectOid,
//                    _In_ DWORD dwRetrievalFlags,
//                    _In_ PCRYPT_BLOB_ARRAY pObject,
//                    _Outptr_ LPVOID* ppvContext
//                    )
//

//
// Remote Object Retrieval API
//

//
// Retrieval flags
//

#define CRYPT_RETRIEVE_MULTIPLE_OBJECTS         0x00000001
#define CRYPT_CACHE_ONLY_RETRIEVAL              0x00000002
#define CRYPT_WIRE_ONLY_RETRIEVAL               0x00000004
#define CRYPT_DONT_CACHE_RESULT                 0x00000008
#define CRYPT_ASYNC_RETRIEVAL                   0x00000010
#define CRYPT_STICKY_CACHE_RETRIEVAL            0x00001000
#define CRYPT_LDAP_SCOPE_BASE_ONLY_RETRIEVAL    0x00002000
#define CRYPT_OFFLINE_CHECK_RETRIEVAL           0x00004000

// When the following flag is set, the following 2 NULL terminated ascii
// strings are inserted at the beginning of each returned blob:
//  "%d\0%s\0", dwEntryIndex, pszAttribute
//
//  The first dwEntryIndex is 0, "0\0".
//
// When set, pszObjectOid must be NULL, so that a PCRYPT_BLOB_ARRAY is returned.
#define CRYPT_LDAP_INSERT_ENTRY_ATTRIBUTE       0x00008000

// Set this flag to digitally sign all of the ldap traffic to and from a
// Windows 2000 LDAP server using the Kerberos authentication protocol.
// This feature provides integrity required by some applications.
#define CRYPT_LDAP_SIGN_RETRIEVAL               0x00010000

// Set this flag to inhibit automatic authentication handling. See the
// wininet flag, INTERNET_FLAG_NO_AUTH, for more details.
#define CRYPT_NO_AUTH_RETRIEVAL                 0x00020000

// Performs an A-Record only DNS lookup on the supplied host string.
// This prevents bogus DNS queries from being generated when resolving host
// names. Use this flag whenever passing a hostname as opposed to a
// domain name for the hostname parameter.
//
// See LDAP_OPT_AREC_EXCLUSIVE defined in winldap.h for more details.
#define CRYPT_LDAP_AREC_EXCLUSIVE_RETRIEVAL     0x00040000

// Apply AIA URL restrictions, such as, validate retrieved content before
// writing to cache.
#define CRYPT_AIA_RETRIEVAL                     0x00080000

// For HTTP: use POST instead of the default GET
//
// The POST additional binary data and header strings are appended to
// the host name and path URL as follows:
//  + L'/'<Optional url escaped and base64 encoded additional data>
//  + L'?'<Optional additional headers>
//
// Here's an example of an OCSP POST URL:
//  http://ocsp.openvalidation.org/MEIwQDA%2BMDwwOjAJBgUrDgMCGgUABBQdKNE
//      wjytjKBQADcgM61jfflNpyQQUv1NDgnjQnsOA5RtnygUA37lIg6UCA
//      QI%3D?Content-Type: application/ocsp-request
//
//
// When this flag is set, CryptRetrieveObjectByUrl, searches for the
// last L'/' and L'?' POST marker characters in the URL string.
// These are removed from the URL before it is passed to the WinHttp
// APIs. The L'?' string is passed as the AdditionHeaders to
// WinHttpSendRequest. The L'/' string is url unescaped (%xx converted
// to appropriate character) and base64 decoded into binary. This
// decoded binary is passed as the additional data to WinHttpSendRequest.
#define CRYPT_HTTP_POST_RETRIEVAL               0x00100000

// When this flag is set we won't attempt to bypass any potential proxy caches.
// If a proxy cache wasn't explicitly bypassed, fProxyCacheRetrieval will be
// set in pAuxInfo. Only applicable to http URL retrievals.
#define CRYPT_PROXY_CACHE_RETRIEVAL             0x00200000

// When this flag is set, for a conditional retrieval returning not modified,
// TRUE is returned and *ppvObject is set to NULL. For a nonNULL pAuxInfo,
// dwHttpStatusCode is set to winhttp.h's HTTP_STATUS_NOT_MODIFIED. Otherwise,
// *ppvObject is updated for a successful retrieval. Only applicable to
// http URL retrievals.
#define CRYPT_NOT_MODIFIED_RETRIEVAL            0x00400000

// When this flag is set, revocation checking is enabled for https URLs.
// If the server's certificate is revoked, then, LastError is set to
// CRYPT_E_REVOKED. For no other errors, LastError is set to
// CRYPT_E_REVOCATION_OFFLINE for any offline revocation error.
//
// To ignore offline revocation errors, this API can be called again without
// setting this flag.
#define CRYPT_ENABLE_SSL_REVOCATION_RETRIEVAL   0x00800000

// Set this flag to append a random query string to the URL passed to
// WinHttpOpenRequest. This should only be set on URL's accessing Windows
// Update content. The random query string ensures that cached proxy content
// isn't used and the HTTP request will always reach the Content Delivery
// Network (CDN) used by Windows Update which removes a query string
// before doing a cache lookup.
#define CRYPT_RANDOM_QUERY_STRING_RETRIEVAL     0x04000000

// File scheme retrieval's are disabled by default. This flag can be set to
// allow file retrievals.
#define CRYPT_ENABLE_FILE_RETRIEVAL             0x08000000

// Set this flag to check if a cache flush entry already exists for this URL.
// If it already exists, this API will fail and set LastError to
// ERROR_FILE_EXISTS. Otherwise, the pvVerify parameter will be used.
// If NULL, we only check if the cache entry exists. If nonNULL, then,
// pvVerify should be a PCRYPTNET_URL_CACHE_FLUSH_INFO containing the
// flush information to be written.
#define CRYPT_CREATE_NEW_FLUSH_ENTRY            0x10000000

//
// Data verification retrieval flags
//
// CRYPT_VERIFY_CONTEXT_SIGNATURE is used to get signature verification
// on the context created.  In this case pszObjectOid must be non-NULL and
// pvVerify points to the signer certificate context
//
// CRYPT_VERIFY_DATA_HASH is used to get verification of the blob data
// retrieved by the protocol.  The pvVerify points to an URL_DATA_HASH
// structure (TBD)
//

#define CRYPT_VERIFY_CONTEXT_SIGNATURE          0x00000020
#define CRYPT_VERIFY_DATA_HASH                  0x00000040

//
// Time Valid Object flags
//

#define CRYPT_KEEP_TIME_VALID                   0x00000080
#define CRYPT_DONT_VERIFY_SIGNATURE             0x00000100
#define CRYPT_DONT_CHECK_TIME_VALIDITY          0x00000200

// The default checks if ftNextUpdate >= ftValidFor. Set this flag to
// check if ftThisUpdate >= ftValidFor.
#define CRYPT_CHECK_FRESHNESS_TIME_VALIDITY     0x00000400

#define CRYPT_ACCUMULATIVE_TIMEOUT              0x00000800

// Set this flag to only use OCSP AIA URLs.
#define CRYPT_OCSP_ONLY_RETRIEVAL               0x01000000

// Set this flag to only use the OCSP AIA URL if present. If the subject
// doesn't have an OCSP AIA URL, then, the CDP URLs are used.
#define CRYPT_NO_OCSP_FAILOVER_TO_CRL_RETRIEVAL 0x02000000


//
// Cryptnet URL Cache Pre-Fetch Info
//
typedef struct _CRYPTNET_URL_CACHE_PRE_FETCH_INFO {
    DWORD           cbSize;
    DWORD           dwObjectType;

    // Possible errors:
    //  S_OK                - Pending
    //  ERROR_MEDIA_OFFLINE - CRL pre-fetch disabled due to OCSP offline.
    //  ERROR_FILE_OFFLINE  - Unchanged pre-fetch content
    //  ERROR_INVALID_DATA  - Invalid pre-fetch content
    //  Other errors        - Unable to retrieve pre-fetch content
    DWORD           dwError;
    DWORD           dwReserved;

    FILETIME        ThisUpdateTime;
    FILETIME        NextUpdateTime;
    FILETIME        PublishTime;    // May be zero
} CRYPTNET_URL_CACHE_PRE_FETCH_INFO, *PCRYPTNET_URL_CACHE_PRE_FETCH_INFO;

// Pre-fetch ObjectTypes
#define CRYPTNET_URL_CACHE_PRE_FETCH_NONE                   0
#define CRYPTNET_URL_CACHE_PRE_FETCH_BLOB                   1
#define CRYPTNET_URL_CACHE_PRE_FETCH_CRL                    2
#define CRYPTNET_URL_CACHE_PRE_FETCH_OCSP                   3
#define CRYPTNET_URL_CACHE_PRE_FETCH_AUTOROOT_CAB           5
#define CRYPTNET_URL_CACHE_PRE_FETCH_DISALLOWED_CERT_CAB    6
#define CRYPTNET_URL_CACHE_PRE_FETCH_PIN_RULES_CAB          7

//
// Cryptnet URL Cache Flush Info
//
typedef struct _CRYPTNET_URL_CACHE_FLUSH_INFO {
    DWORD           cbSize;
    // If pre-fetching is enabled, following is ignored
    //
    // 0          - use default flush exempt seconds (2 weeks)
    // 0xFFFFFFFF - disable flushing
    DWORD           dwExemptSeconds;

    // Time the object expires. The above dwExemptSeconds is added to
    // to determine the flush time. The LastSyncTime is used if
    // after this time.
    FILETIME        ExpireTime;
} CRYPTNET_URL_CACHE_FLUSH_INFO, *PCRYPTNET_URL_CACHE_FLUSH_INFO;

#define CRYPTNET_URL_CACHE_DEFAULT_FLUSH                0
#define CRYPTNET_URL_CACHE_DISABLE_FLUSH                0xFFFFFFFF


//
// Cryptnet URL Cache Response Info
//
typedef struct _CRYPTNET_URL_CACHE_RESPONSE_INFO {
    DWORD           cbSize;
    WORD            wResponseType;
    WORD            wResponseFlags;

    // The following are zero if not present
    FILETIME        LastModifiedTime;
    DWORD           dwMaxAge;
    LPCWSTR         pwszETag;
    DWORD           dwProxyId;
} CRYPTNET_URL_CACHE_RESPONSE_INFO, *PCRYPTNET_URL_CACHE_RESPONSE_INFO;


// ResponseTypes
#define CRYPTNET_URL_CACHE_RESPONSE_NONE            0
#define CRYPTNET_URL_CACHE_RESPONSE_HTTP            1

// ResponseFlags
#define CRYPTNET_URL_CACHE_RESPONSE_VALIDATED       0x8000

//
// CryptRetrieveObjectByUrl Auxilliary Info
//
//
// All unused fields in this data structure must be zeroed. More fields
// could be added in a future release.
//
typedef struct _CRYPT_RETRIEVE_AUX_INFO {
    DWORD                               cbSize;
    FILETIME                            *pLastSyncTime;

    // 0 => implies no limit
    DWORD                               dwMaxUrlRetrievalByteCount;

    // To get any PreFetchInfo, set the following pointer to a
    // CRYPTNET_URL_CACHE_PRE_FETCH_INFO structure with its cbSize set
    // upon input. For no PreFetchInfo, except for cbSize, the data
    // structure is zeroed upon return.
    PCRYPTNET_URL_CACHE_PRE_FETCH_INFO  pPreFetchInfo;

    // To get any FlushInfo, set the following pointer to a
    // CRYPTNET_URL_CACHE_FLUSH_INFO structure with its cbSize set
    // upon input. For no FlushInfo, except for cbSize, the data structure
    // is zeroed upon return.
    PCRYPTNET_URL_CACHE_FLUSH_INFO      pFlushInfo;

    // To get any ResponseInfo, set the following pointer to the address
    // of a PCRYPTNET_URL_CACHE_RESPONSE_INFO pointer updated with
    // the allocated structure. For no ResponseInfo, *ppResponseInfo is set
    // to NULL. Otherwise, *ppResponseInfo must be free via CryptMemFree().
    PCRYPTNET_URL_CACHE_RESPONSE_INFO   *ppResponseInfo;

    // If nonNULL, the specified prefix string is prepended to the
    // cached filename.
    LPWSTR                              pwszCacheFileNamePrefix;

    // If nonNULL, any cached information before this time is considered
    // time invalid. For CRYPT_CACHE_ONLY_RETRIEVAL, if there is a
    // cached entry before this time, LastError is set to ERROR_INVALID_TIME.
    // Also used to set max-age for http retrievals.
    LPFILETIME                          pftCacheResync;

    // The following flag is set upon return if CRYPT_PROXY_CACHE_RETRIEVAL
    // was set in dwRetrievalFlags and the proxy cache wasn't explicitly
    // bypassed for the retrieval. This flag won't be explicitly cleared.
    // This flag will only be set for http URL retrievals.
    BOOL                                fProxyCacheRetrieval;

    // This value is only updated upon return for a nonSuccessful status code
    // returned in a HTTP response header. This value won't be explicitly
    // cleared. This value will only be updated for http or https URL
    // retrievals.
    //
    // If CRYPT_NOT_MODIFIED_RETRIEVAL was set in dwFlags, set to winhttp.h's
    // HTTP_STATUS_NOT_MODIFIED if the retrieval returned not modified. In
    // this case TRUE is returned with *ppvObject set to NULL.
    DWORD                               dwHttpStatusCode;

    // To get the HTTP response headers for a retrieval error, set the following
    // pointer to the address of a LPWSTR to receive the list of
    // headers. L'|' is used as the separator between headers.
    // The *ppwszErrorResponseHeaders must be freed via CryptMemFree().
    LPWSTR                              *ppwszErrorResponseHeaders;

    // To get the content for a retrieval decode error, set the following
    // pointer to the address of a PCRYPT_DATA_BLOB.
    // The *ppErrorContentBlob must be freed via CryptMemFree().
    PCRYPT_DATA_BLOB                    *ppErrorContentBlob;
} CRYPT_RETRIEVE_AUX_INFO, *PCRYPT_RETRIEVE_AUX_INFO;

// Limit the error content to be allocated and returned.
#define CRYPT_RETRIEVE_MAX_ERROR_CONTENT_LENGTH 0x1000

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINCRYPT32API
_Success_(return == TRUE)
BOOL
WINAPI
CryptRetrieveObjectByUrlA (
    _In_ LPCSTR pszUrl,
    _In_opt_ LPCSTR pszObjectOid,
    _In_ DWORD dwRetrievalFlags,
    _In_ DWORD dwTimeout,                     // milliseconds
    _Outptr_ LPVOID* ppvObject,
    _In_opt_ HCRYPTASYNC hAsyncRetrieve,
    _In_opt_ PCRYPT_CREDENTIALS pCredentials,
    _In_opt_ LPVOID pvVerify,
    _Inout_opt_ PCRYPT_RETRIEVE_AUX_INFO pAuxInfo
    );
WINCRYPT32API
_Success_(return == TRUE)
BOOL
WINAPI
CryptRetrieveObjectByUrlW (
    _In_ LPCWSTR pszUrl,
    _In_opt_ LPCSTR pszObjectOid,
    _In_ DWORD dwRetrievalFlags,
    _In_ DWORD dwTimeout,                     // milliseconds
    _Outptr_ LPVOID* ppvObject,
    _In_opt_ HCRYPTASYNC hAsyncRetrieve,
    _In_opt_ PCRYPT_CREDENTIALS pCredentials,
    _In_opt_ LPVOID pvVerify,
    _Inout_opt_ PCRYPT_RETRIEVE_AUX_INFO pAuxInfo
    );
#ifdef UNICODE
#define CryptRetrieveObjectByUrl  CryptRetrieveObjectByUrlW
#else
#define CryptRetrieveObjectByUrl  CryptRetrieveObjectByUrlA
#endif // !UNICODE

//
// Call back function to cancel object retrieval
//
// The function can be installed on a per thread basis.
// If CryptInstallCancelRetrieval is called for multiple times, only the most recent
// installation will be kept.
//
// This is only effective for http, https, gopher, and ftp protocol.
// It is ignored by the rest of the protocols.


typedef BOOL (WINAPI *PFN_CRYPT_CANCEL_RETRIEVAL)(
    _In_ DWORD dwFlags,
    _Inout_opt_ void  *pvArg
    );


//
// PFN_CRYPT_CANCEL_RETRIEVAL
//
// This function should return FALSE when the object retrieval should be continued
// and return TRUE when the object retrieval should be cancelled.
//

WINCRYPT32API
BOOL
WINAPI
CryptInstallCancelRetrieval(
    __callback PFN_CRYPT_CANCEL_RETRIEVAL pfnCancel,
    _In_opt_ const void *pvArg,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved
);


WINCRYPT32API
BOOL
WINAPI
CryptUninstallCancelRetrieval(
        _In_ DWORD dwFlags,
        _Reserved_ void *pvReserved
        );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_GAMES)

WINCRYPT32API
BOOL
WINAPI
CryptCancelAsyncRetrieval (
    _In_opt_ HCRYPTASYNC hAsyncRetrieval
    );

//
// Remote Object Async Retrieval parameters
//

//
// A client that wants to be notified of asynchronous object retrieval
// completion sets this parameter on the async handle
//

#define CRYPT_PARAM_ASYNC_RETRIEVAL_COMPLETION ((LPCSTR)1)

typedef VOID (WINAPI *PFN_CRYPT_ASYNC_RETRIEVAL_COMPLETION_FUNC) (
    _Inout_opt_ LPVOID pvCompletion,
    _In_ DWORD dwCompletionCode,
    _In_ LPCSTR pszUrl,
    _In_opt_ LPSTR pszObjectOid,
    _In_ LPVOID pvObject
    );

typedef struct _CRYPT_ASYNC_RETRIEVAL_COMPLETION {
    __callback PFN_CRYPT_ASYNC_RETRIEVAL_COMPLETION_FUNC pfnCompletion;
    _Inout_opt_ LPVOID pvCompletion;
} CRYPT_ASYNC_RETRIEVAL_COMPLETION, *PCRYPT_ASYNC_RETRIEVAL_COMPLETION;

//
// This function is set on the async handle by a scheme provider that
// supports asynchronous retrieval
//

#define CRYPT_PARAM_CANCEL_ASYNC_RETRIEVAL ((LPCSTR)2)

typedef BOOL (WINAPI *PFN_CANCEL_ASYNC_RETRIEVAL_FUNC) (
    _In_opt_ HCRYPTASYNC hAsyncRetrieve
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
//
// Get the locator for a CAPI object
//

#define CRYPT_GET_URL_FROM_PROPERTY         0x00000001
#define CRYPT_GET_URL_FROM_EXTENSION        0x00000002
#define CRYPT_GET_URL_FROM_UNAUTH_ATTRIBUTE 0x00000004
#define CRYPT_GET_URL_FROM_AUTH_ATTRIBUTE   0x00000008

typedef struct _CRYPT_URL_ARRAY {
    DWORD   cUrl;
    LPWSTR* rgwszUrl;
} CRYPT_URL_ARRAY, *PCRYPT_URL_ARRAY;

typedef struct _CRYPT_URL_INFO {
    DWORD   cbSize;

    // Seconds between syncs
    DWORD   dwSyncDeltaTime;

    // Returned URLs may be grouped. For instance, groups of cross cert
    // distribution points. Each distribution point may have multiple
    // URLs, (LDAP and HTTP scheme).
    DWORD   cGroup;
    DWORD   *rgcGroupEntry;
} CRYPT_URL_INFO, *PCRYPT_URL_INFO;

WINCRYPT32API
BOOL
WINAPI
CryptGetObjectUrl (
    _In_ LPCSTR pszUrlOid,
    _In_ LPVOID pvPara,
    _In_ DWORD dwFlags,
    _Out_writes_bytes_to_opt_(*pcbUrlArray, *pcbUrlArray) PCRYPT_URL_ARRAY pUrlArray,
    _Inout_ DWORD* pcbUrlArray,
    _Out_writes_bytes_to_opt_(*pcbUrlInfo, *pcbUrlInfo) PCRYPT_URL_INFO pUrlInfo,
    _Inout_opt_ DWORD* pcbUrlInfo,
    _Reserved_ LPVOID pvReserved
    );

#define URL_OID_GET_OBJECT_URL_FUNC "UrlDllGetObjectUrl"

//
// UrlDllGetObjectUrl has the same signature as CryptGetObjectUrl
//

//
// URL_OID_CERTIFICATE_ISSUER
//
// pvPara == PCCERT_CONTEXT, certificate whose issuer's URL is being requested
//
// This will be retrieved from the authority info access extension or property
// on the certificate
//
// URL_OID_CERTIFICATE_CRL_DIST_POINT
//
// pvPara == PCCERT_CONTEXT, certificate whose CRL distribution point is being
// requested
//
// This will be retrieved from the CRL distribution point extension or property
// on the certificate
//
// URL_OID_CTL_ISSUER
//
// pvPara == PCCTL_CONTEXT, Signer Index, CTL whose issuer's URL (identified
// by the signer index) is being requested
//
// This will be retrieved from an authority info access attribute method encoded
// in each signer info in the PKCS7 (CTL)
//
// URL_OID_CTL_NEXT_UPDATE
//
// pvPara == PCCTL_CONTEXT, Signer Index, CTL whose next update URL is being
// requested and an optional signer index in case we need to check signer
// info attributes
//
// This will be retrieved from an authority info access CTL extension, property,
// or signer info attribute method
//
// URL_OID_CRL_ISSUER
//
// pvPara == PCCRL_CONTEXT, CRL whose issuer's URL is being requested
//
// This will be retrieved from a property on the CRL which has been inherited
// from the subject cert (either from the subject cert issuer or the subject
// cert distribution point extension).  It will be encoded as an authority
// info access extension method.
//
// URL_OID_CERTIFICATE_FRESHEST_CRL
//
// pvPara == PCCERT_CONTEXT, certificate whose freshest CRL distribution point
// is being requested
//
// This will be retrieved from the freshest CRL extension or property
// on the certificate
//
// URL_OID_CRL_FRESHEST_CRL
//
// pvPara == PCCERT_CRL_CONTEXT_PAIR, certificate's base CRL whose
// freshest CRL distribution point is being requested
//
// This will be retrieved from the freshest CRL extension or property
// on the CRL
//
// URL_OID_CROSS_CERT_DIST_POINT
//
// pvPara == PCCERT_CONTEXT, certificate whose cross certificate distribution
// point is being requested
//
// This will be retrieved from the cross certificate distribution point
// extension or property on the certificate
//
// URL_OID_CERTIFICATE_OCSP
//
// pvPara == PCCERT_CONTEXT, certificate whose OCSP URL is being requested
//
// This will be retrieved from the authority info access extension or property
// on the certificate
//
// URL_OID_CERTIFICATE_OCSP_AND_CRL_DIST_POINT
//
// pvPara == PCCERT_CONTEXT, certificate whose OCSP URL and
// CRL distribution point are being requested
//
// This will be retrieved from the authority info access and
// CRL distribution point extension or property on the certificate.
// If any OCSP URLs are present, they will be first with each URL prefixed
// with L"ocsp:". The L"ocsp:" prefix should be removed before using.
//
// URL_OID_CERTIFICATE_CRL_DIST_POINT_AND_OCSP
//
// Same as URL_OID_CERTIFICATE_OCSP_AND_CRL_DIST_POINT, except,
// the CRL URLs will be first
//
// URL_OID_CERTIFICATE_ONLY_OCSP
//
// Same as URL_OID_CERTIFICATE_OCSP_AND_CRL_DIST_POINT, except,
// only OCSP URLs are retrieved.
//
// URL_OID_CROSS_CERT_SUBJECT_INFO_ACCESS
//
// pvPara == PCCERT_CONTEXT, certificate whose cross certificates
// are being requested
//
// This will be retrieved from the Authority Info Access
// extension or property on the certificate. Only access methods
// matching szOID_PKIX_CA_REPOSITORY will be returned.

#define URL_OID_CERTIFICATE_ISSUER         ((LPCSTR)1)
#define URL_OID_CERTIFICATE_CRL_DIST_POINT ((LPCSTR)2)
#define URL_OID_CTL_ISSUER                 ((LPCSTR)3)
#define URL_OID_CTL_NEXT_UPDATE            ((LPCSTR)4)
#define URL_OID_CRL_ISSUER                 ((LPCSTR)5)
#define URL_OID_CERTIFICATE_FRESHEST_CRL   ((LPCSTR)6)
#define URL_OID_CRL_FRESHEST_CRL           ((LPCSTR)7)
#define URL_OID_CROSS_CERT_DIST_POINT      ((LPCSTR)8)
#define URL_OID_CERTIFICATE_OCSP           ((LPCSTR)9)
#define URL_OID_CERTIFICATE_OCSP_AND_CRL_DIST_POINT ((LPCSTR)10)
#define URL_OID_CERTIFICATE_CRL_DIST_POINT_AND_OCSP ((LPCSTR)11)
#define URL_OID_CROSS_CERT_SUBJECT_INFO_ACCESS ((LPCSTR)12)
#define URL_OID_CERTIFICATE_ONLY_OCSP      ((LPCSTR)13)

typedef struct _CERT_CRL_CONTEXT_PAIR {
    PCCERT_CONTEXT          pCertContext;
    PCCRL_CONTEXT           pCrlContext;
} CERT_CRL_CONTEXT_PAIR, *PCERT_CRL_CONTEXT_PAIR;
typedef const CERT_CRL_CONTEXT_PAIR *PCCERT_CRL_CONTEXT_PAIR;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_GAMES)

//
// Get a time valid CAPI2 object
//

//+-------------------------------------------------------------------------
//  The following optional Extra Info may be passed to
//  CryptGetTimeValidObject().
//
//  All unused fields in this data structure must be zeroed. More fields
//  could be added in a future release.
//--------------------------------------------------------------------------
typedef struct _CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO {
    DWORD                       cbSize;

    // If > 0, check that the CRL's number is >=
    // Should be 0x7fffffff if pDeltaCrlIndicator is nonNull
    int                         iDeltaCrlIndicator;

    // If nonNULL, any cached information before this time is considered
    // time invalid and forces a wire retrieval.
    LPFILETIME                  pftCacheResync;

    // If nonNull, returns the cache's LastSyncTime
    LPFILETIME                  pLastSyncTime;

    // If nonNull, returns the internal MaxAge expiration time
    // for the object. If the object doesn't have a MaxAge expiration, set
    // to zero.
    LPFILETIME                  pMaxAgeTime;

    // If nonNULL, CertGetCertificateChain() parameters used by the caller.
    // Enables independent OCSP signer certificate chain verification.
    PCERT_REVOCATION_CHAIN_PARA pChainPara;

    // Should be used if the DeltaCrlIndicator value is more than 4 bytes
    // If nonNull and iDeltaCrlIndicator == MAXLONG, check that the CRL's number is >=
    PCRYPT_INTEGER_BLOB pDeltaCrlIndicator;

} CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO,
    *PCRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO;

WINCRYPT32API
_Success_(return == TRUE)
BOOL
WINAPI
CryptGetTimeValidObject (
    _In_ LPCSTR pszTimeValidOid,
    _In_ LPVOID pvPara,
    _In_ PCCERT_CONTEXT pIssuer,
    _In_opt_ LPFILETIME pftValidFor,
    _In_ DWORD dwFlags,
    _In_ DWORD dwTimeout,                         // milliseconds
    _Outptr_opt_ LPVOID* ppvObject,
    _In_opt_ PCRYPT_CREDENTIALS pCredentials,
    _Inout_opt_ PCRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO pExtraInfo
    );

#define TIME_VALID_OID_GET_OBJECT_FUNC "TimeValidDllGetObject"

//
// TimeValidDllGetObject has the same signature as CryptGetTimeValidObject
//

//
// TIME_VALID_OID_GET_CTL
//
// pvPara == PCCTL_CONTEXT, the current CTL
//
// TIME_VALID_OID_GET_CRL
//
// pvPara == PCCRL_CONTEXT, the current CRL
//
// TIME_VALID_OID_GET_CRL_FROM_CERT
//
// pvPara == PCCERT_CONTEXT, the subject cert
//
// TIME_VALID_OID_GET_FRESHEST_CRL_FROM_CERT
//
// pvPara == PCCERT_CONTEXT, the subject cert
//
// TIME_VALID_OID_GET_FRESHEST_CRL_FROM_CRL
//
// pvPara == PCCERT_CRL_CONTEXT_PAIR, the subject cert and its base CRL
//

#define TIME_VALID_OID_GET_CTL           ((LPCSTR)1)
#define TIME_VALID_OID_GET_CRL           ((LPCSTR)2)
#define TIME_VALID_OID_GET_CRL_FROM_CERT ((LPCSTR)3)

#define TIME_VALID_OID_GET_FRESHEST_CRL_FROM_CERT   ((LPCSTR)4)
#define TIME_VALID_OID_GET_FRESHEST_CRL_FROM_CRL    ((LPCSTR)5)

WINCRYPT32API
BOOL
WINAPI
CryptFlushTimeValidObject (
     _In_ LPCSTR pszFlushTimeValidOid,
     _In_ LPVOID pvPara,
     _In_ PCCERT_CONTEXT pIssuer,
     _In_ DWORD dwFlags,
     _Reserved_ LPVOID pvReserved
     );

#define TIME_VALID_OID_FLUSH_OBJECT_FUNC "TimeValidDllFlushObject"

//
// TimeValidDllFlushObject has the same signature as CryptFlushTimeValidObject
//

//
// TIME_VALID_OID_FLUSH_CTL
//
// pvPara == PCCTL_CONTEXT, the CTL to flush
//
// TIME_VALID_OID_FLUSH_CRL
//
// pvPara == PCCRL_CONTEXT, the CRL to flush
//
// TIME_VALID_OID_FLUSH_CRL_FROM_CERT
//
// pvPara == PCCERT_CONTEXT, the subject cert's CRL to flush
//
// TIME_VALID_OID_FLUSH_FRESHEST_CRL_FROM_CERT
//
// pvPara == PCCERT_CONTEXT, the subject cert's freshest CRL to flush
//
// TIME_VALID_OID_FLUSH_FRESHEST_CRL_FROM_CRL
//
// pvPara == PCCERT_CRL_CONTEXT_PAIR, the subject cert and its base CRL's
// freshest CRL to flush
//

#define TIME_VALID_OID_FLUSH_CTL           ((LPCSTR)1)
#define TIME_VALID_OID_FLUSH_CRL           ((LPCSTR)2)
#define TIME_VALID_OID_FLUSH_CRL_FROM_CERT ((LPCSTR)3)

#define TIME_VALID_OID_FLUSH_FRESHEST_CRL_FROM_CERT ((LPCSTR)4)
#define TIME_VALID_OID_FLUSH_FRESHEST_CRL_FROM_CRL  ((LPCSTR)5)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+=========================================================================
//  Helper functions to build certificates
//==========================================================================

//+-------------------------------------------------------------------------
//
// Builds a self-signed certificate and returns a PCCERT_CONTEXT representing
// the certificate. A hProv may be specified to build the cert context.
//
// pSubjectIssuerBlob is the DN for the certifcate. If an alternate subject
// name is desired it must be specified as an extension in the pExtensions
// parameter. pSubjectIssuerBlob can NOT be NULL, so minimually an empty DN
// must be specified.
//
// By default:
// pKeyProvInfo - The CSP is queried for the KeyProvInfo parameters. Only the Provider,
// Provider Type and Container is queried. Many CSPs don't support these
// queries and will cause a failure. In such cases the pKeyProvInfo
// must be specified (RSA BASE works fine).
//
// pSignatureAlgorithm - will default to SHA1RSA
// pStartTime will default to the current time
// pEndTime will default to 1 year
// pEntensions will be empty.
//
// The returned PCCERT_CONTEXT will reference the private keys by setting the
// CERT_KEY_PROV_INFO_PROP_ID. However, if this property is not desired specify the
// CERT_CREATE_SELFSIGN_NO_KEY_INFO in dwFlags.
//
// If the cert being built is only a dummy placeholder cert for speed it may not
// need to be signed. Signing of the cert is skipped if CERT_CREATE_SELFSIGN_NO_SIGN
// is specified in dwFlags.
//
// Following flags can be passed to CertCreateSelfSignCertificate which will be
// directly passed to CryptExportPublicKeyInfo to indicate the preference of
// putting ECC Curve OID vs ECC Curve Parameters in Cert's Public Key information's
// algorithm section:
//      CRYPT_OID_USE_CURVE_NAME_FOR_ENCODE_FLAG
//      CRYPT_OID_USE_CURVE_PARAMETERS_FOR_ENCODE_FLAG
//--------------------------------------------------------------------------
WINCRYPT32API
PCCERT_CONTEXT
WINAPI
CertCreateSelfSignCertificate(
    _In_opt_  HCRYPTPROV_OR_NCRYPT_KEY_HANDLE hCryptProvOrNCryptKey,
    _In_      PCERT_NAME_BLOB             pSubjectIssuerBlob,
    _In_      DWORD                       dwFlags,
    _In_opt_  PCRYPT_KEY_PROV_INFO        pKeyProvInfo,
    _In_opt_  PCRYPT_ALGORITHM_IDENTIFIER pSignatureAlgorithm,
    _In_opt_  PSYSTEMTIME                 pStartTime,
    _In_opt_  PSYSTEMTIME                 pEndTime,
    _In_opt_  PCERT_EXTENSIONS            pExtensions
    );

#define CERT_CREATE_SELFSIGN_NO_SIGN    1
#define CERT_CREATE_SELFSIGN_NO_KEY_INFO 2

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+=========================================================================
//  Key Identifier Property Data Structures and APIs
//==========================================================================

//+-------------------------------------------------------------------------
//  Get the property for the specified Key Identifier.
//
//  The Key Identifier is the SHA1 hash of the encoded CERT_PUBLIC_KEY_INFO.
//  The Key Identifier for a certificate can be obtained by getting the
//  certificate's CERT_KEY_IDENTIFIER_PROP_ID. The
//  CryptCreateKeyIdentifierFromCSP API can be called to create the Key
//  Identifier from a CSP Public Key Blob.
//
//  A Key Identifier can have the same properties as a certificate context.
//  CERT_KEY_PROV_INFO_PROP_ID is the property of most interest.
//  For CERT_KEY_PROV_INFO_PROP_ID, pvData points to a CRYPT_KEY_PROV_INFO
//  structure. Elements pointed to by fields in the pvData structure follow the
//  structure. Therefore, *pcbData will exceed the size of the structure.
//
//  If CRYPT_KEYID_ALLOC_FLAG is set, then, *pvData is updated with a
//  pointer to allocated memory. LocalFree() must be called to free the
//  allocated memory.
//
//  By default, searches the CurrentUser's list of Key Identifiers.
//  CRYPT_KEYID_MACHINE_FLAG can be set to search the LocalMachine's list
//  of Key Identifiers. When CRYPT_KEYID_MACHINE_FLAG is set, pwszComputerName
//  can also be set to specify the name of a remote computer to be searched
//  instead of the local machine.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptGetKeyIdentifierProperty(
    _In_ const CRYPT_HASH_BLOB *pKeyIdentifier,
    _In_ DWORD dwPropId,
    _In_ DWORD dwFlags,
    _In_opt_ LPCWSTR pwszComputerName,
    _Reserved_ void *pvReserved,
    _Out_writes_bytes_to_opt_(*pcbData, *pcbData) void *pvData,
    _Inout_ DWORD *pcbData
    );

// When the following flag is set, searches the LocalMachine instead of the
// CurrentUser. This flag is applicable to all the KeyIdentifierProperty APIs.
#define CRYPT_KEYID_MACHINE_FLAG        0x00000020

// When the following flag is set, *pvData is updated with a pointer to
// allocated memory. LocalFree() must be called to free the allocated memory.
#define CRYPT_KEYID_ALLOC_FLAG          0x00008000


//+-------------------------------------------------------------------------
//  Set the property for the specified Key Identifier.
//
//  For CERT_KEY_PROV_INFO_PROP_ID pvData points to the
//  CRYPT_KEY_PROV_INFO data structure. For all other properties, pvData
//  points to a CRYPT_DATA_BLOB.
//
//  Setting pvData == NULL, deletes the property.
//
//  Set CRYPT_KEYID_MACHINE_FLAG to set the property for a LocalMachine
//  Key Identifier. Set pwszComputerName, to select a remote computer.
//
//  If CRYPT_KEYID_DELETE_FLAG is set, the Key Identifier and all its
//  properties is deleted.
//
//  If CRYPT_KEYID_SET_NEW_FLAG is set, the set fails if the property already
//  exists. For an existing property, FALSE is returned with LastError set to
//  CRYPT_E_EXISTS.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptSetKeyIdentifierProperty(
    _In_ const CRYPT_HASH_BLOB *pKeyIdentifier,
    _In_ DWORD dwPropId,
    _In_ DWORD dwFlags,
    _In_opt_ LPCWSTR pwszComputerName,
    _Reserved_ void *pvReserved,
    _In_opt_ const void *pvData
    );

// When the following flag is set, the Key Identifier and all its properties
// are deleted.
#define CRYPT_KEYID_DELETE_FLAG         0x00000010

// When the following flag is set, the set fails if the property already
// exists.
#define CRYPT_KEYID_SET_NEW_FLAG        0x00002000


//+-------------------------------------------------------------------------
//  For CERT_KEY_PROV_INFO_PROP_ID, rgppvData[] points to a
//  CRYPT_KEY_PROV_INFO.
//
//  Return FALSE to stop the enumeration.
//--------------------------------------------------------------------------
typedef BOOL (WINAPI *PFN_CRYPT_ENUM_KEYID_PROP)(
    _In_ const CRYPT_HASH_BLOB *pKeyIdentifier,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved,
    _Inout_opt_ void *pvArg,
    _In_ DWORD cProp,
    _In_reads_(cProp) DWORD *rgdwPropId,
    _In_reads_(cProp) void **rgpvData,
    _In_reads_(cProp) DWORD *rgcbData
    );

//+-------------------------------------------------------------------------
//  Enumerate the Key Identifiers.
//
//  If pKeyIdentifier is NULL, enumerates all Key Identifers. Otherwise,
//  calls the callback for the specified KeyIdentifier. If dwPropId is
//  0, calls the callback with all the properties. Otherwise, only calls
//  the callback with the specified property (cProp = 1).
//  Furthermore, when dwPropId is specified, skips KeyIdentifiers not
//  having the property.
//
//  Set CRYPT_KEYID_MACHINE_FLAG to enumerate the LocalMachine
//  Key Identifiers. Set pwszComputerName, to enumerate Key Identifiers on
//  a remote computer.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptEnumKeyIdentifierProperties(
    _In_opt_ const CRYPT_HASH_BLOB *pKeyIdentifier,
    _In_ DWORD dwPropId,
    _In_ DWORD dwFlags,
    _In_opt_ LPCWSTR pwszComputerName,
    _Reserved_ void *pvReserved,
    _Inout_opt_ void *pvArg,
    __callback PFN_CRYPT_ENUM_KEYID_PROP pfnEnum
    );

//+-------------------------------------------------------------------------
//  Create a KeyIdentifier from the CSP Public Key Blob.
//
//  Converts the CSP PUBLICKEYSTRUC into a X.509 CERT_PUBLIC_KEY_INFO and
//  encodes. The encoded CERT_PUBLIC_KEY_INFO is SHA1 hashed to obtain
//  the Key Identifier.
//
//  By default, the pPubKeyStruc->aiKeyAlg is used to find the appropriate
//  public key Object Identifier. pszPubKeyOID can be set to override
//  the default OID obtained from the aiKeyAlg.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CryptCreateKeyIdentifierFromCSP(
    _In_ DWORD dwCertEncodingType,
    _In_opt_ LPCSTR pszPubKeyOID,
    _In_reads_bytes_(cbPubKeyStruc) const PUBLICKEYSTRUC *pPubKeyStruc,
    _In_ DWORD cbPubKeyStruc,
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved,
    _Out_writes_bytes_to_opt_(*pcbHash, *pcbHash) BYTE *pbHash,
    _Inout_ DWORD *pcbHash
    );


//+=========================================================================
//  Certificate Chaining Infrastructure
//==========================================================================

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#define CERT_CHAIN_CONFIG_REGPATH \
    L"Software\\Microsoft\\Cryptography\\OID\\EncodingType 0\\CertDllCreateCertificateChainEngine\\Config"

// max size of the cryptographic object to download, in bytes
// NOTE: AIA has different configuration
#define CERT_CHAIN_MAX_URL_RETRIEVAL_BYTE_COUNT_VALUE_NAME      \
    L"MaxUrlRetrievalByteCount"
#define CERT_CHAIN_MAX_URL_RETRIEVAL_BYTE_COUNT_DEFAULT         (100*1024*1024)

// The following is a REG_BINARY. It contains the cache resync FILETIME.
// Any cached information before this time is considered time invalid
// and forces a wire retrieval. By default this is disabled.

#define CERT_CHAIN_CACHE_RESYNC_FILETIME_VALUE_NAME    \
    L"ChainCacheResyncFiletime"

// The following are REG_DWORD's. These configuration parameters are used
// to disable different chain building semantics enabled by default. Set
// the appropriate registry value to nonzero to disable.

#define CERT_CHAIN_DISABLE_MANDATORY_BASIC_CONSTRAINTS_VALUE_NAME  \
    L"DisableMandatoryBasicConstraints"
// By default the BasicConstraints extension must be present with CA enabled
// for non-Root intermediate CA certificates.

#define CERT_CHAIN_DISABLE_CA_NAME_CONSTRAINTS_VALUE_NAME  \
    L"DisableCANameConstraints"
// By default the NameConstraints extension is applied to the intermediate
// CA certificates in addition to the end entity certificate.

#define CERT_CHAIN_DISABLE_UNSUPPORTED_CRITICAL_EXTENSIONS_VALUE_NAME  \
    L"DisableUnsupportedCriticalExtensions"
// By default any unsupported extension marked critical sets the following
// dwErrorStatus bit: CERT_TRUST_HAS_NOT_SUPPORTED_CRITICAL_EXT.

// The following are REG_DWORD's. These configuration parameters are used
// to restrict Authority Info Access (AIA) URL retrieval.

#define CERT_CHAIN_MAX_AIA_URL_COUNT_IN_CERT_VALUE_NAME             \
    L"MaxAIAUrlCountInCert"
#define CERT_CHAIN_MAX_AIA_URL_COUNT_IN_CERT_DEFAULT                5

#define CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_COUNT_PER_CHAIN_VALUE_NAME \
    L"MaxAIAUrlRetrievalCountPerChain"
#define CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_COUNT_PER_CHAIN_DEFAULT    3

// max size of the object to download, specified by a URL in AIA extention, in bytes
#define CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_BYTE_COUNT_VALUE_NAME      \
    L"MaxAIAUrlRetrievalByteCount"
#define CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_BYTE_COUNT_DEFAULT         100000

#define CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_CERT_COUNT_VALUE_NAME      \
    L"MaxAIAUrlRetrievalCertCount"
#define CERT_CHAIN_MAX_AIA_URL_RETRIEVAL_CERT_COUNT_DEFAULT         10

// The following is a REG_DWORD. If the OCSP response NextUpdate is zero,
// this value is added to the ThisUpdate to get a nonzero NextUpdate.
#define CERT_CHAIN_OCSP_VALIDITY_SECONDS_VALUE_NAME                 \
    L"OcspValiditySeconds"
// 12 hours
#define CERT_CHAIN_OCSP_VALIDITY_SECONDS_DEFAULT    (12 * 60 * 60)


// The following is a REG_DWORD. It can be set to a nonzero value to disable
// the use of the Serial Chain optimization for SSL ServerAuth chains. This
// value is queried in each process on the first CertGetCertificateChain call
// where the CERT_SERIAL_CHAIN_PROP_ID property is set.
#define CERT_CHAIN_DISABLE_SERIAL_CHAIN_VALUE_NAME  \
    L"DisableSerialChain"

// The following is a REG_SZ containing the name of the file to log
// Serial Chain errors.
// The file's directory must already exist. If the file already
// exists, events are appended. Otherwise, the file is created.
// The directory/file should be ACL'ed so all processes and users have
// write access.
#define CERT_CHAIN_SERIAL_CHAIN_LOG_FILE_NAME_VALUE_NAME \
    L"SerialChainLogFileName"


// The following is a REG_DWORD. It can be set to a nonzero value to disable
// using SSL handshakes as a source of time. Will also disable the above
// Serial Chain optimization. Value is queried once on the first SSL
// handshake in the lsass.exe process.
#define CERT_CHAIN_DISABLE_SYNC_WITH_SSL_TIME_VALUE_NAME  \
    L"DisableSyncWithSslTime"

// The following is a REG_DWORD that specifies the maximum number of
// "SslTimeUpdated" events to be uploaded after boot or when the
// current time is synch'ed with SSL time. If not defined or a value of
// 0, uses the default value.
#define CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_VALUE_NAME \
    L"MaxSslTimeUpdatedEventCount"
#define CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_DEFAULT     5

// The following value disables uploading "SslTimeUpdated" events
#define CERT_CHAIN_MAX_SSL_TIME_UPDATED_EVENT_COUNT_DISABLE     0xFFFFFFFF

// The following is a REG_SZ containing the name of the file to log
// SSL handshakes that were processed.
// The file's directory must already exist. If the file already
// exists, events are appended. Otherwise, the file is created.
#define CERT_CHAIN_SSL_HANDSHAKE_LOG_FILE_NAME_VALUE_NAME \
    L"SslHandshakeLogFileName"


// The following is a REG_DWORD. Flags can be set to enable weak
// signature hash algorithms and/or weak public key lengths that
// are disabled by default. Also, has flags to enable logging of weak
// certificates.
//
#define CERT_CHAIN_ENABLE_WEAK_SIGNATURE_FLAGS_VALUE_NAME  \
    L"EnableWeakSignatureFlags"

// The following flag is set to enable MD2 or MD4 hashes that are
// disabled by default. If none, code signing, driver signing
// or time stamping requested EKUs are passed to CertGetCertificateChain API,
// then MD2 or MD4 isn't disabled by default.
#define CERT_CHAIN_ENABLE_MD2_MD4_FLAG              0x00000001

// The following flag is set to enable weak RSA public key lengths
// for trusted roots that are disabled by default.
#define CERT_CHAIN_ENABLE_WEAK_RSA_ROOT_FLAG        0x00000002

// The following flag is set to enable the logging of weak certificates
// to the directory identified by CERT_CHAIN_WEAK_SIGNATURE_LOG_DIR_VALUE_NAME.
// Not applicable to MD2 or MD4 certificates.
#define CERT_CHAIN_ENABLE_WEAK_LOGGING_FLAG         0x00000004

// The following flag is set to only log weak certificates. Disables
// weak signature errors from being returned. Not applicable
// to MD2 or MD4 certificates. 
#define CERT_CHAIN_ENABLE_ONLY_WEAK_LOGGING_FLAG    0x00000008


// The following is a REG_DWORD that specifies the minimum RSA public
// key length in bits. If not defined or a value of 0, uses the
// default value.
#define CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_VALUE_NAME    \
    L"MinRsaPubKeyBitLength"
#define CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_DEFAULT       1023

// The following value disables checking for weak RSA public key lengths.
#define CERT_CHAIN_MIN_RSA_PUB_KEY_BIT_LENGTH_DISABLE       \
    0xFFFFFFFF

// The following is a REG_DWORD that specifies the minimum RSA public
// key length in bits needed to trigger weak RSA errors.
#define CERT_CHAIN_MIN_WEAK_RSA_PUB_KEY_BIT_LENGTH_VALUE_NAME    \
    L"MinWeakRsaPubKeyBitLength"
#define CERT_CHAIN_MIN_WEAK_RSA_PUB_KEY_BIT_LENGTH_DEFAULT       2047

// The following value disables checking for weak RSA public key lengths.
#define CERT_CHAIN_MIN_WEAK_RSA_PUB_KEY_BIT_LENGTH_DISABLE       \
    0xFFFFFFFF

// The following is a REG_DWORD that specifies the minimum RSA public
// key length in bits needed to trigger telemetry.
#define CERT_CHAIN_MIN_TELEMETRY_RSA_PUB_KEY_BIT_LENGTH_VALUE_NAME    \
    L"MinTelemetryRsaPubKeyBitLength"
#define CERT_CHAIN_MIN_TELEMETRY_RSA_PUB_KEY_BIT_LENGTH_DEFAULT       2047

// The following value disables telemetry for weak RSA public key lengths.
#define CERT_CHAIN_MIN_TELEMETRY_RSA_PUB_KEY_BIT_LENGTH_DISABLE       \
    0xFFFFFFFF

// The following is a REG_BINARY containing the 8 byte FILETIME. The weak
// RSA public key length check is disabled for timestamped files before
// this time. If not defined or a zero FILETIME, uses the default value. 
#define CERT_CHAIN_WEAK_RSA_PUB_KEY_TIME_VALUE_NAME         \
    L"WeakRsaPubKeyTime"

// The default time: UTC: Fri Jan 01 00:00:00 2010
#define CERT_CHAIN_WEAK_RSA_PUB_KEY_TIME_DEFAULT            \
    0x01CA8A755C6E0000ui64

// The following is a REG_SZ. When defined, weak certificates are
// written to this directory. This directory should be ACL'ed to allow
// modify access by Authenticated Users and All Application Packages.
#define CERT_CHAIN_WEAK_SIGNATURE_LOG_DIR_VALUE_NAME        \
    L"WeakSignatureLogDir"

//+=========================================================================
//
// Weak Signature Registry Configuration
//
//==========================================================================

//
// The administrator will continue to place the weak crypto settings under:
// CERT_CHAIN_CONFIG_REGPATH defined above.
//
// The OS and Windows Update will configure the same settings in the
// "Default" subkey under CERT_CHAIN_CONFIG_REGPATH.
//

#define CERT_CHAIN_DEFAULT_CONFIG_SUBDIR        L"Default"

//
// The registry values will have the following name syntax:
//  "Weak"<CryptoAlg><ConfigType><ValueType>
//
// Where:
//  - <CryptoAlg> can be: "Md5", "Sha1", "Rsa", "Dsa" or "Ecdsa"
//  - <ConfigType> can be: "ThirdParty" or "All"
//  - <ValueType> can be: "Flags", "Hygiene", "MinBitLength", "AfterTime",
//    "FileHashAfterTime "TimestampHashAfterTime" or "Sha256Allow"
//     - "Hygiene" is only applicable to hash algorithms
//     - "FileHashAfterTime" and "TimestampHashAfterTime" are only applicable
//        to hash algorithms. These times can be in the future.
//     - "MinBitLengh" is only applicable to key algorithms
//  - <CryptoAlg>, <ConfigType> and <ValueType> will be present in all names.
//
//
// For example, all possible registry value names for Md5:
//  WeakMd5ThirdPartyFlags
//  WeakMd5AllFlags
//  WeakMd5ThirdPartyHygiene
//  WeakMd5AllHygiene
//  WeakMd5ThirdPartyAfterTime
//  WeakMd5AllAfterTime
//  WeakMd5ThirdPartyFileHashAfterTime
//  WeakMd5AllFileHashAfterTime
//  WeakMd5ThirdPartyTimestampHashAfterTime
//  WeakMd5AllTimestampHashAfterTime
//  WeakMd5ThirdPartySha256Allow
//  WeakMd5AllSha256Allow
//
// For example, all possible registry value names for Rsa:
//  WeakRsaThirdPartyFlags
//  WeakRsaAllFlags
//  WeakRsaThirdPartyAfterTime
//  WeakRsaAllAfterTime
//  WeakRsaThirdPartyMinBitLength
//  WeakRsaAllMinBitLength
//  WeakRsaThirdPartySha256Allow
//  WeakRsaAllSha256Allow
//
// The following registry values can be set:
//  - "Weak"<CryptoAlg><ConfigType>"Flags"
//     - REG_DWORD
//     - Flags can be set to disable the hash algorithm or enable a
//       minimum key length. See below for a complete list.
//  - "Weak"<HashCryptoAlg><ConfigType>"Hygiene"
//     - REG_DWORD or REG_QWORD. The REG_DWORD can be used until the number
//       of hygiene functions exceeds 32. We will support either registry type
//       for this value.
//     - This value corresponds to the qwHygieneFlags parameter passed to the
//       I_CertGetCertificateHygieneStatus internal API.
//     - The hygiene checks are skipped if the hash algorithm has been disabled.
//  - "Weak"<KeyCryptoAlg><ConfigType>"MinBitLength"
//     - REG_DWORD
//     - This value specifies the minimum public key length in bits.
//  - "Weak"<CryptoAlg><ConfigType>"AfterTime"
//     - REG_BINARY
//     - This value contains an 8 byte FILETIME. The weak crypto algorithm
//       check is disabled for time stamped files before this time.
//     - This configuration value isn't applicable to timestamp chains.
//     - This configuration value isn't applicable to hygiene checks.
//     - If this time is after the CurrentTime, then, the CurrentTime is used.
//  - "Weak"<CryptoAlg><ConfigType>"FileHashAfterTime"
//     - REG_BINARY
//     - This value contains an 8 byte FILETIME. The file hash weak crypto
//       algorithm check is disabled for time stamped files before this time.
//       This can be set to a date/time in the future.
//     - Only applicable to the API: CertIsWeakHash.
//  - "Weak"<CryptoAlg><ConfigType>"TimestampHashAfterTime"
//     - REG_BINARY
//     - This value contains an 8 byte FILETIME. The timestamp hash weak crypto
//       algorithm check is disabled before this time is reached. This can be
//       set to a date/time in the future.
//     - Only applicable to the API: CertIsWeakHash.
//  - "Weak"<CryptoAlg><ConfigType>"Sha256Allow"
//     - REG_SZ or REG_MULTI_SZ
//     - This value contains the list of certificate SHA256 thumbprints
//       (ASCII_HEX formatted) identifying weak certificates to be explicitly
//       allowed. Non ASCII_HEX characters in the string are skipped.
//       This allows embedded spaces.
//     - The resultant set used for either "Weak"<CryptoAlg>"ThirdParty" or
//       "Weak"<CryptoAlg>"All" is the union of:
//        - Default and Administrator
//        - "ThirdParty" and "All"
//

#define CERT_CHAIN_WEAK_PREFIX_NAME                     L"Weak"
#define CERT_CHAIN_WEAK_THIRD_PARTY_CONFIG_NAME         L"ThirdParty"
#define CERT_CHAIN_WEAK_ALL_CONFIG_NAME                 L"All"
#define CERT_CHAIN_WEAK_FLAGS_NAME                      L"Flags"
#define CERT_CHAIN_WEAK_HYGIENE_NAME                    L"Hygiene"
#define CERT_CHAIN_WEAK_AFTER_TIME_NAME                 L"AfterTime"
#define CERT_CHAIN_WEAK_FILE_HASH_AFTER_TIME_NAME       L"FileHashAfterTime"
#define CERT_CHAIN_WEAK_TIMESTAMP_HASH_AFTER_TIME_NAME  L"TimestampHashAfterTime"
#define CERT_CHAIN_WEAK_MIN_BIT_LENGTH_NAME             L"MinBitLength"
#define CERT_CHAIN_WEAK_SHA256_ALLOW_NAME               L"Sha256Allow"


// The following value disables checking for weak public key lengths.
#define CERT_CHAIN_MIN_PUB_KEY_BIT_LENGTH_DISABLE       \
    0xFFFFFFFF

// The following flags can be set in the above
// "Weak"<CryptoAlg><ConfigType>"Flags"

// If the following flag isn't set, then all other flags and registry values
// are ignored for this "Weak"<CryptoAlg><ConfigType>.
//
// If the administrator sets this flag for its "Weak"<CryptoAlg><ConfigType>,
// then, the corresponding Default OS/Windows Update settings are ignored.
//
// If this flag is set in "Weak"<CryptoAlg>"AllFlags":
//  - Resultant "Weak"<CryptoAlg>"ThirdPartyFlags" will or with
//    "Weak"<CryptoAlg>"AllFlags".  However, "Weak"<CryptoAlg>"ThirdPartyFlags"
//    logging flags won't be updated.
//
//    ThirdPartyFlags |= AllFlags &
//                           ~(CERT_CHAIN_ENABLE_WEAK_LOGGING_FLAG |
//                             CERT_CHAIN_ENABLE_ONLY_WEAK_LOGGING_FLAG);
//  - Resultant "Weak"<HashCryptoAlg>"ThirdPartyHygine" will or with
//    "Weak"<HashCryptoAlg>AllHygiene"
//  - Resultant "Weak<CryptoAlg>"ThirdPartyAfterTime" will be earliest
//    ("Weak"<CryptoAlg>"AllAfterTime", "Weak"<CryptoAlg>"ThirdPartyAfterTime").
//
//    Only applicable if "Weak"<CryptoAlg>"AllAfterTime" is defined and nonzero.
//  - Resultant "Weak"<KeyCryptoAlg>"ThirdPartyMinBitLength" will be largest
//    ("Weak"<KeyCryptoAlg>"AllMinBitLength",
//     "Weak"<KeyCryptoAlg>"ThirdPartyMinBitLength"
#define CERT_CHAIN_ENABLE_WEAK_SETTINGS_FLAG            0x80000000

// The following flag is set to enable the logging of weak certificates to the
// directory identified by CERT_CHAIN_WEAK_SIGNATURE_LOG_DIR_VALUE_NAME.
// #define CERT_CHAIN_ENABLE_WEAK_LOGGING_FLAG         0x00000004

// The following flag is set to only log weak certificates to the directory
// identified by CERT_CHAIN_WEAK_SIGNATURE_LOG_DIR_VALUE_NAME. Weak signature
// errors aren't returned.
// #define CERT_CHAIN_ENABLE_ONLY_WEAK_LOGGING_FLAG    0x00000008

// The following flag is set to disable ECC certificates using
// key parameters instead of normal key curve name OID.
#define CERT_CHAIN_DISABLE_ECC_PARA_FLAG                0x00000010

// In addition to setting the above CERT_CHAIN_ENABLE_WEAK_SETTINGS_FLAG flag,
// the following flags corresponding to the EKU must be set to disable weak
// signature or enable weak hash hygiene checks:

// This flag disables for all EKUs.
#define CERT_CHAIN_DISABLE_ALL_EKU_WEAK_FLAG            0x00010000

// This flag enables hygiene for all EKUs not disabling.
#define CERT_CHAIN_ENABLE_ALL_EKU_HYGIENE_FLAG          0x00020000

// This flag disables for ServerAuth EKUs only when CertGetCertificateChain
// is called with CERT_CHAIN_OPT_IN_WEAK_SIGNATURE.
#define CERT_CHAIN_DISABLE_OPT_IN_SERVER_AUTH_WEAK_FLAG 0x00040000

// This flag disables for ServerAuth EKUs.
#define CERT_CHAIN_DISABLE_SERVER_AUTH_WEAK_FLAG        0x00100000

// This flag enables hygiene for ServerAuth EKUs.
#define CERT_CHAIN_ENABLE_SERVER_AUTH_HYGIENE_FLAG      0x00200000

// This flag disables for code signing EKUs
#define CERT_CHAIN_DISABLE_CODE_SIGNING_WEAK_FLAG       0x00400000

// This flag disables for code signing EKUs only when CertGetCertificateChain
// is called with the Mark-Of-The-Web (CERT_CHAIN_HAS_MOTW) 
#define CERT_CHAIN_DISABLE_MOTW_CODE_SIGNING_WEAK_FLAG  0x00800000

// This flag enables hygiene for code signing EKUs
#define CERT_CHAIN_ENABLE_CODE_SIGNING_HYGIENE_FLAG     0x01000000

// This flag enables hygiene for code signing EKUs only when
// CertGetCertificateChain is called with the
// Mark-Of-The-Web (CERT_CHAIN_HAS_MOTW)
#define CERT_CHAIN_ENABLE_MOTW_CODE_SIGNING_HYGIENE_FLAG 0x02000000

// This flag disables for timestamp EKUs.
#define CERT_CHAIN_DISABLE_TIMESTAMP_WEAK_FLAG          0x04000000

// This flag disables for timestamp EKUs only when CertGetCertificateChain is
// called with the Mark-Of-The-Web (CERT_CHAIN_HAS_MOTW)
#define CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_WEAK_FLAG     0x08000000

// This flag enables hygiene for timestamp EKUs
#define CERT_CHAIN_ENABLE_TIMESTAMP_HYGIENE_FLAG        0x10000000

// This flag enables hygiene for timestamp EKUs only when
// CertGetCertificateChain is called with the
// Mark-Of-The-Web (CERT_CHAIN_HAS_MOTW)
#define CERT_CHAIN_ENABLE_MOTW_TIMESTAMP_HYGIENE_FLAG   0x20000000

// This flag ignores the "Weak"<CryptoAlg><ConfigType>"AfterTime" value
// when CertGetCertificateChain is called with the
// Mark-Of-The-Web (CERT_CHAIN_HAS_MOTW)
#define CERT_CHAIN_MOTW_IGNORE_AFTER_TIME_WEAK_FLAG     0x40000000

// If the hash algorithm is disabled, then, the hygiene check will be skipped.
// The hygiene flags are only applicable to hash algorithms.

// If no EKU flags are set, then, weak crypto isn't enforced for the
// "Weak"<CryptoAlg><ConfigType>. This allows the administrator to always
// ignore the Default OS/Windows Update settings.

// This flag disables for file hashes. Only applicable to CertIsWeakHash()
// API.
#define CERT_CHAIN_DISABLE_FILE_HASH_WEAK_FLAG          0x00001000

// This flag disables for file hashes. Only applicable when CertIsWeakHash()
// API is called with the Mark-Of-The-Web (CERT_CHAIN_HAS_MOTW)
#define CERT_CHAIN_DISABLE_MOTW_FILE_HASH_WEAK_FLAG     0x00002000

// This flag disables for timestamp hashes. Only applicable to CertIsWeakHash()
// API.
#define CERT_CHAIN_DISABLE_TIMESTAMP_HASH_WEAK_FLAG     0x00004000

// This flag disables for timestamp hashes. Only applicable when CertIsWeakHash()
// API is called with the Mark-Of-The-Web (CERT_CHAIN_HAS_MOTW)
#define CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_HASH_WEAK_FLAG 0x00008000


#define CERT_CHAIN_DISABLE_WEAK_FLAGS (                 \
    CERT_CHAIN_DISABLE_ECC_PARA_FLAG |                  \
    CERT_CHAIN_DISABLE_ALL_EKU_WEAK_FLAG |              \
    CERT_CHAIN_DISABLE_SERVER_AUTH_WEAK_FLAG |          \
    CERT_CHAIN_DISABLE_OPT_IN_SERVER_AUTH_WEAK_FLAG |   \
    CERT_CHAIN_DISABLE_CODE_SIGNING_WEAK_FLAG |         \
    CERT_CHAIN_DISABLE_MOTW_CODE_SIGNING_WEAK_FLAG |    \
    CERT_CHAIN_DISABLE_TIMESTAMP_WEAK_FLAG |            \
    CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_WEAK_FLAG )

#define CERT_CHAIN_DISABLE_FILE_HASH_WEAK_FLAGS (       \
    CERT_CHAIN_DISABLE_FILE_HASH_WEAK_FLAG |            \
    CERT_CHAIN_DISABLE_MOTW_FILE_HASH_WEAK_FLAG )

#define CERT_CHAIN_DISABLE_TIMESTAMP_HASH_WEAK_FLAGS (  \
    CERT_CHAIN_DISABLE_TIMESTAMP_HASH_WEAK_FLAG |       \
    CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_HASH_WEAK_FLAG )

#define CERT_CHAIN_ENABLE_HYGIENE_FLAGS (               \
    CERT_CHAIN_ENABLE_ALL_EKU_HYGIENE_FLAG |            \
    CERT_CHAIN_ENABLE_SERVER_AUTH_HYGIENE_FLAG |        \
    CERT_CHAIN_ENABLE_CODE_SIGNING_HYGIENE_FLAG |       \
    CERT_CHAIN_ENABLE_MOTW_CODE_SIGNING_HYGIENE_FLAG |  \
    CERT_CHAIN_ENABLE_TIMESTAMP_HYGIENE_FLAG |          \
    CERT_CHAIN_ENABLE_MOTW_TIMESTAMP_HYGIENE_FLAG )

#define CERT_CHAIN_MOTW_WEAK_FLAGS (                    \
    CERT_CHAIN_DISABLE_MOTW_CODE_SIGNING_WEAK_FLAG |    \
    CERT_CHAIN_DISABLE_MOTW_TIMESTAMP_WEAK_FLAG |       \
    CERT_CHAIN_ENABLE_MOTW_CODE_SIGNING_HYGIENE_FLAG |  \
    CERT_CHAIN_ENABLE_MOTW_TIMESTAMP_HYGIENE_FLAG |     \
    CERT_CHAIN_MOTW_IGNORE_AFTER_TIME_WEAK_FLAG)

#define CERT_CHAIN_OPT_IN_WEAK_FLAGS (                  \
    CERT_CHAIN_DISABLE_OPT_IN_SERVER_AUTH_WEAK_FLAG)

//+=========================================================================
//
// Certificate Chain Engine Auto Flush Registry Configuration
//
//==========================================================================

//
// The following registry values are under the 
// CERT_CHAIN_CONFIG_REGPATH defined above.
//

//
// Types of certificate chain engine auto create and flush events
//

#define CERT_CHAIN_AUTO_CURRENT_USER                1
#define CERT_CHAIN_AUTO_LOCAL_MACHINE               2
#define CERT_CHAIN_AUTO_IMPERSONATED                3
#define CERT_CHAIN_AUTO_PROCESS_INFO                4
#define CERT_CHAIN_AUTO_PINRULE_INFO                5
#define CERT_CHAIN_AUTO_NETWORK_INFO                6
#define CERT_CHAIN_AUTO_SERIAL_LOCAL_MACHINE        7
#define CERT_CHAIN_AUTO_HPKP_RULE_INFO              8

// The following is a REG_DWORD that can be set to disable
// auto flush or enable the logging of auto create, free or
// flush events. By default, auto flush is enabled without
// any logging.
#define CERT_CHAIN_AUTO_FLAGS_VALUE_NAME                        \
    L"AutoFlags"

#define CERT_CHAIN_AUTO_FLUSH_DISABLE_FLAG          0x00000001
#define CERT_CHAIN_AUTO_LOG_CREATE_FLAG             0x00000002
#define CERT_CHAIN_AUTO_LOG_FREE_FLAG               0x00000004
#define CERT_CHAIN_AUTO_LOG_FLUSH_FLAG              0x00000008

#define CERT_CHAIN_AUTO_LOG_FLAGS (         \
    CERT_CHAIN_AUTO_LOG_CREATE_FLAG |       \
    CERT_CHAIN_AUTO_LOG_FREE_FLAG |         \
    CERT_CHAIN_AUTO_LOG_FLUSH_FLAG )


// The following are REG_DWORDs. If the registry value doesn't exist or
// is set to zero, then, the DEFAULT is used.

// This is the delta time in seconds to set the first timeout.
// At the first timeout we set the initial next timeout.
#define CERT_CHAIN_AUTO_FLUSH_FIRST_DELTA_SECONDS_VALUE_NAME    \
    L"AutoFlushFirstDeltaSeconds"

// 5 Minutes
#define CERT_CHAIN_AUTO_FLUSH_FIRST_DELTA_SECONDS_DEFAULT       \
    (5 * 60)

// This is the delta time in seconds to set the next timeouts.
// For each next timeout, we check if there was any chain
// engine usage (such as CertGetCertificateChain) since
// the last timeout. Auto flush is triggered if there wasn't
// any usage. Otherwise, we set the next timeout to this delta time.
#define CERT_CHAIN_AUTO_FLUSH_NEXT_DELTA_SECONDS_VALUE_NAME     \
    L"AutoFlushNextDeltaSeconds"

// 30 Minutes
#define CERT_CHAIN_AUTO_FLUSH_NEXT_DELTA_SECONDS_DEFAULT        \
    (30 * 60)

// The following is REG_SZ containing the name of the file to log
// the certificate chain engine auto events to.
// The file's directory must already exist. If the file already
// exists, events are appended. Otherwise, the file is created.
// The directory/file should be ACL'ed so all processes and users have
// write access.
#define CERT_CHAIN_AUTO_LOG_FILE_NAME_VALUE_NAME                \
    L"AutoLogFileName"

// The following is REG_MULTI_SZ containing the list of
// process names to disable auto flush for.
//
// Auto flush is always disabled for the lsass.exe process. It doesn't need
// to be in the following registry value list.
#define CERT_CHAIN_DISABLE_AUTO_FLUSH_PROCESS_NAME_LIST_VALUE_NAME \
    L"DisableAutoFlushProcessNameList"


// The following are REG_DWORD's. These configuration parameters are
// used by the following APIs to get a non-blocking, time valid OCSP
// response for a server certificate chain:
//   CertOpenServerOcspResponse
//   CertAddRefServerOcspResponse
//   CertCloseServerOcspResponse
//   CertGetServerOcspResponseContext
//   CertAddRefServerOcspResponseContext
//   CertFreeServerOcspResponseContext

// This is the minimum validity of the server OCSP response to be
// returned by CertGetServerOcspResponseContext(). Since this OCSP
// response will be returned to the client, it must be sufficiently long
// so that the client will treat it as being time valid.
#define CERT_SRV_OCSP_RESP_MIN_VALIDITY_SECONDS_VALUE_NAME \
    L"SrvOcspRespMinValiditySeconds"
// 10 minutes
#define CERT_SRV_OCSP_RESP_MIN_VALIDITY_SECONDS_DEFAULT \
    (10 * 60)

// This is the maximum number of milliseconds for each server OCSP response
// pre-fetch wire URL retrieval.
#define CERT_SRV_OCSP_RESP_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_VALUE_NAME \
    L"SrvOcspRespUrlRetrievalTimeoutMilliseconds"
// 15 seconds
#define CERT_SRV_OCSP_RESP_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_DEFAULT \
    (15 * 1000)

// This is the maximum number of seconds to do a server OCSP response
// pre-fetch retrieval before the OCSP response's NextUpdate. The
// server OCSP response pre-fetch thread will wait until CurrentTime >=
// NextUpdate - MaxBeforeNextUpdateSeconds before doing the next retrieval.
#define CERT_SRV_OCSP_RESP_MAX_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME \
    L"SrvOcspRespMaxBeforeNextUpdateSeconds"
// 4 hours
#define CERT_SRV_OCSP_RESP_MAX_BEFORE_NEXT_UPDATE_SECONDS_DEFAULT \
    (4 * 60 * 60)

// This is the minimum number of seconds to do a server OCSP response
// pre-fetch retrieval before the OCSP response's NextUpdate.
// If CurrentTime >= NextUpdate - MinBeforeNextUpdateSeconds, will wait until
// after NextUpdate + MinAfterNextUpdateSeconds.
#define CERT_SRV_OCSP_RESP_MIN_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME \
    L"SrvOcspRespMinBeforeNextUpdateSeconds"
// 2 minutes
#define CERT_SRV_OCSP_RESP_MIN_BEFORE_NEXT_UPDATE_SECONDS_DEFAULT \
    (2 * 60)

// This is the minimum number of seconds to do a server OCSP response
// pre-fetch retrieval after the OCSP response's NextUpdate when
// (NextUpdate - MinBeforeNextUpdateSeconds) < CurrentTime < NextUpdate.
#define CERT_SRV_OCSP_RESP_MIN_AFTER_NEXT_UPDATE_SECONDS_VALUE_NAME\
    L"SrvOcspRespMinAfterNextUpdateSeconds"
// 1 minute
#define CERT_SRV_OCSP_RESP_MIN_AFTER_NEXT_UPDATE_SECONDS_DEFAULT \
    (1 * 60)


// This is the minimum number of seconds between certificate directory
// update sync checks. Used by certutil.exe for the downloadOcsp option.
#define CERT_SRV_OCSP_RESP_MIN_SYNC_CERT_FILE_SECONDS_VALUE_NAME \
    L"SrvOcspRespMinSyncCertFileSeconds"
// 5 seconds
#define CERT_SRV_OCSP_RESP_MIN_SYNC_CERT_FILE_SECONDS_DEFAULT \
    5

// This is the maximum number of seconds between certificate directory
// update sync checks. Used by certutil.exe for the downloadOcsp option.
#define CERT_SRV_OCSP_RESP_MAX_SYNC_CERT_FILE_SECONDS_VALUE_NAME \
    L"SrvOcspRespMaxSyncCertFileSeconds"
// 1 hour
#define CERT_SRV_OCSP_RESP_MAX_SYNC_CERT_FILE_SECONDS_DEFAULT \
    (1 * 60 * 60)

// The following are REG_DWORD's. These configuration parameters are used
// in the ordering of the revocation retrieval URLs.


// When the number of cached OCSP URLs associated with the same CDP extension
// equal or exceed this number, the OCSP AIA URLs aren't used.
#define CRYPTNET_MAX_CACHED_OCSP_PER_CRL_COUNT_VALUE_NAME \
    L"CryptnetMaxCachedOcspPerCrlCount"
#define CRYPTNET_MAX_CACHED_OCSP_PER_CRL_COUNT_DEFAULT \
    500

// The above registry value can be set to this value, to disable OCSP
// when a CDP extension is present. Note, a registry value of 0, uses the
// above default value.
#define CRYPTNET_OCSP_AFTER_CRL_DISABLE \
    0xFFFFFFFF

// The following are REG_DWORD's. These configuration parameters are
// used by the Cryptnet Url Cache Service (CUCS).

// The following parameter is used as the default flush exempt seconds
#define CRYPTNET_URL_CACHE_DEFAULT_FLUSH_EXEMPT_SECONDS_VALUE_NAME \
    L"CryptnetDefaultFlushExemptSeconds"

// 4 Weeks : 28 days * 24 hours * 60 minutes * 60 seconds
#define CRYPTNET_URL_CACHE_DEFAULT_FLUSH_EXEMPT_SECONDS_DEFAULT \
    (28 * 24 * 60 * 60)

// Following 2 parameters are used to set the lower and upper limit
// on the max-age retrievals done before the Publish and NextUpdate times.
#define CRYPTNET_PRE_FETCH_MIN_MAX_AGE_SECONDS_VALUE_NAME \
    L"CryptnetPreFetchMinMaxAgeSeconds"
// 1 hour
#define CRYPTNET_PRE_FETCH_MIN_MAX_AGE_SECONDS_DEFAULT \
    (1 * 60 * 60)

#define CRYPTNET_PRE_FETCH_MAX_MAX_AGE_SECONDS_VALUE_NAME \
    L"CryptnetPreFetchMaxMaxAgeSeconds"
// 2 Weeks : 14 days * 24 hours * 60 minutes * 60 seconds
#define CRYPTNET_PRE_FETCH_MAX_MAX_AGE_SECONDS_DEFAULT \
    (14 * 24 * 60 * 60)

// Following parameter is used to set the lower limit on the
// OCSP validity period
#define CRYPTNET_PRE_FETCH_MIN_OCSP_VALIDITY_PERIOD_SECONDS_VALUE_NAME \
    L"CryptnetPreFetchMinOcspValidityPeriodSeconds"
// 2 Weeks : 14 days * 24 hours * 60 minutes * 60 seconds
#define CRYPTNET_PRE_FETCH_MIN_OCSP_VALIDITY_PERIOD_SECONDS_DEFAULT \
    (14 * 24 * 60 * 60)

// Following 3 parameters are used to calculate the PreFetch start before
// the NextUpdate
//
// Where PreFetchStartTime = PublishTime +
//                              PublishPeriod / AfterPublishPreFetchDivisor
//       PreFetchEndTime = NextUpdate -
//                              PublishPeriod / BeforeNextUpdatePreFetchDivisor
//
//       PreFetchPeriod = PreFetchEndTime - PreFetchStartTime
//
//       if (PreFetchPeriod < MinBeforeNextUpdatePreFetchPeriodSeconds)
//          - No PreFetch is done before NextUpdate
//       else
//          - PreFetch starts are randomized over this period

// The start of the PreFetch period is delayed after the start of the
// Publish period by dividing the PublishPeriod (NextUpdate - PublishTime)
// by this integer divisor.
#define CRYPTNET_PRE_FETCH_AFTER_PUBLISH_PRE_FETCH_DIVISOR_VALUE_NAME \
    L"CryptnetPreFetchAfterPublishPreFetchDivisor"
// 10, where 12 hours / 10 = 72 minutes or 1.2 hours / 10 = 7.2 minutes
#define CRYPTNET_PRE_FETCH_AFTER_PUBLISH_PRE_FETCH_DIVISOR_DEFAULT \
    10

// The finish of the PreFetch period occurs before NextUpdate
// by dividing the PublishPeriod (NextUpdate - PublishTime)
// by this integer divisor.
#define CRYPTNET_PRE_FETCH_BEFORE_NEXT_UPDATE_PRE_FETCH_DIVISOR_VALUE_NAME \
    L"CryptnetPreFetchBeforeNextUpdatePreFetchDivisor"
// 20, where 12 hours / 20 = 36 minutes or 1.2 hours / 10 = 3.6 minutes
#define CRYPTNET_PRE_FETCH_BEFORE_NEXT_UPDATE_PRE_FETCH_DIVISOR_DEFAULT \
    20

// The PreFetch period must exceed this minimum duration in seconds
// to do a PreFetch before NextUpdate
#define CRYPTNET_PRE_FETCH_MIN_BEFORE_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME \
    L"CryptnetPreFetchMinBeforeNextUpdatePreFetchSeconds"
// 1 hour
//
// For the default OCSP period of 12 hours using above defaults,
// PreFetchPeriod = 72 minutes - 7.2 minutes - 3.6 mintes = 61.2 minutes
#define CRYPTNET_PRE_FETCH_MIN_BEFORE_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_DEFAULT \
    (1 * 60 * 60)

// Following 4 parameters are used to calculate the PreFetch start after
// the NextUpdate
//
// ValidityPeriod = NextUpdate - ThisUpdate
//
// PreFetchPeriod = ValidityPeriod / AfterNextUpdatePreFetchDivisor
//
// Where PreFetchPeriod is decreased to MaxAfterNextUpdatePreFetchPeriodSeconds
// or increased to MinAfterNextUpdatePreFetchPeriodSeconds;
//
// PreFetchStartTime = NextUpdate
// PreFetchEndTime = PreFetchStartTime + PreFetchPeriod
//
// PreFetch starts are randomized over the above PreFetchPeriod
//
// If CurrentTime > RandomPreFetchStartTime, then, the
// AfterCurrentTimePreFetchPeriodSeconds is randomized and added to
// CurrentTime for the RandomPreFetchStartTime

// The PreFetch period after NextUpdate is initially calculated by
// dividing the ValidityPeriod (NextUpdate - ThisUpdate) by this integer
// divisor.
#define CRYPTNET_PRE_FETCH_VALIDITY_PERIOD_AFTER_NEXT_UPDATE_PRE_FETCH_DIVISOR_VALUE_NAME \
    L"CryptnetPreFetchValidityPeriodAfterNextUpdatePreFetchDivisor"
// 10, where 1 week / 10 = 16.8 hours
#define CRYPTNET_PRE_FETCH_VALIDITY_PERIOD_AFTER_NEXT_UPDATE_PRE_FETCH_DIVISOR_DEFAULT \
    10

// If necessary, the above PreFetch period will be decreased
// to this maximum duration in seconds.
#define CRYPTNET_PRE_FETCH_MAX_AFTER_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME \
    L"CryptnetPreFetchMaxAfterNextUpdatePreFetchPeriodSeconds"
// 4 hours
#define CRYPTNET_PRE_FETCH_MAX_AFTER_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_DEFAULT \
    (4 * 60 * 60)

// If necessary, the above PreFetch period will be increased
// to this minimum duration in seconds.
#define CRYPTNET_PRE_FETCH_MIN_AFTER_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME \
    L"CryptnetPreFetchMinAfterNextUpdatePreFetchPeriodSeconds"
// 30 minutes
#define CRYPTNET_PRE_FETCH_MIN_AFTER_NEXT_UPDATE_PRE_FETCH_PERIOD_SECONDS_DEFAULT \
    (30 * 60)

// If the CurrentTime is after the above randomized start time, the following
// parameter will be randomized and added to the CurrentTime.
#define CRYPTNET_PRE_FETCH_AFTER_CURRENT_TIME_PRE_FETCH_PERIOD_SECONDS_VALUE_NAME \
    L"CryptnetPreFetchAfterCurrentTimePreFetchPeriodSeconds"
// 30 minutes
#define CRYPTNET_PRE_FETCH_AFTER_CURRENT_TIME_PRE_FETCH_PERIOD_SECONDS_DEFAULT \
    (30 * 60)


// Following parameter specifies the minimum time period between sending
// trigger URL cache PreFetch LRPC messages to cryptsvc after doing online
// revocation enabled chain builds.
#define CRYPTNET_PRE_FETCH_TRIGGER_PERIOD_SECONDS_VALUE_NAME \
    L"CryptnetPreFetchTriggerPeriodSeconds"
// 10 minutes
#define CRYPTNET_PRE_FETCH_TRIGGER_PERIOD_SECONDS_DEFAULT \
    (10 * 60)

// The above registry value can be set to this value, to disable the
// sending of trigger URL cache PreFetch LRPC messages. Note, a registry
// value of 0, uses the above default value.
#define CRYPTNET_PRE_FETCH_TRIGGER_DISABLE \
    0xFFFFFFFF

// Following parameter specifies the delay time to wait to scan the
// URL cache directory after receiving a trigger LRPC message request.
#define CRYPTNET_PRE_FETCH_SCAN_AFTER_TRIGGER_DELAY_SECONDS_VALUE_NAME \
    L"CryptnetPreFetchScanAfterTriggerDelaySeconds"
// 60 seconds
#define CRYPTNET_PRE_FETCH_SCAN_AFTER_TRIGGER_DELAY_SECONDS_DEFAULT \
    60

// Following parameter specifies the maximum amount of time to wait for any
// PreFetch retrieval to complete
#define CRYPTNET_PRE_FETCH_RETRIEVAL_TIMEOUT_SECONDS_VALUE_NAME \
    L"CryptnetPreFetchRetrievalTimeoutSeconds"
// 5 minutes
#define CRYPTNET_PRE_FETCH_RETRIEVAL_TIMEOUT_SECONDS_DEFAULT \
    (5 * 60)


//+-------------------------------------------------------------------------
// Cryptnet in-process CRL PreFetch configuration
//--------------------------------------------------------------------------

#define CRYPTNET_CRL_PRE_FETCH_CONFIG_REGPATH  \
    CERT_CHAIN_CONFIG_REGPATH L"\\CrlPreFetch"

// The following is REG_MULTI_SZ containing the list of
// process names to enable PreFetching for
#define CRYPTNET_CRL_PRE_FETCH_PROCESS_NAME_LIST_VALUE_NAME \
    L"ProcessNameList"

// The following is REG_MULTI_SZ containing the list of
// CRL Urls to be PreFetched. This should be the encoded format used
// in the certificate. Normally, the one with the %20 for the " " space 
// character.
#define CRYPTNET_CRL_PRE_FETCH_URL_LIST_VALUE_NAME \
    L"PreFetchUrlList"

// By default PreFetch information events are logged to the Windows
// Application Logs. The following REG_DWORD can be set to nonzero
// value to disable the logging.
#define CRYPTNET_CRL_PRE_FETCH_DISABLE_INFORMATION_EVENTS_VALUE_NAME \
    L"DisableInformationEvents"

// The following is REG_SZ containing the name of the file to log verbose events
// to. The file's directory must already exist. If the file already
// exists, events are appended. Otherwise, the file is created.
// The directory/file should be ACL'ed so all processes and users have
// write access.
#define CRYPTNET_CRL_PRE_FETCH_LOG_FILE_NAME_VALUE_NAME \
    L"LogFileName"

// The following are REG_DWORDs. If the registry value doesn't exist or
// is set to zero, then, the DEFAULT is used.

// Following parameter specifies the maximum amount of time to wait for any
// CRL PreFetch retrieval to complete
#define CRYPTNET_CRL_PRE_FETCH_TIMEOUT_SECONDS_VALUE_NAME \
    L"TimeoutSeconds"
// 5 minutes
#define CRYPTNET_CRL_PRE_FETCH_TIMEOUT_SECONDS_DEFAULT \
    (5 * 60)

// Following parameter specifies the max-age retrievals before the
// expected publish time. Setting to any value >= NextUpdate - ThisUpdate
// will disable. 5 minutes is the minimum.
#define CRYPTNET_CRL_PRE_FETCH_MAX_AGE_SECONDS_VALUE_NAME \
    L"MaxAgeSeconds"
// 2 hours
#define CRYPTNET_CRL_PRE_FETCH_MAX_AGE_SECONDS_DEFAULT \
    (2 * 60 * 60)

// 5 minutes
#define CRYPTNET_CRL_PRE_FETCH_MAX_AGE_SECONDS_MIN \
    (5 * 60)

// Following parameter specifies the expected publish time before
// NextUpdate
#define CRYPTNET_CRL_PRE_FETCH_PUBLISH_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME \
    L"PublishBeforeNextUpdateSeconds"
// 1 hour
#define CRYPTNET_CRL_PRE_FETCH_PUBLISH_BEFORE_NEXT_UPDATE_SECONDS_DEFAULT \
    (1 * 60 * 60)

// Following parameter specifies the interval to be randomized and
// subtracted from the expected publish time. Setting to any value
// >= publish time will disable randomization. Setting to 1
// will disable any randomization. Setting to 0 will use the DEFAULT.
#define CRYPTNET_CRL_PRE_FETCH_PUBLISH_RANDOM_INTERVAL_SECONDS_VALUE_NAME \
    L"PublishRandomIntervalSeconds"
// 5 minutes
#define CRYPTNET_CRL_PRE_FETCH_PUBLISH_RANDOM_INTERVAL_SECONDS_DEFAULT \
    (5 * 60)

// Following parameter specifies the minimum time before NextUpdate
#define CRYPTNET_CRL_PRE_FETCH_MIN_BEFORE_NEXT_UPDATE_SECONDS_VALUE_NAME \
    L"MinBeforeNextUpdateSeconds"
// 5 minutes
#define CRYPTNET_CRL_PRE_FETCH_MIN_BEFORE_NEXT_UPDATE_SECONDS_DEFAULT \
    (5 * 60)

// Following parameter specifies the minimum time after NextUpdate
#define CRYPTNET_CRL_PRE_FETCH_MIN_AFTER_NEXT_UPDATE_SECONDS_VALUE_NAME \
    L"MinAfterNextUpdateSeconds"
// 5 minutes
#define CRYPTNET_CRL_PRE_FETCH_MIN_AFTER_NEXT_UPDATE_SECONDS_DEFAULT \
    (5 * 60)


//+-------------------------------------------------------------------------
// The following configuration parameters are store in HKLM group policy
//--------------------------------------------------------------------------

#define CERT_GROUP_POLICY_CHAIN_CONFIG_REGPATH \
    CERT_GROUP_POLICY_SYSTEM_STORE_REGPATH L"\\ChainEngine\\Config"

// In Vista, the following have been moved from the above HKLM
// configuration parameters:

// The following are REG_DWORD's. These configuration parameters are used
// to override the default URL timeouts in chain building

// This is the default URL timeout in milliseconds
#define CERT_CHAIN_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_VALUE_NAME    \
    L"ChainUrlRetrievalTimeoutMilliseconds"
// 15 seconds
#define CERT_CHAIN_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_DEFAULT       \
    (15 * 1000)

// This is the default revocation accumulative URL timeout in milliseconds
// The first revocation URL retrieval uses half of this timeout
#define CERT_CHAIN_REV_ACCUMULATIVE_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_VALUE_NAME \
    L"ChainRevAccumulativeUrlRetrievalTimeoutMilliseconds"
// 20 seconds
#define CERT_CHAIN_REV_ACCUMULATIVE_URL_RETRIEVAL_TIMEOUT_MILLISECONDS_DEFAULT \
    (20 * 1000)

// REG_DWORD: Set this value to non-zero in order to enable Internet connections
// with Unknown Authorization
#define CERT_RETR_BEHAVIOR_INET_AUTH_VALUE_NAME     L"EnableInetUnknownAuth"

// REG_DWORD: Set this value to non-zero in order to override Internet
// connectivity status allowing LOCAL to be treated as INTERNET.
#define CERT_RETR_BEHAVIOR_INET_STATUS_VALUE_NAME   L"EnableInetLocal"

// REG_DWORD: Set this value to non-zero in order to allow
// file:// URL scheme.
#define CERT_RETR_BEHAVIOR_FILE_VALUE_NAME          L"AllowFileUrlScheme"

// REG_DWORD: Set this value to non-zero in order to disable
// LDAP mutual authentication and & encryption.
#define CERT_RETR_BEHAVIOR_LDAP_VALUE_NAME          L"DisableLDAPSignAndEncrypt"

// Note, will allow the machine setting to be used if this value isn't
// defined.


// By default AIA OCSP URLs are before CDP CRL URLs. When the number of cached
// OCSP URLs associated with the same CDP extension equal or exceed this
// number, the CRL URLs are placed before the OCSP URLs.
#define CRYPTNET_CACHED_OCSP_SWITCH_TO_CRL_COUNT_VALUE_NAME \
    L"CryptnetCachedOcspSwitchToCrlCount"
#define CRYPTNET_CACHED_OCSP_SWITCH_TO_CRL_COUNT_DEFAULT \
    50

// The above registry value can be set to this value, to always place
// the CRL URLs before the OCSP URLs. Note, a registry value of 0, uses the
// above default value.
#define CRYPTNET_CRL_BEFORE_OCSP_ENABLE \
    0xFFFFFFFF


// Support for the following was removed in Vista. Changed to use
// the following OPTIONS flags in HKLM Group Policy
#define CERT_CHAIN_DISABLE_AIA_URL_RETRIEVAL_VALUE_NAME             \
    L"DisableAIAUrlRetrieval"
// By default AIA Url Retrieval is enabled. Set this registry value to nonzero
// to disable


// This is the name of the REG_DWORD for chain engine Options
#define CERT_CHAIN_OPTIONS_VALUE_NAME \
    L"Options"
// Disable AIA URL retrieval when this bit is set in the Options
#define CERT_CHAIN_OPTION_DISABLE_AIA_URL_RETRIEVAL                 0x2
// Enable SIA URL retrieval when this bit is set in the Options
#define CERT_CHAIN_OPTION_ENABLE_SIA_URL_RETRIEVAL                  0x4


#define CERT_CHAIN_CROSS_CERT_DOWNLOAD_INTERVAL_HOURS_VALUE_NAME \
    L"CrossCertDownloadIntervalHours"
// 7 days
#define CERT_CHAIN_CROSS_CERT_DOWNLOAD_INTERVAL_HOURS_DEFAULT       (24 * 7)

// When not defined or zero, the CRL validity isn't extended
#define CERT_CHAIN_CRL_VALIDITY_EXT_PERIOD_HOURS_VALUE_NAME \
    L"CRLValidityExtensionPeriod"
// 12 hour
#define CERT_CHAIN_CRL_VALIDITY_EXT_PERIOD_HOURS_DEFAULT            12


//
// The chain engine defines the store namespace and cache partitioning for
// the Certificate Chaining infrastructure.  A default chain engine
// is defined for the process which uses all default system stores e.g.
// Root, CA, Trust, for chain building and caching.  If an application
// wishes to define its own store namespace or have its own partitioned
// cache then it can create its own chain engine.  It is advisable to create
// a chain engine at application startup and use it throughout the lifetime
// of the application in order to get optimal caching behavior
//

typedef HANDLE HCERTCHAINENGINE;

#define HCCE_CURRENT_USER           ((HCERTCHAINENGINE)NULL)
#define HCCE_LOCAL_MACHINE          ((HCERTCHAINENGINE)0x1)
#define HCCE_SERIAL_LOCAL_MACHINE   ((HCERTCHAINENGINE)0x2)

//
// Create a certificate chain engine.
//

//
// Configuration parameters for the certificate chain engine
//
//      hRestrictedRoot - restrict the root store (must be a subset of "Root")
//
//      hRestrictedTrust - restrict the store for CTLs
//
//      hRestrictedOther - restrict the store for certs and CRLs
//
//      cAdditionalStore, rghAdditionalStore - additional stores
//
//      hExclusiveRoot - the root store to be used exclusively.
//                       If not NULL, then the restricted  stores
//                       the system "Root" and "TrustedPeople" are not used
//
//      hExclusiveTrustedPeople - the trusted people store to be used exclusively.
//                       If not NULL, then the restricted  stores
//                       the system "Root" and "TrustedPeople" are not used
//
//      NOTE:
//
//        (hExclusiveRoot, hExclusiveTrustedPeople) are mutually exclusive
//        with (hRestrictedRoot, hRestrictedTrust, hRestrictedOther).
//        If either hExclusiveRoot or hExclusiveTrustedPeople are used,
//        then all restricted handles must be NULL and non of the system
//        "Root" and "TrustedPeople" are used.
//
//      The algorithm used to define the stores for the engine is as
//            follows:
//
//          If NULL!=hExclusiveRoot or NULL!=hExclusiveTrustedPeople
//              hRoot = hExclusiveRoot
//
//              hTrust = hWorld (defined later)
//
//              hOther = hWorld
//
//              hWorld = hRoot + hExclusiveTrustedPeople + "CA" + "My" + rghAdditionalStore
//
//          Else
//              hRoot = hRestrictedRoot or System Store "Root"
//
//              hTrust = hRestrictedTrust or hWorld (defined later)
//
//              hOther = hRestrictedOther or (hRestrictedTrust == NULL) ? hWorld :
//                       hRestrictedTrust + hWorld
//
//              hWorld = hRoot + "CA" + "My" + "Trust" + rghAdditionalStore
//          Endif
//
//      dwFlags  - flags
//
//          CERT_CHAIN_CACHE_END_CERT - information will be cached on
//                                      the end cert as well as the other
//                                      certs in the chain
//
//          CERT_CHAIN_THREAD_STORE_SYNC - use separate thread for store syncs
//                                         and related cache updates
//
//          CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL - don't hit the wire to get
//                                                URL based objects
//
//      dwUrlRetrievalTimeout - timeout for wire based URL object retrievals
//                              (milliseconds)
//

#define CERT_CHAIN_CACHE_END_CERT                           0x00000001
#define CERT_CHAIN_THREAD_STORE_SYNC                        0x00000002
#define CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL                 0x00000004
#define CERT_CHAIN_USE_LOCAL_MACHINE_STORE                  0x00000008
#define CERT_CHAIN_ENABLE_CACHE_AUTO_UPDATE                 0x00000010
#define CERT_CHAIN_ENABLE_SHARE_STORE                       0x00000020

// Following CertGetCertificateChain dwFlag can also be set on the 
// chain engine flags
// #define CERT_CHAIN_DISABLE_AIA                      0x00002000

typedef struct _CERT_CHAIN_ENGINE_CONFIG {

    DWORD       cbSize;
    HCERTSTORE  hRestrictedRoot;
    HCERTSTORE  hRestrictedTrust;
    HCERTSTORE  hRestrictedOther;
    DWORD       cAdditionalStore;
    HCERTSTORE* rghAdditionalStore;
    DWORD       dwFlags;
    DWORD       dwUrlRetrievalTimeout;      // milliseconds
    DWORD       MaximumCachedCertificates;
    DWORD       CycleDetectionModulus;

#if (NTDDI_VERSION >= NTDDI_WIN7)
    HCERTSTORE  hExclusiveRoot;
    HCERTSTORE  hExclusiveTrustedPeople;
#endif

#if (NTDDI_VERSION >= NTDDI_WIN8)
    DWORD       dwExclusiveFlags;
#endif

} CERT_CHAIN_ENGINE_CONFIG, *PCERT_CHAIN_ENGINE_CONFIG;

#if (NTDDI_VERSION >= NTDDI_WIN8)
//
// dwExclusiveFlags
//

// CA certificates in hExclusiveRoot are also trusted. Chain building
// can terminate in a trusted CA certificate.
#define CERT_CHAIN_EXCLUSIVE_ENABLE_CA_FLAG                 0x00000001
#endif

WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertCreateCertificateChainEngine (
    _In_ PCERT_CHAIN_ENGINE_CONFIG pConfig,
    _Out_ HCERTCHAINENGINE* phChainEngine
    );

//
// Free a certificate trust engine
//

WINCRYPT32API
VOID
WINAPI
CertFreeCertificateChainEngine (
    _In_opt_ HCERTCHAINENGINE hChainEngine
    );

//
// Resync the certificate chain engine.  This resync's the stores backing
// the engine and updates the engine caches.
//

WINCRYPT32API
BOOL
WINAPI
CertResyncCertificateChainEngine (
    _In_opt_ HCERTCHAINENGINE hChainEngine
    );

//
// When an application requests a certificate chain, the data structure
// returned is in the form of a CERT_CHAIN_CONTEXT.  This contains
// an array of CERT_SIMPLE_CHAIN where each simple chain goes from
// an end cert to a self signed cert and the chain context connects simple
// chains via trust lists.  Each simple chain contains the chain of
// certificates, summary trust information about the chain and trust information
// about each certificate element in the chain.
//

//
// Trust status bits
//

typedef struct _CERT_TRUST_STATUS {

    DWORD dwErrorStatus;
    DWORD dwInfoStatus;

} CERT_TRUST_STATUS, *PCERT_TRUST_STATUS;

//
// The following are error status bits
//

// These can be applied to certificates and chains

#define CERT_TRUST_NO_ERROR                             0x00000000
#define CERT_TRUST_IS_NOT_TIME_VALID                    0x00000001
#define CERT_TRUST_IS_NOT_TIME_NESTED                   0x00000002
#define CERT_TRUST_IS_REVOKED                           0x00000004
#define CERT_TRUST_IS_NOT_SIGNATURE_VALID               0x00000008
#define CERT_TRUST_IS_NOT_VALID_FOR_USAGE               0x00000010
#define CERT_TRUST_IS_UNTRUSTED_ROOT                    0x00000020
#define CERT_TRUST_REVOCATION_STATUS_UNKNOWN            0x00000040
#define CERT_TRUST_IS_CYCLIC                            0x00000080

#define CERT_TRUST_INVALID_EXTENSION                    0x00000100
#define CERT_TRUST_INVALID_POLICY_CONSTRAINTS           0x00000200
#define CERT_TRUST_INVALID_BASIC_CONSTRAINTS            0x00000400
#define CERT_TRUST_INVALID_NAME_CONSTRAINTS             0x00000800
#define CERT_TRUST_HAS_NOT_SUPPORTED_NAME_CONSTRAINT    0x00001000

// In LH, this error will never be set.
#define CERT_TRUST_HAS_NOT_DEFINED_NAME_CONSTRAINT      0x00002000

#define CERT_TRUST_HAS_NOT_PERMITTED_NAME_CONSTRAINT    0x00004000
#define CERT_TRUST_HAS_EXCLUDED_NAME_CONSTRAINT         0x00008000

#define CERT_TRUST_IS_OFFLINE_REVOCATION                0x01000000
#define CERT_TRUST_NO_ISSUANCE_CHAIN_POLICY             0x02000000
#define CERT_TRUST_IS_EXPLICIT_DISTRUST                 0x04000000
#define CERT_TRUST_HAS_NOT_SUPPORTED_CRITICAL_EXT       0x08000000
#define CERT_TRUST_HAS_WEAK_SIGNATURE                   0x00100000
#define CERT_TRUST_HAS_WEAK_HYGIENE                     0x00200000
#define CERT_TRUST_HAS_MIN_TELEMETRY_RSA                0x00400000
#define CERT_TRUST_HAS_MIN_WEAK_RSA                     0x00800000

// These can be applied to chains only

#define CERT_TRUST_IS_PARTIAL_CHAIN                     0x00010000
#define CERT_TRUST_CTL_IS_NOT_TIME_VALID                0x00020000
#define CERT_TRUST_CTL_IS_NOT_SIGNATURE_VALID           0x00040000
#define CERT_TRUST_CTL_IS_NOT_VALID_FOR_USAGE           0x00080000

//
// The following are info status bits
//

// These can be applied to certificates only

#define CERT_TRUST_HAS_EXACT_MATCH_ISSUER               0x00000001
#define CERT_TRUST_HAS_KEY_MATCH_ISSUER                 0x00000002
#define CERT_TRUST_HAS_NAME_MATCH_ISSUER                0x00000004
#define CERT_TRUST_IS_SELF_SIGNED                       0x00000008
#define CERT_TRUST_AUTO_UPDATE_CA_REVOCATION            0x00000010
#define CERT_TRUST_AUTO_UPDATE_END_REVOCATION           0x00000020
#define CERT_TRUST_NO_OCSP_FAILOVER_TO_CRL              0x00000040
#define CERT_TRUST_IS_KEY_ROLLOVER                      0x00000080
#define CERT_TRUST_SSL_HANDSHAKE_OCSP                   0x00040000
#define CERT_TRUST_SSL_TIME_VALID_OCSP                  0x00080000
#define CERT_TRUST_SSL_RECONNECT_OCSP                   0x00100000

// These can be applied to certificates and chains

#define CERT_TRUST_HAS_PREFERRED_ISSUER                 0x00000100
#define CERT_TRUST_HAS_ISSUANCE_CHAIN_POLICY            0x00000200
#define CERT_TRUST_HAS_VALID_NAME_CONSTRAINTS           0x00000400
#define CERT_TRUST_IS_PEER_TRUSTED                      0x00000800
#define CERT_TRUST_HAS_CRL_VALIDITY_EXTENDED            0x00001000

// Indicates that the certificate was found in
// a store specified by hExclusiveRoot or hExclusiveTrustedPeople
#define CERT_TRUST_IS_FROM_EXCLUSIVE_TRUST_STORE        0x00002000

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define CERT_TRUST_IS_CA_TRUSTED                        0x00004000
#define CERT_TRUST_HAS_AUTO_UPDATE_WEAK_SIGNATURE       0x00008000
#define CERT_TRUST_HAS_ALLOW_WEAK_SIGNATURE             0x00020000

// Following is set if the input time is before the
// DISALLOWED_CA_FILETIME.
#define CERT_TRUST_BEFORE_DISALLOWED_CA_FILETIME        0x00200000
#endif

// These can be applied to chains only

#define CERT_TRUST_IS_COMPLEX_CHAIN                     0x00010000
#define CERT_TRUST_SSL_TIME_VALID                       0x01000000
#define CERT_TRUST_NO_TIME_CHECK                        0x02000000


//
// Each certificate context in a simple chain has a corresponding chain element
// in the simple chain context
//
// dwErrorStatus has CERT_TRUST_IS_REVOKED, pRevocationInfo set
// dwErrorStatus has CERT_TRUST_REVOCATION_STATUS_UNKNOWN, pRevocationInfo set

//
//         Note that the post processing revocation supported in the first
//         version only sets cbSize and dwRevocationResult.  Everything else
//         is NULL
//

//
// Revocation Information
//

typedef struct _CERT_REVOCATION_INFO {

    DWORD                       cbSize;
    DWORD                       dwRevocationResult;
    LPCSTR                      pszRevocationOid;
    LPVOID                      pvOidSpecificInfo;

    // fHasFreshnessTime is only set if we are able to retrieve revocation
    // information. For a CRL its CurrentTime - ThisUpdate.
    BOOL                        fHasFreshnessTime;
    DWORD                       dwFreshnessTime;    // seconds

    // NonNULL for CRL base revocation checking
    PCERT_REVOCATION_CRL_INFO   pCrlInfo;

} CERT_REVOCATION_INFO, *PCERT_REVOCATION_INFO;

//
// Trust List Information
//

typedef struct _CERT_TRUST_LIST_INFO {

    DWORD         cbSize;
    PCTL_ENTRY    pCtlEntry;
    PCCTL_CONTEXT pCtlContext;

} CERT_TRUST_LIST_INFO, *PCERT_TRUST_LIST_INFO;

//
// Chain Element
//

typedef struct _CERT_CHAIN_ELEMENT {

    DWORD                 cbSize;
    PCCERT_CONTEXT        pCertContext;
    CERT_TRUST_STATUS     TrustStatus;
    PCERT_REVOCATION_INFO pRevocationInfo;

    PCERT_ENHKEY_USAGE    pIssuanceUsage;       // If NULL, any
    PCERT_ENHKEY_USAGE    pApplicationUsage;    // If NULL, any

    LPCWSTR               pwszExtendedErrorInfo;    // If NULL, none
} CERT_CHAIN_ELEMENT, *PCERT_CHAIN_ELEMENT;
typedef const CERT_CHAIN_ELEMENT* PCCERT_CHAIN_ELEMENT;

//
// The simple chain is an array of chain elements and a summary trust status
// for the chain
//
// rgpElements[0] is the end certificate chain element
//
// rgpElements[cElement-1] is the self-signed "root" certificate chain element
//

typedef struct _CERT_SIMPLE_CHAIN {

    DWORD                 cbSize;
    CERT_TRUST_STATUS     TrustStatus;
    DWORD                 cElement;
    PCERT_CHAIN_ELEMENT*  rgpElement;
    PCERT_TRUST_LIST_INFO pTrustListInfo;

    // fHasRevocationFreshnessTime is only set if we are able to retrieve
    // revocation information for all elements checked for revocation.
    // For a CRL its CurrentTime - ThisUpdate.
    //
    // dwRevocationFreshnessTime is the largest time across all elements
    // checked.
    BOOL                   fHasRevocationFreshnessTime;
    DWORD                  dwRevocationFreshnessTime;    // seconds

} CERT_SIMPLE_CHAIN, *PCERT_SIMPLE_CHAIN;
typedef const CERT_SIMPLE_CHAIN* PCCERT_SIMPLE_CHAIN;

//
// And the chain context contains an array of simple chains and summary trust
// status for all the connected simple chains
//
// rgpChains[0] is the end certificate simple chain
//
// rgpChains[cChain-1] is the final (possibly trust list signer) chain which
// ends in a certificate which is contained in the root store
//

typedef struct _CERT_CHAIN_CONTEXT CERT_CHAIN_CONTEXT, *PCERT_CHAIN_CONTEXT;
typedef const CERT_CHAIN_CONTEXT *PCCERT_CHAIN_CONTEXT;

struct _CERT_CHAIN_CONTEXT {
    DWORD                   cbSize;
    CERT_TRUST_STATUS       TrustStatus;
    DWORD                   cChain;
    PCERT_SIMPLE_CHAIN*     rgpChain;

    // Following is returned when CERT_CHAIN_RETURN_LOWER_QUALITY_CONTEXTS
    // is set in dwFlags
    DWORD                   cLowerQualityChainContext;
    PCCERT_CHAIN_CONTEXT*   rgpLowerQualityChainContext;

    // fHasRevocationFreshnessTime is only set if we are able to retrieve
    // revocation information for all elements checked for revocation.
    // For a CRL its CurrentTime - ThisUpdate.
    //
    // dwRevocationFreshnessTime is the largest time across all elements
    // checked.
    BOOL                    fHasRevocationFreshnessTime;
    DWORD                   dwRevocationFreshnessTime;    // seconds

    // Flags passed when created via CertGetCertificateChain
    DWORD                   dwCreateFlags;

    // Following is updated with unique Id when the chain context is logged.
    GUID                    ChainId;
};


//
// When building a chain, the there are various parameters used for finding
// issuing certificates and trust lists.  They are identified in the
// following structure
//

// Default usage match type is AND with value zero
#define USAGE_MATCH_TYPE_AND 0x00000000
#define USAGE_MATCH_TYPE_OR  0x00000001

typedef struct _CERT_USAGE_MATCH {

    DWORD             dwType;
    CERT_ENHKEY_USAGE Usage;

} CERT_USAGE_MATCH, *PCERT_USAGE_MATCH;

typedef struct _CTL_USAGE_MATCH {

    DWORD     dwType;
    CTL_USAGE Usage;

} CTL_USAGE_MATCH, *PCTL_USAGE_MATCH;

typedef struct _CERT_CHAIN_PARA {

    DWORD            cbSize;
    CERT_USAGE_MATCH RequestedUsage;

#ifdef CERT_CHAIN_PARA_HAS_EXTRA_FIELDS

    // Note, if you #define CERT_CHAIN_PARA_HAS_EXTRA_FIELDS, then, you
    // must zero all unused fields in this data structure.
    // More fields could be added in a future release.

    CERT_USAGE_MATCH RequestedIssuancePolicy;
    DWORD            dwUrlRetrievalTimeout;     // milliseconds
    BOOL             fCheckRevocationFreshnessTime;
    DWORD            dwRevocationFreshnessTime; // seconds

    // If nonNULL, any cached information before this time is considered
    // time invalid and forces a wire retrieval. When set overrides
    // the registry configuration CacheResync time.
    LPFILETIME                  pftCacheResync;

    //
    // The following is set to check for Strong Signatures
    //
    PCCERT_STRONG_SIGN_PARA pStrongSignPara;

    //
    // By default the public key in the end certificate is checked.
    // CERT_CHAIN_STRONG_SIGN_DISABLE_END_CHECK_FLAG can be
    // set in the following flags to not check if the end certificate's public
    // key length is strong.
    //
    DWORD                   dwStrongSignFlags;

#endif

} CERT_CHAIN_PARA, *PCERT_CHAIN_PARA;

#define CERT_CHAIN_STRONG_SIGN_DISABLE_END_CHECK_FLAG   0x00000001

//
// The following API is used for retrieving certificate chains
//
// Parameters:
//
//      hChainEngine     - the chain engine (namespace and cache) to use, NULL
//                         mean use the default chain engine
//
//      pCertContext     - the context we are retrieving the chain for, it
//                         will be the zero index element in the chain
//
//      pTime            - the point in time that we want the chain validated
//                         for.  Note that the time does not affect trust list,
//                         revocation, or root store checking.  NULL means use
//                         the current system time
//
//      hAdditionalStore - additional store to use when looking up objects
//
//      pChainPara       - parameters for chain building
//
//      dwFlags          - flags such as should revocation checking be done
//                         on the chain?
//
//      pvReserved       - reserved parameter, must be NULL
//
//      ppChainContext   - chain context returned
//

// CERT_CHAIN_CACHE_END_CERT can be used here as well
// Revocation flags are in the high nibble
#define CERT_CHAIN_REVOCATION_CHECK_END_CERT           0x10000000
#define CERT_CHAIN_REVOCATION_CHECK_CHAIN              0x20000000
#define CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT 0x40000000
#define CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY         0x80000000

// By default, the dwUrlRetrievalTimeout in pChainPara is the timeout used
// for each revocation URL wire retrieval. When the following flag is set,
// dwUrlRetrievalTimeout is the accumulative timeout across all
// revocation URL wire retrievals.
#define CERT_CHAIN_REVOCATION_ACCUMULATIVE_TIMEOUT     0x08000000


// Revocation checking for an independent OCSP signer certificate.
//
// The above revocation flags indicate if just the signer certificate or all
// the certificates in the chain, excluding the root should be checked
// for revocation. If the signer certificate contains the
// szOID_PKIX_OCSP_NOCHECK extension, then, revocation checking is skipped
// for the leaf signer certificate. Both OCSP and CRL checking are allowed.
// However, recursive, independent OCSP signer certs are disabled.
#define CERT_CHAIN_REVOCATION_CHECK_OCSP_CERT          0x04000000


// First pass determines highest quality based upon:
//  - Chain signature valid (higest quality bit of this set)
//  - Complete chain
//  - Trusted root          (lowestest quality bit of this set)
// By default, second pass only considers paths >= highest first pass quality
#define CERT_CHAIN_DISABLE_PASS1_QUALITY_FILTERING  0x00000040

#define CERT_CHAIN_RETURN_LOWER_QUALITY_CONTEXTS    0x00000080

#define CERT_CHAIN_DISABLE_AUTH_ROOT_AUTO_UPDATE    0x00000100


// When this flag is set, pTime will be used as the timestamp time.
// pTime will be used to determine if the end certificate was valid at this
// time. Revocation checking will be relative to pTime.
// In addition, current time will also be used
// to determine if the certificate is still time valid. All remaining
// CA and root certificates will be checked using current time and not pTime.
//
// This flag was added 4/5/01 in WXP.
#define CERT_CHAIN_TIMESTAMP_TIME                   0x00000200


// When this flag is set, "My" certificates having a private key or end
// entity certificates in the "TrustedPeople" store are trusted without
// doing any chain building. Neither the CERT_TRUST_IS_PARTIAL_CHAIN or
// CERT_TRUST_IS_UNTRUSTED_ROOT dwErrorStatus bits will be set for
// such certificates.
//
// This flag was added 6/9/03 in LH.
#define CERT_CHAIN_ENABLE_PEER_TRUST                0x00000400

// When this flag is set, "My" certificates aren't considered for
// PEER_TRUST.
//
// This flag was added 11/12/04 in LH.
//
// On 8-05-05 changed to never consider "My" certificates for PEER_TRUST.
#define CERT_CHAIN_DISABLE_MY_PEER_TRUST            0x00000800


// The following flag should be set to explicitly disable MD2 or MD4 for
// any requested EKU. By default, MD2 or MD4 isn't disabled for none,
// code signing, driver signing or time stamping requested EKUs.
#define CERT_CHAIN_DISABLE_MD2_MD4                  0x00001000

// The following flag can be set to explicitly disable AIA retrievals.
// If can also be set in the chain engine dwFlags.
#define CERT_CHAIN_DISABLE_AIA                      0x00002000

// The following flag should be set when verifying the certificate
// associated with a file having the Mark-Of-The-Web
#define CERT_CHAIN_HAS_MOTW                         0x00004000

// Only use certificates from the Additional and AuthRoot stores.
// If disabled, AuthRoot trust is enabled for this call.
#define CERT_CHAIN_ONLY_ADDITIONAL_AND_AUTH_ROOT    0x00008000

// The following flag should be set when the caller is prepared
// for opt-in weak signature errors. Should support an user
// option to click through. First for SHA1. In the future
// for RSA < 2048 bits.
#define CERT_CHAIN_OPT_IN_WEAK_SIGNATURE            0x00010000

// The following flag should be set when the caller is prepared
// to match the returned chain context elements when
// CERT_TRUST_BEFORE_DISALLOWED_CA_FILETIME is set in the
// dwInfoStatus.
#define CERT_CHAIN_ENABLE_DISALLOWED_CA             0x00020000

WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertGetCertificateChain (
    _In_opt_ HCERTCHAINENGINE hChainEngine,
    _In_ PCCERT_CONTEXT pCertContext,
    _In_opt_ LPFILETIME pTime,
    _In_opt_ HCERTSTORE hAdditionalStore,
    _In_ PCERT_CHAIN_PARA pChainPara,
    _In_ DWORD dwFlags,
    _Reserved_ LPVOID pvReserved,
    _Out_ PCCERT_CHAIN_CONTEXT* ppChainContext
    );

//
// Free a certificate chain
//

WINCRYPT32API
VOID
WINAPI
CertFreeCertificateChain (
    _In_ PCCERT_CHAIN_CONTEXT pChainContext
    );

//
// Duplicate (add a reference to) a certificate chain
//

WINCRYPT32API
PCCERT_CHAIN_CONTEXT
WINAPI
CertDuplicateCertificateChain (
    _In_ PCCERT_CHAIN_CONTEXT pChainContext
    );

//+-------------------------------------------------------------------------
//  This data structure is optionally pointed to by the pChainPara field
//  in the CERT_REVOCATION_PARA and CRYPT_GET_TIME_VALID_OBJECT_EXTRA_INFO
//  data structures. CertGetCertificateChain() populates when it calls
//  the CertVerifyRevocation() API.
//--------------------------------------------------------------------------
struct _CERT_REVOCATION_CHAIN_PARA {
    DWORD                       cbSize;
    HCERTCHAINENGINE            hChainEngine;
    HCERTSTORE                  hAdditionalStore;
    DWORD                       dwChainFlags;
    DWORD                       dwUrlRetrievalTimeout;     // milliseconds
    LPFILETIME                  pftCurrentTime;
    LPFILETIME                  pftCacheResync;

    // Max size of the URL object to download, in bytes.
    // 0 value means no limit.
    DWORD                       cbMaxUrlRetrievalByteCount;
};

//
// Specific Revocation Type OID and structure definitions
//

//
// CRL Revocation OID
//

#define REVOCATION_OID_CRL_REVOCATION ((LPCSTR)1)

//
// For the CRL revocation OID the pvRevocationPara is NULL
//

//
// CRL Revocation Info
//

typedef struct _CRL_REVOCATION_INFO {

    PCRL_ENTRY           pCrlEntry;
    PCCRL_CONTEXT        pCrlContext;
    PCCERT_CHAIN_CONTEXT pCrlIssuerChain;

} CRL_REVOCATION_INFO, *PCRL_REVOCATION_INFO;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+-------------------------------------------------------------------------
//  Find the first or next certificate chain context in the store.
//
//  The chain context is found according to the dwFindFlags, dwFindType and
//  its pvFindPara. See below for a list of the find types and its parameters.
//
//  If the first or next chain context isn't found, NULL is returned.
//  Otherwise, a pointer to a read only CERT_CHAIN_CONTEXT is returned.
//  CERT_CHAIN_CONTEXT must be freed by calling CertFreeCertificateChain
//  or is freed when passed as the
//  pPrevChainContext on a subsequent call. CertDuplicateCertificateChain
//  can be called to make a duplicate.
//
//  pPrevChainContext MUST BE NULL on the first
//  call to find the chain context. To find the next chain context, the
//  pPrevChainContext is set to the CERT_CHAIN_CONTEXT returned by a previous
//  call.
//
//  NOTE: a NON-NULL pPrevChainContext is always CertFreeCertificateChain'ed by
//  this function, even for an error.
//--------------------------------------------------------------------------
WINCRYPT32API
PCCERT_CHAIN_CONTEXT
WINAPI
CertFindChainInStore(
    _In_ HCERTSTORE hCertStore,
    _In_ DWORD dwCertEncodingType,
    _In_ DWORD dwFindFlags,
    _In_ DWORD dwFindType,
    _In_opt_ const void *pvFindPara,
    _In_opt_ PCCERT_CHAIN_CONTEXT pPrevChainContext
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#define CERT_CHAIN_FIND_BY_ISSUER       1

//+-------------------------------------------------------------------------
//  CERT_CHAIN_FIND_BY_ISSUER
//
//  Find a certificate chain having a private key for the end certificate and
//  matching one of the given issuer names. A matching dwKeySpec and
//  enhanced key usage can also be specified. Additionally a callback can
//  be provided for even more caller provided filtering before building the
//  chain.
//
//  By default, only the issuers in the first simple chain are compared
//  for a name match. CERT_CHAIN_FIND_BY_ISSUER_COMPLEX_CHAIN_FLAG can
//  be set in dwFindFlags to match issuers in all the simple chains.
//
//  CERT_CHAIN_FIND_BY_ISSUER_NO_KEY_FLAG can be set in dwFindFlags to
//  not check if the end certificate has a private key.
//
//  CERT_CHAIN_FIND_BY_ISSUER_COMPARE_KEY_FLAG can be set in dwFindFlags
//  to compare the public key in the end certificate with the crypto
//  provider's public key. The dwAcquirePrivateKeyFlags can be set
//  in CERT_CHAIN_FIND_BY_ISSUER_PARA to enable caching of the private key's
//  HKEY returned by the CSP.
//
//  If dwCertEncodingType == 0, defaults to X509_ASN_ENCODING for the
//  array of encoded issuer names.
//
//  By default, the hCertStore passed to CertFindChainInStore, is passed
//  as an additional store to CertGetCertificateChain.
//  CERT_CHAIN_FIND_BY_ISSUER_CACHE_ONLY_FLAG can be set in dwFindFlags
//  to improve performance by only searching the cached system stores
//  (root, my, ca, trust) to find the issuer certificates. If you are doing
//  a find in the "my" system store, than, this flag should be set to
//  improve performance.
//
//  Setting CERT_CHAIN_FIND_BY_ISSUER_LOCAL_MACHINE_FLAG in dwFindFlags
//  restricts CertGetCertificateChain to search the Local Machine
//  cached system stores instead of the Current User's.
//
//  Setting CERT_CHAIN_FIND_BY_ISSUER_CACHE_ONLY_URL_FLAG in dwFindFlags
//  restricts CertGetCertificateChain to only search the URL cache
//  and not hit the wire.
//--------------------------------------------------------------------------

// Returns FALSE to skip this certificate. Otherwise, returns TRUE to
// build a chain for this certificate.
typedef BOOL (WINAPI *PFN_CERT_CHAIN_FIND_BY_ISSUER_CALLBACK)(
    _In_ PCCERT_CONTEXT pCert,
    _Inout_opt_ void *pvFindArg
    );

typedef struct _CERT_CHAIN_FIND_BY_ISSUER_PARA {
    DWORD                                   cbSize;

    // If pszUsageIdentifier == NULL, matches any usage.
    LPCSTR                                  pszUsageIdentifier;

    // If dwKeySpec == 0, matches any KeySpec
    DWORD                                   dwKeySpec;

    // When CERT_CHAIN_FIND_BY_ISSUER_COMPARE_KEY_FLAG is set in dwFindFlags,
    // CryptAcquireCertificatePrivateKey is called to do the public key
    // comparison. The following flags can be set to enable caching
    // of the acquired private key or suppress CSP UI. See the API for more
    // details on these flags.
    DWORD                                   dwAcquirePrivateKeyFlags;

    // Pointer to an array of X509, ASN.1 encoded issuer name blobs. If
    // cIssuer == 0, matches any issuer
    DWORD                                   cIssuer;
    CERT_NAME_BLOB                          *rgIssuer;

    // If NULL or Callback returns TRUE, builds the chain for the end
    // certificate having a private key with the specified KeySpec and
    // enhanced key usage.
    PFN_CERT_CHAIN_FIND_BY_ISSUER_CALLBACK pfnFindCallback;
    void                                    *pvFindArg;

#ifdef CERT_CHAIN_FIND_BY_ISSUER_PARA_HAS_EXTRA_FIELDS
    // Note, if you #define CERT_CHAIN_FIND_BY_ISSUER_PARA_HAS_EXTRA_FIELDS,
    // then, you must zero all unused fields in this data structure.
    // More fields could be added in a future release.

    // If the following pointers are nonNull, returns the index of the
    // matching issuer certificate, which is at:
    // pChainContext->
    //      rgpChain[*pdwIssuerChainIndex]->rgpElement[*pdwIssuerElementIndex].
    //
    // The issuer name blob is compared against the Issuer field in the
    // certificate. The *pdwIssuerElementIndex is set to the index of this
    // subject certificate + 1. Therefore, its possible for a partial chain or
    // a self signed certificate matching the name blob, where
    // *pdwIssuerElementIndex points past the last certificate in the chain.
    //
    // Note, not updated if the above cIssuer == 0.
    DWORD                                   *pdwIssuerChainIndex;
    DWORD                                   *pdwIssuerElementIndex;
#endif
} CERT_CHAIN_FIND_ISSUER_PARA, *PCERT_CHAIN_FIND_ISSUER_PARA,
    CERT_CHAIN_FIND_BY_ISSUER_PARA, *PCERT_CHAIN_FIND_BY_ISSUER_PARA;

// The following dwFindFlags can be set for CERT_CHAIN_FIND_BY_ISSUER

// If set, compares the public key in the end certificate with the crypto
// provider's public key. This comparison is the last check made on the
// build chain.
#define CERT_CHAIN_FIND_BY_ISSUER_COMPARE_KEY_FLAG          0x0001

// If not set, only checks the first simple chain for an issuer name match.
// When set, also checks second and subsequent simple chains.
#define CERT_CHAIN_FIND_BY_ISSUER_COMPLEX_CHAIN_FLAG        0x0002

// If set, CertGetCertificateChain only searches the URL cache and
// doesn't hit the wire.
#define CERT_CHAIN_FIND_BY_ISSUER_CACHE_ONLY_URL_FLAG       0x0004

// If set, CertGetCertificateChain only opens the Local Machine
// certificate stores instead of the Current User's.
#define CERT_CHAIN_FIND_BY_ISSUER_LOCAL_MACHINE_FLAG        0x0008

// If set, no check is made to see if the end certificate has a private
// key associated with it.
#define CERT_CHAIN_FIND_BY_ISSUER_NO_KEY_FLAG               0x4000


// By default, the hCertStore passed to CertFindChainInStore, is passed
// as the additional store to CertGetCertificateChain. This flag can be
// set to improve performance by only searching the cached system stores
// (root, my, ca, trust) to find the issuer certificates. If not set, then,
// the hCertStore is always searched in addition to the cached system
// stores.
#define CERT_CHAIN_FIND_BY_ISSUER_CACHE_ONLY_FLAG           0x8000


//+=========================================================================
//  Certificate Chain Policy Data Structures and APIs
//==========================================================================
typedef struct _CERT_CHAIN_POLICY_PARA {
    DWORD                   cbSize;
    DWORD                   dwFlags;
    void                    *pvExtraPolicyPara;     // pszPolicyOID specific
} CERT_CHAIN_POLICY_PARA, *PCERT_CHAIN_POLICY_PARA;

// If both lChainIndex and lElementIndex are set to -1, the dwError applies
// to the whole chain context. If only lElementIndex is set to -1, the
// dwError applies to the lChainIndex'ed chain. Otherwise, the dwError applies
// to the certificate element at
// pChainContext->rgpChain[lChainIndex]->rgpElement[lElementIndex].
typedef struct _CERT_CHAIN_POLICY_STATUS {
    DWORD                   cbSize;
    DWORD                   dwError;
    LONG                    lChainIndex;
    LONG                    lElementIndex;
    void                    *pvExtraPolicyStatus;   // pszPolicyOID specific
} CERT_CHAIN_POLICY_STATUS, *PCERT_CHAIN_POLICY_STATUS;

// Common chain policy flags
#define CERT_CHAIN_POLICY_IGNORE_NOT_TIME_VALID_FLAG                0x00000001
#define CERT_CHAIN_POLICY_IGNORE_CTL_NOT_TIME_VALID_FLAG            0x00000002
#define CERT_CHAIN_POLICY_IGNORE_NOT_TIME_NESTED_FLAG               0x00000004
#define CERT_CHAIN_POLICY_IGNORE_INVALID_BASIC_CONSTRAINTS_FLAG     0x00000008

#define CERT_CHAIN_POLICY_IGNORE_ALL_NOT_TIME_VALID_FLAGS ( \
    CERT_CHAIN_POLICY_IGNORE_NOT_TIME_VALID_FLAG                | \
    CERT_CHAIN_POLICY_IGNORE_CTL_NOT_TIME_VALID_FLAG            | \
    CERT_CHAIN_POLICY_IGNORE_NOT_TIME_NESTED_FLAG                 \
    )


#define CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG                     0x00000010
#define CERT_CHAIN_POLICY_IGNORE_WRONG_USAGE_FLAG                   0x00000020
#define CERT_CHAIN_POLICY_IGNORE_INVALID_NAME_FLAG                  0x00000040
#define CERT_CHAIN_POLICY_IGNORE_INVALID_POLICY_FLAG                0x00000080

#define CERT_CHAIN_POLICY_IGNORE_END_REV_UNKNOWN_FLAG               0x00000100
#define CERT_CHAIN_POLICY_IGNORE_CTL_SIGNER_REV_UNKNOWN_FLAG        0x00000200
#define CERT_CHAIN_POLICY_IGNORE_CA_REV_UNKNOWN_FLAG                0x00000400
#define CERT_CHAIN_POLICY_IGNORE_ROOT_REV_UNKNOWN_FLAG              0x00000800

#define CERT_CHAIN_POLICY_IGNORE_ALL_REV_UNKNOWN_FLAGS ( \
    CERT_CHAIN_POLICY_IGNORE_END_REV_UNKNOWN_FLAG         | \
    CERT_CHAIN_POLICY_IGNORE_CTL_SIGNER_REV_UNKNOWN_FLAG  | \
    CERT_CHAIN_POLICY_IGNORE_CA_REV_UNKNOWN_FLAG          | \
    CERT_CHAIN_POLICY_IGNORE_ROOT_REV_UNKNOWN_FLAG          \
    )

#define CERT_CHAIN_POLICY_ALLOW_TESTROOT_FLAG                       0x00008000
#define CERT_CHAIN_POLICY_TRUST_TESTROOT_FLAG                       0x00004000

#define CERT_CHAIN_POLICY_IGNORE_NOT_SUPPORTED_CRITICAL_EXT_FLAG    0x00002000
#define CERT_CHAIN_POLICY_IGNORE_PEER_TRUST_FLAG                    0x00001000
#define CERT_CHAIN_POLICY_IGNORE_WEAK_SIGNATURE_FLAG                0x08000000


//+-------------------------------------------------------------------------
//  Verify that the certificate chain satisfies the specified policy
//  requirements. If we were able to verify the chain policy, TRUE is returned
//  and the dwError field of the pPolicyStatus is updated. A dwError of 0
//  (ERROR_SUCCESS, S_OK) indicates the chain satisfies the specified policy.
//
//  If dwError applies to the entire chain context, both lChainIndex and
//  lElementIndex are set to -1. If dwError applies to a simple chain,
//  lElementIndex is set to -1 and lChainIndex is set to the index of the
//  first offending chain having the error. If dwError applies to a
//  certificate element, lChainIndex and lElementIndex are updated to
//  index the first offending certificate having the error, where, the
//  the certificate element is at:
//      pChainContext->rgpChain[lChainIndex]->rgpElement[lElementIndex].
//
//  The dwFlags in pPolicyPara can be set to change the default policy checking
//  behaviour. In addition, policy specific parameters can be passed in
//  the pvExtraPolicyPara field of pPolicyPara.
//
//  In addition to returning dwError, in pPolicyStatus, policy OID specific
//  extra status may be returned via pvExtraPolicyStatus.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
CertVerifyCertificateChainPolicy(
    _In_ LPCSTR pszPolicyOID,
    _In_ PCCERT_CHAIN_CONTEXT pChainContext,
    _In_ PCERT_CHAIN_POLICY_PARA pPolicyPara,
    _Inout_ PCERT_CHAIN_POLICY_STATUS pPolicyStatus
    );

// Predefined OID Function Names
#define CRYPT_OID_VERIFY_CERTIFICATE_CHAIN_POLICY_FUNC  \
    "CertDllVerifyCertificateChainPolicy"

// CertDllVerifyCertificateChainPolicy has same function signature as
// CertVerifyCertificateChainPolicy.

//+-------------------------------------------------------------------------
//  Predefined verify chain policies
//--------------------------------------------------------------------------
#define CERT_CHAIN_POLICY_BASE              ((LPCSTR) 1)
#define CERT_CHAIN_POLICY_AUTHENTICODE      ((LPCSTR) 2)
#define CERT_CHAIN_POLICY_AUTHENTICODE_TS   ((LPCSTR) 3)
#define CERT_CHAIN_POLICY_SSL               ((LPCSTR) 4)
#define CERT_CHAIN_POLICY_BASIC_CONSTRAINTS ((LPCSTR) 5)
#define CERT_CHAIN_POLICY_NT_AUTH           ((LPCSTR) 6)
#define CERT_CHAIN_POLICY_MICROSOFT_ROOT    ((LPCSTR) 7)
#define CERT_CHAIN_POLICY_EV                ((LPCSTR) 8)
#define CERT_CHAIN_POLICY_SSL_F12           ((LPCSTR) 9)
#define CERT_CHAIN_POLICY_SSL_HPKP_HEADER   ((LPCSTR) 10)
#define CERT_CHAIN_POLICY_THIRD_PARTY_ROOT  ((LPCSTR) 11)
#define CERT_CHAIN_POLICY_SSL_KEY_PIN       ((LPCSTR) 12)
#define CERT_CHAIN_POLICY_CT                ((LPCSTR) 13)

//+-------------------------------------------------------------------------
//  CERT_CHAIN_POLICY_BASE
//
//  Implements the base chain policy verification checks. dwFlags can
//  be set in pPolicyPara to alter the default policy checking behaviour.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_CHAIN_POLICY_AUTHENTICODE
//
//  Implements the Authenticode chain policy verification checks.
//
//  pvExtraPolicyPara may optionally be set to point to the following
//  AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA.
//
//  pvExtraPolicyStatus may optionally be set to point to the following
//  AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS.
//--------------------------------------------------------------------------

// dwRegPolicySettings are defined in wintrust.h
typedef struct _AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA {
    DWORD               cbSize;
    DWORD               dwRegPolicySettings;
    PCMSG_SIGNER_INFO   pSignerInfo;                // optional
} AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA,
    *PAUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_PARA;

typedef struct _AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS {
    DWORD               cbSize;
    BOOL                fCommercial;        // obtained from signer statement
} AUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS,
    *PAUTHENTICODE_EXTRA_CERT_CHAIN_POLICY_STATUS;

//+-------------------------------------------------------------------------
//  CERT_CHAIN_POLICY_AUTHENTICODE_TS
//
//  Implements the Authenticode Time Stamp chain policy verification checks.
//
//  pvExtraPolicyPara may optionally be set to point to the following
//  AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA.
//
//  pvExtraPolicyStatus isn't used and must be set to NULL.
//--------------------------------------------------------------------------

// dwRegPolicySettings are defined in wintrust.h
typedef struct _AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA {
    DWORD               cbSize;
    DWORD               dwRegPolicySettings;
    BOOL                fCommercial;
} AUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA,
    *PAUTHENTICODE_TS_EXTRA_CERT_CHAIN_POLICY_PARA;


//+-------------------------------------------------------------------------
//  CERT_CHAIN_POLICY_SSL
//
//  Implements the SSL client/server chain policy verification checks.
//
//  pvExtraPolicyPara may optionally be set to point to the following
//  SSL_EXTRA_CERT_CHAIN_POLICY_PARA data structure
//--------------------------------------------------------------------------

// fdwChecks flags are defined in wininet.h
typedef struct _HTTPSPolicyCallbackData
{
    union {
        DWORD           cbStruct;       // sizeof(HTTPSPolicyCallbackData);
        DWORD           cbSize;         // sizeof(HTTPSPolicyCallbackData);
    } DUMMYUNIONNAME;

    DWORD           dwAuthType;
#                       define      AUTHTYPE_CLIENT         1
#                       define      AUTHTYPE_SERVER         2

    DWORD           fdwChecks;

    WCHAR           *pwszServerName; // used to check against CN=xxxx

} HTTPSPolicyCallbackData, *PHTTPSPolicyCallbackData,
    SSL_EXTRA_CERT_CHAIN_POLICY_PARA, *PSSL_EXTRA_CERT_CHAIN_POLICY_PARA;

//+-------------------------------------------------------------------------
//  CERT_CHAIN_POLICY_BASIC_CONSTRAINTS
//
//  Implements the basic constraints chain policy.
//
//  Iterates through all the certificates in the chain checking for either
//  a szOID_BASIC_CONSTRAINTS or a szOID_BASIC_CONSTRAINTS2 extension. If
//  neither extension is present, the certificate is assumed to have
//  valid policy. Otherwise, for the first certificate element, checks if
//  it matches the expected CA_FLAG or END_ENTITY_FLAG specified in
//  pPolicyPara->dwFlags. If neither or both flags are set, then, the first
//  element can be either a CA or END_ENTITY. All other elements must be
//  a CA. If the PathLenConstraint is present in the extension, its
//  checked.
//
//  The first elements in the remaining simple chains (ie, the certificate
//  used to sign the CTL) are checked to be an END_ENTITY.
//
//  If this verification fails, dwError will be set to
//  TRUST_E_BASIC_CONSTRAINTS.
//--------------------------------------------------------------------------

#define BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_CA_FLAG         0x80000000
#define BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_END_ENTITY_FLAG 0x40000000

//+-------------------------------------------------------------------------
//  CERT_CHAIN_POLICY_NT_AUTH
//
//  Implements the NT Authentication chain policy.
//
//  The NT Authentication chain policy consists of 3 distinct chain
//  verifications in the following order:
//      [1] CERT_CHAIN_POLICY_BASE - Implements the base chain policy
//          verification checks. The LOWORD of dwFlags can be set in
//          pPolicyPara to alter the default policy checking behaviour. See
//          CERT_CHAIN_POLICY_BASE for more details.
//
//      [2] CERT_CHAIN_POLICY_BASIC_CONSTRAINTS - Implements the basic
//          constraints chain policy. The HIWORD of dwFlags can be set
//          to specify if the first element must be either a CA or END_ENTITY.
//          See CERT_CHAIN_POLICY_BASIC_CONSTRAINTS for more details.
//
//      [3] Checks if the second element in the chain, the CA that issued
//          the end certificate, is a trusted CA for NT
//          Authentication. A CA is considered to be trusted if it exists in
//          the "NTAuth" system registry store found in the
//          CERT_SYSTEM_STORE_LOCAL_MACHINE_ENTERPRISE store location.
//          If this verification fails, whereby the CA isn't trusted,
//          dwError is set to CERT_E_UNTRUSTEDCA.
//
//          If CERT_PROT_ROOT_DISABLE_NT_AUTH_REQUIRED_FLAG is set
//          in the "Flags" value of the HKLM policy "ProtectedRoots" subkey
//          defined by CERT_PROT_ROOT_FLAGS_REGPATH, then,
//          if the above check fails, checks if the chain
//          has CERT_TRUST_HAS_VALID_NAME_CONSTRAINTS set in dwInfoStatus. This
//          will only be set if there was a valid name constraint for all
//          name spaces including UPN. If the chain doesn't have this info
//          status set, dwError is set to CERT_E_UNTRUSTEDCA.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_CHAIN_POLICY_MICROSOFT_ROOT
//
//  Checks if the last element of the first simple chain contains a
//  Microsoft root public key. If it doesn't contain a Microsoft root
//  public key, dwError is set to CERT_E_UNTRUSTEDROOT.
//
//  pPolicyPara is optional. However,
//  MICROSOFT_ROOT_CERT_CHAIN_POLICY_ENABLE_TEST_ROOT_FLAG can be set in
//  the dwFlags in pPolicyPara to also check for the Microsoft Test Roots.
//
//  MICROSOFT_ROOT_CERT_CHAIN_POLICY_CHECK_APPLICATION_ROOT_FLAG can be set
//  in the dwFlags in pPolicyPara to check for the Microsoft root for
//  application signing instead of the Microsoft product root. This flag
//  explicitly checks for the application root only and cannot be combined
//  with the test root flag.  
//
//  MICROSOFT_ROOT_CERT_CHAIN_POLICY_DISABLE_FLIGHT_ROOT_FLAG can be set
//  in the dwFlags in pPolicyPara to always disable the Flight root.
//
//  pvExtraPolicyPara and pvExtraPolicyStatus aren't used and must be set
//  to NULL.
//--------------------------------------------------------------------------
#define MICROSOFT_ROOT_CERT_CHAIN_POLICY_ENABLE_TEST_ROOT_FLAG       0x00010000
#define MICROSOFT_ROOT_CERT_CHAIN_POLICY_CHECK_APPLICATION_ROOT_FLAG 0x00020000
#define MICROSOFT_ROOT_CERT_CHAIN_POLICY_DISABLE_FLIGHT_ROOT_FLAG    0x00040000

//+-------------------------------------------------------------------------
//  CERT_CHAIN_POLICY_EV
//
//  Verify the issuance policy in the end certificate of the first simple
//  chain matches with the root certificate EV policy.
//
//  pvExtraPolicyPara may optionally be set to point to the following
//  EV_EXTRA_CERT_CHAIN_POLICY_PARA. The dwRootProgramQualifierFlags member
//  can be set to one or more of the CERT_ROOT_PROGRAM_FLAG_* to define
//  which of the EV policy qualifier bits are required for validation.
//
//  pvExtraPolicyStatus may optionally be set to point to the following
//  EV_EXTRA_CERT_CHAIN_POLICY_STATUS. The fQualifiers member will contain
//  a combination of CERT_ROOT_PROGRAM_FLAG_* flags.
//--------------------------------------------------------------------------

typedef struct _EV_EXTRA_CERT_CHAIN_POLICY_PARA {
    DWORD               cbSize;
    DWORD               dwRootProgramQualifierFlags;
} EV_EXTRA_CERT_CHAIN_POLICY_PARA,
    *PEV_EXTRA_CERT_CHAIN_POLICY_PARA;

typedef struct _EV_EXTRA_CERT_CHAIN_POLICY_STATUS {
    DWORD   cbSize;
    DWORD   dwQualifiers;
    DWORD   dwIssuanceUsageIndex;
} EV_EXTRA_CERT_CHAIN_POLICY_STATUS, *PEV_EXTRA_CERT_CHAIN_POLICY_STATUS;

//+-------------------------------------------------------------------------
//  CERT_CHAIN_POLICY_SSL_F12
//
//  Checks if any certificates in the chain have weak crypto to
//  change the default https lock and provide an F12 error string.
//
//  For a Third Party root, checks if it is in compliance with the Microsoft
//  Root Program requirements. (Will be implemented in RS2.)
//
//  pvExtraPolicyStatus must point to the following
//  SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS. It will be updated with the
//  results of the weak crypto and root program compliance checks.
//  Before calling, the cbSize must be set to a
//  value >= sizeof(SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS).
//
//  dwError in CERT_CHAIN_POLICY_STATUS will be set to TRUST_E_CERT_SIGNATURE
//  for potential weak crypto and set to CERT_E_UNTRUSTEDROOT for Third Party
//  Roots not in compliance with the Microsoft Root Program.
//--------------------------------------------------------------------------
#define SSL_F12_ERROR_TEXT_LENGTH   256
typedef struct _SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS {
    DWORD   cbSize;
    DWORD   dwErrorLevel;
    DWORD   dwErrorCategory;
    DWORD   dwReserved;
    WCHAR   wszErrorText[SSL_F12_ERROR_TEXT_LENGTH];  // Localized
} SSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS, *PSSL_F12_EXTRA_CERT_CHAIN_POLICY_STATUS;

//+-------------------------------------------------------------------------
//  SSL_F12 Error Levels
//--------------------------------------------------------------------------
#define CERT_CHAIN_POLICY_SSL_F12_SUCCESS_LEVEL         0
#define CERT_CHAIN_POLICY_SSL_F12_WARNING_LEVEL         1
#define CERT_CHAIN_POLICY_SSL_F12_ERROR_LEVEL           2

//+-------------------------------------------------------------------------
//  SSL_F12 Error Categories
//--------------------------------------------------------------------------
#define CERT_CHAIN_POLICY_SSL_F12_NONE_CATEGORY         0
#define CERT_CHAIN_POLICY_SSL_F12_WEAK_CRYPTO_CATEGORY  1
#define CERT_CHAIN_POLICY_SSL_F12_ROOT_PROGRAM_CATEGORY 2

// Error Level for CERT_CHAIN_POLICY_SSL_F12_WEAK_CRYPTO_CATEGORY:
//  - CERT_CHAIN_POLICY_SSL_F12_ERROR_LEVEL
//     -- Third Party Root
//  - CERT_CHAIN_POLICY_SSL_F12_WARNING_LEVEL
//     -- All other roots including enterprise

// Error Level for CERT_CHAIN_POLICY_SSL_F12_ROOT_PROGRAM_CATEGORY:
//  - CERT_CHAIN_POLICY_SSL_F12_WARNING_LEVEL
//     -- All Root Program compliance failures will map to warning level


//+-------------------------------------------------------------------------
//  CERT_CHAIN_POLICY_SSL_HPKP_HEADER
//
//  Processes the Http Public Key Pinning (HPKP) responses headers.
//  There are two possible response headers:
//   - "Public-Key-Pins" (PKP_HEADER)
//   - "Public-Key-Pins-Report-Only" (PKP_RO_HEADER)
//
//  One or both of the above header values must be present.
//
//  pvExtraPolicyPara must be set to point to the following
//  SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA
//
//  One of the following dwError's will be set if the HPKP header isn't
//  used:
//   ERROR_SERVICE_DISABLED
//      HPKP has been explicitly disabled
//   ERROR_NOT_FOUND
//      No previous call using CERT_CHAIN_POLICY_SSL policy for chain and
//      server name.
//   ERROR_ALREADY_EXISTS
//      Second add to same server within 10 minutes
//   ERROR_NOT_SUPPORTED
//      Only the "Public-Key-Pins-Report-Only" header was set. It will
//      only be trace logged.
//   CRYPT_E_NO_MATCH
//      No public key match in the chain context being used.
//   CERT_E_UNTRUSTEDROOT
//      Didn't chain up to a Third Party Root.
//   ERROR_INVALID_TIME
//      max-age value was less than the supported minimum. Default is 7 days.
//   ERROR_INVALID_DATA
//      HPKP header parsing errors.
//--------------------------------------------------------------------------

// "Public-Key-Pins" and "Public-Key-Pins-Report-Only" header indices
#define SSL_HPKP_PKP_HEADER_INDEX       0
#define SSL_HPKP_PKP_RO_HEADER_INDEX    1
#define SSL_HPKP_HEADER_COUNT           2

typedef struct _SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA {
    DWORD               cbSize;
    DWORD               dwReserved;
    LPWSTR              pwszServerName;

    // One or both of the following must be nonNULL.
    LPSTR               rgpszHpkpValue[SSL_HPKP_HEADER_COUNT];
} SSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA,
    *PSSL_HPKP_HEADER_EXTRA_CERT_CHAIN_POLICY_PARA;

//+-------------------------------------------------------------------------
//  CERT_CHAIN_POLICY_THIRD_PARTY_ROOT
//
//  Checks if the last element of the first simple chain is
//  a Third Party root. If it isn't dwError is set to CERT_E_UNTRUSTEDROOT.
//
//
//  pvExtraPolicyPara and pvExtraPolicyStatus aren't used and must be set
//  to NULL.
//--------------------------------------------------------------------------

//+-------------------------------------------------------------------------
//  CERT_CHAIN_POLICY_SSL_KEY_PIN
//
//  Uses the machine's non-expired HPKP rules to check for
//  SSL server certificate Key Pin matches.
//
//  Also uses the Microsoft Windows Update Pin Rules to check if a potential
//  MiTM root was used.
//
//  pvExtraPolicyPara must point to the 
//  SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA data structure.
//
//  pvExtraPolicyStatus must point to the
//  SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS data structure.
//
//  lError will be updated as follows:
//   = 0 - SUCCESS. No MiTM or mismatch errors
//   < 0 - ERROR.   User should be prompted with a click through option
//   > 0 - WARNING. Only F12 warning
//
//  Two types of errors:
//   - MITM -     Server certificates didn't chain up to a third party root
//                 ERROR -   Current User or Local Machine root
//                 WARNING - Group Policy or Enterprise root
//   - MISMATCH - Server certificates chained up to a third party root
//                 ERROR -   Only domain mismatches
//                 WARNING - Both a domain mismatch and a domain match
//
//  For any errors wszErrorText will be updated with localized error string
//  to be included in F12.
//
//  Before calling, the cbSize must be set to a
//  value >= sizeof(SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS).
//
//  dwError in CERT_CHAIN_POLICY_STATUS will be set to CERT_E_CN_NO_MATCH
//  for either MITM or MISMATCH error.
//--------------------------------------------------------------------------

typedef struct _SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA {
    DWORD   cbSize;
    DWORD   dwReserved;
    PCWSTR  pwszServerName;
} SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA, *PSSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_PARA;

#define SSL_KEY_PIN_ERROR_TEXT_LENGTH   512
typedef struct _SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS {
    DWORD   cbSize;
    LONG    lError;
    WCHAR   wszErrorText[SSL_KEY_PIN_ERROR_TEXT_LENGTH];  // Localized
} SSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS, *PSSL_KEY_PIN_EXTRA_CERT_CHAIN_POLICY_STATUS;


//+-------------------------------------------------------------------------
//  SSL_KEY_PIN Errors
//--------------------------------------------------------------------------
#define CERT_CHAIN_POLICY_SSL_KEY_PIN_MISMATCH_ERROR   -2
#define CERT_CHAIN_POLICY_SSL_KEY_PIN_MITM_ERROR       -1
#define CERT_CHAIN_POLICY_SSL_KEY_PIN_SUCCESS           0
#define CERT_CHAIN_POLICY_SSL_KEY_PIN_MITM_WARNING      1
#define CERT_CHAIN_POLICY_SSL_KEY_PIN_MISMATCH_WARNING  2

//+-------------------------------------------------------------------------
//  CERT_CHAIN_POLICY_CT 
//
//  Implements the Certificate Transparency chain policy verification checks.
//
//  pvExtraPolicyStatus must be set to point to the following
//  CT_EXTRA_CHAIN_POLICY_STATUS data structure
//--------------------------------------------------------------------------
typedef struct _CT_EXTRA_CERT_CHAIN_POLICY_STATUS {
    DWORD   cbSize;
    LONG    lErrorStatus;
    LONG    lErrorSubStatus;
    DWORD   cEntries;
    DWORD   cValidated;
} CT_EXTRA_CERT_CHAIN_POLICY_STATUS, *PCT_EXTRA_CERT_CHAIN_POLICY_STATUS;

//+-------------------------------------------------------------------------
//  Certificate Transparency Errors
//
//  CertVerifyCertificateChainPolicy will return a CERT_CHAIN_POLICY_STATUS structure 
//  which holds the certificate chain status information from when the certificate chains are validated. 
//  The dwError in the returned CERT_CHAIN_POLICY_STATUS will be set to one of three Windows 
//  API return code values: 
//  S_OK, ERROR_SUCCESS, 0 - Indicates the leaf certificate has a verified SCT extension. 
//  E_FAIL - Indicates the leaf certificate doesn't have an SCT extension or is unable to verify. 
//  S_FALSE - Same as E_FAIL, however, a non-fatal reason for not finding or being able to verify. 
//  For example, code signing certificates that were issued before CA's started adding SCT extension to code signing certificates. 
//  The pvExtraPolicyStatus in the CERT_CHAIN_POLICY_STATUS will point to a struct containing more detailed extra error information. 
//  The struct will contain the following two fields: 
//  LONG lErrorStatus; 
//      > 0 : warning - These errors can be ignored. Will map to S_FALSE 
//      == 0 : success - Will map to S_OK 
//      < 0 : fatal error - These errors shouldn't be ignored. Will map to E_FAIL 
//  LONG lErrorSubStatus; 
//  - Depends on lErrorStatus. Can override lErrorStatus. 
//--------------------------------------------------------------------------
#define CERT_CHAIN_POLICY_CT_ERROR_UNDECODABLE_SCT_EXTENSION               -112
#define CERT_CHAIN_POLICY_CT_ERROR_UNRETRIEVABLE_SCT_EXTENSION             -111
#define CERT_CHAIN_POLICY_CT_ERROR_MISSING_SCT_EXTENSION                   -110

#define CERT_CHAIN_POLICY_CT_ERROR_INVALID_ISSUER_CERT                     -101
#define CERT_CHAIN_POLICY_CT_ERROR_INVALID_SUBJECT_CERT                    -100

#define CERT_CHAIN_POLICY_CT_ERROR_CANNOT_VALIDATE_SCT                      -50

// Validation Statuses
#define CERT_CHAIN_POLICY_CT_ERROR_SCT_VALIDATION_STATUS_INSUFFICIENT       -4
#define CERT_CHAIN_POLICY_CT_ERROR_SCT_VALIDATION_STATUS_UNKNOWN_VERSION    -3
#define CERT_CHAIN_POLICY_CT_ERROR_SCT_VALIDATION_STATUS_UNKNOWN_LOG        -2
#define CERT_CHAIN_POLICY_CT_ERROR_SCT_VALIDATION_STATUS_INVALID            -1
#define CERT_CHAIN_POLICY_CT_SUCCESS_SCT_VALIDIDATION_STATUS_VALID           0

// Override Errors
#define CERT_CHAIN_POLICY_CT_WARNING_OUT_OF_MEMORY                           1
#define CERT_CHAIN_POLICY_CT_WARNING_BEFORE_CODE_SIGNING_CT_LOGGING          2
#define CERT_CHAIN_POLICY_CT_WARNING_NOT_THIRD_PARTY_CERT                    3
#define CERT_CHAIN_POLICY_CT_WARNING_EXPIRED_ROOT_CTL                        4

// Early Failure Warnings
#define CERT_CHAIN_POLICY_CT_WARNING_INVALID_CHAIN_CONTEXT                   50
#define CERT_CHAIN_POLICY_CT_WARNING_NOT_SUPPORTED_CA                        51
#define CERT_CHAIN_POLICY_CT_WARNING_MISSING_ROOT_CTL                        52

#define CERT_CHAIN_POLICY_CT_WARNING_MISSING_CT_EXT                          60
#define CERT_CHAIN_POLICY_CT_WARNING_INVALID_CT_EXT                          61
#define CERT_CHAIN_POLICY_CT_WARNING_UNABLE_TO_DECODE_EXT                    62

#define CERT_CHAIN_POLICY_CT_WARNING_UNABLE_TO_DECODE_PARAMETERS             70

#define CERT_CHAIN_POLICY_CT_WARNING_INVALID_TEMP_FILE                       80
#define CERT_CHAIN_POLICY_CT_WARNING_CANNOT_CREATE_TEMP_FILE                 81
#define CERT_CHAIN_POLICY_CT_WARNING_CANNOT_WRITE_TEMP_FILE                  82
#define CERT_CHAIN_POLICY_CT_WARNING_CANNOT_LOAD_CTLOG_STORE_FILE            83

#define CERT_CHAIN_POLICY_CT_WARNING_FAILED_INIT                             90

// General Warnings
#define CERT_CHAIN_POLICY_CT_WARNING_HASHING_ERROR                           200
#define CERT_CHAIN_POLICY_CT_WARNING_INVALID_STR                             201

// OpenSSL Warnings
#define CERT_CHAIN_POLICY_CT_WARNING_CANNOT_CREATE_POLICY                    300
//+-------------------------------------------------------------------------
// convert formatted string to binary
// If cchString is 0, then pszString is NULL terminated and
// cchString is obtained via strlen() + 1.
// dwFlags defines string format
// if pbBinary is NULL, *pcbBinary returns the size of required memory
// *pdwSkip returns the character count of skipped strings, optional
// *pdwFlags returns the actual format used in the conversion, optional
//--------------------------------------------------------------------------
_Success_(return)
WINCRYPT32STRINGAPI
BOOL
WINAPI
CryptStringToBinaryA(
    _When_(dwFlags != CRYPT_STRING_BINARY, _In_reads_(cchString))
    _When_(dwFlags == CRYPT_STRING_BINARY, _In_reads_bytes_(cchString))
        LPCSTR pszString,
    _In_ DWORD cchString,
    _In_ DWORD dwFlags,
    _Out_writes_bytes_to_opt_(*pcbBinary, *pcbBinary) BYTE *pbBinary,
    _Inout_ DWORD  *pcbBinary,
    _Out_opt_ DWORD *pdwSkip,
    _Out_opt_ DWORD *pdwFlags
    );
// OpenSSL Warnings
#define CERT_CHAIN_POLICY_CT_WARNING_CANNOT_CREATE_POLICY                    300
//+-------------------------------------------------------------------------
// convert formatted string to binary
// If cchString is 0, then pszString is NULL terminated and
// cchString is obtained via strlen() + 1.
// dwFlags defines string format
// if pbBinary is NULL, *pcbBinary returns the size of required memory
// *pdwSkip returns the character count of skipped strings, optional
// *pdwFlags returns the actual format used in the conversion, optional
//--------------------------------------------------------------------------
_Success_(return)
WINCRYPT32STRINGAPI
BOOL
WINAPI
CryptStringToBinaryW(
    _When_(dwFlags != CRYPT_STRING_BINARY, _In_reads_(cchString))
    _When_(dwFlags == CRYPT_STRING_BINARY, _In_reads_bytes_(cchString))
        LPCWSTR pszString,
    _In_ DWORD cchString,
    _In_ DWORD dwFlags,
    _Out_writes_bytes_to_opt_(*pcbBinary, *pcbBinary) BYTE *pbBinary,
    _Inout_ DWORD  *pcbBinary,
    _Out_opt_ DWORD *pdwSkip,
    _Out_opt_ DWORD *pdwFlags
    );
#ifdef UNICODE
#define CryptStringToBinary  CryptStringToBinaryW
#else
#define CryptStringToBinary  CryptStringToBinaryA
#endif // !UNICODE

//+-------------------------------------------------------------------------
// convert binary to formatted string
// dwFlags defines string format
// if pszString is NULL, *pcchString returns size in characters
// including null-terminator
//--------------------------------------------------------------------------
_Success_(return)
WINCRYPT32STRINGAPI
BOOL
WINAPI
CryptBinaryToStringA(
    _In_reads_bytes_(cbBinary) CONST BYTE *pbBinary,
    _In_ DWORD cbBinary,
    _In_ DWORD dwFlags,
    _Out_writes_to_opt_(*pcchString, *pcchString) LPSTR pszString,
    _Inout_ DWORD *pcchString
    );
//+-------------------------------------------------------------------------
// convert binary to formatted string
// dwFlags defines string format
// if pszString is NULL, *pcchString returns size in characters
// including null-terminator
//--------------------------------------------------------------------------
_Success_(return)
WINCRYPT32STRINGAPI
BOOL
WINAPI
CryptBinaryToStringW(
    _In_reads_bytes_(cbBinary) CONST BYTE *pbBinary,
    _In_ DWORD cbBinary,
    _In_ DWORD dwFlags,
    _Out_writes_to_opt_(*pcchString, *pcchString) LPWSTR pszString,
    _Inout_ DWORD *pcchString
    );
#ifdef UNICODE
#define CryptBinaryToString  CryptBinaryToStringW
#else
#define CryptBinaryToString  CryptBinaryToStringA
#endif // !UNICODE

// dwFlags has the following defines
// certenrolld_begin -- CRYPT_STRING_*
#define CRYPT_STRING_BASE64HEADER           0x00000000
#define CRYPT_STRING_BASE64                 0x00000001
#define CRYPT_STRING_BINARY                 0x00000002
#define CRYPT_STRING_BASE64REQUESTHEADER    0x00000003
#define CRYPT_STRING_HEX                    0x00000004
#define CRYPT_STRING_HEXASCII               0x00000005
#define CRYPT_STRING_BASE64_ANY             0x00000006
#define CRYPT_STRING_ANY                    0x00000007
#define CRYPT_STRING_HEX_ANY                0x00000008
#define CRYPT_STRING_BASE64X509CRLHEADER    0x00000009
#define CRYPT_STRING_HEXADDR                0x0000000a
#define CRYPT_STRING_HEXASCIIADDR           0x0000000b
#define CRYPT_STRING_HEXRAW                 0x0000000c
#define CRYPT_STRING_BASE64URI              0x0000000d

#define CRYPT_STRING_ENCODEMASK             0x000000ff
#define CRYPT_STRING_RESERVED100            0x00000100
#define CRYPT_STRING_RESERVED200            0x00000200

#define CRYPT_STRING_PERCENTESCAPE          0x08000000	// base64 formats only
#define CRYPT_STRING_HASHDATA               0x10000000
#define CRYPT_STRING_STRICT                 0x20000000
#define CRYPT_STRING_NOCRLF                 0x40000000
#define CRYPT_STRING_NOCR                   0x80000000
// certenrolld_end

// CryptBinaryToString uses the following flags
// CRYPT_STRING_BASE64HEADER - base64 format with certificate begin
//                             and end headers
// CRYPT_STRING_BASE64 - only base64 without headers
// CRYPT_STRING_BINARY - pure binary copy
// CRYPT_STRING_BASE64REQUESTHEADER - base64 format with request begin
//                                    and end headers
// CRYPT_STRING_BASE64X509CRLHEADER - base64 format with x509 crl begin
//                                    and end headers
// CRYPT_STRING_HEX - only hex format
// CRYPT_STRING_HEXASCII - hex format with ascii char display
// CRYPT_STRING_HEXADDR - hex format with address display
// CRYPT_STRING_HEXASCIIADDR - hex format with ascii char and address display
//
// CryptBinaryToString accepts CRYPT_STRING_NOCR or'd into one of the above.
// When set, line breaks contain only LF, instead of CR-LF pairs.

// CryptStringToBinary uses the following flags
// CRYPT_STRING_BASE64_ANY tries the following, in order:
//    CRYPT_STRING_BASE64HEADER
//    CRYPT_STRING_BASE64
// CRYPT_STRING_ANY tries the following, in order:
//    CRYPT_STRING_BASE64_ANY
//    CRYPT_STRING_BINARY -- should always succeed
// CRYPT_STRING_HEX_ANY tries the following, in order:
//    CRYPT_STRING_HEXADDR
//    CRYPT_STRING_HEXASCIIADDR
//    CRYPT_STRING_HEXASCII
//    CRYPT_STRING_HEX

//+=========================================================================
//  PFX (PKCS #12) function definitions and types
//==========================================================================

//+-------------------------------------------------------------------------
//  PKCS#12 OIDs
//--------------------------------------------------------------------------

#define szOID_PKCS_12_PbeIds                        "1.2.840.113549.1.12.1"
#define szOID_PKCS_12_pbeWithSHA1And128BitRC4       "1.2.840.113549.1.12.1.1"
#define szOID_PKCS_12_pbeWithSHA1And40BitRC4        "1.2.840.113549.1.12.1.2"
#define szOID_PKCS_12_pbeWithSHA1And3KeyTripleDES   "1.2.840.113549.1.12.1.3"
#define szOID_PKCS_12_pbeWithSHA1And2KeyTripleDES   "1.2.840.113549.1.12.1.4"
#define szOID_PKCS_12_pbeWithSHA1And128BitRC2       "1.2.840.113549.1.12.1.5"
#define szOID_PKCS_12_pbeWithSHA1And40BitRC2        "1.2.840.113549.1.12.1.6"
#define szOID_PKCS_5_PBKDF2                         "1.2.840.113549.1.5.12"
#define szOID_PKCS_5_PBES2                          "1.2.840.113549.1.5.13"

//+-------------------------------------------------------------------------
//  PBE parameters as defined in PKCS#12 as pkcs-12PbeParams.
//
//  NOTE that the salt bytes will immediately follow this structure.
//  we avoid using pointers in this structure for easy of passing
//  it into NCryptExportKey() as a NCryptBuffer (may be sent via RPC
//  to the key isolation process).
//--------------------------------------------------------------------------
typedef struct _CRYPT_PKCS12_PBE_PARAMS
{
    int                 iIterations;        /* iteration count              */
    ULONG               cbSalt;             /* byte size of the salt        */
}
CRYPT_PKCS12_PBE_PARAMS;

//+-------------------------------------------------------------------------
//      PFXImportCertStore
//
//  Import the PFX blob and return a store containing certificates
//
//  If the password parameter is incorrect or any other problems decoding
//  the PFX blob are encountered, the function will return NULL and the
//      error code can be found from GetLastError().
//
//  The dwFlags parameter may be set to the following:
//  PKCS12_IMPORT_SILENT    - only allow importing key in silent mode. If the
//                            csp or ksp requires ui then this call will fail
//                            with the error from the csp or ksp.
//  CRYPT_EXPORTABLE - specify that any imported keys should be marked as
//                     exportable (see documentation on CryptImportKey)
//  CRYPT_USER_PROTECTED - (see documentation on CryptImportKey)
//  CRYPT_MACHINE_KEYSET - used to force the private key to be stored in the
//                        the local machine and not the current user.
//  CRYPT_USER_KEYSET - used to force the private key to be stored in the
//                      the current user and not the local machine, even if
//                      the pfx blob specifies that it should go into local
//                      machine.
//  PKCS12_INCLUDE_EXTENDED_PROPERTIES - used to import all extended
//                     properties that were saved with CertExportCertStore()
//                     using the same flag.
//
//  PKCS12_ONLY_CERTIFICATES - the returned store only contains certificates.
//                             Private keys aren't decrypted or imported.
//                             If the certificates weren't encrypted, then,
//                             we won't use a password to decrypt. Otherwise,
//                             will do normal password decryption.
//                             For certificates having an associated private
//                             key, we add the CERT_KEY_PROV_INFO_PROP_ID.
//                             The KeyProvInfo will have the following special
//                             values:
//                               dwProvType = 0
//                               pwszProvName = L"PfxProvider"
//                               pwszProvName = L"PfxContainer"
//
//                             For not encrypted certificates, we won't use
//                             any password to do the MAC check. If a MAC
//                             check is necessary, then, PKCS12_NO_PERSIST_KEY
//                             option should be selected instead.
//
//  PKCS12_ONLY_NOT_ENCRYPTED_CERTIFICATES - same as for PKCS12_ONLY_CERTIFICATES
//                                           except, we won't fallback to
//                                           using the password to decrypt.
//--------------------------------------------------------------------------
WINCRYPT32API
HCERTSTORE
WINAPI
PFXImportCertStore(
    _In_ CRYPT_DATA_BLOB* pPFX,
    _In_ LPCWSTR szPassword,
    _In_ DWORD   dwFlags);

// dwFlags definitions for PFXImportCertStore
//#define CRYPT_EXPORTABLE          0x00000001  // CryptImportKey dwFlags
//#define CRYPT_USER_PROTECTED      0x00000002  // CryptImportKey dwFlags
//#define CRYPT_MACHINE_KEYSET      0x00000020  // CryptAcquireContext dwFlags
//#define CRYPT_USER_PROTECTED_STRONG 0x00100000
//#define PKCS12_INCLUDE_EXTENDED_PROPERTIES 0x10
#define PKCS12_IMPORT_SILENT        0x00000040
#define CRYPT_USER_KEYSET           0x00001000
#define PKCS12_PREFER_CNG_KSP       0x00000100  // prefer using CNG KSP
#define PKCS12_ALWAYS_CNG_KSP       0x00000200  // always use CNG KSP
#define PKCS12_ONLY_CERTIFICATES    0x00000400
#define PKCS12_ONLY_NOT_ENCRYPTED_CERTIFICATES 0x00000800
#define PKCS12_ALLOW_OVERWRITE_KEY  0x00004000  // allow overwrite existing key
#define PKCS12_NO_PERSIST_KEY       0x00008000  // key will not be persisted
#define PKCS12_VIRTUAL_ISOLATION_KEY 0x00010000  // key will be saved into VSM
#define PKCS12_IMPORT_RESERVED_MASK 0xffff0000
#define PKCS12_NAMED_NO_PERSIST_KEY 0x00020000  // PKCS12_NO_PERSIST_KEY and PKCS12_ALWAYS_CNG_KSP also need to be set

#define PKCS12_OBJECT_LOCATOR_ALL_IMPORT_FLAGS          \
              ( PKCS12_ALWAYS_CNG_KSP               |   \
                PKCS12_NO_PERSIST_KEY               |   \
                PKCS12_IMPORT_SILENT                |   \
                PKCS12_INCLUDE_EXTENDED_PROPERTIES  )

#define PKCS12_ONLY_CERTIFICATES_PROVIDER_TYPE  0
#define PKCS12_ONLY_CERTIFICATES_PROVIDER_NAME  L"PfxProvider"
#define PKCS12_ONLY_CERTIFICATES_CONTAINER_NAME L"PfxContainer"

//+-------------------------------------------------------------------------
//      PFXIsPFXBlob
//
//  This function will try to decode the outer layer of the blob as a pfx
//  blob, and if that works it will return TRUE, it will return FALSE otherwise
//
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
PFXIsPFXBlob(
    _In_ CRYPT_DATA_BLOB* pPFX);

//+-------------------------------------------------------------------------
//      PFXVerifyPassword
//
//  This function will attempt to decode the outer layer of the blob as a pfx
//  blob and decrypt with the given password. No data from the blob will be
//  imported.
//
//  Return value is TRUE if password appears correct, FALSE otherwise.
//
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
PFXVerifyPassword(
    _In_ CRYPT_DATA_BLOB* pPFX,
    _In_ LPCWSTR szPassword,
    _In_ DWORD dwFlags);

//+-------------------------------------------------------------------------
//      PFXExportCertStoreEx
//
//  Export the certificates and private keys referenced in the passed-in store
//
//  This API encodes the blob under a stronger algorithm. The resulting
//  PKCS12 blobs are incompatible with the earlier PFXExportCertStore API.
//
//  The value passed in the password parameter will be used to encrypt and
//  verify the integrity of the PFX packet. If any problems encoding the store
//  are encountered, the function will return FALSE and the error code can
//  be found from GetLastError().
//
//  The PKCS12_PROTECT_TO_DOMAIN_SIDS flag together with an 
//  NCRYPT_DESCRIPTOR_HANDLE* for pvPara means the password will be stored
//  in the pfx protected to the NCRYPT_DESCRIPTOR_HANDLE. On import, any
//  principal that is listed in NCRYPT_DESCRIPTOR_HANDLE can decrypt the 
//  password within the pfx and use it to descrypt the entire pfx.
//  
//  If the password parameter is NULL or L"" and the 
//  PKCS12_PROTECT_TO_DOMAIN_SIDS flag is set together with an
//  NCRYPT_DESCRIPTOR_HANDLE* for pvPara then a random password of length
//  40 characters is chosen to protect the pfx. This password will be 
//  protected inside the pfx.
//
//  If the certificates don't need to be private, such as, the PFX is
//  hosted on a file share accessed by IIS, then,
//  the PKCS12_DISABLE_ENCRYPT_CERTIFICATES flag should be set.
//
//  Note, OpenSSL and down level platforms support certificates that weren't
//  encrypted.
//
//  In Threshold the default was changed not to encrypt the certificates.
//  The following registry value can be set to change the default to enable
//  the encryption.
//      HKLM\Software\Microsoft\Windows\CurrentVersion\PFX
//          REG_DWORD EncryptCertificates
//
//  The PKCS12_ENCRYPT_CERTIFICATES flag should be set to always
//  encrypt the certificates.
//
//  The dwFlags parameter may be set to any combination of
//      EXPORT_PRIVATE_KEYS
//      REPORT_NO_PRIVATE_KEY
//      REPORT_NOT_ABLE_TO_EXPORT_PRIVATE_KEY
//      PKCS12_EXPORT_SILENT
//      PKCS12_INCLUDE_EXTENDED_PROPERTIES
//      PKCS12_PROTECT_TO_DOMAIN_SIDS
//      PKCS12_DISABLE_ENCRYPT_CERTIFICATES or PKCS12_ENCRYPT_CERTIFICATES
//      PKCS12_EXPORT_ECC_CURVE_PARAMETERS
//      PKCS12_EXPORT_ECC_CURVE_OID
//
//  The encoded PFX blob is returned in *pPFX. If pPFX->pbData is NULL upon
//  input, this is a length only calculation, whereby, pPFX->cbData is updated
//  with the number of bytes required for the encoded blob. Otherwise,
//  the memory pointed to by pPFX->pbData is updated with the encoded bytes
//  and pPFX->cbData is updated with the encoded byte length.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
PFXExportCertStoreEx(
    _In_ HCERTSTORE hStore,
    _Inout_ CRYPT_DATA_BLOB* pPFX,
    _In_ LPCWSTR szPassword,
    _In_ void* pvPara,
    _In_ DWORD dwFlags);


// dwFlags definitions for PFXExportCertStoreEx
#define REPORT_NO_PRIVATE_KEY                   0x0001
#define REPORT_NOT_ABLE_TO_EXPORT_PRIVATE_KEY   0x0002
#define EXPORT_PRIVATE_KEYS                     0x0004
#define PKCS12_INCLUDE_EXTENDED_PROPERTIES      0x0010
#define PKCS12_PROTECT_TO_DOMAIN_SIDS           0x0020
#define PKCS12_EXPORT_SILENT                    0x0040
#define PKCS12_EXPORT_PBES2_PARAMS              0x0080 
#define PKCS12_DISABLE_ENCRYPT_CERTIFICATES     0x0100
#define PKCS12_ENCRYPT_CERTIFICATES             0x0200
#define PKCS12_EXPORT_ECC_CURVE_PARAMETERS      0x1000
#define PKCS12_EXPORT_ECC_CURVE_OID             0x2000
#define PKCS12_EXPORT_RESERVED_MASK             0xffff0000

#define PKCS12_PBKDF2_ID_HMAC_SHA1     "1.2.840.113549.2.7"
#define PKCS12_PBKDF2_ID_HMAC_SHA256   "1.2.840.113549.2.9"
#define PKCS12_PBKDF2_ID_HMAC_SHA384   "1.2.840.113549.2.10"
#define PKCS12_PBKDF2_ID_HMAC_SHA512   "1.2.840.113549.2.11"

//
// PKCS12 Pbes2 Parameter Structure
//     It is passed into PFXExportCertStoreEx as pvPara 
//     when PKCS12_EXPORT_PBES2_PARAMS is set for dwFlags.
//
typedef struct _PKCS12_PBES2_EXPORT_PARAMS 
{ 
    DWORD dwSize;            // structure size of _PKCS12_PBES2_EXPORT_PARAMS    
    PVOID hNcryptDescriptor;
    LPWSTR pwszPbes2Alg; 
} PKCS12_PBES2_EXPORT_PARAMS, *PPKCS12_PBES2_EXPORT_PARAMS; 

//
// PKCS12 Pbes2 Algorithm string definition
//     This string is passed into pwszPbes2Alg of the structure
//     PKCS12_PBES2_EXPORT_PARAMS, which indicates which
//     algorithms will be used for key and certificate encryption,
//     and MacData hashing.
//

// AES256 will be used for key/certificate encryption, and
// SHA256 will be used for KDF2, and MacData hashing.
#define PKCS12_PBES2_ALG_AES256_SHA256  L"AES256-SHA256"

// Registry path to the PFX configuration local machine subkey
#define PKCS12_CONFIG_REGPATH        \
    L"Software\\Microsoft\\Windows\\CurrentVersion\\PFX"

// The default is not to encrypt the certificates included in the PFX.
// The following is a REG_DWORD. It should be set to a nonzero value
// to change the default to enable encrypting the certificates.
#define PKCS12_ENCRYPT_CERTIFICATES_VALUE_NAME \
    L"EncryptCertificates"

//+-------------------------------------------------------------------------
//      PFXExportCertStore
//
//  Export the certificates and private keys referenced in the passed-in store
//
//  This is an old API kept for compatibility with IE4 clients. New applications
//  should call the above PfxExportCertStoreEx for enhanced security.
//--------------------------------------------------------------------------
WINCRYPT32API
BOOL
WINAPI
PFXExportCertStore(
    _In_ HCERTSTORE hStore,
    _Inout_ CRYPT_DATA_BLOB* pPFX,
    _In_ LPCWSTR szPassword,
    _In_ DWORD dwFlags);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//+=========================================================================
//  APIs to get a non-blocking, time valid OCSP response for
//  a server certificate chain.
//
//  Normally, this OCSP response will be included along with the server
//  certificate in a message returned to the client. As a result only the
//  server should need to contact the OCSP responser for its certificate.
//==========================================================================
#if (NTDDI_VERSION >= NTDDI_VISTA)

//+-------------------------------------------------------------------------
//  Server OCSP response handle.
//--------------------------------------------------------------------------
typedef VOID *HCERT_SERVER_OCSP_RESPONSE;

//+-------------------------------------------------------------------------
//  Server OCSP response context.
//--------------------------------------------------------------------------
typedef struct _CERT_SERVER_OCSP_RESPONSE_CONTEXT
    CERT_SERVER_OCSP_RESPONSE_CONTEXT,
    *PCERT_SERVER_OCSP_RESPONSE_CONTEXT;
typedef const CERT_SERVER_OCSP_RESPONSE_CONTEXT
    *PCCERT_SERVER_OCSP_RESPONSE_CONTEXT;

struct _CERT_SERVER_OCSP_RESPONSE_CONTEXT {
    DWORD       cbSize;
    BYTE        *pbEncodedOcspResponse;
    DWORD       cbEncodedOcspResponse;
};

//+-------------------------------------------------------------------------
//  Server OCSP response update callback
//
//  If CERT_SERVER_OCSP_RESPONSE_OPEN_PARA_WRITE_FLAG has been enabled
//  then dwWriteOcspFileError will be set. Otherwise, always set to 0.
//--------------------------------------------------------------------------
typedef VOID (CALLBACK *PFN_CERT_SERVER_OCSP_RESPONSE_UPDATE_CALLBACK)(
    _In_ PCCERT_CHAIN_CONTEXT pChainContext,
    _In_ PCCERT_SERVER_OCSP_RESPONSE_CONTEXT pServerOcspResponseContext,
    _In_ PCCRL_CONTEXT pNewCrlContext,
    _In_opt_ PCCRL_CONTEXT pPrevCrlContext,
    _Inout_opt_ PVOID pvArg,
    _In_ DWORD dwWriteOcspFileError
    );


//+-------------------------------------------------------------------------
//  Server OCSP response open parameters
//--------------------------------------------------------------------------
typedef struct _CERT_SERVER_OCSP_RESPONSE_OPEN_PARA {
    DWORD                                           cbSize;
    DWORD                                           dwFlags;

    // If nonNULL, *pcbUsedSize is updated with subset of cbSize that was
    // used. If OPEN_PARA isn't supported, then, *pcbUsedSize won't be
    // updated.
    DWORD                                           *pcbUsedSize;

    // If nonNULL, the OCSP response is either read from or written to
    // this directory. The CERT_SERVER_OCSP_RESPONSE_OPEN_PARA_READ_FLAG
    // dwFlags must be set to read.
    // The CERT_SERVER_OCSP_RESPONSE_OPEN_PARA_WRITE_FLAG dwFlags must be
    // set to write. Its an ERROR_INVALID_PARAMETER error to set both dwFlags.
    //
    // The format of the OCSP response file name:
    // <ASCII HEX ServerCert SHA1 Thumbprint>".ocsp"
    PWSTR                                           pwszOcspDirectory;

    // If nonNULL, the callback is called whenever the OCSP response is
    // updated. Note, the updated OCSP response might not be time valid.
    PFN_CERT_SERVER_OCSP_RESPONSE_UPDATE_CALLBACK   pfnUpdateCallback;
    PVOID                                           pvUpdateCallbackArg;
} CERT_SERVER_OCSP_RESPONSE_OPEN_PARA, *PCERT_SERVER_OCSP_RESPONSE_OPEN_PARA;


// Set either of these flags in the above dwFlags to use the
// pwszOcspDirectory.
#define CERT_SERVER_OCSP_RESPONSE_OPEN_PARA_READ_FLAG       0x00000001
#define CERT_SERVER_OCSP_RESPONSE_OPEN_PARA_WRITE_FLAG      0x00000002

//+-------------------------------------------------------------------------
//  Open a handle to an OCSP response associated with a server certificate
//  chain. If the end certificate doesn't have an OCSP AIA URL, NULL is
//  returned with LastError set to CRYPT_E_NOT_IN_REVOCATION_DATABASE. NULL
//  will also be returned if unable to allocate memory or create system
//  objects.
//
//  This API will try to retrieve an initial OCSP response before returning.
//  This API will block during the retrieval. If unable to successfully
//  retrieve the first OCSP response, a non-NULL handle will still be returned
//  if not one of the error cases mentioned above.
//
//  The CERT_SERVER_OCSP_RESPONSE_ASYNC_FLAG flag can be set to
//  return immediately without making the initial synchronous retrieval.
//
//  A background thread is created that will pre-fetch time valid
//  OCSP responses.
//
//  The input chain context will be AddRef'ed and not freed until
//  the returned handle is closed.
//
//  CertCloseServerOcspResponse() must be called to close the returned
//  handle.
//--------------------------------------------------------------------------
WINCRYPT32API
HCERT_SERVER_OCSP_RESPONSE
WINAPI
CertOpenServerOcspResponse(
    _In_ PCCERT_CHAIN_CONTEXT pChainContext,
    _In_ DWORD dwFlags,
    _In_opt_ PCERT_SERVER_OCSP_RESPONSE_OPEN_PARA pOpenPara
    );

// Set this flag to return immediately without making the initial
// synchronous retrieval
#define CERT_SERVER_OCSP_RESPONSE_ASYNC_FLAG        0x00000001

//+-------------------------------------------------------------------------
//  AddRef a HCERT_SERVER_OCSP_RESPONSE returned by
//  CertOpenServerOcspResponse(). Each Open and AddRef requires a
//  corresponding CertCloseServerOcspResponse().
//--------------------------------------------------------------------------
WINCRYPT32API
VOID
WINAPI
CertAddRefServerOcspResponse(
    _In_opt_ HCERT_SERVER_OCSP_RESPONSE hServerOcspResponse
    );

//+-------------------------------------------------------------------------
//  Close the handle returned by CertOpenServerOcspResponse() or AddRef'ed
//  by CertAddRefServerOcspResponse().
//
//  dwFlags isn't currently used and must be set to 0.
//--------------------------------------------------------------------------
WINCRYPT32API
VOID
WINAPI
CertCloseServerOcspResponse(
    _In_opt_ HCERT_SERVER_OCSP_RESPONSE hServerOcspResponse,
    _In_ DWORD dwFlags
    );


//+-------------------------------------------------------------------------
//  Get a time valid OCSP response context for the handle created for
//  the server certificate chain.
//
//  This API won't block to retrieve the OCSP response. It will return
//  the current pre-fetched OCSP response. If a time valid OCSP response
//  isn't available, NULL will be returned with LAST_ERROR set to
//  CRYPT_E_REVOCATION_OFFLINE.
//
//  CertFreeServerOcspResponseContext() must be called to free the
//  returned OCSP response context.
//--------------------------------------------------------------------------
WINCRYPT32API
PCCERT_SERVER_OCSP_RESPONSE_CONTEXT
WINAPI
CertGetServerOcspResponseContext(
    _In_ HCERT_SERVER_OCSP_RESPONSE hServerOcspResponse,
    _In_ DWORD dwFlags,
    _Reserved_ LPVOID pvReserved
    );

//+-------------------------------------------------------------------------
//  AddRef a PCCERT_SERVER_OCSP_RESPONSE_CONTEXT returned by
//  CertGetServerOcspResponseContext(). Each Get and AddRef requires a
//  corresponding CertFreeServerOcspResponseContext().
//--------------------------------------------------------------------------
WINCRYPT32API
VOID
WINAPI
CertAddRefServerOcspResponseContext(
    _In_opt_ PCCERT_SERVER_OCSP_RESPONSE_CONTEXT pServerOcspResponseContext
    );

//+-------------------------------------------------------------------------
//  Free the OCSP response context returned by
//  CertGetServerOcspResponseContext().
//--------------------------------------------------------------------------
WINCRYPT32API
VOID
WINAPI
CertFreeServerOcspResponseContext(
    _In_opt_ PCCERT_SERVER_OCSP_RESPONSE_CONTEXT pServerOcspResponseContext
    );

#endif // (NTDDI_VERSION >= NTDDI_VISTA)


//+-------------------------------------------------------------------------
//  Helper function to do URL retrieval of logo or biometric information
//  specified in either the szOID_LOGOTYPE_EXT or szOID_BIOMETRIC_EXT
//  certificate extension.
//
//  Only the first hashed URL matching lpszLogoOrBiometricType is used
//  to do the URL retrieval. Only direct logotypes are supported.
//  The bytes at the first URL are retrieved via
//  CryptRetrieveObjectByUrlW and hashed. The computed hash is compared
//  against the hash in the certificate.  For success, ppbData, pcbData
//  and optionally ppwszMimeType are updated with
//  CryptMemAlloc'ed memory which must be freed by calling CryptMemFree().
//  For failure, *ppbData, *pcbData and optionally *ppwszMimeType are
//  zero'ed.
//
//  For failure, the following errors may be set in LastError:
//      E_INVALIDARG - invalid lpszLogoOrBiometricType, not one of the
//          acceptable predefined types.
//      CRYPT_E_NOT_FOUND - certificate doesn't have the
//          szOID_LOGOTYPE_EXT or szOID_BIOMETRIC_EXT extension or a matching
//          lpszLogoOrBiometricType wasn't found with a non-empty
//          hashed URL.
//      ERROR_NOT_SUPPORTED - matched the unsupported indirect logotype
//      NTE_BAD_ALGID - unknown hash algorithm OID
//      ERROR_INVALID_DATA - no bytes were retrieved at the specified URL
//          in the certificate extension
//      CRYPT_E_HASH_VALUE - the computed hash doesn't match the hash
//          in the certificate
//  CertRetrieveLogoOrBiometricInfo calls the following functions which
//  will set LastError for failure:
//      CryptDecodeObjectEx(szOID_LOGOTYPE_EXT or szOID_BIOMETRIC_EXT)
//      CryptRetrieveObjectByUrlW
//      CryptHashCertificate
//      CryptMemAlloc
//
//  lpszLogoOrBiometricType is one of the predefined logotype or biometric
//  types, an other logotype OID or a biometric OID.
//
//  dwRetrievalFlags - see CryptRetrieveObjectByUrlW
//  dwTimeout - see CryptRetrieveObjectByUrlW
//
//  dwFlags - reserved, must be set to 0
//  pvReserved - reserved, must be set to NULL
//
//  *ppwszMimeType is always NULL for the biometric types. For success,
//  the caller must always check if non-NULL before dereferencing.
//--------------------------------------------------------------------------
WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertRetrieveLogoOrBiometricInfo(
    _In_ PCCERT_CONTEXT pCertContext,
    _In_ LPCSTR lpszLogoOrBiometricType,
    _In_ DWORD dwRetrievalFlags,
    _In_ DWORD dwTimeout,                             // milliseconds
    _In_ DWORD dwFlags,
    _Reserved_ void *pvReserved,
    _Outptr_result_bytebuffer_(*pcbData) BYTE **ppbData,      // CryptMemFree()
    _Out_ DWORD *pcbData,
    _Outptr_opt_result_maybenull_ LPWSTR *ppwszMimeType         // CryptMemFree()
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

// Predefined Logotypes
#define CERT_RETRIEVE_ISSUER_LOGO                       ((LPCSTR) 1)
#define CERT_RETRIEVE_SUBJECT_LOGO                      ((LPCSTR) 2)
#define CERT_RETRIEVE_COMMUNITY_LOGO                    ((LPCSTR) 3)

// Predefined Biometric types
#define CERT_RETRIEVE_BIOMETRIC_PREDEFINED_BASE_TYPE    ((LPCSTR) 1000)

#define CERT_RETRIEVE_BIOMETRIC_PICTURE_TYPE            \
    (CERT_RETRIEVE_BIOMETRIC_PREDEFINED_BASE_TYPE + CERT_BIOMETRIC_PICTURE_TYPE)
#define CERT_RETRIEVE_BIOMETRIC_SIGNATURE_TYPE          \
    (CERT_RETRIEVE_BIOMETRIC_PREDEFINED_BASE_TYPE + CERT_BIOMETRIC_SIGNATURE_TYPE)


//
// Certificate Selection API
//

#if (NTDDI_VERSION >= NTDDI_WIN7)

typedef struct _CERT_SELECT_CHAIN_PARA
{
    HCERTCHAINENGINE    hChainEngine;
    PFILETIME           pTime;
    HCERTSTORE          hAdditionalStore;
    PCERT_CHAIN_PARA    pChainPara;
    DWORD               dwFlags;
}
CERT_SELECT_CHAIN_PARA, *PCERT_SELECT_CHAIN_PARA;
typedef const CERT_SELECT_CHAIN_PARA*    PCCERT_SELECT_CHAIN_PARA;

#define CERT_SELECT_MAX_PARA                500

typedef struct _CERT_SELECT_CRITERIA
{
    DWORD                           dwType;
    DWORD                           cPara;
    _Field_size_(cPara) void**    ppPara;
}
CERT_SELECT_CRITERIA, *PCERT_SELECT_CRITERIA;
typedef const CERT_SELECT_CRITERIA*     PCCERT_SELECT_CRITERIA;


// Selection Criteria

#define CERT_SELECT_BY_ENHKEY_USAGE          1
#define CERT_SELECT_BY_KEY_USAGE             2
#define CERT_SELECT_BY_POLICY_OID            3
#define CERT_SELECT_BY_PROV_NAME             4
#define CERT_SELECT_BY_EXTENSION             5
#define CERT_SELECT_BY_SUBJECT_HOST_NAME     6
#define CERT_SELECT_BY_ISSUER_ATTR           7
#define CERT_SELECT_BY_SUBJECT_ATTR          8
#define CERT_SELECT_BY_ISSUER_NAME           9
#define CERT_SELECT_BY_PUBLIC_KEY            10
#define CERT_SELECT_BY_TLS_SIGNATURES        11

//add for WinRT
#define CERT_SELECT_BY_ISSUER_DISPLAYNAME    12
#define CERT_SELECT_BY_FRIENDLYNAME          13
#define CERT_SELECT_BY_THUMBPRINT            14

#define CERT_SELECT_LAST                    CERT_SELECT_BY_TLS_SIGNATURES
#define CERT_SELECT_MAX                     (CERT_SELECT_LAST * 3)

// Selection Flags

#define CERT_SELECT_ALLOW_EXPIRED                   0x00000001
#define CERT_SELECT_TRUSTED_ROOT                    0x00000002
#define CERT_SELECT_DISALLOW_SELFSIGNED             0x00000004
#define CERT_SELECT_HAS_PRIVATE_KEY                 0x00000008
#define CERT_SELECT_HAS_KEY_FOR_SIGNATURE           0x00000010
#define CERT_SELECT_HAS_KEY_FOR_KEY_EXCHANGE        0x00000020
#define CERT_SELECT_HARDWARE_ONLY                   0x00000040
#define CERT_SELECT_ALLOW_DUPLICATES                0x00000080
#define CERT_SELECT_IGNORE_AUTOSELECT               0x00000100

//+-------------------------------------------------------------------------
//  Build certificate chains from the certificates in the store and select
//  the matching ones based on the flags and selection criteria.
//--------------------------------------------------------------------------

WINCRYPT32API
_Success_(return != FALSE)
BOOL
WINAPI
CertSelectCertificateChains(
    _In_opt_ LPCGUID pSelectionContext,
    _In_ DWORD dwFlags,
    _In_opt_ PCCERT_SELECT_CHAIN_PARA pChainParameters,
    _In_ DWORD cCriteria,
    _In_reads_opt_(cCriteria) PCCERT_SELECT_CRITERIA rgpCriteria,
    _In_ HCERTSTORE hStore,
    _Out_ PDWORD pcSelection,
    _Outptr_result_buffer_(*pcSelection) PCCERT_CHAIN_CONTEXT** pprgpSelection
    );

//+-------------------------------------------------------------------------
//  Free the array of pointers to chain contexts.
//  CertFreeCertificateChain is NOT called for each entry.
//--------------------------------------------------------------------------

WINCRYPT32API
VOID
WINAPI
CertFreeCertificateChainList(
    _In_ PCCERT_CHAIN_CONTEXT* prgpSelection
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN7)

//
// Time stamp API
//

#if (NTDDI_VERSION >= NTDDI_WIN7)

//+-------------------------------------------------------------------------
//  CRYPT_TIMESTAMP_REQUEST
//
//--------------------------------------------------------------------------
#define TIMESTAMP_VERSION  1

typedef struct _CRYPT_TIMESTAMP_REQUEST
{
    DWORD                       dwVersion;              // v1
    CRYPT_ALGORITHM_IDENTIFIER  HashAlgorithm;
    CRYPT_DER_BLOB              HashedMessage;
    LPSTR                       pszTSAPolicyId;         // OPTIONAL
    CRYPT_INTEGER_BLOB          Nonce;                  // OPTIONAL
    BOOL                        fCertReq;               // DEFAULT FALSE
    DWORD                       cExtension;
    _Field_size_(cExtension)
    PCERT_EXTENSION             rgExtension;            // OPTIONAL
} CRYPT_TIMESTAMP_REQUEST, *PCRYPT_TIMESTAMP_REQUEST;

//+-------------------------------------------------------------------------
//  CRYPT_TIMESTAMP_RESPONSE
//
//--------------------------------------------------------------------------
typedef struct _CRYPT_TIMESTAMP_RESPONSE
{
    DWORD                       dwStatus;
    DWORD                       cFreeText;              // OPTIONAL
    _Field_size_(cFreeText)
    LPWSTR*                     rgFreeText;
    CRYPT_BIT_BLOB              FailureInfo;            // OPTIONAL
    CRYPT_DER_BLOB              ContentInfo;            // OPTIONAL
} CRYPT_TIMESTAMP_RESPONSE, *PCRYPT_TIMESTAMP_RESPONSE;

#define TIMESTAMP_STATUS_GRANTED                        0
#define TIMESTAMP_STATUS_GRANTED_WITH_MODS              1
#define TIMESTAMP_STATUS_REJECTED                       2
#define TIMESTAMP_STATUS_WAITING                        3
#define TIMESTAMP_STATUS_REVOCATION_WARNING             4
#define TIMESTAMP_STATUS_REVOKED                        5

#define TIMESTAMP_FAILURE_BAD_ALG                       0
#define TIMESTAMP_FAILURE_BAD_REQUEST                   2
#define TIMESTAMP_FAILURE_BAD_FORMAT                    5
#define TIMESTAMP_FAILURE_TIME_NOT_AVAILABLE            14
#define TIMESTAMP_FAILURE_POLICY_NOT_SUPPORTED          15
#define TIMESTAMP_FAILURE_EXTENSION_NOT_SUPPORTED       16
#define TIMESTAMP_FAILURE_INFO_NOT_AVAILABLE            17
#define TIMESTAMP_FAILURE_SYSTEM_FAILURE                25

//+-------------------------------------------------------------------------
//  CRYPT_TIMESTAMP_ACCURACY
//
//--------------------------------------------------------------------------
typedef struct _CRYPT_TIMESTAMP_ACCURACY
{
    DWORD                       dwSeconds;                  // OPTIONAL
    DWORD                       dwMillis;                   // OPTIONAL
    DWORD                       dwMicros;                   // OPTIONAL
} CRYPT_TIMESTAMP_ACCURACY, *PCRYPT_TIMESTAMP_ACCURACY;

//+-------------------------------------------------------------------------
//  CRYPT_TIMESTAMP_INFO
//
//--------------------------------------------------------------------------
typedef struct _CRYPT_TIMESTAMP_INFO
{
    DWORD                       dwVersion;                  // v1
    LPSTR                       pszTSAPolicyId;
    CRYPT_ALGORITHM_IDENTIFIER  HashAlgorithm;
    CRYPT_DER_BLOB              HashedMessage;
    CRYPT_INTEGER_BLOB          SerialNumber;
    FILETIME                    ftTime;
    PCRYPT_TIMESTAMP_ACCURACY   pvAccuracy;                 // OPTIONAL
    BOOL                        fOrdering;                  // OPTIONAL
    CRYPT_DER_BLOB              Nonce;                      // OPTIONAL
    CRYPT_DER_BLOB              Tsa;                        // OPTIONAL
    DWORD                       cExtension;
    _Field_size_(cExtension)
    PCERT_EXTENSION             rgExtension;                // OPTIONAL
} CRYPT_TIMESTAMP_INFO, *PCRYPT_TIMESTAMP_INFO;

//+-------------------------------------------------------------------------
//  CRYPT_TIMESTAMP_CONTEXT
//
//--------------------------------------------------------------------------
typedef struct _CRYPT_TIMESTAMP_CONTEXT
{
    DWORD                       cbEncoded;
    _Field_size_bytes_(cbEncoded)
    BYTE                        *pbEncoded;
    PCRYPT_TIMESTAMP_INFO       pTimeStamp;
} CRYPT_TIMESTAMP_CONTEXT, *PCRYPT_TIMESTAMP_CONTEXT;

//+-------------------------------------------------------------------------
//  CRYPT_TIMESTAMP_PARA
//
//  pszTSAPolicyId
//      [optional] Specifies the TSA policy under which the time stamp token
//      should be provided.
//
//  Nonce
//      [optional] Specifies the nonce value used by the client to verify the
//      timeliness of the response when no local clock is available.
//
//  fCertReq
//      Specifies whether the TSA must include in response the certificates
//      used to sign the time stamp token.
//
//  rgExtension
//      [optional]  Specifies Extensions to be included in request.

//--------------------------------------------------------------------------
typedef struct _CRYPT_TIMESTAMP_PARA
{
    LPCSTR                      pszTSAPolicyId;             // OPTIONAL
    BOOL                        fRequestCerts;              // Default is TRUE
    CRYPT_INTEGER_BLOB          Nonce;                      // OPTIONAL
    DWORD                       cExtension;
    _Field_size_(cExtension)
    PCERT_EXTENSION             rgExtension;                // OPTIONAL
} CRYPT_TIMESTAMP_PARA, *PCRYPT_TIMESTAMP_PARA;

//+-------------------------------------------------------------------------
//  CryptRetrieveTimeStamp
//
//  wszUrl
//     [in] Specifies TSA where to send request to.
//
//  dwRetrievalFlags
//     [in]
//         TIMESTAMP_VERIFY_CONTEXT_SIGNATURE
//         TIMESTAMP_NO_AUTH_RETRIEVAL
//         TIMESTAMP_DONT_HASH_DATA
//
//  dwTimeout
//     [in] Specifies the maximum number of milliseconds to wait for retrieval.
//     If a value of zero is specified, this function does not time-out.
//
//  pszHashId
//      [in] Specifies hash algorithm OID.
//
//  pPara
//      [in, optional] Specifies additional request parameters.
//
//  pbData
//      [in] Points to array of bytes to be timestamped.
//
//  cbData
//      [in] Number of bytes in pbData.
//
//  ppTsContext
//     [out] The caller must free ppTsContext with CryptMemFree.
//
//  ppTsSigner
//     [out, optional] The address of a CERT_CONTEXT structure pointer that
//     receives the certificate of the signer.
//     When you have finished using this structure, free it by passing this
//     pointer to the CertFreeCertificateContext function.
//     This parameter can be NULL if the TSA signer's certificate is not needed.
//
// Remarks:
//
//     The TIMESTAMP_VERIFY_CONTEXT_SIGNATURE flag can be only used,
//     if fRequestCerts value is TRUE.
//
//--------------------------------------------------------------------------
BOOL
WINAPI
CryptRetrieveTimeStamp(
    _In_                        LPCWSTR     wszUrl,
                                DWORD       dwRetrievalFlags,
                                DWORD       dwTimeout,
    _In_                        LPCSTR      pszHashId,
    _In_opt_                    const CRYPT_TIMESTAMP_PARA *pPara,
    _In_reads_bytes_(cbData)
                                const BYTE  *pbData,
                                DWORD       cbData,
    _Outptr_                 PCRYPT_TIMESTAMP_CONTEXT *ppTsContext,
    _Outptr_result_maybenull_             PCCERT_CONTEXT *ppTsSigner,
    _Out_opt_                   HCERTSTORE  *phStore
    );

// Set this flag to inhibit hash calculation on pbData
#define TIMESTAMP_DONT_HASH_DATA                0x00000001

// Set this flag to enforce signature validation on retrieved time stamp.
#define TIMESTAMP_VERIFY_CONTEXT_SIGNATURE      0x00000020   // CRYPT_VERIFY_CONTEXT_SIGNATURE

// Set this flag to inhibit automatic authentication handling. See the
// wininet flag, INTERNET_FLAG_NO_AUTH, for more details.
#define TIMESTAMP_NO_AUTH_RETRIEVAL             0x00020000  //  CRYPT_NO_AUTH_RETRIEVAL

//+-------------------------------------------------------------------------
// CryptVerifyTimeStampSignature
//
//  pbTSContentInfo
//      [in] Points to a buffer with timestamp content.
//      These bytes are the same as returned in response by CRYPT_TIMESTAMP_CONTEXT::pbEncoded
//
//  cbTSContentInfo
//      [in] Number of bytes in pbTSContentInfo.
//
//  pbData
//      [in] Points to array of bytes to be timestamped.
//
//  cbData
//      [in] Number of bytes in pbData.
//
// hAdditionalStore
//    [in] Handle of any additional store to search for supporting
//    TSA's signing certificates and certificate trust lists (CTLs).
//    This parameter can be NULL if no additional store is to be searched.
//
// ppTsContext
//    [out] The caller must free ppTsContext with CryptMemFree
//
// ppTsSigner
//    [out, optional] The address of a CERT_CONTEXT structure pointer that
//    receives the certificate of the signer.
//    When you have finished using this structure, free it by passing this
//    pointer to the CertFreeCertificateContext function.
//    This parameter can be NULL if the TSA signer's certificate is not needed.
//
// NOTE:
//    The caller should validate pszTSAPolicyId, if any was specified in the request,
//    and ftTime.
//    The caller should also build a chain for ppTsSigner and validate the trust.
//--------------------------------------------------------------------------

_Success_(return == TRUE)
BOOL
WINAPI
CryptVerifyTimeStampSignature (
    _In_reads_bytes_( cbTSContentInfo )
                                const BYTE  *pbTSContentInfo,
                                DWORD       cbTSContentInfo,
    _In_reads_bytes_opt_(cbData)
                                const BYTE  *pbData,
                                DWORD 	    cbData,
    _In_opt_                    HCERTSTORE  hAdditionalStore,
    _Outptr_                 PCRYPT_TIMESTAMP_CONTEXT   *ppTsContext,
    _Outptr_result_maybenull_             PCCERT_CONTEXT *ppTsSigner,
    _Out_opt_                   HCERTSTORE  *phStore
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)*/
#pragma endregion

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// Object Locator Provider API
//

#if (NTDDI_VERSION >= NTDDI_WIN8)

#define CRYPT_OBJECT_LOCATOR_SPN_NAME_TYPE                   1   //ex. "HTTP/www.contoso.com"
#define CRYPT_OBJECT_LOCATOR_LAST_RESERVED_NAME_TYPE         32
#define CRYPT_OBJECT_LOCATOR_FIRST_RESERVED_USER_NAME_TYPE   33
#define CRYPT_OBJECT_LOCATOR_LAST_RESERVED_USER_NAME_TYPE    0x0000FFFF

#define SSL_OBJECT_LOCATOR_PFX_FUNC                      "SslObjectLocatorInitializePfx"
#define SSL_OBJECT_LOCATOR_ISSUER_LIST_FUNC              "SslObjectLocatorInitializeIssuerList"
#define SSL_OBJECT_LOCATOR_CERT_VALIDATION_CONFIG_FUNC   "SslObjectLocatorInitializeCertValidationConfig"


//--------------------------------------------------------------------------
// Releasing the locator can be done with the following reasons
// On system shutdown and process exit, the provider is not expected to
// release all memory. However, on service stop and dll unload the provider
// should clean itself up.
//--------------------------------------------------------------------------

#define CRYPT_OBJECT_LOCATOR_RELEASE_SYSTEM_SHUTDOWN   1
#define CRYPT_OBJECT_LOCATOR_RELEASE_SERVICE_STOP      2
#define CRYPT_OBJECT_LOCATOR_RELEASE_PROCESS_EXIT      3
#define CRYPT_OBJECT_LOCATOR_RELEASE_DLL_UNLOAD        4


//--------------------------------------------------------------------------
// The object locator provider receives this function when it is initialized.
// The object locator provider is expected to call this function when an
// object has changed. This indicates to the application that its copy of the
// object is stale and it should get an updated object.
//
// pContext
//    This is the context pararameter passed into the object locator providers
//    initialize function. The object locator provider must hold onto this context
//    and pass it back into this flush function.
//
// rgIdentifierOrNameList
//    An array of name/identifier blobs for objects that are stale. If an object
//    has an identifier then pass in the identifier name. If an object does not have
//    an identifier then pass in the name. You can pass in NULL which indicates all
//    objects are stale but this is not recommended for performance reasons.
//
// dwIdentifierOrNameListCount
//    Number of names/identifiers in the array. 0 implies that rgIdentifierOrNameList
//    is NULL which means all objects are stale.
//
//--------------------------------------------------------------------------
_Success_(return != 0)
typedef BOOL (WINAPI *PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FLUSH)(
    _In_ LPVOID pContext,
    _In_reads_(dwIdentifierOrNameListCount) PCERT_NAME_BLOB *rgIdentifierOrNameList,
    _In_ DWORD dwIdentifierOrNameListCount); 


//--------------------------------------------------------------------------
// An application will call on the object provider with the GET function when
// the application needs an object. The name blob uniquely identifies the content
// to return. This function can return an identifier data blob. Subsequent calls
// to this function for the same object will pass in the identifier that was previously
// returned. The identifier does not need to uniquely identify a particular object.
//
// pPluginContext
//    This is the context that is returned by the object locator provider when
//    it is initialized. 
//
// pIdentifier
//    This is the identifier that was returned on a previous GET call for this object.
//    On the first call for a particular object it is always NULL.
//
// dwNameType, pNameBlob
//    The name the application is using for the object. The name will uniquely identify
//    an object.
//
// ppContent, pcbContent
//    The returned object.
//
// ppwszPassword
//    If the returned object is a pfx then this is the password for the pfx.
//
// ppIdentifier
//    The identifier for the object.
//--------------------------------------------------------------------------
_Success_(return != 0)
typedef BOOL (WINAPI *PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_GET)(
    _In_opt_ LPVOID pPluginContext,
    _In_opt_ PCRYPT_DATA_BLOB pIdentifier,
    _In_ DWORD dwNameType,
    _In_ PCERT_NAME_BLOB pNameBlob,
    _Outptr_result_bytebuffer_(*pcbContent) PBYTE *ppbContent,
    _Out_ DWORD *pcbContent,
    _Outptr_result_maybenull_ PCWSTR *ppwszPassword,
    _Outptr_result_maybenull_ PCRYPT_DATA_BLOB *ppIdentifier);


//--------------------------------------------------------------------------
// The application has indicated it no longer needs to locate objects by
// calling this release function.
//
// dwReason
//   Can be one of:
//       CRYPT_OBJECT_LOCATOR_RELEASE_SYSTEM_SHUTDOWN
//       CRYPT_OBJECT_LOCATOR_RELEASE_SERVICE_STOP 
//       CRYPT_OBJECT_LOCATOR_RELEASE_PROCESS_EXIT
//       CRYPT_OBJECT_LOCATOR_RELEASE_DLL_UNLOAD
//
//  pPluginContext
//    This is the context that is returned by the object locator provider when
//    it is initialized. 
//--------------------------------------------------------------------------
typedef void (WINAPI * PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_RELEASE)(
    _In_ DWORD dwReason,
    _In_opt_ LPVOID pPluginContext);


//--------------------------------------------------------------------------
// If the PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_GET function returns a password
// that is non-NULL then this function will be called to release the memory.
// Best practice is to zero the memory before releasing it.
//
//  pPluginContext
//    This is the context that is returned by the object locator provider when
//    it is initialized. 
//
//  pwszPassword
//    Password obtained from PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_GET
//--------------------------------------------------------------------------
typedef void (WINAPI *PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_PASSWORD)(
    _In_opt_ LPVOID pPluginContext,
    _In_ PCWSTR pwszPassword
);


//--------------------------------------------------------------------------
// The content returned by the PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_GET function
// is released using this function.
//
//  pPluginContext
//    This is the context that is returned by the object locator provider when
//    it is initialized. 
//
//  pbData
//    Content returned by the GET function.
//--------------------------------------------------------------------------
typedef void (WINAPI *PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE)(
    _In_opt_ LPVOID pPluginContext,
    _In_  PBYTE pbData
);


//--------------------------------------------------------------------------
//
// The identifier returned by the PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_GET function
// is released with this function. This will be called only if the identifier is 
// non-NULL.
// The identifier will be released when the application no longer needs the
// object that was returned by the GET call.
//
// pPluginContext
//    This is the context that is returned by the object locator provider when
//    it is initialized. 
//
// pIdentifier
//    Identifier returned by the GET function.
//--------------------------------------------------------------------------
typedef void (WINAPI *PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_IDENTIFIER)(
    _In_opt_ LPVOID pPluginContext,
    _In_ PCRYPT_DATA_BLOB pIdentifier);


typedef struct _CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE
{
    _Field_range_(sizeof(CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE), sizeof(CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE)) DWORD cbSize;
    PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_GET pfnGet;
    PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_RELEASE pfnRelease;
    PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_PASSWORD pfnFreePassword;
    PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE pfnFree;
    PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FREE_IDENTIFIER pfnFreeIdentifier;
} CRYPT_OBJECT_LOCATOR_PROVIDER_TABLE, *PCRYPT_OBJECT_LOCATOR_PROVIDER_TABLE;


//--------------------------------------------------------------------------
//
// This is the initialization function of the object locator provider.
// 
// pfnFlush
//    This is the function which the provider must call when it detects that
//    an object has changed and the calling application should know about it
//    to prevent stale copies of the object from being used.
//
// pContext
//    This context is passed to the intialization function. The provider
//    is expected to hold onto this context and pass it back with the call
//    call to the flush function
//
// pdwExpectedObjectCount
//    The number of objects that the provider expects it will need to locate.
//    This number will determine the size of a hash table used internally.
//
// pFuncTable
//    A structure that describes a set of callback functions which can be used
//    to get objects and free objects.
//
// ppPluginContext
//    Extra information that the provider can return in its initialize call which
//    will be passed back to each of the subsequent callback functions.
//--------------------------------------------------------------------------
_Success_(return != 0)
typedef BOOL (WINAPI *PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_INITIALIZE)(
    _In_ PFN_CRYPT_OBJECT_LOCATOR_PROVIDER_FLUSH pfnFlush,
    _In_ LPVOID pContext,
    _Out_ DWORD *pdwExpectedObjectCount,
    _Outptr_ PCRYPT_OBJECT_LOCATOR_PROVIDER_TABLE *ppFuncTable,
    _Outptr_result_maybenull_ void **ppPluginContext);


//
// If pTimeStamp is NULL or zero time, then, current time is used.
// For CERT_TIMESTAMP_HASH_USE_TYPE, current time is always used.
//
// If pSignerChainContext is NULL, then, checks if weak hash has
// been disabled for the more restrictive Third Party Chain. If TRUE
// is returned, then, this API must be called again with a nonNULL
// pSignerChainContext which might return FALSE for logging only or
// if this isn't a Third Party Chain and weak hash hasn't been disabled for
// all signers.
//
// For CERT_TIMESTAMP_HASH_USE_TYPE, this should be the file signer and
// not the timestamp chain signer.
//
// The following WinVerifyTrust dwProvFlags map to the corresponding
// dwChainFlags:
//  WTD_DISABLE_MD2_MD4 -> CERT_CHAIN_DISABLE_MD2_MD4
//  WTD_MOTW -> CERT_CHAIN_HAS_MOTW
//

WINCRYPT32API
BOOL
WINAPI
CertIsWeakHash(
    _In_ DWORD dwHashUseType,
    _In_ LPCWSTR pwszCNGHashAlgid,
    _In_ DWORD dwChainFlags,
    _In_opt_ PCCERT_CHAIN_CONTEXT pSignerChainContext,
    _In_opt_ LPFILETIME pTimeStamp,
    _In_opt_ LPCWSTR pwszFileName
    );

typedef WINCRYPT32API BOOL (WINAPI *PFN_CERT_IS_WEAK_HASH)(
    _In_ DWORD dwHashUseType,
    _In_ LPCWSTR pwszCNGHashAlgid,
    _In_ DWORD dwChainFlags,
    _In_opt_ PCCERT_CHAIN_CONTEXT pSignerChainContext,
    _In_opt_ LPFILETIME pTimeStamp,
    _In_opt_ LPCWSTR pwszFileName
    );


//
// Hash Use Types
//

#define CERT_FILE_HASH_USE_TYPE             1
#define CERT_TIMESTAMP_HASH_USE_TYPE        2


#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP|WINAPI_PARTITION_PHONE_RESTRICTED | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif //!defined(_DDK_DRIVER_)

#ifdef __cplusplus
}       // Balance extern "C" above
#endif

#if defined (_MSC_VER)
#if ( _MSC_VER >= 800 )
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201)
#endif
#endif
#endif

#pragma region Desktop Family or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// The following headers are refactored out of WinCrypt.h,
// reflecting the separate binaries.
// Applications which do not depend on all functionality of CRYPT32 etc.,
// can link directly to smaller binaries using smaller headers also.
//

#ifndef _WINCRYPT_NO_DPAPI
#include <dpapi.h>
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // __WINCRYPT_H__
