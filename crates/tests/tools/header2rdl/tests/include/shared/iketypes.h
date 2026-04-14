

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */


#ifndef __iketypes_h__
#define __iketypes_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

/* header files for imported files */
#include "fwptypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_iketypes_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family or AppRuntime Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_APPRUNTIME)
#pragma once
#pragma warning(push)
#pragma warning(disable:4668) 
#pragma once
#pragma region Input Buffer SAL 1 compatibility macros
#pragma endregion Input Buffer SAL 1 compatibility macros
#pragma once
#pragma once
#pragma warning(pop)
#if _MSC_VER >=  800
#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201)
#endif
typedef struct IPSEC_V4_UDP_ENCAPSULATION0_ IPSEC_V4_UDP_ENCAPSULATION0;

typedef /* [v1_enum] */ 
enum IKEEXT_KEY_MODULE_TYPE_
    {
        IKEEXT_KEY_MODULE_IKE	= 0,
        IKEEXT_KEY_MODULE_AUTHIP	= ( IKEEXT_KEY_MODULE_IKE + 1 ) ,
        IKEEXT_KEY_MODULE_IKEV2	= ( IKEEXT_KEY_MODULE_AUTHIP + 1 ) ,
        IKEEXT_KEY_MODULE_MAX	= ( IKEEXT_KEY_MODULE_IKEV2 + 1 ) 
    } 	IKEEXT_KEY_MODULE_TYPE;

typedef /* [v1_enum] */ 
enum IKEEXT_AUTHENTICATION_METHOD_TYPE_
    {
        IKEEXT_PRESHARED_KEY	= 0,
        IKEEXT_CERTIFICATE	= ( IKEEXT_PRESHARED_KEY + 1 ) ,
        IKEEXT_KERBEROS	= ( IKEEXT_CERTIFICATE + 1 ) ,
        IKEEXT_ANONYMOUS	= ( IKEEXT_KERBEROS + 1 ) ,
        IKEEXT_SSL	= ( IKEEXT_ANONYMOUS + 1 ) ,
        IKEEXT_NTLM_V2	= ( IKEEXT_SSL + 1 ) ,
        IKEEXT_IPV6_CGA	= ( IKEEXT_NTLM_V2 + 1 ) ,
        IKEEXT_CERTIFICATE_ECDSA_P256	= ( IKEEXT_IPV6_CGA + 1 ) ,
        IKEEXT_CERTIFICATE_ECDSA_P384	= ( IKEEXT_CERTIFICATE_ECDSA_P256 + 1 ) ,
        IKEEXT_SSL_ECDSA_P256	= ( IKEEXT_CERTIFICATE_ECDSA_P384 + 1 ) ,
        IKEEXT_SSL_ECDSA_P384	= ( IKEEXT_SSL_ECDSA_P256 + 1 ) ,
        IKEEXT_EAP	= ( IKEEXT_SSL_ECDSA_P384 + 1 ) ,
        IKEEXT_RESERVED	= ( IKEEXT_EAP + 1 ) ,
        IKEEXT_AUTHENTICATION_METHOD_TYPE_MAX	= ( IKEEXT_RESERVED + 1 ) 
    } 	IKEEXT_AUTHENTICATION_METHOD_TYPE;

typedef /* [v1_enum] */ 
enum IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE_
    {
        IKEEXT_IMPERSONATION_NONE	= 0,
        IKEEXT_IMPERSONATION_SOCKET_PRINCIPAL	= ( IKEEXT_IMPERSONATION_NONE + 1 ) ,
        IKEEXT_IMPERSONATION_MAX	= ( IKEEXT_IMPERSONATION_SOCKET_PRINCIPAL + 1 ) 
    } 	IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE;

typedef struct IKEEXT_PRESHARED_KEY_AUTHENTICATION0__
    {
    FWP_BYTE_BLOB presharedKey;
    } 	IKEEXT_PRESHARED_KEY_AUTHENTICATION0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
#define IKEEXT_PSK_FLAG_LOCAL_AUTH_ONLY  (0x00000001)
#define IKEEXT_PSK_FLAG_REMOTE_AUTH_ONLY  (0x00000002)
typedef struct IKEEXT_PRESHARED_KEY_AUTHENTICATION1__
    {
    FWP_BYTE_BLOB presharedKey;
    UINT32 flags;
    } 	IKEEXT_PRESHARED_KEY_AUTHENTICATION1;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#define IKEEXT_CERT_FLAG_ENABLE_ACCOUNT_MAPPING  (0x00000001)
#define IKEEXT_CERT_FLAG_DISABLE_REQUEST_PAYLOAD (0x00000002)
#define IKEEXT_CERT_FLAG_USE_NAP_CERTIFICATE     (0x00000004)
#define IKEEXT_CERT_FLAG_INTERMEDIATE_CA         (0x00000008)
#define IKEEXT_CERT_FLAG_IGNORE_INIT_CERT_MAP_FAILURE (0x00000010)
#define IKEEXT_CERT_FLAG_PREFER_NAP_CERTIFICATE_OUTBOUND (0x00000020)
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define IKEEXT_CERT_FLAG_SELECT_NAP_CERTIFICATE (0x00000040)
#define IKEEXT_CERT_FLAG_VERIFY_NAP_CERTIFICATE (0x00000080)
#define IKEEXT_CERT_FLAG_FOLLOW_RENEWAL_CERTIFICATE (0x00000100)
#endif // (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct IKEEXT_CERT_ROOT_CONFIG0_
    {
    FWP_BYTE_BLOB certData;
    UINT32 flags;
    } 	IKEEXT_CERT_ROOT_CONFIG0;

