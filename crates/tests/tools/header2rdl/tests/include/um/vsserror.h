/*--

Copyright (c) 1999-2005  Microsoft Corporation

Module Name:

    errors.h

Abstract:

    This file contains the message definitions for common VSS errors.
    They are a subset of message definitions of vssadmin.exe.

Author:

Revision History:

--*/

#ifndef _ERRORS_H_
#define _ERRORS_H_
//
//  VSS error codes.
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


//
// MessageId: VSS_E_BAD_STATE
//
// MessageText:
//
// A function call was made when the object was in an incorrect state
// for that function
//
#define VSS_E_BAD_STATE                  ((HRESULT)0x80042301L)

//
// MessageId: VSS_E_UNEXPECTED
//
// MessageText:
//
// A Volume Shadow Copy Service component encountered an unexpected error.
// Check the Application event log for more information.
//
#define VSS_E_UNEXPECTED                 ((HRESULT)0x80042302L)

//
// MessageId: VSS_E_PROVIDER_ALREADY_REGISTERED
//
// MessageText:
//
// The provider has already been registered.
//
#define VSS_E_PROVIDER_ALREADY_REGISTERED ((HRESULT)0x80042303L)

//
// MessageId: VSS_E_PROVIDER_NOT_REGISTERED
//
// MessageText:
//
// The volume shadow copy provider is not registered in the system.
//
#define VSS_E_PROVIDER_NOT_REGISTERED    ((HRESULT)0x80042304L)

//
// MessageId: VSS_E_PROVIDER_VETO
//
// MessageText:
//
// The shadow copy provider had an error. Check the System and Application event logs for more information.
//
#define VSS_E_PROVIDER_VETO              ((HRESULT)0x80042306L)

//
// MessageId: VSS_E_PROVIDER_IN_USE
//
// MessageText:
//
// The shadow copy provider is currently in use and cannot be unregistered.
//
#define VSS_E_PROVIDER_IN_USE            ((HRESULT)0x80042307L)

//
// MessageId: VSS_E_OBJECT_NOT_FOUND
//
// MessageText:
//
// The specified object was not found.
//
#define VSS_E_OBJECT_NOT_FOUND           ((HRESULT)0x80042308L)

//
// MessageId: VSS_S_ASYNC_PENDING
//
// MessageText:
//
// The asynchronous operation is pending.
//
#define VSS_S_ASYNC_PENDING              ((HRESULT)0x00042309L)

//
// MessageId: VSS_S_ASYNC_FINISHED
//
// MessageText:
//
// The asynchronous operation has completed.
//
#define VSS_S_ASYNC_FINISHED             ((HRESULT)0x0004230AL)

//
// MessageId: VSS_S_ASYNC_CANCELLED
//
// MessageText:
//
// The asynchronous operation has been cancelled.
//
#define VSS_S_ASYNC_CANCELLED            ((HRESULT)0x0004230BL)

//
// MessageId: VSS_E_VOLUME_NOT_SUPPORTED
//
// MessageText:
//
// Shadow copying the specified volume is not supported.
//
#define VSS_E_VOLUME_NOT_SUPPORTED       ((HRESULT)0x8004230CL)

//
// MessageId: VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER
//
// MessageText:
//
// The given shadow copy provider does not support shadow copying the specified volume.
//
#define VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER ((HRESULT)0x8004230EL)

//
// MessageId: VSS_E_OBJECT_ALREADY_EXISTS
//
// MessageText:
//
// The object already exists.
//
#define VSS_E_OBJECT_ALREADY_EXISTS      ((HRESULT)0x8004230DL)

//
// MessageId: VSS_E_UNEXPECTED_PROVIDER_ERROR
//
// MessageText:
//
// The shadow copy provider had an unexpected error while trying to process the specified operation.
//
#define VSS_E_UNEXPECTED_PROVIDER_ERROR  ((HRESULT)0x8004230FL)

//
// MessageId: VSS_E_CORRUPT_XML_DOCUMENT
//
// MessageText:
//
// The given XML document is invalid.  It is either incorrectly-formed XML or it does not match the
// schema.  This error code is deprecated.
//
#define VSS_E_CORRUPT_XML_DOCUMENT       ((HRESULT)0x80042310L)

