//+-------------------------------------------------------------------------
//  Microsoft Windows
//
//  Copyright (C) Microsoft Corporation, 2002 - 2002
//
//  File:       i_cryptasn1tls.h
//
//  Contents:   Crypt ASN.1 Thread Local Storage (TLS) functions
//--------------------------------------------------------------------------

#ifndef __I_CRYPTASN1TLS_H__
#define __I_CRYPTASN1TLS_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif


// Handle to an installed Asn1 module
typedef DWORD HCRYPTASN1MODULE;

// Pointers to ASN1 data structures
typedef void *ASN1module_t;
typedef void *ASN1encoding_t;
typedef void *ASN1decoding_t;


//+-------------------------------------------------------------------------
//  Install an Asn1 module entry and return a handle for future access.
//
//  Each thread has its own copy of the decoder and encoder associated
//  with the Asn1 module. Creation is deferred until first referenced by
//  the thread.
//
//  I_CryptGetAsn1Encoder or I_CryptGetAsn1Decoder must be called with the
//  handle returned by I_CryptInstallAsn1Module to get the thread specific
//  Asn1 encoder or decoder.
//
//  Currently, dwFlags and pvReserved aren't used and must be set to 0.
//--------------------------------------------------------------------------

HCRYPTASN1MODULE
WINAPI
I_CryptInstallAsn1Module(
    IN ASN1module_t pMod,
    IN DWORD dwFlags,
    IN void *pvReserved
    );


//+-------------------------------------------------------------------------
//  Called at DLL_PROCESS_DETACH to uninstall an hAsn1Module entry. Iterates
//  through the threads and frees their created Asn1 encoders and decoders.
//--------------------------------------------------------------------------
BOOL
WINAPI
I_CryptUninstallAsn1Module(
    IN HCRYPTASN1MODULE hAsn1Module
    );


//+-------------------------------------------------------------------------
//  Get the thread specific pointer to the Asn1 encoder specified by the
//  hAsn1Module returned by CryptInstallAsn1Module. If the
//  encoder doesn't exist, then, its created using the Asn1 module
//  associated with hAsn1Module.
//--------------------------------------------------------------------------
ASN1encoding_t
WINAPI
I_CryptGetAsn1Encoder(
    IN HCRYPTASN1MODULE hAsn1Module
    );


//+-------------------------------------------------------------------------
//  Get the thread specific pointer to the Asn1 decoder specified by the
//  hAsn1Module returned by CryptInstallAsn1Module. If the
//  decoder doesn't exist, then, its created using the Asn1 module
//  associated with hAsn1Module.
//--------------------------------------------------------------------------
ASN1decoding_t
WINAPI
I_CryptGetAsn1Decoder(
    IN HCRYPTASN1MODULE hAsn1Module
    );

#ifdef __cplusplus
}       // Balance extern "C" above
#endif



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
