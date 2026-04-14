/*++

Copyright (c) Microsoft Corporation

Module Name:

    vdserr.h

Abstract:

    Constant definitions for the Virtual Disk Service error messages.

    Error code values:
    Added in Windows Server 2003:          0x2400-0x24FF
    Added in Windows Vista:                0x2500-0x26FF
    Added in VDS 1.1:                      0x2700-0x27FF
    Added in Windows Server 2008:          0x2800-0x28FF
    Added in Win7:                         0x2900-0x29FF
    Added in Win8:                         0x2A00-0x2AFF

    NOTE:
    If porting over an error code which was added to Server 2003 SP make sure
    the error code's value is in the designated range for Windows Server 2003.
    If the value is not in the designated range make sure it does not conflict
    with any of the already defined errors and if it does change the value
    to the next available Vista value.

Author:

    Kevin Seng   [KSeng]    04/01/05

Revision History:

--*/
#pragma once
// HRESULT codes
//////////////////////////////////////////////////////////////////////////////
//
// Codes added in Windows Server 2003:           0x2400-0x24FF
//
//////////////////////////////////////////////////////////////////////////////
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
// MessageId: VDS_E_NOT_SUPPORTED
//
// MessageText:
//
// The operation is not supported by the object.
//
#define VDS_E_NOT_SUPPORTED              ((HRESULT)0x80042400L)

//
// MessageId: VDS_E_INITIALIZED_FAILED
//
// MessageText:
//
// The service failed to initialize.
//
#define VDS_E_INITIALIZED_FAILED         ((HRESULT)0x80042401L)

//
// MessageId: VDS_E_INITIALIZE_NOT_CALLED
//
// MessageText:
//
// The initialization method was not called.
//
#define VDS_E_INITIALIZE_NOT_CALLED      ((HRESULT)0x80042402L)

//
// MessageId: VDS_E_ALREADY_REGISTERED
//
// MessageText:
//
// The provider is already registered.
//
#define VDS_E_ALREADY_REGISTERED         ((HRESULT)0x80042403L)

//
// MessageId: VDS_E_ANOTHER_CALL_IN_PROGRESS
//
// MessageText:
//
// A concurrent second call is made on an object before the first is completed.
//
#define VDS_E_ANOTHER_CALL_IN_PROGRESS   ((HRESULT)0x80042404L)

//
// MessageId: VDS_E_OBJECT_NOT_FOUND
//
// MessageText:
//
// The object is not found.
//
#define VDS_E_OBJECT_NOT_FOUND           ((HRESULT)0x80042405L)

//
// MessageId: VDS_E_INVALID_SPACE
//
// MessageText:
//
// The specified space is not free or not valid.
//
#define VDS_E_INVALID_SPACE              ((HRESULT)0x80042406L)

//
// MessageId: VDS_E_PARTITION_LIMIT_REACHED
//
// MessageText:
//
// The number of partitions has reached the limit on a disk.
//
#define VDS_E_PARTITION_LIMIT_REACHED    ((HRESULT)0x80042407L)

//
// MessageId: VDS_E_PARTITION_NOT_EMPTY
//
// MessageText:
//
// The extended partition is not empty.
//
#define VDS_E_PARTITION_NOT_EMPTY        ((HRESULT)0x80042408L)

//
// MessageId: VDS_E_OPERATION_PENDING
//
// MessageText:
//
// The operation has not been completed yet.
//
#define VDS_E_OPERATION_PENDING          ((HRESULT)0x80042409L)

//
// MessageId: VDS_E_OPERATION_DENIED
//
// MessageText:
//
// This operation is not allowed on the current boot, system or pagefile 
// volume. It is also not allowed on any volume on a basic MBR disk that contains 
// the boot, system or pagefile volume. 
//
#define VDS_E_OPERATION_DENIED           ((HRESULT)0x8004240AL)

//
// MessageId: VDS_E_OBJECT_DELETED
//
// MessageText:
//
// The object has been deleted.
//
#define VDS_E_OBJECT_DELETED             ((HRESULT)0x8004240BL)

//
// MessageId: VDS_E_CANCEL_TOO_LATE
//
// MessageText:
//
// The operation cannot be cancelled.
//
#define VDS_E_CANCEL_TOO_LATE            ((HRESULT)0x8004240CL)

//
// MessageId: VDS_E_OPERATION_CANCELED
//
// MessageText:
//
// The operation has been cancelled.
//
#define VDS_E_OPERATION_CANCELED         ((HRESULT)0x8004240DL)

//
// MessageId: VDS_E_CANNOT_EXTEND
//
// MessageText:
//
// The volume cannot be extended because the file system does not support it.
//
#define VDS_E_CANNOT_EXTEND              ((HRESULT)0x8004240EL)

//
// MessageId: VDS_E_NOT_ENOUGH_SPACE
//
// MessageText:
//
// There is not enough usable space for this operation.
//
#define VDS_E_NOT_ENOUGH_SPACE           ((HRESULT)0x8004240FL)

//
// MessageId: VDS_E_NOT_ENOUGH_DRIVE
//
// MessageText:
//
// Not enough drives are specified to complete this operation.
//
#define VDS_E_NOT_ENOUGH_DRIVE           ((HRESULT)0x80042410L)

//
// MessageId: VDS_E_BAD_COOKIE
//
// MessageText:
//
// The cookie is not found.
//
#define VDS_E_BAD_COOKIE                 ((HRESULT)0x80042411L)

//
// MessageId: VDS_E_NO_MEDIA
//
// MessageText:
//
// There is no media in the device.
//
#define VDS_E_NO_MEDIA                   ((HRESULT)0x80042412L)

//
// MessageId: VDS_E_DEVICE_IN_USE
//
// MessageText:
//
// The device is in use.
//
#define VDS_E_DEVICE_IN_USE              ((HRESULT)0x80042413L)

//
// MessageId: VDS_E_DISK_NOT_EMPTY
//
// MessageText:
//
// The disk is not empty.
//
#define VDS_E_DISK_NOT_EMPTY             ((HRESULT)0x80042414L)

//
// MessageId: VDS_E_INVALID_OPERATION
//
// MessageText:
//
// Invalid operation.
//
#define VDS_E_INVALID_OPERATION          ((HRESULT)0x80042415L)

//
// MessageId: VDS_E_PATH_NOT_FOUND
//
// MessageText:
//
// The path is not found.
//
#define VDS_E_PATH_NOT_FOUND             ((HRESULT)0x80042416L)

//
// MessageId: VDS_E_DISK_NOT_INITIALIZED
//
// MessageText:
//
// The disk is not initialized.
//
#define VDS_E_DISK_NOT_INITIALIZED       ((HRESULT)0x80042417L)

//
// MessageId: VDS_E_NOT_AN_UNALLOCATED_DISK
//
// MessageText:
//
// The disk is not unallocated.
//
#define VDS_E_NOT_AN_UNALLOCATED_DISK    ((HRESULT)0x80042418L)

//
// MessageId: VDS_E_UNRECOVERABLE_ERROR
//
// MessageText:
//
// An unrecoverable error occurred. The service must shut down.
//
#define VDS_E_UNRECOVERABLE_ERROR        ((HRESULT)0x80042419L)

//
// MessageId: VDS_S_DISK_PARTIALLY_CLEANED
//
// MessageText:
//
// The disk is not fully cleaned due to I/O error.
//
#define VDS_S_DISK_PARTIALLY_CLEANED     ((HRESULT)0x0004241AL)

//
// MessageId: VDS_E_DMADMIN_SERVICE_CONNECTION_FAILED
//
// MessageText:
//
// The provider failed to connect to the Logical Disk Management 
// Administrative service.
//
#define VDS_E_DMADMIN_SERVICE_CONNECTION_FAILED ((HRESULT)0x8004241BL)

//
// MessageId: VDS_E_PROVIDER_INITIALIZATION_FAILED
//
// MessageText:
//
// The provider failed to initialize.
//
#define VDS_E_PROVIDER_INITIALIZATION_FAILED ((HRESULT)0x8004241CL)

//
// MessageId: VDS_E_OBJECT_EXISTS
//
// MessageText:
//
// The object already exists.
//
#define VDS_E_OBJECT_EXISTS              ((HRESULT)0x8004241DL)

//
// MessageId: VDS_E_NO_DISKS_FOUND
//
// MessageText:
//
// No disks were found on the target machine.
//
#define VDS_E_NO_DISKS_FOUND             ((HRESULT)0x8004241EL)

//
// MessageId: VDS_E_PROVIDER_CACHE_CORRUPT
//
// MessageText:
//
// The provider's cache has become corrupt.
//
#define VDS_E_PROVIDER_CACHE_CORRUPT     ((HRESULT)0x8004241FL)

//
// MessageId: VDS_E_DMADMIN_METHOD_CALL_FAILED
//
// MessageText:
//
// A method call to the Logical Disk Management Administrative service failed.
//
#define VDS_E_DMADMIN_METHOD_CALL_FAILED ((HRESULT)0x80042420L)

//
// MessageId: VDS_S_PROVIDER_ERROR_LOADING_CACHE
//
// MessageText:
//
// The provider encountered errors while loading the cache. 
// See the NT Event Log for more information.
//
#define VDS_S_PROVIDER_ERROR_LOADING_CACHE ((HRESULT)0x00042421L)

//
// MessageId: VDS_E_PROVIDER_VOL_DEVICE_NAME_NOT_FOUND
//
// MessageText:
//
// The device form of the volume pathname could not be retrieved.
//
#define VDS_E_PROVIDER_VOL_DEVICE_NAME_NOT_FOUND ((HRESULT)0x80042422L)

//
// MessageId: VDS_E_PROVIDER_VOL_OPEN
//
// MessageText:
//
// Failed to open the volume device.
//
#define VDS_E_PROVIDER_VOL_OPEN          ((HRESULT)0x80042423L)

//
// MessageId: VDS_E_DMADMIN_CORRUPT_NOTIFICATION
//
// MessageText:
//
// A corrupt notification was sent from the Logical Disk Manager 
// Administrative service.
//
#define VDS_E_DMADMIN_CORRUPT_NOTIFICATION ((HRESULT)0x80042424L)

//
// MessageId: VDS_E_INCOMPATIBLE_FILE_SYSTEM
//
// MessageText:
//
// The file system is incompatible.
//
#define VDS_E_INCOMPATIBLE_FILE_SYSTEM   ((HRESULT)0x80042425L)

//
// MessageId: VDS_E_INCOMPATIBLE_MEDIA
//
// MessageText:
//
// The media is incompatible.
//
#define VDS_E_INCOMPATIBLE_MEDIA         ((HRESULT)0x80042426L)

//
// MessageId: VDS_E_ACCESS_DENIED
//
// MessageText:
//
// Access is denied.
//
#define VDS_E_ACCESS_DENIED              ((HRESULT)0x80042427L)

//
// MessageId: VDS_E_MEDIA_WRITE_PROTECTED
//
// MessageText:
//
// The media is write protected.
//
#define VDS_E_MEDIA_WRITE_PROTECTED      ((HRESULT)0x80042428L)

//
// MessageId: VDS_E_BAD_LABEL
//
// MessageText:
//
// The label is illegal.
//
#define VDS_E_BAD_LABEL                  ((HRESULT)0x80042429L)

//
// MessageId: VDS_E_CANT_QUICK_FORMAT
//
// MessageText:
//
// Can not quick format the volume.
//
#define VDS_E_CANT_QUICK_FORMAT          ((HRESULT)0x8004242AL)

//
// MessageId: VDS_E_IO_ERROR
//
// MessageText:
//
// IO error occured during format.
//
#define VDS_E_IO_ERROR                   ((HRESULT)0x8004242BL)

//
// MessageId: VDS_E_VOLUME_TOO_SMALL
//
// MessageText:
//
// The volume size is too small.
//
#define VDS_E_VOLUME_TOO_SMALL           ((HRESULT)0x8004242CL)

//
// MessageId: VDS_E_VOLUME_TOO_BIG
//
// MessageText:
//
// The volume size is too big.
//
#define VDS_E_VOLUME_TOO_BIG             ((HRESULT)0x8004242DL)

//
// MessageId: VDS_E_CLUSTER_SIZE_TOO_SMALL
//
// MessageText:
//
// The cluster size is too small.
//
#define VDS_E_CLUSTER_SIZE_TOO_SMALL     ((HRESULT)0x8004242EL)

//
// MessageId: VDS_E_CLUSTER_SIZE_TOO_BIG
//
// MessageText:
//
// The cluster size is too big.
//
#define VDS_E_CLUSTER_SIZE_TOO_BIG       ((HRESULT)0x8004242FL)

//
// MessageId: VDS_E_CLUSTER_COUNT_BEYOND_32BITS
//
// MessageText:
//
// The number of clusters is too big for 32 bit integer.
//
#define VDS_E_CLUSTER_COUNT_BEYOND_32BITS ((HRESULT)0x80042430L)

//
// MessageId: VDS_E_OBJECT_STATUS_FAILED
//
// MessageText:
//
// The object is in failed status.
//
#define VDS_E_OBJECT_STATUS_FAILED       ((HRESULT)0x80042431L)

