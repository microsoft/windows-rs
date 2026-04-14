/***************************************************************************
*                                                                          *
*   bitsmsg.h --  error code definitions for the background file copier    *
*                                                                          *
*   Copyright (c) 2000, Microsoft Corp. All rights reserved.               *
*                                                                          *
***************************************************************************/

#ifndef _BGCPYMSG_
#define _BGCPYMSG_

#if defined (_MSC_VER) && (_MSC_VER >= 1020) && !defined(__midl)
#pragma once
#endif

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


//
// Define the severity codes
//


//
// MessageId: BG_E_NOT_FOUND
//
// MessageText:
//
// The requested job was not found.
//
#define BG_E_NOT_FOUND                   0x80200001L

//
// MessageId: BG_E_INVALID_STATE
//
// MessageText:
//
// The requested action is not allowed in the current job state. The job might have been canceled or completed transferring. It is in a read-only state now.
//
#define BG_E_INVALID_STATE               0x80200002L

//
// MessageId: BG_E_EMPTY
//
// MessageText:
//
// There are no files attached to this job. Attach files to the job, and then try again.
//
#define BG_E_EMPTY                       0x80200003L

//
// MessageId: BG_E_FILE_NOT_AVAILABLE
//
// MessageText:
//
// No file is available because no URL generated an error.
//
#define BG_E_FILE_NOT_AVAILABLE          0x80200004L

//
// MessageId: BG_E_PROTOCOL_NOT_AVAILABLE
//
// MessageText:
//
// No protocol is available because no URL generated an error.
//
#define BG_E_PROTOCOL_NOT_AVAILABLE      0x80200005L

//
// MessageId: BG_S_ERROR_CONTEXT_NONE
//
// MessageText:
//
// No errors have occurred.
//
#define BG_S_ERROR_CONTEXT_NONE          0x00200006L

//
// MessageId: BG_E_ERROR_CONTEXT_UNKNOWN
//
// MessageText:
//
// The error occurred in an unknown location.
//
#define BG_E_ERROR_CONTEXT_UNKNOWN       0x80200007L

//
// MessageId: BG_E_ERROR_CONTEXT_GENERAL_QUEUE_MANAGER
//
// MessageText:
//
// The error occurred in the Background Intelligent Transfer Service (BITS) queue manager.
//
#define BG_E_ERROR_CONTEXT_GENERAL_QUEUE_MANAGER 0x80200008L

//
// MessageId: BG_E_ERROR_CONTEXT_LOCAL_FILE
//
// MessageText:
//
// The error occurred while the local file was being processed. Verify that the file is not in use, and then try again.
//
#define BG_E_ERROR_CONTEXT_LOCAL_FILE    0x80200009L

//
// MessageId: BG_E_ERROR_CONTEXT_REMOTE_FILE
//
// MessageText:
//
// The error occurred while the remote file was being processed.
//
#define BG_E_ERROR_CONTEXT_REMOTE_FILE   0x8020000AL

//
// MessageId: BG_E_ERROR_CONTEXT_GENERAL_TRANSPORT
//
// MessageText:
//
// The error occurred in the transport layer. The client could not connect to the server.
//
#define BG_E_ERROR_CONTEXT_GENERAL_TRANSPORT 0x8020000BL

//
// MessageId: BG_E_ERROR_CONTEXT_QUEUE_MANAGER_NOTIFICATION
//
// MessageText:
//
// The error occurred while the notification callback was being processed. Background Intelligent Transfer Service (BITS) will try again later.
//
#define BG_E_ERROR_CONTEXT_QUEUE_MANAGER_NOTIFICATION 0x8020000CL

//
// MessageId: BG_E_DESTINATION_LOCKED
//
// MessageText:
//
// The destination file system volume is not available. Verify that another program, such as CheckDisk, is not running, which would lock the volume. When the volume is available, Background Intelligent Transfer Service (BITS) will try again.
//
#define BG_E_DESTINATION_LOCKED          0x8020000DL

//
// MessageId: BG_E_VOLUME_CHANGED
//
// MessageText:
//
// The destination volume has changed. If the disk is removable, it might have been replaced with a different disk. Reinsert the original disk and resume the job.
//
#define BG_E_VOLUME_CHANGED              0x8020000EL

//
// MessageId: BG_E_ERROR_INFORMATION_UNAVAILABLE
//
// MessageText:
//
// No errors have occurred.
//
#define BG_E_ERROR_INFORMATION_UNAVAILABLE 0x8020000FL

//
// MessageId: BG_E_NETWORK_DISCONNECTED
//
// MessageText:
//
// There are currently no active network connections. Background Intelligent Transfer Service (BITS) will try again when an adapter is connected.
//
#define BG_E_NETWORK_DISCONNECTED        0x80200010L

//
// MessageId: BG_E_MISSING_FILE_SIZE
//
// MessageText:
//
// The server did not return the file size. The URL might point to dynamic content. The Content-Length header is not available in the server's HTTP reply.
//
#define BG_E_MISSING_FILE_SIZE           0x80200011L

