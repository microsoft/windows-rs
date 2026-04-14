// FileSystemImageMsg.h

// Help for constructing this file was provided by
// ms-help://MS.MSDNQTR.2003FEB.1033/tools/tools/about_message_text_files.htm
// Refer to this documentation for message text syntax.
#pragma once
//Since the message codes are also included in the header file produced from the .idl, don't include them here
#ifndef _SKIP_FSI_ERROR_MESSAGE_CODES
// -----  Catch-all error(s) -- should not actually occur, indicative of internal state error  --------
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
#define FACILITY_IMAPI2                  0xAA


//
// Define the severity codes
//


//
// MessageId: IMAPI_E_FSI_INTERNAL_ERROR
//
// MessageText:
//
// Internal file system error occurred.
//
#define IMAPI_E_FSI_INTERNAL_ERROR       ((HRESULT)0xC0AAB100L)

// ----------------  Miscellenous interface errors   ------------------
//
// MessageId: IMAPI_E_INVALID_PARAM
//
// MessageText:
//
// The value specified for parameter '%1!ls!' is not valid.
//
#define IMAPI_E_INVALID_PARAM            ((HRESULT)0xC0AAB101L)

//  NOTE:  the quote characters delimiting the parameter name are used by the Managed Shim
//         to parse the parameter name from the message text for exception handling
//         Don't remove or change these quote-delimiters w/o also changing the exception mapping
//         in the managed shim.
//
// MessageId: IMAPI_E_READONLY
//
// MessageText:
//
// FileSystemImage object is in read only mode.
//
#define IMAPI_E_READONLY                 ((HRESULT)0xC0AAB102L)

//
// MessageId: IMAPI_E_NO_OUTPUT
//
// MessageText:
//
// No output file system specified.
//
#define IMAPI_E_NO_OUTPUT                ((HRESULT)0xC0AAB103L)

//
// MessageId: IMAPI_E_INVALID_VOLUME_NAME
//
// MessageText:
//
// The specified Volume Identifier is either too long or contains one or more invalid characters.
//
#define IMAPI_E_INVALID_VOLUME_NAME      ((HRESULT)0xC0AAB104L)

//
// MessageId: IMAPI_E_INVALID_DATE
//
// MessageText:
//
// Invalid file dates.  %1!ls! time is earlier than %2!ls! time.
//
#define IMAPI_E_INVALID_DATE             ((HRESULT)0xC0AAB105L)

//
// MessageId: IMAPI_E_FILE_SYSTEM_NOT_EMPTY
//
// MessageText:
//
// The file system must be empty for this function.
//
#define IMAPI_E_FILE_SYSTEM_NOT_EMPTY    ((HRESULT)0xC0AAB106L)

// ----------------  Errors associated with state of item   ------------------
//
// MessageId: IMAPI_E_NOT_FILE
//
// MessageText:
//
// Specified path '%1!ls!' does not identify a file.
//
#define IMAPI_E_NOT_FILE                 ((HRESULT)0xC0AAB108L)

//
// MessageId: IMAPI_E_NOT_DIR
//
// MessageText:
//
// Specified path '%1!ls!' does not identify a directory.
//
#define IMAPI_E_NOT_DIR                  ((HRESULT)0xC0AAB109L)

//
// MessageId: IMAPI_E_DIR_NOT_EMPTY
//
// MessageText:
//
// The directory '%1!s!' is not empty.
//
#define IMAPI_E_DIR_NOT_EMPTY            ((HRESULT)0xC0AAB10AL)

//
// MessageId: IMAPI_E_NOT_IN_FILE_SYSTEM
//
// MessageText:
//
// '%1!ls!' is not part of the file system.  It must be added to complete this operation.
//
#define IMAPI_E_NOT_IN_FILE_SYSTEM       ((HRESULT)0xC0AAB10BL)

// ----------------  Errors associated with file/directory naming problems   ------------------
//
// MessageId: IMAPI_E_INVALID_PATH
//
// MessageText:
//
// Path '%1!s!' is badly formed or contains invalid characters.
//
#define IMAPI_E_INVALID_PATH             ((HRESULT)0xC0AAB110L)

