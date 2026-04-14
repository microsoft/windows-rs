/*++

Copyright (c) Microsoft Corporation

    Constant definitions for common File Server Resource Management service
    error messages.

--*/

#pragma once


//
// HRESULT Success codes
//

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
// MessageId: FSRM_S_PARTIAL_BATCH
//
// MessageText:
//
// At least one failure occurred in a batch operation.
//
#define FSRM_S_PARTIAL_BATCH             ((HRESULT)0x00045304L)

//
// MessageId: FSRM_S_PARTIAL_CLASSIFICATION
//
// MessageText:
//
// The file may only have partial classification because a failure occurred while loading or classifying the file properties.
//
#define FSRM_S_PARTIAL_CLASSIFICATION    ((HRESULT)0x00045305L)

//
// MessageId: FSRM_S_CLASSIFICATION_SCAN_FAILURES
//
// MessageText:
//
// Classification failed on one or more volumes. Check the application event log for more information.
//
#define FSRM_S_CLASSIFICATION_SCAN_FAILURES ((HRESULT)0x00045306L)


//
// HRESULT Failure codes
//

//
// MessageId: FSRM_E_NOT_FOUND
//
// MessageText:
//
// The requested object was not found.
//
#define FSRM_E_NOT_FOUND                 ((HRESULT)0x80045301L)

//
// MessageId: FSRM_E_INVALID_SCHEDULER_ARGUMENT
//
// MessageText:
//
// One or more of the arguments supplied to the task scheduler are not valid.
//
#define FSRM_E_INVALID_SCHEDULER_ARGUMENT ((HRESULT)0x80045302L)

//
// MessageId: FSRM_E_ALREADY_EXISTS
//
// MessageText:
//
// The specified object already exists.
//
#define FSRM_E_ALREADY_EXISTS            ((HRESULT)0x80045303L)

//
// MessageId: FSRM_E_PATH_NOT_FOUND
//
// MessageText:
//
// The specified path was not found.
//
#define FSRM_E_PATH_NOT_FOUND            ((HRESULT)0x80045304L)

//
// MessageId: FSRM_E_INVALID_USER
//
// MessageText:
//
// The specified user is invalid.
//
#define FSRM_E_INVALID_USER              ((HRESULT)0x80045305L)

//
// MessageId: FSRM_E_INVALID_PATH
//
// MessageText:
//
// The specified path is invalid.
//
#define FSRM_E_INVALID_PATH              ((HRESULT)0x80045306L)

//
// MessageId: FSRM_E_INVALID_LIMIT
//
// MessageText:
//
// The specified limit is invalid.
//
#define FSRM_E_INVALID_LIMIT             ((HRESULT)0x80045307L)

//
// MessageId: FSRM_E_INVALID_NAME
//
// MessageText:
//
// The specified name is invalid.
//
#define FSRM_E_INVALID_NAME              ((HRESULT)0x80045308L)

//
// MessageId: FSRM_E_FAIL_BATCH
//
// MessageText:
//
// All items in a batch operation failed.
//
#define FSRM_E_FAIL_BATCH                ((HRESULT)0x80045309L)

//
// MessageId: FSRM_E_INVALID_TEXT
//
// MessageText:
//
// The specified text is invalid.
//
#define FSRM_E_INVALID_TEXT              ((HRESULT)0x8004530AL)

//
// MessageId: FSRM_E_INVALID_IMPORT_VERSION
//
// MessageText:
//
// The version of the configuration file you are trying to import is not supported. You cannot import configuration files with database versions earlier than 2.0.
//
#define FSRM_E_INVALID_IMPORT_VERSION    ((HRESULT)0x8004530BL)

//
// MessageId: FSRM_E_OUT_OF_RANGE
//
// MessageText:
//
// The specified property is out of range.
//
#define FSRM_E_OUT_OF_RANGE              ((HRESULT)0x8004530DL)

//
// MessageId: FSRM_E_REQD_PARAM_MISSING
//
// MessageText:
//
// The specified required property is missing.
//
#define FSRM_E_REQD_PARAM_MISSING        ((HRESULT)0x8004530EL)

//
// MessageId: FSRM_E_INVALID_COMBINATION
//
// MessageText:
//
// The specified property combination is invalid.
//
#define FSRM_E_INVALID_COMBINATION       ((HRESULT)0x8004530FL)

//
// MessageId: FSRM_E_DUPLICATE_NAME
//
// MessageText:
//
// Duplicate names were detected for the same object.
//
#define FSRM_E_DUPLICATE_NAME            ((HRESULT)0x80045310L)

