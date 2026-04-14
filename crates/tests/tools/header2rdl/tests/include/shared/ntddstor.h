/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ntddstor.h

Abstract:

    This is the include file that defines all common constants and types
    accessing the storage class drivers

--*/

#include <winapifamily.h>



#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifndef RtlCopyMemory
#define RtlCopyMemory(Destination,Source,Length) memcpy((Destination),(Source),(Length))
#endif

#ifndef RtlZeroMemory
#define RtlZeroMemory(Destination,Length) memset((Destination),0,(Length))
#endif

//
// Interface GUIDs
//
// need these GUIDs outside conditional includes so that user can
//   #include <ntddstor.h> in precompiled header
//   #include <initguid.h> in a single source file
//   #include <ntddstor.h> in that source file a second time to instantiate the GUIDs
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

// begin_wioctlguids

DEFINE_GUID(GUID_DEVINTERFACE_DISK,                   0x53f56307L, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID(GUID_DEVINTERFACE_CDROM,                  0x53f56308L, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID(GUID_DEVINTERFACE_PARTITION,              0x53f5630aL, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID(GUID_DEVINTERFACE_TAPE,                   0x53f5630bL, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID(GUID_DEVINTERFACE_WRITEONCEDISK,          0x53f5630cL, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID(GUID_DEVINTERFACE_VOLUME,                 0x53f5630dL, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID(GUID_DEVINTERFACE_MEDIUMCHANGER,          0x53f56310L, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID(GUID_DEVINTERFACE_FLOPPY,                 0x53f56311L, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID(GUID_DEVINTERFACE_CDCHANGER,              0x53f56312L, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID(GUID_DEVINTERFACE_STORAGEPORT,            0x2accfe60L, 0xc130, 0x11d2, 0xb0, 0x82, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID(GUID_DEVINTERFACE_VMLUN,                  0x6f416619L, 0x9f29, 0x42a5, 0xb2, 0x0b, 0x37, 0xe2, 0x19, 0xca, 0x02, 0xb0);
DEFINE_GUID(GUID_DEVINTERFACE_SES,                    0x1790c9ecL, 0x47d5, 0x4df3, 0xb5, 0xaf, 0x9a, 0xdf, 0x3c, 0xf2, 0x3e, 0x48);
DEFINE_GUID(GUID_DEVINTERFACE_ZNSDISK,                0xb87941c5L, 0xffdb, 0x43c7, 0xb6, 0xb1, 0x20, 0xb6, 0x32, 0xf0, 0xb1, 0x09);

#define  WDI_STORAGE_PREDICT_FAILURE_DPS_GUID        {0xe9f2d03aL, 0x747c, 0x41c2, {0xbb, 0x9a, 0x02, 0xc6, 0x2b, 0x6d, 0x5f, 0xcb}};

//
// Interfaces to discover devices that are
// not reported  through conventional APIs
//

DEFINE_GUID(GUID_DEVINTERFACE_HIDDEN_DISK,            0x7fccc86cL, 0x228a, 0x40ad, 0x8a, 0x58, 0xf5, 0x90, 0xaf, 0x7b, 0xfd, 0xce);
DEFINE_GUID(GUID_DEVINTERFACE_SERVICE_VOLUME,         0x6ead3d82L, 0x25ec, 0x46bc, 0xb7, 0xfd, 0xc1, 0xf0, 0xdf, 0x8f, 0x50, 0x37);
DEFINE_GUID(GUID_DEVINTERFACE_HIDDEN_VOLUME,          0x7f108a28L, 0x9833, 0x4b3b, 0xb7, 0x80, 0x2c, 0x6b, 0x5f, 0xa5, 0xc0, 0x62);

//
// Interface to register for RPMB commands
//

DEFINE_GUID(GUID_DEVINTERFACE_UNIFIED_ACCESS_RPMB,    0x27447c21L, 0xbcc3, 0x4d07, 0xa0, 0x5b, 0xa3, 0x39, 0x5b, 0xb4, 0xee, 0xe7);

// end_wioctlguids

// begin_wioctlobsoleteguids

#define DiskClassGuid               GUID_DEVINTERFACE_DISK
#define CdRomClassGuid              GUID_DEVINTERFACE_CDROM
#define PartitionClassGuid          GUID_DEVINTERFACE_PARTITION
#define TapeClassGuid               GUID_DEVINTERFACE_TAPE
#define WriteOnceDiskClassGuid      GUID_DEVINTERFACE_WRITEONCEDISK
#define VolumeClassGuid             GUID_DEVINTERFACE_VOLUME
#define MediumChangerClassGuid      GUID_DEVINTERFACE_MEDIUMCHANGER
#define FloppyClassGuid             GUID_DEVINTERFACE_FLOPPY
#define CdChangerClassGuid          GUID_DEVINTERFACE_CDCHANGER
#define StoragePortClassGuid        GUID_DEVINTERFACE_STORAGEPORT

#define HiddenVolumeClassGuid       GUID_DEVINTERFACE_HIDDEN_VOLUME

// end_wioctlobsoleteguids

// begin_tcioctlguids

//
// GUID, identifying crash dump section, containing device information or driver information. These GUIDs are used both to identify secondary dump section in the crashdump
// and in IOCTL to identify why section is requested by the user mode application
//

// /* d8e2592f-1aab-4d56-a746-1f7585df40f4 */
DEFINE_GUID(GUID_DEVICEDUMP_STORAGE_DEVICE,             0xd8e2592f,0x1aab,0x4d56,0xa7, 0x46, 0x1f, 0x75, 0x85, 0xdf, 0x40, 0xf4);

//  /* da82441d-7142-4bc1-b844-0807c5a4b67f */
DEFINE_GUID(GUID_DEVICEDUMP_DRIVER_STORAGE_PORT,        0xda82441d,0x7142,0x4bc1,0xb8, 0x44, 0x08, 0x07, 0xc5, 0xa4, 0xb6, 0x7f);

// end_tcioctlguids

#endif


//
// Interface DEVPROPKEY
//
// need these DEVPROPKEYs outside conditional includes so that user can
//   #include <ntddstor.h> in precompiled header
//   #include <devpropdef.h> in a single source file
//   #include <ntddstor.h> in that source file a second time to instantiate the DEVPROPKEYs
//
#ifdef DEFINE_DEVPROPKEY

// begin_wioctldevpropkeys

//
// Properties associated with the volume interface.
//

DEFINE_DEVPROPKEY(DEVPKEY_Storage_Portable,           0x4d1ebee8, 0x803, 0x4774, 0x98, 0x42, 0xb7, 0x7d, 0xb5, 0x2, 0x65, 0xe9, 2);    // DEVPROP_TYPE_BOOLEAN
DEFINE_DEVPROPKEY(DEVPKEY_Storage_Removable_Media,    0x4d1ebee8, 0x803, 0x4774, 0x98, 0x42, 0xb7, 0x7d, 0xb5, 0x2, 0x65, 0xe9, 3);    // DEVPROP_TYPE_BOOLEAN
DEFINE_DEVPROPKEY(DEVPKEY_Storage_System_Critical,    0x4d1ebee8, 0x803, 0x4774, 0x98, 0x42, 0xb7, 0x7d, 0xb5, 0x2, 0x65, 0xe9, 4);    // DEVPROP_TYPE_BOOLEAN
DEFINE_DEVPROPKEY(DEVPKEY_Storage_Disk_Number,        0x4d1ebee8, 0x803, 0x4774, 0x98, 0x42, 0xb7, 0x7d, 0xb5, 0x2, 0x65, 0xe9, 5);    // DEVPROP_TYPE_UINT32
DEFINE_DEVPROPKEY(DEVPKEY_Storage_Partition_Number,   0x4d1ebee8, 0x803, 0x4774, 0x98, 0x42, 0xb7, 0x7d, 0xb5, 0x2, 0x65, 0xe9, 6);    // DEVPROP_TYPE_UINT32
DEFINE_DEVPROPKEY(DEVPKEY_Storage_Mbr_Type,           0x4d1ebee8, 0x803, 0x4774, 0x98, 0x42, 0xb7, 0x7d, 0xb5, 0x2, 0x65, 0xe9, 7);    // DEVPROP_TYPE_BYTE
DEFINE_DEVPROPKEY(DEVPKEY_Storage_Gpt_Type,           0x4d1ebee8, 0x803, 0x4774, 0x98, 0x42, 0xb7, 0x7d, 0xb5, 0x2, 0x65, 0xe9, 8);    // DEVPROP_TYPE_GUID
DEFINE_DEVPROPKEY(DEVPKEY_Storage_Gpt_Name,           0x4d1ebee8, 0x803, 0x4774, 0x98, 0x42, 0xb7, 0x7d, 0xb5, 0x2, 0x65, 0xe9, 9);    // DEVPROP_TYPE_STRING
// end_wioctldevpropkeys

#endif

// begin_winioctl

#ifndef _NTDDSTOR_H_
#define _NTDDSTOR_H_

#if defined __cplusplus && !defined __ALT_GENERATOR__
extern "C" {
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

//
// IoControlCode values for storage devices
//

#define IOCTL_STORAGE_BASE FILE_DEVICE_MASS_STORAGE

//
// The following device control codes are common for all class drivers.  They
// should be used in place of the older IOCTL_DISK, IOCTL_CDROM and IOCTL_TAPE
// common codes
//

#define IOCTL_STORAGE_CHECK_VERIFY            CTL_CODE(IOCTL_STORAGE_BASE, 0x0200, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_STORAGE_CHECK_VERIFY2           CTL_CODE(IOCTL_STORAGE_BASE, 0x0200, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_STORAGE_MEDIA_REMOVAL           CTL_CODE(IOCTL_STORAGE_BASE, 0x0201, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_STORAGE_EJECT_MEDIA             CTL_CODE(IOCTL_STORAGE_BASE, 0x0202, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_STORAGE_LOAD_MEDIA              CTL_CODE(IOCTL_STORAGE_BASE, 0x0203, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_STORAGE_LOAD_MEDIA2             CTL_CODE(IOCTL_STORAGE_BASE, 0x0203, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_STORAGE_RESERVE                 CTL_CODE(IOCTL_STORAGE_BASE, 0x0204, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_STORAGE_RELEASE                 CTL_CODE(IOCTL_STORAGE_BASE, 0x0205, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_STORAGE_FIND_NEW_DEVICES        CTL_CODE(IOCTL_STORAGE_BASE, 0x0206, METHOD_BUFFERED, FILE_READ_ACCESS)

// end_winioctl
#define IOCTL_STORAGE_MANAGE_BYPASS_IO        CTL_CODE(IOCTL_STORAGE_BASE, 0x0230, METHOD_BUFFERED, FILE_ANY_ACCESS)
// begin_winioctl

#define IOCTL_STORAGE_EJECTION_CONTROL        CTL_CODE(IOCTL_STORAGE_BASE, 0x0250, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_STORAGE_MCN_CONTROL             CTL_CODE(IOCTL_STORAGE_BASE, 0x0251, METHOD_BUFFERED, FILE_ANY_ACCESS)

#define IOCTL_STORAGE_GET_MEDIA_TYPES         CTL_CODE(IOCTL_STORAGE_BASE, 0x0300, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_STORAGE_GET_MEDIA_TYPES_EX      CTL_CODE(IOCTL_STORAGE_BASE, 0x0301, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_STORAGE_GET_MEDIA_SERIAL_NUMBER CTL_CODE(IOCTL_STORAGE_BASE, 0x0304, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_STORAGE_GET_HOTPLUG_INFO        CTL_CODE(IOCTL_STORAGE_BASE, 0x0305, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_STORAGE_SET_HOTPLUG_INFO        CTL_CODE(IOCTL_STORAGE_BASE, 0x0306, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_STORAGE_GET_SYSTEM_FEATURE_SUPPORT CTL_CODE(IOCTL_STORAGE_BASE, 0x0307, METHOD_BUFFERED, FILE_READ_ACCESS)

#define IOCTL_STORAGE_RESET_BUS               CTL_CODE(IOCTL_STORAGE_BASE, 0x0400, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_STORAGE_RESET_DEVICE            CTL_CODE(IOCTL_STORAGE_BASE, 0x0401, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_STORAGE_BREAK_RESERVATION       CTL_CODE(IOCTL_STORAGE_BASE, 0x0405, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_STORAGE_PERSISTENT_RESERVE_IN   CTL_CODE(IOCTL_STORAGE_BASE, 0x0406, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_STORAGE_PERSISTENT_RESERVE_OUT  CTL_CODE(IOCTL_STORAGE_BASE, 0x0407, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)


//
// This IOCTL allows a custom request to be sent directly to a StorMQ miniport. Input and output buffer formats are established by the miniport writer.
//
#define IOCTL_STORAGE_MINIPORT_PASSTHROUGH_REQUEST  CTL_CODE(IOCTL_STORAGE_BASE, 0x0414, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

#define IOCTL_STORAGE_GET_DEVICE_NUMBER       CTL_CODE(IOCTL_STORAGE_BASE, 0x0420, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// This IOCTL includes the same information as IOCTL_STORAGE_GET_DEVICE_NUMBER, plus the device GUID.
//
#define IOCTL_STORAGE_GET_DEVICE_NUMBER_EX    CTL_CODE(IOCTL_STORAGE_BASE, 0x0421, METHOD_BUFFERED, FILE_ANY_ACCESS)


#define IOCTL_STORAGE_PREDICT_FAILURE         CTL_CODE(IOCTL_STORAGE_BASE, 0x0440, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_STORAGE_FAILURE_PREDICTION_CONFIG CTL_CODE(IOCTL_STORAGE_BASE, 0x0441, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// This IOCTL retrieves reliability counters for a device.
//
#define IOCTL_STORAGE_GET_COUNTERS     CTL_CODE(IOCTL_STORAGE_BASE, 0x442, METHOD_BUFFERED, FILE_ANY_ACCESS)

#define IOCTL_STORAGE_READ_CAPACITY           CTL_CODE(IOCTL_STORAGE_BASE, 0x0450, METHOD_BUFFERED, FILE_READ_ACCESS)

//
// IOCTLs 0x0463 to 0x0468 reserved for dependent disk support.
//


//
// IOCTLs 0x0470 to 0x047f reserved for device and stack telemetry interfaces
//

#define IOCTL_STORAGE_GET_DEVICE_TELEMETRY      CTL_CODE(IOCTL_STORAGE_BASE, 0x0470, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_STORAGE_DEVICE_TELEMETRY_NOTIFY   CTL_CODE(IOCTL_STORAGE_BASE, 0x0471, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_STORAGE_DEVICE_TELEMETRY_QUERY_CAPS CTL_CODE(IOCTL_STORAGE_BASE, 0x0472, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_STORAGE_GET_DEVICE_TELEMETRY_RAW  CTL_CODE(IOCTL_STORAGE_BASE, 0x0473, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)


#define IOCTL_STORAGE_SET_TEMPERATURE_THRESHOLD     CTL_CODE(IOCTL_STORAGE_BASE, 0x0480, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

#define IOCTL_STORAGE_PROTOCOL_COMMAND              CTL_CODE(IOCTL_STORAGE_BASE, 0x04F0, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)


#define IOCTL_STORAGE_SET_PROPERTY                  CTL_CODE(IOCTL_STORAGE_BASE, 0x04FF, METHOD_BUFFERED, FILE_WRITE_ACCESS)
#define IOCTL_STORAGE_QUERY_PROPERTY                CTL_CODE(IOCTL_STORAGE_BASE, 0x0500, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_STORAGE_MANAGE_DATA_SET_ATTRIBUTES    CTL_CODE(IOCTL_STORAGE_BASE, 0x0501, METHOD_BUFFERED, FILE_WRITE_ACCESS)
#define IOCTL_STORAGE_GET_LB_PROVISIONING_MAP_RESOURCES  CTL_CODE(IOCTL_STORAGE_BASE, 0x0502, METHOD_BUFFERED, FILE_READ_ACCESS)

//
// IOCTLs 0x0503 to 0x0580 reserved for Enhanced Storage devices.
//

//
// This IOCTL offloads the erasure process to the storage device. There is no guarantee as to the successful
// deletion or recoverability of the data on the storage device after command completion. This IOCTL is limited
// to data disks in regular Windows. In WinPE, this IOCTL is supported for both boot and data disks.
//
// This IOCTL has an optional input and returns no output other than status. Callers should first call
// FSCTL_LOCK_VOLUME before calling this ioctl to flush out cached data in upper layers. No waiting of outstanding
// request completion is done before issuing the command to the device.
//
#define IOCTL_STORAGE_REINITIALIZE_MEDIA    CTL_CODE(IOCTL_STORAGE_BASE, 0x0590, METHOD_BUFFERED, FILE_WRITE_ACCESS)


//
// IOCTLs for bandwidth contracts on storage devices
// (Move this to ntddsfio if we decide to use a new base)
//

#define IOCTL_STORAGE_GET_BC_PROPERTIES         CTL_CODE(IOCTL_STORAGE_BASE, 0x0600, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_STORAGE_ALLOCATE_BC_STREAM        CTL_CODE(IOCTL_STORAGE_BASE, 0x0601, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_STORAGE_FREE_BC_STREAM            CTL_CODE(IOCTL_STORAGE_BASE, 0x0602, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// IOCTL to check for priority support
//
#define IOCTL_STORAGE_CHECK_PRIORITY_HINT_SUPPORT    CTL_CODE(IOCTL_STORAGE_BASE, 0x0620, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// IOCTL for data integrity check support
//

#define IOCTL_STORAGE_START_DATA_INTEGRITY_CHECK     CTL_CODE(IOCTL_STORAGE_BASE, 0x0621, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_STORAGE_STOP_DATA_INTEGRITY_CHECK      CTL_CODE(IOCTL_STORAGE_BASE, 0x0622, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// These ioctl codes are obsolete.  They are defined here to avoid resuing them
// and to allow class drivers to respond to them more easily.
//

#define OBSOLETE_IOCTL_STORAGE_RESET_BUS        CTL_CODE(IOCTL_STORAGE_BASE, 0x0400, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define OBSOLETE_IOCTL_STORAGE_RESET_DEVICE     CTL_CODE(IOCTL_STORAGE_BASE, 0x0401, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// IOCTLs 0x0643 to 0x0655 reserved for VHD disk support.
//


//
// IOCTLs for firmware upgrade on storage devices
//

#define IOCTL_STORAGE_FIRMWARE_GET_INFO         CTL_CODE(IOCTL_STORAGE_BASE, 0x0700, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_STORAGE_FIRMWARE_DOWNLOAD         CTL_CODE(IOCTL_STORAGE_BASE, 0x0701, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_STORAGE_FIRMWARE_ACTIVATE         CTL_CODE(IOCTL_STORAGE_BASE, 0x0702, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)


//
// IOCTL to support Idle Power Management, including Device Wake
//
#define IOCTL_STORAGE_ENABLE_IDLE_POWER         CTL_CODE(IOCTL_STORAGE_BASE, 0x0720, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_STORAGE_GET_IDLE_POWERUP_REASON   CTL_CODE(IOCTL_STORAGE_BASE, 0x0721, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// IOCTLs to allow class drivers to acquire and release active references on
// a unit.  These should only be used if the class driver previously sent a
// successful IOCTL_STORAGE_ENABLE_IDLE_POWER request to the port driver.
//
#define IOCTL_STORAGE_POWER_ACTIVE  CTL_CODE(IOCTL_STORAGE_BASE, 0x0722, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_STORAGE_POWER_IDLE    CTL_CODE(IOCTL_STORAGE_BASE, 0x0723, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// This IOCTL indicates that the physical device has triggered some sort of event.
//
#define IOCTL_STORAGE_EVENT_NOTIFICATION CTL_CODE(IOCTL_STORAGE_BASE, 0x0724, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// IOCTL to specify a power cap for a storage device.
//
#define IOCTL_STORAGE_DEVICE_POWER_CAP CTL_CODE(IOCTL_STORAGE_BASE, 0x0725, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// IOCTL to send commands to the RPMB for a storage device.
//
#define IOCTL_STORAGE_RPMB_COMMAND     CTL_CODE(IOCTL_STORAGE_BASE, 0x0726, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// IOCTL to manage attributes for storage devices
//
#define IOCTL_STORAGE_ATTRIBUTE_MANAGEMENT CTL_CODE(IOCTL_STORAGE_BASE, 0x0727, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// IOCTL_STORAGE_DIAGNOSTIC IOCTL to query diagnostic data from the storage driver stack
//
#define IOCTL_STORAGE_DIAGNOSTIC    CTL_CODE(IOCTL_STORAGE_BASE, 0x0728, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// IOCTLs for storage device depopulation support.
//

//
// IOCTL_STORAGE_GET_PHYSICAL_ELEMENT_STATUS IOCTL to query physical element status from device.
//
#define IOCTL_STORAGE_GET_PHYSICAL_ELEMENT_STATUS    CTL_CODE(IOCTL_STORAGE_BASE, 0x0729, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// IOCTL_STORAGE_REMOVE_ELEMENT_AND_TRUNCATE IOCTL to remove and truncate element from device.
//
#define IOCTL_STORAGE_REMOVE_ELEMENT_AND_TRUNCATE    CTL_CODE(IOCTL_STORAGE_BASE, 0x0730, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// IOCTL_STORAGE_GET_DEVICE_INTERNAL_LOG IOCTL to get device internal status data.
//
#define IOCTL_STORAGE_GET_DEVICE_INTERNAL_LOG    CTL_CODE(IOCTL_STORAGE_BASE, 0x0731, METHOD_BUFFERED, FILE_ANY_ACCESS)


//
// Note: Function code values of less than 0x800 are reserved for Microsoft. Values of 0x800 and higher can be used by vendors.
//       So do not use function code of 0x800 and higher to define new IOCTLs in this file.
//


//
// IOCTL_STORAGE_GET_HOTPLUG_INFO
//

typedef struct _STORAGE_HOTPLUG_INFO {
    ULONG Size; // version
    BOOLEAN MediaRemovable; // ie. zip, jaz, cdrom, mo, etc. vs hdd
    BOOLEAN MediaHotplug;   // ie. does the device succeed a lock even though its not lockable media?
    BOOLEAN DeviceHotplug;  // ie. 1394, USB, etc.
    BOOLEAN WriteCacheEnableOverride; // This field should not be relied upon because it is no longer used
} STORAGE_HOTPLUG_INFO, *PSTORAGE_HOTPLUG_INFO;

//
// IOCTL_STORAGE_GET_SYSTEM_FEATURE_SUPPORT
//
// This IOCTL can be sent to any disk or adapter device but the query itself returns system-wide
// feature support as offered by the currently-installed version of the storage stack.
//

#define STORAGE_FEATURE_SUPPORT_V1              0x1

#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/unions
#pragma warning(disable:4214) // bit fields other than int to disable this around the struct

typedef struct _STORAGE_FEATURE_SUPPORT {
    //
    // Size of this structure
    //
    ULONG Size;

    //
    // Version of this structure
    //
    ULONG Version;

    union {

        struct {

            //
            // If set to '1', indicates that support for StorMQ miniports is present
            //
            ULONGLONG StorMQMiniportsSupported : 1;

            //
            // Reserved for future use. Must be set to zero.
            //
            ULONGLONG Reserved : 63;

        } DUMMYSTRUCTNAME;

        ULONGLONG AsUlonglong;

    } Flags;

    ULONGLONG Reserved[6];

} STORAGE_FEATURE_SUPPORT, *PSTORAGE_FEATURE_SUPPORT;

#pragma warning(pop)

//
// IOCTL_STORAGE_GET_DEVICE_NUMBER
//
// input - none
//
// output - STORAGE_DEVICE_NUMBER structure
//          The values in the STORAGE_DEVICE_NUMBER structure are guaranteed
//          to remain unchanged until the system is rebooted.  They are not
//          guaranteed to be persistant across boots.
//

typedef struct _STORAGE_DEVICE_NUMBER {

    //
    // The FILE_DEVICE_XXX type for this device.
    //

    DEVICE_TYPE DeviceType;

    //
    // The number of this device
    //

    ULONG       DeviceNumber;

    //
    // If the device is partitionable, the partition number of the device.
    // Otherwise -1
    //

    ULONG       PartitionNumber;
} STORAGE_DEVICE_NUMBER, *PSTORAGE_DEVICE_NUMBER;

typedef struct _STORAGE_DEVICE_NUMBERS {

    //
    // Size of this structure serves
    // as the version
    //

    ULONG Version;

    //
    // Size of this structure
    //

    ULONG Size;

    ULONG NumberOfDevices;

    STORAGE_DEVICE_NUMBER Devices[ANYSIZE_ARRAY];

} STORAGE_DEVICE_NUMBERS, *PSTORAGE_DEVICE_NUMBERS;

//
// IOCTL_STORAGE_GET_DEVICE_NUMBER_EX
//
// input - none
//
// output - STORAGE_DEVICE_NUMBER_EX structure
//

//
// Possible flags that can be set in Flags field of
// STORAGE_DEVICE_NUMBER_EX structure defined below
//

//
// This flag indicates that deviceguid is randomly created because a deviceguid conflict was observed
//
#define STORAGE_DEVICE_FLAGS_RANDOM_DEVICEGUID_REASON_CONFLICT 0x1

//
// This flag indicates that deviceguid is randomly created because the HW ID was not available
//
#define STORAGE_DEVICE_FLAGS_RANDOM_DEVICEGUID_REASON_NOHWID   0x2

//
// This flag indicates that deviceguid is created from the scsi page83 data.
// If this flag is not set this implies it's created from serial number or is randomly generated.
//
#define STORAGE_DEVICE_FLAGS_PAGE_83_DEVICEGUID                0x4

typedef struct _STORAGE_DEVICE_NUMBER_EX {

    //
    // Sizeof(STORAGE_DEVICE_NUMBER_EX).
    //

    ULONG       Version;

    //
    // Total size of the structure, including any additional data. Currently
    // this will always be the same as sizeof(STORAGE_DEVICE_NUMBER_EX).
    //

    ULONG       Size;

    //
    // Flags - this shall be a combination of STORAGE_DEVICE_FLAGS_XXX flags
    // that gives more information about the members of this structure.
    //

    ULONG       Flags;

    //
    // The FILE_DEVICE_XXX type for this device. This IOCTL is only
    // supported for disk devices.
    //

    DEVICE_TYPE DeviceType;

    //
    // The number of this device.
    //

    ULONG       DeviceNumber;

    //
    // A globally-unique identification number for this device.
    // A GUID of {0} indicates that a GUID could not be generated. The GUID
    // is based on hardware information that doesn't change with firmware updates
    // (for instance, serial number can be used to form the GUID, but not the firmware
    // revision). The device GUID remains the same across reboots.
    //
    // In general, if a device exposes a globally unique identifier, the storage driver
    // will use that identifier to form the GUID. Otherwise, the storage driver will combine
    // the device's vendor ID, product ID and serial number to create the GUID.
    //
    // If a storage driver detects two devices with the same hardware information (which is
    // an indication of a problem with the device), the driver will generate a random GUID for
    // one of the two devices. When handling IOCTL_STORAGE_GET_DEVICE_NUMBER_EX for the device
    // with the random GUID, the driver will add STORAGE_DEVICE_FLAGS_RANDOM_DEVICEGUID_REASON_CONFLICT
    // to the Flags member of this structure.
    //
    // If a storage device does not provide any identifying information, the driver will generate a random
    // GUID and add STORAGE_DEVICE_FLAGS_RANDOM_DEVICEGUID_REASON_NOHWID to the Flags member of this structure.
    //
    // A random GUID is not persisted and will not be the same after a reboot.
    //

    GUID        DeviceGuid;

    //
    // If the device is partitionable, the partition number of the device.
    // Otherwise -1.
    //

    ULONG       PartitionNumber;
} STORAGE_DEVICE_NUMBER_EX, *PSTORAGE_DEVICE_NUMBER_EX;


//
// Define the structures for scsi resets
//

typedef struct _STORAGE_BUS_RESET_REQUEST {
    UCHAR PathId;
} STORAGE_BUS_RESET_REQUEST, *PSTORAGE_BUS_RESET_REQUEST;


//
// Break reservation is sent to the Adapter/FDO with the given lun information.
//

typedef struct STORAGE_BREAK_RESERVATION_REQUEST {
    ULONG Length;
    UCHAR _unused;
    UCHAR PathId;
    UCHAR TargetId;
    UCHAR Lun;
} STORAGE_BREAK_RESERVATION_REQUEST, *PSTORAGE_BREAK_RESERVATION_REQUEST;


//
// IOCTL_STORAGE_MEDIA_REMOVAL disables the mechanism
// on a storage device that ejects media. This function
// may or may not be supported on storage devices that
// support removable media.
//
// TRUE means prevent media from being removed.
// FALSE means allow media removal.
//

typedef struct _PREVENT_MEDIA_REMOVAL {
    BOOLEAN PreventMediaRemoval;
} PREVENT_MEDIA_REMOVAL, *PPREVENT_MEDIA_REMOVAL;



//
//  This is the format of TARGET_DEVICE_CUSTOM_NOTIFICATION.CustomDataBuffer
//  passed to applications by the classpnp autorun code (via IoReportTargetDeviceChangeAsynchronous).
//
typedef struct _CLASS_MEDIA_CHANGE_CONTEXT {
    ULONG MediaChangeCount;
    ULONG NewState;         // see MEDIA_CHANGE_DETECTION_STATE enum in classpnp.h in DDK
} CLASS_MEDIA_CHANGE_CONTEXT, *PCLASS_MEDIA_CHANGE_CONTEXT;


// begin_ntminitape

typedef struct _TAPE_STATISTICS {
    ULONG Version;
    ULONG Flags;
    LARGE_INTEGER RecoveredWrites;
    LARGE_INTEGER UnrecoveredWrites;
    LARGE_INTEGER RecoveredReads;
    LARGE_INTEGER UnrecoveredReads;
    UCHAR         CompressionRatioReads;
    UCHAR         CompressionRatioWrites;
} TAPE_STATISTICS, *PTAPE_STATISTICS;

#define RECOVERED_WRITES_VALID   0x00000001
#define UNRECOVERED_WRITES_VALID 0x00000002
#define RECOVERED_READS_VALID    0x00000004
#define UNRECOVERED_READS_VALID  0x00000008
#define WRITE_COMPRESSION_INFO_VALID  0x00000010
#define READ_COMPRESSION_INFO_VALID   0x00000020

typedef struct _TAPE_GET_STATISTICS {
    ULONG Operation;
} TAPE_GET_STATISTICS, *PTAPE_GET_STATISTICS;

#define TAPE_RETURN_STATISTICS 0L
#define TAPE_RETURN_ENV_INFO   1L
#define TAPE_RESET_STATISTICS  2L

//
// IOCTL_STORAGE_GET_MEDIA_TYPES_EX will return an array of DEVICE_MEDIA_INFO
// structures, one per supported type, embedded in the GET_MEDIA_TYPES struct.
//

typedef enum _STORAGE_MEDIA_TYPE {
    //
    // Following are defined in ntdddisk.h in the MEDIA_TYPE enum
    //
    // Unknown,                // Format is unknown
    // F5_1Pt2_512,            // 5.25", 1.2MB,  512 bytes/sector
    // F3_1Pt44_512,           // 3.5",  1.44MB, 512 bytes/sector
    // F3_2Pt88_512,           // 3.5",  2.88MB, 512 bytes/sector
    // F3_20Pt8_512,           // 3.5",  20.8MB, 512 bytes/sector
    // F3_720_512,             // 3.5",  720KB,  512 bytes/sector
    // F5_360_512,             // 5.25", 360KB,  512 bytes/sector
    // F5_320_512,             // 5.25", 320KB,  512 bytes/sector
    // F5_320_1024,            // 5.25", 320KB,  1024 bytes/sector
    // F5_180_512,             // 5.25", 180KB,  512 bytes/sector
    // F5_160_512,             // 5.25", 160KB,  512 bytes/sector
    // RemovableMedia,         // Removable media other than floppy
    // FixedMedia,             // Fixed hard disk media
    // F3_120M_512,            // 3.5", 120M Floppy
    // F3_640_512,             // 3.5" ,  640KB,  512 bytes/sector
    // F5_640_512,             // 5.25",  640KB,  512 bytes/sector
    // F5_720_512,             // 5.25",  720KB,  512 bytes/sector
    // F3_1Pt2_512,            // 3.5" ,  1.2Mb,  512 bytes/sector
    // F3_1Pt23_1024,          // 3.5" ,  1.23Mb, 1024 bytes/sector
    // F5_1Pt23_1024,          // 5.25",  1.23MB, 1024 bytes/sector
    // F3_128Mb_512,           // 3.5" MO 128Mb   512 bytes/sector
    // F3_230Mb_512,           // 3.5" MO 230Mb   512 bytes/sector
    // F8_256_128,             // 8",     256KB,  128 bytes/sector
    // F3_200Mb_512,           // 3.5",   200M Floppy (HiFD)
    //

    DDS_4mm = 0x20,            // Tape - DAT DDS1,2,... (all vendors)
    MiniQic,                   // Tape - miniQIC Tape
    Travan,                    // Tape - Travan TR-1,2,3,...
    QIC,                       // Tape - QIC
    MP_8mm,                    // Tape - 8mm Exabyte Metal Particle
    AME_8mm,                   // Tape - 8mm Exabyte Advanced Metal Evap
    AIT1_8mm,                  // Tape - 8mm Sony AIT
    DLT,                       // Tape - DLT Compact IIIxt, IV
    NCTP,                      // Tape - Philips NCTP
    IBM_3480,                  // Tape - IBM 3480
    IBM_3490E,                 // Tape - IBM 3490E
    IBM_Magstar_3590,          // Tape - IBM Magstar 3590
    IBM_Magstar_MP,            // Tape - IBM Magstar MP
    STK_DATA_D3,               // Tape - STK Data D3
    SONY_DTF,                  // Tape - Sony DTF
    DV_6mm,                    // Tape - 6mm Digital Video
    DMI,                       // Tape - Exabyte DMI and compatibles
    SONY_D2,                   // Tape - Sony D2S and D2L
    CLEANER_CARTRIDGE,         // Cleaner - All Drive types that support Drive Cleaners
    CD_ROM,                    // Opt_Disk - CD
    CD_R,                      // Opt_Disk - CD-Recordable (Write Once)
    CD_RW,                     // Opt_Disk - CD-Rewriteable
    DVD_ROM,                   // Opt_Disk - DVD-ROM
    DVD_R,                     // Opt_Disk - DVD-Recordable (Write Once)
    DVD_RW,                    // Opt_Disk - DVD-Rewriteable
    MO_3_RW,                   // Opt_Disk - 3.5" Rewriteable MO Disk
    MO_5_WO,                   // Opt_Disk - MO 5.25" Write Once
    MO_5_RW,                   // Opt_Disk - MO 5.25" Rewriteable (not LIMDOW)
    MO_5_LIMDOW,               // Opt_Disk - MO 5.25" Rewriteable (LIMDOW)
    PC_5_WO,                   // Opt_Disk - Phase Change 5.25" Write Once Optical
    PC_5_RW,                   // Opt_Disk - Phase Change 5.25" Rewriteable
    PD_5_RW,                   // Opt_Disk - PhaseChange Dual Rewriteable
    ABL_5_WO,                  // Opt_Disk - Ablative 5.25" Write Once Optical
    PINNACLE_APEX_5_RW,        // Opt_Disk - Pinnacle Apex 4.6GB Rewriteable Optical
    SONY_12_WO,                // Opt_Disk - Sony 12" Write Once
    PHILIPS_12_WO,             // Opt_Disk - Philips/LMS 12" Write Once
    HITACHI_12_WO,             // Opt_Disk - Hitachi 12" Write Once
    CYGNET_12_WO,              // Opt_Disk - Cygnet/ATG 12" Write Once
    KODAK_14_WO,               // Opt_Disk - Kodak 14" Write Once
    MO_NFR_525,                // Opt_Disk - Near Field Recording (Terastor)
    NIKON_12_RW,               // Opt_Disk - Nikon 12" Rewriteable
    IOMEGA_ZIP,                // Mag_Disk - Iomega Zip
    IOMEGA_JAZ,                // Mag_Disk - Iomega Jaz
    SYQUEST_EZ135,             // Mag_Disk - Syquest EZ135
    SYQUEST_EZFLYER,           // Mag_Disk - Syquest EzFlyer
    SYQUEST_SYJET,             // Mag_Disk - Syquest SyJet
    AVATAR_F2,                 // Mag_Disk - 2.5" Floppy
    MP2_8mm,                   // Tape - 8mm Hitachi
    DST_S,                     // Ampex DST Small Tapes
    DST_M,                     // Ampex DST Medium Tapes
    DST_L,                     // Ampex DST Large Tapes
    VXATape_1,                 // Ecrix 8mm Tape
    VXATape_2,                 // Ecrix 8mm Tape
#if (NTDDI_VERSION < NTDDI_WINXP)
    STK_EAGLE,                 // STK Eagle
#else
    STK_9840,                  // STK 9840
#endif
    LTO_Ultrium,               // IBM, HP, Seagate LTO Ultrium
    LTO_Accelis,               // IBM, HP, Seagate LTO Accelis
    DVD_RAM,                   // Opt_Disk - DVD-RAM
    AIT_8mm,                   // AIT2 or higher
    ADR_1,                     // OnStream ADR Mediatypes
    ADR_2,
    STK_9940,                  // STK 9940
    SAIT,                      // SAIT Tapes
    VXATape                    // VXA (Ecrix 8mm) Tape
}STORAGE_MEDIA_TYPE, *PSTORAGE_MEDIA_TYPE;

#define MEDIA_ERASEABLE         0x00000001
#define MEDIA_WRITE_ONCE        0x00000002
#define MEDIA_READ_ONLY         0x00000004
#define MEDIA_READ_WRITE        0x00000008

#define MEDIA_WRITE_PROTECTED   0x00000100
#define MEDIA_CURRENTLY_MOUNTED 0x80000000

//
// Define the different storage bus types
// Bus types below 128 (0x80) are reserved for Microsoft use
//

typedef enum _STORAGE_BUS_TYPE {
    BusTypeUnknown = 0x00,
    BusTypeScsi,
    BusTypeAtapi,
    BusTypeAta,
    BusType1394,
    BusTypeSsa,
    BusTypeFibre,
    BusTypeUsb,
    BusTypeRAID,
    BusTypeiScsi,
    BusTypeSas,
    BusTypeSata,
    BusTypeSd,
    BusTypeMmc,
    BusTypeVirtual,
    BusTypeFileBackedVirtual,
    BusTypeSpaces,
    BusTypeNvme,
    BusTypeSCM,
    BusTypeUfs,
    BusTypeNvmeof,
    BusTypeMax,
    BusTypeMaxReserved = 0x7F
} STORAGE_BUS_TYPE, *PSTORAGE_BUS_TYPE;

//
// Macro to identify which bus types
// support shared storage
//

#define SupportsDeviceSharing( BusType ) ( \
        (BusType == BusTypeScsi)  ||       \
        (BusType == BusTypeFibre) ||       \
        (BusType == BusTypeiScsi) ||       \
        (BusType == BusTypeSas)   ||       \
        (BusType == BusTypeSpaces) )

typedef struct _DEVICE_MEDIA_INFO {
    union {
        struct {
            LARGE_INTEGER Cylinders;
            STORAGE_MEDIA_TYPE MediaType;
            ULONG TracksPerCylinder;
            ULONG SectorsPerTrack;
            ULONG BytesPerSector;
            ULONG NumberMediaSides;
            ULONG MediaCharacteristics; // Bitmask of MEDIA_XXX values.
        } DiskInfo;

        struct {
            LARGE_INTEGER Cylinders;
            STORAGE_MEDIA_TYPE MediaType;
            ULONG TracksPerCylinder;
            ULONG SectorsPerTrack;
            ULONG BytesPerSector;
            ULONG NumberMediaSides;
            ULONG MediaCharacteristics; // Bitmask of MEDIA_XXX values.
        } RemovableDiskInfo;

        struct {
            STORAGE_MEDIA_TYPE MediaType;
            ULONG   MediaCharacteristics; // Bitmask of MEDIA_XXX values.
            ULONG   CurrentBlockSize;
            STORAGE_BUS_TYPE BusType;

            //
            // Bus specific information describing the medium supported.
            //

            union {
                struct {
                    UCHAR MediumType;
                    UCHAR DensityCode;
                } ScsiInformation;
            } BusSpecificData;

        } TapeInfo;
    } DeviceSpecific;
} DEVICE_MEDIA_INFO, *PDEVICE_MEDIA_INFO;

typedef struct _GET_MEDIA_TYPES {
    ULONG DeviceType;              // FILE_DEVICE_XXX values
    ULONG MediaInfoCount;
    DEVICE_MEDIA_INFO MediaInfo[1];
} GET_MEDIA_TYPES, *PGET_MEDIA_TYPES;


//
// IOCTL_STORAGE_PREDICT_FAILURE
//
// input - none
//
// output - STORAGE_PREDICT_FAILURE structure
//          PredictFailure returns zero if no failure predicted and non zero
//                         if a failure is predicted.
//
//          VendorSpecific returns 512 bytes of vendor specific information
//                         if a failure is predicted
//
typedef struct _STORAGE_PREDICT_FAILURE
{
    ULONG PredictFailure;
    UCHAR VendorSpecific[512];
} STORAGE_PREDICT_FAILURE, *PSTORAGE_PREDICT_FAILURE;


//
// IOCTL_STORAGE_FAILURE_PREDICTION_CONFIG
//
// Input - STORAGE_FAILURE_PREDICTION_CONFIG structure.
//         If the sender wants to enable or disable failure prediction then
//         the sender should set the "Set" field to TRUE.
// Output - STORAGE_FAILURE_PREDICTION_CONFIG structure.
//          If successful, the "Enabled" field will indicate if failure
//          prediction is currently enabled or not.
//
typedef struct _STORAGE_FAILURE_PREDICTION_CONFIG {
    ULONG Version;      // Set to 1 for Blue.
    ULONG Size;
    BOOLEAN Set;        // TRUE if the sender wants to enable/disable failure prediction.
    BOOLEAN Enabled;
    USHORT Reserved;
} STORAGE_FAILURE_PREDICTION_CONFIG, *PSTORAGE_FAILURE_PREDICTION_CONFIG;

#define STORAGE_FAILURE_PREDICTION_CONFIG_V1 1

// end_ntminitape



//
// Property Query Structures
//

//
// IOCTL_STORAGE_QUERY_PROPERTY
//
// Input Buffer:
//      a STORAGE_PROPERTY_QUERY structure which describes what type of query
//      is being done, what property is being queried for, and any additional
//      parameters which a particular property query requires.
//
//  Output Buffer:
//      Contains a buffer to place the results of the query into.  Since all
//      property descriptors can be cast into a STORAGE_DESCRIPTOR_HEADER,
//      the IOCTL can be called once with a small buffer then again using
//      a buffer as large as the header reports is necessary.
//


//
// Types of queries
//

typedef enum _STORAGE_QUERY_TYPE {
    PropertyStandardQuery = 0,          // Retrieves the descriptor
    PropertyExistsQuery,                // Used to test whether the descriptor is supported
    PropertyMaskQuery,                  // Used to retrieve a mask of writeable fields in the descriptor
    PropertyQueryMaxDefined     // use to validate the value
} STORAGE_QUERY_TYPE, *PSTORAGE_QUERY_TYPE;

//
// IOCTL_STORAGE_SET_PROPERTY
//
// Input Buffer:
//      a STORAGE_PROPERTY_SET structure which describes what type of property set
//      is being done, what property is being set, and any additional
//      parameters which a particular property set requires.
//
//  Output Buffer:
//      Contains a buffer to place the results of the query into.  Since all
//      property descriptors can be cast into a STORAGE_DESCRIPTOR_HEADER,
//      the IOCTL can be called once with a small buffer then again using
//      a buffer as large as the header reports is necessary.
//


//
// Types of set operation
//

typedef enum _STORAGE_SET_TYPE {
    PropertyStandardSet = 0,          // Sets the descriptor
    PropertyExistsSet,                // Used to test whether the descriptor is supported
    PropertySetMaxDefined             // use to validate the value
} STORAGE_SET_TYPE, *PSTORAGE_SET_TYPE;

//
// define some initial property id's
//

typedef enum _STORAGE_PROPERTY_ID {
    StorageDeviceProperty = 0,
    StorageAdapterProperty,
    StorageDeviceIdProperty,
    StorageDeviceUniqueIdProperty,                  // See storduid.h for details
    StorageDeviceWriteCacheProperty,
    StorageMiniportProperty,
    StorageAccessAlignmentProperty,
    StorageDeviceSeekPenaltyProperty,
    StorageDeviceTrimProperty,
    StorageDeviceWriteAggregationProperty,
    StorageDeviceDeviceTelemetryProperty,
    StorageDeviceLBProvisioningProperty,
    StorageDevicePowerProperty,
    StorageDeviceCopyOffloadProperty,
    StorageDeviceResiliencyProperty,
    StorageDeviceMediumProductType,
    StorageAdapterRpmbProperty,
    StorageAdapterCryptoProperty,                   // Deprecated for GE or greater OS. Use StorageHwCryptoProperty.
// end_winioctl
    StorageDeviceTieringProperty,
    StorageDeviceFaultDomainProperty,
    StorageDeviceClusportProperty,
    StorageDeviceDependantDevicesProperty,
// begin_winioctl
    StorageDeviceIoCapabilityProperty = 48,
    StorageAdapterProtocolSpecificProperty,
    StorageDeviceProtocolSpecificProperty,
    StorageAdapterTemperatureProperty,
    StorageDeviceTemperatureProperty,
    StorageAdapterPhysicalTopologyProperty,
    StorageDevicePhysicalTopologyProperty,
    StorageDeviceAttributesProperty,
    StorageDeviceManagementStatus,
    StorageAdapterSerialNumberProperty,
    StorageDeviceLocationProperty,
    StorageDeviceNumaProperty,
    StorageDeviceZonedDeviceProperty,
    StorageDeviceUnsafeShutdownCount,
    StorageDeviceEnduranceProperty,
    StorageDeviceLedStateProperty,
    StorageDeviceSelfEncryptionProperty = 64,
    StorageFruIdProperty,
    StorageStackProperty,
    StorageAdapterProtocolSpecificPropertyEx,
    StorageDeviceProtocolSpecificPropertyEx,
    StorageHwCryptoProperty
} STORAGE_PROPERTY_ID, *PSTORAGE_PROPERTY_ID;


//
// Query structure - additional parameters for specific queries can follow
// the header
//

typedef struct _STORAGE_PROPERTY_QUERY {

    //
    // ID of the property being retrieved
    //

    STORAGE_PROPERTY_ID PropertyId;

    //
    // Flags indicating the type of query being performed
    //

    STORAGE_QUERY_TYPE QueryType;

    //
    // Space for additional parameters if necessary
    //

    UCHAR AdditionalParameters[1];

} STORAGE_PROPERTY_QUERY, *PSTORAGE_PROPERTY_QUERY;

//
// Set structure - additional parameters for specific set property that can follow
// the header
//

typedef struct _STORAGE_PROPERTY_SET {

    //
    // ID of the property being retrieved
    //

    STORAGE_PROPERTY_ID PropertyId;

    //
    // Flags indicating the type of set property being performed
    //

    STORAGE_SET_TYPE SetType;

    //
    // Space for additional parameters if necessary
    //

    UCHAR AdditionalParameters[1];

} STORAGE_PROPERTY_SET, *PSTORAGE_PROPERTY_SET;

//
// Standard property descriptor header.  All property pages should use this
// as their first element or should contain these two elements
//

typedef _Struct_size_bytes_(Size) struct _STORAGE_DESCRIPTOR_HEADER {

    ULONG Version;

    ULONG Size;

} STORAGE_DESCRIPTOR_HEADER, *PSTORAGE_DESCRIPTOR_HEADER;

//
// Device property descriptor - this is really just a rehash of the inquiry
// data retrieved from a scsi device
//
// This may only be retrieved from a target device.  Sending this to the bus
// will result in an error
//

typedef _Struct_size_bytes_(Size) struct _STORAGE_DEVICE_DESCRIPTOR {

    //
    // Sizeof(STORAGE_DEVICE_DESCRIPTOR)
    //

    ULONG Version;

    //
    // Total size of the descriptor, including the space for additional
    // data and id strings
    //

    ULONG Size;

    //
    // The SCSI-2 device type
    //

    UCHAR DeviceType;

    //
    // The SCSI-2 device type modifier (if any) - this may be zero
    //

    UCHAR DeviceTypeModifier;

    //
    // Flag indicating whether the device's media (if any) is removable.  This
    // field should be ignored for media-less devices
    //

    BOOLEAN RemovableMedia;

    //
    // Flag indicating whether the device can support mulitple outstanding
    // commands.  The actual synchronization in this case is the responsibility
    // of the port driver.
    //

    BOOLEAN CommandQueueing;

    //
    // Byte offset to the zero-terminated ascii string containing the device's
    // vendor id string.  For devices with no such ID this will be zero
    //

    ULONG VendorIdOffset;

    //
    // Byte offset to the zero-terminated ascii string containing the device's
    // product id string.  For devices with no such ID this will be zero
    //

    ULONG ProductIdOffset;

    //
    // Byte offset to the zero-terminated ascii string containing the device's
    // product revision string.  For devices with no such string this will be
    // zero
    //

    ULONG ProductRevisionOffset;

    //
    // Byte offset to the zero-terminated ascii string containing the device's
    // serial number.  For devices with no serial number this will be zero
    //

    ULONG SerialNumberOffset;

    //
    // Contains the bus type (as defined above) of the device.  It should be
    // used to interpret the raw device properties at the end of this structure
    // (if any)
    //

    STORAGE_BUS_TYPE BusType;

    //
    // The number of bytes of bus-specific data which have been appended to
    // this descriptor
    //

    ULONG RawPropertiesLength;

    //
    // Place holder for the first byte of the bus specific property data
    //

    UCHAR RawDeviceProperties[1];

} STORAGE_DEVICE_DESCRIPTOR, *PSTORAGE_DEVICE_DESCRIPTOR;


//
// Adapter properties
//
// This descriptor can be retrieved from a target device object of from the
// device object for the bus.  Retrieving from the target device object will
// forward the request to the underlying bus
//

typedef _Struct_size_bytes_(Size) struct _STORAGE_ADAPTER_DESCRIPTOR {

    ULONG Version;

    ULONG Size;

    ULONG MaximumTransferLength;

    ULONG MaximumPhysicalPages;

    ULONG AlignmentMask;

    BOOLEAN AdapterUsesPio;

    BOOLEAN AdapterScansDown;

    BOOLEAN CommandQueueing;

    BOOLEAN AcceleratedTransfer;

#if (NTDDI_VERSION < NTDDI_WINXP)
    BOOLEAN BusType;
#else
    UCHAR BusType;
#endif

    USHORT BusMajorVersion;

    USHORT BusMinorVersion;

#if (NTDDI_VERSION >= NTDDI_WIN8)

    UCHAR SrbType;

    UCHAR AddressType;
#endif

} STORAGE_ADAPTER_DESCRIPTOR, *PSTORAGE_ADAPTER_DESCRIPTOR;


#if (NTDDI_VERSION >= NTDDI_WIN8)

#define NO_SRBTYPE_ADAPTER_DESCRIPTOR_SIZE  \
    UFIELD_OFFSET(STORAGE_ADAPTER_DESCRIPTOR, SrbType)

#if !defined(SRB_TYPE_SCSI_REQUEST_BLOCK)
#define SRB_TYPE_SCSI_REQUEST_BLOCK         0
#endif

#if !defined(SRB_TYPE_STORAGE_REQUEST_BLOCK)
#define SRB_TYPE_STORAGE_REQUEST_BLOCK      1
#endif

#if !defined(STORAGE_ADDRESS_TYPE_BTL8)
#define STORAGE_ADDRESS_TYPE_BTL8                   0
#endif

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

typedef _Struct_size_bytes_(Size) struct _STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR {

    //
    // Sizeof(STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR)
    //

    ULONG Version;

    //
    // Total size of the descriptor, including the space for additional
    // data and id strings
    //

    ULONG Size;

    //
    // The number of bytes in a cache line of the device
    //

    ULONG BytesPerCacheLine;

    //
    // The address offset neccessary for proper cache access alignment in bytes
    //

    ULONG BytesOffsetForCacheAlignment;

    //
    // The number of bytes in a physical sector of the device
    //

    ULONG BytesPerLogicalSector;

    //
    // The number of bytes in an addressable logical sector (LBA)of the device
    //

    ULONG BytesPerPhysicalSector;

    //
    // The address offset neccessary for proper sector access alignment in bytes
    //

    ULONG BytesOffsetForSectorAlignment;

} STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR, *PSTORAGE_ACCESS_ALIGNMENT_DESCRIPTOR;

typedef _Struct_size_bytes_(Size) struct _STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {

    //
    // Sizeof(STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR)
    //

    ULONG Version;

    //
    // Total size of the descriptor, including the space for additional data
    //

    ULONG Size;

    //
    // Product type of the supporting storage medium
    //

    ULONG MediumProductType;

} STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR, *PSTORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR;


typedef enum _STORAGE_PORT_CODE_SET {
    StoragePortCodeSetReserved  = 0,
    StoragePortCodeSetStorport  = 1,
    StoragePortCodeSetSCSIport  = 2,
    StoragePortCodeSetSpaceport = 3,
    StoragePortCodeSetATAport   = 4,
    StoragePortCodeSetUSBport   = 5,
    StoragePortCodeSetSBP2port  = 6,
    StoragePortCodeSetSDport    = 7
} STORAGE_PORT_CODE_SET, *PSTORAGE_PORT_CODE_SET;

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define STORAGE_MINIPORT_DESCRIPTOR_V1_SIZE     RTL_SIZEOF_THROUGH_FIELD(STORAGE_MINIPORT_DESCRIPTOR, IoTimeoutValue)
#endif

#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/unions
#pragma warning(disable:4214) // bit fields other than int to disable this around the struct

typedef struct _STORAGE_MINIPORT_DESCRIPTOR {

    ULONG Version;

    ULONG Size;

    STORAGE_PORT_CODE_SET Portdriver;

    BOOLEAN LUNResetSupported;

    BOOLEAN TargetResetSupported;

#if (NTDDI_VERSION >= NTDDI_WIN8)
    USHORT  IoTimeoutValue;
#endif

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
    BOOLEAN ExtraIoInfoSupported;

#if (NTDDI_VERSION >= NTDDI_WIN10_FE)

    union {
        struct {
            UCHAR LogicalPoFxForDisk : 1;
            UCHAR ForwardIo : 1;
            UCHAR Reserved : 6;
        } DUMMYSTRUCTNAME;
        UCHAR AsUCHAR;
    } Flags;

    UCHAR   Reserved0[2];

#else
    UCHAR   Reserved0[3];
#endif

    ULONG   Reserved1;
#endif

} STORAGE_MINIPORT_DESCRIPTOR, *PSTORAGE_MINIPORT_DESCRIPTOR;

#pragma warning(pop)

//
// Storage identification descriptor.
// The definitions here are based on the SCSI/SBP vital product data
// device identifier page.
//

typedef enum _STORAGE_IDENTIFIER_CODE_SET {
    StorageIdCodeSetReserved = 0,
    StorageIdCodeSetBinary = 1,
    StorageIdCodeSetAscii = 2,
    StorageIdCodeSetUtf8 = 3
} STORAGE_IDENTIFIER_CODE_SET, *PSTORAGE_IDENTIFIER_CODE_SET;

typedef enum _STORAGE_IDENTIFIER_TYPE {
    StorageIdTypeVendorSpecific = 0,
    StorageIdTypeVendorId = 1,
    StorageIdTypeEUI64 = 2,
    StorageIdTypeFCPHName = 3,
    StorageIdTypePortRelative = 4,
    StorageIdTypeTargetPortGroup = 5,
    StorageIdTypeLogicalUnitGroup = 6,
    StorageIdTypeMD5LogicalUnitIdentifier = 7,
    StorageIdTypeScsiNameString = 8
} STORAGE_IDENTIFIER_TYPE, *PSTORAGE_IDENTIFIER_TYPE;

// Mislabeled above but need to keep it for backwards compatibility
#define StorageIdTypeNAA StorageIdTypeFCPHName

// NAA formats (Used with StorageIdTypeNAA)
typedef enum _STORAGE_ID_NAA_FORMAT {
        StorageIdNAAFormatIEEEExtended = 2,
        StorageIdNAAFormatIEEERegistered = 3,
        StorageIdNAAFormatIEEEERegisteredExtended = 5
} STORAGE_ID_NAA_FORMAT, *PSTORAGE_ID_NAA_FORMAT;

typedef enum _STORAGE_ASSOCIATION_TYPE {
    StorageIdAssocDevice = 0,
    StorageIdAssocPort = 1,
    StorageIdAssocTarget = 2
} STORAGE_ASSOCIATION_TYPE, *PSTORAGE_ASSOCIATION_TYPE;

typedef struct _STORAGE_IDENTIFIER {

    STORAGE_IDENTIFIER_CODE_SET CodeSet;

    STORAGE_IDENTIFIER_TYPE Type;

    USHORT IdentifierSize;

    USHORT NextOffset;

    //
    // Add new fields here since existing code depends on
    // the above layout not changing.
    //

    STORAGE_ASSOCIATION_TYPE Association;

    //
    // The identifier is a variable length array of bytes.
    //

    UCHAR Identifier[1];

} STORAGE_IDENTIFIER, *PSTORAGE_IDENTIFIER;

typedef _Struct_size_bytes_(Size) struct _STORAGE_DEVICE_ID_DESCRIPTOR {

    ULONG Version;

    ULONG Size;

    //
    // The number of identifiers reported by the device.
    //

    ULONG NumberOfIdentifiers;

    //
    // The following field is actually a variable length array of identification
    // descriptors.  Unfortunately there's no C notation for an array of
    // variable length structures so we're forced to just pretend.
    //

    UCHAR Identifiers[1];

} STORAGE_DEVICE_ID_DESCRIPTOR, *PSTORAGE_DEVICE_ID_DESCRIPTOR;

// output buffer for   StorageDeviceSeekPenaltyProperty & PropertyStandardQuery
typedef struct _DEVICE_SEEK_PENALTY_DESCRIPTOR {

    ULONG       Version;          // keep compatible with STORAGE_DESCRIPTOR_HEADER

    ULONG       Size;             // keep compatible with STORAGE_DESCRIPTOR_HEADER

    BOOLEAN     IncursSeekPenalty;

} DEVICE_SEEK_PENALTY_DESCRIPTOR, *PDEVICE_SEEK_PENALTY_DESCRIPTOR;

// output buffer for   StorageDeviceWriteAggregationProperty & PropertyStandardQuery
typedef struct _DEVICE_WRITE_AGGREGATION_DESCRIPTOR {
    ULONG       Version;          // keep compatible with STORAGE_DESCRIPTOR_HEADER
    ULONG       Size;             // keep compatible with STORAGE_DESCRIPTOR_HEADER

    BOOLEAN     BenefitsFromWriteAggregation;
} DEVICE_WRITE_AGGREGATION_DESCRIPTOR, *PDEVICE_WRITE_AGGREGATION_DESCRIPTOR;

// output buffer for   StorageDeviceTrimProperty & PropertyStandardQuery
typedef struct _DEVICE_TRIM_DESCRIPTOR {

    ULONG       Version;          // keep compatible with STORAGE_DESCRIPTOR_HEADER

    ULONG       Size;             // keep compatible with STORAGE_DESCRIPTOR_HEADER

    BOOLEAN     TrimEnabled;

} DEVICE_TRIM_DESCRIPTOR, *PDEVICE_TRIM_DESCRIPTOR;

#pragma warning(push)
#pragma warning(disable:4214) // bit fields other than int
//
// Output buffer for StorageDeviceLBProvisioningProperty & PropertyStandardQuery
//
typedef struct _DEVICE_LB_PROVISIONING_DESCRIPTOR {

    ULONG       Version;          // keep compatible with STORAGE_DESCRIPTOR_HEADER

    ULONG       Size;             // keep compatible with STORAGE_DESCRIPTOR_HEADER

    UCHAR ThinProvisioningEnabled : 1;

    UCHAR ThinProvisioningReadZeros : 1;

    UCHAR AnchorSupported : 3;

    UCHAR UnmapGranularityAlignmentValid : 1;

    UCHAR GetFreeSpaceSupported : 1;        // Supports DeviceDsmAction_GetFreeSpace

    UCHAR MapSupported : 1;                 // Supports DeviceDsmAction_Map

    UCHAR Reserved1[7];

    ULONGLONG OptimalUnmapGranularity;      // Granularity in bytes.

    ULONGLONG UnmapGranularityAlignment;    // Granularity alignment in bytes.

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

    ULONG MaxUnmapLbaCount;                 // Max LBAs that can be unmapped in a single UNMAP command, in logical blocks.

    ULONG MaxUnmapBlockDescriptorCount;     // Max number of descriptors allowed in a single UNMAP command.

#endif
} DEVICE_LB_PROVISIONING_DESCRIPTOR, *PDEVICE_LB_PROVISIONING_DESCRIPTOR;

#define DEVICE_LB_PROVISIONING_DESCRIPTOR_V1_SIZE     RTL_SIZEOF_THROUGH_FIELD(DEVICE_LB_PROVISIONING_DESCRIPTOR, UnmapGranularityAlignment)

//
// IOCTL_STORAGE_GET_LB_PROVISIONING_MAP_RESOURCES
//
// Input Buffer:
//      None
//
// Output Buffer:
//      Structure of type STORAGE_LB_PROVISIONING_MAP_RESOURCES
//

typedef struct _STORAGE_LB_PROVISIONING_MAP_RESOURCES {
    ULONG       Size;
    ULONG       Version;
    UCHAR       AvailableMappingResourcesValid : 1;
    UCHAR       UsedMappingResourcesValid : 1;
    UCHAR       Reserved0 : 6;
    UCHAR       Reserved1[3];
    UCHAR       AvailableMappingResourcesScope : 2; // See LOG_PAGE_LBP_RESOURCE_SCOPE_* definitions in scsi.h for scope values.
    UCHAR       UsedMappingResourcesScope : 2;
    UCHAR       Reserved2 : 4;
    UCHAR       Reserved3[3];
    ULONGLONG   AvailableMappingResources;  // Available LBA mapping resources, in bytes.
    ULONGLONG   UsedMappingResources;       // Used LBA mapping resources, in bytes.
} STORAGE_LB_PROVISIONING_MAP_RESOURCES, *PSTORAGE_LB_PROVISIONING_MAP_RESOURCES;

#pragma warning(pop)

// output buffer for   StorageDevicePowerProperty & PropertyStandardQuery
typedef struct _DEVICE_POWER_DESCRIPTOR {
    ULONG       Version;          // keep compatible with STORAGE_DESCRIPTOR_HEADER
    ULONG       Size;             // keep compatible with STORAGE_DESCRIPTOR_HEADER

    BOOLEAN     DeviceAttentionSupported;   // The device supports "device attention".
    BOOLEAN     AsynchronousNotificationSupported;  // The device supports asynchronous notifications, delivered via IOCTL_STORAGE_EVENT_NOTIFICATION.
    BOOLEAN     IdlePowerManagementEnabled; // The device has been registered for runtime idle power management.
    BOOLEAN     D3ColdEnabled;              // The device will be powered off when put into D3.
    BOOLEAN     D3ColdSupported;            // The platform supports D3Cold for this device.
    BOOLEAN     NoVerifyDuringIdlePower;    // Device require no verification during idle power transitions.
    UCHAR       Reserved[2];
    ULONG       IdleTimeoutInMS;            // The idle timeout value in milliseconds. Only valid if IdlePowerManagementEnabled == TRUE.
} DEVICE_POWER_DESCRIPTOR, *PDEVICE_POWER_DESCRIPTOR;

//
// Output buffer for StorageDeviceCopyOffloadProperty & PropertyStandardQuery
//
typedef struct _DEVICE_COPY_OFFLOAD_DESCRIPTOR {
    ULONG       Version;          // keep compatible with STORAGE_DESCRIPTOR_HEADER
    ULONG       Size;             // keep compatible with STORAGE_DESCRIPTOR_HEADER

    ULONG       MaximumTokenLifetime;
    ULONG       DefaultTokenLifetime;
    ULONGLONG   MaximumTransferSize;
    ULONGLONG   OptimalTransferCount;
    ULONG       MaximumDataDescriptors;
    ULONG       MaximumTransferLengthPerDescriptor;
    ULONG       OptimalTransferLengthPerDescriptor;
    USHORT      OptimalTransferLengthGranularity;
    UCHAR       Reserved[2];
} DEVICE_COPY_OFFLOAD_DESCRIPTOR, *PDEVICE_COPY_OFFLOAD_DESCRIPTOR;

//
// Output buffer for StorageDeviceResiliencyProperty & PropertyStandardQuery
//

typedef _Struct_size_bytes_(Size) struct _STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {

    //
    // Size of this structure serves
    // as the version
    //

    ULONG Version;

    //
    // Size of  this structure  plus
    // all the variable sized fields
    //

    ULONG Size;

    //
    // Friendly name associated with
    // this descriptor
    //

    ULONG NameOffset;

    //
    // Number of  logical  copies of
    // data that are available
    //

    ULONG NumberOfLogicalCopies;

    //
    // Number of  complete copies of
    // data that are stored
    //

    ULONG NumberOfPhysicalCopies;

    //
    // Number of disks that can fail
    // without leading to  data loss
    //

    ULONG PhysicalDiskRedundancy;

    //
    // Number  of columns associated
    // with this descriptor
    //

    ULONG NumberOfColumns;

    //
    // Stripe  width associated with
    // this descriptor, in bytes
    //

    ULONG Interleave;

} STORAGE_DEVICE_RESILIENCY_DESCRIPTOR, *PSTORAGE_DEVICE_RESILIENCY_DESCRIPTOR;

//
// Output buffer for StorageAdapterRpmbProperty & PropertyStandardQuery
//

typedef enum _STORAGE_RPMB_FRAME_TYPE {

    StorageRpmbFrameTypeUnknown = 0,
    StorageRpmbFrameTypeStandard,
    StorageRpmbFrameTypeMax,

} STORAGE_RPMB_FRAME_TYPE, *PSTORAGE_RPMB_FRAME_TYPE;

#define STORAGE_RPMB_DESCRIPTOR_VERSION_1             1

#define STORAGE_RPMB_MINIMUM_RELIABLE_WRITE_SIZE      512

typedef struct _STORAGE_RPMB_DESCRIPTOR {

    //
    // Keep compatible with STORAGE_DESCRIPTOR_HEADER
    // Shall be set to STORAGE_RPMB_DESCRIPTOR_VERSION_1
    //

    ULONG Version;

    //
    // Keep compatible with STORAGE_DESCRIPTOR_HEADER
    // Shall be set to sizeof(STORAGE_RPMB_DESCRIPTOR)
    //

    ULONG Size;

    //
    // The size of the RPMB, in bytes.
    //
    // 0 if not supported, RPMB size in bytes otherwise
    //

    ULONG SizeInBytes;

    //
    // The maximum amount of data supported in one transaction
    // in bytes.
    //
    // 0 if not supported, minimum 512 bytes
    //

    ULONG MaxReliableWriteSizeInBytes;

    //
    // To support different RPMB frame formats, specify which
    // frame format the payload will be in so the port driver
    // can take the appropriate action
    //

    STORAGE_RPMB_FRAME_TYPE FrameFormat;

} STORAGE_RPMB_DESCRIPTOR, *PSTORAGE_RPMB_DESCRIPTOR;

// begin_storport begin_privstorport

#ifndef STORAGE_CRYPTO_ALGORITHMS_DEFINED
#define STORAGE_CRYPTO_ALGORITHMS_DEFINED

//
// Output buffer for StorageAdapterCryptoProperty & PropertyStandardQuery
//

typedef enum _STORAGE_CRYPTO_ALGORITHM_ID {

    StorageCryptoAlgorithmUnknown = 0,
    StorageCryptoAlgorithmXTSAES = 1,
    StorageCryptoAlgorithmBitlockerAESCBC,
    StorageCryptoAlgorithmAESECB,
    StorageCryptoAlgorithmESSIVAESCBC,
    StorageCryptoAlgorithmMax,

    //
    // Legacy compatibility algorithm names.
    // Use the names above.
    //

    StorCryptoAlgorithmUnknown = StorageCryptoAlgorithmUnknown,
    StorCryptoAlgorithmXTSAES = StorageCryptoAlgorithmXTSAES,
    StorCryptoAlgorithmBitlockerAESCBC = StorageCryptoAlgorithmBitlockerAESCBC,
    StorCryptoAlgorithmAESECB = StorageCryptoAlgorithmAESECB,
    StorCryptoAlgorithmESSIVAESCBC = StorageCryptoAlgorithmESSIVAESCBC,
} STORAGE_CRYPTO_ALGORITHM_ID, *PSTORAGE_CRYPTO_ALGORITHM_ID;

typedef enum _STORAGE_CRYPTO_KEY_SIZE {

    StorageCryptoKeySizeUnknown = 0,
    StorageCryptoKeySize128Bits = 1,
    StorageCryptoKeySize192Bits,
    StorageCryptoKeySize256Bits,
    StorageCryptoKeySize512Bits,
    StorageCryptoKeySizeMax,

    //
    // Legacy compatibility key size names.
    // Use the names above.
    //
    StorCryptoKeySizeUnknown = StorageCryptoKeySizeUnknown,
    StorCryptoKeySize128Bits = StorageCryptoKeySize128Bits,
    StorCryptoKeySize192Bits = StorageCryptoKeySize192Bits,
    StorCryptoKeySize256Bits = StorageCryptoKeySize256Bits,
    StorCryptoKeySize512Bits = StorageCryptoKeySize512Bits,
} STORAGE_CRYPTO_KEY_SIZE, *PSTORAGE_CRYPTO_KEY_SIZE;

#endif // STORAGE_CRYPTO_ALGORITHMS_DEFINED

// end_storport end_privstorport

#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/unions

#define STORAGE_CRYPTO_CAPABILITY_VERSION_1           1

//
// Note: Starting in Win11 24H2 and WS2025 or GE, this struct is deprecated. 
// Use STORAGE_HW_CRYPTO_CAPABILITY.
//
typedef struct _STORAGE_CRYPTO_CAPABILITY {

    //
    // To enable versioning of this structure. This shall be set
    // to STORAGE_CRYPTO_CAPABILITY_VERSION_1
    //

    ULONG Version;

    //
    // Size of this structure. This shall be set to
    // sizeof(STORAGE_CRYPTO_CAPABILITY)
    //

    ULONG Size;

    //
    // The index for this crypto capability
    //

    ULONG CryptoCapabilityIndex;

    //
    // Supported algorithm for this crypto capability
    //

    STORAGE_CRYPTO_ALGORITHM_ID AlgorithmId;

    //
    // The supported key size for this algorithm
    //

    STORAGE_CRYPTO_KEY_SIZE KeySize;

    //
    // Bitmask for the supported sizes of encryptable data blocks. When bit
    // j is set (j=0...7), a data unit size of 512*2^j bytes is supported.
    // Bit 0 represents 512 bytes, 1 represents 1 KB, bit 7 represents 64 KB
    //

    ULONG DataUnitSizeBitmask;

} STORAGE_CRYPTO_CAPABILITY, *PSTORAGE_CRYPTO_CAPABILITY;

#define STORAGE_CRYPTO_CAPABILITY_VERSION_2           2

// begin_storport begin_privstorport

#ifndef STORAGE_SECURITY_COMPLIANCE_BITMASK_DEFINED
#define STORAGE_SECURITY_COMPLIANCE_BITMASK_DEFINED

typedef union _STORAGE_SECURITY_COMPLIANCE_BITMASK {
    struct {
        UCHAR FIPS : 1;
        UCHAR Reserved : 7;
    };
    UCHAR AsUchar;
} STORAGE_SECURITY_COMPLIANCE_BITMASK;

#endif

#ifndef STORAGE_CRYPTO_KEY_TYPE_DEFINED
#define STORAGE_CRYPTO_KEY_TYPE_DEFINED

typedef union _STORAGE_CRYPTO_KEY_TYPE {
    struct {
        UCHAR DirectKey : 1;
        UCHAR PlatformWrappedKey : 1;
        UCHAR PlutonWrappedKey : 1;
        UCHAR Reserved : 5;
    };
    UCHAR AsUchar;
} STORAGE_CRYPTO_KEY_TYPE;

#endif

// end_storport end_privstorport

typedef struct _STORAGE_CRYPTO_CAPABILITY_V2 {

    //
    // To enable versioning of this structure. This shall be set
    // to STORAGE_CRYPTO_CAPABILITY_VERSION_2
    //

    ULONG Version;

    //
    // Size of this structure. This shall be set to
    // sizeof(STORAGE_CRYPTO_CAPABILITY_V2)
    //

    ULONG Size;

    //
    // The index for this crypto capability
    //

    ULONG CryptoCapabilityIndex;

    //
    // Supported algorithm for this crypto capability
    //

    STORAGE_CRYPTO_ALGORITHM_ID AlgorithmId;

    //
    // The supported key size for this algorithm
    //

    STORAGE_CRYPTO_KEY_SIZE KeySize;

    //
    // Bitmask for the supported sizes of encryptable data blocks. When bit
    // j is set (j=0...7), a data unit size of 512*2^j bytes is supported.
    // Bit 0 represents 512 bytes, 1 represents 1 KB, bit 7 represents 64 KB
    //

    ULONG DataUnitSizeBitmask;

    //
    // Maximum supported initialization vector bit size. This can be 0 if
    // this concept does not apply to the algorithm.
    //

    USHORT MaxIVBitSize;
    USHORT Reserved;

    //
    // Bitmask of compliant security standards at the algorithm level.
    //

    STORAGE_SECURITY_COMPLIANCE_BITMASK SecurityComplianceBitmask;

} STORAGE_CRYPTO_CAPABILITY_V2, *PSTORAGE_CRYPTO_CAPABILITY_V2;

#define STORAGE_CRYPTO_DESCRIPTOR_VERSION_1           1

//
// Note: Starting in Win11 24H2 and WS2025 or GE, this structure is deprecated.
// Use STORAGE_HW_CRYPTO_DESCRIPTOR.
//
typedef struct _STORAGE_CRYPTO_DESCRIPTOR {

    //
    // Keep compatible with STORAGE_DESCRIPTOR_HEADER
    // Shall be set to STORAGE_CRYPTO_DESCRIPTOR_VERSION_1
    //

    ULONG Version;

    //
    // Keep compatible with STORAGE_DESCRIPTOR_HEADER
    // Shall be set to sizeof(STORAGE_CRYPTO_DESCRIPTOR)
    //

    ULONG Size;

    //
    // The number of keys the crypto engine in the adapter supports
    //

    ULONG NumKeysSupported;

    //
    // The number of crypto capability entries. This outlines the
    // crypto configurations the adapter supports
    //

    ULONG NumCryptoCapabilities;

    //
    // Array of Crypto Capabilities
    //

    _Field_size_(NumCryptoCapabilities) STORAGE_CRYPTO_CAPABILITY CryptoCapabilities[ANYSIZE_ARRAY];

} STORAGE_CRYPTO_DESCRIPTOR, *PSTORAGE_CRYPTO_DESCRIPTOR;

#define STORAGE_CRYPTO_DESCRIPTOR_VERSION_2           2

typedef enum _STORAGE_ICE_TYPE {

    StorageIceTypeUnknown = 0,
    StorageIceTypeUfs,
    StorageIceTypeNvme,

} STORAGE_ICE_TYPE, *PSTORAGE_ICE_TYPE;

//
// Note: Starting in Win11 24H2 and WS2025 or GE, this structure is deprecated. 
// Use STORAGE_HW_CRYPTO_DESCRIPTOR.
//
typedef struct _STORAGE_CRYPTO_DESCRIPTOR_V2 {

    //
    // Keep compatible with STORAGE_DESCRIPTOR_HEADER
    // Shall be set to STORAGE_CRYPTO_DESCRIPTOR_VERSION_2
    //

    ULONG Version;

    //
    // Keep compatible with STORAGE_DESCRIPTOR_HEADER
    // Shall be set to sizeof(STORAGE_CRYPTO_DESCRIPTOR_V2)
    //

    ULONG Size;

    //
    // The number of keys the crypto engine in the adapter supports
    //

    ULONG NumKeysSupported;

    //
    // The number of crypto capability entries. This outlines the
    // crypto configurations the adapter supports
    //

    ULONG NumCryptoCapabilities;

    //
    // Which type of inline crypto engine this is
    //

    STORAGE_ICE_TYPE IceType;

    //
    // Bitmask of compliant security standards.
    //

    STORAGE_SECURITY_COMPLIANCE_BITMASK SecurityComplianceBitmask;

#if (NTDDI_VERSION >= NTDDI_WIN11_DT)

    //
    // Bitmask of supported key types.
    //

    STORAGE_CRYPTO_KEY_TYPE KeyTypeBitmask;
#endif

    //
    // Array of Crypto Capabilities.
    // NOTE: You cannot index into this array.
    //       Instead compute the next offset as
    //       curCryptoCapability =
    //          (STORAGE_CRYPTO_CAPABILITY_V2*)((PUCHAR)curCryptoCapability + curCryptoCapability->Size)
    //

    _Field_size_(NumCryptoCapabilities) STORAGE_CRYPTO_CAPABILITY_V2 CryptoCapabilities[ANYSIZE_ARRAY];

} STORAGE_CRYPTO_DESCRIPTOR_V2, *PSTORAGE_CRYPTO_DESCRIPTOR_V2;

//
// Output buffer for StorageHwCryptoProperty
//

#define STORAGE_HW_CRYPTO_CAPABILITY_VERSION_1           1

typedef struct _STORAGE_HW_CRYPTO_CAPABILITY {

    //
    // To enable versioning of this structure. This shall be set
    // to STORAGE_HW_CRYPTO_CAPABILITY_VERSION_1
    //

    ULONG Version;

    //
    // Size of this structure. This shall be set to
    // sizeof(STORAGE_HW_CRYPTO_CAPABILITY)
    //

    ULONG Size;

    //
    // The index for this crypto capability
    //

    ULONG CryptoCapabilityIndex;

    //
    // Supported algorithm for this crypto capability
    //

    STORAGE_CRYPTO_ALGORITHM_ID AlgorithmId;

    //
    // The supported key size for this algorithm
    //

    STORAGE_CRYPTO_KEY_SIZE KeySize;

    //
    // Bitmask for the supported sizes of encryptable data blocks. When bit
    // j is set (j=0...7), a data unit size of 512*2^j bytes is supported.
    // Bit 0 represents 512 bytes, 1 represents 1 KB, bit 7 represents 64 KB
    //

    ULONG DataUnitSizeBitmask;

    //
    // Maximum supported initialization vector bit size. This can be 0 if
    // this concept does not apply to the algorithm.
    //

    USHORT MaxIVBitSize;
    USHORT Reserved;

    //
    // Bitmask of compliant security standards at the algorithm level.
    //

    STORAGE_SECURITY_COMPLIANCE_BITMASK SecurityComplianceBitmask;

} STORAGE_HW_CRYPTO_CAPABILITY, *PSTORAGE_HW_CRYPTO_CAPABILITY;

#define STORAGE_HW_CRYPTO_DESCRIPTOR_VERSION_1           1

typedef struct _STORAGE_HW_CRYPTO_DESCRIPTOR {

    //
    // Header.Version is set to STORAGE_HW_CRYPTO_DESCRIPTOR_VERSION_1
    // to enable future version updates.
    //
    // Header.Size is set to the size of the entire buffer, including
    // the trailing array of crypto capabilities.
    //

    STORAGE_DESCRIPTOR_HEADER Header;

    //
    // The number of keys the crypto engine supports
    //

    ULONG NumKeysSupported;

    //
    // The number of crypto capability entries. This outlines the
    // crypto configurations the crypto engine supports.
    //

    ULONG NumCryptoCapabilities;

    //
    // Offset to an array of STORAGE_HW_CRYPTO_CAPABILITY
    // structures from the beginning of STORAGE_HW_CRYPTO_DESCRIPTOR.
    // Use STORAGE_HW_CRYPTO_CAPABILITY::Size to iterate through the
    // elements.
    //

    _Field_range_(sizeof(struct _STORAGE_HW_CRYPTO_DESCRIPTOR), Header.Size)
    ULONG OffsetToCryptoCapabilities;

    //
    // Size of each crypto capability array element.
    //

    ULONG SizeOfCryptoCapability;

    //
    // Which type of inline crypto engine this is
    //

    STORAGE_ICE_TYPE IceType;

    //
    // Bitmask of compliant security standards.
    //

    STORAGE_SECURITY_COMPLIANCE_BITMASK SecurityComplianceBitmask;

    //
    // Bitmask of supported key types.
    //

    STORAGE_CRYPTO_KEY_TYPE KeyTypeBitmask;

    //
    // The following array exists at `OffsetToCryptoCapabilities`.
    // Each element must be `SizeOfCryptoCapability` in size.
    //
    // STORAGE_HW_CRYPTO_CAPABILITY Capabilities[]
    //

} STORAGE_HW_CRYPTO_DESCRIPTOR, *PSTORAGE_HW_CRYPTO_DESCRIPTOR;

FORCEINLINE
const STORAGE_HW_CRYPTO_CAPABILITY *
GetStorageHwCryptoCapability (
    const STORAGE_HW_CRYPTO_DESCRIPTOR *CryptoDescriptor,
    ULONG Index
    )
{
    SIZE_T Offset = CryptoDescriptor->OffsetToCryptoCapabilities +
                    Index * CryptoDescriptor->SizeOfCryptoCapability;

#if defined(NT_ASSERT)
    NT_ASSERT(Offset <= CryptoDescriptor->Header.Size);
#endif

    return (STORAGE_HW_CRYPTO_CAPABILITY *)((const char *)CryptoDescriptor + Offset);
}

//
// Same as GetStorageHwCryptoCapability except returns a non const (mutable)
// pointer. Useful when creating a storage crypto descriptor.
//
FORCEINLINE
STORAGE_HW_CRYPTO_CAPABILITY *
GetStorageHwCryptoCapabilityMut (
    _In_reads_bytes_(CryptoDescriptor->Header.Size) STORAGE_HW_CRYPTO_DESCRIPTOR *CryptoDescriptor,
    ULONG Index
    )
{
    SIZE_T Offset = CryptoDescriptor->OffsetToCryptoCapabilities +
                    Index * CryptoDescriptor->SizeOfCryptoCapability;

#if defined(NT_ASSERT)
    NT_ASSERT(Offset <= CryptoDescriptor->Header.Size);
#endif

    return (STORAGE_HW_CRYPTO_CAPABILITY *)((char *)CryptoDescriptor + Offset);
}
#pragma warning(pop)

// end_winioctl
// begin_winioctl

//
//  The STORAGE_TIER is an identifier for the storage tier relative to the volume/LUN.
//  The storage tier ID for a particular volume has no relationship to the storage tier
//  ID with the same value on a different volume.
//

#define STORAGE_TIER_NAME_LENGTH           (256)
#define STORAGE_TIER_DESCRIPTION_LENGTH    (512)

#define STORAGE_TIER_FLAG_NO_SEEK_PENALTY  (0x00020000)
#define STORAGE_TIER_FLAG_WRITE_BACK_CACHE (0x00200000)
#define STORAGE_TIER_FLAG_READ_CACHE       (0x00400000)
#define STORAGE_TIER_FLAG_PARITY           (0x00800000)
#define STORAGE_TIER_FLAG_SMR              (0x01000000)

typedef enum _STORAGE_TIER_MEDIA_TYPE {

    StorageTierMediaTypeUnspecified = 0,
    StorageTierMediaTypeDisk        = 1,
    StorageTierMediaTypeSsd         = 2,
    StorageTierMediaTypeScm         = 4,
    StorageTierMediaTypeMax

} STORAGE_TIER_MEDIA_TYPE, *PSTORAGE_TIER_MEDIA_TYPE;

typedef enum _STORAGE_TIER_CLASS {

    StorageTierClassUnspecified = 0,
    StorageTierClassCapacity,
    StorageTierClassPerformance,
    StorageTierClassMax

} STORAGE_TIER_CLASS, *PSTORAGE_TIER_CLASS;

typedef struct _STORAGE_TIER {

    //
    // Tier ID
    //

    GUID Id;

    //
    // Name for the tier
    //

    WCHAR Name[STORAGE_TIER_NAME_LENGTH];

    //
    // Note for the tier
    //

    WCHAR Description[STORAGE_TIER_NAME_LENGTH];

    //
    // Flags: STORAGE_TIER_FLAG_xxx
    //

    ULONGLONG Flags;

    //
    // Provisioned capacity of the tier
    //

    ULONGLONG ProvisionedCapacity;

    //
    // Media type of the tier
    //

    STORAGE_TIER_MEDIA_TYPE MediaType;

    //
    // Classification of the tier
    //

    STORAGE_TIER_CLASS Class;

} STORAGE_TIER, *PSTORAGE_TIER;

//
//  The response returns a single structure of STORAGE_DEVICE_TIERING_DESCRIPTOR that has
//  all the tiers for this disk.
//

typedef _Struct_size_bytes_(Size) struct _STORAGE_DEVICE_TIERING_DESCRIPTOR {

    //
    // Size of this structure serves
    // as the version
    //

    ULONG Version;

    //
    // Size of  this structure  plus
    // all the variable sized fields
    //

    ULONG Size;

    //
    // Flags. The upper USHORT of these flags is reserved for file system use as
    // this structure is returned slightly tweaked in FSCTL_QUERY_STORAGE_CLASSES_OUTPUT.
    //

    ULONG Flags;

    //
    // The total number of available tiers for this disk
    //

    ULONG TotalNumberOfTiers;

    //
    // The number of tiers that fit in the output
    //

    ULONG NumberOfTiersReturned;

    //
    // Detailed info on the storage tiers.
    //

    _Field_size_(NumberOfTiersReturned) STORAGE_TIER Tiers[ANYSIZE_ARRAY];

} STORAGE_DEVICE_TIERING_DESCRIPTOR, *PSTORAGE_DEVICE_TIERING_DESCRIPTOR;

//
// Output buffer for StorageDeviceFaultDomainProperty & PropertyStandardQuery
//

typedef _Struct_size_bytes_(Size) struct _STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {

    //
    // Size of this structure serves
    // as the version
    //

    ULONG Version;

    //
    // Size of  this structure  plus
    // all the variable sized fields
    //

    ULONG Size;

    //
    // Number of fault domains
    //

    ULONG NumberOfFaultDomains;

    //
    // Fault domain ids
    //

    _Field_size_(NumberOfFaultDomains)
    GUID FaultDomainIds[ANYSIZE_ARRAY];

} STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR, *PSTORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR;

//
// Parameters for StorageAdapterProtocolSpecificProperty (or StorageDeviceProtocolSpecificProperty) & PropertyStandardQuery
//

//
// Define the different storage command protocols that used between software and hardware.
// e.g. command protocol software uses to communicate with hardware.
// Protocol types below 128 (0x80) are reserved for Microsoft use.
//
typedef enum _STORAGE_PROTOCOL_TYPE {
    ProtocolTypeUnknown = 0x00,
    ProtocolTypeScsi,
    ProtocolTypeAta,
    ProtocolTypeNvme,
    ProtocolTypeSd,
    ProtocolTypeUfs,
    ProtocolTypeProprietary = 0x7E,
    ProtocolTypeMaxReserved = 0x7F
} STORAGE_PROTOCOL_TYPE, *PSTORAGE_PROTOCOL_TYPE;


typedef enum _STORAGE_PROTOCOL_NVME_DATA_TYPE {
    NVMeDataTypeUnknown = 0,

    NVMeDataTypeIdentify,       // Retrieved by command - IDENTIFY CONTROLLER or IDENTIFY NAMESPACE
                                // Corresponding values in STORAGE_PROTOCOL_SPECIFIC_DATA,
                                //      ProtocolDataRequestValue - CNS as defined in NVME_IDENTIFY_CNS_CODES
                                //      ProtocolDataRequestSubValue - Namespace Id
                                //      ProtocolDataRequestSubValue2 - CNS Specific Id (CNSID)
                                //      ProtocolDataRequestSubValue3 - Controller Id (CNTID)
                                //      ProtocolDataRequestSubValue4 - Command Set Identifier (CSI)

    NVMeDataTypeLogPage,        // Retrieved by command - GET LOG PAGE
                                // Corresponding values in STORAGE_PROTOCOL_SPECIFIC_DATA,
                                //      ProtocolDataRequestValue - Log page id
                                //      ProtocolDataRequestSubValue - Lower 32-bit offset value
                                //      ProtocolDataRequestSubValue2 - Upper 32-bit offset value
                                //      ProtocolDataRequestSubValue3 - Log specific identifier
                                //      ProtocolDataRequestSubValue4 - STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE

    NVMeDataTypeFeature,        // Retrieved by command - GET FEATURES or SET FEATURES
                                // Corresponding values in STORAGE_PROTOCOL_SPECIFIC_DATA (get) or STORAGE_PROTOCOL_SPECIFIC_DATA_EXT (set),
                                //      ProtocolDataRequestValue - Defined in NVME_CDW10_GET_FEATURES / NVME_CDW10_SET_FEATURES
                                //      ProtocolDataRequestSubValue - Defined in NVME_CDW11_FEATURES
                                //      ProtocolDataRequestSubValue2 - Defined in NVME_CDW12_FEATURES
                                //      ProtocolDataRequestSubValue3 - Defined in NVME_CDW13_FEATURES
                                //      ProtocolDataRequestSubValue4 - Defined in NVME_CDW14_FEATURES
                                //      ProtocolDataRequestSubValue5 - Defined in NVME_CDW15_FEATURES

    NVMeDataTypeLogPageEx,      // Retrieved by command - GET LOG PAGE
                                // Corresponding values in STORAGE_PROTOCOL_SPECIFIC_DATA_EXT,
                                //      ProtocolDataValue - Defined in NVME_CDW10_GET_LOG_PAGE
                                //      ProtocolDataSubValue - Defined in NVME_CDW11_GET_LOG_PAGE
                                //      ProtocolDataSubValue2 - Defined in NVME_CDW12_GET_LOG_PAGE
                                //      ProtocolDataSubValue3 - Defined in NVME_CDW13_GET_LOG_PAGE
                                //      ProtocolDataSubValue4 - Defined in NVME_CDW14_GET_LOG_PAGE
                                //      ProtocolDataSubValue5 - Defined in NVME_CDW15_GET_LOG_PAGE (not used currently)
                                //      ProtocolDataSubValue6 - Namespace ID

    NVMeDataTypeFeatureEx,      // Retrieved by command - GET FEATURES or SET FEATURES
                                // Corresponding values in STORAGE_cd PROTOCOL_SPECIFIC_DATA_EXT,
                                //      ProtocolDataValue - Defined in NVME_CDW10_GET_FEATURES / NVME_CDW10_SET_FEATURES
                                //      ProtocolDataSubValue - Defined in NVME_CDW11_FEATURES
                                //      ProtocolDataSubValue2 - Defined in NVME_CDW12_FEATURES
                                //      ProtocolDataSubValue3 - Defined in NVME_CDW13_FEATURES
                                //      ProtocolDataSubValue4 - Defined in NVME_CDW14_FEATURES
                                //      ProtocolDataSubValue5 - Defined in NVME_CDW15_FEATURES
                                //      ProtocolDataSubValue6 - Namespace ID

    // For NVMeDataTypeLogPageEx and NVMeDataTypeFeatureEx the namespace ID field is only used for requests sent to
    // an adapter or controller.  In these scenarios, the caller sets ProtocolDataSubValue6 to either 0 (NSID not used) or
    // FFFFFFFFF (request applies to all namespaces). For requests being targeted at a disk, the storage stack driver
    // will substitute in the corresponding NSID automatically.  Callers must set ProtocolDataSubValue6 to 0 for these requests.

} STORAGE_PROTOCOL_NVME_DATA_TYPE, *PSTORAGE_PROTOCOL_NVME_DATA_TYPE;

typedef enum _STORAGE_PROTOCOL_ATA_DATA_TYPE {
    AtaDataTypeUnknown = 0,
    AtaDataTypeIdentify,        // Retrieved by command - IDENTIFY DEVICE
    AtaDataTypeLogPage,         // Retrieved by command - READ LOG EXT
} STORAGE_PROTOCOL_ATA_DATA_TYPE, *PSTORAGE_PROTOCOL_ATA_DATA_TYPE;

typedef enum _STORAGE_PROTOCOL_UFS_DATA_TYPE {
    UfsDataTypeUnknown = 0,
    UfsDataTypeQueryDescriptor,         // Retrieved by command - QUERY UPIU
    UfsDataTypeQueryAttribute,          // Retrieved by command - QUERY UPIU
    UfsDataTypeQueryFlag,               // Retrieved by command - QUERY UPIU
    UfsDataTypeQueryDmeAttribute,       // Retrieved by command - QUERY UPIU
    UfsDataTypeQueryDmePeerAttribute,   // Retrieved by command - QUERY UPIU
    UfsDataTypeMax,
} STORAGE_PROTOCOL_UFS_DATA_TYPE, *PSTORAGE_PROTOCOL_UFS_DATA_TYPE;

//
// Below definition is used to specify particular command fields when querying
// NVMeDataTypeLogPage, and this definition maps to ProtocolDataRequestSubValue4
// field in STORAGE_PROTOCOL_SPECIFIC_DATA.
//
#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/unions
#pragma warning(disable:4214) // bit fields other than int to disable this around the struct

typedef union _STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE {

    struct {

        ULONG RetainAsynEvent   :  1;
        ULONG LogSpecificField  :  4;
        ULONG Reserved0         :  3;
        ULONG UUIDIndex         :  7;
        ULONG Reserved          : 17;

    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE, *PSTORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE;

#pragma warning(pop)

//
// Protocol Data should follow this data structure in the same buffer.
// The offset of Protocol Data from the beginning of this data structure
// is reported in data field - "ProtocolDataOffset".
//
typedef struct _STORAGE_PROTOCOL_SPECIFIC_DATA {

    STORAGE_PROTOCOL_TYPE ProtocolType;
    ULONG   DataType;                     // The value will be protocol specific, as defined in STORAGE_PROTOCOL_NVME_DATA_TYPE or STORAGE_PROTOCOL_ATA_DATA_TYPE.

    ULONG   ProtocolDataRequestValue;
    ULONG   ProtocolDataRequestSubValue;  // Data sub request value

    ULONG   ProtocolDataOffset;           // The offset of data buffer is from beginning of this data structure.
    ULONG   ProtocolDataLength;

    ULONG   FixedProtocolReturnData;
    ULONG   ProtocolDataRequestSubValue2; // First additional data sub request value

    ULONG   ProtocolDataRequestSubValue3; // Second additional data sub request value
    ULONG   ProtocolDataRequestSubValue4; // Third additional data sub request value

} STORAGE_PROTOCOL_SPECIFIC_DATA, *PSTORAGE_PROTOCOL_SPECIFIC_DATA;

//
// Extended type incorporates both Get/Set protocol data
// Protocol Data should follow this data structure in the same buffer.
// The offset of Protocol Data from the beginning of this data structure
// is reported in data field - "ProtocolDataOffset".
//
typedef struct _STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {

    STORAGE_PROTOCOL_TYPE ProtocolType;
    ULONG   DataType;                  // The value will be protocol specific, as defined in STORAGE_PROTOCOL_NVME_DATA_TYPE or STORAGE_PROTOCOL_ATA_DATA_TYPE.

    ULONG   ProtocolDataValue;
    ULONG   ProtocolDataSubValue;      // Data sub request value

    ULONG   ProtocolDataOffset;        // The offset of data buffer is from beginning of this data structure.
    ULONG   ProtocolDataLength;

    ULONG   FixedProtocolReturnData;
    ULONG   ProtocolDataSubValue2;     // First additional data sub request value

    ULONG   ProtocolDataSubValue3;     // Second additional data sub request value
    ULONG   ProtocolDataSubValue4;     // Third additional data sub request value

    ULONG   ProtocolDataSubValue5;     // Fourth additional data sub request value
    ULONG   ProtocolDataSubValue6;     // Fifth additional data sub request value

    ULONG   Reserved[4];

} STORAGE_PROTOCOL_SPECIFIC_DATA_EXT, *PSTORAGE_PROTOCOL_SPECIFIC_DATA_EXT;

//
// Input parameter for StorageAdapterProtocolSpecificProperty (or StorageDeviceProtocolSpecificProperty) & PropertyStandardQuery
// will be data structure STORAGE_PROPERTY_QUERY, where the data field "AdditionalParameters" is a buffer
// in format of STORAGE_PROTOCOL_SPECIFIC_DATA.
//

//
// Out parameter for StorageAdapterProtocolSpecificProperty (or StorageDeviceProtocolSpecificProperty) & PropertyStandardQuery
// will be STORAGE_PROTOCOL_DATA_DESCRIPTOR.
//
typedef struct _STORAGE_PROTOCOL_DATA_DESCRIPTOR {

    ULONG   Version;
    ULONG   Size;

    STORAGE_PROTOCOL_SPECIFIC_DATA ProtocolSpecificData;

} STORAGE_PROTOCOL_DATA_DESCRIPTOR, *PSTORAGE_PROTOCOL_DATA_DESCRIPTOR;

//
// Input parameter for StorageAdapterProtocolSpecificProperty (or StorageDeviceProtocolSpecificProperty) & PropertyStandardSet
// will be data structure STORAGE_PROPERTY_SET, where the data field "AdditionalParameters" is a buffer
// in format of STORAGE_PROTOCOL_SPECIFIC_DATA_EXT.
//

//
// Input parameter for StorageAdapterProtocolSpecificPropertyEx (or StorageDeviceProtocolSpecificPropertyEx) & PropertyStandardQuery (or PropertyStandardSet)
// will be data structure STORAGE_PROPERTY_QUERY/STORAGE_PROPERTY_SET, where the data field "AdditionalParameters" is a buffer
// in format of STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT.
//
// N.B. this differs from the non-Ex properties which use STORAGE_PROTOCOL_SPECIFIC_DATA_EXT in the AdditionalParameters field.
//

//
// Out parameter for StorageAdapterProtocolSpecificProperty/StorageAdapterProtocolSpecificPropertyEx
// (or StorageDeviceProtocolSpecificProperty/StorageDeviceProtocolSpecificPropertyEx) & PropertyStandardSet
// will be STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT.
//
typedef struct _STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT {

    ULONG   Version;
    ULONG   Size;

    STORAGE_PROTOCOL_SPECIFIC_DATA_EXT ProtocolSpecificData;

} STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT, *PSTORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT;

//
// For StorageAdapterProtocolSpecificPropertyEx/StorageDeviceProtocolSpecificPropertyEx we require an actual
// version whereas the older properties used the sizeof as a version.
//
#define STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT_VERSION 1

//
// Parameters for StorageAdapterTemperatureProperty (or StorageDeviceTemperatureProperty) & PropertyStandardQuery
//


//
// Input parameters for StorageAdapterTemperatureProperty (or StorageDeviceTemperatureProperty) & PropertyStandardQuery
// uses data structure STORAGE_PROPERTY_QUERY.
//

//
// Out parameters for StorageAdapterTemperatureProperty (or StorageDeviceTemperatureProperty) & PropertyStandardQuery
// For temperature/threshold data fields, the smallest value of SHORT type - 0x8000 indicates the value is not reported.
//
#define STORAGE_TEMPERATURE_VALUE_NOT_REPORTED           0x8000

typedef struct _STORAGE_TEMPERATURE_INFO {

    USHORT  Index;                      // Starts from 0. Index 0 may indicate a composite value.
    SHORT   Temperature;                // Signed value; in Celsius.
    SHORT   OverThreshold;              // Signed value; in Celsius.
    SHORT   UnderThreshold;             // Signed value; in Celsius.

    BOOLEAN OverThresholdChangable;     // Can the threshold value being changed by using IOCTL_STORAGE_SET_TEMPERATURE_THRESHOLD.
    BOOLEAN UnderThresholdChangable;    // Can the threshold value being changed by using IOCTL_STORAGE_SET_TEMPERATURE_THRESHOLD.
    BOOLEAN EventGenerated;             // Indicates that notification will be generated when temperature cross threshold.
    UCHAR   Reserved0;
    ULONG   Reserved1;

} STORAGE_TEMPERATURE_INFO, *PSTORAGE_TEMPERATURE_INFO;

typedef struct _STORAGE_TEMPERATURE_DATA_DESCRIPTOR {

    ULONG   Version;
    ULONG   Size;

    //
    // Indicates the maximum temperature in degrees Celsius that may prevent continued normal operation,
    // possibility of data loss, automatic device shutdown, extreme performance throttling, or permanent damage.
    //
    SHORT   CriticalTemperature;    // Signed value; in Celsius.

    //
    // Indicates the maximum temperature in degrees Celsius at which the device is capable of
    // operating continuously without degrading operation or reliability.
    //
    SHORT   WarningTemperature;     // Signed value; in Celsius.

    USHORT  InfoCount;              // Some devices may report more than one temperature information as there can be multiple sensors implemented.

    UCHAR   Reserved0[2];

    ULONG   Reserved1[2];

    STORAGE_TEMPERATURE_INFO TemperatureInfo[ANYSIZE_ARRAY];

} STORAGE_TEMPERATURE_DATA_DESCRIPTOR, *PSTORAGE_TEMPERATURE_DATA_DESCRIPTOR;


//
// Input parameters for IOCTL_STORAGE_SET_TEMPERATURE_THRESHOLD
//

//
// Indicate the target of the request other than the device handle/object itself.
// This is used in "Flags" field of data structures.
//
#define STORAGE_TEMPERATURE_THRESHOLD_FLAG_ADAPTER_REQUEST       0x0001

typedef struct _STORAGE_TEMPERATURE_THRESHOLD {

    ULONG   Version;
    ULONG   Size;

    USHORT  Flags;
    USHORT  Index;

    SHORT   Threshold;          // Signed value; in Celsius.
    BOOLEAN OverThreshold;      // If TRUE, set the OverThreshold value; Otherwise, set the UnderThreshold value.
    UCHAR   Reserved;

} STORAGE_TEMPERATURE_THRESHOLD, *PSTORAGE_TEMPERATURE_THRESHOLD;

//
// Parameters for StorageAdapterPhysicalTopologyProperty (or StorageDevicePhysicalTopologyProperty) & PropertyStandardQuery
//


//
// Input parameters for StorageAdapterPhysicalTopologyProperty (or StorageDevicePhysicalTopologyProperty) & PropertyStandardQuery
// uses data structure STORAGE_PROPERTY_QUERY.
//

//
// Out parameters for StorageAdapterPhysicalTopologyProperty (or StorageDevicePhysicalTopologyProperty) & PropertyStandardQuery
// uses data structure STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR
//

//
// Multiple roles are allowed for a single device.
//
#define STORAGE_COMPONENT_ROLE_CACHE        0x00000001
#define STORAGE_COMPONENT_ROLE_TIERING      0x00000002
#define STORAGE_COMPONENT_ROLE_DATA         0x00000004


typedef enum _STORAGE_DEVICE_FORM_FACTOR {
    FormFactorUnknown = 0,

    FormFactor3_5,          // 3.5 inch nominal form factor
    FormFactor2_5,          // 2.5 inch nominal form factor
    FormFactor1_8,          // 1.8 inch nominal form factor
    FormFactor1_8Less,      // Less than 1.8 inch nominal form factor

    FormFactorEmbedded,     // Embedded on board.
    FormFactorMemoryCard,   // Memory card such as SD, CF.
    FormFactormSata,        // mSATA
    FormFactorM_2,          // M.2
    FormFactorPCIeBoard,    // PCIe card plug into slot.
    FormFactorDimm,         // DIMM Slot

} STORAGE_DEVICE_FORM_FACTOR, *PSTORAGE_DEVICE_FORM_FACTOR;


typedef enum _STORAGE_COMPONENT_HEALTH_STATUS {
    HealthStatusUnknown = 0,
    HealthStatusNormal,
    HealthStatusThrottled,
    HealthStatusWarning,
    HealthStatusDisabled,
    HealthStatusFailed,
} STORAGE_COMPONENT_HEALTH_STATUS, *PSTORAGE_COMPONENT_HEALTH_STATUS;

#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/unions

typedef union _STORAGE_SPEC_VERSION {

    struct {
        union {
            struct {
                UCHAR   SubMinor;
                UCHAR   Minor;
            } DUMMYSTRUCTNAME;

            USHORT  AsUshort;

        } MinorVersion;

        USHORT  MajorVersion;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} STORAGE_SPEC_VERSION, *PSTORAGE_SPEC_VERSION;

#pragma warning(pop)


typedef struct _STORAGE_PHYSICAL_DEVICE_DATA {

    ULONG       DeviceId;
    ULONG       Role;                                   // Value(s) of bitmask from STORAGE_COMPONENT_ROLE_xxx

    STORAGE_COMPONENT_HEALTH_STATUS HealthStatus;
    STORAGE_PROTOCOL_TYPE           CommandProtocol;
    STORAGE_SPEC_VERSION            SpecVersion;        // Supported storage spec version. For example: SBC 3, SATA 3.2, NVMe 1.2
    STORAGE_DEVICE_FORM_FACTOR      FormFactor;

    UCHAR       Vendor[8];
    UCHAR       Model[40];
    UCHAR       FirmwareRevision[16];

    ULONGLONG   Capacity;                               // in unit of Kilo-Bytes (1024 bytes).

    UCHAR       PhysicalLocation[32];                   // Reserved for future.

    ULONG       Reserved[2];

} STORAGE_PHYSICAL_DEVICE_DATA, *PSTORAGE_PHYSICAL_DEVICE_DATA;


typedef struct _STORAGE_PHYSICAL_ADAPTER_DATA {

    ULONG       AdapterId;
    STORAGE_COMPONENT_HEALTH_STATUS HealthStatus;
    STORAGE_PROTOCOL_TYPE           CommandProtocol;
    STORAGE_SPEC_VERSION            SpecVersion;        // Supported storage spec version. For example: AHCI 1.3.1

    UCHAR       Vendor[8];
    UCHAR       Model[40];
    UCHAR       FirmwareRevision[16];

    UCHAR       PhysicalLocation[32];   // Reserve for future.

    BOOLEAN     ExpanderConnected;
    UCHAR       Reserved0[3];
    ULONG       Reserved1[3];

} STORAGE_PHYSICAL_ADAPTER_DATA, *PSTORAGE_PHYSICAL_ADAPTER_DATA;


typedef struct _STORAGE_PHYSICAL_NODE_DATA {

    ULONG       NodeId;

    ULONG       AdapterCount;           // 0 or 1
    ULONG       AdapterDataLength;
    ULONG       AdapterDataOffset;      // Offset from beginning of this data structure. The buffer contains an array of STORAGE_PHYSICAL_ADAPTER_DATA.

    ULONG       DeviceCount;            // >= 1
    ULONG       DeviceDataLength;
    ULONG       DeviceDataOffset;       // Offset from beginning of this data structure. The buffer contains an array of STORAGE_PHYSICAL_DEVICE_DATA.

    ULONG       Reserved[3];

} STORAGE_PHYSICAL_NODE_DATA, *PSTORAGE_PHYSICAL_NODE_DATA;


typedef struct _STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {

    ULONG       Version;            // sizeof(STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR)
    ULONG       Size;               // Total size of the data. Should be >= sizeof(STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR)

    ULONG       NodeCount;
    ULONG       Reserved;

    STORAGE_PHYSICAL_NODE_DATA Node[ANYSIZE_ARRAY];

} STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR, *PSTORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR;


//
// Output buffer for StorageDeviceIoCapabilityProperty & PropertyStandardQuery
//

typedef _Struct_size_bytes_(Size) struct _STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR {

    //
    // Size of this structure serves
    // as the version
    //

    ULONG Version;

    //
    // Size of this structure
    //

    ULONG Size;

    //
    // LUN max outstanding IO count
    //

    ULONG LunMaxIoCount;

    //
    // Adapter max outstanding IO count
    //

    ULONG AdapterMaxIoCount;

} STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR, *PSTORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR;

//
// Output buffer for StorageDeviceAttributesProperty & PropertyStandardQuery
//

typedef _Struct_size_bytes_(Size) struct _STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {

    //
    // Size of this structure serves
    // as the version
    //

    ULONG Version;

    //
    // Size of this structure
    //

    ULONG Size;

    //
    // Attributes (bit flags)
    //

    ULONG64 Attributes;

} STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR, *PSTORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR;

//
// Storage Device Attributes Flags
//

#define STORAGE_ATTRIBUTE_BYTE_ADDRESSABLE_IO       0x01
#define STORAGE_ATTRIBUTE_BLOCK_IO                  0x02
#define STORAGE_ATTRIBUTE_DYNAMIC_PERSISTENCE       0x04
#define STORAGE_ATTRIBUTE_VOLATILE                  0x08
#define STORAGE_ATTRIBUTE_ASYNC_EVENT_NOTIFICATION  0x10
#define STORAGE_ATTRIBUTE_PERF_SIZE_INDEPENDENT     0x20

//
// Constants for StorageDeviceManagementStatus
//

typedef enum _STORAGE_DISK_HEALTH_STATUS {
    DiskHealthUnknown = 0,
    DiskHealthUnhealthy,
    DiskHealthWarning,
    DiskHealthHealthy,
    DiskHealthMax
} STORAGE_DISK_HEALTH_STATUS, *PSTORAGE_DISK_HEALTH_STATUS;

//
// Operational States
//
typedef enum _STORAGE_DISK_OPERATIONAL_STATUS {
    DiskOpStatusNone = 0,
    DiskOpStatusUnknown,
    DiskOpStatusOk,
    DiskOpStatusPredictingFailure,
    DiskOpStatusInService,
    DiskOpStatusHardwareError,
    DiskOpStatusNotUsable,
    DiskOpStatusTransientError,
    DiskOpStatusMissing,
} STORAGE_DISK_OPERATIONAL_STATUS, *PSTORAGE_DISK_OPERATIONAL_STATUS;

//
// Operational Reasons
//
typedef enum _STORAGE_OPERATIONAL_STATUS_REASON {
    DiskOpReasonUnknown = 0,
    DiskOpReasonScsiSenseCode,
    DiskOpReasonMedia,
    DiskOpReasonIo,
    DiskOpReasonThresholdExceeded,
    DiskOpReasonLostData,
    DiskOpReasonEnergySource,
    DiskOpReasonConfiguration,
    DiskOpReasonDeviceController,
    DiskOpReasonMediaController,
    DiskOpReasonComponent,
    DiskOpReasonNVDIMM_N,
    DiskOpReasonBackgroundOperation,
    DiskOpReasonInvalidFirmware,
    DiskOpReasonHealthCheck,
    DiskOpReasonLostDataPersistence,
    DiskOpReasonDisabledByPlatform,
    DiskOpReasonLostWritePersistence,
    DiskOpReasonDataPersistenceLossImminent,
    DiskOpReasonWritePersistenceLossImminent,
    DiskOpReasonMax
} STORAGE_OPERATIONAL_STATUS_REASON, *PSTORAGE_OPERATIONAL_STATUS_REASON;

typedef struct _STORAGE_OPERATIONAL_REASON {
    ULONG Version;
    ULONG Size;
    STORAGE_OPERATIONAL_STATUS_REASON Reason;

    union {

        //
        // This is the format if Reason == DiskOpReasonScsiSenseCode.
        //
        struct {
            UCHAR SenseKey;
            UCHAR ASC;
            UCHAR ASCQ;
            UCHAR Reserved;
        } ScsiSenseKey;

        //
        // This is the format if Reason == DiskOpReasonNVDIMM_N.
        //
        struct {
            UCHAR CriticalHealth;
            UCHAR ModuleHealth[2];
            UCHAR ErrorThresholdStatus;
        } NVDIMM_N;

        ULONG AsUlong;
    } RawBytes;
} STORAGE_OPERATIONAL_REASON, *PSTORAGE_OPERATIONAL_REASON;

//
// Output buffer for StorageDeviceManagementStatus & PropertyStandardQuery
//

#define STORAGE_DEVICE_MAX_OPERATIONAL_STATUS    16

typedef struct _STORAGE_DEVICE_MANAGEMENT_STATUS {

    //
    // Sizeof() of this structure serves
    // as the version.
    //

    ULONG Version;

    //
    // The total size of the structure, including operational status reasons
    // that didn't fit in the caller's array. Callers should use this field to learn
    // how big the input buffer should be to contain all the available information.
    //

    ULONG Size;

    //
    // Health status.
    //

    STORAGE_DISK_HEALTH_STATUS Health;

    //
    // The number of operational status returned.
    //

    ULONG NumberOfOperationalStatus;

    //
    // The number of additional reasons returned.
    //

    ULONG NumberOfAdditionalReasons;

    //
    // Operational statuses. The primary operational status is the first element
    // in the array. There are NumberOfOperationalStatus valid elements in the array.
    //

    STORAGE_DISK_OPERATIONAL_STATUS OperationalStatus[STORAGE_DEVICE_MAX_OPERATIONAL_STATUS];

    //
    // Additional reasons. There are NumberOfAdditionalReasons valid elements in the array.
    //

    STORAGE_OPERATIONAL_REASON AdditionalReasons[ANYSIZE_ARRAY];

} STORAGE_DEVICE_MANAGEMENT_STATUS, *PSTORAGE_DEVICE_MANAGEMENT_STATUS;

//
// Parameter for StorageAdapterSerialNumberProperty.
//
// Use this to get the serial number of the storage adapter.  Note that not all
// controllers and host controller interfaces may provide a serial number for
// the adapter.  If the serial number is malformed or cannot be obtained this
// query will fail.
//
// The serial number can have a maximum of 128 Unicode characters, including
// the trailing NULL character.
//

#define STORAGE_ADAPTER_SERIAL_NUMBER_V1_MAX_LENGTH (128)

typedef _Struct_size_bytes_(Size) struct _STORAGE_ADAPTER_SERIAL_NUMBER {

    ULONG Version;

    ULONG Size;

    //
    // NULL-terminated Unicode string of the adapter's serial number.
    //

    WCHAR SerialNumber[STORAGE_ADAPTER_SERIAL_NUMBER_V1_MAX_LENGTH];

} STORAGE_ADAPTER_SERIAL_NUMBER, *PSTORAGE_ADAPTER_SERIAL_NUMBER;

#define STORAGE_ADAPTER_SERIAL_NUMBER_V1_VERSION (sizeof(STORAGE_ADAPTER_SERIAL_NUMBER))
#define STORAGE_ADAPTER_SERIAL_NUMBER_V1_SIZE (sizeof(STORAGE_ADAPTER_SERIAL_NUMBER))

//
// Output buffer for StorageDeviceZonedDeviceProperty & PropertyStandardQuery
//

typedef enum _STORAGE_ZONED_DEVICE_TYPES {
    ZonedDeviceTypeUnknown = 0,
    ZonedDeviceTypeHostManaged,
    ZonedDeviceTypeHostAware,
    ZonedDeviceTypeDeviceManaged,
} STORAGE_ZONED_DEVICE_TYPES, *PSTORAGE_ZONED_DEVICE_TYPES;

typedef enum _STORAGE_ZONE_TYPES {
    ZoneTypeUnknown = 0,
    ZoneTypeConventional = 1,
    ZoneTypeSequentialWriteRequired = 2,
    ZoneTypeSequentialWritePreferred = 3,
    ZoneTypeMax
} STORAGE_ZONE_TYPES, *PSTORAGE_ZONE_TYPES;

typedef struct _STORAGE_ZONE_GROUP {

    ULONG ZoneCount;                // Count of zones in this group.

    STORAGE_ZONE_TYPES ZoneType;

    ULONGLONG ZoneSize;             // In Bytes

} STORAGE_ZONE_GROUP, *PSTORAGE_ZONE_GROUP;

typedef _Struct_size_bytes_(Size) struct _STORAGE_ZONED_DEVICE_DESCRIPTOR {

    //
    // Size of this structure serves as the version
    //

    ULONG Version;

    //
    // Size of buffer. The returned value indicates how big the buffer should be
    // to store complete data.
    //

    ULONG Size;

    //
    // Zoned device type
    //

    STORAGE_ZONED_DEVICE_TYPES DeviceType;

    //
    // Total zone count
    //

    ULONG ZoneCount;

    //
    // Zone Attributes
    //

    union {
        struct {

            ULONG   MaxOpenZoneCount;

            BOOLEAN UnrestrictedRead;

            UCHAR   Reserved[3];

        }  SequentialRequiredZone;      // Host managed device only

        struct {

            ULONG   OptimalOpenZoneCount;

            ULONG   Reserved;

        }  SequentialPreferredZone;     // Host aware device only

    } ZoneAttributes;

    //
    // Zone Layout Information, to provide a picture about locations of different type of zones on disk.
    // The zone layout starts from the first zone, and groups together zones with same type and size.
    //

    ULONG ZoneGroupCount;

    STORAGE_ZONE_GROUP ZoneGroup[ANYSIZE_ARRAY];

} STORAGE_ZONED_DEVICE_DESCRIPTOR, *PSTORAGE_ZONED_DEVICE_DESCRIPTOR;


//
// Output buffer for StorageDeviceLocationProperty & PropertyStandardQuery
//

#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/unions
typedef struct _DEVICE_LOCATION {

    ULONG Socket;

    ULONG Slot;

    ULONG Adapter;

    ULONG Port;

    union {

        struct {

            ULONG Channel;

            ULONG Device;

        } DUMMYSTRUCTNAME;

        struct {

            ULONG Target;

            ULONG Lun;

        } DUMMYSTRUCTNAME2;

    } DUMMYUNIONNAME;

} DEVICE_LOCATION, *PDEVICE_LOCATION;
#pragma warning(pop)

typedef _Struct_size_bytes_(Size) struct _STORAGE_DEVICE_LOCATION_DESCRIPTOR {

    ULONG Version;

    ULONG Size;

    DEVICE_LOCATION Location;

    ULONG StringOffset;

} STORAGE_DEVICE_LOCATION_DESCRIPTOR, *PSTORAGE_DEVICE_LOCATION_DESCRIPTOR;

//
// Output buffer for StorageDeviceNumaProperty.
//
// If the query for this property is successful, then the caller should
// validate the NumaNode field before using it to optimize any operations.
// That is, the caller should ensure the NumaNode value is less than or equal
// to the system's highest NUMA node value and the NumaNode value is not equal
// to STORAGE_DEVICE_NUMA_NODE_UNKNOWN.
//
typedef struct _STORAGE_DEVICE_NUMA_PROPERTY {
    ULONG Version;
    ULONG Size;
    ULONG NumaNode;
} STORAGE_DEVICE_NUMA_PROPERTY, *PSTORAGE_DEVICE_NUMA_PROPERTY;

#define STORAGE_DEVICE_NUMA_NODE_UNKNOWN MAXULONG

//
// Output buffer for StorageDeviceUnsafeShutdownCount.
//
// On persistent memory devices, the unsafe shutdown count is the number of times
// the logical persistent memory disk was shut down in a way that might have caused
// data loss.
//
typedef struct _STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT {
    ULONG Version;
    ULONG Size;
    ULONG UnsafeShutdownCount;
} STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT, *PSTORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT;

#pragma warning(push)
#pragma warning(disable:4214)   // bit fields other than int to disable this around the struct
#pragma warning(disable:4201)   // nameless struct/union

//
// Parameters for StorageDeviceEnduranceProperty & PropertyStandardQuery
//

//
// Input parameters for StorageDeviceEnduranceProperty & PropertyStandardQuery
// uses data structure STORAGE_PROPERTY_QUERY.
//

//
// Out parameters for StorageDeviceEnduranceProperty  & PropertyStandardQuery
// For endurance info fields, ValidFields represents bit mapping of valid fields.
//

typedef struct _STORAGE_HW_ENDURANCE_INFO {
    ULONG       ValidFields;        // ValidFields represents bit mapping of valid fields of any type
                                    // Eg: Bit 0 stands for GroupId, Bit 1 stands for Flags, Bit 3 for BytesReadCount

    ULONG       GroupId;            // Set Id Eg: Set Id for NVMe sets

    struct {
        ULONG   Shared:1;           // TRUE if information is shared with multiple units/groups

        ULONG   Reserved:31;
    } Flags;

    ULONG       LifePercentage;         // Used life percentage

    UCHAR       BytesReadCount[16];     // Total bytes read from device (Billion Unit)

    UCHAR       ByteWriteCount[16];     // Total bytes written to device (Billion Unit)

} STORAGE_HW_ENDURANCE_INFO, *PSTORAGE_HW_ENDURANCE_INFO;

typedef struct _STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR {
    ULONG                           Version;            // keep compatible with STORAGE_DESCRIPTOR_HEADER

    ULONG                           Size;               // keep compatible with STORAGE_DESCRIPTOR_HEADER

    STORAGE_HW_ENDURANCE_INFO       EnduranceInfo;      // Endurance Information of the device

} STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR, *PSTORAGE_HW_ENDURANCE_DATA_DESCRIPTOR;

//
// Output buffer for StorageStackProperty.
//

typedef enum _STORAGE_STACK_TYPE {
    StorageStackTypeUnknown = 0,
    StorageStackTypeScsi,
    StorageStackTypeNVMe,
} STORAGE_STACK_TYPE, *PSTORAGE_STACK_TYPE;

typedef _Struct_size_bytes_(Size) struct _STORAGE_STACK_DESCRIPTOR {

    //
    // Size of this structure serves as the version
    //

    ULONG Version;

    //
    // Size of buffer. The returned value indicates how big the buffer should be
    // to store complete data.
    //

    ULONG Size;

    //
    // Type of storage stack for the device.
    //

    STORAGE_STACK_TYPE StorageStackType;

} STORAGE_STACK_DESCRIPTOR, *PSTORAGE_STACK_DESCRIPTOR;

#pragma warning(pop)

//
// Output buffer for StorageDeviceLedStateProperty.
//
typedef struct _STORAGE_DEVICE_LED_STATE_DESCRIPTOR {

    ULONG Version;

    ULONG Size;

    ULONGLONG State;

} STORAGE_DEVICE_LED_STATE_DESCRIPTOR, *PSTORAGE_DEVICE_LED_STATE_DESCRIPTOR;

//
// Output buffer for StorageDeviceSelfEncryptionProperty.
//

// Version 1 of the structure
// Note: This version only checks if the storage device
//       is eDrive capable to keep it backward compatible.
//       To handle both eDrive and native TCG implementations
//       use Version 2 of the structure.
typedef struct _STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY {

    ULONG Version; // Sizeof(STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY)

    ULONG Size;

    BOOLEAN SupportsSelfEncryption;

} STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY, *PSTORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY;


typedef enum _STORAGE_ENCRYPTION_TYPE {

    StorageEncryptionTypeUnknown  = 0x00,
    StorageEncryptionTypeEDrive   = 0x01,
    StorageEncryptionTypeTcgOpal  = 0x02,

} STORAGE_ENCRYPTION_TYPE, *PSTORAGE_ENCRYPTION_TYPE;

// Version 2 of the structure
// Note: This version handles both eDrive and native TCG implementations.
//       To check if a storage device supports a native TCG implementation,
//       a client must use this version of the structure.
typedef struct _STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY_V2 {

    ULONG Version; // Sizeof(STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY_V2)

    ULONG Size;

    BOOLEAN SupportsSelfEncryption;

    STORAGE_ENCRYPTION_TYPE EncryptionType;

} STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY_V2, *PSTORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY_V2;

//
// Output buffer for StorageFruIdProperty.
//
typedef struct _STORAGE_FRU_ID_DESCRIPTOR {

    //
    // Sizeof(STORAGE_FRU_ID_DESCRIPTOR)
    //

    ULONG Version;

    //
    // Total size of the data.
    // Should be >= sizeof(STORAGE_FRU_ID_DESCRIPTOR)
    //

    ULONG Size;

    //
    // The identifier is a variable length array of bytes.
    //

    ULONG IdentifierSize;
    UCHAR Identifier[ANYSIZE_ARRAY];

} STORAGE_FRU_ID_DESCRIPTOR, *PSTORAGE_FRU_ID_DESCRIPTOR;


////////////////////////////////////////////////////////////////////////////////
//
// IOCTL_STORAGE_MANAGE_DATA_SET_ATTRIBUTES
//
// Input Buffer:
//     Structure of type DEVICE_DSM_INPUT
//
// Output Buffer:
//     Structure of type DEVICE_DSM_OUTPUT
//

//
// DEVICE_DSM_INPUT.Action
//

typedef ULONG DEVICE_DATA_MANAGEMENT_SET_ACTION, DEVICE_DSM_ACTION;

//
// This indicates that the action is
// non-destructive and a driver that
// does not understand it may safely
// forward the IOCTL
//

#define DeviceDsmActionFlag_NonDestructive      (0x80000000)
#define IsDsmActionNonDestructive(_Action)      ((BOOLEAN)((_Action & DeviceDsmActionFlag_NonDestructive) != 0))

#define DeviceDsmAction_None                    (0x00000000u)
#define DeviceDsmAction_Trim                    (0x00000001u)
#define DeviceDsmAction_Notification            (0x00000002u | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_OffloadRead             (0x00000003u | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_OffloadWrite            (0x00000004u)
#define DeviceDsmAction_Allocation              (0x00000005u | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_Repair                  (0x00000006u | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_Scrub                   (0x00000007u | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_DrtQuery                (0x00000008u | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_DrtClear                (0x00000009u | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_DrtDisable              (0x0000000Au | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_TieringQuery            (0x0000000Bu | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_Map                     (0x0000000Cu | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_RegenerateParity        (0x0000000Du | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_NvCache_Change_Priority (0x0000000Eu | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_NvCache_Evict           (0x0000000Fu | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_TopologyIdQuery         (0x00000010u | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_GetPhysicalAddresses    (0x00000011u | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_ScopeRegen              (0x00000012u | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_ReportZones             (0x00000013u | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_OpenZone                (0x00000014u | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_FinishZone              (0x00000015u | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_CloseZone               (0x00000016u | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_ResetWritePointer       (0x00000017u)
#define DeviceDsmAction_GetRangeErrorInfo       (0x00000018u | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_WriteZeroes             (0x00000019u)
#define DeviceDsmAction_LostQuery               (0x0000001Au | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_GetFreeSpace            (0x0000001Bu | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_ConversionQuery         (0x0000001Cu | DeviceDsmActionFlag_NonDestructive)
#define DeviceDsmAction_VdtSet                  (0x0000001Du)
#define DeviceDsmAction_QueryPreferLocalRepair  (0x0000001Eu | DeviceDsmActionFlag_NonDestructive)

//
// DEVICE_DSM_INPUT.Flags
//
// Flags that are not specific to an
// action are in the lower 16-bits
//
// Action-specific flags  are in the
// higher 16-bits
//

//
// When specified, the DataSetRanges
// fields should be 0
//

#define DEVICE_DSM_FLAG_ENTIRE_DATA_SET_RANGE   0x00000001

typedef struct _DEVICE_DATA_SET_RANGE {

    //
    // Must be a  multiple of sector
    // size, in bytes
    //

    LONGLONG StartingOffset;
    ULONGLONG LengthInBytes;

} DEVICE_DATA_SET_RANGE, *PDEVICE_DATA_SET_RANGE,
  DEVICE_DSM_RANGE, *PDEVICE_DSM_RANGE;

typedef struct _DEVICE_MANAGE_DATA_SET_ATTRIBUTES {

    //
    // Size of this structure serves
    // as the version
    //

    ULONG Size;

    DEVICE_DSM_ACTION Action;
    ULONG Flags;

    //
    // Must be aligned to __alignof(action-specific struct)
    //

    ULONG ParameterBlockOffset;
    ULONG ParameterBlockLength;

    //
    // Must be aligned to __alignof(DEVICE_DSM_RANGE)
    //

    ULONG DataSetRangesOffset;
    ULONG DataSetRangesLength;

} DEVICE_MANAGE_DATA_SET_ATTRIBUTES, *PDEVICE_MANAGE_DATA_SET_ATTRIBUTES,
  DEVICE_DSM_INPUT, *PDEVICE_DSM_INPUT;

typedef struct _DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {

    //
    // Size of this structure serves
    // as the version
    //

    ULONG Size;

    DEVICE_DSM_ACTION Action;
    ULONG Flags;

    ULONG OperationStatus;
    ULONG ExtendedError;
    ULONG TargetDetailedError;
    ULONG ReservedStatus;

    //
    // Must be aligned to __alignof(corresponding struct)
    //

    ULONG OutputBlockOffset;
    ULONG OutputBlockLength;

} DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT, *PDEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT,
  DEVICE_DSM_OUTPUT, *PDEVICE_DSM_OUTPUT;

typedef struct _DEVICE_DSM_DEFINITION {

    DEVICE_DSM_ACTION Action;

    BOOLEAN SingleRange;

    ULONG ParameterBlockAlignment;
    ULONG ParameterBlockLength;

    BOOLEAN HasOutput;

    ULONG OutputBlockAlignment;
    ULONG OutputBlockLength;

} DEVICE_DSM_DEFINITION, *PDEVICE_DSM_DEFINITION;


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_None
//

//
// SingleRange    - No
// ParameterBlock - No
// Output         - No
// OutputBlock    - No
//

#define DeviceDsmDefinition_None {DeviceDsmAction_None, \
                                  FALSE,                \
                                  0,                    \
                                  0,                    \
                                  FALSE,                \
                                  0,                    \
                                  0}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_Trim
//

//
// DEVICE_DSM_INPUT.Flags
//

//
// Indicates that the ranges are not
// part of any file
//

#define DEVICE_DSM_FLAG_TRIM_NOT_FS_ALLOCATED   0x80000000

//
// Indicates that RZAT is not needed
// RZAT only applies to ranges  that
// are part of a file that  need the
// additional protection
//

#define DEVICE_DSM_FLAG_TRIM_BYPASS_RZAT        0x40000000

//
// SingleRange    - No
// ParameterBlock - No
// Output         - No
// OutputBlock    - No
//

#define DeviceDsmDefinition_Trim {DeviceDsmAction_Trim, \
                                  FALSE,                \
                                  0,                    \
                                  0,                    \
                                  FALSE,                \
                                  0,                    \
                                  0}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_Notification
//

//
// DEVICE_DSM_NOTIFICATION_PARAMETERS.Flags
//

//
// The  ranges are now in use by the
// file type identifier
//

#define DEVICE_DSM_NOTIFY_FLAG_BEGIN    0x00000001

//
// The  ranges are no longer  in use
// by the file type identifier
//

#define DEVICE_DSM_NOTIFY_FLAG_END      0x00000002

typedef struct _DEVICE_DSM_NOTIFICATION_PARAMETERS {

    ULONG Size;

    ULONG Flags;

    ULONG NumFileTypeIDs;
    GUID  FileTypeID[ANYSIZE_ARRAY];

} DEVICE_DSM_NOTIFICATION_PARAMETERS, *PDEVICE_DSM_NOTIFICATION_PARAMETERS;

//
// SingleRange    - No
// ParameterBlock - Yes
// Output         - No
// OutputBlock    - No
//

#define DeviceDsmDefinition_Notification {DeviceDsmAction_Notification,                  \
                                          FALSE,                                         \
                                          __alignof(DEVICE_DSM_NOTIFICATION_PARAMETERS), \
                                          sizeof(DEVICE_DSM_NOTIFICATION_PARAMETERS),    \
                                          FALSE,                                         \
                                          0,                                             \
                                          0}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_OffloadRead
//

#define STORAGE_OFFLOAD_MAX_TOKEN_LENGTH        512        // Keep as ULONG multiple
#define STORAGE_OFFLOAD_TOKEN_ID_LENGTH         0x1F8
#define STORAGE_OFFLOAD_TOKEN_TYPE_ZERO_DATA    0xFFFF0001

#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/unions

typedef struct _STORAGE_OFFLOAD_TOKEN {

    UCHAR TokenType[4];
    UCHAR Reserved[2];
    UCHAR TokenIdLength[2];
    union {
        struct {
            UCHAR Reserved2[STORAGE_OFFLOAD_TOKEN_ID_LENGTH];
        } StorageOffloadZeroDataToken;
        UCHAR Token[STORAGE_OFFLOAD_TOKEN_ID_LENGTH];
    } DUMMYUNIONNAME;

} STORAGE_OFFLOAD_TOKEN, *PSTORAGE_OFFLOAD_TOKEN;

#pragma warning(pop)

#define MAKE_ZERO_TOKEN(T) (                                  \
    ((PSTORAGE_OFFLOAD_TOKEN)T)->TokenType[0] = 0xFF,         \
    ((PSTORAGE_OFFLOAD_TOKEN)T)->TokenType[1] = 0xFF,         \
    ((PSTORAGE_OFFLOAD_TOKEN)T)->TokenType[2] = 0x00,         \
    ((PSTORAGE_OFFLOAD_TOKEN)T)->TokenType[3] = 0x01,         \
    ((PSTORAGE_OFFLOAD_TOKEN)T)->TokenIdLength[0] = 0x01,     \
    ((PSTORAGE_OFFLOAD_TOKEN)T)->TokenIdLength[1] = 0xF8      \
)

#define IS_ZERO_TOKEN(T) (                                    \
    (((PSTORAGE_OFFLOAD_TOKEN)T)->TokenType[0] == 0xFF     && \
     ((PSTORAGE_OFFLOAD_TOKEN)T)->TokenType[1] == 0xFF     && \
     ((PSTORAGE_OFFLOAD_TOKEN)T)->TokenType[2] == 0x00     && \
     ((PSTORAGE_OFFLOAD_TOKEN)T)->TokenType[3] == 0x01     && \
     ((PSTORAGE_OFFLOAD_TOKEN)T)->TokenIdLength[0] == 0x01 && \
     ((PSTORAGE_OFFLOAD_TOKEN)T)->TokenIdLength[1] == 0xF8)   \
)

typedef struct _DEVICE_DSM_OFFLOAD_READ_PARAMETERS {

    //
    // Reserved for future use
    //

    ULONG Flags;

    //
    // Token TTL in milli-seconds as
    // requested by the initiator
    //

    ULONG TimeToLive;

    ULONG Reserved[2];

} DEVICE_DSM_OFFLOAD_READ_PARAMETERS, *PDEVICE_DSM_OFFLOAD_READ_PARAMETERS;

//
// STORAGE_OFFLOAD_READ_OUTPUT.OffloadReadFlags
//

#define STORAGE_OFFLOAD_READ_RANGE_TRUNCATED    0x00000001

//
// The token  returned by the target
// uniquely  identifies  a "point in
// time" snapshot of ranges taken by
// the target.  Its format is opaque
//
// We  arbitrarily limit token length to 512. The SCSI interface will/may enable
// negotiable size. If we want to add support, we'll need to create a new action
//

typedef struct _STORAGE_OFFLOAD_READ_OUTPUT {

    ULONG OffloadReadFlags;
    ULONG Reserved;

    //
    // Length of the "snapshot" that
    // is bound to  the token.  Must
    // be from the lowest range
    //

    ULONGLONG LengthProtected;

    ULONG TokenLength;
    STORAGE_OFFLOAD_TOKEN Token;

} STORAGE_OFFLOAD_READ_OUTPUT, *PSTORAGE_OFFLOAD_READ_OUTPUT;

//
// SingleRange    - No
// ParameterBlock - Yes
// Output         - No
// OutputBlock    - Yes
//

#define DeviceDsmDefinition_OffloadRead {DeviceDsmAction_OffloadRead,                   \
                                         FALSE,                                         \
                                         __alignof(DEVICE_DSM_OFFLOAD_READ_PARAMETERS), \
                                         sizeof(DEVICE_DSM_OFFLOAD_READ_PARAMETERS),    \
                                         FALSE,                                         \
                                         __alignof(STORAGE_OFFLOAD_READ_OUTPUT),        \
                                         sizeof(STORAGE_OFFLOAD_READ_OUTPUT)}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_OffloadWrite
//

typedef struct _DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS {

    //
    // Reserved for future use
    //

    ULONG Flags;
    ULONG Reserved;

    //
    // Starting  offset to copy from
    // "snapshot" bound to the token
    //

    ULONGLONG TokenOffset;

    STORAGE_OFFLOAD_TOKEN Token;

} DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS, *PDEVICE_DSM_OFFLOAD_WRITE_PARAMETERS;

//
// STORAGE_OFFLOAD_WRITE_OUTPUT.OffloadWriteFlags
//

#define STORAGE_OFFLOAD_WRITE_RANGE_TRUNCATED   0x0001
#define STORAGE_OFFLOAD_TOKEN_INVALID           0x0002

typedef struct _STORAGE_OFFLOAD_WRITE_OUTPUT {

    ULONG OffloadWriteFlags;
    ULONG Reserved;

    //
    // Length of content copied from
    // the "snapshot" from the start
    //

    ULONGLONG LengthCopied;

} STORAGE_OFFLOAD_WRITE_OUTPUT, *PSTORAGE_OFFLOAD_WRITE_OUTPUT;

//
// SingleRange    - No
// ParameterBlock - Yes
// Output         - No
// OutputBlock    - Yes
//

#define DeviceDsmDefinition_OffloadWrite {DeviceDsmAction_OffloadWrite,                   \
                                          FALSE,                                          \
                                          __alignof(DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS), \
                                          sizeof(DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS),    \
                                          FALSE,                                          \
                                          __alignof(STORAGE_OFFLOAD_WRITE_OUTPUT),        \
                                          sizeof(STORAGE_OFFLOAD_WRITE_OUTPUT)}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_Allocation
//

//
// DEVICE_DSM_INPUT.Flags
//

#define DEVICE_DSM_FLAG_ALLOCATION_CONSOLIDATEABLE_ONLY 0x40000000

typedef struct _DEVICE_DATA_SET_LBP_STATE_PARAMETERS {

    ULONG Version;
    ULONG Size;

    //
    // Reserved for future use
    //

    ULONG Flags;

    //
    // DEVICE_DSM_ALLOCATION_OUTPUT_V1 or
    // DEVICE_DSM_ALLOCATION_OUTPUT_V2
    //

    ULONG OutputVersion;

} DEVICE_DATA_SET_LBP_STATE_PARAMETERS, *PDEVICE_DATA_SET_LBP_STATE_PARAMETERS,
  DEVICE_DSM_ALLOCATION_PARAMETERS, *PDEVICE_DSM_ALLOCATION_PARAMETERS;

#define DEVICE_DSM_PARAMETERS_V1                        1
#define DEVICE_DATA_SET_LBP_STATE_PARAMETERS_VERSION_V1 DEVICE_DSM_PARAMETERS_V1

typedef struct _DEVICE_DATA_SET_LB_PROVISIONING_STATE {

    ULONG Size;
    ULONG Version;

    ULONGLONG SlabSizeInBytes;

    //
    // Delta  from the  start offset
    // if the requested range is not
    // aligned to a slab boundary
    //

    ULONG SlabOffsetDeltaInBytes;

    //
    // Number of bits that are valid
    //

    ULONG SlabAllocationBitMapBitCount;

    //
    // Count of ULONGs in the bitmap
    //

    ULONG SlabAllocationBitMapLength;

    //
    // 1 = mapped, 0 = unmapped
    //

    ULONG SlabAllocationBitMap[ANYSIZE_ARRAY];

} DEVICE_DATA_SET_LB_PROVISIONING_STATE, *PDEVICE_DATA_SET_LB_PROVISIONING_STATE,
  DEVICE_DSM_ALLOCATION_OUTPUT, *PDEVICE_DSM_ALLOCATION_OUTPUT;

#define DEVICE_DSM_ALLOCATION_OUTPUT_V1                  (sizeof(DEVICE_DSM_ALLOCATION_OUTPUT))
#define DEVICE_DATA_SET_LB_PROVISIONING_STATE_VERSION_V1 DEVICE_DSM_ALLOCATION_OUTPUT_V1

typedef struct _DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {

    ULONG Size;
    ULONG Version;

    ULONGLONG SlabSizeInBytes;

    //
    // Delta  from the  start offset
    // if the requested range is not
    // aligned to a slab boundary
    //

    ULONGLONG SlabOffsetDeltaInBytes;

    //
    // Number of bits that are valid
    //

    ULONG SlabAllocationBitMapBitCount;

    //
    // Count of ULONGs in the bitmap
    //

    ULONG SlabAllocationBitMapLength;

    //
    // 1 = mapped, 0 = unmapped
    //

    ULONG SlabAllocationBitMap[ANYSIZE_ARRAY];

} DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2, *PDEVICE_DATA_SET_LB_PROVISIONING_STATE_V2,
  DEVICE_DSM_ALLOCATION_OUTPUT2, *PDEVICE_DSM_ALLOCATION_OUTPUT2;

#define DEVICE_DSM_ALLOCATION_OUTPUT_V2                  (sizeof(DEVICE_DSM_ALLOCATION_OUTPUT2))
#define DEVICE_DATA_SET_LB_PROVISIONING_STATE_VERSION_V2 DEVICE_DSM_ALLOCATION_OUTPUT_V2

//
// SingleRange    - Yes
// ParameterBlock - Yes
// Output         - Yes
// OutputBlock    - Yes
//

#define DeviceDsmDefinition_Allocation {DeviceDsmAction_Allocation,                  \
                                        TRUE,                                        \
                                        __alignof(DEVICE_DSM_ALLOCATION_PARAMETERS), \
                                        sizeof(DEVICE_DSM_ALLOCATION_PARAMETERS),    \
                                        TRUE,                                        \
                                        __alignof(DEVICE_DSM_ALLOCATION_OUTPUT2),    \
                                        sizeof(DEVICE_DSM_ALLOCATION_OUTPUT2)}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_Repair
//

//
// DEVICE_DSM_INPUT.Flags
//

#define DEVICE_DSM_FLAG_REPAIR_INPUT_TOPOLOGY_ID_PRESENT    0x40000000

typedef struct _DEVICE_DATA_SET_REPAIR_PARAMETERS {

    ULONG NumberOfRepairCopies;
    ULONG SourceCopy;
    ULONG RepairCopies[ANYSIZE_ARRAY];

    //
    // Valid iff DEVICE_DSM_FLAG_REPAIR_INPUT_TOPOLOGY_ID_PRESENT is set
    //
    // To access this field, use the
    // below macro
    //

    // UCHAR TopologyId[16];

} DEVICE_DATA_SET_REPAIR_PARAMETERS, *PDEVICE_DATA_SET_REPAIR_PARAMETERS,
  DEVICE_DSM_REPAIR_PARAMETERS, *PDEVICE_DSM_REPAIR_PARAMETERS;

#define GET_REPAIR_TOPOLOGY_ID(R)                                                                  \
    RtlOffsetToPointer(R,                                                                          \
                       ALIGN_UP_BY(FIELD_OFFSET(DEVICE_DATA_SET_REPAIR_PARAMETERS, RepairCopies) + \
                       sizeof(ULONG) * R->NumberOfRepairCopies,                                    \
                       8))

//
// DEVICE_DSM_OUTPUT.Flags
//

#define DEVICE_DSM_FLAG_REPAIR_OUTPUT_PARITY_EXTENT         0x20000000

typedef struct _DEVICE_DATA_SET_REPAIR_OUTPUT {

    //
    // Valid iff DEVICE_DSM_FLAG_REPAIR_OUTPUT_PARITY_EXTENT is set
    //

    DEVICE_DSM_RANGE ParityExtent;

} DEVICE_DATA_SET_REPAIR_OUTPUT, *PDEVICE_DATA_SET_REPAIR_OUTPUT,
  DEVICE_DSM_REPAIR_OUTPUT, *PDEVICE_DSM_REPAIR_OUTPUT;

//
// SingleRange    - Yes
// ParameterBlock - Yes
// Output         - Yes
// OutputBlock    - Yes
//

#define DeviceDsmDefinition_Repair {DeviceDsmAction_Repair,                  \
                                    TRUE,                                    \
                                    __alignof(DEVICE_DSM_REPAIR_PARAMETERS), \
                                    sizeof(DEVICE_DSM_REPAIR_PARAMETERS),    \
                                    TRUE,                                    \
                                    __alignof(DEVICE_DSM_REPAIR_OUTPUT),     \
                                    sizeof(DEVICE_DSM_REPAIR_OUTPUT)}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_QueryPreferLocalRepair
//

typedef struct _DEVICE_DSM_QUERY_PREFER_LOCAL_REPAIR_OUTPUT {

    ULONG Version;
    BOOLEAN PreferLocalRepair;

} DEVICE_DSM_QUERY_PREFER_LOCAL_REPAIR_OUTPUT, *PDEVICE_DSM_QUERY_PREFER_LOCAL_REPAIR_OUTPUT;

//
// SingleRange    - No
// ParameterBlock - No
// Output         - Yes
// OutputBlock    - Yes
//

#define DeviceDsmDefinition_QueryPreferLocalRepair {DeviceDsmAction_QueryPreferLocalRepair, \
                                                    FALSE,                                  \
                                                    0,                                      \
                                                    0,                                      \
                                                    TRUE,                                   \
                                                    __alignof(DEVICE_DSM_QUERY_PREFER_LOCAL_REPAIR_OUTPUT), \
                                                    sizeof(DEVICE_DSM_QUERY_PREFER_LOCAL_REPAIR_OUTPUT)}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_Scrub
//

//
// DEVICE_DSM_INPUT.Flags
//

#define DEVICE_DSM_FLAG_SCRUB_SKIP_IN_SYNC           0x10000000

typedef struct _DEVICE_DATA_SET_SCRUB_OUTPUT {

    ULONGLONG BytesProcessed;
    ULONGLONG BytesRepaired;
    ULONGLONG BytesFailed;

} DEVICE_DATA_SET_SCRUB_OUTPUT, *PDEVICE_DATA_SET_SCRUB_OUTPUT,
  DEVICE_DSM_SCRUB_OUTPUT, *PDEVICE_DSM_SCRUB_OUTPUT;

//
// DEVICE_DSM_OUTPUT.Flags
//

#define DEVICE_DSM_FLAG_SCRUB_OUTPUT_PARITY_EXTENT   0x20000000

typedef struct _DEVICE_DATA_SET_SCRUB_EX_OUTPUT {

    ULONGLONG BytesProcessed;
    ULONGLONG BytesRepaired;
    ULONGLONG BytesFailed;

    //
    // Valid iff DEVICE_DSM_FLAG_SCRUB_OUTPUT_PARITY_EXTENT is set
    //

    DEVICE_DSM_RANGE ParityExtent;

    ULONGLONG BytesScrubbed;

} DEVICE_DATA_SET_SCRUB_EX_OUTPUT, *PDEVICE_DATA_SET_SCRUB_EX_OUTPUT,
  DEVICE_DSM_SCRUB_OUTPUT2, *PDEVICE_DSM_SCRUB_OUTPUT2;

//
// SingleRange    - No
// ParameterBlock - No
// Output         - Yes
// OutputBlock    - Yes
//

#define DeviceDsmDefinition_Scrub {DeviceDsmAction_Scrub,               \
                                   FALSE,                               \
                                   0,                                   \
                                   0,                                   \
                                   TRUE,                                \
                                   __alignof(DEVICE_DSM_SCRUB_OUTPUT2), \
                                   sizeof(DEVICE_DSM_SCRUB_OUTPUT2)}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_DrtQuery
//

//
// SingleRange    - No
// ParameterBlock - No
// Output         - Yes
// OutputBlock    - No
//

#define DeviceDsmDefinition_DrtQuery {DeviceDsmAction_DrtQuery, \
                                      FALSE,                    \
                                      0,                        \
                                      0,                        \
                                      TRUE,                     \
                                      0,                        \
                                      0}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_DrtClear
//

//
// SingleRange    - No
// ParameterBlock - No
// Output         - No
// OutputBlock    - No
//

#define DeviceDsmDefinition_DrtClear {DeviceDsmAction_DrtClear, \
                                      FALSE,                    \
                                      0,                        \
                                      0,                        \
                                      FALSE,                    \
                                      0,                        \
                                      0}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_DrtDisable
