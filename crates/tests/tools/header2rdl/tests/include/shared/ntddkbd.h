/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ntddkbd.h

Abstract:

    This is the include file that defines all constants and types for
    accessing the keyboard device.

--*/

#ifndef _NTDDKBD_
#define _NTDDKBD_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
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

#define DD_KEYBOARD_DEVICE_NAME    "\\Device\\KeyboardClass"
#define DD_KEYBOARD_DEVICE_NAME_U L"\\Device\\KeyboardClass"

//
// NtDeviceIoControlFile IoControlCode values for this device.
//
// Warning:  Remember that the low two bits of the code specify how the
//           buffers are passed to the driver!
//

#define IOCTL_KEYBOARD_QUERY_ATTRIBUTES      CTL_CODE(FILE_DEVICE_KEYBOARD, 0x0000, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_KEYBOARD_SET_TYPEMATIC         CTL_CODE(FILE_DEVICE_KEYBOARD, 0x0001, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_KEYBOARD_SET_INDICATORS        CTL_CODE(FILE_DEVICE_KEYBOARD, 0x0002, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_KEYBOARD_QUERY_TYPEMATIC       CTL_CODE(FILE_DEVICE_KEYBOARD, 0x0008, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_KEYBOARD_QUERY_INDICATORS      CTL_CODE(FILE_DEVICE_KEYBOARD, 0x0010, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_KEYBOARD_QUERY_INDICATOR_TRANSLATION   CTL_CODE(FILE_DEVICE_KEYBOARD, 0x0020, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_KEYBOARD_INSERT_DATA           CTL_CODE(FILE_DEVICE_KEYBOARD, 0x0040, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_KEYBOARD_QUERY_EXTENDED_ATTRIBUTES     CTL_CODE(FILE_DEVICE_KEYBOARD, 0x0080, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// These Device IO control query/set IME status to keyboard hardware.
//
#define IOCTL_KEYBOARD_QUERY_IME_STATUS      CTL_CODE(FILE_DEVICE_KEYBOARD, 0x0400, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_KEYBOARD_SET_IME_STATUS        CTL_CODE(FILE_DEVICE_KEYBOARD, 0x0401, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// Declare the GUID that represents the device interface for keyboards.
//
#ifndef FAR
#define FAR
#endif

DEFINE_GUID( GUID_DEVINTERFACE_KEYBOARD, 0x884b96c3, 0x56ef, 0x11d1, \
             0xbc, 0x8c, 0x00, 0xa0, 0xc9, 0x14, 0x05, 0xdd);

//
// Obsolete device interface class GUID name.
// (use of above GUID_DEVINTERFACE_* name is recommended).
//

#define GUID_CLASS_KEYBOARD  GUID_DEVINTERFACE_KEYBOARD


//
// NtReadFile Output Buffer record structures for this device.
//

typedef struct _KEYBOARD_INPUT_DATA {

    //
    // Unit number.  E.g., for \Device\KeyboardPort0 the unit is '0',
    // for \Device\KeyboardPort1 the unit is '1', and so on.
    //

    USHORT UnitId;

    //
    // The "make" scan code (key depression).
    //

    USHORT MakeCode;

    //
    // The flags field indicates a "break" (key release) and other
    // miscellaneous scan code information defined below.
    //

    USHORT Flags;

    USHORT Reserved;

    //
    // Device-specific additional information for the event.
    //

    ULONG ExtraInformation;

} KEYBOARD_INPUT_DATA, *PKEYBOARD_INPUT_DATA;


//
// Define the keyboard overrun MakeCode.
//

#define KEYBOARD_OVERRUN_MAKE_CODE    0xFF

//
// Define the keyboard input data Flags.
//

