/*++

  Copyright (c) Microsoft Corporation. All rights reserved.

  Module Name:

    lamp.h

  Abstract:

    Defines the IOCTL interface to a lamp device.

  Environment:

    Kernel and user modes

--*/

#ifndef _LAMP_H
#define _LAMP_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// The device interface class GUID for a lamp device.
DEFINE_GUID(GUID_DEVINTERFACE_LAMP,
    0x6c11e9e3, 0x8238, 0x4f0a, 0x0a, 0x19, 0xaa, 0xec, 0x26, 0xca, 0x5e, 0x98);

// The string literal for constructing the reference string of the lamp device interface.
#define LAMP_DEVICE_SUFFIX  L"Lamp"

// The PnP event which indicates the hardware resources previously assigned to a
// lamp device have been reassigned to support other activities.
DEFINE_GUID(GUID_LAMP_RESOURCES_LOST,
    0xf770e98c, 0x4403, 0x48c9, 0xb1, 0xd2, 0x4e, 0xec, 0x33, 0x02, 0xe4, 0x1f);

// The PnP event which indicates the hardware resources previously lost have
// become available to the lamp device again.
DEFINE_GUID(GUID_LAMP_RESOURCES_AVAILABLE,
    0x185fe7ce, 0x2616, 0x481b, 0x90, 0x94, 0x20, 0xbb, 0x89, 0x3a, 0xcd, 0x81);

//
// IOCTL codes
//

#define IOCTL_LAMP_BASE     FILE_DEVICE_UNKNOWN

// IOCTL_LAMP_GET_CAPABILITIES_{WHITE|COLOR}

// Gets the capabilities if the device can emit white light.
// I/O parameter type: LAMP_CAPABILITIES_WHITE
#define IOCTL_LAMP_GET_CAPABILITIES_WHITE \
    CTL_CODE(IOCTL_LAMP_BASE, 0x0000, METHOD_BUFFERED, FILE_ANY_ACCESS)

// Gets the capabilities if the device can emit color light.
// I/O parameter type: LAMP_CAPABILITIES_COLOR
#define IOCTL_LAMP_GET_CAPABILITIES_COLOR \
    CTL_CODE(IOCTL_LAMP_BASE, 0x0001, METHOD_BUFFERED, FILE_ANY_ACCESS)

// IOCTL_LAMP_{GET|SET}_MODE

// Gets the operating mode (i.e. white-vs-color light) with which the device is currently configured.
// I/O parameter type: LAMP_MODE
#define IOCTL_LAMP_GET_MODE \
    CTL_CODE(IOCTL_LAMP_BASE, 0x0002, METHOD_BUFFERED, FILE_ANY_ACCESS)

// Sets the operating mode. 
// I/O parameter type: LAMP_MODE
#define IOCTL_LAMP_SET_MODE \
    CTL_CODE(IOCTL_LAMP_BASE, 0x0003, METHOD_BUFFERED, FILE_ANY_ACCESS)

// IOCTL_LAMP_{GET|SET}_INTENSITY_{WHITE|COLOR}

// Gets the current light intensity if the device is configured to emit white light.
// I/O parameter type: LAMP_INTENSITY_WHITE
#define IOCTL_LAMP_GET_INTENSITY_WHITE \
    CTL_CODE(IOCTL_LAMP_BASE, 0x0004, METHOD_BUFFERED, FILE_ANY_ACCESS)

// Sets the white light intensity. 
// I/O parameter type: LAMP_INTENSITY_WHITE
#define IOCTL_LAMP_SET_INTENSITY_WHITE \
    CTL_CODE(IOCTL_LAMP_BASE, 0x0005, METHOD_BUFFERED, FILE_ANY_ACCESS)

// Gets the current light intensity if the device is configured to emit color light.
// I/O parameter type: LAMP_INTENSITY_COLOR
#define IOCTL_LAMP_GET_INTENSITY_COLOR \
    CTL_CODE(IOCTL_LAMP_BASE, 0x0006, METHOD_BUFFERED, FILE_ANY_ACCESS)

// Sets the color light intensity. 
// I/O parameter type: LAMP_INTENSITY_COLOR
#define IOCTL_LAMP_SET_INTENSITY_COLOR \
    CTL_CODE(IOCTL_LAMP_BASE, 0x0007, METHOD_BUFFERED, FILE_ANY_ACCESS)

// IOCTL_LAMP_{GET|SET}_EMITTING_LIGHT

// Determines if the device is currently emitting light.
// I/O parameter type: BOOLEAN
#define IOCTL_LAMP_GET_EMITTING_LIGHT \
    CTL_CODE(IOCTL_LAMP_BASE, 0x0008, METHOD_BUFFERED, FILE_ANY_ACCESS)

// Toggle the device to start/stop emitting light.
// I/O parameter type: BOOLEAN
#define IOCTL_LAMP_SET_EMITTING_LIGHT \
    CTL_CODE(IOCTL_LAMP_BASE, 0x0009, METHOD_BUFFERED, FILE_ANY_ACCESS)

// The operating mode of a lamp device.
// This is the I/O parameter type of IOCTL_LAMP_{GET|SET}_MODE.
typedef enum LAMP_MODE
{
    LAMP_MODE_WHITE = 0,    // (Required) White light only
    LAMP_MODE_COLOR         // (Optional) Color light
} LAMP_MODE;

// The I/O parameter type of IOCTL_LAMP_GET_CAPABILITIES_WHITE.
typedef struct LAMP_CAPABILITIES_WHITE
{
    BOOLEAN IsLightIntensityAdjustable;
                            // If this field evaluates true, a client can get/set light
                            // intensity by calling IOCTL_LAMP_{GET|SET}_INTENSITY_WHITE.
} LAMP_CAPABILITIES_WHITE;

// The I/O parameter type of IOCTL_LAMP_GET_CAPABILITIES_COLOR.
typedef struct LAMP_CAPABILITIES_COLOR
{
    BOOLEAN IsSupported;    // True if the device can emit color light; false otherwise.
    BOOLEAN IsLightIntensityAdjustable;
                            // If IsSupported evaluates true (i.e. the driver is capable
                            // of emitting color light) and this field evaluates true, a
                            // client can get/set light intensity of a color lamp by
                            // calling IOCTL_LAMP_{GET|SET}_INTENSITY_COLOR.
} LAMP_CAPABILITIES_COLOR;

// The I/O parameter type of IOCTL_LAMP_{GET|SET}_INTENSITY_WHITE.
typedef struct LAMP_INTENSITY_WHITE
{
    BYTE Value;             // White light intensity in percentage (0-100)
} LAMP_INTENSITY_WHITE;

// The I/O parameter type of IOCTL_LAMP_{GET|SET}_INTENSITY_COLOR.
typedef struct LAMP_INTENSITY_COLOR
{
    BYTE Red;               // Red light intensity in percentage (0-100)
    BYTE Green;             // Green light intensity in percentage (0-100)
    BYTE Blue;              // Blue light intensity in percentage (0-100)
} LAMP_INTENSITY_COLOR;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _LAMP_H
