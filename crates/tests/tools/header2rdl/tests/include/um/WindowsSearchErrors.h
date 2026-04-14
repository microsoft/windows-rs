//
// Event Viewer categories
//
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
// MessageId: CATEGORY_SEARCH
//
// MessageText:
//
// Search service
//
#define CATEGORY_SEARCH                  0x00000001L

//
// MessageId: CATEGORY_COLLATOR
//
// MessageText:
//
// Collator
//
#define CATEGORY_COLLATOR                0x00000002L

//
// MessageId: CATEGORY_GATHERER
//
// MessageText:
//
// Gatherer
//
#define CATEGORY_GATHERER                0x00000003L

//
// MessageId: CATEGORY_INDEXER
//
// MessageText:
//
// Content index server
//
#define CATEGORY_INDEXER                 0x00000004L

//
// Event Viewer messages
//
//
// MessageId: EVENT_SSSEARCH_STARTED
//
// MessageText:
//
// The Windows Search Service started.%1
//
#define EVENT_SSSEARCH_STARTED           0x400003EBL

//
// MessageId: EVENT_SSSEARCH_STARTING_SETUP
//
// MessageText:
//
// The Windows Search service is creating the new search index {Reason: %2}. %1
//
#define EVENT_SSSEARCH_STARTING_SETUP    0x400003ECL

//
// MessageId: EVENT_SSSEARCH_SETUP_SUCCEEDED
//
// MessageText:
//
// The Windows Search Service has successfully created the new search index. %1
//
#define EVENT_SSSEARCH_SETUP_SUCCEEDED   0x400003EDL

//
// MessageId: EVENT_SSSEARCH_SETUP_FAILED
//
// MessageText:
//
// The Windows Search Service has failed to create the new search index. Internal error <%2, %3, %4>. %1
//
#define EVENT_SSSEARCH_SETUP_FAILED      0xC00003EEL

//
// MessageId: EVENT_OUTOFMEMORY
//
// MessageText:
//
// The Windows Search Service was unable to allocate memory.%1
//
#define EVENT_OUTOFMEMORY                0xC00003EFL

//
// MessageId: EVENT_SSSEARCH_SETUP_CLEANUP_STARTED
//
// MessageText:
//
// The Windows Search Service is starting up and attempting to remove the old search index {Reason: %2}. %1
//
#define EVENT_SSSEARCH_SETUP_CLEANUP_STARTED 0x800003F0L

//
// MessageId: EVENT_EXCEPTION
//
// MessageText:
//
// An exception occurred in %2. Check other related Event Log messages.%1
//
#define EVENT_EXCEPTION                  0xC00003F1L

//
// MessageId: EVENT_SSSEARCH_SETUP_CLEANUP_SUCCEEDED
//
// MessageText:
//
// The Windows Search Service has successfully removed the old search index. %1
//
#define EVENT_SSSEARCH_SETUP_CLEANUP_SUCCEEDED 0x400003F2L

//
// MessageId: EVENT_SSSEARCH_SETUP_CLEANUP_FAILED
//
// MessageText:
//
// The Windows Search Service has failed to remove the old search index. Internal error <%2,%3>. %1
//
#define EVENT_SSSEARCH_SETUP_CLEANUP_FAILED 0xC00003F3L

//
// MessageId: EVENT_SSSEARCH_STOPPED
//
// MessageText:
//
// Windows Search Service stopped normally.%1
//
#define EVENT_SSSEARCH_STOPPED           0x400003F5L

//
// MessageId: EVENT_SSSEARCH_CREATE_PATH_RULES_FAILED
//
// MessageText:
//
// The Windows Search Service has failed to create one or more path rules. 
// The service will continue creating the SystemIndex search index. 
// Debug information: <%2>. %1
//
#define EVENT_SSSEARCH_CREATE_PATH_RULES_FAILED 0x800003F6L

//
// MessageId: EVENT_SSSEARCH_DROPPED_EVENTS
//
// MessageText:
//
// Event ID %2 for the Windows Search Service has been suppressed %3 time(s) since %4. This event is used to suppress Windows Search Service events that have occurred frequently within a short period of time.  See Event ID %2 for further details on this event.%1
//
#define EVENT_SSSEARCH_DROPPED_EVENTS    0x800003F7L

//
// MessageId: EVENT_SSSEARCH_DATAFILES_MOVE_FAILED
//
// MessageText:
//
// The Windows Search Service failed to move Index files from %2 to %3 with the following error: <%4,%5>. This might be because the target directory is not empty, or because the SYSTEM account doesn't have write access to the target directory. %1
//
#define EVENT_SSSEARCH_DATAFILES_MOVE_FAILED 0xC00003F8L

//
// MessageId: EVENT_SSSEARCH_DATAFILES_MOVE_SUCCEEDED
//
// MessageText:
//
// The Windows Search Service successfully moved index files from %2 to %3. %1
//
#define EVENT_SSSEARCH_DATAFILES_MOVE_SUCCEEDED 0x400003F9L

//
// MessageId: EVENT_SSSEARCH_DATAFILES_MOVE_ROLLBACK_ERRORS
//
// MessageText:
//
// While rolling back the index, the Windows Search Service encountered the following error: <%4,%5>. Index files were not moved from %2 to %3. %1
//
#define EVENT_SSSEARCH_DATAFILES_MOVE_ROLLBACK_ERRORS 0x800003FAL

//
// MessageId: EVENT_SSSEARCH_CSM_SAVE_FAILED
//
// MessageText:
//
// Windows Search Service failed to process the list of included and excluded locations with the error <%2, %3, "%4">. %1
//
#define EVENT_SSSEARCH_CSM_SAVE_FAILED   0xC00003FBL

//
// MessageId: EVENT_CONFIG_SYNTAX
//
// MessageText:
//
// An error occurred in configuration file <%2>.%1
//
#define EVENT_CONFIG_SYNTAX              0x80000414L

//
// MessageId: EVENT_UNPRIVILEGED_SERVICE_ACCOUNT
//
// MessageText:
//
// The account used to start the Windows Search Service does not have the 
// rights to operate the service.  The account must be either the LocalSystem 
// account or a member of the Administrators group.  Use the Services tab in the 
// Control Panel to change the startup account for the search service.%1
//
#define EVENT_UNPRIVILEGED_SERVICE_ACCOUNT 0x8000041CL

//
// MessageId: EVENT_SYSTEM_EXCEPTION
//
// MessageText:
//
// The system exception %1 occurred, and will be handled. 
// If this causes problems, contact Microsoft Product Support Services and include the stack trace in the event. %2.
//
#define EVENT_SYSTEM_EXCEPTION           0x8000041DL

//
// MessageId: EVENT_CONFIG_ERROR
//
// MessageText:
//
// A configuration error occurred.%1
//
#define EVENT_CONFIG_ERROR               0xC0000BBBL

//
// MessageId: EVENT_GATHERSVC_PERFMON
//
// MessageText:
//
// Performance monitoring cannot be initialized for the gatherer service, because the counters are not loaded or the shared memory object cannot be opened. This only affects availability of the perfmon counters. Restart the computer.%1
//
#define EVENT_GATHERSVC_PERFMON          0xC0000BBEL

//
// MessageId: EVENT_GATHERER_PERFMON
//
// MessageText:
//
// Performance monitoring cannot be initialized for the gatherer object, because the counters are not loaded or the shared memory object cannot be opened. This only affects availability of the perfmon counters. Restart the computer.%1
//
#define EVENT_GATHERER_PERFMON           0xC0000BBFL

//
// MessageId: EVENT_HASHMAP_INSERT
//
// MessageText:
//
// The entry <%2> cannot be inserted into the history.%1
//
#define EVENT_HASHMAP_INSERT             0xC0000BC0L

//
// MessageId: EVENT_TRANSLOG_CREATE_TRX
//
// MessageText:
//
// The transaction object cannot be created.%1
//
#define EVENT_TRANSLOG_CREATE_TRX        0xC0000BC1L

//
// MessageId: EVENT_TRANSLOG_APPEND
//
// MessageText:
//
// The transaction cannot be appended to the queue. File: %2.%1
//
#define EVENT_TRANSLOG_APPEND            0xC0000BC2L

//
// MessageId: EVENT_TRANSLOG_UPDATE
//
// MessageText:
//
// The transaction cannot be updated in the queue. File: %2.%1
//
#define EVENT_TRANSLOG_UPDATE            0xC0000BC3L

//
// MessageId: EVENT_HASHMAP_UPDATE
//
// MessageText:
//
// The entry <%2> in the hash map cannot be updated.%1
//
#define EVENT_HASHMAP_UPDATE             0xC0000BC5L

//
// MessageId: EVENT_GATHER_EXCEPTION
//
// MessageText:
//
// An exception occurred. ID: %2. This is an internal error. Reproduce the error with the debugger attached and enable exceptions, then contact product support. One of the components loaded in your system is bad. You may be able to avoid the problem by recreating the index.%1
//
#define EVENT_GATHER_EXCEPTION           0xC0000BC6L

//
// MessageId: EVENT_TRANSACTION_READ
//
// MessageText:
//
// The transaction file cannot be read.%1
//
#define EVENT_TRANSACTION_READ           0xC0000BC7L

//
// MessageId: EVENT_GATHER_END_CRAWL
//
// MessageText:
//
// The update is complete. The gatherer successfully processed %2 documents totaling %3K. It failed to filter %4 documents. %5 documents were modified. %6 URLs were not found or denied access.%1
//
#define EVENT_GATHER_END_CRAWL           0x40000BCAL

//
// MessageId: EVENT_GATHER_START_CRAWL
//
// MessageText:
//
// The update started.%1
//
#define EVENT_GATHER_START_CRAWL         0x40000BCBL

//
// MessageId: EVENT_GATHER_INTERNAL
//
// MessageText:
//
// Internal gatherer error %2 occurred. Please contact Microsoft Product Support Services.%1
//
#define EVENT_GATHER_INTERNAL            0xC0000BCCL

//
// MessageId: EVENT_GATHER_CRAWL_NOT_STARTED
//
// MessageText:
//
// The update cannot be started because all of the content sources were excluded by site path rules, or removed from the index configuration.%1
//
#define EVENT_GATHER_CRAWL_NOT_STARTED   0x80000BCFL

//
// MessageId: EVENT_GATHER_CRAWL_SEED_ERROR
//
// MessageText:
//
// The update cannot be started because the content sources cannot be accessed. Fix the errors and try the update again.%1
//
#define EVENT_GATHER_CRAWL_SEED_ERROR    0x80000BD0L

//
// MessageId: EVENT_GATHER_CRITICAL_ERROR
//
// MessageText:
//
// Critical error %2 occurred, and the index was shut down. The system is probably low on resources. Free up resources and restart the service.%1
//
#define EVENT_GATHER_CRITICAL_ERROR      0xC0000BD1L

//
// MessageId: EVENT_GATHER_ADVISE_FAILED
//
// MessageText:
//
// Advise Status Change failed. The system is probably low on resources. Free up resources and restart the service.%1
//
#define EVENT_GATHER_ADVISE_FAILED       0xC0000BD2L

//
// MessageId: EVENT_GATHER_TRANSACTION_FAIL
//
// MessageText:
//
// The URL <%2> cannot be crawled.%1
//
#define EVENT_GATHER_TRANSACTION_FAIL    0xC0000BD3L

//
// MessageId: EVENT_GATHER_OBJ_INIT_FAILED
//
// MessageText:
//
// The gatherer object cannot be initialized.%1
//
#define EVENT_GATHER_OBJ_INIT_FAILED     0xC0000BD4L

//
// MessageId: EVENT_GATHER_PLUGIN_INIT_FAILED
//
// MessageText:
//
// The plug-in in <%2> cannot be initialized.%1
//
#define EVENT_GATHER_PLUGIN_INIT_FAILED  0xC0000BD5L

//
// MessageId: EVENT_GATHER_SERVICE_INIT
//
// MessageText:
//
// The gatherer service cannot be initialized.%1
//
#define EVENT_GATHER_SERVICE_INIT        0xC0000BD6L

//
// MessageId: EVENT_GATHER_CANT_CREATE_DOCID
//
// MessageText:
//
// A document ID cannot be allocated.%1
//
#define EVENT_GATHER_CANT_CREATE_DOCID   0xC0000BD7L

//
// MessageId: EVENT_GATHER_CANT_DELETE_DOCID
//
// MessageText:
//
// A document ID cannot be freed.%1
//
#define EVENT_GATHER_CANT_DELETE_DOCID   0xC0000BD8L

//
// MessageId: EVENT_TRANSLOG_CREATE
//
// MessageText:
//
// A new queue file cannot be created.%1
//
#define EVENT_TRANSLOG_CREATE            0xC0000BD9L

//
// MessageId: EVENT_REG_VERSION
//
// MessageText:
//
// The registry version does not match with the expected <%2>, or the registry cannot be accessed because the service account does not have the correct permissions.  Uninstall the previous version before installing the new one.%1
//
#define EVENT_REG_VERSION                0xC0000BDAL

//
// MessageId: EVENT_GATHER_CRAWL_SEED_FAILED
//
// MessageText:
//
// Crawl could not be completed on content source <%2>.%1
//
#define EVENT_GATHER_CRAWL_SEED_FAILED   0x80000BDCL

//
// MessageId: EVENT_GATHER_CRAWL_SEED_FAILED_INIT
//
// MessageText:
//
// Crawl could not be started on content source <%2>.%1
//
#define EVENT_GATHER_CRAWL_SEED_FAILED_INIT 0x80000BDDL

//
// MessageId: EVENT_GATHER_REG_MISSING
//
// MessageText:
//
// The gatherer is unable to read the registry %2.%1
//
#define EVENT_GATHER_REG_MISSING         0x80000BDEL

//
// MessageId: EVENT_GATHER_CRAWL_IN_PROGRESS
//
// MessageText:
//
// A request to start the update has been ignored because the update is already in progress or is scheduled on one or more content sources.%1
//
#define EVENT_GATHER_CRAWL_IN_PROGRESS   0x80000BDFL

//
// MessageId: EVENT_GATHER_LOCK_FAILED
//
// MessageText:
//
// The status change request %2 cannot be processed.%1
//
#define EVENT_GATHER_LOCK_FAILED         0xC0000BE0L

//
// MessageId: EVENT_GATHER_RESET_START
//
// MessageText:
//
// The index is being reset.%1
//
#define EVENT_GATHER_RESET_START         0x40000BE1L

//
// MessageId: EVENT_GATHER_START_PAUSE
//
// MessageText:
//
// The index was paused.%1
//
#define EVENT_GATHER_START_PAUSE         0x80000BE2L

//
// MessageId: EVENT_GATHER_THROTTLE
//
// MessageText:
//
// The gatherer index was interrupted.%1
//
#define EVENT_GATHER_THROTTLE            0x40000BE3L

//
// MessageId: EVENT_GATHER_RESUME
//
// MessageText:
//
// The gatherer index resumed.%1
//
#define EVENT_GATHER_RESUME              0x40000BE4L

//
// MessageId: EVENT_GATHER_AUTODESCLEN_ADJUSTED
//
// MessageText:
//
// The automatic description length was adjusted from %2 to %3.%1
//
#define EVENT_GATHER_AUTODESCLEN_ADJUSTED 0x80000BE5L

//
// MessageId: EVENT_GATHER_NO_CRAWL_SEEDS
//
// MessageText:
//
// The update for the index cannot be started because the specified content sources were not configured for updates. Add at least one content source.%1
//
#define EVENT_GATHER_NO_CRAWL_SEEDS      0x80000BE6L

//
// MessageId: EVENT_GATHER_END_INCREMENTAL
//
// MessageText:
//
// The incremental update is complete. The gatherer successfully processed %2 documents totaling %3K. It failed to filter %4 documents. %5 documents were modified. %6 URLs were not found or denied access.%1
//
#define EVENT_GATHER_END_INCREMENTAL     0x40000BE7L

//
// MessageId: EVENT_GATHER_FROM_NOT_SET
//
// MessageText:
//
// No documents were accessed because no e-mail address is specified in the content index server properties. Specify the e-mail address in the service configuration.%1
//
#define EVENT_GATHER_FROM_NOT_SET        0xC0000BE8L

//
// MessageId: EVENT_GATHER_DELETING_HISTORY_ITEMS
//
// MessageText:
//
// Unvisited items cannot be deleted from the history after a full update.%1
//
#define EVENT_GATHER_DELETING_HISTORY_ITEMS 0xC0000BEAL

//
// MessageId: EVENT_GATHER_STOP_START
//
// MessageText:
//
// The crawl was requested to be stopped.%1
//
#define EVENT_GATHER_STOP_START          0x40000BECL

//
// MessageId: EVENT_GATHER_START_CRAWL_IF_RESET
//
// MessageText:
//
// The previous update was reset, or was otherwise interrupted. A full update of all content sources will be automatically started. %1
//
#define EVENT_GATHER_START_CRAWL_IF_RESET 0x80000BEDL

//
// MessageId: EVENT_GATHER_DISK_FULL
//
// MessageText:
//
// The update has been delayed because a disk is full. Check the system default temp location and the drive on which search catalog is created. The system default temp location is used for creation of temporary files during crawling. If it is full, crawling pauses. If the system default temp location is full, change the location to a disk with more free space and restart the computer. Changes to the system temp location do not take effect for system services until the computer is restarted.%1
//
#define EVENT_GATHER_DISK_FULL           0x80000BEEL

//
// MessageId: EVENT_GATHER_NO_SCHEMA
//
// MessageText:
//
// The gatherer property mapping file cannot be opened. The default values are being used. You may have to copy the property mapping file from the setup CD, or reinstall the application.%1
//
#define EVENT_GATHER_NO_SCHEMA           0x80000BEFL

//
// MessageId: EVENT_GATHER_AUTODESCENCODE_INVALID
//
// MessageText:
//
// The automatic description encoding tag value is invalid. The gatherer is setting this value to "yes"". Fix the gthrprm.txt file.%1
//
#define EVENT_GATHER_AUTODESCENCODE_INVALID 0x80000BF0L

//
// MessageId: EVENT_GATHER_PLUGINMGR_INIT_FAILED
//
// MessageText:
//
// The plug-in manager <%2> cannot be initialized.%1
//
#define EVENT_GATHER_PLUGINMGR_INIT_FAILED 0xC0000BF1L

//
// MessageId: EVENT_GATHER_APP_INIT_FAILED
//
// MessageText:
//
// The application cannot be initialized.%1
//
#define EVENT_GATHER_APP_INIT_FAILED     0xC0000BF2L

//
// MessageId: EVENT_FAILED_INITIALIZE_CRAWL
//
// MessageText:
//
// The update cannot be initialized.%1
//
#define EVENT_FAILED_INITIALIZE_CRAWL    0xC0000BF3L

//
// MessageId: EVENT_CRAWL_SCHEDULED
//
// MessageText:
//
// An update cannot begin because the content source <%2> is in use by another update already in progress. The update will start as soon as all its content sources are released by updates already in progress.%1
//
#define EVENT_CRAWL_SCHEDULED            0x40000BF4L

//
// MessageId: EVENT_FAILED_CREATE_GATHERER_LOG
//
// MessageText:
//
// The gatherer log cannot be created.%1
//
#define EVENT_FAILED_CREATE_GATHERER_LOG 0x80000BF5L

//
// MessageId: EVENT_WBREAKER_NOT_LOADED
//
// MessageText:
//
// The word breaker for language <%2> cannot be loaded.%1
//
#define EVENT_WBREAKER_NOT_LOADED        0x80000BF6L

//
// MessageId: EVENT_LEARN_PROPAGATION_COPY_FAILED
//
// MessageText:
//
// Propagation failed while copying <%2> to <%3>.%1
//
#define EVENT_LEARN_PROPAGATION_COPY_FAILED 0x80000BF7L

//
// MessageId: EVENT_LEARN_CREATE_DB_FAILED
//
// MessageText:
//
// The Topic Assistant training database could not be created.%1
//
#define EVENT_LEARN_CREATE_DB_FAILED     0x80000BF8L

//
// MessageId: EVENT_LEARN_COMPILE_FAILED
//
// MessageText:
//
// The Topic Assistant training database could not be compiled.%1
//
#define EVENT_LEARN_COMPILE_FAILED       0x80000BF9L

//
// MessageId: EVENT_LEARN_PROPAGATION_FAILED
//
// MessageText:
//
// Propagation of the Topic Assistant training database to <%2:%3:%4> failed.%1
//
#define EVENT_LEARN_PROPAGATION_FAILED   0x80000BFAL

//
// MessageId: EVENT_GATHER_END_ADAPTIVE
//
// MessageText:
//
// The adaptive update is complete. The gatherer successfully processed %2 documents totaling %3K. It failed to filter %4 documents. %5 documents were modified. %6 URLs were not found or denied access.%1
//
#define EVENT_GATHER_END_ADAPTIVE        0x40000BFBL

//
// MessageId: EVENT_USING_DIFFERENT_WORD_BREAKER
//
// MessageText:
//
// The gatherer is using the word breaker for language id <%2> for text in language id <%3>. The corresponding language resources are not installed on your computer.%1
//
#define EVENT_USING_DIFFERENT_WORD_BREAKER 0x80000BFCL

// NewLocStr 11/9/99
//
// MessageId: EVENT_GATHER_RESTORE_COMPLETE
//
// MessageText:
//
// The application was successfully restored.%1
//
#define EVENT_GATHER_RESTORE_COMPLETE    0x00000BFDL

// NewLocStr 11/9/99
//
// MessageId: EVENT_GATHER_RESTORE_ERROR
//
// MessageText:
//
// The index cannot be restored. You may need to delete and recreate the index.%1
//
#define EVENT_GATHER_RESTORE_ERROR       0xC0000BFEL

// NewLocStr 2/2/00
//
// MessageId: EVENT_AUTOCAT_PERFMON
//
// MessageText:
//
// Performance monitoring cannot be initialized for the topic assistant object, because the counters are not loaded or the shared memory object cannot be opened. This only affects availability of the perfmon counters. Restart the computer.%1
//
#define EVENT_AUTOCAT_PERFMON            0xC0000BFFL

// NewLocStr 2/15/00
//
// MessageId: EVENT_GATHER_DIRTY_STARTUP
//
// MessageText:
//
// The gatherer is recovering after an improper shutdown.  This will delay availability of gathering functions, and may result in some noncritical data loss.%1
//
#define EVENT_GATHER_DIRTY_STARTUP       0x80000C00L

// NewLocStr 2/15/00
//
// MessageId: EVENT_GATHER_HISTORY_CORRUPTION_DETECTED
//
// MessageText:
//
// The gatherer detected pages in the history during recovery that cannot be read, and repaired them.  However, statistical data for some URLs may have been lost.  This can be caused by restarting a computer without first shutting down Windows, or by disk failure.%1
//
#define EVENT_GATHER_HISTORY_CORRUPTION_DETECTED 0x80000C01L

// NewLocStr 4/7/00
//
// MessageId: EVENT_GATHER_RESTOREAPP_ERROR
//
// MessageText:
//
// The application restore failed. This can be caused by system errors (indicated by previously logged events), or a corrupt backup image (retry restore with valid backup image). %1
//
#define EVENT_GATHER_RESTOREAPP_ERROR    0xC0000C02L

// NewLocStr 4/7/00
//
// MessageId: EVENT_GATHER_RESTOREAPP_COMPLETE
//
// MessageText:
//
// The application was successfully restored.%1
//
#define EVENT_GATHER_RESTOREAPP_COMPLETE 0x00000C03L

// NewLocStr 7/10/00
//
// MessageId: EVENT_GATHER_BACKUPAPP_ERROR
//
// MessageText:
//
// The application cannot be backed up. You may be unable to restore from the backup image.%1
//
#define EVENT_GATHER_BACKUPAPP_ERROR     0xC0000C04L

// NewLocStr 7/10/00
//
// MessageId: EVENT_GATHER_BACKUPAPP_COMPLETE
//
// MessageText:
//
// The application was successfully backed up.%1
//
#define EVENT_GATHER_BACKUPAPP_COMPLETE  0x00000C05L

// NewLocStr 7/31/00
//
// MessageId: EVENT_GATHER_DAEMON_TERMINATED
//
// MessageText:
//
// The Windows Search service stopped the Protocol Host process because it was consuming too many resources.  A new Protocol Host process will be started.  No user action is required.%1
//
#define EVENT_GATHER_DAEMON_TERMINATED   0x80000C06L

// NewLocStr 7/31/00
//
// MessageId: EVENT_NOTIFICATION_FAILURE
//
// MessageText:
//
// Notifications for the volume %2 are not active. %1
//
#define EVENT_NOTIFICATION_FAILURE       0xC0000C07L

// NewLocStr 7/31/00
//
// MessageId: EVENT_NOTIFICATION_FAILURE_SCOPE_EXCEEDED_LOGGING
//
// MessageText:
//
// Notifications for the scope %2 are not active. The event logging threshold for this scope was exceeded.  No further events will be sent for one hour. %1
//
#define EVENT_NOTIFICATION_FAILURE_SCOPE_EXCEEDED_LOGGING 0x80000C08L

// NewLocStr 7/31/00
//
// MessageId: EVENT_NOTIFICATION_RESTORED
//
// MessageText:
//
// Notifications for the scope %2 have been reactivated. %1
//
#define EVENT_NOTIFICATION_RESTORED      0x40000C09L

// NewLocStr 7/31/00
//
// MessageId: EVENT_NOTIFICATION_RESTORED_SCOPE_EXCEEDED_LOGGING
//
// MessageText:
//
// Notifications for the scope %2 were reactivated. The event logging threshold for this scope was exceeded.  No further events will be sent for one hour. %1
//
#define EVENT_NOTIFICATION_RESTORED_SCOPE_EXCEEDED_LOGGING 0x80000C0AL

//
// MessageId: EVENT_GATHER_PROTOCOLHANDLER_LOAD_FAILED
//
// MessageText:
//
// The protocol handler %2 cannot be loaded. Error description: %3. %1
//
#define EVENT_GATHER_PROTOCOLHANDLER_LOAD_FAILED 0xC0000C0BL

//
// MessageId: EVENT_GATHER_PROTOCOLHANDLER_INIT_FAILED
//
// MessageText:
//
// Failed to load protocol handler %2. Error description: %3. %1
//
#define EVENT_GATHER_PROTOCOLHANDLER_INIT_FAILED 0xC0000C0CL

//
// MessageId: EVENT_GATHER_INVALID_NETWORK_ACCESS_ACCOUNT
//
// MessageText:
//
// The application network access account is invalid.  Update the account with a valid username and password. %1
//
#define EVENT_GATHER_INVALID_NETWORK_ACCESS_ACCOUNT 0xC0000C0DL

//
// MessageId: EVENT_GATHER_SYSTEM_LCID_CHANGED
//
// MessageText:
//
// The system locale has changed. Existing data will be deleted and the index must be recreated.%1
//
#define EVENT_GATHER_SYSTEM_LCID_CHANGED 0x80000C0EL

//
// MessageId: EVENT_GATHER_FLUSH_FAILED
//
// MessageText:
//
// The gatherer files cannot be flushed, and this action cannot be completed. The gatherer will attempt to flush files again. If the problem persists, restart the service, free system resources or verify that your hardware is working properly. %1
//
#define EVENT_GATHER_FLUSH_FAILED        0xC0000C0FL

//
// MessageId: EVENT_GATHER_CHECKPOINT_FAILED
//
// MessageText:
//
// The checkpoint record cannot be updated, and this action cannot be completed. The gatherer will attempt to update the checkpoint record again. If the problem persists, restart the service, free system resources or verify that your hardware is working properly. %1
//
#define EVENT_GATHER_CHECKPOINT_FAILED   0xC0000C10L

//
// MessageId: EVENT_GATHER_SAVE_FAILED
//
// MessageText:
//
// The gatherer files cannot be saved, and this action cannot be completed. The gatherer will attempt to save the files again. If the problem persists, restart the service, free system resources or verify that your hardware is working properly. %1
//
#define EVENT_GATHER_SAVE_FAILED         0xC0000C11L

//
// MessageId: EVENT_GATHER_RESTORE_CHECKPOINT_FAILED
//
// MessageText:
//
// The gatherer files from the previous checkpoint cannot be restored, and this action cannot be completed. The gatherer will attempt to restore the files again. If the problem persists, restart the service, free system resources or verify that your hardware is working properly. %1
//
#define EVENT_GATHER_RESTORE_CHECKPOINT_FAILED 0xC0000C12L

//
// MessageId: EVENT_GATHER_READ_CHECKPOINT_FAILED
//
// MessageText:
//
// The checkpoint record cannot be read, and this action cannot be completed.  If the problem persists, restart the service, free system resources or verify that your hardware is working properly. %1
//
#define EVENT_GATHER_READ_CHECKPOINT_FAILED 0xC0000C13L

//
// MessageId: EVENT_GATHER_CHECKPOINT_CORRUPT
//
// MessageText:
//
// The project cannot be initialized, because the checkpoint record cannot be read. The data structures on the disk will be reset.  Verify that your hardware is working properly. %1
//
#define EVENT_GATHER_CHECKPOINT_CORRUPT  0xC0000C14L

//
// MessageId: EVENT_GATHER_CHECKPOINT_FILE_MISSING
//
// MessageText:
//
// The project cannot be initialized, because one of the checkpoint files is missing. The data structures on the disk will be reset.  Check to see if someone is manually deleting files, and verify that your hardware is working properly. %1
//
#define EVENT_GATHER_CHECKPOINT_FILE_MISSING 0xC0000C15L

//
// MessageId: EVENT_STS_INIT_SECURITY_FAILED
//
// MessageText:
//
// Security information was not obtained from server %2.%1
//
#define EVENT_STS_INIT_SECURITY_FAILED   0x80000C16L

//
// MessageId: EVENT_LOCAL_GROUP_NOT_EXPANDED
//
// MessageText:
//
// The group %2\%3 contains %4 members. Groups over %5 members are not expanded. %1
//
#define EVENT_LOCAL_GROUP_NOT_EXPANDED   0x40000C17L

//
// MessageId: EVENT_LOCAL_GROUPS_CACHE_FLUSHED
//
// MessageText:
//
// The local groups cache was flushed, because %2. %1
//
#define EVENT_LOCAL_GROUPS_CACHE_FLUSHED 0x40000C18L

//
// MessageId: EVENT_GATHERER_DATASOURCE
//
// MessageText:
//
// The gatherer did not connect to the SQLServer instance.%1
//
#define EVENT_GATHERER_DATASOURCE        0xC0000C19L

//
// MessageId: EVENT_AUTOCAT_CANT_CREATE_FILE_SHARE
//
// MessageText:
//
// Unable to create file share to accept topic assistant training data. Reason: %2.%1
//
#define EVENT_AUTOCAT_CANT_CREATE_FILE_SHARE 0xC0000C1AL

