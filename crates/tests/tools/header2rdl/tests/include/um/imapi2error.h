/*++ BUILD Version: 0001    // Increment this if a change has global effects
Copyright (c) Microsoft Corporation. All rights reserved.
--*/

#ifndef _IMAPI2ERROR_
#define _IMAPI2ERROR_


//
// Error Messages used throughout IMAPIv2
// Range: 0x80AA0000 - 0x80AA00FF
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
// MessageId: E_IMAPI_REQUEST_CANCELLED
//
// MessageText:
//
// The request was cancelled.
//
#define E_IMAPI_REQUEST_CANCELLED        ((HRESULT)0xC0AA0002L)

//
// MessageId: E_IMAPI_RECORDER_REQUIRED
//
// MessageText:
//
// The request requires a current disc recorder to be selected.
//
#define E_IMAPI_RECORDER_REQUIRED        ((HRESULT)0xC0AA0003L)

//
// MessageId: S_IMAPI_SPEEDADJUSTED
//
// MessageText:
//
// The requested write speed was not supported by the drive and the speed was adjusted.
//
#define S_IMAPI_SPEEDADJUSTED            ((HRESULT)0x00AA0004L)

//
// MessageId: S_IMAPI_ROTATIONADJUSTED
//
// MessageText:
//
// The requested rotation type was not supported by the drive and the rotation type was adjusted.
//
#define S_IMAPI_ROTATIONADJUSTED         ((HRESULT)0x00AA0005L)

//
// MessageId: S_IMAPI_BOTHADJUSTED
//
// MessageText:
//
// The requested write speed and rotation type were not supported by the drive and they were both adjusted.
//
#define S_IMAPI_BOTHADJUSTED             ((HRESULT)0x00AA0006L)

//
// MessageId: E_IMAPI_BURN_VERIFICATION_FAILED
//
// MessageText:
//
// The disc did not pass burn verification and may contain corrupt data or be unusable. 
//
#define E_IMAPI_BURN_VERIFICATION_FAILED ((HRESULT)0xC0AA0007L)


//
// Error Messages for IDiscMaster2
// Range: 0x80AA0100 - 0x80AA01FF
//


//
// Error Messages for IDiscRecorder2
// Range: 0x80AA0200 - 0x80AA02FF
//

//
// MessageId: S_IMAPI_COMMAND_HAS_SENSE_DATA
//
// MessageText:
//
// The device accepted the command, but returned sense data, indicating an error.
//
#define S_IMAPI_COMMAND_HAS_SENSE_DATA   ((HRESULT)0x00AA0200L)

//
// MessageId: E_IMAPI_RECORDER_NO_SUCH_MODE_PAGE
//
// MessageText:
//
// The device reported that the requested mode page (and type) is not present.
//
#define E_IMAPI_RECORDER_NO_SUCH_MODE_PAGE ((HRESULT)0xC0AA0201L)

//
// MessageId: E_IMAPI_RECORDER_MEDIA_NO_MEDIA
//
// MessageText:
//
// There is no media in the device.
//
#define E_IMAPI_RECORDER_MEDIA_NO_MEDIA  ((HRESULT)0xC0AA0202L)

//
// MessageId: E_IMAPI_RECORDER_MEDIA_INCOMPATIBLE
//
// MessageText:
//
// The media is not compatible or of unknown physical format.
//
#define E_IMAPI_RECORDER_MEDIA_INCOMPATIBLE ((HRESULT)0xC0AA0203L)

//
// MessageId: E_IMAPI_RECORDER_MEDIA_UPSIDE_DOWN
//
// MessageText:
//
// The media is inserted upside down.
//
#define E_IMAPI_RECORDER_MEDIA_UPSIDE_DOWN ((HRESULT)0xC0AA0204L)

//
// MessageId: E_IMAPI_RECORDER_MEDIA_BECOMING_READY
//
// MessageText:
//
// The drive reported that it is in the process of becoming ready.  Please try the request again later.
//
#define E_IMAPI_RECORDER_MEDIA_BECOMING_READY ((HRESULT)0xC0AA0205L)

//
// MessageId: E_IMAPI_RECORDER_MEDIA_FORMAT_IN_PROGRESS
//
// MessageText:
//
// The media is currently being formatted.  Please wait for the format to complete before attempting to use the media.
//
#define E_IMAPI_RECORDER_MEDIA_FORMAT_IN_PROGRESS ((HRESULT)0xC0AA0206L)

