/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ioevent.h

Abstract:

    This module contains the GUIDS and event structures for io system
    initiated events.  These events are reported in kernel mode and are
    available to both user mode and kernel mode clients.


--*/
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


//
//  Label change event.  This event is signalled upon successful completion
//  of a label change.  There is no additional data.
//

DEFINE_GUID( GUID_IO_VOLUME_CHANGE, 0x7373654aL, 0x812a, 0x11d0, 0xbe, 0xc7, 0x08, 0x00, 0x2b, 0xe2, 0x09, 0x2f );

//
//  Volume dismount event.  This event is signalled when an attempt is made to
//  dismount a volume.  There is no additional data.  Note that this will not
//  necessarily be preceded by a GUID_IO_VOLUME_LOCK notification.
//

DEFINE_GUID( GUID_IO_VOLUME_DISMOUNT, 0xd16a55e8L, 0x1059, 0x11d2, 0x8f, 0xfd, 0x00, 0xa0, 0xc9, 0xa0, 0x6d, 0x32 );

//
//  Volume dismount failed event.  This event is signalled when a volume dismount fails.
//  There is no additional data.
//

DEFINE_GUID( GUID_IO_VOLUME_DISMOUNT_FAILED, 0xe3c5b178L, 0x105d, 0x11d2, 0x8f, 0xfd, 0x00, 0xa0, 0xc9, 0xa0, 0x6d, 0x32 );

//
//  Volume mount event.  This event is signalled when a volume mount occurs.
//  There is no additional data.
//

DEFINE_GUID( GUID_IO_VOLUME_MOUNT, 0xb5804878L, 0x1a96, 0x11d2, 0x8f, 0xfd, 0x00, 0xa0, 0xc9, 0xa0, 0x6d, 0x32 );

typedef struct _DEVICE_EVENT_MOUNT {
    ULONG Version;
    ULONG Flags;
    ULONG FileSystemNameLength;
    ULONG FileSystemNameOffset;
} DEVICE_EVENT_MOUNT, *PDEVICE_EVENT_MOUNT;

//
//  Volume lock event.  This event is signalled when an attempt is made to
//  lock a volume.  There is no additional data.
//

DEFINE_GUID( GUID_IO_VOLUME_LOCK, 0x50708874L, 0xc9af, 0x11d1, 0x8f, 0xef, 0x00, 0xa0, 0xc9, 0xa0, 0x6d, 0x32 );

//
//  Volume lock failed event.  This event is signalled when an attempt is made to
//  lock a volume, but it fails.  There is no additional data.
//

DEFINE_GUID( GUID_IO_VOLUME_LOCK_FAILED, 0xae2eed10L, 0x0ba8, 0x11d2, 0x8f, 0xfb, 0x00, 0xa0, 0xc9, 0xa0, 0x6d, 0x32 );


//
//  Volume unlock event.  This event is signalled when an attempt is made to
//  unlock a volume.  There is no additional data.
//

DEFINE_GUID( GUID_IO_VOLUME_UNLOCK, 0x9a8c3d68L, 0xd0cb, 0x11d1, 0x8f, 0xef, 0x00, 0xa0, 0xc9, 0xa0, 0x6d, 0x32 );


//
//  Volume name change.  This event is signalled when the list of persistent
//  names (like drive letters) for a volume changes.  There is no additional
//  data.
//

DEFINE_GUID( GUID_IO_VOLUME_NAME_CHANGE, 0x2de97f83, 0x4c06, 0x11d2, 0xa5, 0x32, 0x0, 0x60, 0x97, 0x13, 0x5, 0x5a);


//
//      Volume needs chkdsk event. Sent when a file system detects corruption.
//

DEFINE_GUID( GUID_IO_VOLUME_NEED_CHKDSK, 0x799a0960, 0x0a0b, 0x4e03, 0xad, 0x88, 0x2f, 0xa7, 0xc6, 0xce, 0x74, 0x8a);


//
//  WORK near full event. Send when a wolume with write-once-read-many characteristics
//  (e.g. CD-R) is becoming full (on these media modifying existing file data
//  consumes space) so that the user can be notified.
//

DEFINE_GUID( GUID_IO_VOLUME_WORM_NEAR_FULL, 0xf3bfff82, 0xf3de, 0x48d2, 0xaf, 0x95, 0x45, 0x7f, 0x80, 0xb7, 0x63, 0xf2);


//
//  Media wearing out. Sent when a file sytem determines that the error rate
//  on a volume is too high, or sparing (defect replacement) space is almost
//  exhausted.
//

DEFINE_GUID( GUID_IO_VOLUME_WEARING_OUT, 0x873113ca, 0x1486, 0x4508, 0x82, 0xac, 0xc3, 0xb2, 0xe5, 0x29, 0x7a, 0xaa);


//
//  Volume force closed event. Sent when a volume has been finalised and made
//  read-only by the filesystem due to (e,g.) WORM type volume full, or sparing
//  (defect replacement) space has been exhausted.
//