//
// MessageId: EVENT_NOTIFICATION_THREAD_EXIT_FAILED
//
// MessageText:
//
// Unable to terminate notifications normally.  Restart the service or contact Product Support.%1
//
#define EVENT_NOTIFICATION_THREAD_EXIT_FAILED 0xC0000C1BL

//
// MessageId: EVENT_FILTER_HOST_NOT_INITIALIZED
//
// MessageText:
//
// Unable to initialize the filter host process. Terminating.%1
//
#define EVENT_FILTER_HOST_NOT_INITIALIZED 0xC0000C1CL

//
// MessageId: EVENT_FILTER_HOST_NOT_TERMINATED
//
// MessageText:
//
// The filter host process could not be terminated.
//
#define EVENT_FILTER_HOST_NOT_TERMINATED 0xC0000C1DL

//
// MessageId: EVENT_FILTERPOOL_ADD_FAILED
//
// MessageText:
//
// The per-user filter pool for session %2 could not be added.%1
//
#define EVENT_FILTERPOOL_ADD_FAILED      0xC0000C1EL

//
// MessageId: EVENT_FILTERPOOL_DELETE_FAILED
//
// MessageText:
//
// The per-user filter pool for user with SID {%2} could not be removed. %1
//
#define EVENT_FILTERPOOL_DELETE_FAILED   0xC0000C1FL

//
// MessageId: EVENT_ENUMERATE_SESSIONS_FAILED
//
// MessageText:
//
// Enumerating user sessions to generate filter pools failed.%1
//
#define EVENT_ENUMERATE_SESSIONS_FAILED  0xC0000C20L

//
// MessageId: EVENT_DETAILED_FILTERPOOL_ADD_FAILED
//
// MessageText:
//
// The per-user filter pool for session %2 and user context token %3 could not be added <%4,%5>.%1
//
#define EVENT_DETAILED_FILTERPOOL_ADD_FAILED 0xC0000C21L

//
// MessageId: EVENT_AUDIENCECOMPUTATION_CANNOTSTART
//
// MessageText:
//
// The audience compilation process cannot start. The error code is "%2" .%1
//
#define EVENT_AUDIENCECOMPUTATION_CANNOTSTART 0xC0000E11L

//
// MessageId: EVENT_GATHER_RECOVERY_FAILURE
//
// MessageText:
//
// Error ID %2 happened in Windows Search recovery stage, please restart the service. If this error persists, please recreate the index.%1
//
#define EVENT_GATHER_RECOVERY_FAILURE    0xC0000E12L

//
// MessageId: EVENT_GATHER_INPLACE_INDEX_REBUILD
//
// MessageText:
//
// The Windows Search Service is rebuilding the index inplace.%1
//
#define EVENT_GATHER_INPLACE_INDEX_REBUILD 0x40000E13L

//
// MessageId: EVENT_INDEXER_STARTED
//
// MessageText:
//
// The content index service started successfully.%1
//
#define EVENT_INDEXER_STARTED            0x40001B58L

//
// MessageId: EVENT_INDEXER_SCHEMA_COPY_ERROR
//
// MessageText:
//
// The schema file <%2> cannot be copied to <%3>.%1
//
#define EVENT_INDEXER_SCHEMA_COPY_ERROR  0xC0001B59L

//
// MessageId: EVENT_INDEXER_INIT_ERROR
//
// MessageText:
//
// The index cannot be initialized.%1
//
#define EVENT_INDEXER_INIT_ERROR         0xC0001B62L

//
// MessageId: EVENT_INDEXER_INVALID_DIRECTORY
//
// MessageText:
//
// Directory location <%2> is invalid. The application configuration cannot be read.  Reinstall the application.%1
//
#define EVENT_INDEXER_INVALID_DIRECTORY  0xC0001B63L

//
// MessageId: EVENT_INDEXER_PROP_ERROR
//
// MessageText:
//
// An error occurred while propagating to search server <%2>.%1
//
#define EVENT_INDEXER_PROP_ERROR         0xC0001B64L

//
// MessageId: EVENT_INDEXER_PAUSED_FOR_DISKFULL
//
// MessageText:
//
// The update was paused because the disk <%2> is full. Free up disk space to continue crawling the index.%1
//
#define EVENT_INDEXER_PAUSED_FOR_DISKFULL 0xC0001B65L

//
// MessageId: EVENT_INDEXER_PROP_STOPPED
//
// MessageText:
//
// Index propagation to search server <%2> was stopped by a user.%1
//
#define EVENT_INDEXER_PROP_STOPPED       0x80001B67L

//
// MessageId: EVENT_INDEXER_PROP_SUCCEEDED
//
// MessageText:
//
// Index propagation to search server <%2> succeeded.  The service is now waiting for the search server to accept the propagation.%1
//
#define EVENT_INDEXER_PROP_SUCCEEDED     0x00001B68L

//
// MessageId: EVENT_INDEXER_PROP_STARTED
//
// MessageText:
//
// Index propagation to search server <%2> started.%1
//
#define EVENT_INDEXER_PROP_STARTED       0x40001B69L

//
// MessageId: EVENT_INDEXER_NO_SEARCH_SERVERS
//
// MessageText:
//
// There are no search servers selected.  Reconfigure the index to identify a suitable search server.  If the problem persists, delete and recreate the index.%1
//
#define EVENT_INDEXER_NO_SEARCH_SERVERS  0x80001B6AL

//
// MessageId: EVENT_INDEXER_ADD_DSS_SUCCEEDED
//
// MessageText:
//
// The search server <%2> was successfully added.%1
//
#define EVENT_INDEXER_ADD_DSS_SUCCEEDED  0x00001B6BL

//
// MessageId: EVENT_INDEXER_REMOVE_DSS_SUCCEEDED
//
// MessageText:
//
// The search server <%2> was successfully removed.%1
//
#define EVENT_INDEXER_REMOVE_DSS_SUCCEEDED 0x00001B6CL

//
// MessageId: EVENT_INDEXER_ADD_DSS_FAILED
//
// MessageText:
//
// The search server <%2> cannot be added.%1
//
#define EVENT_INDEXER_ADD_DSS_FAILED     0x80001B6DL

//
// MessageId: EVENT_INDEXER_REMOVE_DSS_FAILED
//
// MessageText:
//
// The search server <%2> cannot be removed.%1
//
#define EVENT_INDEXER_REMOVE_DSS_FAILED  0xC0001B6FL

//
// MessageId: EVENT_INDEXER_DSS_CONTACT_FAILED
//
// MessageText:
//
// Failed to inform the search server <%2> about a propagation error.%1
//
#define EVENT_INDEXER_DSS_CONTACT_FAILED 0xC0001B70L

//
// MessageId: EVENT_INDEXER_BUILD_FAILED
//
// MessageText:
//
// The index cannot be copied, and propagation cannot start.%1
//
#define EVENT_INDEXER_BUILD_FAILED       0xC0001B73L

//
// MessageId: EVENT_INDEXER_REG_MISSING
//
// MessageText:
//
// The content index server cannot read the registry.%1
//
#define EVENT_INDEXER_REG_MISSING        0xC0001B74L

//
// MessageId: EVENT_INDEXER_PROPSTORE_INIT_FAILED
//
// MessageText:
//
// The property store was not initialized.%1
//
#define EVENT_INDEXER_PROPSTORE_INIT_FAILED 0xC0001B7DL

//
// MessageId: EVENT_INDEXER_CI_LOAD_ERROR
//
// MessageText:
//
// The content index cannot be loaded.%1
//
#define EVENT_INDEXER_CI_LOAD_ERROR      0xC0001B7FL

//
// MessageId: EVENT_INDEXER_RESET_FOR_CORRUPTION
//
// MessageText:
//
// The search service has detected corrupted data files in the index {id=%2}. The service will attempt to automatically correct this problem by rebuilding the index.%1
//
#define EVENT_INDEXER_RESET_FOR_CORRUPTION 0xC0001B80L

//
// MessageId: EVENT_INDEXER_SHUTDOWN
//
// MessageText:
//
// The Windows Search Service is being stopped because there is a problem with the indexer: %2.%1
//
#define EVENT_INDEXER_SHUTDOWN           0x40001B82L

//
// MessageId: EVENT_INDEXER_LOAD_FAIL
//
// MessageText:
//
// The index cannot be loaded.%1
//
#define EVENT_INDEXER_LOAD_FAIL          0xC0001B83L

//
// MessageId: EVENT_INDEXER_PROP_STATE_CORRUPT
//
// MessageText:
//
// Propagation stopped because the propagation state for search server <%2> cannot be read. Try propagation again.  If this error persists, delete and recreate the propagated index.%1
//
#define EVENT_INDEXER_PROP_STATE_CORRUPT 0xC0001B84L

//
// MessageId: EVENT_INDEXER_DSS_ALREADY_ADDED
//
// MessageText:
//
// The index already exists on search server <%2>.%1
//
#define EVENT_INDEXER_DSS_ALREADY_ADDED  0x40001B86L

//
// MessageId: EVENT_INDEXER_BUILD_START
//
// MessageText:
//
// The index is being copied to the drop location in preparation for propagation.%1
//
#define EVENT_INDEXER_BUILD_START        0x40001B88L

//
// MessageId: EVENT_INDEXER_BUILD_ENDED
//
// MessageText:
//
// The index was completely copied.  The index will now be propagated.%1
//
#define EVENT_INDEXER_BUILD_ENDED        0x40001B89L

//
// MessageId: EVENT_INDEXER_VERIFY_PROP_ACCOUNT
//
// MessageText:
//
// The content index server received an "Access Denied"" error while propagating to the <%2> search server. This usually results from an incorrect configuration of the propagation account. Check that the propagation account is valid.%1
//
#define EVENT_INDEXER_VERIFY_PROP_ACCOUNT 0xC0001B90L

//
// MessageId: EVENT_INDEXER_ADD_DSS_DISCONNECT
//
// MessageText:
//
// Search server <%2> cannot be contacted. However, <%2> was successfully added as a propagation recipient.%1
//
#define EVENT_INDEXER_ADD_DSS_DISCONNECT 0x80001B97L

//
// MessageId: EVENT_INDEXER_PERFMON
//
// MessageText:
//
// Performance monitoring cannot be initialized because the counters are not loaded or the shared memory object cannot be opened. Stop and restart the search service.  If this error continues, reinstall the application.%1
//
#define EVENT_INDEXER_PERFMON            0xC0001B98L

//
// MessageId: EVENT_INDEXER_MISSING_APP_DIRECTORY
//
// MessageText:
//
// Configuration directory %2 is missing, and disaster recovery must be performed. If there are existing indexes, they must be restored from the last backup. If there is no backup of index data, then delete the catalogs and recreate them.%1
//
#define EVENT_INDEXER_MISSING_APP_DIRECTORY 0xC0001B9AL

//
// MessageId: EVENT_INDEXER_REG_ERROR
//
// MessageText:
//
// The registry cannot be read, possibly because the registry keys for this index are missing. You may have to delete and recreate the index %1.
//
#define EVENT_INDEXER_REG_ERROR          0xC0001B9CL

//
// MessageId: EVENT_INDEXER_DSS_UNABLE_TO_REMOVE
//
// MessageText:
//
// The content index %1 cannot be removed.
//
#define EVENT_INDEXER_DSS_UNABLE_TO_REMOVE 0xC0001B9DL

// NewLocStr 7/26/00
//
// MessageId: EVENT_INDEXER_NEW_PROJECT
//
// MessageText:
//
// The Windows Search Service added catalog %1
//
#define EVENT_INDEXER_NEW_PROJECT        0xC0001B9EL

// NewLocStr 7/26/00
//
// MessageId: EVENT_INDEXER_REMOVED_PROJECT
//
// MessageText:
//
// The Windows Search Service removed index %1
//
#define EVENT_INDEXER_REMOVED_PROJECT    0xC0001B9FL

//
// MessageId: EVENT_INDEXER_PROP_COMMITTED
//
// MessageText:
//
// Index propagation was committed to at least one target search server.%1
//
#define EVENT_INDEXER_PROP_COMMITTED     0x40001BA2L

//
// MessageId: EVENT_INDEXER_PROP_ABORTED
//
// MessageText:
//
// Index propagation stopped.%1
//
#define EVENT_INDEXER_PROP_ABORTED       0x40001BA3L

//
// MessageId: EVENT_DSS_NOT_ENABLED
//
// MessageText:
//
// The search server <%2> cannot be brought online.%1
//
#define EVENT_DSS_NOT_ENABLED            0x80001BA4L

//
// MessageId: EVENT_INDEXER_PROP_COMMIT_FAILED
//
// MessageText:
//
// Index propagation failed to commit to any of the search servers.  The Windows Search Service is now trying to revert the search servers to the previous index. %1
//
#define EVENT_INDEXER_PROP_COMMIT_FAILED 0xC0001BA5L

//
// MessageId: JET_INIT_ERROR
//
// MessageText:
//
// The Windows Search Service cannot open the Jet property store.%1
//
#define JET_INIT_ERROR                   0xC0002328L

//
// MessageId: JET_NEW_PROP_STORE_ERROR
//
// MessageText:
//
// The Windows Search Service cannot create a Jet property store.%1
//
#define JET_NEW_PROP_STORE_ERROR         0xC0002329L

//
// MessageId: JET_GET_PROP_STORE_ERROR
//
// MessageText:
//
// The Windows Search Service cannot load the property store information.%1
//
#define JET_GET_PROP_STORE_ERROR         0xC000232AL

//
// MessageId: JET_MULTIINSTANCE_DISABLED
//
// MessageText:
//
// The Windows Search Service cannot initialize multi-instancing in Jet. If the application is used in a cluster environment, all applications using Jet will fail in the same group.%1
//
#define JET_MULTIINSTANCE_DISABLED       0x8000232BL

// NewLocStr 5/12/00
//
// MessageId: EVENT_WARNING_CANNOT_UPGRADE_NOISE_FILES
//
// MessageText:
//
// The noise files cannot be renamed.%1
//
#define EVENT_WARNING_CANNOT_UPGRADE_NOISE_FILES 0x8000271DL

// NewLocStr 5/12/00
//
// MessageId: EVENT_WARNING_CANNOT_UPGRADE_NOISE_FILE
//
// MessageText:
//
// The noise file "%2"" cannot be renamed to ""%3"".%1
//
#define EVENT_WARNING_CANNOT_UPGRADE_NOISE_FILE 0x8000271EL

// LocStr Re-added 6/28/00
//uebug Unless it is absolutely necessary, the "Unexpected Win32  rror:" part of the message should be removed.
//
// MessageId: EVENT_WIN32_ERROR
//
// MessageText:
//
// %2Unexpected Win32 error: %3 failed in %4. Error: %5. %1
//
#define EVENT_WIN32_ERROR                0x8000271FL

// NewLocStr 5/29/02
//
// MessageId: EVENT_PERF_COUNTERS_NOT_LOADED
//
// MessageText:
//
// Performance Counters could not be loaded for %1 for instance %2 %3 due to the following error: %4.
//
#define EVENT_PERF_COUNTERS_NOT_LOADED   0x80002724L

// NewLocStr 5/29/02
//
// MessageId: EVENT_PERF_COUNTERS_REGISTRY_TROUBLE
//
// MessageText:
//
// Could not get performance counter registry info for %1 for instance %2 %3 due to the following error: %4.
//
#define EVENT_PERF_COUNTERS_REGISTRY_TROUBLE 0x80002725L

// NewLocStr 5/29/02
//
// MessageId: EVENT_PERF_COUNTERS_ALREADY_EXISTS
//
// MessageText:
//
// Performance counters will not be loaded because the named objects (shared memory or events) are in use for %1 for instance %2 %3.
//
#define EVENT_PERF_COUNTERS_ALREADY_EXISTS 0x80002726L

//
// MessageId: EVENT_PROTOCOL_HOST_FORCE_TERMINATE
//
// MessageText:
//
// The protocol host process %2 did not respond and is being forcibly terminated {filter host process %3}. %1
//
#define EVENT_PROTOCOL_HOST_FORCE_TERMINATE 0x80002727L

//
// MessageId: EVENT_FILTER_HOST_FORCE_TERMINATE
//
// MessageText:
//
// The filter host process %2 did not respond and is being forcibly terminated. %1
//
#define EVENT_FILTER_HOST_FORCE_TERMINATE 0x80002728L

//
// MessageId: EVENT_INDEXER_OUT_OF_DATABASE_INSTANCE
//
// MessageText:
//
// The search service has failed to create database instance for the index {%1} due to maximum number of instance reached. Please re-configure maximum value and restart the service.
//
#define EVENT_INDEXER_OUT_OF_DATABASE_INSTANCE 0xC0002729L

//
// MessageId: EVENT_INDEXER_FAIL_TO_SET_MAX_JETINSTANCE
//
// MessageText:
//
// The search service has failed to configure maximum number of database instance. {%1}. Please re-configure maximum value and restart the service.
//
#define EVENT_INDEXER_FAIL_TO_SET_MAX_JETINSTANCE 0xC000272AL

//
// MessageId: EVENT_INDEXER_FAIL_TO_CREATE_PER_USER_CATALOG
//
// MessageText:
//
// The search service has failed to create or load catalog for an user with SID {%2}. Please inspect a profile directory for the user is accessible. You should re-start the search service after fixing any issue found from the profile directory. %1
//
#define EVENT_INDEXER_FAIL_TO_CREATE_PER_USER_CATALOG 0xC000272BL

//
// MessageId: EVENT_INDEXER_FAIL_TO_UNLOAD_PER_USER_CATALOG
//
// MessageText:
//
// The search service has failed to unload catalog for an user with SID {%2}. %1
//
#define EVENT_INDEXER_FAIL_TO_UNLOAD_PER_USER_CATALOG 0xC000272CL


// errorlst.mc

#define ERROR_SOURCE_NETWORKING         0x0300
#define ERROR_SOURCE_DATASOURCE         0x0400
#define ERROR_SOURCE_COLLATOR           0x0500
#define ERROR_SOURCE_CONNMGR            0x0600
#define ERROR_SOURCE_QUERY              0x0700
#define ERROR_SOURCE_SCHEMA             0x0C00
#define ERROR_SOURCE_GATHERER           0x0D00
// DO NOT use 0x0E00.  Errors 0x0E00 - 0x0EFF are OLE-DB error codes
//#define ERROR_SOURCE_CATALOG            0x0F00
#define ERROR_SOURCE_INDEXER            0x1100
#define ERROR_SOURCE_SETUP              0x1300
#define ERROR_SOURCE_SECURITY           0x1400
#define ERROR_SOURCE_CMDLINE            0x1500
// DO NOT use 0x1600 - 0x1800   these belong to CI
#define ERROR_SOURCE_NLADMIN            0x1900
#define ERROR_SOURCE_SCRIPTPI           0x2000
#define ERROR_SOURCE_MSS                0x2100
#define ERROR_SOURCE_XML                0x2200
#define ERROR_SOURCE_DAV                0x2300
#define ERROR_SOURCE_FLTRDMN            0x2400
#define ERROR_SOURCE_OLEDB_BINDER       0x2500
#define ERROR_SOURCE_NOTESPH            0x2600
#define ERROR_SOURCE_EXSTOREPH          0x2700
#define ERROR_SOURCE_SRCH_SCHEMA_CACHE  0x3300
#define ERROR_SOURCE_CONTENT_SOURCE     0x3400
#define ERROR_SOURCE_REMOTE_EXSTOREPH   0x3500
#define ERROR_SOURCE_PEOPLE_IMPORT      0x4000
#define ERROR_FTE                       0x3600
#define ERROR_FTE_CB                    0xCB00
#define ERROR_FTE_FD                    0xFD00
//
// MessageId: XML_E_NODEFAULTNS
//
// MessageText:
//
// This scope has no default namespace.  Add a default namespace before continuing.
//
#define XML_E_NODEFAULTNS                0x80042200L

//uebug SXQL is jargon.  Use a more familiar term, if possible, and/or suggest an action that makes the error clear.
//
// MessageId: XML_E_BADSXQL
//
// MessageText:
//
// The SXQL is invalid.
//
#define XML_E_BADSXQL                    0x80042201L

//uebug what about it is invalid?  Provide guidelines for valid names.
//
// MessageId: MSS_E_INVALIDAPPNAME
//
// MessageText:
//
// The specified application name is not valid. Check to see whether a search application with that name already exists.
//
#define MSS_E_INVALIDAPPNAME             0x80042100L

//uebug Perhaps suggest verification that application was installed.
//
// MessageId: MSS_E_APPNOTFOUND
//
// MessageText:
//
// The specified search application name was not found. It may have already been deleted.
//
#define MSS_E_APPNOTFOUND                0x80042101L

//
// MessageId: MSS_E_APPALREADYEXISTS
//
// MessageText:
//
// The search application name already exists. Use a different name.
//
#define MSS_E_APPALREADYEXISTS           0x80042102L

//
// MessageId: MSS_E_CATALOGNOTFOUND
//
// MessageText:
//
// The specified catalog was not found. Check to see if it was deleted, or if there are errors in your application code.
//
#define MSS_E_CATALOGNOTFOUND            0x80042103L

//uebug The phrase "it is stopping" is unclear.  What sort of action is "stopping?  Use clearer terminology.
//
// MessageId: MSS_E_CATALOGSTOPPING
//
// MessageText:
//
// The content index cannot be deleted while the search service is stopped. Restart the search service and try again.
//
#define MSS_E_CATALOGSTOPPING            0x80042104L

//
// MessageId: MSS_E_UNICODEFILEHEADERMISSING
//
// MessageText:
//
// This file is shorter than 2 bytes. Unicode text file must begin with a wide character that indicates byte order.
//
#define MSS_E_UNICODEFILEHEADERMISSING   0x80042105L

//
// MessageId: MSS_E_CATALOGALREADYEXISTS
//
// MessageText:
//
// The specified catalog already exists.
//
#define MSS_E_CATALOGALREADYEXISTS       0x80042106L

//
// MessageId: NET_E_GENERAL
//
// MessageText:
//
// A network read or write operation has failed.
//
#define NET_E_GENERAL                    0x80040300L

//
// MessageId: NET_E_DISCONNECTED
//
// MessageText:
//
// The network connection was lost. Try the query again.
//
#define NET_E_DISCONNECTED               0x80040303L

//
// MessageId: NET_E_INVALIDPARAMS
//
// MessageText:
//
// The parameters passed were not valid.
//
#define NET_E_INVALIDPARAMS              0x80040308L

//
// MessageId: NET_E_OPERATIONINPROGRESS
//
// MessageText:
//
// Another operation is already in progress on this socket. Try the query again.
//
#define NET_E_OPERATIONINPROGRESS        0x80040309L

//
// MessageId: SEC_E_INVALIDCONTEXT
//
// MessageText:
//
// The search service is running as a local System service. To access Exchange documents, check that the search service is running in a user account with administrative privileges on the Exchange server.
//
#define SEC_E_INVALIDCONTEXT             0x80041403L

//
// MessageId: SEC_E_INITFAILED
//
// MessageText:
//
// The Exchange search provider cannot be initialized. You may have to reinstall the  application.
//
#define SEC_E_INITFAILED                 0x80041401L

//
// MessageId: SEC_E_NOTINITIALIZED
//
// MessageText:
//
// The security provider was not initialized. You may have to reinstall the application.
//
#define SEC_E_NOTINITIALIZED             0x80041402L

//
// MessageId: SEC_E_ACCESSDENIED
//
// MessageText:
//
// Access is denied. You may not have sufficient privileges to perform the operation.
//
#define SEC_E_ACCESSDENIED               0x800414FFL

//uebug Return to what?
//
// MessageId: DS_E_NOMOREDATA
//
// MessageText:
//
// There is no more data to return. 
//
#define DS_E_NOMOREDATA                  0x80040400L

//uebug What about it is invalid?
//
// MessageId: DS_E_INVALIDDATASOURCE
//
// MessageText:
//
// The requested content index name is not valid. Use a valid content index name.
//
#define DS_E_INVALIDDATASOURCE           0x80040401L

//
// MessageId: DS_E_DATASOURCENOTAVAILABLE
//
// MessageText:
//
// The requested content index is disabled on the search server. Contact the system administrator to enable the content index. Check the event log for possible errors.
//
#define DS_E_DATASOURCENOTAVAILABLE      0x80040402L

//
// MessageId: DS_E_QUERYCANCELED
//
// MessageText:
//
// The query was cancelled at the user's request.
//
#define DS_E_QUERYCANCELED               0x80040403L

//
// MessageId: DS_E_UNKNOWNREQUEST
//
// MessageText:
//
// The request code is not valid.
//
#define DS_E_UNKNOWNREQUEST              0x80040404L

//
// MessageId: DS_E_BADREQUEST
//
// MessageText:
//
// The request data is not valid for the given request type.
//
#define DS_E_BADREQUEST                  0x80040405L

//
// MessageId: DS_E_SERVERCAPACITY
//
// MessageText:
//
// The request cannot be processed, because the system is low on resources. Try again later, or contact your system administrator to free up system resources.
//
#define DS_E_SERVERCAPACITY              0x80040406L

//this error code is currently not in use
//
// MessageId: DS_E_BADSEQUENCE
//
// MessageText:
//
// The request or function call is out of sequence. Check for programming errors.
//
#define DS_E_BADSEQUENCE                 0x80040407L

//uebug Provide maximum text length in suggested action.
//this error code is currently not in use
//
// MessageId: DS_E_MESSAGETOOLONG
//
// MessageText:
//
// The query text is too long. 
//
#define DS_E_MESSAGETOOLONG              0x80040408L

//this error code is currently not in use
//
// MessageId: DS_E_SERVERERROR
//
// MessageText:
//
// The request cannot be processed because of an error in the search server.
//
#define DS_E_SERVERERROR                 0x80040409L

//this error code is currently not in use
//
// MessageId: DS_E_CONFIGBAD
//
// MessageText:
//
// The configuration file for the content index is not valid. 
//
#define DS_E_CONFIGBAD                   0x8004040AL

//
// MessageId: DS_E_DATANOTPRESENT
//
// MessageText:
//
// The search server cannot find one or more catalog files. Check the event log for related failures.
//
#define DS_E_DATANOTPRESENT              0x80040410L

//
// MessageId: DS_E_SETSTATUSINPROGRESS
//
// MessageText:
//
// The SetStatus operation cannot be completed, because another SetStatus operation is already in progress. Try again after the current operation is completed.
//
#define DS_E_SETSTATUSINPROGRESS         0x80040411L

//
// MessageId: DS_E_DUPLICATEID
//
// MessageText:
//
// The content index cannot be created, because another content index with the same name already exists. Create the content index using a different name.
//
#define DS_E_DUPLICATEID                 0x80040412L

//uebug suggest: "Increase the number of possible catalogs, or remove another catalog."
//
// MessageId: DS_E_TOOMANYDATASOURCES
//
// MessageText:
//
// The content index cannot be created, because the limit on the number of indexes was reached. 
//
#define DS_E_TOOMANYDATASOURCES          0x80040413L

//
// MessageId: DS_E_REGISTRY
//
// MessageText:
//
// The search server cannot read or write to the registry. Check that the search server is running with administrative privileges.
//
#define DS_E_REGISTRY                    0x80040414L

//
// MessageId: DS_E_DATASOURCENOTDISABLED
//
// MessageText:
//
// The content index cannot be removed in the current state. Disable and remove the content index.
//
#define DS_E_DATASOURCENOTDISABLED       0x80040415L

//
// MessageId: DS_E_INVALIDTAGDB
//
// MessageText:
//
// The tag database is not valid. Contact your system administrator.
//
#define DS_E_INVALIDTAGDB                0x80040416L

//
// MessageId: DS_E_INVALIDCATALOGNAME
//
// MessageText:
//
// The content index name is in the wrong format. Use a name without spaces or punctuation.
//
#define DS_E_INVALIDCATALOGNAME          0x80040417L

//this is not in use
//
// MessageId: DS_E_CONFIGNOTRIGHTTYPE
//
// MessageText:
//
// The configuration file is not the correct type.
//
#define DS_E_CONFIGNOTRIGHTTYPE          0x80040418L

//
// MessageId: DS_E_PROTOCOLVERSION
//
// MessageText:
//
// There is a mismatch in the protocol between the search service and the client. Install the correct version of the client.
//
#define DS_E_PROTOCOLVERSION             0x80040419L

//
// MessageId: DS_E_ALREADYENABLED
//
// MessageText:
//
// The index is already enabled. 
//
#define DS_E_ALREADYENABLED              0x8004041AL

//
// MessageId: DS_E_INDEXDIRECTORY
//
// MessageText:
//
// The content index directory structure cannot be created. See the event log for related errors.
//
#define DS_E_INDEXDIRECTORY              0x8004041CL

//uebug The value of what?  Point out the maximum length in the suggested action.
//this is not in use.
//
// MessageId: DS_E_VALUETOOLARGE
//
// MessageText:
//
// The value exceeds the maximum length.
//
#define DS_E_VALUETOOLARGE               0x8004041DL

//
// MessageId: DS_E_UNKNOWNPARAM
//
// MessageText:
//
// The requested parameter is unknown.
//
#define DS_E_UNKNOWNPARAM                0x8004041EL

//uebug It's unclear what operation the buffer is too small for, or whether that buffer can be changed.
//
// MessageId: DS_E_BUFFERTOOSMALL
//
// MessageText:
//
// The buffer is too small. 
//
#define DS_E_BUFFERTOOSMALL              0x8004041FL

//uebug mention valid range in suggested action
//
// MessageId: DS_E_PARAMOUTOFRANGE
//
// MessageText:
//
// The parameter value is out of range. 
//
#define DS_E_PARAMOUTOFRANGE             0x80040420L

//
// MessageId: DS_E_ALREADYDISABLED
//
// MessageText:
//
// The content index is already disabled.
//
#define DS_E_ALREADYDISABLED             0x80040421L

//
// MessageId: DS_E_QUERYHUNG
//
// MessageText:
//
// The operation cannot be completed, because one or more queries failed to terminate in a timely manner. 
//
#define DS_E_QUERYHUNG                   0x80040422L

//uebug What result? How/why is it invalid? Be specific.
//
// MessageId: DS_E_BADRESULT
//
// MessageText:
//
// The result is invalid. 
//
#define DS_E_BADRESULT                   0x80040423L

//
// MessageId: DS_E_CANNOTWRITEREGISTRY
//
// MessageText:
//
// The registry data cannot be written. Check that the search service is running with administrator privileges.
//
#define DS_E_CANNOTWRITEREGISTRY         0x80040424L

//
// MessageId: DS_E_CANNOTREMOVECONCURRENT
//
// MessageText:
//
//  DS_E_CANNOTREMOVECONCURRENT
//
#define DS_E_CANNOTREMOVECONCURRENT      0x80040425L

