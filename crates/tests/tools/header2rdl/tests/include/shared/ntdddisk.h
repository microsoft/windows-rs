/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ntdddisk.h

Abstract:

    This is the include file that defines all constants and types for
    accessing the Disk device.

Revision History:

--*/


// begin_winioctl

#ifndef _NTDDDISK_H_
#define _NTDDDISK_H_

#include <winapifamily.h>


#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

// end_winioctl

#if _MSC_VER > 1000
#pragma once
#endif

#if (NTDDI_VERSION >= NTDDI_WINXP)
#include <diskguid.h>
#endif

#if defined __cplusplus && !defined __ALT_GENERATOR__
extern "C" {
#endif

// begin_winioctl

#if defined(_MSC_VER)
#if (_MSC_VER >= 1200)
#pragma warning(push)
#pragma warning(disable:4201) // nonstandard extension used : nameless struct/union
#pragma warning(disable:4214) // nonstandard extension used : bitfield other than int
#pragma warning(disable:4820) // padding added after data member
#endif
#endif

// end_winioctl

//
// Device Name - this string is the name of the device.  It is the name
// that should be passed to NtOpenFile when accessing the device.
//
// Note:  For devices that support multiple units, it should be suffixed
//        with the Ascii representation of the unit number.
//

#define DD_DISK_DEVICE_NAME "\\Device\\UNKNOWN"


//
// NtDeviceIoControlFile

// begin_winioctl

//
// IoControlCode values for disk devices.
//

#define IOCTL_DISK_BASE                 FILE_DEVICE_DISK
#define IOCTL_DISK_GET_DRIVE_GEOMETRY   CTL_CODE(IOCTL_DISK_BASE, 0x0000, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_DISK_GET_PARTITION_INFO   CTL_CODE(IOCTL_DISK_BASE, 0x0001, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DISK_SET_PARTITION_INFO   CTL_CODE(IOCTL_DISK_BASE, 0x0002, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_DISK_GET_DRIVE_LAYOUT     CTL_CODE(IOCTL_DISK_BASE, 0x0003, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DISK_SET_DRIVE_LAYOUT     CTL_CODE(IOCTL_DISK_BASE, 0x0004, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_DISK_VERIFY               CTL_CODE(IOCTL_DISK_BASE, 0x0005, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_DISK_FORMAT_TRACKS        CTL_CODE(IOCTL_DISK_BASE, 0x0006, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_DISK_REASSIGN_BLOCKS      CTL_CODE(IOCTL_DISK_BASE, 0x0007, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_DISK_PERFORMANCE          CTL_CODE(IOCTL_DISK_BASE, 0x0008, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_DISK_IS_WRITABLE          CTL_CODE(IOCTL_DISK_BASE, 0x0009, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_DISK_LOGGING              CTL_CODE(IOCTL_DISK_BASE, 0x000a, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_DISK_FORMAT_TRACKS_EX     CTL_CODE(IOCTL_DISK_BASE, 0x000b, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_DISK_HISTOGRAM_STRUCTURE  CTL_CODE(IOCTL_DISK_BASE, 0x000c, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_DISK_HISTOGRAM_DATA       CTL_CODE(IOCTL_DISK_BASE, 0x000d, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_DISK_HISTOGRAM_RESET      CTL_CODE(IOCTL_DISK_BASE, 0x000e, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_DISK_REQUEST_STRUCTURE    CTL_CODE(IOCTL_DISK_BASE, 0x000f, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_DISK_REQUEST_DATA         CTL_CODE(IOCTL_DISK_BASE, 0x0010, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_DISK_PERFORMANCE_OFF      CTL_CODE(IOCTL_DISK_BASE, 0x0018, METHOD_BUFFERED, FILE_ANY_ACCESS)



#if(_WIN32_WINNT >= 0x0400)
#define IOCTL_DISK_CONTROLLER_NUMBER    CTL_CODE(IOCTL_DISK_BASE, 0x0011, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// IOCTL support for SMART drive fault prediction.
//

#define SMART_GET_VERSION               CTL_CODE(IOCTL_DISK_BASE, 0x0020, METHOD_BUFFERED, FILE_READ_ACCESS)
#define SMART_SEND_DRIVE_COMMAND        CTL_CODE(IOCTL_DISK_BASE, 0x0021, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define SMART_RCV_DRIVE_DATA            CTL_CODE(IOCTL_DISK_BASE, 0x0022, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#if(NTDDI_VERSION >= NTDDI_WIN10_CO)
#define SMART_RCV_DRIVE_DATA_EX         CTL_CODE(IOCTL_DISK_BASE, 0x0023, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif /* NTDDI_VERSION >= NTDDI_WIN10_CO */

#endif /* _WIN32_WINNT >= 0x0400 */

#if (_WIN32_WINNT >= 0x500)

//
// New IOCTLs for GUID Partition tabled disks.
//

#define IOCTL_DISK_GET_PARTITION_INFO_EX    CTL_CODE(IOCTL_DISK_BASE, 0x0012, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_DISK_SET_PARTITION_INFO_EX    CTL_CODE(IOCTL_DISK_BASE, 0x0013, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_DISK_GET_DRIVE_LAYOUT_EX      CTL_CODE(IOCTL_DISK_BASE, 0x0014, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_DISK_SET_DRIVE_LAYOUT_EX      CTL_CODE(IOCTL_DISK_BASE, 0x0015, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_DISK_CREATE_DISK              CTL_CODE(IOCTL_DISK_BASE, 0x0016, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_DISK_GET_LENGTH_INFO          CTL_CODE(IOCTL_DISK_BASE, 0x0017, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DISK_GET_DRIVE_GEOMETRY_EX    CTL_CODE(IOCTL_DISK_BASE, 0x0028, METHOD_BUFFERED, FILE_ANY_ACCESS)

#endif /* _WIN32_WINNT >= 0x0500 */


#if (_WIN32_WINNT >= 0x0502)

//
// New IOCTL for disk devices that support 8 byte LBA
//
#define IOCTL_DISK_REASSIGN_BLOCKS_EX       CTL_CODE(IOCTL_DISK_BASE, 0x0029, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

#endif //_WIN32_WINNT >= 0x0502

#if(_WIN32_WINNT >= 0x0500)
#define IOCTL_DISK_UPDATE_DRIVE_SIZE        CTL_CODE(IOCTL_DISK_BASE, 0x0032, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_DISK_GROW_PARTITION           CTL_CODE(IOCTL_DISK_BASE, 0x0034, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

#define IOCTL_DISK_GET_CACHE_INFORMATION    CTL_CODE(IOCTL_DISK_BASE, 0x0035, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DISK_SET_CACHE_INFORMATION    CTL_CODE(IOCTL_DISK_BASE, 0x0036, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#if (NTDDI_VERSION < NTDDI_WS03)
#define IOCTL_DISK_GET_WRITE_CACHE_STATE    CTL_CODE(IOCTL_DISK_BASE, 0x0037, METHOD_BUFFERED, FILE_READ_ACCESS)
#else
#define OBSOLETE_DISK_GET_WRITE_CACHE_STATE CTL_CODE(IOCTL_DISK_BASE, 0x0037, METHOD_BUFFERED, FILE_READ_ACCESS)
#endif
#define IOCTL_DISK_DELETE_DRIVE_LAYOUT      CTL_CODE(IOCTL_DISK_BASE, 0x0040, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// Called to flush cached information that the driver may have about this
// device's characteristics.  Not all drivers cache characteristics, and not
// cached properties can be flushed.  This simply serves as an update to the
// driver that it may want to do an expensive reexamination of the device's
// characteristics now (fixed media size, partition table, etc...)
//

#define IOCTL_DISK_UPDATE_PROPERTIES    CTL_CODE(IOCTL_DISK_BASE, 0x0050, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
//  Special IOCTLs needed to support PC-98 machines in Japan
//

#define IOCTL_DISK_FORMAT_DRIVE         CTL_CODE(IOCTL_DISK_BASE, 0x00f3, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_DISK_SENSE_DEVICE         CTL_CODE(IOCTL_DISK_BASE, 0x00f8, METHOD_BUFFERED, FILE_ANY_ACCESS)

#endif /* _WIN32_WINNT >= 0x0500 */

// end_winioctl

//
// IOCTLs to report and modify our caching behavior
//

#define IOCTL_DISK_GET_CACHE_SETTING    CTL_CODE(IOCTL_DISK_BASE, 0x0038, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DISK_SET_CACHE_SETTING    CTL_CODE(IOCTL_DISK_BASE, 0x0039, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

typedef enum _DISK_CACHE_STATE {

    DiskCacheNormal,
    DiskCacheWriteThroughNotSupported,
    DiskCacheModifyUnsuccessful

} DISK_CACHE_STATE, *PDISK_CACHE_STATE;

typedef struct _DISK_CACHE_SETTING {

    //
    // The size of this structure is used for versioning
    //
    ULONG Version;

    //
    // Indicates whether there are any issues with the disk cache
    //
    DISK_CACHE_STATE State;

    //
    // Indicates whether the disk cache is power protected or not
    //
    BOOLEAN IsPowerProtected;

} DISK_CACHE_SETTING, *PDISK_CACHE_SETTING;


//
// IOCTL for moving copying a run of sectors from one location of the disk
// to another.  The caller of this IOCTL needs to be prepared for the call to
// fail and do the copy manually since this IOCTL will only rarely be
// implemented.
//

#define IOCTL_DISK_COPY_DATA            CTL_CODE(IOCTL_DISK_BASE, 0x0019, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// This structure is passed in for a IOCTL_DISK_COPY_DATA call.
//

typedef struct _DISK_COPY_DATA_PARAMETERS {

    //
    // Byte offset from which to start the copy.
    //
    LARGE_INTEGER   SourceOffset;

    //
    // Byte offset of the copy destination.
    //
    LARGE_INTEGER   DestinationOffset;

    //
    // Length, in bytes, of the copy.
    //
    LARGE_INTEGER   CopyLength;

    //
    // Must be 0.
    //
    ULONGLONG       Reserved;


} DISK_COPY_DATA_PARAMETERS, *PDISK_COPY_DATA_PARAMETERS;

//
// Internal disk driver device controls to maintain the verify status bit
// for the device object.
//

#define IOCTL_DISK_INTERNAL_SET_VERIFY   CTL_CODE(IOCTL_DISK_BASE, 0x0100, METHOD_NEITHER, FILE_ANY_ACCESS)
#define IOCTL_DISK_INTERNAL_CLEAR_VERIFY CTL_CODE(IOCTL_DISK_BASE, 0x0101, METHOD_NEITHER, FILE_ANY_ACCESS)

//
// Internal disk driver device control to set notification routine for
// the device object. Used in DiskPerf.
//

#define IOCTL_DISK_INTERNAL_SET_NOTIFY   CTL_CODE(IOCTL_DISK_BASE, 0x0102, METHOD_BUFFERED, FILE_ANY_ACCESS)

// begin_winioctl
//
// The following device control codes are common for all class drivers.  The
// functions codes defined here must match all of the other class drivers.
//
// Warning: these codes will be replaced in the future by equivalent
// IOCTL_STORAGE codes
//

#define IOCTL_DISK_CHECK_VERIFY     CTL_CODE(IOCTL_DISK_BASE, 0x0200, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DISK_MEDIA_REMOVAL    CTL_CODE(IOCTL_DISK_BASE, 0x0201, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DISK_EJECT_MEDIA      CTL_CODE(IOCTL_DISK_BASE, 0x0202, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DISK_LOAD_MEDIA       CTL_CODE(IOCTL_DISK_BASE, 0x0203, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DISK_RESERVE          CTL_CODE(IOCTL_DISK_BASE, 0x0204, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DISK_RELEASE          CTL_CODE(IOCTL_DISK_BASE, 0x0205, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DISK_FIND_NEW_DEVICES CTL_CODE(IOCTL_DISK_BASE, 0x0206, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DISK_GET_MEDIA_TYPES  CTL_CODE(IOCTL_DISK_BASE, 0x0300, METHOD_BUFFERED, FILE_ANY_ACCESS)

// end_winioctl

//
// The following file contains the IOCTL_STORAGE class ioctls
//

#include <ntddstor.h>

// begin_winioctl
//
// Define the partition types returnable by known disk drivers.
//

#define PARTITION_ENTRY_UNUSED          0x00      // Entry unused
#define PARTITION_FAT_12                0x01      // 12-bit FAT entries
#define PARTITION_XENIX_1               0x02      // Xenix
#define PARTITION_XENIX_2               0x03      // Xenix
#define PARTITION_FAT_16                0x04      // 16-bit FAT entries
#define PARTITION_EXTENDED              0x05      // Extended partition entry
#define PARTITION_HUGE                  0x06      // Huge partition MS-DOS V4
#define PARTITION_IFS                   0x07      // IFS Partition
#define PARTITION_OS2BOOTMGR            0x0A      // OS/2 Boot Manager/OPUS/Coherent swap
#define PARTITION_FAT32                 0x0B      // FAT32
#define PARTITION_FAT32_XINT13          0x0C      // FAT32 using extended int13 services
#define PARTITION_XINT13                0x0E      // Win95 partition using extended int13 services
#define PARTITION_XINT13_EXTENDED       0x0F      // Same as type 5 but uses extended int13 services
#define PARTITION_MSFT_RECOVERY         0x27      // Microsoft recovery partition
#define PARTITION_MAIN_OS               0x28      // Main OS partition
#define PARTIITON_OS_DATA               0x29      // OS data partition
#define PARTITION_PRE_INSTALLED         0x2a      // PreInstalled partition
#define PARTITION_BSP                   0x2b      // BSP partition
#define PARTITION_DPP                   0x2c      // DPP partition
#define PARTITION_WINDOWS_SYSTEM        0x2d      // Windows system partition
#define PARTITION_PREP                  0x41      // PowerPC Reference Platform (PReP) Boot Partition
#define PARTITION_LDM                   0x42      // Logical Disk Manager partition
#define PARTITION_DM                    0x54      // OnTrack Disk Manager partition
#define PARTITION_EZDRIVE               0x55      // EZ-Drive partition
#define PARTITION_UNIX                  0x63      // Unix
#define PARTITION_SPACES_DATA           0xD7      // Storage Spaces protective partition
#define PARTITION_SPACES                0xE7      // Storage Spaces protective partition
#define PARTITION_GPT                   0xEE      // Gpt protective partition
#define PARTITION_SYSTEM                0xEF      // System partition

#define VALID_NTFT                      0xC0      // NTFT uses high order bits

//
// The high bit of the partition type code indicates that a partition
// is part of an NTFT mirror or striped array.
//

#define PARTITION_NTFT                  0x80     // NTFT partition

//
// The following macro is used to determine which partitions should be
// assigned drive letters.
//

//++
//
// BOOLEAN
// IsRecognizedPartition(
//     IN ULONG PartitionType
//     )
//
// Routine Description:
//
//     This macro is used to determine to which partitions drive letters
//     should be assigned.
//
// Arguments:
//
//     PartitionType - Supplies the type of the partition being examined.
//
// Return Value:
//
//     The return value is TRUE if the partition type is recognized,
//     otherwise FALSE is returned.
//
//--
#if (NTDDI_VERSION < NTDDI_VISTA)
#define IsRecognizedPartition( PartitionType ) (    \
    ((PartitionType & PARTITION_NTFT) && (((PartitionType & ~0xC0) == PARTITION_HUGE)           ||  \
                                          ((PartitionType & ~0xC0) == PARTITION_IFS)            ||  \
                                          ((PartitionType & ~0xC0) == PARTITION_FAT32)          ||  \
                                          ((PartitionType & ~0xC0) == PARTITION_FAT32_XINT13))) ||  \
    ((PartitionType) == PARTITION_FAT_12)       ||  \
    ((PartitionType) == PARTITION_FAT_16)       ||  \
    ((PartitionType) == PARTITION_HUGE)         ||  \
    ((PartitionType) == PARTITION_IFS)          ||  \
    ((PartitionType) == PARTITION_FAT32)        ||  \
    ((PartitionType) == PARTITION_FAT32_XINT13) ||  \
    ((PartitionType) == PARTITION_XINT13) )