//
// MessageId: IMAPI_E_RESTRICTED_NAME_VIOLATION
//
// MessageText:
//
// The name '%1!ls!' specified is not legal:  Name of file or directory object created while the UseRestrictedCharacterSet property is set may only contain ANSI characters.
//
#define IMAPI_E_RESTRICTED_NAME_VIOLATION ((HRESULT)0xC0AAB111L)

//
// MessageId: IMAPI_E_DUP_NAME
//
// MessageText:
//
// '%1!ls!' name already exists.
//
#define IMAPI_E_DUP_NAME                 ((HRESULT)0xC0AAB112L)

//
// MessageId: IMAPI_E_NO_UNIQUE_NAME
//
// MessageText:
//
// Attempt to add '%1!ls!' failed:  cannot create a file-system-specific unique name for the %2!ls! file system.
//
#define IMAPI_E_NO_UNIQUE_NAME           ((HRESULT)0xC0AAB113L)

// --------------  Errors reported when specified item not found in hierarchy  ---------------
//
// MessageId: IMAPI_E_ITEM_NOT_FOUND
//
// MessageText:
//
// Cannot find item '%1!ls!' in FileSystemImage hierarchy.
//
#define IMAPI_E_ITEM_NOT_FOUND           ((HRESULT)0xC0AAB118L)

//
// MessageId: IMAPI_E_FILE_NOT_FOUND
//
// MessageText:
//
// The file '%1!s!' not found in FileSystemImage hierarchy.
//
#define IMAPI_E_FILE_NOT_FOUND           ((HRESULT)0xC0AAB119L)

//
// MessageId: IMAPI_E_DIR_NOT_FOUND
//
// MessageText:
//
// The directory '%1!s!' not found in FileSystemImage hierarchy.
//
#define IMAPI_E_DIR_NOT_FOUND            ((HRESULT)0xC0AAB11AL)

// ----------------  Image-size exceeds limit errors  ------------------
//
// MessageId: IMAPI_E_IMAGE_SIZE_LIMIT
//
// MessageText:
//
// Adding '%1!ls!' would result in a result image having a size larger than the current configured limit.
//
#define IMAPI_E_IMAGE_SIZE_LIMIT         ((HRESULT)0xC0AAB120L)

//
// MessageId: IMAPI_E_IMAGE_TOO_BIG
//
// MessageText:
//
// Value specified for FreeMediaBlocks property is too small for estimated image size based on current data.  
//
#define IMAPI_E_IMAGE_TOO_BIG            ((HRESULT)0xC0AAB121L)

// ----------------  User file-data stream errors  ------------------
//
// MessageId: IMAPI_E_DATA_STREAM_INCONSISTENCY
//
// MessageText:
//
// Data stream supplied for file '%1!ls!' is inconsistent:  expected %2!I64d! bytes, found %3!I64d!. 
//
#define IMAPI_E_DATA_STREAM_INCONSISTENCY ((HRESULT)0xC0AAB128L)

//
// MessageId: IMAPI_E_DATA_STREAM_READ_FAILURE
//
// MessageText:
//
// Cannot read data from stream supplied for file '%1!ls!'.
//
#define IMAPI_E_DATA_STREAM_READ_FAILURE ((HRESULT)0xC0AAB129L)

//
// MessageId: IMAPI_E_DATA_STREAM_CREATE_FAILURE
//
// MessageText:
//
// The following error was encountered when trying to create data stream for '%1!ls!':  
//
#define IMAPI_E_DATA_STREAM_CREATE_FAILURE ((HRESULT)0xC0AAB12AL)

//
// MessageId: IMAPI_E_DIRECTORY_READ_FAILURE
//
// MessageText:
//
// The following error was encountered when trying to enumerate files in directory '%1!ls!':  
//
#define IMAPI_E_DIRECTORY_READ_FAILURE   ((HRESULT)0xC0AAB12BL)

// ----------------  FileSystem-specific limitations  ------------------
//
// MessageId: IMAPI_E_TOO_MANY_DIRS
//
// MessageText:
//
// This file system image has too many directories for the %1!ls! file system.
//
#define IMAPI_E_TOO_MANY_DIRS            ((HRESULT)0xC0AAB130L)

//
// MessageId: IMAPI_E_ISO9660_LEVELS
//
// MessageText:
//
// ISO9660 is limited to 8 levels of directories.
//
#define IMAPI_E_ISO9660_LEVELS           ((HRESULT)0xC0AAB131L)

