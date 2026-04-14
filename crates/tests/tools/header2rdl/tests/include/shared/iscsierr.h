/*++

Copyright (c) 2001  Microsoft Corporation

Module Name:

    iscsierr.h

Abstract:

    Constant definitions for the IScsi discover error codes

Revision History:

--*/

#ifndef _ISCSIERR_
#define _ISCSIERR_

//
//  Status values are 32 bit values layed out as follows:
//
//   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//  +---+-+-------------------------+-------------------------------+
//  |Sev|C|       Facility          |               Code            |
//  +---+-+-------------------------+-------------------------------+
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
//      Facility - is the facility code
//
//      Code - is the facility's status code
//

//
// Error status code for ISCSI discovery apis. Error codes can be a 
// standard Windows error code as defined in Winerror.h or one of the
// iscsi discovery specific error codes defined below.
//
#ifndef MOFCOMP_PASS
typedef ULONG ISDSC_STATUS;
#endif

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
#define STATUS_SEVERITY_SUCCESS          0x0
#define STATUS_SEVERITY_INFORMATIONAL    0x1
#define STATUS_SEVERITY_WARNING          0x2
#define STATUS_SEVERITY_ERROR            0x3


//
// MessageId: ISDSC_NON_SPECIFIC_ERROR
//
// MessageText:
//
// A non specific error occurred.
//
#define ISDSC_NON_SPECIFIC_ERROR         ((ISDSC_STATUS)0xEFFF0001L)

//
// MessageId: ISDSC_LOGIN_FAILED
//
// MessageText:
//
// Login Failed.
//
#define ISDSC_LOGIN_FAILED               ((ISDSC_STATUS)0xEFFF0002L)

//
// MessageId: ISDSC_CONNECTION_FAILED
//
// MessageText:
//
// Connection Failed.
//
#define ISDSC_CONNECTION_FAILED          ((ISDSC_STATUS)0xEFFF0003L)

//
// MessageId: ISDSC_INITIATOR_NODE_ALREADY_EXISTS
//
// MessageText:
//
// Initiator Node Already Exists.
//
#define ISDSC_INITIATOR_NODE_ALREADY_EXISTS ((ISDSC_STATUS)0xEFFF0004L)

//
// MessageId: ISDSC_INITIATOR_NODE_NOT_FOUND
//
// MessageText:
//
// Initiator Node Does Not Exist.
//
#define ISDSC_INITIATOR_NODE_NOT_FOUND   ((ISDSC_STATUS)0xEFFF0005L)

//
// MessageId: ISDSC_TARGET_MOVED_TEMPORARILY
//
// MessageText:
//
// Target Moved Temporarily.
//
#define ISDSC_TARGET_MOVED_TEMPORARILY   ((ISDSC_STATUS)0xEFFF0006L)

//
// MessageId: ISDSC_TARGET_MOVED_PERMANENTLY
//
// MessageText:
//
// Target Moved Permanently.
//
#define ISDSC_TARGET_MOVED_PERMANENTLY   ((ISDSC_STATUS)0xEFFF0007L)

//
// MessageId: ISDSC_INITIATOR_ERROR
//
// MessageText:
//
// Initiator Error.
//
#define ISDSC_INITIATOR_ERROR            ((ISDSC_STATUS)0xEFFF0008L)

//
// MessageId: ISDSC_AUTHENTICATION_FAILURE
//
// MessageText:
//
// Authentication Failure.
//
#define ISDSC_AUTHENTICATION_FAILURE     ((ISDSC_STATUS)0xEFFF0009L)

//
// MessageId: ISDSC_AUTHORIZATION_FAILURE
//
// MessageText:
//
// Authorization Failure.
//
#define ISDSC_AUTHORIZATION_FAILURE      ((ISDSC_STATUS)0xEFFF000AL)

//
// MessageId: ISDSC_NOT_FOUND
//
// MessageText:
//
// Not Found.
//
#define ISDSC_NOT_FOUND                  ((ISDSC_STATUS)0xEFFF000BL)

//
// MessageId: ISDSC_TARGET_REMOVED
//
// MessageText:
//
// Target Removed.
//
#define ISDSC_TARGET_REMOVED             ((ISDSC_STATUS)0xEFFF000CL)

//
// MessageId: ISDSC_UNSUPPORTED_VERSION
//
// MessageText:
//
// Unsupported Version.
//
#define ISDSC_UNSUPPORTED_VERSION        ((ISDSC_STATUS)0xEFFF000DL)