#define IKEEXT_CERT_AUTH_FLAG_SSL_ONE_WAY              (0x00000001)
#define IKEEXT_CERT_AUTH_FLAG_DISABLE_CRL_CHECK        (0x00000002)
#define IKEEXT_CERT_AUTH_ENABLE_CRL_CHECK_STRONG       (0x00000004)
#define IKEEXT_CERT_AUTH_DISABLE_SSL_CERT_VALIDATION (0x00000008)
#if (NTDDI_VERSION >= NTDDI_WIN7)
#define IKEEXT_CERT_AUTH_ALLOW_HTTP_CERT_LOOKUP  (0x00000010)
#define IKEEXT_CERT_AUTH_URL_CONTAINS_BUNDLE  (0x00000020)
#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
#define IKEEXT_CERT_AUTH_FLAG_DISABLE_REQUEST_PAYLOAD (0x00000040)
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS3)
typedef /* [v1_enum] */ 
enum IKEEXT_CERT_CONFIG_TYPE_
    {
        IKEEXT_CERT_CONFIG_EXPLICIT_TRUST_LIST	= 0,
        IKEEXT_CERT_CONFIG_ENTERPRISE_STORE	= ( IKEEXT_CERT_CONFIG_EXPLICIT_TRUST_LIST + 1 ) ,
        IKEEXT_CERT_CONFIG_TRUSTED_ROOT_STORE	= ( IKEEXT_CERT_CONFIG_ENTERPRISE_STORE + 1 ) ,
        IKEEXT_CERT_CONFIG_UNSPECIFIED	= ( IKEEXT_CERT_CONFIG_TRUSTED_ROOT_STORE + 1 ) ,
        IKEEXT_CERT_CONFIG_TYPE_MAX	= ( IKEEXT_CERT_CONFIG_UNSPECIFIED + 1 ) 
    } 	IKEEXT_CERT_CONFIG_TYPE;

typedef struct IKEEXT_CERTIFICATE_AUTHENTICATION0_
    {
    IKEEXT_CERT_CONFIG_TYPE inboundConfigType;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ struct 
            {
            UINT32 inboundRootArraySize;
            /* [unique][size_is] */ IKEEXT_CERT_ROOT_CONFIG0 *inboundRootArray;
            } 	;
        /* [case()][unique] */ IKEEXT_CERT_ROOT_CONFIG0 *inboundEnterpriseStoreConfig;
        /* [case()][unique] */ IKEEXT_CERT_ROOT_CONFIG0 *inboundTrustedRootStoreConfig;
        } 	;
    IKEEXT_CERT_CONFIG_TYPE outboundConfigType;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ struct 
            {
            UINT32 outboundRootArraySize;
            /* [unique][size_is] */ IKEEXT_CERT_ROOT_CONFIG0 *outboundRootArray;
            } 	;
        /* [case()][unique] */ IKEEXT_CERT_ROOT_CONFIG0 *outboundEnterpriseStoreConfig;
        /* [case()][unique] */ IKEEXT_CERT_ROOT_CONFIG0 *outboundTrustedRootStoreConfig;
        } 	;
    UINT32 flags;
    } 	IKEEXT_CERTIFICATE_AUTHENTICATION0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct IKEEXT_CERTIFICATE_AUTHENTICATION1_
    {
    IKEEXT_CERT_CONFIG_TYPE inboundConfigType;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ struct 
            {
            UINT32 inboundRootArraySize;
            /* [unique][size_is] */ IKEEXT_CERT_ROOT_CONFIG0 *inboundRootArray;
            } 	;
        /* [case()][unique] */ IKEEXT_CERT_ROOT_CONFIG0 *inboundEnterpriseStoreConfig;
        /* [case()][unique] */ IKEEXT_CERT_ROOT_CONFIG0 *inboundTrustedRootStoreConfig;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    IKEEXT_CERT_CONFIG_TYPE outboundConfigType;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ struct 
            {
            UINT32 outboundRootArraySize;
            /* [unique][size_is] */ IKEEXT_CERT_ROOT_CONFIG0 *outboundRootArray;
            } 	;
        /* [case()][unique] */ IKEEXT_CERT_ROOT_CONFIG0 *outboundEnterpriseStoreConfig;
        /* [case()][unique] */ IKEEXT_CERT_ROOT_CONFIG0 *outboundTrustedRootStoreConfig;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    UINT32 flags;
    FWP_BYTE_BLOB localCertLocationUrl;
    } 	IKEEXT_CERTIFICATE_AUTHENTICATION1;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef 
enum IKEEXT_CERT_CRITERIA_NAME_TYPE_
    {
        IKEEXT_CERT_CRITERIA_DNS	= 0,
        IKEEXT_CERT_CRITERIA_UPN	= ( IKEEXT_CERT_CRITERIA_DNS + 1 ) ,
        IKEEXT_CERT_CRITERIA_RFC822	= ( IKEEXT_CERT_CRITERIA_UPN + 1 ) ,
        IKEEXT_CERT_CRITERIA_CN	= ( IKEEXT_CERT_CRITERIA_RFC822 + 1 ) ,
        IKEEXT_CERT_CRITERIA_OU	= ( IKEEXT_CERT_CRITERIA_CN + 1 ) ,
        IKEEXT_CERT_CRITERIA_O	= ( IKEEXT_CERT_CRITERIA_OU + 1 ) ,
        IKEEXT_CERT_CRITERIA_DC	= ( IKEEXT_CERT_CRITERIA_O + 1 ) ,
        IKEEXT_CERT_CRITERIA_NAME_TYPE_MAX	= ( IKEEXT_CERT_CRITERIA_DC + 1 ) 
    } 	IKEEXT_CERT_CRITERIA_NAME_TYPE;