//
// MessageId: BG_E_INSUFFICIENT_HTTP_SUPPORT
//
// MessageText:
//
// The server does not support HTTP 1.1.
//
#define BG_E_INSUFFICIENT_HTTP_SUPPORT   0x80200012L

//
// MessageId: BG_E_INSUFFICIENT_RANGE_SUPPORT
//
// MessageText:
//
// The server does not support the necessary HTTP protocol. Background Intelligent Transfer Service (BITS) requires that the server support the Range protocol header.
//
#define BG_E_INSUFFICIENT_RANGE_SUPPORT  0x80200013L

//
// MessageId: BG_E_REMOTE_NOT_SUPPORTED
//
// MessageText:
//
// Background Intelligent Transfer Service (BITS) cannot be used remotely.
//
#define BG_E_REMOTE_NOT_SUPPORTED        0x80200014L

//
// MessageId: BG_E_NEW_OWNER_DIFF_MAPPING
//
// MessageText:
//
// The drive mapping for the job is different for the current owner than for the previous owner. Use a UNC path instead.
//
#define BG_E_NEW_OWNER_DIFF_MAPPING      0x80200015L

//
// MessageId: BG_E_NEW_OWNER_NO_FILE_ACCESS
//
// MessageText:
//
// The new owner has insufficient access to the local files for the job. The new owner might not have permissions to access the job files. Verify that the new owner has sufficient permissions, and then try again.
//
#define BG_E_NEW_OWNER_NO_FILE_ACCESS    0x80200016L

//
// MessageId: BG_S_PARTIAL_COMPLETE
//
// MessageText:
//
// Some of the transferred files were deleted because they were incomplete.
//
#define BG_S_PARTIAL_COMPLETE            0x00200017L

//
// MessageId: BG_E_PROXY_LIST_TOO_LARGE
//
// MessageText:
//
// The HTTP proxy list cannot be longer than 32,000 characters. Try again with a shorter proxy list.
//
#define BG_E_PROXY_LIST_TOO_LARGE        0x80200018L

//
// MessageId: BG_E_PROXY_BYPASS_LIST_TOO_LARGE
//
// MessageText:
//
// The HTTP proxy bypass list cannot be longer than 32,000 characters. Try again with a shorter bypass proxy list.
//
#define BG_E_PROXY_BYPASS_LIST_TOO_LARGE 0x80200019L

//
// MessageId: BG_S_UNABLE_TO_DELETE_FILES
//
// MessageText:
//
// Some of the temporary files could not be deleted. Check the system event log for the complete list of files that could not be deleted.
//
#define BG_S_UNABLE_TO_DELETE_FILES      0x0020001AL

//
// MessageId: BG_E_INVALID_SERVER_RESPONSE
//
// MessageText:
//
// The server's response was not valid. The server was not following the defined protocol. Resume the job, and then Background Intelligent Transfer Service (BITS) will try again.
//
#define BG_E_INVALID_SERVER_RESPONSE     0x8020001BL

//
// MessageId: BG_E_TOO_MANY_FILES
//
// MessageText:
//
// No more files can be added to this job.
//
#define BG_E_TOO_MANY_FILES              0x8020001CL

//
// MessageId: BG_E_LOCAL_FILE_CHANGED
//
// MessageText:
//
// The local file was changed during the transfer. Recreate the job, and then try to transfer it again.
//
#define BG_E_LOCAL_FILE_CHANGED          0x8020001DL

//
// MessageId: BG_E_ERROR_CONTEXT_REMOTE_APPLICATION
//
// MessageText:
//
// The program on the remote server reported the error.
//
#define BG_E_ERROR_CONTEXT_REMOTE_APPLICATION 0x8020001EL

//
// MessageId: BG_E_SESSION_NOT_FOUND
//
// MessageText:
//
// The specified session could not be found on the server. Background Intelligent Transfer Service (BITS) will try again.
//
#define BG_E_SESSION_NOT_FOUND           0x8020001FL

//
// MessageId: BG_E_TOO_LARGE
//
// MessageText:
//
// The job is too large for the server to accept. This job might exceed a job size limit set by the server administrator. Reduce the size of the job, and then try again.
//
#define BG_E_TOO_LARGE                   0x80200020L

//
// MessageId: BG_E_STRING_TOO_LONG
//
// MessageText:
//
// The specified string is too long.
//
#define BG_E_STRING_TOO_LONG             0x80200021L

//
// MessageId: BG_E_CLIENT_SERVER_PROTOCOL_MISMATCH
//
// MessageText:
//
// The client and server versions of Background Intelligent Transfer Service (BITS) are incompatible.
//
#define BG_E_CLIENT_SERVER_PROTOCOL_MISMATCH 0x80200022L