//
// MessageId: IMAPI_E_DATA_TOO_BIG
//
// MessageText:
//
// Data file is too large for '%1!ls!' file system.
//
#define IMAPI_E_DATA_TOO_BIG             ((HRESULT)0xC0AAB132L)

//
// MessageId: IMAPI_E_INCOMPATIBLE_PREVIOUS_SESSION
//
// MessageText:
//
// Operation failed because of incompatible layout of the previous session imported from the medium.
//
#define IMAPI_E_INCOMPATIBLE_PREVIOUS_SESSION ((HRESULT)0xC0AAB133L)

// ----------------  Errors associated with stash-file operations   ------------------
//
// MessageId: IMAPI_E_STASHFILE_OPEN_FAILURE
//
// MessageText:
//
// Cannot initialize %1!ls! stash file.
//
#define IMAPI_E_STASHFILE_OPEN_FAILURE   ((HRESULT)0xC0AAB138L)

//
// MessageId: IMAPI_E_STASHFILE_SEEK_FAILURE
//
// MessageText:
//
// Error seeking in '%1!ls!' stash file.
//
#define IMAPI_E_STASHFILE_SEEK_FAILURE   ((HRESULT)0xC0AAB139L)

//
// MessageId: IMAPI_E_STASHFILE_WRITE_FAILURE
//
// MessageText:
//
// Error encountered writing to '%1!ls!' stash file.
//
#define IMAPI_E_STASHFILE_WRITE_FAILURE  ((HRESULT)0xC0AAB13AL)

//
// MessageId: IMAPI_E_STASHFILE_READ_FAILURE
//
// MessageText:
//
// Error encountered reading from '%1!ls!' stash file.
//
#define IMAPI_E_STASHFILE_READ_FAILURE   ((HRESULT)0xC0AAB13BL)

// ----------------  Errors associated with attempt to set working directory  ------------------
//
// MessageId: IMAPI_E_INVALID_WORKING_DIRECTORY
//
// MessageText:
//
// The working directory '%1!ls!' is not valid.
//
#define IMAPI_E_INVALID_WORKING_DIRECTORY ((HRESULT)0xC0AAB140L)

//
// MessageId: IMAPI_E_WORKING_DIRECTORY_SPACE
//
// MessageText:
//
// Cannot set working directory to '%1!ls!'.  Space available is %2!I64d! bytes, approximately %3!I64d! bytes required. 
//
#define IMAPI_E_WORKING_DIRECTORY_SPACE  ((HRESULT)0xC0AAB141L)

//
// MessageId: IMAPI_E_STASHFILE_MOVE
//
// MessageText:
//
// Attempt to move the data stash file to directory '%1!ls!' was not successful.
//
#define IMAPI_E_STASHFILE_MOVE           ((HRESULT)0xC0AAB142L)

// ----------------  Errors associated with Boot Image object   ------------------
//
// MessageId: IMAPI_E_BOOT_IMAGE_DATA
//
// MessageText:
//
// The boot object could not be added to the image.
//
#define IMAPI_E_BOOT_IMAGE_DATA          ((HRESULT)0xC0AAB148L)

//
// MessageId: IMAPI_E_BOOT_OBJECT_CONFLICT
//
// MessageText:
//
// A boot object can only be included in an initial disc image.
//
#define IMAPI_E_BOOT_OBJECT_CONFLICT     ((HRESULT)0xC0AAB149L)

//
// MessageId: IMAPI_E_BOOT_EMULATION_IMAGE_SIZE_MISMATCH
//
// MessageText:
//
// The emulation type requested does not match the boot image size.
//
#define IMAPI_E_BOOT_EMULATION_IMAGE_SIZE_MISMATCH ((HRESULT)0xC0AAB14AL)

// ----------------  Errors associated with import operation   ------------------
//
// MessageId: IMAPI_E_EMPTY_DISC
//
// MessageText:
//
// Optical media is empty.
//
#define IMAPI_E_EMPTY_DISC               ((HRESULT)0xC0AAB150L)

//
// MessageId: IMAPI_E_NO_SUPPORTED_FILE_SYSTEM
//
// MessageText:
//
// The specified disc does not contain one of the supported file systems.
//
#define IMAPI_E_NO_SUPPORTED_FILE_SYSTEM ((HRESULT)0xC0AAB151L)

