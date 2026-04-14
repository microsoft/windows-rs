/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    diskguid.h

Abstract:

    GPT disk GUIDs.

Revision History:

--*/
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


//
// GPT Partition Type GUIDs
//
// need these GUIDs outside conditional includes so that user can
//   #include <ntdddisk.h> in precompiled header
//   #include <initguid.h> in a single source file
//   #include <ntdddisk.h> in that source file a second time to instantiate the GUIDs
//

#ifdef DEFINE_GUID

//
// Make sure FAR is defined...
//
#ifndef FAR
#ifdef _WIN32
#define FAR
#else
#define FAR _far
#endif
#endif


//
// Define the GPT partition guids known by disk drivers and volume managers.
//

DEFINE_GUID(PARTITION_BASIC_DATA_GUID,             0xEBD0A0A2L, 0xB9E5, 0x4433, 0x87, 0xC0, 0x68, 0xB6, 0xB7, 0x26, 0x99, 0xC7);    // Basic data partition
DEFINE_GUID(PARTITION_BSP_GUID,                    0x57434F53L, 0x4DF9, 0x45B9, 0x8E, 0x9E, 0x23, 0x70, 0xF0, 0x06, 0x45, 0x7C);    // BSP partition
DEFINE_GUID(PARTITION_CLUSTER_GUID,                0xDB97DBA9L, 0x0840, 0x4BAE, 0x97, 0xF0, 0xFF, 0xB9, 0xA3, 0x27, 0xC7, 0xE1);    // Cluster metadata partition
DEFINE_GUID(PARTITION_DPP_GUID,                    0x57434F53L, 0x94CB, 0x43F0, 0xA5, 0x33, 0xD7, 0x3C, 0x10, 0xCF, 0xA5, 0x7D);    // DPP partition
DEFINE_GUID(PARTITION_ENTRY_UNUSED_GUID,           0x00000000L, 0x0000, 0x0000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);    // Entry unused
DEFINE_GUID(PARTITION_LDM_DATA_GUID,               0xAF9B60A0L, 0x1431, 0x4F62, 0xBC, 0x68, 0x33, 0x11, 0x71, 0x4A, 0x69, 0xAD);    // Logical Disk Manager data partition
DEFINE_GUID(PARTITION_LDM_METADATA_GUID,           0x5808C8AAL, 0x7E8F, 0x42E0, 0x85, 0xD2, 0xE1, 0xE9, 0x04, 0x34, 0xCF, 0xB3);    // Logical Disk Manager metadata partition
DEFINE_GUID(PARTITION_MAIN_OS_GUID,                0x57434F53L, 0x8F45, 0x405E, 0x8A, 0x23, 0x18, 0x6D, 0x8A, 0x43, 0x30, 0xD3);    // Main OS partition
DEFINE_GUID(PARTITION_MSFT_RECOVERY_GUID,          0xDE94BBA4L, 0x06D1, 0x4D40, 0xA1, 0x6A, 0xBF, 0xD5, 0x01, 0x79, 0xD6, 0xAC);    // Microsoft recovery partition
DEFINE_GUID(PARTITION_MSFT_RESERVED_GUID,          0xE3C9E316L, 0x0B5C, 0x4DB8, 0x81, 0x7D, 0xF9, 0x2D, 0xF0, 0x02, 0x15, 0xAE);    // Microsoft reserved space
DEFINE_GUID(PARTITION_MSFT_SNAPSHOT_GUID,          0xCADDEBF1L, 0x4400, 0x4DE8, 0xB1, 0x03, 0x12, 0x11, 0x7D, 0xCF, 0x3C, 0xCF);    // Microsoft shadow copy partition
DEFINE_GUID(PARTITION_OS_DATA_GUID,                0x57434F53L, 0x23F2, 0x44D5, 0xA8, 0x30, 0x67, 0xBB, 0xDA, 0xA6, 0x09, 0xF9);    // OS data partition
DEFINE_GUID(PARTITION_PATCH_GUID,                  0x8967A686L, 0x96AA, 0x6AA8, 0x95, 0x89, 0xA8, 0x42, 0x56, 0x54, 0x10, 0x90);    // Patch partition
DEFINE_GUID(PARTITION_PRE_INSTALLED_GUID,          0x57434F53L, 0x7FE0, 0x4196, 0x9B, 0x42, 0x42, 0x7B, 0x51, 0x64, 0x34, 0x84);    // PreInstalled partition
DEFINE_GUID(PARTITION_SBL_CACHE_SSD_GUID,          0xeeff8352L, 0xdd2a, 0x44db, 0xae, 0x83, 0xbe, 0xe1, 0xcf, 0x74, 0x81, 0xdc);    // Storage Bus Layer Cache partition
DEFINE_GUID(PARTITION_SBL_CACHE_SSD_RESERVED_GUID, 0xdcc0c7c1L, 0x55ad, 0x4f17, 0x9d, 0x43, 0x4b, 0xc7, 0x76, 0xe0, 0x11, 0x7e);    // Storage Bus Layer Cache partition
DEFINE_GUID(PARTITION_SBL_CACHE_HDD_GUID,          0x03aaa829L, 0xebfc, 0x4e7e, 0xaa, 0xc9, 0xc4, 0xd7, 0x6c, 0x63, 0xb2, 0x4b);    // Storage Bus Layer Cache partition
DEFINE_GUID(PARTITION_SERVICING_FILES_GUID,        0x57434F53L, 0x432E, 0x4014, 0xAE, 0x4C, 0x8D, 0xEA, 0xA9, 0xC0, 0x00, 0x6A);    // Servicing files partition
DEFINE_GUID(PARTITION_SERVICING_METADATA_GUID,     0x57434F53L, 0xC691, 0x4A05, 0xBB, 0x4E, 0x70, 0x3D, 0xAF, 0xD2, 0x29, 0xCE);    // Servicing metadata partition
DEFINE_GUID(PARTITION_SERVICING_RESERVE_GUID,      0x57434F53L, 0x4B81, 0x460B, 0xA3, 0x19, 0xFF, 0xB6, 0xFE, 0x13, 0x6D, 0x14);    // Servicing reserve partition
DEFINE_GUID(PARTITION_SERVICING_STAGING_ROOT_GUID, 0x57434F53L, 0xE84d, 0x4E84, 0xAA, 0xF3, 0xEC, 0xBB, 0xBD, 0x04, 0xB9, 0xDF);    // Servicing staging root partition
DEFINE_GUID(PARTITION_SPACES_GUID,                 0xE75CAF8FL, 0xF680, 0x4CEE, 0xAF, 0xA3, 0xB0, 0x01, 0xE5, 0x6E, 0xFC, 0x2D);    // Storage Spaces protective partition
DEFINE_GUID(PARTITION_SPACES_DATA_GUID,            0xE7ADDCB4L, 0xDC34, 0x4539, 0x9A, 0x76, 0xEB, 0xBD, 0x07, 0xBE, 0x6F, 0x7E);    // Storage Spaces protective partition
DEFINE_GUID(PARTITION_SYSTEM_GUID,                 0xC12A7328L, 0xF81F, 0x11D2, 0xBA, 0x4B, 0x00, 0xA0, 0xC9, 0x3E, 0xC9, 0x3B);    // EFI system partition
DEFINE_GUID(PARTITION_WINDOWS_SYSTEM_GUID,         0x57434F53L, 0xE3E3, 0x4631, 0xA5, 0xC5, 0x26, 0xD2, 0x24, 0x38, 0x73, 0xAA);    // Windows system partition
#endif

