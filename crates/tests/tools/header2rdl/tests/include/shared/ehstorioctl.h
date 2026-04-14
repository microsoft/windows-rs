/*++
    Copyright (c) Microsoft Corporation. All rights reserved.

    Module Name:
        EhStorIoctl.h

    Abstract:
        Header file for Enhanced Storage IOCTLs

    Environment:
        Kernel mode only

    Revision History:
        05-01-07 : Created
        06-19-09 : Renamed from UsbStorIoctl.h to EhStorIoctl.h
--*/

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifndef MAX_PATH
#define MAX_PATH 260
#endif

//
// IOCTL_EHSTOR_DEVICE_GET_AUTHZ_STATE and IOCTL_EHSTOR_DEVICE_SET_AUTHZ_STATE
// parameters.
//
typedef struct tagACT_AUTHZ_STATE
{
    UCHAR   ACT;
    BOOLEAN fAuthorized;
} ACT_AUTHZ_STATE, *PACT_AUTHZ_STATE;

//
// IOCTL_EHSTOR_DEVICE_GET_QUEUE_STATE and IOCTL_EHSTOR_DEVICE_SET_QUEUE_STATE
// parameters.
//
typedef struct tagACT_QUEUE_STATE
{
    BOOLEAN fFrozen;
} ACT_QUEUE_STATE, *PACT_QUEUE_STATE;

//
// IOCTL_EHSTOR_DEVICE_SILO_COMMAND parameters.
//
typedef struct tagSILO_COMMAND
{
    UCHAR   SiloIndex;
    UCHAR   Command;
    ULONG   cbCommandBuffer;
    UCHAR   rgbCommandBuffer[ANYSIZE_ARRAY];
} SILO_COMMAND, *PSILO_COMMAND;

#define SIZE_SILO_COMMAND_HEADER FIELD_OFFSET(SILO_COMMAND, rgbCommandBuffer)

//
// IOCTL_EHSTOR_DEVICE_ENUMERATE_PDOS parameters.
//

// Enumeration
typedef enum _PDO_TYPE
{
    PDO_TYPE_UNDEFINED = 0,
    // Types either enumerated or provided as filter parameter to
    // IOCTL_EHSTOR_DEVICE_ENUMERATE_PDOS
    PDO_TYPE_DISK,
    PDO_TYPE_CONTROL,
    PDO_TYPE_SILO,
    // This type is never enumerated, only provided as a filter parameter to
    // IOCTL_EHSTOR_DEVICE_ENUMERATE_PDOS
    PDO_TYPE_THIS      = 256
} PDO_TYPE;

// Enumeration
typedef enum _PDO_STATE
{
    PDO_STATE_UNDEFINED = 0,
    PDO_STATE_STARTED,
    PDO_STATE_NOT_STARTED
} PDO_STATE;

// Bit-mask
typedef enum _PDO_CAPS
{
    PDO_CAPABILITY_UNDEFINED    = 0,
    PDO_CAPABILITY_INC512_SET   = 1,
    PDO_CAPABILITY_INC512_CLEAR = 2
} PDO_CAPS;

typedef struct _ENUM_PDO_ENTRY
{
    UCHAR   type;
    UCHAR   state;
    UCHAR   capabilities;
    ULONG   ulSTID;
    UCHAR   bSpecificationMajor;
    UCHAR   bSpecificationMinor;
    UCHAR   bImplementationMajor;
    UCHAR   bImplementationMinor;
    WCHAR   wszDeviceInstancePath[(2 * MAX_PATH) + 1];
} ENUM_PDO_ENTRY, *PENUM_PDO_ENTRY;

typedef struct _ENUM_PDO_RESULTS
{
    ULONG           cEntries;
    ENUM_PDO_ENTRY  rgEntries[ANYSIZE_ARRAY];
} ENUM_PDO_RESULTS, *PENUM_PDO_RESULTS;

#define SIZE_ENUM_PDO_RESULTS_HEADER FIELD_OFFSET(ENUM_PDO_RESULTS, rgEntries)

//
// IOCTL_EHSTOR_DEVICE_QUERY_PROPERTIES parameters.
//
typedef struct _EHSTOR_DEVICE_PROPERTIES
{
    ULONG  StructSize;
    ULONG  BytesPerSector;
} EHSTOR_DEVICE_PROPERTIES, *PEHSTOR_DEVICE_PROPERTIES;

//
// IOCTL_EHSTOR_DRIVER_REPORT_CAPABILITIES parameters.
//
typedef struct _SILO_DRIVER_CAPABILITIES
{
    ULONG  StructSize;
    ULONG  Capabilities;
    ULONG  MaxLbaFilterCount;
    ULONG  RedirectedIoctlListCount;
    ULONG  RedirectedIoctlListOffset;
} SILO_DRIVER_CAPABILITIES, *PSILO_DRIVER_CAPABILITIES;