//

//
// SingleRange    - No
// ParameterBlock - No
// Output         - No
// OutputBlock    - No
//

#define DeviceDsmDefinition_DrtDisable {DeviceDsmAction_DrtDisable, \
                                        FALSE,                      \
                                        0,                          \
                                        0,                          \
                                        FALSE,                      \
                                        0,                          \
                                        0}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_TieringQuery
//

typedef struct _DEVICE_DSM_TIERING_QUERY_INPUT {

    ULONG Version;
    ULONG Size;

    //
    // Reserved for future use
    //

    ULONG Flags;

    ULONG NumberOfTierIds;
    GUID  TierIds[ANYSIZE_ARRAY];

} DEVICE_DSM_TIERING_QUERY_INPUT, *PDEVICE_DSM_TIERING_QUERY_INPUT,
  DEVICE_DSM_TIERING_QUERY_PARAMETERS, *PDEVICE_DSM_TIERING_QUERY_PARAMETERS;

typedef struct _STORAGE_TIER_REGION {

    GUID TierId;

    ULONGLONG Offset;
    ULONGLONG Length;

} STORAGE_TIER_REGION, *PSTORAGE_TIER_REGION;

typedef struct _DEVICE_DSM_TIERING_QUERY_OUTPUT {

    ULONG Version;
    ULONG Size;

    //
    // Reserved for future use
    //

    ULONG Flags;
    ULONG Reserved;

    //
    // Delta  from the  start offset
    // if the requested range is not
    // aligned to a  region boundary
    //

    ULONGLONG Alignment;

    //
    // Total  number of regions that
    // are in the specified range
    //

    ULONG TotalNumberOfRegions;

    ULONG NumberOfRegionsReturned;
    _Field_size_(NumberOfRegionsReturned) STORAGE_TIER_REGION Regions[ANYSIZE_ARRAY];

} DEVICE_DSM_TIERING_QUERY_OUTPUT, *PDEVICE_DSM_TIERING_QUERY_OUTPUT;

