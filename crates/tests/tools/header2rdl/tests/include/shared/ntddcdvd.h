/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ntddcdvd.h

Abstract:

    This module contains structures and definitions
    associated with DVD ioctls.

    This module is used in conjunction with ntddcdrm.h which contains the
    cdrom specific ioctls which will work on CDVD drives


--*/

// begin_winioctl

#ifndef _NTDDCDVD_
#define _NTDDCDVD_
#pragma warning(push)
#pragma warning(disable:4200) // zero-sized array
#pragma warning(disable:4201) // nonstandard extension used : nameless struct/union
#pragma warning(disable:4214) // bitfield other than int

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


//
// NtDeviceIoControlFile IoControlCode values for this device.
//
// Warning:  Remember that the low two bits of the code specify how the
//           buffers are passed to the driver!
//

#define IOCTL_DVD_BASE                 FILE_DEVICE_DVD

//
// CDVD Device Control Functions
//
// Warning: Ioctls from 200 through 300 are used for the old common class
// driver ioctls and should not be used for device specific functionality
//

//
// CSS-related IOCTLs
//

#define IOCTL_DVD_START_SESSION     CTL_CODE(IOCTL_DVD_BASE, 0x0400, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DVD_READ_KEY          CTL_CODE(IOCTL_DVD_BASE, 0x0401, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DVD_SEND_KEY          CTL_CODE(IOCTL_DVD_BASE, 0x0402, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DVD_END_SESSION       CTL_CODE(IOCTL_DVD_BASE, 0x0403, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DVD_SET_READ_AHEAD    CTL_CODE(IOCTL_DVD_BASE, 0x0404, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DVD_GET_REGION        CTL_CODE(IOCTL_DVD_BASE, 0x0405, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DVD_SEND_KEY2         CTL_CODE(IOCTL_DVD_BASE, 0x0406, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// AACS-related IOCTLs
//
#define IOCTL_AACS_READ_MEDIA_KEY_BLOCK_SIZE  CTL_CODE(IOCTL_DVD_BASE, 0x430, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_AACS_READ_MEDIA_KEY_BLOCK       CTL_CODE(IOCTL_DVD_BASE, 0x431, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_AACS_START_SESSION              CTL_CODE(IOCTL_DVD_BASE, 0x432, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_AACS_END_SESSION                CTL_CODE(IOCTL_DVD_BASE, 0x433, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_AACS_SEND_CERTIFICATE           CTL_CODE(IOCTL_DVD_BASE, 0x434, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_AACS_GET_CERTIFICATE            CTL_CODE(IOCTL_DVD_BASE, 0x435, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_AACS_GET_CHALLENGE_KEY          CTL_CODE(IOCTL_DVD_BASE, 0x436, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_AACS_SEND_CHALLENGE_KEY         CTL_CODE(IOCTL_DVD_BASE, 0x437, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_AACS_READ_VOLUME_ID             CTL_CODE(IOCTL_DVD_BASE, 0x438, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_AACS_READ_SERIAL_NUMBER         CTL_CODE(IOCTL_DVD_BASE, 0x439, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_AACS_READ_MEDIA_ID              CTL_CODE(IOCTL_DVD_BASE, 0x43A, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_AACS_READ_BINDING_NONCE         CTL_CODE(IOCTL_DVD_BASE, 0x43B, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_AACS_GENERATE_BINDING_NONCE     CTL_CODE(IOCTL_DVD_BASE, 0x43C, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// DVD Structure queries
//

#define IOCTL_DVD_READ_STRUCTURE    CTL_CODE(IOCTL_DVD_BASE, 0x0450, METHOD_BUFFERED, FILE_READ_ACCESS)

//
// The following file contains the IOCTL_STORAGE class ioctl definitions
//

#define IOCTL_STORAGE_SET_READ_AHEAD        CTL_CODE(IOCTL_STORAGE_BASE, 0x0100, METHOD_BUFFERED, FILE_READ_ACCESS)

// end_winioctl

#include <ntddstor.h>

// begin_winioctl


