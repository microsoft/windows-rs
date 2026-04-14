// sherrors.h - Shell API error code values
// Copyright (c) Microsoft Corporation. All rights reserved.

#ifndef _SHERROR_
#define _SHERROR_
#if defined (_MSC_VER) && (_MSC_VER >= 1020) && !defined(__midl)
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <winerror.h>

//  COPYENGINE codes use FACILITY_SHELL and 0 in the second lowest byte
// Success/Informational codes
#define COPYENGINE_S_YES                         _HRESULT_TYPEDEF_(0x00270001L)
#define COPYENGINE_S_NOT_HANDLED                 _HRESULT_TYPEDEF_(0x00270003L)
#define COPYENGINE_S_USER_RETRY                  _HRESULT_TYPEDEF_(0x00270004L)
#define COPYENGINE_S_USER_IGNORED                _HRESULT_TYPEDEF_(0x00270005L)
#define COPYENGINE_S_MERGE                       _HRESULT_TYPEDEF_(0x00270006L)
#define COPYENGINE_S_DONT_PROCESS_CHILDREN       _HRESULT_TYPEDEF_(0x00270008L)
#define COPYENGINE_S_ALREADY_DONE                _HRESULT_TYPEDEF_(0x0027000AL)
#define COPYENGINE_S_PENDING                     _HRESULT_TYPEDEF_(0x0027000BL)
#define COPYENGINE_S_KEEP_BOTH                   _HRESULT_TYPEDEF_(0x0027000CL)
#define COPYENGINE_S_CLOSE_PROGRAM               _HRESULT_TYPEDEF_(0x0027000DL) // Close the program using the current file
#define COPYENGINE_S_COLLISIONRESOLVED           _HRESULT_TYPEDEF_(0x0027000EL) // Returned by a transfer source when a collision occurs
                                                                                // during a file operation but the source resolved the collision
                                                                                // on the users behalf
#define COPYENGINE_S_PROGRESS_PAUSE              _HRESULT_TYPEDEF_(0x0027000FL)
#define COPYENGINE_S_PENDING_DELETE              _HRESULT_TYPEDEF_(0x00270010L) // Returned by initiating async delete operation. It's different
                                                                                // from COPYENGINE_S_PENDING which is used for operation queued due to error.
#define COPYENGINE_S_PENDING_BATCH_COPY          _HRESULT_TYPEDEF_(0x00270011L) // Returned by initiating batch copy operation. Like COPYENGINE_S_PENDING_DELETE,
                                                                                // it indicates a non-error queued retry of the operation.

// Failure/Error codes
#define COPYENGINE_E_USER_CANCELLED      _HRESULT_TYPEDEF_(0x80270000L)  // User wants to canceled entire job
#define COPYENGINE_E_CANCELLED           _HRESULT_TYPEDEF_(0x80270001L)  // Engine wants to canceled entire job, don't set the CANCELLED bit
#define COPYENGINE_E_REQUIRES_ELEVATION  _HRESULT_TYPEDEF_(0x80270002L)  // Need to elevate the process to complete the operation

#define COPYENGINE_E_SAME_FILE           _HRESULT_TYPEDEF_(0x80270003L)  // Source and destination file are the same
#define COPYENGINE_E_DIFF_DIR            _HRESULT_TYPEDEF_(0x80270004L)  // Trying to rename a file into a different location, use move instead
#define COPYENGINE_E_MANY_SRC_1_DEST     _HRESULT_TYPEDEF_(0x80270005L)  // One source specified, multiple destinations

#define COPYENGINE_E_DEST_SUBTREE        _HRESULT_TYPEDEF_(0x80270009L)  // The destination is a sub-tree of the source
#define COPYENGINE_E_DEST_SAME_TREE      _HRESULT_TYPEDEF_(0x8027000AL)  // The destination is the same folder as the source

#define COPYENGINE_E_FLD_IS_FILE_DEST    _HRESULT_TYPEDEF_(0x8027000BL)  // Existing destination file with same name as folder
#define COPYENGINE_E_FILE_IS_FLD_DEST    _HRESULT_TYPEDEF_(0x8027000CL)  // Existing destination folder with same name as file

#define COPYENGINE_E_FILE_TOO_LARGE      _HRESULT_TYPEDEF_(0x8027000DL)  // File too large for destination file system
#define COPYENGINE_E_REMOVABLE_FULL      _HRESULT_TYPEDEF_(0x8027000EL)  // Destination device is full and happens to be removable