//
// MessageId: DS_E_SEARCHCATNAMECOLLISION
//
// MessageText:
//
// The content index cannot be created because an index with the same name already exists on the search server. Create the content index using a different name.
//
#define DS_E_SEARCHCATNAMECOLLISION      0x80040426L

//
// MessageId: DS_E_PROPVERSIONMISMATCH
//
// MessageText:
//
// The content index cannot be enabled because its propagation version cannot be verified against the dedicated index server.  This can occur when either the dedicated index server is offline or the content index is out-of-date.  
//
#define DS_E_PROPVERSIONMISMATCH         0x80040427L

//
// MessageId: DS_E_MISSINGCATALOG
//
// MessageText:
//
// .  
//
#define DS_E_MISSINGCATALOG              0x80040428L

//
// MessageId: COLL_E_BADSEQUENCE
//
// MessageText:
//
// The requested operation is not valid at this time. 
//
#define COLL_E_BADSEQUENCE               0x80040501L

//uebug Return to what?
//
// MessageId: COLL_E_NOMOREDATA
//
// MessageText:
//
// There is no more data to return. 
//
#define COLL_E_NOMOREDATA                0x80040502L

//
// MessageId: COLL_E_INCOMPATIBLECOLUMNS
//
// MessageText:
//
// The query cannot be collated, because columns using the same name have different types. 
//
#define COLL_E_INCOMPATIBLECOLUMNS       0x80040503L

//
// MessageId: COLL_E_BUFFERTOOSMALL
//
// MessageText:
//
// Not enough buffer space is available to collate search results. 
//
#define COLL_E_BUFFERTOOSMALL            0x80040504L

//
// MessageId: COLL_E_BADRESULT
//
// MessageText:
//
// The result row is corrupted. 
//
#define COLL_E_BADRESULT                 0x80040506L

//
// MessageId: COLL_E_NOSORTCOLUMN
//
// MessageText:
//
// The search server did not return a column required for collation. 
//
#define COLL_E_NOSORTCOLUMN              0x80040507L

//
// MessageId: COLL_E_DUPLICATEDBID
//
// MessageText:
//
// The database ID is a duplicate. 
//
#define COLL_E_DUPLICATEDBID             0x80040508L

//
// MessageId: COLL_E_TOOMANYMERGECOLUMNS
//
// MessageText:
//
// The data source merge exceeded the number of result columns. 
//
#define COLL_E_TOOMANYMERGECOLUMNS       0x80040509L

//
// MessageId: COLL_E_NODEFAULTCATALOG
//
// MessageText:
//
// No default index was set. 
//
#define COLL_E_NODEFAULTCATALOG          0x8004050AL

//uebug Provide maximum number in suggested action.
//
// MessageId: COLL_E_MAXCONNEXCEEDED
//
// MessageText:
//
// The maximum number of open provider connections was exceeded. 
//
#define COLL_E_MAXCONNEXCEEDED           0x8004050BL

//
// MessageId: CM_E_TOOMANYDATASERVERS
//
// MessageText:
//
// The limit on the number of search servers was reached. 
//
#define CM_E_TOOMANYDATASERVERS          0x80040601L

//
// MessageId: CM_E_TOOMANYDATASOURCES
//
// MessageText:
//
// The limit on the number of indexes was reached. 
//
#define CM_E_TOOMANYDATASOURCES          0x80040602L

//
// MessageId: CM_E_NOQUERYCONNECTIONS
//
// MessageText:
//
// No query connections to the server can be established. 
//
#define CM_E_NOQUERYCONNECTIONS          0x80040603L

//
// MessageId: CM_E_DATASOURCENOTAVAILABLE
//
// MessageText:
//
// The requested content index is disabled. 
//
#define CM_E_DATASOURCENOTAVAILABLE      0x80040604L

//uebug "The network" may be the wrong subject for this message.
//
// MessageId: CM_E_CONNECTIONTIMEOUT
//
// MessageText:
//
// The network timed out trying to acquire a connection to a search server. 
//
#define CM_E_CONNECTIONTIMEOUT           0x80040605L

//
// MessageId: CM_E_SERVERNOTFOUND
//
// MessageText:
//
// The specified server cannot be found. 
//
#define CM_E_SERVERNOTFOUND              0x80040606L

//
// MessageId: CM_S_NODATASERVERS
//
// MessageText:
//
// No connection to a Search server can be established. 
//
#define CM_S_NODATASERVERS               0x00040607L

//
// MessageId: CM_E_REGISTRY
//
// MessageText:
//
// The Windows NT registry cannot be accessed. 
//
#define CM_E_REGISTRY                    0x80040608L

//
// MessageId: CM_E_INVALIDDATASOURCE
//
// MessageText:
//
// The requested content index is empty or has not been added to the search server. 
//
#define CM_E_INVALIDDATASOURCE           0x80040609L

//
// MessageId: CM_E_TIMEOUT
//
// MessageText:
//
// The query timed out. 
//
#define CM_E_TIMEOUT                     0x8004060AL

//
// MessageId: CM_E_INSUFFICIENTBUFFER
//
// MessageText:
//
// The buffer area passed to the function is not large enough. 
//
#define CM_E_INSUFFICIENTBUFFER          0x8004060BL

//uebug Include guidelines for proper syntax in suggested action.
//
// MessageId: QRY_E_QUERYSYNTAX
//
// MessageText:
//
// The query syntax is not valid. 
//
#define QRY_E_QUERYSYNTAX                0x80040701L

//
// MessageId: QRY_E_TYPEMISMATCH
//
// MessageText:
//
// The column type in the search query does not match the column type in the index. 
//
#define QRY_E_TYPEMISMATCH               0x80040702L

//uebug Who/what specified the query type?  Be specific.
//
// MessageId: QRY_E_UNHANDLEDTYPE
//
// MessageText:
//
// The query type specified is unknown. 
//
#define QRY_E_UNHANDLEDTYPE              0x80040703L

//
// MessageId: QRY_S_NOROWSFOUND
//
// MessageText:
//
// No matching items were found for this query. 
//
#define QRY_S_NOROWSFOUND                0x00040704L

//
// MessageId: QRY_E_TOOMANYCOLUMNS
//
// MessageText:
//
// The number of columns requested exceeds the limit. 
//
#define QRY_E_TOOMANYCOLUMNS             0x80040705L

//uebug Provide max. # of IDs in suggested action.
//
// MessageId: QRY_E_TOOMANYDATABASES
//
// MessageText:
//
// The query contained too many content index IDs. 
//
#define QRY_E_TOOMANYDATABASES           0x80040706L

//uebug "query start hit" might not be clear to users.  Is there a clearer way of saying this?
//
// MessageId: QRY_E_STARTHITTOBIG
//
// MessageText:
//
// The query start hit is larger than the total number of results. 
//
#define QRY_E_STARTHITTOBIG              0x80040707L

//
// MessageId: QRY_E_TOOMANYQUERYTERMS
//
// MessageText:
//
// The query contains too many terms. 
//
#define QRY_E_TOOMANYQUERYTERMS          0x80040708L

//
// MessageId: QRY_E_NODATASOURCES
//
// MessageText:
//
// No indexes were specified. 
//
#define QRY_E_NODATASOURCES              0x80040709L

//
// MessageId: QRY_E_TIMEOUT
//
// MessageText:
//
// The query operation timed out. 
//
#define QRY_E_TIMEOUT                    0x8004070AL

//
// MessageId: QRY_E_COLUMNNOTSORTABLE
//
// MessageText:
//
// The query specified sorting a non-sortable column. 
//
#define QRY_E_COLUMNNOTSORTABLE          0x8004070BL

//
// MessageId: QRY_E_COLUMNNOTSEARCHABLE
//
// MessageText:
//
// The query specified searching a non-searchable column. 
//
#define QRY_E_COLUMNNOTSEARCHABLE        0x8004070CL

//
// MessageId: QRY_E_INVALIDCOLUMN
//
// MessageText:
//
// The query specified a nonexistent column. 
//
#define QRY_E_INVALIDCOLUMN              0x8004070DL

//uebug Provide example of valid data in suggested action.
//
// MessageId: QRY_E_QUERYCORRUPT
//
// MessageText:
//
// The query data is not valid or is inconsistent. 
//
#define QRY_E_QUERYCORRUPT               0x8004070EL

//
// MessageId: QRY_E_PREFIXWILDCARD
//
// MessageText:
//
// A query term begins with a wildcard character. 
//
#define QRY_E_PREFIXWILDCARD             0x8004070FL

//
// MessageId: QRY_E_INFIXWILDCARD
//
// MessageText:
//
// A query term contains an invalid embedded wildcard character. 
//
#define QRY_E_INFIXWILDCARD              0x80040710L

//
// MessageId: QRY_E_WILDCARDPREFIXLENGTH
//
// MessageText:
//
// A wildcard term in the query does not contain a sufficiently long prefix. 
//
#define QRY_E_WILDCARDPREFIXLENGTH       0x80040711L

//
// MessageId: QRY_S_TERMIGNORED
//
// MessageText:
//
// The query term was successfully ignored.
//
#define QRY_S_TERMIGNORED                0x00040712L

//
// MessageId: QRY_E_ENGINEFAILED
//
// MessageText:
//
// The search server is unable to process the query. 
//
#define QRY_E_ENGINEFAILED               0x80040713L

//uebug Give options for narrowing the criteria.
//
// MessageId: QRY_E_SEARCHTOOBIG
//
// MessageText:
//
// The search criteria are too general or matched too many terms. 
//
#define QRY_E_SEARCHTOOBIG               0x80040714L

//uebug Give examples of meaningful restrictions.
//
// MessageId: QRY_E_NULLQUERY
//
// MessageText:
//
// The query does not contain any meaningful restrictions. 
//
#define QRY_E_NULLQUERY                  0x80040715L

//
// MessageId: QRY_S_INEXACTRESULTS
//
// MessageText:
//
// The query found some matching items, but is unable to compile the entire result set. 
//
#define QRY_S_INEXACTRESULTS             0x00040716L

//
// MessageId: QRY_E_NOCOLUMNS
//
// MessageText:
//
// The query did not specify any return columns. 
//
#define QRY_E_NOCOLUMNS                  0x80040717L

//uebug Provide examples of proper formatting in suggested action.
//
// MessageId: QRY_E_INVALIDSCOPES
//
// MessageText:
//
// The scopes specified for the query were incorrectly formatted. 
//
#define QRY_E_INVALIDSCOPES              0x80040718L

//uebug Provide examples of proper formatting in suggested action.
//
// MessageId: QRY_E_INVALIDCATALOG
//
// MessageText:
//
// The indexes specified for the query were incorrectly formatted. 
//
#define QRY_E_INVALIDCATALOG             0x80040719L

//uebug Provide valid range of scopes and/or depths.
//
// MessageId: QRY_E_SCOPECARDINALIDY
//
// MessageText:
//
// The number of scopes or depths was not valid. 
//
#define QRY_E_SCOPECARDINALIDY           0x8004071AL

//uebug What is meant by "the current context"?  Be specific.  How is it invalid?  Provide a valid operation for the context.
//
// MessageId: QRY_E_UNEXPECTED
//
// MessageText:
//
// The operation is invalid in the current context. 
//
#define QRY_E_UNEXPECTED                 0x8004071BL

//
// MessageId: QRY_E_INVALIDPATH
//
// MessageText:
//
// An invalid path was supplied. 
//
#define QRY_E_INVALIDPATH                0x8004071CL

//
// MessageId: QRY_E_LMNOTINITIALIZED
//
// MessageText:
//
// The log manager is not initialized. 
//
#define QRY_E_LMNOTINITIALIZED           0x8004071DL

//
// MessageId: QRY_E_INVALIDINTERVAL
//
// MessageText:
//
// An invalid interval was passed to the log manager. 
//
#define QRY_E_INVALIDINTERVAL            0x8004071EL

//
// MessageId: QRY_E_NOLOGMANAGER
//
// MessageText:
//
// No log manager was attached to the logger. 
//
#define QRY_E_NOLOGMANAGER               0x8004071FL

//
// MessageId: SCHEMA_E_LOAD_SPECIAL
//
// MessageText:
//
// The configuration file attempted to load a reserved column. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define SCHEMA_E_LOAD_SPECIAL            0x80040C01L

//
// MessageId: SCHEMA_E_FILENOTFOUND
//
// MessageText:
//
// The configuration file cannot be found. Reinstall the application.
//
#define SCHEMA_E_FILENOTFOUND            0x80040C02L

//
// MessageId: SCHEMA_E_NESTEDTAG
//
// MessageText:
//
// The tag nesting configuration for the application is not valid.  Reinstall the application.
//
#define SCHEMA_E_NESTEDTAG               0x80040C03L

//
// MessageId: SCHEMA_E_UNEXPECTEDTAG
//
// MessageText:
//
// The configuration file for the application contains an invalid tag.  Reinstall the application.
//
#define SCHEMA_E_UNEXPECTEDTAG           0x80040C04L

//
// MessageId: SCHEMA_E_VERSIONMISMATCH
//
// MessageText:
//
// The configuration file version number for the application is invalid.  Reinstall the application.
//
#define SCHEMA_E_VERSIONMISMATCH         0x80040C05L

//
// MessageId: SCHEMA_E_CANNOTCREATEFILE
//
// MessageText:
//
// The output configuration file cannot be created. Stop and restart the system, and then reinstall the application.
//
#define SCHEMA_E_CANNOTCREATEFILE        0x80040C06L

//
// MessageId: SCHEMA_E_CANNOTWRITEFILE
//
// MessageText:
//
// Data cannot be written to the output configuration file. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define SCHEMA_E_CANNOTWRITEFILE         0x80040C07L

//
// MessageId: SCHEMA_E_EMPTYFILE
//
// MessageText:
//
// The configuration file for the application contains invalid information. Reinstall the application.
//
#define SCHEMA_E_EMPTYFILE               0x80040C08L

//
// MessageId: SCHEMA_E_INVALIDFILETYPE
//
// MessageText:
//
// The configuration file type for the application is not recognized. Reinstall the application.
//
#define SCHEMA_E_INVALIDFILETYPE         0x80040C09L

//
// MessageId: SCHEMA_E_INVALIDDATATYPE
//
// MessageText:
//
// The configuration file column data type for the application is not recognized. Reinstall the application.
//
#define SCHEMA_E_INVALIDDATATYPE         0x80040C0AL

//
// MessageId: SCHEMA_E_CANNOTCREATENOISEWORDFILE
//
// MessageText:
//
// The noise word file cannot be created. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define SCHEMA_E_CANNOTCREATENOISEWORDFILE 0x80040C0BL

//
// MessageId: SCHEMA_E_ADDSTOPWORDS
//
// MessageText:
//
// There was an error writing to the temporary noise word file. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define SCHEMA_E_ADDSTOPWORDS            0x80040C0CL

//
// MessageId: SCHEMA_E_NAMEEXISTS
//
// MessageText:
//
// The specified column name already exists. The application's configuration was corrupted.  Reinstall the application.
//
#define SCHEMA_E_NAMEEXISTS              0x80040C0DL

//
// MessageId: SCHEMA_E_INVALIDVALUE
//
// MessageText:
//
// The value for the attribute is invalid. The application's configuration was corrupted.  Reinstall the application.
//
#define SCHEMA_E_INVALIDVALUE            0x80040C0EL

//
// MessageId: SCHEMA_E_BADPROPSPEC
//
// MessageText:
//
// The property set GUID is invalid. The application's configuration was corrupted.  Reinstall the application.
//
#define SCHEMA_E_BADPROPSPEC             0x80040C0FL

//
// MessageId: SCHEMA_E_NOMORECOLUMNS
//
// MessageText:
//
// No more columns.  This is an internal error code and should not be reported to users.  Call Microsoft Product Support.
//
#define SCHEMA_E_NOMORECOLUMNS           0x80040C10L

//
// MessageId: SCHEMA_E_FILECHANGED
//
// MessageText:
//
// The system cannot write to the configuration file, because it was modified since it was last read. Stop and restart the search service.  If this problem persists, reinstall search.
//
#define SCHEMA_E_FILECHANGED             0x80040C11L

//
// MessageId: SCHEMA_E_BADCOLUMNNAME
//
// MessageText:
//
// The specified column name is invalid. The application's configuration has been corrupted.  Reinstall the application.
//
#define SCHEMA_E_BADCOLUMNNAME           0x80040C12L

//
// MessageId: SCHEMA_E_BADPROPPID
//
// MessageText:
//
// The property ID specified is invalid. The application's configuration has been corrupted.  Reinstall the application.
//
#define SCHEMA_E_BADPROPPID              0x80040C13L

//
// MessageId: SCHEMA_E_BADATTRIBUTE
//
// MessageText:
//
// An invalid attribute was specified for this tag, or a required attribute is missing. The application's configuration was corrupted.  Reinstall the application.
//
#define SCHEMA_E_BADATTRIBUTE            0x80040C14L

//
// MessageId: SCHEMA_E_BADFILENAME
//
// MessageText:
//
// The specified file name is invalid. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define SCHEMA_E_BADFILENAME             0x80040C15L

//
// MessageId: SCHEMA_E_PROPEXISTS
//
// MessageText:
//
// A column having the specified property set and PID already exists. The application's configuration has been corrupted.  Reinstall the application.
//
#define SCHEMA_E_PROPEXISTS              0x80040C16L

//
// MessageId: SCHEMA_E_DUPLICATENOISE
//
// MessageText:
//
// The configuration describing a language and sublanguage contains unexpected duplicate information. The application's configuration may be corrupted.  Though basic functionality is unlikely to be impacted, it is recommended that you reinstall the application.
//
#define SCHEMA_E_DUPLICATENOISE          0x80040C17L

//
// MessageId: GTHR_E_DUPLICATE_OBJECT
//
// MessageText:
//
// The object you are trying to create already exists. Try again using a different name.
//
#define GTHR_E_DUPLICATE_OBJECT          0x80040D02L

//
// MessageId: GTHR_E_UNABLE_TO_READ_REGISTRY
//
// MessageText:
//
// The registry value cannot be read because the configuration is invalid. Recreate the content index configuration by removing the content index.
//
#define GTHR_E_UNABLE_TO_READ_REGISTRY   0x80040D03L

//
// MessageId: GTHR_E_ERROR_WRITING_REGISTRY
//
// MessageText:
//
// The value cannot be set, because the object was already deleted or was not initialized properly. Check that the object reference is still valid, increase the registry size, or recreate the content index configuration.
//
#define GTHR_E_ERROR_WRITING_REGISTRY    0x80040D04L

//
// MessageId: GTHR_E_ERROR_INITIALIZING_PERFMON
//
// MessageText:
//
// Performance monitoring cannot be initialized. This affects performance counters only. Try restarting the service or restarting the server.
//
#define GTHR_E_ERROR_INITIALIZING_PERFMON 0x80040D05L

//
// MessageId: GTHR_E_ERROR_OBJECT_NOT_FOUND
//
// MessageText:
//
// The specified object cannot be found. Specify the name of an existing object.
//
#define GTHR_E_ERROR_OBJECT_NOT_FOUND    0x80040D06L

//
// MessageId: GTHR_E_URL_EXCLUDED
//
// MessageText:
//
// The specified address was excluded from the index. The site path rules may have to be modified to include this address.
//
#define GTHR_E_URL_EXCLUDED              0x80040D07L

//
// MessageId: GTHR_E_CONFIG_DUP_PROJECT
//
// MessageText:
//
// A duplicate index entry exists in the registry. No other indexes are affected, but it is recommended that the duplicate entry be deleted. 
//
#define GTHR_E_CONFIG_DUP_PROJECT        0x80040D0AL

//
// MessageId: GTHR_E_CONFIG_DUP_EXTENSION
//
// MessageText:
//
// A duplicate extension entry exists in the registry. The duplicate extension should be deleted, but all other extensions are unaffected.
//
#define GTHR_E_CONFIG_DUP_EXTENSION      0x80040D0BL

//
// MessageId: GTHR_E_DUPLICATE_URL
//
// MessageText:
//
// The URL was already processed during this update. If you received this message while processing alerts, then the alerts are redundant, or else Modify should be used instead of Add.
//
#define GTHR_E_DUPLICATE_URL             0x80040D0DL

//
// MessageId: GTHR_E_TOO_MANY_PLUGINS
//
// MessageText:
//
// The gatherer attempted to create more gatherer plug-in objects than are allowed. Remove another plug-in before adding this one.
//
#define GTHR_E_TOO_MANY_PLUGINS          0x80040D0EL

//
// MessageId: GTHR_E_INVALIDFUNCTION
//
// MessageText:
//
// The function is not implemented in this context, and cannot be called.  This error is internal, and cannot normally occur. Reinstall the application.
//
#define GTHR_E_INVALIDFUNCTION           0x80040D0FL

//
// MessageId: GTHR_E_NOFILTERSINK
//
// MessageText:
//
// The plug-in did not provide a filter sink object. You may have installed a custom plug-in that is not supported. 
//
#define GTHR_E_NOFILTERSINK              0x80040D10L

//
// MessageId: GTHR_E_FILTER_PROCESS_TERMINATED
//
// MessageText:
//
// The filtering process ended and is now unavailable. The address will be retried, but it is possible that the system is low in resources. Free some system resources, or restart the update when resources are available.
//
#define GTHR_E_FILTER_PROCESS_TERMINATED 0x80040D11L

//
// MessageId: GTHR_E_FILTER_INVALID_MESSAGE
//
// MessageText:
//
// An unexpected message was received from the filtering process. The address will be retried, but if this occurs frequently, your system is low on resources. Free some system resources, or restart the update when resources are available.
//
#define GTHR_E_FILTER_INVALID_MESSAGE    0x80040D12L

//
// MessageId: GTHR_E_UNSUPPORTED_PROPERTY_TYPE
//
// MessageText:
//
// The filtering process returned an unsupported property type. Fix the filter to allow successful use of the document.
//
#define GTHR_E_UNSUPPORTED_PROPERTY_TYPE 0x80040D13L

//
// MessageId: GTHR_E_NAME_TOO_LONG
//
// MessageText:
//
// The specified name for this object exceeds the maximum length, which is usually 2047 characters. Use a shorter name for the object.
//
#define GTHR_E_NAME_TOO_LONG             0x80040D14L

//
// MessageId: GTHR_E_NO_IDENTITY
//
// MessageText:
//
// The user agent or e-mail address was not specified. Specify the e-mail address to use in the protocol headers.
//
#define GTHR_E_NO_IDENTITY               0x80040D15L

//
// MessageId: GTHR_E_FILTER_NOT_FOUND
//
// MessageText:
//
// A filter for the document cannot be created. Install the corresponding filter for this document format.
//
#define GTHR_E_FILTER_NOT_FOUND          0x80040D16L

//
// MessageId: GTHR_E_FILTER_NO_MORE_THREADS
//
// MessageText:
//
// All threads in the filtering process are currently in use. 
//
#define GTHR_E_FILTER_NO_MORE_THREADS    0x80040D17L

//
// MessageId: GTHR_E_PRT_HNDLR_PROGID_MISSING
//
// MessageText:
//
// The ProgID for a protocol handler cannot be obtained, and the protocol handler registration is invalid. Reregister the protocol handlers.
//
#define GTHR_E_PRT_HNDLR_PROGID_MISSING  0x80040D18L

//
// MessageId: GTHR_E_FILTER_PROCESS_TERMINATED_QUOTA
//
// MessageText:
//
// The filtering process was stopped because its memory quota was exceeded.
//
#define GTHR_E_FILTER_PROCESS_TERMINATED_QUOTA 0x80040D19L

//
// MessageId: GTHR_E_UNKNOWN_PROTOCOL
//
// MessageText:
//
// The protocol handler cannot be found. Check that the handler has been installed.
//
#define GTHR_E_UNKNOWN_PROTOCOL          0x80040D1AL

//
// MessageId: GTHR_E_PROJECT_NOT_INITIALIZED
//
// MessageText:
//
// The gatherer index was not initialized. The content index must be remounted. If the index is still not initialized, remove it.
//
#define GTHR_E_PROJECT_NOT_INITIALIZED   0x80040D1BL

//
// MessageId: GTHR_S_STATUS_CHANGE_IGNORED
//
// MessageText:
//
// The status change request was ignored, because the same status change is already pending. 
//
#define GTHR_S_STATUS_CHANGE_IGNORED     0x00040D1CL

//
// MessageId: GTHR_S_STATUS_END_CRAWL
//
// MessageText:
//
// The update ended.
//
#define GTHR_S_STATUS_END_CRAWL          0x00040D1DL

//this is only used to display a localized name for word "reset"
//
// MessageId: GTHR_S_STATUS_RESET
//
// MessageText:
//
// Reset
//
#define GTHR_S_STATUS_RESET              0x00040D1EL

//
// MessageId: GTHR_S_STATUS_THROTTLE
//
// MessageText:
//
// Preparing to propagate
//
#define GTHR_S_STATUS_THROTTLE           0x00040D1FL

//
// MessageId: GTHR_S_STATUS_RESUME
//
// MessageText:
//
// Resume
//
#define GTHR_S_STATUS_RESUME             0x00040D20L

//
// MessageId: GTHR_S_STATUS_PAUSE
//
// MessageText:
//
// Pause
//
#define GTHR_S_STATUS_PAUSE              0x00040D21L

//
// MessageId: GTHR_E_INVALID_PROJECT_NAME
//
// MessageText:
//
// The catalog name is invalid. Choose a catalog name that is shorter or does not contain special characters.
//
#define GTHR_E_INVALID_PROJECT_NAME      0x80040D22L

//
// MessageId: GTHR_E_SHUTTING_DOWN
//
// MessageText:
//
// The gatherer is shutting down.
//
#define GTHR_E_SHUTTING_DOWN             0x80040D23L

//
// MessageId: GTHR_S_END_STD_CHUNKS
//
// MessageText:
//
// The gatherer has finished filtering standard properties.
//
#define GTHR_S_END_STD_CHUNKS            0x00040D24L

//
// MessageId: GTHR_E_VALUE_NOT_AVAILABLE
//
// MessageText:
//
// The requested value was not set by the filter process. This is a recoverable internal error. The document may still succeed.
//
#define GTHR_E_VALUE_NOT_AVAILABLE       0x80040D25L

//
// MessageId: GTHR_E_OUT_OF_DOC_ID
//
// MessageText:
//
// All valid document IDs were used. Recreate the content index.
//
#define GTHR_E_OUT_OF_DOC_ID             0x80040D26L

//
// MessageId: GTHR_E_NOTIFICATION_START_PAGE
//
// MessageText:
//
// This content source cannot be crawled, because its is set up to accept alerts.
//
#define GTHR_E_NOTIFICATION_START_PAGE   0x80040D27L

//
// MessageId: GTHR_E_DUP_PROPERTY_MAPPING
//
// MessageText:
//
// The property mapping is duplicated in the schema. Remove the duplicate property schema.
//
#define GTHR_E_DUP_PROPERTY_MAPPING      0x80040D2AL

//
// MessageId: GTHR_S_NO_CRAWL_SEEDS
//
// MessageText:
//
// The update cannot begin without content sources. Add at least one content source that is not used for alerts.
//
#define GTHR_S_NO_CRAWL_SEEDS            0x00040D2BL

//
// MessageId: GTHR_E_INVALID_ACCOUNT
//
// MessageText:
//
// The specified account information is incorrect or invalid.  Check that the correct account and password are being used.
//
#define GTHR_E_INVALID_ACCOUNT           0x80040D2CL

//
// MessageId: GTHR_E_FILTER_INIT
//
// MessageText:
//
// The document cannot be filtered, because the document and filter versions do not appear to match. Install a matching document filter.
//
#define GTHR_E_FILTER_INIT               0x80040D2EL

//
// MessageId: GTHR_E_INVALID_ACCOUNT_SYNTAX
//
// MessageText:
//
// The specified account name cannot be used because it contains invalid characters, such as '\'.  Use a valid account name.
//
#define GTHR_E_INVALID_ACCOUNT_SYNTAX    0x80040D2FL

//
// MessageId: GTHR_S_CANNOT_FILTER
//
// MessageText:
//
// Failed to load document into IFilter. Possibly caused by unrecognized document format or document corruption. Only metadata obtained from the document storage was indexed.
//
#define GTHR_S_CANNOT_FILTER             0x00040D30L

//
// MessageId: GTHR_E_PROXY_NAME
//
// MessageText:
//
// The proxy server name is missing. Specify a proxy server, and try again.
//
#define GTHR_E_PROXY_NAME                0x80040D31L

//
// MessageId: GTHR_E_SERVER_UNAVAILABLE
//
// MessageText:
//
// The server is unavailable and cannot be accessed. The server is probably  disconnected from the network. Access attempts will be delayed for ten minutes.
//
#define GTHR_E_SERVER_UNAVAILABLE        0x80040D32L

//
// MessageId: GTHR_S_STATUS_STOP
//
// MessageText:
//
// Stop
//
#define GTHR_S_STATUS_STOP               0x00040D33L

//
// MessageId: GTHR_E_INVALID_PATH
//
// MessageText:
//
// This path does not correspond to the selected site. Use a path name that matches the site, or use a wildcard (*).
//
#define GTHR_E_INVALID_PATH              0x80040D34L

//
// MessageId: GTHR_E_FILTER_NO_CODEPAGE
//
// MessageText:
//
// The document cannot be filtered, because the required code page is not installed.  Install the code page for this document.
//
#define GTHR_E_FILTER_NO_CODEPAGE        0x80040D35L

//
// MessageId: GTHR_S_STATUS_START
//
// MessageText:
//
// Start Update
//
#define GTHR_S_STATUS_START              0x00040D36L

//
// MessageId: GTHR_E_NO_PRTCLHNLR
//
// MessageText:
//
// No protocol handler is available. Install a protocol handler that can process this URL type.
//
#define GTHR_E_NO_PRTCLHNLR              0x80040D37L

//
// MessageId: GTHR_E_IE_OFFLINE
//
// MessageText:
//
// Internet Explorer is set to offline mode. Change Internet Explorer to online mode, and continue.
//
#define GTHR_E_IE_OFFLINE                0x80040D38L

//
// MessageId: GTHR_E_BAD_FILTER_DAEMON
//
// MessageText:
//
// The filter process cannot be started. The system is most likely low on resources, or the filter process binary was modified. If the resources are available, check the search binaries with an antivirus program.
//
#define GTHR_E_BAD_FILTER_DAEMON         0x80040D39L

//
// MessageId: GTHR_E_INVALID_MAPPING
//
// MessageText:
//
// The specified URL mapping is invalid. The "from"" mapping is empty, or the ""from"" mapping is the same as the ""to"" mapping. Specify a valid ""from"" mapping that differs from the ""to"" mapping.
//
#define GTHR_E_INVALID_MAPPING           0x80040D40L