#else
#define IsRecognizedPartition( PartitionType ) (      \
    ((PartitionType) == PARTITION_BSP)            ||  \
    ((PartitionType) == PARTITION_DPP)            ||  \
    ((PartitionType) == PARTITION_FAT_12)         ||  \
    ((PartitionType) == PARTITION_FAT_16)         ||  \
    ((PartitionType) == PARTITION_FAT32)          ||  \
    ((PartitionType) == PARTITION_FAT32_XINT13)   ||  \
    ((PartitionType) == PARTITION_HUGE)           ||  \
    ((PartitionType) == PARTITION_IFS)            ||  \
    ((PartitionType) == PARTITION_MAIN_OS)        ||  \
    ((PartitionType) == PARTITION_MSFT_RECOVERY)  ||  \
    ((PartitionType) == PARTIITON_OS_DATA)        ||  \
    ((PartitionType) == PARTITION_PRE_INSTALLED)  ||  \
    ((PartitionType) == PARTITION_SYSTEM)         ||  \
    ((PartitionType) == PARTITION_WINDOWS_SYSTEM) ||  \
    ((PartitionType) == PARTITION_XINT13) )
#endif

//++
//
// BOOLEAN
// IsContainerPartition(
//     IN ULONG PartitionType
//     )
//
// Routine Description:
//
//     This macro is used to determine to which partition types are actually
//     containers for other partitions (ie, extended partitions).
//
// Arguments:
//
//     PartitionType - Supplies the type of the partition being examined.
//
// Return Value:
//
//     The return value is TRUE if the partition type is a container,
//     otherwise FALSE is returned.
//
//--