//
// MessageId: FSRM_E_NOT_SUPPORTED
//
// MessageText:
//
// The operation or the specified combination of parameters is not supported.
//
#define FSRM_E_NOT_SUPPORTED             ((HRESULT)0x80045311L)

//
// MessageId: FSRM_E_DRIVER_NOT_READY
//
// MessageText:
//
// A required filter driver is not installed, loaded or ready for service.
//
#define FSRM_E_DRIVER_NOT_READY          ((HRESULT)0x80045313L)

//
// MessageId: FSRM_E_INSUFFICIENT_DISK
//
// MessageText:
//
// There is insufficient disk space to perform the requested operation.
//
#define FSRM_E_INSUFFICIENT_DISK         ((HRESULT)0x80045314L)

//
// MessageId: FSRM_E_VOLUME_UNSUPPORTED
//
// MessageText:
//
// The specified volume is unsupported.
//
#define FSRM_E_VOLUME_UNSUPPORTED        ((HRESULT)0x80045315L)

//
// MessageId: FSRM_E_UNEXPECTED
//
// MessageText:
//
// The File Server Resource Manager service encountered an unexpected error.
// Check the application event log for more information.
//
#define FSRM_E_UNEXPECTED                ((HRESULT)0x80045316L)

//
// MessageId: FSRM_E_INSECURE_PATH
//
// MessageText:
//
// The specified path is insecure.
//
#define FSRM_E_INSECURE_PATH             ((HRESULT)0x80045317L)

//
// MessageId: FSRM_E_INVALID_SMTP_SERVER
//
// MessageText:
//
// The SMTP server is invalid.
//
#define FSRM_E_INVALID_SMTP_SERVER       ((HRESULT)0x80045318L)

//
// MessageId: FSRM_E_AUTO_QUOTA
//
// MessageText:
//
// Auto apply quota configuration for one or more folders failed.  Check the application event log for more information.
//
#define FSRM_E_AUTO_QUOTA                ((HRESULT)0x0004531BL)

//
// MessageId: FSRM_E_EMAIL_NOT_SENT
//
// MessageText:
//
// The File Server Resource Manager service could not send email due to an error.
// Check the application event log for more information.
//
#define FSRM_E_EMAIL_NOT_SENT            ((HRESULT)0x8004531CL)

//
// MessageId: FSRM_E_INVALID_EMAIL_ADDRESS
//
// MessageText:
//
// The specified email address is invalid.
//
#define FSRM_E_INVALID_EMAIL_ADDRESS     ((HRESULT)0x8004531EL)

//
// MessageId: FSRM_E_FILE_SYSTEM_CORRUPT
//
// MessageText:
//
// The file system might be corrupted.  Please run the CHKDSK utility.
//
#define FSRM_E_FILE_SYSTEM_CORRUPT       ((HRESULT)0x8004531FL)

//
// MessageId: FSRM_E_LONG_CMDLINE
//
// MessageText:
//
// The specified command-line executable path is longer than MAX_PATH.
//
#define FSRM_E_LONG_CMDLINE              ((HRESULT)0x80045320L)

//
// MessageId: FSRM_E_INVALID_FILEGROUP_DEFINITION
//
// MessageText:
//
// The specified file group definition is invalid.
//
#define FSRM_E_INVALID_FILEGROUP_DEFINITION ((HRESULT)0x80045321L)

//
// MessageId: FSRM_E_INVALID_DATASCREEN_DEFINITION
//
// MessageText:
//
// The specified file screen is invalid.
//
#define FSRM_E_INVALID_DATASCREEN_DEFINITION ((HRESULT)0x80045324L)

//
// MessageId: FSRM_E_INVALID_REPORT_FORMAT
//
// MessageText:
//
// The specified report format is invalid.
//
#define FSRM_E_INVALID_REPORT_FORMAT     ((HRESULT)0x80045328L)

//
// MessageId: FSRM_E_INVALID_REPORT_DESC
//
// MessageText:
//
// The specified report description is invalid.
//
#define FSRM_E_INVALID_REPORT_DESC       ((HRESULT)0x80045329L)

//
// MessageId: FSRM_E_INVALID_FILENAME
//
// MessageText:
//
// The specified file name is invalid.
//
#define FSRM_E_INVALID_FILENAME          ((HRESULT)0x8004532AL)

//
// MessageId: FSRM_E_SHADOW_COPY
//
// MessageText:
//
// A volume shadow copy could not be created or was unexpectedly deleted.
//
#define FSRM_E_SHADOW_COPY               ((HRESULT)0x8004532CL)