//
// MessageId: BG_E_SERVER_EXECUTE_ENABLE
//
// MessageText:
//
// Scripting OR execute permissions are enabled on the IIS virtual directory associated with the job. To upload files to the virtual directory, disable the scripting and execute permissions on the virtual directory.
//
#define BG_E_SERVER_EXECUTE_ENABLE       0x80200023L

//
// MessageId: BG_E_NO_PROGRESS
//
// MessageText:
//
// The job is not making headway.  The server may be misconfigured.  Background Intelligent Transfer Service (BITS) will try again later.
//
#define BG_E_NO_PROGRESS                 0x80200024L

//
// MessageId: BG_E_USERNAME_TOO_LARGE
//
// MessageText:
//
// The user name cannot be longer than 300 characters. Try again with a shorter name.
//
#define BG_E_USERNAME_TOO_LARGE          0x80200025L

//
// MessageId: BG_E_PASSWORD_TOO_LARGE
//
// MessageText:
//
// The password cannot be longer than 65536 characters. Try again with a shorter password.
//
#define BG_E_PASSWORD_TOO_LARGE          0x80200026L

//
// MessageId: BG_E_INVALID_AUTH_TARGET
//
// MessageText:
//
// The authentication target specified in the credentials is not defined.
//
#define BG_E_INVALID_AUTH_TARGET         0x80200027L

//
// MessageId: BG_E_INVALID_AUTH_SCHEME
//
// MessageText:
//
// The authentication scheme specified in the credentials is not defined.
//
#define BG_E_INVALID_AUTH_SCHEME         0x80200028L

//
// MessageId: BG_E_FILE_NOT_FOUND
//
// MessageText:
//
// The specified file name does not match any of the files in the job.
//
#define BG_E_FILE_NOT_FOUND              0x80200029L

//
// MessageId: BG_S_PROXY_CHANGED
//
// MessageText:
//
// The proxy server was changed.
//
#define BG_S_PROXY_CHANGED               0x0020002AL

//
// MessageId: BG_E_INVALID_RANGE
//
// MessageText:
//
// The requested byte range extends beyond the end of the web page.  Use byte ranges that are wholly within the page.
//
#define BG_E_INVALID_RANGE               0x8020002BL

//
// MessageId: BG_E_OVERLAPPING_RANGES
//
// MessageText:
//
// The list of byte ranges contains some overlapping ranges, which are not supported.
//
#define BG_E_OVERLAPPING_RANGES          0x8020002CL

//
// MessageId: BG_E_CONNECT_FAILURE
//
// MessageText:
//
// A connection could not be established.
//
#define BG_E_CONNECT_FAILURE             0x8020002DL

//
// MessageId: BG_E_CONNECTION_CLOSED
//
// MessageText:
//
// The connection was closed prematurely.
//
#define BG_E_CONNECTION_CLOSED           0x8020002EL

//
// Codes 2F through 3D are reserved for future use.
//
//
// MessageId: BG_E_BLOCKED_BY_POLICY
//
// MessageText:
//
// Group Policy settings prevent background jobs from running at this time.
//
#define BG_E_BLOCKED_BY_POLICY           0x8020003EL

//
// MessageId: BG_E_INVALID_PROXY_INFO
//
// MessageText:
//
// The supplied proxy server or bypass list is invalid.
//
#define BG_E_INVALID_PROXY_INFO          0x8020003FL

//
// MessageId: BG_E_INVALID_CREDENTIALS
//
// MessageText:
//
// The format of the supplied security credentials is invalid.
//
#define BG_E_INVALID_CREDENTIALS         0x80200040L

//
// MessageId: BG_E_INVALID_HASH_ALGORITHM
//
// MessageText:
//
// The application chose an unsupported hashing algorithm.
//
#define BG_E_INVALID_HASH_ALGORITHM      0x80200041L

//
// MessageId: BG_E_RECORD_DELETED
//
// MessageText:
//
// The chosen cache record has been deleted.  The attempt to update it has been abandoned.
//
#define BG_E_RECORD_DELETED              0x80200042L

//
// MessageId: BG_E_COMMIT_IN_PROGRESS
//
// MessageText:
//
// Another application thread is already updating the cache record.  
//
#define BG_E_COMMIT_IN_PROGRESS          0x80200043L

//
// MessageId: BG_E_DISCOVERY_IN_PROGRESS
//
// MessageText:
//
// The system is already searching for neighbors.  Retry the operation after a few seconds.
//
#define BG_E_DISCOVERY_IN_PROGRESS       0x80200044L

//
// MessageId: BG_E_UPNP_ERROR
//
// MessageText:
//
// A Universal Plug and Play (UPnP) error has occured.  Please check your Internet Gateway Device.
//
#define BG_E_UPNP_ERROR                  0x80200045L

//
// MessageId: BG_E_TEST_OPTION_BLOCKED_DOWNLOAD
//
// MessageText:
//
// The test option is blocking the download after the search completed.
//
#define BG_E_TEST_OPTION_BLOCKED_DOWNLOAD 0x80200046L