//
// MessageId: VSS_E_INVALID_XML_DOCUMENT
//
// MessageText:
//
// The given XML document is invalid.  It is either incorrectly-formed XML or it does not match the
// schema.
//
#define VSS_E_INVALID_XML_DOCUMENT       ((HRESULT)0x80042311L)

//
// MessageId: VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED
//
// MessageText:
//
// The maximum number of volumes for this operation has been reached.
//
#define VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED ((HRESULT)0x80042312L)

//
// MessageId: VSS_E_FLUSH_WRITES_TIMEOUT
//
// MessageText:
//
// The shadow copy provider timed out while flushing data to the volume being shadow copied. This is probably due to excessive activity on the volume. Try again later when the volume is not being used so heavily.
//
#define VSS_E_FLUSH_WRITES_TIMEOUT       ((HRESULT)0x80042313L)

//
// MessageId: VSS_E_HOLD_WRITES_TIMEOUT
//
// MessageText:
//
// The shadow copy provider timed out while holding writes to the volume being shadow copied. This is probably due to excessive activity on the volume by an application or a system service. Try again later when activity on the volume is reduced.
//
#define VSS_E_HOLD_WRITES_TIMEOUT        ((HRESULT)0x80042314L)

//
// MessageId: VSS_E_UNEXPECTED_WRITER_ERROR
//
// MessageText:
//
// VSS encountered problems while sending events to writers.
//
#define VSS_E_UNEXPECTED_WRITER_ERROR    ((HRESULT)0x80042315L)

//
// MessageId: VSS_E_SNAPSHOT_SET_IN_PROGRESS
//
// MessageText:
//
// Another shadow copy creation is already in progress. Wait a few moments and try again.
//
#define VSS_E_SNAPSHOT_SET_IN_PROGRESS   ((HRESULT)0x80042316L)

//
// MessageId: VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED
//
// MessageText:
//
// The specified volume has already reached its maximum number of shadow copies.
//
#define VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED ((HRESULT)0x80042317L)

//
// MessageId: VSS_E_WRITER_INFRASTRUCTURE
//
// MessageText:
//
// An error was detected in the Volume Shadow Copy Service (VSS). The problem occurred while trying to contact VSS writers.
// Verify that the Event System service and the VSS service are running and check for associated errors in the event logs.
//
#define VSS_E_WRITER_INFRASTRUCTURE      ((HRESULT)0x80042318L)

//
// MessageId: VSS_E_WRITER_NOT_RESPONDING
//
// MessageText:
//
// A writer did not respond to a GatherWriterStatus call.  The writer may either have terminated
// or it may be stuck.  Check the System and Application event logs for more information.
//
#define VSS_E_WRITER_NOT_RESPONDING      ((HRESULT)0x80042319L)

//
// MessageId: VSS_E_WRITER_ALREADY_SUBSCRIBED
//
// MessageText:
//
// The writer has already successfully called the Subscribe function.  It cannot call
// Subscribe multiple times.
//
#define VSS_E_WRITER_ALREADY_SUBSCRIBED  ((HRESULT)0x8004231AL)

//
// MessageId: VSS_E_UNSUPPORTED_CONTEXT
//
// MessageText:
//
// The shadow copy provider does not support the specified shadow copy type.
//
#define VSS_E_UNSUPPORTED_CONTEXT        ((HRESULT)0x8004231BL)

//
// MessageId: VSS_E_VOLUME_IN_USE
//
// MessageText:
//
// The specified shadow copy storage association is in use and so can't be deleted.
//
#define VSS_E_VOLUME_IN_USE              ((HRESULT)0x8004231DL)

//
// MessageId: VSS_E_MAXIMUM_DIFFAREA_ASSOCIATIONS_REACHED
//
// MessageText:
//
// Maximum number of shadow copy storage associations already reached.
//
#define VSS_E_MAXIMUM_DIFFAREA_ASSOCIATIONS_REACHED ((HRESULT)0x8004231EL)

//
// MessageId: VSS_E_INSUFFICIENT_STORAGE
//
// MessageText:
//
// Insufficient storage available to create either the shadow copy storage file or other shadow copy data.
//
#define VSS_E_INSUFFICIENT_STORAGE       ((HRESULT)0x8004231FL)

//
// MessageId: VSS_E_NO_SNAPSHOTS_IMPORTED
//
// MessageText:
//
// No shadow copies were successfully imported.
//
#define VSS_E_NO_SNAPSHOTS_IMPORTED      ((HRESULT)0x80042320L)