//
// MessageId: IMAPI_E_FILE_SYSTEM_NOT_FOUND
//
// MessageText:
//
// The specified disc does not contain a '%1!ls!' file system.
//
#define IMAPI_E_FILE_SYSTEM_NOT_FOUND    ((HRESULT)0xC0AAB152L)

//
// MessageId: IMAPI_E_FILE_SYSTEM_READ_CONSISTENCY_ERROR
//
// MessageText:
//
// Consistency error encountered while importing the '%1!ls!' file system.
//
#define IMAPI_E_FILE_SYSTEM_READ_CONSISTENCY_ERROR ((HRESULT)0xC0AAB153L)

//
// MessageId: IMAPI_E_FILE_SYSTEM_FEATURE_NOT_SUPPORTED
//
// MessageText:
//
// The '%1!ls!'file system on the selected disc contains a feature not supported for import.
//
#define IMAPI_E_FILE_SYSTEM_FEATURE_NOT_SUPPORTED ((HRESULT)0xC0AAB154L)

//
// MessageId: IMAPI_E_IMPORT_TYPE_COLLISION_FILE_EXISTS_AS_DIRECTORY
//
// MessageText:
//
// Could not import %2!ls! file system from disc.  The file '%1!ls!' already exists within the image hierarchy as a directory.
//
#define IMAPI_E_IMPORT_TYPE_COLLISION_FILE_EXISTS_AS_DIRECTORY ((HRESULT)0xC0AAB155L)

//
// MessageId: IMAPI_E_IMPORT_SEEK_FAILURE
//
// MessageText:
//
// Cannot seek to block %1!I64d! on source disc. 
//
#define IMAPI_E_IMPORT_SEEK_FAILURE      ((HRESULT)0xC0AAB156L)

//
// MessageId: IMAPI_E_IMPORT_READ_FAILURE
//
// MessageText:
//
// Import from previous session failed due to an error reading a block on the media (most likely block %1!u!).
//
#define IMAPI_E_IMPORT_READ_FAILURE      ((HRESULT)0xC0AAB157L)

//
// MessageId: IMAPI_E_DISC_MISMATCH
//
// MessageText:
//
// Current disc is not the same one from which file system was imported.
//
#define IMAPI_E_DISC_MISMATCH            ((HRESULT)0xC0AAB158L)

//
// MessageId: IMAPI_E_IMPORT_MEDIA_NOT_ALLOWED
//
// MessageText:
//
// IMAPI does not allow multi-session with the current media type.
//
#define IMAPI_E_IMPORT_MEDIA_NOT_ALLOWED ((HRESULT)0xC0AAB159L)

//
// MessageId: IMAPI_E_UDF_NOT_WRITE_COMPATIBLE
//
// MessageText:
//
// IMAPI can not do multi-session with the current media because it does not support a compatible UDF revision for write.
//
#define IMAPI_E_UDF_NOT_WRITE_COMPATIBLE ((HRESULT)0xC0AAB15AL)

//
// MessageId: IMAPI_E_INCOMPATIBLE_MULTISESSION_TYPE
//
// MessageText:
//
// IMAPI does not support the multisession type requested.
//
#define IMAPI_E_INCOMPATIBLE_MULTISESSION_TYPE ((HRESULT)0xC0AAB15BL)

//
// MessageId: IMAPI_E_NO_COMPATIBLE_MULTISESSION_TYPE
//
// MessageText:
//
// IMAPI supports none of the multisession type(s) provided on the current media.
//
#define IMAPI_E_NO_COMPATIBLE_MULTISESSION_TYPE ((HRESULT)0xC0AAB15CL)

//
// MessageId: IMAPI_E_MULTISESSION_NOT_SET
//
// MessageText:
//
// MultisessionInterfaces property must be set prior calling this method.
//
#define IMAPI_E_MULTISESSION_NOT_SET     ((HRESULT)0xC0AAB15DL)

//
// MessageId: IMAPI_E_IMPORT_TYPE_COLLISION_DIRECTORY_EXISTS_AS_FILE
//
// MessageText:
//
// Could not import %2!ls! file system from disc.  The directory '%1!ls!' already exists within the image hierarchy as a file.
//
#define IMAPI_E_IMPORT_TYPE_COLLISION_DIRECTORY_EXISTS_AS_FILE ((HRESULT)0xC0AAB15EL)

