/*--

Copyright (c) 2008  Microsoft Corporation

Module Name:

    EhStorMsg.h

Abstract:

    This file contains the message definitions for Enhanced Storage APIs.

Revision History:

--*/

#pragma once

//
// --------------------------------------------------------------------------
// Enhanced Storage error codes will use FACILITY_ITF. According to MSDN doc,
// it is recommended that code values start at 0x0200 as COM FACILITY_ITF
// will use the code values from 0x0000 - 0x01FF.
//
// The code value range is partition based on the following scheme:
// 
// 0x0200 - 0x03FF: general error codes
// 0x0400 - 0x04FF: authentication related error codes
// 0x0500 - 0x0FFF: reserved for other high level feature related error codes
// 0x1100 - 0x11FF: password silo related error codes
// 0x1200 - 0xBFFF: reserved for other silo specific error codes
// 0xC000 - 0xFFFF: reserved for 3rd party silo specific error codes
// 
// --------------------------------------------------------------------------
//

#define ES_RESERVED_COM_ERROR_START            0x0000
#define ES_RESERVED_COM_ERROR_END              0x01FF
#define ES_GENERAL_ERROR_START                 0x0200
#define ES_GENERAL_ERROR_END                   0x03FF
#define ES_AUTHN_ERROR_START                   0x0400
#define ES_AUTHN_ERROR_END                     0x04FF
#define ES_RESERVED_SILO_ERROR_START           0x0500
#define ES_RESERVED_SILO_ERROR_END             0x0FFF
#define ES_PW_SILO_ERROR_START                 0x1100
#define ES_PW_SILO_ERROR_END                   0x11FF
#define ES_RESERVED_SILO_SPECIFIC_ERROR_START  0x1200
#define ES_RESERVED_SILO_SPECIFIC_ERROR_END    0xBFFF
#define ES_VENDOR_ERROR_START                  0xC000
#define ES_VENDOR_ERROR_END                    0xFFFF

// -----------------------------------
// Error code related macros
// -----------------------------------

#define IS_ENHANCED_STORAGE_GENERAL_ERROR(x)     ((x) >= ES_GENERAL_ERROR_START && \
                                                (x) <= ES_GENERAL_ERROR_END)

#define IS_ENHANCED_STORAGE_AUTHN_ERROR(x)       ((x) >= ES_AUTHN_ERROR_START && \
                                                (x) <= ES_AUTHN_ERROR_END)

#define IS_ENHANCED_STORAGE_PW_SILO_ERROR(x)     ((x) >= ES_PW_SILO_ERROR_START && \
                                                (x) <= ES_PW_SILO_ERROR_END)

#define IS_ENHANCED_STORAGE_VENDOR_ERROR(x)      ((x) >= ES_VENDOR_ERROR_START && \
                                                (x) <= ES_VENDOR_ERROR_END)

#define IS_ENHANCED_STORAGE_RESERVED_ERROR(x)    (((x) >= ES_RESERVED_SILO_ERROR_START && \
                                                 (x) <= ES_RESERVED_SILO_ERROR_END) || \
                                                ((x) >= ES_RESERVED_SILO_SPECIFIC_ERROR_START && \
                                                 (x) <= ES_RESERVED_SILO_SPECIFIC_ERROR_END) || \
                                                ((x) >= ES_RESERVED_COM_ERROR_START && \
                                                 (x) <= ES_RESERVED_COM_ERROR_END))

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
#define FACILITY_ENHANCED_STORAGE        0x4


//
// Define the severity codes
//
#define STATUS_SEVERITY_SUCCESS          0x0
#define STATUS_SEVERITY_INFORMATIONAL    0x1
#define STATUS_SEVERITY_WARNING          0x2
#define STATUS_SEVERITY_ERROR            0x3


//
// MessageId: ES_E_INVALID_RESPONSE
//
// MessageText:
//
// Hardware return invalid response.
//
#define ES_E_INVALID_RESPONSE            ((DWORD)0xC0040200L)

//
// MessageId: ES_E_UNPROVISIONED_HARDWARE
//
// MessageText:
//
// Hardware is not provision.
//
#define ES_E_UNPROVISIONED_HARDWARE      ((DWORD)0xC0040204L)

//
// MessageId: ES_E_UNSUPPORTED_HARDWARE
//
// MessageText:
//
// Hardware is not supported.
//
#define ES_E_UNSUPPORTED_HARDWARE        ((DWORD)0xC0040205L)

//
// MessageId: ES_E_INCOMPLETE_COMMAND
//
// MessageText:
//
// An incomplete command was received.
//
#define ES_E_INCOMPLETE_COMMAND          ((DWORD)0xC0040206L)

//
// MessageId: ES_E_BAD_SEQUENCE
//
// MessageText:
//
// Command rejected for being out of sequence.
//
#define ES_E_BAD_SEQUENCE                ((DWORD)0xC0040207L)

//
// MessageId: ES_E_NO_PROBE
//
// MessageText:
//
// Non-Probe command received before Probe command.
//
#define ES_E_NO_PROBE                    ((DWORD)0xC0040208L)

//
// MessageId: ES_E_INVALID_SILO
//
// MessageText:
//
// Invalid silo specified.
//
#define ES_E_INVALID_SILO                ((DWORD)0xC0040209L)

//
// MessageId: ES_E_INVALID_CAPABILITY
//
// MessageText:
//
// Invalid capability requested.
//
#define ES_E_INVALID_CAPABILITY          ((DWORD)0xC004020AL)

