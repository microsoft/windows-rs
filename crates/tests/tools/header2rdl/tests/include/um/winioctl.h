/*++ BUILD Version: 0032    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    winioctl.h

Abstract:

    This module defines the 32-Bit Windows Device I/O control codes.

Revision History:

--*/


//
// Device interface class GUIDs.
//
// need these GUIDs outside conditional includes so that user can
//   #include <winioctl.h> in precompiled header
//   #include <initguid.h> in a single source file
//   #include <winioctl.h> in that source file a second time to instantiate the GUIDs
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


//
// This interface represents a physical persistent memory device, such as an NVDIMM.
// {4283609D-4DC2-43BE-BBB4-4F15DFCE2C61}
//
DEFINE_GUID(GUID_DEVINTERFACE_SCM_PHYSICAL_DEVICE, 0x4283609d, 0x4dc2, 0x43be, 0xbb, 0xb4, 0x4f, 0x15, 0xdf, 0xce, 0x2c, 0x61);

//
// When a physical device driver detects a change in the health status of a physical device,
// it triggers a PNP custom event (through TARGET_DEVICE_CUSTOM_NOTIFICATION) to alert any
// registered components. The custom event's GUID is GUID_SCM_PD_HEALTH_NOTIFICATION
// and its payload is SCM_PD_HEALTH_NOTIFICATION_DATA
// {9DA2D386-72F5-4EE3-8155-ECA0678E3B06}
//
DEFINE_GUID(GUID_SCM_PD_HEALTH_NOTIFICATION, 0x9da2d386, 0x72f5, 0x4ee3, 0x81, 0x55, 0xec, 0xa0, 0x67, 0x8e, 0x3b, 0x6);

//
// The passthrough protocol GUID for INVDIMM devices. The application and the driver use this value
// for the "ProtocolGuid" field of the SCM_PD_PASSTHROUGH_INPUT and SCM_PD_PASSTHROUGH_OUTPUT structures.
// {4309AC30-0D11-11E4-9191-0800200C9A66}
//
DEFINE_GUID(GUID_SCM_PD_PASSTHROUGH_INVDIMM, 0x4309AC30, 0x0D11, 0x11E4, 0x91, 0x91, 0x08, 0x00, 0x20, 0x0C, 0x9A, 0x66);

// {86E0D1E0-8089-11D0-9CE4-08003E301F73}
DEFINE_GUID(GUID_DEVINTERFACE_COMPORT,                0X86E0D1E0L, 0X8089, 0X11D0, 0X9C, 0XE4, 0X08, 0X00, 0X3E, 0X30, 0X1F, 0X73);
// {4D36E978-E325-11CE-BFC1-08002BE10318}
DEFINE_GUID(GUID_DEVINTERFACE_SERENUM_BUS_ENUMERATOR, 0x4D36E978L, 0xE325, 0x11CE, 0xBF, 0xC1, 0x08, 0x00, 0x2B, 0xE1, 0x03, 0x18);

//
// Obsolete device interface class GUID names.
// (use of above GUID_DEVINTERFACE_* names is recommended).
//

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

#define GUID_CLASS_COMPORT          GUID_DEVINTERFACE_COMPORT
#define GUID_SERENUM_BUS_ENUMERATOR GUID_DEVINTERFACE_SERENUM_BUS_ENUMERATOR

#endif // DEFINE_GUID

//
// Interface DEVPROPKEY
//
// need these DEVPROPKEYs outside conditional includes so that user can
//   #include <winioctl.h> in precompiled header
//   #include <devpropdef.h> in a single source file
//   #include <winioctl.h> in that source file a second time to instantiate the DEVPROPKEYs
//
#ifdef DEFINE_DEVPROPKEY


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

#endif // DEFINE_DEVPROPKEY

#ifndef _WINIOCTL_
#define _WINIOCTL_

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4201) // nonstandard extension used : nameless struct/union
#pragma warning(disable:4820) // padding added after data member
#endif


#ifndef _DEVIOCTL_
#define _DEVIOCTL_

// begin_ntddk begin_wdm begin_nthal begin_ntifs
//
// Define the various device type values.  Note that values used by Microsoft
// Corporation are in the range 0-32767, and 32768-65535 are reserved for use
// by customers.
//

#define DEVICE_TYPE DWORD

#define FILE_DEVICE_BEEP                0x00000001
#define FILE_DEVICE_CD_ROM              0x00000002
#define FILE_DEVICE_CD_ROM_FILE_SYSTEM  0x00000003
#define FILE_DEVICE_CONTROLLER          0x00000004
#define FILE_DEVICE_DATALINK            0x00000005
#define FILE_DEVICE_DFS                 0x00000006
#define FILE_DEVICE_DISK                0x00000007
#define FILE_DEVICE_DISK_FILE_SYSTEM    0x00000008
#define FILE_DEVICE_FILE_SYSTEM         0x00000009
#define FILE_DEVICE_INPORT_PORT         0x0000000a
#define FILE_DEVICE_KEYBOARD            0x0000000b
#define FILE_DEVICE_MAILSLOT            0x0000000c
#define FILE_DEVICE_MIDI_IN             0x0000000d
#define FILE_DEVICE_MIDI_OUT            0x0000000e
#define FILE_DEVICE_MOUSE               0x0000000f
#define FILE_DEVICE_MULTI_UNC_PROVIDER  0x00000010
#define FILE_DEVICE_NAMED_PIPE          0x00000011
#define FILE_DEVICE_NETWORK             0x00000012
#define FILE_DEVICE_NETWORK_BROWSER     0x00000013
#define FILE_DEVICE_NETWORK_FILE_SYSTEM 0x00000014
#define FILE_DEVICE_NULL                0x00000015
#define FILE_DEVICE_PARALLEL_PORT       0x00000016
#define FILE_DEVICE_PHYSICAL_NETCARD    0x00000017
#define FILE_DEVICE_PRINTER             0x00000018
#define FILE_DEVICE_SCANNER             0x00000019
#define FILE_DEVICE_SERIAL_MOUSE_PORT   0x0000001a
#define FILE_DEVICE_SERIAL_PORT         0x0000001b
#define FILE_DEVICE_SCREEN              0x0000001c
#define FILE_DEVICE_SOUND               0x0000001d
#define FILE_DEVICE_STREAMS             0x0000001e
#define FILE_DEVICE_TAPE                0x0000001f
#define FILE_DEVICE_TAPE_FILE_SYSTEM    0x00000020
#define FILE_DEVICE_TRANSPORT           0x00000021
#define FILE_DEVICE_UNKNOWN             0x00000022
#define FILE_DEVICE_VIDEO               0x00000023
#define FILE_DEVICE_VIRTUAL_DISK        0x00000024
#define FILE_DEVICE_WAVE_IN             0x00000025
#define FILE_DEVICE_WAVE_OUT            0x00000026
#define FILE_DEVICE_8042_PORT           0x00000027
#define FILE_DEVICE_NETWORK_REDIRECTOR  0x00000028
#define FILE_DEVICE_BATTERY             0x00000029
#define FILE_DEVICE_BUS_EXTENDER        0x0000002a
#define FILE_DEVICE_MODEM               0x0000002b
#define FILE_DEVICE_VDM                 0x0000002c
#define FILE_DEVICE_MASS_STORAGE        0x0000002d
#define FILE_DEVICE_SMB                 0x0000002e
#define FILE_DEVICE_KS                  0x0000002f
#define FILE_DEVICE_CHANGER             0x00000030
#define FILE_DEVICE_SMARTCARD           0x00000031
#define FILE_DEVICE_ACPI                0x00000032
#define FILE_DEVICE_DVD                 0x00000033
#define FILE_DEVICE_FULLSCREEN_VIDEO    0x00000034
#define FILE_DEVICE_DFS_FILE_SYSTEM     0x00000035
#define FILE_DEVICE_DFS_VOLUME          0x00000036
#define FILE_DEVICE_SERENUM             0x00000037
#define FILE_DEVICE_TERMSRV             0x00000038
#define FILE_DEVICE_KSEC                0x00000039
#define FILE_DEVICE_FIPS                0x0000003A
#define FILE_DEVICE_INFINIBAND          0x0000003B
#define FILE_DEVICE_VMBUS               0x0000003E
#define FILE_DEVICE_CRYPT_PROVIDER      0x0000003F
#define FILE_DEVICE_WPD                 0x00000040
#define FILE_DEVICE_BLUETOOTH           0x00000041
#define FILE_DEVICE_MT_COMPOSITE        0x00000042
#define FILE_DEVICE_MT_TRANSPORT        0x00000043
#define FILE_DEVICE_BIOMETRIC           0x00000044
#define FILE_DEVICE_PMI                 0x00000045
#define FILE_DEVICE_EHSTOR              0x00000046
#define FILE_DEVICE_DEVAPI              0x00000047
#define FILE_DEVICE_GPIO                0x00000048
#define FILE_DEVICE_USBEX               0x00000049
#define FILE_DEVICE_CONSOLE             0x00000050
#define FILE_DEVICE_NFP                 0x00000051
#define FILE_DEVICE_SYSENV              0x00000052
#define FILE_DEVICE_VIRTUAL_BLOCK       0x00000053
#define FILE_DEVICE_POINT_OF_SERVICE    0x00000054
#define FILE_DEVICE_STORAGE_REPLICATION 0x00000055
#define FILE_DEVICE_TRUST_ENV           0x00000056
#define FILE_DEVICE_UCM                 0x00000057
#define FILE_DEVICE_UCMTCPCI            0x00000058
#define FILE_DEVICE_PERSISTENT_MEMORY   0x00000059
#define FILE_DEVICE_NVDIMM              0x0000005a
#define FILE_DEVICE_HOLOGRAPHIC         0x0000005b
#define FILE_DEVICE_SDFXHCI             0x0000005c
#define FILE_DEVICE_UCMUCSI             0x0000005d
#define FILE_DEVICE_PRM                 0x0000005e
#define FILE_DEVICE_EVENT_COLLECTOR     0x0000005f
#define FILE_DEVICE_USB4                0x00000060
#define FILE_DEVICE_SOUNDWIRE           0x00000061
#define FILE_DEVICE_FABRIC_NVME         0x00000062
#define FILE_DEVICE_SVM                 0x00000063
#define FILE_DEVICE_HARDWARE_ACCELERATOR 0x00000064
#define FILE_DEVICE_I3C                 0x00000065

//
// Macro definition for defining IOCTL and FSCTL function control codes.  Note
// that function codes 0-2047 are reserved for Microsoft Corporation, and
// 2048-4095 are reserved for customers.
//

#define CTL_CODE( DeviceType, Function, Method, Access ) (                 \
    ((DeviceType) << 16) | ((Access) << 14) | ((Function) << 2) | (Method) \
)

//
// Macro to extract device type out of the device io control code
//
#define DEVICE_TYPE_FROM_CTL_CODE(ctrlCode)     (((DWORD)(ctrlCode & 0xffff0000)) >> 16)

//
// Macro to extract buffering method out of the device io control code
//
#define METHOD_FROM_CTL_CODE(ctrlCode)          ((DWORD)(ctrlCode & 3))

//
// Define the method codes for how buffers are passed for I/O and FS controls
//

#define METHOD_BUFFERED                 0
#define METHOD_IN_DIRECT                1
#define METHOD_OUT_DIRECT               2
#define METHOD_NEITHER                  3

//
// Define some easier to comprehend aliases:
//   METHOD_DIRECT_TO_HARDWARE (writes, aka METHOD_IN_DIRECT)
//   METHOD_DIRECT_FROM_HARDWARE (reads, aka METHOD_OUT_DIRECT)
//

#define METHOD_DIRECT_TO_HARDWARE       METHOD_IN_DIRECT
#define METHOD_DIRECT_FROM_HARDWARE     METHOD_OUT_DIRECT

//
// Define the access check value for any access
//
//
// The FILE_READ_ACCESS and FILE_WRITE_ACCESS constants are also defined in
// ntioapi.h as FILE_READ_DATA and FILE_WRITE_DATA. The values for these
// constants *MUST* always be in sync.
//
//
// FILE_SPECIAL_ACCESS is checked by the NT I/O system the same as FILE_ANY_ACCESS.
// The file systems, however, may add additional access checks for I/O and FS controls
// that use this value.
//


#define FILE_ANY_ACCESS                 0
#define FILE_SPECIAL_ACCESS    (FILE_ANY_ACCESS)
#define FILE_READ_ACCESS          ( 0x0001 )    // file & pipe
#define FILE_WRITE_ACCESS         ( 0x0002 )    // file & pipe

// end_ntddk end_wdm end_nthal end_ntifs

#endif // _DEVIOCTL_


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
    DWORD Size; // version
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
    DWORD Size;

    //
    // Version of this structure
    //
    DWORD Version;

    union {

        struct {

            //
            // If set to '1', indicates that support for StorMQ miniports is present
            //
            DWORDLONG StorMQMiniportsSupported : 1;

            //
            // Reserved for future use. Must be set to zero.
            //
            DWORDLONG Reserved : 63;

        } DUMMYSTRUCTNAME;

        DWORDLONG AsUlonglong;

    } Flags;

    DWORDLONG Reserved[6];

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

    DWORD       DeviceNumber;

    //
    // If the device is partitionable, the partition number of the device.
    // Otherwise -1
    //

    DWORD       PartitionNumber;
} STORAGE_DEVICE_NUMBER, *PSTORAGE_DEVICE_NUMBER;

typedef struct _STORAGE_DEVICE_NUMBERS {

    //
    // Size of this structure serves
    // as the version
    //

    DWORD Version;

    //
    // Size of this structure
    //

    DWORD Size;

    DWORD NumberOfDevices;

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

    DWORD       Version;

    //
    // Total size of the structure, including any additional data. Currently
    // this will always be the same as sizeof(STORAGE_DEVICE_NUMBER_EX).
    //

    DWORD       Size;

    //
    // Flags - this shall be a combination of STORAGE_DEVICE_FLAGS_XXX flags
    // that gives more information about the members of this structure.
    //

    DWORD       Flags;

    //
    // The FILE_DEVICE_XXX type for this device. This IOCTL is only
    // supported for disk devices.
    //

    DEVICE_TYPE DeviceType;

    //
    // The number of this device.
    //

    DWORD       DeviceNumber;

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

    DWORD       PartitionNumber;
} STORAGE_DEVICE_NUMBER_EX, *PSTORAGE_DEVICE_NUMBER_EX;


//
// Define the structures for scsi resets
//

typedef struct _STORAGE_BUS_RESET_REQUEST {
    BYTE  PathId;
} STORAGE_BUS_RESET_REQUEST, *PSTORAGE_BUS_RESET_REQUEST;


//
// Break reservation is sent to the Adapter/FDO with the given lun information.
//

typedef struct STORAGE_BREAK_RESERVATION_REQUEST {
    DWORD Length;
    BYTE  _unused;
    BYTE  PathId;
    BYTE  TargetId;
    BYTE  Lun;
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
    DWORD MediaChangeCount;
    DWORD NewState;         // see MEDIA_CHANGE_DETECTION_STATE enum in classpnp.h in DDK
} CLASS_MEDIA_CHANGE_CONTEXT, *PCLASS_MEDIA_CHANGE_CONTEXT;


// begin_ntminitape

typedef struct _TAPE_STATISTICS {
    DWORD Version;
    DWORD Flags;
    LARGE_INTEGER RecoveredWrites;
    LARGE_INTEGER UnrecoveredWrites;
    LARGE_INTEGER RecoveredReads;
    LARGE_INTEGER UnrecoveredReads;
    BYTE          CompressionRatioReads;
    BYTE          CompressionRatioWrites;
} TAPE_STATISTICS, *PTAPE_STATISTICS;

#define RECOVERED_WRITES_VALID   0x00000001
#define UNRECOVERED_WRITES_VALID 0x00000002
#define RECOVERED_READS_VALID    0x00000004
#define UNRECOVERED_READS_VALID  0x00000008
#define WRITE_COMPRESSION_INFO_VALID  0x00000010
#define READ_COMPRESSION_INFO_VALID   0x00000020

typedef struct _TAPE_GET_STATISTICS {
    DWORD Operation;
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
            DWORD TracksPerCylinder;
            DWORD SectorsPerTrack;
            DWORD BytesPerSector;
            DWORD NumberMediaSides;
            DWORD MediaCharacteristics; // Bitmask of MEDIA_XXX values.
        } DiskInfo;

        struct {
            LARGE_INTEGER Cylinders;
            STORAGE_MEDIA_TYPE MediaType;
            DWORD TracksPerCylinder;
            DWORD SectorsPerTrack;
            DWORD BytesPerSector;
            DWORD NumberMediaSides;
            DWORD MediaCharacteristics; // Bitmask of MEDIA_XXX values.
        } RemovableDiskInfo;

        struct {
            STORAGE_MEDIA_TYPE MediaType;
            DWORD   MediaCharacteristics; // Bitmask of MEDIA_XXX values.
            DWORD   CurrentBlockSize;
            STORAGE_BUS_TYPE BusType;

            //
            // Bus specific information describing the medium supported.
            //

            union {
                struct {
                    BYTE  MediumType;
                    BYTE  DensityCode;
                } ScsiInformation;
            } BusSpecificData;

        } TapeInfo;
    } DeviceSpecific;
} DEVICE_MEDIA_INFO, *PDEVICE_MEDIA_INFO;

typedef struct _GET_MEDIA_TYPES {
    DWORD DeviceType;              // FILE_DEVICE_XXX values
    DWORD MediaInfoCount;
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
    DWORD PredictFailure;
    BYTE  VendorSpecific[512];
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
    DWORD Version;      // Set to 1 for Blue.
    DWORD Size;
    BOOLEAN Set;        // TRUE if the sender wants to enable/disable failure prediction.
    BOOLEAN Enabled;
    WORD   Reserved;
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

    BYTE  AdditionalParameters[1];

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

    BYTE  AdditionalParameters[1];

} STORAGE_PROPERTY_SET, *PSTORAGE_PROPERTY_SET;

//
// Standard property descriptor header.  All property pages should use this
// as their first element or should contain these two elements
//

typedef _Struct_size_bytes_(Size) struct _STORAGE_DESCRIPTOR_HEADER {

    DWORD Version;

    DWORD Size;

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

    DWORD Version;

    //
    // Total size of the descriptor, including the space for additional
    // data and id strings
    //

    DWORD Size;

    //
    // The SCSI-2 device type
    //

    BYTE  DeviceType;

    //
    // The SCSI-2 device type modifier (if any) - this may be zero
    //

    BYTE  DeviceTypeModifier;

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

    DWORD VendorIdOffset;

    //
    // Byte offset to the zero-terminated ascii string containing the device's
    // product id string.  For devices with no such ID this will be zero
    //

    DWORD ProductIdOffset;

    //
    // Byte offset to the zero-terminated ascii string containing the device's
    // product revision string.  For devices with no such string this will be
    // zero
    //

    DWORD ProductRevisionOffset;

    //
    // Byte offset to the zero-terminated ascii string containing the device's
    // serial number.  For devices with no serial number this will be zero
    //

    DWORD SerialNumberOffset;

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

    DWORD RawPropertiesLength;

    //
    // Place holder for the first byte of the bus specific property data
    //

    BYTE  RawDeviceProperties[1];

} STORAGE_DEVICE_DESCRIPTOR, *PSTORAGE_DEVICE_DESCRIPTOR;


//
// Adapter properties
//
// This descriptor can be retrieved from a target device object of from the
// device object for the bus.  Retrieving from the target device object will
// forward the request to the underlying bus
//

typedef _Struct_size_bytes_(Size) struct _STORAGE_ADAPTER_DESCRIPTOR {

    DWORD Version;

    DWORD Size;

    DWORD MaximumTransferLength;

    DWORD MaximumPhysicalPages;

    DWORD AlignmentMask;

    BOOLEAN AdapterUsesPio;

    BOOLEAN AdapterScansDown;

    BOOLEAN CommandQueueing;

    BOOLEAN AcceleratedTransfer;

#if (NTDDI_VERSION < NTDDI_WINXP)
    BOOLEAN BusType;
#else
    BYTE  BusType;
#endif

    WORD   BusMajorVersion;

    WORD   BusMinorVersion;

#if (NTDDI_VERSION >= NTDDI_WIN8)

    BYTE  SrbType;

    BYTE  AddressType;
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

    DWORD Version;

    //
    // Total size of the descriptor, including the space for additional
    // data and id strings
    //

    DWORD Size;

    //
    // The number of bytes in a cache line of the device
    //

    DWORD BytesPerCacheLine;

    //
    // The address offset neccessary for proper cache access alignment in bytes
    //

    DWORD BytesOffsetForCacheAlignment;

    //
    // The number of bytes in a physical sector of the device
    //

    DWORD BytesPerLogicalSector;

    //
    // The number of bytes in an addressable logical sector (LBA)of the device
    //

    DWORD BytesPerPhysicalSector;

    //
    // The address offset neccessary for proper sector access alignment in bytes
    //

    DWORD BytesOffsetForSectorAlignment;

} STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR, *PSTORAGE_ACCESS_ALIGNMENT_DESCRIPTOR;

typedef _Struct_size_bytes_(Size) struct _STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {

    //
    // Sizeof(STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR)
    //

    DWORD Version;

    //
    // Total size of the descriptor, including the space for additional data
    //

    DWORD Size;

    //
    // Product type of the supporting storage medium
    //

    DWORD MediumProductType;

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

    DWORD Version;

    DWORD Size;

    STORAGE_PORT_CODE_SET Portdriver;

    BOOLEAN LUNResetSupported;

    BOOLEAN TargetResetSupported;

#if (NTDDI_VERSION >= NTDDI_WIN8)
    WORD    IoTimeoutValue;
#endif

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
    BOOLEAN ExtraIoInfoSupported;

#if (NTDDI_VERSION >= NTDDI_WIN10_FE)

    union {
        struct {
            BYTE  LogicalPoFxForDisk : 1;
            BYTE  ForwardIo : 1;
            BYTE  Reserved : 6;
        } DUMMYSTRUCTNAME;
        BYTE  AsBYTE ;
    } Flags;

    BYTE    Reserved0[2];

#else
    BYTE    Reserved0[3];
#endif

    DWORD   Reserved1;
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

    WORD   IdentifierSize;

    WORD   NextOffset;

    //
    // Add new fields here since existing code depends on
    // the above layout not changing.
    //

    STORAGE_ASSOCIATION_TYPE Association;

    //
    // The identifier is a variable length array of bytes.
    //

    BYTE  Identifier[1];

} STORAGE_IDENTIFIER, *PSTORAGE_IDENTIFIER;

typedef _Struct_size_bytes_(Size) struct _STORAGE_DEVICE_ID_DESCRIPTOR {

    DWORD Version;

    DWORD Size;

    //
    // The number of identifiers reported by the device.
    //

    DWORD NumberOfIdentifiers;

    //
    // The following field is actually a variable length array of identification
    // descriptors.  Unfortunately there's no C notation for an array of
    // variable length structures so we're forced to just pretend.
    //

    BYTE  Identifiers[1];

} STORAGE_DEVICE_ID_DESCRIPTOR, *PSTORAGE_DEVICE_ID_DESCRIPTOR;

// output buffer for   StorageDeviceSeekPenaltyProperty & PropertyStandardQuery
typedef struct _DEVICE_SEEK_PENALTY_DESCRIPTOR {

    DWORD       Version;          // keep compatible with STORAGE_DESCRIPTOR_HEADER

    DWORD       Size;             // keep compatible with STORAGE_DESCRIPTOR_HEADER

    BOOLEAN     IncursSeekPenalty;

} DEVICE_SEEK_PENALTY_DESCRIPTOR, *PDEVICE_SEEK_PENALTY_DESCRIPTOR;

// output buffer for   StorageDeviceWriteAggregationProperty & PropertyStandardQuery
typedef struct _DEVICE_WRITE_AGGREGATION_DESCRIPTOR {
    DWORD       Version;          // keep compatible with STORAGE_DESCRIPTOR_HEADER
    DWORD       Size;             // keep compatible with STORAGE_DESCRIPTOR_HEADER

    BOOLEAN     BenefitsFromWriteAggregation;
} DEVICE_WRITE_AGGREGATION_DESCRIPTOR, *PDEVICE_WRITE_AGGREGATION_DESCRIPTOR;

// output buffer for   StorageDeviceTrimProperty & PropertyStandardQuery
typedef struct _DEVICE_TRIM_DESCRIPTOR {

    DWORD       Version;          // keep compatible with STORAGE_DESCRIPTOR_HEADER

    DWORD       Size;             // keep compatible with STORAGE_DESCRIPTOR_HEADER

    BOOLEAN     TrimEnabled;

} DEVICE_TRIM_DESCRIPTOR, *PDEVICE_TRIM_DESCRIPTOR;

#pragma warning(push)
#pragma warning(disable:4214) // bit fields other than int
//
// Output buffer for StorageDeviceLBProvisioningProperty & PropertyStandardQuery
//
typedef struct _DEVICE_LB_PROVISIONING_DESCRIPTOR {

    DWORD       Version;          // keep compatible with STORAGE_DESCRIPTOR_HEADER

    DWORD       Size;             // keep compatible with STORAGE_DESCRIPTOR_HEADER

    BYTE  ThinProvisioningEnabled : 1;

    BYTE  ThinProvisioningReadZeros : 1;

    BYTE  AnchorSupported : 3;

    BYTE  UnmapGranularityAlignmentValid : 1;

    BYTE  GetFreeSpaceSupported : 1;        // Supports DeviceDsmAction_GetFreeSpace

    BYTE  MapSupported : 1;                 // Supports DeviceDsmAction_Map

    BYTE  Reserved1[7];

    DWORDLONG OptimalUnmapGranularity;      // Granularity in bytes.

    DWORDLONG UnmapGranularityAlignment;    // Granularity alignment in bytes.

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

    DWORD MaxUnmapLbaCount;                 // Max LBAs that can be unmapped in a single UNMAP command, in logical blocks.

    DWORD MaxUnmapBlockDescriptorCount;     // Max number of descriptors allowed in a single UNMAP command.

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
    DWORD       Size;
    DWORD       Version;
    BYTE        AvailableMappingResourcesValid : 1;
    BYTE        UsedMappingResourcesValid : 1;
    BYTE        Reserved0 : 6;
    BYTE        Reserved1[3];
    BYTE        AvailableMappingResourcesScope : 2; // See LOG_PAGE_LBP_RESOURCE_SCOPE_* definitions in scsi.h for scope values.
    BYTE        UsedMappingResourcesScope : 2;
    BYTE        Reserved2 : 4;
    BYTE        Reserved3[3];
    DWORDLONG   AvailableMappingResources;  // Available LBA mapping resources, in bytes.
    DWORDLONG   UsedMappingResources;       // Used LBA mapping resources, in bytes.
} STORAGE_LB_PROVISIONING_MAP_RESOURCES, *PSTORAGE_LB_PROVISIONING_MAP_RESOURCES;

#pragma warning(pop)

// output buffer for   StorageDevicePowerProperty & PropertyStandardQuery
typedef struct _DEVICE_POWER_DESCRIPTOR {
    DWORD       Version;          // keep compatible with STORAGE_DESCRIPTOR_HEADER
    DWORD       Size;             // keep compatible with STORAGE_DESCRIPTOR_HEADER

    BOOLEAN     DeviceAttentionSupported;   // The device supports "device attention".
    BOOLEAN     AsynchronousNotificationSupported;  // The device supports asynchronous notifications, delivered via IOCTL_STORAGE_EVENT_NOTIFICATION.
    BOOLEAN     IdlePowerManagementEnabled; // The device has been registered for runtime idle power management.
    BOOLEAN     D3ColdEnabled;              // The device will be powered off when put into D3.
    BOOLEAN     D3ColdSupported;            // The platform supports D3Cold for this device.
    BOOLEAN     NoVerifyDuringIdlePower;    // Device require no verification during idle power transitions.
    BYTE        Reserved[2];
    DWORD       IdleTimeoutInMS;            // The idle timeout value in milliseconds. Only valid if IdlePowerManagementEnabled == TRUE.
} DEVICE_POWER_DESCRIPTOR, *PDEVICE_POWER_DESCRIPTOR;

//
// Output buffer for StorageDeviceCopyOffloadProperty & PropertyStandardQuery
//
typedef struct _DEVICE_COPY_OFFLOAD_DESCRIPTOR {
    DWORD       Version;          // keep compatible with STORAGE_DESCRIPTOR_HEADER
    DWORD       Size;             // keep compatible with STORAGE_DESCRIPTOR_HEADER

    DWORD       MaximumTokenLifetime;
    DWORD       DefaultTokenLifetime;
    DWORDLONG   MaximumTransferSize;
    DWORDLONG   OptimalTransferCount;
    DWORD       MaximumDataDescriptors;
    DWORD       MaximumTransferLengthPerDescriptor;
    DWORD       OptimalTransferLengthPerDescriptor;
    WORD        OptimalTransferLengthGranularity;
    BYTE        Reserved[2];
} DEVICE_COPY_OFFLOAD_DESCRIPTOR, *PDEVICE_COPY_OFFLOAD_DESCRIPTOR;

//
// Output buffer for StorageDeviceResiliencyProperty & PropertyStandardQuery
//

typedef _Struct_size_bytes_(Size) struct _STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {

    //
    // Size of this structure serves
    // as the version
    //

    DWORD Version;

    //
    // Size of  this structure  plus
    // all the variable sized fields
    //

    DWORD Size;

    //
    // Friendly name associated with
    // this descriptor
    //

    DWORD NameOffset;

    //
    // Number of  logical  copies of
    // data that are available
    //

    DWORD NumberOfLogicalCopies;

    //
    // Number of  complete copies of
    // data that are stored
    //

    DWORD NumberOfPhysicalCopies;

    //
    // Number of disks that can fail
    // without leading to  data loss
    //

    DWORD PhysicalDiskRedundancy;

    //
    // Number  of columns associated
    // with this descriptor
    //

    DWORD NumberOfColumns;

    //
    // Stripe  width associated with
    // this descriptor, in bytes
    //

    DWORD Interleave;

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

    DWORD Version;

    //
    // Keep compatible with STORAGE_DESCRIPTOR_HEADER
    // Shall be set to sizeof(STORAGE_RPMB_DESCRIPTOR)
    //

    DWORD Size;

    //
    // The size of the RPMB, in bytes.
    //
    // 0 if not supported, RPMB size in bytes otherwise
    //

    DWORD SizeInBytes;

    //
    // The maximum amount of data supported in one transaction
    // in bytes.
    //
    // 0 if not supported, minimum 512 bytes
    //

    DWORD MaxReliableWriteSizeInBytes;

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

    DWORD Version;

    //
    // Size of this structure. This shall be set to
    // sizeof(STORAGE_CRYPTO_CAPABILITY)
    //

    DWORD Size;

    //
    // The index for this crypto capability
    //

    DWORD CryptoCapabilityIndex;

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

    DWORD DataUnitSizeBitmask;

} STORAGE_CRYPTO_CAPABILITY, *PSTORAGE_CRYPTO_CAPABILITY;

#define STORAGE_CRYPTO_CAPABILITY_VERSION_2           2

// begin_storport begin_privstorport

#ifndef STORAGE_SECURITY_COMPLIANCE_BITMASK_DEFINED
#define STORAGE_SECURITY_COMPLIANCE_BITMASK_DEFINED

typedef union _STORAGE_SECURITY_COMPLIANCE_BITMASK {
    struct {
        BYTE  FIPS : 1;
        BYTE  Reserved : 7;
    };
    BYTE  AsUchar;
} STORAGE_SECURITY_COMPLIANCE_BITMASK;

#endif

#ifndef STORAGE_CRYPTO_KEY_TYPE_DEFINED
#define STORAGE_CRYPTO_KEY_TYPE_DEFINED

typedef union _STORAGE_CRYPTO_KEY_TYPE {
    struct {
        BYTE  DirectKey : 1;
        BYTE  PlatformWrappedKey : 1;
        BYTE  PlutonWrappedKey : 1;
        BYTE  Reserved : 5;
    };
    BYTE  AsUchar;
} STORAGE_CRYPTO_KEY_TYPE;

#endif

// end_storport end_privstorport

typedef struct _STORAGE_CRYPTO_CAPABILITY_V2 {

    //
    // To enable versioning of this structure. This shall be set
    // to STORAGE_CRYPTO_CAPABILITY_VERSION_2
    //

    DWORD Version;

    //
    // Size of this structure. This shall be set to
    // sizeof(STORAGE_CRYPTO_CAPABILITY_V2)
    //

    DWORD Size;

    //
    // The index for this crypto capability
    //

    DWORD CryptoCapabilityIndex;

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

    DWORD DataUnitSizeBitmask;

    //
    // Maximum supported initialization vector bit size. This can be 0 if
    // this concept does not apply to the algorithm.
    //

    WORD   MaxIVBitSize;
    WORD   Reserved;

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

    DWORD Version;

    //
    // Keep compatible with STORAGE_DESCRIPTOR_HEADER
    // Shall be set to sizeof(STORAGE_CRYPTO_DESCRIPTOR)
    //

    DWORD Size;

    //
    // The number of keys the crypto engine in the adapter supports
    //

    DWORD NumKeysSupported;

    //
    // The number of crypto capability entries. This outlines the
    // crypto configurations the adapter supports
    //

    DWORD NumCryptoCapabilities;

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

    DWORD Version;

    //
    // Keep compatible with STORAGE_DESCRIPTOR_HEADER
    // Shall be set to sizeof(STORAGE_CRYPTO_DESCRIPTOR_V2)
    //

    DWORD Size;

    //
    // The number of keys the crypto engine in the adapter supports
    //

    DWORD NumKeysSupported;

    //
    // The number of crypto capability entries. This outlines the
    // crypto configurations the adapter supports
    //

    DWORD NumCryptoCapabilities;

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
    //          (STORAGE_CRYPTO_CAPABILITY_V2*)((PBYTE )curCryptoCapability + curCryptoCapability->Size)
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

    DWORD Version;

    //
    // Size of this structure. This shall be set to
    // sizeof(STORAGE_HW_CRYPTO_CAPABILITY)
    //

    DWORD Size;

    //
    // The index for this crypto capability
    //

    DWORD CryptoCapabilityIndex;

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

    DWORD DataUnitSizeBitmask;

    //
    // Maximum supported initialization vector bit size. This can be 0 if
    // this concept does not apply to the algorithm.
    //

    WORD   MaxIVBitSize;
    WORD   Reserved;

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

    DWORD NumKeysSupported;

    //
    // The number of crypto capability entries. This outlines the
    // crypto configurations the crypto engine supports.
    //

    DWORD NumCryptoCapabilities;

    //
    // Offset to an array of STORAGE_HW_CRYPTO_CAPABILITY
    // structures from the beginning of STORAGE_HW_CRYPTO_DESCRIPTOR.
    // Use STORAGE_HW_CRYPTO_CAPABILITY::Size to iterate through the
    // elements.
    //

    _Field_range_(sizeof(struct _STORAGE_HW_CRYPTO_DESCRIPTOR), Header.Size)
    DWORD OffsetToCryptoCapabilities;

    //
    // Size of each crypto capability array element.
    //

    DWORD SizeOfCryptoCapability;

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
    DWORD Index
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
    DWORD Index
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

    DWORDLONG Flags;

    //
    // Provisioned capacity of the tier
    //

    DWORDLONG ProvisionedCapacity;

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

    DWORD Version;

    //
    // Size of  this structure  plus
    // all the variable sized fields
    //

    DWORD Size;

    //
    // Flags. The upper WORD   of these flags is reserved for file system use as
    // this structure is returned slightly tweaked in FSCTL_QUERY_STORAGE_CLASSES_OUTPUT.
    //

    DWORD Flags;

    //
    // The total number of available tiers for this disk
    //

    DWORD TotalNumberOfTiers;

    //
    // The number of tiers that fit in the output
    //

    DWORD NumberOfTiersReturned;

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

    DWORD Version;

    //
    // Size of  this structure  plus
    // all the variable sized fields
    //

    DWORD Size;

    //
    // Number of fault domains
    //

    DWORD NumberOfFaultDomains;

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

        DWORD RetainAsynEvent   :  1;
        DWORD LogSpecificField  :  4;
        DWORD Reserved0         :  3;
        DWORD UUIDIndex         :  7;
        DWORD Reserved          : 17;

    } DUMMYSTRUCTNAME;

    DWORD AsUlong;

} STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE, *PSTORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE;

#pragma warning(pop)

//
// Protocol Data should follow this data structure in the same buffer.
// The offset of Protocol Data from the beginning of this data structure
// is reported in data field - "ProtocolDataOffset".
//
typedef struct _STORAGE_PROTOCOL_SPECIFIC_DATA {

    STORAGE_PROTOCOL_TYPE ProtocolType;
    DWORD   DataType;                     // The value will be protocol specific, as defined in STORAGE_PROTOCOL_NVME_DATA_TYPE or STORAGE_PROTOCOL_ATA_DATA_TYPE.

    DWORD   ProtocolDataRequestValue;
    DWORD   ProtocolDataRequestSubValue;  // Data sub request value

    DWORD   ProtocolDataOffset;           // The offset of data buffer is from beginning of this data structure.
    DWORD   ProtocolDataLength;

    DWORD   FixedProtocolReturnData;
    DWORD   ProtocolDataRequestSubValue2; // First additional data sub request value

    DWORD   ProtocolDataRequestSubValue3; // Second additional data sub request value
    DWORD   ProtocolDataRequestSubValue4; // Third additional data sub request value

} STORAGE_PROTOCOL_SPECIFIC_DATA, *PSTORAGE_PROTOCOL_SPECIFIC_DATA;

//
// Extended type incorporates both Get/Set protocol data
// Protocol Data should follow this data structure in the same buffer.
// The offset of Protocol Data from the beginning of this data structure
// is reported in data field - "ProtocolDataOffset".
//
typedef struct _STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {

    STORAGE_PROTOCOL_TYPE ProtocolType;
    DWORD   DataType;                  // The value will be protocol specific, as defined in STORAGE_PROTOCOL_NVME_DATA_TYPE or STORAGE_PROTOCOL_ATA_DATA_TYPE.

    DWORD   ProtocolDataValue;
    DWORD   ProtocolDataSubValue;      // Data sub request value

    DWORD   ProtocolDataOffset;        // The offset of data buffer is from beginning of this data structure.
    DWORD   ProtocolDataLength;

    DWORD   FixedProtocolReturnData;
    DWORD   ProtocolDataSubValue2;     // First additional data sub request value

    DWORD   ProtocolDataSubValue3;     // Second additional data sub request value
    DWORD   ProtocolDataSubValue4;     // Third additional data sub request value

    DWORD   ProtocolDataSubValue5;     // Fourth additional data sub request value
    DWORD   ProtocolDataSubValue6;     // Fifth additional data sub request value

    DWORD   Reserved[4];

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

    DWORD   Version;
    DWORD   Size;

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

    DWORD   Version;
    DWORD   Size;

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

    WORD    Index;                      // Starts from 0. Index 0 may indicate a composite value.
    SHORT   Temperature;                // Signed value; in Celsius.
    SHORT   OverThreshold;              // Signed value; in Celsius.
    SHORT   UnderThreshold;             // Signed value; in Celsius.

    BOOLEAN OverThresholdChangable;     // Can the threshold value being changed by using IOCTL_STORAGE_SET_TEMPERATURE_THRESHOLD.
    BOOLEAN UnderThresholdChangable;    // Can the threshold value being changed by using IOCTL_STORAGE_SET_TEMPERATURE_THRESHOLD.
    BOOLEAN EventGenerated;             // Indicates that notification will be generated when temperature cross threshold.
    BYTE    Reserved0;
    DWORD   Reserved1;

} STORAGE_TEMPERATURE_INFO, *PSTORAGE_TEMPERATURE_INFO;

typedef struct _STORAGE_TEMPERATURE_DATA_DESCRIPTOR {

    DWORD   Version;
    DWORD   Size;

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

    WORD    InfoCount;              // Some devices may report more than one temperature information as there can be multiple sensors implemented.

    BYTE    Reserved0[2];

    DWORD   Reserved1[2];

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

    DWORD   Version;
    DWORD   Size;

    WORD    Flags;
    WORD    Index;

    SHORT   Threshold;          // Signed value; in Celsius.
    BOOLEAN OverThreshold;      // If TRUE, set the OverThreshold value; Otherwise, set the UnderThreshold value.
    BYTE    Reserved;

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
                BYTE    SubMinor;
                BYTE    Minor;
            } DUMMYSTRUCTNAME;

            WORD    AsUshort;

        } MinorVersion;

        WORD    MajorVersion;
    } DUMMYSTRUCTNAME;

    DWORD   AsUlong;

} STORAGE_SPEC_VERSION, *PSTORAGE_SPEC_VERSION;

#pragma warning(pop)


typedef struct _STORAGE_PHYSICAL_DEVICE_DATA {

    DWORD       DeviceId;
    DWORD       Role;                                   // Value(s) of bitmask from STORAGE_COMPONENT_ROLE_xxx

    STORAGE_COMPONENT_HEALTH_STATUS HealthStatus;
    STORAGE_PROTOCOL_TYPE           CommandProtocol;
    STORAGE_SPEC_VERSION            SpecVersion;        // Supported storage spec version. For example: SBC 3, SATA 3.2, NVMe 1.2
    STORAGE_DEVICE_FORM_FACTOR      FormFactor;

    BYTE        Vendor[8];
    BYTE        Model[40];
    BYTE        FirmwareRevision[16];

    DWORDLONG   Capacity;                               // in unit of Kilo-Bytes (1024 bytes).

    BYTE        PhysicalLocation[32];                   // Reserved for future.

    DWORD       Reserved[2];

} STORAGE_PHYSICAL_DEVICE_DATA, *PSTORAGE_PHYSICAL_DEVICE_DATA;


typedef struct _STORAGE_PHYSICAL_ADAPTER_DATA {

    DWORD       AdapterId;
    STORAGE_COMPONENT_HEALTH_STATUS HealthStatus;
    STORAGE_PROTOCOL_TYPE           CommandProtocol;
    STORAGE_SPEC_VERSION            SpecVersion;        // Supported storage spec version. For example: AHCI 1.3.1

    BYTE        Vendor[8];
    BYTE        Model[40];
    BYTE        FirmwareRevision[16];

    BYTE        PhysicalLocation[32];   // Reserve for future.

    BOOLEAN     ExpanderConnected;
    BYTE        Reserved0[3];
    DWORD       Reserved1[3];

} STORAGE_PHYSICAL_ADAPTER_DATA, *PSTORAGE_PHYSICAL_ADAPTER_DATA;


typedef struct _STORAGE_PHYSICAL_NODE_DATA {

    DWORD       NodeId;

    DWORD       AdapterCount;           // 0 or 1
    DWORD       AdapterDataLength;
    DWORD       AdapterDataOffset;      // Offset from beginning of this data structure. The buffer contains an array of STORAGE_PHYSICAL_ADAPTER_DATA.

    DWORD       DeviceCount;            // >= 1
    DWORD       DeviceDataLength;
    DWORD       DeviceDataOffset;       // Offset from beginning of this data structure. The buffer contains an array of STORAGE_PHYSICAL_DEVICE_DATA.

    DWORD       Reserved[3];

} STORAGE_PHYSICAL_NODE_DATA, *PSTORAGE_PHYSICAL_NODE_DATA;


typedef struct _STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {

    DWORD       Version;            // sizeof(STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR)
    DWORD       Size;               // Total size of the data. Should be >= sizeof(STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR)

    DWORD       NodeCount;
    DWORD       Reserved;

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

    DWORD Version;

    //
    // Size of this structure
    //

    DWORD Size;

    //
    // LUN max outstanding IO count
    //

    DWORD LunMaxIoCount;

    //
    // Adapter max outstanding IO count
    //

    DWORD AdapterMaxIoCount;

} STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR, *PSTORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR;

//
// Output buffer for StorageDeviceAttributesProperty & PropertyStandardQuery
//

typedef _Struct_size_bytes_(Size) struct _STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {

    //
    // Size of this structure serves
    // as the version
    //

    DWORD Version;

    //
    // Size of this structure
    //

    DWORD Size;

    //
    // Attributes (bit flags)
    //

    DWORD64 Attributes;

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
    DWORD Version;
    DWORD Size;
    STORAGE_OPERATIONAL_STATUS_REASON Reason;

    union {

        //
        // This is the format if Reason == DiskOpReasonScsiSenseCode.
        //
        struct {
            BYTE  SenseKey;
            BYTE  ASC;
            BYTE  ASCQ;
            BYTE  Reserved;
        } ScsiSenseKey;

        //
        // This is the format if Reason == DiskOpReasonNVDIMM_N.
        //
        struct {
            BYTE  CriticalHealth;
            BYTE  ModuleHealth[2];
            BYTE  ErrorThresholdStatus;
        } NVDIMM_N;

        DWORD AsUlong;
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

    DWORD Version;

    //
    // The total size of the structure, including operational status reasons
    // that didn't fit in the caller's array. Callers should use this field to learn
    // how big the input buffer should be to contain all the available information.
    //

    DWORD Size;

    //
    // Health status.
    //

    STORAGE_DISK_HEALTH_STATUS Health;

    //
    // The number of operational status returned.
    //

    DWORD NumberOfOperationalStatus;

    //
    // The number of additional reasons returned.
    //

    DWORD NumberOfAdditionalReasons;

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

    DWORD Version;

    DWORD Size;

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

    DWORD ZoneCount;                // Count of zones in this group.

    STORAGE_ZONE_TYPES ZoneType;

    DWORDLONG ZoneSize;             // In Bytes

} STORAGE_ZONE_GROUP, *PSTORAGE_ZONE_GROUP;

typedef _Struct_size_bytes_(Size) struct _STORAGE_ZONED_DEVICE_DESCRIPTOR {

    //
    // Size of this structure serves as the version
    //

    DWORD Version;

    //
    // Size of buffer. The returned value indicates how big the buffer should be
    // to store complete data.
    //

    DWORD Size;

    //
    // Zoned device type
    //

    STORAGE_ZONED_DEVICE_TYPES DeviceType;

    //
    // Total zone count
    //

    DWORD ZoneCount;

    //
    // Zone Attributes
    //

    union {
        struct {

            DWORD   MaxOpenZoneCount;

            BOOLEAN UnrestrictedRead;

            BYTE    Reserved[3];

        }  SequentialRequiredZone;      // Host managed device only

        struct {

            DWORD   OptimalOpenZoneCount;

            DWORD   Reserved;

        }  SequentialPreferredZone;     // Host aware device only

    } ZoneAttributes;

    //
    // Zone Layout Information, to provide a picture about locations of different type of zones on disk.
    // The zone layout starts from the first zone, and groups together zones with same type and size.
    //

    DWORD ZoneGroupCount;

    STORAGE_ZONE_GROUP ZoneGroup[ANYSIZE_ARRAY];

} STORAGE_ZONED_DEVICE_DESCRIPTOR, *PSTORAGE_ZONED_DEVICE_DESCRIPTOR;


//
// Output buffer for StorageDeviceLocationProperty & PropertyStandardQuery
//

#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/unions
typedef struct _DEVICE_LOCATION {

    DWORD Socket;

    DWORD Slot;

    DWORD Adapter;

    DWORD Port;

    union {

        struct {

            DWORD Channel;

            DWORD Device;

        } DUMMYSTRUCTNAME;

        struct {

            DWORD Target;

            DWORD Lun;

        } DUMMYSTRUCTNAME2;

    } DUMMYUNIONNAME;

} DEVICE_LOCATION, *PDEVICE_LOCATION;
#pragma warning(pop)

typedef _Struct_size_bytes_(Size) struct _STORAGE_DEVICE_LOCATION_DESCRIPTOR {

    DWORD Version;

    DWORD Size;

    DEVICE_LOCATION Location;

    DWORD StringOffset;

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
    DWORD Version;
    DWORD Size;
    DWORD NumaNode;
} STORAGE_DEVICE_NUMA_PROPERTY, *PSTORAGE_DEVICE_NUMA_PROPERTY;

#define STORAGE_DEVICE_NUMA_NODE_UNKNOWN MAXDWORD

//
// Output buffer for StorageDeviceUnsafeShutdownCount.
//
// On persistent memory devices, the unsafe shutdown count is the number of times
// the logical persistent memory disk was shut down in a way that might have caused
// data loss.
//
typedef struct _STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT {
    DWORD Version;
    DWORD Size;
    DWORD UnsafeShutdownCount;
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
    DWORD       ValidFields;        // ValidFields represents bit mapping of valid fields of any type
                                    // Eg: Bit 0 stands for GroupId, Bit 1 stands for Flags, Bit 3 for BytesReadCount

    DWORD       GroupId;            // Set Id Eg: Set Id for NVMe sets

    struct {
        DWORD   Shared:1;           // TRUE if information is shared with multiple units/groups

        DWORD   Reserved:31;
    } Flags;

    DWORD       LifePercentage;         // Used life percentage

    BYTE        BytesReadCount[16];     // Total bytes read from device (Billion Unit)

    BYTE        ByteWriteCount[16];     // Total bytes written to device (Billion Unit)

} STORAGE_HW_ENDURANCE_INFO, *PSTORAGE_HW_ENDURANCE_INFO;

typedef struct _STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR {
    DWORD                           Version;            // keep compatible with STORAGE_DESCRIPTOR_HEADER

    DWORD                           Size;               // keep compatible with STORAGE_DESCRIPTOR_HEADER

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

    DWORD Version;

    //
    // Size of buffer. The returned value indicates how big the buffer should be
    // to store complete data.
    //

    DWORD Size;

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

    DWORD Version;

    DWORD Size;

    DWORDLONG State;

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

    DWORD Version; // Sizeof(STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY)

    DWORD Size;

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

    DWORD Version; // Sizeof(STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY_V2)

    DWORD Size;

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

    DWORD Version;

    //
    // Total size of the data.
    // Should be >= sizeof(STORAGE_FRU_ID_DESCRIPTOR)
    //

    DWORD Size;

    //
    // The identifier is a variable length array of bytes.
    //

    DWORD IdentifierSize;
    BYTE  Identifier[ANYSIZE_ARRAY];

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

typedef DWORD DEVICE_DATA_MANAGEMENT_SET_ACTION, DEVICE_DSM_ACTION;

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
    DWORDLONG LengthInBytes;

} DEVICE_DATA_SET_RANGE, *PDEVICE_DATA_SET_RANGE,
  DEVICE_DSM_RANGE, *PDEVICE_DSM_RANGE;

typedef struct _DEVICE_MANAGE_DATA_SET_ATTRIBUTES {

    //
    // Size of this structure serves
    // as the version
    //

    DWORD Size;

    DEVICE_DSM_ACTION Action;
    DWORD Flags;

    //
    // Must be aligned to __alignof(action-specific struct)
    //

    DWORD ParameterBlockOffset;
    DWORD ParameterBlockLength;

    //
    // Must be aligned to __alignof(DEVICE_DSM_RANGE)
    //

    DWORD DataSetRangesOffset;
    DWORD DataSetRangesLength;

} DEVICE_MANAGE_DATA_SET_ATTRIBUTES, *PDEVICE_MANAGE_DATA_SET_ATTRIBUTES,
  DEVICE_DSM_INPUT, *PDEVICE_DSM_INPUT;

typedef struct _DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {

    //
    // Size of this structure serves
    // as the version
    //

    DWORD Size;

    DEVICE_DSM_ACTION Action;
    DWORD Flags;

    DWORD OperationStatus;
    DWORD ExtendedError;
    DWORD TargetDetailedError;
    DWORD ReservedStatus;

    //
    // Must be aligned to __alignof(corresponding struct)
    //

    DWORD OutputBlockOffset;
    DWORD OutputBlockLength;

} DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT, *PDEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT,
  DEVICE_DSM_OUTPUT, *PDEVICE_DSM_OUTPUT;

typedef struct _DEVICE_DSM_DEFINITION {

    DEVICE_DSM_ACTION Action;

    BOOLEAN SingleRange;

    DWORD ParameterBlockAlignment;
    DWORD ParameterBlockLength;

    BOOLEAN HasOutput;

    DWORD OutputBlockAlignment;
    DWORD OutputBlockLength;

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

    DWORD Size;

    DWORD Flags;

    DWORD NumFileTypeIDs;
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

#define STORAGE_OFFLOAD_MAX_TOKEN_LENGTH        512        // Keep as DWORD multiple
#define STORAGE_OFFLOAD_TOKEN_ID_LENGTH         0x1F8
#define STORAGE_OFFLOAD_TOKEN_TYPE_ZERO_DATA    0xFFFF0001

#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/unions

typedef struct _STORAGE_OFFLOAD_TOKEN {

    BYTE  TokenType[4];
    BYTE  Reserved[2];
    BYTE  TokenIdLength[2];
    union {
        struct {
            BYTE  Reserved2[STORAGE_OFFLOAD_TOKEN_ID_LENGTH];
        } StorageOffloadZeroDataToken;
        BYTE  Token[STORAGE_OFFLOAD_TOKEN_ID_LENGTH];
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

    DWORD Flags;

    //
    // Token TTL in milli-seconds as
    // requested by the initiator
    //

    DWORD TimeToLive;

    DWORD Reserved[2];

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

    DWORD OffloadReadFlags;
    DWORD Reserved;

    //
    // Length of the "snapshot" that
    // is bound to  the token.  Must
    // be from the lowest range
    //

    DWORDLONG LengthProtected;

    DWORD TokenLength;
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

    DWORD Flags;
    DWORD Reserved;

    //
    // Starting  offset to copy from
    // "snapshot" bound to the token
    //

    DWORDLONG TokenOffset;

    STORAGE_OFFLOAD_TOKEN Token;

} DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS, *PDEVICE_DSM_OFFLOAD_WRITE_PARAMETERS;

//
// STORAGE_OFFLOAD_WRITE_OUTPUT.OffloadWriteFlags
//

#define STORAGE_OFFLOAD_WRITE_RANGE_TRUNCATED   0x0001
#define STORAGE_OFFLOAD_TOKEN_INVALID           0x0002

typedef struct _STORAGE_OFFLOAD_WRITE_OUTPUT {

    DWORD OffloadWriteFlags;
    DWORD Reserved;

    //
    // Length of content copied from
    // the "snapshot" from the start
    //

    DWORDLONG LengthCopied;

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

    DWORD Version;
    DWORD Size;

    //
    // Reserved for future use
    //

    DWORD Flags;

    //
    // DEVICE_DSM_ALLOCATION_OUTPUT_V1 or
    // DEVICE_DSM_ALLOCATION_OUTPUT_V2
    //

    DWORD OutputVersion;

} DEVICE_DATA_SET_LBP_STATE_PARAMETERS, *PDEVICE_DATA_SET_LBP_STATE_PARAMETERS,
  DEVICE_DSM_ALLOCATION_PARAMETERS, *PDEVICE_DSM_ALLOCATION_PARAMETERS;

#define DEVICE_DSM_PARAMETERS_V1                        1
#define DEVICE_DATA_SET_LBP_STATE_PARAMETERS_VERSION_V1 DEVICE_DSM_PARAMETERS_V1

typedef struct _DEVICE_DATA_SET_LB_PROVISIONING_STATE {

    DWORD Size;
    DWORD Version;

    DWORDLONG SlabSizeInBytes;

    //
    // Delta  from the  start offset
    // if the requested range is not
    // aligned to a slab boundary
    //

    DWORD SlabOffsetDeltaInBytes;

    //
    // Number of bits that are valid
    //

    DWORD SlabAllocationBitMapBitCount;

    //
    // Count of DWORDs in the bitmap
    //

    DWORD SlabAllocationBitMapLength;

    //
    // 1 = mapped, 0 = unmapped
    //

    DWORD SlabAllocationBitMap[ANYSIZE_ARRAY];

} DEVICE_DATA_SET_LB_PROVISIONING_STATE, *PDEVICE_DATA_SET_LB_PROVISIONING_STATE,
  DEVICE_DSM_ALLOCATION_OUTPUT, *PDEVICE_DSM_ALLOCATION_OUTPUT;

#define DEVICE_DSM_ALLOCATION_OUTPUT_V1                  (sizeof(DEVICE_DSM_ALLOCATION_OUTPUT))
#define DEVICE_DATA_SET_LB_PROVISIONING_STATE_VERSION_V1 DEVICE_DSM_ALLOCATION_OUTPUT_V1

typedef struct _DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {

    DWORD Size;
    DWORD Version;

    DWORDLONG SlabSizeInBytes;

    //
    // Delta  from the  start offset
    // if the requested range is not
    // aligned to a slab boundary
    //

    DWORDLONG SlabOffsetDeltaInBytes;

    //
    // Number of bits that are valid
    //

    DWORD SlabAllocationBitMapBitCount;

    //
    // Count of DWORDs in the bitmap
    //

    DWORD SlabAllocationBitMapLength;

    //
    // 1 = mapped, 0 = unmapped
    //

    DWORD SlabAllocationBitMap[ANYSIZE_ARRAY];

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

    DWORD NumberOfRepairCopies;
    DWORD SourceCopy;
    DWORD RepairCopies[ANYSIZE_ARRAY];

    //
    // Valid iff DEVICE_DSM_FLAG_REPAIR_INPUT_TOPOLOGY_ID_PRESENT is set
    //
    // To access this field, use the
    // below macro
    //

    // BYTE  TopologyId[16];

} DEVICE_DATA_SET_REPAIR_PARAMETERS, *PDEVICE_DATA_SET_REPAIR_PARAMETERS,
  DEVICE_DSM_REPAIR_PARAMETERS, *PDEVICE_DSM_REPAIR_PARAMETERS;

#define GET_REPAIR_TOPOLOGY_ID(R)                                                                  \
    RtlOffsetToPointer(R,                                                                          \
                       ALIGN_UP_BY(FIELD_OFFSET(DEVICE_DATA_SET_REPAIR_PARAMETERS, RepairCopies) + \
                       sizeof(DWORD) * R->NumberOfRepairCopies,                                    \
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

    DWORD Version;
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

    DWORDLONG BytesProcessed;
    DWORDLONG BytesRepaired;
    DWORDLONG BytesFailed;

} DEVICE_DATA_SET_SCRUB_OUTPUT, *PDEVICE_DATA_SET_SCRUB_OUTPUT,
  DEVICE_DSM_SCRUB_OUTPUT, *PDEVICE_DSM_SCRUB_OUTPUT;

//
// DEVICE_DSM_OUTPUT.Flags
//

#define DEVICE_DSM_FLAG_SCRUB_OUTPUT_PARITY_EXTENT   0x20000000

typedef struct _DEVICE_DATA_SET_SCRUB_EX_OUTPUT {

    DWORDLONG BytesProcessed;
    DWORDLONG BytesRepaired;
    DWORDLONG BytesFailed;

    //
    // Valid iff DEVICE_DSM_FLAG_SCRUB_OUTPUT_PARITY_EXTENT is set
    //

    DEVICE_DSM_RANGE ParityExtent;

    DWORDLONG BytesScrubbed;

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

    DWORD Version;
    DWORD Size;

    //
    // Reserved for future use
    //

    DWORD Flags;

    DWORD NumberOfTierIds;
    GUID  TierIds[ANYSIZE_ARRAY];

} DEVICE_DSM_TIERING_QUERY_INPUT, *PDEVICE_DSM_TIERING_QUERY_INPUT,
  DEVICE_DSM_TIERING_QUERY_PARAMETERS, *PDEVICE_DSM_TIERING_QUERY_PARAMETERS;

typedef struct _STORAGE_TIER_REGION {

    GUID TierId;

    DWORDLONG Offset;
    DWORDLONG Length;

} STORAGE_TIER_REGION, *PSTORAGE_TIER_REGION;

typedef struct _DEVICE_DSM_TIERING_QUERY_OUTPUT {

    DWORD Version;
    DWORD Size;

    //
    // Reserved for future use
    //

    DWORD Flags;
    DWORD Reserved;

    //
    // Delta  from the  start offset
    // if the requested range is not
    // aligned to a  region boundary
    //

    DWORDLONG Alignment;

    //
    // Total  number of regions that
    // are in the specified range
    //

    DWORD TotalNumberOfRegions;

    DWORD NumberOfRegionsReturned;
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

    DWORD Size;

    BYTE  TargetPriority;
    BYTE  Reserved[3];

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

    DWORDLONG TopologyRangeBytes;

    //
    // The corresponding topology id
    //

    BYTE  TopologyId[16];

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
    DWORDLONG   LengthInBytes;

} DEVICE_STORAGE_ADDRESS_RANGE, *PDEVICE_STORAGE_ADDRESS_RANGE;

typedef struct _DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {

    DWORD Version;

    //
    // Reserved for future use
    //

    DWORD Flags;

    //
    // Total number of ranges within
    // the specified ranges. Callers
    // may use it to  determine  the
    // correct  size of  this output
    // buffer
    //

    DWORD TotalNumberOfRanges;

    //
    // If the buffer provided by the
    // caller is not large enough to
    // hold all the requested ranges
    // a STATUS_BUFFER_OVERFLOW will
    // be returned
    //

    DWORD NumberOfRangesReturned;
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

    DWORD Size;

    BYTE  ReportOption;

    //
    // This  bit affects calculation
    // of the zone list length
    //

    BYTE  Partial;

    BYTE  Reserved[2];

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

    DWORD Size;

    STORAGE_ZONE_TYPES ZoneType;
    STORAGE_ZONE_CONDITION ZoneCondition;

    BOOLEAN ResetWritePointerRecommend;
    BYTE  Reserved0[3];

    //
    // In bytes
    //

    DWORDLONG ZoneSize;
    DWORDLONG WritePointerOffset;

} STORAGE_ZONE_DESCRIPTOR, *PSTORAGE_ZONE_DESCRIPTOR;

typedef struct _DEVICE_DSM_REPORT_ZONES_DATA {

    DWORD Size;

    //
    // Represents the number of ZoneDescriptors.
    //
    DWORD ZoneCount;

    STORAGE_ZONES_ATTRIBUTES Attributes;

    DWORD Reserved0;

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

    DWORDLONG LengthInBytes;

    union {

        DWORD AllFlags;

        struct {

            //
            // 1 = bad, 0 = good
            //

            DWORD IsRangeBad : 1;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;

    DWORD Reserved;

} DEVICE_STORAGE_RANGE_ATTRIBUTES, *PDEVICE_STORAGE_RANGE_ATTRIBUTES;

#pragma warning(pop)

//
// DEVICE_DSM_RANGE_ERROR_OUTPUT.Flags
//

#define DEVICE_STORAGE_NO_ERRORS                0x1

typedef struct _DEVICE_DSM_RANGE_ERROR_INFO {

    DWORD Version;

    DWORD Flags;

    //
    // Total number of ranges within
    // the specified ranges. Callers
    // may use it to  determine  the
    // correct  size of  this output
    // buffer
    //

    DWORD TotalNumberOfRanges;

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

    DWORD NumberOfRangesReturned;
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

    DWORD Version;

    DWORDLONG Granularity;

} DEVICE_DSM_LOST_QUERY_PARAMETERS, *PDEVICE_DSM_LOST_QUERY_PARAMETERS;

typedef struct _DEVICE_DSM_LOST_QUERY_OUTPUT {

    //
    // Size of this structure serves
    // as the version
    //

    DWORD Version;

    //
    // Size of  this structure  plus
    // all the variable sized fields
    // needed for  the  entire range
    //

    DWORD Size;

    //
    // Delta  from the  start offset
    // if the requested range is not
    // granularity aligned
    //

    DWORDLONG Alignment;

    //
    // 1 = lost, 0 = readable
    //

    DWORD NumberOfBits;
    DWORD BitMap[ANYSIZE_ARRAY];

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

    DWORD Version;

    //
    // Shared free space available
    //

    DWORDLONG FreeSpace;

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

    DWORD Version;

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
           ((DWORD_PTR)Input +
                       Input->ParameterBlockOffset);
}


FORCEINLINE
PDEVICE_DSM_RANGE
DeviceDsmDataSetRanges (
    _In_ PDEVICE_DSM_INPUT Input
    )
{
    return (PDEVICE_DSM_RANGE)
           ((DWORD_PTR)Input +
                       Input->DataSetRangesOffset);
}


FORCEINLINE
DWORD
DeviceDsmNumberOfDataSetRanges (
    _In_ PDEVICE_DSM_INPUT Input
    )
{
    return Input->DataSetRangesLength /
           sizeof(DEVICE_DSM_RANGE);
}


FORCEINLINE
DWORD
DeviceDsmGetInputLength (
    _In_ PDEVICE_DSM_DEFINITION Definition,
    _In_ DWORD ParameterBlockLength,
    _In_ DWORD NumberOfDataSetRanges
    )
{
    DWORD Bytes = sizeof(DEVICE_DSM_INPUT);

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
DWORD
DeviceDsmGetNumberOfDataSetRanges (
    _In_ PDEVICE_DSM_DEFINITION Definition,
    _In_ DWORD InputLength,
    _In_ DWORD ParameterBlockLength
    )
{
    DWORD Bytes = sizeof(DEVICE_DSM_INPUT);

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
    _In_ _In_range_(>=, sizeof(DEVICE_DSM_INPUT)) DWORD InputLength,
    _In_ DWORD Flags,
    _In_reads_bytes_opt_(ParameterBlockLength) PVOID Parameters,
    _In_ DWORD ParameterBlockLength
    )
{
    DWORD Bytes = sizeof(DEVICE_DSM_INPUT);

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
    _In_ _In_range_(>=, sizeof(DEVICE_DSM_INPUT)) DWORD InputLength,
    _In_ LONGLONG Offset,
    _In_ DWORDLONG Length
    )
{
    DWORD             Bytes  = 0;
    DWORD             Index  = 0;
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
    _In_  _In_range_(>=, sizeof(DEVICE_DSM_INPUT)) DWORD InputLength
    )
{
    DWORD   Max   = 0;
    DWORD   Min   = 0;
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
           ((DWORD_PTR)Output + Output->OutputBlockOffset);
}


FORCEINLINE
DWORD
DeviceDsmGetOutputLength (
    _In_ PDEVICE_DSM_DEFINITION Definition,
    _In_ DWORD OutputBlockLength
    )
{
    DWORD Bytes = 0;

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
    _In_ DWORD OutputLength
    )
{
    DWORD Bytes = DeviceDsmGetOutputLength(Definition,
                                           0);

    return (OutputLength >= Bytes);
}


FORCEINLINE
DWORD
DeviceDsmGetOutputBlockLength (
    _In_ PDEVICE_DSM_DEFINITION Definition,
    _In_ DWORD OutputLength
    )
{
    DWORD Bytes = 0;

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
    _In_  _In_range_(>=, sizeof(DEVICE_DSM_OUTPUT)) DWORD OutputLength,
    _In_ DWORD Flags
    )
{
    DWORD Bytes = sizeof(DEVICE_DSM_OUTPUT);

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
    _In_ _In_range_(>=, sizeof(DEVICE_DSM_OUTPUT)) DWORD OutputLength
    )
{
    DWORD   Max   = 0;
    DWORD   Min   = 0;
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
    DWORD MaximumRequestsPerPeriod;

    //
    // Specifies the minimum period that the
    // device uses  when scheduling requests
    //
    DWORD MinimumPeriod;

    //
    // Specifies the maximum transfer size supported
    // for  bandwidth contracts  on this  device. To
    // achieve the highest level of performance, all
    // requests should be of this size
    //
    DWORDLONG MaximumRequestSize;

    //
    // Specifies the estimated time taken to
    // perform an  Io operstion. This  field
    // is  for  informational purposes  only
    //
    DWORD EstimatedTimePerRequest;

    //
    // Specifies the number of requests that should be
    // kept outstanding.  This helps  keep the  device
    // device busy and thus obtain maximum throughput.
    // This will only be filled in if the target  file
    // has an outstanding contract.
    //
    DWORD NumOutStandingRequests;

    //
    // Specifies the required size of requests in this
    // stream.  This  will  only  be filled in  if the
    // target file has an outstanding contract.
    //
    DWORDLONG RequestSize;

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
    DWORD Version;

    //
    // Specifies the number of requests that
    // need to  complete  per period of time
    //
    DWORD RequestsPerPeriod;

    //
    // Specifies the period of time wherein the
    // above  number of requests  must complete
    //
    DWORD Period;

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
    DWORD AccessType;

    //
    // Indicates whether the  Io  to the
    // file will be sequential or random
    //
    DWORD AccessMode;

} STORAGE_ALLOCATE_BC_STREAM_INPUT, *PSTORAGE_ALLOCATE_BC_STREAM_INPUT;

typedef struct _STORAGE_ALLOCATE_BC_STREAM_OUTPUT {

    //
    // Specifies the required size
    // of  requests in this stream
    //
    DWORDLONG RequestSize;

    //
    // Specifies the number of requests that should be
    // kept outstanding.  This helps  keep the  device
    // device busy and thus obtain maximum  throughput
    //
    DWORD NumOutStandingRequests;

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
    DWORD SupportFlags;
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
    DWORD Version;

    // Whole size of the structure and the associated data buffer.
    // (In case adding variable-sized buffer in future.)
    DWORD Size;

    // Request flag.
    DWORD Flags;

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
    DWORD Version;

    // Whole size of the structure and the associated data buffer.
    DWORD Size;

    // GUID of diagnostic data provider.
    GUID ProviderId;

    // If the request failed because of buffer too small, this field should be filled with the required buffer
    // size for DiagnosticDataBuffer needed by provider;
    // if the request is successful, it should be filled with returned buffer size of DiagnosticDataBuffer;
    // it should be cleared to zero for other cases.
    DWORD BufferSize;

    // Reserved for future use.
    DWORD Reserved;

    // Diagnostic data buffer.
    _Field_size_(BufferSize) BYTE  DiagnosticDataBuffer[ANYSIZE_ARRAY];

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

    DWORD Version;
    DWORD Size;

    DWORD StartingElement;
    BYTE  Filter;
    BYTE  ReportType;
    BYTE  Reserved[2];

} PHYSICAL_ELEMENT_STATUS_REQUEST, *PPHYSICAL_ELEMENT_STATUS_REQUEST;

typedef struct _PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {

    DWORD Version;
    DWORD Size;

    DWORD ElementIdentifier;
    BYTE  PhysicalElementType;
    BYTE  PhysicalElementHealth;
    BYTE  Reserved1[2];

    // In unit of LBA.
    DWORDLONG AssociatedCapacity;

    DWORD Reserved2[4];

} PHYSICAL_ELEMENT_STATUS_DESCRIPTOR, *PPHYSICAL_ELEMENT_STATUS_DESCRIPTOR;

typedef struct _PHYSICAL_ELEMENT_STATUS {

    DWORD Version;
    DWORD Size;

    DWORD DescriptorCount;
    DWORD ReturnedDescriptorCount;

    DWORD ElementIdentifierBeingDepoped;
    DWORD Reserved;

    PHYSICAL_ELEMENT_STATUS_DESCRIPTOR Descriptors[ANYSIZE_ARRAY];

} PHYSICAL_ELEMENT_STATUS, *PPHYSICAL_ELEMENT_STATUS;

//
// IOCTL_STORAGE_REMOVE_ELEMENT_AND_TRUNCATE
//
// Input:
//       REMOVE_ELEMENT_AND_TRUNCATE_REQUEST
//

typedef struct _REMOVE_ELEMENT_AND_TRUNCATE_REQUEST {

    DWORD Version;
    DWORD Size;

    // In unit of LBA.
    DWORDLONG RequestCapacity;

    DWORD ElementIdentifier;
    DWORD Reserved;

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

    DWORD Version;
    DWORD Size;

    DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE RequestDataType;
    DEVICE_INTERNAL_STATUS_DATA_SET RequestDataSet;

} GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST, *PGET_DEVICE_INTERNAL_STATUS_DATA_REQUEST;

typedef struct _DEVICE_INTERNAL_STATUS_DATA {

    // Size of this structure.
    DWORD Version;

    // Whole size of the structure and the associated data buffer.
    DWORD Size;

    DWORDLONG T10VendorId;

    DWORD DataSet1Length;
    DWORD DataSet2Length;
    DWORD DataSet3Length;
    DWORD DataSet4Length;

    BYTE  StatusDataVersion;
    BYTE  Reserved[3];
    BYTE  ReasonIdentifier[128];
    DWORD StatusDataLength;
    BYTE  StatusData[ANYSIZE_ARRAY];

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

    DWORD Version;

    DWORD Size;

    DWORD TimeoutInSeconds;

    //
    // The SanitizeOption field is only applicable to NVMe devices.
    //
    struct {

        //
        // This field specifies the sanitize method defined in STORAGE_SANITIZE_METHOD enum.
        //
        DWORD SanitizeMethod : 4;

        //
        // This field specifies if unrestricted sanitize exit is allowed or not.
        // By default unrestricted sanitize exit is allowed.
        //
        DWORD DisallowUnrestrictedSanitizeExit : 1;

        DWORD Reserved : 27;
    } SanitizeOption;

} STORAGE_REINITIALIZE_MEDIA, *PSTORAGE_REINITIALIZE_MEDIA;

#pragma warning(pop)


#pragma warning(push)
#pragma warning(disable:4200)

#if defined(_MSC_EXTENSIONS)

typedef struct _STORAGE_MEDIA_SERIAL_NUMBER_DATA {

    WORD   Reserved;

    //
    // the SerialNumberLength will be set to zero
    // if the command is supported and the media
    // does not have a valid serial number.
    //

    WORD   SerialNumberLength;

    //
    // the following data is binary, and is not guaranteed
    // to be NULL terminated.  this is an excercise for the
    // caller.
    //

#if !defined(__midl)
    BYTE  SerialNumber[0];
#endif

} STORAGE_MEDIA_SERIAL_NUMBER_DATA, *PSTORAGE_MEDIA_SERIAL_NUMBER_DATA;

#endif /* _MSC_EXTENSIONS */

typedef _Struct_size_bytes_(Size) struct _STORAGE_READ_CAPACITY {

    //
    // The version number, size of the STORAGE_READ_CAPACITY structure
    //
    DWORD Version;

    //
    // The size of the date returned, size of the STORAGE_READ_CAPACITY structure
    //
    DWORD Size;

    //
    // Number of bytes per block
    //

    DWORD BlockLength;

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

    DWORD Version;

    //
    // The size of the date returned
    // Size of STORAGE_WRITE_CACHE_PROPERTY structure
    //

    DWORD Size;

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

    DWORD Version;
    DWORD Size;

    union {

        struct {

            //
            // Persistent Reserve service action.
            //

            BYTE  ServiceAction : 5;
            BYTE  Reserved1 : 3;

            //
            // Number of bytes allocated for returned parameter list.
            //

            WORD   AllocationLength;

        } PR_IN;

        struct {

            //
            // Persistent Reserve service action.
            //

            BYTE  ServiceAction : 5;
            BYTE  Reserved1 : 3;

            //
            // Persistent Reserve type and scope.
            //

            BYTE  Type : 4;
            BYTE  Scope : 4;

            //
            // Space for additional PR Out parameters.
            //

#if !defined(__midl)
            BYTE  ParameterList[0];
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
#define DEVICEDUMP_MAX_IDSTRING 32       // Keep proportional to sizeof (DWORD)
#define MAX_FW_BUCKET_ID_LENGTH 132     // 128 (ACS specification + 1 for zero termination + 3 to align on DWORD)


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
        DWORD    dwSize;        // Size (in bytes) of the subsection
        DWORD    dwFlags;       // Parameter flags for the subsection
        DWORD    dwOffset;      // Offset (in bytes) of the subsection block from the start of the buffer
} DEVICEDUMP_SUBSECTION_POINTER,*PDEVICEDUMP_SUBSECTION_POINTER;

//
// Data structure tagging fields (version and size)
//
typedef struct _DEVICEDUMP_STRUCTURE_VERSION {
        //
        // Header signature, useful for identifying the structure when reading the dump
        //
        DWORD   dwSignature;

        //
        // Version of the template
        //
        DWORD   dwVersion;

        //
        // Size of the parent structure in bytes
        //
        DWORD   dwSize;

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
        BYTE     sOrganizationID[16];

        //
        // Firmware revision as indicated in IDENITFY or INQUIRY structures
        //
        DWORD    dwFirmwareRevision;

        //
        // Device model number (keep the length of the field proportional to sizeof (DWORD))
        //
        BYTE    sModelNumber[DEVICEDUMP_MAX_IDSTRING];

        //
        // Vendor specific device cookie, identifying process and manufacturing parameters. Opaque to the OS and applications.
        //
        BYTE    szDeviceManufacturingID[DEVICEDUMP_MAX_IDSTRING];       // Keep the length of the field proportional to sizeof (DWORD)

        //
        // Sourcing indicator flag - used to detect if data was emulated from other structures or obtained directly from the firmware using log command
        //      Set to 1 if public data was filled in using data from the device telemetry log
        //      Set to 0 if the device doesn't support the command and the driver filled in as best it could
        //
        DWORD  dwFlags;

        //
        // Version of private data as indicated by the firmware.Initially always 0 to specify Private only unspecified data
        //
        DWORD bRestrictedPrivateDataVersion;

        //
        // Issue identifier (hash value) generated by the firmware. Reflects state of the device firmware and used for cross device type/vendor queries.
        // We will rely on standardized namespace of issue IDs and good will of firmware developers to taxonomize
        //
        DWORD dwFirmwareIssueId;                //currently unused

        //
        // Firmware bucket ID - long string, opague to Windows , but useful to create unique bucket in concatenation with device identification data
        //
        BYTE     szIssueDescriptionString[MAX_FW_BUCKET_ID_LENGTH];      // zero terminated

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
        WORD    LogAddress;
        WORD    LogSectors;
} GP_LOG_PAGE_DESCRIPTOR,*PGP_LOG_PAGE_DESCRIPTOR;

typedef struct _DEVICEDUMP_PUBLIC_SUBSECTION {
        DWORD   dwFlags;
        GP_LOG_PAGE_DESCRIPTOR  GPLogTable[TC_PUBLIC_DEVICEDUMP_CONTENT_GPLOG_MAX];
        CHAR    szDescription[TC_DEVICEDUMP_SUBSECTION_DESC_LENGTH]; // Zero terminated
        BYTE    bData[ANYSIZE_ARRAY];      // Data byte array ANYSIZE_ARRAY
} DEVICEDUMP_PUBLIC_SUBSECTION, *PDEVICEDUMP_PUBLIC_SUBSECTION;

//
//  Restricted subsection header - subsection is holding data, describing device state and accessible only to Microsoft and a device vendor
//
typedef struct _DEVICEDUMP_RESTRICTED_SUBSECTION {

        BYTE    bData[ANYSIZE_ARRAY];           // Data byte array (ANYSIZE_ARRAY)

} DEVICEDUMP_RESTRICTED_SUBSECTION, *PDEVICEDUMP_RESTRICTED_SUBSECTION;

//
//  Private subsection header - subsection is holding data, describing device state and accessible only to a device vendor
//
typedef struct _DEVICEDUMP_PRIVATE_SUBSECTION {

        DWORD   dwFlags;
        GP_LOG_PAGE_DESCRIPTOR GPLogId;

        BYTE    bData[ANYSIZE_ARRAY];   // Data byte array (ANYSIZE_ARRAY)

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
        DWORD   dwBufferSize;

        //
        // Reason for collecting telemetry
        //
        DWORD   dwReasonForCollection;

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
        BYTE      Cdb[CDB_SIZE];

        // The actual command for this request.
        BYTE      Command[TELEMETRY_COMMAND_SIZE];

        // the time when driver received the request
        DWORDLONG StartTime;

        // the system time when the request was completed
        DWORDLONG EndTime;

        // Status value ()
        DWORD  OperationStatus;

        // Error value (eg error reg for ATAPort, SCSI error for storport)
        DWORD   OperationError;

        // Stack specific information
        union {
         struct {
                DWORD dwReserved;
                } ExternalStack;

         struct {
                DWORD   dwAtaPortSpecific;
         }  AtaPort;

         struct {
                DWORD   SrbTag  ;
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
        DWORD   dwReasonForCollection;

        //
        // Driver stack and instance
        //
        BYTE    cDriverName[16];

        //
        // Standardized log of IO requests issued to the target, starting with number of records.
        // Log is circular, order is not guaranteed
        //
        DWORD   uiNumRecords;

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
    DWORD Version;              // Structure version, should be set to 1 for Win8.
    DWORD Size;                 // Size of this structure in bytes.
    DWORD WakeCapableHint : 1;  // Storage device supports wake from low power states.
    DWORD D3ColdSupported : 1;  // Storage device supports D3Cold
    DWORD Reserved : 30;
    DWORD D3IdleTimeout;        // Idle time in msec before storage device is transitioned to D3 (max of ~49.7 days).
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
    DWORD Version;                          // Structure version, should be set to 1 for Win8.
    DWORD Size;                             // Size of this structure in bytes.
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
    DWORD Version;
    DWORD Size;
    STORAGE_DEVICE_POWER_CAP_UNITS Units;
    DWORDLONG MaxPower;
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
    BYTE  Stuff[196];

    //
    // Either the key to be programmed or the MAC authenticating this frame or series of frames
    //
    BYTE  KeyOrMAC[32];

    //
    // The data input or output
    //
    BYTE  Data[256];

    //
    // Random 128-bit number generated by host
    //
    BYTE  Nonce[16];

    //
    // 32-bit counter
    //
    BYTE  WriteCounter[4];

    //
    // The half-sector address to operate on
    //
    BYTE  Address[2];

    //
    // The count of half-sector blocks to read/write
    //
    BYTE  BlockCount[2];

    //
    // The result of the operation
    //
    BYTE  OperationResult[2];

    //
    // The type of request or response
    //
    BYTE  RequestOrResponseType[2];

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
    DWORD Version;                          // Structure version, should be set to 1 for Win8.
    DWORD Size;                             // Size of this structure in bytes.
    DWORDLONG Events;                       // Bitmask of event(s) that occurred.
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

#define ReadCopyNumberToKey(_c)                 (READ_COPY_NUMBER_KEY | (BYTE )(_c))
#define ReadCopyNumberFromKey(_k)               (BYTE )((_k) & 0x000000FF)


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
            DWORD Week;

            //
            // Year is the last two digits of the year, e.g. 2016 is simply "16".
            //
            DWORD Year;
        } ManufactureDate;

        DWORDLONG AsUlonglong;
    } Value;

} STORAGE_COUNTER, *PSTORAGE_COUNTER;

typedef _Struct_size_bytes_(Size) struct _STORAGE_COUNTERS {

    //
    // Size of this structure serves as the version.
    //
    DWORD Version;

    //
    // Total size of this structure plus all the variable-sized fields.
    //
    DWORD Size;

    DWORD NumberOfCounters;

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
    DWORD   Version;            // sizeof(STORAGE_FIRMWARE_INFO_QUERY)
    DWORD   Size;               // Whole size of the buffer (in case this data structure being extended to be variable length)
    DWORD   Flags;
    DWORD   Reserved;
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

    DWORD   Version;            // sizeof(STORAGE_HW_FIRMWARE_SLOT_INFO)

    DWORD   Size;               // size the data contained in STORAGE_HW_FIRMWARE_SLOT_INFO.

    BYTE    SlotNumber;

    BYTE    ReadOnly : 1;

    BYTE    Reserved0 : 7;

    BYTE    Reserved1[6];

    BYTE    Revision[STORAGE_HW_FIRMWARE_REVISION_LENGTH];

} STORAGE_HW_FIRMWARE_SLOT_INFO, *PSTORAGE_HW_FIRMWARE_SLOT_INFO;

typedef struct _STORAGE_HW_FIRMWARE_INFO {

    DWORD   Version;            // sizeof(STORAGE_HW_FIRMWARE_INFO)

    DWORD   Size;               // size of the whole buffer including slot[]

    BYTE    SupportUpgrade : 1;

    BYTE    Reserved0 : 7;

    BYTE    SlotCount;

    BYTE    ActiveSlot;

    BYTE    PendingActivateSlot;

    BOOLEAN FirmwareShared;     // The firmware applies to both device and adapter. For example: PCIe SSD.

    BYTE    Reserved[3];

    DWORD   ImagePayloadAlignment;  // Number of bytes. Max: PAGE_SIZE. The transfer size should be multiple of this unit size. Some protocol requires at least sector size. 0 means the value is not valid.

    DWORD   ImagePayloadMaxSize;    // for a single command.

    STORAGE_HW_FIRMWARE_SLOT_INFO Slot[ANYSIZE_ARRAY];

} STORAGE_HW_FIRMWARE_INFO, *PSTORAGE_HW_FIRMWARE_INFO;
#pragma warning(pop)


//
// Input parameter for IOCTL_STORAGE_FIRMWARE_DOWNLOAD
//
#pragma warning(push)
#pragma warning(disable:4200)

typedef struct _STORAGE_HW_FIRMWARE_DOWNLOAD {

    DWORD       Version;            // sizeof(STORAGE_HW_FIRMWARE_DOWNLOAD)
    DWORD       Size;               // size of the whole buffer include "ImageBuffer"

    DWORD       Flags;
    BYTE        Slot;               // Slot number that firmware image will be downloaded into.
    BYTE        Reserved[3];

    DWORDLONG   Offset;             // Image file offset, should be aligned to "ImagePayloadAlignment" value from STORAGE_FIRMWARE_INFO.
    DWORDLONG   BufferSize;         // should be multiple of "ImagePayloadAlignment" value from STORAGE_FIRMWARE_INFO.

    BYTE        ImageBuffer[ANYSIZE_ARRAY];     // firmware image file.

} STORAGE_HW_FIRMWARE_DOWNLOAD, *PSTORAGE_HW_FIRMWARE_DOWNLOAD;

typedef struct _STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {

    DWORD       Version;            // sizeof(STORAGE_HW_FIRMWARE_DOWNLOAD_V2)
    DWORD       Size;               // size of the whole buffer include "ImageBuffer"

    DWORD       Flags;
    BYTE        Slot;               // Slot number that firmware image will be downloaded into.
    BYTE        Reserved[3];

    DWORDLONG   Offset;             // Image file offset, should be aligned to "ImagePayloadAlignment" value from STORAGE_FIRMWARE_INFO.
    DWORDLONG   BufferSize;         // should be multiple of "ImagePayloadAlignment" value from STORAGE_FIRMWARE_INFO.

    DWORD       ImageSize;          // Firmware Image size.
    DWORD       Reserved2;

    BYTE        ImageBuffer[ANYSIZE_ARRAY];     // firmware image file.

} STORAGE_HW_FIRMWARE_DOWNLOAD_V2, *PSTORAGE_HW_FIRMWARE_DOWNLOAD_V2;

#pragma warning(pop)

//
// Input parameter for IOCTL_STORAGE_FIRMWARE_ACTIVATE
//
typedef struct _STORAGE_HW_FIRMWARE_ACTIVATE {

    DWORD   Version;
    DWORD   Size;

    DWORD   Flags;
    BYTE    Slot;                   // Slot with firmware image to be activated.
    BYTE    Reserved0[3];

} STORAGE_HW_FIRMWARE_ACTIVATE, *PSTORAGE_HW_FIRMWARE_ACTIVATE;

//
// Parameter for IOCTL_STORAGE_PROTOCOL_COMMAND
// Buffer layout: <STORAGE_PROTOCOL_COMMAND> <Command> [Error Info Buffer] [Data-to-Device Buffer] [Data-from-Device Buffer]
//
#define STORAGE_PROTOCOL_STRUCTURE_VERSION              0x1

typedef struct _STORAGE_PROTOCOL_COMMAND {

    DWORD   Version;                        // STORAGE_PROTOCOL_STRUCTURE_VERSION
    DWORD   Length;                         // sizeof(STORAGE_PROTOCOL_COMMAND)

    STORAGE_PROTOCOL_TYPE  ProtocolType;
    DWORD   Flags;                          // Flags for the request

    DWORD   ReturnStatus;                   // return value
    DWORD   ErrorCode;                      // return value, optional

    DWORD   CommandLength;                  // non-zero value should be set by caller
    DWORD   ErrorInfoLength;                // optional, can be zero
    DWORD   DataToDeviceTransferLength;     // optional, can be zero. Used by WRITE type of request.
    DWORD   DataFromDeviceTransferLength;   // optional, can be zero. Used by READ type of request.

    DWORD   TimeOutValue;                   // in unit of seconds

    DWORD   ErrorInfoOffset;                // offsets need to be pointer aligned
    DWORD   DataToDeviceBufferOffset;       // offsets need to be pointer aligned
    DWORD   DataFromDeviceBufferOffset;     // offsets need to be pointer aligned

    DWORD   CommandSpecific;                // optional information passed along with Command.
    DWORD   Reserved0;

    DWORD   FixedProtocolReturnData;        // return data, optional. Some protocol, such as NVMe, may return a small amount data (DWORD0 from completion queue entry) without the need of separate device data transfer.
    DWORD   FixedProtocolReturnData2;       // return data2, optional. Some protocol, such as NVMe, may return a small amount data (DWORD1 from completion queue entry) without the need of separate device data transfer.
    DWORD   Reserved1[2];

    _Field_size_bytes_full_(CommandLength) BYTE  Command[ANYSIZE_ARRAY];

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
    DWORD Version;

    //
    // Size of  this structure  plus
    // all the variable sized fields.
    //
    DWORD Size;

    //
    // Indicates what action is requested.
    //
    STORAGE_ATTRIBUTE_MGMT_ACTION Action;

    //
    // The attribute on which specified "Action"
    // needs to be taken.
    //
    DWORD Attribute;

} STORAGE_ATTRIBUTE_MGMT, *PSTORAGE_ATTRIBUTE_MGMT;



#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#if defined __cplusplus && !defined __ALT_GENERATOR__
}
#endif

#endif // _NTDDSTOR_H_

#ifndef _NTDDSCM_H_
#define _NTDDSCM_H_

#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/union
#pragma warning(disable:4214) // bit field types other than int


#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)

//
// Functions 0 to 0x2FF are reserved for the bus device.
// Functions 0x300 to 0x5FF are reserved for the logical persistent memory device.
// Functions 0x600 to 0x7FF are reserved for the physical persistent memory device.
// Functions 0x800 and above are reserved for non-Microsoft users.
//

#define IOCTL_SCMBUS_BASE FILE_DEVICE_PERSISTENT_MEMORY

#define IOCTL_SCMBUS_DEVICE_FUNCTION_BASE           0x0
#define IOCTL_SCM_LOGICAL_DEVICE_FUNCTION_BASE      0x300
#define IOCTL_SCM_PHYSICAL_DEVICE_FUNCTION_BASE     0x600

#define SCMBUS_FUNCTION(x)              (IOCTL_SCMBUS_DEVICE_FUNCTION_BASE + x)
#define SCM_LOGICAL_DEVICE_FUNCTION(x)  (IOCTL_SCM_LOGICAL_DEVICE_FUNCTION_BASE + x)
#define SCM_PHYSICAL_DEVICE_FUNCTION(x) (IOCTL_SCM_PHYSICAL_DEVICE_FUNCTION_BASE + x)

//
// Persistent memory (SCM) bus device IOCTLs.
//

#define IOCTL_SCM_BUS_GET_LOGICAL_DEVICES           CTL_CODE(IOCTL_SCMBUS_BASE, SCMBUS_FUNCTION(0x00), METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCM_BUS_GET_PHYSICAL_DEVICES          CTL_CODE(IOCTL_SCMBUS_BASE, SCMBUS_FUNCTION(0x01), METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCM_BUS_GET_REGIONS                   CTL_CODE(IOCTL_SCMBUS_BASE, SCMBUS_FUNCTION(0x02), METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCM_BUS_QUERY_PROPERTY                CTL_CODE(IOCTL_SCMBUS_BASE, SCMBUS_FUNCTION(0x03), METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCM_BUS_SET_PROPERTY                  CTL_CODE(IOCTL_SCMBUS_BASE, SCMBUS_FUNCTION(0x05), METHOD_BUFFERED, FILE_WRITE_ACCESS)

//
// This IOCTL does not require any input nor produce any output data.
//
#define IOCTL_SCM_BUS_RUNTIME_FW_ACTIVATE           CTL_CODE(IOCTL_SCMBUS_BASE, SCMBUS_FUNCTION(0x04), METHOD_BUFFERED, FILE_WRITE_ACCESS)
#define IOCTL_SCM_BUS_REFRESH_NAMESPACE             CTL_CODE(IOCTL_SCMBUS_BASE, SCMBUS_FUNCTION(0x06), METHOD_BUFFERED, FILE_ANY_ACCESS)


//
// Logical persistent memory device IOCTLs.
//
#define IOCTL_SCM_LD_GET_INTERLEAVE_SET             CTL_CODE(IOCTL_SCMBUS_BASE, SCM_LOGICAL_DEVICE_FUNCTION(0x00), METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// IOCTLs exposed by physical persistent memory device objects.
//

#define IOCTL_SCM_PD_QUERY_PROPERTY                 CTL_CODE(IOCTL_SCMBUS_BASE, SCM_PHYSICAL_DEVICE_FUNCTION(0x00), METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCM_PD_FIRMWARE_DOWNLOAD              CTL_CODE(IOCTL_SCMBUS_BASE, SCM_PHYSICAL_DEVICE_FUNCTION(0x01), METHOD_BUFFERED, FILE_WRITE_ACCESS)
#define IOCTL_SCM_PD_FIRMWARE_ACTIVATE              CTL_CODE(IOCTL_SCMBUS_BASE, SCM_PHYSICAL_DEVICE_FUNCTION(0x02), METHOD_BUFFERED, FILE_WRITE_ACCESS)
#define IOCTL_SCM_PD_PASSTHROUGH                    CTL_CODE(IOCTL_SCMBUS_BASE, SCM_PHYSICAL_DEVICE_FUNCTION(0x03), METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_SCM_PD_UPDATE_MANAGEMENT_STATUS       CTL_CODE(IOCTL_SCMBUS_BASE, SCM_PHYSICAL_DEVICE_FUNCTION(0x04), METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCM_PD_REINITIALIZE_MEDIA             CTL_CODE(IOCTL_SCMBUS_BASE, SCM_PHYSICAL_DEVICE_FUNCTION(0x05), METHOD_BUFFERED, FILE_WRITE_ACCESS)
#define IOCTL_SCM_PD_SET_PROPERTY                   CTL_CODE(IOCTL_SCMBUS_BASE, SCM_PHYSICAL_DEVICE_FUNCTION(0x06), METHOD_BUFFERED, FILE_WRITE_ACCESS)


//
// The payload for a physical device health notification.
//
typedef struct _SCM_PD_HEALTH_NOTIFICATION_DATA {

    //
    // The GUID of the device reporting the health change.
    // This is the same GUID returned by IOCTL_SCM_PD_QUERY_PROPERTY with
    // ScmPhysicalDeviceProperty_DeviceInfo.
    //
    GUID  DeviceGuid;

} SCM_PD_HEALTH_NOTIFICATION_DATA, *PSCM_PD_HEALTH_NOTIFICATION_DATA;


//
// IOCTL_SCM_BUS_GET_LOGICAL_DEVICES
//
// Send this IOCTL to the ScmBus adapter to get a list of all the logical persistent memory
// devices on the system.
//
// Input Buffer:
//      None
//
// Output Buffer:
//      SCM_LOGICAL_DEVICES
//

#define SCM_MAX_SYMLINK_LEN_IN_CHARS 256

typedef struct _SCM_LOGICAL_DEVICE_INSTANCE {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // The total size of the data structure.
    //
    DWORD Size;

    //
    // The logical device GUID.
    //
    GUID DeviceGuid;

    //
    // Symbolic link that can be used to get a handle to the device.
    //
    WCHAR SymbolicLink[SCM_MAX_SYMLINK_LEN_IN_CHARS];

} SCM_LOGICAL_DEVICE_INSTANCE, *PSCM_LOGICAL_DEVICE_INSTANCE;

typedef struct _SCM_LOGICAL_DEVICES {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // The total size of the data structure, including all the elements in the
    // Devices array.
    //
    DWORD Size;

    //
    // The number of valid elements in the Devices array.
    //
    DWORD DeviceCount;

    //
    // Array of logical device instances.
    //
    SCM_LOGICAL_DEVICE_INSTANCE Devices[ANYSIZE_ARRAY];

} SCM_LOGICAL_DEVICES, *PSCM_LOGICAL_DEVICES;

//
// IOCTL_SCM_BUS_GET_PHYSICAL_DEVICES
//
// Send this IOCTL to the ScmBus adapter to get a list of all the physical persistent memory
// devices on the system.
//
// Input Buffer:
//      None
//
// Output Buffer:
//      SCM_PHYSICAL_DEVICES
//
typedef struct _SCM_PHYSICAL_DEVICE_INSTANCE {
    
    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // The total size of the data structure.
    //
    DWORD Size;

    //
    // The NFIT handle of the physical device.
    //
    DWORD NfitHandle;

    //
    // Symbolic link that can be used to get a handle on the device.
    //
    WCHAR SymbolicLink[SCM_MAX_SYMLINK_LEN_IN_CHARS];

} SCM_PHYSICAL_DEVICE_INSTANCE, *PSCM_PHYSICAL_DEVICE_INSTANCE;

typedef struct _SCM_PHYSICAL_DEVICES {
    
    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // The total size of the data structure, including all the elements in the
    // Devices array.
    //
    DWORD Size;

    //
    // The number of valid elements in the Devices array.
    //
    DWORD DeviceCount;

    //
    // Array of physical device instances.
    //
    SCM_PHYSICAL_DEVICE_INSTANCE Devices[ANYSIZE_ARRAY];

} SCM_PHYSICAL_DEVICES, *PSCM_PHYSICAL_DEVICES;

//
// IOCTL_SCM_BUS_GET_REGIONS
//
// Send to a logical persistent memory device stack to get a list of all regions that make up
// the logical device.
//
// Send to a physical persistent memory device stack to get a list of all the regions that
// reside on that physical device.
//
// Input Buffer:
//      None
//
// Output Buffer:
//      SCM_REGIONS
//

typedef enum _SCM_REGION_FLAG {
    ScmRegionFlagNone = 0x0,

    //
    // Indicates this region is described by a label.
    //
    ScmRegionFlagLabel = 0x1

} SCM_REGION_FLAG, *PSCM_REGION_FLAG;

#define SCM_REGION_SPA_UNKNOWN MAXDWORD64

typedef struct _SCM_REGION {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // The total size of the data structure.
    //
    DWORD Size;

    //
    // Bitmask of SCM_REGION_FLAG values.
    //
    DWORD Flags;

    //
    // The NFIT handle of the physical device for this region.
    //
    DWORD NfitHandle;

    //
    // The GUID of the logical device for this region, if any.
    //
    GUID LogicalDeviceGuid;

    //
    // The address range type (e.g. byte-addressable persistent memory).
    //
    GUID AddressRangeType;

    //
    // Regions that are associated with each other (e.g. part of an interleave
    // set) will share an associated ID.
    //
    DWORD AssociatedId;

    //
    // The total size of the region, in bytes.
    //
    DWORD64 Length;

    //
    // The starting device physical address of the region
    // within the physical device.
    //
    DWORD64 StartingDPA;

    //
    // The base system physical address.
    //
    DWORD64 BaseSPA;

    //
    // The region's offset from the base system physical address.
    // This field may be SCM_REGION_SPA_UNKNOWN if there is not enough
    // context to calculate the SPA offset for this particular region.
    //
    DWORD64 SPAOffset;

    //
    // The value of the Region Offset field from the associated Region Mapping
    // Structure.
    //
    DWORD64 RegionOffset;

} SCM_REGION, *PSCM_REGION;

typedef struct _SCM_REGIONS {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // The total size of the data structure, including all the elements in the
    // Regions array.
    //
    DWORD Size;

    //
    // The number of valid elements in the Regions array.
    //
    DWORD RegionCount;

    //
    // Array of regions for the logical or physical device.
    //
    SCM_REGION Regions[ANYSIZE_ARRAY];

} SCM_REGIONS, *PSCM_REGIONS;

//
// IOCTL_SCM_BUS_QUERY_PROPERTY
//
// Input Buffer:
//      An SCM_BUS_PROPERTY_QUERY structure that describes the type of query
//      being done, the property being queried, and any additional parameters
//      the query requires.
//
//  Output Buffer:
//      Contains a buffer to place the results of the query into. Since all
//      property descriptors can be cast into an SCM_BUS_DESCRIPTOR_HEADER,
//      the IOCTL can be called once with a small buffer then again using
//      a buffer as large as the header reports is necessary.
//


//
// Types of queries
//

typedef enum _SCM_BUS_QUERY_TYPE {
    ScmBusQuery_Descriptor = 0, // Retrieves the descriptor
    ScmBusQuery_IsSupported,    // Used to test whether the descriptor is supported

    ScmBusQuery_Max
} SCM_BUS_QUERY_TYPE, *PSCM_BUS_QUERY_TYPE;


//
// IOCTL_SCM_BUS_SET_PROPERTY
//
// Input Buffer:
//      An SCM_BUS_PROPERTY_SET structure that describes the type of set
//      being done, the property being set, and any additional parameters
//      the set requires.
//
//  Output Buffer:
//      Contains a buffer to place the results of the set into. Since all
//      property descriptors can be cast into an SCM_BUS_DESCRIPTOR_HEADER,
//      the IOCTL can be called once with a small buffer then again using
//      a buffer as large as the header reports is necessary.
//


//
// Types of sets
//

typedef enum _SCM_BUS_SET_TYPE {
    ScmBusSet_Descriptor = 0, // Retrieves the descriptor
    ScmBusSet_IsSupported,    // Used to test whether the descriptor is supported

    ScmBusSet_Max
} SCM_BUS_SET_TYPE, *PSCM_BUS_SET_TYPE;


typedef enum _SCM_BUS_PROPERTY_ID {

    //
    // Runtime Firmware Activation Information.
    //
    ScmBusProperty_RuntimeFwActivationInfo = 0,

    //
    // Dedicated Memory Information.
    //
    ScmBusProperty_DedicatedMemoryInfo = 1,

    //
    // Activate/Deactivate the Dedicated Memory.
    //
    ScmBusProperty_DedicatedMemoryState = 2,

    ScmBusProperty_Max
} SCM_BUS_PROPERTY_ID, *PSCM_BUS_PROPERTY_ID;

//
// Query structure - additional parameters for specific queries can follow
// the header
//

typedef struct _SCM_BUS_PROPERTY_QUERY {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // The size of this structure, including any additional
    // parameters.
    //
    DWORD Size;

    //
    // ID of the property being retrieved.
    //
    SCM_BUS_PROPERTY_ID PropertyId;

    //
    // The type of query being performed.
    //
    SCM_BUS_QUERY_TYPE QueryType;

    //
    // Space for additional parameters if necessary.
    //
    BYTE  AdditionalParameters[ANYSIZE_ARRAY];

} SCM_BUS_PROPERTY_QUERY, *PSCM_BUS_PROPERTY_QUERY;

//
// Output buffer for ScmBusProperty_RuntimeFwActivationInfo
//

//
// ScmBus Firmware Activation State
//
typedef enum _SCM_BUS_FIRMWARE_ACTIVATION_STATE {
    ScmBusFirmwareActivationState_Idle = 0,     // NVDIMM is Idle for firmware update
    ScmBusFirmwareActivationState_Armed = 1,    // NVDIMM is armed to activate the staging firmware
    ScmBusFirmwareActivationState_Busy = 2      // NVDIMM firmware activation is underway

} SCM_BUS_FIRMWARE_ACTIVATION_STATE, *PSCM_BUS_FIRMWARE_ACTIVATION_STATE;

typedef struct _SCM_BUS_RUNTIME_FW_ACTIVATION_INFO {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // Size of the data contained in this structure. If the output buffer is too small
    // to contain the requested information, the Size field indicates the length of the
    // output buffer the caller should provide in order to retrieve all the data.
    //
    DWORD Size;

    //
    // Indicates if runtime firmware activation is supported or not.
    //
    BOOLEAN RuntimeFwActivationSupported;

    //
    // Indicates the current Firmware activation state of the DIMMs.
    // Note: If any one of the NVDIMMs is Armed, the state is Armed.
    //
    SCM_BUS_FIRMWARE_ACTIVATION_STATE FirmwareActivationState;

    //
    // Firmware activation capabilities.
    //
    struct {

        //
        // Live activation supported with platform firmware managed processor and I/O quiesce.
        //
        DWORD FwManagedIoQuiesceFwActivationSupported : 1;

        //
        // Live activation supported with OS managed I/O quiesce (device idle) and platform managed processor quiesce.
        //
        DWORD OsManagedIoQuiesceFwActivationSupported : 1;

        //
        // Warm reset-based activation supported.
        //
        DWORD WarmResetBasedFwActivationSupported : 1;

        DWORD Reserved : 29;
    } FirmwareActivationCapability;

    //
    // Estimated firmware activation time in micro seconds.
    //
    DWORDLONG EstimatedFirmwareActivationTimeInUSecs;

    //
    // Estimated processor quiesce time during firmware activation in micro seconds.
    // 0 - no processor quiesce required.
    //
    DWORDLONG EstimatedProcessorAccessQuiesceTimeInUSecs;

    //
    // Estimated I/O access to host memory quiesce time during firmware activation in micro seconds.
    // 0 - no I/O quiesce required.
    //
    DWORDLONG EstimatedIOAccessQuiesceTimeInUSecs;

    //
    // Platform firmware supported Max I/O access to memory quiesce time during firmware activation in micro seconds.
    // 0 - Informaiton not available.
    //
    DWORDLONG PlatformSupportedMaxIOAccessQuiesceTimeInUSecs;

} SCM_BUS_RUNTIME_FW_ACTIVATION_INFO, *PSCM_BUS_RUNTIME_FW_ACTIVATION_INFO;

//
// Output buffer for ScmBusProperty_DedicatedMemoryInfo
//
typedef struct _SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {

    //
    // The dedicated memory device GUID.
    //
    GUID DeviceGuid;

    //
    // The dedicated memory device number.
    //
    DWORD DeviceNumber;

    struct {

        //
        // Indicates if the dedicated memory is created by registry settings.
        //
        DWORD ForcedByRegistry : 1;

        //
        // Indicates if the dedicated memory is initialized.
        //
        DWORD Initialized : 1;

        DWORD Reserved : 30;
    } Flags;

    //
    // The dedicated memory device size in bytes.
    //
    DWORDLONG DeviceSize;

} SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO, *PSCM_BUS_DEDICATED_MEMORY_DEVICE_INFO;

typedef struct _SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // The total size of the data structure, including all the elements in the
    // Devices array.
    //
    DWORD Size;

    //
    // The number of valid elements in the Devices array.
    //
    DWORD DeviceCount;

    //
    // Array of dedicated memory devices info.
    //
    SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO Devices[ANYSIZE_ARRAY];

} SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO, *PSCM_BUS_DEDICATED_MEMORY_DEVICES_INFO;


//
// Set structure - additional parameters for specific sets can follow
// the header
//

typedef struct _SCM_BUS_PROPERTY_SET {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // The size of this structure, including any additional
    // parameters.
    //
    DWORD Size;

    //
    // ID of the property being set.
    //
    SCM_BUS_PROPERTY_ID PropertyId;

    //
    // The type of set being performed.
    //
    SCM_BUS_SET_TYPE SetType;

    //
    // Space for additional parameters if necessary.
    //
    BYTE  AdditionalParameters[ANYSIZE_ARRAY];

} SCM_BUS_PROPERTY_SET, *PSCM_BUS_PROPERTY_SET;

//
// Input AdditionalParameters for ScmBusProperty_DedicatedMemoryState
//

typedef struct _SCM_BUS_DEDICATED_MEMORY_STATE {

    //
    // Dedicated Memory state (Deactivate - FALSE, Activate - TRUE).
    //
    BOOLEAN ActivateState;
} SCM_BUS_DEDICATED_MEMORY_STATE, *PSCM_BUS_DEDICATED_MEMORY_STATE;


//
// Definitions for interfaces related to logical persistent memory devices (LDs).
//

//
// IOCTL_SCM_LD_GET_INTERLEAVE_SET
//
// This IOCTL retrieves the interleave set of a logical persistent memory disk. The interleave set
// is comprised of one or more physical persistent memory devices.
//
// Input Buffer:
//      None.
//
// Output Buffer:
//      SCM_LD_INTERLEAVE_SET_INFO
//

typedef struct _SCM_INTERLEAVED_PD_INFO {

    //
    // An identifier for the physical device that comes from the NFIT table and
    // is unique on the local system.
    //
    DWORD DeviceHandle;

    //
    // A GUID that uniquely identifies the physical device on the system.
    //
    GUID DeviceGuid;

} SCM_INTERLEAVED_PD_INFO, *PSCM_INTERLEAVED_PD_INFO;

typedef struct _SCM_LD_INTERLEAVE_SET_INFO {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;
    
    //
    // Total size of the structure, in bytes, including the InterleaveSet array.
    // If the output buffer is too small to contain the requested information,
    // the Size field indicates the length of the output buffer the caller should provide
    // in order to retrieve all the data.
    //
    DWORD Size;

    //
    // The number of elements in the InterleaveSet array.
    //
    DWORD InterleaveSetSize;

    //
    // Information about the physical devices that make up this interleave
    // set.
    //
    SCM_INTERLEAVED_PD_INFO InterleaveSet[ANYSIZE_ARRAY];

} SCM_LD_INTERLEAVE_SET_INFO, *PSCM_LD_INTERLEAVE_SET_INFO;


//
// Definitions for interfaces related to physical persistent memory devices (PDs).
//

//
// IOCTL_SCM_PD_QUERY_PROPERTY
//
// Input Buffer:
//      An SCM_PD_PROPERTY_QUERY structure that describes the type of query
//      being done, the property being queried, and any additional parameters
//      the query requires.
//
//  Output Buffer:
//      Contains a buffer to place the results of the query into. Since all
//      property descriptors can be cast into an SCM_PD_DESCRIPTOR_HEADER,
//      the IOCTL can be called once with a small buffer then again using
//      a buffer as large as the header reports is necessary.
//


//
// Types of queries
//

typedef enum _SCM_PD_QUERY_TYPE {
    ScmPhysicalDeviceQuery_Descriptor = 0, // Retrieves the descriptor
    ScmPhysicalDeviceQuery_IsSupported, // Used to test whether the descriptor is supported

    ScmPhysicalDeviceQuery_Max
} SCM_PD_QUERY_TYPE, *PSCM_PD_QUERY_TYPE;


//
// IOCTL_SCM_PD_SET_PROPERTY
//
// Input Buffer:
//      An SCM_PD_PROPERTY_SET structure that describes the type of set
//      being done, the property being set, and any additional parameters
//      the set requires.
//
//  Output Buffer:
//      Contains a buffer to place the results of the set into. Since all
//      property descriptors can be cast into an SCM_PD_DESCRIPTOR_HEADER,
//      the IOCTL can be called once with a small buffer then again using
//      a buffer as large as the header reports is necessary.
//


//
// Types of sets
//

typedef enum _SCM_PD_SET_TYPE {
    ScmPhysicalDeviceSet_Descriptor = 0, // Retrieves the descriptor
    ScmPhysicalDeviceSet_IsSupported, // Used to test whether the descriptor is supported

    ScmPhysicalDeviceSet_Max
} SCM_PD_SET_TYPE, *PSCM_PD_SET_TYPE;


typedef enum _SCM_PD_PROPERTY_ID {

    //
    // General information about the device.
    //
    ScmPhysicalDeviceProperty_DeviceInfo = 0,

    //
    // Information about the device's health.
    //
    ScmPhysicalDeviceProperty_ManagementStatus,

    //
    // Firmware-related information.
    //
    ScmPhysicalDeviceProperty_FirmwareInfo,

    //
    // Returns a string that identifies where the device is located
    // on the local system.
    //
    ScmPhysicalDeviceProperty_LocationString,

    //
    // Returns a series of device-specific information, which give more detail
    // on the device's status.
    //
    ScmPhysicalDeviceProperty_DeviceSpecificInfo,

    //
    // Returns a identifying handle of the physical device, which comes from
    // the NFIT table.
    //
    ScmPhysicalDeviceProperty_DeviceHandle,

    //
    // Returns a string that identifies the replacement unit housing
    // the device on the local system.
    //
    ScmPhysicalDeviceProperty_FruIdString,

    //
    // Returns runtime firmware activation information.
    //
    ScmPhysicalDeviceProperty_RuntimeFwActivationInfo,

    //
    // Runtime firmware activation arm state.
    //
    ScmPhysicalDeviceProperty_RuntimeFwActivationArmState,

    ScmPhysicalDeviceProperty_Max
} SCM_PD_PROPERTY_ID, *PSCM_PD_PROPERTY_ID;


//
// Query structure - additional parameters for specific queries can follow
// the header
//

typedef struct _SCM_PD_PROPERTY_QUERY {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // The size of this structure, including any additional
    // parameters.
    //
    DWORD Size;

    //
    // ID of the property being retrieved.
    //
    SCM_PD_PROPERTY_ID PropertyId;

    //
    // The type of query being performed.
    //
    SCM_PD_QUERY_TYPE QueryType;

    //
    // Space for additional parameters if necessary.
    //
    BYTE  AdditionalParameters[ANYSIZE_ARRAY];

} SCM_PD_PROPERTY_QUERY, *PSCM_PD_PROPERTY_QUERY;

//
// Set structure - additional parameters for specific sets can follow
// the header
//

typedef struct _SCM_PD_PROPERTY_SET {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // The size of this structure, including any additional
    // parameters.
    //
    DWORD Size;

    //
    // ID of the property being retrieved.
    //
    SCM_PD_PROPERTY_ID PropertyId;

    //
    // The type of set being performed.
    //
    SCM_PD_SET_TYPE SetType;

    //
    // Space for additional parameters if necessary.
    //
    BYTE  AdditionalParameters[ANYSIZE_ARRAY];

} SCM_PD_PROPERTY_SET, *PSCM_PD_PROPERTY_SET;

//
// Input AdditionalParameters for ScmPhysicalDeviceProperty_RuntimeFwActivationArmState
//

typedef struct _SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE {

    //
    // Runtime firmware activation arm state (Disarm - FALSE, Arm - TRUE).
    //
    BOOLEAN ArmState;
} SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE, *PSCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE;

//
// Standard property descriptor header. All property pages should use this
// as their first element or should contain these two elements
//

typedef struct _SCM_PD_DESCRIPTOR_HEADER {

    //
    // The sizeof() of the entire descriptor (not just the header).
    //
    DWORD Version;

    //
    // The size of the entire descriptor (not just the header).
    //
    DWORD Size;
} SCM_PD_DESCRIPTOR_HEADER, *PSCM_PD_DESCRIPTOR_HEADER;

//
// Output buffer for ScmPhysicalDeviceProperty_DeviceHandle & ScmPhysicalDeviceQuery_Descriptor
//

//
// The ScmPhysicalDeviceProperty_DeviceHandle property gets identifying information about
// a physical device.
//
typedef struct _SCM_PD_DEVICE_HANDLE {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // The total size of the structure.
    //
    DWORD Size;

    //
    // A GUID that uniquely identifies the physical device, based on hardware information.
    //
    GUID DeviceGuid;

    //
    // A handle, exposed in the NFIT table, that uniquely identifies the physical device on a local
    // system.
    //
    DWORD DeviceHandle;

} SCM_PD_DEVICE_HANDLE, *PSCM_PD_DEVICE_HANDLE;

//
// Output buffer for ScmPhysicalDeviceProperty_DeviceInfo & ScmPhysicalDeviceQuery_Descriptor
//

#define MAX_INTERFACE_CODES 8
#define SCM_PD_FIRMWARE_REVISION_LENGTH_BYTES 32

#define SCM_PD_MEMORY_SIZE_UNKNOWN MAXDWORD64

typedef struct _SCM_PD_DEVICE_INFO {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;
    
    //
    // The total size of the structure, including the serial number at the end.
    // If the output buffer is too small to contain the requested information,
    // the Size field indicates the length of the output buffer the caller should provide
    // in order to retrieve all the data.
    //
    DWORD Size;
    
    //
    // A GUID that uniquely identifies the physical device, based on hardware information.
    //
    GUID DeviceGuid;
    
    //
    // The number of times this device went through an unsafe shutdown (i.e. a shutdown
    // that might have led to data loss).
    //
    DWORD UnsafeShutdownCount;

        
    //
    // The combined size of all the persistent memory regions of the physical device.
    //
    DWORD64 PersistentMemorySizeInBytes;

    //
    // The total size of the volatile memory this device contains, if any.
    // May be SCM_PD_MEMORY_SIZE_UNKNOWN if it is not reported by the platform.
    //
    DWORD64 VolatileMemorySizeInBytes;

    //
    // The total capacity of this memory device, including persistent and any
    // volatile memory.
    // May be SCM_PD_MEMORY_SIZE_UNKNOWN if it is not reported by the platform.
    //
    DWORD64 TotalMemorySizeInBytes;
    
    //
    // The number of the slot in which the physical device is installed on the system.
    //
    DWORD SlotNumber;
    
    //
    // A handle, exposed in the NFIT table, that uniquely identifies the physical device on a local
    // system.
    //
    DWORD DeviceHandle;

    //
    // The unique ID for this physical device as indicated in the SMBIOS.
    //
    WORD   PhysicalId;

    //
    // An physical device can have regions that implement different format interface
    // codes. This is a list of all format interface codes on this physical device.
    //
    BYTE   NumberOfFormatInterfaceCodes;
    WORD   FormatInterfaceCodes[MAX_INTERFACE_CODES];

    //
    // Vendor and product IDs.
    //
    DWORD VendorId;
    DWORD ProductId;
    DWORD SubsystemDeviceId;
    DWORD SubsystemVendorId;
    BYTE  ManufacturingLocation;
    BYTE  ManufacturingWeek; // *Not* in BCD format.
    BYTE  ManufacturingYear; // *Not* in BCD format.
    DWORD SerialNumber4Byte; // 4-byte serial number as defined in the JEDEC SPD spec and reported in the NFIT.

    //
    // The physical device's serial number, as a string.
    //
    DWORD SerialNumberLengthInChars;
    _Field_size_(SerialNumberLengthInChars) CHAR SerialNumber[ANYSIZE_ARRAY];
} SCM_PD_DEVICE_INFO, *PSCM_PD_DEVICE_INFO;

//
// Output buffer for ScmPhysicalDeviceProperty_DeviceSpecificInfo & ScmPhysicalDeviceQuery_Descriptor
//

//
// A device specific property is a key-value pair where the key is a string
// and the value is a number.
//
#define SCM_PD_PROPERTY_NAME_LENGTH_IN_CHARS 128

typedef struct _SCM_PD_DEVICE_SPECIFIC_PROPERTY {
    // NULL-terminated string.
    WCHAR Name[SCM_PD_PROPERTY_NAME_LENGTH_IN_CHARS];
    LONGLONG Value;
} SCM_PD_DEVICE_SPECIFIC_PROPERTY, *PSCM_PD_DEVICE_SPECIFIC_PROPERTY;

//
// The physical device driver fills in this structure with arbitrary numeric information.
//
typedef struct _SCM_PD_DEVICE_SPECIFIC_INFO {
    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // The total size of the structure, including the DeviceSpecificProperties array.
    // If the output buffer is too small to contain the requested information,
    // the Size field indicates the length of the output buffer the caller should provide
    // in order to retrieve all the data.
    //
    DWORD Size;

    //
    // The number of elements in the DeviceSpecificProperties array.
    //
    DWORD NumberOfProperties;
    
    //
    // A series of device-specific properties filled in by the driver.
    //
    SCM_PD_DEVICE_SPECIFIC_PROPERTY DeviceSpecificProperties[ANYSIZE_ARRAY];
} SCM_PD_DEVICE_SPECIFIC_INFO, *PSCM_PD_DEVICE_SPECIFIC_INFO;

typedef struct _SCM_PD_FIRMWARE_SLOT_INFO {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD   Version;
    
    //
    // Size of the data contained in this structure.
    //
    DWORD   Size;

    BYTE    SlotNumber;
    BYTE    ReadOnly : 1;
    BYTE    Reserved0 : 7;
    BYTE    Reserved1[6];

    BYTE    Revision[SCM_PD_FIRMWARE_REVISION_LENGTH_BYTES];

} SCM_PD_FIRMWARE_SLOT_INFO, *PSCM_PD_FIRMWARE_SLOT_INFO;


//
// Output buffer for ScmPhysicalDeviceQuery_Descriptor & ScmPhysicalDeviceProperty_FirmwareInfo
//
typedef struct _SCM_PD_FIRMWARE_INFO {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;
    
    //
    // Size of the data contained in this structure, including the Slots
    // array. If the output buffer is too small to contain the requested information,
    // the Size field indicates the length of the output buffer the caller should provide
    // in order to retrieve all the data.
    //
    DWORD Size;


    //
    // The firmware slot that is currently active.
    // 
    BYTE  ActiveSlot;

    //
    // The slot that will become active once the device is reset. A value of 0xFF means
    // there is no slot waiting to be activated.
    //
    BYTE  NextActiveSlot;

    BYTE  SlotCount;
    
    _Field_size_(SlotCount) SCM_PD_FIRMWARE_SLOT_INFO Slots[ANYSIZE_ARRAY];

} SCM_PD_FIRMWARE_INFO, *PSCM_PD_FIRMWARE_INFO;


//
// Constants for ScmPhysicalDeviceProperty_ManagementStatus, which can be queried via
// ScmPhysicalDeviceQuery_Descriptor.
//

//
// Health states.
//
typedef enum _SCM_PD_HEALTH_STATUS {
    ScmPhysicalDeviceHealth_Unknown = 0,
    ScmPhysicalDeviceHealth_Unhealthy,
    ScmPhysicalDeviceHealth_Warning,
    ScmPhysicalDeviceHealth_Healthy,

    ScmPhysicalDeviceHealth_Max
} SCM_PD_HEALTH_STATUS, *PSCM_PD_HEALTH_STATUS;

//
// Operational states.
//
typedef enum _SCM_PD_OPERATIONAL_STATUS {
    ScmPhysicalDeviceOpStatus_Unknown = 0,
    ScmPhysicalDeviceOpStatus_Ok,
    ScmPhysicalDeviceOpStatus_PredictingFailure,
    ScmPhysicalDeviceOpStatus_InService,
    ScmPhysicalDeviceOpStatus_HardwareError,
    ScmPhysicalDeviceOpStatus_NotUsable,
    ScmPhysicalDeviceOpStatus_TransientError,
    ScmPhysicalDeviceOpStatus_Missing,

    ScmPhysicalDeviceOpStatus_Max
} SCM_PD_OPERATIONAL_STATUS, *PSCM_PD_OPERATIONAL_STATUS;

//
// Operational reasons.
//
typedef enum _SCM_PD_OPERATIONAL_STATUS_REASON {
    ScmPhysicalDeviceOpReason_Unknown = 0,
    ScmPhysicalDeviceOpReason_Media,
    ScmPhysicalDeviceOpReason_ThresholdExceeded,
    ScmPhysicalDeviceOpReason_LostData,
    ScmPhysicalDeviceOpReason_EnergySource,
    ScmPhysicalDeviceOpReason_Configuration,
    ScmPhysicalDeviceOpReason_DeviceController,
    ScmPhysicalDeviceOpReason_MediaController,
    ScmPhysicalDeviceOpReason_Component,
    ScmPhysicalDeviceOpReason_BackgroundOperation,
    ScmPhysicalDeviceOpReason_InvalidFirmware,
    ScmPhysicalDeviceOpReason_HealthCheck,
    ScmPhysicalDeviceOpReason_LostDataPersistence,
    ScmPhysicalDeviceOpReason_DisabledByPlatform,
    ScmPhysicalDeviceOpReason_PermanentError,
    ScmPhysicalDeviceOpReason_LostWritePersistence,
    ScmPhysicalDeviceOpReason_FatalError,
    ScmPhysicalDeviceOpReason_DataPersistenceLossImminent,
    ScmPhysicalDeviceOpReason_WritePersistenceLossImminent,
    ScmPhysicalDeviceOpReason_MediaRemainingSpareBlock,
    ScmPhysicalDeviceOpReason_PerformanceDegradation,
    ScmPhysicalDeviceOpReason_ExcessiveTemperature,
    ScmPhysicalDeviceOpReason_InternalFailure,

    ScmPhysicalDeviceOpReason_Max
} SCM_PD_OPERATIONAL_STATUS_REASON, *PSCM_PD_OPERATIONAL_STATUS_REASON;

//
// Output buffer for ScmPhysicalDeviceProperty_ManagementStatus & ScmPhysicalDeviceQuery_Descriptor
//

#define SCM_PD_MAX_OPERATIONAL_STATUS    16

typedef struct _SCM_PD_MANAGEMENT_STATUS {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // The total size of the structure, including operational status reasons
    // that didn't fit in the caller's array. If the output buffer is too small to
    // contain the requested information, the Size field indicates the length of the
    // output buffer the caller should provide in order to retrieve all the data.
    //
    DWORD Size;

    //
    // Health status.
    //
    SCM_PD_HEALTH_STATUS Health;

    //
    // The number of operational statuses returned.
    //
    DWORD NumberOfOperationalStatus;

    //
    // The number of additional reasons returned.
    //
    DWORD NumberOfAdditionalReasons;

    //
    // Operational statuses. The primary operational status is the first element
    // in the array. There are NumberOfOperationalStatus valid elements in the array.
    //
    SCM_PD_OPERATIONAL_STATUS OperationalStatus[SCM_PD_MAX_OPERATIONAL_STATUS];

    //
    // Additional reasons. There are NumberOfAdditionalReasons valid elements in the array.
    //
    _Field_size_(NumberOfAdditionalReasons) SCM_PD_OPERATIONAL_STATUS_REASON AdditionalReasons[ANYSIZE_ARRAY];

} SCM_PD_MANAGEMENT_STATUS, *PSCM_PD_MANAGEMENT_STATUS;

//
// Output buffer for ScmPhysicalDeviceQuery_Descriptor & ScmPhysicalDeviceProperty_LocationString
//
typedef struct _SCM_PD_LOCATION_STRING {
    
    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // The total size of the structure, including the buffer for the Unicode
    // string. If the output buffer is too small to contain the requested information,
    // the Size field indicates the length of the output buffer the caller should provide
    // in order to retrieve all the data.
    //
    DWORD Size;

    //
    // The Unicode string that represents the physical location of this physical device.
    //
    WCHAR Location[ANYSIZE_ARRAY];

} SCM_PD_LOCATION_STRING, *PSCM_PD_LOCATION_STRING;

//
// Output buffer for ScmPhysicalDeviceQuery_Descriptor & ScmPhysicalDeviceProperty_FruIdString
//
typedef struct _SCM_PD_FRU_ID_STRING {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // The total size of the structure, including the buffer for the fru id string.
    // If the output buffer is too small to contain the requested information,
    // the Size field indicates the length of the output buffer the caller should provide
    // in order to retrieve all the data.
    //
    DWORD Size;

    //
    // The string that represents the fru id of this physical device.
    //
    DWORD IdentifierSize;
    BYTE  Identifier[ANYSIZE_ARRAY];

} SCM_PD_FRU_ID_STRING, *PSCM_PD_FRU_ID_STRING;

//
// Firmware update IOCTLs.
//

//
// Signals that the firmware image regions contained in the SCM_PD_FIRMWARE_DOWNLOAD
// structure are the last ones of the image. The physical device driver finishes the firmware update
// operation when this flag is set.
//
#define SCM_PD_FIRMWARE_LAST_DOWNLOAD 0x1

//
// Input buffer for IOCTL_SCM_PD_FIRMWARE_DOWNLOAD.
//
typedef struct _SCM_PD_FIRMWARE_DOWNLOAD {
    
    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;
    
    //
    // The structure size, including the image.
    //
    DWORD Size;
    
    //
    // Additional information about the region being download, such as whether it
    // is the last region in the image.
    //
    DWORD Flags;
    
    //
    // The firmware slot being upgraded.
    //
    BYTE  Slot;
    
    BYTE  Reserved[3];

    //
    // The offset of this region of the firmware image.
    //
    DWORD64 Offset;

    //
    // The size of the FirmwareImage array.
    //
    DWORD FirmwareImageSizeInBytes;

    //
    // The firmware region being downloaded to the device.
    //
    BYTE  FirmwareImage[ANYSIZE_ARRAY];

} SCM_PD_FIRMWARE_DOWNLOAD, *PSCM_PD_FIRMWARE_DOWNLOAD;

//
// Input buffer for IOCTL_SCM_PD_FIRMWARE_ACTIVATE.
//
typedef struct _SCM_PD_FIRMWARE_ACTIVATE {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // Total size of the structure
    //
    DWORD Size;

    //
    // Reserved. Callers should set to 0.
    //
    DWORD Flags;
    
    //
    // The slot that contains the firmware image being activated.
    //
    BYTE  Slot;

} SCM_PD_FIRMWARE_ACTIVATE, *PSCM_PD_FIRMWARE_ACTIVATE;

//
// Output buffer for ScmPhysicalDeviceProperty_RuntimeFwActivationInfo
//

//
// ScmBus Physical Device Last Firmware Activation Status
//
typedef enum _SCM_PD_LAST_FW_ACTIVATION_STATUS {
    ScmPdLastFwActivationStatus_None = 0,                   // No Firmware Activation performed. Reset to None on boot
    ScmPdLastFwActivationStatus_Success = 1,                // Success
    ScmPdLastFwActivationStatus_FwNotFound = 2,             // No new staged firmware to activate
    ScmPdLastFwActivationStatus_ColdRebootRequired = 3,     // NVDIMM Reset required to activate staged firmware
    ScmPdLastFwActivaitonStatus_ActivationInProgress = 4,   // Media disabled during firmware activation
    ScmPdLastFwActivaitonStatus_Retry = 5,                  // Activation aborted due to throttling. Retry recommended
    ScmPdLastFwActivaitonStatus_FwUnsupported = 6,          // Staged firmware version does not meet activation requirements
    ScmPdLastFwActivaitonStatus_UnknownError = 7            // Unclassified firmware activation error

} SCM_PD_LAST_FW_ACTIVATION_STATUS, *PSCM_PD_LAST_FW_ACTIVATION_STATUS;

//
// ScmBus Physical Device Firmware Activation State
//
typedef enum _SCM_PD_FIRMWARE_ACTIVATION_STATE {
    ScmPdFirmwareActivationState_Idle = 0,  // NVDIMM is Idle for firmware update
    ScmPdFirmwareActivationState_Armed = 1, // NVDIMM is armed to activate the staging firmware
    ScmPdFirmwareActivationState_Busy = 2   // NVDIMM firmware activation is underway

} SCM_PD_FIRMWARE_ACTIVATION_STATE, *PSCM_PD_FIRMWARE_ACTIVATION_STATE;

typedef struct _SCM_PD_RUNTIME_FW_ACTIVATION_INFO {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;

    //
    // Size of the data contained in this structure. If the output buffer is too small
    // to contain the requested information, the Size field indicates the length of the
    // output buffer the caller should provide in order to retrieve all the data.
    //
    DWORD Size;

    //
    // This provides the previous Firmware Activation status.
    //
    SCM_PD_LAST_FW_ACTIVATION_STATUS LastFirmwareActivationStatus;

    //
    // Indicates the current Firmware activation state of the DIMM.
    //
    SCM_PD_FIRMWARE_ACTIVATION_STATE FirmwareActivationState;

} SCM_PD_RUNTIME_FW_ACTIVATION_INFO, *PSCM_PD_RUNTIME_FW_ACTIVATION_INFO;

//
// IOCTL_SCM_PD_PASSTHROUGH
//
// This IOCTL sends a vendor-specific command to a physical device and returns its
// output.
//
// Input buffer:
//      SCM_PD_PASSTHROUGH_INPUT
//      The input structure contains another, physical device-type specific structure.
//
// Output buffer:
//      SCM_PD_PASSTHROUGH_OUTPUT
//      The output structure contains another, physical device-type specific structure.
//

typedef struct _SCM_PD_PASSTHROUGH_INPUT {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;
    
    //
    // The size of the structure, including the Data field, in bytes.
    //
    DWORD Size;

    //
    // This GUID defines which command protocol is being used. The driver will
    // check this field to make sure the application is sending commands for
    // device types that the driver understands.
    //
    GUID ProtocolGuid;

    //
    // The size, in bytes, of the data field.
    //
    DWORD DataSize;

    //
    // The physical device-type specific structure which contains the passthrough command.
    //
    BYTE  Data[ANYSIZE_ARRAY];
} SCM_PD_PASSTHROUGH_INPUT, *PSCM_PD_PASSTHROUGH_INPUT;

typedef struct _SCM_PD_PASSTHROUGH_OUTPUT {
    
    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;
    
    //
    // The size of the structure, including the Data field, in bytes.
    // The caller is responsible for knowing how large the output buffer
    // will be. The common approach of sending the IOCTL twice - once to learn
    // the total required size, and again to retrieve the data - isn't recommended
    // here because of the performance impact of executing a passthrough command.
    //
    DWORD Size;

    //
    // This GUID defines which command protocol is being used. The application should
    // check this field to make sure the driver is using a protocol that it understands.
    //
    GUID ProtocolGuid;

    //
    // The size, in bytes, of the data field.
    //
    DWORD DataSize;

    //
    // The physical device-type specific structure which contains the output of the passthrough command.
    //
    BYTE  Data[ANYSIZE_ARRAY];
} SCM_PD_PASSTHROUGH_OUTPUT, *PSCM_PD_PASSTHROUGH_OUTPUT;

//
// Passthrough structures and definitions for INVDIMM devices.
//

//
// This structure defines the input of an INVDIMM command. The application sending a passthrough
// command uses this structure as the "Data" field of the SCM_PD_PASSTHROUGH_INPUT structure.
//
typedef struct _SCM_PD_PASSTHROUGH_INVDIMM_INPUT {

    //
    // The command's opcode.
    //
    DWORD Opcode;

    //
    // The length, in bytes, of the OpcodeParameters field.
    // This can be zero, but the size of this structure must always be equal to
    // or greater than sizeof(SCM_PD_PASSTHROUGH_INVDIMM_INPUT).
    //
    DWORD OpcodeParametersLength;

    //
    // The opcode input payload, if any.
    //
    BYTE  OpcodeParameters[ANYSIZE_ARRAY];
} SCM_PD_PASSTHROUGH_INVDIMM_INPUT, *PSCM_PD_PASSTHROUGH_INVDIMM_INPUT;

//
// This structure defines the output of an INVDIMM command. The driver will respond to
// a passthrough request by bundling this structure in the "Data" field of the
// SCM_PD_PASSTHROUGH_OUTPUT structure.
//
typedef struct _SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {

    //
    // The general status of the command (see the INVDIMM _DSM specification for details).
    //
    WORD   GeneralStatus;

    //
    // The extended status of the command (see the INVDIMM _DSM specification for details).
    //
    WORD   ExtendedStatus;

    //
    // The length, in bytes, of the OutputData field. Even when this is zero, the total
    // size of this structure will be equal to or greater than sizeof(SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT).
    //
    DWORD OutputDataLength;

    //
    // The data returned by the device in response to the command.
    //
    BYTE  OutputData[ANYSIZE_ARRAY];
} SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT, *PSCM_PD_PASSTHROUGH_INVDIMM_OUTPUT;

//
// IOCTL_SCM_PD_REINITIALIZE_MEDIA
//
// This IOCTL reinitializes the media of a physical persistent memory device, which erases
// all the data on it.
//
// Input buffer:
//      SCM_PD_REINITIALIZE_MEDIA_INPUT
//
// Output buffer:
//      SCM_PD_REINITIALIZE_MEDIA_OUTPUT
//
typedef struct _SCM_PD_REINITIALIZE_MEDIA_INPUT {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;
    
    //
    // The size of the structure, in bytes.
    //
    DWORD Size;
    
    //
    // If Overwrite is set to 1, the physical persistent memory device will
    // overwrite the entire media, which might take several minutes.
    // If it is set to 0, the physical persistent device may do a crypto-erase or some
    // other quicker form of clearing the data on the media.
    //
    struct {
        DWORD Overwrite : 1;
    } Options;
} SCM_PD_REINITIALIZE_MEDIA_INPUT, *PSCM_PD_REINITIALIZE_MEDIA_INPUT;

typedef enum _SCM_PD_MEDIA_REINITIALIZATION_STATUS {

    //
    // The media reinitialization was successful and the device is ready for use.
    //
    ScmPhysicalDeviceReinit_Success = 0,
    
    //
    // The media reinitialization was successful, but the device requires a reboot before being used.
    //
    ScmPhysicalDeviceReinit_RebootNeeded,
    
    //
    // The media reinitialization was successful, but the device requires a cold boot before being used.
    //
    ScmPhysicalDeviceReinit_ColdBootNeeded,

    ScmPhysicalDeviceReinit_Max
} SCM_PD_MEDIA_REINITIALIZATION_STATUS, *PSCM_PD_MEDIA_REINITIALIZATION_STATUS;

typedef struct _SCM_PD_REINITIALIZE_MEDIA_OUTPUT {

    //
    // Sizeof() of this structure serves as the version.
    //
    DWORD Version;
    
    //
    // The size of the structure, in bytes.
    //
    DWORD Size;
    
    //
    // The detailed status of the reinitialization operation, in case it
    // was successful. If it failed, the IOCTL itself will fail and callers
    // should not look at the returned status code instead of this field.
    //
    SCM_PD_MEDIA_REINITIALIZATION_STATUS Status;
} SCM_PD_REINITIALIZE_MEDIA_OUTPUT, *PSCM_PD_REINITIALIZE_MEDIA_OUTPUT;


#endif // NTDDI_WIN10_RS5

#pragma warning(pop)


#endif // _NTDDSCM_H_


#ifndef _NTDDDISK_H_
#define _NTDDDISK_H_

#include <winapifamily.h>


#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#if defined(_MSC_VER)
#if (_MSC_VER >= 1200)
#pragma warning(push)
#pragma warning(disable:4201) // nonstandard extension used : nameless struct/union
#pragma warning(disable:4214) // nonstandard extension used : bitfield other than int
#pragma warning(disable:4820) // padding added after data member
#endif
#endif


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
//     IN DWORD PartitionType
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
//     IN DWORD PartitionType
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
//     IN DWORD PartitionType
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
   DWORD StartCylinderNumber;
   DWORD EndCylinderNumber;
   DWORD StartHeadNumber;
   DWORD EndHeadNumber;
} FORMAT_PARAMETERS, *PFORMAT_PARAMETERS;

//
// Define the BAD_TRACK_NUMBER type. An array of elements of this type is
// returned by the driver on IOCTL_DISK_FORMAT_TRACKS requests, to indicate
// what tracks were bad during formatting. The length of that array is
// reported in the `Information' field of the I/O Status Block.
//

typedef WORD   BAD_TRACK_NUMBER;
typedef WORD   *PBAD_TRACK_NUMBER;

//
// Define the input buffer structure for the driver, when
// it is called with IOCTL_DISK_FORMAT_TRACKS_EX.
//

typedef struct _FORMAT_EX_PARAMETERS {
   MEDIA_TYPE MediaType;
   DWORD StartCylinderNumber;
   DWORD EndCylinderNumber;
   DWORD StartHeadNumber;
   DWORD EndHeadNumber;
   WORD   FormatGapLength;
   WORD   SectorsPerTrack;
   WORD   SectorNumber[1];
} FORMAT_EX_PARAMETERS, *PFORMAT_EX_PARAMETERS;

//
// The following structure is returned on an IOCTL_DISK_GET_DRIVE_GEOMETRY
// request and an array of them is returned on an IOCTL_DISK_GET_MEDIA_TYPES
// request.
//

typedef struct _DISK_GEOMETRY {

    LARGE_INTEGER Cylinders;

    MEDIA_TYPE MediaType;

    DWORD TracksPerCylinder;

    DWORD SectorsPerTrack;

    DWORD BytesPerSector;

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
    DWORD HiddenSectors;
    DWORD PartitionNumber;
    BYTE  PartitionType;
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
    BYTE  PartitionType;
} SET_PARTITION_INFORMATION, *PSET_PARTITION_INFORMATION;

//
// The following structures is returned on an IOCTL_DISK_GET_DRIVE_LAYOUT
// request and given as input to an IOCTL_DISK_SET_DRIVE_LAYOUT request.
//

typedef struct _DRIVE_LAYOUT_INFORMATION {
    DWORD PartitionCount;
    DWORD Signature;
    PARTITION_INFORMATION PartitionEntry[1];
} DRIVE_LAYOUT_INFORMATION, *PDRIVE_LAYOUT_INFORMATION;

//
// The following structure is passed in on an IOCTL_DISK_VERIFY request.
// The offset and length parameters are both given in bytes.
//

typedef struct _VERIFY_INFORMATION {
    LARGE_INTEGER StartingOffset;
    DWORD Length;
} VERIFY_INFORMATION, *PVERIFY_INFORMATION;

//
// The following structure is passed in on an IOCTL_DISK_REASSIGN_BLOCKS
// request.
//

typedef struct _REASSIGN_BLOCKS {
    WORD   Reserved;
    WORD   Count;
    DWORD BlockNumber[1];
} REASSIGN_BLOCKS, *PREASSIGN_BLOCKS;

//
// The following structure is passed in on an IOCTL_DISK_REASSIGN_BLOCKS_EX
// request.
//

#include <pshpack1.h>
typedef struct _REASSIGN_BLOCKS_EX {
    WORD   Reserved;
    WORD   Count;
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

    DWORD64 Attributes;                 // See table 16-4.

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

    BYTE  PartitionType;

    BOOLEAN BootIndicator;

    BOOLEAN RecognizedPartition;

    DWORD HiddenSectors;

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
    DWORD MaxPartitionCount;        // Maximim number of partitions allowable.
} CREATE_DISK_GPT, *PCREATE_DISK_GPT;

//
// The structure CREATE_DISK_MBR with the ioctl IOCTL_DISK_CREATE_DISK
// to initialize a disk with an empty MBR partition table.
//

typedef struct _CREATE_DISK_MBR {
    DWORD Signature;
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

    DWORD PartitionNumber;

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

    DWORD MaxPartitionCount;

} DRIVE_LAYOUT_INFORMATION_GPT, *PDRIVE_LAYOUT_INFORMATION_GPT;


//
// MBR specific drive layout information.
//

typedef struct _DRIVE_LAYOUT_INFORMATION_MBR {

    DWORD Signature;

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)  /* ABRACADABRA_WIN10_RS1 */
    DWORD CheckSum;
#endif

} DRIVE_LAYOUT_INFORMATION_MBR, *PDRIVE_LAYOUT_INFORMATION_MBR;

//
// The structure DRIVE_LAYOUT_INFORMATION_EX is used with the
// IOCTL_SET_DRIVE_LAYOUT_EX and IOCTL_GET_DRIVE_LAYOUT_EX calls.
//

typedef struct _DRIVE_LAYOUT_INFORMATION_EX {

    DWORD PartitionStyle;

    DWORD PartitionCount;

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
        WORD   DriveSelect;
        DWORD MaxCylinders;
        WORD   SectorsPerTrack;
        WORD   MaxHeads;
        WORD   NumberDrives;
} DISK_INT13_INFO, *PDISK_INT13_INFO;

typedef struct _DISK_EX_INT13_INFO {
        WORD   ExBufferSize;
        WORD   ExFlags;
        DWORD ExCylinders;
        DWORD ExHeads;
        DWORD ExSectorsPerTrack;
        DWORD64 ExSectorsPerDrive;
        WORD   ExSectorSize;
        WORD   ExReserved;
} DISK_EX_INT13_INFO, *PDISK_EX_INT13_INFO;

#if (_MSC_VER >= 1200)
#pragma warning(push)
#pragma warning(disable:4201) // nonstandard extension used : nameless struct/union
#endif

typedef struct _DISK_DETECTION_INFO {
        DWORD SizeOfDetectInfo;
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
        DWORD SizeOfPartitionInfo;
        PARTITION_STYLE PartitionStyle;                 // PartitionStyle = RAW, GPT or MBR
        union {
                struct {                                                        // If PartitionStyle == MBR
                        DWORD Signature;                                // MBR Signature
                        DWORD CheckSum;                                 // MBR CheckSum
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
                        ((PDISK_DETECTION_INFO)(((DWORD_PTR)DiskGeometryGetPartition(Geometry)+\
                                        DiskGeometryGetPartition(Geometry)->SizeOfPartitionInfo)))
#endif
typedef struct _DISK_GEOMETRY_EX {
        DISK_GEOMETRY Geometry;                                 // Standard disk geometry: may be faked by driver.
        LARGE_INTEGER DiskSize;                                 // Must always be correct
        BYTE  Data[1];                                                  // Partition, Detect info
} DISK_GEOMETRY_EX, *PDISK_GEOMETRY_EX;

#endif // (_WIN32_WINNT > 0x0500)

#if(_WIN32_WINNT >= 0x0400)
//
// IOCTL_DISK_CONTROLLER_NUMBER returns the controller and disk
// number for the handle.  This is used to determine if a disk
// is attached to the primary or secondary IDE controller.
//

typedef struct _DISK_CONTROLLER_NUMBER {
    DWORD ControllerNumber;
    DWORD DiskNumber;
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

    WORD   DisablePrefetchTransferLength;

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
            WORD   Minimum;
            WORD   Maximum;

            //
            // The maximum number of blocks which will be prefetched - useful
            // with the scalar limits to set definite upper limits.
            //

            WORD   MaximumBlocks;
        } ScalarPrefetch;

        struct {
            WORD   Minimum;
            WORD   Maximum;
        } BlockPrefetch;
    } DUMMYUNIONNAME;

} DISK_CACHE_INFORMATION, *PDISK_CACHE_INFORMATION;


//
// IOCTL_DISK_GROW_PARTITION will update the size of a partition
// by adding sectors to the length. The number of sectors must be
// predetermined by examining PARTITION_INFORMATION.
//

typedef struct _DISK_GROW_PARTITION {
    DWORD PartitionNumber;
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
    DWORD       Reads;
    DWORD       Writes;
} HISTOGRAM_BUCKET, *PHISTOGRAM_BUCKET;

#define HISTOGRAM_BUCKET_SIZE   sizeof(HISTOGRAM_BUCKET)

typedef struct _DISK_HISTOGRAM {
    LARGE_INTEGER   DiskSize;
    LARGE_INTEGER   Start;
    LARGE_INTEGER   End;
    LARGE_INTEGER   Average;
    LARGE_INTEGER   AverageRead;
    LARGE_INTEGER   AverageWrite;
    DWORD           Granularity;
    DWORD           Size;
    DWORD           ReadCount;
    DWORD           WriteCount;
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
        DWORD ReadCount;
        DWORD WriteCount;
        DWORD QueueDepth;
        DWORD SplitCount;
        LARGE_INTEGER QueryTime;
        DWORD   StorageDeviceNumber;
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
   DWORD NumberOfBytes;
   BYTE  DeviceNumber;
   BOOLEAN ReadRequest;
} DISK_RECORD, *PDISK_RECORD;

//
// The following structure is exchanged on an IOCTL_DISK_LOG request.
// Not all fields are valid with each function type.
//

typedef struct _DISK_LOGGING {
    BYTE  Function;
    PVOID BufferAddress;
    DWORD BufferSize;
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
    DWORD NumberOfBins;
    DWORD TypeOfBin;
    BIN_RANGE BinsRanges[1];
} PERF_BIN, *PPERF_BIN ;

//
// Bin count
//

typedef struct _BIN_COUNT {
    BIN_RANGE BinRange;
    DWORD BinCount;
} BIN_COUNT, *PBIN_COUNT;

//
// Bin results
//

typedef struct _BIN_RESULTS {
    DWORD NumberOfBins;
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
        BYTE     bVersion;               // Binary driver version.
        BYTE     bRevision;              // Binary driver revision.
        BYTE     bReserved;              // Not used.
        BYTE     bIDEDeviceMap;          // Bit map of IDE devices.
        DWORD   fCapabilities;          // Bit mask of driver capabilities.
        DWORD   dwReserved[4];          // For future use.
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
        BYTE     bFeaturesReg;           // Used for specifying SMART "commands".
        BYTE     bSectorCountReg;        // IDE sector count register
        BYTE     bSectorNumberReg;       // IDE sector number register
        BYTE     bCylLowReg;             // IDE low order cylinder value
        BYTE     bCylHighReg;            // IDE high order cylinder value
        BYTE     bDriveHeadReg;          // IDE drive/head register
        BYTE     bCommandReg;            // Actual IDE command.
        BYTE     bReserved;                      // reserved for future use.  Must be zero.
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
        DWORD   cBufferSize;            // Buffer size in bytes
        IDEREGS irDriveRegs;            // Structure with drive register values.
        BYTE     bDriveNumber;           // Physical drive number to send
                                                                // command to (0,1,2,3).
        BYTE     bReserved[3];           // Reserved for future expansion.
        DWORD   dwReserved[4];          // For future use.
        BYTE     bBuffer[1];                     // Input buffer.
} SENDCMDINPARAMS, *PSENDCMDINPARAMS, *LPSENDCMDINPARAMS;
#include <poppack.h>

//
// Status returned from driver
//

#include <pshpack1.h>
typedef struct _DRIVERSTATUS {
        BYTE     bDriverError;           // Error code from driver,
                                                                // or 0 if no error.
        BYTE     bIDEError;                      // Contents of IDE Error register.
                                                                // Only valid when bDriverError
                                                                // is SMART_IDE_ERROR.
        BYTE     bReserved[2];           // Reserved for future expansion.
        DWORD   dwReserved[2];          // Reserved for future expansion.
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
        DWORD                   cBufferSize;            // Size of bBuffer in bytes
        DRIVERSTATUS            DriverStatus;           // Driver status structure.
        BYTE                    bBuffer[1];             // Buffer of arbitrary length in which to store the data read from the                                                                                  // drive.
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

//
// IOCTLs to query and modify attributes
// associated with the given disk. These
// are persisted within the registry.
//

#define IOCTL_DISK_GET_DISK_ATTRIBUTES      CTL_CODE(IOCTL_DISK_BASE, 0x003c, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_DISK_SET_DISK_ATTRIBUTES      CTL_CODE(IOCTL_DISK_BASE, 0x003d, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

#define DISK_ATTRIBUTE_OFFLINE              0x0000000000000001
#define DISK_ATTRIBUTE_READ_ONLY            0x0000000000000002
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
    DWORD Version;

    //
    // Reserved. Must ignore.
    //
    DWORD Reserved1;

    //
    // Specifies the attributes
    // associated with the disk.
    //
    DWORDLONG Attributes;

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
    DWORD Version;

    //
    // Indicates whether to remember
    // these settings across reboots
    // or not.
    //
    BOOLEAN Persist;

    //
    // Reserved. Must set to zero.
    //
    BYTE  Reserved1[3];

    //
    // Specifies the new attributes.
    //
    DWORDLONG Attributes;

    //
    // Specifies the attributes
    // that are being modified.
    //
    DWORDLONG AttributesMask;

    //
    // Reserved. Must set to zero.
    //
    DWORD Reserved2[4];

} SET_DISK_ATTRIBUTES, *PSET_DISK_ATTRIBUTES;

#define IOCTL_DISK_RESET_SNAPSHOT_INFO      CTL_CODE(IOCTL_DISK_BASE, 0x0084, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)


#if defined(_MSC_VER)
#if (_MSC_VER >= 1200)
#pragma warning(pop)
#endif
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // _NTDDDISK_H_


#define IOCTL_CHANGER_BASE                FILE_DEVICE_CHANGER

#define IOCTL_CHANGER_GET_PARAMETERS         CTL_CODE(IOCTL_CHANGER_BASE, 0x0000, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CHANGER_GET_STATUS             CTL_CODE(IOCTL_CHANGER_BASE, 0x0001, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CHANGER_GET_PRODUCT_DATA       CTL_CODE(IOCTL_CHANGER_BASE, 0x0002, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CHANGER_SET_ACCESS             CTL_CODE(IOCTL_CHANGER_BASE, 0x0004, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_CHANGER_GET_ELEMENT_STATUS     CTL_CODE(IOCTL_CHANGER_BASE, 0x0005, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_CHANGER_INITIALIZE_ELEMENT_STATUS  CTL_CODE(IOCTL_CHANGER_BASE, 0x0006, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CHANGER_SET_POSITION           CTL_CODE(IOCTL_CHANGER_BASE, 0x0007, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CHANGER_EXCHANGE_MEDIUM        CTL_CODE(IOCTL_CHANGER_BASE, 0x0008, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CHANGER_MOVE_MEDIUM            CTL_CODE(IOCTL_CHANGER_BASE, 0x0009, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CHANGER_REINITIALIZE_TRANSPORT CTL_CODE(IOCTL_CHANGER_BASE, 0x000A, METHOD_BUFFERED, FILE_READ_ACCESS)
#define IOCTL_CHANGER_QUERY_VOLUME_TAGS      CTL_CODE(IOCTL_CHANGER_BASE, 0x000B, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)



#define MAX_VOLUME_ID_SIZE       36
#define MAX_VOLUME_TEMPLATE_SIZE 40

#define VENDOR_ID_LENGTH          8
#define PRODUCT_ID_LENGTH        16
#define REVISION_LENGTH           4
#define SERIAL_NUMBER_LENGTH     32

//
// Common structures describing elements.
//

typedef  enum _ELEMENT_TYPE {
    AllElements,        // As defined by SCSI
    ChangerTransport,   // As defined by SCSI
    ChangerSlot,        // As defined by SCSI
    ChangerIEPort,      // As defined by SCSI
    ChangerDrive,       // As defined by SCSI
    ChangerDoor,        // Front panel, used to access internal of cabinet.
    ChangerKeypad,      // Keypad/input on front panel.
    ChangerMaxElement   // Placeholder only. Not a valid type.
} ELEMENT_TYPE, *PELEMENT_TYPE;

typedef  struct _CHANGER_ELEMENT {
    ELEMENT_TYPE    ElementType;
    DWORD   ElementAddress;
} CHANGER_ELEMENT, *PCHANGER_ELEMENT;

typedef  struct _CHANGER_ELEMENT_LIST {
    CHANGER_ELEMENT Element;
    DWORD   NumberOfElements;
} CHANGER_ELEMENT_LIST , *PCHANGER_ELEMENT_LIST;


//
// Definitions for  IOCTL_CHANGER_GET_PARAMETERS
//

//
// Definitions for Features0 of GET_CHANGER_PARAMETERS
//

#define CHANGER_BAR_CODE_SCANNER_INSTALLED  0x00000001 // The medium-changer has a bar code scanner installed.
#define CHANGER_INIT_ELEM_STAT_WITH_RANGE   0x00000002 // The medium-changer has the ability to initialize elements within a specified range.
#define CHANGER_CLOSE_IEPORT                0x00000004 // The medium-changer has the ability to close the i/e port door.
#define CHANGER_OPEN_IEPORT                 0x00000008 // The medium-changer can open the i/e port door.

#define CHANGER_STATUS_NON_VOLATILE         0x00000010 // The medium-changer uses non-volatile memory for element status information.
#define CHANGER_EXCHANGE_MEDIA              0x00000020 // The medium-changer supports exchange operations.
#define CHANGER_CLEANER_SLOT                0x00000040 // The medium-changer has a fixed slot designated for cleaner cartridges.
#define CHANGER_LOCK_UNLOCK                 0x00000080 // The medium-changer can be (un)secured to (allow)prevent media removal.

#define CHANGER_CARTRIDGE_MAGAZINE          0x00000100 // The medium-changer uses cartridge magazines for some storage slots.
#define CHANGER_MEDIUM_FLIP                 0x00000200 // The medium-changer can flip medium.
#define CHANGER_POSITION_TO_ELEMENT         0x00000400 // The medium-changer can position the transport to a particular element.
#define CHANGER_REPORT_IEPORT_STATE         0x00000800 // The medium-changer can determine whether media is present
                                                       // in the IE Port.

#define CHANGER_STORAGE_DRIVE               0x00001000 // The medium-changer can use a drive as an independent storage element.
#define CHANGER_STORAGE_IEPORT              0x00002000 // The medium-changer can use a i/e port as an independent storage element.
#define CHANGER_STORAGE_SLOT                0x00004000 // The medium-changer can use a slot as an independent storage element.
#define CHANGER_STORAGE_TRANSPORT           0x00008000 // The medium-changer can use a transport as an independent storage element.

#define CHANGER_DRIVE_CLEANING_REQUIRED     0x00010000 // The drives controlled by the medium changer require periodic cleaning
                                                       // initiated by an application.
#define CHANGER_PREDISMOUNT_EJECT_REQUIRED  0x00020000 // The medium-changer requires a drive eject command to be issued, before a changer
                                                       // move / exchange command can be issued to the drive.

#define CHANGER_CLEANER_ACCESS_NOT_VALID    0x00040000 // The access bit in GES isn't valid for cleaner cartridges.
#define CHANGER_PREMOUNT_EJECT_REQUIRED     0x00080000 // The medium-changer requires a drive eject command to be issued
                                                       // before a move / exchange command can be issued with the drive as src/dst.

#define CHANGER_VOLUME_IDENTIFICATION       0x00100000 // The medium-changer supports volume identification.
#define CHANGER_VOLUME_SEARCH               0x00200000 // The medium-changer can search for volume information.
#define CHANGER_VOLUME_ASSERT               0x00400000 // The medium-changer can verify volume information.
#define CHANGER_VOLUME_REPLACE              0x00800000 // The medium-changer can replace volume information.
#define CHANGER_VOLUME_UNDEFINE             0x01000000 // The medium-changer can undefine volume information.

#define CHANGER_SERIAL_NUMBER_VALID         0x04000000 // The serial number reported in GetProductData is valid
                                                       // and unique.

#define CHANGER_DEVICE_REINITIALIZE_CAPABLE 0x08000000 // The medium-changer can be issued a ChangerReinitializeUnit.
#define CHANGER_KEYPAD_ENABLE_DISABLE       0x10000000 // Indicates that the keypad can be enabled/disabled.
#define CHANGER_DRIVE_EMPTY_ON_DOOR_ACCESS  0x20000000 // Drives must be empty before access via the door is possible.

#define CHANGER_RESERVED_BIT                0x80000000 // Will be used to indicate Features1 capability bits.


//
// Definitions for Features1 of GET_CHANGER_PARAMETERS
//

#define CHANGER_PREDISMOUNT_ALIGN_TO_SLOT   0x80000001 // The transport must be prepositioned to the slot prior to ejecting the media.
#define CHANGER_PREDISMOUNT_ALIGN_TO_DRIVE  0x80000002 // The transport must be prepositioned to the drive prior to ejecting the media.
#define CHANGER_CLEANER_AUTODISMOUNT        0x80000004 // The device will move the cleaner cartridge back into the slot when cleaning has completed.
#define CHANGER_TRUE_EXCHANGE_CAPABLE       0x80000008 // Device can do src -> dest2 exchanges.
#define CHANGER_SLOTS_USE_TRAYS             0x80000010 // Slots have removable trays, requiring multiple moves for inject/eject.
#define CHANGER_RTN_MEDIA_TO_ORIGINAL_ADDR  0x80000020 // Media must be returned to the slot from which it originated after a move to another element.
#define CHANGER_CLEANER_OPS_NOT_SUPPORTED   0x80000040 // Automated cleaning operations are not supported on this device.
#define CHANGER_IEPORT_USER_CONTROL_OPEN    0x80000080 // Indicates that user action is necessary to open a closed ieport.
#define CHANGER_IEPORT_USER_CONTROL_CLOSE   0x80000100 // Indicates that user action is necessary to close an opened ieport.
#define CHANGER_MOVE_EXTENDS_IEPORT         0x80000200 // Indicates that a move media to the ieport extends the tray.
#define CHANGER_MOVE_RETRACTS_IEPORT        0x80000400 // Indicates that a move media from the ieport retracts the tray.


//
// Definitions for MoveFrom, ExchangeFrom, and PositionCapabilities
//

#define CHANGER_TO_TRANSPORT    0x01 // The device can carry out the operation to a transport from the specified element.
#define CHANGER_TO_SLOT         0x02 // The device can carry out the operation to a slot from the specified element.
#define CHANGER_TO_IEPORT       0x04 // The device can carry out the operation to an IE Port from the specified element.
#define CHANGER_TO_DRIVE        0x08 // The device can carry out the operation to a drive from the specified element.

//
// Definitions for LockUnlockCapabilities
//

#define LOCK_UNLOCK_IEPORT      0x01 // The device can lock/unlock the ieport(s).
#define LOCK_UNLOCK_DOOR        0x02 // The device can lock/unlock the door(s).
#define LOCK_UNLOCK_KEYPAD      0x04 // The device can lock/unlock the keypad.

typedef  struct _GET_CHANGER_PARAMETERS {

    //
    // Size of the structure. Can be used for versioning.
    //

    DWORD Size;

    //
    // Number of N element(s) as defined by the Element Address Page (or equivalent...).
    //

    WORD   NumberTransportElements;
    WORD   NumberStorageElements;                // for data cartridges only
    WORD   NumberCleanerSlots;                   // for cleaner cartridges
    WORD   NumberIEElements;
    WORD   NumberDataTransferElements;

    //
    // Number of doors/front panels (allows user entry into the cabinet).
    //

    WORD   NumberOfDoors;

    //
    // The device-specific address (from user manual of the device) of the first N element. Used
    // by the UI to relate the various elements to the user.
    //

    WORD   FirstSlotNumber;
    WORD   FirstDriveNumber;
    WORD   FirstTransportNumber;
    WORD   FirstIEPortNumber;
    WORD   FirstCleanerSlotAddress;

    //
    // Indicates the capacity of each magazine, if they exist.
    //

    WORD   MagazineSize;

    //
    // Specifies the approximate number of seconds for when a cleaning should be completed.
    // Only applicable if drive cleaning is supported. See Features0.
    //

    DWORD DriveCleanTimeout;

    //
    // See features bits, above.
    //

    DWORD Features0;
    DWORD Features1;

    //
    // Bitmask defining Move from N element to element. Defined by Device Capabilities Page (or equivalent).
    // AND-masking with the TO_XXX values will indicate legal destinations.
    //

    BYTE  MoveFromTransport;
    BYTE  MoveFromSlot;
    BYTE  MoveFromIePort;
    BYTE  MoveFromDrive;

    //
    // Bitmask defining Exchange from N element to element. Defined by Device Capabilities Page (or equivalent).
    // AND-masking with the TO_XXX values will indicate legal destinations.
    //

    BYTE  ExchangeFromTransport;
    BYTE  ExchangeFromSlot;
    BYTE  ExchangeFromIePort;
    BYTE  ExchangeFromDrive;

    //
    // Bitmask defining which elements are capable of lock/unlock. Valid only if
    // CHANGER_LOCK_UNLOCK is set in Features0.
    //

    BYTE  LockUnlockCapabilities;

    //
    // Bitmask defining which elements valid for positioning operations. Valid only if
    // CHANGER_POSITION_TO_ELEMENT is set in Features0.
    //

    BYTE  PositionCapabilities;

    //
    // For future expansion.
    //

    BYTE  Reserved1[2];
    DWORD Reserved2[2];

} GET_CHANGER_PARAMETERS, * PGET_CHANGER_PARAMETERS;


//
// Definitions for IOCTL_CHANGER_GET_PRODUCT_DATA
//

typedef  struct _CHANGER_PRODUCT_DATA {

    //
    // Device manufacturer's name - based on inquiry data
    //

    BYTE  VendorId[VENDOR_ID_LENGTH];

    //
    // Product identification as defined by the vendor - based on Inquiry data
    //

    BYTE  ProductId[PRODUCT_ID_LENGTH];

    //
    // Product revision as defined by the vendor.
    //

    BYTE  Revision[REVISION_LENGTH];

    //
    // Vendor unique value used to globally identify this device. Can
    // be from Vital Product Data, for example.
    //

    BYTE  SerialNumber[SERIAL_NUMBER_LENGTH];

    //
    // Indicates device type of data transports, as defined by SCSI-2.
    //

    BYTE  DeviceType;

} CHANGER_PRODUCT_DATA, *PCHANGER_PRODUCT_DATA;


//
// Definitions for IOCTL_CHANGER_SET_ACCESS
//

#define LOCK_ELEMENT        0
#define UNLOCK_ELEMENT      1
#define EXTEND_IEPORT       2
#define RETRACT_IEPORT      3

typedef struct _CHANGER_SET_ACCESS {

    //
    // Element can be ChangerIEPort, ChangerDoor, ChangerKeypad
    //

    CHANGER_ELEMENT Element;

    //
    // See above for possible operations.
    //

    DWORD           Control;
} CHANGER_SET_ACCESS, *PCHANGER_SET_ACCESS;


//
// Definitions for IOCTL_CHANGER_GET_ELEMENT_STATUS
//

//
// Input buffer.
//

typedef struct _CHANGER_READ_ELEMENT_STATUS {

    //
    // List describing the elements and range on which to return information.
    //

    CHANGER_ELEMENT_LIST ElementList;

    //
    // Indicates whether volume tag information is to be returned.
    //

    BOOLEAN VolumeTagInfo;
} CHANGER_READ_ELEMENT_STATUS, *PCHANGER_READ_ELEMENT_STATUS;

//
// Output buffer.
//

typedef  struct _CHANGER_ELEMENT_STATUS {

    //
    // Element to which this structure refers.
    //

    CHANGER_ELEMENT Element;

    //
    // Address of the element from which the media was originally moved.
    // Valid if ELEMENT_STATUS_SVALID bit of Flags DWORD is set.
    // Needs to be converted to a zero-based offset from the device-unique value.
    //

    CHANGER_ELEMENT SrcElementAddress;

    //
    // See below.
    //

    DWORD Flags;

    //
    // See below for possible values.
    //

    DWORD ExceptionCode;

    //
    // Scsi Target Id of this element.
    // Valid only if ELEMENT_STATUS_ID_VALID is set in Flags.
    //

    BYTE  TargetId;

    //
    // LogicalUnitNumber of this element.
    // Valid only if ELEMENT_STATUS_LUN_VALID is set in Flags.
    //

    BYTE  Lun;
    WORD   Reserved;

    //
    // Primary volume identification for the media.
    // Valid only if ELEMENT_STATUS_PVOLTAG bit is set in Flags.
    //

    BYTE  PrimaryVolumeID[MAX_VOLUME_ID_SIZE];

    //
    // Alternate volume identification for the media.
    // Valid for two-sided media only, and pertains to the id. of the inverted side.
    // Valid only if ELEMENT_STATUS_AVOLTAG bit is set in Flags.
    //

    BYTE  AlternateVolumeID[MAX_VOLUME_ID_SIZE];

} CHANGER_ELEMENT_STATUS, *PCHANGER_ELEMENT_STATUS;

//
// Output buffer. This is same as CHANGER_ELEMENT_STATUS with
// the addition of product info fields. New applications should
// use this struct instead of the older CHANGER_ELEMENT_STATUS
//

typedef  struct _CHANGER_ELEMENT_STATUS_EX {

    //
    // Element to which this structure refers.
    //

    CHANGER_ELEMENT Element;

    //
    // Address of the element from which the media was originally moved.
    // Valid if ELEMENT_STATUS_SVALID bit of Flags DWORD is set.
    // Needs to be converted to a zero-based offset from the device-unique value.
    //

    CHANGER_ELEMENT SrcElementAddress;

    //
    // See below.
    //

    DWORD Flags;

    //
    // See below for possible values.
    //

    DWORD ExceptionCode;

    //
    // Scsi Target Id of this element.
    // Valid only if ELEMENT_STATUS_ID_VALID is set in Flags.
    //

    BYTE  TargetId;

    //
    // LogicalUnitNumber of this element.
    // Valid only if ELEMENT_STATUS_LUN_VALID is set in Flags.
    //

    BYTE  Lun;
    WORD   Reserved;

    //
    // Primary volume identification for the media.
    // Valid only if ELEMENT_STATUS_PVOLTAG bit is set in Flags.
    //

    BYTE  PrimaryVolumeID[MAX_VOLUME_ID_SIZE];

    //
    // Alternate volume identification for the media.
    // Valid for two-sided media only, and pertains to the id. of the inverted side.
    // Valid only if ELEMENT_STATUS_AVOLTAG bit is set in Flags.
    //

    BYTE  AlternateVolumeID[MAX_VOLUME_ID_SIZE];

    //
    // Vendor ID
    //
    BYTE  VendorIdentification[VENDOR_ID_LENGTH];

    //
    // Product ID
    //
    BYTE  ProductIdentification[PRODUCT_ID_LENGTH];

    //
    // Serial number
    //
    BYTE  SerialNumber[SERIAL_NUMBER_LENGTH];

} CHANGER_ELEMENT_STATUS_EX, *PCHANGER_ELEMENT_STATUS_EX;

//
// Possible flag values
//

#define ELEMENT_STATUS_FULL      0x00000001 // Element contains a unit of media.
#define ELEMENT_STATUS_IMPEXP    0x00000002 // Media in i/e port was placed there by an operator.
#define ELEMENT_STATUS_EXCEPT    0x00000004 // Element is in an abnormal state; check ExceptionCode field for more information.
#define ELEMENT_STATUS_ACCESS    0x00000008 // Access to the i/e port from the medium changer is allowed.
#define ELEMENT_STATUS_EXENAB    0x00000010 // Export of media is supported.
#define ELEMENT_STATUS_INENAB    0x00000020 // Import of media is supported.

#define ELEMENT_STATUS_PRODUCT_DATA 0x00000040 // Serial number valid for the drive

#define ELEMENT_STATUS_LUN_VALID 0x00001000 // Lun information is valid.
#define ELEMENT_STATUS_ID_VALID  0x00002000 // SCSI Id information is valid.
#define ELEMENT_STATUS_NOT_BUS   0x00008000 // Lun and SCSI Id fields are not on same bus as medium changer.
#define ELEMENT_STATUS_INVERT    0x00400000 // Media in element was inverted (valid only if ELEMENT_STATUS_SVALID bit is set)
#define ELEMENT_STATUS_SVALID    0x00800000 // SourceElementAddress field and ELEMENT_STATUS_INVERT bit are valid.

#define ELEMENT_STATUS_PVOLTAG   0x10000000 // Primary volume information is valid.
#define ELEMENT_STATUS_AVOLTAG   0x20000000 // Alternate volume information is valid.

//
// ExceptionCode values.
//

#define ERROR_LABEL_UNREADABLE    0x00000001 // Bar code scanner could not read bar code label.
#define ERROR_LABEL_QUESTIONABLE  0x00000002 // Label could be invalid due to unit attention condition.
#define ERROR_SLOT_NOT_PRESENT    0x00000004 // Slot is currently not addressable in the device.
#define ERROR_DRIVE_NOT_INSTALLED 0x00000008 // Drive is not installed.
#define ERROR_TRAY_MALFUNCTION    0x00000010 // Media tray is malfunctioning/broken.
#define ERROR_INIT_STATUS_NEEDED  0x00000011 // An Initialize Element Status command is needed.
#define ERROR_UNHANDLED_ERROR     0xFFFFFFFF // Unknown error condition


//
// Definitions for IOCTL_CHANGER_INITIALIZE_ELEMENT_STATUS
//

typedef struct _CHANGER_INITIALIZE_ELEMENT_STATUS {

    //
    // List describing the elements and range on which to initialize.
    //

    CHANGER_ELEMENT_LIST ElementList;

    //
    // Indicates whether a bar code scan should be used. Only applicable if
    // CHANGER_BAR_CODE_SCANNER_INSTALLED is set in Features0 of CHANGER_GET_PARAMETERS.
    //

    BOOLEAN BarCodeScan;
} CHANGER_INITIALIZE_ELEMENT_STATUS, *PCHANGER_INITIALIZE_ELEMENT_STATUS;


//
// Definitions for IOCTL_CHANGER_SET_POSITION
//

typedef struct _CHANGER_SET_POSITION {


    //
    // Indicates which transport to move.
    //

    CHANGER_ELEMENT Transport;

    //
    // Indicates the final destination of the transport.
    //

    CHANGER_ELEMENT Destination;

    //
    // Indicates whether the media currently carried by Transport, should be flipped.
    //

    BOOLEAN         Flip;
} CHANGER_SET_POSITION, *PCHANGER_SET_POSITION;


//
// Definitions for IOCTL_CHANGER_EXCHANGE_MEDIUM
//

typedef struct _CHANGER_EXCHANGE_MEDIUM {

    //
    // Indicates which transport to use for the exchange operation.
    //

    CHANGER_ELEMENT Transport;

    //
    // Indicates the source for the media that is to be moved.
    //

    CHANGER_ELEMENT Source;

    //
    // Indicates the final destination of the media originally at Source.
    //

    CHANGER_ELEMENT Destination1;

    //
    // Indicates the destination of the media moved from Destination1.
    //

    CHANGER_ELEMENT Destination2;

    //
    // Indicates whether the medium should be flipped.
    //

    BOOLEAN         Flip1;
    BOOLEAN         Flip2;
} CHANGER_EXCHANGE_MEDIUM, *PCHANGER_EXCHANGE_MEDIUM;


//
// Definitions for IOCTL_CHANGER_MOVE_MEDIUM
//

typedef struct _CHANGER_MOVE_MEDIUM {

    //
    // Indicates which transport to use for the move operation.
    //

    CHANGER_ELEMENT Transport;

    //
    // Indicates the source for the media that is to be moved.
    //

    CHANGER_ELEMENT Source;

    //
    // Indicates the destination of the media originally at Source.
    //

    CHANGER_ELEMENT Destination;

    //
    // Indicates whether the media should be flipped.
    //

    BOOLEAN         Flip;
} CHANGER_MOVE_MEDIUM, *PCHANGER_MOVE_MEDIUM;



//
// Definitions for IOCTL_QUERY_VOLUME_TAGS
//

//
// Input buffer.
//

typedef  struct _CHANGER_SEND_VOLUME_TAG_INFORMATION {

    //
    // Describes the starting element for which to return information.
    //

    CHANGER_ELEMENT StartingElement;

    //
    // Indicates the specific action to perform. See below.
    //

    DWORD ActionCode;

    //
    // Template used by the device to search for volume ids.
    //

    BYTE  VolumeIDTemplate[MAX_VOLUME_TEMPLATE_SIZE];
} CHANGER_SEND_VOLUME_TAG_INFORMATION, *PCHANGER_SEND_VOLUME_TAG_INFORMATION;


//
// Output buffer.
//

typedef struct _READ_ELEMENT_ADDRESS_INFO {

    //
    // Number of elements matching criteria set forth by ActionCode.
    //

    DWORD NumberOfElements;

    //
    // Array of CHANGER_ELEMENT_STATUS structures, one for each element that corresponded
    // with the information passed in with the CHANGER_SEND_VOLUME_TAG_INFORMATION structure.
    //

    CHANGER_ELEMENT_STATUS ElementStatus[1];
} READ_ELEMENT_ADDRESS_INFO, *PREAD_ELEMENT_ADDRESS_INFO;

//
// Possible ActionCode values. See Features0 of CHANGER_GET_PARAMETERS for compatibility with
// the current device.
//

#define SEARCH_ALL         0x0 // Translate - search all defined volume tags.
#define SEARCH_PRIMARY     0x1 // Translate - search only primary volume tags.
#define SEARCH_ALTERNATE   0x2 // Translate - search only alternate volume tags.
#define SEARCH_ALL_NO_SEQ  0x4 // Translate - search all defined volume tags but ignore sequence numbers.
#define SEARCH_PRI_NO_SEQ  0x5 // Translate - search only primary volume tags but ignore sequence numbers.
#define SEARCH_ALT_NO_SEQ  0x6 // Translate - search only alternate volume tags but ignore sequence numbers.

#define ASSERT_PRIMARY     0x8 // Assert - as the primary volume tag - if tag now undefined.
#define ASSERT_ALTERNATE   0x9 // Assert - as the alternate volume tag - if tag now undefined.

#define REPLACE_PRIMARY    0xA // Replace - the primary volume tag - current tag ignored.
#define REPLACE_ALTERNATE  0xB // Replace - the alternate volume tag - current tag ignored.

#define UNDEFINE_PRIMARY   0xC // Undefine - the primary volume tag - current tag ignored.
#define UNDEFINE_ALTERNATE 0xD // Undefine - the alternate volume tag - current tag ignored.


//
// Changer diagnostic test related definitions
//
typedef enum _CHANGER_DEVICE_PROBLEM_TYPE {
   DeviceProblemNone,
   DeviceProblemHardware,
   DeviceProblemCHMError,
   DeviceProblemDoorOpen,
   DeviceProblemCalibrationError,
   DeviceProblemTargetFailure,
   DeviceProblemCHMMoveError,
   DeviceProblemCHMZeroError,
   DeviceProblemCartridgeInsertError,
   DeviceProblemPositionError,
   DeviceProblemSensorError,
   DeviceProblemCartridgeEjectError,
   DeviceProblemGripperError,
   DeviceProblemDriveError
} CHANGER_DEVICE_PROBLEM_TYPE, *PCHANGER_DEVICE_PROBLEM_TYPE;


#define IOCTL_SERIAL_LSRMST_INSERT      CTL_CODE(FILE_DEVICE_SERIAL_PORT,31,METHOD_BUFFERED,FILE_ANY_ACCESS)

#define IOCTL_SERENUM_EXPOSE_HARDWARE   CTL_CODE(FILE_DEVICE_SERENUM,128,METHOD_BUFFERED,FILE_ANY_ACCESS)
#define IOCTL_SERENUM_REMOVE_HARDWARE   CTL_CODE(FILE_DEVICE_SERENUM,129,METHOD_BUFFERED,FILE_ANY_ACCESS)
#define IOCTL_SERENUM_PORT_DESC         CTL_CODE(FILE_DEVICE_SERENUM,130,METHOD_BUFFERED,FILE_ANY_ACCESS)
#define IOCTL_SERENUM_GET_PORT_NAME     CTL_CODE(FILE_DEVICE_SERENUM,131,METHOD_BUFFERED,FILE_ANY_ACCESS)


//
// The following values follow the escape designator in the
// data stream if the LSRMST_INSERT mode has been turned on.
//
#define SERIAL_LSRMST_ESCAPE     ((BYTE )0x00)

//
// Following this value is the contents of the line status
// register, and then the character in the RX hardware when
// the line status register was encountered.
//
#define SERIAL_LSRMST_LSR_DATA   ((BYTE )0x01)

//
// Following this value is the contents of the line status
// register.  No error character follows
//
#define SERIAL_LSRMST_LSR_NODATA ((BYTE )0x02)

//
// Following this value is the contents of the modem status
// register.
//
#define SERIAL_LSRMST_MST        ((BYTE )0x03)

//
// Bit values for FIFO Control Register
//

#define SERIAL_IOC_FCR_FIFO_ENABLE      ((DWORD)0x00000001)
#define SERIAL_IOC_FCR_RCVR_RESET       ((DWORD)0x00000002)
#define SERIAL_IOC_FCR_XMIT_RESET       ((DWORD)0x00000004)
#define SERIAL_IOC_FCR_DMA_MODE         ((DWORD)0x00000008)
#define SERIAL_IOC_FCR_RES1             ((DWORD)0x00000010)
#define SERIAL_IOC_FCR_RES2             ((DWORD)0x00000020)
#define SERIAL_IOC_FCR_RCVR_TRIGGER_LSB ((DWORD)0x00000040)
#define SERIAL_IOC_FCR_RCVR_TRIGGER_MSB ((DWORD)0x00000080)

//
// Bit values for Modem Control Register
//

#define SERIAL_IOC_MCR_DTR              ((DWORD)0x00000001)
#define SERIAL_IOC_MCR_RTS              ((DWORD)0x00000002)
#define SERIAL_IOC_MCR_OUT1             ((DWORD)0x00000004)
#define SERIAL_IOC_MCR_OUT2             ((DWORD)0x00000008)
#define SERIAL_IOC_MCR_LOOP             ((DWORD)0x00000010)


#ifndef _FILESYSTEMFSCTL_
#define _FILESYSTEMFSCTL_

//
// The following is a list of the native file system fsctls followed by
// additional network file system fsctls.  Some values have been
// decommissioned.
//

#define FSCTL_REQUEST_OPLOCK_LEVEL_1    CTL_CODE(FILE_DEVICE_FILE_SYSTEM,  0, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_REQUEST_OPLOCK_LEVEL_2    CTL_CODE(FILE_DEVICE_FILE_SYSTEM,  1, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_REQUEST_BATCH_OPLOCK      CTL_CODE(FILE_DEVICE_FILE_SYSTEM,  2, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_OPLOCK_BREAK_ACKNOWLEDGE  CTL_CODE(FILE_DEVICE_FILE_SYSTEM,  3, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_OPBATCH_ACK_CLOSE_PENDING CTL_CODE(FILE_DEVICE_FILE_SYSTEM,  4, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_OPLOCK_BREAK_NOTIFY       CTL_CODE(FILE_DEVICE_FILE_SYSTEM,  5, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_LOCK_VOLUME               CTL_CODE(FILE_DEVICE_FILE_SYSTEM,  6, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_UNLOCK_VOLUME             CTL_CODE(FILE_DEVICE_FILE_SYSTEM,  7, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_DISMOUNT_VOLUME           CTL_CODE(FILE_DEVICE_FILE_SYSTEM,  8, METHOD_BUFFERED, FILE_ANY_ACCESS)
// decommissioned fsctl value                                              9
#define FSCTL_IS_VOLUME_MOUNTED         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 10, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_IS_PATHNAME_VALID         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 11, METHOD_BUFFERED, FILE_ANY_ACCESS) // PATHNAME_BUFFER,
#define FSCTL_MARK_VOLUME_DIRTY         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 12, METHOD_BUFFERED, FILE_ANY_ACCESS)
// decommissioned fsctl value                                             13
#define FSCTL_QUERY_RETRIEVAL_POINTERS  CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 14,  METHOD_NEITHER, FILE_ANY_ACCESS)
#define FSCTL_GET_COMPRESSION           CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 15, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_SET_COMPRESSION           CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 16, METHOD_BUFFERED, FILE_READ_DATA | FILE_WRITE_DATA)
// decommissioned fsctl value                                             17
// decommissioned fsctl value                                             18
#define FSCTL_SET_BOOTLOADER_ACCESSED   CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 19,  METHOD_NEITHER, FILE_ANY_ACCESS)
#define FSCTL_MARK_AS_SYSTEM_HIVE       FSCTL_SET_BOOTLOADER_ACCESSED
#define FSCTL_OPLOCK_BREAK_ACK_NO_2     CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 20, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_INVALIDATE_VOLUMES        CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 21, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_QUERY_FAT_BPB             CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 22, METHOD_BUFFERED, FILE_ANY_ACCESS) // FSCTL_QUERY_FAT_BPB_BUFFER
#define FSCTL_REQUEST_FILTER_OPLOCK     CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 23, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_FILESYSTEM_GET_STATISTICS CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 24, METHOD_BUFFERED, FILE_ANY_ACCESS) // FILESYSTEM_STATISTICS

#if (_WIN32_WINNT >= _WIN32_WINNT_NT4)
#define FSCTL_GET_NTFS_VOLUME_DATA      CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 25, METHOD_BUFFERED, FILE_ANY_ACCESS) // NTFS_VOLUME_DATA_BUFFER
#define FSCTL_GET_NTFS_FILE_RECORD      CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 26, METHOD_BUFFERED, FILE_ANY_ACCESS) // NTFS_FILE_RECORD_INPUT_BUFFER, NTFS_FILE_RECORD_OUTPUT_BUFFER
#define FSCTL_GET_VOLUME_BITMAP         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 27,  METHOD_NEITHER, FILE_ANY_ACCESS) // STARTING_LCN_INPUT_BUFFER, VOLUME_BITMAP_BUFFER
#define FSCTL_GET_RETRIEVAL_POINTERS    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 28,  METHOD_NEITHER, FILE_ANY_ACCESS) // STARTING_VCN_INPUT_BUFFER, RETRIEVAL_POINTERS_BUFFER
#define FSCTL_MOVE_FILE                 CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 29, METHOD_BUFFERED, FILE_SPECIAL_ACCESS) // MOVE_FILE_DATA,
#define FSCTL_IS_VOLUME_DIRTY           CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 30, METHOD_BUFFERED, FILE_ANY_ACCESS)
// decommissioned fsctl value                                             31
#define FSCTL_ALLOW_EXTENDED_DASD_IO    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 32, METHOD_NEITHER,  FILE_ANY_ACCESS)
#endif /* _WIN32_WINNT >= _WIN32_WINNT_NT4 */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
// decommissioned fsctl value                                             33
// decommissioned fsctl value                                             34
#define FSCTL_FIND_FILES_BY_SID         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 35, METHOD_NEITHER, FILE_ANY_ACCESS)
// decommissioned fsctl value                                             36
// decommissioned fsctl value                                             37
#define FSCTL_SET_OBJECT_ID             CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 38, METHOD_BUFFERED, FILE_SPECIAL_ACCESS) // FILE_OBJECTID_BUFFER
#define FSCTL_GET_OBJECT_ID             CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 39, METHOD_BUFFERED, FILE_ANY_ACCESS) // FILE_OBJECTID_BUFFER
#define FSCTL_DELETE_OBJECT_ID          CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 40, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)
#define FSCTL_SET_REPARSE_POINT         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 41, METHOD_BUFFERED, FILE_SPECIAL_ACCESS) // REPARSE_DATA_BUFFER,
#define FSCTL_GET_REPARSE_POINT         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 42, METHOD_BUFFERED, FILE_ANY_ACCESS) // REPARSE_DATA_BUFFER
#define FSCTL_DELETE_REPARSE_POINT      CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 43, METHOD_BUFFERED, FILE_SPECIAL_ACCESS) // REPARSE_DATA_BUFFER,
#define FSCTL_ENUM_USN_DATA             CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 44,  METHOD_NEITHER, FILE_ANY_ACCESS) // MFT_ENUM_DATA,
#define FSCTL_SECURITY_ID_CHECK         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 45,  METHOD_NEITHER, FILE_READ_DATA)  // BULK_SECURITY_TEST_DATA,
#define FSCTL_READ_USN_JOURNAL          CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 46,  METHOD_NEITHER, FILE_ANY_ACCESS) // READ_USN_JOURNAL_DATA, USN
#define FSCTL_SET_OBJECT_ID_EXTENDED    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 47, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)
#define FSCTL_CREATE_OR_GET_OBJECT_ID   CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 48, METHOD_BUFFERED, FILE_ANY_ACCESS) // FILE_OBJECTID_BUFFER
#define FSCTL_SET_SPARSE                CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 49, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)
#define FSCTL_SET_ZERO_DATA             CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 50, METHOD_BUFFERED, FILE_WRITE_DATA) // FILE_ZERO_DATA_INFORMATION,
#define FSCTL_QUERY_ALLOCATED_RANGES    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 51,  METHOD_NEITHER, FILE_READ_DATA)  // FILE_ALLOCATED_RANGE_BUFFER, FILE_ALLOCATED_RANGE_BUFFER
#define FSCTL_ENABLE_UPGRADE            CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 52, METHOD_BUFFERED, FILE_WRITE_DATA)
#define FSCTL_SET_ENCRYPTION            CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 53,  METHOD_NEITHER, FILE_ANY_ACCESS) // ENCRYPTION_BUFFER, DECRYPTION_STATUS_BUFFER
#define FSCTL_ENCRYPTION_FSCTL_IO       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 54,  METHOD_NEITHER, FILE_ANY_ACCESS)
#define FSCTL_WRITE_RAW_ENCRYPTED       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 55,  METHOD_NEITHER, FILE_SPECIAL_ACCESS) // ENCRYPTED_DATA_INFO, EXTENDED_ENCRYPTED_DATA_INFO
#define FSCTL_READ_RAW_ENCRYPTED        CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 56,  METHOD_NEITHER, FILE_SPECIAL_ACCESS) // REQUEST_RAW_ENCRYPTED_DATA, ENCRYPTED_DATA_INFO, EXTENDED_ENCRYPTED_DATA_INFO
#define FSCTL_CREATE_USN_JOURNAL        CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 57,  METHOD_NEITHER, FILE_ANY_ACCESS) // CREATE_USN_JOURNAL_DATA,
#define FSCTL_READ_FILE_USN_DATA        CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 58,  METHOD_NEITHER, FILE_ANY_ACCESS) // Read the Usn Record for a file
#define FSCTL_WRITE_USN_CLOSE_RECORD    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 59,  METHOD_NEITHER, FILE_ANY_ACCESS) // Generate Close Usn Record
#define FSCTL_EXTEND_VOLUME             CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 60, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_QUERY_USN_JOURNAL         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 61, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_DELETE_USN_JOURNAL        CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 62, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_MARK_HANDLE               CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 63, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_SIS_COPYFILE              CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 64, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_SIS_LINK_FILES            CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 65, METHOD_BUFFERED, FILE_READ_DATA | FILE_WRITE_DATA)
// decommissioned fsctl value                                             66
// decommissioned fsctl value                                             67
// decommissioned fsctl value                                             68
#define FSCTL_RECALL_FILE               CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 69, METHOD_NEITHER, FILE_ANY_ACCESS)
// decommissioned fsctl value                                             70
#define FSCTL_READ_FROM_PLEX            CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 71, METHOD_OUT_DIRECT, FILE_READ_DATA)
#define FSCTL_FILE_PREFETCH             CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 72, METHOD_BUFFERED, FILE_SPECIAL_ACCESS) // FILE_PREFETCH
#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN2K */

#if (_WIN32_WINNT >= _WIN32_WINNT_VISTA)
#define FSCTL_MAKE_MEDIA_COMPATIBLE         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 76, METHOD_BUFFERED, FILE_WRITE_DATA) // UDFS R/W
#define FSCTL_SET_DEFECT_MANAGEMENT         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 77, METHOD_BUFFERED, FILE_WRITE_DATA) // UDFS R/W
#define FSCTL_QUERY_SPARING_INFO            CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 78, METHOD_BUFFERED, FILE_ANY_ACCESS) // UDFS R/W
#define FSCTL_QUERY_ON_DISK_VOLUME_INFO     CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 79, METHOD_BUFFERED, FILE_ANY_ACCESS) // C/UDFS
#define FSCTL_SET_VOLUME_COMPRESSION_STATE  CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 80, METHOD_BUFFERED, FILE_SPECIAL_ACCESS) // VOLUME_COMPRESSION_STATE
// decommissioned fsctl value                                                 80
#define FSCTL_TXFS_MODIFY_RM                CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 81, METHOD_BUFFERED, FILE_WRITE_DATA) // TxF
#define FSCTL_TXFS_QUERY_RM_INFORMATION     CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 82, METHOD_BUFFERED, FILE_READ_DATA)  // TxF
// decommissioned fsctl value                                                 83
#define FSCTL_TXFS_ROLLFORWARD_REDO         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 84, METHOD_BUFFERED, FILE_WRITE_DATA) // TxF
#define FSCTL_TXFS_ROLLFORWARD_UNDO         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 85, METHOD_BUFFERED, FILE_WRITE_DATA) // TxF
#define FSCTL_TXFS_START_RM                 CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 86, METHOD_BUFFERED, FILE_WRITE_DATA) // TxF
#define FSCTL_TXFS_SHUTDOWN_RM              CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 87, METHOD_BUFFERED, FILE_WRITE_DATA) // TxF
#define FSCTL_TXFS_READ_BACKUP_INFORMATION  CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 88, METHOD_BUFFERED, FILE_READ_DATA)  // TxF
#define FSCTL_TXFS_WRITE_BACKUP_INFORMATION CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 89, METHOD_BUFFERED, FILE_WRITE_DATA) // TxF
#define FSCTL_TXFS_CREATE_SECONDARY_RM      CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 90, METHOD_BUFFERED, FILE_WRITE_DATA) // TxF
#define FSCTL_TXFS_GET_METADATA_INFO        CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 91, METHOD_BUFFERED, FILE_READ_DATA)  // TxF
#define FSCTL_TXFS_GET_TRANSACTED_VERSION   CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 92, METHOD_BUFFERED, FILE_READ_DATA)  // TxF
// decommissioned fsctl value                                                 93
#define FSCTL_TXFS_SAVEPOINT_INFORMATION    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 94, METHOD_BUFFERED, FILE_WRITE_DATA) // TxF
#define FSCTL_TXFS_CREATE_MINIVERSION       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 95, METHOD_BUFFERED, FILE_WRITE_DATA) // TxF
// decommissioned fsctl value                                                 96
// decommissioned fsctl value                                                 97
// decommissioned fsctl value                                                 98
#define FSCTL_TXFS_TRANSACTION_ACTIVE       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 99, METHOD_BUFFERED, FILE_READ_DATA)  // TxF
#define FSCTL_SET_ZERO_ON_DEALLOCATION      CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 101, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)
#define FSCTL_SET_REPAIR                    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 102, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_GET_REPAIR                    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 103, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_WAIT_FOR_REPAIR               CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 104, METHOD_BUFFERED, FILE_ANY_ACCESS)
// decommissioned fsctl value                                                 105
#define FSCTL_INITIATE_REPAIR               CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 106, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_CSC_INTERNAL                  CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 107, METHOD_NEITHER, FILE_ANY_ACCESS) // CSC internal implementation
#define FSCTL_SHRINK_VOLUME                 CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 108, METHOD_BUFFERED, FILE_SPECIAL_ACCESS) // SHRINK_VOLUME_INFORMATION
#define FSCTL_SET_SHORT_NAME_BEHAVIOR       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 109, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_DFSR_SET_GHOST_HANDLE_STATE   CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 110, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
//  Values 111 - 119 are reserved for FSRM.
//

#define FSCTL_TXFS_LIST_TRANSACTION_LOCKED_FILES \
                                            CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 120, METHOD_BUFFERED, FILE_READ_DATA) // TxF
#define FSCTL_TXFS_LIST_TRANSACTIONS        CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 121, METHOD_BUFFERED, FILE_READ_DATA) // TxF
#define FSCTL_QUERY_PAGEFILE_ENCRYPTION     CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 122, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif /* _WIN32_WINNT >= _WIN32_WINNT_VISTA */

#if (_WIN32_WINNT >= _WIN32_WINNT_VISTA)
#define FSCTL_RESET_VOLUME_ALLOCATION_HINTS CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 123, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif /* _WIN32_WINNT >= _WIN32_WINNT_VISTA */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)
#define FSCTL_QUERY_DEPENDENT_VOLUME        CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 124, METHOD_BUFFERED, FILE_ANY_ACCESS)    // Dependency File System Filter
#define FSCTL_SD_GLOBAL_CHANGE              CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 125, METHOD_BUFFERED, FILE_ANY_ACCESS) // Query/Change NTFS Security Descriptors
#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN7 */

#if (_WIN32_WINNT >= _WIN32_WINNT_VISTA)
#define FSCTL_TXFS_READ_BACKUP_INFORMATION2 CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 126, METHOD_BUFFERED, FILE_ANY_ACCESS) // TxF
#endif /* _WIN32_WINNT >= _WIN32_WINNT_VISTA */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)
#define FSCTL_LOOKUP_STREAM_FROM_CLUSTER    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 127, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_TXFS_WRITE_BACKUP_INFORMATION2 CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 128, METHOD_BUFFERED, FILE_ANY_ACCESS) // TxF
#define FSCTL_FILE_TYPE_NOTIFICATION        CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 129, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)
#define FSCTL_FILE_LEVEL_TRIM               CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 130, METHOD_BUFFERED, FILE_WRITE_DATA)
#endif /*_WIN32_WINNT >= _WIN32_WINNT_WIN8 */

//
//  Values 131 - 139 are reserved for FSRM.
//

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)
#define FSCTL_GET_BOOT_AREA_INFO            CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 140, METHOD_BUFFERED, FILE_ANY_ACCESS) // BOOT_AREA_INFO
#define FSCTL_GET_RETRIEVAL_POINTER_BASE    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 141, METHOD_BUFFERED, FILE_ANY_ACCESS) // RETRIEVAL_POINTER_BASE
#define FSCTL_SET_PERSISTENT_VOLUME_STATE   CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 142, METHOD_BUFFERED, FILE_ANY_ACCESS)  // FILE_FS_PERSISTENT_VOLUME_INFORMATION
#define FSCTL_QUERY_PERSISTENT_VOLUME_STATE CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 143, METHOD_BUFFERED, FILE_ANY_ACCESS)  // FILE_FS_PERSISTENT_VOLUME_INFORMATION

#define FSCTL_REQUEST_OPLOCK                CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 144, METHOD_BUFFERED, FILE_ANY_ACCESS)

#define FSCTL_CSV_TUNNEL_REQUEST            CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 145, METHOD_BUFFERED, FILE_ANY_ACCESS) // CSV_TUNNEL_REQUEST
#define FSCTL_IS_CSV_FILE                   CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 146, METHOD_BUFFERED, FILE_ANY_ACCESS) // IS_CSV_FILE

#define FSCTL_QUERY_FILE_SYSTEM_RECOGNITION CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 147, METHOD_BUFFERED, FILE_ANY_ACCESS) //
#define FSCTL_CSV_GET_VOLUME_PATH_NAME      CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 148, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_CSV_GET_VOLUME_NAME_FOR_VOLUME_MOUNT_POINT CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 149, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_CSV_GET_VOLUME_PATH_NAMES_FOR_VOLUME_NAME CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 150,  METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_IS_FILE_ON_CSV_VOLUME         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 151,  METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN7 */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)
#define FSCTL_CORRUPTION_HANDLING           CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 152, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_OFFLOAD_READ                  CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 153, METHOD_BUFFERED, FILE_READ_ACCESS)
#define FSCTL_OFFLOAD_WRITE                 CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 154, METHOD_BUFFERED, FILE_WRITE_ACCESS)
#endif /*_WIN32_WINNT >= _WIN32_WINNT_WIN8 */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)
#define FSCTL_CSV_INTERNAL                  CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 155,  METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN7 */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)
#define FSCTL_SET_PURGE_FAILURE_MODE        CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 156, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_QUERY_FILE_LAYOUT             CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 157, METHOD_NEITHER, FILE_ANY_ACCESS)
#define FSCTL_IS_VOLUME_OWNED_BYCSVFS       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 158,  METHOD_BUFFERED, FILE_ANY_ACCESS)

#define FSCTL_GET_INTEGRITY_INFORMATION     CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 159, METHOD_BUFFERED, FILE_ANY_ACCESS)                  // FSCTL_GET_INTEGRITY_INFORMATION_BUFFER
#define FSCTL_SET_INTEGRITY_INFORMATION     CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 160, METHOD_BUFFERED, FILE_READ_DATA | FILE_WRITE_DATA) // FSCTL_SET_INTEGRITY_INFORMATION_BUFFER

#define FSCTL_QUERY_FILE_REGIONS            CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 161, METHOD_BUFFERED, FILE_ANY_ACCESS)

#endif /*_WIN32_WINNT >= _WIN32_WINNT_WIN8 */


#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)
#define FSCTL_RKF_INTERNAL                  CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 171, METHOD_NEITHER, FILE_ANY_ACCESS) // Resume Key Filter

#define FSCTL_SCRUB_DATA                    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 172, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_REPAIR_COPIES                 CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 173, METHOD_BUFFERED, FILE_READ_DATA | FILE_WRITE_DATA)
#define FSCTL_DISABLE_LOCAL_BUFFERING       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 174, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_CSV_MGMT_LOCK                 CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 175, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_CSV_QUERY_DOWN_LEVEL_FILE_SYSTEM_CHARACTERISTICS CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 176, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_ADVANCE_FILE_ID               CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 177, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_CSV_SYNC_TUNNEL_REQUEST       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 178, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_CSV_QUERY_VETO_FILE_DIRECT_IO CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 179, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_WRITE_USN_REASON              CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 180, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_CSV_CONTROL                   CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 181, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_GET_REFS_VOLUME_DATA          CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 182, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_CSV_H_BREAKING_SYNC_TUNNEL_REQUEST CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 185, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif /*_WIN32_WINNT >= _WIN32_WINNT_WIN8 */

#if (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)
#define FSCTL_QUERY_STORAGE_CLASSES         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 187, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_QUERY_REGION_INFO             CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 188, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_USN_TRACK_MODIFIED_RANGES     CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 189, METHOD_BUFFERED, FILE_ANY_ACCESS) // USN_TRACK_MODIFIED_RANGES
#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE) */
#if (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)
#define FSCTL_QUERY_SHARED_VIRTUAL_DISK_SUPPORT CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 192, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_SVHDX_SYNC_TUNNEL_REQUEST         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 193, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_SVHDX_SET_INITIATOR_INFORMATION   CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 194, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE) */
#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)
#define FSCTL_SET_EXTERNAL_BACKING              CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 195, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)
#define FSCTL_GET_EXTERNAL_BACKING              CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 196, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_DELETE_EXTERNAL_BACKING           CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 197, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)
#define FSCTL_ENUM_EXTERNAL_BACKING             CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 198, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_ENUM_OVERLAY                      CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 199, METHOD_NEITHER, FILE_ANY_ACCESS)
#define FSCTL_ADD_OVERLAY                       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 204, METHOD_BUFFERED, FILE_WRITE_DATA)
#define FSCTL_REMOVE_OVERLAY                    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 205, METHOD_BUFFERED, FILE_WRITE_DATA)
#define FSCTL_UPDATE_OVERLAY                    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 206, METHOD_BUFFERED, FILE_WRITE_DATA)
#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WIN7) */
#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)
#define FSCTL_SHUFFLE_FILE                      CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 208, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS) // SHUFFLE_FILE_DATA
#endif /*_WIN32_WINNT >= _WIN32_WINNT_WIN8 */
#if (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)
#define FSCTL_DUPLICATE_EXTENTS_TO_FILE         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 209, METHOD_BUFFERED, FILE_WRITE_DATA )
#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE) */
#if (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)
#define FSCTL_SPARSE_OVERALLOCATE               CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 211, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)
#define FSCTL_STORAGE_QOS_CONTROL               CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 212, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE) */
#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)
#define FSCTL_INITIATE_FILE_METADATA_OPTIMIZATION       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 215, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)
#define FSCTL_QUERY_FILE_METADATA_OPTIMIZATION          CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 216, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)
#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD) */

#if (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)
#define FSCTL_SVHDX_ASYNC_TUNNEL_REQUEST         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 217, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE) */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)
#define FSCTL_GET_WOF_VERSION                    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 218, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif

#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)
#define FSCTL_HCS_SYNC_TUNNEL_REQUEST            CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 219, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_HCS_ASYNC_TUNNEL_REQUEST           CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 220, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_QUERY_EXTENT_READ_CACHE_INFO       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 221, METHOD_NEITHER, FILE_ANY_ACCESS)  // VCN_RANGE_INPUT_BUFFER, EXTENT_READ_CACHE_INFO_BUFFER
#define FSCTL_QUERY_REFS_VOLUME_COUNTER_INFO     CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 222, METHOD_NEITHER, FILE_ANY_ACCESS)  // REFS_VOLUME_COUNTER_INFO_INPUT_BUFFER, VOLUME_REFS_INFO_BUFFER
#define FSCTL_CLEAN_VOLUME_METADATA              CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 223, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_SET_INTEGRITY_INFORMATION_EX       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 224, METHOD_BUFFERED, FILE_ANY_ACCESS) // FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX
#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD) */
#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)
#define FSCTL_SUSPEND_OVERLAY                    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 225, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif
#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)
#define FSCTL_VIRTUAL_STORAGE_QUERY_PROPERTY     CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 226, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_FILESYSTEM_GET_STATISTICS_EX       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 227, METHOD_BUFFERED, FILE_ANY_ACCESS) // FILESYSTEM_STATISTICS_EX
#define FSCTL_QUERY_VOLUME_CONTAINER_STATE       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 228, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_SET_LAYER_ROOT                     CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 229, METHOD_BUFFERED, FILE_ANY_ACCESS) // CONTAINER_ROOT_INFO_INPUT CONTAINER_ROOT_INFO_OUTPUT
#endif

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_TH2)
#define FSCTL_QUERY_DIRECT_ACCESS_EXTENTS        CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 230, METHOD_NEITHER, FILE_ANY_ACCESS) // QUERY_DIRECT_ACCESS_EXTENTS
#define FSCTL_NOTIFY_STORAGE_SPACE_ALLOCATION    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 231, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_SSDI_STORAGE_REQUEST               CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 232, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS1)
#define FSCTL_QUERY_DIRECT_IMAGE_ORIGINAL_BASE   CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 233, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_READ_UNPRIVILEGED_USN_JOURNAL      CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 234,  METHOD_NEITHER, FILE_ANY_ACCESS) // READ_USN_JOURNAL_DATA, USN
#endif

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_TH2)
#define FSCTL_GHOST_FILE_EXTENTS                 CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 235, METHOD_BUFFERED, FILE_WRITE_ACCESS) // FSCTL_GHOST_FILE_EXTENTS_INPUT_BUFFER
#define FSCTL_QUERY_GHOSTED_FILE_EXTENTS         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 236, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_TH2)
#define FSCTL_UNMAP_SPACE                        CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 237, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif

#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)
#define FSCTL_HCS_SYNC_NO_WRITE_TUNNEL_REQUEST   CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 238, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif
#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS1)
#define FSCTL_START_VIRTUALIZATION_INSTANCE     CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 240, METHOD_BUFFERED, FILE_ANY_ACCESS) // VIRTUALIZATION_INSTANCE_INFO_INPUT, VIRTUALIZATION_INSTANCE_INFO_OUTPUT
#define FSCTL_GET_FILTER_FILE_IDENTIFIER        CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 241, METHOD_BUFFERED, FILE_ANY_ACCESS) // GET_FILTER_FILE_IDENTIFIER_INPUT, GET_FILTER_FILE_IDENTIFIER_OUTPUT
#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS1) */

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
#define FSCTL_STREAMS_QUERY_PARAMETERS          CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 241, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_STREAMS_ASSOCIATE_ID              CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 242, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_STREAMS_QUERY_ID                  CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 243, METHOD_BUFFERED, FILE_ANY_ACCESS)

#define FSCTL_GET_RETRIEVAL_POINTERS_AND_REFCOUNT CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 244, METHOD_NEITHER,  FILE_ANY_ACCESS) // STARTING_VCN_INPUT_BUFFER, RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER

#define FSCTL_QUERY_VOLUME_NUMA_INFO            CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 245, METHOD_BUFFERED, FILE_ANY_ACCESS)

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#define FSCTL_REFS_DEALLOCATE_RANGES            CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 246, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_TH2)
#define FSCTL_QUERY_REFS_SMR_VOLUME_INFO         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 247, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_SET_REFS_SMR_VOLUME_GC_PARAMETERS  CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 248, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_SET_REFS_FILE_STRICTLY_SEQUENTIAL  CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 249, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS3)
#define FSCTL_DUPLICATE_EXTENTS_TO_FILE_EX      CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 250, METHOD_BUFFERED, FILE_WRITE_DATA)
#define FSCTL_QUERY_BAD_RANGES                  CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 251, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_SET_DAX_ALLOC_ALIGNMENT_HINT      CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 252, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_DELETE_CORRUPTED_REFS_CONTAINER   CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 253, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_SCRUB_UNDISCOVERABLE_ID           CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 254, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS3) */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS4)
#define FSCTL_NOTIFY_DATA_CHANGE                CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 255, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS4) */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS1)
#define FSCTL_START_VIRTUALIZATION_INSTANCE_EX  CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 256, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS1) */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS4)
#define FSCTL_ENCRYPTION_KEY_CONTROL            CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 257, METHOD_BUFFERED, FILE_ANY_ACCESS)  // protect/unprotect under DPL
#define FSCTL_VIRTUAL_STORAGE_SET_BEHAVIOR      CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 258, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS4) */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS1)
#define FSCTL_SET_REPARSE_POINT_EX              CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 259, METHOD_BUFFERED, FILE_SPECIAL_ACCESS) // REPARSE_DATA_BUFFER_EX
#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS1) */
#if (NTDDI_VERSION >= NTDDI_WIN10_RS5) || (NTDDI_VERSION >= NTDDI_WIN8) //Win8 check is for backward compatibility.
#define FSCTL_REARRANGE_FILE                    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 264, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS) // REARRANGE_FILE_DATA
#define FSCTL_VIRTUAL_STORAGE_PASSTHROUGH       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 265, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_GET_RETRIEVAL_POINTER_COUNT       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 266, METHOD_NEITHER,  FILE_ANY_ACCESS) // STARTING_VCN_INPUT_BUFFER, RETRIEVAL_POINTER_COUNT
#if defined(_WIN64)
#define FSCTL_ENABLE_PER_IO_FLAGS               CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 267, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif /* _WIN64 */
#endif /* (NTDDI_VERSION >= NTDDI_WIN10_RS5) */
#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
#define FSCTL_QUERY_ASYNC_DUPLICATE_EXTENTS_STATUS  CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 268, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif
#if (NTDDI_VERSION >= NTDDI_WIN10_MN)
#define FSCTL_SMB_SHARE_FLUSH_AND_PURGE         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 271, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_FE)
#define FSCTL_REFS_STREAM_SNAPSHOT_MANAGEMENT   CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 272, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif
#if (NTDDI_VERSION >= NTDDI_WIN10_CO)
#define FSCTL_MANAGE_BYPASS_IO                  CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 274, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_FE)
#define FSCTL_REFS_DEALLOCATE_RANGES_EX         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 275, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_FE)
#define FSCTL_SET_CACHED_RUNS_STATE             CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 276, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif
#if (NTDDI_VERSION >= NTDDI_WIN10_NI)
#define FSCTL_REFS_SET_VOLUME_COMPRESSION_INFO    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 277, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_REFS_QUERY_VOLUME_COMPRESSION_INFO  CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 278, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif
#if (NTDDI_VERSION >= NTDDI_WIN10_NI)
#define FSCTL_DUPLICATE_CLUSTER                     CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 279, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_CREATE_LCN_WEAK_REFERENCE             CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 280, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_CLEAR_LCN_WEAK_REFERENCE              CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 281, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_QUERY_LCN_WEAK_REFERENCE              CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 282, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_CLEAR_ALL_LCN_WEAK_REFERENCES         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 283, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_REFS_SET_VOLUME_DEDUP_INFO            CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 284, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_REFS_QUERY_VOLUME_DEDUP_INFO          CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 285, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif
#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
#define FSCTL_LMR_QUERY_INFO                        CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 286, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif
#if (NTDDI_VERSION >= NTDDI_WIN10_NI)
#define FSCTL_REFS_CHECKPOINT_VOLUME                CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 287, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_REFS_QUERY_VOLUME_TOTAL_SHARED_LCNS   CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 288, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif
#if (NTDDI_VERSION >= NTDDI_WIN10_CU)
#define FSCTL_UPGRADE_VOLUME                    CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 289, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif
#if (NTDDI_VERSION >= NTDDI_WIN11_ZN)
#define FSCTL_REFS_SET_VOLUME_IO_METRICS_INFO       CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 290, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define FSCTL_REFS_QUERY_VOLUME_IO_METRICS_INFO     CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 291, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif // #if (NTDDI_VERSION >= NTDDI_WIN11_ZN)
#if (NTDDI_VERSION >= NTDDI_WIN11_GE)
#define FSCTL_REFS_SET_ROLLBACK_PROTECTION_INFO     CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 292, METHOD_BUFFERED, FILE_ANY_ACCESS) // REFS_SET_ROLLBACK_PROTECTION_INFO_INPUT_BUFFER
#define FSCTL_REFS_QUERY_ROLLBACK_PROTECTION_INFO   CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 293, METHOD_BUFFERED, FILE_ANY_ACCESS) // REFS_QUERY_ROLLBACK_PROTECTION_INFO_OUTPUT_BUFFER
#endif // #if (NTDDI_VERSION >= NTDDI_WIN11_GE)
#if (NTDDI_VERSION >= NTDDI_WIN11_GE)
#define FSCTL_FILE_SOV_CHECK_RANGE                  CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 294, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif
#if (NTDDI_VERSION >= NTDDI_WIN11_GE)
#define FSCTL_CASCADES_REFS_SET_FILE_REMOTE         CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 295, METHOD_BUFFERED, FILE_ANY_ACCESS)
#endif
//
// AVIO IOCTLS.
//

#define IOCTL_AVIO_ALLOCATE_STREAM      CTL_CODE(FILE_DEVICE_AVIO, 1, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)
#define IOCTL_AVIO_FREE_STREAM          CTL_CODE(FILE_DEVICE_AVIO, 2, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)
#define IOCTL_AVIO_MODIFY_STREAM        CTL_CODE(FILE_DEVICE_AVIO, 3, METHOD_BUFFERED, FILE_SPECIAL_ACCESS)

// end_ntddk
// begin_ntifs

//
// The following long list of structs are associated with the preceding
// file system fsctls.
//

//
//==================== FSCTL_IS_PATHNAME_VALID ======================
//
// Structure for FSCTL_IS_PATHNAME_VALID
//

typedef struct _PATHNAME_BUFFER {

    DWORD PathNameLength;
    WCHAR Name[1];

} PATHNAME_BUFFER, *PPATHNAME_BUFFER;

//
//==================== FSCTL_QUERY_BPB_INFO ======================
//
// Structure for FSCTL_QUERY_BPB_INFO
//

typedef struct _FSCTL_QUERY_FAT_BPB_BUFFER {

    BYTE  First0x24BytesOfBootSector[0x24];

} FSCTL_QUERY_FAT_BPB_BUFFER, *PFSCTL_QUERY_FAT_BPB_BUFFER;

#if (_WIN32_WINNT >= _WIN32_WINNT_NT4)

//
//==================== FSCTL_GET_NTFS_VOLUME_DATA ======================
//
// Structures for FSCTL_GET_NTFS_VOLUME_DATA.
// The user must pass the basic buffer below.  Ntfs
// will return as many fields as available in the extended
// buffer which follows immediately after the VOLUME_DATA_BUFFER.
//

typedef struct {

    LARGE_INTEGER VolumeSerialNumber;
    LARGE_INTEGER NumberSectors;
    LARGE_INTEGER TotalClusters;
    LARGE_INTEGER FreeClusters;
    LARGE_INTEGER TotalReserved;
    DWORD BytesPerSector;
    DWORD BytesPerCluster;
    DWORD BytesPerFileRecordSegment;
    DWORD ClustersPerFileRecordSegment;
    LARGE_INTEGER MftValidDataLength;
    LARGE_INTEGER MftStartLcn;
    LARGE_INTEGER Mft2StartLcn;
    LARGE_INTEGER MftZoneStart;
    LARGE_INTEGER MftZoneEnd;

} NTFS_VOLUME_DATA_BUFFER, *PNTFS_VOLUME_DATA_BUFFER;

typedef struct {

    DWORD ByteCount;

    WORD   MajorVersion;
    WORD   MinorVersion;

    DWORD BytesPerPhysicalSector;

    WORD   LfsMajorVersion;
    WORD   LfsMinorVersion;

#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)
    DWORD MaxDeviceTrimExtentCount;
    DWORD MaxDeviceTrimByteCount;

    DWORD MaxVolumeTrimExtentCount;
    DWORD MaxVolumeTrimByteCount;
#endif

} NTFS_EXTENDED_VOLUME_DATA, *PNTFS_EXTENDED_VOLUME_DATA;
#endif /* _WIN32_WINNT >= _WIN32_WINNT_NT4 */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

//
//==================== FSCTL_GET_REFS_VOLUME_DATA ======================
//
// Structures for FSCTL_GET_REFS_VOLUME_DATA.
//

typedef struct {

    DWORD ByteCount;
    DWORD MajorVersion;
    DWORD MinorVersion;

    DWORD BytesPerPhysicalSector;

    LARGE_INTEGER VolumeSerialNumber;
    LARGE_INTEGER NumberSectors;
    LARGE_INTEGER TotalClusters;
    LARGE_INTEGER FreeClusters;
    LARGE_INTEGER TotalReserved;
    DWORD BytesPerSector;
    DWORD BytesPerCluster;
    LARGE_INTEGER MaximumSizeOfResidentFile;

    WORD   FastTierDataFillRatio;               // between 0 and 10000
    WORD   SlowTierDataFillRatio;               // between 0 and 10000

    DWORD DestagesFastTierToSlowTierRate;       // in clusters per second

    WORD   MetadataChecksumType;

    BYTE  Reserved0[6];

    DWORD DriverMajorVersion;
    DWORD DriverMinorVersion;

    LARGE_INTEGER Reserved[7];

} REFS_VOLUME_DATA_BUFFER, *PREFS_VOLUME_DATA_BUFFER;

#define REFS_VOLUME_DATA_BUFFER_CONTAINS_DRIVER_VERSION(VOLUME_DATA_BUFFER) \
    ((VOLUME_DATA_BUFFER)->ByteCount >= RTL_SIZEOF_THROUGH_FIELD( REFS_VOLUME_DATA_BUFFER, DriverMinorVersion ))

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN8 */

#if (_WIN32_WINNT >= _WIN32_WINNT_NT4)
//
//==================== FSCTL_GET_VOLUME_BITMAP ======================
//
// Structure for FSCTL_GET_VOLUME_BITMAP
//

typedef struct {

    LARGE_INTEGER StartingLcn;

} STARTING_LCN_INPUT_BUFFER, *PSTARTING_LCN_INPUT_BUFFER;

#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)

#define GET_VOLUME_BITMAP_FLAG_MASK_METADATA         0x00000001

typedef struct {

    LARGE_INTEGER StartingLcn;
    DWORD Flags;

} STARTING_LCN_INPUT_BUFFER_EX, *PSTARTING_LCN_INPUT_BUFFER_EX;

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD */

typedef struct {

    LARGE_INTEGER StartingLcn;
    LARGE_INTEGER BitmapSize;
    BYTE  Buffer[1];

} VOLUME_BITMAP_BUFFER, *PVOLUME_BITMAP_BUFFER;
#endif /* _WIN32_WINNT >= _WIN32_WINNT_NT4 */

#if (_WIN32_WINNT >= _WIN32_WINNT_NT4)
//
//==================== FSCTL_GET_RETRIEVAL_POINTERS ======================
//
// Structure for FSCTL_GET_RETRIEVAL_POINTERS
//

typedef struct {

    LARGE_INTEGER StartingVcn;

} STARTING_VCN_INPUT_BUFFER, *PSTARTING_VCN_INPUT_BUFFER;

typedef struct RETRIEVAL_POINTERS_BUFFER {

    DWORD ExtentCount;
    LARGE_INTEGER StartingVcn;
    struct {
        LARGE_INTEGER NextVcn;
        LARGE_INTEGER Lcn;
    } Extents[1];

} RETRIEVAL_POINTERS_BUFFER, *PRETRIEVAL_POINTERS_BUFFER;
#endif /* _WIN32_WINNT >= _WIN32_WINNT_NT4 */

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
//
//==================== FSCTL_GET_RETRIEVAL_POINTERS_AND_REFCOUNT ======================
//
// Structure for FSCTL_GET_RETRIEVAL_POINTERS_AND_REFCOUNT
//

//
// Input structure is STARTING_VCN_INPUT_BUFFER
//

typedef struct RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {

    DWORD ExtentCount;
    LARGE_INTEGER StartingVcn;
    struct {
        LARGE_INTEGER NextVcn;
        LARGE_INTEGER Lcn;
        DWORD ReferenceCount;
    } Extents[1];

} RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER, *PRETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER;
#endif /* (NTDDI_VERSION >= NTDDI_WIN10_RS2) */

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5) || (NTDDI_VERSION >= NTDDI_WIN8) //Win8 check is for backward compatibility.
//
//==================== FSCTL_GET_RETRIEVAL_POINTER_COUNT ======================
//
// Structure for FSCTL_GET_RETRIEVAL_POINTER_COUNT
//

//
// Input structure is STARTING_VCN_INPUT_BUFFER
//

typedef struct RETRIEVAL_POINTER_COUNT {

    DWORD ExtentCount;

} RETRIEVAL_POINTER_COUNT, *PRETRIEVAL_POINTER_COUNT;
#endif /* #if (NTDDI_VERSION >= NTDDI_WIN10_RS5) */

#if (_WIN32_WINNT >= _WIN32_WINNT_NT4)
//
//==================== FSCTL_GET_NTFS_FILE_RECORD ======================
//
// Structures for FSCTL_GET_NTFS_FILE_RECORD
//

typedef struct {

    LARGE_INTEGER FileReferenceNumber;

} NTFS_FILE_RECORD_INPUT_BUFFER, *PNTFS_FILE_RECORD_INPUT_BUFFER;

typedef struct {

    LARGE_INTEGER FileReferenceNumber;
    DWORD FileRecordLength;
    BYTE  FileRecordBuffer[1];

} NTFS_FILE_RECORD_OUTPUT_BUFFER, *PNTFS_FILE_RECORD_OUTPUT_BUFFER;
#endif /* _WIN32_WINNT >= _WIN32_WINNT_NT4 */

#if (_WIN32_WINNT >= _WIN32_WINNT_NT4)
//
//==================== FSCTL_MOVE_FILE ======================
//
// Structure for FSCTL_MOVE_FILE
//

typedef struct {

    HANDLE FileHandle;
    LARGE_INTEGER StartingVcn;
    LARGE_INTEGER StartingLcn;
    DWORD ClusterCount;

} MOVE_FILE_DATA, *PMOVE_FILE_DATA;

typedef struct {

    HANDLE FileHandle;
    LARGE_INTEGER SourceFileRecord;
    LARGE_INTEGER TargetFileRecord;

} MOVE_FILE_RECORD_DATA, *PMOVE_FILE_RECORD_DATA;

#if defined(_WIN64)
//
//  32/64 Bit thunking support structure
//

typedef struct _MOVE_FILE_DATA32 {

    UINT32 FileHandle;
    LARGE_INTEGER StartingVcn;
    LARGE_INTEGER StartingLcn;
    DWORD ClusterCount;

} MOVE_FILE_DATA32, *PMOVE_FILE_DATA32;
#endif
#endif /* _WIN32_WINNT >= _WIN32_WINNT_NT4 */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
//
//==================== FSCTL_FIND_FILES_BY_SID ======================
//
// Structures for FSCTL_FIND_FILES_BY_SID
//

typedef struct {
    DWORD Restart;
    SID Sid;
} FIND_BY_SID_DATA, *PFIND_BY_SID_DATA;

typedef struct {
    DWORD NextEntryOffset;
    DWORD FileIndex;
    DWORD FileNameLength;
    WCHAR FileName[1];
} FIND_BY_SID_OUTPUT, *PFIND_BY_SID_OUTPUT;

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN2K */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
//
//  The following structures apply to Usn operations.
//

//
//==================== FSCTL_ENUM_USN_DATA ======================
//
// Structure for FSCTL_ENUM_USN_DATA
//

typedef struct {

    DWORDLONG StartFileReferenceNumber;
    USN LowUsn;
    USN HighUsn;

} MFT_ENUM_DATA_V0, *PMFT_ENUM_DATA_V0;

typedef struct {

    DWORDLONG StartFileReferenceNumber;
    USN LowUsn;
    USN HighUsn;
    WORD   MinMajorVersion;
    WORD   MaxMajorVersion;

} MFT_ENUM_DATA_V1, *PMFT_ENUM_DATA_V1;

#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef MFT_ENUM_DATA_V1 MFT_ENUM_DATA, *PMFT_ENUM_DATA;
#else
typedef MFT_ENUM_DATA_V0 MFT_ENUM_DATA, *PMFT_ENUM_DATA;
#endif

//
//==================== FSCTL_CREATE_USN_JOURNAL ======================
//
// Structure for FSCTL_CREATE_USN_JOURNAL
//

typedef struct {

    DWORDLONG MaximumSize;
    DWORDLONG AllocationDelta;

} CREATE_USN_JOURNAL_DATA, *PCREATE_USN_JOURNAL_DATA;

//
//==================== FSCTL_READ_FILE_USN_DATA ====================
//
// Structure for FSCTL_READ_FILE_USN_DATA
//

//
//  Windows 7 and earlier releases did not use an input buffer
//  for this FSCTL.  It is valid to omit this, and doing so
//  will default to MinMajorVersion == 2, MaxMajorVersion == 2.
//

typedef struct {

    WORD   MinMajorVersion;
    WORD   MaxMajorVersion;

} READ_FILE_USN_DATA, *PREAD_FILE_USN_DATA;

//
//==================== FSCTL_READ_USN_JOURNAL ======================
//
// Structure for FSCTL_READ_USN_JOURNAL
//

typedef struct {

    USN StartUsn;
    DWORD ReasonMask;
    DWORD ReturnOnlyOnClose;
    DWORDLONG Timeout;
    DWORDLONG BytesToWaitFor;
    DWORDLONG UsnJournalID;

} READ_USN_JOURNAL_DATA_V0, *PREAD_USN_JOURNAL_DATA_V0;

typedef struct {

    USN StartUsn;
    DWORD ReasonMask;
    DWORD ReturnOnlyOnClose;
    DWORDLONG Timeout;
    DWORDLONG BytesToWaitFor;
    DWORDLONG UsnJournalID;
    WORD   MinMajorVersion;
    WORD   MaxMajorVersion;

} READ_USN_JOURNAL_DATA_V1, *PREAD_USN_JOURNAL_DATA_V1;

#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef READ_USN_JOURNAL_DATA_V1 READ_USN_JOURNAL_DATA, *PREAD_USN_JOURNAL_DATA;
#else
typedef READ_USN_JOURNAL_DATA_V0 READ_USN_JOURNAL_DATA, *PREAD_USN_JOURNAL_DATA;
#endif

//
//==================== FSCTL_USN_TRACK_MODIFIED_RANGES ======================
//
// Structure for FSCTL_USN_TRACK_MODIFIED_RANGES
//

typedef struct {
    DWORD Flags;
    DWORD Unused;
    DWORDLONG ChunkSize;
    LONGLONG FileSizeThreshold;
} USN_TRACK_MODIFIED_RANGES, *PUSN_TRACK_MODIFIED_RANGES;

typedef struct {
    USN Usn;
} USN_RANGE_TRACK_OUTPUT, *PUSN_RANGE_TRACK_OUTPUT;

#define FLAG_USN_TRACK_MODIFIED_RANGES_ENABLE 0x00000001

//
//  The initial Major.Minor version of the Usn record will be 2.0.
//  In general, the MinorVersion may be changed if fields are added
//  to this structure in such a way that the previous version of the
//  software can still correctly the fields it knows about.  The
//  MajorVersion should only be changed if the previous version of
//  any software using this structure would incorrectly handle new
//  records due to structure changes.
//
//  The first update to this will force the structure to version 2.0.
//  This will add the extended information about the source as
//  well as indicate the file name offset within the structure.
//
//  The following structure is returned with these fsctls.
//
//      FSCTL_READ_USN_JOURNAL
//      FSCTL_READ_FILE_USN_DATA
//      FSCTL_ENUM_USN_DATA
//

typedef struct {

    DWORD RecordLength;
    WORD   MajorVersion;
    WORD   MinorVersion;
    DWORDLONG FileReferenceNumber;
    DWORDLONG ParentFileReferenceNumber;
    USN Usn;
    LARGE_INTEGER TimeStamp;
    DWORD Reason;
    DWORD SourceInfo;
    DWORD SecurityId;
    DWORD FileAttributes;
    WORD   FileNameLength;
    WORD   FileNameOffset;
    WCHAR FileName[1];

} USN_RECORD_V2, *PUSN_RECORD_V2;

typedef struct {

    DWORD RecordLength;
    WORD   MajorVersion;
    WORD   MinorVersion;
    FILE_ID_128 FileReferenceNumber;
    FILE_ID_128 ParentFileReferenceNumber;
    USN Usn;
    LARGE_INTEGER TimeStamp;
    DWORD Reason;
    DWORD SourceInfo;
    DWORD SecurityId;
    DWORD FileAttributes;
    WORD   FileNameLength;
    WORD   FileNameOffset;
    WCHAR FileName[1];

} USN_RECORD_V3, *PUSN_RECORD_V3;

typedef USN_RECORD_V2 USN_RECORD, *PUSN_RECORD;

typedef struct {
    DWORD RecordLength;
    WORD   MajorVersion;
    WORD   MinorVersion;
} USN_RECORD_COMMON_HEADER, *PUSN_RECORD_COMMON_HEADER;

typedef struct {
    LONGLONG Offset;
    LONGLONG Length;
} USN_RECORD_EXTENT, *PUSN_RECORD_EXTENT;

typedef struct {
    USN_RECORD_COMMON_HEADER Header;
    FILE_ID_128 FileReferenceNumber;
    FILE_ID_128 ParentFileReferenceNumber;
    USN Usn;
    DWORD Reason;
    DWORD SourceInfo;
    DWORD RemainingExtents;
    WORD   NumberOfExtents;
    WORD   ExtentSize;
    USN_RECORD_EXTENT Extents[1];
} USN_RECORD_V4, *PUSN_RECORD_V4;

typedef union {
    USN_RECORD_COMMON_HEADER Header;
    USN_RECORD_V2 V2;
    USN_RECORD_V3 V3;
    USN_RECORD_V4 V4;
} USN_RECORD_UNION, *PUSN_RECORD_UNION;

#define USN_PAGE_SIZE                    (0x1000)

#define USN_REASON_DATA_OVERWRITE        (0x00000001)
#define USN_REASON_DATA_EXTEND           (0x00000002)
#define USN_REASON_DATA_TRUNCATION       (0x00000004)
#define USN_REASON_NAMED_DATA_OVERWRITE  (0x00000010)
#define USN_REASON_NAMED_DATA_EXTEND     (0x00000020)
#define USN_REASON_NAMED_DATA_TRUNCATION (0x00000040)
#define USN_REASON_FILE_CREATE           (0x00000100)
#define USN_REASON_FILE_DELETE           (0x00000200)
#define USN_REASON_EA_CHANGE             (0x00000400)
#define USN_REASON_SECURITY_CHANGE       (0x00000800)
#define USN_REASON_RENAME_OLD_NAME       (0x00001000)
#define USN_REASON_RENAME_NEW_NAME       (0x00002000)
#define USN_REASON_INDEXABLE_CHANGE      (0x00004000)
#define USN_REASON_BASIC_INFO_CHANGE     (0x00008000)
#define USN_REASON_HARD_LINK_CHANGE      (0x00010000)
#define USN_REASON_COMPRESSION_CHANGE    (0x00020000)
#define USN_REASON_ENCRYPTION_CHANGE     (0x00040000)
#define USN_REASON_OBJECT_ID_CHANGE      (0x00080000)
#define USN_REASON_REPARSE_POINT_CHANGE  (0x00100000)
#define USN_REASON_STREAM_CHANGE         (0x00200000)
#define USN_REASON_TRANSACTED_CHANGE     (0x00400000)
#define USN_REASON_INTEGRITY_CHANGE      (0x00800000)
#define USN_REASON_DESIRED_STORAGE_CLASS_CHANGE (0x01000000)
#define USN_REASON_CLOSE                 (0x80000000)

//
//==================== FSCTL_QUERY_USN_JOURNAL ======================
//
//  Structure for FSCTL_QUERY_USN_JOURNAL
//

typedef struct {

    DWORDLONG UsnJournalID;
    USN FirstUsn;
    USN NextUsn;
    USN LowestValidUsn;
    USN MaxUsn;
    DWORDLONG MaximumSize;
    DWORDLONG AllocationDelta;

} USN_JOURNAL_DATA_V0, *PUSN_JOURNAL_DATA_V0;

typedef struct {

    DWORDLONG UsnJournalID;
    USN FirstUsn;
    USN NextUsn;
    USN LowestValidUsn;
    USN MaxUsn;
    DWORDLONG MaximumSize;
    DWORDLONG AllocationDelta;
    WORD   MinSupportedMajorVersion;
    WORD   MaxSupportedMajorVersion;

} USN_JOURNAL_DATA_V1, *PUSN_JOURNAL_DATA_V1;

typedef struct {

    DWORDLONG UsnJournalID;
    USN FirstUsn;
    USN NextUsn;
    USN LowestValidUsn;
    USN MaxUsn;
    DWORDLONG MaximumSize;
    DWORDLONG AllocationDelta;
    WORD   MinSupportedMajorVersion;
    WORD   MaxSupportedMajorVersion;
    DWORD Flags;
    DWORDLONG RangeTrackChunkSize;
    LONGLONG RangeTrackFileSizeThreshold;

} USN_JOURNAL_DATA_V2, *PUSN_JOURNAL_DATA_V2;

#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef USN_JOURNAL_DATA_V1 USN_JOURNAL_DATA, *PUSN_JOURNAL_DATA;
#else
typedef USN_JOURNAL_DATA_V0 USN_JOURNAL_DATA, *PUSN_JOURNAL_DATA;
#endif

//
//==================== FSCTL_DELETE_USN_JOURNAL ======================
//
//  Structure for FSCTL_DELETE_USN_JOURNAL
//

typedef struct {

    DWORDLONG UsnJournalID;
    DWORD DeleteFlags;

} DELETE_USN_JOURNAL_DATA, *PDELETE_USN_JOURNAL_DATA;

#define USN_DELETE_FLAG_DELETE              (0x00000001)
#define USN_DELETE_FLAG_NOTIFY              (0x00000002)

#define USN_DELETE_VALID_FLAGS              (0x00000003)

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN2K */

//
//==================== FSCTL_MARK_HANDLE ======================
//
//  Structure for FSCTL_MARK_HANDLE
//

#if (NTDDI_VERSION >= NTDDI_WIN2K)

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201)       // unnamed struct

typedef struct _MARK_HANDLE_INFO {

#if (NTDDI_VERSION >= NTDDI_WIN8)
    union {
        DWORD UsnSourceInfo;
        DWORD CopyNumber;
    } DUMMYUNIONNAME;
#else
    DWORD UsnSourceInfo;
#endif /*NTDDI_VERSION >= NTDDI_WIN8 */

    HANDLE VolumeHandle;
    DWORD HandleInfo;           //Flags

} MARK_HANDLE_INFO, *PMARK_HANDLE_INFO;

#if defined(_WIN64)
//
//  32/64 Bit thunking support structure
//

typedef struct _MARK_HANDLE_INFO32 {

#if (NTDDI_VERSION >= NTDDI_WIN8)
    union {
        DWORD UsnSourceInfo;
        DWORD CopyNumber;
    } DUMMYUNIONNAME;
#else
    DWORD UsnSourceInfo;
#endif /*NTDDI_VERSION >= NTDDI_WIN8 */
    UINT32 VolumeHandle;
    DWORD HandleInfo;           //Flags

} MARK_HANDLE_INFO32, *PMARK_HANDLE_INFO32;
#endif

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201)       // unnamed struct
#endif

//
//  Flags for the additional source information above.
//  To set any of these values required a volume DASD handle to be specified in
//  VolumeHandle field.
//
//      USN_SOURCE_DATA_MANAGEMENT - Service is not modifying the external view
//          of any part of the file.  Typical case is HSM moving data to
//          and from external storage.
//
//      USN_SOURCE_AUXILIARY_DATA - Service is not modifying the external view
//          of the file with regard to the application that created this file.
//          Can be used to add private data streams to a file.
//
//      USN_SOURCE_REPLICATION_MANAGEMENT - Service is modifying a file to match
//          the contents of the same file which exists in another member of the
//          replica set.
//
//      USN_SOURCE_CLIENT_REPLICATION_MANAGEMENT - Replication is being performed
//          on client systems either from the cloud or servers.  A volume handle
//          is not required to set this value
//

#define USN_SOURCE_DATA_MANAGEMENT                  (0x00000001)
#define USN_SOURCE_AUXILIARY_DATA                   (0x00000002)
#define USN_SOURCE_REPLICATION_MANAGEMENT           (0x00000004)
#define USN_SOURCE_CLIENT_REPLICATION_MANAGEMENT    (0x00000008)

#define USN_SOURCE_VALID_FLAGS      (USN_SOURCE_DATA_MANAGEMENT |               \
                                     USN_SOURCE_AUXILIARY_DATA |                \
                                     USN_SOURCE_REPLICATION_MANAGEMENT |        \
                                     USN_SOURCE_CLIENT_REPLICATION_MANAGEMENT)

//
//  Flags for the HandleInfo field above:
//
//  ------ Introduced in W2K:
//  MARK_HANDLE_PROTECT_CLUSTERS - disallow any defragmenting (FSCTL_MOVE_FILE) until the
//      the handle is closed
//
//  ------ Introduced in Vista:
//  MARK_HANDLE_TXF_SYSTEM_LOG - indicates that this stream is being used as the Txf
//      log for an RM on the volume.  Must be called in the kernel using IRP_MN_KERNEL_CALL.
//
//  MARK_HANDLE_NOT_TXF_SYSTEM_LOG - indicates that this component is no longer using this
//      object as a TxF log file.
//
//  ------ Introduced in Win7:
//  MARK_HANDLE_REALTIME - only supported by the UDFS file system.  Marks the device
//      to do realtime streaming of video
//
//  MARK_HANDLE_NOT_REALTIME - only supported by the UDFS file system.  Marks the device
//      to no longer do realtime streaming of video
//
//  MARK_HANDLE_CLOUD_SYNC - this flag is deprecated and is no longer used
//
//  ------ Introduced in Win8
//  MARK_HANDLE_READ_COPY - indicates the data must be read from the specified copy
//      of data.  Only supported for spaces redundent volumes.
//
//  MARK_HANDLE_NOT_READ_COPY - indicates the data is no longer to be read from a specific copy.
//
//  ------ Introduced in WinBlue (win 8.1)
//  MARK_HANDLE_FILTER_METADATA - Flag reserved for internal Microsoft use
//
//  MARK_HANDLE_RETURN_PURGE_FAILURE - When intermixing memory mapped/cached IO with
//      non-cached IO the system attempts, when a non-cached io is issued, to purge
//      memory mappings for the range of the non-cached IO.  If these purges fail
//      the system normally does not return the failure to the caller which can
//      lead to corrupted state (which is why the documentation says to not do this).
//      This flag tells the system to return purge failures for the given handle
//      so the application can better handle this situation
//
//  ------ Introduced in WinThreshold (win10)
//  MARK_HANDLE_DISABLE_FILE_METADATA_OPTIMIZATION - This disabled the FRS compaction
//      feature on the given file.
//
//  MARK_HANDLE_ENABLE_USN_SOURCE_ON_PAGING_IO - Tells NTFS to set the given UsnSourceInfo
//      value on Paging writes in the USN Journal.  Traditionally this was not
//      done on paging writes since you did not know what thread made the given
//      changes.  This is an override.  This only works of the FileObject MM is
//      holding on to has this state associated with it.
//
//  MARK_HANDLE_SKIP_COHERENCY_SYNC_DISALLOW_WRITES - Setting this flag tells
//      the system that writes are not allowed on this file.  If someone tries
//      to open the file for write access, the operation is failed with STATUS_ACCESS_DENIED.
//      If a write is seen the operation is failed with STATUS_MARKED_TO_DISALLOW_WRITES
//
//  ------ Introduced in RS4 (win10)
//  MARK_HANDLE_ENABLE_CPU_CACHE - Flag reserved for internal Microsoft use
//
//  ------ Introduced in Cobalt (win10)
//  MARK_HANDLE_SUPPRESS_VOLUME_OPEN_FLUSH - Normally, on the first read/write operation
//      on a volume handle (DASD open) the file system flushes the volume.  This can
//      have performance consequences in certain scenarios.  If this flag is set on
//      a volume handle it will suppress that flush on first IO.
//

#define MARK_HANDLE_PROTECT_CLUSTERS                    (0x00000001)
//#define ReservedForFutureUse                          (0x00000002)
#define MARK_HANDLE_TXF_SYSTEM_LOG                      (0x00000004)
#define MARK_HANDLE_NOT_TXF_SYSTEM_LOG                  (0x00000008)
//#define ReservedForFutureUse                          (0x00000010)

#endif /* NTDDI_VERSION >= NTDDI_WIN2K */

#if (NTDDI_VERSION >= NTDDI_WIN7)

#define MARK_HANDLE_REALTIME                            (0x00000020)
#define MARK_HANDLE_NOT_REALTIME                        (0x00000040)
#define MARK_HANDLE_CLOUD_SYNC                          (0x00000800)    //deprecated flag - do not use

#endif /* NTDDI_VERSION >= NTDDI_WIN7 */

#if (NTDDI_VERSION >= NTDDI_WIN8)

#define MARK_HANDLE_READ_COPY                           (0x00000080)
#define MARK_HANDLE_NOT_READ_COPY                       (0x00000100)

#endif /*NTDDI_VERSION >= NTDDI_WIN8 */

#if (NTDDI_VERSION >= NTDDI_WINBLUE) || (NTDDI_VERSION >= NTDDI_WIN7)       //Win7 check is for backward compatibility

#define MARK_HANDLE_FILTER_METADATA                     (0x00000200)        // 8.1 and newer
#define MARK_HANDLE_RETURN_PURGE_FAILURE                (0x00000400)        // 8.1 and newer
//                                                      (0x00000800)        // defined above: MARK_HANDLE_CLOUD_SYNC

#endif /*NTDDI_VERSION >= NTDDI_WINBLUE */

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#define MARK_HANDLE_DISABLE_FILE_METADATA_OPTIMIZATION  (0x00001000)
#define MARK_HANDLE_ENABLE_USN_SOURCE_ON_PAGING_IO      (0x00002000)
#define MARK_HANDLE_SKIP_COHERENCY_SYNC_DISALLOW_WRITES (0x00004000)

#endif /*NTDDI_VERSION >= NTDDI_WINTHRESHOLD */

#if (NTDDI_VERSION >= NTDDI_WIN10_CO)

#define MARK_HANDLE_SUPPRESS_VOLUME_OPEN_FLUSH          (0x00008000)

#endif /*NTDDI_VERSION >= NTDDI_WIN10_CO */

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)

#define MARK_HANDLE_ENABLE_CPU_CACHE                    (0x10000000)

#endif /*NTDDI_VERSION >= NTDDI_WIN10_RS4 */

//
//==================== FSCTL_SECURITY_ID_CHECK ======================
//
// Structure for FSCTL_SECURITY_ID_CHECK
//

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)

typedef struct {

    ACCESS_MASK DesiredAccess;
    DWORD SecurityIds[1];

} BULK_SECURITY_TEST_DATA, *PBULK_SECURITY_TEST_DATA;
#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN2K */

//
//==================== FSCTL_IS_VOLUME_DIRTY ======================
//
//  Output flags for the FSCTL_IS_VOLUME_DIRTY is a DWORD
//

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)

#define VOLUME_IS_DIRTY                  (0x00000001)
#define VOLUME_UPGRADE_SCHEDULED         (0x00000002)
#define VOLUME_SESSION_OPEN              (0x00000004)

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN2K */

//
//==================== FSCTL_FILE_PREFETCH ======================
//

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)

typedef struct _FILE_PREFETCH {
    DWORD Type;
    DWORD Count;
    DWORDLONG Prefetch[1];
} FILE_PREFETCH, *PFILE_PREFETCH;

typedef struct _FILE_PREFETCH_EX {
    DWORD Type;
    DWORD Count;
    PVOID Context;
    DWORDLONG Prefetch[1];
} FILE_PREFETCH_EX, *PFILE_PREFETCH_EX;

#define FILE_PREFETCH_TYPE_FOR_CREATE       0x1
#define FILE_PREFETCH_TYPE_FOR_DIRENUM      0x2
#define FILE_PREFETCH_TYPE_FOR_CREATE_EX    0x3
#define FILE_PREFETCH_TYPE_FOR_DIRENUM_EX   0x4

#define FILE_PREFETCH_TYPE_MAX              0x4

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN2K */

//
//==================== FSCTL_FILESYSTEM_GET_STATISTICS ======================
//
// Structures for FSCTL_FILESYSTEM_GET_STATISTICS
//
// Filesystem performance counters
//

typedef struct _FILESYSTEM_STATISTICS {

    WORD   FileSystemType;
    WORD   Version;                     // currently version 1

    DWORD SizeOfCompleteStructure;      // must by a multiple of 64 bytes

    DWORD UserFileReads;
    DWORD UserFileReadBytes;
    DWORD UserDiskReads;
    DWORD UserFileWrites;
    DWORD UserFileWriteBytes;
    DWORD UserDiskWrites;

    DWORD MetaDataReads;
    DWORD MetaDataReadBytes;
    DWORD MetaDataDiskReads;
    DWORD MetaDataWrites;
    DWORD MetaDataWriteBytes;
    DWORD MetaDataDiskWrites;

    //
    //  The file system's private structure is appended here.
    //

} FILESYSTEM_STATISTICS, *PFILESYSTEM_STATISTICS;

// values for FS_STATISTICS.FileSystemType

#define FILESYSTEM_STATISTICS_TYPE_NTFS     1
#define FILESYSTEM_STATISTICS_TYPE_FAT      2
#define FILESYSTEM_STATISTICS_TYPE_EXFAT    3
#define FILESYSTEM_STATISTICS_TYPE_REFS     4

//
//  File System Specific Statistics Data
//

typedef struct _FAT_STATISTICS {
    DWORD CreateHits;
    DWORD SuccessfulCreates;
    DWORD FailedCreates;

    DWORD NonCachedReads;
    DWORD NonCachedReadBytes;
    DWORD NonCachedWrites;
    DWORD NonCachedWriteBytes;

    DWORD NonCachedDiskReads;
    DWORD NonCachedDiskWrites;
} FAT_STATISTICS, *PFAT_STATISTICS;

typedef struct _EXFAT_STATISTICS {
    DWORD CreateHits;
    DWORD SuccessfulCreates;
    DWORD FailedCreates;

    DWORD NonCachedReads;
    DWORD NonCachedReadBytes;
    DWORD NonCachedWrites;
    DWORD NonCachedWriteBytes;

    DWORD NonCachedDiskReads;
    DWORD NonCachedDiskWrites;
} EXFAT_STATISTICS, *PEXFAT_STATISTICS;

typedef struct _NTFS_STATISTICS {

    DWORD LogFileFullExceptions;
    DWORD OtherExceptions;

    //
    // Other meta data io's
    //

    DWORD MftReads;
    DWORD MftReadBytes;
    DWORD MftWrites;
    DWORD MftWriteBytes;
    struct {
        WORD   Write;
        WORD   Create;
        WORD   SetInfo;
        WORD   Flush;
    } MftWritesUserLevel;

    WORD   MftWritesFlushForLogFileFull;
    WORD   MftWritesLazyWriter;
    WORD   MftWritesUserRequest;

    DWORD Mft2Writes;
    DWORD Mft2WriteBytes;
    struct {
        WORD   Write;
        WORD   Create;
        WORD   SetInfo;
        WORD   Flush;
    } Mft2WritesUserLevel;

    WORD   Mft2WritesFlushForLogFileFull;
    WORD   Mft2WritesLazyWriter;
    WORD   Mft2WritesUserRequest;

    DWORD RootIndexReads;
    DWORD RootIndexReadBytes;
    DWORD RootIndexWrites;
    DWORD RootIndexWriteBytes;

    DWORD BitmapReads;
    DWORD BitmapReadBytes;
    DWORD BitmapWrites;
    DWORD BitmapWriteBytes;

    WORD   BitmapWritesFlushForLogFileFull;
    WORD   BitmapWritesLazyWriter;
    WORD   BitmapWritesUserRequest;

    struct {
        WORD   Write;
        WORD   Create;
        WORD   SetInfo;
    } BitmapWritesUserLevel;

    DWORD MftBitmapReads;
    DWORD MftBitmapReadBytes;
    DWORD MftBitmapWrites;
    DWORD MftBitmapWriteBytes;

    WORD   MftBitmapWritesFlushForLogFileFull;
    WORD   MftBitmapWritesLazyWriter;
    WORD   MftBitmapWritesUserRequest;

    struct {
        WORD   Write;
        WORD   Create;
        WORD   SetInfo;
        WORD   Flush;
    } MftBitmapWritesUserLevel;

    DWORD UserIndexReads;
    DWORD UserIndexReadBytes;
    DWORD UserIndexWrites;
    DWORD UserIndexWriteBytes;

    //
    // Additions for NT 5.0
    //

    DWORD LogFileReads;
    DWORD LogFileReadBytes;
    DWORD LogFileWrites;
    DWORD LogFileWriteBytes;

    struct {
        DWORD Calls;                // number of individual calls to allocate clusters
        DWORD Clusters;             // number of clusters allocated
        DWORD Hints;                // number of times a hint was specified

        DWORD RunsReturned;         // number of runs used to satisfy all the requests

        DWORD HintsHonored;         // number of times the hint was useful
        DWORD HintsClusters;        // number of clusters allocated via the hint
        DWORD Cache;                // number of times the cache was useful other than the hint
        DWORD CacheClusters;        // number of clusters allocated via the cache other than the hint
        DWORD CacheMiss;            // number of times the cache wasn't useful
        DWORD CacheMissClusters;    // number of clusters allocated without the cache
    } Allocate;

    //
    //  Additions for Windows 8.1
    //

    DWORD DiskResourcesExhausted;

    //
    //  All future expansion of this structure needs to be in NTFS_STATISTICS_EX starting Windows 10
    //

} NTFS_STATISTICS, *PNTFS_STATISTICS;

typedef struct _FILESYSTEM_STATISTICS_EX {

    WORD   FileSystemType;
    WORD   Version;                     // currently version 1

    DWORD SizeOfCompleteStructure;      // must by a multiple of 64 bytes

    DWORDLONG UserFileReads;
    DWORDLONG UserFileReadBytes;
    DWORDLONG UserDiskReads;
    DWORDLONG UserFileWrites;
    DWORDLONG UserFileWriteBytes;
    DWORDLONG UserDiskWrites;

    DWORDLONG MetaDataReads;
    DWORDLONG MetaDataReadBytes;
    DWORDLONG MetaDataDiskReads;
    DWORDLONG MetaDataWrites;
    DWORDLONG MetaDataWriteBytes;
    DWORDLONG MetaDataDiskWrites;

    //
    //  The file system's private structure is appended here.
    //

} FILESYSTEM_STATISTICS_EX, *PFILESYSTEM_STATISTICS_EX;

typedef struct _NTFS_STATISTICS_EX {

    DWORD LogFileFullExceptions;
    DWORD OtherExceptions;

    //
    // Other meta data io's
    //

    DWORDLONG MftReads;
    DWORDLONG MftReadBytes;
    DWORDLONG MftWrites;
    DWORDLONG MftWriteBytes;
    struct {
        DWORD Write;
        DWORD Create;
        DWORD SetInfo;
        DWORD Flush;
    } MftWritesUserLevel;

    DWORD MftWritesFlushForLogFileFull;
    DWORD MftWritesLazyWriter;
    DWORD MftWritesUserRequest;

    DWORDLONG Mft2Writes;
    DWORDLONG Mft2WriteBytes;
    struct {
        DWORD Write;
        DWORD Create;
        DWORD SetInfo;
        DWORD Flush;
    } Mft2WritesUserLevel;

    DWORD Mft2WritesFlushForLogFileFull;
    DWORD Mft2WritesLazyWriter;
    DWORD Mft2WritesUserRequest;

    DWORDLONG RootIndexReads;
    DWORDLONG RootIndexReadBytes;
    DWORDLONG RootIndexWrites;
    DWORDLONG RootIndexWriteBytes;

    DWORDLONG BitmapReads;
    DWORDLONG BitmapReadBytes;
    DWORDLONG BitmapWrites;
    DWORDLONG BitmapWriteBytes;

    DWORD BitmapWritesFlushForLogFileFull;
    DWORD BitmapWritesLazyWriter;
    DWORD BitmapWritesUserRequest;

    struct {
        DWORD Write;
        DWORD Create;
        DWORD SetInfo;
        DWORD Flush;
    } BitmapWritesUserLevel;

    DWORDLONG MftBitmapReads;
    DWORDLONG MftBitmapReadBytes;
    DWORDLONG MftBitmapWrites;
    DWORDLONG MftBitmapWriteBytes;

    DWORD MftBitmapWritesFlushForLogFileFull;
    DWORD MftBitmapWritesLazyWriter;
    DWORD MftBitmapWritesUserRequest;

    struct {
        DWORD Write;
        DWORD Create;
        DWORD SetInfo;
        DWORD Flush;
    } MftBitmapWritesUserLevel;

    DWORDLONG UserIndexReads;
    DWORDLONG UserIndexReadBytes;
    DWORDLONG UserIndexWrites;
    DWORDLONG UserIndexWriteBytes;

    //
    // Additions for NT 5.0
    //

    DWORDLONG LogFileReads;
    DWORDLONG LogFileReadBytes;
    DWORDLONG LogFileWrites;
    DWORDLONG LogFileWriteBytes;

    struct {
        DWORD Calls;                    // number of individual calls to allocate clusters
        DWORD RunsReturned;             // number of runs used to satisfy all the requests
        DWORD Hints;                    // number of times a hint was specified
        DWORD HintsHonored;             // number of times the hint was useful
        DWORD Cache;                    // number of times the cache was useful other than the hint
        DWORD CacheMiss;                // number of times the cache wasn't useful

        DWORDLONG Clusters;             // number of clusters allocated
        DWORDLONG HintsClusters;        // number of clusters allocated via the hint
        DWORDLONG CacheClusters;        // number of clusters allocated via the cache other than the hint
        DWORDLONG CacheMissClusters;    // number of clusters allocated without the cache
    } Allocate;

    //
    //  Additions for Windows 8.1
    //

    DWORD DiskResourcesExhausted;

    //
    //  Additions for Windows 10
    //

    DWORDLONG VolumeTrimCount;
    DWORDLONG VolumeTrimTime;
    DWORDLONG VolumeTrimByteCount;

    DWORDLONG FileLevelTrimCount;
    DWORDLONG FileLevelTrimTime;
    DWORDLONG FileLevelTrimByteCount;

    DWORDLONG VolumeTrimSkippedCount;
    DWORDLONG VolumeTrimSkippedByteCount;

    //
    //  Additions for NtfsFillStatInfoFromMftRecord
    //

    DWORDLONG NtfsFillStatInfoFromMftRecordCalledCount;
    DWORDLONG NtfsFillStatInfoFromMftRecordBailedBecauseOfAttributeListCount;
    DWORDLONG NtfsFillStatInfoFromMftRecordBailedBecauseOfNonResReparsePointCount;

} NTFS_STATISTICS_EX, *PNTFS_STATISTICS_EX;

//
//==================== FSCTL_SET_OBJECT_ID ================================
//==================== FSCTL_GET_OBJECT_ID ================================
//==================== FSCTL_CREATE_OR_GET_OBJECT_ID ======================
//
//  Structures for FSCTL_SET_OBJECT_ID, FSCTL_GET_OBJECT_ID, and
//  FSCTL_CREATE_OR_GET_OBJECT_ID
//

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201)       // unnamed struct

typedef struct _FILE_OBJECTID_BUFFER {

    //
    //  This is the portion of the object id that is indexed.
    //

    BYTE  ObjectId[16];

    //
    //  This portion of the object id is not indexed, it's just
    //  some metadata for the user's benefit.
    //

    union {
        struct {
            BYTE  BirthVolumeId[16];
            BYTE  BirthObjectId[16];
            BYTE  DomainId[16];
        } DUMMYSTRUCTNAME;
        BYTE  ExtendedInfo[48];
    } DUMMYUNIONNAME;

} FILE_OBJECTID_BUFFER, *PFILE_OBJECTID_BUFFER;

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning( default : 4201 ) /* nonstandard extension used : nameless struct/union */
#endif

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN2K */

//
//==================== FSCTL_SET_SPARSE ======================
//
// Structure for FSCTL_SET_SPARSE
//

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)

typedef struct _FILE_SET_SPARSE_BUFFER {
    BOOLEAN SetSparse;
} FILE_SET_SPARSE_BUFFER, *PFILE_SET_SPARSE_BUFFER;

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN2K */

//
//==================== FSCTL_SET_ZERO_DATA ======================
//
// Structure for FSCTL_SET_ZERO_DATA
//

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)

typedef struct _FILE_ZERO_DATA_INFORMATION {

    LARGE_INTEGER FileOffset;
    LARGE_INTEGER BeyondFinalZero;

} FILE_ZERO_DATA_INFORMATION, *PFILE_ZERO_DATA_INFORMATION;

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN2K */

#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)

typedef struct _FILE_ZERO_DATA_INFORMATION_EX {

    LARGE_INTEGER FileOffset;
    LARGE_INTEGER BeyondFinalZero;
    DWORD Flags;

} FILE_ZERO_DATA_INFORMATION_EX, *PFILE_ZERO_DATA_INFORMATION_EX;

//
//  When set tells the file system to not purge the cache for the given range
//  being zeroed
//

#define FILE_ZERO_DATA_INFORMATION_FLAG_PRESERVE_CACHED_DATA       (0x00000001)

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD */

//
//==================== FSCTL_QUERY_ALLOCATED_RANGES ======================
//
// Structure for FSCTL_QUERY_ALLOCATED_RANGES
//

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)

//
// Querying the allocated ranges requires an output buffer to store the
// allocated ranges and an input buffer to specify the range to query.
// The input buffer contains a single entry, the output buffer is an
// array of the following structure.
//

typedef struct _FILE_ALLOCATED_RANGE_BUFFER {

    LARGE_INTEGER FileOffset;
    LARGE_INTEGER Length;

} FILE_ALLOCATED_RANGE_BUFFER, *PFILE_ALLOCATED_RANGE_BUFFER;

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN2K */

//
//====================== FSCTL_SET_ENCRYPTION ===============================
//====================== FSCTL_WRITE_RAW_ENCRYPTED ==========================
//====================== FSCTL_READ_RAW_ENCRYPTED ===========================
//
// Structures for FSCTL_SET_ENCRYPTION, FSCTL_WRITE_RAW_ENCRYPTED, and FSCTL_READ_RAW_ENCRYPTED
//

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)

//
//  The input buffer to set encryption indicates whether we are to encrypt/decrypt a file
//  or an individual stream.
//

typedef struct _ENCRYPTION_BUFFER {

    DWORD EncryptionOperation;
    BYTE  Private[1];

} ENCRYPTION_BUFFER, *PENCRYPTION_BUFFER;

#define FILE_SET_ENCRYPTION         0x00000001
#define FILE_CLEAR_ENCRYPTION       0x00000002
#define STREAM_SET_ENCRYPTION       0x00000003
#define STREAM_CLEAR_ENCRYPTION     0x00000004

#define MAXIMUM_ENCRYPTION_VALUE    0x00000004

//
//  The optional output buffer to set encryption indicates that the last encrypted
//  stream in a file has been marked as decrypted.
//

typedef struct _DECRYPTION_STATUS_BUFFER {

    BOOLEAN NoEncryptedStreams;

} DECRYPTION_STATUS_BUFFER, *PDECRYPTION_STATUS_BUFFER;

#define ENCRYPTION_FORMAT_DEFAULT        (0x01)

//
//  Request Encrypted Data structure.  This is used to indicate
//  the range of the file to read.  It also describes the
//  output buffer used to return the data.
//

typedef struct _REQUEST_RAW_ENCRYPTED_DATA {

    //
    //  Requested file offset and requested length to read.
    //  The fsctl will round the starting offset down
    //  to a file system boundary.  It will also
    //  round the length up to a file system boundary.
    //

    LONGLONG FileOffset;
    DWORD Length;

} REQUEST_RAW_ENCRYPTED_DATA, *PREQUEST_RAW_ENCRYPTED_DATA;

//
//  Encrypted Data Information structure.  This structure
//  is used to return raw encrypted data from a file in
//  order to perform off-line recovery.  The data will be
//  encrypted or encrypted and compressed.  The off-line
//  service will need to use the encryption and compression
//  format information to recover the file data.  In the
//  event that the data is both encrypted and compressed then
//  the decryption must occur before decompression.  All
//  the data units below must be encrypted and compressed
//  with the same format.
//
//  The data will be returned in units.  The data unit size
//  will be fixed per request.  If the data is compressed
//  then the data unit size will be the compression unit size.
//
//  This structure is at the beginning of the buffer used to
//  return the encrypted data.  The actual raw bytes from
//  the file will follow this buffer.  The offset of the
//  raw bytes from the beginning of this structure is
//  specified in the REQUEST_RAW_ENCRYPTED_DATA structure
//  described above.
//

typedef struct _ENCRYPTED_DATA_INFO {

    //
    //  This is the file offset for the first entry in the
    //  data block array.  The file system will round
    //  the requested start offset down to a boundary
    //  that is consistent with the format of the file.
    //

    DWORDLONG StartingFileOffset;

    //
    //  Data offset in output buffer.  The output buffer
    //  begins with an ENCRYPTED_DATA_INFO structure.
    //  The file system will then store the raw bytes from
    //  disk beginning at the following offset within the
    //  output buffer.
    //

    DWORD OutputBufferOffset;

    //
    //  The number of bytes being returned that are within
    //  the size of the file.  If this value is less than
    //  (NumberOfDataBlocks << DataUnitShift), it means the
    //  end of the file occurs within this transfer.  Any
    //  data beyond file size is invalid and was never
    //  passed to the encryption driver.
    //

    DWORD BytesWithinFileSize;

    //
    //  The number of bytes being returned that are below
    //  valid data length.  If this value is less than
    //  (NumberOfDataBlocks << DataUnitShift), it means the
    //  end of the valid data occurs within this transfer.
    //  After decrypting the data from this transfer, any
    //  byte(s) beyond valid data length must be zeroed.
    //

    DWORD BytesWithinValidDataLength;

    //
    //  Code for the compression format as defined in
    //  ntrtl.h.  Note that COMPRESSION_FORMAT_NONE
    //  and COMPRESSION_FORMAT_DEFAULT are invalid if
    //  any of the described chunks are compressed.
    //

    WORD   CompressionFormat;

    //
    //  The DataUnit is the granularity used to access the
    //  disk.  It will be the same as the compression unit
    //  size for a compressed file.  For an uncompressed
    //  file, it will be some cluster-aligned power of 2 that
    //  the file system deems convenient.  A caller should
    //  not expect that successive calls will have the
    //  same data unit shift value as the previous call.
    //
    //  Since chunks and compression units are expected to be
    //  powers of 2 in size, we express them log2.  So, for
    //  example (1 << ChunkShift) == ChunkSizeInBytes.  The
    //  ClusterShift indicates how much space must be saved
    //  to successfully compress a compression unit - each
    //  successfully compressed data unit must occupy
    //  at least one cluster less in bytes than an uncompressed
    //  data block unit.
    //

    BYTE  DataUnitShift;
    BYTE  ChunkShift;
    BYTE  ClusterShift;

    //
    //  The format for the encryption.
    //

    BYTE  EncryptionFormat;

    //
    //  This is the number of entries in the data block size
    //  array.
    //

    WORD   NumberOfDataBlocks;

    //
    //  This is an array of sizes in the data block array.  There
    //  must be one entry in this array for each data block
    //  read from disk.  The size has a different meaning
    //  depending on whether the file is compressed.
    //
    //  A size of zero always indicates that the final data consists entirely
    //  of zeroes.  There is no decryption or decompression to
    //  perform.
    //
    //  If the file is compressed then the data block size indicates
    //  whether this block is compressed.  A size equal to
    //  the block size indicates that the corresponding block did
    //  not compress.  Any other non-zero size indicates the
    //  size of the compressed data which needs to be
    //  decrypted/decompressed.
    //
    //  If the file is not compressed then the data block size
    //  indicates the amount of data within the block that
    //  needs to be decrypted.  Any other non-zero size indicates
    //  that the remaining bytes in the data unit within the file
    //  consists of zeros.  An example of this is when the
    //  the read spans the valid data length of the file.  There
    //  is no data to decrypt past the valid data length.
    //

    DWORD DataBlockSize[ANYSIZE_ARRAY];

} ENCRYPTED_DATA_INFO, *PENCRYPTED_DATA_INFO;

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN2K */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)

//
//  Extended encryption structure for read/write raw encrypted operations.
//  This was needed so we can explicitly indicate if a file is sparse or not
//
//  Flag to indicate the encrypted file is sparse
//

#define ENCRYPTED_DATA_INFO_SPARSE_FILE        0x00000001

//
//  This encrypted file data is from a filesystem which has support
//  for sparse data ranges (e.g. ReFS) for non-sparse files.
//

#define ENCRYPTED_DATA_INFO_SPARSE_DATA        0x00000002

//
//  This encrypted file data is for a sparse file from a filesystem
//  which has a 4k sparse data unit size.  It cannot be restored to
//  a filesystem with larger sparse unit size.
//

#define ENCRYPTED_DATA_INFO_4K_SPARSE_UNIT     0x00000004

typedef struct _EXTENDED_ENCRYPTED_DATA_INFO {

    //
    //  This is really a 4 byte character array which
    //  must have the value "EXTD".  We use this
    //  to determine if we should read the extended data
    //  or not.
    //

    DWORD ExtendedCode;

    //
    //  The length of the extended data structure
    //

    DWORD Length;

    //
    //  Encrypted data flags
    //

    DWORD Flags;
    DWORD Reserved;

} EXTENDED_ENCRYPTED_DATA_INFO, *PEXTENDED_ENCRYPTED_DATA_INFO;

#endif /*(_WIN32_WINNT >= _WIN32_WINNT_WIN7)*/

//
//======================== FSCTL_READ_FROM_PLEX ===========================
//
//  Request Plex Read Data structure.  This is used to indicate
//  the range of the file to read.  It also describes
//  which plex to perform the read from.
//

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)

typedef struct _PLEX_READ_DATA_REQUEST {

    //
    //  Requested offset and length to read.
    //  The offset can be the virtual offset (vbo) in to a file,
    //  or a volume. In the case of a file offset,
    //  the fsd will round the starting offset down
    //  to a file system boundary.  It will also
    //  round the length up to a file system boundary and
    //  enforce any other applicable limits.
    //

    LARGE_INTEGER ByteOffset;
    DWORD ByteLength;
    DWORD PlexNumber;

} PLEX_READ_DATA_REQUEST, *PPLEX_READ_DATA_REQUEST;

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN2K */

//
//======================== FSCTL_SIS_COPYFILE ===========================
//
// Source and destination file names are passed in the FileNameBuffer.
// Both strings are null terminated, with the source name starting at
// the beginning of FileNameBuffer, and the destination name immediately
// following.  Length fields include terminating nulls.
//

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)

typedef struct _SI_COPYFILE {
    DWORD SourceFileNameLength;
    DWORD DestinationFileNameLength;
    DWORD Flags;
    WCHAR FileNameBuffer[1];
} SI_COPYFILE, *PSI_COPYFILE;

#define COPYFILE_SIS_LINK       0x0001              // Copy only if source is SIS
#define COPYFILE_SIS_REPLACE    0x0002              // Replace destination if it exists, otherwise don't.
#define COPYFILE_SIS_FLAGS      0x0003

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN2K */

//
//======================== FSCTL_MAKE_MEDIA_COMPATIBLE ===========================
//
//  Input parameter structure for FSCTL_MAKE_MEDIA_COMPATIBLE
//

#if (_WIN32_WINNT >= _WIN32_WINNT_VISTA)

typedef struct _FILE_MAKE_COMPATIBLE_BUFFER {
    BOOLEAN CloseDisc;
} FILE_MAKE_COMPATIBLE_BUFFER, *PFILE_MAKE_COMPATIBLE_BUFFER;

//
//======================== FSCTL_SET_DEFECT_MANAGEMENT ===========================
//
//  Input parameter structure for FSCTL_SET_DEFECT_MANAGEMENT
//

typedef struct _FILE_SET_DEFECT_MGMT_BUFFER {
    BOOLEAN Disable;
} FILE_SET_DEFECT_MGMT_BUFFER, *PFILE_SET_DEFECT_MGMT_BUFFER;

//
//======================== FSCTL_QUERY_SPARING_INFO ===========================
//
//  Output structure for FSCTL_QUERY_SPARING_INFO
//

typedef struct _FILE_QUERY_SPARING_BUFFER {
    DWORD SparingUnitBytes;
    BOOLEAN SoftwareSparing;
    DWORD TotalSpareBlocks;
    DWORD FreeSpareBlocks;
} FILE_QUERY_SPARING_BUFFER, *PFILE_QUERY_SPARING_BUFFER;

//
//===================== FSCTL_QUERY_ON_DISK_VOLUME_INFO ========================
//
//  Output structure for FSCTL_QUERY_ON_DISK_VOLUME_INFO
//

typedef struct _FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {
    LARGE_INTEGER DirectoryCount;       // -1 = unknown
    LARGE_INTEGER FileCount;            // -1 = unknown
    WORD   FsFormatMajVersion;          // -1 = unknown or n/a
    WORD   FsFormatMinVersion;          // -1 = unknown or n/a
    WCHAR FsFormatName[ 12];
    LARGE_INTEGER FormatTime;
    LARGE_INTEGER LastUpdateTime;
    WCHAR CopyrightInfo[ 34];
    WCHAR AbstractInfo[ 34];
    WCHAR FormattingImplementationInfo[ 34];
    WCHAR LastModifyingImplementationInfo[ 34];
} FILE_QUERY_ON_DISK_VOL_INFO_BUFFER, *PFILE_QUERY_ON_DISK_VOL_INFO_BUFFER;

//
//===================== FSCTL_SET_REPAIR ========================
//
//  Input flags for FSCTL_SET_REPAIR
//

#define SET_REPAIR_ENABLED                                      (0x00000001)
#define SET_REPAIR_WARN_ABOUT_DATA_LOSS                         (0x00000008)
#define SET_REPAIR_DISABLED_AND_BUGCHECK_ON_CORRUPT             (0x00000010)
#define SET_REPAIR_VALID_MASK                                   (0x00000019)

//
//===================== FSCTL_INITIATE_REPAIR ========================
//
//  Optional output structure for FSCTL_INITIATE_REPAIR
//

#define FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_IN_USE                               (0x0000000000000001)
#define FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_REUSED                                   (0x0000000000000002)
#define FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_EXIST                                (0x0000000000000004)
#define FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_BASE_RECORD                          (0x0000000000000008)
#define FILE_INITIATE_REPAIR_HINT1_SYSTEM_FILE                                          (0x0000000000000010)
#define FILE_INITIATE_REPAIR_HINT1_NOT_IMPLEMENTED                                      (0x0000000000000020)
#define FILE_INITIATE_REPAIR_HINT1_UNABLE_TO_REPAIR                                     (0x0000000000000040)
#define FILE_INITIATE_REPAIR_HINT1_REPAIR_DISABLED                                      (0x0000000000000080)
#define FILE_INITIATE_REPAIR_HINT1_RECURSIVELY_CORRUPTED                                (0x0000000000000100)
#define FILE_INITIATE_REPAIR_HINT1_ORPHAN_GENERATED                                     (0x0000000000000200)
#define FILE_INITIATE_REPAIR_HINT1_REPAIRED                                             (0x0000000000000400)
#define FILE_INITIATE_REPAIR_HINT1_NOTHING_WRONG                                        (0x0000000000000800)
#define FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_NOT_FOUND                                  (0x0000000000001000)
#define FILE_INITIATE_REPAIR_HINT1_POTENTIAL_CROSSLINK                                  (0x0000000000002000)
#define FILE_INITIATE_REPAIR_HINT1_STALE_INFORMATION                                    (0x0000000000004000)
#define FILE_INITIATE_REPAIR_HINT1_CLUSTERS_ALREADY_IN_USE                              (0x0000000000008000)
#define FILE_INITIATE_REPAIR_HINT1_LCN_NOT_EXIST                                        (0x0000000000010000)
#define FILE_INITIATE_REPAIR_HINT1_INVALID_RUN_LENGTH                                   (0x0000000000020000)
#define FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_NOT_ORPHAN                               (0x0000000000040000)
#define FILE_INITIATE_REPAIR_HINT1_FILE_RECORD_IS_BASE_RECORD                           (0x0000000000080000)
#define FILE_INITIATE_REPAIR_HINT1_INVALID_ARRAY_LENGTH_COUNT                           (0x0000000000100000)
#define FILE_INITIATE_REPAIR_HINT1_SID_VALID                                            (0x0000000000200000)
#define FILE_INITIATE_REPAIR_HINT1_SID_MISMATCH                                         (0x0000000000400000)
#define FILE_INITIATE_REPAIR_HINT1_INVALID_PARENT                                       (0x0000000000800000)
#define FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_IN_USE                        (0x0000000001000000)
#define FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_REUSED                            (0x0000000002000000)
#define FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_EXIST                         (0x0000000004000000)
#define FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_BASE_RECORD                   (0x0000000008000000)
#define FILE_INITIATE_REPAIR_HINT1_PARENT_FILE_RECORD_NOT_INDEX                         (0x0000000010000000)
#define FILE_INITIATE_REPAIR_HINT1_VALID_INDEX_ENTRY                                    (0x0000000020000000)
#define FILE_INITIATE_REPAIR_HINT1_OUT_OF_GENERIC_NAMES                                 (0x0000000040000000)
#define FILE_INITIATE_REPAIR_HINT1_OUT_OF_RESOURCE                                      (0x0000000080000000)
#define FILE_INITIATE_REPAIR_HINT1_INVALID_LCN                                          (0x0000000100000000)
#define FILE_INITIATE_REPAIR_HINT1_INVALID_VCN                                          (0x0000000200000000)
#define FILE_INITIATE_REPAIR_HINT1_NAME_CONFLICT                                        (0x0000000400000000)
#define FILE_INITIATE_REPAIR_HINT1_ORPHAN                                               (0x0000000800000000)
#define FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_TOO_SMALL                                  (0x0000001000000000)
#define FILE_INITIATE_REPAIR_HINT1_ATTRIBUTE_NON_RESIDENT                               (0x0000002000000000)
#define FILE_INITIATE_REPAIR_HINT1_DENY_DEFRAG                                          (0x0000004000000000)
#define FILE_INITIATE_REPAIR_HINT1_PREVIOUS_PARENT_STILL_VALID                          (0x0000008000000000)
#define FILE_INITIATE_REPAIR_HINT1_INDEX_ENTRY_MISMATCH                                 (0x0000010000000000)
#define FILE_INITIATE_REPAIR_HINT1_INVALID_ORPHAN_RECOVERY_NAME                         (0x0000020000000000)
#define FILE_INITIATE_REPAIR_HINT1_MULTIPLE_FILE_NAME_ATTRIBUTES                        (0x0000040000000000)

//
//  We need to expose CLSN definition???????????????
//
typedef DWORDLONG CLSN;

typedef struct _FILE_INITIATE_REPAIR_OUTPUT_BUFFER {
    DWORDLONG Hint1;
    DWORDLONG Hint2;
    CLSN Clsn;
    DWORD    Status;
} FILE_INITIATE_REPAIR_OUTPUT_BUFFER, *PFILE_INITIATE_REPAIR_OUTPUT_BUFFER;

//
//===================== FSCTL_SHRINK_VOLUME ========================
//
//  Input structures for FSCTL_SHRINK_VOLUME.
//

typedef enum _SHRINK_VOLUME_REQUEST_TYPES
{
    ShrinkPrepare = 1,
    ShrinkCommit,
    ShrinkAbort

} SHRINK_VOLUME_REQUEST_TYPES, *PSHRINK_VOLUME_REQUEST_TYPES;

typedef struct _SHRINK_VOLUME_INFORMATION
{
    SHRINK_VOLUME_REQUEST_TYPES ShrinkRequestType;
    DWORDLONG Flags;
    LONGLONG NewNumberOfSectors;

} SHRINK_VOLUME_INFORMATION, *PSHRINK_VOLUME_INFORMATION;

//
//========= FSCTL_TXFS_MODIFY_RM,  FSCTL_TXFS_QUERY_RM_INFORMATION ============
//
//  Structures for FSCTL_TXFS_MODIFY_RM and FSCTL_TXFS_QUERY_RM_INFORMATION
//
//  For ModifyRM, TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS and
//  TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT are mutually exclusive.
//  You can specify the log growth amount in number of containers or as a percentage.
//
//  For ModifyRM, TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MAX and
//  TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX are mutually exclusive.
//
//  For ModifyRM, TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MIN and
//  TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN are mutually exclusive.
//
//  For ModifyRM, TXFS_RM_FLAG_RESET_RM_AT_NEXT_START and
//  TXFS_RM_FLAG_DO_NOT_RESET_RM_AT_NEXT_START are mutually exclusive and only
//  apply to default RMs.
//
//  For ModifyRM, TXFS_RM_FLAG_PREFER_CONSISTENCY and
//  TXFS_RM_FLAG_PREFER_AVAILABILITY are mutually exclusive.  After calling ModifyRM
//  with one of these flags set the RM must be restarted for the change to take effect.
//

#define TXFS_RM_FLAG_LOGGING_MODE                           0x00000001
#define TXFS_RM_FLAG_RENAME_RM                              0x00000002
#define TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MAX                0x00000004
#define TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MIN                0x00000008
#define TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS    0x00000010
#define TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT           0x00000020
#define TXFS_RM_FLAG_LOG_AUTO_SHRINK_PERCENTAGE             0x00000040
#define TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX             0x00000080
#define TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN             0x00000100
#define TXFS_RM_FLAG_GROW_LOG                               0x00000400
#define TXFS_RM_FLAG_SHRINK_LOG                             0x00000800
#define TXFS_RM_FLAG_ENFORCE_MINIMUM_SIZE                   0x00001000
#define TXFS_RM_FLAG_PRESERVE_CHANGES                       0x00002000
#define TXFS_RM_FLAG_RESET_RM_AT_NEXT_START                 0x00004000
#define TXFS_RM_FLAG_DO_NOT_RESET_RM_AT_NEXT_START          0x00008000
#define TXFS_RM_FLAG_PREFER_CONSISTENCY                     0x00010000
#define TXFS_RM_FLAG_PREFER_AVAILABILITY                    0x00020000

#define TXFS_LOGGING_MODE_SIMPLE        (0x0001)
#define TXFS_LOGGING_MODE_FULL          (0x0002)

#define TXFS_TRANSACTION_STATE_NONE         0x00
#define TXFS_TRANSACTION_STATE_ACTIVE       0x01
#define TXFS_TRANSACTION_STATE_PREPARED     0x02
#define TXFS_TRANSACTION_STATE_NOTACTIVE    0x03

#define TXFS_MODIFY_RM_VALID_FLAGS                                      \
                (TXFS_RM_FLAG_LOGGING_MODE                          |   \
                 TXFS_RM_FLAG_RENAME_RM                             |   \
                 TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MAX               |   \
                 TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MIN               |   \
                 TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS   |   \
                 TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT          |   \
                 TXFS_RM_FLAG_LOG_AUTO_SHRINK_PERCENTAGE            |   \
                 TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX            |   \
                 TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN            |   \
                 TXFS_RM_FLAG_SHRINK_LOG                            |   \
                 TXFS_RM_FLAG_GROW_LOG                              |   \
                 TXFS_RM_FLAG_ENFORCE_MINIMUM_SIZE                  |   \
                 TXFS_RM_FLAG_PRESERVE_CHANGES                      |   \
                 TXFS_RM_FLAG_RESET_RM_AT_NEXT_START                |   \
                 TXFS_RM_FLAG_DO_NOT_RESET_RM_AT_NEXT_START         |   \
                 TXFS_RM_FLAG_PREFER_CONSISTENCY                    |   \
                 TXFS_RM_FLAG_PREFER_AVAILABILITY)

typedef struct _TXFS_MODIFY_RM {

    //
    //  TXFS_RM_FLAG_* flags
    //

    DWORD Flags;

    //
    //  Maximum log container count if TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MAX is set.
    //

    DWORD LogContainerCountMax;

    //
    //  Minimum log container count if TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MIN is set.
    //

    DWORD LogContainerCountMin;

    //
    //  Target log container count for TXFS_RM_FLAG_SHRINK_LOG or _GROW_LOG.
    //

    DWORD LogContainerCount;

    //
    //  When the log is full, increase its size by this much.  Indicated as either a percent of
    //  the log size or absolute container count, depending on which of the TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_*
    //  flags is set.
    //

    DWORD LogGrowthIncrement;

    //
    //  Sets autoshrink policy if TXFS_RM_FLAG_LOG_AUTO_SHRINK_PERCENTAGE is set.  Autoshrink
    //  makes the log shrink so that no more than this percentage of the log is free at any time.
    //

    DWORD LogAutoShrinkPercentage;

    //
    //  Reserved.
    //

    DWORDLONG Reserved;

    //
    //  If TXFS_RM_FLAG_LOGGING_MODE is set, this must contain one of TXFS_LOGGING_MODE_SIMPLE
    //  or TXFS_LOGGING_MODE_FULL.
    //

    WORD   LoggingMode;

} TXFS_MODIFY_RM,
 *PTXFS_MODIFY_RM;

#define TXFS_RM_STATE_NOT_STARTED       0
#define TXFS_RM_STATE_STARTING          1
#define TXFS_RM_STATE_ACTIVE            2
#define TXFS_RM_STATE_SHUTTING_DOWN     3

//
//  The flags field for query RM information is used for the following information:
//
//  1)  To indicate whether the LogGrowthIncrement field is reported as a percent
//      or as a number of containers.  Possible flag values for this are:
//
//      TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS xor TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT
//
//  2)  To indicate that there is no set maximum or minimum container count.  Possible
//      flag values for this are:
//
//      TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX
//      TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN
//
//      Note that these flags are not mutually exclusive.
//
//  2)  To report whether the RM will be reset the next time it is started.  Note that
//      only the default RM will report a meaningful value (secondary RMs will always
//      report DO_NOT_RESET) Possible flag values for this are:
//
//      TXFS_RM_FLAG_RESET_RM_AT_NEXT_START xor TXFS_RM_FLAG_DO_NOT_RESET_RM_AT_NEXT_START
//
//  3)  To report whether the RM is in consistency mode or availability mode.  Possible
//      flag values for this are:
//
//      TXFS_RM_FLAG_PREFER_CONSISTENCY xor TXFS_RM_FLAG_PREFER_AVAILABILITY
//
//  The RmState field can have exactly one of the above-defined TXF_RM_STATE_ values.
//

#define TXFS_QUERY_RM_INFORMATION_VALID_FLAGS                           \
                (TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS   |   \
                 TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT          |   \
                 TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX            |   \
                 TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN            |   \
                 TXFS_RM_FLAG_RESET_RM_AT_NEXT_START                |   \
                 TXFS_RM_FLAG_DO_NOT_RESET_RM_AT_NEXT_START         |   \
                 TXFS_RM_FLAG_PREFER_CONSISTENCY                    |   \
                 TXFS_RM_FLAG_PREFER_AVAILABILITY)

typedef struct _TXFS_QUERY_RM_INFORMATION {

    //
    //  If the return value is STATUS_BUFFER_OVERFLOW (ERROR_MORE_DATA), this
    //  will indicate how much space is required to hold everything.
    //

    DWORD BytesRequired;

    //
    //  LSN of earliest available record in the RM's log.
    //

    DWORDLONG TailLsn;

    //
    //  LSN of most recently-written record in the RM's log.
    //

    DWORDLONG CurrentLsn;

    //
    //  LSN of the log's archive tail.
    //

    DWORDLONG ArchiveTailLsn;

    //
    //  Size of a log container in bytes.
    //

    DWORDLONG LogContainerSize;

    //
    //  Highest virtual clock value recorded in this RM's log.
    //

    LARGE_INTEGER HighestVirtualClock;

    //
    //  Number of containers in this RM's log.
    //

    DWORD LogContainerCount;

    //
    //  Maximum-allowed log container count.
    //

    DWORD LogContainerCountMax;

    //
    //  Minimum-allowed log container count.
    //

    DWORD LogContainerCountMin;

    //
    //  Amount by which log will grow when it gets full.  Indicated as either a percent of
    //  the log size or absolute container count, depending on which of the TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_*
    //  flags is set.
    //

    DWORD LogGrowthIncrement;

    //
    //  Reports on the autoshrink policy if.  Autoshrink makes the log shrink so that no more than this
    //  percentage of the log is free at any time.  A value of 0 indicates that autoshrink is off (i.e.
    //  the log will not automatically shrink).
    //

    DWORD LogAutoShrinkPercentage;

    //
    //  TXFS_RM_FLAG_* flags.  See the comment above at TXFS_QUERY_RM_INFORMATION_VALID_FLAGS to see
    //  what the flags here mean.
    //

    DWORD Flags;

    //
    //  Exactly one of TXFS_LOGGING_MODE_SIMPLE or TXFS_LOGGING_MODE_FULL.
    //

    WORD   LoggingMode;

    //
    //  Reserved.
    //

    WORD   Reserved;

    //
    //  Activity state of the RM.  May be exactly one of the above-defined TXF_RM_STATE_ values.
    //

    DWORD RmState;

    //
    //  Total capacity of the log in bytes.
    //

    DWORDLONG LogCapacity;

    //
    //  Amount of free space in the log in bytes.
    //

    DWORDLONG LogFree;

    //
    //  Size of $Tops in bytes.
    //

    DWORDLONG TopsSize;

    //
    //  Amount of space in $Tops in use.
    //

    DWORDLONG TopsUsed;

    //
    //  Number of transactions active in the RM at the time of the call.
    //

    DWORDLONG TransactionCount;

    //
    //  Total number of single-phase commits that have happened the RM.
    //

    DWORDLONG OnePCCount;

    //
    //  Total number of two-phase commits that have happened the RM.
    //

    DWORDLONG TwoPCCount;

    //
    //  Number of times the log has filled up.
    //

    DWORDLONG NumberLogFileFull;

    //
    //  Age of oldest active transaction in the RM, in milliseconds.
    //

    DWORDLONG OldestTransactionAge;

    //
    //  Name of the RM.
    //

    GUID RMName;

    //
    //  Offset in bytes from the beginning of this structure to a NULL-terminated Unicode
    //  string indicating the path to the RM's transaction manager's log.
    //

    DWORD TmLogPathOffset;

} TXFS_QUERY_RM_INFORMATION,
 *PTXFS_QUERY_RM_INFORMATION;

//
//======================== FSCTL_TXFS_ROLLFORWARD_REDO ========================
//
// Structures for FSCTL_TXFS_ROLLFORWARD_REDO
//

#define TXFS_ROLLFORWARD_REDO_FLAG_USE_LAST_REDO_LSN        0x01
#define TXFS_ROLLFORWARD_REDO_FLAG_USE_LAST_VIRTUAL_CLOCK   0x02

#define TXFS_ROLLFORWARD_REDO_VALID_FLAGS                               \
                (TXFS_ROLLFORWARD_REDO_FLAG_USE_LAST_REDO_LSN |         \
                 TXFS_ROLLFORWARD_REDO_FLAG_USE_LAST_VIRTUAL_CLOCK)

typedef struct _TXFS_ROLLFORWARD_REDO_INFORMATION {
    LARGE_INTEGER  LastVirtualClock;
    DWORDLONG LastRedoLsn;
    DWORDLONG HighestRecoveryLsn;
    DWORD Flags;
} TXFS_ROLLFORWARD_REDO_INFORMATION,
 *PTXFS_ROLLFORWARD_REDO_INFORMATION;

#ifdef DEPRECATE_SUPPORTED
#ifndef USE_TXF_DEPRECATED_FUNCTIONALITY
#pragma deprecated(TXFS_ROLLFORWARD_REDO_INFORMATION)
#pragma deprecated(PTXFS_ROLLFORWARD_REDO_INFORMATION)
#endif
#endif

//
//======================== FSCTL_TXFS_START_RM ========================
//
//  Structures for FSCTL_TXFS_START_RM
//
//  Note that TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS and
//  TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT are mutually exclusive.
//  You can specify the log growth amount in number of containers or as a percentage.
//
//  TXFS_START_RM_FLAG_CONTAINER_COUNT_MAX and TXFS_START_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX
//  are mutually exclusive.
//
//  TXFS_START_RM_FLAG_LOG_CONTAINER_COUNT_MIN and TXFS_START_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN
//  are mutually exclusive.
//
//  TXFS_START_RM_FLAG_PREFER_CONSISTENCY and TXFS_START_RM_FLAG_PREFER_AVAILABILITY
//  are mutually exclusive.
//
//  Optional parameters will have system-supplied defaults applied if omitted.
//

#define TXFS_START_RM_FLAG_LOG_CONTAINER_COUNT_MAX              0x00000001
#define TXFS_START_RM_FLAG_LOG_CONTAINER_COUNT_MIN              0x00000002
#define TXFS_START_RM_FLAG_LOG_CONTAINER_SIZE                   0x00000004
#define TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS  0x00000008
#define TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT         0x00000010
#define TXFS_START_RM_FLAG_LOG_AUTO_SHRINK_PERCENTAGE           0x00000020
#define TXFS_START_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX           0x00000040
#define TXFS_START_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN           0x00000080

#define TXFS_START_RM_FLAG_RECOVER_BEST_EFFORT                  0x00000200
#define TXFS_START_RM_FLAG_LOGGING_MODE                         0x00000400
#define TXFS_START_RM_FLAG_PRESERVE_CHANGES                     0x00000800

#define TXFS_START_RM_FLAG_PREFER_CONSISTENCY                   0x00001000
#define TXFS_START_RM_FLAG_PREFER_AVAILABILITY                  0x00002000

#define TXFS_START_RM_VALID_FLAGS                                           \
                (TXFS_START_RM_FLAG_LOG_CONTAINER_COUNT_MAX             |   \
                 TXFS_START_RM_FLAG_LOG_CONTAINER_COUNT_MIN             |   \
                 TXFS_START_RM_FLAG_LOG_CONTAINER_SIZE                  |   \
                 TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS |   \
                 TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT        |   \
                 TXFS_START_RM_FLAG_LOG_AUTO_SHRINK_PERCENTAGE          |   \
                 TXFS_START_RM_FLAG_RECOVER_BEST_EFFORT                 |   \
                 TXFS_START_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX          |   \
                 TXFS_START_RM_FLAG_LOGGING_MODE                        |   \
                 TXFS_START_RM_FLAG_PRESERVE_CHANGES                    |   \
                 TXFS_START_RM_FLAG_PREFER_CONSISTENCY                  |   \
                 TXFS_START_RM_FLAG_PREFER_AVAILABILITY)

typedef struct _TXFS_START_RM_INFORMATION {

    //
    //  TXFS_START_RM_FLAG_* flags.
    //

    DWORD Flags;

    //
    //  RM log container size, in bytes.  This parameter is optional.
    //

    DWORDLONG LogContainerSize;

    //
    //  RM minimum log container count.  This parameter is optional.
    //

    DWORD LogContainerCountMin;

    //
    //  RM maximum log container count.  This parameter is optional.
    //

    DWORD LogContainerCountMax;

    //
    //  RM log growth increment in number of containers or percent, as indicated
    //  by TXFS_START_RM_FLAG_LOG_GROWTH_INCREMENT_* flag.  This parameter is
    //  optional.
    //

    DWORD LogGrowthIncrement;

    //
    //  RM log auto shrink percentage.  This parameter is optional.
    //

    DWORD LogAutoShrinkPercentage;

    //
    //  Offset from the beginning of this structure to the log path for the KTM
    //  instance to be used by this RM.  This must be a two-byte (WCHAR) aligned
    //  value.  This parameter is required.
    //

    DWORD TmLogPathOffset;

    //
    //  Length in bytes of log path for the KTM instance to be used by this RM.
    //  This parameter is required.
    //

    WORD   TmLogPathLength;

    //
    //  Logging mode for this RM.  One of TXFS_LOGGING_MODE_SIMPLE or
    //  TXFS_LOGGING_MODE_FULL (mutually exclusive).  This parameter is optional,
    //  and will default to TXFS_LOGGING_MODE_SIMPLE.
    //

    WORD   LoggingMode;

    //
    //  Length in bytes of the path to the log to be used by the RM.  This parameter
    //  is required.
    //

    WORD   LogPathLength;

    //
    //  Reserved.
    //

    WORD   Reserved;

    //
    //  The path to the log (in Unicode characters) to be used by the RM goes here.
    //  This parameter is required.
    //

    WCHAR LogPath[1];

} TXFS_START_RM_INFORMATION,
 *PTXFS_START_RM_INFORMATION;

#ifdef DEPRECATE_SUPPORTED
#ifndef USE_TXF_DEPRECATED_FUNCTIONALITY
#pragma deprecated(TXFS_START_RM_INFORMATION)
#pragma deprecated(PTXFS_START_RM_INFORMATION)
#endif
#endif

//
//======================== FSCTL_TXFS_GET_METADATA_INFO ========================
//
//  Structures for FSCTL_TXFS_GET_METADATA_INFO
//

typedef struct _TXFS_GET_METADATA_INFO_OUT {

    //
    //  Returns the TxfId of the file referenced by the handle used to call this routine.
    //

    struct {
        LONGLONG LowPart;
        LONGLONG HighPart;
    } TxfFileId;

    //
    //  The GUID of the transaction that has the file locked, if applicable.
    //

    GUID LockingTransaction;

    //
    //  Returns the LSN for the most recent log record we've written for the file.
    //

    DWORDLONG LastLsn;

    //
    //  Transaction state, a TXFS_TRANSACTION_STATE_* value.
    //

    DWORD TransactionState;

} TXFS_GET_METADATA_INFO_OUT, *PTXFS_GET_METADATA_INFO_OUT;

//
//================= FSCTL_TXFS_LIST_TRANSACTION_LOCKED_FILES ==================
//
//  Structures for FSCTL_TXFS_LIST_TRANSACTION_LOCKED_FILES
//
//  TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY_FLAG_CREATED means the reported name was created
//  in the locking transaction.
//
//  TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY_FLAG_DELETED means the reported name was deleted
//  in the locking transaction.
//
//  Note that both flags may appear if the name was both created and deleted in the same
//  transaction.  In that case the FileName[] member will contain only "\0", as there is
//  no meaningful name to report.
//

#define TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY_FLAG_CREATED   0x00000001
#define TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY_FLAG_DELETED   0x00000002

typedef struct _TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {

    //
    //  Offset in bytes from the beginning of the TXFS_LIST_TRANSACTION_LOCKED_FILES
    //  structure to the next TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY.
    //

    DWORDLONG Offset;

    //
    //  TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY_FLAG_* flags to indicate whether the
    //  current name was deleted or created in the transaction.
    //

    DWORD NameFlags;

    //
    //  NTFS File ID of the file.
    //

    LONGLONG FileId;

    //
    //  Reserved.
    //

    DWORD Reserved1;
    DWORD Reserved2;
    LONGLONG Reserved3;

    //
    //  NULL-terminated Unicode path to this file, relative to RM root.
    //

    WCHAR FileName[1];
} TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY, *PTXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY;

typedef struct _TXFS_LIST_TRANSACTION_LOCKED_FILES {

    //
    //  GUID name of the KTM transaction that files should be enumerated from.
    //

    GUID KtmTransaction;

    //
    //  On output, the number of files involved in the transaction on this RM.
    //

    DWORDLONG NumberOfFiles;

    //
    //  The length of the buffer required to obtain the complete list of files.
    //  This value may change from call to call as the transaction locks more files.
    //

    DWORDLONG BufferSizeRequired;

    //
    //  Offset in bytes from the beginning of this structure to the first
    //  TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY.
    //

    DWORDLONG Offset;
} TXFS_LIST_TRANSACTION_LOCKED_FILES, *PTXFS_LIST_TRANSACTION_LOCKED_FILES;

//
//==================== FSCTL_TXFS_LIST_TRANSACTIONS ======================
//
//  Structures for FSCTL_TXFS_LIST_TRANSACTIONS
//

typedef struct _TXFS_LIST_TRANSACTIONS_ENTRY {

    //
    //  Transaction GUID.
    //

    GUID TransactionId;

    //
    //  Transaction state, a TXFS_TRANSACTION_STATE_* value.
    //

    DWORD TransactionState;

    //
    //  Reserved fields
    //

    DWORD Reserved1;
    DWORD Reserved2;
    LONGLONG Reserved3;
} TXFS_LIST_TRANSACTIONS_ENTRY, *PTXFS_LIST_TRANSACTIONS_ENTRY;

typedef struct _TXFS_LIST_TRANSACTIONS {

    //
    //  On output, the number of transactions involved in this RM.
    //

    DWORDLONG NumberOfTransactions;

    //
    //  The length of the buffer required to obtain the complete list of
    //  transactions.  Note that this value may change from call to call
    //  as transactions enter and exit the system.
    //

    DWORDLONG BufferSizeRequired;
} TXFS_LIST_TRANSACTIONS, *PTXFS_LIST_TRANSACTIONS;

//
//==================== FSCTL_TXFS_READ_BACKUP_INFORMATION ======================
//
//  Structures for FSCTL_TXFS_READ_BACKUP_INFORMATION
//

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201)       // unnamed struct

typedef struct _TXFS_READ_BACKUP_INFORMATION_OUT {
    union {

        //
        //  Used to return the required buffer size if return code is STATUS_BUFFER_OVERFLOW
        //

        DWORD BufferLength;

        //
        //  On success the data is copied here.
        //

        BYTE  Buffer[1];
    } DUMMYUNIONNAME;
} TXFS_READ_BACKUP_INFORMATION_OUT, *PTXFS_READ_BACKUP_INFORMATION_OUT;

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning( default : 4201 )
#endif

//
//==================== FSCTL_TXFS_WRITE_BACKUP_INFORMATION ======================
//
//  Structures for FSCTL_TXFS_WRITE_BACKUP_INFORMATION
//

typedef struct _TXFS_WRITE_BACKUP_INFORMATION {

    //
    //  The data returned in the Buffer member of a previous call to
    //  FSCTL_TXFS_READ_BACKUP_INFORMATION goes here.
    //

    BYTE  Buffer[1];
} TXFS_WRITE_BACKUP_INFORMATION, *PTXFS_WRITE_BACKUP_INFORMATION;

//
//==================== FSCTL_TXFS_GET_TRANSACTED_VERSION ======================
//
//  Output structure for FSCTL_TXFS_GET_TRANSACTED_VERSION
//

#define TXFS_TRANSACTED_VERSION_NONTRANSACTED   0xFFFFFFFE
#define TXFS_TRANSACTED_VERSION_UNCOMMITTED     0xFFFFFFFF

typedef struct _TXFS_GET_TRANSACTED_VERSION {

    //
    //  The version that this handle is opened to.  This will be
    //  TXFS_TRANSACTED_VERSION_UNCOMMITTED for nontransacted and
    //  transactional writer handles.
    //

    DWORD ThisBaseVersion;

    //
    //  The most recent committed version available.
    //

    DWORD LatestVersion;

    //
    //  If this is a handle to a miniversion, the ID of the miniversion.
    //  If it is not a handle to a miniversion, this field will be 0.
    //

    WORD   ThisMiniVersion;

    //
    //  The first available miniversion.  Unless the miniversions are
    //  visible to the transaction bound to this handle, this field will be zero.
    //

    WORD   FirstMiniVersion;

    //
    //  The latest available miniversion.  Unless the miniversions are
    //  visible to the transaction bound to this handle, this field will be zero.
    //

    WORD   LatestMiniVersion;

} TXFS_GET_TRANSACTED_VERSION, *PTXFS_GET_TRANSACTED_VERSION;

//
//==================== FSCTL_TXFS_SAVEPOINT_INFORMATION ======================
//
//  Structures for FSCTL_TXFS_SAVEPOINT_INFORMATION
//
//  Note that the TXFS_SAVEPOINT_INFORMATION structure is both and in and out structure.
//  The KtmTransaction and ActionCode members are always in-parameters, and the SavepointId
//  member is either an in-parameter, an out-parameter, or not used (see its definition below).
//

//
//  Create a new savepoint.
//

#define TXFS_SAVEPOINT_SET                      0x00000001

//
//  Roll back to a specified savepoint.
//

#define TXFS_SAVEPOINT_ROLLBACK                 0x00000002

//
//  Clear (make unavailable for rollback) the most recently set savepoint
//  that has not yet been cleared.
//

#define TXFS_SAVEPOINT_CLEAR                    0x00000004

//
//  Clear all savepoints from the transaction.
//

#define TXFS_SAVEPOINT_CLEAR_ALL                0x00000010

typedef struct _TXFS_SAVEPOINT_INFORMATION {

    //
    //  Handle to the transaction on which to perform the savepoint operation.
    //

    HANDLE KtmTransaction;

    //
    //  Specifies the savepoint action to take.  A TXFS_SAVEPOINT_* value.
    //

    DWORD ActionCode;

    //
    //  In-parameter for TXFS_ROLLBACK_TO_SAVEPOINT - specifies the savepoint to which
    //  to roll back.
    //
    //  Out-parameter for TXFS_SET_SAVEPOINT - the newly-created savepoint ID will be
    //  returned here.
    //
    //  Not used for TXFS_CLEAR_SAVEPOINT or TXFS_CLEAR_ALL_SAVEPOINTS.
    //

    DWORD SavepointId;

} TXFS_SAVEPOINT_INFORMATION, *PTXFS_SAVEPOINT_INFORMATION;

#ifdef DEPRECATE_SUPPORTED
#ifndef USE_TXF_DEPRECATED_FUNCTIONALITY
#pragma deprecated(TXFS_SAVEPOINT_INFORMATION)
#pragma deprecated(PTXFS_SAVEPOINT_INFORMATION)
#endif
#endif

//
//==================== FSCTL_TXFS_CREATE_MINIVERSION ======================
//
//  Structures for FSCTL_TXFS_CREATE_MINIVERSION
//
//      Only an out parameter is necessary.  That returns the identifier of the new miniversion created.
//

typedef struct _TXFS_CREATE_MINIVERSION_INFO {

    WORD   StructureVersion;

    WORD   StructureLength;

    //
    //  The base version for the newly created miniversion.
    //

    DWORD BaseVersion;

    //
    //  The miniversion that was just created.
    //

    WORD   MiniVersion;

} TXFS_CREATE_MINIVERSION_INFO, *PTXFS_CREATE_MINIVERSION_INFO;

#ifdef DEPRECATE_SUPPORTED
#ifndef USE_TXF_DEPRECATED_FUNCTIONALITY
#pragma deprecated(TXFS_CREATE_MINIVERSION_INFO)
#pragma deprecated(PTXFS_CREATE_MINIVERSION_INFO)
#endif
#endif

//
//==================== FSCTL_TXFS_TRANSACTION_ACTIVE ======================
//
//  Structure for FSCTL_TXFS_TRANSACTION_ACTIVE
//

typedef struct _TXFS_TRANSACTION_ACTIVE_INFO {

    //
    //  Whether or not the volume had active transactions when this snapshot was taken.
    //

    BOOLEAN TransactionsActiveAtSnapshot;

} TXFS_TRANSACTION_ACTIVE_INFO, *PTXFS_TRANSACTION_ACTIVE_INFO;

#endif /* _WIN32_WINNT >= _WIN32_WINNT_VISTA */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)

//
//======================= FSCTL_GET_BOOT_AREA_INFO ==========================
//
// Output structure for FSCTL_GET_BOOT_AREA_INFO
//

typedef struct _BOOT_AREA_INFO {

    DWORD               BootSectorCount;  // the count of boot sectors present on the file system
    struct {
        LARGE_INTEGER   Offset;
    } BootSectors[2];                     // variable number of boot sectors.

} BOOT_AREA_INFO, *PBOOT_AREA_INFO;

//
//==================== FSCTL_GET_RETRIEVAL_POINTER_BASE ======================
//
// Output structure for FSCTL_GET_RETRIEVAL_POINTER_BASE
//

typedef struct _RETRIEVAL_POINTER_BASE {

    LARGE_INTEGER       FileAreaOffset; // sector offset to the first allocatable unit on the filesystem
} RETRIEVAL_POINTER_BASE, *PRETRIEVAL_POINTER_BASE;

//
//==================== FSCTL_QUERY_PERSISTENT_VOLUME_STATE ====================
//==================== FSCTL_SET_PERSISTENT_VOLUME_STATE ======================
//
// Structure for FSCTL_SET_PERSISTENT_VOLUME_STATE and
//  FSCTL_GET_PERSISTENT_VOLUME_STATE.  The initial version will be 1.0
//

typedef struct _FILE_FS_PERSISTENT_VOLUME_INFORMATION {

    DWORD VolumeFlags;
    DWORD FlagMask;
    DWORD Version;
    DWORD Reserved;

} FILE_FS_PERSISTENT_VOLUME_INFORMATION, *PFILE_FS_PERSISTENT_VOLUME_INFORMATION;

//
//  VolumeFlags values
//

#define PERSISTENT_VOLUME_STATE_SHORT_NAME_CREATION_DISABLED        (0x00000001)

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

#define PERSISTENT_VOLUME_STATE_VOLUME_SCRUB_DISABLED               (0x00000002)

#endif  /* (_WIN32_WINNT >= _WIN32_WINNT_WIN8) */

#if (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)
//
//  Persistent volume flags to control the file system's storage tiering
//  awareness.
//

#define PERSISTENT_VOLUME_STATE_GLOBAL_METADATA_NO_SEEK_PENALTY     (0x00000004)
#define PERSISTENT_VOLUME_STATE_LOCAL_METADATA_NO_SEEK_PENALTY      (0x00000008)
#define PERSISTENT_VOLUME_STATE_NO_HEAT_GATHERING                   (0x00000010)
#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE) */

//
//  These are flags that define a volume's dependency on WimBoot file.
//

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)

//
//  The volume backs a system critical volume, typically the one that has wimboot
//  image file serving the system files
//

#define PERSISTENT_VOLUME_STATE_CONTAINS_BACKING_WIM                (0x00000020)

//
//  The volume is backed by other volume that actually has the system files.
//  And hence this relies on the other volume being present in order for the system to boot up.
//

#define PERSISTENT_VOLUME_STATE_BACKED_BY_WIM                       (0x00000040)

#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WIN7) */

#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)

//
//  Writes dynamic redirection on tiered volumes will be disabled
//

#define PERSISTENT_VOLUME_STATE_NO_WRITE_AUTO_TIERING               (0x00000080)

#endif // #if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)

#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)

//
//  Disable txf on volume
//

#define PERSISTENT_VOLUME_STATE_TXF_DISABLED                        (0x00000100)

#endif // #if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)

//
//  Always reallocate data writes
//

#define PERSISTENT_VOLUME_STATE_REALLOCATE_ALL_DATA_WRITES          (0x00000200)

#endif // #if (NTDDI_VERSION >= NTDDI_WIN10_RS5)

#if (NTDDI_VERSION >= NTDDI_WIN10_MN)

//
//  This indicates that AutoChk modified this volume and is cleared by AutoChk
//  after it ensured that the volume is dismounted or the system is rebooted.
//  This can be set or queried.
//

#define PERSISTENT_VOLUME_STATE_CHKDSK_RAN_ONCE                     (0x00000400)

//
//  This again indicates that AutoChk modified this volume but is cleared by
//  NTFS on next mount.  So if this flag is set it means that the volume was
//  modified by AutoChk while it's still mounted and the on-disk state and
//  the in memory state could be different.  This can only be queried.
//

#define PERSISTENT_VOLUME_STATE_MODIFIED_BY_CHKDSK                  (0x00000800)

//
//  The volume was formatted as DAX.  The volume may not be mounted as DAX
//  if the storage is not DAX capable.  This can only be queried.
//

#define PERSISTENT_VOLUME_STATE_DAX_FORMATTED                       (0x00001000)

#endif // #if (NTDDI_VERSION >= NTDDI_WIN10_MN)

#if (NTDDI_VERSION >= NTDDI_WIN11_ZN)

//
//  The volume was formatted as a developer volume.  This can be used by the
//  file system and other system components to enable optimizations that doe
//  not require an administrator to trust the volume on a given machine.
//
//  This is set at format time and can only be queried.
//

#define PERSISTENT_VOLUME_STATE_DEV_VOLUME                          (0x00002000)

//
//  An adminstrator on a given machine had trust this volume and is willing
//  to enable optimizations like not having anti-virus filters attach to the
//  volume.  This information is persisted in the registry on a given machine.
//  This can be used by the file system and other system components to enable
//  optimizations that require an administrator to trust the volume on a given
//  machine.
//
//  NOTE: Today only developer volumes can be trusted, i.e. this flag can be
//  set only when PERSISTENT_VOLUME_STATE_DEV_VOLUME is set.
//
//  This can be set and queried.
//

#define PERSISTENT_VOLUME_STATE_TRUSTED_VOLUME                      (0x00004000)

#endif // #if (NTDDI_VERSION >= NTDDI_WIN11_ZN)

//
//==================== FSCTL_QUERY_FILE_SYSTEM_RECOGNITION ====================
//
//  Structure for FSCTL_QUERY_FILE_SYSTEM_RECOGNITION
//

typedef struct _FILE_SYSTEM_RECOGNITION_INFORMATION {

    CHAR FileSystem[9];

} FILE_SYSTEM_RECOGNITION_INFORMATION, *PFILE_SYSTEM_RECOGNITION_INFORMATION;

//
//=========================== FSCTL_REQUEST_OPLOCK ===========================
//
//  Structures for FSCTL_REQUEST_OPLOCK
//

#define OPLOCK_LEVEL_CACHE_READ         (0x00000001)
#define OPLOCK_LEVEL_CACHE_HANDLE       (0x00000002)
#define OPLOCK_LEVEL_CACHE_WRITE        (0x00000004)

#define REQUEST_OPLOCK_INPUT_FLAG_REQUEST               (0x00000001)
#define REQUEST_OPLOCK_INPUT_FLAG_ACK                   (0x00000002)
#define REQUEST_OPLOCK_INPUT_FLAG_COMPLETE_ACK_ON_CLOSE (0x00000004)

#define REQUEST_OPLOCK_CURRENT_VERSION          1

typedef struct _REQUEST_OPLOCK_INPUT_BUFFER {

    //
    //  This should be set to REQUEST_OPLOCK_CURRENT_VERSION.
    //

    WORD   StructureVersion;

    WORD   StructureLength;

    //
    //  One or more OPLOCK_LEVEL_CACHE_* values to indicate the desired level of the oplock.
    //

    DWORD RequestedOplockLevel;

    //
    //  REQUEST_OPLOCK_INPUT_FLAG_* flags.
    //

    DWORD Flags;

} REQUEST_OPLOCK_INPUT_BUFFER, *PREQUEST_OPLOCK_INPUT_BUFFER;

#define REQUEST_OPLOCK_OUTPUT_FLAG_ACK_REQUIRED                 (0x00000001)
#define REQUEST_OPLOCK_OUTPUT_FLAG_MODES_PROVIDED               (0x00000002)

#if (NTDDI_VERSION >= NTDDI_WIN10_CU)
// If the oplock request fails with STATUS_OPLOCK_NOT_GRANTED, this flag indicates that the oplock
// could not be granted due to the presence of a writable user-mapped section.
#define REQUEST_OPLOCK_OUTPUT_FLAG_WRITABLE_SECTION_PRESENT     (0x00000004)
#endif

typedef struct _REQUEST_OPLOCK_OUTPUT_BUFFER {

    //
    //  This should be set to REQUEST_OPLOCK_CURRENT_VERSION.
    //

    WORD   StructureVersion;

    WORD   StructureLength;

    //
    //  One or more OPLOCK_LEVEL_CACHE_* values indicating the level of the oplock that
    //  was just broken.
    //

    DWORD OriginalOplockLevel;

    //
    //  One or more OPLOCK_LEVEL_CACHE_* values indicating the level to which an oplock
    //  is being broken, or an oplock level that may be available for granting, depending
    //  on the operation returning this buffer.
    //

    DWORD NewOplockLevel;

    //
    //  REQUEST_OPLOCK_OUTPUT_FLAG_* flags.
    //

    DWORD Flags;

    //
    //  When REQUEST_OPLOCK_OUTPUT_FLAG_MODES_PROVIDED is set, and when the
    //  OPLOCK_LEVEL_CACHE_HANDLE level is being lost in an oplock break, these fields
    //  contain the access mode and share mode of the request that is causing the break.
    //

    ACCESS_MASK AccessMode;

    WORD   ShareMode;

} REQUEST_OPLOCK_OUTPUT_BUFFER, *PREQUEST_OPLOCK_OUTPUT_BUFFER;

//
//======================= FSCTL_QUERY_DEPENDENT_VOLUME =======================
//

#ifndef _VIRTUAL_STORAGE_TYPE_DEFINED
#define _VIRTUAL_STORAGE_TYPE_DEFINED
typedef struct _VIRTUAL_STORAGE_TYPE
{
    DWORD DeviceId;
    GUID  VendorId;
} VIRTUAL_STORAGE_TYPE, *PVIRTUAL_STORAGE_TYPE;
#endif

//
//  These structures are used by the FSCTL_QUERY_DEPENDENT_VOLUME
//

typedef struct _STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {
    DWORD   RequestLevel;
    DWORD   RequestFlags;
} STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST, *PSTORAGE_QUERY_DEPENDENT_VOLUME_REQUEST;

#define QUERY_DEPENDENT_VOLUME_REQUEST_FLAG_HOST_VOLUMES    0x1
#define QUERY_DEPENDENT_VOLUME_REQUEST_FLAG_GUEST_VOLUMES   0x2

typedef struct _STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {
    DWORD   EntryLength;
    DWORD   DependencyTypeFlags;
    DWORD   ProviderSpecificFlags;
    VIRTUAL_STORAGE_TYPE VirtualStorageType;
} STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY, *PSTORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY;

typedef struct _STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {
    DWORD   EntryLength;
    DWORD   DependencyTypeFlags;
    DWORD   ProviderSpecificFlags;
    VIRTUAL_STORAGE_TYPE VirtualStorageType;
    DWORD   AncestorLevel;      // Root parent is 0, every child level after that is incremented
    DWORD   HostVolumeNameOffset;
    DWORD   HostVolumeNameSize;
    DWORD   DependentVolumeNameOffset;
    DWORD   DependentVolumeNameSize;
    DWORD   RelativePathOffset;
    DWORD   RelativePathSize;
    DWORD   DependentDeviceNameOffset;
    DWORD   DependentDeviceNameSize;
} STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY, *PSTORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY;

#ifdef _MSC_EXTENSIONS

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4200) // zero length array
#endif

typedef struct _STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE {
    DWORD   ResponseLevel;
    DWORD   NumberEntries;
    union {
        STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY Lev1Depends[];
        STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY Lev2Depends[];
    } DUMMYUNIONNAME;
} STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE, *PSTORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE;

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif

//
//========================== FSCTL_SD_GLOBAL_CHANGE ==========================
//
//  Structures for FSCTL_SD_GLOBAL_CHANGE
//

//
//  list of operations supported
//

#define SD_GLOBAL_CHANGE_TYPE_MACHINE_SID   1
#define SD_GLOBAL_CHANGE_TYPE_QUERY_STATS   (1 << 16)
#define SD_GLOBAL_CHANGE_TYPE_ENUM_SDS      (2 << 16)

//
//  Operation specific structures for SD_GLOBAL_CHANGE_TYPE_MACHINE_SID
//

typedef struct _SD_CHANGE_MACHINE_SID_INPUT {

    //
    //  The current machine SID to change.
    //  This defines the offset from the beginning of the SD_GLOBAL_CHANGE_INPUT
    //  structure of where the CurrentMachineSID to replace begins.  This will
    //  be a SID structure.  The length defines the length of the imbedded SID
    //  structure.
    //

    WORD   CurrentMachineSIDOffset;
    WORD   CurrentMachineSIDLength;

    //
    //  The new machine SID value to set in-place of the current machine SID
    //  This defines the offset from the beginning of the SD_GLOBAL_CHANGE_INPUT
    //  structure of where the NewMachineSID to set begins.  This will
    //  be a SID structure.  The length defines the length of the imbedded SID
    //  structure.
    //

    WORD   NewMachineSIDOffset;
    WORD   NewMachineSIDLength;

} SD_CHANGE_MACHINE_SID_INPUT, *PSD_CHANGE_MACHINE_SID_INPUT;

typedef struct _SD_CHANGE_MACHINE_SID_OUTPUT {

    //
    //  How many entries were successfully changed in the $Secure stream
    //

    DWORDLONG NumSDChangedSuccess;

    //
    //  How many entries failed the update in the $Secure stream
    //

    DWORDLONG NumSDChangedFail;

    //
    //  How many entries are unused in the current security stream
    //

    DWORDLONG NumSDUnused;

    //
    //  The total number of entries processed in the $Secure stream
    //

    DWORDLONG NumSDTotal;

    //
    //  How many entries were successfully changed in the $MFT file
    //

    DWORDLONG NumMftSDChangedSuccess;

    //
    //  How many entries failed the update in the $MFT file
    //

    DWORDLONG NumMftSDChangedFail;

    //
    //  Total number of entries processed in the $MFT file
    //

    DWORDLONG NumMftSDTotal;

} SD_CHANGE_MACHINE_SID_OUTPUT, *PSD_CHANGE_MACHINE_SID_OUTPUT;

//
//  Operation specific structures for SD_GLOBAL_CHANGE_TYPE_QUERY_STATS
//

typedef struct _SD_QUERY_STATS_INPUT {

    DWORD Reserved;

} SD_QUERY_STATS_INPUT, *PSD_QUERY_STATS_INPUT;

typedef struct _SD_QUERY_STATS_OUTPUT {

    //
    //  Stream size and allocation size for the security descriptor
    //  data stream ($Secure:$SDS).
    //

    DWORDLONG SdsStreamSize;
    DWORDLONG SdsAllocationSize;

    //
    //  Stream size and allocation size for the security ID index
    //  stream ($Secure:$SII).
    //

    DWORDLONG SiiStreamSize;
    DWORDLONG SiiAllocationSize;

    //
    //  Stream size and allocation size for the security descriptor
    //  hash index stream ($Secure:$SDH).
    //

    DWORDLONG SdhStreamSize;
    DWORDLONG SdhAllocationSize;

    //
    //  The total number of entries in the security descriptor data
    //  stream.
    //

    DWORDLONG NumSDTotal;

    //
    //  The number of unused entries in the security descriptor data
    //  stream.
    //

    DWORDLONG NumSDUnused;

} SD_QUERY_STATS_OUTPUT, *PSD_QUERY_STATS_OUTPUT;

//
//  Operation specific structures for SD_GLOBAL_CHANGE_TYPE_ENUM_SDS
//

typedef struct _SD_ENUM_SDS_INPUT {

    //
    //  The byte offset within the security descriptor data stream to look
    //  for security descriptors.  This must be a multiple of 16.
    //
    //  Note this is a virtual byte offset.  The security descriptor data
    //  stream contains some additional data embedded inside the stream for
    //  redundancy, and this extra data does not count toward this byte
    //  offset.
    //

    DWORDLONG StartingOffset;

    //
    //  The maximum number of SD entries to return.  This is primarily
    //  useful for supporting a model where you only want one entry at
    //  a time.  Set to 0 for no max.
    //

    DWORDLONG MaxSDEntriesToReturn;

} SD_ENUM_SDS_INPUT, *PSD_ENUM_SDS_INPUT;

typedef struct _SD_ENUM_SDS_ENTRY {

    //
    //  Hash value of the security decriptor.
    //

    DWORD Hash;

    //
    //  SecurityId for the security descriptor.
    //

    DWORD SecurityId;

    //
    //  Virtual byte offset of this header in the security descriptor
    //  data stream.
    //

    DWORDLONG Offset;

    //
    //  Length of this header + security descriptor that follows this
    //  header.
    //

    DWORD Length;

    //
    //  Actual security decriptor, variable length.
    //

    BYTE  Descriptor[1];

} SD_ENUM_SDS_ENTRY, *PSD_ENUM_SDS_ENTRY;

typedef struct _SD_ENUM_SDS_OUTPUT {

    //
    //  The current machine SID to change.
    //  This defines the offset from the beginning of the SD_GLOBAL_CHANGE_INPUT
    //  structure of where the CurrentMachineSID to replace begins.  This will
    //  be a SID structure.  The length defines the length of the imbedded SID
    //  structure.
    //

    DWORDLONG NextOffset;

    //
    //  The total number of entries returned.
    //

    DWORDLONG NumSDEntriesReturned;

    //
    //  The total number of bytes of entries returned.
    //

    DWORDLONG NumSDBytesReturned;

    //
    //  Variable length array of security descriptor stream entries.
    //  There should be NumSDEntriesReturned of them.  To walk this array,
    //  first point to SDEntry[0], then advance a number of bytes equal
    //  to the entry's Length rounded up to a multiple of 16.  (Each entry
    //  will be aligned to a 16-byte boundary.)
    //

    SD_ENUM_SDS_ENTRY SDEntry[1];

} SD_ENUM_SDS_OUTPUT, *PSD_ENUM_SDS_OUTPUT;

//
//  Generic INPUT & OUTPUT structures for FSCTL_SD_GLOBAL_CHANGE
//

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201)       // unnamed struct

typedef struct _SD_GLOBAL_CHANGE_INPUT
{
    //
    //  Input flags (none currently defined)
    //

    DWORD Flags;

    //
    //  Specifies which type of change we are doing and pics which member
    //  of the below union is in use.
    //

    DWORD ChangeType;

    union {

        SD_CHANGE_MACHINE_SID_INPUT SdChange;
        SD_QUERY_STATS_INPUT SdQueryStats;
        SD_ENUM_SDS_INPUT SdEnumSds;
    } DUMMYUNIONNAME;

} SD_GLOBAL_CHANGE_INPUT, *PSD_GLOBAL_CHANGE_INPUT;

typedef struct _SD_GLOBAL_CHANGE_OUTPUT
{

    //
    //  Output State Flags (none currently defined)
    //

    DWORD Flags;

    //
    //  Specifies which below union to use
    //

    DWORD ChangeType;

    union {

        SD_CHANGE_MACHINE_SID_OUTPUT SdChange;
        SD_QUERY_STATS_OUTPUT SdQueryStats;
        SD_ENUM_SDS_OUTPUT SdEnumSds;
    } DUMMYUNIONNAME;

} SD_GLOBAL_CHANGE_OUTPUT, *PSD_GLOBAL_CHANGE_OUTPUT;

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning( default : 4201 ) /* nonstandard extension used : nameless struct/union */
#endif

//
//==================== FSCTL_LOOKUP_STREAM_FROM_CLUSTER =======================
//

typedef struct _LOOKUP_STREAM_FROM_CLUSTER_INPUT {

    //
    //  Flags for the operation.  Currently no flags are defined.
    //
    DWORD         Flags;

    //
    //  Number of clusters in the following array of clusters.
    //  The input buffer must be large enough to contain this
    //  number or the operation will fail.
    //
    DWORD         NumberOfClusters;

    //
    //  An array of one or more clusters to look up.
    //
    LARGE_INTEGER Cluster[1];
} LOOKUP_STREAM_FROM_CLUSTER_INPUT, *PLOOKUP_STREAM_FROM_CLUSTER_INPUT;

typedef struct _LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {
    //
    //  Offset from the beginning of this structure to the first entry
    //  returned.  If no entries are returned, this value is zero.
    //
    DWORD         Offset;

    //
    //  Number of matches to the input criteria.  Note that more matches
    //  may be found than entries returned if the buffer is not large
    //  enough.
    //
    DWORD         NumberOfMatches;

    //
    //  Minimum size of the buffer, in bytes, which would be needed to
    //  contain all matching entries to the input criteria.
    //
    DWORD         BufferSizeRequired;
} LOOKUP_STREAM_FROM_CLUSTER_OUTPUT, *PLOOKUP_STREAM_FROM_CLUSTER_OUTPUT;

#define LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_PAGE_FILE          0x00000001
#define LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_DENY_DEFRAG_SET    0x00000002
#define LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_FS_SYSTEM_FILE     0x00000004
#define LOOKUP_STREAM_FROM_CLUSTER_ENTRY_FLAG_TXF_SYSTEM_FILE    0x00000008

#define LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_MASK          0xff000000
#define LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_DATA          0x01000000
#define LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_INDEX         0x02000000
#define LOOKUP_STREAM_FROM_CLUSTER_ENTRY_ATTRIBUTE_SYSTEM        0x03000000

typedef struct _LOOKUP_STREAM_FROM_CLUSTER_ENTRY {
    //
    //  Offset from the beginning of this structure to the next entry
    //  returned.  If there are no more entries, this value is zero.
    //
    DWORD         OffsetToNext;

    //
    //  Flags describing characteristics about this stream.
    //
    DWORD         Flags;

    //
    //  This value is reserved and is currently zero.
    //
    LARGE_INTEGER Reserved;

    //
    //  This is the cluster that this entry refers to.  It will be one
    //  of the clusters passed in the input structure.
    //
    LARGE_INTEGER Cluster;

    //
    //  A NULL-terminated Unicode string containing the path of the
    //  object relative to the root of the volume.  This string
    //  will refer to the attribute or stream represented by the
    //  cluster.
    //
    WCHAR         FileName[1];
} LOOKUP_STREAM_FROM_CLUSTER_ENTRY, *PLOOKUP_STREAM_FROM_CLUSTER_ENTRY;

//
//==================== FSCTL_FILE_TYPE_NOTIFICATION =======================
//
//  This is the structure for the FSCTL_FILE_TYPE_NOTIFICATION operation.
//  Its purpose is to notify the storage stack about the extents of certain
//  types of files.  This is only callable from kernel mode
//

typedef struct _FILE_TYPE_NOTIFICATION_INPUT {

    //
    //  Flags for this operation
    //  FILE_TYPE_NOTIFICATION_FLAG_*
    //

    DWORD Flags;

    //
    //  A count of how many FileTypeID guids are given
    //

    DWORD NumFileTypeIDs;

    //
    //  This is a unique identifier for the type of file notification occurring
    //

    GUID FileTypeID[1];

} FILE_TYPE_NOTIFICATION_INPUT, *PFILE_TYPE_NOTIFICATION_INPUT;

//
//  Flags for the given operation
//

#define FILE_TYPE_NOTIFICATION_FLAG_USAGE_BEGIN     0x00000001      //Set when adding the specified usage on the given file
#define FILE_TYPE_NOTIFICATION_FLAG_USAGE_END       0x00000002      //Set when removing the specified usage on the given file

//
//  These are the globally defined file types
//

DEFINE_GUID( FILE_TYPE_NOTIFICATION_GUID_PAGE_FILE,         0x0d0a64a1, 0x38fc, 0x4db8, 0x9f, 0xe7, 0x3f, 0x43, 0x52, 0xcd, 0x7c, 0x5c );
DEFINE_GUID( FILE_TYPE_NOTIFICATION_GUID_HIBERNATION_FILE,  0xb7624d64, 0xb9a3, 0x4cf8, 0x80, 0x11, 0x5b, 0x86, 0xc9, 0x40, 0xe7, 0xb7 );
DEFINE_GUID( FILE_TYPE_NOTIFICATION_GUID_CRASHDUMP_FILE,    0x9d453eb7, 0xd2a6, 0x4dbd, 0xa2, 0xe3, 0xfb, 0xd0, 0xed, 0x91, 0x09, 0xa9 );

//
//=========================FSCTL_CSV_MGMT_LOCK===========================
//
#define CSV_MGMTLOCK_CHECK_VOLUME_REDIRECTED 0x00000001
typedef struct _CSV_MGMT_LOCK {
    DWORD      Flags;
}CSV_MGMT_LOCK, *PCSV_MGMT_LOCK;

//
//========================= FSCTL_IS_CSV_FILE ============================
//
// Structure for FSCTL_IS_CSV_FILE
//

typedef struct _CSV_NAMESPACE_INFO {

    DWORD         Version;
    DWORD         DeviceNumber;
    LARGE_INTEGER StartingOffset;
    DWORD         SectorSize;

} CSV_NAMESPACE_INFO, *PCSV_NAMESPACE_INFO;

#define CSV_NAMESPACE_INFO_V1 (sizeof(CSV_NAMESPACE_INFO))
#define CSV_INVALID_DEVICE_NUMBER 0xFFFFFFFF

//
//========================= FSCTL_CSV_CONTROL =============================
//

typedef enum _CSV_CONTROL_OP {
    CsvControlStartRedirectFile                  = 0x02,
    CsvControlStopRedirectFile                   = 0x03,
    CsvControlQueryRedirectState                 = 0x04,
    CsvControlQueryFileRevision                  = 0x06,
    CsvControlQueryMdsPath                       = 0x08,
    CsvControlQueryFileRevisionFileId128         = 0x09,
    CsvControlQueryVolumeRedirectState           = 0x0a,
    CsvControlEnableUSNRangeModificationTracking = 0x0d,
    CsvControlMarkHandleLocalVolumeMount         = 0x0e,
    CsvControlUnmarkHandleLocalVolumeMount       = 0x0f,
    CsvControlGetCsvFsMdsPathV2                  = 0x12,
    CsvControlDisableCaching                     = 0x13,
    CsvControlEnableCaching                      = 0x14,
    CsvControlStartForceDFO                      = 0x15,
    CsvControlStopForceDFO                       = 0x16,
    CsvControlQueryMdsPathNoPause                = 0x17,
    CsvControlSetVolumeId                        = 0x18,
    CsvControlQueryVolumeId                      = 0x19,
} CSV_CONTROL_OP, *PCSV_CONTROL_OP;

typedef struct _CSV_CONTROL_PARAM {
    CSV_CONTROL_OP Operation;
    LONGLONG Unused;
} CSV_CONTROL_PARAM, *PCSV_CONTROL_PARAM;

//
// Output for the CsvControlQueryRedirectState
//
typedef struct _CSV_QUERY_REDIRECT_STATE {
    DWORD MdsNodeId;
    DWORD DsNodeId;
    BOOLEAN FileRedirected;
} CSV_QUERY_REDIRECT_STATE, *PCSV_QUERY_REDIRECT_STATE;

//
// Output for the CsvControlQueryFileRevision
// Note that revision tracking is per file, and not per
// stream so it changes every time one of the stream
// changes.
//
typedef struct _CSV_QUERY_FILE_REVISION {
    //
    // NTFS File Id
    //
    LONGLONG FileId;
    //
    // FileRevision[0] increases each time the CSV MDS
    // stack is rebuilt and CSVFLT loses its state.
    //
    // FileRevision[1] increases each time CSV MDS
    // stack purges the cached revision # for the
    // file.
    //
    // FileRevision[2] increases each time CSV MDS
    // observes that file sizes might have
    // changed or the file might have been written
    // to. It also is incremented when one of the
    // nodes performs the first direct IO on a stream
    // associated with this file after opening this stream.
    //
    // If any of the numbers are 0 the caller should
    // assume that the file was modified
    //
    LONGLONG FileRevision[3];

} CSV_QUERY_FILE_REVISION, *PCSV_QUERY_FILE_REVISION;

//
// Output for the CsvControlQueryFileRevisionFileId128
// Note that revision tracking is per file, and not per
// stream so it changes every time one of the stream
// changes.
//
typedef struct _CSV_QUERY_FILE_REVISION_FILE_ID_128 {
    //
    // File Id
    //
    FILE_ID_128 FileId;
    //
    // FileRevision[0] increases each time the CSV MDS
    // stack is rebuilt and CSVFLT loses its state.
    //
    // FileRevision[1] increases each time CSV MDS
    // stack purges the cached revision # for the
    // file.
    //
    // FileRevision[2] increases each time CSV MDS
    // observes that file sizes might have
    // changed or the file might have been written
    // to. It also is incremented when one of the
    // nodes performs the first direct IO on a stream
    // associated with this file after opening this stream.
    //
    // If any of the numbers are 0 the caller should
    // assume that the file was modified
    //
    LONGLONG FileRevision[3];

} CSV_QUERY_FILE_REVISION_FILE_ID_128, *PCSV_QUERY_FILE_REVISION_FILE_ID_128;

//
// Output for the CsvControlQueryMdsPath
// This control returns the path that is used by CSV to
// communicate to the MDS
//
typedef struct _CSV_QUERY_MDS_PATH {
    DWORD MdsNodeId;
    DWORD DsNodeId;
    DWORD PathLength;
    WCHAR Path[1];
} CSV_QUERY_MDS_PATH, *PCSV_QUERY_MDS_PATH;

typedef enum _CSVFS_DISK_CONNECTIVITY
{
    CsvFsDiskConnectivityNone          = 0,
    CsvFsDiskConnectivityMdsNodeOnly   = 1,
    CsvFsDiskConnectivitySubsetOfNodes = 2,
    CsvFsDiskConnectivityAllNodes      = 3
} CSVFS_DISK_CONNECTIVITY, *PCSVFS_DISK_CONNECTIVITY;

//
// Output for the CsvControlQueryVolumeRedirectState
//
typedef struct _CSV_QUERY_VOLUME_REDIRECT_STATE {
    DWORD MdsNodeId;
    DWORD DsNodeId;
    BOOLEAN IsDiskConnected;
    BOOLEAN ClusterEnableDirectIo;
    CSVFS_DISK_CONNECTIVITY DiskConnectivity;
} CSV_QUERY_VOLUME_REDIRECT_STATE, *PCSV_QUERY_VOLUME_REDIRECT_STATE;

//
// Structure is defined up to and including field PathLength
//
#define CSV_QUERY_MDS_PATH_V2_VERSION_1 1

#define CSV_QUERY_MDS_PATH_FLAG_STORAGE_ON_THIS_NODE_IS_CONNECTED 0x1
#define CSV_QUERY_MDS_PATH_FLAG_CSV_DIRECT_IO_ENABLED             0x2
#define CSV_QUERY_MDS_PATH_FLAG_SMB_BYPASS_CSV_ENABLED            0x4

//
// Output for the CsvControlGetCsvFsMdsPathV2
//
typedef struct _CSV_QUERY_MDS_PATH_V2 {
    //
    // Version of the structure.
    // CSV_QUERY_MDS_PATH_V2_VERSION_*
    //
    LONGLONG Version;
    //
    // Required output buffer size to completely fit
    // all the data
    //
    DWORD RequiredSize;
    //
    // Id of MDS and of the current node.
    // When these two Ids are the same then current node is MDS
    // otherwise current node is a DS
    //
    DWORD MdsNodeId;
    DWORD DsNodeId;
    //
    // A combination of CSV_QUERY_MDS_PATH_FLAG_* flags
    //
    DWORD Flags;
    //
    // Describes disk connectivity across all cluster nodes
    //
    CSVFS_DISK_CONNECTIVITY DiskConnectivity;
    //
    // Volume GUID of this CSV volume
    //
    GUID VolumeId;
    //
    // Offset to an array of DIPs of MDS and number of bytes
    // in that array. Each array entry has type SOCKADDR_INET
    // number of elements is IpAddressLength / sizeof(SOCKADDR_INET)
    //
    DWORD IpAddressOffset;
    DWORD IpAddressLength;
    //
    // Offset to the a Unicode string that contains
    // path this node is using to open files on MDS, and number of
    // characters in that string
    //
    DWORD PathOffset;
    DWORD PathLength;

} CSV_QUERY_MDS_PATH_V2, *PCSV_QUERY_MDS_PATH_V2;

//
// Input for the CsvControlSetVolumeId
//
typedef struct _CSV_SET_VOLUME_ID {
    GUID VolumeId;
} CSV_SET_VOLUME_ID, *PCSV_SET_VOLUME_ID;

//
// Output for the CsvControlQueryVolumeId
//
typedef struct _CSV_QUERY_VOLUME_ID {
    GUID VolumeId;
} CSV_QUERY_VOLUME_ID, *PCSV_QUERY_VOLUME_ID;

//
//========================= FSCTL_LMR_QUERY_INFO =============================
//

typedef enum _LMR_QUERY_INFO_CLASS {
    LMRQuerySessionInfo = 1,
} LMR_QUERY_INFO_CLASS, *PLMR_QUERY_INFO_CLASS;

typedef struct _LMR_QUERY_INFO_PARAM {
    LMR_QUERY_INFO_CLASS Operation;
} LMR_QUERY_INFO_PARAM, *PLMR_QUERY_INFO_PARAM;

//
// Output for the LMRQuerySessionInfo
//
typedef struct _LMR_QUERY_SESSION_INFO {
    UINT64 SessionId;
} LMR_QUERY_SESSION_INFO, *PLMR_QUERY_SESSION_INFO;

//
//====================== FSCTL_CSV_QUERY_VETO_FILE_DIRECT_IO =========================
//
// In output buffer set Veto to TRUE to prevent CsvFs from
// performing DirectIO on the stream.
// VetoedFromAltitude and Reason are optional fields that
// are used to help with troubleshooting. CSV writes these
// strings to the diagnostic log. Filter can set it to a
// string that would hint why volume is in a redirected mode.
//
typedef struct _CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {
    DWORDLONG VetoedFromAltitudeIntegral;
    DWORDLONG VetoedFromAltitudeDecimal;
    WCHAR   Reason[256];
} CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT, *PCSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT;

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN7 */

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5) || (NTDDI_VERSION >= NTDDI_WIN8) //Win8 check is for backward compatibility.
//
// Storage Reserve common definitions
//

typedef enum _STORAGE_RESERVE_ID {

    StorageReserveIdNone = 0,
    StorageReserveIdHard,
    StorageReserveIdSoft,
    StorageReserveIdUpdateScratch,

    StorageReserveIdMax

} STORAGE_RESERVE_ID, *PSTORAGE_RESERVE_ID;
#endif /* (NTDDI_VERSION >= NTDDI_WIN10_RS5) */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

//
//========================= FSCTL_IS_VOLUME_OWNED_BYCSVFS ============================
//

typedef struct _CSV_IS_OWNED_BY_CSVFS {
    BOOLEAN OwnedByCSVFS;
}CSV_IS_OWNED_BY_CSVFS, *PCSV_IS_OWNED_BY_CSVFS;

//
//======================== FSCTL_FILE_LEVEL_TRIM ===========================
//
//  Structure definitions for supporting file level trim
//

typedef struct _FILE_LEVEL_TRIM_RANGE {

    //
    //  Bytes offset from the front of the given file to trim at
    //

    DWORDLONG Offset;

    //
    //  Length in bytes to trim from the given offset
    //

    DWORDLONG Length;
} FILE_LEVEL_TRIM_RANGE, *PFILE_LEVEL_TRIM_RANGE;

//
//  Input buffer defining what ranges to trim
//

typedef struct _FILE_LEVEL_TRIM {

    //
    // Used when interacting with byte range locks. Set to zero if not SMB or
    //  similar.
    //

    DWORD Key;

    //
    // A count of how many Offset:Length pairs are given
    //

    DWORD NumRanges;

    //
    //  All the pairs.
    //

    FILE_LEVEL_TRIM_RANGE Ranges[1];

} FILE_LEVEL_TRIM, *PFILE_LEVEL_TRIM;

//
//  This is an optional output buffer
//

typedef struct _FILE_LEVEL_TRIM_OUTPUT {

    //
    //  Receives the number of input ranges
    //  that were processed
    //

    DWORD NumRangesProcessed;

} FILE_LEVEL_TRIM_OUTPUT, *PFILE_LEVEL_TRIM_OUTPUT;

//
//==================== FSCTL_QUERY_FILE_LAYOUT ===========================
//

//
// Clear the state of the internal cursor.
//
#define QUERY_FILE_LAYOUT_RESTART                                       (0x00000001)

//
//  Request that the API call retrieve name information for the
//  objects on the volume.
//
#define QUERY_FILE_LAYOUT_INCLUDE_NAMES                                 (0x00000002)

//
//  Request that the API call include streams of the file.
//
#define QUERY_FILE_LAYOUT_INCLUDE_STREAMS                               (0x00000004)

//
//  Include extent information with the attribute entries, where applicable.
//  Use of this flag requires the _INCLUDE_STREAMS flag.
//
#define QUERY_FILE_LAYOUT_INCLUDE_EXTENTS                               (0x00000008)

//
//  Include extra information, such as modification times and security
//  IDs, with each returned file layout entry.
//
#define QUERY_FILE_LAYOUT_INCLUDE_EXTRA_INFO                            (0x00000010)

//
//  Include unallocated attributes in the enumeration, which in NTFS means one
//  of three cases:
//      1. Resident attributes.
//      2. Nonresident attributes of length 0.
//      3. Compressed or sparse nonresident attributes with no physical
//         allocation (consisting only of a sparse hole).
//  This flag may only be used when no cluster ranges are specified (i.e.
//  on a whole-volume query).
//
#define QUERY_FILE_LAYOUT_INCLUDE_STREAMS_WITH_NO_CLUSTERS_ALLOCATED    (0x00000020)

//
//  Request the full path to the file be included in the file name.
//  This flag must be used with QUERY_FILE_LAYOUT_INCLUDE_NAMES
//
#define QUERY_FILE_LAYOUT_INCLUDE_FULL_PATH_IN_NAMES                    (0x00000040)

//
//  Enable QueryFileLayout to include information on attribute streams.
//  Additionally, individual stream information flags must be enabled for
//  information on a given stream to be returned.
//
#define QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION                    (0x00000080)

//
//  Have QueryFileLayout include information (defined by DesiredStorageClass in StreamInformation)
//  on DSC streams.
//  This flag must be used in conjunction with QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION
//
#define QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_DSC_ATTRIBUTE  (0x00000100)

//
//  Have QueryFileLayout include information on TxF streams.
//  This flag must be used in conjunction with QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION
//
#define QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_TXF_ATTRIBUTE  (0x00000200)

//
//  Have QueryFileLayout include information on EFS streams.
//  This flag must be used in conjunction with QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION
//
#define QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_EFS_ATTRIBUTE  (0x00000400)

//
//  We can ask (politely) QueryFileLayout to only return files that have
//  a given set of attributes present. This flag must be used with at least
//  one attribute type flag or Query File Layout will return no files.
//
#define QUERY_FILE_LAYOUT_INCLUDE_ONLY_FILES_WITH_SPECIFIC_ATTRIBUTES   (0x00000800)

//
//  Have QueryFileLayout include files with a DSC attribute in the output buffer.
//  This must be used in conjunction with QUERY_FILE_LAYOUT_INCLUDE_ONLY_FILES_WITH_SPECIFIC_ATTRIBUTES
//
#define QUERY_FILE_LAYOUT_INCLUDE_FILES_WITH_DSC_ATTRIBUTE              (0x00001000)

//
//  Have QueryFileLayout include information (defined by DataStream in StreamInformation) on $DATA streams.
//  This flag must be used in conjunction with QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION
//
#define QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_DATA_ATTRIBUTE (0x00002000)

//
//  Have QueryFileLayout include information (defined by Reparse in StreamInformation) on $REPARSE_POINT streams.
//  This flag must be used in conjunction with QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION
//
#define QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_REPARSE_ATTRIBUTE  (0x00004000)

//
//  Have QueryFileLayout include information (defined by Ea as in StreamInformation) on $EA streams.
//  This flag must be used in conjunction with QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION
//
#define QUERY_FILE_LAYOUT_INCLUDE_STREAM_INFORMATION_FOR_EA_ATTRIBUTE   (0x00008000)

typedef enum _QUERY_FILE_LAYOUT_FILTER_TYPE {

    QUERY_FILE_LAYOUT_FILTER_TYPE_NONE = 0,
    QUERY_FILE_LAYOUT_FILTER_TYPE_CLUSTERS = 1,
    QUERY_FILE_LAYOUT_FILTER_TYPE_FILEID = 2,
#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
    QUERY_FILE_LAYOUT_FILTER_TYPE_STORAGE_RESERVE_ID = 3,
#endif /* (NTDDI_VERSION >= NTDDI_WIN10_RS5) */

    QUERY_FILE_LAYOUT_NUM_FILTER_TYPES

} QUERY_FILE_LAYOUT_FILTER_TYPE;

typedef struct _CLUSTER_RANGE {

    //
    // The starting cluster for this query region
    // (inclusive).
    //
    LARGE_INTEGER       StartingCluster;

    //
    // The length of the cluster region.
    //
    LARGE_INTEGER       ClusterCount;

} CLUSTER_RANGE, *PCLUSTER_RANGE;

typedef struct _FILE_REFERENCE_RANGE {

    //
    // The starting file reference number for this
    // query region (inclusive).
    //
    DWORDLONG           StartingFileReferenceNumber;

    //
    // The ending file reference number for this
    // query region (inclusive).
    //
    DWORDLONG           EndingFileReferenceNumber;

} FILE_REFERENCE_RANGE, *PFILE_REFERENCE_RANGE;

typedef struct _QUERY_FILE_LAYOUT_INPUT {

    //
    // Number of filter entries in the following array.
    // The input buffer must be large enough to contain this
    // number or the operation will fail.
    //
    // This was originally named NumberOfPairs when there
    // was only one type of filter.  The union is simply to
    // maintain code compatibility.
    //

    union {
        DWORD           FilterEntryCount;
        DWORD           NumberOfPairs;
    } DUMMYUNIONNAME;

    //
    // Flags for the operation.
    //
    DWORD               Flags;

    //
    //  The type of filter being applied for this operation.
    //

    QUERY_FILE_LAYOUT_FILTER_TYPE   FilterType;

    //
    //  Reserved for future use. Should be set to zero.
    //

    DWORD               Reserved;

    //
    //  A pointer to the filter-type-specific information.  This is
    //  the caller's actual set of cluster ranges, etc.
    //

    union {

        //
        //  The following is used when the caller wishes to filter
        //  on a set of cluster ranges.
        //

        _When_((FilterType == QUERY_FILE_LAYOUT_FILTER_TYPE_CLUSTERS),
               _Field_size_(FilterEntryCount))
        CLUSTER_RANGE ClusterRanges[1];

        //
        //  The following is used when the caller wishes to filter
        //  on a set of file reference ranges.
        //

        _When_((FilterType == QUERY_FILE_LAYOUT_FILTER_TYPE_FILEID),
               _Field_size_(FilterEntryCount))
        FILE_REFERENCE_RANGE FileReferenceRanges[1];

        //
        //  The following is used when the caller wishes to filter
        //  on a set of storage reserve IDs.
        //

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
        _When_((FilterType == QUERY_FILE_LAYOUT_FILTER_TYPE_STORAGE_RESERVE_ID),
                _Field_size_(FilterEntryCount))
        STORAGE_RESERVE_ID StorageReserveIds[1];
#endif /* (NTDDI_VERSION >= NTDDI_WIN10_RS5) */

    } Filter;

} QUERY_FILE_LAYOUT_INPUT, *PQUERY_FILE_LAYOUT_INPUT;

//
// Indicates that the filesystem is returning stream extents in a
// single-instanced fashion.
//
#define QUERY_FILE_LAYOUT_SINGLE_INSTANCED                              (0x00000001)

typedef struct _QUERY_FILE_LAYOUT_OUTPUT {

    //
    // Number of file entries following this header.
    // Includes only the number of file entries in
    // this iteration.
    //
    DWORD               FileEntryCount;

    //
    // Offset to the first file entry in this buffer,
    // expressed in bytes.
    //
    DWORD               FirstFileOffset;

    //
    // Flags indicating context that is applicable to the
    // entire output set.
    //
    DWORD               Flags;

    //
    // For alignment/later use.
    //
    DWORD               Reserved;

} QUERY_FILE_LAYOUT_OUTPUT, *PQUERY_FILE_LAYOUT_OUTPUT;

typedef struct _FILE_LAYOUT_ENTRY {

    //
    // Version number of this structure
    // (current version number is 1).
    //
    DWORD         Version;

    //
    // Offset to next file entry (in bytes)
    // or zero if this is the last entry.
    //
    DWORD         NextFileOffset;

    //
    // Flags containing context applicable to this
    // file.
    //
    DWORD         Flags;

    //
    // File attributes.
    //
    DWORD         FileAttributes;

    //
    // File ID for this file.
    //
    DWORDLONG     FileReferenceNumber;

    //
    // Offset to the first name entry
    // from the start of this record, or
    // zero if there are no link records.
    //
    DWORD         FirstNameOffset;

    //
    // Offset to the first stream entry
    // from the start of this record, or
    // zero if there are no stream records.
    //
    DWORD         FirstStreamOffset;

    //
    // Offset to additional per-file information,
    // contained in a FILE_LAYOUT_INFO_ENTRY
    // structure, or zero if this information was
    // not returned.
    //
    DWORD         ExtraInfoOffset;

#if (NTDDI_VERSION < NTDDI_WIN10_RS5) && (NTDDI_VERSION < NTDDI_WIN10_RS1) //RS1  check is for backward compatibility.

    //
    // For alignment/future use.
    //
    DWORD         Reserved;
#else
    //
    // Number of bytes accessible in additional per-file
    // information, contained in a FILE_LAYOUT_INFO_ENTRY
    // structure, or zero if this information was
    // not returned.
    //
    // Since pre-RS5 this was a reserved field that was
    // always set to zero by the file system, if
    // ExtraInfoOffset is non-zero but ExtraInfoLength
    // is zero then callers can assume the extra info
    // includes all fields up to Usn.  If ExtraInfoLength
    // is non-zero then it should be used by callers to
    // determine which fields are safe to access.
    //
    DWORD         ExtraInfoLength;
#endif /* (NTDDI_VERSION < NTDDI_WIN10_RS5) */

    //
    // The structure may be extended here to support
    // additional static fields (e.g. pointing to
    // a FILE_BASIC_INFORMATION structure, etc.). This
    // sort of change should coincide with a version
    // number increase.

} FILE_LAYOUT_ENTRY, *PFILE_LAYOUT_ENTRY;

//
// Each file name entry may be one, both, or neither of
// these.
//
#define FILE_LAYOUT_NAME_ENTRY_PRIMARY                                  (0x00000001)
#define FILE_LAYOUT_NAME_ENTRY_DOS                                      (0x00000002)

typedef struct _FILE_LAYOUT_NAME_ENTRY {

    //
    // Offset to next name entry (in bytes)
    // or zero if this is the last entry.
    //
    DWORD         NextNameOffset;

    //
    // Flags for this file name entry.
    //
    DWORD         Flags;

    //
    // Parent FRN for this link.
    //
    DWORDLONG     ParentFileReferenceNumber;

    //
    // File name length (bytes).
    //
    DWORD         FileNameLength;

    //
    // For later use/alignment.
    //
    DWORD         Reserved;

    //
    // Starting point for the name itself
    // (NOT null-terminated).
    //
    _Field_size_bytes_(FileNameLength)
    WCHAR         FileName[1];

} FILE_LAYOUT_NAME_ENTRY, *PFILE_LAYOUT_NAME_ENTRY;

typedef struct _FILE_LAYOUT_INFO_ENTRY {

    //
    // Basic information for this file.
    //
    struct {
        LARGE_INTEGER CreationTime;
        LARGE_INTEGER LastAccessTime;
        LARGE_INTEGER LastWriteTime;
        LARGE_INTEGER ChangeTime;
        DWORD FileAttributes;
    } BasicInformation;

    //
    // Owner ID for this file.
    //
    DWORD                       OwnerId;

    //
    // Security ID for this file.
    //
    DWORD                       SecurityId;

    //
    // Update sequence number for this file.
    //
    USN                         Usn;

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5) || (NTDDI_VERSION >= NTDDI_WIN10_RS1) //RS1 check is for backward compatibility.
    //
    // Storage Reserve ID assigned to the file (0 for none).
    //
    STORAGE_RESERVE_ID  StorageReserveId;
#endif /* (NTDDI_VERSION >= NTDDI_WIN10_RS5) */

} FILE_LAYOUT_INFO_ENTRY, *PFILE_LAYOUT_INFO_ENTRY;

//
// This attribute/stream is known to the filesystem to be immovable.
//
#define STREAM_LAYOUT_ENTRY_IMMOVABLE                                   (0x00000001)

//
// This attribute/stream is currently pinned by another application.
// It is unmovable for the duration of the pin.
//
#define STREAM_LAYOUT_ENTRY_PINNED                                      (0x00000002)

//
// This attribute is resident.
//
#define STREAM_LAYOUT_ENTRY_RESIDENT                                    (0x00000004)

//
// This attribute has no clusters allocated to it.
//
#define STREAM_LAYOUT_ENTRY_NO_CLUSTERS_ALLOCATED                       (0x00000008)

//
// This layout entry contains the information (data) for the attribute
//
#define STREAM_LAYOUT_ENTRY_HAS_INFORMATION                             (0x00000010)

typedef struct _STREAM_LAYOUT_ENTRY {

    //
    // Version of this struct. Current version is 2.
    //
    DWORD         Version;

    //
    // Offset to the next stream entry (bytes).
    //
    DWORD         NextStreamOffset;

    //
    // FSCTL-specific flags.
    //
    DWORD         Flags;

    //
    // Offset to the extent information buffer
    // for this stream, or zero if none exists.
    // This is relative to the start of this
    // stream record.
    //
    DWORD         ExtentInformationOffset;

    //
    // Total allocated size of this stream,
    // in bytes.
    //
    LARGE_INTEGER AllocationSize;

    //
    // End of file location as a byte offset.
    //
    LARGE_INTEGER EndOfFile;

    //
    //  Offset to stream information. This is the
    //  content of the stream
    //
    DWORD         StreamInformationOffset;

    //
    // Attribute code.
    //
    DWORD         AttributeTypeCode;

    //
    // Stream attribute flags.
    //
    DWORD         AttributeFlags;

    //
    // Length of the stream identifier, in bytes.
    //
    DWORD         StreamIdentifierLength;

    //
    // Starting point for the stream identifier
    // buffer.
    //
    _Field_size_bytes_(StreamIdentifierLength)
    WCHAR         StreamIdentifier[1];

} STREAM_LAYOUT_ENTRY, *PSTREAM_LAYOUT_ENTRY;

//
// Flag noting that the extent information may be interpreted as
// a RETRIEVAL_POINTERS_BUFFER structure
//
#define STREAM_EXTENT_ENTRY_AS_RETRIEVAL_POINTERS                       (0x00000001)

//
// Flag noting that all of the stream's extents are returned in
// this structure, even if only some of them fall within the caller's
// specified interest region(s).
//
#define STREAM_EXTENT_ENTRY_ALL_EXTENTS                                 (0x00000002)

typedef struct _STREAM_EXTENT_ENTRY {

    //
    // Extent-level flags for this entry.
    //
    DWORD        Flags;

    union {

        //
        // All that's defined for now is a retrieval
        // pointers buffer, since this is what NTFS
        // will use.
        //
        RETRIEVAL_POINTERS_BUFFER        RetrievalPointers;

    } ExtentInformation;

} STREAM_EXTENT_ENTRY, *PSTREAM_EXTENT_ENTRY;

//
//==================== FSCTL_GET_INTEGRITY_INFORMATION / FSCTL_SET_INTEGRITY_INFORMATION ===========================
//

#define CHECKSUM_TYPE_UNCHANGED         (WORD  )0xFFFF

#define CHECKSUM_TYPE_NONE              (0)
#define CHECKSUM_TYPE_CRC32             (1)
#define CHECKSUM_TYPE_CRC64             (2)
#define CHECKSUM_TYPE_ECC               (3)
#define CHECKSUM_TYPE_SHA256            (4)
#define CHECKSUM_TYPE_XXH64             (5)
#define CHECKSUM_TYPE_FIRST_UNUSED_TYPE (6)

#define FSCTL_INTEGRITY_FLAG_CHECKSUM_ENFORCEMENT_OFF        (1)

typedef struct _FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {
    WORD   ChecksumAlgorithm;   // Checksum algorithm. e.g. CHECKSUM_TYPE_UNCHANGED, CHECKSUM_TYPE_NONE, CHECKSUM_TYPE_CRC32
    WORD   Reserved;            // Must be 0
    DWORD Flags;                // FSCTL_INTEGRITY_FLAG_xxx
    DWORD ChecksumChunkSizeInBytes;
    DWORD ClusterSizeInBytes;
} FSCTL_GET_INTEGRITY_INFORMATION_BUFFER, *PFSCTL_GET_INTEGRITY_INFORMATION_BUFFER;

typedef struct _FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {
    WORD   ChecksumAlgorithm;   // Checksum algorithm. e.g. CHECKSUM_TYPE_UNCHANGED, CHECKSUM_TYPE_NONE, CHECKSUM_TYPE_CRC32
    WORD   Reserved;            // Must be 0
    DWORD Flags;                // FSCTL_INTEGRITY_FLAG_xxx
} FSCTL_SET_INTEGRITY_INFORMATION_BUFFER, *PFSCTL_SET_INTEGRITY_INFORMATION_BUFFER;

//
//==================== FSCTL_SET_INTEGRITY_INFORMATION_EX ===========================
//

#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)
typedef struct _FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {
    BYTE  EnableIntegrity;             // 0 to disable integrity, 1 to enable integrity
    BYTE  KeepIntegrityStateUnchanged; // 1 to keep the current integrity state and ignore the value in EnableIntegrity, 0 to use EnableIntegrity value
    WORD   Reserved;                   // Must be 0
    DWORD Flags;                       // FSCTL_INTEGRITY_FLAG_xxx
    BYTE  Version;                     // Structure version info, must be 1
    BYTE  Reserved2[7];                // Must be 0
} FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX, *PFSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX;
#endif // #if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)

//
//======================== FSCTL_OFFLOAD_READ ==============================
//
//  Structures used by FSCTL_OFFLOAD_READ operation
//

typedef struct _FSCTL_OFFLOAD_READ_INPUT {
    DWORD Size;
    DWORD Flags;
    DWORD TokenTimeToLive; // In milliseconds
    DWORD Reserved;
    DWORDLONG FileOffset;
    DWORDLONG CopyLength;
} FSCTL_OFFLOAD_READ_INPUT, *PFSCTL_OFFLOAD_READ_INPUT;

typedef struct _FSCTL_OFFLOAD_READ_OUTPUT {
    DWORD Size;
    DWORD Flags;
    DWORDLONG TransferLength;
    BYTE  Token[512];
} FSCTL_OFFLOAD_READ_OUTPUT, *PFSCTL_OFFLOAD_READ_OUTPUT;

#define OFFLOAD_READ_FLAG_ALL_ZERO_BEYOND_CURRENT_RANGE       (1)

//
//======================== FSCTL_OFFLOAD_WRITE =============================
//
//  Structures used by FSCTL_OFFLOAD_WRITE operation
//

typedef struct _FSCTL_OFFLOAD_WRITE_INPUT {
    DWORD Size;
    DWORD Flags;
    DWORDLONG FileOffset;
    DWORDLONG CopyLength;
    DWORDLONG TransferOffset;
    BYTE  Token[512];
} FSCTL_OFFLOAD_WRITE_INPUT, *PFSCTL_OFFLOAD_WRITE_INPUT;

typedef struct _FSCTL_OFFLOAD_WRITE_OUTPUT {
    DWORD Size;
    DWORD Flags;
    DWORDLONG LengthWritten;
} FSCTL_OFFLOAD_WRITE_OUTPUT, *PFSCTL_OFFLOAD_WRITE_OUTPUT;

//
//======================== FSCTL_SET_PURGE_FAILURE_MODE ===========================
//
//  Structure definitions for supporting purge failure mode
//

typedef struct _SET_PURGE_FAILURE_MODE_INPUT {
    DWORD Flags;
} SET_PURGE_FAILURE_MODE_INPUT, *PSET_PURGE_FAILURE_MODE_INPUT;

#define SET_PURGE_FAILURE_MODE_ENABLED   0x00000001    // Enable purge failure mode
#define SET_PURGE_FAILURE_MODE_DISABLED  0x00000002    // Disable purge failure mode


//
//======================= FSCTL_REPAIR_COPIES =============================
//

typedef struct _REPAIR_COPIES_INPUT {

    DWORD Size;                                 // sizeof(REPAIR_COPIES_INPUT)

    DWORD Flags;                                // Reserved (must be zero)

    LARGE_INTEGER FileOffset;

    DWORD Length;

    DWORD SourceCopy;                           // The copy number of the source copy.

    DWORD NumberOfRepairCopies;                 // The number of copies that will be repaired.

    DWORD RepairCopies[ANYSIZE_ARRAY];          // The copy numbers of all the copies that will be repaired.

} REPAIR_COPIES_INPUT, *PREPAIR_COPIES_INPUT;

typedef struct _REPAIR_COPIES_OUTPUT {

    DWORD Size;                                 // sizeof(REPAIR_COPIES_OUTPUT)

    DWORD    Status;                            // Operational status

    LARGE_INTEGER ResumeFileOffset;             // File Offset hint to use to resume repair operation skipping
                                                // the range where errors were found (operational Status is non-success).
} REPAIR_COPIES_OUTPUT, *PREPAIR_COPIES_OUTPUT;

//======================= FSCTL_QUERY_FILE_REGIONS =============================

//
//  Structures used for querying for Various file region definitions
//  The initial values if for query VDL
//

//
//  Bit flags which may be OR'd together to define the usage of the given range
//      If the given bit is SET, then the specified region has that attribute
//      if the bit is NOT set, then the specified region does NOT have that attribute
//

#define FILE_REGION_USAGE_VALID_CACHED_DATA     0x00000001
#define FILE_REGION_USAGE_VALID_NONCACHED_DATA  0x00000002
#define FILE_REGION_USAGE_OTHER_PAGE_ALIGNMENT  0x00000004
#define FILE_REGION_USAGE_LARGE_PAGE_ALIGNMENT  0x00000008
#ifdef _WIN64
#define FILE_REGION_USAGE_HUGE_PAGE_ALIGNMENT   0x00000010
#define FILE_REGION_USAGE_QUERY_ALIGNMENT       (FILE_REGION_USAGE_LARGE_PAGE_ALIGNMENT   |\
                                                 FILE_REGION_USAGE_HUGE_PAGE_ALIGNMENT)
#else
#define FILE_REGION_USAGE_QUERY_ALIGNMENT       (FILE_REGION_USAGE_LARGE_PAGE_ALIGNMENT)
#endif  // _WIN64

typedef struct _FILE_REGION_INFO {
    LONGLONG FileOffset;
    LONGLONG Length;
    DWORD Usage;
    DWORD Reserved;
} FILE_REGION_INFO, *PFILE_REGION_INFO;

typedef struct _FILE_REGION_OUTPUT {
    DWORD Flags;                            //none currently defined
    DWORD TotalRegionEntryCount;            //count of total regions that could be returned
    DWORD RegionEntryCount;                 //count of regions that were returned
    DWORD Reserved;                         //for longlong alignment
    FILE_REGION_INFO Region[1];
} FILE_REGION_OUTPUT, *PFILE_REGION_OUTPUT;

//
//  This is an optional buffer given on input to specify a region of the
//  file the caller is interested in
//

typedef struct _FILE_REGION_INPUT {

    LONGLONG FileOffset;
    LONGLONG Length;
    DWORD DesiredUsage;

} FILE_REGION_INPUT, *PFILE_REGION_INPUT;

#endif /*_WIN32_WINNT >= _WIN32_WINNT_WIN8 */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

//======================= FSCTL_WRITE_USN_REASON =============================

//
//  The list of valid USN reasons that can be set with this FSCTL.  Any bits
//  which are specified which are not defined by this mask are silently
//  stripped.  This operation will not be failed for sending undefined reasons
//  This operation is only supported from kernel mode.
//  The output of this operation is a USN structure.
//

#define VALID_WRITE_USN_REASON_MASK     (USN_REASON_DATA_OVERWRITE |        \
                                         USN_REASON_CLOSE)

typedef struct _WRITE_USN_REASON_INPUT {

    DWORD Flags;                        // Flags for this operation (none defined)
    DWORD UsnReasonToWrite;             // A list of USN reasons to set

} WRITE_USN_REASON_INPUT, *PWRITE_USN_REASON_INPUT;

//No Flags currently defined


#endif /*_WIN32_WINNT >= _WIN32_WINNT_WIN8 */

#if (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)

// ****************** FSCTL_QUERY_STORAGE_CLASSES ***************************

//
//  The FILE_STORAGE_TIER is an identifier for the storage tier relative to the volume.
//  The storage tier ID for a particular volume has no relationship to the storage tier
//  ID with the same value on a different volume.
//

//
//  Note! The MediaType is used to indicate an uninitialized in-memory DSC structure.
//  Do not use values 0xfe or 0xff as media types.
//

#define FILE_STORAGE_TIER_NAME_LENGTH           (256)
#define FILE_STORAGE_TIER_DESCRIPTION_LENGTH    (512)

//
//  These flags *must* match those for _STORAGE_TIER as the file system does a
//  pass through.
//

#define FILE_STORAGE_TIER_FLAG_NO_SEEK_PENALTY    (0x00020000)
#define FILE_STORAGE_TIER_FLAG_WRITE_BACK_CACHE   (0x00200000)
#define FILE_STORAGE_TIER_FLAG_READ_CACHE         (0x00400000)
#define FILE_STORAGE_TIER_FLAG_PARITY             (0x00800000)
#define FILE_STORAGE_TIER_FLAG_SMR                (0x01000000)

typedef enum _FILE_STORAGE_TIER_MEDIA_TYPE {

    FileStorageTierMediaTypeUnspecified = 0,
    FileStorageTierMediaTypeDisk        = 1,
    FileStorageTierMediaTypeSsd         = 2,
    FileStorageTierMediaTypeScm         = 4,
    FileStorageTierMediaTypeMax

} FILE_STORAGE_TIER_MEDIA_TYPE, *PFILE_STORAGE_TIER_MEDIA_TYPE;

typedef enum _FILE_STORAGE_TIER_CLASS {

    FileStorageTierClassUnspecified = 0,
    FileStorageTierClassCapacity,
    FileStorageTierClassPerformance,
    FileStorageTierClassMax

} FILE_STORAGE_TIER_CLASS, *PFILE_STORAGE_TIER_CLASS;

typedef struct _FILE_STORAGE_TIER {

    //
    // Tier ID
    //

    GUID Id;

    //
    // Name for the tier
    //

    WCHAR Name[FILE_STORAGE_TIER_NAME_LENGTH];

    //
    // Note for the tier
    //

    WCHAR Description[FILE_STORAGE_TIER_NAME_LENGTH];

    //
    // Flags: FILE_STORAGE_TIER_FLAG_xxx
    //

    DWORDLONG Flags;

    //
    // Provisioned capacity of the tier
    //

    DWORDLONG ProvisionedCapacity;

    //
    // Media type of the tier
    //

    FILE_STORAGE_TIER_MEDIA_TYPE MediaType;

    //
    // Classification of the tier
    //

    FILE_STORAGE_TIER_CLASS Class;

} FILE_STORAGE_TIER, *PFILE_STORAGE_TIER;

//
//  This structure has the same fields as STORAGE_DEVICE_TIERING_DESCRIPTOR and
//  that structure reserves the upper WORD   of the Flags field for file system use.
//  We define the following possible values for the Flags field.
//

#define QUERY_STORAGE_CLASSES_FLAGS_MEASURE_WRITE   0x80000000
#define QUERY_STORAGE_CLASSES_FLAGS_MEASURE_READ    0x40000000
#define QUERY_STORAGE_CLASSES_FLAGS_NO_DEFRAG_VOLUME   0x20000000

//
//  The response returns a single structure of FSCTL_QUERY_STORAGE_CLASSES_OUTPUT
//  that has all the tiers for this volume.
//

typedef _Struct_size_bytes_(Size) struct _FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {

    //
    // Size of this structure serves
    // as the version
    //

    DWORD Version;

    //
    // Size of  this structure  plus
    // all the variable sized fields
    //

    DWORD Size;

    //
    // Flags
    //

    DWORD Flags;

    //
    // The total number of available tiers for this disk
    //

    DWORD TotalNumberOfTiers;

    //
    // The number of tiers that fit in the output
    //

    DWORD NumberOfTiersReturned;

    //
    // Detailed info on the storage tiers.
    //

    FILE_STORAGE_TIER Tiers[ANYSIZE_ARRAY];

} FSCTL_QUERY_STORAGE_CLASSES_OUTPUT, *PFSCTL_QUERY_STORAGE_CLASSES_OUTPUT;

#define FSCTL_QUERY_STORAGE_CLASSES_OUTPUT_VERSION          sizeof(FSCTL_QUERY_STORAGE_CLASSES_OUTPUT)

//
//  Below are flags used by the Reparse union type within STREAM_INFORMATION_ENTRY.
//

#define QUERY_FILE_LAYOUT_REPARSE_DATA_INVALID              (0x0001)  // invalid reparse data, corresponds to ERROR_INVALID_REPARSE_DATA
#define QUERY_FILE_LAYOUT_REPARSE_TAG_INVALID               (0x0002)  // invalid reparse tag, corresponds to ERROR_REPARSE_TAG_INVALID

//
//  This structure lists information on the stream.
//

typedef struct _STREAM_INFORMATION_ENTRY {

    //
    // Version of this struct. Current version is 1.
    //
    DWORD         Version;

    //
    // Flags
    //
    DWORD         Flags;

    //
    // The stream information varies by type of stream. We enclose
    // the various types in a union.
    //
    union _StreamInformation {

        //
        //  Desired Storage Class
        //

        struct _DesiredStorageClass {

            //
            //  Class
            //

            FILE_STORAGE_TIER_CLASS          Class;

            //
            //  Flags
            //

            DWORD                            Flags;

        } DesiredStorageClass;

#if (NTDDI_VERSION >= NTDDI_WIN10_VB)
        struct _DataStream {

            //
            //  Total Length of STREAM_INFORMATION_ENTRY structure.
            //

            WORD        Length;

            //
            //  Flags (Reserved for future use)
            //

            WORD        Flags;

            //
            //  Reserved.
            //

            DWORD       Reserved;

            //
            //  The Vdl (Valid Data Length) of data stream.
            //

            DWORDLONG   Vdl;

        } DataStream;

        struct _Reparse {

            //
            //  Total Length of STREAM_INFORMATION_ENTRY structure.
            //

            WORD   Length;

            //
            //  Flags
            //

            WORD   Flags;

            //
            //  The size of Reparse data buffer.
            //

            DWORD ReparseDataSize;

            //
            //  Offset to reparse point data buffer (REPARSE_DATA_BUFFER or REPARSE_GUID_DATA_BUFFER).
            //

            DWORD ReparseDataOffset;

        } Reparse;

        struct _Ea {

            //
            //  Total Length of STREAM_INFORMATION_ENTRY structure.
            //

            WORD   Length;

            //
            //  Flags (Reserved for future use)
            //

            WORD   Flags;

            //
            //  The size of Ea.
            //

            DWORD EaSize;

            //
            //  Offset to EA (Extended Attributes) information buffer (FILE_FULL_EA_INFORMATION).
            //

            DWORD EaInformationOffset;

        } Ea;
#endif // (NTDDI_VERSION >= NTDDI_WIN10_VB)

    } StreamInformation;

} STREAM_INFORMATION_ENTRY, *PSTREAM_INFORMATION_ENTRY;

// ****************** FSCTL_QUERY_REGION_INFO *******************************

//
//  Input structure for FSCTL_QUERY_REGION_INFO.
//
//  This FSCTL returns the storage tier regions from the storage
//  stack for a particular volume.
//

typedef struct _FSCTL_QUERY_REGION_INFO_INPUT {

    DWORD     Version;                      // The version of this structure.
    DWORD     Size;                         // The size of this structure in bytes.

    DWORD     Flags;                        // Reserved for future use.

    DWORD     NumberOfTierIds;              // Number of entries in TierIds, 0 for all for volume
    GUID      TierIds[ANYSIZE_ARRAY];       // Storage tiers to return information for

} FSCTL_QUERY_REGION_INFO_INPUT, *PFSCTL_QUERY_REGION_INFO_INPUT;

#define FSCTL_QUERY_REGION_INFO_INPUT_VERSION               sizeof(FSCTL_QUERY_REGION_INFO_INPUT)

//
//  Structure that describes a single storage tier region.
//

typedef struct _FILE_STORAGE_TIER_REGION {

    GUID        TierId;     // Tier ID

    DWORDLONG   Offset;     // offset of region in bytes
    DWORDLONG   Length;     // length of region in bytes

} FILE_STORAGE_TIER_REGION, *PFILE_STORAGE_TIER_REGION;

//
//  Output structure for FSCTL_QUERY_REGION_INFO.
//
//  The FSCTL_QUERY_REGION_INFO response returns a single one of these that include
//  multiple FILE_STORAGE_TIER_REGION records, one for each region.
//

typedef struct _FSCTL_QUERY_REGION_INFO_OUTPUT {

    DWORD     Version;                      // The version of this structure.
    DWORD     Size;                         // The size of this structure in bytes.

    DWORD     Flags;                        // Reserved for future use.
    DWORD     Reserved;                     // Reserved for future use.

    DWORDLONG Alignment;                    // in bytes, must align to slab boundary

    DWORD     TotalNumberOfRegions;             // Total number of available regions.
    DWORD     NumberOfRegionsReturned;          // Number of regions that fit in the output.

    FILE_STORAGE_TIER_REGION Regions[ANYSIZE_ARRAY]; // Detailed info on the regions.

} FSCTL_QUERY_REGION_INFO_OUTPUT, *PFSCTL_QUERY_REGION_INFO_OUTPUT;

#define FSCTL_QUERY_REGION_INFO_OUTPUT_VERSION              sizeof(FSCTL_QUERY_REGION_INFO_OUTPUT)

//
//  This structure contains the information for the Desired Storage Class attribute.
//

typedef struct _FILE_DESIRED_STORAGE_CLASS_INFORMATION {

    //
    // Class type of the tier
    //

    FILE_STORAGE_TIER_CLASS Class;

    //
    // Flags
    //

    DWORD Flags;

} FILE_DESIRED_STORAGE_CLASS_INFORMATION, *PFILE_DESIRED_STORAGE_CLASS_INFORMATION;

//
//  This structure has the same fields as STORAGE_DEVICE_TIERING_DESCRIPTOR and
//  that structure reserves the upper WORD   of the Flags field for file system use.
//  We define the following possible values for the Flags field.
//

#define QUERY_STORAGE_CLASSES_FLAGS_MEASURE_WRITE   0x80000000
#define QUERY_STORAGE_CLASSES_FLAGS_MEASURE_READ    0x40000000

//
//=============== FSCTL_DUPLICATE_EXTENTS_TO_FILE ====================
//

typedef struct _DUPLICATE_EXTENTS_DATA {
    HANDLE FileHandle;
    LARGE_INTEGER SourceFileOffset;
    LARGE_INTEGER TargetFileOffset;
    LARGE_INTEGER ByteCount;
} DUPLICATE_EXTENTS_DATA, *PDUPLICATE_EXTENTS_DATA;

#if ((NTDDI_VERSION >= NTDDI_WIN10_RS2) && defined(_WIN64))

//
//  32/64 Bit thunking support structure
//

typedef struct _DUPLICATE_EXTENTS_DATA32 {
    UINT32 FileHandle;
    LARGE_INTEGER SourceFileOffset;
    LARGE_INTEGER TargetFileOffset;
    LARGE_INTEGER ByteCount;
} DUPLICATE_EXTENTS_DATA32, *PDUPLICATE_EXTENTS_DATA32;

#endif /* ((NTDDI_VERSION >= NTDDI_WIN10_RS2) && defined(_WIN64)) */

#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE) */

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)

//
//=============== FSCTL_DUPLICATE_EXTENTS_TO_FILE_EX ==================
//

#define DUPLICATE_EXTENTS_DATA_EX_SOURCE_ATOMIC     0x00000001
#if (NTDDI_VERSION >= NTDDI_WIN10_VB)
#define DUPLICATE_EXTENTS_DATA_EX_ASYNC             0x00000002
#endif // (NTDDI_VERSION >= NTDDI_WIN10_VB)

typedef struct _DUPLICATE_EXTENTS_DATA_EX {
    SIZE_T Size;
    HANDLE FileHandle;
    LARGE_INTEGER SourceFileOffset;
    LARGE_INTEGER TargetFileOffset;
    LARGE_INTEGER ByteCount;
    DWORD Flags;
} DUPLICATE_EXTENTS_DATA_EX, *PDUPLICATE_EXTENTS_DATA_EX;

#if ((NTDDI_VERSION >= NTDDI_WIN10_RS3) && defined(_WIN64))

//
//  32/64 Bit thunking support structure
//

typedef struct _DUPLICATE_EXTENTS_DATA_EX32 {
    DWORD32 Size;
    DWORD32 FileHandle;
    LARGE_INTEGER SourceFileOffset;
    LARGE_INTEGER TargetFileOffset;
    LARGE_INTEGER ByteCount;
    DWORD Flags;
} DUPLICATE_EXTENTS_DATA_EX32, *PDUPLICATE_EXTENTS_DATA_EX32;

#endif /* ((NTDDI_VERSION >= NTDDI_WIN10_RS3) && defined(_WIN64)) */

#endif /* (NTDDI_VERSION >= NTDDI_WIN10_RS3) */

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)

//
//=============== FSCTL_QUERY_ASYNC_DUPLICATE_EXTENTS_STATUS ==================
//

typedef enum _DUPLICATE_EXTENTS_STATE {

    FileSnapStateInactive = 0,
    FileSnapStateSource,
    FileSnapStateTarget,

} DUPLICATE_EXTENTS_STATE, *PDUPLICATE_EXTENTS_STATE;

typedef struct _ASYNC_DUPLICATE_EXTENTS_STATUS {

    DWORD Version;

    DUPLICATE_EXTENTS_STATE State;

    DWORDLONG SourceFileOffset;
    DWORDLONG TargetFileOffset;
    DWORDLONG ByteCount;

    DWORDLONG BytesDuplicated;

} ASYNC_DUPLICATE_EXTENTS_STATUS, *PASYNC_DUPLICATE_EXTENTS_STATUS;

#define ASYNC_DUPLICATE_EXTENTS_STATUS_V1   sizeof(ASYNC_DUPLICATE_EXTENTS_STATUS)

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

//
//==================== FSCTL_QUERY_REFS_SMR_VOLUME_INFO =======================
//

#define REFS_SMR_VOLUME_INFO_OUTPUT_VERSION_V0      0
#define REFS_SMR_VOLUME_INFO_OUTPUT_VERSION_V1      1

typedef enum _REFS_SMR_VOLUME_GC_STATE {

    SmrGcStateInactive = 0,
    SmrGcStatePaused = 1,
    SmrGcStateActive = 2,
    SmrGcStateActiveFullSpeed = 3,

} REFS_SMR_VOLUME_GC_STATE, *PREFS_SMR_VOLUME_GC_STATE;

typedef struct _REFS_SMR_VOLUME_INFO_OUTPUT {

    DWORD Version;
    DWORD Flags;

    LARGE_INTEGER SizeOfRandomlyWritableTier;
    LARGE_INTEGER FreeSpaceInRandomlyWritableTier;
    LARGE_INTEGER SizeofSMRTier;
    LARGE_INTEGER FreeSpaceInSMRTier;
    LARGE_INTEGER UsableFreeSpaceInSMRTier;

    REFS_SMR_VOLUME_GC_STATE VolumeGcState;
    DWORD    VolumeGcLastStatus;

    //
    //  Fields added in V1
    //

    DWORD CurrentGcBandFillPercentage;

    DWORDLONG Unused[6];

} REFS_SMR_VOLUME_INFO_OUTPUT, *PREFS_SMR_VOLUME_INFO_OUTPUT;

//
//==================== FSCTL_SET_REFS_SMR_VOLUME_GC_PARAMETERS =======================
//

#define REFS_SMR_VOLUME_GC_PARAMETERS_VERSION_V1    1

typedef enum _REFS_SMR_VOLUME_GC_ACTION {

    SmrGcActionStart = 1,
    SmrGcActionStartFullSpeed = 2,
    SmrGcActionPause = 3,
    SmrGcActionStop = 4,

} REFS_SMR_VOLUME_GC_ACTION, *PREFS_SMR_VOLUME_GC_ACTION;

typedef enum _REFS_SMR_VOLUME_GC_METHOD {

    SmrGcMethodCompaction = 1,
    SmrGcMethodCompression = 2,
    SmrGcMethodRotation = 3,

} REFS_SMR_VOLUME_GC_METHOD, *PREFS_SMR_VOLUME_GC_METHOD;

typedef struct _REFS_SMR_VOLUME_GC_PARAMETERS {

    DWORD Version;
    DWORD Flags;

    REFS_SMR_VOLUME_GC_ACTION Action;
    REFS_SMR_VOLUME_GC_METHOD Method;

    DWORD IoGranularity;
    DWORD CompressionFormat;

    DWORDLONG Unused[8];

} REFS_SMR_VOLUME_GC_PARAMETERS, *PREFS_SMR_VOLUME_GC_PARAMETERS;

//
//==================== STREAMS CONSTANTS =======================
//

#define STREAMS_INVALID_ID                      (0)
#define STREAMS_MAX_ID                          (MAXWORD  )

//
//==================== FSCTL_STREAMS_QUERY_PARAMETERS =======================
//

typedef struct _STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {

    DWORD OptimalWriteSize;
    DWORD StreamGranularitySize;
    DWORD StreamIdMin;
    DWORD StreamIdMax;

} STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER, *PSTREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER;

//
//==================== FSCTL_STREAMS_ASSOCIATE_ID =======================
//

#define STREAMS_ASSOCIATE_ID_CLEAR              (0x1)
#define STREAMS_ASSOCIATE_ID_SET                (0x2)

typedef struct _STREAMS_ASSOCIATE_ID_INPUT_BUFFER {

    DWORD Flags;
    DWORD StreamId;

} STREAMS_ASSOCIATE_ID_INPUT_BUFFER, *PSTREAMS_ASSOCIATE_ID_INPUT_BUFFER;

//
//==================== FSCTL_STREAMS_QUERY_ID =======================
//

typedef struct _STREAMS_QUERY_ID_OUTPUT_BUFFER {

    DWORD StreamId;

} STREAMS_QUERY_ID_OUTPUT_BUFFER, *PSTREAMS_QUERY_ID_OUTPUT_BUFFER;

#endif // #if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS3)

//
//=============== FSCTL_QUERY_BAD_RANGES ==================
//

typedef struct _QUERY_BAD_RANGES_INPUT_RANGE {

    //
    //  Starting offset of the range in bytes.
    //

    DWORDLONG StartOffset;

    //
    //  Length of the range in bytes.
    //

    DWORDLONG LengthInBytes;

} QUERY_BAD_RANGES_INPUT_RANGE, *PQUERY_BAD_RANGES_INPUT_RANGE;

//
//  Input buffer defining the ranges in the file to look for
//  bad ranges.  A NULL input buffer would lookup the entire
//  file.
//

typedef struct _QUERY_BAD_RANGES_INPUT {

    DWORD Flags;

    //
    //  Number of ranges given, in the Ranges array.
    //

    DWORD NumRanges;

    //
    //  Array of ranges (<Offset,  Length> pairs)  to look for
    //  bad ranges.  A range with MAXDWORDDWORD as LengthInBytes
    //  would represent a range till end of the file allocation.
    //  For that matter a range going beyond allocation size
    //  will be capped at allocation size.
    //

    QUERY_BAD_RANGES_INPUT_RANGE Ranges[1];

} QUERY_BAD_RANGES_INPUT, *PQUERY_BAD_RANGES_INPUT;

typedef struct _QUERY_BAD_RANGES_OUTPUT_RANGE {

    //
    //  Flags, reserved for future.
    //

    DWORD Flags;

    DWORD Reserved;

    //
    //  Starting offset of the range in bytes.
    //

    DWORDLONG StartOffset;

    //
    //  Length of the range in bytes.
    //

    DWORDLONG LengthInBytes;

} QUERY_BAD_RANGES_OUTPUT_RANGE, *PQUERY_BAD_RANGES_OUTPUT_RANGE;

//
//  Output buffer defining the bad ranges.
//

typedef struct _QUERY_BAD_RANGES_OUTPUT {

    DWORD Flags;

    //
    //  Number of bad ranges populated in the Ranges
    //  array.
    //

    DWORD NumBadRanges;

    //
    //  If the output buffer is not big enough to hold all the
    //  bad ranges, the FSCTL would get STATUS_BUFFER_OVERFLOW
    //  with as many bad ranges as  that can fit  in the given
    //  output buffer and  NextOffsetToLookUp gives the offset
    //  from which the user can requery the remaining bad ranges.
    //

    DWORDLONG NextOffsetToLookUp;

    //
    //  Array of bad ranges (<Offset, Length> pairs) in the
    //  ranges that user asked for.
    //

    QUERY_BAD_RANGES_OUTPUT_RANGE BadRanges[1];

} QUERY_BAD_RANGES_OUTPUT, *PQUERY_BAD_RANGES_OUTPUT;

//
//========= FSCTL_SET_DAX_ALLOC_ALIGNMENT_HINT =========
//
//  Once this FSCTL is issued for a file, the following
//  allocations to the file will be examined to see if
//  the given offset is being allocated and if so the
//  allocation will be placed such that the physical
//  address corresponding to that allocation is aligned
//  to the given AlignmentValue, if we have free space
//  available at aligned regions.
//
//  To reset the alignment constraint you can issue this
//  FSCTL with AlignmentValue of 1 i.e. AlignmentShift
//  as 0.
//

typedef struct _SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {

    DWORD Flags;

    //
    //  The block size to align the given offset of the file.
    //  The file is allocated such that the physical address
    //  corresponding to the file is aligned to this block
    //  size.  The actual alignment value is given by:
    //
    //    AlignmentValue = (1 << AlignmentShift);
    //

    DWORD AlignmentShift;

    //
    //  Offset in file that should be aligned to the given
    //  AlignmentValue.  This should be cluster aligned.
    //
    //  NOTE: MM supports huge page only if the file offset
    //  and the physical address are 1gb aligned.
    //

    DWORDLONG FileOffsetToAlign;

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS4)
    //
    //  If DAX_ALLOC_ALIGNMENT_FLAG_FALLBACK_SPECIFIED is present in
    //  Flags, this field specifies a fallback block size to align
    //  the given offset of the file whenever allocation satisfying
    //  AlignmentShift could not be found.
    //

    DWORD FallbackAlignmentShift;
#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS4) */

} SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT, *PSET_DAX_ALLOC_ALIGNMENT_HINT_INPUT;

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS4)

//
//  MANDATORY - If allocation satisfying AlignmentShift (or at least
//     FallbackAlignmentShift if specified) cannot be found, then fail
//     the file system operation (e.g. extending the file).
//

#define DAX_ALLOC_ALIGNMENT_FLAG_MANDATORY             (0x00000001)

//
//  FALLBACK_SPECIFIED - Indicates that the FallbackAlignmentShift field
//      is present in the input structure and indicates a fallback
//      alignment if the optimal alignment isn't available.
//

#define DAX_ALLOC_ALIGNMENT_FLAG_FALLBACK_SPECIFIED    (0x00000002)

#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS4) */

//
//========= FSCTL_VIRTUAL_STORAGE_SET_BEHAVIOR =========
//
//  Configures file system-specific behaviors for files
//  used as backing stored for virtual storage devices.
//

typedef enum _VIRTUAL_STORAGE_BEHAVIOR_CODE {

    VirtualStorageBehaviorUndefined = 0,
    VirtualStorageBehaviorCacheWriteThrough = 1,
    VirtualStorageBehaviorCacheWriteBack = 2,
    VirtualStorageBehaviorStopIoProcessing = 3,
    VirtualStorageBehaviorRestartIoProcessing = 4

} VIRTUAL_STORAGE_BEHAVIOR_CODE, *PVIRTUAL_STORAGE_BEHAVIOR_CODE;

typedef struct _VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {

    DWORD Size;
    VIRTUAL_STORAGE_BEHAVIOR_CODE BehaviorCode;

} VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT, *PVIRTUAL_STORAGE_SET_BEHAVIOR_INPUT;

//
// EFS DPL key availability data structure for FSCTL_ENCRYPTION_KEY_CONTROL
//

typedef struct _ENCRYPTION_KEY_CTRL_INPUT {

    DWORD HeaderSize;           // Structure header size, usable for structure versioning.

    DWORD StructureSize;        // Full structure size.

    WORD   KeyOffset;           // Byte offset of the key blob relative to the start of this structure.
                                // Could be 0 if key blob is not passed.

    WORD   KeySize;             // Size of the key blob.
                                // Could be 0 if key blob is not passed.

    DWORD DplLock;              // DPL lock/unlock indicator: 1 means lock, 0 means unlock.

    DWORDLONG DplUserId;        // DPL user runtime ID for who this control is issued.

    DWORDLONG DplCredentialId;  // DPL credential runtime ID which is being impacted.

} ENCRYPTION_KEY_CTRL_INPUT, *PENCRYPTION_KEY_CTRL_INPUT;

#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS3) */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)

//
// Generic fsctl headers which can be sent to the driver.
//

#define WOF_CURRENT_VERSION (0x00000001)

#define WOF_PROVIDER_WIM            (0x00000001)
#define WOF_PROVIDER_FILE           (0x00000002)
#define WOF_PROVIDER_CLOUD          (0x00000003)

typedef struct _WOF_EXTERNAL_INFO {
    DWORD Version;
    DWORD Provider;
} WOF_EXTERNAL_INFO, *PWOF_EXTERNAL_INFO;

typedef struct _WOF_EXTERNAL_FILE_ID {
    FILE_ID_128 FileId;
} WOF_EXTERNAL_FILE_ID, *PWOF_EXTERNAL_FILE_ID;

typedef struct _WOF_VERSION_INFO {
    DWORD WofVersion;
} WOF_VERSION_INFO, *PWOF_VERSION_INFO;

//
// Structures for WIM provider specific fsctl's.
//

#ifndef WIM_PROVIDER_HASH_SIZE
#define WIM_PROVIDER_HASH_SIZE 20
#endif

#define WIM_PROVIDER_CURRENT_VERSION  (0x00000001)

#define WIM_PROVIDER_EXTERNAL_FLAG_NOT_ACTIVE    (0x00000001)
#define WIM_PROVIDER_EXTERNAL_FLAG_SUSPENDED     (0x00000002)

typedef struct _WIM_PROVIDER_EXTERNAL_INFO {
    DWORD Version;
    DWORD Flags;
    LARGE_INTEGER DataSourceId;
    BYTE  ResourceHash[WIM_PROVIDER_HASH_SIZE];
} WIM_PROVIDER_EXTERNAL_INFO, *PWIM_PROVIDER_EXTERNAL_INFO;

//
//  WimType specifies a set of flags corresponding to the type of WIM.
//
//  WIM_BOOT_OS_WIM means the wim contains Windows system files.
//  WIM_BOOT_NOT_OS_WIM means the wim contains 3rd party files or data files.
//
//  Do not use WIM_BOOT_OS_WIM for data that should be preserved on
//  system upgrade.
//

#define WIM_BOOT_OS_WIM       (0x00000001)
#define WIM_BOOT_NOT_OS_WIM   (0x00000000)

typedef struct _WIM_PROVIDER_ADD_OVERLAY_INPUT {
    DWORD WimType;
    DWORD WimIndex;
    DWORD WimFileNameOffset;
    DWORD WimFileNameLength;
} WIM_PROVIDER_ADD_OVERLAY_INPUT, *PWIM_PROVIDER_ADD_OVERLAY_INPUT;

typedef struct _WIM_PROVIDER_UPDATE_OVERLAY_INPUT {
    LARGE_INTEGER DataSourceId;
    DWORD WimFileNameOffset;
    DWORD WimFileNameLength;
} WIM_PROVIDER_UPDATE_OVERLAY_INPUT, *PWIM_PROVIDER_UPDATE_OVERLAY_INPUT;

typedef struct _WIM_PROVIDER_REMOVE_OVERLAY_INPUT {
    LARGE_INTEGER DataSourceId;
} WIM_PROVIDER_REMOVE_OVERLAY_INPUT, *PWIM_PROVIDER_REMOVE_OVERLAY_INPUT;

typedef struct _WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {
    LARGE_INTEGER DataSourceId;
} WIM_PROVIDER_SUSPEND_OVERLAY_INPUT, *PWIM_PROVIDER_SUSPEND_OVERLAY_INPUT;

typedef struct _WIM_PROVIDER_OVERLAY_ENTRY {
    DWORD NextEntryOffset;
    LARGE_INTEGER DataSourceId;
    GUID WimGuid;
    DWORD WimFileNameOffset;
    DWORD WimType;
    DWORD WimIndex;
    DWORD Flags;
} WIM_PROVIDER_OVERLAY_ENTRY, *PWIM_PROVIDER_OVERLAY_ENTRY;

#endif


#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)

//
// Structures for WOF File provider specific fsctl's.
//

#define FILE_PROVIDER_CURRENT_VERSION        (0x00000001)

#define FILE_PROVIDER_SINGLE_FILE            (0x00000001)

#define FILE_PROVIDER_COMPRESSION_XPRESS4K   (0x00000000)
#define FILE_PROVIDER_COMPRESSION_LZX        (0x00000001)
#define FILE_PROVIDER_COMPRESSION_XPRESS8K   (0x00000002)
#define FILE_PROVIDER_COMPRESSION_XPRESS16K  (0x00000003)
#define FILE_PROVIDER_COMPRESSION_MAXIMUM    (0x00000004)

#define FILE_PROVIDER_FLAG_COMPRESS_ON_WRITE (0x00000001)

typedef struct _FILE_PROVIDER_EXTERNAL_INFO_V0 {
    DWORD Version;
    DWORD Algorithm;
} FILE_PROVIDER_EXTERNAL_INFO_V0, *PFILE_PROVIDER_EXTERNAL_INFO_V0;

typedef struct _FILE_PROVIDER_EXTERNAL_INFO_V1 {
    DWORD Version;
    DWORD Algorithm;
    DWORD Flags;
} FILE_PROVIDER_EXTERNAL_INFO_V1, *PFILE_PROVIDER_EXTERNAL_INFO_V1;

typedef FILE_PROVIDER_EXTERNAL_INFO_V1  FILE_PROVIDER_EXTERNAL_INFO;
typedef PFILE_PROVIDER_EXTERNAL_INFO_V1 PFILE_PROVIDER_EXTERNAL_INFO;

#endif  //  (_WIN32_WINNT >= _WIN32_WINNT_WIN7)

#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)

typedef struct _CONTAINER_VOLUME_STATE {
    DWORD Flags;
} CONTAINER_VOLUME_STATE, *PCONTAINER_VOLUME_STATE;

#define CONTAINER_VOLUME_STATE_HOSTING_CONTAINER    (0x00000001)

typedef struct _CONTAINER_ROOT_INFO_INPUT {
    DWORD Flags;
} CONTAINER_ROOT_INFO_INPUT, *PCONTAINER_ROOT_INFO_INPUT;

typedef struct _CONTAINER_ROOT_INFO_OUTPUT {
    WORD   ContainerRootIdLength;
    BYTE  ContainerRootId[ANYSIZE_ARRAY];
} CONTAINER_ROOT_INFO_OUTPUT, *PCONTAINER_ROOT_INFO_OUTPUT;

#define CONTAINER_ROOT_INFO_FLAG_SCRATCH_ROOT                   (0x00000001)
#define CONTAINER_ROOT_INFO_FLAG_LAYER_ROOT                     (0x00000002)
#define CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_ROOT            (0x00000004)
#define CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_TARGET_ROOT     (0x00000008)
#define CONTAINER_ROOT_INFO_FLAG_VIRTUALIZATION_EXCEPTION_ROOT  (0x00000010)
#define CONTAINER_ROOT_INFO_FLAG_BIND_ROOT                      (0x00000020)
#define CONTAINER_ROOT_INFO_FLAG_BIND_TARGET_ROOT               (0x00000040)
#define CONTAINER_ROOT_INFO_FLAG_BIND_EXCEPTION_ROOT            (0x00000080)
#define CONTAINER_ROOT_INFO_FLAG_BIND_DO_NOT_MAP_NAME           (0x00000100)
#define CONTAINER_ROOT_INFO_FLAG_UNION_LAYER_ROOT               (0x00000200)

#define CONTAINER_ROOT_INFO_VALID_FLAGS                         (0x000003ff)

#endif


#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS1)

typedef struct _VIRTUALIZATION_INSTANCE_INFO_INPUT {
    DWORD NumberOfWorkerThreads;
    DWORD Flags;
} VIRTUALIZATION_INSTANCE_INFO_INPUT, *PVIRTUALIZATION_INSTANCE_INFO_INPUT;

#define PROJFS_PROTOCOL_VERSION     3

typedef struct _VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {
    WORD   HeaderSize;               // sizeof(VIRTUALIZATION_INSTANCE_INFO_INPUT_EX)
    DWORD Flags;
    DWORD NotificationInfoSize;      // Total Size of the NotificationInfo Buffer.
    WORD   NotificationInfoOffset;   // Offset from beginning of this struct to the NotificationInfo Buffer.
    WORD   ProviderMajorVersion;     // This should be set to PROJFS_PROTOCOL_VERSION.
} VIRTUALIZATION_INSTANCE_INFO_INPUT_EX, *PVIRTUALIZATION_INSTANCE_INFO_INPUT_EX;

typedef struct _VIRTUALIZATION_INSTANCE_INFO_OUTPUT {
    GUID VirtualizationInstanceID;
} VIRTUALIZATION_INSTANCE_INFO_OUTPUT, *PVIRTUALIZATION_INSTANCE_INFO_OUTPUT;

//
//  Structures for FSCTL_GET_FILTER_FILE_IDENTIFIER.
//

typedef struct _GET_FILTER_FILE_IDENTIFIER_INPUT {
    WORD   AltitudeLength;
    WCHAR Altitude[ANYSIZE_ARRAY];
} GET_FILTER_FILE_IDENTIFIER_INPUT, *PGET_FILTER_FILE_IDENTIFIER_INPUT;

typedef struct _GET_FILTER_FILE_IDENTIFIER_OUTPUT {
    WORD   FilterFileIdentifierLength;
    BYTE  FilterFileIdentifier[ANYSIZE_ARRAY];
} GET_FILTER_FILE_IDENTIFIER_OUTPUT, *PGET_FILTER_FILE_IDENTIFIER_OUTPUT;

#endif  //  (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS1)

// end_ntosifs
// begin_ntifs begin_winioctl

#if (NTDDI_VERSION >= NTDDI_WIN10_CO)

#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/unions

//
//============== FSCTL_MANAGE_BYPASS_IO ================
//
//  This FSCTL is used to control BypassIO operations on a given file in the
//  filter and file system stacks
//

//
//  Defines the various operations supported by the BypassIO FSCTL
//

typedef enum _FS_BPIO_OPERATIONS {

    //
    //  This request can come from user or kernel mode.  This is a request to enable
    //  BypassIO for the given file which means a driver may not see all reads/writes
    //  for that file.
    //
    //  On the pre-operation, if a driver can support BypassIO for the given file
    //  it should forward the request down the stack.
    //
    //  On the pre-operation if a driver CAN NOT support BypassIO for the given
    //  file they should complete the FSCTL with STATUS_SUCCESS and should call
    //  FltVetoBypassIo to update the FS_BPIO_OUTPUT structure with appropriate
    //  OpStatus, FailingDriverName and FailureReason reasons.
    //
    //  During the post-operation they can see if all drivers below them are
    //  capable of supporting BypassIO.  If yes, the driver should preserve any
    //  needed state for the file and continue completion processing. It is the
    //  filter and file systems responsibility to maintain state to properly
    //  handle requests that may not be compatible with the BypassIO enabled state.
    //
    //  During the post-operation processing if a driver determines they can no
    //  longer support BypassIO, they can  call FltVetoBypassIo to inform the
    //  stack below them that BypassIO is now disabled.  They should set appropriate
    //  OpStatus, FailingDriverName and FailureReason reasons as to why it can not
    //  be supported.
    //

    FS_BPIO_OP_ENABLE = 1,

    //
    //  This request can come from user or kernel mode.  This informs filters and
    //  file systems that BypassIO is being disabled on this file.  It allows a
    //  driver to cleanup any associated BypassIO state.
    //
    //  If a driver has previously allowed BypassIO to be enabled on this file and
    //  now needs to turn off BypassIO support for a file (a good example is an encryption
    //  driver wanting to encrypt this file).  They should send this FSCTL to the top
    //  of the file system stack using the associated handle.
    //
    //  This can be received by a driver that currently does not have BypassIO enabled,
    //  it should be ignored.  If sent on a file that currently does not have BypassIO
    //  enabled, it should be ignored.
    //
    //  This operation should not be failed.
    //

    FS_BPIO_OP_DISABLE = 2,

    //
    //  This request can come from user or kernel mode.  This is an informational
    //  request to see if BypassIO can be enabled for the given file.  This
    //  should be processed the same as an ENABLE operation with the appropriate
    //  fields in the FS_BPIO_OUTPUT structure filled out.  The only difference
    //  is that the driver does not enter the ENABLE state.
    //

    FS_BPIO_OP_QUERY = 3,

    //
    //  This request can come from user or kernel mode.  This is a request to PAUSE
    //  BypassIO on this volume/storage stack.  All active BypassIO enabled files will
    //  stop doing storage stack level BypassIO operations.  This can be sent on a
    //  volume handle or any file handle for the given volume.
    //
    //  Bitlocker is an example of a component that would use this when it needs to
    //  enable encryption on a volume.  It is fine to send this even if BypassIO is
    //  not currently enabled on any files on the volume
    //
    //  This operation will not return until all active BypassIO files have transitioned
    //  to normal operating mode and all outstanding Bypass IO's have completed.
    //
    //  This operation should not be failed.
    //

    FS_BPIO_OP_VOLUME_STACK_PAUSE = 4,

    //
    //  This request can come from user or kernel mode.  This should be sent by a driver
    //  which has previously PAUSED BypassIO on a volume/storage stack and can
    //  now allow the stack to continue doing BypassIO.
    //
    //  Volsnap is an example of a component that would PAUSE BypassIO (via
    //  FS_BPIO_VOLUME_STACK_PAUSE) when a snapshot is created and would send this when
    //  the snapshot is deleted.
    //
    //  This operation should not be failed.
    //

    FS_BPIO_OP_VOLUME_STACK_RESUME = 5,

    //
    //  This request can come from user or kernel mode.  This is a request to PAUSE
    //  BypassIO on this stream.  All active BypassIO enabled files will stop doing
    //  BypassIO operations.  It is fine to send this even if BypassIO is
    //  not currently enabled on any files on the stream
    //
    //  This operation will not return until all active BypassIO's on this stream
    //  have completed and no new BypassIO operations are being generated.
    //
    //  This operation should not be failed.
    //

    FS_BPIO_OP_STREAM_PAUSE = 6,

    //
    //  This request can come from user or kernel mode.  This should be sent by a
    //  filter which has previously PAUSED BypassIO on this stream and can now
    //  allow them to be used again.
    //
    //  This operaiton should not be failed by filters.  The file system will
    //  determine if Bypass can be resumed on the stream by issuing a FS_BPIO_OP_QUERY
    //  to the top of the filter stack.
    //

    FS_BPIO_OP_STREAM_RESUME = 7,

    //
    //  This request can come from user or kernel mode.  This returns information
    //  about the BypassIO state of the volume.
    //

    FS_BPIO_OP_GET_INFO = 8,

    //
    //  All valid operations will be less than this value
    //

    FS_BPIO_OP_MAX_OPERATION

} FS_BPIO_OPERATIONS;

//
//  Defines the BypassIO INPUT flags
//

typedef enum _FS_BPIO_INFLAGS {

    FSBPIO_INFL_None = 0,

    //
    //  Only applies to QUERY operations. When set it will suppress sending the
    //  BypassIO IOCTL to the storage stack.  It will return results based only
    //  on the filter stack.  This has no effect on ENABLE operations.
    //

    FSBPIO_INFL_SKIP_STORAGE_STACK_QUERY = 1,

} FS_BPIO_INFLAGS;
DEFINE_ENUM_FLAG_OPERATORS( FS_BPIO_INFLAGS )

//
//  Defines the INPUT structure for FSCTL_MANAGE_BYPASS_IO
//

typedef struct _FS_BPIO_INPUT {

    //
    //  The BypassIO operation being requested
    //

    FS_BPIO_OPERATIONS Operation;

    //
    //  Input flags for this operation.
    //

    FS_BPIO_INFLAGS InFlags;

    //
    //  Reserved fields for future improvements, these fields MUST be set to zero
    //

    DWORDLONG Reserved1;
    DWORDLONG Reserved2;

} FS_BPIO_INPUT, *PFS_BPIO_INPUT;

//
//  Defines the BypassIO OUTPUT flags
//

typedef enum _FS_BPIO_OUTFLAGS {

    FSBPIO_OUTFL_None = 0,

    //
    //  When set it means BypassIO has been temporarily PAUSED for this volume
    //

    FSBPIO_OUTFL_VOLUME_STACK_BYPASS_PAUSED =   0x00000001,

    //
    //  When set it means BypassIO has been temporarily PAUSED for this Stream
    //

    FSBPIO_OUTFL_STREAM_BYPASS_PAUSED =         0x00000002,

    //
    //  When set, a minifilter has attached to this volume whose supported
    //  features state says it does not suport BypassIO.  All BypassIO is
    //  blocked on this volume.
    //

    FSBPIO_OUTFL_FILTER_ATTACH_BLOCKED =        0x00000004,

    //
    //  When set, the storage driver for this volume is BypassIO compatible.
    //  This flag is only defined for ENABLE/QUERY/GETINFO operations
    //

    FSBPIO_OUTFL_COMPATIBLE_STORAGE_DRIVER =    0x00000008,

} FS_BPIO_OUTFLAGS;
DEFINE_ENUM_FLAG_OPERATORS( FS_BPIO_OUTFLAGS )

//
//  This structure defines operation specific outputs for both the ENABLE
//  and QUERY operations
//

typedef struct _FS_BPIO_RESULTS {

    //
    //  A status code that will be available to users which identifies WHY
    //  the specified driver can't support BypassIO for this file.  This should
    //  only be set by the first driver to fail the enable/acquire request.
    //

    DWORD    OpStatus;

    //
    //  Receives the name of the driver that failed the request.  For diagnostic
    //  reasons it is required that drivers store their name when failing an
    //  ENABLE/ASK requests.  The name should match the actual name of the driver
    //  used by the system with extension.  Ex: "ntfs.sys".
    //
    //  FailingDriversNameLen contains the length of the string in CHARACTERS.
    //  No one should assume the string is NULL terminated.
    //

    WORD   FailingDriverNameLen;
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

    WORD   FailureReasonLen;
    WCHAR FailureReason[128];

} FS_BPIO_RESULTS, *PFS_BPIO_RESULTS;

//
//  This structure defines operation specific outputs for both the ENABLE
//  and QUERY operations
//

typedef struct _FS_BPIO_INFO {

    //
    //  Count of how many BypassIO enabled files are currently open
    //

    DWORD ActiveBypassIoCount;

    //
    //  Returns the name of the storage driver for this volume
    //

    WORD   StorageDriverNameLen;
    WCHAR StorageDriverName[32];

} FS_BPIO_INFO, *PFS_BPIO_INFO;

//
//  Defines the variable sized OUTPUT structure for FSCTL_MANAGE_BYPASS_IO
//

typedef struct _FS_BPIO_OUTPUT {

    //
    //  The BypassIO operation being requested.  This should be set to the
    //  same value passed in the INPUT structure
    //

    FS_BPIO_OPERATIONS Operation;

    //
    //  Output flags for this operation.
    //

    FS_BPIO_OUTFLAGS OutFlags;

    //
    //  Reserved fields used for future improvements
    //

    DWORDLONG Reserved1;
    DWORDLONG Reserved2;

    //
    //  Operation specific outputs
    //

    union {
        FS_BPIO_RESULTS Enable;
        FS_BPIO_RESULTS Query;
        FS_BPIO_RESULTS VolumeStackResume;
        FS_BPIO_RESULTS StreamResume;
        FS_BPIO_INFO GetInfo;
    };

} FS_BPIO_OUTPUT, *PFS_BPIO_OUTPUT;

//
//  Structure size based on operations type
//

#define FS_BPIO_OUTPUT_ENABLE_SIZE                  (RTL_SIZEOF_THROUGH_FIELD(FS_BPIO_OUTPUT,Enable))
#define FS_BPIO_OUTPUT_QUERY_SIZE                   (RTL_SIZEOF_THROUGH_FIELD(FS_BPIO_OUTPUT,Query))
#define FS_BPIO_OUTPUT_DISABLE_SIZE                 ((size_t)FIELD_OFFSET(FS_BPIO_OUTPUT, Enable))
#define FS_BPIO_OUTPUT_VOLUME_STACK_PAUSE_SIZE      ((size_t)FIELD_OFFSET(FS_BPIO_OUTPUT, Enable))
#define FS_BPIO_OUTPUT_VOLUME_STACK_RESUME_SIZE     (RTL_SIZEOF_THROUGH_FIELD(FS_BPIO_OUTPUT,VolumeStackResume))
#define FS_BPIO_OUTPUT_STREAM_PAUSE_SIZE            ((size_t)FIELD_OFFSET(FS_BPIO_OUTPUT, Enable))
#define FS_BPIO_OUTPUT_STREAM_RESUME_SIZE           (RTL_SIZEOF_THROUGH_FIELD(FS_BPIO_OUTPUT,StreamResume))
#define FS_BPIO_OUTPUT_GET_INFO_SIZE                (RTL_SIZEOF_THROUGH_FIELD(FS_BPIO_OUTPUT,GetInfo))

#pragma warning(pop)

#endif // (NTDDI_VERSION >= NTDDI_WIN10_CO)

//
// ****************** Insert New FSCTLs above Here ****************************
//

#endif // _FILESYSTEMFSCTL_


//
//=============== END FileSystem FSCTL Structure Definitions ==================
//

    

//
// Some applications include both ntioapi_x.h and winioctl.h
//

#ifndef SMB_CCF_APP_INSTANCE_EA_NAME
#define SMB_CCF_APP_INSTANCE_EA_NAME   "ClusteredApplicationInstance"
#endif //SMB_CCF_APP_INSTANCE_EA_NAME

#ifndef _NETWORK_APP_INSTANCE_EA_DEFINED
#define _NETWORK_APP_INSTANCE_EA_DEFINED

#if (NTDDI_VERSION >= NTDDI_WIN10)

//
// Define the SMB Cluster Client Failover AppInstance Extended Attribute name
// newer version of input payload assumes that EA is not just a GUID,
// but instead is a structure that contains additional information
//

//
// Is used only when file is opened directly on CSVFS. This flag is ignored when file
// is opened over SMB.
// Tells CSVFS that this file open should be valid only on coordinating node.
// If open comes to CSVFS, and this node is not a coordinating then open would fail.
// If file is opened, and coordinating node is moved then file open will be invalidated
//
#ifndef NETWORK_APP_INSTANCE_CSV_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR
#define NETWORK_APP_INSTANCE_CSV_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR 0x00000001
#endif //NETWORK_APP_INSTANCE_CSV_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR

typedef struct _NETWORK_APP_INSTANCE_EA {

    //
    //  The caller places a GUID that should always be unique for a single instance of
    //  the application.
    //

    GUID AppInstanceID;

    //
    //  Combination of the NETWORK_APP_INSTANCE_CSV_FLAGS_* flags
    //

    DWORD CsvFlags;

} NETWORK_APP_INSTANCE_EA, *PNETWORK_APP_INSTANCE_EA;

#endif // (NTDDI_VERSION >= NTDDI_WIN10)

#endif //_NETWORK_APP_INSTANCE_EA_DEFINED


#if (NTDDI_VERSION >= NTDDI_WIN10_MN)

//
//=============== FSCTL_SMB_SHARE_FLUSH_AND_PURGE =================
//

#ifndef SMB_SHARE_FLUSH_AND_PURGE_INPUT_DESCRIPTORS_DEFINED
#define SMB_SHARE_FLUSH_AND_PURGE_INPUT_DESCRIPTORS_DEFINED

typedef struct _SMB_SHARE_FLUSH_AND_PURGE_INPUT {

    WORD   Version;             // sizeof(SMB_SHARE_FLUSH_AND_PURGE)
} SMB_SHARE_FLUSH_AND_PURGE_INPUT, *PSMB_SHARE_FLUSH_AND_PURGE_INPUT;
typedef struct _SMB_SHARE_FLUSH_AND_PURGE_INPUT const *PCSMB_SHARE_FLUSH_AND_PURGE_INPUT;

typedef struct _SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {

    DWORD cEntriesPurged;
} SMB_SHARE_FLUSH_AND_PURGE_OUTPUT, *PSMB_SHARE_FLUSH_AND_PURGE_OUTPUT;
typedef struct _SMB_SHARE_FLUSH_AND_PURGE_OUTPUT const *PCSMB_SHARE_FLUSH_AND_PURGE_OUTPUT;

#endif // defined(SMB_SHARE_FLUSH_AND_PURGE_INPUT_DESCRIPTORS_DEFINED)
#endif // (NTDDI_VERSION >= NTDDI_WIN10_MN)

#define IOCTL_VOLUME_BASE                       0x00000056 // 'V'

#if (NTDDI_VERSION >= NTDDI_WIN2K)

//
// IOCTL to obtain the physical location of
// the specified volume on one or more disks.
//

#define IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS    CTL_CODE(IOCTL_VOLUME_BASE, 0, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS
//
// Input Buffer:
//     None
//
// Output Buffer:
//     Structure of type VOLUME_DISK_EXTENTS
//

typedef struct _DISK_EXTENT {

    //
    // Specifies the storage device number of
    // the disk on which this extent resides.
    //
    DWORD DiskNumber;

    //
    // Specifies the offset and length of this
    // extent relative to the beginning of the
    // disk.
    //
    LARGE_INTEGER StartingOffset;
    LARGE_INTEGER ExtentLength;

} DISK_EXTENT, *PDISK_EXTENT;

typedef struct _VOLUME_DISK_EXTENTS {

    //
    // Specifies one or more contiguous range
    // of sectors that make up this volume.
    //
    DWORD NumberOfDiskExtents;
    DISK_EXTENT Extents[ANYSIZE_ARRAY];

} VOLUME_DISK_EXTENTS, *PVOLUME_DISK_EXTENTS;


//
// IOCTLs to transition the specified volume
// between r/w and non r/w modes.
//

#define IOCTL_VOLUME_ONLINE                     CTL_CODE(IOCTL_VOLUME_BASE, 2, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_VOLUME_OFFLINE                    CTL_CODE(IOCTL_VOLUME_BASE, 3, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// IOCTL_VOLUME_ONLINE
// IOCTL_VOLUME_OFFLINE
//
// Input Buffer:
//     None
//
// Output Buffer:
//     None
//

#endif  // NTDDI_VERSION >= NTDDI_WIN2K


#if (NTDDI_VERSION >= NTDDI_WINXP)

//
// IOCTL to determine  whether  the specified
// volume resides on a disk that is an online
// cluster resource or not.
//

#define IOCTL_VOLUME_IS_CLUSTERED               CTL_CODE(IOCTL_VOLUME_BASE, 12, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// IOCTL_VOLUME_IS_CLUSTERED
//
// Input Buffer:
//     None
//
// Output Buffer:
//     None
//

//
// IOCTL to query the attributes on volumes.
//

#define IOCTL_VOLUME_GET_GPT_ATTRIBUTES         CTL_CODE(IOCTL_VOLUME_BASE, 14, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// IOCTL_VOLUME_GET_GPT_ATTRIBUTES
//
// Input Buffer:
//     None
//
// Output Buffer:
//     Structure of type VOLUME_GET_GPT_ATTRIBUTES_INFORMATION
//

typedef struct _VOLUME_GET_GPT_ATTRIBUTES_INFORMATION {

    //
    // Specifies all the attributes
    // associated with this volume.
    //
    DWORDLONG GptAttributes;

} VOLUME_GET_GPT_ATTRIBUTES_INFORMATION, *PVOLUME_GET_GPT_ATTRIBUTES_INFORMATION;

#endif  // NTDDI_VERSION >= NTDDI_WINXP

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

//
//  This is a function prototype for a routine that will be called from the
//  IoPropagateIrpExtensionEx routine whenever the IopFsTrackOffsetType
//  extension type is set for a given IRP
//

#ifndef _IO_IRP_EXT_TRACK_OFFSET_HEADER_
#define _IO_IRP_EXT_TRACK_OFFSET_HEADER_

struct _IO_IRP_EXT_TRACK_OFFSET_HEADER;

typedef VOID
(*PIO_IRP_EXT_PROCESS_TRACKED_OFFSET_CALLBACK)(
    _In_ struct _IO_IRP_EXT_TRACK_OFFSET_HEADER *SourceContext,
    _Inout_ struct _IO_IRP_EXT_TRACK_OFFSET_HEADER *TargetContext,
    _In_ LONGLONG RelativeOffset
    );

//
//  When any IRP extension exists which has an OFFSET which needs processing,
//  the Irp extension field in the IRP must point to one of these structures.
//  This is so IoPropagateIrpExtensionEx can calculate proper file offset
//  adjustments for the sub IRPS as they are split and shifted
//

#define IRP_EXT_TRACK_OFFSET_HEADER_VALIDATION_VALUE 'TO'    //Track Offset

typedef struct _IO_IRP_EXT_TRACK_OFFSET_HEADER {

    WORD   Validation;

    //  Each consumer of this IRP extension must define a flag that helps
    //  identify the usage of the header. eg: EFS_TRACKED_OFFSET_HEADER_FLAG
    WORD   Flags;

    PIO_IRP_EXT_PROCESS_TRACKED_OFFSET_CALLBACK TrackedOffsetCallback;

} IO_IRP_EXT_TRACK_OFFSET_HEADER, *PIO_IRP_EXT_TRACK_OFFSET_HEADER;

//
//  When using the TrackedOffset IRP extension one of the following
//  flag must be specified so that we can distinguish it from other
//  components setting this IRP extension
//

#define EFS_TRACKED_OFFSET_HEADER_FLAG 0x0001 //EFS Flag
#define SPACES_TRACKED_OFFSET_HEADER_FLAG 0x0002 //SPACES Flag

#endif // _IO_IRP_EXT_TRACK_OFFSET_HEADER_
#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)


#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif // _WINIOCTL_

