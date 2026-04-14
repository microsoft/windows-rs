/*
 *	NTDSBMSG.H
 *
 *	Windows NT Directory Service Backup/Restore API error codes
 *	Copyright (C) 1996-1998, Microsoft Corporation
 *	
 */

#ifndef _NTDSBMSG_
#define _NTDSBMSG_

//
//	SUCCESS
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
#define FACILITY_NTDSB                   0x800
#define FACILITY_BACKUP                  0x7FF
#define FACILITY_SYSTEM                  0x0


//
// Define the severity codes
//
#define STATUS_SEVERITY_SUCCESS          0x0
#define STATUS_SEVERITY_INFORMATIONAL    0x1
#define STATUS_SEVERITY_WARNING          0x2
#define STATUS_SEVERITY_ERROR            0x3


//
// MessageId: hrNone
//
// MessageText:
//
// The operation was successful
//
#define hrNone                           ((HRESULT)0x00000000L)

//
//	ERRORS
//
//
// MessageId: hrNyi
//
// MessageText:
//
// The function is not yet implemented
//
#define hrNyi                            ((HRESULT)0xC0000001L)

//
//	Backup errors
//
//
// MessageId: hrInvalidParam
//
// MessageText:
//
// The parameter is not valid.
//
#define hrInvalidParam                   ((HRESULT)0xC7FF0001L)

//
// MessageId: hrError
//
// MessageText:
//
// An internal error has occurred.
//
#define hrError                          ((HRESULT)0xC7FF0002L)

//
// MessageId: hrInvalidHandle
//
// MessageText:
//
// The handle is not valid.
//
#define hrInvalidHandle                  ((HRESULT)0xC7FF0003L)

//
// MessageId: hrRestoreInProgress
//
// MessageText:
//
// The Restore process is already in progress.
//
#define hrRestoreInProgress              ((HRESULT)0xC7FF0004L)

//
// MessageId: hrAlreadyOpen
//
// MessageText:
//
// The file specified is already open.
//
#define hrAlreadyOpen                    ((HRESULT)0xC7FF0005L)

//
// MessageId: hrInvalidRecips
//
// MessageText:
//
// The recipients are invalid.
//
#define hrInvalidRecips                  ((HRESULT)0xC7FF0006L)

//
// MessageId: hrCouldNotConnect
//
// MessageText:
//
// Unable to perform the backup. Either you are not connected to the specified backup server
// or the service you are trying to backup is not running.
//
#define hrCouldNotConnect                ((HRESULT)0xC7FF0007L)

//
// MessageId: hrRestoreMapExists
//
// MessageText:
//
// A restore map already exists for the specified component.  You can only specify
// a restore map when performing a full restore.
//
#define hrRestoreMapExists               ((HRESULT)0xC7FF0008L)

//
// MessageId: hrIncrementalBackupDisabled
//
// MessageText:
//
// Another application has modified the specified Windows NT Directory Service database such that any
// subsequent backups will fail. You must perform a full backup to fix this problem.
//
#define hrIncrementalBackupDisabled      ((HRESULT)0xC7FF0009L)

//
// MessageId: hrLogFileNotFound
//
// MessageText:
//
// Unable to perform an incremental backup because a required Windows NT Directory Service database log file could not be found.
//
#define hrLogFileNotFound                ((HRESULT)0xC7FF000AL)

//
// MessageId: hrCircularLogging
//
// MessageText:
//
// The Windows NT Directory Service component specified is configured to use circular database logs.
// It cannot be backed up without a full backup.
//
#define hrCircularLogging                ((HRESULT)0xC7FF000BL)

//
// MessageId: hrNoFullRestore
//
// MessageText:
//
// The databases have not been restored to this machine. You cannot restore an incremental backup
// until a full backup has been restored.
//
#define hrNoFullRestore                  ((HRESULT)0xC7FF000CL)

//
// MessageId: hrCommunicationError
//
// MessageText:
//
// A communications error occurred while attempting to perform a local backup.
//
#define hrCommunicationError             ((HRESULT)0xC7FF000DL)

//
// MessageId: hrFullBackupNotTaken
//
// MessageText:
//
// You must perform a full backup before you can perform an incremental backup.
//
#define hrFullBackupNotTaken             ((HRESULT)0xC7FF000EL)

//
// MessageId: hrMissingExpiryToken
//
// MessageText:
//
// Expiry token is missing. Cannot restore without knowing the expiry information.
//
#define hrMissingExpiryToken             ((HRESULT)0xC7FF000FL)

//
// MessageId: hrUnknownExpiryTokenFormat
//
// MessageText:
//
// Expiry token is in unrecognizable format.
//
#define hrUnknownExpiryTokenFormat       ((HRESULT)0xC7FF0010L)

