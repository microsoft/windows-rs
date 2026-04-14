/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ntddtape.h

Abstract:

    This is the include file that defines all constants and types for
    accessing the Tape device.

--*/

#ifndef _NTDDTAPE_
#define _NTDDTAPE_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#ifdef __cplusplus
extern "C" {
#endif

//
// Device Name - this string is the name of the device.  It is the name
// that should be passed to NtOpenFile when accessing the device.
//
// Note:  For devices that support multiple units, it should be suffixed
//        with the Ascii representation of the unit number.
//

#define DD_TAPE_DEVICE_NAME "\\Device\\Tape"


//
// NtDeviceIoControlFile IoControlCode values for this device.
//
// Warning:  Remember that the low two bits of the code specify how the
//           buffers are passed to the driver!
//

#define IOCTL_TAPE_BASE                 FILE_DEVICE_TAPE

#define IOCTL_TAPE_ERASE                CTL_CODE(IOCTL_TAPE_BASE, 0x0000, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_TAPE_PREPARE              CTL_CODE(IOCTL_TAPE_BASE, 0x0001, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_TAPE_WRITE_MARKS          CTL_CODE(IOCTL_TAPE_BASE, 0x0002, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_TAPE_GET_POSITION         CTL_CODE(IOCTL_TAPE_BASE, 0x0003, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_TAPE_SET_POSITION         CTL_CODE(IOCTL_TAPE_BASE, 0x0004, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_TAPE_GET_DRIVE_PARAMS     CTL_CODE(IOCTL_TAPE_BASE, 0x0005, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_TAPE_SET_DRIVE_PARAMS     CTL_CODE(IOCTL_TAPE_BASE, 0x0006, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_TAPE_GET_MEDIA_PARAMS     CTL_CODE(IOCTL_TAPE_BASE, 0x0007, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_TAPE_SET_MEDIA_PARAMS     CTL_CODE(IOCTL_TAPE_BASE, 0x0008, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_TAPE_GET_STATUS           CTL_CODE(IOCTL_TAPE_BASE, 0x0009, METHOD_BUFFERED, FILE_READ_ACCESS )
#define IOCTL_TAPE_CREATE_PARTITION     CTL_CODE(IOCTL_TAPE_BASE, 0x000a, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// The following device control codes are common for all class drivers.  The
// functions codes defined here must match all of the other class drivers.
//
// Warning: these codes will be replaced in the future with the IOCTL_STORAGE
// codes included below
//

#define IOCTL_TAPE_MEDIA_REMOVAL        CTL_CODE(IOCTL_TAPE_BASE, 0x0201, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_TAPE_EJECT_MEDIA          CTL_CODE(IOCTL_TAPE_BASE, 0x0202, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_TAPE_LOAD_MEDIA           CTL_CODE(IOCTL_TAPE_BASE, 0x0203, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_TAPE_RESERVE              CTL_CODE(IOCTL_TAPE_BASE, 0x0204, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_TAPE_RELEASE              CTL_CODE(IOCTL_TAPE_BASE, 0x0205, METHOD_BUFFERED, FILE_READ_ACCESS)

#if (NTDDI_VERSION < NTDDI_WS03)
#define IOCTL_TAPE_CHECK_VERIFY         CTL_CODE(IOCTL_TAPE_BASE, 0x0200, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_TAPE_FIND_NEW_DEVICES     CTL_CODE(IOCTL_DISK_BASE, 0x0206, METHOD_BUFFERED, FILE_READ_ACCESS)
#endif
//
// The following file contains the IOCTL_STORAGE class ioctls
//

#include <ntddstor.h>

// begin_winnt begin_ntminitape

#ifndef _NTDDTAPE_WINNT_
#define _NTDDTAPE_WINNT_

//
// IOCTL_TAPE_ERASE definitions
//

#define TAPE_ERASE_SHORT            0L
#define TAPE_ERASE_LONG             1L

typedef struct _TAPE_ERASE {
    ULONG Type;
    BOOLEAN Immediate;
} TAPE_ERASE, *PTAPE_ERASE;

//
// IOCTL_TAPE_PREPARE definitions
//

#define TAPE_LOAD                   0L
#define TAPE_UNLOAD                 1L
#define TAPE_TENSION                2L
#define TAPE_LOCK                   3L
#define TAPE_UNLOCK                 4L
#define TAPE_FORMAT                 5L

typedef struct _TAPE_PREPARE {
    ULONG Operation;
    BOOLEAN Immediate;
} TAPE_PREPARE, *PTAPE_PREPARE;

//
// IOCTL_TAPE_WRITE_MARKS definitions
//

#define TAPE_SETMARKS               0L
#define TAPE_FILEMARKS              1L
#define TAPE_SHORT_FILEMARKS        2L
#define TAPE_LONG_FILEMARKS         3L

typedef struct _TAPE_WRITE_MARKS {
    ULONG Type;
    ULONG Count;
    BOOLEAN Immediate;
} TAPE_WRITE_MARKS, *PTAPE_WRITE_MARKS;

//
// IOCTL_TAPE_GET_POSITION definitions
//

#define TAPE_ABSOLUTE_POSITION       0L
#define TAPE_LOGICAL_POSITION        1L
#define TAPE_PSEUDO_LOGICAL_POSITION 2L

typedef struct _TAPE_GET_POSITION {
    ULONG Type;
    ULONG Partition;
    LARGE_INTEGER Offset;
} TAPE_GET_POSITION, *PTAPE_GET_POSITION;

//
// IOCTL_TAPE_SET_POSITION definitions
//

#define TAPE_REWIND                 0L
#define TAPE_ABSOLUTE_BLOCK         1L
#define TAPE_LOGICAL_BLOCK          2L
#define TAPE_PSEUDO_LOGICAL_BLOCK   3L
#define TAPE_SPACE_END_OF_DATA      4L
#define TAPE_SPACE_RELATIVE_BLOCKS  5L
#define TAPE_SPACE_FILEMARKS        6L
#define TAPE_SPACE_SEQUENTIAL_FMKS  7L
#define TAPE_SPACE_SETMARKS         8L
#define TAPE_SPACE_SEQUENTIAL_SMKS  9L

typedef struct _TAPE_SET_POSITION {
    ULONG Method;
    ULONG Partition;
    LARGE_INTEGER Offset;
    BOOLEAN Immediate;
} TAPE_SET_POSITION, *PTAPE_SET_POSITION;

//
// IOCTL_TAPE_GET_DRIVE_PARAMS definitions
//

//
// Definitions for FeaturesLow parameter
//

#define TAPE_DRIVE_FIXED            0x00000001
#define TAPE_DRIVE_SELECT           0x00000002
#define TAPE_DRIVE_INITIATOR        0x00000004

#define TAPE_DRIVE_ERASE_SHORT      0x00000010
#define TAPE_DRIVE_ERASE_LONG       0x00000020
#define TAPE_DRIVE_ERASE_BOP_ONLY   0x00000040
#define TAPE_DRIVE_ERASE_IMMEDIATE  0x00000080

#define TAPE_DRIVE_TAPE_CAPACITY    0x00000100
#define TAPE_DRIVE_TAPE_REMAINING   0x00000200
#define TAPE_DRIVE_FIXED_BLOCK      0x00000400
#define TAPE_DRIVE_VARIABLE_BLOCK   0x00000800

#define TAPE_DRIVE_WRITE_PROTECT    0x00001000
#define TAPE_DRIVE_EOT_WZ_SIZE      0x00002000

#define TAPE_DRIVE_ECC              0x00010000
#define TAPE_DRIVE_COMPRESSION      0x00020000
#define TAPE_DRIVE_PADDING          0x00040000
#define TAPE_DRIVE_REPORT_SMKS      0x00080000

#define TAPE_DRIVE_GET_ABSOLUTE_BLK 0x00100000
#define TAPE_DRIVE_GET_LOGICAL_BLK  0x00200000
#define TAPE_DRIVE_SET_EOT_WZ_SIZE  0x00400000

#define TAPE_DRIVE_EJECT_MEDIA      0x01000000
#define TAPE_DRIVE_CLEAN_REQUESTS   0x02000000
#define TAPE_DRIVE_SET_CMP_BOP_ONLY 0x04000000

#define TAPE_DRIVE_RESERVED_BIT     0x80000000  //don't use this bit!
//                                              //can't be a low features bit!
//                                              //reserved; high features only

//
// Definitions for FeaturesHigh parameter
//

#define TAPE_DRIVE_LOAD_UNLOAD      0x80000001
#define TAPE_DRIVE_TENSION          0x80000002
#define TAPE_DRIVE_LOCK_UNLOCK      0x80000004
#define TAPE_DRIVE_REWIND_IMMEDIATE 0x80000008

#define TAPE_DRIVE_SET_BLOCK_SIZE   0x80000010
#define TAPE_DRIVE_LOAD_UNLD_IMMED  0x80000020
#define TAPE_DRIVE_TENSION_IMMED    0x80000040
#define TAPE_DRIVE_LOCK_UNLK_IMMED  0x80000080

#define TAPE_DRIVE_SET_ECC          0x80000100
#define TAPE_DRIVE_SET_COMPRESSION  0x80000200
#define TAPE_DRIVE_SET_PADDING      0x80000400
#define TAPE_DRIVE_SET_REPORT_SMKS  0x80000800

#define TAPE_DRIVE_ABSOLUTE_BLK     0x80001000
#define TAPE_DRIVE_ABS_BLK_IMMED    0x80002000
#define TAPE_DRIVE_LOGICAL_BLK      0x80004000
#define TAPE_DRIVE_LOG_BLK_IMMED    0x80008000

#define TAPE_DRIVE_END_OF_DATA      0x80010000
#define TAPE_DRIVE_RELATIVE_BLKS    0x80020000
#define TAPE_DRIVE_FILEMARKS        0x80040000
#define TAPE_DRIVE_SEQUENTIAL_FMKS  0x80080000

#define TAPE_DRIVE_SETMARKS         0x80100000
#define TAPE_DRIVE_SEQUENTIAL_SMKS  0x80200000
#define TAPE_DRIVE_REVERSE_POSITION 0x80400000
#define TAPE_DRIVE_SPACE_IMMEDIATE  0x80800000

#define TAPE_DRIVE_WRITE_SETMARKS   0x81000000
#define TAPE_DRIVE_WRITE_FILEMARKS  0x82000000
#define TAPE_DRIVE_WRITE_SHORT_FMKS 0x84000000
#define TAPE_DRIVE_WRITE_LONG_FMKS  0x88000000

#define TAPE_DRIVE_WRITE_MARK_IMMED 0x90000000
#define TAPE_DRIVE_FORMAT           0xA0000000
#define TAPE_DRIVE_FORMAT_IMMEDIATE 0xC0000000
#define TAPE_DRIVE_HIGH_FEATURES    0x80000000  //mask for high features flag

typedef struct _TAPE_GET_DRIVE_PARAMETERS {
    BOOLEAN ECC;
    BOOLEAN Compression;
    BOOLEAN DataPadding;
    BOOLEAN ReportSetmarks;
    ULONG DefaultBlockSize;
    ULONG MaximumBlockSize;
    ULONG MinimumBlockSize;
    ULONG MaximumPartitionCount;
    ULONG FeaturesLow;
    ULONG FeaturesHigh;
    ULONG EOTWarningZoneSize;
} TAPE_GET_DRIVE_PARAMETERS, *PTAPE_GET_DRIVE_PARAMETERS;

//
// IOCTL_TAPE_SET_DRIVE_PARAMETERS definitions
//

typedef struct _TAPE_SET_DRIVE_PARAMETERS {
    BOOLEAN ECC;
    BOOLEAN Compression;
    BOOLEAN DataPadding;
    BOOLEAN ReportSetmarks;
    ULONG EOTWarningZoneSize;
} TAPE_SET_DRIVE_PARAMETERS, *PTAPE_SET_DRIVE_PARAMETERS;

//
// IOCTL_TAPE_GET_MEDIA_PARAMETERS definitions
//

typedef struct _TAPE_GET_MEDIA_PARAMETERS {
    LARGE_INTEGER Capacity;
    LARGE_INTEGER Remaining;
    ULONG BlockSize;
    ULONG PartitionCount;
    BOOLEAN WriteProtected;
} TAPE_GET_MEDIA_PARAMETERS, *PTAPE_GET_MEDIA_PARAMETERS;

//
// IOCTL_TAPE_SET_MEDIA_PARAMETERS definitions
//

typedef struct _TAPE_SET_MEDIA_PARAMETERS {
    ULONG BlockSize;
} TAPE_SET_MEDIA_PARAMETERS, *PTAPE_SET_MEDIA_PARAMETERS;

//
// IOCTL_TAPE_CREATE_PARTITION definitions
//

#define TAPE_FIXED_PARTITIONS       0L
#define TAPE_SELECT_PARTITIONS      1L
#define TAPE_INITIATOR_PARTITIONS   2L

typedef struct _TAPE_CREATE_PARTITION {
    ULONG Method;
    ULONG Count;
    ULONG Size;
} TAPE_CREATE_PARTITION, *PTAPE_CREATE_PARTITION;


//
// WMI Methods
//
#define TAPE_QUERY_DRIVE_PARAMETERS       0L
#define TAPE_QUERY_MEDIA_CAPACITY         1L
#define TAPE_CHECK_FOR_DRIVE_PROBLEM      2L
#define TAPE_QUERY_IO_ERROR_DATA          3L
#define TAPE_QUERY_DEVICE_ERROR_DATA      4L

typedef struct _TAPE_WMI_OPERATIONS {
   ULONG Method;
   ULONG DataBufferSize;
   PVOID DataBuffer;
} TAPE_WMI_OPERATIONS, *PTAPE_WMI_OPERATIONS;

//
// Type of drive errors
//
typedef enum _TAPE_DRIVE_PROBLEM_TYPE {
   TapeDriveProblemNone, TapeDriveReadWriteWarning,
   TapeDriveReadWriteError, TapeDriveReadWarning,
   TapeDriveWriteWarning, TapeDriveReadError,
   TapeDriveWriteError, TapeDriveHardwareError,
   TapeDriveUnsupportedMedia, TapeDriveScsiConnectionError,
   TapeDriveTimetoClean, TapeDriveCleanDriveNow,
   TapeDriveMediaLifeExpired, TapeDriveSnappedTape
} TAPE_DRIVE_PROBLEM_TYPE;

#endif
// end_winnt end_ntminitape

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif  // _NTDDTAPE_

