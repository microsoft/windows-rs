//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 2004.
//
//  File:       credssp.hxx
//
//  Contents:  Public CredSSP Security Package structures.
//
//----------------------------------------------------------------------------

#ifndef __CREDSSP_H__
#define __CREDSSP_H__
#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

#define CREDSSP_NAME L"CREDSSP"
#define TS_SSP_NAME_A "TSSSP"
#define TS_SSP_NAME  L"TSSSP"

#define szOID_TS_KP_TS_SERVER_AUTH "1.3.6.1.4.1.311.54.1.2"

typedef struct _SecPkgContext_ClientCreds
{
    ULONG       AuthBufferLen;
    PUCHAR      AuthBuffer;
} SecPkgContext_ClientCreds, *PSecPkgContext_ClientCreds;

#define CREDSSP_SERVER_AUTH_NEGOTIATE   0x1
#define CREDSSP_SERVER_AUTH_CERTIFICATE 0x2
#define CREDSSP_SERVER_AUTH_LOOPBACK    0x4

#define SECPKG_ALT_ATTR                 0x80000000UL
#define SECPKG_ATTR_CREDS               0x80000080UL
#define SECPKG_ATTR_NEGOTIATION_PACKAGE 0x80000081UL
#define SECPKG_ATTR_C_ACCESS_TOKEN      (SECPKG_ATTR_ACCESS_TOKEN | SECPKG_ALT_ATTR)
#define SECPKG_ATTR_C_FULL_ACCESS_TOKEN 0x80000082UL
#define SECPKG_ATTR_SERVER_AUTH_FLAGS   0x80000083UL
#define SECPKG_ATTR_CERT_TRUST_STATUS   0x80000084UL
#define SECPKG_ATTR_C_FULL_IDENT_TOKEN  0x80000085UL
#define SECPKG_ATTR_CREDS_2             0x80000086UL


typedef enum _CREDSSP_SUBMIT_TYPE {
    CredsspPasswordCreds = 2,
    CredsspSchannelCreds = 4,
    CredsspCertificateCreds = 13,
    CredsspSubmitBufferBoth = 50,
    CredsspSubmitBufferBothOld = 51,
    CredsspCredEx = 100,
} CREDSPP_SUBMIT_TYPE;

typedef struct _CREDSSP_CRED {
    CREDSPP_SUBMIT_TYPE Type;
    PVOID pSchannelCred;
    PVOID pSpnegoCred;
} CREDSSP_CRED, *PCREDSSP_CRED;

#define CREDSSP_CRED_EX_VERSION 0

typedef struct _CREDSSP_CRED_EX {
    CREDSPP_SUBMIT_TYPE Type; // must be CredsspCredEx
    DWORD Version; // must be CREDSSP_CRED_EX_VERSION
    DWORD Flags;
    DWORD Reserved;
    CREDSSP_CRED Cred;
} CREDSSP_CRED_EX, *PCREDSSP_CRED_EX;

#ifdef __cplusplus
static_assert(FIELD_OFFSET(CREDSSP_CRED_EX, Cred) == 16, "CREDSSP_CRED_EX layout changed - this may break WoW64");
#endif

#define CREDSSP_FLAG_REDIRECT 0x00000001

#ifdef SECURITY_KERNEL
PSecurityFunctionTableW
SEC_ENTRY
SpInitSecurityInterfaceW(VOID);
#endif

#ifdef __cplusplus
}  // extern "C"
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //__CREDSSP_H__