//
// MessageId: hrContentsExpired
//
// MessageText:
//
// DS Contents in the backup copy are out of date. Try restoring with a more recent copy.
//
#define hrContentsExpired                ((HRESULT)0xC7FF0011L)

#define	hrAlreadyListening	((HRESULT)RPC_S_ALREADY_LISTENING)
//
//	ERRORS
//
//
// SYSTEM errors
//
//
// MessageId: hrFileClose
//
// MessageText:
//
// Unable to close the DOS file
//
#define hrFileClose                      ((HRESULT)0xC8000066L)

//
// MessageId: hrOutOfThreads
//
// MessageText:
//
// Unable to start a thread because there are none available.
//
#define hrOutOfThreads                   ((HRESULT)0xC8000067L)

//
// MessageId: hrTooManyIO
//
// MessageText:
//
// The system is busy because there are too many I/Os.
//
#define hrTooManyIO                      ((HRESULT)0xC8000069L)

//
//	BUFFER MANAGER errors
//
//
// MessageId: hrBFNotSynchronous
//
// MessageText:
//
// The buffer page has been evicted.
//
#define hrBFNotSynchronous               ((HRESULT)0x880000C8L)

//
// MessageId: hrBFPageNotFound
//
// MessageText:
//
// Unable to find the page.
//
#define hrBFPageNotFound                 ((HRESULT)0x880000C9L)

//
// MessageId: hrBFInUse
//
// MessageText:
//
// Unable to abandon the buffer.
//
#define hrBFInUse                        ((HRESULT)0xC80000CAL)

//
//	DIRECTORY MANAGER errors
//
//
// MessageId: hrPMRecDeleted
//
// MessageText:
//
// The record has been deleted.
//
#define hrPMRecDeleted                   ((HRESULT)0xC800012EL)

//
// MessageId: hrRemainingVersions
//
// MessageText:
//
// There is idle work remaining.
//
#define hrRemainingVersions              ((HRESULT)0x88000141L)

//
//	RECORD MANAGER errors
//
//
// MessageId: hrFLDKeyTooBig
//
// MessageText:
//
// The key was truncated because it exceeded the maximum length.
//
#define hrFLDKeyTooBig                   ((HRESULT)0x88000190L)

//
// MessageId: hrFLDTooManySegments
//
// MessageText:
//
// There are too many key segments.
//
#define hrFLDTooManySegments             ((HRESULT)0xC8000191L)

//
// MessageId: hrFLDNullKey
//
// MessageText:
//
// The key is NULL.
//
#define hrFLDNullKey                     ((HRESULT)0x88000192L)

//
//	LOGGING/RECOVERY errors
//
//
// MessageId: hrLogFileCorrupt
//
// MessageText:
//
// The log file is damaged.
//
#define hrLogFileCorrupt                 ((HRESULT)0xC80001F5L)

//
// MessageId: hrNoBackupDirectory
//
// MessageText:
//
// No backup directory was given.
//
#define hrNoBackupDirectory              ((HRESULT)0xC80001F7L)

//
// MessageId: hrBackupDirectoryNotEmpty
//
// MessageText:
//
// The backup directory is not empty.
//
#define hrBackupDirectoryNotEmpty        ((HRESULT)0xC80001F8L)

//
// MessageId: hrBackupInProgress
//
// MessageText:
//
// Backup is already active.
//
#define hrBackupInProgress               ((HRESULT)0xC80001F9L)

//
// MessageId: hrMissingPreviousLogFile
//
// MessageText:
//
// A log file for the checkpoint is missing.
//
#define hrMissingPreviousLogFile         ((HRESULT)0xC80001FDL)

//
// MessageId: hrLogWriteFail
//
// MessageText:
//
// Unable to write to the log file.
//
#define hrLogWriteFail                   ((HRESULT)0xC80001FEL)

//
// MessageId: hrBadLogVersion
//
// MessageText:
//
// The version of the log file is not compatible with the version of the Windows NT Directory Service database (NTDS).
//
#define hrBadLogVersion                  ((HRESULT)0xC8000202L)

//
// MessageId: hrInvalidLogSequence
//
// MessageText:
//
// The time stamp in the next log does not match what was expected.
//
#define hrInvalidLogSequence             ((HRESULT)0xC8000203L)

//
// MessageId: hrLoggingDisabled
//
// MessageText:
//
// The log is not active.
//
#define hrLoggingDisabled                ((HRESULT)0xC8000204L)

//
// MessageId: hrLogBufferTooSmall
//
// MessageText:
//
// The log buffer is too small to be recovered.
//
#define hrLogBufferTooSmall              ((HRESULT)0xC8000205L)