//
// MessageId: VSS_S_SOME_SNAPSHOTS_NOT_IMPORTED
//
// MessageText:
//
// Some shadow copies were not successfully imported.
//
#define VSS_S_SOME_SNAPSHOTS_NOT_IMPORTED ((HRESULT)0x00042321L)

//
// MessageId: VSS_E_SOME_SNAPSHOTS_NOT_IMPORTED
//
// MessageText:
//
// Some shadow copies were not successfully imported.
//
#define VSS_E_SOME_SNAPSHOTS_NOT_IMPORTED ((HRESULT)0x80042321L)

//
// MessageId: VSS_E_MAXIMUM_NUMBER_OF_REMOTE_MACHINES_REACHED
//
// MessageText:
//
// The maximum number of remote machines for this operation has been reached.
//
#define VSS_E_MAXIMUM_NUMBER_OF_REMOTE_MACHINES_REACHED ((HRESULT)0x80042322L)

//
// MessageId: VSS_E_REMOTE_SERVER_UNAVAILABLE
//
// MessageText:
//
// The remote server is unavailable.
//
#define VSS_E_REMOTE_SERVER_UNAVAILABLE  ((HRESULT)0x80042323L)

//
// MessageId: VSS_E_REMOTE_SERVER_UNSUPPORTED
//
// MessageText:
//
// The remote server is running a version of the Volume Shadow Copy Service that does not
// support remote shadow-copy creation.
//
#define VSS_E_REMOTE_SERVER_UNSUPPORTED  ((HRESULT)0x80042324L)

//
// MessageId: VSS_E_REVERT_IN_PROGRESS
//
// MessageText:
//
// A revert is currently in progress for the specified volume.  Another revert
// cannot be initiated until the current revert completes.
//
#define VSS_E_REVERT_IN_PROGRESS         ((HRESULT)0x80042325L)

//
// MessageId: VSS_E_REVERT_VOLUME_LOST
//
// MessageText:
//
// The volume being reverted was lost during revert.
//
#define VSS_E_REVERT_VOLUME_LOST         ((HRESULT)0x80042326L)

//
// MessageId: VSS_E_REBOOT_REQUIRED
//
// MessageText:
//
// A reboot is required after completing this operation.
//
#define VSS_E_REBOOT_REQUIRED            ((HRESULT)0x80042327L)

//
// MessageId: VSS_E_TRANSACTION_FREEZE_TIMEOUT
//
// MessageText:
//
// A timeout occurred while freezing a transaction manager.
//
#define VSS_E_TRANSACTION_FREEZE_TIMEOUT ((HRESULT)0x80042328L)

//
// MessageId: VSS_E_TRANSACTION_THAW_TIMEOUT
//
// MessageText:
//
// Too much time elapsed between freezing a transaction manager and thawing
// the transaction manager.
//
#define VSS_E_TRANSACTION_THAW_TIMEOUT   ((HRESULT)0x80042329L)

//
// MessageId: VSS_E_VOLUME_NOT_LOCAL
//
// MessageText:
//
// The volume being backed up is not mounted on the local host.
//
#define VSS_E_VOLUME_NOT_LOCAL           ((HRESULT)0x8004232DL)

//
// MessageId: VSS_E_CLUSTER_TIMEOUT
//
// MessageText:
//
// A timeout occurred while preparing a cluster shared volume for backup.
//
#define VSS_E_CLUSTER_TIMEOUT            ((HRESULT)0x8004232EL)

//
// MessageId: VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT
//
// MessageText:
//
// The shadow-copy set only contains only a subset of the
// volumes needed to correctly backup the selected components
// of the writer.
//
#define VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT ((HRESULT)0x800423F0L)

//
// MessageId: VSS_E_WRITERERROR_OUTOFRESOURCES
//
// MessageText:
//
// A resource allocation failed while processing this operation.
//
#define VSS_E_WRITERERROR_OUTOFRESOURCES ((HRESULT)0x800423F1L)

//
// MessageId: VSS_E_WRITERERROR_TIMEOUT
//
// MessageText:
//
// The writer's timeout expired between the Freeze and Thaw events.
//
#define VSS_E_WRITERERROR_TIMEOUT        ((HRESULT)0x800423F2L)