DEFINE_GUID( GUID_IO_VOLUME_FORCE_CLOSED, 0x411ad84f, 0x433e, 0x4dc2, 0xa5, 0xae, 0x4a, 0x2d, 0x1a, 0x2d, 0xe6, 0x54);


//
//  Notify make compatible function available. Sent after the user removes a disc
//  which may not be readable in all drives in it's current state (e.g. CD-R with
//  open session).
//

DEFINE_GUID( GUID_IO_VOLUME_INFO_MAKE_COMPAT, 0x3ab9a0d2, 0xef80, 0x45cf, 0x8c, 0xdc, 0xcb, 0xe0, 0x2a, 0x21, 0x29, 0x06);


//
//  Notify that the drive is preparing the disc for eject (e.g. stopping a background format).
//

DEFINE_GUID( GUID_IO_VOLUME_PREPARING_EJECT, 0xc79eb16e, 0x0dac, 0x4e7a, 0xa8, 0x6c, 0xb2, 0x5c, 0xee, 0xaa, 0x88, 0xf6);


//
//  Notify that a background format has been initiated on the disc.
//

DEFINE_GUID( GUID_IO_VOLUME_BACKGROUND_FORMAT, 0xa2e5fc86, 0xd5cd, 0x4038, 0xb2, 0xe3, 0x44, 0x45, 0x6, 0x5c, 0x23, 0x77);

//
//  Volume physical configuration change.  This event is signalled when the
//  physical makeup or current physical state of the volume changes.
//

DEFINE_GUID( GUID_IO_VOLUME_PHYSICAL_CONFIGURATION_CHANGE, 0x2de97f84, 0x4c06, 0x11d2, 0xa5, 0x32, 0x0, 0x60, 0x97, 0x13, 0x5, 0x5a);

//
//  Volume being deleted.  This event is signalled when the
//  volume is in the process of being deleted.
//

DEFINE_GUID( GUID_IO_VOLUME_PREPARE_DELETE, 0xac0707fb, 0x4a9a, 0x4c81, 0x9e, 0x2e, 0x38, 0x5b, 0x79, 0xa8, 0xfd, 0x28);

//
//  Volume unique ID change.  This event is signalled when the
//  unique ID of the volume changes.
//

DEFINE_GUID( GUID_IO_VOLUME_UNIQUE_ID_CHANGE, 0xaf39da42, 0x6622, 0x41f5, 0x97, 0xb, 0x13, 0x9d, 0x9, 0x2f, 0xa3, 0xd9);

//
//  Volume BitLocker Drive Encryption status change.
//  This event is signalled when BDE is enabled / disabled, or when encryption
//  begins, ends, pauses or resumes.
//

DEFINE_GUID( GUID_IO_VOLUME_FVE_STATUS_CHANGE, 0x062998b2, 0xee1f, 0x4b6a, 0xb8, 0x57, 0xe7, 0x6c, 0xbb, 0xe9, 0xa6, 0xda);


//
//  Volume device interface.  This is a device interface GUID that appears
//  when the device object associated with a volume is created and disappears
//  when the device object associated with the volume is destroyed.
//

DEFINE_GUID( GUID_IO_VOLUME_DEVICE_INTERFACE, 0x53f5630d, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);

//
// The size of the filesystem on the volume has changed.
//

DEFINE_GUID( GUID_IO_VOLUME_CHANGE_SIZE, 0x3a1625be, 0xad03, 0x49f1, 0x8e, 0xf8, 0x6b, 0xba, 0xc1, 0x82, 0xd1, 0xfd);

//
//  Sent when the removable media is changed (added, removed) from a device
//  (such as a CDROM, tape, changer, etc).
//
//  The additional data is a DWORD representing the data event.
//

DEFINE_GUID( GUID_IO_MEDIA_ARRIVAL,         0xd07433c0, 0xa98e, 0x11d2, 0x91, 0x7a, 0x00, 0xa0, 0xc9, 0x06, 0x8f, 0xf3);
DEFINE_GUID( GUID_IO_MEDIA_REMOVAL,         0xd07433c1, 0xa98e, 0x11d2, 0x91, 0x7a, 0x00, 0xa0, 0xc9, 0x06, 0x8f, 0xf3);

//
// Sent when the CDROM device locked/unlocked for exclusive access
//

DEFINE_GUID(GUID_IO_CDROM_EXCLUSIVE_LOCK,   0xbc56c139, 0x7a10, 0x47ee, 0xa2, 0x94, 0x4c, 0x6a, 0x38, 0xf0, 0x14, 0x9a);
DEFINE_GUID(GUID_IO_CDROM_EXCLUSIVE_UNLOCK, 0xa3b6d27d, 0x5e35, 0x4885, 0x81, 0xe5, 0xee, 0x18, 0xc0, 0xe, 0xd7, 0x79);

//
// Sent when the media is returning that it is not ready right now, but will
// be ready soon. This can be because the drive has spun down to save power
// or because new media has been inserted but is not ready for access yet.
//