//
// MessageId: FSRM_E_XML_CORRUPTED
//
// MessageText:
//
// A File Server Resource Manager XML configuration file or import-export file is corrupted.
//
#define FSRM_E_XML_CORRUPTED             ((HRESULT)0x8004532DL)

//
// MessageId: FSRM_E_CLUSTER_NOT_RUNNING
//
// MessageText:
//
// File Server Resource Manager global configuration cannot be accessed since the cluster service is not running.
//
#define FSRM_E_CLUSTER_NOT_RUNNING       ((HRESULT)0x8004532EL)

//
// MessageId: FSRM_E_STORE_NOT_INSTALLED
//
// MessageText:
//
// File Server Resource Manager global configuration cannot be accessed since it is not installed yet.
//
#define FSRM_E_STORE_NOT_INSTALLED       ((HRESULT)0x8004532FL)

//
// MessageId: FSRM_E_NOT_CLUSTER_VOLUME
//
// MessageText:
//
// The volume does not reside on a cluster shared disk with an associated cluster resource.
//
#define FSRM_E_NOT_CLUSTER_VOLUME        ((HRESULT)0x80045330L)

//
// MessageId: FSRM_E_DIFFERENT_CLUSTER_GROUP
//
// MessageText:
//
// There are at least two paths which reside on different cluster shared disks which are not in the same cluster resource group.
//
#define FSRM_E_DIFFERENT_CLUSTER_GROUP   ((HRESULT)0x80045331L)

//
// MessageId: FSRM_E_REPORT_TYPE_ALREADY_EXISTS
//
// MessageText:
//
// A report of the specified type already exists in the report job.
//
#define FSRM_E_REPORT_TYPE_ALREADY_EXISTS ((HRESULT)0x80045332L)

//
// MessageId: FSRM_E_REPORT_JOB_ALREADY_RUNNING
//
// MessageText:
//
// The report job is already running or queued for running.
//
#define FSRM_E_REPORT_JOB_ALREADY_RUNNING ((HRESULT)0x80045333L)

//
// MessageId: FSRM_E_REPORT_GENERATION_ERR
//
// MessageText:
//
// An error occurred during report generation.
//
#define FSRM_E_REPORT_GENERATION_ERR     ((HRESULT)0x80045334L)

//
// MessageId: FSRM_E_REPORT_TASK_TRIGGER
//
// MessageText:
//
// The task contains zero or unsupported triggers.
//
#define FSRM_E_REPORT_TASK_TRIGGER       ((HRESULT)0x80045335L)

//
// MessageId: FSRM_E_LOADING_DISABLED_MODULE
//
// MessageText:
//
// A rule or policy attempted to load/use a disabled module.
//
#define FSRM_E_LOADING_DISABLED_MODULE   ((HRESULT)0x80045336L)

//
// MessageId: FSRM_E_CANNOT_AGGREGATE
//
// MessageText:
//
// File Server Resource Manager cannot aggregate the value for the specified file property.
//
#define FSRM_E_CANNOT_AGGREGATE          ((HRESULT)0x80045337L)

//
// MessageId: FSRM_E_MESSAGE_LIMIT_EXCEEDED
//
// MessageText:
//
// The limit of the number of messages that the current pipeline context can add to the property bag has been reached.
//
#define FSRM_E_MESSAGE_LIMIT_EXCEEDED    ((HRESULT)0x80045338L)

//
// MessageId: FSRM_E_OBJECT_IN_USE
//
// MessageText:
//
// The object is in use and cannot be deleted.
//
#define FSRM_E_OBJECT_IN_USE             ((HRESULT)0x80045339L)

//
// MessageId: FSRM_E_CANNOT_RENAME_PROPERTY
//
// MessageText:
//
// Cannot change the name of a property definition once it is set.
//
#define FSRM_E_CANNOT_RENAME_PROPERTY    ((HRESULT)0x8004533AL)

//
// MessageId: FSRM_E_CANNOT_CHANGE_PROPERTY_TYPE
//
// MessageText:
//
// Cannot change the type of a property definition once it is set.
//
#define FSRM_E_CANNOT_CHANGE_PROPERTY_TYPE ((HRESULT)0x8004533BL)

//
// MessageId: FSRM_E_MAX_PROPERTY_DEFINITIONS
//
// MessageText:
//
// A new property definition cannot be created.  The maximum number of property definitions, {0}, has been reached.
//
#define FSRM_E_MAX_PROPERTY_DEFINITIONS  ((HRESULT)0x8004533CL)

//
// MessageId: FSRM_E_CLASSIFICATION_ALREADY_RUNNING
//
// MessageText:
//
// A classification job is currently running.  Only one classification job can be running at a time.
//
#define FSRM_E_CLASSIFICATION_ALREADY_RUNNING ((HRESULT)0x8004533DL)