//
// MessageId: IMAPI_S_IMAGE_FEATURE_NOT_SUPPORTED
//
// MessageText:
//
// Feature is not supported for the current file system revision, image will be created without this feature.
//
#define IMAPI_S_IMAGE_FEATURE_NOT_SUPPORTED ((HRESULT)0x00AAB15FL)

//
// MessageId: IMAPI_E_PROPERTY_NOT_ACCESSIBLE
//
// MessageText:
//
// Property '%1!ls!' is not accessible
//
#define IMAPI_E_PROPERTY_NOT_ACCESSIBLE  ((HRESULT)0xC0AAB160L)

//
// MessageId: IMAPI_E_UDF_REVISION_CHANGE_NOT_ALLOWED
//
// MessageText:
//
// UDF revision cannot be changed because of the previously imported session
//
#define IMAPI_E_UDF_REVISION_CHANGE_NOT_ALLOWED ((HRESULT)0xC0AAB161L)

//
// MessageId: IMAPI_E_BAD_MULTISESSION_PARAMETER
//
// MessageText:
//
// One of the multisession parameters cannot be retrieved or has a wrong value.
//
#define IMAPI_E_BAD_MULTISESSION_PARAMETER ((HRESULT)0xC0AAB162L)

//
// MessageId: IMAPI_E_FILE_SYSTEM_CHANGE_NOT_ALLOWED
//
// MessageText:
//
// You cannot change the file system to be created, because the file system in the imported session and the one in the new session must match.
//
#define IMAPI_E_FILE_SYSTEM_CHANGE_NOT_ALLOWED ((HRESULT)0xC0AAB163L)

// ------ Error messages for IIsoImageManager (range: 0x80AAB200 - 0x80AAB2FF) ------
//
// MessageId: IMAPI_E_IMAGEMANAGER_IMAGE_NOT_ALIGNED
//
// MessageText:
//
// The image is not 2kb aligned. Only 2048 bytes aligned images are supported.
//
#define IMAPI_E_IMAGEMANAGER_IMAGE_NOT_ALIGNED ((HRESULT)0xC0AAB200L)

//
// MessageId: IMAPI_E_IMAGEMANAGER_NO_VALID_VD_FOUND
//
// MessageText:
//
// No valid file system Volume Descriptor was found in the iso image. This image format is not supported and the resulting disc might not be readable.
//
#define IMAPI_E_IMAGEMANAGER_NO_VALID_VD_FOUND ((HRESULT)0xC0AAB201L)

//
// MessageId: IMAPI_E_IMAGEMANAGER_NO_IMAGE
//
// MessageText:
//
// No image was set (neither path nor stream was given).
//
#define IMAPI_E_IMAGEMANAGER_NO_IMAGE    ((HRESULT)0xC0AAB202L)

//
// MessageId: IMAPI_E_IMAGEMANAGER_IMAGE_TOO_BIG
//
// MessageText:
//
// Image size exceeds MAXLONG sectors - too big.
//
#define IMAPI_E_IMAGEMANAGER_IMAGE_TOO_BIG ((HRESULT)0xC0AAB203L)

// ----------- Empty message, should not occur, used for internal purposes ------------------
//
// MessageId: 0x0000FFFFL (No symbolic name defined)
//
// MessageText:
//
// (empty message)
//


#endif		// #ifndef _SKIP_FSI_ERROR_MESSAGE_CODES
#define IMAPI_ERROR_FIRST	(unsigned int)0xB100
#define IMAPI_ERROR_LAST	(unsigned int)0xB2FF
#define IMAPIError(hr) ((HRESULT_FACILITY(hr) == FACILITY_IMAPI2) && (HRESULT_CODE(hr) <= IMAPI_ERROR_LAST) && (HRESULT_CODE(hr) >= IMAPI_ERROR_FIRST))
#define IMAPI_FSI_ERROR_FIRST	(unsigned int)0xB100
#define IMAPI_FSI_ERROR_LAST	(unsigned int)0xB2FF
#define IMAPIFsiError(hr) ((HRESULT_FACILITY(hr) == FACILITY_IMAPI2) && (HRESULT_CODE(hr) <= IMAPI_FSI_ERROR_LAST) && (HRESULT_CODE(hr) >= IMAPI_FSI_ERROR_FIRST))
