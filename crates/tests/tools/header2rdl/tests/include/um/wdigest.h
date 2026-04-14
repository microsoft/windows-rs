//+-----------------------------------------------------------------------
//
// Copyright (c) 2001 Microsoft Corporation
//
// File:        WDIGEST.H
//
// Contents:    Public WDigest Security Package structures for use
//              with APIs from SECURITY.H
//
//
// History:     28Mar01,  KDamour    Created
//
//------------------------------------------------------------------------

#ifndef __WDIGEST_H__
#define __WDIGEST_H__
#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



// begin_ntsecapi


#ifndef WDIGEST_SP_NAME_A

#define WDIGEST_SP_NAME_A            "WDigest"
#define WDIGEST_SP_NAME_W             L"WDigest"


#ifdef UNICODE

#define WDIGEST_SP_NAME              WDIGEST_SP_NAME_W

#else

#define WDIGEST_SP_NAME              WDIGEST_SP_NAME_A

#endif



#endif // WDIGEST_SP_NAME_A


// end_ntsecapi


// begin_ntsecapi


// This flag indicates to EncryptMessage that the message is not to actually
// be encrypted, but a header/trailer is to be produced - SECQOP_WRAP_NO_ENCRYPT


// end_ntsecapi



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // __WDIGEST_H__