#ifdef __cplusplus
extern "C" {
#endif


typedef enum {
    DvdChallengeKey = 0x01,
    DvdBusKey1,
    DvdBusKey2,
    DvdTitleKey,
    DvdAsf,
    DvdSetRpcKey = 0x6,
    DvdGetRpcKey = 0x8,
    DvdDiskKey = 0x80,
    DvdInvalidateAGID = 0x3f
} DVD_KEY_TYPE;

typedef _At_((LONG)_Curr_, _Field_range_(-1,3)) ULONG DVD_SESSION_ID, *PDVD_SESSION_ID;

#include <pshpack1.h>
typedef struct _DVD_COPY_PROTECT_KEY {
    ULONG KeyLength;
    DVD_SESSION_ID SessionId;
    DVD_KEY_TYPE KeyType;
    ULONG KeyFlags;
    union {
        HANDLE FileHandle;
        LARGE_INTEGER TitleOffset;
    } Parameters;
    UCHAR KeyData[0];
} DVD_COPY_PROTECT_KEY, *PDVD_COPY_PROTECT_KEY;
#include <poppack.h>

//
// Predefined (Mt. Fuji) key sizes
// Add sizeof(DVD_COPY_PROTECT_KEY) to get allocation size for
// the full key structure
//

#define DVD_CHALLENGE_KEY_LENGTH    (12 + sizeof(DVD_COPY_PROTECT_KEY))
#define DVD_BUS_KEY_LENGTH          (8 + sizeof(DVD_COPY_PROTECT_KEY))
#define DVD_TITLE_KEY_LENGTH        (8 + sizeof(DVD_COPY_PROTECT_KEY))
#define DVD_DISK_KEY_LENGTH         (2048 + sizeof(DVD_COPY_PROTECT_KEY))
#define DVD_RPC_KEY_LENGTH          (sizeof(DVD_RPC_KEY) + sizeof(DVD_COPY_PROTECT_KEY))
#define DVD_SET_RPC_KEY_LENGTH      (sizeof(DVD_SET_RPC_KEY) + sizeof(DVD_COPY_PROTECT_KEY))
#define DVD_ASF_LENGTH              (sizeof(DVD_ASF) + sizeof(DVD_COPY_PROTECT_KEY))

//
// Used with IOCTL_DVD_END_SESSION to end all DVD sessions at once
//

#define DVD_END_ALL_SESSIONS ((DVD_SESSION_ID) 0xffffffff)

//
// CGMS Copy Protection Flags
//

#define DVD_CGMS_RESERVED_MASK      0x00000078

#define DVD_CGMS_COPY_PROTECT_MASK  0x00000018
#define DVD_CGMS_COPY_PERMITTED     0x00000000
#define DVD_CGMS_COPY_ONCE          0x00000010
#define DVD_CGMS_NO_COPY            0x00000018

#define DVD_COPYRIGHT_MASK          0x00000040
#define DVD_NOT_COPYRIGHTED         0x00000000
#define DVD_COPYRIGHTED             0x00000040

#define DVD_SECTOR_PROTECT_MASK     0x00000020
#define DVD_SECTOR_NOT_PROTECTED    0x00000000
#define DVD_SECTOR_PROTECTED        0x00000020

/*++

IOCTL_STORAGE_SET_READ_AHEAD

Requests that the storage device skip to TargetAddress once it has run across
TriggerAddress during the course of it's read-ahead caching operations.

Input:

    a STORAGE_SET_READ_AHEAD structure which contains:
        * the trigger address
        * the target address

Output:

    none

--*/

#include <pshpack1.h>
typedef struct _STORAGE_SET_READ_AHEAD {
    LARGE_INTEGER TriggerAddress;
    LARGE_INTEGER TargetAddress;
} STORAGE_SET_READ_AHEAD, *PSTORAGE_SET_READ_AHEAD;
#include <poppack.h>

/*++

IOCTL_DVD_READ_STRUCTURE

Issues a READ_DVD_STRUCTURE command to the drive.

Input:

    a DVD_READ_STRUCTURE describing what information is requested

Output:

    a DVD Layer Descriptor as defined below

--*/

typedef enum DVD_STRUCTURE_FORMAT {
    DvdPhysicalDescriptor,     // 0x00
    DvdCopyrightDescriptor,    // 0x01
    DvdDiskKeyDescriptor,      // 0x02
    DvdBCADescriptor,          // 0x03
    DvdManufacturerDescriptor, // 0x04
    DvdMaxDescriptor           // 0x05
} DVD_STRUCTURE_FORMAT, *PDVD_STRUCTURE_FORMAT;

/////////////////////////////////////////////////////////////
// ALL THE FOLLOWING STRUCTURES ARE BYTE-PACKED:
#include <pshpack1.h>

typedef struct DVD_READ_STRUCTURE {
    LARGE_INTEGER BlockByteOffset;
    DVD_STRUCTURE_FORMAT Format;
    DVD_SESSION_ID SessionId;
    UCHAR LayerNumber;
} DVD_READ_STRUCTURE, *PDVD_READ_STRUCTURE;

typedef struct _DVD_DESCRIPTOR_HEADER {
    USHORT Length;
    UCHAR Reserved[2];
#if !defined(__midl)
    UCHAR Data[0];
#endif
} DVD_DESCRIPTOR_HEADER, *PDVD_DESCRIPTOR_HEADER;
C_ASSERT(sizeof(DVD_DESCRIPTOR_HEADER) == 4);

// format 0x00 - DvdPhysicalDescriptor
typedef struct _DVD_LAYER_DESCRIPTOR {
    UCHAR BookVersion : 4;      // in MMC 5 :   Part Version
    UCHAR BookType : 4;         //              Disk Category
    UCHAR MinimumRate : 4;      //              Maximum Rate
    UCHAR DiskSize : 4;
    UCHAR LayerType : 4;
    UCHAR TrackPath : 1;
    UCHAR NumberOfLayers : 2;
    UCHAR Reserved1 : 1;
    UCHAR TrackDensity : 4;
    UCHAR LinearDensity : 4;
    ULONG StartingDataSector;   //              3bytes + 1 zeroed byte
    ULONG EndDataSector;        //              3bytes + 1 zeroed byte
    ULONG EndLayerZeroSector;   //              3bytes + 1 zeroed byte
    UCHAR Reserved5 : 7;
    UCHAR BCAFlag : 1;
    // The large Media Specific field is not declared here to enable stack allocation
} DVD_LAYER_DESCRIPTOR, *PDVD_LAYER_DESCRIPTOR;
C_ASSERT(sizeof(DVD_LAYER_DESCRIPTOR) == 17);
typedef struct _DVD_FULL_LAYER_DESCRIPTOR {
    DVD_LAYER_DESCRIPTOR commonHeader;
    UCHAR MediaSpecific[2031];
} DVD_FULL_LAYER_DESCRIPTOR, *PDVD_FULL_LAYER_DESCRIPTOR;
C_ASSERT(sizeof(DVD_FULL_LAYER_DESCRIPTOR) == 2048);

// format 0x01 - DvdCopyrightDescriptor
typedef struct _DVD_COPYRIGHT_DESCRIPTOR {
    UCHAR CopyrightProtectionType;
    UCHAR RegionManagementInformation;
    USHORT Reserved;
} DVD_COPYRIGHT_DESCRIPTOR, *PDVD_COPYRIGHT_DESCRIPTOR;
C_ASSERT(sizeof(DVD_COPYRIGHT_DESCRIPTOR) == 4);

// format 0x02 - DvdDiskKeyDescriptor
typedef struct _DVD_DISK_KEY_DESCRIPTOR {
    UCHAR DiskKeyData[2048];
} DVD_DISK_KEY_DESCRIPTOR, *PDVD_DISK_KEY_DESCRIPTOR;
C_ASSERT(sizeof(DVD_DISK_KEY_DESCRIPTOR) == 2048);

// format 0x03 - DvdBCADescriptor
typedef struct _DVD_BCA_DESCRIPTOR {
    UCHAR BCAInformation[0];
} DVD_BCA_DESCRIPTOR, *PDVD_BCA_DESCRIPTOR;

// format 0x04 - DvdManufacturerDescriptor
typedef struct _DVD_MANUFACTURER_DESCRIPTOR {
    UCHAR ManufacturingInformation[2048];
} DVD_MANUFACTURER_DESCRIPTOR, *PDVD_MANUFACTURER_DESCRIPTOR;
C_ASSERT(sizeof(DVD_MANUFACTURER_DESCRIPTOR) == 2048);

// format 0x05 - not defined in enum
typedef struct _DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR {
    union {
        struct {
            UCHAR CopyProtectionMode : 4;
            UCHAR ContentGenerationManagementSystem : 2;
            UCHAR CopyProtectedSector : 1;
            UCHAR CopyProtectedMaterial : 1;
        } Dvdrom;
        struct {
            UCHAR Reserved0001 : 4;
            UCHAR ContentGenerationManagementSystem : 2;
            UCHAR Reserved0002 : 1;
            UCHAR CopyProtectedMaterial : 1;
        } DvdRecordable_Version1;
        struct {
            UCHAR Reserved0003;
        } Dvdram;
        struct {
            UCHAR Reserved0004 : 2;
            UCHAR ADP_TY : 2; // what is this mean?
            UCHAR Reserved0005 : 4;
        } DvdRecordable;
        UCHAR CPR_MAI;
    };
    UCHAR Reserved0[3];
} DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR, *PDVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR;
C_ASSERT(FIELD_OFFSET(DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR, Reserved0) == 1);
C_ASSERT(sizeof(DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR) == 4);

// format 0x06 (media ID) is unstructured in public spec
// format 0x07 (media key block) is unstructured in public spec
// format 0x08 (DVD-RAM DDS) is unstructured in public spec

// format 0x09 - not defined in enum
// This is valid for DVD-RAM and also HD DVD-RAM
typedef struct _DVD_RAM_MEDIUM_STATUS {
    UCHAR Reserved0                 : 1;
    UCHAR PersistentWriteProtect    : 1;
    UCHAR CartridgeWriteProtect     : 1;
    UCHAR MediaSpecificWriteInhibit : 1;
    UCHAR Reserved1          : 2;
    UCHAR CartridgeNotSealed : 1;
    UCHAR MediaInCartridge   : 1;
    UCHAR DiscTypeIdentification;
    UCHAR Reserved2;
    UCHAR MediaSpecificWriteInhibitInformation;
} DVD_RAM_MEDIUM_STATUS, *PDVD_RAM_MEDIUM_STATUS;
C_ASSERT(sizeof(DVD_RAM_MEDIUM_STATUS) == 4);

// format 0x0A - not defined in enum
typedef struct _DVD_RAM_SPARE_AREA_INFORMATION {
    UCHAR FreePrimarySpareSectors[4];
    UCHAR FreeSupplementalSpareSectors[4];
    UCHAR AllocatedSupplementalSpareSectors[4];
} DVD_RAM_SPARE_AREA_INFORMATION, *PDVD_RAM_SPARE_AREA_INFORMATION;
C_ASSERT(sizeof(DVD_RAM_SPARE_AREA_INFORMATION) == 12);

// format 0x0B - not defined in enum
typedef struct _DVD_RAM_RECORDING_TYPE {
    UCHAR Reserved0 : 4;
    UCHAR RealTimeData : 1;
    UCHAR Reserved1 : 3;
    UCHAR Reserved2[3];
} DVD_RAM_RECORDING_TYPE, *PDVD_RAM_RECORDING_TYPE;
C_ASSERT(sizeof(DVD_RAM_RECORDING_TYPE) == 4);

// format 0x0C (RMD in last border-out) is unstructured in public spec
// format 0x0D - not defined in enum
typedef struct _DVD_RECORDING_MANAGEMENT_AREA_DATA {
    UCHAR LastRecordedRMASectorNumber[4];
#if !defined(__midl)
    UCHAR RMDBytes[0];
#endif
} DVD_RECORDING_MANAGEMENT_AREA_DATA, *PDVD_RECORDING_MANAGEMENT_AREA_DATA;
C_ASSERT(sizeof(DVD_RECORDING_MANAGEMENT_AREA_DATA) == 4);

// format 0x0E - not define in enum
typedef struct _DVD_PRERECORDED_INFORMATION {
    UCHAR FieldID_1;
    UCHAR DiscApplicationCode;
    UCHAR DiscPhysicalCode;
    UCHAR LastAddressOfDataRecordableArea[3];
    UCHAR ExtensionCode : 4; // -R for general/authoring v2.0
    UCHAR PartVers1on   : 4; // -R for general/authoring v2.0
    UCHAR Reserved0;
    UCHAR FieldID_2;
    UCHAR OpcSuggestedCode;
    UCHAR WavelengthCode;
    UCHAR WriteStrategyCode[4];
    UCHAR Reserved2;
    UCHAR FieldID_3;
    UCHAR ManufacturerId_3[6];
    UCHAR Reserved3;
    UCHAR FieldID_4;
    UCHAR ManufacturerId_4[6];
    UCHAR Reserved4;
    UCHAR FieldID_5;
    UCHAR ManufacturerId_5[6];
    UCHAR Reserved5;
    UCHAR Reserved99[24];
} DVD_PRERECORDED_INFORMATION, *PDVD_PRERECORDED_INFORMATION;
C_ASSERT(FIELD_OFFSET(DVD_PRERECORDED_INFORMATION, FieldID_2) ==  8);
C_ASSERT(FIELD_OFFSET(DVD_PRERECORDED_INFORMATION, FieldID_3) == 16);
C_ASSERT(FIELD_OFFSET(DVD_PRERECORDED_INFORMATION, FieldID_4) == 24);
C_ASSERT(FIELD_OFFSET(DVD_PRERECORDED_INFORMATION, FieldID_5) == 32);
C_ASSERT(sizeof(DVD_PRERECORDED_INFORMATION) == 64);

// format 0x0F - not defined in enum
typedef struct _DVD_UNIQUE_DISC_IDENTIFIER {
    UCHAR Reserved0[2];
    UCHAR RandomNumber[2];
    UCHAR Year[4];   // ASCII?
    UCHAR Month[2];  // ASCII?
    UCHAR Day[2];    // ASCII?
    UCHAR Hour[2];   // ASCII?
    UCHAR Minute[2]; // ASCII?
    UCHAR Second[2]; // ASCII?
} DVD_UNIQUE_DISC_IDENTIFIER, *PDVD_UNIQUE_DISC_IDENTIFIER;
C_ASSERT(sizeof(DVD_UNIQUE_DISC_IDENTIFIER) == 18);

// format 0x10 - not define in enum - use DVD_LAYER_DESCRIPTOR structure above
// format 0x11 (ADIP information) is unstructured in public spec
// formats 0x12, 0x15 are is unstructured in public spec
// formats 0x13, 0x14, 0x16 through 0x18 are not yet defined

// format 0x19 - not defined in enum
typedef struct _HD_DVD_R_MEDIUM_STATUS {
    UCHAR ExtendedTestZone : 1;
    UCHAR Reserved1 : 7;
    UCHAR NumberOfRemainingRMDsInRDZ;
    UCHAR NumberOfRemainingRMDsInCurrentRMZ[2];
} HD_DVD_R_MEDIUM_STATUS, *PHD_DVD_R_MEDIUM_STATUS;
C_ASSERT(sizeof(HD_DVD_R_MEDIUM_STATUS) == 4);

// format 0x1A (HD DVD-R - Last recorded RMD in the latest R) is unstructured in public spec
// formats 0x1B through 0x1F are not yet defined

// format 0x20 - not define in enum
typedef struct _DVD_DUAL_LAYER_RECORDING_INFORMATION {
    UCHAR Reserved0 : 7;
    UCHAR Layer0SectorsImmutable : 1;
    UCHAR Reserved1[3];
    UCHAR Layer0Sectors[4];
} DVD_DUAL_LAYER_RECORDING_INFORMATION, *PDVD_DUAL_LAYER_RECORDING_INFORMATION;

// format 0x21 - not define in enum
typedef struct _DVD_DUAL_LAYER_MIDDLE_ZONE_START_ADDRESS {
    UCHAR Reserved0 : 7;
    UCHAR InitStatus : 1;
    UCHAR Reserved1[3];
    UCHAR ShiftedMiddleAreaStartAddress[4];
} DVD_DUAL_LAYER_MIDDLE_ZONE_START_ADDRESS, *PDVD_DUAL_LAYER_MIDDLE_ZONE_START_ADDRESS;

// format 0x22 - not define in enum
typedef struct _DVD_DUAL_LAYER_JUMP_INTERVAL_SIZE {
    UCHAR Reserved1[4];
    UCHAR JumpIntervalSize[4];
} DVD_DUAL_LAYER_JUMP_INTERVAL_SIZE, *PDVD_DUAL_LAYER_JUMP_INTERVAL_SIZE;

// format 0x23 - not define in enum
typedef struct _DVD_DUAL_LAYER_MANUAL_LAYER_JUMP {
    UCHAR Reserved1[4];
    UCHAR ManualJumpLayerAddress[4];
} DVD_DUAL_LAYER_MANUAL_LAYER_JUMP, *PDVD_DUAL_LAYER_MANUAL_LAYER_JUMP;

// format 0x24 - not define in enum
typedef struct _DVD_DUAL_LAYER_REMAPPING_INFORMATION {
    UCHAR Reserved1[4];
    UCHAR RemappingAddress[4];
} DVD_DUAL_LAYER_REMAPPING_INFORMATION, *PDVD_DUAL_LAYER_REMAPPING_INFORMATION;

// formats 0x25 through 0x2F are not yet defined

// format 0x30 - not defined in enum (common header)
typedef struct _DVD_DISC_CONTROL_BLOCK_HEADER {
    UCHAR ContentDescriptor[4];
    union {
        struct {
            UCHAR ReservedDoNotUse_UseAsByteInstead_0[3];
            UCHAR RecordingWithinTheUserDataArea      : 1;
            UCHAR ReadingDiscControlBlocks            : 1;
            UCHAR FormattingTheMedium                 : 1;
            UCHAR ModificationOfThisDiscControlBlock  : 1;
            UCHAR ReservedDoNotUse_UseAsByteInstead_1 : 4;
        };
        UCHAR AsByte[4];
    } ProhibitedActions;
    UCHAR VendorId[32]; // actually "non-specified" data
    // UCHAR DCBData[32728];
} DVD_DISC_CONTROL_BLOCK_HEADER, *PDVD_DISC_CONTROL_BLOCK_HEADER;
C_ASSERT(sizeof(DVD_DISC_CONTROL_BLOCK_HEADER) == 40);

// publicly defined DCB types
typedef enum _DISC_CONTROL_BLOCK_TYPE {
    FormattingDiscControlBlock   = 0x46444300, // 'FDC\0'
    WriteInhibitDiscControlBlock = 0x57444300, // 'WDC\0'
    SessionInfoDiscControlBlock  = 0x53444300, // 'SDC\0'
    DiscControlBlockList         = 0xFFFFFFFF
} DISC_CONTROL_BLOCK_TYPE, *PDISC_CONTROL_BLOCK_TYPE;

// format 0x30 - not defined in enum -- Format DCB, not in MMC.

// format 0x30 - not defined in enum -- Write Inhibit DCB
typedef struct _DVD_DISC_CONTROL_BLOCK_WRITE_INHIBIT {
    DVD_DISC_CONTROL_BLOCK_HEADER header;
    UCHAR UpdateCount[4];
    union {
        struct {
            UCHAR ReservedDoNotUse_UseAsByteInstead_0[3];
            UCHAR WriteProtectStatus : 2;
            UCHAR ReservedDoNotUse_UseAsByteInstead_1 : 5;
            UCHAR UpdateRequiresPassword : 1;
        };
        UCHAR AsByte[4];
    } WriteProtectActions;
    UCHAR Reserved0[16];
    UCHAR UpdatePassword[32];
    UCHAR Reserved1[32672];
} DVD_DISC_CONTROL_BLOCK_WRITE_INHIBIT, *PDVD_DISC_CONTROL_BLOCK_WRITE_INHIBIT;
C_ASSERT(sizeof(DVD_DISC_CONTROL_BLOCK_WRITE_INHIBIT) == (16*2048));

// format 0x30 - not defined in enum - Session DCB
typedef struct _DVD_DISC_CONTROL_BLOCK_SESSION_ITEM {
    UCHAR AsByte[16]; // not publicly defined?
} DVD_DISC_CONTROL_BLOCK_SESSION_ITEM, *PDVD_DISC_CONTROL_BLOCK_SESSION_ITEM;
typedef struct _DVD_DISC_CONTROL_BLOCK_SESSION {
    DVD_DISC_CONTROL_BLOCK_HEADER header;
    UCHAR SessionNumber[2];
    UCHAR Reserved0[22];
    UCHAR DiscID[32];
    UCHAR Reserved1[32];
    DVD_DISC_CONTROL_BLOCK_SESSION_ITEM SessionItem[504];
    UCHAR Reserved2[24576]; // 3 Repetitions of bytes 0 through 8191
} DVD_DISC_CONTROL_BLOCK_SESSION, *PDVD_DISC_CONTROL_BLOCK_SESSION;
C_ASSERT(sizeof(DVD_DISC_CONTROL_BLOCK_SESSION) == ((8*1024) * 4));

// format 0x30 - not defined in enum - DCB list
typedef struct _DVD_DISC_CONTROL_BLOCK_LIST_DCB {
    UCHAR DcbIdentifier[4];
} DVD_DISC_CONTROL_BLOCK_LIST_DCB, *PDVD_DISC_CONTROL_BLOCK_LIST_DCB;
typedef struct _DVD_DISC_CONTROL_BLOCK_LIST {
    DVD_DISC_CONTROL_BLOCK_HEADER header;
    UCHAR Reserved0;
    UCHAR ReadabldDCBs;
    UCHAR Reserved1;
    UCHAR WritableDCBs;
#if !defined(__midl)
    DVD_DISC_CONTROL_BLOCK_LIST_DCB Dcbs[0];
#endif
} DVD_DISC_CONTROL_BLOCK_LIST, *PDVD_DISC_CONTROL_BLOCK_LIST;
C_ASSERT(sizeof(DVD_DISC_CONTROL_BLOCK_LIST) == 44);

// format 0x31 (MTA ECC Block) is unstructured in public spec
// formats 0x32 through 0xBF are not yet defined

// format 0xC0 - not defined in enum
typedef struct _DVD_WRITE_PROTECTION_STATUS {
    UCHAR SoftwareWriteProtectUntilPowerdown : 1;
    UCHAR MediaPersistentWriteProtect        : 1;
    UCHAR CartridgeWriteProtect              : 1;
    UCHAR MediaSpecificWriteProtect          : 1;
    UCHAR Reserved0                          : 4;
    UCHAR Reserved1[3];
} DVD_WRITE_PROTECTION_STATUS, *PDVD_WRITE_PROTECTION_STATUS;
C_ASSERT(sizeof(DVD_WRITE_PROTECTION_STATUS) == 4);

// formats 0xC1 through 0x7F are not yet defined
// format 0x80 (AACS volume identifier) is unstructured in public spec
// format 0x81 (Pre-Recorded AACS media serial number) is unstructured in public spec
// format 0x82 (AACS media identifier) is unstructured in public spec
// format 0x83 (AACS media key block) is unstructured in public spec
// formats 0x84 through 0x8F are not yet defined

// format 0x90 - not defined in enum
typedef struct _DVD_LIST_OF_RECOGNIZED_FORMAT_LAYERS_TYPE_CODE {
    UCHAR TypeCodeOfFormatLayer[2];
} DVD_LIST_OF_RECOGNIZED_FORMAT_LAYERS, *PDVD_LIST_OF_RECOGNIZED_FORMAT_LAYERS;
typedef struct _DVD_LIST_OF_RECOGNIZED_FORMAT_LAYERS {
    UCHAR NumberOfRecognizedFormatLayers;
    UCHAR OnlineFormatlayer     : 2;
    UCHAR Reserved1             : 2;
    UCHAR DefaultFormatLayer    : 2;
    UCHAR Reserved2             : 2;
    // DVD_LIST_OF_RECOGNIZED_FORMAT_LAYERS_TYPE_CODE TypeCodes[0];
} DVD_LIST_OF_RECOGNIZED_FORMAT_LAYERS_TYPE_CODE, *PDVD_LIST_OF_RECOGNIZED_FORMAT_LAYERS_TYPE_CODE;
C_ASSERT(sizeof(DVD_LIST_OF_RECOGNIZED_FORMAT_LAYERS_TYPE_CODE) == 2);

// formats 0x91 through 0xFE are not yet defined

// format 0xFF - not defined in enum
typedef struct _DVD_STRUCTURE_LIST_ENTRY {
    UCHAR FormatCode;
    UCHAR Reserved0 : 6;
    UCHAR Readable  : 1;
    UCHAR Sendable  : 1;
    UCHAR FormatLength[2];
} DVD_STRUCTURE_LIST_ENTRY, *PDVD_STRUCTURE_LIST_ENTRY;
C_ASSERT(sizeof(DVD_STRUCTURE_LIST_ENTRY) == 4);

// BD Disc Structures

// format 0x00 (BD Disc Information) is unstructured in public spec
// format 0x08 (BD Disc Definition Structure) is unstructured in public spec
// format 0x09 (BD Cartridge Status) is identical to DVD_RAM_MEDIUM_STATUS but
// only CartridgeWriteProtect, CartridgeNotSealed and MediaInCartridge are
// valid. Other fields are reserved.

// format 0x09 (BD Spare Area Information) - not defined in enum
typedef struct _DVD_BD_SPARE_AREA_INFORMATION {
    UCHAR Reserved1[4];
    UCHAR NumberOfFreeSpareBlocks[4];
    UCHAR NumberOfAllocatedSpareBlocks[4];
} DVD_BD_SPARE_AREA_INFORMATION, *PDVD_BD_SPARE_AREA_INFORMATION;
C_ASSERT(sizeof(DVD_BD_SPARE_AREA_INFORMATION) == 12);

// format 0x12 (BD Raw Defect List). DFL is not fully defined in public spec

// format 0x30 (BD Physical Access Control).
typedef struct _BD_PAC_HEADER {
    UCHAR PACId[3];
    UCHAR PACFormatNumber;
    UCHAR PACUpdateCount[4];
    UCHAR UnknownPACRules[4];
    UCHAR UnkownPACEntireDiscFlag;
    UCHAR Reserved1[2];
    UCHAR NumberOfSegments;
    UCHAR Segments[8][32];
    UCHAR Reserved2[112];
} BD_PAC_HEADER, *PBD_PAC_HEADER;
C_ASSERT(sizeof(BD_PAC_HEADER) == 384);

// Primary PAC is unstructured in public spec

// Disc Write Protect PAC
typedef struct _BD_DISC_WRITE_PROTECT_PAC {
    BD_PAC_HEADER Header;
    UCHAR KnownPACEntireDiscFlags;
    UCHAR Reserved1[3];
    UCHAR WriteProtectControlByte;
    UCHAR Reserved2[7];
    UCHAR WriteProtectPassword[32];
} BD_DISC_WRITE_PROTECT_PAC, *PBD_DISC_WRITE_PROTECT_PAC;
C_ASSERT(sizeof(BD_DISC_WRITE_PROTECT_PAC) == 428);


typedef struct _DVD_RPC_KEY {
    UCHAR UserResetsAvailable:3;
    UCHAR ManufacturerResetsAvailable:3;
    UCHAR TypeCode:2;
    UCHAR RegionMask;
    UCHAR RpcScheme;
    UCHAR Reserved02;
} DVD_RPC_KEY, * PDVD_RPC_KEY;
C_ASSERT(sizeof(DVD_RPC_KEY) == 4);

typedef struct _DVD_SET_RPC_KEY {
    UCHAR PreferredDriveRegionCode;
    UCHAR Reserved[3];
} DVD_SET_RPC_KEY, * PDVD_SET_RPC_KEY;
C_ASSERT(sizeof(DVD_SET_RPC_KEY) == 4);

typedef struct _DVD_ASF { // Authentication Success Flag
    UCHAR Reserved0[3];
    UCHAR SuccessFlag:1;
    UCHAR Reserved1:7;
} DVD_ASF, * PDVD_ASF;
C_ASSERT(sizeof(DVD_ASF) == 4);

typedef struct _DVD_REGION {
     UCHAR CopySystem;
     UCHAR RegionData;                      // current media region (not playable when set)
     UCHAR SystemRegion;                    // current drive region (playable when set)
     UCHAR ResetCount;                      // number of resets available
} DVD_REGION, *PDVD_REGION;
C_ASSERT(sizeof(DVD_REGION) == 4);

// ALL THE ABOVE STRUCTURES ARE BYTE-PACKED:
/////////////////////////////////////////////////////////////
#include <poppack.h>


#ifdef __cplusplus
}
#endif