#define KEY_MAKE  0
#define KEY_BREAK 1
#define KEY_E0    2
#define KEY_E1    4
#define KEY_TERMSRV_SET_LED 8
#define KEY_TERMSRV_SHADOW  0x10
#define KEY_TERMSRV_VKPACKET 0x20
#define KEY_RIM_VKEY 0x40
#define KEY_FROM_KEYBOARD_OVERRIDER 0x80
#define KEY_UNICODE_SEQUENCE_ITEM 0x100
#define KEY_UNICODE_SEQUENCE_END 0x200

//
// NtDeviceIoControlFile Input/Output Buffer record structures for
// IOCTL_KEYBOARD_QUERY_TYPEMATIC/IOCTL_KEYBOARD_SET_TYPEMATIC.
//

typedef struct _KEYBOARD_TYPEMATIC_PARAMETERS {

    //
    // Unit identifier.  Specifies the device unit for which this
    // request is intended.
    //

    USHORT UnitId;

    //
    // Typematic rate, in repeats per second.
    //

    USHORT  Rate;

    //
    // Typematic delay, in milliseconds.
    //

    USHORT  Delay;

} KEYBOARD_TYPEMATIC_PARAMETERS, *PKEYBOARD_TYPEMATIC_PARAMETERS;

//
// NtDeviceIoControlFile OutputBuffer record structures for
// IOCTL_KEYBOARD_QUERY_ATTRIBUTES.
//

typedef struct _KEYBOARD_ID {
    UCHAR Type;       // Keyboard type
    UCHAR Subtype;    // Keyboard subtype (OEM-dependent value)
} KEYBOARD_ID, *PKEYBOARD_ID;

typedef struct _KEYBOARD_ATTRIBUTES {

    //
    // Keyboard ID value.  Used to distinguish between keyboard types.
    //

    KEYBOARD_ID KeyboardIdentifier;

    //
    // Scan code mode.
    //

    USHORT KeyboardMode;

    //
    // Number of function keys located on the keyboard.
    //

    USHORT NumberOfFunctionKeys;

    //
    // Number of LEDs located on the keyboard.
    //

    USHORT NumberOfIndicators;

    //
    // Total number of keys located on the keyboard.
    //

    USHORT NumberOfKeysTotal;

    //
    // Length of the typeahead buffer, in bytes.
    //

    ULONG  InputDataQueueLength;

    //
    // Minimum allowable values of keyboard typematic rate and delay.
    //

    KEYBOARD_TYPEMATIC_PARAMETERS KeyRepeatMinimum;

    //
    // Maximum allowable values of keyboard typematic rate and delay.
    //

    KEYBOARD_TYPEMATIC_PARAMETERS KeyRepeatMaximum;

} KEYBOARD_ATTRIBUTES, *PKEYBOARD_ATTRIBUTES;


//
// The structure used by IOCTL_KEYBOARD_QUERY_EXTENDED_ATTRIBUTES
//

#define KEYBOARD_EXTENDED_ATTRIBUTES_STRUCT_VERSION_1  (1)

typedef struct _KEYBOARD_EXTENDED_ATTRIBUTES {
    //
    // The version of this structure.
    // Only accept KEYBOARD_EXTENDED_ATTRIBUTES_STRUCT_VERSION_1 now.
    //
    UCHAR Version; 

    //
    // Keyboard Form Factor (Usage ID: 0x2C1)
    //
    UCHAR FormFactor;

    //
    // Keyboard Key Type (Usage ID: 0x2C2)
    //
    UCHAR KeyType;

    //
    // Keyboard Physical Layout (Usage ID: 0x2C3)
    //
    UCHAR PhysicalLayout;

    //
    // Vendor-Specific Keyboard Layout (Usage ID: 0x2C4)
    //
    UCHAR VendorSpecificPhysicalLayout;

    //
    // Keyboard IETF Language Tag Index (Usage ID: 0x2C5)
    //
    UCHAR IETFLanguageTagIndex;

    //
    // Implemented Keyboard Input Assist Controls (Usage ID: 0x2C6)
    //
    UCHAR ImplementedInputAssistControls;

} KEYBOARD_EXTENDED_ATTRIBUTES, *PKEYBOARD_EXTENDED_ATTRIBUTES;