//
// MessageId: ES_E_GROUP_POLICY_FORBIDDEN_USE
//
// MessageText:
//
// Group policy setting forbids use.
//
#define ES_E_GROUP_POLICY_FORBIDDEN_USE  ((DWORD)0xC004020BL)

//
// MessageId: ES_E_GROUP_POLICY_FORBIDDEN_OPERATION
//
// MessageText:
//
// Group policy setting forbids operation.
//
#define ES_E_GROUP_POLICY_FORBIDDEN_OPERATION ((DWORD)0xC004020CL)

//
// MessageId: ES_E_INVALID_PARAM_COMBINATION
//
// MessageText:
//
// Invalid combination of parameters specified in input data.
//
#define ES_E_INVALID_PARAM_COMBINATION   ((DWORD)0xC004020DL)

//
// MessageId: ES_E_INVALID_PARAM_LENGTH
//
// MessageText:
//
// Parameter Byte Length specified in the payload is invalid.
//
#define ES_E_INVALID_PARAM_LENGTH        ((DWORD)0xC004020EL)

//
// MessageId: ES_E_INCONSISTENT_PARAM_LENGTH
//
// MessageText:
//
// Parameter Byte Length specified in the payload is not consistent with the number of bytes transferred.
//
#define ES_E_INCONSISTENT_PARAM_LENGTH   ((DWORD)0xC004020FL)


// -----------------------------------------
// Authentication silo related error codes
// -----------------------------------------

//
// MessageId: ES_E_NO_AUTHENTICATION_REQUIRED
//
// MessageText:
//
// Hardware does not require authentication.
//
#define ES_E_NO_AUTHENTICATION_REQUIRED  ((DWORD)0xC0040400L)


// -----------------------------------
// Password silo related error codes
// -----------------------------------

//
// MessageId: ES_E_INVALID_FIELD_IDENTIFIER
//
// MessageText:
//
// An invalid field identifier was found in the data provided by the device.
//
#define ES_E_INVALID_FIELD_IDENTIFIER    ((DWORD)0xC0041100L)

//
// MessageId: ES_E_CHALLENGE_MISMATCH
//
// MessageText:
//
// The challenge provided by the device does not match the one provided in an earlier step in the authentication sequence.
//
#define ES_E_CHALLENGE_MISMATCH          ((DWORD)0xC0041101L)

//
// MessageId: ES_E_CHALLENGE_SIZE_MISMATCH
//
// MessageText:
//
// The size of the challenge specified by the device does not match the choice of the digest algorithm.
//
#define ES_E_CHALLENGE_SIZE_MISMATCH     ((DWORD)0xC0041102L)

//
// MessageId: ES_E_FRIENDLY_NAME_TOO_LONG
//
// MessageText:
//
// The friendly name specified for User is too long.
//
#define ES_E_FRIENDLY_NAME_TOO_LONG      ((DWORD)0xC0041103L)

//
// MessageId: ES_E_SILO_NAME_TOO_LONG
//
// MessageText:
//
// The silo name specified is too long.
//
#define ES_E_SILO_NAME_TOO_LONG          ((DWORD)0xC0041104L)

//
// MessageId: ES_E_PASSWORD_TOO_LONG
//
// MessageText:
//
// The password specified is too long.
//
#define ES_E_PASSWORD_TOO_LONG           ((DWORD)0xC0041105L)

//
// MessageId: ES_E_PASSWORD_HINT_TOO_LONG
//
// MessageText:
//
// The password hint specified is too long.
//
#define ES_E_PASSWORD_HINT_TOO_LONG      ((DWORD)0xC0041106L)

//
// MessageId: ES_E_OTHER_SECURITY_PROTOCOL_ACTIVE
//
// MessageText:
//
// Cannot enable IEEE 1667 password security since another security protocol is still active on the device.
//
#define ES_E_OTHER_SECURITY_PROTOCOL_ACTIVE ((DWORD)0xC0041107L)

//
// MessageId: ES_E_DEVICE_DIGEST_MISSING
//
// MessageText:
//
// Device digest was expected but was not found in the payload.
//
#define ES_E_DEVICE_DIGEST_MISSING       ((DWORD)0xC0041108L)

//
// MessageId: ES_E_NOT_AUTHORIZED_UNEXPECTED
//
// MessageText:
//
// Expected the silo to be in Authorized state, but it was not.
//
#define ES_E_NOT_AUTHORIZED_UNEXPECTED   ((DWORD)0xC0041109L)

//
// MessageId: ES_E_AUTHORIZED_UNEXPECTED
//
// MessageText:
//
// Expected the silo to be in Not Authorized state, but it was not.
//
#define ES_E_AUTHORIZED_UNEXPECTED       ((DWORD)0xC004110AL)

//
// MessageId: ES_E_PROVISIONED_UNEXPECTED
//
// MessageText:
//
// Expected the silo to be in Not Provisioned state, but it was not.
//
#define ES_E_PROVISIONED_UNEXPECTED      ((DWORD)0xC004110BL)

//
// MessageId: ES_E_UNKNOWN_DIGEST_ALGORITHM
//
// MessageText:
//
// The digest algorithm supported by the device is not supported in Windows.
//
#define ES_E_UNKNOWN_DIGEST_ALGORITHM    ((DWORD)0xC004110CL)

