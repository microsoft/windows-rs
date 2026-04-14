// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region App, Games, or System family
#include <winapifamily.h>
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

EXTERN_C_START

//
// Device Name - this string is the name of the device. It is the name
// that should be passed to NtOpenFile when accessing the device.
//

#define DD_NDIS_DEVICE_NAME L"\\Device\\NDIS"

//
// NtDeviceIoControlFile IoControlCode values for this device.
//
// Warning: Remember that the low two bits of the code specify how the
//          buffers are passed to the driver!
//

#define _NDIS_CONTROL_CODE(request,method) \
    CTL_CODE(FILE_DEVICE_PHYSICAL_NETCARD, request, method, FILE_ANY_ACCESS)

EXTERN_C_END

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

