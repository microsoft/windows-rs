/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) 1991-1999  Microsoft Corporation

Module Name:

    oledserr.mc

Abstract:

    Error codes for ADs

Revision History:

--*/


#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


// ---------------------- HRESULT value definitions -----------------
//
// HRESULT definitions
//

#ifdef RC_INVOKED
#define _HRESULT_TYPEDEF_(_sc) _sc
#else // RC_INVOKED
#define _HRESULT_TYPEDEF_(_sc) ((HRESULT)_sc)
#endif // RC_INVOKED

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
#define FACILITY_WINDOWS                 8
#define FACILITY_STORAGE                 3
#define FACILITY_RPC                     1
#define FACILITY_SSPI                    9
#define FACILITY_WIN32                   7
#define FACILITY_CONTROL                 10
#define FACILITY_NULL                    0
#define FACILITY_ITF                     4
#define FACILITY_DISPATCH                2


//
// Define the severity codes
//


//
// MessageId: E_ADS_BAD_PATHNAME
//
// MessageText:
//
//  An invalid directory pathname was passed
//
#define E_ADS_BAD_PATHNAME               _HRESULT_TYPEDEF_(0x80005000L)

//
// MessageId: E_ADS_INVALID_DOMAIN_OBJECT
//
// MessageText:
//
//  An unknown directory domain object was requested
//
#define E_ADS_INVALID_DOMAIN_OBJECT      _HRESULT_TYPEDEF_(0x80005001L)

//
// MessageId: E_ADS_INVALID_USER_OBJECT
//
// MessageText:
//
//  An unknown directory user object was requested
//
#define E_ADS_INVALID_USER_OBJECT        _HRESULT_TYPEDEF_(0x80005002L)

//
// MessageId: E_ADS_INVALID_COMPUTER_OBJECT
//
// MessageText:
//
//  An unknown directory computer object was requested
//
#define E_ADS_INVALID_COMPUTER_OBJECT    _HRESULT_TYPEDEF_(0x80005003L)

//
// MessageId: E_ADS_UNKNOWN_OBJECT
//
// MessageText:
//
//  An unknown directory object was requested
//
#define E_ADS_UNKNOWN_OBJECT             _HRESULT_TYPEDEF_(0x80005004L)

//
// MessageId: E_ADS_PROPERTY_NOT_SET
//
// MessageText:
//
//  The specified directory property was not set
//
#define E_ADS_PROPERTY_NOT_SET           _HRESULT_TYPEDEF_(0x80005005L)

//
// MessageId: E_ADS_PROPERTY_NOT_SUPPORTED
//
// MessageText:
//
//  The specified directory property is not supported
//
#define E_ADS_PROPERTY_NOT_SUPPORTED     _HRESULT_TYPEDEF_(0x80005006L)

//
// MessageId: E_ADS_PROPERTY_INVALID
//
// MessageText:
//
//  The specified directory property is invalid
//
#define E_ADS_PROPERTY_INVALID           _HRESULT_TYPEDEF_(0x80005007L)

//
// MessageId: E_ADS_BAD_PARAMETER
//
// MessageText:
//
//  One or more input parameters are invalid
//
#define E_ADS_BAD_PARAMETER              _HRESULT_TYPEDEF_(0x80005008L)

//
// MessageId: E_ADS_OBJECT_UNBOUND
//
// MessageText:
//
//  The specified directory object is not bound to a remote resource
//
#define E_ADS_OBJECT_UNBOUND             _HRESULT_TYPEDEF_(0x80005009L)

//
// MessageId: E_ADS_PROPERTY_NOT_MODIFIED
//
// MessageText:
//
//  The specified directory object has not been modified
//
#define E_ADS_PROPERTY_NOT_MODIFIED      _HRESULT_TYPEDEF_(0x8000500AL)

//
// MessageId: E_ADS_PROPERTY_MODIFIED
//
// MessageText:
//
//  The specified directory object has been modified
//
#define E_ADS_PROPERTY_MODIFIED          _HRESULT_TYPEDEF_(0x8000500BL)

//
// MessageId: E_ADS_CANT_CONVERT_DATATYPE
//
// MessageText:
//
//  The directory datatype cannot be converted to/from a native DS datatype
//
#define E_ADS_CANT_CONVERT_DATATYPE      _HRESULT_TYPEDEF_(0x8000500CL)

//
// MessageId: E_ADS_PROPERTY_NOT_FOUND
//
// MessageText:
//
//  The directory property cannot be found in the cache.
//
#define E_ADS_PROPERTY_NOT_FOUND         _HRESULT_TYPEDEF_(0x8000500DL)

//
// MessageId: E_ADS_OBJECT_EXISTS
//
// MessageText:
//
//  The directory object exists.
//
#define E_ADS_OBJECT_EXISTS              _HRESULT_TYPEDEF_(0x8000500EL)

//
// MessageId: E_ADS_SCHEMA_VIOLATION
//
// MessageText:
//
//  The attempted action violates the DS schema rules.
//
#define E_ADS_SCHEMA_VIOLATION           _HRESULT_TYPEDEF_(0x8000500FL)

//
// MessageId: E_ADS_COLUMN_NOT_SET
//
// MessageText:
//
//  The specified column in the directory was not set.
//
#define E_ADS_COLUMN_NOT_SET             _HRESULT_TYPEDEF_(0x80005010L)

//
// MessageId: S_ADS_ERRORSOCCURRED
//
// MessageText:
//
//  One or more errors occurred
//
#define S_ADS_ERRORSOCCURRED             _HRESULT_TYPEDEF_(0x00005011L)

//
// MessageId: S_ADS_NOMORE_ROWS
//
// MessageText:
//
//  No more rows to be obatained by the search result.
//
#define S_ADS_NOMORE_ROWS                _HRESULT_TYPEDEF_(0x00005012L)

//
// MessageId: S_ADS_NOMORE_COLUMNS
//
// MessageText:
//
//  No more columns to be obatained for the current row.
//
#define S_ADS_NOMORE_COLUMNS             _HRESULT_TYPEDEF_(0x00005013L)

//
// MessageId: E_ADS_INVALID_FILTER
//
// MessageText:
//
//  The search filter specified is invalid
//
#define E_ADS_INVALID_FILTER             _HRESULT_TYPEDEF_(0x80005014L)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