//
// MessageId: FSRM_E_CLASSIFICATION_NOT_RUNNING
//
// MessageText:
//
// Classification is not currently running.
//
#define FSRM_E_CLASSIFICATION_NOT_RUNNING ((HRESULT)0x8004533EL)

//
// MessageId: FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_RUNNING
//
// MessageText:
//
// The file management task is already running or queued for running.
//
#define FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_RUNNING ((HRESULT)0x8004533FL)

//
// MessageId: FSRM_E_FILE_MANAGEMENT_JOB_EXPIRATION
//
// MessageText:
//
// Cannot expire a file while running a file management task.
//
#define FSRM_E_FILE_MANAGEMENT_JOB_EXPIRATION ((HRESULT)0x80045340L)

//
// MessageId: FSRM_E_FILE_MANAGEMENT_JOB_CUSTOM
//
// MessageText:
//
// Cannot perform a custom action on a file while running a file management task.
//
#define FSRM_E_FILE_MANAGEMENT_JOB_CUSTOM ((HRESULT)0x80045341L)

//
// MessageId: FSRM_E_FILE_MANAGEMENT_JOB_NOTIFICATION
//
// MessageText:
//
// Cannot send a notification for a file management task.
//
#define FSRM_E_FILE_MANAGEMENT_JOB_NOTIFICATION ((HRESULT)0x80045342L)

//
// MessageId: FSRM_E_FILE_OPEN_ERROR
//
// MessageText:
//
// File Server Resource Manager cannot open the file.
//
#define FSRM_E_FILE_OPEN_ERROR           ((HRESULT)0x80045343L)

//
// MessageId: FSRM_E_UNSECURE_LINK_TO_HOSTED_MODULE
//
// MessageText:
//
// File Server Resource Manager failed to perform a secure link with a hosted module process.
//
#define FSRM_E_UNSECURE_LINK_TO_HOSTED_MODULE ((HRESULT)0x80045344L)

//
// MessageId: FSRM_E_CACHE_INVALID
//
// MessageText:
//
// The property cache for the file is invalid and could not be read.
//
#define FSRM_E_CACHE_INVALID             ((HRESULT)0x80045345L)

//
// MessageId: FSRM_E_CACHE_MODULE_ALREADY_EXISTS
//
// MessageText:
//
// A cache storage module already exists.
//
#define FSRM_E_CACHE_MODULE_ALREADY_EXISTS ((HRESULT)0x80045346L)

//
// MessageId: FSRM_E_FILE_MANAGEMENT_EXPIRATION_DIR_IN_SCOPE
//
// MessageText:
//
// The expiration directory cannot be within the file management scope.
//
#define FSRM_E_FILE_MANAGEMENT_EXPIRATION_DIR_IN_SCOPE ((HRESULT)0x80045347L)

//
// MessageId: FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_EXISTS
//
// MessageText:
//
// A file management task of the specified name already exists.
//
#define FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_EXISTS ((HRESULT)0x80045348L)

//
// MessageId: FSRM_E_PROPERTY_DELETED
//
// MessageText:
//
// The specified file property has been deleted.
//
#define FSRM_E_PROPERTY_DELETED          ((HRESULT)0x80045349L)

//
// MessageId: FSRM_E_LAST_ACCESS_UPDATE_DISABLED
//
// MessageText:
//
// The updating of last access times is disabled on this server.  To create a report or file management task that uses the last access time the updating of last access time must be enabled.
//
#define FSRM_E_LAST_ACCESS_UPDATE_DISABLED ((HRESULT)0x80045350L)

//
// MessageId: FSRM_E_NO_PROPERTY_VALUE
//
// MessageText:
//
// The specified file property should not be assigned a value.
//
#define FSRM_E_NO_PROPERTY_VALUE         ((HRESULT)0x80045351L)

//
// MessageId: FSRM_E_INPROC_MODULE_BLOCKED
//
// MessageText:
//
// An unknown module cannot be run inside the service process.
//
#define FSRM_E_INPROC_MODULE_BLOCKED     ((HRESULT)0x80045352L)

//
// MessageId: FSRM_E_ENUM_PROPERTIES_FAILED
//
// MessageText:
//
// File Server Resource Manager failed to enumerate file properties because a failure occurred while loading or classifying the file properties.
//
#define FSRM_E_ENUM_PROPERTIES_FAILED    ((HRESULT)0x80045353L)

//
// MessageId: FSRM_E_SET_PROPERTY_FAILED
//
// MessageText:
//
// File Server Resource Manager failed to set a file property to the file because a failure occurred while saving the file properties.
//
#define FSRM_E_SET_PROPERTY_FAILED       ((HRESULT)0x80045354L)