//
// MessageId: GTHR_E_USER_AGENT_NOT_SPECIFIED
//
// MessageText:
//
// The user agent string was not specified or contains invalid characters. Specify a user agent which is not empty and does not contain double quotation marks.
//
#define GTHR_E_USER_AGENT_NOT_SPECIFIED  0x80040D41L

//
// MessageId: GTHR_E_FROM_NOT_SPECIFIED
//
// MessageText:
//
// The e-mail address is missing.  Type an e-mail address.
//
#define GTHR_E_FROM_NOT_SPECIFIED        0x80040D43L

//
// MessageId: GTHR_E_INVALID_STREAM_LOGS_COUNT
//
// MessageText:
//
// The specified number of logs to keep is invalid. Set the number of logs to be greater than zero.
//
#define GTHR_E_INVALID_STREAM_LOGS_COUNT 0x80040D44L

//
// MessageId: GTHR_E_INVALID_EXTENSION
//
// MessageText:
//
// The extension string is not specified or is invalid. Specify an extension string that does not contain the following characters: [./?*:\#] or spaces.
//
#define GTHR_E_INVALID_EXTENSION         0x80040D45L

//
// MessageId: GTHR_E_INVALID_GROW_FACTOR
//
// MessageText:
//
// The specified maximum grow factor is invalid. Set the maximum grow factor to be greater than or equal to zero.
//
#define GTHR_E_INVALID_GROW_FACTOR       0x80040D46L

//
// MessageId: GTHR_E_INVALID_TIME_OUT
//
// MessageText:
//
// The specified time out value is invalid. Set the time out value to be greater than or equal to zero.
//
#define GTHR_E_INVALID_TIME_OUT          0x80040D47L

//
// MessageId: GTHR_E_INVALID_RETRIES
//
// MessageText:
//
// The specified retry limit value is invalid. Set the retry limit value to be greater than or equal to zero.
//
#define GTHR_E_INVALID_RETRIES           0x80040D48L

//
// MessageId: GTHR_E_INVALID_LOG_FILE_NAME
//
// MessageText:
//
// The gatherer log file name is not specified. Specify a log file name.
//
#define GTHR_E_INVALID_LOG_FILE_NAME     0x80040D49L

//
// MessageId: GTHR_E_INVALID_HOST_NAME
//
// MessageText:
//
// The site name is not specified or is invalid. Specify a site name that does not contain the following characters: [/\\@#|] or spaces.
//
#define GTHR_E_INVALID_HOST_NAME         0x80040D50L

//
// MessageId: GTHR_E_INVALID_START_PAGE
//
// MessageText:
//
// The content source is not specified or is invalid. Type a host name that does not contain * or spaces.
//
#define GTHR_E_INVALID_START_PAGE        0x80040D51L

//
// MessageId: GTHR_E_DUPLICATE_PROJECT
//
// MessageText:
//
// A catalog with the name specified already exists. Create the catalog using a different name.
//
#define GTHR_E_DUPLICATE_PROJECT         0x80040D52L

//
// MessageId: GTHR_E_INVALID_DIRECTORY
//
// MessageText:
//
// The path is not specified or contains invalid characters, such as ["" ?*]. Specify a path that does not contain these characters.
//
#define GTHR_E_INVALID_DIRECTORY         0x80040D53L

//
// MessageId: GTHR_E_FILTER_INTERRUPTED
//
// MessageText:
//
// The filtering was stopped because of a user action, such as stopping the crawl. 
//
#define GTHR_E_FILTER_INTERRUPTED        0x80040D54L

//
// MessageId: GTHR_E_INVALID_PROXY_PORT
//
// MessageText:
//
// The specified port for the HTTP proxy is invalid. Specify a port between 0 and 0xffff
//
#define GTHR_E_INVALID_PROXY_PORT        0x80040D55L

//
// MessageId: GTHR_S_CONFIG_HAS_ACCOUNTS
//
// MessageText:
//
// The index configuration contains account information.
//
#define GTHR_S_CONFIG_HAS_ACCOUNTS       0x00040D56L

//
// MessageId: GTHR_E_SECRET_NOT_FOUND
//
// MessageText:
//
// The account password was not specified. Specify the password.
//
#define GTHR_E_SECRET_NOT_FOUND          0x80040D57L

//
// MessageId: GTHR_E_INVALID_PATH_EXPRESSION
//
// MessageText:
//
// The path expression cannot contain the reserved escape character '|'. Use an expression without reserved characters.
// |' - a reserved escape character. Use an expression without reserved characters.
//
#define GTHR_E_INVALID_PATH_EXPRESSION   0x80040D58L

//
// MessageId: GTHR_E_INVALID_START_PAGE_HOST
//
// MessageText:
//
// The specification for the content source is missing the host name. Specify a valid URL.
//
#define GTHR_E_INVALID_START_PAGE_HOST   0x80040D59L

//
// MessageId: GTHR_E_INVALID_START_PAGE_PATH
//
// MessageText:
//
// The path specification for the content source cannot contain wildcard characters, such as [?*]. Remove all wildcards from the path specification.
//
#define GTHR_E_INVALID_START_PAGE_PATH   0x80040D60L

//
// MessageId: GTHR_E_APPLICATION_NOT_FOUND
//
// MessageText:
//
// The specified gatherer application cannot be found. Use the name of an existing application, or reinstall the application.
//
#define GTHR_E_APPLICATION_NOT_FOUND     0x80040D61L

//
// MessageId: GTHR_E_CANNOT_REMOVE_PLUGINMGR
//
// MessageText:
//
// The application plug-in manager cannot be removed, because one or more content indexes was configured to use the plug-in.  Remove all indexes, and then remove the plug-in manager.
//
#define GTHR_E_CANNOT_REMOVE_PLUGINMGR   0x80040D62L

//
// MessageId: GTHR_E_INVALID_APPLICATION_NAME
//
// MessageText:
//
// The application name is invalid, because it contains special characters. Specify a different application name without special characters.
//
#define GTHR_E_INVALID_APPLICATION_NAME  0x80040D63L

//
// MessageId: GTHR_E_FILTER_FAULT
//
// MessageText:
//
// The data size returned by the filter is greater than the allocated buffer. Download and install an update for the filter.
//
#define GTHR_E_FILTER_FAULT              0x80040D65L

//
// MessageId: GTHR_E_NON_FIXED_DRIVE
//
// MessageText:
//
// The object cannot be created on a non-fixed drive. Create the object on another drive.
//
#define GTHR_E_NON_FIXED_DRIVE           0x80040D66L

//
// MessageId: GTHR_S_PROB_NOT_MODIFIED
//
// MessageText:
//
// The content was not likely to be modified during the adaptive update, so the gatherer is not going to check the document for changes.
//
#define GTHR_S_PROB_NOT_MODIFIED         0x00040D67L

//
// MessageId: GTHR_S_CRAWL_SCHEDULED
//
// MessageText:
//
// The update will start as soon as all content sources are released by other updates that are already in progress.
//
#define GTHR_S_CRAWL_SCHEDULED           0x00040D68L

//
// MessageId: GTHR_S_TRANSACTION_IGNORED
//
// MessageText:
//
// The transaction was ignored as redundant, because another transaction for the same URL was active at the same time.
//
#define GTHR_S_TRANSACTION_IGNORED       0x00040D69L

//
// MessageId: GTHR_S_START_FILTER_FROM_PROTOCOL
//
// MessageText:
//
// The gatherer started getting properties using the filter from the protocol handler. 
//
#define GTHR_S_START_FILTER_FROM_PROTOCOL 0x00040D6AL

//
// MessageId: GTHR_E_FILTER_SINGLE_THREADED
//
// MessageText:
//
// The system attempted to load a filter marked as apartment in a multi-threaded filter daemon. The document will be retried in a single-threaded filter. Since multithreaded filtering is more efficient, try to obtain the version of the filter that is multi-threaded.
//
#define GTHR_E_FILTER_SINGLE_THREADED    0x80040D6BL

//
// MessageId: GTHR_S_BAD_FILE_LINK
//
// MessageText:
//
// This file link cannot be followed. 
//
#define GTHR_S_BAD_FILE_LINK             0x00040D6CL

//
// MessageId: GTHR_E_URL_UNIDENTIFIED
//
// MessageText:
//
// The identified content was not crawled for this address. Recreate the content index.
//
#define GTHR_E_URL_UNIDENTIFIED          0x80040D6DL

 // NewLocStr 6/22/00
//
// MessageId: GTHR_S_NOT_ALL_PARTS
//
// MessageText:
//
// Some parts of this document cannot be accessed.
//
#define GTHR_S_NOT_ALL_PARTS             0x00040D6EL

//
// MessageId: GTHR_E_FORCE_NOTIFICATION_RESET
//
// MessageText:
//
// The gatherer cannot handle the alert stream. All alert sources will be crawled again.
//
#define GTHR_E_FORCE_NOTIFICATION_RESET  0x80040D6FL

//
// MessageId: GTHR_S_END_PROCESS_LOOP_NOTIFY_QUEUE
//
// MessageText:
//
// The process notify queue was stopped successfully. 
//
#define GTHR_S_END_PROCESS_LOOP_NOTIFY_QUEUE 0x00040D70L

//
// MessageId: GTHR_S_START_FILTER_FROM_BODY
//
// MessageText:
//
// The gatherer started getting properties using the document filter.
//
#define GTHR_S_START_FILTER_FROM_BODY    0x00040D71L

//
// MessageId: GTHR_E_CONTENT_ID_CONFLICT
//
// MessageText:
//
// A duplicate of this document was detected, and the document will be retried. If this problem persists, recreate the content index.
//
#define GTHR_E_CONTENT_ID_CONFLICT       0x80040D72L

//
// MessageId: GTHR_E_UNABLE_TO_READ_EXCHANGE_STORE
//
// MessageText:
//
// The administrative data from the Exchange store or registry cannot be read. Check that the Exchange store is started, and that it was properly restored.
//
#define GTHR_E_UNABLE_TO_READ_EXCHANGE_STORE 0x80040D73L

//
// MessageId: GTHR_E_RECOVERABLE_EXOLEDB_ERROR
//
// MessageText:
//
// Read access to the Exchange store is temporarily unavailable. If this message persists, restart the Exchange store.
//
#define GTHR_E_RECOVERABLE_EXOLEDB_ERROR 0x80040D74L

//
// MessageId: GTHR_E_INVALID_CALL_FROM_WBREAKER
//
// MessageText:
//
// The function is unavailable, because it was called while the word breaker was attempting to fill the text buffer. Download and install an update for the word breaker.
//
#define GTHR_E_INVALID_CALL_FROM_WBREAKER 0x80040D76L

//
// MessageId: GTHR_E_PROPERTY_LIST_NOT_INITIALIZED
//
// MessageText:
//
// The property list for the content class of this document cannot be loaded, and the schema cannot be accessed. Check that the schema is correctly configured.
//
#define GTHR_E_PROPERTY_LIST_NOT_INITIALIZED 0x80040D77L

//
// MessageId: GTHR_S_MODIFIED_PARTS
//
// MessageText:
//
// Some parts of the document were modified, and others were not. The gatherer will refilter the entire document.
//
#define GTHR_S_MODIFIED_PARTS            0x00040D78L

//
// MessageId: GHTR_E_LOCAL_SERVER_UNAVAILABLE
//
// MessageText:
//
// Crawling cannot continue because the local server is not responding. Restart Microsoft SQL Server 2000.
//
#define GHTR_E_LOCAL_SERVER_UNAVAILABLE  0x80040D79L

//
// MessageId: GTHR_E_SCHEMA_ERRORS_OCCURRED
//
// MessageText:
//
// Schema/schema cache error (%1) occurred. Schema collection ref: %2, content class: %3. This error is internal, for debugging.
//
#define GTHR_E_SCHEMA_ERRORS_OCCURRED    0x80040D7AL

//
// MessageId: GTHR_E_TIMEOUT
//
// MessageText:
//
// Document filtering cannot be completed because the document server did not respond within the specified timeout. Try crawling the server later, or increase the timeout values.
//
#define GTHR_E_TIMEOUT                   0x80040D7BL

 // NewLocStr 11/30/99
//
// MessageId: GTHR_S_CRAWL_FULL
//
// MessageText:
//
// Full
//
#define GTHR_S_CRAWL_FULL                0x00040D83L

 // NewLocStr 11/30/99
//
// MessageId: GTHR_S_CRAWL_INCREMENTAL
//
// MessageText:
//
// Incremental
//
#define GTHR_S_CRAWL_INCREMENTAL         0x00040D84L

 // NewLocStr 11/30/99
//
// MessageId: GTHR_S_CRAWL_ADAPTIVE
//
// MessageText:
//
// Adaptive
//
#define GTHR_S_CRAWL_ADAPTIVE            0x00040D85L

 // NewLocStr 1/2/00
//
// MessageId: GTHR_E_NOTIFICATION_START_ADDRESS_INVALID
//
// MessageText:
//
// The content source for this alert must refer to a valid file system directory
//
#define GTHR_E_NOTIFICATION_START_ADDRESS_INVALID 0x80040D86L

 // NewLocStr 2/9/00
//
// MessageId: GTHR_E_NOTIFICATION_TYPE_NOT_SUPPORTED
//
// MessageText:
//
// Alerts for this content source are not supported.
//
#define GTHR_E_NOTIFICATION_TYPE_NOT_SUPPORTED 0x80040D87L

 // NewLocStr 2/9/00
//
// MessageId: GTHR_E_NOTIFICATION_FILE_SHARE_INFO_NOT_AVAILABLE
//
// MessageText:
//
// Alerts for this content source cannot be established because information about the file share cannot be obtained.
//
#define GTHR_E_NOTIFICATION_FILE_SHARE_INFO_NOT_AVAILABLE 0x80040D88L

 // NewLocStr 2/9/00
//
// MessageId: GTHR_E_NOTIFICATION_LOCAL_PATH_MUST_USE_FIXED_DRIVE
//
// MessageText:
//
// Alerts for a content source with a local address are only supported for a fixed media drive type. Use the UNC name for remote file shares.
//
#define GTHR_E_NOTIFICATION_LOCAL_PATH_MUST_USE_FIXED_DRIVE 0x80040D89L

 // NewLocStr 2/22/00
//
// MessageId: GHTR_E_INSUFFICIENT_DISK_SPACE
//
// MessageText:
//
// The new content index cannot be added because there is not enough disk space.  Free up at least 10 megabytes of free space, and then try again.
//
#define GHTR_E_INSUFFICIENT_DISK_SPACE   0x80040D8BL

 // NewLocStr 2/29/00
//
// MessageId: GTHR_E_INVALID_RESOURCE_ID
//
// MessageText:
//
// The document received an invalid document resource ID.  The document must be crawled again.
//
#define GTHR_E_INVALID_RESOURCE_ID       0x80040D8DL

 // NewLocStr 3/15/00
//
// MessageId: GTHR_E_NESTED_HIERARCHICAL_START_ADDRESSES
//
// MessageText:
//
// Hierarchical content sources such as those in the file system cannot be nested.
//
#define GTHR_E_NESTED_HIERARCHICAL_START_ADDRESSES 0x80040D8EL

 // NewLocStr 5/25/00
//
// MessageId: GTHR_S_NO_INDEX
//
// MessageText:
//
// Content for this URL is excluded by the server because a no-index attribute.
//
#define GTHR_S_NO_INDEX                  0x00040D90L

 // NewLocStr 6/12/00
//
// MessageId: GTHR_S_PAUSE_REASON_EXTERNAL
//
// MessageText:
//
// by user
//
#define GTHR_S_PAUSE_REASON_EXTERNAL     0x00040D92L

 // NewLocStr 6/22/00
//
// MessageId: GTHR_S_PAUSE_REASON_UPGRADING
//
// MessageText:
//
// for upgrade
//
#define GTHR_S_PAUSE_REASON_UPGRADING    0x00040D93L

 // NewLocStr 6/13/00
//
// MessageId: GTHR_S_PAUSE_REASON_BACKOFF
//
// MessageText:
//
// low resources
//
#define GTHR_S_PAUSE_REASON_BACKOFF      0x00040D94L

 // NewLocStr 6/22/00
//
// MessageId: GTHR_E_RETRY
//
// MessageText:
//
// The first attempt to crawl this object failed. Another attempt is being made.
//
#define GTHR_E_RETRY                     0x80040D95L

 // NewLocStr 6/28/00
//
// MessageId: GTHR_E_JET_BACKUP_ERROR
//
// MessageText:
//
// Backup and restore failed during an ESE database phase. For more detailed information, check the event log.
//
#define GTHR_E_JET_BACKUP_ERROR          0x80040D96L

 // NewLocStr 6/28/00
//
// MessageId: GTHR_E_JET_RESTORE_ERROR
//
// MessageText:
//
// Backup and restore failed during an ESE database phase. For more detailed information, check the event log.
//
#define GTHR_E_JET_RESTORE_ERROR         0x80040D97L

 // NewLocStr 7/17/00
//
// MessageId: GTHR_S_OFFICE_CHILD
//
// MessageText:
//
// This document is a child of another document.  It will not be cataloged separately.
//
#define GTHR_S_OFFICE_CHILD              0x00040D9AL

 // NewLocStr 7/31/00
//
// MessageId: GTHR_E_PLUGIN_NOT_REGISTERED
//
// MessageText:
//
// The plug-in is not properly registered on this computer.  Check that the application is properly installed, or contact Microsoft Product Support.
//
#define GTHR_E_PLUGIN_NOT_REGISTERED     0x80040D9BL

 // NewLocStr 7/31/00
//
// MessageId: GTHR_E_NOTIF_ACCESS_TOKEN_UPDATED
//
// MessageText:
//
// Alerts for this scope were interrupted so that authentication tokens can be updated. 
//
#define GTHR_E_NOTIF_ACCESS_TOKEN_UPDATED 0x80040D9CL

 // NewLocStr 7/31/00
//
// MessageId: GTHR_E_DIRMON_NOT_INITIALZED
//
// MessageText:
//
// File system alerts were not initialized successfully   If the problem persists after restarting the service, try rerunning Setup.
//
#define GTHR_E_DIRMON_NOT_INITIALZED     0x80040D9DL

 // NewLocStr 7/31/00
//
// MessageId: GTHR_E_NOTIF_BEING_REMOVED
//
// MessageText:
//
// File system alerts for this scope are stopped because the scope is being removed.
//
#define GTHR_E_NOTIF_BEING_REMOVED       0x80040D9EL

 // NewLocStr 7/31/00
//
// MessageId: GTHR_E_NOTIF_EXCESSIVE_THROUGHPUT
//
// MessageText:
//
// Alerts failed because of network failures or because the internal cache size was exceeded.  If the problem persists, change the content source type to adaptive or crawl.
//
#define GTHR_E_NOTIF_EXCESSIVE_THROUGHPUT 0x80040D9FL

 // NewLocStr 8/7/00
//
// MessageId: GTHR_E_INVALID_PATH_SPEC
//
// MessageText:
//
// The operation could not be completed because the specified path is invalid.  Try again using a valid path.
//
#define GTHR_E_INVALID_PATH_SPEC         0x80040DA0L

 // NewLocStr 8/21/00
//
// MessageId: GTHR_E_INSUFFICIENT_FEATURE_TERMS
//
// MessageText:
//
// There were not enough keywords in the sample documents to train the Topic Assistant.  Try again when more sample documents are available.
//
#define GTHR_E_INSUFFICIENT_FEATURE_TERMS 0x80040DA1L

 // NewLocStr 8/21/00
//
// MessageId: GTHR_E_INSUFFICIENT_EXAMPLE_CATEGORIES
//
// MessageText:
//
// The Topic Assistant requires sample documents from at least two categories.  Try again when sample documents for more categories are available.
//
#define GTHR_E_INSUFFICIENT_EXAMPLE_CATEGORIES 0x80040DA2L

 // NewLocStr 8/21/00
//
// MessageId: GTHR_E_INSUFFICIENT_EXAMPLE_DOCUMENTS
//
// MessageText:
//
// There were not enough sample documents to train the Topic Assistant.  Try again when more sample documents are available.
//
#define GTHR_E_INSUFFICIENT_EXAMPLE_DOCUMENTS 0x80040DA3L

 // NewLocStr 8/21/00
//
// MessageId: GTHR_E_AUTOCAT_UNEXPECTED
//
// MessageText:
//
// An error occurred using the topic assistant.  Check the event log for related failures.
//
#define GTHR_E_AUTOCAT_UNEXPECTED        0x80040DA4L

 // NewLocStr 8/15/00
//
// MessageId: GTHR_E_SINGLE_THREADED_EMBEDDING
//
// MessageText:
//
// The system attempted to load a filter marked as apartment from an embedded component in a multi-threaded filter daemon. The document will be retried in a single-threaded filter. Since multithreaded filtering is more efficient, try to obtain the version of the filter that is multi-threaded.
//
#define GTHR_E_SINGLE_THREADED_EMBEDDING 0x80040DA5L

 // NewLocStr 10/2/00
//
// MessageId: GTHR_S_CANNOT_WORDBREAK
//
// MessageText:
//
// The document contains text that can not be broken into words. The document won't be indexed.
//
#define GTHR_S_CANNOT_WORDBREAK          0x00040DA6L

  // No need to localize this
//
// MessageId: GTHR_S_USE_MIME_FILTER
//
// MessageText:
//
// This is a success code that indicates that the MIME filter should be used to filter the document stream. This is an internal error code and should not be reported to users.  Call Microsoft Product Support.
//
#define GTHR_S_USE_MIME_FILTER           0x00040DA7L

 // NewLocStr 10/22/00
//
// MessageId: GTHR_E_FOLDER_CRAWLED_BY_ANOTHER_WORKSPACE
//
// MessageText:
//
// The data for this folder will not be crawled because it is configured to be crawled by (%1).
//
#define GTHR_E_FOLDER_CRAWLED_BY_ANOTHER_WORKSPACE 0x80040DA9L

 // NewLocStr 10/26/00
//
// MessageId: GTHR_E_EMPTY_DACL
//
// MessageText:
//
// The security descriptor for this document only contained built-in access control entries. The built-in ACEs have to be removed because they won't be valid for this document during queries.
//
#define GTHR_E_EMPTY_DACL                0x80040DAAL

 // NewLocStr 11/08/00
//
// MessageId: GTHR_E_OBJECT_NOT_VALID
//
// MessageText:
//
// The context for this object is no longer valid.  Refresh the context for this object.
//
#define GTHR_E_OBJECT_NOT_VALID          0x80040DABL

 // NewLocStr 6/6/01
//
// MessageId: GTHR_E_CANNOT_ENABLE_CHECKPOINT
//
// MessageText:
//
// The checkpoint cannot be enabled, because the content index is not idle.  Wait until the index is idle, and then try again.
//
#define GTHR_E_CANNOT_ENABLE_CHECKPOINT  0x80040DAEL

//
// MessageId: GTHR_E_SCOPES_EXCEEDED
//
// MessageText:
//
// The maximum number of search scopes was reached. Use one of the existing scopes.
//
#define GTHR_E_SCOPES_EXCEEDED           0x80040DAFL

//
// MessageId: GTHR_E_PROPERTIES_EXCEEDED
//
// MessageText:
//
// Maximum number of properties per document has been reached. The document is most likely corrupt, or it represents a denial of service threat, the indexing of this document has been aborted.
//
#define GTHR_E_PROPERTIES_EXCEEDED       0x80040DB0L

//
// MessageId: GTHR_E_INVALID_START_ADDRESS
//
// MessageText:
//
// The start address is invalid because it is a local path.  Please enter a valid start address.
//
#define GTHR_E_INVALID_START_ADDRESS     0x80040DB2L

 // NewLocStr 6/13/00
//
// MessageId: GTHR_S_PAUSE_REASON_PROFILE_IMPORT
//
// MessageText:
//
// for profile import
//
#define GTHR_S_PAUSE_REASON_PROFILE_IMPORT 0x00040DB3L

 // used to replace ERROR_PIPE_NOT_CONNECTED
//
// MessageId: GTHR_E_PIPE_NOT_CONNECTTED
//
// MessageText:
//
// The filtering process has been terminated
//
#define GTHR_E_PIPE_NOT_CONNECTTED       0x80040DB4L

//
// MessageId: GTHR_E_BACKUP_VALIDATION_FAIL
//
// MessageText:
//
// The backup image cannot be verified because the image may be corrupted. To preserve your data, it is recommended that you run Data Backup and Restore again.
//
#define GTHR_E_BACKUP_VALIDATION_FAIL    0x80040DB6L

//
// MessageId: GTHR_E_BAD_FILTER_HOST
//
// MessageText:
//
// The filter host process cannot be started. The system is most likely low on resources, or the filter host process binary was modified. If the resources are available, check the search binaries with an antivirus program.
//
#define GTHR_E_BAD_FILTER_HOST           0x80040DB7L

//
// MessageId: GTHR_E_NTF_CLIENT_NOT_SUBSCRIBED
//
// MessageText:
//
// Notification client is not subscribed.
//
#define GTHR_E_NTF_CLIENT_NOT_SUBSCRIBED 0xC0040DB9L

//
// MessageId: GTHR_E_FILTERPOOL_NOTFOUND
//
// MessageText:
//
// The per-user filter pool could not be found.
//
#define GTHR_E_FILTERPOOL_NOTFOUND       0x80040DBAL

//
// MessageId: GTHR_E_ADDLINKS_FAILED_WILL_RETRY_PARENT
//
// MessageText:
//
// Some of the links were not successfully added to the queue. Parent transaction will be retried.
//
#define GTHR_E_ADDLINKS_FAILED_WILL_RETRY_PARENT 0x80040DBBL

//
// MessageId: IDX_E_INVALIDTAG
//
// MessageText:
//
// The tag is invalid. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define IDX_E_INVALIDTAG                 0x80041101L

//
// MessageId: IDX_E_METAFILE_CORRUPT
//
// MessageText:
//
// The propagation configuration file cannot be read.  Check that the same versions of search are used on both the content index and search servers.  If the problem persists, recrawl and repropagate the content index. 
//
#define IDX_E_METAFILE_CORRUPT           0x80041102L

//
// MessageId: IDX_E_TOO_MANY_SEARCH_SERVERS
//
// MessageText:
//
// The number of search servers exceeded the limit.  Ensure that the number of propagated content indexes is less than the documented maximum.  If so, start and restart the search service, and then try again.
//
#define IDX_E_TOO_MANY_SEARCH_SERVERS    0x80041103L

//
// MessageId: IDX_E_SEARCH_SERVER_ALREADY_EXISTS
//
// MessageText:
//
// The specified search server already exists. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define IDX_E_SEARCH_SERVER_ALREADY_EXISTS 0x80041104L

//
// MessageId: IDX_E_BUILD_IN_PROGRESS
//
// MessageText:
//
// An update is already in progress. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define IDX_E_BUILD_IN_PROGRESS          0x80041105L

//
// MessageId: IDX_E_IDXLSTFILE_CORRUPT
//
// MessageText:
//
// The index list file cannot be read. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define IDX_E_IDXLSTFILE_CORRUPT         0x80041106L

//
// MessageId: IDX_E_REGISTRY_ENTRY
//
// MessageText:
//
// A registry entry required for content index creation cannot be read or created. Delete and recreate the content index.  If the problem persists, you may need to reinstall search.
//
#define IDX_E_REGISTRY_ENTRY             0x80041107L

//
// MessageId: IDX_E_OBJECT_NOT_FOUND
//
// MessageText:
//
// One or more files required for content index creation were not found. Stop and restart the search service.  If the problem persists, delete and recreate the content index.
//
#define IDX_E_OBJECT_NOT_FOUND           0x80041108L

//
// MessageId: IDX_E_SEARCH_SERVER_NOT_FOUND
//
// MessageText:
//
// The search server was not found. Stop and restart the search service.  If propagation is being used, stop and restart the search service on the remote computers.
//
#define IDX_E_SEARCH_SERVER_NOT_FOUND    0x80041109L

//
// MessageId: IDX_E_WB_NOTFOUND
//
// MessageText:
//
// A word breaker was not found for the given language. Check your current language settings and ensure that search supports the current language.  If the problem persists, reinstall search.
//
#define IDX_E_WB_NOTFOUND                0x8004110AL

//
// MessageId: IDX_E_NOISELIST_NOTFOUND
//
// MessageText:
//
// A noise word list was not found for the given language. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define IDX_E_NOISELIST_NOTFOUND         0x8004110BL

//
// MessageId: IDX_E_STEMMER_NOTFOUND
//
// MessageText:
//
// A stemmer was not found for the given language. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define IDX_E_STEMMER_NOTFOUND           0x8004110CL

//
// MessageId: IDX_E_PROP_STOPPED
//
// MessageText:
//
// Index propagation was stopped.  This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define IDX_E_PROP_STOPPED               0x8004110DL

//
// MessageId: IDX_E_DISKFULL
//
// MessageText:
//
// The disk is full. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define IDX_E_DISKFULL                   0x8004110EL

//
// MessageId: IDX_E_INVALID_INDEX
//
// MessageText:
//
// The index data is invalid. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define IDX_E_INVALID_INDEX              0x8004110FL

//
// MessageId: IDX_E_CORRUPT_INDEX
//
// MessageText:
//
// The index data is corrupt. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define IDX_E_CORRUPT_INDEX              0x80041110L

//
// MessageId: IDX_E_PROPSTORE_INIT_FAILED
//
// MessageText:
//
// The property store cannot be initialized. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define IDX_E_PROPSTORE_INIT_FAILED      0x80041112L

//
// MessageId: IDX_E_PROP_STATE_CORRUPT
//
// MessageText:
//
// The propagation state information cannot be read. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define IDX_E_PROP_STATE_CORRUPT         0x80041113L

//
// MessageId: IDX_S_NO_BUILD_IN_PROGRESS
//
// MessageText:
//
// There is no update in progress.  This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define IDX_S_NO_BUILD_IN_PROGRESS       0x00041114L

//
// MessageId: IDX_S_SEARCH_SERVER_ALREADY_EXISTS
//
// MessageText:
//
// The Search server already exists.
//
#define IDX_S_SEARCH_SERVER_ALREADY_EXISTS 0x00041115L

//
// MessageId: IDX_S_SEARCH_SERVER_DOES_NOT_EXIST
//
// MessageText:
//
// The search server does not exist.  This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define IDX_S_SEARCH_SERVER_DOES_NOT_EXIST 0x00041116L

//
// MessageId: IDX_E_NOT_LOADED
//
// MessageText:
//
// The content index is not loaded. Stop and restart the search service.  If the problem persists, review other errors in the event log. If necessary, delete and recreate the content index.
//
#define IDX_E_NOT_LOADED                 0x80041117L