//
// SingleRange    - Yes
// ParameterBlock - Yes
// Output         - Yes
// OutputBlock    - Yes
//

#define DeviceDsmDefinition_TieringQuery {DeviceDsmAction_TieringQuery,                   \
                                          TRUE,                                           \
                                          __alignof(DEVICE_DSM_TIERING_QUERY_PARAMETERS), \
                                          sizeof(DEVICE_DSM_TIERING_QUERY_PARAMETERS),    \
                                          TRUE,                                           \
                                          __alignof(DEVICE_DSM_TIERING_QUERY_OUTPUT),     \
                                          sizeof(DEVICE_DSM_TIERING_QUERY_OUTPUT)}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_Map
//

//
// SingleRange    - No
// ParameterBlock - No
// Output         - No
// OutputBlock    - No
//

#define DeviceDsmDefinition_Map {DeviceDsmAction_Map, \
                                 FALSE,               \
                                 0,                   \
                                 0,                   \
                                 FALSE,               \
                                 0,                   \
                                 0}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_RegenerateParity
//

//
// SingleRange    - No
// ParameterBlock - No
// Output         - No
// OutputBlock    - No
//

#define DeviceDsmDefinition_RegenerateParity {DeviceDsmAction_RegenerateParity, \
                                              FALSE,                            \
                                              0,                                \
                                              0,                                \
                                              FALSE,                            \
                                              0,                                \
                                              0}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_NvCache_Change_Priority