typedef struct IKEEXT_CERT_EKUS0_
    {
    ULONG numEku;
    /* [ref][size_is][string] */ LPSTR *eku;
    } 	IKEEXT_CERT_EKUS0;

typedef struct IKEEXT_CERT_NAME0_
    {
    IKEEXT_CERT_CRITERIA_NAME_TYPE nameType;
    /* [ref][string] */ LPWSTR certName;
    } 	IKEEXT_CERT_NAME0;

typedef struct IKEEXT_CERTIFICATE_CRITERIA0_
    {
    FWP_BYTE_BLOB certData;
    FWP_BYTE_BLOB certHash;
    /* [unique] */ IKEEXT_CERT_EKUS0 *eku;
    /* [unique] */ IKEEXT_CERT_NAME0 *name;
    UINT32 flags;
    } 	IKEEXT_CERTIFICATE_CRITERIA0;

typedef struct IKEEXT_CERTIFICATE_AUTHENTICATION2_
    {
    IKEEXT_CERT_CONFIG_TYPE inboundConfigType;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ struct 
            {
            UINT32 inboundRootArraySize;
            /* [unique][size_is] */ IKEEXT_CERTIFICATE_CRITERIA0 *inboundRootCriteria;
            } 	;
        /* [case()] */ struct 
            {
            UINT32 inboundEnterpriseStoreArraySize;
            /* [unique][size_is] */ IKEEXT_CERTIFICATE_CRITERIA0 *inboundEnterpriseStoreCriteria;
            } 	;
        /* [case()] */ struct 
            {
            UINT32 inboundRootStoreArraySize;
            /* [unique][size_is] */ IKEEXT_CERTIFICATE_CRITERIA0 *inboundTrustedRootStoreCriteria;
            } 	;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    IKEEXT_CERT_CONFIG_TYPE outboundConfigType;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ struct 
            {
            UINT32 outboundRootArraySize;
            /* [unique][size_is] */ IKEEXT_CERTIFICATE_CRITERIA0 *outboundRootCriteria;
            } 	;
        /* [case()] */ struct 
            {
            UINT32 outboundEnterpriseStoreArraySize;
            /* [unique][size_is] */ IKEEXT_CERTIFICATE_CRITERIA0 *outboundEnterpriseStoreCriteria;
            } 	;
        /* [case()] */ struct 
            {
            UINT32 outboundRootStoreArraySize;
            /* [unique][size_is] */ IKEEXT_CERTIFICATE_CRITERIA0 *outboundTrustedRootStoreCriteria;
            } 	;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    UINT32 flags;
    FWP_BYTE_BLOB localCertLocationUrl;
    } 	IKEEXT_CERTIFICATE_AUTHENTICATION2;

#endif // (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct IKEEXT_IPV6_CGA_AUTHENTICATION0_
    {
    /* [ref][string] */ wchar_t *keyContainerName;
    /* [unique][string] */ wchar_t *cspName;
    UINT32 cspType;
    FWP_BYTE_ARRAY16 cgaModifier;
    BYTE cgaCollisionCount;
    } 	IKEEXT_IPV6_CGA_AUTHENTICATION0;

#define IKEEXT_KERB_AUTH_DISABLE_INITIATOR_TOKEN_GENERATION (0x00000001)
#define IKEEXT_KERB_AUTH_DONT_ACCEPT_EXPLICIT_CREDENTIALS (0x00000002)
typedef struct IKEEXT_KERBEROS_AUTHENTICATION0__
    {
    UINT32 flags;
    } 	IKEEXT_KERBEROS_AUTHENTICATION0;

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define IKEEXT_KERB_AUTH_FORCE_PROXY_ON_INITIATOR (0x00000004)
typedef struct IKEEXT_KERBEROS_AUTHENTICATION1__
    {
    UINT32 flags;
    /* [unique][string] */ wchar_t *proxyServer;
    } 	IKEEXT_KERBEROS_AUTHENTICATION1;

#endif // (NTDDI_VERSION >= NTDDI_WIN8)
#define IKEEXT_RESERVED_AUTH_DISABLE_INITIATOR_TOKEN_GENERATION (0x00000001)
typedef struct IKEEXT_RESERVED_AUTHENTICATION0__
    {
    UINT32 flags;
    } 	IKEEXT_RESERVED_AUTHENTICATION0;

#define IKEEXT_NTLM_V2_AUTH_DONT_ACCEPT_EXPLICIT_CREDENTIALS (0x00000001)
typedef struct IKEEXT_NTLM_V2_AUTHENTICATION0__
    {
    UINT32 flags;
    } 	IKEEXT_NTLM_V2_AUTHENTICATION0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