//
// MessageId: E_IMAPI_RECORDER_MEDIA_BUSY
//
// MessageText:
//
// The drive reported that it is performing a long-running operation, such as finishing a write.  The drive may be unusable for a long period of time.
//
#define E_IMAPI_RECORDER_MEDIA_BUSY      ((HRESULT)0xC0AA0207L)

//
// MessageId: E_IMAPI_RECORDER_INVALID_MODE_PARAMETERS
//
// MessageText:
//
// The drive reported that the combination of parameters provided in the mode page for a MODE SELECT command were not supported.
//
#define E_IMAPI_RECORDER_INVALID_MODE_PARAMETERS ((HRESULT)0xC0AA0208L)

//
// MessageId: E_IMAPI_RECORDER_MEDIA_WRITE_PROTECTED
//
// MessageText:
//
// The drive reported that the media is write protected.
//
#define E_IMAPI_RECORDER_MEDIA_WRITE_PROTECTED ((HRESULT)0xC0AA0209L)

//
// MessageId: E_IMAPI_RECORDER_NO_SUCH_FEATURE
//
// MessageText:
//
// The feature page requested is not supported by the device.
//
#define E_IMAPI_RECORDER_NO_SUCH_FEATURE ((HRESULT)0xC0AA020AL)

//
// MessageId: E_IMAPI_RECORDER_FEATURE_IS_NOT_CURRENT
//
// MessageText:
//
// The feature page requested is supported, but is not marked as current.
//
#define E_IMAPI_RECORDER_FEATURE_IS_NOT_CURRENT ((HRESULT)0xC0AA020BL)

//
// MessageId: E_IMAPI_RECORDER_GET_CONFIGURATION_NOT_SUPPORTED
//
// MessageText:
//
// The drive does not support the GET CONFIGURATION command.
//
#define E_IMAPI_RECORDER_GET_CONFIGURATION_NOT_SUPPORTED ((HRESULT)0xC0AA020CL)

//
// MessageId: E_IMAPI_RECORDER_COMMAND_TIMEOUT
//
// MessageText:
//
// The device failed to accept the command within the timeout period. This may be caused by the device having entered an inconsistent state, or the timeout value for the command may need to be increased.
//
#define E_IMAPI_RECORDER_COMMAND_TIMEOUT ((HRESULT)0xC0AA020DL)

//
// MessageId: E_IMAPI_RECORDER_DVD_STRUCTURE_NOT_PRESENT
//
// MessageText:
//
// The DVD structure is not present. This may be caused by incompatible drive/medium used.
//
#define E_IMAPI_RECORDER_DVD_STRUCTURE_NOT_PRESENT ((HRESULT)0xC0AA020EL)

//
// MessageId: E_IMAPI_RECORDER_MEDIA_SPEED_MISMATCH
//
// MessageText:
//
// The media's speed is incompatible with the device.  This may be caused by using higher or lower speed media than the range of speeds supported by the device.
//
#define E_IMAPI_RECORDER_MEDIA_SPEED_MISMATCH ((HRESULT)0xC0AA020FL)

//
// MessageId: E_IMAPI_RECORDER_LOCKED
//
// MessageText:
//
// The device associated with this recorder during the last operation has been exclusively locked, causing this operation to failed.
//
#define E_IMAPI_RECORDER_LOCKED          ((HRESULT)0xC0AA0210L)

//
// MessageId: E_IMAPI_RECORDER_CLIENT_NAME_IS_NOT_VALID
//
// MessageText:
//
// The client name is not valid.
//
#define E_IMAPI_RECORDER_CLIENT_NAME_IS_NOT_VALID ((HRESULT)0xC0AA0211L)

//
// MessageId: E_IMAPI_RECORDER_MEDIA_NOT_FORMATTED
//
// MessageText:
//
// The media is not formatted. Please format the media before attempting to use it.
//
#define E_IMAPI_RECORDER_MEDIA_NOT_FORMATTED ((HRESULT)0xC0AA0212L)

//
// MessageId: E_IMAPI_RECORDER_INVALID_RESPONSE_FROM_DEVICE
//
// MessageText:
//
// The device reported unexpected or invalid data for a command.
//
#define E_IMAPI_RECORDER_INVALID_RESPONSE_FROM_DEVICE ((HRESULT)0xC0AA02FFL)