#define IsContainerPartition( PartitionType )       \
    ((PartitionType == PARTITION_EXTENDED) || (PartitionType == PARTITION_XINT13_EXTENDED))

//++
//
// BOOLEAN
// IsFTPartition(
//     IN ULONG PartitionType
//     )
//
// Routine Description:
//
//     This macro is used to determine if the given partition is an FT
//     partition.
//
// Arguments:
//
//     PartitionType - Supplies the type of the partition being examined.
//
// Return Value:
//
//     The return value is TRUE if the partition type is an FT partition,
//     otherwise FALSE is returned.
//
//--

#define IsFTPartition( PartitionType )              \
    ((PartitionType & PARTITION_NTFT) && (((PartitionType & ~0xC0) == PARTITION_HUGE)         ||  \
                                          ((PartitionType & ~0xC0) == PARTITION_IFS)          ||  \
                                          ((PartitionType & ~0xC0) == PARTITION_FAT32)        ||  \
                                          ((PartitionType & ~0xC0) == PARTITION_FAT32_XINT13)))

//
// Define the media types supported by the driver.
//

typedef enum _MEDIA_TYPE {
    Unknown,                // Format is unknown
    F5_1Pt2_512,            // 5.25", 1.2MB,  512 bytes/sector
    F3_1Pt44_512,           // 3.5",  1.44MB, 512 bytes/sector
    F3_2Pt88_512,           // 3.5",  2.88MB, 512 bytes/sector
    F3_20Pt8_512,           // 3.5",  20.8MB, 512 bytes/sector
    F3_720_512,             // 3.5",  720KB,  512 bytes/sector
    F5_360_512,             // 5.25", 360KB,  512 bytes/sector
    F5_320_512,             // 5.25", 320KB,  512 bytes/sector
    F5_320_1024,            // 5.25", 320KB,  1024 bytes/sector
    F5_180_512,             // 5.25", 180KB,  512 bytes/sector
    F5_160_512,             // 5.25", 160KB,  512 bytes/sector
    RemovableMedia,         // Removable media other than floppy
    FixedMedia,             // Fixed hard disk media
    F3_120M_512,            // 3.5", 120M Floppy
    F3_640_512,             // 3.5" ,  640KB,  512 bytes/sector
    F5_640_512,             // 5.25",  640KB,  512 bytes/sector
    F5_720_512,             // 5.25",  720KB,  512 bytes/sector
    F3_1Pt2_512,            // 3.5" ,  1.2Mb,  512 bytes/sector
    F3_1Pt23_1024,          // 3.5" ,  1.23Mb, 1024 bytes/sector
    F5_1Pt23_1024,          // 5.25",  1.23MB, 1024 bytes/sector
    F3_128Mb_512,           // 3.5" MO 128Mb   512 bytes/sector
    F3_230Mb_512,           // 3.5" MO 230Mb   512 bytes/sector
    F8_256_128,             // 8",     256KB,  128 bytes/sector
    F3_200Mb_512,           // 3.5",   200M Floppy (HiFD)
    F3_240M_512,            // 3.5",   240Mb Floppy (HiFD)
    F3_32M_512              // 3.5",   32Mb Floppy
} MEDIA_TYPE, *PMEDIA_TYPE;

//
// Define the input buffer structure for the driver, when
// it is called with IOCTL_DISK_FORMAT_TRACKS.
//

typedef struct _FORMAT_PARAMETERS {
   MEDIA_TYPE MediaType;
   ULONG StartCylinderNumber;
   ULONG EndCylinderNumber;
   ULONG StartHeadNumber;
   ULONG EndHeadNumber;
} FORMAT_PARAMETERS, *PFORMAT_PARAMETERS;

//
// Define the BAD_TRACK_NUMBER type. An array of elements of this type is
// returned by the driver on IOCTL_DISK_FORMAT_TRACKS requests, to indicate
// what tracks were bad during formatting. The length of that array is
// reported in the `Information' field of the I/O Status Block.
//

typedef USHORT BAD_TRACK_NUMBER;
typedef USHORT *PBAD_TRACK_NUMBER;

//
// Define the input buffer structure for the driver, when
// it is called with IOCTL_DISK_FORMAT_TRACKS_EX.
//

typedef struct _FORMAT_EX_PARAMETERS {
   MEDIA_TYPE MediaType;
   ULONG StartCylinderNumber;
   ULONG EndCylinderNumber;
   ULONG StartHeadNumber;
   ULONG EndHeadNumber;
   USHORT FormatGapLength;
   USHORT SectorsPerTrack;
   USHORT SectorNumber[1];
} FORMAT_EX_PARAMETERS, *PFORMAT_EX_PARAMETERS;

//
// The following structure is returned on an IOCTL_DISK_GET_DRIVE_GEOMETRY
// request and an array of them is returned on an IOCTL_DISK_GET_MEDIA_TYPES
// request.
//

typedef struct _DISK_GEOMETRY {

    LARGE_INTEGER Cylinders;

    MEDIA_TYPE MediaType;

    ULONG TracksPerCylinder;

    ULONG SectorsPerTrack;

    ULONG BytesPerSector;

} DISK_GEOMETRY, *PDISK_GEOMETRY;



//
// This wmi guid returns a DISK_GEOMETRY structure
//
#define WMI_DISK_GEOMETRY_GUID         { 0x25007f51, 0x57c2, 0x11d1, { 0xa5, 0x28, 0x0, 0xa0, 0xc9, 0x6, 0x29, 0x10 } }



//
// The following structure is returned on an IOCTL_DISK_GET_PARTITION_INFO
// and an IOCTL_DISK_GET_DRIVE_LAYOUT request.  It is also used in a request
// to change the drive layout, IOCTL_DISK_SET_DRIVE_LAYOUT.
//

typedef struct _PARTITION_INFORMATION {
    LARGE_INTEGER StartingOffset;
    LARGE_INTEGER PartitionLength;
    ULONG HiddenSectors;
    ULONG PartitionNumber;
    UCHAR PartitionType;
    BOOLEAN BootIndicator;
    BOOLEAN RecognizedPartition;
    BOOLEAN RewritePartition;
} PARTITION_INFORMATION, *PPARTITION_INFORMATION;

//
// The following structure is used to change the partition type of a
// specified disk partition using an IOCTL_DISK_SET_PARTITION_INFO
// request.
//

typedef struct _SET_PARTITION_INFORMATION {
    UCHAR PartitionType;
} SET_PARTITION_INFORMATION, *PSET_PARTITION_INFORMATION;

//
// The following structures is returned on an IOCTL_DISK_GET_DRIVE_LAYOUT
// request and given as input to an IOCTL_DISK_SET_DRIVE_LAYOUT request.
//

typedef struct _DRIVE_LAYOUT_INFORMATION {
    ULONG PartitionCount;
    ULONG Signature;
    PARTITION_INFORMATION PartitionEntry[1];
} DRIVE_LAYOUT_INFORMATION, *PDRIVE_LAYOUT_INFORMATION;

//
// The following structure is passed in on an IOCTL_DISK_VERIFY request.
// The offset and length parameters are both given in bytes.
//

typedef struct _VERIFY_INFORMATION {
    LARGE_INTEGER StartingOffset;
    ULONG Length;
} VERIFY_INFORMATION, *PVERIFY_INFORMATION;

//
// The following structure is passed in on an IOCTL_DISK_REASSIGN_BLOCKS
// request.
//

typedef struct _REASSIGN_BLOCKS {
    USHORT Reserved;
    USHORT Count;
    ULONG BlockNumber[1];
} REASSIGN_BLOCKS, *PREASSIGN_BLOCKS;

//
// The following structure is passed in on an IOCTL_DISK_REASSIGN_BLOCKS_EX
// request.
//

#include <pshpack1.h>
typedef struct _REASSIGN_BLOCKS_EX {
    USHORT Reserved;
    USHORT Count;
    LARGE_INTEGER BlockNumber[1];
} REASSIGN_BLOCKS_EX, *PREASSIGN_BLOCKS_EX;
#include <poppack.h>