#define IKEEXT_EAP_FLAG_LOCAL_AUTH_ONLY  (0x00000001)
#define IKEEXT_EAP_FLAG_REMOTE_AUTH_ONLY  (0x00000002)
typedef struct IKEEXT_EAP_AUTHENTICATION0__
    {
    UINT32 flags;
    } 	IKEEXT_EAP_AUTHENTICATION0;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct IKEEXT_AUTHENTICATION_METHOD0_
    {
    IKEEXT_AUTHENTICATION_METHOD_TYPE authenticationMethodType;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ IKEEXT_PRESHARED_KEY_AUTHENTICATION0 presharedKeyAuthentication;
        /* [case()] */ IKEEXT_CERTIFICATE_AUTHENTICATION0 certificateAuthentication;
        /* [case()] */ IKEEXT_KERBEROS_AUTHENTICATION0 kerberosAuthentication;
        /* [case()] */ IKEEXT_NTLM_V2_AUTHENTICATION0 ntlmV2Authentication;
        /* [case()] */  /* Empty union arm */ 
        /* [case()] */ IKEEXT_CERTIFICATE_AUTHENTICATION0 sslAuthentication;
        /* [case()] */ IKEEXT_IPV6_CGA_AUTHENTICATION0 cgaAuthentication;
        } 	;
    } 	IKEEXT_AUTHENTICATION_METHOD0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct IKEEXT_AUTHENTICATION_METHOD1_
    {
    IKEEXT_AUTHENTICATION_METHOD_TYPE authenticationMethodType;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ IKEEXT_PRESHARED_KEY_AUTHENTICATION1 presharedKeyAuthentication;
        /* [case()] */ IKEEXT_CERTIFICATE_AUTHENTICATION1 certificateAuthentication;
        /* [case()] */ IKEEXT_KERBEROS_AUTHENTICATION0 kerberosAuthentication;
        /* [case()] */ IKEEXT_NTLM_V2_AUTHENTICATION0 ntlmV2Authentication;
        /* [case()] */  /* Empty union arm */ 
        /* [case()] */ IKEEXT_CERTIFICATE_AUTHENTICATION1 sslAuthentication;
        /* [case()] */ IKEEXT_IPV6_CGA_AUTHENTICATION0 cgaAuthentication;
        /* [case()] */ IKEEXT_EAP_AUTHENTICATION0 eapAuthentication;
        } 	;
    } 	IKEEXT_AUTHENTICATION_METHOD1;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct IKEEXT_AUTHENTICATION_METHOD2_
    {
    IKEEXT_AUTHENTICATION_METHOD_TYPE authenticationMethodType;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ IKEEXT_PRESHARED_KEY_AUTHENTICATION1 presharedKeyAuthentication;
        /* [case()] */ IKEEXT_CERTIFICATE_AUTHENTICATION2 certificateAuthentication;
        /* [case()] */ IKEEXT_KERBEROS_AUTHENTICATION1 kerberosAuthentication;
        /* [case()] */ IKEEXT_RESERVED_AUTHENTICATION0 reservedAuthentication;
        /* [case()] */ IKEEXT_NTLM_V2_AUTHENTICATION0 ntlmV2Authentication;
        /* [case()] */  /* Empty union arm */ 
        /* [case()] */ IKEEXT_CERTIFICATE_AUTHENTICATION2 sslAuthentication;
        /* [case()] */ IKEEXT_IPV6_CGA_AUTHENTICATION0 cgaAuthentication;
        /* [case()] */ IKEEXT_EAP_AUTHENTICATION0 eapAuthentication;
        } 	;
    } 	IKEEXT_AUTHENTICATION_METHOD2;

#endif // (NTDDI_VERSION >= NTDDI_WIN8)
typedef /* [v1_enum] */ 
enum IKEEXT_CIPHER_TYPE_
    {
        IKEEXT_CIPHER_DES	= 0,
        IKEEXT_CIPHER_3DES	= ( IKEEXT_CIPHER_DES + 1 ) ,
        IKEEXT_CIPHER_AES_128	= ( IKEEXT_CIPHER_3DES + 1 ) ,
        IKEEXT_CIPHER_AES_192	= ( IKEEXT_CIPHER_AES_128 + 1 ) ,
        IKEEXT_CIPHER_AES_256	= ( IKEEXT_CIPHER_AES_192 + 1 ) ,
        IKEEXT_CIPHER_AES_GCM_128_16ICV	= ( IKEEXT_CIPHER_AES_256 + 1 ) ,
        IKEEXT_CIPHER_AES_GCM_256_16ICV	= ( IKEEXT_CIPHER_AES_GCM_128_16ICV + 1 ) ,
        IKEEXT_CIPHER_TYPE_MAX	= ( IKEEXT_CIPHER_AES_GCM_256_16ICV + 1 ) 
    } 	IKEEXT_CIPHER_TYPE;

typedef struct IKEEXT_CIPHER_ALGORITHM0_
    {
    IKEEXT_CIPHER_TYPE algoIdentifier;
    UINT32 keyLen;
    UINT32 rounds;
    } 	IKEEXT_CIPHER_ALGORITHM0;

typedef /* [v1_enum] */ 
enum IKEEXT_INTEGRITY_TYPE_
    {
        IKEEXT_INTEGRITY_MD5	= 0,
        IKEEXT_INTEGRITY_SHA1	= ( IKEEXT_INTEGRITY_MD5 + 1 ) ,
        IKEEXT_INTEGRITY_SHA_256	= ( IKEEXT_INTEGRITY_SHA1 + 1 ) ,
        IKEEXT_INTEGRITY_SHA_384	= ( IKEEXT_INTEGRITY_SHA_256 + 1 ) ,
        IKEEXT_INTEGRITY_TYPE_MAX	= ( IKEEXT_INTEGRITY_SHA_384 + 1 ) 
    } 	IKEEXT_INTEGRITY_TYPE;