//
// MessageId: BG_E_PEERCACHING_DISABLED
//
// MessageText:
//
// Peer-caching is disabled.
//
#define BG_E_PEERCACHING_DISABLED        0x80200047L

//
// MessageId: BG_E_BUSYCACHERECORD
//
// MessageText:
//
// The cache record is in use and can not be changed or deleted at this time.  Try again after a few seconds.
//
#define BG_E_BUSYCACHERECORD             0x80200048L

//
// MessageId: BG_E_TOO_MANY_JOBS_PER_USER
//
// MessageText:
//
// Job count for the current user has exceeded the per user job limit.
//
#define BG_E_TOO_MANY_JOBS_PER_USER      0x80200049L

//
// MessageId: BG_E_TOO_MANY_JOBS_PER_MACHINE
//
// MessageText:
//
// Job count for the current machine has exceeded the per machine job limit.
//
#define BG_E_TOO_MANY_JOBS_PER_MACHINE   0x80200050L

//
// MessageId: BG_E_TOO_MANY_FILES_IN_JOB
//
// MessageText:
//
// File count for the current job has exceeded the per job file limit.
//
#define BG_E_TOO_MANY_FILES_IN_JOB       0x80200051L

//
// MessageId: BG_E_TOO_MANY_RANGES_IN_FILE
//
// MessageText:
//
// Range count for the current file has exceeded the per file Range limit.
//
#define BG_E_TOO_MANY_RANGES_IN_FILE     0x80200052L

//
// MessageId: BG_E_VALIDATION_FAILED
//
// MessageText:
//
// The application requested data from a web site, but the response was invalid. Using Event Viewer, check the log 'Application Logs \ Microsoft \ Windows \ Bits-client \ Operational' for more details.
//
#define BG_E_VALIDATION_FAILED           0x80200053L

//
// MessageId: BG_E_MAXDOWNLOAD_TIMEOUT
//
// MessageText:
//
// BITS timed out downloading the job. The download did not complete within the maximum download time set on the job.
//
#define BG_E_MAXDOWNLOAD_TIMEOUT         0x80200054L

//
// MessageId: BG_S_OVERRIDDEN_BY_POLICY
//
// MessageText:
//
// The configuration preferences have been saved successfully, but one or more of them are overridden by Group Policy.
//
#define BG_S_OVERRIDDEN_BY_POLICY        0x00200055L

//
// MessageId: BG_E_TOKEN_REQUIRED
//
// MessageText:
//
// The job is configured to use a different security token for some operations, but the token is not currently available.  The application must provide a token and then resume the job.
//
#define BG_E_TOKEN_REQUIRED              0x80200056L

//
// MessageId: BG_E_UNKNOWN_PROPERTY_ID
//
// MessageText:
//
// SetProperty() or GetProperty() called with an unknown property ID.
//
#define BG_E_UNKNOWN_PROPERTY_ID         0x80200057L

//
// MessageId: BG_E_READ_ONLY_PROPERTY
//
// MessageText:
//
// Unable to call SetProperty() on a read-only property.
//
#define BG_E_READ_ONLY_PROPERTY          0x80200058L

//
// MessageId: BG_E_BLOCKED_BY_COST_TRANSFER_POLICY
//
// MessageText:
//
// The job's cost transfer policy settings prevent the job from transferring at this time.
//
#define BG_E_BLOCKED_BY_COST_TRANSFER_POLICY 0x80200059L

//
// MessageId: BG_E_PROPERTY_SUPPORTED_FOR_DOWNLOAD_JOBS_ONLY
//
// MessageText:
//
// The property is supported for download jobs only.
//
#define BG_E_PROPERTY_SUPPORTED_FOR_DOWNLOAD_JOBS_ONLY 0x80200060L

//
// MessageId: BG_E_READ_ONLY_PROPERTY_AFTER_ADDFILE
//
// MessageText:
//
// The property cannot be changed after adding a file to job.
//
#define BG_E_READ_ONLY_PROPERTY_AFTER_ADDFILE 0x80200061L

//
// MessageId: BG_E_READ_ONLY_PROPERTY_AFTER_RESUME
//
// MessageText:
//
// The property cannot be changed after Resuming the job.
//
#define BG_E_READ_ONLY_PROPERTY_AFTER_RESUME 0x80200062L

//
// MessageId: BG_E_MAX_DOWNLOAD_SIZE_INVALID_VALUE
//
// MessageText:
//
// The value provided for BITS_JOB_PROPERTY_MAX_DOWNLOAD_SIZE property is invalid. Please provide value between 1 and UINT64_MAX.
//
#define BG_E_MAX_DOWNLOAD_SIZE_INVALID_VALUE 0x80200063L

//
// MessageId: BG_E_MAX_DOWNLOAD_SIZE_LIMIT_REACHED
//
// MessageText:
//
// BITS cannot continue downloading the job. The download reached the maximum download size limit set on the job.
//
#define BG_E_MAX_DOWNLOAD_SIZE_LIMIT_REACHED 0x80200064L