//
// MessageId: ISDSC_TOO_MANY_CONNECTIONS
//
// MessageText:
//
// Too many Connections.
//
#define ISDSC_TOO_MANY_CONNECTIONS       ((ISDSC_STATUS)0xEFFF000EL)

//
// MessageId: ISDSC_MISSING_PARAMETER
//
// MessageText:
//
// Missing Parameter.
//
#define ISDSC_MISSING_PARAMETER          ((ISDSC_STATUS)0xEFFF000FL)

//
// MessageId: ISDSC_CANT_INCLUDE_IN_SESSION
//
// MessageText:
//
// Can not include in session.
//
#define ISDSC_CANT_INCLUDE_IN_SESSION    ((ISDSC_STATUS)0xEFFF0010L)

//
// MessageId: ISDSC_SESSION_TYPE_NOT_SUPPORTED
//
// MessageText:
//
// Session type not supported.
//
#define ISDSC_SESSION_TYPE_NOT_SUPPORTED ((ISDSC_STATUS)0xEFFF0011L)

//
// MessageId: ISDSC_TARGET_ERROR
//
// MessageText:
//
// Target Error.
//
#define ISDSC_TARGET_ERROR               ((ISDSC_STATUS)0xEFFF0012L)

//
// MessageId: ISDSC_SERVICE_UNAVAILABLE
//
// MessageText:
//
// Service Unavailable.
//
#define ISDSC_SERVICE_UNAVAILABLE        ((ISDSC_STATUS)0xEFFF0013L)

//
// MessageId: ISDSC_OUT_OF_RESOURCES
//
// MessageText:
//
// Out of Resources.
//
#define ISDSC_OUT_OF_RESOURCES           ((ISDSC_STATUS)0xEFFF0014L)

//
// MessageId: ISDSC_CONNECTION_ALREADY_EXISTS
//
// MessageText:
//
// Connections already exist on initiator node.
//
#define ISDSC_CONNECTION_ALREADY_EXISTS  ((ISDSC_STATUS)0xEFFF0015L)

//
// MessageId: ISDSC_SESSION_ALREADY_EXISTS
//
// MessageText:
//
// Session Already Exists.
//
#define ISDSC_SESSION_ALREADY_EXISTS     ((ISDSC_STATUS)0xEFFF0016L)

//
// MessageId: ISDSC_INITIATOR_INSTANCE_NOT_FOUND
//
// MessageText:
//
// Initiator Instance Does Not Exist.
//
#define ISDSC_INITIATOR_INSTANCE_NOT_FOUND ((ISDSC_STATUS)0xEFFF0017L)

//
// MessageId: ISDSC_TARGET_ALREADY_EXISTS
//
// MessageText:
//
// Target Already Exists.
//
#define ISDSC_TARGET_ALREADY_EXISTS      ((ISDSC_STATUS)0xEFFF0018L)

//
// MessageId: ISDSC_DRIVER_BUG
//
// MessageText:
//
// The iscsi driver implementation did not complete an operation correctly.
//
#define ISDSC_DRIVER_BUG                 ((ISDSC_STATUS)0xEFFF0019L)

//
// MessageId: ISDSC_INVALID_TEXT_KEY
//
// MessageText:
//
// An invalid key text was encountered.
//
#define ISDSC_INVALID_TEXT_KEY           ((ISDSC_STATUS)0xEFFF001AL)

//
// MessageId: ISDSC_INVALID_SENDTARGETS_TEXT
//
// MessageText:
//
// Invalid SendTargets response text was encountered.
//
#define ISDSC_INVALID_SENDTARGETS_TEXT   ((ISDSC_STATUS)0xEFFF001BL)

//
// MessageId: ISDSC_INVALID_SESSION_ID
//
// MessageText:
//
// Invalid Session Id.
//
#define ISDSC_INVALID_SESSION_ID         ((ISDSC_STATUS)0xEFFF001CL)

//
// MessageId: ISDSC_SCSI_REQUEST_FAILED
//
// MessageText:
//
// The scsi request failed.
//
#define ISDSC_SCSI_REQUEST_FAILED        ((ISDSC_STATUS)0xEFFF001DL)

//
// MessageId: ISDSC_TOO_MANY_SESSIONS
//
// MessageText:
//
// Exceeded max sessions for this initiator.
//
#define ISDSC_TOO_MANY_SESSIONS          ((ISDSC_STATUS)0xEFFF001EL)