//
// MessageId: hrLogSequenceEnd
//
// MessageText:
//
// The maximum number of log files has been exceeded.
//
#define hrLogSequenceEnd                 ((HRESULT)0xC8000207L)

//
// MessageId: hrNoBackup
//
// MessageText:
//
// There is no backup in progress.
//
#define hrNoBackup                       ((HRESULT)0xC8000208L)

//
// MessageId: hrInvalidBackupSequence
//
// MessageText:
//
// The backup call is out of sequence.
//
#define hrInvalidBackupSequence          ((HRESULT)0xC8000209L)

//
// MessageId: hrBackupNotAllowedYet
//
// MessageText:
//
// Unable to perform a backup now.
//
#define hrBackupNotAllowedYet            ((HRESULT)0xC800020BL)

//
// MessageId: hrDeleteBackupFileFail
//
// MessageText:
//
// Unable to delete the backup file.
//
#define hrDeleteBackupFileFail           ((HRESULT)0xC800020CL)

//
// MessageId: hrMakeBackupDirectoryFail
//
// MessageText:
//
// Unable to make a backup temporary directory.
//
#define hrMakeBackupDirectoryFail        ((HRESULT)0xC800020DL)

//
// MessageId: hrInvalidBackup
//
// MessageText:
//
// An incremental backup cannot be performed when circular logging is enabled.
//
#define hrInvalidBackup                  ((HRESULT)0xC800020EL)

//
// MessageId: hrRecoveredWithErrors
//
// MessageText:
//
// Errors were encountered during the repair process.
//
#define hrRecoveredWithErrors            ((HRESULT)0xC800020FL)

//
// MessageId: hrMissingLogFile
//
// MessageText:
//
// The current log file is missing.
//
#define hrMissingLogFile                 ((HRESULT)0xC8000210L)

//
// MessageId: hrLogDiskFull
//
// MessageText:
//
// The log disk is full.
//
#define hrLogDiskFull                    ((HRESULT)0xC8000211L)

//
// MessageId: hrBadLogSignature
//
// MessageText:
//
// A log file is damaged.
//
#define hrBadLogSignature                ((HRESULT)0xC8000212L)

//
// MessageId: hrBadDbSignature
//
// MessageText:
//
// A database file is damaged.
//
#define hrBadDbSignature                 ((HRESULT)0xC8000213L)

//
// MessageId: hrBadCheckpointSignature
//
// MessageText:
//
// A checkpoint file is damaged.
//
#define hrBadCheckpointSignature         ((HRESULT)0xC8000214L)

//
// MessageId: hrCheckpointCorrupt
//
// MessageText:
//
// A checkpoint file either could not be found or is damaged.
//
#define hrCheckpointCorrupt              ((HRESULT)0xC8000215L)

//
// MessageId: hrDatabaseInconsistent
//
// MessageText:
//
// The database is damaged.
//
#define hrDatabaseInconsistent           ((HRESULT)0xC8000226L)

//
// MessageId: hrConsistentTimeMismatch
//
// MessageText:
//
// There is a mismatch in the database's last consistent time.
//
#define hrConsistentTimeMismatch         ((HRESULT)0xC8000227L)

//
// MessageId: hrPatchFileMismatch
//
// MessageText:
//
// The patch file is not generated from this backup.
//
#define hrPatchFileMismatch              ((HRESULT)0xC8000228L)

//
// MessageId: hrRestoreLogTooLow
//
// MessageText:
//
// The starting log number is too low for the restore.
//
#define hrRestoreLogTooLow               ((HRESULT)0xC8000229L)

//
// MessageId: hrRestoreLogTooHigh
//
// MessageText:
//
// The starting log number is too high for the restore.
//
#define hrRestoreLogTooHigh              ((HRESULT)0xC800022AL)

//
// MessageId: hrGivenLogFileHasBadSignature
//
// MessageText:
//
// The log file downloaded from the tape is damaged.
//
#define hrGivenLogFileHasBadSignature    ((HRESULT)0xC800022BL)

//
// MessageId: hrGivenLogFileIsNotContiguous
//
// MessageText:
//
// Unable to find a mandatory log file after the tape was downloaded.
//
#define hrGivenLogFileIsNotContiguous    ((HRESULT)0xC800022CL)

//
// MessageId: hrMissingRestoreLogFiles
//
// MessageText:
//
// The data is not fully restored because some log files are missing.
//
#define hrMissingRestoreLogFiles         ((HRESULT)0xC800022DL)

//
// MessageId: hrExistingLogFileHasBadSignature
//
// MessageText:
//
// The log file in the log file path is damaged.
//
#define hrExistingLogFileHasBadSignature ((HRESULT)0x8800022EL)

