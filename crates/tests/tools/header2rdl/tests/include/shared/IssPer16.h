//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 1992-1999.
//
//  File:      issperr.h
//
//  Contents:  Constant definitions for OLE HRESULT values.
//
//  History:   dd-mmm-yy Author    Comment
//
//  Notes:
//     This is a generated file. Do not modify directly.
//     The MC tool generates this file from dsyserr.mc
//
//--------------------------------------------------------------------------
#ifndef _ISSPERR_H_
#define _ISSPERR_H_
#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


// Define the status type.

#ifdef FACILITY_SECURITY
#undef FACILITY_SECURITY
#endif

#ifdef STATUS_SEVERITY_SUCCESS
#undef STATUS_SEVERITY_SUCCESS
#endif
//#ifdef STATUS_SEVERITY_ERROR
//#undef STATUS_SEVERITY_ERROR
//#endif

// Define the severities
//
//  Values are 32 bit values layed out as follows:
//
//   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//  +---+-+-+-----------------------+-------------------------------+
//  |Sev|C|R|     Facility          |               Code            |
//  +---+-+-+-----------------------+-------------------------------+
//
//  where
//
//      Sev - is the severity code
//
//          00 - Success
//          01 - Informational
//          10 - Warning
//          11 - Error
//
//      C - is the Customer code flag
//
//      R - is a reserved bit
//
//      Facility - is the facility code
//
//      Code - is the facility's status code
//
//
// Define the facility codes
//
#define FACILITY_SECURITY                0x9
#define FACILITY_NULL                    0


//
// Define the severity codes
//
#define STATUS_SEVERITY_SUCCESS          0x0
#define STATUS_SEVERITY_COERROR          0x2


//
// MessageId: SEC_E_INSUFFICIENT_MEMORY
//
// MessageText:
//
//  Not enough memory is available to complete this request
//
#define SEC_E_INSUFFICIENT_MEMORY        ((SECURITY_STATUS)0x1300)

//
// MessageId: SEC_E_INVALID_HANDLE
//
// MessageText:
//
//  The handle specified is invalid
//
#define SEC_E_INVALID_HANDLE             ((SECURITY_STATUS)0x1301)

//
// MessageId: SEC_E_UNSUPPORTED_FUNCTION
//
// MessageText:
//
//  The function requested is not supported
//
#define SEC_E_UNSUPPORTED_FUNCTION       ((SECURITY_STATUS)0x1302)


//
// MessageId: SEC_E_TARGET_UNKNOWN
//
// MessageText:
//
//  The specified target is unknown or unreachable
//
#define SEC_E_TARGET_UNKNOWN             ((SECURITY_STATUS)0x1303)

//
// MessageId: SEC_E_INTERNAL_ERROR
//
// MessageText:
//
//  The Local Security Authority cannot be contacted
//
#define SEC_E_INTERNAL_ERROR             ((SECURITY_STATUS)0x1304)

//
// MessageId: SEC_E_SECPKG_NOT_FOUND
//
// MessageText:
//
//  The requested security package does not exist
//
#define SEC_E_SECPKG_NOT_FOUND           ((SECURITY_STATUS)0x1305)


//
// MessageId: SEC_E_NOT_OWNER
//
// MessageText:
//
//  The caller is not the owner of the desired credentials
//
#define SEC_E_NOT_OWNER                  ((SECURITY_STATUS)0x1306)

//
// MessageId: SEC_E_CANNOT_INSTALL
//
// MessageText:
//
//  The security package failed to initialize, and cannot be installed
//
#define SEC_E_CANNOT_INSTALL             ((SECURITY_STATUS)0x1307)

//
// MessageId: SEC_E_INVALID_TOKEN
//
// MessageText:
//
//  The token supplied to the function is invalid
//
#define SEC_E_INVALID_TOKEN              ((SECURITY_STATUS)0x1308)

//
// MessageId: SEC_E_CANNOT_PACK
//
// MessageText:
//
//  The security package is not able to marshall the logon buffer,
//  so the logon attempt has failed
//
#define SEC_E_CANNOT_PACK                ((SECURITY_STATUS)0x1309)