//
// MessageId: ISDSC_SESSION_BUSY
//
// MessageText:
//
// Session is busy since a request is already in progress.
//
#define ISDSC_SESSION_BUSY               ((ISDSC_STATUS)0xEFFF001FL)

//
// MessageId: ISDSC_TARGET_MAPPING_UNAVAILABLE
//
// MessageText:
//
// The target mapping requested is not available.
//
#define ISDSC_TARGET_MAPPING_UNAVAILABLE ((ISDSC_STATUS)0xEFFF0020L)

//
// MessageId: ISDSC_ADDRESS_TYPE_NOT_SUPPORTED
//
// MessageText:
//
// The Target Address type given is not supported.
//
#define ISDSC_ADDRESS_TYPE_NOT_SUPPORTED ((ISDSC_STATUS)0xEFFF0021L)

//
// MessageId: ISDSC_LOGON_FAILED
//
// MessageText:
//
// Logon Failed.
//
#define ISDSC_LOGON_FAILED               ((ISDSC_STATUS)0xEFFF0022L)

//
// MessageId: ISDSC_SEND_FAILED
//
// MessageText:
//
// TCP Send Failed.
//
#define ISDSC_SEND_FAILED                ((ISDSC_STATUS)0xEFFF0023L)

//
// MessageId: ISDSC_TRANSPORT_ERROR
//
// MessageText:
//
// TCP Transport Error
//
#define ISDSC_TRANSPORT_ERROR            ((ISDSC_STATUS)0xEFFF0024L)

//
// MessageId: ISDSC_VERSION_MISMATCH
//
// MessageText:
//
// iSCSI Version Mismatch
//
#define ISDSC_VERSION_MISMATCH           ((ISDSC_STATUS)0xEFFF0025L)

//
// MessageId: ISDSC_TARGET_MAPPING_OUT_OF_RANGE
//
// MessageText:
//
// The Target Mapping Address passed is out of range for the adapter configuration.
//
#define ISDSC_TARGET_MAPPING_OUT_OF_RANGE ((ISDSC_STATUS)0xEFFF0026L)

//
// MessageId: ISDSC_TARGET_PRESHAREDKEY_UNAVAILABLE
//
// MessageText:
//
// The preshared key for the target or IKE identification payload is not available.
//
#define ISDSC_TARGET_PRESHAREDKEY_UNAVAILABLE ((ISDSC_STATUS)0xEFFF0027L)

//
// MessageId: ISDSC_TARGET_AUTHINFO_UNAVAILABLE
//
// MessageText:
//
// The authentication information for the target is not available.
//
#define ISDSC_TARGET_AUTHINFO_UNAVAILABLE ((ISDSC_STATUS)0xEFFF0028L)

//
// MessageId: ISDSC_TARGET_NOT_FOUND
//
// MessageText:
//
// The target name is not found or is marked as hidden from login.
//
#define ISDSC_TARGET_NOT_FOUND           ((ISDSC_STATUS)0xEFFF0029L)

//
// MessageId: ISDSC_LOGIN_USER_INFO_BAD
//
// MessageText:
//
// One or more parameters specified in LoginTargetIN structure is invalid.
//
#define ISDSC_LOGIN_USER_INFO_BAD        ((ISDSC_STATUS)0xEFFF002AL)

//
// MessageId: ISDSC_TARGET_MAPPING_EXISTS
//
// MessageText:
//
// Given target mapping already exists.
//
#define ISDSC_TARGET_MAPPING_EXISTS      ((ISDSC_STATUS)0xEFFF002BL)

//
// MessageId: ISDSC_HBA_SECURITY_CACHE_FULL
//
// MessageText:
//
// The HBA security information cache is full.
//
#define ISDSC_HBA_SECURITY_CACHE_FULL    ((ISDSC_STATUS)0xEFFF002CL)

//
// MessageId: ISDSC_INVALID_PORT_NUMBER
//
// MessageText:
//
// The port number passed is not valid for the initiator.
//
#define ISDSC_INVALID_PORT_NUMBER        ((ISDSC_STATUS)0xEFFF002DL)

//
// MessageId: ISDSC_OPERATION_NOT_ALL_SUCCESS
//
// MessageText:
//
// The operation was not successful for all initiators or discovery methods.
//
#define ISDSC_OPERATION_NOT_ALL_SUCCESS  ((ISDSC_STATUS)0xAFFF002EL)