//
// MessageId: VDS_E_VOLUME_INCOMPLETE
//
// MessageText:
//
// All extents for the volume could not be found.
//
#define VDS_E_VOLUME_INCOMPLETE          ((HRESULT)0x80042432L)

//
// MessageId: VDS_E_EXTENT_SIZE_LESS_THAN_MIN
//
// MessageText:
//
// The size of the extent is less than the minimum.
//
#define VDS_E_EXTENT_SIZE_LESS_THAN_MIN  ((HRESULT)0x80042433L)

//
// MessageId: VDS_S_UPDATE_BOOTFILE_FAILED
//
// MessageText:
//
// Failed to update the boot configuration data.
//
#define VDS_S_UPDATE_BOOTFILE_FAILED     ((HRESULT)0x00042434L)

//
// MessageId: VDS_S_BOOT_PARTITION_NUMBER_CHANGE
//
// MessageText:
//
// The boot partition's partition number will change as a result of the operation.
//
#define VDS_S_BOOT_PARTITION_NUMBER_CHANGE ((HRESULT)0x00042436L)

//
// MessageId: VDS_E_BOOT_PARTITION_NUMBER_CHANGE
//
// MessageText:
//
// The boot partition's partition number will change as a result of the 
// migration operation.
//
#define VDS_E_BOOT_PARTITION_NUMBER_CHANGE ((HRESULT)0x80042436L)

//
// MessageId: VDS_E_NO_FREE_SPACE
//
// MessageText:
//
// The selected disk does not have enough free space to complete the operation.
//
#define VDS_E_NO_FREE_SPACE              ((HRESULT)0x80042437L)

//
// MessageId: VDS_E_ACTIVE_PARTITION
//
// MessageText:
//
// An active partition was detected on the selected disk, and it is not the active partition used to boot the currently running OS.
//
#define VDS_E_ACTIVE_PARTITION           ((HRESULT)0x80042438L)

//
// MessageId: VDS_E_PARTITION_OF_UNKNOWN_TYPE
//
// MessageText:
//
// Cannot read partition information.
//
#define VDS_E_PARTITION_OF_UNKNOWN_TYPE  ((HRESULT)0x80042439L)

//
// MessageId: VDS_E_LEGACY_VOLUME_FORMAT
//
// MessageText:
//
// A partition with an unknown type was detected on the selected disk.
//
#define VDS_E_LEGACY_VOLUME_FORMAT       ((HRESULT)0x8004243AL)

//
// MessageId: VDS_E_NON_CONTIGUOUS_DATA_PARTITIONS
//
// MessageText:
//
// The selected GPT formatted disk contains a partition which is not of type 
// 'PARTITION_BASIC_DATA_GUID', and is both preceeded and followed by a partition 
// of type 'PARTITION_BASIC_DATA_GUID'.
//
#define VDS_E_NON_CONTIGUOUS_DATA_PARTITIONS ((HRESULT)0x8004243BL)

//
// MessageId: VDS_E_MIGRATE_OPEN_VOLUME
//
// MessageText:
//
// A volume on the selected disk could not be opened.
//
#define VDS_E_MIGRATE_OPEN_VOLUME        ((HRESULT)0x8004243CL)

//
// MessageId: VDS_E_VOLUME_NOT_ONLINE
//
// MessageText:
//
// The volume is not online.
//
#define VDS_E_VOLUME_NOT_ONLINE          ((HRESULT)0x8004243DL)

//
// MessageId: VDS_E_VOLUME_NOT_HEALTHY
//
// MessageText:
//
// The volume is not healthy.
//
#define VDS_E_VOLUME_NOT_HEALTHY         ((HRESULT)0x8004243EL)

//
// MessageId: VDS_E_VOLUME_SPANS_DISKS
//
// MessageText:
//
// The volume spans multiple disks.
//
#define VDS_E_VOLUME_SPANS_DISKS         ((HRESULT)0x8004243FL)

//
// MessageId: VDS_E_REQUIRES_CONTIGUOUS_DISK_SPACE
//
// MessageText:
//
// The volume requires contiguous disk extents.
//
#define VDS_E_REQUIRES_CONTIGUOUS_DISK_SPACE ((HRESULT)0x80042440L)

//
// MessageId: VDS_E_BAD_PROVIDER_DATA
//
// MessageText:
//
// A provider returned bad data.
//
#define VDS_E_BAD_PROVIDER_DATA          ((HRESULT)0x80042441L)

//
// MessageId: VDS_E_PROVIDER_FAILURE
//
// MessageText:
//
// A provider failed to complete an operation.
//
#define VDS_E_PROVIDER_FAILURE           ((HRESULT)0x80042442L)

//
// MessageId: VDS_S_VOLUME_COMPRESS_FAILED
//
// MessageText:
//
// Failed to compress the volume.
//
#define VDS_S_VOLUME_COMPRESS_FAILED     ((HRESULT)0x00042443L)

//
// MessageId: VDS_E_PACK_OFFLINE
//
// MessageText:
//
// The pack is not online.
//
#define VDS_E_PACK_OFFLINE               ((HRESULT)0x80042444L)

//
// MessageId: VDS_E_VOLUME_NOT_A_MIRROR
//
// MessageText:
//
// The volume is not a mirror.
//
#define VDS_E_VOLUME_NOT_A_MIRROR        ((HRESULT)0x80042445L)

//
// MessageId: VDS_E_NO_EXTENTS_FOR_VOLUME
//
// MessageText:
//
// No extents were found for the volume.
//
#define VDS_E_NO_EXTENTS_FOR_VOLUME      ((HRESULT)0x80042446L)

//
// MessageId: VDS_E_DISK_NOT_LOADED_TO_CACHE
//
// MessageText:
//
// The disk failed to load to the cache.
//
#define VDS_E_DISK_NOT_LOADED_TO_CACHE   ((HRESULT)0x80042447L)

//
// MessageId: VDS_E_INTERNAL_ERROR
//
// MessageText:
//
// Check the event log for errors.
//
#define VDS_E_INTERNAL_ERROR             ((HRESULT)0x80042448L)

//
// MessageId: VDS_S_ACCESS_PATH_NOT_DELETED
//
// MessageText:
//
// The access paths on the volume may not be deleted.
//
#define VDS_S_ACCESS_PATH_NOT_DELETED    ((HRESULT)0x00044244L)

//
// MessageId: VDS_E_PROVIDER_TYPE_NOT_SUPPORTED
//
// MessageText:
//
// The method call is not supported for the specified provider type.
//
#define VDS_E_PROVIDER_TYPE_NOT_SUPPORTED ((HRESULT)0x8004244AL)

//
// MessageId: VDS_E_DISK_NOT_ONLINE
//
// MessageText:
//
// This disk is already offline. 
//
#define VDS_E_DISK_NOT_ONLINE            ((HRESULT)0x8004244BL)

//
// MessageId: VDS_E_DISK_IN_USE_BY_VOLUME
//
// MessageText:
//
// The disk is already in use by the volume.
//
#define VDS_E_DISK_IN_USE_BY_VOLUME      ((HRESULT)0x8004244CL)

//
// MessageId: VDS_S_IN_PROGRESS
//
// MessageText:
//
// The asynchronous operation is in progress.
//
#define VDS_S_IN_PROGRESS                ((HRESULT)0x0004244DL)

//
// MessageId: VDS_E_ASYNC_OBJECT_FAILURE
//
// MessageText:
//
// Failure initializing the asynchronous object.
//
#define VDS_E_ASYNC_OBJECT_FAILURE       ((HRESULT)0x8004244EL)

//
// MessageId: VDS_E_VOLUME_NOT_MOUNTED
//
// MessageText:
//
// A file system is not mounted on the volume.
//
#define VDS_E_VOLUME_NOT_MOUNTED         ((HRESULT)0x8004244FL)

//
// MessageId: VDS_E_PACK_NOT_FOUND
//
// MessageText:
//
// The pack was not found.
//
#define VDS_E_PACK_NOT_FOUND             ((HRESULT)0x80042450L)

//
// MessageId: VDS_E_IMPORT_SET_INCOMPLETE
//
// MessageText:
//
// Import failed. Attempt to import a subset of the disks in the foreign pack.
//
#define VDS_E_IMPORT_SET_INCOMPLETE      ((HRESULT)0x80042451L)

//
// MessageId: VDS_E_DISK_NOT_IMPORTED
//
// MessageText:
//
// A disk in the import's source pack was not imported.
//
#define VDS_E_DISK_NOT_IMPORTED          ((HRESULT)0x80042452L)

//
// MessageId: VDS_E_OBJECT_OUT_OF_SYNC
//
// MessageText:
//
// The system's information about the object may not be up to date.
//
#define VDS_E_OBJECT_OUT_OF_SYNC         ((HRESULT)0x80042453L)

//
// MessageId: VDS_E_MISSING_DISK
//
// MessageText:
//
// The disk is missing.
//
#define VDS_E_MISSING_DISK               ((HRESULT)0x80042454L)

//
// MessageId: VDS_E_DISK_PNP_REG_CORRUPT
//
// MessageText:
//
// The provider's list of Pnp registered disks has become corrupt.
//
#define VDS_E_DISK_PNP_REG_CORRUPT       ((HRESULT)0x80042455L)

//
// MessageId: VDS_E_LBN_REMAP_ENABLED_FLAG
//
// MessageText:
//
// The provider does not support the LBN REMAP ENABLED volume flag.
//
#define VDS_E_LBN_REMAP_ENABLED_FLAG     ((HRESULT)0x80042456L)

//
// MessageId: VDS_E_NO_DRIVELETTER_FLAG
//
// MessageText:
//
// The provider does not support the NO DRIVELETTER volume flag.
//
#define VDS_E_NO_DRIVELETTER_FLAG        ((HRESULT)0x80042457L)

//
// MessageId: VDS_E_REVERT_ON_CLOSE
//
// MessageText:
//
// REVERT ON CLOSE may only be used with GPT flags HIDDEN, READ ONLY, 
// NO DRIVE LETTER or SHADOW COPY.
//
#define VDS_E_REVERT_ON_CLOSE            ((HRESULT)0x80042458L)

//
// MessageId: VDS_E_REVERT_ON_CLOSE_SET
//
// MessageText:
//
// The REVERT ON CLOSE volume flag is already set for this volume.
//
#define VDS_E_REVERT_ON_CLOSE_SET        ((HRESULT)0x80042459L)

//
// MessageId: VDS_E_IA64_BOOT_MIRRORED_TO_MBR
//
// MessageText:
//
// Not used! You have mirrored your boot volume on a GPT disk, to an MBR disk. 
// You will not be able to boot your machine from the secondary plex.
//
#define VDS_E_IA64_BOOT_MIRRORED_TO_MBR  ((HRESULT)0x8004245AL)

//
// MessageId: VDS_S_IA64_BOOT_MIRRORED_TO_MBR
//
// MessageText:
//
// You have mirrored your boot volume on a GPT disk, to an MBR disk. 
// You will not be able to boot your machine from the secondary plex.
//
#define VDS_S_IA64_BOOT_MIRRORED_TO_MBR  ((HRESULT)0x0004245AL)

//
// MessageId: VDS_S_UNABLE_TO_GET_GPT_ATTRIBUTES
//
// MessageText:
//
// Unable to retrieve the GPT attributes for this volume, (hidden, read only 
// and no drive letter).
//
#define VDS_S_UNABLE_TO_GET_GPT_ATTRIBUTES ((HRESULT)0x0004245BL)

//
// MessageId: VDS_E_VOLUME_TEMPORARILY_DISMOUNTED
//
// MessageText:
//
// The volume is temporarily dismounted.
//
#define VDS_E_VOLUME_TEMPORARILY_DISMOUNTED ((HRESULT)0x8004245CL)

//
// MessageId: VDS_E_VOLUME_PERMANENTLY_DISMOUNTED
//
// MessageText:
//
// The volume is permanently dismounted.
//
#define VDS_E_VOLUME_PERMANENTLY_DISMOUNTED ((HRESULT)0x8004245DL)

//
// MessageId: VDS_E_VOLUME_HAS_PATH
//
// MessageText:
//
// The volume still has access path to it.
//
#define VDS_E_VOLUME_HAS_PATH            ((HRESULT)0x8004245EL)

//
// MessageId: VDS_E_TIMEOUT
//
// MessageText:
//
// The operation timed out.
//
#define VDS_E_TIMEOUT                    ((HRESULT)0x8004245FL)

//
// MessageId: VDS_E_REPAIR_VOLUMESTATE
//
// MessageText:
//
// To repair a volume, both the volume and plex must be online, and must not be 
// healthy or rebuilding.
//
#define VDS_E_REPAIR_VOLUMESTATE         ((HRESULT)0x80042460L)

//
// MessageId: VDS_E_LDM_TIMEOUT
//
// MessageText:
//
// The operation timed out in the Logical Disk Manager Administrative service. 
// Retry the operation.
//
#define VDS_E_LDM_TIMEOUT                ((HRESULT)0x80042461L)

