/*++

Copyright (c) Microsoft Corporation.  All Rights Reserved.

Module Name:

    emi.h

Abstract:

    This header defines the standard IOCTL interface to an energy meter device.

--*/

#if !defined(_EMI_)
#define _EMI_

#ifdef _MSC_VER
#pragma once
#endif  // _MSC_VER

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//
// Energy Metering Device Interface
// {45BD8344-7ED6-49cf-A440-C276C933B053}
//

DEFINE_GUID(GUID_DEVICE_ENERGY_METER, 
0x45bd8344, 0x7ed6, 0x49cf, 0xa4, 0x40, 0xc2, 0x76, 0xc9, 0x33, 0xb0, 0x53);

#define IOCTL_EMI_GET_VERSION        CTL_CODE(FILE_DEVICE_UNKNOWN, 0, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_EMI_GET_METADATA_SIZE  CTL_CODE(FILE_DEVICE_UNKNOWN, 1, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_EMI_GET_METADATA       CTL_CODE(FILE_DEVICE_UNKNOWN, 2, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_EMI_GET_MEASUREMENT    CTL_CODE(FILE_DEVICE_UNKNOWN, 3, METHOD_BUFFERED, FILE_READ_ACCESS)

//
// The maximum length for a name parameter including the null terminator.
//

#define EMI_NAME_MAX                        16

#define EMI_VERSION_V1                       1
#define EMI_VERSION_V2                       2

typedef enum
{
    EmiMeasurementUnitPicowattHours
} EMI_MEASUREMENT_UNIT;

typedef struct
{
    USHORT EmiVersion;
} EMI_VERSION;

typedef struct
{
    ULONG MetadataSize;
} EMI_METADATA_SIZE;

typedef struct
{
    ULONGLONG AbsoluteEnergy;
    ULONGLONG AbsoluteTime;
} EMI_CHANNEL_MEASUREMENT_DATA;

//
// V1 Interface Definition
//

typedef struct
{
    EMI_MEASUREMENT_UNIT MeasurementUnit; 
    WCHAR HardwareOEM[EMI_NAME_MAX];
    WCHAR HardwareModel[EMI_NAME_MAX];
    USHORT HardwareRevision;
    USHORT MeteredHardwareNameSize;
    WCHAR MeteredHardwareName[ANYSIZE_ARRAY];
} EMI_METADATA_V1;

typedef EMI_CHANNEL_MEASUREMENT_DATA EMI_MEASUREMENT_DATA_V1;

//
// Backwards Compatability Typedefs
//

typedef EMI_METADATA_V1 EMI_METADATA;
typedef EMI_MEASUREMENT_DATA_V1 EMI_MEASUREMENT_DATA;

//
// V2 Interface Definition
//

typedef struct
{
    EMI_MEASUREMENT_UNIT MeasurementUnit;
    USHORT ChannelNameSize;
    WCHAR ChannelName[ANYSIZE_ARRAY];
} EMI_CHANNEL_V2;

typedef struct
{
    WCHAR HardwareOEM[EMI_NAME_MAX];
    WCHAR HardwareModel[EMI_NAME_MAX];
    USHORT HardwareRevision;
    USHORT ChannelCount;
    EMI_CHANNEL_V2 Channels[ANYSIZE_ARRAY];
} EMI_METADATA_V2;

typedef struct
{
    EMI_CHANNEL_MEASUREMENT_DATA ChannelData[ANYSIZE_ARRAY];
} EMI_MEASUREMENT_DATA_V2;

#define EMI_CHANNEL_V2_LENGTH(_ChannelNameSize) \
    (FIELD_OFFSET(EMI_CHANNEL_V2, ChannelName) + (_ChannelNameSize))

#define EMI_CHANNEL_V2_NEXT_CHANNEL(_Channel) \
    ((EMI_CHANNEL_V2*)((PUCHAR)(_Channel) + \
        EMI_CHANNEL_V2_LENGTH((_Channel)->ChannelNameSize)))

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE) 

#endif // !defined(_EMI_)