//
// MessageId: FSRM_E_CANNOT_STORE_PROPERTIES
//
// MessageText:
//
// Classification properties will not be stored because a failure occurred while loading or classifying the file properties.
//
#define FSRM_E_CANNOT_STORE_PROPERTIES   ((HRESULT)0x80045355L)

//
// MessageId: FSRM_E_CANNOT_ALLOW_REPARSE_POINT_TAG
//
// MessageText:
//
// Classification is not supported on the specified reparse point. File Server Resource Manager does not recognize the reparse point's identifier tag for the purposes of classification.
//
#define FSRM_E_CANNOT_ALLOW_REPARSE_POINT_TAG ((HRESULT)0x80045356L)

//
// MessageId: FSRM_E_PARTIAL_CLASSIFICATION_PROPERTY_NOT_FOUND
//
// MessageText:
//
// The requested property was not found. The file may only have partial classification because a failure occurred while loading or classifying the file properties.
//
#define FSRM_E_PARTIAL_CLASSIFICATION_PROPERTY_NOT_FOUND ((HRESULT)0x80045357L)

//
// MessageId: FSRM_E_TEXTREADER_NOT_INITIALIZED
//
// MessageText:
//
// The File Server Resource Manager text reader was not initialized.
//
#define FSRM_E_TEXTREADER_NOT_INITIALIZED ((HRESULT)0x80045358L)

//
// MessageId: FSRM_E_TEXTREADER_IFILTER_NOT_FOUND
//
// MessageText:
//
// There is no IFilter registered for this extension.
//
#define FSRM_E_TEXTREADER_IFILTER_NOT_FOUND ((HRESULT)0x80045359L)

//
// MessageId: FSRM_E_PERSIST_PROPERTIES_FAILED_ENCRYPTED
//
// MessageText:
//
// File Server Resource Manager failed to write the properties to the file because the file is either corrupt or protected by Rights Management Services.
//
#define FSRM_E_PERSIST_PROPERTIES_FAILED_ENCRYPTED ((HRESULT)0x8004535AL)

//
// MessageId: FSRM_E_TEXTREADER_IFILTER_CLSID_MALFORMED
//
// MessageText:
//
// The IFilter for this extension is not registered correctly.
//
#define FSRM_E_TEXTREADER_IFILTER_CLSID_MALFORMED ((HRESULT)0x80045360L)

//
// MessageId: FSRM_E_TEXTREADER_STREAM_ERROR
//
// MessageText:
//
// There was an error obtaining the file's streaming interface.
//
#define FSRM_E_TEXTREADER_STREAM_ERROR   ((HRESULT)0x80045361L)

//
// MessageId: FSRM_E_TEXTREADER_FILENAME_TOO_LONG
//
// MessageText:
//
// The file name's extension is too long.
//
#define FSRM_E_TEXTREADER_FILENAME_TOO_LONG ((HRESULT)0x80045362L)

//
// MessageId: FSRM_E_INCOMPATIBLE_FORMAT
//
// MessageText:
//
// The module will not process the specified file because it is unable to determine a compatible file format.
//
#define FSRM_E_INCOMPATIBLE_FORMAT       ((HRESULT)0x80045363L)

//
// MessageId: FSRM_E_FILE_ENCRYPTED
//
// MessageText:
//
// File Server Resource Manager could not access the file because it is encrypted.
//
#define FSRM_E_FILE_ENCRYPTED            ((HRESULT)0x80045364L)

//
// MessageId: FSRM_E_PERSIST_PROPERTIES_FAILED
//
// MessageText:
//
// File Server Resource Manager failed to persist the properties to the file.
//
#define FSRM_E_PERSIST_PROPERTIES_FAILED ((HRESULT)0x80045365L)

//
// MessageId: FSRM_E_VOLUME_OFFLINE
//
// MessageText:
//
// File Server Resource Manager failed to access the volume. It may be offline.
//
#define FSRM_E_VOLUME_OFFLINE            ((HRESULT)0x80045366L)

//
// MessageId: FSRM_E_FILE_MANAGEMENT_ACTION_TIMEOUT
//
// MessageText:
//
// The file management action command timed out.
//
#define FSRM_E_FILE_MANAGEMENT_ACTION_TIMEOUT ((HRESULT)0x80045367L)

//
// MessageId: FSRM_E_FILE_MANAGEMENT_ACTION_GET_EXITCODE_FAILED
//
// MessageText:
//
// The file management action completed successfully, but the exit code cannot be obtained.
//
#define FSRM_E_FILE_MANAGEMENT_ACTION_GET_EXITCODE_FAILED ((HRESULT)0x80045368L)