#define COPYENGINE_E_DEST_IS_RO_CD       _HRESULT_TYPEDEF_(0x8027000FL)  // Destination is a Read-Only CDRom, possibly unformatted
#define COPYENGINE_E_DEST_IS_RW_CD       _HRESULT_TYPEDEF_(0x80270010L)  // Destination is a Read/Write CDRom, possibly unformatted
#define COPYENGINE_E_DEST_IS_R_CD        _HRESULT_TYPEDEF_(0x80270011L)  // Destination is a Recordable (AudioL) CDRom, possibly unformatted

#define COPYENGINE_E_DEST_IS_RO_DVD      _HRESULT_TYPEDEF_(0x80270012L)  // Destination is a Read-Only DVD, possibly unformatted
#define COPYENGINE_E_DEST_IS_RW_DVD      _HRESULT_TYPEDEF_(0x80270013L)  // Destination is a Read/Wrote DVD, possibly unformatted
#define COPYENGINE_E_DEST_IS_R_DVD       _HRESULT_TYPEDEF_(0x80270014L)  // Destination is a Recordable (AudioL) DVD, possibly unformatted

#define COPYENGINE_E_SRC_IS_RO_CD        _HRESULT_TYPEDEF_(0x80270015L)  // Source is a Read-Only CDRom, possibly unformatted
#define COPYENGINE_E_SRC_IS_RW_CD        _HRESULT_TYPEDEF_(0x80270016L)  // Source is a Read/Write CDRom, possibly unformatted
#define COPYENGINE_E_SRC_IS_R_CD         _HRESULT_TYPEDEF_(0x80270017L)  // Source is a Recordable (AudioL) CDRom, possibly unformatted

#define COPYENGINE_E_SRC_IS_RO_DVD       _HRESULT_TYPEDEF_(0x80270018L)  // Source is a Read-Only DVD, possibly unformatted
#define COPYENGINE_E_SRC_IS_RW_DVD       _HRESULT_TYPEDEF_(0x80270019L)  // Source is a Read/Wrote DVD, possibly unformatted
#define COPYENGINE_E_SRC_IS_R_DVD        _HRESULT_TYPEDEF_(0x8027001AL)  // Source is a Recordable (AudioL) DVD, possibly unformatted

#define COPYENGINE_E_INVALID_FILES_SRC   _HRESULT_TYPEDEF_(0x8027001BL)  // Invalid source path
#define COPYENGINE_E_INVALID_FILES_DEST  _HRESULT_TYPEDEF_(0x8027001CL)  // Invalid destination path
#define COPYENGINE_E_PATH_TOO_DEEP_SRC   _HRESULT_TYPEDEF_(0x8027001DL)  // Source Files within folders where the overall path is longer than MAX_PATH
#define COPYENGINE_E_PATH_TOO_DEEP_DEST  _HRESULT_TYPEDEF_(0x8027001EL)  // Destination files would be within folders where the overall path is longer than MAX_PATH
#define COPYENGINE_E_ROOT_DIR_SRC        _HRESULT_TYPEDEF_(0x8027001FL)  // Source is a root directory, cannot be moved or renamed
#define COPYENGINE_E_ROOT_DIR_DEST       _HRESULT_TYPEDEF_(0x80270020L)  // Destination is a root directory, cannot be renamed
#define COPYENGINE_E_ACCESS_DENIED_SRC   _HRESULT_TYPEDEF_(0x80270021L)  // Security problem on source
#define COPYENGINE_E_ACCESS_DENIED_DEST  _HRESULT_TYPEDEF_(0x80270022L)  // Security problem on destination
#define COPYENGINE_E_PATH_NOT_FOUND_SRC  _HRESULT_TYPEDEF_(0x80270023L)  // Source file does not exist, or is unavailable
#define COPYENGINE_E_PATH_NOT_FOUND_DEST _HRESULT_TYPEDEF_(0x80270024L)  // Destination file does not exist, or is unavailable
#define COPYENGINE_E_NET_DISCONNECT_SRC  _HRESULT_TYPEDEF_(0x80270025L)  // Source file is on a disconnected network location
#define COPYENGINE_E_NET_DISCONNECT_DEST        _HRESULT_TYPEDEF_(0x80270026L)  // Destination file is on a disconnected network location
#define COPYENGINE_E_SHARING_VIOLATION_SRC      _HRESULT_TYPEDEF_(0x80270027L)  // Sharing Violation on source
#define COPYENGINE_E_SHARING_VIOLATION_DEST     _HRESULT_TYPEDEF_(0x80270028L)  // Sharing Violation on destination