//
// MessageId: hrExistingLogFileIsNotContiguous
//
// MessageText:
//
// Unable to find a mandatory log file in the log file path.
//
#define hrExistingLogFileIsNotContiguous ((HRESULT)0x8800022FL)

//
// MessageId: hrMissingFullBackup
//
// MessageText:
//
// The database missed a previous full backup before the incremental backup.
//
#define hrMissingFullBackup              ((HRESULT)0xC8000230L)

//
// MessageId: hrBadBackupDatabaseSize
//
// MessageText:
//
// The backup database size must be a multiple of 4K (4096 bytes).
//
#define hrBadBackupDatabaseSize          ((HRESULT)0xC8000231L)

//
// MessageId: hrTermInProgress
//
// MessageText:
//
// The database is being shut down.
//
#define hrTermInProgress                 ((HRESULT)0xC80003E8L)

//
// MessageId: hrFeatureNotAvailable
//
// MessageText:
//
// The feature is not available.
//
#define hrFeatureNotAvailable            ((HRESULT)0xC80003E9L)

//
// MessageId: hrInvalidName
//
// MessageText:
//
// The name is not valid.
//
#define hrInvalidName                    ((HRESULT)0xC80003EAL)

//
// MessageId: hrInvalidParameter
//
// MessageText:
//
// The parameter is not valid.
//
#define hrInvalidParameter               ((HRESULT)0xC80003EBL)

//
// MessageId: hrColumnNull
//
// MessageText:
//
// The value of the column is null.
//
#define hrColumnNull                     ((HRESULT)0x880003ECL)

//
// MessageId: hrBufferTruncated
//
// MessageText:
//
// The buffer is too small for data.
//
#define hrBufferTruncated                ((HRESULT)0x880003EEL)

//
// MessageId: hrDatabaseAttached
//
// MessageText:
//
// The database is already attached.
//
#define hrDatabaseAttached               ((HRESULT)0x880003EFL)

//
// MessageId: hrInvalidDatabaseId
//
// MessageText:
//
// The database ID is not valid.
//
#define hrInvalidDatabaseId              ((HRESULT)0xC80003F2L)

//
// MessageId: hrOutOfMemory
//
// MessageText:
//
// The computer is out of memory.
//
#define hrOutOfMemory                    ((HRESULT)0xC80003F3L)

//
// MessageId: hrOutOfDatabaseSpace
//
// MessageText:
//
// The database has reached the maximum size of 16 GB.
//
#define hrOutOfDatabaseSpace             ((HRESULT)0xC80003F4L)

//
// MessageId: hrOutOfCursors
//
// MessageText:
//
// Out of table cursors.
//
#define hrOutOfCursors                   ((HRESULT)0xC80003F5L)

//
// MessageId: hrOutOfBuffers
//
// MessageText:
//
// Out of database page buffers.
//
#define hrOutOfBuffers                   ((HRESULT)0xC80003F6L)

//
// MessageId: hrTooManyIndexes
//
// MessageText:
//
// There are too many indexes.
//
#define hrTooManyIndexes                 ((HRESULT)0xC80003F7L)

//
// MessageId: hrTooManyKeys
//
// MessageText:
//
// There are too many columns in an index.
//
#define hrTooManyKeys                    ((HRESULT)0xC80003F8L)

//
// MessageId: hrRecordDeleted
//
// MessageText:
//
// The record has been deleted.
//
#define hrRecordDeleted                  ((HRESULT)0xC80003F9L)

//
// MessageId: hrReadVerifyFailure
//
// MessageText:
//
// A read verification error occurred.
//
#define hrReadVerifyFailure              ((HRESULT)0xC80003FAL)

//
// MessageId: hrOutOfFileHandles
//
// MessageText:
//
// Out of file handles.
//
#define hrOutOfFileHandles               ((HRESULT)0xC80003FCL)

//
// MessageId: hrDiskIO
//
// MessageText:
//
// A disk I/O error occurred.
//
#define hrDiskIO                         ((HRESULT)0xC80003FEL)

//
// MessageId: hrInvalidPath
//
// MessageText:
//
// The path to the file is not valid.
//
#define hrInvalidPath                    ((HRESULT)0xC80003FFL)

//
// MessageId: hrRecordTooBig
//
// MessageText:
//
// The record has exceeded the maximum size.
//
#define hrRecordTooBig                   ((HRESULT)0xC8000402L)

//
// MessageId: hrTooManyOpenDatabases
//
// MessageText:
//
// There are too many open databases.
//
#define hrTooManyOpenDatabases           ((HRESULT)0xC8000403L)