//

typedef struct _DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {

    ULONG Size;

    UCHAR TargetPriority;
    UCHAR Reserved[3];

} DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS, *PDEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS;

//
// SingleRange    - No
// ParameterBlock - Yes
// Output         - No
// OutputBlock    - No
//

#define DeviceDsmDefinition_NvCache_Change_Priority {DeviceDsmAction_NvCache_Change_Priority,                  \
                                                     FALSE,                                                    \
                                                     __alignof(DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS), \
                                                     sizeof(DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS),    \
                                                     FALSE,                                                    \
                                                     0,                                                        \
                                                     0}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_NvCache_Evict
//

//
// SingleRange    - No
// ParameterBlock - No
// Output         - No
// OutputBlock    - No
//

#define DeviceDsmDefinition_NvCache_Evict {DeviceDsmAction_NvCache_Evict, \
                                           FALSE,                         \
                                           0,                             \
                                           0,                             \
                                           FALSE,                         \
                                           0,                             \
                                           0}

////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_TopologyIdQuery
//

typedef struct _DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {

    //
    // Number of bytes that topology
    // id describes  relative to the
    // start of an input range
    //

    ULONGLONG TopologyRangeBytes;

    //
    // The corresponding topology id
    //

    UCHAR TopologyId[16];

} DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT, *PDEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT,
  DEVICE_DSM_TOPOLOGY_ID_QUERY_OUTPUT, *PDEVICE_DSM_TOPOLOGY_ID_QUERY_OUTPUT;