//
// Error Messages for IWriteEngine2
// Range: 0x80AA0300 - 0x80AA03FF
//

//
// MessageId: E_IMAPI_LOSS_OF_STREAMING
//
// MessageText:
//
// The write failed because the drive did not receive data quickly enough to continue writing. Moving the source data to the local computer, reducing the write speed, or enabling a "buffer underrun free" setting may resolve this issue.
//
#define E_IMAPI_LOSS_OF_STREAMING        ((HRESULT)0xC0AA0300L)

//
// MessageId: E_IMAPI_UNEXPECTED_RESPONSE_FROM_DEVICE
//
// MessageText:
//
// The write failed because the drive returned error information that could not be recovered from.
//
#define E_IMAPI_UNEXPECTED_RESPONSE_FROM_DEVICE ((HRESULT)0xC0AA0301L)

//
// MessageId: S_IMAPI_WRITE_NOT_IN_PROGRESS
//
// MessageText:
//
// There is no write operation currently in progress.
//
#define S_IMAPI_WRITE_NOT_IN_PROGRESS    ((HRESULT)0x00AA0302L)


//
// Error Messages for IDiscFormat2Data
// Range: 0x80AA0400 - 0x80AA04FF
//

//
// MessageId: E_IMAPI_DF2DATA_WRITE_IN_PROGRESS
//
// MessageText:
//
// There is currently a write operation in progress.
//
#define E_IMAPI_DF2DATA_WRITE_IN_PROGRESS ((HRESULT)0xC0AA0400L)

//
// MessageId: E_IMAPI_DF2DATA_WRITE_NOT_IN_PROGRESS
//
// MessageText:
//
// There is no write operation currently in progress.
//
#define E_IMAPI_DF2DATA_WRITE_NOT_IN_PROGRESS ((HRESULT)0xC0AA0401L)

//
// MessageId: E_IMAPI_DF2DATA_INVALID_MEDIA_STATE
//
// MessageText:
//
// The requested operation is only valid with supported media.
//
#define E_IMAPI_DF2DATA_INVALID_MEDIA_STATE ((HRESULT)0xC0AA0402L)

//
// MessageId: E_IMAPI_DF2DATA_STREAM_NOT_SUPPORTED
//
// MessageText:
//
// The provided stream to write is not supported.
//
#define E_IMAPI_DF2DATA_STREAM_NOT_SUPPORTED ((HRESULT)0xC0AA0403L)

//
// MessageId: E_IMAPI_DF2DATA_STREAM_TOO_LARGE_FOR_CURRENT_MEDIA
//
// MessageText:
//
// The provided stream to write is too large for the currently inserted media.
//
#define E_IMAPI_DF2DATA_STREAM_TOO_LARGE_FOR_CURRENT_MEDIA ((HRESULT)0xC0AA0404L)

//
// MessageId: E_IMAPI_DF2DATA_MEDIA_NOT_BLANK
//
// MessageText:
//
// Overwriting non-blank media is not allowed without the ForceOverwrite property set to VARIANT_TRUE.
//
#define E_IMAPI_DF2DATA_MEDIA_NOT_BLANK  ((HRESULT)0xC0AA0405L)

//
// MessageId: E_IMAPI_DF2DATA_MEDIA_IS_NOT_SUPPORTED
//
// MessageText:
//
// The current media type is unsupported.
//
#define E_IMAPI_DF2DATA_MEDIA_IS_NOT_SUPPORTED ((HRESULT)0xC0AA0406L)

//
// MessageId: E_IMAPI_DF2DATA_RECORDER_NOT_SUPPORTED
//
// MessageText:
//
// This device does not support the operations required by this disc format.
//
#define E_IMAPI_DF2DATA_RECORDER_NOT_SUPPORTED ((HRESULT)0xC0AA0407L)

//
// MessageId: E_IMAPI_DF2DATA_CLIENT_NAME_IS_NOT_VALID
//
// MessageText:
//
// The client name is not valid.
//
#define E_IMAPI_DF2DATA_CLIENT_NAME_IS_NOT_VALID ((HRESULT)0xC0AA0408L)


//
// Error Messages for IDiscFormat2TrackAtOnce
// Range: 0x80AA0500 - 0x80AA05FF
//

//
// MessageId: E_IMAPI_DF2TAO_WRITE_IN_PROGRESS
//
// MessageText:
//
// There is currently a write operation in progress.
//
#define E_IMAPI_DF2TAO_WRITE_IN_PROGRESS ((HRESULT)0xC0AA0500L)