// Bit-mask
typedef enum _SILO_DRIVER_CAPS
{
    CAP_ON_DEMAND_AUTHENTICATION = (1 << 0),    //The silo driver supports
                                                //on-demand authentication and
                                                //deauthentication
    CAP_BANDING_SUPPORT = (1 << 1)              //The silo driver supports the
                                                //concept of different states
                                                //for different LBA bands
} SILO_DRIVER_CAPS;

//
// IOCTL_EHSTOR_DRIVER_UPDATE_LBA_FILTER_TABLE parameters.
//
typedef struct _LBA_FILTER_TABLE_ENTRY
{
    ULARGE_INTEGER  StartLba;
    ULARGE_INTEGER  LbaCount;
    BOOLEAN         ReadLock;
    BOOLEAN         WriteLock;
} LBA_FILTER_TABLE_ENTRY, *PLBA_FILTER_TABLE_ENTRY;

typedef struct _LBA_FILTER_TABLE
{
    ULONG           StructSize;
    BOOLEAN         GlobalReadLock;
    LONG            Reserved1;
    BOOLEAN         GlobalWriteLock;
    LONG            Reserved2;
    ULONG           LbaFilterCount;
    ULONG           LbaFilterSize;
    ULONG           LbaFiltersOffset;
} LBA_FILTER_TABLE, *PLBA_FILTER_TABLE;

//
// IOCTL_EHSTOR_DRIVER_PERFORM_AUTHZ parameters.
//
typedef struct _AUTHZ_STATE
{
    ULONG AuthzState;
} AUTHZ_STATE, *PAUTHZ_STATE;

#define AUTHZSTATE_AUTHENTICATE         0x00000001
#define AUTHZSTATE_CLEAR_AUTHKEY_CACHE  0x00000002

//
// Enhanced Storage IOCTL codes
//
#define IOCTL_EHSTOR_DEVICE_SILO_COMMAND        \
        CTL_CODE(IOCTL_STORAGE_BASE, 0x503, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_EHSTOR_DEVICE_ENUMERATE_PDOS      \
        CTL_CODE(IOCTL_STORAGE_BASE, 0x504, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_EHSTOR_DEVICE_SET_AUTHZ_STATE     \
        CTL_CODE(IOCTL_STORAGE_BASE, 0x505, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_EHSTOR_DEVICE_QUERY_PROPERTIES    \
        CTL_CODE(IOCTL_STORAGE_BASE, 0x506, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_EHSTOR_DEVICE_GET_QUEUE_STATE     \
        CTL_CODE(IOCTL_STORAGE_BASE, 0x507, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_EHSTOR_DEVICE_SET_QUEUE_STATE     \
        CTL_CODE(IOCTL_STORAGE_BASE, 0x508, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_EHSTOR_DEVICE_GET_AUTHZ_STATE     \
        CTL_CODE(IOCTL_STORAGE_BASE, 0x509, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_EHSTOR_DRIVER_REPORT_CAPABILITIES \
        CTL_CODE(IOCTL_STORAGE_BASE, 0x510, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_EHSTOR_DRIVER_UPDATE_LBA_FILTER_TABLE \
        CTL_CODE(IOCTL_STORAGE_BASE, 0x511, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_EHSTOR_DRIVER_PERFORM_AUTHZ       \
        CTL_CODE(IOCTL_STORAGE_BASE, 0x512, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// Enhanced Storage GUIDs
//
DEFINE_GUID(GUID_EHSTOR_SILO_INTERFACE,
            0x7c2bcf57, 0x2bea, 0x46da, 0xad, 0x26, 0x78, 0xfd, 0xc8, 0x3c, 0xee, 0x46);
DEFINE_GUID(GUID_EHSTOR_CONTROL_INTERFACE,
            0x4f40006f, 0xb933, 0x4550, 0xb5, 0x32, 0x2b, 0x58, 0xce, 0xe6, 0x14, 0xd3);

// Legacy USBSTOR definitions left for compatibility reasons
DEFINE_GUID(GUID_USBSTOR_EHSTOR_SILO_INTERFACE,
            0x7c2bcf57, 0x2bea, 0x46da, 0xad, 0x26, 0x78, 0xfd, 0xc8, 0x3c, 0xee, 0x46);
DEFINE_GUID(GUID_USBSTOR_EHSTOR_CONTROL_INTERFACE,
            0x4f40006f, 0xb933, 0x4550, 0xb5, 0x32, 0x2b, 0x58, 0xce, 0xe6, 0x14, 0xd3);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