//
// MessageId: ISDSC_HBA_SECURITY_CACHE_NOT_SUPPORTED
//
// MessageText:
//
// The HBA security information cache is not supported by this adapter.
//
#define ISDSC_HBA_SECURITY_CACHE_NOT_SUPPORTED ((ISDSC_STATUS)0xEFFF002FL)

//
// MessageId: ISDSC_IKE_ID_PAYLOAD_TYPE_NOT_SUPPORTED
//
// MessageText:
//
// The IKE id payload type specified is not supported.
//
#define ISDSC_IKE_ID_PAYLOAD_TYPE_NOT_SUPPORTED ((ISDSC_STATUS)0xEFFF0030L)

//
// MessageId: ISDSC_IKE_ID_PAYLOAD_INCORRECT_SIZE
//
// MessageText:
//
// The IKE id payload size specified is not correct.
//
#define ISDSC_IKE_ID_PAYLOAD_INCORRECT_SIZE ((ISDSC_STATUS)0xEFFF0031L)

//
// MessageId: ISDSC_TARGET_PORTAL_ALREADY_EXISTS
//
// MessageText:
//
// Target Portal Structure Already Exists.
//
#define ISDSC_TARGET_PORTAL_ALREADY_EXISTS ((ISDSC_STATUS)0xEFFF0032L)

//
// MessageId: ISDSC_TARGET_ADDRESS_ALREADY_EXISTS
//
// MessageText:
//
// Target Address Structure Already Exists.
//
#define ISDSC_TARGET_ADDRESS_ALREADY_EXISTS ((ISDSC_STATUS)0xEFFF0033L)

//
// MessageId: ISDSC_NO_AUTH_INFO_AVAILABLE
//
// MessageText:
//
// There is no IKE authentication information available.
//
#define ISDSC_NO_AUTH_INFO_AVAILABLE     ((ISDSC_STATUS)0xEFFF0034L)

//
// MessageId: ISDSC_NO_TUNNEL_OUTER_MODE_ADDRESS
//
// MessageText:
//
// There is no tunnel mode outer address specified.
//
#define ISDSC_NO_TUNNEL_OUTER_MODE_ADDRESS ((ISDSC_STATUS)0xEFFF0035L)

//
// MessageId: ISDSC_CACHE_CORRUPTED
//
// MessageText:
//
// Authentication or tunnel address cache is corrupted.
//
#define ISDSC_CACHE_CORRUPTED            ((ISDSC_STATUS)0xEFFF0036L)

//
// MessageId: ISDSC_REQUEST_NOT_SUPPORTED
//
// MessageText:
//
// The request or operation is not supported.
//
#define ISDSC_REQUEST_NOT_SUPPORTED      ((ISDSC_STATUS)0xEFFF0037L)

//
// MessageId: ISDSC_TARGET_OUT_OF_RESORCES
//
// MessageText:
//
// The target does not have enough resources to process the given request.
//
#define ISDSC_TARGET_OUT_OF_RESORCES     ((ISDSC_STATUS)0xEFFF0038L)

//
// MessageId: ISDSC_SERVICE_DID_NOT_RESPOND
//
// MessageText:
//
// The initiator service did not respond to the request sent by the driver.
//
#define ISDSC_SERVICE_DID_NOT_RESPOND    ((ISDSC_STATUS)0xEFFF0039L)

//
// MessageId: ISDSC_ISNS_SERVER_NOT_FOUND
//
// MessageText:
//
// The Internet Storage Name Server (iSNS) server was not found or is unavailable.
//
#define ISDSC_ISNS_SERVER_NOT_FOUND      ((ISDSC_STATUS)0xEFFF003AL)

//
// MessageId: ISDSC_OPERATION_REQUIRES_REBOOT
//
// MessageText:
//
// The operation was successful but requires a driver reload or reboot to become effective.
//
#define ISDSC_OPERATION_REQUIRES_REBOOT  ((ISDSC_STATUS)0xAFFF003BL)

//
// MessageId: ISDSC_NO_PORTAL_SPECIFIED
//
// MessageText:
//
// There is no target portal available to complete the login.
//
#define ISDSC_NO_PORTAL_SPECIFIED        ((ISDSC_STATUS)0xEFFF003CL)

//
// MessageId: ISDSC_CANT_REMOVE_LAST_CONNECTION
//
// MessageText:
//
// Cannot remove the last connection for a session.
//
#define ISDSC_CANT_REMOVE_LAST_CONNECTION ((ISDSC_STATUS)0xEFFF003DL)