//
// MessageId: E_IMAPI_DF2TAO_WRITE_NOT_IN_PROGRESS
//
// MessageText:
//
// There is no write operation currently in progress.
//
#define E_IMAPI_DF2TAO_WRITE_NOT_IN_PROGRESS ((HRESULT)0xC0AA0501L)

//
// MessageId: E_IMAPI_DF2TAO_MEDIA_IS_NOT_PREPARED
//
// MessageText:
//
// The requested operation is only valid when media has been "prepared".
//
#define E_IMAPI_DF2TAO_MEDIA_IS_NOT_PREPARED ((HRESULT)0xC0AA0502L)

//
// MessageId: E_IMAPI_DF2TAO_MEDIA_IS_PREPARED
//
// MessageText:
//
// The requested operation is not valid when media has been "prepared" but not released.
//
#define E_IMAPI_DF2TAO_MEDIA_IS_PREPARED ((HRESULT)0xC0AA0503L)

//
// MessageId: E_IMAPI_DF2TAO_PROPERTY_FOR_BLANK_MEDIA_ONLY
//
// MessageText:
//
// The property cannot be changed once the media has been written to.
//
#define E_IMAPI_DF2TAO_PROPERTY_FOR_BLANK_MEDIA_ONLY ((HRESULT)0xC0AA0504L)

//
// MessageId: E_IMAPI_DF2TAO_TABLE_OF_CONTENTS_EMPTY_DISC
//
// MessageText:
//
// The table of contents cannot be retrieved from an empty disc.
//
#define E_IMAPI_DF2TAO_TABLE_OF_CONTENTS_EMPTY_DISC ((HRESULT)0xC0AA0505L)

//
// MessageId: E_IMAPI_DF2TAO_MEDIA_IS_NOT_BLANK
//
// MessageText:
//
// Only blank CD-R/RW media is supported.
//
#define E_IMAPI_DF2TAO_MEDIA_IS_NOT_BLANK ((HRESULT)0xC0AA0506L)

//
// MessageId: E_IMAPI_DF2TAO_MEDIA_IS_NOT_SUPPORTED
//
// MessageText:
//
// Only blank CD-R/RW media is supported.
//
#define E_IMAPI_DF2TAO_MEDIA_IS_NOT_SUPPORTED ((HRESULT)0xC0AA0507L)

//
// MessageId: E_IMAPI_DF2TAO_TRACK_LIMIT_REACHED
//
// MessageText:
//
// CD-R and CD-RW media support a maximum of 99 audio tracks.
//
#define E_IMAPI_DF2TAO_TRACK_LIMIT_REACHED ((HRESULT)0xC0AA0508L)

//
// MessageId: E_IMAPI_DF2TAO_NOT_ENOUGH_SPACE
//
// MessageText:
//
// There is not enough space left on the media to add the provided audio track.
//
#define E_IMAPI_DF2TAO_NOT_ENOUGH_SPACE  ((HRESULT)0xC0AA0509L)

//
// MessageId: E_IMAPI_DF2TAO_NO_RECORDER_SPECIFIED
//
// MessageText:
//
// You cannot prepare the media until you choose a recorder to use.
//
#define E_IMAPI_DF2TAO_NO_RECORDER_SPECIFIED ((HRESULT)0xC0AA050AL)

//
// MessageId: E_IMAPI_DF2TAO_INVALID_ISRC
//
// MessageText:
//
// The ISRC provided is not valid.
//
#define E_IMAPI_DF2TAO_INVALID_ISRC      ((HRESULT)0xC0AA050BL)

//
// MessageId: E_IMAPI_DF2TAO_INVALID_MCN
//
// MessageText:
//
// The Media Catalog Number provided is not valid.
//
#define E_IMAPI_DF2TAO_INVALID_MCN       ((HRESULT)0xC0AA050CL)

//
// MessageId: E_IMAPI_DF2TAO_STREAM_NOT_SUPPORTED
//
// MessageText:
//
// The provided audio stream is not valid.
//
#define E_IMAPI_DF2TAO_STREAM_NOT_SUPPORTED ((HRESULT)0xC0AA050DL)

//
// MessageId: E_IMAPI_DF2TAO_RECORDER_NOT_SUPPORTED
//
// MessageText:
//
// This device does not support the operations required by this disc format.
//
#define E_IMAPI_DF2TAO_RECORDER_NOT_SUPPORTED ((HRESULT)0xC0AA050EL)