typedef struct IKEEXT_INTEGRITY_ALGORITHM0_
    {
    IKEEXT_INTEGRITY_TYPE algoIdentifier;
    } 	IKEEXT_INTEGRITY_ALGORITHM0;

typedef /* [v1_enum] */ 
enum IKEEXT_DH_GROUP_
    {
        IKEEXT_DH_GROUP_NONE	= 0,
        IKEEXT_DH_GROUP_1	= ( IKEEXT_DH_GROUP_NONE + 1 ) ,
        IKEEXT_DH_GROUP_2	= ( IKEEXT_DH_GROUP_1 + 1 ) ,
        IKEEXT_DH_GROUP_14	= ( IKEEXT_DH_GROUP_2 + 1 ) ,
        IKEEXT_DH_GROUP_2048	= IKEEXT_DH_GROUP_14,
        IKEEXT_DH_ECP_256	= ( IKEEXT_DH_GROUP_2048 + 1 ) ,
        IKEEXT_DH_ECP_384	= ( IKEEXT_DH_ECP_256 + 1 ) ,
        IKEEXT_DH_GROUP_24	= ( IKEEXT_DH_ECP_384 + 1 ) ,
        IKEEXT_DH_GROUP_MAX	= ( IKEEXT_DH_GROUP_24 + 1 ) 
    } 	IKEEXT_DH_GROUP;

typedef struct IKEEXT_PROPOSAL0_
    {
    IKEEXT_CIPHER_ALGORITHM0 cipherAlgorithm;
    IKEEXT_INTEGRITY_ALGORITHM0 integrityAlgorithm;
    UINT32 maxLifetimeSeconds;
    IKEEXT_DH_GROUP dhGroup;
    UINT32 quickModeLimit;
    } 	IKEEXT_PROPOSAL0;

#define IKEEXT_POLICY_FLAG_DISABLE_DIAGNOSTICS (0x00000001)
#define IKEEXT_POLICY_FLAG_NO_MACHINE_LUID_VERIFY (0x00000002)
#define IKEEXT_POLICY_FLAG_NO_IMPERSONATION_LUID_VERIFY (0x00000004)
#define IKEEXT_POLICY_FLAG_ENABLE_OPTIONAL_DH (0x00000008)
#define IKEEXT_POLICY_FLAG_MOBIKE_NOT_SUPPORTED (0x00000010)
#define IKEEXT_POLICY_FLAG_SITE_TO_SITE (0x00000020)
#define IKEEXT_POLICY_FLAG_IMS_VPN (0x00000040)
#define IKEEXT_POLICY_ENABLE_IKEV2_FRAGMENTATION (0x00000080)
#define IKEEXT_POLICY_SUPPORT_LOW_POWER_MODE (0x00000100)
#define IKEEXT_POLICY_FLAG_POINT_TO_SITE (0x00000200)
typedef struct IKEEXT_POLICY0_
    {
    UINT32 softExpirationTime;
    UINT32 numAuthenticationMethods;
    /* [ref][size_is] */ IKEEXT_AUTHENTICATION_METHOD0 *authenticationMethods;
    IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE initiatorImpersonationType;
    UINT32 numIkeProposals;
    /* [ref][size_is] */ IKEEXT_PROPOSAL0 *ikeProposals;
    UINT32 flags;
    UINT32 maxDynamicFilters;
    } 	IKEEXT_POLICY0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct IKEEXT_POLICY1_
    {
    UINT32 softExpirationTime;
    UINT32 numAuthenticationMethods;
    /* [ref][size_is] */ IKEEXT_AUTHENTICATION_METHOD1 *authenticationMethods;
    IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE initiatorImpersonationType;
    UINT32 numIkeProposals;
    /* [ref][size_is] */ IKEEXT_PROPOSAL0 *ikeProposals;
    UINT32 flags;
    UINT32 maxDynamicFilters;
    UINT32 retransmitDurationSecs;
    } 	IKEEXT_POLICY1;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct IKEEXT_POLICY2_
    {
    UINT32 softExpirationTime;
    UINT32 numAuthenticationMethods;
    /* [ref][size_is] */ IKEEXT_AUTHENTICATION_METHOD2 *authenticationMethods;
    IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE initiatorImpersonationType;
    UINT32 numIkeProposals;
    /* [ref][size_is] */ IKEEXT_PROPOSAL0 *ikeProposals;
    UINT32 flags;
    UINT32 maxDynamicFilters;
    UINT32 retransmitDurationSecs;
    } 	IKEEXT_POLICY2;

#endif // (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct IKEEXT_EM_POLICY0_
    {
    UINT32 numAuthenticationMethods;
    /* [ref][size_is] */ IKEEXT_AUTHENTICATION_METHOD0 *authenticationMethods;
    IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE initiatorImpersonationType;
    } 	IKEEXT_EM_POLICY0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct IKEEXT_EM_POLICY1_
    {
    UINT32 numAuthenticationMethods;
    /* [ref][size_is] */ IKEEXT_AUTHENTICATION_METHOD1 *authenticationMethods;
    IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE initiatorImpersonationType;
    } 	IKEEXT_EM_POLICY1;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct IKEEXT_EM_POLICY2_
    {
    UINT32 numAuthenticationMethods;
    /* [ref][size_is] */ IKEEXT_AUTHENTICATION_METHOD2 *authenticationMethods;
    IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE initiatorImpersonationType;
    } 	IKEEXT_EM_POLICY2;