#define COPYENGINE_E_ALREADY_EXISTS_NORMAL      _HRESULT_TYPEDEF_(0x80270029L) // Destination exists, cannot replace
#define COPYENGINE_E_ALREADY_EXISTS_READONLY    _HRESULT_TYPEDEF_(0x8027002AL) // Destination with read-only attribute exists, cannot replace
#define COPYENGINE_E_ALREADY_EXISTS_SYSTEM      _HRESULT_TYPEDEF_(0x8027002BL) // Destination with system attribute exists, cannot replace
#define COPYENGINE_E_ALREADY_EXISTS_FOLDER      _HRESULT_TYPEDEF_(0x8027002CL) // Destination folder exists, cannot replace
#define COPYENGINE_E_STREAM_LOSS                _HRESULT_TYPEDEF_(0x8027002DL) // Secondary Stream information would be lost
#define COPYENGINE_E_EA_LOSS                    _HRESULT_TYPEDEF_(0x8027002EL) // Extended Attributes would be lost
#define COPYENGINE_E_PROPERTY_LOSS              _HRESULT_TYPEDEF_(0x8027002FL) // Property would be lost
#define COPYENGINE_E_PROPERTIES_LOSS            _HRESULT_TYPEDEF_(0x80270030L) // Properties would be lost
#define COPYENGINE_E_ENCRYPTION_LOSS            _HRESULT_TYPEDEF_(0x80270031L) // Encryption would be lost
#define COPYENGINE_E_DISK_FULL                  _HRESULT_TYPEDEF_(0x80270032L) // Entire operation likely won't fit
#define COPYENGINE_E_DISK_FULL_CLEAN            _HRESULT_TYPEDEF_(0x80270033L) // Entire operation likely won't fit, clean-up wizard available
#define COPYENGINE_E_EA_NOT_SUPPORTED           _HRESULT_TYPEDEF_(0x80270034L) // Volume does not support Extended Attributes
#define COPYENGINE_E_CANT_REACH_SOURCE          _HRESULT_TYPEDEF_(0x80270035L) // Can't reach source folder

#define COPYENGINE_E_RECYCLE_UNKNOWN_ERROR      _HRESULT_TYPEDEF_(0x80270035L) // ???
#define COPYENGINE_E_RECYCLE_FORCE_NUKE         _HRESULT_TYPEDEF_(0x80270036L) // Recycling not available (usually turned offL)
#define COPYENGINE_E_RECYCLE_SIZE_TOO_BIG       _HRESULT_TYPEDEF_(0x80270037L) // Item is too large for the recycle-bin
#define COPYENGINE_E_RECYCLE_PATH_TOO_LONG      _HRESULT_TYPEDEF_(0x80270038L) // Folder is too deep to fit in the recycle-bin
#define COPYENGINE_E_RECYCLE_BIN_NOT_FOUND      _HRESULT_TYPEDEF_(0x8027003AL) // Recycle bin could not be found or is unavailable
#define COPYENGINE_E_NEWFILE_NAME_TOO_LONG      _HRESULT_TYPEDEF_(0x8027003BL) // Name of the new file being created is too long
#define COPYENGINE_E_NEWFOLDER_NAME_TOO_LONG    _HRESULT_TYPEDEF_(0x8027003CL) // Name of the new folder being created is too long
#define COPYENGINE_E_DIR_NOT_EMPTY              _HRESULT_TYPEDEF_(0x8027003DL) // The directory being processed is not empty
#define COPYENGINE_E_FAT_MAX_IN_ROOT            _HRESULT_TYPEDEF_(0x8027003EL) // A Fat drive cannot only store and rename a limited number of items on the root.
#define COPYENGINE_E_ACCESSDENIED_READONLY      _HRESULT_TYPEDEF_(0x8027003FL) // The item cannot be modified because it is set to readonly.

#define COPYENGINE_E_REDIRECTED_TO_WEBPAGE      _HRESULT_TYPEDEF_(0x80270040L) // The server redirected the download request to a web page.
#define COPYENGINE_E_SERVER_BAD_FILE_TYPE       _HRESULT_TYPEDEF_(0x80270041L) // The server returned data with an unexpected MIME type or extension.