//
// MessageId: E_IMAPI_DF2TAO_CLIENT_NAME_IS_NOT_VALID
//
// MessageText:
//
// The client name is not valid.
//
#define E_IMAPI_DF2TAO_CLIENT_NAME_IS_NOT_VALID ((HRESULT)0xC0AA050FL)


//
// Error Messages for IDiscFormat2RawCD
// Range: 0x80AA0600 - 0x80AA06FF
//
// 0x0605, 0x0608, 0x060B, 0x060C are not used

//
// MessageId: E_IMAPI_DF2RAW_WRITE_IN_PROGRESS
//
// MessageText:
//
// There is currently a write operation in progress.
//
#define E_IMAPI_DF2RAW_WRITE_IN_PROGRESS ((HRESULT)0xC0AA0600L)

//
// MessageId: E_IMAPI_DF2RAW_WRITE_NOT_IN_PROGRESS
//
// MessageText:
//
// There is no write operation currently in progress.
//
#define E_IMAPI_DF2RAW_WRITE_NOT_IN_PROGRESS ((HRESULT)0xC0AA0601L)

//
// MessageId: E_IMAPI_DF2RAW_MEDIA_IS_NOT_PREPARED
//
// MessageText:
//
// The requested operation is only valid when media has been "prepared".
//
#define E_IMAPI_DF2RAW_MEDIA_IS_NOT_PREPARED ((HRESULT)0xC0AA0602L)

//
// MessageId: E_IMAPI_DF2RAW_MEDIA_IS_PREPARED
//
// MessageText:
//
// The requested operation is not valid when media has been "prepared" but not released.
//
#define E_IMAPI_DF2RAW_MEDIA_IS_PREPARED ((HRESULT)0xC0AA0603L)

//
// MessageId: E_IMAPI_DF2RAW_CLIENT_NAME_IS_NOT_VALID
//
// MessageText:
//
// The client name is not valid.
//
#define E_IMAPI_DF2RAW_CLIENT_NAME_IS_NOT_VALID ((HRESULT)0xC0AA0604L)

//
// MessageId: E_IMAPI_DF2RAW_MEDIA_IS_NOT_BLANK
//
// MessageText:
//
// Only blank CD-R/RW media is supported.
//
#define E_IMAPI_DF2RAW_MEDIA_IS_NOT_BLANK ((HRESULT)0xC0AA0606L)

//
// MessageId: E_IMAPI_DF2RAW_MEDIA_IS_NOT_SUPPORTED
//
// MessageText:
//
// Only blank CD-R/RW media is supported.
//
#define E_IMAPI_DF2RAW_MEDIA_IS_NOT_SUPPORTED ((HRESULT)0xC0AA0607L)

//
// MessageId: E_IMAPI_DF2RAW_NOT_ENOUGH_SPACE
//
// MessageText:
//
// There is not enough space on the media to add the provided session.
//
#define E_IMAPI_DF2RAW_NOT_ENOUGH_SPACE  ((HRESULT)0xC0AA0609L)

//
// MessageId: E_IMAPI_DF2RAW_NO_RECORDER_SPECIFIED
//
// MessageText:
//
// You cannot prepare the media until you choose a recorder to use.
//
#define E_IMAPI_DF2RAW_NO_RECORDER_SPECIFIED ((HRESULT)0xC0AA060AL)

//
// MessageId: E_IMAPI_DF2RAW_STREAM_NOT_SUPPORTED
//
// MessageText:
//
// The provided audio stream is not valid.
//
#define E_IMAPI_DF2RAW_STREAM_NOT_SUPPORTED ((HRESULT)0xC0AA060DL)

//
// MessageId: E_IMAPI_DF2RAW_DATA_BLOCK_TYPE_NOT_SUPPORTED
//
// MessageText:
//
// The requested data block type is not supported by the current device.
//
#define E_IMAPI_DF2RAW_DATA_BLOCK_TYPE_NOT_SUPPORTED ((HRESULT)0xC0AA060EL)

//
// MessageId: E_IMAPI_DF2RAW_STREAM_LEADIN_TOO_SHORT
//
// MessageText:
//
// The stream does not contain a sufficient number of sectors in the leadin for the current media.
//
#define E_IMAPI_DF2RAW_STREAM_LEADIN_TOO_SHORT ((HRESULT)0xC0AA060FL)