#if(_WIN32_WINNT >= 0x500)

//
// Support for GUID Partition Table (GPT) disks.
//

//
// There are currently two ways a disk can be partitioned. With a traditional
// AT-style master boot record (PARTITION_STYLE_MBR) and with a new, GPT
// partition table (PARTITION_STYLE_GPT). RAW is for an unrecognizable
// partition style. There are a very limited number of things you can
// do with a RAW partititon.
//

typedef enum _PARTITION_STYLE {
    PARTITION_STYLE_MBR,
    PARTITION_STYLE_GPT,
    PARTITION_STYLE_RAW
} PARTITION_STYLE;


//
// The following structure defines information in a GPT partition that is
// not common to both GPT and MBR partitions.
//

typedef struct _PARTITION_INFORMATION_GPT {

    GUID PartitionType;                 // Partition type. See table 16-3.

    GUID PartitionId;                   // Unique GUID for this partition.

    ULONG64 Attributes;                 // See table 16-4.

    WCHAR Name [36];                    // Partition Name in Unicode.

} PARTITION_INFORMATION_GPT, *PPARTITION_INFORMATION_GPT;

//
//  The following are GPT partition attributes applicable for any
//  partition type. These attributes are not OS-specific
//

#define GPT_ATTRIBUTE_PLATFORM_REQUIRED             (0x0000000000000001)
#define GPT_ATTRIBUTE_NO_BLOCK_IO_PROTOCOL          (0x0000000000000002)
#define GPT_ATTRIBUTE_LEGACY_BIOS_BOOTABLE          (0x0000000000000004)

//
// The following are GPT partition attributes applicable when the
// PartitionType is PARTITION_BASIC_DATA_GUID.
//

#define GPT_BASIC_DATA_ATTRIBUTE_NO_DRIVE_LETTER    (0x8000000000000000)
#define GPT_BASIC_DATA_ATTRIBUTE_HIDDEN             (0x4000000000000000)
#define GPT_BASIC_DATA_ATTRIBUTE_SHADOW_COPY        (0x2000000000000000)
#define GPT_BASIC_DATA_ATTRIBUTE_READ_ONLY          (0x1000000000000000)
#define GPT_BASIC_DATA_ATTRIBUTE_OFFLINE            (0x0800000000000000)
#define GPT_BASIC_DATA_ATTRIBUTE_DAX                (0x0400000000000000)
#define GPT_BASIC_DATA_ATTRIBUTE_SERVICE            (0x0200000000000000)

//
// The following are GPT partition attributes applicable when the
// PartitionType is PARTITION_SPACES_GUID.
//

#define GPT_SPACES_ATTRIBUTE_NO_METADATA            (0x8000000000000000)

//
// The following structure defines information in an MBR partition that is not
// common to both GPT and MBR partitions.
//

typedef struct _PARTITION_INFORMATION_MBR {

    UCHAR PartitionType;

    BOOLEAN BootIndicator;

    BOOLEAN RecognizedPartition;

    ULONG HiddenSectors;

#if (NTDDI_VERSION >= NTDDI_WINBLUE)    /* ABRACADABRA_THRESHOLD */
    GUID PartitionId;
#endif

} PARTITION_INFORMATION_MBR, *PPARTITION_INFORMATION_MBR;


//
// The structure SET_PARTITION_INFO_EX is used with the ioctl
// IOCTL_SET_PARTITION_INFO_EX to set information about a specific
// partition. Note that for MBR partitions, you can only set the partition
// signature, whereas GPT partitions allow setting of all fields that
// you can get.
//

typedef SET_PARTITION_INFORMATION SET_PARTITION_INFORMATION_MBR;
typedef PARTITION_INFORMATION_GPT SET_PARTITION_INFORMATION_GPT;


typedef struct _SET_PARTITION_INFORMATION_EX {
    PARTITION_STYLE PartitionStyle;
    union {
        SET_PARTITION_INFORMATION_MBR Mbr;
        SET_PARTITION_INFORMATION_GPT Gpt;
    } DUMMYUNIONNAME;
} SET_PARTITION_INFORMATION_EX, *PSET_PARTITION_INFORMATION_EX;


//
// The structure CREATE_DISK_GPT with the ioctl IOCTL_DISK_CREATE_DISK
// to initialize a disk with an empty GPT partition table.
//

typedef struct _CREATE_DISK_GPT {
    GUID DiskId;                    // Unique disk id for the disk.
    ULONG MaxPartitionCount;        // Maximim number of partitions allowable.
} CREATE_DISK_GPT, *PCREATE_DISK_GPT;

//
// The structure CREATE_DISK_MBR with the ioctl IOCTL_DISK_CREATE_DISK
// to initialize a disk with an empty MBR partition table.
//

typedef struct _CREATE_DISK_MBR {
    ULONG Signature;
} CREATE_DISK_MBR, *PCREATE_DISK_MBR;


typedef struct _CREATE_DISK {
    PARTITION_STYLE PartitionStyle;
    union {
        CREATE_DISK_MBR Mbr;
        CREATE_DISK_GPT Gpt;
    } DUMMYUNIONNAME;
} CREATE_DISK, *PCREATE_DISK;


//
// The structure GET_LENGTH_INFORMATION is used with the ioctl
// IOCTL_DISK_GET_LENGTH_INFO to obtain the length, in bytes, of the
// disk, partition, or volume.
//

typedef struct _GET_LENGTH_INFORMATION {
    LARGE_INTEGER   Length;
} GET_LENGTH_INFORMATION, *PGET_LENGTH_INFORMATION;

//
// The PARTITION_INFORMATION_EX structure is used with the
// IOCTL_DISK_GET_DRIVE_LAYOUT_EX, IOCTL_DISK_SET_DRIVE_LAYOUT_EX,
// IOCTL_DISK_GET_PARTITION_INFO_EX and IOCTL_DISK_SET_PARTITION_INFO_EX calls.
//

typedef struct _PARTITION_INFORMATION_EX {

    PARTITION_STYLE PartitionStyle;

    LARGE_INTEGER StartingOffset;

    LARGE_INTEGER PartitionLength;

    ULONG PartitionNumber;

    BOOLEAN RewritePartition;

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)  /* ABRACADABRA_WIN10_RS3 */
    BOOLEAN IsServicePartition;
#endif

    union {

        PARTITION_INFORMATION_MBR Mbr;

        PARTITION_INFORMATION_GPT Gpt;

    } DUMMYUNIONNAME;

} PARTITION_INFORMATION_EX, *PPARTITION_INFORMATION_EX;


//
// GPT specific drive layout information.
//

typedef struct _DRIVE_LAYOUT_INFORMATION_GPT {

    GUID DiskId;

    LARGE_INTEGER StartingUsableOffset;

    LARGE_INTEGER UsableLength;

    ULONG MaxPartitionCount;

} DRIVE_LAYOUT_INFORMATION_GPT, *PDRIVE_LAYOUT_INFORMATION_GPT;


//
// MBR specific drive layout information.
//

typedef struct _DRIVE_LAYOUT_INFORMATION_MBR {

    ULONG Signature;

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)  /* ABRACADABRA_WIN10_RS1 */
    ULONG CheckSum;
#endif

} DRIVE_LAYOUT_INFORMATION_MBR, *PDRIVE_LAYOUT_INFORMATION_MBR;

//
// The structure DRIVE_LAYOUT_INFORMATION_EX is used with the
// IOCTL_SET_DRIVE_LAYOUT_EX and IOCTL_GET_DRIVE_LAYOUT_EX calls.
//

typedef struct _DRIVE_LAYOUT_INFORMATION_EX {

    ULONG PartitionStyle;

    ULONG PartitionCount;

    union {

        DRIVE_LAYOUT_INFORMATION_MBR Mbr;

        DRIVE_LAYOUT_INFORMATION_GPT Gpt;

    } DUMMYUNIONNAME;

    PARTITION_INFORMATION_EX PartitionEntry[1];

} DRIVE_LAYOUT_INFORMATION_EX, *PDRIVE_LAYOUT_INFORMATION_EX;


#endif // (_WIN32_WINNT >= 0x0500)


#if(_WIN32_WINNT >= 0x0500)

//
// The DISK_GEOMETRY_EX structure is returned on issuing an
// IOCTL_DISK_GET_DRIVE_GEOMETRY_EX ioctl.
//

typedef enum _DETECTION_TYPE {
        DetectNone,
        DetectInt13,
        DetectExInt13
} DETECTION_TYPE;

typedef struct _DISK_INT13_INFO {
        USHORT DriveSelect;
        ULONG MaxCylinders;
        USHORT SectorsPerTrack;
        USHORT MaxHeads;
        USHORT NumberDrives;
} DISK_INT13_INFO, *PDISK_INT13_INFO;