#define COPYENGINE_E_INTERNET_ITEM_UNAVAILABLE  _HRESULT_TYPEDEF_(0x80270042L) // The item is unavailable currently due to no internet connectivity

#define COPYENGINE_E_CANNOT_MOVE_FROM_RECYCLE_BIN _HRESULT_TYPEDEF_(0x80270043L) // The item cannot be moved out of the recycle bin, can perform other operations like restored/delete

#define COPYENGINE_E_CANNOT_MOVE_SHARED_FOLDER _HRESULT_TYPEDEF_(0x80270044L) // The item cannot be moved from a grouped folder to another shared folder

#define COPYENGINE_E_INTERNET_ITEM_STORAGE_PROVIDER_ERROR   _HRESULT_TYPEDEF_(0x80270045L) // The item is unavailable currently due to storage provider being in error state
#define COPYENGINE_E_INTERNET_ITEM_STORAGE_PROVIDER_PAUSED  _HRESULT_TYPEDEF_(0x80270046L) // The item is unavailable currently due to storage provider being paused

#define COPYENGINE_E_REQUIRES_EDP_CONSENT                          _HRESULT_TYPEDEF_(0x80270047L) // The file can only be copied if the user consents to override the EDP block
#define COPYENGINE_E_BLOCKED_BY_EDP_POLICY                         _HRESULT_TYPEDEF_(0x80270048L) // The file cannot be copied per EDP policy
#define COPYENGINE_E_REQUIRES_EDP_CONSENT_FOR_REMOVABLE_DRIVE      _HRESULT_TYPEDEF_(0x80270049L) // The file can be copied as personal if the user consents to override the EDP block
#define COPYENGINE_E_BLOCKED_BY_EDP_FOR_REMOVABLE_DRIVE            _HRESULT_TYPEDEF_(0x8027004AL) // The file can only be copied as work protected per EDP policy
#define COPYENGINE_E_RMS_REQUIRES_EDP_CONSENT_FOR_REMOVABLE_DRIVE  _HRESULT_TYPEDEF_(0x8027004BL) // The file can be copied as personal if the user consents to override the EDP block
#define COPYENGINE_E_RMS_BLOCKED_BY_EDP_FOR_REMOVABLE_DRIVE        _HRESULT_TYPEDEF_(0x8027004CL) // The file can only be copied as work protected per EDP policy

#define COPYENGINE_E_WARNED_BY_DLP_POLICY                          _HRESULT_TYPEDEF_(0x8027004DL) // The file is warned against being copied per DLP policy
#define COPYENGINE_E_BLOCKED_BY_DLP_POLICY                         _HRESULT_TYPEDEF_(0x8027004EL) // The file cannot be copied per DLP policy
#define COPYENGINE_E_SILENT_FAIL_BY_DLP_POLICY                     _HRESULT_TYPEDEF_(0x8027004FL) // The file cannot be copied per DLP policy, and the caller is requested to silently fail

#define COPYENGINE_E_SUPPRESS_DIALOG                               _HRESULT_TYPEDEF_(0x80270050L) // Returned by copy engine to indicate that the shell dialog should not be shown in favor to the cloud provider's or the app's dialog

//  error codes without a more specific group use FACILITY_SHELL and 0x01 in the second lowest byte.
#define NETCACHE_E_NEGATIVE_CACHE           _HRESULT_TYPEDEF_(0x80270100L) // The item requested is in the negative net parsing cache
#define EXECUTE_E_LAUNCH_APPLICATION        _HRESULT_TYPEDEF_(0x80270101L) // for returned by command delegates to indicate that they did no work 
#define SHELL_E_WRONG_BITDEPTH              _HRESULT_TYPEDEF_(0x80270102L) // returned when trying to create a thumbnail extractor at too low a bitdepth for high fidelity
#define LINK_E_DELETE                       _HRESULT_TYPEDEF_(0x80270103L) // returned from IShellLink::Resolve when SLR_OFFER_DELETE_WITHOUT_FILE is passed and the user requested to delete the item
#define STORE_E_NEWER_VERSION_AVAILABLE     _HRESULT_TYPEDEF_(0x80270104L) // returned from IAppItemsStateModify to indicate a commit failed because there is newer version already available