//
// MessageId: E_IMAPI_DF2RAW_RECORDER_NOT_SUPPORTED
//
// MessageText:
//
// This device does not support the operations required by this disc format.
//
#define E_IMAPI_DF2RAW_RECORDER_NOT_SUPPORTED ((HRESULT)0xC0AA0610L)


//
// Error Messages for IDiscFormat2Erase
// Range: 0x80AA0900 - 0x80AA09FF
//

//
// MessageId: E_IMAPI_ERASE_RECORDER_IN_USE
//
// MessageText:
//
// The format is currently using the disc recorder for an erase operation.
// Please wait for the erase to complete before attempting to set or clear the
// current disc recorder.
//
#define E_IMAPI_ERASE_RECORDER_IN_USE    ((HRESULT)0x80AA0900L)

//
// MessageId: E_IMAPI_ERASE_ONLY_ONE_RECORDER_SUPPORTED
//
// MessageText:
//
// The erase format only supports one recorder.  You must clear the current
// recorder before setting a new one.
//
#define E_IMAPI_ERASE_ONLY_ONE_RECORDER_SUPPORTED ((HRESULT)0x80AA0901L)

//
// MessageId: E_IMAPI_ERASE_DISC_INFORMATION_TOO_SMALL
//
// MessageText:
//
// The drive did not report sufficient data for a READ DISC INFORMATION command.
// The drive may not be supported, or the media may not be correct.
//
#define E_IMAPI_ERASE_DISC_INFORMATION_TOO_SMALL ((HRESULT)0x80AA0902L)

//
// MessageId: E_IMAPI_ERASE_MODE_PAGE_2A_TOO_SMALL
//
// MessageText:
//
// The drive did not report sufficient data for a MODE SENSE (page 0x2A) command.
// The drive may not be supported, or the media may not be correct.
//
#define E_IMAPI_ERASE_MODE_PAGE_2A_TOO_SMALL ((HRESULT)0x80AA0903L)

//
// MessageId: E_IMAPI_ERASE_MEDIA_IS_NOT_ERASABLE
//
// MessageText:
//
// The drive reported that the media is not erasable.
//
#define E_IMAPI_ERASE_MEDIA_IS_NOT_ERASABLE ((HRESULT)0x80AA0904L)

//
// MessageId: E_IMAPI_ERASE_DRIVE_FAILED_ERASE_COMMAND
//
// MessageText:
//
// The drive failed the erase command.
//
#define E_IMAPI_ERASE_DRIVE_FAILED_ERASE_COMMAND ((HRESULT)0x80AA0905L)

//
// MessageId: E_IMAPI_ERASE_TOOK_LONGER_THAN_ONE_HOUR
//
// MessageText:
//
// The drive did not complete the erase in one hour.  The drive may require a power cycle, media removal, or other manual intervention to resume proper operation.
//
#define E_IMAPI_ERASE_TOOK_LONGER_THAN_ONE_HOUR ((HRESULT)0x80AA0906L)

//
// MessageId: E_IMAPI_ERASE_UNEXPECTED_DRIVE_RESPONSE_DURING_ERASE
//
// MessageText:
//
// The drive returned an unexpected error during the erase.  The the media may be
// unusable, the erase may be complete, or the drive may still be in the process
// of erasing the disc.
//
#define E_IMAPI_ERASE_UNEXPECTED_DRIVE_RESPONSE_DURING_ERASE ((HRESULT)0x80AA0907L)

//
// MessageId: E_IMAPI_ERASE_DRIVE_FAILED_SPINUP_COMMAND
//
// MessageText:
//
// The drive returned an error for a START UNIT (spinup) command.  Manual intervention may be required.
//
#define E_IMAPI_ERASE_DRIVE_FAILED_SPINUP_COMMAND ((HRESULT)0x80AA0908L)

//
// MessageId: E_IMAPI_ERASE_MEDIA_IS_NOT_SUPPORTED
//
// MessageText:
//
// The current media type is unsupported.
//
#define E_IMAPI_ERASE_MEDIA_IS_NOT_SUPPORTED ((HRESULT)0xC0AA0909L)

//
// MessageId: E_IMAPI_ERASE_RECORDER_NOT_SUPPORTED
//
// MessageText:
//
// This device does not support the operations required by this disc format.
//
#define E_IMAPI_ERASE_RECORDER_NOT_SUPPORTED ((HRESULT)0xC0AA090AL)