typedef struct _DISK_EX_INT13_INFO {
        USHORT ExBufferSize;
        USHORT ExFlags;
        ULONG ExCylinders;
        ULONG ExHeads;
        ULONG ExSectorsPerTrack;
        ULONG64 ExSectorsPerDrive;
        USHORT ExSectorSize;
        USHORT ExReserved;
} DISK_EX_INT13_INFO, *PDISK_EX_INT13_INFO;

#if (_MSC_VER >= 1200)
#pragma warning(push)
#pragma warning(disable:4201) // nonstandard extension used : nameless struct/union
#endif

typedef struct _DISK_DETECTION_INFO {
        ULONG SizeOfDetectInfo;
        DETECTION_TYPE DetectionType;
        union {
                struct {

                        //
                        // If DetectionType == DETECTION_INT13 then we have just the Int13
                        // information.
                        //

                        DISK_INT13_INFO Int13;

                        //
                        // If DetectionType == DETECTION_EX_INT13, then we have the
                        // extended int 13 information.
                        //

                        DISK_EX_INT13_INFO ExInt13;     // If DetectionType == DetectExInt13
                } DUMMYSTRUCTNAME;
        } DUMMYUNIONNAME;
} DISK_DETECTION_INFO, *PDISK_DETECTION_INFO;


typedef struct _DISK_PARTITION_INFO {
        ULONG SizeOfPartitionInfo;
        PARTITION_STYLE PartitionStyle;                 // PartitionStyle = RAW, GPT or MBR
        union {
                struct {                                                        // If PartitionStyle == MBR
                        ULONG Signature;                                // MBR Signature
                        ULONG CheckSum;                                 // MBR CheckSum
                } Mbr;
                struct {                                                        // If PartitionStyle == GPT
                        GUID DiskId;
                } Gpt;
        } DUMMYUNIONNAME;
} DISK_PARTITION_INFO, *PDISK_PARTITION_INFO;

#if (_MSC_VER >= 1200)
#pragma warning(pop)
#endif

//
// The Geometry structure is a variable length structure composed of a
// DISK_GEOMETRY_EX structure followed by a DISK_PARTITION_INFO structure
// followed by a DISK_DETECTION_DATA structure.
//

#if (NTDDI_VERSION < NTDDI_WS03)
#define DiskGeometryGetPartition(Geometry)\
                        ((PDISK_PARTITION_INFO)((Geometry)+1))

#define DiskGeometryGetDetect(Geometry)\
                        ((PDISK_DETECTION_INFO)(((PBYTE)DiskGeometryGetPartition(Geometry)+\
                                        DiskGeometryGetPartition(Geometry)->SizeOfPartitionInfo)))
#else
#define DiskGeometryGetPartition(Geometry)\
                        ((PDISK_PARTITION_INFO)((Geometry)->Data))

#define DiskGeometryGetDetect(Geometry)\
                        ((PDISK_DETECTION_INFO)(((ULONG_PTR)DiskGeometryGetPartition(Geometry)+\
                                        DiskGeometryGetPartition(Geometry)->SizeOfPartitionInfo)))
#endif
typedef struct _DISK_GEOMETRY_EX {
        DISK_GEOMETRY Geometry;                                 // Standard disk geometry: may be faked by driver.
        LARGE_INTEGER DiskSize;                                 // Must always be correct
        UCHAR Data[1];                                                  // Partition, Detect info
} DISK_GEOMETRY_EX, *PDISK_GEOMETRY_EX;

#endif // (_WIN32_WINNT > 0x0500)

#if(_WIN32_WINNT >= 0x0400)
//
// IOCTL_DISK_CONTROLLER_NUMBER returns the controller and disk
// number for the handle.  This is used to determine if a disk
// is attached to the primary or secondary IDE controller.
//

typedef struct _DISK_CONTROLLER_NUMBER {
    ULONG ControllerNumber;
    ULONG DiskNumber;
} DISK_CONTROLLER_NUMBER, *PDISK_CONTROLLER_NUMBER;
#endif /* _WIN32_WINNT >= 0x0400 */

#if(_WIN32_WINNT >= 0x0500)

//
// IOCTL_DISK_SET_CACHE_INFORMATION
//
// Input Buffer:
//      A DISK_CACHE_INFORMATION structure which describes how the disk
//      read/write caches should be configured.
//
// Output Buffer:
//      None
//

//
// IOCTL_DISK_GET_CACHE_INFORMATION
//
// Input Buffer:
//      None
//
// Output Buffer:
//      A DISK_CACHE_INFORMATION structure which contains the current state
//      of the disk read/write caches.
//

typedef enum {
    EqualPriority,
    KeepPrefetchedData,
    KeepReadData
} DISK_CACHE_RETENTION_PRIORITY;

#if (OSVER(NTDDI_VERSION) == NTDDI_WINXP)
typedef enum _DISK_WRITE_CACHE_STATE {
    DiskWriteCacheNormal,
    DiskWriteCacheForceDisable,
    DiskWriteCacheDisableNotSupported
} DISK_WRITE_CACHE_STATE, *PDISK_WRITE_CACHE_STATE;
#endif

typedef struct _DISK_CACHE_INFORMATION {

    //
    // on return indicates that the device is capable of saving any parameters
    // in non-volatile storage.  On send indicates that the device should
    // save the state in non-volatile storage.
    //

    BOOLEAN ParametersSavable;

    //
    // Indicates whether the write and read caches are enabled.
    //

    BOOLEAN ReadCacheEnabled;
    BOOLEAN WriteCacheEnabled;

    //
    // Controls the likelyhood of data remaining in the cache depending on how
    // it got there.  Data cached from a READ or WRITE operation may be given
    // higher, lower or equal priority to data entered into the cache for other
    // means (like prefetch)
    //

    DISK_CACHE_RETENTION_PRIORITY ReadRetentionPriority;
    DISK_CACHE_RETENTION_PRIORITY WriteRetentionPriority;

    //
    // Requests for a larger number of blocks than this may have prefetching
    // disabled.  If this value is set to 0 prefetch will be disabled.
    //

    USHORT DisablePrefetchTransferLength;

    //
    // If TRUE then ScalarPrefetch (below) will be valid.  If FALSE then
    // the minimum and maximum values should be treated as a block count
    // (BlockPrefetch)
    //

    BOOLEAN PrefetchScalar;

    //
    // Contains the minimum and maximum amount of data which will be
    // will be prefetched into the cache on a disk operation.  This value
    // may either be a scalar multiplier of the transfer length of the request,
    // or an abolute number of disk blocks.  PrefetchScalar (above) indicates
    // which interpretation is used.
    //

    union {
        struct {
            USHORT Minimum;
            USHORT Maximum;

            //
            // The maximum number of blocks which will be prefetched - useful
            // with the scalar limits to set definite upper limits.
            //

            USHORT MaximumBlocks;
        } ScalarPrefetch;

        struct {
            USHORT Minimum;
            USHORT Maximum;
        } BlockPrefetch;
    } DUMMYUNIONNAME;

} DISK_CACHE_INFORMATION, *PDISK_CACHE_INFORMATION;


//
// IOCTL_DISK_GROW_PARTITION will update the size of a partition
// by adding sectors to the length. The number of sectors must be
// predetermined by examining PARTITION_INFORMATION.
//

typedef struct _DISK_GROW_PARTITION {
    ULONG PartitionNumber;
    LARGE_INTEGER BytesToGrow;
} DISK_GROW_PARTITION, *PDISK_GROW_PARTITION;
#endif /* _WIN32_WINNT >= 0x0500 */

///////////////////////////////////////////////////////
//                                                   //
// The following structures define disk performance  //
// statistics: specifically the locations of all the //
// reads and writes which have occured on the disk.  //
//                                                   //
// To use these structures, you must issue an IOCTL_ //
// DISK_HIST_STRUCTURE (with a DISK_HISTOGRAM) to    //
// obtain the basic histogram information. The       //
// number of buckets which must allocated is part of //
// this structure. Allocate the required number of   //
// buckets and call an IOCTL_DISK_HIST_DATA to fill  //
// in the data                                       //
//                                                   //
///////////////////////////////////////////////////////

#define HIST_NO_OF_BUCKETS  24

typedef struct _HISTOGRAM_BUCKET {
    ULONG       Reads;
    ULONG       Writes;
} HISTOGRAM_BUCKET, *PHISTOGRAM_BUCKET;

#define HISTOGRAM_BUCKET_SIZE   sizeof(HISTOGRAM_BUCKET)