/////////////////////////////////////////////////////////////
// AACS-related structures
// (mostly opaque data, but useful for allocation)

// The AACS layer number refers to the layer of the disc a structure
// is read from.  This can only be a single byte in the CDB, so limit
// the value to 0..255.
typedef _Field_range_(0,255)       ULONG  AACS_LAYER_NUMBER, *PAACS_LAYER_NUMBER;
typedef _Field_range_(0,255) const ULONG CAACS_LAYER_NUMBER, *PCAACS_LAYER_NUMBER;


// The AACS Certificate (opaque data structure) is used to validate
// the host to the logical unit, as well as to validate the logical
// unit to the host.
typedef struct _AACS_CERTIFICATE {
    UCHAR Nonce[20];
    UCHAR Certificate[92];
} AACS_CERTIFICATE, *PAACS_CERTIFICATE;
typedef const AACS_CERTIFICATE   CAACS_CERTIFICATE;
typedef const AACS_CERTIFICATE *PCAACS_CERTIFICATE;
C_ASSERT(sizeof(AACS_CERTIFICATE) == 112);

// The AACS challenge key (opaque data structure) is used to setup
// a shared bus key for AACS-protected structure transfer.
typedef struct _AACS_CHALLENGE_KEY {
    UCHAR EllipticCurvePoint[40];
    UCHAR Signature[40];
} AACS_CHALLENGE_KEY, *PAACS_CHALLENGE_KEY;
typedef const AACS_CHALLENGE_KEY   CAACS_CHALLENGE_KEY;
typedef const AACS_CHALLENGE_KEY *PCAACS_CHALLENGE_KEY;
C_ASSERT(sizeof(AACS_CHALLENGE_KEY) == 80);

