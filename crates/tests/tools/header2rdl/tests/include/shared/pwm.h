/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    pwm.h

Abstract:

    This module contains the Pulse Width Modulator (PWM) IOCTL interface for
    use by client applications and kernel-mode drivers.

Environment:

    Kernel-mode and user-mode.

--*/

#include <winapifamily.h>

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_WIN10)

#ifdef _MSC_VER
#pragma once
#endif //_MSC_VER

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

//
// IOCTL codes enumeration
//
enum {
    // Controller IOCTLs
    PWM_IOCTL_ID_CONTROLLER_GET_INFO = 0,
    PWM_IOCTL_ID_CONTROLLER_GET_ACTUAL_PERIOD,
    PWM_IOCTL_ID_CONTROLLER_SET_DESIRED_PERIOD,

    // Pin IOCTLs
    PWM_IOCTL_ID_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE= 100,
    PWM_IOCTL_ID_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE,
    PWM_IOCTL_ID_PIN_GET_POLARITY,
    PWM_IOCTL_ID_PIN_SET_POLARITY,
    PWM_IOCTL_ID_PIN_START,
    PWM_IOCTL_ID_PIN_STOP,
    PWM_IOCTL_ID_PIN_IS_STARTED
};

//
// PWM Device Interface GUID
//
DEFINE_GUID(GUID_DEVINTERFACE_PWM_CONTROLLER, 
    0x60824b4c, 0xeed1, 0x4c9c, 0xb4, 0x9c, 0x1b, 0x96, 0x14, 0x61, 0xa8, 0x19);
#define GUID_DEVINTERFACE_PWM_CONTROLLER_WSZ L"{60824B4C-EED1-4C9C-B49C-1B961461A819}"

//
// PWM wave period in picoseconds, where 1 picosecond is the period of a wave with
// frequency equals to 1 Terahertz(THz).
//
typedef ULONGLONG PWM_PERIOD;

//
// PWM duty cycle percentage is represented as a 64-bit integer in the range
// 0 to PMW_PERCENTAGE_MAX inclusive where Percentage / PMW_PERCENTAGE_MAX is a
// floating-point number in the range 0.0 and 1.0 inclusive.
// Example: A percentage value of 25% (0.25) can be represented as
// (0.25 * ULONGLONG_MAX) which equals 9223372036854775807 and a percentage
// value of 40.25% (0.4025) can be represented as (0.4025 * ULONGLONG_MAX).
//
// To convert from PWM_PERCENTAGE to floating-point value divide by PWM_PERCENTAGE_MAX.
// To convert from floating-point to PWM_PERCENTAGE value multiply by PWM_PERCENTAGE_MAX.
//
typedef ULONGLONG PWM_PERCENTAGE;

#define PWM_PERCENTAGE_MAX  ULONGLONG_MAX

//
// PWM_CONTROLLER_INFO struct is versioned by its byte size including the Size
// field.
//
typedef struct _PWM_CONTROLLER_INFO {

    //
    // Info size - must be set to sizeof(PWM_CONTROLLER_INFO)
    //
    _Field_range_(==, sizeof(PWM_CONTROLLER_INFO))
    SIZE_T Size;

    ULONG PinCount;
    PWM_PERIOD MinimumPeriod;
    PWM_PERIOD MaximumPeriod;
} PWM_CONTROLLER_INFO;

//
// IOCTL_PWM_CONTROLLER_GET_INFO
//

#define IOCTL_PWM_CONTROLLER_GET_INFO \
            CTL_CODE( \
                FILE_DEVICE_CONTROLLER, \
                PWM_IOCTL_ID_CONTROLLER_GET_INFO, \
                METHOD_BUFFERED, \
                FILE_ANY_ACCESS)

typedef PWM_CONTROLLER_INFO PWM_CONTROLLER_GET_INFO_OUTPUT;

//
// IOCTL_PWM_CONTROLLER_GET_ACTUAL_PERIOD
//

#define IOCTL_PWM_CONTROLLER_GET_ACTUAL_PERIOD \
            CTL_CODE( \
                FILE_DEVICE_CONTROLLER, \
                PWM_IOCTL_ID_CONTROLLER_GET_ACTUAL_PERIOD, \
                METHOD_BUFFERED, \
                FILE_ANY_ACCESS)

