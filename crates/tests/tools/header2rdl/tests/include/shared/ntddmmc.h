/* Copyright (c) Microsoft Corporation. All rights reserved. */

#ifndef __NTDDMMC__
#define __NTDDMMC__

#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifdef __cplusplus
extern "C" {
#endif


#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4200) // array[0] is not a warning for this file
#pragma warning(disable:4214) // bitfield other than int


//
// NOTE: All FEATURE_* structures may be extended.  use of these structures
//       requires verification that the FeatureHeader->AdditionLength field
//       contains AT LEAST enough data to cover the data fields being accessed.
//       This is due to the design, which allows extending the size of the
//       various structures, which will result in these structures sizes
//       being changed over time.
//       A 0-element array is however not declared in the variable size
//       structures. Such array is declared on some structures to preserve
//       legacy, yet it is deprecated. To access variable size structures,
//       as they are always at the end of the fixed size structure, use a sizeof
//       of the declared fixed size structure as an offset.
//       *** Programmers beware! ***
//

//
// NOTE: This is based on MMC 3, extended to MMC 5 rev 3
//       Further revisions will maintain backward compatibility
//       with the non-reserved fields listed here.  If you need
//       to access a new field, please typecast to FEATURE_DATA_RESERVED
//       and access the appropriate bits there.
//

//
// NOTE: The definition of FEATURE_DATA_REMOVABLE_MEDIUM is based on MMC 6 rev 2.
//       MMC 5 rev 4, section 5.3.4, specified bit 4 after feature header as reserved.
//       MMC 6 rev 2, section 5.3.4, specifies this bit as the Load bit.
//

typedef struct _GET_CONFIGURATION_HEADER {
    UCHAR DataLength[4];      // [0] == MSB, [3] == LSB
    UCHAR Reserved[2];
    UCHAR CurrentProfile[2];  // [0] == MSB, [1] == LSB
#if !defined(__midl)
    UCHAR Data[0];            // extra data, typically FEATURE_HEADER
#endif
} GET_CONFIGURATION_HEADER, *PGET_CONFIGURATION_HEADER;

typedef struct _FEATURE_HEADER {
    UCHAR FeatureCode[2];     // [0] == MSB, [1] == LSB
    UCHAR Current    : 1;     // The feature is currently active
    UCHAR Persistent : 1;     // The feature is always current
    UCHAR Version    : 4;
    UCHAR Reserved0  : 2;
    UCHAR AdditionalLength;   // sizeof(Header) + AdditionalLength = size
} FEATURE_HEADER, *PFEATURE_HEADER;

typedef enum _FEATURE_PROFILE_TYPE {
    ProfileInvalid                  = 0x0000,
    ProfileNonRemovableDisk         = 0x0001,
    ProfileRemovableDisk            = 0x0002,
    ProfileMOErasable               = 0x0003,
    ProfileMOWriteOnce              = 0x0004,
    ProfileAS_MO                    = 0x0005,
    // Reserved                 0x0006 - 0x0007
    ProfileCdrom                    = 0x0008,
    ProfileCdRecordable             = 0x0009,
    ProfileCdRewritable             = 0x000a,
    // Reserved                 0x000b - 0x000f
    ProfileDvdRom                   = 0x0010,
    ProfileDvdRecordable            = 0x0011,
    ProfileDvdRam                   = 0x0012,
    ProfileDvdRewritable            = 0x0013,  // restricted overwrite
    ProfileDvdRWSequential          = 0x0014,
    ProfileDvdDashRDualLayer        = 0x0015,
    ProfileDvdDashRLayerJump        = 0x0016,
    // Reserved                 0x0017 - 0x0019
    ProfileDvdPlusRW                = 0x001A,
    ProfileDvdPlusR                 = 0x001B,
    // Reserved                 0x001C - 001F
    ProfileDDCdrom                  = 0x0020,  // obsolete
    ProfileDDCdRecordable           = 0x0021,  // obsolete
    ProfileDDCdRewritable           = 0x0022,  // obsolete
    // Reserved                 0x0023 - 0x0029
    ProfileDvdPlusRWDualLayer       = 0x002A,
    ProfileDvdPlusRDualLayer        = 0x002B,
    // Reserved                 0x002C - 0x003F
    ProfileBDRom                    = 0x0040,
    ProfileBDRSequentialWritable    = 0x0041,  // BD-R 'SRM'
    ProfileBDRRandomWritable        = 0x0042,  // BD-R 'RRM'
    ProfileBDRewritable             = 0x0043,
    //  Reserved                0x0044 - 0x004F
    ProfileHDDVDRom                 = 0x0050,
    ProfileHDDVDRecordable          = 0x0051,
    ProfileHDDVDRam                 = 0x0052,
    ProfileHDDVDRewritable          = 0x0053,
	// Reserved                 0x0054 - 0x0057
    ProfileHDDVDRDualLayer          = 0x0058,
    // Reserved                 0x0059 - 0x0059
    ProfileHDDVDRWDualLayer         = 0x005A,
    // Reserved                 0x005B - 0xfffe
    ProfileNonStandard              = 0xffff
} FEATURE_PROFILE_TYPE, *PFEATURE_PROFILE_TYPE;

typedef enum _FEATURE_NUMBER {
    FeatureProfileList              = 0x0000,
    FeatureCore                     = 0x0001,
    FeatureMorphing                 = 0x0002,
    FeatureRemovableMedium          = 0x0003,
    FeatureWriteProtect             = 0x0004,
    // Reserved                  0x0005 - 0x000f
    FeatureRandomReadable           = 0x0010,
    // Reserved                  0x0011 - 0x001c
    FeatureMultiRead                = 0x001D,
    FeatureCdRead                   = 0x001E,
    FeatureDvdRead                  = 0x001F,
    FeatureRandomWritable           = 0x0020,
    FeatureIncrementalStreamingWritable = 0x0021,
    FeatureSectorErasable           = 0x0022,
    FeatureFormattable              = 0x0023,
    FeatureDefectManagement         = 0x0024,
    FeatureWriteOnce                = 0x0025,
    FeatureRestrictedOverwrite      = 0x0026,
    FeatureCdrwCAVWrite             = 0x0027,
    FeatureMrw                      = 0x0028,
    FeatureEnhancedDefectReporting  = 0x0029,
    FeatureDvdPlusRW                = 0x002A,
    FeatureDvdPlusR                 = 0x002B,
    FeatureRigidRestrictedOverwrite = 0x002C,
    FeatureCdTrackAtOnce            = 0x002D,
    FeatureCdMastering              = 0x002E,
    FeatureDvdRecordableWrite       = 0x002F,   // both -R and -RW
    FeatureDDCDRead                 = 0x0030,   // obsolete
    FeatureDDCDRWrite               = 0x0031,   // obsolete
    FeatureDDCDRWWrite              = 0x0032,   // obsolete
    FeatureLayerJumpRecording       = 0x0033,
    // Reserved                  0x0034 - 0x0036
    FeatureCDRWMediaWriteSupport    = 0x0037,
    FeatureBDRPseudoOverwrite       = 0x0038,
    // Reserved                       0x0039
    FeatureDvdPlusRWDualLayer       = 0x003A,
    FeatureDvdPlusRDualLayer        = 0x003B,
    // Reserved                  0x003c - 0x003f
    FeatureBDRead                   = 0x0040,
    FeatureBDWrite                  = 0x0041,
    FeatureTSR                      = 0x0042,
    // Reserved                  0x0043 - 0x004f
    FeatureHDDVDRead                = 0x0050,
    FeatureHDDVDWrite               = 0x0051,
    // Reserved                  0x0052 - 0x007f
    FeatureHybridDisc               = 0x0080,
    // Reserved                  0x0081 - 0x00ff
    FeaturePowerManagement          = 0x0100,
    FeatureSMART                    = 0x0101,
    FeatureEmbeddedChanger          = 0x0102,
    FeatureCDAudioAnalogPlay        = 0x0103,  // obsolete
    FeatureMicrocodeUpgrade         = 0x0104,
    FeatureTimeout                  = 0x0105,
    FeatureDvdCSS                   = 0x0106,
    FeatureRealTimeStreaming        = 0x0107,
    FeatureLogicalUnitSerialNumber  = 0x0108,
    FeatureMediaSerialNumber        = 0x0109,
    FeatureDiscControlBlocks        = 0x010A,
    FeatureDvdCPRM                  = 0x010B,
    FeatureFirmwareDate             = 0x010C,
    FeatureAACS                     = 0x010D,
    // Reserved                  0x010e - 0x010f
    FeatureVCPS                     = 0x0110,
    // Reserved                  0x0111 - 0xfeff
    // Vendor Unique             0xff00 - 0xffff
} FEATURE_NUMBER, *PFEATURE_NUMBER;

// 0x0000 - FeatureProfileList
// an integral multiple of the _EX structures are returned for page 0000
typedef struct _FEATURE_DATA_PROFILE_LIST_EX {
    UCHAR ProfileNumber[2]; // [0] == MSB, [1] == LSB
    UCHAR Current                   : 1;
    UCHAR Reserved1                 : 7;
    UCHAR Reserved2;
} FEATURE_DATA_PROFILE_LIST_EX, *PFEATURE_DATA_PROFILE_LIST_EX;

typedef struct _FEATURE_DATA_PROFILE_LIST {
    FEATURE_HEADER Header;
#if !defined(__midl)
    FEATURE_DATA_PROFILE_LIST_EX Profiles[0];
#endif
} FEATURE_DATA_PROFILE_LIST, *PFEATURE_DATA_PROFILE_LIST;

// 0x0001 - FeatureCore
typedef struct _FEATURE_DATA_CORE {
    FEATURE_HEADER Header;
    UCHAR PhysicalInterface[4];  // [0] == MSB, [3] == LSB
    UCHAR DeviceBusyEvent           : 1;
    UCHAR INQUIRY2                  : 1;
    UCHAR Reserved1                 : 6;
    UCHAR Reserved2[3];
} FEATURE_DATA_CORE, *PFEATURE_DATA_CORE;

// 0x0002 - FeatureMorphing
typedef struct _FEATURE_DATA_MORPHING {
    FEATURE_HEADER Header;
    UCHAR Asynchronous              : 1;
    UCHAR OCEvent                   : 1;
    UCHAR Reserved01                : 6;
    UCHAR Reserved2[3];
} FEATURE_DATA_MORPHING, *PFEATURE_DATA_MORPHING;

// 0x0003 - FeatureRemovableMedium
typedef struct _FEATURE_DATA_REMOVABLE_MEDIUM {
    FEATURE_HEADER Header;
    UCHAR Lockable                  : 1;
    UCHAR DBML                      : 1; // MMC 6 rev 2g
    UCHAR DefaultToPrevent          : 1;
    UCHAR Eject                     : 1;
    UCHAR Load                      : 1; // MMC 6 rev 2
    UCHAR LoadingMechanism          : 3;
    UCHAR Reserved3[3];
} FEATURE_DATA_REMOVABLE_MEDIUM, *PFEATURE_DATA_REMOVABLE_MEDIUM;

// 0x0004 - FeatureWriteProtect
typedef struct _FEATURE_DATA_WRITE_PROTECT {
    FEATURE_HEADER Header;
    UCHAR SupportsSWPPBit                : 1;
    UCHAR SupportsPersistentWriteProtect : 1;
    UCHAR WriteInhibitDCB                : 1;
    UCHAR DiscWriteProtectPAC            : 1;
    UCHAR Reserved01                     : 4;
    UCHAR Reserved2[3];
} FEATURE_DATA_WRITE_PROTECT, *PFEATURE_DATA_WRITE_PROTECT;

// 0x0005 - 0x000f are Reserved

// 0x0010 - FeatureRandomReadable
typedef struct _FEATURE_DATA_RANDOM_READABLE {
    FEATURE_HEADER Header;
    UCHAR LogicalBlockSize[4];
    UCHAR Blocking[2];
    UCHAR ErrorRecoveryPagePresent : 1;
    UCHAR Reserved1                : 7;
    UCHAR Reserved2;
} FEATURE_DATA_RANDOM_READABLE, *PFEATURE_DATA_RANDOM_READABLE;

// 0x0011 - 0x001c are Reserved

// 0x001D - FeatureMultiRead
typedef struct _FEATURE_DATA_MULTI_READ {
    FEATURE_HEADER Header;
} FEATURE_DATA_MULTI_READ, *PFEATURE_DATA_MULTI_READ;

// 0x001E - FeatureCdRead
typedef struct _FEATURE_DATA_CD_READ {
    FEATURE_HEADER Header;
    UCHAR CDText                   : 1;
    UCHAR C2ErrorData              : 1;
    UCHAR Reserved01               : 5;
    UCHAR DigitalAudioPlay         : 1;
    UCHAR Reserved2[3];
} FEATURE_DATA_CD_READ, *PFEATURE_DATA_CD_READ;

// 0x001F - FeatureDvdRead
typedef struct _FEATURE_DATA_DVD_READ {
    FEATURE_HEADER Header;
    UCHAR Multi110                 : 1;
    UCHAR Reserved1                : 7;
    UCHAR Reserved2;
    UCHAR DualDashR                : 1;
    UCHAR Reserved3                : 7;
    UCHAR Reserved4;
} FEATURE_DATA_DVD_READ, *PFEATURE_DATA_DVD_READ;

// 0x0020 - FeatureRandomWritable
typedef struct _FEATURE_DATA_RANDOM_WRITABLE {
    FEATURE_HEADER Header;
    UCHAR LastLBA[4];
    UCHAR LogicalBlockSize[4];
    UCHAR Blocking[2];
    UCHAR ErrorRecoveryPagePresent : 1;
    UCHAR Reserved1                : 7;
    UCHAR Reserved2;
} FEATURE_DATA_RANDOM_WRITABLE, *PFEATURE_DATA_RANDOM_WRITABLE;

// 0x0021 - FeatureIncrementalStreamingWritable
typedef struct _FEATURE_DATA_INCREMENTAL_STREAMING_WRITABLE {
    FEATURE_HEADER Header;
    UCHAR DataTypeSupported[2];   // [0] == MSB, [1] == LSB // see also FeatureCdTrackAtOnce
    UCHAR BufferUnderrunFree            : 1;
    UCHAR AddressModeReservation        : 1;
    UCHAR TrackRessourceInformation     : 1;
    UCHAR Reserved01                    : 5;
    UCHAR NumberOfLinkSizes;
#if !defined(__midl)
    UCHAR LinkSize[0];
#endif
} FEATURE_DATA_INCREMENTAL_STREAMING_WRITABLE, *PFEATURE_DATA_INCREMENTAL_STREAMING_WRITABLE;

// 0x0022 - FeatureSectorErasable
typedef struct _FEATURE_DATA_SECTOR_ERASABLE {
    FEATURE_HEADER Header;
} FEATURE_DATA_SECTOR_ERASABLE, *PFEATURE_DATA_SECTOR_ERASABLE;

// 0x0023 - FeatureFormattable
typedef struct _FEATURE_DATA_FORMATTABLE {
    FEATURE_HEADER Header;
    UCHAR FullCertification     : 1;
    UCHAR QuickCertification    : 1;
    UCHAR SpareAreaExpansion    : 1;
    UCHAR RENoSpareAllocated    : 1;
    UCHAR Reserved1             : 4;
    UCHAR Reserved2[3];
    UCHAR RRandomWritable       : 1;
    UCHAR Reserved3             : 7;
    UCHAR Reserved4[3];
} FEATURE_DATA_FORMATTABLE, *PFEATURE_DATA_FORMATTABLE;

// 0x0024 - FeatureDefectManagement
typedef struct _FEATURE_DATA_DEFECT_MANAGEMENT {
    FEATURE_HEADER Header;
    UCHAR Reserved1             : 7;
    UCHAR SupplimentalSpareArea : 1;
    UCHAR Reserved2[3];
} FEATURE_DATA_DEFECT_MANAGEMENT, *PFEATURE_DATA_DEFECT_MANAGEMENT;

// 0x0025 - FeatureWriteOnce
typedef struct _FEATURE_DATA_WRITE_ONCE {
    FEATURE_HEADER Header;
    UCHAR LogicalBlockSize[4];
    UCHAR Blocking[2];
    UCHAR ErrorRecoveryPagePresent : 1;
    UCHAR Reserved1                : 7;
    UCHAR Reserved2;
} FEATURE_DATA_WRITE_ONCE, *PFEATURE_DATA_WRITE_ONCE;

// 0x0026 - FeatureRestrictedOverwrite
typedef struct _FEATURE_DATA_RESTRICTED_OVERWRITE {
    FEATURE_HEADER Header;
} FEATURE_DATA_RESTRICTED_OVERWRITE, *PFEATURE_DATA_RESTRICTED_OVERWRITE;

// 0x0027 - FeatureCdrwCAVWrite
typedef struct _FEATURE_DATA_CDRW_CAV_WRITE {
    FEATURE_HEADER Header;
    UCHAR Reserved1[4];
    /*
    UCHAR SupportsWritingCDRWSubType0 : 1; // 1x - 4x media
    UCHAR SupportsWritingCDRWSubType1 : 1; // 4x - 10x media ??
    UCHAR SupportsWritingCDRWSubType2 : 1; // ???
    UCHAR SupportsWritingCDRWSubType3 : 1; // ???
    UCHAR SupportsWritingCDRWSubType4 : 1; // ???
    UCHAR SupportsWritingCDRWSubType5 : 1; // ???
    UCHAR SupportsWritingCDRWSubType6 : 1; // ???
    UCHAR SupportsWritingCDRWSubType7 : 1; // ???
    UCHAR Reserved2[3];
    */
} FEATURE_DATA_CDRW_CAV_WRITE, *PFEATURE_DATA_CDRW_CAV_WRITE;

// 0x0028 - FeatureMrw
typedef struct _FEATURE_DATA_MRW {
    FEATURE_HEADER Header;
    UCHAR Write         : 1; // Cd Write
    UCHAR DvdPlusRead   : 1;
    UCHAR DvdPlusWrite  : 1;
    UCHAR Reserved01    : 5;
    UCHAR Reserved2[3];
} FEATURE_DATA_MRW, *PFEATURE_DATA_MRW;

// 0x0029 - Enhanced Defect Reporting
typedef struct _FEATURE_ENHANCED_DEFECT_REPORTING {
    FEATURE_HEADER Header;
    UCHAR DRTDMSupported : 1;
    UCHAR Reserved0      : 7;
    UCHAR NumberOfDBICacheZones;
    UCHAR NumberOfEntries[2];
} FEATURE_ENHANCED_DEFECT_REPORTING, *PFEATURE_ENHANCED_DEFECT_REPORTING;

// 0x002A - FeatureDvdPlusRW
typedef struct _FEATURE_DATA_DVD_PLUS_RW {
    FEATURE_HEADER Header;
    UCHAR Write         : 1;
    UCHAR Reserved1     : 7;
    UCHAR CloseOnly     : 1;
    UCHAR QuickStart    : 1;
    UCHAR Reserved02    : 6;
    UCHAR Reserved03[2];
} FEATURE_DATA_DVD_PLUS_RW, *PFEATURE_DATA_DVD_PLUS_RW;

// 0x002B - FeatureDvdPlusR
typedef struct _FEATURE_DATA_DVD_PLUS_R {
    FEATURE_HEADER Header;
    UCHAR Write     : 1;
    UCHAR Reserved1 : 7;
    UCHAR Reserved2[3];
} FEATURE_DATA_DVD_PLUS_R, *PFEATURE_DATA_DVD_PLUS_R;

// 0x002C - FeatureDvdRwRestrictedOverwrite
typedef struct _FEATURE_DATA_DVD_RW_RESTRICTED_OVERWRITE {
    FEATURE_HEADER Header;
    UCHAR Blank                    : 1;
    UCHAR Intermediate             : 1;
    UCHAR DefectStatusDataRead     : 1;
    UCHAR DefectStatusDataGenerate : 1;
    UCHAR Reserved0                : 4;
    UCHAR Reserved1[3];
} FEATURE_DATA_DVD_RW_RESTRICTED_OVERWRITE, *PFEATURE_DATA_DVD_RW_RESTRICTED_OVERWRITE;

// 0x002D - FeatureCdTrackAtOnce
typedef struct _FEATURE_DATA_CD_TRACK_AT_ONCE {
    FEATURE_HEADER Header;
    UCHAR RWSubchannelsRecordable  : 1;
    UCHAR CdRewritable             : 1;
    UCHAR TestWriteOk              : 1;
    UCHAR RWSubchannelPackedOk     : 1; // MMC 3 +
    UCHAR RWSubchannelRawOk        : 1; // MMC 3 +
    UCHAR Reserved1                : 1;
    UCHAR BufferUnderrunFree       : 1; // MMC 3 +
    UCHAR Reserved3                : 1;
    UCHAR Reserved2;
    UCHAR DataTypeSupported[2];   // [0] == MSB, [1] == LSB // see also FeatureIncrementalStreamingWritable
} FEATURE_DATA_CD_TRACK_AT_ONCE, *PFEATURE_DATA_CD_TRACK_AT_ONCE;

// 0x002E - FeatureCdMastering
typedef struct _FEATURE_DATA_CD_MASTERING {
    FEATURE_HEADER Header;
    UCHAR RWSubchannelsRecordable  : 1;
    UCHAR CdRewritable             : 1;
    UCHAR TestWriteOk              : 1;
    UCHAR RawRecordingOk           : 1;
    UCHAR RawMultiSessionOk        : 1;
    UCHAR SessionAtOnceOk          : 1;
    UCHAR BufferUnderrunFree       : 1;
    UCHAR Reserved1                : 1;
    UCHAR MaximumCueSheetLength[3]; // [0] == MSB, [2] == LSB
} FEATURE_DATA_CD_MASTERING, *PFEATURE_DATA_CD_MASTERING;

// 0x002F - FeatureDvdRecordableWrite
typedef struct _FEATURE_DATA_DVD_RECORDABLE_WRITE {
    FEATURE_HEADER Header;
    UCHAR Reserved1                : 1;
    UCHAR DVD_RW                   : 1;
    UCHAR TestWrite                : 1;
    UCHAR RDualLayer               : 1;
    UCHAR Reserved02               : 2;
    UCHAR BufferUnderrunFree       : 1;
    UCHAR Reserved3                : 1;
    UCHAR Reserved4[3];
} FEATURE_DATA_DVD_RECORDABLE_WRITE, *PFEATURE_DATA_DVD_RECORDABLE_WRITE;

// 0x0030 - FeatureDDCDRead
typedef struct _FEATURE_DATA_DDCD_READ {
    FEATURE_HEADER Header;
} FEATURE_DATA_DDCD_READ, *PFEATURE_DATA_DDCD_READ;

// 0x0031 - FeatureDDCDRWrite (obsolete)
typedef struct _FEATURE_DATA_DDCD_R_WRITE {
    FEATURE_HEADER Header;
    UCHAR Reserved1               : 2;
    UCHAR TestWrite               : 1;
    UCHAR Reserved2               : 5;
    UCHAR Reserved3[3];
} FEATURE_DATA_DDCD_R_WRITE, *PFEATURE_DATA_DDCD_R_WRITE;

// 0x0032 - FeatureDDCDRWWrite (obsolete)
typedef struct _FEATURE_DATA_DDCD_RW_WRITE {
    FEATURE_HEADER Header;
    UCHAR Blank                   : 1;
    UCHAR Intermediate            : 1;
    UCHAR Reserved1               : 6;
    UCHAR Reserved2[3];
} FEATURE_DATA_DDCD_RW_WRITE, *PFEATURE_DATA_DDCD_RW_WRITE;

// 0x0033 - FeatureLayerJumpRecording
typedef struct _FEATURE_DATA_LAYER_JUMP_RECORDING {
    FEATURE_HEADER Header;
    UCHAR Reserved0[3];
    UCHAR NumberOfLinkSizes;
#if !defined(__midl)
    UCHAR LinkSizes[0];
#endif
} FEATURE_DATA_LAYER_JUMP_RECORDING, *PFEATURE_DATA_LAYER_JUMP_RECORDING;

// 0x0034 - 0x0036 are Reserved

// 0x0037 - FeatureCDRWMediaWriteSupport
typedef struct _FEATURE_CD_RW_MEDIA_WRITE_SUPPORT {
    FEATURE_HEADER Header;
    UCHAR Reserved1;
    struct{
        UCHAR Subtype0 :1;
        UCHAR Subtype1 :1;
        UCHAR Subtype2 :1;
        UCHAR Subtype3 :1;
        UCHAR Subtype4 :1;
        UCHAR Subtype5 :1;
        UCHAR Subtype6 :1;
        UCHAR Subtype7 :1;
    } CDRWMediaSubtypeSupport;
    UCHAR Reserved2[2];
} FEATURE_CD_RW_MEDIA_WRITE_SUPPORT, *PFEATURE_CD_RW_MEDIA_WRITE_SUPPORT;

// 0x0038 - FeatureBDRPseudoOverwrite
typedef struct _FEATURE_BD_R_PSEUDO_OVERWRITE {
    FEATURE_HEADER Header;
    UCHAR Reserved[4];
} FEATURE_BD_R_PSEUDO_OVERWRITE, *PFEATURE_BD_R_PSEUDO_OVERWRITE;

// 0x0039 is Reserved

// 0x003A - FeatureDvdPlusRWDualLayer
typedef struct _FEATURE_DATA_DVD_PLUS_RW_DUAL_LAYER {
    FEATURE_HEADER Header;
    UCHAR Write         : 1;
    UCHAR Reserved1     : 7;
    UCHAR CloseOnly     : 1;
    UCHAR QuickStart    : 1;
    UCHAR Reserved2     : 6;
    UCHAR Reserved3[2];
} FEATURE_DATA_DVD_PLUS_RW_DUAL_LAYER, *PFEATURE_DATA_DVD_PLUS_RW_DUAL_LAYER;

// 0x003B - FeatureDvdPlusRDualLayer
typedef struct _FEATURE_DATA_DVD_PLUS_R_DUAL_LAYER {
    FEATURE_HEADER Header;
    UCHAR Write     : 1;
    UCHAR Reserved1 : 7;
    UCHAR Reserved2[3];
} FEATURE_DATA_DVD_PLUS_R_DUAL_LAYER, *PFEATURE_DATA_DVD_PLUS_R_DUAL_LAYER;

// 0x003C - 0x0039 are Reserved

// 0x0040 - FeatureBDRead

typedef struct _BD_CLASS_SUPPORT_BITMAP {
    UCHAR Version8  :1;
    UCHAR Version9  :1;
    UCHAR Version10 :1;
    UCHAR Version11 :1;
    UCHAR Version12 :1;
    UCHAR Version13 :1;
    UCHAR Version14 :1;
    UCHAR Version15 :1;
    UCHAR Version0  :1;
    UCHAR Version1  :1;
    UCHAR Version2  :1;
    UCHAR Version3  :1;
    UCHAR Version4  :1;
    UCHAR Version5  :1;
    UCHAR Version6  :1;
    UCHAR Version7  :1;
} BD_CLASS_SUPPORT_BITMAP, *PBD_CLASS_SUPPORT_BITMAP;

typedef struct _FEATURE_BD_READ {
    FEATURE_HEADER Header;
    UCHAR Reserved[4];
    BD_CLASS_SUPPORT_BITMAP Class0BitmapBDREReadSupport;
    BD_CLASS_SUPPORT_BITMAP Class1BitmapBDREReadSupport;
    BD_CLASS_SUPPORT_BITMAP Class2BitmapBDREReadSupport;
    BD_CLASS_SUPPORT_BITMAP Class3BitmapBDREReadSupport;
    BD_CLASS_SUPPORT_BITMAP Class0BitmapBDRReadSupport;
    BD_CLASS_SUPPORT_BITMAP Class1BitmapBDRReadSupport;
    BD_CLASS_SUPPORT_BITMAP Class2BitmapBDRReadSupport;
    BD_CLASS_SUPPORT_BITMAP Class3BitmapBDRReadSupport;
    BD_CLASS_SUPPORT_BITMAP Class0BitmapBDROMReadSupport;
    BD_CLASS_SUPPORT_BITMAP Class1BitmapBDROMReadSupport;
    BD_CLASS_SUPPORT_BITMAP Class2BitmapBDROMReadSupport;
    BD_CLASS_SUPPORT_BITMAP Class3BitmapBDROMReadSupport;
} FEATURE_BD_READ, *PFEATURE_BD_READ;

// 0x0041 - FeatureBDWrite
typedef struct _FEATURE_BD_WRITE {
    FEATURE_HEADER Header;
    UCHAR SupportsVerifyNotRequired :1;
    UCHAR Reserved1                 :7;
    UCHAR Reserved2[3];
    BD_CLASS_SUPPORT_BITMAP Class0BitmapBDREWriteSupport;
    BD_CLASS_SUPPORT_BITMAP Class1BitmapBDREWriteSupport;
    BD_CLASS_SUPPORT_BITMAP Class2BitmapBDREWriteSupport;
    BD_CLASS_SUPPORT_BITMAP Class3BitmapBDREWriteSupport;
    BD_CLASS_SUPPORT_BITMAP Class0BitmapBDRWriteSupport;
    BD_CLASS_SUPPORT_BITMAP Class1BitmapBDRWriteSupport;
    BD_CLASS_SUPPORT_BITMAP Class2BitmapBDRWriteSupport;
    BD_CLASS_SUPPORT_BITMAP Class3BitmapBDRWriteSupport;
} FEATURE_BD_WRITE, *PFEATURE_BD_WRITE;

// 0x0042 - FeatureTSR
typedef struct _FEATURE_TSR {
    FEATURE_HEADER Header;
} FEATURE_TSR, *PFEATURE_TSR;

// 0x0043 - 0x004F are Reserved

// 0x0050 - FeatureHDDVDRead
typedef struct _FEATURE_DATA_HDDVD_READ {
    FEATURE_HEADER Header;
    UCHAR Recordable : 1;
    UCHAR Reserved0  : 7;
    UCHAR Reserved1;
    UCHAR Rewritable : 1; // Stands for HD DVD-RAM
    UCHAR Reserved2  : 7;
    UCHAR Reserved3;
} FEATURE_DATA_HDDVD_READ, *PFEATURE_DATA_HDDVD_READ;

// 0x0051 - FeatureHDDVDWrite
typedef struct _FEATURE_DATA_HDDVD_WRITE {
    FEATURE_HEADER Header;
    UCHAR Recordable : 1;
    UCHAR Reserved0  : 7;
    UCHAR Reserved1;
    UCHAR Rewritable : 1; // Stands for HD DVD-RAM
    UCHAR Reserved2  : 7;
    UCHAR Reserved3;
} FEATURE_DATA_HDDVD_WRITE, *PFEATURE_DATA_HDDVD_WRITE;

// 0x0052 - 0x007F are Reserved

// 0x0080 - FeatureHybridDisc
typedef struct _FEATURE_HYBRID_DISC {
    FEATURE_HEADER Header;
    UCHAR ResetImmunity : 1;
    UCHAR Reserved1     : 7;
    UCHAR Reserved2[3];
} FEATURE_HYBRID_DISC, *PFEATURE_HYBRID_DISC;

// 0x0081 - 0x00FF are Reserved

// 0x0100 - FeaturePowerManagement
typedef struct _FEATURE_DATA_POWER_MANAGEMENT {
    FEATURE_HEADER Header;
} FEATURE_DATA_POWER_MANAGEMENT, *PFEATURE_DATA_POWER_MANAGEMENT;

// 0x0101 - FeatureSMART (not in MMC 2)
typedef struct _FEATURE_DATA_SMART {
    FEATURE_HEADER Header;
    UCHAR FaultFailureReportingPagePresent : 1;
    UCHAR Reserved1                        : 7;
    UCHAR Reserved02[3];
} FEATURE_DATA_SMART, *PFEATURE_DATA_SMART;

// 0x0102 - FeatureEmbeddedChanger
typedef struct _FEATURE_DATA_EMBEDDED_CHANGER {
    FEATURE_HEADER Header;
    UCHAR Reserved1                : 2;
    UCHAR SupportsDiscPresent      : 1;
    UCHAR Reserved2                : 1;
    UCHAR SideChangeCapable        : 1;
    UCHAR Reserved3                : 3;
    UCHAR Reserved4[2];
    UCHAR HighestSlotNumber        : 5;
    UCHAR Reserved                 : 3;
} FEATURE_DATA_EMBEDDED_CHANGER, *PFEATURE_DATA_EMBEDDED_CHANGER;

// 0x0103 - FeatureCDAudioAnalogPlay (obsolete)
typedef struct _FEATURE_DATA_CD_AUDIO_ANALOG_PLAY {
    FEATURE_HEADER Header;
    UCHAR SeperateVolume           : 1;
    UCHAR SeperateChannelMute      : 1;
    UCHAR ScanSupported            : 1;
    UCHAR Reserved1                : 5;
    UCHAR Reserved2;
    UCHAR NumerOfVolumeLevels[2];  // [0] == MSB, [1] == LSB
} FEATURE_DATA_CD_AUDIO_ANALOG_PLAY, *PFEATURE_DATA_CD_AUDIO_ANALOG_PLAY;

// 0x0104 - FeatureMicrocodeUpgrade
typedef struct _FEATURE_DATA_MICROCODE_UPDATE {
    FEATURE_HEADER Header;
    UCHAR M5        : 1;
    UCHAR Reserved1 : 7;
    UCHAR Reserved2[3];
} FEATURE_DATA_MICROCODE_UPDATE, *PFEATURE_DATA_MICROCODE_UPDATE;

// 0x0105 - FeatureTimeout
typedef struct _FEATURE_DATA_TIMEOUT {
    FEATURE_HEADER Header;
    UCHAR Group3    : 1;
    UCHAR Reserved1 : 7;
    UCHAR Reserved2;
    UCHAR UnitLength[2];
} FEATURE_DATA_TIMEOUT, *PFEATURE_DATA_TIMEOUT;

// 0x0106 - FeatureDvdCSS
typedef struct _FEATURE_DATA_DVD_CSS {
    FEATURE_HEADER Header;
    UCHAR Reserved1[3];
    UCHAR CssVersion;
} FEATURE_DATA_DVD_CSS, *PFEATURE_DATA_DVD_CSS;

// 0x0107 - FeatureRealTimeStreaming
typedef struct _FEATURE_DATA_REAL_TIME_STREAMING {
    FEATURE_HEADER Header;
    UCHAR StreamRecording         : 1;
    UCHAR WriteSpeedInGetPerf     : 1;
    UCHAR WriteSpeedInMP2A        : 1;
    UCHAR SetCDSpeed              : 1;
    UCHAR ReadBufferCapacityBlock : 1;
    UCHAR Reserved1               : 3;
    UCHAR Reserved2[3];
} FEATURE_DATA_REAL_TIME_STREAMING, *PFEATURE_DATA_REAL_TIME_STREAMING;

// 0x0108 - FeatureLogicalUnitSerialNumber
typedef struct _FEATURE_DATA_LOGICAL_UNIT_SERIAL_NUMBER {
    FEATURE_HEADER Header;
#if !defined(__midl)
    UCHAR SerialNumber[0];
#endif
} FEATURE_DATA_LOGICAL_UNIT_SERIAL_NUMBER, *PFEATURE_DATA_LOGICAL_UNIT_SERIAL_NUMBER;

// 0x0109 - FeatureMediaSerialNumber
typedef struct _FEATURE_MEDIA_SERIAL_NUMBER {
    FEATURE_HEADER Header;
} FEATURE_MEDIA_SERIAL_NUMBER, *PFEATURE_MEDIA_SERIAL_NUMBER;

// 0x010A - FeatureDiscControlBlocks
// an integral multiple of the _EX structures are returned for page 010A
typedef struct _FEATURE_DATA_DISC_CONTROL_BLOCKS_EX {
    UCHAR ContentDescriptor[4];
} FEATURE_DATA_DISC_CONTROL_BLOCKS_EX, *PFEATURE_DATA_DISC_CONTROL_BLOCKS_EX;
// use a zero-sized array for this....
typedef struct _FEATURE_DATA_DISC_CONTROL_BLOCKS {
    FEATURE_HEADER Header;
#if !defined(__midl)
    FEATURE_DATA_DISC_CONTROL_BLOCKS_EX Data[0];
#endif
} FEATURE_DATA_DISC_CONTROL_BLOCKS, *PFEATURE_DATA_DISC_CONTROL_BLOCKS;

// 0x010B - FeatureDvdCPRM
typedef struct _FEATURE_DATA_DVD_CPRM {
    FEATURE_HEADER Header;
    UCHAR Reserved0[3];
    UCHAR CPRMVersion;
} FEATURE_DATA_DVD_CPRM, *PFEATURE_DATA_DVD_CPRM;

// 0x010C - FeatureFirmwareDate
typedef struct _FEATURE_DATA_FIRMWARE_DATE {
    FEATURE_HEADER Header;
    UCHAR Year[4];
    UCHAR Month[2];
    UCHAR Day[2];
    UCHAR Hour[2];
    UCHAR Minute[2];
    UCHAR Seconds[2];
    UCHAR Reserved[2];
} FEATURE_DATA_FIRMWARE_DATE, *PFEATURE_DATA_FIRMWARE_DATE;

// 0x010D - FeatureAACS
typedef struct _FEATURE_DATA_AACS {
    FEATURE_HEADER Header;
    UCHAR BindingNonceGeneration : 1;
    UCHAR Reserved0              : 7;
    UCHAR BindingNonceBlockCount;
    UCHAR NumberOfAGIDs  : 4;
    UCHAR Reserved1      : 4;
    UCHAR AACSVersion;
} FEATURE_DATA_AACS, *PFEATURE_DATA_AACS;

// 0x010E - 0x010F are Reserved

// 0x0110 - FeatureVCPS
typedef struct _FEATURE_VCPS {
    FEATURE_HEADER Header;
    UCHAR Reserved[4];
} FEATURE_VCPS, *PFEATURE_VCPS;

// 0x0111 - 0xFEFF are Reserved
typedef struct _FEATURE_DATA_RESERVED {
    FEATURE_HEADER Header;
#if !defined(__midl)
    UCHAR Data[0];
#endif
} FEATURE_DATA_RESERVED, *PFEATURE_DATA_RESERVED;

// 0xff00 - 0xffff are Vendor Specific
typedef struct _FEATURE_DATA_VENDOR_SPECIFIC {
    FEATURE_HEADER Header;
#if !defined(__midl)
    UCHAR VendorSpecificData[0];
#endif
} FEATURE_DATA_VENDOR_SPECIFIC, *PFEATURE_DATA_VENDOR_SPECIFIC;


//
// NOTE: All FEATURE_* structures may be extended.  use of these structures
//       requires verification that the FeatureHeader->AdditionLength field
//       contains AT LEAST enough data to cover the data fields being accessed.
//       This is due to the design, which allows extending the size of the
//       various structures, which will result in these structures sizes
//       being changed over time.
//       A 0-element array is however not declared in the variable size
//       structures. Such array is declared on some structures to preserve
//       legacy, yet it is deprecated. To access variable size structures,
//       as they are always at the end of the fixed size structure, use a sizeof
//       of the declared fixed size structure as an offset.
//       *** Programmers beware! ***
//

//
// NOTE: This is based on MMC 3, extended to MMC 5 rev 3
//       Further revisions will maintain backward compatibility
//       with the non-reserved fields listed here.  If you need
//       to access a new field, please typecast to FEATURE_DATA_RESERVED
//       and access the appropriate bits there.
//

//
// IOCTL_CDROM_GET_CONFIGURATION returns a FEATURE_* struct, which always
//       starts with a FEATURE_HEADER structure.
//

//
// these are to be used for the request type
//

#define SCSI_GET_CONFIGURATION_REQUEST_TYPE_ALL     0x0
#define SCSI_GET_CONFIGURATION_REQUEST_TYPE_CURRENT 0x1
#define SCSI_GET_CONFIGURATION_REQUEST_TYPE_ONE     0x2


typedef struct _GET_CONFIGURATION_IOCTL_INPUT {
    FEATURE_NUMBER Feature;
    ULONG          RequestType; // SCSI_GET_CONFIGURATION_REQUEST_TYPE_*
    PVOID          Reserved[2];
} GET_CONFIGURATION_IOCTL_INPUT, *PGET_CONFIGURATION_IOCTL_INPUT;

#if defined(_WIN64)
typedef struct _GET_CONFIGURATION_IOCTL_INPUT32 {
    FEATURE_NUMBER   Feature;
    ULONG            RequestType; // SCSI_GET_CONFIGURATION_REQUEST_TYPE_*
    VOID* UPOINTER_32 Reserved[2];
} GET_CONFIGURATION_IOCTL_INPUT32, *PGET_CONFIGURATION_IOCTL_INPUT32;
#endif


#if _MSC_VER >= 1200
#pragma warning(pop)          // un-sets any local warning changes
#else
#pragma warning(default:4200) // array[0] is not a warning for this file
#endif

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // __NTDDMMC__