#endif // (NTDDI_VERSION >= NTDDI_WIN8)
#define IKEEXT_ERROR_CODE_COUNT (ERROR_IPSEC_IKE_NEG_STATUS_END - ERROR_IPSEC_IKE_NEG_STATUS_BEGIN)
typedef struct IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0_
    {
    UINT32 currentActiveMainModes;
    UINT32 totalMainModesStarted;
    UINT32 totalSuccessfulMainModes;
    UINT32 totalFailedMainModes;
    UINT32 totalResponderMainModes;
    UINT32 currentNewResponderMainModes;
    UINT32 currentActiveQuickModes;
    UINT32 totalQuickModesStarted;
    UINT32 totalSuccessfulQuickModes;
    UINT32 totalFailedQuickModes;
    UINT32 totalAcquires;
    UINT32 totalReinitAcquires;
    UINT32 currentActiveExtendedModes;
    UINT32 totalExtendedModesStarted;
    UINT32 totalSuccessfulExtendedModes;
    UINT32 totalFailedExtendedModes;
    UINT32 totalImpersonationExtendedModes;
    UINT32 totalImpersonationMainModes;
    } 	IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1_
    {
    UINT32 currentActiveMainModes;
    UINT32 totalMainModesStarted;
    UINT32 totalSuccessfulMainModes;
    UINT32 totalFailedMainModes;
    UINT32 totalResponderMainModes;
    UINT32 currentNewResponderMainModes;
    UINT32 currentActiveQuickModes;
    UINT32 totalQuickModesStarted;
    UINT32 totalSuccessfulQuickModes;
    UINT32 totalFailedQuickModes;
    UINT32 totalAcquires;
    UINT32 totalReinitAcquires;
    UINT32 currentActiveExtendedModes;
    UINT32 totalExtendedModesStarted;
    UINT32 totalSuccessfulExtendedModes;
    UINT32 totalFailedExtendedModes;
    UINT32 totalImpersonationExtendedModes;
    UINT32 totalImpersonationMainModes;
    } 	IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct IKEEXT_KEYMODULE_STATISTICS0_
    {
    IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 v4Statistics;
    IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 v6Statistics;
    UINT32 errorFrequencyTable[ 97 ];
    UINT32 mainModeNegotiationTime;
    UINT32 quickModeNegotiationTime;
    UINT32 extendedModeNegotiationTime;
    } 	IKEEXT_KEYMODULE_STATISTICS0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct IKEEXT_KEYMODULE_STATISTICS1_
    {
    IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 v4Statistics;
    IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 v6Statistics;
    UINT32 errorFrequencyTable[ 97 ];
    UINT32 mainModeNegotiationTime;
    UINT32 quickModeNegotiationTime;
    UINT32 extendedModeNegotiationTime;
    } 	IKEEXT_KEYMODULE_STATISTICS1;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0_
    {
    UINT32 totalSocketReceiveFailures;
    UINT32 totalSocketSendFailures;
    } 	IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1_
    {
    UINT32 totalSocketReceiveFailures;
    UINT32 totalSocketSendFailures;
    } 	IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct IKEEXT_COMMON_STATISTICS0_
    {
    IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 v4Statistics;
    IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 v6Statistics;
    UINT32 totalPacketsReceived;
    UINT32 totalInvalidPacketsReceived;
    UINT32 currentQueuedWorkitems;
    } 	IKEEXT_COMMON_STATISTICS0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct IKEEXT_COMMON_STATISTICS1_
    {
    IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 v4Statistics;
    IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 v6Statistics;
    UINT32 totalPacketsReceived;
    UINT32 totalInvalidPacketsReceived;
    UINT32 currentQueuedWorkitems;
    } 	IKEEXT_COMMON_STATISTICS1;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct IKEEXT_STATISTICS0_
    {
    IKEEXT_KEYMODULE_STATISTICS0 ikeStatistics;
    IKEEXT_KEYMODULE_STATISTICS0 authipStatistics;
    IKEEXT_COMMON_STATISTICS0 commonStatistics;
    } 	IKEEXT_STATISTICS0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct IKEEXT_STATISTICS1_
    {
    IKEEXT_KEYMODULE_STATISTICS1 ikeStatistics;
    IKEEXT_KEYMODULE_STATISTICS1 authipStatistics;
    IKEEXT_KEYMODULE_STATISTICS1 ikeV2Statistics;
    IKEEXT_COMMON_STATISTICS1 commonStatistics;
    } 	IKEEXT_STATISTICS1;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct IKEEXT_TRAFFIC0_
    {
    FWP_IP_VERSION ipVersion;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ UINT32 localV4Address;
        /* [case()] */ UINT8 localV6Address[ 16 ];
        } 	;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()] */ UINT32 remoteV4Address;
        /* [case()] */ UINT8 remoteV6Address[ 16 ];
        } 	;
    UINT64 authIpFilterId;
    } 	IKEEXT_TRAFFIC0;

typedef UINT64 IKEEXT_COOKIE;