//
// MessageId: VDS_E_REVERT_ON_CLOSE_MISMATCH
//
// MessageText:
//
// When clearing volume flags that have been set using revert on close, 
// the same combination of HIDDEN, READ ONLY, NO DRIVE LETTER or SHADOW COPY 
// flags must be passed to both the SetFlags and ClearFlags calls.
//
#define VDS_E_REVERT_ON_CLOSE_MISMATCH   ((HRESULT)0x80042462L)

//
// MessageId: VDS_E_RETRY
//
// MessageText:
//
// The operation failed. Retry the operation.
//
#define VDS_E_RETRY                      ((HRESULT)0x80042463L)

//
// MessageId: VDS_E_ONLINE_PACK_EXISTS
//
// MessageText:
//
// Create pack operation failed. An online pack already exists.
//
#define VDS_E_ONLINE_PACK_EXISTS         ((HRESULT)0x80042464L)

//
// MessageId: VDS_S_EXTEND_FILE_SYSTEM_FAILED
//
// MessageText:
//
// The volume was extended successfully but the file system failed to extend.
//
#define VDS_S_EXTEND_FILE_SYSTEM_FAILED  ((HRESULT)0x00042465L)

//
// MessageId: VDS_E_EXTEND_FILE_SYSTEM_FAILED
//
// MessageText:
//
// The file system failed to extend.
//
#define VDS_E_EXTEND_FILE_SYSTEM_FAILED  ((HRESULT)0x80042466L)

//
// MessageId: VDS_S_MBR_BOOT_MIRRORED_TO_GPT
//
// MessageText:
//
// You have mirrored your boot volume on an MBR disk, to a GPT disk. 
// You will not be able to boot your machine from the secondary plex.
//
#define VDS_S_MBR_BOOT_MIRRORED_TO_GPT   ((HRESULT)0x00042467L)

//
// MessageId: VDS_E_MAX_USABLE_MBR
//
// MessageText:
//
// Only the first 2TB are usable on large MBR disks. 
// Cannot create partitions beyond the 2TB mark, nor convert 
// the disk to dynamic.
//
#define VDS_E_MAX_USABLE_MBR             ((HRESULT)0x80042468L)

//
// MessageId: VDS_S_GPT_BOOT_MIRRORED_TO_MBR
//
// MessageText:
//
// You have mirrored your boot volume on a GPT disk, to an MBR disk. 
// You will not be able to boot your machine from the secondary plex.
//
#define VDS_S_GPT_BOOT_MIRRORED_TO_MBR   ((HRESULT)0x80042469L)

//////////////////////////////////////////////////////////////////////////////
//
// Codes added in Windows Vista:                 0x2500-0x26FF
//
//////////////////////////////////////////////////////////////////////////////
//
// MessageId: VDS_E_NO_SOFTWARE_PROVIDERS_LOADED
//
// MessageText:
//
// There are no software provders loaded.
//
#define VDS_E_NO_SOFTWARE_PROVIDERS_LOADED ((HRESULT)0x80042500L)

//
// MessageId: VDS_E_DISK_NOT_MISSING
//
// MessageText:
//
// The disk is not missing.
//
#define VDS_E_DISK_NOT_MISSING           ((HRESULT)0x80042501L)

//
// MessageId: VDS_E_NO_VOLUME_LAYOUT
//
// MessageText:
//
// Failed to retrieve the volume's layout. Operations on the volume will fail.
//
#define VDS_E_NO_VOLUME_LAYOUT           ((HRESULT)0x80042502L)

//
// MessageId: VDS_E_CORRUPT_VOLUME_INFO
//
// MessageText:
//
// The volume's driver information is corrupt. Operations on the volume will fail.
//
#define VDS_E_CORRUPT_VOLUME_INFO        ((HRESULT)0x80042503L)

//
// MessageId: VDS_E_INVALID_ENUMERATOR
//
// MessageText:
//
// The enumerator is corrupted.
//
#define VDS_E_INVALID_ENUMERATOR         ((HRESULT)0x80042504L)

//
// MessageId: VDS_E_DRIVER_INTERNAL_ERROR
//
// MessageText:
//
// Internal error in the volume management driver.
//
#define VDS_E_DRIVER_INTERNAL_ERROR      ((HRESULT)0x80042505L)

//
// MessageId: VDS_E_VOLUME_INVALID_NAME
//
// MessageText:
//
// The volume name is invalid.
//
#define VDS_E_VOLUME_INVALID_NAME        ((HRESULT)0x80042507L)

//
// MessageId: VDS_S_DISK_IS_MISSING
//
// MessageText:
//
// The disk is missing, not all information could be returned.
//
#define VDS_S_DISK_IS_MISSING            ((HRESULT)0x00042508L)

//
// MessageId: VDS_E_CORRUPT_PARTITION_INFO
//
// MessageText:
//
// The disk's partition information is corrupted.
//
#define VDS_E_CORRUPT_PARTITION_INFO     ((HRESULT)0x80042509L)

//
// MessageId: VDS_S_NONCONFORMANT_PARTITION_INFO
//
// MessageText:
//
// The disk's partition information is does not conform to that expected 
// on a dynamic disk.
//
#define VDS_S_NONCONFORMANT_PARTITION_INFO ((HRESULT)0x0004250AL)

//
// MessageId: VDS_E_CORRUPT_EXTENT_INFO
//
// MessageText:
//
// The disk's extent information is corrupted.
//
#define VDS_E_CORRUPT_EXTENT_INFO        ((HRESULT)0x8004250BL)

//
// MessageId: VDS_E_DUP_EMPTY_PACK_GUID
//
// MessageText:
//
// An empty pack already exists. Release the existing empty pack before 
// creating another empty pack.
//
#define VDS_E_DUP_EMPTY_PACK_GUID        ((HRESULT)0x8004250CL)

//
// MessageId: VDS_E_DRIVER_NO_PACK_NAME
//
// MessageText:
//
// The volume management driver did not return a pack name. 
// Internal driver error.
//
#define VDS_E_DRIVER_NO_PACK_NAME        ((HRESULT)0x8004250DL)

//
// MessageId: VDS_S_SYSTEM_PARTITION
//
// MessageText:
//
// Warning, there was a failure while checking for the system partition.
//
#define VDS_S_SYSTEM_PARTITION           ((HRESULT)0x0004250EL)

//
// MessageId: VDS_E_BAD_PNP_MESSAGE
//
// MessageText:
//
// The PNP service sent a corrupted notification to the provider.
//
#define VDS_E_BAD_PNP_MESSAGE            ((HRESULT)0x8004250FL)

//
// MessageId: VDS_E_NO_PNP_DISK_ARRIVE
//
// MessageText:
//
// No disk arrival notification was received.
//
#define VDS_E_NO_PNP_DISK_ARRIVE         ((HRESULT)0x80042510L)

//
// MessageId: VDS_E_NO_PNP_VOLUME_ARRIVE
//
// MessageText:
//
// No volume arrival notification was received.
//
#define VDS_E_NO_PNP_VOLUME_ARRIVE       ((HRESULT)0x80042511L)

//
// MessageId: VDS_E_NO_PNP_DISK_REMOVE
//
// MessageText:
//
// No disk removal notification was received.
//
#define VDS_E_NO_PNP_DISK_REMOVE         ((HRESULT)0x80042512L)

//
// MessageId: VDS_E_NO_PNP_VOLUME_REMOVE
//
// MessageText:
//
// No volume removal notification was received.
//
#define VDS_E_NO_PNP_VOLUME_REMOVE       ((HRESULT)0x80042513L)

//
// MessageId: VDS_E_PROVIDER_EXITING
//
// MessageText:
//
// The provider is exiting.
//
#define VDS_E_PROVIDER_EXITING           ((HRESULT)0x80042514L)

//
// MessageId: VDS_E_EXTENT_EXCEEDS_DISK_FREE_SPACE
//
// MessageText:
//
// The specified disk extent size exceeds the size of free disk space.
//
#define VDS_E_EXTENT_EXCEEDS_DISK_FREE_SPACE ((HRESULT)0x80042515L)

//
// MessageId: VDS_E_MEMBER_SIZE_INVALID
//
// MessageText:
//
// The size of the plex member is invalid.
//
#define VDS_E_MEMBER_SIZE_INVALID        ((HRESULT)0x80042516L)

//
// MessageId: VDS_S_NO_NOTIFICATION
//
// MessageText:
//
// No volume arrival notification was received. 
// Refresh the Disk Management display.
//
#define VDS_S_NO_NOTIFICATION            ((HRESULT)0x00042517L)

//
// MessageId: VDS_S_DEFAULT_PLEX_MEMBER_IDS
//
// MessageText:
//
// Defaults have been used for the member ids or plex ids.
//
#define VDS_S_DEFAULT_PLEX_MEMBER_IDS    ((HRESULT)0x00042518L)

//
// MessageId: VDS_E_INVALID_DISK
//
// MessageText:
//
// This operation is not allowed on an invalid disk. The disk may be 
// invalid because it is corrupted or failing, or it may be invalid 
// because it is OFFLINE.
//
#define VDS_E_INVALID_DISK               ((HRESULT)0x80042519L)

//
// MessageId: VDS_E_INVALID_PACK
//
// MessageText:
//
// This operation is not allowed on the invalid disk pack.
//
#define VDS_E_INVALID_PACK               ((HRESULT)0x8004251AL)

//
// MessageId: VDS_E_VOLUME_ON_DISK
//
// MessageText:
//
// This operation is not allowed on disks with volumes.
//
#define VDS_E_VOLUME_ON_DISK             ((HRESULT)0x8004251BL)

//
// MessageId: VDS_E_DRIVER_INVALID_PARAM
//
// MessageText:
//
// The driver returned an invalid parameter error.
//
#define VDS_E_DRIVER_INVALID_PARAM       ((HRESULT)0x8004251CL)

//
// MessageId: VDS_E_TARGET_PACK_NOT_EMPTY
//
// MessageText:
//
// The target pack is not empty.
//
#define VDS_E_TARGET_PACK_NOT_EMPTY      ((HRESULT)0x8004251DL)

//
// MessageId: VDS_E_CANNOT_SHRINK
//
// MessageText:
//
// The volume cannot be shrunk because the file system does not support it.
//
#define VDS_E_CANNOT_SHRINK              ((HRESULT)0x8004251EL)

//
// MessageId: VDS_E_MULTIPLE_PACKS
//
// MessageText:
//
// Specified disks are not all from the same pack.
//
#define VDS_E_MULTIPLE_PACKS             ((HRESULT)0x8004251FL)

//
// MessageId: VDS_E_PACK_ONLINE
//
// MessageText:
//
// This operation is not allowed on online packs. The pack must be offline.
//
#define VDS_E_PACK_ONLINE                ((HRESULT)0x80042520L)

//
// MessageId: VDS_E_INVALID_PLEX_COUNT
//
// MessageText:
//
// The plex count for the volume must be non-zero.
//
#define VDS_E_INVALID_PLEX_COUNT         ((HRESULT)0x80042521L)

//
// MessageId: VDS_E_INVALID_MEMBER_COUNT
//
// MessageText:
//
// The member count for the volume must be non-zero.
//
#define VDS_E_INVALID_MEMBER_COUNT       ((HRESULT)0x80042522L)

//
// MessageId: VDS_E_INVALID_PLEX_ORDER
//
// MessageText:
//
// The plex indexes must be monotonically increasing and begin with zero.
//
#define VDS_E_INVALID_PLEX_ORDER         ((HRESULT)0x80042523L)

//
// MessageId: VDS_E_INVALID_MEMBER_ORDER
//
// MessageText:
//
// The member indexes must be monotonically increasing and begin with zero.
//
#define VDS_E_INVALID_MEMBER_ORDER       ((HRESULT)0x80042524L)

//
// MessageId: VDS_E_INVALID_STRIPE_SIZE
//
// MessageText:
//
// The stripe size must be a multiple of 2, between 512 bytes and 1 MB, 
// for stripe and raid5 volumes. The stripe size must be zero for other 
// volume types.
//
#define VDS_E_INVALID_STRIPE_SIZE        ((HRESULT)0x80042525L)

//
// MessageId: VDS_E_INVALID_DISK_COUNT
//
// MessageText:
//
// The number of disks specified is invalid for this operation.
//
#define VDS_E_INVALID_DISK_COUNT         ((HRESULT)0x80042526L)

//
// MessageId: VDS_E_INVALID_EXTENT_COUNT
//
// MessageText:
//
// An invalid number of extents was specified for at least one disk.
//
#define VDS_E_INVALID_EXTENT_COUNT       ((HRESULT)0x80042527L)

//
// MessageId: VDS_E_SOURCE_IS_TARGET_PACK
//
// MessageText:
//
// The source and target packs must be distinct.
//
#define VDS_E_SOURCE_IS_TARGET_PACK      ((HRESULT)0x80042528L)

//
// MessageId: VDS_E_VOLUME_DISK_COUNT_MAX_EXCEEDED
//
// MessageText:
//
// The disk count for the volume exceeds the maximum.
//
#define VDS_E_VOLUME_DISK_COUNT_MAX_EXCEEDED ((HRESULT)0x80042529L)

//
// MessageId: VDS_E_CORRUPT_NOTIFICATION_INFO
//
// MessageText:
//
// The driver's notification info is corrupt.
//
#define VDS_E_CORRUPT_NOTIFICATION_INFO  ((HRESULT)0x8004252AL)

