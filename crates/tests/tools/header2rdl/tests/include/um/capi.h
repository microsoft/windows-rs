//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation.  All Rights Reserved.
//
//  File:       capi.h
//
//----------------------------------------------------------------------------

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <wincrypt.h>
#include <sipbase.h>
#include <mscat.h>
#include <mssip.h>
#include <wintrust.h>

#ifndef _JTRUST_H
#define _JTRUST_H

#if !defined(JAVA_TRUST_PROVIDER)

#ifdef __cplusplus
extern "C" {
#endif

typedef struct _JAVA_TRUST {
    DWORD       cbSize;                   // Size of structure
    DWORD       flag;                     // Reserved
    BOOL        fAllActiveXPermissions;   // ActiveX explicitly asked for all (must have been signed)
    BOOL        fAllPermissions;          // Java permissions, explicit ask for all
    DWORD       dwEncodingType;           // Encoding type
    PBYTE       pbJavaPermissions;        // Encoded java permission blob
    DWORD       cbJavaPermissions;
    PBYTE       pbSigner;                 // Encoded signer.
    DWORD       cbSigner;
    LPCWSTR     pwszZone;                 // Zone index (copied from action data)
    GUID        guidZone;                 // Not used currently
    HRESULT     hVerify;                  // Authenticode policy return
} JAVA_TRUST, *PJAVA_TRUST;

#ifdef __cplusplus
}
#endif

#endif // !defined(JAVA_TRUST_PROVIDER)
#endif // _JTRUST_H

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