//
// MessageId: VSS_E_WRITERERROR_RETRYABLE
//
// MessageText:
//
// The writer experienced a transient error.  If the backup process is retried,
// the error may not reoccur.
//
#define VSS_E_WRITERERROR_RETRYABLE      ((HRESULT)0x800423F3L)

//
// MessageId: VSS_E_WRITERERROR_NONRETRYABLE
//
// MessageText:
//
// The writer experienced a non-transient error.  If the backup process is retried,
// the error is likely to reoccur.
//
#define VSS_E_WRITERERROR_NONRETRYABLE   ((HRESULT)0x800423F4L)

//
// MessageId: VSS_E_WRITERERROR_RECOVERY_FAILED
//
// MessageText:
//
// The writer experienced an error while trying to recover the shadow-copy volume.
//
#define VSS_E_WRITERERROR_RECOVERY_FAILED ((HRESULT)0x800423F5L)

//
// MessageId: VSS_E_BREAK_REVERT_ID_FAILED
//
// MessageText:
//
// The shadow copy set break operation failed because the disk/partition identities could not be reverted. The target identity already exists on the machine or cluster and must be masked before this operation can succeed.
//
#define VSS_E_BREAK_REVERT_ID_FAILED     ((HRESULT)0x800423F6L)

//
// MessageId: VSS_E_LEGACY_PROVIDER
//
// MessageText:
//
// This version of the hardware provider does not support this operation.
//
#define VSS_E_LEGACY_PROVIDER            ((HRESULT)0x800423F7L)

//
// MessageId: VSS_E_MISSING_DISK
//
// MessageText:
//
// An expected disk did not arrive in the system.
//
#define VSS_E_MISSING_DISK               ((HRESULT)0x800423F8L)

//
// MessageId: VSS_E_MISSING_HIDDEN_VOLUME
//
// MessageText:
//
// An expected hidden volume did not arrive in the system. Check the Application event log for more information.
//
#define VSS_E_MISSING_HIDDEN_VOLUME      ((HRESULT)0x800423F9L)

//
// MessageId: VSS_E_MISSING_VOLUME
//
// MessageText:
//
// An expected volume did not arrive in the system. Check the Application event log for more information.
//
#define VSS_E_MISSING_VOLUME             ((HRESULT)0x800423FAL)

//
// MessageId: VSS_E_AUTORECOVERY_FAILED
//
// MessageText:
//
// The autorecovery operation failed to complete on the shadow copy.
//
#define VSS_E_AUTORECOVERY_FAILED        ((HRESULT)0x800423FBL)

//
// MessageId: VSS_E_DYNAMIC_DISK_ERROR
//
// MessageText:
//
// An error occurred in processing the dynamic disks involved in the operation.
//
#define VSS_E_DYNAMIC_DISK_ERROR         ((HRESULT)0x800423FCL)

//
// MessageId: VSS_E_NONTRANSPORTABLE_BCD
//
// MessageText:
//
// The given Backup Components Document is for a non-transportable shadow copy. This operation can only be done on transportable shadow copies.
//
#define VSS_E_NONTRANSPORTABLE_BCD       ((HRESULT)0x800423FDL)

//
// MessageId: VSS_E_CANNOT_REVERT_DISKID
//
// MessageText:
//
// The MBR signature or GPT ID for one or more disks could not be set to the intended value. Check the Application event log for more information.
//
#define VSS_E_CANNOT_REVERT_DISKID       ((HRESULT)0x800423FEL)

//
// MessageId: VSS_E_RESYNC_IN_PROGRESS
//
// MessageText:
//
// The LUN resynchronization operation could not be started because another resynchronization operation is already in progress.
//
#define VSS_E_RESYNC_IN_PROGRESS         ((HRESULT)0x800423FFL)

//
// MessageId: VSS_E_CLUSTER_ERROR
//
// MessageText:
//
// The clustered disks could not be enumerated or could not be put into cluster maintenance mode. Check the System event log for cluster related events and the Application event log for VSS related events.
//
#define VSS_E_CLUSTER_ERROR              ((HRESULT)0x80042400L)

//
// MessageId: VSS_E_UNSELECTED_VOLUME
//
// MessageText:
//
// The requested operation would overwrite a volume that is not explicitly selected. For more information, check the Application event log.
//
#define VSS_E_UNSELECTED_VOLUME          ((HRESULT)0x8004232AL)

