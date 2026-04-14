/*++ BUILD Version: 0005    // Increment this if a change has global effects

Copyright (c) 1989-2002  Microsoft Corporation.  All rights reserved.

Module Name:

    fltWinError.h

Abstract:

    Constant definitions for the HRESULTS values defined by the Filter Manager.

Environment:

    User mode

--*/

#ifndef _FLT_WINERROR_
#define _FLT_WINERROR_

//
// For Windows version 6.00 and later, these error codes are defined in
// winerror.h.  Only use these definitions if not already defined
//
#if NTDDI_VERSION < NTDDI_VISTA
#ifndef FACILITY_USERMODE_FILTER_MANAGER

//
//  HRESULT
//  FILTER_HRESULT_FROM_FLT_NTSTATUS (
//      IN NTSTATUS FltNtStatus
//      )
//
//  Macro Description:
//
//      This macro does the translation from a Filter Manager defined NTSTATUS
//      code to a Filter Manager Library HRESULT.  The Filter Manager Library
//      error code is built as follows:
//
//       3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//       1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//      +-+-+-+-+-----------------------+-------------------------------+
//      |S|R|C|R|   FltMgr Facility     |(Code part of FltNtStatus)     |
//      | | | | |                       |     bit-wise or'd with 0x0000 |
//      +-+-+-+-+-----------------------+-------------------------------+
//
//      where
//
//         S - Severity - 1 to indicate FAILURE
//
//         R - reserved portion of the facility code, corresponds to NT's
//             second severity bit.
//
//         C - is the Customer code flag
//
//         R - is a reserved bit
//
//         Facility - FACILITY_USERMODE_FILTER_MANAGER
//
//         Code - Code portion of the NTSTATUS
//
//  Arguments:
//
//      FltNtStatus - The NTSTATUS error code with the Filter Manager facility
//          code to translate to an Filter Manager Library HRESULT.
//
//  Return Value:
//
//      The appropriate HRESULT.
//

#define FILTER_HRESULT_FROM_FLT_NTSTATUS(x) (NT_ASSERT((x & 0xfff0000) == 0x001c0000),(HRESULT) (((x) & 0x8000FFFF) | (FACILITY_USERMODE_FILTER_MANAGER << 16)))


//////////////////////////////////////////////////////////////////////
//
//  HRESULTs for Filter Manager defined NTSTATUS codes
//
//////////////////////////////////////////////////////////////////////

//
//  Values are 32 bit values laid out as follows:
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
#define FACILITY_USERMODE_FILTER_MANAGER 0x1F


//
// Define the severity codes
//


//
// MessageId: ERROR_FLT_IO_COMPLETE
//
// MessageText:
//
// The IO was completed by a filter.
//
#define ERROR_FLT_IO_COMPLETE            ((HRESULT)0x001F0001L)

//
// MessageId: ERROR_FLT_NO_HANDLER_DEFINED
//
// MessageText:
//
// A handler was not defined by the filter for this operation.
//
#define ERROR_FLT_NO_HANDLER_DEFINED     ((HRESULT)0x801F0001L)

//
// MessageId: ERROR_FLT_CONTEXT_ALREADY_DEFINED
//
// MessageText:
//
// A context is already defined for this object.
//
#define ERROR_FLT_CONTEXT_ALREADY_DEFINED ((HRESULT)0x801F0002L)

//
// MessageId: ERROR_FLT_INVALID_ASYNCHRONOUS_REQUEST
//
// MessageText:
//
// Asynchronous requests are not valid for this operation.
//
#define ERROR_FLT_INVALID_ASYNCHRONOUS_REQUEST ((HRESULT)0x801F0003L)

//
// MessageId: ERROR_FLT_DISALLOW_FAST_IO
//
// MessageText:
//
// Disallow the Fast IO path for this operation.
//
#define ERROR_FLT_DISALLOW_FAST_IO       ((HRESULT)0x801F0004L)

//
// MessageId: ERROR_FLT_INVALID_NAME_REQUEST
//
// MessageText:
//
// An invalid name request was made.  The name requested cannot be retrieved at this time.
//
#define ERROR_FLT_INVALID_NAME_REQUEST   ((HRESULT)0x801F0005L)

//
// MessageId: ERROR_FLT_NOT_SAFE_TO_POST_OPERATION
//
// MessageText:
//
// Posting this operation to a worker thread for further processing is not safe
// at this time because it could lead to a system deadlock.
//
#define ERROR_FLT_NOT_SAFE_TO_POST_OPERATION ((HRESULT)0x801F0006L)

//
// MessageId: ERROR_FLT_NOT_INITIALIZED
//
// MessageText:
//
// The Filter Manager was not initialized when a filter tried to register.  Make
// sure that the Filter Manager is getting loaded as a driver.
//
#define ERROR_FLT_NOT_INITIALIZED        ((HRESULT)0x801F0007L)

//
// MessageId: ERROR_FLT_FILTER_NOT_READY
//
// MessageText:
//
// The filter is not ready for attachment to volumes because it has not finished
// initializing (FltStartFiltering has not been called).
//
#define ERROR_FLT_FILTER_NOT_READY       ((HRESULT)0x801F0008L)

