/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ntddmou.h

Abstract:

    This is the include file that defines all constants and types for
    accessing the mouse device.

--*/

#ifndef _NTDDMOU_
#define _NTDDMOU_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#ifdef __cplusplus
extern "C" {
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

//
// Device Name - this string is the name of the device.  It is the name
// that should be passed to NtOpenFile when accessing the device.
//
// Note:  For devices that support multiple units, it should be suffixed
//        with the Ascii representation of the unit number.
//

#define DD_MOUSE_DEVICE_NAME    "\\Device\\PointerClass"
#define DD_MOUSE_DEVICE_NAME_U L"\\Device\\PointerClass"

//
// NtDeviceIoControlFile IoControlCode values for this device.
//
// Warning:  Remember that the low two bits of the code specify how the
//           buffers are passed to the driver!
//

#define IOCTL_MOUSE_QUERY_ATTRIBUTES CTL_CODE(FILE_DEVICE_MOUSE, 0, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_MOUSE_INSERT_DATA      CTL_CODE(FILE_DEVICE_MOUSE, 1, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// Declare the GUID that represents the device interface for mice.
//

#ifndef FAR
#define FAR
#endif

DEFINE_GUID( GUID_DEVINTERFACE_MOUSE, 0x378de44c, 0x56ef, 0x11d1,
             0xbc, 0x8c, 0x00, 0xa0, 0xc9, 0x14, 0x05, 0xdd );

//
// Obsolete device interface class GUID name.
// (use of above GUID_DEVINTERFACE_* name is recommended).
//

#define GUID_CLASS_MOUSE  GUID_DEVINTERFACE_MOUSE

//
// NtReadFile Output Buffer record structures for this device.
//

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4201) // nonstandard extension used : nameless struct/union
#endif

typedef struct _MOUSE_INPUT_DATA {

    //
    // Unit number.  E.g., for \Device\PointerPort0  the unit is '0',
    // for \Device\PointerPort1 the unit is '1', and so on.
    //

    USHORT UnitId;

    //
    // Indicator flags.
    //

    USHORT Flags;

    //
    // The transition state of the mouse buttons.
    //

    union {
        ULONG Buttons;
        struct  {
            USHORT  ButtonFlags;
            USHORT  ButtonData;
        };
    };


    //
    // The raw state of the mouse buttons.
    //

    ULONG RawButtons;

    //
    // The signed relative or absolute motion in the X direction.
    //

    LONG LastX;

    //
    // The signed relative or absolute motion in the Y direction.
    //

    LONG LastY;

    //
    // Device-specific additional information for the event.
    //

    ULONG ExtraInformation;

} MOUSE_INPUT_DATA, *PMOUSE_INPUT_DATA;

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

//
// Define the mouse button state indicators.
//

#define MOUSE_LEFT_BUTTON_DOWN   0x0001  // Left Button changed to down.
#define MOUSE_LEFT_BUTTON_UP     0x0002  // Left Button changed to up.
#define MOUSE_RIGHT_BUTTON_DOWN  0x0004  // Right Button changed to down.
#define MOUSE_RIGHT_BUTTON_UP    0x0008  // Right Button changed to up.
#define MOUSE_MIDDLE_BUTTON_DOWN 0x0010  // Middle Button changed to down.
#define MOUSE_MIDDLE_BUTTON_UP   0x0020  // Middle Button changed to up.

#define MOUSE_BUTTON_1_DOWN     MOUSE_LEFT_BUTTON_DOWN
#define MOUSE_BUTTON_1_UP       MOUSE_LEFT_BUTTON_UP
#define MOUSE_BUTTON_2_DOWN     MOUSE_RIGHT_BUTTON_DOWN
#define MOUSE_BUTTON_2_UP       MOUSE_RIGHT_BUTTON_UP
#define MOUSE_BUTTON_3_DOWN     MOUSE_MIDDLE_BUTTON_DOWN
#define MOUSE_BUTTON_3_UP       MOUSE_MIDDLE_BUTTON_UP

#define MOUSE_BUTTON_4_DOWN     0x0040
#define MOUSE_BUTTON_4_UP       0x0080
#define MOUSE_BUTTON_5_DOWN     0x0100
#define MOUSE_BUTTON_5_UP       0x0200

#define MOUSE_WHEEL             0x0400
#define MOUSE_HWHEEL		0x0800

//
// Define the mouse indicator flags.
//

#define MOUSE_MOVE_RELATIVE         0
#define MOUSE_MOVE_ABSOLUTE         1
#define MOUSE_VIRTUAL_DESKTOP    0x02  // the coordinates are mapped to the virtual desktop
#define MOUSE_ATTRIBUTES_CHANGED 0x04  // requery for mouse attributes
#if(_WIN32_WINNT >= 0x0600)
#define MOUSE_MOVE_NOCOALESCE    0x08  // do not coalesce WM_MOUSEMOVEs
#endif /* _WIN32_WINNT >= 0x0600 */

#define MOUSE_TERMSRV_SRC_SHADOW        0x100

//
// NtDeviceIoControlFile OutputBuffer record structures for
// IOCTL_MOUSE_QUERY_ATTRIBUTES.
//

typedef struct _MOUSE_ATTRIBUTES {

    //
    // Mouse ID value.  Used to distinguish between mouse types.
    //

    USHORT MouseIdentifier;

    //
    // Number of buttons located on the mouse.
    //

    USHORT NumberOfButtons;

    //
    // Specifies the rate at which the hardware reports mouse input
    // (reports per second).  This may not be applicable for every mouse device.
    //

    USHORT SampleRate;

    //
    // Length of the readahead buffer, in bytes.
    //

    ULONG  InputDataQueueLength;

} MOUSE_ATTRIBUTES, *PMOUSE_ATTRIBUTES;

//
// Define the mouse identifier types.
//

#define MOUSE_INPORT_HARDWARE       0x0001
#define MOUSE_I8042_HARDWARE        0x0002
#define MOUSE_SERIAL_HARDWARE       0x0004
#define BALLPOINT_I8042_HARDWARE    0x0008
#define BALLPOINT_SERIAL_HARDWARE   0x0010
#define WHEELMOUSE_I8042_HARDWARE   0x0020
#define WHEELMOUSE_SERIAL_HARDWARE  0x0040
#define MOUSE_HID_HARDWARE          0x0080
#define WHEELMOUSE_HID_HARDWARE     0x0100
#define HORIZONTAL_WHEEL_PRESENT    0x8000

//
// Generic NtDeviceIoControlFile Input Buffer record structure for
// various mouse IOCTLs.
//

typedef struct _MOUSE_UNIT_ID_PARAMETER {

    //
    // Unit identifier.  Specifies the device unit for which this
    // request is intended.
    //

    USHORT UnitId;

} MOUSE_UNIT_ID_PARAMETER, *PMOUSE_UNIT_ID_PARAMETER;

//
// Define the base values for the mouse error log packet's
// UniqueErrorValue field.
//

#define MOUSE_ERROR_VALUE_BASE        20000

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // _NTDDMOU_