//
// MessageId: VDS_E_INVALID_PLEX_GUID
//
// MessageText:
//
// GUID_NULL is not a valid plex GUID.
//
#define VDS_E_INVALID_PLEX_GUID          ((HRESULT)0x8004252CL)

//
// MessageId: VDS_E_DISK_NOT_FOUND_IN_PACK
//
// MessageText:
//
// The specified disks do not belong to the same pack.
//
#define VDS_E_DISK_NOT_FOUND_IN_PACK     ((HRESULT)0x8004252DL)

//
// MessageId: VDS_E_DUPLICATE_DISK
//
// MessageText:
//
// The same disk was specified more than once.
//
#define VDS_E_DUPLICATE_DISK             ((HRESULT)0x8004252EL)

//
// MessageId: VDS_E_LAST_VALID_DISK
//
// MessageText:
//
// The operation cannot be completed because there is only one valid disk in the pack.
//
#define VDS_E_LAST_VALID_DISK            ((HRESULT)0x8004252FL)

//
// MessageId: VDS_E_INVALID_SECTOR_SIZE
//
// MessageText:
//
// All disks holding extents for a given volume must have the same 
// sector size, and the sector size must be valid.
//
#define VDS_E_INVALID_SECTOR_SIZE        ((HRESULT)0x80042530L)

//
// MessageId: VDS_E_ONE_EXTENT_PER_DISK
//
// MessageText:
//
// This call requires the parameters to specify one extent per disk. 
// A single disk cannot contribute to multiple members or plexes of the 
// same volume.
//
#define VDS_E_ONE_EXTENT_PER_DISK        ((HRESULT)0x80042531L)

//
// MessageId: VDS_E_INVALID_BLOCK_SIZE
//
// MessageText:
//
// Neither the volume stripe size or disk sector size was found to be non-zero.
//
#define VDS_E_INVALID_BLOCK_SIZE         ((HRESULT)0x80042532L)

//
// MessageId: VDS_E_PLEX_SIZE_INVALID
//
// MessageText:
//
// The size of the volume plex is invalid.
//
#define VDS_E_PLEX_SIZE_INVALID          ((HRESULT)0x80042533L)

//
// MessageId: VDS_E_NO_EXTENTS_FOR_PLEX
//
// MessageText:
//
// No extents were found for the plex.
//
#define VDS_E_NO_EXTENTS_FOR_PLEX        ((HRESULT)0x80042534L)

//
// MessageId: VDS_E_INVALID_PLEX_TYPE
//
// MessageText:
//
// The plex type is invalid.
//
#define VDS_E_INVALID_PLEX_TYPE          ((HRESULT)0x80042535L)

//
// MessageId: VDS_E_INVALID_PLEX_BLOCK_SIZE
//
// MessageText:
//
// The plex block size must be non-zero.
//
#define VDS_E_INVALID_PLEX_BLOCK_SIZE    ((HRESULT)0x80042536L)

//
// MessageId: VDS_E_NO_HEALTHY_DISKS
//
// MessageText:
//
// All of the disks involved in the operation are either missing or failed.
//
#define VDS_E_NO_HEALTHY_DISKS           ((HRESULT)0x80042537L)

//
// MessageId: VDS_E_CONFIG_LIMIT
//
// MessageText:
//
// The Logical Disk Manangement database is full, no more volumes or disks may be configured.
//
#define VDS_E_CONFIG_LIMIT               ((HRESULT)0x80042538L)

//
// MessageId: VDS_E_DISK_CONFIGURATION_CORRUPTED
//
// MessageText:
//
// The disk configuration data is corrupted.
//
#define VDS_E_DISK_CONFIGURATION_CORRUPTED ((HRESULT)0x80042539L)

//
// MessageId: VDS_E_DISK_CONFIGURATION_NOT_IN_SYNC
//
// MessageText:
//
// The disk configuration is not insync with the in-memory configuration.
//
#define VDS_E_DISK_CONFIGURATION_NOT_IN_SYNC ((HRESULT)0x8004253AL)

//
// MessageId: VDS_E_DISK_CONFIGURATION_UPDATE_FAILED
//
// MessageText:
//
// One or more disks failed to be updated with the new configuration.
//
#define VDS_E_DISK_CONFIGURATION_UPDATE_FAILED ((HRESULT)0x8004253BL)

//
// MessageId: VDS_E_DISK_DYNAMIC
//
// MessageText:
//
// The disk is already dynamic.
//
#define VDS_E_DISK_DYNAMIC               ((HRESULT)0x8004253CL)

//
// MessageId: VDS_E_DRIVER_OBJECT_NOT_FOUND
//
// MessageText:
//
// The object was not found in the driver cache.
//
#define VDS_E_DRIVER_OBJECT_NOT_FOUND    ((HRESULT)0x8004253DL)

//
// MessageId: VDS_E_PARTITION_NOT_CYLINDER_ALIGNED
//
// MessageText:
//
// The disk layout contains partitions which are not cylinder aligned.
//
#define VDS_E_PARTITION_NOT_CYLINDER_ALIGNED ((HRESULT)0x8004253EL)

//
// MessageId: VDS_E_DISK_LAYOUT_PARTITIONS_TOO_SMALL
//
// MessageText:
//
// The disk layout contains partitions which less than the minimum required size.
//
#define VDS_E_DISK_LAYOUT_PARTITIONS_TOO_SMALL ((HRESULT)0x8004253FL)

//
// MessageId: VDS_E_DISK_IO_FAILING
//
// MessageText:
//
// The IO to the disk is failing.
//
#define VDS_E_DISK_IO_FAILING            ((HRESULT)0x80042540L)

//
// MessageId: VDS_E_DYNAMIC_DISKS_NOT_SUPPORTED
//
// MessageText:
//
// Dynamic disks are not supported by this operating system or server configuration. Dynamic disks are not supported on clusters.
//
#define VDS_E_DYNAMIC_DISKS_NOT_SUPPORTED ((HRESULT)0x80042541L)

//
// MessageId: VDS_E_FAULT_TOLERANT_DISKS_NOT_SUPPORTED
//
// MessageText:
//
// The fault tolerant disks are not supported by this operating system.
//
#define VDS_E_FAULT_TOLERANT_DISKS_NOT_SUPPORTED ((HRESULT)0x80042542L)

//
// MessageId: VDS_E_GPT_ATTRIBUTES_INVALID
//
// MessageText:
//
// Invalid GPT attributes were specified.
//
#define VDS_E_GPT_ATTRIBUTES_INVALID     ((HRESULT)0x80042543L)

//
// MessageId: VDS_E_MEMBER_IS_HEALTHY
//
// MessageText:
//
// The member is not stale or detached.
//
#define VDS_E_MEMBER_IS_HEALTHY          ((HRESULT)0x80042544L)

//
// MessageId: VDS_E_MEMBER_REGENERATING
//
// MessageText:
//
// The member is regenerating.
//
#define VDS_E_MEMBER_REGENERATING        ((HRESULT)0x80042545L)

//
// MessageId: VDS_E_PACK_NAME_INVALID
//
// MessageText:
//
// The pack name is invalid.
//
#define VDS_E_PACK_NAME_INVALID          ((HRESULT)0x80042546L)

//
// MessageId: VDS_E_PLEX_IS_HEALTHY
//
// MessageText:
//
// The plex is not stale or detached.
//
#define VDS_E_PLEX_IS_HEALTHY            ((HRESULT)0x80042547L)

//
// MessageId: VDS_E_PLEX_LAST_ACTIVE
//
// MessageText:
//
// The last healthy plex cannot be removed.
//
#define VDS_E_PLEX_LAST_ACTIVE           ((HRESULT)0x80042548L)

//
// MessageId: VDS_E_PLEX_MISSING
//
// MessageText:
//
// The plex is missing.
//
#define VDS_E_PLEX_MISSING               ((HRESULT)0x80042549L)

//
// MessageId: VDS_E_MEMBER_MISSING
//
// MessageText:
//
// The member is missing.
//
#define VDS_E_MEMBER_MISSING             ((HRESULT)0x8004254AL)

//
// MessageId: VDS_E_PLEX_REGENERATING
//
// MessageText:
//
// The plex is regenerating.
//
#define VDS_E_PLEX_REGENERATING          ((HRESULT)0x8004254BL)

//
// MessageId: VDS_E_UNEXPECTED_DISK_LAYOUT_CHANGE
//
// MessageText:
//
// An unexpected layout change occurred external to the volume manager.
//
#define VDS_E_UNEXPECTED_DISK_LAYOUT_CHANGE ((HRESULT)0x8004254DL)

//
// MessageId: VDS_E_INVALID_VOLUME_LENGTH
//
// MessageText:
//
// The volume length is invalid.
//
#define VDS_E_INVALID_VOLUME_LENGTH      ((HRESULT)0x8004254EL)

//
// MessageId: VDS_E_VOLUME_LENGTH_NOT_SECTOR_SIZE_MULTIPLE
//
// MessageText:
//
// The volume length is not a multiple of the sector size.
//
#define VDS_E_VOLUME_LENGTH_NOT_SECTOR_SIZE_MULTIPLE ((HRESULT)0x8004254FL)

//
// MessageId: VDS_E_VOLUME_NOT_RETAINED
//
// MessageText:
//
// The volume does not have a retained partition association.
//
#define VDS_E_VOLUME_NOT_RETAINED        ((HRESULT)0x80042550L)

//
// MessageId: VDS_E_VOLUME_RETAINED
//
// MessageText:
//
// The volume already has a retained partition association.
//
#define VDS_E_VOLUME_RETAINED            ((HRESULT)0x80042551L)

//
// MessageId: VDS_E_ALIGN_BEYOND_FIRST_CYLINDER
//
// MessageText:
//
// The specified alignment is beyond the first cylinder.
//
#define VDS_E_ALIGN_BEYOND_FIRST_CYLINDER ((HRESULT)0x80042553L)

//
// MessageId: VDS_E_ALIGN_NOT_SECTOR_SIZE_MULTIPLE
//
// MessageText:
//
// The specified alignment is not a multiple of the sector size.
//
#define VDS_E_ALIGN_NOT_SECTOR_SIZE_MULTIPLE ((HRESULT)0x80042554L)

//
// MessageId: VDS_E_ALIGN_NOT_ZERO
//
// MessageText:
//
// The specified partition type cannot be created with a non-zero alignment.
//
#define VDS_E_ALIGN_NOT_ZERO             ((HRESULT)0x80042555L)

//
// MessageId: VDS_E_CACHE_CORRUPT
//
// MessageText:
//
// The service's cache has become corrupt.
//
#define VDS_E_CACHE_CORRUPT              ((HRESULT)0x80042556L)

//
// MessageId: VDS_E_CANNOT_CLEAR_VOLUME_FLAG
//
// MessageText:
//
// The specified volume flag cannot be cleared.
//
#define VDS_E_CANNOT_CLEAR_VOLUME_FLAG   ((HRESULT)0x80042557L)

//
// MessageId: VDS_E_DISK_BEING_CLEANED
//
// MessageText:
//
// The operation is not allowed on a disk that is in the process of being cleaned.
//
#define VDS_E_DISK_BEING_CLEANED         ((HRESULT)0x80042558L)

//
// MessageId: VDS_E_DISK_NOT_CONVERTIBLE
//
// MessageText:
//
// The specified disk is not convertible. CDROMs and DVDs 
// are examples of disks that are not convertable.
//
#define VDS_E_DISK_NOT_CONVERTIBLE       ((HRESULT)0x80042559L)

//
// MessageId: VDS_E_DISK_REMOVEABLE
//
// MessageText:
//
// The operation is not supported on removable media.
//
#define VDS_E_DISK_REMOVEABLE            ((HRESULT)0x8004255AL)

//
// MessageId: VDS_E_DISK_REMOVEABLE_NOT_EMPTY
//
// MessageText:
//
// The operation is not supported on a non-empty removable disk.
//
#define VDS_E_DISK_REMOVEABLE_NOT_EMPTY  ((HRESULT)0x8004255BL)

//
// MessageId: VDS_E_DRIVE_LETTER_NOT_FREE
//
// MessageText:
//
// The specified drive letter is not free to be assigned.
//
#define VDS_E_DRIVE_LETTER_NOT_FREE      ((HRESULT)0x8004255CL)

//
// MessageId: VDS_E_EXTEND_MULTIPLE_DISKS_NOT_SUPPORTED
//
// MessageText:
//
// Extending the volume onto multiple disks is not supported by this provider.
//
#define VDS_E_EXTEND_MULTIPLE_DISKS_NOT_SUPPORTED ((HRESULT)0x8004255DL)

//
// MessageId: VDS_E_INVALID_DRIVE_LETTER
//
// MessageText:
//
// The specified drive letter is invalid.
//
#define VDS_E_INVALID_DRIVE_LETTER       ((HRESULT)0x8004255EL)

//
// MessageId: VDS_E_INVALID_DRIVE_LETTER_COUNT
//
// MessageText:
//
// The specified number of drive letters to retrieve is invalid.
//
#define VDS_E_INVALID_DRIVE_LETTER_COUNT ((HRESULT)0x8004255FL)