//
// MessageId: BG_E_STANDBY_MODE
//
// MessageText:
//
// The transfer was paused because the computer is in power-saving mode. The transfer will resume when the computer wakes up.
//
#define BG_E_STANDBY_MODE                0x80200065L

//
// MessageId: BG_E_USE_STORED_CREDENTIALS_NOT_SUPPORTED
//
// MessageText:
//
// The value provided for BITS_JOB_PROPERTY_USE_STORED_CREDENTIALS property is invalid. The property is only supported for Proxy (BG_AUTH_TARGET_PROXY) targets.
//
#define BG_E_USE_STORED_CREDENTIALS_NOT_SUPPORTED 0x80200066L

//
// MessageId: BG_E_BLOCKED_BY_BATTERY_POLICY
//
// MessageText:
//
// The transfer was paused because the computer is running on battery power. The transfer will resume when the system is reconnected to A/C power.
//
#define BG_E_BLOCKED_BY_BATTERY_POLICY   0x80200067L

//
// MessageId: BG_E_BLOCKED_BY_BATTERY_SAVER
//
// MessageText:
//
// The transfer was paused because the computer is in Battery Saver mode. The transfer will resume when the system is reconnected to A/C power or no longer in Battery Saver mode.
//
#define BG_E_BLOCKED_BY_BATTERY_SAVER    0x80200068L

//
// MessageId: BG_E_WATCHDOG_TIMEOUT
//
// MessageText:
//
// An operation did not complete within the expected time interval.
//
#define BG_E_WATCHDOG_TIMEOUT            0x80200069L

//
// MessageId: BG_E_APP_PACKAGE_NOT_FOUND
//
// MessageText:
//
// An app package matching the job owner or COM caller was not found.
//
#define BG_E_APP_PACKAGE_NOT_FOUND       0x8020006AL

//
// MessageId: BG_E_APP_PACKAGE_SCENARIO_NOT_SUPPORTED
//
// MessageText:
//
// BITS usage by app packages cannot be supported on this device because one or more dependencies are missing.
//
#define BG_E_APP_PACKAGE_SCENARIO_NOT_SUPPORTED 0x8020006BL

//
// MessageId: BG_E_DATABASE_CORRUPT
//
// MessageText:
//
// BITS persistent state corruption was detected. The necessary steps will be taken to fix this issue once BITS is restarted. BITS will now shutdown.
//
#define BG_E_DATABASE_CORRUPT            0x8020006CL

//
// MessageId: BG_E_RANDOM_ACCESS_NOT_SUPPORTED
//
// MessageText:
//
// BITS IBackgroundCopyFile6 methods are not supported in SMB, dynamic content, multirange, upload, or upload-reply transfers.
//
#define BG_E_RANDOM_ACCESS_NOT_SUPPORTED 0x8020006DL

//
// MessageId: BG_E_BLOCKED_BY_BACKGROUND_ACCESS_POLICY
//
// MessageText:
//
// The background access settings of the job's owner app prevent the job from transferring at this time.
//
#define BG_E_BLOCKED_BY_BACKGROUND_ACCESS_POLICY 0x8020006EL

//
// MessageId: BG_E_BLOCKED_BY_GAME_MODE
//
// MessageText:
//
// The job was paused because there is an active Game Mode recipient that isn't the job's owner. The job will resume once the Game Mode recipient loses input focus or is closed.
//
#define BG_E_BLOCKED_BY_GAME_MODE        0x8020006FL

//
// MessageId: BG_E_BLOCKED_BY_SYSTEM_POLICY
//
// MessageText:
//
// The job was paused due to system resource constraints. The job will automatically resume once system conditions change.
//
#define BG_E_BLOCKED_BY_SYSTEM_POLICY    0x80200070L

//
// MessageId: BG_E_NOT_SUPPORTED_WITH_CUSTOM_HTTP_METHOD
//
// MessageText:
//
// The requested action is not supported by jobs configured with a custom HTTP method.
//
#define BG_E_NOT_SUPPORTED_WITH_CUSTOM_HTTP_METHOD 0x80200071L

//
// MessageId: BG_E_UNSUPPORTED_JOB_CONFIGURATION
//
// MessageText:
//
// The current job configuration is not compatible with the requested action.
//
#define BG_E_UNSUPPORTED_JOB_CONFIGURATION 0x80200072L

//
// MessageId: BG_E_REMOTE_FILE_CHANGED
//
// MessageText:
//
// The remote file was changed during the transfer. If this is expected, call Resume on the job to initiate a retry.
//
#define BG_E_REMOTE_FILE_CHANGED         0x80200073L

//
// MessageId: BG_E_SERVER_CERT_VALIDATION_INTERFACE_REQUIRED
//
// MessageText:
//
// The job is configured to raise a certificate validation callback, but the callback object is not currently available. The job owner must provide a new callback object and then resume the job.
//
#define BG_E_SERVER_CERT_VALIDATION_INTERFACE_REQUIRED 0x80200074L