typedef struct _DISK_HISTOGRAM {
    LARGE_INTEGER   DiskSize;
    LARGE_INTEGER   Start;
    LARGE_INTEGER   End;
    LARGE_INTEGER   Average;
    LARGE_INTEGER   AverageRead;
    LARGE_INTEGER   AverageWrite;
    ULONG           Granularity;
    ULONG           Size;
    ULONG           ReadCount;
    ULONG           WriteCount;
    PHISTOGRAM_BUCKET  Histogram;
} DISK_HISTOGRAM, *PDISK_HISTOGRAM;

#define DISK_HISTOGRAM_SIZE sizeof(DISK_HISTOGRAM)

///////////////////////////////////////////////////////
//                                                   //
// The following structures define disk debugging    //
// capabilities. The IOCTLs are directed to one of   //
// the two disk filter drivers.                      //
//                                                   //
// DISKPERF is a utilty for collecting disk request  //
// statistics.                                       //
//                                                   //
// SIMBAD is a utility for injecting faults in       //
// IO requests to disks.                             //
//                                                   //
///////////////////////////////////////////////////////

//
// The following structure is exchanged on an IOCTL_DISK_GET_PERFORMANCE
// request. This ioctl collects summary disk request statistics used
// in measuring performance.
//

typedef struct _DISK_PERFORMANCE {
        LARGE_INTEGER BytesRead;
        LARGE_INTEGER BytesWritten;
        LARGE_INTEGER ReadTime;
        LARGE_INTEGER WriteTime;
        LARGE_INTEGER IdleTime;
        ULONG ReadCount;
        ULONG WriteCount;
        ULONG QueueDepth;
        ULONG SplitCount;
        LARGE_INTEGER QueryTime;
        ULONG   StorageDeviceNumber;
        WCHAR   StorageManagerName[8];
} DISK_PERFORMANCE, *PDISK_PERFORMANCE;

//
// This structure defines the disk logging record. When disk logging
// is enabled, one of these is written to an internal buffer for each
// disk request.
//

typedef struct _DISK_RECORD {
   LARGE_INTEGER ByteOffset;
   LARGE_INTEGER StartTime;
   LARGE_INTEGER EndTime;
   PVOID VirtualAddress;
   ULONG NumberOfBytes;
   UCHAR DeviceNumber;
   BOOLEAN ReadRequest;
} DISK_RECORD, *PDISK_RECORD;

//
// The following structure is exchanged on an IOCTL_DISK_LOG request.
// Not all fields are valid with each function type.
//

typedef struct _DISK_LOGGING {
    UCHAR Function;
    PVOID BufferAddress;
    ULONG BufferSize;
} DISK_LOGGING, *PDISK_LOGGING;

//
// Disk logging functions
//
// Start disk logging. Only the Function and BufferSize fields are valid.
//

#define DISK_LOGGING_START    0

//
// Stop disk logging. Only the Function field is valid.
//

#define DISK_LOGGING_STOP     1

//
// Return disk log. All fields are valid. Data will be copied from internal
// buffer to buffer specified for the number of bytes requested.
//

#define DISK_LOGGING_DUMP     2

//
// DISK BINNING
//
// DISKPERF will keep counters for IO that falls in each of these ranges.
// The application determines the number and size of the ranges.
// Joe Lin wanted me to keep it flexible as possible, for instance, IO
// sizes are interesting in ranges like 0-4096, 4097-16384, 16385-65536, 65537+.
//

#define DISK_BINNING          3

//
// Bin types
//

typedef enum _BIN_TYPES {
    RequestSize,
    RequestLocation
} BIN_TYPES;

//
// Bin ranges
//

typedef struct _BIN_RANGE {
    LARGE_INTEGER StartValue;
    LARGE_INTEGER Length;
} BIN_RANGE, *PBIN_RANGE;

//
// Bin definition
//

typedef struct _PERF_BIN {
    ULONG NumberOfBins;
    ULONG TypeOfBin;
    BIN_RANGE BinsRanges[1];
} PERF_BIN, *PPERF_BIN ;

//
// Bin count
//

typedef struct _BIN_COUNT {
    BIN_RANGE BinRange;
    ULONG BinCount;
} BIN_COUNT, *PBIN_COUNT;

//
// Bin results
//

typedef struct _BIN_RESULTS {
    ULONG NumberOfBins;
    BIN_COUNT BinCounts[1];
} BIN_RESULTS, *PBIN_RESULTS;

#if(_WIN32_WINNT >= 0x0400)
//
// Data structures for SMART drive fault prediction.
//
// GETVERSIONINPARAMS contains the data returned from the
// Get Driver Version function.
//

#include <pshpack1.h>
typedef struct _GETVERSIONINPARAMS {
        UCHAR    bVersion;               // Binary driver version.
        UCHAR    bRevision;              // Binary driver revision.
        UCHAR    bReserved;              // Not used.
        UCHAR    bIDEDeviceMap;          // Bit map of IDE devices.
        ULONG   fCapabilities;          // Bit mask of driver capabilities.
        ULONG   dwReserved[4];          // For future use.
} GETVERSIONINPARAMS, *PGETVERSIONINPARAMS, *LPGETVERSIONINPARAMS;
#include <poppack.h>

//
// Bits returned in the fCapabilities member of GETVERSIONINPARAMS
//

#define CAP_ATA_ID_CMD          1       // ATA ID command supported
#define CAP_ATAPI_ID_CMD        2       // ATAPI ID command supported
#define CAP_SMART_CMD           4       // SMART commannds supported

//
// IDE registers
//

#include <pshpack1.h>
typedef struct _IDEREGS {
        UCHAR    bFeaturesReg;           // Used for specifying SMART "commands".
        UCHAR    bSectorCountReg;        // IDE sector count register
        UCHAR    bSectorNumberReg;       // IDE sector number register
        UCHAR    bCylLowReg;             // IDE low order cylinder value
        UCHAR    bCylHighReg;            // IDE high order cylinder value
        UCHAR    bDriveHeadReg;          // IDE drive/head register
        UCHAR    bCommandReg;            // Actual IDE command.
        UCHAR    bReserved;                      // reserved for future use.  Must be zero.
} IDEREGS, *PIDEREGS, *LPIDEREGS;
#include <poppack.h>

//
// Valid values for the bCommandReg member of IDEREGS.
//

#define ATAPI_ID_CMD    0xA1            // Returns ID sector for ATAPI.
#define ID_CMD          0xEC            // Returns ID sector for ATA.
#define SMART_CMD       0xB0            // Performs SMART cmd.
                                        // Requires valid bFeaturesReg,
                                        // bCylLowReg, and bCylHighReg

//
// Cylinder register defines for SMART command
//

#define SMART_CYL_LOW   0x4F
#define SMART_CYL_HI    0xC2


//
// SENDCMDINPARAMS contains the input parameters for the
// Send Command to Drive function.
//

#include <pshpack1.h>
typedef struct _SENDCMDINPARAMS {
        ULONG   cBufferSize;            // Buffer size in bytes
        IDEREGS irDriveRegs;            // Structure with drive register values.
        UCHAR    bDriveNumber;           // Physical drive number to send
                                                                // command to (0,1,2,3).
        UCHAR    bReserved[3];           // Reserved for future expansion.
        ULONG   dwReserved[4];          // For future use.
        UCHAR    bBuffer[1];                     // Input buffer.
} SENDCMDINPARAMS, *PSENDCMDINPARAMS, *LPSENDCMDINPARAMS;
#include <poppack.h>

//
// Status returned from driver
//

#include <pshpack1.h>
typedef struct _DRIVERSTATUS {
        UCHAR    bDriverError;           // Error code from driver,
                                                                // or 0 if no error.
        UCHAR    bIDEError;                      // Contents of IDE Error register.
                                                                // Only valid when bDriverError
                                                                // is SMART_IDE_ERROR.
        UCHAR    bReserved[2];           // Reserved for future expansion.
        ULONG   dwReserved[2];          // Reserved for future expansion.
} DRIVERSTATUS, *PDRIVERSTATUS, *LPDRIVERSTATUS;
#include <poppack.h>

//
// bDriverError values
//

#define SMART_NO_ERROR          0       // No error
#define SMART_IDE_ERROR         1       // Error from IDE controller
#define SMART_INVALID_FLAG      2       // Invalid command flag
#define SMART_INVALID_COMMAND   3       // Invalid command byte
#define SMART_INVALID_BUFFER    4       // Bad buffer (null, invalid addr..)
#define SMART_INVALID_DRIVE     5       // Drive number not valid
#define SMART_INVALID_IOCTL     6       // Invalid IOCTL
#define SMART_ERROR_NO_MEM      7       // Could not lock user's buffer
#define SMART_INVALID_REGISTER  8       // Some IDE Register not valid
#define SMART_NOT_SUPPORTED     9       // Invalid cmd flag set
#define SMART_NO_IDE_DEVICE     10      // Cmd issued to device not present
                                        // although drive number is valid