//
// MessageId: hrInvalidDatabase
//
// MessageText:
//
// The file is not a database file.
//
#define hrInvalidDatabase                ((HRESULT)0xC8000404L)

//
// MessageId: hrNotInitialized
//
// MessageText:
//
// The database was not yet called.
//
#define hrNotInitialized                 ((HRESULT)0xC8000405L)

//
// MessageId: hrAlreadyInitialized
//
// MessageText:
//
// The database was already called.
//
#define hrAlreadyInitialized             ((HRESULT)0xC8000406L)

//
// MessageId: hrFileAccessDenied
//
// MessageText:
//
// Unable to access the file.
//
#define hrFileAccessDenied               ((HRESULT)0xC8000408L)

//
// MessageId: hrBufferTooSmall
//
// MessageText:
//
// The buffer is too small.
//
#define hrBufferTooSmall                 ((HRESULT)0xC800040EL)

//
// MessageId: hrSeekNotEqual
//
// MessageText:
//
// Either SeekLE or SeekGE did not find an exact match.
//
#define hrSeekNotEqual                   ((HRESULT)0x8800040FL)

//
// MessageId: hrTooManyColumns
//
// MessageText:
//
// There are too many columns defined.
//
#define hrTooManyColumns                 ((HRESULT)0xC8000410L)

//
// MessageId: hrContainerNotEmpty
//
// MessageText:
//
// The container is not empty.
//
#define hrContainerNotEmpty              ((HRESULT)0xC8000413L)

//
// MessageId: hrInvalidFilename
//
// MessageText:
//
// The filename is not valid.
//
#define hrInvalidFilename                ((HRESULT)0xC8000414L)

//
// MessageId: hrInvalidBookmark
//
// MessageText:
//
// The bookmark is not valid.
//
#define hrInvalidBookmark                ((HRESULT)0xC8000415L)

//
// MessageId: hrColumnInUse
//
// MessageText:
//
// The column is used in an index.
//
#define hrColumnInUse                    ((HRESULT)0xC8000416L)

//
// MessageId: hrInvalidBufferSize
//
// MessageText:
//
// The data buffer does not match the column size.
//
#define hrInvalidBufferSize              ((HRESULT)0xC8000417L)

//
// MessageId: hrColumnNotUpdatable
//
// MessageText:
//
// Unable to set the column value.
//
#define hrColumnNotUpdatable             ((HRESULT)0xC8000418L)

//
// MessageId: hrIndexInUse
//
// MessageText:
//
// The index is in use.
//
#define hrIndexInUse                     ((HRESULT)0xC800041BL)

//
// MessageId: hrNullKeyDisallowed
//
// MessageText:
//
// Null keys are not allowed on an index.
//
#define hrNullKeyDisallowed              ((HRESULT)0xC800041DL)

//
// MessageId: hrNotInTransaction
//
// MessageText:
//
// The operation must be within a transaction.
//
#define hrNotInTransaction               ((HRESULT)0xC800041EL)

//
// MessageId: hrNoIdleActivity
//
// MessageText:
//
// No idle activity occurred.
//
#define hrNoIdleActivity                 ((HRESULT)0x88000422L)

//
// MessageId: hrTooManyActiveUsers
//
// MessageText:
//
// There are too many active database users.
//
#define hrTooManyActiveUsers             ((HRESULT)0xC8000423L)

//
// MessageId: hrInvalidCountry
//
// MessageText:
//
// The country code is either not known or is not valid.
//
#define hrInvalidCountry                 ((HRESULT)0xC8000425L)

//
// MessageId: hrInvalidLanguageId
//
// MessageText:
//
// The language ID is either not known or is not valid.
//
#define hrInvalidLanguageId              ((HRESULT)0xC8000426L)

//
// MessageId: hrInvalidCodePage
//
// MessageText:
//
// The code page is either not known or is not valid.
//
#define hrInvalidCodePage                ((HRESULT)0xC8000427L)

//
// MessageId: hrNoWriteLock
//
// MessageText:
//
// There is no write lock at transaction level 0.
//
#define hrNoWriteLock                    ((HRESULT)0x8800042BL)

//
// MessageId: hrColumnSetNull
//
// MessageText:
//
// The column value is set to null.
//
#define hrColumnSetNull                  ((HRESULT)0x8800042CL)

//
// MessageId: hrVersionStoreOutOfMemory
//
// MessageText:
//
//  lMaxVerPages exceeded (XJET only)
//
#define hrVersionStoreOutOfMemory        ((HRESULT)0xC800042DL)

//
// MessageId: hrCurrencyStackOutOfMemory
//
// MessageText:
//
// Out of cursors.
//
#define hrCurrencyStackOutOfMemory       ((HRESULT)0xC800042EL)