typedef struct IKEEXT_COOKIE_PAIR0_
    {
    IKEEXT_COOKIE initiator;
    IKEEXT_COOKIE responder;
    } 	IKEEXT_COOKIE_PAIR0;

#define IKEEXT_CERT_CREDENTIAL_FLAG_NAP_CERT     (0x00000001)
typedef struct IKEEXT_CERTIFICATE_CREDENTIAL0_
    {
    FWP_BYTE_BLOB subjectName;
    FWP_BYTE_BLOB certHash;
    UINT32 flags;
    } 	IKEEXT_CERTIFICATE_CREDENTIAL0;

typedef struct IKEEXT_NAME_CREDENTIAL0_
    {
    /* [ref][string] */ wchar_t *principalName;
    } 	IKEEXT_NAME_CREDENTIAL0;

typedef struct IKEEXT_CREDENTIAL0_
    {
    IKEEXT_AUTHENTICATION_METHOD_TYPE authenticationMethodType;
    IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE impersonationType;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()][unique] */ IKEEXT_PRESHARED_KEY_AUTHENTICATION0 *presharedKey;
        /* [case()][unique] */ IKEEXT_CERTIFICATE_CREDENTIAL0 *certificate;
        /* [case()][unique] */ IKEEXT_NAME_CREDENTIAL0 *name;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    } 	IKEEXT_CREDENTIAL0;

typedef struct IKEEXT_CREDENTIAL_PAIR0_
    {
    IKEEXT_CREDENTIAL0 localCredentials;
    IKEEXT_CREDENTIAL0 peerCredentials;
    } 	IKEEXT_CREDENTIAL_PAIR0;

typedef struct IKEEXT_CREDENTIALS0_
    {
    UINT32 numCredentials;
    /* [ref][size_is] */ IKEEXT_CREDENTIAL_PAIR0 *credentials;
    } 	IKEEXT_CREDENTIALS0;

typedef struct IKEEXT_SA_DETAILS0_
    {
    UINT64 saId;
    IKEEXT_KEY_MODULE_TYPE keyModuleType;
    FWP_IP_VERSION ipVersion;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()][unique] */ IPSEC_V4_UDP_ENCAPSULATION0 *v4UdpEncapsulation;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    IKEEXT_TRAFFIC0 ikeTraffic;
    IKEEXT_PROPOSAL0 ikeProposal;
    IKEEXT_COOKIE_PAIR0 cookiePair;
    IKEEXT_CREDENTIALS0 ikeCredentials;
    GUID ikePolicyKey;
    UINT64 virtualIfTunnelId;
    } 	IKEEXT_SA_DETAILS0;

#if (NTDDI_VERSION >= NTDDI_WIN7)
typedef struct IKEEXT_CERTIFICATE_CREDENTIAL1_
    {
    FWP_BYTE_BLOB subjectName;
    FWP_BYTE_BLOB certHash;
    UINT32 flags;
    FWP_BYTE_BLOB certificate;
    } 	IKEEXT_CERTIFICATE_CREDENTIAL1;

typedef struct IKEEXT_CREDENTIAL1_
    {
    IKEEXT_AUTHENTICATION_METHOD_TYPE authenticationMethodType;
    IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE impersonationType;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()][unique] */ IKEEXT_PRESHARED_KEY_AUTHENTICATION1 *presharedKey;
        /* [case()][unique] */ IKEEXT_CERTIFICATE_CREDENTIAL1 *certificate;
        /* [case()][unique] */ IKEEXT_NAME_CREDENTIAL0 *name;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    } 	IKEEXT_CREDENTIAL1;

typedef struct IKEEXT_CREDENTIAL_PAIR1_
    {
    IKEEXT_CREDENTIAL1 localCredentials;
    IKEEXT_CREDENTIAL1 peerCredentials;
    } 	IKEEXT_CREDENTIAL_PAIR1;

typedef struct IKEEXT_CREDENTIALS1_
    {
    UINT32 numCredentials;
    /* [ref][size_is] */ IKEEXT_CREDENTIAL_PAIR1 *credentials;
    } 	IKEEXT_CREDENTIALS1;

typedef struct IKEEXT_SA_DETAILS1_
    {
    UINT64 saId;
    IKEEXT_KEY_MODULE_TYPE keyModuleType;
    FWP_IP_VERSION ipVersion;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()][unique] */ IPSEC_V4_UDP_ENCAPSULATION0 *v4UdpEncapsulation;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    IKEEXT_TRAFFIC0 ikeTraffic;
    IKEEXT_PROPOSAL0 ikeProposal;
    IKEEXT_COOKIE_PAIR0 cookiePair;
    IKEEXT_CREDENTIALS1 ikeCredentials;
    GUID ikePolicyKey;
    UINT64 virtualIfTunnelId;
    FWP_BYTE_BLOB correlationKey;
    } 	IKEEXT_SA_DETAILS1;

#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct IKEEXT_CREDENTIAL2_
    {
    IKEEXT_AUTHENTICATION_METHOD_TYPE authenticationMethodType;
    IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE impersonationType;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()][unique] */ IKEEXT_PRESHARED_KEY_AUTHENTICATION1 *presharedKey;
        /* [case()][unique] */ IKEEXT_CERTIFICATE_CREDENTIAL1 *certificate;
        /* [case()][unique] */ IKEEXT_NAME_CREDENTIAL0 *name;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    } 	IKEEXT_CREDENTIAL2;