// The VolumeID is one of the unique identifiers on AACS protected media
typedef struct _AACS_VOLUME_ID {
    UCHAR VolumeID[16];
    UCHAR MAC[16]; // MessageAuthenticationCode
} AACS_VOLUME_ID, *PAACS_VOLUME_ID;
typedef const AACS_VOLUME_ID   CAACS_VOLUME_ID;
typedef const AACS_VOLUME_ID *PCAACS_VOLUME_ID;
C_ASSERT(sizeof(AACS_VOLUME_ID) == 32);

// The prerecorded Serial Number is one of the unique identifiers on AACS protected media
typedef struct _AACS_SERIAL_NUMBER {
    UCHAR PrerecordedSerialNumber[16];
    UCHAR MAC[16]; // MessageAuthenticationCode
} AACS_SERIAL_NUMBER, *PAACS_SERIAL_NUMBER;
typedef const AACS_SERIAL_NUMBER   CAACS_SERIAL_NUMBER;
typedef const AACS_SERIAL_NUMBER *PCAACS_SERIAL_NUMBER;
C_ASSERT(sizeof(AACS_SERIAL_NUMBER) == 32);

// The MediaID is one of the unique identifiers on AACS protected media
typedef struct _AACS_MEDIA_ID {
    UCHAR MediaID[16];
    UCHAR MAC[16]; // MessageAuthenticationCode
} AACS_MEDIA_ID, *PAACS_MEDIA_ID;
typedef const AACS_MEDIA_ID   CAACS_MEDIA_ID;
typedef const AACS_MEDIA_ID *PCAACS_MEDIA_ID;
C_ASSERT(sizeof(AACS_MEDIA_ID) == 32);