//
// MessageId: ISDSC_SERVICE_NOT_RUNNING
//
// MessageText:
//
// The Microsoft iSCSI Initiator Service is not running. Please start the service and retry.
//
#define ISDSC_SERVICE_NOT_RUNNING        ((ISDSC_STATUS)0xEFFF003EL)

//
// MessageId: ISDSC_TARGET_ALREADY_LOGGED_IN
//
// MessageText:
//
// The target has already been logged in via an iSCSI session.
//
#define ISDSC_TARGET_ALREADY_LOGGED_IN   ((ISDSC_STATUS)0xEFFF003FL)

//
// MessageId: ISDSC_DEVICE_BUSY_ON_SESSION
//
// MessageText:
//
// The session cannot be logged out since a device on that session is currently being used.
//
#define ISDSC_DEVICE_BUSY_ON_SESSION     ((ISDSC_STATUS)0xEFFF0040L)

//
// MessageId: ISDSC_COULD_NOT_SAVE_PERSISTENT_LOGIN_DATA
//
// MessageText:
//
// Failed to save persistent login information.
//
#define ISDSC_COULD_NOT_SAVE_PERSISTENT_LOGIN_DATA ((ISDSC_STATUS)0xEFFF0041L)

//
// MessageId: ISDSC_COULD_NOT_REMOVE_PERSISTENT_LOGIN_DATA
//
// MessageText:
//
// Failed to remove persistent login information.
//
#define ISDSC_COULD_NOT_REMOVE_PERSISTENT_LOGIN_DATA ((ISDSC_STATUS)0xEFFF0042L)

//
// MessageId: ISDSC_PORTAL_NOT_FOUND
//
// MessageText:
//
// The specified portal was not found.
//
#define ISDSC_PORTAL_NOT_FOUND           ((ISDSC_STATUS)0xEFFF0043L)

//
// MessageId: ISDSC_INITIATOR_NOT_FOUND
//
// MessageText:
//
// The specified initiator name was not found.
//
#define ISDSC_INITIATOR_NOT_FOUND        ((ISDSC_STATUS)0xEFFF0044L)

//
// MessageId: ISDSC_DISCOVERY_MECHANISM_NOT_FOUND
//
// MessageText:
//
// The specified discovery mechanism was not found.
//
#define ISDSC_DISCOVERY_MECHANISM_NOT_FOUND ((ISDSC_STATUS)0xEFFF0045L)

//
// MessageId: ISDSC_IPSEC_NOT_SUPPORTED_ON_OS
//
// MessageText:
//
// iSCSI does not support IPSEC for this version of the OS.
//
#define ISDSC_IPSEC_NOT_SUPPORTED_ON_OS  ((ISDSC_STATUS)0xEFFF0046L)

//
// MessageId: ISDSC_PERSISTENT_LOGIN_TIMEOUT
//
// MessageText:
//
// The iSCSI service timed out waiting for all persistent logins to complete.
//
#define ISDSC_PERSISTENT_LOGIN_TIMEOUT   ((ISDSC_STATUS)0xEFFF0047L)

//
// MessageId: ISDSC_SHORT_CHAP_SECRET
//
// MessageText:
//
// The specified CHAP secret is less than 96 bits and will not be usable for authenticating over non ipsec connections.
//
#define ISDSC_SHORT_CHAP_SECRET          ((ISDSC_STATUS)0xAFFF0048L)

//
// MessageId: ISDSC_EVALUATION_PEROID_EXPIRED
//
// MessageText:
//
// The evaluation period for the iSCSI initiator service has expired.
//
#define ISDSC_EVALUATION_PEROID_EXPIRED  ((ISDSC_STATUS)0xEFFF0049L)

//
// MessageId: ISDSC_INVALID_CHAP_SECRET
//
// MessageText:
//
// CHAP secret given does not conform to the standard. Please see system event log for more information.
//
#define ISDSC_INVALID_CHAP_SECRET        ((ISDSC_STATUS)0xEFFF004AL)

//
// MessageId: ISDSC_INVALID_TARGET_CHAP_SECRET
//
// MessageText:
//
// Target CHAP secret given is invalid. Maximum size of CHAP secret is 16 bytes. Minimum size is 12 bytes if IPSec is not used.
//
#define ISDSC_INVALID_TARGET_CHAP_SECRET ((ISDSC_STATUS)0xEFFF004BL)

