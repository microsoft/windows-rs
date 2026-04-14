/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ntddcdrm.h

Abstract:

    This module contains structures and definitions
    associated with CDROM IOCTls.


--*/

// begin_winioctl

#ifndef _NTDDCDRM_
#define _NTDDCDRM_


#ifdef __cplusplus
extern "C" {
#endif


#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


//
// remove some level 4 warnings for this header file:
#pragma warning(disable:4200) // array[0]
#pragma warning(disable:4201) // nameless struct/unions
#pragma warning(disable:4214) // bit fields other than int

#ifdef __cplusplus
extern "C" {
#endif

//
// NtDeviceIoControlFile IoControlCode values for this device.
//
// Warning:  Remember that the low two bits of the code specify how the
//           buffers are passed to the driver!
//

#define IOCTL_CDROM_BASE                 FILE_DEVICE_CD_ROM

#define IOCTL_CDROM_UNLOAD_DRIVER        CTL_CODE(IOCTL_CDROM_BASE, 0x0402, METHOD_BUFFERED, FILE_READ_ACCESS)

//
// CDROM Audio Device Control Functions
//

#define IOCTL_CDROM_READ_TOC              CTL_CODE(IOCTL_CDROM_BASE, 0x0000, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_SEEK_AUDIO_MSF        CTL_CODE(IOCTL_CDROM_BASE, 0x0001, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_STOP_AUDIO            CTL_CODE(IOCTL_CDROM_BASE, 0x0002, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_PAUSE_AUDIO           CTL_CODE(IOCTL_CDROM_BASE, 0x0003, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_RESUME_AUDIO          CTL_CODE(IOCTL_CDROM_BASE, 0x0004, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_GET_VOLUME            CTL_CODE(IOCTL_CDROM_BASE, 0x0005, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_PLAY_AUDIO_MSF        CTL_CODE(IOCTL_CDROM_BASE, 0x0006, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_SET_VOLUME            CTL_CODE(IOCTL_CDROM_BASE, 0x000A, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_READ_Q_CHANNEL        CTL_CODE(IOCTL_CDROM_BASE, 0x000B, METHOD_BUFFERED, FILE_READ_ACCESS)
#if (NTDDI_VERSION < NTDDI_WS03)
#define IOCTL_CDROM_GET_CONTROL           CTL_CODE(IOCTL_CDROM_BASE, 0x000D, METHOD_BUFFERED, FILE_READ_ACCESS)
#else
#define OBSOLETE_IOCTL_CDROM_GET_CONTROL  CTL_CODE(IOCTL_CDROM_BASE, 0x000D, METHOD_BUFFERED, FILE_READ_ACCESS)
#endif
#define IOCTL_CDROM_GET_LAST_SESSION      CTL_CODE(IOCTL_CDROM_BASE, 0x000E, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_RAW_READ              CTL_CODE(IOCTL_CDROM_BASE, 0x000F, METHOD_OUT_DIRECT,  FILE_READ_ACCESS)
#define IOCTL_CDROM_DISK_TYPE             CTL_CODE(IOCTL_CDROM_BASE, 0x0010, METHOD_BUFFERED, FILE_ANY_ACCESS)

#define IOCTL_CDROM_GET_DRIVE_GEOMETRY    CTL_CODE(IOCTL_CDROM_BASE, 0x0013, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_GET_DRIVE_GEOMETRY_EX CTL_CODE(IOCTL_CDROM_BASE, 0x0014, METHOD_BUFFERED, FILE_READ_ACCESS)

#define IOCTL_CDROM_READ_TOC_EX           CTL_CODE(IOCTL_CDROM_BASE, 0x0015, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_GET_CONFIGURATION     CTL_CODE(IOCTL_CDROM_BASE, 0x0016, METHOD_BUFFERED, FILE_READ_ACCESS)

#define IOCTL_CDROM_EXCLUSIVE_ACCESS      CTL_CODE(IOCTL_CDROM_BASE, 0x0017, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_CDROM_SET_SPEED             CTL_CODE(IOCTL_CDROM_BASE, 0x0018, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_GET_INQUIRY_DATA      CTL_CODE(IOCTL_CDROM_BASE, 0x0019, METHOD_BUFFERED, FILE_READ_ACCESS)

#define IOCTL_CDROM_ENABLE_STREAMING      CTL_CODE(IOCTL_CDROM_BASE, 0x001A, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_SEND_OPC_INFORMATION  CTL_CODE(IOCTL_CDROM_BASE, 0x001B, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_CDROM_GET_PERFORMANCE       CTL_CODE(IOCTL_CDROM_BASE, 0x001C, METHOD_BUFFERED, FILE_READ_ACCESS)

// end_winioctl

//
// The following device control codes are common for all class drivers.  The
// functions codes defined here must match all of the other class drivers.
//
// Warning: these codes will be replaced in the future with the IOCTL_STORAGE
// codes included below
//

#define IOCTL_CDROM_CHECK_VERIFY    CTL_CODE(IOCTL_CDROM_BASE, 0x0200, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_MEDIA_REMOVAL   CTL_CODE(IOCTL_CDROM_BASE, 0x0201, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_EJECT_MEDIA     CTL_CODE(IOCTL_CDROM_BASE, 0x0202, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_LOAD_MEDIA      CTL_CODE(IOCTL_CDROM_BASE, 0x0203, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_RESERVE         CTL_CODE(IOCTL_CDROM_BASE, 0x0204, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_RELEASE         CTL_CODE(IOCTL_CDROM_BASE, 0x0205, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CDROM_FIND_NEW_DEVICES CTL_CODE(IOCTL_CDROM_BASE, 0x0206, METHOD_BUFFERED, FILE_READ_ACCESS)

//
// The following file contains the IOCTL_STORAGE class ioctl definitions
//

#include <ntddstor.h>

// begin_winioctl


#define MINIMUM_CDROM_INQUIRY_SIZE  36 // RTL_SIZEOF_THROUGH_FIELD(INQUIRYDATA, ProductRevisionLevel)
#define MAXIMUM_CDROM_INQUIRY_SIZE 260 // MAXUCHAR + RTL_SIZEOF_THROUGH_FIELD(INQUIRYDATA, AdditionalLength)

//
// The following device control code is for the SIMBAD simulated bad
// sector facility. See SIMBAD.H in this directory for related structures.
//

#define IOCTL_CDROM_SIMBAD          CTL_CODE(IOCTL_CDROM_BASE, 0x1003, METHOD_BUFFERED, FILE_READ_ACCESS)

//
// Maximum CD Rom size
//

#define MAXIMUM_NUMBER_TRACKS 100
#define MAXIMUM_CDROM_SIZE 804
#define MINIMUM_CDROM_READ_TOC_EX_SIZE 2  // two bytes min transferred

//
// READ_TOC_EX structure
//
typedef struct _CDROM_READ_TOC_EX {
    UCHAR Format    : 4;
    UCHAR Reserved1 : 3; // future expansion
    UCHAR Msf       : 1;
    UCHAR SessionTrack;
    UCHAR Reserved2;     // future expansion
    UCHAR Reserved3;     // future expansion
} CDROM_READ_TOC_EX, *PCDROM_READ_TOC_EX;

#define CDROM_READ_TOC_EX_FORMAT_TOC      0x00
#define CDROM_READ_TOC_EX_FORMAT_SESSION  0x01
#define CDROM_READ_TOC_EX_FORMAT_FULL_TOC 0x02
#define CDROM_READ_TOC_EX_FORMAT_PMA      0x03
#define CDROM_READ_TOC_EX_FORMAT_ATIP     0x04
#define CDROM_READ_TOC_EX_FORMAT_CDTEXT   0x05

//
// CD ROM Table OF Contents (TOC)
// Format 0 - Get table of contents
//

typedef struct _TRACK_DATA {
    UCHAR Reserved;
    UCHAR Control : 4;
    UCHAR Adr : 4;
    UCHAR TrackNumber;
    UCHAR Reserved1;
    UCHAR Address[4];
} TRACK_DATA, *PTRACK_DATA;

typedef struct _CDROM_TOC {

    //
    // Header
    //

    UCHAR Length[2];  // add two bytes for this field
    UCHAR FirstTrack;
    UCHAR LastTrack;

    //
    // Track data
    //

    TRACK_DATA TrackData[MAXIMUM_NUMBER_TRACKS];
} CDROM_TOC, *PCDROM_TOC;

#define CDROM_TOC_SIZE sizeof(CDROM_TOC)

//
// CD ROM Table OF Contents
// Format 1 - Session Information
//

typedef struct _CDROM_TOC_SESSION_DATA {

    //
    // Header
    //

    UCHAR Length[2];  // add two bytes for this field
    UCHAR FirstCompleteSession;
    UCHAR LastCompleteSession;

    //
    // One track, representing the first track
    // of the last finished session
    //

    TRACK_DATA TrackData[1];

} CDROM_TOC_SESSION_DATA, *PCDROM_TOC_SESSION_DATA;


//
// CD ROM Table OF Contents
// Format 2 - Full TOC
//

typedef struct _CDROM_TOC_FULL_TOC_DATA_BLOCK {
    UCHAR SessionNumber;
    UCHAR Control      : 4;
    UCHAR Adr          : 4;
    UCHAR Reserved1;
    UCHAR Point;
    UCHAR MsfExtra[3];
    UCHAR Zero;
    UCHAR Msf[3];
} CDROM_TOC_FULL_TOC_DATA_BLOCK, *PCDROM_TOC_FULL_TOC_DATA_BLOCK;

typedef struct _CDROM_TOC_FULL_TOC_DATA {

    //
    // Header
    //

    UCHAR Length[2];  // add two bytes for this field
    UCHAR FirstCompleteSession;
    UCHAR LastCompleteSession;

    //
    // one to N descriptors included
    //

#if !defined(__midl)
    CDROM_TOC_FULL_TOC_DATA_BLOCK Descriptors[0];
#endif

} CDROM_TOC_FULL_TOC_DATA, *PCDROM_TOC_FULL_TOC_DATA;

//
// CD ROM Table OF Contents
// Format 3 - Program Memory Area
//
typedef struct _CDROM_TOC_PMA_DATA {

    //
    // Header
    //

    UCHAR Length[2];  // add two bytes for this field
    UCHAR Reserved1;
    UCHAR Reserved2;

    //
    // one to N descriptors included
    //

#if !defined(__midl)
    CDROM_TOC_FULL_TOC_DATA_BLOCK Descriptors[0];
#endif

} CDROM_TOC_PMA_DATA, *PCDROM_TOC_PMA_DATA;

//
// CD ROM Table OF Contents
// Format 4 - Absolute Time In Pregroove
//

typedef struct _CDROM_TOC_ATIP_DATA_BLOCK {

    UCHAR CdrwReferenceSpeed : 3;
    UCHAR Reserved3          : 1;
    UCHAR WritePower         : 3;
    UCHAR True1              : 1;
    UCHAR Reserved4       : 6;
    UCHAR UnrestrictedUse : 1;
    UCHAR Reserved5       : 1;
    UCHAR A3Valid     : 1;
    UCHAR A2Valid     : 1;
    UCHAR A1Valid     : 1;
    UCHAR DiscSubType : 3;
    UCHAR IsCdrw      : 1;
    UCHAR True2       : 1;
    UCHAR Reserved7;

    UCHAR LeadInMsf[3];
    UCHAR Reserved8;

    UCHAR LeadOutMsf[3];
    UCHAR Reserved9;

    UCHAR A1Values[3];
    UCHAR Reserved10;

    UCHAR A2Values[3];
    UCHAR Reserved11;

    UCHAR A3Values[3];
    UCHAR Reserved12;

} CDROM_TOC_ATIP_DATA_BLOCK, *PCDROM_TOC_ATIP_DATA_BLOCK;

typedef struct _CDROM_TOC_ATIP_DATA {

    //
    // Header
    //

    UCHAR Length[2];  // add two bytes for this field
    UCHAR Reserved1;
    UCHAR Reserved2;

    //
    // zero? to N descriptors included.
    //

#if !defined(__midl)
    CDROM_TOC_ATIP_DATA_BLOCK Descriptors[0];
#endif

} CDROM_TOC_ATIP_DATA, *PCDROM_TOC_ATIP_DATA;

//
// CD ROM Table OF Contents
// Format 5 - CD Text Info
//
typedef struct _CDROM_TOC_CD_TEXT_DATA_BLOCK {
    UCHAR PackType;
    UCHAR TrackNumber       : 7;
    UCHAR ExtensionFlag     : 1;  // should be zero!
    UCHAR SequenceNumber;
    UCHAR CharacterPosition : 4;
    UCHAR BlockNumber       : 3;
    UCHAR Unicode           : 1;
    union {
        UCHAR Text[12];
        WCHAR WText[6];
    };
    UCHAR CRC[2];
} CDROM_TOC_CD_TEXT_DATA_BLOCK, *PCDROM_TOC_CD_TEXT_DATA_BLOCK;

typedef struct _CDROM_TOC_CD_TEXT_DATA {

    //
    // Header
    //

    UCHAR Length[2];  // add two bytes for this field
    UCHAR Reserved1;
    UCHAR Reserved2;

    //
    // the text info comes in discrete blocks of
    // a heavily-overloaded structure
    //

#if !defined(__midl)
    CDROM_TOC_CD_TEXT_DATA_BLOCK Descriptors[0];
#endif

} CDROM_TOC_CD_TEXT_DATA, *PCDROM_TOC_CD_TEXT_DATA;

//
// These are the types used for PackType field in CDROM_TOC_CD_TEXT_DATA_BLOCK
// and also for requesting specific info from IOCTL_CDROM_READ_CD_TEXT
//
#define CDROM_CD_TEXT_PACK_ALBUM_NAME 0x80
#define CDROM_CD_TEXT_PACK_PERFORMER  0x81
#define CDROM_CD_TEXT_PACK_SONGWRITER 0x82
#define CDROM_CD_TEXT_PACK_COMPOSER   0x83
#define CDROM_CD_TEXT_PACK_ARRANGER   0x84
#define CDROM_CD_TEXT_PACK_MESSAGES   0x85
#define CDROM_CD_TEXT_PACK_DISC_ID    0x86
#define CDROM_CD_TEXT_PACK_GENRE      0x87
#define CDROM_CD_TEXT_PACK_TOC_INFO   0x88
#define CDROM_CD_TEXT_PACK_TOC_INFO2  0x89
// 0x8a - 0x8d are reserved....
#define CDROM_CD_TEXT_PACK_UPC_EAN    0x8e
#define CDROM_CD_TEXT_PACK_SIZE_INFO  0x8f

//
// Play audio starting at MSF and ending at MSF
//

typedef struct _CDROM_PLAY_AUDIO_MSF {
    UCHAR StartingM;
    UCHAR StartingS;
    UCHAR StartingF;
    UCHAR EndingM;
    UCHAR EndingS;
    UCHAR EndingF;
} CDROM_PLAY_AUDIO_MSF, *PCDROM_PLAY_AUDIO_MSF;

//
// Seek to MSF
//

typedef struct _CDROM_SEEK_AUDIO_MSF {
    UCHAR M;
    UCHAR S;
    UCHAR F;
} CDROM_SEEK_AUDIO_MSF, *PCDROM_SEEK_AUDIO_MSF;


//
//  Flags for the disk type
//

typedef struct _CDROM_DISK_DATA {

    ULONG DiskData;

} CDROM_DISK_DATA, *PCDROM_DISK_DATA;

#define CDROM_DISK_AUDIO_TRACK      (0x00000001)
#define CDROM_DISK_DATA_TRACK       (0x00000002)

//
// CD ROM Data Mode Codes, used with IOCTL_CDROM_READ_Q_CHANNEL
//

#define IOCTL_CDROM_SUB_Q_CHANNEL    0x00
#define IOCTL_CDROM_CURRENT_POSITION 0x01
#define IOCTL_CDROM_MEDIA_CATALOG    0x02
#define IOCTL_CDROM_TRACK_ISRC       0x03

typedef struct _CDROM_SUB_Q_DATA_FORMAT {
    UCHAR Format;
    UCHAR Track;
} CDROM_SUB_Q_DATA_FORMAT, *PCDROM_SUB_Q_DATA_FORMAT;


//
// CD ROM Sub-Q Channel Data Format
//

typedef struct _SUB_Q_HEADER {
    UCHAR Reserved;
    UCHAR AudioStatus;
    UCHAR DataLength[2];
} SUB_Q_HEADER, *PSUB_Q_HEADER;

typedef struct _SUB_Q_CURRENT_POSITION {
    SUB_Q_HEADER Header;
    UCHAR FormatCode;
    UCHAR Control : 4;
    UCHAR ADR : 4;
    UCHAR TrackNumber;
    UCHAR IndexNumber;
    UCHAR AbsoluteAddress[4];
    UCHAR TrackRelativeAddress[4];
} SUB_Q_CURRENT_POSITION, *PSUB_Q_CURRENT_POSITION;

typedef struct _SUB_Q_MEDIA_CATALOG_NUMBER {
    SUB_Q_HEADER Header;
    UCHAR FormatCode;
    UCHAR Reserved[3];
    UCHAR Reserved1 : 7;
    UCHAR Mcval : 1;
    UCHAR MediaCatalog[15];
} SUB_Q_MEDIA_CATALOG_NUMBER, *PSUB_Q_MEDIA_CATALOG_NUMBER;

typedef struct _SUB_Q_TRACK_ISRC {
    SUB_Q_HEADER Header;
    UCHAR FormatCode;
    UCHAR Reserved0;
    UCHAR Track;
    UCHAR Reserved1;
    UCHAR Reserved2 : 7;
    UCHAR Tcval : 1;
    UCHAR TrackIsrc[15];
} SUB_Q_TRACK_ISRC, *PSUB_Q_TRACK_ISRC;

typedef union _SUB_Q_CHANNEL_DATA {
    SUB_Q_CURRENT_POSITION CurrentPosition;
    SUB_Q_MEDIA_CATALOG_NUMBER MediaCatalog;
    SUB_Q_TRACK_ISRC TrackIsrc;
} SUB_Q_CHANNEL_DATA, *PSUB_Q_CHANNEL_DATA;

//
// Audio Status Codes
//

#define AUDIO_STATUS_NOT_SUPPORTED  0x00
#define AUDIO_STATUS_IN_PROGRESS    0x11
#define AUDIO_STATUS_PAUSED         0x12
#define AUDIO_STATUS_PLAY_COMPLETE  0x13
#define AUDIO_STATUS_PLAY_ERROR     0x14
#define AUDIO_STATUS_NO_STATUS      0x15

//
// ADR Sub-channel Q Field
//

#define ADR_NO_MODE_INFORMATION     0x0
#define ADR_ENCODES_CURRENT_POSITION 0x1
#define ADR_ENCODES_MEDIA_CATALOG   0x2
#define ADR_ENCODES_ISRC            0x3

//
// Sub-channel Q Control Bits
//

#define AUDIO_WITH_PREEMPHASIS      0x1
#define DIGITAL_COPY_PERMITTED      0x2
#define AUDIO_DATA_TRACK            0x4
#define TWO_FOUR_CHANNEL_AUDIO      0x8

#if (NTDDI_VERSION < NTDDI_WS03)
typedef struct _CDROM_AUDIO_CONTROL {
    UCHAR LbaFormat;
    USHORT LogicalBlocksPerSecond;
} CDROM_AUDIO_CONTROL, *PCDROM_AUDIO_CONTROL;
#else
#if PRAGMA_DEPRECATED_DDK
#define _CDROM_AUDIO_CONTROL _this_is_obsoleted__CDROM_AUDIO_CONTROL
#define CDROM_AUDIO_CONTROL  _this_is_obsoleted_CDROM_AUDIO_CONTROL
#define PCDROM_AUDIO_CONTROL _this_is_obsoleted_PCDROM_AUDIO_CONTROL
#endif // PRAGMA_DEPRECATED_DDK
#endif

//
// Volume control - Volume takes a value between 1 and 0xFF.
// SCSI-II CDROM audio suppports up to 4 audio ports with
// Independent volume control.
//

typedef struct _VOLUME_CONTROL {
    UCHAR PortVolume[4];
} VOLUME_CONTROL, *PVOLUME_CONTROL;

typedef enum _TRACK_MODE_TYPE {
    YellowMode2,
    XAForm2,
    CDDA,
    RawWithC2AndSubCode,   // CD_RAW_SECTOR_WITH_C2_AND_SUBCODE_SIZE per sector
    RawWithC2,             // CD_RAW_SECTOR_WITH_C2_SIZE per sector
    RawWithSubCode         // CD_RAW_SECTOR_WITH_SUBCODE_SIZE per sector
} TRACK_MODE_TYPE, *PTRACK_MODE_TYPE;

#define CD_RAW_READ_C2_SIZE                    (     296   )
#define CD_RAW_READ_SUBCODE_SIZE               (         96)
#define CD_RAW_SECTOR_WITH_C2_SIZE             (2352+296   )
#define CD_RAW_SECTOR_WITH_SUBCODE_SIZE        (2352    +96)
#define CD_RAW_SECTOR_WITH_C2_AND_SUBCODE_SIZE (2352+296+96)

//
// Passed to cdrom to describe the raw read, ie. Mode 2, Form 2, CDDA...
//

typedef struct __RAW_READ_INFO {
    LARGE_INTEGER DiskOffset;
    ULONG    SectorCount;
    TRACK_MODE_TYPE TrackMode;
} RAW_READ_INFO, *PRAW_READ_INFO;

typedef enum _MEDIA_BLANK_TYPE {
    MediaBlankTypeFull = 0,               // mandatory support
    MediaBlankTypeMinimal = 1,            // mandatory support
    MediaBlankTypeIncompleteTrack = 2,    // optional support
    MediaBlankTypeUnreserveLastTrack = 3, // optional support, hairy
    MediaBlankTypeTrackTail = 4,          // mandatory support
    MediaBlankTypeUncloseLastSession = 5, // optional support
    MediaBlankTypeEraseLastSession = 6,   // optional support
    // MediaBlankType7 is reserved
} MEDIA_BLANK_TYPE, *PMEDIA_BLANK_TYPE;

//
// IOCTL_CDROM_EXCLUSIVE_ACCESS can be used to get exclusive
// access to the CDROM device.
//

#define CDROM_EXCLUSIVE_CALLER_LENGTH   64

//
// Input values (Flags) for ExclusiveAccessLockDevice
//
// CDROM_LOCK_IGNORE_VOLUME:
//      Set it to lock the device even if the file system is mounted.
//      WARNING: setting this may cause data corruption!
//
// CDROM_NO_MEDIA_NOTIFICATIONS:
//      Set it to prevent sending of a media removal notification
//      on exclusive access lock and a media arrival notification 
//      on unlock.
//

#define CDROM_LOCK_IGNORE_VOLUME        (1 << 0)
#define CDROM_NO_MEDIA_NOTIFICATIONS    (1 << 1)

//
// Output values (Flags) for ExclusiveAccessQueryState
//

#define CDROM_NOT_IN_EXCLUSIVE_MODE     0
#define CDROM_IN_EXCLUSIVE_MODE         1


typedef enum _EXCLUSIVE_ACCESS_REQUEST_TYPE {
    ExclusiveAccessQueryState,
    ExclusiveAccessLockDevice,
    ExclusiveAccessUnlockDevice
} EXCLUSIVE_ACCESS_REQUEST_TYPE, *PEXCLUSIVE_ACCESS_REQUEST_TYPE;


typedef struct _CDROM_EXCLUSIVE_ACCESS {

    //
    // Request type
    //
    EXCLUSIVE_ACCESS_REQUEST_TYPE RequestType;

    //
    // Additional parameters for each request type
    //
    ULONG Flags;

} CDROM_EXCLUSIVE_ACCESS, *PCDROM_EXCLUSIVE_ACCESS;


typedef struct _CDROM_EXCLUSIVE_LOCK {

    CDROM_EXCLUSIVE_ACCESS  Access;

    //
    // Caller name string
    //
    UCHAR CallerName[CDROM_EXCLUSIVE_CALLER_LENGTH];

} CDROM_EXCLUSIVE_LOCK, *PCDROM_EXCLUSIVE_LOCK;


typedef struct _CDROM_EXCLUSIVE_LOCK_STATE {

    BOOLEAN  LockState;

    //
    // Caller name string
    //
    UCHAR CallerName[CDROM_EXCLUSIVE_CALLER_LENGTH];

} CDROM_EXCLUSIVE_LOCK_STATE, *PCDROM_EXCLUSIVE_LOCK_STATE;

//
// Structure definitions for IOCTL_CDROM_SET_SPEED
//

typedef enum _CDROM_SPEED_REQUEST {
    CdromSetSpeed,
    CdromSetStreaming
} CDROM_SPEED_REQUEST, *PCDROM_SPEED_REQUEST;

typedef enum _WRITE_ROTATION {
    CdromDefaultRotation,
    CdromCAVRotation
} WRITE_ROTATION, *PWRITE_ROTATION;

typedef struct _CDROM_SET_SPEED {

    //
    // Request type for setting speed
    //
    CDROM_SPEED_REQUEST RequestType;

    //
    // Drive read speed in KB/sec.
    //
    USHORT ReadSpeed;

    //
    // Drive write speed in KB/sec.
    //
    USHORT WriteSpeed;

    //
    // Drive rotation control for write
    //
    WRITE_ROTATION RotationControl;

} CDROM_SET_SPEED, *PCDROM_SET_SPEED;

typedef struct _CDROM_SET_STREAMING {

    //
    // Request type for setting speed
    //
    CDROM_SPEED_REQUEST RequestType;

    //
    // Drive read size in KB per ReadTime
    //
    ULONG ReadSize;

    //
    // Read time in milliseconds
    //
    ULONG ReadTime;

    //
    // Drive write size in KB per WriteTime
    //
    ULONG WriteSize;

    //
    // Write time in milliseconds
    //
    ULONG WriteTime;

    //
    // First Logical Block Address of the request
    //
    ULONG StartLba;

    //
    // Last Logical Block Address of the request
    //
    ULONG EndLba;

    //
    // Drive rotation control for write
    //
    WRITE_ROTATION RotationControl;

    //
    // Restore drive defaults
    //
    BOOLEAN RestoreDefaults;

    //
    // Set drive to exact value given
    //
    BOOLEAN SetExact;

    //
    // Optimize performance for random changes
    //
    BOOLEAN RandomAccess;

    //
    // Restore default speed on media change
    //
    BOOLEAN Persistent;

} CDROM_SET_STREAMING, *PCDROM_SET_STREAMING;


//
// Structure definitions for IOCTL_CDROM_ENABLE_STREAMING
//

typedef enum _STREAMING_CONTROL_REQUEST_TYPE {
    CdromStreamingDisable = 1,
    CdromStreamingEnableForReadOnly = 2,
    CdromStreamingEnableForWriteOnly = 3,
    CdromStreamingEnableForReadWrite = 4
} STREAMING_CONTROL_REQUEST_TYPE, *PSTREAMING_CONTROL_REQUEST_TYPE;

typedef struct _CDROM_STREAMING_CONTROL {

    //
    // Request type
    //
    STREAMING_CONTROL_REQUEST_TYPE   RequestType;

} CDROM_STREAMING_CONTROL, *PCDROM_STREAMING_CONTROL;


//
// Structure definitions for IOCTL_CDROM_SEND_OPC_INFORMATION
//

typedef enum _CDROM_OPC_INFO_TYPE {
    SimpleOpcInfo = 1
} CDROM_OPC_INFO_TYPE, *PCDROM_OPC_INFO_TYPE;

typedef struct _CDROM_SIMPLE_OPC_INFO {
    
    //
    // Request type (must be SimpleOpcInfo)
    //
    CDROM_OPC_INFO_TYPE   RequestType;

    //
    // Exclude Layer 0 from OPC
    //
    BOOLEAN               Exclude0;

    //
    // Exclude Layer 1 from OPC
    //
    BOOLEAN               Exclude1;

} CDROM_SIMPLE_OPC_INFO, *PCDROM_SIMPLE_OPC_INFO;


//
// Structure definitions for IOCTL_CDROM_GET_PERFORMANCE
//

typedef enum _CDROM_PERFORMANCE_REQUEST_TYPE {
    CdromPerformanceRequest = 1,
    CdromWriteSpeedRequest = 2
} CDROM_PERFORMANCE_REQUEST_TYPE, *PCDROM_PERFORMANCE_REQUEST_TYPE;

typedef enum _CDROM_PERFORMANCE_TYPE {
    CdromReadPerformance = 1,
    CdromWritePerformance = 2
} CDROM_PERFORMANCE_TYPE, *PCDROM_PERFORMANCE_TYPE;

typedef enum _CDROM_PERFORMANCE_EXCEPTION_TYPE {
    CdromNominalPerformance = 1,
    CdromEntirePerformanceList = 2,
    CdromPerformanceExceptionsOnly = 3
} CDROM_PERFORMANCE_EXCEPTION_TYPE, *PCDROM_PERFORMANCE_EXCEPTION_TYPE;

typedef enum _CDROM_PERFORMANCE_TOLERANCE_TYPE {
    Cdrom10Nominal20Exceptions = 1
} CDROM_PERFORMANCE_TOLERANCE_TYPE, *PCDROM_PERFORMANCE_TOLERANCE_TYPE;

typedef struct _CDROM_PERFORMANCE_REQUEST {

    //
    // Request type (must be CdromPerformanceRequest)
    //
    CDROM_PERFORMANCE_REQUEST_TYPE   RequestType;

    //
    // Performance type
    //
    CDROM_PERFORMANCE_TYPE           PerformanceType;

    //
    // Exceptions to be covered
    //
    CDROM_PERFORMANCE_EXCEPTION_TYPE Exceptions;

    //
    // Tolerance to be ensured
    //
    CDROM_PERFORMANCE_TOLERANCE_TYPE Tolerance;

    //
    // Starting LBA for entire performance list
    //
    ULONG                            StaringLba;

} CDROM_PERFORMANCE_REQUEST, *PCDROM_PERFORMANCE_REQUEST;

typedef struct _CDROM_WRITE_SPEED_REQUEST {

    //
    // Request type (must be CdromWriteSpeedRequest)
    //
    CDROM_PERFORMANCE_REQUEST_TYPE   RequestType;

} CDROM_WRITE_SPEED_REQUEST, *PCDROM_WRITE_SPEED_REQUEST;

// Header for data returned by IOCTL_CDROM_GET_PERFORMANCE
typedef struct _CDROM_PERFORMANCE_HEADER {

    //
    // Size of available data (vs returned data), not including this field 
    //
    UCHAR                        DataLength[4];

    UCHAR                        Except : 1;
    UCHAR                        Write : 1;
    UCHAR                        Reserved1 : 6;
    UCHAR                        Reserved2[3];

    //
    // Contains a list of the following records (depending on the request):
    //      CDROM_NOMINAL_PERFORMANCE_DESCRIPTOR,
    //      CDROM_EXCEPTION_PERFORMANCE_DESCRIPTOR,
    //      CDROM_WRITE_SPEED_DESCRIPTOR
    //
    UCHAR                        Data[0];

} CDROM_PERFORMANCE_HEADER, *PCDROM_PERFORMANCE_HEADER;

typedef struct _CDROM_NOMINAL_PERFORMANCE_DESCRIPTOR {
    UCHAR                        StartLba[4];
    UCHAR                        StartPerformance[4];
    UCHAR                        EndLba[4];
    UCHAR                        EndPerformance[4];
} CDROM_NOMINAL_PERFORMANCE_DESCRIPTOR, *PCDROM_NOMINAL_PERFORMANCE_DESCRIPTOR;

typedef struct _CDROM_EXCEPTION_PERFORMANCE_DESCRIPTOR {
    UCHAR                        Lba[4];
    UCHAR                        Time[2];
} CDROM_EXCEPTION_PERFORMANCE_DESCRIPTOR, *PCDROM_EXCEPTION_PERFORMANCE_DESCRIPTOR;

typedef struct _CDROM_WRITE_SPEED_DESCRIPTOR {
    UCHAR                        MixedReadWrite : 1;
    UCHAR                        Exact : 1;
    UCHAR                        Reserved1 : 1;
    UCHAR                        WriteRotationControl : 2;
    UCHAR                        Reserved2 : 3;
    UCHAR                        Reserved3[3];
    UCHAR                        EndLba[4];
    UCHAR                        ReadSpeed[4];
    UCHAR                        WriteSpeed[4];
} CDROM_WRITE_SPEED_DESCRIPTOR, *PCDROM_WRITE_SPEED_DESCRIPTOR;


#ifdef __cplusplus
}
#endif


#if _MSC_VER >= 1200
#pragma warning(pop)          // un-sets any local warning changes
#else
#pragma warning(default:4200) // array[0] is not a warning for this file
#pragma warning(default:4201) // nameless struct/unions
#pragma warning(default:4214) // bit fields other than int
#endif


#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif  // _NTDDCDRM_

// end_winioctl