typedef struct _PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT {
    PWM_PERIOD ActualPeriod;
} PWM_CONTROLLER_GET_ACTUAL_PERIOD_OUTPUT;

//
// IOCTL_PWM_CONTROLLER_SET_DESIRED_PERIOD
//

#define IOCTL_PWM_CONTROLLER_SET_DESIRED_PERIOD \
            CTL_CODE( \
                FILE_DEVICE_CONTROLLER, \
                PWM_IOCTL_ID_CONTROLLER_SET_DESIRED_PERIOD, \
                METHOD_BUFFERED, \
                FILE_WRITE_DATA)

typedef struct _PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT {
    PWM_PERIOD DesiredPeriod;
} PWM_CONTROLLER_SET_DESIRED_PERIOD_INPUT;

typedef struct _PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT {
    PWM_PERIOD ActualPeriod;
} PWM_CONTROLLER_SET_DESIRED_PERIOD_OUTPUT;

//
// PWM_IOCTL_ID_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE
//

#define IOCTL_PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE \
            CTL_CODE( \
                FILE_DEVICE_CONTROLLER, \
                PWM_IOCTL_ID_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE, \
                METHOD_BUFFERED, \
                FILE_ANY_ACCESS)

typedef struct _PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT {
    PWM_PERCENTAGE Percentage;
} PWM_PIN_GET_ACTIVE_DUTY_CYCLE_PERCENTAGE_OUTPUT;

//
// PWM_IOCTL_ID_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE
//

#define IOCTL_PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE \
            CTL_CODE( \
                FILE_DEVICE_CONTROLLER, \
                PWM_IOCTL_ID_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE, \
                METHOD_BUFFERED, \
                FILE_WRITE_DATA)

typedef struct _PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT {
    PWM_PERCENTAGE Percentage;
} PWM_PIN_SET_ACTIVE_DUTY_CYCLE_PERCENTAGE_INPUT;

//
// IOCTL_PWM_PIN_GET_POLARITY
//

#define IOCTL_PWM_PIN_GET_POLARITY \
            CTL_CODE( \
                FILE_DEVICE_CONTROLLER, \
                PWM_IOCTL_ID_PIN_GET_POLARITY, \
                METHOD_BUFFERED, \
                FILE_ANY_ACCESS)

typedef enum _PWM_POLARITY {
    PWM_ACTIVE_HIGH,
    PWM_ACTIVE_LOW,
} PWM_POLARITY;

typedef struct _PWM_PIN_GET_POLARITY_OUTPUT {
    PWM_POLARITY Polarity;
} PWM_PIN_GET_POLARITY_OUTPUT;

//
// IOCTL_PWM_PIN_SET_POLARITY
//

#define IOCTL_PWM_PIN_SET_POLARITY \
            CTL_CODE( \
                FILE_DEVICE_CONTROLLER, \
                PWM_IOCTL_ID_PIN_SET_POLARITY, \
                METHOD_BUFFERED, \
                FILE_WRITE_DATA)

typedef struct _PWM_PIN_SET_POLARITY_INPUT {
    PWM_POLARITY Polarity;
} PWM_PIN_SET_POLARITY_INPUT;

//
// IOCTL_PWM_PIN_START
//

#define IOCTL_PWM_PIN_START \
            CTL_CODE( \
                FILE_DEVICE_CONTROLLER, \
                PWM_IOCTL_ID_PIN_START, \
                METHOD_NEITHER, \
                FILE_WRITE_DATA)

//
// IOCTL_PWM_PIN_STOP
//

#define IOCTL_PWM_PIN_STOP \
            CTL_CODE( \
                FILE_DEVICE_CONTROLLER, \
                PWM_IOCTL_ID_PIN_STOP, \
                METHOD_NEITHER, \
                FILE_WRITE_DATA)

//
// IOCTL_PWM_PIN_IS_STARTED
//

#define IOCTL_PWM_PIN_IS_STARTED \
            CTL_CODE( \
                FILE_DEVICE_CONTROLLER, \
                PWM_IOCTL_ID_PIN_IS_STARTED, \
                METHOD_BUFFERED, \
                FILE_ANY_ACCESS)

typedef struct _PWM_PIN_IS_STARTED_OUTPUT {
    BOOLEAN IsStarted;
} PWM_PIN_IS_STARTED_OUTPUT;

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif // NTDDI_VERSION >= NTDDI_WIN10

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