//
// MessageId: BG_E_READ_ONLY_WHEN_JOB_ACTIVE
//
// MessageText:
//
// This method should only be called when the job is in a paused state (SUSPENDED/ERROR/TRANSFERRED).
//
#define BG_E_READ_ONLY_WHEN_JOB_ACTIVE   0x80200075L

//
// MessageId: BG_E_ERROR_CONTEXT_SERVER_CERTIFICATE_CALLBACK
//
// MessageText:
//
// The error occurred while the user-provided server certificate validation callback was being processed.
//
#define BG_E_ERROR_CONTEXT_SERVER_CERTIFICATE_CALLBACK 0x80200076L

//
// MessageId: BG_E_HTTP_ERROR_100
//
// MessageText:
//
// HTTP status 100: The request can be continued.
//
#define BG_E_HTTP_ERROR_100              0x80190064L

//
// MessageId: BG_E_HTTP_ERROR_101
//
// MessageText:
//
// HTTP status 101: The server switched protocols in an upgrade header.
//
#define BG_E_HTTP_ERROR_101              0x80190065L

//
// MessageId: BG_E_HTTP_ERROR_200
//
// MessageText:
//
// HTTP status 200: The server's response was not valid. The server was not following the defined protocol. Resume the job, and then Background Intelligent Transfer Service (BITS) will try again.
//
#define BG_E_HTTP_ERROR_200              0x801900C8L

//
// MessageId: BG_E_HTTP_ERROR_201
//
// MessageText:
//
// HTTP status 201: The request was fulfilled and resulted in the creation of a new resource.
//
#define BG_E_HTTP_ERROR_201              0x801900C9L

//
// MessageId: BG_E_HTTP_ERROR_202
//
// MessageText:
//
// HTTP status 202: The request was accepted for processing, but the processing has not been completed yet.
//
#define BG_E_HTTP_ERROR_202              0x801900CAL

//
// MessageId: BG_E_HTTP_ERROR_203
//
// MessageText:
//
// HTTP status 203: The returned metadata in the entity-header is not the definitive set available from the server of origin.
//
#define BG_E_HTTP_ERROR_203              0x801900CBL

//
// MessageId: BG_E_HTTP_ERROR_204
//
// MessageText:
//
// HTTP status 204: The server has fulfilled the request, but there is no new information to send back.
//
#define BG_E_HTTP_ERROR_204              0x801900CCL

//
// MessageId: BG_E_HTTP_ERROR_205
//
// MessageText:
//
// HTTP status 205: The server's response was not valid. The server was not following the defined protocol. Resume the job, and then Background Intelligent Transfer Service (BITS) will try again.
//
#define BG_E_HTTP_ERROR_205              0x801900CDL

//
// MessageId: BG_E_HTTP_ERROR_206
//
// MessageText:
//
// HTTP status 206: The server fulfilled the partial GET request for the resource.
//
#define BG_E_HTTP_ERROR_206              0x801900CEL

//
// MessageId: BG_E_HTTP_ERROR_300
//
// MessageText:
//
// HTTP status 300: The server could not return the requested data.
//
#define BG_E_HTTP_ERROR_300              0x8019012CL

//
// MessageId: BG_E_HTTP_ERROR_301
//
// MessageText:
//
// HTTP status 301: The requested resource was assigned to a new permanent Uniform Resource Identifier (URI), and any future references to this resource should use one of the returned URIs.
//
#define BG_E_HTTP_ERROR_301              0x8019012DL

//
// MessageId: BG_E_HTTP_ERROR_302
//
// MessageText:
//
// HTTP status 302: The requested resource was assigned a different Uniform Resource Identifier (URI). This change is temporary.
//
#define BG_E_HTTP_ERROR_302              0x8019012EL

//
// MessageId: BG_E_HTTP_ERROR_303
//
// MessageText:
//
// HTTP status 303: The response to the request is under a different Uniform Resource Identifier (URI) and must be retrieved using a GET method on that resource.
//
#define BG_E_HTTP_ERROR_303              0x8019012FL

//
// MessageId: BG_E_HTTP_ERROR_304
//
// MessageText:
//
// HTTP status 304: The server's response was not valid. The server was not following the defined protocol. Resume the job, and then Background Intelligent Transfer Service (BITS) will try again.
//
#define BG_E_HTTP_ERROR_304              0x80190130L

//
// MessageId: BG_E_HTTP_ERROR_305
//
// MessageText:
//
// HTTP status 305: The requested resource must be accessed through the proxy given by the location field.
//
#define BG_E_HTTP_ERROR_305              0x80190131L

//
// MessageId: BG_E_HTTP_ERROR_307
//
// MessageText:
//
// HTTP status 307: The URL has been temporarily relocated. Try again later.
//
#define BG_E_HTTP_ERROR_307              0x80190133L

