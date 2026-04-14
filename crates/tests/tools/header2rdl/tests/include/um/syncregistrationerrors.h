//--------------------------------------------------------------------------
//    Copyright (c) Microsoft Corporation.
//
//    @File: syncregistrationerrors.h
//
//    Purpose:  Error Messages for Microsoft Synchronization Platform registration
//
//
//---------------------------------------------------------------------------
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


//
// Define the severity codes
//


//
// MessageId: SYNC_E_REGISTRATION_INTERNAL_ERROR
//
// MessageText:
//
// An internal error occurred in the sync registration runtime.
//
#define SYNC_E_REGISTRATION_INTERNAL_ERROR 0x80041600L

//
// MessageId: SYNC_E_REGISTRATION_ALREADY_REGISTERED
//
// MessageText:
//
// The requested item has already been registered.
//
#define SYNC_E_REGISTRATION_ALREADY_REGISTERED 0x80041601L

//
// MessageId: SYNC_E_REGISTRATION_NOT_REGISTERED
//
// MessageText:
//
// The requested item does not exist in the registration store.
//
#define SYNC_E_REGISTRATION_NOT_REGISTERED 0x80041602L

//
// MessageId: SYNC_E_REGISTRATION_CORRUPT_DATA
//
// MessageText:
//
// The data for this item is corrupted and cannot be read.
//
#define SYNC_E_REGISTRATION_CORRUPT_DATA 0x80041603L

//
// MessageId: SYNC_E_REGISTRATION_ALREADY_COMMITTED
//
// MessageText:
//
// The properties for this item have already been committed on another instance.
//
#define SYNC_E_REGISTRATION_ALREADY_COMMITTED 0x80041604L

