/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

    THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY
    KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
    IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR
    PURPOSE.

Module Name:

    gpio.h

Abstract:

    Header file that defines structures, prototypes and definitions required
    by consumers of GPIO services.

Environment:

    Kernel mode

--*/

#ifndef __GPIO_W__
#define __GPIO_W__

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_WIN8)

#ifdef __cplusplus
extern "C" {
#endif

//
// Define IOCTL to read from a  set of GPIO pins (input).
//

#define IOCTL_GPIO_READ_PINS                                                \
    CTL_CODE(FILE_DEVICE_GPIO, 0x0, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// Define IOCTL to write to a set of GPIO pins (output).
//

#define IOCTL_GPIO_WRITE_PINS                                               \
    CTL_CODE(FILE_DEVICE_GPIO, 0x1, METHOD_BUFFERED, FILE_ANY_ACCESS)

//
// Define IOCTL for handling GPIO controller-specific functionality.
//

#define IOCTL_GPIO_CONTROLLER_SPECIFIC_FUNCTION                             \
    CTL_CODE(FILE_DEVICE_GPIO, 0x2, METHOD_BUFFERED, FILE_ANY_ACCESS)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)

//
// Define IOCTL to commit function config pin settings to HW.
//

#define IOCTL_GPIO_COMMIT_FUNCTION_CONFIG_PINS                            \
    CTL_CODE(FILE_DEVICE_GPIO, 0x4, METHOD_BUFFERED, FILE_ANY_ACCESS)

#endif


#ifdef __cplusplus
}
#endif

#endif // (NTDDI_VERSION >= NTDDI_WIN8)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __GPIO_W__