//
// MessageId: BG_E_HTTP_ERROR_400
//
// MessageText:
//
// HTTP status 400: The server cannot process the request because the syntax is not valid.
//
#define BG_E_HTTP_ERROR_400              0x80190190L

//
// MessageId: BG_E_HTTP_ERROR_401
//
// MessageText:
//
// HTTP status 401: The requested resource requires user authentication.
//
#define BG_E_HTTP_ERROR_401              0x80190191L

//
// MessageId: BG_E_HTTP_ERROR_402
//
// MessageText:
//
// HTTP status 402: The server's response was not valid. The server was not following the defined protocol. Resume the job, and then Background Intelligent Transfer Service (BITS) will try again.
//
#define BG_E_HTTP_ERROR_402              0x80190192L

//
// MessageId: BG_E_HTTP_ERROR_403
//
// MessageText:
//
// HTTP status 403: The client does not have sufficient access rights to the requested server object.
//
#define BG_E_HTTP_ERROR_403              0x80190193L

//
// MessageId: BG_E_HTTP_ERROR_404
//
// MessageText:
//
// HTTP status 404: The requested URL does not exist on the server.
//
#define BG_E_HTTP_ERROR_404              0x80190194L

//
// MessageId: BG_E_HTTP_ERROR_405
//
// MessageText:
//
// HTTP status 405: The method used is not allowed.
//
#define BG_E_HTTP_ERROR_405              0x80190195L

//
// MessageId: BG_E_HTTP_ERROR_406
//
// MessageText:
//
// HTTP status 406: No responses acceptable to the client were found.
//
#define BG_E_HTTP_ERROR_406              0x80190196L

//
// MessageId: BG_E_HTTP_ERROR_407
//
// MessageText:
//
// HTTP status 407: Proxy authentication is required.
//
#define BG_E_HTTP_ERROR_407              0x80190197L

//
// MessageId: BG_E_HTTP_ERROR_408
//
// MessageText:
//
// HTTP status 408: The server timed out waiting for the request.
//
#define BG_E_HTTP_ERROR_408              0x80190198L

//
// MessageId: BG_E_HTTP_ERROR_409
//
// MessageText:
//
// HTTP status 409: The request could not be completed because of a conflict with the current state of the resource. The user should resubmit the request with more information.
//
#define BG_E_HTTP_ERROR_409              0x80190199L

//
// MessageId: BG_E_HTTP_ERROR_410
//
// MessageText:
//
// HTTP status 410: The requested resource is not currently available at the server, and no forwarding address is known.
//
#define BG_E_HTTP_ERROR_410              0x8019019AL

//
// MessageId: BG_E_HTTP_ERROR_411
//
// MessageText:
//
// HTTP status 411: The server cannot accept the request without a defined content length.
//
#define BG_E_HTTP_ERROR_411              0x8019019BL

//
// MessageId: BG_E_HTTP_ERROR_412
//
// MessageText:
//
// HTTP status 412: The precondition given in one or more of the request header fields evaluated to false when it was tested on the server.
//
#define BG_E_HTTP_ERROR_412              0x8019019CL

//
// MessageId: BG_E_HTTP_ERROR_413
//
// MessageText:
//
// HTTP status 413: The server cannot process the request because the request entity is too large.
//
#define BG_E_HTTP_ERROR_413              0x8019019DL

//
// MessageId: BG_E_HTTP_ERROR_414
//
// MessageText:
//
// HTTP status 414: The server cannot process the request because the request Uniform Resource Identifier (URI) is longer than the server can interpret.
//
#define BG_E_HTTP_ERROR_414              0x8019019EL

//
// MessageId: BG_E_HTTP_ERROR_415
//
// MessageText:
//
// HTTP status 415: The server's response was not valid. The server was not following the defined protocol. Resume the job, and then Background Intelligent Transfer Service (BITS) will try again.
//
#define BG_E_HTTP_ERROR_415              0x8019019FL

//
// MessageId: BG_E_HTTP_ERROR_416
//
// MessageText:
//
// HTTP status 416: The server could not satisfy the range request.
//
#define BG_E_HTTP_ERROR_416              0x801901A0L

//
// MessageId: BG_E_HTTP_ERROR_417
//
// MessageText:
//
// HTTP status 417: The server could not meet the expectation given in an Expect request-header field.
//
#define BG_E_HTTP_ERROR_417              0x801901A1L

//
// MessageId: BG_E_HTTP_ERROR_449
//
// MessageText:
//
// HTTP status 449: The server's response was not valid. The server was not following the defined protocol. Resume the job, and then Background Intelligent Transfer Service (BITS) will try again.
//
#define BG_E_HTTP_ERROR_449              0x801901C1L

//
// MessageId: BG_E_HTTP_ERROR_500
//
// MessageText:
//
// HTTP status 500: An unexpected condition prevented the server from fulfilling the request.
//
#define BG_E_HTTP_ERROR_500              0x801901F4L

