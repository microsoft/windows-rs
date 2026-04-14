/*--

 Copyright (C) 2007 Microsoft Corporation

 Module Name:

     wsbapperror.h

 Abstract:

     This module contains the specific error codes returned by
     the COM interfaces implemented by the application to integrate 
     with Windows Server Backup

 --*/

#ifndef _WSBAPPERROR_H
#define _WSBAPPERROR_H

#define SEVERITY_OF(code)          ((code>>30) & 0x00000003)
#define FACILITY_OF(code)          ((code>>16) & 0x00000FFF)

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
#define FACILITY_WSBAPP                  122


//
// Define the severity codes
//


//
// MessageId: WSBAPP_E_COMPONENT_CONSISTENCY_CHECK_FAILED
//
// MessageText:
//
// The component consistency check failed.
//
#define WSBAPP_E_COMPONENT_CONSISTENCY_CHECK_FAILED ((HRESULT)-2139488255L)

//
// MessageId: WSBAPP_E_COMPONENT_PRE_RESTORE_FAILED
//
// MessageText:
//
// The pre-restore steps for the component failed.
//
#define WSBAPP_E_COMPONENT_PRE_RESTORE_FAILED ((HRESULT)-2139488254L)

//
// MessageId: WSBAPP_E_COMPONENT_POST_RESTORE_FAILED
//
// MessageText:
//
// The post-restore steps for the component failed.
//
#define WSBAPP_E_COMPONENT_POST_RESTORE_FAILED ((HRESULT)-2139488253L)

//
// MessageId: WSBAPP_ASYNC_IN_PROGRESS
//
// MessageText:
//
// An asynchronous operation is in progress.
//
#define WSBAPP_ASYNC_IN_PROGRESS         ((HRESULT)7995396L)

#endif // _WSBAPPERROR_H