// When sending a certificate or challenge key, need to wrap
// the data structure with a DVD_SESSION_ID.
typedef struct _AACS_SEND_CERTIFICATE {
    DVD_SESSION_ID SessionId;
    AACS_CERTIFICATE Certificate;
} AACS_SEND_CERTIFICATE, *PAACS_SEND_CERTIFICATE;
typedef const AACS_SEND_CERTIFICATE   CAACS_SEND_CERTIFICATE;
typedef const AACS_SEND_CERTIFICATE *PCAACS_SEND_CERTIFICATE;

// When sending a certificate or challenge key, need to wrap
// the data structure with a DVD_SESSION_ID.
typedef struct _AACS_SEND_CHALLENGE_KEY {
    DVD_SESSION_ID SessionId;
    AACS_CHALLENGE_KEY ChallengeKey;
} AACS_SEND_CHALLENGE_KEY, *PAACS_SEND_CHALLENGE_KEY;
typedef const AACS_SEND_CHALLENGE_KEY   CAACS_SEND_CHALLENGE_KEY;
typedef const AACS_SEND_CHALLENGE_KEY *PCAACS_SEND_CHALLENGE_KEY;


// The AACS binding nonce (opaque data structure) is used to
// protect individual content.
typedef struct _AACS_BINDING_NONCE {
    UCHAR BindingNonce[16];
    UCHAR MAC[16]; // MessageAuthenticationCode
} AACS_BINDING_NONCE, *PAACS_BINDING_NONCE;
typedef const AACS_BINDING_NONCE   CAACS_BINDING_NONCE;
typedef const AACS_BINDING_NONCE *PCAACS_BINDING_NONCE;
C_ASSERT(sizeof(AACS_BINDING_NONCE) == 32);


// This structure is sent when reading a binding nonce
// either from the medium or when having the logical unit
// generate a new binding nonce for a set of sectors
// NOTE: This structure must be identically aligned for 32/64 bit builds
//       
typedef struct _AACS_READ_BINDING_NONCE {
    DVD_SESSION_ID        SessionId;
    _Field_range_(0,255) ULONG  NumberOfSectors; // spec only provides one byte
    ULONGLONG             StartLba;

    // 32-bit HANDLE is 32 bits, 64-bit HANDLE is 64 bits
    union {
        HANDLE                Handle;
        ULONGLONG             ForceStructureLengthToMatch64bit;
    };
} AACS_READ_BINDING_NONCE, *PAACS_READ_BINDING_NONCE;

/////////////////////////////////////////////////////////////

#pragma warning(pop)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif  // _NTDDCDVD_

// end_winioctl