//
// MessageId: BG_E_HTTP_ERROR_501
//
// MessageText:
//
// HTTP status 501: The server does not support the functionality required to fulfill the request.
//
#define BG_E_HTTP_ERROR_501              0x801901F5L

//
// MessageId: BG_E_HTTP_ERROR_502
//
// MessageText:
//
// HTTP status 502: The server, while acting as a gateway or proxy to fulfill the request, received an invalid response from the upstream server it accessed.
//
#define BG_E_HTTP_ERROR_502              0x801901F6L

//
// MessageId: BG_E_HTTP_ERROR_503
//
// MessageText:
//
// HTTP status 503: The service is temporarily overloaded.
//
#define BG_E_HTTP_ERROR_503              0x801901F7L

//
// MessageId: BG_E_HTTP_ERROR_504
//
// MessageText:
//
// HTTP status 504: The request was timed out waiting for a gateway.
//
#define BG_E_HTTP_ERROR_504              0x801901F8L

//
// MessageId: BG_E_HTTP_ERROR_505
//
// MessageText:
//
// HTTP status 505: The server does not support the HTTP protocol version that was used in the request message.
//
#define BG_E_HTTP_ERROR_505              0x801901F9L

//
// Additional Background Intelligent Transfer Service (BITS) mc entries
// Reserved range is 0x4000 to 0x4100
//
//
// MessageId: BITS_MC_JOB_CANCELLED
//
// MessageText:
//
// The administrator %4 canceled job "%2" on behalf of %3.  The job ID was %1.
//
#define BITS_MC_JOB_CANCELLED            0x80194000L

//
// MessageId: BITS_MC_FILE_DELETION_FAILED
//
// MessageText:
//
// While canceling job "%2", BITS was not able to remove the temporary files listed below.
// If you can delete them, then you will regain some disk space.  The job ID was %1.
// 
// %3
//
#define BITS_MC_FILE_DELETION_FAILED     0x80194001L

//
// MessageId: BITS_MC_FILE_DELETION_FAILED_MORE
//
// MessageText:
//
// While canceling job "%2", BITS was not able to remove the temporary files listed below.
// If you can delete them, then you will regain some disk space.  The job ID was %1.
// 
// %3
// 
// Due to space limitations, not all files are listed here.  Check for additional files of the form BITxxx.TMP in the same directory.
//
#define BITS_MC_FILE_DELETION_FAILED_MORE 0x80194002L

//
// MessageId: BITS_MC_JOB_PROPERTY_CHANGE
//
// MessageText:
//
// The administrator %3 modified the %4 property of job "%2".  The job ID was %1.
//
#define BITS_MC_JOB_PROPERTY_CHANGE      0x80194003L

//
// MessageId: BITS_MC_JOB_TAKE_OWNERSHIP
//
// MessageText:
//
// The administrator %4 took ownership of job "%2" from %3.  The job ID was %1.
//
#define BITS_MC_JOB_TAKE_OWNERSHIP       0x80194004L

//
// MessageId: BITS_MC_JOB_SCAVENGED
//
// MessageText:
//
// Job "%2" owned by %3 was canceled after being inactive for more than %4 days.  The job ID was %1.
//
#define BITS_MC_JOB_SCAVENGED            0x80194005L

//
// MessageId: BITS_MC_JOB_NOTIFICATION_FAILURE
//
// MessageText:
//
// Job "%2" owned by %3 failed to notify its associated application.  BITS will retry in %4 minutes.  The job ID was %1.
//
#define BITS_MC_JOB_NOTIFICATION_FAILURE 0x80194006L

//
// MessageId: BITS_MC_STATE_FILE_CORRUPT
//
// MessageText:
//
// The BITS job list is not in a recognized format.  It may have been created by a different version of BITS.  The job list has been cleared.
//
#define BITS_MC_STATE_FILE_CORRUPT       0x80194007L

//
// MessageId: BITS_MC_FAILED_TO_START
//
// MessageText:
//
// The BITS service failed to start. Try restarting the service at a later time.
//
#define BITS_MC_FAILED_TO_START          0x80194008L

//
// MessageId: BITS_MC_FATAL_IGD_ERROR
//
// MessageText:
//
// BITS has hit a fatal error communicating with an Internet Gateway Device.  Please check that the device is functioning properly. BITS will not attempt to use this device until the next system reboot.
//
#define BITS_MC_FATAL_IGD_ERROR          0x80194009L

//
// MessageId: BITS_MC_PEERCACHING_PORT
//
// MessageText:
//
// BITS Peer-caching protocol
//
#define BITS_MC_PEERCACHING_PORT         0x8019400AL

//
// MessageId: BITS_MC_WSD_PORT
//
// MessageText:
//
// Web Services-Discovery protocol
//
#define BITS_MC_WSD_PORT                 0x8019400BL

#endif //_BGCPYMSG_