//
// MessageId: hrOutOfSessions
//
// MessageText:
//
// Out of sessions.
//
#define hrOutOfSessions                  ((HRESULT)0xC800044DL)

//
// MessageId: hrWriteConflict
//
// MessageText:
//
// The write lock failed due to an outstanding write lock.
//
#define hrWriteConflict                  ((HRESULT)0xC800044EL)

//
// MessageId: hrTransTooDeep
//
// MessageText:
//
// The transactions are nested too deeply.
//
#define hrTransTooDeep                   ((HRESULT)0xC800044FL)

//
// MessageId: hrInvalidSesid
//
// MessageText:
//
// The session handle is not valid.
//
#define hrInvalidSesid                   ((HRESULT)0xC8000450L)

//
// MessageId: hrSessionWriteConflict
//
// MessageText:
//
// Another session has a private version of the page.
//
#define hrSessionWriteConflict           ((HRESULT)0xC8000453L)

//
// MessageId: hrInTransaction
//
// MessageText:
//
// The operation is not allowed within a transaction.
//
#define hrInTransaction                  ((HRESULT)0xC8000454L)

//
// MessageId: hrDatabaseDuplicate
//
// MessageText:
//
// The database already exists.
//
#define hrDatabaseDuplicate              ((HRESULT)0xC80004B1L)

//
// MessageId: hrDatabaseInUse
//
// MessageText:
//
// The database is in use.
//
#define hrDatabaseInUse                  ((HRESULT)0xC80004B2L)

//
// MessageId: hrDatabaseNotFound
//
// MessageText:
//
// The database does not exist.
//
#define hrDatabaseNotFound               ((HRESULT)0xC80004B3L)

//
// MessageId: hrDatabaseInvalidName
//
// MessageText:
//
// The database name is not valid.
//
#define hrDatabaseInvalidName            ((HRESULT)0xC80004B4L)

//
// MessageId: hrDatabaseInvalidPages
//
// MessageText:
//
// The number of pages is not valid.
//
#define hrDatabaseInvalidPages           ((HRESULT)0xC80004B5L)

//
// MessageId: hrDatabaseCorrupted
//
// MessageText:
//
// The database file is either damaged or cannot be found.
//
#define hrDatabaseCorrupted              ((HRESULT)0xC80004B6L)

//
// MessageId: hrDatabaseLocked
//
// MessageText:
//
// The database is locked.
//
#define hrDatabaseLocked                 ((HRESULT)0xC80004B7L)

//
// MessageId: hrTableEmpty
//
// MessageText:
//
// An empty table was opened.
//
#define hrTableEmpty                     ((HRESULT)0x88000515L)

//
// MessageId: hrTableLocked
//
// MessageText:
//
// The table is locked.
//
#define hrTableLocked                    ((HRESULT)0xC8000516L)

//
// MessageId: hrTableDuplicate
//
// MessageText:
//
// The table already exists.
//
#define hrTableDuplicate                 ((HRESULT)0xC8000517L)

//
// MessageId: hrTableInUse
//
// MessageText:
//
// Unable to lock the table because it is already in use.
//
#define hrTableInUse                     ((HRESULT)0xC8000518L)

//
// MessageId: hrObjectNotFound
//
// MessageText:
//
// The table or object does not exist.
//
#define hrObjectNotFound                 ((HRESULT)0xC8000519L)

//
// MessageId: hrCannotRename
//
// MessageText:
//
// Unable to rename the temporary file.
//
#define hrCannotRename                   ((HRESULT)0xC800051AL)

//
// MessageId: hrDensityInvalid
//
// MessageText:
//
// The file/index density is not valid.
//
#define hrDensityInvalid                 ((HRESULT)0xC800051BL)

//
// MessageId: hrTableNotEmpty
//
// MessageText:
//
// Unable to define the clustered index.
//
#define hrTableNotEmpty                  ((HRESULT)0xC800051CL)

//
// MessageId: hrInvalidTableId
//
// MessageText:
//
// The table ID is not valid.
//
#define hrInvalidTableId                 ((HRESULT)0xC800051EL)

//
// MessageId: hrTooManyOpenTables
//
// MessageText:
//
// Unable to open any more tables.
//
#define hrTooManyOpenTables              ((HRESULT)0xC800051FL)

//
// MessageId: hrIllegalOperation
//
// MessageText:
//
// The operation is not supported on tables.
//
#define hrIllegalOperation               ((HRESULT)0xC8000520L)

//
// MessageId: hrObjectDuplicate
//
// MessageText:
//
// The table or object name is already being used.
//
#define hrObjectDuplicate                ((HRESULT)0xC8000522L)