//
// MessageId: ISDSC_INVALID_INITIATOR_CHAP_SECRET
//
// MessageText:
//
// Initiator CHAP secret given is invalid. Maximum size of CHAP secret is 16 bytes. Minimum size is 12 bytes if IPSec is not used.
//
#define ISDSC_INVALID_INITIATOR_CHAP_SECRET ((ISDSC_STATUS)0xEFFF004CL)

//
// MessageId: ISDSC_INVALID_CHAP_USER_NAME
//
// MessageText:
//
// CHAP Username given is invalid.
//
#define ISDSC_INVALID_CHAP_USER_NAME     ((ISDSC_STATUS)0xEFFF004DL)

//
// MessageId: ISDSC_INVALID_LOGON_AUTH_TYPE
//
// MessageText:
//
// Logon Authentication type given is invalid.
//
#define ISDSC_INVALID_LOGON_AUTH_TYPE    ((ISDSC_STATUS)0xEFFF004EL)

//
// MessageId: ISDSC_INVALID_TARGET_MAPPING
//
// MessageText:
//
// Target Mapping information given is invalid.
//
#define ISDSC_INVALID_TARGET_MAPPING     ((ISDSC_STATUS)0xEFFF004FL)

//
// MessageId: ISDSC_INVALID_TARGET_ID
//
// MessageText:
//
// Target Id given in Target Mapping is invalid.
//
#define ISDSC_INVALID_TARGET_ID          ((ISDSC_STATUS)0xEFFF0050L)

//
// MessageId: ISDSC_INVALID_ISCSI_NAME
//
// MessageText:
//
// The iSCSI name specified contains invalid characters or is too long.
//
#define ISDSC_INVALID_ISCSI_NAME         ((ISDSC_STATUS)0xEFFF0051L)

//
// MessageId: ISDSC_INCOMPATIBLE_ISNS_VERSION
//
// MessageText:
//
// The version number returned from the Internet Storage Name Server (iSNS) server is not compatible with this version of the iSNS client.
//
#define ISDSC_INCOMPATIBLE_ISNS_VERSION  ((ISDSC_STATUS)0xEFFF0052L)

//
// MessageId: ISDSC_FAILED_TO_CONFIGURE_IPSEC
//
// MessageText:
//
// Initiator failed to configure IPSec for the given connection. This could be because of low resources.
//
#define ISDSC_FAILED_TO_CONFIGURE_IPSEC  ((ISDSC_STATUS)0xEFFF0053L)

//
// MessageId: ISDSC_BUFFER_TOO_SMALL
//
// MessageText:
//
// The buffer given for processing the request is too small.
//
#define ISDSC_BUFFER_TOO_SMALL           ((ISDSC_STATUS)0xEFFF0054L)

//
// MessageId: ISDSC_INVALID_LOAD_BALANCE_POLICY
//
// MessageText:
//
// The given Load Balance policy is not recognized by iScsi initiator.
//
#define ISDSC_INVALID_LOAD_BALANCE_POLICY ((ISDSC_STATUS)0xEFFF0055L)

//
// MessageId: ISDSC_INVALID_PARAMETER
//
// MessageText:
//
// One or more paramaters specified is not valid.
//
#define ISDSC_INVALID_PARAMETER          ((ISDSC_STATUS)0xEFFF0056L)

//
// MessageId: ISDSC_DUPLICATE_PATH_SPECIFIED
//
// MessageText:
//
// Duplicate PathIds were specified in the call to set Load Balance Policy.
//
#define ISDSC_DUPLICATE_PATH_SPECIFIED   ((ISDSC_STATUS)0xEFFF0057L)

//
// MessageId: ISDSC_PATH_COUNT_MISMATCH
//
// MessageText:
//
// Number of paths specified in Set Load Balance Policy does not match the number of paths to the target.
//
#define ISDSC_PATH_COUNT_MISMATCH        ((ISDSC_STATUS)0xEFFF0058L)

//
// MessageId: ISDSC_INVALID_PATH_ID
//
// MessageText:
//
// Path Id specified in the call to set Load Balance Policy is not valid
//
#define ISDSC_INVALID_PATH_ID            ((ISDSC_STATUS)0xEFFF0059L)

