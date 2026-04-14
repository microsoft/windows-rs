/*
Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    TBT3IOCTLS.H

Abstract:

    Common header file for all user-mode IOCTLs defined for Thunderbolt 3 routers

Environment:

    Kernel & user-mode

--*/

#define NVM_DATA_LENGTH (16 * sizeof(DWORD))

/* {9013316D-7092-4E8E-8F9D-24214977ADCE} */

DEFINE_GUID(GUID_TBT3_FW_UPDATE_INTERFACE,
0x9013316D, 0x7092, 0x4E8E, 0x8F, 0x9D, 0x24, 0x21, 0x49, 0x77, 0xAD, 0xCE);

#include <pshpack1.h>

typedef struct _TBT3_NVM_GET_UUID_OUTPUT {
    ULONG64 Uuid;
    USHORT  ProductId;
} TBT3_GET_UUID_OUTPUT, *PTBT3_GET_UUID_OUTPUT;

typedef struct _TBT3_NVM_WRITE_INPUT {
    ULONG    NVMOffset;
    UCHAR    Data[NVM_DATA_LENGTH];
} TBT3_NVM_WRITE_INPUT, *PTBT3_NVM_WRITE_INPUT;

typedef struct _TBT3_NVM_WRITE_OUTPUT {
    ULONG    NVMStatus;
} TBT3_NVM_WRITE_OUTPUT, *PTBT3_NVM_WRITE_OUTPUT;

typedef struct _TBT3_NVM_WRITE_AUTHENTICATE_OUTPUT {
    ULONG    NVMStatus;
} TBT3_NVM_WRITE_AUTHENTICATE_OUTPUT, *PTBT3_NVM_WRITE_AUTHENTICATE_OUTPUT;

typedef struct _TBT3_NVM_READ_INPUT {
    ULONG    NVMOffset;
    USHORT   DataLength;
} TBT3_NVM_READ_INPUT, *PTBT3_NVM_READ_INPUT;

typedef struct _TBT3_NVM_READ_OUTPUT {
    ULONG    NVMStatus;
    UCHAR    Data[NVM_DATA_LENGTH];
} TBT3_NVM_READ_OUTPUT, *PTBT3_NVM_READ_OUTPUT;

#include <poppack.h>

#define TBT3_GET_UUID                       2000
#define TBT3_NVM_WRITE                      2001  
#define TBT3_NVM_WRITE_AUTHENTICATE         2002
#define TBT3_NVM_READ                       2003
#define TBT3_NVM_POWER_CYCLE                2004


#define IOCTL_TBT3_GET_UUID  \
    CTL_CODE(FILE_DEVICE_USB4,\
    TBT3_GET_UUID,  \
    METHOD_BUFFERED,  \
    FILE_ANY_ACCESS)

#define IOCTL_TBT3_NVM_WRITE  \
    CTL_CODE(FILE_DEVICE_USB4,\
    TBT3_NVM_WRITE,  \
    METHOD_BUFFERED,  \
    FILE_ANY_ACCESS)

#define IOCTL_TBT3_NVM_WRITE_AUTHENTICATE  \
    CTL_CODE(FILE_DEVICE_USB4,\
    TBT3_NVM_WRITE_AUTHENTICATE,  \
    METHOD_BUFFERED,  \
    FILE_ANY_ACCESS)

#define IOCTL_TBT3_NVM_READ  \
    CTL_CODE(FILE_DEVICE_USB4,\
    TBT3_NVM_READ,  \
    METHOD_BUFFERED,  \
    FILE_ANY_ACCESS)

#define IOCTL_TBT3_NVM_POWER_CYCLE  \
    CTL_CODE(FILE_DEVICE_USB4,\
    TBT3_NVM_POWER_CYCLE,  \
    METHOD_BUFFERED,  \
    FILE_ANY_ACCESS)