typedef struct IKEEXT_CREDENTIAL_PAIR2_
    {
    IKEEXT_CREDENTIAL2 localCredentials;
    IKEEXT_CREDENTIAL2 peerCredentials;
    } 	IKEEXT_CREDENTIAL_PAIR2;

typedef struct IKEEXT_CREDENTIALS2_
    {
    UINT32 numCredentials;
    /* [ref][size_is] */ IKEEXT_CREDENTIAL_PAIR2 *credentials;
    } 	IKEEXT_CREDENTIALS2;

typedef struct IKEEXT_SA_DETAILS2_
    {
    UINT64 saId;
    IKEEXT_KEY_MODULE_TYPE keyModuleType;
    FWP_IP_VERSION ipVersion;
    /* [switch_is][switch_type] */ union 
        {
        /* [case()][unique] */ IPSEC_V4_UDP_ENCAPSULATION0 *v4UdpEncapsulation;
        /* [case()] */  /* Empty union arm */ 
        } 	;
    IKEEXT_TRAFFIC0 ikeTraffic;
    IKEEXT_PROPOSAL0 ikeProposal;
    IKEEXT_COOKIE_PAIR0 cookiePair;
    IKEEXT_CREDENTIALS2 ikeCredentials;
    GUID ikePolicyKey;
    UINT64 virtualIfTunnelId;
    FWP_BYTE_BLOB correlationKey;
    } 	IKEEXT_SA_DETAILS2;

#endif // (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct IKEEXT_SA_ENUM_TEMPLATE0_
    {
    FWP_CONDITION_VALUE0 localSubNet;
    FWP_CONDITION_VALUE0 remoteSubNet;
    FWP_BYTE_BLOB localMainModeCertHash;
    } 	IKEEXT_SA_ENUM_TEMPLATE0;

typedef /* [v1_enum] */ 
enum IKEEXT_MM_SA_STATE_
    {
        IKEEXT_MM_SA_STATE_NONE	= 0,
        IKEEXT_MM_SA_STATE_SA_SENT	= ( IKEEXT_MM_SA_STATE_NONE + 1 ) ,
        IKEEXT_MM_SA_STATE_SSPI_SENT	= ( IKEEXT_MM_SA_STATE_SA_SENT + 1 ) ,
        IKEEXT_MM_SA_STATE_FINAL	= ( IKEEXT_MM_SA_STATE_SSPI_SENT + 1 ) ,
        IKEEXT_MM_SA_STATE_FINAL_SENT	= ( IKEEXT_MM_SA_STATE_FINAL + 1 ) ,
        IKEEXT_MM_SA_STATE_COMPLETE	= ( IKEEXT_MM_SA_STATE_FINAL_SENT + 1 ) ,
        IKEEXT_MM_SA_STATE_MAX	= ( IKEEXT_MM_SA_STATE_COMPLETE + 1 ) 
    } 	IKEEXT_MM_SA_STATE;

typedef /* [v1_enum] */ 
enum IKEEXT_QM_SA_STATE_
    {
        IKEEXT_QM_SA_STATE_NONE	= 0,
        IKEEXT_QM_SA_STATE_INITIAL	= ( IKEEXT_QM_SA_STATE_NONE + 1 ) ,
        IKEEXT_QM_SA_STATE_FINAL	= ( IKEEXT_QM_SA_STATE_INITIAL + 1 ) ,
        IKEEXT_QM_SA_STATE_COMPLETE	= ( IKEEXT_QM_SA_STATE_FINAL + 1 ) ,
        IKEEXT_QM_SA_STATE_MAX	= ( IKEEXT_QM_SA_STATE_COMPLETE + 1 ) 
    } 	IKEEXT_QM_SA_STATE;

typedef /* [v1_enum] */ 
enum IKEEXT_EM_SA_STATE_
    {
        IKEEXT_EM_SA_STATE_NONE	= 0,
        IKEEXT_EM_SA_STATE_SENT_ATTS	= ( IKEEXT_EM_SA_STATE_NONE + 1 ) ,
        IKEEXT_EM_SA_STATE_SSPI_SENT	= ( IKEEXT_EM_SA_STATE_SENT_ATTS + 1 ) ,
        IKEEXT_EM_SA_STATE_AUTH_COMPLETE	= ( IKEEXT_EM_SA_STATE_SSPI_SENT + 1 ) ,
        IKEEXT_EM_SA_STATE_FINAL	= ( IKEEXT_EM_SA_STATE_AUTH_COMPLETE + 1 ) ,
        IKEEXT_EM_SA_STATE_COMPLETE	= ( IKEEXT_EM_SA_STATE_FINAL + 1 ) ,
        IKEEXT_EM_SA_STATE_MAX	= ( IKEEXT_EM_SA_STATE_COMPLETE + 1 ) 
    } 	IKEEXT_EM_SA_STATE;

typedef /* [v1_enum] */ 
enum IKEEXT_SA_ROLE_
    {
        IKEEXT_SA_ROLE_INITIATOR	= 0,
        IKEEXT_SA_ROLE_RESPONDER	= ( IKEEXT_SA_ROLE_INITIATOR + 1 ) ,
        IKEEXT_SA_ROLE_MAX	= ( IKEEXT_SA_ROLE_RESPONDER + 1 ) 
    } 	IKEEXT_SA_ROLE;

#if _MSC_VER >=  800
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201)
#endif
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_APPRUNTIME) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_iketypes_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_iketypes_0000_0000_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