//
// MessageId: VDS_E_INVALID_FS_FLAG
//
// MessageText:
//
// The specified file system flag is invalid.
//
#define VDS_E_INVALID_FS_FLAG            ((HRESULT)0x80042560L)

//
// MessageId: VDS_E_INVALID_FS_TYPE
//
// MessageText:
//
// The specified file system is invalid.
//
#define VDS_E_INVALID_FS_TYPE            ((HRESULT)0x80042561L)

//
// MessageId: VDS_E_INVALID_OBJECT_TYPE
//
// MessageText:
//
// The specified object type is invalid.
//
#define VDS_E_INVALID_OBJECT_TYPE        ((HRESULT)0x80042562L)

//
// MessageId: VDS_E_INVALID_PARTITION_LAYOUT
//
// MessageText:
//
// The specified partition layout is invalid.
//
#define VDS_E_INVALID_PARTITION_LAYOUT   ((HRESULT)0x80042563L)

//
// MessageId: VDS_E_INVALID_PARTITION_STYLE
//
// MessageText:
//
// The specified disk's partition style is INVALID.  
// VDS only supports MBR or GPT partition style disks.
//
#define VDS_E_INVALID_PARTITION_STYLE    ((HRESULT)0x80042564L)

//
// MessageId: VDS_E_INVALID_PARTITION_TYPE
//
// MessageText:
//
// The specified partition type is not valid for this operation.
//
#define VDS_E_INVALID_PARTITION_TYPE     ((HRESULT)0x80042565L)

//
// MessageId: VDS_E_INVALID_PROVIDER_CLSID
//
// MessageText:
//
// The specified provider clsid cannot be a NULL GUID.
//
#define VDS_E_INVALID_PROVIDER_CLSID     ((HRESULT)0x80042566L)

//
// MessageId: VDS_E_INVALID_PROVIDER_ID
//
// MessageText:
//
// The specified provider id cannot be a NULL GUID.
//
#define VDS_E_INVALID_PROVIDER_ID        ((HRESULT)0x80042567L)

//
// MessageId: VDS_E_INVALID_PROVIDER_NAME
//
// MessageText:
//
// The specified provider name is invalid.
//
#define VDS_E_INVALID_PROVIDER_NAME      ((HRESULT)0x80042568L)

//
// MessageId: VDS_E_INVALID_PROVIDER_TYPE
//
// MessageText:
//
// The specified provider type is invalid.
//
#define VDS_E_INVALID_PROVIDER_TYPE      ((HRESULT)0x80042569L)

//
// MessageId: VDS_E_INVALID_PROVIDER_VERSION_GUID
//
// MessageText:
//
// The specified provider version GUID cannot be a NULL GUID.
//
#define VDS_E_INVALID_PROVIDER_VERSION_GUID ((HRESULT)0x8004256AL)

//
// MessageId: VDS_E_INVALID_PROVIDER_VERSION_STRING
//
// MessageText:
//
// The specified provider version string is invalid.
//
#define VDS_E_INVALID_PROVIDER_VERSION_STRING ((HRESULT)0x8004256BL)

//
// MessageId: VDS_E_INVALID_QUERY_PROVIDER_FLAG
//
// MessageText:
//
// The specified query provider flag is invalid.
//
#define VDS_E_INVALID_QUERY_PROVIDER_FLAG ((HRESULT)0x8004256CL)

//
// MessageId: VDS_E_INVALID_SERVICE_FLAG
//
// MessageText:
//
// The specified service flag is invalid.
//
#define VDS_E_INVALID_SERVICE_FLAG       ((HRESULT)0x8004256DL)

//
// MessageId: VDS_E_INVALID_VOLUME_FLAG
//
// MessageText:
//
// The specified volume flag is invalid.
//
#define VDS_E_INVALID_VOLUME_FLAG        ((HRESULT)0x8004256EL)

//
// MessageId: VDS_E_PARTITION_NOT_OEM
//
// MessageText:
//
// The operation is not supported on non-OEM partitions.
//
#define VDS_E_PARTITION_NOT_OEM          ((HRESULT)0x8004256FL)

//
// MessageId: VDS_E_PARTITION_PROTECTED
//
// MessageText:
//
// Cannot delete a protected partition without the force protected parameter set.
//
#define VDS_E_PARTITION_PROTECTED        ((HRESULT)0x80042570L)

//
// MessageId: VDS_E_PARTITION_STYLE_MISMATCH
//
// MessageText:
//
// The specified partition style does not match that of the disk.
//
#define VDS_E_PARTITION_STYLE_MISMATCH   ((HRESULT)0x80042571L)

//
// MessageId: VDS_E_PROVIDER_INTERNAL_ERROR
//
// MessageText:
//
// An internal error has occurred in the provider.
//
#define VDS_E_PROVIDER_INTERNAL_ERROR    ((HRESULT)0x80042572L)

//
// MessageId: VDS_E_SHRINK_SIZE_LESS_THAN_MIN
//
// MessageText:
//
// The specified shrink size is less than then minimum shrink size allowed.
//
#define VDS_E_SHRINK_SIZE_LESS_THAN_MIN  ((HRESULT)0x80042573L)

//
// MessageId: VDS_E_SHRINK_SIZE_TOO_BIG
//
// MessageText:
//
// The specified shrink size is too big and will cause the volume to be 
// smaller than the minimum volume size.
//
#define VDS_E_SHRINK_SIZE_TOO_BIG        ((HRESULT)0x80042574L)

//
// MessageId: VDS_E_UNRECOVERABLE_PROVIDER_ERROR
//
// MessageText:
//
// An unrecoverable error occurred in a provider.  
// The service must be shut down to regain full functionality.
//
#define VDS_E_UNRECOVERABLE_PROVIDER_ERROR ((HRESULT)0x80042575L)

//
// MessageId: VDS_E_VOLUME_HIDDEN
//
// MessageText:
//
// Cannot assign a mount point to a hidden volume.
//
#define VDS_E_VOLUME_HIDDEN              ((HRESULT)0x80042576L)

//
// MessageId: VDS_S_DISMOUNT_FAILED
//
// MessageText:
//
// Failed to dismount the volume after setting the volume flags.
//
#define VDS_S_DISMOUNT_FAILED            ((HRESULT)0x00042577L)

//
// MessageId: VDS_S_REMOUNT_FAILED
//
// MessageText:
//
// Failed to remount the volume after setting the volume flags.
//
#define VDS_S_REMOUNT_FAILED             ((HRESULT)0x00042578L)

//
// MessageId: VDS_E_FLAG_ALREADY_SET
//
// MessageText:
//
// Can't set the specified flag as revert-on-close, because it is already set.
//
#define VDS_E_FLAG_ALREADY_SET           ((HRESULT)0x80042579L)

//
// MessageId: VDS_S_RESYNC_NOTIFICATION_TASK_FAILED
//
// MessageText:
//
// Failure. If the volume is a mirror volume or a raid5 volume, no 
// resynchronization notifications will be sent.
//
#define VDS_S_RESYNC_NOTIFICATION_TASK_FAILED ((HRESULT)0x0004257AL)

//
// MessageId: VDS_E_DISTINCT_VOLUME
//
// MessageText:
//
// The input volume id cannot be the id of the volume that is the target of 
// the operation.
//
#define VDS_E_DISTINCT_VOLUME            ((HRESULT)0x8004257BL)

//
// MessageId: VDS_E_VOLUME_NOT_FOUND_IN_PACK
//
// MessageText:
//
// The specified volumes do not belong to the same pack.
//
#define VDS_E_VOLUME_NOT_FOUND_IN_PACK   ((HRESULT)0x8004257CL)

//
// MessageId: VDS_E_PARTITION_NON_DATA
//
// MessageText:
//
// The specified partition is a not a primary or logical volume.
//
#define VDS_E_PARTITION_NON_DATA         ((HRESULT)0x8004257DL)

//
// MessageId: VDS_E_CRITICAL_PLEX
//
// MessageText:
//
// The specified plex is a the current system or boot plex.
//
#define VDS_E_CRITICAL_PLEX              ((HRESULT)0x8004257EL)

//
// MessageId: VDS_E_VOLUME_SYNCHRONIZING
//
// MessageText:
//
// The operation cannot be completed because the volume is synchronizing.
//
#define VDS_E_VOLUME_SYNCHRONIZING       ((HRESULT)0x8004257FL)

//
// MessageId: VDS_E_VOLUME_REGENERATING
//
// MessageText:
//
// The operation cannot be completed because the volume is regenerating.
//
#define VDS_E_VOLUME_REGENERATING        ((HRESULT)0x80042580L)

//
// MessageId: VDS_S_VSS_FLUSH_AND_HOLD_WRITES
//
// MessageText:
//
// Failed to flush and hold Volume Snapshot Service writes.
//
#define VDS_S_VSS_FLUSH_AND_HOLD_WRITES  ((HRESULT)0x00042581L)

//
// MessageId: VDS_S_VSS_RELEASE_WRITES
//
// MessageText:
//
// Failed to release Volume Snapshot Service writes.
//
#define VDS_S_VSS_RELEASE_WRITES         ((HRESULT)0x00042582L)

//
// MessageId: VDS_S_FS_LOCK
//
// MessageText:
//
// Failed to obtain a file system lock.
//
#define VDS_S_FS_LOCK                    ((HRESULT)0x00042583L)

//
// MessageId: VDS_E_READONLY
//
// MessageText:
//
// The volume is read only.
//
#define VDS_E_READONLY                   ((HRESULT)0x80042584L)

//
// MessageId: VDS_E_INVALID_VOLUME_TYPE
//
// MessageText:
//
// The volume type is invalid for this operation.
//
#define VDS_E_INVALID_VOLUME_TYPE        ((HRESULT)0x80042585L)

//
// MessageId: VDS_E_BAD_BOOT_DISK
//
// MessageText:
//
// The boot disk experienced failures when the driver attempted to online 
// the pack.
//
#define VDS_E_BAD_BOOT_DISK              ((HRESULT)0x80042586L)

//
// MessageId: VDS_E_LOG_UPDATE
//
// MessageText:
//
// The driver failed to update the log on at least one disk.
//
#define VDS_E_LOG_UPDATE                 ((HRESULT)0x80042587L)

//
// MessageId: VDS_E_VOLUME_MIRRORED
//
// MessageText:
//
// This operation is not supported on a mirrored volume.
//
#define VDS_E_VOLUME_MIRRORED            ((HRESULT)0x80042588L)

//
// MessageId: VDS_E_VOLUME_SIMPLE_SPANNED
//
// MessageText:
//
// This operation is only supported on simple or spanned volumes.
//
#define VDS_E_VOLUME_SIMPLE_SPANNED      ((HRESULT)0x80042589L)

//
// MessageId: VDS_E_NO_VALID_LOG_COPIES
//
// MessageText:
//
// This pack has no valid log copies.
//
#define VDS_E_NO_VALID_LOG_COPIES        ((HRESULT)0x8004258AL)

//
// MessageId: VDS_S_PLEX_NOT_LOADED_TO_CACHE
//
// MessageText:
//
// This plex is present in the driver, but has not yet been loaded to the 
// provider cache. A volume modified notification will be sent by the service 
// once the plex has been loaded to the provider cache.
//
#define VDS_S_PLEX_NOT_LOADED_TO_CACHE   ((HRESULT)0x0004258BL)

//
// MessageId: VDS_E_PLEX_NOT_LOADED_TO_CACHE
//
// MessageText:
//
// This plex is present in the driver, but has not yet been loaded to the 
// provider cache. A volume modified notification will be sent by the service 
// once the plex has been loaded to the provider cache.
//
#define VDS_E_PLEX_NOT_LOADED_TO_CACHE   ((HRESULT)0x8004258BL)

//
// MessageId: VDS_E_PARTITION_MSR
//
// MessageText:
//
// This operation is not supported on MSR partitions.
//
#define VDS_E_PARTITION_MSR              ((HRESULT)0x8004258CL)

//
// MessageId: VDS_E_PARTITION_LDM
//
// MessageText:
//
// This operation is not supported on LDM partitions.
//
#define VDS_E_PARTITION_LDM              ((HRESULT)0x8004258DL)

//
// MessageId: VDS_S_WINPE_BOOTENTRY
//
// MessageText:
//
// The boot entries cannot be updated automatically on WinPE. You may need 
// to manually update the boot entry for any installed operating systems.
//
#define VDS_S_WINPE_BOOTENTRY            ((HRESULT)0x0004258EL)

//
// MessageId: VDS_E_ALIGN_NOT_A_POWER_OF_TWO
//
// MessageText:
//
// The specified alignment is not a power of two.
//
#define VDS_E_ALIGN_NOT_A_POWER_OF_TWO   ((HRESULT)0x8004258FL)

//
// MessageId: VDS_E_ALIGN_IS_ZERO
//
// MessageText:
//
// The specified alignment is zero.
//
#define VDS_E_ALIGN_IS_ZERO              ((HRESULT)0x80042590L)