//
// SingleRange    - No
// ParameterBlock - No
// Output         - Yes
// OutputBlock    - Yes
//

#define DeviceDsmDefinition_TopologyIdQuery {DeviceDsmAction_TopologyIdQuery,                \
                                             FALSE,                                          \
                                             0,                                              \
                                             0,                                              \
                                             TRUE,                                           \
                                             __alignof(DEVICE_DSM_TOPOLOGY_ID_QUERY_OUTPUT), \
                                             sizeof(DEVICE_DSM_TOPOLOGY_ID_QUERY_OUTPUT)}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_GetPhysicalAddresses
//

//
// DEVICE_DSM_INPUT.Flags
//

//
// If set, TotalNumberOfRanges field will be 0.
// A caller that doesn't need to know the total
// number of ranges  should set this flag  as a
// performance optimization, because the device
// might incur some cost  calculating the total
// number of ranges.
//

#define DEVICE_DSM_FLAG_PHYSICAL_ADDRESSES_OMIT_TOTAL_RANGES 0x10000000

//
// A driver can set the StartAddress field to this value
// to indicate that an address range has a memory error.
// Address ranges with memory errors must not be merged:
// if there are two physically contiguous address ranges
// with errors, they must be reported as two separate
// address ranges, both of which have StartAddress set
// to this value.
//