//
// SMART sub commands for execute offline diags
//
#define SMART_OFFLINE_ROUTINE_OFFLINE       0
#define SMART_SHORT_SELFTEST_OFFLINE        1
#define SMART_EXTENDED_SELFTEST_OFFLINE     2
#define SMART_ABORT_OFFLINE_SELFTEST        127
#define SMART_SHORT_SELFTEST_CAPTIVE        129
#define SMART_EXTENDED_SELFTEST_CAPTIVE     130


#include <pshpack1.h>
typedef struct _SENDCMDOUTPARAMS {
        ULONG                   cBufferSize;            // Size of bBuffer in bytes
        DRIVERSTATUS            DriverStatus;           // Driver status structure.
        UCHAR                   bBuffer[1];             // Buffer of arbitrary length in which to store the data read from the                                                                                  // drive.
} SENDCMDOUTPARAMS, *PSENDCMDOUTPARAMS, *LPSENDCMDOUTPARAMS;
#include <poppack.h>


#define READ_ATTRIBUTE_BUFFER_SIZE  512
#define IDENTIFY_BUFFER_SIZE        512
#define READ_THRESHOLD_BUFFER_SIZE  512
#define SMART_LOG_SECTOR_SIZE       512

//
// Feature register defines for SMART "sub commands"
//

#define READ_ATTRIBUTES         0xD0
#define READ_THRESHOLDS         0xD1
#define ENABLE_DISABLE_AUTOSAVE 0xD2
#define SAVE_ATTRIBUTE_VALUES   0xD3
#define EXECUTE_OFFLINE_DIAGS   0xD4
#define SMART_READ_LOG          0xD5
#define SMART_WRITE_LOG         0xd6
#define ENABLE_SMART            0xD8
#define DISABLE_SMART           0xD9
#define RETURN_SMART_STATUS     0xDA
#define ENABLE_DISABLE_AUTO_OFFLINE 0xDB
#endif /* _WIN32_WINNT >= 0x0400 */

// end_winioctl


#if (NTDDI_VERSION >= NTDDI_VISTA)

//
// IOCTLs to query and modify attributes
// associated with partitions. These are
// persisted within the partition table.
//

#define IOCTL_DISK_GET_PARTITION_ATTRIBUTES CTL_CODE(IOCTL_DISK_BASE, 0x003a, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_DISK_SET_PARTITION_ATTRIBUTES CTL_CODE(IOCTL_DISK_BASE, 0x003b, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// IOCTL_DISK_GET_PARTITION_ATTRIBUTES
//
// Input Buffer:
//     None
//
// Output Buffer:
//     Structure of type GET_PARTITION_ATTRIBUTES
//

typedef struct _GET_PARTITION_ATTRIBUTES {

    //
    // Specifies the size of the
    // structure for versioning.
    //
    ULONG Version;

    //
    // For alignment purposes.
    //
    ULONG Reserved1;

    //
    // Specifies the partition
    // attributes.
    //
    ULONGLONG Attributes;

} GET_PARTITION_ATTRIBUTES, *PGET_PARTITION_ATTRIBUTES;

//
// IOCTL_DISK_SET_PARTITION_ATTRIBUTES
//
// Input Buffer:
//     Structure of type SET_PARTITION_ATTRIBUTES
//
// Output Buffer:
//     None
//

typedef struct _SET_PARTITION_ATTRIBUTES {

    //
    // Specifies the size of the
    // structure for versioning.
    //
    ULONG Version;

    //
    // Indicates whether to remember
    // these settings across reboots
    // or not.
    //
    BOOLEAN Persist;

    //
    // For alignment purposes.
    //
    BOOLEAN Reserved1[3];

    //
    // Specifies the new attributes.
    //
    ULONGLONG Attributes;

    //
    // Specifies the attributes
    // that are being modified.
    //
    ULONGLONG AttributesMask;

} SET_PARTITION_ATTRIBUTES, *PSET_PARTITION_ATTRIBUTES;


// begin_winioctl
//
// IOCTLs to query and modify attributes
// associated with the given disk. These
// are persisted within the registry.
//

#define IOCTL_DISK_GET_DISK_ATTRIBUTES      CTL_CODE(IOCTL_DISK_BASE, 0x003c, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_DISK_SET_DISK_ATTRIBUTES      CTL_CODE(IOCTL_DISK_BASE, 0x003d, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

#define DISK_ATTRIBUTE_OFFLINE              0x0000000000000001
#define DISK_ATTRIBUTE_READ_ONLY            0x0000000000000002
// end_winioctl
#define DISK_ATTRIBUTE_HIDDEN               0x0000000000000004
#define DISK_ATTRIBUTE_MAINTENANCE          0x0000000000000008
#define DISK_ATTRIBUTE_SPACES_BYPASS        0x0000000000000010
#define DISK_ATTRIBUTE_IGNORE_CONFLICTS     0x0000000000000020
#define DISK_ATTRIBUTE_PASSTHROUGH          0x0000000000000040

// begin_winioctl
//
// IOCTL_DISK_GET_DISK_ATTRIBUTES
//
// Input Buffer:
//     None
//
// Output Buffer:
//     Structure of type GET_DISK_ATTRIBUTES
//

typedef struct _GET_DISK_ATTRIBUTES {

    //
    // Specifies the size of the
    // structure for versioning.
    //
    ULONG Version;

    //
    // Reserved. Must ignore.
    //
    ULONG Reserved1;

    //
    // Specifies the attributes
    // associated with the disk.
    //
    ULONGLONG Attributes;

} GET_DISK_ATTRIBUTES, *PGET_DISK_ATTRIBUTES;

//
// IOCTL_DISK_SET_DISK_ATTRIBUTES
//
// Input Buffer:
//     Structure of type SET_DISK_ATTRIBUTES
//
// Output Buffer:
//     None
//

typedef struct _SET_DISK_ATTRIBUTES {

    //
    // Specifies the size of the
    // structure for versioning.
    //
    ULONG Version;

    //
    // Indicates whether to remember
    // these settings across reboots
    // or not.
    //
    BOOLEAN Persist;
// end_winioctl

    //
    // Indicates whether the ownership
    // taken earlier is being released.
    //
    BOOLEAN RelinquishOwnership;

    //
    // For alignment purposes.
    //
    BOOLEAN Reserved1[2];

    /*
        The following field obfuscates
        the above two for winioctl.h
// begin_winioctl

    //
    // Reserved. Must set to zero.
    //
    UCHAR Reserved1[3];
// end_winioctl
    */
// begin_winioctl

    //
    // Specifies the new attributes.
    //
    ULONGLONG Attributes;

    //
    // Specifies the attributes
    // that are being modified.
    //
    ULONGLONG AttributesMask;
// end_winioctl

    //
    // Specifies an identifier to be
    // associated  with  the caller.
    // This setting is not persisted
    // across reboots.
    //
    GUID Owner;

    /*
        The following field obfuscates
        the GUID above for winioctl.h
// begin_winioctl

    //
    // Reserved. Must set to zero.
    //
    ULONG Reserved2[4];
// end_winioctl
    */
// begin_winioctl

} SET_DISK_ATTRIBUTES, *PSET_DISK_ATTRIBUTES;
// end_winioctl


//
// IOCTL to determine if  the  disk is
// owned by the cluster service or not.
//

#define IOCTL_DISK_IS_CLUSTERED             CTL_CODE(IOCTL_DISK_BASE, 0x003e, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// IOCTL_DISK_IS_CLUSTERED
//
// Input Buffer:
//     None
//
// Output Buffer:
//     Structure of type BOOLEAN
//


//
// IOCTLs to query and modify the current
// SAN settings. For instance, the policy
// associated with newly discovered disks.
//

#define IOCTL_DISK_GET_SAN_SETTINGS         CTL_CODE(IOCTL_DISK_BASE, 0x0080, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DISK_SET_SAN_SETTINGS         CTL_CODE(IOCTL_DISK_BASE, 0x0081, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// IOCTL_DISK_GET_SAN_SETTINGS
//
// Input Buffer:
//     None
//
// Output Buffer:
//     Structure of type DISK_SAN_SETTINGS
//

//
// IOCTL_DISK_SET_SAN_SETTINGS
//
// Input Buffer:
//     Structure of type DISK_SAN_SETTINGS
//
// Output Buffer:
//     None
//