//
// MessageId: VDS_E_SHRINK_IN_PROGRESS
//
// MessageText:
//
// A defragmentation or volume shrink operation is already in progress. 
// Only one of these operations can run at a time.
//
#define VDS_E_SHRINK_IN_PROGRESS         ((HRESULT)0x80042591L)

//
// MessageId: VDS_E_CANT_INVALIDATE_FVE
//
// MessageText:
//
// BitLocker encryption on the volume could not be removed.
//
#define VDS_E_CANT_INVALIDATE_FVE        ((HRESULT)0x80042592L)

//
// MessageId: VDS_E_FS_NOT_DETERMINED
//
// MessageText:
//
// The default file system could not be determined.
//
#define VDS_E_FS_NOT_DETERMINED          ((HRESULT)0x80042593L)

//
// MessageId: VDS_E_DISK_NOT_OFFLINE
//
// MessageText:
//
// This disk is already online. 
//
#define VDS_E_DISK_NOT_OFFLINE           ((HRESULT)0x80042595L)

//
// MessageId: VDS_E_FAILED_TO_ONLINE_DISK
//
// MessageText:
//
// The online operation failed.
//
#define VDS_E_FAILED_TO_ONLINE_DISK      ((HRESULT)0x80042596L)

//
// MessageId: VDS_E_FAILED_TO_OFFLINE_DISK
//
// MessageText:
//
// The offline operation failed.
//
#define VDS_E_FAILED_TO_OFFLINE_DISK     ((HRESULT)0x80042597L)

//
// MessageId: VDS_E_BAD_REVISION_NUMBER
//
// MessageText:
//
// The operation could not be completed because the specified revision number 
// is not supported.
//
#define VDS_E_BAD_REVISION_NUMBER        ((HRESULT)0x80042598L)

//
// MessageId: VDS_E_SHRINK_USER_CANCELLED
//
// MessageText:
//
// The shrink operation was cancelled by the user.
//
#define VDS_E_SHRINK_USER_CANCELLED      ((HRESULT)0x80042599L)

//
// MessageId: VDS_E_SHRINK_DIRTY_VOLUME
//
// MessageText:
//
// The volume you have selected to shrink may be corrupted. 
// Use Chkdsk to fix the corruption problem, and then try to shrink the 
// volume again.
//
#define VDS_E_SHRINK_DIRTY_VOLUME        ((HRESULT)0x8004259AL)

//////////////////////////////////////////////////////////////////////////////
//
// Codes added in VDS 1.1:                       0x2700-0x27FF
//
//////////////////////////////////////////////////////////////////////////////
//
// MessageId: VDS_S_NAME_TRUNCATED
//
// MessageText:
//
// The name was accepted but had to be truncated.
//
#define VDS_S_NAME_TRUNCATED             ((HRESULT)0x00042700L)

//
// MessageId: VDS_E_NAME_NOT_UNIQUE
//
// MessageText:
//
// The name is not unique.
//
#define VDS_E_NAME_NOT_UNIQUE            ((HRESULT)0x80042701L)

//
// MessageId: VDS_S_STATUSES_INCOMPLETELY_SET
//
// MessageText:
//
// At least one path status is not set successfully due to a non-fatal error, 
// for example the status conflicts with the current load balance policy.
//
#define VDS_S_STATUSES_INCOMPLETELY_SET  ((HRESULT)0x00042702L)

//
// MessageId: VDS_E_ADDRESSES_INCOMPLETELY_SET
//
// MessageText:
//
// At least one portal's tunnel address is not set successfully.
//
#define VDS_E_ADDRESSES_INCOMPLETELY_SET ((HRESULT)0x80042703L)

//
// MessageId: VDS_E_SECURITY_INCOMPLETELY_SET
//
// MessageText:
//
// At least one portal's security settings are not set successfully.
//
#define VDS_E_SECURITY_INCOMPLETELY_SET  ((HRESULT)0x80042705L)

//
// MessageId: VDS_E_TARGET_SPECIFIC_NOT_SUPPORTED
//
// MessageText:
//
// The initiator does not support setting target-specific shared secrets.
//
#define VDS_E_TARGET_SPECIFIC_NOT_SUPPORTED ((HRESULT)0x80042706L)

//
// MessageId: VDS_E_INITIATOR_SPECIFIC_NOT_SUPPORTED
//
// MessageText:
//
// The target does not support setting initiator-specific shared secrets.
//
#define VDS_E_INITIATOR_SPECIFIC_NOT_SUPPORTED ((HRESULT)0x80042707L)

//
// MessageId: VDS_E_ISCSI_LOGIN_FAILED
//
// MessageText:
//
// An iSCSI login session could not be established.
//
#define VDS_E_ISCSI_LOGIN_FAILED         ((HRESULT)0x80042708L)

//
// MessageId: VDS_E_ISCSI_LOGOUT_FAILED
//
// MessageText:
//
// The attempt to log out from the specified iSCSI session failed.
//
#define VDS_E_ISCSI_LOGOUT_FAILED        ((HRESULT)0x80042709L)

//
// MessageId: VDS_E_ISCSI_SESSION_NOT_FOUND
//
// MessageText:
//
// The specified iSCSI session with a connection matching the specified target, 
// target portal, and/or initiator portal could not be found.
//
#define VDS_E_ISCSI_SESSION_NOT_FOUND    ((HRESULT)0x8004270AL)

//
// MessageId: VDS_E_ASSOCIATED_LUNS_EXIST
//
// MessageText:
//
// LUNs are associated with the specified target and must first be 
// deassociated before the target can be deleted.
//
#define VDS_E_ASSOCIATED_LUNS_EXIST      ((HRESULT)0x8004270BL)

//
// MessageId: VDS_E_ASSOCIATED_PORTALS_EXIST
//
// MessageText:
//
// Portals are associated with the specified portal group and must first be 
// deassociated before the portal group can be deleted.
//
#define VDS_E_ASSOCIATED_PORTALS_EXIST   ((HRESULT)0x8004270CL)

//
// MessageId: VDS_E_NO_DISCOVERY_DOMAIN
//
// MessageText:
//
// The initiator does not exist in a iSNS discovery domain.
//
#define VDS_E_NO_DISCOVERY_DOMAIN        ((HRESULT)0x8004270DL)

//
// MessageId: VDS_E_MULTIPLE_DISCOVERY_DOMAINS
//
// MessageText:
//
// The initiator exists in more than one iSNS discovery domain.
//
#define VDS_E_MULTIPLE_DISCOVERY_DOMAINS ((HRESULT)0x8004270EL)

//
// MessageId: VDS_E_NO_DISK_PATHNAME
//
// MessageText:
//
// Failed to retrieve the disk's pathname. Some operations on the disk may fail.
//
#define VDS_E_NO_DISK_PATHNAME           ((HRESULT)0x8004270FL)

//
// MessageId: VDS_E_ISCSI_LOGOUT_INCOMPLETE
//
// MessageText:
//
// At least one session did not logout successfully.
//
#define VDS_E_ISCSI_LOGOUT_INCOMPLETE    ((HRESULT)0x80042710L)

//
// MessageId: VDS_E_NO_VOLUME_PATHNAME
//
// MessageText:
//
// Failed to retrieve the volume's pathname.
//
#define VDS_E_NO_VOLUME_PATHNAME         ((HRESULT)0x80042711L)

//
// MessageId: VDS_E_PROVIDER_CACHE_OUTOFSYNC
//
// MessageText:
//
// The provider's cache is not in-sync with the driver cache.
//
#define VDS_E_PROVIDER_CACHE_OUTOFSYNC   ((HRESULT)0x80042712L)

//
// MessageId: VDS_E_NO_IMPORT_TARGET
//
// MessageText:
//
// No import target was set for this subsystem in the registry.
//
#define VDS_E_NO_IMPORT_TARGET           ((HRESULT)0x80042713L)

//
// MessageId: VDS_S_ALREADY_EXISTS
//
// MessageText:
//
// The object already exists and does not need to be further created or added.
//
#define VDS_S_ALREADY_EXISTS             ((HRESULT)0x00042714L)

//
// MessageId: VDS_S_PROPERTIES_INCOMPLETE
//
// MessageText:
//
// Unable to retrieve all properties for this object. 
// Some attributes may be incomplete or missing.
//
#define VDS_S_PROPERTIES_INCOMPLETE      ((HRESULT)0x00042715L)

//////////////////////////////////////////////////////////////////////////////
//
// Codes added in Windows Server 2008:     0x2800-0x28FF
//
//////////////////////////////////////////////////////////////////////////////
//
// MessageId: VDS_S_ISCSI_SESSION_NOT_FOUND_PERSISTENT_LOGIN_REMOVED
//
// MessageText:
//
// No iSCSI sessions to the target were found, but the persistent login 
// setup to the target was removed.
//
#define VDS_S_ISCSI_SESSION_NOT_FOUND_PERSISTENT_LOGIN_REMOVED ((HRESULT)0x00042800L)

//
// MessageId: VDS_S_ISCSI_PERSISTENT_LOGIN_MAY_NOT_BE_REMOVED
//
// MessageText:
//
// If a persistent login was set up for the target, it may not have been 
// removed. Check the iSCSI Initiator Control Panel to remove it if necessary.
//
#define VDS_S_ISCSI_PERSISTENT_LOGIN_MAY_NOT_BE_REMOVED ((HRESULT)0x00042801L)

//
// MessageId: VDS_S_ISCSI_LOGIN_ALREAD_EXISTS
//
// MessageText:
//
// Login failed because the target is already logged in.
//
#define VDS_S_ISCSI_LOGIN_ALREAD_EXISTS  ((HRESULT)0x00042802L)

//
// MessageId: VDS_E_UNABLE_TO_FIND_BOOT_DISK
//
// MessageText:
//
// A system error occurred while retrieving the boot disk information.
//
#define VDS_E_UNABLE_TO_FIND_BOOT_DISK   ((HRESULT)0x80042803L)

//
// MessageId: VDS_E_INCORRECT_BOOT_VOLUME_EXTENT_INFO
//
// MessageText:
//
// Multiple disk extents reported for the boot volume - system error.
//
#define VDS_E_INCORRECT_BOOT_VOLUME_EXTENT_INFO ((HRESULT)0x80042804L)

//
// MessageId: VDS_E_GET_SAN_POLICY
//
// MessageText:
//
// A driver error was reported when getting the SAN policy.
//
#define VDS_E_GET_SAN_POLICY             ((HRESULT)0x80042805L)

//
// MessageId: VDS_E_SET_SAN_POLICY
//
// MessageText:
//
// A driver error was reported when setting the SAN policy.
//
#define VDS_E_SET_SAN_POLICY             ((HRESULT)0x80042806L)

//
// MessageId: VDS_E_BOOT_DISK
//
// MessageText:
//
// Disk attributes may not be changed on the boot disk.
//
#define VDS_E_BOOT_DISK                  ((HRESULT)0x80042807L)

//
// MessageId: VDS_S_DISK_MOUNT_FAILED
//
// MessageText:
//
// Failed to mount one or more of the volumes on the disk.
//
#define VDS_S_DISK_MOUNT_FAILED          ((HRESULT)0x00042808L)

//
// MessageId: VDS_S_DISK_DISMOUNT_FAILED
//
// MessageText:
//
// Failed to dismount one or more of the volumes on the disk.
//
#define VDS_S_DISK_DISMOUNT_FAILED       ((HRESULT)0x00042809L)

//
// MessageId: VDS_E_DISK_IS_OFFLINE
//
// MessageText:
//
// The operation is not allowed on a disk that is offline.
//
#define VDS_E_DISK_IS_OFFLINE            ((HRESULT)0x8004280AL)

//
// MessageId: VDS_E_DISK_IS_READ_ONLY
//
// MessageText:
//
// The operation is not allowed on a disk that is read only.
//
#define VDS_E_DISK_IS_READ_ONLY          ((HRESULT)0x8004280BL)

//
// MessageId: VDS_E_PAGEFILE_DISK
//
// MessageText:
//
// The operation is not allowed on a disk that contains a pagefile volume.
//
#define VDS_E_PAGEFILE_DISK              ((HRESULT)0x8004280CL)

//
// MessageId: VDS_E_HIBERNATION_FILE_DISK
//
// MessageText:
//
// The operation is not allowed on a disk that contains a hibernation file volume.
//
#define VDS_E_HIBERNATION_FILE_DISK      ((HRESULT)0x8004280DL)

//
// MessageId: VDS_E_CRASHDUMP_DISK
//
// MessageText:
//
// The operation is not allowed on a disk that contains a crashdump file volume.
//
#define VDS_E_CRASHDUMP_DISK             ((HRESULT)0x8004280EL)

//
// MessageId: VDS_E_UNABLE_TO_FIND_SYSTEM_DISK
//
// MessageText:
//
// A system error occurred while retrieving the system disk information.
//
#define VDS_E_UNABLE_TO_FIND_SYSTEM_DISK ((HRESULT)0x8004280FL)

//
// MessageId: VDS_E_INCORRECT_SYSTEM_VOLUME_EXTENT_INFO
//
// MessageText:
//
// Multiple disk extents reported for the system volume - system error.
//
#define VDS_E_INCORRECT_SYSTEM_VOLUME_EXTENT_INFO ((HRESULT)0x80042810L)