// File Placeholder success and error codes
#define E_FILE_PLACEHOLDER_NOT_INITIALIZED              _HRESULT_TYPEDEF_(0x80270110L) // The placeholder file or property store isn't initialized.
#define E_FILE_PLACEHOLDER_VERSION_MISMATCH             _HRESULT_TYPEDEF_(0x80270111L) // The sync engine detected that the local placeholder file's version doesn't match the latest version.
#define E_FILE_PLACEHOLDER_SERVER_TIMED_OUT             _HRESULT_TYPEDEF_(0x80270112L) // The placeholder platform timed out waiting for a stream resolver call that didn't complete in time.
#define E_FILE_PLACEHOLDER_STORAGEPROVIDER_NOT_FOUND    _HRESULT_TYPEDEF_(0x80270113L) // The placeholder platform cannot find the storage provider for the placeholder.

// Camera Roll error codes
#define CAMERAROLL_E_NO_DOWNSAMPLING_REQUIRED   _HRESULT_TYPEDEF_(0x80270120L) // The provided image didn't require downsampling because it was small enough already

// Error codes for when Shell denies a WinRT app activation request via Window Manager/View Manager.
// These should be considered subsets of NAV_E_SHELLVALIDATIONFAILED.
// By the time the activation request reached shell, the scenario was already over and the activation is intentionally denied (not an error).
#define E_ACTIVATIONDENIED_USERCLOSE    _HRESULT_TYPEDEF_(0x80270130L)
// The shell hit a critical error while handling the activation request and cannot process it correctly, so the activation must be denied.
#define E_ACTIVATIONDENIED_SHELLERROR   _HRESULT_TYPEDEF_(0x80270131L)
// The shell restarted while the activation was in-progress, and it is unable to process it properly.
#define E_ACTIVATIONDENIED_SHELLRESTART _HRESULT_TYPEDEF_(0x80270132L)
// Placeholder for an unexpected error that we cannot definitively map into one of the above buckets. Not expected to fire in practice.
#define E_ACTIVATIONDENIED_UNEXPECTED   _HRESULT_TYPEDEF_(0x80270133L)
// The shell was not ready when the activation started, so the activation was aborted immediately.
#define E_ACTIVATIONDENIED_SHELLNOTREADY _HRESULT_TYPEDEF_(0x80270134L)

// Library error/failure code
#define LIBRARY_E_NO_SAVE_LOCATION          _HRESULT_TYPEDEF_(0x80270200L)
#define LIBRARY_E_NO_ACCESSIBLE_LOCATION    _HRESULT_TYPEDEF_(0x80270201L)


// User Tile error/failure codes
#define E_USERTILE_UNSUPPORTEDFILETYPE      _HRESULT_TYPEDEF_(0x80270210L)

// User Tile error codes that map to the WinRT enum Windows.System.UserProfile.SetAccountPictureResult
#define E_USERTILE_CHANGEDISABLED           _HRESULT_TYPEDEF_(0x80270211L)
#define E_USERTILE_LARGEORDYNAMIC           _HRESULT_TYPEDEF_(0x80270212L)
#define E_USERTILE_VIDEOFRAMESIZE           _HRESULT_TYPEDEF_(0x80270213L)
#define E_USERTILE_FILESIZE                 _HRESULT_TYPEDEF_(0x80270214L)


// Immersive Accessibility Docking Service Error Codes
#define IMM_ACC_DOCKING_E_INSUFFICIENTHEIGHT    _HRESULT_TYPEDEF_(0x80270230L)
#define IMM_ACC_DOCKING_E_DOCKOCCUPIED          _HRESULT_TYPEDEF_(0x80270231L)

// Immersive Shell Startup Failure Code
#define IMSC_E_SHELL_COMPONENT_STARTUP_FAILURE  _HRESULT_TYPEDEF_(0x80270233L) // An immersive shell component failed to start correctly

// Shell Service Host Startup Failure Code
#define SHC_E_SHELL_COMPONENT_STARTUP_FAILURE  _HRESULT_TYPEDEF_(0x80270234L) // A shell service host component failed to start correctly

// Tile notifications - Failure to initialize the notification platform
#define E_TILE_NOTIFICATIONS_PLATFORM_FAILURE		_HRESULT_TYPEDEF_(0x80270249L)

#define E_SHELL_EXTENSION_BLOCKED           _HRESULT_TYPEDEF_(0x80270301L)

#define E_IMAGEFEED_CHANGEDISABLED           _HRESULT_TYPEDEF_(0x80270310L)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //  _SHERROR_