//
// MessageId: FSRM_E_MODULE_INVALID_PARAM
//
// MessageText:
//
// The module encountered an invalid parameter or a valid parameter with an invalid value or an expected module parameter is not found. Check the application event log for more information.
//
#define FSRM_E_MODULE_INVALID_PARAM      ((HRESULT)0x80045369L)

//
// MessageId: FSRM_E_MODULE_INITIALIZATION
//
// MessageText:
//
// The module initialization failed. Check the application event log for more information.
//
#define FSRM_E_MODULE_INITIALIZATION     ((HRESULT)0x8004536AL)

//
// MessageId: FSRM_E_MODULE_SESSION_INITIALIZATION
//
// MessageText:
//
// The module session initialization failed. Check the application event log for more information.
//
#define FSRM_E_MODULE_SESSION_INITIALIZATION ((HRESULT)0x8004536BL)

//
// MessageId: FSRM_E_CLASSIFICATION_SCAN_FAIL
//
// MessageText:
//
// Classification failed on all volumes. Check the application event log for more information.
//
#define FSRM_E_CLASSIFICATION_SCAN_FAIL  ((HRESULT)0x8004536CL)

//
// MessageId: FSRM_E_FILE_MANAGEMENT_JOB_NOT_LEGACY_ACCESSIBLE
//
// MessageText:
//
// The file management task cannot be accessed because task conditions were modified by using WMI or Windows PowerShell interfaces. To access or edit the file management task, use the WMI or Windows PowerShell interfaces.
//
#define FSRM_E_FILE_MANAGEMENT_JOB_NOT_LEGACY_ACCESSIBLE ((HRESULT)0x8004536DL)

//
// MessageId: FSRM_E_FILE_MANAGEMENT_JOB_MAX_FILE_CONDITIONS
//
// MessageText:
//
// The file management task has reached its maximum number of conditions.
//
#define FSRM_E_FILE_MANAGEMENT_JOB_MAX_FILE_CONDITIONS ((HRESULT)0x8004536EL)

//
// MessageId: FSRM_E_CANNOT_USE_DEPRECATED_PROPERTY
//
// MessageText:
//
// This object uses a property definition that is deprecated.  You must change it to use a non-deprecated property definition.
//
#define FSRM_E_CANNOT_USE_DEPRECATED_PROPERTY ((HRESULT)0x8004536FL)

//
// MessageId: FSRM_E_SYNC_TASK_TIMEOUT
//
// MessageText:
//
// The property definition sync task timed out.
//
#define FSRM_E_SYNC_TASK_TIMEOUT         ((HRESULT)0x80045370L)

//
// MessageId: FSRM_E_CANNOT_USE_DELETED_PROPERTY
//
// MessageText:
//
// This object uses a property definition that doesn't exist.  You must change it to use an existing property definition.
//
#define FSRM_E_CANNOT_USE_DELETED_PROPERTY ((HRESULT)0x80045371L)

//
// MessageId: FSRM_E_INVALID_AD_CLAIM
//
// MessageText:
//
// File Server Resource Manager encountered an invalid resource claim in Active Directory.
//
#define FSRM_E_INVALID_AD_CLAIM          ((HRESULT)0x80045372L)

//
// MessageId: FSRM_E_CLASSIFICATION_CANCELED
//
// MessageText:
//
// The classification operation was canceled.
//
#define FSRM_E_CLASSIFICATION_CANCELED   ((HRESULT)0x80045373L)

//
// MessageId: FSRM_E_INVALID_FOLDER_PROPERTY_STORE
//
// MessageText:
//
// File Server Resource Manager encountered an invalid folder property store.
//
#define FSRM_E_INVALID_FOLDER_PROPERTY_STORE ((HRESULT)0x80045374L)

//
// MessageId: FSRM_E_REBUILDING_FODLER_TYPE_INDEX
//
// MessageText:
//
// File Server Resource Manager is rebuilding the index of Folder Usage property values.
//
#define FSRM_E_REBUILDING_FODLER_TYPE_INDEX ((HRESULT)0x80045375L)

//
// MessageId: FSRM_E_PROPERTY_MUST_APPLY_TO_FILES
//
// MessageText:
//
// The specified property definition doesn't apply to files.
//
#define FSRM_E_PROPERTY_MUST_APPLY_TO_FILES ((HRESULT)0x80045376L)

//
// MessageId: FSRM_E_CLASSIFICATION_TIMEOUT
//
// MessageText:
//
// The classification request timed out.
//
#define FSRM_E_CLASSIFICATION_TIMEOUT    ((HRESULT)0x80045377L)