//
// MessageId: VDS_E_SYSTEM_DISK
//
// MessageText:
//
// Disk attributes may not be changed on the current system disk or BIOS disk 0.
//
#define VDS_E_SYSTEM_DISK                ((HRESULT)0x80042811L)

//
// MessageId: VDS_E_VOLUME_SHRINK_FVE_LOCKED
//
// MessageText:
//
// The volume could not be shrunk because it is locked by BitLocker. 
// Unlock the volume and try again.
//
#define VDS_E_VOLUME_SHRINK_FVE_LOCKED   ((HRESULT)0x80042812L)

//
// MessageId: VDS_E_VOLUME_SHRINK_FVE_CORRUPT
//
// MessageText:
//
// The volume could not be shrunk because it is locked due to a BitLocker error. 
// Use BitLocker tools to recover the volume and try again.
//
#define VDS_E_VOLUME_SHRINK_FVE_CORRUPT  ((HRESULT)0x80042813L)

//
// MessageId: VDS_E_VOLUME_SHRINK_FVE_RECOVERY
//
// MessageText:
//
// The volume could not be shrunk because it is marked for BitLocker recovery. 
// Use BitLocker tools to recover the volume and try again.
//
#define VDS_E_VOLUME_SHRINK_FVE_RECOVERY ((HRESULT)0x80042814L)

//
// MessageId: VDS_E_VOLUME_SHRINK_FVE
//
// MessageText:
//
// The volume could not be shrunk because it is encrypted by BitLocker and 
// Fveapi.dll could not be loaded to determine its status. For this operation to 
// succeed, Fveapi.dll must be available in %SystemRoot%\System32\.
//
#define VDS_E_VOLUME_SHRINK_FVE          ((HRESULT)0x80042815L)

//
// MessageId: VDS_E_SHRINK_OVER_DATA
//
// MessageText:
//
// The SHRINK operation against the selected LUN cannot be completed. 
// Completing the operation using the specified parameters will 
// overwrite volumes containing user data.
//
#define VDS_E_SHRINK_OVER_DATA           ((HRESULT)0x80042816L)

//
// MessageId: VDS_E_INVALID_SHRINK_SIZE
//
// MessageText:
//
// The SHRINK operation against the selected LUN cannot be completed. 
// The specified size is greater than the size of the LUN.
//
#define VDS_E_INVALID_SHRINK_SIZE        ((HRESULT)0x80042817L)

//
// MessageId: VDS_E_LUN_DISK_MISSING
//
// MessageText:
//
// The SHRINK operation against the selected LUN cannot be completed. 
// The current state of the disk associated with the LUN is MISSING.
//
#define VDS_E_LUN_DISK_MISSING           ((HRESULT)0x80042818L)

//
// MessageId: VDS_E_LUN_DISK_FAILED
//
// MessageText:
//
// The SHRINK operation against the selected LUN cannot be completed. 
// The current state of the disk associated with the LUN is FAILED.
//
#define VDS_E_LUN_DISK_FAILED            ((HRESULT)0x80042819L)

//
// MessageId: VDS_E_LUN_DISK_NOT_READY
//
// MessageText:
//
// The SHRINK operation against the selected LUN cannot be completed. 
// The current state of the disk associated with the LUN is NOT READY.
//
#define VDS_E_LUN_DISK_NOT_READY         ((HRESULT)0x8004281AL)

//
// MessageId: VDS_E_LUN_DISK_NO_MEDIA
//
// MessageText:
//
// The SHRINK operation against the selected LUN cannot be completed. 
// The current state of the disk associated with the LUN is NO MEDIA.
//
#define VDS_E_LUN_DISK_NO_MEDIA          ((HRESULT)0x8004281BL)

//
// MessageId: VDS_E_LUN_NOT_READY
//
// MessageText:
//
// The SHRINK operation against the selected LUN cannot be completed. 
// The current state of the LUN is NOT READY.
//
#define VDS_E_LUN_NOT_READY              ((HRESULT)0x8004281CL)

//
// MessageId: VDS_E_LUN_OFFLINE
//
// MessageText:
//
// The SHRINK operation against the selected LUN cannot be completed. 
// The current state of the LUN is OFFLINE.
//
#define VDS_E_LUN_OFFLINE                ((HRESULT)0x8004281DL)

//
// MessageId: VDS_E_LUN_FAILED
//
// MessageText:
//
// The SHRINK operation against the selected LUN cannot be completed. 
// The current state of the LUN is FAILED.
//
#define VDS_E_LUN_FAILED                 ((HRESULT)0x8004281EL)

//
// MessageId: VDS_E_VOLUME_EXTEND_FVE_LOCKED
//
// MessageText:
//
// The volume could not be extended because it is locked by BitLocker. 
// Unlock the volume and retry the operation.
//
#define VDS_E_VOLUME_EXTEND_FVE_LOCKED   ((HRESULT)0x8004281FL)

//
// MessageId: VDS_E_VOLUME_EXTEND_FVE_CORRUPT
//
// MessageText:
//
// The volume could not be extended because it is locked due to a BitLocker 
// error. Use BitLocker tools to recover the volume and retry the operation.
//
#define VDS_E_VOLUME_EXTEND_FVE_CORRUPT  ((HRESULT)0x80042820L)

//
// MessageId: VDS_E_VOLUME_EXTEND_FVE_RECOVERY
//
// MessageText:
//
// The volume could not be extended because it is marked for BitLocker 
// recovery. Use BitLocker tools to recover the volume and retry the operation.
//
#define VDS_E_VOLUME_EXTEND_FVE_RECOVERY ((HRESULT)0x80042821L)

//
// MessageId: VDS_E_VOLUME_EXTEND_FVE
//
// MessageText:
//
// The volume could not be extended because it is encrypted by BitLocker and 
// Fveapi.dll could not be loaded to determine its status. For this operation to 
// succeed, Fveapi.dll must be available in %SystemRoot%\System32\.
//
#define VDS_E_VOLUME_EXTEND_FVE          ((HRESULT)0x80042822L)

//
// MessageId: VDS_E_SECTOR_SIZE_ERROR
//
// MessageText:
//
// The sector size must be non-zero, a power of 2, and less than the 
// maximum sector size.
//
#define VDS_E_SECTOR_SIZE_ERROR          ((HRESULT)0x80042823L)

//////////////////////////////////////////////////////////////////////////////
//
// Codes added in Win7:     0x2900-0x29FF
//
//////////////////////////////////////////////////////////////////////////////
//
// MessageId: VDS_E_INITIATOR_ADAPTER_NOT_FOUND
//
// MessageText:
//
// The initiator adapter was not found. For calls to GetPathInfo(), the 
// initiator adapater is associated with the path end point.
//
#define VDS_E_INITIATOR_ADAPTER_NOT_FOUND ((HRESULT)0x80042900L)

//
// MessageId: VDS_E_TARGET_PORTAL_NOT_FOUND
//
// MessageText:
//
// The target portal was not found. For calls to GetPathInfo(), the target portal 
// is associated with the path end point.
//
#define VDS_E_TARGET_PORTAL_NOT_FOUND    ((HRESULT)0x80042901L)

//
// MessageId: VDS_E_INVALID_PORT_PATH
//
// MessageText:
//
// The path returned for the port is invalid. Either it has an incorrect port type 
// specified, or, the HBA port properties structure is NULL.
//
#define VDS_E_INVALID_PORT_PATH          ((HRESULT)0x80042902L)

//
// MessageId: VDS_E_INVALID_ISCSI_TARGET_NAME
//
// MessageText:
//
// An invalid iSCSI target name was returned from the provider.
//
#define VDS_E_INVALID_ISCSI_TARGET_NAME  ((HRESULT)0x80042903L)

//
// MessageId: VDS_E_SET_TUNNEL_MODE_OUTER_ADDRESS
//
// MessageText:
//
// Call to set the iSCSI tunnel mode outer address failed.
//
#define VDS_E_SET_TUNNEL_MODE_OUTER_ADDRESS ((HRESULT)0x80042904L)

//
// MessageId: VDS_E_ISCSI_GET_IKE_INFO
//
// MessageText:
//
// Call to get the iSCSI IKE info failed.
//
#define VDS_E_ISCSI_GET_IKE_INFO         ((HRESULT)0x80042905L)

//
// MessageId: VDS_E_ISCSI_SET_IKE_INFO
//
// MessageText:
//
// Call to set the iSCSI IKE info failed.
//
#define VDS_E_ISCSI_SET_IKE_INFO         ((HRESULT)0x80042906L)

//
// MessageId: VDS_E_SUBSYSTEM_ID_IS_NULL
//
// MessageText:
//
// The provider returned a NULL subsystem identification string.
//
#define VDS_E_SUBSYSTEM_ID_IS_NULL       ((HRESULT)0x80042907L)

//
// MessageId: VDS_E_ISCSI_INITIATOR_NODE_NAME
//
// MessageText:
//
// Failed to get the iSCSI initiator node name.
//
#define VDS_E_ISCSI_INITIATOR_NODE_NAME  ((HRESULT)0x80042908L)

//
// MessageId: VDS_E_ISCSI_GROUP_PRESHARE_KEY
//
// MessageText:
//
// Failed to set iSCSI group preshared key.
//
#define VDS_E_ISCSI_GROUP_PRESHARE_KEY   ((HRESULT)0x80042909L)

//
// MessageId: VDS_E_ISCSI_CHAP_SECRET
//
// MessageText:
//
// Failed to set iSCSI initiator CHAP secret.
//
#define VDS_E_ISCSI_CHAP_SECRET          ((HRESULT)0x8004290AL)

//
// MessageId: VDS_E_INVALID_IP_ADDRESS
//
// MessageText:
//
// An invalid IP address was encountered.
//
#define VDS_E_INVALID_IP_ADDRESS         ((HRESULT)0x8004290BL)

//
// MessageId: VDS_E_REBOOT_REQUIRED
//
// MessageText:
//
// A reboot is required before any further operations may be initiated. 
// If you do not reboot, machine behavior and machine state, are undefined 
// for any further operations.
//
#define VDS_E_REBOOT_REQUIRED            ((HRESULT)0x8004290CL)

//
// MessageId: VDS_E_VOLUME_GUID_PATHNAME_NOT_ALLOWED
//
// MessageText:
//
// Volume GUID pathnames are not valid input to this method.
//
#define VDS_E_VOLUME_GUID_PATHNAME_NOT_ALLOWED ((HRESULT)0x8004290DL)

//
// MessageId: VDS_E_BOOT_PAGEFILE_DRIVE_LETTER
//
// MessageText:
//
// Assigning or removing drive letters on the current boot or pagefile 
// volume is not allowed. 
//
#define VDS_E_BOOT_PAGEFILE_DRIVE_LETTER ((HRESULT)0x8004290EL)

//
// MessageId: VDS_E_DELETE_WITH_CRITICAL
//
// MessageText:
//
// Delete is not allowed on the current boot, system, pagefile,
// crashdump or hibernation volume. 
//
#define VDS_E_DELETE_WITH_CRITICAL       ((HRESULT)0x8004290FL)

//
// MessageId: VDS_E_CLEAN_WITH_DATA
//
// MessageText:
//
// The FORCE parameter must be set to TRUE in order to clean a disk
// that contains a data volume. 
//
#define VDS_E_CLEAN_WITH_DATA            ((HRESULT)0x80042910L)

//
// MessageId: VDS_E_CLEAN_WITH_OEM
//
// MessageText:
//
// The FORCE parameter must be set to TRUE in order to clean a disk
// that contains an OEM volume. 
//
#define VDS_E_CLEAN_WITH_OEM             ((HRESULT)0x80042911L)

//
// MessageId: VDS_E_CLEAN_WITH_CRITICAL
//
// MessageText:
//
// Clean is not allowed on the disk containing the current boot,
// system, pagefile, crashdump or hibernation volume. 
//
#define VDS_E_CLEAN_WITH_CRITICAL        ((HRESULT)0x80042912L)

//
// MessageId: VDS_E_FORMAT_CRITICAL
//
// MessageText:
//
// Format is not allowed on the current boot, system, pagefile,
// crashdump or hibernation volume. 
//
#define VDS_E_FORMAT_CRITICAL            ((HRESULT)0x80042913L)

//
// MessageId: VDS_E_NTFS_FORMAT_NOT_SUPPORTED
//
// MessageText:
//
// The NTFS file system format is not supported on this volume. 
//
#define VDS_E_NTFS_FORMAT_NOT_SUPPORTED  ((HRESULT)0x80042914L)

//
// MessageId: VDS_E_FAT32_FORMAT_NOT_SUPPORTED
//
// MessageText:
//
// The FAT32 file system format is not supported on this volume. 
//
#define VDS_E_FAT32_FORMAT_NOT_SUPPORTED ((HRESULT)0x80042915L)

//
// MessageId: VDS_E_FAT_FORMAT_NOT_SUPPORTED
//
// MessageText:
//
// The FAT file system format is not supported on this volume. 
//
#define VDS_E_FAT_FORMAT_NOT_SUPPORTED   ((HRESULT)0x80042916L)