//
// MessageId: IDX_E_PROP_MAJOR_VERSION_MISMATCH
//
// MessageText:
//
// There is a mismatch in the major version number. Check that the same versions of search are used on both the content index and search servers.  If necessary, crawl the index again and repropagate the index. 
//
#define IDX_E_PROP_MAJOR_VERSION_MISMATCH 0x80041118L

//
// MessageId: IDX_E_PROP_MINOR_VERSION_MISMATCH
//
// MessageText:
//
// The index is a mismatch in the minor version number. This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define IDX_E_PROP_MINOR_VERSION_MISMATCH 0x80041119L

//
// MessageId: IDX_E_DSS_NOT_CONNECTED
//
// MessageText:
//
// A connection with the Search service on a remote machine could not be established. Check the connectivity with the remote machine.  If necessary, stop and restart the Search service on the remote machine.
//
#define IDX_E_DSS_NOT_CONNECTED          0x8004111AL

//
// MessageId: IDX_E_DOCUMENT_ABORTED
//
// MessageText:
//
// Crawling of documents was stopped. This is an internal error code and should not be reported to users.  Call Microsoft Product Support.
//
#define IDX_E_DOCUMENT_ABORTED           0x8004111BL

//
// MessageId: IDX_E_CATALOG_DISMOUNTED
//
// MessageText:
//
// The index was dismounted because it is being removed or the search service stopped.  If the problem persists, restart the search service.
//
#define IDX_E_CATALOG_DISMOUNTED         0x8004111CL

//
// MessageId: IDX_S_DSS_NOT_AVAILABLE
//
// MessageText:
//
// The search server was added successfully, but cannot be contacted.  Check that the search service is executing on the remote computer, and that you can connect to the remote computer.
//
#define IDX_S_DSS_NOT_AVAILABLE          0x0004111DL

//
// MessageId: IDX_E_USE_DEFAULT_CONTENTCLASS
//
// MessageText:
//
// No index level information exists for this index. Use application level tables for this index.
//
#define IDX_E_USE_DEFAULT_CONTENTCLASS   0x8004111FL

//
// MessageId: IDX_E_USE_APPGLOBAL_PROPTABLE
//
// MessageText:
//
// Information for this content class was not found. Use the default content class.  This error is obsolete and should no longer be reported.  Call Microsoft Product Support.
//
#define IDX_E_USE_APPGLOBAL_PROPTABLE    0x80041120L

// Jet property store error(s)
//
// MessageId: JPS_E_JET_ERR
//
// MessageText:
//
// The content index server cannot update or access information because of a database error.  Stop and restart the search service.  If the problem persists, reset and recrawl the content index.  In some cases it may be necessary to delete and recreate the content index.
//
#define JPS_E_JET_ERR                    0x8004117FL

//
// MessageId: JPS_S_DUPLICATE_DOC_DETECTED
//
// MessageText:
//
// The content index server detected a previously crawled document with the same content. This is an internal error code and should not be reported to users.  Call Microsoft Product Support.
//
#define JPS_S_DUPLICATE_DOC_DETECTED     0x00041180L

//
// MessageId: JPS_E_CATALOG_DECSRIPTION_MISSING
//
// MessageText:
//
// The content index server cannot find a description of the content index in its database. Search will automatically attempt to recreate the content index description.  If this problem persists, stop and restart the search service and, if necessary, delete  and recreate the content index.
//
#define JPS_E_CATALOG_DECSRIPTION_MISSING 0x80041181L

//
// MessageId: JPS_E_MISSING_INFORMATION
//
// MessageText:
//
// The content index server cannot find needed information in its database. Reset and recrawl the content index.
//
#define JPS_E_MISSING_INFORMATION        0x80041182L

//
// MessageId: JPS_E_INSUFFICIENT_VERSION_STORAGE
//
// MessageText:
//
// The content index server cannot update or access its database, because the version store has insufficient resources.  Increase the system resource usage setting for the search service.  If the problem persists, stop and restart the search service.
//
#define JPS_E_INSUFFICIENT_VERSION_STORAGE 0x80041183L

//
// MessageId: JPS_E_INSUFFICIENT_DATABASE_SESSIONS
//
// MessageText:
//
// The content index server cannot update or access its database because sessions are unavailable. Increase the system resource usage setting for the search service.  If the problem persists, stop and restart the search service.
//
#define JPS_E_INSUFFICIENT_DATABASE_SESSIONS 0x80041184L

//
// MessageId: JPS_E_INSUFFICIENT_DATABASE_RESOURCES
//
// MessageText:
//
// The content index server cannot update or access its database because insufficient system resources are available. Increase the system resource usage setting for the search service.  If the problem persists, stop and restart the search service.
//
#define JPS_E_INSUFFICIENT_DATABASE_RESOURCES 0x80041185L

//
// MessageId: JPS_E_SCHEMA_ERROR
//
// MessageText:
//
// The content index server cannot update an unrecognized or recently defined property attribute. Reset and recrawl the content index.
//
#define JPS_E_SCHEMA_ERROR               0x80041186L

//
// MessageId: JPS_E_PROPAGATION_FILE
//
// MessageText:
//
// The property store was unable to open, close, or delete a data file used in propagation.  Check that your index volume has enough disk space.
//
#define JPS_E_PROPAGATION_FILE           0x80041187L

//
// MessageId: JPS_E_PROPAGATION_CORRUPTION
//
// MessageText:
//
// The property store encountered propagation data files that cannot be used.  Try propagation again.
//
#define JPS_E_PROPAGATION_CORRUPTION     0x80041188L

//
// MessageId: JPS_E_PROPAGATION_VERSION_MISMATCH
//
// MessageText:
//
// There is a mismatch in the property store version numbers. Ensure that the same versions of search are used on both the content index and search servers.  If necessary, recrawl the content index and try to propagate the content index again.
//
#define JPS_E_PROPAGATION_VERSION_MISMATCH 0x80041189L

//
// MessageId: JPS_E_SHARING_VIOLATION
//
// MessageText:
//
// The content index server cannot update or access information because of a sharing violation.  Stop and restart the search service.  If the problem persists, reset and recrawl the content index.  In some cases it may be necessary to delete and recreate the content index.
//
#define JPS_E_SHARING_VIOLATION          0x8004118AL

// 0xA0-0xCF are exchange specific errors
//
// MessageId: EXCI_E_NO_CONFIG
//
// MessageText:
//
// The configuration properties for Microsoft Exchange 5.5 were not set. Specify these properties, and then try again.
//
#define EXCI_E_NO_CONFIG                 0x800411A0L

//
// MessageId: EXCI_E_INVALID_SERVER_CONFIG
//
// MessageText:
//
// The name of the Microsoft Exchange 5.5 server specified in the content source does not match the Exchange server specified in the Microsoft Exchange 5.5 configuration properties. Change the name of the Exchange server in the content source or configuration properties to match, and then try again.
//
#define EXCI_E_INVALID_SERVER_CONFIG     0x800411A1L

//
// MessageId: EXCI_E_ACCESS_DENIED
//
// MessageText:
//
// There is insufficient privilege to access the Microsoft Exchange 5.5 server. Check that the Microsoft Exchange 5.5 configuration is valid and that the search service is running in the context of a user having administrative privileges on the Exchange server configuration node.
//
#define EXCI_E_ACCESS_DENIED             0x800411A2L

//
// MessageId: EXCI_E_INVALID_EXCHANGE_SERVER
//
// MessageText:
//
// The Microsoft Exchange 5.5 server name specified in the address does not match the one specified in the Microsoft Exchange 5.5 configuration. 
//
#define EXCI_E_INVALID_EXCHANGE_SERVER   0x800411A3L

//
// MessageId: EXCI_E_BADCONFIG_OR_ACCESSDENIED
//
// MessageText:
//
// There is insufficient privilege to access the Microsoft Exchange 5.5 Server. Verify that the Microsoft Exchange 5.5 configuration is valid and that the Search Service is running in the context of a user having administrative privileges on the Exchange Server configuration node.
//
#define EXCI_E_BADCONFIG_OR_ACCESSDENIED 0x800411A4L

//
// MessageId: EXCI_E_WRONG_SERVER_OR_ACCT
//
// MessageText:
//
// The service cannot access the Exchange 5.5 server. The configured Microsoft Exchange 5.5 server name may be invalid, the Exchange 5.5 server may be temporarily unavailable, or the account used by the search service may not have administrative access to the Exchange 5.5 server. Check all of these properties, and then try again.
//
#define EXCI_E_WRONG_SERVER_OR_ACCT      0x800411A5L

//
// MessageId: EXCI_E_NOT_ADMIN_OR_WRONG_SITE
//
// MessageText:
//
// The service cannot access the Exchange 5.5 service. The account used by the search service may not be an Exchange administrator, or the site or organization settings on the search host may be invalid.
//
#define EXCI_E_NOT_ADMIN_OR_WRONG_SITE   0x800411A6L

//
// MessageId: EXCI_E_NO_MAPI
//
// MessageText:
//
// Outlook is not installed on this machine, or is missing the CDO or MSEMS components.  Please install and configure Outlook, then try again.
//
#define EXCI_E_NO_MAPI                   0x800411A7L

//
// MessageId: EXCI_E_INVALID_ACCOUNT_INFO
//
// MessageText:
//
// The account information specified for accessing the Exchange 5.5 server cannot be used. Verify the account and password information.
//
#define EXCI_E_INVALID_ACCOUNT_INFO      0x800411A8L

// The following PRTH errors are here because we don't want to expose them to Protocol
// handler developers. Those we can afford to expose are in prtherr.mc.
//
// MessageId: PRTH_E_INTERNAL_ERROR
//
// MessageText:
//
// An unrecognized HTTP status was received. Check that the address can be accessed using Internet Explorer.
//
#define PRTH_E_INTERNAL_ERROR            0x80041204L

//I beleive we don't expose setting the maximum download limit in the UI. But is it a good idea to put the limit in the error message?
//It is 16Mb unless the setting in the registry is changed.
//
// MessageId: PRTH_S_MAX_GROWTH
//
// MessageText:
//
// Filtering of this file ended because the file reached the maximum filter output limit. Check that the filter does not generate a large amount of data relative to the size of the document.
//
#define PRTH_S_MAX_GROWTH                0x00041209L

//
// MessageId: PRTH_E_WININET
//
// MessageText:
//
// The address cannot be accessed. Check that the address can be accessed from Internet Explorer.
//
#define PRTH_E_WININET                   0x8004120AL

//this is an internal error and should not be visible to the user.
//
// MessageId: PRTH_E_RETRY
//
// MessageText:
//
// The attempt to access the URL again failed.
//
#define PRTH_E_RETRY                     0x8004120BL

//
// MessageId: PRTH_S_MAX_DOWNLOAD
//
// MessageText:
//
// The file reached the maximum download limit. Check that the full text of the document can be meaningfully crawled.
//
#define PRTH_S_MAX_DOWNLOAD              0x0004120CL

//
// MessageId: PRTH_E_MIME_EXCLUDED
//
// MessageText:
//
// The URL was excluded because its content type (multipart/x-mixed-replace) is not supported.
//
#define PRTH_E_MIME_EXCLUDED             0x8004120DL

//
// MessageId: PRTH_E_CANT_TRANSFORM_EXTERNAL_ACL
//
// MessageText:
//
// Search was unable to convert the Access Control List of the crawled store into a Windows NT Access Control List. Check that local information is not used in the store.
//
#define PRTH_E_CANT_TRANSFORM_EXTERNAL_ACL 0x8004120EL

//
// MessageId: PRTH_E_CANT_TRANSFORM_DENIED_ACE
//
// MessageText:
//
// An object cannot be processed because search cannot convert a Notes Access-Denied entry in the Access Control List into a Windows NT identity. Check the Notes to Windows NT identity mapping.
//
#define PRTH_E_CANT_TRANSFORM_DENIED_ACE 0x8004120FL

//
// MessageId: PRTH_E_NO_PROPERTY
//
// MessageText:
//
// The item cannot be processed further because search failed to find one of its properties. Check that the item is valid in the store.
//
#define PRTH_E_NO_PROPERTY               0x80041213L

//
// MessageId: PRTH_S_USE_ROSEBUD
//
// MessageText:
//
// You must use Rosebud to access this address.
//
#define PRTH_S_USE_ROSEBUD               0x00041214L

//
// MessageId: PRTH_E_DATABASE_OPEN_ERROR
//
// MessageText:
//
// The database cannot be opened. Check that the database is functional.
//
#define PRTH_E_DATABASE_OPEN_ERROR       0x80041215L

 // NewLocStr 2/24/00
//
// MessageId: PRTH_E_OPLOCK_BROKEN
//
// MessageText:
//
// Filtering was interrupted because of a sharing conflict with another process. Crawl the file again when it is not in use.
//
#define PRTH_E_OPLOCK_BROKEN             0x80041216L

//
// MessageId: PRTH_E_LOAD_FAILED
//
// MessageText:
//
// The protocol handler cannot be loaded.
//
#define PRTH_E_LOAD_FAILED               0x80041217L

//
// MessageId: PRTH_E_INIT_FAILED
//
// MessageText:
//
// The protocol handler cannot be initialized.
//
#define PRTH_E_INIT_FAILED               0x80041218L

//
// MessageId: PRTH_E_VOLUME_MOUNT_POINT
//
// MessageText:
//
// The volume mount point is not supported.
//
#define PRTH_E_VOLUME_MOUNT_POINT        0x80041219L

//
// MessageId: PRTH_E_TRUNCATED
//
// MessageText:
//
// Some data was truncated. Check that the content can still be read.
//
#define PRTH_E_TRUNCATED                 0x8004121AL

//
// MessageId: GTHR_E_LOCAL_GROUPS_EXPANSION_INTERNAL_ERROR
//
// MessageText:
//
// An internal error occurred while expanding local groups.
//
#define GTHR_E_LOCAL_GROUPS_EXPANSION_INTERNAL_ERROR 0x8004121DL

//
// MessageId: PRTH_E_HTTPS_CERTIFICATE_ERROR
//
// MessageText:
//
// Error found in the Secure Sockets Layer (SSL) certificate sent by the server.
//
#define PRTH_E_HTTPS_CERTIFICATE_ERROR   0x80041223L

//
// MessageId: PRTH_E_HTTPS_REQUIRE_CERTIFICATE
//
// MessageText:
//
// Accessing this site requires a client certificate.  Specify a client certificate in the site path rules.
//
#define PRTH_E_HTTPS_REQUIRE_CERTIFICATE 0x80041224L

//
// MessageId: PRTH_S_TRY_IMPERSONATING
//
// MessageText:
//
// The item should be accessed while impersonating a user. Protocol handler is expected to implement IUrlAccessor3.
//
#define PRTH_S_TRY_IMPERSONATING         0x00041225L

//
// MessageId: PRTH_E_TRANSACTION_FAILED
//
// MessageText:
//
// Transaction failed. Requeue the item for transaction.
//
#define PRTH_E_TRANSACTION_FAILED        0x80041226L

//uebug "unexpected error is jargon we should avoid, if possible.
//
// MessageId: CMDLINE_E_UNEXPECTED
//
// MessageText:
//
// An unexpected error occurred.
//
#define CMDLINE_E_UNEXPECTED             0x80041501L

//
// MessageId: CMDLINE_E_PAREN
//
// MessageText:
//
// There is an unmatched parenthesis on the command line. 
//
#define CMDLINE_E_PAREN                  0x80041502L

//
// MessageId: CMDLINE_E_PARAM_SIZE
//
// MessageText:
//
// The parameter is too long. 
//
#define CMDLINE_E_PARAM_SIZE             0x80041503L

//uebug Is this an internal warning only?  If not, then why the reference to a function call?
//
// MessageId: CMDLINE_E_NOT_INIT
//
// MessageText:
//
// The object is not initialized. Call Init() first to initialize the object.
//
#define CMDLINE_E_NOT_INIT               0x80041504L

//
// MessageId: CMDLINE_E_ALREADY_INIT
//
// MessageText:
//
// The object is already initialized. 
//
#define CMDLINE_E_ALREADY_INIT           0x80041505L

//uebug Provide the limit to the parameters in the suggested action.
//
// MessageId: CMDLINE_E_NUM_PARAMS
//
// MessageText:
//
// There are too many parameters. 
//
#define CMDLINE_E_NUM_PARAMS             0x80041506L

//
// MessageId: NLADMIN_E_DUPLICATE_CATALOG
//
// MessageText:
//
// A catalog with the name specified already exists.  Choose another name to create a new catalog.
//
#define NLADMIN_E_DUPLICATE_CATALOG      0x80041901L

//
// MessageId: NLADMIN_S_NOT_ALL_BUILD_CATALOGS_INITIALIZED
//
// MessageText:
//
// At least one content index definition was not initialized.  Check the amount of free disk space, and verify that the registry configuration is correct.
//
#define NLADMIN_S_NOT_ALL_BUILD_CATALOGS_INITIALIZED 0x00041902L

//
// MessageId: NLADMIN_E_FAILED_TO_GIVE_ACCOUNT_PRIVILEGE
//
// MessageText:
//
// The account cannot be granted a privilege to "log on as service"" on the domain controller.  The domain security administrator needs to grant the default account this privilege.  All of the default accounts specified for the server need this account privilege.
//
#define NLADMIN_E_FAILED_TO_GIVE_ACCOUNT_PRIVILEGE 0x80041903L

//
// MessageId: NLADMIN_E_BUILD_CATALOG_NOT_INITIALIZED
//
// MessageText:
//
// The selected content index definition did not initialize and cannot be used.  Check server status, content index configuration, and user access to the server.
//
#define NLADMIN_E_BUILD_CATALOG_NOT_INITIALIZED 0x80041904L

//uebug Will users understand "chunk type"?
//
// MessageId: SCRIPTPI_E_CHUNK_NOT_TEXT
//
// MessageText:
//
// The chunk type was not text. 
//
#define SCRIPTPI_E_CHUNK_NOT_TEXT        0x80042000L

//uebug What are the implications of the property ID being another type?
//
// MessageId: SCRIPTPI_E_PID_NOT_NAME
//
// MessageText:
//
// The property ID type was not name. 
//
#define SCRIPTPI_E_PID_NOT_NAME          0x80042001L

//
// MessageId: SCRIPTPI_E_PID_NOT_NUMERIC
//
// MessageText:
//
// The property ID type was not numeric. 
//
#define SCRIPTPI_E_PID_NOT_NUMERIC       0x80042002L

//
// MessageId: SCRIPTPI_E_CHUNK_NOT_VALUE
//
// MessageText:
//
// The chunk type is not value. 
//
#define SCRIPTPI_E_CHUNK_NOT_VALUE       0x80042003L

//uebug Is this jargon (chunk, IGatherScriptFilterSink) something users will understand? 
//
// MessageId: SCRIPTPI_E_CANNOT_ALTER_CHUNK
//
// MessageText:
//
// The chunk does not allow changes. To set properties on a chunk, create it using IGatherScriptFilterSink::CreateChunk
//
#define SCRIPTPI_E_CANNOT_ALTER_CHUNK    0x80042004L

//
// MessageId: SCRIPTPI_E_ALREADY_COMPLETED
//
// MessageText:
//
// The resource completion code was already set. 
//
#define SCRIPTPI_E_ALREADY_COMPLETED     0x80042005L

// mapi errors
//uebug What is not supported?  Also, for all MAPI bugs, we should capitalize MAPI, and consider removing it from the beginning of the message and inserting into the text instead.
//
// MessageId: _MAPI_E_NO_SUPPORT
//
// MessageText:
//
// MAPI: This is not supported
//
#define _MAPI_E_NO_SUPPORT               0x80040102L

//uebug Avoid "bad".  Where is the character width "bad"?  Avoid prefacing bug with "MAPI:".
//
// MessageId: _MAPI_E_BAD_CHARWIDTH
//
// MessageText:
//
// MAPI: The character width is bad.
//
#define _MAPI_E_BAD_CHARWIDTH            0x80040103L

//uebug Which string?
//
// MessageId: _MAPI_E_STRING_TOO_LONG
//
// MessageText:
//
// MAPI: The string is too long.
//
#define _MAPI_E_STRING_TOO_LONG          0x80040105L

//uebug Is the caller MAPI, or the parameters?  In other words, is this correct?: "The MAPI caller passed an invalid parameter or flag."  
//
// MessageId: _MAPI_E_UNKNOWN_FLAGS
//
// MessageText:
//
// MAPI: The caller passed an invalid parameter or flag. 
//
#define _MAPI_E_UNKNOWN_FLAGS            0x80040106L

//uebug suggest: "The MAPI Entry ID is invalid. "
//
// MessageId: _MAPI_E_INVALID_ENTRYID
//
// MessageText:
//
// MAPI: The entry ID is invalid
//
#define _MAPI_E_INVALID_ENTRYID          0x80040107L

//
// MessageId: _MAPI_E_INVALID_OBJECT
//
// MessageText:
//
// The MAPI object is invalid. 
//
#define _MAPI_E_INVALID_OBJECT           0x80040108L

//
// MessageId: _MAPI_E_OBJECT_CHANGED
//
// MessageText:
//
// The MAPI object was changed.  
//
#define _MAPI_E_OBJECT_CHANGED           0x80040109L

//
// MessageId: _MAPI_E_OBJECT_DELETED
//
// MessageText:
//
// The MAPI object was deleted. 
//
#define _MAPI_E_OBJECT_DELETED           0x8004010AL

//uebug "The MAPI server is busy."  <--Is this accurate?
//
// MessageId: _MAPI_E_BUSY
//
// MessageText:
//
// MAPI: The server is busy
//
#define _MAPI_E_BUSY                     0x8004010BL

//uebug How is disk space related to MAPI here?
//
// MessageId: _MAPI_E_NOT_ENOUGH_DISK
//
// MessageText:
//
// MAPI: The computer is out of disk space
//
#define _MAPI_E_NOT_ENOUGH_DISK          0x8004010DL

//uebug how about "...to complete the MAPI operation"?
//
// MessageId: _MAPI_E_NOT_ENOUGH_RESOURCES
//
// MessageText:
//
// MAPI: Not enough system resources were available to complete the operation. 
//
#define _MAPI_E_NOT_ENOUGH_RESOURCES     0x8004010EL

//
// MessageId: _MAPI_E_NOT_FOUND
//
// MessageText:
//
// The MAPI object was not found. 
//
#define _MAPI_E_NOT_FOUND                0x8004010FL

//
// MessageId: _MAPI_E_VERSION
//
// MessageText:
//
// The version of OLE installed on the workstation is not compatible with this version of MAPI. 
//
#define _MAPI_E_VERSION                  0x80040110L

//
// MessageId: _MAPI_E_LOGON_FAILED
//
// MessageText:
//
// MAPI: Logon failed.  
//
#define _MAPI_E_LOGON_FAILED             0x80040111L

//
// MessageId: _MAPI_E_SESSION_LIMIT
//
// MessageText:
//
// MAPI: The session limit was exceeded. 
//
#define _MAPI_E_SESSION_LIMIT            0x80040112L

//
// MessageId: _MAPI_E_USER_CANCEL
//
// MessageText:
//
// MAPI: The user cancelled the operation. 
//
#define _MAPI_E_USER_CANCEL              0x80040113L

//
// MessageId: _MAPI_E_UNABLE_TO_ABORT
//
// MessageText:
//
// MAPI: The operation cannot be stopped. 
//
#define _MAPI_E_UNABLE_TO_ABORT          0x80040114L

//
// MessageId: _MAPI_E_NETWORK_ERROR
//
// MessageText:
//
// MAPI: A network error occurred. 
//
#define _MAPI_E_NETWORK_ERROR            0x80040115L

//
// MessageId: _MAPI_E_DISK_ERROR
//
// MessageText:
//
// MAPI: A disk error occurred. 
//
#define _MAPI_E_DISK_ERROR               0x80040116L

//
// MessageId: _MAPI_E_TOO_COMPLEX
//
// MessageText:
//
// The calling process is an NT service, and registry keys required by MAPI could not be initialized.  
//
#define _MAPI_E_TOO_COMPLEX              0x80040117L

//uebug Avoid "bad".  Rewrite so MAPI is part of the text, and not preceding it with a colon.  
//
// MessageId: _MAPI_E_BAD_COLUMN
//
// MessageText:
//
// MAPI: A bad column was requested.
//
#define _MAPI_E_BAD_COLUMN               0x80040118L

//uebug What is an extended error?  It is vague terminology, which we should avoid, if possible.
//
// MessageId: _MAPI_E_EXTENDED_ERROR
//
// MessageText:
//
// MAPI:An extended error occurred.  
//
#define _MAPI_E_EXTENDED_ERROR           0x80040119L

//uebug What was computed?
//
// MessageId: _MAPI_E_COMPUTED
//
// MessageText:
//
// MAPI: Computed
//
#define _MAPI_E_COMPUTED                 0x8004011AL

//uebug What about "The MAPI data is corrupt"?
//
// MessageId: _MAPI_E_CORRUPT_DATA
//
// MessageText:
//
// MAPI: The data is corrupt. 
//
#define _MAPI_E_CORRUPT_DATA             0x8004011BL

//uebug What is not configured?  How about "MAPI is not configured.  "
//
// MessageId: _MAPI_E_UNCONFIGURED
//
// MessageText:
//
// MAPI: Not configured
//
#define _MAPI_E_UNCONFIGURED             0x8004011CL

//
// MessageId: _MAPI_E_FAILONEPROVIDER
//
// MessageText:
//
// MAPI: At least one provider failed. 
//
#define _MAPI_E_FAILONEPROVIDER          0x8004011DL

//
// MessageId: _MAPI_E_UNKNOWN_CPID
//
// MessageText:
//
// MAPI: A code page ID is unknown. 
//
#define _MAPI_E_UNKNOWN_CPID             0x8004011EL

//
// MessageId: _MAPI_E_UNKNOWN_LCID
//
// MessageText:
//
// MAPI: A locale ID is unknown. 
//
#define _MAPI_E_UNKNOWN_LCID             0x8004011FL

//
// MessageId: _MAPI_E_PASSWORD_CHANGE_REQUIRED
//
// MessageText:
//
// MAPI: Access is denied. Change your password.
//
#define _MAPI_E_PASSWORD_CHANGE_REQUIRED 0x80040120L

//
// MessageId: _MAPI_E_PASSWORD_EXPIRED
//
// MessageText:
//
// MAPI: Access is denied, because the password has expired. 
//
#define _MAPI_E_PASSWORD_EXPIRED         0x80040121L

//
// MessageId: _MAPI_E_INVALID_WORKSTATION_ACCOUNT
//
// MessageText:
//
// MAPI: Access is denied, because the workstation account is invalid. 
//
#define _MAPI_E_INVALID_WORKSTATION_ACCOUNT 0x80040122L

//
// MessageId: _MAPI_E_INVALID_ACCESS_TIME
//
// MessageText:
//
// MAPI: Access is denied, because the access time is invalid. 
//
#define _MAPI_E_INVALID_ACCESS_TIME      0x80040123L

//
// MessageId: _MAPI_E_ACCOUNT_DISABLED
//
// MessageText:
//
// MAPI: Access is denied, because the account is disabled. 
//
#define _MAPI_E_ACCOUNT_DISABLED         0x80040124L

//uebug How about "The MAPI session has ended"?
//
// MessageId: _MAPI_E_END_OF_SESSION
//
// MessageText:
//
// MAPI: The session has ended. 
//
#define _MAPI_E_END_OF_SESSION           0x80040200L

//uebug Is "entry ID" capitalized? If not, previous references should be made lowercase. If so, this should be capitalized.
//
// MessageId: _MAPI_E_UNKNOWN_ENTRYID
//
// MessageText:
//
// MAPI: An entry ID is unknown. 
//
#define _MAPI_E_UNKNOWN_ENTRYID          0x80040201L

//
// MessageId: _MAPI_E_MISSING_REQUIRED_COLUMN
//
// MessageText:
//
// MAPI: A required column is missing. 
//
#define _MAPI_E_MISSING_REQUIRED_COLUMN  0x80040202L

//
// MessageId: _MAPI_W_NO_SERVICE
//
// MessageText:
//
// The MAPI service is unavailable. 
//
#define _MAPI_W_NO_SERVICE               0x00040203L

//uebug Is this for testing purposes only, or is this exposed to the users?
//
// MessageId: MSG_TEST_MESSAGE
//
// MessageText:
//
// The %1 message facility is running.
//
#define MSG_TEST_MESSAGE                 0x40041000L

//
// MessageId: FLTRDMN_E_UNEXPECTED
//
// MessageText:
//
// An unexpected error occurred in the filtering process. Contact Microsoft Product Support.
//
#define FLTRDMN_E_UNEXPECTED             0x80042401L

//
// MessageId: FLTRDMN_E_QI_FILTER_FAILED
//
// MessageText:
//
// The document IFilter cannot provide the needed interface. The IFilter may contain errors.
//
#define FLTRDMN_E_QI_FILTER_FAILED       0x80042402L

//
// MessageId: FLTRDMN_E_FILTER_INIT_FAILED
//
// MessageText:
//
// A document IFilter cannot be initialized. The document or IFilter may contain errors.
//
#define FLTRDMN_E_FILTER_INIT_FAILED     0x80042404L

//
// MessageId: FLTRDMN_E_ENCRYPTED_DOCUMENT
//
// MessageText:
//
// An encrypted document cannot be filtered.
//
#define FLTRDMN_E_ENCRYPTED_DOCUMENT     0x80042405L

//
// MessageId: FLTRDMN_E_CANNOT_DECRYPT_PASSWORD
//
// MessageText:
//
// The password for the content access account cannot be decrypted because it was stored with different credentials.  Re-type the password for the account used to crawl this content.
//
#define FLTRDMN_E_CANNOT_DECRYPT_PASSWORD 0x80042406L

//
// MessageId: OLEDB_BINDER_CUSTOM_ERROR
//
// MessageText:
//
// A custom error (%1) in the OLEDB provider has occurred. %2.
//
#define OLEDB_BINDER_CUSTOM_ERROR        0x80042500L

//
// MessageId: NOTESPH_E_UNEXPECTED_STATE
//
// MessageText:
//
// An unexpected error occurred in the Notes protocol handler while processing the URL.
//
#define NOTESPH_E_UNEXPECTED_STATE       0x80042601L

//
// MessageId: NOTESPH_S_IGNORE_ID
//
// MessageText:
//
// The group or person was successfully ignored.
//
#define NOTESPH_S_IGNORE_ID              0x00042602L

//
// MessageId: NOTESPH_E_UNSUPPORTED_CONTENT_FIELD_TYPE
//
// MessageText:
//
// Only rich text, HTML, and text types are supported for the content field.
//
#define NOTESPH_E_UNSUPPORTED_CONTENT_FIELD_TYPE 0x80042603L

//
// MessageId: NOTESPH_E_ITEM_NOT_FOUND
//
// MessageText:
//
// A requested item is not found.
//
#define NOTESPH_E_ITEM_NOT_FOUND         0x80042604L