#define DEVICE_DSM_PHYSICAL_ADDRESS_HAS_MEMORY_ERROR ((LONGLONG)-1)

typedef struct _DEVICE_STORAGE_ADDRESS_RANGE {

    LONGLONG    StartAddress;
    ULONGLONG   LengthInBytes;

} DEVICE_STORAGE_ADDRESS_RANGE, *PDEVICE_STORAGE_ADDRESS_RANGE;

typedef struct _DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {

    ULONG Version;

    //
    // Reserved for future use
    //

    ULONG Flags;

    //
    // Total number of ranges within
    // the specified ranges. Callers
    // may use it to  determine  the
    // correct  size of  this output
    // buffer
    //

    ULONG TotalNumberOfRanges;

    //
    // If the buffer provided by the
    // caller is not large enough to
    // hold all the requested ranges
    // a STATUS_BUFFER_OVERFLOW will
    // be returned
    //

    ULONG NumberOfRangesReturned;
    DEVICE_STORAGE_ADDRESS_RANGE Ranges[ANYSIZE_ARRAY];

} DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT, *PDEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT;

#define DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT_V1         1
#define DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT_VERSION_V1 DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT_V1

//
// SingleRange    - No
// ParameterBlock - No
// Output         - Yes
// OutputBlock    - Yes
//

#define DeviceDsmDefinition_GetPhysicalAddresses {DeviceDsmAction_GetPhysicalAddresses,            \
                                                  FALSE,                                           \
                                                  0,                                               \
                                                  0,                                               \
                                                  TRUE,                                            \
                                                  __alignof(DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT), \
                                                  sizeof(DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT)}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_ScopeRegen
//

//
// SingleRange    - No
// ParameterBlock - No
// Output         - No
// OutputBlock    - No
//

#define DeviceDsmDefinition_ScopeRegen {DeviceDsmAction_ScopeRegen, \
                                        FALSE,                      \
                                        0,                          \
                                        0,                          \
                                        FALSE,                      \
                                        0,                          \
                                        0}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_ReportZones
//

typedef struct _DEVICE_DSM_REPORT_ZONES_PARAMETERS {

    ULONG Size;

    UCHAR ReportOption;

    //
    // This  bit affects calculation
    // of the zone list length
    //

    UCHAR Partial;

    UCHAR Reserved[2];

} DEVICE_DSM_REPORT_ZONES_PARAMETERS, *PDEVICE_DSM_REPORT_ZONES_PARAMETERS;

typedef enum _STORAGE_ZONES_ATTRIBUTES {

    ZonesAttributeTypeAndLengthMayDifferent       = 0,
    ZonesAttributeTypeSameLengthSame              = 1,
    ZonesAttributeTypeSameLastZoneLengthDifferent = 2,
    ZonesAttributeTypeMayDifferentLengthSame      = 3,

} STORAGE_ZONES_ATTRIBUTES, *PSTORAGE_ZONES_ATTRIBUTES;

typedef enum _STORAGE_ZONE_CONDITION {

    ZoneConditionConventional     = 0x00,
    ZoneConditionEmpty            = 0x01,
    ZoneConditionImplicitlyOpened = 0x02,
    ZoneConditionExplicitlyOpened = 0x03,
    ZoneConditionClosed           = 0x04,

    ZoneConditionReadOnly         = 0x0D,
    ZoneConditionFull             = 0x0E,
    ZoneConditionOffline          = 0x0F,

} STORAGE_ZONE_CONDITION, *PSTORAGE_ZONE_CONDITION;

typedef struct _STORAGE_ZONE_DESCRIPTOR {

    ULONG Size;

    STORAGE_ZONE_TYPES ZoneType;
    STORAGE_ZONE_CONDITION ZoneCondition;

    BOOLEAN ResetWritePointerRecommend;
    UCHAR Reserved0[3];

    //
    // In bytes
    //

    ULONGLONG ZoneSize;
    ULONGLONG WritePointerOffset;

} STORAGE_ZONE_DESCRIPTOR, *PSTORAGE_ZONE_DESCRIPTOR;

typedef struct _DEVICE_DSM_REPORT_ZONES_DATA {

    ULONG Size;

    //
    // Represents the number of ZoneDescriptors.
    //
    ULONG ZoneCount;

    STORAGE_ZONES_ATTRIBUTES Attributes;

    ULONG Reserved0;

    _Field_size_(ZoneCount) STORAGE_ZONE_DESCRIPTOR ZoneDescriptors[ANYSIZE_ARRAY];

} DEVICE_DSM_REPORT_ZONES_DATA, *PDEVICE_DSM_REPORT_ZONES_DATA,
  DEVICE_DSM_REPORT_ZONES_OUTPUT, *PDEVICE_DSM_REPORT_ZONES_OUTPUT;

//
// SingleRange    - No
// ParameterBlock - Yes
// Output         - Yes
// OutputBlock    - Yes
//

#define DeviceDsmDefinition_ReportZones  {DeviceDsmAction_ReportZones,                   \
                                          FALSE,                                         \
                                          __alignof(DEVICE_DSM_REPORT_ZONES_PARAMETERS), \
                                          sizeof(DEVICE_DSM_REPORT_ZONES_PARAMETERS),    \
                                          TRUE,                                          \
                                          __alignof(DEVICE_DSM_REPORT_ZONES_OUTPUT),     \
                                          sizeof(DEVICE_DSM_REPORT_ZONES_OUTPUT)}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_OpenZone
//

//
// SingleRange    - No
// ParameterBlock - No
// Output         - No
// OutputBlock    - No
//

#define DeviceDsmDefinition_OpenZone {DeviceDsmAction_OpenZone, \
                                      FALSE,                    \
                                      0,                        \
                                      0,                        \
                                      FALSE,                    \
                                      0,                        \
                                      0}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_FinishZone
//

//
// SingleRange    - No
// ParameterBlock - No
// Output         - No
// OutputBlock    - No
//

#define DeviceDsmDefinition_FinishZone {DeviceDsmAction_FinishZone, \
                                        FALSE,                      \
                                        0,                          \
                                        0,                          \
                                        FALSE,                      \
                                        0,                          \
                                        0}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_CloseZone
//

//
// SingleRange    - No
// ParameterBlock - No
// Output         - No
// OutputBlock    - No
//

#define DeviceDsmDefinition_CloseZone {DeviceDsmAction_CloseZone, \
                                       FALSE,                     \
                                       0,                         \
                                       0,                         \
                                       FALSE,                     \
                                       0,                         \
                                       0}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_ResetWritePointer
//

//
// SingleRange    - No
// ParameterBlock - No
// Output         - No
// OutputBlock    - No
//

#define DeviceDsmDefinition_ResetWritePointer {DeviceDsmAction_ResetWritePointer, \
                                               FALSE,                             \
                                               0,                                 \
                                               0,                                 \
                                               FALSE,                             \
                                               0,                                 \
                                               0}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_GetRangeErrorInfo
//

#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/unions
#pragma warning(disable:4214) // bit fields other than int

typedef struct _DEVICE_STORAGE_RANGE_ATTRIBUTES {

    //
    // Must be a  multiple of sector
    // size, in bytes
    //

    ULONGLONG LengthInBytes;

    union {

        ULONG AllFlags;

        struct {

            //
            // 1 = bad, 0 = good
            //

            ULONG IsRangeBad : 1;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;

    ULONG Reserved;

} DEVICE_STORAGE_RANGE_ATTRIBUTES, *PDEVICE_STORAGE_RANGE_ATTRIBUTES;

#pragma warning(pop)

//
// DEVICE_DSM_RANGE_ERROR_OUTPUT.Flags
//

#define DEVICE_STORAGE_NO_ERRORS                0x1

typedef struct _DEVICE_DSM_RANGE_ERROR_INFO {

    ULONG Version;

    ULONG Flags;

    //
    // Total number of ranges within
    // the specified ranges. Callers
    // may use it to  determine  the
    // correct  size of  this output
    // buffer
    //

    ULONG TotalNumberOfRanges;

    //
    // If the buffer provided by the
    // caller is not large enough to
    // hold all the requested ranges
    // a STATUS_BUFFER_OVERFLOW will
    // be returned
    //
    // The output ranges, which inform the caller about which regions of the requested
    // ranges are good or bad. The elements of this array are sorted so that their order
    // corresponds to the order of the input ranges. For example, if the first input
    // range was broken into 3 output ranges, those will be the first 3 ranges in the array.
    // The caller can learn which output ranges correspond to an input range by keeping track
    // of the length of the output ranges.
    //

    ULONG NumberOfRangesReturned;
    DEVICE_STORAGE_RANGE_ATTRIBUTES Ranges[ANYSIZE_ARRAY];

} DEVICE_DSM_RANGE_ERROR_INFO, *PDEVICE_DSM_RANGE_ERROR_INFO,
  DEVICE_DSM_RANGE_ERROR_OUTPUT, *PDEVICE_DSM_RANGE_ERROR_OUTPUT;

#define DEVICE_DSM_RANGE_ERROR_OUTPUT_V1        1
#define DEVICE_DSM_RANGE_ERROR_INFO_VERSION_V1  DEVICE_DSM_RANGE_ERROR_OUTPUT_V1

//
// SingleRange    - No
// ParameterBlock - No
// Output         - Yes
// OutputBlock    - Yes
//

#define DeviceDsmDefinition_GetRangeErrorInfo {DeviceDsmAction_GetRangeErrorInfo,        \
                                               FALSE,                                    \
                                               0,                                        \
                                               0,                                        \
                                               TRUE,                                     \
                                               __alignof(DEVICE_DSM_RANGE_ERROR_OUTPUT), \
                                               sizeof(DEVICE_DSM_RANGE_ERROR_OUTPUT)}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_WriteZeroes
//
// ParameterBlock - No
// Output         - No
//

//
// SingleRange    - No
// ParameterBlock - No
// Output         - No
// OutputBlock    - No
//

#define DeviceDsmDefinition_WriteZeroes {DeviceDsmAction_WriteZeroes, \
                                         FALSE,                       \
                                         0,                           \
                                         0,                           \
                                         FALSE,                       \
                                         0,                           \
                                         0}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_LostQuery
//

typedef struct _DEVICE_DSM_LOST_QUERY_PARAMETERS {

    //
    // Size of this structure serves
    // as the version
    //

    ULONG Version;

    ULONGLONG Granularity;

} DEVICE_DSM_LOST_QUERY_PARAMETERS, *PDEVICE_DSM_LOST_QUERY_PARAMETERS;

typedef struct _DEVICE_DSM_LOST_QUERY_OUTPUT {

    //
    // Size of this structure serves
    // as the version
    //

    ULONG Version;

    //
    // Size of  this structure  plus
    // all the variable sized fields
    // needed for  the  entire range
    //

    ULONG Size;

    //
    // Delta  from the  start offset
    // if the requested range is not
    // granularity aligned
    //

    ULONGLONG Alignment;

    //
    // 1 = lost, 0 = readable
    //

    ULONG NumberOfBits;
    ULONG BitMap[ANYSIZE_ARRAY];

} DEVICE_DSM_LOST_QUERY_OUTPUT, *PDEVICE_DSM_LOST_QUERY_OUTPUT;

//
// SingleRange    - Yes
// ParameterBlock - Yes
// Output         - Yes
// OutputBlock    - Yes
//

#define DeviceDsmDefinition_LostQuery {DeviceDsmAction_LostQuery,                   \
                                       TRUE,                                        \
                                       __alignof(DEVICE_DSM_LOST_QUERY_PARAMETERS), \
                                       sizeof(DEVICE_DSM_LOST_QUERY_PARAMETERS),    \
                                       TRUE,                                        \
                                       __alignof(DEVICE_DSM_LOST_QUERY_OUTPUT),     \
                                       sizeof(DEVICE_DSM_LOST_QUERY_OUTPUT)}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_GetFreeSpace
//

typedef struct _DEVICE_DSM_FREE_SPACE_OUTPUT {

    //
    // Size of this structure serves
    // as the version
    //

    ULONG Version;

    //
    // Shared free space available
    //

    ULONGLONG FreeSpace;

} DEVICE_DSM_FREE_SPACE_OUTPUT, *PDEVICE_DSM_FREE_SPACE_OUTPUT;

//
// SingleRange    - No
// ParameterBlock - No
// Output         - Yes
// OutputBlock    - Yes
//

#define DeviceDsmDefinition_GetFreeSpace {DeviceDsmAction_GetFreeSpace,            \
                                          FALSE,                                   \
                                          0,                                       \
                                          0,                                       \
                                          TRUE,                                    \
                                          __alignof(DEVICE_DSM_FREE_SPACE_OUTPUT), \
                                          sizeof(DEVICE_DSM_FREE_SPACE_OUTPUT)}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_ConversionQuery
//

typedef struct _DEVICE_DSM_CONVERSION_OUTPUT {

    //
    // Size of this structure serves
    // as the version
    //

    ULONG Version;

    //
    // Stable  identifier associated
    // with the source
    //

    GUID Source;

} DEVICE_DSM_CONVERSION_OUTPUT, *PDEVICE_DSM_CONVERSION_OUTPUT;

//
// SingleRange    - Yes
// ParameterBlock - No
// Output         - Yes
// OutputBlock    - Yes
//

#define DeviceDsmDefinition_ConversionQuery {DeviceDsmAction_ConversionQuery,         \
                                             TRUE,                                    \
                                             0,                                       \
                                             0,                                       \
                                             TRUE,                                    \
                                             __alignof(DEVICE_DSM_CONVERSION_OUTPUT), \
                                             sizeof(DEVICE_DSM_CONVERSION_OUTPUT)}


////////////////////////////////////////////////////////////////////////////////
//
// DeviceDsmAction_VdtSet
//

//
// SingleRange    - No
// ParameterBlock - No
// Output         - No
// OutputBlock    - No
//

#define DeviceDsmDefinition_VdtSet {DeviceDsmAction_VdtSet, \
                                    FALSE,                  \
                                    0,                      \
                                    0,                      \
                                    FALSE,                  \
                                    0,                      \
                                    0}


////////////////////////////////////////////////////////////////////////////////
//
// Dsm helper routines
//

#define DEVICE_DSM_ROUND_UP(_a, _b)             (((_a) + ((_b) - 1)) / (_b) * (_b))
#define DEVICE_DSM_ROUND_DN(_a, _b)             (((_a)             ) / (_b) * (_b))


FORCEINLINE
PVOID
DeviceDsmParameterBlock (
    _In_ PDEVICE_DSM_INPUT Input
    )
{
    return (PVOID)
           ((ULONG_PTR)Input +
                       Input->ParameterBlockOffset);
}


FORCEINLINE
PDEVICE_DSM_RANGE
DeviceDsmDataSetRanges (
    _In_ PDEVICE_DSM_INPUT Input
    )
{
    return (PDEVICE_DSM_RANGE)
           ((ULONG_PTR)Input +
                       Input->DataSetRangesOffset);
}


FORCEINLINE
ULONG
DeviceDsmNumberOfDataSetRanges (
    _In_ PDEVICE_DSM_INPUT Input
    )
{
    return Input->DataSetRangesLength /
           sizeof(DEVICE_DSM_RANGE);
}


FORCEINLINE
ULONG
DeviceDsmGetInputLength (
    _In_ PDEVICE_DSM_DEFINITION Definition,
    _In_ ULONG ParameterBlockLength,
    _In_ ULONG NumberOfDataSetRanges
    )
{
    ULONG Bytes = sizeof(DEVICE_DSM_INPUT);

    if (Definition->ParameterBlockLength != 0 &&
        ParameterBlockLength != 0) {

        Bytes  = DEVICE_DSM_ROUND_UP(Bytes, Definition->ParameterBlockAlignment);
        Bytes += ParameterBlockLength;
    }

    if (NumberOfDataSetRanges != 0) {

        Bytes  = DEVICE_DSM_ROUND_UP(Bytes, __alignof(DEVICE_DSM_RANGE));
        Bytes += sizeof(DEVICE_DSM_RANGE) * NumberOfDataSetRanges;
    }

    return Bytes;
}


FORCEINLINE
ULONG
DeviceDsmGetNumberOfDataSetRanges (
    _In_ PDEVICE_DSM_DEFINITION Definition,
    _In_ ULONG InputLength,
    _In_ ULONG ParameterBlockLength
    )
{
    ULONG Bytes = sizeof(DEVICE_DSM_INPUT);

    if (Definition->ParameterBlockLength != 0 &&
        ParameterBlockLength != 0) {

        Bytes  = DEVICE_DSM_ROUND_UP(Bytes, Definition->ParameterBlockAlignment);
        Bytes += ParameterBlockLength;
    }

    Bytes = DEVICE_DSM_ROUND_UP(Bytes, __alignof(DEVICE_DSM_RANGE));
    Bytes = InputLength - Bytes;

    return Bytes / sizeof(DEVICE_DSM_RANGE);
}


FORCEINLINE
VOID
DeviceDsmInitializeInput (
    _In_ PDEVICE_DSM_DEFINITION Definition,
    _Out_writes_bytes_(InputLength) PDEVICE_DSM_INPUT Input,
    _In_ _In_range_(>=, sizeof(DEVICE_DSM_INPUT)) ULONG InputLength,
    _In_ ULONG Flags,
    _In_reads_bytes_opt_(ParameterBlockLength) PVOID Parameters,
    _In_ ULONG ParameterBlockLength
    )
{
    ULONG Bytes = sizeof(DEVICE_DSM_INPUT);

    RtlZeroMemory(Input, InputLength);

    Input->Size   = Bytes;
    Input->Action = Definition->Action;
    Input->Flags  = Flags;

    if (Definition->ParameterBlockLength == 0 ||
        ParameterBlockLength == 0) {

        goto Cleanup;
    }

    Bytes = DEVICE_DSM_ROUND_UP(Bytes, Definition->ParameterBlockAlignment);

    Input->ParameterBlockOffset = Bytes;
    Input->ParameterBlockLength = ParameterBlockLength;

    if (!Parameters) {
        goto Cleanup;
    }

    RtlCopyMemory(DeviceDsmParameterBlock(Input),
                  Parameters,
                  Input->ParameterBlockLength);

Cleanup:

    return;
}


FORCEINLINE
BOOLEAN
DeviceDsmAddDataSetRange (
    _Inout_updates_bytes_(InputLength) PDEVICE_DSM_INPUT Input,
    _In_ _In_range_(>=, sizeof(DEVICE_DSM_INPUT)) ULONG InputLength,
    _In_ LONGLONG Offset,
    _In_ ULONGLONG Length
    )
{
    ULONG             Bytes  = 0;
    ULONG             Index  = 0;
    PDEVICE_DSM_RANGE Ranges = NULL;
    BOOLEAN           Return = FALSE;

    if (Input->Flags & DEVICE_DSM_FLAG_ENTIRE_DATA_SET_RANGE) {
        goto Cleanup;
    }

    if (Input->DataSetRangesLength == 0) {

        if (Input->ParameterBlockLength == 0) {

            Bytes = sizeof(DEVICE_DSM_INPUT);

        } else {

            Bytes = Input->ParameterBlockOffset +
                    Input->ParameterBlockLength;
        }

        Bytes = DEVICE_DSM_ROUND_UP(Bytes, __alignof(DEVICE_DSM_RANGE));

    } else {

        Bytes = Input->DataSetRangesOffset +
                Input->DataSetRangesLength;
    }

    if ((InputLength - Bytes) < sizeof(DEVICE_DSM_RANGE)) {
        goto Cleanup;
    }

    if (Input->DataSetRangesOffset == 0) {
        Input->DataSetRangesOffset  = Bytes;
    }

    Ranges = DeviceDsmDataSetRanges(Input);
    Index  = DeviceDsmNumberOfDataSetRanges(Input);

    Ranges[Index].StartingOffset = Offset;
    Ranges[Index].LengthInBytes  = Length;

    Input->DataSetRangesLength += sizeof(DEVICE_DSM_RANGE);

    Return = TRUE;

Cleanup:

    return Return;
}


FORCEINLINE
BOOLEAN
DeviceDsmValidateInput (
    _In_ PDEVICE_DSM_DEFINITION Definition,
    _In_reads_bytes_(InputLength) PDEVICE_DSM_INPUT Input,
    _In_  _In_range_(>=, sizeof(DEVICE_DSM_INPUT)) ULONG InputLength
    )
{
    ULONG   Max   = 0;
    ULONG   Min   = 0;
    BOOLEAN Valid = FALSE;

    if (Input->Size != sizeof(*Input)) {
        goto Cleanup;
    }

    if (Definition->Action != Input->Action) {
        goto Cleanup;
    }

    if (Definition->ParameterBlockLength != 0) {

        Min = sizeof(*Input);
        Max = InputLength;

        if (Input->ParameterBlockOffset < Min ||
            Input->ParameterBlockOffset > Max ||
            Input->ParameterBlockOffset % Definition->ParameterBlockAlignment) {
            goto Cleanup;
        }

        Min = Definition->ParameterBlockLength;
        Max = InputLength - Input->ParameterBlockOffset;

        if (Input->ParameterBlockLength < Min ||
            Input->ParameterBlockLength > Max) {
            goto Cleanup;
        }

    } else {

        if (Input->ParameterBlockLength != 0 ||
            Input->ParameterBlockOffset != 0) {
            goto Cleanup;
        }
    }

    if (!(Input->Flags & DEVICE_DSM_FLAG_ENTIRE_DATA_SET_RANGE)) {

        Min = sizeof(*Input);
        Max = InputLength;

        if (Input->DataSetRangesOffset < Min ||
            Input->DataSetRangesOffset > Max ||
            Input->DataSetRangesOffset % __alignof(DEVICE_DSM_RANGE)) {
            goto Cleanup;
        }

        Min = sizeof(DEVICE_DSM_RANGE);
        Max = InputLength - Input->DataSetRangesOffset;

        if (Input->DataSetRangesLength < Min ||
            Input->DataSetRangesLength > Max ||
            Input->DataSetRangesLength % Min) {
            goto Cleanup;
        }

        if (Definition->SingleRange &&
            Input->DataSetRangesLength != Min) {
            goto Cleanup;
        }

    } else {

        if (Input->DataSetRangesOffset != 0 ||
            Input->DataSetRangesLength != 0) {
            goto Cleanup;
        }
    }

    if (Input->ParameterBlockOffset < Input->DataSetRangesOffset &&
        Input->ParameterBlockOffset +
        Input->ParameterBlockLength > Input->DataSetRangesOffset) {
        goto Cleanup;
    }

    if (Input->DataSetRangesOffset < Input->ParameterBlockOffset &&
        Input->DataSetRangesOffset +
        Input->DataSetRangesLength > Input->ParameterBlockOffset) {
        goto Cleanup;
    }

    Valid = TRUE;

Cleanup:

    return Valid;
}


FORCEINLINE
PVOID
DeviceDsmOutputBlock (
    _In_ PDEVICE_DSM_OUTPUT Output
    )
{
    return (PVOID)
           ((ULONG_PTR)Output + Output->OutputBlockOffset);
}


FORCEINLINE
ULONG
DeviceDsmGetOutputLength (
    _In_ PDEVICE_DSM_DEFINITION Definition,
    _In_ ULONG OutputBlockLength
    )
{
    ULONG Bytes = 0;

    if (!Definition->HasOutput) {
        goto Cleanup;
    }

    Bytes  = sizeof(DEVICE_DSM_OUTPUT);

    if (Definition->OutputBlockLength == 0) {
        goto Cleanup;
    }

    if (Definition->OutputBlockLength > OutputBlockLength) {
        OutputBlockLength = Definition->OutputBlockLength;
    }

    Bytes  = DEVICE_DSM_ROUND_UP(Bytes, Definition->OutputBlockAlignment);
    Bytes += OutputBlockLength;

Cleanup:

    return Bytes;
}


FORCEINLINE
BOOLEAN
DeviceDsmValidateOutputLength (
    _In_ PDEVICE_DSM_DEFINITION Definition,
    _In_ ULONG OutputLength
    )
{
    ULONG Bytes = DeviceDsmGetOutputLength(Definition,
                                           0);

    return (OutputLength >= Bytes);
}


FORCEINLINE
ULONG
DeviceDsmGetOutputBlockLength (
    _In_ PDEVICE_DSM_DEFINITION Definition,
    _In_ ULONG OutputLength
    )
{
    ULONG Bytes = 0;

    if (Definition->OutputBlockLength == 0) {
        goto Cleanup;
    }

    Bytes = sizeof(DEVICE_DSM_OUTPUT);
    Bytes = DEVICE_DSM_ROUND_UP(Bytes, Definition->OutputBlockAlignment);
    Bytes = OutputLength - Bytes;

Cleanup:

    return Bytes;
}


FORCEINLINE
VOID
DeviceDsmInitializeOutput (
    _In_ PDEVICE_DSM_DEFINITION Definition,
    _Out_writes_bytes_(OutputLength) PDEVICE_DSM_OUTPUT Output,
    _In_  _In_range_(>=, sizeof(DEVICE_DSM_OUTPUT)) ULONG OutputLength,
    _In_ ULONG Flags
    )
{
    ULONG Bytes = sizeof(DEVICE_DSM_OUTPUT);

    RtlZeroMemory(Output, OutputLength);

    Output->Size   = Bytes;
    Output->Action = Definition->Action;
    Output->Flags  = Flags;

    if (Definition->OutputBlockLength != 0) {

        Bytes = DEVICE_DSM_ROUND_UP(Bytes, Definition->OutputBlockAlignment);

        Output->OutputBlockOffset = Bytes;
        Output->OutputBlockLength = OutputLength - Bytes;
    }

    return;
}


FORCEINLINE
BOOLEAN
DeviceDsmValidateOutput (
    _In_ PDEVICE_DSM_DEFINITION Definition,
    _In_reads_bytes_(OutputLength) PDEVICE_DSM_OUTPUT Output,
    _In_ _In_range_(>=, sizeof(DEVICE_DSM_OUTPUT)) ULONG OutputLength
    )
{
    ULONG   Max   = 0;
    ULONG   Min   = 0;
    BOOLEAN Valid = FALSE;

    if (Definition->Action != Output->Action) {
        goto Cleanup;
    }

    if (!Definition->HasOutput) {
        goto Cleanup;
    }

    if (Definition->OutputBlockLength != 0) {

        Min = sizeof(*Output);
        Max = OutputLength;

        if (Output->OutputBlockOffset < Min ||
            Output->OutputBlockOffset > Max ||
            Output->OutputBlockOffset % Definition->OutputBlockAlignment) {
            goto Cleanup;
        }

        Min = Definition->OutputBlockLength;
        Max = OutputLength - Output->OutputBlockOffset;

        if (Output->OutputBlockLength < Min ||
            Output->OutputBlockLength > Max) {
            goto Cleanup;
        }

    } else {

        if (Output->OutputBlockOffset != 0 ||
            Output->OutputBlockLength != 0) {
            goto Cleanup;
        }
    }

    Valid = TRUE;

Cleanup:

    return Valid;
}

//
// end IOCTL_STORAGE_MANAGE_DATA_SET_ATTRIBUTES
//
////////////////////////////////////////////////////////////////////////////////


//
//  There are some well known GUIDS for certain types of files.  They are
//  defined in NTIFS.H
//

//
// IOCTL_STORAGE_GET_BC_PROPERTIES
//
// Input Buffer:
//     None
//
// Output Buffer:
//     Structure of type STORAGE_GET_BC_PROPERTIES_OUTPUT
//

typedef struct _STORAGE_GET_BC_PROPERTIES_OUTPUT {

    //
    // Specifies the maximum number of requests
    // that can be scheduled per period of time
    //
    ULONG MaximumRequestsPerPeriod;

    //
    // Specifies the minimum period that the
    // device uses  when scheduling requests
    //
    ULONG MinimumPeriod;

    //
    // Specifies the maximum transfer size supported
    // for  bandwidth contracts  on this  device. To
    // achieve the highest level of performance, all
    // requests should be of this size
    //
    ULONGLONG MaximumRequestSize;

    //
    // Specifies the estimated time taken to
    // perform an  Io operstion. This  field
    // is  for  informational purposes  only
    //
    ULONG EstimatedTimePerRequest;

    //
    // Specifies the number of requests that should be
    // kept outstanding.  This helps  keep the  device
    // device busy and thus obtain maximum throughput.
    // This will only be filled in if the target  file
    // has an outstanding contract.
    //
    ULONG NumOutStandingRequests;

    //
    // Specifies the required size of requests in this
    // stream.  This  will  only  be filled in  if the
    // target file has an outstanding contract.
    //
    ULONGLONG RequestSize;

} STORAGE_GET_BC_PROPERTIES_OUTPUT, *PSTORAGE_GET_BC_PROPERTIES_OUTPUT;