//
// MessageId: FSRM_E_CLASSIFICATION_PARTIAL_BATCH
//
// MessageText:
//
// Classification failed on one or more files in the batch operation.
//
#define FSRM_E_CLASSIFICATION_PARTIAL_BATCH ((HRESULT)0x80045378L)

//
// MessageId: FSRM_E_CANNOT_DELETE_SYSTEM_PROPERTY
//
// MessageText:
//
// This property is a system property and cannot be deleted.
//
#define FSRM_E_CANNOT_DELETE_SYSTEM_PROPERTY ((HRESULT)0x80045379L)

//
// MessageId: FSRM_E_FILE_IN_USE
//
// MessageText:
//
// The file is being used by another application and cannot be accessed at this time.
//
#define FSRM_E_FILE_IN_USE               ((HRESULT)0x8004537AL)

//
// MessageId: FSRM_E_ERROR_NOT_ENABLED
//
// MessageText:
//
// Access-denied assistance is not enabled for this error.
//
#define FSRM_E_ERROR_NOT_ENABLED         ((HRESULT)0x8004537BL)

//
// MessageId: FSRM_E_CANNOT_CREATE_TEMP_COPY
//
// MessageText:
//
// File Server Resource Manager could not create a temporary file copy.
//
#define FSRM_E_CANNOT_CREATE_TEMP_COPY   ((HRESULT)0x8004537CL)

//
// MessageId: FSRM_E_NO_EMAIL_ADDRESS
//
// MessageText:
//
// Access-denied assistance cannot send an email because an email address could not be found for the path specified, and sending email to the administrator is not enabled. 
//
#define FSRM_E_NO_EMAIL_ADDRESS          ((HRESULT)0x8004537DL)

//
// MessageId: FSRM_E_ADR_MAX_EMAILS_SENT
//
// MessageText:
//
// The current user has sent the maximum number of requests for access-denied assistance. 
//
#define FSRM_E_ADR_MAX_EMAILS_SENT       ((HRESULT)0x8004537EL)

//
// MessageId: FSRM_E_PATH_NOT_IN_NAMESPACE
//
// MessageText:
//
// The path is not included in a classification rule.
//
#define FSRM_E_PATH_NOT_IN_NAMESPACE     ((HRESULT)0x8004537FL)

//
// MessageId: FSRM_E_RMS_TEMPLATE_NOT_FOUND
//
// MessageText:
//
// The RMS template used to configure the file management task no longer exists.  Please select another template.
//
#define FSRM_E_RMS_TEMPLATE_NOT_FOUND    ((HRESULT)0x80045380L)

//
// MessageId: FSRM_E_SECURE_PROPERTIES_NOT_SUPPORTED
//
// MessageText:
//
// The computer hosting the file or folder does not support setting secure properties. This can occur if the computer is running Windows Server 2008 R2, Windows 7, or earlier, or if the computer is not running Windows.
//
#define FSRM_E_SECURE_PROPERTIES_NOT_SUPPORTED ((HRESULT)0x80045381L)

//
// MessageId: FSRM_E_RMS_NO_PROTECTORS_INSTALLED
//
// MessageText:
//
// File Server Resource Manager cannot run the file management task because no RMS protectors are installed.
//
#define FSRM_E_RMS_NO_PROTECTORS_INSTALLED ((HRESULT)0x80045382L)

//
// MessageId: FSRM_E_RMS_NO_PROTECTOR_INSTALLED_FOR_FILE
//
// MessageText:
//
// File Server Resource Manager cannot protect the file because an RMS protector for the file type is not installed.
//
#define FSRM_E_RMS_NO_PROTECTOR_INSTALLED_FOR_FILE ((HRESULT)0x80045383L)

//
// MessageId: FSRM_E_PROPERTY_MUST_APPLY_TO_FOLDERS
//
// MessageText:
//
// The specified property definition doesn't apply to folders.
//
#define FSRM_E_PROPERTY_MUST_APPLY_TO_FOLDERS ((HRESULT)0x80045384L)

//
// MessageId: FSRM_E_PROPERTY_MUST_BE_SECURE
//
// MessageText:
//
// The specified property definition type is not secure.
//
#define FSRM_E_PROPERTY_MUST_BE_SECURE   ((HRESULT)0x80045385L)

//
// MessageId: FSRM_E_PROPERTY_MUST_BE_GLOBAL
//
// MessageText:
//
// The specified property definition type is not global.
//
#define FSRM_E_PROPERTY_MUST_BE_GLOBAL   ((HRESULT)0x80045386L)