//
// MessageId: ERROR_FLT_POST_OPERATION_CLEANUP
//
// MessageText:
//
// The filter must cleanup any operation specific context at this time because
// it is being removed from the system before the operation is completed by
// the lower drivers.
//
#define ERROR_FLT_POST_OPERATION_CLEANUP ((HRESULT)0x801F0009L)

//
// MessageId: ERROR_FLT_INTERNAL_ERROR
//
// MessageText:
//
// The Filter Manager had an internal error from which it cannot recover,
// therefore the operation has been failed.  This is usually the result
// of a filter returning an invalid value from a pre-operation callback.
//
#define ERROR_FLT_INTERNAL_ERROR         ((HRESULT)0x801F000AL)

//
// MessageId: ERROR_FLT_DELETING_OBJECT
//
// MessageText:
//
// The object specified for this action is in the process of being
// deleted, therefore the action requested cannot be completed at
// this time.
//
#define ERROR_FLT_DELETING_OBJECT        ((HRESULT)0x801F000BL)

//
// MessageId: ERROR_FLT_MUST_BE_NONPAGED_POOL
//
// MessageText:
//
// Non-paged pool must be used for this type of context.
//
#define ERROR_FLT_MUST_BE_NONPAGED_POOL  ((HRESULT)0x801F000CL)

//
// MessageId: ERROR_FLT_DUPLICATE_ENTRY
//
// MessageText:
//
// A duplicate handler definition has been provided for an operation.
//
#define ERROR_FLT_DUPLICATE_ENTRY        ((HRESULT)0x801F000DL)

//
// MessageId: ERROR_FLT_CBDQ_DISABLED
//
// MessageText:
//
// The callback data queue has been disabled.
//
#define ERROR_FLT_CBDQ_DISABLED          ((HRESULT)0x801F000EL)

//
// MessageId: ERROR_FLT_DO_NOT_ATTACH
//
// MessageText:
//
// Do not attach the filter to the volume at this time.
//
#define ERROR_FLT_DO_NOT_ATTACH          ((HRESULT)0x801F000FL)

//
// MessageId: ERROR_FLT_DO_NOT_DETACH
//
// MessageText:
//
// Do not detach the filter from the volume at this time.
//
#define ERROR_FLT_DO_NOT_DETACH          ((HRESULT)0x801F0010L)

//
// MessageId: ERROR_FLT_INSTANCE_ALTITUDE_COLLISION
//
// MessageText:
//
// An instance already exists at this altitude on the volume specified.
//
#define ERROR_FLT_INSTANCE_ALTITUDE_COLLISION ((HRESULT)0x801F0011L)

//
// MessageId: ERROR_FLT_INSTANCE_NAME_COLLISION
//
// MessageText:
//
// An instance already exists with this name on the volume specified.
//
#define ERROR_FLT_INSTANCE_NAME_COLLISION ((HRESULT)0x801F0012L)

//
// MessageId: ERROR_FLT_FILTER_NOT_FOUND
//
// MessageText:
//
// The system could not find the filter specified.
//
#define ERROR_FLT_FILTER_NOT_FOUND       ((HRESULT)0x801F0013L)

//
// MessageId: ERROR_FLT_VOLUME_NOT_FOUND
//
// MessageText:
//
// The system could not find the volume specified.
//
#define ERROR_FLT_VOLUME_NOT_FOUND       ((HRESULT)0x801F0014L)

//
// MessageId: ERROR_FLT_INSTANCE_NOT_FOUND
//
// MessageText:
//
// The system could not find the instance specified.
//
#define ERROR_FLT_INSTANCE_NOT_FOUND     ((HRESULT)0x801F0015L)

//
// MessageId: ERROR_FLT_CONTEXT_ALLOCATION_NOT_FOUND
//
// MessageText:
//
// No registered context allocation definition was found for the given request.
//
#define ERROR_FLT_CONTEXT_ALLOCATION_NOT_FOUND ((HRESULT)0x801F0016L)

//
// MessageId: ERROR_FLT_INVALID_CONTEXT_REGISTRATION
//
// MessageText:
//
// An invalid parameter was specified during context registration.
//
#define ERROR_FLT_INVALID_CONTEXT_REGISTRATION ((HRESULT)0x801F0017L)

//
// MessageId: ERROR_FLT_NAME_CACHE_MISS
//
// MessageText:
//
// The name requested was not found in Filter Manager's name cache and could not be retrieved from the file system.
//
#define ERROR_FLT_NAME_CACHE_MISS        ((HRESULT)0x801F0018L)

//
// MessageId: ERROR_FLT_NO_DEVICE_OBJECT
//
// MessageText:
//
// The requested device object does not exist for the given volume.
//
#define ERROR_FLT_NO_DEVICE_OBJECT       ((HRESULT)0x801F0019L)

//
// MessageId: ERROR_FLT_VOLUME_ALREADY_MOUNTED
//
// MessageText:
//
// The specified volume is already mounted.
//
#define ERROR_FLT_VOLUME_ALREADY_MOUNTED ((HRESULT)0x801F001AL)

//
// MessageId: ERROR_FLT_NO_WAITER_FOR_REPLY
//
// MessageText:
//
// No waiter is present for the filter's reply to this message.
//
#define ERROR_FLT_NO_WAITER_FOR_REPLY    ((HRESULT)0x801F0020L)

#endif // !FACILITY_USERMODE_FILTER_MANAGER
#endif //NTDDIVER < WIN_LH
#endif //_FLT_WINERROR_