//
// IOCTL_STORAGE_ALLOCATE_BC_STREAM
//
// Input Buffer:
//     Structure of type STORAGE_ALLOCATE_BC_STREAM_INPUT
//
// Output Buffer:
//     Structure of type STORAGE_ALLOCATE_BC_STREAM_OUTPUT
//


//
// Current version
//
#define IOCTL_STORAGE_BC_VERSION                1

typedef struct _STORAGE_ALLOCATE_BC_STREAM_INPUT {

    //
    // Specifies the corresponding structure version
    //
    ULONG Version;

    //
    // Specifies the number of requests that
    // need to  complete  per period of time
    //
    ULONG RequestsPerPeriod;

    //
    // Specifies the period of time wherein the
    // above  number of requests  must complete
    //
    ULONG Period;

    //
    // Indicates whether failures
    // should  be retried  or not
    //
    BOOLEAN RetryFailures;

    //
    // Indicates whether reqests that  will miss
    // their deadline should be discarded or not
    //
    BOOLEAN Discardable;

    //
    // Helps align the following field
    //
    BOOLEAN Reserved1[2];

    //
    // Indicates whether the  Io  will be
    // comprised of reads, writes or both
    //
    ULONG AccessType;

    //
    // Indicates whether the  Io  to the
    // file will be sequential or random
    //
    ULONG AccessMode;

} STORAGE_ALLOCATE_BC_STREAM_INPUT, *PSTORAGE_ALLOCATE_BC_STREAM_INPUT;

typedef struct _STORAGE_ALLOCATE_BC_STREAM_OUTPUT {

    //
    // Specifies the required size
    // of  requests in this stream
    //
    ULONGLONG RequestSize;

    //
    // Specifies the number of requests that should be
    // kept outstanding.  This helps  keep the  device
    // device busy and thus obtain maximum  throughput
    //
    ULONG NumOutStandingRequests;

} STORAGE_ALLOCATE_BC_STREAM_OUTPUT, *PSTORAGE_ALLOCATE_BC_STREAM_OUTPUT;


//
// IOCTL_STORAGE_FREE_BC_STREAM
//
// Input Buffer:
//     None
//
// Output Buffer:
//     None
//

//
// IOCTL_STORAGE_CHECK_PRIORITY_HINT_SUPPORT
//
// Input Buffer :
//      None
// Output Buffer :
//      Structure of type STORAGE_PRIORITY_HINT_SUPPORT
//

#define STORAGE_PRIORITY_HINT_SUPPORTED     0x0001

typedef struct _STORAGE_PRIORITY_HINT_SUPPORT {
    ULONG SupportFlags;
} STORAGE_PRIORITY_HINT_SUPPORT, *PSTORAGE_PRIORITY_HINT_SUPPORT;

//
// IOCTL_STORAGE_DIAGNOSTIC
//
// Input Buffer :
//      STORAGE_DIAGNOSTIC_REQUEST
// Output Buffer :
//      STORAGE_DIAGNOSTIC_DATA
//

typedef enum _STORAGE_DIAGNOSTIC_LEVEL {
    StorageDiagnosticLevelDefault = 0,
    StorageDiagnosticLevelMax
} STORAGE_DIAGNOSTIC_LEVEL, *PSTORAGE_DIAGNOSTIC_LEVEL;

typedef enum _STORAGE_DIAGNOSTIC_TARGET_TYPE {

    StorageDiagnosticTargetTypeUndefined = 0,
    StorageDiagnosticTargetTypePort,
    StorageDiagnosticTargetTypeMiniport,
    StorageDiagnosticTargetTypeHbaFirmware,
    StorageDiagnosticTargetTypeMax

} STORAGE_DIAGNOSTIC_TARGET_TYPE, *PSTORAGE_DIAGNOSTIC_TARGET_TYPE;

//
// Indicate the target of the request other than the device handle/object itself.
// This is used in "Flags" field of data structures.
//
#define STORAGE_DIAGNOSTIC_FLAG_ADAPTER_REQUEST     0x00000001

//
// STORAGE_DIAGNOSTIC_REQUEST
//

typedef struct _STORAGE_DIAGNOSTIC_REQUEST {

    // Size of this structure.
    ULONG Version;

    // Whole size of the structure and the associated data buffer.
    // (In case adding variable-sized buffer in future.)
    ULONG Size;

    // Request flag.
    ULONG Flags;

    // Request target type. See definitions for STORAGE_DIAGNOSTIC_TARGET_TYPE.
    STORAGE_DIAGNOSTIC_TARGET_TYPE TargetType;

    // Diagnostic level. See definitions for STORAGE_DIAGNOSTIC_LEVEL.
    STORAGE_DIAGNOSTIC_LEVEL Level;

} STORAGE_DIAGNOSTIC_REQUEST, *PSTORAGE_DIAGNOSTIC_REQUEST;

//
// STORAGE_DIAGNOSTIC_DATA
//

typedef struct _STORAGE_DIAGNOSTIC_DATA {

    // Size of this structure.
    ULONG Version;

    // Whole size of the structure and the associated data buffer.
    ULONG Size;

    // GUID of diagnostic data provider.
    GUID ProviderId;

    // If the request failed because of buffer too small, this field should be filled with the required buffer
    // size for DiagnosticDataBuffer needed by provider;
    // if the request is successful, it should be filled with returned buffer size of DiagnosticDataBuffer;
    // it should be cleared to zero for other cases.
    ULONG BufferSize;

    // Reserved for future use.
    ULONG Reserved;

    // Diagnostic data buffer.
    _Field_size_(BufferSize) UCHAR DiagnosticDataBuffer[ANYSIZE_ARRAY];

} STORAGE_DIAGNOSTIC_DATA, *PSTORAGE_DIAGNOSTIC_DATA;

//
// IOCTL_STORAGE_GET_PHYSICAL_ELEMENT_STATUS
//
// Input:
//       PHYSICAL_ELEMENT_STATUS_REQUEST
// Output:
//       PHYSICAL_ELEMENT_STATUS
//

typedef struct _PHYSICAL_ELEMENT_STATUS_REQUEST {

    ULONG Version;
    ULONG Size;

    ULONG StartingElement;
    UCHAR Filter;
    UCHAR ReportType;
    UCHAR Reserved[2];

} PHYSICAL_ELEMENT_STATUS_REQUEST, *PPHYSICAL_ELEMENT_STATUS_REQUEST;

typedef struct _PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {

    ULONG Version;
    ULONG Size;

    ULONG ElementIdentifier;
    UCHAR PhysicalElementType;
    UCHAR PhysicalElementHealth;
    UCHAR Reserved1[2];

    // In unit of LBA.
    ULONGLONG AssociatedCapacity;

    ULONG Reserved2[4];

} PHYSICAL_ELEMENT_STATUS_DESCRIPTOR, *PPHYSICAL_ELEMENT_STATUS_DESCRIPTOR;

typedef struct _PHYSICAL_ELEMENT_STATUS {

    ULONG Version;
    ULONG Size;

    ULONG DescriptorCount;
    ULONG ReturnedDescriptorCount;

    ULONG ElementIdentifierBeingDepoped;
    ULONG Reserved;

    PHYSICAL_ELEMENT_STATUS_DESCRIPTOR Descriptors[ANYSIZE_ARRAY];

} PHYSICAL_ELEMENT_STATUS, *PPHYSICAL_ELEMENT_STATUS;

//
// IOCTL_STORAGE_REMOVE_ELEMENT_AND_TRUNCATE
//
// Input:
//       REMOVE_ELEMENT_AND_TRUNCATE_REQUEST
//

typedef struct _REMOVE_ELEMENT_AND_TRUNCATE_REQUEST {

    ULONG Version;
    ULONG Size;

    // In unit of LBA.
    ULONGLONG RequestCapacity;

    ULONG ElementIdentifier;
    ULONG Reserved;

} REMOVE_ELEMENT_AND_TRUNCATE_REQUEST, *PREMOVE_ELEMENT_AND_TRUNCATE_REQUEST;

//
// IOCTL_STORAGE_GET_DEVICE_INTERNAL_LOG
//
// Input:
//       GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST
// Output:
//       DEVICE_INTERNAL_STATUS_DATA
//

#define ERROR_HISTORY_DIRECTORY_ENTRY_DEFAULT_COUNT    8

typedef enum _DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE {
    DeviceInternalStatusDataRequestTypeUndefined = 0,
    DeviceCurrentInternalStatusDataHeader,
    DeviceCurrentInternalStatusData,
    DeviceSavedInternalStatusDataHeader,
    DeviceSavedInternalStatusData
} DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE, *PDEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE;

typedef enum _DEVICE_INTERNAL_STATUS_DATA_SET {
    DeviceStatusDataSetUndefined = 0,
    DeviceStatusDataSet1,
    DeviceStatusDataSet2,
    DeviceStatusDataSet3,
    DeviceStatusDataSet4,
    DeviceStatusDataSetMax
} DEVICE_INTERNAL_STATUS_DATA_SET, *PDEVICE_INTERNAL_STATUS_DATA_SET;

typedef struct _GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST {

    ULONG Version;
    ULONG Size;

    DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE RequestDataType;
    DEVICE_INTERNAL_STATUS_DATA_SET RequestDataSet;

} GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST, *PGET_DEVICE_INTERNAL_STATUS_DATA_REQUEST;

typedef struct _DEVICE_INTERNAL_STATUS_DATA {

    // Size of this structure.
    ULONG Version;

    // Whole size of the structure and the associated data buffer.
    ULONG Size;

    ULONGLONG T10VendorId;

    ULONG DataSet1Length;
    ULONG DataSet2Length;
    ULONG DataSet3Length;
    ULONG DataSet4Length;

    UCHAR StatusDataVersion;
    UCHAR Reserved[3];
    UCHAR ReasonIdentifier[128];
    ULONG StatusDataLength;
    UCHAR StatusData[ANYSIZE_ARRAY];

} DEVICE_INTERNAL_STATUS_DATA, *PDEVICE_INTERNAL_STATUS_DATA;

//
// IOCTL_STORAGE_REINITIALIZE_MEDIA
//
// Input Buffer :
//      STORAGE_REINITIALIZE_MEDIA - Optional
// Output Buffer :
//      None
//

typedef enum _STORAGE_SANITIZE_METHOD {
    StorageSanitizeMethodDefault = 0,
    StorageSanitizeMethodBlockErase,
    StorageSanitizeMethodCryptoErase
} STORAGE_SANITIZE_METHOD, *PSTORAGE_SANITIZE_METHOD;

#pragma warning(push)
#pragma warning(disable:4214) // bit fields other than int to disable this around the struct

typedef struct _STORAGE_REINITIALIZE_MEDIA {

    ULONG Version;

    ULONG Size;

    ULONG TimeoutInSeconds;

    //
    // The SanitizeOption field is only applicable to NVMe devices.
    //
    struct {

        //
        // This field specifies the sanitize method defined in STORAGE_SANITIZE_METHOD enum.
        //
        ULONG SanitizeMethod : 4;

        //
        // This field specifies if unrestricted sanitize exit is allowed or not.
        // By default unrestricted sanitize exit is allowed.
        //
        ULONG DisallowUnrestrictedSanitizeExit : 1;

        ULONG Reserved : 27;
    } SanitizeOption;

} STORAGE_REINITIALIZE_MEDIA, *PSTORAGE_REINITIALIZE_MEDIA;

#pragma warning(pop)


#pragma warning(push)
#pragma warning(disable:4200)

#if defined(_MSC_EXTENSIONS)

typedef struct _STORAGE_MEDIA_SERIAL_NUMBER_DATA {

    USHORT Reserved;

    //
    // the SerialNumberLength will be set to zero
    // if the command is supported and the media
    // does not have a valid serial number.
    //

    USHORT SerialNumberLength;

    //
    // the following data is binary, and is not guaranteed
    // to be NULL terminated.  this is an excercise for the
    // caller.
    //

#if !defined(__midl)
    UCHAR SerialNumber[0];
#endif

} STORAGE_MEDIA_SERIAL_NUMBER_DATA, *PSTORAGE_MEDIA_SERIAL_NUMBER_DATA;

#endif /* _MSC_EXTENSIONS */

typedef _Struct_size_bytes_(Size) struct _STORAGE_READ_CAPACITY {

    //
    // The version number, size of the STORAGE_READ_CAPACITY structure
    //
    ULONG Version;

    //
    // The size of the date returned, size of the STORAGE_READ_CAPACITY structure
    //
    ULONG Size;

    //
    // Number of bytes per block
    //

    ULONG BlockLength;

    //
    // Total number of blocks in the disk
    // This will have the last LBA + 1
    //

    LARGE_INTEGER NumberOfBlocks;

    //
    // Disk size in bytes
    //

    LARGE_INTEGER DiskLength;

} STORAGE_READ_CAPACITY, *PSTORAGE_READ_CAPACITY;

#pragma warning(pop)

//
// Device write cache property
//
// This property provides the write cache information
// about the target device.
//

typedef enum _WRITE_CACHE_TYPE {
    WriteCacheTypeUnknown,
    WriteCacheTypeNone,
    WriteCacheTypeWriteBack,
    WriteCacheTypeWriteThrough
} WRITE_CACHE_TYPE;

typedef enum _WRITE_CACHE_ENABLE {
    WriteCacheEnableUnknown,
    WriteCacheDisabled,
    WriteCacheEnabled
} WRITE_CACHE_ENABLE;

typedef enum _WRITE_CACHE_CHANGE {
    WriteCacheChangeUnknown,
    WriteCacheNotChangeable,
    WriteCacheChangeable
} WRITE_CACHE_CHANGE;

typedef enum _WRITE_THROUGH {
    WriteThroughUnknown,
    WriteThroughNotSupported,
    WriteThroughSupported
} WRITE_THROUGH;

typedef _Struct_size_bytes_(Size) struct _STORAGE_WRITE_CACHE_PROPERTY {

    //
    // The version number
    // Size of STORAGE_WRITE_CACHE_PROPERTY structure
    //

    ULONG Version;

    //
    // The size of the date returned
    // Size of STORAGE_WRITE_CACHE_PROPERTY structure
    //

    ULONG Size;

    //
    // Current write cache type
    //

    WRITE_CACHE_TYPE WriteCacheType;

    //
    // Current write cache value
    //

    WRITE_CACHE_ENABLE WriteCacheEnabled;

    //
    // Device write cache change capability
    //

    WRITE_CACHE_CHANGE WriteCacheChangeable;

    //
    // Device write through support capability
    //

    WRITE_THROUGH WriteThroughSupported;

    //
    // Device flush cache capability
    //

    BOOLEAN FlushCacheSupported;

    //
    // User selected power protection option through registry
    //

    BOOLEAN UserDefinedPowerProtection;

    //
    // Device has battery backup for write cache
    //

    BOOLEAN NVCacheEnabled;

} STORAGE_WRITE_CACHE_PROPERTY, *PSTORAGE_WRITE_CACHE_PROPERTY;

#pragma warning(push)
#pragma warning(disable:4200) // array[0]
#pragma warning(disable:4201) // nameless struct/unions
#pragma warning(disable:4214) // bit fields other than int


#if defined(_MSC_EXTENSIONS)

typedef struct _PERSISTENT_RESERVE_COMMAND {

    ULONG Version;
    ULONG Size;

    union {

        struct {

            //
            // Persistent Reserve service action.
            //

            UCHAR ServiceAction : 5;
            UCHAR Reserved1 : 3;

            //
            // Number of bytes allocated for returned parameter list.
            //

            USHORT AllocationLength;

        } PR_IN;

        struct {

            //
            // Persistent Reserve service action.
            //

            UCHAR ServiceAction : 5;
            UCHAR Reserved1 : 3;

            //
            // Persistent Reserve type and scope.
            //

            UCHAR Type : 4;
            UCHAR Scope : 4;

            //
            // Space for additional PR Out parameters.
            //

#if !defined(__midl)
            UCHAR ParameterList[0];
#endif

        } PR_OUT;
    } DUMMYUNIONNAME;

} PERSISTENT_RESERVE_COMMAND, *PPERSISTENT_RESERVE_COMMAND;

#endif /* _MSC_EXTENSIONS */
#pragma warning(pop)

//
//  Device telemetry definitions
//
//  Structures and interfaces dealing with acquistion of device and driver internal telemetry.
//

// For variable size fields we use byte array, defined with zero length in structure template. Length of the field is stored as a separate field.
// No more than one variable size field is allowed in one structure and it is always placed last.

#pragma warning(push)


//
// Persistent data structures are versioned and "sized" by adding structure version field and structure size field
//
#define DEVICEDUMP_STRUCTURE_VERSION_V1         1

//
// Max size of the identification string
//
#define DEVICEDUMP_MAX_IDSTRING 32       // Keep proportional to sizeof (ULONG)
#define MAX_FW_BUCKET_ID_LENGTH 132     // 128 (ACS specification + 1 for zero termination + 3 to align on ULONG)


//
// Global telemetry collection parameters in the registry
//
#define STORAGE_CRASH_TELEMETRY_REGKEY  L"\\Registry\\Machine\\System\\CurrentControlSet\\Control\\CrashControl\\StorageTelemetry"
#define STORAGE_DEVICE_TELEMETRY_REGKEY L"\\Registry\\Machine\\System\\CurrentControlSet\\Control\\Storage\\StorageTelemetry"

//
// Reasons for telemetry collection
//
typedef enum _DEVICEDUMP_COLLECTION_TYPE {
    TCCollectionBugCheck = 1,                   // 1
    TCCollectionApplicationRequested,           // 2 (Host Initiated - HITC)
    TCCollectionDeviceRequested                 // 3 (Device initiated - DITC)
} DEVICEDUMP_COLLECTION_TYPEIDE_NOTIFICATION_TYPE, *PDEVICEDUMP_COLLECTION_TYPE;

//
// Flags for the device dump section
//
#define DDUMP_FLAG_DATA_READ_FROM_DEVICE        0x0001

//
// Firmware issue IDs (similar to bug check reasons)
//
#define FW_ISSUEID_NO_ISSUE     0x00000000
#define FW_ISSUEID_UNKNOWN      0xFFFFFFFF

//
#include <pshpack1.h>   // Structures are packed on a byte boundary , because parsers may run on separate machines and different OS flavors

//
//  Device dump section contains common device dump header, followed by set of relative pointers to sub sections
//  Each relative pointer contain size in bytes of the subsection and starting offset from the beginning of the section
//
// Layout of the device dump section is as
//
//   HEADER         - common descriptor
//   PUBLIC DATA    - publicly accessible data (eg SMART structures)
//   RESRICTED DATA - restricted access data (eg encrypted with Microsoft and IHV public keys)
//   PRIVATE DATA   - private device data
//
// All fields in the section definition are used cross platform => types used are platform neutral
//

//
// Relative descript    or of a subsection, contains size of the subsection and relative offset (0 is the start of the section)
//
typedef struct _DEVICEDUMP_SUBSECTION_POINTER {
        ULONG    dwSize;        // Size (in bytes) of the subsection
        ULONG    dwFlags;       // Parameter flags for the subsection
        ULONG    dwOffset;      // Offset (in bytes) of the subsection block from the start of the buffer
} DEVICEDUMP_SUBSECTION_POINTER,*PDEVICEDUMP_SUBSECTION_POINTER;

//
// Data structure tagging fields (version and size)
//
typedef struct _DEVICEDUMP_STRUCTURE_VERSION {
        //
        // Header signature, useful for identifying the structure when reading the dump
        //
        ULONG   dwSignature;

        //
        // Version of the template
        //
        ULONG   dwVersion;

        //
        // Size of the parent structure in bytes
        //
        ULONG   dwSize;

} DEVICEDUMP_STRUCTURE_VERSION, *PDEVICEDUMP_STRUCTURE_VERSION;

//
//  Device data header for the secondary data (in crashdump) section , holding device dump information.
//
typedef  struct _DEVICEDUMP_SECTION_HEADER {
        //
        // GUID, identifying device dump section. Same GUID as used in registering for SecondaryData callback, stored here for live telemetry interface consistency
        //
        GUID    guidDeviceDataId;

        //
        // Device identification fields.
        // These fields together should uniquely identify the device firmware image.

        //
        //  ID value, common for storage device vendors. This ID will be used by !analyze to create a vendor ID for WinQual.
        //
        //
        // Unique identifier assigned to the organization responsible for device quality (firmware quality). In most cases this is OUID (IEEE) or WorldWideName of the device vendor
        //
        UCHAR    sOrganizationID[16];

        //
        // Firmware revision as indicated in IDENITFY or INQUIRY structures
        //
        ULONG    dwFirmwareRevision;

        //
        // Device model number (keep the length of the field proportional to sizeof (ULONG))
        //
        UCHAR   sModelNumber[DEVICEDUMP_MAX_IDSTRING];

        //
        // Vendor specific device cookie, identifying process and manufacturing parameters. Opaque to the OS and applications.
        //
        UCHAR   szDeviceManufacturingID[DEVICEDUMP_MAX_IDSTRING];       // Keep the length of the field proportional to sizeof (ULONG)

        //
        // Sourcing indicator flag - used to detect if data was emulated from other structures or obtained directly from the firmware using log command
        //      Set to 1 if public data was filled in using data from the device telemetry log
        //      Set to 0 if the device doesn't support the command and the driver filled in as best it could
        //
        ULONG  dwFlags;

        //
        // Version of private data as indicated by the firmware.Initially always 0 to specify Private only unspecified data
        //
        ULONG bRestrictedPrivateDataVersion;

        //
        // Issue identifier (hash value) generated by the firmware. Reflects state of the device firmware and used for cross device type/vendor queries.
        // We will rely on standardized namespace of issue IDs and good will of firmware developers to taxonomize
        //
        ULONG dwFirmwareIssueId;                //currently unused

        //
        // Firmware bucket ID - long string, opague to Windows , but useful to create unique bucket in concatenation with device identification data
        //
        UCHAR    szIssueDescriptionString[MAX_FW_BUCKET_ID_LENGTH];      // zero terminated

} DEVICEDUMP_SECTION_HEADER, *PDEVICEDUMP_SECTION_HEADER;

//
//  Public subsection header - subsection is holding data, describing device state and accessible to everybody.
//

#define TC_PUBLIC_DEVICEDUMP_CONTENT_SMART      0x01
#define TC_PUBLIC_DEVICEDUMP_CONTENT_GPLOG      0x02

//
// Maximum number of log pages collected into the public section
#define TC_PUBLIC_DEVICEDUMP_CONTENT_GPLOG_MAX  16

// Maximum length of the description of the collected pages (filled by the miniport)
#define TC_DEVICEDUMP_SUBSECTION_DESC_LENGTH    16

//
// Standard types of collected pages
//
#define TC_PUBLIC_DATA_TYPE_ATAGP "ATAGPLogPages"
#define TC_PUBLIC_DATA_TYPE_ATASMART "ATASMARTPages"

//
// Public data is tagged with the table of "log descriptors". Each descriptor has LogAddress and number of pages.
// Specific meaning, assigned to the descriptor, is relative to the command set used.
//

typedef struct _GP_LOG_PAGE_DESCRIPTOR {
        USHORT  LogAddress;
        USHORT  LogSectors;
} GP_LOG_PAGE_DESCRIPTOR,*PGP_LOG_PAGE_DESCRIPTOR;

typedef struct _DEVICEDUMP_PUBLIC_SUBSECTION {
        ULONG   dwFlags;
        GP_LOG_PAGE_DESCRIPTOR  GPLogTable[TC_PUBLIC_DEVICEDUMP_CONTENT_GPLOG_MAX];
        CHAR    szDescription[TC_DEVICEDUMP_SUBSECTION_DESC_LENGTH]; // Zero terminated
        UCHAR   bData[ANYSIZE_ARRAY];      // Data byte array ANYSIZE_ARRAY
} DEVICEDUMP_PUBLIC_SUBSECTION, *PDEVICEDUMP_PUBLIC_SUBSECTION;

//
//  Restricted subsection header - subsection is holding data, describing device state and accessible only to Microsoft and a device vendor
//
typedef struct _DEVICEDUMP_RESTRICTED_SUBSECTION {

        UCHAR   bData[ANYSIZE_ARRAY];           // Data byte array (ANYSIZE_ARRAY)

} DEVICEDUMP_RESTRICTED_SUBSECTION, *PDEVICEDUMP_RESTRICTED_SUBSECTION;

//
//  Private subsection header - subsection is holding data, describing device state and accessible only to a device vendor
//
typedef struct _DEVICEDUMP_PRIVATE_SUBSECTION {

        ULONG   dwFlags;
        GP_LOG_PAGE_DESCRIPTOR GPLogId;

        UCHAR   bData[ANYSIZE_ARRAY];   // Data byte array (ANYSIZE_ARRAY)

} DEVICEDUMP_PRIVATE_SUBSECTION, *PDEVICEDUMP_PRIVATE_SUBSECTION;

//
// Descriptor of the storage device dump section
//
typedef _Struct_size_bytes_(Descriptor.dwSize) struct _DEVICEDUMP_STORAGEDEVICE_DATA {

        //
        // Common descriptor (signature,version of the structure)
        //
        DEVICEDUMP_STRUCTURE_VERSION    Descriptor;

        //
        // Header - set of fields, describing dump section and device (not requiring protocol communication)
        //
        DEVICEDUMP_SECTION_HEADER SectionHeader;

        //
        // Size of the whole section buffer, in bytes , including header and sum total of all the variable sized sub sections
        //
        ULONG   dwBufferSize;

        //
        // Reason for collecting telemetry
        //
        ULONG   dwReasonForCollection;

        //
        // "Pointers" to individual sub-sections. Sub sections are filled with the information, obtained from the device
        //
        DEVICEDUMP_SUBSECTION_POINTER PublicData;
        DEVICEDUMP_SUBSECTION_POINTER RestrictedData;
        DEVICEDUMP_SUBSECTION_POINTER PrivateData;

} DEVICEDUMP_STORAGEDEVICE_DATA, *PDEVICEDUMP_STORAGEDEVICE_DATA;


//
// Driver dump section contains common device driver context information:
//      - circular buffer of the IO requests as visible by the lower edge of the driver
//              (in case of storage that would be an interface to the controller stack or controller itself)
//
// All fields in the section definition are used cross platform => types used are platform neutral
//

//
// Format of the single record for publicly accessible driver state table
//

#define         CDB_SIZE                  16
#define         TELEMETRY_COMMAND_SIZE    16

#define         TCRecordStorportSrbFunction       Command[0]

typedef struct _DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD {
        // The CDB for this request. variable sized CDBs are truncated.
        UCHAR     Cdb[CDB_SIZE];

        // The actual command for this request.
        UCHAR     Command[TELEMETRY_COMMAND_SIZE];

        // the time when driver received the request
        ULONGLONG StartTime;

        // the system time when the request was completed
        ULONGLONG EndTime;

        // Status value ()
        ULONG  OperationStatus;

        // Error value (eg error reg for ATAPort, SCSI error for storport)
        ULONG   OperationError;

        // Stack specific information
        union {
         struct {
                ULONG dwReserved;
                } ExternalStack;

         struct {
                ULONG   dwAtaPortSpecific;
         }  AtaPort;

         struct {
                ULONG   SrbTag  ;
         }  StorPort;

        } StackSpecific;

} DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD,*PDEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD;


typedef _Struct_size_bytes_(Descriptor.dwSize) struct _DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP {

        //
        // Common descriptor (signature,version of the structure)
        //
        DEVICEDUMP_STRUCTURE_VERSION    Descriptor;

        //
        // Reason for collecting telemetry
        //
        ULONG   dwReasonForCollection;

        //
        // Driver stack and instance
        //
        UCHAR   cDriverName[16];

        //
        // Standardized log of IO requests issued to the target, starting with number of records.
        // Log is circular, order is not guaranteed
        //
        ULONG   uiNumRecords;

        DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD RecordArray[ANYSIZE_ARRAY]; //ANYSIZE_ARRAY

} DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP,*PDEVICEDUMP_STORAGESTACK_PUBLIC_DUMP;

// End of the packed structure group
#include <poppack.h>


//
// Telemetry information block descriptor - bit flags in DumpCapabilities field
//

#define DEVICEDUMP_CAP_PRIVATE_SECTION          0x00000001      // Target supports private data
#define DEVICEDUMP_CAP_RESTRICTED_SECTION       0x00000002      // Target supports restricted data



#pragma warning(push)
#pragma warning(disable:4214) // bit fields other than int

//
// IOCTL_STORAGE_ENABLE_IDLE_POWER
//
// Input Buffer:
//      A STORAGE_IDLE_POWER structure specifying the idle power management
//      properties of the device.
//
// Output Buffer:
//      None.
//
typedef struct _STORAGE_IDLE_POWER {
    ULONG Version;              // Structure version, should be set to 1 for Win8.
    ULONG Size;                 // Size of this structure in bytes.
    ULONG WakeCapableHint : 1;  // Storage device supports wake from low power states.
    ULONG D3ColdSupported : 1;  // Storage device supports D3Cold
    ULONG Reserved : 30;
    ULONG D3IdleTimeout;        // Idle time in msec before storage device is transitioned to D3 (max of ~49.7 days).
} STORAGE_IDLE_POWER, *PSTORAGE_IDLE_POWER;

#pragma warning(pop)

//
// IOCTL_STORAGE_GET_IDLE_POWERUP_REASON
//
// Input Buffer:
//      None.
//
// Output Buffer:
//      A STORAGE_IDLE_POWERUP_REASON structure specifying what caused the power up.
//
typedef enum _STORAGE_POWERUP_REASON_TYPE {
  StoragePowerupUnknown           = 0,
  StoragePowerupIO,
  StoragePowerupDeviceAttention
} STORAGE_POWERUP_REASON_TYPE, *PSTORAGE_POWERUP_REASON_TYPE;

typedef struct _STORAGE_IDLE_POWERUP_REASON {
    ULONG Version;                          // Structure version, should be set to 1 for Win8.
    ULONG Size;                             // Size of this structure in bytes.
    STORAGE_POWERUP_REASON_TYPE PowerupReason;   // The reason for the power up (see above).
} STORAGE_IDLE_POWERUP_REASON, *PSTORAGE_IDLE_POWERUP_REASON;

#define STORAGE_IDLE_POWERUP_REASON_VERSION_V1 1