DEFINE_GUID( GUID_IO_DEVICE_BECOMING_READY, 0xd07433f0, 0xa98e, 0x11d2, 0x91, 0x7a, 0x00, 0xa0, 0xc9, 0x06, 0x8f, 0xf3);

typedef struct _DEVICE_EVENT_BECOMING_READY {
    ULONG Version;
    ULONG Reason;
    ULONG Estimated100msToReady;
} DEVICE_EVENT_BECOMING_READY, *PDEVICE_EVENT_BECOMING_READY;

//
// Sent when the user presses the eject button on the front of the drive,
// or when other buttons on the front are pressed via GESN command polling
// (GESN support to be added)
//

DEFINE_GUID( GUID_IO_DEVICE_EXTERNAL_REQUEST, 0xd07433d0, 0xa98e, 0x11d2, 0x91, 0x7a, 0x00, 0xa0, 0xc9, 0x06, 0x8f, 0xf3);
DEFINE_GUID( GUID_IO_MEDIA_EJECT_REQUEST,     0xd07433d1, 0xa98e, 0x11d2, 0x91, 0x7a, 0x00, 0xa0, 0xc9, 0x06, 0x8f, 0xf3);

typedef struct _DEVICE_EVENT_EXTERNAL_REQUEST {
    ULONG  Version;
    ULONG  DeviceClass;       // 0 == MMC Storage Devices
    USHORT ButtonStatus;      // 1 == down, 2 == up
    USHORT Request;
    LARGE_INTEGER SystemTime; // for time-related info
} DEVICE_EVENT_EXTERNAL_REQUEST, *PDEVICE_EVENT_EXTERNAL_REQUEST;

//
// Sent when a tape drive requires cleaning
//
DEFINE_GUID(GUID_IO_DRIVE_REQUIRES_CLEANING, 0x7207877c, 0x90ed, 0x44e5, 0xa0, 0x0, 0x81, 0x42, 0x8d, 0x4c, 0x79, 0xbb);

//
// Sent when a tape is erased
//
DEFINE_GUID(GUID_IO_TAPE_ERASE, 0x852d11eb, 0x4bb8, 0x4507, 0x9d, 0x9b, 0x41, 0x7c, 0xc2, 0xb1, 0xb4, 0x38);

typedef struct _DEVICE_EVENT_GENERIC_DATA {
    ULONG EventNumber;
} DEVICE_EVENT_GENERIC_DATA, *PDEVICE_EVENT_GENERIC_DATA;


//
//  Represents any asynchronous notification coming from a device driver whose
//  notification protocol is RBC
//  Additional data is provided

DEFINE_GUID( GUID_DEVICE_EVENT_RBC, 0xd0744792, 0xa98e, 0x11d2, 0x91, 0x7a, 0x00, 0xa0, 0xc9, 0x06, 0x8f, 0xf3);

typedef struct _DEVICE_EVENT_RBC_DATA {
    ULONG EventNumber;
    UCHAR SenseQualifier;
    UCHAR SenseCode;
    UCHAR SenseKey;
    UCHAR Reserved;
    ULONG Information;
} DEVICE_EVENT_RBC_DATA, *PDEVICE_EVENT_RBC_DATA;

//
//  A clone of this disk has just arrived in the system.
//

DEFINE_GUID( GUID_IO_DISK_CLONE_ARRIVAL, 0x6a61885b, 0x7c39, 0x43dd, 0x9b, 0x56, 0xb8, 0xac, 0x22, 0xa5, 0x49, 0xaa);

typedef struct _GUID_IO_DISK_CLONE_ARRIVAL_INFORMATION {
    ULONG DiskNumber;   // The disk number of the new disk arriving in the system.
} GUID_IO_DISK_CLONE_ARRIVAL_INFORMATION, *PGUID_IO_DISK_CLONE_ARRIVAL_INFORMATION;

//
// The disk layout has changed
//

DEFINE_GUID( GUID_IO_DISK_LAYOUT_CHANGE, 0x11dff54c, 0x8469, 0x41f9, 0xb3, 0xde, 0xef, 0x83, 0x64, 0x87, 0xc5, 0x4a);

//
// This notification may be sent by storage drivers to
// alert other components of changes in the health status
// of a device. A component that registers for this
// custom PNP notification should query for the
// StorageDeviceManagementStatus property to learn
// more about the device's health status.
//

// {0F1BD644-3916-49C5-B063-991940118FB2}
DEFINE_GUID( GUID_IO_DISK_HEALTH_NOTIFICATION, 0xf1bd644, 0x3916, 0x49c5, 0xb0, 0x63, 0x99, 0x19, 0x40, 0x11, 0x8f, 0xb2);

typedef struct _DISK_HEALTH_NOTIFICATION_DATA {

    //
    // The GUID of the device reporting the health change.
    // This is the same GUID returned by IOCTL_STORAGE_GET_DEVICE_NUMBER_EX.
    //
    GUID  DeviceGuid;

} DISK_HEALTH_NOTIFICATION_DATA, *PDISK_HEALTH_NOTIFICATION_DATA;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion
