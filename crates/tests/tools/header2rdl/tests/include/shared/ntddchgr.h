/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ntddchgr.h

Abstract:

    This is the include file that defines all constants and types for
    accessing medium changer devices.

--*/

#ifndef _NTDDCHGR_H_
#define _NTDDCHGR_H_

#include <winapifamily.h>

#if _MSC_VER > 1000
#pragma once
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

//
// Device Name - this string is the name of the device.  It is the name
// that should be passed to NtOpenFile when accessing the device.
//
// Note:  For devices that support multiple units, it should be suffixed
//        with the Ascii representation of the unit number.
//

#define DD_CHANGER_DEVICE_NAME "\\Device\\Changer"

//
// NtDeviceIoControlFile IoControlCode values for changer devices.
//


// begin_winioctl

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


// end_winioctl

//
// The following file contains the IOCTL_STORAGE class ioctls
//

#include <ntddstor.h>

#ifdef __cplusplus
extern "C" {
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

// begin_winioctl

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
    ULONG   ElementAddress;
} CHANGER_ELEMENT, *PCHANGER_ELEMENT;

typedef  struct _CHANGER_ELEMENT_LIST {
    CHANGER_ELEMENT Element;
    ULONG   NumberOfElements;
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

    ULONG Size;

    //
    // Number of N element(s) as defined by the Element Address Page (or equivalent...).
    //

    USHORT NumberTransportElements;
    USHORT NumberStorageElements;                // for data cartridges only
    USHORT NumberCleanerSlots;                   // for cleaner cartridges
    USHORT NumberIEElements;
    USHORT NumberDataTransferElements;

    //
    // Number of doors/front panels (allows user entry into the cabinet).
    //

    USHORT NumberOfDoors;

    //
    // The device-specific address (from user manual of the device) of the first N element. Used
    // by the UI to relate the various elements to the user.
    //

    USHORT FirstSlotNumber;
    USHORT FirstDriveNumber;
    USHORT FirstTransportNumber;
    USHORT FirstIEPortNumber;
    USHORT FirstCleanerSlotAddress;

    //
    // Indicates the capacity of each magazine, if they exist.
    //

    USHORT MagazineSize;

    //
    // Specifies the approximate number of seconds for when a cleaning should be completed.
    // Only applicable if drive cleaning is supported. See Features0.
    //

    ULONG DriveCleanTimeout;

    //
    // See features bits, above.
    //

    ULONG Features0;
    ULONG Features1;

    //
    // Bitmask defining Move from N element to element. Defined by Device Capabilities Page (or equivalent).
    // AND-masking with the TO_XXX values will indicate legal destinations.
    //

    UCHAR MoveFromTransport;
    UCHAR MoveFromSlot;
    UCHAR MoveFromIePort;
    UCHAR MoveFromDrive;

    //
    // Bitmask defining Exchange from N element to element. Defined by Device Capabilities Page (or equivalent).
    // AND-masking with the TO_XXX values will indicate legal destinations.
    //

    UCHAR ExchangeFromTransport;
    UCHAR ExchangeFromSlot;
    UCHAR ExchangeFromIePort;
    UCHAR ExchangeFromDrive;

    //
    // Bitmask defining which elements are capable of lock/unlock. Valid only if
    // CHANGER_LOCK_UNLOCK is set in Features0.
    //

    UCHAR LockUnlockCapabilities;

    //
    // Bitmask defining which elements valid for positioning operations. Valid only if
    // CHANGER_POSITION_TO_ELEMENT is set in Features0.
    //

    UCHAR PositionCapabilities;

    //
    // For future expansion.
    //

    UCHAR Reserved1[2];
    ULONG Reserved2[2];

} GET_CHANGER_PARAMETERS, * PGET_CHANGER_PARAMETERS;


//
// Definitions for IOCTL_CHANGER_GET_PRODUCT_DATA
//

typedef  struct _CHANGER_PRODUCT_DATA {

    //
    // Device manufacturer's name - based on inquiry data
    //

    UCHAR VendorId[VENDOR_ID_LENGTH];

    //
    // Product identification as defined by the vendor - based on Inquiry data
    //

    UCHAR ProductId[PRODUCT_ID_LENGTH];

    //
    // Product revision as defined by the vendor.
    //

    UCHAR Revision[REVISION_LENGTH];

    //
    // Vendor unique value used to globally identify this device. Can
    // be from Vital Product Data, for example.
    //

    UCHAR SerialNumber[SERIAL_NUMBER_LENGTH];

    //
    // Indicates device type of data transports, as defined by SCSI-2.
    //

    UCHAR DeviceType;

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

    ULONG           Control;
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
    // Valid if ELEMENT_STATUS_SVALID bit of Flags ULONG is set.
    // Needs to be converted to a zero-based offset from the device-unique value.
    //

    CHANGER_ELEMENT SrcElementAddress;

    //
    // See below.
    //

    ULONG Flags;

    //
    // See below for possible values.
    //

    ULONG ExceptionCode;

    //
    // Scsi Target Id of this element.
    // Valid only if ELEMENT_STATUS_ID_VALID is set in Flags.
    //

    UCHAR TargetId;

    //
    // LogicalUnitNumber of this element.
    // Valid only if ELEMENT_STATUS_LUN_VALID is set in Flags.
    //

    UCHAR Lun;
    USHORT Reserved;

    //
    // Primary volume identification for the media.
    // Valid only if ELEMENT_STATUS_PVOLTAG bit is set in Flags.
    //

    UCHAR PrimaryVolumeID[MAX_VOLUME_ID_SIZE];

    //
    // Alternate volume identification for the media.
    // Valid for two-sided media only, and pertains to the id. of the inverted side.
    // Valid only if ELEMENT_STATUS_AVOLTAG bit is set in Flags.
    //

    UCHAR AlternateVolumeID[MAX_VOLUME_ID_SIZE];

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
    // Valid if ELEMENT_STATUS_SVALID bit of Flags ULONG is set.
    // Needs to be converted to a zero-based offset from the device-unique value.
    //

    CHANGER_ELEMENT SrcElementAddress;

    //
    // See below.
    //

    ULONG Flags;

    //
    // See below for possible values.
    //

    ULONG ExceptionCode;

    //
    // Scsi Target Id of this element.
    // Valid only if ELEMENT_STATUS_ID_VALID is set in Flags.
    //

    UCHAR TargetId;

    //
    // LogicalUnitNumber of this element.
    // Valid only if ELEMENT_STATUS_LUN_VALID is set in Flags.
    //

    UCHAR Lun;
    USHORT Reserved;

    //
    // Primary volume identification for the media.
    // Valid only if ELEMENT_STATUS_PVOLTAG bit is set in Flags.
    //

    UCHAR PrimaryVolumeID[MAX_VOLUME_ID_SIZE];

    //
    // Alternate volume identification for the media.
    // Valid for two-sided media only, and pertains to the id. of the inverted side.
    // Valid only if ELEMENT_STATUS_AVOLTAG bit is set in Flags.
    //

    UCHAR AlternateVolumeID[MAX_VOLUME_ID_SIZE];

    //
    // Vendor ID
    //
    UCHAR VendorIdentification[VENDOR_ID_LENGTH];

    //
    // Product ID
    //
    UCHAR ProductIdentification[PRODUCT_ID_LENGTH];

    //
    // Serial number
    //
    UCHAR SerialNumber[SERIAL_NUMBER_LENGTH];

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

    ULONG ActionCode;

    //
    // Template used by the device to search for volume ids.
    //

    UCHAR VolumeIDTemplate[MAX_VOLUME_TEMPLATE_SIZE];
} CHANGER_SEND_VOLUME_TAG_INFORMATION, *PCHANGER_SEND_VOLUME_TAG_INFORMATION;


//
// Output buffer.
//

typedef struct _READ_ELEMENT_ADDRESS_INFO {

    //
    // Number of elements matching criteria set forth by ActionCode.
    //

    ULONG NumberOfElements;

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

// end_winioctl

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif // _NTDDCHGR_H_