//
// MessageId: ISDSC_MULTIPLE_PRIMARY_PATHS_SPECIFIED
//
// MessageText:
//
// Multiple primary paths specified when only one primary path is expected.
//
#define ISDSC_MULTIPLE_PRIMARY_PATHS_SPECIFIED ((ISDSC_STATUS)0xEFFF005AL)

//
// MessageId: ISDSC_NO_PRIMARY_PATH_SPECIFIED
//
// MessageText:
//
// No primary path specified when at least one is expected.
//
#define ISDSC_NO_PRIMARY_PATH_SPECIFIED  ((ISDSC_STATUS)0xEFFF005BL)

//
// MessageId: ISDSC_DEVICE_ALREADY_PERSISTENTLY_BOUND
//
// MessageText:
//
// Device is already a persistently bound device.
//
#define ISDSC_DEVICE_ALREADY_PERSISTENTLY_BOUND ((ISDSC_STATUS)0xEFFF005CL)

//
// MessageId: ISDSC_DEVICE_NOT_FOUND
//
// MessageText:
//
// Device was not found.
//
#define ISDSC_DEVICE_NOT_FOUND           ((ISDSC_STATUS)0xEFFF005DL)

//
// MessageId: ISDSC_DEVICE_NOT_ISCSI_OR_PERSISTENT
//
// MessageText:
//
// The device specified does not originate from an iSCSI disk or a persistent iSCSI login.
//
#define ISDSC_DEVICE_NOT_ISCSI_OR_PERSISTENT ((ISDSC_STATUS)0xEFFF005EL)

//
// MessageId: ISDSC_DNS_NAME_UNRESOLVED
//
// MessageText:
//
// The DNS name specified was not resolved.
//
#define ISDSC_DNS_NAME_UNRESOLVED        ((ISDSC_STATUS)0xEFFF005FL)

//
// MessageId: ISDSC_NO_CONNECTION_AVAILABLE
//
// MessageText:
//
// There is no connection available in the iSCSI session to process the request.
//
#define ISDSC_NO_CONNECTION_AVAILABLE    ((ISDSC_STATUS)0xEFFF0060L)

//
// MessageId: ISDSC_LB_POLICY_NOT_SUPPORTED
//
// MessageText:
//
// The given Load Balance policy is not supported.
//
#define ISDSC_LB_POLICY_NOT_SUPPORTED    ((ISDSC_STATUS)0xEFFF0061L)

//
// MessageId: ISDSC_REMOVE_CONNECTION_IN_PROGRESS
//
// MessageText:
//
// A remove connection request is already in progress for this session.
//
#define ISDSC_REMOVE_CONNECTION_IN_PROGRESS ((ISDSC_STATUS)0xEFFF0062L)

//
// MessageId: ISDSC_INVALID_CONNECTION_ID
//
// MessageText:
//
// Given connection was not found in the session.
//
#define ISDSC_INVALID_CONNECTION_ID      ((ISDSC_STATUS)0xEFFF0063L)

//
// MessageId: ISDSC_CANNOT_REMOVE_LEADING_CONNECTION
//
// MessageText:
//
// The leading connection in the session cannot be removed.
//
#define ISDSC_CANNOT_REMOVE_LEADING_CONNECTION ((ISDSC_STATUS)0xEFFF0064L)

//
// MessageId: ISDSC_RESTRICTED_BY_GROUP_POLICY
//
// MessageText:
//
// The operation cannot be performed since it does not conform with the group policy assigned to this computer.
//
#define ISDSC_RESTRICTED_BY_GROUP_POLICY ((ISDSC_STATUS)0xEFFF0065L)

//
// MessageId: ISDSC_ISNS_FIREWALL_BLOCKED
//
// MessageText:
//
// The operation cannot be performed since the Internet Storage Name Server (iSNS) firewall exception has not been enabled.
//
#define ISDSC_ISNS_FIREWALL_BLOCKED      ((ISDSC_STATUS)0xEFFF0066L)

//
// MessageId: ISDSC_FAILURE_TO_PERSIST_LB_POLICY
//
// MessageText:
//
// Failed to persist load balancing policy parameters.
//
#define ISDSC_FAILURE_TO_PERSIST_LB_POLICY ((ISDSC_STATUS)0xEFFF0067L)

//
// MessageId: ISDSC_INVALID_HOST
//
// MessageText:
//
// The name could not be resolved to an IP Address.
//
#define ISDSC_INVALID_HOST               ((ISDSC_STATUS)0xEFFF0068L)

#endif /* _ISCSIERR_ */