//
// MessageId: VSS_E_SNAPSHOT_NOT_IN_SET
//
// MessageText:
//
// The shadow copy ID was not found in the backup components document for the shadow copy set.
//
#define VSS_E_SNAPSHOT_NOT_IN_SET        ((HRESULT)0x8004232BL)

//
// MessageId: VSS_E_NESTED_VOLUME_LIMIT
//
// MessageText:
//
// The specified volume is nested too deeply to participate in the VSS operation.
//
#define VSS_E_NESTED_VOLUME_LIMIT        ((HRESULT)0x8004232CL)

//
// MessageId: VSS_E_NOT_SUPPORTED
//
// MessageText:
//
// The requested operation is not supported.
//
#define VSS_E_NOT_SUPPORTED              ((HRESULT)0x8004232FL)

//
// MessageId: VSS_E_WRITERERROR_PARTIAL_FAILURE
//
// MessageText:
//
// The writer experienced a partial failure. Check the component level error state for more information.
//
#define VSS_E_WRITERERROR_PARTIAL_FAILURE ((HRESULT)0x80042336L)

 //
 // ASR error codes
 //
//
// MessageId: VSS_E_ASRERROR_DISK_ASSIGNMENT_FAILED
//
// MessageText:
//
// There are too few disks on this computer or one or more of the disks is too small. Add or change disks so they match the disks in the backup, and try the restore again.
//
#define VSS_E_ASRERROR_DISK_ASSIGNMENT_FAILED ((HRESULT)0x80042401L)

//
// MessageId: VSS_E_ASRERROR_DISK_RECREATION_FAILED
//
// MessageText:
//
// Windows cannot create a disk on this computer needed to restore from the backup. Make sure the disks are properly connected, or add or change disks, and try the restore again.
//
#define VSS_E_ASRERROR_DISK_RECREATION_FAILED ((HRESULT)0x80042402L)

//
// MessageId: VSS_E_ASRERROR_NO_ARCPATH
//
// MessageText:
//
// The computer needs to be restarted to finish preparing a hard disk for restore. To continue, restart your computer and run the restore again.
//
#define VSS_E_ASRERROR_NO_ARCPATH        ((HRESULT)0x80042403L)

//
// MessageId: VSS_E_ASRERROR_MISSING_DYNDISK
//
// MessageText:
//
// The backup failed due to a missing disk for a dynamic volume. Ensure the disk is online and retry the backup.
//
#define VSS_E_ASRERROR_MISSING_DYNDISK   ((HRESULT)0x80042404L)

//
// MessageId: VSS_E_ASRERROR_SHARED_CRIDISK
//
// MessageText:
//
// Automated System Recovery failed the shadow copy, because a selected critical volume is located on a cluster shared disk. This is an unsupported configuration.
//
#define VSS_E_ASRERROR_SHARED_CRIDISK    ((HRESULT)0x80042405L)

//
// MessageId: VSS_E_ASRERROR_DATADISK_RDISK0
//
// MessageText:
//
// A data disk is currently set as active in BIOS. Set some other disk as active or use the DiskPart utility to clean the data disk, and then retry the restore operation.
//
#define VSS_E_ASRERROR_DATADISK_RDISK0   ((HRESULT)0x80042406L)

//
// MessageId: VSS_E_ASRERROR_RDISK0_TOOSMALL
//
// MessageText:
//
// The disk that is set as active in BIOS is too small to recover the original system disk. Replace the disk with a larger one and retry the restore operation.
//
#define VSS_E_ASRERROR_RDISK0_TOOSMALL   ((HRESULT)0x80042407L)

//
// MessageId: VSS_E_ASRERROR_CRITICAL_DISKS_TOO_SMALL
//
// MessageText:
//
// Failed to find enough suitable disks for recreating all critical disks. The number of available disks should be same or greater than the number of critical disks at time of backup, and each one of the disks must be of same or greater size.
//
#define VSS_E_ASRERROR_CRITICAL_DISKS_TOO_SMALL ((HRESULT)0x80042408L)

//
// MessageId: VSS_E_WRITER_STATUS_NOT_AVAILABLE
//
// MessageText:
//
// Writer status is not available for one or more writers.  A writer may have reached the limit to the number of available backup-restore session states.
//
#define VSS_E_WRITER_STATUS_NOT_AVAILABLE ((HRESULT)0x80042409L)