//
// MessageId: SEC_E_QOP_NOT_SUPPORTED
//
// MessageText:
//
//  The per-message Quality of Protection is not supported by the
//  security package
//
#define SEC_E_QOP_NOT_SUPPORTED          ((SECURITY_STATUS)0x130A)

//
// MessageId: SEC_E_NO_IMPERSONATION
//
// MessageText:
//
//  The security context does not allow impersonation of the client
//
#define SEC_E_NO_IMPERSONATION           ((SECURITY_STATUS)0x130B)

//
// MessageId: SEC_E_LOGON_DENIED
//
// MessageText:
//
//  The logon attempt failed
//
#define SEC_E_LOGON_DENIED               ((SECURITY_STATUS)0x130C)

//
// MessageId: SEC_E_UNKNOWN_CREDENTIALS
//
// MessageText:
//
//  The credentials supplied to the package were not
//  recognized
//
#define SEC_E_UNKNOWN_CREDENTIALS        ((SECURITY_STATUS)0x130D)

//
// MessageId: SEC_E_NO_CREDENTIALS
//
// MessageText:
//
//  No credentials are available in the security package
//
#define SEC_E_NO_CREDENTIALS             ((SECURITY_STATUS)0x130E)

//
// MessageId: SEC_E_MESSAGE_ALTERED
//
// MessageText:
//
//  The message supplied for verification has been altered
//
#define SEC_E_MESSAGE_ALTERED            ((SECURITY_STATUS)0x130F)

//
// MessageId: SEC_E_OUT_OF_SEQUENCE
//
// MessageText:
//
//  The message supplied for verification is out of sequence
//
#define SEC_E_OUT_OF_SEQUENCE            ((SECURITY_STATUS)0x1310)

//
// MessageId: SEC_E_NO_AUTHENTICATING_AUTHORITY
//
// MessageText:
//
//  No authority could be contacted for authentication.
//
#define SEC_E_NO_AUTHENTICATING_AUTHORITY ((SECURITY_STATUS)0x1311)

// MessageId: SEC_E_CONTEXT_EXPIRED
//
// MessageText:
//
//  The context has expired and can no longer be used.
//
#define SEC_E_CONTEXT_EXPIRED            ((SECURITY_STATUS)0x1312)

//
// MessageId: SEC_E_INCOMPLETE_MESSAGE
//
// MessageText:
//
//  The supplied message is incomplete.  The signature was not verified.
//
#define SEC_E_INCOMPLETE_MESSAGE         ((SECURITY_STATUS)0x1313)

//
// MessageId: SEC_I_CONTINUE_NEEDED
//
// MessageText:
//
//  The function completed successfully, but must be called
//  again to complete the context
//
#define SEC_I_CONTINUE_NEEDED            ((SECURITY_STATUS)0x1012)

//
// MessageId: SEC_I_COMPLETE_NEEDED
//
// MessageText:
//
//  The function completed successfully, but CompleteToken
//  must be called
//
#define SEC_I_COMPLETE_NEEDED            ((SECURITY_STATUS)0x1013)

//
// MessageId: SEC_I_COMPLETE_AND_CONTINUE
//
// MessageText:
//
//  The function completed successfully, but both CompleteToken
//  and this function must be called to complete the context
//
#define SEC_I_COMPLETE_AND_CONTINUE      ((SECURITY_STATUS)0x1014)

//
// MessageId: SEC_I_LOCAL_LOGON
//
// MessageText:
//
//  The logon was completed, but no network authority was
//  available.  The logon was made using locally known information
//
#define SEC_I_LOCAL_LOGON                ((SECURITY_STATUS)0x1015)

//
// MessageId: SEC_E_OK
//
// MessageText:
//
//  Call completed successfully
//
#define SEC_E_OK                         ((SECURITY_STATUS)0x0000)

//
// Older error names for backwards compatibility
//


#define SEC_E_NOT_SUPPORTED              SEC_E_UNSUPPORTED_FUNCTION
#define SEC_E_NO_SPM                     SEC_E_INTERNAL_ERROR
#define SEC_E_BAD_PKGID                  SEC_E_SECPKG_NOT_FOUND



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _ISSPERR_H_