//
// MessageId: hrInvalidObject
//
// MessageText:
//
// The object is not valid for operation.
//
#define hrInvalidObject                  ((HRESULT)0xC8000524L)

//
// MessageId: hrIndexCantBuild
//
// MessageText:
//
// Unable to build a clustered index.
//
#define hrIndexCantBuild                 ((HRESULT)0xC8000579L)

//
// MessageId: hrIndexHasPrimary
//
// MessageText:
//
// The primary index is already defined.
//
#define hrIndexHasPrimary                ((HRESULT)0xC800057AL)

//
// MessageId: hrIndexDuplicate
//
// MessageText:
//
// The index is already defined.
//
#define hrIndexDuplicate                 ((HRESULT)0xC800057BL)

//
// MessageId: hrIndexNotFound
//
// MessageText:
//
// The index does not exist.
//
#define hrIndexNotFound                  ((HRESULT)0xC800057CL)

//
// MessageId: hrIndexMustStay
//
// MessageText:
//
// Unable to delete a clustered index.
//
#define hrIndexMustStay                  ((HRESULT)0xC800057DL)

//
// MessageId: hrIndexInvalidDef
//
// MessageText:
//
// The index definition is illegal.
//
#define hrIndexInvalidDef                ((HRESULT)0xC800057EL)

//
// MessageId: hrIndexHasClustered
//
// MessageText:
//
// The clustered index is already defined.
//
#define hrIndexHasClustered              ((HRESULT)0xC8000580L)

//
// MessageId: hrCreateIndexFailed
//
// MessageText:
//
// Unable to create the index because an error occurred while creating a table.
//
#define hrCreateIndexFailed              ((HRESULT)0x88000581L)

//
// MessageId: hrTooManyOpenIndexes
//
// MessageText:
//
// Out of index description blocks.
//
#define hrTooManyOpenIndexes             ((HRESULT)0xC8000582L)

//
// MessageId: hrColumnLong
//
// MessageText:
//
// The column value is too long.
//
#define hrColumnLong                     ((HRESULT)0xC80005DDL)

//
// MessageId: hrColumnDoesNotFit
//
// MessageText:
//
// The field will not fit in the record.
//
#define hrColumnDoesNotFit               ((HRESULT)0xC80005DFL)

//
// MessageId: hrNullInvalid
//
// MessageText:
//
// The value cannot be null.
//
#define hrNullInvalid                    ((HRESULT)0xC80005E0L)

//
// MessageId: hrColumnIndexed
//
// MessageText:
//
// Unable to delete because the column is indexed.
//
#define hrColumnIndexed                  ((HRESULT)0xC80005E1L)

//
// MessageId: hrColumnTooBig
//
// MessageText:
//
// The length of the field exceeds the maximum length.
//
#define hrColumnTooBig                   ((HRESULT)0xC80005E2L)

//
// MessageId: hrColumnNotFound
//
// MessageText:
//
// Unable to find the column.
//
#define hrColumnNotFound                 ((HRESULT)0xC80005E3L)

//
// MessageId: hrColumnDuplicate
//
// MessageText:
//
// The field is already defined.
//
#define hrColumnDuplicate                ((HRESULT)0xC80005E4L)

//
// MessageId: hrColumn2ndSysMaint
//
// MessageText:
//
// Only one auto-increment or version column is allowed per table.
//
#define hrColumn2ndSysMaint              ((HRESULT)0xC80005E6L)

//
// MessageId: hrInvalidColumnType
//
// MessageText:
//
// The column data type is not valid.
//
#define hrInvalidColumnType              ((HRESULT)0xC80005E7L)

//
// MessageId: hrColumnMaxTruncated
//
// MessageText:
//
// The column was truncated because it exceeded the maximum length.
//
#define hrColumnMaxTruncated             ((HRESULT)0x880005E8L)

//
// MessageId: hrColumnCannotIndex
//
// MessageText:
//
// Unable to index a long value column.
//
#define hrColumnCannotIndex              ((HRESULT)0xC80005E9L)

//
// MessageId: hrTaggedNotNULL
//
// MessageText:
//
// Tagged columns cannot be null.
//
#define hrTaggedNotNULL                  ((HRESULT)0xC80005EAL)

//
// MessageId: hrNoCurrentIndex
//
// MessageText:
//
// The entry is not valid without a current index.
//
#define hrNoCurrentIndex                 ((HRESULT)0xC80005EBL)

//
// MessageId: hrKeyIsMade
//
// MessageText:
//
// The key is completely made.
//
#define hrKeyIsMade                      ((HRESULT)0xC80005ECL)