//
// MessageId: VDS_E_FORMAT_NOT_SUPPORTED
//
// MessageText:
//
// The volume is not formattable.
//
#define VDS_E_FORMAT_NOT_SUPPORTED       ((HRESULT)0x80042917L)

//
// MessageId: VDS_E_COMPRESSION_NOT_SUPPORTED
//
// MessageText:
//
// The specified file system does not support compression.
//
#define VDS_E_COMPRESSION_NOT_SUPPORTED  ((HRESULT)0x80042918L)

//
// MessageId: VDS_E_VDISK_NOT_OPEN
//
// MessageText:
//
// The virtual disk object has not been open yet.
//
#define VDS_E_VDISK_NOT_OPEN             ((HRESULT)0x80042919L)

//
// MessageId: VDS_E_VDISK_INVALID_OP_STATE
//
// MessageText:
//
// The requested operation cannot be performed on the virtual disk 
// object, because it is not in a state that permits it.
//
#define VDS_E_VDISK_INVALID_OP_STATE     ((HRESULT)0x8004291AL)

//
// MessageId: VDS_E_INVALID_PATH
//
// MessageText:
//
// The path returned for the LUN is invalid. It has an incorrect path type 
// specified.
//
#define VDS_E_INVALID_PATH               ((HRESULT)0x8004291BL)

//
// MessageId: VDS_E_INVALID_ISCSI_PATH
//
// MessageText:
//
// The path returned for the LUN is invalid. Either it has an incorrect 
// path type specified, or, the initiator portal properties structure 
// is NULL.
//
#define VDS_E_INVALID_ISCSI_PATH         ((HRESULT)0x8004291CL)

//
// MessageId: VDS_E_SHRINK_LUN_NOT_UNMASKED
//
// MessageText:
//
// The SHRINK operation against the selected LUN cannot be completed. 
// The LUN is not unmasked to the local server.
//
#define VDS_E_SHRINK_LUN_NOT_UNMASKED    ((HRESULT)0x8004291DL)

//
// MessageId: VDS_E_LUN_DISK_READ_ONLY
//
// MessageText:
//
// The SHRINK operation against the selected LUN cannot be completed. 
// The current state of the disk associated with the LUN is READ ONLY.
//
#define VDS_E_LUN_DISK_READ_ONLY         ((HRESULT)0x8004291EL)

//
// MessageId: VDS_E_LUN_UPDATE_DISK
//
// MessageText:
//
// The operation against the selected LUN completed, but there was 
// a failure updating the status of the disk associated with the lun. 
// Call REFRESH to retry the status update for the disk.
//
#define VDS_E_LUN_UPDATE_DISK            ((HRESULT)0x8004291FL)

//
// MessageId: VDS_E_LUN_DYNAMIC
//
// MessageText:
//
// The SHRINK operation against the selected LUN cannot be completed. 
// The current state of the disk associated with the LUN is DYNAMIC.
//
#define VDS_E_LUN_DYNAMIC                ((HRESULT)0x80042920L)

//
// MessageId: VDS_E_LUN_DYNAMIC_OFFLINE
//
// MessageText:
//
// The SHRINK operation against the selected LUN cannot be completed. 
// The current state of the disk associated with the LUN is DYNAMIC 
// OFFLINE.
//
#define VDS_E_LUN_DYNAMIC_OFFLINE        ((HRESULT)0x80042921L)

//
// MessageId: VDS_E_LUN_SHRINK_GPT_HEADER
//
// MessageText:
//
// The SHRINK operation against the selected LUN cannot be completed. 
// The disk has the GPT partitioning format. The specified new lun 
// size does not allow space for a new GPT backup header to be created.
// Please increase the resulting lun size.
//
#define VDS_E_LUN_SHRINK_GPT_HEADER      ((HRESULT)0x80042922L)

//
// MessageId: VDS_E_MIRROR_NOT_SUPPORTED
//
// MessageText:
//
// Mirrored volumes are not supported by this operating system.
//
#define VDS_E_MIRROR_NOT_SUPPORTED       ((HRESULT)0x80042923L)

//
// MessageId: VDS_E_RAID5_NOT_SUPPORTED
//
// MessageText:
//
// RAID-5 volumes are not supported by this operating system.
//
#define VDS_E_RAID5_NOT_SUPPORTED        ((HRESULT)0x80042924L)

//
// MessageId: VDS_E_DISK_NOT_CONVERTIBLE_SIZE
//
// MessageText:
//
// The specified disk is not convertible because the size is less than
// the minimum size required for GPT disks.
//
#define VDS_E_DISK_NOT_CONVERTIBLE_SIZE  ((HRESULT)0x80042925L)

//
// MessageId: VDS_E_OFFLINE_NOT_SUPPORTED
//
// MessageText:
//
// The volume does not support offlining.
//
#define VDS_E_OFFLINE_NOT_SUPPORTED      ((HRESULT)0x80042926L)

//
// MessageId: VDS_E_VDISK_PATHNAME_INVALID
//
// MessageText:
//
// The pathname for a virtual disk must be fully qualified.
//
#define VDS_E_VDISK_PATHNAME_INVALID     ((HRESULT)0x80042927L)

//
// MessageId: VDS_E_EXTEND_TOO_MANY_CLUSTERS
//
// MessageText:
//
// The volume cannot be extended because the number of clusters will
// exceed the maximum number of clusters supported by the file system.
//
#define VDS_E_EXTEND_TOO_MANY_CLUSTERS   ((HRESULT)0x80042928L)

//
// MessageId: VDS_E_EXTEND_UNKNOWN_FILESYSTEM
//
// MessageText:
//
// The volume cannot be extended because the volume does not contain
// a recognized file system.
//
#define VDS_E_EXTEND_UNKNOWN_FILESYSTEM  ((HRESULT)0x80042929L)

//
// MessageId: VDS_E_SHRINK_UNKNOWN_FILESYSTEM
//
// MessageText:
//
// The volume cannot be shrunk because the volume does not contain
// a recognized file system.
//
#define VDS_E_SHRINK_UNKNOWN_FILESYSTEM  ((HRESULT)0x8004292AL)

//
// MessageId: VDS_E_VD_DISK_NOT_OPEN
//
// MessageText:
//
// The requested operation requires that the virtual disk be opened.
//
#define VDS_E_VD_DISK_NOT_OPEN           ((HRESULT)0x8004292BL)

//
// MessageId: VDS_E_VD_DISK_IS_EXPANDING
//
// MessageText:
//
// The requested operation cannot be performed while the virtual disk
// is expanding.
//
#define VDS_E_VD_DISK_IS_EXPANDING       ((HRESULT)0x8004292CL)

//
// MessageId: VDS_E_VD_DISK_IS_COMPACTING
//
// MessageText:
//
// The requested operation cannot be performed while the virtual disk
// is compacting.
//
#define VDS_E_VD_DISK_IS_COMPACTING      ((HRESULT)0x8004292DL)

//
// MessageId: VDS_E_VD_DISK_IS_MERGING
//
// MessageText:
//
// The requested operation cannot be performed while the virtual disk
// is merging.
//
#define VDS_E_VD_DISK_IS_MERGING         ((HRESULT)0x8004292EL)

//
// MessageId: VDS_E_VD_IS_ATTACHED
//
// MessageText:
//
// The requested operation cannot be performed while the virtual disk
// is attached.
//
#define VDS_E_VD_IS_ATTACHED             ((HRESULT)0x8004292FL)

//
// MessageId: VDS_E_VD_DISK_ALREADY_OPEN
//
// MessageText:
//
// The virtual disk is already open and cannot be opened
// a second time. Please close all clients that may have opened
// the virtual disk and retry.
//
#define VDS_E_VD_DISK_ALREADY_OPEN       ((HRESULT)0x80042930L)

//
// MessageId: VDS_E_VD_DISK_ALREADY_EXPANDING
//
// MessageText:
//
// The virtual disk is already in the process of expanding.
//
#define VDS_E_VD_DISK_ALREADY_EXPANDING  ((HRESULT)0x80042931L)

//
// MessageId: VDS_E_VD_ALREADY_COMPACTING
//
// MessageText:
//
// The virtual disk is already in the process of compacting.
//
#define VDS_E_VD_ALREADY_COMPACTING      ((HRESULT)0x80042932L)

//
// MessageId: VDS_E_VD_ALREADY_MERGING
//
// MessageText:
//
// The virtual disk is already in the process of merging.
//
#define VDS_E_VD_ALREADY_MERGING         ((HRESULT)0x80042933L)

//
// MessageId: VDS_E_VD_ALREADY_ATTACHED
//
// MessageText:
//
// The virtual disk is already attached.
//
#define VDS_E_VD_ALREADY_ATTACHED        ((HRESULT)0x80042934L)

//
// MessageId: VDS_E_VD_ALREADY_DETACHED
//
// MessageText:
//
// The virtual disk is already detached.
//
#define VDS_E_VD_ALREADY_DETACHED        ((HRESULT)0x80042935L)

//
// MessageId: VDS_E_VD_NOT_ATTACHED_READONLY
//
// MessageText:
//
// The requested operation requires that the virtual disk be
// attached read only.
//
#define VDS_E_VD_NOT_ATTACHED_READONLY   ((HRESULT)0x80042936L)

//
// MessageId: VDS_E_VD_IS_BEING_ATTACHED
//
// MessageText:
//
// The requested operation cannot be performed while the virtual disk
// is being attached.
//
#define VDS_E_VD_IS_BEING_ATTACHED       ((HRESULT)0x80042937L)

//
// MessageId: VDS_E_VD_IS_BEING_DETACHED
//
// MessageText:
//
// The requested operation cannot be performed while the virtual disk
// is being detached.
//
#define VDS_E_VD_IS_BEING_DETACHED       ((HRESULT)0x80042938L)

//////////////////////////////////////////////////////////////////////////////
//
// Codes added in Win8:     0x2A00-0x2AFF
//
//////////////////////////////////////////////////////////////////////////////
//
// MessageId: VDS_E_NO_POOL
//
// MessageText:
//
// The drive is not contained in a pool.
//
#define VDS_E_NO_POOL                    ((HRESULT)0x80042A00L)

//
// MessageId: VDS_E_NO_POOL_CREATED
//
// MessageText:
//
// No pool is created. 
//
#define VDS_E_NO_POOL_CREATED            ((HRESULT)0x80042A01L)

//
// MessageId: VDS_E_NO_MAINTENANCE_MODE
//
// MessageText:
//
// The specified disk or volume is managed by the Microsoft Failover Clustering component. The disk must be in cluster maintenance mode and the cluster resource status must be online to perform this operation. 
//
#define VDS_E_NO_MAINTENANCE_MODE        ((HRESULT)0x80042A02L)

//
// MessageId: VDS_E_BLOCK_CLUSTERED
//
// MessageText:
//
// The specified disk or volume is managed by Microsoft Failover Clustering. The disk must be removed from the cluster to perform this operation. 
//
#define VDS_E_BLOCK_CLUSTERED            ((HRESULT)0x80042A03L)

//
// MessageId: VDS_E_DISK_HAS_BANDS
//
// MessageText:
//
// The disk could not be converted to dynamic because security is enabled
// on one or more partitions. 
//
#define VDS_E_DISK_HAS_BANDS             ((HRESULT)0x80042A04L)

//
// MessageId: VDS_E_INVALID_STATE
//
// MessageText:
//
// The requested operation cannot be performed on the object, because it is not in a state that permits it.
// Call Refresh and retry the operation.
//
#define VDS_E_INVALID_STATE              ((HRESULT)0x80042A05L)

//
// MessageId: VDS_E_REFS_FORMAT_NOT_SUPPORTED
//
// MessageText:
//
// The ReFS file system format is not supported on this volume. 
//
#define VDS_E_REFS_FORMAT_NOT_SUPPORTED  ((HRESULT)0x80042A06L)

//
// MessageId: VDS_E_DELETE_WITH_BOOTBACKING
//
// MessageText:
//
// Delete is not allowed on the volume that backs your Windows boot volume. 
//
#define VDS_E_DELETE_WITH_BOOTBACKING    ((HRESULT)0x80042A07L)

//
// MessageId: VDS_E_FORMAT_WITH_BOOTBACKING
//
// MessageText:
//
// Format is not allowed on the volume that backs your Windows boot volume. 
//
#define VDS_E_FORMAT_WITH_BOOTBACKING    ((HRESULT)0x80042A08L)

//
// MessageId: VDS_E_CLEAN_WITH_BOOTBACKING
//
// MessageText:
//
// 
// Clean is not allowed on the disk containing volume that backs your Windows boot volume.
//
#define VDS_E_CLEAN_WITH_BOOTBACKING     ((HRESULT)0x80042A09L)

//////////////////////////////////////////////////////////////////////////////
//
// Codes added in redstone fe:     0x2B00-0x2BFF
//
//////////////////////////////////////////////////////////////////////////////
//
// MessageId: VDS_E_SHRINK_EXTEND_UNALIGNED
//
// MessageText:
//
// 
// The specified shrink or extend size does not meet alignment requirements for the device.
//
#define VDS_E_SHRINK_EXTEND_UNALIGNED    ((HRESULT)0x80042B00L)