//
// MessageId: NOTESPH_E_SERVER_CONFIG
//
// MessageText:
//
// The Notes data cannot be accessed. Check that the server is properly configured for accessing Notes data.
//
#define NOTESPH_E_SERVER_CONFIG          0x80042605L

//
// MessageId: NOTESPH_E_ATTACHMENTS
//
// MessageText:
//
// One or more attachments cannot be processed.
//
#define NOTESPH_E_ATTACHMENTS            0x80042606L

//
// MessageId: NOTESPH_E_NO_NTID
//
// MessageText:
//
// The Windows NT identity does not exist.
//
#define NOTESPH_E_NO_NTID                0x80042607L

//
// MessageId: NOTESPH_E_DB_ACCESS_DENIED
//
// MessageText:
//
// Access to %1 is denied to user %1. To allow access, contact your Notes administrator.
//
#define NOTESPH_E_DB_ACCESS_DENIED       0x80042608L

//
// MessageId: NOTESPH_E_NOTESSETUP_ID_MAPPING_ERROR
//
// MessageText:
//
// The user name mapping cannot be read. Check that the database, view, and column names are correct, that the Notes database is accessible, and that the sort option is enabled. For more information, see the Administrator's Guide.
//
#define NOTESPH_E_NOTESSETUP_ID_MAPPING_ERROR 0x80042609L

//
// MessageId: NOTESPH_S_LISTKNOWNFIELDS
//
// MessageText:
//
// One or more Lotus Notes database forms cannot be read. Some fields might not be available.
//
#define NOTESPH_S_LISTKNOWNFIELDS        0x00042610L

//
// MessageId: NOTESPH_E_FAIL
//
// MessageText:
//
// Unknown Lotus Notes Error: %1.
//
#define NOTESPH_E_FAIL                   0x80042611L

//
// MessageId: STS_ABORTXMLPARSE
//
// MessageText:
//
// Stop Parsing XML 
//
#define STS_ABORTXMLPARSE                0x80042614L

//
// MessageId: STS_WS_ERROR
//
// MessageText:
//
// Error in the Site Data Web Service.
//
#define STS_WS_ERROR                     0x80042616L

//
// MessageId: SPS_WS_ERROR
//
// MessageText:
//
// Error in PortalCrawl Web Service.
//
#define SPS_WS_ERROR                     0x80042617L

//
// MessageId: EXSTOREPH_E_UNEXPECTED
//
// MessageText:
//
// An unexpected error occurred in the exstore protocol handler. Contact Microsoft Product Support.
//
#define EXSTOREPH_E_UNEXPECTED           0x80042701L

//
// MessageId: CERT_E_NOT_FOUND_OR_NO_PERMISSSION
//
// MessageText:
//
// Cannot find or access the client certificate specified for crawling this site.
//
#define CERT_E_NOT_FOUND_OR_NO_PERMISSSION 0x80042801L

//
// MessageId: SRCH_SCHEMA_CACHE_E_UNEXPECTED
//
// MessageText:
//
// An unexpected error occurred in the in-memory schema cache.
//
#define SRCH_SCHEMA_CACHE_E_UNEXPECTED   0x80043301L

//
// MessageId: CONTENT_SOURCE_E_PROPERTY_MAPPING_READ
//
// MessageText:
//
// The property mapping information on the content source cannot be read.
//
#define CONTENT_SOURCE_E_PROPERTY_MAPPING_READ 0x80043401L

//
// MessageId: CONTENT_SOURCE_E_UNEXPECTED_NULL_POINTER
//
// MessageText:
//
// The content source information code contains unexpected null pointers.
//
#define CONTENT_SOURCE_E_UNEXPECTED_NULL_POINTER 0x80043402L

//
// MessageId: CONTENT_SOURCE_E_PROPERTY_MAPPING_BAD_VECTOR_SIZE
//
// MessageText:
//
// Parallel vectors for property mapping information on the content source have different dimensions.
//
#define CONTENT_SOURCE_E_PROPERTY_MAPPING_BAD_VECTOR_SIZE 0x80043403L

//
// MessageId: CONTENT_SOURCE_E_CONTENT_CLASS_READ
//
// MessageText:
//
// The content class information for the content source cannot be read.
//
#define CONTENT_SOURCE_E_CONTENT_CLASS_READ 0x80043404L

//
// MessageId: CONTENT_SOURCE_E_UNEXPECTED_EXCEPTION
//
// MessageText:
//
// An unexpected exception occurred in the content source information code.
//
#define CONTENT_SOURCE_E_UNEXPECTED_EXCEPTION 0x80043405L

//
// MessageId: CONTENT_SOURCE_E_NULL_CONTENT_CLASS_BSTR
//
// MessageText:
//
// The content class of the content source is empty.
//
#define CONTENT_SOURCE_E_NULL_CONTENT_CLASS_BSTR 0x80043406L

//
// MessageId: CONTENT_SOURCE_E_CONTENT_SOURCE_COLUMN_TYPE
//
// MessageText:
//
// An unknown data type was found when reading content class information from the content source.
//
#define CONTENT_SOURCE_E_CONTENT_SOURCE_COLUMN_TYPE 0x80043407L

//
// MessageId: CONTENT_SOURCE_E_OUT_OF_RANGE
//
// MessageText:
//
// A request for property mapping information is out of range.
//
#define CONTENT_SOURCE_E_OUT_OF_RANGE    0x80043408L

//
// MessageId: CONTENT_SOURCE_E_NULL_URI
//
// MessageText:
//
// Empty or null URIs are not valid as property mapping.
//
#define CONTENT_SOURCE_E_NULL_URI        0x80043409L

//
// MessageId: REXSPH_E_INVALID_CALL
//
// MessageText:
//
// This is an internal error code. A method was called when it should not be. Call Microsoft Product Support.
//
#define REXSPH_E_INVALID_CALL            0x80043500L

//
// MessageId: REXSPH_S_REDIRECTED
//
// MessageText:
//
// The URL was redirected and will be handled automatically.
//
#define REXSPH_S_REDIRECTED              0x00043501L

//
// MessageId: REXSPH_E_REDIRECT_ON_SECURITY_UPDATE
//
// MessageText:
//
// A security update was attempted on an address that was redirected to a different server.
//
#define REXSPH_E_REDIRECT_ON_SECURITY_UPDATE 0x80043502L

//
// MessageId: REXSPH_E_MULTIPLE_REDIRECT
//
// MessageText:
//
// An address was redirected more than once.
//
#define REXSPH_E_MULTIPLE_REDIRECT       0x80043503L

//
// MessageId: REXSPH_E_NO_PROPERTY_ON_ROW
//
// MessageText:
//
// The requested property does not exist on the row.
//
#define REXSPH_E_NO_PROPERTY_ON_ROW      0x80043504L

//
// MessageId: REXSPH_E_TYPE_MISMATCH_ON_READ
//
// MessageText:
//
// The requested read type does not match the data type.
//
#define REXSPH_E_TYPE_MISMATCH_ON_READ   0x80043505L

//
// MessageId: REXSPH_E_UNEXPECTED_DATA_STATUS
//
// MessageText:
//
// The status of the request data is unexpected. Call Microsoft Product Support.
//
#define REXSPH_E_UNEXPECTED_DATA_STATUS  0x80043506L

//
// MessageId: REXSPH_E_UNKNOWN_DATA_TYPE
//
// MessageText:
//
// An unknown data type was found. Call Microsoft Product Support.
//
#define REXSPH_E_UNKNOWN_DATA_TYPE       0x80043507L

//
// MessageId: REXSPH_E_UNEXPECTED_FILTER_STATE
//
// MessageText:
//
// An unexpected filter state occurred. Call Microsoft Product Support.
//
#define REXSPH_E_UNEXPECTED_FILTER_STATE 0x80043508L

//
// MessageId: REXSPH_E_DUPLICATE_PROPERTY
//
// MessageText:
//
// A duplicate property was found. Call Microsoft support if this error is encountered.
//
#define REXSPH_E_DUPLICATE_PROPERTY      0x80043509L

//
// MessageId: PEOPLE_IMPORT_E_DBCONNFAIL
//
// MessageText:
//
// The user profile database connection cannot be made, because of error %1.
//
#define PEOPLE_IMPORT_E_DBCONNFAIL       0x80044000L

//
// MessageId: PEOPLE_IMPORT_NODSDEFINED
//
// MessageText:
//
// No data source is defined.
//
#define PEOPLE_IMPORT_NODSDEFINED        0x80044001L

//
// MessageId: PEOPLE_IMPORT_E_FAILTOGETDSDEF
//
// MessageText:
//
// The data source definition cannot be retrieved, because of error %1.
//
#define PEOPLE_IMPORT_E_FAILTOGETDSDEF   0x80044002L

//
// MessageId: PEOPLE_IMPORT_NOMAPPINGDEFINED
//
// MessageText:
//
// No data source property mapping is defined.
//
#define PEOPLE_IMPORT_NOMAPPINGDEFINED   0x80044003L

//
// MessageId: PEOPLE_IMPORT_E_FAILTOGETDSMAPPING
//
// MessageText:
//
// Data source mapping cannot be retrieved. Check to see that the mapping entry is valid.
//
#define PEOPLE_IMPORT_E_FAILTOGETDSMAPPING 0x80044004L

//
// MessageId: PEOPLE_IMPORT_E_DATATYPENOTSUPPORTED
//
// MessageText:
//
// Datatype (%1) in the data source is not supported.
//
#define PEOPLE_IMPORT_E_DATATYPENOTSUPPORTED 0x80044005L

//
// MessageId: PEOPLE_IMPORT_E_NOCASTINGSUPPORTED
//
// MessageText:
//
// (%1) cannot be converted to type (%2).
//
#define PEOPLE_IMPORT_E_NOCASTINGSUPPORTED 0x80044006L

//
// MessageId: PEOPLE_IMPORT_E_UPDATE_DIRSYNC_COOKIE
//
// MessageText:
//
// Incremental import information cannot be updated.
//
#define PEOPLE_IMPORT_E_UPDATE_DIRSYNC_COOKIE 0x80044007L

//
// MessageId: PEOPLE_IMPORT_E_DIRSYNC_ZERO_COOKIE
//
// MessageText:
//
// A zero-length cookie was retrieved at the end of a Dirsync search. The account may not have the appropriate rights.
//
#define PEOPLE_IMPORT_E_DIRSYNC_ZERO_COOKIE 0x80044008L

//
// MessageId: PEOPLE_IMPORT_E_LDAPPATH_TOOLONG
//
// MessageText:
//
// The specified LDAP distinguished name is too long.
//
#define PEOPLE_IMPORT_E_LDAPPATH_TOOLONG 0x80044009L

//
// MessageId: PEOPLE_IMPORT_E_CANONICALURL_TOOLONG
//
// MessageText:
//
// The canonical URL is too long.
//
#define PEOPLE_IMPORT_E_CANONICALURL_TOOLONG 0x8004400AL

//
// MessageId: PEOPLE_IMPORT_E_USERNAME_NOTRESOLVED
//
// MessageText:
//
// The user account name can not be resolved within the Active Directory. You would need to verify whether the user is a real user or just a temporary user object generated for some application purpose. You might be able to refine the LDAP search filter to avoid this error.
//
#define PEOPLE_IMPORT_E_USERNAME_NOTRESOLVED 0x8004400BL

//
// MessageId: PEOPLE_IMPORT_E_DC_NOT_AVAILABLE
//
// MessageText:
//
// Error (%1) occurred, which may have resulted from the unavailability of directory service server (%2). If the auto discovery option is turned on, import will re-try it with the newly discovered domain controller.  Otherwise, verify your import settings and make sure the server is still available.
//
#define PEOPLE_IMPORT_E_DC_NOT_AVAILABLE 0x8004400CL

//
// MessageId: PEOPLE_IMPORT_E_DOMAIN_DISCOVER_FAILED
//
// MessageText:
//
// The attempt to discover domain controller for domain (%1) was failed with error (%2). Please make sure whether there is problem in network connectivity or the domain is renamed.
//
#define PEOPLE_IMPORT_E_DOMAIN_DISCOVER_FAILED 0x8004400DL

//
// MessageId: PEOPLE_IMPORT_E_FAILTOGETLCID
//
// MessageText:
//
// Fail to retrieve locale from site database - Error (%1) occured.
//
#define PEOPLE_IMPORT_E_FAILTOGETLCID    0x8004400EL

//
// MessageId: PEOPLE_IMPORT_E_DOMAIN_REMOVED
//
// MessageText:
//
// The specified domain (%1) has been removed from import configuration. 
//
#define PEOPLE_IMPORT_E_DOMAIN_REMOVED   0x8004400FL

//
// MessageId: PEOPLE_IMPORT_E_ENUM_ACCESSDENIED
//
// MessageText:
//
// Access was denied on domain: (%1), and user information from the domain was not imported. Check the user name and password of the access account specified on the Configure Profile Import page. If incremental import is enabled and you are importing from a Windows 2000 domain, check that the access account has the Replicate Changes permission for Active Directory directory services.
//
#define PEOPLE_IMPORT_E_ENUM_ACCESSDENIED 0x80044010L

//
// MessageId: PEOPLE_IMPORT_E_DIRSYNC_NOTREFRESHED
//
// MessageText:
//
// You do not have sufficient permissions for incremental import, or the DirSync cookie stored in the user profile database is corrupt. If you are importing from a Windows 2000 domain, check that the import account has the Replicate Changes permission for Active Directory directory services. If the import account has the permission, start a full import to refresh the cookie.
//
#define PEOPLE_IMPORT_E_DIRSYNC_NOTREFRESHED 0x80044011L

  // copied from GTHR_E_SECRET_NOT_FOUND
// Not used by MSFTESQL
//
// MessageId: FTE_E_SECRET_NOT_FOUND
//
// MessageText:
//
// The account password was not specified. Specify the password.
//
#define FTE_E_SECRET_NOT_FOUND           0x80043602L

  // NewLocStr 7/01
//
// MessageId: FTE_E_PIPE_NOT_CONNECTED
//
// MessageText:
//
// The named pipe used to communicate with the filter daemon has not been connected.
//
#define FTE_E_PIPE_NOT_CONNECTED         0x80043603L

  // NewLocStr 7/01
//
// MessageId: FTE_E_ADMIN_BLOB_CORRUPT
//
// MessageText:
//
// The configuration data given to the MSFTESQL service is corrupt.
//
#define FTE_E_ADMIN_BLOB_CORRUPT         0x80043604L

 // copied from GTHR_E_FILTER_SINGLE_THREADED
//
// MessageId: FTE_E_FILTER_SINGLE_THREADED
//
// MessageText:
//
// The system attempted to load an apartment threading model filter marked in a multi-threaded filter daemon. The document will be retried in a single-threaded filter daemon process. Since multithreaded filtering is more efficient, try to obtain the version of the filter that is multi-threaded.
//
#define FTE_E_FILTER_SINGLE_THREADED     0x80043605L

 // copied from GTHR_E_ERROR_WRITING_REGISTRY
//
// MessageId: FTE_E_ERROR_WRITING_REGISTRY
//
// MessageText:
//
// The value cannot be set, because the object was already deleted or was not initialized properly. Make sure the object reference is still valid, increase the registry size, or recreate the catalog configuration.
//
#define FTE_E_ERROR_WRITING_REGISTRY     0x80043606L

  // NewLocStr 7/01
//
// MessageId: FTE_E_PROJECT_SHUTDOWN
//
// MessageText:
//
// An internal interface is being used after the corresponding catalog has been shutdown. The operation will be aborted.
//
#define FTE_E_PROJECT_SHUTDOWN           0x80043607L

  // NewLocStr 7/01
//
// MessageId: FTE_E_PROJECT_NOT_INITALIZED
//
// MessageText:
//
// An internal interface is being used prior to being initialized. The operation will be aborted.
//
#define FTE_E_PROJECT_NOT_INITALIZED     0x80043608L

  // NewLocStr 7/01
//
// MessageId: FTE_E_PIPE_DATA_CORRUPTED
//
// MessageText:
//
// Data transferred between the MSFTESQL service and a filter daemon process is corrupted. This is an internal error.
//
#define FTE_E_PIPE_DATA_CORRUPTED        0x80043609L

  // NewLocStr 7/01
//
// MessageId: FTE_E_URB_TOO_BIG
//
// MessageText:
//
// This is an internal error: The URB has exceeded the maximum size.
//
#define FTE_E_URB_TOO_BIG                0x80043610L

  // NewLocStr 7/01
//
// MessageId: FTE_E_INVALID_DOCID
//
// MessageText:
//
// This is an internal error: Document IDs should be greater than 0 and less than or equal to 0x7fffffff.
//
#define FTE_E_INVALID_DOCID              0x80043611L

  // NewLocStr 7/01
//
// MessageId: FTE_E_PAUSE_EXTERNAL
//
// MessageText:
//
// An external status change has put the catalog in a paused state.
//
#define FTE_E_PAUSE_EXTERNAL             0x80043612L

  // NewLocStr 7/01
//
// MessageId: FTE_E_REJECTED_DUE_TO_PROJECT_STATUS
//
// MessageText:
//
// A status change is occurring or the project is in a force paused state, so MSFTESQL cannot accept input at this time.
//
#define FTE_E_REJECTED_DUE_TO_PROJECT_STATUS 0x80043613L

  // NewLocStr 7/13
//
// MessageId: FTE_E_FD_DID_NOT_CONNECT
//
// MessageText:
//
// The MSFTEFD process was launched but did not connect with the MSFTESQL service.
//
#define FTE_E_FD_DID_NOT_CONNECT         0x80043614L

  // NewLocStr 7/14
//
// MessageId: FTE_E_PROGID_REQUIRED
//
// MessageText:
//
// This is an internal error: Initialization of the datasink is incorrect.  At least one protocol handler PROGID is required.
//
#define FTE_E_PROGID_REQUIRED            0x80043616L

  // NewLocStr 7/14
//
// MessageId: FTE_E_STATIC_THREAD_INVALID_ARGUMENTS
//
// MessageText:
//
// This is an internal error:  A static thread has gotten invalid arguments and will force batches to be aborted and retried.
//
#define FTE_E_STATIC_THREAD_INVALID_ARGUMENTS 0x80043617L

  // NewLocStr 7/25
//
// MessageId: FTE_E_CATALOG_ALREADY_EXISTS
//
// MessageText:
//
// A catalog already exists with this name, so another can be created or mounted.
//
#define FTE_E_CATALOG_ALREADY_EXISTS     0x80043618L

  // NewLocStr 7/31
//
// MessageId: FTE_S_RESOURCES_STARTING_TO_GET_LOW
//
// MessageText:
//
// The Full Text Engine's input queue is getting full.  This batch has been accepted for processing.  This success code is intended to help pause input until the queue is less full.
//
#define FTE_S_RESOURCES_STARTING_TO_GET_LOW 0x00043619L

  // NewLocStr 5/12/2002
//
// MessageId: FTE_E_PATH_TOO_LONG
//
// MessageText:
//
// A file path exceeds the maximum limit for paths in Windows, so it can't be used.
//
#define FTE_E_PATH_TOO_LONG              0x8004361AL

  // NewLocStr 5/28/2002
//
// MessageId: FTE_INVALID_ADMIN_CLIENT
//
// MessageText:
//
// Access is denied to the caller of this administration interface.
//
#define FTE_INVALID_ADMIN_CLIENT         0x8004361BL

  // NewLocStr 6/18/2002
//
// MessageId: FTE_E_COM_SIGNATURE_VALIDATION
//
// MessageText:
//
// Signature validation cannot be performed on modules loaded by COM, so the object will not be created. The object is likely a filter, wordbreaker, stemmer, or protocol handler.
//
#define FTE_E_COM_SIGNATURE_VALIDATION   0x8004361CL

  // NewLocStr 7/1/2002
//
// MessageId: FTE_E_AFFINITY_MASK
//
// MessageText:
//
// The processor affinity mask is invalid.
//
#define FTE_E_AFFINITY_MASK              0x8004361DL

  // NewLocStr 3/2003
//
// MessageId: FTE_E_FD_OWNERSHIP_OBSOLETE
//
// MessageText:
//
// This is an internal error that should be handled. The FD has been killed and this chunk buffer has already been reassigned.
//
#define FTE_E_FD_OWNERSHIP_OBSOLETE      0x8004361EL

  // NewLocStr 8/6
//
// MessageId: FTE_E_EXCEEDED_MAX_PLUGINS
//
// MessageText:
//
// The maximum number of plug-ins has been exceeded, so a new plug-in can't be loaded.
//
#define FTE_E_EXCEEDED_MAX_PLUGINS       0x80043621L

  // NewLocStr 8/12
//
// MessageId: FTE_S_BEYOND_QUOTA
//
// MessageText:
//
// The Full Text Engine's input queue is full.  This batch has been accepted for processing; however, the Full Text Engine will soon go into a forced paused state until the queue is less full.
//
#define FTE_S_BEYOND_QUOTA               0x00043622L

  // NewLocStr 8/17
//
// MessageId: FTE_E_DUPLICATE_OBJECT
//
// MessageText:
//
// An object could not be inserted because it was a duplicate of an existing object. The object may be a catalog or other named entity.
//
#define FTE_E_DUPLICATE_OBJECT           0x80043624L

  // NewLocStr 8/30
//
// MessageId: FTE_S_REDUNDANT
//
// MessageText:
//
// This transaction was superseded by a subsequent transaction, so it will not be completed.
//
#define FTE_S_REDUNDANT                  0x00043625L

  // NewLocStr 8/30
//
// MessageId: FTE_E_REDUNDANT_TRAN_FAILURE
//
// MessageText:
//
// The transaction that superseded this one ended in error.
//
#define FTE_E_REDUNDANT_TRAN_FAILURE     0x80043626L

  // NewLocStr 8/30
//
// MessageId: FTE_E_DEPENDENT_TRAN_FAILED_TO_PERSIST
//
// MessageText:
//
// The transaction that superseded this one ended in error.
//
#define FTE_E_DEPENDENT_TRAN_FAILED_TO_PERSIST 0x80043627L

  // NewLocStr 9/1
//
// MessageId: FTE_E_FD_SHUTDOWN
//
// MessageText:
//
// This is an internal error: This request cannot be completed because the Filter Daemon has been shutdown.
//
#define FTE_E_FD_SHUTDOWN                0x80043628L

  // NewLocStr 9/1
//
// MessageId: FTE_E_CATALOG_DOES_NOT_EXIST
//
// MessageText:
//
// The catalog does not exist, so the operation can't be performed.
//
#define FTE_E_CATALOG_DOES_NOT_EXIST     0x80043629L

  // NewLocStr 11/28
//
// MessageId: FTE_E_NO_PLUGINS
//
// MessageText:
//
// There are no plug-in components in the indexing pipeline, so the data collected will not be used.
//
#define FTE_E_NO_PLUGINS                 0x8004362AL

  // NewLocStr 12/05
//
// MessageId: FTE_S_STATUS_CHANGE_REQUEST
//
// MessageText:
//
// The project state has changed or is changing due to a status change request.
//
#define FTE_S_STATUS_CHANGE_REQUEST      0x0004362BL

  // NewLocStr 12/07
//
// MessageId: FTE_E_BATCH_ABORTED
//
// MessageText:
//
// Processing of this batch of transactions has been aborted.
//
#define FTE_E_BATCH_ABORTED              0x8004362CL

  // NewLocStr 12/27
//
// MessageId: FTE_E_ANOTHER_STATUS_CHANGE_IS_ALREADY_ACTIVE
//
// MessageText:
//
// A status change is active on another thread. Since only one status change is allowed at a time this request can't be handled.
//
#define FTE_E_ANOTHER_STATUS_CHANGE_IS_ALREADY_ACTIVE 0x8004362DL

  // NewLocStr 02/04
//
// MessageId: FTE_S_RESUME
//
// MessageText:
//
// This is an internal error: The project will be resumed.
//
#define FTE_S_RESUME                     0x0004362EL

  // NewLocStr 02/05
//
// MessageId: FTE_E_NOT_PROCESSED_DUE_TO_PREVIOUS_ERRORS
//
// MessageText:
//
// A previous error prevented further processing of the batch.
//
#define FTE_E_NOT_PROCESSED_DUE_TO_PREVIOUS_ERRORS 0x8004362FL

  // newLocStr 9/20
//
// MessageId: FTE_E_FD_TIMEOUT
//
// MessageText:
//
// The filter daemon process MSFTEFD timed out for an unknown reason. This may indicate a bug in a filter, wordbreaker, or protocol handler.
//
#define FTE_E_FD_TIMEOUT                 0x80043630L

  // NewLocStr 9/1
//
// MessageId: FTE_E_RESOURCE_SHUTDOWN
//
// MessageText:
//
// This is an internal error: This activity is no longer valid because the resource is shutdown.
//
#define FTE_E_RESOURCE_SHUTDOWN          0x80043631L

  // NewLocStr 9/29
//
// MessageId: FTE_E_INVALID_PROPERTY
//
// MessageText:
//
// The property specified is invalid.
//
#define FTE_E_INVALID_PROPERTY           0x80043632L

  // NewLocStr 9/29
//
// MessageId: FTE_E_NO_MORE_PROPERTIES
//
// MessageText:
//
// There are no more properties.
//
#define FTE_E_NO_MORE_PROPERTIES         0x80043633L

  // NewLocStr 9/29
//
// MessageId: FTE_E_UNKNOWN_PLUGIN
//
// MessageText:
//
// The plug-in specified is not known likely because it isn't loaded, so the operation can't succeed. Only specify plug-ins that are loaded.
//
#define FTE_E_UNKNOWN_PLUGIN             0x80043634L

  // NewLocStr 10/05
//
// MessageId: FTE_E_LIBRARY_NOT_LOADED
//
// MessageText:
//
// The performance monitor library could not be loaded.
//
#define FTE_E_LIBRARY_NOT_LOADED         0x80043635L

  // NewLocStr 10/05
//
// MessageId: FTE_E_PERFMON_FULL
//
// MessageText:
//
// There are no more slots available for this performance monitor instance
//
#define FTE_E_PERFMON_FULL               0x80043636L

  // NewLocStr 10/08
//
// MessageId: FTE_E_FAILED_TO_CREATE_ACCESSOR
//
// MessageText:
//
// The filter daemon process MSFTEFD was not able to create an accessor object for the batch.
//
#define FTE_E_FAILED_TO_CREATE_ACCESSOR  0x80043637L

  // NewLocStr 10/29
//
// MessageId: FTE_E_INVALID_TYPE
//
// MessageText:
//
// The property type specified is incorrect for this property. Please see the product documentation for the correct data type for this property.
//
#define FTE_E_INVALID_TYPE               0x80043638L

  // NewLocStr 10/29
//
// MessageId: FTE_E_OUT_OF_RANGE
//
// MessageText:
//
// The value specified is out of range. Please see the product documentation for the valid range.
//
#define FTE_E_OUT_OF_RANGE               0x80043639L

#ifdef UPGRADE_INTERFACE
  // NewLocStr 12/26
//
// MessageId: FTE_E_CORRUPT_PROPERTY_STORE
//
// MessageText:
//
// The property store for upgrade is corrupted.
//
#define FTE_E_CORRUPT_PROPERTY_STORE     0x8004363AL

  // NewLocStr 12/26
  // not used for MSFTESQL
//
// MessageId: FTE_E_PROPERTY_STORE_WORKID_NOTVALID
//
// MessageText:
//
// The workid is not valid for the property store.
//
#define FTE_E_PROPERTY_STORE_WORKID_NOTVALID 0x8004363BL

  // NewLocStr 12/26
  // not used for MSFTESQL
//
// MessageId: FTE_S_PROPERTY_STORE_END_OF_ENUMERATION
//
// MessageText:
//
// The enumeration has finished for the property store.
//
#define FTE_S_PROPERTY_STORE_END_OF_ENUMERATION 0x0004363CL

  // NewLocStr 12/26
  // not used for MSFTESQL
//
// MessageId: FTE_E_CORRUPT_GATHERER_HASH_MAP
//
// MessageText:
//
// The gatherer hash map for upgrade is corrupted.
//
#define FTE_E_CORRUPT_GATHERER_HASH_MAP  0x8004363DL

  // NewLocStr 12/26
  // not used for MSFTESQL
//
// MessageId: FTE_E_KEY_NOT_CACHED
//
// MessageText:
//
// The key is not cached in the property store.
//
#define FTE_E_KEY_NOT_CACHED             0x8004363EL

  // NewLocStr 12/27
  // not used for MSFTESQL
//
// MessageId: FTE_E_UPGRADE_INTERFACE_ALREADY_SHUTDOWN
//
// MessageText:
//
// The upgrade interface has already been shutdown.
//
#define FTE_E_UPGRADE_INTERFACE_ALREADY_SHUTDOWN 0x8004363FL

  // NewLocStr 12/28
  // not used for MSFTESQL
//
// MessageId: FTE_E_UPGRADE_INTERFACE_ALREADY_INSTANTIATED
//
// MessageText:
//
// The upgrade interface has already been instantiated. Only one instance of the upgrade interface is allowed.
//
#define FTE_E_UPGRADE_INTERFACE_ALREADY_INSTANTIATED 0x80043640L

#endif
  // NewLocStr 01/03/02
//
// MessageId: FTE_E_STACK_CORRUPTED
//
// MessageText:
//
// The stack for a thread in this process is corrupted due to a programming bug. This may be a security threat and indicate your system has been attacked. The process will be shutdown.
//
#define FTE_E_STACK_CORRUPTED            0x80043641L

  // NewLocStr 01/22/02
//
// MessageId: FTE_E_INVALID_PROG_ID
//
// MessageText:
//
// The protocol handler index passed to the OnDataChange function is invalid.
//
#define FTE_E_INVALID_PROG_ID            0x80043642L

  // NewLocStr 01/29/02
//
// MessageId: FTE_E_SERIAL_STREAM_CORRUPT
//
// MessageText:
//
// The serial stream being indexed is corrupted.
//
#define FTE_E_SERIAL_STREAM_CORRUPT      0x80043643L

  // NewLocStr 02/05/02
//
// MessageId: FTE_E_READONLY_CATALOG
//
// MessageText:
//
// The catalog is opened for read-only operations. Writing is prohibited to this catalog.
//
#define FTE_E_READONLY_CATALOG           0x80043644L

  // NewLocStr 02/06/02
//
// MessageId: FTE_E_PERF_NOT_LOADED
//
// MessageText:
//
// The performance monitor counters cannot be unloaded because they are not loaded in the first place.
//
#define FTE_E_PERF_NOT_LOADED            0x80043645L

  // NewLocStr 03/08/02
//
// MessageId: FTE_S_READONLY_CATALOG
//
// MessageText:
//
// The catalog has been opened for read-only operations.
//
#define FTE_S_READONLY_CATALOG           0x00043646L

  // NewLocStr 07/26/02