//
// MessageId: FSRM_E_WMI_FAILURE
//
// MessageText:
//
// Unexpected failure from a WMI call.
//
#define FSRM_E_WMI_FAILURE               ((HRESULT)0x80045387L)

//
// MessageId: FSRM_E_FILE_MANAGEMENT_JOB_RMS
//
// MessageText:
//
// Cannot protect a file while running a file management task.
//
#define FSRM_E_FILE_MANAGEMENT_JOB_RMS   ((HRESULT)0x80045388L)

//
// MessageId: FSRM_E_SYNC_TASK_HAD_ERRORS
//
// MessageText:
//
// The property definition sync task encountered errors.
// Check the application event log for more information.
//
#define FSRM_E_SYNC_TASK_HAD_ERRORS      ((HRESULT)0x80045389L)

//
// MessageId: FSRM_E_ADR_SRV_NOT_SUPPORTED
//
// MessageText:
//
// The server does not provide access-denied assistance.
//
#define FSRM_E_ADR_SRV_NOT_SUPPORTED     ((HRESULT)0x80045390L)

//
// MessageId: FSRM_E_ADR_PATH_IS_LOCAL
//
// MessageText:
//
// Access-denied assistance cannot be provided for local paths.
//
#define FSRM_E_ADR_PATH_IS_LOCAL         ((HRESULT)0x80045391L)

//
// MessageId: FSRM_E_ADR_NOT_DOMAIN_JOINED
//
// MessageText:
//
// Access-denied assistance requires that the server be joined to a domain.
//
#define FSRM_E_ADR_NOT_DOMAIN_JOINED     ((HRESULT)0x80045392L)

//
// MessageId: FSRM_E_CANNOT_REMOVE_READONLY
//
// MessageText:
//
// File Server Resource Manager could not remove the read-only attribute from a file.
//
#define FSRM_E_CANNOT_REMOVE_READONLY    ((HRESULT)0x80045393L)

//
// MessageId: FSRM_E_FILE_MANAGEMENT_JOB_INVALID_CONTINUOUS_CONFIG
//
// MessageText:
//
// A continuous file management job cannot have conditions based on the file's last accessed/modified or created times and cannot define any notifications.
//
#define FSRM_E_FILE_MANAGEMENT_JOB_INVALID_CONTINUOUS_CONFIG ((HRESULT)0x80045394L)

//
// MessageId: FSRM_E_LEGACY_SCHEDULE
//
// MessageText:
//
// The object contains a schedule that was created by using an earlier version of File Server Resource Manager and that is incompatible with the current version of Windows Server. Edit the schedule on this computer to update it.
//
#define FSRM_E_LEGACY_SCHEDULE           ((HRESULT)0x80045395L)

//
// MessageId: FSRM_E_CSC_PATH_NOT_SUPPORTED
//
// MessageText:
//
// This operation is not supported for paths on which Offline Files is enabled.
//
#define FSRM_E_CSC_PATH_NOT_SUPPORTED    ((HRESULT)0x80045396L)

//
// MessageId: FSRM_E_EXPIRATION_PATH_NOT_WRITEABLE
//
// MessageText:
//
// Cannot write to the specified expiration directory. Confirm that the permissions of the expiration directory grant Write permission to the computer account of the server performing the file expiration task.
//
#define FSRM_E_EXPIRATION_PATH_NOT_WRITEABLE ((HRESULT)0x80045397L)

//
// MessageId: FSRM_E_EXPIRATION_PATH_TOO_LONG
//
// MessageText:
//
// The expiration path must be 150 characters or shorter.
//
#define FSRM_E_EXPIRATION_PATH_TOO_LONG  ((HRESULT)0x80045398L)

//
// MessageId: FSRM_E_EXPIRATION_VOLUME_NOT_NTFS
//
// MessageText:
//
// The expiration directory must be on a volume formatted with the NTFS file system.
//
#define FSRM_E_EXPIRATION_VOLUME_NOT_NTFS ((HRESULT)0x80045399L)

//
// MessageId: FSRM_E_FILE_MANAGEMENT_JOB_DEPRECATED
//
// MessageText:
//
// This file management job is deprecated. Please check the configuration of the file management job and verify that it is up-to-date.
//
#define FSRM_E_FILE_MANAGEMENT_JOB_DEPRECATED ((HRESULT)0x8004539AL)

//
// MessageId: FSRM_E_MODULE_TIMEOUT
//
// MessageText:
//
// A module was restarted due to excessive processing time of a file. Check the application event log for more information.
//
#define FSRM_E_MODULE_TIMEOUT            ((HRESULT)0x8004539BL)

