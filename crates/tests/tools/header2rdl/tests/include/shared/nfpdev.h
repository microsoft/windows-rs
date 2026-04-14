/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    NfpDev.h

Abstract:

    This is the include file that defines all constants and types for
    implementing a NearFieldProximity device.

Revision History:

--*/

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef _MSC_VER 
#pragma once
#endif 

#if NTDDI_VERSION >= NTDDI_VISTA

//
// Interface GUIDs
//
#ifdef DEFINE_GUID

/* {FB3842CD-9E2A-4F83-8FCC-4B0761139AE9} */
DEFINE_GUID(GUID_DEVINTERFACE_NFP, 0xFB3842CD, 0x9E2A, 0x4F83, 0x8F, 0xCC, 0x4B, 0x07, 0x61, 0x13, 0x9A, 0xE9);

DEFINE_DEVPROPKEY(DEVPKEY_NFP_Capabilities, 0xFB3842CD, 0x9E2A, 0x4F83, 0x8F, 0xCC, 0x4B, 0x07, 0x61, 0x13, 0x9A, 0xE9, 2);     // DEVPROP_TYPE_STRING_LIST  
// NOTE: All DEVPKEY_NFP_* values are reserved for future use.
//       An IHV MAY use a GUID owned by the IHV to define their own properties.

#endif

#define IOCTL_NFP_GET_NEXT_SUBSCRIBED_MESSAGE  CTL_CODE(FILE_DEVICE_NFP, 0x0010, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_NFP_SET_PAYLOAD                  CTL_CODE(FILE_DEVICE_NFP, 0x0011, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_NFP_GET_NEXT_TRANSMITTED_MESSAGE CTL_CODE(FILE_DEVICE_NFP, 0x0012, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_NFP_DISABLE                      CTL_CODE(FILE_DEVICE_NFP, 0x0013, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_NFP_ENABLE                       CTL_CODE(FILE_DEVICE_NFP, 0x0014, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_NFP_GET_MAX_MESSAGE_BYTES        CTL_CODE(FILE_DEVICE_NFP, 0x0020, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_NFP_GET_KILO_BYTES_PER_SECOND    CTL_CODE(FILE_DEVICE_NFP, 0x0021, METHOD_BUFFERED, FILE_ANY_ACCESS)

// NOTE: For FILE_DEVICE_NFP, functions 0x0000 - 0x00FF are reserved for future use
//       An IHV MAY use functions 0x0100 - 0x01FF for IHV-specific extension IOCTLs

typedef struct _SUBSCRIBED_MESSAGE
{
    DWORD cbPayloadHint;

    BYTE  payload[1]; // Variably sized payload
} SUBSCRIBED_MESSAGE;


#endif // NTDDI_VERSION >= NTDDI_VISTA

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