//
// MessageId: FTE_E_RETRY_HUGE_DOC
//
// MessageText:
//
// This huge document will be retried in dedicated filter daemon MSFTEFD process.
//
#define FTE_E_RETRY_HUGE_DOC             0x80043648L

  // NewLocStr 07/26/02
//
// MessageId: FTE_E_UNKNOWN_FD_TYPE
//
// MessageText:
//
// This is an internal error: The filter daemon MSFTEFD type is unknown or invalid.
//
#define FTE_E_UNKNOWN_FD_TYPE            0x80043649L

  // NewLocStr 07/29/02
//
// MessageId: FTE_E_DOC_TOO_HUGE
//
// MessageText:
//
// There are not enough resources to process the document or row.
//
#define FTE_E_DOC_TOO_HUGE               0x8004364AL

 // E_OUTOFMEMORY and STATUS_DATATYPE_MISALIGNMENT are both 0x80000002
 // STATUS_DATATYPE_MISALIGNMENT is transformed into FTE_E_DATATYPE_MISALIGNMENT
//
// MessageId: FTE_E_DATATYPE_MISALIGNMENT
//
// MessageText:
//
// This is an internal error: Datatype misalignment was detected likely due to a programming error.
//
#define FTE_E_DATATYPE_MISALIGNMENT      0x8004364BL

  // NewLocStr 09/11/02
//
// MessageId: FTE_E_ALREADY_INITIALIZED
//
// MessageText:
//
// The object is already initialized.
//
#define FTE_E_ALREADY_INITIALIZED        0x8004364CL

  // NewLocStr 09/17/02
//
// MessageId: FTE_E_FD_USED_TOO_MUCH_MEMORY
//
// MessageText:
//
// The filter daemon process MSFTEFD used too much memory and will be terminated.
//
#define FTE_E_FD_USED_TOO_MUCH_MEMORY    0x8004364DL

  // newLocStr 10/11/02
//
// MessageId: FTE_E_UNEXPECTED_EXIT
//
// MessageText:
//
// The MSFTESQL service process exited unexpectedly.
// Here is the stack Trace:
// %1
//
#define FTE_E_UNEXPECTED_EXIT            0x8004364EL

  // newLocStr 10/17/02
//
// MessageId: FTE_E_HIGH_MEMORY_PRESSURE
//
// MessageText:
//
// High memory pressure was detected by the MSFTESQL memory manager.
//
#define FTE_E_HIGH_MEMORY_PRESSURE       0x8004364FL

  // newLocStr 11/13/02
//
// MessageId: FTE_E_INVALID_ISOLATE_ERROR_BATCH
//
// MessageText:
//
// A batch flagged to isolate a previous error had too many transactions in it.
//
#define FTE_E_INVALID_ISOLATE_ERROR_BATCH 0x80043650L

  // newLocStr 11/13/02
//
// MessageId: FTE_E_RETRY_SINGLE_DOC_PER_BATCH
//
// MessageText:
//
// msftesql should reprocess this document in an isolated fashion to confirm the error.
//
#define FTE_E_RETRY_SINGLE_DOC_PER_BATCH 0x80043651L

  // newLocStr 11/21/02
//
// MessageId: FTE_E_INVALID_PROJECT_ID
//
// MessageText:
//
// An internal identifier for naming catalogs has been corrupted.
//
#define FTE_E_INVALID_PROJECT_ID         0x80043652L

  // newLocStr 11/2002
//
// MessageId: FTE_E_FAILURE_TO_POST_SETCOMPLETION_STATUS
//
// MessageText:
//
// A failure occurred when tracking the completion of wordlists which will cause current activity to be aborted.
//
#define FTE_E_FAILURE_TO_POST_SETCOMPLETION_STATUS 0x80043653L

  // newLocStr 02/20/2003
//
// MessageId: FTE_E_INVALID_CODEPAGE
//
// MessageText:
//
// The specified code page is not installed or not available.
//
#define FTE_E_INVALID_CODEPAGE           0x80043654L

  // newLocStr 07/25/2003
//
// MessageId: FTE_E_FD_IDLE
//
// MessageText:
//
// Internal Error Code: Filter Daemon is terminated because it is idle.
//
#define FTE_E_FD_IDLE                    0x80043655L

  // newLocStr 07/25/2003
//
// MessageId: FTE_E_FD_UNRESPONSIVE
//
// MessageText:
//
// Filter Daemon was unresponsive to a directive to shut itself down.
//
#define FTE_E_FD_UNRESPONSIVE            0x80043656L

  // newLocStr 07/30/2003
//
// MessageId: FTE_S_TRY_TO_FLUSH
//
// MessageText:
//
// Internal Success Code.  Crawl is done.  Chunk buffers can be flushed; there is no additional input data.
//
#define FTE_S_TRY_TO_FLUSH               0x00043657L

  // newLocStr 10/18/2003
//
// MessageId: FTE_S_CATALOG_BLOB_MISMATCHED
//
// MessageText:
//
// The property requested from the catalog blob received doesn't match the current value of the catalog property.
// The property value used by the catalog was successfully returned.
// The caller should either call SetProperty with the value returned or dismount the catalog and mount it back with the blob.
//
#define FTE_S_CATALOG_BLOB_MISMATCHED    0x00043658L

  // newLocStr 10/29/2003
//
// MessageId: FTE_S_PROPERTY_RESET
//
// MessageText:
//
// One or more properties were reset to the default value.
//
#define FTE_S_PROPERTY_RESET             0x00043659L

  // newLocStr 03/13/2006
//
// MessageId: FTE_E_NO_PROPERTY_STORE
//
// MessageText:
//
// Property Store is not found in the indexer. The possible causes are that the call came during a catalog reset or that the property store plug-in failed to load during indexer initialization.
//
#define FTE_E_NO_PROPERTY_STORE          0xC004365AL

// Use 0xCBxx for Chunk Buffer errors
  // NewLocStr 8/12
//
// MessageId: FTE_E_CB_OUT_OF_MEMORY
//
// MessageText:
//
// This is an internal error: The chunk buffer is out of memory.
//
#define FTE_E_CB_OUT_OF_MEMORY           0x8004CB00L

  // NewLocStr 8/12
//
// MessageId: FTE_E_CB_CBID_OUT_OF_BOUND
//
// MessageText:
//
// This is an internal error: The chunk buffer id is out of bounds.
//
#define FTE_E_CB_CBID_OUT_OF_BOUND       0x8004CB01L

  // NewLocStr 10/10
//
// MessageId: FTE_E_CB_NOT_ENOUGH_AVAIL_PHY_MEM
//
// MessageText:
//
// There is not enough available physical or virtual memory for chunk buffers. Chunk buffers are needed to index data. Please free up memory.
//
#define FTE_E_CB_NOT_ENOUGH_AVAIL_PHY_MEM 0x8004CB02L

  // NewLocStr 12/12
//
// MessageId: FTE_E_CB_NOT_ENOUGH_OCC_BUFFER
//
// MessageText:
//
// There is not enough occurrence buffer memory available. Possible causes include too many processors, too many filter threads, or the occurrence buffer is not being returned.
//
#define FTE_E_CB_NOT_ENOUGH_OCC_BUFFER   0x8004CB03L

  // NewLocStr 10/15/2002
//
// MessageId: FTE_E_CORRUPT_WORDLIST
//
// MessageText:
//
// The in-memory wordlist is corrupted. This is due to a faulty filter, wordbreaker, or other indexing component.
//
#define FTE_E_CORRUPT_WORDLIST           0x8004CB04L

  // NewLocStr 9/08
//
// MessageId: FTE_E_FD_NO_IPERSIST_INTERFACE
//
// MessageText:
//
// The IPersistStream and IPersistFile interfaces were unavailable from an IFilter filter DLL to load data for indexing.
//
#define FTE_E_FD_NO_IPERSIST_INTERFACE   0x8004FD00L

  // NewLocStr 9/08
//
// MessageId: FTE_E_FD_IFILTER_INIT_FAILED
//
// MessageText:
//
// The IFilter::Init() function call failed.
//
#define FTE_E_FD_IFILTER_INIT_FAILED     0x8004FD01L

  // NewLocStr 10/07
//
// MessageId: FTE_E_FD_FAILED_TO_LOAD_IFILTER
//
// MessageText:
//
// The filter daemon MSFTEFD failed to load an IFilter interface for document, so it can't be indexed.
//
#define FTE_E_FD_FAILED_TO_LOAD_IFILTER  0x8004FD02L

  // newLocStr 12/20
//
// MessageId: FTE_E_FD_DOC_TIMEOUT
//
// MessageText:
//
// The document being indexed timed out for an unknown reason. This may be due to a bug in a filter or wordbreaker.
//
#define FTE_E_FD_DOC_TIMEOUT             0x8004FD03L

  // newLocStr 01/02/02
//
// MessageId: FTE_E_FD_UNEXPECTED_EXIT
//
// MessageText:
//
// The filter daemon process MSFTEFD exited unexpectedly.
// Here is the stack trace:
// %1
//
#define FTE_E_FD_UNEXPECTED_EXIT         0x8004FD04L

  // newLocStr 01/02/02
//
// MessageId: FTE_E_FD_DOC_UNEXPECTED_EXIT
//
// MessageText:
//
// The filter daemon process MSFTEFD exited unexpectedly because of document %1.
// Batch Id: %2
// Locale Id: %3
// Property Id: %4
// Stack Trace:
// %5
//
#define FTE_E_FD_DOC_UNEXPECTED_EXIT     0x8004FD05L

  // newLocStr 01/17/02
//
// MessageId: FTE_E_FD_NOISE_NO_TEXT_FILTER
//
// MessageText:
//
// The system failed to load the text filter for parsing a noise word file.
//
#define FTE_E_FD_NOISE_NO_TEXT_FILTER    0x8004FD06L

  // newLocStr 01/17/02
//
// MessageId: FTE_E_FD_NOISE_NO_IPERSISTSTREAM_ON_TEXT_FILTER
//
// MessageText:
//
// The system failed to load noise words stream into text filter because text filter does not support IPersistStream.
//
#define FTE_E_FD_NOISE_NO_IPERSISTSTREAM_ON_TEXT_FILTER 0x8004FD07L

  // newLocStr 01/17/02
//
// MessageId: FTE_E_FD_NOISE_TEXT_FILTER_LOAD_FAILED
//
// MessageText:
//
// The system failed to load noise words stream into text filter because the IPersistStream::Load() function failed.
//
#define FTE_E_FD_NOISE_TEXT_FILTER_LOAD_FAILED 0x8004FD08L

  // newLocStr 01/17/02
//
// MessageId: FTE_E_FD_NOISE_TEXT_FILTER_INIT_FAILED
//
// MessageText:
//
// The system failed to load noise words stream into text filter because IFilter::Init() function failed.
//
#define FTE_E_FD_NOISE_TEXT_FILTER_INIT_FAILED 0x8004FD09L

  // newLocStr 12/12/02
//
// MessageId: FTE_E_FD_OCCURRENCE_OVERFLOW
//
// MessageText:
//
// Occurrence counter overflow, document is not indexed.
//
#define FTE_E_FD_OCCURRENCE_OVERFLOW     0x8004FD0AL

  // newLocStr 01/14/03
//
// MessageId: FTE_E_FD_FILTER_CAUSED_SHARING_VIOLATION
//
// MessageText:
//
// The filter has caused a sharing violation.
//
#define FTE_E_FD_FILTER_CAUSED_SHARING_VIOLATION 0x8004FD0BL

#define ERROR_SOURCE_PROTHNDLR          0x1200
//
// MessageId: PRTH_E_COMM_ERROR
//
// MessageText:
//
// A communication error occurred.
//
#define PRTH_E_COMM_ERROR                ((DWORD)0x80041200L)

//
// MessageId: PRTH_E_OBJ_NOT_FOUND
//
// MessageText:
//
// The object was not found.
//
#define PRTH_E_OBJ_NOT_FOUND             ((DWORD)0x80041201L)

//
// MessageId: PRTH_E_REQUEST_ERROR
//
// MessageText:
//
// The options used are not supported. Check that your version of Internet Explorer is supported.
//
#define PRTH_E_REQUEST_ERROR             ((DWORD)0x80041202L)

//
// MessageId: PRTH_S_NOT_MODIFIED
//
// MessageText:
//
// The content did not change.
//
#define PRTH_S_NOT_MODIFIED              ((DWORD)0x00041203L)

//
// MessageId: PRTH_E_ACCESS_DENIED
//
// MessageText:
//
// Access is denied. Check that the Default Content Access Account in Windows Search Central Administration is correct, or follow the "Exclude and Include Content" link to add a rule to specify the proper crawling account to access this URL.
//
#define PRTH_E_ACCESS_DENIED             ((DWORD)0x80041205L)

//
// MessageId: PRTH_E_SERVER_ERROR
//
// MessageText:
//
// A server error occurred. Check that the server is available.
//
#define PRTH_E_SERVER_ERROR              ((DWORD)0x80041206L)

//
// MessageId: PRTH_E_NOT_REDIRECTED
//
// MessageText:
//
// You have been redirected to an address that does not exist. Check that the source address is redirecting to an accessible address.
//
#define PRTH_E_NOT_REDIRECTED            ((DWORD)0x80041207L)

//
// MessageId: PRTH_E_BAD_REQUEST
//
// MessageText:
//
// The address appears to be in error. Check that the address is valid.
//
#define PRTH_E_BAD_REQUEST               ((DWORD)0x80041208L)

//
// MessageId: PRTH_E_HTTP_CANNOT_CONNECT
//
// MessageText:
//
// Cannot Connect to the server.  Please make sure the site is accessible.
//
#define PRTH_E_HTTP_CANNOT_CONNECT       ((DWORD)0x80041209L)

//
// MessageId: PRTH_S_ACL_IS_READ_EVERYONE
//
// MessageText:
//
// The Access Control List was successfully changed to be readable by everyone.
//
#define PRTH_S_ACL_IS_READ_EVERYONE      ((DWORD)0x00041210L)

//
// MessageId: PRTH_E_ACL_IS_READ_NONE
//
// MessageText:
//
// The item will not be crawled because the Access Control List allows no one to read the item. Check that this is intended.
//
#define PRTH_E_ACL_IS_READ_NONE          ((DWORD)0x80041211L)

//
// MessageId: PRTH_E_ACL_TOO_BIG
//
// MessageText:
//
// Search cannot crawl the item, because its Access Control List exceeded 64 KB. Check that the item has a valid Access Control List.
//
#define PRTH_E_ACL_TOO_BIG               ((DWORD)0x80041212L)

//
// MessageId: PRTH_S_NOT_ALL_PARTS
//
// MessageText:
//
// Some parts of this document cannot be accessed.
//
#define PRTH_S_NOT_ALL_PARTS             ((DWORD)0x0004121BL)

#ifndef _CIERROR_H_
#define _CIERROR_H_
#ifndef FACILITY_WINDOWS
//
// MessageId: NOT_AN_ERROR1
//
// MessageText:
//
// NOTE:  This dummy error message is necessary to force MC to output
//        the above defines inside the FACILITY_WINDOWS guard instead
//        of leaving it empty.
//
#define NOT_AN_ERROR1                    ((HRESULT)0x00081600L)

#endif // FACILITY_WINDOWS
//
// Range 0x1600-0x1850 is reserved by Content Index.
//
//
// Codes 0x1600-0x164f are reserved for QUERY
//
//
// MessageId: QUERY_E_FAILED
//
// MessageText:
//
// The call failed for an unknown reason.
//
#define QUERY_E_FAILED                   ((HRESULT)0x80041600L)

//
// MessageId: QUERY_E_INVALIDQUERY
//
// MessageText:
//
// The parameter is invalid.
//
#define QUERY_E_INVALIDQUERY             ((HRESULT)0x80041601L)

//
// MessageId: QUERY_E_INVALIDRESTRICTION
//
// MessageText:
//
// The query restriction cannot be parsed.
//
#define QUERY_E_INVALIDRESTRICTION       ((HRESULT)0x80041602L)

//
// MessageId: QUERY_E_INVALIDSORT
//
// MessageText:
//
// An invalid sort order was requested.
//
#define QUERY_E_INVALIDSORT              ((HRESULT)0x80041603L)

//
// MessageId: QUERY_E_INVALIDCATEGORIZE
//
// MessageText:
//
// An invalid categorization order was requested.
//
#define QUERY_E_INVALIDCATEGORIZE        ((HRESULT)0x80041604L)

//
// MessageId: QUERY_E_ALLNOISE
//
// MessageText:
//
// A clause of the query contained only words that are ignored.
//
#define QUERY_E_ALLNOISE                 ((HRESULT)0x80041605L)

//
// MessageId: QUERY_E_TOOCOMPLEX
//
// MessageText:
//
// The query was too complex to be executed.
//
#define QUERY_E_TOOCOMPLEX               ((HRESULT)0x80041606L)

//
// MessageId: QUERY_E_TIMEDOUT
//
// MessageText:
//
// The query exceeded its execution time limit.
//
#define QUERY_E_TIMEDOUT                 ((HRESULT)0x80041607L)

//
// MessageId: QUERY_E_DUPLICATE_OUTPUT_COLUMN
//
// MessageText:
//
// One or more columns in the output column list is a duplicate.
//
#define QUERY_E_DUPLICATE_OUTPUT_COLUMN  ((HRESULT)0x80041608L)

//
// MessageId: QUERY_E_INVALID_OUTPUT_COLUMN
//
// MessageText:
//
// One or more columns in the output column list is not valid.
//
#define QUERY_E_INVALID_OUTPUT_COLUMN    ((HRESULT)0x80041609L)

//
// MessageId: QUERY_E_INVALID_DIRECTORY
//
// MessageText:
//
// The directory name is invalid.
//
#define QUERY_E_INVALID_DIRECTORY        ((HRESULT)0x8004160AL)

//
// MessageId: QUERY_E_DIR_ON_REMOVABLE_DRIVE
//
// MessageText:
//
// The specified directory is on a removable medium.
//
#define QUERY_E_DIR_ON_REMOVABLE_DRIVE   ((HRESULT)0x8004160BL)

//
// MessageId: QUERY_S_NO_QUERY
//
// MessageText:
//
// The index is still being crawled, but queries are no longer allowed.
//
#define QUERY_S_NO_QUERY                 ((HRESULT)0x8004160CL)

//
// MessageId: QUERY_E_ALLNOISE_AND_NO_RELDOC
//
// MessageText:
//
// The relevant specified documents were not found, because a clause of the query contained only words that are ignored.
//
#define QUERY_E_ALLNOISE_AND_NO_RELDOC   ((HRESULT)0x8004160DL)

//
// MessageId: QUERY_E_NO_RELDOC
//
// MessageText:
//
// None of the relevant documents specified in the query can be found.
//
#define QUERY_E_NO_RELDOC                ((HRESULT)0x8004160EL)

//
// MessageId: QUERY_E_ALLNOISE_AND_NO_RELPROP
//
// MessageText:
//
// No information was found in the relevant documents with the specified properties, because a clause of the query contained only words that are ignored.
//
#define QUERY_E_ALLNOISE_AND_NO_RELPROP  ((HRESULT)0x8004160FL)

//
// MessageId: QUERY_E_NO_RELPROP
//
// MessageText:
//
// No relevant information is found in the relevant documents with the specified properties for this query.
//
#define QUERY_E_NO_RELPROP               ((HRESULT)0x80041610L)

//
// MessageId: QUERY_E_REPEATED_RELDOC
//
// MessageText:
//
// The same relevant document is specified multiple times.
//
#define QUERY_E_REPEATED_RELDOC          ((HRESULT)0x80041611L)

//
// MessageId: QUERY_E_RELDOC_SYNTAX_NOT_SUPPORTED
//
// MessageText:
//
// The reldoc specification is not supported.
//
#define QUERY_E_RELDOC_SYNTAX_NOT_SUPPORTED ((HRESULT)0x80041612L)

//
// MessageId: QUERY_E_INVALID_DOCUMENT_IDENTIFIER
//
// MessageText:
//
// The specified document identifier is not valid.
//
#define QUERY_E_INVALID_DOCUMENT_IDENTIFIER ((HRESULT)0x80041613L)

//
// MessageId: QUERY_E_INCORRECT_VERSION
//
// MessageText:
//
// The server is running an older version of software that cannot handle this query.
//
#define QUERY_E_INCORRECT_VERSION        ((HRESULT)0x80041614L)

//
// MessageId: QUERY_E_INVALIDSCOPE_COALESCE
//
// MessageText:
//
// Scopes should be the same in all of the components of a coalesce query
//
#define QUERY_E_INVALIDSCOPE_COALESCE    ((HRESULT)0x80041615L)

//
// MessageId: QUERY_E_INVALIDSORT_COALESCE
//
// MessageText:
//
// Order by list should be same in all the components of a coalesce query.
//
#define QUERY_E_INVALIDSORT_COALESCE     ((HRESULT)0x80041616L)

//
// MessageId: QUERY_E_INVALIDCOALESCE
//
// MessageText:
//
// Coalesce is either not used properly or this form is not currently supported. Check for possible mixing of grouping with coalesce
//
#define QUERY_E_INVALIDCOALESCE          ((HRESULT)0x80041617L)

//
// MessageId: QUERY_E_UPGRADEINPROGRESS
//
// MessageText:
//
// Queries are disabled because the index is being updated. Try your query again in a few minutes.
//
#define QUERY_E_UPGRADEINPROGRESS        ((HRESULT)0x80041618L)

//
// MessageId: QUERY_E_AGGREGATE_NOT_SUPPORTED
//
// MessageText:
//
// Aggregate is not supported.
//
#define QUERY_E_AGGREGATE_NOT_SUPPORTED  ((HRESULT)0x80041619L)

//
// MessageId: QUERY_E_TOP_LEVEL_IN_GROUP
//
// MessageText:
//
// The top level group does not support ORDER IN GROUP since there is no parent group.
//
#define QUERY_E_TOP_LEVEL_IN_GROUP       ((HRESULT)0x8004161AL)

//
// MessageId: QUERY_E_DUPLICATE_RANGE_NAME
//
// MessageText:
//
// Each name for a ranged label must be unique.
//
#define QUERY_E_DUPLICATE_RANGE_NAME     ((HRESULT)0x8004161BL)

//
// Codes 0x1650-0x167f are reserved for QUERYLIB (see querylib\include\qutilerr.mc)
//
//
// MessageId: QPLIST_E_CANT_OPEN_FILE
//
// MessageText:
//
// The file cannot be opened.
//
#define QPLIST_E_CANT_OPEN_FILE          ((HRESULT)0x80041651L)

//
// MessageId: QPLIST_E_READ_ERROR
//
// MessageText:
//
// Read error in file.
//
#define QPLIST_E_READ_ERROR              ((HRESULT)0x80041652L)

//
// MessageId: QPLIST_E_EXPECTING_NAME
//
// MessageText:
//
// Expecting property name.
//
#define QPLIST_E_EXPECTING_NAME          ((HRESULT)0x80041653L)

//
// MessageId: QPLIST_E_EXPECTING_TYPE
//
// MessageText:
//
// Expecting type specifier.
//
#define QPLIST_E_EXPECTING_TYPE          ((HRESULT)0x80041654L)

//
// MessageId: QPLIST_E_UNRECOGNIZED_TYPE
//
// MessageText:
//
// Unrecognized type.
//
#define QPLIST_E_UNRECOGNIZED_TYPE       ((HRESULT)0x80041655L)

//
// MessageId: QPLIST_E_EXPECTING_INTEGER
//
// MessageText:
//
// Expecting integer.
//
#define QPLIST_E_EXPECTING_INTEGER       ((HRESULT)0x80041656L)

//
// MessageId: QPLIST_E_EXPECTING_CLOSE_PAREN
//
// MessageText:
//
// Expecting closing parenthesis.
//
#define QPLIST_E_EXPECTING_CLOSE_PAREN   ((HRESULT)0x80041657L)

//
// MessageId: QPLIST_E_EXPECTING_GUID
//
// MessageText:
//
// Expecting GUID.
//
#define QPLIST_E_EXPECTING_GUID          ((HRESULT)0x80041658L)

//
// MessageId: QPLIST_E_BAD_GUID
//
// MessageText:
//
// Invalid guid.
//
#define QPLIST_E_BAD_GUID                ((HRESULT)0x80041659L)

//
// MessageId: QPLIST_E_EXPECTING_PROP_SPEC
//
// MessageText:
//
// Expecting property specifier.
//
#define QPLIST_E_EXPECTING_PROP_SPEC     ((HRESULT)0x8004165AL)

//
// MessageId: QPLIST_E_CANT_SET_PROPERTY
//
// MessageText:
//
// Failed to set property name.
//
#define QPLIST_E_CANT_SET_PROPERTY       ((HRESULT)0x8004165BL)

//
// MessageId: QPLIST_E_DUPLICATE
//
// MessageText:
//
// Duplicate property name.
//
#define QPLIST_E_DUPLICATE               ((HRESULT)0x8004165CL)

//
// MessageId: QPLIST_E_VECTORBYREF_USED_ALONE
//
// MessageText:
//
// DBTYPE_VECTOR or DBTYPE_BYREF used alone.
//
#define QPLIST_E_VECTORBYREF_USED_ALONE  ((HRESULT)0x8004165DL)

//
// MessageId: QPLIST_E_BYREF_USED_WITHOUT_PTRTYPE
//
// MessageText:
//
// DBTYPE_BYREF must be used with DBTYPE_STR, DBTYPE_WSTR, DBTYPE_GUID
//  or DBTYPE_UI1 types.
//
#define QPLIST_E_BYREF_USED_WITHOUT_PTRTYPE ((HRESULT)0x8004165EL)

//
// MessageId: QPARSE_E_UNEXPECTED_EOS
//
// MessageText:
//
// Unexpected end of string.
//
#define QPARSE_E_UNEXPECTED_EOS          ((HRESULT)0x80041672L)

//
// Filter error codes
//
//
// MessageId: FILTER_E_TOO_BIG
//
// MessageText:
//
// The file is too large to filter.
//
#define FILTER_E_TOO_BIG                 ((HRESULT)0x80041730L)

//
// MessageId: FILTER_S_PARTIAL_CONTENTSCAN_IMMEDIATE
//
// MessageText:
//
// A partial content scan of the disk must be scheduled for immediate execution.
//
#define FILTER_S_PARTIAL_CONTENTSCAN_IMMEDIATE ((HRESULT)0x00041731L)

//
// MessageId: FILTER_S_FULL_CONTENTSCAN_IMMEDIATE
//
// MessageText:
//
// A full content scan of the disk must be scheduled for immediate execution.
//
#define FILTER_S_FULL_CONTENTSCAN_IMMEDIATE ((HRESULT)0x00041732L)

//
// MessageId: FILTER_S_CONTENTSCAN_DELAYED
//
// MessageText:
//
// A content scan of the disk needs to be scheduled for execution later.
//
#define FILTER_S_CONTENTSCAN_DELAYED     ((HRESULT)0x00041733L)

//
// MessageId: FILTER_E_CONTENTINDEXCORRUPT
//
// MessageText:
//
// The content index cannot be read. A content scan will be scheduled after chkdsk or autochk is run.
//
#define FILTER_E_CONTENTINDEXCORRUPT     ((HRESULT)0xC0041734L)

//
// MessageId: FILTER_S_DISK_FULL
//
// MessageText:
//
// The disk is almost full.
//
#define FILTER_S_DISK_FULL               ((HRESULT)0x00041735L)

//
// MessageId: FILTER_E_ALREADY_OPEN
//
// MessageText:
//
// The file cannot be opened, because another file is already open.
//
#define FILTER_E_ALREADY_OPEN            ((HRESULT)0x80041736L)

//
// MessageId: FILTER_E_UNREACHABLE
//
// MessageText:
//
// The file is not reachable.
//
#define FILTER_E_UNREACHABLE             ((HRESULT)0x80041737L)

//
// MessageId: FILTER_E_IN_USE
//
// MessageText:
//
// The document is in use by another process.
//
#define FILTER_E_IN_USE                  ((HRESULT)0x80041738L)

//
// MessageId: FILTER_E_NOT_OPEN
//
// MessageText:
//
// The document is not open.
//
#define FILTER_E_NOT_OPEN                ((HRESULT)0x80041739L)

//
// MessageId: FILTER_S_NO_PROPSETS
//
// MessageText:
//
// The document has no property sets.
//
#define FILTER_S_NO_PROPSETS             ((HRESULT)0x0004173AL)

//
// MessageId: FILTER_E_NO_SUCH_PROPERTY
//
// MessageText:
//
// There is no property with the given GUID.
//
#define FILTER_E_NO_SUCH_PROPERTY        ((HRESULT)0x8004173BL)

//
// MessageId: FILTER_S_NO_SECURITY_DESCRIPTOR
//
// MessageText:
//
// The document has no security descriptor.
//
#define FILTER_S_NO_SECURITY_DESCRIPTOR  ((HRESULT)0x0004173CL)

//
// MessageId: FILTER_E_OFFLINE
//
// MessageText:
//
// The document is offline.
//
#define FILTER_E_OFFLINE                 ((HRESULT)0x8004173DL)

//
// MessageId: FILTER_E_PARTIALLY_FILTERED
//
// MessageText:
//
// The document was too large to filter completely.  Portions of the document were not emitted.
//
#define FILTER_E_PARTIALLY_FILTERED      ((HRESULT)0x8004173EL)

//
// Word breaker error codes
//
//
// MessageId: WBREAK_E_END_OF_TEXT
//
// MessageText:
//
// End of text was reached in the text source.
//
#define WBREAK_E_END_OF_TEXT             ((HRESULT)0x80041780L)

//
// MessageId: LANGUAGE_S_LARGE_WORD
//
// MessageText:
//
// The word exceeds the maximum length, and may be truncated by the word sink.
//
#define LANGUAGE_S_LARGE_WORD            ((HRESULT)0x00041781L)

//
// MessageId: WBREAK_E_QUERY_ONLY
//
// MessageText:
//
// This feature is only available in query mode.
//
#define WBREAK_E_QUERY_ONLY              ((HRESULT)0x80041782L)

//
// MessageId: WBREAK_E_BUFFER_TOO_SMALL
//
// MessageText:
//
// The buffer is too small to hold the composed phrase.
//
#define WBREAK_E_BUFFER_TOO_SMALL        ((HRESULT)0x80041783L)

//
// MessageId: LANGUAGE_E_DATABASE_NOT_FOUND
//
// MessageText:
//
// The language database/cache file cannot be found.
//
#define LANGUAGE_E_DATABASE_NOT_FOUND    ((HRESULT)0x80041784L)