typedef enum _DISK_SAN_POLICY {

    DiskSanPolicyUnknown,
    DiskSanPolicyOnline,
    DiskSanPolicyOfflineShared,
    DiskSanPolicyOffline,
    DiskSanPolicyOfflineInternal,
    DiskSanPolicyPassthrough,
    DiskSanPolicyMax

} DISK_SAN_POLICY, *PDISK_SAN_POLICY;

typedef struct _DISK_SAN_SETTINGS {

    //
    // Specifies the size of the
    // structure for versioning.
    //
    ULONG Version;

    //
    // Specifies the policy to be
    // applied to all new disks.
    //
    DISK_SAN_POLICY SanPolicy;

} DISK_SAN_SETTINGS, *PDISK_SAN_SETTINGS;


//
// IOCTLs to query and modify the context
// associated with snapshot disks created
// in hardware.
//

#define IOCTL_DISK_GET_SNAPSHOT_INFO        CTL_CODE(IOCTL_DISK_BASE, 0x0082, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DISK_SET_SNAPSHOT_INFO        CTL_CODE(IOCTL_DISK_BASE, 0x0083, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

// begin_winioctl

#define IOCTL_DISK_RESET_SNAPSHOT_INFO      CTL_CODE(IOCTL_DISK_BASE, 0x0084, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

// end_winioctl

//
// IOCTL_DISK_GET_SNAPSHOT_INFO
//
// Input Buffer:
//     None
//
// Output Buffer:
//     Structure of type DISK_SNAPSHOT_INFO
//

//
// IOCTL_DISK_SET_SNAPSHOT_INFO
//
// Input Buffer:
//     Structure of type DISK_SNAPSHOT_INFO
//
// Output Buffer:
//     None
//

typedef enum _DISK_SNAPSHOT_STATE {

    DiskSnapshotNormalDisk,
    DiskSnapshotSnapshotCheckRequired,
    DiskSnapshotPreSnapshot,
    DiskSnapshotSnapshotDisk

} DISK_SNAPSHOT_STATE, *PDISK_SNAPSHOT_STATE;

typedef struct _DISK_SNAPSHOT_INFO {

    //
    // Specifies the size of the
    // structure for versioning.
    //
    ULONG Version;

    //
    // Specifies the state that this
    // disk is in or the state to be
    // transitioned to.
    //
    DISK_SNAPSHOT_STATE State;

    //
    // Specifies a unique id  that
    // represents all of the disks
    // involved in this snapshot.
    //
    GUID SnapshotSetId;

    //
    // Specifies a unique id  that
    // represents this snapshot.
    //
    GUID SnapshotId;

    //
    // Specifies a unique id  that
    // represents the logical unit
    // whose snapshot was taken.
    //
    GUID LunId;

    //
    // Specifies the time when this
    // snapshot was taken.
    //
    LARGE_INTEGER CreationTimeStamp;

    //
    // Specifies the number of times
    // that  this  snapshot has been
    // imported.
    //
    ULONG ImportCount;

    //
    // Specifies attributes that are
    // associated with this snapshot.
    //
    ULONG Flags;

    //
    // Specifies the size in bytes of
    // the following field.
    //
    ULONG AdditionalDataSize;

    //
    // Specifies disk meta data that
    // needs to be  restored  in the
    // event of a fast recovery.
    //
    UCHAR AdditionalData[ANYSIZE_ARRAY];

} DISK_SNAPSHOT_INFO, *PDISK_SNAPSHOT_INFO;

#endif  // NTDDI_VERSION >= NTDDI_VISTA


#if (NTDDI_VERSION >= NTDDI_WIN8)


//
// IOCTLs to query and modify the context
// associated with clustered disks.
//

#define IOCTL_DISK_GET_CLUSTER_INFO         CTL_CODE(IOCTL_DISK_BASE, 0x0085, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_DISK_SET_CLUSTER_INFO         CTL_CODE(IOCTL_DISK_BASE, 0x0086, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// IOCTL_DISK_GET_CLUSTER_INFO
//
// Input Buffer:
//     None
//
// Output Buffer:
//     Structure of type DISK_CLUSTER_INFO
//

//
// IOCTL_DISK_SET_CLUSTER_INFO
//
// Input Buffer:
//     Structure of type DISK_CLUSTER_INFO
//
// Output Buffer:
//     None
//

#define DISK_CLUSTER_FLAG_ENABLED              0x0000000000000001
#define DISK_CLUSTER_FLAG_CSV                  0x0000000000000002
#define DISK_CLUSTER_FLAG_IN_MAINTENANCE       0x0000000000000004
#define DISK_CLUSTER_FLAG_PNP_ARRIVAL_COMPLETE 0x0000000000000008

typedef struct _DISK_CLUSTER_INFO {

    //
    // Size of this structure serves
    // as the version
    //
    ULONG Version;

    //
    // Specifies whether or not this
    // disk  is clustered, csv,  etc
    //
    ULONGLONG Flags;

    //
    // Specifies  the flags that are
    // being modified
    //
    ULONGLONG FlagsMask;

    //
    // Indicates  whether  or  not a
    // layout change notification is
    // to be sent
    //
    BOOLEAN Notify;

} DISK_CLUSTER_INFO, *PDISK_CLUSTER_INFO;


//
// IOCTLs to determine when all of the
// volumes on a disk are ready for use
//

#define IOCTL_DISK_ARE_VOLUMES_READY    CTL_CODE(IOCTL_DISK_BASE, 0x0087, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_DISK_VOLUMES_ARE_READY    CTL_CODE(IOCTL_DISK_BASE, 0x0088, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// IOCTL_DISK_ARE_VOLUMES_READY
//
// Input Buffer:
//     None
//
// Output Buffer:
//     None
//

//
// IOCTL_DISK_VOLUMES_ARE_READY
//
// Input Buffer:
//     Structure of type ULONG
//
// Output Buffer:
//     None
//


//
// IOCTL to query disk performance counters.
// Similar to IOCTL_DISK_PERFORMANCE with support
// for flushes and non-low priority I/Os.
//

#define IOCTL_DISK_GET_PERFORMANCE_INFO     CTL_CODE(IOCTL_DISK_BASE, 0x0089, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// IOCTL_DISK_GET_PERFORMANCE_INFO
//
// Input Buffer:
//     Structure of type DISK_PERFORMANCE_PARAMETERS
//
// Output Buffer:
//     Structure of type DISK_PERFORMANCE_INFO
//

typedef enum _DISK_PERFORMANCE_TYPE {

    DiskPerformanceTypeAllPriority,
    DiskPerformanceTypeNonLowPriority,
    DiskPerformanceTypeMax

} DISK_PERFORMANCE_TYPE, *PDISK_PERFORMANCE_TYPE;

typedef struct _DISK_PERFORMANCE_PARAMETERS {

    ULONG Version;
    DISK_PERFORMANCE_TYPE Type;

} DISK_PERFORMANCE_PARAMETERS, *PDISK_PERFORMANCE_PARAMETERS;

typedef struct _DISK_PERFORMANCE_INFO {

    ULONG Version;
    DISK_PERFORMANCE_TYPE Type;
    LARGE_INTEGER QueryTime;
    LARGE_INTEGER BytesRead;
    LARGE_INTEGER BytesWritten;
    LARGE_INTEGER ReadTime;
    LARGE_INTEGER WriteTime;
    LARGE_INTEGER FlushTime;
    LARGE_INTEGER IdleTime;
    ULONG ReadCount;
    ULONG WriteCount;
    ULONG FlushCount;
    ULONG QueueDepth;
    ULONG SplitCount;

} DISK_PERFORMANCE_INFO, *PDISK_PERFORMANCE_INFO;

#endif  // NTDDI_VERSION >= NTDDI_WIN8

//
// The following device control code is for the SIMBAD simulated bad
// sector facility. See SIMBAD.H in this directory for related structures.
//

#define IOCTL_DISK_SIMBAD               CTL_CODE(IOCTL_DISK_BASE, 0x0400, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// Queue link for mapped addresses stored for unmapping.
//

typedef struct _MAPPED_ADDRESS {
    struct _MAPPED_ADDRESS *NextMappedAddress;
    PVOID MappedAddress;
    ULONG NumberOfBytes;
    LARGE_INTEGER IoAddress;
    ULONG BusNumber;
} MAPPED_ADDRESS, *PMAPPED_ADDRESS;

// begin_winioctl

#if defined(_MSC_VER)
#if (_MSC_VER >= 1200)
#pragma warning(pop)
#endif
#endif

// end_winioctl

#if defined __cplusplus && !defined __ALT_GENERATOR__
}
#endif

// begin_winioctl

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // _NTDDDISK_H_

// end_winioctl
