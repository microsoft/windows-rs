/*++

Copyright (c) Microsoft Corporation.  All Rights Reserved

Abstract:

   This is the header file defining the error codes 
   returned by Windows Server Backup APIs that provide
   extensibility to 3rd party online backup applications

--*/


#if (NTDDI_VERSION >= NTDDI_WIN8)
#ifndef _WSBONLINEERROR_H
#define _WSBONLINEERROR_H

#define SEVERITY_OF(code)          ((code>>30) & 0x00000003)
#define FACILITY_OF(code)          ((code>>16) & 0x00000FFF)

/***************************************************************************/
/*                           Success Codes                                 */
/***************************************************************************/
//
//  Values are 32 bit values laid out as follows:
//
//   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//  +-+-+-+-+-+---------------------+-------------------------------+
//  |S|R|C|N|r|    Facility         |               Code            |
//  +-+-+-+-+-+---------------------+-------------------------------+
//
//  where
//
//      S - Severity - indicates success/fail
//
//          0 - Success
//          1 - Fail (COERROR)
//
//      R - reserved portion of the facility code, corresponds to NT's
//              second severity bit.
//
//      C - reserved portion of the facility code, corresponds to NT's
//              C field.
//
//      N - reserved portion of the facility code. Used to indicate a
//              mapped NT status value.
//
//      r - reserved portion of the facility code. Reserved for internal
//              use. Used to indicate HRESULT values that are not status
//              values, but are instead message ids for display strings.
//
//      Facility - is the facility code
//
//      Code - is the facility's status code
//
//
// Define the facility codes
//
#define FACILITY_WSB_ONLINE              133


//
// Define the severity codes
//


//
// MessageId: WSB_ONLINE_INVALID_PARAMETER_ERROR
//
// MessageText:
//
// The information passed to Windows Server Backup is invalid.
//
#define WSB_ONLINE_INVALID_PARAMETER_ERROR ((HRESULT)-2138767359L)

//
// MessageId: WSB_ONLINE_REGISTRATION_ALREADY_EXISTS
//
// MessageText:
//
// Another Online Backup solution is already registered with Windows Server Backup.
//
#define WSB_ONLINE_REGISTRATION_ALREADY_EXISTS ((HRESULT)-2138767358L)

//
// MessageId: WSB_ONLINE_REGISTRATION_MISMATCH
//
// MessageText:
//
// Snapin identifier does not match with Online Backup registered with Windows Server Backup.
//
#define WSB_ONLINE_REGISTRATION_MISMATCH ((HRESULT)-2138767357L)

#endif // _WSBONLINEERROR_H
#endif // NTDDI_VERSION >= NTDDI_WIN8
