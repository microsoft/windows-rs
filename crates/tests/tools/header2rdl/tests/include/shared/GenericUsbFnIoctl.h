/*++

Copyright (C) Microsoft Corporation. All rights reserved.

Module Name:

    GenericUsbFnIoctl.h

Abstract:

    This header defines the IOCTL codes for the generic USB function class driver

Environment:

    Kernel and user mode

--*/

#pragma once

// TRANSFERS
#define IOCTL_GENERICUSBFN_TRANSFER_IN                   CTL_CODE(FILE_DEVICE_UNKNOWN,  \
                                                                    0x03,                      \
                                                                    METHOD_IN_DIRECT,          \
                                                                    FILE_READ_DATA |           \
                                                                    FILE_WRITE_DATA)

#define IOCTL_GENERICUSBFN_TRANSFER_IN_APPEND_ZERO_PKT   CTL_CODE(FILE_DEVICE_UNKNOWN,  \
                                                                    0x04,                      \
                                                                    METHOD_IN_DIRECT,          \
                                                                    FILE_READ_DATA |           \
                                                                    FILE_WRITE_DATA)

#define IOCTL_GENERICUSBFN_TRANSFER_OUT                  CTL_CODE(FILE_DEVICE_UNKNOWN,  \
                                                                    0x05,                      \
                                                                    METHOD_OUT_DIRECT,         \
                                                                    FILE_READ_DATA |           \
                                                                    FILE_WRITE_DATA)

#define IOCTL_GENERICUSBFN_CONTROL_STATUS_HANDSHAKE_IN   CTL_CODE(FILE_DEVICE_UNKNOWN,  \
                                                                    0x06,                      \
                                                                    METHOD_BUFFERED,           \
                                                                    FILE_READ_DATA |           \
                                                                    FILE_WRITE_DATA)             

#define IOCTL_GENERICUSBFN_CONTROL_STATUS_HANDSHAKE_OUT  CTL_CODE(FILE_DEVICE_UNKNOWN,  \
                                                                    0x07,                      \
                                                                    METHOD_BUFFERED,           \
                                                                    FILE_READ_DATA |           \
                                                                    FILE_WRITE_DATA)             

//COMMANDS
#define IOCTL_GENERICUSBFN_GET_CLASS_INFO                CTL_CODE(FILE_DEVICE_UNKNOWN,  \
                                                                    0x08,                      \
                                                                    METHOD_OUT_DIRECT,         \
                                                                    FILE_READ_DATA |           \
                                                                    FILE_WRITE_DATA)

#define IOCTL_GENERICUSBFN_GET_PIPE_STATE                CTL_CODE(FILE_DEVICE_UNKNOWN,  \
                                                                    0x09,                      \
                                                                    METHOD_OUT_DIRECT,         \
                                                                    FILE_READ_DATA |           \
                                                                    FILE_WRITE_DATA)

#define IOCTL_GENERICUSBFN_SET_PIPE_STATE                CTL_CODE(FILE_DEVICE_UNKNOWN,  \
                                                                    0x0A,                      \
                                                                    METHOD_IN_DIRECT,          \
                                                                    FILE_READ_DATA |           \
                                                                    FILE_WRITE_DATA)


#define IOCTL_GENERICUSBFN_ACTIVATE_USB_BUS              CTL_CODE(FILE_DEVICE_UNKNOWN,  \
                                                                    0x0B,                      \
                                                                    METHOD_BUFFERED,           \
                                                                    FILE_READ_DATA |           \
                                                                    FILE_WRITE_DATA)

#define IOCTL_GENERICUSBFN_DEACTIVATE_USB_BUS            CTL_CODE(FILE_DEVICE_UNKNOWN,  \
                                                                    0x0C,                      \
                                                                    METHOD_BUFFERED,           \
                                                                    FILE_READ_DATA |           \
                                                                    FILE_WRITE_DATA)

#define IOCTL_GENERICUSBFN_BUS_EVENT_NOTIFICATION        CTL_CODE(FILE_DEVICE_UNKNOWN, \
                                                                    0x0D,                     \
                                                                    METHOD_OUT_DIRECT,        \
                                                                    FILE_READ_DATA |          \
                                                                    FILE_WRITE_DATA)

#define IOCTL_GENERICUSBFN_GET_CLASS_INFO_EX             CTL_CODE(FILE_DEVICE_UNKNOWN, \
                                                                    0x0E,                     \
                                                                    METHOD_OUT_DIRECT,        \
                                                                    FILE_READ_DATA |          \
                                                                    FILE_WRITE_DATA)

#define IOCTL_GENERICUSBFN_GET_INTERFACE_DESCRIPTOR_SET  CTL_CODE(FILE_DEVICE_UNKNOWN, \
                                                                    0x0F,                     \
                                                                    METHOD_OUT_DIRECT,        \
                                                                    FILE_READ_DATA |          \
                                                                    FILE_WRITE_DATA)

#define IOCTL_GENERICUSBFN_REGISTER_USB_STRING           CTL_CODE(FILE_DEVICE_UNKNOWN, \
                                                                    0x10,                     \
                                                                    METHOD_IN_DIRECT,         \
                                                                    FILE_READ_DATA |          \
                                                                    FILE_WRITE_DATA)
