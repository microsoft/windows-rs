//----------------------------------------------------------------------------
//
// File: GPIOButtonTypes.h (Shared with Button Injection Tool)
//
// Copyright (c) Microsoft Corporation.  All rights reserved.
//
//----------------------------------------------------------------------------


#pragma once

typedef enum
{
    GPIO_BUTTON_POWER           = 0,
    GPIO_BUTTON_WINDOWS         = 1,
    GPIO_BUTTON_VOLUME_UP       = 2,
    GPIO_BUTTON_VOLUME_DOWN     = 3,
    GPIO_BUTTON_ROTATION_LOCK   = 4,
    GPIO_BUTTON_BACK            = 5,
    GPIO_BUTTON_SEARCH          = 6,
    GPIO_BUTTON_CAMERA_FOCUS    = 7,
    GPIO_BUTTON_CAMERA_SHUTTER  = 8,
    GPIO_BUTTON_RINGER_TOGGLE   = 9,
    GPIO_BUTTON_HEADSET         = 10,
    GPIO_BUTTON_HWKB_DEPLOY     = 11,
    GPIO_BUTTON_CAMERA_LENS     = 12,
    GPIO_BUTTON_OEM_CUSTOM      = 13,
    GPIO_BUTTON_OEM_CUSTOM2     = 14,
    GPIO_BUTTON_OEM_CUSTOM3     = 15,

    GPIO_BUTTON_COUNT_MIN       = 5,
    GPIO_BUTTON_COUNT           = 16
} GPIOBUTTONS_BUTTON_TYPE;


//
// Button Bits for reference.  We can bitshift to get this based on the
// GPIOBUTTONS_BUTTON_TYPE.
//

#define BUTTON_BIT_POWER            0x0001
#define BUTTON_BIT_WINDOWS          0x0002
#define BUTTON_BIT_VOLUMEUP         0x0004
#define BUTTON_BIT_VOLUMEDOWN       0x0008
#define BUTTON_BIT_ROTATION_LOCK    0x0010
#define BUTTON_BIT_BACK             0x0020
#define BUTTON_BIT_SEARCH           0x0040
#define BUTTON_BIT_CAMERAFOCUS      0x0080
#define BUTTON_BIT_CAMERASHUTTER    0x0100
#define BUTTON_BIT_RINGERTOGGLE     0x0200
#define BUTTON_BIT_HEADSET          0x0400
#define BUTTON_BIT_HWKBDEPLOY       0x0800
#define BUTTON_BIT_CAMERALENS       0x1000
#define BUTTON_BIT_OEMCUSTOM        0x2000
#define BUTTON_BIT_OEMCUSTOM2       0x4000
#define BUTTON_BIT_OEMCUSTOM3       0x8000
#define BUTTON_BIT_ALLBUTTONSMASK   0x3FFF


#define IOCTL_BUTTON_SET_ENABLED_ON_IDLE                                    \
    CTL_CODE(FILE_DEVICE_KEYBOARD, 170, METHOD_BUFFERED, FILE_ANY_ACCESS)

typedef struct _INPUT_BUTTON_ENABLE_INFO
{
    GPIOBUTTONS_BUTTON_TYPE ButtonType;
    BOOLEAN Enabled;
} INPUT_BUTTON_ENABLE_INFO, *PINPUT_BUTTON_ENABLE_INFO;


#define IOCTL_BUTTON_GET_ENABLED_ON_IDLE                                    \
    CTL_CODE(FILE_DEVICE_KEYBOARD, 171, METHOD_BUFFERED, FILE_ANY_ACCESS)