//
// MessageId: VSS_E_ASRERROR_DYNAMIC_VHD_NOT_SUPPORTED
//
// MessageText:
//
// A critical dynamic disk is a Virtual Hard Disk (VHD). This is an unsupported configuration. Check the Application event log for more details.
//
#define VSS_E_ASRERROR_DYNAMIC_VHD_NOT_SUPPORTED ((HRESULT)0x8004240AL)

//
// MessageId: VSS_E_CRITICAL_VOLUME_ON_INVALID_DISK
//
// MessageText:
//
// A critical volume selected for backup exists on a disk which cannot be backed up by ASR.
//
#define VSS_E_CRITICAL_VOLUME_ON_INVALID_DISK ((HRESULT)0x80042411L)

//
// MessageId: VSS_E_ASRERROR_RDISK_FOR_SYSTEM_DISK_NOT_FOUND
//
// MessageText:
//
// No disk that can be used for recovering the system disk can be found.
// Try the following:
// 1) A probable system disk may have been excluded by mistake.
//     a.  Review the list of disks that you have excluded from the recovery
//         for a likely disk.
//     b.  Type LIST DISK command in the DISKPART command interpreter. The
//         probable system disk is usually the first disk listed in the results.
//     c.  If possible, remove the disk from the exclusion list and then retry
//        the recovery.
// 2) A USB disk may have been assigned as a system disk.
//     a.  Detach all USB disks from the computer.
//     b.  Reboot into Windows Recovery Environment (Win RE),
//         then reattach USB disks and retry the recovery.
// 3) An invalid disk may have been assigned as system disk.
//     a.  Physically detach the disk from your computer. Then boot
//         into Win RE to retry the recovery.
//
#define VSS_E_ASRERROR_RDISK_FOR_SYSTEM_DISK_NOT_FOUND ((HRESULT)0x80042412L)

//
// MessageId: VSS_E_ASRERROR_NO_PHYSICAL_DISK_AVAILABLE
//
// MessageText:
//
// Windows did not find any fixed disk that can be used to recreate volumes present in backup. Ensure disks are online, and disk drivers are installed to access the disk(s). 'diskpart.exe' tool with list disks command can be used to see the list of available fixed disks on the system.
//
#define VSS_E_ASRERROR_NO_PHYSICAL_DISK_AVAILABLE ((HRESULT)0x80042413L)

//
// MessageId: VSS_E_ASRERROR_FIXED_PHYSICAL_DISK_AVAILABLE_AFTER_DISK_EXCLUSION
//
// MessageText:
//
// Windows did not find any disk which it can use for recreating volumes present in backup. Offline disks, cluster shared disks or disks explicitly excluded by user will not be used by Windows. Ensure that disks are online and no disks are excluded by mistake.
//
#define VSS_E_ASRERROR_FIXED_PHYSICAL_DISK_AVAILABLE_AFTER_DISK_EXCLUSION ((HRESULT)0x80042414L)

//
// MessageId: VSS_E_ASRERROR_CRITICAL_DISK_CANNOT_BE_EXCLUDED
//
// MessageText:
//
// Restore failed because a disk which was critical at backup is excluded. To continue you need to either remove the disk from exclusion list or detach it from machine or clean it using DiskPart utility, and then retry restore. If you cannot clean or detach it then change the disk signature using DiskPart command UNIQUEID DISK ID.
//
#define VSS_E_ASRERROR_CRITICAL_DISK_CANNOT_BE_EXCLUDED ((HRESULT)0x80042415L)

//
// MessageId: VSS_E_ASRERROR_SYSTEM_PARTITION_HIDDEN
//
// MessageText:
//
// System partition (partition marked "active") is hidden or contains an unrecognized file system. Backup does not support this configuration.
//
#define VSS_E_ASRERROR_SYSTEM_PARTITION_HIDDEN ((HRESULT)0x80042416L)

//
// MessageId: VSS_E_FSS_TIMEOUT
//
// MessageText:
//
// A timeout occurred while preparing a file share shadowcopy for backup.
//
#define VSS_E_FSS_TIMEOUT                ((HRESULT)0x80042417L)

#endif // _ERRORS_H_