//
// MessageId: hrBadColumnId
//
// MessageText:
//
// The column ID is not correct.
//
#define hrBadColumnId                    ((HRESULT)0xC80005EDL)

//
// MessageId: hrBadItagSequence
//
// MessageText:
//
// There is a bad instance identifier for a multivalued column.
//
#define hrBadItagSequence                ((HRESULT)0xC80005EEL)

//
// MessageId: hrCannotBeTagged
//
// MessageText:
//
// AutoIncrement and Version cannot be multivalued.
//
#define hrCannotBeTagged                 ((HRESULT)0xC80005F1L)

//
// MessageId: hrRecordNotFound
//
// MessageText:
//
// Unable to find the key.
//
#define hrRecordNotFound                 ((HRESULT)0xC8000641L)

//
// MessageId: hrNoCurrentRecord
//
// MessageText:
//
// The currency is not on a record.
//
#define hrNoCurrentRecord                ((HRESULT)0xC8000643L)

//
// MessageId: hrRecordClusteredChanged
//
// MessageText:
//
// A clustered key cannot be changed.
//
#define hrRecordClusteredChanged         ((HRESULT)0xC8000644L)

//
// MessageId: hrKeyDuplicate
//
// MessageText:
//
// The key already exists.
//
#define hrKeyDuplicate                   ((HRESULT)0xC8000645L)

//
// MessageId: hrAlreadyPrepared
//
// MessageText:
//
// The current entry has already been copied or cleared.
//
#define hrAlreadyPrepared                ((HRESULT)0xC8000647L)

//
// MessageId: hrKeyNotMade
//
// MessageText:
//
// No key was made.
//
#define hrKeyNotMade                     ((HRESULT)0xC8000648L)

//
// MessageId: hrUpdateNotPrepared
//
// MessageText:
//
// Update was not prepared.
//
#define hrUpdateNotPrepared              ((HRESULT)0xC8000649L)

//
// MessageId: hrwrnDataHasChanged
//
// MessageText:
//
// Data has changed.
//
#define hrwrnDataHasChanged              ((HRESULT)0x8800064AL)

//
// MessageId: hrerrDataHasChanged
//
// MessageText:
//
// The operation was abandoned because data has changed.
//
#define hrerrDataHasChanged              ((HRESULT)0xC800064BL)

//
// MessageId: hrKeyChanged
//
// MessageText:
//
// Moved to a new key.
//
#define hrKeyChanged                     ((HRESULT)0x88000652L)

//
// MessageId: hrTooManySorts
//
// MessageText:
//
// There are too many sort processes.
//
#define hrTooManySorts                   ((HRESULT)0xC80006A5L)

//
// MessageId: hrInvalidOnSort
//
// MessageText:
//
// An operation that is not valid occurred in the sort.
//
#define hrInvalidOnSort                  ((HRESULT)0xC80006A6L)

//
// MessageId: hrTempFileOpenError
//
// MessageText:
//
// Unable to open the temporary file.
//
#define hrTempFileOpenError              ((HRESULT)0xC800070BL)

//
// MessageId: hrTooManyAttachedDatabases
//
// MessageText:
//
// There are too many databases open.
//
#define hrTooManyAttachedDatabases       ((HRESULT)0xC800070DL)

//
// MessageId: hrDiskFull
//
// MessageText:
//
// The disk is full.
//
#define hrDiskFull                       ((HRESULT)0xC8000710L)

//
// MessageId: hrPermissionDenied
//
// MessageText:
//
// Permission is denied.
//
#define hrPermissionDenied               ((HRESULT)0xC8000711L)

//
// MessageId: hrFileNotFound
//
// MessageText:
//
// Unable to find the file.
//
#define hrFileNotFound                   ((HRESULT)0xC8000713L)

//
// MessageId: hrFileOpenReadOnly
//
// MessageText:
//
// The database file is read only.
//
#define hrFileOpenReadOnly               ((HRESULT)0x88000715L)

//
// MessageId: hrAfterInitialization
//
// MessageText:
//
// Unable to restore after initialization.
//
#define hrAfterInitialization            ((HRESULT)0xC800073AL)

//
// MessageId: hrLogCorrupted
//
// MessageText:
//
// The database log files are damaged.
//
#define hrLogCorrupted                   ((HRESULT)0xC800073CL)

//
// MessageId: hrInvalidOperation
//
// MessageText:
//
// The operation is not valid.
//
#define hrInvalidOperation               ((HRESULT)0xC8000772L)

//
// MessageId: hrAccessDenied
//
// MessageText:
//
// Access is denied.
//
#define hrAccessDenied                   ((HRESULT)0xC8000773L)

#endif	// _NTDSBMSG_