//
// IOCTL_STORAGE_DEVICE_POWER_CAP
//
// This IOCTL specifies a maximum *operational* power consumption level for a
// storage device.
// The storage stack will do its best to transition the device to a power state
// that will not exceed the given maximum.  However, this depends on what the
// device supports.  The actual maximum may be less than or greater than the
// desired maximum.
//
// Input buffer:
//  A STORAGE_DEVICE_POWER_CAP structure.
//  * The Units field specifies the units of the MaxPower field.  It can be
//    either a percentage (0-100%) or an absolute value in milliwatts.
//  * The MaxPower field is used to set the desired maximum power consumption
//    value for the storage device.
//
// Output buffer:
//  On success, the output buffer will contain a STORAGE_DEVICE_POWER_CAP
//  structure.
//  * The Units field will continue to specify the units of the MaxPower field
//    and will match the value from the input buffer.
//  * The MaxPower field will contain the value of the actual maximum
//    power consumption level of the device.  This may be equal to, less than,
//    or greater than the desired cap, depending on what the device supports.
//
typedef enum _STORAGE_DEVICE_POWER_CAP_UNITS {
    StorageDevicePowerCapUnitsPercent,
    StorageDevicePowerCapUnitsMilliwatts
} STORAGE_DEVICE_POWER_CAP_UNITS, *PSTORAGE_DEVICE_POWER_CAP_UNITS;

typedef struct _STORAGE_DEVICE_POWER_CAP {
    ULONG Version;
    ULONG Size;
    STORAGE_DEVICE_POWER_CAP_UNITS Units;
    ULONGLONG MaxPower;
} STORAGE_DEVICE_POWER_CAP, *PSTORAGE_DEVICE_POWER_CAP;

#define STORAGE_DEVICE_POWER_CAP_VERSION_V1 1

//
// IOCTL_STORAGE_RPMB_COMMAND
//
// This IOCTL sends an RPMB command to the underlying storage device.
//
// Input buffer:
//  An array of STORAGE_RPMB_DATA_FRAME structures
//  * The number of frames included can be calculated by InputBufferLength / sizeof(STORAGE_RPMB_DATA_FRAME)
//
// Output buffer:
//  An array of STORAGE_RPMB_DATA_FRAME structures
//  * The number of frames included can be calculated by OutputBufferLength / sizeof(STORAGE_RPMB_DATA_FRAME)
//

// Ensure we are byte aligned
#pragma pack(push)
#pragma pack(1)

//
// This is the RPMB data frame used to compose all RPMB requests and responses.
//
// This corresponds to StorageRpmbFrameTypeStandard
//

typedef struct _STORAGE_RPMB_DATA_FRAME {

    //
    // Reserved
    //
    UCHAR Stuff[196];

    //
    // Either the key to be programmed or the MAC authenticating this frame or series of frames
    //
    UCHAR KeyOrMAC[32];

    //
    // The data input or output
    //
    UCHAR Data[256];

    //
    // Random 128-bit number generated by host
    //
    UCHAR Nonce[16];

    //
    // 32-bit counter
    //
    UCHAR WriteCounter[4];

    //
    // The half-sector address to operate on
    //
    UCHAR Address[2];

    //
    // The count of half-sector blocks to read/write
    //
    UCHAR BlockCount[2];

    //
    // The result of the operation
    //
    UCHAR OperationResult[2];

    //
    // The type of request or response
    //
    UCHAR RequestOrResponseType[2];

} STORAGE_RPMB_DATA_FRAME, *PSTORAGE_RPMB_DATA_FRAME;

//
// RPMB RequestOrResponseType Values
//

typedef enum _STORAGE_RPMB_COMMAND_TYPE {
    StorRpmbProgramAuthKey                 = 0x00000001,
    StorRpmbQueryWriteCounter              = 0x00000002,
    StorRpmbAuthenticatedWrite             = 0x00000003,
    StorRpmbAuthenticatedRead              = 0x00000004,
    StorRpmbReadResultRequest              = 0x00000005,
    StorRpmbAuthenticatedDeviceConfigWrite = 0x00000006,
    StorRpmbAuthenticatedDeviceConfigRead  = 0x00000007,
} STORAGE_RPMB_COMMAND_TYPE, *PSTORAGE_RPMB_COMMAND_TYPE;

#pragma pack(pop)

//
// IOCTL_STORAGE_EVENT_NOTIFICATION
//
// Input Buffer:
//      A STORAGE_EVENT_NOTIFICATION structure specifying the event(s) that occurred.
//
// Output Buffer:
//      None
//
typedef struct _STORAGE_EVENT_NOTIFICATION {
    ULONG Version;                          // Structure version, should be set to 1 for Win8.
    ULONG Size;                             // Size of this structure in bytes.
    ULONGLONG Events;                       // Bitmask of event(s) that occurred.
} STORAGE_EVENT_NOTIFICATION, *PSTORAGE_EVENT_NOTIFICATION;

#define STORAGE_EVENT_NOTIFICATION_VERSION_V1 1

#define STORAGE_EVENT_MEDIA_STATUS        0x0000000000000001
#define STORAGE_EVENT_DEVICE_STATUS       0x0000000000000002
#define STORAGE_EVENT_DEVICE_OPERATION    0x0000000000000004

#define STORAGE_EVENT_ALL (STORAGE_EVENT_MEDIA_STATUS | STORAGE_EVENT_DEVICE_STATUS | STORAGE_EVENT_DEVICE_OPERATION)

#pragma warning(pop)


#define READ_COPY_NUMBER_KEY                    0x52434e00  // 'RCN'
#define READ_COPY_NUMBER_BYPASS_CACHE_FLAG      0x00000100

#define IsKeyReadCopyNumber(_k)                 (((_k) & 0xFFFFFE00) == READ_COPY_NUMBER_KEY)

#define IsKeyReadCopyNumberBypassCache(_k)      ((_k) & READ_COPY_NUMBER_BYPASS_CACHE_FLAG)
#define SetReadCopyNumberBypassCacheToKey(_k)   ((_k) |= READ_COPY_NUMBER_BYPASS_CACHE_FLAG)

#define ReadCopyNumberToKey(_c)                 (READ_COPY_NUMBER_KEY | (UCHAR)(_c))
#define ReadCopyNumberFromKey(_k)               (UCHAR)((_k) & 0x000000FF)


//
// IOCTL_STORAGE_GET_COUNTERS
//
// This IOCTL retrieves reliability counters for a storage device.
//
// The caller can find out the required output buffer size by simply sending
// down a single STORAGE_COUNTERS structure. STATUS_BUFFER_OVERFLOW will be
// returned and the Size field in the STORAGE_COUNTERS structure will contain
// the total size of the required output buffer.
//
// When the output buffer is sufficiently large, STATUS_SUCCESS will be
// returned and the output buffer will contain a STORAGE_COUNTERS structure
// followed by an array of STORAGE_COUNTER structures.
//
// Input Buffer:
//      STORAGE_COUNTERS with the Version field set appropriately.
//
// Output Buffer:
//      When successful, a STORAGE_COUNTERS structure with the Counters array
//      filled out.  NumberOfCounters will indicate the number of elements in
//      the Counters array.
//      See above for more details.
//

typedef enum _STORAGE_COUNTER_TYPE {

    StorageCounterTypeUnknown = 0,

    StorageCounterTypeTemperatureCelsius,
    StorageCounterTypeTemperatureCelsiusMax,
    StorageCounterTypeReadErrorsTotal,
    StorageCounterTypeReadErrorsCorrected,
    StorageCounterTypeReadErrorsUncorrected,
    StorageCounterTypeWriteErrorsTotal,
    StorageCounterTypeWriteErrorsCorrected,
    StorageCounterTypeWriteErrorsUncorrected,
    StorageCounterTypeManufactureDate,
    StorageCounterTypeStartStopCycleCount,
    StorageCounterTypeStartStopCycleCountMax,
    StorageCounterTypeLoadUnloadCycleCount,
    StorageCounterTypeLoadUnloadCycleCountMax,
    StorageCounterTypeWearPercentage,
    StorageCounterTypeWearPercentageWarning,
    StorageCounterTypeWearPercentageMax,
    StorageCounterTypePowerOnHours,
    StorageCounterTypeReadLatency100NSMax,
    StorageCounterTypeWriteLatency100NSMax,
    StorageCounterTypeFlushLatency100NSMax,

    StorageCounterTypeMax

} STORAGE_COUNTER_TYPE, *PSTORAGE_COUNTER_TYPE;

typedef struct _STORAGE_COUNTER {

    STORAGE_COUNTER_TYPE Type;

    union {

        struct {
            //
            // Week is the number of the week in the year, 1-52.
            //
            ULONG Week;

            //
            // Year is the last two digits of the year, e.g. 2016 is simply "16".
            //
            ULONG Year;
        } ManufactureDate;

        ULONGLONG AsUlonglong;
    } Value;

} STORAGE_COUNTER, *PSTORAGE_COUNTER;

typedef _Struct_size_bytes_(Size) struct _STORAGE_COUNTERS {

    //
    // Size of this structure serves as the version.
    //
    ULONG Version;

    //
    // Total size of this structure plus all the variable-sized fields.
    //
    ULONG Size;

    ULONG NumberOfCounters;

    _Field_size_(NumberOfCounters) STORAGE_COUNTER Counters[ANYSIZE_ARRAY];

} STORAGE_COUNTERS, *PSTORAGE_COUNTERS;

#define STORAGE_COUNTERS_VERSION_V1 sizeof(STORAGE_COUNTERS)

//
// Parameter and data structure for firmware upgrade IOCTLs
// IOCTL_STORAGE_FIRMWARE_GET_INFO, IOCTL_STORAGE_FIRMWARE_DOWNLOAD, IOCTL_STORAGE_FIRMWARE_ACTIVATE
//

//
// Indicate the target of the request other than the device handle/object itself.
// This is used in "Flags" field of data structures for firmware upgrade request.
//
#define STORAGE_HW_FIRMWARE_REQUEST_FLAG_CONTROLLER                     0x00000001

//
// Indicate that current FW image segment is the last one.
//
#define STORAGE_HW_FIRMWARE_REQUEST_FLAG_LAST_SEGMENT                   0x00000002

//
// Indicate that current FW image segment is the first one.
//
#define STORAGE_HW_FIRMWARE_REQUEST_FLAG_FIRST_SEGMENT                  0x00000004

//
// Indicate that the existing firmware in slot should be activated immediately without
// controller reset. Only valid for IOCTL_STORAGE_FIRMWARE_ACTIVATE.
//
#define STORAGE_HW_FIRMWARE_REQUEST_FLAG_SWITCH_TO_FIRMWARE_WITHOUT_RESET   0x10000000

//
// Indicate that any existing firmware in slot should be replaced with the downloaded image,
// and activated with controller reset. Only valid for IOCTL_STORAGE_FIRMWARE_ACTIVATE.
//
#define STORAGE_HW_FIRMWARE_REQUEST_FLAG_REPLACE_AND_SWITCH_UPON_RESET      0x20000000

//
// Indicate that any existing firmware in slot should be replaced with the downloaded image.
// Only valid for IOCTL_STORAGE_FIRMWARE_ACTIVATE.
//
#define STORAGE_HW_FIRMWARE_REQUEST_FLAG_REPLACE_EXISTING_IMAGE             0x40000000

//
// Indicate that the existing firmware in slot should be activated with a controller reset.
// Only valid for IOCTL_STORAGE_FIRMWARE_ACTIVATE.
//
#define STORAGE_HW_FIRMWARE_REQUEST_FLAG_SWITCH_TO_EXISTING_FIRMWARE        0x80000000

//
// Input parameter for IOCTL_STORAGE_FIRMWARE_GET_INFO
//
typedef struct _STORAGE_HW_FIRMWARE_INFO_QUERY {
    ULONG   Version;            // sizeof(STORAGE_FIRMWARE_INFO_QUERY)
    ULONG   Size;               // Whole size of the buffer (in case this data structure being extended to be variable length)
    ULONG   Flags;
    ULONG   Reserved;
} STORAGE_HW_FIRMWARE_INFO_QUERY, *PSTORAGE_HW_FIRMWARE_INFO_QUERY;

//
// Output parameter for IOCTL_STORAGE_FIRMWARE_GET_INFO
// The total size of returned data is for Firmware Info is:
//   sizeof(STORAGE_HW_FIRMWARE_INFO) + sizeof(STORAGE_HW_FIRMWARE_SLOT_INFO) * (SlotCount - 1).
// If the buffer is not big enough, callee should set the required length in "Size" field of STORAGE_HW_FIRMWARE_INFO,
//

//
// Following value maybe used in "PendingActiveSlot" field indicating there is no firmware pending to activate.
//
#define STORAGE_HW_FIRMWARE_INVALID_SLOT              0xFF

#pragma warning(push)
#pragma warning(disable:4214) // bit fields other than int

#define STORAGE_HW_FIRMWARE_REVISION_LENGTH             16

typedef struct _STORAGE_HW_FIRMWARE_SLOT_INFO {

    ULONG   Version;            // sizeof(STORAGE_HW_FIRMWARE_SLOT_INFO)

    ULONG   Size;               // size the data contained in STORAGE_HW_FIRMWARE_SLOT_INFO.

    UCHAR   SlotNumber;

    UCHAR   ReadOnly : 1;

    UCHAR   Reserved0 : 7;

    UCHAR   Reserved1[6];

    UCHAR   Revision[STORAGE_HW_FIRMWARE_REVISION_LENGTH];

} STORAGE_HW_FIRMWARE_SLOT_INFO, *PSTORAGE_HW_FIRMWARE_SLOT_INFO;

typedef struct _STORAGE_HW_FIRMWARE_INFO {

    ULONG   Version;            // sizeof(STORAGE_HW_FIRMWARE_INFO)

    ULONG   Size;               // size of the whole buffer including slot[]

    UCHAR   SupportUpgrade : 1;

    UCHAR   Reserved0 : 7;

    UCHAR   SlotCount;

    UCHAR   ActiveSlot;

    UCHAR   PendingActivateSlot;

    BOOLEAN FirmwareShared;     // The firmware applies to both device and adapter. For example: PCIe SSD.

    UCHAR   Reserved[3];

    ULONG   ImagePayloadAlignment;  // Number of bytes. Max: PAGE_SIZE. The transfer size should be multiple of this unit size. Some protocol requires at least sector size. 0 means the value is not valid.

    ULONG   ImagePayloadMaxSize;    // for a single command.

    STORAGE_HW_FIRMWARE_SLOT_INFO Slot[ANYSIZE_ARRAY];

} STORAGE_HW_FIRMWARE_INFO, *PSTORAGE_HW_FIRMWARE_INFO;
#pragma warning(pop)


//
// Input parameter for IOCTL_STORAGE_FIRMWARE_DOWNLOAD
//
#pragma warning(push)
#pragma warning(disable:4200)

typedef struct _STORAGE_HW_FIRMWARE_DOWNLOAD {

    ULONG       Version;            // sizeof(STORAGE_HW_FIRMWARE_DOWNLOAD)
    ULONG       Size;               // size of the whole buffer include "ImageBuffer"

    ULONG       Flags;
    UCHAR       Slot;               // Slot number that firmware image will be downloaded into.
    UCHAR       Reserved[3];

    ULONGLONG   Offset;             // Image file offset, should be aligned to "ImagePayloadAlignment" value from STORAGE_FIRMWARE_INFO.
    ULONGLONG   BufferSize;         // should be multiple of "ImagePayloadAlignment" value from STORAGE_FIRMWARE_INFO.

    UCHAR       ImageBuffer[ANYSIZE_ARRAY];     // firmware image file.

} STORAGE_HW_FIRMWARE_DOWNLOAD, *PSTORAGE_HW_FIRMWARE_DOWNLOAD;

typedef struct _STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {

    ULONG       Version;            // sizeof(STORAGE_HW_FIRMWARE_DOWNLOAD_V2)
    ULONG       Size;               // size of the whole buffer include "ImageBuffer"

    ULONG       Flags;
    UCHAR       Slot;               // Slot number that firmware image will be downloaded into.
    UCHAR       Reserved[3];

    ULONGLONG   Offset;             // Image file offset, should be aligned to "ImagePayloadAlignment" value from STORAGE_FIRMWARE_INFO.
    ULONGLONG   BufferSize;         // should be multiple of "ImagePayloadAlignment" value from STORAGE_FIRMWARE_INFO.

    ULONG       ImageSize;          // Firmware Image size.
    ULONG       Reserved2;

    UCHAR       ImageBuffer[ANYSIZE_ARRAY];     // firmware image file.

} STORAGE_HW_FIRMWARE_DOWNLOAD_V2, *PSTORAGE_HW_FIRMWARE_DOWNLOAD_V2;

#pragma warning(pop)

//
// Input parameter for IOCTL_STORAGE_FIRMWARE_ACTIVATE
//
typedef struct _STORAGE_HW_FIRMWARE_ACTIVATE {

    ULONG   Version;
    ULONG   Size;

    ULONG   Flags;
    UCHAR   Slot;                   // Slot with firmware image to be activated.
    UCHAR   Reserved0[3];

} STORAGE_HW_FIRMWARE_ACTIVATE, *PSTORAGE_HW_FIRMWARE_ACTIVATE;

//
// Parameter for IOCTL_STORAGE_PROTOCOL_COMMAND
// Buffer layout: <STORAGE_PROTOCOL_COMMAND> <Command> [Error Info Buffer] [Data-to-Device Buffer] [Data-from-Device Buffer]
//
#define STORAGE_PROTOCOL_STRUCTURE_VERSION              0x1

typedef struct _STORAGE_PROTOCOL_COMMAND {

    ULONG   Version;                        // STORAGE_PROTOCOL_STRUCTURE_VERSION
    ULONG   Length;                         // sizeof(STORAGE_PROTOCOL_COMMAND)

    STORAGE_PROTOCOL_TYPE  ProtocolType;
    ULONG   Flags;                          // Flags for the request

    ULONG   ReturnStatus;                   // return value
    ULONG   ErrorCode;                      // return value, optional

    ULONG   CommandLength;                  // non-zero value should be set by caller
    ULONG   ErrorInfoLength;                // optional, can be zero
    ULONG   DataToDeviceTransferLength;     // optional, can be zero. Used by WRITE type of request.
    ULONG   DataFromDeviceTransferLength;   // optional, can be zero. Used by READ type of request.

    ULONG   TimeOutValue;                   // in unit of seconds

    ULONG   ErrorInfoOffset;                // offsets need to be pointer aligned
    ULONG   DataToDeviceBufferOffset;       // offsets need to be pointer aligned
    ULONG   DataFromDeviceBufferOffset;     // offsets need to be pointer aligned

    ULONG   CommandSpecific;                // optional information passed along with Command.
    ULONG   Reserved0;

    ULONG   FixedProtocolReturnData;        // return data, optional. Some protocol, such as NVMe, may return a small amount data (DWORD0 from completion queue entry) without the need of separate device data transfer.
    ULONG   FixedProtocolReturnData2;       // return data2, optional. Some protocol, such as NVMe, may return a small amount data (DWORD1 from completion queue entry) without the need of separate device data transfer.
    ULONG   Reserved1[2];

    _Field_size_bytes_full_(CommandLength) UCHAR Command[ANYSIZE_ARRAY];

} STORAGE_PROTOCOL_COMMAND, *PSTORAGE_PROTOCOL_COMMAND;

//
// Bit-mask values for STORAGE_PROTOCOL_COMMAND - "Flags" field.
//
#define STORAGE_PROTOCOL_COMMAND_FLAG_ADAPTER_REQUEST    0x80000000     // Flag indicates the request targeting to adapter instead of device.

//
// Status values for STORAGE_PROTOCOL_COMMAND - "ReturnStatus" field.
//
#define STORAGE_PROTOCOL_STATUS_PENDING                 0x0
#define STORAGE_PROTOCOL_STATUS_SUCCESS                 0x1
#define STORAGE_PROTOCOL_STATUS_ERROR                   0x2
#define STORAGE_PROTOCOL_STATUS_INVALID_REQUEST         0x3
#define STORAGE_PROTOCOL_STATUS_NO_DEVICE               0x4
#define STORAGE_PROTOCOL_STATUS_BUSY                    0x5
#define STORAGE_PROTOCOL_STATUS_DATA_OVERRUN            0x6
#define STORAGE_PROTOCOL_STATUS_INSUFFICIENT_RESOURCES  0x7
#define STORAGE_PROTOCOL_STATUS_THROTTLED_REQUEST       0x8

#define STORAGE_PROTOCOL_STATUS_NOT_SUPPORTED           0xFF

//
// Command Length for Storage Protocols.
//
#define STORAGE_PROTOCOL_COMMAND_LENGTH_NVME            0x40            // NVMe commands are always 64 bytes.

//
// Command Specific Information for Storage Protocols - "CommandSpecific" field.
//
#define STORAGE_PROTOCOL_SPECIFIC_NVME_ADMIN_COMMAND    0x01
#define STORAGE_PROTOCOL_SPECIFIC_NVME_NVM_COMMAND      0x02

//
// Additional notes when STORAGE_PROTOCOL_TYPE is ProtocolTypeNvme:
//  1.  When flag STORAGE_PROTOCOL_COMMAND_FLAG_ADAPTER_REQUEST is set, or the request is sent through adapter, namespace Id from "Command" field is used;
//      otherwise, the underneath driver should determine namespace Id from the device that receives the command.
//  2.  When a command fails, the "ErrorCode" field contains value from NVMe Completion Queue Entry - DW3 - Status Field.
//  3.  "CommandLength" field must have value of 64. i.e. STORAGE_PROTOCOL_COMMAND_LENGTH_NVME.
//  4.  "CommandSpecific" field must have value of either STORAGE_PROTOCOL_SPECIFIC_NVME_ADMIN_COMMAND, or STORAGE_PROTOCOL_SPECIFIC_NVME_NVM_COMMAND.
//  5.  When a command succeeds, field "FixedProtocolReturnData" may contain value from NVMe Completion Queue Entry - DW0.
//

//
// IOCTL_STORAGE_ATTRIBUTE_MANAGEMENT
//
// This IOCTL manages an attribute
// for a storage device.
//
// When a driver receives this IOCTL it should first
// let any lower drivers process the IOCTL.
// The driver can override the completion status
// from the lower driver layers, if needed.
// For example when driver can provide
// the required functionality after
// doing "Action" on "Attribute", without any support
// from lower drivers, it can choose to ignore failure
// from lower driver layers, complete the "Action"
// and return success.
//
// Input Buffer:
//      STORAGE_ATTRIBUTE_MGMT
//
// Output Buffer:
//      None.
//

//
// Valid Management Actions allowed
// on the Attribute
//
typedef enum _STORAGE_ATTRIBUTE_MGMT_ACTION {
    StorAttributeMgmt_ClearAttribute = 0,
    StorAttributeMgmt_SetAttribute   = 1,
    StorAttributeMgmt_ResetAttribute = 2
} STORAGE_ATTRIBUTE_MGMT_ACTION, *PSTORAGE_ATTRIBUTE_MGMT_ACTION;

//
// Valid Storage Device Attributes
//

//
// Reserved for future usage.
//
#define STORATTRIBUTE_NONE    0

//
// When this attribute is reset, a driver reverts to its
// default state. The definition of default state is specific
// to each individual driver.
//
// Supported actions: Reset.
//
#define STORATTRIBUTE_MANAGEMENT_STATE 1

typedef struct _STORAGE_ATTRIBUTE_MGMT {

    //
    // Size of this structure serves
    // as the version.
    //
    ULONG Version;

    //
    // Size of  this structure  plus
    // all the variable sized fields.
    //
    ULONG Size;

    //
    // Indicates what action is requested.
    //
    STORAGE_ATTRIBUTE_MGMT_ACTION Action;

    //
    // The attribute on which specified "Action"
    // needs to be taken.
    //
    ULONG Attribute;

} STORAGE_ATTRIBUTE_MGMT, *PSTORAGE_ATTRIBUTE_MGMT;

// end_winioctl

#if (NTDDI_VERSION >= NTDDI_WIN10_CO)

//
// Defines the bit for each supported features in storage drivers
// The supported features is defined in "StorageSupportedFeatures" as a DWORD value under
// "Parameters" registry key of the driver
//

//
// Bypass IO feature. If set, the driver understands bypass IO and supports bypass IO related IOCTL(s).
//
#define STORAGE_SUPPORTED_FEATURES_BYPASS_IO    0x00000001


//
// Supported Features Mask. This is "OR" of all defined bits of supported features.
//
#define STORAGE_SUPPORTED_FEATURES_MASK         (STORAGE_SUPPORTED_FEATURES_BYPASS_IO)

#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/unions

//
//          ======= IOCTL_STORAGE_MANAGE_BYPASS_IO ======
//

//
//  This IOCTL is used to control BypassIO operations in all layers of the
//  volume and storage stacks
//

//
//  Defines the various operations supported by the BypassIO IOCTL
//

typedef enum _BPIO_OPERATIONS {

    //
    //  This is a request to enable BypassIO for the given volume/disk (device)
    //  which means a driver may not see all reads/writes for that stack.
    //
    //  On the pre-operation, if a driver can support BypassIO for the given
    //  device it should forward the request down the stack.
    //
    //  On the pre-operation if a driver CAN NOT support BypassIO for the given
    //  device they should complete the FSCTL with STATUS_SUCCESS and update the
    //  BPIO_OUTPUT structure with appropriate OpStatus, FailingDriverNames,
    //  and FailureReason reasons.
    //
    //  During the post-operation they can see if all drivers below them are
    //  capable of supporting BypassIO.  If yes, the driver should preserve any
    //  needed state for the file and continue completion processing. It is the
    //  drivers responsibility to maintain state to properly handle requests that
    //  may not be compatible with the BypassIO enabled state.
    //
    //  During the post-operation processing if a driver determines they can no
    //  longer support BypassIO, they can call <TBD> to inform the stack below them
    //  that BypassIO is now disabled.  They should set appropriate state as to
    //  why it can not be supported.
    //

    BPIO_OP_ENABLE = 1,

    //
    //  This is a request to disable BypassIO for the given volume/disk.  It allows
    //  a  driver to cleanup any associated BypassIO state.
    //
    //  This can be received by a driver that currently does not have BypassIO
    //  enabled.  It should be ignored.
    //
    //  This operation should not be failed.
    //

    BPIO_OP_DISABLE = 2,

    //
    //  This is an informational request to see if BypassIO can be enabled for
    //  the given volume/disk.  This should be processed the same as an ENABLE operation
    //  with the appropriate fields in the BPIO_OUTPUT structure filled out.  The
    //  only difference is that the driver does not enter the ENABLE state.
    //

    BPIO_OP_QUERY = 3

} BPIO_OPERATIONS;


//
//  Defines the BypassIO INPUT flags
//

typedef enum _BPIO_INFLAGS {

    BPIO_INFL_NONE = 0

} BPIO_INFLAGS;
DEFINE_ENUM_FLAG_OPERATORS( BPIO_INFLAGS )

//
//  Defines the INPUT structure for IOCTL_STORAGE_MANAGE_BYPASS_IO
//

typedef struct _BPIO_INPUT {

    //
    // Size of this structure serves
    // as the version.
    //
    ULONG Version;

    //
    // Size of this structure plus
    // all the variable sized fields.
    //
    ULONG Size;

    //
    //  The BypassIO operation being requested.
    //

    BPIO_OPERATIONS Operation;

    //
    //  Input flags for this operation.
    //

    BPIO_INFLAGS InFlags;

    ULONGLONG Reserved2;

} BPIO_INPUT, *PBPIO_INPUT;


//
//  Defines the BypassIO OUTPUT flags
//

typedef enum _BPIO_OUTFLAGS {

    BPIO_OUTFL_NONE = 0

} BPIO_OUTFLAGS;
DEFINE_ENUM_FLAG_OPERATORS( BPIO_OUTFLAGS )

//
//  This structure defines operation specific outputs for both the ENABLE
//  and QUERY operations
//

typedef struct _BPIO_RESULTS {

    //
    //  A status code that will be available to users which identifies WHY
    //  the specified driver can't support BypassIO.  This should only be
    //  set by the first driver to fail the request.
    //

    LONG OpStatus;      // This should be NTSTATUS but it is not always available

    //
    //  Receives the name of the driver that failed the request.  For diagnostic
    //  reasons it is required that drivers store their name when failing an
    //  ENABLE/QUERY requests.  The name should match the actual name of the driver
    //  used by the system with extension.  Ex: "ntfs.sys".
    //
    //  FailingDriversNameLen contains the length of the string in CHARACTERS.
    //  No one should assume the string is NULL terminated.
    //

    USHORT FailingDriverNameLen;
    WCHAR FailingDriverName[32];

    //
    //  Receives a reasonable description of why the driver can not support the
    //  ENABLE/QUERY request.  This string is used for diagnostic reasons and should
    //  identify the WHY.
    //
    //  This string should be in English and does not need to be localized.
    //
    //  FailureReasonLen contains the length of the string in CHARACTERS.
    //  No one should assume the strings is NULL terminated.
    //

    USHORT FailureReasonLen;
    WCHAR FailureReason[128];

} BPIO_RESULTS, *PBPIO_RESULTS;

//
//  Defines the variable sized OUTPUT structure for IOCTL_STORAGE_MANAGE_BYPASS_IO
//

typedef struct _BPIO_OUTPUT {

    //
    // Size of this structure serves
    // as the version.
    //
    ULONG Version;

    //
    // Size of this structure plus
    // all the variable sized fields.
    //
    ULONG Size;

    //
    //  The BypassIO operation being requested.  This should be set to the
    //  same value passed in the input structure
    //

    BPIO_OPERATIONS Operation;

    //
    //  Output flags for this operation.
    //

    BPIO_OUTFLAGS OutFlags;

    ULONGLONG Reserved2;    //reserved for future expansion

    //
    //  This defines operation specific outputs
    //

    union {

        BPIO_RESULTS Enable;
        BPIO_RESULTS Query;
    };

} BPIO_OUTPUT, *PBPIO_OUTPUT;

#define BPIO_OUTPUT_ENABLE_SIZE          (sizeof(BPIO_OUTPUT))
#define BPIO_OUTPUT_QUERY_SIZE           (sizeof(BPIO_OUTPUT))
#define BPIO_OUTPUT_DISABLE_SIZE         ((size_t)FIELD_OFFSET(BPIO_OUTPUT, Enable))
#define BPIO_OUTPUT_VOLUME_DISABLE_SIZE  ((size_t)FIELD_OFFSET(BPIO_OUTPUT, Enable))

#pragma warning(pop)

#endif // (NTDDI_VERSION >= NTDDI_WIN10_CO)

// begin_winioctl


#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#if defined __cplusplus && !defined __ALT_GENERATOR__
}
#endif

#endif // _NTDDSTOR_H_
// end_winioctl


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