//
// MessageId: WBREAK_E_INIT_FAILED
//
// MessageText:
//
// Initialization of the word breaker failed.
//
#define WBREAK_E_INIT_FAILED             ((HRESULT)0x80041785L)

//
// MessageId: PSINK_E_QUERY_ONLY
//
// MessageText:
//
// Feature only available in query mode.
//
#define PSINK_E_QUERY_ONLY               ((HRESULT)0x80041790L)

//
// MessageId: PSINK_E_INDEX_ONLY
//
// MessageText:
//
// This feature is only available in index mode.
//
#define PSINK_E_INDEX_ONLY               ((HRESULT)0x80041791L)

//
// MessageId: PSINK_E_LARGE_ATTACHMENT
//
// MessageText:
//
// The attachment type is beyond the valid range.
//
#define PSINK_E_LARGE_ATTACHMENT         ((HRESULT)0x80041792L)

//
// MessageId: PSINK_S_LARGE_WORD
//
// MessageText:
//
// The word exceeds the maximum length, and may be truncated by the phrase sink.
//
#define PSINK_S_LARGE_WORD               ((HRESULT)0x00041793L)

//
// Content Index Framework Error Codes
//
//
// MessageId: CI_CORRUPT_DATABASE
//
// MessageText:
//
// The content index database is corrupt.
//
#define CI_CORRUPT_DATABASE              ((HRESULT)0xC0041800L)

//
// MessageId: CI_CORRUPT_CATALOG
//
// MessageText:
//
// The content index catalog is corrupt.
//
#define CI_CORRUPT_CATALOG               ((HRESULT)0xC0041801L)

//
// MessageId: CI_INVALID_PARTITION
//
// MessageText:
//
// The content index partition is invalid.
//
#define CI_INVALID_PARTITION             ((HRESULT)0xC0041802L)

//
// MessageId: CI_INVALID_PRIORITY
//
// MessageText:
//
// The priority is invalid.
//
#define CI_INVALID_PRIORITY              ((HRESULT)0xC0041803L)

//
// MessageId: CI_NO_STARTING_KEY
//
// MessageText:
//
// There is no starting key.
//
#define CI_NO_STARTING_KEY               ((HRESULT)0xC0041804L)

//
// MessageId: CI_OUT_OF_INDEX_IDS
//
// MessageText:
//
// The content index is out of index ids.
//
#define CI_OUT_OF_INDEX_IDS              ((HRESULT)0xC0041805L)

//
// MessageId: CI_NO_CATALOG
//
// MessageText:
//
// There is no index.
//
#define CI_NO_CATALOG                    ((HRESULT)0xC0041806L)

//
// MessageId: CI_CORRUPT_FILTER_BUFFER
//
// MessageText:
//
// The filter buffer cannot be read.
//
#define CI_CORRUPT_FILTER_BUFFER         ((HRESULT)0xC0041807L)

//
// MessageId: CI_INVALID_INDEX
//
// MessageText:
//
// The index is invalid.
//
#define CI_INVALID_INDEX                 ((HRESULT)0xC0041808L)

//
// MessageId: CI_PROPSTORE_INCONSISTENCY
//
// MessageText:
//
// Inconsistency was detected in the property store.
//
#define CI_PROPSTORE_INCONSISTENCY       ((HRESULT)0xC0041809L)

//
// MessageId: CI_E_ALREADY_INITIALIZED
//
// MessageText:
//
// The object is already initialized.
//
#define CI_E_ALREADY_INITIALIZED         ((HRESULT)0x8004180AL)

//
// MessageId: CI_E_NOT_INITIALIZED
//
// MessageText:
//
// The object is not initialized.
//
#define CI_E_NOT_INITIALIZED             ((HRESULT)0x8004180BL)

//
// MessageId: CI_E_BUFFERTOOSMALL
//
// MessageText:
//
// The buffer is too small.
//
#define CI_E_BUFFERTOOSMALL              ((HRESULT)0x8004180CL)

//
// MessageId: CI_E_PROPERTY_NOT_CACHED
//
// MessageText:
//
// The given property is not cached.
//
#define CI_E_PROPERTY_NOT_CACHED         ((HRESULT)0x8004180DL)

//
// MessageId: CI_S_WORKID_DELETED
//
// MessageText:
//
// The workid is deleted.
//
#define CI_S_WORKID_DELETED              ((HRESULT)0x0004180EL)

//
// MessageId: CI_E_INVALID_STATE
//
// MessageText:
//
// The object is not in a valid state.
//
#define CI_E_INVALID_STATE               ((HRESULT)0x8004180FL)

//
// MessageId: CI_E_FILTERING_DISABLED
//
// MessageText:
//
// Filtering is disabled for this content index.
//
#define CI_E_FILTERING_DISABLED          ((HRESULT)0x80041810L)

//
// MessageId: CI_E_DISK_FULL
//
// MessageText:
//
// The disk is full and the specified operation cannot be completed.
//
#define CI_E_DISK_FULL                   ((HRESULT)0x80041811L)

//
// MessageId: CI_E_SHUTDOWN
//
// MessageText:
//
// The content index service was stopped.
//
#define CI_E_SHUTDOWN                    ((HRESULT)0x80041812L)

//
// MessageId: CI_E_WORKID_NOTVALID
//
// MessageText:
//
// The workid is not valid.
//
#define CI_E_WORKID_NOTVALID             ((HRESULT)0x80041813L)

//
// MessageId: CI_S_END_OF_ENUMERATION
//
// MessageText:
//
// There are no more documents to enumerate.
//
#define CI_S_END_OF_ENUMERATION          ((HRESULT)0x00041814L)    

//
// MessageId: CI_E_NOT_FOUND
//
// MessageText:
//
// The object was not found.
//
#define CI_E_NOT_FOUND                   ((HRESULT)0x80041815L)

//
// MessageId: CI_E_USE_DEFAULT_PID
//
// MessageText:
//
// The passed-in property id is not supported.
//
#define CI_E_USE_DEFAULT_PID             ((HRESULT)0x80041816L)

//
// MessageId: CI_E_DUPLICATE_NOTIFICATION
//
// MessageText:
//
// There were two alerts for the same workid.
//
#define CI_E_DUPLICATE_NOTIFICATION      ((HRESULT)0x80041817L)

//
// MessageId: CI_E_UPDATES_DISABLED
//
// MessageText:
//
// A document update was rejected because updates were disabled.
//
#define CI_E_UPDATES_DISABLED            ((HRESULT)0x80041818L)

//
// MessageId: CI_E_INVALID_FLAGS_COMBINATION
//
// MessageText:
//
// The combination of flags specified is invalid.
//
#define CI_E_INVALID_FLAGS_COMBINATION   ((HRESULT)0x80041819L)

//
// MessageId: CI_E_OUTOFSEQ_INCREMENT_DATA
//
// MessageText:
//
// The incremental data given to Load is not valid. It may be out of sequence.
//
#define CI_E_OUTOFSEQ_INCREMENT_DATA     ((HRESULT)0x8004181AL)

//
// MessageId: CI_E_SHARING_VIOLATION
//
// MessageText:
//
// A sharing or locking violation caused a failure.
//
#define CI_E_SHARING_VIOLATION           ((HRESULT)0x8004181BL)

//
// MessageId: CI_E_LOGON_FAILURE
//
// MessageText:
//
// A logon permission violation caused a failure.
//
#define CI_E_LOGON_FAILURE               ((HRESULT)0x8004181CL)

//
// MessageId: CI_E_NO_CATALOG
//
// MessageText:
//
// The index does not exist or is currently unavailable. Try again later. If this problem persists, contact the system administrator.
//
#define CI_E_NO_CATALOG                  ((HRESULT)0x8004181DL)

//
// MessageId: CI_E_STRANGE_PAGEORSECTOR_SIZE
//
// MessageText:
//
// Page size is not an integral multiple of the sector size of the volume where the index is located.
//
#define CI_E_STRANGE_PAGEORSECTOR_SIZE   ((HRESULT)0x8004181EL)

//
// MessageId: CI_E_TIMEOUT
//
// MessageText:
//
// The service is too busy.
//
#define CI_E_TIMEOUT                     ((HRESULT)0x8004181FL)

//
// MessageId: CI_E_NOT_RUNNING
//
// MessageText:
//
// The service is not running.
//
#define CI_E_NOT_RUNNING                 ((HRESULT)0x80041820L)

//
// MessageId: CI_INCORRECT_VERSION
//
// MessageText:
//
// The content index data on disk is for the wrong version.
//
#define CI_INCORRECT_VERSION             ((HRESULT)0xC0041821L)

//
// MessageId: CI_E_ENUMERATION_STARTED
//
// MessageText:
//
// Enumeration was already started for this query.
//
#define CI_E_ENUMERATION_STARTED         ((HRESULT)0xC0041822L)

//
// MessageId: CI_E_PROPERTY_TOOLARGE
//
// MessageText:
//
// The specified variable length property is too large for the property cache.
//
#define CI_E_PROPERTY_TOOLARGE           ((HRESULT)0xC0041823L)

//
// MessageId: CI_E_CLIENT_FILTER_ABORT
//
// MessageText:
//
// The filtering of the object was stopped by the client.
//
#define CI_E_CLIENT_FILTER_ABORT         ((HRESULT)0xC0041824L)

//
// MessageId: CI_S_NO_DOCSTORE
//
// MessageText:
//
// Administrative connections from client were not associated with a document store.
//
#define CI_S_NO_DOCSTORE                 ((HRESULT)0x00041825L)

//
// MessageId: CI_S_CAT_STOPPED
//
// MessageText:
//
// The index has been stopped.
//
#define CI_S_CAT_STOPPED                 ((HRESULT)0x00041826L)

//
// MessageId: CI_E_CARDINALITY_MISMATCH
//
// MessageText:
//
// The cardinality of machine, indexes, and scopes is mismatched.
//
#define CI_E_CARDINALITY_MISMATCH        ((HRESULT)0x80041827L)

//
// MessageId: CI_E_CONFIG_DISK_FULL
//
// MessageText:
//
// The disk has reached its configured space limit.
//
#define CI_E_CONFIG_DISK_FULL            ((HRESULT)0x80041828L)

//
// MessageId: CI_S_NEW_AUXMETADATA
//
// MessageText:
//
// This is a new entry in the auxiliary metadata storage.
//
#define CI_S_NEW_AUXMETADATA             ((HRESULT)0x00041829L)

//
// MessageId: CI_E_NO_AUXMETADATA
//
// MessageText:
//
// The property store is not configured to handle auxiliary metadata storage.
//
#define CI_E_NO_AUXMETADATA              ((HRESULT)0x8004182AL)

//
// MessageId: CI_S_CLIENT_REQUESTED_ABORT
//
// MessageText:
//
// The client requested that the document be stopped.
//
#define CI_S_CLIENT_REQUESTED_ABORT      ((HRESULT)0x0004182BL)

//
// MessageId: CI_S_RETRY_DOCUMENT
//
// MessageText:
//
// The client can now try to crawl a document.
//
#define CI_S_RETRY_DOCUMENT              ((HRESULT)0x0004182CL)

//
// MessageId: CI_E_CORRUPT_FWIDX
//
// MessageText:
//
// The forward index cannot be read.
//
#define CI_E_CORRUPT_FWIDX               ((HRESULT)0xC004182DL)

//
// MessageId: CI_E_DIACRITIC_SETTINGS_DIFFER
//
// MessageText:
//
// Catalog was created with different diacritic settings.
//
#define CI_E_DIACRITIC_SETTINGS_DIFFER   ((HRESULT)0xC004182EL)

//
// MessageId: CI_E_INVALID_CATALOG_LIST_VERSION
//
// MessageText:
//
// The specified index list version is outdated or invalid.
//
#define CI_E_INVALID_CATALOG_LIST_VERSION ((HRESULT)0x8004182FL)

//
// MessageId: CI_S_CATALOG_RESET
//
// MessageText:
//
// The catalog was reset due to diacritic difference or corruption.
//
#define CI_S_CATALOG_RESET               ((HRESULT)0x00041830L)

//
// MessageId: CI_E_NO_CATALOG_MANAGER
//
// MessageText:
//
// The catalog manager does not exist.
//
#define CI_E_NO_CATALOG_MANAGER          ((HRESULT)0xC0041831L)

//
// MessageId: CI_E_INCONSISTENT_TRANSACTION
//
// MessageText:
//
// The transaction in catalog is found as inconsistent.
//
#define CI_E_INCONSISTENT_TRANSACTION    ((HRESULT)0xC0041832L)

//
// MessageId: CI_E_PROTECTED_CATALOG_NOT_AVAILABLE
//
// MessageText:
//
// Protected catalog for the user is not available due to key drop status.
//
#define CI_E_PROTECTED_CATALOG_NOT_AVAILABLE ((HRESULT)0xC0041833L)

//
// MessageId: CI_E_NO_PROTECTED_USER
//
// MessageText:
//
// The system only stores index entries in a protected catalog, but no protected user currently exists.
//
#define CI_E_NO_PROTECTED_USER           ((HRESULT)0xC0041834L)

//
// MessageId: CI_E_MULTIPLE_PROTECTED_USERS_UNSUPPORTED
//
// MessageText:
//
// Content indexer only currently supports having a single protected user per device, but multiple protected users were present.
//
#define CI_E_MULTIPLE_PROTECTED_USERS_UNSUPPORTED ((HRESULT)0xC0041835L)

//
// MessageId: CI_E_PROTECTED_CATALOG_SID_MISMATCH
//
// MessageText:
//
// The system detected a SID owner for a protected item that did not match the SID owner of the protected catalog for the item.
//
#define CI_E_PROTECTED_CATALOG_SID_MISMATCH ((HRESULT)0xC0041836L)

//
// MessageId: CI_E_PROTECTED_CATALOG_NON_INTERACTIVE_USER
//
// MessageText:
//
// On a system with a protected catalog, indexer only supports queries from the interactive user account.
//
#define CI_E_PROTECTED_CATALOG_NON_INTERACTIVE_USER ((HRESULT)0xC0041837L)

//
// MessageId: CI_DATABASE_DECRYPTION_FAILED
//
// MessageText:
//
// Failed to decrypt an encrypted database file.
//
#define CI_DATABASE_DECRYPTION_FAILED    ((HRESULT)0xC0041838L)

//
// MessageId: CI_DATABASE_ENCRYPTION_FAILED
//
// MessageText:
//
// Failed to encrypt an existing database file during first encryption upgrade for the device.
//
#define CI_DATABASE_ENCRYPTION_FAILED    ((HRESULT)0xC0041839L)

#endif // _CIERROR_H_
#ifndef _FILTERR_H_
#define _FILTERR_H_
#ifndef FACILITY_WINDOWS
//
// MessageId: NOT_AN_ERROR
//
// MessageText:
//
// NOTE:  This dummy error message is necessary to force MC to output
//        the above defines inside the FACILITY_WINDOWS guard instead
//        of leaving it empty.
//
#define NOT_AN_ERROR                     ((HRESULT)0x00080000L)

#endif // FACILITY_WINDOWS
//
// Codes 0x1700-0x172F are reserved for FILTER
//
//
// MessageId: FILTER_E_END_OF_CHUNKS
//
// MessageText:
//
// There are no more chunks of text available in the object.
//
#define FILTER_E_END_OF_CHUNKS           ((HRESULT)0x80041700L)

//
// MessageId: FILTER_E_NO_MORE_TEXT
//
// MessageText:
//
// There is no more text available in the chunk.
//
#define FILTER_E_NO_MORE_TEXT            ((HRESULT)0x80041701L)

//
// MessageId: FILTER_E_NO_MORE_VALUES
//
// MessageText:
//
// There are no more property values available in the chunk.
//
#define FILTER_E_NO_MORE_VALUES          ((HRESULT)0x80041702L)

//
// MessageId: FILTER_E_ACCESS
//
// MessageText:
//
// The object cannot be accessed.
//
#define FILTER_E_ACCESS                  ((HRESULT)0x80041703L)

//
// MessageId: FILTER_W_MONIKER_CLIPPED
//
// MessageText:
//
// The moniker doesn't cover the entire region.
//
#define FILTER_W_MONIKER_CLIPPED         ((HRESULT)0x00041704L)

//
// MessageId: FILTER_E_NO_TEXT
//
// MessageText:
//
// There is no text in the current chunk.
//
#define FILTER_E_NO_TEXT                 ((HRESULT)0x80041705L)

//
// MessageId: FILTER_E_NO_VALUES
//
// MessageText:
//
// There are no values in the current chunk.
//
#define FILTER_E_NO_VALUES               ((HRESULT)0x80041706L)

//
// MessageId: FILTER_E_EMBEDDING_UNAVAILABLE
//
// MessageText:
//
// The Ifilter for the embedded object cannot be bound.
//
#define FILTER_E_EMBEDDING_UNAVAILABLE   ((HRESULT)0x80041707L)

//
// MessageId: FILTER_E_LINK_UNAVAILABLE
//
// MessageText:
//
// The Ifilter for the linked object cannot be bound.
//
#define FILTER_E_LINK_UNAVAILABLE        ((HRESULT)0x80041708L)

//
// MessageId: FILTER_S_LAST_TEXT
//
// MessageText:
//
// This is the last text in the current chunk.
//
#define FILTER_S_LAST_TEXT               ((HRESULT)0x00041709L)

//
// MessageId: FILTER_S_LAST_VALUES
//
// MessageText:
//
// This is the last value in the current chunk.
//
#define FILTER_S_LAST_VALUES             ((HRESULT)0x0004170AL)

//
// MessageId: FILTER_E_PASSWORD
//
// MessageText:
//
// The file was not filtered due to password protection.
//
#define FILTER_E_PASSWORD                ((HRESULT)0x8004170BL)

//
// MessageId: FILTER_E_UNKNOWNFORMAT
//
// MessageText:
//
// The document format is not recognized by the filter.
//
#define FILTER_E_UNKNOWNFORMAT           ((HRESULT)0x8004170CL)

//
// MessageId: FILTER_E_NO_IMAGE_FRAMES
//
// MessageText:
//
// No image frames in current chunk.
//
#define FILTER_E_NO_IMAGE_FRAMES         ((HRESULT)0x8004170EL)

#endif // _FILTERR_H_
/*++ BUILD Version: 0001    // Increment this if a change has global effects

Microsoft Windows
Copyright (c) Microsoft Corporation, 1994 - 2002.

Module Name:

    cimsg.mc

Abstract:

    This file contains the message definitions for the content index.

Author:

    DwightKr 06-Jul-1994

Revision History:

Notes:     MessageIds in the range 0x0001 - 0x1000 are categories
                                   0x1001 - 0x1FFF are events

           A .mc file is compiled by the MC tool to generate a .h file and
           a .rc (resource compiler script) file.

 The LanguageNames keyword defines the set of names that are allowed
 as the value of the Language keyword in the message definition. The
 set is delimited by left and right parentheses. Associated with each
 language name is a number and a file name that are used to name the
 generated resource file that contains the messages for that
 language. The number corresponds to the language identifier to use
 in the resource table. The number is separated from the file name
 with a colon.

--*/
//
// messages 1001 - 1FFF come from query.dll.
//
// Not used in MSFTESQL
//
// MessageId: MSG_CI_MASTER_MERGE_STARTED
//
// MessageText:
//
// %1A master merge has started for catalog %2.
//
#define MSG_CI_MASTER_MERGE_STARTED      ((HRESULT)0x40001006L)

//
// MessageId: MSG_CI_MASTER_MERGE_COMPLETED
//
// MessageText:
//
// %1A master merge has completed for catalog %2.
//
#define MSG_CI_MASTER_MERGE_COMPLETED    ((HRESULT)0x40001007L)

//
// MessageId: MSG_CI_MASTER_MERGE_ABORTED
//
// MessageText:
//
// %1A master merge has been paused for catalog %2 due to error %3.
// It will be rescheduled later.
//
#define MSG_CI_MASTER_MERGE_ABORTED      ((HRESULT)0x40001008L)

//
// MessageId: MSG_CI_MASTER_MERGE_CANT_START
//
// MessageText:
//
// %1A master merge cannot be started for catalog %2 due to error %3.
//
#define MSG_CI_MASTER_MERGE_CANT_START   ((HRESULT)0xC0001009L)

//
// MessageId: MSG_CI_MASTER_MERGE_CANT_RESTART
//
// MessageText:
//
// %1A master merge cannot be re-started for catalog %2 due to error %3.
//
#define MSG_CI_MASTER_MERGE_CANT_RESTART ((HRESULT)0xC000100AL)

//
// MessageId: MSG_CI_MASTER_MERGE_RESTARTED
//
// MessageText:
//
// %1A master merge has restarted for catalog %2.
//
#define MSG_CI_MASTER_MERGE_RESTARTED    ((HRESULT)0x40001019L)

//
// MessageId: MSG_CI_CORRUPT_INDEX_COMPONENT
//
// MessageText:
//
// An index corruption was detected in component %2 in catalog %3.%1
//
#define MSG_CI_CORRUPT_INDEX_COMPONENT   ((HRESULT)0x4000102AL)

//
// MessageId: MSG_CI_MASTER_MERGE_ABORTED_LOW_DISK
//
// MessageText:
//
// %1A master merge has been paused for catalog %2 due to low disk space.
// The merge will be rescheduled later.  Please free some disk space for indexing to continue.
//
#define MSG_CI_MASTER_MERGE_ABORTED_LOW_DISK ((HRESULT)0x40001043L)

//
// MessageId: MSG_CI_MASTER_MERGE_REASON_EXTERNAL
//
// MessageText:
//
// %1Catalog: %2. A master merge was started due to an external request.
//
#define MSG_CI_MASTER_MERGE_REASON_EXTERNAL ((HRESULT)0x40001044L)

//
// MessageId: MSG_CI_MASTER_MERGE_REASON_INDEX_LIMIT
//
// MessageText:
//
// %1Catalog: %2.
// A master merge was started because the catalog reached the maximum number of indexes on the last level (%3).
//
#define MSG_CI_MASTER_MERGE_REASON_INDEX_LIMIT ((HRESULT)0x40001045L)

// Not used in MSFTESQL
//
// MessageId: MSG_CI_MASTER_MERGE_REASON_EXPECTED_DOCS
//
// MessageText:
//
// %1Catalog: %2.
// A master merge was started because the expected number of documents in the catalog (%3) were indexed.
//
#define MSG_CI_MASTER_MERGE_REASON_EXPECTED_DOCS ((HRESULT)0x40001046L)

//
// This message is provided for future master merge reasons so that if some other reason appears and
// the change is made in a hotfix/service pack no localization is necessary.
//
//
// MessageId: MSG_CI_MASTER_MERGE_REASON_NUMBER
//
// MessageText:
//
// %1Catalog: %2. The master merge was started because of internal reason number %3.
//
#define MSG_CI_MASTER_MERGE_REASON_NUMBER ((HRESULT)0x40001047L)

//
// This message is provided for future master merge reasons so that if some other reason appears and
// the change is made in a hotfix/service pack no localization is necessary.
//
//
// MessageId: MSG_CI_CREATE_SEVER_ITEM_FAILED
//
// MessageText:
//
// %1 Unable to create the query engine's first request item due to error %2. It's possible that the MSFTESQL service account is invalid or the password has expired.
//
#define MSG_CI_CREATE_SEVER_ITEM_FAILED  ((HRESULT)0x80001048L)

//+-------------------------------------------------------------------------
//
//  Microsoft Full-Text SQL Parser
//  Copyright (C) Microsoft Corporation, 1997 - 1999
//
//  File:       parserr.mc
//
//  Contents:   
//
//  History:    10-23-97    Briants         Created
//
//--------------------------------------------------------------------------

#pragma once

// ****************************************************************************
//                 PLEASE DO NOT MODIFY PARSERR.h DIRECTLY
//                  Changes need to be made in PARSERR.mc
// ****************************************************************************

#ifndef FACILITY_WINDOWS

//
// MessageId: NOT_N_PARSE_ERROR
//
// MessageText:
//
// NOTE:  This dummy error message is necessary to force MC to output
//        the above defines inside the FACILITY_WINDOWS guard instead
//        of leaving it empty.
//
#define NOT_N_PARSE_ERROR                ((HRESULT)0x0008092EL)


#endif // FACILITY_WINDOWS

//--------------------------------------------------------------------------------
//Language-dependent resources (localize)
//--------------------------------------------------------------------------------
//
// messages 0x092e - 0x0992 are for msidxtr.lib
//
//
// MessageId: IDS_MON_DEFAULT_ERROR
//
// MessageText:
//
// Parser Error
//
#define IDS_MON_DEFAULT_ERROR            ((HRESULT)0x0004092FL)

//
// MessageId: IDS_MON_ILLEGAL_PASSTHROUGH
//
// MessageText:
//
// The PASSTHROUGH query is not allowed: '%1'
//
#define IDS_MON_ILLEGAL_PASSTHROUGH      ((HRESULT)0x00040930L)

//
// MessageId: IDS_MON_PARSE_ERR_1_PARAM
//
// MessageText:
//
// The syntax near '%1' is incorrect. SQLSTATE=42000
//
#define IDS_MON_PARSE_ERR_1_PARAM        ((HRESULT)0x00040931L)

//
// MessageId: IDS_MON_PARSE_ERR_2_PARAM
//
// MessageText:
//
// The syntax near '%1' is incorrect.  Expected %2. SQLSTATE=42000
//
#define IDS_MON_PARSE_ERR_2_PARAM        ((HRESULT)0x00040932L)

//
// MessageId: IDS_MON_SEMI_COLON
//
// MessageText:
//
// Multiple statement commands are not supported. SQLSTATE=42000
//
#define IDS_MON_SEMI_COLON               ((HRESULT)0x00040933L)

//
// MessageId: IDS_MON_ORDINAL_OUT_OF_RANGE
//
// MessageText:
//
// ORDER BY ordinal (%1) must be between 1 and %2. SQLSTATE=42000
//
#define IDS_MON_ORDINAL_OUT_OF_RANGE     ((HRESULT)0x00040934L)

//
// MessageId: IDS_MON_VIEW_NOT_DEFINED
//
// MessageText:
//
// View '%1' was not defined in catalog '%2'. SQLSTATE=42S02
//
#define IDS_MON_VIEW_NOT_DEFINED         ((HRESULT)0x00040935L)

//
// MessageId: IDS_MON_COLUMN_NOT_DEFINED
//
// MessageText:
//
// Column '%1' was not defined. SQLSTATE=42S22
//
#define IDS_MON_COLUMN_NOT_DEFINED       ((HRESULT)0x00040936L)

//
// MessageId: IDS_MON_BUILTIN_VIEW
//
// MessageText:
//
// The view name conflicts with a predefined view definition.
//
#define IDS_MON_BUILTIN_VIEW             ((HRESULT)0x00040937L)

//
// MessageId: IDS_MON_OUT_OF_MEMORY
//
// MessageText:
//
// Out of memory
//
#define IDS_MON_OUT_OF_MEMORY            ((HRESULT)0x00040938L)

//
// MessageId: IDS_MON_SELECT_STAR
//
// MessageText:
//
// SELECT * is only allowed on views
//
#define IDS_MON_SELECT_STAR              ((HRESULT)0x00040939L)

//
// MessageId: IDS_MON_OR_NOT
//
// MessageText:
//
// <content search condition> OR NOT <content boolean term> is not allowed
//
#define IDS_MON_OR_NOT                   ((HRESULT)0x0004093AL)

//
// MessageId: IDS_MON_CANNOT_CONVERT
//
// MessageText:
//
// '%1'cannot be converted to type %2
//
#define IDS_MON_CANNOT_CONVERT           ((HRESULT)0x0004093BL)

//
// MessageId: IDS_MON_OUT_OF_RANGE
//
// MessageText:
//
// %1 is out of range for type %2
//
#define IDS_MON_OUT_OF_RANGE             ((HRESULT)0x0004093CL)

//
// MessageId: IDS_MON_RELATIVE_INTERVAL
//
// MessageText:
//
// The specification of <relative interval> must be negative
//
#define IDS_MON_RELATIVE_INTERVAL        ((HRESULT)0x0004093DL)

//
// MessageId: IDS_MON_NOT_COLUMN_OF_VIEW
//
// MessageText:
//
// '%1' is not a column in the view definition
//
#define IDS_MON_NOT_COLUMN_OF_VIEW       ((HRESULT)0x0004093EL)

//
// MessageId: IDS_MON_BUILTIN_PROPERTY
//
// MessageText:
//
// The property name conflicts with a predefined property definition
//
#define IDS_MON_BUILTIN_PROPERTY         ((HRESULT)0x0004093FL)

//
// MessageId: IDS_MON_WEIGHT_OUT_OF_RANGE
//
// MessageText:
//
// Weight value must be between 0.0 and 1.0
//
#define IDS_MON_WEIGHT_OUT_OF_RANGE      ((HRESULT)0x00040940L)

//
// MessageId: IDS_MON_MATCH_STRING
//
// MessageText:
//
// The matches string contains an error.
//
#define IDS_MON_MATCH_STRING             ((HRESULT)0x00040941L)

//
// MessageId: IDS_MON_PROPERTY_NAME_IN_VIEW
//
// MessageText:
//
// The property name cannot be set because it is already being used in a VIEW. SQLSTATE=42000
//
#define IDS_MON_PROPERTY_NAME_IN_VIEW    ((HRESULT)0x00040942L)

//
// MessageId: IDS_MON_VIEW_ALREADY_DEFINED
//
// MessageText:
//
// View '%1' already exists in the index '%2' and cannot be redefined. SQLSTATE=42S01
//
#define IDS_MON_VIEW_ALREADY_DEFINED     ((HRESULT)0x00040943L)

//
// MessageId: IDS_MON_INVALID_CATALOG
//
// MessageText:
//
// The index name '%1' is invalid. SQLSTATE=42000
//
#define IDS_MON_INVALID_CATALOG          ((HRESULT)0x00040944L)

//
// MessageId: IDS_MON_INVALIDSELECT_COALESCE
//
// MessageText:
//
// The select list for the coalesce table is invalid.
//
#define IDS_MON_INVALIDSELECT_COALESCE   ((HRESULT)0x00040945L)

//
// MessageId: IDS_MON_CANNOT_CAST
//
// MessageText:
//
// The literal value cannot cast to the requested type.
//
#define IDS_MON_CANNOT_CAST              ((HRESULT)0x00040946L)

//
// MessageId: IDS_MON_DATE_OUT_OF_RANGE
//
// MessageText:
//
// The relative interval given in the DATEADD function is too large.
//
#define IDS_MON_DATE_OUT_OF_RANGE        ((HRESULT)0x00040947L)

//
// MessageId: IDS_MON_INVALID_IN_GROUP_CLAUSE
//
// MessageText:
//
// The IN GROUP identifier is invalid.
//
#define IDS_MON_INVALID_IN_GROUP_CLAUSE  ((HRESULT)0x00040948L)