//
// ENHANCED_KEYBOARD() is TRUE if the value for keyboard type indicates an
// Enhanced (101- or 102-key) or compatible keyboard.  The result is FALSE
// if the keyboard is an old-style AT keyboard (83- or 84- or 86-key keyboard).
//
#define ENHANCED_KEYBOARD(Id) ((Id).Type == 2 || (Id).Type == 4 || FAREAST_KEYBOARD(Id))
//
// Japanese keyboard(7) and Korean keyboard(8) are also Enhanced (101-)
// or compatible keyboard.
//
#define FAREAST_KEYBOARD(Id)  ((Id).Type == 7 || (Id).Type == 8)

//
// NtDeviceIoControlFile Input/Output Buffer record structures for
// IOCTL_KEYBOARD_QUERY_INDICATORS/IOCTL_KEYBOARD_SET_INDICATORS.
//

typedef struct _KEYBOARD_INDICATOR_PARAMETERS {

    //
    // Unit identifier.  Specifies the device unit for which this
    // request is intended.
    //

    USHORT UnitId;

    //
    // LED indicator state.
    //

    USHORT    LedFlags;

} KEYBOARD_INDICATOR_PARAMETERS, *PKEYBOARD_INDICATOR_PARAMETERS;

//
// NtDeviceIoControlFile Output Buffer record structures for
// IOCTL_KEYBOARD_QUERY_INDICATOR_TRANSLATION.
//

typedef struct _INDICATOR_LIST {

    //
    // The "make" scan code (key depression).
    //

    USHORT MakeCode;

    //
    // The associated LED indicators.
    //

    USHORT IndicatorFlags;

} INDICATOR_LIST, *PINDICATOR_LIST;

typedef struct _KEYBOARD_INDICATOR_TRANSLATION {

    //
    // Number of entries in IndicatorList.
    //

    USHORT NumberOfIndicatorKeys;

    //
    // List of the scancode-to-indicator mappings.
    //

    INDICATOR_LIST IndicatorList[1];

} KEYBOARD_INDICATOR_TRANSLATION, *PKEYBOARD_INDICATOR_TRANSLATION;

//
// Define the keyboard indicators.
//

#define KEYBOARD_LED_INJECTED     0x8000 //Used by Terminal Server
#define KEYBOARD_SHADOW           0x4000 //Used by Terminal Server
#define KEYBOARD_KANA_LOCK_ON     8 // Japanese keyboard
#define KEYBOARD_CAPS_LOCK_ON     4
#define KEYBOARD_NUM_LOCK_ON      2
#define KEYBOARD_SCROLL_LOCK_ON   1

//
// Generic NtDeviceIoControlFile Input Buffer record structure for
// various keyboard IOCTLs.
//

typedef struct _KEYBOARD_UNIT_ID_PARAMETER {

    //
    // Unit identifier.  Specifies the device unit for which this
    // request is intended.
    //

    USHORT UnitId;

} KEYBOARD_UNIT_ID_PARAMETER, *PKEYBOARD_UNIT_ID_PARAMETER;

//
// Define the base values for the keyboard error log packet's
// UniqueErrorValue field.
//

#define KEYBOARD_ERROR_VALUE_BASE        10000

//
// NtDeviceIoControlFile Input/Output Buffer record structures for
// IOCTL_KEYBOARD_QUERY_IME_STATUS/IOCTL_KEYBOARD_SET_IME_STATUS.
//

typedef struct _KEYBOARD_IME_STATUS {

    //
    // Unit identifier.  Specifies the device unit for which this
    // request is intended.
    //

    USHORT UnitId;

    //
    // Ime open or close status.
    //
    ULONG ImeOpen;

    //
    // Ime conversion status.
    //
    ULONG ImeConvMode;

} KEYBOARD_IME_STATUS, *PKEYBOARD_IME_STATUS;

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif // _NTDDKBD_