#ifdef __cplusplus
#define IsEqualPartitionType         IsEqualGUID
#else
#define IsEqualPartitionType(_a, _b) IsEqualGUID(&(_a), &(_b))
#endif

#define IsRecognizedGptPartition(_t) (                                    \
    IsEqualPartitionType((_t), PARTITION_BSP_GUID)                    ||  \
    IsEqualPartitionType((_t), PARTITION_DPP_GUID)                    ||  \
    IsEqualPartitionType((_t), PARTITION_BASIC_DATA_GUID)             ||  \
    IsEqualPartitionType((_t), PARTITION_MAIN_OS_GUID)                ||  \
    IsEqualPartitionType((_t), PARTITION_MSFT_RECOVERY_GUID)          ||  \
    IsEqualPartitionType((_t), PARTITION_OS_DATA_GUID)                ||  \
    IsEqualPartitionType((_t), PARTITION_PRE_INSTALLED_GUID)          ||  \
    IsEqualPartitionType((_t), PARTITION_SERVICING_FILES_GUID)        ||  \
    IsEqualPartitionType((_t), PARTITION_SERVICING_METADATA_GUID)     ||  \
    IsEqualPartitionType((_t), PARTITION_SERVICING_RESERVE_GUID)      ||  \
    IsEqualPartitionType((_t), PARTITION_SERVICING_STAGING_ROOT_GUID) ||  \
    IsEqualPartitionType((_t), PARTITION_WINDOWS_SYSTEM_GUID))

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