//
// MessageId: E_IMAPI_ERASE_CLIENT_NAME_IS_NOT_VALID
//
// MessageText:
//
// The client name is not valid.
//
#define E_IMAPI_ERASE_CLIENT_NAME_IS_NOT_VALID ((HRESULT)0xC0AA090BL)


//
// Error Messages for IRawCDImageCreator
// Range: 0x80AA0A00 - 0x80AA0AFF
//

//
// MessageId: E_IMAPI_RAW_IMAGE_IS_READ_ONLY
//
// MessageText:
//
// The image has become read-only from a call to CreateResultImage().  
// The object can no longer be modified.
//
#define E_IMAPI_RAW_IMAGE_IS_READ_ONLY   ((HRESULT)0x80AA0A00L)

//
// MessageId: E_IMAPI_RAW_IMAGE_TOO_MANY_TRACKS
//
// MessageText:
//
// No more tracks may be added, as CD media is restricted to track numbers 
// between 1 and 99.
//
#define E_IMAPI_RAW_IMAGE_TOO_MANY_TRACKS ((HRESULT)0x80AA0A01L)

//
// MessageId: E_IMAPI_RAW_IMAGE_SECTOR_TYPE_NOT_SUPPORTED
//
// MessageText:
//
// The requested sector type is not supported.
//
#define E_IMAPI_RAW_IMAGE_SECTOR_TYPE_NOT_SUPPORTED ((HRESULT)0x80AA0A02L)

//
// MessageId: E_IMAPI_RAW_IMAGE_NO_TRACKS
//
// MessageText:
//
// Tracks must be added to the image before using this function.
//
#define E_IMAPI_RAW_IMAGE_NO_TRACKS      ((HRESULT)0x80AA0A03L)

//
// MessageId: E_IMAPI_RAW_IMAGE_TRACKS_ALREADY_ADDED
//
// MessageText:
//
// Tracks may not be added to the image prior to the use of this function.
//
#define E_IMAPI_RAW_IMAGE_TRACKS_ALREADY_ADDED ((HRESULT)0x80AA0A04L)

//
// MessageId: E_IMAPI_RAW_IMAGE_INSUFFICIENT_SPACE
//
// MessageText:
//
// Adding the track would result in exceeding the limit for the start of the leadout.
//
#define E_IMAPI_RAW_IMAGE_INSUFFICIENT_SPACE ((HRESULT)0x80AA0A05L)

//
// MessageId: E_IMAPI_RAW_IMAGE_TOO_MANY_TRACK_INDEXES
//
// MessageText:
//
// Adding the track index would result in exceeding the 99 index limit.
//
#define E_IMAPI_RAW_IMAGE_TOO_MANY_TRACK_INDEXES ((HRESULT)0x80AA0A06L)

//
// MessageId: E_IMAPI_RAW_IMAGE_TRACK_INDEX_NOT_FOUND
//
// MessageText:
//
// The specified LBA offset is not in the list of track indexes.
//
#define E_IMAPI_RAW_IMAGE_TRACK_INDEX_NOT_FOUND ((HRESULT)0x80AA0A07L)

//
// MessageId: S_IMAPI_RAW_IMAGE_TRACK_INDEX_ALREADY_EXISTS
//
// MessageText:
//
// The specified LBA offset is already in the list of track indexes.
//
#define S_IMAPI_RAW_IMAGE_TRACK_INDEX_ALREADY_EXISTS ((HRESULT)0x00AA0A08L)

//
// MessageId: E_IMAPI_RAW_IMAGE_TRACK_INDEX_OFFSET_ZERO_CANNOT_BE_CLEARED
//
// MessageText:
//
// Index 1 (LBA offset zero) may not be cleared.
//
#define E_IMAPI_RAW_IMAGE_TRACK_INDEX_OFFSET_ZERO_CANNOT_BE_CLEARED ((HRESULT)0x80AA0A09L)

//
// MessageId: E_IMAPI_RAW_IMAGE_TRACK_INDEX_TOO_CLOSE_TO_OTHER_INDEX
//
// MessageText:
//
// Each index must have a minimum size of ten sectors.
//
#define E_IMAPI_RAW_IMAGE_TRACK_INDEX_TOO_CLOSE_TO_OTHER_INDEX ((HRESULT)0x80AA0A0AL)

#endif /* _IMAPI2ERROR_ */
